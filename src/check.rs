//! Checks a program before running it, to report as many mistakes as possible
//! at once: unknown names, wrong numbers of arguments, values defined twice,
//! forgotten cases in a `match`, unused variables...
//!
//! It also infers the types of expressions when they are certain (see
//! [`crate::types`]), to report operations that would fail every time they
//! run: `"age: " + 36`, `len(5)`, a field that a record does not have...
//! When the checker is not sure, it says nothing.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::ast::*;
use crate::diagnostic::Diagnostic;
use crate::interp::{arity_error, Interp};
use crate::parser::immutable_help;
use crate::source::Span;
use crate::suggest;
use crate::symbol::Sym;
use crate::types::{self, Outcome, Ty, BOOL, DATA, FUNC, INT, LIST, NUMBER, OPTION, RECORD, RESULT, TEXT, UNIT};
use crate::value::Value;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GKind {
    Builtin,
    Func,
    Ctor,
    /// A constructor without fields, like `None`: a value, not a function.
    CtorValue,
    Value,
}

#[derive(Clone, Debug)]
pub struct GlobalInfo {
    pub kind: GKind,
    pub arity: Option<(usize, usize)>,
    pub sig: String,
    pub span: Option<Span>,
}

#[derive(Clone, Debug)]
pub struct CtorMeta {
    pub type_name: Sym,
    pub arity: usize,
    /// The names of its fields (`Circle(radius)`: `radius`).
    pub fields: Vec<Sym>,
}

/// What is known before checking a module: built-ins, and what earlier code defined.
#[derive(Clone, Default)]
pub struct Knowledge {
    pub globals: HashMap<Sym, GlobalInfo>,
    pub ctors: HashMap<Sym, CtorMeta>,
    pub types: HashMap<Sym, Vec<Sym>>,
    /// The types of the values defined by the `use`d files, when certain.
    pub value_types: HashMap<Sym, Ty>,
}

fn func_info(def: &FnDef) -> GlobalInfo {
    GlobalInfo { kind: GKind::Func, arity: Some((def.arity, def.arity)), sig: def.signature(), span: Some(def.span) }
}

fn ctor_sig(name: Sym, fields: &[Sym]) -> String {
    let f: Vec<String> = fields.iter().map(|f| f.name().to_string()).collect();
    format!("{name}({})", f.join(", "))
}

impl Knowledge {
    pub fn from_interp(it: &Interp) -> Knowledge {
        let mut k = Knowledge::default();
        for (s, v) in &it.globals {
            let info = match v {
                Value::Builtin(b) => GlobalInfo { kind: GKind::Builtin, arity: Some((b.min, b.max)), sig: b.sig.to_string(), span: None },
                Value::Func(c) => func_info(&c.def),
                Value::Ctor(c) => GlobalInfo {
                    kind: GKind::Ctor,
                    arity: Some((c.fields.len(), c.fields.len())),
                    sig: ctor_sig(c.name, &c.fields),
                    span: None,
                },
                Value::Variant(var) if var.fields.is_empty() && var.ctor.name == *s => {
                    GlobalInfo { kind: GKind::CtorValue, arity: None, sig: s.name().to_string(), span: None }
                }
                _ if &*s.name() == "pi" => GlobalInfo { kind: GKind::Builtin, arity: None, sig: "pi".into(), span: None },
                _ => GlobalInfo { kind: GKind::Value, arity: None, sig: String::new(), span: None },
            };
            k.globals.insert(*s, info);
        }
        for (t, ctors) in &it.types {
            k.types.insert(*t, ctors.iter().map(|c| c.name).collect());
            for c in ctors {
                k.ctors.insert(c.name, CtorMeta { type_name: *t, arity: c.fields.len(), fields: c.fields.clone() });
            }
        }
        k
    }

    /// Adds the definitions of a module, without running it.
    pub fn add_items(&mut self, items: &[Item]) {
        // A used file runs completely before the file that uses it: the
        // types of its values are known from its own code.
        let inferred = top_level_types(items, self);
        for item in items {
            match item {
                Item::Stmt(Stmt::Fn(def)) => {
                    if let Some(n) = def.name {
                        self.value_types.remove(&n);
                        self.globals.insert(n, func_info(def));
                    }
                }
                Item::Stmt(Stmt::Let(p, _)) => {
                    let mut b = Vec::new();
                    p.bindings(&mut b);
                    for (s, sp) in b {
                        // Defined by two files: the value depends on the order they run in.
                        let again = self.globals.contains_key(&s);
                        match inferred.get(&s) {
                            Some(t) if !again => self.value_types.insert(s, t.without_results()),
                            _ => self.value_types.remove(&s),
                        };
                        self.globals.insert(s, GlobalInfo { kind: GKind::Value, arity: None, sig: String::new(), span: Some(sp) });
                    }
                }
                Item::Type(t) => {
                    self.types.insert(t.name, t.variants.iter().map(|v| v.name).collect());
                    for v in &t.variants {
                        self.value_types.remove(&v.name);
                        self.ctors.insert(v.name, CtorMeta { type_name: t.name, arity: v.fields.len(), fields: v.fields.clone() });
                        self.globals.insert(v.name, ctor_info(v));
                    }
                }
                _ => {}
            }
        }
    }
}

fn ctor_info(v: &VariantDef) -> GlobalInfo {
    if v.fields.is_empty() {
        GlobalInfo { kind: GKind::CtorValue, arity: None, sig: v.name.name().to_string(), span: Some(v.span) }
    } else {
        GlobalInfo {
            kind: GKind::Ctor,
            arity: Some((v.fields.len(), v.fields.len())),
            sig: ctor_sig(v.name, &v.fields),
            span: Some(v.span),
        }
    }
}

/// Checks a module. Returns errors and warnings (errors prevent running).
pub fn check(items: &[Item], know: &Knowledge, repl: bool) -> Vec<Diagnostic> {
    let mut c = Checker::new(items, know, repl);
    c.infer(items);
    c.walk(items);
    c.diags
}

/// The types inferred for the top-level names of a module, without checking
/// it. The tests use it to verify that every value a program computes has
/// the type inferred for it.
#[doc(hidden)]
pub fn top_level_types(items: &[Item], know: &Knowledge) -> HashMap<Sym, Ty> {
    let mut c = Checker::new(items, know, false);
    c.infer(items);
    c.gtypes
}

/// How many times the module may be walked to infer the types of its
/// top-level definitions, before what still changes is given up.
const MAX_ROUNDS: usize = 12;

struct Local {
    sym: Sym,
    span: Span,
    used: bool,
    warn: bool,
    /// For a local function: its number of parameters and signature.
    info: Option<GlobalInfo>,
    ty: Ty,
    /// For a local function: its definition.
    def: Option<Rc<FnDef>>,
}

enum Res {
    Local(Option<GlobalInfo>, Option<Rc<FnDef>>),
    Global(GlobalInfo),
    Unknown,
}

struct Checker<'a> {
    know: &'a Knowledge,
    own: HashMap<Sym, GlobalInfo>,
    ctors: HashMap<Sym, CtorMeta>,
    types: HashMap<Sym, Vec<Sym>>,
    /// Top-level values already defined at this point of the file.
    defined: HashSet<Sym>,
    scopes: Vec<Vec<Local>>,
    fn_depth: usize,
    diags: Vec<Diagnostic>,
    repl: bool,
    /// Names bound by a record pattern under the field's own name (`{age}`).
    field_binds: HashSet<Span>,
    /// Field names written in the records of this module (for `record.f()`).
    record_fields: HashSet<Rc<str>>,
    /// Types of the top-level values and functions of this module.
    gtypes: HashMap<Sym, Ty>,
    /// The definitions of the top-level functions of this module.
    fn_defs: HashMap<Sym, Rc<FnDef>>,
    /// For each function being checked, what its `?` may return early.
    early: Vec<Ty>,
    /// Did the code just checked use something that may check a value
    /// (`assert`, `?`, a function of the program...)? For tests.
    may_check: bool,
    /// How many `fold` functions are being checked inside each other.
    folds: usize,
}

