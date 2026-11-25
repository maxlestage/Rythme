//! Texts encoded for the web: Base64 (RFC 4648, the standard alphabet with
//! `=` padding) and the percent-encoding of web addresses (RFC 3986). What
//! can fail to decode gives `Err(message)`.

use super::*;

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub(super) fn base64_encode(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let bytes = text(a, 0, "base64_encode", span)?.as_bytes();
    if bytes.len() / 3 * 4 > MAX_TEXT {
        return Err(too_big(false, span));
    }
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk.iter().enumerate().fold(0u32, |acc, (i, &b)| acc | (u32::from(b) << (16 - 8 * i)));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    Ok(text_value(out))
}

pub(super) fn base64_decode(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "base64_decode", span)?;
    let digits: Vec<u8> = s.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    let body = digits.strip_suffix(b"==").or_else(|| digits.strip_suffix(b"=")).unwrap_or(&digits);
    let mut values = Vec::with_capacity(body.len());
    for (i, &b) in body.iter().enumerate() {
        match ALPHABET.iter().position(|&c| c == b) {
            Some(v) => values.push(v as u32),
            None => {
                let c = s.chars().filter(|c| !c.is_ascii_whitespace()).nth(i).unwrap_or('?');
                return Ok(it.err(text_value(t!("`{c}` is not a Base64 character", "`{c}` n'est pas un caractère Base64"))));
            }
        }
    }
    if values.len() % 4 == 1 || (digits.len() != body.len() && !digits.len().is_multiple_of(4)) {
        return Ok(it.err(text_value(t!("this Base64 text is cut short", "ce texte Base64 est coupé"))));
    }
    let mut bytes = Vec::with_capacity(values.len() * 3 / 4);
    for chunk in values.chunks(4) {
        let n = chunk.iter().enumerate().fold(0u32, |acc, (i, &v)| acc | (v << (18 - 6 * i)));
        for i in 0..chunk.len() - 1 {
            bytes.push((n >> (16 - 8 * i)) as u8);
        }
    }
    Ok(match String::from_utf8(bytes) {
        Ok(text) => it.ok(text_value(text)),
        Err(_) => it.err(text_value(t!("the decoded bytes are not a text (UTF-8)", "les octets décodés ne sont pas un texte (UTF-8)"))),
    })
}

pub(super) fn url_encode(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "url_encode", span)?;
    if s.len().saturating_mul(3) > MAX_TEXT {
        return Err(too_big(false, span));
    }
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    Ok(text_value(out))
}

pub(super) fn url_decode(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "url_decode", span)?;
    let raw = s.as_bytes();
    let mut bytes = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'%' {
            let hex = raw.get(i + 1..i + 3).and_then(|h| std::str::from_utf8(h).ok()).and_then(|h| u8::from_str_radix(h, 16).ok());
            match hex {
                Some(b) if raw[i + 1].is_ascii_hexdigit() => {
                    bytes.push(b);
                    i += 3;
                }
                _ => {
                    return Ok(it.err(text_value(t!(
                        "a `%` must be followed by two hexadecimal digits, like `%20`",
                        "un `%` doit être suivi de deux chiffres hexadécimaux, comme `%20`"
                    ))))
                }
            }
        } else {
            bytes.push(raw[i]);
            i += 1;
        }
    }
    Ok(match String::from_utf8(bytes) {
        Ok(text) => it.ok(text_value(text)),
        Err(_) => it.err(text_value(t!("the decoded bytes are not a text (UTF-8)", "les octets décodés ne sont pas un texte (UTF-8)"))),
    })
}
