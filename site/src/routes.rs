//! The URLs of the site: `/{lang}/` and its four pages, behind an optional base path
//! (`/Rythme` when the site is exported for GitHub Pages).

use std::sync::OnceLock;

pub use crate::i18n::Lang;

static BASE: OnceLock<String> = OnceLock::new();

/// Sets the base path once, before anything is rendered: `""` (the default) or
/// `/Rythme`.
pub fn set_base(base: &str) {
    let _ = BASE.set(normalize_base(base));
}

/// `""`, or `/something` without a trailing slash.
pub fn normalize_base(base: &str) -> String {
    match base.trim().trim_end_matches('/') {
        "" => String::new(),
        b if b.starts_with('/') => b.to_owned(),
        b => format!("/{b}"),
    }
}

/// The base path, without trailing slash (empty by default).
pub fn base() -> &'static str {
    BASE.get().map_or("", String::as_str)
}

/// A link to `path` (which starts with `/`), behind the base path.
pub fn link(path: &str) -> String {
    format!("{}{path}", base())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Home,
    Learn,
    Examples,
    Playground,
    Teach,
    NotFound,
}

impl Page {
    /// Every page that has an address, in the order of the navigation.
    pub const ALL: [Page; 5] = [Page::Home, Page::Learn, Page::Examples, Page::Playground, Page::Teach];

    /// The part of the path after the language (`""` for the home page).
    pub fn slug(self) -> &'static str {
        match self {
            Page::Home => "",
            Page::Learn => "learn",
            Page::Examples => "examples",
            Page::Playground => "playground",
            Page::Teach => "teach",
            Page::NotFound => "404",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Route {
    pub lang: Lang,
    pub page: Page,
}

impl Route {
    pub fn new(lang: Lang, page: Page) -> Route {
        Route { lang, page }
    }

    /// The route of a path; `None` when it does not start with a language.
    pub fn parse(path: &str) -> Option<Route> {
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        let lang = Lang::parse(parts.first()?)?;
        let page = match parts[1..] {
            [] => Page::Home,
            [slug] => Page::ALL.into_iter().find(|p| p.slug() == slug && *p != Page::Home).unwrap_or(Page::NotFound),
            _ => Page::NotFound,
        };
        Some(Route { lang, page })
    }

    /// The path of the page, e.g. `/fr/` or `/en/playground/`.
    pub fn path(&self) -> String {
        match self.page {
            Page::Home => format!("/{}/", self.lang.code()),
            page => format!("/{}/{}/", self.lang.code(), page.slug()),
        }
    }

    /// The link to the page, behind the base path.
    pub fn href(&self) -> String {
        link(&self.path())
    }

    /// The same page in another language.
    pub fn with_lang(&self, lang: Lang) -> Route {
        Route { lang, ..*self }
    }

    /// Every page in every language, for the sitemap.
    pub fn all() -> impl Iterator<Item = Route> {
        Lang::ALL.into_iter().flat_map(|l| Page::ALL.into_iter().map(move |p| Route::new(l, p)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_round_trip() {
        for route in Route::all() {
            assert_eq!(Route::parse(&route.path()), Some(route));
        }
    }

    #[test]
    fn unknown_paths() {
        assert_eq!(Route::parse("/"), None);
        assert_eq!(Route::parse("/de/"), None);
        assert_eq!(Route::parse("/fr/nothing/").map(|r| r.page), Some(Page::NotFound));
        assert_eq!(Route::parse("/en/learn/more/").map(|r| r.page), Some(Page::NotFound));
    }
}
