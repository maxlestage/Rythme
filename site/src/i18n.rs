//! The languages of the site: French, English, Spanish, German, Portuguese (Brazilian)
//! and Chinese (simplified), the languages of the interpreter (`rythme::lang`). The
//! language of a page comes from its URL (`/fr/…`, `/zh/…`).
//!
//! Each language is a [`Texts`] constant in `i18n/<code>.rs`, and every value that
//! depends on the language is an array of [`COUNT`] values given to [`Lang::pick`]:
//! forgetting a text, or a language, is a compile error.

mod de;
mod en;
mod es;
mod fr;
mod pt;
mod zh;

pub use de::DE;
pub use en::EN;
pub use es::ES;
pub use fr::FR;
pub use pt::PT;
pub use zh::ZH;

/// The number of languages.
pub const COUNT: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Lang {
    Fr,
    En,
    Es,
    De,
    Pt,
    Zh,
}

impl Lang {
    /// In the order of the selector, and of every array given to [`Lang::pick`].
    pub const ALL: [Lang; COUNT] = [Lang::Fr, Lang::En, Lang::Es, Lang::De, Lang::Pt, Lang::Zh];

    /// The code of the addresses, of the cookie and of `rythme --lang`.
    pub fn code(self) -> &'static str {
        self.pick(["fr", "en", "es", "de", "pt", "zh"])
    }

    /// The BCP 47 tag of the pages (`lang`, `hreflang`, the manifests): Brazilian
    /// Portuguese, and Chinese in simplified characters, so that browsers pick the
    /// right glyphs.
    pub fn tag(self) -> &'static str {
        self.pick(["fr", "en", "es", "de", "pt-BR", "zh-Hans"])
    }

    /// The name of the language, in the language itself.
    pub fn name(self) -> &'static str {
        self.pick(["Français", "English", "Español", "Deutsch", "Português", "中文"])
    }

    /// The Open Graph locale.
    pub fn locale(self) -> &'static str {
        self.pick(["fr_FR", "en_US", "es_ES", "de_DE", "pt_BR", "zh_CN"])
    }

    /// The position of the language in `rythme::lang::LANGUAGES`: the language of the
    /// messages of the playground's interpreter (the tests check it).
    pub fn interpreter(self) -> usize {
        self.pick([1, 0, 2, 3, 4, 5])
    }

    /// Picks the value of this language in `[fr, en, es, de, pt, zh]`.
    pub fn pick<T: Copy>(self, values: [T; COUNT]) -> T {
        values[self as usize]
    }

    /// `fr`, `en`, `es`, `de`, `pt` or `zh` exactly.
    pub fn parse(code: &str) -> Option<Lang> {
        Lang::ALL.into_iter().find(|l| l.code() == code)
    }

    /// The language of a tag such as `fr-CA`, `EN` or `zh-Hant-TW`.
    pub fn from_tag(tag: &str) -> Option<Lang> {
        Lang::parse(&tag.get(..2)?.to_ascii_lowercase())
    }

    pub fn texts(self) -> &'static Texts {
        self.pick([&FR, &EN, &ES, &DE, &PT, &ZH])
    }
}

/// The preferred supported language of an `Accept-Language` header, French by default.
pub fn pick_lang(header: &str) -> Lang {
    let mut ranked: Vec<(Lang, f32)> = header
        .split(',')
        .filter_map(|part| {
            let mut pieces = part.trim().split(";q=");
            let lang = Lang::from_tag(pieces.next()?)?;
            let q = pieces.next().and_then(|q| q.trim().parse().ok()).unwrap_or(1.0);
            Some((lang, q))
        })
        .filter(|(_, q)| *q > 0.0)
        .collect();
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1));
    ranked.first().map_or(Lang::Fr, |r| r.0)
}

/// A title and its text.
pub struct Item {
    pub title: &'static str,
    pub text: &'static str,
}

const fn item(title: &'static str, text: &'static str) -> Item {
    Item { title, text }
}

/// A habit from another language and the Rythme way.
pub struct Habit {
    pub other: &'static str,
    pub rythme: &'static str,
    pub note: &'static str,
}

const fn habit(other: &'static str, rythme: &'static str, note: &'static str) -> Habit {
    Habit { other, rythme, note }
}

/// A step of the tour of the language: a title, a text and code that runs.
pub struct Step {
    pub title: &'static str,
    pub text: &'static str,
    pub code: &'static str,
}

const fn step(title: &'static str, text: &'static str, code: &'static str) -> Step {
    Step { title, text, code }
}

