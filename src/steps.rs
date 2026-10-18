//! Step-by-step evaluation, for teaching: `rythme steps file.ry` (in French
//! `rythme etapes`), `:steps` in the REPL and the "Steps" button of the
//! playground show how each expression is reduced, in the substitution model:
//!
//! ```text
//! fib(3)
//!   → fib(2) + fib(1)
//!   → fib(1) + fib(0) + fib(1)
//!   → ...
//!   → 2
//! ```
//!
//! This is a small evaluator of its own, over the syntax tree: the expression
//! being computed is a term, in which the arguments of a call are written
//! into the body of the function. It follows the interpreter exactly (the
//! same order of evaluation, the same errors), and uses its operations, its
//! built-in functions and its patterns, so that each result is the one the
//! program gives when it runs (`tests/steps.rs` checks it on every example).
//!
//! A step is one call or one operation. A call shows the body of the
//! function with its arguments in place, already simplified as far as it can
//! be without calling another function: `fib(3)` gives `fib(2) + fib(1)`, not
//! `if 3 < 2 then 3 else fib(3 - 1) + fib(3 - 2)`. Built-in functions, and
//! functions of other files, give their result in one step.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::ast::*;
use crate::compile;
use crate::diagnostic::{Diagnostic, Fallible};
use crate::driver::{Driver, Module, Reported};
use crate::interp::{self, too_big, Interp, Mode, Wrap, MAX_LIST, MAX_TEXT};
use crate::source::Span;
use crate::symbol::Sym;
use crate::value::*;

/// The limits of a step-by-step evaluation.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Steps shown for each expression; the rest is computed without being shown.
    pub max_steps: usize,
    /// The longest line shown, in characters.
    pub width: usize,
    /// Steps made in all, shown or not, before the evaluation is stopped (a
    /// program that never ends).
    pub limit: u64,
    /// Show the steps (otherwise only compute, for the tests).
    pub show: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options { max_steps: 200, width: 100, limit: 10_000_000, show: true }
    }
}

/// The most lines shown for one step.
const MAX_LINES: usize = 24;

/// How deeply terms may be nested: each level is a nested call of the
/// functions below, and a browser has a small stack.
#[cfg(target_arch = "wasm32")]
const MAX_DEPTH: u32 = 1200;
#[cfg(not(target_arch = "wasm32"))]
const MAX_DEPTH: u32 = 10_000;

// ----- terms --------------------------------------------------------------

/// An expression being computed: the syntax tree, where the names already
/// known are replaced by their values.
struct Term {
    k: K,
    /// Where its own errors are shown.
    span: Span,
    /// The expression it came from: where its value is said to come from.
    /// (`hide(x)` becomes `x`, but an error about its value points at `hide(x)`.)
    outer: Span,
}

enum K {
    Val(Value),
    /// A local name whose value is not known yet (defined by a block or a
    /// case of `match` around it, before it runs).
    Local(Sym),
    /// A name of the program, looked up when it is needed.
    Global(Sym),
    Template(Vec<Part>),
    List(Vec<Elem>),
    Record(Vec<Fld>),
    /// An anonymous function, with the values of the names it uses.
    Lambda(Rc<FnDef>, Env),
    Call(Box<Term>, Vec<Term>, Look),
    /// `value.name(args)`: the value, the name, the function `name` and the arguments.
    Method(Box<Term>, Sym, Box<Term>, Vec<Term>, Span),
    Field(Box<Term>, Rc<str>, Span),
    Index(Box<Term>, Box<Term>),
    Unary(UnOp, Box<Term>),
    Binary(BinOp, Box<Term>, Box<Term>, Span),
    Logic(BinOp, Box<Term>, Box<Term>),
    /// The right side of `and` / `or`, whose value must be true or false.
    Truth(Box<Term>),
    If(Box<Term>, Box<Term>, Box<Term>),
    Match(Box<Term>, Vec<Arm>),
    Block(Vec<St>),
    Try(Box<Term>),
    Range(Box<Term>, Box<Term>),
    /// The body of a called function: where a `?` returns to, and the
    /// function named in the call trace of an error. Not shown.
    Frame(Box<Term>, Option<(Rc<FnDef>, Span)>),
}

enum Part {
    Lit(Rc<str>),
    Expr(Term),
}

enum Elem {
    One(Term),
    Spread(Term),
}

enum Fld {
    Field(Rc<str>, Term),
    Spread(Term),
}

struct Arm {
    pat: Pattern,
    guard: Option<Term>,
    body: Term,
}

enum St {
    Let(Pattern, Term),
    Expr(Term),
    /// Local functions written one after the other (they see each other).
    Fns(Vec<Rc<FnDef>>, Env),
    /// `name = fn(...) = ...`, which sees itself.
    SelfFn(Sym, Rc<FnDef>, Env),
}

/// How a call is written.
#[derive(Clone)]
enum Look {
    Plain,
    /// `x.name(args)`, calling `name(x, args)`.
    Method(Sym),
    /// `record.name(args)`, calling the function in the field `name`.
    Field(Value, Sym),
}

/// The values of names, for the functions made while stepping.
type Env = Vec<(Sym, Value)>;

/// Names visible while a piece of code becomes a term: a value, or `None`
/// for a local name defined later.
type Sub = Vec<(Sym, Option<Value>)>;

impl Term {
    fn new(k: K, span: Span) -> Term {
        Term { k, span, outer: span }
    }

    fn val(v: Value, span: Span) -> Term {
        Term::new(K::Val(v), span)
    }

    fn value(&self) -> Option<&Value> {
        match &self.k {
            K::Val(v) => Some(v),
            _ => None,
        }
    }
}

/// The value of a term known to be a value.
fn v(t: &Term) -> &Value {
    match &t.k {
        K::Val(v) => v,
        _ => unreachable!("a value"),
    }
}

/// Puts `new` in the place of `t`, the expression it comes from.
fn replace(t: &mut Term, mut new: Term) {
    new.outer = t.outer;
    *t = new;
}

fn take(t: &mut Term) -> Term {
    std::mem::replace(t, Term::val(Value::Unit, t.span))
}

fn bound_names(p: &Pattern) -> Vec<Sym> {
    let mut out = Vec::new();
    p.bindings(&mut out);
    out.into_iter().map(|(s, _)| s).collect()
}

fn lookup(sub: &Sub, s: Sym) -> Option<&Option<Value>> {
    sub.iter().rev().find(|(n, _)| *n == s).map(|(_, v)| v)
}

/// The known values of `sub`, without the names hidden by a later one.
fn env_of(sub: &Sub) -> Env {
    let mut seen = HashSet::new();
    let mut env = Vec::new();
    for (s, val) in sub.iter().rev() {
        if seen.insert(*s) {
            if let Some(val) = val {
                env.push((*s, val.clone()));
            }
        }
    }
    env.reverse();
    env
}

fn key(k: &str) -> Rc<str> {
    Sym::new(k).name()
}

