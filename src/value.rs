//! Runtime values. All of them are immutable: "changing" a value builds a new one.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Write;
use std::rc::Rc;

use crate::ast::FnDef;
use crate::builtins::Builtin;
use crate::compile::FnCode;
use crate::symbol::Sym;

#[derive(Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(Rc<str>),
    List(List),
    Record(Rc<Record>),
    Variant(Rc<Variant>),
    Func(Rc<Closure>),
    Builtin(&'static Builtin),
    Ctor(Rc<CtorInfo>),
}

/// An immutable list. Taking the rest of a list (`[first, ..rest]`) shares the
/// elements instead of copying them.
#[derive(Clone)]
pub struct List {
    data: Rc<Vec<Value>>,
    // 32-bit positions keep every value small (a list of 4 billion
    // elements would not fit in memory anyway).
    start: u32,
    end: u32,
}

impl List {
    pub fn new(v: Vec<Value>) -> List {
        let end = u32::try_from(v.len()).expect("a list of more than 4 billion elements");
        List { data: Rc::new(v), start: 0, end }
    }

    pub fn as_slice(&self) -> &[Value] {
        &self.data[self.start as usize..self.end as usize]
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.as_slice().iter()
    }

    /// Elements `from..to` (relative to this list), sharing memory.
    pub fn slice(&self, from: usize, to: usize) -> List {
        let to = to.min(self.len());
        let from = from.min(to);
        List { data: self.data.clone(), start: self.start + from as u32, end: self.start + to as u32 }
    }

    pub fn get(&self, i: usize) -> Option<&Value> {
        self.as_slice().get(i)
    }
}

/// A record: named fields, kept in the order they were written.
#[derive(Clone, Default)]
pub struct Record {
    /// The names of the fields, shared by the records made from one another
    /// (`{..r, x: 1}`) and by the records made by the same `{...}` in the code.
    shape: Rc<Shape>,
    vals: Vec<Value>,
}

/// The names of the fields of records, in order.
#[derive(Clone, Default)]
pub struct Shape {
    keys: Vec<Rc<str>>,
    /// Where each name is, for records with many fields: small ones are
    /// searched in order, which is faster.
    index: Option<HashMap<Rc<str>, usize>>,
}

/// Records with more fields than this find them with an index.
const INDEXED: usize = 12;

impl Shape {
    /// The fields of a record, all different.
    pub fn new(keys: Vec<Rc<str>>) -> Shape {
        let mut shape = Shape { keys, index: None };
        shape.add_index();
        shape
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    fn add_index(&mut self) {
        if self.index.is_none() && self.keys.len() > INDEXED {
            self.index = Some(self.keys.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect());
        }
    }

    fn position(&self, key: &str) -> Option<usize> {
        match &self.index {
            Some(index) => index.get(key).copied(),
            None => self.keys.iter().position(|k| same_key(k, key)),
        }
    }

    fn push(&mut self, key: Rc<str>) {
        if let Some(index) = &mut self.index {
            index.insert(key.clone(), self.keys.len());
        }
        self.keys.push(key);
        self.add_index();
    }
}

/// Names written in the code are shared: comparing their addresses is
/// often enough, and they are short.
#[inline]
fn same_key(a: &str, b: &str) -> bool {
    a.len() == b.len() && (std::ptr::eq(a.as_ptr(), b.as_ptr()) || a.bytes().zip(b.bytes()).all(|(x, y)| x == y))
}

impl Record {
    pub fn new() -> Record {
        Record::default()
    }

    /// A record with the fields of `shape`, in order.
    pub fn with_shape(shape: Rc<Shape>, vals: Vec<Value>) -> Record {
        debug_assert_eq!(shape.len(), vals.len());
        Record { shape, vals }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.shape.position(key).map(|i| &self.vals[i])
    }

    pub fn set(&mut self, key: Rc<str>, val: Value) {
        match self.shape.position(&key) {
            Some(i) => self.vals[i] = val,
            None => {
                Rc::make_mut(&mut self.shape).push(key);
                self.vals.push(val);
            }
        }
    }

    pub fn without(&self, key: &str) -> Record {
        let mut r = Record::new();
        for (k, v) in self.iter() {
            if &**k != key {
                r.set(k.clone(), v.clone());
            }
        }
        r
    }

