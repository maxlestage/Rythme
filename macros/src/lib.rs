//! The macro behind `t!("English {x}", "Français {x}", ...)` in Rythme.
//!
//! English and French are written at each call. Every other language has a
//! catalog, `src/i18n/<code>.rs`, a list of (English text, translation) pairs.
//! This macro looks the English text up in every catalog while compiling and
//! adds one `format!` per translation found, so a translation is checked by the
//! compiler like the two texts of the call (same `{names}`). A text missing
//! from a catalog is shown in English. `tests/i18n.rs` reports what is missing.
//!
//! With the feature `runtime-catalogs` of Rythme (the website's playground, in
//! WebAssembly), only English and French are compiled in: the other languages
//! come from tables registered while running, looked up by a key of the English
//! text, and this macro formats the placeholders of their translations from the
//! arguments (each with `format!`, so the text is the same as compiled in).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::OnceLock;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// One catalog: its language code and its translations, by English text.
struct Catalog {
    code: String,
    name: String,
    texts: HashMap<String, String>,
}

/// `__tr!($crate; "English", "Français" [, arguments])`, called by `t!`; with
/// `runtime;` after the crate (the feature `runtime-catalogs` of Rythme), the
/// languages of the catalogs come from tables registered while running.
#[proc_macro]
pub fn __tr(input: TokenStream) -> TokenStream {
    match expand(input) {
        Ok(ts) => ts,
        Err((message, span)) => error(&message, span),
    }
}

/// `__language!("es")`: `Language { code: "es", name: "Español", texts: &[] }`, the
/// code and the name of a catalog without its texts (the feature
/// `runtime-catalogs` of Rythme, whose texts are registered while running). The
/// catalog is included in a constant that nothing uses, so that the compiler
/// builds again when it changes (the placeholders of `t!` come from it).
#[proc_macro]
pub fn __language(input: TokenStream) -> TokenStream {
    let first = input.into_iter().next();
    let span = first.as_ref().map_or(Span::call_site(), |t| t.span());
    let code = match literal(first, span).map(|l| unescape(&l.to_string())) {
        Ok(Some(code)) => code,
        _ => return error("__language!: expected the code of a catalog, like \"es\"", span),
    };
    let catalogs = match catalogs() {
        Ok(c) => c,
        Err(e) => return error(e, span),
    };
    let Some(c) = catalogs.iter().find(|c| c.code == code) else {
        return error(&format!("__language!: no catalog `src/i18n/{code}.rs`"), span);
    };
    let file = format!("i18n/{code}.rs");
    let source = format!(
        "Language {{ code: {code:?}, name: {name:?}, texts: {{ const _: &str = include_str!({file:?}); &[] }} }}",
        code = c.code,
        name = c.name
    );
    TokenStream::from_str(&source).unwrap_or_else(|e| error(&format!("__language!: {e}"), span))
}

