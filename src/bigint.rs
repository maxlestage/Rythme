//! Whole numbers of any size.
//!
//! Most whole numbers fit in 64 bits: they stay `Value::Int(i64)`, which the
//! interpreter computes on the spot. Only a result outside of that range
//! becomes a `Value::Big`, so the two never hold the same number and the
//! usual cases never come here. A number is a sign and its digits in base
//! 2^32, the lowest first.
//!
//! Numbers are kept under 100000 decimal digits (`MAX_DIGITS`): beyond, a
//! calculation stops with E023 instead of filling the memory.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use crate::ast::BinOp;
use crate::diagnostic::{Diagnostic, Fallible};
use crate::interp::{div_zero, zero_to_negative_power};
use crate::source::Span;
use crate::value::Value;

/// A whole number may have at most this many decimal digits.
pub const MAX_DIGITS: usize = 100_000;

/// The number of bits of 10^MAX_DIGITS: smaller numbers surely fit, larger
/// ones surely do not.
const LIMIT_BITS: u64 = 332_193;

/// Under this many digits (in base 2^32), multiplying digit by digit is the fastest.
const KARATSUBA: usize = 40;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BigInt {
    neg: bool,
    /// The digits in base 2^32, the lowest first, without zeros at the end
    /// (zero has none, and is never negative).
    mag: Vec<u32>,
}

// ----- Magnitudes (digits in base 2^32, the lowest first) -----------------

fn trim(v: &mut Vec<u32>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}

