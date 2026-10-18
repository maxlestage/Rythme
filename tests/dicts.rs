//! Dictionaries and sets: what the functions give, how the values are
//! printed, compared and written as JSON, the errors and the checker; and
//! property tests of the persistent tree against a simple model.

mod common;

use std::cmp::Ordering;
use std::rc::Rc;

use common::{fails, ok, rythme, rythme_lang};
use rythme::dict::{bad_key, key_cmp, Map};
use rythme::value::{Record, Value};

#[test]
fn dicts_give_a_value_for_each_key() {
    ok(
        "d = dict([[2, \"b\"], [1, \"a\"]])\nprint(d, d[1], get(d, 2), get(d, 3), len(d), is_empty(d))",
        "dict(1 => \"a\", 2 => \"b\") a Some(\"b\") None 2 false",
    );
    ok(
        "print(dict(), dict([]), is_empty(dict()), dict({b: 1, a: 2}), dict(dict({x: 1})))",
        "dict() dict() true dict(\"a\" => 2, \"b\" => 1) dict(\"x\" => 1)",
    );
    // The last value of a key wins; the first key stays.
    ok("print(dict([[1, \"a\"], [2, \"b\"], [1.0, \"c\"]]), put(dict([[1, 0]]), 1.0, 5))", "dict(1 => \"c\", 2 => \"b\") dict(1 => 5)");
    // Keys of any kind but functions.
    ok(
        "d = dict([[[0, 0], \"origin\"], [{x: 1, y: 2}, \"p\"], [Some(1), \"s\"], [(), \"u\"], [true, \"t\"], [set([1]), \"set\"]])\nprint(d[[0, 0]], d[{y: 2, x: 1}], d[Some(1)], d[()], d[true], d[set([1])])",
        "origin p s u t set",
    );
    ok(
        "print(keys(dict([[\"b\", 1], [\"a\", 2]])), values(dict([[\"b\", 1], [\"a\", 2]])), entries(dict([[2, 3]])))",
        "[\"a\", \"b\"] [2, 1] [[2, 3]]",
    );
    ok(
        "d = dict([[1, 10]])\nprint(has(d, 1), has(d, 2), contains(d, 1), has(d, print), has(d, float(\"nan\")))",
        "true false true false false",
    );
    ok(
        "d = dict([[1, 10], [2, 20]])\nprint(remove(d, 1), remove(d, 5), remove(d, print), d)",
        "dict(2 => 20) dict(1 => 10, 2 => 20) dict(1 => 10, 2 => 20) dict(1 => 10, 2 => 20)",
    );
    ok(
        "print(map_values(dict([[1, 10]]), fn(x) = x + 1), filter(dict([[1, 10], [2, 20]]), fn(k, v) = k + v > 15))",
        "dict(1 => 11) dict(2 => 20)",
    );
    ok("d = dict([[1, 10]])\nprint(d.get(1), d.len(), d |> put(2, 0) |> keys)", "Some(10) 1 [1, 2]");
}

#[test]
fn dicts_merge_and_compare() {
    ok("print(union(dict({a: 1, b: 2}), dict({b: 20, c: 30})))", "dict(\"a\" => 1, \"b\" => 20, \"c\" => 30)");
    ok("print(union(dict([[1, \"one\"]]), dict([[1.0, \"un\"]])))", "dict(1 => \"un\")");
    ok(
        "print(intersection(dict({a: 1, b: 2}), dict({b: 0})), difference(dict({a: 1, b: 2}), dict({b: 0})))",
        "dict(\"b\" => 2) dict(\"a\" => 1)",
    );
    ok("print(dict({a: 1}) == dict([[\"a\", 1]]), dict({a: 1}) == dict({a: 2}), dict() == set(), dict() == {})", "true false false false");
    ok(
        "print(dict([[1, 2]]) < dict([[1, 3]]), dict([[1, 2]]) < dict([[0, 9]]), sort([dict([[2, 0]]), dict(), dict([[1, 5]])]))",
        "true false [dict(), dict(1 => 5), dict(2 => 0)]",
    );
    fails("print(dict([[1, print]]) < dict([[1, print]]))", "E005");
    fails("print(dict() + dict())", "E005");
}

