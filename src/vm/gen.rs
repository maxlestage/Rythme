//! Turns the tree that `compile` prepares (local names as slots) into the
//! instructions of the virtual machine.
//!
//! Each expression is written to a register given by its parent (`expr`), or
//! read from the register of a local name when it is one (`operand`): `x + 1`
//! reads `x` where it is. Temporary values take the registers after the slots
//! of the local names, one after the other, and give them back when their
//! expression is done. The arguments of a call take the registers right
//! after the callee: they become the first slots of the callee's frame.
//!
//! The order of evaluation is the one of the tree-walking interpreter it
//! replaces: left to right, the function of a call before its arguments, so
//! that the same error is reported first.

use std::rc::Rc;

use super::{Callee, Chunk, FieldSite, GroupSite, MethodSite, Op, Pc, Reg, SiblingSite};
use crate::ast::{BinOp, UnOp};
use crate::compile::{Arm, ClauseCode, Field, Item, Kind, Method, Node, Part, Pat, Stmt};
use crate::source::Span;
use crate::symbol::Sym;
use crate::value::{Record, Shape, Value};

/// The instructions of a function whose local names take `slots` slots.
/// `self_slot`: where it sees itself by its name; `simple`: one definition
/// whose parameters are plain names, without guard.
pub fn function(slots: usize, self_slot: Option<u32>, simple: bool, clauses: &[ClauseCode]) -> Chunk {
    let mut g = Gen::new(slots);
    if let Some(slot) = self_slot {
        g.emit(Op::SetSelf { dst: slot }, Span::default());
    }
    for clause in clauses {
        let mut fails = Vec::new();
        let start = g.here();
        for (i, p) in &clause.tests {
            fails.push(g.test(*i as Reg, p, Span::default()));
        }
        if let Some(guard) = &clause.guard {
            g.cond(guard, false, &mut fails);
        }
        if g.here() > start {
            let end = g.here();
            g.c.choosing.push((start, end));
        }
        g.tail(&clause.body);
        g.land(fails);
    }
    if !simple {
        g.emit(Op::NoClause, Span::default());
    }
    g.c.check();
    g.c
}

/// The instructions of an expression outside of any function, whose local
/// names take `slots` slots: it gives its value.
pub fn top(node: &Node, slots: usize) -> Chunk {
    let mut g = Gen::new(slots);
    let r = g.operand(node);
    g.emit(Op::Return { src: r }, node.span);
    g.c.check();
    g.c
}

struct Gen {
    c: Chunk,
    /// The first free register.
    next: Reg,
}

/// A small whole number written in the code, given inside an instruction.
fn small_int(n: &Node) -> Option<i32> {
    match &n.kind {
        Kind::Const(Value::Int(v)) => i32::try_from(*v).ok(),
        _ => None,
    }
}

fn is_comparison(op: BinOp) -> bool {
    matches!(op, BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge)
}

/// A call (or a method call) whose result must be true or false when it
/// is the right side of `and` / `or` in tail position.
fn is_call(n: &Node) -> bool {
    matches!(n.kind, Kind::Call(..) | Kind::Method(_) | Kind::SiblingCall(..))
}

impl Gen {
    fn new(slots: usize) -> Gen {
        let next = Reg::try_from(slots).expect("a function with fewer than 4 billion names");
        Gen { c: Chunk { regs: slots, ..Chunk::default() }, next }
    }

    fn emit(&mut self, op: Op, span: Span) -> usize {
        self.emit2(op, span, span)
    }

    fn emit2(&mut self, op: Op, span: Span, extra: Span) -> usize {
        self.c.ops.push(op);
        self.c.spans.push((span, extra));
        self.c.ops.len() - 1
    }

    fn here(&self) -> Pc {
        Pc::try_from(self.c.ops.len()).expect("fewer than 4 billion instructions")
    }

    /// The jumps at `from` go to the next instruction.
    fn land(&mut self, from: Vec<usize>) {
        let to = self.here();
        for at in from {
            match &mut self.c.ops[at] {
                Op::Jump { to: t }
                | Op::JumpIf { to: t, .. }
                | Op::JumpCmp { to: t, .. }
                | Op::JumpCmpInt { to: t, .. }
                | Op::Match { fail: t, .. }
                | Op::MatchCtor { fail: t, .. } => *t = to,
                _ => unreachable!("not a jump"),
            }
        }
    }

