//! The interpreter: evaluates the syntax tree, once `compile` has turned its
//! local names into numbered slots of the frames of the value stack.
//!
//! Calls in tail position (the last thing a function does) do not grow the
//! stack, so recursive functions can loop millions of times.

use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::*;
use crate::bigint::{self, BigInt};
use crate::builtins::BUILTINS;
use crate::compile::{self, Capture, FnCode, Kind, Node, Pat, Scope, Sibling};
use crate::diagnostic::{Diagnostic, Fallible};
use crate::source::Span;
use crate::suggest;
use crate::symbol::{Sym, SymMap};
use crate::value::*;

/// Why the evaluation of an expression stopped early.
pub enum Flow {
    Err(Box<Diagnostic>),
    /// `?` found a `None` or an `Err`: the current function returns it.
    /// (Boxed: it is rare, and it keeps the results of evaluation small.)
    Return(Box<Value>),
}

impl From<Box<Diagnostic>> for Flow {
    fn from(d: Box<Diagnostic>) -> Flow {
        Flow::Err(d)
    }
}

pub type Eval = Result<Value, Flow>;

/// Result of an expression in tail position: a value, or a call still to
/// make, whose arguments are on top of the stack.
enum Tail {
    Done(Value),
    Call(Value, usize, Span),
    /// A call on the right of `and` / `or`: its result must be true or false.
    Test(Value, usize, Span),
}

/// The running function: where its slots start on the stack, the values its
/// closure captured, and the functions of its group.
struct Frame<'a> {
    base: usize,
    captured: &'a [Value],
    siblings: Option<&'a Rc<[Sibling]>>,
}

#[derive(Clone)]
pub struct TestCase {
    pub name: Rc<str>,
    pub body: Rc<Expr>,
    pub span: Span,
    /// How the names of its file are stored.
    pub names: compile::Names,
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

/// The longest list (in elements) and text (in bytes) a program may build.
/// Beyond, the computer would run out of memory, and Rythme would stop
/// without a word; these limits stop a list or a text that grows without end.
pub const MAX_LIST: usize = 50_000_000;
pub const MAX_TEXT: usize = 100_000_000;

/// A list (or a text) would be longer than `MAX_LIST` (`MAX_TEXT`).
#[inline(never)]
pub fn too_big(list: bool, span: Span) -> Box<Diagnostic> {
    let msg = if list {
        t!("this list would be too large (more than {MAX_LIST} elements)", "cette liste serait trop grande (plus de {MAX_LIST} éléments)")
    } else {
        let mb = MAX_TEXT / 1_000_000;
        t!("this text would be too large (more than {mb} MB)", "ce texte serait trop grand (plus de {mb} Mo)")
    };
    Diagnostic::error("E023", msg)
        .at(span)
        .help(t!(
            "does it grow in a function that calls itself? Check that the function always reaches a case where it stops",
            "grandit-il dans une fonction qui s'appelle elle-même ? Vérifiez que la fonction atteint toujours un cas où elle s'arrête"
        ))
        .boxed()
}

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
    /// Where the values of `check_all` and `sample` come from (`rythme test --seed N`).
    pub property_seed: u64,
    /// How the top-level names of the file being run are stored (see
    /// `compile::Names`); set by the driver before running each file.
    pub names: compile::Names,
    depth: usize,
    stack_base: usize,
    /// The frames of the running functions, one after the other: their
    /// arguments and local names, then the arguments of the calls being prepared.
    stack: Vec<Value>,
    /// The stack of the functions that a builtin calls, kept between calls.
    spare: Vec<Value>,
    /// Top-level names of the running file that are not defined yet.
    pending: SymMap<Span>,
    /// Calls that may still be made (see `set_step_limit`).
    steps_left: u64,
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
        let ctor = |t: Sym, n: &str, fields: &[&str], index: usize| {
            Rc::new(CtorInfo { type_name: t, name: Sym::new(n), fields: fields.iter().map(|f| Sym::new(f)).collect(), index, doc: None })
        };
        // In the order of `<`, as in Rust: `None < Some(x)`, `Ok(x) < Err(e)`.
        let some = ctor(option, "Some", &["value"], 1);
        let none_info = ctor(option, "None", &[], 0);
        let ok = ctor(result, "Ok", &["value"], 0);
        let err = ctor(result, "Err", &["error"], 1);
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
            property_seed: crate::builtins::property::DEFAULT_SEED,
            names: compile::Names::default(),
            depth: 0,
            stack_base: stack_position(),
            stack: Vec::new(),
            spare: Vec::new(),
            pending: SymMap::default(),
            steps_left: u64::MAX,
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

    /// Stops the evaluation with an error after `calls` more function calls, so
    /// that any program, even one that never ends, can be run safely (tests).
    pub fn set_step_limit(&mut self, calls: u64) {
        self.steps_left = calls;
    }

    /// The calls allowed by `set_step_limit` are all made.
    pub fn out_of_steps(&self) -> bool {
        self.steps_left == 0
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
        for (index, v) in t.variants.iter().enumerate() {
            let info = Rc::new(CtorInfo { type_name: t.name, name: v.name, fields: v.fields.clone(), index, doc: t.doc.clone() });
            let value = if v.fields.is_empty() {
                Value::Variant(Rc::new(Variant { ctor: info.clone(), fields: Vec::new() }))
            } else {
                Value::Ctor(info.clone())
            };
            self.globals.insert(self.stored(v.name), value);
            ctors.push(info);
        }
        self.types.insert(self.stored(t.name), ctors);
    }

    pub fn define_fn(&mut self, def: &Rc<FnDef>) {
        if let Some(name) = def.name {
            let clo =
                Closure { def: def.clone(), code: Rc::new(compile::function(def, &self.names)), captured: Vec::new(), siblings: None };
            self.globals.insert(self.stored(name), Value::Func(Rc::new(clo)));
        }
    }

    /// The name under which a top-level name of the running file is stored.
    pub fn stored(&self, s: Sym) -> Sym {
        self.names.get(&s).copied().unwrap_or(s)
    }

    /// Runs the items of a module (its `use` dependencies must already have run).
    pub fn exec_items(&mut self, items: &[Item], mode: Mode) -> Fallible<Option<Value>> {
        self.define_items(items);
        let mut last = None;
        for item in items {
            last = None;
            match item {
                Item::Stmt(Stmt::Let(pat, e)) => {
                    let v = self.eval_top(e)?;
                    self.bind_global(pat, v)?;
                }
                Item::Stmt(Stmt::Expr(e)) => {
                    if mode != Mode::Test {
                        last = Some(self.eval_top(e)?);
                    }
                }
                Item::Test(name, body, span) if mode == Mode::Test => {
                    self.tests.push(TestCase { name: name.clone(), body: body.clone(), span: *span, names: self.names.clone() });
                }
                _ => {}
            }
        }
        Ok(last)
    }