    pub fn len(&self) -> usize {
        self.vals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vals.is_empty()
    }

    pub fn keys(&self) -> &[Rc<str>] {
        &self.shape.keys
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Rc<str>, &Value)> {
        self.shape.keys.iter().zip(self.vals.iter())
    }
}

pub struct Closure {
    pub def: Rc<FnDef>,
    pub code: Rc<FnCode>,
    /// The local names of enclosing functions that the function uses, in the
    /// order of `code.captures`.
    pub captured: Vec<Value>,
}

pub struct CtorInfo {
    pub type_name: Sym,
    pub name: Sym,
    pub fields: Vec<Sym>,
}

pub struct Variant {
    pub ctor: Rc<CtorInfo>,
    pub fields: Vec<Value>,
}

impl Value {
    pub fn text(s: &str) -> Value {
        Value::Text(Rc::from(s))
    }

    pub fn list(v: Vec<Value>) -> Value {
        Value::List(List::new(v))
    }

    pub fn type_name(&self) -> String {
        match self {
            Value::Unit => "Unit".into(),
            Value::Bool(_) => "Bool".into(),
            Value::Int(_) => "Int".into(),
            Value::Float(_) => "Float".into(),
            Value::Text(_) => "Text".into(),
            Value::List(_) => "List".into(),
            Value::Record(_) => "Record".into(),
            Value::Variant(v) => v.ctor.type_name.name().to_string(),
            Value::Func(_) | Value::Builtin(_) | Value::Ctor(_) => "Function".into(),
        }
    }

    pub fn is_callable(&self) -> bool {
        matches!(self, Value::Func(_) | Value::Builtin(_) | Value::Ctor(_))
    }

    /// How the value is printed by `print` and inside texts: texts without quotes.
    pub fn display(&self) -> String {
        match self {
            Value::Text(s) => s.to_string(),
            _ => self.repr(),
        }
    }

    /// Writes what `display` returns at the end of `out`.
    pub fn write_display(&self, out: &mut String) {
        match self {
            Value::Text(s) => out.push_str(s),
            _ => self.write_repr(out, usize::MAX),
        }
    }

    /// How the value is written in code: texts with quotes.
    pub fn repr(&self) -> String {
        let mut out = String::new();
        self.write_repr(&mut out, usize::MAX);
        out
    }

