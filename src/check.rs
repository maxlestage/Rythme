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

/// The names of the types of Rythme's own values (see `type_of`).
const BUILTIN_TYPES: &[&str] = &["Unit", "Bool", "Int", "Float", "Text", "List", "Record", "Function", "Option", "Result"];

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
        field_binds: HashSet::new(),
        record_fields: HashSet::new(),
    };
    for item in items {
        match item {
            Item::Stmt(s) => fields_in_stmt(s, &mut c.record_fields),
            Item::Test(_, body, _) => fields_in_expr(body, &mut c.record_fields),
            _ => {}
        }
    }
    c.collect(items);
    c.walk(items);
    c.diags
}

struct Local {
    sym: Sym,
    span: Span,
    used: bool,
    warn: bool,
    /// For a local function: its number of parameters and signature.
    info: Option<GlobalInfo>,
}

enum Res {
    Local(Option<GlobalInfo>),
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
                    let name = t.name.name();
                    if BUILTIN_TYPES.contains(&&*name) {
                        self.diags.push(
                            Diagnostic::error("E011", t!("the type `{name}` is part of Rythme", "le type `{name}` fait partie de Rythme"))
                                .at(t.span)
                                .help(t!("give your type another name", "donnez un autre nom à votre type")),
                        );
                        continue;
                    }
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
                return;
            }
            if let Some(g) = self.know.globals.get(&s) {
                match g.kind {
                    GKind::Builtin => self.diags.push(
                        // `pi` is a value, not a function.
                        Diagnostic::warning("W003", match g.arity {
                            Some(_) => t!("`{s}` hides the built-in function `{s}`", "`{s}` masque la fonction intégrée `{s}`"),
                            None => t!("`{s}` hides the built-in value `{s}`", "`{s}` masque la valeur intégrée `{s}`"),
                        })
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
        self.declare_local_fn(s, span, warn, None);
    }

    fn declare_local_fn(&mut self, s: Sym, span: Span, warn: bool, info: Option<GlobalInfo>) {
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
        scope.push(Local { sym: s, span, used: false, warn, info });
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
                    return Res::Local(l.info.clone());
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
        if def.arity == 1 && def.name.is_some() && !self.repl {
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
        let n = stmts.len();
        for (i, s) in stmts.iter().enumerate() {
            match s {
                Stmt::Let(p, e) => {
                    if let (PatKind::Bind(name), ExprKind::Lambda(def)) = (&p.kind, &e.kind) {
                        // `name = fn(...)` sees its own name, so that it can be recursive.
                        let info = GlobalInfo { sig: def.signature().replacen("fn(", &format!("{name}("), 1), ..func_info(def) };
                        self.declare_local_fn(*name, p.span, true, Some(info));
                        self.expr(e);
                    } else {
                        self.expr(e);
                        self.bind_pattern(p, true);
                    }
                }
                Stmt::Expr(e) => {
                    self.expr(e);
                    // Only the last line of a block gives its value.
                    if i + 1 < n && !has_effect(e) {
                        self.diags.push(unused_value(e));
                    }
                }
                Stmt::Fn(def) => {
                    // Functions written one after the other see each other.
                    if i == 0 || !matches!(stmts[i - 1], Stmt::Fn(_)) {
                        for d in stmts[i..].iter().map_while(|s| if let Stmt::Fn(d) = s { Some(d) } else { None }) {
                            if let Some(name) = d.name {
                                self.declare_local_fn(name, d.span, true, Some(func_info(d)));
                            }
                        }
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
                self.method(*name, args.len() + 1, e.span, name_span_of(e));
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
                // In `x??`, the inner `?` is already reported.
                if self.fn_depth == 0 && !matches!(x.kind, ExprKind::Try(_)) {
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
                let d = Diagnostic::error("E009", t!("this value is not a function", "cette valeur n'est pas une fonction"))
                    .at(callee.span)
                    .label(t!("cannot be called", "ne peut pas être appelé"));
                let d = match &callee.kind {
                    ExprKind::Int(_) | ExprKind::Float(_) => d.help(t!(
                        "to multiply, write `*` explicitly: `2 * (x + 1)`",
                        "pour multiplier, écrivez `*` explicitement : `2 * (x + 1)`"
                    )),
                    ExprKind::Text(_) | ExprKind::Template(_) | ExprKind::List(_) => d.help(t!(
                        "to take one element, use brackets: `x[0]`; between two values, a `,` may be missing",
                        "pour prendre un élément, utilisez des crochets : `x[0]` ; entre deux valeurs, il manque peut-être une `,`"
                    )),
                    ExprKind::Record(_) => {
                        d.help(t!("to read a field, use a dot: `person.name`", "pour lire un champ, utilisez un point : `personne.nom`"))
                    }
                    _ => d,
                };
                self.diags.push(d);
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
        if let Some((name, Res::Global(g) | Res::Local(Some(g)))) = res {
            self.check_arity(name, &g, args.len(), span);
        }
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
            None => {
                let shown = name.name();
                let mut d =
                    Diagnostic::warning("W005", t!("there is no function named `{shown}`", "il n'existe pas de fonction nommée `{shown}`"))
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
                    .help(if missing[0] == "_" {
                        t!(
                            "add a last case `_ -> ...` for all the other values",
                            "ajoutez un dernier cas `_ -> ...` pour toutes les autres valeurs"
                        )
                    } else {
                        t!(
                            "add the missing cases, like `{} -> ...`, or a last case `_ -> ...`",
                            "ajoutez les cas manquants, comme `{} -> ...`, ou un dernier cas `_ -> ...`",
                            missing[0]
                        )
                    }),
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

/// Where the name is in `value.name(args)`.
fn name_span_of(e: &Expr) -> Span {
    match &e.kind {
        ExprKind::Method(_, _, _, span) => *span,
        _ => e.span,
    }
}
