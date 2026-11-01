//! Records and dictionaries, by their keys: a value deep inside (`dig`),
//! merging with a function, keys and values swapped (`invert`), keeping or
//! leaving out some keys, renaming the keys, changing one value with a
//! default (`update`); and the last questions on sets.

use super::*;
use crate::dict::Map;

pub(super) fn dig(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = list(a, 1, "dig", span)?;
    let mut x = a[0].clone();
    for k in path.iter() {
        let next = match (&x, k) {
            (Value::Record(r), Value::Text(name)) => r.get(name).cloned(),
            (Value::Dict(m), k) if dicts::findable(k) => m.get(k).cloned(),
            (Value::List(l), Value::Int(i)) => resolve_index(*i, l.len()).map(|i| l.as_slice()[i].clone()),
            (Value::Text(s), Value::Int(i)) => {
                let n = s.chars().count();
                resolve_index(*i, n).and_then(|i| s.chars().nth(i)).map(|c| text_value(c.to_string()))
            }
            _ => None,
        };
        match next {
            Some(v) => x = v,
            None => return Ok(it.none()),
        }
    }
    Ok(it.some(x))
}

/// Argument 1, of the same kind as argument 0 (a record or a dict).
fn same_kind_as_first<'a>(a: &'a [Value], f: &str, span: Span) -> Fallible<&'a Value> {
    match (&a[0], &a[1]) {
        (Value::Record(_), Value::Record(_)) | (Value::Dict(_), Value::Dict(_)) => Ok(&a[1]),
        (Value::Record(_), v) => Err(wrong(f, 1, Expected::Record, v, span)),
        (Value::Dict(_), v) => Err(wrong(f, 1, Expected::Dict, v, span)),
        (v, _) => Err(wrong(f, 0, Expected::RecordOrDict, v, span)),
    }
}

pub(super) fn merge_with(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    same_kind_as_first(a, "merge_with", span)?;
    let f = func(a, 2, "merge_with", span)?;
    match (&a[0], &a[1]) {
        (Value::Record(x), Value::Record(y)) => {
            let mut out = (**x).clone();
            for (k, v) in y.iter() {
                let merged = match x.get(k) {
                    Some(old) => call_given2(it, "merge_with", &f, old.clone(), v.clone(), span)?,
                    None => v.clone(),
                };
                out.set(k.clone(), merged);
            }
            Ok(Value::Record(Rc::new(out)))
        }
        (Value::Dict(x), Value::Dict(y)) => {
            let mut out = x.clone();
            for (k, v) in y.iter() {
                let merged = match x.get(k) {
                    Some(old) => call_given2(it, "merge_with", &f, old.clone(), v.clone(), span)?,
                    None => v.clone(),
                };
                out = out.insert(k.clone(), merged);
            }
            Ok(Value::Dict(out))
        }
        _ => Ok(Value::Unit),
    }
}

pub(super) fn invert(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let pairs: Vec<(Value, Value)> = match &a[0] {
        Value::Record(r) => r.iter().map(|(k, v)| (v.clone(), text_value(&**k))).collect(),
        Value::Dict(m) => m.iter().map(|(k, v)| (v.clone(), k.clone())).collect(),
        v => return Err(wrong("invert", 0, Expected::RecordOrDict, v, span)),
    };
    for (k, _) in &pairs {
        dicts::key("invert", k, span)?;
    }
    Ok(Value::dict(pairs))
}

/// The keys of argument 1: a list or a set.
fn key_list(a: &[Value], f: &str, span: Span) -> Fallible<Vec<Value>> {
    match &a[1] {
        Value::List(l) => Ok(l.as_slice().to_vec()),
        Value::Set(m) => Ok(m.keys().cloned().collect()),
        v => Err(wrong(f, 1, Expected::ListOrSet, v, span)),
    }
}