    /// A new temporary register.
    fn temp(&mut self) -> Reg {
        let r = self.next;
        self.next += 1;
        self.c.regs = self.c.regs.max(self.next as usize);
        r
    }

    fn index<T>(table: &mut Vec<T>, x: T) -> u32 {
        table.push(x);
        u32::try_from(table.len() - 1).expect("fewer than 4 billion entries")
    }

    fn konst(&mut self, v: Value, dst: Reg, span: Span) {
        let k = Gen::index(&mut self.c.consts, v);
        self.emit(Op::Const { dst, k }, span);
    }

    fn str(&mut self, s: &Rc<str>) -> u32 {
        Gen::index(&mut self.c.strs, s.clone())
    }

    fn pat(&mut self, p: &Pat) -> u32 {
        Gen::index(&mut self.c.pats, p.clone())
    }

    fn global(&mut self, s: Sym, scope: &crate::compile::Scope) -> u32 {
        let site = super::GlobalSite { name: s, scope: scope.clone(), seen: Default::default(), value: Default::default() };
        Gen::index(&mut self.c.globals, site)
    }

    /// Jumps (the instruction returned, to land later) when the value of
    /// `src` does not match `p`, and gives its names their parts otherwise.
    fn test(&mut self, src: Reg, p: &Pat, span: Span) -> usize {
        match p {
            // A constructor whose parts are names or `_`: its fields go to
            // their slots at once.
            Pat::Ctor(id, parts) if parts.iter().all(|p| matches!(p, Pat::Bind(_) | Pat::Any)) => {
                let binds = parts.iter().map(|p| if let Pat::Bind(slot) = p { Some(*slot) } else { None }).collect();
                let site = Gen::index(&mut self.c.ctors, super::CtorSite { id: *id, binds });
                self.emit(Op::MatchCtor { src, site, fail: 0 }, span)
            }
            // A whole number matches the numbers equal to it: what `==` says.
            Pat::Int(n) if i32::try_from(*n).is_ok() => {
                let jumps = super::jumps(BinOp::Eq, false);
                self.emit(Op::JumpCmpInt { op: BinOp::Eq, jumps, when: false, a: src, n: *n as i32, to: 0 }, span)
            }
            p => {
                let pat = self.pat(p);
                self.emit(Op::Match { src, pat, fail: 0 }, span)
            }
        }
    }

    /// Where `Op::Field` and `Op::MethodFn` find the function named `func`.
    fn callee(&mut self, func: &Node) -> Callee {
        match &func.kind {
            Kind::Global(s, scope, _) => Callee::Global(self.global(*s, scope)),
            Kind::Local(slot) => Callee::Local(*slot),
            Kind::Captured(i) => Callee::Captured(*i),
            Kind::Sibling(i) => Callee::Sibling(*i),
            _ => unreachable!("a name"),
        }
    }

    /// The register that holds the value of `n`: the slot of a local name,
    /// or a new temporary.
    fn operand(&mut self, n: &Node) -> Reg {
        if let Kind::Local(slot) = n.kind {
            return slot;
        }
        let r = self.temp();
        self.expr(n, r);
        r
    }

