//! What the parameters of a function must be, inferred from how its code
//! uses them, and the calls that therefore always fail.
//!
//! While the checker goes through a function, it notes, for each parameter,
//! the kinds of values with which the function may still give a value:
//! after `side * side`, only numbers are left for `side`; after `len(x)`,
//! texts, lists and records for `x`. One operation after the other, the
//! kinds narrow down; the branches of an `if` or of a `match`, and the
//! definitions of a function (one per case), add theirs up. A `?` that may
//! leave the function, or a call that may end the program without an error
//! (`exit`), keeps what is known at that point for the function.
//!
//! An argument whose kind is not among them makes the call fail every time
//! (or never end): `area("big")` is reported with the error the program
//! would stop with, found by checking the function again with these
//! arguments. Calls to functions the checker cannot see (another file, the
//! value of a parameter) say nothing about their arguments.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;

use super::{comparable, literal, may_be_equal, Checker, GKind, Res};
use crate::ast::*;
use crate::diagnostic::Diagnostic;
use crate::source::Span;
use crate::symbol::Sym;
use crate::types::{self, Kinds, Outcome, Ty, ALL, BOOL, DATA, FLOAT, FUNC, INT, LIST, NUMBER, OPTION, RECORD, RESULT, SYMBOL, TEXT, UNIT};
use crate::value::Value;

/// Each kind of value, alone.
const EACH: [Kinds; 12] = [UNIT, BOOL, INT, FLOAT, TEXT, SYMBOL, LIST, RECORD, OPTION, RESULT, DATA, FUNC];

/// The name given to "a value of any type of the program" when trying the
/// kinds one by one: [`comparable`] and [`may_be_equal`] take it as the same
/// type as any other.
pub(super) fn any_data() -> Sym {
    Sym::new("#data")
}

/// Any value of the kind `k`.
fn sample(k: Kinds) -> Ty {
    match k {
        UNIT => Ty::Unit,
        BOOL => Ty::Bool,
        INT => Ty::Int,
        FLOAT => Ty::Float,
        TEXT => Ty::Text,
        SYMBOL => Ty::Symbol,
        LIST => Ty::list(Ty::Any),
        RECORD => Ty::record(Vec::new(), false),
        OPTION => Ty::data(Sym::new("Option"), None),
        RESULT => Ty::data(Sym::new("Result"), None),
        DATA => Ty::data(any_data(), None),
        _ => Ty::func(None, Ty::Any, None),
    }
}

/// The kinds whose values make `ok` true.
fn kinds_where(mut ok: impl FnMut(&Ty) -> bool) -> Kinds {
    EACH.iter().filter(|&&k| ok(&sample(k))).fold(0, |all, k| all | k)
}

/// What a function needs, for one way through its code.
struct Frame {
    /// For each parameter, the kinds with which the code gone through so far
    /// may run without error.
    now: Vec<Kinds>,
    /// The same, added up for the ways that already left the function.
    left: Vec<Kinds>,
    /// The function may end the program without an error (`exit`).
    exits: bool,
    /// What the guards of the definitions tried so far need, when every
    /// argument reaches them: a guard that fails stops the call, whatever
    /// the definitions after it.
    guards: Vec<Kinds>,
    /// How many ways left the function so far.
    leaves: usize,
}

/// What the checker knows of the parameters of the functions.
#[derive(Default)]
pub(super) struct Needs {
    frames: Vec<Frame>,
    /// For each top-level function of the module, what its parameters must
    /// be. Empty while the types are inferred; then narrowed round after
    /// round from "anything".
    pub(super) top: HashMap<Sym, Vec<Kinds>>,
    /// Is `top` being narrowed?
    pub(super) narrowing: bool,
    /// The top-level functions that may end the program without an error.
    pub(super) exits: HashSet<Sym>,
    /// Local functions and functions written in place, by the place of
    /// their definition: what they need, and whether they may end the program.
    local: HashMap<Span, (Vec<Kinds>, bool)>,
    /// The same, for the function checked last.
    pub(super) last: (Vec<Kinds>, bool),
    /// The parameter called, in a call being checked (`f(x)` where `f` is a parameter).
    pub(super) callee: Option<usize>,
    /// The last walk, which reports problems.
    pub(super) reporting: bool,
    /// How many functions are being checked again with the arguments of a call.
    specializing: usize,
    /// The next function checked is checked with the arguments of a call:
    /// its definitions that do not match them say nothing.
    quiet: bool,
    /// The signatures inferred for the functions, by the place of their definition.
    pub(super) signatures: Option<HashMap<Span, Signature>>,
}

