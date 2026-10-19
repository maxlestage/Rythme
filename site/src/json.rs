//! A small JSON writer, enough for the manifests of the app and the structured data
//! (JSON-LD) of the pages. Strings never close a `<script>`: `<` is written `<`.

use std::fmt;

/// A JSON value.
pub enum Json {
    Str(String),
    Bool(bool),
    Arr(Vec<Json>),
    Obj(Vec<(&'static str, Json)>),
}

/// A JSON string.
pub fn s(value: impl Into<String>) -> Json {
    Json::Str(value.into())
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Json::Str(value) => f.write_str(&js_string(value)),
            Json::Bool(value) => write!(f, "{value}"),
            Json::Arr(items) => {
                f.write_str("[")?;
                for (n, item) in items.iter().enumerate() {
                    if n > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, "{item}")?;
                }
                f.write_str("]")
            }
            Json::Obj(fields) => {
                f.write_str("{")?;
                for (n, (key, value)) in fields.iter().enumerate() {
                    if n > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, "{}:{value}", js_string(key))?;
                }
                f.write_str("}")
            }
        }
    }
}

/// A JSON (and JavaScript) string.
pub fn js_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '<' => out.push_str("\\u003c"),
            c if (c as u32) < 0x20 || c == '\u{2028}' || c == '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_cannot_close_a_script() {
        assert_eq!(js_string("a\"</script>\n"), r#""a\"\u003c/script>\u000a""#);
    }

    #[test]
    fn values() {
        let value = Json::Obj(vec![("a", s("x")), ("b", Json::Bool(true)), ("c", Json::Arr(vec![s("1"), Json::Arr(vec![])]))]);
        assert_eq!(value.to_string(), r#"{"a":"x","b":true,"c":["1",[]]}"#);
    }
}
