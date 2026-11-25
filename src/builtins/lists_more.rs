//! List functions that students and professionals expect from Ruby's
//! Enumerable, Python's itertools and Elixir's Enum: finding a position,
//! changing one element, turning and splitting, counting values (`tally`),
//! sorting from the largest, runs of the same key, combinations,
//! permutations and products, and lists built from a function (`iterate`,
//! `unfold`). Every result is bounded: a list too large is E023.

use super::*;
use crate::dict::{key_cmp, Map};

pub(super) fn find_index(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "find_index", span)?;
    let f = func(a, 1, "find_index", span)?;
    for (i, x) in l.iter().enumerate() {
        if test(it, &f, x, "find_index", span)? {
            return Ok(it.some(Value::Int(i as i64)));
        }
    }
    Ok(it.none())
}

/// From the end, so that the function is called only on the elements after
/// the one found, as JavaScript's `findLast` does.
pub(super) fn find_last(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "find_last", span)?;
    let f = func(a, 1, "find_last", span)?;
    for x in l.iter().rev() {
        if test(it, &f, x, "find_last", span)? {
            return Ok(it.some(x.clone()));
        }
    }
    Ok(it.none())
}

pub(super) fn find_last_index(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "find_last_index", span)?;
    let f = func(a, 1, "find_last_index", span)?;
    for (i, x) in l.iter().enumerate().rev() {
        if test(it, &f, x, "find_last_index", span)? {
            return Ok(it.some(Value::Int(i as i64)));
        }
    }
    Ok(it.none())
}

pub(super) fn update_at(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "update_at", span)?;
    int(a, 1, "update_at", span)?;
    let f = func(a, 2, "update_at", span)?;
    let i = position(a, 1, l.len(), "update_at", span)?;
    let new = call_given1(it, "update_at", &f, l.as_slice()[i].clone(), span)?;
    let mut v = l.as_slice().to_vec();
    v[i] = new;
    Ok(Value::list(v))
}

pub(super) fn rotate_by(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let len = length_of(&a[0], "rotate_by", span)?;
    let n = if a.len() == 2 { int(a, 1, "rotate_by", span)? } else { 1 };
    if len == 0 {
        return Ok(a[0].clone());
    }
    let k = n.rem_euclid(len as i64) as usize;
    Ok(match &a[0] {
        Value::List(l) => Value::list(l.as_slice()[k..].iter().chain(&l.as_slice()[..k]).cloned().collect()),
        Value::Text(s) => {
            let at = s.char_indices().nth(k).map_or(s.len(), |(b, _)| b);
            text_value(format!("{}{}", &s[at..], &s[..at]))
        }
        _ => Value::Unit,
    })
}

pub(super) fn intersperse(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "intersperse", span)?;
    if l.len().saturating_mul(2) > MAX_LIST {
        return Err(too_big(true, span));
    }
    let mut out = Vec::with_capacity(l.len() * 2);
    for (i, x) in l.iter().enumerate() {
        if i > 0 {
            out.push(a[1].clone());
        }
        out.push(x.clone());
    }
    Ok(Value::list(out))
}

pub(super) fn interleave(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = list(a, 0, "interleave", span)?;
    let y = list(a, 1, "interleave", span)?;
    if x.len().saturating_add(y.len()) > MAX_LIST {
        return Err(too_big(true, span));
    }
    let mut out = Vec::with_capacity(x.len() + y.len());
    let (mut p, mut q) = (x.iter(), y.iter());
    loop {
        match (p.next(), q.next()) {
            (None, None) => break,
            (u, v) => out.extend(u.into_iter().chain(v).cloned()),
        }
    }
    Ok(Value::list(out))
}