thread_local! {
    /// The values written in the code of a call, for the parameters of the
    /// function checked again with them: messages show them like the run
    /// time does (`Int 5`).
    static ARGS: RefCell<Vec<Vec<(Sym, Value)>>> = const { RefCell::new(Vec::new()) };
}

/// The value of a parameter given as a value written in the code, while its
/// function is checked again with the arguments of a call.
pub(super) fn literal_arg(s: Sym) -> Option<Value> {
    ARGS.with(|a| a.borrow().last().and_then(|top| top.iter().find(|(n, _)| *n == s).map(|(_, v)| v.clone())))
}

/// What the checker infers of a function: what each parameter must be, and
/// what it gives back. Shown as `fn area(side: Int|Float) -> Int|Float`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    pub name: String,
    /// The name of each parameter, and the kinds of values it must be, when
    /// they are known (`Int|Float`).
    pub params: Vec<(String, Option<String>)>,
    /// What a call gives back, when it is known.
    pub ret: Option<String>,
}

impl Signature {
    /// Does it say more than the names of the parameters?
    pub fn is_informative(&self) -> bool {
        self.ret.is_some() || self.params.iter().any(|(_, t)| t.is_some())
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| match t {
                Some(t) => format!("{n}: {t}"),
                None => n.clone(),
            })
            .collect();
        write!(f, "fn {}({})", self.name, params.join(", "))?;
        if let Some(r) = &self.ret {
            write!(f, " -> {r}")?;
        }
        Ok(())
    }
}

/// The names of the parameters of a function, as in [`FnDef::signature`].
fn param_names(def: &FnDef) -> Vec<String> {
    (0..def.arity)
        .map(|i| {
            def.clauses
                .iter()
                .find_map(|c| match &c.params[i].kind {
                    PatKind::Bind(s) => Some(s.name().to_string()),
                    _ => None,
                })
                .unwrap_or_else(|| format!("arg{}", i + 1))
        })
        .collect()
}

/// The kinds, as shown in a signature: `Int|Float`. Nothing when any kind
/// will do, or when there would be too many.
fn kinds_text(k: Kinds, data: Option<Sym>) -> Option<String> {
    if k == ALL || k == 0 || (k & DATA != 0 && data.is_none()) {
        return None;
    }
    let names: Vec<String> = EACH
        .iter()
        .filter(|&&x| k & x != 0)
        .map(|&x| match x {
            DATA => data.map(|d| d.to_string()).unwrap_or_default(),
            _ => sample(x).name(),
        })
        .collect();
    (names.len() <= 4).then(|| names.join("|"))
}

/// A type, as shown in a signature; nothing when it is not known.
fn type_text(t: &Ty) -> Option<String> {
    match t {
        Ty::Any | Ty::Never => None,
        Ty::Num => Some("Int|Float".into()),
        t => Some(t.name()),
    }
}