    /// Code that leaves the value of `n` in `dst`.
    fn expr(&mut self, n: &Node, dst: Reg) {
        let mark = self.next;
        let span = n.span;
        match &n.kind {
            Kind::Const(v) => self.konst(v.clone(), dst, span),
            Kind::Local(slot) => {
                if *slot != dst {
                    self.emit(Op::Move { dst, src: *slot }, span);
                }
            }
            Kind::Captured(i) => {
                self.emit(Op::Captured { dst, i: *i }, span);
            }
            Kind::Sibling(i) => {
                self.emit(Op::Sibling { dst, i: *i }, span);
            }
            Kind::Global(s, scope, _) => {
                let g = self.global(*s, scope);
                self.emit(Op::Global { dst, g }, span);
            }
            Kind::Template(parts) => self.template(parts, dst, span),
            Kind::List(items) => self.list(items, dst, span),
            Kind::NewRecord(shape, values) => self.new_record(shape, values, dst, span),
            Kind::Record(items) => self.record(items, dst, span),
            Kind::Lambda(def, code) => {
                let f = Gen::index(&mut self.c.lambdas, (def.clone(), code.clone()));
                self.emit(Op::Closure { dst, f }, span);
            }
            Kind::Call(callee, args) => self.call(callee, args, Some(dst), false, span),
            Kind::SiblingCall(i, args) => self.sibling_call(*i, args, Some(dst), false, span),
            Kind::Method(m) => self.method(m, Some(dst), false, span),
            Kind::Field(obj, name, name_span, func) => {
                let src = self.operand(obj);
                let func = self.callee(func);
                let arg = self.temp();
                let site = Gen::index(&mut self.c.fields, FieldSite { key: name.clone(), func, arg });
                self.emit2(Op::Field { dst, src, site }, span, *name_span);
            }
            Kind::Index(obj, idx) => {
                let o = self.operand(obj);
                let i = self.operand(idx);
                self.emit2(Op::Index { dst, obj: o, idx: i }, span, idx.span);
            }
            Kind::Unary(op, x) => {
                let src = self.operand(x);
                self.emit2(Op::Unary { op: *op, dst, src }, span, x.span);
            }
            Kind::Binary(op, l, r, op_span) => {
                let a = self.operand(l);
                match (op, small_int(r)) {
                    (BinOp::Add, Some(n)) => self.emit(Op::AddInt { dst, a, n }, *op_span),
                    (BinOp::Sub, Some(n)) => self.emit(Op::SubInt { dst, a, n }, *op_span),
                    (_, Some(n)) => self.emit(Op::BinaryInt { op: *op, dst, a, n }, *op_span),
                    (BinOp::Add, None) => {
                        let b = self.operand(r);
                        self.emit(Op::Add { dst, a, b }, *op_span)
                    }
                    (_, None) => {
                        let b = self.operand(r);
                        self.emit(Op::Binary { op: *op, dst, a, b }, *op_span)
                    }
                };
            }
            Kind::Logic(..) => {
                // `a and b`: true or false, the right side only when needed.
                let mut no = Vec::new();
                self.cond(n, false, &mut no);
                self.konst(Value::Bool(true), dst, span);
                let done = self.emit(Op::Jump { to: 0 }, span);
                self.land(no);
                self.konst(Value::Bool(false), dst, span);
                self.land(vec![done]);
            }
            Kind::If(c, a, b) => {
                let mut no = Vec::new();
                self.cond(c, false, &mut no);
                self.expr(a, dst);
                let done = self.emit(Op::Jump { to: 0 }, span);
                self.land(no);
                self.expr(b, dst);
                self.land(vec![done]);
            }
            Kind::Match(subject, arms) => self.match_(subject, arms, Some(dst)),
            Kind::Block(stmts, last) => {
                self.stmts(stmts);
                match last {
                    Some(x) => self.expr(x, dst),
                    None => self.konst(Value::Unit, dst, span),
                }
            }
            Kind::Try(x) => {
                let src = self.operand(x);
                self.emit(Op::Try { dst, src }, span);
            }
            Kind::Range(a, b) => {
                let a = self.operand(a);
                let b = self.operand(b);
                self.emit(Op::Range { dst, a, b }, span);
            }
        }
        self.next = mark;
    }