/// An argument of `t!` after the two texts: `name = value`, or a value.
struct Arg {
    name: Option<String>,
    value: Vec<TokenTree>,
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
    // `runtime;`: the catalogs are registered while running.
    let runtime = matches!(tokens.peek(), Some(TokenTree::Ident(i)) if i.to_string() == "runtime");
    if runtime {
        tokens.next();
        tokens.next();
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
        let mut ts = code("::std::format!");
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
    // `$crate::lang::<name>`
    let lang_fn = |name: &str| -> Vec<TokenTree> {
        let mut path = krate.clone();
        path.extend(code(&format!("::lang::{name}")));
        path
    };

    // match $crate::lang::code() { "fr" => format!(fr, ..), "es" => format!(es, ..), _ => format!(en, ..) }
    let mut arms = arm("fr", format(fr.into()));
    // With `runtime`, the placeholders of the translations registered while running,
    // formatted here from the arguments: the translations stay out of the program.
    let mut placeholders = Vec::new();
    let mut at_runtime = false;
    for c in catalogs {
        let Some(text) = c.texts.get(&key) else { continue };
        if runtime {
            if let Some(found) = placeholders_of(text) {
                at_runtime = true;
                for p in found {
                    if !placeholders.contains(&p) {
                        placeholders.push(p);
                    }
                }
                continue;
            }
            // A width or a precision taken from the arguments (`{:1$}`, `{:.*}`): too
            // rare to fill while running, compiled in as without `runtime`.
        }
        let mut lit = Literal::string(text);
        // The `{names}` of the text are the names written at the call.
        lit.set_span(en.span());
        arms.extend(arm(&c.code, format(lit.into())));
    }
    if !at_runtime {
        arms.extend(arm("_", format(en.into())));
    } else {
        // match $crate::lang::__runtime(KEY) {
        //     Some(__rythme_tr) => match (&(..), &(..),) {
        //         (name, __rythme_p0,) => $crate::lang::__fill(__rythme_tr, &[("name", format!("{name}")), ("0", format!("{__rythme_p0}"))]),
        //     },
        //     None => format!(en, ..),
        // }
        // The arguments are evaluated once, like `format!` does, and each placeholder
        // is formatted by `format!` itself: the same text as the translation compiled in.
        let args = arguments(&rest);
        let mut names: Vec<TokenTree> = Vec::new();
        let mut values: Vec<TokenTree> = Vec::new();
        let mut position = 0;
        for arg in &args {
            let name = match &arg.name {
                Some(name) => name.clone(),
                None => {
                    position += 1;
                    format!("__rythme_p{}", position - 1)
                }
            };
            names.push(Ident::new(&name, en.span()).into());
            names.push(Punct::new(',', Spacing::Alone).into());
            values.push(Punct::new('&', Spacing::Alone).into());
            values.push(Group::new(Delimiter::Parenthesis, arg.value.iter().cloned().collect()).into());
            values.push(Punct::new(',', Spacing::Alone).into());
        }
        let mut pairs: Vec<TokenTree> = Vec::new();
        for p in &placeholders {
            let mut lit = Literal::string(&rust_placeholder(p));
            lit.set_span(en.span());
            let mut pair = vec![Literal::string(p).into(), Punct::new(',', Spacing::Alone).into()];
            pair.extend(code("::std::format!"));
            pair.push(Group::new(Delimiter::Parenthesis, TokenTree::from(lit).into()).into());
            pairs.push(Group::new(Delimiter::Parenthesis, pair.into_iter().collect()).into());
            pairs.push(Punct::new(',', Spacing::Alone).into());
        }
        let mut fill = lang_fn("__fill");
        let mut fill_args = code("__rythme_tr, &");
        fill_args.push(Group::new(Delimiter::Bracket, pairs.into_iter().collect()).into());
        fill.push(Group::new(Delimiter::Parenthesis, fill_args.into_iter().collect()).into());
        // A `match` rather than a `let`: the temporaries of the arguments live until
        // the text is made, as with `format!`.
        let mut body: Vec<TokenTree> = vec![Ident::new("match", Span::call_site()).into()];
        body.push(Group::new(Delimiter::Parenthesis, values.into_iter().collect()).into());
        let mut case = code("#[allow(unused_variables)]");
        case.push(Group::new(Delimiter::Parenthesis, names.into_iter().collect()).into());
        case.extend(code("=>"));
        case.push(Group::new(Delimiter::Brace, fill.into_iter().collect()).into());
        body.push(Group::new(Delimiter::Brace, case.into_iter().collect()).into());

        let mut lookup: Vec<TokenTree> = vec![Ident::new("match", Span::call_site()).into()];
        lookup.extend(lang_fn("__runtime"));
        lookup.push(Group::new(Delimiter::Parenthesis, TokenTree::from(Literal::u32_suffixed(key_of(&key))).into()).into());
        let mut cases = code("::std::option::Option::Some(__rythme_tr) =>");
        cases.push(Group::new(Delimiter::Brace, body.into_iter().collect()).into());
        cases.extend(code("::std::option::Option::None =>"));
        cases.push(Group::new(Delimiter::Brace, format(en.into())).into());
        lookup.push(Group::new(Delimiter::Brace, cases.into_iter().collect()).into());
        arms.extend(arm("_", lookup.into_iter().collect()));
    }
    let mut out: Vec<TokenTree> = vec![Ident::new("match", Span::call_site()).into()];
    out.extend(lang_fn("code"));
    out.push(Group::new(Delimiter::Parenthesis, TokenStream::new()).into());
    out.push(Group::new(Delimiter::Brace, arms.into_iter().collect()).into());
    Ok(out.into_iter().collect())
}

/// Tokens of fixed code, at the call.
fn code(source: &str) -> Vec<TokenTree> {
    TokenStream::from_str(source).expect("valid tokens").into_iter().collect()
}

/// The arguments after the two texts (`rest` starts with their comma).
fn arguments(rest: &[TokenTree]) -> Vec<Arg> {
    let mut parts: Vec<Vec<TokenTree>> = vec![Vec::new()];
    for tt in rest {
        match tt {
            TokenTree::Punct(p) if p.as_char() == ',' => parts.push(Vec::new()),
            other => parts.last_mut().expect("a part").push(other.clone()),
        }
    }
    parts
        .into_iter()
        .filter(|p| !p.is_empty())
        .map(|part| match (part.first(), part.get(1)) {
            (Some(TokenTree::Ident(name)), Some(TokenTree::Punct(eq)))
                if eq.as_char() == '=' && eq.spacing() == Spacing::Alone && part.len() > 2 =>
            {
                Arg { name: Some(name.to_string()), value: part[2..].to_vec() }
            }
            _ => Arg { name: None, value: part },
        })
        .collect()
}

/// The placeholders of a text of `format!`, each as `name`, `name:spec`, `position`
/// or `position:spec` (`{}` numbered as `format!` does); `None` when one takes its
/// width or precision from the arguments. `rythme::lang::__fill` reads them the
/// same way.
fn placeholders_of(text: &str) -> Option<Vec<String>> {
    let mut found = Vec::new();
    let mut next = 0;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '{' {
            continue;
        }
        if chars.peek() == Some(&'{') {
            chars.next();
            continue;
        }
        let inner: String = chars.by_ref().take_while(|c| *c != '}').collect();
        let (arg, spec) = match inner.split_once(':') {
            Some((arg, spec)) => (arg.trim(), Some(spec)),
            None => (inner.trim(), None),
        };
        if spec.is_some_and(|s| s.contains(['$', '*'])) {
            return None;
        }
        let arg = if arg.is_empty() {
            next += 1;
            (next - 1).to_string()
        } else {
            arg.to_string()
        };
        found.push(match spec {
            Some(spec) => format!("{arg}:{spec}"),
            None => arg,
        });
    }
    Some(found)
}

