//! Every message in every language: the catalogs of `src/i18n/` against the
//! texts of the program.
//!
//! The texts to translate are the English texts of `t!("English", "Français")`
//! and `pick("English", "Français")` in the sources, and the fixed texts of the
//! help (`rythme::help::fixed_texts`). For each catalog, this test prints how
//! many are translated and fails when a complete language misses one, when a
//! catalog keeps a text the program no longer has (stale: the English text
//! changed), or when a translation does not have the `{names}` of its text.
//!
//! `RYTHME_I18N_MISSING=missing.rs cargo test --test i18n` writes the missing
//! entries of each catalog, with the French text as a guide, ready to fill in.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

mod common;

use rythme::lang::{self, Lang, LANGUAGES};

/// The languages whose catalog must have every text. A new language can
/// start partial (shown in English where missing) and join this list once done.
const COMPLETE: &[&str] = &["es"];

/// One text of the program: English, French, and where it is.
struct Text {
    en: String,
    fr: String,
    at: String,
    /// A `format!` text (from `t!`), whose `{names}` must be kept.
    format: bool,
}

#[test]
fn every_catalog_matches_the_texts_of_the_program() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut texts: Vec<Text> = Vec::new();
    for dir in ["src", "playground/src"] {
        for file in rust_files(&root.join(dir)) {
            let source = std::fs::read_to_string(&file).unwrap();
            let name = file.strip_prefix(root).unwrap().display().to_string().replace('\\', "/");
            texts.extend(calls(&source, &name));
        }
    }
    assert!(texts.len() > 1000, "only {} texts found in the sources: is the scanner broken?", texts.len());
    for fixed in rythme::help::fixed_texts() {
        texts.push(Text { en: fixed.to_string(), fr: String::new(), at: "rythme::help::fixed_texts".into(), format: false });
    }
    // Each English text once, in the order of the sources.
    let mut wanted: HashMap<&str, &Text> = HashMap::new();
    let mut order: Vec<&Text> = Vec::new();
    for t in &texts {
        if !wanted.contains_key(t.en.as_str()) {
            wanted.insert(&t.en, t);
            order.push(t);
        }
    }

    let mut problems = Vec::new();
    let mut report = vec![format!("{} texts to translate ({} uses)", wanted.len(), texts.len())];
    let mut skeleton = String::new();
    for lang in Lang::all() {
        let language = lang.language();
        if lang == Lang::En || lang == Lang::Fr {
            report.push(format!("  {}  {:<10} {}/{} (written in the sources)", language.code, language.name, wanted.len(), wanted.len()));
            continue;
        }
        let mut seen: HashMap<&str, usize> = HashMap::new();
        for (en, _) in language.texts {
            *seen.entry(en).or_default() += 1;
        }
        for (en, n) in &seen {
            if *n > 1 {
                problems.push(format!("{}: translated {n} times: {en:?}", language.code));
            }
        }
        for (en, tr) in language.texts {
            match wanted.get(en) {
                None => problems
                    .push(format!("{}: stale entry, no longer in the program (did its English text change?): {en:?}", language.code)),
                Some(t) if t.format && !tr.is_empty() && !same_values(en, tr) => problems.push(format!(
                    "{}: the translation must take its {{names}} from {en:?}, and as many {{}} ({}): {tr:?}",
                    language.code, t.at
                )),
                Some(_) => {}
            }
        }
        let missing: Vec<&&Text> = order.iter().filter(|t| lang::translation(lang, &t.en).is_none()).collect();
        let done = wanted.len() - missing.len();
        report.push(format!(
            "  {}  {:<10} {done}/{} ({:.1} %), {} missing",
            language.code,
            language.name,
            wanted.len(),
            100.0 * done as f64 / wanted.len() as f64,
            missing.len()
        ));
        if COMPLETE.contains(&language.code) {
            for t in missing.iter().take(20) {
                problems.push(format!("{}: missing translation of {:?} ({})", language.code, t.en, t.at));
            }
        }
        if !missing.is_empty() {
            skeleton.push_str(&format!("// ----- {}: {} missing\n", language.code, missing.len()));
            let mut file = "";
            for t in &missing {
                let here = t.at.split(':').next().unwrap_or("");
                if here != file {
                    file = here;
                    skeleton.push_str(&format!("\n        // {file}\n"));
                }
                if !t.fr.is_empty() {
                    skeleton.push_str(&format!("        // fr: {}\n", literal(&t.fr)));
                }
                skeleton.push_str(&format!("        ({}, \"\"),\n", literal(&t.en)));
            }
        }
    }
    println!("{}", report.join("\n"));
    if let Some(path) = std::env::var_os("RYTHME_I18N_MISSING") {
        std::fs::write(&path, &skeleton).unwrap();
        println!("missing entries written to {}", Path::new(&path).display());
    }
    assert!(problems.is_empty(), "{}\n\n{}", report.join("\n"), problems.join("\n"));
}