fn cmp_mag(a: &[u32], b: &[u32]) -> Ordering {
    a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

fn add_mag(a: &[u32], b: &[u32]) -> Vec<u32> {
    let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = Vec::with_capacity(a.len() + 1);
    let mut carry = 0u64;
    for (i, &x) in a.iter().enumerate() {
        let s = x as u64 + b.get(i).copied().unwrap_or(0) as u64 + carry;
        out.push(s as u32);
        carry = s >> 32;
    }
    if carry != 0 {
        out.push(carry as u32);
    }
    trim(&mut out);
    out
}

/// `a - b`, for `a >= b`.
fn sub_mag(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = 0u64;
    for (i, &x) in a.iter().enumerate() {
        let (d, o1) = x.overflowing_sub(b.get(i).copied().unwrap_or(0));
        let (d, o2) = d.overflowing_sub(borrow as u32);
        out.push(d);
        borrow = (o1 || o2) as u64;
    }
    debug_assert_eq!(borrow, 0);
    trim(&mut out);
    out
}

/// Adds `x` into `acc` (which is long enough for the sum).
fn add_into(acc: &mut [u32], x: &[u32]) {
    let mut carry = 0u64;
    let mut i = 0;
    while i < x.len() || carry != 0 {
        let s = acc[i] as u64 + x.get(i).copied().unwrap_or(0) as u64 + carry;
        acc[i] = s as u32;
        carry = s >> 32;
        i += 1;
    }
}

fn mul_mag(a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = if a.len().min(b.len()) < KARATSUBA { mul_school(a, b) } else { mul_karatsuba(a, b) };
    trim(&mut out);
    out
}

fn mul_school(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = vec![0u32; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        let mut carry = 0u64;
        for (j, &y) in b.iter().enumerate() {
            let t = x as u64 * y as u64 + out[i + j] as u64 + carry;
            out[i + j] = t as u32;
            carry = t >> 32;
        }
        out[i + b.len()] = carry as u32;
    }
    out
}

/// Karatsuba's method: three products of halves instead of four.
fn mul_karatsuba(a: &[u32], b: &[u32]) -> Vec<u32> {
    let m = a.len().max(b.len()) / 2;
    if a.len() <= m || b.len() <= m {
        // Very different lengths: the longer one in pieces as long as the other.
        let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        let mut out = vec![0u32; long.len() + short.len()];
        for (k, piece) in long.chunks(short.len()).enumerate() {
            add_into(&mut out[k * short.len()..], &mul_mag(piece, short));
        }
        return out;
    }
    let (a0, a1) = a.split_at(m);
    let (b0, b1) = b.split_at(m);
    let z0 = mul_mag(a0, b0);
    let z2 = mul_mag(a1, b1);
    let z1 = mul_mag(&add_mag(a0, a1), &add_mag(b0, b1));
    let z1 = sub_mag(&sub_mag(&z1, &z0), &z2);
    let mut out = vec![0u32; a.len() + b.len() + 1];
    add_into(&mut out, &z0);
    add_into(&mut out[m..], &z1);
    add_into(&mut out[2 * m..], &z2);
    out
}

/// Divides `a` in place by a single digit, and gives the remainder.
fn div_small_in_place(a: &mut Vec<u32>, d: u32) -> u32 {
    let mut r = 0u64;
    for x in a.iter_mut().rev() {
        let cur = (r << 32) | *x as u64;
        *x = (cur / d as u64) as u32;
        r = cur % d as u64;
    }
    trim(a);
    r as u32
}

/// `a * m + add`, in place.
fn mul_small_add(a: &mut Vec<u32>, m: u32, add: u32) {
    let mut carry = add as u64;
    for x in a.iter_mut() {
        let t = *x as u64 * m as u64 + carry;
        *x = t as u32;
        carry = t >> 32;
    }
    if carry != 0 {
        a.push(carry as u32);
    }
}

fn shl_mag(a: &[u32], bits: u64) -> Vec<u32> {
    if a.is_empty() {
        return Vec::new();
    }
    let (limbs, r) = ((bits / 32) as usize, (bits % 32) as u32);
    let mut out = vec![0u32; limbs];
    if r == 0 {
        out.extend_from_slice(a);
    } else {
        let mut carry = 0u32;
        for &x in a {
            out.push((x << r) | carry);
            carry = x >> (32 - r);
        }
        out.push(carry);
    }
    trim(&mut out);
    out
}

fn shr_mag(a: &[u32], bits: u64) -> Vec<u32> {
    let limbs = (bits / 32) as usize;
    if limbs >= a.len() {
        return Vec::new();
    }
    let r = (bits % 32) as u32;
    let a = &a[limbs..];
    let mut out: Vec<u32> =
        if r == 0 { a.to_vec() } else { (0..a.len()).map(|i| (a[i] >> r) | a.get(i + 1).map_or(0, |&hi| hi << (32 - r))).collect() };
    trim(&mut out);
    out
}

fn bits_mag(a: &[u32]) -> u64 {
    match a.last() {
        None => 0,
        Some(top) => (a.len() as u64 - 1) * 32 + (32 - top.leading_zeros()) as u64,
    }
}

/// Quotient and remainder of `a / b` (Knuth's algorithm D), `b` not zero.
fn divrem_mag(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<u32>) {
    debug_assert!(!b.is_empty());
    if cmp_mag(a, b) == Ordering::Less {
        return (Vec::new(), a.to_vec());
    }
    if b.len() == 1 {
        let mut q = a.to_vec();
        let r = div_small_in_place(&mut q, b[0]);
        return (q, if r == 0 { Vec::new() } else { vec![r] });
    }
    // Shift both so that the top digit of the divisor is at least 2^31.
    let s = b.last().unwrap().leading_zeros() as u64;
    let v = shl_mag(b, s);
    let mut u = shl_mag(a, s);
    u.resize(a.len() + 1, 0);
    let n = v.len();
    let m = a.len() - n;
    let mut q = vec![0u32; m + 1];
    let (vtop, vnext) = (v[n - 1] as u64, v[n - 2] as u64);
    const B: u64 = 1 << 32;
    for j in (0..=m).rev() {
        let num = ((u[j + n] as u64) << 32) | u[j + n - 1] as u64;
        let mut qhat = num / vtop;
        let mut rhat = num % vtop;
        while qhat >= B || qhat * vnext > ((rhat << 32) | u[j + n - 2] as u64) {
            qhat -= 1;
            rhat += vtop;
            if rhat >= B {
                break;
            }
        }
        // u[j..j+n+1] -= qhat * v
        let mut borrow = 0i64;
        let mut carry = 0u64;
        for i in 0..n {
            let p = qhat * v[i] as u64 + carry;
            carry = p >> 32;
            let t = u[i + j] as i64 - borrow - (p & 0xffff_ffff) as i64;
            u[i + j] = t as u32;
            borrow = (t < 0) as i64;
        }
        let t = u[j + n] as i64 - borrow - carry as i64;
        u[j + n] = t as u32;
        if t < 0 {
            // One too many: add the divisor back.
            qhat -= 1;
            let mut c = 0u64;
            for i in 0..n {
                let s = u[i + j] as u64 + v[i] as u64 + c;
                u[i + j] = s as u32;
                c = s >> 32;
            }
            u[j + n] = u[j + n].wrapping_add(c as u32);
        }
        q[j] = qhat as u32;
    }
    trim(&mut q);
    u.truncate(n);
    trim(&mut u);
    (q, shr_mag(&u, s))
}

fn from_u64(n: u64) -> Vec<u32> {
    let mut v = vec![n as u32, (n >> 32) as u32];
    trim(&mut v);
    v
}

/// 10^MAX_DIGITS, computed once.
fn limit() -> Rc<Vec<u32>> {
    thread_local! {
        static LIMIT: Rc<Vec<u32>> = Rc::new(pow_mag(&[10], MAX_DIGITS as u64));
    }
    LIMIT.with(|l| l.clone())
}

fn pow_mag(base: &[u32], mut exp: u64) -> Vec<u32> {
    let mut result = vec![1u32];
    let mut b = base.to_vec();
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mag(&result, &b);
        }
        exp >>= 1;
        if exp > 0 {
            b = mul_mag(&b, &b);
        }
    }
    result
}

