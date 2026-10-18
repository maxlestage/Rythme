//! What the checker knows for sure about the values of expressions: their
//! types.
//!
//! Rythme checks values while the program runs, but many expressions have a
//! type that is certain before: `"age: "` is always a Text, `len(x)` always
//! gives an Int. The checker infers these types to report, before running,
//! operations that would fail every time they run, like `"age: " + 36`.
//!
//! A type only says what is certain. When nothing is known, it is `Any`, and
//! nothing is reported: a correct program must never be rejected.

use std::rc::Rc;

use crate::builtins::{Builtin, Expected};
use crate::symbol::Sym;
use crate::value::Value;

/// What is known about the values an expression can give.
#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    /// No value at all: the expression never gives one (it stops the
    /// program), or nothing is known yet while the types are being inferred.
    Never,
    /// Any value: nothing is known for sure.
    Any,
    Unit,
    Bool,
    Int,
    Float,
    /// A number: an Int or a Float.
    Num,
    Text,
    /// `:ok`, `:error`...
    Symbol,
    /// A list whose elements all have this type.
    List(Rc<Ty>),
    Record(Rc<RecordTy>),
    /// A value of a type defined with `type`, or an Option, or a Result.
    Data(Rc<DataTy>),
    Func(Rc<FuncTy>),
}

/// A record: fields it has for sure, with their types.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordTy {
    pub fields: Vec<(Rc<str>, Ty)>,
    /// The record has exactly these fields, and no other.
    pub exact: bool,
}

/// A value of the type `name`.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTy {
    pub name: Sym,
    /// The constructors it may have been built with, and the types of their
    /// fields; `None` when it may be any constructor of the type.
    pub ctors: Option<Vec<(Sym, Vec<Ty>)>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncTy {
    /// How many arguments it takes, when known.
    pub arity: Option<(usize, usize)>,
    /// What it gives back.
    pub ret: Ty,
    /// How to call it, for messages: `area(shape)`.
    pub sig: Option<Rc<str>>,
}

/// Sets of kinds of values, as bits: what a value can be at run time.
pub type Kinds = u16;
pub const UNIT: Kinds = 1;
pub const BOOL: Kinds = 1 << 1;
pub const INT: Kinds = 1 << 2;
pub const FLOAT: Kinds = 1 << 3;
pub const TEXT: Kinds = 1 << 4;
pub const LIST: Kinds = 1 << 5;
pub const RECORD: Kinds = 1 << 6;
pub const OPTION: Kinds = 1 << 7;
pub const RESULT: Kinds = 1 << 8;
/// A value of a type defined with `type`.
pub const DATA: Kinds = 1 << 9;
pub const FUNC: Kinds = 1 << 10;
pub const SYMBOL: Kinds = 1 << 11;
pub const NUMBER: Kinds = INT | FLOAT;
pub const ALL: Kinds = (1 << 12) - 1;

/// Types nested deeper than this are not followed (`Any` instead), so that
/// inferring the types of recursive functions always ends.
const MAX_DEPTH: usize = 5;

impl Ty {
    pub fn list(t: Ty) -> Ty {
        Ty::List(Rc::new(t))
    }

    pub fn func(arity: Option<(usize, usize)>, ret: Ty, sig: Option<Rc<str>>) -> Ty {
        Ty::Func(Rc::new(FuncTy { arity, ret, sig }))
    }

    pub fn data(name: Sym, ctors: Option<Vec<(Sym, Vec<Ty>)>>) -> Ty {
        Ty::Data(Rc::new(DataTy { name, ctors }))
    }

    /// `Some(t)` or `None`.
    pub fn option(t: Ty) -> Ty {
        Ty::data(Sym::new("Option"), Some(vec![(Sym::new("Some"), vec![t]), (Sym::new("None"), Vec::new())]))
    }

    /// `Ok(ok)` or `Err(err)`.
    pub fn result(ok: Ty, err: Ty) -> Ty {
        Ty::data(Sym::new("Result"), Some(vec![(Sym::new("Ok"), vec![ok]), (Sym::new("Err"), vec![err])]))
    }

    pub fn record(fields: Vec<(Rc<str>, Ty)>, exact: bool) -> Ty {
        Ty::Record(Rc::new(RecordTy { fields, exact }))
    }

    /// The kinds of values this type contains.
    pub fn kinds(&self) -> Kinds {
        match self {
            Ty::Never => 0,
            Ty::Any => ALL,
            Ty::Unit => UNIT,
            Ty::Bool => BOOL,
            Ty::Int => INT,
            Ty::Float => FLOAT,
            Ty::Num => NUMBER,
            Ty::Text => TEXT,
            Ty::Symbol => SYMBOL,
            Ty::List(_) => LIST,
            Ty::Record(_) => RECORD,
            Ty::Data(d) => d.kind(),
            Ty::Func(_) => FUNC,
        }
    }