/// Turns code into a term, replacing the names of `sub` by their values.
fn convert(e: &Expr, sub: &mut Sub) -> Term {
    let span = e.span;
    let bx = |x: &Expr, sub: &mut Sub| Box::new(convert(x, sub));
    let k = match &e.kind {
        ExprKind::Unit => K::Val(Value::Unit),
        ExprKind::Bool(b) => K::Val(Value::Bool(*b)),
        ExprKind::Int(n) => K::Val(Value::Int(*n)),
        ExprKind::Float(f) => K::Val(Value::Float(*f)),
        ExprKind::Text(s) => K::Val(Value::Text(s.clone())),
        ExprKind::Template(parts) => K::Template(
            parts
                .iter()
                .map(|p| match p {
                    TemplatePart::Lit(l) => Part::Lit(l.clone()),
                    TemplatePart::Expr(x) => Part::Expr(convert(x, sub)),
                })
                .collect(),
        ),
        ExprKind::Var(s) | ExprKind::Ctor(s) => name_term(sub, *s),
        ExprKind::List(items) => K::List(
            items
                .iter()
                .map(|it| match it {
                    ListItem::One(x) => Elem::One(convert(x, sub)),
                    ListItem::Spread(x) => Elem::Spread(convert(x, sub)),
                })
                .collect(),
        ),
        ExprKind::Record(items) => K::Record(
            items
                .iter()
                .map(|it| match it {
                    RecordItem::Field(k, _, x) => Fld::Field(key(k), convert(x, sub)),
                    RecordItem::Spread(x) => Fld::Spread(convert(x, sub)),
                })
                .collect(),
        ),
        ExprKind::Lambda(def) => K::Lambda(def.clone(), env_of(sub)),
        ExprKind::Call(f, args) => K::Call(bx(f, sub), args.iter().map(|a| convert(a, sub)).collect(), Look::Plain),
        ExprKind::Method(recv, name, args, nspan) => {
            let recv = bx(recv, sub);
            let func = Box::new(Term::new(name_term(sub, *name), *nspan));
            K::Method(recv, *name, func, args.iter().map(|a| convert(a, sub)).collect(), *nspan)
        }
        ExprKind::Field(x, name, nspan) => K::Field(bx(x, sub), key(name), *nspan),
        ExprKind::Index(a, b) => K::Index(bx(a, sub), bx(b, sub)),
        ExprKind::Unary(op, x) => K::Unary(*op, bx(x, sub)),
        ExprKind::Binary(op @ (BinOp::And | BinOp::Or), l, r, _) => K::Logic(*op, bx(l, sub), bx(r, sub)),
        ExprKind::Binary(op, l, r, ospan) => K::Binary(*op, bx(l, sub), bx(r, sub), *ospan),
        ExprKind::If(c, a, b) => K::If(bx(c, sub), bx(a, sub), bx(b, sub)),
        ExprKind::Match(subject, arms) => {
            let subject = bx(subject, sub);
            let arms = arms
                .iter()
                .map(|a| {
                    let mark = sub.len();
                    sub.extend(bound_names(&a.pat).into_iter().map(|s| (s, None)));
                    let guard = a.guard.as_ref().map(|g| convert(g, sub));
                    let body = convert(&a.body, sub);
                    sub.truncate(mark);
                    Arm { pat: a.pat.clone(), guard, body }
                })
                .collect();
            K::Match(subject, arms)
        }
        ExprKind::Block(stmts) => K::Block(convert_block(stmts, sub)),
        ExprKind::Try(x) => K::Try(bx(x, sub)),
        ExprKind::Range(a, b) => K::Range(bx(a, sub), bx(b, sub)),
    };
    Term::new(k, span)
}

fn name_term(sub: &Sub, s: Sym) -> K {
    match lookup(sub, s) {
        Some(Some(val)) => K::Val(val.clone()),
        Some(None) => K::Local(s),
        None => K::Global(s),
    }
}

/// The statements of a block, grouped like the interpreter groups them.
fn convert_block(stmts: &[Stmt], sub: &mut Sub) -> Vec<St> {
    let mark = sub.len();
    let mut out = Vec::with_capacity(stmts.len());
    let mut i = 0;
    while i < stmts.len() {
        match &stmts[i] {
            Stmt::Fn(_) => {
                let defs: Vec<Rc<FnDef>> = stmts[i..]
                    .iter()
                    .map_while(|s| if let Stmt::Fn(d) = s { Some(d.clone()) } else { None })
                    .filter(|d| d.name.is_some())
                    .collect();
                i += stmts[i..].iter().take_while(|s| matches!(s, Stmt::Fn(_))).count();
                if defs.is_empty() {
                    continue;
                }
                let env = env_of(sub);
                sub.extend(defs.iter().filter_map(|d| d.name).map(|n| (n, None)));
                out.push(St::Fns(defs, env));
                continue;
            }
            Stmt::Let(p, x) => match (&p.kind, &x.kind) {
                (PatKind::Bind(name), ExprKind::Lambda(def)) => {
                    out.push(St::SelfFn(*name, def.clone(), env_of(sub)));
                    sub.push((*name, None));
                }
                _ => {
                    let t = convert(x, sub);
                    sub.extend(bound_names(p).into_iter().map(|s| (s, None)));
                    out.push(St::Let(p.clone(), t));
                }
            },
            Stmt::Expr(x) => out.push(St::Expr(convert(x, sub))),
        }
        i += 1;
    }
    sub.truncate(mark);
    out
}

/// Replaces the local names `b` by their values in `t`.
fn subst(t: &mut Term, b: &[(Sym, Value)]) {
    if b.is_empty() {
        return;
    }
    match &mut t.k {
        K::Val(_) | K::Global(_) => {}
        K::Local(s) => {
            if let Some((_, val)) = b.iter().rev().find(|(n, _)| n == s) {
                t.k = K::Val(val.clone());
            }
        }
        K::Template(parts) => {
            for p in parts {
                if let Part::Expr(x) = p {
                    subst(x, b);
                }
            }
        }
        K::List(items) => {
            for it in items {
                match it {
                    Elem::One(x) | Elem::Spread(x) => subst(x, b),
                }
            }
        }
        K::Record(items) => {
            for it in items {
                match it {
                    Fld::Field(_, x) | Fld::Spread(x) => subst(x, b),
                }
            }
        }
        K::Lambda(_, env) => extend_env(env, b, &[]),
        K::Call(f, args, _) => {
            subst(f, b);
            args.iter_mut().for_each(|a| subst(a, b));
        }
        K::Method(recv, _, func, args, _) => {
            subst(recv, b);
            subst(func, b);
            args.iter_mut().for_each(|a| subst(a, b));
        }
        K::Field(x, ..) | K::Unary(_, x) | K::Truth(x) | K::Try(x) | K::Frame(x, _) => subst(x, b),
        K::Index(x, y) | K::Binary(_, x, y, _) | K::Logic(_, x, y) | K::Range(x, y) => {
            subst(x, b);
            subst(y, b);
        }
        K::If(c, x, y) => {
            subst(c, b);
            subst(x, b);
            subst(y, b);
        }
        K::Match(subject, arms) => {
            subst(subject, b);
            for arm in arms {
                let hidden = bound_names(&arm.pat);
                let inner: Vec<(Sym, Value)> = b.iter().filter(|(s, _)| !hidden.contains(s)).cloned().collect();
                if let Some(g) = &mut arm.guard {
                    subst(g, &inner);
                }
                subst(&mut arm.body, &inner);
            }
        }
        K::Block(stmts) => {
            let mut b: Vec<(Sym, Value)> = b.to_vec();
            for st in stmts {
                if b.is_empty() {
                    break;
                }
                match st {
                    St::Let(p, x) => {
                        subst(x, &b);
                        let hidden = bound_names(p);
                        b.retain(|(s, _)| !hidden.contains(s));
                    }
                    St::Expr(x) => subst(x, &b),
                    St::Fns(defs, env) => {
                        let hidden: Vec<Sym> = defs.iter().filter_map(|d| d.name).collect();
                        extend_env(env, &b, &hidden);
                        b.retain(|(s, _)| !hidden.contains(s));
                    }
                    St::SelfFn(name, _, env) => {
                        extend_env(env, &b, &[*name]);
                        b.retain(|(s, _)| s != name);
                    }
                }
            }
        }
    }
}

fn extend_env(env: &mut Env, b: &[(Sym, Value)], hidden: &[Sym]) {
    for (s, val) in b {
        if hidden.contains(s) {
            continue;
        }
        match env.iter_mut().find(|(n, _)| n == s) {
            Some(slot) => slot.1 = val.clone(),
            None => env.push((*s, val.clone())),
        }
    }
}

// ----- the evaluator ------------------------------------------------------

/// What one step did.
enum S {
    /// The term is a value: nothing to do.
    Val,
    Did,
    Err(Box<Diagnostic>),
    /// A `?` found a `None` or an `Err`: the function returns it.
    Ret(Value),
}

/// The result of an evaluation that is not shown (a guard).
enum H {
    Val(Value),
    Ret(Value),
    Err(Box<Diagnostic>),
}

pub struct Stepper<'a> {
    it: &'a mut Interp,
    opts: Options,
    /// The functions made while stepping, with the values of the names they use.
    known: HashMap<usize, Known>,
    /// The files whose functions are shown step by step.
    files: HashSet<u32>,
    /// In the REPL, the functions of the earlier inputs too.
    repl: bool,
    left: u64,
}

/// A function made while stepping.
struct Known {
    /// Kept, so that its address is not given to another one.
    _function: Rc<Closure>,
    /// The values of the names it uses, and the functions of its group.
    env: Env,
    /// `step` for `step = fn(...) = ...`.
    name: Option<Sym>,
}

macro_rules! sub {
    ($self:ident, $x:expr, $d:expr) => {
        match $self.step($x, $d) {
            S::Val => {}
            other => return other,
        }
    };
}

macro_rules! fail {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(d) => return S::Err(d),
        }
    };
}

