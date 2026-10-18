//! More text functions: trimming one side, words, comparing without the
//! case, and numbers written for people (`1,234,567.89`, `1 234 567,89`).

use super::*;

pub(super) fn trim_start(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "trim_start", span)?.trim_start()))
}

pub(super) fn trim_end(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "trim_end", span)?.trim_end()))
}

/// The accents written as a separate character after a letter (`e` + `◌́`).
fn is_mark(c: char) -> bool {
    matches!(c as u32, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

/// The words of a text: letters and digits, with `'`, `’` or `-` between
/// two of them (`it's`, `aujourd'hui`, `well-known`).
pub(super) fn words(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "words", span)?;
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut word = String::new();
    for (i, &c) in chars.iter().enumerate() {
        let inside = !word.is_empty();
        let joins = matches!(c, '\'' | '’' | '-') && inside && chars.get(i + 1).is_some_and(|n| n.is_alphanumeric());
        if c.is_alphanumeric() || (inside && is_mark(c)) || joins {
            word.push(c);
        } else if inside {
            out.push(text_value(std::mem::take(&mut word)));
        }
    }
    if !word.is_empty() {
        out.push(text_value(word));
    }
    Ok(Value::list(out))
}

/// The same text for every case: `Straße` and `STRASSE` give `strasse`.
fn folded(s: &str) -> String {
    s.to_uppercase().to_lowercase()
}

pub(super) fn equals_ignore_case(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = text(a, 0, "equals_ignore_case", span)?;
    let y = text(a, 1, "equals_ignore_case", span)?;
    Ok(Value::Bool(x == y || folded(x) == folded(y)))
}

/// The separator of thousands and the decimal point of a language.
fn number_style(language: &str) -> Option<(&'static str, &'static str)> {
    let code = language.split(['-', '_']).next().unwrap_or("").to_lowercase();
    Some(match code.as_str() {
        "en" => (",", "."),
        "fr" => (" ", ","),
        "de" | "es" | "it" | "nl" | "pt" => (".", ","),
        _ => return None,
    })
}

pub(super) fn format_number(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = "format_number";
    number(a, 0, f, span)?;
    if a.len() >= 2 {
        int(a, 1, f, span)?;
    }
    let language = if a.len() == 3 { Some(text(a, 2, f, span)?) } else { None };
    let decimals = if a.len() >= 2 {
        let d = count_arg(a, 1, f, span)?;
        if d > 100 {
            return Err(invalid(t!("`{f}` writes at most 100 decimals, not {d}", "`{f}` écrit au plus 100 décimales, pas {d}"), span));
        }
        Some(d)
    } else {
        None
    };
    let (group, point) = match language {
        None => (",", "."),
        Some(l) => number_style(l).ok_or_else(|| {
            Diagnostic::error("E023", t!("`{f}` does not know the language \"{l}\"", "`{f}` ne connaît pas la langue \"{l}\""))
                .at(span)
                .help(t!(
                    "the languages: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" and \"pt\" (1.234,5)",
                    "les langues : \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" et \"pt\" (1.234,5)"
                ))
                .boxed()
        })?,
    };
    let plain = match (&a[0], decimals) {
        (Value::Int(n), None | Some(0)) => n.to_string(),
        (Value::Int(n), Some(d)) => format!("{n}.{}", "0".repeat(d)),
        // Whole numbers beyond 64 bits are written exactly, digit by digit.
        (Value::Big(n), None | Some(0)) => n.to_string(),
        (Value::Big(n), Some(d)) => format!("{n}.{}", "0".repeat(d)),
        (Value::Float(x), Some(d)) => fixed(*x, d),
        (Value::Float(x), None) => fmt_float(*x),
        (v, _) => return Err(wrong(f, 0, Expected::Number, v, span)),
    };
    // `inf`, `nan` and `1e300` are left as they are.
    if !plain.bytes().all(|b| b.is_ascii_digit() || b == b'.' || b == b'-') {
        return Ok(text_value(plain.replace('.', point)));
    }
    let (sign, rest) = match plain.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", plain.as_str()),
    };
    let (whole, fraction) = match rest.split_once('.') {
        Some((w, f)) => (w, Some(f)),
        None => (rest, None),
    };
    let mut out = String::from(sign);
    for (i, d) in whole.chars().enumerate() {
        if i > 0 && (whole.len() - i) % 3 == 0 {
            out.push_str(group);
        }
        out.push(d);
    }
    if let Some(fraction) = fraction {
        out.push_str(point);
        out.push_str(fraction);
    }
    Ok(text_value(out))
}
