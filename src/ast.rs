//! The syntax tree of a Rythme program.

use std::rc::Rc;

use crate::bigint::BigInt;
use crate::source::Span;
use crate::symbol::Sym;

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    /// How many expressions are nested inside each other here, this one
    /// included (1 for a name or a number). The parser refuses code that is
    /// too deep, so that the passes that walk the tree never run out of stack.
    pub depth: u32,
    /// A name read as a value in `show [1, 2]` or `show -1`, where a call
    /// without parentheses may start, only because the code gives this name a
    /// value at this other place (see `parser::command_at`): the checker
    /// warns when the name is not a value here (W011).
    pub value_elsewhere: Option<Span>,
}

/// Without `value_elsewhere`, a hint for the checker: `rythme fmt` compares
/// the trees of a program before and after (written `xs[0]`, `xs [0]` is the
/// same index).
impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expr").field("kind", &self.kind).field("span", &self.span).field("depth", &self.depth).finish()
    }
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Expr {
        let depth = kind.inner_depth().saturating_add(1);
        Expr { kind, span, depth, value_elsewhere: None }
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
            | ExprKind::Big(_)
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
    /// A whole number too large for 64 bits.
    Big(Rc<BigInt>),
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
    /// The type written for parameter `i`, in the first definition that has one.
    pub fn param_type(&self, i: usize) -> Option<&Rc<TypeExpr>> {
        self.clauses.iter().find_map(|c| c.params.get(i).and_then(|p| p.ty.as_ref()))
    }

    /// The type written for the result, in the first definition that has one.
    pub fn ret_type(&self) -> Option<&Rc<TypeExpr>> {
        self.clauses.iter().find_map(|c| c.ret.as_ref())
    }

    /// `name(a, b)`, built from the parameters of the first clause; with
    /// the types written in the code, if any: `area(w: Int, h: Int) -> Int`.
    pub fn signature(&self) -> String {
        let name = self.name.map(|n| n.name().to_string()).unwrap_or_else(|| "fn".to_string());
        let params: Vec<String> = self.clauses[0]
            .params
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let shown = match &p.kind {
                    PatKind::Bind(s) => s.name().to_string(),
                    // Use a name from another clause if one has it.
                    _ => self
                        .clauses
                        .iter()
                        .find_map(|c| match &c.params[i].kind {
                            PatKind::Bind(s) => Some(s.name().to_string()),
                            _ => None,
                        })
                        .unwrap_or_else(|| format!("arg{}", i + 1)),
                };
                match self.param_type(i) {
                    Some(t) => format!("{shown}: {t}"),
                    None => shown,
                }
            })
            .collect();
        match self.ret_type() {
            Some(r) => format!("{name}({}) -> {r}", params.join(", ")),
            None => format!("{name}({})", params.join(", ")),
        }
    }
}

pub struct Clause {
    pub params: Vec<Pattern>,
    pub guard: Option<Expr>,
    pub body: Expr,
    /// From `fn` to the `=` (or the end of the line): the name, the
    /// parameters, the type of the result and the guard.
    pub span: Span,
    /// The type of the result, when it is written: `fn area(r) -> Float = ...`.
    pub ret: Option<Rc<TypeExpr>>,
}

/// Like a derived `Debug`, with the type of the result only when it is
/// written: the trees of programs without written types stay the same.
impl std::fmt::Debug for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Clause");
        d.field("params", &self.params).field("guard", &self.guard).field("body", &self.body).field("span", &self.span);
        if let Some(ret) = &self.ret {
            d.field("ret", ret);
        }
        d.finish()
    }
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: Sym,
    pub variants: Vec<VariantDef>,
    pub doc: Option<Rc<str>>,
    pub span: Span,
}

pub struct VariantDef {
    pub name: Sym,
    pub fields: Vec<Sym>,
    pub span: Span,
    /// The type written for each field (`Circle(radius: Float)`), if any.
    pub types: Vec<Option<Rc<TypeExpr>>>,
}

/// Like a derived `Debug`, with the types of the fields only when some are
/// written: the trees of programs without written types stay the same.
impl std::fmt::Debug for VariantDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("VariantDef");
        d.field("name", &self.name).field("fields", &self.fields).field("span", &self.span);
        if self.types.iter().any(Option::is_some) {
            d.field("types", &self.types);
        }
        d.finish()
    }
}

impl VariantDef {
    /// `Circle(radius)`, or `Circle(radius: Float)` with the types written.
    pub fn signature(&self) -> String {
        if self.fields.is_empty() {
            return self.name.to_string();
        }
        let fields: Vec<String> = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| match self.types.get(i).and_then(Option::as_ref) {
                Some(t) => format!("{f}: {t}"),
                None => f.to_string(),
            })
            .collect();
        format!("{}({})", self.name, fields.join(", "))
    }
}

