//! Files used with a name: `use geometry`, then `geometry.area(2)`.
//!
//! Right after parsing, `geometry.area` (a field or a method of the name of
//! a used file) becomes the single name `geometry.area`, so that the checker,
//! the compiler and the language server treat it as any other name of the
//! program. The checker refuses any other use of `geometry` (a local name
//! with the same name, or `geometry` alone), so that this is always right.

use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::*;
use crate::symbol::Sym;

/// The names given to used files by these items (`use geometry` gives
/// `geometry`), with the path of each file as written.
pub fn names(items: &[Item]) -> Vec<(Sym, &Use)> {
    items
        .iter()
        .filter_map(|it| match it {
            Item::Use(u) => u.name.map(|(n, _)| (n, u)),
            _ => None,
        })
        .collect()
}

/// The names a file defines at its top level (functions, values and
/// constructors), with where each one is defined.
pub fn top_names(items: &[Item]) -> Vec<(Sym, crate::source::Span)> {
    let mut out = Vec::new();
    for item in items {
        match item {
            Item::Stmt(Stmt::Fn(def)) => {
                if let Some(n) = def.name {
                    out.push((n, def.span));
                }
            }
            Item::Stmt(Stmt::Let(p, _)) => p.bindings(&mut out),
            Item::Type(t) => out.extend(t.variants.iter().map(|v| (v.name, v.span))),
            _ => {}
        }
    }
    out
}

/// Is this name private to its file? `_helper` is; `_` alone is not a name.
pub fn is_private(s: Sym) -> bool {
    let name = s.name();
    name.starts_with('_') && name.len() > 1
}

/// Rewrites `m.name` and `m.name(args)` into the name `m.name`, for each
/// name `m` of `modules`.
pub fn qualify(items: &mut [Item], modules: &HashSet<Sym>) {
    if modules.is_empty() {
        return;
    }
    let q = Qualify { modules };
    for item in items {
        match item {
            Item::Stmt(s) => q.stmt(s),
            Item::Test(_, body, _) => {
                if let Some(body) = Rc::get_mut(body) {
                    q.expr(body);
                }
            }
            Item::Type(_) | Item::Use(_) => {}
        }
    }
}

struct Qualify<'a> {
    modules: &'a HashSet<Sym>,
}

impl Qualify<'_> {
    fn stmt(&self, s: &mut Stmt) {
        match s {
            Stmt::Let(_, e) | Stmt::Expr(e) => self.expr(e),
            Stmt::Fn(def) => self.func(def),
        }
    }

    fn func(&self, def: &mut Rc<FnDef>) {
        let Some(def) = Rc::get_mut(def) else { return };
        for c in &mut def.clauses {
            let Some(c) = Rc::get_mut(c) else { continue };
            if let Some(g) = &mut c.guard {
                self.expr(g);
            }
            self.expr(&mut c.body);
        }
    }

    /// `m.name` written at `span`, as a single name.
    fn name(&self, m: Sym, name: &str, span: crate::source::Span) -> Expr {
        let s = Sym::new(&format!("{m}.{name}"));
        Expr::new(if s.is_ctor() { ExprKind::Ctor(s) } else { ExprKind::Var(s) }, span)
    }

    fn module(&self, e: &Expr) -> Option<Sym> {
        match &e.kind {
            ExprKind::Var(m) if self.modules.contains(m) => Some(*m),
            _ => None,
        }
    }

    fn expr(&self, e: &mut Expr) {
        match &mut e.kind {
            ExprKind::Method(recv, name, args, name_span) => {
                if let Some(m) = self.module(recv) {
                    let callee = self.name(m, &name.name(), recv.span.to(*name_span));
                    let args = std::mem::take(args);
                    e.kind = ExprKind::Call(Box::new(callee), args);
                    return self.expr(e);
                }
                self.expr(recv);
                args.iter_mut().for_each(|a| self.expr(a));
            }
            ExprKind::Field(obj, name, name_span) => {
                if let Some(m) = self.module(obj) {
                    let span = obj.span.to(*name_span);
                    *e = self.name(m, name, span);
                    return;
                }
                self.expr(obj);
            }
            ExprKind::Unit
            | ExprKind::Bool(_)
            | ExprKind::Int(_)
            | ExprKind::Float(_)
            | ExprKind::Text(_)
            | ExprKind::Var(_)
            | ExprKind::Ctor(_) => {}
            ExprKind::Template(parts) => {
                for p in parts {
                    if let TemplatePart::Expr(x) = p {
                        self.expr(x);
                    }
                }
            }
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
                        RecordItem::Field(_, _, x) | RecordItem::Spread(x) => self.expr(x),
                    }
                }
            }
            ExprKind::Lambda(def) => self.func(def),
            ExprKind::Call(f, args) => {
                self.expr(f);
                args.iter_mut().for_each(|a| self.expr(a));
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
                    if let Some(g) = &mut arm.guard {
                        self.expr(g);
                    }
                    self.expr(&mut arm.body);
                }
            }
            ExprKind::Block(stmts) => stmts.iter_mut().for_each(|s| self.stmt(s)),
        }
    }
}
