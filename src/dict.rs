//! Persistent ordered maps: what dictionaries and sets are made of.
//!
//! A [`Map`] is a weight-balanced binary search tree (the kind Haskell's
//! `Data.Map` uses): every node knows the size of its subtree, and no side
//! of a node is more than three times larger than the other, so the tree is
//! never deeper than about 2.5 log2(n). Nodes are shared, never changed:
//! adding a key builds a new path from the root to the key (O(log n) new
//! nodes) and shares everything else with the old map, which stays as it was.
//!
//! Keys are ordered by [`key_cmp`], a total order over the values that can
//! be keys (all of them but functions and `nan`), which agrees with `==`:
//! two keys are the same key exactly when they are equal (`1` and `1.0` too).
//! A set is a map whose values are all `()`.

use std::cmp::Ordering;
use std::rc::Rc;

use crate::value::Value;

type Link = Option<Rc<Node>>;

struct Node {
    key: Value,
    val: Value,
    size: usize,
    left: Link,
    right: Link,
}

/// An immutable map from keys to values, ordered by its keys.
#[derive(Clone, Default)]
pub struct Map {
    root: Link,
}

/// A side is rebalanced when it is more than `DELTA` times larger than the
/// other; `RATIO` chooses between a single and a double rotation. These are
/// the values proven correct for insertion and deletion (Hirai and Yamamoto, 2011).
const DELTA: usize = 3;
const RATIO: usize = 2;

fn size(l: &Link) -> usize {
    l.as_ref().map_or(0, |n| n.size)
}

fn node(key: Value, val: Value, left: Link, right: Link) -> Link {
    Some(Rc::new(Node { size: size(&left) + size(&right) + 1, key, val, left, right }))
}

/// A node whose sides were balanced, but one of which may have grown or
/// shrunk by one element since.
fn balance(key: Value, val: Value, left: Link, right: Link) -> Link {
    let (sl, sr) = (size(&left), size(&right));
    if sl + sr <= 1 {
        node(key, val, left, right)
    } else if sr > DELTA * sl {
        let Some(r) = right else { unreachable!() };
        if size(&r.left) < RATIO * size(&r.right) {
            node(r.key.clone(), r.val.clone(), node(key, val, left, r.left.clone()), r.right.clone())
        } else {
            let Some(rl) = &r.left else { unreachable!() };
            node(
                rl.key.clone(),
                rl.val.clone(),
                node(key, val, left, rl.left.clone()),
                node(r.key.clone(), r.val.clone(), rl.right.clone(), r.right.clone()),
            )
        }
    } else if sl > DELTA * sr {
        let Some(l) = left else { unreachable!() };
        if size(&l.right) < RATIO * size(&l.left) {
            node(l.key.clone(), l.val.clone(), l.left.clone(), node(key, val, l.right.clone(), right))
        } else {
            let Some(lr) = &l.right else { unreachable!() };
            node(
                lr.key.clone(),
                lr.val.clone(),
                node(l.key.clone(), l.val.clone(), l.left.clone(), lr.left.clone()),
                node(key, val, lr.right.clone(), right),
            )
        }
    } else {
        node(key, val, left, right)
    }
}

fn insert(t: &Link, key: Value, val: Value) -> Link {
    match t {
        None => node(key, val, None, None),
        Some(n) => match key_cmp(&key, &n.key) {
            Ordering::Less => balance(n.key.clone(), n.val.clone(), insert(&n.left, key, val), n.right.clone()),
            Ordering::Greater => balance(n.key.clone(), n.val.clone(), n.left.clone(), insert(&n.right, key, val)),
            // The key stays the one already there (`1` when `1.0` is put).
            Ordering::Equal => node(n.key.clone(), val, n.left.clone(), n.right.clone()),
        },
    }
}

fn remove(t: &Link, key: &Value) -> Link {
    match t {
        None => None,
        Some(n) => match key_cmp(key, &n.key) {
            Ordering::Less => balance(n.key.clone(), n.val.clone(), remove(&n.left, key), n.right.clone()),
            Ordering::Greater => balance(n.key.clone(), n.val.clone(), n.left.clone(), remove(&n.right, key)),
            Ordering::Equal => glue(&n.left, &n.right),
        },
    }
}

