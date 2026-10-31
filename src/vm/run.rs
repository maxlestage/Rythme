//! Runs the instructions of the virtual machine (see the module `vm`).
//!
//! A call of a function of the program (`Interp::call_member`) runs the
//! instructions of its definition in `exec`, until they give a result, make a
//! call in tail position, or stop with an error. A call in tail position
//! comes back to `call_loop`, which runs the function called in the same
//! frame: a function that calls itself (or another one) last loops without
//! growing the stack; a function that calls itself last even starts again in
//! `exec` at once. Other calls nest: `exec` calls `Interp::call_member`.
//!
//! The frame of a running function, on the value stack (`Interp::stack`),
//! holds its arguments, its local names and its temporary values: its
//! registers. The arguments of a call are the last registers of the caller,
//! which become the first ones of the callee. A call that ends frees the
//! values its registers hold; what is left there is never read: every
//! register is written before it is read.
//!
//! The hooks of the debugger and the limit of calls (`call_hook`,
//! `return_hook`, `error_hook`, `steps_left`) are met at the same moments as
//! in the tree-walking interpreter of Rythme 1.0, whose structure `call_member` and
//! `call_loop` keep: the same calls are seen, counted and traced.
//!
//! # The unsafe code
//!
//! `exec` reads and writes the registers of its frame through a pointer,
//! without checking bounds nor going through the stack each time, which
//! makes the interpreter much faster:
//!
//! - the registers of a frame are `self.stack[base..base + chunk.regs]`: the
//!   stack is made that long before the frame runs (`call_loop`, `run_top`),
//!   and it only gets shorter above the frames that run;
//! - the pointer is taken again from the stack before each instruction, and
//!   an instruction that may move the stack (a call, a hook, anything that
//!   runs code) no longer uses it afterwards;
//! - `Chunk::check` ensures, when a chunk is made, that every register an
//!   instruction names is a register of its frame.
//!
//! `cargo +nightly miri test --lib vm` runs programs through it under Miri.

use std::rc::Rc;

use super::{Callee, Chunk, Op, Reg};
use crate::ast::{BinOp, FnDef, UnOp};
use crate::compile::FnCode;
use crate::diagnostic::{Diagnostic, Fallible};
use crate::interp::*;
use crate::source::Span;
use crate::value::*;

/// What runs in `exec`.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// The body of a function called.
    Call,
    /// A top-level expression: a `?` that returns from it is an error.
    Top,
    /// The body of a test: a `?` gives the value.
    Test,
}

/// How `exec` stops.
enum Exec {
    Return(Value),
    /// A call in tail position: the function (and the function of its group
    /// it calls), where its arguments are, how many, where it is written,
    /// and whether its result must be true or false (`a and f(x)`).
    Tail(Value, Option<u32>, usize, usize, Span, bool),
    /// An error; when the body of the definition had started, the place of
    /// the call, for the trace of calls (an error while the function chooses
    /// its definition does not show it, see `Chunk::choosing`). After the
    /// function called itself last, the place of that call.
    Error(Box<Diagnostic>, Option<Span>),
}

/// The nominal stack a nested call used in the tree-walking interpreter: the
/// budget of stack (`interp::set_stack_budget`) allows that many nested
/// calls, with their registers.
const CALL_COST: usize = 1024;

/// The definition and the code of the function `member` of the group of
/// `clo`, or of `clo` itself.
#[inline(always)]
fn member_code(clo: &Closure, member: Option<u32>) -> (&Rc<FnDef>, &Rc<FnCode>) {
    match member {
        None => (&clo.def, &clo.code),
        Some(i) => {
            let s = &clo.siblings.as_ref().expect("a function of a group")[i as usize];
            (&s.def, &s.code)
        }
    }
}

/// Takes the value out of a register, leaving `()`.
#[inline(always)]
fn take(slot: &mut Value) -> Value {
    std::mem::replace(slot, Value::Unit)
}

/// A copy of a value: the common small ones without a call.
#[inline(always)]
fn dup(v: &Value) -> Value {
    match v {
        Value::Int(n) => Value::Int(*n),
        Value::Bool(b) => Value::Bool(*b),
        Value::Unit => Value::Unit,
        Value::Func(f) => Value::Func(f.clone()),
        v => v.clone(),
    }
}

/// Values that hold no memory of their own.
#[inline(always)]
fn is_plain(v: &Value) -> bool {
    matches!(v, Value::Unit | Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::Symbol(_) | Value::Builtin(_))
}

impl Interp {
    /// Runs `code` as a top-level expression whose first slots hold `values`
    /// (`test`: the body of a test, where a `?` gives the value; otherwise a
    /// `?` is an error at `span`).
    pub(crate) fn run_top(&mut self, code: &Rc<FnCode>, values: Vec<Value>, span: Span, test: bool) -> Fallible<Value> {
        let base = self.stack.len();
        let n = values.len();
        self.stack.extend(values);
        self.stack.resize_with(base + code.chunk.regs.max(n), || Value::Unit);
        let kind = if test { Kind::Test } else { Kind::Top };
        match self.exec(code, None, None, base, kind, span) {
            Exec::Return(v) => Ok(v),
            Exec::Error(d, _) => Err(d),
            Exec::Tail(..) => unreachable!("no call in tail position at the top level"),
        }
    }

    /// Calls a function value with the `argc` values on top of the stack as
    /// arguments, and removes them: for built-in functions that call
    /// functions of the program, and for the tools.
    pub(crate) fn call_value(&mut self, f: Value, argc: usize, span: Span) -> Fallible<Value> {
        let args = self.stack.len() - argc;
        let r = self.call_member(f, None, args, argc, span);
        // What is left above holds no memory: no need to free it value by value.
        self.pop_to(args);
        r
    }