    /// Defines the types and functions of a module, and notes the names its
    /// top-level definitions will give (for "not defined yet").
    pub fn define_items(&mut self, items: &[Item]) {
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
                let names: Vec<(Sym, Span)> = names.into_iter().map(|(s, sp)| (self.stored(s), sp)).collect();
                self.pending.extend(names.into_iter().filter(|(s, _)| !self.globals.contains_key(s)));
            }
        }
    }

    /// Gives the names of a top-level definition `pattern = value`.
    pub fn bind_global(&mut self, pat: &Pattern, v: Value) -> Fallible<()> {
        let mut binds = Vec::new();
        if !match_pattern(pat, &v, &mut binds) {
            return Err(pattern_mismatch(pat.span, &v));
        }
        for (s, v) in binds {
            let s = self.stored(s);
            self.pending.remove(&s);
            self.globals.insert(s, v);
        }
        Ok(())
    }

    /// Evaluates an expression outside of any function.
    pub fn eval_top(&mut self, e: &Expr) -> Fallible<Value> {
        self.depth = 0;
        let code = compile::top(e, &self.names);
        let base = self.push_frame(code.slots);
        let r = self.eval(&code.node, &Frame { base, captured: &[], siblings: None });
        self.end_top(base);
        match r {
            Ok(v) => Ok(v),
            Err(Flow::Err(d)) => Err(d),
            Err(Flow::Return(_)) => Err(question_outside(e.span)),
        }
    }

    /// Evaluates the body of a test (like a function body: `?` is allowed).
    pub fn eval_test(&mut self, test: &TestCase) -> Fallible<Value> {
        self.depth = 0;
        let e = &*test.body;
        let code = compile::top(e, &test.names);
        let base = self.push_frame(code.slots);
        let r = match self.eval_tail(&code.node, &Frame { base, captured: &[], siblings: None }) {
            Ok(Tail::Done(v)) => Ok(v),
            Ok(Tail::Call(f, argc, span)) => self.call_value(f, argc, span),
            Ok(Tail::Test(f, argc, span)) => self.call_value(f, argc, span).and_then(|v| truth(&v, span).map(Value::Bool)),
            Err(Flow::Return(v)) => Ok(*v),
            Err(Flow::Err(d)) => Err(d),
        };
        self.end_top(base);
        r
    }

    /// Removes the frame of a top-level expression. Back at the top level,
    /// gives back the memory that a deep recursion used.
    fn end_top(&mut self, base: usize) {
        self.stack.truncate(base);
        if base == 0 && self.stack.capacity() > 1 << 16 {
            self.stack = Vec::new();
            self.spare = Vec::new();
        }
    }

    /// Removes the values above `len` from the stack.
    #[inline(always)]
    fn pop_to(&mut self, len: usize) {
        while self.stack.len() > len {
            if let Some(v) = self.stack.pop() {
                discard(v);
            }
        }
    }

    /// Adds a frame of `slots` local names on top of the stack.
    fn push_frame(&mut self, slots: usize) -> usize {
        let base = self.stack.len();
        self.stack.resize_with(base + slots, || Value::Unit);
        base
    }

    // ----- evaluation --------------------------------------------------

    /// Evaluates an expression. The common cases are kept here and the rare
    /// ones live in `eval_rest`, so that each nested call uses little stack.
    fn eval(&mut self, e: &Node, fr: &Frame) -> Eval {
        match &e.kind {
            Kind::Local(slot) => Ok(self.stack[fr.base + *slot as usize].clone()),
            Kind::Const(v) => Ok(v.clone()),
            Kind::Captured(i) => Ok(fr.captured[*i as usize].clone()),
            Kind::Global(s, scope) => match self.globals.get(s) {
                Some(v) => Ok(v.clone()),
                None => Err(self.unknown_name(*s, scope, e.span).into()),
            },
            Kind::Call(callee, args) => {
                let f = self.operand(callee, fr)?;
                self.push_args(args, fr)?;
                Ok(self.call_value(f, args.len(), e.span)?)
            }
            Kind::Method(_) | Kind::If(..) | Kind::Match(..) | Kind::Block(..) => match self.eval_tail(e, fr)? {
                Tail::Done(v) => Ok(v),
                Tail::Call(f, argc, span) => Ok(self.call_value(f, argc, span)?),
                Tail::Test(f, argc, span) => Ok(Value::Bool(truth(&self.call_value(f, argc, span)?, span)?)),
            },
            Kind::Binary(op, l, r, op_span) => {
                // Local names and constants are read in place, without a copy:
                // they cannot fail, so reading them last changes nothing.
                let lv = if is_leaf(l) { None } else { Some(self.eval(l, fr)?) };
                let rv = if is_leaf(r) { None } else { Some(self.eval(r, fr)?) };
                let a = match &lv {
                    Some(v) => v,
                    None => self.leaf(l, fr),
                };
                let b = match &rv {
                    Some(v) => v,
                    None => self.leaf(r, fr),
                };
                if let (Value::Int(x), Value::Int(y)) = (a, b) {
                    if let Some(v) = int_op(*op, *x, *y) {
                        return Ok(v);
                    }
                }
                Ok(binop(*op, a, b, *op_span)?)
            }
            _ => self.eval_rest(e, fr),
        }
    }

    /// The value of a local name or a constant (see `is_leaf`), in place.
    #[inline(always)]
    fn leaf<'a>(&'a self, e: &'a Node, fr: &'a Frame) -> &'a Value {
        match &e.kind {
            Kind::Local(slot) => &self.stack[fr.base + *slot as usize],
            Kind::Captured(i) => &fr.captured[*i as usize],
            Kind::Const(v) => v,
            _ => unreachable!(),
        }
    }

    /// Evaluates an operand: names and constants without a nested call.
    #[inline(always)]
    fn operand(&mut self, e: &Node, fr: &Frame) -> Eval {
        match &e.kind {
            Kind::Local(slot) => Ok(self.stack[fr.base + *slot as usize].clone()),
            Kind::Const(v) => Ok(v.clone()),
            Kind::Global(s, _) => match self.globals.get(s) {
                Some(v) => Ok(v.clone()),
                None => self.eval(e, fr),
            },
            _ => self.eval(e, fr),
        }
    }

    #[inline(never)]
    fn eval_rest(&mut self, e: &Node, fr: &Frame) -> Eval {
        match &e.kind {
            // Building values happens in functions of their own, so that
            // each nested evaluation through here uses little stack.
            Kind::Template(parts) => self.template(parts, e.span, fr),
            Kind::List(items) => self.list(items, e.span, fr),
            Kind::NewRecord(shape, values) => self.new_record(shape, values, fr),
            Kind::Record(items) => self.record(items, fr),
            Kind::Lambda(def, code) => Ok(self.closure(def, code, fr)),
            Kind::Sibling(i) => Ok(sibling(fr, *i)),
            // Local names and constants are read in place (see `Kind::Binary`).
            Kind::Field(obj, name, nspan) if is_leaf(obj) => Ok(self.field(self.leaf(obj, fr), name, *nspan)?),
            Kind::Field(obj, name, nspan) => {
                let v = self.eval(obj, fr)?;
                Ok(self.field(&v, name, *nspan)?)
            }
            Kind::Index(obj, idx) => self.index(obj, idx, e.span, fr),
            Kind::Unary(op, x) => {
                let v = self.eval(x, fr)?;
                Ok(unary(*op, &v, e.span, x.span)?)
            }
            Kind::Logic(op, l, r) => {
                // `and` / `or`: the right side is only evaluated when needed.
                let left = self.condition(l, fr)?;
                if (*op == BinOp::And) != left {
                    return Ok(Value::Bool(left));
                }
                Ok(Value::Bool(self.condition(r, fr)?))
            }
            Kind::Try(x) => {
                let v = self.eval(x, fr)?;
                match self.wrap(&v) {
                    Wrap::Some | Wrap::Ok => Ok(Interp::payload(&v)),
                    Wrap::None | Wrap::Err => Err(Flow::Return(Box::new(v))),
                    Wrap::Other => Err(try_error(&v, e.span).into()),
                }
            }
            Kind::Range(a, b) => {
                let av = self.eval(a, fr)?;
                let bv = self.eval(b, fr)?;
                Ok(range(&av, &bv, e.span)?)
            }
            Kind::Const(_)
            | Kind::Local(_)
            | Kind::Captured(_)
            | Kind::Global(..)
            | Kind::Call(..)
            | Kind::Method(_)
            | Kind::Binary(..)
            | Kind::If(..)
            | Kind::Match(..)
            | Kind::Block(..) => self.eval(e, fr),
        }
    }

    /// `"... {x} ..."`
    #[inline(never)]
    fn template(&mut self, parts: &[compile::Part], span: Span, fr: &Frame) -> Eval {
        let mut s = String::new();
        for p in parts {
            match p {
                compile::Part::Lit(l) => s.push_str(l),
                compile::Part::Expr(x) if is_leaf(x) => self.leaf(x, fr).write_display(&mut s),
                compile::Part::Expr(x) => self.eval(x, fr)?.write_display(&mut s),
            }
            if s.len() > MAX_TEXT {
                return Err(too_big(false, span).into());
            }
        }
        Ok(Value::Text(Rc::from(s)))
    }

    /// `[a, ..xs, b]`
    #[inline(never)]
    fn list(&mut self, items: &[compile::Item], span: Span, fr: &Frame) -> Eval {
        let mut v = Vec::with_capacity(items.len());
        for it in items {
            match it {
                compile::Item::One(x) => v.push(self.eval(x, fr)?),
                compile::Item::Spread(x) => match self.eval(x, fr)? {
                    Value::List(l) if v.len() + l.len() > MAX_LIST => return Err(too_big(true, span).into()),
                    Value::List(l) => v.extend(l.iter().cloned()),
                    other => return Err(spread_error(&other, x.span, true).into()),
                },
            }
        }
        Ok(Value::list(v))
    }

    /// `{a: x, b: y}`: different names and no `..`.
    #[inline(never)]
    fn new_record(&mut self, shape: &Rc<Shape>, values: &[Node], fr: &Frame) -> Eval {
        let mut vals = Vec::with_capacity(values.len());
        for x in values {
            vals.push(self.operand(x, fr)?);
        }
        Ok(Value::Record(Rc::new(Record::with_shape(shape.clone(), vals))))
    }

    /// `{..r, a: x}`, or names given twice.
    #[inline(never)]
    fn record(&mut self, items: &[compile::Field], fr: &Frame) -> Eval {
        let mut r = Record::new();
        for (i, it) in items.iter().enumerate() {
            match it {
                compile::Field::Field(k, x) => {
                    let v = self.eval(x, fr)?;
                    r.set(k.clone(), v);
                }
                compile::Field::Spread(x) => match self.eval(x, fr)? {
                    // `{..r, ...}`: a copy of `r` to start with, which shares its names.
                    Value::Record(src) if i == 0 => r = (*src).clone(),
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

    /// `x[i]`
    #[inline(never)]
    fn index(&mut self, obj: &Node, idx: &Node, span: Span, fr: &Frame) -> Eval {
        let o = if is_leaf(obj) { None } else { Some(self.eval(obj, fr)?) };
        let i = if is_leaf(idx) { None } else { Some(self.eval(idx, fr)?) };
        let o = match &o {
            Some(v) => v,
            None => self.leaf(obj, fr),
        };
        let i = match &i {
            Some(v) => v,
            None => self.leaf(idx, fr),
        };
        Ok(index(o, i, span, idx.span)?)
    }

    /// Evaluates an operation on names or constants (`x * 2`, the body of
    /// many functions given to `map`), computed on the spot for whole numbers.
    #[inline(never)]
    fn leaf_operation(&mut self, e: &Node, fr: &Frame) -> Eval {
        if let Kind::Binary(op, l, r, _) = &e.kind {
            if is_leaf(l) && is_leaf(r) {
                if let (Value::Int(x), Value::Int(y)) = (self.leaf(l, fr), self.leaf(r, fr)) {
                    if let Some(v) = int_op(*op, *x, *y) {
                        return Ok(v);
                    }
                }
            }
        }
        self.eval(e, fr)
    }

    /// Evaluates a condition: `true` or `false`.
    #[inline(never)]
    fn condition(&mut self, c: &Node, fr: &Frame) -> Result<bool, Flow> {
        // `n == 0`, `x < v`: whole numbers given by names or constants.
        if let Kind::Binary(op, l, r, _) = &c.kind {
            if is_leaf(l) && is_leaf(r) {
                if let (Value::Int(a), Value::Int(b)) = (self.leaf(l, fr), self.leaf(r, fr)) {
                    match op {
                        BinOp::Eq => return Ok(a == b),
                        BinOp::Ne => return Ok(a != b),
                        BinOp::Lt => return Ok(a < b),
                        BinOp::Le => return Ok(a <= b),
                        BinOp::Gt => return Ok(a > b),
                        BinOp::Ge => return Ok(a >= b),
                        _ => {}
                    }
                }
            }
        }
        let v = self.eval(c, fr)?;
        match v {
            Value::Bool(b) => {
                discard(v);
                Ok(b)
            }
            other => Err(condition_error(&other, c.span).into()),
        }
    }

    /// Evaluates an expression in tail position: a call there is not made but
    /// returned, with its arguments on top of the stack.
    fn eval_tail(&mut self, mut e: &Node, fr: &Frame) -> Result<Tail, Flow> {
        loop {
            match &e.kind {
                // `a and f(x)`: when the right side is a call, it is a tail call.
                Kind::Logic(op, l, r) if matches!(r.kind, Kind::Call(..) | Kind::Method(_)) => {
                    let left = self.condition(l, fr)?;
                    if (*op == BinOp::And) != left {
                        return Ok(Tail::Done(Value::Bool(left)));
                    }
                    // Still a tail call, but its result must be true or false.
                    return match self.eval_tail(r, fr)? {
                        Tail::Call(f, argc, span) | Tail::Test(f, argc, span) => Ok(Tail::Test(f, argc, span)),
                        Tail::Done(v) => Ok(Tail::Done(Value::Bool(truth(&v, r.span)?))),
                    };
                }
                Kind::If(c, a, b) => e = if self.condition(c, fr)? { a } else { b },
                Kind::Match(subject, arms) => e = self.choose_arm(subject, arms, fr)?,
                Kind::Block(stmts, last) => {
                    self.exec_stmts(stmts, fr)?;
                    match last {
                        Some(x) => e = x,
                        None => return Ok(Tail::Done(Value::Unit)),
                    }
                }
                Kind::Call(callee, args) => {
                    let f = self.operand(callee, fr)?;
                    self.push_args(args, fr)?;
                    return Ok(Tail::Call(f, args.len(), e.span));
                }
                Kind::Method(m) => return self.method_call(m, e.span, fr),
                _ => return Ok(Tail::Done(self.eval(e, fr)?)),
            }
        }
    }

    /// The body of the first case of a `match` that accepts the value.
    // In a browser, the stack is small: what is not part of every nested
    // call stays out of the functions that are.
    #[cfg_attr(target_arch = "wasm32", inline(never))]
    fn choose_arm<'n>(&mut self, subject: &Node, arms: &'n [compile::Arm], fr: &Frame) -> Result<&'n Node, Flow> {
        let v = self.eval(subject, fr)?;
        for arm in arms {
            if !may_match(&arm.pat, &v) || !match_pat(&arm.pat, &v, &mut self.stack[fr.base..], 0) {
                continue;
            }
            if let Some(g) = &arm.guard {
                if !self.condition(g, fr)? {
                    continue;
                }
            }
            return Ok(&arm.body);
        }
        Err(no_case(&v, subject.span).into())
    }

    /// Runs the statements of a block (all but its last expression).
    #[cfg_attr(target_arch = "wasm32", inline(never))]
    fn exec_stmts(&mut self, stmts: &[compile::Stmt], fr: &Frame) -> Result<(), Flow> {
        for s in stmts {
            match s {
                compile::Stmt::Set(slot, x) => {
                    let v = self.eval(x, fr)?;
                    set(&mut self.stack[fr.base + *slot as usize], v);
                }
                compile::Stmt::Let(p, x, span) => {
                    let v = self.eval(x, fr)?;
                    if !match_pat(p, &v, &mut self.stack[fr.base..], 0) {
                        return Err(pattern_mismatch(*span, &v).into());
                    }
                }
                compile::Stmt::Expr(x) => {
                    self.eval(x, fr)?;
                }
                compile::Stmt::Fn(slot, def, code) => {
                    let f = self.closure(def, code, fr);
                    set(&mut self.stack[fr.base + *slot as usize], f);
                }
                compile::Stmt::Group(slots, group) => {
                    // They all capture the same values.
                    let captured = self.captures(&group[0].code, fr);
                    for &(slot, i) in slots {
                        let Sibling { def, code } = &group[i as usize];
                        let clo =
                            Closure { def: def.clone(), code: code.clone(), captured: captured.clone(), siblings: Some(group.clone()) };
                        set(&mut self.stack[fr.base + slot as usize], Value::Func(Rc::new(clo)));
                    }
                }
            }
        }
        Ok(())
    }

    /// Evaluates the arguments of a call onto the stack.
    fn push_args(&mut self, args: &[Node], fr: &Frame) -> Result<(), Flow> {
        for a in args {
            let v = match &a.kind {
                // `n - 1`, `acc + x`: computed on the spot.
                Kind::Binary(op, l, r, _) if is_leaf(l) && is_leaf(r) => match (self.leaf(l, fr), self.leaf(r, fr)) {
                    (Value::Int(x), Value::Int(y)) => match int_op(*op, *x, *y) {
                        Some(v) => v,
                        None => self.eval(a, fr)?,
                    },
                    _ => self.eval(a, fr)?,
                },
                _ => self.operand(a, fr)?,
            };
            self.stack.push(v);
        }
        Ok(())
    }

    /// A new closure, with the values it captures from the current frame.
    fn closure(&self, def: &Rc<FnDef>, code: &Rc<FnCode>, fr: &Frame) -> Value {
        Value::Func(Rc::new(Closure { def: def.clone(), code: code.clone(), captured: self.captures(code, fr), siblings: None }))
    }

    /// The values a new closure of `code` captures from the current frame.
    fn captures(&self, code: &FnCode, fr: &Frame) -> Vec<Value> {
        code.captures
            .iter()
            .map(|c| match *c {
                Capture::Local(slot) => self.stack[fr.base + slot as usize].clone(),
                Capture::Captured(i) => fr.captured[i as usize].clone(),
                Capture::Sibling(i) => sibling(fr, i),
            })
            .collect()
    }

    /// `value.name(args)`: a function stored in a record field, or `name(value, args)`.
    #[inline(never)]
    fn method_call(&mut self, m: &compile::Method, span: Span, fr: &Frame) -> Result<Tail, Flow> {
        let rv = self.eval(&m.recv, fr)?;
        let field = match &rv {
            Value::Record(r) => r.get(&m.key).cloned(),
            _ => None,
        };
        let (f, argc) = match field {
            Some(f) => (f, m.args.len()),
            None => {
                let f = match &m.func.kind {
                    Kind::Global(name, scope) => match self.globals.get(name) {
                        Some(f) => f.clone(),
                        None => {
                            let name = m.name;
                            let mut d = *self.unknown_name(name, scope, m.name_span);
                            d.message = t!("no function named `{}`", "aucune fonction nommée `{}`", name);
                            d.notes.push(t!(
                                "`value.{0}(...)` calls `{0}(value, ...)`",
                                "`valeur.{0}(...)` appelle `{0}(valeur, ...)`",
                                name
                            ));
                            return Err(d.boxed().into());
                        }
                    },
                    _ => self.eval(&m.func, fr)?,
                };
                self.stack.push(rv);
                (f, m.args.len() + 1)
            }
        };
        self.push_args(&m.args, fr)?;
        Ok(Tail::Call(f, argc, span))
    }

    pub fn unknown_name(&self, s: Sym, scope: &Scope, span: Span) -> Box<Diagnostic> {
        let name = s.shown().name();
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
            let names: Vec<Rc<str>> =
                scope.names().into_iter().chain(self.globals.keys().copied().filter(|s| !s.is_stored())).map(|s| s.name()).collect();
            if let Some(best) = suggest::closest(&name, names.iter().map(|s| &**s)) {
                d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
            }
        }
        d.boxed()
    }

    pub(crate) fn field(&self, v: &Value, name: &str, span: Span) -> Fallible<Value> {
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
        let argc = args.len();
        self.stack.extend(args);
        self.call_value(f, argc, span)
    }

    /// Calls a function value with one argument (like `call`, without a vector).
    pub fn call1(&mut self, f: &Value, a: Value, span: Span) -> Fallible<Value> {
        self.stack.push(a);
        self.call_value(f.clone(), 1, span)
    }

    /// Calls a function value with two arguments (like `call`, without a vector).
    pub fn call2(&mut self, f: &Value, a: Value, b: Value, span: Span) -> Fallible<Value> {
        self.stack.push(a);
        self.stack.push(b);
        self.call_value(f.clone(), 2, span)
    }

    /// Calls a function value with the `argc` values on top of the stack as
    /// arguments, and removes them.
    fn call_value(&mut self, f: Value, argc: usize, span: Span) -> Fallible<Value> {
        let base = self.stack.len() - argc;
        if stack_position().abs_diff(self.stack_base) > STACK_BUDGET.load(std::sync::atomic::Ordering::Relaxed)
            || self.depth >= MAX_DEPTH.load(std::sync::atomic::Ordering::Relaxed)
        {
            self.stack.truncate(base);
            return Err(too_deep(self.depth, span));
        }
        self.depth += 1;
        let mut test = None;
        let r = self.call_loop(f, base, argc, span, &mut test);
        self.depth -= 1;
        self.pop_to(base);
        match test {
            Some(at) => r.and_then(|v| truth(&v, at).map(Value::Bool)),
            None => r,
        }
    }

    /// Runs a call whose frame starts at `base`, with the arguments in its
    /// first slots. A call in tail position reuses the frame. `test` is set
    /// when the result must be true or false.
    fn call_loop(&mut self, mut f: Value, base: usize, mut argc: usize, mut span: Span, test: &mut Option<Span>) -> Fallible<Value> {
        loop {
            if self.steps_left == 0 {
                return Err(out_of_steps(span));
            }
            self.steps_left -= 1;
            match f {
                Value::Func(clo) => {
                    let code = &*clo.code;
                    if argc != code.arity {
                        return Err(fn_arity_error(&clo.def, argc, span));
                    }
                    if code.slots > argc {
                        self.stack.resize_with(base + code.slots, || Value::Unit);
                    }
                    let fr = Frame { base, captured: &clo.captured, siblings: clo.siblings.as_ref() };
                    let ci = if code.simple && code.self_slot.is_none() {
                        0
                    } else {
                        match self.enter(&clo, &fr, span) {
                            Ok(ci) => ci,
                            // A `?` in a guard ended the call.
                            Err(Flow::Return(v)) => return Ok(*v),
                            Err(Flow::Err(d)) => return Err(d),
                        }
                    };
                    let body = &code.clauses[ci].body;
                    let r = match body.kind {
                        Kind::Call(..) | Kind::Method(_) | Kind::If(..) | Kind::Match(..) | Kind::Block(..) | Kind::Logic(..) => {
                            self.eval_tail(body, &fr)
                        }
                        // A body that cannot end with a call (`x * 2`).
                        Kind::Binary(_, ref l, ref r, _) if is_leaf(l) && is_leaf(r) => self.leaf_operation(body, &fr).map(Tail::Done),
                        _ => self.eval(body, &fr).map(Tail::Done),
                    };
                    let (nf, nargc, nspan) = match r {
                        Ok(Tail::Done(v)) => return Ok(v),
                        Ok(Tail::Call(nf, nargc, nspan)) => (nf, nargc, nspan),
                        Ok(Tail::Test(nf, nargc, nspan)) => {
                            *test = Some(nspan);
                            (nf, nargc, nspan)
                        }
                        Err(Flow::Return(v)) => return Ok(*v),
                        Err(Flow::Err(mut d)) => {
                            add_frame(&mut d, &clo.def, span);
                            return Err(d);
                        }
                    };
                    let args = self.stack.len() - nargc;
                    // A value built last (`Node(l, v, r)`) takes its
                    // fields where they are; `call_value` drops the frame.
                    if let Value::Ctor(c) = &nf {
                        if c.fields.len() == nargc {
                            let fields = self.stack.split_off(args);
                            return Ok(Value::Variant(Rc::new(Variant { ctor: c.clone(), fields })));
                        }
                    }
                    // The arguments of the next call replace this frame.
                    for i in 0..nargc {
                        self.stack.swap(base + i, args + i);
                    }
                    self.pop_to(base + nargc);
                    f = nf;
                    argc = nargc;
                    span = nspan;
                }
                Value::Builtin(b) => return self.call_builtin(b, base, argc, span),
                Value::Ctor(c) => return self.construct(c, base, argc, span),
                other => return Err(not_callable(&other, span)),
            }
        }
    }

    /// Calls a builtin with the `argc` values from `base` on as arguments.
    /// (Kept out of `call_loop`, so that each nested call uses little stack.)
    #[inline(never)]
    fn call_builtin(&mut self, b: &'static crate::builtins::Builtin, base: usize, argc: usize, span: Span) -> Fallible<Value> {
        if argc < b.min || argc > b.max {
            return Err(builtin_arity_error(b, argc, span));
        }
        // The builtin reads its arguments where they are, and the
        // functions it calls (`map`...) get a stack of their own.
        let spare = std::mem::take(&mut self.spare);
        let stack = std::mem::replace(&mut self.stack, spare);
        let r = (b.func)(self, &stack[base..], span);
        let mut spare = std::mem::replace(&mut self.stack, stack);
        spare.clear();
        self.spare = spare;
        r
    }

    /// Builds a value of a type (`Some(x)`, `Circle(r)`) from the `argc`
    /// values from `base` on.
    #[inline(never)]
    fn construct(&mut self, c: Rc<CtorInfo>, base: usize, argc: usize, span: Span) -> Fallible<Value> {
        if argc != c.fields.len() {
            return Err(ctor_arity_error(&c, argc, span));
        }
        let fields = self.stack.split_off(base);
        Ok(Value::Variant(Rc::new(Variant { ctor: c, fields })))
    }

    /// Picks the first definition (clause) of a function that accepts the
    /// arguments, which are in the first slots of the frame. `Flow::Return`:
    /// a `?` in a guard ended the call.
    #[cfg_attr(target_arch = "wasm32", inline(never))]
    fn enter(&mut self, clo: &Rc<Closure>, fr: &Frame, span: Span) -> Result<usize, Flow> {
        let code = &*clo.code;
        let base = fr.base;
        if let Some(slot) = code.self_slot {
            set(&mut self.stack[base + slot as usize], Value::Func(clo.clone()));
        }
        // Fast path: one definition whose parameters are plain names.
        if code.simple {
            return Ok(0);
        }
        'clauses: for (ci, clause) in code.clauses.iter().enumerate() {
            if !clause.tests.is_empty() {
                // The parts of the arguments go to the slots after the arguments.
                let (args, rest) = self.stack[base..].split_at_mut(code.arity);
                for (i, p) in &clause.tests {
                    if !may_match(p, &args[*i]) || !match_pat(p, &args[*i], rest, code.arity) {
                        continue 'clauses;
                    }
                }
            }
            if let Some(g) = &clause.guard {
                let gv = self.eval(g, fr)?;
                if !truth(&gv, g.span)? {
                    continue;
                }
            }
            return Ok(ci);
        }
        Err(no_clause(&clo.def, &self.stack[base..base + code.arity], span).into())
    }
}

