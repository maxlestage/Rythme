//! The URLs of the site: `/{lang}/` (every language of `i18n.rs`) and its pages, behind an optional base path
//! (`/Rythme` when the site is exported for GitHub Pages). The pages with content also
//! exist as Markdown, for AI assistants: `/{lang}/learn/index.md`.

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
    /// For AI assistants: how to give them Rythme.
    Ai,
    /// Shown by the service worker for a page that was not saved, without network.
    Offline,
    NotFound,
}

impl Page {
    /// Every page that has an address, in the order of the navigation.
    pub const ALL: [Page; 6] = [Page::Home, Page::Learn, Page::Examples, Page::Playground, Page::Teach, Page::Ai];
    /// The pages that have an address but are not in the navigation nor the sitemap.
    pub const EXTRA: [Page; 1] = [Page::Offline];

    /// The part of the path after the language (`""` for the home page).
    pub fn slug(self) -> &'static str {
        match self {
            Page::Home => "",
            Page::Learn => "learn",
            Page::Examples => "examples",
            Page::Playground => "playground",
            Page::Teach => "teach",
            Page::Ai => "ai",
            Page::Offline => "offline",
            Page::NotFound => "404",
        }
    }

    /// The pages that also exist as Markdown (`index.md`): those with content to read.
    pub fn has_markdown(self) -> bool {
        matches!(self, Page::Home | Page::Learn | Page::Examples | Page::Teach | Page::Ai)
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
            [slug] => Page::ALL.into_iter().chain(Page::EXTRA).find(|p| p.slug() == slug && *p != Page::Home).unwrap_or(Page::NotFound),
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

    /// The route of the Markdown version of a page, `/{lang}/learn/index.md`.
    pub fn parse_markdown(path: &str) -> Option<Route> {
        let page = path.strip_suffix("index.md")?;
        Route::parse(page).filter(|r| r.page.has_markdown() && r.path() == page)
    }

    /// The path of the Markdown version of the page, e.g. `/en/learn/index.md`.
    pub fn markdown_path(&self) -> String {
        format!("{}index.md", self.path())
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
        assert_eq!(Route::parse("/en/offline/"), Some(Route::new(Lang::En, Page::Offline)));
    }

    #[test]
    fn unknown_paths() {
        assert_eq!(Route::parse("/"), None);
        assert_eq!(Route::parse("/it/"), None);
        assert_eq!(Route::parse("/zh/learn/"), Some(Route::new(Lang::Zh, Page::Learn)));
        assert_eq!(Route::parse("/fr/nothing/").map(|r| r.page), Some(Page::NotFound));
        assert_eq!(Route::parse("/en/learn/more/").map(|r| r.page), Some(Page::NotFound));
    }

    #[test]
    fn markdown_paths() {
        for route in Route::all().filter(|r| r.page.has_markdown()) {
            assert_eq!(Route::parse_markdown(&route.markdown_path()), Some(route));
        }
        assert_eq!(Route::parse_markdown("/en/index.md"), Some(Route::new(Lang::En, Page::Home)));
        for path in [
            "/en/playground/index.md",
            "/en/offline/index.md",
            "/en/nothing/index.md",
            "/en/learn/index.html",
            "/it/index.md",
            "/en//learn/index.md",
        ] {
            assert_eq!(Route::parse_markdown(path), None, "{path}");
        }
    }
}