    /// Makes room on the stack for registers up to `len`.
    #[inline(always)]
    fn ensure(&mut self, len: usize) {
        if self.stack.len() < len {
            self.grow(len);
        }
    }

    #[inline(never)]
    fn grow(&mut self, len: usize) {
        self.stack.resize_with(len, || Value::Unit);
    }

    /// Frees the values of the registers `from..to`.
    #[inline(always)]
    fn reset(&mut self, from: usize, to: usize) {
        let to = to.min(self.stack.len());
        if let Some(regs) = self.stack.get_mut(from..to) {
            for v in regs {
                if !is_plain(v) {
                    *v = Value::Unit;
                }
            }
        }
    }

    /// `call_member`, with the common call of a function of the program (no
    /// debugger, no limit reached, no call in tail position) in place: what
    /// it does is what `call_member` and `call_loop` do then.
    #[inline(always)]
    fn call_fast(&mut self, f: Value, member: Option<u32>, args: usize, argc: usize, span: Span) -> Fallible<Value> {
        if let Value::Func(clo) = &f {
            let (calls, values) = call_limits();
            let (def, code) = member_code(clo, member);
            if argc == code.arity && self.steps_left != 0 && self.depth < calls && self.stack.len() <= values && !self.stack_too_deep() {
                self.depth += 1;
                self.steps_left -= 1;
                let regs = code.chunk.regs;
                self.ensure(args + regs);
                match self.exec(code, Some(clo), member, args, Kind::Call, span) {
                    Exec::Return(v) => {
                        self.reset(args, args + regs);
                        let r = if self.steps_left == 0 { self.return_hook(Ok(v)) } else { Ok(v) };
                        self.depth -= 1;
                        return r;
                    }
                    Exec::Error(d, called) => {
                        let d = self.call_failed(d, called, def, args, regs);
                        let r = if self.steps_left == 0 { self.return_hook(Err(d)) } else { Err(d) };
                        self.depth -= 1;
                        return r;
                    }
                    Exec::Tail(next, next_member, next_args, next_argc, next_span, next_test) => {
                        let mut test = next_test.then_some(next_span);
                        let mut r = self.tail_chain(next, next_member, args, regs, next_args, next_argc, next_span, &mut test);
                        if self.steps_left == 0 {
                            r = self.return_hook(r);
                        }
                        self.depth -= 1;
                        return match test {
                            Some(at) => r.and_then(|v| truth(&v, at).map(Value::Bool)),
                            None => r,
                        };
                    }
                }
            }
        }
        self.call_member(f, member, args, argc, span)
    }

    /// The frame at `base` (of `regs` registers) of the function `def`,
    /// called at `span`, stopped with the error `d`.
    #[cold]
    #[inline(never)]
    fn call_failed(&mut self, mut d: Box<Diagnostic>, called: Option<Span>, def: &FnDef, base: usize, regs: usize) -> Box<Diagnostic> {
        if let Some(span) = called {
            if self.steps_left == 0 {
                d = self.error_hook(d);
            }
            add_frame(&mut d, def, span);
        }
        self.reset(base, base + regs);
        d
    }

    /// The frame at `base` (of `regs` registers) made a call in tail position
    /// (`next`, with `argc` arguments from `args`): goes on in `call_loop`.
    #[allow(clippy::too_many_arguments)]
    #[inline(never)]
    fn tail_chain(
        &mut self,
        next: Value,
        member: Option<u32>,
        base: usize,
        regs: usize,
        args: usize,
        argc: usize,
        span: Span,
        test: &mut Option<Span>,
    ) -> Fallible<Value> {
        // A value built last (`Node(l, v, r)`) takes its fields where they
        // are, without counting as a call.
        if let Value::Ctor(c) = &next {
            if c.fields.len() == argc {
                let fields = self.stack[args..args + argc].iter_mut().map(take).collect();
                self.reset(base, base + regs);
                return Ok(Value::Variant(Rc::new(Variant { ctor: c.clone(), fields })));
            }
        }
        for i in 0..argc {
            self.stack.swap(base + i, args + i);
        }
        self.reset(base + argc, base + regs);
        self.call_loop(next, member, base, argc, span, test)
    }

    /// A call of `f` (or of the function `member` of its group) with the
    /// `argc` values from `args` on the stack as arguments, which it uses up.
    #[inline(never)]
    fn call_member(&mut self, f: Value, member: Option<u32>, args: usize, argc: usize, span: Span) -> Fallible<Value> {
        let (calls, values) = call_limits();
        if self.depth >= calls || self.stack.len() > values || self.stack_too_deep() {
            self.reset(args, args + argc);
            return Err(too_deep(self.depth, span));
        }
        self.depth += 1;
        let mut test = None;
        let mut r = self.call_loop(f, member, args, argc, span, &mut test);
        if self.steps_left == 0 {
            r = self.return_hook(r);
        }
        self.depth -= 1;
        match test {
            Some(at) => r.and_then(|v| truth(&v, at).map(Value::Bool)),
            None => r,
        }
    }

