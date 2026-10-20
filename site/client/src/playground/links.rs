//! The program in the link itself (`#code=…`), see [`crate::share`]: packing it with
//! the browser's `CompressionStream` (when it has one), unpacking it with its
//! `DecompressionStream` or the inflate of `share`, and reading what the address asks
//! for.

use wasm_bindgen::{JsCast, JsValue};
use web_sys::UrlSearchParams;

use crate::js;
use crate::share::{self, Broken, CODE_MAX, LINK_MAX};

/// A compression stream of the browser (`CompressionStream` or `DecompressionStream`),
/// if it knows deflate-raw.
fn browser_stream(kind: &str) -> Option<JsValue> {
    js::construct(kind, &[JsValue::from_str("deflate-raw")]).ok()
}

/// `bytes` through a stream of the browser, at most [`CODE_MAX`] bytes out.
async fn through(bytes: &[u8], stream: JsValue) -> Result<Vec<u8>, Broken> {
    let parts = js_sys::Array::of1(&js_sys::Uint8Array::from(bytes));
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(|_| Broken::Damaged)?;
    let piped = js::call(&blob.stream(), "pipeThrough", &[stream]).map_err(|_| Broken::Damaged)?;
    let reader = js::call(&piped, "getReader", &[]).map_err(|_| Broken::Damaged)?;
    let mut out = Vec::new();
    loop {
        let chunk = js::call_async(&reader, "read", &[]).await.map_err(|_| Broken::Damaged)?;
        if js::get(&chunk, "done").is_truthy() {
            return Ok(out);
        }
        let value: js_sys::Uint8Array = js::get(&chunk, "value").dyn_into().map_err(|_| Broken::Damaged)?;
        if out.len() + value.length() as usize > CODE_MAX {
            let _ = js::call(&reader, "cancel", &[]);
            return Err(Broken::TooLong);
        }
        out.extend(value.to_vec());
    }
}

/// The program, packed for a link: compressed when the browser can and it is shorter.
pub async fn pack(code: &str) -> String {
    let bytes = code.as_bytes();
    let plain = format!("0:{}", share::to_base64(bytes));
    let Some(deflate) = browser_stream("CompressionStream") else { return plain };
    if bytes.len() > CODE_MAX {
        return plain;
    }
    match through(bytes, deflate).await {
        Ok(packed) => {
            let packed = format!("1:{}", share::to_base64(&packed));
            if packed.len() < plain.len() { packed } else { plain }
        }
        Err(_) => plain,
    }
}

/// The program of a packed link.
async fn unpack(compressed: bool, data: &str) -> Result<String, Broken> {
    let mut bytes = share::from_base64(data)?;
    if compressed {
        bytes = match browser_stream("DecompressionStream") {
            Some(native) => through(&bytes, native).await?,
            None => share::inflate(&bytes, CODE_MAX)?,
        };
    }
    share::program(bytes)
}

/// What the address asks for.
#[derive(Default)]
pub struct Asked {
    /// An example of the menu (`#example=fizzbuzz`).
    pub example: Option<String>,
    /// A program (`#code=…`, or a text shared with the installed app).
    pub code: Option<String>,
    /// What Run does: 0 runs, 2 tests (`&mode=test`).
    pub mode: u32,
    /// The seed of the draws (`&seed=42`).
    pub seed: Option<String>,
    /// The steps of a calculation (`&mode=steps`), of `expr` (`&expr=…`, empty: the last
    /// expression), opened on `step` (`&step=3`, from 1).
    pub steps: bool,
    pub expr: String,
    pub step: usize,
    /// A link that cannot be read.
    pub broken: bool,
}

/// A seed of the option: up to 15 digits.
pub fn is_seed(text: &str) -> bool {
    (1..=15).contains(&text.len()) && text.bytes().all(|b| b.is_ascii_digit())
}