    /// Is something certain known about the values (not `Any`, not `Never`)?
    pub fn is_known(&self) -> bool {
        !matches!(self, Ty::Any | Ty::Never)
    }

    /// Is every value of this type of one of these kinds?
    pub fn only(&self, kinds: Kinds) -> bool {
        self.kinds() & !kinds == 0
    }

    /// Can no value of this type be of these kinds? (False for `Never`:
    /// nothing is said about code that never runs.)
    pub fn never(&self, kinds: Kinds) -> bool {
        *self != Ty::Never && self.kinds() & kinds == 0
    }

    /// The smallest type that contains the values of both.
    pub fn join(&self, other: &Ty) -> Ty {
        match (self, other) {
            (Ty::Never, t) | (t, Ty::Never) => t.clone(),
            (Ty::Any, _) | (_, Ty::Any) => Ty::Any,
            (a, b) if a == b => a.clone(),
            (Ty::Int | Ty::Float | Ty::Num, Ty::Int | Ty::Float | Ty::Num) => Ty::Num,
            (Ty::List(a), Ty::List(b)) => Ty::list(a.join(b)),
            (Ty::Record(a), Ty::Record(b)) => {
                let fields: Vec<(Rc<str>, Ty)> = a.fields.iter().filter_map(|(k, t)| b.get(k).map(|u| (k.clone(), t.join(u)))).collect();
                let exact = a.exact && b.exact && fields.len() == a.fields.len() && fields.len() == b.fields.len();
                Ty::record(fields, exact)
            }
            (Ty::Data(a), Ty::Data(b)) if a.name == b.name => {
                let ctors = match (&a.ctors, &b.ctors) {
                    (Some(x), Some(y)) => {
                        let mut out = x.clone();
                        for (c, fields) in y {
                            match out.iter_mut().find(|(d, _)| d == c) {
                                Some((_, mine)) if mine.len() == fields.len() => {
                                    for (m, f) in mine.iter_mut().zip(fields) {
                                        *m = m.join(f);
                                    }
                                }
                                Some((_, mine)) => *mine = vec![Ty::Any; mine.len().max(fields.len())],
                                None => out.push((*c, fields.clone())),
                            }
                        }
                        Some(out)
                    }
                    _ => None,
                };
                Ty::data(a.name, ctors)
            }
            (Ty::Func(a), Ty::Func(b)) => Ty::func(
                if a.arity == b.arity { a.arity } else { None },
                a.ret.join(&b.ret),
                if a.sig == b.sig { a.sig.clone() } else { None },
            ),
            _ => Ty::Any,
        }
    }

    /// The same type, with what is nested too deeply replaced by `Any`.
    pub fn limit(&self) -> Ty {
        self.limit_at(MAX_DEPTH)
    }

    fn limit_at(&self, depth: usize) -> Ty {
        match self {
            Ty::List(_) | Ty::Record(_) | Ty::Data(_) | Ty::Func(_) if depth == 0 => Ty::Any,
            Ty::List(t) => Ty::list(t.limit_at(depth - 1)),
            Ty::Record(r) => Ty::record(r.fields.iter().map(|(k, t)| (k.clone(), t.limit_at(depth - 1))).collect(), r.exact),
            Ty::Data(d) => Ty::data(
                d.name,
                d.ctors.as_ref().map(|cs| cs.iter().map(|(c, fs)| (*c, fs.iter().map(|f| f.limit_at(depth - 1)).collect())).collect()),
            ),
            Ty::Func(f) => Ty::func(f.arity, f.ret.limit_at(depth - 1), f.sig.clone()),
            t => t.clone(),
        }
    }

    /// The same type, where functions may give back anything: what a
    /// function of another file gives depends on the names it calls, which
    /// the program may define again.
    pub fn without_results(&self) -> Ty {
        match self {
            Ty::Func(f) => Ty::func(f.arity, Ty::Any, f.sig.clone()),
            Ty::List(t) => Ty::list(t.without_results()),
            Ty::Record(r) => Ty::record(r.fields.iter().map(|(k, t)| (k.clone(), t.without_results())).collect(), r.exact),
            Ty::Data(d) => Ty::data(
                d.name,
                d.ctors.as_ref().map(|cs| cs.iter().map(|(c, fs)| (*c, fs.iter().map(Ty::without_results).collect())).collect()),
            ),
            t => t.clone(),
        }
    }

    /// The type of the elements, for a list.
    pub fn elem(&self) -> Ty {
        match self {
            Ty::List(t) => (**t).clone(),
            Ty::Never => Ty::Never,
            _ => Ty::Any,
        }
    }