    /// Runs a call whose frame starts at `base`, with the arguments in its
    /// first registers. A call in tail position runs in the same frame.
    /// `test` is set when the result must be true or false.
    #[inline(always)]
    fn call_loop(
        &mut self,
        mut f: Value,
        mut member: Option<u32>,
        base: usize,
        mut argc: usize,
        mut span: Span,
        test: &mut Option<Span>,
    ) -> Fallible<Value> {
        loop {
            if self.steps_left == 0 {
                if let Err(d) = self.call_hook(&f, member, base, argc, span) {
                    self.reset(base, base + argc);
                    return Err(d);
                }
            } else {
                self.steps_left -= 1;
            }
            let Value::Func(clo) = &f else { return self.call_other(f, base, argc, span) };
            let (def, code) = member_code(clo, member);
            if argc != code.arity {
                let d = fn_arity_error(def, argc, span);
                self.reset(base, base + argc);
                return Err(d);
            }
            let regs = code.chunk.regs;
            self.ensure(base + regs);
            match self.exec(code, Some(clo), member, base, Kind::Call, span) {
                Exec::Return(v) => {
                    self.reset(base, base + regs);
                    return Ok(v);
                }
                Exec::Error(mut d, called) => {
                    if let Some(span) = called {
                        if self.steps_left == 0 {
                            d = self.error_hook(d);
                        }
                        add_frame(&mut d, def, span);
                    }
                    self.reset(base, base + regs);
                    return Err(d);
                }
                Exec::Tail(next, next_member, args, next_argc, next_span, next_test) => {
                    if next_test {
                        *test = Some(next_span);
                    }
                    // A value built last (`Node(l, v, r)`) takes its fields
                    // where they are, without counting as a call.
                    if let Value::Ctor(c) = &next {
                        if c.fields.len() == next_argc {
                            let fields = self.stack[args..args + next_argc].iter_mut().map(take).collect();
                            self.reset(base, base + regs);
                            return Ok(Value::Variant(Rc::new(Variant { ctor: c.clone(), fields })));
                        }
                    }
                    // The arguments of the next call replace the frame (they
                    // are above it); the old values go above them, and are freed.
                    for i in 0..next_argc {
                        self.stack.swap(base + i, args + i);
                    }
                    self.reset(base + next_argc, base + regs);
                    f = next;
                    member = next_member;
                    argc = next_argc;
                    span = next_span;
                }
            }
        }
    }

    /// A call of something else than a function of the program, with the
    /// `argc` values from `args` on the stack.
    #[inline(never)]
    fn call_other(&mut self, f: Value, args: usize, argc: usize, span: Span) -> Fallible<Value> {
        let r = match f {
            Value::Builtin(b) => self.call_builtin(b, args, argc, span),
            Value::Ctor(c) => self.construct(c, args, argc, span),
            other => Err(not_callable(&other, span)),
        };
        self.reset(args, args + argc);
        r
    }