impl Checker<'_> {
    // ----- following the ways through a function -------------------------

    pub(super) fn need_enter(&mut self, arity: usize) {
        self.needs.frames.push(Frame { now: vec![ALL; arity], left: vec![0; arity], exits: false, guards: vec![ALL; arity], leaves: 0 });
    }

    /// A definition of the function starts: it runs only for arguments that match its patterns.
    pub(super) fn need_clause(&mut self, params: &[Pattern]) {
        let now: Vec<Kinds> = params.iter().map(|p| self.pattern_kinds(p)).collect();
        if let Some(f) = self.needs.frames.last_mut() {
            f.now = now.into_iter().zip(&f.guards).map(|(k, g)| k & g).collect();
        }
    }

    /// Before the guard of a definition: how many ways left the function.
    pub(super) fn need_guard_start(&self) -> usize {
        self.needs.frames.last().map_or(0, |f| f.leaves)
    }

    /// After the guard of a definition whose patterns accept every
    /// argument: unless it may leave the function (`?`), what it needs is
    /// needed by the definitions after it too.
    pub(super) fn need_guard_end(&mut self, start: usize, params: &[Pattern]) {
        if !params.iter().all(|p| matches!(p.kind, PatKind::Bind(_) | PatKind::Wildcard)) {
            return;
        }
        if let Some(f) = self.needs.frames.last_mut() {
            if f.leaves == start {
                f.guards = f.now.clone();
            }
        }
    }

    /// The end of a definition: a way that leaves the function.
    pub(super) fn need_clause_end(&mut self) {
        self.leaves_early();
    }

    /// The end of the function: what it needs is kept as `last`, and for a
    /// function written in the code, by the place of its definition.
    pub(super) fn need_leave(&mut self, def: &FnDef) {
        let Some(f) = self.needs.frames.pop() else { return };
        self.needs.last = (f.left, f.exits);
        if self.needs.specializing == 0 {
            self.needs.local.insert(def.span, self.needs.last.clone());
        }
    }

    /// A local function is defined again: what was known of it may have been
    /// known in another place, with other values around it.
    pub(super) fn forget_local(&mut self, def: &FnDef) {
        self.needs.local.remove(&def.span);
    }

    /// Marks the name just declared for a parameter of the function being checked.
    pub(super) fn mark_param(&mut self, s: Sym, span: Span, i: usize) {
        let frame = self.needs.frames.len().wrapping_sub(1);
        if let Some(l) = self.scopes.last_mut().and_then(|scope| scope.iter_mut().rev().find(|l| l.sym == s && l.span == span)) {
            l.param = Some((frame, i));
        }
    }

    /// `y = x`, or a case `y -> ...` of a `match` on `x`: `y` is the
    /// parameter `x` too.
    pub(super) fn alias(&mut self, p: &Pattern, e: &Expr) {
        if let (PatKind::Bind(s), Some(i)) = (&p.kind, self.tracked(e)) {
            self.mark_param(*s, p.span, i);
        }
    }

    /// The parameter of the function being checked that the expression is, if it is one.
    pub(super) fn tracked(&self, e: &Expr) -> Option<usize> {
        let ExprKind::Var(s) = &e.kind else { return None };
        let l = self.scopes.iter().rev().flat_map(|scope| scope.iter().rev()).find(|l| l.sym == *s)?;
        let (frame, i) = l.param?;
        (frame + 1 == self.needs.frames.len()).then_some(i)
    }

    /// The expression must be of these kinds, or the function fails here.
    pub(super) fn require(&mut self, e: &Expr, k: Kinds) {
        if let Some(i) = self.tracked(e) {
            self.require_param(i, k);
        }
    }

    fn require_param(&mut self, i: usize, k: Kinds) {
        if let Some(n) = self.needs.frames.last_mut().and_then(|f| f.now.get_mut(i)) {
            *n &= k;
        }
    }

    /// What is known on the way being checked, to check another way from the same point.
    pub(super) fn ways(&self) -> Option<Vec<Kinds>> {
        self.needs.frames.last().map(|f| f.now.clone())
    }

    /// Goes on from this point; gives what was known on the way left.
    pub(super) fn set_ways(&mut self, w: Option<Vec<Kinds>>) -> Option<Vec<Kinds>> {
        match (self.needs.frames.last_mut(), w) {
            (Some(f), Some(w)) => Some(std::mem::replace(&mut f.now, w)),
            _ => None,
        }
    }

    /// Either of the two ways may have been taken.
    pub(super) fn join_ways(&mut self, w: Option<Vec<Kinds>>) {
        if let (Some(f), Some(w)) = (self.needs.frames.last_mut(), w) {
            f.now.iter_mut().zip(w).for_each(|(n, k)| *n |= k);
        }
    }

    /// The function may be left here (`?`).
    pub(super) fn leaves_early(&mut self) {
        if let Some(f) = self.needs.frames.last_mut() {
            f.leaves += 1;
            let now = f.now.clone();
            f.left.iter_mut().zip(now).for_each(|(l, k)| *l |= k);
        }
    }

    /// The program may end here without an error.
    pub(super) fn may_stop(&mut self) {
        if let Some(f) = self.needs.frames.last_mut() {
            f.exits = true;
        }
        self.leaves_early();
    }

    /// The kinds of values a pattern can match.
    pub(super) fn pattern_kinds(&self, p: &Pattern) -> Kinds {
        match &p.kind {
            PatKind::Wildcard | PatKind::Bind(_) => ALL,
            PatKind::Unit => UNIT,
            PatKind::Bool(_) => BOOL,
            // `1` matches `1.0`.
            PatKind::Int(_) | PatKind::Big(_) | PatKind::Float(_) => NUMBER,
            PatKind::Text(_) => TEXT,
            PatKind::Symbol(_) => SYMBOL,
            PatKind::List(..) => LIST,
            PatKind::Record(_) => RECORD,
            PatKind::Or(alts) => alts.iter().fold(0, |k, a| k | self.pattern_kinds(a)),
            PatKind::Ctor(name, args) => match self.ctors.get(name) {
                Some(m) if m.arity == args.len() => Ty::data(m.type_name, None).kinds(),
                // Already an error.
                _ => ALL,
            },
        }
    }

    // ----- what operations need ---------------------------------------------

    /// An operation on these values fails for the kinds of values that make
    /// `fails` true: the parameters among them need the other kinds.
    pub(super) fn need_operands(&mut self, operands: &[(Option<&Expr>, &Ty)], fails: impl Fn(&[Ty]) -> bool) {
        if self.needs.frames.is_empty() || operands.iter().any(|(_, t)| **t == Ty::Never) {
            return;
        }
        let params: Vec<Option<usize>> = operands.iter().map(|(e, _)| e.and_then(|e| self.tracked(e))).collect();
        let mut done: Vec<usize> = Vec::new();
        for p in params.iter().flatten() {
            if done.contains(p) {
                continue;
            }
            done.push(*p);
            let k = kinds_where(|s| {
                let tys: Vec<Ty> =
                    operands.iter().zip(&params).map(|((_, t), q)| if *q == Some(*p) { s.clone() } else { (*t).clone() }).collect();
                !fails(&tys)
            });
            self.require_param(*p, k);
        }
    }

    /// `l op r` with `+ - * / // % **`.
    pub(super) fn need_arith(&mut self, op: BinOp, l: Option<&Expr>, lt: &Ty, r: Option<&Expr>, rt: &Ty) {
        self.need_operands(&[(l, lt), (r, rt)], |t| {
            let (lk, rk) = (t[0].kinds(), t[1].kinds());
            let numbers = lk & NUMBER != 0 && rk & NUMBER != 0;
            let texts = op == BinOp::Add && lk & TEXT != 0 && rk & TEXT != 0;
            let lists = op == BinOp::Add && lk & LIST != 0 && rk & LIST != 0;
            !(numbers || texts || lists)
        });
    }

    /// `l < r`, `l >= r`...
    pub(super) fn need_comparable(&mut self, l: &Expr, lt: &Ty, r: &Expr, rt: &Ty) {
        self.need_operands(&[(Some(l), lt), (Some(r), rt)], |t| !comparable(&t[0], &t[1]));
    }

    /// `a[b]`.
    pub(super) fn need_index(&mut self, a: &Expr, ta: &Ty, b: &Expr, tb: &Ty) {
        self.need_operands(&[(Some(a), ta), (Some(b), tb)], |t| {
            t[0].never(LIST | TEXT | RECORD) || (t[0].only(LIST | TEXT) && t[1].never(INT)) || (t[0].only(RECORD) && t[1].never(TEXT))
        });
    }

    /// `x.name`: a record, or a value built with a constructor that has this field.
    pub(super) fn need_field(&mut self, obj: &Expr, name: &str) {
        if self.tracked(obj).is_none() {
            return;
        }
        let mut k = RECORD;
        for m in self.ctors.values() {
            if m.fields.iter().any(|f| &*f.name() == name) {
                k |= Ty::data(m.type_name, None).kinds();
            }
        }
        // `t.upcase`: without such a field, the function `upcase` is called with the value.
        let mut exits = false;
        if let Some((need, may_exit)) = self.function_need(Sym::new(name), &[], &[]) {
            k |= need;
            exits = may_exit;
        }
        self.require(obj, k);
        if exits {
            self.may_stop();
        }
    }

    /// The arguments of a call to the built-in function `b`.
    fn need_builtin(&mut self, b: &str, exprs: &[&Expr], tys: &[Ty]) {
        let operands: Vec<(Option<&Expr>, &Ty)> = exprs.iter().map(|e| Some(*e)).zip(tys).collect();
        self.need_operands(&operands, |t| {
            if b == "assert_eq" && t.len() == 2 && t[0].is_known() && t[1].is_known() && !may_be_equal(&t[0], &t[1]) {
                return true;
            }
            matches!(types::builtin_call(b, t), Some(Outcome::Wrong { .. }))
        });
    }

    /// Can calling this function value end the program without an error? False only when surely not.
    fn may_exit(&self, e: &Expr) -> bool {
        match &e.kind {
            ExprKind::Lambda(def) => self.needs.local.get(&def.span).is_none_or(|(_, exits)| *exits),
            ExprKind::Var(s) | ExprKind::Ctor(s) => {
                if let Some(l) = self.scopes.iter().rev().flat_map(|scope| scope.iter().rev()).find(|l| l.sym == *s) {
                    return match &l.def {
                        Some(def) => self.needs.local.get(&def.span).is_none_or(|(_, exits)| *exits),
                        None => true,
                    };
                }
                let g = match self.own.get(s) {
                    Some(g) => g,
                    None => match self.know.globals.get(s) {
                        Some(g) => g,
                        None => return true,
                    },
                };
                match g.kind {
                    GKind::Ctor | GKind::CtorValue => false,
                    GKind::Builtin => super::builtin_name(g).is_none_or(|b| b == "exit" || described(b).is_none()),
                    GKind::Func => !self.own.contains_key(s) || self.needs.exits.contains(s),
                    GKind::Value => true,
                }
            }
            _ => true,
        }
    }

    /// A call, once its arguments are checked: what it needs of the
    /// parameters given to it, and whether it may end the program. Returns
    /// true when it surely fails (it is then reported).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn need_call(
        &mut self,
        name: Option<Sym>,
        res: &Res,
        builtin: Option<&str>,
        callee: Option<usize>,
        exprs: &[&Expr],
        tys: &[Ty],
        span: Span,
    ) -> bool {
        if let Some(i) = callee {
            self.require_param(i, FUNC);
        }
        if let Some(b) = builtin {
            self.need_builtin(b, exprs, tys);
            let calls_back = (0..exprs.len()).any(|i| types::callback(b, i, &[]).is_some() && self.may_exit(exprs[i]));
            if b == "exit" || calls_back || (b != "pow" && described(b).is_none()) {
                self.may_stop();
            }
            return false;
        }
        let (def, needs, exits) = match (name, res) {
            (_, Res::Global(g)) if matches!(g.kind, GKind::Ctor | GKind::CtorValue) => return false,
            (Some(s), Res::Global(g)) if g.kind == GKind::Func && self.own.contains_key(&s) => {
                let def = self.fn_defs.get(&s).cloned();
                (def, self.needs.top.get(&s).cloned(), self.needs.exits.contains(&s))
            }
            (_, Res::Local(_, Some(def))) => match self.needs.local.get(&def.span).cloned() {
                Some((needs, exits)) => (Some(def.clone()), Some(needs), exits),
                None => (None, None, true),
            },
            _ => (None, None, true),
        };
        if let Some(needs) = &needs {
            for (e, k) in exprs.iter().zip(needs) {
                self.require(e, *k);
            }
        }
        if exits {
            self.may_stop();
        }
        let (Some(def), Some(needs)) = (def, needs) else { return false };
        let Some(i) = tys.iter().zip(&needs).position(|(t, k)| t.is_known() && t.never(*k)) else { return false };
        if self.needs.reporting || self.needs.specializing > 0 {
            let d = self.failing_call(&def, &needs, i, exprs, tys, span);
            self.diags.push(d);
        }
        true
    }

    /// The error of a call that always fails: the one the function stops
    /// with, found by checking it again with these arguments.
    fn failing_call(&mut self, def: &FnDef, needs: &[Kinds], i: usize, exprs: &[&Expr], tys: &[Ty], span: Span) -> Diagnostic {
        let name = def.name.map(|n| n.to_string()).unwrap_or_else(|| "fn".into());
        let (inner, ret) =
            if self.needs.specializing < 2 { (self.check_again(def, exprs, tys).1, Some(self.gives(def, needs))) } else { (None, None) };
        let sig = self.signature_of(def, needs, ret.as_ref());
        let mut d = match inner {
            Some(inner) => {
                let mut d = Diagnostic::error(inner.code, inner.message.clone())
                    .at(span)
                    .label(t!("`{name}` stops with this error, every time", "`{name}` s'arrête sur cette erreur, à chaque fois"));
                if let Some(at) = inner.span {
                    let label = if inner.label.is_empty() { t!("here", "ici") } else { inner.label.clone() };
                    d = d.secondary(at, t!("in `{name}`: {label}", "dans `{name}` : {label}"));
                }
                d.secondary.extend(inner.secondary.iter().cloned());
                d.notes = inner.notes.clone();
                d.helps = inner.helps.clone();
                d
            }
            None => Diagnostic::error(
                "E005",
                t!("`{name}` always fails with {}", "`{name}` échoue toujours avec {}", super::what(Some(exprs[i]), &tys[i])),
            )
            .at(span)
            .label(t!("fails every time", "échoue à chaque fois"))
            .secondary(def.span, t!("defined here", "définie ici")),
        };
        if sig.params[i].1.is_some() {
            let p = &sig.params[i].0;
            d.notes.insert(
                0,
                t!(
                    "from how `{name}` uses `{p}`, it needs: `{sig}`",
                    "d'après la façon dont `{name}` utilise `{p}`, elle a besoin de : `{sig}`"
                ),
            );
        }
        d
    }

    /// Checks a function again with arguments of these types (the values
    /// written in the code of a call are `exprs`), without reporting
    /// anything: what it gives, and its first error.
    fn check_again(&mut self, def: &FnDef, exprs: &[&Expr], tys: &[Ty]) -> (Ty, Option<Diagnostic>) {
        let top = def.name.is_some_and(|n| self.fn_defs.get(&n).is_some_and(|d| std::ptr::eq(&**d, def)));
        // A top-level function sees the top-level names, not the ones around the call.
        let scopes = if top { std::mem::take(&mut self.scopes) } else { Vec::new() };
        let block_defs = if top { std::mem::take(&mut self.block_defs) } else { Vec::new() };
        let reported = self.diags.len();
        let (may_fail, pure_call, last) = (self.may_fail, self.pure_call.take(), self.needs.last.clone());
        let mut args = Vec::new();
        for c in &def.clauses {
            for (p, e) in c.params.iter().zip(exprs) {
                if let (PatKind::Bind(s), Some(v)) = (&p.kind, literal(e)) {
                    args.push((*s, v));
                }
            }
        }
        ARGS.with(|a| a.borrow_mut().push(args));
        self.needs.specializing += 1;
        self.needs.quiet = true;
        let t = self.func(def, Some(tys));
        self.needs.specializing -= 1;
        ARGS.with(|a| a.borrow_mut().pop());
        let found = self.diags[reported..].iter().find(|d| d.is_error()).cloned();
        self.diags.truncate(reported);
        (self.may_fail, self.pure_call, self.needs.last) = (may_fail, pure_call, last);
        if top {
            self.scopes = scopes;
            self.block_defs = block_defs;
        }
        (t.ret(), found)
    }

    /// What a function gives when it does not fail: what it gives with
    /// parameters of the kinds they need.
    fn gives(&mut self, def: &FnDef, needs: &[Kinds]) -> Ty {
        let tys: Vec<Ty> = needs.iter().enumerate().map(|(i, k)| self.needed_type(def, i, *k)).collect();
        self.check_again(def, &[], &tys).0
    }

    /// Takes the "checked with the arguments of a call" mark, at the start
    /// of a function: its definitions that do not match them say nothing.
    pub(super) fn take_quiet(&mut self) -> bool {
        std::mem::take(&mut self.needs.quiet)
    }

    /// `value.name(args)` where the value may be a record (its field is
    /// called) or not (the function `name` is).
    pub(super) fn need_method(&mut self, recv: &Expr, name: Sym, args: &[&Expr], tys: &[Ty]) {
        let Some(p) = self.tracked(recv) else {
            self.may_stop();
            return;
        };
        // A record: its field is called, which may do anything.
        let saved = self.ways();
        self.require_param(p, RECORD);
        self.may_stop();
        self.set_ways(saved);
        let (need, exits) = self.function_need(name, args, tys).unwrap_or((ALL, true));
        self.require_param(p, RECORD | need);
        if exits {
            self.may_stop();
        }
    }

    /// What the function `name` needs as its first argument, before `args`,
    /// and whether it may end the program; `None` when no function has this
    /// name (a local value may be one: anything will do then).
    fn function_need(&self, name: Sym, args: &[&Expr], tys: &[Ty]) -> Option<(Kinds, bool)> {
        let local = self.scopes.iter().rev().flat_map(|scope| scope.iter().rev()).find(|l| l.sym == name).map(|l| l.def.clone());
        Some(match local {
            Some(Some(def)) => self.needs.local.get(&def.span).map(|(n, e)| (n.first().copied().unwrap_or(ALL), *e)).unwrap_or((ALL, true)),
            Some(None) => (ALL, true),
            None => match self.own.get(&name).or_else(|| self.know.globals.get(&name)) {
                Some(g) if g.kind == GKind::Builtin => match super::builtin_name(g) {
                    Some(b) => {
                        let first = kinds_where(|s| {
                            let all: Vec<Ty> = std::iter::once(s.clone()).chain(tys.iter().cloned()).collect();
                            !matches!(types::builtin_call(b, &all), Some(Outcome::Wrong { .. }))
                        });
                        let calls_back = (0..args.len()).any(|i| types::callback(b, i + 1, &[]).is_some() && self.may_exit(args[i]));
                        (first, b == "exit" || calls_back || described(b).is_none())
                    }
                    None => (ALL, true),
                },
                Some(g) if g.kind == GKind::Func && self.own.contains_key(&name) => {
                    (self.needs.top.get(&name).and_then(|n| n.first().copied()).unwrap_or(ALL), self.needs.exits.contains(&name))
                }
                Some(g) if matches!(g.kind, GKind::Ctor | GKind::CtorValue) => (ALL, false),
                Some(_) => (ALL, true),
                None => return None,
            },
        })
    }

    // ----- inference over the module ---------------------------------------

    /// What the top-level functions need: first which ones may end the
    /// program, then what their parameters must be, narrowed from "anything"
    /// until it is stable. Each round only uses what earlier rounds found
    /// for sure, so each one is right, and the last one too.
    pub(super) fn infer_needs(&mut self, order: &[std::rc::Rc<FnDef>]) {
        if order.is_empty() {
            return;
        }
        let reported = self.diags.len();
        let mut stable = false;
        for _ in 0..super::MAX_ROUNDS {
            let before = self.needs.exits.len();
            for def in order {
                self.top_fn(def);
            }
            if self.needs.exits.len() == before {
                stable = true;
                break;
            }
        }
        if !stable {
            self.needs.exits.extend(self.fn_defs.keys().copied());
        }
        self.needs.top = self.fn_defs.iter().map(|(s, d)| (*s, vec![ALL; d.arity])).collect();
        self.needs.narrowing = true;
        for _ in 0..super::MAX_ROUNDS {
            let before = self.needs.top.clone();
            for def in order {
                self.top_fn(def);
            }
            if self.needs.top == before {
                break;
            }
        }
        self.needs.narrowing = false;
        self.diags.truncate(reported);
        self.defined.clear();
    }

    /// Narrows what a top-level function needs with what its code just showed.
    pub(super) fn narrow_top(&mut self, name: Sym) {
        let (needs, exits) = self.needs.last.clone();
        if exits {
            self.needs.exits.insert(name);
        }
        if self.needs.narrowing {
            if let Some(old) = self.needs.top.get_mut(&name) {
                old.iter_mut().zip(needs).for_each(|(o, n)| *o &= n);
            }
        }
    }

    // ----- signatures --------------------------------------------------------

    /// The type of the constructors a parameter is matched with, when every
    /// definition of the function matches it with constructors of one type.
    fn data_type_of(&self, def: &FnDef, i: usize) -> Option<Sym> {
        fn types_of(c: &Checker, p: &Pattern, out: &mut Vec<Sym>) -> bool {
            match &p.kind {
                PatKind::Ctor(name, _) => match c.ctors.get(name) {
                    Some(m) => {
                        out.push(m.type_name);
                        true
                    }
                    None => false,
                },
                PatKind::Or(alts) => alts.iter().all(|a| types_of(c, a, out)),
                _ => false,
            }
        }
        let mut names = Vec::new();
        if !def.clauses.iter().all(|c| types_of(self, &c.params[i], &mut names)) {
            return None;
        }
        let first = *names.first()?;
        match names.iter().all(|n| *n == first) {
            true if !matches!(&*first.name(), "Option" | "Result") && !self.types_defined_again.contains(&first) => Some(first),
            _ => None,
        }
    }

    /// The signature of a function that needs this; `ret`: what it gives, if known.
    fn signature_of(&self, def: &FnDef, needs: &[Kinds], ret: Option<&Ty>) -> Signature {
        let params =
            param_names(def).into_iter().zip(needs).enumerate().map(|(i, (n, k))| (n, kinds_text(*k, self.data_type_of(def, i)))).collect();
        Signature { name: def.name.map(|n| n.to_string()).unwrap_or_else(|| "fn".into()), params, ret: ret.and_then(type_text) }
    }

    /// The type of the values of the kinds a parameter needs.
    fn needed_type(&self, def: &FnDef, i: usize, k: Kinds) -> Ty {
        match k {
            NUMBER => Ty::Num,
            DATA => self.data_type_of(def, i).map_or(Ty::Any, |t| Ty::data(t, None)),
            k if EACH.contains(&k) => sample(k),
            _ => Ty::Any,
        }
    }

    /// Notes the signature of a local function, once checked.
    pub(super) fn note_local_signature(&mut self, def: &FnDef, t: &Ty) {
        if self.needs.specializing > 0 || def.name.is_none() {
            return;
        }
        let needs = self.needs.last.0.clone();
        let sig = self.signature_of(def, &needs, Some(&t.ret()));
        if let Some(sigs) = &mut self.needs.signatures {
            sigs.insert(def.span, sig);
        }
    }

    /// The signatures of the top-level functions: what a call gives is
    /// inferred again with the parameters of the kinds they need (a call
    /// with other ones fails).
    pub(super) fn top_signatures(&mut self) {
        let Some(mut sigs) = self.needs.signatures.take() else { return };
        let defs: Vec<(Sym, std::rc::Rc<FnDef>)> = self.fn_defs.iter().map(|(s, d)| (*s, d.clone())).collect();
        for (s, def) in defs {
            let needs = self.needs.top.get(&s).cloned().unwrap_or_else(|| vec![ALL; def.arity]);
            let ret = self.gives(&def, &needs);
            sigs.insert(def.span, self.signature_of(&def, &needs, Some(&ret)));
        }
        self.needs.signatures = Some(sigs);
    }
}

/// Is the built-in function described to the checker (see
/// [`types::builtin_call`])? The others may do anything.
fn described(b: &str) -> Option<()> {
    let n = crate::builtins::lookup(b)?.min;
    types::builtin_call(b, &vec![Ty::Any; n]).map(|_| ())
}
