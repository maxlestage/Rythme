//! Language used for messages: English or French.
//!
//! Every message shown to the user exists in both languages. The language is
//! picked from `RYTHME_LANG`, then from the usual locale variables, and can be
//! forced with `--lang fr` / `--lang en` or `:lang` in the REPL.

use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    Fr,
}

static CURRENT: AtomicU8 = AtomicU8::new(0);

pub fn set(lang: Lang) {
    CURRENT.store(lang as u8, Ordering::Relaxed);
}

pub fn get() -> Lang {
    if CURRENT.load(Ordering::Relaxed) == Lang::Fr as u8 {
        Lang::Fr
    } else {
        Lang::En
    }
}

pub fn is_fr() -> bool {
    get() == Lang::Fr
}

pub fn parse(s: &str) -> Option<Lang> {
    let s = s.trim().to_ascii_lowercase();
    if s.starts_with("fr") {
        Some(Lang::Fr)
    } else if s.starts_with("en") {
        Some(Lang::En)
    } else {
        None
    }
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

/// Picks one of two texts according to the current language.
pub fn pick(en: &'static str, fr: &'static str) -> &'static str {
    if is_fr() {
        fr
    } else {
        en
    }
}

/// `t!("English {x}", "Français {x}")` formats the message in the current language.
#[macro_export]
macro_rules! t {
    ($en:literal, $fr:literal $(,)?) => {
        if $crate::lang::is_fr() { format!($fr) } else { format!($en) }
    };
    ($en:literal, $fr:literal, $($arg:tt)*) => {
        if $crate::lang::is_fr() { format!($fr, $($arg)*) } else { format!($en, $($arg)*) }
    };
}