impl<'a> Checker<'a> {
    fn new(items: &[Item], know: &'a Knowledge, repl: bool) -> Checker<'a> {
        let mut c = Checker {
            know,
            own: HashMap::new(),
            ctors: know.ctors.clone(),
            types: know.types.clone(),
            defined: HashSet::new(),
            scopes: Vec::new(),
            fn_depth: 0,
            diags: Vec::new(),
            repl,
            field_binds: HashSet::new(),
            record_fields: HashSet::new(),
            gtypes: HashMap::new(),
            fn_defs: HashMap::new(),
            early: Vec::new(),
            may_check: false,
            folds: 0,
        };
        for item in items {
            match item {
                Item::Stmt(s) => fields_in_stmt(s, &mut c.record_fields),
                Item::Test(_, body, _) => fields_in_expr(body, &mut c.record_fields),
                _ => {}
            }
        }
        c.collect(items);
        c
    }

    // ----- top level ---------------------------------------------------

    fn collect(&mut self, items: &[Item]) {
        let mut first: HashMap<Sym, Span> = HashMap::new();
        let mut type_first: HashMap<Sym, Span> = HashMap::new();
        for item in items {
            match item {
                Item::Stmt(Stmt::Fn(def)) => {
                    if let Some(n) = def.name {
                        if self.declare_global(n, def.span, func_info(def), &mut first) {
                            self.gtypes.insert(n, Ty::func(Some((def.arity, def.arity)), Ty::Never, Some(def.signature().into())));
                            self.fn_defs.insert(n, def.clone());
                        }
                    }
                }
                Item::Stmt(Stmt::Let(p, _)) => {
                    let mut b = Vec::new();
                    p.bindings(&mut b);
                    for (s, sp) in b {
                        let info = GlobalInfo { kind: GKind::Value, arity: None, sig: String::new(), span: Some(sp) };
                        if self.declare_global(s, sp, info, &mut first) {
                            self.gtypes.insert(s, Ty::Never);
                        }
                    }
                }
                Item::Type(t) => {
                    if let Some(prev) = type_first.get(&t.name) {
                        if !self.repl {
                            self.diags.push(
                                Diagnostic::error("E011", t!("the type `{}` is already defined", "le type `{}` est déjà défini", t.name))
                                    .at(t.span)
                                    .secondary(*prev, t!("first defined here", "défini une première fois ici"))
                                    .help(t!("give it another name", "donnez-lui un autre nom")),
                            );
                        }
                        continue;
                    }
                    type_first.insert(t.name, t.span);
                    for v in &t.variants {
                        let mut seen: HashSet<Sym> = HashSet::new();
                        for f in &v.fields {
                            if !seen.insert(*f) {
                                self.diags.push(
                                    Diagnostic::error(
                                        "E011",
                                        t!("the field `{f}` appears twice in `{}`", "le champ `{f}` apparaît deux fois dans `{}`", v.name),
                                    )
                                    .at(v.span),
                                );
                            }
                        }
                        self.declare_global(v.name, v.span, ctor_info(v), &mut first);
                        self.ctors.insert(v.name, CtorMeta { type_name: t.name, arity: v.fields.len(), fields: v.fields.clone() });
                    }
                    self.types.insert(t.name, t.variants.iter().map(|v| v.name).collect());
                }
                _ => {}
            }
        }
    }

    /// Returns false if the name was already defined.
    fn declare_global(&mut self, s: Sym, span: Span, info: GlobalInfo, first: &mut HashMap<Sym, Span>) -> bool {
        if !self.repl {
            if let Some(prev) = first.get(&s) {
                let mut d = Diagnostic::error("E011", t!("`{s}` is already defined", "`{s}` est déjà défini"))
                    .at(span)
                    .label(t!("defined again here", "défini à nouveau ici"))
                    .secondary(*prev, t!("first defined here", "défini une première fois ici"));
                d = if matches!(info.kind, GKind::Ctor | GKind::CtorValue) {
                    d.help(t!(
                        "a constructor belongs to a single type: rename one of them",
                        "un constructeur appartient à un seul type : renommez l'un des deux"
                    ))
                } else if info.kind == GKind::Func {
                    d.help(t!(
                        "to give several definitions to a function (one per case), write them one right after the other",
                        "pour donner plusieurs définitions à une fonction (une par cas), écrivez-les les unes juste après les autres"
                    ))
                } else {
                    d.help(immutable_help())
                };
                self.diags.push(d);
                return false;
            }
            if let Some(g) = self.know.globals.get(&s) {
                match g.kind {
                    GKind::Builtin => self.diags.push(
                        Diagnostic::warning("W003", t!("`{s}` hides the built-in function `{s}`", "`{s}` masque la fonction intégrée `{s}`"))
                            .at(span)
                            .help(t!(
                                "in this file, `{s}` will now mean your definition; choose another name to keep the built-in one",
                                "dans ce fichier, `{s}` désignera désormais votre définition ; choisissez un autre nom pour garder celle intégrée"
                            )),
                    ),
                    _ => {
                        let mut d = Diagnostic::error("E011", t!("`{s}` is already defined", "`{s}` est déjà défini")).at(span);
                        d = match g.span {
                            Some(prev) => d.secondary(prev, t!("first defined here", "défini une première fois ici")),
                            None => d.note(t!("`{s}` is part of Rythme", "`{s}` fait partie de Rythme")),
                        };
                        self.diags.push(d.help(t!("choose another name", "choisissez un autre nom")));
                        return false;
                    }
                }
            }
        }
        first.insert(s, span);
        self.own.insert(s, info);
        true
    }

    /// Infers the types of the top-level values and functions. They may use
    /// each other in any order, and functions may call themselves: starting
    /// from "no value" for all of them, the module is walked again and again
    /// until their types stop changing. Only the last walk reports problems.
    fn infer(&mut self, items: &[Item]) {
        if self.gtypes.is_empty() {
            return;
        }
        let order = callees_first(items);
        let reported = self.diags.len();
        for attempt in 0..2 {
            let mut changed: Vec<Sym> = Vec::new();
            for _ in 0..MAX_ROUNDS {
                let before = self.gtypes.clone();
                for def in &order {
                    self.top_fn(def);
                }
                for item in items {
                    if let Item::Stmt(Stmt::Let(p, e)) = item {
                        self.top_let(p, e);
                    }
                }
                self.diags.truncate(reported);
                self.defined.clear();
                changed = self.gtypes.iter().filter(|(s, t)| before.get(s) != Some(t)).map(|(s, _)| *s).collect();
                if changed.is_empty() {
                    return;
                }
            }
            // Rare: what still changes is given up (it may be anything), and
            // the others are inferred again from there.
            let give_up: Vec<Sym> = if attempt == 0 { changed } else { self.gtypes.keys().copied().collect() };
            for s in give_up {
                if let Some(t) = self.gtypes.get_mut(&s) {
                    *t = match t {
                        Ty::Func(f) => Ty::func(f.arity, Ty::Any, f.sig.clone()),
                        _ => Ty::Any,
                    };
                }
            }
        }
    }

    /// Records a type that a top-level name may have.
    fn set_global(&mut self, s: Sym, t: Ty) {
        if let Some(old) = self.gtypes.get_mut(&s) {
            *old = old.join(&t).limit();
        }
    }

    fn walk(&mut self, items: &[Item]) {
        // The value of the last expression is shown by the REPL and `rythme -e`;
        // the values of the other top-level expressions are lost.
        let last_expr = items.iter().rposition(|it| matches!(it, Item::Stmt(Stmt::Expr(_))));
        for (i, item) in items.iter().enumerate() {
            if let Item::Stmt(Stmt::Expr(e)) = item {
                if Some(i) != last_expr && !has_effect(e) {
                    self.diags.push(unused_value(e));
                }
            }
            match item {
                Item::Stmt(Stmt::Fn(def)) => self.top_fn(def),
                Item::Stmt(Stmt::Let(p, e)) => self.top_let(p, e),
                Item::Stmt(Stmt::Expr(e)) => {
                    self.expr(e);
                }
                Item::Test(_, body, span) => {
                    self.fn_depth += 1;
                    self.early.push(Ty::Never);
                    self.scopes.push(Vec::new());
                    self.may_check = false;
                    let t = match &body.kind {
                        ExprKind::Block(stmts) => self.stmts(stmts),
                        _ => self.expr(body),
                    };
                    self.pop_scope();
                    self.early.pop();
                    self.fn_depth -= 1;
                    self.test_checks_something(body, &t, *span);
                }
                Item::Type(_) | Item::Use(..) => {}
            }
        }
    }

    fn top_fn(&mut self, def: &FnDef) {
        let t = self.func(def, None);
        if let Some(name) = def.name {
            if self.own.get(&name).is_some_and(|g| g.kind == GKind::Func) {
                self.set_global(name, t);
            }
        }
    }

    fn top_let(&mut self, p: &Pattern, e: &Expr) {
        let t = self.expr(e);
        self.pattern(p);
        self.let_pattern(p, e, &t);
        let mut b = Vec::new();
        p.bindings(&mut b);
        let mut tys = Vec::new();
        self.binding_types(p, &t, &mut tys);
        self.check_duplicates(&b);
        for ((s, _), t) in b.into_iter().zip(tys) {
            self.defined.insert(s);
            if self.own.get(&s).is_some_and(|g| g.kind == GKind::Value) {
                self.set_global(s, t);
            }
        }
    }

    /// A test fails when it stops with an error, or ends with `false`,
    /// `None` or `Err(...)`. One that can do none of these checks nothing.
    fn test_checks_something(&mut self, body: &Expr, t: &Ty, span: Span) {
        if self.may_check || !t.is_known() || !t.never(BOOL | OPTION | RESULT) {
            return;
        }
        let at = match &body.kind {
            ExprKind::Block(stmts) => match stmts.last() {
                Some(Stmt::Expr(e)) => e.span,
                _ => span,
            },
            _ => body.span,
        };
        self.diags.push(
            Diagnostic::warning("W007", t!("this test checks nothing", "ce test ne vérifie rien"))
                .at(at)
                .label(t!("its result, {}, is never checked", "son résultat, {}, n'est jamais vérifié", what(None, t)))
                .note(t!(
                    "a test fails when it stops with an error, or ends with `false`, `None` or `Err(...)`",
                    "un test échoue s'il s'arrête sur une erreur, ou s'il finit par `false`, `None` ou `Err(...)`"
                ))
                .help(t!(
                    "compare the result with what you expect: `assert_eq(actual, expected)`",
                    "comparez le résultat avec ce que vous attendez : `assert_eq(obtenu, attendu)`"
                )),
        );
    }

    // ----- scopes ------------------------------------------------------

    fn declare_local(&mut self, s: Sym, span: Span, warn: bool, ty: Ty) {
        self.declare_local_fn(s, span, warn, None, ty, None);
    }

    fn declare_local_fn(&mut self, s: Sym, span: Span, warn: bool, info: Option<GlobalInfo>, ty: Ty, def: Option<Rc<FnDef>>) {
        let scope = self.scopes.last_mut().expect("scope");
        if let Some(prev) = scope.iter().find(|l| l.sym == s).map(|l| l.span) {
            self.diags.push(
                Diagnostic::error("E011", t!("`{s}` is already defined", "`{s}` est déjà défini"))
                    .at(span)
                    .label(t!("defined again here", "défini à nouveau ici"))
                    .secondary(prev, t!("first defined here", "défini une première fois ici"))
                    .help(immutable_help()),
            );
            return;
        }
        scope.push(Local { sym: s, span, used: false, warn, info, ty, def });
    }

    /// Gives its type to a local function, once its body is checked.
    fn set_local_type(&mut self, s: Sym, span: Span, ty: Ty) {
        if let Some(l) = self.scopes.last_mut().and_then(|scope| scope.iter_mut().rev().find(|l| l.sym == s && l.span == span)) {
            l.ty = ty;
        }
    }

    fn pop_scope(&mut self) {
        let Some(scope) = self.scopes.pop() else { return };
        for l in scope {
            let name = l.sym.name();
            if l.warn && !l.used && !name.starts_with('_') {
                // In `{name, age} = person`, renaming `age` would read another field.
                let help = if self.field_binds.contains(&l.span) {
                    t!(
                        "remove `{name}` from the pattern, or write `{name}: _{name}` if this is on purpose",
                        "retirez `{name}` du motif, ou écrivez `{name}: _{name}` si c'est voulu"
                    )
                } else {
                    t!("remove it, or name it `_{name}` if this is on purpose", "supprimez-le, ou nommez-le `_{name}` si c'est voulu")
                };
                self.diags
                    .push(Diagnostic::warning("W001", t!("`{name}` is never used", "`{name}` n'est jamais utilisé")).at(l.span).help(help));
            }
        }
    }

    fn check_duplicates(&mut self, b: &[(Sym, Span)]) -> Vec<bool> {
        let mut ok = Vec::with_capacity(b.len());
        for (i, (s, sp)) in b.iter().enumerate() {
            if let Some((_, prev)) = b[..i].iter().find(|(t, _)| t == s) {
                self.diags.push(
                    Diagnostic::error("E011", t!("`{s}` appears twice in this pattern", "`{s}` apparaît deux fois dans ce motif"))
                        .at(*sp)
                        .secondary(*prev, t!("first here", "une première fois ici"))
                        .help(t!(
                            "use two different names, then compare them with `==` in a condition: `[a, b] if a == b -> ...`",
                            "utilisez deux noms différents, puis comparez-les avec `==` dans une condition : `[a, b] if a == b -> ...`"
                        )),
                );
                ok.push(false);
            } else {
                ok.push(true);
            }
        }
        ok
    }

    /// Declares the names of a pattern that receives a value of type `ty`.
    fn bind_pattern(&mut self, p: &Pattern, warn: bool, ty: &Ty) {
        self.pattern(p);
        let mut b = Vec::new();
        p.bindings(&mut b);
        let mut tys = Vec::new();
        self.binding_types(p, ty, &mut tys);
        let ok = self.check_duplicates(&b);
        for (((s, sp), ok), t) in b.into_iter().zip(ok).zip(tys) {
            if ok {
                self.declare_local(s, sp, warn, t);
            }
        }
    }

    fn resolve(&mut self, s: Sym, span: Span) -> (Res, Ty) {
        for scope in self.scopes.iter_mut().rev() {
            for l in scope.iter_mut().rev() {
                if l.sym == s {
                    l.used = true;
                    let r = (Res::Local(l.info.clone(), l.def.clone()), l.ty.clone());
                    self.note_use(&r);
                    return r;
                }
            }
        }
        if let Some(g) = self.own.get(&s) {
            if g.kind == GKind::Value && self.fn_depth == 0 && !self.defined.contains(&s) && !self.repl {
                let mut d = Diagnostic::error("E003", t!("`{s}` is used before being defined", "`{s}` est utilisé avant d'être défini"))
                    .at(span)
                    .label(t!("used here", "utilisé ici"));
                if let Some(def) = g.span {
                    d = d.secondary(def, t!("defined later, here", "défini plus loin, ici"));
                }
                self.diags.push(d.help(t!("move the definition above this line", "déplacez la définition au-dessus de cette ligne")));
                return (Res::Unknown, Ty::Never);
            }
            let g = g.clone();
            let ty = self.own_type(s, &g);
            let r = (Res::Global(g), ty);
            self.note_use(&r);
            return r;
        }
        if let Some(g) = self.know.globals.get(&s) {
            let g = g.clone();
            let ty = self.known_type(s, &g);
            let r = (Res::Global(g), ty);
            self.note_use(&r);
            return r;
        }
        self.unknown(s, span);
        (Res::Unknown, Ty::Never)
    }

    /// Notes that the code uses something that may check a value: anything
    /// but plain data, constructors and built-in functions other than
    /// `assert`, `assert_eq` and `fail`.
    fn note_use(&mut self, (res, ty): &(Res, Ty)) {
        let plain = match res {
            Res::Global(g) if g.kind == GKind::Builtin => !matches!(builtin_name(g), Some("assert" | "assert_eq" | "fail")),
            Res::Global(g) if matches!(g.kind, GKind::Ctor | GKind::CtorValue) => true,
            _ => matches!(ty, Ty::Unit | Ty::Bool | Ty::Int | Ty::Float | Ty::Num | Ty::Text),
        };
        if !plain {
            self.may_check = true;
        }
    }

    /// The type of a top-level name of this module.
    fn own_type(&self, s: Sym, g: &GlobalInfo) -> Ty {
        match g.kind {
            GKind::Ctor => self.ctor_fn_type(s),
            GKind::CtorValue => self.ctor_value_type(s),
            // In the REPL, a value defined again may still be the old one
            // where it is used before its new definition.
            GKind::Value if self.repl && self.know.globals.contains_key(&s) => Ty::Any,
            _ => self.gtypes.get(&s).cloned().unwrap_or(Ty::Any),
        }
    }

    /// The type of a name defined before this module (built-in, `use`d file, REPL).
    fn known_type(&self, s: Sym, g: &GlobalInfo) -> Ty {
        match g.kind {
            GKind::Builtin if g.arity.is_none() && g.sig == "pi" => Ty::Float,
            GKind::Builtin => match builtin_name(g).and_then(crate::builtins::lookup) {
                Some(b) => types::builtin_value(b),
                None => Ty::Any,
            },
            GKind::Func => Ty::func(g.arity, Ty::Any, Some(g.sig.as_str().into())),
            GKind::Ctor => self.ctor_fn_type(s),
            GKind::CtorValue => self.ctor_value_type(s),
            GKind::Value => self.know.value_types.get(&s).cloned().unwrap_or(Ty::Any),
        }
    }

    fn ctor_fn_type(&self, c: Sym) -> Ty {
        match self.ctors.get(&c) {
            Some(m) => Ty::func(
                Some((m.arity, m.arity)),
                Ty::data(m.type_name, Some(vec![(c, vec![Ty::Any; m.arity])])),
                Some(ctor_sig(c, &m.fields).into()),
            ),
            None => Ty::Any,
        }
    }

    fn ctor_value_type(&self, c: Sym) -> Ty {
        match self.ctors.get(&c) {
            Some(m) => Ty::data(m.type_name, Some(vec![(c, Vec::new())])),
            None => Ty::Any,
        }
    }

    fn visible_names(&self) -> Vec<Rc<str>> {
        let mut names: Vec<Rc<str>> = Vec::new();
        for scope in &self.scopes {
            names.extend(scope.iter().map(|l| l.sym.name()));
        }
        names.extend(self.own.keys().map(|s| s.name()));
        names.extend(self.know.globals.keys().map(|s| s.name()));
        names
    }

    fn unknown(&mut self, s: Sym, span: Span) {
        let name = s.name();
        let mut d =
            Diagnostic::error("E003", t!("unknown name `{name}`", "nom inconnu `{name}`")).at(span).label(t!("not defined", "non défini"));
        if let Some(h) = suggest::alias_hint(&name) {
            d = d.help(h);
        } else {
            let names = self.visible_names();
            let wanted_ctor = s.is_ctor();
            let candidates = names.iter().filter(|n| n.chars().next().is_some_and(|c| c.is_uppercase()) == wanted_ctor);
            if let Some(best) = suggest::closest(&name, candidates.map(|n| &**n)) {
                d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
            } else if wanted_ctor {
                d = d.help(t!(
                    "to create your own kind of value, define a type: `type Name = {name}(field) | Other`",
                    "pour créer votre propre sorte de valeur, définissez un type : `type Nom = {name}(champ) | Autre`"
                ));
            } else {
                d = d.help(t!(
                    "define it before using it: `{name} = ...` or `fn {name}(...) = ...`",
                    "définissez-le avant de l'utiliser : `{name} = ...` ou `fn {name}(...) = ...`"
                ));
            }
        }
        self.diags.push(d);
    }

    // ----- functions and statements -----------------------------------

    /// Checks a function and gives its type. `params`: the types of the
    /// values it receives, when they are known for sure (a function written
    /// in place for `map`, or called right where it is written).
    fn func(&mut self, def: &FnDef, params: Option<&[Ty]>) -> Ty {
        self.fn_depth += 1;
        self.early.push(Ty::Never);
        let mut ret = Ty::Never;
        let mut catch_all: Option<Span> = None;
        let mut reported_unreachable = false;
        let name = def.name.map(|n| n.to_string()).unwrap_or_default();
        let param_ty = |i: usize| params.and_then(|p| p.get(i).cloned()).unwrap_or(Ty::Any);
        for (ci, clause) in def.clauses.iter().enumerate() {
            if let (Some(prev), false) = (catch_all, reported_unreachable) {
                reported_unreachable = true;
                self.diags.push(
                    Diagnostic::warning(
                        "W004",
                        t!("this definition of `{name}` can never be used", "cette définition de `{name}` ne peut jamais être utilisée"),
                    )
                    .at(clause.span)
                    .secondary(
                        prev,
                        t!("this definition already accepts every argument", "cette définition accepte déjà tous les arguments"),
                    )
                    .help(t!(
                        "definitions are tried in order: put the most general one last",
                        "les définitions sont essayées dans l'ordre : mettez la plus générale en dernier"
                    )),
                );
            } else if catch_all.is_none() {
                // An earlier definition already accepts all these arguments.
                let earlier = def.clauses[..ci]
                    .iter()
                    .find(|c| c.guard.is_none() && c.params.iter().zip(&clause.params).all(|(q, p)| self.covers(q, p)));
                if let Some(prev) = earlier {
                    self.diags.push(
                        Diagnostic::warning(
                            "W004",
                            t!(
                                "this definition of `{name}` can never be used",
                                "cette définition de `{name}` ne peut jamais être utilisée"
                            ),
                        )
                        .at(clause.span)
                        .secondary(
                            prev.span,
                            t!("this definition already accepts these arguments", "cette définition accepte déjà ces arguments"),
                        )
                        .help(t!(
                            "definitions are tried in order: remove this one, or change its patterns",
                            "les définitions sont essayées dans l'ordre : supprimez celle-ci, ou changez ses motifs"
                        )),
                    );
                }
            }
            if clause.guard.is_none()
                && clause.params.iter().enumerate().all(|(i, p)| self.catches_all(p, &param_ty(i)))
                && catch_all.is_none()
            {
                catch_all = Some(clause.span);
            }
            self.scopes.push(Vec::new());
            let mut b = Vec::new();
            let mut tys = Vec::new();
            let mut matches = true;
            for (i, p) in clause.params.iter().enumerate() {
                let t = param_ty(i);
                self.pattern(p);
                p.bindings(&mut b);
                self.binding_types(p, &t, &mut tys);
                if !self.can_match(p, &t, true) {
                    matches = false;
                    self.diags.push(
                        Diagnostic::error(
                            "E006",
                            t!(
                                "this function receives {}, which never matches this pattern",
                                "cette fonction reçoit {}, qui ne correspond jamais à ce motif",
                                what(None, &t)
                            ),
                        )
                        .at(p.span)
                        .label(t!("never matches", "ne correspond jamais"))
                        .help(pattern_help()),
                    );
                }
            }
            let ok = self.check_duplicates(&b);
            for (((s, sp), ok), t) in b.into_iter().zip(ok).zip(tys) {
                if ok {
                    self.declare_local(s, sp, false, t);
                }
            }
            if let Some(g) = &clause.guard {
                let t = self.expr(g);
                self.condition(g, &t);
            }
            let body = match &clause.body.kind {
                ExprKind::Block(stmts) => self.stmts(stmts),
                _ => self.expr(&clause.body),
            };
            if matches {
                ret = ret.join(&body);
            }
            self.pop_scope();
        }
        if def.arity == 1 && def.name.is_some() && !self.repl {
            let pats: Vec<&Pattern> = def.clauses.iter().filter(|c| c.guard.is_none()).map(|c| &c.params[0]).collect();
            let missing = self.missing(&pats);
            if !missing.is_empty() {
                let list: Vec<String> = missing.iter().map(|m| format!("`{m}`")).collect();
                self.diags.push(
                    Diagnostic::warning(
                        "W002",
                        t!("`{name}` does not handle every possible argument", "`{name}` ne traite pas tous les arguments possibles"),
                    )
                    .at(def.span)
                    .note(t!("not handled: {}", "non traité : {}", list.join(", ")))
                    .help(t!(
                        "add a definition for it, like `fn {name}({}) = ...`",
                        "ajoutez une définition pour ce cas, comme `fn {name}({}) = ...`",
                        missing[0]
                    )),
                );
            }
        }
        let early = self.early.pop().unwrap_or(Ty::Never);
        self.fn_depth -= 1;
        Ty::func(Some((def.arity, def.arity)), ret.join(&early), Some(def.signature().into()))
    }

    /// Checks the statements of a block and gives the type of its value.
    fn stmts(&mut self, stmts: &[Stmt]) -> Ty {
        let n = stmts.len();
        let mut last = Ty::Unit;
        let mut stops = false;
        for (i, s) in stmts.iter().enumerate() {
            match s {
                Stmt::Let(p, e) => {
                    last = Ty::Unit;
                    if let (PatKind::Bind(name), ExprKind::Lambda(def)) = (&p.kind, &e.kind) {
                        // `name = fn(...)` sees its own name, so that it can be recursive.
                        let info = GlobalInfo { sig: def.signature().replacen("fn(", &format!("{name}("), 1), ..func_info(def) };
                        let first = Ty::func(Some((def.arity, def.arity)), Ty::Any, Some(info.sig.as_str().into()));
                        self.declare_local_fn(*name, p.span, true, Some(info), first, Some(def.clone()));
                        let t = self.func(def, None);
                        self.set_local_type(*name, p.span, t);
                    } else {
                        let t = self.expr(e);
                        stops |= t == Ty::Never;
                        self.let_pattern(p, e, &t);
                        self.bind_pattern(p, true, &t);
                    }
                }
                Stmt::Expr(e) => {
                    let t = self.expr(e);
                    stops |= t == Ty::Never;
                    // Only the last line of a block gives its value.
                    if i + 1 < n && !has_effect(e) {
                        self.diags.push(unused_value(e));
                    }
                    last = t;
                }
                Stmt::Fn(def) => {
                    last = Ty::Unit;
                    if let Some(name) = def.name {
                        let first = Ty::func(Some((def.arity, def.arity)), Ty::Any, Some(def.signature().into()));
                        self.declare_local_fn(name, def.span, true, Some(func_info(def)), first, Some(def.clone()));
                        let t = self.func(def, None);
                        self.set_local_type(name, def.span, t);
                    } else {
                        self.func(def, None);
                    }
                }
            }
        }
        if stops {
            Ty::Never
        } else {
            last
        }
    }

    /// `pattern = value`: the value must fit the pattern.
    fn let_pattern(&mut self, p: &Pattern, e: &Expr, t: &Ty) {
        if !t.is_known() || self.can_match(p, t, true) {
            return;
        }
        let msg = match literal(e) {
            Some(v) => {
                t!("the value {} does not match this pattern", "la valeur {} ne correspond pas à ce motif", v.describe())
            }
            None => t!("a value of type {} never matches this pattern", "une valeur de type {} ne correspond jamais à ce motif", t.name()),
        };
        self.diags.push(Diagnostic::error("E006", msg).at(p.span).label(t!("never matches", "ne correspond jamais")).help(pattern_help()));
    }

    // ----- expressions -------------------------------------------------

    /// Checks an expression and gives its type.
    fn expr(&mut self, e: &Expr) -> Ty {
        match &e.kind {
            ExprKind::Unit => Ty::Unit,
            ExprKind::Bool(_) => Ty::Bool,
            ExprKind::Int(_) => Ty::Int,
            ExprKind::Float(_) => Ty::Float,
            ExprKind::Text(_) => Ty::Text,
            ExprKind::Template(parts) => {
                let mut stops = false;
                for p in parts {
                    if let TemplatePart::Expr(x) = p {
                        stops |= self.expr(x) == Ty::Never;
                    }
                }
                if stops {
                    Ty::Never
                } else {
                    Ty::Text
                }
            }
            ExprKind::Var(s) => {
                if &*s.name() == "_" {
                    self.diags.push(
                        Diagnostic::error("E001", t!("`_` can only be used in patterns", "`_` ne peut être utilisé que dans les motifs"))
                            .at(e.span)
                            .help(t!(
                                "`_` means \"anything, ignored\"; give the value a real name to use it",
                                "`_` signifie « n'importe quoi, ignoré » ; donnez un vrai nom à la valeur pour l'utiliser"
                            )),
                    );
                    return Ty::Never;
                }
                self.resolve(*s, e.span).1
            }
            ExprKind::Ctor(s) => self.resolve(*s, e.span).1,
            ExprKind::List(items) => self.list(items),
            ExprKind::Record(items) => self.record(items),
            ExprKind::Lambda(def) => self.func(def, None),
            ExprKind::Call(callee, args) => self.call(callee, args, e.span),
            ExprKind::Method(recv, name, args, name_span) => self.method_call(e, recv, *name, args, *name_span),
            ExprKind::Field(obj, name, name_span) => {
                let t = self.expr(obj);
                self.field(obj, &t, name, *name_span)
            }
            ExprKind::Index(a, b) => {
                let ta = self.expr(a);
                let tb = self.expr(b);
                self.index(e, a, &ta, b, &tb)
            }
            ExprKind::Binary(op, a, b, op_span) => self.binary(*op, a, b, *op_span),
            ExprKind::Range(a, b) => {
                let ta = self.expr(a);
                let tb = self.expr(b);
                self.range(e, a, &ta, b, &tb)
            }
            ExprKind::Unary(op, x) => {
                let t = self.expr(x);
                self.unary(e, *op, x, &t)
            }
            ExprKind::If(c, a, b) => {
                let tc = self.expr(c);
                let ok = self.condition(c, &tc);
                let ta = self.expr(a);
                let tb = self.expr(b);
                if tc == Ty::Never || !ok {
                    Ty::Never
                } else {
                    ta.join(&tb)
                }
            }
            ExprKind::Match(subject, arms) => self.match_expr(e, subject, arms),
            ExprKind::Block(stmts) => {
                self.scopes.push(Vec::new());
                let t = self.stmts(stmts);
                self.pop_scope();
                t
            }
            ExprKind::Try(x) => {
                let t = self.expr(x);
                self.may_check = true;
                if self.fn_depth == 0 {
                    self.diags.push(
                        Diagnostic::error(
                            "E016",
                            t!("`?` can only be used inside a function", "`?` ne peut être utilisé que dans une fonction"),
                        )
                        .at(e.span)
                        .help(t!(
                            "outside of a function, handle the value with `match` or `unwrap_or`",
                            "en dehors d'une fonction, traitez la valeur avec `match` ou `unwrap_or`"
                        )),
                    );
                }
                self.question(e, x, &t)
            }
        }
    }

    /// `x?`: the value inside `Some(...)` or `Ok(...)`; `None` or `Err(...)`
    /// is returned by the function.
    fn question(&mut self, e: &Expr, x: &Expr, t: &Ty) -> Ty {
        match t {
            Ty::Never => Ty::Never,
            Ty::Any => {
                self.return_early(Ty::Any);
                Ty::Any
            }
            Ty::Data(d) if matches!(d.kind(), OPTION | RESULT) => {
                self.return_early(d.failure());
                d.payload()
            }
            _ => {
                self.diags.push(
                    Diagnostic::error(
                        "E016",
                        t!(
                            "`?` needs an Option or a Result, but got {}",
                            "`?` a besoin d'une Option ou d'un Result, mais a reçu {}",
                            what(Some(x), t)
                        ),
                    )
                    .at(e.span)
                    .help(t!(
                        "`?` takes the value out of `Some(x)` / `Ok(x)`, and returns `None` / `Err(e)` from the current function",
                        "`?` sort la valeur de `Some(x)` / `Ok(x)`, et renvoie `None` / `Err(e)` depuis la fonction en cours"
                    )),
                );
                Ty::Never
            }
        }
    }

    fn return_early(&mut self, t: Ty) {
        if let Some(early) = self.early.last_mut() {
            *early = early.join(&t);
        }
    }

    fn list(&mut self, items: &[ListItem]) -> Ty {
        let mut elem = Ty::Never;
        let mut stops = false;
        for it in items {
            match it {
                ListItem::One(x) => {
                    let t = self.expr(x);
                    stops |= t == Ty::Never;
                    elem = elem.join(&t);
                }
                ListItem::Spread(x) => {
                    let t = self.expr(x);
                    stops |= t == Ty::Never;
                    if t.never(LIST) {
                        self.spread_error(x, &t, true);
                        stops = true;
                    }
                    elem = elem.join(&t.elem());
                }
            }
        }
        if stops {
            Ty::Never
        } else {
            Ty::list(elem)
        }
    }

    fn record(&mut self, items: &[RecordItem]) -> Ty {
        let mut seen: HashMap<Rc<str>, Span> = HashMap::new();
        let mut fields: Vec<(Rc<str>, Ty)> = Vec::new();
        let mut exact = true;
        let mut stops = false;
        let set = |fields: &mut Vec<(Rc<str>, Ty)>, k: &Rc<str>, t: Ty| match fields.iter_mut().find(|(f, _)| f == k) {
            Some(f) => f.1 = t,
            None => fields.push((k.clone(), t)),
        };
        for it in items {
            match it {
                RecordItem::Field(k, ks, x) => {
                    if let Some(prev) = seen.get(k) {
                        self.diags.push(
                            Diagnostic::error("E011", t!("the field `{k}` appears twice", "le champ `{k}` apparaît deux fois"))
                                .at(*ks)
                                .secondary(*prev, t!("first here", "une première fois ici")),
                        );
                    }
                    seen.insert(k.clone(), *ks);
                    let t = self.expr(x);
                    stops |= t == Ty::Never;
                    set(&mut fields, k, t);
                }
                RecordItem::Spread(x) => {
                    let t = self.expr(x);
                    match &t {
                        Ty::Never => stops = true,
                        Ty::Record(r) => {
                            if !r.exact {
                                // It may have other fields, replacing the ones above.
                                exact = false;
                                fields.iter_mut().for_each(|f| f.1 = Ty::Any);
                            }
                            for (k, ft) in &r.fields {
                                set(&mut fields, k, ft.clone());
                            }
                        }
                        t if t.never(RECORD) => {
                            self.spread_error(x, t, false);
                            stops = true;
                        }
                        _ => {
                            exact = false;
                            fields.iter_mut().for_each(|f| f.1 = Ty::Any);
                        }
                    }
                }
            }
        }
        if stops {
            Ty::Never
        } else {
            Ty::record(fields, exact)
        }
    }

    fn spread_error(&mut self, x: &Expr, t: &Ty, list: bool) {
        let expected = if list { t!("a list", "une liste") } else { t!("a record", "un enregistrement") };
        self.diags.push(
            Diagnostic::error(
                "E005",
                t!("`..` expects {expected} here, but got {}", "`..` attend {expected} ici, mais a reçu {}", what(Some(x), t)),
            )
            .at(x.span),
        );
    }

    fn call(&mut self, callee: &Expr, args: &[Expr], span: Span) -> Ty {
        match &callee.kind {
            ExprKind::Var(s) | ExprKind::Ctor(s) if &*s.name() != "_" => {
                let (res, ty) = self.resolve(*s, callee.span);
                self.call_target(Some(*s), &res, ty, None, args, span)
            }
            ExprKind::Int(_)
            | ExprKind::Float(_)
            | ExprKind::Text(_)
            | ExprKind::Template(_)
            | ExprKind::Bool(_)
            | ExprKind::List(_)
            | ExprKind::Record(_) => {
                self.diags.push(
                    Diagnostic::error("E009", t!("this value is not a function", "cette valeur n'est pas une fonction"))
                        .at(callee.span)
                        .label(t!("cannot be called", "ne peut pas être appelé"))
                        .help(t!(
                            "to multiply, write `*` explicitly: `2 * (x + 1)`",
                            "pour multiplier, écrivez `*` explicitement : `2 * (x + 1)`"
                        )),
                );
                for a in args {
                    self.expr(a);
                }
                Ty::Never
            }
            // A function called right where it is written (`x |> fn(v) = ...`)
            // receives exactly these arguments.
            ExprKind::Lambda(def) => {
                let tys: Vec<Ty> = args.iter().map(|a| self.expr(a)).collect();
                if def.arity != args.len() {
                    self.func(def, None);
                    self.diags.push(*arity_error("fn", &def.signature(), def.arity, def.arity, args.len(), span, Some(def.span)));
                    return Ty::Never;
                }
                let t = self.func(def, Some(&tys));
                if tys.contains(&Ty::Never) {
                    Ty::Never
                } else {
                    t.ret()
                }
            }
            _ => {
                let ty = self.expr(callee);
                self.may_check = true;
                self.call_target(None, &Res::Unknown, ty, None, args, span)
            }
        }
    }

    /// A call, once the function is known: `f(args)`, or `x.f(args)` with the
    /// receiver `x` given first.
    fn call_target(&mut self, name: Option<Sym>, res: &Res, fty: Ty, first: Option<(&Expr, Ty)>, args: &[Expr], span: Span) -> Ty {
        let builtin = match res {
            Res::Global(g) if g.kind == GKind::Builtin => builtin_name(g),
            _ => None,
        };
        let exprs: Vec<&Expr> = first.as_ref().map(|(e, _)| *e).into_iter().chain(args.iter()).collect();
        let pre: Vec<Ty> = first.map(|(_, t)| t).into_iter().collect();
        let tys = self.args(builtin, &exprs, pre);
        let n = tys.len();
        let arity_ok = match (name, res) {
            (Some(name), Res::Global(g) | Res::Local(Some(g), _)) if g.kind != GKind::Value => self.check_arity(name, g, n, span),
            _ => self.check_value_arity(name, &fty, n, span),
        };
        if !arity_ok || fty == Ty::Never || tys.contains(&Ty::Never) {
            return Ty::Never;
        }
        if let Some(b) = builtin {
            let t = self.builtin_result(b, &exprs, &tys, span);
            if t != Ty::Never {
                self.callbacks(b, &exprs, &tys);
            }
            return t;
        }
        match &fty {
            Ty::Func(f) => {
                if let (Some(c), Res::Global(g)) = (name, res) {
                    if g.kind == GKind::Ctor {
                        return match self.ctors.get(&c) {
                            Some(m) => Ty::data(m.type_name, Some(vec![(c, tys)])),
                            None => f.ret.clone(),
                        };
                    }
                }
                let def = match (name, res) {
                    (Some(s), Res::Global(g)) if g.kind == GKind::Func && self.own.contains_key(&s) => self.fn_defs.get(&s).cloned(),
                    (_, Res::Local(_, def)) => def.clone(),
                    _ => None,
                };
                if let Some(def) = def {
                    if def.arity == n && !self.some_clause_accepts(&def, &tys) {
                        self.no_clause(&def, &exprs, &tys, span);
                        return Ty::Never;
                    }
                }
                f.ret.clone()
            }
            Ty::Any => Ty::Any,
            t => {
                let (shown, msg) = match name {
                    Some(s) => (Some(s), t!("`{s}` is not a function", "`{s}` n'est pas une fonction")),
                    None => (None, t!("this value is not a function", "cette valeur n'est pas une fonction")),
                };
                let mut d = Diagnostic::error("E009", msg).at(span).label(t!("cannot be called", "ne peut pas être appelé"));
                if let Some(s) = shown {
                    d = d.note(t!("`{s}` is {}", "`{s}` est {}", what(None, t)));
                }
                self.diags.push(d);
                Ty::Never
            }
        }
    }

    /// Checks the arguments of a call and gives their types. A function
    /// written in place for a built-in function (`map(xs, fn(x) = ...)`)
    /// knows what it receives.
    fn args(&mut self, builtin: Option<&str>, exprs: &[&Expr], mut tys: Vec<Ty>) -> Vec<Ty> {
        let offset = tys.len();
        let mut later = Vec::new();
        for (i, a) in exprs.iter().enumerate().skip(offset) {
            if let (Some(b), ExprKind::Lambda(_)) = (builtin, &a.kind) {
                if types::callback(b, i, &[]).is_some() {
                    later.push(i);
                    tys.push(Ty::Any);
                    continue;
                }
            }
            tys.push(self.expr(a));
        }
        let Some(b) = builtin else { return tys };
        for i in later {
            let ExprKind::Lambda(def) = &exprs[i].kind else { continue };
            let Some(cb) = types::callback(b, i, &tys) else { continue };
            tys[i] = if def.arity != cb.params.len() {
                self.func(def, None)
            } else if b == "fold" {
                self.fold_fn(def, &tys[1], &cb.params[1])
            } else {
                self.func(def, Some(&cb.params))
            };
        }
        tys
    }

    /// The functions given to a built-in function must take what it gives
    /// them, and return what it needs.
    fn callbacks(&mut self, b: &str, exprs: &[&Expr], tys: &[Ty]) {
        for (i, t) in tys.iter().enumerate() {
            let Some(cb) = types::callback(b, i, tys) else { continue };
            let Ty::Func(f) = t else { continue };
            let n = cb.params.len();
            if let Some((min, max)) = f.arity {
                if n < min || n > max {
                    let expected = if min == max { min.to_string() } else { t!("{min} to {max}", "de {min} à {max}") };
                    self.diags.push(
                        Diagnostic::error(
                            "E004",
                            t!(
                                "`{b}` gives {n} argument(s) to this function, but it takes {expected}",
                                "`{b}` donne {n} argument(s) à cette fonction, mais elle en attend {expected}"
                            ),
                        )
                        .at(exprs[i].span)
                        .label(t!("takes {expected} argument(s)", "attend {expected} argument(s)"))
                        .help(usage(b)),
                    );
                    continue;
                }
            }
            if let Some(kinds) = cb.returns {
                if f.ret.never(kinds) {
                    let returned = what(None, &f.ret);
                    let d = if kinds == BOOL {
                        Diagnostic::error(
                            "E005",
                            t!(
                                "the function given to `{b}` must return true or false, but it returns {returned}",
                                "la fonction donnée à `{b}` doit renvoyer true ou false, mais elle renvoie {returned}"
                            ),
                        )
                    } else {
                        Diagnostic::error(
                            "E023",
                            t!(
                                "the function given to `{b}` must return a list, but it returns {returned}",
                                "la fonction donnée à `{b}` doit renvoyer une liste, mais elle renvoie {returned}"
                            ),
                        )
                    };
                    self.diags.push(d.at(exprs[i].span).label(t!("returns {returned}", "renvoie {returned}")).help(usage(b)));
                }
            }
        }
    }

    /// `fold(list, start, fn(total, x) = ...)`: the total starts as `start`,
    /// then is what the function gave back. Its type is found by trying
    /// again until it is stable.
    fn fold_fn(&mut self, def: &FnDef, start: &Ty, elem: &Ty) -> Ty {
        // Inside other `fold`s, the function would be checked too many times.
        if self.folds >= 2 {
            return self.func(def, Some(&[Ty::Any, elem.clone()]));
        }
        self.folds += 1;
        let mut total = start.clone();
        let reported = self.diags.len();
        let mut stable = false;
        for _ in 0..4 {
            let f = self.func(def, Some(&[total.clone(), elem.clone()]));
            self.diags.truncate(reported);
            let next = total.join(&f.ret()).limit();
            if next == total {
                stable = true;
                break;
            }
            total = next;
        }
        let t = self.func(def, Some(&[if stable { total } else { Ty::Any }, elem.clone()]));
        self.folds -= 1;
        t
    }

    /// What a call to a built-in function gives; reports an argument it
    /// never accepts.
    fn builtin_result(&mut self, name: &str, exprs: &[&Expr], tys: &[Ty], span: Span) -> Ty {
        if name == "pow" && tys.len() == 2 {
            return self.arith(BinOp::Pow, Some(exprs[0]), &tys[0], Some(exprs[1]), &tys[1], span);
        }
        match types::builtin_call(name, tys) {
            None => Ty::Any,
            Some(Outcome::Gives(t)) => {
                // `get(record, "field")`: the type of the field.
                if let (Some(Ty::Record(r)), Some(ExprKind::Text(k))) = (tys.first(), exprs.get(1).map(|e| &e.kind)) {
                    if name == "get" {
                        return match r.get(k) {
                            Some(ft) => Ty::option(ft.clone()),
                            None if r.exact => Ty::data(Sym::new("Option"), Some(vec![(Sym::new("None"), Vec::new())])),
                            None => t,
                        };
                    }
                }
                t
            }
            Some(Outcome::Wrong { index, expected }) => {
                let pos = index + 1;
                self.diags.push(
                    Diagnostic::error(
                        "E005",
                        t!(
                            "`{name}` expects {expected} as argument {pos}, but got {}",
                            "`{name}` attend {expected} comme argument {pos}, mais a reçu {}",
                            what(Some(exprs[index]), &tys[index])
                        ),
                    )
                    .at(exprs[index].span)
                    .label(t!("wrong kind of value", "mauvaise sorte de valeur"))
                    .help(usage(name)),
                );
                Ty::Never
            }
        }
    }

    /// Can at least one definition of the function accept arguments of these types?
    fn some_clause_accepts(&self, def: &FnDef, tys: &[Ty]) -> bool {
        def.clauses.iter().any(|c| c.params.iter().zip(tys).all(|(p, t)| self.can_match(p, t, true)))
    }

    fn no_clause(&mut self, def: &FnDef, exprs: &[&Expr], tys: &[Ty], span: Span) {
        let name = def.name.map(|n| n.name().to_string()).unwrap_or_else(|| "fn".to_string());
        // Like the run time shows them when they are written in the code (`5`, `None`), or by their type.
        let shown: Vec<String> = exprs
            .iter()
            .zip(tys)
            .map(|(e, t)| match (&e.kind, literal(e)) {
                (_, Some(v)) => v.repr(),
                (ExprKind::Ctor(c), None) if matches!(t, Ty::Data(_)) => c.to_string(),
                _ => t.name(),
            })
            .collect();
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
        self.diags.push(d.help(t!(
            "add a definition that handles this case, for example one that ends with a plain name or `_`",
            "ajoutez une définition qui traite ce cas, par exemple une qui se termine par un simple nom ou `_`"
        )));
    }

    /// `value.name(args)` calls the field `name` of a record, or else the
    /// function `name` with the value first.
    fn method_call(&mut self, e: &Expr, recv: &Expr, name: Sym, args: &[Expr], name_span: Span) -> Ty {
        let rt = self.expr(recv);
        let field = match &rt {
            Ty::Record(r) => match r.get(&name.name()) {
                Some(t) => Some(Some(t.clone())),
                None if r.exact => Some(None),
                None => None,
            },
            Ty::Any | Ty::Never => None,
            _ => Some(None),
        };
        match field {
            // The field of the record.
            Some(Some(ft)) => {
                self.may_check = true;
                self.call_target(None, &Res::Unknown, ft, None, args, e.span)
            }
            // Surely the function.
            Some(None) => match self.lookup_method(name) {
                Some((res, fty)) => self.call_target(Some(name), &res, fty, Some((recv, rt)), args, e.span),
                None => {
                    for a in args {
                        self.expr(a);
                    }
                    self.unknown_method(name, name_span);
                    Ty::Never
                }
            },
            None => {
                for a in args {
                    self.expr(a);
                }
                self.method(name, args.len() + 1, e.span, name_span);
                self.may_check = true;
                if rt == Ty::Never {
                    Ty::Never
                } else {
                    Ty::Any
                }
            }
        }
    }

    /// The function called by `value.name(...)`, if there is one.
    fn lookup_method(&mut self, name: Sym) -> Option<(Res, Ty)> {
        for scope in self.scopes.iter_mut().rev() {
            for l in scope.iter_mut().rev() {
                if l.sym == name {
                    l.used = true;
                    let r = (Res::Local(l.info.clone(), l.def.clone()), l.ty.clone());
                    self.note_use(&r);
                    return Some(r);
                }
            }
        }
        let g = self.own.get(&name).or_else(|| self.know.globals.get(&name)).cloned()?;
        let ty = if self.own.contains_key(&name) { self.own_type(name, &g) } else { self.known_type(name, &g) };
        let r = (Res::Global(g), ty);
        self.note_use(&r);
        Some(r)
    }

    /// `value.name(args)` calls a record's field `name`, or the function `name`.
    fn method(&mut self, name: Sym, n: usize, span: Span, name_span: Span) {
        for scope in self.scopes.iter_mut().rev() {
            for l in scope.iter_mut().rev() {
                if l.sym == name {
                    l.used = true;
                    return;
                }
            }
        }
        // A record of this file has a field with this name: it may be what is called.
        let maybe_field = self.record_fields.contains(&*name.name());
        let g = self.own.get(&name).or_else(|| self.know.globals.get(&name)).cloned();
        match g {
            Some(g) if matches!(g.kind, GKind::Builtin | GKind::Func) => {
                if !maybe_field {
                    self.check_arity(name, &g, n, span);
                }
            }
            Some(_) => {}
            None if maybe_field => {}
            None => self.unknown_method(name, name_span),
        }
    }

    fn unknown_method(&mut self, name: Sym, name_span: Span) {
        let shown = name.name();
        let mut d = Diagnostic::warning("W005", t!("there is no function named `{shown}`", "il n'existe pas de fonction nommée `{shown}`"))
            .at(name_span)
            .label(t!("unknown", "inconnue"));
        if let Some(h) = suggest::alias_hint(&shown) {
            d = d.help(h);
        } else {
            let names: Vec<Rc<str>> = self.own.keys().chain(self.know.globals.keys()).map(|s| s.name()).collect();
            if let Some(best) = suggest::closest(&shown, names.iter().map(|s| &**s)) {
                d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
            }
        }
        self.diags.push(d.note(t!(
            "`value.{shown}(...)` calls `{shown}(value, ...)`, or the field `{shown}` of a record",
            "`valeur.{shown}(...)` appelle `{shown}(valeur, ...)`, ou le champ `{shown}` d'un enregistrement"
        )));
    }

    /// Returns false if the call surely fails.
    fn check_arity(&mut self, name: Sym, g: &GlobalInfo, n: usize, span: Span) -> bool {
        match g.kind {
            GKind::CtorValue => {
                self.diags.push(
                    Diagnostic::error("E009", t!("`{name}` is a value, not a function", "`{name}` est une valeur, pas une fonction"))
                        .at(span)
                        .help(t!("write `{name}` without parentheses", "écrivez `{name}` sans parenthèses")),
                );
                false
            }
            GKind::Value => true,
            _ => {
                if let Some((min, max)) = g.arity {
                    if n < min || n > max {
                        self.diags.push(*arity_error(&name.name(), &g.sig, min, max, n, span, g.span));
                        return false;
                    }
                }
                true
            }
        }
    }

    /// The number of arguments of a call to a function held by a value.
    fn check_value_arity(&mut self, name: Option<Sym>, t: &Ty, n: usize, span: Span) -> bool {
        let Ty::Func(f) = t else { return true };
        let Some((min, max)) = f.arity else { return true };
        if n >= min && n <= max {
            return true;
        }
        let shown = name.map(|s| s.name().to_string()).unwrap_or_else(|| "fn".into());
        let sig = f.sig.as_deref().map(|s| s.replacen("fn(", &format!("{shown}("), 1)).unwrap_or_else(|| format!("{shown}(...)"));
        self.diags.push(*arity_error(&shown, &sig, min, max, n, span, None));
        false
    }

    fn field(&mut self, obj: &Expr, t: &Ty, name: &str, span: Span) -> Ty {
        match t {
            Ty::Never => Ty::Never,
            Ty::Any => Ty::Any,
            Ty::Record(r) => match r.get(name) {
                Some(ft) => ft.clone(),
                None if r.exact => {
                    let mut d =
                        Diagnostic::error("E010", t!("this record has no field `{name}`", "cet enregistrement n'a pas de champ `{name}`"))
                            .at(span)
                            .label(t!("unknown field", "champ inconnu"));
                    if let Some(best) = suggest::closest(name, r.fields.iter().map(|(k, _)| &**k)) {
                        d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
                    }
                    if r.fields.is_empty() {
                        d = d.note(t!("the record is empty: {{}}", "l'enregistrement est vide : {{}}"));
                    } else {
                        let keys: Vec<String> = r.fields.iter().take(12).map(|(k, _)| format!("`{k}`")).collect();
                        d = d.note(t!("its fields are: {}", "ses champs sont : {}", keys.join(", ")));
                    }
                    self.diags.push(d.help(t!(
                        "to get an Option instead of an error, use `get(record, \"{name}\")`",
                        "pour obtenir une Option au lieu d'une erreur, utilisez `get(enregistrement, \"{name}\")`"
                    )));
                    Ty::Never
                }
                None => Ty::Any,
            },
            Ty::Data(d) => self.data_field(d, name, span),
            other => {
                let mut d = Diagnostic::error("E010", t!("{} has no fields", "{} n'a pas de champs", what(Some(obj), other)))
                    .at(span)
                    .label(t!("no field `{name}` here", "pas de champ `{name}` ici"));
                let callable = self
                    .own
                    .get(&Sym::new(name))
                    .or_else(|| self.know.globals.get(&Sym::new(name)))
                    .is_some_and(|g| matches!(g.kind, GKind::Builtin | GKind::Func | GKind::Ctor));
                if callable {
                    d = d.help(t!(
                        "to call `{name}` on this value, add parentheses: `.{name}()`",
                        "pour appeler `{name}` sur cette valeur, ajoutez des parenthèses : `.{name}()`"
                    ));
                }
                self.diags.push(d);
                Ty::Never
            }
        }
    }

    /// `value.name` on a value built with a constructor.
    fn data_field(&mut self, d: &types::DataTy, name: &str, span: Span) -> Ty {
        let ctors: Vec<(Sym, Option<Vec<Ty>>)> = match &d.ctors {
            Some(cs) => cs.iter().map(|(c, fs)| (*c, Some(fs.clone()))).collect(),
            None => self.types.get(&d.name).map(|cs| cs.iter().map(|c| (*c, None)).collect()).unwrap_or_default(),
        };
        let mut found = None;
        let mut names: Vec<Rc<str>> = Vec::new();
        for (c, fs) in &ctors {
            let Some(meta) = self.ctors.get(c) else { return Ty::Any };
            if let Some(i) = meta.fields.iter().position(|f| &*f.name() == name) {
                let t = fs.as_ref().and_then(|fs| fs.get(i).cloned()).unwrap_or(Ty::Any);
                found = Some(found.unwrap_or(Ty::Never).join(&t));
            }
            for f in &meta.fields {
                if !names.contains(&f.name()) {
                    names.push(f.name());
                }
            }
        }
        if let Some(t) = found {
            return t;
        }
        if ctors.is_empty() {
            return Ty::Any;
        }
        let tn = d.name;
        let mut diag = match ctors.as_slice() {
            [(c, _)] => Diagnostic::error("E010", t!("`{c}` has no field `{name}`", "`{c}` n'a pas de champ `{name}`")),
            _ => Diagnostic::error(
                "E010",
                t!("no variant of `{tn}` has a field `{name}`", "aucune variante de `{tn}` n'a de champ `{name}`"),
            ),
        }
        .at(span)
        .label(t!("unknown field", "champ inconnu"));
        if let Some(best) = suggest::closest(name, names.iter().map(|s| &**s)) {
            diag = diag.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
        }
        let shown: Vec<String> = names.iter().map(|f| format!("`{f}`")).collect();
        diag = match (ctors.as_slice(), shown.is_empty()) {
            ([(c, _)], true) => diag.note(t!("`{c}` has no fields", "`{c}` n'a aucun champ")),
            ([(c, _)], false) => diag.note(t!("the fields of `{c}` are: {}", "les champs de `{c}` sont : {}", shown.join(", "))),
            (_, true) => diag.note(t!("the variants of `{tn}` have no fields", "les variantes de `{tn}` n'ont aucun champ")),
            (_, false) => diag.note(t!("their fields are: {}", "leurs champs sont : {}", shown.join(", "))),
        };
        self.diags.push(diag.help(t!(
            "a value of type `{tn}` may be another variant: use `match` to handle each case",
            "une valeur du type `{tn}` peut être une autre variante : utilisez `match` pour traiter chaque cas"
        )));
        Ty::Never
    }

    fn index(&mut self, e: &Expr, a: &Expr, ta: &Ty, b: &Expr, tb: &Ty) -> Ty {
        if *ta == Ty::Never || *tb == Ty::Never {
            return Ty::Never;
        }
        if ta.never(LIST | TEXT | RECORD) {
            self.diags.push(
                Diagnostic::error("E005", t!("cannot use `[...]` on {}", "impossible d'utiliser `[...]` sur {}", what(Some(a), ta)))
                    .at(e.span)
                    .help(t!(
                        "`[...]` works on lists, texts and records",
                        "`[...]` fonctionne sur les listes, les textes et les enregistrements"
                    )),
            );
            return Ty::Never;
        }
        if ta.only(LIST | TEXT) && tb.never(INT) {
            self.diags.push(
                Diagnostic::error(
                    "E005",
                    t!(
                        "an index must be a whole number (Int), but got {}",
                        "un indice doit être un nombre entier (Int), mais a reçu {}",
                        what(Some(b), tb)
                    ),
                )
                .at(b.span),
            );
            return Ty::Never;
        }
        if ta.only(RECORD) && tb.never(TEXT) {
            self.diags.push(
                Diagnostic::error(
                    "E005",
                    t!(
                        "the name of a field must be a text, but got {}",
                        "le nom d'un champ doit être un texte, mais a reçu {}",
                        what(Some(b), tb)
                    ),
                )
                .at(b.span)
                .help(t!(
                    "read a field with `record[\"name\"]` or `record.name`",
                    "lisez un champ avec `enregistrement[\"nom\"]` ou `enregistrement.nom`"
                )),
            );
            return Ty::Never;
        }
        match ta {
            Ty::List(t) => (**t).clone(),
            Ty::Text => Ty::Text,
            Ty::Record(r) => {
                let ExprKind::Text(k) = &b.kind else { return Ty::Any };
                match r.get(k) {
                    Some(t) => t.clone(),
                    None if r.exact => {
                        let mut d =
                            Diagnostic::error("E010", t!("this record has no field `{k}`", "cet enregistrement n'a pas de champ `{k}`"))
                                .at(b.span);
                        if let Some(best) = suggest::closest(k, r.fields.iter().map(|(f, _)| &**f)) {
                            d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
                        }
                        self.diags.push(d.help(t!(
                            "use `get(record, key)` to receive an Option instead",
                            "utilisez `get(enregistrement, clé)` pour obtenir une Option à la place"
                        )));
                        Ty::Never
                    }
                    None => Ty::Any,
                }
            }
            _ => Ty::Any,
        }
    }

    fn range(&mut self, e: &Expr, a: &Expr, ta: &Ty, b: &Expr, tb: &Ty) -> Ty {
        if *ta == Ty::Never || *tb == Ty::Never {
            return Ty::Never;
        }
        if !ta.never(INT) && !tb.never(INT) {
            return Ty::list(Ty::Int);
        }
        let got = if ta.is_known() && tb.is_known() {
            t!("{} and {}", "{} et {}", what(Some(a), ta), what(Some(b), tb))
        } else if ta.never(INT) {
            what(Some(a), ta)
        } else {
            what(Some(b), tb)
        };
        self.diags.push(
            Diagnostic::error(
                "E005",
                t!("a range needs whole numbers, but got {got}", "un intervalle a besoin de nombres entiers, mais a reçu {got}"),
            )
            .at(e.span)
            .help(t!("example: `1..10` is the list of the numbers from 1 to 10", "exemple : `1..10` est la liste des nombres de 1 à 10")),
        );
        Ty::Never
    }

    fn unary(&mut self, e: &Expr, op: UnOp, x: &Expr, t: &Ty) -> Ty {
        if *t == Ty::Never {
            return Ty::Never;
        }
        match op {
            UnOp::Neg => {
                if t.never(NUMBER) {
                    self.diags.push(
                        Diagnostic::error("E005", t!("cannot use `-` on {}", "impossible d'utiliser `-` sur {}", what(Some(x), t)))
                            .at(e.span),
                    );
                    return Ty::Never;
                }
                match t {
                    Ty::Int | Ty::Float => t.clone(),
                    _ => Ty::Num,
                }
            }
            UnOp::Not => {
                if self.condition(x, t) {
                    Ty::Bool
                } else {
                    Ty::Never
                }
            }
        }
    }

    /// A condition must be true or false. Returns false if it surely is not.
    fn condition(&mut self, e: &Expr, t: &Ty) -> bool {
        if !t.never(BOOL) {
            return true;
        }
        let mut d = Diagnostic::error(
            "E005",
            t!(
                "a condition must be `true` or `false`, but this is {}",
                "une condition doit être `true` ou `false`, mais ceci est {}",
                what(Some(e), t)
            ),
        )
        .at(e.span)
        .label(t!("not a Bool", "pas un Bool"));
        if t.only(NUMBER) {
            d = d.help(t!(
                "compare it explicitly, for example `x != 0` or `x > 0`",
                "comparez explicitement, par exemple `x != 0` ou `x > 0`"
            ));
        } else if t.only(LIST | TEXT | RECORD) {
            d = d.help(t!(
                "to test if it is empty, use `is_empty(x)` or `not is_empty(x)`",
                "pour tester si c'est vide, utilisez `is_empty(x)` ou `not is_empty(x)`"
            ));
        } else if t.only(OPTION | RESULT | DATA) {
            d = d.help(t!(
                "use `is_some(x)`, `is_ok(x)`, or `match` to look inside",
                "utilisez `is_some(x)`, `is_ok(x)`, ou `match` pour regarder à l'intérieur"
            ));
        }
        self.diags.push(d);
        false
    }

    fn binary(&mut self, op: BinOp, l: &Expr, r: &Expr, op_span: Span) -> Ty {
        let lt = self.expr(l);
        let rt = self.expr(r);
        match op {
            BinOp::And | BinOp::Or => {
                let left = self.condition(l, &lt);
                // The right side runs only when needed, but when it runs, it must be a Bool too.
                self.condition(r, &rt);
                if lt == Ty::Never || !left {
                    Ty::Never
                } else {
                    Ty::Bool
                }
            }
            _ if lt == Ty::Never || rt == Ty::Never => Ty::Never,
            BinOp::Eq | BinOp::Ne => {
                self.equality(op, l, &lt, r, &rt, op_span);
                Ty::Bool
            }
            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                if comparable(&lt, &rt) {
                    Ty::Bool
                } else {
                    self.op_error(op, Some(l), &lt, Some(r), &rt, op_span);
                    Ty::Never
                }
            }
            _ => self.arith(op, Some(l), &lt, Some(r), &rt, op_span),
        }
    }

