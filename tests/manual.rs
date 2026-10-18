//! The reference manual shows real output: every ```rythme block followed by a
//! ```text block is run, and must print exactly that text (docs/reference.md in
//! English, docs/reference.fr.md in French).

use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

struct Block {
    kind: String,
    line: usize,
    text: String,
    /// The prose between the previous block and this one.
    before: String,
}

fn blocks(manual: &str) -> Vec<Block> {
    let lines: Vec<&str> = manual.lines().collect();
    let mut out = Vec::new();
    let mut prose = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let Some(kind) = lines[i].strip_prefix("```").filter(|k| k.chars().all(|c| c.is_alphanumeric() || c == '_')) {
            let start = i + 1;
            let mut j = start;
            while lines[j] != "```" {
                j += 1;
            }
            out.push(Block { kind: kind.to_string(), line: start, text: lines[start..j].join("\n"), before: prose.join(" ") });
            prose.clear();
            i = j + 1;
            continue;
        }
        if !lines[i].trim().is_empty() {
            prose.push(lines[i].trim());
        }
        i += 1;
    }
    out
}

/// The name of the file a sample is shown in (`--> name.ry:1:2`), else `sample.ry`.
fn file_name(expected: &str) -> String {
    for word in expected.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-' || c == '.')) {
        if let Some(stem) = word.trim_end_matches('.').strip_suffix(".ry").filter(|s| !s.is_empty() && !s.starts_with('.')) {
            return format!("{stem}.ry");
        }
    }
    "sample.ry".to_string()
}

/// The number of calls in progress (E014) depends on the stack of the machine:
/// on those lines, every number counts as the same.
fn normalize(text: &str) -> String {
    let lines: Vec<String> = text
        .trim_end_matches('\n')
        .lines()
        .map(|line| {
            let counts_calls = ["calls in progress", "more calls", "appels en cours", "autres appels"].iter().any(|w| line.contains(w));
            // The time a test run took (`4 test(s): 4 passed, 0 failed (3 ms)`) changes from run to run.
            let timed = line.ends_with(" ms)") || line.ends_with(" s)");
            if !counts_calls && !timed {
                return line.to_string();
            }
            let mut out = String::new();
            let mut in_number = false;
            for c in line.chars() {
                if c.is_ascii_digit() {
                    if !in_number {
                        out.push('N');
                    }
                    in_number = true;
                } else {
                    out.push(c);
                    in_number = false;
                }
            }
            out
        })
        .collect();
    lines.join("\n")
}

fn run(lang: &str, command: Option<&str>, name: &str, code: &str) -> String {
    let dir = std::env::temp_dir().join(format!("rythme-manual-{}-{lang}-{}", std::process::id(), name.trim_end_matches(".ry")));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(name), format!("{code}\n")).unwrap();
    // One file for both streams, so that they stay in the order they were written.
    let out_path: PathBuf = dir.join("output.txt");
    let out = File::create(&out_path).unwrap();
    let mut args = vec!["--lang", lang];
    args.extend(command);
    args.push(name);
    Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(&args)
        .current_dir(&dir)
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .stdout(out.try_clone().unwrap())
        .stderr(out)
        .status()
        .expect("run rythme");
    let printed = std::fs::read_to_string(&out_path).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    printed.replace("\r\n", "\n")
}

fn check_manual(path: &str, lang: &str) {
    let manual = std::fs::read_to_string(path).unwrap();
    let blocks = blocks(&manual);
    let mut checked = 0;
    let mut failures = Vec::new();
    for pair in blocks.windows(2) {
        let (sample, shown) = (&pair[0], &pair[1]);
        if sample.kind != "rythme" || shown.kind != "text" {
            continue;
        }
        // A module file shown on its own (`# geometry.ry` on the first line).
        let first = sample.text.lines().next().unwrap_or("").trim();
        if first.starts_with("# ") && first.ends_with(".ry") {
            continue;
        }
        let command = if shown.before.contains("rythme test") {
            Some("test")
        } else if shown.before.contains("rythme check") {
            Some("check")
        } else if shown.before.contains("rythme doc") {
            Some("doc")
        } else {
            None
        };
        // Another command (modules, arguments) or a transcript: checked by hand.
        let other_command = shown.before.split('`').skip(1).step_by(2).any(|code| code.starts_with("rythme "));
        if (other_command && command.is_none()) || shown.text.starts_with("Rythme ") || shown.text.starts_with("$ ") {
            continue;
        }
        let name = file_name(&shown.text);
        let got = run(lang, command, &name, &sample.text);
        checked += 1;
        if normalize(&got) != normalize(&shown.text) {
            failures.push(format!("{path}:{}: ({name})\n--- shown\n{}\n--- printed\n{got}", sample.line, shown.text));
        }
    }
    assert!(checked >= 30, "{path}: only {checked} samples found");
    assert!(failures.is_empty(), "{} sample(s) of {path} print something else:\n\n{}", failures.len(), failures.join("\n\n"));
}

#[test]
fn the_english_manual_shows_real_output() {
    check_manual(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/reference.md"), "en");
}

#[test]
fn the_french_manual_shows_real_output() {
    check_manual(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/reference.fr.md"), "fr");
}
