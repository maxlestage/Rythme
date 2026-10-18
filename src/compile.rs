//! Prepares the syntax tree for the interpreter. Each local name becomes a
//! numbered slot in the frame of its function, or a value captured by a
//! closure, so that running a program never searches for a local name. Only
//! the names of the program (globals) are still looked up by name, when they
//! are used: they may be defined later.

use std::rc::Rc;

use crate::ast::{BinOp, Expr, ExprKind, FnDef, ListItem, PatKind, Pattern, RecordItem, Stmt as AstStmt, TemplatePart, UnOp};
use crate::source::Span;
use crate::symbol::Sym;
use crate::value::{Shape, Value};

/// An expression ready to run.
pub struct Node {
    pub kind: Kind,
    pub span: Span,
}

pub enum Kind {
    /// A literal value.
    Const(Value),
    /// A local name: a slot of the current frame.
    Local(u32),
    /// A local name of an enclosing function, captured by the closure.
    Captured(u32),
    /// A name of the program, with the local names visible here (for "did you mean").
    Global(Sym, Scope),
    Template(Vec<Part>),
    List(Vec<Item>),
    Record(Vec<Field>),
    /// `{a: x, b: y}`: different names and no `..`.
    NewRecord(Rc<Shape>, Vec<Node>),
    Lambda(Rc<FnDef>, Rc<FnCode>),
    Call(Box<Node>, Vec<Node>),
    Method(Box<Method>),
    Field(Box<Node>, Rc<str>, Span),
    Index(Box<Node>, Box<Node>),
    Unary(UnOp, Box<Node>),
    /// Every operator except `and` and `or`.
    Binary(BinOp, Box<Node>, Box<Node>, Span),
    /// `and` / `or`: the right side is only evaluated when needed.
    Logic(BinOp, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Match(Box<Node>, Vec<Arm>),
    /// The statements, then the last expression (the value of the block), if
    /// the block ends with one.
    Block(Vec<Stmt>, Option<Box<Node>>),
    Try(Box<Node>),
    Range(Box<Node>, Box<Node>),
}

pub enum Part {
    Lit(Rc<str>),
    Expr(Node),
}

pub enum Item {
    One(Node),
    Spread(Node),
}

pub enum Field {
    Field(Rc<str>, Node),
    Spread(Node),
}

/// `value.name(args)`: calls the field `name` of a record, or `name(value, args)`.
pub struct Method {
    pub recv: Node,
    pub name: Sym,
    /// The name as a record key.
    pub key: Rc<str>,
    /// The function `name`, used when the value has no such field.
    pub func: Node,
    pub args: Vec<Node>,
    pub name_span: Span,
}

pub struct Arm {
    pub pat: Pat,
    pub guard: Option<Node>,
    pub body: Node,
}

pub enum Stmt {
    /// `name = value`
    Set(u32, Node),
    /// `pattern = value`; the span of the pattern is for errors.
    Let(Pat, Node, Span),
    Expr(Node),
    /// A local function (`fn name(...)` or `name = fn(...)`), stored in a slot.
    Fn(u32, Rc<FnDef>, Rc<FnCode>),
}

/// A pattern whose names are slots.
pub enum Pat {
    /// Matches anything, binds nothing (`_`, or a parameter that is a plain name).
    Any,
    Bind(u32),
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(Rc<str>),
    Ctor(Sym, Vec<Pat>),
    /// Elements, then the rest: `None` = no rest, `Some(None)` = `..`, `Some(Some(slot))` = `..name`.
    List(Vec<Pat>, Option<Option<u32>>),
    Record(Vec<(Rc<str>, Pat)>),
    Or(Vec<Pat>),
}

/// A function ready to run. Its frame holds the arguments in the first slots,
/// then itself (for a local function that calls itself), then its other names.
pub struct FnCode {
    pub arity: usize,
    pub slots: usize,
    /// Where a local function sees itself, when it uses its own name.
    pub self_slot: Option<u32>,
    /// Where a new closure takes the values it captures, in the frame that creates it.
    pub captures: Vec<Capture>,
    pub clauses: Vec<ClauseCode>,
    /// One definition whose parameters are plain names, without guard: no
    /// pattern to try when it is called.
    pub simple: bool,
}

pub struct ClauseCode {
    /// The parameters that are not plain names: their position and pattern.
    pub tests: Vec<(usize, Pat)>,
    pub guard: Option<Node>,
    pub body: Node,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Capture {
    Local(u32),
    Captured(u32),
}

/// A top-level expression or test, with the size of its frame.
pub struct Code {
    pub node: Node,
    pub slots: usize,
}

/// The local names visible at some point of the program, innermost first.
#[derive(Clone, Default)]
pub struct Scope(Option<Rc<ScopeNode>>);

pub struct ScopeNode {
    sym: Sym,
    next: Scope,
}

impl Scope {
    fn push(&self, sym: Sym) -> Scope {
        Scope(Some(Rc::new(ScopeNode { sym, next: self.clone() })))
    }