// ----- Signed numbers ------------------------------------------------------

impl BigInt {
    fn new(neg: bool, mut mag: Vec<u32>) -> BigInt {
        trim(&mut mag);
        BigInt { neg: neg && !mag.is_empty(), mag }
    }

    pub fn from_i64(n: i64) -> BigInt {
        BigInt::new(n < 0, from_u64(n.unsigned_abs()))
    }

    pub fn from_i128(n: i128) -> BigInt {
        let u = n.unsigned_abs();
        BigInt::new(n < 0, vec![u as u32, (u >> 32) as u32, (u >> 64) as u32, (u >> 96) as u32])
    }

    /// The number, if it fits in 64 bits.
    pub fn to_i64(&self) -> Option<i64> {
        if self.mag.len() > 2 {
            return None;
        }
        let u = self.mag.first().copied().unwrap_or(0) as u64 | (self.mag.get(1).copied().unwrap_or(0) as u64) << 32;
        if self.neg {
            (u <= 1 << 63).then(|| 0i64.wrapping_sub(u as i64))
        } else {
            i64::try_from(u).ok()
        }
    }

    pub fn is_zero(&self) -> bool {
        self.mag.is_empty()
    }

    pub fn is_negative(&self) -> bool {
        self.neg
    }

    pub fn is_even(&self) -> bool {
        self.mag.first().is_none_or(|d| d % 2 == 0)
    }

    /// How many bits the absolute value has.
    pub fn bits(&self) -> u64 {
        bits_mag(&self.mag)
    }

    pub fn neg(&self) -> BigInt {
        BigInt::new(!self.neg, self.mag.clone())
    }

    pub fn abs(&self) -> BigInt {
        BigInt::new(false, self.mag.clone())
    }

    pub fn add(&self, other: &BigInt) -> BigInt {
        if self.neg == other.neg {
            return BigInt::new(self.neg, add_mag(&self.mag, &other.mag));
        }
        match cmp_mag(&self.mag, &other.mag) {
            Ordering::Equal => BigInt::default(),
            Ordering::Greater => BigInt::new(self.neg, sub_mag(&self.mag, &other.mag)),
            Ordering::Less => BigInt::new(other.neg, sub_mag(&other.mag, &self.mag)),
        }
    }

    pub fn sub(&self, other: &BigInt) -> BigInt {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &BigInt) -> BigInt {
        BigInt::new(self.neg != other.neg, mul_mag(&self.mag, &other.mag))
    }

    /// The quotient rounded down, and the remainder, which has the sign of
    /// the divisor (like `//` and `%`). The divisor is not zero.
    pub fn div_mod_floor(&self, other: &BigInt) -> (BigInt, BigInt) {
        let (q, r) = divrem_mag(&self.mag, &other.mag);
        let (q, r) = (BigInt::new(self.neg != other.neg, q), BigInt::new(self.neg, r));
        if !r.is_zero() && self.neg != other.neg {
            (q.sub(&BigInt::from_i64(1)), r.add(other))
        } else {
            (q, r)
        }
    }

    pub fn pow(&self, exp: u64) -> BigInt {
        BigInt::new(self.neg && exp % 2 == 1, pow_mag(&self.mag, exp))
    }

    pub fn gcd(&self, other: &BigInt) -> BigInt {
        let (mut a, mut b) = (self.mag.clone(), other.mag.clone());
        while !b.is_empty() {
            let (_, r) = divrem_mag(&a, &b);
            a = std::mem::replace(&mut b, r);
        }
        BigInt::new(false, a)
    }

