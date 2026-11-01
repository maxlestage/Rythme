//! The bytecode virtual machine: functions run as lists of instructions on
//! registers, instead of walking the syntax tree.
//!
//! - [`gen`] turns the tree prepared by `compile` (where each local name is
//!   already a numbered slot) into instructions, once per function;
//! - [`run`] runs them.
//!
//! **Registers.** Each running function has a frame on the value stack of the
//! interpreter: its arguments in the first slots, then its local names (the
//! slots of `compile`), then the temporary values of its expressions. An
//! instruction names the registers it reads and writes (`Binary { op, dst,
//! a, b }`), so that `n - 1` or `x < v` is one instruction that reads its
//! operands where they are, without copying them: values hold reference
//! counts, and every copy avoided is work saved. (A stack machine would need
//! three instructions and two copies for `n - 1`.)
//!
//! **Calls.** A call of a function of the program pushes a frame on the value
//! stack and runs the callee's instructions in a nested `run::exec` (a loop
//! that keeps every frame on the value stack was tried: in safe Rust, it
//! keeps less in the machine's registers and was slower). The arguments of
//! a call are evaluated into the last registers of the caller, which become
//! the first slots of the callee: they are not copied. A call in tail
//! position puts its arguments in place of the frame of its caller, so that
//! a function that calls itself (or another one) last loops forever without
//! growing either stack. Built-in functions that call functions of the
//! program (`map`, `fold`...) enter `exec` again (`Interp::call_given`). The
//! limit of nested calls (E014) counts every call, and the machine's stack
//! is measured at each one (`interp::set_stack_budget`).
//!
//! **Same behavior.** Every instruction does what the tree-walking
//! interpreter of Rythme 1.0 did, in the same order: the same values, the
//! same errors with the same places, the same calls seen by the debugger and
//! counted by the limit of calls (`Interp::set_step_limit`).

use std::rc::Rc;

use crate::ast::{BinOp, FnDef, UnOp};
use crate::compile::{FnCode, Pat, Scope, Sibling, StopInfo};
use crate::source::Span;
use crate::symbol::Sym;
use crate::value::{Shape, Value};

pub mod gen;
pub mod run;
mod stack;

pub use stack::Stack;

/// A register: a slot of the frame of the running function.
pub type Reg = u32;

/// A position in the instructions of a chunk.
pub type Pc = u32;