#[test]
fn sets_hold_values_once() {
    ok("s = set([3, 1, 3, 2, 1.0])\nprint(s, len(s), has(s, 2), contains(s, 5), values(s))", "set(1, 2, 3) 3 true false [1, 2, 3]");
    ok(
        "print(set(), set([]) == set(), push(set([1]), 0), push(set([1]), 1), remove(set([1, 2]), 1), remove(set([1]), 7))",
        "set() true set(0, 1) set(1) set(2) set(1)",
    );
    ok(
        "a = set([1, 2, 3])\nb = set([2, 3, 4])\nprint(union(a, b), intersection(a, b), difference(a, b), difference(b, a))",
        "set(1, 2, 3, 4) set(2, 3) set(1) set(4)",
    );
    ok("print(is_subset(set([1]), set([1, 2])), is_subset(set([1, 3]), set([1, 2])), is_subset(set(), set()))", "true false true");
    ok(
        "print(map(set([1, 2, 3]), fn(x) = x % 2), filter(set([1, 2, 3]), is_odd), set([[1], [1], []]))",
        "set(0, 1) set(1, 3) set([], [1])",
    );
    ok("print(set([1, 2]) == set([2, 1]), set([1, 2]) < set([1, 3]), set([1]) < set([1, 0]))", "true true false");
    ok("print(type_of(set()), type_of(dict()), set([set([2]), set([1])]))", "Set Dict set(set(1), set(2))");
    // Values of different kinds, in the order of their kinds.
    ok("print(set([[1], \"x\", 2, (), false, {a: 1}, None, dict(), set()]))", "set((), false, 2, \"x\", [1], {a: 1}, None, dict(), set())");
}

#[test]
fn dicts_and_sets_as_json() {
    ok("print(to_json(dict({b: [1], a: set([2, 1])})), to_json(dict()), to_json(set()))", "{\"a\": [1, 2], \"b\": [1]} {} []");
    ok("print(to_json(dict({a: 1, b: 2}), 2))", "{\n  \"a\": 1,\n  \"b\": 2\n}");
    ok("print(dict(unwrap(parse_json(\"\\{\\\"x\\\": [1, 2]}\"))))", "dict(\"x\" => [1, 2])");
    let r = fails("to_json(dict([[1, 2]]))", "E023");
    assert!(r.stderr.contains("only when all its keys are texts"), "{}", r.stderr);
}

#[test]
fn wrong_keys_and_wrong_values_are_explained() {
    let r = fails("set([1, print])", "E023");
    assert!(r.stderr.contains("<fn print> cannot be a key of a dict, nor an element of a set"), "{}", r.stderr);
    let r = fails("dict([[[1, float(\"nan\")], 2]])", "E023");
    assert!(r.stderr.contains("it holds Float nan"), "{}", r.stderr);
    fails("put(dict(), fn(x) = x, 1)", "E023");
    fails("push(set(), float(\"nan\"))", "E023");
    fails("map(set([1]), fn(x) = print)", "E023");
    let r = fails("dict([1, 2])", "E023");
    assert!(r.stderr.contains("[key, value] pairs"), "{}", r.stderr);
    let r = fails("print(dict([[1, 2]])[3])", "E010");
    assert!(r.stderr.contains("this dict has no key 3"), "{}", r.stderr);
    fails("dict(5)", "E005");
    fails("set(\"abc\")", "E005");
    fails("union(set([1]), dict())", "E005");
    fails("is_subset(dict(), set())", "E005");
    let r = fails("sum(set([1, 2]))", "E005");
    assert!(r.stderr.contains("`values(set)`"), "{}", r.stderr);
    let r = fails("fold(dict(), 0, fn(a, b) = a)", "E005");
    assert!(r.stderr.contains("`entries(dict)`"), "{}", r.stderr);
    let r = fails("map(dict([[1, 2]]), fn(x) = x)", "E005");
    assert!(r.stderr.contains("map_values"), "{}", r.stderr);
    fails("filter(dict([[1, 2]]), fn(x) = true)", "E004");
    fails("filter(set([1]), fn(x) = 1)", "E005");
    let r = fails("print(union(set([1]), set([2])) + set([3]))", "E005");
    assert!(r.stderr.contains("union(a, b)"), "{}", r.stderr);
    let r = rythme_lang("fr", &["-e", "set([print])"]);
    assert!(r.stderr.contains("ne peut pas être une clé de dictionnaire"), "{}", r.stderr);
    let r = rythme(&["-e", "x = 1 => 2"]);
    assert!(r.stderr.contains("dict([[key, value], ...])"), "{}", r.stderr);
}

