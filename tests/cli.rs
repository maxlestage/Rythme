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
        "system",
        "projects",
        "dates",
        "regex",
        "csv",
        "random",
        "repl",
        "fmt",
        "doc",
        "steps",
        "debug",
        "editors",
        "ruby",
        "javascript",
        "builtins",
    ] {
        let r = rythme(&["help", topic]);
        assert_eq!(r.code, 0, "topic {topic}");
        assert!(r.stdout.len() > 100, "topic {topic} is too short");
    }
    let r = rythme(&["help", "map"]);
    assert!(r.stdout.contains("map(list, f)") && r.stdout.contains("Example"));
    let r = rythme(&["help", "foldl"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("`fold`"));
    // A Ruby name is the built-in function it names.
    let r = rythme(&["help", "reduce"]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("the Ruby name of `fold`") && r.stdout.contains("fold(list, start, f)"), "{}", r.stdout);
    let r = rythme(&["help", "fold"]);
    assert!(r.stdout.contains("Same as: `reduce`, `inject`"), "{}", r.stdout);
    let r = rythme(&["help", "ruby"]);
    assert!(r.stdout.starts_with("COMING FROM RUBY"), "{}", r.stdout);
    let r = common::rythme_lang("fr", &["help", "ruby"]);
    assert!(r.stdout.starts_with("VENIR DE RUBY"), "{}", r.stdout);
    for name in ["javascript", "js", "typescript"] {
        let r = rythme(&["help", name]);
        assert!(r.stdout.starts_with("COMING FROM JAVASCRIPT") && r.stdout.contains("find_last"), "{}", r.stdout);
    }
    let r = rythme(&["help", "builtins"]);
    assert!(r.stdout.contains("upcase (same as upper)"), "{}", r.stdout);
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

/// A folder of its own for one test, removed first.
fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-cli-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn every_command_has_a_help_in_every_language() {
    for cmd in ["run", "check", "test", "fmt", "fix", "steps", "debug", "dap", "exercise", "learn", "help", "explain", "-e", "repl"] {
        for flag in ["--help", "-h"] {
            let r = rythme(&[cmd, flag]);
            assert_eq!(r.code, 0, "{cmd} {flag}: {}", r.stderr);
            assert!(r.stdout.starts_with(&format!("rythme {cmd}")), "{cmd} {flag}:\n{}", r.stdout);
            assert!(r.stdout.contains("--lang CODE"), "{cmd} {flag}:\n{}", r.stdout);
        }
    }
    for cmd in ["version", "completions", "man", "upgrade", "search", "info", "add", "update", "publish"] {
        assert!(rythme(&[cmd, "--help"]).stdout.starts_with(&format!("rythme {cmd}")), "{cmd}");
    }
    let r = common::rythme_lang("fr", &["test", "--help"]);
    assert!(r.stdout.contains("lancer les tests d'un fichier ou d'un dossier"), "{}", r.stdout);
    let r = common::rythme_lang("zh", &["run", "--help"]);
    assert!(r.stdout.contains("运行程序"), "{}", r.stdout);
    let r = common::rythme_lang("it", &["run", "--help"]);
    assert!(r.stdout.contains("esegue un programma"), "{}", r.stdout);
    let r = common::rythme_lang("ja", &["run", "--help"]);
    assert!(r.stdout.contains("プログラムを実行する"), "{}", r.stdout);
    // Aliases in French, and options before the help.
    assert!(rythme(&["lancer", "--seed", "--help"]).stdout.starts_with("rythme run"));
    assert!(rythme(&["fmt", "--check", "--help"]).stdout.starts_with("rythme fmt"));
    // After the file, the words are the program's.
    let dir = scratch("help-after-file");
    let file = dir.join("args.ry");
    std::fs::write(&file, "print(args())\n").unwrap();
    let r = rythme(&["run", file.to_str().unwrap(), "--help"]);
    assert_eq!(r.stdout.trim(), "[\"--help\"]", "{}", r.stderr);
    let tested = dir.join("tested.ry");
    std::fs::write(&tested, "test \"its arguments\" = assert_eq(args(), [\"--help\"])\n").unwrap();
    let r = rythme(&["test", tested.to_str().unwrap(), "--", "--help"]);
    assert_eq!(r.code, 0, "{}{}", r.stdout, r.stderr);
    assert!(!r.stdout.starts_with("rythme test"), "{}", r.stdout);
    let _ = std::fs::remove_dir_all(&dir);
    // `rythme help` names the tools, and `rythme help CMD` shows the usage of a command.
    assert!(rythme(&["help"]).stdout.contains("rythme completions SHELL"));
    assert!(rythme(&["help", "check"]).stdout.starts_with("rythme check"));
    assert!(rythme(&["help", "upgrade"]).stdout.starts_with("rythme upgrade"));
}

#[test]
fn programs_from_the_standard_input_and_folders_to_check() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let run = |args: &[&str], input: &str| {
        let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .args(args)
            .env("RYTHME_LANG", "en")
            .env("NO_COLOR", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        (String::from_utf8_lossy(&out.stdout).into_owned(), out.status.code())
    };
    assert_eq!(run(&["run", "-", "a", "b"], "print(1 + 1, args())\n"), ("2 [\"a\", \"b\"]\n".into(), Some(0)));
    assert_eq!(run(&["-"], "print(\"stdin\")\n"), ("stdin\n".into(), Some(0)));
    assert_eq!(run(&["run", "--seed", "3", "-"], "print(random(1, 1000) == random(1, 1000))\n").1, Some(0));
    assert_eq!(run(&["run", "-"], "print(nope)\n").1, Some(1));
    // With --format json too: one document, the output kept in it.
    let (doc, code) = run(&["run", "--format", "json", "-", "a"], "print(args())\n");
    assert_eq!(code, Some(0));
    assert!(doc.contains("\"ok\":true") && doc.contains("\"stdout\":\"[\\\"a\\\"]\\n\""), "{doc}");
    let (doc, code) = run(&["run", "--format", "json", "-"], "print(nope)\n");
    assert_eq!(code, Some(1));
    assert!(doc.contains("\"ok\":false") && doc.contains("\"file\":\"<stdin>\""), "{doc}");

    let dir = scratch("check-folder");
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    std::fs::write(dir.join("good.ry"), "print(1)\n").unwrap();
    std::fs::write(dir.join("sub/bad.ry"), "print(nope)\n").unwrap();
    let r = rythme(&["check", dir.to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("good.ry: no problems found") && r.stdout.contains("bad.ry: 1 error(s)"), "{}", r.stdout);
    std::fs::remove_file(dir.join("sub/bad.ry")).unwrap();
    assert_eq!(rythme(&["check", dir.to_str().unwrap()]).code, 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn version_upgrade_completions_and_manual_page() {
    assert_eq!(rythme(&["--version"]).stdout, format!("rythme {}\n", rythme::VERSION));
    let r = rythme(&["version", "--verbose"]);
    assert!(r.stdout.starts_with(&format!("rythme {}\n", rythme::VERSION)), "{}", r.stdout);
    assert!(r.stdout.contains(&format!("built-in functions: {}", rythme::builtins::BUILTINS.len())), "{}", r.stdout);
    assert!(r.stdout.contains("JSON output (--format json): version 1"), "{}", r.stdout);
    assert_eq!(rythme(&["version", "--bogus"]).code, 2);
    let r = rythme(&["upgrade"]);
    assert!(r.stdout.contains("To install the latest version") && r.stdout.contains("https://github.com/maxlestage/Rythme/releases"));
    for shell in ["bash", "zsh", "fish", "powershell"] {
        let r = rythme(&["completions", shell]);
        assert_eq!(r.code, 0, "{shell}");
        for name in ["check", "completions", "lang"] {
            assert!(r.stdout.contains(name), "{shell}: {name}");
        }
    }
    assert_eq!(rythme(&["completions"]).code, 2);
    assert_eq!(rythme(&["completions", "tcsh"]).code, 2);
    // The bash script is valid bash, when bash is there to say so. Not on
    // Windows: `bash` there may be the stub of WSL, which fails without a
    // Linux installed, whatever the script.
    let bash = if cfg!(windows) {
        Err(std::io::ErrorKind::Unsupported.into())
    } else {
        std::process::Command::new("bash").args(["-n", "-c", &rythme(&["completions", "bash"]).stdout]).output()
    };
    if let Ok(out) = bash {
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    }
    let r = rythme(&["man"]);
    assert!(r.stdout.starts_with(".TH RYTHME 1"), "{}", r.stdout);
    for part in [".SH NAME", ".SH COMMANDS", ".SH EXIT STATUS", "rythme check"] {
        assert!(r.stdout.contains(part), "{part}");
    }
    assert!(common::rythme_lang("fr", &["man"]).stdout.contains(".SH COMMANDES"));
}
