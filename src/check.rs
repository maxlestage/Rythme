//! Checks a program before running it, to report as many mistakes as possible
//! at once: unknown names, wrong numbers of arguments, values defined twice,
//! forgotten cases in a `match`, unused variables...

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::ast::*;
use crate::diagnostic::Diagnostic;
use crate::interp::{arity_error, Interp};
use crate::parser::immutable_help;
use crate::source::Span;
use crate::suggest;
use crate::symbol::Sym;
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
}

/// What is known before checking a module: built-ins, and what earlier code defined.
#[derive(Clone, Default)]
pub struct Knowledge {
    pub globals: HashMap<Sym, GlobalInfo>,
    pub ctors: HashMap<Sym, CtorMeta>,
    pub types: HashMap<Sym, Vec<Sym>>,
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
                k.ctors.insert(c.name, CtorMeta { type_name: *t, arity: c.fields.len() });
            }
        }
        k
    }

    /// Adds the definitions of a module, without running it.
    pub fn add_items(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::Stmt(Stmt::Fn(def)) => {
                    if let Some(n) = def.name {
                        self.globals.insert(n, func_info(def));
                    }
                }
                Item::Stmt(Stmt::Let(p, _)) => {
                    let mut b = Vec::new();
                    p.bindings(&mut b);
                    for (s, sp) in b {
                        self.globals.insert(s, GlobalInfo { kind: GKind::Value, arity: None, sig: String::new(), span: Some(sp) });
                    }
                }
                Item::Type(t) => {
                    self.types.insert(t.name, t.variants.iter().map(|v| v.name).collect());
                    for v in &t.variants {
                        self.ctors.insert(v.name, CtorMeta { type_name: t.name, arity: v.fields.len() });
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
    };
    c.collect(items);
    c.walk(items);
    c.diags
}

struct Local {
    sym: Sym,
    span: Span,
    used: bool,
    warn: bool,
}

enum Res {
    Local,
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
}

impl<'a> Checker<'a> {
    // ----- top level ---------------------------------------------------

    fn collect(&mut self, items: &[Item]) {
        let mut first: HashMap<Sym, Span> = HashMap::new();
        let mut type_first: HashMap<Sym, Span> = HashMap::new();
        for item in items {
            match item {
                Item::Stmt(Stmt::Fn(def)) => {
                    if let Some(n) = def.name {
                        self.declare_global(n, def.span, func_info(def), &mut first);
                    }
                }
                Item::Stmt(Stmt::Let(p, _)) => {
                    let mut b = Vec::new();
                    p.bindings(&mut b);
                    for (s, sp) in b {
                        let info = GlobalInfo { kind: GKind::Value, arity: None, sig: String::new(), span: Some(sp) };
                        self.declare_global(s, sp, info, &mut first);
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
                        self.ctors.insert(v.name, CtorMeta { type_name: t.name, arity: v.fields.len() });
                    }
                    self.types.insert(t.name, t.variants.iter().map(|v| v.name).collect());
                }
                _ => {}
            }
        }
    }

    fn declare_global(&mut self, s: Sym, span: Span, info: GlobalInfo, first: &mut HashMap<Sym, Span>) {
        if !self.repl {
            if let Some(prev) = first.get(&s) {
                let mut d = Diagnostic::error("E011", t!("`{s}` is already defined", "`{s}` est déjà défini"))
                    .at(span)
                    .label(t!("defined again here", "défini à nouveau ici"))
                    .secondary(*prev, t!("first defined here", "défini une première fois ici"));
                d = if info.kind == GKind::Func {
                    d.help(t!(
                        "to give several definitions to a function (one per case), write them one right after the other",
                        "pour donner plusieurs définitions à une fonction (une par cas), écrivez-les les unes juste après les autres"
                    ))
                } else {
                    d.help(immutable_help())
                };
                self.diags.push(d);
                return;
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
                        return;
                    }
                }
            }
        }
        first.insert(s, span);
        self.own.insert(s, info);
    }

    fn walk(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::Stmt(Stmt::Fn(def)) => self.func(def),
                Item::Stmt(Stmt::Let(p, e)) => {
                    self.expr(e);
                    self.pattern(p);
                    let mut b = Vec::new();
                    p.bindings(&mut b);
                    self.check_duplicates(&b);
                    for (s, _) in b {
                        self.defined.insert(s);
                    }
                }
                Item::Stmt(Stmt::Expr(e)) => self.expr(e),
                Item::Test(_, body, _) => {
                    self.fn_depth += 1;
                    self.scopes.push(Vec::new());
                    match &body.kind {
                        ExprKind::Block(stmts) => self.stmts(stmts),
                        _ => self.expr(body),
                    }
                    self.pop_scope();
                    self.fn_depth -= 1;
                }
                Item::Type(_) | Item::Use(..) => {}
            }
        }
    }

    // ----- scopes ------------------------------------------------------

    fn declare_local(&mut self, s: Sym, span: Span, warn: bool) {
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
        scope.push(Local { sym: s, span, used: false, warn });
    }

    fn pop_scope(&mut self) {
        let Some(scope) = self.scopes.pop() else { return };
        for l in scope {
            let name = l.sym.name();
            if l.warn && !l.used && !name.starts_with('_') {
                self.diags.push(
                    Diagnostic::warning("W001", t!("`{name}` is never used", "`{name}` n'est jamais utilisé")).at(l.span).help(t!(
                        "remove it, or name it `_{name}` if this is on purpose",
                        "supprimez-le, ou nommez-le `_{name}` si c'est voulu"
                    )),
                );
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

    fn bind_pattern(&mut self, p: &Pattern, warn: bool) {
        self.pattern(p);
        let mut b = Vec::new();
        p.bindings(&mut b);
        let ok = self.check_duplicates(&b);
        for ((s, sp), ok) in b.into_iter().zip(ok) {
            if ok {
                self.declare_local(s, sp, warn);
            }
        }
    }

    fn resolve(&mut self, s: Sym, span: Span) -> Res {
        for scope in self.scopes.iter_mut().rev() {
            for l in scope.iter_mut().rev() {
                if l.sym == s {
                    l.used = true;
                    return Res::Local;
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
                return Res::Unknown;
            }
            return Res::Global(g.clone());
        }
        if let Some(g) = self.know.globals.get(&s) {
            return Res::Global(g.clone());
        }
        self.unknown(s, span);
        Res::Unknown
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

    fn func(&mut self, def: &FnDef) {
        self.fn_depth += 1;
        let mut catch_all: Option<Span> = None;
        let mut reported_unreachable = false;
        for clause in &def.clauses {
            if let (Some(prev), false) = (catch_all, reported_unreachable) {
                reported_unreachable = true;
                let name = def.name.map(|n| n.to_string()).unwrap_or_default();
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
            }
            if clause.guard.is_none() && clause.params.iter().all(|p| self.irrefutable(p)) && catch_all.is_none() {
                catch_all = Some(clause.span);
            }
            self.scopes.push(Vec::new());
            let mut b = Vec::new();
            for p in &clause.params {
                self.pattern(p);
                p.bindings(&mut b);
            }
            let ok = self.check_duplicates(&b);
            for ((s, sp), ok) in b.into_iter().zip(ok) {
                if ok {
                    self.declare_local(s, sp, false);
                }
            }
            if let Some(g) = &clause.guard {
                self.expr(g);
            }
            match &clause.body.kind {
                ExprKind::Block(stmts) => self.stmts(stmts),
                _ => self.expr(&clause.body),
            }
            self.pop_scope();
        }
        if def.arity == 1 && def.name.is_some() {
            let pats: Vec<&Pattern> = def.clauses.iter().filter(|c| c.guard.is_none()).map(|c| &c.params[0]).collect();
            let missing = self.missing(&pats);
            if !missing.is_empty() {
                let name = def.name.map(|n| n.to_string()).unwrap_or_default();
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
        self.fn_depth -= 1;
    }

    fn stmts(&mut self, stmts: &[Stmt]) {
        for s in stmts {
            match s {
                Stmt::Let(p, e) => {
                    self.expr(e);
                    self.bind_pattern(p, true);
                }
                Stmt::Expr(e) => self.expr(e),
                Stmt::Fn(def) => {
                    if let Some(n) = def.name {
                        self.declare_local(n, def.span, true);
                    }
                    self.func(def);
                }
            }
        }
    }

    // ----- expressions -------------------------------------------------

    fn expr(&mut self, e: &Expr) {
        match &e.kind {
            ExprKind::Unit | ExprKind::Bool(_) | ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Text(_) => {}
            ExprKind::Template(parts) => {
                for p in parts {
                    if let TemplatePart::Expr(x) = p {
                        self.expr(x);
                    }
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
                    return;
                }
                self.resolve(*s, e.span);
            }
            ExprKind::Ctor(s) => {
                self.resolve(*s, e.span);
            }
            ExprKind::List(items) => {
                for it in items {
                    match it {
                        ListItem::One(x) | ListItem::Spread(x) => self.expr(x),
                    }
                }
            }
            ExprKind::Record(items) => {
                let mut seen: HashMap<Rc<str>, Span> = HashMap::new();
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
                            self.expr(x);
                        }
                        RecordItem::Spread(x) => self.expr(x),
                    }
                }
            }
            ExprKind::Lambda(def) => self.func(def),
            ExprKind::Call(callee, args) => self.call(callee, args, e.span),
            ExprKind::Method(recv, name, args, _) => {
                self.expr(recv);
                for a in args {
                    self.expr(a);
                }
                self.method(*name, args.len() + 1, e.span);
            }
            ExprKind::Field(obj, _, _) => self.expr(obj),
            ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => {
                self.expr(a);
                self.expr(b);
            }
            ExprKind::Unary(_, x) => self.expr(x),
            ExprKind::If(c, a, b) => {
                self.expr(c);
                self.expr(a);
                self.expr(b);
            }
            ExprKind::Match(subject, arms) => self.match_expr(e, subject, arms),
            ExprKind::Block(stmts) => {
                self.scopes.push(Vec::new());
                self.stmts(stmts);
                self.pop_scope();
            }
            ExprKind::Try(x) => {
                self.expr(x);
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
            }
        }
    }

    fn call(&mut self, callee: &Expr, args: &[Expr], span: Span) {
        let res = match &callee.kind {
            ExprKind::Var(s) | ExprKind::Ctor(s) if &*s.name() != "_" => Some((*s, self.resolve(*s, callee.span))),
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
                None
            }
            _ => {
                self.expr(callee);
                None
            }
        };
        for a in args {
            self.expr(a);
        }
        if let Some((name, Res::Global(g))) = res {
            self.check_arity(name, &g, args.len(), span);
        }
    }

    fn method(&mut self, name: Sym, n: usize, span: Span) {
        for scope in self.scopes.iter_mut().rev() {
            for l in scope.iter_mut().rev() {
                if l.sym == name {
                    l.used = true;
                    return;
                }
            }
        }
        let g = self.own.get(&name).or_else(|| self.know.globals.get(&name)).cloned();
        if let Some(g) = g {
            if matches!(g.kind, GKind::Builtin | GKind::Func) {
                self.check_arity(name, &g, n, span);
            }
        }
    }

    fn check_arity(&mut self, name: Sym, g: &GlobalInfo, n: usize, span: Span) {
        match g.kind {
            GKind::CtorValue => self.diags.push(
                Diagnostic::error("E009", t!("`{name}` is a value, not a function", "`{name}` est une valeur, pas une fonction"))
                    .at(span)
                    .help(t!("write `{name}` without parentheses", "écrivez `{name}` sans parenthèses")),
            ),
            GKind::Value => {}
            _ => {
                if let Some((min, max)) = g.arity {
                    if n < min || n > max {
                        self.diags.push(*arity_error(&name.name(), &g.sig, min, max, n, span, g.span));
                    }
                }
            }
        }
    }

    fn match_expr(&mut self, e: &Expr, subject: &Expr, arms: &[Arm]) {
        self.expr(subject);
        let mut catch_all: Option<Span> = None;
        let mut reported = false;
        for arm in arms {
            if let (Some(prev), false) = (catch_all, reported) {
                reported = true;
                self.diags.push(
                    Diagnostic::warning("W004", t!("this case can never be reached", "ce cas ne peut jamais être atteint"))
                        .at(arm.pat.span)
                        .secondary(prev, t!("this case already handles everything", "ce cas traite déjà tout"))
                        .help(t!("cases are tried in order: put `_` last", "les cas sont essayés dans l'ordre : mettez `_` en dernier")),
                );
            }
            if arm.guard.is_none() && self.irrefutable(&arm.pat) && catch_all.is_none() {
                catch_all = Some(arm.pat.span);
            }
            self.scopes.push(Vec::new());
            self.bind_pattern(&arm.pat, true);
            if let Some(g) = &arm.guard {
                self.expr(g);
            }
            self.expr(&arm.body);
            self.pop_scope();
        }
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
            PatKind::Record(fs) => fs.iter().for_each(|(_, x)| self.pattern(x)),
            PatKind::Or(alts) => {
                for a in alts {
                    self.pattern(a);
                    let mut b = Vec::new();
                    a.bindings(&mut b);
                    if let Some((s, sp)) = b.first() {
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

    /// Does the pattern accept every value it may receive?
    fn irrefutable(&self, p: &Pattern) -> bool {
        match &p.kind {
            PatKind::Wildcard | PatKind::Bind(_) | PatKind::Unit => true,
            PatKind::Record(fs) => fs.iter().all(|(_, x)| self.irrefutable(x)),
            PatKind::Ctor(name, args) => {
                let single = self.ctors.get(name).and_then(|m| self.types.get(&m.type_name)).is_some_and(|v| v.len() == 1);
                single && args.iter().all(|a| self.irrefutable(a))
            }
            PatKind::Or(alts) => alts.iter().any(|a| self.irrefutable(a)),
            _ => false,
        }
    }

    /// Cases that none of the patterns handle (empty if all are handled or unknown).
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
                let covered: HashSet<Sym> = flat
                    .iter()
                    .filter_map(|p| match &p.kind {
                        PatKind::Ctor(n, args) if args.iter().all(|a| self.irrefutable(a)) => Some(*n),
                        _ => None,
                    })
                    .collect();
                variants
                    .iter()
                    .filter(|v| !covered.contains(v))
                    .map(|v| {
                        let arity = self.ctors.get(v).map(|m| m.arity).unwrap_or(0);
                        if arity == 0 {
                            v.to_string()
                        } else {
                            format!("{v}({})", vec!["_"; arity].join(", "))
                        }
                    })
                    .collect()
            }
            PatKind::Bool(_) => [true, false]
                .into_iter()
                .filter(|b| !flat.iter().any(|p| matches!(p.kind, PatKind::Bool(x) if x == *b)))
                .map(|b| b.to_string())
                .collect(),
            PatKind::List(..) => {
                let mut exact: HashSet<usize> = HashSet::new();
                let mut min_rest: Option<usize> = None;
                for p in &flat {
                    if let PatKind::List(elems, rest) = &p.kind {
                        if elems.iter().all(|e| self.irrefutable(e)) {
                            match rest {
                                None => {
                                    exact.insert(elems.len());
                                }
                                Some(_) => min_rest = Some(min_rest.map_or(elems.len(), |m| m.min(elems.len()))),
                            }
                        }
                    }
                }
                let shape = |n: usize| format!("[{}]", vec!["_"; n].join(", "));
                match min_rest {
                    Some(m) => (0..m).filter(|n| !exact.contains(n)).map(shape).collect(),
                    None => {
                        let k = exact.iter().max().map_or(0, |m| m + 1);
                        if k == 0 {
                            vec!["[..]".to_string()]
                        } else {
                            vec![format!("[{}, ..]", vec!["_"; k].join(", "))]
                        }
                    }
                }
            }
            PatKind::Int(_) | PatKind::Float(_) | PatKind::Text(_) => vec!["_".to_string()],
            _ => Vec::new(),
        }
    }
}
