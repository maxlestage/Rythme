//! Every name of a file: where it is defined and where it is used.
//!
//! Names are resolved with the same rules as the checker (`check.rs`): the
//! innermost local name first, then the definitions of the file, then those
//! of the files it uses, then the built-in functions.

use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

use crate::ast::*;
use crate::builtins;
use crate::source::Span;
use crate::symbol::Sym;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefKind {
    Function,
    Value,
    Param,
    Local,
    /// A name given by a case of a `match`.
    Case,
    Type,
    Ctor,
    Test,
}

/// A definition: its file (source id) and its place in that file's index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DefId {
    pub file: u32,
    pub idx: u32,
}

#[derive(Clone, Debug)]
pub struct Def {
    pub name: Rc<str>,
    pub kind: DefKind,
    /// The name, where it is defined.
    pub span: Span,
    /// The whole definition.
    pub full: Span,
    /// `fib(n)` for functions (local ones too) and constructors.
    pub sig: Option<String>,
    pub doc: Option<Rc<str>>,
    /// The heads of the definitions of a function: `fn fib(0)`, `fn fib(n)`.
    pub heads: Vec<Span>,
    /// The name again in the other definitions of a function (`fn fib(1)`...).
    pub names: Vec<Span>,
    /// The type of a constructor, the function of a parameter.
    pub owner: Option<Rc<str>>,
    pub top: bool,
    /// Where a local name can be used (byte offsets, both included).
    pub visible: (u32, u32),
    /// Written `{name}` in a record pattern: the name is also a field name.
    pub punned: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Def(DefId),
    Builtin(&'static str),
    Pi,
    /// `Some`, `None`, `Ok`, `Err`.
    Prelude(&'static str),
    Unknown,
}

/// A name written in the file.
#[derive(Clone, Debug)]
pub struct Occ {
    pub span: Span,
    pub name: Rc<str>,
    pub target: Target,
    /// The definition itself, not a use.
    pub def: bool,
    /// `{name}` in a record: the name is also the field name.
    pub punned: bool,
    /// `value.name(...)`: may also call a record field named `name`.
    pub method: bool,
}

#[derive(Default)]
pub struct Index {
    pub file: u32,
    pub defs: Vec<Def>,
    /// Sorted by position.
    pub occs: Vec<Occ>,
    /// The top-level names, in order: what files that `use` this one see.
    pub tops: Vec<(Sym, u32)>,
    /// Field names written in records, record patterns and `.field`.
    pub fields: BTreeSet<Rc<str>>,
    /// `use "file"`: the place and the path written.
    pub uses: Vec<(Span, Rc<str>)>,
}

pub const PRELUDE: [&str; 4] = ["Some", "None", "Ok", "Err"];

impl Index {
    /// The name written at a byte offset (a name ending right there counts too).
    pub fn occ_at(&self, offset: usize) -> Option<&Occ> {
        let o = offset as u32;
        self.occs.iter().find(|x| x.span.start <= o && o < x.span.end).or_else(|| self.occs.iter().find(|x| x.span.end == o))
    }

    pub fn def(&self, idx: u32) -> Option<&Def> {
        self.defs.get(idx as usize)
    }

    /// The local names that can be used at a byte offset, the innermost first.
    pub fn locals_at(&self, offset: usize) -> Vec<(u32, &Def)> {
        let o = offset as u32;
        let mut found: Vec<(u32, &Def)> = self
            .defs
            .iter()
            .enumerate()
            .filter(|(_, d)| !d.top && d.visible.0 <= o && o <= d.visible.1)
            .map(|(i, d)| (i as u32, d))
            .collect();
        found.sort_by_key(|(_, d)| std::cmp::Reverse(d.visible.0));
        let mut seen = std::collections::HashSet::new();
        found.retain(|(_, d)| seen.insert(d.name.clone()));
        found
    }
}

/// Builds the index of a file. `deps` holds the top-level names of the files
/// it uses, as the checker sees them (a later file wins).
pub fn build(items: &[Item], file: u32, text: &str, deps: &HashMap<Sym, DefId>) -> Index {
    let mut ix =
        Indexer { file, text, out: Index { file, ..Index::default() }, scopes: Vec::new(), own: HashMap::new(), deps, owner: None };
    ix.collect(items);
    ix.walk(items);
    let mut out = ix.out;
    out.occs.sort_by_key(|o| (o.span.start, o.span.end));
    out
}

struct Indexer<'a> {
    file: u32,
    text: &'a str,
    out: Index,
    scopes: Vec<Vec<(Sym, u32)>>,
    own: HashMap<Sym, u32>,
    deps: &'a HashMap<Sym, DefId>,
    /// The function whose parameters are being read.
    owner: Option<Rc<str>>,
}

impl Indexer<'_> {
    fn add(&mut self, def: Def) -> u32 {
        self.out.defs.push(def);
        (self.out.defs.len() - 1) as u32
    }