/// One instruction. The places used by the messages of its errors are kept
/// beside the instructions (`Chunk::spans`), and what does not fit in a few
/// numbers in tables of the chunk (`Chunk::fields`...): instructions stay
/// small, which keeps the loop that runs them fast.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    // ----- values ------------------------------------------------------
    /// `dst = src`
    Move {
        dst: Reg,
        src: Reg,
    },
    /// `dst = consts[k]`
    Const {
        dst: Reg,
        k: u32,
    },
    /// The value number `i` captured by the closure of the running function.
    Captured {
        dst: Reg,
        i: u32,
    },
    /// A name of the program, `globals[g]` (E003 when it has no value).
    Global {
        dst: Reg,
        g: u32,
    },
    /// The function number `i` of the group of the running function, as a value.
    Sibling {
        dst: Reg,
        i: u32,
    },
    /// A new closure of `lambdas[f]`, with the values it captures from this frame.
    Closure {
        dst: Reg,
        f: u32,
    },
    /// The local functions `groups[g]`, written one after the other, in their slots.
    Group {
        g: u32,
    },
    /// The running function, in the slot where it sees itself by its name.
    SetSelf {
        dst: Reg,
    },

    // ----- operators ---------------------------------------------------
    /// `dst = a op b` (every operator but `and` and `or`); span: the operator.
    Binary {
        op: BinOp,
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    /// `dst = a op n`, for a small whole number written in the code (`n * 2`).
    BinaryInt {
        op: BinOp,
        dst: Reg,
        a: Reg,
        n: i32,
    },
    /// `dst = a + b`: the most common operator, on its own.
    Add {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    /// `dst = a + n` (`acc + 1`).
    AddInt {
        dst: Reg,
        a: Reg,
        n: i32,
    },
    /// `dst = a - n` (`n - 1`).
    SubInt {
        dst: Reg,
        a: Reg,
        n: i32,
    },
    /// `-x`, `not x`; spans: the expression, the operand.
    Unary {
        op: UnOp,
        dst: Reg,
        src: Reg,
    },
    /// `obj[idx]`; spans: the expression, the index.
    Index {
        dst: Reg,
        obj: Reg,
        idx: Reg,
    },
    /// `a..b`
    Range {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    /// `src?`: the value inside `Some` / `Ok`, or the running function
    /// returns `src` (`None`, `Err`).
    Try {
        dst: Reg,
        src: Reg,
    },
    /// `src.name`: a field, or the function `name` called with `src`
    /// (`fields[site]`); spans: the expression, the name.
    Field {
        dst: Reg,
        src: Reg,
        site: u32,
    },

    // ----- building values ---------------------------------------------
    /// The list of the `n` values from register `first` on (taken from them).
    List {
        dst: Reg,
        first: Reg,
        n: u32,
    },
    /// An empty list with room for `cap` elements, to add to.
    ListNew {
        dst: Reg,
        cap: u32,
    },
    /// `[..src, ...]`: the list `src`, to add to; span: `..src`.
    ListFrom {
        dst: Reg,
        src: Reg,
    },
    /// Adds `src` at the end of the list `dst`.
    ListPush {
        dst: Reg,
        src: Reg,
    },
    /// Adds the elements of the list `src` at the end of `dst`; spans: the list, `..src`.
    ListSpread {
        dst: Reg,
        src: Reg,
    },
    /// The list `dst` is not too long (after `ListFrom`); span: the list.
    ListCheck {
        dst: Reg,
    },
    /// The record with the fields of `shapes[shape]` and the values from register `first` on.
    Record {
        dst: Reg,
        shape: u32,
        first: Reg,
    },
    /// An empty record, to add fields to.
    RecordNew {
        dst: Reg,
    },
    /// The field `strs[key]` of the record `dst` becomes `src`.
    RecordSet {
        dst: Reg,
        key: u32,
        src: Reg,
    },
    /// The fields of `updates[site]` of the record `dst` take the values of
    /// their registers (`{..r, a: x, b: y}`).
    RecordSets {
        dst: Reg,
        site: u32,
    },
    /// The fields of the record `src` in the record `dst` (`first`: it is a
    /// copy of `src`); span: `..src`.
    RecordSpread {
        dst: Reg,
        src: Reg,
        first: bool,
    },
    /// Adds the text `strs[k]` at the end of the text `dst`; with `cap`, `dst`
    /// starts with it, with room for `cap` bytes. Span: the text.
    TextLit {
        dst: Reg,
        k: u32,
        cap: u32,
    },
    /// Adds the value of `src`, shown, at the end of the text `dst`; with
    /// `cap`, `dst` starts with it (with `head`, a text `src` itself, to add
    /// to in the room after it). Span: the text.
    TextVal {
        dst: Reg,
        src: Reg,
        cap: u32,
        head: bool,
    },

    // ----- control -----------------------------------------------------
    Jump {
        to: Pc,
    },
    /// Jumps when `cond` is `when`; E005 when it is not a Bool. Span: the condition.
    JumpIf {
        cond: Reg,
        when: bool,
        to: Pc,
    },
    /// Jumps when the comparison `a op b` is `when`; span: the operator.
    /// `jumps`: for whole numbers, whether it jumps when `a` is less than,
    /// equal to or greater than `b` (bits 0, 1, 2), found when it is made.
    JumpCmp {
        op: BinOp,
        jumps: u8,
        when: bool,
        a: Reg,
        b: Reg,
        to: Pc,
    },
    /// Jumps when the comparison `a op n` is `when`; span: the operator.
    JumpCmpInt {
        op: BinOp,
        jumps: u8,
        when: bool,
        a: Reg,
        n: i32,
        to: Pc,
    },
    /// Gives the names of the pattern `pats[pat]` the parts of `src`, or
    /// jumps to `fail` when it does not match.
    Match {
        src: Reg,
        pat: u32,
        fail: Pc,
    },
    /// The same for a constructor whose parts are names or `_` (`ctors[site]`).
    MatchCtor {
        src: Reg,
        site: u32,
        fail: Pc,
    },
    /// `pattern = src`; E006 when it does not match. Span: the pattern.
    Let {
        src: Reg,
        pat: u32,
    },
    /// No case of a `match` handles `src`; span: the value matched.
    NoCase {
        src: Reg,
    },
    /// No definition of the running function accepts its arguments.
    NoClause,
    /// A place where the debugger may pause (`stops[info]`), shown at the span.
    Stop {
        info: u32,
    },

    // ----- calls -------------------------------------------------------
    /// Calls the function in register `f` with the `argc` values after it;
    /// the result goes to `dst`. Span: the call.
    Call {
        dst: Reg,
        f: Reg,
        argc: u32,
    },
    /// The same call in tail position: the callee replaces the running
    /// function. `test`: its result must be true or false (the right side of
    /// `and` / `or`).
    TailCall {
        f: Reg,
        argc: u32,
        test: bool,
    },
    /// Calls the function named by `calls[site]`, a name of the program that
    /// surely has a value: read at the call, after the arguments.
    CallGlobal {
        dst: Reg,
        site: u32,
    },
    TailCallGlobal {
        site: u32,
        test: bool,
    },
    /// Calls a function of the group of the running function (`siblings[site]`).
    CallSibling {
        dst: Reg,
        site: u32,
    },
    TailCallSibling {
        site: u32,
        test: bool,
    },
    /// `recv.name(args)`, with `recv` in `f + 1`: puts in `f` the field `name`
    /// of `recv`, or else the function `name` (`methods[site]`). Spans: the
    /// call, the name.
    MethodFn {
        site: u32,
        f: Reg,
    },
    /// Calls the function of `MethodFn` with the arguments after `f + 1`
    /// (and `recv` first, when it is not a field).
    CallMethod {
        dst: Reg,
        site: u32,
        f: Reg,
    },
    TailCallMethod {
        site: u32,
        f: Reg,
        test: bool,
    },
    /// The running function gives `src` as its result.
    Return {
        src: Reg,
    },
}

/// Where a function called by `Op::Field` or `Op::MethodFn` comes from.
#[derive(Clone, Copy, Debug)]
pub enum Callee {
    /// A name of the program: `globals[g]`.
    Global(u32),
    Local(Reg),
    Captured(u32),
    Sibling(u32),
}

/// `value.name` (see `Op::Field`).
pub struct FieldSite {
    pub key: Rc<str>,
    pub func: Callee,
    /// The register for the value when the function is called with it.
    pub arg: Reg,
    /// The fields of the last record read here, and the position of `key`
    /// in them: the same for every record with the same fields.
    pub seen: ShapeSeen,
}

/// The positions of names in the fields of records with one shape, kept by
/// an instruction (`FieldSite`, `UpdateSite`): records with the same fields
/// (`Rc<Shape>`) have each name at the same position. The shape is kept
/// alive, so that its address is never one of another shape; and a shape
/// kept here is shared, so it never changes (`Rc::make_mut` copies it).
pub struct ShapeSeen {
    /// Its address, only compared.
    pub addr: std::cell::Cell<*const Shape>,
    pub shape: std::cell::RefCell<Option<Rc<Shape>>>,
    pub at: std::cell::RefCell<Vec<usize>>,
}

impl Default for ShapeSeen {
    fn default() -> ShapeSeen {
        ShapeSeen { addr: std::cell::Cell::new(std::ptr::null()), shape: Default::default(), at: Default::default() }
    }
}

impl ShapeSeen {
    /// The positions seen for the fields of `r`, if they are the same as last time.
    #[inline(always)]
    pub fn get(&self, r: &crate::value::Record) -> Option<std::cell::Ref<'_, Vec<usize>>> {
        std::ptr::eq(self.addr.get(), Rc::as_ptr(r.shape())).then(|| self.at.borrow())
    }

    /// Keeps the positions `at` of the fields of `r`.
    pub fn keep(&self, r: &crate::value::Record, at: Vec<usize>) {
        self.addr.set(Rc::as_ptr(r.shape()));
        *self.shape.borrow_mut() = Some(r.shape().clone());
        *self.at.borrow_mut() = at;
    }
}

/// `value.name(args)` (see `Op::MethodFn`).
pub struct MethodSite {
    /// The name as written (for the message when there is no such function).
    pub name: Sym,
    /// The name as a record key.
    pub key: Rc<str>,
    pub func: Callee,
    /// The number of arguments written.
    pub argc: u32,
}

/// A name of the program read by the code (`Op::Global`).
pub struct GlobalSite {
    pub name: Sym,
    /// The local names visible there (for "did you mean").
    pub scope: Scope,
    /// Its value, read when the names of the program had this stamp
    /// (`Globals::stamp`): still its value while they have it.
    pub seen: std::cell::Cell<u64>,
    pub value: std::cell::RefCell<Seen>,
    /// The address of the function of `value` (null when it is not one), to
    /// tell at once whether a call is a call of the running function. It is
    /// only compared, never followed.
    pub func: std::cell::Cell<*const crate::value::Closure>,
}

/// A value of a name of the program, as an instruction keeps it. A function
/// is kept without counting as a reference, since its code may be the code
/// that keeps it (a cycle would never be freed); values that may hold
/// functions are not kept.
#[derive(Default)]
pub enum Seen {
    #[default]
    Nothing,
    Plain(Value),
    Func(std::rc::Weak<crate::value::Closure>),
}

/// A call of a function of the group of the running function.
pub struct SiblingSite {
    /// Its position in the group.
    pub i: u32,
    /// The register of the first argument.
    pub args: Reg,
    pub argc: u32,
}

/// A call of a name of the program that surely has a value (`Op::CallGlobal`).
pub struct CallSite {
    /// The name: `globals[g]`.
    pub g: u32,
    /// The register of the first argument.
    pub args: Reg,
    pub argc: u32,
}

/// A constructor pattern whose parts are names or `_` (`Op::MatchCtor`).
pub struct CtorSite {
    /// The constructor, as the interpreter stores it (`CtorInfo::id`).
    pub id: Sym,
    /// For each field, the slot of its name (none for `_`).
    pub binds: Box<[Option<Reg>]>,
}

/// The fields set by `{..r, a: x, b: y}` (`Op::RecordSets`).
pub struct UpdateSite {
    pub keys: Box<[Rc<str>]>,
    /// The register of the value of the first one; the others follow.
    pub first: Reg,
    /// The positions of `keys` in the fields of the last record updated here.
    pub seen: ShapeSeen,
}

/// Local functions written one after the other (see `compile::Stmt::Group`).
pub struct GroupSite {
    /// The slot of each named one, with its position in the group.
    pub slots: Vec<(u32, u32)>,
    pub group: Rc<[Sibling]>,
}

/// The instructions of a function or of a top-level expression, and what
/// they refer to.
#[derive(Default)]
pub struct Chunk {
    pub ops: Vec<Op>,
    /// For each instruction, the places shown by its errors (see `Op`): the
    /// first one, and a second one for the few that need two.
    pub spans: Vec<(Span, Span)>,
    /// The size of the frame: arguments, local names and temporary values.
    pub regs: usize,
    pub consts: Vec<Value>,
    /// The names of the program used.
    pub globals: Vec<GlobalSite>,
    /// Texts: names of record fields, pieces of texts.
    pub strs: Vec<Rc<str>>,
    pub pats: Vec<Pat>,
    pub shapes: Vec<Rc<Shape>>,
    pub lambdas: Vec<(Rc<FnDef>, Rc<FnCode>)>,
    pub groups: Vec<GroupSite>,
    pub fields: Vec<FieldSite>,
    pub methods: Vec<MethodSite>,
    pub siblings: Vec<SiblingSite>,
    pub calls: Vec<CallSite>,
    pub ctors: Vec<CtorSite>,
    pub updates: Vec<UpdateSite>,
    pub stops: Vec<Rc<StopInfo>>,
    /// Where the instructions that choose a definition of the function are
    /// (its tests and guards, `start..end`): an error there does not show
    /// the function in the trace of calls, since its body did not start.
    pub choosing: Vec<(Pc, Pc)>,
}

impl Chunk {
    /// Checks what the loop of `run` relies on to read instructions and
    /// registers without checking each time: the chunk ends with an
    /// instruction that does not go on to the next one, every jump goes to an
    /// instruction, and every register named is a register of the frame.
    /// (A failure is a mistake of `gen`.)
    pub fn check(&self) {
        let n = self.ops.len();
        assert!(matches!(
            self.ops.last(),
            Some(
                Op::Return { .. }
                    | Op::TailCall { .. }
                    | Op::TailCallGlobal { .. }
                    | Op::TailCallSibling { .. }
                    | Op::TailCallMethod { .. }
                    | Op::NoCase { .. }
                    | Op::NoClause
                    | Op::Jump { .. }
            )
        ));
        assert_eq!(self.spans.len(), n);
        let regs = self.regs;
        // The registers `from..to` are in the frame.
        let range = |from: Reg, to: u64| assert!(from as u64 <= to && to <= regs as u64, "registers {from}..{to} of {regs}");
        let one = |r: Reg| range(r, r as u64 + 1);
        let target = |to: Pc| assert!((to as usize) < n, "a jump to {to} of {n}");
        for op in &self.ops {
            match *op {
                Op::Move { dst, src } => {
                    one(dst);
                    one(src);
                }
                Op::Const { dst, k } => {
                    one(dst);
                    assert!((k as usize) < self.consts.len());
                }
                Op::Captured { dst, .. }
                | Op::Global { dst, .. }
                | Op::Sibling { dst, .. }
                | Op::Closure { dst, .. }
                | Op::SetSelf { dst }
                | Op::ListNew { dst, .. }
                | Op::ListCheck { dst }
                | Op::RecordNew { dst }
                | Op::TextLit { dst, .. } => one(dst),
                Op::Group { g } => self.groups[g as usize].slots.iter().for_each(|&(slot, _)| one(slot)),
                Op::Binary { dst, a, b, .. } | Op::Add { dst, a, b } | Op::Index { dst, obj: a, idx: b } | Op::Range { dst, a, b } => {
                    one(dst);
                    one(a);
                    one(b);
                }
                Op::BinaryInt { dst, a, .. }
                | Op::AddInt { dst, a, .. }
                | Op::SubInt { dst, a, .. }
                | Op::Unary { dst, src: a, .. }
                | Op::Try { dst, src: a }
                | Op::ListFrom { dst, src: a }
                | Op::ListPush { dst, src: a }
                | Op::ListSpread { dst, src: a }
                | Op::RecordSet { dst, src: a, .. }
                | Op::RecordSpread { dst, src: a, .. }
                | Op::TextVal { dst, src: a, .. } => {
                    one(dst);
                    one(a);
                }
                Op::Field { dst, src, site } => {
                    one(dst);
                    one(src);
                    one(self.fields[site as usize].arg);
                }
                Op::List { dst, first, n } => {
                    one(dst);
                    range(first, first as u64 + n as u64);
                }
                Op::Record { dst, shape, first } => {
                    one(dst);
                    range(first, first as u64 + self.shapes[shape as usize].len() as u64);
                }
                Op::Jump { to } => target(to),
                Op::JumpIf { cond, to, .. } => {
                    one(cond);
                    target(to);
                }
                Op::JumpCmp { a, b, to, .. } => {
                    one(a);
                    one(b);
                    target(to);
                }
                Op::JumpCmpInt { a, to, .. } => {
                    one(a);
                    target(to);
                }
                Op::Match { src, fail, .. } => {
                    one(src);
                    target(fail);
                }
                Op::MatchCtor { src, site, fail } => {
                    one(src);
                    target(fail);
                    self.ctors[site as usize].binds.iter().flatten().for_each(|&slot| one(slot));
                }
                Op::RecordSets { dst, site } => {
                    one(dst);
                    let s = &self.updates[site as usize];
                    range(s.first, s.first as u64 + s.keys.len() as u64);
                }
                Op::CallGlobal { dst, site } => {
                    one(dst);
                    let s = &self.calls[site as usize];
                    range(s.args, s.args as u64 + s.argc as u64);
                }
                Op::TailCallGlobal { site, .. } => {
                    let s = &self.calls[site as usize];
                    range(s.args, s.args as u64 + s.argc as u64);
                }
                Op::Let { src, .. } | Op::NoCase { src } | Op::Return { src } => one(src),
                Op::NoClause | Op::Stop { .. } => {}
                Op::Call { dst, f, argc } => {
                    one(dst);
                    range(f, f as u64 + 1 + argc as u64);
                }
                Op::TailCall { f, argc, .. } => range(f, f as u64 + 1 + argc as u64),
                Op::CallSibling { dst, site } => {
                    one(dst);
                    let s = &self.siblings[site as usize];
                    range(s.args, s.args as u64 + s.argc as u64);
                }
                Op::TailCallSibling { site, .. } => {
                    let s = &self.siblings[site as usize];
                    range(s.args, s.args as u64 + s.argc as u64);
                }
                Op::MethodFn { site, f } | Op::TailCallMethod { site, f, .. } => {
                    range(f, f as u64 + 2 + self.methods[site as usize].argc as u64);
                }
                Op::CallMethod { dst, site, f } => {
                    one(dst);
                    range(f, f as u64 + 2 + self.methods[site as usize].argc as u64);
                }
            }
        }
    }

    /// The place of the instruction at `pc`.
    #[inline(never)]
    pub fn span(&self, pc: usize) -> Span {
        self.spans[pc].0
    }

    /// The second place of the instruction at `pc` (see `Op`).
    #[inline(never)]
    pub fn span2(&self, pc: usize) -> Span {
        self.spans[pc].1
    }

    /// Whether the instruction at `pc` chooses a definition (see `choosing`).
    pub fn chooses(&self, pc: usize) -> bool {
        self.choosing.iter().any(|&(start, end)| (start as usize..end as usize).contains(&pc))
    }
}

/// For `Op::JumpCmp`: whether a comparison `op` of whole numbers jumps when
/// it is `when`, for each order of its operands (bit 0: less, 1: equal, 2: greater).
pub fn jumps(op: BinOp, when: bool) -> u8 {
    let holds = |o: std::cmp::Ordering| match op {
        BinOp::Eq => o.is_eq(),
        BinOp::Ne => o.is_ne(),
        BinOp::Lt => o.is_lt(),
        BinOp::Le => o.is_le(),
        BinOp::Gt => o.is_gt(),
        _ => o.is_ge(),
    };
    [std::cmp::Ordering::Less, std::cmp::Ordering::Equal, std::cmp::Ordering::Greater]
        .iter()
        .enumerate()
        .map(|(i, &o)| u8::from(holds(o) == when) << i)
        .sum()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::Op;
    use crate::interp::{Interp, Mode};

    /// The name of each kind of instruction, in the order of `kind`.
    const KINDS: [&str; 51] = [
        "Move",
        "Const",
        "Captured",
        "Global",
        "Sibling",
        "Closure",
        "Group",
        "SetSelf",
        "Binary",
        "BinaryInt",
        "Add",
        "AddInt",
        "SubInt",
        "Unary",
        "Index",
        "Range",
        "Try",
        "Field",
        "List",
        "ListNew",
        "ListFrom",
        "ListPush",
        "ListSpread",
        "ListCheck",
        "Record",
        "RecordNew",
        "RecordSet",
        "RecordSets",
        "RecordSpread",
        "TextLit",
        "TextVal",
        "Jump",
        "JumpIf",
        "JumpCmp",
        "JumpCmpInt",
        "Match",
        "MatchCtor",
        "Let",
        "NoCase",
        "NoClause",
        "Stop",
        "Call",
        "TailCall",
        "CallGlobal",
        "TailCallGlobal",
        "CallSibling",
        "TailCallSibling",
        "MethodFn",
        "CallMethod",
        "TailCallMethod",
        "Return",
    ];

    /// The number of the kind of `op` in `KINDS` (a new instruction must be added to both).
    fn kind(op: &Op) -> usize {
        match op {
            Op::Move { .. } => 0,
            Op::Const { .. } => 1,
            Op::Captured { .. } => 2,
            Op::Global { .. } => 3,
            Op::Sibling { .. } => 4,
            Op::Closure { .. } => 5,
            Op::Group { .. } => 6,
            Op::SetSelf { .. } => 7,
            Op::Binary { .. } => 8,
            Op::BinaryInt { .. } => 9,
            Op::Add { .. } => 10,
            Op::AddInt { .. } => 11,
            Op::SubInt { .. } => 12,
            Op::Unary { .. } => 13,
            Op::Index { .. } => 14,
            Op::Range { .. } => 15,
            Op::Try { .. } => 16,
            Op::Field { .. } => 17,
            Op::List { .. } => 18,
            Op::ListNew { .. } => 19,
            Op::ListFrom { .. } => 20,
            Op::ListPush { .. } => 21,
            Op::ListSpread { .. } => 22,
            Op::ListCheck { .. } => 23,
            Op::Record { .. } => 24,
            Op::RecordNew { .. } => 25,
            Op::RecordSet { .. } => 26,
            Op::RecordSets { .. } => 27,
            Op::RecordSpread { .. } => 28,
            Op::TextLit { .. } => 29,
            Op::TextVal { .. } => 30,
            Op::Jump { .. } => 31,
            Op::JumpIf { .. } => 32,
            Op::JumpCmp { .. } => 33,
            Op::JumpCmpInt { .. } => 34,
            Op::Match { .. } => 35,
            Op::MatchCtor { .. } => 36,
            Op::Let { .. } => 37,
            Op::NoCase { .. } => 38,
            Op::NoClause => 39,
            Op::Stop { .. } => 40,
            Op::Call { .. } => 41,
            Op::TailCall { .. } => 42,
            Op::CallGlobal { .. } => 43,
            Op::TailCallGlobal { .. } => 44,
            Op::CallSibling { .. } => 45,
            Op::TailCallSibling { .. } => 46,
            Op::MethodFn { .. } => 47,
            Op::CallMethod { .. } => 48,
            Op::TailCallMethod { .. } => 49,
            Op::Return { .. } => 50,
        }
    }

    thread_local! {
        /// The kinds of instructions run by this thread (bit `kind`).
        static SEEN: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }

    /// Called by `run` before each instruction, in tests.
    pub(crate) fn saw(op: &Op) {
        SEEN.with(|s| s.set(s.get() | 1 << kind(op)));
    }

    /// The kinds of instructions this thread did not run.
    fn not_seen() -> Vec<&'static str> {
        let seen = SEEN.with(|s| s.get());
        KINDS.iter().enumerate().filter(|&(i, _)| seen & 1 << i == 0).map(|(_, k)| *k).collect()
    }

    #[test]
    fn instructions_stay_small() {
        assert!(std::mem::size_of::<super::Op>() <= 16, "{}", std::mem::size_of::<super::Op>());
    }

    /// The value of the last line of `code`, or its error (with its code).
    fn run(code: &str) -> String {
        let file = crate::source::add("vm.ry", code);
        let items = crate::parser::parse(crate::lexer::lex(code, file).unwrap()).unwrap();
        let mut it = Interp::new();
        match it.exec_items(&items, Mode::Run) {
            Ok(v) => v.map(|v| v.repr()).unwrap_or_default(),
            Err(d) => format!("error[{}] {}", d.code, d.trace.len()),
        }
    }

    /// Steps through `code` under the debugger, reading the frames, the local
    /// names and an expression at each pause: its value and how many pauses.
    fn debug(code: &str) -> String {
        use crate::debug::{Breakpoints, Frontend, Paused, Resume, Session};
        struct Steps(std::rc::Rc<std::cell::Cell<usize>>);
        impl Frontend for Steps {
            fn paused(&mut self, p: &mut Paused) -> Resume {
                self.0.set(self.0.get() + 1);
                let frames = p.frames();
                for i in 0..frames.len() {
                    let _ = p.locals(i);
                }
                let _ = p.eval(0, "1 + 2");
                // Into the calls, then over them, then out of them.
                [Resume::Step, Resume::Step, Resume::Next, Resume::Finish][self.0.get() % 4]
            }
        }
        let pauses = std::rc::Rc::new(std::cell::Cell::new(0));
        let file = crate::source::add("debugged.ry", code);
        let items = crate::parser::parse(crate::lexer::lex(code, file).unwrap()).unwrap();
        let mut it = Interp::new();
        it.watched = true;
        it.steps_left = 0;
        it.debugger = Some(Box::new(Session::new(Box::new(Steps(pauses.clone())), Breakpoints::default(), Resume::Step, true)));
        let r = match it.exec_items(&items, Mode::Run) {
            Ok(v) => format!("ok {}", v.map(|v| v.repr()).unwrap_or_default()),
            Err(d) => format!("error[{}]", d.code),
        };
        format!("{r}, pauses {}", pauses.get())
    }

    /// Programs that go through every kind of instruction: run under Miri
    /// (`cargo +nightly miri test --lib vm`), they check the unsafe code of `run`.
    #[test]
    fn programs_run_through_the_registers() {
        let cases = [
            ("fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)\nfib(12)", "144"),
            ("fn count(0, acc) = acc\nfn count(n, acc) = count(n - 1, acc + 1)\ncount(300, 0)", "300"),
            ("fn add(n) = fn(x) = x + n\nmap([1, 2, 3], add(10))", "[11, 12, 13]"),
            ("type T = L | N(l, v, r)\nfn ins(L, x) = N(L, x, L)\nfn ins(N(l, v, r), x) = if x < v then N(ins(l, x), v, r) else N(l, v, ins(r, x))\nfn sum(L) = 0\nfn sum(N(l, v, r)) = sum(l) + v + sum(r)\nsum(fold([5, 3, 8, 1], L, ins))", "17"),
            ("r = {a: 1, b: [2]}\ns = {..r, a: r.a + 1, c: \"x{r.b}\"}\n[s, s.c, s[\"a\"]]", "[{a: 2, b: [2], c: \"x[2]\"}, \"x[2]\", 2]"),
            ("fn f(xs)\n  [a, ..rest] = xs\n  m = match a\n    0 -> \"zero\"\n    k if k > 1 -> \"big {k}\"\n    _ -> \"one\"\n  end\n  [m, ..rest, len(rest)]\nend\nf([2, 7, 9])", "[\"big 2\", 7, 9, 2]"),
            ("fn even(0) = true\nfn even(n) = odd(n - 1)\nfn odd(0) = false\nfn odd(n) = even(n - 1)\n[even(10), odd(7), not even(3)]", "[true, true, true]"),
            ("fn g(n)\n  fn a(0) = 0\n  fn a(k) = b(k - 1) + 1\n  fn b(k) = a(k)\n  a(n)\nend\n[g(20), 1..3, [1, 2].sum, \"ab\".upper()]", "[20, [1, 2, 3], 3, \"AB\"]"),
            ("fn h(x) = Some(x)?\nfn k(x) = None?\n[h(1), k(2)]", "[1, None]"),
            ("fn bad(n) = n + \"a\"\nfn outer(n) = 1 + bad(n)\nouter(1)", "error[E005] 2"),
            ("fn deep(n) = 1 + deep(n + 1)\ndeep(0)", "error[E014] 40"),
            (
                "fn outer(n)\n  fn a(k) = k + 1\n  fn b(k) = map([k, n], a)\n  go = fn(k) = if k == 0 then 0 else go(k - 1)\n  [b(n), go(3)]\nend\n\
                 fn app(f, x) = f(x) * x\nfn up(s) = s.upper()\nfn pick(b, x) = if b then x * 2 else x == 3\n\
                 fn ends(xs, r) = [[..xs, 1], {a: len(xs), ..r}, {..r, b: 2, c: 3}]\nfn firsts([a, ..rest]) = a\nfn firsts([]) = 0\n\
                 [outer(4), app(fn(x) = x + 1, 5), up(\"ab\"), pick(true, 4), pick(false, 3), ends([7], {b: 1}), firsts([5, 6]), firsts([])]",
                "[[[5, 5], 0], 30, \"AB\", 8, true, [[7, 1], {a: 1, b: 1}, {b: 2, c: 3}], 5, 0]",
            ),
            ("fn f(0) = 1\nfn g(n) = f(n) + 1\ng(2)", "error[E006] 1"),
            // The positions of fields kept by the instructions that read and update them.
            (
                "rs = [{a: 1, b: 0}, {a: 2, b: 0}, {b: 5, a: 3}]\n[fold(rs, 0, fn(acc, r) = acc + r.a), fold(1..3, {n: 0, m: 1}, fn(s, i) = {..s, n: s.n + i})]",
                "[6, {n: 6, m: 1}]",
            ),
            ("x = Some(2)\nmatch x\n  None -> 0\nend", "error[E006] 0"),
        ];
        // More stack than the budget of nested calls (4 MB), as `rythme` has.
        let runs = std::thread::Builder::new()
            .stack_size(64 << 20)
            .spawn(move || {
                let got = cases.map(|(code, _)| run(code));
                // The places where the debugger pauses, with what it reads there.
                let debugged = debug(cases[7].0);
                (got, debugged, not_seen())
            })
            .unwrap();
        let (got, debugged, missing) = runs.join().unwrap();
        assert!(debugged.starts_with("ok [20, [1, 2, 3], 3, \"AB\"], pauses "), "{debugged}");
        for ((code, expected), got) in cases.iter().zip(got) {
            assert_eq!(&got, expected, "{code}");
        }
        assert!(missing.is_empty(), "instructions that no program runs: {missing:?}");
    }
}
