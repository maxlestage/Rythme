//! Text functions that students and professionals expect from Python, Ruby
//! and Elixir: centering, the first replacement, counting and finding the
//! last occurrence, prefixes and suffixes, the cases of names in code
//! (`snake_case`, `camelCase`, `kebab-case`), bytes, wrapping a paragraph,
//! the distance between two words; and `inspect`, `print_error`.

use super::*;

/// The error for a piece of text to look for that is empty.
fn empty_piece(f: &str, span: Span) -> Box<Diagnostic> {
    invalid(t!("`{f}`: the text to look for is empty", "`{f}` : le texte à chercher est vide"), span)
}

pub(super) fn center(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let width = count_arg(a, 1, "center", span)?;
    let fill = if a.len() == 3 { one_char(a, 2, "center", span)? } else { ' ' };
    let s = a[0].display();
    let missing = width.saturating_sub(s.chars().count());
    if missing == 0 {
        return Ok(text_value(s));
    }
    if s.len().saturating_add(missing.saturating_mul(fill.len_utf8())) > MAX_TEXT {
        return Err(too_big(false, span));
    }
    // The extra character, when there is one, goes to the right (like Ruby).
    let left = missing / 2;
    let mut out: String = std::iter::repeat_n(fill, left).collect();
    out.push_str(&s);
    out.extend(std::iter::repeat_n(fill, missing - left));
    Ok(text_value(out))
}

pub(super) fn replace_first(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "replace_first", span)?;
    let old = text(a, 1, "replace_first", span)?;
    let new = text(a, 2, "replace_first", span)?;
    if old.is_empty() {
        return Err(invalid(
            t!("`{f}` cannot replace an empty text", "`{f}` ne peut pas remplacer un texte vide", f = "replace_first"),
            span,
        ));
    }
    Ok(text_value(s.replacen(old, new, 1)))
}

pub(super) fn occurrences(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Int(match (&a[0], &a[1]) {
        (Value::List(l), x) => l.iter().filter(|v| v.equals(x)).count() as i64,
        (Value::Text(s), Value::Text(piece)) => {
            if piece.is_empty() {
                return Err(empty_piece("occurrences", span));
            }
            s.matches(&**piece).count() as i64
        }
        (Value::Text(_), v) => return Err(wrong("occurrences", 1, Expected::Text, v, span)),
        (v, _) => return Err(wrong("occurrences", 0, Expected::ListOrText, v, span)),
    }))
}

pub(super) fn last_index_of(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match (&a[0], &a[1]) {
        (Value::List(l), x) => it.option(l.iter().rposition(|v| v.equals(x)).map(|i| Value::Int(i as i64))),
        (Value::Text(s), Value::Text(sub)) => it.option(s.rfind(&**sub).map(|byte| Value::Int(s[..byte].chars().count() as i64))),
        (Value::Text(_), v) => return Err(wrong("last_index_of", 1, Expected::Text, v, span)),
        (v, _) => return Err(wrong("last_index_of", 0, Expected::ListOrText, v, span)),
    })
}

pub(super) fn remove_prefix(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "remove_prefix", span)?;
    let prefix = text(a, 1, "remove_prefix", span)?;
    Ok(text_value(s.strip_prefix(prefix).unwrap_or(s)))
}

pub(super) fn remove_suffix(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "remove_suffix", span)?;
    let suffix = text(a, 1, "remove_suffix", span)?;
    Ok(text_value(s.strip_suffix(suffix).unwrap_or(s)))
}

pub(super) fn capitalize_words(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "capitalize_words", span)?;
    let mut out = String::with_capacity(s.len());
    let mut start = true;
    for c in s.chars() {
        if c.is_whitespace() {
            out.push(c);
            start = true;
        } else if start {
            out.push_str(&titlecase(c));
            start = false;
        } else {
            out.push(c);
        }
    }
    Ok(text_value(out))
}