/// Messages, help and explanations in Spanish, chosen by `--lang`, by
/// `RYTHME_LANG` or by the locale; English where no language is known.
#[test]
fn spanish_messages() {
    let r = common::rythme_lang("es", &["-e", "x = [1, 2]\nprint(lenght(x))"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E003]: nombre desconocido `lenght`"), "{}", r.stderr);
    assert!(r.stderr.contains("= ayuda: en Rythme, esto se llama `len`"), "{}", r.stderr);
    assert!(r.stderr.contains("más sobre este error: `rythme explain E003`"), "{}", r.stderr);

    let r = common::rythme_lang("es", &["-e", "print(1 / 0)"]);
    assert!(r.stderr.contains("error[E008]: división entre cero"), "{}", r.stderr);
    let r = common::rythme_lang("es", &["-e", "fn sumar(a, b) = a + b\nprint(sumar(1))"]);
    assert!(r.stderr.contains("`sumar` espera 2 argumentos, pero recibió 1"), "{}", r.stderr);
    let r = common::rythme_lang("es", &["-e", "print(parse_date(\"2023-02-30\"))"]);
    assert!(r.stdout.contains("el día debe ir de 1 a 28 en febrero de 2023, no 30"), "{}", r.stdout);
    let r = common::rythme_lang("es", &["-e", "print(len(42))"]);
    assert!(r.stderr.contains("`len` espera"), "{}", r.stderr);

    // The help, the explanations and the documentation of the functions.
    let r = common::rythme_lang("es", &["help"]);
    assert!(r.stdout.starts_with(&format!("Rythme {} — ayuda", rythme::VERSION)), "{}", r.stdout);
    let r = common::rythme_lang("es", &["help", "listas"]);
    assert!(r.stdout.starts_with("LISTAS"), "{}", r.stdout);
    let r = common::rythme_lang("es", &["explain", "E001"]);
    assert!(r.stdout.starts_with("E001 — Error de sintaxis"), "{}", r.stdout);
    let r = common::rythme_lang("es", &["help", "map"]);
    assert!(r.stdout.contains("Aplica la función f a cada elemento"), "{}", r.stdout);

    // `--lang es` wins over the environment; the locale is enough.
    let r = common::rythme_lang("en", &["--lang", "es", "check", "missing.ry"]);
    assert!(r.stderr.contains("archivo `missing.ry` no encontrado"), "{}", r.stderr);
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["-e", "print(1 / 0)"])
        .env_remove("RYTHME_LANG")
        .env_remove("LC_ALL")
        .env_remove("LC_MESSAGES")
        .env("LANG", "es_ES.UTF-8")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&out.stderr).contains("división entre cero"));
    let r = common::rythme(&["--lang", "de", "help"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("unknown language `de`: write `--lang` and one of en, fr, es"), "{}", r.stderr);
}