    pub fn names(&self) -> Vec<Sym> {
        let mut out = Vec::new();
        let mut cur = &self.0;
        while let Some(node) = cur {
            out.push(node.sym);
            cur = &node.next.0;
        }
        out
    }
}

/// Prepares a function of the program (it captures nothing).
pub fn function(def: &FnDef) -> FnCode {
    Compiler::default().func(def, None)
}

/// Prepares an expression that runs outside of any function.
pub fn top(e: &Expr) -> Code {
    let mut c = Compiler::default();
    c.funcs.push(Func::new(0));
    let node = c.expr(e);
    let f = c.funcs.pop().unwrap();
    Code { node, slots: f.max as usize }
}

/// The same name of a record field everywhere in the code (finding a field
/// then often only compares addresses).
fn key(k: &str) -> Rc<str> {
    Sym::new(k).name()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Loc {
    Local(u32),
    Captured(u32),
}

/// A function being prepared.
struct Func {
    /// The local names visible now, innermost last.
    names: Vec<(Sym, Loc)>,
    next: u32,
    max: u32,
    self_slot: Option<u32>,
    self_used: bool,
    captures: Vec<Capture>,
}

impl Func {
    fn new(first_free: u32) -> Func {
        Func { names: Vec::new(), next: first_free, max: first_free, self_slot: None, self_used: false, captures: Vec::new() }
    }
}

#[derive(Default)]
struct Compiler {
    funcs: Vec<Func>,
    /// The same names as `funcs`, all functions together, for messages.
    scope: Scope,
}

impl Compiler {
    fn func(&mut self, def: &FnDef, self_name: Option<Sym>) -> FnCode {
        let arity = def.arity;
        let outer = self.scope.clone();
        let self_slot = self_name.map(|_| arity as u32);
        let first_free = arity as u32 + u32::from(self_slot.is_some());
        let mut f = Func::new(first_free);
        f.self_slot = self_slot;
        self.funcs.push(f);
        let mut clauses = Vec::with_capacity(def.clauses.len());
        for clause in &def.clauses {
            let f = self.funcs.last_mut().unwrap();
            f.names.clear();
            f.next = first_free;
            self.scope = outer.clone();
            // The function sees itself first: its parameters may hide it.
            if let (Some(name), Some(slot)) = (self_name, self_slot) {
                self.bind(name, slot);
            }
            let mut tests = Vec::new();
            for (i, p) in clause.params.iter().enumerate() {
                match &p.kind {
                    PatKind::Bind(s) => self.bind(*s, i as u32),
                    _ => match self.pattern(p) {
                        Pat::Any => {}
                        pat => tests.push((i, pat)),
                    },
                }
            }
            let guard = clause.guard.as_ref().map(|g| self.expr(g));
            let body = self.expr(&clause.body);
            clauses.push(ClauseCode { tests, guard, body });
        }
        let f = self.funcs.pop().unwrap();
        self.scope = outer;
        let simple = matches!(def.clauses.as_slice(), [c] if c.guard.is_none()
            && c.params.iter().all(|p| matches!(p.kind, PatKind::Bind(_) | PatKind::Wildcard)));
        FnCode {
            arity,
            slots: f.max as usize,
            self_slot: if f.self_used { self_slot } else { None },
            captures: f.captures,
            clauses,
            simple,
        }
    }

    fn current(&mut self) -> &mut Func {
        self.funcs.last_mut().unwrap()
    }

    fn new_slot(&mut self) -> u32 {
        let f = self.current();
        let slot = f.next;
        f.next += 1;
        f.max = f.max.max(f.next);
        slot
    }

    fn bind(&mut self, sym: Sym, slot: u32) {
        self.current().names.push((sym, Loc::Local(slot)));
        self.scope = self.scope.push(sym);
    }

    /// Where the names bound from now on will be forgotten.
    fn mark(&mut self) -> (usize, u32, Scope) {
        let f = self.current();
        (f.names.len(), f.next, self.scope.clone())
    }

    fn restore(&mut self, (names, next, scope): (usize, u32, Scope)) {
        let f = self.current();
        f.names.truncate(names);
        f.next = next;
        self.scope = scope;
    }

    /// Finds a local name in the function at `level`, capturing it from the
    /// enclosing functions if needed.
    fn resolve(&mut self, level: usize, sym: Sym) -> Option<Loc> {
        let f = &mut self.funcs[level];
        if let Some(&(_, loc)) = f.names.iter().rev().find(|(s, _)| *s == sym) {
            if f.self_slot.is_some_and(|slot| loc == Loc::Local(slot)) {
                f.self_used = true;
            }
            return Some(loc);
        }
        if level == 0 {
            return None;
        }
        let capture = match self.resolve(level - 1, sym)? {
            Loc::Local(slot) => Capture::Local(slot),
            Loc::Captured(i) => Capture::Captured(i),
        };
        let f = &mut self.funcs[level];
        let i = match f.captures.iter().position(|c| *c == capture) {
            Some(i) => i,
            None => {
                f.captures.push(capture);
                f.captures.len() - 1
            }
        };
        Some(Loc::Captured(i as u32))
    }

    fn name(&mut self, sym: Sym) -> Kind {
        match self.resolve(self.funcs.len() - 1, sym) {
            Some(Loc::Local(slot)) => Kind::Local(slot),
            Some(Loc::Captured(i)) => Kind::Captured(i),
            None => Kind::Global(sym, self.scope.clone()),
        }
    }

    fn pattern(&mut self, p: &Pattern) -> Pat {
        match &p.kind {
            PatKind::Wildcard => Pat::Any,
            PatKind::Bind(s) => {
                let slot = self.new_slot();
                self.bind(*s, slot);
                Pat::Bind(slot)
            }
            PatKind::Unit => Pat::Unit,
            PatKind::Bool(b) => Pat::Bool(*b),
            PatKind::Int(n) => Pat::Int(*n),
            PatKind::Float(f) => Pat::Float(*f),
            PatKind::Text(t) => Pat::Text(t.clone()),
            PatKind::Ctor(name, ps) => Pat::Ctor(*name, ps.iter().map(|p| self.pattern(p)).collect()),
            PatKind::List(ps, rest) => {
                let items = ps.iter().map(|p| self.pattern(p)).collect();
                let rest = rest.map(|r| {
                    r.map(|(s, _)| {
                        let slot = self.new_slot();
                        self.bind(s, slot);
                        slot
                    })
                });
                Pat::List(items, rest)
            }
            PatKind::Record(fields) => Pat::Record(fields.iter().map(|(k, p)| (key(k), self.pattern(p))).collect()),
            PatKind::Or(alts) => Pat::Or(alts.iter().map(|p| self.pattern(p)).collect()),
        }
    }

    fn boxed(&mut self, e: &Expr) -> Box<Node> {
        Box::new(self.expr(e))
    }

    fn exprs(&mut self, es: &[Expr]) -> Vec<Node> {
        es.iter().map(|e| self.expr(e)).collect()
    }

    fn expr(&mut self, e: &Expr) -> Node {
        let kind = match &e.kind {
            ExprKind::Unit => Kind::Const(Value::Unit),
            ExprKind::Bool(b) => Kind::Const(Value::Bool(*b)),
            ExprKind::Int(n) => Kind::Const(Value::Int(*n)),
            ExprKind::Float(f) => Kind::Const(Value::Float(*f)),
            ExprKind::Text(s) => Kind::Const(Value::Text(s.clone())),
            ExprKind::Template(parts) => Kind::Template(
                parts
                    .iter()
                    .map(|p| match p {
                        TemplatePart::Lit(l) => Part::Lit(l.clone()),
                        TemplatePart::Expr(x) => Part::Expr(self.expr(x)),
                    })
                    .collect(),
            ),
            ExprKind::Var(s) | ExprKind::Ctor(s) => self.name(*s),
            ExprKind::List(items) => Kind::List(
                items
                    .iter()
                    .map(|it| match it {
                        ListItem::One(x) => Item::One(self.expr(x)),
                        ListItem::Spread(x) => Item::Spread(self.expr(x)),
                    })
                    .collect(),
            ),
            ExprKind::Record(items) => {
                let mut keys: Vec<Rc<str>> = Vec::with_capacity(items.len());
                for it in items {
                    match it {
                        RecordItem::Field(k, ..) if !keys.contains(k) => keys.push(key(k)),
                        _ => break,
                    }
                }
                if keys.len() == items.len() {
                    let values = items
                        .iter()
                        .map(|it| match it {
                            RecordItem::Field(_, _, x) | RecordItem::Spread(x) => self.expr(x),
                        })
                        .collect();
                    Kind::NewRecord(Rc::new(Shape::new(keys)), values)
                } else {
                    Kind::Record(
                        items
                            .iter()
                            .map(|it| match it {
                                RecordItem::Field(k, _, x) => Field::Field(key(k), self.expr(x)),
                                RecordItem::Spread(x) => Field::Spread(self.expr(x)),
                            })
                            .collect(),
                    )
                }
            }
            ExprKind::Lambda(def) => Kind::Lambda(def.clone(), Rc::new(self.func(def, None))),
            ExprKind::Call(callee, args) => Kind::Call(self.boxed(callee), self.exprs(args)),
            ExprKind::Method(recv, name, args, name_span) => {
                let recv = self.expr(recv);
                let func = Node { kind: self.name(*name), span: *name_span };
                let args = self.exprs(args);
                Kind::Method(Box::new(Method { recv, name: *name, key: name.name(), func, args, name_span: *name_span }))
            }
            ExprKind::Field(obj, name, span) => Kind::Field(self.boxed(obj), key(name), *span),
            ExprKind::Index(obj, idx) => Kind::Index(self.boxed(obj), self.boxed(idx)),
            ExprKind::Unary(op, x) => Kind::Unary(*op, self.boxed(x)),
            ExprKind::Binary(op @ (BinOp::And | BinOp::Or), l, r, _) => Kind::Logic(*op, self.boxed(l), self.boxed(r)),
            ExprKind::Binary(op, l, r, span) => Kind::Binary(*op, self.boxed(l), self.boxed(r), *span),
            ExprKind::If(c, a, b) => Kind::If(self.boxed(c), self.boxed(a), self.boxed(b)),
            ExprKind::Match(subject, arms) => {
                let subject = self.boxed(subject);
                let arms = arms
                    .iter()
                    .map(|arm| {
                        let mark = self.mark();
                        let pat = self.pattern(&arm.pat);
                        let guard = arm.guard.as_ref().map(|g| self.expr(g));
                        let body = self.expr(&arm.body);
                        self.restore(mark);
                        Arm { pat, guard, body }
                    })
                    .collect();
                Kind::Match(subject, arms)
            }
            ExprKind::Block(stmts) => self.block(stmts),
            ExprKind::Try(x) => Kind::Try(self.boxed(x)),
            ExprKind::Range(a, b) => Kind::Range(self.boxed(a), self.boxed(b)),
        };
        Node { kind, span: e.span }
    }

    fn block(&mut self, stmts: &[AstStmt]) -> Kind {
        let mark = self.mark();
        let mut out = Vec::with_capacity(stmts.len());
        let mut last = None;
        for (i, s) in stmts.iter().enumerate() {
            match s {
                AstStmt::Let(p, x) => out.push(match (&p.kind, &x.kind) {
                    // `name = fn(...) = ...` can call itself by its name.
                    (PatKind::Bind(name), ExprKind::Lambda(def)) => {
                        let code = self.func(def, Some(*name));
                        let slot = self.new_slot();
                        self.bind(*name, slot);
                        Stmt::Fn(slot, def.clone(), Rc::new(code))
                    }
                    (PatKind::Bind(name), _) => {
                        let value = self.expr(x);
                        let slot = self.new_slot();
                        self.bind(*name, slot);
                        Stmt::Set(slot, value)
                    }
                    _ => {
                        let value = self.expr(x);
                        Stmt::Let(self.pattern(p), value, p.span)
                    }
                }),
                AstStmt::Expr(x) if i + 1 == stmts.len() => last = Some(self.boxed(x)),
                AstStmt::Expr(x) => out.push(Stmt::Expr(self.expr(x))),
                AstStmt::Fn(def) => {
                    let code = self.func(def, def.name);
                    if let Some(name) = def.name {
                        let slot = self.new_slot();
                        self.bind(name, slot);
                        out.push(Stmt::Fn(slot, def.clone(), Rc::new(code)));
                    }
                }
            }
        }
        self.restore(mark);
        Kind::Block(out, last)
    }
}
