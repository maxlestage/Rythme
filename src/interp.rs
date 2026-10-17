//! The interpreter: evaluates the syntax tree.
//!
//! Calls in tail position (the last thing a function does) do not grow the
//! stack, so recursive functions can loop millions of times.

use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::*;
use crate::builtins::BUILTINS;
use crate::diagnostic::{Diagnostic, Fallible};
use crate::source::Span;
use crate::suggest;
use crate::symbol::{Sym, SymMap};
use crate::value::*;

/// Why the evaluation of an expression stopped early.
pub enum Flow {
    Err(Box<Diagnostic>),
    /// `?` found a `None` or an `Err`: the current function returns it.
    Return(Value),
}

impl From<Box<Diagnostic>> for Flow {
    fn from(d: Box<Diagnostic>) -> Flow {
        Flow::Err(d)
    }
}

pub type Eval = Result<Value, Flow>;

/// Result of an expression in tail position: a value, or a call still to make.
pub enum Tail {
    Done(Value),
    Call(Value, Vec<Value>, Span),
}

#[derive(Clone)]
pub struct TestCase {
    pub name: Rc<str>,
    pub body: Rc<Expr>,
    pub span: Span,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Run,
    /// Load definitions, skip top-level expressions, collect tests.
    Test,
    Repl,
}

/// What `?`, `unwrap`... see in a value.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wrap {
    Some,
    None,
    Ok,
    Err,
    Other,
}

const MAX_RANGE: i64 = 50_000_000;

/// How many bytes of stack the interpreter may use for nested calls. The
/// `rythme` command runs on a thread with a large stack and raises it.
static STACK_BUDGET: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(4 << 20);

pub fn set_stack_budget(bytes: usize) {
    STACK_BUDGET.store(bytes, std::sync::atomic::Ordering::Relaxed);
}

/// Maximum number of nested calls, for hosts whose stack cannot be measured
/// (WebAssembly in a browser).
static MAX_DEPTH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(usize::MAX);

pub fn set_max_depth(depth: usize) {
    MAX_DEPTH.store(depth, std::sync::atomic::Ordering::Relaxed);
}

/// Approximate address of the top of the stack.
#[inline(always)]
fn stack_position() -> usize {
    let marker = 0u8;
    std::hint::black_box(&marker) as *const u8 as usize
}

pub struct Interp {
    pub globals: SymMap<Value>,
    pub types: HashMap<Sym, Vec<Rc<CtorInfo>>>,
    pub tests: Vec<TestCase>,
    pub script_args: Vec<String>,
    depth: usize,
    stack_base: usize,
    /// Reused buffer for the names bound by patterns (avoids an allocation per call).
    scratch: Vec<(Sym, Value)>,
    /// Top-level names of the running file that are not defined yet.
    pending: SymMap<Span>,
    rng: u64,
    some: Rc<CtorInfo>,
    none_info: Rc<CtorInfo>,
    none: Value,
    ok: Rc<CtorInfo>,
    err: Rc<CtorInfo>,
}

impl Default for Interp {
    fn default() -> Self {
        Self::new()
    }
}

impl Interp {
    pub fn new() -> Interp {
        let option = Sym::new("Option");
        let result = Sym::new("Result");
        let ctor = |t: Sym, n: &str, fields: &[&str]| {
            Rc::new(CtorInfo { type_name: t, name: Sym::new(n), fields: fields.iter().map(|f| Sym::new(f)).collect() })
        };
        let some = ctor(option, "Some", &["value"]);
        let none_info = ctor(option, "None", &[]);
        let ok = ctor(result, "Ok", &["value"]);
        let err = ctor(result, "Err", &["error"]);
        let none = Value::Variant(Rc::new(Variant { ctor: none_info.clone(), fields: Vec::new() }));

        let mut globals = SymMap::default();
        for b in BUILTINS {
            globals.insert(Sym::new(b.name), Value::Builtin(b));
        }
        globals.insert(Sym::new("pi"), Value::Float(std::f64::consts::PI));
        globals.insert(some.name, Value::Ctor(some.clone()));
        globals.insert(none_info.name, none.clone());
        globals.insert(ok.name, Value::Ctor(ok.clone()));
        globals.insert(err.name, Value::Ctor(err.clone()));

        let mut types = HashMap::new();
        types.insert(option, vec![some.clone(), none_info.clone()]);
        types.insert(result, vec![ok.clone(), err.clone()]);

        let seed = ((crate::io::now() * 1e9) as u64 ^ 0x2545_F491_4F6C_DD1D) | 1;

        Interp {
            globals,
            types,
            tests: Vec::new(),
            script_args: Vec::new(),
            depth: 0,
            stack_base: stack_position(),
            scratch: Vec::new(),
            pending: SymMap::default(),
            rng: seed,
            some,
            none_info,
            none,
            ok,
            err,
        }
    }

    // ----- Option and Result ------------------------------------------

    pub fn some(&self, v: Value) -> Value {
        Value::Variant(Rc::new(Variant { ctor: self.some.clone(), fields: vec![v] }))
    }

    pub fn none(&self) -> Value {
        self.none.clone()
    }

    pub fn ok(&self, v: Value) -> Value {
        Value::Variant(Rc::new(Variant { ctor: self.ok.clone(), fields: vec![v] }))
    }

    pub fn err(&self, v: Value) -> Value {
        Value::Variant(Rc::new(Variant { ctor: self.err.clone(), fields: vec![v] }))
    }

    pub fn option(&self, v: Option<Value>) -> Value {
        match v {
            Some(v) => self.some(v),
            None => self.none(),
        }
    }

    pub fn wrap(&self, v: &Value) -> Wrap {
        let Value::Variant(var) = v else { return Wrap::Other };
        if Rc::ptr_eq(&var.ctor, &self.some) {
            Wrap::Some
        } else if Rc::ptr_eq(&var.ctor, &self.none_info) {
            Wrap::None
        } else if Rc::ptr_eq(&var.ctor, &self.ok) {
            Wrap::Ok
        } else if Rc::ptr_eq(&var.ctor, &self.err) {
            Wrap::Err
        } else {
            Wrap::Other
        }
    }