fn keep_keys(a: &[Value], f: &str, keep: bool, span: Span) -> Fallible<Value> {
    if !matches!(a[0], Value::Record(_) | Value::Dict(_)) {
        return Err(wrong(f, 0, Expected::RecordOrDict, &a[0], span));
    }
    let keys = key_list(a, f, span)?;
    Ok(match &a[0] {
        Value::Record(r) => {
            let mut out = Record::new();
            for (k, v) in r.iter() {
                let listed = keys.iter().any(|x| matches!(x, Value::Text(t) if **t == **k));
                if listed == keep {
                    out.set(k.clone(), v.clone());
                }
            }
            Value::Record(Rc::new(out))
        }
        Value::Dict(m) => {
            let wanted = Map::from_pairs(keys.into_iter().filter(dicts::findable).map(|k| (k, Value::Unit)).collect());
            Value::Dict(m.filter_keys(&wanted, keep))
        }
        _ => Value::Unit,
    })
}

pub(super) fn select_keys(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    keep_keys(a, "select_keys", true, span)
}

pub(super) fn reject_keys(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    keep_keys(a, "reject_keys", false, span)
}

pub(super) fn map_keys(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if !matches!(a[0], Value::Record(_) | Value::Dict(_)) {
        return Err(wrong("map_keys", 0, Expected::RecordOrDict, &a[0], span));
    }
    let f = func(a, 1, "map_keys", span)?;
    match &a[0] {
        Value::Record(r) => {
            let mut out = Record::new();
            for (k, v) in r.iter() {
                match call_given1(it, "map_keys", &f, text_value(&**k), span)? {
                    Value::Text(name) => out.set(Rc::from(&*name), v.clone()),
                    other => return Err(wrong_return("map_keys", TEXT, &other.describe(), span)),
                }
            }
            Ok(Value::Record(Rc::new(out)))
        }
        Value::Dict(m) => {
            let mut pairs = Vec::with_capacity(m.len());
            for (k, v) in m.iter() {
                let new = call_given1(it, "map_keys", &f, k.clone(), span)?;
                dicts::key("map_keys", &new, span)?;
                pairs.push((new, v.clone()));
            }
            Ok(Value::dict(pairs))
        }
        _ => Ok(Value::Unit),
    }
}

pub(super) fn update(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Record(_) => {
            text(a, 1, "update", span)?;
        }
        Value::Dict(_) => {}
        v => return Err(wrong("update", 0, Expected::RecordOrDict, v, span)),
    }
    let f = func(a, 3, "update", span)?;
    match &a[0] {
        Value::Record(r) => {
            let Value::Text(name) = &a[1] else { return Ok(Value::Unit) };
            let new = match r.get(name) {
                Some(old) => call_given1(it, "update", &f, old.clone(), span)?,
                None => a[2].clone(),
            };
            let mut out = (**r).clone();
            out.set(Rc::from(&**name), new);
            Ok(Value::Record(Rc::new(out)))
        }
        Value::Dict(m) => {
            let k = dicts::key("update", &a[1], span)?;
            let new = match m.get(k) {
                Some(old) => call_given1(it, "update", &f, old.clone(), span)?,
                None => a[2].clone(),
            };
            Ok(Value::Dict(m.insert(k.clone(), new)))
        }
        _ => Ok(Value::Unit),
    }
}

fn two_sets<'a>(a: &'a [Value], f: &str, span: Span) -> Fallible<(&'a Map, &'a Map)> {
    match (&a[0], &a[1]) {
        (Value::Set(x), Value::Set(y)) => Ok((x, y)),
        (Value::Set(_), v) => Err(wrong(f, 1, Expected::Set, v, span)),
        (v, _) => Err(wrong(f, 0, Expected::Set, v, span)),
    }
}

pub(super) fn symmetric_difference(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, y) = two_sets(a, "symmetric_difference", span)?;
    Ok(Value::Set(x.filter_keys(y, false).union(&y.filter_keys(x, false))))
}

pub(super) fn is_disjoint(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, y) = two_sets(a, "is_disjoint", span)?;
    let (small, large) = if x.len() <= y.len() { (x, y) } else { (y, x) };
    Ok(Value::Bool(!small.keys().any(|k| large.has(k))))
}
