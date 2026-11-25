//! The catalogs registered while running give the same messages as those
//! compiled in, in every language.
//!
//! The website's playground runs Rythme in WebAssembly with the feature
//! `runtime-catalogs`: only English and French are compiled in, and the page
//! fetches the catalog of its language, a file written by `lang::catalog`. This
//! test writes those files, builds Rythme with the feature (in its own folder of
//! `target/`, once, then only when the sources change), points it at them
//! (`RYTHME_CATALOGS`) and compares what it prints with what the program compiled
//! without the feature prints: diagnostics of every kind, warnings, failed tests,
//! formatting, the help of every topic, function and code. The two must never
//! drift.

// Built with `runtime-catalogs` itself, the program has no catalog to write.
#![cfg(not(feature = "runtime-catalogs"))]

use std::path::{Path, PathBuf};
use std::process::Command;

use rythme::lang::{self, Lang};

/// Programs with problems: each error and warning the checker, the parser and the
/// interpreter say, with their notes, helps and suggestions.
const PROGRAMS: &[&str] = &[
    "fn add(a, b) = a + b\nprint(add(1))",
    "print(lenght([1, 2]))",
    "x = 1\nx = 2\nprint(x)",
    "print(1 + \"a\")",
    "print([1, 2][5])",
    "print(1 / 0)",
    "if x > 0 { print(x) }",
    "print({name: \"Ada\"}.nmae)",
    "print([1, 2].lenght())",
    "match 3 {\n  1 => print(\"one\")\n}",
    "type Shape = Circle(r) | Square(s)\nfn area(s) = match s {\n  Circle(r) => r * r\n}\nprint(area(Square(2)))",
    "print(int(\"abc\")?)",
    "fn f(x) = x\nunused = 3\nprint(f(1))",
    "print(\"a\" * 3.5)",
    "for x in [1, 2] { print(x) }",
    "print(sqrt(-1))",
    "print(range(1, 3, 0))",
    "print(map([1, 2]))",
    "print(map([1, 2], 3))",
    "x: Int = \"text\"\nprint(x)",
    "print(\"unterminated)",
    "print((1 + 2)",
    "print(1 +)",
    "let x = 3\nprint(x)",
    "def f(x):\n  return x\nprint(f(1))",
    "print(null)",
    "print(Some(1) + 1)",
    "print(head([]))",
    "print(get({a: 1}, \"b\"))",
    "print(date(2023, 2, 29))",
    "print(regex(\"(\"))",
    "print(csv_parse(\"a,b\\n1\"))",
    "print(\"abc\"[10])",
    "print(split(\"a b\"))",
    "print(sort([1, \"a\"]))",
    "print(json_parse(\"{\"))",
    "print(read_file(\"/no/such/file.txt\"))",
    "use nowhere\nprint(1)",
    "assert(1 == 2)",
    "panic(\"stop\")",
    "fn f(x) =\n  y = x\n  y\nprint(f())",
    "print([1, 2, 3] |> filter(fn(x) = x))",
    "print(1 == \"1\")",
    "{a: 1, a: 2}",
    "print(dict([(1, 2), (3)]))",
];

/// Tests that fail, and a file that does not format.
const TESTS: &str = "fn double(x) = x * 2\n\ntest \"double\" {\n  assert(double(2) == 5)\n  assert_eq(double(3), 7)\n}\n\ntest \"fine\" {\n  assert(true)\n}\n";

