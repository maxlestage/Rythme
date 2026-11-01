//! Language used for messages: English, French, and the languages of `src/i18n/`.
//!
//! Every message is written in English and French where it is used:
//! `t!("unknown name `{name}`", "nom inconnu `{name}`")`. Every other language
//! has a catalog, `src/i18n/<code>.rs`: the English texts with their
//! translation. `t!` finds the translations while compiling (the compiler
//! checks their `{names}`), [`pick`] finds those of fixed texts (help pages,
//! documentation of functions) while running. A text missing from a catalog
//! is shown in English; `tests/i18n.rs` lists them.
//!
//! With the feature `runtime-catalogs` (the website's playground, in
//! WebAssembly), only English and French are compiled in: the catalog of
//! another language is a file ([`catalog`]) given to [`register`] while
//! running, and until then that language shows English. `tests/catalogs.rs`
//! checks that both ways give the same messages.
//!
//! The language is picked from `RYTHME_LANG`, then from the usual locale
//! variables, and can be forced with `--lang es` or `:lang` in the REPL.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
#[cfg(not(feature = "runtime-catalogs"))]
use std::sync::OnceLock;

/// A language and its catalog (empty for English and French, written in `t!`).
pub struct Language {
    /// The code, as in `--lang es` and `LANG=es_ES.UTF-8`.
    pub code: &'static str,
    /// The name, in the language itself.
    pub name: &'static str,
    /// (English text, translation): an empty translation is still to write.
    pub texts: &'static [(&'static str, &'static str)],
}

/// A catalog of `src/i18n/`, compiled in.
#[cfg(not(feature = "runtime-catalogs"))]
macro_rules! catalog {
    ($code:literal) => {
        include!(concat!("i18n/", $code, ".rs"))
    };
}

/// With `runtime-catalogs`, the code and the name of a catalog of `src/i18n/`;
/// its texts are registered while running ([`register`]).
#[cfg(feature = "runtime-catalogs")]
macro_rules! catalog {
    ($code:literal) => {
        rythme_macros::__language!($code)
    };
}

/// Every language. Adding one: its catalog in `src/i18n/` and its line here.
pub static LANGUAGES: &[Language] = &[
    Language { code: "en", name: "English", texts: &[] },
    Language { code: "fr", name: "français", texts: &[] },
    catalog!("es"),
    catalog!("de"),
    catalog!("pt"),
    catalog!("zh"),
    catalog!("it"),
    catalog!("ja"),
];

/// A language of [`LANGUAGES`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lang(u8);

#[allow(non_upper_case_globals)]
impl Lang {
    pub const En: Lang = Lang(0);
    pub const Fr: Lang = Lang(1);

    pub fn language(self) -> &'static Language {
        &LANGUAGES[self.0 as usize]
    }

    pub fn code(self) -> &'static str {
        self.language().code
    }

    /// Every language, English first.
    pub fn all() -> impl Iterator<Item = Lang> {
        (0..LANGUAGES.len() as u8).map(Lang)
    }
}

static CURRENT: AtomicU8 = AtomicU8::new(0);

pub fn set(lang: Lang) {
    CURRENT.store(lang.0, Ordering::Relaxed);
}

pub fn get() -> Lang {
    Lang(CURRENT.load(Ordering::Relaxed))
}

/// The code of the current language: `"en"`, `"fr"`, `"es"`...
pub fn code() -> &'static str {
    get().code()
}

pub fn is_fr() -> bool {
    get() == Lang::Fr
}

/// `fr`, `fr_FR.UTF-8`, `français`, `ES`... (the first two letters are the code).
pub fn parse(s: &str) -> Option<Lang> {
    let s = s.trim().to_ascii_lowercase();
    Lang::all().find(|l| s.starts_with(l.code()))
}

/// The codes, for messages: "en, fr, es".
pub fn codes() -> String {
    LANGUAGES.iter().map(|l| l.code).collect::<Vec<_>>().join(", ")
}

/// Picks the language from the environment.
pub fn detect() -> Lang {
    if let Some(l) = std::env::var("RYTHME_LANG").ok().as_deref().and_then(parse) {
        return l;
    }
    for var in ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
        if let Ok(v) = std::env::var(var) {
            if !v.is_empty() {
                return parse(&v).unwrap_or(Lang::En);
            }
        }
    }
    Lang::En
}

/// The translation of `en` in `lang`, if its catalog has one.
#[cfg(not(feature = "runtime-catalogs"))]
pub fn translation(lang: Lang, en: &str) -> Option<&'static str> {
    static TABLES: OnceLock<Vec<HashMap<&'static str, &'static str>>> = OnceLock::new();
    let tables =
        TABLES.get_or_init(|| LANGUAGES.iter().map(|l| l.texts.iter().copied().filter(|(_, tr)| !tr.is_empty()).collect()).collect());
    tables.get(lang.0 as usize)?.get(en).copied()
}

