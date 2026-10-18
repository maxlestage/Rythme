//! The macro behind `t!("English {x}", "Français {x}", ...)` in Rythme.
//!
//! English and French are written at each call. Every other language has a
//! catalog, `src/i18n/<code>.rs`, a list of (English text, translation) pairs.
//! This macro looks the English text up in every catalog while compiling and
//! adds one `format!` per translation found, so a translation is checked by the
//! compiler like the two texts of the call (same `{names}`). A text missing
//! from a catalog is shown in English. `tests/i18n.rs` reports what is missing.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::OnceLock;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// One catalog: its language code and its translations, by English text.
struct Catalog {
    code: String,
    texts: HashMap<String, String>,
}

/// `__tr!($crate; "English", "Français" [, arguments])`, called by `t!`.
#[proc_macro]
pub fn __tr(input: TokenStream) -> TokenStream {
    match expand(input) {
        Ok(ts) => ts,
        Err((message, span)) => error(&message, span),
    }
}

fn expand(input: TokenStream) -> Result<TokenStream, (String, Span)> {
    let mut tokens = input.into_iter().peekable();
    // The path of the crate (`$crate`), up to the `;`.
    let mut krate = Vec::new();
    for tt in tokens.by_ref() {
        if matches!(&tt, TokenTree::Punct(p) if p.as_char() == ';') {
            break;
        }
        krate.push(tt);
    }
    let en = literal(tokens.next(), Span::call_site())?;
    expect_comma(tokens.next(), en.span())?;
    let fr = literal(tokens.next(), en.span())?;
    // The arguments, with their leading comma.
    let rest: Vec<TokenTree> = tokens.collect();
    let key = unescape(&en.to_string()).ok_or_else(|| ("t!: this text cannot be read".to_string(), en.span()))?;
    let catalogs = match catalogs() {
        Ok(c) => c,
        Err(e) => return Err((e.clone(), en.span())),
    };

    let format = |text: TokenTree| -> TokenStream {
        let mut ts: Vec<TokenTree> = vec![
            Punct::new(':', Spacing::Joint).into(),
            Punct::new(':', Spacing::Alone).into(),
            Ident::new("std", Span::call_site()).into(),
            Punct::new(':', Spacing::Joint).into(),
            Punct::new(':', Spacing::Alone).into(),
            Ident::new("format", Span::call_site()).into(),
            Punct::new('!', Spacing::Alone).into(),
        ];
        let mut args = vec![text];
        args.extend(rest.iter().cloned());
        ts.push(Group::new(Delimiter::Parenthesis, args.into_iter().collect()).into());
        ts.into_iter().collect()
    };
    let arm = |code: &str, body: TokenStream| -> Vec<TokenTree> {
        let pattern: TokenTree = if code == "_" { Ident::new("_", Span::call_site()).into() } else { Literal::string(code).into() };
        vec![
            pattern,
            Punct::new('=', Spacing::Joint).into(),
            Punct::new('>', Spacing::Alone).into(),
            Group::new(Delimiter::Brace, body).into(),
        ]
    };

    // match $crate::lang::code() { "fr" => format!(fr, ..), "es" => format!(es, ..), _ => format!(en, ..) }
    let mut arms = arm("fr", format(fr.into()));
    for c in catalogs {
        if let Some(text) = c.texts.get(&key) {
            let mut lit = Literal::string(text);
            // The `{names}` of the text are the names written at the call.
            lit.set_span(en.span());
            arms.extend(arm(&c.code, format(lit.into())));
        }
    }
    arms.extend(arm("_", format(en.into())));
    let mut out: Vec<TokenTree> = vec![Ident::new("match", Span::call_site()).into()];
    out.extend(krate);
    out.extend([
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new("lang", Span::call_site()).into(),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new("code", Span::call_site()).into(),
        Group::new(Delimiter::Parenthesis, TokenStream::new()).into(),
        Group::new(Delimiter::Brace, arms.into_iter().collect()).into(),
    ]);
    Ok(out.into_iter().collect())
}

/// A string literal, also when `macro_rules!` wrapped it in an invisible group.
fn literal(tt: Option<TokenTree>, span: Span) -> Result<Literal, (String, Span)> {
    match tt {
        Some(TokenTree::Literal(l)) if l.to_string().starts_with(['"', 'r']) => Ok(l),
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::None => {
            let mut inner = g.stream().into_iter();
            match (inner.next(), inner.next()) {
                (Some(tt), None) => literal(Some(tt), g.span()),
                _ => Err(("t!: expected a text".to_string(), g.span())),
            }
        }
        Some(other) => Err(("t!: expected a text (English, then French)".to_string(), other.span())),
        None => Err(("t!: expected two texts: English, then French".to_string(), span)),
    }
}