impl<'a> Stepper<'a> {
    pub fn new(it: &'a mut Interp, opts: Options) -> Stepper<'a> {
        Stepper { it, left: opts.limit, opts, known: HashMap::new(), files: HashSet::new(), repl: false }
    }

    /// The functions of this file are shown step by step.
    pub fn add_file(&mut self, file: u32) {
        self.files.insert(file);
    }

    /// The values of the names a function sees, if its calls are shown step by step.
    fn env_for(&self, clo: &Rc<Closure>) -> Option<Env> {
        if let Some(k) = self.known.get(&(Rc::as_ptr(clo) as usize)) {
            return Some(k.env.clone());
        }
        let file = clo.def.span.file;
        let own = self.files.contains(&file) || (self.repl && crate::source::get(file).is_some_and(|f| f.name == "<repl>"));
        (own && clo.captured.is_empty() && clo.siblings.is_none()).then(Vec::new)
    }

    fn lookup(&self, s: Sym, span: Span) -> Fallible<Value> {
        let stored = self.it.stored(s);
        match self.it.globals.get(&stored) {
            Some(val) => Ok(val.clone()),
            None => Err(self.it.unknown_name(stored, &compile::Scope::default(), span)),
        }
    }

    /// Makes the functions of a block (or an anonymous function) with the
    /// interpreter itself, so that built-in functions can call them: the
    /// code `fn(names...) = do <definitions>; [functions] end`, called with
    /// the values of the names they use.
    fn make_functions(&mut self, defs: &[(Sym, Rc<FnDef>)], lambda: bool, env: &Env, span: Span) -> Fallible<Vec<Value>> {
        let e = |kind: ExprKind| Expr::new(kind, span);
        let body = if lambda {
            e(ExprKind::Lambda(defs[0].1.clone()))
        } else {
            let mut stmts: Vec<Stmt> = Vec::new();
            for (name, def) in defs {
                if def.name.is_some() {
                    stmts.push(Stmt::Fn(def.clone()));
                } else {
                    stmts.push(Stmt::Let(Pattern { kind: PatKind::Bind(*name), span }, e(ExprKind::Lambda(def.clone()))));
                }
            }
            stmts.push(Stmt::Expr(e(ExprKind::List(defs.iter().map(|(n, _)| ListItem::One(e(ExprKind::Var(*n)))).collect()))));
            e(ExprKind::Block(stmts))
        };
        let params = env.iter().map(|(s, _)| Pattern { kind: PatKind::Bind(*s), span }).collect();
        let clause = Rc::new(Clause { params, guard: None, body, span });
        let maker = FnDef { name: None, clauses: vec![clause], arity: env.len(), doc: None, span };
        let code = compile::function(&maker, &self.it.names);
        let maker = Closure { def: Rc::new(maker), code: Rc::new(code), captured: Vec::new(), siblings: None };
        let made = self.it.call(Value::Func(Rc::new(maker)), env.iter().map(|(_, v)| v.clone()).collect(), span)?;
        let made = match made {
            Value::List(l) => l.iter().cloned().collect(),
            single => vec![single],
        };
        // Each one sees the values of the names it uses, and the functions of its group.
        let mut inner = env.clone();
        if !lambda {
            let named: Env = defs.iter().map(|(n, _)| *n).zip(made.iter().cloned()).collect();
            extend_env(&mut inner, &named, &[]);
        }
        for (f, (name, _)) in made.iter().zip(defs) {
            if let Value::Func(c) = f {
                let name = (!lambda).then_some(*name);
                self.known.insert(Rc::as_ptr(c) as usize, Known { _function: c.clone(), env: inner.clone(), name });
            }
        }
        Ok(made)
    }

    fn lambda(&mut self, def: &Rc<FnDef>, env: &Env, span: Span) -> Fallible<Value> {
        let name = Sym::new("fn");
        Ok(self.make_functions(&[(name, def.clone())], true, env, span)?.remove(0))
    }

    /// One step of the evaluation of `t`, in the order of the interpreter.
    fn step(&mut self, t: &mut Term, depth: u32) -> S {
        if depth > MAX_DEPTH {
            return S::Err(too_deep(t.span));
        }
        let d = depth + 1;
        let span = t.span;
        match &mut t.k {
            K::Val(_) => {
                t.span = t.outer;
                S::Val
            }
            K::Local(s) | K::Global(s) => {
                let s = *s;
                t.k = K::Val(fail!(self.lookup(s, span)));
                S::Did
            }
            K::Template(parts) => {
                for p in parts.iter_mut() {
                    if let Part::Expr(x) = p {
                        sub!(self, x, d);
                    }
                }
                t.k = K::Val(fail!(template(parts, span)));
                S::Did
            }
            K::List(items) => {
                for it in items.iter_mut() {
                    match it {
                        Elem::One(x) => sub!(self, x, d),
                        Elem::Spread(x) => {
                            sub!(self, x, d);
                            if !matches!(v(x), Value::List(_)) {
                                return S::Err(interp::spread_error(v(x), x.span, true));
                            }
                        }
                    }
                }
                t.k = K::Val(fail!(list(items, span)));
                S::Did
            }
            K::Record(items) => {
                for it in items.iter_mut() {
                    match it {
                        Fld::Field(_, x) => sub!(self, x, d),
                        Fld::Spread(x) => {
                            sub!(self, x, d);
                            if !matches!(v(x), Value::Record(_)) {
                                return S::Err(interp::spread_error(v(x), x.span, false));
                            }
                        }
                    }
                }
                t.k = K::Val(record(items));
                S::Did
            }
            K::Lambda(def, env) => {
                let (def, env) = (def.clone(), std::mem::take(env));
                t.k = K::Val(fail!(self.lambda(&def, &env, span)));
                S::Did
            }
            K::Call(f, args, _) => {
                sub!(self, f, d);
                for a in args.iter_mut() {
                    sub!(self, a, d);
                }
                let fv = v(f).clone();
                let av: Vec<Value> = args.iter().map(|a| v(a).clone()).collect();
                replace(t, fail!(self.call(fv, av, span, d)));
                S::Did
            }
            K::Method(recv, name, func, args, nspan) => {
                sub!(self, recv, d);
                let (name, nspan) = (*name, *nspan);
                let rv = v(recv).clone();
                let field = match &rv {
                    Value::Record(r) => r.get(&name.name()).cloned(),
                    _ => None,
                };
                let mut args = std::mem::take(args);
                let (f, look) = match field {
                    Some(f) => (f, Look::Field(rv, name)),
                    None => {
                        let f = match &func.k {
                            K::Val(f) => f.clone(),
                            _ => fail!(self.method_function(name, nspan)),
                        };
                        args.insert(0, Term::val(rv, recv.span));
                        (f, Look::Method(name))
                    }
                };
                t.k = K::Call(Box::new(Term::val(f, nspan)), args, look);
                S::Did
            }
            K::Field(x, name, nspan) => {
                sub!(self, x, d);
                t.k = K::Val(fail!(self.it.field(v(x), name, *nspan)));
                S::Did
            }
            K::Index(x, i) => {
                sub!(self, x, d);
                sub!(self, i, d);
                t.k = K::Val(fail!(interp::index(v(x), v(i), span, i.span)));
                S::Did
            }
            K::Unary(op, x) => {
                sub!(self, x, d);
                t.k = K::Val(fail!(interp::unary(*op, v(x), span, x.span)));
                S::Did
            }
            K::Binary(op, l, r, ospan) => {
                sub!(self, l, d);
                sub!(self, r, d);
                t.k = K::Val(fail!(interp::binop(*op, v(l), v(r), *ospan)));
                S::Did
            }
            K::Logic(op, l, r) => {
                sub!(self, l, d);
                let left = fail!(interp::truth(v(l), l.span));
                if (*op == BinOp::And) != left {
                    t.k = K::Val(Value::Bool(left));
                } else {
                    t.k = K::Truth(Box::new(take(r)));
                }
                S::Did
            }
            K::Truth(r) => {
                sub!(self, r, d);
                t.k = K::Val(Value::Bool(fail!(interp::truth(v(r), r.span))));
                S::Did
            }
            K::If(c, a, b) => {
                sub!(self, c, d);
                let chosen = if fail!(interp::truth(v(c), c.span)) { take(a) } else { take(b) };
                replace(t, chosen);
                S::Did
            }
            K::Match(subject, arms) => {
                sub!(self, subject, d);
                let val = v(subject).clone();
                let subject_span = subject.span;
                let mut arms = std::mem::take(arms);
                for mut arm in arms.drain(..) {
                    let mut binds = Vec::new();
                    if !interp::match_pattern(&arm.pat, &val, &mut binds) {
                        continue;
                    }
                    if let Some(mut g) = arm.guard.take() {
                        subst(&mut g, &binds);
                        let gspan = g.span;
                        match self.hidden(g, d) {
                            H::Val(gv) => {
                                if !fail!(interp::truth(&gv, gspan)) {
                                    continue;
                                }
                            }
                            H::Ret(r) => return S::Ret(r),
                            H::Err(e) => return S::Err(e),
                        }
                    }
                    subst(&mut arm.body, &binds);
                    replace(t, arm.body);
                    return S::Did;
                }
                S::Err(interp::no_case(&val, subject_span))
            }
            K::Block(stmts) => {
                if stmts.is_empty() {
                    t.k = K::Val(Value::Unit);
                    return S::Did;
                }
                if stmts.len() == 1 {
                    if let St::Expr(x) = &mut stmts[0] {
                        let new = take(x);
                        replace(t, new);
                        return S::Did;
                    }
                }
                let binds = match &mut stmts[0] {
                    St::Let(p, x) => {
                        sub!(self, x, d);
                        let mut binds = Vec::new();
                        if !interp::match_pattern(p, v(x), &mut binds) {
                            return S::Err(interp::pattern_mismatch(p.span, v(x)));
                        }
                        binds
                    }
                    St::Expr(x) => {
                        sub!(self, x, d);
                        Vec::new()
                    }
                    St::Fns(defs, env) => {
                        let defs: Vec<(Sym, Rc<FnDef>)> = defs.iter().filter_map(|d| Some((d.name?, d.clone()))).collect();
                        let env = std::mem::take(env);
                        let made = fail!(self.make_functions(&defs, false, &env, span));
                        defs.iter().map(|(n, _)| *n).zip(made).collect()
                    }
                    St::SelfFn(name, def, env) => {
                        let (name, def, env) = (*name, def.clone(), std::mem::take(env));
                        let made = fail!(self.make_functions(&[(name, def)], false, &env, span));
                        vec![(name, made[0].clone())]
                    }
                };
                stmts.remove(0);
                let mut rest = Term::new(K::Block(std::mem::take(stmts)), span);
                subst(&mut rest, &binds);
                // `do <expression> end` is that expression.
                if let K::Block(stmts) = &mut rest.k {
                    if let [St::Expr(x)] = stmts.as_mut_slice() {
                        rest = take(x);
                    }
                }
                replace(t, rest);
                S::Did
            }
            K::Try(x) => {
                sub!(self, x, d);
                let val = v(x).clone();
                match self.it.wrap(&val) {
                    Wrap::Some | Wrap::Ok => {
                        t.k = K::Val(Interp::payload(&val));
                        S::Did
                    }
                    Wrap::None | Wrap::Err => S::Ret(val),
                    Wrap::Other => S::Err(interp::try_error(&val, span)),
                }
            }
            K::Range(a, b) => {
                sub!(self, a, d);
                sub!(self, b, d);
                t.k = K::Val(fail!(interp::range(v(a), v(b), span)));
                S::Did
            }
            K::Frame(body, info) => {
                // A call in the last position of a function replaces it (a tail call).
                if matches!(body.k, K::Frame(..)) {
                    let new = take(body);
                    replace(t, new);
                    return S::Did;
                }
                match self.step(body, d) {
                    S::Val => {
                        let new = take(body);
                        replace(t, new);
                        S::Did
                    }
                    S::Did => S::Did,
                    S::Ret(r) => {
                        t.k = K::Val(r);
                        S::Did
                    }
                    S::Err(mut e) => {
                        if let Some((def, call)) = info {
                            interp::add_frame(&mut e, def, *call);
                        }
                        S::Err(e)
                    }
                }
            }
        }
    }

    /// The function `name` of `value.name(...)`, when the value has no such field.
    fn method_function(&self, name: Sym, nspan: Span) -> Fallible<Value> {
        self.lookup(name, nspan).map_err(|mut d| {
            d.message = t!("no function named `{}`", "aucune fonction nommée `{}`", name);
            d.notes.push(t!("`value.{0}(...)` calls `{0}(value, ...)`", "`valeur.{0}(...)` appelle `{0}(valeur, ...)`", name));
            d
        })
    }

    /// A call: the body of the function, with its arguments in place and
    /// simplified, or the result of a built-in function.
    fn call(&mut self, f: Value, args: Vec<Value>, span: Span, depth: u32) -> Fallible<Term> {
        let Value::Func(clo) = &f else { return Ok(Term::val(self.it.call(f, args, span)?, span)) };
        let Some(env) = self.env_for(clo) else { return Ok(Term::val(self.it.call(f, args, span)?, span)) };
        let def = clo.def.clone();
        if args.len() != def.arity {
            return Err(interp::fn_arity_error(&def, args.len(), span));
        }
        'clauses: for clause in &def.clauses {
            let mut binds = Vec::new();
            for (p, a) in clause.params.iter().zip(&args) {
                if !interp::match_pattern(p, a, &mut binds) {
                    continue 'clauses;
                }
            }
            let mut sub: Sub = env.iter().map(|(s, v)| (*s, Some(v.clone()))).collect();
            sub.extend(binds.into_iter().map(|(s, v)| (s, Some(v))));
            if let Some(g) = &clause.guard {
                match self.hidden(convert(g, &mut sub), depth) {
                    H::Val(gv) => {
                        if !interp::truth(&gv, g.span)? {
                            continue;
                        }
                    }
                    // A `?` in a guard ends the call.
                    H::Ret(r) => return Ok(Term::val(r, span)),
                    H::Err(e) => return Err(e),
                }
            }
            let body = convert(&clause.body, &mut sub);
            let mut frame = Term::new(K::Frame(Box::new(body), Some((def.clone(), span))), span);
            self.pure(&mut frame, depth);
            return Ok(frame);
        }
        Err(interp::no_clause(&def, &args, span))
    }