    /// `+ - * / // % **`: what they give, or an error when they surely fail.
    fn arith(&mut self, op: BinOp, l: Option<&Expr>, lt: &Ty, r: Option<&Expr>, rt: &Ty, at: Span) -> Ty {
        if *lt == Ty::Never || *rt == Ty::Never {
            return Ty::Never;
        }
        let (lk, rk) = (lt.kinds(), rt.kinds());
        let numbers = lk & NUMBER != 0 && rk & NUMBER != 0;
        let texts = op == BinOp::Add && lk & TEXT != 0 && rk & TEXT != 0;
        let lists = op == BinOp::Add && lk & LIST != 0 && rk & LIST != 0;
        if !(numbers || texts || lists) {
            self.op_error(op, l, lt, r, rt, at);
            return Ty::Never;
        }
        let mut t = Ty::Never;
        if numbers {
            let int = |t: &Ty| *t == Ty::Int;
            let float = |t: &Ty| *t == Ty::Float;
            t = t.join(&match op {
                BinOp::Div => Ty::Float,
                _ if float(lt) || float(rt) => Ty::Float,
                // `2 ** -1` is 0.5.
                BinOp::Pow => Ty::Num,
                _ if int(lt) && int(rt) => Ty::Int,
                _ => Ty::Num,
            });
        }
        if texts {
            t = t.join(&Ty::Text);
        }
        if lists {
            t = t.join(&Ty::list(lt.elem().join(&rt.elem())));
        }
        t
    }

