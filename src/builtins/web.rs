//! What JavaScript's `URL`, `URLSearchParams` and `crypto` give: a web
//! address cut into its parts (RFC 3986), the query of an address read into a
//! dict and written from one, the SHA-256 of a text, and random identifiers
//! (UUID version 4, RFC 9562).

use super::*;

/// The schemes whose addresses always name a host, and whose path is at
/// least `/` (the "special" schemes of the URL standard, but `file`).
const WITH_HOST: &[&str] = &["http", "https", "ws", "wss", "ftp"];

/// The parts of a web address, kept as they are written (still
/// percent-encoded, like JavaScript's `URL`), but the scheme and the host in
/// lower case.
struct Url<'a> {
    scheme: String,
    user: &'a str,
    password: &'a str,
    host: String,
    port: Option<u16>,
    path: String,
    query: &'a str,
    fragment: &'a str,
}

fn url_parts(s: &str) -> Result<Url<'_>, String> {
    if s.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(t!(
            "a web address cannot contain spaces, tabs or line breaks: write a space `%20`",
            "une adresse web ne peut pas contenir d'espaces, de tabulations ni de retours à la ligne : écrivez une espace `%20`"
        ));
    }
    let scheme = match s.split_once(':') {
        Some((scheme, _))
            if scheme.starts_with(|c: char| c.is_ascii_alphabetic())
                && scheme.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.')) =>
        {
            scheme
        }
        _ => {
            return Err(t!(
                "this web address does not start with its scheme: write it like `https://example.com/page`",
                "cette adresse web ne commence pas par son schéma : écrivez-la comme `https://example.com/page`"
            ))
        }
    };
    let rest = &s[scheme.len() + 1..];
    let scheme = scheme.to_ascii_lowercase();
    let (rest, fragment) = rest.split_once('#').unwrap_or((rest, ""));
    let (rest, query) = rest.split_once('?').unwrap_or((rest, ""));
    let (authority, path) = match rest.strip_prefix("//") {
        Some(r) => {
            let end = r.find('/').unwrap_or(r.len());
            (Some(&r[..end]), &r[end..])
        }
        None => (None, rest),
    };
    let mut url = Url { scheme, user: "", password: "", host: String::new(), port: None, path: path.to_owned(), query, fragment };
    if let Some(authority) = authority {
        let host_port = match authority.rsplit_once('@') {
            Some((info, host_port)) => {
                (url.user, url.password) = info.split_once(':').unwrap_or((info, ""));
                host_port
            }
            None => authority,
        };
        // An IPv6 address is written between brackets: `[::1]:8080`.
        let (host, port) = if host_port.starts_with('[') {
            match host_port.find(']') {
                Some(end) => host_port.split_at(end + 1),
                None => {
                    return Err(t!(
                        "the host `{host_port}` opens a `[` that no `]` closes",
                        "l'hôte `{host_port}` ouvre un `[` qu'aucun `]` ne ferme"
                    ))
                }
            }
        } else {
            host_port.find(':').map_or((host_port, ""), |at| host_port.split_at(at))
        };
        if let Some(c) = host.chars().find(|c| matches!(c, '<' | '>' | '"' | '\\' | '^' | '`' | '{' | '|' | '}' | '/' | '@')) {
            return Err(t!("the host `{host}` cannot contain `{c}`", "l'hôte `{host}` ne peut pas contenir `{c}`"));
        }
        url.host = host.to_ascii_lowercase();
        url.port = match port.strip_prefix(':') {
            None if port.is_empty() => None,
            Some("") => None,
            Some(digits) if digits.bytes().all(|b| b.is_ascii_digit()) && digits.len() <= 5 => match digits.parse::<u16>() {
                Ok(p) => Some(p),
                Err(_) => return Err(bad_port(digits)),
            },
            _ => return Err(bad_port(port.strip_prefix(':').unwrap_or(port))),
        };
    }
    if WITH_HOST.contains(&url.scheme.as_str()) {
        if url.host.is_empty() {
            let scheme = &url.scheme;
            return Err(t!(
                "an address `{scheme}:` names a host: write it like `{scheme}://example.com/page`",
                "une adresse `{scheme}:` nomme un hôte : écrivez-la comme `{scheme}://example.com/page`"
            ));
        }
        if url.path.is_empty() {
            url.path.push('/');
        }
    }
    Ok(url)
}

fn bad_port(port: &str) -> String {
    t!("the port `{port}` is not a number from 0 to 65535", "le port `{port}` n'est pas un nombre de 0 à 65535")
}