    /// Runs the instructions of `code` (of the closure `clo`, the function
    /// `member` of its group) in the frame at `base`, from the first one.
    /// `span`: where it was called, or the place of a top-level expression.
    fn exec(&mut self, code: &FnCode, clo: Option<&Rc<Closure>>, member: Option<u32>, base: usize, kind: Kind, mut span: Span) -> Exec {
        // What each instruction needs, kept in registers of the machine:
        // where the instruction is, and where the registers of the frame
        // are. The rest is read where it is needed.
        let start: *const Op = code.chunk.ops.as_ptr();
        let mut ip = start;
        debug_assert!(base + code.chunk.regs <= self.stack.len());
        // SAFETY: the registers of the frame are on the stack (see the module).
        let mut regs: *mut Value = unsafe { self.stack.as_mut_ptr().add(base) };
        // The table of the chunk, read when an instruction needs it (the
        // compiler would read them all when the call starts).
        macro_rules! chunk {
            () => {
                std::hint::black_box(&code.chunk)
            };
        }
        // The position of the instruction that runs.
        macro_rules! pc {
            () => {
                // SAFETY: both point into the instructions of the chunk.
                unsafe { ip.offset_from(start) as usize - 1 }
            };
        }
        // The pointer to the registers, after something that may have moved
        // the stack, or used it otherwise (a call, a hook, a function of the
        // interpreter that writes a register).
        macro_rules! refresh {
            () => {
                debug_assert!(base + code.chunk.regs <= self.stack.len());
                // SAFETY: as above.
                regs = unsafe { self.stack.as_mut_ptr().add(base) };
            };
        }
        // A register read or written in place.
        macro_rules! r {
            ($r:expr) => {
                // SAFETY: a register of the frame (`Chunk::check`).
                unsafe { &*regs.add($r as usize) }
            };
        }
        macro_rules! w {
            ($r:expr, $v:expr) => {{
                let v = $v;
                // SAFETY: a register of the frame (`Chunk::check`), and no
                // other reference to it is alive.
                set(unsafe { &mut *regs.add($r as usize) }, v)
            }};
        }
        macro_rules! fail {
            ($e:expr) => {
                match $e {
                    Ok(v) => v,
                    Err(d) => return Exec::Error(d, (!chunk!().chooses(pc!())).then_some(span)),
                }
            };
        }
        macro_rules! at {
            () => {
                chunk!().spans[pc!()].0
            };
        }
        macro_rules! at2 {
            () => {
                chunk!().spans[pc!()].1
            };
        }
        loop {
            // SAFETY: `ip` is at an instruction of the chunk: it starts at the
            // first one, goes to the next one only after an instruction that
            // may go on (the last one does not), or jumps to one (`Chunk::check`).
            let op = unsafe { *ip };
            ip = unsafe { ip.add(1) };
            match op {
                Op::Move { dst, src } => w!(dst, dup(r!(src))),
                Op::Const { dst, k } => w!(dst, dup(&code.chunk.consts[k as usize])),
                Op::Captured { dst, i } => w!(dst, dup(&clo.expect("a closure").captured[i as usize])),
                Op::Global { dst, g } => {
                    let v = fail!(self.global(&code.chunk, g, pc!()));
                    w!(dst, v);
                }
                Op::Sibling { dst, i } => w!(dst, sibling(clo, i)),
                Op::Closure { dst, f } => {
                    self.closure(chunk!(), f, base, dst, clo);
                    refresh!();
                }
                Op::Group { g } => {
                    self.group(chunk!(), g, base, clo);
                    refresh!();
                }
                Op::SetSelf { dst } => w!(dst, Value::Func(clo.expect("a closure").clone())),

                Op::Add { dst, a, b } => {
                    let v = match (r!(a), r!(b)) {
                        (Value::Int(x), Value::Int(y)) => x.checked_add(*y).map(Value::Int),
                        _ => None,
                    };
                    let v = match v {
                        Some(v) => v,
                        None => fail!(binop(BinOp::Add, r!(a), r!(b), at!())),
                    };
                    w!(dst, v);
                }
                Op::AddInt { dst, a, n } => {
                    let v = match r!(a) {
                        Value::Int(x) => x.checked_add(n as i64).map(Value::Int),
                        _ => None,
                    };
                    let v = match v {
                        Some(v) => v,
                        None => fail!(binop(BinOp::Add, r!(a), &Value::Int(n as i64), at!())),
                    };
                    w!(dst, v);
                }
                Op::SubInt { dst, a, n } => {
                    let v = match r!(a) {
                        Value::Int(x) => x.checked_sub(n as i64).map(Value::Int),
                        _ => None,
                    };
                    let v = match v {
                        Some(v) => v,
                        None => fail!(binop(BinOp::Sub, r!(a), &Value::Int(n as i64), at!())),
                    };
                    w!(dst, v);
                }
                Op::Binary { op, dst, a, b } => {
                    let v = match (r!(a), r!(b)) {
                        (Value::Int(x), Value::Int(y)) => int_op(op, *x, *y),
                        _ => None,
                    };
                    let v = match v {
                        Some(v) => v,
                        None => fail!(binop(op, r!(a), r!(b), at!())),
                    };
                    w!(dst, v);
                }
                Op::BinaryInt { op, dst, a, n } => {
                    let v = match r!(a) {
                        Value::Int(x) => int_op(op, *x, n as i64),
                        _ => None,
                    };
                    let v = match v {
                        Some(v) => v,
                        None => fail!(binop(op, r!(a), &Value::Int(n as i64), at!())),
                    };
                    w!(dst, v);
                }
                Op::Unary { op, dst, src } => {
                    let v = match (op, r!(src)) {
                        (UnOp::Not, Value::Bool(b)) => Value::Bool(!b),
                        (UnOp::Neg, Value::Int(n)) if *n != i64::MIN => Value::Int(-n),
                        (_, v) => fail!(unary(op, v, at!(), at2!())),
                    };
                    w!(dst, v);
                }
                Op::Index { dst, obj, idx } => {
                    let v = fail!(index(r!(obj), r!(idx), at!(), at2!()));
                    w!(dst, v);
                }
                Op::Range { dst, a, b } => {
                    let v = fail!(range(r!(a), r!(b), at!()));
                    w!(dst, v);
                }
                Op::Try { dst, src } => match self.wrap(r!(src)) {
                    Wrap::Some | Wrap::Ok => w!(dst, Interp::payload(r!(src))),
                    Wrap::Other => fail!(Err(try_error(r!(src), at!()))),
                    // The running function returns it.
                    Wrap::None | Wrap::Err if kind != Kind::Top => return Exec::Return(r!(src).clone()),
                    // Outside of any function: an error, not one the debugger stops on.
                    Wrap::None | Wrap::Err => {
                        self.question_at_top = true;
                        return Exec::Error(question_outside(span), None);
                    }
                },
                Op::Field { dst, src, site } => {
                    let found = self.field_or_fn(chunk!(), site, base, src, clo, pc!());
                    refresh!();
                    match fail!(found) {
                        Ok(v) => w!(dst, v),
                        Err((f, arg)) => {
                            let v = self.call_member(f, None, base + arg as usize, 1, at!());
                            refresh!();
                            w!(dst, fail!(v));
                        }
                    }
                }

                Op::List { dst, first, n } => {
                    // SAFETY: registers of the frame (`Chunk::check`), one at a time.
                    let items = (first..first + n).map(|i| take(unsafe { &mut *regs.add(i as usize) })).collect();
                    w!(dst, Value::list(items));
                }
                Op::ListNew { dst, cap } => w!(dst, Value::list(Vec::with_capacity(cap as usize))),
                Op::ListFrom { dst, src } => match r!(src) {
                    Value::List(l) => w!(dst, Value::List(l.clone())),
                    other => fail!(Err(spread_error(other, at!(), true))),
                },
                Op::ListPush { dst, src } => {
                    let x = r!(src).clone();
                    // SAFETY: a register of the frame, the only reference to it.
                    if let Value::List(l) = unsafe { &mut *regs.add(dst as usize) } {
                        l.push_mut(x);
                    }
                }
                Op::ListSpread { dst, src } => {
                    let r = self.list_spread(base + dst as usize, base + src as usize, chunk!().spans[pc!()]);
                    refresh!();
                    fail!(r);
                }
                Op::ListCheck { dst } => {
                    if matches!(r!(dst), Value::List(l) if l.len() > MAX_LIST) {
                        fail!(Err(too_big(true, at!())));
                    }
                }
                Op::Record { dst, shape, first } => {
                    self.record(chunk!(), shape, base, dst, first);
                    refresh!();
                }
                Op::RecordNew { dst } => w!(dst, Value::Record(Rc::new(Record::new()))),
                Op::RecordSets { dst, site } => {
                    self.record_sets(&chunk!().updates[site as usize], base, dst);
                    refresh!();
                }
                Op::RecordSet { dst, key, src } => {
                    self.record_set(chunk!(), key, base + dst as usize, base + src as usize);
                    refresh!();
                }
                Op::RecordSpread { dst, src, first } => {
                    let r = self.record_spread(base + dst as usize, base + src as usize, first, at!());
                    refresh!();
                    fail!(r);
                }
                Op::TextLit { dst, k, cap } => {
                    let r = self.text_lit(chunk!(), k, cap, base + dst as usize, at!());
                    refresh!();
                    fail!(r);
                }
                Op::TextVal { dst, src, cap, head } => {
                    let r = self.text_piece(base + dst as usize, base + src as usize, cap, head, at!());
                    refresh!();
                    fail!(r);
                }

                // SAFETY (the jumps): an instruction of the chunk (`Chunk::check`).
                Op::Jump { to } => ip = unsafe { start.add(to as usize) },
                Op::JumpIf { cond, when, to } => match r!(cond) {
                    Value::Bool(b) => {
                        if *b == when {
                            ip = unsafe { start.add(to as usize) };
                        }
                    }
                    other => fail!(Err(condition_error(other, at!()))),
                },
                Op::JumpCmp { op, jumps, when, a, b, to } => {
                    let jump = match (r!(a), r!(b)) {
                        (Value::Int(x), Value::Int(y)) => jumps >> (x.cmp(y) as i8 + 1) as u8 & 1 != 0,
                        (x, y) => fail!(compare(op, x, y, at!())) == when,
                    };
                    if jump {
                        ip = unsafe { start.add(to as usize) };
                    }
                }
                Op::JumpCmpInt { op, jumps, when, a, n, to } => {
                    let jump = match r!(a) {
                        Value::Int(x) => jumps >> (x.cmp(&(n as i64)) as i8 + 1) as u8 & 1 != 0,
                        x => fail!(compare(op, x, &Value::Int(n as i64), at!())) == when,
                    };
                    if jump {
                        ip = unsafe { start.add(to as usize) };
                    }
                }
                Op::MatchCtor { src, site, fail } => {
                    let s = &chunk!().ctors[site as usize];
                    match r!(src) {
                        Value::Variant(var) if var.ctor.id == s.id && var.fields.len() == s.binds.len() => {
                            for (bind, v) in s.binds.iter().zip(&var.fields) {
                                if let Some(slot) = *bind {
                                    // A slot of a name, never `src`: `var` stays as it is.
                                    w!(slot, v.clone());
                                }
                            }
                        }
                        _ => ip = unsafe { start.add(fail as usize) },
                    }
                }
                Op::Match { src, pat, fail } => {
                    let yes = self.match_slots(chunk!(), pat, base, src);
                    refresh!();
                    if !yes {
                        ip = unsafe { start.add(fail as usize) };
                    }
                }
                Op::Let { src, pat } => {
                    let yes = self.match_slots(chunk!(), pat, base, src);
                    refresh!();
                    if !yes {
                        fail!(Err(pattern_mismatch(at!(), r!(src))));
                    }
                }
                Op::NoCase { src } => fail!(Err(no_case(r!(src), at!()))),
                Op::NoClause => {
                    let def = member_code(clo.expect("a function"), member).0;
                    return Exec::Error(no_clause(def, &self.stack[base..base + code.arity], span), None);
                }
                Op::Stop { info } => {
                    let info = chunk!().stops[info as usize].clone();
                    self.debug_stop(at!(), &info, base);
                    refresh!();
                }

                Op::Call { dst, f, argc } => {
                    // SAFETY: a register of the frame, the only reference to it.
                    let callee = take(unsafe { &mut *regs.add(f as usize) });
                    let v = self.call_fast(callee, None, base + f as usize + 1, argc as usize, at!());
                    refresh!();
                    w!(dst, fail!(v));
                }
                Op::CallGlobal { dst, site } => {
                    let s = &chunk!().calls[site as usize];
                    let (args, argc) = (base + s.args as usize, s.argc as usize);
                    let callee = fail!(self.global(chunk!(), s.g, pc!()));
                    let v = self.call_fast(callee, None, args, argc, at!());
                    refresh!();
                    w!(dst, fail!(v));
                }
                Op::TailCallGlobal { site, test } => {
                    let s = &chunk!().calls[site as usize];
                    let (args, argc) = (s.args as usize, s.argc as usize);
                    // A function that calls itself last starts again here.
                    if let (Some(me), None) = (clo, member) {
                        if argc == code.arity && !test && self.steps_left != 0 && self.is_global(chunk!(), s.g, me) {
                            self.steps_left -= 1;
                            span = at!();
                            self.restart(regs, args, argc, code.chunk.regs);
                            ip = start;
                            continue;
                        }
                    }
                    let callee = fail!(self.global(chunk!(), s.g, pc!()));
                    return Exec::Tail(callee, None, base + args, argc, at!(), test);
                }
                Op::CallSibling { dst, site } => {
                    let s = &chunk!().siblings[site as usize];
                    let (i, args, argc) = (s.i, base + s.args as usize, s.argc as usize);
                    let group = Value::Func(clo.expect("a function of a group").clone());
                    let v = self.call_fast(group, Some(i), args, argc, at!());
                    refresh!();
                    w!(dst, fail!(v));
                }
                Op::MethodFn { site, f } => {
                    let func = self.method_fn(chunk!(), site, base, f, clo, pc!());
                    refresh!();
                    w!(f, fail!(func));
                }
                Op::CallMethod { dst, site, f } => {
                    let (args, argc) = self.method_args(chunk!(), site, base + f as usize + 1);
                    let callee = take(&mut self.stack[base + f as usize]);
                    let v = self.call_member(callee, None, args, argc, at!());
                    refresh!();
                    w!(dst, fail!(v));
                }
                Op::TailCall { f, argc, test } => {
                    let argc = argc as usize;
                    // A function that calls itself last starts again here.
                    if let (Value::Func(c), Some(me), None) = (r!(f), clo, member) {
                        if Rc::ptr_eq(c, me) && argc == code.arity && !test && self.steps_left != 0 {
                            self.steps_left -= 1;
                            span = at!();
                            self.restart(regs, f as usize + 1, argc, code.chunk.regs);
                            ip = start;
                            continue;
                        }
                    }
                    // SAFETY: a register of the frame, the only reference to it.
                    let callee = take(unsafe { &mut *regs.add(f as usize) });
                    return Exec::Tail(callee, None, base + f as usize + 1, argc, at!(), test);
                }
                Op::TailCallSibling { site, test } => {
                    let s = &chunk!().siblings[site as usize];
                    let (i, args, argc) = (s.i, s.args as usize, s.argc as usize);
                    if member == Some(i) && argc == code.arity && !test && self.steps_left != 0 {
                        self.steps_left -= 1;
                        span = at!();
                        self.restart(regs, args, argc, code.chunk.regs);
                        ip = start;
                        continue;
                    }
                    let group = Value::Func(clo.expect("a function of a group").clone());
                    return Exec::Tail(group, Some(i), base + args, argc, at!(), test);
                }
                Op::TailCallMethod { site, f, test } => {
                    let (args, argc) = self.method_args(chunk!(), site, base + f as usize + 1);
                    let callee = take(&mut self.stack[base + f as usize]);
                    return Exec::Tail(callee, None, args, argc, at!(), test);
                }
                // SAFETY: a register of the frame, the only reference to it.
                Op::Return { src } => return Exec::Return(take(unsafe { &mut *regs.add(src as usize) })),
            }
        }
    }