    /// The error of an operator that never works with these values, like
    /// the one the program would stop with.
    fn op_error(&mut self, op: BinOp, l: Option<&Expr>, lt: &Ty, r: Option<&Expr>, rt: &Ty, at: Span) {
        let sym = op.symbol();
        let (ln, rn) = (lt.name(), rt.name());
        let msg = match (lt.is_known(), rt.is_known()) {
            (true, true) => t!("cannot use `{sym}` with {ln} and {rn}", "impossible d'utiliser `{sym}` avec {ln} et {rn}"),
            (true, false) => t!("cannot use `{sym}` with {ln}", "impossible d'utiliser `{sym}` avec {ln}"),
            _ => t!("cannot use `{sym}` with {rn}", "impossible d'utiliser `{sym}` avec {rn}"),
        };
        let mut d = Diagnostic::error("E005", msg).at(at).label(format!("{} {sym} {}", short(l, lt), short(r, rt)));
        let text = |t: &Ty| *t == Ty::Text;
        let list = |t: &Ty| matches!(t, Ty::List(_));
        let num = |t: &Ty| matches!(t, Ty::Int | Ty::Float | Ty::Num);
        if op == BinOp::Add && (text(lt) || text(rt)) {
            d = d.help(t!(
                "to build a text with a value inside, use braces: \"total: {{x}}\"\nor convert the value first: text(x)",
                "pour construire un texte contenant une valeur, utilisez des accolades : \"total : {{x}}\"\nou convertissez d'abord la valeur : text(x)"
            ));
        } else if op == BinOp::Add && (list(lt) || list(rt)) {
            d = d.help(t!(
                "to add one element to a list, use `push(list, x)` or `[..list, x]`",
                "pour ajouter un élément à une liste, utilisez `push(liste, x)` ou `[..liste, x]`"
            ));
        } else if op == BinOp::Mul && (text(lt) || list(lt)) && num(rt) {
            d = d.help(t!("to repeat a text or a list, use `repeat(x, n)`", "pour répéter un texte ou une liste, utilisez `repeat(x, n)`"));
        } else if (text(lt) && num(rt)) || (num(lt) && text(rt)) {
            d = d.help(t!(
                "convert the text to a number first, with `int(t)` or `float(t)`",
                "convertissez d'abord le texte en nombre, avec `int(t)` ou `float(t)`"
            ));
        }
        if matches!(op, BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge) && lt.is_known() && rt.is_known() {
            if ln != rn {
                d = d.help(t!("only values of the same kind can be compared", "seules des valeurs de même nature peuvent être comparées"));
            } else if matches!((lt, rt), (Ty::Data(_), Ty::Data(_))) {
                d = d.help(t!(
                    "values built with different constructors cannot be put in order;\nto order them, give each one a number with a function, and compare the numbers",
                    "des valeurs construites avec des constructeurs différents ne peuvent pas être ordonnées ;\npour les ordonner, donnez à chacune un nombre avec une fonction, et comparez les nombres"
                ));
            }
        }
        self.diags.push(d);
    }

