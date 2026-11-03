//! Numbers: keeping a value between two bounds (`constrain`, Ruby's
//! `clamp`), the quotient and the remainder at once (`divmod`), binomial
//! coefficients of any size, and the questions asked of a number
//! (`is_positive`, `is_zero`, `is_nan`...).

use super::*;
use crate::ast::BinOp;

pub(super) fn constrain(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, low, high) = (&a[0], &a[1], &a[2]);
    let cmp = |p: &Value, q: &Value| -> Fallible<Ordering> {
        p.compare(q).ok_or_else(|| {
            Diagnostic::error(
                "E005",
                t!("`{f}` cannot compare {} and {}", "`{f}` ne peut pas comparer {} et {}", p.describe(), q.describe(), f = "constrain"),
            )
            .at(span)
            .help(t!(
                "the value and its bounds must be of the same kind: all numbers, or all texts...",
                "la valeur et ses bornes doivent être de même nature : tous des nombres, ou tous des textes..."
            ))
            .boxed()
        })
    };
    if cmp(low, high)? == Ordering::Greater {
        return Err(invalid(
            t!(
                "`constrain`: the lower bound {} is above the upper bound {}",
                "`constrain` : la borne basse {} est au-dessus de la borne haute {}",
                low.repr(),
                high.repr()
            ),
            span,
        ));
    }
    Ok(if cmp(x, low)? == Ordering::Less {
        low.clone()
    } else if cmp(x, high)? == Ordering::Greater {
        high.clone()
    } else {
        x.clone()
    })
}

pub(super) fn divmod(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    number(a, 0, "divmod", span)?;
    number(a, 1, "divmod", span)?;
    let q = binop(BinOp::IntDiv, &a[0], &a[1], span)?;
    let r = binop(BinOp::Mod, &a[0], &a[1], span)?;
    Ok(Value::list(vec![q, r]))
}

pub(super) fn binomial(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = int(a, 0, "binomial", span)?;
    let k = int(a, 1, "binomial", span)?;
    if n < 0 {
        return Err(invalid(
            t!("`binomial` needs a number of 0 or more, not {n}", "`binomial` a besoin d'un nombre positif ou nul, pas {n}"),
            span,
        ));
    }
    if k < 0 || k > n {
        return Ok(Value::Int(0));
    }
    let k = k.min(n - k);
    // C(n, k) >= C(2k, k) >= 2^k: beyond, more than MAX_DIGITS digits for sure.
    if k > 340_000 {
        return Err(bigint::too_large(span));
    }
    // Its number of digits, before computing it.
    let digits: f64 = (1..=k).map(|i| ((n - k + i) as f64 / i as f64).log10()).sum();
    if digits > bigint::MAX_DIGITS as f64 {
        return Err(bigint::too_large(span));
    }
    let mut c = BigInt::from_i64(1);
    for i in 1..=k {
        let (q, _) = c.mul(&BigInt::from_i64(n - k + i)).div_mod_floor(&BigInt::from_i64(i));
        c = q;
    }
    bigint::checked(c, span)
}

/// The sign of a number: -1, 0 or 1 (`None` for `nan`).
fn sign(a: &[Value], f: &str, span: Span) -> Fallible<Option<Ordering>> {
    Ok(match &a[0] {
        Value::Int(n) => Some(n.cmp(&0)),
        Value::Big(n) => Some(if n.is_negative() { Ordering::Less } else { Ordering::Greater }),
        Value::Float(x) => x.partial_cmp(&0.0),
        v => return Err(wrong(f, 0, Expected::Number, v, span)),
    })
}

pub(super) fn is_positive(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(sign(a, "is_positive", span)? == Some(Ordering::Greater)))
}

pub(super) fn is_negative(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(sign(a, "is_negative", span)? == Some(Ordering::Less)))
}

pub(super) fn is_zero(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(sign(a, "is_zero", span)? == Some(Ordering::Equal)))
}

pub(super) fn is_nan(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(number(a, 0, "is_nan", span)?.is_nan() && matches!(a[0], Value::Float(_))))
}

pub(super) fn is_finite(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    // A whole number is always finite, even one too large for a Float.
    Ok(Value::Bool(match &a[0] {
        Value::Float(x) => x.is_finite(),
        _ => {
            number(a, 0, "is_finite", span)?;
            true
        }
    }))
}

// ----- what JavaScript's Math has too ------------------------------------

pub(super) fn cbrt(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "cbrt", span)?.cbrt()))
}

/// The length of (x, y), computed without overflow: `sqrt(x * x + y * y)`
/// would give infinity for numbers above 1e154.
pub(super) fn hypot(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "hypot", span)?.hypot(number(a, 1, "hypot", span)?)))
}

pub(super) fn sinh(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "sinh", span)?.sinh()))
}

pub(super) fn cosh(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "cosh", span)?.cosh()))
}

pub(super) fn asinh(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "asinh", span)?.asinh()))
}

/// Like `asin` and `log`, a number outside the domain is an error, not NaN.
pub(super) fn acosh(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = number(a, 0, "acosh", span)?;
    if x.is_nan() || x < 1.0 {
        return Err(invalid(
            t!("`acosh` needs a number of 1 or more, not {x}", "`acosh` a besoin d'un nombre de 1 ou plus, pas {x}"),
            span,
        ));
    }
    Ok(Value::Float(x.acosh()))
}

/// At -1 and 1 the result would be infinite: an error, like `log(0)`.
pub(super) fn atanh(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = number(a, 0, "atanh", span)?;
    if x.is_nan() || x <= -1.0 || x >= 1.0 {
        return Err(invalid(
            t!(
                "`atanh` needs a number between -1 and 1 (both excluded), not {x}",
                "`atanh` a besoin d'un nombre entre -1 et 1 (exclus), pas {x}"
            ),
            span,
        ));
    }
    Ok(Value::Float(x.atanh()))
}