    /// Evaluates a term without showing its steps.
    fn hidden(&mut self, mut t: Term, depth: u32) -> H {
        loop {
            match self.step(&mut t, depth) {
                S::Val => return H::Val(v(&t).clone()),
                S::Did => {
                    if let Err(e) = self.count(t.span) {
                        return H::Err(e);
                    }
                }
                S::Ret(r) => return H::Ret(r),
                S::Err(e) => return H::Err(e),
            }
        }
    }

    fn count(&mut self, span: Span) -> Fallible<()> {
        if self.left == 0 {
            return Err(too_many_steps(self.opts.limit, span));
        }
        self.left -= 1;
        Ok(())
    }

    /// Simplifies `t` as far as possible without calling a function and
    /// without an error: the operations on known values, the cases chosen by
    /// known values. Returns whether `t` is now a value.
    fn pure(&mut self, t: &mut Term, depth: u32) -> bool {
        if depth > MAX_DEPTH {
            return false;
        }
        let d = depth + 1;
        let span = t.span;
        loop {
            let done = match &mut t.k {
                K::Val(_) => {
                    t.span = t.outer;
                    return true;
                }
                K::Local(_) => return false,
                K::Global(s) => match self.lookup(*s, span) {
                    Ok(val) => K::Val(val),
                    Err(_) => return false,
                },
                K::Template(parts) => {
                    let mut all = true;
                    for p in parts.iter_mut() {
                        if let Part::Expr(x) = p {
                            all &= self.pure(x, d);
                        }
                    }
                    match all.then(|| template(parts, span)) {
                        Some(Ok(val)) => K::Val(val),
                        _ => return false,
                    }
                }
                K::List(items) => {
                    let mut all = true;
                    for it in items.iter_mut() {
                        match it {
                            Elem::One(x) => all &= self.pure(x, d),
                            Elem::Spread(x) => all &= self.pure(x, d) && matches!(x.value(), Some(Value::List(_))),
                        }
                    }
                    match all.then(|| list(items, span)) {
                        Some(Ok(val)) => K::Val(val),
                        _ => return false,
                    }
                }
                K::Record(items) => {
                    let mut all = true;
                    for it in items.iter_mut() {
                        match it {
                            Fld::Field(_, x) => all &= self.pure(x, d),
                            Fld::Spread(x) => all &= self.pure(x, d) && matches!(x.value(), Some(Value::Record(_))),
                        }
                    }
                    if !all {
                        return false;
                    }
                    K::Val(record(items))
                }
                K::Lambda(def, env) => match self.lambda(&def.clone(), &env.clone(), span) {
                    Ok(val) => K::Val(val),
                    Err(_) => return false,
                },
                K::Call(f, args, _) => {
                    let mut all = self.pure(f, d);
                    for a in args.iter_mut() {
                        all &= self.pure(a, d);
                    }
                    // Only a value of a type is made without a call.
                    match f.value() {
                        Some(Value::Ctor(c)) if all && c.fields.len() == args.len() => {
                            let fields = args.iter().map(|a| v(a).clone()).collect();
                            K::Val(Value::Variant(Rc::new(Variant { ctor: c.clone(), fields })))
                        }
                        _ => return false,
                    }
                }
                K::Method(recv, ..) => {
                    if !self.pure(recv, d) {
                        if let K::Method(_, _, _, args, _) = &mut t.k {
                            for a in args.iter_mut() {
                                self.pure(a, d);
                            }
                        }
                        return false;
                    }
                    match self.step(t, d) {
                        S::Did => continue,
                        _ => return false,
                    }
                }
                K::Field(x, name, nspan) => {
                    if !self.pure(x, d) {
                        return false;
                    }
                    match self.it.field(v(x), name, *nspan) {
                        Ok(val) => K::Val(val),
                        Err(_) => return false,
                    }
                }
                K::Index(x, i) => {
                    let a = self.pure(x, d);
                    if !(self.pure(i, d) && a) {
                        return false;
                    }
                    match interp::index(v(x), v(i), span, i.span) {
                        Ok(val) => K::Val(val),
                        Err(_) => return false,
                    }
                }
                K::Unary(op, x) => {
                    if !self.pure(x, d) {
                        return false;
                    }
                    match interp::unary(*op, v(x), span, x.span) {
                        Ok(val) => K::Val(val),
                        Err(_) => return false,
                    }
                }
                K::Binary(op, l, r, ospan) => {
                    let a = self.pure(l, d);
                    if !(self.pure(r, d) && a) {
                        return false;
                    }
                    match interp::binop(*op, v(l), v(r), *ospan) {
                        Ok(val) => K::Val(val),
                        Err(_) => return false,
                    }
                }
                K::Range(a, b) => {
                    let x = self.pure(a, d);
                    if !(self.pure(b, d) && x) {
                        return false;
                    }
                    match interp::range(v(a), v(b), span) {
                        Ok(val) => K::Val(val),
                        Err(_) => return false,
                    }
                }
                K::Logic(op, l, r) => {
                    if !self.pure(l, d) {
                        return false;
                    }
                    match v(l) {
                        Value::Bool(left) if (*op == BinOp::And) != *left => K::Val(Value::Bool(*left)),
                        Value::Bool(_) => {
                            t.k = K::Truth(Box::new(take(r)));
                            continue;
                        }
                        _ => return false,
                    }
                }
                K::Truth(r) => {
                    if !self.pure(r, d) {
                        return false;
                    }
                    match v(r) {
                        Value::Bool(b) => K::Val(Value::Bool(*b)),
                        _ => return false,
                    }
                }
                K::If(c, a, b) => {
                    if !self.pure(c, d) {
                        return false;
                    }
                    match v(c) {
                        Value::Bool(true) => {
                            let new = take(a);
                            replace(t, new);
                            continue;
                        }
                        Value::Bool(false) => {
                            let new = take(b);
                            replace(t, new);
                            continue;
                        }
                        _ => return false,
                    }
                }
                K::Match(subject, arms) => {
                    if !self.pure(subject, d) {
                        return false;
                    }
                    let val = v(subject).clone();
                    let mut chosen = None;
                    for (i, arm) in arms.iter().enumerate() {
                        let mut binds = Vec::new();
                        if !interp::match_pattern(&arm.pat, &val, &mut binds) {
                            continue;
                        }
                        if let Some(g) = &arm.guard {
                            let mut g = clone_term(g);
                            subst(&mut g, &binds);
                            if !self.pure(&mut g, d) {
                                return false;
                            }
                            match v(&g) {
                                Value::Bool(true) => {}
                                Value::Bool(false) => continue,
                                _ => return false,
                            }
                        }
                        chosen = Some((i, binds));
                        break;
                    }
                    let Some((i, binds)) = chosen else { return false };
                    let mut body = take(&mut arms[i].body);
                    subst(&mut body, &binds);
                    replace(t, body);
                    continue;
                }
                K::Block(stmts) => {
                    let alone = stmts.len() == 1;
                    match stmts.first_mut() {
                        None => K::Val(Value::Unit),
                        Some(St::Expr(x)) if alone => {
                            let new = take(x);
                            replace(t, new);
                            continue;
                        }
                        Some(St::Let(p, x)) => {
                            if !self.pure(x, d) {
                                return false;
                            }
                            let mut binds = Vec::new();
                            if !interp::match_pattern(p, v(x), &mut binds) {
                                return false;
                            }
                            stmts.remove(0);
                            let mut rest = Term::new(K::Block(std::mem::take(stmts)), span);
                            subst(&mut rest, &binds);
                            replace(t, rest);
                            continue;
                        }
                        Some(St::Expr(x)) => {
                            if !self.pure(x, d) {
                                return false;
                            }
                            stmts.remove(0);
                            continue;
                        }
                        Some(St::Fns(..) | St::SelfFn(..)) => match self.step(t, d) {
                            S::Did => continue,
                            _ => return false,
                        },
                    }
                }
                K::Try(x) => {
                    if !self.pure(x, d) {
                        return false;
                    }
                    let val = v(x);
                    match self.it.wrap(val) {
                        Wrap::Some | Wrap::Ok => K::Val(Interp::payload(val)),
                        _ => return false,
                    }
                }
                K::Frame(body, _) => {
                    if matches!(body.k, K::Frame(..)) {
                        let new = take(body);
                        replace(t, new);
                        continue;
                    }
                    if !self.pure(body, d) {
                        return false;
                    }
                    let new = take(body);
                    replace(t, new);
                    return true;
                }
            };
            t.k = done;
            return true;
        }
    }

