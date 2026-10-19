//! Prepares the syntax tree for the interpreter. Each local name becomes a
//! numbered slot in the frame of its function, or a value captured by a
//! closure, so that running a program never searches for a local name. Only
//! the names of the program (globals) are still looked up by name, when they
//! are used: they may be defined later.

use std::rc::Rc;

use crate::ast::{BinOp, Expr, ExprKind, FnDef, ListItem, Num, PatKind, Pattern, RecordItem, Stmt as AstStmt, TemplatePart, UnOp};
use crate::source::Span;
use crate::symbol::{Sym, SymMap};
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
    /// Another function of the group of the running function (see `Sibling`).
    Sibling(u32),
    /// A name of the program, with the local names visible here (for "did you mean").
    Global(Sym, Scope),
    Template(Vec<Part>),
    List(Vec<Item>),
    Record(Vec<Field>),
    /// `{a: x, b: y}`: different names and no `..`.
    NewRecord(Rc<Shape>, Vec<Node>),
    Lambda(Rc<FnDef>, Rc<FnCode>),
    Call(Box<Node>, Vec<Node>),
    /// A call of another function of the group of the running function (or
    /// of itself): it runs with the same closure, without making one.
    SiblingCall(u32, Vec<Node>),
    Method(Box<Method>),
    /// `value.name`: the field `name` of a record or a variant, or else the
    /// function `name` called with the value (the last node).
    Field(Box<Node>, Rc<str>, Span, Box<Node>),
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

/// What the debugger needs to know at a place where it may pause.
pub struct StopInfo {
    /// The local names visible here, with the slot of their value in the
    /// frame, in the order they were given. The names captured by a closure
    /// are in `FnCode::capture_names`.
    pub names: Vec<(Sym, u32)>,
    /// How the top-level names of the file are stored (to evaluate an
    /// expression there).
    pub file_names: Names,
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
    /// Local functions written one after the other: the slot of each named
    /// one, with its position in the group.
    Group(Vec<(u32, u32)>, Rc<[Sibling]>),
    /// A place where the debugger may pause, shown at the span (only in code
    /// prepared for `rythme debug`: see `function_for`). A body or a branch
    /// that is not a block becomes a block that starts with it: the
    /// interpreter runs it with the other statements, and the rest of the
    /// code is the same as without a debugger.
    Stop(Rc<StopInfo>, Span),
}

/// One of several local functions written one after the other in a block:
/// they see each other, as functions do at the top level of a file. They
/// capture the same values, so that each one can make the others.
pub struct Sibling {
    pub def: Rc<FnDef>,
    pub code: Rc<FnCode>,
}

