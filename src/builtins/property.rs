//! Property tests: `check_all(fn(x) = ..., generator)` calls the function
//! with many values made by a generator, and when one of them makes it fail,
//! simplifies that value (shrinking) before reporting it.
//!
//! A generator is a plain record, like `{generator: "ints", min: 0, max: 9}`,
//! made by `ints`, `floats`, `texts`, `bools`, `lists_of`, `one_of` and
//! `records_of`. The values come from a seed: the same seed gives the same
//! values, on every run and on every machine.

use super::*;

/// The seed used when none is given (`rythme test --seed N` gives another).
pub const DEFAULT_SEED: u64 = 42;

/// The number of values `check_all` tries when none is given.
const CASES: i64 = 100;
const MAX_CASES: i64 = 100_000;
/// The longest lists and texts a generator may be asked for.
const MAX_LENGTH: i64 = 1000;
/// Generators inside generators, at most.
const MAX_DEPTH: usize = 32;
/// The number of elements (of all lists and texts) in one generated value, at most.
const BUDGET: usize = 10_000;
/// Calls of the property while simplifying a failing value, at most.
const MAX_SHRINK_CALLS: usize = 2000;

/// The characters of generated texts: mostly letters, sometimes digits,
/// spaces, punctuation and characters outside of ASCII.
const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const PUNCTUATION: &str = ".,;:!?-'\"()";
const OTHERS: &str = "éèàçùüößñÉ€✓😀\t\n";

/// A generator, read from its record.
#[derive(Clone)]
enum Gen {
    Int(i64, i64),
    /// Whole numbers between bounds that do not both fit in 64 bits.
    Big(Rc<BigInt>, Rc<BigInt>),
    Float(f64, f64),
    Text(usize),
    Bool,
    List(Box<Gen>, usize),
    OneOf(Vec<Value>),
    Record(Vec<(Rc<str>, Gen)>),
}

/// splitmix64: small, fast, and good enough to make test values.
struct Rng(u64);