/// The translation of `en` in `lang`, if its catalog has one and was registered.
#[cfg(feature = "runtime-catalogs")]
pub fn translation(lang: Lang, en: &str) -> Option<&'static str> {
    runtime::lookup(lang, key(en))
}

/// Picks one of two fixed texts according to the current language; for the
/// languages of `src/i18n/`, the translation of the English one.
pub fn pick(en: &'static str, fr: &'static str) -> &'static str {
    pick_in(get(), en, fr)
}

/// Like [`pick`], in a given language instead of the current one.
pub fn pick_in(lang: Lang, en: &'static str, fr: &'static str) -> &'static str {
    match lang {
        Lang::En => en,
        Lang::Fr => fr,
        other => translation(other, en).unwrap_or(en),
    }
}

/// `t!("English {x}", "Français {x}")` formats the message in the current
/// language (the other languages come from their catalog, see above).
#[cfg(not(feature = "runtime-catalogs"))]
#[macro_export]
macro_rules! t {
    ($($all:tt)*) => {
        $crate::lang::__tr!($crate; $($all)*)
    };
}

/// `t!("English {x}", "Français {x}")` formats the message in the current
/// language (the other languages come from the catalogs registered, see
/// [`register`]).
#[cfg(feature = "runtime-catalogs")]
#[macro_export]
macro_rules! t {
    ($($all:tt)*) => {
        $crate::lang::__tr!($crate; runtime; $($all)*)
    };
}

#[doc(hidden)]
pub use rythme_macros::__tr;

// ----- Catalogs as files, registered while running -----
//
// The website's playground runs Rythme in WebAssembly: every language compiled in
// would be most of its download, while a visitor reads one. With the feature
// `runtime-catalogs`, English and French stay compiled in (they are written at
// each call) and the page fetches the catalog of its language, made by
// [`catalog`] from the program compiled without the feature, then gives it to
// [`register`]. A text is found by the [`key`] of its English text.

/// The key of an English text in a catalog file: FNV-1a on 32 bits, the same as
/// the macro `t!` computes while compiling (which checks that no two texts of the
/// catalogs have the same).
pub fn key(en: &str) -> u32 {
    en.bytes().fold(0x811c_9dc5_u32, |h, b| (h ^ u32::from(b)).wrapping_mul(0x0100_0193))
}

/// The first line of a catalog file: its format, then the code of its language.
const CATALOG_HEADER: &str = "rythme-catalog 1";

/// The catalog of `lang` as a file for [`register`]: a first line, then one line
/// per translation, `key<TAB>text`, the key in hexadecimal and in the order of the
/// keys, the backslashes, line feeds and carriage returns of the text written
/// `\\`, `\n` and `\r`. Empty for English and French, which have no catalog.
#[cfg(not(feature = "runtime-catalogs"))]
pub fn catalog(lang: Lang) -> String {
    let language = lang.language();
    if language.texts.is_empty() {
        return String::new();
    }
    let mut entries: Vec<(u32, &str)> = language.texts.iter().filter(|(_, tr)| !tr.is_empty()).map(|(en, tr)| (key(en), *tr)).collect();
    entries.sort_by_key(|(k, _)| *k);
    let mut out = format!("{CATALOG_HEADER} {}\n", language.code);
    for (k, text) in entries {
        out.push_str(&format!("{k:08x}\t"));
        for c in text.chars() {
            match c {
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                c => out.push(c),
            }
        }
        out.push('\n');
    }
    out
}

/// Reads a catalog file written by [`catalog`]: its language and its texts, by
/// key.
pub fn read_catalog(file: &str) -> Result<(Lang, HashMap<u32, String>), String> {
    let mut lines = file.split('\n');
    let header = lines.next().unwrap_or_default();
    let code = header.strip_prefix(CATALOG_HEADER).and_then(|rest| rest.strip_prefix(' ')).ok_or("not a catalog of Rythme")?;
    let lang = Lang::all().find(|l| l.code() == code).ok_or_else(|| format!("unknown language `{code}`"))?;
    let mut texts = HashMap::new();
    for (n, line) in lines.enumerate().filter(|(_, l)| !l.is_empty()) {
        let bad = || format!("line {}: expected `key<TAB>text`", n + 2);
        let (k, text) = line.split_once('\t').ok_or_else(bad)?;
        let k = u32::from_str_radix(k, 16).map_err(|_| bad())?;
        let mut out = String::with_capacity(text.len());
        let mut chars = text.chars();
        while let Some(c) = chars.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            match chars.next() {
                Some('\\') => out.push('\\'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                _ => return Err(bad()),
            }
        }
        texts.insert(k, out);
    }
    Ok((lang, texts))
}

