//! Dictionaries and sets: `dict`, `set`, `union`, `intersection`,
//! `difference` and `is_subset`, and what the functions shared with lists
//! and records (`get`, `put`, `has`, `filter`...) do with them.
//!
//! Both are [`Map`]s, persistent ordered trees: "changing" one builds a new
//! one in O(log n), sharing almost everything with the old one.

use super::{call_given1, call_given2, test, wrong, wrong_return, Expected};
use crate::diagnostic::{Diagnostic, Fallible};
use crate::dict::{bad_key, Map};
use crate::interp::Interp;
use crate::source::Span;
use crate::types::BOOL;
use crate::value::Value;

/// The error for a key (or an element of a set) that cannot be compared.
fn bad_key_error(f: &str, key: &Value, bad: &Value, span: Span) -> Box<Diagnostic> {
    let mut d = Diagnostic::error(
        "E023",
        t!(
            "`{f}`: {} cannot be a key of a dict, nor an element of a set",
            "`{f}` : {} ne peut pas être une clé de dictionnaire, ni un élément d'ensemble",
            key.describe()
        ),
    )
    .at(span);
    if !std::ptr::eq(key, bad) {
        d = d.note(t!("it holds {}", "il contient {}", bad.describe()));
    }
    d.help(t!(
        "keys and elements must be values that can be compared: anything but functions and `nan`",
        "les clés et les éléments doivent être des valeurs comparables : tout sauf les fonctions et `nan`"
    ))
    .boxed()
}

/// A value that may be a key, or the error of `f`.
pub(super) fn key<'a>(f: &str, k: &'a Value, span: Span) -> Fallible<&'a Value> {
    match k {
        // The usual keys, at once.
        Value::Int(_) | Value::Text(_) | Value::Bool(_) | Value::Unit => Ok(k),
        _ => match bad_key(k) {
            Some(bad) => Err(bad_key_error(f, k, bad, span)),
            None => Ok(k),
        },
    }
}

/// Can this value be looked for in a dictionary or a set? (A function or
/// `nan` never is in one.)
pub(super) fn findable(k: &Value) -> bool {
    matches!(k, Value::Int(_) | Value::Text(_)) || bad_key(k).is_none()
}

fn pairs_error(f: &str, got: &Value, span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "E023",
        t!(
            "`{f}` needs a list of [key, value] pairs, but it contains {}",
            "`{f}` a besoin d'une liste de paires [clé, valeur], mais elle contient {}",
            got.describe()
        ),
    )
    .at(span)
    .help(t!(
        "each pair is a list of two elements, like `dict([[\"Ada\", 36], [\"Alan\", 41]])`",
        "chaque paire est une liste de deux éléments, comme `dict([[\"Ada\", 36], [\"Alan\", 41]])`"
    ))
    .boxed()
}

/// `dict()`, `dict([[key, value], ...])`, `dict(record)`.
pub fn dict(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let Some(x) = a.first() else { return Ok(Value::Dict(Map::new())) };
    match x {
        Value::List(l) => {
            let mut pairs = Vec::with_capacity(l.len());
            for p in l.iter() {
                match p {
                    Value::List(p) if p.len() == 2 => {
                        let k = key("dict", &p.as_slice()[0], span)?;
                        pairs.push((k.clone(), p.as_slice()[1].clone()));
                    }
                    other => return Err(pairs_error("dict", other, span)),
                }
            }
            Ok(Value::Dict(Map::from_pairs(pairs)))
        }
        Value::Record(r) => Ok(Value::Dict(Map::from_pairs(r.iter().map(|(k, v)| (Value::Text(k.clone()), v.clone())).collect()))),
        Value::Dict(_) => Ok(x.clone()),
        v => Err(wrong("dict", 0, Expected::Pairs, v, span)),
    }
}

/// `set()`, `set(list)`.
pub fn set(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let Some(x) = a.first() else { return Ok(Value::Set(Map::new())) };
    match x {
        Value::List(l) => {
            let mut items = Vec::with_capacity(l.len());
            for x in l.iter() {
                items.push((key("set", x, span)?.clone(), Value::Unit));
            }
            Ok(Value::Set(Map::from_pairs(items)))
        }
        Value::Set(_) => Ok(x.clone()),
        v => Err(wrong("set", 0, Expected::ListOrSet, v, span)),
    }
}