/// The two sides of a removed node, joined.
fn glue(left: &Link, right: &Link) -> Link {
    match (left, right) {
        (None, r) => r.clone(),
        (l, None) => l.clone(),
        (Some(l), Some(r)) => {
            if l.size > r.size {
                let (k, v, rest) = remove_max(l);
                balance(k, v, rest, right.clone())
            } else {
                let (k, v, rest) = remove_min(r);
                balance(k, v, left.clone(), rest)
            }
        }
    }
}

fn remove_min(n: &Node) -> (Value, Value, Link) {
    match &n.left {
        None => (n.key.clone(), n.val.clone(), n.right.clone()),
        Some(l) => {
            let (k, v, rest) = remove_min(l);
            (k, v, balance(n.key.clone(), n.val.clone(), rest, n.right.clone()))
        }
    }
}

fn remove_max(n: &Node) -> (Value, Value, Link) {
    match &n.right {
        None => (n.key.clone(), n.val.clone(), n.left.clone()),
        Some(r) => {
            let (k, v, rest) = remove_max(r);
            (k, v, balance(n.key.clone(), n.val.clone(), n.left.clone(), rest))
        }
    }
}

/// A perfectly balanced tree of the next `n` pairs, in order.
fn build(items: &mut impl Iterator<Item = (Value, Value)>, n: usize) -> Link {
    if n == 0 {
        return None;
    }
    let left = build(items, n / 2);
    let (k, v) = items.next()?;
    let right = build(items, n - n / 2 - 1);
    node(k, v, left, right)
}

impl Map {
    pub fn new() -> Map {
        Map::default()
    }

    /// A map of pairs whose keys are already in order, all different.
    pub fn from_sorted(pairs: Vec<(Value, Value)>) -> Map {
        let n = pairs.len();
        Map { root: build(&mut pairs.into_iter(), n) }
    }

    /// A map of pairs in any order: when a key comes several times, its last
    /// value wins (and its first key stays, `1` for `1` then `1.0`). The
    /// keys must be valid (see [`bad_key`]).
    pub fn from_pairs(mut pairs: Vec<(Value, Value)>) -> Map {
        // A stable sort keeps the pairs of one key in their order.
        pairs.sort_by(|a, b| key_cmp(&a.0, &b.0));
        let mut out: Vec<(Value, Value)> = Vec::with_capacity(pairs.len());
        for (k, v) in pairs {
            match out.last_mut() {
                Some(last) if key_cmp(&last.0, &k) == Ordering::Equal => last.1 = v,
                _ => out.push((k, v)),
            }
        }
        Map::from_sorted(out)
    }

    pub fn len(&self) -> usize {
        size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// The key as it is stored, and its value.
    pub fn get_entry(&self, key: &Value) -> Option<(&Value, &Value)> {
        let mut t = &self.root;
        while let Some(n) = t {
            match key_cmp(key, &n.key) {
                Ordering::Less => t = &n.left,
                Ordering::Greater => t = &n.right,
                Ordering::Equal => return Some((&n.key, &n.val)),
            }
        }
        None
    }

    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.get_entry(key).map(|(_, v)| v)
    }

    pub fn has(&self, key: &Value) -> bool {
        self.get_entry(key).is_some()
    }

    /// A new map where `key` gives `val`.
    pub fn insert(&self, key: Value, val: Value) -> Map {
        Map { root: insert(&self.root, key, val) }
    }

    /// A new map without `key` (the same map if it has no such key).
    pub fn remove(&self, key: &Value) -> Map {
        if !self.has(key) {
            return self.clone();
        }
        Map { root: remove(&self.root, key) }
    }