impl Rng {
    fn new(seed: u64, span: Span) -> Rng {
        // The place of the call in its file: two calls get different values,
        // and running only some tests (`--filter`) does not change them.
        Rng(seed ^ (u64::from(span.start) << 32 | u64::from(span.end)).wrapping_mul(0x9E37_79B9_7F4A_7C15))
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A number from 0 to n - 1 (n > 0).
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    /// A number from lo to hi, both included.
    fn between(&mut self, lo: i128, hi: i128) -> i128 {
        let size = (hi - lo + 1) as u128;
        lo + (u128::from(self.next()) % size) as i128
    }

    /// A number from 0 to 1 (1 excluded).
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn chance(&mut self, one_in: u64) -> bool {
        self.below(one_in) == 0
    }
}

fn field(name: &str, v: Value) -> (Rc<str>, Value) {
    (Rc::from(name), v)
}

fn make(kind: &str, fields: Vec<(Rc<str>, Value)>) -> Value {
    let mut r = Record::new();
    r.set(Rc::from("generator"), text_value(kind));
    for (k, v) in fields {
        r.set(k, v);
    }
    Value::Record(Rc::new(r))
}

fn not_a_generator(v: &Value, why: String, span: Span) -> Box<Diagnostic> {
    let mut d = Diagnostic::error("E023", t!("this is not a generator: {}", "ceci n'est pas un générateur : {}", v.describe()))
        .at(span)
        .label(t!("not a generator", "pas un générateur"));
    if !why.is_empty() {
        d = d.note(why);
    }
    d.help(t!(
        "make one with `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` or `records_of({{x: g}})`",
        "fabriquez-en un avec `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(liste)` ou `records_of({{x: g}})`"
    ))
    .boxed()
}

/// Reads the generator that is argument `i` of `f`.
fn generator(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Gen> {
    match &a[i] {
        Value::Record(_) => read(&a[i], 0, span),
        v => Err(wrong(f, i, Expected::Generator, v, span)),
    }
}

fn read(v: &Value, depth: usize, span: Span) -> Fallible<Gen> {
    let Value::Record(r) = v else {
        return Err(not_a_generator(v, String::new(), span));
    };
    if depth > MAX_DEPTH {
        return Err(not_a_generator(
            v,
            t!("generators can be put inside each other {MAX_DEPTH} times at most", "les générateurs s'emboîtent {MAX_DEPTH} fois au plus"),
            span,
        ));
    }
    let bad =
        |what: &str| not_a_generator(v, t!("its field `{what}` is missing or wrong", "son champ `{what}` est absent ou incorrect"), span);
    let int_field = |name: &str| match r.get(name) {
        Some(Value::Int(n)) => Ok(*n),
        _ => Err(bad(name)),
    };
    let whole_field = |name: &str| r.get(name).and_then(|v| bigint::big(v).map(std::borrow::Cow::into_owned)).ok_or_else(|| bad(name));
    let float_field = |name: &str| match r.get(name) {
        Some(Value::Int(n)) => Ok(*n as f64),
        Some(Value::Big(n)) => Ok(n.to_f64()),
        Some(Value::Float(x)) if x.is_finite() => Ok(*x),
        _ => Err(bad(name)),
    };
    let length = |name: &str| int_field(name).and_then(|n| if (0..=MAX_LENGTH).contains(&n) { Ok(n as usize) } else { Err(bad(name)) });
    let kind = match r.get("generator") {
        Some(Value::Text(k)) => k.clone(),
        _ => return Err(bad("generator")),
    };
    Ok(match &*kind {
        "ints" => {
            let (lo, hi) = (whole_field("min")?, whole_field("max")?);
            if lo > hi {
                return Err(bad("max"));
            }
            match (lo.to_i64(), hi.to_i64()) {
                (Some(lo), Some(hi)) => Gen::Int(lo, hi),
                _ => Gen::Big(Rc::new(lo), Rc::new(hi)),
            }
        }
        "floats" => {
            let (lo, hi) = (float_field("min")?, float_field("max")?);
            if lo > hi {
                return Err(bad("max"));
            }
            Gen::Float(lo, hi)
        }
        "texts" => Gen::Text(length("max_length")?),
        "bools" => Gen::Bool,
        "lists_of" => {
            let of = r.get("of").ok_or_else(|| bad("of"))?;
            Gen::List(Box::new(read(of, depth + 1, span)?), length("max_length")?)
        }
        "one_of" => match r.get("values") {
            Some(Value::List(l)) if !l.is_empty() => Gen::OneOf(l.iter().cloned().collect()),
            _ => return Err(bad("values")),
        },
        "records_of" => match r.get("fields") {
            Some(Value::Record(fields)) => {
                let mut out = Vec::new();
                for (k, g) in fields.iter() {
                    out.push((k.clone(), read(g, depth + 1, span)?));
                }
                Gen::Record(out)
            }
            _ => return Err(bad("fields")),
        },
        _ => return Err(bad("generator")),
    })
}

// ----- the generators ----------------------------------------------------

fn no_or_two(f: &str, a: &[Value], span: Span) -> Fallible<()> {
    if a.len() == 1 {
        return Err(invalid(
            t!(
                "`{f}` takes no argument, or two: the smallest and the largest value, like `{f}(0, 10)`",
                "`{f}` ne prend aucun argument, ou deux : la plus petite et la plus grande valeur, comme `{f}(0, 10)`"
            ),
            span,
        ));
    }
    Ok(())
}

fn min_above_max(f: &str, lo: &Value, hi: &Value, span: Span) -> Box<Diagnostic> {
    invalid(
        t!(
            "{f}({}, {}): the minimum is larger than the maximum",
            "{f}({}, {}) : le minimum est plus grand que le maximum",
            lo.repr(),
            hi.repr()
        ),
        span,
    )
}

pub(crate) fn ints(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    no_or_two("ints", a, span)?;
    if a.is_empty() {
        return Ok(make("ints", vec![field("min", Value::Int(-1000)), field("max", Value::Int(1000))]));
    }
    let (lo, hi) = (whole(a, 0, "ints", span)?, whole(a, 1, "ints", span)?);
    if lo > hi {
        return Err(min_above_max("ints", &a[0], &a[1], span));
    }
    Ok(make("ints", vec![field("min", bigint::value(lo.into_owned())), field("max", bigint::value(hi.into_owned()))]))
}

pub(crate) fn floats(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    no_or_two("floats", a, span)?;
    let (lo, hi) = if a.is_empty() { (-1000.0, 1000.0) } else { (number(a, 0, "floats", span)?, number(a, 1, "floats", span)?) };
    if !lo.is_finite() || !hi.is_finite() {
        return Err(invalid(
            t!(
                "`floats` needs finite numbers, not {} and {}",
                "`floats` a besoin de nombres finis, pas {} et {}",
                fmt_float(lo),
                fmt_float(hi)
            ),
            span,
        ));
    }
    if lo > hi {
        return Err(min_above_max("floats", &a[0], &a[1], span));
    }
    Ok(make("floats", vec![field("min", Value::Float(lo)), field("max", Value::Float(hi))]))
}

fn length_arg(a: &[Value], i: usize, default: i64, f: &str, span: Span) -> Fallible<i64> {
    if a.len() <= i {
        return Ok(default);
    }
    let n = int(a, i, f, span)?;
    if !(0..=MAX_LENGTH).contains(&n) {
        return Err(invalid(
            t!("`{f}` needs a length from 0 to {MAX_LENGTH}, not {n}", "`{f}` a besoin d'une longueur de 0 à {MAX_LENGTH}, pas {n}"),
            span,
        ));
    }
    Ok(n)
}

pub(crate) fn texts(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = length_arg(a, 0, 20, "texts", span)?;
    Ok(make("texts", vec![field("max_length", Value::Int(n))]))
}

pub(crate) fn bools(_: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
    Ok(make("bools", Vec::new()))
}

pub(crate) fn lists_of(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    generator(a, 0, "lists_of", span)?;
    let n = length_arg(a, 1, 10, "lists_of", span)?;
    Ok(make("lists_of", vec![field("of", a[0].clone()), field("max_length", Value::Int(n))]))
}

pub(crate) fn one_of(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if list(a, 0, "one_of", span)?.is_empty() {
        return Err(invalid(
            t!("`one_of` needs at least one value to choose from", "`one_of` a besoin d'au moins une valeur où choisir"),
            span,
        ));
    }
    Ok(make("one_of", vec![field("values", a[0].clone())]))
}

pub(crate) fn records_of(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = record(a, 0, "records_of", span)?;
    for (k, g) in r.iter() {
        if !matches!(g, Value::Record(_)) {
            return Err(invalid(
                t!(
                    "the field `{k}` given to `records_of` must be a generator, like `ints()`, not {}",
                    "le champ `{k}` donné à `records_of` doit être un générateur, comme `ints()`, pas {}",
                    g.describe()
                ),
                span,
            ));
        }
        read(g, 1, span)?;
    }
    Ok(make("records_of", vec![field("fields", a[0].clone())]))
}

// ----- making values -----------------------------------------------------

/// A value of `g`. `size` grows from 0 to 1 over the cases: the first values
/// are small, the last ones may use the whole range.
fn generate(g: &Gen, rng: &mut Rng, size: f64, budget: &mut usize) -> Value {
    match g {
        Gen::Int(lo, hi) => Value::Int(gen_int(*lo, *hi, rng, size)),
        Gen::Big(lo, hi) => bigint::value(gen_big(lo, hi, rng, size)),
        Gen::Float(lo, hi) => Value::Float(gen_float(*lo, *hi, rng, size)),
        Gen::Bool => Value::Bool(rng.chance(2)),
        Gen::Text(max) => {
            let n = gen_length(*max, rng, size, budget);
            let s: String = (0..n).map(|_| gen_char(rng)).collect();
            text_value(s)
        }
        Gen::List(of, max) => {
            let n = gen_length(*max, rng, size, budget);
            Value::list((0..n).map(|_| generate(of, rng, size, budget)).collect())
        }
        Gen::OneOf(values) => values[rng.below(values.len() as u64) as usize].clone(),
        Gen::Record(fields) => {
            let mut r = Record::new();
            for (k, g) in fields {
                r.set(k.clone(), generate(g, rng, size, budget));
            }
            Value::Record(Rc::new(r))
        }
    }
}

/// The simplest value of a range: 0 if it is inside, else the end closest to 0.
fn target_int(lo: i64, hi: i64) -> i64 {
    0.clamp(lo, hi)
}

fn target_big(lo: &BigInt, hi: &BigInt) -> BigInt {
    BigInt::default().clamp(lo.clone(), hi.clone())
}

/// A whole number from 0 to 2^bits - 1.
fn random_bits(rng: &mut Rng, bits: u64) -> BigInt {
    let mut digits: Vec<u32> = (0..bits.div_ceil(32)).map(|_| rng.next() as u32).collect();
    if let (Some(top), r @ 1..) = (digits.last_mut(), bits % 32) {
        *top &= (1 << r) - 1;
    }
    BigInt::from_digits(false, digits)
}

fn gen_big(lo: &BigInt, hi: &BigInt, rng: &mut Rng, size: f64) -> BigInt {
    let t = target_big(lo, hi);
    let one = BigInt::from_i64(1);
    if rng.chance(6) {
        let special = [lo.clone(), hi.clone(), t.clone(), t.add(&one).min(hi.clone()), t.sub(&one).max(lo.clone())];
        return special[rng.below(special.len() as u64) as usize].clone();
    }
    // Small numbers often; the others with more and more digits over the cases.
    let bits = if rng.chance(3) { 4 } else { ((hi.sub(lo).bits() as f64 * size) as u64).max(4) };
    let r = random_bits(rng, bits);
    let (up, down) = (t.add(&r), t.sub(&r));
    let x = if rng.chance(2) && up <= *hi || down < *lo { up } else { down };
    x.clamp(lo.clone(), hi.clone())
}

fn target_float(lo: f64, hi: f64) -> f64 {
    0.0f64.clamp(lo, hi)
}

fn gen_int(lo: i64, hi: i64, rng: &mut Rng, size: f64) -> i64 {
    let t = target_int(lo, hi);
    // The edges, where mistakes hide.
    if rng.chance(6) {
        let special = [lo, hi, t, t.saturating_add(1).min(hi), t.saturating_sub(1).max(lo)];
        return special[rng.below(special.len() as u64) as usize];
    }
    let (lo, hi, t) = (i128::from(lo), i128::from(hi), i128::from(t));
    // Small numbers often; the others in a range that grows with the cases.
    let spread = if rng.chance(3) { 10 } else { (((hi - lo) as f64 * size * size) as i128).max(10) };
    rng.between((t - spread).max(lo), (t + spread).min(hi)) as i64
}

fn gen_float(lo: f64, hi: f64, rng: &mut Rng, size: f64) -> f64 {
    let t = target_float(lo, hi);
    if rng.chance(6) {
        let special = [lo, hi, t, (t + 0.5).min(hi), (t - 0.5).max(lo)];
        return special[rng.below(special.len() as u64) as usize];
    }
    let spread = if rng.chance(3) { 10.0 } else { ((hi - lo) * size * size).max(10.0) };
    let (from, to) = ((t - spread).max(lo), (t + spread).min(hi));
    let x = from + (to - from) * rng.unit();
    // Whole numbers sometimes, and few decimals often: they are easier to read.
    let x = match rng.below(3) {
        0 => x.round(),
        1 => (x * 100.0).round() / 100.0,
        _ => x,
    };
    x.clamp(lo, hi)
}

fn gen_length(max: usize, rng: &mut Rng, size: f64, budget: &mut usize) -> usize {
    let most = ((max as f64 * size).ceil() as usize).clamp(max.min(3), max).min(*budget);
    let n = rng.below(most as u64 + 1) as usize;
    *budget -= n;
    n
}

fn gen_char(rng: &mut Rng) -> char {
    let pool = match rng.below(20) {
        0..=11 => LOWER,
        12 | 13 => UPPER,
        14 | 15 => DIGITS,
        16 => " ",
        17 | 18 => PUNCTUATION,
        _ => OTHERS,
    };
    let n = pool.chars().count() as u64;
    pool.chars().nth(rng.below(n) as usize).unwrap_or('a')
}

// ----- simplifying a failing value -----------------------------------------

/// Simpler values than `v`, the simplest first.
fn shrink(g: &Gen, v: &Value) -> Vec<Value> {
    match (g, v) {
        (Gen::Int(lo, hi), Value::Int(x)) => shrink_int(*x, target_int(*lo, *hi)).into_iter().map(Value::Int).collect(),
        (Gen::Big(lo, hi), Value::Int(_) | Value::Big(_)) => match bigint::big(v) {
            Some(x) => shrink_big(&x, &target_big(lo, hi)).into_iter().map(bigint::value).collect(),
            None => Vec::new(),
        },
        (Gen::Float(lo, hi), Value::Float(x)) => shrink_float(*x, target_float(*lo, *hi), *lo, *hi).into_iter().map(Value::Float).collect(),
        (Gen::Bool, Value::Bool(true)) => vec![Value::Bool(false)],
        (Gen::Text(_), Value::Text(s)) => {
            let chars: Vec<char> = s.chars().collect();
            let mut out: Vec<Value> = shorter(&chars).into_iter().map(|c| text_value(c.into_iter().collect::<String>())).collect();
            // Then each character, replaced by `a`.
            for (i, c) in chars.iter().enumerate().take(32) {
                if *c != 'a' {
                    let mut simpler = chars.clone();
                    simpler[i] = 'a';
                    out.push(text_value(simpler.into_iter().collect::<String>()));
                }
            }
            out
        }
        (Gen::List(of, _), Value::List(l)) => {
            let items: Vec<Value> = l.iter().cloned().collect();
            let mut out: Vec<Value> = shorter(&items).into_iter().map(Value::list).collect();
            // Then each element, simplified.
            for (i, x) in items.iter().enumerate().take(32) {
                for simpler in shrink(of, x).into_iter().take(8) {
                    let mut copy = items.clone();
                    copy[i] = simpler;
                    out.push(Value::list(copy));
                }
            }
            out
        }
        (Gen::OneOf(values), v) => {
            let at = values.iter().position(|x| x.equals(v)).unwrap_or(0);
            values[..at].to_vec()
        }
        (Gen::Record(fields), Value::Record(r)) => {
            let mut out = Vec::new();
            for (k, g) in fields {
                let Some(x) = r.get(k) else { continue };
                for simpler in shrink(g, x).into_iter().take(16) {
                    let mut copy = (**r).clone();
                    copy.set(k.clone(), simpler);
                    out.push(Value::Record(Rc::new(copy)));
                }
            }
            out
        }
        _ => Vec::new(),
    }
}

/// The target, then values ever closer to `x`.
fn shrink_int(x: i64, t: i64) -> Vec<i64> {
    if x == t {
        return Vec::new();
    }
    let (x, t) = (i128::from(x), i128::from(t));
    let mut out = vec![t as i64];
    let mut gap = (x - t) / 2;
    while gap != 0 {
        out.push((x - gap) as i64);
        gap /= 2;
    }
    out.dedup();
    out
}

/// The same for numbers of any size; at most 128 of them, as the halves of
/// a number of 100000 digits are many.
fn shrink_big(x: &BigInt, t: &BigInt) -> Vec<BigInt> {
    if x == t {
        return Vec::new();
    }
    let two = BigInt::from_i64(2);
    let d = x.sub(t);
    let mut out = vec![t.clone()];
    let mut gap = d.abs().div_mod_floor(&two).0;
    while !gap.is_zero() && out.len() < 128 {
        out.push(if d.is_negative() { x.add(&gap) } else { x.sub(&gap) });
        gap = gap.div_mod_floor(&two).0;
    }
    out
}

fn shrink_float(x: f64, t: f64, lo: f64, hi: f64) -> Vec<f64> {
    if x == t {
        return Vec::new();
    }
    let mut out = vec![t];
    let candidates = [x.trunc(), (x * 10.0).trunc() / 10.0, (x * 100.0).trunc() / 100.0, t + (x - t) / 2.0];
    for c in candidates {
        let closer = (c - t).abs() < (x - t).abs();
        if c >= lo && c <= hi && closer && (c - t).abs() > 1e-6 && !out.contains(&c) {
            out.push(c);
        }
    }
    out
}

/// Shorter sequences: empty, each half, then without one of the elements.
fn shorter<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let n = items.len();
    if n == 0 {
        return Vec::new();
    }
    let mut out = vec![Vec::new()];
    if n >= 2 {
        out.push(items[..n / 2].to_vec());
        out.push(items[n / 2..].to_vec());
    }
    if n >= 4 {
        out.push(items[..n - n / 4].to_vec());
    }
    for i in 0..n.min(32) {
        let mut copy = items.to_vec();
        copy.remove(i);
        if n - 1 > 0 {
            out.push(copy);
        }
    }
    out
}

// ----- check_all and sample ------------------------------------------------

/// How the property answered for one value.
enum Verdict {
    Pass,
    /// It returned this value: `false`, `None` or `Err(...)`.
    Returned(Value),
    /// It stopped with this error.
    Stopped(Box<Diagnostic>),
}

fn verdict(it: &mut Interp, f: &Value, x: &Value, span: Span) -> Fallible<Verdict> {
    match it.call1(f, x.clone(), span) {
        Ok(Value::Bool(true) | Value::Unit) => Ok(Verdict::Pass),
        Ok(v @ Value::Bool(false)) => Ok(Verdict::Returned(v)),
        Ok(v) => match it.wrap(&v) {
            Wrap::Some | Wrap::Ok => Ok(Verdict::Pass),
            Wrap::None | Wrap::Err => Ok(Verdict::Returned(v)),
            Wrap::Other => Err(wrong_return("check_all", BOOL, &v.describe(), span)),
        },
        // Out of the calls allowed for this run, or too deep: nothing more can
        // be tried (simplifying would go as deep again, at every `check_all`
        // around this one).
        Err(d) if it.out_of_steps() || d.code == "E014" => Err(d),
        Err(d) => Ok(Verdict::Stopped(d)),
    }
}

/// A value as shown in a message: whole, unless it is very long.
fn shown(v: &Value) -> String {
    let r = v.repr();
    if r.chars().count() > 200 {
        r.chars().take(197).collect::<String>() + "..."
    } else {
        r
    }
}

pub(crate) fn check_all(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    // `check_all(ints()) do |x| ... end`: a block is the last argument, so
    // the property may also come after the generator (and the number of cases).
    let (fi, gi, ci) = if !a[0].is_callable() && a.last().is_some_and(Value::is_callable) { (a.len() - 1, 0, 1) } else { (0, 1, 2) };
    let f = func(a, fi, "check_all", span)?;
    let g = generator(a, gi, "check_all", span)?;
    let cases = if a.len() > 2 { int(a, ci, "check_all", span)? } else { CASES };
    if !(1..=MAX_CASES).contains(&cases) {
        return Err(invalid(
            t!("`check_all` tries from 1 to {MAX_CASES} values, not {cases}", "`check_all` essaie de 1 à {MAX_CASES} valeurs, pas {cases}"),
            span,
        ));
    }
    given_arity("check_all", &f, 1, span)?;
    let seed = it.property_seed;
    let mut rng = Rng::new(seed, span);
    for case in 0..cases {
        let size = (case + 1) as f64 / cases as f64;
        let x = generate(&g, &mut rng, size, &mut BUDGET.clone());
        let first = verdict(it, &f, &x, span)?;
        if matches!(first, Verdict::Pass) {
            continue;
        }
        // Simplify: take the first simpler value that still fails, again and again.
        let (mut value, mut why, mut steps, mut calls) = (x.clone(), first, 0, 0);
        'simpler: while calls < MAX_SHRINK_CALLS {
            for candidate in shrink(&g, &value) {
                calls += 1;
                match verdict(it, &f, &candidate, span)? {
                    Verdict::Pass => {}
                    failed => {
                        (value, why, steps) = (candidate, failed, steps + 1);
                        continue 'simpler;
                    }
                }
                if calls >= MAX_SHRINK_CALLS {
                    break;
                }
            }
            break;
        }
        return Err(failure(why, Found { value: &value, first: &x, case: case + 1, cases, steps, seed }, span));
    }
    Ok(Value::Unit)
}