/// The keys that are different, each with how many times it comes and
/// where it comes first, in the order of the keys. Each key must be one
/// that a dict accepts.
fn counted(keys: Vec<Value>, f: &str, span: Span) -> Fallible<Vec<(Value, i64, usize)>> {
    for k in &keys {
        dicts::key(f, k, span)?;
    }
    let mut order: Vec<usize> = (0..keys.len()).collect();
    order.sort_by(|&i, &j| key_cmp(&keys[i], &keys[j]));
    let mut out: Vec<(Value, i64, usize)> = Vec::new();
    for i in order {
        match out.last_mut() {
            Some(last) if key_cmp(&last.0, &keys[i]) == Ordering::Equal => last.1 += 1,
            _ => out.push((keys[i].clone(), 1, i)),
        }
    }
    Ok(out)
}

fn counts_dict(counts: Vec<(Value, i64, usize)>) -> Value {
    Value::Dict(Map::from_sorted(counts.into_iter().map(|(k, n, _)| (k, Value::Int(n))).collect()))
}

pub(super) fn tally(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "tally", span)?;
    Ok(counts_dict(counted(l.as_slice().to_vec(), "tally", span)?))
}

pub(super) fn count_by(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "count_by", span)?;
    let f = func(a, 1, "count_by", span)?;
    let mut keys = Vec::with_capacity(l.len());
    for x in l.iter() {
        keys.push(call_given1(it, "count_by", &f, x.clone(), span)?);
    }
    Ok(counts_dict(counted(keys, "count_by", span)?))
}

pub(super) fn most_common(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "most_common", span)?;
    let n = if a.len() == 2 { count_arg(a, 1, "most_common", span)? } else { usize::MAX };
    let mut counts = counted(l.as_slice().to_vec(), "most_common", span)?;
    // The most frequent first; on a tie, the one that comes first in the list.
    counts.sort_by(|x, y| y.1.cmp(&x.1).then(x.2.cmp(&y.2)));
    Ok(Value::list(counts.into_iter().take(n).map(|(k, c, _)| Value::list(vec![k, Value::Int(c)])).collect()))
}

pub(super) fn unique_by(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "unique_by", span)?;
    let f = func(a, 1, "unique_by", span)?;
    let mut seen: Vec<Value> = Vec::new();
    let mut found = Map::new();
    let mut out = Vec::new();
    for x in l.iter() {
        let k = call_given1(it, "unique_by", &f, x.clone(), span)?;
        // Keys a dict accepts are found fast; the others (`nan`, functions)
        // are compared with `==` to those seen before.
        let new = if dicts::findable(&k) {
            let new = !found.has(&k);
            if new {
                found = found.insert(k, Value::Unit);
            }
            new
        } else {
            let new = !seen.iter().any(|s| s.equals(&k));
            if new {
                seen.push(k);
            }
            new
        };
        if new {
            out.push(x.clone());
        }
    }
    Ok(Value::list(out))
}

/// Sorts from the largest to the smallest; equal elements keep their order.
fn sorted_desc(mut pairs: Vec<(Value, Value)>, name: &str, span: Span) -> Fallible<Value> {
    pairs.reverse();
    sort_pairs(&mut pairs, name, span)?;
    pairs.reverse();
    Ok(Value::list(pairs.into_iter().map(|p| p.1).collect()))
}

pub(super) fn sort_desc(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "sort_desc", span)?;
    if l.iter().all(|v| matches!(v, Value::Int(_))) {
        let mut ints: Vec<i64> = l.iter().map(|v| if let Value::Int(n) = v { *n } else { 0 }).collect();
        ints.sort_unstable_by(|x, y| y.cmp(x));
        return Ok(Value::list(ints.into_iter().map(Value::Int).collect()));
    }
    sorted_desc(l.iter().map(|v| (v.clone(), v.clone())).collect(), "sort_desc", span)
}

pub(super) fn sort_by_desc(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "sort_by_desc", span)?;
    let f = func(a, 1, "sort_by_desc", span)?;
    let mut pairs = Vec::with_capacity(l.len());
    for x in l.iter() {
        pairs.push((call_given1(it, "sort_by_desc", &f, x.clone(), span)?, x.clone()));
    }
    sorted_desc(pairs, "sort_by_desc", span)
}