pub struct Texts {
    // Meta
    pub meta_title: &'static str,
    pub meta_description: &'static str,
    // Navigation
    pub skip: &'static str,
    pub brand_aria: &'static str,
    pub nav_aria: &'static str,
    pub nav_learn: &'static str,
    pub nav_examples: &'static str,
    pub nav_playground: &'static str,
    pub nav_teach: &'static str,
    pub menu: &'static str,
    pub lang_aria: &'static str,
    // Home: hero
    pub hero_eyebrow: &'static str,
    pub hero_title: &'static str,
    pub hero_lead: &'static str,
    pub cta_try: &'static str,
    pub cta_install: &'static str,
    pub hero_code: &'static str,
    pub hero_output: &'static str,
    // Home: why
    pub why_title: &'static str,
    pub why_lead: &'static str,
    pub why: [Item; 4],
    // Home: errors
    pub errors_title: &'static str,
    pub errors_lead: &'static str,
    pub errors_code: &'static str,
    pub errors_message: &'static str,
    pub errors_points: [&'static str; 3],
    // Home: habits from other languages
    pub habits_title: &'static str,
    pub habits_lead: &'static str,
    pub habits_head: [&'static str; 3],
    pub habits: [Habit; 6],
    // Home: install and commands
    pub install_title: &'static str,
    pub install_lead: &'static str,
    pub install_steps: [Item; 3],
    pub commands_title: &'static str,
    pub commands: [(&'static str, &'static str); 12],
    pub commands_lang: &'static str,
    pub copy: &'static str,
    pub copied: &'static str,
    // Home: call to action
    pub cta_title: &'static str,
    pub cta_text: &'static str,
    // Footer
    pub footer_tagline: &'static str,
    pub footer_made: &'static str,
    pub footer_license: &'static str,
    pub footer_source: &'static str,
    // Playground
    pub pg_title: &'static str,
    pub pg_lead: &'static str,
    pub pg_example: &'static str,
    pub pg_welcome: &'static str,
    pub pg_run: &'static str,
    pub pg_check: &'static str,
    pub pg_test: &'static str,
    pub pg_steps: &'static str,
    pub pg_steps_title: &'static str,
    pub pg_format: &'static str,
    pub pg_format_title: &'static str,
    pub pg_stop: &'static str,
    pub pg_code: &'static str,
    pub pg_output: &'static str,
    pub pg_help: &'static str,
    pub pg_help_intro: &'static str,
    pub pg_search: &'static str,
    pub pg_search_go: &'static str,
    pub pg_limits: &'static str,
    pub pg_noscript: &'static str,
    pub pg_welcome_code: &'static str,
    pub pg_open: &'static str,
    pub pg_open_title: &'static str,
    pub pg_save: &'static str,
    pub pg_save_title: &'static str,
    pub pg_offline: &'static str,
    /// The Share button, its tooltip, the label of the field that holds the link, the
    /// buttons of that box; the notice of a program opened from a link, its button.
    pub pg_share: &'static str,
    pub pg_share_title: &'static str,
    pub pg_share_link: &'static str,
    pub pg_share_copy: &'static str,
    pub pg_share_close: &'static str,
    pub pg_shared_note: &'static str,
    pub pg_shared_back: &'static str,
    // Playground: what `playground.js` writes while it runs (`{ms}` and `{n}` are
    // replaced)
    pub pg_loading: &'static str,
    pub pg_load_failed: &'static str,
    pub pg_running: &'static str,
    pub pg_stopped: &'static str,
    pub pg_ran_in: &'static str,
    pub pg_failed_in: &'static str,
    pub pg_nothing: &'static str,
    pub pg_too_deep: &'static str,
    /// The number of lines of the editor: one line, several.
    pub pg_lines: [&'static str; 2],
    pub pg_explain: &'static str,
    pub pg_custom: &'static str,
    pub pg_formatted: &'static str,
    pub pg_already_formatted: &'static str,
    pub pg_not_formatted: &'static str,
    /// Sharing: the link copied, shared, to copy by hand, too long (`{n}`, `{max}`),
    /// and a link that cannot be read.
    pub pg_share_copied: &'static str,
    pub pg_share_shared: &'static str,
    pub pg_share_select: &'static str,
    pub pg_share_too_long: &'static str,
    pub pg_share_broken: &'static str,
    // Examples
    pub ex_title: &'static str,
    pub ex_lead: &'static str,
    pub ex_output: &'static str,
    pub ex_tests: &'static str,
    pub ex_open: &'static str,
    pub ex_jump: &'static str,
    // Learn
    pub learn_title: &'static str,
    pub learn_lead: &'static str,
    pub learn_try: &'static str,
    pub tour: [Step; 8],
    pub learn_names: &'static str,
    pub learn_next_title: &'static str,
    pub learn_next: [Item; 3],
    // Teach
    pub teach_lead: &'static str,
    pub teach_toc: &'static str,
    /// Above the English guide, in a language that has no guide of its own (`None`:
    /// the language has one, see `teach.rs`).
    pub teach_note: Option<&'static str>,
    // Not found
    pub nf_title: &'static str,
    pub nf_text: &'static str,
    pub nf_home: &'static str,
    // The installed app: manifest, install button, updates, offline page
    pub app_description: &'static str,
    pub app_install: &'static str,
    pub app_install_title: &'static str,
    pub app_update: &'static str,
    pub app_reload: &'static str,
    pub app_later: &'static str,
    pub app_ios: &'static str,
    pub app_close: &'static str,
    /// The labels of the screenshots of the manifest: the home page, the playground.
    pub app_shots: [&'static str; 2],
    pub install_app: &'static str,
    pub offline_title: &'static str,
    pub offline_text: &'static str,
    pub offline_retry: &'static str,
    // The color themes (`themes.rs`): the picker of the menu
    pub theme_label: &'static str,
    pub theme_title: &'static str,
    /// In the order of `themes::THEMES`: Auto, Light, Dark, High contrast, Sepia, Tempo.
    pub theme_names: [&'static str; 6],
}