    /// `==` and `!=` never fail, but values of different kinds are never equal.
    fn equality(&mut self, op: BinOp, l: &Expr, lt: &Ty, r: &Expr, rt: &Ty, at: Span) {
        // `print(1 == "1")` shows on purpose that Rythme does not convert values.
        if !lt.is_known() || !rt.is_known() || (literal(l).is_some() && literal(r).is_some()) || may_be_equal(lt, rt) {
            return;
        }
        let (ln, rn) = (lt.name(), rt.name());
        let msg = if op == BinOp::Eq {
            t!(
                "this comparison is always false: {ln} and {rn} are never equal",
                "cette comparaison est toujours fausse : {ln} et {rn} ne sont jamais égaux"
            )
        } else {
            t!(
                "this comparison is always true: {ln} and {rn} are never equal",
                "cette comparaison est toujours vraie : {ln} et {rn} ne sont jamais égaux"
            )
        };
        let sym = op.symbol();
        let mut d = Diagnostic::warning("W006", msg).at(at).label(format!("{} {sym} {}", short(Some(l), lt), short(Some(r), rt)));
        let option = |t: &Ty| t.kinds() == OPTION;
        let text = |t: &Ty| *t == Ty::Text;
        let num = |t: &Ty| t.only(NUMBER);
        if option(lt) || option(rt) {
            d = d.help(t!(
                "to look inside an Option, use `match`, `unwrap_or(x, default)` or `is_some(x)`",
                "pour regarder dans une Option, utilisez `match`, `unwrap_or(x, defaut)` ou `is_some(x)`"
            ));
        } else if (text(lt) && num(rt)) || (num(lt) && text(rt)) {
            d = d.help(t!(
                "Rythme never converts values silently: convert one side first, with `text(x)` or `int(t)`",
                "Rythme ne convertit jamais les valeurs en silence : convertissez d'abord un côté, avec `text(x)` ou `int(t)`"
            ));
        } else {
            d = d.help(t!(
                "values of different kinds are never equal: Rythme never converts values silently",
                "des valeurs de natures différentes ne sont jamais égales : Rythme ne convertit jamais les valeurs en silence"
            ));
        }
        self.diags.push(d);
    }

