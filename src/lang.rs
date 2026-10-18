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
//! The language is picked from `RYTHME_LANG`, then from the usual locale
//! variables, and can be forced with `--lang es` or `:lang` in the REPL.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
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

/// Every language. Adding one: its catalog in `src/i18n/` and its line here.
pub static LANGUAGES: &[Language] = &[
    Language { code: "en", name: "English", texts: &[] },
    Language { code: "fr", name: "français", texts: &[] },
    include!("i18n/es.rs"),
    include!("i18n/de.rs"),
    include!("i18n/pt.rs"),
    include!("i18n/zh.rs"),
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
pub fn translation(lang: Lang, en: &str) -> Option<&'static str> {
    static TABLES: OnceLock<Vec<HashMap<&'static str, &'static str>>> = OnceLock::new();
    let tables =
        TABLES.get_or_init(|| LANGUAGES.iter().map(|l| l.texts.iter().copied().filter(|(_, tr)| !tr.is_empty()).collect()).collect());
    tables.get(lang.0 as usize)?.get(en).copied()
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
#[macro_export]
macro_rules! t {
    ($($all:tt)*) => {
        $crate::lang::__tr!($crate; $($all)*)
    };
}

#[doc(hidden)]
pub use rythme_macros::__tr;
