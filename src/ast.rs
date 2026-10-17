//! The syntax tree of a Rythme program.

use std::rc::Rc;

use crate::source::Span;
use crate::symbol::Sym;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Expr {
        Expr { kind, span }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(Rc<str>),
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
    Use(Rc<str>, Span),
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
