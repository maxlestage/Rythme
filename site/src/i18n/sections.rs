//! The texts of the five sections added to the site: *Install & tools*, *Reference*,
//! *Packages*, *Rythme at work* and *Exercises*, with their names in the menu. One
//! [`Sections`] constant per language, in `sections/<code>.rs`; what the pages list
//! (the built-in functions, the codes, the exercises) comes from the interpreter itself,
//! so only the words around it are written here.

mod de;
mod en;
mod es;
mod fr;
mod it;
mod ja;
mod pt;
mod zh;

pub use de::DE;
pub use en::EN;
pub use es::ES;
pub use fr::FR;
pub use it::IT;
pub use ja::JA;
pub use pt::PT;
pub use zh::ZH;

use super::Item;

pub struct Sections {
    // The menu: the five pages, and the button that opens them on a computer.
    pub nav_install: &'static str,
    pub nav_reference: &'static str,
    pub nav_packages: &'static str,
    pub nav_work: &'static str,
    pub nav_exercises: &'static str,
    pub nav_more: &'static str,
    /// The description of the AI page for search engines (its lead is longer than 160
    /// characters).
    pub ai_description: &'static str,

    // Install & tools (`/{lang}/install/`)
    pub inst_title: &'static str,
    pub inst_lead: &'static str,
    pub inst_get_title: &'static str,
    pub inst_get_lead: &'static str,
    /// In the order of `sections::INSTALL_WAYS`: the script for Linux and macOS, the one
    /// for Windows, by hand, Homebrew and winget, Cargo, a clone.
    pub inst_ways: [Item; 6],
    pub inst_keep_title: &'static str,
    pub inst_keep_lead: &'static str,
    /// What each command of `sections::KEEP` does.
    pub inst_keep: [&'static str; 6],
    pub inst_editors_title: &'static str,
    pub inst_editors_lead: &'static str,
    /// VS Code, Zed, Helix, Tree-sitter, any other editor.
    pub inst_editors: [Item; 5],
    pub inst_lsp_title: &'static str,
    pub inst_lsp_lead: &'static str,
    /// What the language server gives an editor.
    pub inst_lsp: [&'static str; 6],
    pub inst_debug_title: &'static str,
    pub inst_debug_lead: &'static str,
    /// What each command of `sections::DEBUG` does.
    pub inst_debug: [&'static str; 6],
    pub inst_ai_title: &'static str,
    pub inst_ai_lead: &'static str,

    // Reference (`/{lang}/reference/`)
    pub ref_title: &'static str,
    pub ref_lead: &'static str,
    pub ref_fns_title: &'static str,
    /// `{n}`: the number of built-in functions.
    pub ref_fns_lead: &'static str,
    pub ref_filter_label: &'static str,
    pub ref_filter_placeholder: &'static str,
    /// `{n}` functions shown of `{total}`.
    pub ref_filter_count: &'static str,
    pub ref_filter_none: &'static str,
    /// The groups of the functions, in the order of `rythme::builtins::Category::ALL`.
    pub ref_categories: [&'static str; 12],
    pub ref_example: &'static str,
    pub ref_jump: &'static str,
    pub ref_codes_title: &'static str,
    /// `{n}`: the number of codes.
    pub ref_codes_lead: &'static str,
    pub ref_errors: &'static str,
    pub ref_warnings: &'static str,
    /// `{version}`: the version of Rythme the page was made from.
    pub ref_generated: &'static str,

    // Packages (`/{lang}/packages/`)
    pub pkg_title: &'static str,
    pub pkg_lead: &'static str,
    pub pkg_find_title: &'static str,
    pub pkg_find_lead: &'static str,
    /// What each command of `sections::PACKAGE_COMMANDS` does.
    pub pkg_find: [&'static str; 6],
    pub pkg_toml_title: &'static str,
    pub pkg_toml_lead: &'static str,
    /// The meaning of each range of `sections::RANGES`.
    pub pkg_ranges: [&'static str; 5],
    pub pkg_lock: &'static str,
    pub pkg_publish_title: &'static str,
    pub pkg_publish_lead: &'static str,
    /// The steps of publishing a version.
    pub pkg_publish: [Item; 5],
    pub pkg_index_title: &'static str,
    pub pkg_index_lead: &'static str,
    /// The default index, another index, a folder shared by a school or a team.
    pub pkg_index: [&'static str; 3],
    pub pkg_safety_title: &'static str,
    pub pkg_safety: [&'static str; 5],

    // Rythme at work (`/{lang}/work/`)
    pub work_title: &'static str,
    pub work_lead: &'static str,
    pub work_data_title: &'static str,
    pub work_data_lead: &'static str,
    /// The programs of `sections::WORK_SAMPLES`: CSV, JSON, text, a command-line tool.
    pub work_samples: [Item; 4],
    pub work_ci_title: &'static str,
    pub work_ci_lead: &'static str,
    /// What each command of `sections::CI_COMMANDS` does.
    pub work_ci: [&'static str; 4],
    pub work_ci_file: &'static str,
    pub work_json_title: &'static str,
    pub work_json_lead: &'static str,
    /// What each command of `sections::JSON_COMMANDS` does.
    pub work_json: [&'static str; 3],
    pub work_stable_title: &'static str,
    pub work_stable_lead: &'static str,
    pub work_stable: [&'static str; 3],
    pub work_speed_title: &'static str,
    pub work_speed_lead: &'static str,
    pub work_speed: [&'static str; 5],

    // Exercises (`/{lang}/exercises/`)
    pub exo_title: &'static str,
    pub exo_lead: &'static str,
    pub exo_how_title: &'static str,
    /// What each command of `sections::EXERCISE_COMMANDS` does.
    pub exo_how: [&'static str; 3],
    /// The three levels: ★☆☆, ★★☆, ★★★.
    pub exo_levels: [Item; 3],
    pub exo_try: &'static str,
    pub exo_statement: &'static str,
    pub exo_note: &'static str,
}