/// A type written in the code, after `:` or `->`: `Int`, `List(Text)`,
/// `[Int]`, `{name: Text}`, `fn(Int) -> Bool`, `Shape`, `geo.Shape`, or a
/// type variable, `a`. It only tells the checker what is meant: running the
/// program never looks at it.
#[derive(Clone, Debug)]
pub struct TypeExpr {
    pub kind: TypeKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum TypeKind {
    /// `Int`, `List(Int)`, `Dict(Text, Int)`, `Shape`, `geo.Shape`: a name,
    /// with the types it takes.
    Named(Sym, Vec<TypeExpr>),
    /// `a`: any type, the same one everywhere in a definition.
    Var(Sym),
    /// `[Int]`: the same as `List(Int)`.
    List(Box<TypeExpr>),
    /// `{name: Text, age: Int}`: a record with at least these fields.
    Record(Vec<(Rc<str>, TypeExpr)>),
    /// `fn(Int, Int) -> Int`.
    Func(Vec<TypeExpr>, Box<TypeExpr>),
}

/// As `rythme fmt` writes it: `List(Int)`, `{name: Text}`, `fn(Int) -> Int`.
impl std::fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = |ts: &[TypeExpr]| ts.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ");
        match &self.kind {
            TypeKind::Named(n, args) if args.is_empty() => write!(f, "{n}"),
            TypeKind::Named(n, args) => write!(f, "{n}({})", list(args)),
            TypeKind::Var(v) => write!(f, "{v}"),
            TypeKind::List(t) => write!(f, "[{t}]"),
            TypeKind::Record(fields) => {
                let fs: Vec<String> = fields.iter().map(|(k, t)| format!("{k}: {t}")).collect();
                write!(f, "{{{}}}", fs.join(", "))
            }
            TypeKind::Func(params, ret) => write!(f, "fn({}) -> {ret}", list(params)),
        }
    }
}

impl TypeExpr {
    /// Calls `f` on this type and on every type inside it.
    pub fn each(&self, f: &mut dyn FnMut(&TypeExpr)) {
        f(self);
        match &self.kind {
            TypeKind::Named(_, args) => args.iter().for_each(|a| a.each(f)),
            TypeKind::Var(_) => {}
            TypeKind::List(t) => t.each(f),
            TypeKind::Record(fields) => fields.iter().for_each(|(_, t)| t.each(f)),
            TypeKind::Func(params, ret) => {
                params.iter().for_each(|p| p.each(f));
                ret.each(f);
            }
        }
    }
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

#[derive(Clone)]
pub struct Pattern {
    pub kind: PatKind,
    pub span: Span,
    /// The type written after it, for a parameter or a name given a value:
    /// `fn area(w: Int)`, `total: Float = 0.0`. Only the checker reads it.
    pub ty: Option<Rc<TypeExpr>>,
}

/// Like a derived `Debug`, with the written type only when there is one: the
/// trees of programs without written types stay the same.
impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Pattern");
        d.field("kind", &self.kind).field("span", &self.span);
        if let Some(t) = &self.ty {
            d.field("ty", t);
        }
        d.finish()
    }
}

#[derive(Clone, Debug)]
pub enum PatKind {
    Wildcard,
    Bind(Sym),
    Unit,
    Bool(bool),
    Int(i64),
    /// A whole number too large for 64 bits.
    Big(Rc<BigInt>),
    Float(f64),
    Text(Rc<str>),
    Symbol(Sym),
    /// `1..9`, `0.0..0.5`: a number from the first to the second, both included.
    Range(Num, Num),
    /// `"Mr " + name`: a text that starts with this text; the rest matches
    /// the pattern, a name or `_`.
    Prefix(Rc<str>, Box<Pattern>),
    Ctor(Sym, Vec<Pattern>),
    /// Elements, then the rest: `None` = no rest, `Some(None)` = `..`, `Some(Some(name))` = `..name`.
    List(Vec<Pattern>, Option<Option<(Sym, Span)>>),
    Record(Vec<(Rc<str>, Pattern)>),
    Or(Vec<Pattern>),
}

impl Pattern {
    /// A pattern without a written type.
    pub fn new(kind: PatKind, span: Span) -> Pattern {
        Pattern { kind, span, ty: None }
    }

    /// Names bound by this pattern, with their position.
    pub fn bindings(&self, out: &mut Vec<(Sym, Span)>) {
        match &self.kind {
            PatKind::Bind(s) => out.push((*s, self.span)),
            PatKind::Ctor(_, ps) => ps.iter().for_each(|p| p.bindings(out)),
            PatKind::Prefix(_, rest) => rest.bindings(out),
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

/// A bound of a range pattern.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Num {
    Int(i64),
    Float(f64),
}

impl Num {
    /// Compares two numbers exactly, an Int with a Float too.
    pub fn compare(self, other: Num) -> Option<std::cmp::Ordering> {
        use crate::value::cmp_int_float;
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Some(a.cmp(&b)),
            (Num::Float(a), Num::Float(b)) => a.partial_cmp(&b),
            (Num::Int(a), Num::Float(b)) => cmp_int_float(a, b),
            (Num::Float(a), Num::Int(b)) => cmp_int_float(b, a).map(std::cmp::Ordering::reverse),
        }
    }
}

/// As written in the code: `5`, `-2.5`.
impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Int(n) => write!(f, "{n}"),
            Num::Float(x) => f.write_str(&crate::value::fmt_float(*x)),
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