    fn match_expr(&mut self, e: &Expr, subject: &Expr, arms: &[Arm]) -> Ty {
        let st = self.expr(subject);
        let mut catch_all: Option<Span> = None;
        let mut reported = false;
        let mut result = Ty::Never;
        let mut any_matches = false;
        // Cases that never match the type of the value: reported unless no case matches at all.
        let mut dead: Vec<Diagnostic> = Vec::new();
        for (i, arm) in arms.iter().enumerate() {
            if let (Some(prev), false) = (catch_all, reported) {
                reported = true;
                self.diags.push(
                    Diagnostic::warning("W004", t!("this case can never be reached", "ce cas ne peut jamais être atteint"))
                        .at(arm.pat.span)
                        .secondary(prev, t!("this case already handles everything", "ce cas traite déjà tout"))
                        .help(t!("cases are tried in order: put `_` last", "les cas sont essayés dans l'ordre : mettez `_` en dernier")),
                );
            } else if catch_all.is_none() {
                if let Some(prev) = arms[..i].iter().find(|a| a.guard.is_none() && self.covers(&a.pat, &arm.pat)) {
                    self.diags.push(
                        Diagnostic::warning("W004", t!("this case can never be reached", "ce cas ne peut jamais être atteint"))
                            .at(arm.pat.span)
                            .secondary(prev.pat.span, t!("this case already handles these values", "ce cas traite déjà ces valeurs"))
                            .help(t!(
                                "a value goes to the first case that fits: remove this case, or change it",
                                "une valeur va au premier cas qui lui convient : supprimez ce cas, ou modifiez-le"
                            )),
                    );
                } else if !self.can_match(&arm.pat, &st, false) {
                    dead.push(
                        Diagnostic::warning("W004", t!("this case can never be reached", "ce cas ne peut jamais être atteint"))
                            .at(arm.pat.span)
                            .label(t!(
                                "a value of type {} never matches this pattern",
                                "une valeur de type {} ne correspond jamais à ce motif",
                                st.name()
                            ))
                            .secondary(subject.span, st.name())
                            .help(pattern_help()),
                    );
                }
            }
            if arm.guard.is_none() && self.catches_all(&arm.pat, &st) && catch_all.is_none() {
                catch_all = Some(arm.pat.span);
            }
            self.scopes.push(Vec::new());
            self.bind_pattern(&arm.pat, true, &st);
            if let Some(g) = &arm.guard {
                let t = self.expr(g);
                self.condition(g, &t);
            }
            let body = self.expr(&arm.body);
            if self.can_match(&arm.pat, &st, true) {
                any_matches = true;
                result = result.join(&body);
            }
            self.pop_scope();
        }
        if st.is_known() && !any_matches {
            self.diags.push(
                Diagnostic::error(
                    "E006",
                    t!("no case of this `match` handles {}", "aucun cas de ce `match` ne traite {}", what(Some(subject), &st)),
                )
                .at(subject.span)
                .label(t!("this value", "cette valeur"))
                .help(t!(
                    "add a case for this value, or a last case `_ -> ...` that handles everything else",
                    "ajoutez un cas pour cette valeur, ou un dernier cas `_ -> ...` qui traite tout le reste"
                )),
            );
            return Ty::Never;
        }
        self.diags.extend(dead);
        let pats: Vec<&Pattern> = arms.iter().filter(|a| a.guard.is_none()).map(|a| &a.pat).collect();
        let missing = self.missing(&pats);
        if !missing.is_empty() {
            let list: Vec<String> = missing.iter().map(|m| format!("`{m}`")).collect();
            self.diags.push(
                Diagnostic::warning("W002", t!("this `match` does not handle every case", "ce `match` ne traite pas tous les cas"))
                    .at(Span { end: subject.span.end, ..e.span })
                    .note(t!("not handled: {}", "non traité : {}", list.join(", ")))
                    .help(t!(
                        "add the missing cases, like `{} -> ...`, or a last case `_ -> ...`",
                        "ajoutez les cas manquants, comme `{} -> ...`, ou un dernier cas `_ -> ...`",
                        missing[0]
                    )),
            );
        }
        if st == Ty::Never {
            Ty::Never
        } else {
            result
        }
    }