/// The language is found from its code, from a locale, and is listed.
#[test]
fn languages_are_chosen_by_code_or_locale() {
    let es = lang::parse("es").expect("es");
    assert_eq!(es.code(), "es");
    assert_eq!(lang::parse("es_ES.UTF-8"), Some(es));
    assert_eq!(lang::parse("ES"), Some(es));
    assert_eq!(lang::parse("español"), Some(es));
    assert_eq!(lang::parse("fr_CA"), Some(Lang::Fr));
    assert_eq!(lang::parse("de_DE"), None);
    assert!(lang::codes().contains("es"));
    let codes: BTreeSet<&str> = LANGUAGES.iter().map(|l| l.code).collect();
    assert_eq!(codes.len(), LANGUAGES.len(), "two languages with the same code");
}

/// The scanner finds the texts of `t!` and `pick`, not those in comments or strings.
#[test]
fn the_scanner_reads_rust_texts() {
    let source = r##"
        // t!("in a comment", "dans un commentaire")
        let a = "t!(\"in a string\", \"x\")";
        let b = 'x'; let c: &'static str = "";
        let d = t!("one {x}\n", "un {x}\n", x = 1);
        let e = crate::lang::pick(r#"two "quoted""#, "deux");
    "##;
    let found: Vec<String> = calls(source, "test").into_iter().map(|t| t.en).collect();
    assert_eq!(found, ["one {x}\n", "two \"quoted\""]);
    assert_eq!(placeholders("a {x} {{b}} {:>4} {y:?}"), [":>4", "x", "y:?"]);
}

/// A Rust literal for `text`: a raw one when it has several lines.
fn literal(text: &str) -> String {
    if !text.contains('\n') || text.contains('\r') {
        return format!("{text:?}");
    }
    let mut hashes = String::from("#");
    while text.contains(&format!("\"{hashes}")) {
        hashes.push('#');
    }
    format!("r{hashes}\"{text}\"{hashes}")
}

fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            // The catalogs hold translations, not texts of the program.
            if path.file_name().is_some_and(|n| n != "i18n") {
                out.extend(rust_files(&path));
            }
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out
}

/// Whether a translation shows the values of its English text: each `{name}`
/// of the translation is one of the English text (a translation may leave out
/// a word given by name, like the verb of `{got} {verb}`), and both have the
/// same `{}` without a name, which the compiler wants used.
fn same_values(en: &str, tr: &str) -> bool {
    let (en, tr) = (placeholders(en), placeholders(tr));
    let unnamed = |p: &Vec<String>| p.iter().filter(|p| p.is_empty() || p.starts_with(':')).cloned().collect::<Vec<_>>();
    unnamed(&en) == unnamed(&tr) && tr.iter().all(|p| en.contains(p))
}

/// The `{...}` of a format text, without `{{` and `}}`, sorted.
fn placeholders(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
            }
            '{' => out.push(chars.by_ref().take_while(|c| *c != '}').collect()),
            _ => {}
        }
    }
    out.sort();
    out
}

#[derive(Debug, PartialEq)]
enum Token {
    Ident(String),
    Punct(char),
    Str(String),
}

/// The calls `t!("en", "fr"...)` and `pick("en", "fr")` of a Rust file.
fn calls(source: &str, file: &str) -> Vec<Text> {
    let tokens = tokenize(source);
    let mut out = Vec::new();
    for (i, (tok, line)) in tokens.iter().enumerate() {
        let next = |k: usize| tokens.get(i + k).map(|t| &t.0);
        let (macro_call, args) = match tok {
            Token::Ident(name) if name == "t" && next(1) == Some(&Token::Punct('!')) && next(2) == Some(&Token::Punct('(')) => {
                (true, i + 3)
            }
            Token::Ident(name) if name == "pick" && next(1) == Some(&Token::Punct('(')) => (false, i + 2),
            _ => continue,
        };
        if let (Some((Token::Str(en), _)), Some((Token::Punct(','), _)), Some((Token::Str(fr), _))) =
            (tokens.get(args), tokens.get(args + 1), tokens.get(args + 2))
        {
            out.push(Text { en: en.clone(), fr: fr.clone(), at: format!("{file}:{line}"), format: macro_call });
        } else if macro_call {
            panic!("{file}:{line}: t! must start with two texts");
        }
    }
    out
}