fn expect_comma(tt: Option<TokenTree>, span: Span) -> Result<(), (String, Span)> {
    match tt {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => Ok(()),
        Some(other) => Err(("t!: expected `,` between the two texts".to_string(), other.span())),
        None => Err(("t!: expected the French text after the English one".to_string(), span)),
    }
}

/// `compile_error!("...")` at `span`.
fn error(message: &str, span: Span) -> TokenStream {
    let mut lit = Literal::string(message);
    lit.set_span(span);
    let tokens: Vec<TokenTree> = vec![
        Ident::new("compile_error", span).into(),
        Punct::new('!', Spacing::Alone).into(),
        Group::new(Delimiter::Parenthesis, TokenTree::from(lit).into()).into(),
    ];
    tokens.into_iter().collect()
}

/// The folder of the catalogs: `src/i18n/` of the crate being compiled
/// (Rythme), or of the repository when another crate (the playground) uses `t!`.
fn folder() -> PathBuf {
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        let here = Path::new(&dir).join("src").join("i18n");
        if here.is_dir() {
            return here;
        }
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("src").join("i18n")
}

/// Every catalog, read once per compilation.
fn catalogs() -> &'static Result<Vec<Catalog>, String> {
    static CATALOGS: OnceLock<Result<Vec<Catalog>, String>> = OnceLock::new();
    CATALOGS.get_or_init(|| {
        let dir = folder();
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .map_err(|e| format!("t!: cannot read the catalogs in {}: {e}", dir.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "rs"))
            .collect();
        files.sort();
        files.iter().map(|p| read_catalog(p)).collect()
    })
}

/// Reads `Language { code: "es", name: "...", texts: &[("English", "Español"), ...] }`.
fn read_catalog(path: &Path) -> Result<Catalog, String> {
    let fail = |what: &str| format!("t!: {}: {what}", path.display());
    let source = std::fs::read_to_string(path).map_err(|e| fail(&e.to_string()))?;
    let tokens = TokenStream::from_str(&source).map_err(|e| fail(&format!("{e}")))?;
    let body = tokens
        .into_iter()
        .find_map(|tt| match tt {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => Some(g.stream()),
            _ => None,
        })
        .ok_or_else(|| fail("expected `Language { ... }`"))?;
    let mut code = None;
    let mut texts = HashMap::new();
    let mut previous: Option<String> = None;
    for tt in body {
        match tt {
            TokenTree::Ident(i) => previous = Some(i.to_string()),
            TokenTree::Literal(l) if previous.as_deref() == Some("code") => {
                code = unescape(&l.to_string());
                previous = None;
            }
            TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket && previous.as_deref() == Some("texts") => {
                for entry in g.stream() {
                    let TokenTree::Group(pair) = entry else { continue };
                    let parts: Vec<TokenTree> = pair.stream().into_iter().collect();
                    let text = |tt: Option<&TokenTree>| match tt {
                        Some(TokenTree::Literal(l)) => unescape(&l.to_string()),
                        _ => None,
                    };
                    match (text(parts.first()), text(parts.get(2))) {
                        // An empty translation is a translation still to write.
                        (Some(en), Some(tr)) if !tr.is_empty() => {
                            texts.insert(en, tr);
                        }
                        (Some(_), Some(_)) => {}
                        _ => return Err(fail(&format!("expected (\"English\", \"translation\"), found {pair}"))),
                    }
                }
            }
            _ => {}
        }
    }
    let code = code.ok_or_else(|| fail("no `code: \"..\"`"))?;
    Ok(Catalog { code, texts })
}

/// The value of a string literal as written in Rust: `"a\n"`, `r#"a"#`.
pub(crate) fn unescape(lit: &str) -> Option<String> {
    if let Some(raw) = lit.strip_prefix('r') {
        let hashes = raw.len() - raw.trim_start_matches('#').len();
        let inner = raw.get(hashes..raw.len() - hashes)?;
        return Some(inner.strip_prefix('"')?.strip_suffix('"')?.to_string());
    }
    let inner = lit.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            '0' => out.push('\0'),
            '\\' => out.push('\\'),
            '\'' => out.push('\''),
            '"' => out.push('"'),
            'x' => {
                let hex: String = [chars.next()?, chars.next()?].iter().collect();
                out.push(char::from(u8::from_str_radix(&hex, 16).ok()?));
            }
            'u' => {
                chars.next()?; // {
                let hex: String = chars.by_ref().take_while(|c| *c != '}').filter(|c| *c != '_').collect();
                out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
            }
            // A backslash at the end of a line: the line goes on after the spaces.
            '\n' | '\r' => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            _ => return None,
        }
    }
    Some(out)
}