    /// What `f(...)` gives, for a function.
    pub fn ret(&self) -> Ty {
        match self {
            Ty::Func(f) => f.ret.clone(),
            Ty::Never => Ty::Never,
            _ => Ty::Any,
        }
    }

    /// The name of the type in messages, like the run time says it: `Int`, `Text`, `Shape`...
    pub fn name(&self) -> String {
        match self {
            Ty::Never | Ty::Any => "?".into(),
            Ty::Unit => "Unit".into(),
            Ty::Bool => "Bool".into(),
            Ty::Int => "Int".into(),
            Ty::Float => "Float".into(),
            Ty::Num => "Int/Float".into(),
            Ty::Text => "Text".into(),
            Ty::Symbol => "Symbol".into(),
            Ty::List(_) => "List".into(),
            Ty::Record(_) => "Record".into(),
            Ty::Data(d) => d.name.to_string(),
            Ty::Func(_) => "Function".into(),
        }
    }

    /// The precise type of a value.
    pub fn of_value(v: &Value) -> Ty {
        Ty::of_value_at(v, MAX_DEPTH)
    }

    fn of_value_at(v: &Value, depth: usize) -> Ty {
        let inner = |x: &Value| if depth == 0 { Ty::Any } else { Ty::of_value_at(x, depth - 1) };
        match v {
            Value::Unit => Ty::Unit,
            Value::Bool(_) => Ty::Bool,
            Value::Int(_) => Ty::Int,
            Value::Float(_) => Ty::Float,
            Value::Text(_) => Ty::Text,
            Value::Symbol(_) => Ty::Symbol,
            Value::List(l) => {
                let mut t = Ty::Never;
                for (i, x) in l.iter().enumerate() {
                    if t == Ty::Any || i > 10_000 {
                        return Ty::list(Ty::Any);
                    }
                    t = t.join(&inner(x));
                }
                Ty::list(t)
            }
            Value::Record(r) => Ty::record(r.iter().map(|(k, x)| (k.clone(), inner(x))).collect(), true),
            Value::Variant(var) => Ty::data(var.ctor.type_name, Some(vec![(var.ctor.name, var.fields.iter().map(inner).collect())])),
            Value::Func(c) => Ty::func(Some((c.def.arity, c.def.arity)), Ty::Any, None),
            Value::Builtin(b) => Ty::func(Some((b.min, b.max)), Ty::Any, Some(Rc::from(b.sig))),
            Value::Ctor(c) => {
                let n = c.fields.len();
                Ty::func(Some((n, n)), Ty::data(c.type_name, Some(vec![(c.name, vec![Ty::Any; n])])), None)
            }
        }
    }

    /// Is this value one of the values of this type? (The tests check with it
    /// that the inferred types are right.)
    pub fn contains(&self, v: &Value) -> bool {
        match (self, v) {
            (Ty::Any, _) => true,
            (Ty::Unit, Value::Unit) | (Ty::Bool, Value::Bool(_)) | (Ty::Int, Value::Int(_)) | (Ty::Float, Value::Float(_)) => true,
            (Ty::Num, Value::Int(_) | Value::Float(_)) | (Ty::Text, Value::Text(_)) | (Ty::Symbol, Value::Symbol(_)) => true,
            (Ty::List(t), Value::List(l)) => l.iter().all(|x| t.contains(x)),
            (Ty::Record(r), Value::Record(rec)) => {
                (!r.exact || r.fields.len() == rec.len()) && r.fields.iter().all(|(k, t)| rec.get(k).is_some_and(|x| t.contains(x)))
            }
            (Ty::Data(d), Value::Variant(var)) => {
                var.ctor.type_name == d.name
                    && match &d.ctors {
                        None => true,
                        Some(cs) => cs.iter().any(|(c, fs)| {
                            *c == var.ctor.name && fs.len() == var.fields.len() && fs.iter().zip(&var.fields).all(|(t, x)| t.contains(x))
                        }),
                    }
            }
            (Ty::Func(f), Value::Func(_) | Value::Builtin(_) | Value::Ctor(_)) => {
                let n = match v {
                    Value::Func(c) => (c.def.arity, c.def.arity),
                    Value::Builtin(b) => (b.min, b.max),
                    Value::Ctor(c) => (c.fields.len(), c.fields.len()),
                    _ => return false,
                };
                f.arity.is_none_or(|a| a == n)
            }
            _ => false,
        }
    }
}

impl RecordTy {
    pub fn get(&self, name: &str) -> Option<&Ty> {
        self.fields.iter().find(|(k, _)| &**k == name).map(|(_, t)| t)
    }
}

/// The kind of the values of the type `name`: Option, Result, or a type of the program.
fn data_kind(name: Sym) -> Kinds {
    match &*name.name() {
        "Option" => OPTION,
        "Result" => RESULT,
        _ => DATA,
    }
}