/// Runs `rythme check` on a program.
fn check(code: &str) -> common::Run {
    let dir = std::env::temp_dir().join(format!("rythme-dicts-{}-{}", std::process::id(), code.len()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("check.ry");
    std::fs::write(&file, code).unwrap();
    let r = rythme(&["check", file.to_str().unwrap()]);
    let _ = std::fs::remove_dir_all(&dir);
    r
}

#[test]
fn the_checker_knows_dicts_and_sets() {
    for (code, error) in [
        ("print(union(set([1]), [1]))", "`union` expects a set as argument 2"),
        ("print(is_subset([1], set()))", "`is_subset` expects a set as argument 1"),
        ("print(dict(5))", "`dict` expects a list of [key, value] pairs (or a record) as argument 1"),
        ("print(set([1]) |> sum)", "`sum` expects a list as argument 1"),
        ("print(filter(dict([[1, 2]]), fn(x) = x > 1))", "`filter` gives 2 argument(s) to this function, but it takes 1"),
        ("print(filter(set([1]), fn(x) = x + 1))", "must return true or false"),
        ("print(len(dict()) + \"a\")", "cannot use `+` with Int and Text"),
        ("print(upper(get(dict([[1, 2]]), 1)))", "`upper` expects a text as argument 1"),
        ("print(upper(values(set([1]))[0]))", "`upper` expects a text as argument 1"),
        ("print(5[1], put([1], 0, 1))", "`[...]` works on lists, texts, records and dicts"),
    ] {
        let r = check(code);
        assert!(r.stderr.contains(error), "{code}: expected `{error}`:\n{}", r.stderr);
    }
    // Correct programs are never rejected.
    let good = "fn pick(x) = filter(x, fn(k, v) = v > 1)
fn odd(x) = filter(x, is_odd)
d = dict([[1, 2], [2, 3]])
print(pick(d), odd([1, 2]), odd(set([1, 2])))
print(get(d, 1) |> unwrap_or(0) |> fn(n) = n + 1)
print(keys(d) |> map(fn(k) = k * 2), values(set([\"a\"])) |> map(upper))
print(map_values(dict({a: 1}), fn(n) = n * 2)[\"a\"] + 1, put(dict(), \"k\", [1])[\"k\"] |> len)
print(union(d, dict()) |> len, intersection(set([1]), set([1])) |> values, d[1] + 1)
print(set([1]) == set([1.0]), dict() != dict([[1, 1]]), entries(d)[0][1] + 1)
fn total(x) = fold(values(x), 0, fn(a, b) = a + b)
print(total(d), total(set([1, 2])), total({a: 1}))
";
    let r = check(good);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stderr.is_empty(), "{}", r.stderr);
}

#[test]
fn help_knows_dicts_and_sets() {
    let r = rythme(&["help", "dicts"]);
    assert!(r.stdout.contains("DICTIONARIES AND SETS") && r.stdout.contains("union(a, b)"), "{}", r.stdout);
    let r = rythme_lang("fr", &["help", "ensembles"]);
    assert!(r.stdout.contains("DICTIONNAIRES ET ENSEMBLES"), "{}", r.stdout);
    let r = rythme(&["help", "dict"]);
    assert!(r.stdout.contains("DICTIONARIES AND SETS") && r.stdout.contains("dict(pairs)"), "{}", r.stdout);
    ok("help(set([1]))", "This is a value of type Set: set(1)\nUseful functions: dict, set, union, intersection, difference, is_subset\nAlso: has, push, remove, len, is_empty, values, filter, map, to_json");
    let r = rythme(&["-e", "help(dict([[1, 2]]))"]);
    assert!(r.stdout.contains("Also: get, put, has, remove, keys"), "{}", r.stdout);
    let r = rythme(&["help", "builtins"]);
    assert!(r.stdout.contains("Dictionaries and sets\n  dict(pairs)"), "{}", r.stdout);
    let r = rythme(&["help", "hashset"]);
    assert!(format!("{}{}", r.stdout, r.stderr).contains("this is called `set`"), "{}", r.stderr);
}

#[test]
fn deeply_nested_dicts_and_sets_are_freed_compared_and_shown() {
    ok(
        "deep = fold(1..100000, dict(), fn(d, i) = dict([[i, d]]))
nested = fold(1..100000, set(), fn(s, i) = set([s]))
print(len(deep), len(nested), nested == nested, deep == deep, len(text(nested)))",
        "1 1 true true 500005",
    );
}

// ----- property tests -----------------------------------------------------

/// A small random generator (xorshift), the same on every run.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Keys of many kinds, often equal to one another (`1` and `1.0`, records
/// with their fields in another order).
fn key(rng: &mut Rng, depth: usize) -> Value {
    let small = |rng: &mut Rng| rng.below(8) as i64 - 3;
    match rng.below(if depth == 0 { 6 } else { 10 }) {
        0 | 1 => Value::Int(small(rng)),
        2 => Value::Float(small(rng) as f64),
        3 => Value::Float(small(rng) as f64 + 0.5),
        4 => Value::text(["", "a", "b", "ab", "é"][rng.below(5)]),
        5 => [Value::Unit, Value::Bool(true), Value::Bool(false)][rng.below(3)].clone(),
        6 => Value::list((0..rng.below(3)).map(|_| key(rng, depth - 1)).collect()),
        7 => {
            let mut fields = vec![("x", key(rng, depth - 1)), ("y", key(rng, depth - 1))];
            fields.truncate(1 + rng.below(2));
            if rng.below(2) == 0 {
                fields.reverse();
            }
            let mut r = Record::new();
            for (k, v) in fields {
                r.set(Rc::from(k), v);
            }
            Value::Record(Rc::new(r))
        }
        8 => Value::set((0..rng.below(3)).map(|_| key(rng, depth - 1)).collect()),
        _ => Value::dict((0..rng.below(3)).map(|_| (key(rng, depth - 1), key(rng, depth - 1))).collect()),
    }
}

/// The model: the pairs in a list, found with `==`.
#[derive(Clone)]
struct Model(Vec<(Value, Value)>);

impl Model {
    fn find(&self, k: &Value) -> Option<usize> {
        self.0.iter().position(|(x, _)| x.equals(k))
    }

    fn insert(&mut self, k: Value, v: Value) {
        match self.find(&k) {
            Some(i) => self.0[i].1 = v,
            None => self.0.push((k, v)),
        }
    }

    fn remove(&mut self, k: &Value) {
        if let Some(i) = self.find(k) {
            self.0.remove(i);
        }
    }
}

/// The map holds exactly the pairs of the model, in order, in a valid tree.
#[track_caller]
fn same(m: &Map, model: &Model) {
    assert!(m.is_valid(), "the tree is not valid");
    assert_eq!(m.len(), model.0.len());
    assert_eq!(m.iter().len(), model.0.len());
    for (k, v) in &model.0 {
        let (stored, found) = m.get_entry(k).unwrap_or_else(|| panic!("{} is missing", k.repr()));
        assert!(found.equals(v) && stored.equals(k), "{} gives {}, not {}", k.repr(), found.repr(), v.repr());
    }
    let keys: Vec<&Value> = m.keys().collect();
    assert!(keys.windows(2).all(|w| key_cmp(w[0], w[1]) == Ordering::Less), "keys out of order");
}

#[test]
fn the_tree_behaves_like_a_list_of_pairs() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for round in 0..300 {
        let mut m = Map::new();
        let mut model = Model(Vec::new());
        // Old versions stay as they were.
        let mut versions: Vec<(Map, Model)> = Vec::new();
        for step in 0..60 {
            let k = key(&mut rng, 2);
            match rng.below(10) {
                0..=4 => {
                    let v = Value::Int((round * 1000 + step) as i64);
                    m = m.insert(k.clone(), v.clone());
                    model.insert(k.clone(), v);
                }
                5 | 6 => {
                    m = m.remove(&k);
                    model.remove(&k);
                }
                7 => {
                    let other: Vec<(Value, Value)> = (0..rng.below(20)).map(|i| (key(&mut rng, 2), Value::Int(-(i as i64)))).collect();
                    let o = Map::from_pairs(other.clone());
                    let mut om = Model(Vec::new());
                    for (k, v) in other {
                        om.insert(k, v);
                    }
                    same(&o, &om);
                    match rng.below(3) {
                        0 => {
                            m = m.union(&o);
                            for (k, v) in &om.0 {
                                model.insert(k.clone(), v.clone());
                            }
                        }
                        n => {
                            let keep = n == 1;
                            m = m.filter_keys(&o, keep);
                            model.0.retain(|(k, _)| om.find(k).is_some() == keep);
                        }
                    }
                    assert_eq!(o.keys_within(&m), om.0.iter().all(|(k, _)| model.find(k).is_some()));
                }
                _ => versions.push((m.clone(), model.clone())),
            }
            assert_eq!(m.has(&k), model.find(&k).is_some());
            same(&m, &model);
        }
        for (old, old_model) in &versions {
            same(old, old_model);
        }
    }
}

#[test]
fn large_trees_stay_balanced() {
    let mut rng = Rng(12345);
    let mut m = Map::new();
    let mut present = std::collections::BTreeSet::new();
    for _ in 0..20_000 {
        let k = rng.below(5_000) as i64;
        if rng.below(3) == 0 {
            m = m.remove(&Value::Int(k));
            present.remove(&k);
        } else {
            m = m.insert(Value::Int(k), Value::Unit);
            present.insert(k);
        }
    }
    assert!(m.is_valid());
    let keys: Vec<i64> = m.keys().map(|k| if let Value::Int(n) = k { *n } else { panic!() }).collect();
    assert_eq!(keys, present.into_iter().collect::<Vec<_>>());
    let big = Map::from_pairs((0..10_000).rev().map(|i| (Value::Int(i % 7_000), Value::Int(i))).collect());
    assert!(big.is_valid() && big.len() == 7_000);
    // The first value of each key comes last: it wins.
    assert!(big.get(&Value::Int(5)).unwrap().equals(&Value::Int(5)));
    let both = big.union(&m);
    assert!(both.is_valid() && both.len() == 7_000);
    assert!(big.filter_keys(&m, true).is_valid() && big.filter_keys(&m, false).is_valid());
}

#[test]
fn the_order_of_keys_agrees_with_equality_and_comparison() {
    let mut rng = Rng(0xdead_beef);
    let values: Vec<Value> = (0..250).map(|_| key(&mut rng, 3)).collect();
    for a in &values {
        assert!(bad_key(a).is_none());
        assert_eq!(key_cmp(a, a), Ordering::Equal, "{}", a.repr());
        for b in &values {
            let o = key_cmp(a, b);
            assert_eq!(o == Ordering::Equal, a.equals(b), "{} and {}", a.repr(), b.repr());
            assert_eq!(o, key_cmp(b, a).reverse(), "{} and {}", a.repr(), b.repr());
            if let Some(c) = a.compare(b) {
                assert_eq!(o, c, "{} and {}: `<` says otherwise", a.repr(), b.repr());
            }
        }
    }
    for _ in 0..20_000 {
        let (a, b, c) = (&values[rng.below(250)], &values[rng.below(250)], &values[rng.below(250)]);
        if key_cmp(a, b) != Ordering::Greater && key_cmp(b, c) != Ordering::Greater {
            assert_ne!(key_cmp(a, c), Ordering::Greater, "{} {} {}", a.repr(), b.repr(), c.repr());
        }
    }
    assert!(bad_key(&Value::Float(f64::NAN)).is_some());
    assert!(bad_key(&Value::list(vec![Value::Int(1), Value::Builtin(rythme::builtins::lookup("len").unwrap())])).is_some());
}
