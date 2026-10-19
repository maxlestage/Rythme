//! Slopes, for teaching how a model learns: `derivative(f, x)` and
//! `gradient(f, xs)`, computed by central differences:
//!
//! ```text
//! f'(x) ≈ (f(x + h) - f(x - h)) / 2h,    h = 0.00001 × max(1, |x|)
//! ```
//!
//! The error of this formula is about h² times the third derivative of f,
//! plus the rounding of `f(x + h)` and `f(x - h)` divided by h. The same
//! formula with 2h estimates the first error; the result keeps only the
//! digits that both leave sure: `derivative(fn(x) = x * x, 3)` is `6.0`, not
//! `6.000000000012662`. For a smooth function of reasonable numbers that is
//! about 9 significant digits; a function with a corner (`abs` at 0) gets the
//! mean of the slopes on both sides.
//!
//! Exact derivatives (forward-mode automatic differentiation, with "dual
//! numbers") would need a hidden kind of number going through every
//! operation, comparison and `print` of the interpreter: what a value shows
//! would no longer be what it is. The numerical slope is visible, pure, and
//! is itself a lesson: the slope between two very close points.

use super::ml::{as_number, vector};
use super::*;

/// The number that `g` (given to `f`) gives for `x`.
fn number_from(it: &mut Interp, f: &str, g: &Value, x: Value, span: Span) -> Fallible<f64> {
    let y = call_given1(it, f, g, x, span)?;
    as_number(&y).ok_or_else(|| {
        let got = y.describe();
        Diagnostic::error(
            "E005",
            t!(
                "the function given to `{f}` must return a number, but it returned {got}",
                "la fonction donnée à `{f}` doit renvoyer un nombre, mais elle a renvoyé {got}"
            ),
        )
        .at(span)
        .help(usage(f))
        .boxed()
    })
}

/// The step of the central differences around `x`: small, but large enough
/// for the rounding of `f(x + h) - f(x - h)` to stay small.
fn step(x: f64) -> f64 {
    1e-5 * x.abs().max(1.0)
}

/// The slope at 0 of `g(dx)` (f moved by dx): central differences with the
/// steps h and 2h. Their difference estimates the error of the first one
/// (about h², four times more with 2h); with the rounding of the values of
/// f, it says which digits are sure, and only those are kept:
/// `derivative(fn(x) = x * x, 3)` is `6.0`, not `6.000000000012662`.
fn slope(h: f64, g: &mut dyn FnMut(f64) -> Fallible<f64>) -> Fallible<f64> {
    let (a, b) = (g(h)?, g(-h)?);
    let (c, d) = (g(2.0 * h)?, g(-2.0 * h)?);
    let near = (a - b) / (2.0 * h);
    let far = (c - d) / (4.0 * h);
    let biggest = a.abs().max(b.abs()).max(c.abs()).max(d.abs());
    let error = (near - far).abs() / 3.0 + 4.0 * f64::EPSILON * biggest / h;
    Ok(keep_sure_digits(near, error))
}

/// `x` rounded to the power of ten just above `error`.
fn keep_sure_digits(x: f64, error: f64) -> f64 {
    if !x.is_finite() || !error.is_normal() {
        return x;
    }
    let digits = -error.log10().ceil();
    if !(-300.0..=300.0).contains(&digits) {
        return x;
    }
    // Divided by an exact power of ten, a whole number gives the closest
    // Float to the decimal number: 6.0, not 6.000000000000001.
    let p = 10f64.powi(digits.abs() as i32);
    let rounded = if digits >= 0.0 { (x * p).round() / p } else { (x / p).round() * p };
    if rounded.is_finite() {
        rounded
    } else {
        x
    }
}

pub(super) fn derivative(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = func(a, 0, "derivative", span)?;
    let x = number(a, 1, "derivative", span)?;
    let mut g = |dx: f64| number_from(it, "derivative", &f, Value::Float(x + dx), span);
    Ok(Value::Float(slope(step(x), &mut g)?))
}

pub(super) fn gradient(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = func(a, 0, "gradient", span)?;
    let xs = vector(a, 1, "gradient", span)?;
    let mut out = Vec::with_capacity(xs.len());
    for i in 0..xs.len() {
        let mut g = |dx: f64| {
            let moved = Value::list(xs.iter().enumerate().map(|(j, &x)| Value::Float(if j == i { x + dx } else { x })).collect());
            number_from(it, "gradient", &f, moved, span)
        };
        out.push(Value::Float(slope(step(xs[i]), &mut g)?));
    }
    Ok(Value::list(out))
}