    /// Code that makes `n` the result of the function: a call there is a
    /// tail call.
    fn tail(&mut self, n: &Node) {
        let mark = self.next;
        match &n.kind {
            Kind::If(c, a, b) => {
                let mut no = Vec::new();
                self.cond(c, false, &mut no);
                self.tail(a);
                self.land(no);
                self.tail(b);
            }
            Kind::Match(subject, arms) => self.match_(subject, arms, None),
            Kind::Block(stmts, last) => {
                self.stmts(stmts);
                match last {
                    Some(x) => self.tail(x),
                    None => {
                        let r = self.temp();
                        self.konst(Value::Unit, r, n.span);
                        self.emit(Op::Return { src: r }, n.span);
                    }
                }
            }
            Kind::Call(callee, args) => self.call(callee, args, None, false, n.span),
            Kind::SiblingCall(i, args) => self.sibling_call(*i, args, None, false, n.span),
            Kind::Method(m) => self.method(m, None, false, n.span),
            Kind::Logic(op, l, r) if is_call(r) => {
                // `a and f(x)`: when `a` decides, it is the result; otherwise
                // `f(x)` is a tail call whose result must be true or false.
                let mut decided = Vec::new();
                let or = *op == BinOp::Or;
                self.cond(l, or, &mut decided);
                match &r.kind {
                    Kind::Call(callee, args) => self.call(callee, args, None, true, r.span),
                    Kind::SiblingCall(i, args) => self.sibling_call(*i, args, None, true, r.span),
                    Kind::Method(m) => self.method(m, None, true, r.span),
                    _ => unreachable!(),
                }
                self.land(decided);
                let res = self.temp();
                self.konst(Value::Bool(or), res, n.span);
                self.emit(Op::Return { src: res }, n.span);
            }
            _ => {
                let r = self.operand(n);
                self.emit(Op::Return { src: r }, n.span);
            }
        }
        self.next = mark;
    }

    /// Jumps (the instructions added to `out`, to land later) when the
    /// condition `c` is `when`; E005 when it is not true or false.
    fn cond(&mut self, c: &Node, when: bool, out: &mut Vec<usize>) {
        let mark = self.next;
        match &c.kind {
            Kind::Logic(op, l, r) => {
                let and = *op == BinOp::And;
                if and != when {
                    // `a and b` is false when either is, `a or b` true when either is.
                    self.cond(l, when, out);
                    self.cond(r, when, out);
                } else {
                    // `a and b` is true when both are, `a or b` false when both are.
                    let mut skip = Vec::new();
                    self.cond(l, !when, &mut skip);
                    self.cond(r, when, out);
                    self.land(skip);
                }
            }
            // `not x`: an error names `x`, as when it is evaluated.
            Kind::Unary(UnOp::Not, x) => self.cond(x, !when, out),
            Kind::Binary(op, l, r, op_span) if is_comparison(*op) => {
                let a = self.operand(l);
                let jumps = super::jumps(*op, when);
                let at = match small_int(r) {
                    Some(n) => self.emit(Op::JumpCmpInt { op: *op, jumps, when, a, n, to: 0 }, *op_span),
                    None => {
                        let b = self.operand(r);
                        self.emit(Op::JumpCmp { op: *op, jumps, when, a, b, to: 0 }, *op_span)
                    }
                };
                out.push(at);
            }
            _ => {
                let cond = self.operand(c);
                out.push(self.emit(Op::JumpIf { cond, when, to: 0 }, c.span));
            }
        }
        self.next = mark;
    }

    fn stmts(&mut self, stmts: &[Stmt]) {
        for s in stmts {
            let mark = self.next;
            match s {
                Stmt::Set(slot, x) => self.expr(x, *slot),
                Stmt::Let(p, x, span) => {
                    let src = self.operand(x);
                    let pat = self.pat(p);
                    self.emit(Op::Let { src, pat }, *span);
                }
                Stmt::Expr(x) => {
                    self.operand(x);
                }
                Stmt::Fn(slot, def, code) => {
                    let f = Gen::index(&mut self.c.lambdas, (def.clone(), code.clone()));
                    self.emit(Op::Closure { dst: *slot, f }, def.span);
                }
                Stmt::Group(slots, group) => {
                    let g = Gen::index(&mut self.c.groups, GroupSite { slots: slots.clone(), group: group.clone() });
                    self.emit(Op::Group { g }, Span::default());
                }
                Stmt::Stop(info, at) => {
                    let info = Gen::index(&mut self.c.stops, info.clone());
                    self.emit(Op::Stop { info }, *at);
                }
            }
            self.next = mark;
        }
    }