    /// The value inside `Some(x)`, `Ok(x)` or `Err(x)`.
    pub fn payload(v: &Value) -> Value {
        match v {
            Value::Variant(var) => var.fields.first().cloned().unwrap_or(Value::Unit),
            _ => Value::Unit,
        }
    }

    pub fn random_u64(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.rng;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    // ----- definitions -------------------------------------------------

    pub fn register_type(&mut self, t: &TypeDef) {
        let mut ctors = Vec::new();
        for v in &t.variants {
            let info = Rc::new(CtorInfo { type_name: t.name, name: v.name, fields: v.fields.clone() });
            let value = if v.fields.is_empty() {
                Value::Variant(Rc::new(Variant { ctor: info.clone(), fields: Vec::new() }))
            } else {
                Value::Ctor(info.clone())
            };
            self.globals.insert(v.name, value);
            ctors.push(info);
        }
        self.types.insert(t.name, ctors);
    }

    pub fn define_fn(&mut self, def: &Rc<FnDef>) {
        if let Some(name) = def.name {
            let clo = Closure { def: def.clone(), env: Env::empty(), self_name: None };
            self.globals.insert(name, Value::Func(Rc::new(clo)));
        }
    }

    /// Runs the items of a module (its `use` dependencies must already have run).
    pub fn exec_items(&mut self, items: &[Item], mode: Mode) -> Fallible<Option<Value>> {
        for item in items {
            match item {
                Item::Type(t) => self.register_type(t),
                Item::Stmt(Stmt::Fn(def)) => self.define_fn(def),
                _ => {}
            }
        }
        self.pending.clear();
        for item in items {
            if let Item::Stmt(Stmt::Let(pat, _)) = item {
                let mut names = Vec::new();
                pat.bindings(&mut names);
                self.pending.extend(names.into_iter().filter(|(s, _)| !self.globals.contains_key(s)));
            }
        }
        let mut last = None;
        for item in items {
            last = None;
            match item {
                Item::Stmt(Stmt::Let(pat, e)) => {
                    let v = self.eval_top(e)?;
                    let mut binds = Vec::new();
                    if !match_pattern(pat, &v, &mut binds) {
                        return Err(pattern_mismatch(pat, &v));
                    }
                    for (s, v) in binds {
                        self.pending.remove(&s);
                        self.globals.insert(s, v);
                    }
                }
                Item::Stmt(Stmt::Expr(e)) => {
                    if mode != Mode::Test {
                        last = Some(self.eval_top(e)?);
                    }
                }
                Item::Test(name, body, span) if mode == Mode::Test => {
                    self.tests.push(TestCase { name: name.clone(), body: body.clone(), span: *span });
                }
                _ => {}
            }
        }
        Ok(last)
    }

    /// Evaluates an expression outside of any function.
    pub fn eval_top(&mut self, e: &Expr) -> Fallible<Value> {
        self.depth = 0;
        match self.eval(e, &Env::empty()) {
            Ok(v) => Ok(v),
            Err(Flow::Err(d)) => Err(d),
            Err(Flow::Return(_)) => Err(question_outside(e.span)),
        }
    }

    /// Evaluates the body of a test (like a function body: `?` is allowed).
    pub fn eval_test(&mut self, e: &Expr) -> Fallible<Value> {
        self.depth = 0;
        let r = match &e.kind {
            ExprKind::Block(stmts) => self.exec_block(stmts, Env::empty()),
            _ => self.eval_tail(e, &Env::empty()),
        };
        match r {
            Ok(Tail::Done(v)) => Ok(v),
            Ok(Tail::Call(f, args, span)) => self.call(f, args, span),
            Err(Flow::Return(v)) => Ok(v),
            Err(Flow::Err(d)) => Err(d),
        }
    }

    // ----- evaluation --------------------------------------------------

    /// Evaluates an expression. The common cases are kept here and the rare
    /// ones live in `eval_rest`, so that each nested call uses little stack.
    pub fn eval(&mut self, e: &Expr, env: &Env) -> Eval {
        match &e.kind {
            ExprKind::Int(n) => Ok(Value::Int(*n)),
            ExprKind::Var(s) | ExprKind::Ctor(s) => match env.lookup(*s).or_else(|| self.globals.get(s)) {
                Some(v) => Ok(v.clone()),
                None => Err(self.unknown_name(*s, env, e.span).into()),
            },
            ExprKind::Call(..) | ExprKind::Method(..) | ExprKind::If(..) | ExprKind::Match(..) | ExprKind::Block(_) => {
                match self.eval_tail(e, env)? {
                    Tail::Done(v) => Ok(v),
                    Tail::Call(f, args, span) => Ok(self.call(f, args, span)?),
                }
            }
            ExprKind::Binary(op, l, r, op_span) if !matches!(op, BinOp::And | BinOp::Or) => {
                let lv = self.eval(l, env)?;
                let rv = self.eval(r, env)?;
                Ok(binop(*op, &lv, &rv, *op_span)?)
            }
            _ => self.eval_rest(e, env),
        }
    }

    #[inline(never)]
    fn eval_rest(&mut self, e: &Expr, env: &Env) -> Eval {
        match &e.kind {
            ExprKind::Unit => Ok(Value::Unit),
            ExprKind::Bool(b) => Ok(Value::Bool(*b)),
            ExprKind::Float(f) => Ok(Value::Float(*f)),
            ExprKind::Text(s) => Ok(Value::Text(s.clone())),
            ExprKind::Template(parts) => {
                let mut s = String::new();
                for p in parts {
                    match p {
                        TemplatePart::Lit(l) => s.push_str(l),
                        TemplatePart::Expr(x) => match self.eval(x, env)? {
                            Value::Text(t) => s.push_str(&t),
                            v => s.push_str(&v.display()),
                        },
                    }
                }
                Ok(Value::Text(Rc::from(s)))
            }
            ExprKind::List(items) => {
                let mut v = Vec::with_capacity(items.len());
                for it in items {
                    match it {
                        ListItem::One(x) => v.push(self.eval(x, env)?),
                        ListItem::Spread(x) => match self.eval(x, env)? {
                            Value::List(l) => v.extend(l.iter().cloned()),
                            other => return Err(spread_error(&other, x.span, true).into()),
                        },
                    }
                }
                Ok(Value::list(v))
            }
            ExprKind::Record(items) => {
                let mut r = Record::new();
                for it in items {
                    match it {
                        RecordItem::Field(k, _, x) => {
                            let v = self.eval(x, env)?;
                            r.set(k.clone(), v);
                        }
                        RecordItem::Spread(x) => match self.eval(x, env)? {
                            Value::Record(src) => {
                                for (k, v) in src.iter() {
                                    r.set(k.clone(), v.clone());
                                }
                            }
                            other => return Err(spread_error(&other, x.span, false).into()),
                        },
                    }
                }
                Ok(Value::Record(Rc::new(r)))
            }
            ExprKind::Lambda(def) => Ok(Value::Func(Rc::new(Closure { def: def.clone(), env: env.clone(), self_name: None }))),
            ExprKind::Field(obj, name, nspan) => {
                let v = self.eval(obj, env)?;
                Ok(self.field(&v, name, *nspan)?)
            }
            ExprKind::Index(obj, idx) => {
                let o = self.eval(obj, env)?;
                let i = self.eval(idx, env)?;
                Ok(index(&o, &i, e.span, idx.span)?)
            }
            ExprKind::Unary(op, x) => {
                let v = self.eval(x, env)?;
                Ok(unary(*op, &v, e.span, x.span)?)
            }
            ExprKind::Binary(op, l, r, _) => {
                // `and` / `or`: the right side is only evaluated when needed.
                let lv = self.eval(l, env)?;
                let left = truth(&lv, l.span)?;
                if (*op == BinOp::And) != left {
                    return Ok(Value::Bool(left));
                }
                let rv = self.eval(r, env)?;
                Ok(Value::Bool(truth(&rv, r.span)?))
            }
            ExprKind::Try(x) => {
                let v = self.eval(x, env)?;
                match self.wrap(&v) {
                    Wrap::Some | Wrap::Ok => Ok(Interp::payload(&v)),
                    Wrap::None | Wrap::Err => Err(Flow::Return(v)),
                    Wrap::Other => Err(try_error(&v, e.span).into()),
                }
            }
            ExprKind::Range(a, b) => {
                let av = self.eval(a, env)?;
                let bv = self.eval(b, env)?;
                Ok(range(&av, &bv, e.span)?)
            }
            ExprKind::Int(_)
            | ExprKind::Var(_)
            | ExprKind::Ctor(_)
            | ExprKind::Call(..)
            | ExprKind::Method(..)
            | ExprKind::If(..)
            | ExprKind::Match(..)
            | ExprKind::Block(_) => self.eval(e, env),
        }
    }

    fn eval_tail(&mut self, e: &Expr, env: &Env) -> Result<Tail, Flow> {
        match &e.kind {
            ExprKind::If(c, a, b) => {
                let cv = self.eval(c, env)?;
                if truth(&cv, c.span)? {
                    self.eval_tail(a, env)
                } else {
                    self.eval_tail(b, env)
                }
            }
            ExprKind::Match(subject, arms) => {
                let v = self.eval(subject, env)?;
                let mut binds = std::mem::take(&mut self.scratch);
                for arm in arms {
                    binds.clear();
                    if !match_pattern(&arm.pat, &v, &mut binds) {
                        continue;
                    }
                    let mut arm_env = env.clone();
                    for (s, x) in binds.drain(..) {
                        arm_env = arm_env.bind(s, x);
                    }
                    if let Some(g) = &arm.guard {
                        self.scratch = binds;
                        let gv = self.eval(g, &arm_env)?;
                        binds = std::mem::take(&mut self.scratch);
                        if !truth(&gv, g.span)? {
                            continue;
                        }
                    }
                    self.scratch = binds;
                    return self.eval_tail(&arm.body, &arm_env);
                }
                self.scratch = binds;
                Err(no_case(&v, subject.span).into())
            }
            ExprKind::Block(stmts) => self.exec_block(stmts, env.clone()),
            ExprKind::Call(callee, args) => {
                let f = self.eval(callee, env)?;
                let mut argv = Vec::with_capacity(args.len());
                for a in args {
                    argv.push(self.eval(a, env)?);
                }
                Ok(Tail::Call(f, argv, e.span))
            }
            ExprKind::Method(recv, name, args, name_span) => self.method_call(e, recv, *name, args, *name_span, env),
            _ => Ok(Tail::Done(self.eval(e, env)?)),
        }
    }

    /// `value.name(args)`: a function stored in a record field, or `name(value, args)`.
    #[inline(never)]
    fn method_call(&mut self, e: &Expr, recv: &Expr, name: Sym, args: &[Expr], name_span: Span, env: &Env) -> Result<Tail, Flow> {
        let rv = self.eval(recv, env)?;
        let mut argv = Vec::with_capacity(args.len() + 1);
        let field = match &rv {
            Value::Record(r) => r.get(&name.name()).cloned(),
            _ => None,
        };
        let f = match field {
            Some(f) => f,
            None => {
                let f = match env.lookup(name).or_else(|| self.globals.get(&name)) {
                    Some(f) => f.clone(),
                    None => {
                        let mut d = *self.unknown_name(name, env, name_span);
                        d.message = t!("no function named `{}`", "aucune fonction nommée `{}`", name);
                        d.notes.push(t!("`value.{0}(...)` calls `{0}(value, ...)`", "`valeur.{0}(...)` appelle `{0}(valeur, ...)`", name));
                        return Err(d.boxed().into());
                    }
                };
                argv.push(rv);
                f
            }
        };
        for a in args {
            argv.push(self.eval(a, env)?);
        }
        Ok(Tail::Call(f, argv, e.span))
    }

    fn exec_block(&mut self, stmts: &[Stmt], mut env: Env) -> Result<Tail, Flow> {
        let n = stmts.len();
        for (i, s) in stmts.iter().enumerate() {
            match s {
                Stmt::Let(p, x) => {
                    let v = match (&p.kind, &x.kind) {
                        // `name = fn(...) = ...` can call itself by its name.
                        (PatKind::Bind(name), ExprKind::Lambda(def)) => {
                            Value::Func(Rc::new(Closure { def: def.clone(), env: env.clone(), self_name: Some(*name) }))
                        }
                        _ => self.eval(x, &env)?,
                    };
                    let mut binds = Vec::new();
                    if !match_pattern(p, &v, &mut binds) {
                        return Err(pattern_mismatch(p, &v).into());
                    }
                    for (s, v) in binds {
                        env = env.bind(s, v);
                    }
                }
                Stmt::Expr(x) => {
                    if i + 1 == n {
                        return self.eval_tail(x, &env);
                    }
                    self.eval(x, &env)?;
                }
                Stmt::Fn(def) => {
                    let clo = Closure { def: def.clone(), env: env.clone(), self_name: def.name };
                    if let Some(name) = def.name {
                        env = env.bind(name, Value::Func(Rc::new(clo)));
                    }
                }
            }
        }
        Ok(Tail::Done(Value::Unit))
    }

    pub fn unknown_name(&self, s: Sym, env: &Env, span: Span) -> Box<Diagnostic> {
        let name = s.name();
        if let Some(def) = self.pending.get(&s) {
            return Diagnostic::error("E003", t!("`{name}` is not defined yet", "`{name}` n'est pas encore défini"))
                .at(span)
                .label(t!("used here", "utilisé ici"))
                .secondary(*def, t!("defined later, here", "défini plus loin, ici"))
                .help(t!(
                    "the file runs from top to bottom: move this definition above the line that uses it (directly or through a function)",
                    "le fichier s'exécute de haut en bas : déplacez cette définition au-dessus de la ligne qui l'utilise (directement ou via une fonction)"
                ))
                .boxed();
        }
        let mut d =
            Diagnostic::error("E003", t!("unknown name `{name}`", "nom inconnu `{name}`")).at(span).label(t!("not defined", "non défini"));
        if let Some(h) = suggest::alias_hint(&name) {
            d = d.help(h);
        } else {
            let names: Vec<Rc<str>> = env.names().into_iter().chain(self.globals.keys().copied()).map(|s| s.name()).collect();
            if let Some(best) = suggest::closest(&name, names.iter().map(|s| &**s)) {
                d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
            }
        }
        d.boxed()
    }

    fn field(&self, v: &Value, name: &str, span: Span) -> Fallible<Value> {
        match v {
            Value::Record(r) => r.get(name).cloned().ok_or_else(|| {
                let mut d =
                    Diagnostic::error("E010", t!("this record has no field `{name}`", "cet enregistrement n'a pas de champ `{name}`"))
                        .at(span)
                        .label(t!("unknown field", "champ inconnu"));
                if let Some(best) = suggest::closest(name, r.keys().iter().map(|k| &**k)) {
                    d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
                }
                if r.is_empty() {
                    d = d.note(t!("the record is empty: {{}}", "l'enregistrement est vide : {{}}"));
                } else {
                    let keys: Vec<String> = r.keys().iter().take(12).map(|k| format!("`{k}`")).collect();
                    d = d.note(t!("its fields are: {}", "ses champs sont : {}", keys.join(", ")));
                }
                d.help(t!(
                    "to get an Option instead of an error, use `get(record, \"{name}\")`",
                    "pour obtenir une Option au lieu d'une erreur, utilisez `get(enregistrement, \"{name}\")`"
                ))
                .boxed()
            }),
            Value::Variant(var) => match var.ctor.fields.iter().position(|f| &*f.name() == name) {
                Some(i) => Ok(var.fields[i].clone()),
                None => {
                    let ctor = var.ctor.name;
                    let mut d = Diagnostic::error("E010", t!("`{ctor}` has no field `{name}`", "`{ctor}` n'a pas de champ `{name}`"))
                        .at(span)
                        .label(t!("unknown field", "champ inconnu"));
                    if var.ctor.fields.is_empty() {
                        d = d.note(t!("`{ctor}` has no fields", "`{ctor}` n'a aucun champ"));
                    } else {
                        let f: Vec<String> = var.ctor.fields.iter().map(|f| format!("`{f}`")).collect();
                        d = d.note(t!("the fields of `{ctor}` are: {}", "les champs de `{ctor}` sont : {}", f.join(", ")));
                    }
                    d = d.help(t!(
                        "a value of type `{}` may be another variant: use `match` to handle each case",
                        "une valeur du type `{}` peut être une autre variante : utilisez `match` pour traiter chaque cas",
                        var.ctor.type_name
                    ));
                    Err(d.boxed())
                }
            },
            other => {
                let mut d = Diagnostic::error("E010", t!("{} has no fields", "{} n'a pas de champs", other.describe()))
                    .at(span)
                    .label(t!("no field `{name}` here", "pas de champ `{name}` ici"));
                if self.globals.get(&Sym::new(name)).is_some_and(|f| f.is_callable()) {
                    d = d.help(t!(
                        "to call `{name}` on this value, add parentheses: `.{name}()`",
                        "pour appeler `{name}` sur cette valeur, ajoutez des parenthèses : `.{name}()`"
                    ));
                }
                Err(d.boxed())
            }
        }
    }

    // ----- calls -------------------------------------------------------

    /// Calls a function value with arguments.
    pub fn call(&mut self, f: Value, args: Vec<Value>, span: Span) -> Fallible<Value> {
        if stack_position().abs_diff(self.stack_base) > STACK_BUDGET.load(std::sync::atomic::Ordering::Relaxed)
            || self.depth >= MAX_DEPTH.load(std::sync::atomic::Ordering::Relaxed)
        {
            return Err(too_deep(self.depth, span));
        }
        self.depth += 1;
        let r = self.call_loop(f, args, span);
        self.depth -= 1;
        r
    }

    fn call_loop(&mut self, mut f: Value, mut args: Vec<Value>, mut span: Span) -> Fallible<Value> {
        loop {
            match f {
                Value::Func(clo) => {
                    let def = clo.def.clone();
                    let (ci, env) = self.enter(&clo, &def, args, span)?;
                    drop(clo);
                    let body = &def.clauses[ci].body;
                    let r = match &body.kind {
                        ExprKind::Block(stmts) => self.exec_block(stmts, env),
                        _ => self.eval_tail(body, &env),
                    };
                    match r {
                        Ok(Tail::Done(v)) => return Ok(v),
                        Ok(Tail::Call(nf, nargs, nspan)) => {
                            f = nf;
                            args = nargs;
                            span = nspan;
                        }
                        Err(Flow::Return(v)) => return Ok(v),
                        Err(Flow::Err(mut d)) => {
                            add_frame(&mut d, &def, span);
                            return Err(d);
                        }
                    }
                }
                Value::Builtin(b) => {
                    if args.len() < b.min || args.len() > b.max {
                        return Err(builtin_arity_error(b, args.len(), span));
                    }
                    return (b.func)(self, &args, span);
                }
                Value::Ctor(c) => {
                    if args.len() != c.fields.len() {
                        return Err(ctor_arity_error(&c, args.len(), span));
                    }
                    return Ok(Value::Variant(Rc::new(Variant { ctor: c, fields: args })));
                }
                other => return Err(not_callable(&other, span)),
            }
        }
    }

    /// Picks the first definition (clause) of a function that accepts the arguments.
    fn enter(&mut self, clo: &Rc<Closure>, def: &FnDef, args: Vec<Value>, span: Span) -> Fallible<(usize, Env)> {
        if args.len() != def.arity {
            return Err(fn_arity_error(def, args.len(), span));
        }
        let mut base = clo.env.clone();
        if let Some(n) = clo.self_name {
            base = base.bind(n, Value::Func(clo.clone()));
        }
        // Fast path: one definition whose parameters are plain names.
        if let [clause] = def.clauses.as_slice() {
            if clause.guard.is_none() && clause.params.iter().all(|p| matches!(p.kind, PatKind::Bind(_) | PatKind::Wildcard)) {
                let mut env = base;
                for (p, a) in clause.params.iter().zip(args) {
                    if let PatKind::Bind(s) = p.kind {
                        env = env.bind(s, a);
                    }
                }
                return Ok((0, env));
            }
        }
        let mut binds = std::mem::take(&mut self.scratch);
        'clauses: for (ci, clause) in def.clauses.iter().enumerate() {
            binds.clear();
            for (p, a) in clause.params.iter().zip(&args) {
                if !match_pattern(p, a, &mut binds) {
                    continue 'clauses;
                }
            }
            let mut env = base.clone();
            for (s, v) in binds.drain(..) {
                env = env.bind(s, v);
            }
            if let Some(g) = &clause.guard {
                self.scratch = binds;
                let gv = match self.eval(g, &env) {
                    Ok(v) => v,
                    Err(Flow::Err(d)) => return Err(d),
                    Err(Flow::Return(_)) => return Err(question_outside(g.span)),
                };
                binds = std::mem::take(&mut self.scratch);
                if !truth(&gv, g.span)? {
                    continue;
                }
            }
            self.scratch = binds;
            return Ok((ci, env));
        }
        self.scratch = binds;
        Err(no_clause(def, &args, span))
    }
}

