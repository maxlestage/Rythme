//! The `rythme` command: help, explain, check, test.

mod common;

use common::rythme;

#[test]
fn help_overview_and_topics() {
    let r = rythme(&["help"]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("rythme check"));
    for topic in [
        "start",
        "syntax",
        "functions",
        "lists",
        "text",
        "records",
        "types",
        "match",
        "errors",
        "functional",
        "tests",
        "exercises",
        "modules",
        "dates",
        "regex",
        "repl",
        "doc",
        "editors",
        "builtins",
    ] {
        let r = rythme(&["help", topic]);
        assert_eq!(r.code, 0, "topic {topic}");
        assert!(r.stdout.len() > 100, "topic {topic} is too short");
    }
    let r = rythme(&["help", "map"]);
    assert!(r.stdout.contains("map(list, f)") && r.stdout.contains("Example"));
    let r = rythme(&["help", "reduce"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("`fold`"));
}

#[test]
fn every_builtin_is_documented_in_both_languages() {
    for b in rythme::builtins::BUILTINS {
        assert!(!b.en.is_empty() && !b.fr.is_empty() && !b.example.is_empty(), "{}", b.name);
        assert!(b.sig.starts_with(b.name), "{}", b.name);
    }
}

#[test]
fn every_error_code_is_explained() {
    for code in rythme::help::CODES {
        let r = rythme(&["explain", code]);
        assert_eq!(r.code, 0, "{code}");
        assert!(r.stdout.starts_with(code), "{code}");
        let r = common::rythme_lang("fr", &["explain", code]);
        assert!(r.stdout.starts_with(code), "{code} (fr)");
    }
}

#[test]
fn every_error_code_used_in_the_sources_is_explained() {
    // The sources, folders included.
    let mut todo = vec![std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src")];
    while let Some(path) = todo.pop() {
        if path.is_dir() {
            todo.extend(std::fs::read_dir(&path).unwrap().map(|e| e.unwrap().path()));
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        for (i, _) in text.match_indices("(\"E0").chain(text.match_indices("(\"W0")) {
            let code = &text[i + 2..i + 6];
            assert!(rythme::help::CODES.contains(&code), "{code} has no explanation");
        }
    }
}

#[test]
fn check_command() {
    let dir = std::env::temp_dir().join(format!("rythme-check-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let good = dir.join("good.ry");
    std::fs::write(&good, "fn f(x) = x\nprint(f(1))\n").unwrap();
    let bad = dir.join("bad.ry");
    std::fs::write(&bad, "print(f(1))\nprint(g(2))\n").unwrap();
    let r = rythme(&["check", good.to_str().unwrap()]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("no problems found"));
    assert!(!r.stdout.lines().any(|l| l.trim() == "1"), "check must not run the program");
    let r = rythme(&["check", bad.to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert_eq!(r.stderr.matches("error[E003]").count(), 2, "all errors are reported at once:\n{}", r.stderr);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_command_reports_failures() {
    let dir = std::env::temp_dir().join(format!("rythme-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("t.ry");
    std::fs::write(&file, "test \"good\" = assert_eq(1 + 1, 2)\ntest \"bad\" = assert_eq(1 + 1, 3)\ntest \"false\" = 1 > 2\n").unwrap();
    let r = rythme(&["test", file.to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("1 passed, 2 failed"), "{}", r.stdout);
    assert!(r.stderr.contains("expected: 3"), "{}", r.stderr);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn modules_and_missing_files() {
    let dir = std::env::temp_dir().join(format!("rythme-use-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("lib.ry"), "fn triple(x) = x * 3\nprint(\"lib loaded\")\n").unwrap();
    std::fs::write(dir.join("main.ry"), "use \"lib.ry\"\nuse \"lib\"\nprint(triple(5))\n").unwrap();
    std::fs::write(dir.join("broken.ry"), "use \"lbi.ry\"\n").unwrap();
    std::fs::write(dir.join("a.ry"), "use \"b.ry\"\n").unwrap();
    std::fs::write(dir.join("b.ry"), "use \"a.ry\"\n").unwrap();
    let r = rythme(&[dir.join("main.ry").to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(r.stdout, "lib loaded\n15\n", "a file runs only once");
    let r = rythme(&[dir.join("broken.ry").to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("E017") && r.stderr.contains("lib.ry"), "{}", r.stderr);
    let r = rythme(&[dir.join("a.ry").to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("E017"), "{}", r.stderr);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn script_arguments_and_unknown_commands() {
    let dir = std::env::temp_dir().join(format!("rythme-args-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("args.ry");
    std::fs::write(&file, "print(args())\n").unwrap();
    let r = rythme(&[file.to_str().unwrap(), "a", "b"]);
    assert_eq!(r.stdout, "[\"a\", \"b\"]\n");
    let r = rythme(&["chek", "x.ry"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("rythme check"), "{}", r.stderr);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn repl_keeps_definitions() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .env("HOME", std::env::temp_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"fn sq(x) = x * x\nsq(12)\nx = 1\nx = 2\nfn f(n)\n  n + x\nend\nf(1)\n:names\n:quit\n").unwrap();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("144"), "{stdout}");
    assert!(stdout.contains("\n3\n"), "a multi-line definition and a redefinition work in the REPL: {stdout}");
    assert!(stdout.contains("f  sq  x"), "{stdout}");
}

#[test]
fn editor_grammar_knows_every_builtin() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("editors/vscode/syntaxes/rythme.tmLanguage.json");
    let grammar = std::fs::read_to_string(path).unwrap();
    for b in rythme::builtins::BUILTINS {
        assert!(
            grammar.contains(&format!("|{}|", b.name))
                || grammar.contains(&format!("({}|", b.name))
                || grammar.contains(&format!("|{})", b.name)),
            "{} is missing from the VS Code grammar",
            b.name
        );
    }
}

#[test]
fn tree_sitter_grammar_knows_every_keyword() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("editors");
    let read = |p: &str| std::fs::read_to_string(dir.join(p)).unwrap();
    let grammar = read("tree-sitter-rythme/grammar.js");
    let line = grammar.lines().find(|l| l.starts_with("const KEYWORDS = [")).expect("KEYWORDS in grammar.js");
    let listed: Vec<&str> = line.split('\'').skip(1).step_by(2).collect();
    assert_eq!(listed, rythme::lexer::KEYWORDS, "the KEYWORDS of grammar.js");
    let queries =
        ["tree-sitter-rythme/queries/highlights.scm", "zed/languages/rythme/highlights.scm", "helix/queries/rythme/highlights.scm"];
    for q in queries {
        let text = read(q);
        for k in rythme::lexer::KEYWORDS.iter().filter(|k| !matches!(**k, "true" | "false")) {
            assert!(text.contains(&format!("\"{k}\"")), "`{k}` has no color in {q}");
        }
    }
}