/// Which value failed, and how it was found.
struct Found<'a> {
    value: &'a Value,
    first: &'a Value,
    case: i64,
    cases: i64,
    steps: usize,
    seed: u64,
}

fn failure(why: Verdict, found: Found, span: Span) -> Box<Diagnostic> {
    let Found { value, first, case, cases, steps, seed } = found;
    let v = shown(value);
    let mut d = match why {
        Verdict::Returned(Value::Bool(false)) => {
            Diagnostic::error("E015", t!("the property is false for {v}", "la propriété est fausse pour {v}"))
                .at(span)
                .label(t!("false for {v}", "fausse pour {v}"))
        }
        Verdict::Returned(r) => {
            let r = shown(&r);
            Diagnostic::error("E015", t!("the property gives {r} for {v}", "la propriété donne {r} pour {v}"))
                .at(span)
                .label(t!("{r} for {v}", "{r} pour {v}"))
        }
        Verdict::Stopped(inner) => {
            let mut d = *inner;
            d.message = t!("the property fails for {v}: {}", "la propriété échoue pour {v} : {}", d.message);
            let inside = |s: Span| s.file == span.file && s.start >= span.start && s.end <= span.end;
            if d.span.is_some_and(|s| s.file == span.file && !inside(s)) {
                d = d.secondary(span, t!("checked here", "vérifiée ici"));
            } else if d.span.is_none() {
                d = d.at(span);
            }
            d
        }
        Verdict::Pass => Diagnostic::error("E015", String::new()).at(span),
    };
    let found = if steps == 0 {
        t!("found on try {case} of {cases}", "trouvé à l'essai {case} sur {cases}")
    } else {
        t!(
            "found on try {case} of {cases}, then simplified from {} in {steps} step(s)",
            "trouvé à l'essai {case} sur {cases}, puis simplifié depuis {} en {steps} étape(s)",
            shown(first)
        )
    };
    d.notes.insert(0, found);
    d.notes.push(t!(
        "seed {seed}: every run tries the same values; `rythme test --seed N` tries others",
        "graine {seed} : chaque lancement essaie les mêmes valeurs ; `rythme test --seed N` en essaie d'autres"
    ));
    if d.code == "E015" && d.helps.is_empty() {
        d = d.help(t!(
            "the function must hold for every value of the generator: fix the function, or give the generator a smaller range, like `ints(1, 100)`",
            "la fonction doit être vraie pour chaque valeur du générateur : corrigez la fonction, ou donnez au générateur un intervalle plus petit, comme `ints(1, 100)`"
        ));
    }
    d.boxed()
}

