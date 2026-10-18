//! The syntax tree of a Rythme program.

use std::rc::Rc;

use crate::source::Span;
use crate::symbol::Sym;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    /// How many expressions are nested inside each other here, this one
    /// included (1 for a name or a number). The parser refuses code that is
    /// too deep, so that the passes that walk the tree never run out of stack.
    pub depth: u32,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Expr {
        let depth = kind.inner_depth().saturating_add(1);
        Expr { kind, span, depth }
    }
}

impl ExprKind {
    /// The depth of the deepest expression directly inside this one.
    fn inner_depth(&self) -> u32 {
        fn deepest<'a>(exprs: impl Iterator<Item = &'a Expr>) -> u32 {
            exprs.map(|e| e.depth).max().unwrap_or(0)
        }
        fn clauses(def: &FnDef) -> u32 {
            deepest(def.clauses.iter().flat_map(|c| std::iter::once(&c.body).chain(c.guard.as_ref())))
        }
        match self {
            ExprKind::Unit
            | ExprKind::Bool(_)
            | ExprKind::Int(_)
            | ExprKind::Float(_)
            | ExprKind::Text(_)
            | ExprKind::Var(_)
            | ExprKind::Symbol(_)
            | ExprKind::Ctor(_) => 0,
            ExprKind::Template(parts) => deepest(parts.iter().filter_map(|p| match p {
                TemplatePart::Expr(e) => Some(e),
                TemplatePart::Lit(_) => None,
            })),
            ExprKind::List(items) => deepest(items.iter().map(|it| match it {
                ListItem::One(e) | ListItem::Spread(e) => e,
            })),
            ExprKind::Record(items) => deepest(items.iter().map(|it| match it {
                RecordItem::Field(_, _, e) | RecordItem::Spread(e) => e,
            })),
            ExprKind::Lambda(def) => clauses(def),
            ExprKind::Call(f, args) | ExprKind::Method(f, _, args, _) => f.depth.max(deepest(args.iter())),
            ExprKind::Field(x, _, _) | ExprKind::Unary(_, x) | ExprKind::Try(x) => x.depth,
            ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => a.depth.max(b.depth),
            ExprKind::If(c, a, b) => c.depth.max(a.depth).max(b.depth),
            ExprKind::Match(subject, arms) => {
                subject.depth.max(deepest(arms.iter().flat_map(|a| std::iter::once(&a.body).chain(a.guard.as_ref()))))
            }
            ExprKind::Block(stmts) => stmts
                .iter()
                .map(|s| match s {
                    Stmt::Let(_, e) | Stmt::Expr(e) => e.depth,
                    Stmt::Fn(def) => clauses(def),
                })
                .max()
                .unwrap_or(0),
        }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(Rc<str>),
    /// `:ok`: a symbol, a value that is only its name.
    Symbol(Sym),
    /// A text with `{expressions}` inside.
    Template(Vec<TemplatePart>),
    Var(Sym),
    /// An uppercase name: a constructor (`Some`, `Circle`) or a value like `None`.
    Ctor(Sym),
    List(Vec<ListItem>),
    Record(Vec<RecordItem>),
    Lambda(Rc<FnDef>),
    Call(Box<Expr>, Vec<Expr>),
    /// `value.name(args)`: calls the field `name` of a record, or `name(value, args)`.
    Method(Box<Expr>, Sym, Vec<Expr>, Span),
    Field(Box<Expr>, Rc<str>, Span),
    Index(Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>, Span),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Vec<Arm>),
    Block(Vec<Stmt>),
    /// `expr?`
    Try(Box<Expr>),
    Range(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum TemplatePart {
    Lit(Rc<str>),
    Expr(Expr),
}

#[derive(Debug)]
pub enum ListItem {
    One(Expr),
    Spread(Expr),
}

#[derive(Debug)]
pub enum RecordItem {
    Field(Rc<str>, Span, Expr),
    Spread(Expr),
}

#[derive(Debug)]
pub struct Arm {
    pub pat: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

#[derive(Debug)]
pub enum Stmt {
    Let(Pattern, Expr),
    Expr(Expr),
    Fn(Rc<FnDef>),
}

#[derive(Debug)]
pub struct FnDef {
    pub name: Option<Sym>,
    /// One per definition (`fn fib(0) = 0`, `fn fib(n) = ...`), tried in order.
    pub clauses: Vec<Rc<Clause>>,
    pub arity: usize,
    pub doc: Option<Rc<str>>,
    pub span: Span,
}

impl FnDef {
    /// `name(a, b)`, built from the parameters of the first clause.
    pub fn signature(&self) -> String {
        let name = self.name.map(|n| n.name().to_string()).unwrap_or_else(|| "fn".to_string());
        let params: Vec<String> = self.clauses[0]
            .params
            .iter()
            .enumerate()
            .map(|(i, p)| match &p.kind {
                PatKind::Bind(s) => s.name().to_string(),
                _ => {
                    // Use a name from another clause if one has it.
                    for c in &self.clauses {
                        if let PatKind::Bind(s) = &c.params[i].kind {
                            return s.name().to_string();
                        }
                    }
                    format!("arg{}", i + 1)
                }
            })
            .collect();
        format!("{name}({})", params.join(", "))
    }
}

#[derive(Debug)]
pub struct Clause {
    pub params: Vec<Pattern>,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub span: Span,
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: Sym,
    pub variants: Vec<VariantDef>,
    pub doc: Option<Rc<str>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct VariantDef {
    pub name: Sym,
    pub fields: Vec<Sym>,
    pub span: Span,
}

#[derive(Debug)]
pub enum Item {
    Stmt(Stmt),
    Type(TypeDef),
    Test(Rc<str>, Rc<Expr>, Span),
    Use(Use),
}

/// `use geometry`, `use shapes/circle as circle`, `use "geometry.ry" as geo`
/// or `use "geometry.ry"`.
pub struct Use {
    /// The path of the file, as written; `use geometry` gives `geometry.ry`.
    pub path: Rc<str>,
    /// From `use` to the end of the path.
    pub span: Span,
    /// The path alone.
    pub path_span: Span,
    /// The name its definitions are used through (`geo.area`): written after
    /// `as`, or the name of the file in `use geometry`. Without it (`use
    /// "geometry.ry"`), they are used directly, by their own names.
    pub name: Option<(Sym, Span)>,
    /// Written without quotes: `use geometry`.
    pub short: bool,
}

/// What a `use` means, without how it is written: `use geometry` and `use
/// "geometry.ry" as geometry` are the same (the formatter relies on it).
impl std::fmt::Debug for Use {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Use").field("path", &self.path).field("name", &self.name.map(|(n, _)| n)).finish()
    }
}

#[derive(Debug)]
pub struct Pattern {
    pub kind: PatKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum PatKind {
    Wildcard,
    Bind(Sym),
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(Rc<str>),
    Symbol(Sym),
    Ctor(Sym, Vec<Pattern>),
    /// Elements, then the rest: `None` = no rest, `Some(None)` = `..`, `Some(Some(name))` = `..name`.
    List(Vec<Pattern>, Option<Option<(Sym, Span)>>),
    Record(Vec<(Rc<str>, Pattern)>),
    Or(Vec<Pattern>),
}

impl Pattern {
    /// Names bound by this pattern, with their position.
    pub fn bindings(&self, out: &mut Vec<(Sym, Span)>) {
        match &self.kind {
            PatKind::Bind(s) => out.push((*s, self.span)),
            PatKind::Ctor(_, ps) => ps.iter().for_each(|p| p.bindings(out)),
            PatKind::List(ps, rest) => {
                ps.iter().for_each(|p| p.bindings(out));
                if let Some(Some((s, sp))) = rest {
                    out.push((*s, *sp));
                }
            }
            PatKind::Record(fs) => fs.iter().for_each(|(_, p)| p.bindings(out)),
            // Alternatives may not bind names; looking at the first one avoids repeated errors.
            PatKind::Or(ps) => ps.iter().take(1).for_each(|p| p.bindings(out)),
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl BinOp {
    pub fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::IntDiv => "//",
            BinOp::Mod => "%",
            BinOp::Pow => "**",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
            BinOp::And => "and",
            BinOp::Or => "or",
        }
    }
}