    /// Writes the value without recursion (values can be nested millions of
    /// levels deep), stopping once `limit` bytes are written.
    fn write_repr(&self, out: &mut String, limit: usize) {
        enum Item<'a> {
            Value(&'a Value),
            Str(&'static str),
            Key(&'a str),
        }
        let mut todo = vec![Item::Value(self)];
        while let Some(item) = todo.pop() {
            if out.len() > limit {
                return;
            }
            match item {
                Item::Str(s) => out.push_str(s),
                Item::Key(k) => {
                    if is_plain_key(k) {
                        out.push_str(k);
                    } else {
                        write_text_repr(k, out);
                    }
                    out.push_str(": ");
                }
                Item::Value(v) => match v {
                    Value::Unit => out.push_str("()"),
                    Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                    Value::Int(n) => {
                        let _ = write!(out, "{n}");
                    }
                    Value::Float(f) => out.push_str(&fmt_float(*f)),
                    Value::Text(s) => write_text_repr(s, out),
                    Value::List(l) => {
                        out.push('[');
                        todo.push(Item::Str("]"));
                        for (i, x) in l.iter().enumerate().rev() {
                            todo.push(Item::Value(x));
                            if i > 0 {
                                todo.push(Item::Str(", "));
                            }
                        }
                    }
                    Value::Record(r) => {
                        out.push('{');
                        todo.push(Item::Str("}"));
                        for i in (0..r.len()).rev() {
                            todo.push(Item::Value(&r.vals[i]));
                            todo.push(Item::Key(&r.shape.keys[i]));
                            if i > 0 {
                                todo.push(Item::Str(", "));
                            }
                        }
                    }
                    Value::Variant(var) => {
                        out.push_str(&var.ctor.name.name());
                        if !var.fields.is_empty() {
                            out.push('(');
                            todo.push(Item::Str(")"));
                            for (i, x) in var.fields.iter().enumerate().rev() {
                                todo.push(Item::Value(x));
                                if i > 0 {
                                    todo.push(Item::Str(", "));
                                }
                            }
                        }
                    }
                    Value::Func(c) => match c.def.name {
                        Some(n) => out.push_str(&format!("<fn {n}>")),
                        None => out.push_str("<fn>"),
                    },
                    Value::Builtin(b) => out.push_str(&format!("<fn {}>", b.name)),
                    Value::Ctor(c) => out.push_str(&format!("<{} {}>", t!("constructor", "constructeur"), c.name)),
                },
            }
        }
    }

    /// A short description for error messages: `Text "abc"`, `List [1, 2]`.
    pub fn describe(&self) -> String {
        let mut r = String::new();
        self.write_repr(&mut r, 160);
        if r.chars().count() > 40 {
            r = r.chars().take(37).collect::<String>() + "...";
        }
        match self {
            Value::Unit => "()".into(),
            Value::Func(_) | Value::Builtin(_) | Value::Ctor(_) => r,
            _ => format!("{} {r}", self.type_name()),
        }
    }

    /// Structural equality, without recursion.
    pub fn equals(&self, other: &Value) -> bool {
        let mut todo = vec![(self, other)];
        while let Some((a, b)) = todo.pop() {
            let same = match (a, b) {
                (Value::Unit, Value::Unit) => true,
                (Value::Bool(x), Value::Bool(y)) => x == y,
                (Value::Int(x), Value::Int(y)) => x == y,
                (Value::Int(x), Value::Float(y)) | (Value::Float(y), Value::Int(x)) => (*x as f64) == *y,
                (Value::Float(x), Value::Float(y)) => x == y,
                (Value::Text(x), Value::Text(y)) => x == y,
                (Value::List(x), Value::List(y)) => {
                    if x.len() != y.len() {
                        return false;
                    }
                    todo.extend(x.iter().zip(y.iter()));
                    true
                }
                (Value::Record(x), Value::Record(y)) => {
                    if x.len() != y.len() {
                        return false;
                    }
                    for (k, v) in x.iter() {
                        match y.get(k) {
                            Some(w) => todo.push((v, w)),
                            None => return false,
                        }
                    }
                    true
                }
                (Value::Variant(x), Value::Variant(y)) => {
                    if x.ctor.name != y.ctor.name || x.ctor.type_name != y.ctor.type_name || x.fields.len() != y.fields.len() {
                        return false;
                    }
                    todo.extend(x.fields.iter().zip(y.fields.iter()));
                    true
                }
                (Value::Func(x), Value::Func(y)) => Rc::ptr_eq(x, y),
                (Value::Builtin(x), Value::Builtin(y)) => std::ptr::eq(*x, *y),
                (Value::Ctor(x), Value::Ctor(y)) => x.name == y.name,
                _ => false,
            };
            if !same {
                return false;
            }
        }
        true
    }

    /// Order between two values of the same kind; `None` if they cannot be compared.
    pub fn compare(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Some(a.cmp(b)),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Text(a), Value::Text(b)) => Some(a.cmp(b)),
            (Value::Bool(a), Value::Bool(b)) => Some(a.cmp(b)),
            (Value::Unit, Value::Unit) => Some(Ordering::Equal),
            (Value::List(a), Value::List(b)) => {
                for (x, y) in a.iter().zip(b.iter()) {
                    match x.compare(y)? {
                        Ordering::Equal => continue,
                        o => return Some(o),
                    }
                }
                Some(a.len().cmp(&b.len()))
            }
            (Value::Variant(a), Value::Variant(b)) if a.ctor.type_name == b.ctor.type_name => {
                if a.ctor.name != b.ctor.name {
                    return None;
                }
                for (x, y) in a.fields.iter().zip(b.fields.iter()) {
                    match x.compare(y)? {
                        Ordering::Equal => continue,
                        o => return Some(o),
                    }
                }
                Some(Ordering::Equal)
            }
            _ => None,
        }
    }