    /// The pairs, in the order of their keys.
    pub fn iter(&self) -> Iter<'_> {
        let mut it = Iter { stack: Vec::new(), left: self.len() };
        it.descend(&self.root);
        it
    }

    pub fn keys(&self) -> impl Iterator<Item = &Value> {
        self.iter().map(|(k, _)| k)
    }

    /// The pairs of both maps; for a key in both, the value of `other` wins
    /// (and the key of `self` stays).
    pub fn union(&self, other: &Map) -> Map {
        let (n, m) = (self.len(), other.len());
        if m == 0 {
            return self.clone();
        }
        if n == 0 {
            return other.clone();
        }
        // Few new keys: add them one by one.
        if m * log2(n) < n {
            let mut out = self.clone();
            for (k, v) in other.iter() {
                let key = match self.get_entry(k) {
                    Some((mine, _)) => mine.clone(),
                    None => k.clone(),
                };
                out = out.insert(key, v.clone());
            }
            return out;
        }
        let mut out = Vec::with_capacity(n + m);
        let (mut a, mut b) = (self.iter().peekable(), other.iter().peekable());
        loop {
            let order = match (a.peek(), b.peek()) {
                (Some(x), Some(y)) => key_cmp(x.0, y.0),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => break,
            };
            match order {
                Ordering::Less => {
                    let (k, v) = a.next().unwrap();
                    out.push((k.clone(), v.clone()));
                }
                Ordering::Greater => {
                    let (k, v) = b.next().unwrap();
                    out.push((k.clone(), v.clone()));
                }
                Ordering::Equal => {
                    let (k, _) = a.next().unwrap();
                    let (_, v) = b.next().unwrap();
                    out.push((k.clone(), v.clone()));
                }
            }
        }
        Map::from_sorted(out)
    }

    /// The pairs of `self` whose key is in `other` (`keep`), or is not (`!keep`).
    pub fn filter_keys(&self, other: &Map, keep: bool) -> Map {
        let (n, m) = (self.len(), other.len());
        let pairs: Vec<(Value, Value)> = if n * log2(m) < m + n {
            // Few keys to look for.
            self.iter().filter(|(k, _)| other.has(k) == keep).map(|(k, v)| (k.clone(), v.clone())).collect()
        } else if !keep && m * log2(n) < n {
            // Few keys to take away.
            let mut out = self.clone();
            for k in other.keys() {
                out = out.remove(k);
            }
            return out;
        } else {
            let mut out = Vec::new();
            let mut b = other.keys().peekable();
            for (k, v) in self.iter() {
                while b.peek().is_some_and(|x| key_cmp(x, k) == Ordering::Less) {
                    b.next();
                }
                let found = b.peek().is_some_and(|x| key_cmp(x, k) == Ordering::Equal);
                if found == keep {
                    out.push((k.clone(), v.clone()));
                }
            }
            out
        };
        Map::from_sorted(pairs)
    }

    /// Is every key of `self` a key of `other`?
    pub fn keys_within(&self, other: &Map) -> bool {
        self.len() <= other.len() && self.keys().all(|k| other.has(k))
    }

    /// Is the tree right: keys in strictly increasing order, the size of
    /// every node right, and every node balanced? (For the tests.)
    pub fn is_valid(&self) -> bool {
        fn check(t: &Link, low: Option<&Value>, high: Option<&Value>) -> bool {
            let Some(n) = t else { return true };
            let (sl, sr) = (size(&n.left), size(&n.right));
            n.size == sl + sr + 1
                && (sl + sr <= 1 || (sl <= DELTA * sr && sr <= DELTA * sl))
                && low.is_none_or(|k| key_cmp(k, &n.key) == Ordering::Less)
                && high.is_none_or(|k| key_cmp(&n.key, k) == Ordering::Less)
                && check(&n.left, low, Some(&n.key))
                && check(&n.right, Some(&n.key), high)
        }
        check(&self.root, None, None)
    }

    /// Takes the keys and values out of a map that nothing else shares,
    /// putting those that hold other values in `parts` (see `value::free_parts`).
    pub(crate) fn dismantle(&mut self, parts: &mut Vec<Value>) {
        fn take(link: Link, parts: &mut Vec<Value>) {
            if let Some(rc) = link {
                if let Ok(n) = Rc::try_unwrap(rc) {
                    let Node { key, val, left, right, .. } = n;
                    for v in [key, val] {
                        if crate::value::is_container(&v) {
                            parts.push(v);
                        }
                    }
                    take(left, parts);
                    take(right, parts);
                }
            }
        }
        take(self.root.take(), parts);
    }
}

/// log2(n) rounded up, at least 1.
fn log2(n: usize) -> usize {
    (usize::BITS - n.leading_zeros()).max(1) as usize
}

impl Drop for Map {
    fn drop(&mut self) {
        if self.root.as_ref().is_some_and(|r| Rc::strong_count(r) == 1) {
            let mut parts = Vec::new();
            self.dismantle(&mut parts);
            if !parts.is_empty() {
                crate::value::free_parts(&mut parts);
            }
        }
    }
}

/// The pairs of a map, in order.
pub struct Iter<'a> {
    stack: Vec<&'a Node>,
    left: usize,
}