// ----- helpers ----------------------------------------------------------

/// The function number `i` of the group of the running function, which
/// captures the same values.
#[inline(never)]
fn sibling(fr: &Frame, i: u32) -> Value {
    let group = fr.siblings.expect("a function of a group");
    let Sibling { def, code } = &group[i as usize];
    Value::Func(Rc::new(Closure { def: def.clone(), code: code.clone(), captured: fr.captured.to_vec(), siblings: Some(group.clone()) }))
}

/// Like `match_pattern`, for a prepared pattern: the names go to their slots,
/// `slots` holding the slots of the frame from number `first` on.
fn match_pat(p: &Pat, v: &Value, slots: &mut [Value], first: usize) -> bool {
    match (p, v) {
        (Pat::Any, _) => true,
        (Pat::Bind(slot), _) => {
            set(&mut slots[*slot as usize - first], v.clone());
            true
        }
        (Pat::Unit, Value::Unit) => true,
        (Pat::Bool(a), Value::Bool(b)) => a == b,
        (Pat::Int(a), Value::Int(b)) => a == b,
        (Pat::Int(a), Value::Float(b)) => cmp_int_float(*a, *b) == Some(std::cmp::Ordering::Equal),
        (Pat::Big(a), Value::Big(b)) => a == b,
        (Pat::Big(a), Value::Float(b)) => a.cmp_f64(*b) == Some(std::cmp::Ordering::Equal),
        (Pat::Float(a), Value::Float(b)) => a == b,
        (Pat::Float(a), Value::Int(b)) => cmp_int_float(*b, *a) == Some(std::cmp::Ordering::Equal),
        (Pat::Text(a), Value::Text(b)) => a == b,
        (Pat::Ctor(name, pats), Value::Variant(var)) => {
            var.ctor.name == *name && pats.len() == var.fields.len() && match_each(pats, &var.fields, slots, first)
        }
        (Pat::List(pats, rest), Value::List(l)) => {
            let n = pats.len();
            let len_ok = match rest {
                None => l.len() == n,
                Some(_) => l.len() >= n,
            };
            if !len_ok {
                return false;
            }
            if !match_each(pats, l.as_slice(), slots, first) {
                return false;
            }
            if let Some(Some(slot)) = rest {
                set(&mut slots[*slot as usize - first], Value::List(l.slice(n, l.len())));
            }
            true
        }
        (Pat::Record(fields), Value::Record(r)) => fields.iter().all(|(k, p)| r.get(k).is_some_and(|x| match_pat(p, x, slots, first))),
        (Pat::Or(alts), _) => alts.iter().any(|a| match_pat(a, v, slots, first)),
        _ => false,
    }
}