/// The kind of a value, like [`Ty::kinds`] says it for its type.
pub fn kind_of(v: &Value) -> Kinds {
    match v {
        Value::Unit => UNIT,
        Value::Bool(_) => BOOL,
        Value::Int(_) => INT,
        Value::Float(_) => FLOAT,
        Value::Text(_) => TEXT,
        Value::Symbol(_) => SYMBOL,
        Value::List(_) => LIST,
        Value::Record(_) => RECORD,
        Value::Variant(var) => data_kind(var.ctor.type_name),
        Value::Func(_) | Value::Builtin(_) | Value::Ctor(_) => FUNC,
    }
}

impl DataTy {
    pub fn kind(&self) -> Kinds {
        data_kind(self.name)
    }

    /// The types of the fields of a constructor, if the value may have been
    /// built with it (`None`: it surely was not); `Any` when unknown.
    pub fn fields_of(&self, ctor: &str, arity: usize) -> Option<Vec<Ty>> {
        match &self.ctors {
            None => Some(vec![Ty::Any; arity]),
            Some(cs) => cs.iter().find(|(c, _)| &*c.name() == ctor).map(|(_, fs)| fs.clone()),
        }
    }

    /// What `Some(x)` or `Ok(x)` holds: what `?` and `unwrap` give.
    pub fn payload(&self) -> Ty {
        let ctor = if self.kind() == OPTION { "Some" } else { "Ok" };
        match self.fields_of(ctor, 1) {
            Some(fs) => fs.into_iter().next().unwrap_or(Ty::Any),
            None => Ty::Never,
        }
    }

    /// The `None` or `Err(e)` that `?` returns from the function.
    pub fn failure(&self) -> Ty {
        let ctor = if self.kind() == OPTION { "None" } else { "Err" };
        match self.fields_of(ctor, usize::from(ctor == "Err")) {
            Some(fs) => Ty::data(self.name, Some(vec![(Sym::new(ctor), fs)])),
            None => Ty::Never,
        }
    }
}

// ----- built-in functions -------------------------------------------------

/// What the checker knows of a call to a built-in function.
pub enum Outcome {
    /// The call gives a value of this type (when it does not stop).
    Gives(Ty),
    /// This argument is never accepted: the call fails every time, with the
    /// message of [`crate::builtins::wrong_argument`].
    Wrong { index: usize, expected: Expected },
}

/// Fails if argument `i` can never be what the function expects. The
/// arguments are checked in the order the function checks them when it runs,
/// so that the error is the one it stops with.
fn need(a: &[Ty], i: usize, expected: Expected) -> Result<(), Outcome> {
    match a.get(i) {
        Some(t) if t.never(expected.kinds()) => Err(Outcome::Wrong { index: i, expected }),
        _ => Ok(()),
    }
}

/// The type of a list or a text after taking some of it (`rest`, `take`...).
fn same_list_or_text(t: &Ty) -> Ty {
    match t {
        Ty::List(_) | Ty::Text => t.clone(),
        _ => Ty::Any,
    }
}

/// The type of each element of a list, or of each character of a text.
fn piece(t: &Ty) -> Ty {
    match t {
        Ty::Text => Ty::Text,
        t => t.elem(),
    }
}

fn payload(t: &Ty) -> Ty {
    match t {
        Ty::Data(d) => d.payload(),
        _ => Ty::Any,
    }
}

/// Int for Int, Float for Float, a number otherwise.
fn numeric(t: &Ty) -> Ty {
    match t {
        Ty::Int | Ty::Float => t.clone(),
        _ => Ty::Num,
    }
}

/// What the error of a Result holds, if it may be an error (`Never` otherwise).
fn error_of(d: &DataTy) -> Ty {
    d.fields_of("Err", 1).and_then(|f| f.into_iter().next()).unwrap_or(Ty::Never)
}

/// `and_then(x, f)`: `None` and `Err(e)` stay as they are; `Some(v)` and
/// `Ok(v)` give `f(v)`, which must be of the same kind (`ret`), else the call fails.
fn chained(d: &DataTy, ret: &Ty) -> Ty {
    let given = if d.payload() == Ty::Never || ret.never(d.kind()) {
        Ty::Never
    } else if ret.is_known() && ret.only(d.kind()) {
        ret.clone()
    } else {
        Ty::data(d.name, None)
    };
    d.failure().join(&given)
}

/// A date: `{year, month, day}`, with `hour`, `minute` and `second` when
/// `timed`; `exact` for `now()` and `today()`, which have no other field
/// (`now()` also has `offset`).
fn date_ty(timed: bool, exact: bool) -> Ty {
    let names: &[&str] = match (timed, exact) {
        (true, true) => &["year", "month", "day", "hour", "minute", "second", "offset"],
        (true, false) => &["year", "month", "day", "hour", "minute", "second"],
        _ => &["year", "month", "day"],
    };
    Ty::record(names.iter().map(|n| (Rc::from(*n), Ty::Int)).collect(), exact)
}