    /// A total order over all values, used to sort without ever failing:
    /// first by kind, then by value. Agrees with `compare` where it is defined.
    pub fn total_cmp(&self, other: &Value) -> Ordering {
        fn rank(v: &Value) -> u8 {
            match v {
                Value::Unit => 0,
                Value::Bool(_) => 1,
                Value::Int(_) | Value::Float(_) => 2,
                Value::Text(_) => 3,
                Value::List(_) => 4,
                Value::Record(_) => 5,
                Value::Variant(_) => 6,
                Value::Func(_) | Value::Builtin(_) | Value::Ctor(_) => 7,
            }
        }
        // Numbers: by value, then whole numbers before decimals, then exact value.
        fn number_key(v: &Value) -> (f64, bool, i64) {
            match v {
                Value::Int(n) => (*n as f64, false, *n),
                Value::Float(f) => (*f, true, 0),
                _ => (0.0, false, 0),
            }
        }
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => {
                let (a, b) = (number_key(self), number_key(other));
                a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2))
            }
            (Value::Text(a), Value::Text(b)) => a.cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => {
                for (x, y) in a.iter().zip(b.iter()) {
                    match x.total_cmp(y) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }
                a.len().cmp(&b.len())
            }
            (Value::Variant(a), Value::Variant(b)) => {
                let by_name = a.ctor.type_name.name().cmp(&b.ctor.type_name.name()).then(a.ctor.name.name().cmp(&b.ctor.name.name()));
                if by_name != Ordering::Equal {
                    return by_name;
                }
                for (x, y) in a.fields.iter().zip(b.fields.iter()) {
                    match x.total_cmp(y) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }
                a.fields.len().cmp(&b.fields.len())
            }
            _ => match rank(self).cmp(&rank(other)) {
                Ordering::Equal => self.repr().cmp(&other.repr()),
                o => o,
            },
        }
    }
}

/// Values that may hold other values.
fn is_container(v: &Value) -> bool {
    matches!(v, Value::List(_) | Value::Record(_) | Value::Variant(_))
}

/// Frees nested values one by one instead of recursively, so that a structure
/// nested millions of levels deep does not overflow the stack when it is freed.
fn drop_iteratively(mut todo: Vec<Value>) {
    while let Some(v) = todo.pop() {
        match v {
            // A value whose parts hold no other values is freed directly.
            Value::Variant(mut rc) => {
                if let Some(var) = Rc::get_mut(&mut rc) {
                    if var.fields.iter().any(is_container) {
                        todo.append(&mut var.fields);
                    }
                }
            }
            Value::List(mut l) => {
                if let Some(items) = Rc::get_mut(&mut l.data) {
                    if items.iter().any(is_container) {
                        todo.append(items);
                    }
                }
            }
            Value::Record(mut rc) => {
                if let Some(r) = Rc::get_mut(&mut rc) {
                    if r.vals.iter().any(is_container) {
                        todo.append(&mut r.vals);
                    }
                }
            }
            _ => {}
        }
    }
}

impl Drop for Variant {
    fn drop(&mut self) {
        if self.fields.iter().any(is_container) {
            drop_iteratively(std::mem::take(&mut self.fields));
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        if let Some(items) = Rc::get_mut(&mut self.data) {
            if items.iter().any(is_container) {
                drop_iteratively(std::mem::take(items));
            }
        }
    }
}

impl Drop for Record {
    fn drop(&mut self) {
        if self.vals.iter().any(is_container) {
            drop_iteratively(std::mem::take(&mut self.vals));
        }
    }
}

fn is_plain_key(k: &str) -> bool {
    let mut chars = k.chars();
    matches!(chars.next(), Some(c) if c.is_alphabetic() || c == '_') && chars.all(|c| c.is_alphanumeric() || c == '_')
}

fn write_text_repr(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '{' => out.push_str("\\{"),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Decimal numbers always show a decimal point: `3.0`, `0.5`, `1e300`.
pub fn fmt_float(f: f64) -> String {
    if f.is_nan() {
        "nan".into()
    } else if f.is_infinite() {
        if f > 0.0 {
            "inf".into()
        } else {
            "-inf".into()
        }
    } else if f != 0.0 && (f.abs() >= 1e16 || f.abs() < 1e-5) {
        format!("{f:e}")
    } else if f == f.trunc() {
        format!("{f:.1}")
    } else {
        format!("{f}")
    }
}