// ----- helpers ----------------------------------------------------------

pub fn match_pattern(p: &Pattern, v: &Value, out: &mut Vec<(Sym, Value)>) -> bool {
    match (&p.kind, v) {
        (PatKind::Wildcard, _) => true,
        (PatKind::Bind(s), _) => {
            out.push((*s, v.clone()));
            true
        }
        (PatKind::Unit, Value::Unit) => true,
        (PatKind::Bool(a), Value::Bool(b)) => a == b,
        (PatKind::Int(a), Value::Int(b)) => a == b,
        (PatKind::Int(a), Value::Float(b)) => (*a as f64) == *b,
        (PatKind::Float(a), Value::Float(b)) => a == b,
        (PatKind::Float(a), Value::Int(b)) => *a == (*b as f64),
        (PatKind::Text(a), Value::Text(b)) => a == b,
        (PatKind::Ctor(name, pats), Value::Variant(var)) => {
            var.ctor.name == *name && pats.len() == var.fields.len() && pats.iter().zip(&var.fields).all(|(p, f)| match_pattern(p, f, out))
        }
        (PatKind::List(pats, rest), Value::List(l)) => {
            let n = pats.len();
            let len_ok = match rest {
                None => l.len() == n,
                Some(_) => l.len() >= n,
            };
            if !len_ok {
                return false;
            }
            if !pats.iter().zip(l.iter()).all(|(p, x)| match_pattern(p, x, out)) {
                return false;
            }
            if let Some(Some((s, _))) = rest {
                out.push((*s, Value::List(l.slice(n, l.len()))));
            }
            true
        }
        (PatKind::Record(fields), Value::Record(r)) => fields.iter().all(|(k, p)| r.get(k).is_some_and(|x| match_pattern(p, x, out))),
        (PatKind::Or(alts), _) => {
            let mark = out.len();
            for a in alts {
                if match_pattern(a, v, out) {
                    return true;
                }
                out.truncate(mark);
            }
            false
        }
        _ => false,
    }
}