    /// Whether the number has more than `MAX_DIGITS` digits.
    pub fn too_large(&self) -> bool {
        match self.bits() {
            b if b < LIMIT_BITS => false,
            b if b > LIMIT_BITS => true,
            _ => cmp_mag(&self.mag, &limit()) != Ordering::Less,
        }
    }

    /// The nearest decimal number (halves to even), or an infinity.
    pub fn to_f64(&self) -> f64 {
        let bits = self.bits();
        let x = if bits <= 64 {
            (self.mag.first().copied().unwrap_or(0) as u64 | (self.mag.get(1).copied().unwrap_or(0) as u64) << 32) as f64
        } else {
            // The 64 highest bits, the lowest of them set if any bit below is:
            // converting them rounds exactly like the whole number would.
            let shift = bits - 64;
            let top = shr_mag(&self.mag, shift);
            let mut t = top[0] as u64 | (top[1] as u64) << 32;
            let lost = shl_mag(&top, shift) != self.mag;
            t |= lost as u64;
            scale(t as f64, shift as i64)
        };
        if self.neg {
            -x
        } else {
            x
        }
    }

    /// The exact value of a whole decimal number (finite and without decimals).
    pub fn from_whole_f64(f: f64) -> BigInt {
        debug_assert!(f.is_finite() && f.fract() == 0.0);
        let bits = f.to_bits();
        let exp = ((bits >> 52) & 0x7ff) as i64;
        if exp == 0 {
            return BigInt::default(); // zero (whole subnormal numbers do not exist)
        }
        let mantissa = (bits & ((1 << 52) - 1)) | (1 << 52);
        let shift = exp - 1075;
        let mag = if shift >= 0 { shl_mag(&from_u64(mantissa), shift as u64) } else { from_u64(mantissa >> -shift) };
        BigInt::new(f < 0.0, mag)
    }

    /// The exact order between the number and a decimal one, `None` for `nan`.
    pub fn cmp_f64(&self, f: f64) -> Option<Ordering> {
        if f.is_nan() {
            return None;
        }
        if f.is_infinite() {
            return Some(if f > 0.0 { Ordering::Less } else { Ordering::Greater });
        }
        let whole = f.trunc();
        Some(self.cmp(&BigInt::from_whole_f64(whole)).then_with(|| 0.0.partial_cmp(&(f - whole)).unwrap_or(Ordering::Equal)))
    }

    /// `self / other` as a decimal number, close to the exact quotient even
    /// when both are too large for decimal numbers. The divisor is not zero.
    pub fn div_f64(&self, other: &BigInt) -> f64 {
        // About 66 bits of quotient, then the rest tells if it is above.
        let k = 66 + other.bits() as i64 - self.bits() as i64;
        let (num, den) =
            if k >= 0 { (shl_mag(&self.mag, k as u64), other.mag.clone()) } else { (self.mag.clone(), shl_mag(&other.mag, (-k) as u64)) };
        let (q, r) = divrem_mag(&num, &den);
        let mut q = BigInt::new(false, q);
        if !r.is_empty() {
            if let Some(d) = q.mag.first_mut() {
                *d |= 1;
            }
        }
        let x = scale(q.to_f64(), -k);
        if self.neg != other.neg {
            -x
        } else {
            x
        }
    }

    /// Reads decimal digits (only `0` to `9`, at least one).
    fn from_decimal(digits: &str) -> BigInt {
        let mut mag = Vec::new();
        let first = digits.len() % 9;
        let mut chunks = Vec::new();
        if first > 0 {
            chunks.push(&digits[..first]);
        }
        chunks.extend(digits.as_bytes()[first..].chunks(9).map(|c| std::str::from_utf8(c).unwrap()));
        for c in chunks {
            mul_small_add(&mut mag, 10u32.pow(c.len() as u32), c.parse().unwrap());
        }
        BigInt::new(false, mag)
    }