/// Whether `p` may match `v`: `false` when an obvious difference (another
/// whole number, another constructor) rules it out, without a call.
#[inline(always)]
fn may_match(p: &Pat, v: &Value) -> bool {
    match (p, v) {
        (Pat::Int(a), Value::Int(b)) => a == b,
        (Pat::Ctor(name, _), Value::Variant(var)) => var.ctor.name == *name,
        _ => true,
    }
}

/// `match_pat` on each value with the pattern at the same place; names and
/// `_`, the most common parts, without a nested call.
#[inline(always)]
fn match_each(pats: &[Pat], vals: &[Value], slots: &mut [Value], first: usize) -> bool {
    for (p, v) in pats.iter().zip(vals) {
        let ok = match p {
            Pat::Any => true,
            Pat::Bind(slot) => {
                set(&mut slots[*slot as usize - first], v.clone());
                true
            }
            _ => match_pat(p, v, slots, first),
        };
        if !ok {
            return false;
        }
    }
    true
}

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
        (PatKind::Int(a), Value::Float(b)) => cmp_int_float(*a, *b) == Some(std::cmp::Ordering::Equal),
        (PatKind::Big(a), Value::Big(b)) => a == b,
        (PatKind::Big(a), Value::Float(b)) => a.cmp_f64(*b) == Some(std::cmp::Ordering::Equal),
        (PatKind::Float(a), Value::Float(b)) => a == b,
        (PatKind::Float(a), Value::Int(b)) => cmp_int_float(*b, *a) == Some(std::cmp::Ordering::Equal),
        (PatKind::Text(a), Value::Text(b)) => a == b,
        (PatKind::Ctor(name, pats), Value::Variant(var)) => {
            var.ctor.name == name.own_name()
                && pats.len() == var.fields.len()
                && pats.iter().zip(&var.fields).all(|(p, f)| match_pattern(p, f, out))
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

#[inline(never)]
pub(crate) fn unary(op: UnOp, v: &Value, span: Span, operand: Span) -> Fallible<Value> {
    match (op, v) {
        (UnOp::Neg, Value::Int(n)) => Ok(n.checked_neg().map_or_else(|| bigint::value(BigInt::from_i64(*n).neg()), Value::Int)),
        (UnOp::Neg, Value::Big(n)) => Ok(bigint::value(n.neg())),
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
pub(crate) fn fn_arity_error(def: &FnDef, got: usize, span: Span) -> Box<Diagnostic> {
    let name = def.name.map(|n| n.name().to_string()).unwrap_or_else(|| "fn".to_string());
    arity_error(&name, &def.signature(), def.arity, def.arity, got, span, Some(def.span))
}

#[inline(never)]
pub(crate) fn no_clause(def: &FnDef, args: &[Value], span: Span) -> Box<Diagnostic> {
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
fn out_of_steps(span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "",
        t!(
            "the program was stopped: it made more calls than allowed for this run",
            "le programme a été arrêté : il a fait plus d'appels que permis pour cette exécution"
        ),
    )
    .at(span)
    .help(t!(
        "does it call itself forever? Check that it always reaches a base case",
        "s'appelle-t-il sans fin ? Vérifiez qu'il atteint toujours un cas de base"
    ))
    .boxed()
}

#[inline(never)]
pub(crate) fn add_frame(d: &mut Diagnostic, def: &FnDef, span: Span) {
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
pub(crate) fn not_callable(v: &Value, span: Span) -> Box<Diagnostic> {
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
pub(crate) fn no_case(v: &Value, span: Span) -> Box<Diagnostic> {
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
pub(crate) fn try_error(v: &Value, span: Span) -> Box<Diagnostic> {
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

pub(crate) fn pattern_mismatch(span: Span, v: &Value) -> Box<Diagnostic> {
    Diagnostic::error("E006", t!("the value {} does not match this pattern", "la valeur {} ne correspond pas à ce motif", v.describe()))
        .at(span)
        .help(t!(
            "use `match` to handle the different possible shapes of a value",
            "utilisez `match` pour traiter les différentes formes possibles d'une valeur"
        ))
        .boxed()
}

pub(crate) fn question_outside(span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E016", t!("`?` can only be used inside a function", "`?` ne peut être utilisé que dans une fonction"))
        .at(span)
        .help(t!(
            "outside of a function, handle the value with `match` or `unwrap_or`",
            "en dehors d'une fonction, traitez la valeur avec `match` ou `unwrap_or`"
        ))
        .boxed()
}

pub(crate) fn spread_error(v: &Value, span: Span, list: bool) -> Box<Diagnostic> {
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

pub(crate) fn condition_error(v: &Value, span: Span) -> Box<Diagnostic> {
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
        Value::Int(_) | Value::Big(_) | Value::Float(_) => {
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

pub fn div_zero(span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E008", t!("division by zero", "division par zéro"))
        .at(span)
        .help(t!(
            "check the divisor first: `if b == 0 then ... else a / b`,\nor return a Result: `if b == 0 then Err(\"division by zero\") else Ok(a / b)`",
            "vérifiez d'abord le diviseur : `if b == 0 then ... else a / b`,\nou renvoyez un Result : `if b == 0 then Err(\"division par zéro\") else Ok(a / b)`"
        ))
        .boxed()
}

pub fn zero_to_negative_power(span: Span) -> Box<Diagnostic> {
    let mut d = div_zero(span);
    d.notes.push(t!("`0 ** -n` is `1 / 0 ** n`", "`0 ** -n` vaut `1 / 0 ** n`"));
    d
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
    let is_num = |v: &Value| matches!(v, Value::Int(_) | Value::Big(_) | Value::Float(_));
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
    } else if op == BinOp::Mul && (is_text(l) || is_list(l)) && is_num(r) {
        d = d.help(t!("to repeat a text or a list, use `repeat(x, n)`", "pour répéter un texte ou une liste, utilisez `repeat(x, n)`"));
    } else if (is_text(l) && is_num(r)) || (is_num(l) && is_text(r)) {
        d = d.help(t!(
            "convert the text to a number first, with `int(t)` or `float(t)`",
            "convertissez d'abord le texte en nombre, avec `int(t)` ou `float(t)`"
        ));
    }
    d.boxed()
}

/// Drops a value. Most values hold no memory: they need no call to the
/// general (and large) code that frees values.
#[inline(always)]
fn discard(v: Value) {
    match v {
        Value::Unit | Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::Builtin(_) => std::mem::forget(v),
        v => drop(v),
    }
}

/// Puts a value in a slot, dropping the one it replaces.
#[inline(always)]
fn set(slot: &mut Value, v: Value) {
    discard(std::mem::replace(slot, v));
}

/// Local names and constants: their value is always there.
#[inline(always)]
fn is_leaf(e: &Node) -> bool {
    matches!(e.kind, Kind::Local(_) | Kind::Captured(_) | Kind::Const(_))
}

/// `binop` on two whole numbers, for the common operators; `None` leaves the
/// other cases and the errors to `binop`.
#[inline(always)]
fn int_op(op: BinOp, a: i64, b: i64) -> Option<Value> {
    Some(match op {
        BinOp::Add => Value::Int(a.checked_add(b)?),
        BinOp::Sub => Value::Int(a.checked_sub(b)?),
        BinOp::Mul => Value::Int(a.checked_mul(b)?),
        // Rounded down, and the remainder has the sign of the divisor.
        BinOp::IntDiv if b > 0 => Value::Int(a.div_euclid(b)),
        BinOp::Mod if b > 0 => Value::Int(a.rem_euclid(b)),
        BinOp::Eq => Value::Bool(a == b),
        BinOp::Ne => Value::Bool(a != b),
        BinOp::Lt => Value::Bool(a < b),
        BinOp::Le => Value::Bool(a <= b),
        BinOp::Gt => Value::Bool(a > b),
        BinOp::Ge => Value::Bool(a >= b),
        _ => return None,
    })
}

fn num(v: &Value) -> Option<f64> {
    match v {
        Value::Int(n) => Some(*n as f64),
        Value::Big(n) => Some(n.to_f64()),
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
                if [l, r].iter().any(|v| matches!(v, Float(f) if f.is_nan())) {
                    return Err(Diagnostic::error(
                        "E005",
                        t!("`nan` (not a number) cannot be compared", "`nan` (pas un nombre) ne peut pas être comparé"),
                    )
                    .at(span)
                    .label(format!("{} {} {}", l.describe(), op.symbol(), r.describe()))
                    .help(t!(
                        "`nan` is the result of a calculation that has none, like `0.0 * inf`: check where this value comes from",
                        "`nan` est le résultat d'un calcul qui n'en a pas, comme `0.0 * inf` : vérifiez d'où vient cette valeur"
                    ))
                    .boxed());
                }
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
                    // Only `i64::MIN // -1` does not fit.
                    a.checked_div(b).map(|q| if (a % b != 0) && ((a < 0) != (b < 0)) { q - 1 } else { q })
                }
                BinOp::Mod => {
                    if b == 0 {
                        return Err(div_zero(span));
                    }
                    if b == -1 {
                        // The remainder is always 0 (and i64::MIN % -1 would overflow).
                        return Ok(Int(0));
                    }
                    a.checked_rem(b).map(|m| if m != 0 && ((m < 0) != (b < 0)) { m + b } else { m })
                }
                BinOp::Pow => {
                    if b < 0 {
                        if a == 0 {
                            return Err(zero_to_negative_power(span));
                        }
                        return Ok(Float((a as f64).powf(b as f64)));
                    }
                    match a {
                        // These never grow, whatever the exponent.
                        0 => Some(if b == 0 { 1 } else { 0 }),
                        1 => Some(1),
                        -1 => Some(if b % 2 == 0 { 1 } else { -1 }),
                        _ => u32::try_from(b).ok().and_then(|b| a.checked_pow(b)),
                    }
                }
                _ => unreachable!(),
            };
            match res {
                Some(n) => Ok(Int(n)),
                // Too large for 64 bits: computed with large numbers.
                None => bigint::binop(op, l, r, span),
            }
        }
        (Int(_) | Big(_), Int(_) | Big(_)) => bigint::binop(op, l, r, span),
        (Int(_) | Big(_) | Float(_), Int(_) | Big(_) | Float(_)) => {
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
                    // Exact, and with the sign of `b`: `-1.5 % 2.0` is 0.5, `5.0 % inf` is 5.0.
                    let m = a % b;
                    if m != 0.0 && (m < 0.0) != (b < 0.0) {
                        m + b
                    } else {
                        m
                    }
                }
                BinOp::Pow => {
                    if a == 0.0 && b < 0.0 {
                        return Err(zero_to_negative_power(span));
                    }
                    if a < 0.0 && b.is_finite() && b.fract() != 0.0 {
                        return Err(Diagnostic::error(
                            "E023",
                            t!(
                                "a negative number ({}) to a fractional power ({}) has no value among real numbers",
                                "un nombre négatif ({}) à une puissance non entière ({}) n'a pas de valeur parmi les nombres réels",
                                fmt_float(a),
                                fmt_float(b)
                            ),
                        )
                        .at(span)
                        .help(t!(
                            "use `abs(x) ** p` for the power of its absolute value",
                            "utilisez `abs(x) ** p` pour la puissance de sa valeur absolue"
                        ))
                        .boxed());
                    }
                    a.powf(b)
                }
                _ => unreachable!(),
            }))
        }
        (Text(a), Text(b)) if op == BinOp::Add => {
            if a.len() + b.len() > MAX_TEXT {
                return Err(too_big(false, span));
            }
            let mut s = String::with_capacity(a.len() + b.len());
            s.push_str(a);
            s.push_str(b);
            Ok(Text(Rc::from(s)))
        }
        (List(a), List(b)) if op == BinOp::Add => {
            if a.len() + b.len() > MAX_LIST {
                return Err(too_big(true, span));
            }
            let mut v = Vec::with_capacity(a.len() + b.len());
            v.extend(a.iter().cloned());
            v.extend(b.iter().cloned());
            Ok(Value::list(v))
        }
        _ => Err(op_error(op, l, r, span)),
    }
}

pub(crate) fn range(a: &Value, b: &Value, span: Span) -> Fallible<Value> {
    if let (Value::Int(a), Value::Int(b)) = (a, b) {
        if b < a {
            return Ok(Value::list(Vec::new()));
        }
        if b.saturating_sub(*a) < MAX_RANGE {
            return Ok(Value::list((*a..=*b).map(Value::Int).collect()));
        }
    }
    match (bigint::big(a), bigint::big(b)) {
        (Some(a), Some(b)) => {
            if b < a {
                return Ok(Value::list(Vec::new()));
            }
            let one = BigInt::from_i64(1);
            let size = b.sub(&a).add(&one);
            match size.to_i64() {
                Some(n) if n <= MAX_RANGE => {
                    // Large numbers, but not many of them.
                    let mut n_items = Vec::with_capacity(n as usize);
                    let mut x = a.into_owned();
                    for _ in 0..n {
                        let next = x.add(&one);
                        n_items.push(bigint::value(x));
                        x = next;
                    }
                    Ok(Value::list(n_items))
                }
                _ => Err(Diagnostic::error(
                    "E023",
                    t!("this range is too large ({} elements)", "cet intervalle est trop grand ({} éléments)", size),
                )
                .at(span)
                .help(t!(
                    "ranges are lists: keep them under {MAX_RANGE} elements",
                    "les intervalles sont des listes : gardez-les sous {MAX_RANGE} éléments"
                ))
                .boxed()),
            }
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

#[inline(never)]
pub(crate) fn index(o: &Value, i: &Value, span: Span, idx_span: Span) -> Fallible<Value> {
    let out_of_range = |n: &dyn std::fmt::Display, len: usize, what: String| {
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
                    n,
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
                    n,
                    len,
                    if len == 0 {
                        t!("the text is empty", "le texte est vide")
                    } else {
                        t!("the text has {len} character(s)", "le texte a {len} caractère(s)")
                    },
                )),
            }
        }
        (Value::List(l), Value::Big(n)) => {
            Err(out_of_range(n, l.len(), t!("the list has {} element(s)", "la liste a {} élément(s)", l.len())))
        }
        (Value::Text(s), Value::Big(n)) => {
            let len = s.chars().count();
            Err(out_of_range(n, len, t!("the text has {len} character(s)", "le texte a {len} caractère(s)")))
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