    /// Evaluates an expression of the top level (or the body of a test, with
    /// `in_function`), showing its steps. `head` is written before the first
    /// line (`x = ` for a definition).
    pub fn eval(&mut self, e: &Expr, head: &str, in_function: bool) -> Fallible<Value> {
        let mut t = convert(e, &mut Vec::new());
        if in_function {
            t = Term::new(K::Frame(Box::new(t), None), e.span);
        }
        let show = self.opts.show;
        let mut last = Vec::new();
        if show {
            last = self.render(&t);
            for (i, line) in last.iter().enumerate() {
                let head = if i == 0 { head } else { "" };
                crate::io::out_line(&clip(&format!("{head}{line}"), self.opts.width));
            }
        }
        let mut shown = 0;
        let mut hid = false;
        loop {
            match self.step(&mut t, 0) {
                S::Val => break,
                S::Did => {
                    self.count(t.span)?;
                    if !show {
                        continue;
                    }
                    if shown < self.opts.max_steps {
                        let lines = self.render(&t);
                        if lines != last {
                            shown += 1;
                            self.write_step(&lines);
                            last = lines;
                        }
                    } else if !hid {
                        hid = true;
                        let n = self.opts.max_steps;
                        crate::io::out_line(&t!(
                            "  ... (more steps, not shown: {n} are shown at most, change it with --max)",
                            "  ... (d'autres étapes, non montrées : {n} au plus sont montrées, à changer avec --max)"
                        ));
                    }
                }
                S::Ret(r) if in_function => {
                    t = Term::val(r, t.span);
                }
                S::Ret(_) => return Err(interp::question_outside(e.span)),
                S::Err(d) => return Err(d),
            }
        }
        let result = v(&t).clone();
        if hid {
            let lines = self.render(&t);
            self.write_step(&lines);
        }
        Ok(result)
    }

    fn write_step(&self, lines: &[String]) {
        for (i, line) in lines.iter().enumerate() {
            let head = if i == 0 { "  → " } else { "    " };
            crate::io::out_line(&clip(&format!("{head}{line}"), self.opts.width));
        }
    }

    /// The lines that show a term.
    fn render(&self, t: &Term) -> Vec<String> {
        let mut p = Printer { out: String::new(), indent: 0, budget: self.opts.width * (MAX_LINES + 1) + 64, known: &self.known };
        p.term(t, 0);
        let mut lines: Vec<String> = p.out.lines().map(str::to_string).collect();
        if p.out.len() > p.budget || lines.len() > MAX_LINES {
            lines.truncate(MAX_LINES);
            if let Some(l) = lines.last_mut() {
                l.push_str(" …");
            }
        }
        lines
    }
}

/// Cuts a line to `width` characters.
fn clip(line: &str, width: usize) -> String {
    if line.chars().count() <= width {
        return line.to_string();
    }
    let mut s: String = line.chars().take(width.saturating_sub(1).max(1)).collect();
    s.push('…');
    s
}

