//! The programs of `examples/`, built into the site with what they print (the `.out`
//! files that `cargo test` checks at the root of the repository).

use crate::i18n::Lang;

pub struct Example {
    /// The file name without `.ry`, also the anchor on the examples page.
    pub name: &'static str,
    pub code: &'static str,
    pub output: &'static str,
    /// What the example shows, `[fr, en]`.
    about: [&'static str; 2],
}

impl Example {
    pub fn about(&self, lang: Lang) -> &'static str {
        lang.pick(self.about)
    }

    /// Run with `rythme test` in the playground.
    pub fn is_test(&self) -> bool {
        self.name == "testing"
    }
}

macro_rules! example {
    ($name:literal, $fr:literal, $en:literal) => {
        Example {
            name: $name,
            code: include_str!(concat!("../../examples/", $name, ".ry")),
            output: include_str!(concat!("../../examples/", $name, ".out")),
            about: [$fr, $en],
        }
    };
}

/// In the order of the README.
pub const EXAMPLES: &[Example] = &[
    example!("hello", "le premier programme classique", "the classic first program"),
    example!("fizzbuzz", "match sur une liste, intervalles, each", "match on a list, ranges, each"),
    example!("fibonacci", "définitions par cas, appels terminaux, documentation ##", "definitions by case, tail calls, ## docs"),
    example!("shapes", "vos propres types, gardes, sort_by", "your own types, guards, sort_by"),
    example!("quicksort", "motifs de listes, filter, + sur les listes", "list patterns, filter, + on lists"),
    example!("words", "traitement de texte, group_by, enregistrements", "text processing, group_by, records"),
    example!(
        "cipher",
        "le chiffre de César : char_code, char, compter les lettres",
        "the Caesar cipher: char_code, char, counting letters"
    ),
    example!("results", "Option, Result et ?", "Option, Result and ?"),
    example!("students", "enregistrements, mise à jour, regroupements, moyennes", "records, record update, grouping, averages"),
    example!(
        "library",
        "le style Ruby : blocs do |x| ... end, enchaînements de méthodes, noms en ?, unless, symboles",
        "the Ruby style: do |x| ... end blocks, method chains, names ending with ?, unless, symbols"
    ),
    example!(
        "sales",
        "un rapport : scan, colonnes alignées avec pad_left, windows, chunks, JSON",
        "a report: scan, columns lined up with pad_left, windows, chunks, JSON"
    ),
    example!(
        "calendar",
        "des dates : la page d'un mois, des textes ISO 8601, days_between, add_days",
        "dates: the page of a month, ISO 8601 texts, days_between, add_days"
    ),
    example!(
        "logs",
        "lire un journal avec des regex : named_captures, matches, find_all, replace_all, format_number",
        "reading a log with regexes: named_captures, matches, find_all, replace_all, format_number"
    ),
    example!("tree", "un type récursif : arbre binaire de recherche", "a recursive type: binary search tree"),
    example!("primes", "un crible récursif, all, and", "a recursive sieve, all, and"),
    example!("testing", "les blocs test", "test blocks"),
    example!("bonjour", "des noms avec des accents", "names with accents"),
];

pub fn find(name: &str) -> Option<&'static Example> {
    EXAMPLES.iter().find(|e| e.name == name)
}

/// `{"hello": "…code…", …}`: the menu of the playground.
pub fn json() -> String {
    let mut out = String::from("{");
    for (i, e) in EXAMPLES.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&json_string(e.name));
        out.push(':');
        out.push_str(&json_string(e.code));
    }
    out.push('}');
    out
}

/// A JSON string literal.
pub fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            // `<` too, so that the JSON can never close a `<script>` element.
            c if (c as u32) < 0x20 || c == '<' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
