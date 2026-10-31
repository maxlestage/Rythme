//! Numbers written exactly as asked: `format(value, spec)` with the format
//! specifications of Python (`".2f"`, `">8"`, `"08,.2f"`, `"#x"`, `".1%"`),
//! and whole numbers in any base (`to_base`, `from_base`, `digits`).

use super::*;

/// A format specification: `[[fill]align][sign][#][0][width][,|_][.precision][type]`.
#[derive(Debug, Default)]
struct Spec {
    fill: Option<char>,
    align: Option<char>,
    sign: Option<char>,
    alternate: bool,
    zero: bool,
    width: usize,
    grouping: Option<char>,
    precision: Option<usize>,
    kind: Option<char>,
}

/// The widest result `format` makes (a width beyond is surely a mistake).
const MAX_WIDTH: usize = 10_000;

/// The most decimals `format` writes, like `text(x, decimals)`.
const MAX_PRECISION: usize = 100;

fn spec_error(spec: &str, why: String, span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E023", t!("`format` cannot use \"{spec}\": {why}", "`format` ne peut pas utiliser \"{spec}\" : {why}"))
        .at(span)
        .help(t!(
            "a spec is written [[fill]align][sign][#][0][width][,][.precision][type]: \">8\" (8 wide, on the right), \"^8\" (centered), \"08.2f\" (2 decimals, zeros before), \",d\" (thousands), \".1%\" (a percentage), \"x\" (hexadecimal), \"b\" (binary), \"e\" (exponent)",
            "une spécification s'écrit [[remplissage]alignement][signe][#][0][largeur][,][.précision][type] : \">8\" (largeur 8, à droite), \"^8\" (centré), \"08.2f\" (2 décimales, des zéros devant), \",d\" (milliers), \".1%\" (un pourcentage), \"x\" (hexadécimal), \"b\" (binaire), \"e\" (exposant)"
        ))
        .boxed()
}

fn parse_spec(s: &str, span: Span) -> Fallible<Spec> {
    let chars: Vec<char> = s.chars().collect();
    let mut spec = Spec::default();
    let mut i = 0;
    let is_align = |c: char| matches!(c, '<' | '>' | '^' | '=');
    if chars.len() >= 2 && is_align(chars[1]) {
        spec.fill = Some(chars[0]);
        spec.align = Some(chars[1]);
        i = 2;
    } else if chars.first().copied().is_some_and(is_align) {
        spec.align = Some(chars[0]);
        i = 1;
    }
    if let Some(&c) = chars.get(i).filter(|c| matches!(c, '+' | '-' | ' ')) {
        spec.sign = Some(c);
        i += 1;
    }
    if chars.get(i) == Some(&'#') {
        spec.alternate = true;
        i += 1;
    }
    if chars.get(i) == Some(&'0') {
        spec.zero = true;
        i += 1;
    }
    let number = |i: &mut usize| -> Option<usize> {
        let start = *i;
        while chars.get(*i).is_some_and(char::is_ascii_digit) {
            *i += 1;
        }
        let digits: String = chars[start..*i].iter().collect();
        if digits.is_empty() {
            None
        } else {
            Some(digits.parse().unwrap_or(usize::MAX))
        }
    };
    if let Some(w) = number(&mut i) {
        if w > MAX_WIDTH {
            return Err(spec_error(s, t!("the width is at most {MAX_WIDTH}", "la largeur est au plus {MAX_WIDTH}"), span));
        }
        spec.width = w;
    }
    if let Some(&c) = chars.get(i).filter(|c| matches!(c, ',' | '_')) {
        spec.grouping = Some(c);
        i += 1;
    }
    if chars.get(i) == Some(&'.') {
        i += 1;
        match number(&mut i) {
            Some(p) if p <= MAX_PRECISION => spec.precision = Some(p),
            Some(_) => {
                return Err(spec_error(s, t!("the precision is at most {MAX_PRECISION}", "la précision est au plus {MAX_PRECISION}"), span))
            }
            None => return Err(spec_error(s, t!("a number must follow the `.`", "un nombre doit suivre le `.`"), span)),
        }
    }
    if let Some(&c) = chars.get(i) {
        if !"bcdoxXeEfFgGns%".contains(c) {
            return Err(spec_error(s, t!("`{c}` is not a type of format", "`{c}` n'est pas un type de format"), span));
        }
        spec.kind = Some(c);
        i += 1;
    }
    if i < chars.len() {
        return Err(spec_error(s, t!("this is not a format spec", "ce n'est pas une spécification de format"), span));
    }
    Ok(spec)
}

