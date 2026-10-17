//! A small JSON reader and writer, enough for the language server protocol.
//!
//! Reading never panics: malformed text, deep nesting and invalid escapes
//! give an error message instead.

use std::fmt::{self, Write};

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    /// Keys keep their order, which makes the output easy to read.
    Obj(Vec<(String, Json)>),
}

static NULL: Json = Json::Null;

/// Deeper documents are refused (the protocol never nests that much).
const MAX_DEPTH: usize = 200;

/// Builds an object from `(key, value)` pairs.
pub fn obj<const N: usize>(pairs: [(&str, Json); N]) -> Json {
    Json::Obj(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

impl Json {
    pub fn parse(text: &str) -> Result<Json, String> {
        let mut r = Reader { s: text.as_bytes(), pos: 0, depth: 0 };
        r.ws();
        let v = r.value()?;
        r.ws();
        if r.pos != r.s.len() {
            return Err(format!("unexpected text after the value at byte {}", r.pos));
        }
        Ok(v)
    }

    /// The value of a field, or `Null` when this is not an object or the field is missing.
    pub fn get(&self, key: &str) -> &Json {
        match self {
            Json::Obj(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v).unwrap_or(&NULL),
            _ => &NULL,
        }
    }

    /// Follows a path of field names: `params.get_path(&["textDocument", "uri"])`.
    pub fn get_path(&self, path: &[&str]) -> &Json {
        path.iter().fold(self, |v, k| v.get(k))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            _ => None,
        }
    }

    /// A whole number that is not negative (positions, versions).
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Json::Num(n) if *n >= 0.0 && n.fract() == 0.0 && *n <= u64::MAX as f64 => Some(*n as u64),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Num(n) if n.fract() == 0.0 && n.abs() <= i64::MAX as f64 => Some(*n as i64),
            _ => None,
        }
    }

    pub fn as_array(&self) -> &[Json] {
        match self {
            Json::Arr(items) => items,
            _ => &[],
        }
    }

    /// Adds or replaces a field of an object.
    pub fn set(&mut self, key: &str, value: Json) {
        if let Json::Obj(fields) = self {
            match fields.iter_mut().find(|(k, _)| k == key) {
                Some((_, v)) => *v = value,
                None => fields.push((key.to_string(), value)),
            }
        }
    }
}

impl From<&str> for Json {
    fn from(s: &str) -> Json {
        Json::Str(s.to_string())
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::Str(s)
    }
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Bool(b)
    }
}

impl From<usize> for Json {
    fn from(n: usize) -> Json {
        Json::Num(n as f64)
    }
}

impl From<u32> for Json {
    fn from(n: u32) -> Json {
        Json::Num(n as f64)
    }
}

impl From<i64> for Json {
    fn from(n: i64) -> Json {
        Json::Num(n as f64)
    }
}

impl From<Vec<Json>> for Json {
    fn from(items: Vec<Json>) -> Json {
        Json::Arr(items)
    }
}

impl<T: Into<Json>> From<Option<T>> for Json {
    fn from(v: Option<T>) -> Json {
        v.map_or(Json::Null, Into::into)
    }
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Json::Null => f.write_str("null"),
            Json::Bool(b) => write!(f, "{b}"),
            Json::Num(n) => {
                if !n.is_finite() {
                    f.write_str("null")
                } else if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{n}")
                }
            }
            Json::Str(s) => write_str(f, s),
            Json::Arr(items) => {
                f.write_char('[')?;
                for (i, v) in items.iter().enumerate() {
                    if i > 0 {
                        f.write_char(',')?;
                    }
                    write!(f, "{v}")?;
                }
                f.write_char(']')
            }
            Json::Obj(fields) => {
                f.write_char('{')?;
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        f.write_char(',')?;
                    }
                    write_str(f, k)?;
                    write!(f, ":{v}")?;
                }
                f.write_char('}')
            }
        }
    }
}