    /// `callee(args)`: the result to `dst`, or a tail call (`None`).
    fn call(&mut self, callee: &Node, args: &[Node], dst: Option<Reg>, test: bool, span: Span) {
        let f = self.temp();
        // A name that surely has a value is read at the call; any other
        // function first, so that an unknown name is reported before the
        // arguments run.
        let global = match &callee.kind {
            Kind::Global(s, scope, true) => Some(self.global(*s, scope)),
            _ => {
                self.expr(callee, f);
                None
            }
        };
        for a in args {
            let r = self.temp();
            self.expr(a, r);
        }
        let argc = u32::try_from(args.len()).expect("fewer than 4 billion arguments");
        match (dst, global) {
            (Some(dst), None) => self.emit(Op::Call { dst, f, argc }, span),
            (None, None) => self.emit(Op::TailCall { f, argc, test }, span),
            (Some(dst), Some(g)) => {
                let site = Gen::index(&mut self.c.calls, super::CallSite { g, args: f + 1, argc });
                self.emit(Op::CallGlobal { dst, site }, span)
            }
            (None, Some(g)) => {
                let site = Gen::index(&mut self.c.calls, super::CallSite { g, args: f + 1, argc });
                self.emit(Op::TailCallGlobal { site, test }, span)
            }
        };
    }

    /// A call of the function number `i` of the group of the running function.
    fn sibling_call(&mut self, i: u32, args: &[Node], dst: Option<Reg>, test: bool, span: Span) {
        let first = self.next;
        for a in args {
            let r = self.temp();
            self.expr(a, r);
        }
        let argc = u32::try_from(args.len()).expect("fewer than 4 billion arguments");
        let site = Gen::index(&mut self.c.siblings, SiblingSite { i, args: first, argc });
        match dst {
            Some(dst) => self.emit(Op::CallSibling { dst, site }, span),
            None => self.emit(Op::TailCallSibling { site, test }, span),
        };
    }

    /// `recv.name(args)`: registers `f` (the function), `f + 1` (the
    /// receiver), then the arguments.
    fn method(&mut self, m: &Method, dst: Option<Reg>, test: bool, span: Span) {
        let f = self.temp();
        let recv = self.temp();
        self.expr(&m.recv, recv);
        let func = self.callee(&m.func);
        let argc = u32::try_from(m.args.len()).expect("fewer than 4 billion arguments");
        let site = Gen::index(&mut self.c.methods, MethodSite { name: m.name, key: m.key.clone(), func, argc });
        self.emit2(Op::MethodFn { site, f }, span, m.name_span);
        for a in &m.args {
            let r = self.temp();
            self.expr(a, r);
        }
        match dst {
            Some(dst) => self.emit(Op::CallMethod { dst, site, f }, span),
            None => self.emit(Op::TailCallMethod { site, f, test }, span),
        };
    }

    /// `match subject ... end`: the result to `dst`, or in tail position (`None`).
    fn match_(&mut self, subject: &Node, arms: &[Arm], dst: Option<Reg>) {
        let src = self.operand(subject);
        let mut done = Vec::new();
        for arm in arms {
            let mut fails = Vec::new();
            match &arm.pat {
                Pat::Any => {}
                Pat::Bind(slot) => {
                    self.emit(Op::Move { dst: *slot, src }, subject.span);
                }
                p => fails.push(self.test(src, p, subject.span)),
            }
            if let Some(g) = &arm.guard {
                self.cond(g, false, &mut fails);
            }
            match dst {
                Some(dst) => {
                    self.expr(&arm.body, dst);
                    done.push(self.emit(Op::Jump { to: 0 }, arm.body.span));
                }
                None => self.tail(&arm.body),
            }
            self.land(fails);
        }
        self.emit(Op::NoCase { src }, subject.span);
        self.land(done);
    }

    /// `"... {x} ..."`: each piece added in turn, so that a text that grows
    /// too large stops before the next piece is evaluated.
    fn template(&mut self, parts: &[Part], dst: Reg, span: Span) {
        if parts.is_empty() {
            self.konst(Value::text(""), dst, span);
            return;
        }
        // Room for the whole text, most of the time.
        let room: usize = parts.iter().map(|p| if let Part::Lit(l) = p { l.len() } else { 8 }).sum();
        let room = u32::try_from(room.max(1)).unwrap_or(u32::MAX);
        let head = parts.len() > 1;
        for (i, p) in parts.iter().enumerate() {
            let cap = if i == 0 { room } else { 0 };
            match p {
                Part::Lit(l) => {
                    let k = self.str(l);
                    self.emit(Op::TextLit { dst, k, cap }, span);
                }
                Part::Expr(x) => {
                    let mark = self.next;
                    let src = self.operand(x);
                    self.emit(Op::TextVal { dst, src, cap, head: head && i == 0 }, span);
                    self.next = mark;
                }
            }
        }
    }