fn unary(op: UnOp, v: &Value, span: Span, operand: Span) -> Fallible<Value> {
    match (op, v) {
        (UnOp::Neg, Value::Int(n)) => n.checked_neg().map(Value::Int).ok_or_else(|| overflow(span)),
        (UnOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
        (UnOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (UnOp::Neg, other) => {
            Err(Diagnostic::error("E005", t!("cannot use `-` on {}", "impossible d'utiliser `-` sur {}", other.describe()))
                .at(span)
                .boxed())
        }
        (UnOp::Not, other) => Err(condition_error(other, operand)),
    }
}

#[inline(never)]
fn fn_arity_error(def: &FnDef, got: usize, span: Span) -> Box<Diagnostic> {
    let name = def.name.map(|n| n.name().to_string()).unwrap_or_else(|| "fn".to_string());
    arity_error(&name, &def.signature(), def.arity, def.arity, got, span, Some(def.span))
}

#[inline(never)]
fn no_clause(def: &FnDef, args: &[Value], span: Span) -> Box<Diagnostic> {
    let name = def.name.map(|n| n.name().to_string()).unwrap_or_else(|| "fn".to_string());
    let shown: Vec<String> = args.iter().map(|a| a.repr()).collect();
    let mut d = Diagnostic::error(
        "E006",
        t!(
            "no definition of `{name}` accepts these arguments: {name}({})",
            "aucune définition de `{name}` n'accepte ces arguments : {name}({})",
            shown.join(", ")
        ),
    )
    .at(span)
    .label(t!("called here", "appelée ici"));
    for clause in def.clauses.iter().take(4) {
        d = d.secondary(clause.span, t!("does not match", "ne correspond pas"));
    }
    d.help(t!(
        "add a definition that handles this case, for example one that ends with a plain name or `_`",
        "ajoutez une définition qui traite ce cas, par exemple une qui se termine par un simple nom ou `_`"
    ))
    .boxed()
}

#[inline(never)]
fn too_deep(depth: usize, span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E014", t!("too many nested calls ({depth} calls in progress)", "trop d'appels imbriqués ({depth} appels en cours)"))
        .at(span)
        .label(t!("this call goes too deep", "cet appel va trop loin"))
        .help(t!(
            "a recursive function that never stops? Check that it always reaches a base case.\nOtherwise, make the recursive call the very last thing the function does\n(tail call), for example with an accumulator:\n  fn sum([], acc) = acc\n  fn sum([x, ..rest], acc) = sum(rest, acc + x)\nor use `fold`, `map`, `filter`, which have no limit.",
            "une fonction récursive qui ne s'arrête jamais ? Vérifiez qu'elle atteint toujours un cas de base.\nSinon, faites de l'appel récursif la toute dernière chose que fait la fonction\n(appel terminal), par exemple avec un accumulateur :\n  fn somme([], acc) = acc\n  fn somme([x, ..reste], acc) = somme(reste, acc + x)\nou utilisez `fold`, `map`, `filter`, qui n'ont pas de limite."
        ))
        .boxed()
}

#[inline(never)]
fn add_frame(d: &mut Diagnostic, def: &FnDef, span: Span) {
    let name = def.name.map(|n| n.name().to_string()).unwrap_or_default();
    d.push_frame(name, span);
}

#[inline(never)]
fn builtin_arity_error(b: &crate::builtins::Builtin, got: usize, span: Span) -> Box<Diagnostic> {
    arity_error(b.name, b.sig, b.min, b.max, got, span, None)
}

#[inline(never)]
fn ctor_arity_error(c: &CtorInfo, got: usize, span: Span) -> Box<Diagnostic> {
    let fields: Vec<String> = c.fields.iter().map(|f| f.name().to_string()).collect();
    let sig = format!("{}({})", c.name, fields.join(", "));
    arity_error(&c.name.name(), &sig, c.fields.len(), c.fields.len(), got, span, None)
}

#[inline(never)]
fn not_callable(v: &Value, span: Span) -> Box<Diagnostic> {
    let mut d = Diagnostic::error("E009", t!("{} is not a function", "{} n'est pas une fonction", v.describe()))
        .at(span)
        .label(t!("cannot be called", "ne peut pas être appelé"));
    if let Value::Variant(var) = v {
        if var.fields.is_empty() {
            d = d.help(t!(
                "`{0}` is a value, not a function: write `{0}` without parentheses",
                "`{0}` est une valeur, pas une fonction : écrivez `{0}` sans parenthèses",
                var.ctor.name
            ));
        }
    }
    d.boxed()
}

#[inline(never)]
fn no_case(v: &Value, span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E006", t!("no case of this `match` handles {}", "aucun cas de ce `match` ne traite {}", v.describe()))
        .at(span)
        .label(t!("this value", "cette valeur"))
        .help(t!(
            "add a case for this value, or a last case `_ -> ...` that handles everything else",
            "ajoutez un cas pour cette valeur, ou un dernier cas `_ -> ...` qui traite tout le reste"
        ))
        .boxed()
}

#[inline(never)]
fn try_error(v: &Value, span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "E016",
        t!("`?` needs an Option or a Result, but got {}", "`?` a besoin d'une Option ou d'un Result, mais a reçu {}", v.describe()),
    )
    .at(span)
    .help(t!(
        "`?` takes the value out of `Some(x)` / `Ok(x)`, and returns `None` / `Err(e)` from the current function",
        "`?` sort la valeur de `Some(x)` / `Ok(x)`, et renvoie `None` / `Err(e)` depuis la fonction en cours"
    ))
    .boxed()
}

fn pattern_mismatch(p: &Pattern, v: &Value) -> Box<Diagnostic> {
    Diagnostic::error("E006", t!("the value {} does not match this pattern", "la valeur {} ne correspond pas à ce motif", v.describe()))
        .at(p.span)
        .help(t!(
            "use `match` to handle the different possible shapes of a value",
            "utilisez `match` pour traiter les différentes formes possibles d'une valeur"
        ))
        .boxed()
}

fn question_outside(span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E016", t!("`?` can only be used inside a function", "`?` ne peut être utilisé que dans une fonction"))
        .at(span)
        .help(t!(
            "outside of a function, handle the value with `match` or `unwrap_or`",
            "en dehors d'une fonction, traitez la valeur avec `match` ou `unwrap_or`"
        ))
        .boxed()
}

fn spread_error(v: &Value, span: Span, list: bool) -> Box<Diagnostic> {
    let what = if list { t!("a list", "une liste") } else { t!("a record", "un enregistrement") };
    Diagnostic::error("E005", t!("`..` expects {what} here, but got {}", "`..` attend {what} ici, mais a reçu {}", v.describe()))
        .at(span)
        .boxed()
}

pub fn truth(v: &Value, span: Span) -> Fallible<bool> {
    match v {
        Value::Bool(b) => Ok(*b),
        other => Err(condition_error(other, span)),
    }
}

fn condition_error(v: &Value, span: Span) -> Box<Diagnostic> {
    let mut d = Diagnostic::error(
        "E005",
        t!(
            "a condition must be `true` or `false`, but this is {}",
            "une condition doit être `true` ou `false`, mais ceci est {}",
            v.describe()
        ),
    )
    .at(span)
    .label(t!("not a Bool", "pas un Bool"));
    d = match v {
        Value::Int(_) | Value::Float(_) => {
            d.help(t!("compare it explicitly, for example `x != 0` or `x > 0`", "comparez explicitement, par exemple `x != 0` ou `x > 0`"))
        }
        Value::List(_) | Value::Text(_) | Value::Record(_) => d.help(t!(
            "to test if it is empty, use `is_empty(x)` or `not is_empty(x)`",
            "pour tester si c'est vide, utilisez `is_empty(x)` ou `not is_empty(x)`"
        )),
        Value::Variant(_) => d.help(t!(
            "use `is_some(x)`, `is_ok(x)`, or `match` to look inside",
            "utilisez `is_some(x)`, `is_ok(x)`, ou `match` pour regarder à l'intérieur"
        )),
        _ => d,
    };
    d.boxed()
}

pub fn overflow(span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E012", t!("the result is too large for a whole number (Int)", "le résultat est trop grand pour un nombre entier (Int)"))
        .at(span)
        .help(t!(
            "whole numbers go from -9223372036854775808 to 9223372036854775807;\nfor bigger values, use decimal numbers (Float), like `2.0 ** 100`",
            "les nombres entiers vont de -9223372036854775808 à 9223372036854775807 ;\npour des valeurs plus grandes, utilisez des nombres décimaux (Float), comme `2.0 ** 100`"
        ))
        .boxed()
}

pub fn div_zero(span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E008", t!("division by zero", "division par zéro"))
        .at(span)
        .help(t!(
            "check the divisor first: `if b == 0 then ... else a / b`,\nor return a Result: `if b == 0 then Err(\"division by zero\") else Ok(a / b)`",
            "vérifiez d'abord le diviseur : `if b == 0 then ... else a / b`,\nou renvoyez un Result : `if b == 0 then Err(\"division par zéro\") else Ok(a / b)`"
        ))
        .boxed()
}

pub fn arity_error(name: &str, sig: &str, min: usize, max: usize, got: usize, span: Span, def: Option<Span>) -> Box<Diagnostic> {
    let s = if min == 1 { "" } else { "s" };
    let expected = if min == max {
        t!("{min} argument{s}", "{min} argument{s}")
    } else if max == usize::MAX {
        t!("at least {min} argument{s}", "au moins {min} argument{s}")
    } else {
        t!("{min} to {max} arguments", "de {min} à {max} arguments")
    };
    let verb = if got == 1 { "was given" } else { "were given" };
    let given = t!("{got} {verb}", "{got}");
    let mut d =
        Diagnostic::error("E004", t!("`{name}` takes {expected}, but {given}", "`{name}` attend {expected}, mais en a reçu {given}"))
            .at(span)
            .label(t!("{got} argument(s) here", "{got} argument(s) ici"))
            .help(t!("usage: `{sig}`", "utilisation : `{sig}`"));
    if let Some(def) = def {
        d = d.secondary(def, t!("defined here", "définie ici"));
    }
    d.boxed()
}

fn op_error(op: BinOp, l: &Value, r: &Value, span: Span) -> Box<Diagnostic> {
    let sym = op.symbol();
    let (lt, rt) = (l.type_name(), r.type_name());
    let mut d = Diagnostic::error("E005", t!("cannot use `{sym}` with {lt} and {rt}", "impossible d'utiliser `{sym}` avec {lt} et {rt}"))
        .at(span)
        .label(format!("{} {sym} {}", l.describe(), r.describe()));
    let is_text = |v: &Value| matches!(v, Value::Text(_));
    let is_list = |v: &Value| matches!(v, Value::List(_));
    let is_num = |v: &Value| matches!(v, Value::Int(_) | Value::Float(_));
    if op == BinOp::Add && (is_text(l) || is_text(r)) {
        d = d.help(t!(
            "to build a text with a value inside, use braces: \"total: {{x}}\"\nor convert the value first: text(x)",
            "pour construire un texte contenant une valeur, utilisez des accolades : \"total : {{x}}\"\nou convertissez d'abord la valeur : text(x)"
        ));
    } else if op == BinOp::Add && (is_list(l) || is_list(r)) {
        d = d.help(t!(
            "to add one element to a list, use `push(list, x)` or `[..list, x]`",
            "pour ajouter un élément à une liste, utilisez `push(liste, x)` ou `[..liste, x]`"
        ));
    } else if (is_text(l) && is_num(r)) || (is_num(l) && is_text(r)) {
        d = d.help(t!(
            "convert the text to a number first, with `int(t)` or `float(t)`",
            "convertissez d'abord le texte en nombre, avec `int(t)` ou `float(t)`"
        ));
    } else if op == BinOp::Mul && (is_text(l) || is_list(l)) {
        d = d.help(t!("to repeat a text or a list, use `repeat(x, n)`", "pour répéter un texte ou une liste, utilisez `repeat(x, n)`"));
    }
    d.boxed()
}

fn num(v: &Value) -> Option<f64> {
    match v {
        Value::Int(n) => Some(*n as f64),
        Value::Float(f) => Some(*f),
        _ => None,
    }
}

pub fn binop(op: BinOp, l: &Value, r: &Value, span: Span) -> Fallible<Value> {
    use Value::*;
    match op {
        BinOp::Eq => return Ok(Bool(l.equals(r))),
        BinOp::Ne => return Ok(Bool(!l.equals(r))),
        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
            let Some(o) = l.compare(r) else {
                let mut d = op_error(op, l, r, span);
                if l.type_name() != r.type_name() {
                    d.helps.push(t!(
                        "only values of the same kind can be compared",
                        "seules des valeurs de même nature peuvent être comparées"
                    ));
                }
                return Err(d);
            };
            use std::cmp::Ordering::*;
            return Ok(Bool(match op {
                BinOp::Lt => o == Less,
                BinOp::Le => o != Greater,
                BinOp::Gt => o == Greater,
                _ => o != Less,
            }));
        }
        _ => {}
    }
    match (l, r) {
        (Int(a), Int(b)) => {
            let (a, b) = (*a, *b);
            let res = match op {
                BinOp::Add => a.checked_add(b),
                BinOp::Sub => a.checked_sub(b),
                BinOp::Mul => a.checked_mul(b),
                BinOp::Div => {
                    if b == 0 {
                        return Err(div_zero(span));
                    }
                    return Ok(Float(a as f64 / b as f64));
                }
                BinOp::IntDiv => {
                    if b == 0 {
                        return Err(div_zero(span));
                    }
                    a.checked_div(b).map(|q| if (a % b != 0) && ((a < 0) != (b < 0)) { q - 1 } else { q })
                }
                BinOp::Mod => {
                    if b == 0 {
                        return Err(div_zero(span));
                    }
                    a.checked_rem(b).map(|m| if m != 0 && ((m < 0) != (b < 0)) { m + b } else { m })
                }
                BinOp::Pow => {
                    if b < 0 {
                        return Ok(Float((a as f64).powf(b as f64)));
                    }
                    u32::try_from(b).ok().and_then(|b| a.checked_pow(b))
                }
                _ => unreachable!(),
            };
            res.map(Int).ok_or_else(|| overflow(span))
        }
        (Int(_) | Float(_), Int(_) | Float(_)) => {
            let (a, b) = (num(l).unwrap(), num(r).unwrap());
            Ok(Float(match op {
                BinOp::Add => a + b,
                BinOp::Sub => a - b,
                BinOp::Mul => a * b,
                BinOp::Div => {
                    if b == 0.0 {
                        return Err(div_zero(span));
                    }
                    a / b
                }
                BinOp::IntDiv => {
                    if b == 0.0 {
                        return Err(div_zero(span));
                    }
                    (a / b).floor()
                }
                BinOp::Mod => {
                    if b == 0.0 {
                        return Err(div_zero(span));
                    }
                    a - b * (a / b).floor()
                }
                BinOp::Pow => a.powf(b),
                _ => unreachable!(),
            }))
        }
        (Text(a), Text(b)) if op == BinOp::Add => {
            let mut s = String::with_capacity(a.len() + b.len());
            s.push_str(a);
            s.push_str(b);
            Ok(Text(Rc::from(s)))
        }
        (List(a), List(b)) if op == BinOp::Add => {
            let mut v = Vec::with_capacity(a.len() + b.len());
            v.extend(a.iter().cloned());
            v.extend(b.iter().cloned());
            Ok(Value::list(v))
        }
        _ => Err(op_error(op, l, r, span)),
    }
}