pub(crate) fn sample(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let g = generator(a, 0, "sample", span)?;
    let n = length_arg(a, 1, 10, "sample", span)?;
    let mut rng = Rng::new(it.property_seed, span);
    let n = n as usize;
    Ok(Value::list((0..n).map(|i| generate(&g, &mut rng, (i + 1) as f64 / n as f64, &mut BUDGET.clone())).collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers_shrink_towards_the_target() {
        assert_eq!(shrink_int(100, 0), [0, 50, 75, 88, 94, 97, 99]);
        assert_eq!(shrink_int(-7, 0), [0, -4, -6]);
        assert_eq!(shrink_int(12, 5), [5, 9, 11]);
        assert!(shrink_int(3, 3).is_empty());
        assert_eq!(shrink_int(i64::MIN, 0)[0], 0);
    }

    #[test]
    fn floats_shrink_and_stop() {
        assert_eq!(shrink_float(3.75, 0.0, -10.0, 10.0)[..2], [0.0, 3.0]);
        assert!(shrink_float(0.0, 0.0, -1.0, 1.0).is_empty());
        let mut x = 7.123_456;
        for _ in 0..100 {
            match shrink_float(x, 0.0, -10.0, 10.0).last() {
                Some(&y) if y != 0.0 => x = y,
                _ => break,
            }
        }
        assert!(x.abs() < 1e-3);
    }

    #[test]
    fn the_same_seed_gives_the_same_values() {
        let g = Gen::List(Box::new(Gen::Int(-5, 5)), 6);
        let values = |seed| {
            let mut rng = Rng::new(seed, Span::default());
            (0..20).map(|i| generate(&g, &mut rng, f64::from(i) / 20.0, &mut BUDGET.clone()).repr()).collect::<Vec<_>>()
        };
        assert_eq!(values(1), values(1));
        assert_ne!(values(1), values(2));
    }

    #[test]
    fn generated_values_stay_in_range() {
        let mut rng = Rng::new(7, Span::default());
        for i in 0..2000 {
            let size = f64::from(i % 100) / 100.0;
            let n = gen_int(-3, 9, &mut rng, size);
            assert!((-3..=9).contains(&n));
            let n = gen_int(i64::MIN, i64::MAX, &mut rng, size);
            let _ = n;
            let x = gen_float(0.5, 2.5, &mut rng, size);
            assert!((0.5..=2.5).contains(&x));
            let mut budget = 5;
            assert!(gen_length(1000, &mut rng, size, &mut budget) <= 5);
        }
    }
}