    /// Reads a whole number written in base 10 (`-123`, `+5`, `1_000`), or
    /// in base 16 or 2 (`0xff`, `0b101`, without sign) when `radix` says so.
    /// `None` if it is not a number; `Some(Err(()))` if it has more than
    /// `MAX_DIGITS` digits.
    pub fn parse(text: &str, radix: u32) -> Option<Result<BigInt, ()>> {
        let (neg, digits) = match text.as_bytes().first() {
            Some(b'-') if radix == 10 => (true, &text[1..]),
            Some(b'+') if radix == 10 => (false, &text[1..]),
            _ => (false, text),
        };
        if digits.is_empty() || !digits.chars().all(|c| c.is_digit(radix)) {
            return None;
        }
        let digits = digits.trim_start_matches('0');
        let n = match radix {
            10 => {
                if digits.len() > MAX_DIGITS {
                    return Some(Err(()));
                }
                BigInt::from_decimal(digits)
            }
            _ => {
                let per = if radix == 16 { 4 } else { 1 };
                if digits.len() as u64 * per > LIMIT_BITS + 4 {
                    return Some(Err(()));
                }
                let mut mag = vec![0u32; (digits.len() * per as usize).div_ceil(32)];
                for (i, c) in digits.chars().rev().enumerate() {
                    let d = c.to_digit(radix).unwrap();
                    let at = i * per as usize;
                    mag[at / 32] |= d << (at % 32);
                }
                BigInt::new(false, mag)
            }
        };
        let n = BigInt::new(neg, n.mag);
        Some(if n.too_large() { Err(()) } else { Ok(n) })
    }
}

/// `x * 2^e`, rounded once at most.
fn scale(mut x: f64, mut e: i64) -> f64 {
    while e > 1000 {
        x *= 2f64.powi(1000);
        e -= 1000;
        if x.is_infinite() {
            return x;
        }
    }
    while e < -1000 {
        x *= 2f64.powi(-1000);
        e += 1000;
    }
    x * 2f64.powi(e as i32)
}

impl Ord for BigInt {
    fn cmp(&self, other: &BigInt) -> Ordering {
        match (self.neg, other.neg) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => cmp_mag(&self.mag, &other.mag),
            (true, true) => cmp_mag(&other.mag, &self.mag),
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mag.is_empty() {
            return f.write_str("0");
        }
        // Groups of 9 digits, the lowest first.
        let mut groups = Vec::with_capacity(self.mag.len() * 32 / 29 + 1);
        let mut rest = self.mag.clone();
        while !rest.is_empty() {
            groups.push(div_small_in_place(&mut rest, 1_000_000_000));
        }
        let mut s = String::with_capacity(groups.len() * 9 + 1);
        if self.neg {
            s.push('-');
        }
        let mut groups = groups.iter().rev();
        s.push_str(&groups.next().unwrap().to_string());
        for g in groups {
            s.push_str(&format!("{g:09}"));
        }
        f.write_str(&s)
    }
}

// ----- Values --------------------------------------------------------------

/// The value of a whole number: an `Int` when it fits in 64 bits.
pub fn value(n: BigInt) -> Value {
    match n.to_i64() {
        Some(i) => Value::Int(i),
        None => Value::Big(Rc::new(n)),
    }
}

/// `value`, unless the number is too large (E023).
pub fn checked(n: BigInt, span: Span) -> Fallible<Value> {
    if n.too_large() {
        return Err(too_large(span));
    }
    Ok(value(n))
}

/// A whole number (`Int` or `Big`) as a `BigInt`.
pub fn big(v: &Value) -> Option<Cow<'_, BigInt>> {
    match v {
        Value::Int(n) => Some(Cow::Owned(BigInt::from_i64(*n))),
        Value::Big(b) => Some(Cow::Borrowed(b)),
        _ => None,
    }
}

/// The value of a whole decimal number, exact (`floor(1e20)`).
pub fn from_whole_f64(f: f64) -> Value {
    if (-9.223_372_036_854_776e18..9.223_372_036_854_776e18).contains(&f) {
        Value::Int(f as i64)
    } else {
        value(BigInt::from_whole_f64(f))
    }
}

pub fn too_large(span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "E023",
        t!(
            "this whole number would be too large (more than {MAX_DIGITS} digits)",
            "ce nombre entier serait trop grand (plus de {MAX_DIGITS} chiffres)"
        ),
    )
    .at(span)
    .help(t!(
        "does it grow without end, in a function that calls itself? For an approximate value, use a decimal number (Float), like `2.0 ** 1000`",
        "grandit-il sans fin, dans une fonction qui s'appelle elle-même ? Pour une valeur approchée, utilisez un nombre décimal (Float), comme `2.0 ** 1000`"
    ))
    .boxed()
}