/// The two dictionaries, or the two sets, of `union`, `intersection` and
/// `difference`; and whether they are sets.
fn both<'a>(f: &str, a: &'a [Value], span: Span) -> Fallible<(&'a Map, &'a Map, bool)> {
    match (&a[0], &a[1]) {
        (Value::Set(x), Value::Set(y)) => Ok((x, y, true)),
        (Value::Dict(x), Value::Dict(y)) => Ok((x, y, false)),
        (Value::Set(_), v) => Err(wrong(f, 1, Expected::Set, v, span)),
        (Value::Dict(_), v) => Err(wrong(f, 1, Expected::Dict, v, span)),
        (v, _) => Err(wrong(f, 0, Expected::DictOrSet, v, span)),
    }
}

fn same_kind(m: Map, set: bool) -> Value {
    if set {
        Value::Set(m)
    } else {
        Value::Dict(m)
    }
}

pub fn union(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, y, set) = both("union", a, span)?;
    Ok(same_kind(x.union(y), set))
}

pub fn intersection(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, y, set) = both("intersection", a, span)?;
    Ok(same_kind(x.filter_keys(y, true), set))
}

pub fn difference(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, y, set) = both("difference", a, span)?;
    Ok(same_kind(x.filter_keys(y, false), set))
}

pub fn is_subset(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match (&a[0], &a[1]) {
        (Value::Set(x), Value::Set(y)) => Ok(Value::Bool(x.keys_within(y))),
        (Value::Set(_), v) => Err(wrong("is_subset", 1, Expected::Set, v, span)),
        (v, _) => Err(wrong("is_subset", 0, Expected::Set, v, span)),
    }
}

// ----- the functions shared with lists and records ----------------------

/// `put(dict, key, value)`.
pub(super) fn put(m: &Map, k: &Value, v: &Value, span: Span) -> Fallible<Value> {
    let k = key("put", k, span)?;
    Ok(Value::Dict(m.insert(k.clone(), v.clone())))
}

/// `push(set, x)`.
pub(super) fn push(m: &Map, x: &Value, span: Span) -> Fallible<Value> {
    let x = key("push", x, span)?;
    if m.has(x) {
        return Ok(Value::Set(m.clone()));
    }
    Ok(Value::Set(m.insert(x.clone(), Value::Unit)))
}

/// `remove(dict, key)` and `remove(set, x)`.
pub(super) fn remove(m: &Map, k: &Value) -> Map {
    if findable(k) {
        m.remove(k)
    } else {
        m.clone()
    }
}

/// `get(dict, key)`.
pub(super) fn get(it: &Interp, m: &Map, k: &Value) -> Value {
    it.option(if findable(k) { m.get(k).cloned() } else { None })
}

pub(super) fn has(m: &Map, k: &Value) -> bool {
    findable(k) && m.has(k)
}

/// `filter(dict, f)`: f receives each key and its value.
pub(super) fn filter_dict(it: &mut Interp, m: &Map, f: &Value, span: Span) -> Fallible<Value> {
    let mut out = Vec::new();
    for (k, v) in m.iter() {
        match call_given2(it, "filter", f, k.clone(), v.clone(), span)? {
            Value::Bool(true) => out.push((k.clone(), v.clone())),
            Value::Bool(false) => {}
            other => return Err(wrong_return("filter", BOOL, &other.describe(), span)),
        }
    }
    Ok(Value::Dict(Map::from_sorted(out)))
}

/// `filter(set, f)`.
pub(super) fn filter_set(it: &mut Interp, m: &Map, f: &Value, span: Span) -> Fallible<Value> {
    let mut out = Vec::new();
    for x in m.keys() {
        if test(it, f, x, "filter", span)? {
            out.push((x.clone(), Value::Unit));
        }
    }
    Ok(Value::Set(Map::from_sorted(out)))
}

/// `map(set, f)`: the set of what f gives.
pub(super) fn map_set(it: &mut Interp, m: &Map, f: &Value, span: Span) -> Fallible<Value> {
    let mut out = Vec::with_capacity(m.len());
    for x in m.keys() {
        let y = call_given1(it, "map", f, x.clone(), span)?;
        key("map", &y, span)?;
        out.push((y, Value::Unit));
    }
    Ok(Value::Set(Map::from_pairs(out)))
}

/// `map_values(dict, f)`.
pub(super) fn map_values(it: &mut Interp, m: &Map, f: &Value, span: Span) -> Fallible<Value> {
    let mut out = Vec::with_capacity(m.len());
    for (k, v) in m.iter() {
        out.push((k.clone(), call_given1(it, "map_values", f, v.clone(), span)?));
    }
    Ok(Value::Dict(Map::from_sorted(out)))
}

/// `entries(dict)`: the `[key, value]` pairs, in the order of the keys.
pub(super) fn entries(m: &Map) -> Value {
    Value::list(m.iter().map(|(k, v)| Value::list(vec![k.clone(), v.clone()])).collect())
}