pub(super) fn chunk_by(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "chunk_by", span)?;
    let f = func(a, 1, "chunk_by", span)?;
    let mut out = Vec::new();
    let mut run: Vec<Value> = Vec::new();
    let mut last: Option<Value> = None;
    for x in l.iter() {
        let k = call_given1(it, "chunk_by", &f, x.clone(), span)?;
        if last.as_ref().is_some_and(|p| !p.equals(&k)) {
            out.push(Value::list(std::mem::take(&mut run)));
        }
        run.push(x.clone());
        last = Some(k);
    }
    if !run.is_empty() {
        out.push(Value::list(run));
    }
    Ok(Value::list(out))
}

pub(super) fn split_at(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let len = length_of(&a[0], "split_at", span)?;
    let i = int(a, 1, "split_at", span)?;
    let (at, _) = bounds(len, i, None);
    Ok(Value::list(vec![cut(&a[0], 0, at), cut(&a[0], at, len)]))
}

/// The most elements, all lists together, that `combinations`,
/// `permutations` and `cartesian_product` give: beyond, E023.
const MAX_COMBINED: u128 = 10_000_000;

/// Checks that `count` lists of `size` elements stay within the bound.
fn combined(f: &str, count: u128, size: usize, span: Span) -> Fallible<()> {
    if count.saturating_mul(size.max(1) as u128) > MAX_COMBINED {
        return Err(Diagnostic::error(
            "E023",
            t!(
                "`{f}` would give {count} lists: too many (at most {MAX_COMBINED} elements in all)",
                "`{f}` donnerait {count} listes : beaucoup trop (au plus {MAX_COMBINED} éléments en tout)"
            ),
        )
        .at(span)
        .help(t!(
            "the number of combinations grows very fast: try with a shorter list, or fewer elements in each one",
            "le nombre de combinaisons grandit très vite : essayez avec une liste plus courte, ou moins d'éléments dans chacune"
        ))
        .boxed());
    }
    Ok(())
}

/// n! / (n - k)! / (k! if `choose`), or `u128::MAX` when it is huge.
fn arrangements(n: usize, k: usize, choose: bool) -> u128 {
    let mut c: u128 = 1;
    for i in 0..k as u128 {
        c = match c.checked_mul(n as u128 - i) {
            Some(x) if choose => x / (i + 1),
            Some(x) => x,
            None => return u128::MAX,
        };
    }
    c
}

pub(super) fn combinations(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "combinations", span)?;
    let k = count_arg(a, 1, "combinations", span)?;
    let n = l.len();
    if k > n {
        return Ok(Value::list(Vec::new()));
    }
    combined("combinations", arrangements(n, k, true), k, span)?;
    let items = l.as_slice();
    let mut out = Vec::new();
    let mut idx: Vec<usize> = (0..k).collect();
    loop {
        out.push(Value::list(idx.iter().map(|&i| items[i].clone()).collect()));
        // The next set of positions, in increasing order.
        let Some(i) = (0..k).rev().find(|&i| idx[i] != i + n - k) else { break };
        idx[i] += 1;
        for j in i + 1..k {
            idx[j] = idx[j - 1] + 1;
        }
    }
    Ok(Value::list(out))
}

pub(super) fn permutations(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "permutations", span)?;
    let n = l.len();
    let k = if a.len() == 2 { count_arg(a, 1, "permutations", span)? } else { n };
    if k > n {
        return Ok(Value::list(Vec::new()));
    }
    combined("permutations", arrangements(n, k, false), k, span)?;
    let items = l.as_slice();
    let mut out = Vec::new();
    // Depth first, in the order of the list: the order Python gives.
    fn go(items: &[Value], k: usize, used: &mut Vec<bool>, current: &mut Vec<Value>, out: &mut Vec<Value>) {
        if current.len() == k {
            out.push(Value::list(current.clone()));
            return;
        }
        for i in 0..items.len() {
            if !used[i] {
                used[i] = true;
                current.push(items[i].clone());
                go(items, k, used, current, out);
                current.pop();
                used[i] = false;
            }
        }
    }
    go(items, k, &mut vec![false; n], &mut Vec::with_capacity(k), &mut out);
    Ok(Value::list(out))
}