/// The words of a name written in any case: `HTTPServer`, `http_server`,
/// `http-server` and `Http Server` all give `HTTP`/`http`... and `Server`.
/// A word ends at a character that is neither a letter nor a digit, before
/// an uppercase letter that follows a lowercase one or a digit, and before
/// the last uppercase letter of a group followed by a lowercase one.
fn name_words(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut words = Vec::new();
    let mut word = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if !c.is_alphanumeric() {
            if !word.is_empty() {
                words.push(std::mem::take(&mut word));
            }
            continue;
        }
        if c.is_uppercase() && !word.is_empty() {
            let prev = chars[i - 1];
            let next_lower = chars.get(i + 1).is_some_and(|n| n.is_lowercase());
            if prev.is_lowercase() || prev.is_ascii_digit() || (prev.is_uppercase() && next_lower) {
                words.push(std::mem::take(&mut word));
            }
        }
        word.push(c);
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

fn joined_case(a: &[Value], f: &str, sep: &str, span: Span) -> Fallible<Value> {
    let s = text(a, 0, f, span)?;
    let words: Vec<String> = name_words(s).iter().map(|w| w.to_lowercase()).collect();
    Ok(text_value(words.join(sep)))
}

pub(super) fn snake_case(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    joined_case(a, "snake_case", "_", span)
}

pub(super) fn kebab_case(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    joined_case(a, "kebab_case", "-", span)
}

pub(super) fn camel_case(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "camel_case", span)?;
    let mut out = String::with_capacity(s.len());
    for (i, w) in name_words(s).iter().enumerate() {
        let lower = w.to_lowercase();
        if i == 0 {
            out.push_str(&lower);
        } else {
            let mut cs = lower.chars();
            if let Some(c) = cs.next() {
                out.extend(c.to_uppercase());
                out.push_str(cs.as_str());
            }
        }
    }
    Ok(text_value(out))
}

pub(super) fn swap_case(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "swap_case", span)?;
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_uppercase() {
            out.extend(c.to_lowercase());
        } else if c.is_lowercase() {
            out.extend(c.to_uppercase());
        } else {
            out.push(c);
        }
    }
    Ok(text_value(out))
}

/// Has the text a letter with a case, and none in the other case?
fn cased(a: &[Value], f: &str, upper: bool, span: Span) -> Fallible<Value> {
    let s = text(a, 0, f, span)?;
    let (mut ups, mut lows) = (false, false);
    for c in s.chars() {
        ups |= c.is_uppercase();
        lows |= c.is_lowercase();
    }
    Ok(Value::Bool(if upper { ups && !lows } else { lows && !ups }))
}

pub(super) fn is_upper(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    cased(a, "is_upper", true, span)
}

pub(super) fn is_lower(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    cased(a, "is_lower", false, span)
}

pub(super) fn bytes(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "bytes", span)?;
    if s.len() > MAX_LIST {
        return Err(too_big(true, span));
    }
    Ok(Value::list(s.bytes().map(|b| Value::Int(i64::from(b))).collect()))
}

pub(super) fn split_once(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "split_once", span)?;
    let sep = text(a, 1, "split_once", span)?;
    if sep.is_empty() {
        return Err(empty_piece("split_once", span));
    }
    Ok(it.option(s.split_once(sep).map(|(x, y)| Value::list(vec![text_value(x), text_value(y)]))))
}

pub(super) fn word_wrap(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "word_wrap", span)?;
    let width = size_arg(a, 1, "word_wrap", span)?;
    let mut out = String::with_capacity(s.len());
    for (i, line) in s.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        let mut used = 0;
        for word in line.split_whitespace() {
            let n = word.chars().count();
            if used > 0 && used + 1 + n > width {
                out.push('\n');
                used = 0;
            } else if used > 0 {
                out.push(' ');
                used += 1;
            }
            out.push_str(word);
            used += n;
        }
    }
    Ok(text_value(out))
}

/// The most cells `edit_distance` fills: two texts of 7000 characters.
const MAX_DISTANCE_WORK: usize = 50_000_000;

pub(super) fn edit_distance(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x: Vec<char> = text(a, 0, "edit_distance", span)?.chars().collect();
    let y: Vec<char> = text(a, 1, "edit_distance", span)?.chars().collect();
    if x.len().saturating_mul(y.len()) > MAX_DISTANCE_WORK {
        return Err(invalid(
            t!(
                "`edit_distance` compares texts of at most {MAX_DISTANCE_WORK} pairs of characters, not {} by {}",
                "`edit_distance` compare des textes d'au plus {MAX_DISTANCE_WORK} paires de caractères, pas {} par {}",
                x.len(),
                y.len()
            ),
            span,
        ));
    }
    // Levenshtein, one row at a time.
    let mut row: Vec<usize> = (0..=y.len()).collect();
    for (i, cx) in x.iter().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, cy) in y.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = (above + 1).min(row[j] + 1).min(diagonal + usize::from(cx != cy));
            diagonal = above;
        }
    }
    Ok(Value::Int(row[y.len()] as i64))
}

pub(super) fn inspect(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    Ok(text_value(a[0].repr()))
}

pub(super) fn print_error(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    crate::io::err_line(&joined(a, " ", usize::MAX).unwrap_or_default());
    Ok(Value::Unit)
}