/// An arithmetic operator on two whole numbers, one of them large or the
/// result too large for 64 bits.
#[inline(never)]
pub fn binop(op: BinOp, l: &Value, r: &Value, span: Span) -> Fallible<Value> {
    let (a, b) = (big(l).unwrap(), big(r).unwrap());
    match op {
        BinOp::Add => checked(a.add(&b), span),
        BinOp::Sub => checked(a.sub(&b), span),
        BinOp::Mul => {
            if !a.is_zero() && !b.is_zero() && a.bits() + b.bits() - 1 > LIMIT_BITS {
                return Err(too_large(span));
            }
            checked(a.mul(&b), span)
        }
        BinOp::Div => {
            if b.is_zero() {
                return Err(div_zero(span));
            }
            Ok(Value::Float(a.div_f64(&b)))
        }
        BinOp::IntDiv | BinOp::Mod => {
            if b.is_zero() {
                return Err(div_zero(span));
            }
            let (q, m) = a.div_mod_floor(&b);
            Ok(value(if op == BinOp::IntDiv { q } else { m }))
        }
        BinOp::Pow => {
            if b.is_negative() {
                if a.is_zero() {
                    return Err(zero_to_negative_power(span));
                }
                return Ok(Value::Float(a.to_f64().powf(b.to_f64())));
            }
            // 0, 1 and -1 never grow, whatever the exponent.
            match a.to_i64() {
                Some(0) => return Ok(Value::Int(b.is_zero() as i64)),
                Some(1) => return Ok(Value::Int(1)),
                Some(-1) => return Ok(Value::Int(if b.is_even() { 1 } else { -1 })),
                _ => {}
            }
            // The result has at least (bits - 1) * exp + 1 bits.
            match b.to_i64() {
                Some(e) if (a.bits() - 1).saturating_mul(e as u64) < LIMIT_BITS => checked(a.pow(e as u64), span),
                _ => Err(too_large(span)),
            }
        }
        _ => unreachable!("not an arithmetic operator: {op:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(s: &str) -> BigInt {
        BigInt::parse(s, 10).unwrap().unwrap()
    }

    #[test]
    fn karatsuba_agrees_with_the_school_method() {
        let mut x = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u32
        };
        for (la, lb) in [(40, 40), (41, 97), (200, 45), (300, 301), (1000, 999), (513, 2000)] {
            let a: Vec<u32> = (0..la).map(|_| next()).collect();
            let b: Vec<u32> = (0..lb).map(|_| next()).collect();
            let mut school = mul_school(&a, &b);
            trim(&mut school);
            assert_eq!(mul_mag(&a, &b), school, "{la} x {lb}");
        }
    }

    #[test]
    fn division_gives_back_the_number() {
        let a = n("123456789012345678901234567890123456789012345678901234567890");
        for d in ["7", "4294967296", "18446744073709551617", "98765432109876543210987", "-3", "-123456789012345678901"] {
            let d = n(d);
            let (q, r) = a.div_mod_floor(&d);
            assert_eq!(q.mul(&d).add(&r), a);
            assert!(r.is_zero() || r.neg == d.neg);
            assert_eq!(r.abs().cmp(&d.abs()), Ordering::Less);
        }
    }

    #[test]
    fn decimal_digits_and_back() {
        for s in ["0", "1", "-1", "4294967296", "18446744073709551616", "-170141183460469231731687303715884105728"] {
            assert_eq!(n(s).to_string(), s);
        }
        assert_eq!(n("+000123").to_string(), "123");
        assert_eq!(BigInt::parse("ff", 16).unwrap().unwrap().to_string(), "255");
        assert_eq!(BigInt::parse("1".repeat(70).as_str(), 2).unwrap().unwrap().to_string(), "1180591620717411303423");
        assert!(BigInt::parse("12a", 10).is_none());
        assert!(BigInt::parse("-", 10).is_none());
    }

    #[test]
    fn the_size_limit_is_exact() {
        let limit = BigInt::new(false, (*super::limit()).clone());
        assert_eq!(limit.to_string().len(), MAX_DIGITS + 1);
        assert!(limit.too_large());
        assert!(!limit.sub(&BigInt::from_i64(1)).too_large());
        assert!(limit.neg().too_large());
    }

    #[test]
    fn conversions_to_decimal_numbers_round_to_nearest() {
        // 2^64 + 2^11 + 1 is just above the half between two decimal numbers.
        let x = n("18446744073709553665");
        assert_eq!(x.to_f64(), 18446744073709555712.0);
        let half = n("18446744073709553664");
        assert_eq!(half.to_f64(), 18446744073709551616.0);
        assert_eq!(n("1").pow(5000).to_f64(), 1.0);
        assert_eq!(n("10").pow(400).to_f64(), f64::INFINITY);
        assert_eq!(n("10").pow(400).div_f64(&n("10").pow(399)), 10.0);
        assert_eq!(BigInt::from_whole_f64(1e300).to_f64(), 1e300);
    }
}