fn range(a: &Value, b: &Value, span: Span) -> Fallible<Value> {
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => {
            if b < a {
                return Ok(Value::list(Vec::new()));
            }
            if b.saturating_sub(*a) >= MAX_RANGE {
                return Err(Diagnostic::error(
                    "E023",
                    t!(
                        "this range is too large ({} elements)",
                        "cet intervalle est trop grand ({} éléments)",
                        (*b as i128) - (*a as i128) + 1
                    ),
                )
                .at(span)
                .help(t!(
                    "ranges are lists: keep them under {MAX_RANGE} elements",
                    "les intervalles sont des listes : gardez-les sous {MAX_RANGE} éléments"
                ))
                .boxed());
            }
            Ok(Value::list((*a..=*b).map(Value::Int).collect()))
        }
        _ => Err(Diagnostic::error(
            "E005",
            t!(
                "a range needs whole numbers, but got {} and {}",
                "un intervalle a besoin de nombres entiers, mais a reçu {} et {}",
                a.describe(),
                b.describe()
            ),
        )
        .at(span)
        .help(t!("example: `1..10` is the list of the numbers from 1 to 10", "exemple : `1..10` est la liste des nombres de 1 à 10"))
        .boxed()),
    }
}

fn index(o: &Value, i: &Value, span: Span, idx_span: Span) -> Fallible<Value> {
    let out_of_range = |n: i64, len: usize, what: String| {
        let mut d = Diagnostic::error("E007", t!("index {n} is out of range: {what}", "l'indice {n} est hors limites : {what}"))
            .at(idx_span)
            .label(t!("no element here", "aucun élément ici"));
        if len > 0 {
            d = d.help(t!(
                "valid indices go from 0 to {} (or -1 for the last element)",
                "les indices valides vont de 0 à {} (ou -1 pour le dernier élément)",
                len - 1
            ));
        }
        d.help(t!(
            "to get an Option instead of an error, use `get(x, {n})`",
            "pour obtenir une Option au lieu d'une erreur, utilisez `get(x, {n})`"
        ))
        .boxed()
    };
    match (o, i) {
        (Value::List(l), Value::Int(n)) => {
            let len = l.len();
            match resolve_index(*n, len) {
                Some(k) => Ok(l.as_slice()[k].clone()),
                None => Err(out_of_range(
                    *n,
                    len,
                    if len == 0 {
                        t!("the list is empty", "la liste est vide")
                    } else {
                        t!("the list has {len} element(s)", "la liste a {len} élément(s)")
                    },
                )),
            }
        }
        (Value::Text(s), Value::Int(n)) => {
            let len = s.chars().count();
            match resolve_index(*n, len) {
                Some(k) => Ok(Value::text(&s.chars().nth(k).unwrap().to_string())),
                None => Err(out_of_range(
                    *n,
                    len,
                    if len == 0 {
                        t!("the text is empty", "le texte est vide")
                    } else {
                        t!("the text has {len} character(s)", "le texte a {len} caractère(s)")
                    },
                )),
            }
        }
        (Value::Record(r), Value::Text(k)) => r.get(k).cloned().ok_or_else(|| {
            Diagnostic::error("E010", t!("this record has no field `{k}`", "cet enregistrement n'a pas de champ `{k}`"))
                .at(idx_span)
                .help(t!(
                    "use `get(record, key)` to receive an Option instead",
                    "utilisez `get(enregistrement, clé)` pour obtenir une Option à la place"
                ))
                .boxed()
        }),
        (Value::List(_) | Value::Text(_), other) => Err(Diagnostic::error(
            "E005",
            t!(
                "an index must be a whole number (Int), but got {}",
                "un indice doit être un nombre entier (Int), mais a reçu {}",
                other.describe()
            ),
        )
        .at(idx_span)
        .boxed()),
        (other, _) => {
            Err(Diagnostic::error("E005", t!("cannot use `[...]` on {}", "impossible d'utiliser `[...]` sur {}", other.describe()))
                .at(span)
                .help(t!(
                    "`[...]` works on lists, texts and records",
                    "`[...]` fonctionne sur les listes, les textes et les enregistrements"
                ))
                .boxed())
        }
    }
}

/// Turns a possibly negative index into a position, if it is in range.
pub fn resolve_index(n: i64, len: usize) -> Option<usize> {
    let len = len as i64;
    let k = if n < 0 { len + n } else { n };
    if k >= 0 && k < len {
        Some(k as usize)
    } else {
        None
    }
}