impl<'a> Iter<'a> {
    fn descend(&mut self, mut t: &'a Link) {
        while let Some(n) = t {
            self.stack.push(n);
            t = &n.left;
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a Value, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.stack.pop()?;
        self.descend(&n.right);
        self.left -= 1;
        Some((&n.key, &n.val))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.left, Some(self.left))
    }
}

impl ExactSizeIterator for Iter<'_> {}

// ----- the order of keys ----------------------------------------------------

/// The part of a value that keeps it from being a key of a dictionary or an
/// element of a set: a function (functions cannot be compared) or `nan`
/// (which is not equal to itself).
pub fn bad_key(v: &Value) -> Option<&Value> {
    let mut todo = vec![v];
    while let Some(v) = todo.pop() {
        match v {
            Value::Func(_) | Value::Builtin(_) | Value::Ctor(_) => return Some(v),
            Value::Float(f) if f.is_nan() => return Some(v),
            Value::List(l) => todo.extend(l.iter()),
            Value::Record(r) => todo.extend(r.iter().map(|(_, x)| x)),
            Value::Variant(var) => todo.extend(var.fields.iter()),
            // The keys of a dict and the elements of a set are keys already.
            Value::Dict(m) => todo.extend(m.iter().map(|(_, x)| x)),
            Value::Set(_) => {}
            Value::Unit | Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::Text(_) => {}
        }
    }
    None
}

fn rank(v: &Value) -> u8 {
    match v {
        Value::Unit => 0,
        Value::Bool(_) => 1,
        Value::Int(_) | Value::Float(_) => 2,
        Value::Text(_) => 3,
        Value::List(_) => 4,
        Value::Record(_) => 5,
        Value::Variant(_) => 6,
        Value::Dict(_) => 7,
        Value::Set(_) => 8,
        Value::Func(_) | Value::Builtin(_) | Value::Ctor(_) => 9,
    }
}

fn seq_cmp<'a>(a: impl Iterator<Item = &'a Value>, b: impl Iterator<Item = &'a Value>) -> Ordering {
    let mut b = b;
    for x in a {
        match b.next() {
            None => return Ordering::Greater,
            Some(y) => match key_cmp(x, y) {
                Ordering::Equal => continue,
                o => return o,
            },
        }
    }
    if b.next().is_some() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

/// The order of the keys of dictionaries and of the elements of sets: a
/// total order over the values without functions nor `nan`, where two values
/// are in the same place exactly when they are equal (`==`). Where `<` works,
/// it agrees with it: numbers by their worth, texts alphabetically, lists
/// element by element... Values of different kinds go in the order: `()`,
/// Bool, numbers, Text, List, Record, values of types, Dict, Set.
pub fn key_cmp(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Text(x), Value::Text(y)) => x.cmp(y),
        (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => a.compare(b).unwrap_or(Ordering::Equal),
        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
        (Value::Unit, Value::Unit) => Ordering::Equal,
        (Value::List(x), Value::List(y)) => seq_cmp(x.iter(), y.iter()),
        (Value::Record(x), Value::Record(y)) => {
            // Records are equal whatever the order of their fields: compare
            // them by their names in order, then by their values.
            fn sorted(r: &crate::value::Record) -> Vec<(&str, &Value)> {
                let mut f: Vec<(&str, &Value)> = r.iter().map(|(k, v)| (&**k, v)).collect();
                f.sort_by(|p, q| p.0.cmp(q.0));
                f
            }
            let (fx, fy) = (sorted(x), sorted(y));
            fx.len()
                .cmp(&fy.len())
                .then_with(|| fx.iter().map(|p| p.0).cmp(fy.iter().map(|p| p.0)))
                .then_with(|| seq_cmp(fx.iter().map(|p| p.1), fy.iter().map(|p| p.1)))
        }
        (Value::Variant(x), Value::Variant(y)) => x
            .ctor
            .type_name
            .name()
            .cmp(&y.ctor.type_name.name())
            .then(x.ctor.index.cmp(&y.ctor.index))
            .then_with(|| x.ctor.name.name().cmp(&y.ctor.name.name()))
            .then_with(|| seq_cmp(x.fields.iter(), y.fields.iter())),
        (Value::Dict(x), Value::Dict(y)) => seq_cmp(x.iter().flat_map(|(k, v)| [k, v]), y.iter().flat_map(|(k, v)| [k, v])),
        (Value::Set(x), Value::Set(y)) => seq_cmp(x.keys(), y.keys()),
        _ => rank(a).cmp(&rank(b)),
    }
}