/// What a call to the built-in function `name` gives, with arguments of
/// these types. `None` for a function the checker knows nothing about: new
/// built-in functions work without being described here.
pub fn builtin_call(name: &str, a: &[Ty]) -> Option<Outcome> {
    if a.contains(&Ty::Never) {
        return Some(Outcome::Gives(Ty::Never));
    }
    let arg = |i: usize| a.get(i).cloned().unwrap_or(Ty::Any);
    let n = a.len();
    let gives = (|| -> Result<Ty, Outcome> {
        Ok(match name {
            "print" | "help" | "assert_eq" => Ty::Unit,
            "debug" => arg(0),
            "type_of" | "input" => Ty::Text,
            "text" => {
                if n == 2 {
                    need(a, 1, Expected::Int)?;
                    need(a, 0, Expected::Number)?;
                }
                Ty::Text
            }
            "assert" => {
                need(a, 0, Expected::Bool)?;
                Ty::Unit
            }
            "fail" => Ty::Never,
            "args" => Ty::list(Ty::Text),
            "env" => {
                need(a, 0, Expected::Text)?;
                Ty::option(Ty::Text)
            }
            "exit" => {
                need(a, 0, Expected::Int)?;
                Ty::Never
            }
            "time" => Ty::Float,
            "random" => match n {
                0 => Ty::Float,
                1 => {
                    need(a, 0, Expected::List)?;
                    arg(0).elem()
                }
                _ => {
                    need(a, 0, Expected::Int)?;
                    need(a, 1, Expected::Int)?;
                    Ty::Int
                }
            },
            "abs" => {
                need(a, 0, Expected::Number)?;
                numeric(&arg(0))
            }
            "min" | "max" if n == 1 => {
                need(a, 0, Expected::ListOrValues)?;
                arg(0).elem()
            }
            "min" | "max" => a.iter().fold(Ty::Never, |acc, t| acc.join(t)),
            "sqrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "exp" => {
                need(a, 0, Expected::Number)?;
                Ty::Float
            }
            "log" | "atan" => {
                need(a, 0, Expected::Number)?;
                need(a, 1, Expected::Number)?;
                Ty::Float
            }
            "floor" | "ceil" => {
                need(a, 0, Expected::Number)?;
                Ty::Int
            }
            "round" => {
                need(a, 0, Expected::Number)?;
                need(a, 1, Expected::Int)?;
                if n == 2 {
                    Ty::Float
                } else {
                    Ty::Int
                }
            }
            "int" => {
                need(a, 0, Expected::ToInt)?;
                Ty::Int
            }
            "float" => {
                need(a, 0, Expected::ToFloat)?;
                Ty::Float
            }
            "is_even" | "is_odd" => {
                need(a, 0, Expected::Int)?;
                Ty::Bool
            }
            "gcd" | "lcm" => {
                need(a, 0, Expected::Int)?;
                need(a, 1, Expected::Int)?;
                Ty::Int
            }
            "len" => {
                need(a, 0, Expected::TextListOrRecord)?;
                Ty::Int
            }
            "upper" | "lower" | "capitalize" | "trim" => {
                need(a, 0, Expected::Text)?;
                Ty::Text
            }
            "pad_left" | "pad_right" => {
                // Any value is padded as `print` shows it.
                need(a, 1, Expected::Int)?;
                need(a, 2, Expected::Text)?;
                Ty::Text
            }
            "split" => {
                need(a, 0, Expected::Text)?;
                need(a, 1, Expected::Text)?;
                Ty::list(Ty::Text)
            }
            "join" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Text)?;
                Ty::Text
            }
            "contains" => {
                need(a, 0, Expected::TextListOrRecord)?;
                // In a text or a record, what is looked for is a text.
                if arg(0).only(TEXT | RECORD) {
                    need(a, 1, Expected::Text)?;
                }
                Ty::Bool
            }
            "starts_with" | "ends_with" => {
                need(a, 0, Expected::Text)?;
                need(a, 1, Expected::Text)?;
                Ty::Bool
            }
            "replace" => {
                need(a, 0, Expected::Text)?;
                need(a, 1, Expected::Text)?;
                need(a, 2, Expected::Text)?;
                Ty::Text
            }
            "chars" | "lines" => {
                need(a, 0, Expected::Text)?;
                Ty::list(Ty::Text)
            }
            "char_code" => {
                need(a, 0, Expected::Text)?;
                Ty::Int
            }
            "char" => {
                need(a, 0, Expected::Int)?;
                Ty::Text
            }
            "is_digit" | "is_letter" | "is_space" => {
                need(a, 0, Expected::Text)?;
                Ty::Bool
            }
            "repeat" => {
                need(a, 1, Expected::Int)?;
                need(a, 0, Expected::TextOrList)?;
                same_list_or_text(&arg(0))
            }
            "parse_int" => {
                need(a, 0, Expected::Text)?;
                Ty::result(Ty::Int, Ty::Text)
            }
            "parse_float" => {
                need(a, 0, Expected::Text)?;
                Ty::result(Ty::Float, Ty::Text)
            }
            "parse_json" => {
                need(a, 0, Expected::Text)?;
                Ty::result(Ty::Any, Ty::Text)
            }
            "to_json" => {
                need(a, 1, Expected::Int)?;
                Ty::Text
            }
            "map" => {
                need(a, 1, Expected::Function)?;
                need(a, 0, Expected::ListOptionOrResult)?;
                let r = arg(1).ret();
                match arg(0) {
                    Ty::List(_) => Ty::list(r),
                    Ty::Data(d) if d.kind() == OPTION => Ty::option(r),
                    Ty::Data(d) if d.kind() == RESULT => Ty::result(r, error_of(&d)),
                    _ => Ty::Any,
                }
            }
            "filter" | "sort_by" | "reject" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                Ty::list(arg(0).elem())
            }
            "fold" => {
                need(a, 0, Expected::List)?;
                need(a, 2, Expected::Function)?;
                arg(1).join(&arg(2).ret())
            }
            "scan" => {
                need(a, 0, Expected::List)?;
                need(a, 2, Expected::Function)?;
                Ty::list(arg(2).ret())
            }
            "each" | "each_with_index" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                Ty::Unit
            }
            "any" | "all" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                Ty::Bool
            }
            "find" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                Ty::option(arg(0).elem())
            }
            "count" => {
                need(a, 0, Expected::List)?;
                if n == 2 {
                    need(a, 1, Expected::Function)?;
                }
                Ty::Int
            }
            "min_by" | "max_by" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                arg(0).elem()
            }
            "partition" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                Ty::list(Ty::list(arg(0).elem()))
            }
            "flat_map" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                Ty::list(arg(1).ret().elem())
            }
            "group_by" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Function)?;
                Ty::record(Vec::new(), false)
            }
            "sum" | "product" => {
                need(a, 0, Expected::List)?;
                match arg(0).elem() {
                    Ty::Int | Ty::Never => Ty::Int,
                    _ => Ty::Num,
                }
            }
            "first" | "last" if n == 2 => {
                need(a, 0, Expected::ListOrText)?;
                need(a, 1, Expected::Int)?;
                same_list_or_text(&arg(0))
            }
            "first" | "last" => {
                need(a, 0, Expected::ListOrText)?;
                Ty::option(piece(&arg(0)))
            }
            "rest" | "reverse" => {
                need(a, 0, Expected::ListOrText)?;
                same_list_or_text(&arg(0))
            }
            "get" => {
                need(a, 0, Expected::ListTextOrRecord)?;
                if arg(0).only(LIST | TEXT) {
                    need(a, 1, Expected::Int)?;
                }
                if arg(0).only(RECORD) {
                    need(a, 1, Expected::Text)?;
                }
                Ty::option(match arg(0) {
                    Ty::Record(_) => Ty::Any,
                    t => piece(&t),
                })
            }
            "push" => {
                need(a, 0, Expected::List)?;
                Ty::list(arg(0).elem().join(&arg(1)))
            }
            "set_at" | "insert_at" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Int)?;
                Ty::list(arg(0).elem().join(&arg(2)))
            }
            "remove_at" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::Int)?;
                Ty::list(arg(0).elem())
            }
            "sort" | "unique" | "shuffle" => {
                need(a, 0, Expected::List)?;
                Ty::list(arg(0).elem())
            }
            "slice" => {
                need(a, 0, Expected::ListOrText)?;
                need(a, 1, Expected::Int)?;
                need(a, 2, Expected::Int)?;
                same_list_or_text(&arg(0))
            }
            "take" | "drop" | "step_by" => {
                need(a, 0, Expected::ListOrText)?;
                need(a, 1, Expected::Int)?;
                same_list_or_text(&arg(0))
            }
            "take_while" | "drop_while" => {
                need(a, 0, Expected::ListOrText)?;
                need(a, 1, Expected::Function)?;
                same_list_or_text(&arg(0))
            }
            "chunks" | "windows" => {
                need(a, 0, Expected::ListOrText)?;
                need(a, 1, Expected::Int)?;
                Ty::list(same_list_or_text(&arg(0)))
            }
            "zip" => {
                need(a, 0, Expected::List)?;
                need(a, 1, Expected::List)?;
                Ty::list(Ty::list(arg(0).elem().join(&arg(1).elem())))
            }
            "enumerate" => {
                need(a, 0, Expected::List)?;
                Ty::list(Ty::list(Ty::Int.join(&arg(0).elem())))
            }
            "index_of" => {
                need(a, 0, Expected::ListOrText)?;
                if arg(0).only(TEXT) {
                    need(a, 1, Expected::Text)?;
                }
                Ty::option(Ty::Int)
            }
            "flatten" => {
                need(a, 0, Expected::List)?;
                Ty::list(arg(0).elem().elem())
            }
            "is_empty" => {
                need(a, 0, Expected::TextListOrRecord)?;
                Ty::Bool
            }
            "keys" => {
                need(a, 0, Expected::Record)?;
                Ty::list(Ty::Text)
            }
            "values" => {
                need(a, 0, Expected::Record)?;
                Ty::list(match arg(0) {
                    Ty::Record(r) if r.exact => r.fields.iter().fold(Ty::Never, |acc, (_, t)| acc.join(t)),
                    _ => Ty::Any,
                })
            }
            "entries" => {
                need(a, 0, Expected::Record)?;
                Ty::list(Ty::list(Ty::Any))
            }
            "from_entries" => {
                need(a, 0, Expected::List)?;
                Ty::record(Vec::new(), false)
            }
            "has" => {
                need(a, 0, Expected::Record)?;
                need(a, 1, Expected::Text)?;
                Ty::Bool
            }
            "put" => {
                need(a, 0, Expected::Record)?;
                need(a, 1, Expected::Text)?;
                // The record keeps its fields, but any of them may be the one replaced.
                let v = arg(2);
                Ty::record(
                    match arg(0) {
                        Ty::Record(r) => r.fields.iter().map(|(k, t)| (k.clone(), t.join(&v))).collect(),
                        _ => Vec::new(),
                    },
                    false,
                )
            }
            "remove" => {
                need(a, 0, Expected::Record)?;
                need(a, 1, Expected::Text)?;
                Ty::record(Vec::new(), false)
            }
            "map_values" => {
                need(a, 0, Expected::Record)?;
                need(a, 1, Expected::Function)?;
                // The same names, each with what the function gives.
                let r = arg(1).ret();
                match arg(0) {
                    Ty::Record(rec) => Ty::record(rec.fields.iter().map(|(k, _)| (k.clone(), r.clone())).collect(), rec.exact),
                    _ => Ty::record(Vec::new(), false),
                }
            }
            "unwrap" => {
                need(a, 0, Expected::OptionOrResult)?;
                payload(&arg(0))
            }
            "unwrap_or" => {
                need(a, 0, Expected::OptionOrResult)?;
                payload(&arg(0)).join(&arg(1))
            }
            "is_some" | "is_none" | "is_ok" | "is_err" => {
                need(a, 0, Expected::OptionOrResult)?;
                Ty::Bool
            }
            "and_then" => {
                need(a, 0, Expected::OptionOrResult)?;
                need(a, 1, Expected::Function)?;
                match arg(0) {
                    Ty::Data(d) if matches!(d.kind(), OPTION | RESULT) => chained(&d, &arg(1).ret()),
                    _ => Ty::Any,
                }
            }
            "ok_or" => {
                need(a, 0, Expected::Option)?;
                match arg(0) {
                    // `Some(x)` gives `Ok(x)`, `None` gives `Err(error)`.
                    Ty::Data(d) => {
                        let mut ctors = Vec::new();
                        if let Some(fs) = d.fields_of("Some", 1) {
                            ctors.push((Sym::new("Ok"), fs));
                        }
                        if d.fields_of("None", 0).is_some() {
                            ctors.push((Sym::new("Err"), vec![arg(1)]));
                        }
                        Ty::data(Sym::new("Result"), Some(ctors))
                    }
                    _ => Ty::result(Ty::Any, arg(1)),
                }
            }
            "map_err" => {
                need(a, 1, Expected::Function)?;
                need(a, 0, Expected::Result)?;
                let e = arg(1).ret();
                match arg(0) {
                    // `Ok(x)` stays, `Err(x)` gives `Err(f(x))`.
                    Ty::Data(d) => {
                        let mut ctors = Vec::new();
                        if let Some(fs) = d.fields_of("Ok", 1) {
                            ctors.push((Sym::new("Ok"), fs));
                        }
                        if d.fields_of("Err", 1).is_some() {
                            ctors.push((Sym::new("Err"), vec![e]));
                        }
                        Ty::data(Sym::new("Result"), Some(ctors))
                    }
                    _ => Ty::result(Ty::Any, e),
                }
            }
            "read_file" => {
                need(a, 0, Expected::Text)?;
                Ty::result(Ty::Text, Ty::Text)
            }
            "write_file" | "append_file" => {
                need(a, 0, Expected::Text)?;
                Ty::result(Ty::Unit, Ty::Text)
            }
            "file_exists" => {
                need(a, 0, Expected::Text)?;
                Ty::Bool
            }
            // A date always has these fields; `now()` exactly these and its time.
            "now" => date_ty(true, true),
            "today" => date_ty(false, true),
            "parse_date" => {
                need(a, 0, Expected::Text)?;
                Ty::result(date_ty(false, false), Ty::Text)
            }
            "format_date" => {
                need(a, 0, Expected::Date)?;
                Ty::Text
            }
            "is_date" => Ty::Bool,
            "add_days" | "add_seconds" => {
                need(a, 0, Expected::Date)?;
                need(a, 1, Expected::Int)?;
                date_ty(name == "add_seconds", false)
            }
            "days_between" | "seconds_between" => {
                need(a, 0, Expected::Date)?;
                need(a, 1, Expected::Date)?;
                Ty::Int
            }
            "day_of_week" => {
                need(a, 0, Expected::Date)?;
                Ty::Int
            }
            "days_in_month" => {
                need(a, 0, Expected::Int)?;
                need(a, 1, Expected::Int)?;
                Ty::Int
            }
            "is_leap_year" => {
                need(a, 0, Expected::Int)?;
                Ty::Bool
            }
            _ => return Err(Outcome::Gives(Ty::Any)),
        })
    })();
    match gives {
        Ok(t) => Some(Outcome::Gives(t)),
        Err(Outcome::Gives(_)) => None,
        Err(wrong) => Some(wrong),
    }
}