fn template(parts: &[Part], span: Span) -> Fallible<Value> {
    let mut s = String::new();
    for p in parts {
        match p {
            Part::Lit(l) => s.push_str(l),
            Part::Expr(x) => v(x).write_display(&mut s),
        }
        if s.len() > MAX_TEXT {
            return Err(too_big(false, span));
        }
    }
    Ok(Value::Text(Rc::from(s)))
}

fn list(items: &[Elem], span: Span) -> Fallible<Value> {
    let mut out = Vec::with_capacity(items.len());
    for it in items {
        match it {
            Elem::One(x) => out.push(v(x).clone()),
            Elem::Spread(x) => match v(x) {
                Value::List(l) if out.len() + l.len() > MAX_LIST => return Err(too_big(true, span)),
                Value::List(l) => out.extend(l.iter().cloned()),
                other => return Err(interp::spread_error(other, x.span, true)),
            },
        }
    }
    Ok(Value::list(out))
}

/// Like the interpreter: different names and no `..` make a record of a
/// known shape; otherwise the fields are set one after the other.
fn record(items: &[Fld]) -> Value {
    let mut keys: Vec<Rc<str>> = Vec::new();
    for it in items {
        match it {
            Fld::Field(k, _) if !keys.contains(k) => keys.push(k.clone()),
            _ => break,
        }
    }
    if keys.len() == items.len() {
        let vals = items
            .iter()
            .map(|it| match it {
                Fld::Field(_, x) | Fld::Spread(x) => v(x).clone(),
            })
            .collect();
        return Value::Record(Rc::new(Record::with_shape(Rc::new(Shape::new(keys)), vals)));
    }
    let mut r = Record::new();
    for (i, it) in items.iter().enumerate() {
        match it {
            Fld::Field(k, x) => r.set(k.clone(), v(x).clone()),
            Fld::Spread(x) => {
                if let Value::Record(src) = v(x) {
                    if i == 0 {
                        r = (**src).clone();
                    } else {
                        for (k, val) in src.iter() {
                            r.set(k.clone(), val.clone());
                        }
                    }
                }
            }
        }
    }
    Value::Record(Rc::new(r))
}

/// A copy of a term (the guard of a case, tried without being used up).
fn clone_term(t: &Term) -> Term {
    let b = |x: &Term| Box::new(clone_term(x));
    let k = match &t.k {
        K::Val(v) => K::Val(v.clone()),
        K::Local(s) => K::Local(*s),
        K::Global(s) => K::Global(*s),
        K::Template(parts) => K::Template(
            parts
                .iter()
                .map(|p| match p {
                    Part::Lit(l) => Part::Lit(l.clone()),
                    Part::Expr(x) => Part::Expr(clone_term(x)),
                })
                .collect(),
        ),
        K::List(items) => K::List(
            items
                .iter()
                .map(|it| match it {
                    Elem::One(x) => Elem::One(clone_term(x)),
                    Elem::Spread(x) => Elem::Spread(clone_term(x)),
                })
                .collect(),
        ),
        K::Record(items) => K::Record(
            items
                .iter()
                .map(|it| match it {
                    Fld::Field(k, x) => Fld::Field(k.clone(), clone_term(x)),
                    Fld::Spread(x) => Fld::Spread(clone_term(x)),
                })
                .collect(),
        ),
        K::Lambda(d, env) => K::Lambda(d.clone(), env.clone()),
        K::Call(f, args, look) => K::Call(b(f), args.iter().map(clone_term).collect(), look.clone()),
        K::Method(r, n, f, args, s) => K::Method(b(r), *n, b(f), args.iter().map(clone_term).collect(), *s),
        K::Field(x, n, s) => K::Field(b(x), n.clone(), *s),
        K::Index(x, y) => K::Index(b(x), b(y)),
        K::Unary(op, x) => K::Unary(*op, b(x)),
        K::Binary(op, x, y, s) => K::Binary(*op, b(x), b(y), *s),
        K::Logic(op, x, y) => K::Logic(*op, b(x), b(y)),
        K::Truth(x) => K::Truth(b(x)),
        K::If(c, x, y) => K::If(b(c), b(x), b(y)),
        K::Match(s, arms) => K::Match(
            b(s),
            arms.iter().map(|a| Arm { pat: a.pat.clone(), guard: a.guard.as_ref().map(clone_term), body: clone_term(&a.body) }).collect(),
        ),
        K::Block(stmts) => K::Block(
            stmts
                .iter()
                .map(|s| match s {
                    St::Let(p, x) => St::Let(p.clone(), clone_term(x)),
                    St::Expr(x) => St::Expr(clone_term(x)),
                    St::Fns(d, env) => St::Fns(d.clone(), env.clone()),
                    St::SelfFn(n, d, env) => St::SelfFn(*n, d.clone(), env.clone()),
                })
                .collect(),
        ),
        K::Try(x) => K::Try(b(x)),
        K::Range(x, y) => K::Range(b(x), b(y)),
        K::Frame(x, info) => K::Frame(b(x), info.clone()),
    };
    Term { k, span: t.span, outer: t.outer }
}

fn too_deep(span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "",
        t!(
            "this calculation became too deep to be shown step by step (more than {MAX_DEPTH} operations inside each other)",
            "ce calcul est devenu trop profond pour être montré étape par étape (plus de {MAX_DEPTH} opérations les unes dans les autres)"
        ),
    )
    .at(span)
    .help(t!(
        "try it with smaller values, or run the program normally: `rythme file.ry`",
        "essayez avec des valeurs plus petites, ou lancez le programme normalement : `rythme fichier.ry`"
    ))
    .boxed()
}

fn too_many_steps(limit: u64, span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "",
        t!(
            "the evaluation was stopped after {limit} steps: does it ever end?",
            "l'évaluation a été arrêtée après {limit} étapes : se termine-t-elle un jour ?"
        ),
    )
    .at(span)
    .help(t!(
        "a function that calls itself must always reach a case where it stops",
        "une fonction qui s'appelle elle-même doit toujours atteindre un cas où elle s'arrête"
    ))
    .boxed()
}

// ----- printing -----------------------------------------------------------

/// Writes terms in the style of `rythme fmt`.
struct Printer<'k> {
    out: String,
    indent: usize,
    budget: usize,
    known: &'k HashMap<usize, Known>,
}

// How tightly each kind of expression binds (see the table of precedence
// in the manual): an operand that binds less tightly gets parentheses.
const LOW: u8 = 0;
const OR: u8 = 1;
const AND: u8 = 2;
const NOT: u8 = 3;
const CMP: u8 = 4;
const RANGE: u8 = 6;
const ADD: u8 = 7;
const MUL: u8 = 8;
const NEG: u8 = 9;
const POW: u8 = 10;
const ATOM: u8 = 11;

fn op_level(op: BinOp) -> u8 {
    match op {
        BinOp::Or => OR,
        BinOp::And => AND,
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => CMP,
        BinOp::Add | BinOp::Sub => ADD,
        BinOp::Mul | BinOp::Div | BinOp::IntDiv | BinOp::Mod => MUL,
        BinOp::Pow => POW,
    }
}