    /// The running function, whose `regs` registers start at `frame`, calls
    /// itself in tail position with the `argc` arguments in the registers
    /// from `args` on: they take the place of its parameters (`args` is above
    /// them), and its other registers are freed.
    #[inline(always)]
    fn restart(&mut self, frame: *mut Value, args: usize, argc: usize, regs: usize) {
        for i in 0..argc {
            // SAFETY: registers of the frame (`Chunk::check`), distinct since `args >= argc`.
            unsafe { std::ptr::swap(frame.add(i), frame.add(args + i)) };
        }
        for i in argc..regs {
            // SAFETY: a register of the frame, the only reference to it.
            let v = unsafe { &mut *frame.add(i) };
            if !is_plain(v) {
                *v = Value::Unit;
            }
        }
    }

    /// The value of the name of the program `chunk.globals[g]` (E003 when it
    /// has none): the one the instruction saw last, while the names of the
    /// program did not change.
    #[inline(always)]
    fn global(&self, chunk: &Chunk, g: u32, at: usize) -> Fallible<Value> {
        let site = &chunk.globals[g as usize];
        if site.seen.get() == self.globals.stamp() {
            return Ok(dup(&site.value.borrow()));
        }
        self.look_up(site, chunk.spans[at].0)
    }

    /// Whether the name of the program `chunk.globals[g]` has the function
    /// `clo` as its value (without a copy of it, when it was seen already).
    #[inline(always)]
    fn is_global(&self, chunk: &Chunk, g: u32, clo: &Rc<Closure>) -> bool {
        let site = &chunk.globals[g as usize];
        if site.seen.get() == self.globals.stamp() {
            return matches!(&*site.value.borrow(), Value::Func(f) if Rc::ptr_eq(f, clo));
        }
        matches!(self.globals.get(&site.name), Some(Value::Func(f)) if Rc::ptr_eq(f, clo))
    }