/// A function given to a built-in function (`map(list, f)`): how it is called.
pub struct Callback {
    /// The types of the values it receives.
    pub params: Vec<Ty>,
    /// What it must give back: `filter` needs true or false, `flat_map` a
    /// list (the error is [`crate::builtins::wrong_return`]).
    pub returns: Option<Kinds>,
}

/// How the built-in function `name` calls its argument `index`, if it is a
/// function it calls.
pub fn callback(name: &str, index: usize, a: &[Ty]) -> Option<Callback> {
    let first = a.first().cloned().unwrap_or(Ty::Any);
    let elem = first.elem();
    let (params, returns) = match (name, index) {
        ("map", 1) => {
            let x = match &first {
                Ty::List(t) => (**t).clone(),
                Ty::Data(d) if matches!(d.kind(), OPTION | RESULT) => d.payload(),
                _ => Ty::Any,
            };
            (vec![x], None)
        }
        ("filter" | "reject" | "any" | "all" | "find" | "count" | "partition", 1) => (vec![elem], Some(BOOL)),
        ("each_with_index", 1) => (vec![elem, Ty::Int], None),
        // Each element of a list, or each character of a text.
        ("take_while" | "drop_while", 1) => (vec![piece(&first)], Some(BOOL)),
        ("flat_map", 1) => (vec![elem], Some(LIST)),
        ("each" | "sort_by" | "group_by" | "min_by" | "max_by", 1) => (vec![elem], None),
        // The total starts as the second argument, then is what the function
        // gave back: the checker finds its type by trying again until it is stable.
        ("fold" | "scan", 2) => (vec![Ty::Any, elem], None),
        ("map_values", 1) => {
            let x = match &first {
                Ty::Record(r) if r.exact => r.fields.iter().fold(Ty::Never, |acc, (_, t)| acc.join(t)),
                _ => Ty::Any,
            };
            (vec![x], None)
        }
        // It must give back the same kind as what it receives: an Option for `Some(x)`.
        ("and_then", 1) => match &first {
            Ty::Data(d) if matches!(d.kind(), OPTION | RESULT) => (vec![d.payload()], Some(d.kind())),
            _ => (vec![Ty::Any], None),
        },
        ("map_err", 1) => match &first {
            Ty::Data(d) if d.kind() == RESULT => (vec![error_of(d)], None),
            _ => (vec![Ty::Any], None),
        },
        _ => return None,
    };
    Some(Callback { params, returns })
}

/// The type of a built-in function used as a value (`map(list, len)`).
pub fn builtin_value(b: &Builtin) -> Ty {
    let mut ret = Ty::Never;
    for n in b.min..=b.max.min(b.min + 2) {
        match builtin_call(b.name, &vec![Ty::Any; n]) {
            Some(Outcome::Gives(t)) => ret = ret.join(&t),
            Some(Outcome::Wrong { .. }) => {}
            None => ret = Ty::Any,
        }
    }
    Ty::func(Some((b.min, b.max)), ret, Some(Rc::from(b.sig)))
}
