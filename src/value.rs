//! Runtime values. All of them are immutable: "changing" a value builds a new one.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::FnDef;
use crate::builtins::Builtin;
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
    start: usize,
    end: usize,
}

impl List {
    pub fn new(v: Vec<Value>) -> List {
        let end = v.len();
        List { data: Rc::new(v), start: 0, end }
    }

    pub fn as_slice(&self) -> &[Value] {
        &self.data[self.start..self.end]
    }

    pub fn len(&self) -> usize {
        self.end - self.start
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
        List { data: self.data.clone(), start: self.start + from, end: self.start + to }
    }

    pub fn get(&self, i: usize) -> Option<&Value> {
        self.as_slice().get(i)
    }
}

/// A record: named fields, kept in the order they were written.
#[derive(Clone, Default)]
pub struct Record {
    keys: Vec<Rc<str>>,
    vals: Vec<Value>,
    index: HashMap<Rc<str>, usize>,
}

impl Record {
    pub fn new() -> Record {
        Record::default()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.index.get(key).map(|&i| &self.vals[i])
    }

    pub fn set(&mut self, key: Rc<str>, val: Value) {
        match self.index.get(&key) {
            Some(&i) => self.vals[i] = val,
            None => {
                self.index.insert(key.clone(), self.keys.len());
                self.keys.push(key);
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
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn keys(&self) -> &[Rc<str>] {
        &self.keys
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Rc<str>, &Value)> {
        self.keys.iter().zip(self.vals.iter())
    }
}

/// The environment of local names. It is persistent: adding a name creates a
/// new environment and leaves the old one untouched.
#[derive(Clone, Default)]
pub struct Env(Option<Rc<EnvNode>>);

pub struct EnvNode {
    sym: Sym,
    val: Value,
    next: Env,
}

impl Env {
    pub fn empty() -> Env {
        Env(None)
    }

    pub fn bind(&self, sym: Sym, val: Value) -> Env {
        Env(Some(Rc::new(EnvNode { sym, val, next: self.clone() })))
    }

    pub fn lookup(&self, sym: Sym) -> Option<&Value> {
        let mut cur = &self.0;
        while let Some(node) = cur {
            if node.sym == sym {
                return Some(&node.val);
            }
            cur = &node.next.0;
        }
        None
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

pub struct Closure {
    pub def: Rc<FnDef>,
    pub env: Env,
    /// Local named functions see themselves (for recursion) through this.
    pub bind_self: bool,
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
            Value::Func(_) | Value::Builtin(_) => "Function".into(),
            Value::Ctor(c) => c.type_name.name().to_string(),
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

    /// How the value is written in code: texts with quotes.
    pub fn repr(&self) -> String {
        let mut out = String::new();
        self.write_repr(&mut out);
        out
    }

    fn write_repr(&self, out: &mut String) {
        match self {
            Value::Unit => out.push_str("()"),
            Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Value::Int(n) => out.push_str(&n.to_string()),
            Value::Float(f) => out.push_str(&fmt_float(*f)),
            Value::Text(s) => write_text_repr(s, out),
            Value::List(l) => {
                out.push('[');
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    v.write_repr(out);
                }
                out.push(']');
            }
            Value::Record(r) => {
                out.push('{');
                for (i, (k, v)) in r.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    if is_plain_key(k) {
                        out.push_str(k);
                    } else {
                        write_text_repr(k, out);
                    }
                    out.push_str(": ");
                    v.write_repr(out);
                }
                out.push('}');
            }
            Value::Variant(v) => {
                out.push_str(&v.ctor.name.name());
                if !v.fields.is_empty() {
                    out.push('(');
                    for (i, f) in v.fields.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        f.write_repr(out);
                    }
                    out.push(')');
                }
            }
            Value::Func(c) => match c.def.name {
                Some(n) => out.push_str(&format!("<fn {n}>")),
                None => out.push_str("<fn>"),
            },
            Value::Builtin(b) => out.push_str(&format!("<fn {}>", b.name)),
            Value::Ctor(c) => out.push_str(&format!("<{} {}>", t!("constructor", "constructeur"), c.name)),
        }
    }

    /// A short description for error messages: `Text "abc"`, `List [1, 2]`.
    pub fn describe(&self) -> String {
        let mut r = self.repr();
        if r.chars().count() > 40 {
            r = r.chars().take(37).collect::<String>() + "...";
        }
        match self {
            Value::Unit => "()".into(),
            Value::Func(_) | Value::Builtin(_) | Value::Ctor(_) => r,
            Value::Variant(_) => format!("{} {r}", self.type_name()),
            _ => format!("{} {r}", self.type_name()),
        }
    }

    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Unit, Value::Unit) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => (*a as f64) == *b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::List(a), Value::List(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.equals(y)),
            (Value::Record(a), Value::Record(b)) => a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|w| v.equals(w))),
            (Value::Variant(a), Value::Variant(b)) => {
                a.ctor.name == b.ctor.name
                    && a.ctor.type_name == b.ctor.type_name
                    && a.fields.len() == b.fields.len()
                    && a.fields.iter().zip(b.fields.iter()).all(|(x, y)| x.equals(y))
            }
            (Value::Func(a), Value::Func(b)) => Rc::ptr_eq(a, b),
            (Value::Builtin(a), Value::Builtin(b)) => std::ptr::eq(*a, *b),
            (Value::Ctor(a), Value::Ctor(b)) => a.name == b.name,
            _ => false,
        }
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