/// `parseInt` of JavaScript, for the step of a link: the digits at the start.
fn leading_number(text: &str) -> Option<i64> {
    let text = text.trim_start();
    let (sign, digits) = match text.strip_prefix('-') {
        Some(rest) => (-1, rest),
        None => (1, text.strip_prefix('+').unwrap_or(text)),
    };
    let digits: String = digits.chars().take_while(char::is_ascii_digit).take(12).collect();
    digits.parse::<i64>().ok().map(|n| sign * n)
}

/// `#example=fizzbuzz`, `#code=…&mode=test` (with `&seed=42` before the mode when the
/// program runs with a seed), or `#code=…&mode=steps&step=3` (with `&expr=…`,
/// percent-encoded, for the steps of a piece of code rather than of the last
/// expression): links from the other pages and shared links; `?text=…&title=…&url=…`:
/// a text shared with the installed app (its share target), which becomes a `#code=`
/// link. A program from a link stays in the address until it is changed, so that a
/// reload shows it again.
pub async fn from_address() -> Asked {
    let window = crate::dom::window();
    let location = window.location();
    let hash = location.hash().unwrap_or_default();
    let hash = hash.strip_prefix('#').unwrap_or(&hash).to_owned();
    let params = UrlSearchParams::new_with_str(&hash).unwrap_or_else(|_| UrlSearchParams::new().expect("search parameters"));
    let mode = params.get("mode");
    let mut asked = Asked {
        example: params.get("example"),
        mode: if mode.as_deref() == Some("test") { 2 } else { 0 },
        seed: params.get("seed").filter(|s| is_seed(s)),
        steps: mode.as_deref() == Some("steps"),
        expr: {
            let expr = params.get("expr").unwrap_or_default();
            expr[..crate::editor::byte_at(&expr, LINK_MAX)].to_owned()
        },
        step: params.get("step").and_then(|s| leading_number(&s)).filter(|n| *n != 0).unwrap_or(1).clamp(1, 100_000) as usize,
        ..Asked::default()
    };
    // The raw value of `code=`, before any decoding.
    let raw = hash.split('&').find_map(|part| part.strip_prefix("code="));
    if let Some(raw) = raw {
        let code = if raw.len() > 4 * LINK_MAX {
            Err(Broken::TooLong)
        } else if let Some((compressed, data)) = share::packed(raw) {
            unpack(compressed, data).await
        } else {
            Ok(params.get("code").unwrap_or_default())
        };
        match code.and_then(|c| if crate::editor::utf16_len(&c) > CODE_MAX { Err(Broken::TooLong) } else { Ok(c) }) {
            Ok(code) => {
                asked.code = Some(code);
                asked.example = None;
            }
            Err(_) => asked.broken = true,
        }
    }
    let query = UrlSearchParams::new_with_str(&location.search().unwrap_or_default())
        .unwrap_or_else(|_| UrlSearchParams::new().expect("search parameters"));
    let history = window.history().ok();
    let path = location.pathname().unwrap_or_default();
    if ["text", "title", "url"].iter().any(|k| query.has(k)) {
        let text = query.get("text").filter(|t| !t.is_empty()).or_else(|| query.get("title")).unwrap_or_default();
        let code = match query.get("url").filter(|u| !u.is_empty()) {
            Some(url) if !text.contains(&url) => format!("{}# {url}\n", if text.is_empty() { String::new() } else { format!("{text}\n") }),
            _ => text,
        };
        let link = format!("{path}#code={}", String::from(js_sys::encode_uri_component(&code)));
        asked.code = Some(code);
        asked.example = None;
        asked.broken = false;
        if let Some(history) = &history {
            let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&link));
        }
    } else if (asked.example.is_some() || asked.broken)
        && let Some(history) = &history
    {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&path));
    }
    asked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_and_steps() {
        assert!(is_seed("42") && is_seed("999999999999999") && !is_seed("") && !is_seed("4x2") && !is_seed("1234567890123456"));
        assert_eq!(leading_number("3abc"), Some(3));
        assert_eq!(leading_number("-2"), Some(-2));
        assert_eq!(leading_number("x"), None);
    }
}