    fn def_occ(&mut self, name: Rc<str>, span: Span, idx: u32, punned: bool) {
        let target = Target::Def(DefId { file: self.file, idx });
        self.out.occs.push(Occ { span, name, target, def: true, punned, method: false });
    }

    fn new_def(name: Rc<str>, kind: DefKind, span: Span, full: Span, top: bool) -> Def {
        Def {
            name,
            kind,
            span,
            full,
            sig: None,
            doc: None,
            heads: Vec::new(),
            names: Vec::new(),
            owner: None,
            top,
            visible: (0, u32::MAX),
            punned: false,
        }
    }

    // ----- definitions -------------------------------------------------

    fn collect(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::Stmt(Stmt::Fn(def)) => {
                    if let Some(n) = def.name {
                        let idx = self.function(def, true, (0, u32::MAX));
                        self.own.entry(n).or_insert(idx);
                        self.out.tops.push((n, idx));
                    }
                }
                Item::Stmt(Stmt::Let(p, e)) => {
                    let mut binds = Vec::new();
                    self.binds(p, &mut binds);
                    for (s, sp, punned) in binds {
                        let mut d = Self::new_def(s.name(), DefKind::Value, sp, p.span.to(e.span), true);
                        d.punned = punned;
                        let idx = self.add(d);
                        self.def_occ(s.name(), sp, idx, punned);
                        self.own.entry(s).or_insert(idx);
                        self.out.tops.push((s, idx));
                    }
                }
                Item::Type(t) => {
                    let name = t.name.name();
                    let span = name_at_end(self.text, t.span, &name);
                    let full = t.variants.last().map_or(t.span, |v| t.span.to(v.span));
                    let mut d = Self::new_def(name.clone(), DefKind::Type, span, full, true);
                    d.doc = t.doc.clone();
                    let idx = self.add(d);
                    self.def_occ(name.clone(), span, idx, false);
                    for v in &t.variants {
                        let vname = v.name.name();
                        let vspan = name_at_start(self.text, v.span, &vname);
                        let mut d = Self::new_def(vname.clone(), DefKind::Ctor, vspan, v.span, true);
                        let fields: Vec<String> = v.fields.iter().map(|f| f.name().to_string()).collect();
                        d.sig = Some(if fields.is_empty() { vname.to_string() } else { format!("{vname}({})", fields.join(", ")) });
                        d.owner = Some(name.clone());
                        d.doc = t.doc.clone();
                        let idx = self.add(d);
                        self.def_occ(vname, vspan, idx, false);
                        self.own.entry(v.name).or_insert(idx);
                        self.out.tops.push((v.name, idx));
                    }
                }
                Item::Test(name, body, span) => {
                    self.add(Self::new_def(name.clone(), DefKind::Test, *span, span.to(body.span), true));
                }
                Item::Use(u) => self.out.uses.push((u.span, u.path.clone())),
                Item::Stmt(Stmt::Expr(_)) => {}
            }
        }
    }

    /// A named function: its definition, and the places where its name is written.
    fn function(&mut self, def: &FnDef, top: bool, visible: (u32, u32)) -> u32 {
        let name = def.name.map(|n| n.name()).unwrap_or_else(|| Rc::from("fn"));
        let first = def.clauses.first().map_or(def.span, |c| c.span);
        let last = def.clauses.last().map_or(def.span, |c| c.body.span);
        let mut d = Self::new_def(name.clone(), DefKind::Function, def.span, def.span.to(first).to(last), top);
        d.sig = Some(def.signature());
        d.doc = def.doc.clone();
        d.heads = def.clauses.iter().map(|c| c.span).collect();
        d.names = def.clauses.iter().skip(1).filter_map(|c| clause_name(self.text, c.span, &name)).collect();
        d.visible = visible;
        let names = d.names.clone();
        let idx = self.add(d);
        self.def_occ(name.clone(), def.span, idx, false);
        for sp in names {
            self.def_occ(name.clone(), sp, idx, false);
        }
        idx
    }

    /// The names bound by a pattern, like `Pattern::bindings`, and whether
    /// each one is written `{name}` in a record pattern.
    fn binds(&self, p: &Pattern, out: &mut Vec<(Sym, Span, bool)>) {
        match &p.kind {
            PatKind::Bind(s) => out.push((*s, p.span, false)),
            PatKind::Ctor(_, ps) => ps.iter().for_each(|x| self.binds(x, out)),
            PatKind::List(ps, rest) => {
                ps.iter().for_each(|x| self.binds(x, out));
                if let Some(Some((s, sp))) = rest {
                    out.push((*s, *sp, false));
                }
            }
            PatKind::Record(fs) => {
                for (k, x) in fs {
                    match &x.kind {
                        PatKind::Bind(s) if *s.name() == **k => out.push((*s, x.span, !after_colon(self.text, x.span))),
                        _ => self.binds(x, out),
                    }
                }
            }
            PatKind::Or(ps) => ps.iter().take(1).for_each(|x| self.binds(x, out)),
            _ => {}
        }
    }

    // ----- scopes ------------------------------------------------------

    fn local(&mut self, s: Sym, span: Span, kind: DefKind, visible: (u32, u32), full: Span, punned: bool) -> u32 {
        let mut d = Self::new_def(s.name(), kind, span, full, false);
        d.visible = visible;
        d.punned = punned;
        if kind == DefKind::Param {
            d.owner = self.owner.clone();
        }
        let idx = self.add(d);
        self.def_occ(s.name(), span, idx, punned);
        self.push_local(s, idx);
        idx
    }

    /// A name defined twice in the same scope is an error; the first one stays.
    fn push_local(&mut self, s: Sym, idx: u32) {
        if self.scopes.is_empty() {
            self.scopes.push(Vec::new());
        }
        let scope = self.scopes.last_mut().expect("a scope");
        if !scope.iter().any(|(n, _)| *n == s) {
            scope.push((s, idx));
        }
    }

    /// Declares the names of a pattern (the first of two equal names only, like the checker).
    fn bind_pattern(&mut self, p: &Pattern, kind: DefKind, visible: (u32, u32), full: Span) {
        self.pattern_refs(p);
        let mut binds = Vec::new();
        self.binds(p, &mut binds);
        let mut seen = Vec::new();
        for (s, sp, punned) in binds {
            if seen.contains(&s) {
                continue;
            }
            seen.push(s);
            self.local(s, sp, kind, visible, full, punned);
        }
    }

    fn resolve(&self, s: Sym) -> Target {
        for scope in self.scopes.iter().rev() {
            if let Some((_, idx)) = scope.iter().rev().find(|(n, _)| *n == s) {
                return Target::Def(DefId { file: self.file, idx: *idx });
            }
        }
        if let Some(idx) = self.own.get(&s) {
            return Target::Def(DefId { file: self.file, idx: *idx });
        }
        if let Some(id) = self.deps.get(&s) {
            return Target::Def(*id);
        }
        let name = s.name();
        if let Some(b) = builtins::lookup(&name) {
            return Target::Builtin(b.name);
        }
        if &*name == "pi" {
            return Target::Pi;
        }
        match PRELUDE.iter().find(|p| **p == &*name) {
            Some(p) => Target::Prelude(p),
            None => Target::Unknown,
        }
    }

    fn reference(&mut self, s: Sym, mut span: Span, punned: bool, method: bool) {
        let target = self.resolve(s);
        let mut name = s.name();
        // `geometry.area`: the name is `area`, written at the end.
        if let Some((_, own)) = s.split_module() {
            name = own.name();
            span.start = span.end.saturating_sub(name.len() as u32).max(span.start);
        }
        self.out.occs.push(Occ { span, name, target, def: false, punned, method });
    }

    // ----- walking -----------------------------------------------------

    fn walk(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::Stmt(Stmt::Fn(def)) => self.func(def),
                Item::Stmt(Stmt::Let(p, e)) => {
                    self.expr(e);
                    self.pattern_refs(p);
                }
                Item::Stmt(Stmt::Expr(e)) => self.expr(e),
                Item::Test(_, body, _) => {
                    self.scopes.push(Vec::new());
                    match &body.kind {
                        ExprKind::Block(stmts) => self.stmts(stmts, body.span.end),
                        _ => self.expr(body),
                    }
                    self.scopes.pop();
                }
                Item::Type(_) | Item::Use(..) => {}
            }
        }
    }

    fn func(&mut self, def: &FnDef) {
        let saved = std::mem::replace(&mut self.owner, def.name.map(|n| n.name()));
        for clause in &def.clauses {
            self.scopes.push(Vec::new());
            let visible = (clause.span.start, clause.body.span.end);
            for p in &clause.params {
                self.pattern_refs(p);
            }
            let mut binds = Vec::new();
            for p in &clause.params {
                self.binds(p, &mut binds);
            }
            let mut seen = Vec::new();
            for (s, sp, punned) in binds {
                if !seen.contains(&s) {
                    seen.push(s);
                    self.local(s, sp, DefKind::Param, visible, clause.span, punned);
                }
            }
            if let Some(g) = &clause.guard {
                self.expr(g);
            }
            match &clause.body.kind {
                ExprKind::Block(stmts) => self.stmts(stmts, clause.body.span.end),
                _ => self.expr(&clause.body),
            }
            self.scopes.pop();
        }
        self.owner = saved;
    }

    fn stmts(&mut self, stmts: &[Stmt], end: u32) {
        for (i, s) in stmts.iter().enumerate() {
            match s {
                Stmt::Let(p, e) => {
                    if let (PatKind::Bind(name), ExprKind::Lambda(lambda)) = (&p.kind, &e.kind) {
                        // `name = fn(...)` sees its own name, so that it can be recursive.
                        let idx = self.local(*name, p.span, DefKind::Local, (p.span.start, end), p.span.to(e.span), false);
                        let sig = lambda.signature().replacen("fn(", &format!("{name}("), 1);
                        self.out.defs[idx as usize].sig = Some(sig);
                        self.expr(e);
                    } else {
                        self.expr(e);
                        self.bind_pattern(p, DefKind::Local, (e.span.end, end), p.span.to(e.span));
                    }
                }
                Stmt::Expr(e) => self.expr(e),
                Stmt::Fn(def) => {
                    // Functions written one after the other see each other.
                    if i == 0 || !matches!(stmts[i - 1], Stmt::Fn(_)) {
                        let start = def.clauses.first().map_or(def.span.start, |c| c.span.start);
                        for d in stmts[i..].iter().map_while(|s| if let Stmt::Fn(d) = s { Some(d) } else { None }) {
                            if let Some(name) = d.name {
                                let idx = self.function(d, false, (start, end));
                                self.push_local(name, idx);
                            }
                        }
                    }
                    self.func(def);
                }
            }
        }
    }

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
                if &*s.name() != "_" {
                    self.reference(*s, e.span, false, false);
                }
            }
            ExprKind::Ctor(s) => self.reference(*s, e.span, false, false),
            ExprKind::List(items) => {
                for it in items {
                    match it {
                        ListItem::One(x) | ListItem::Spread(x) => self.expr(x),
                    }
                }
            }
            ExprKind::Record(items) => {
                for it in items {
                    match it {
                        RecordItem::Field(k, ks, x) => {
                            self.out.fields.insert(k.clone());
                            match &x.kind {
                                ExprKind::Var(s) if x.span == *ks => self.reference(*s, x.span, true, false),
                                _ => self.expr(x),
                            }
                        }
                        RecordItem::Spread(x) => self.expr(x),
                    }
                }
            }
            ExprKind::Lambda(def) => self.func(def),
            ExprKind::Call(callee, args) => {
                self.expr(callee);
                for a in args {
                    self.expr(a);
                }
            }
            ExprKind::Method(recv, name, args, name_span) => {
                self.expr(recv);
                for a in args {
                    self.expr(a);
                }
                self.reference(*name, *name_span, false, true);
            }
            ExprKind::Field(obj, name, _) => {
                self.out.fields.insert(name.clone());
                self.expr(obj);
            }
            ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => {
                self.expr(a);
                self.expr(b);
            }
            ExprKind::Unary(_, x) | ExprKind::Try(x) => self.expr(x),
            ExprKind::If(c, a, b) => {
                self.expr(c);
                self.expr(a);
                self.expr(b);
            }
            ExprKind::Match(subject, arms) => {
                self.expr(subject);
                for arm in arms {
                    self.scopes.push(Vec::new());
                    self.bind_pattern(&arm.pat, DefKind::Case, (arm.pat.span.start, arm.body.span.end), arm.pat.span);
                    if let Some(g) = &arm.guard {
                        self.expr(g);
                    }
                    self.expr(&arm.body);
                    self.scopes.pop();
                }
            }
            ExprKind::Block(stmts) => {
                self.scopes.push(Vec::new());
                self.stmts(stmts, e.span.end);
                self.scopes.pop();
            }
        }
    }

    /// The constructors written in a pattern.
    fn pattern_refs(&mut self, p: &Pattern) {
        match &p.kind {
            PatKind::Ctor(name, args) => {
                let span = name_at_start(self.text, p.span, &name.name());
                self.reference(*name, span, false, false);
                args.iter().for_each(|a| self.pattern_refs(a));
            }
            PatKind::List(ps, _) => ps.iter().for_each(|x| self.pattern_refs(x)),
            PatKind::Record(fs) => {
                for (k, x) in fs {
                    self.out.fields.insert(k.clone());
                    self.pattern_refs(x);
                }
            }
            PatKind::Or(alts) => alts.iter().for_each(|x| self.pattern_refs(x)),
            _ => {}
        }
    }
}