impl Printer<'_> {
    fn full(&self) -> bool {
        self.out.len() > self.budget
    }

    fn push(&mut self, s: &str) {
        self.out.push_str(s);
    }

    fn newline(&mut self) {
        self.out.push('\n');
        for _ in 0..self.indent {
            self.out.push_str("  ");
        }
    }

    fn value_level(&self, v: &Value) -> u8 {
        match v {
            Value::Int(n) if *n < 0 => NEG,
            Value::Float(f) if f.is_sign_negative() && !f.is_nan() => NEG,
            Value::Func(c) if c.def.name.is_none() => match self.known.get(&(Rc::as_ptr(c) as usize)) {
                Some(Known { name: Some(_), .. }) | None => ATOM,
                Some(_) => LOW,
            },
            _ => ATOM,
        }
    }

    fn level(&self, t: &Term) -> u8 {
        match &t.k {
            K::Val(v) => self.value_level(v),
            K::Lambda(..) | K::If(..) => LOW,
            K::Unary(UnOp::Neg, _) => NEG,
            K::Unary(UnOp::Not, _) => NOT,
            K::Binary(op, ..) | K::Logic(op, ..) => op_level(*op),
            K::Range(..) => RANGE,
            K::Truth(x) | K::Frame(x, _) => self.level(x),
            _ => ATOM,
        }
    }

    /// Writes `t`, in parentheses if it binds less tightly than `need`.
    fn term(&mut self, t: &Term, need: u8) {
        if self.full() {
            return;
        }
        let paren = self.level(t) < need;
        if paren {
            self.push("(");
        }
        self.bare(t);
        if paren {
            self.push(")");
        }
    }

    fn args(&mut self, args: &[Term]) {
        self.push("(");
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                self.push(", ");
            }
            self.term(a, LOW);
        }
        self.push(")");
    }

    fn bare(&mut self, t: &Term) {
        match &t.k {
            K::Val(v) => self.value(v),
            K::Local(s) | K::Global(s) => self.push(&s.name()),
            K::Template(parts) => {
                self.push("\"");
                for p in parts {
                    match p {
                        Part::Lit(l) => self.text_inside(l),
                        Part::Expr(x) => match &x.k {
                            K::Val(v) => self.text_inside(&v.display()),
                            _ => {
                                self.push("{");
                                self.term(x, LOW);
                                self.push("}");
                            }
                        },
                    }
                }
                self.push("\"");
            }
            K::List(items) => {
                self.push("[");
                for (i, it) in items.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    match it {
                        Elem::One(x) => self.term(x, LOW),
                        Elem::Spread(x) => {
                            self.push("..");
                            self.term(x, ATOM);
                        }
                    }
                }
                self.push("]");
            }
            K::Record(items) => {
                self.push("{");
                for (i, it) in items.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    match it {
                        Fld::Field(k, x) => {
                            self.key(k);
                            self.term(x, LOW);
                        }
                        Fld::Spread(x) => {
                            self.push("..");
                            self.term(x, ATOM);
                        }
                    }
                }
                self.push("}");
            }
            K::Lambda(def, env) => self.lambda(def, env),
            K::Call(f, args, look) => match look {
                Look::Plain => {
                    self.term(f, ATOM);
                    self.args(args);
                }
                Look::Method(name) => {
                    self.term(&args[0], ATOM);
                    self.push(".");
                    self.push(&name.name());
                    self.args(&args[1..]);
                }
                Look::Field(recv, name) => {
                    if self.value_level(recv) < ATOM {
                        self.push("(");
                        self.value(recv);
                        self.push(")");
                    } else {
                        self.value(recv);
                    }
                    self.push(".");
                    self.push(&name.name());
                    self.args(args);
                }
            },
            K::Method(recv, name, _, args, _) => {
                self.term(recv, ATOM);
                self.push(".");
                self.push(&name.name());
                self.args(args);
            }
            K::Field(x, name, _) => {
                self.term(x, ATOM);
                self.push(".");
                self.push(name);
            }
            K::Index(x, i) => {
                self.term(x, ATOM);
                self.push("[");
                self.term(i, LOW);
                self.push("]");
            }
            K::Unary(UnOp::Neg, x) => {
                self.push("-");
                self.term(x, POW);
            }
            K::Unary(UnOp::Not, x) => {
                self.push("not ");
                self.term(x, CMP);
            }
            K::Binary(op, l, r, _) | K::Logic(op, l, r) => {
                let level = op_level(*op);
                let (left, right) = match op {
                    BinOp::Pow => (ATOM, NEG),
                    _ if level == CMP => (CMP + 1, CMP + 1),
                    _ => (level, level + 1),
                };
                self.term(l, left);
                self.push(&format!(" {} ", op.symbol()));
                self.term(r, right);
            }
            K::Range(a, b) => {
                self.term(a, RANGE + 1);
                self.push("..");
                self.term(b, RANGE + 1);
            }
            K::Truth(x) | K::Frame(x, _) => self.bare(x),
            K::If(c, a, b) => {
                self.push("if ");
                self.term(c, OR);
                self.push(" then ");
                self.term(a, OR);
                self.push(" else ");
                self.term(b, LOW);
            }
            K::Match(subject, arms) => {
                self.push("match ");
                self.term(subject, OR);
                self.indent += 1;
                for arm in arms {
                    self.newline();
                    self.pattern(&arm.pat);
                    if let Some(g) = &arm.guard {
                        self.push(" if ");
                        self.term(g, LOW);
                    }
                    self.push(" -> ");
                    self.term(&arm.body, LOW);
                }
                self.indent -= 1;
                self.newline();
                self.push("end");
            }
            K::Block(stmts) => {
                self.push("do");
                self.statements(stmts);
            }
            K::Try(x) => {
                self.term(x, ATOM);
                self.push("?");
            }
        }
    }

    fn key(&mut self, k: &str) {
        if is_plain_key(k) {
            self.push(k);
        } else {
            write_text_repr(k, &mut self.out);
        }
        self.push(": ");
    }

    /// Characters of a text, written between its quotes.
    fn text_inside(&mut self, s: &str) {
        let mut quoted = String::new();
        write_text_repr(s, &mut quoted);
        self.push(&quoted[1..quoted.len() - 1]);
    }

    fn value(&mut self, v: &Value) {
        if self.full() {
            return;
        }
        match v {
            Value::Unit => self.push("()"),
            Value::Bool(b) => self.push(if *b { "true" } else { "false" }),
            Value::Int(n) => self.push(&n.to_string()),
            Value::Float(f) => self.push(&fmt_float(*f)),
            Value::Text(s) => write_text_repr(s, &mut self.out),
            Value::List(l) => {
                self.push("[");
                for (i, x) in l.iter().enumerate() {
                    if self.full() {
                        break;
                    }
                    if i > 0 {
                        self.push(", ");
                    }
                    self.value(x);
                }
                self.push("]");
            }
            Value::Record(r) => {
                self.push("{");
                for (i, (k, x)) in r.iter().enumerate() {
                    if self.full() {
                        break;
                    }
                    if i > 0 {
                        self.push(", ");
                    }
                    self.key(k);
                    self.value(x);
                }
                self.push("}");
            }
            Value::Variant(var) => {
                self.push(&var.ctor.name.name());
                if !var.fields.is_empty() {
                    self.push("(");
                    for (i, x) in var.fields.iter().enumerate() {
                        if i > 0 {
                            self.push(", ");
                        }
                        self.value(x);
                    }
                    self.push(")");
                }
            }
            Value::Func(c) => match c.def.name {
                Some(n) => self.push(&n.shown().name()),
                None => match self.known.get(&(Rc::as_ptr(c) as usize)) {
                    Some(Known { name: Some(n), .. }) => self.push(&n.name()),
                    Some(Known { env, .. }) => {
                        let (def, env) = (c.def.clone(), env.clone());
                        self.lambda(&def, &env);
                    }
                    None => self.push("<fn>"),
                },
            },
            Value::Builtin(b) => self.push(b.name),
            Value::Ctor(c) => self.push(&c.name.name()),
        }
    }

    fn pattern(&mut self, p: &Pattern) {
        match &p.kind {
            PatKind::Wildcard => self.push("_"),
            PatKind::Bind(s) => self.push(&s.name()),
            PatKind::Unit => self.push("()"),
            PatKind::Bool(b) => self.push(if *b { "true" } else { "false" }),
            PatKind::Int(n) => self.push(&n.to_string()),
            PatKind::Float(f) => self.push(&fmt_float(*f)),
            PatKind::Text(s) => write_text_repr(s, &mut self.out),
            PatKind::Ctor(name, ps) => {
                self.push(&name.name());
                if !ps.is_empty() {
                    self.push("(");
                    for (i, p) in ps.iter().enumerate() {
                        if i > 0 {
                            self.push(", ");
                        }
                        self.pattern(p);
                    }
                    self.push(")");
                }
            }
            PatKind::List(ps, rest) => {
                self.push("[");
                for (i, p) in ps.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.pattern(p);
                }
                if let Some(rest) = rest {
                    if !ps.is_empty() {
                        self.push(", ");
                    }
                    self.push("..");
                    if let Some((s, _)) = rest {
                        self.push(&s.name());
                    }
                }
                self.push("]");
            }
            PatKind::Record(fields) => {
                self.push("{");
                for (i, (k, p)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    match &p.kind {
                        PatKind::Bind(s) if *s.name() == **k => self.push(k),
                        _ => {
                            self.key(k);
                            self.pattern(p);
                        }
                    }
                }
                self.push("}");
            }
            PatKind::Or(alts) => {
                for (i, p) in alts.iter().enumerate() {
                    if i > 0 {
                        self.push(" | ");
                    }
                    self.pattern(p);
                }
            }
        }
    }

    fn params(&mut self, params: &[Pattern]) {
        self.push("(");
        for (i, p) in params.iter().enumerate() {
            if i > 0 {
                self.push(", ");
            }
            self.pattern(p);
        }
        self.push(")");
    }

    /// The body of a clause, with the values of `env` in place: after ` = `,
    /// or on the next lines for a block.
    fn body(&mut self, clause: &Clause, env: &Env) {
        let mut sub: Sub = env.iter().map(|(s, v)| (*s, Some(v.clone()))).collect();
        for p in &clause.params {
            sub.extend(bound_names(p).into_iter().map(|s| (s, None)));
        }
        if let Some(g) = &clause.guard {
            self.push(" if ");
            let g = convert(g, &mut sub);
            self.term(&g, LOW);
        }
        let body = convert(&clause.body, &mut sub);
        match &body.k {
            K::Block(stmts) => self.statements(stmts),
            _ => {
                self.push(" = ");
                self.term(&body, LOW);
            }
        }
    }

    /// The statements of a block, one per line, then `end`.
    fn statements(&mut self, stmts: &[St]) {
        self.indent += 1;
        for st in stmts {
            self.newline();
            match st {
                St::Let(p, x) => {
                    self.pattern(p);
                    self.push(" = ");
                    self.term(x, LOW);
                }
                St::Expr(x) => self.term(x, LOW),
                St::Fns(defs, env) => {
                    for (i, def) in defs.iter().enumerate() {
                        if i > 0 {
                            self.newline();
                        }
                        self.named(def, env);
                    }
                }
                St::SelfFn(name, def, env) => {
                    self.push(&name.name());
                    self.push(" = ");
                    self.lambda(def, env);
                }
            }
        }
        self.indent -= 1;
        self.newline();
        self.push("end");
    }

    fn lambda(&mut self, def: &FnDef, env: &Env) {
        self.push("fn");
        if let Some(clause) = def.clauses.first() {
            self.params(&clause.params);
            self.body(clause, env);
        }
    }

    fn named(&mut self, def: &FnDef, env: &Env) {
        for (i, clause) in def.clauses.iter().enumerate() {
            if i > 0 {
                self.newline();
            }
            self.push("fn ");
            self.push(&def.name.map(|n| n.name()).unwrap_or_else(|| Rc::from("")));
            self.params(&clause.params);
            self.body(clause, env);
        }
    }
}