/// `1234567` → `1,234,567` (every `every` digits from the right).
fn group(digits: &str, sep: char, every: usize) -> String {
    let n = digits.chars().count();
    let mut out = String::with_capacity(n + n / every);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (n - i).is_multiple_of(every) {
            out.push(sep);
        }
        out.push(c);
    }
    out
}

/// Python's exponent: `1.5e+03`, at least two digits.
fn exponent_form(x: f64, precision: usize, upper: bool, alternate: bool) -> String {
    let s = format!("{x:.precision$e}");
    let (mantissa, exp) = s.split_once('e').unwrap_or((&s, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let mut mantissa = mantissa.to_string();
    if alternate && !mantissa.contains('.') {
        mantissa.push('.');
    }
    let e = if upper { 'E' } else { 'e' };
    format!("{mantissa}{e}{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs())
}

/// Python's general form `g`: fixed or exponent, whichever is shorter for
/// `precision` significant digits, without zeros at the end.
fn general_form(x: f64, precision: usize, upper: bool, alternate: bool) -> String {
    let p = precision.max(1);
    let rounded = format!("{x:.prec$e}", prec = p - 1);
    let exp: i32 = rounded.split_once('e').and_then(|(_, e)| e.parse().ok()).unwrap_or(0);
    let mut s = if exp >= -4 && exp < p as i32 {
        let decimals = (p as i32 - 1 - exp).max(0) as usize;
        format!("{x:.decimals$}")
    } else {
        exponent_form(x, p - 1, upper, alternate)
    };
    if !alternate {
        // Zeros at the end of the decimals go, and the point if nothing follows it.
        let (number, exp) = match s.find(['e', 'E']) {
            Some(at) => (s[..at].to_string(), s[at..].to_string()),
            None => (s.clone(), String::new()),
        };
        let number = if number.contains('.') { number.trim_end_matches('0').trim_end_matches('.').to_string() } else { number };
        s = number + &exp;
    }
    s
}

/// Pads `body` (with its sign or prefix `head` before it) to the width of the spec.
fn pad_to(spec: &Spec, head: &str, body: &str, default_align: char) -> String {
    let len = head.chars().count() + body.chars().count();
    let missing = spec.width.saturating_sub(len);
    let (fill, align) = match (spec.fill, spec.align) {
        (f, Some(a)) => (f.unwrap_or(' '), a),
        (_, None) if spec.zero => ('0', '='),
        _ => (' ', default_align),
    };
    let fill = |n: usize| -> String { std::iter::repeat_n(fill, n).collect() };
    match align {
        '<' => format!("{head}{body}{}", fill(missing)),
        '^' => format!("{}{head}{body}{}", fill(missing / 2), fill(missing - missing / 2)),
        '=' => format!("{head}{}{body}", fill(missing)),
        _ => format!("{}{head}{body}", fill(missing)),
    }
}

fn sign_of(spec: &Spec, negative: bool) -> &'static str {
    match (negative, spec.sign) {
        (true, _) => "-",
        (false, Some('+')) => "+",
        (false, Some(' ')) => " ",
        _ => "",
    }
}

fn not_for(spec_text: &str, kind: char, what: String, span: Span) -> Box<Diagnostic> {
    spec_error(spec_text, t!("`{kind}` cannot write {what}", "`{kind}` ne peut pas écrire {what}"), span)
}

fn format_whole(spec: &Spec, spec_text: &str, n: &BigInt, span: Span) -> Fallible<String> {
    let kind = spec.kind.unwrap_or('d');
    let (radix, prefix) = match kind {
        'd' | 'n' => (10, ""),
        'b' => (2, "0b"),
        'o' => (8, "0o"),
        'x' => (16, "0x"),
        'X' => (16, "0X"),
        'c' => {
            let c = n.to_i64().and_then(|v| u32::try_from(v).ok()).and_then(char::from_u32);
            let Some(c) = c else {
                return Err(spec_error(
                    spec_text,
                    t!("{n} is not the number of a character", "{n} n'est pas le numéro d'un caractère"),
                    span,
                ));
            };
            return Ok(pad_to(spec, "", &c.to_string(), '<'));
        }
        _ => return Err(not_for(spec_text, kind, t!("a whole number", "un nombre entier"), span)),
    };
    if spec.precision.is_some() {
        return Err(spec_error(
            spec_text,
            t!(
                "a whole number has no decimals: write a type like `f` for them",
                "un nombre entier n'a pas de décimales : écrivez un type comme `f` pour en avoir"
            ),
            span,
        ));
    }
    let mut digits: String = n.digits_in(radix).iter().map(|&d| char::from_digit(u32::from(d), radix).unwrap_or('?')).collect();
    if kind == 'X' {
        digits = digits.to_uppercase();
    }
    if let Some(sep) = spec.grouping {
        if radix != 10 && sep == ',' {
            return Err(spec_error(
                spec_text,
                t!("`,` groups decimal digits only: use `_`", "`,` ne groupe que des chiffres décimaux : utilisez `_`"),
                span,
            ));
        }
        digits = group(&digits, sep, if radix == 10 { 3 } else { 4 });
    }
    let head = format!("{}{}", sign_of(spec, n.is_negative()), if spec.alternate { prefix } else { "" });
    Ok(pad_to(spec, &head, &digits, '>'))
}

fn format_decimal(spec: &Spec, spec_text: &str, x: f64, span: Span) -> Fallible<String> {
    let kind = spec.kind;
    let upper = matches!(kind, Some('E' | 'F' | 'G'));
    let negative = x.is_sign_negative() && !x.is_nan() && x != 0.0;
    let body = if !x.is_finite() {
        let word = if x.is_nan() { "nan" } else { "inf" };
        let word = if upper { word.to_uppercase() } else { word.to_string() };
        if kind == Some('%') {
            word + "%"
        } else {
            word
        }
    } else {
        let a = x.abs();
        let mut s = match kind {
            Some('f' | 'F') => format!("{a:.prec$}", prec = spec.precision.unwrap_or(6)),
            Some('e' | 'E') => exponent_form(a, spec.precision.unwrap_or(6), upper, spec.alternate),
            Some('g' | 'G') => general_form(a, spec.precision.unwrap_or(6), upper, spec.alternate),
            Some('%') => format!("{:.prec$}", a * 100.0, prec = spec.precision.unwrap_or(6)),
            Some('n') | None => match spec.precision {
                Some(p) => general_form(a, p, false, spec.alternate),
                None => fmt_float(a),
            },
            Some(k) => return Err(not_for(spec_text, k, t!("a decimal number", "un nombre décimal"), span)),
        };
        if spec.alternate && matches!(kind, Some('f' | 'F' | '%')) && !s.contains('.') {
            s.push('.');
        }
        if let Some(sep) = spec.grouping {
            let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
            s = group(&s[..end], sep, 3) + &s[end..];
        }
        if kind == Some('%') {
            s.push('%');
        }
        s
    };
    Ok(pad_to(spec, sign_of(spec, negative), &body, '>'))
}

pub(super) fn format(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let spec_text = text(a, 1, "format", span)?;
    let spec = parse_spec(spec_text, span)?;
    let out = match &a[0] {
        Value::Int(_) | Value::Big(_) => {
            let n = bigint::big(&a[0]).unwrap_or_default();
            match spec.kind {
                Some('e' | 'E' | 'f' | 'F' | 'g' | 'G' | '%') => format_decimal(&spec, spec_text, n.to_f64(), span)?,
                _ => format_whole(&spec, spec_text, &n, span)?,
            }
        }
        Value::Float(x) => format_decimal(&spec, spec_text, *x, span)?,
        v => {
            let what = match v {
                Value::Text(_) => t!("a text", "un texte"),
                _ => v.describe(),
            };
            if let Some(k) = spec.kind.filter(|k| *k != 's') {
                return Err(not_for(spec_text, k, what, span));
            }
            if spec.sign.is_some() || spec.alternate || spec.grouping.is_some() || (spec.zero && spec.align.is_none()) {
                return Err(spec_error(
                    spec_text,
                    t!("signs, `#`, `0` and `,` are for numbers", "les signes, `#`, `0` et `,` sont pour les nombres"),
                    span,
                ));
            }
            let s = v.display();
            let s: String = match spec.precision {
                Some(p) => s.chars().take(p).collect(),
                None => s,
            };
            pad_to(&spec, "", &s, '<')
        }
    };
    Ok(text_value(out))
}

/// A base from 2 to 36.
fn base_arg(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<u32> {
    let b = int(a, i, f, span)?;
    if !(2..=36).contains(&b) {
        return Err(invalid(t!("`{f}`: the base must go from 2 to 36, not {b}", "`{f}` : la base doit aller de 2 à 36, pas {b}"), span));
    }
    Ok(b as u32)
}

pub(super) fn to_base(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = whole(a, 0, "to_base", span)?;
    let radix = base_arg(a, 1, "to_base", span)?;
    let mut s = String::new();
    if n.is_negative() {
        s.push('-');
    }
    s.extend(n.digits_in(radix).iter().map(|&d| char::from_digit(u32::from(d), radix).unwrap_or('?')));
    Ok(text_value(s))
}

pub(super) fn from_base(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "from_base", span)?;
    let radix = base_arg(a, 1, "from_base", span)?;
    let t = s.trim();
    let (neg, rest) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    // `0x` before hexadecimal digits, `0o` before octal ones, `0b` before binary ones.
    let prefix = match radix {
        16 => Some("0x"),
        8 => Some("0o"),
        2 => Some("0b"),
        _ => None,
    };
    let rest = match prefix {
        Some(p) if rest.len() > 2 && rest[..2].eq_ignore_ascii_case(p) => &rest[2..],
        _ => rest,
    };
    let digits: Option<Vec<u8>> = rest
        .chars()
        .filter(|&c| c != '_')
        .map(|c| c.to_digit(radix).map(|d| d as u8))
        .collect::<Option<Vec<u8>>>()
        .filter(|d| !d.is_empty());
    let Some(digits) = digits.filter(|_| !rest.starts_with('_') && !rest.ends_with('_')) else {
        return Ok(
            it.err(text_value(t!("\"{s}\" is not a whole number in base {radix}", "\"{s}\" n'est pas un nombre entier en base {radix}")))
        );
    };
    // More digits than 10^MAX_DIGITS can have in this base: surely too large.
    let at_most = (bigint::MAX_DIGITS as f64 * 10f64.log(f64::from(radix))) as usize + 2;
    let n = if digits.len() > at_most { None } else { Some(BigInt::from_digits_in(neg, &digits, radix)) };
    Ok(match n.filter(|n| !n.too_large()) {
        Some(n) => it.ok(bigint::value(n)),
        None => it.err(text_value(t!("this number has more than {} digits", "ce nombre a plus de {} chiffres", bigint::MAX_DIGITS))),
    })
}

pub(super) fn digits(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = whole(a, 0, "digits", span)?;
    let radix = if a.len() == 2 { base_arg(a, 1, "digits", span)? } else { 10 };
    if n.is_negative() {
        return Err(invalid(
            t!("`digits` needs a number of 0 or more, not {}", "`digits` a besoin d'un nombre positif ou nul, pas {}", short_number(&n)),
            span,
        ));
    }
    Ok(Value::list(n.digits_in(radix).into_iter().map(|d| Value::Int(i64::from(d))).collect()))
}