    #[inline(never)]
    fn look_up(&self, site: &super::GlobalSite, span: Span) -> Fallible<Value> {
        match self.globals.get(&site.name) {
            Some(v) => {
                *site.value.borrow_mut() = v.clone();
                site.seen.set(self.globals.stamp());
                Ok(v.clone())
            }
            None => Err(self.unknown_name(site.name, &site.scope, span)),
        }
    }

    /// Gives the names of the pattern `chunk.pats[pat]` the parts of the
    /// register `src` (the names take other slots than `src`): whether it matches.
    #[inline(never)]
    fn match_slots(&mut self, chunk: &Chunk, pat: u32, base: usize, src: Reg) -> bool {
        let p = &chunk.pats[pat as usize];
        if !may_match(p, &self.stack[base + src as usize]) {
            return false;
        }
        let v = take(&mut self.stack[base + src as usize]);
        let yes = match_pat(p, &v, &mut self.stack[base..], 0);
        self.stack[base + src as usize] = v;
        yes
    }

    /// A new closure of `chunk.lambdas[f]`, with the values it captures from
    /// the frame at `base`, in `dst`.
    #[inline(never)]
    fn closure(&mut self, chunk: &Chunk, f: u32, base: usize, dst: Reg, clo: Option<&Rc<Closure>>) {
        let (def, code) = &chunk.lambdas[f as usize];
        let captured = self.captures(code, base, clo);
        let f = Closure { def: def.clone(), code: code.clone(), captured, siblings: None };
        set(&mut self.stack[base + dst as usize], Value::Func(Rc::new(f)));
    }