fn write_str(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    f.write_char('"')?;
    for c in s.chars() {
        match c {
            '"' => f.write_str("\\\"")?,
            '\\' => f.write_str("\\\\")?,
            '\n' => f.write_str("\\n")?,
            '\r' => f.write_str("\\r")?,
            '\t' => f.write_str("\\t")?,
            c if (c as u32) < 0x20 => write!(f, "\\u{:04x}", c as u32)?,
            c => f.write_char(c)?,
        }
    }
    f.write_char('"')
}

struct Reader<'a> {
    s: &'a [u8],
    pos: usize,
    depth: usize,
}

impl Reader<'_> {
    fn ws(&mut self) {
        while self.pos < self.s.len() && matches!(self.s[self.pos], b' ' | b'\t' | b'\n' | b'\r') {
            self.pos += 1;
        }
    }

    fn error<T>(&self, what: &str) -> Result<T, String> {
        Err(format!("{what} at byte {}", self.pos))
    }

    fn literal(&mut self, word: &str, v: Json) -> Result<Json, String> {
        if self.s[self.pos..].starts_with(word.as_bytes()) {
            self.pos += word.len();
            Ok(v)
        } else {
            self.error("invalid literal")
        }
    }

    fn value(&mut self) -> Result<Json, String> {
        let Some(&c) = self.s.get(self.pos) else { return self.error("unexpected end") };
        match c {
            b'n' => self.literal("null", Json::Null),
            b't' => self.literal("true", Json::Bool(true)),
            b'f' => self.literal("false", Json::Bool(false)),
            b'"' => Ok(Json::Str(self.string()?)),
            b'[' | b'{' => {
                if self.depth >= MAX_DEPTH {
                    return self.error("nested too deeply");
                }
                self.depth += 1;
                let v = if c == b'[' { self.array() } else { self.object() };
                self.depth -= 1;
                v
            }
            b'-' | b'0'..=b'9' => self.number(),
            _ => self.error("unexpected character"),
        }
    }

    fn array(&mut self) -> Result<Json, String> {
        self.pos += 1;
        let mut items = Vec::new();
        self.ws();
        if self.s.get(self.pos) == Some(&b']') {
            self.pos += 1;
            return Ok(Json::Arr(items));
        }
        loop {
            self.ws();
            items.push(self.value()?);
            self.ws();
            match self.s.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Arr(items));
                }
                _ => return self.error("expected `,` or `]`"),
            }
        }
    }

    fn object(&mut self) -> Result<Json, String> {
        self.pos += 1;
        let mut fields: Vec<(String, Json)> = Vec::new();
        self.ws();
        if self.s.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(Json::Obj(fields));
        }
        loop {
            self.ws();
            if self.s.get(self.pos) != Some(&b'"') {
                return self.error("expected a field name");
            }
            let key = self.string()?;
            self.ws();
            if self.s.get(self.pos) != Some(&b':') {
                return self.error("expected `:`");
            }
            self.pos += 1;
            self.ws();
            let v = self.value()?;
            // A repeated key: the last one wins, as in most readers.
            match fields.iter_mut().find(|(k, _)| *k == key) {
                Some((_, old)) => *old = v,
                None => fields.push((key, v)),
            }
            self.ws();
            match self.s.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Obj(fields));
                }
                _ => return self.error("expected `,` or `}`"),
            }
        }
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        if self.s[self.pos] == b'-' {
            self.pos += 1;
        }
        let digits = |r: &mut Reader| {
            let from = r.pos;
            while r.pos < r.s.len() && r.s[r.pos].is_ascii_digit() {
                r.pos += 1;
            }
            r.pos > from
        };
        if !digits(self) {
            return self.error("invalid number");
        }
        if self.s.get(self.pos) == Some(&b'.') {
            self.pos += 1;
            if !digits(self) {
                return self.error("invalid number");
            }
        }
        if matches!(self.s.get(self.pos), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.s.get(self.pos), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !digits(self) {
                return self.error("invalid number");
            }
        }
        // Only ASCII bytes were consumed, so this slice is valid UTF-8.
        let text = std::str::from_utf8(&self.s[start..self.pos]).unwrap_or("0");
        text.parse::<f64>().map(Json::Num).or_else(|_| self.error("invalid number"))
    }

    fn hex4(&mut self) -> Result<u32, String> {
        let Some(hex) = self.s.get(self.pos..self.pos + 4) else { return self.error("invalid \\u escape") };
        let hex = std::str::from_utf8(hex).map_err(|_| format!("invalid \\u escape at byte {}", self.pos))?;
        let n = u32::from_str_radix(hex, 16).map_err(|_| format!("invalid \\u escape at byte {}", self.pos))?;
        self.pos += 4;
        Ok(n)
    }

    fn string(&mut self) -> Result<String, String> {
        self.pos += 1;
        let mut out = String::new();
        loop {
            let start = self.pos;
            while self.pos < self.s.len() && !matches!(self.s[self.pos], b'"' | b'\\') && self.s[self.pos] >= 0x20 {
                self.pos += 1;
            }
            match std::str::from_utf8(&self.s[start..self.pos]) {
                Ok(chunk) => out.push_str(chunk),
                Err(_) => return self.error("invalid UTF-8 in a text"),
            }
            match self.s.get(self.pos) {
                None => return self.error("text never closed"),
                Some(b'"') => {
                    self.pos += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    self.pos += 1;
                    let Some(&e) = self.s.get(self.pos) else { return self.error("text never closed") };
                    self.pos += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let mut n = self.hex4()?;
                            if (0xD800..0xDC00).contains(&n) && self.s[self.pos..].starts_with(b"\\u") {
                                let save = self.pos;
                                self.pos += 2;
                                let low = self.hex4()?;
                                if (0xDC00..0xE000).contains(&low) {
                                    n = 0x10000 + ((n - 0xD800) << 10) + (low - 0xDC00);
                                } else {
                                    self.pos = save;
                                }
                            }
                            out.push(char::from_u32(n).unwrap_or('\u{FFFD}'));
                        }
                        _ => return self.error("invalid escape"),
                    }
                }
                Some(_) => return self.error("control character in a text"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let text = r#"{"a":[1,-2.5,true,false,null],"b":"é\"\\\n🎉","c":{}}"#;
        let v = Json::parse(text).unwrap();
        assert_eq!(v.get("a").as_array().len(), 5);
        assert_eq!(v.get("b").as_str(), Some("é\"\\\n🎉"));
        assert_eq!(v.to_string(), text);
        assert_eq!(Json::parse(&v.to_string()).unwrap(), v);
    }

    #[test]
    fn escapes_and_numbers() {
        assert_eq!(Json::parse(r#""\u00e9\ud83c\udf89\u0041""#).unwrap(), Json::Str("é🎉A".into()));
        assert_eq!(Json::parse(r#""\ud800""#).unwrap(), Json::Str("\u{FFFD}".into()));
        assert_eq!(Json::parse("1e3").unwrap().as_u64(), Some(1000));
        assert_eq!(Json::parse("-7").unwrap().as_i64(), Some(-7));
        assert_eq!(Json::Num(0.5).to_string(), "0.5");
        assert_eq!(Json::Str("\u{1}".into()).to_string(), "\"\\u0001\"");
    }

    #[test]
    fn malformed_input_is_an_error() {
        for bad in ["", "{", "[1,", "{\"a\" 1}", "tru", "\"abc", "01x", "{\"a\":1}x", "\"\\q\"", "-", "1.", "[1 2]", "\"\\u12\""] {
            assert!(Json::parse(bad).is_err(), "{bad}");
        }
        let deep = "[".repeat(10_000);
        assert!(Json::parse(&deep).is_err());
    }
}