/// A pattern whose names are slots.
pub enum Pat {
    /// Matches anything, binds nothing: `_`.
    Any,
    Bind(u32),
    Unit,
    Bool(bool),
    Int(i64),
    Big(Rc<crate::bigint::BigInt>),
    Float(f64),
    Text(Rc<str>),
    Symbol(Sym),
    Range(Num, Num),
    Prefix(Rc<str>, Box<Pat>),
    /// The constructor as the interpreter stores it (`CtorInfo::id`).
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
    /// The names of the captured values, in the same order (for the debugger).
    pub capture_names: Vec<Sym>,
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
    /// A function of the group of the function that creates the closure.
    Sibling(u32),
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

impl Drop for Scope {
    /// Frees a long list of names one by one, not recursively.
    fn drop(&mut self) {
        let mut cur = self.0.take();
        while let Some(rc) = cur {
            cur = match Rc::try_unwrap(rc) {
                Ok(mut node) => node.next.0.take(),
                Err(_) => None,
            };
        }
    }
}

/// How the top-level names of a file are stored, when not under the name
/// written (see `Sym::stored`): its private names, the names of a file used
/// with a name, and `geo.area` for the file used as `geo`.
pub type Names = Rc<SymMap<Sym>>;

/// Prepares a function of the program (it captures nothing).
pub fn function(def: &FnDef, names: &Names) -> FnCode {
    function_for(def, names, false)
}

/// Like [`function`]; with `debug`, the code has the places where the
/// debugger may pause (`Stmt::Stop`). Without it, the code has none and runs
/// as fast as ever.
pub fn function_for(def: &FnDef, names: &Names, debug: bool) -> FnCode {
    Compiler { names: names.clone(), debug, ..Compiler::default() }.func(def, None, Vec::new(), Vec::new())
}

/// Prepares an expression that runs outside of any function.
pub fn top(e: &Expr, names: &Names) -> Code {
    top_for(e, names, false)
}

/// Like [`top`], with the places where the debugger may pause when `debug`.
pub fn top_for(e: &Expr, names: &Names, debug: bool) -> Code {
    let mut c = Compiler { names: names.clone(), debug, ..Compiler::default() };
    c.funcs.push(Func::new(0));
    let node = c.expr(e);
    let node = c.stop(node, e.span);
    let f = c.funcs.pop().unwrap();
    Code { node, slots: f.max as usize }
}

/// Prepares an expression that sees the local names `locals`, whose values
/// are in the first slots of its frame, in this order (the debugger's
/// `print`, in a paused function).
pub fn with_locals(e: &Expr, names: &Names, locals: &[Sym]) -> Code {
    let mut c = Compiler { names: names.clone(), ..Compiler::default() };
    c.funcs.push(Func::new(locals.len() as u32));
    for (i, &s) in locals.iter().enumerate() {
        c.bind(s, i as u32);
    }
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
    Sibling(u32),
}

/// A function being prepared.
struct Func {
    /// The local names visible now, innermost last.
    names: Vec<(Sym, Loc)>,
    /// The other functions of its group, seen behind its own names.
    siblings: Vec<(Sym, u32)>,
    next: u32,
    max: u32,
    self_slot: Option<u32>,
    self_used: bool,
    captures: Vec<Capture>,
    capture_names: Vec<Sym>,
}

impl Func {
    fn new(first_free: u32) -> Func {
        Func {
            names: Vec::new(),
            siblings: Vec::new(),
            next: first_free,
            max: first_free,
            self_slot: None,
            self_used: false,
            captures: Vec::new(),
            capture_names: Vec::new(),
        }
    }
}

#[derive(Default)]
struct Compiler {
    funcs: Vec<Func>,
    /// The same names as `funcs`, all functions together, for messages.
    scope: Scope,
    names: Names,
    /// Add the places where the debugger may pause.
    debug: bool,
}

impl Compiler {
    /// Prepares a function that captures at least `captures` and sees the
    /// functions of its group, `siblings`.
    fn func(&mut self, def: &FnDef, self_name: Option<Sym>, captures: Vec<(Capture, Sym)>, siblings: Vec<(Sym, u32)>) -> FnCode {
        let arity = def.arity;
        let outer = self.scope.clone();
        let self_slot = self_name.map(|_| arity as u32);
        let first_free = arity as u32 + u32::from(self_slot.is_some());
        let mut f = Func::new(first_free);
        f.self_slot = self_slot;
        (f.captures, f.capture_names) = captures.into_iter().unzip();
        f.siblings = siblings;
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
            let body = self.stop(body, clause.body.span);
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
            capture_names: f.capture_names,
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
        if let Some(&(_, i)) = f.siblings.iter().rev().find(|(s, _)| *s == sym) {
            return Some(Loc::Sibling(i));
        }
        if level == 0 {
            return None;
        }
        let capture = match self.resolve(level - 1, sym)? {
            Loc::Local(slot) => Capture::Local(slot),
            Loc::Captured(i) => Capture::Captured(i),
            Loc::Sibling(i) => Capture::Sibling(i),
        };
        let f = &mut self.funcs[level];
        let i = match f.captures.iter().position(|c| *c == capture) {
            Some(i) => i,
            None => {
                f.captures.push(capture);
                f.capture_names.push(sym);
                f.captures.len() - 1
            }
        };
        Some(Loc::Captured(i as u32))
    }