pub(super) fn cartesian_product(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let mut lists = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        lists.push(list(a, i, "cartesian_product", span)?.as_slice());
    }
    let count = lists.iter().fold(1u128, |acc, l| acc.saturating_mul(l.len() as u128));
    combined("cartesian_product", count, lists.len(), span)?;
    let mut out = Vec::with_capacity(count as usize);
    let mut idx = vec![0usize; lists.len()];
    if count == 0 {
        return Ok(Value::list(out));
    }
    loop {
        out.push(Value::list(idx.iter().zip(&lists).map(|(&i, l)| l[i].clone()).collect()));
        // Like an odometer: the last position turns first.
        let mut j = lists.len();
        loop {
            if j == 0 {
                return Ok(Value::list(out));
            }
            j -= 1;
            idx[j] += 1;
            if idx[j] < lists[j].len() {
                break;
            }
            idx[j] = 0;
        }
    }
}

pub(super) fn unzip(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "unzip", span)?;
    let mut rows = Vec::with_capacity(l.len());
    for (k, row) in l.iter().enumerate() {
        match row {
            Value::List(r) => rows.push(r.as_slice()),
            v => {
                return Err(Diagnostic::error(
                    "E005",
                    t!(
                        "`unzip` needs a list of lists, but its element {k} is {}",
                        "`unzip` a besoin d'une liste de listes, mais son élément {k} est {}",
                        v.describe()
                    ),
                )
                .at(span)
                .help(usage("unzip"))
                .boxed())
            }
        }
    }
    // As many columns as the shortest row has elements, like `zip`.
    let width = rows.iter().map(|r| r.len()).min().unwrap_or(0);
    Ok(Value::list((0..width).map(|c| Value::list(rows.iter().map(|r| r[c].clone()).collect())).collect()))
}

pub(super) fn iterate(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = count_arg(a, 1, "iterate", span)?;
    let f = func(a, 2, "iterate", span)?;
    if n > MAX_LIST {
        return Err(too_big(true, span));
    }
    let mut out = Vec::with_capacity(n.min(1 << 16));
    let mut x = a[0].clone();
    for i in 0..n {
        if i > 0 {
            x = call_given1(it, "iterate", &f, x, span)?;
        }
        out.push(x.clone());
    }
    Ok(Value::list(out))
}

pub(super) fn unfold(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = func(a, 1, "unfold", span)?;
    let mut state = a[0].clone();
    let mut out = Vec::new();
    loop {
        let r = call_given1(it, "unfold", &f, state, span)?;
        match it.wrap(&r) {
            Wrap::None => return Ok(Value::list(out)),
            Wrap::Some => match Interp::payload(&r) {
                Value::List(p) if p.len() == 2 => {
                    out.push(p.as_slice()[0].clone());
                    state = p.as_slice()[1].clone();
                }
                other => {
                    return Err(Diagnostic::error(
                        "E005",
                        t!(
                            "the function given to `unfold` must return `Some([value, next])` or `None`, but it returned Some({})",
                            "la fonction donnée à `unfold` doit renvoyer `Some([valeur, suivant])` ou `None`, mais elle a renvoyé Some({})",
                            other.repr()
                        ),
                    )
                    .at(span)
                    .boxed())
                }
            },
            _ => return Err(wrong_return("unfold", OPTION, &r.describe(), span)),
        }
        if out.len() > MAX_LIST {
            return Err(too_big(true, span));
        }
    }
}

pub(super) fn filter_map(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "filter_map", span)?;
    let f = func(a, 1, "filter_map", span)?;
    let mut out = Vec::new();
    for x in l.iter() {
        let r = call_given1(it, "filter_map", &f, x.clone(), span)?;
        match it.wrap(&r) {
            Wrap::Some => out.push(Interp::payload(&r)),
            Wrap::None => {}
            _ => return Err(wrong_return("filter_map", OPTION, &r.describe(), span)),
        }
    }
    Ok(Value::list(out))
}