    /// `value.name` (`Op::Field`): the field, or else the function to call
    /// with the value, and the register of the argument.
    #[inline(never)]
    fn field_or_fn(
        &mut self,
        chunk: &Chunk,
        site: u32,
        base: usize,
        src: Reg,
        clo: Option<&Rc<Closure>>,
        at: usize,
    ) -> Fallible<Result<Value, (Value, Reg)>> {
        let s = &chunk.fields[site as usize];
        let v = &self.stack[base + src as usize];
        if let Some(x) = field_of(v, &s.key) {
            return Ok(Ok(x));
        }
        // `xs.sum`: no field, so the function `sum`, if there is one.
        let f = match s.func {
            Callee::Global(g) => self.globals.get(&chunk.globals[g as usize].name).cloned(),
            func => Some(self.callee_value(func, base, clo)),
        };
        match f {
            Some(f) if f.is_callable() => {
                let v = v.clone();
                set(&mut self.stack[base + s.arg as usize], v);
                Ok(Err((f, s.arg)))
            }
            _ => Err(self.field(v, &s.key, chunk.spans[at].1).err().expect("no such field")),
        }
    }

    /// The record with the fields of `chunk.shapes[shape]` and the values
    /// from register `first` on, in `dst`.
    #[inline(never)]
    fn record(&mut self, chunk: &Chunk, shape: u32, base: usize, dst: Reg, first: Reg) {
        let shape = &chunk.shapes[shape as usize];
        let start = base + first as usize;
        let vals = self.stack[start..start + shape.len()].iter_mut().map(take).collect();
        set(&mut self.stack[base + dst as usize], Value::Record(Rc::new(Record::with_shape(shape.clone(), vals))));
    }

    /// The fields of `site` of the record in the register `dst` take the
    /// values of their registers (`Op::RecordSets`).
    #[inline(never)]
    fn record_sets(&mut self, site: &super::UpdateSite, base: usize, dst: Reg) {
        let first = base + site.first as usize;
        let mut r = match take(&mut self.stack[base + dst as usize]) {
            Value::Record(r) => r,
            _ => unreachable!("a record being built"),
        };
        let fields = Rc::make_mut(&mut r);
        for (i, key) in site.keys.iter().enumerate() {
            let v = take(&mut self.stack[first + i]);
            fields.set(key.clone(), v);
        }
        self.stack[base + dst as usize] = Value::Record(r);
    }

    /// The field `chunk.strs[key]` of the record at `dst` becomes the value at `src`.
    #[inline(never)]
    fn record_set(&mut self, chunk: &Chunk, key: u32, dst: usize, src: usize) {
        let v = self.stack[src].clone();
        if let Value::Record(r) = &mut self.stack[dst] {
            Rc::make_mut(r).set(chunk.strs[key as usize].clone(), v);
        }
    }

    /// Adds the piece of text `chunk.strs[k]` to the text being built at
    /// `dst`, or starts it with room for `cap` bytes (`Op::TextLit`).
    #[inline(never)]
    fn text_lit(&mut self, chunk: &Chunk, k: u32, cap: u32, dst: usize, span: Span) -> Fallible<()> {
        let piece = &chunk.strs[k as usize];
        if cap > 0 {
            set(&mut self.stack[dst], Value::Text(Text::with_room(piece, cap as usize)));
        } else if let Value::Text(t) = &mut self.stack[dst] {
            t.push_str(piece);
        }
        match &self.stack[dst] {
            Value::Text(t) if t.len() > MAX_TEXT => Err(too_big(false, span)),
            _ => Ok(()),
        }
    }

    /// Adds the value at `src`, shown, to the text being built at `dst` (`Op::TextVal`).
    #[inline(never)]
    fn text_piece(&mut self, dst: usize, src: usize, cap: u32, head: bool, span: Span) -> Fallible<()> {
        if cap > 0 {
            let t = match &self.stack[src] {
                // `"{acc}..."`: the rest is added after the text `acc`,
                // in the room it has when it can.
                Value::Text(t) if head => t.clone(),
                Value::Text(t) => Text::with_room(t, cap as usize),
                v => {
                    let mut s = String::with_capacity(cap as usize);
                    v.write_display(&mut s);
                    Text::from(s)
                }
            };
            set(&mut self.stack[dst], Value::Text(t));
        } else {
            let piece = self.stack[src].clone();
            if let Value::Text(t) = &mut self.stack[dst] {
                match &piece {
                    Value::Text(s) => t.push_str(s),
                    v => {
                        self.scratch.clear();
                        v.write_display(&mut self.scratch);
                        t.push_str(&self.scratch);
                    }
                }
            }
        }
        match &self.stack[dst] {
            Value::Text(t) if t.len() > MAX_TEXT => Err(too_big(false, span)),
            _ => Ok(()),
        }
    }

    /// Adds the elements of the list at `src` to the list being built at
    /// `dst` (`Op::ListSpread`); `spans`: the list, the spread item.
    #[inline(never)]
    fn list_spread(&mut self, dst: usize, src: usize, spans: (Span, Span)) -> Fallible<()> {
        let Value::List(l) = &self.stack[dst] else { unreachable!("a list being built") };
        let before = l.len();
        match &self.stack[src] {
            Value::List(more) if before + more.len() > MAX_LIST => Err(too_big(true, spans.0)),
            Value::List(more) => {
                let more = more.clone();
                if let Value::List(l) = &mut self.stack[dst] {
                    l.extend_mut(more.as_slice());
                }
                Ok(())
            }
            other => Err(spread_error(other, spans.1, true)),
        }
    }

    /// Adds the fields of the record at `src` to the record being built at `dst` (`Op::RecordSpread`).
    #[inline(never)]
    fn record_spread(&mut self, dst: usize, src: usize, first: bool, span: Span) -> Fallible<()> {
        match &self.stack[src] {
            // `{..r, ...}`: a copy of `r` to start with, which shares its names.
            Value::Record(from) if first => {
                let from = Value::Record(from.clone());
                set(&mut self.stack[dst], from);
            }
            Value::Record(from) => {
                let from = from.clone();
                if let Value::Record(r) = &mut self.stack[dst] {
                    let r = Rc::make_mut(r);
                    for (k, v) in from.iter() {
                        r.set(k.clone(), v.clone());
                    }
                }
            }
            other => return Err(spread_error(other, span, false)),
        }
        Ok(())
    }