/// Registers a catalog file written by [`catalog`]: the messages and the help of
/// its language come from it from now on. Returns its language and how many
/// texts it has. Without the feature `runtime-catalogs`, the catalogs are
/// compiled in, and this only checks the file.
pub fn register(file: &str) -> Result<(Lang, usize), String> {
    let (lang, texts) = read_catalog(file)?;
    let n = texts.len();
    #[cfg(feature = "runtime-catalogs")]
    runtime::set(lang, texts);
    Ok((lang, n))
}

/// The translation of the current language whose English text has `key`, from
/// the catalog registered (`t!` with `runtime-catalogs`).
#[doc(hidden)]
#[cfg(feature = "runtime-catalogs")]
pub fn __runtime(key: u32) -> Option<&'static str> {
    runtime::lookup(get(), key)
}

/// A translation of a catalog file, its placeholders filled with `values`
/// (computed by `t!` with `format!`): `{name}`, `{name:spec}`, `{}` and `{0}`
/// read as the macro reads them, `{{` and `}}` as `{` and `}`.
#[doc(hidden)]
pub fn __fill(text: &str, values: &[(&str, String)]) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    let mut next = 0;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                out.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                out.push('}');
            }
            '{' => {
                let inner: String = chars.by_ref().take_while(|c| *c != '}').collect();
                let (arg, spec) = match inner.split_once(':') {
                    Some((arg, spec)) => (arg.trim(), Some(spec)),
                    None => (inner.trim(), None),
                };
                let arg = if arg.is_empty() {
                    next += 1;
                    (next - 1).to_string()
                } else {
                    arg.to_string()
                };
                let name = match spec {
                    Some(spec) => format!("{arg}:{spec}"),
                    None => arg,
                };
                match values.iter().find(|(n, _)| *n == name) {
                    Some((_, value)) => out.push_str(value),
                    // Not computed by `t!`: a catalog of another version.
                    None => {
                        out.push('{');
                        out.push_str(&inner);
                        out.push('}');
                    }
                }
            }
            c => out.push(c),
        }
    }
    out
}

/// The catalogs registered, by language. They live as long as the program: the
/// texts are kept once, and lent as `&'static str` like those compiled in.
#[cfg(feature = "runtime-catalogs")]
mod runtime {
    use super::{Lang, LANGUAGES};
    use std::collections::HashMap;
    use std::sync::Mutex;

    type Table = HashMap<u32, &'static str>;

    struct Tables {
        tables: Vec<Option<&'static Table>>,
        /// The languages already looked for in `RYTHME_CATALOGS`.
        #[cfg(not(target_arch = "wasm32"))]
        tried: Vec<bool>,
    }

    static TABLES: Mutex<Option<Tables>> = Mutex::new(None);

    fn with<R>(f: impl FnOnce(&mut Tables) -> R) -> R {
        let mut guard = TABLES.lock().unwrap_or_else(|e| e.into_inner());
        let tables = guard.get_or_insert_with(|| Tables {
            tables: vec![None; LANGUAGES.len()],
            #[cfg(not(target_arch = "wasm32"))]
            tried: vec![false; LANGUAGES.len()],
        });
        f(tables)
    }

    pub fn set(lang: Lang, texts: HashMap<u32, String>) {
        let table: Table = texts.into_iter().map(|(k, v)| (k, &*Box::leak(v.into_boxed_str()))).collect();
        let table: &'static Table = Box::leak(Box::new(table));
        with(|t| t.tables[lang.0 as usize] = Some(table));
    }

    pub fn lookup(lang: Lang, key: u32) -> Option<&'static str> {
        if lang == Lang::En || lang == Lang::Fr {
            return None;
        }
        #[cfg(not(target_arch = "wasm32"))]
        load_from_folder(lang);
        with(|t| t.tables[lang.0 as usize]).and_then(|table| table.get(&key).copied())
    }

    /// Natively, the catalogs of the folder `RYTHME_CATALOGS` (`<code>.txt`),
    /// read the first time a language needs one: how the tests compare these
    /// messages with those compiled in.
    #[cfg(not(target_arch = "wasm32"))]
    fn load_from_folder(lang: Lang) {
        let first = with(|t| !std::mem::replace(&mut t.tried[lang.0 as usize], true));
        if !first {
            return;
        }
        let Some(dir) = std::env::var_os("RYTHME_CATALOGS") else { return };
        let path = std::path::Path::new(&dir).join(format!("{}.txt", lang.code()));
        if let Ok(file) = std::fs::read_to_string(path) {
            let _ = super::register(&file);
        }
    }
}