// ----- running files ------------------------------------------------------

/// Shows the evaluation of the top-level definitions and expressions of a
/// checked module, step by step (its functions and types are defined first,
/// as when it runs). With `only_last`, the items before the last ones run
/// without being shown, and their expressions are skipped (`rythme steps
/// file.ry "fib(5)"`).
pub fn run_module(driver: &mut Driver, m: &Rc<Module>, opts: Options, only_last: Option<usize>) -> Result<(), Reported> {
    let repl = driver.repl;
    driver.prepare(m, Mode::Run)?;
    let first_shown = only_last.map(|n| m.items.len().saturating_sub(n)).unwrap_or(0);
    match run_items(&mut driver.interp, &m.items, opts, first_shown, repl) {
        Ok(shown) => {
            if !shown && opts.show {
                crate::io::out_line(&t!(
                    "Nothing to show: this file only has definitions. Give an expression to compute, like: rythme steps file.ry \"fib(5)\"",
                    "Rien à montrer : ce fichier n'a que des définitions. Donnez une expression à calculer, comme : rythme etapes fichier.ry \"fib(5)\""
                ));
            }
            Ok(())
        }
        Err(d) => {
            driver.report(&d);
            Err(Reported)
        }
    }
}

/// Runs checked items like `Interp::exec_items` in `Mode::Run`, with the
/// stepper, from the item `first_shown` on (the definitions before it are
/// made by the interpreter, and the expressions before it are skipped).
/// `repl`: the functions typed in earlier inputs are shown step by step too.
/// Returns whether an expression was shown.
pub fn run_items(it: &mut Interp, items: &[Item], opts: Options, first_shown: usize, repl: bool) -> Fallible<bool> {
    it.define_items(items);
    let mut st = Stepper::new(it, opts);
    st.files = items.iter().filter_map(item_file).collect();
    st.repl = repl;
    let mut shown_any = false;
    for (i, item) in items.iter().enumerate() {
        let (pat, e) = match item {
            Item::Stmt(Stmt::Let(pat, e)) => (Some(pat), e),
            Item::Stmt(Stmt::Expr(e)) => (None, e),
            _ => continue,
        };
        if i < first_shown {
            if let Some(pat) = pat {
                let v = st.it.eval_top(e)?;
                st.it.bind_global(pat, v)?;
            }
            continue;
        }
        if shown_any && opts.show {
            crate::io::out_line("");
        }
        shown_any = true;
        let head = pat.map(|p| format!("{} = ", pattern_text(p))).unwrap_or_default();
        let v = st.eval(e, &head, false)?;
        if let Some(pat) = pat {
            st.it.bind_global(pat, v)?;
        }
    }
    Ok(shown_any)
}

/// The file of the code of an item.
fn item_file(item: &Item) -> Option<u32> {
    match item {
        Item::Stmt(Stmt::Fn(def)) => Some(def.span.file),
        Item::Stmt(Stmt::Let(p, _)) => Some(p.span.file),
        Item::Stmt(Stmt::Expr(e)) => Some(e.span.file),
        Item::Type(t) => Some(t.span.file),
        Item::Test(_, _, span) => Some(span.file),
        Item::Use(u) => Some(u.span.file),
    }
}

fn pattern_text(p: &Pattern) -> String {
    let known = HashMap::new();
    let mut pr = Printer { out: String::new(), indent: 0, budget: usize::MAX, known: &known };
    pr.pattern(p);
    pr.out
}

/// Evaluates the body of a test with the stepper, without showing it (to
/// compare with the interpreter).
pub fn eval_test(it: &mut Interp, test: &interp::TestCase, opts: Options) -> Fallible<Value> {
    it.names = test.names.clone();
    let mut st = Stepper::new(it, opts);
    st.files.insert(test.span.file);
    st.eval(&test.body, "", true)
}

/// `rythme steps file.ry [expression] [--max N] [--width N]`.
pub fn command(args: &[String]) -> i32 {
    let mut opts = Options::default();
    let mut rest: Vec<&String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        let (flag, value) = match a.split_once('=') {
            Some((f, v)) if f.starts_with("--") => (f, Some(v.to_string())),
            _ => (a.as_str(), None),
        };
        if matches!(flag, "--max" | "--width") {
            let value = match value {
                Some(v) => Some(v),
                None => {
                    i += 1;
                    args.get(i).cloned()
                }
            };
            match value.as_deref().map(str::parse::<usize>) {
                Some(Ok(n)) if n > 0 && flag == "--max" => opts.max_steps = n,
                Some(Ok(n)) if n >= 20 && flag == "--width" => opts.width = n,
                _ => {
                    crate::io::err_line(&t!(
                        "`{flag}` needs a number, like `--max 50` or `--width 80` (20 at least)",
                        "`{flag}` attend un nombre, comme `--max 50` ou `--width 80` (20 au moins)"
                    ));
                    return 2;
                }
            }
        } else {
            rest.push(a);
        }
        i += 1;
    }
    let (file, expr) = match rest.as_slice() {
        [file] => (file.as_str(), None),
        [file, expr] => (file.as_str(), Some(expr.as_str())),
        _ => {
            crate::io::err_line(&t!(
                "usage: rythme steps file.ry [expression] [--max N] [--width N]\nexample: rythme steps fibonacci.ry \"fib(4)\"",
                "utilisation : rythme etapes fichier.ry [expression] [--max N] [--width N]\nexemple : rythme etapes fibonacci.ry \"fib(4)\""
            ));
            return 2;
        }
    };
    let mut driver = Driver::new();
    let mut path = std::path::PathBuf::from(file);
    if !path.is_file() && path.extension().is_none() {
        path.set_extension("ry");
    }
    let module = match expr {
        None => driver.load_file(&path),
        Some(code) => {
            // The expression is computed after the definitions of the file.
            let src = match crate::driver::read_source(&path) {
                Ok(s) => s,
                Err(d) => {
                    driver.report(&d);
                    return 1;
                }
            };
            let items = match crate::lexer::lex(code, u32::MAX).and_then(crate::parser::parse) {
                Ok(items) => items.len(),
                Err(_) => 1,
            };
            let base = path.parent().map(std::path::Path::to_path_buf).unwrap_or_default();
            let canon = path.canonicalize().ok();
            let whole = format!("{}\n{code}\n", src.trim_end());
            let name = path.display().to_string();
            match driver.load_source(&name, &whole, canon, &base) {
                Some(m) => return if run_module(&mut driver, &m, opts, Some(items.max(1))).is_ok() { 0 } else { 1 },
                None => return 1,
            }
        }
    };
    match module {
        Some(m) if run_module(&mut driver, &m, opts, None).is_ok() => 0,
        _ => 1,
    }
}