/// A small Rust tokenizer: names, punctuation and the value of string literals,
/// with their line; comments and characters are skipped.
fn tokenize(source: &str) -> Vec<(Token, usize)> {
    let chars: Vec<char> = source.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let mut line = 1;
    let count = |from: usize, to: usize, chars: &[char]| chars[from..to].iter().filter(|c| **c == '\n').count();
    while i < chars.len() {
        let c = chars[i];
        let start = i;
        let at = line;
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
        } else if c == '/' && chars.get(i + 1) == Some(&'*') {
            let mut depth = 0;
            while i < chars.len() {
                if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                    depth += 1;
                    i += 2;
                } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        } else if c == '"' {
            let (value, end) = quoted(&chars, i + 1);
            out.push((Token::Str(value), at));
            i = end;
        } else if (c == 'r' || (c == 'b' && chars.get(i + 1) == Some(&'r')))
            && {
                let j = if c == 'b' { i + 2 } else { i + 1 };
                let mut k = j;
                while chars.get(k) == Some(&'#') {
                    k += 1;
                }
                chars.get(k) == Some(&'"')
            }
            && (i == 0 || !is_ident(chars[i - 1]))
        {
            let mut j = if c == 'b' { i + 2 } else { i + 1 };
            let mut hashes = 0;
            while chars[j] == '#' {
                hashes += 1;
                j += 1;
            }
            j += 1;
            let body = j;
            loop {
                if chars[j] == '"' && (0..hashes).all(|h| chars.get(j + 1 + h) == Some(&'#')) {
                    break;
                }
                j += 1;
            }
            out.push((Token::Str(chars[body..j].iter().collect()), at));
            i = j + 1 + hashes;
        } else if c == 'b' && chars.get(i + 1) == Some(&'"') {
            let (value, end) = quoted(&chars, i + 2);
            out.push((Token::Str(value), at));
            i = end;
        } else if c == '\'' {
            // A character ('x', '\n', '\u{1F600}') or a lifetime ('a).
            if chars.get(i + 1) == Some(&'\\') {
                i += 2;
                while chars[i] != '\'' {
                    i += 1;
                }
                i += 1;
            } else if chars.get(i + 2) == Some(&'\'') {
                i += 3;
            } else {
                i += 1;
                while i < chars.len() && is_ident(chars[i]) {
                    i += 1;
                }
            }
        } else if is_ident(c) {
            while i < chars.len() && is_ident(chars[i]) {
                i += 1;
            }
            out.push((Token::Ident(chars[start..i].iter().collect()), at));
        } else {
            if !c.is_whitespace() {
                out.push((Token::Punct(c), at));
            }
            i += 1;
        }
        line += count(start, i, &chars);
    }
    out
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The value of a "..." literal whose text starts at `i`, and the index after it.
fn quoted(chars: &[char], mut i: usize) -> (String, usize) {
    let mut value = String::new();
    while chars[i] != '"' {
        if chars[i] != '\\' {
            value.push(chars[i]);
            i += 1;
            continue;
        }
        i += 1;
        match chars[i] {
            'n' => value.push('\n'),
            'r' => value.push('\r'),
            't' => value.push('\t'),
            '0' => value.push('\0'),
            'x' => {
                let hex: String = chars[i + 1..i + 3].iter().collect();
                value.push(char::from(u8::from_str_radix(&hex, 16).unwrap()));
                i += 2;
            }
            'u' => {
                let end = i + chars[i..].iter().position(|c| *c == '}').unwrap();
                let hex: String = chars[i + 2..end].iter().filter(|c| **c != '_').collect();
                value.push(char::from_u32(u32::from_str_radix(&hex, 16).unwrap()).unwrap());
                i = end;
            }
            '\n' | '\r' => {
                while chars[i + 1].is_whitespace() {
                    i += 1;
                }
            }
            other => value.push(other),
        }
        i += 1;
    }
    (value, i + 1)
}