    /// The local functions of `chunk.groups[g]`, made in their slots (`Op::Group`).
    #[inline(never)]
    fn group(&mut self, chunk: &Chunk, g: u32, base: usize, clo: Option<&Rc<Closure>>) {
        let site = &chunk.groups[g as usize];
        // They all capture the same values.
        let captured = self.captures(&site.group[0].code, base, clo);
        for &(slot, i) in &site.slots {
            let s = &site.group[i as usize];
            let f = Closure { def: s.def.clone(), code: s.code.clone(), captured: captured.clone(), siblings: Some(site.group.clone()) };
            set(&mut self.stack[base + slot as usize], Value::Func(Rc::new(f)));
        }
    }

    /// The values a new closure of `code` captures from the frame at `base`.
    fn captures(&self, code: &FnCode, base: usize, clo: Option<&Rc<Closure>>) -> Vec<Value> {
        code.captures
            .iter()
            .map(|c| match *c {
                crate::compile::Capture::Local(slot) => self.stack[base + slot as usize].clone(),
                crate::compile::Capture::Captured(i) => clo.expect("a closure").captured[i as usize].clone(),
                crate::compile::Capture::Sibling(i) => sibling(clo, i),
            })
            .collect()
    }

    /// The function a local name or a captured value holds (see `Callee`).
    fn callee_value(&self, func: Callee, base: usize, clo: Option<&Rc<Closure>>) -> Value {
        match func {
            Callee::Local(slot) => self.stack[base + slot as usize].clone(),
            Callee::Captured(i) => clo.expect("a closure").captured[i as usize].clone(),
            Callee::Sibling(i) => sibling(clo, i),
            Callee::Global(_) => unreachable!("looked up by the caller"),
        }
    }

    /// The function of `recv.name(...)`, whose receiver is in `f + 1`: its
    /// field `name`, or else the function `name` (`Op::MethodFn`).
    #[inline(never)]
    fn method_fn(&self, chunk: &Chunk, site: u32, base: usize, f: Reg, clo: Option<&Rc<Closure>>, at: usize) -> Fallible<Value> {
        let site = &chunk.methods[site as usize];
        let field = match &self.stack[base + f as usize + 1] {
            Value::Record(r) => r.get(&site.key).cloned(),
            _ => None,
        };
        Ok(match (field, site.func) {
            (Some(func), _) => func,
            (None, Callee::Global(g)) => {
                let global = &chunk.globals[g as usize];
                match self.globals.get(&global.name) {
                    Some(func) => func.clone(),
                    None => return Err(self.no_method(site.name, &global.scope, chunk.spans[at].1)),
                }
            }
            (None, func) => self.callee_value(func, base, clo),
        })
    }

    /// The arguments of a method call whose receiver is at `recv`: after it
    /// when the receiver has a field of the name, else from it on.
    #[inline(never)]
    fn method_args(&mut self, chunk: &Chunk, site: u32, recv: usize) -> (usize, usize) {
        let site = &chunk.methods[site as usize];
        let field = matches!(&self.stack[recv], Value::Record(r) if r.get(&site.key).is_some());
        if field {
            set(&mut self.stack[recv], Value::Unit);
            (recv + 1, site.argc as usize)
        } else {
            (recv, site.argc as usize + 1)
        }
    }

    /// `value.name(...)` when there is no function `name`.
    #[cold]
    #[inline(never)]
    fn no_method(&self, name: crate::symbol::Sym, scope: &crate::compile::Scope, span: Span) -> Box<Diagnostic> {
        let mut d = *self.unknown_name(name, scope, span);
        d.message = t!("no function named `{}`", "aucune fonction nommée `{}`", name);
        d.notes.push(t!("`value.{0}(...)` calls `{0}(value, ...)`", "`valeur.{0}(...)` appelle `{0}(valeur, ...)`", name));
        d.boxed()
    }

    /// The stack of the machine is used beyond its budget.
    #[inline(always)]
    fn stack_too_deep(&self) -> bool {
        stack_position().abs_diff(self.stack_base) > STACK_BUDGET.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// How many calls may nest, and how many values their frames may hold.
#[inline(always)]
fn call_limits() -> (usize, usize) {
    let budget = STACK_BUDGET.load(std::sync::atomic::Ordering::Relaxed);
    let calls = MAX_DEPTH.load(std::sync::atomic::Ordering::Relaxed).min(budget / CALL_COST);
    (calls, budget / std::mem::size_of::<Value>())
}

/// The function number `i` of the group of the closure `clo`, as a closure
/// of its own, which captures the same values.
#[inline(never)]
fn sibling(clo: Option<&Rc<Closure>>, i: u32) -> Value {
    let clo = clo.expect("a function of a group");
    let group = clo.siblings.as_ref().expect("a function of a group");
    let s = &group[i as usize];
    Value::Func(Rc::new(Closure {
        def: s.def.clone(),
        code: s.code.clone(),
        captured: clo.captured.clone(),
        siblings: Some(group.clone()),
    }))
}

/// A comparison of any two values, true or false.
#[inline(never)]
fn compare(op: BinOp, a: &Value, b: &Value, span: Span) -> Fallible<bool> {
    match binop(op, a, b, span)? {
        Value::Bool(b) => Ok(b),
        other => Err(condition_error(&other, span)),
    }
}