    fn name(&mut self, sym: Sym) -> Kind {
        match self.resolve(self.funcs.len() - 1, sym) {
            Some(Loc::Local(slot)) => Kind::Local(slot),
            Some(Loc::Captured(i)) => Kind::Captured(i),
            Some(Loc::Sibling(i)) => Kind::Sibling(i),
            None => Kind::Global(self.names.get(&sym).copied().unwrap_or(sym), self.scope.clone()),
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
            PatKind::Big(n) => Pat::Big(n.clone()),
            PatKind::Float(f) => Pat::Float(*f),
            PatKind::Text(t) => Pat::Text(t.clone()),
            PatKind::Symbol(s) => Pat::Symbol(*s),
            PatKind::Range(lo, hi) => Pat::Range(*lo, *hi),
            PatKind::Prefix(t, rest) => Pat::Prefix(t.clone(), Box::new(self.pattern(rest))),
            PatKind::Ctor(name, ps) => {
                Pat::Ctor(self.names.get(name).copied().unwrap_or(*name), ps.iter().map(|p| self.pattern(p)).collect())
            }
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
            ExprKind::Big(n) => Kind::Const(Value::Big(n.clone())),
            ExprKind::Float(f) => Kind::Const(Value::Float(*f)),
            ExprKind::Text(s) => Kind::Const(Value::Text(s.into())),
            ExprKind::Symbol(s) => Kind::Const(Value::Symbol(*s)),
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
            ExprKind::Lambda(def) => Kind::Lambda(def.clone(), Rc::new(self.func(def, None, Vec::new(), Vec::new()))),
            ExprKind::Call(callee, args) => {
                let callee = self.boxed(callee);
                match callee.kind {
                    Kind::Sibling(i) => Kind::SiblingCall(i, self.exprs(args)),
                    _ => Kind::Call(callee, self.exprs(args)),
                }
            }
            ExprKind::Method(recv, name, args, name_span) => {
                let recv = self.expr(recv);
                let func = Node { kind: self.name(*name), span: *name_span };
                let args = self.exprs(args);
                Kind::Method(Box::new(Method { recv, name: *name, key: name.name(), func, args, name_span: *name_span }))
            }
            ExprKind::Field(obj, name, span) => {
                let obj = self.boxed(obj);
                let func = Node { kind: self.name(Sym::new(name)), span: *span };
                Kind::Field(obj, key(name), *span, Box::new(func))
            }
            ExprKind::Index(obj, idx) => Kind::Index(self.boxed(obj), self.boxed(idx)),
            ExprKind::Unary(op, x) => Kind::Unary(*op, self.boxed(x)),
            ExprKind::Binary(op @ (BinOp::And | BinOp::Or), l, r, _) => Kind::Logic(*op, self.boxed(l), self.boxed(r)),
            ExprKind::Binary(op, l, r, span) => Kind::Binary(*op, self.boxed(l), self.boxed(r), *span),
            ExprKind::If(c, a, b) => {
                let c = self.boxed(c);
                let then = self.expr(a);
                let then = self.stop(then, a.span);
                let other = self.expr(b);
                let other = self.stop(other, b.span);
                Kind::If(c, Box::new(then), Box::new(other))
            }
            ExprKind::Match(subject, arms) => {
                let subject = self.boxed(subject);
                let arms = arms
                    .iter()
                    .map(|arm| {
                        let mark = self.mark();
                        let pat = self.pattern(&arm.pat);
                        let guard = arm.guard.as_ref().map(|g| self.expr(g));
                        let body = self.expr(&arm.body);
                        let body = self.stop(body, arm.body.span);
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

    /// With `debug`, a place where the debugger may pause before `node`
    /// runs, shown at `at` (a body, a branch): `{ stop; node }`. A block
    /// pauses at its own statements instead.
    fn stop(&self, node: Node, at: Span) -> Node {
        if !self.debug || matches!(node.kind, Kind::Block(..)) {
            return node;
        }
        let span = node.span;
        Node { kind: Kind::Block(vec![self.stop_here(at)], Some(Box::new(node))), span }
    }

    /// With `debug`, adds to the statements of a block a place where the
    /// debugger may pause before the next one.
    fn stop_before(&self, out: &mut Vec<Stmt>, at: Span) {
        if self.debug {
            out.push(self.stop_here(at));
        }
    }

    /// A place where the debugger may pause, with the local names visible now.
    fn stop_here(&self, at: Span) -> Stmt {
        let f = self.funcs.last().expect("a function being prepared");
        let mut names: Vec<(Sym, u32)> = Vec::new();
        for &(sym, loc) in &f.names {
            // The function itself is not one of its values.
            let Loc::Local(slot) = loc else { continue };
            if f.self_slot == Some(slot) {
                continue;
            }
            names.retain(|(s, _)| *s != sym);
            names.push((sym, slot));
        }
        Stmt::Stop(Rc::new(StopInfo { names, file_names: self.names.clone() }), at)
    }

    fn block(&mut self, stmts: &[AstStmt]) -> Kind {
        let mark = self.mark();
        let mut out = Vec::with_capacity(stmts.len());
        let mut last = None;
        for (i, s) in stmts.iter().enumerate() {
            match s {
                // Functions written one after the other see each other.
                AstStmt::Fn(_) if i > 0 && matches!(stmts[i - 1], AstStmt::Fn(_)) => {}
                AstStmt::Fn(_) if matches!(stmts.get(i + 1), Some(AstStmt::Fn(_))) => {
                    let defs: Vec<&Rc<FnDef>> =
                        stmts[i..].iter().map_while(|s| if let AstStmt::Fn(d) = s { Some(d) } else { None }).collect();
                    out.push(self.group(&defs));
                }
                AstStmt::Let(p, x) => match (&p.kind, &x.kind) {
                    // `name = fn(...) = ...` can call itself by its name.
                    (PatKind::Bind(name), ExprKind::Lambda(def)) => {
                        let code = self.func(def, Some(*name), Vec::new(), Vec::new());
                        let slot = self.new_slot();
                        self.bind(*name, slot);
                        out.push(Stmt::Fn(slot, def.clone(), Rc::new(code)));
                    }
                    (PatKind::Bind(name), _) => {
                        self.stop_before(&mut out, p.span.to(x.span));
                        let value = self.expr(x);
                        let slot = self.new_slot();
                        self.bind(*name, slot);
                        out.push(Stmt::Set(slot, value));
                    }
                    _ => {
                        self.stop_before(&mut out, p.span.to(x.span));
                        let value = self.expr(x);
                        out.push(Stmt::Let(self.pattern(p), value, p.span));
                    }
                },
                AstStmt::Expr(x) if i + 1 == stmts.len() => {
                    self.stop_before(&mut out, x.span);
                    last = Some(self.boxed(x));
                }
                AstStmt::Expr(x) => {
                    self.stop_before(&mut out, x.span);
                    out.push(Stmt::Expr(self.expr(x)));
                }
                AstStmt::Fn(def) => {
                    let code = self.func(def, def.name, Vec::new(), Vec::new());
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

    /// Prepares functions written one after the other in a block: each one
    /// sees the others, and they all capture the same values.
    fn group(&mut self, defs: &[&Rc<FnDef>]) -> Stmt {
        let names: Vec<(Sym, u32)> = defs.iter().enumerate().filter_map(|(i, d)| Some((d.name?, i as u32))).collect();
        let outer = self.scope.clone();
        for &(name, _) in &names {
            self.scope = self.scope.push(name);
        }
        // Each function starts from the values the previous ones capture, so
        // the positions of these values are the same in all of them.
        let mut captures = Vec::new();
        let mut codes = Vec::with_capacity(defs.len());
        for def in defs {
            // Each one sees itself as a function of the group too (behind its
            // parameters): calling it then makes no new closure.
            let code = self.func(def, None, captures, names.clone());
            captures = code.captures.iter().copied().zip(code.capture_names.iter().copied()).collect();
            codes.push(code);
        }
        let (captures, capture_names): (Vec<Capture>, Vec<Sym>) = captures.into_iter().unzip();
        self.scope = outer;
        let group: Rc<[Sibling]> = defs
            .iter()
            .zip(codes)
            .map(|(def, mut code)| {
                code.captures = captures.clone();
                code.capture_names = capture_names.clone();
                Sibling { def: Rc::clone(def), code: Rc::new(code) }
            })
            .collect();
        let mut slots = Vec::with_capacity(names.len());
        for &(name, i) in &names {
            let slot = self.new_slot();
            self.bind(name, slot);
            slots.push((slot, i));
        }
        Stmt::Group(slots, group)
    }
}