/// A placeholder of [`placeholders_of`] as `format!` reads it, a position becoming
/// the name it is bound to: `0:>3` is `{__rythme_p0:>3}`.
fn rust_placeholder(p: &str) -> String {
    let (arg, spec) = match p.split_once(':') {
        Some((arg, spec)) => (arg, Some(spec)),
        None => (p, None),
    };
    let arg = if arg.chars().all(|c| c.is_ascii_digit()) { format!("__rythme_p{arg}") } else { arg.to_string() };
    match spec {
        Some(spec) => format!("{{{arg}:{spec}}}"),
        None => format!("{{{arg}}}"),
    }
}

/// The key of an English text in the tables registered while running: FNV-1a on
/// 32 bits (`rythme::lang::key` must give the same). [`catalogs`] checks that no
/// two texts have the same.
fn key_of(text: &str) -> u32 {
    text.bytes().fold(0x811c_9dc5_u32, |h, b| (h ^ u32::from(b)).wrapping_mul(0x0100_0193))
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
        let catalogs: Vec<Catalog> = files.iter().map(|p| read_catalog(p)).collect::<Result<_, _>>()?;
        // Each English text has its own key in the tables registered while running.
        let mut keys: HashMap<u32, &str> = HashMap::new();
        for text in catalogs.iter().flat_map(|c| c.texts.keys()) {
            if let Some(other) = keys.insert(key_of(text), text) {
                if other != text {
                    return Err(format!(
                        "t!: two English texts have the same key ({:08x}): {other:?} and {text:?}; change one of them a little, or the function `key_of` here and `rythme::lang::key`",
                        key_of(text)
                    ));
                }
            }
        }
        Ok(catalogs)
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
    let mut name = None;
    let mut texts = HashMap::new();
    let mut previous: Option<String> = None;
    for tt in body {
        match tt {
            TokenTree::Ident(i) => previous = Some(i.to_string()),
            TokenTree::Literal(l) if previous.as_deref() == Some("code") => {
                code = unescape(&l.to_string());
                previous = None;
            }
            TokenTree::Literal(l) if previous.as_deref() == Some("name") => {
                name = unescape(&l.to_string());
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
    let name = name.ok_or_else(|| fail("no `name: \"..\"`"))?;
    Ok(Catalog { code, name, texts })
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