#[test]
fn catalogs_registered_while_running_give_the_same_messages() {
    let compiled = Path::new(env!("CARGO_BIN_EXE_rythme"));
    let folder = target_dir(compiled).join("runtime-catalogs");
    let catalogs = folder.join("catalogs");
    std::fs::create_dir_all(&catalogs).unwrap();
    for l in Lang::all() {
        let file = lang::catalog(l);
        assert_eq!(file.is_empty(), l == Lang::En || l == Lang::Fr, "{}", l.code());
        if !file.is_empty() {
            let (read, texts) = lang::read_catalog(&file).unwrap();
            assert_eq!(read, l);
            assert_eq!(texts.len(), l.language().texts.iter().filter(|(_, tr)| !tr.is_empty()).count(), "{}: one key per text", l.code());
            std::fs::write(catalogs.join(format!("{}.txt", l.code())), file).unwrap();
        }
    }
    let at_runtime = build_with_runtime_catalogs(&folder);

    let work = folder.join("work");
    std::fs::create_dir_all(&work).unwrap();
    let mut cases: Vec<Vec<String>> = Vec::new();
    let mut programs: Vec<(String, bool)> = PROGRAMS.iter().map(|p| (p.to_string(), true)).collect();
    // The wrong examples are checked, not run: one may never end.
    programs.extend(wrong_examples(compiled).into_iter().map(|p| (p, false)));
    for (n, (program, to_run)) in programs.iter().enumerate() {
        let file = work.join(format!("p{n}.ry"));
        std::fs::write(&file, program).unwrap();
        let file = file.to_string_lossy().into_owned();
        cases.push(vec!["check".into(), file.clone()]);
        if *to_run {
            cases.push(vec!["run".into(), file.clone()]);
        }
        cases.push(vec!["fmt".into(), "--check".into(), file]);
    }
    let tests = work.join("tests.ry");
    std::fs::write(&tests, TESTS).unwrap();
    cases.push(vec!["test".into(), tests.to_string_lossy().into_owned()]);
    cases.push(vec!["steps".into(), tests.to_string_lossy().into_owned(), "double(4)".into()]);
    // Every topic, function and code in one program: one run per language.
    let mut names: Vec<String> = rythme::help::TOPICS.iter().map(|(t, _)| t.to_string()).collect();
    names.extend(rythme::builtins::BUILTINS.iter().map(|b| b.name.to_string()));
    names.extend(rythme::help::CODES.iter().map(|c| c.to_string()));
    let helps: String = names.iter().map(|n| format!("help({n:?})\n")).collect();
    cases.push(vec!["-e".into(), helps]);
    cases.push(vec!["help".into()]);
    cases.push(vec!["help".into(), "nothing_like_this".into()]);
    cases.push(vec!["explain".into(), "E999".into()]);
    cases.push(vec!["nothing_like_this".into()]);
    cases.push(vec!["check".into(), "/no/such/file.ry".into()]);

    // One thread per language.
    std::thread::scope(|scope| {
        for l in Lang::all() {
            let (cases, at_runtime, catalogs) = (&cases, &at_runtime, &catalogs);
            scope.spawn(move || {
                for case in cases {
                    let expected = run(compiled, l.code(), case, None);
                    let got = run(at_runtime, l.code(), case, Some(catalogs));
                    assert_eq!(got, expected, "--lang {} {:?}: the catalog registered while running gives another text", l.code(), case);
                }
            });
        }
    });

    // Without its catalog (a page whose fetch failed), English.
    for case in &cases[..6] {
        let expected = run(compiled, "en", case, None);
        assert_eq!(run(&at_runtime, "de", case, Some(&folder.join("none"))), expected, "{case:?}");
    }
}

/// The folder `target/` of the program being tested (`target/debug/rythme`).
fn target_dir(binary: &Path) -> PathBuf {
    binary.parent().and_then(Path::parent).expect("target/<profile>/rythme").to_path_buf()
}

/// Rythme built with the feature `runtime-catalogs`, in `folder`.
fn build_with_runtime_catalogs(folder: &Path) -> PathBuf {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(cargo)
        .args(["build", "--quiet", "--features", "runtime-catalogs", "--bin", "rythme", "--target-dir"])
        .arg(folder)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("run cargo");
    assert!(status.success(), "cargo build --features runtime-catalogs failed");
    folder.join("debug").join(format!("rythme{}", std::env::consts::EXE_SUFFIX))
}

/// The wrong examples of the explanations of the codes (`Wrong:   code`).
fn wrong_examples(binary: &Path) -> Vec<String> {
    let mut found = Vec::new();
    for code in rythme::help::CODES {
        let out = run(binary, "en", &["explain".to_string(), code.to_string()], None);
        for line in out.lines() {
            if let Some(example) = line.strip_prefix("Wrong:") {
                found.push(format!("{}\n", example.trim()));
            }
        }
    }
    assert!(found.len() >= 10, "the explanations show wrong examples");
    found
}

/// What the program prints, on both outputs, with its exit code.
fn run(binary: &Path, lang: &str, args: &[String], catalogs: Option<&Path>) -> String {
    let mut command = Command::new(binary);
    command.args(args).env("RYTHME_LANG", lang).env("NO_COLOR", "1").env_remove("RYTHME_CATALOGS");
    if let Some(dir) = catalogs {
        command.env("RYTHME_CATALOGS", dir);
    }
    let out = command.output().expect("run rythme");
    format!(
        "exit {:?}\n--- stdout\n{}\n--- stderr\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}
