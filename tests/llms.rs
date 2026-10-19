//! `docs/llms.txt` and `docs/llms-full.txt`, the language for AI assistants:
//! they are what `rythme help llms.txt` and `rythme help llms` print (built
//! from the sources, so they never drift), and every example in them runs
//! and prints what is written under it.
//!
//! After a change of the language, write them again:
//!   cargo run -- help llms > docs/llms-full.txt
//!   cargo run -- help llms.txt > docs/llms.txt

use std::path::Path;

mod common;
use common::{rythme, rythme_lang};

fn doc(name: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("docs").join(name)).unwrap_or_default().replace("\r\n", "\n")
}

#[test]
fn the_files_of_docs_are_up_to_date() {
    let full = rythme(&["help", "llms"]);
    assert_eq!(full.code, 0);
    assert!(
        full.stdout == doc("llms-full.txt"),
        "docs/llms-full.txt is stale: write it again with `cargo run -- help llms > docs/llms-full.txt`"
    );
    let index = rythme(&["help", "llms.txt"]);
    assert!(index.stdout == doc("llms.txt"), "docs/llms.txt is stale: write it again with `cargo run -- help llms.txt > docs/llms.txt`");
    // The same text in every language of the messages, and with `--llms`.
    assert_eq!(rythme_lang("fr", &["help", "llms"]).stdout, full.stdout);
    assert_eq!(rythme_lang("zh", &["help", "--llms"]).stdout, full.stdout);
}

#[test]
fn the_index_follows_the_llms_txt_format() {
    let index = doc("llms.txt");
    let mut lines = index.lines();
    assert_eq!(lines.next(), Some("# Rythme"), "an H1 title first");
    assert_eq!(lines.next(), Some(""));
    assert!(lines.next().unwrap().starts_with("> "), "then a summary in a quote");
    assert!(index.contains("\n## Docs\n") && index.contains("\n## Optional\n"));
    for line in index.lines().filter(|l| l.starts_with("- [")) {
        let (_, rest) = line.split_once("](").expect("a link");
        let url = rest.split(')').next().unwrap();
        assert!(url.starts_with("https://github.com/maxlestage/Rythme/"), "{url}");
        // The links point to files of the repository.
        let path = url.trim_start_matches("https://github.com/maxlestage/Rythme/blob/master/").split('#').next().unwrap();
        assert!(Path::new(env!("CARGO_MANIFEST_DIR")).join(path).exists() || path == "AGENTS.md", "{path} does not exist");
    }
}

#[test]
fn the_full_text_names_everything() {
    let full = doc("llms-full.txt");
    for b in rythme::builtins::BUILTINS {
        assert!(full.contains(&format!("- `{}`: ", b.sig)), "`{}` is missing", b.name);
    }
    for a in rythme::builtins::ALIASES {
        assert!(full.contains(&format!("`{}` = `{}`", a.name, a.of)), "`{}` is missing", a.name);
    }
    for code in rythme::help::CODES {
        assert!(full.contains(&format!("- {code}: ")), "{code} is missing");
    }
    for word in rythme::help::keywords() {
        assert!(full.contains(&format!("- `{word}`: ")), "keyword `{word}` is missing");
    }
}

#[test]
fn every_example_prints_what_is_written() {
    let full = doc("llms-full.txt");
    let dir = std::env::temp_dir().join(format!("rythme-llms-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let blocks = full.split("```rythme\n").skip(1);
    let mut n = 0;
    for block in blocks {
        let (code, after) = block.split_once("```\n").expect("a closed block");
        let expected = after.trim_start().strip_prefix("```text\n").map(|t| t.split_once("```\n").expect("a closed block").0);
        let file = dir.join(format!("example{n}.ry"));
        std::fs::write(&file, code).unwrap();
        let path = file.to_string_lossy().to_string();
        let check = rythme(&["check", &path]);
        assert_eq!(check.code, 0, "example:\n{code}\n{}", check.stderr);
        assert!(!check.stderr.contains("warning"), "example:\n{code}\n{}", check.stderr);
        let run = rythme(&[&path]);
        assert_eq!(run.code, 0, "example:\n{code}\n{}", run.stderr);
        if let Some(expected) = expected {
            assert_eq!(run.stdout, expected, "example:\n{code}");
        }
        n += 1;
    }
    assert!(n >= 8, "only {n} examples");
    let _ = std::fs::remove_dir_all(&dir);
}