pub(super) fn parse_url(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_url", span)?;
    Ok(match url_parts(s) {
        Ok(url) => {
            let port = match url.port {
                Some(p) => it.some(Value::Int(i64::from(p))),
                None => it.none(),
            };
            let mut r = Record::new();
            r.set(Rc::from("scheme"), text_value(url.scheme));
            r.set(Rc::from("user"), text_value(url.user));
            r.set(Rc::from("password"), text_value(url.password));
            r.set(Rc::from("host"), text_value(url.host));
            r.set(Rc::from("port"), port);
            r.set(Rc::from("path"), text_value(url.path));
            r.set(Rc::from("query"), text_value(url.query));
            r.set(Rc::from("fragment"), text_value(url.fragment));
            it.ok(Value::Record(Rc::new(r)))
        }
        Err(message) => it.err(text_value(message)),
    })
}

/// A name or a value of a query: `+` is a space and `%20` too. What is not a
/// valid `%` sequence stays as it is, like in the browsers: a query comes
/// from anywhere, and reading it never fails.
fn query_decode(s: &str) -> String {
    let raw = s.as_bytes();
    let mut bytes = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        match raw[i] {
            b'+' => bytes.push(b' '),
            b'%' if i + 2 < raw.len() && raw[i + 1].is_ascii_hexdigit() && raw[i + 2].is_ascii_hexdigit() => {
                let hex = |b: u8| (b as char).to_digit(16).unwrap_or(0) as u8;
                bytes.push(hex(raw[i + 1]) * 16 + hex(raw[i + 2]));
                i += 2;
            }
            b => bytes.push(b),
        }
        i += 1;
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

pub(super) fn parse_query(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_query", span)?;
    let s = s.strip_prefix('?').unwrap_or(s);
    let pairs = s
        .split('&')
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            let (name, value) = piece.split_once('=').unwrap_or((piece, ""));
            (text_value(query_decode(name)), text_value(query_decode(value)))
        })
        .collect();
    Ok(Value::Dict(crate::dict::Map::from_pairs(pairs)))
}

/// Encoded like `url_encode`: only the letters, the digits and `-_.~` stay.
fn query_encode(out: &mut String, s: &str) {
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
}

pub(super) fn format_query(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let pairs: Vec<(String, &Value)> = match &a[0] {
        Value::Record(r) => r.iter().map(|(k, v)| (k.to_string(), v)).collect(),
        Value::Dict(d) => d.iter().map(|(k, v)| (k.display(), v)).collect(),
        v => return Err(wrong("format_query", 0, Expected::RecordOrDict, v, span)),
    };
    let mut out = String::new();
    for (name, value) in pairs {
        // A list gives the name once for each of its elements: `tag=a&tag=b`.
        let values: Vec<&Value> = match value {
            Value::List(l) => l.iter().collect(),
            v => vec![v],
        };
        for v in values {
            if !out.is_empty() {
                out.push('&');
            }
            query_encode(&mut out, &name);
            out.push('=');
            // A text as it is; any other value as `print` shows it.
            query_encode(&mut out, &v.display());
            if out.len() > MAX_TEXT {
                return Err(too_big(false, span));
            }
        }
    }
    Ok(text_value(out))
}

pub(super) fn sha256(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(crate::sha256::hex(text(a, 0, "sha256", span)?.as_bytes())))
}

/// From the numbers of `random`: `random_seed` makes them repeatable, which
/// tests need, and they are not for secrets.
pub(super) fn uuid(it: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&it.random_u64().to_be_bytes());
    bytes[8..].copy_from_slice(&it.random_u64().to_be_bytes());
    bytes[6] = (bytes[6] & 0x0f) | 0x40; // version 4: random
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // the variant of RFC 9562
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    Ok(text_value(format!("{}-{}-{}-{}-{}", &hex[..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queries_decode_leniently() {
        assert_eq!(query_decode("a+b%20c"), "a b c");
        assert_eq!(query_decode("100%"), "100%");
        assert_eq!(query_decode("%zz%4"), "%zz%4");
        assert_eq!(query_decode("caf%C3%A9"), "café");
    }

    #[test]
    fn addresses_are_cut_into_parts() {
        let u = url_parts("HTTPS://ada:pw@Example.COM:8080/a/b?x=1#top").unwrap();
        assert_eq!((u.scheme.as_str(), u.user, u.password, u.host.as_str(), u.port), ("https", "ada", "pw", "example.com", Some(8080)));
        assert_eq!((u.path.as_str(), u.query, u.fragment), ("/a/b", "x=1", "top"));
        let u = url_parts("http://[::1]:3000").unwrap();
        assert_eq!((u.host.as_str(), u.port, u.path.as_str()), ("[::1]", Some(3000), "/"));
        let u = url_parts("mailto:ada@example.com").unwrap();
        assert_eq!((u.host.as_str(), u.path.as_str()), ("", "ada@example.com"));
        let u = url_parts("file:///etc/hosts").unwrap();
        assert_eq!((u.host.as_str(), u.path.as_str()), ("", "/etc/hosts"));
        for bad in
            ["example.com/page", "https://", "http:example.com", "https://a.com:99999/", "https://a.com:8o/", "http://[::1", "a b:c", ""]
        {
            assert!(url_parts(bad).is_err(), "{bad}");
        }
    }
}