    /// `[a, ..xs, b]`
    fn list(&mut self, items: &[Item], dst: Reg, span: Span) {
        if items.iter().all(|it| matches!(it, Item::One(_))) {
            // Values written in the code: one list, made once.
            if items.iter().all(|it| matches!(it, Item::One(Node { kind: Kind::Const(_), .. }))) {
                let values = items
                    .iter()
                    .map(|it| match it {
                        Item::One(Node { kind: Kind::Const(v), .. }) => v.clone(),
                        _ => unreachable!(),
                    })
                    .collect();
                self.konst(Value::list(values), dst, span);
                return;
            }
            let first = self.next;
            for it in items {
                if let Item::One(x) = it {
                    let r = self.temp();
                    self.expr(x, r);
                }
            }
            let n = u32::try_from(items.len()).expect("fewer than 4 billion elements");
            self.emit(Op::List { dst, first, n }, span);
            return;
        }
        let (rest, head) = match items {
            // `[..xs, x]`: added after `xs`, in the room it has when it can.
            [Item::Spread(first), rest @ ..] if !rest.is_empty() => {
                let src = self.operand(first);
                self.emit(Op::ListFrom { dst, src }, first.span);
                (rest, true)
            }
            _ => {
                let cap = u32::try_from(items.len()).expect("fewer than 4 billion elements");
                self.emit(Op::ListNew { dst, cap }, span);
                (items, false)
            }
        };
        for it in rest {
            let mark = self.next;
            match it {
                Item::One(x) => {
                    let src = self.operand(x);
                    self.emit(Op::ListPush { dst, src }, span);
                }
                Item::Spread(x) => {
                    let src = self.operand(x);
                    self.emit2(Op::ListSpread { dst, src }, span, x.span);
                }
            }
            self.next = mark;
        }
        if head {
            self.emit(Op::ListCheck { dst }, span);
        }
    }

    /// `{a: x, b: y}`: different names and no `..`.
    fn new_record(&mut self, shape: &Rc<Shape>, values: &[Node], dst: Reg, span: Span) {
        if values.iter().all(|x| matches!(x.kind, Kind::Const(_))) {
            let vals = values
                .iter()
                .map(|x| match &x.kind {
                    Kind::Const(v) => v.clone(),
                    _ => unreachable!(),
                })
                .collect();
            self.konst(Value::Record(Rc::new(Record::with_shape(shape.clone(), vals))), dst, span);
            return;
        }
        let first = self.next;
        for x in values {
            let r = self.temp();
            self.expr(x, r);
        }
        let shape = Gen::index(&mut self.c.shapes, shape.clone());
        self.emit(Op::Record { dst, shape, first }, span);
    }

    /// `{..r, a: x}`, or names given twice.
    fn record(&mut self, items: &[Field], dst: Reg, span: Span) {
        // `{..r, a: x, b: y}`: a copy of `r`, then the fields, set at once.
        if let [Field::Spread(from), rest @ ..] = items {
            if !rest.is_empty() && rest.iter().all(|it| matches!(it, Field::Field(..))) {
                let src = self.operand(from);
                self.emit(Op::RecordSpread { dst, src, first: true }, from.span);
                let first = self.next;
                let mut keys = Vec::with_capacity(rest.len());
                for it in rest {
                    if let Field::Field(k, x) = it {
                        let r = self.temp();
                        self.expr(x, r);
                        keys.push(k.clone());
                    }
                }
                let site = Gen::index(&mut self.c.updates, super::UpdateSite { keys: keys.into(), first });
                self.emit(Op::RecordSets { dst, site }, span);
                return;
            }
        }
        self.emit(Op::RecordNew { dst }, span);
        for (i, it) in items.iter().enumerate() {
            let mark = self.next;
            match it {
                Field::Field(k, x) => {
                    let src = self.operand(x);
                    let key = self.str(k);
                    self.emit(Op::RecordSet { dst, key, src }, span);
                }
                Field::Spread(x) => {
                    let src = self.operand(x);
                    self.emit(Op::RecordSpread { dst, src, first: i == 0 }, x.span);
                }
            }
            self.next = mark;
        }
    }
}