    // ----- patterns ----------------------------------------------------

    fn pattern(&mut self, p: &Pattern) {
        match &p.kind {
            PatKind::Ctor(name, args) => {
                match self.ctors.get(name).cloned() {
                    None => {
                        let n = name.name();
                        let mut d = Diagnostic::error("E003", t!("unknown constructor `{n}`", "constructeur inconnu `{n}`"))
                            .at(p.span)
                            .label(t!("not defined", "non défini"));
                        if let Some(h) = suggest::alias_hint(&n) {
                            d = d.help(h);
                        } else if let Some(best) =
                            suggest::closest(&n, self.ctors.keys().map(|k| k.name()).collect::<Vec<_>>().iter().map(|s| &**s))
                        {
                            d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
                        } else {
                            d = d.help(t!(
                                "constructors are defined with `type`: `type Name = {n}(field) | Other`",
                                "les constructeurs se définissent avec `type` : `type Nom = {n}(champ) | Autre`"
                            ));
                        }
                        self.diags.push(d);
                    }
                    Some(meta) if meta.arity != args.len() => {
                        let blanks = vec!["_"; meta.arity].join(", ");
                        let example = if meta.arity == 0 { name.to_string() } else { format!("{name}({blanks})") };
                        self.diags.push(
                            Diagnostic::error(
                                "E004",
                                t!(
                                    "`{name}` has {} field(s), but this pattern gives {}",
                                    "`{name}` a {} champ(s), mais ce motif en donne {}",
                                    meta.arity,
                                    args.len()
                                ),
                            )
                            .at(p.span)
                            .help(t!("write it like `{example}`", "écrivez-le comme `{example}`")),
                        );
                    }
                    _ => {}
                }
                for a in args {
                    self.pattern(a);
                }
            }
            PatKind::List(ps, _) => ps.iter().for_each(|x| self.pattern(x)),
            PatKind::Record(fs) => {
                for (k, x) in fs {
                    if matches!(&x.kind, PatKind::Bind(s) if *s.name() == **k) {
                        self.field_binds.insert(x.span);
                    }
                    self.pattern(x);
                }
            }
            PatKind::Or(alts) => {
                let mut reported = false;
                for a in alts {
                    self.pattern(a);
                    let mut b = Vec::new();
                    a.bindings(&mut b);
                    if let (Some((s, sp)), false) = (b.first(), reported) {
                        reported = true;
                        self.diags.push(
                            Diagnostic::error(
                                "E001",
                                t!(
                                    "patterns joined with `|` cannot give names (here `{s}`)",
                                    "des motifs reliés par `|` ne peuvent pas donner de noms (ici `{s}`)"
                                ),
                            )
                            .at(*sp)
                            .help(t!("write separate cases instead", "écrivez plutôt des cas séparés")),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    /// The types of the names a pattern gives, in the order of
    /// `Pattern::bindings`, when it receives a value of type `t`. A name
    /// that can never be given gets `Never`.
    fn binding_types(&self, p: &Pattern, t: &Ty, out: &mut Vec<Ty>) {
        match &p.kind {
            PatKind::Bind(_) => out.push(t.clone()),
            PatKind::Ctor(name, ps) => {
                let fields = self.ctor_fields(*name, ps.len(), t);
                for (q, f) in ps.iter().zip(fields) {
                    self.binding_types(q, &f, out);
                }
            }
            PatKind::List(ps, rest) => {
                let (elem, whole) = match t {
                    Ty::List(e) => ((**e).clone(), t.clone()),
                    t if t.never(LIST) => (Ty::Never, Ty::Never),
                    Ty::Never => (Ty::Never, Ty::Never),
                    _ => (Ty::Any, Ty::list(Ty::Any)),
                };
                for q in ps {
                    self.binding_types(q, &elem, out);
                }
                if let Some(Some(_)) = rest {
                    out.push(whole);
                }
            }
            PatKind::Record(fs) => {
                for (k, q) in fs {
                    let ft = match t {
                        Ty::Record(r) => match r.get(k) {
                            Some(ft) => ft.clone(),
                            None if r.exact => Ty::Never,
                            None => Ty::Any,
                        },
                        t if t.never(RECORD) => Ty::Never,
                        Ty::Never => Ty::Never,
                        _ => Ty::Any,
                    };
                    self.binding_types(q, &ft, out);
                }
            }
            // Alternatives may not bind names; like `Pattern::bindings`, look at the first one.
            PatKind::Or(ps) => ps.iter().take(1).for_each(|q| self.binding_types(q, t, out)),
            _ => {}
        }
    }

    /// The types of the fields of a constructor pattern `name(...)`, for a
    /// value of type `t`.
    fn ctor_fields(&self, name: Sym, n: usize, t: &Ty) -> Vec<Ty> {
        match t {
            Ty::Data(d) if self.in_type(d.name, name) => match d.fields_of(&name.name(), n) {
                Some(fs) if fs.len() == n => fs,
                Some(_) => vec![Ty::Any; n],
                None => vec![Ty::Never; n],
            },
            Ty::Any => vec![Ty::Any; n],
            _ => vec![Ty::Never; n],
        }
    }

    /// Is `ctor` one of the constructors of the type `type_name`?
    fn in_type(&self, type_name: Sym, ctor: Sym) -> bool {
        match self.types.get(&type_name) {
            Some(cs) => cs.contains(&ctor),
            None => true,
        }
    }

    /// Can a value of type `t` match the pattern? Only false when surely not.
    /// `precise`: also use which constructors the value may have been built
    /// with (a `Circle(1.0)` never matches `Rect(w, h)`); otherwise only the
    /// type matters.
    fn can_match(&self, p: &Pattern, t: &Ty, precise: bool) -> bool {
        match (&p.kind, t) {
            (_, Ty::Any | Ty::Never) | (PatKind::Wildcard | PatKind::Bind(_), _) => true,
            (PatKind::Or(alts), _) => alts.iter().any(|a| self.can_match(a, t, precise)),
            (PatKind::Unit, _) => !t.never(UNIT),
            (PatKind::Bool(_), _) => !t.never(BOOL),
            // `1` matches `1.0`, and `1.0` matches `1`.
            (PatKind::Int(_) | PatKind::Float(_), _) => !t.never(NUMBER),
            (PatKind::Text(_), _) => !t.never(TEXT),
            (PatKind::Ctor(name, args), _) => {
                // An unknown constructor, or a wrong number of fields, is already an error.
                if self.ctors.get(name).is_none_or(|m| m.arity != args.len()) {
                    return true;
                }
                let Ty::Data(d) = t else { return false };
                if !self.in_type(d.name, *name) {
                    return false;
                }
                let fields = match &d.ctors {
                    None => return true,
                    Some(cs) => match cs.iter().find(|(c, _)| c == name) {
                        Some((_, fs)) => fs.clone(),
                        None if precise => return false,
                        None => return true,
                    },
                };
                fields.len() != args.len() || args.iter().zip(&fields).all(|(a, f)| self.can_match(a, f, precise))
            }
            // An empty list matches `[]` and `[..rest]`, whatever its elements.
            (PatKind::List(ps, _), Ty::List(e)) => ps.iter().all(|q| self.can_match(q, e, precise)),
            (PatKind::List(..), _) => false,
            (PatKind::Record(fs), Ty::Record(r)) => fs.iter().all(|(k, q)| match r.get(k) {
                Some(ft) => self.can_match(q, ft, precise),
                None => !r.exact,
            }),
            (PatKind::Record(_), _) => false,
        }
    }

    /// Does the pattern accept every value of type `t`?
    fn catches_all(&self, p: &Pattern, t: &Ty) -> bool {
        match &p.kind {
            PatKind::Wildcard | PatKind::Bind(_) => true,
            PatKind::Or(alts) => alts.iter().any(|a| self.catches_all(a, t)),
            PatKind::Unit => *t == Ty::Unit,
            // The only constructor of the type of the value.
            PatKind::Ctor(name, args) => match t {
                Ty::Data(d) => {
                    self.types.get(&d.name).is_some_and(|cs| cs.as_slice() == [*name]) && args.iter().all(|a| self.catches_all(a, &Ty::Any))
                }
                _ => false,
            },
            PatKind::Record(fs) => match t {
                Ty::Record(r) => fs.iter().all(|(k, q)| r.get(k).is_some_and(|ft| self.catches_all(q, ft))),
                _ => false,
            },
            _ => false,
        }
    }

    /// Does every value that matches `p` also match `q`? Answers false when
    /// not sure.
    fn covers(&self, q: &Pattern, p: &Pattern) -> bool {
        match (&q.kind, &p.kind) {
            (PatKind::Wildcard | PatKind::Bind(_), _) => true,
            (_, PatKind::Or(alts)) => alts.iter().all(|a| self.covers(q, a)),
            (PatKind::Or(alts), _) => alts.iter().any(|a| self.covers(a, p)),
            (PatKind::Unit, PatKind::Unit) => true,
            (PatKind::Bool(a), PatKind::Bool(b)) => a == b,
            (PatKind::Int(a), PatKind::Int(b)) => a == b,
            (PatKind::Float(a), PatKind::Float(b)) => a == b,
            (PatKind::Text(a), PatKind::Text(b)) => a == b,
            (PatKind::Ctor(a, qs), PatKind::Ctor(b, ps)) => {
                a == b && qs.len() == ps.len() && qs.iter().zip(ps).all(|(x, y)| self.covers(x, y))
            }
            (PatKind::List(qs, qrest), PatKind::List(ps, prest)) => {
                let lengths = match (qrest, prest) {
                    (None, None) => qs.len() == ps.len(),
                    (Some(_), _) => ps.len() >= qs.len(),
                    (None, Some(_)) => false,
                };
                lengths && qs.iter().zip(ps).all(|(x, y)| self.covers(x, y))
            }
            (PatKind::Record(qf), PatKind::Record(pf)) => qf.iter().all(|(k, x)| pf.iter().any(|(j, y)| j == k && self.covers(x, y))),
            _ => false,
        }
    }

    /// Does the pattern accept every value it may receive?
    fn irrefutable(&self, p: &Pattern) -> bool {
        match &p.kind {
            PatKind::Wildcard | PatKind::Bind(_) => true,
            PatKind::Ctor(name, args) => {
                let single = self.ctors.get(name).and_then(|m| self.types.get(&m.type_name)).is_some_and(|v| v.len() == 1);
                single && args.iter().all(|a| self.irrefutable(a))
            }
            PatKind::Or(alts) => alts.iter().any(|a| self.irrefutable(a)),
            // A record pattern needs a record with these fields; `()` needs `()`.
            _ => false,
        }
    }

    /// Cases that none of the patterns handle. When the answer is not clear
    /// (patterns too intricate for this simple analysis), nothing is reported:
    /// a warning must never be wrong.
    fn missing(&self, pats: &[&Pattern]) -> Vec<String> {
        let mut flat: Vec<&Pattern> = Vec::new();
        for p in pats {
            match &p.kind {
                PatKind::Or(alts) => flat.extend(alts.iter()),
                _ => flat.push(p),
            }
        }
        if flat.iter().any(|p| self.irrefutable(p)) {
            return Vec::new();
        }
        let Some(first) = flat.first() else { return vec!["_".to_string()] };
        match &first.kind {
            PatKind::Ctor(name, _) => {
                let Some(meta) = self.ctors.get(name) else { return Vec::new() };
                let Some(variants) = self.types.get(&meta.type_name) else { return Vec::new() };
                let mut out = Vec::new();
                for v in variants {
                    let arity = self.ctors.get(v).map(|m| m.arity).unwrap_or(0);
                    let rows: Vec<&[Pattern]> = flat
                        .iter()
                        .filter_map(|p| match &p.kind {
                            PatKind::Ctor(n, args) if n == v && args.len() == arity => Some(args.as_slice()),
                            _ => None,
                        })
                        .collect();
                    if rows.is_empty() {
                        out.push(if arity == 0 { v.to_string() } else { format!("{v}({})", vec!["_"; arity].join(", ")) });
                        continue;
                    }
                    if rows.iter().any(|args| args.iter().all(|a| self.irrefutable(a))) {
                        continue;
                    }
                    // Look inside when only one field is ever refutable: `Some(Some(x))`, `Some(None)`.
                    let refutable: Vec<usize> = (0..arity).filter(|&j| rows.iter().any(|a| !self.irrefutable(&a[j]))).collect();
                    if let [j] = refutable.as_slice() {
                        let inner: Vec<&Pattern> = rows.iter().map(|a| &a[*j]).collect();
                        for m in self.missing(&inner).into_iter().take(3) {
                            let mut fields = vec!["_".to_string(); arity];
                            fields[*j] = m;
                            out.push(format!("{v}({})", fields.join(", ")));
                        }
                    }
                }
                out
            }
            PatKind::Bool(_) => [true, false]
                .into_iter()
                .filter(|b| !flat.iter().any(|p| matches!(p.kind, PatKind::Bool(x) if x == *b)))
                .map(|b| b.to_string())
                .collect(),
            PatKind::List(..) => {
                let mut exact: HashSet<usize> = HashSet::new();
                let mut min_rest: Option<usize> = None;
                let mut unclear = false;
                for p in &flat {
                    if let PatKind::List(elems, rest) = &p.kind {
                        if !elems.iter().all(|e| self.irrefutable(e)) {
                            unclear = true;
                            continue;
                        }
                        match rest {
                            None => {
                                exact.insert(elems.len());
                            }
                            Some(_) => min_rest = Some(min_rest.map_or(elems.len(), |m| m.min(elems.len()))),
                        }
                    }
                }
                let shape = |n: usize| format!("[{}]", vec!["_"; n].join(", "));
                let missing: Vec<String> = match min_rest {
                    Some(m) => (0..m).filter(|n| !exact.contains(n)).map(shape).collect(),
                    None => {
                        let k = exact.iter().max().map_or(0, |m| m + 1);
                        if k == 0 {
                            vec!["[..]".to_string()]
                        } else {
                            vec![format!("[{}, ..]", vec!["_"; k].join(", "))]
                        }
                    }
                };
                if unclear {
                    Vec::new()
                } else {
                    missing
                }
            }
            PatKind::Int(_) | PatKind::Float(_) | PatKind::Text(_) => vec!["_".to_string()],
            _ => Vec::new(),
        }
    }
}

/// The real name of a built-in function (it may be used under another name in the REPL).
fn builtin_name(g: &GlobalInfo) -> Option<&'static str> {
    let name = g.sig.split('(').next()?;
    crate::builtins::lookup(name).filter(|b| b.sig == g.sig).map(|b| b.name)
}

fn usage(name: &str) -> String {
    match crate::builtins::lookup(name) {
        Some(b) => t!("usage: `{}`", "utilisation : `{}`", b.sig),
        None => String::new(),
    }
}

fn pattern_help() -> String {
    t!(
        "use `match` to handle the different possible shapes of a value",
        "utilisez `match` pour traiter les différentes formes possibles d'une valeur"
    )
}

/// The value of an expression written as a literal (`36`, `"age"`).
fn literal(e: &Expr) -> Option<Value> {
    Some(match &e.kind {
        ExprKind::Int(n) => Value::Int(*n),
        ExprKind::Float(f) => Value::Float(*f),
        ExprKind::Text(s) => Value::Text(s.clone()),
        ExprKind::Bool(b) => Value::Bool(*b),
        ExprKind::Unit => Value::Unit,
        _ => return None,
    })
}

/// How a value is shown in a message: like at run time for a value written
/// in the code (`Int 36`), or else by its type.
fn what(e: Option<&Expr>, t: &Ty) -> String {
    match e.and_then(literal) {
        Some(v) => v.describe(),
        None => t!("a value of type {}", "une valeur de type {}", t.name()),
    }
}

/// An operand in the label of an operator error: `Int 36`, `Text`, or `…` when unknown.
fn short(e: Option<&Expr>, t: &Ty) -> String {
    match e.and_then(literal) {
        Some(v) => v.describe(),
        None if t.is_known() => t.name(),
        None => "…".into(),
    }
}

/// Can two values of these types ever be compared with `<`?
fn comparable(lt: &Ty, rt: &Ty) -> bool {
    let (lk, rk) = (lt.kinds(), rt.kinds());
    let both = |k| lk & k != 0 && rk & k != 0;
    if both(NUMBER) || both(TEXT) || both(BOOL) || both(UNIT) || both(LIST) {
        return true;
    }
    let data = OPTION | RESULT | DATA;
    if !both(data) {
        return false;
    }
    match (lt, rt) {
        // Two variants compare only when they are of the same type and built
        // with the same constructor.
        (Ty::Data(a), Ty::Data(b)) => {
            a.name == b.name
                && match (&a.ctors, &b.ctors) {
                    (Some(x), Some(y)) => x.iter().any(|(c, _)| y.iter().any(|(d, _)| c == d)),
                    _ => true,
                }
        }
        _ => true,
    }
}

/// Can a value of type `lt` ever be equal to a value of type `rt`?
fn may_be_equal(lt: &Ty, rt: &Ty) -> bool {
    let (lk, rk) = (lt.kinds(), rt.kinds());
    if lk & NUMBER != 0 && rk & NUMBER != 0 {
        return true;
    }
    if lk & rk & (UNIT | BOOL | TEXT | LIST | RECORD | FUNC) != 0 {
        return true;
    }
    match (lt, rt) {
        (Ty::Data(a), Ty::Data(b)) => a.name == b.name,
        _ => false,
    }
}

/// The top-level functions, each one after the functions it calls (unless
/// they call each other), so that their types are found in few rounds.
fn callees_first(items: &[Item]) -> Vec<Rc<FnDef>> {
    let defs: Vec<&Rc<FnDef>> = items
        .iter()
        .filter_map(|it| match it {
            Item::Stmt(Stmt::Fn(def)) => Some(def),
            _ => None,
        })
        .collect();
    let index: HashMap<Sym, usize> = defs.iter().enumerate().filter_map(|(i, d)| d.name.map(|n| (n, i))).collect();
    let calls: Vec<Vec<usize>> = defs
        .iter()
        .map(|d| {
            let mut names = Vec::new();
            for c in &d.clauses {
                if let Some(g) = &c.guard {
                    names_in_expr(g, &mut names);
                }
                names_in_expr(&c.body, &mut names);
            }
            names.iter().filter_map(|n| index.get(n).copied()).collect()
        })
        .collect();
    // Depth-first, without recursion: a long chain of functions must not overflow the stack.
    let mut seen = vec![false; defs.len()];
    let mut out = Vec::with_capacity(defs.len());
    for start in 0..defs.len() {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        let mut stack = vec![(start, 0)];
        while let Some((f, next)) = stack.pop() {
            match calls[f].get(next) {
                Some(&g) => {
                    stack.push((f, next + 1));
                    if !seen[g] {
                        seen[g] = true;
                        stack.push((g, 0));
                    }
                }
                None => out.push(defs[f].clone()),
            }
        }
    }
    out
}

/// Collects the names used in an expression.
fn names_in_expr(e: &Expr, out: &mut Vec<Sym>) {
    match &e.kind {
        ExprKind::Var(s) => out.push(*s),
        ExprKind::Template(parts) => parts.iter().for_each(|p| {
            if let TemplatePart::Expr(x) = p {
                names_in_expr(x, out)
            }
        }),
        ExprKind::List(items) => items.iter().for_each(|it| match it {
            ListItem::One(x) | ListItem::Spread(x) => names_in_expr(x, out),
        }),
        ExprKind::Record(items) => items.iter().for_each(|it| match it {
            RecordItem::Field(_, _, x) | RecordItem::Spread(x) => names_in_expr(x, out),
        }),
        ExprKind::Lambda(def) => def.clauses.iter().for_each(|c| {
            if let Some(g) = &c.guard {
                names_in_expr(g, out);
            }
            names_in_expr(&c.body, out)
        }),
        ExprKind::Call(f, args) => {
            names_in_expr(f, out);
            args.iter().for_each(|a| names_in_expr(a, out));
        }
        ExprKind::Method(r, name, args, _) => {
            out.push(*name);
            names_in_expr(r, out);
            args.iter().for_each(|a| names_in_expr(a, out));
        }
        ExprKind::Field(x, _, _) | ExprKind::Unary(_, x) | ExprKind::Try(x) => names_in_expr(x, out),
        ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => {
            names_in_expr(a, out);
            names_in_expr(b, out);
        }
        ExprKind::If(c, a, b) => {
            names_in_expr(c, out);
            names_in_expr(a, out);
            names_in_expr(b, out);
        }
        ExprKind::Match(s, arms) => {
            names_in_expr(s, out);
            for arm in arms {
                if let Some(g) = &arm.guard {
                    names_in_expr(g, out);
                }
                names_in_expr(&arm.body, out);
            }
        }
        ExprKind::Block(stmts) => stmts.iter().for_each(|s| match s {
            Stmt::Let(_, x) | Stmt::Expr(x) => names_in_expr(x, out),
            Stmt::Fn(def) => def.clauses.iter().for_each(|c| names_in_expr(&c.body, out)),
        }),
        _ => {}
    }
}

/// Collects the field names written in record literals (`{name: ...}`).
fn fields_in_stmt(s: &Stmt, out: &mut HashSet<Rc<str>>) {
    match s {
        Stmt::Let(_, e) | Stmt::Expr(e) => fields_in_expr(e, out),
        Stmt::Fn(def) => def.clauses.iter().for_each(|c| fields_in_expr(&c.body, out)),
    }
}

fn fields_in_expr(e: &Expr, out: &mut HashSet<Rc<str>>) {
    match &e.kind {
        ExprKind::Record(items) => {
            for it in items {
                match it {
                    RecordItem::Field(k, _, x) => {
                        out.insert(k.clone());
                        fields_in_expr(x, out);
                    }
                    RecordItem::Spread(x) => fields_in_expr(x, out),
                }
            }
        }
        ExprKind::Template(parts) => parts.iter().for_each(|p| {
            if let TemplatePart::Expr(x) = p {
                fields_in_expr(x, out)
            }
        }),
        ExprKind::List(items) => items.iter().for_each(|it| match it {
            ListItem::One(x) | ListItem::Spread(x) => fields_in_expr(x, out),
        }),
        ExprKind::Lambda(def) => def.clauses.iter().for_each(|c| fields_in_expr(&c.body, out)),
        ExprKind::Call(f, args) => {
            fields_in_expr(f, out);
            args.iter().for_each(|a| fields_in_expr(a, out));
        }
        ExprKind::Method(r, _, args, _) => {
            fields_in_expr(r, out);
            args.iter().for_each(|a| fields_in_expr(a, out));
        }
        ExprKind::Field(x, _, _) | ExprKind::Unary(_, x) | ExprKind::Try(x) => fields_in_expr(x, out),
        ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => {
            fields_in_expr(a, out);
            fields_in_expr(b, out);
        }
        ExprKind::If(c, a, b) => {
            fields_in_expr(c, out);
            fields_in_expr(a, out);
            fields_in_expr(b, out);
        }
        ExprKind::Match(s, arms) => {
            fields_in_expr(s, out);
            for arm in arms {
                if let Some(g) = &arm.guard {
                    fields_in_expr(g, out);
                }
                fields_in_expr(&arm.body, out);
            }
        }
        ExprKind::Block(stmts) => stmts.iter().for_each(|s| fields_in_stmt(s, out)),
        _ => {}
    }
}

/// Can evaluating this expression do anything besides computing a value
/// (call a function, or leave the function with `?`)?
fn has_effect(e: &Expr) -> bool {
    match &e.kind {
        ExprKind::Call(..) | ExprKind::Method(..) | ExprKind::Try(_) => true,
        ExprKind::Template(parts) => parts.iter().any(|p| matches!(p, TemplatePart::Expr(x) if has_effect(x))),
        ExprKind::List(items) => items.iter().any(|it| match it {
            ListItem::One(x) | ListItem::Spread(x) => has_effect(x),
        }),
        ExprKind::Record(items) => items.iter().any(|it| match it {
            RecordItem::Field(_, _, x) | RecordItem::Spread(x) => has_effect(x),
        }),
        ExprKind::Field(x, _, _) | ExprKind::Unary(_, x) => has_effect(x),
        ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => has_effect(a) || has_effect(b),
        ExprKind::If(c, a, b) => has_effect(c) || has_effect(a) || has_effect(b),
        ExprKind::Match(s, arms) => has_effect(s) || arms.iter().any(|a| has_effect(&a.body) || a.guard.as_ref().is_some_and(has_effect)),
        ExprKind::Block(stmts) => stmts.iter().any(|s| match s {
            Stmt::Let(_, x) | Stmt::Expr(x) => has_effect(x),
            Stmt::Fn(_) => false,
        }),
        _ => false,
    }
}

/// Does the expression start with a minus sign (`- tax`, `-3 * x`)?
fn starts_with_minus(e: &Expr) -> bool {
    match &e.kind {
        ExprKind::Unary(UnOp::Neg, _) => true,
        ExprKind::Int(n) => *n < 0,
        ExprKind::Float(f) => *f < 0.0,
        ExprKind::Binary(_, l, _, _) | ExprKind::Range(l, _) | ExprKind::Index(l, _) | ExprKind::Field(l, _, _) => starts_with_minus(l),
        _ => false,
    }
}

/// An error for an expression statement whose value is thrown away.
fn unused_value(e: &Expr) -> Diagnostic {
    let d = Diagnostic::error(
        "E024",
        t!("this line computes a value but does nothing with it", "cette ligne calcule une valeur mais n'en fait rien"),
    )
    .at(e.span)
    .label(t!("this value is lost", "cette valeur est perdue"));
    if starts_with_minus(e) {
        d.help(t!(
            "a line that starts with `-` is a new statement. To continue the line above,\nend that line with `-` instead:\n  total = price -\n    discount",
            "une ligne qui commence par `-` est une nouvelle instruction. Pour continuer la ligne\nprécédente, terminez plutôt cette ligne par `-` :\n  total = prix -\n    remise"
        ))
    } else {
        d.help(t!(
            "to keep the value, give it a name: `result = ...`; to show it: `print(...)`.\nValues never change: `x + 1` does not change `x`.",
            "pour garder la valeur, donnez-lui un nom : `resultat = ...` ; pour l'afficher : `print(...)`.\nLes valeurs ne changent jamais : `x + 1` ne modifie pas `x`."
        ))
    }
}