fn slice(text: &str, start: usize, end: usize) -> Option<&str> {
    text.get(start..end)
}

/// The span of `name` at the start of `span`, when the text there is indeed the name.
fn name_at_start(text: &str, span: Span, name: &str) -> Span {
    let start = span.start as usize;
    match slice(text, start, start + name.len()) {
        Some(s) if s == name => Span::new(span.file, start, start + name.len()),
        _ => span,
    }
}

/// The span of `name` at the end of `span`, when the text there is indeed the name.
fn name_at_end(text: &str, span: Span, name: &str) -> Span {
    let end = span.end as usize;
    match end.checked_sub(name.len()).and_then(|start| slice(text, start, end).map(|s| (start, s))) {
        Some((start, s)) if s == name => Span::new(span.file, start, end),
        _ => span,
    }
}

/// The name in `fn name(...)`, the head of a later definition of a function.
fn clause_name(text: &str, head: Span, name: &str) -> Option<Span> {
    let head_text = slice(text, head.start as usize, head.end as usize)?;
    let rest = head_text.strip_prefix("fn")?;
    let skipped = rest.len() - rest.trim_start().len();
    let start = head.start as usize + 2 + skipped;
    (slice(text, start, start + name.len())? == name).then(|| Span::new(head.file, start, start + name.len()))
}

/// Is the name preceded by `:` (`{age: years}`), rather than written alone (`{age}`)?
fn after_colon(text: &str, span: Span) -> bool {
    text.get(..span.start as usize).is_some_and(|before| before.trim_end().ends_with(':'))
}
