//! `rythme test`: folders, filters, summaries, the watch mode, property tests
//! with shrinking, and the reports of `assert_eq`.

mod common;

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use common::{fails, ok, rythme, Run};

/// A folder of files for one test, removed at the end.
struct Dir(PathBuf);

impl Dir {
    fn new(name: &str, files: &[(&str, &str)]) -> Dir {
        let dir = std::env::temp_dir().join(format!("rythme-testing-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for (path, text) in files {
            let path = dir.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        Dir(dir)
    }

    fn path(&self, p: &str) -> String {
        self.0.join(p).to_string_lossy().into_owned()
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn test_code(code: &str, extra: &[&str]) -> Run {
    let dir = Dir::new(&format!("code-{:x}", code.len() * 31 + code.bytes().map(usize::from).sum::<usize>()), &[("t.ry", code)]);
    let file = dir.path("t.ry");
    let mut args = vec!["test", file.as_str()];
    args.extend(extra);
    rythme(&args)
}

#[test]
fn a_folder_runs_every_file_with_tests() {
    let dir = Dir::new(
        "folder",
        &[
            ("a.ry", "fn double(x) = x * 2\ntest \"double\" = assert_eq(double(2), 4)\ntest \"zero\" = assert_eq(double(0), 0)\n"),
            ("sub/b.ry", "test \"wrong\" = assert_eq(1 + 1, 3)\n"),
            ("c.ry", "print(\"no tests here\")\n"),
            (".hidden/d.ry", "test \"hidden\" = assert(false)\n"),
            ("broken.ry", "test \"broken\" = assert_eq(unknown_name, 1)\n"),
        ],
    );
    let r = rythme(&["test", &dir.path("")]);
    assert_eq!(r.code, 1, "{}{}", r.stdout, r.stderr);
    assert!(r.stdout.contains("Running 2 test(s) of"), "{}", r.stdout);
    assert!(r.stdout.contains("a.ry") && r.stdout.contains("b.ry"), "{}", r.stdout);
    assert!(!r.stdout.contains("hidden") && !r.stdout.contains("no tests here"), "{}", r.stdout);
    assert!(r.stdout.contains("broken.ry: the file has errors"), "{}", r.stdout);
    assert!(r.stderr.contains("error[E003]"), "{}", r.stderr);
    assert!(r.stdout.contains("Failed tests:\n  ✗ wrong ("), "{}", r.stdout);
    assert!(r.stdout.contains("3 file(s), 3 test(s): 2 passed, 1 failed, 1 file(s) with errors ("), "{}", r.stdout);
    assert!(r.stdout.trim_end().ends_with(" ms)") || r.stdout.trim_end().ends_with(" s)"), "the summary gives the time: {}", r.stdout);

    // Several files, one of them twice.
    let r = rythme(&["test", &dir.path("a.ry"), &dir.path("sub"), &dir.path("a.ry")]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("2 file(s), 3 test(s): 2 passed, 1 failed ("), "{}", r.stdout);

    // A folder without tests.
    let empty = Dir::new("empty", &[("x.ry", "print(1)\n")]);
    let r = rythme(&["test", &empty.path("")]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.contains("No file with tests"), "{}", r.stdout);
}

#[test]
fn one_file_shows_what_it_always_showed() {
    let r = test_code("test \"good\" = assert_eq(1 + 1, 2)\ntest \"bad\" = assert_eq(1 + 1, 3)\n", &[]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.starts_with("Running 2 test(s)\n  ✓ good\n  ✗ bad\n"), "{}", r.stdout);
    assert!(r.stdout.contains("2 test(s): 1 passed, 1 failed ("), "{}", r.stdout);
    assert!(!r.stdout.contains("Failed tests"), "one failure is not listed again: {}", r.stdout);
    let r = test_code("x = 1\n", &[]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("No tests found"), "{}", r.stdout);
}

#[test]
fn a_filter_selects_tests_by_name() {
    let code = "test \"Addition works\" = assert_eq(1 + 1, 2)\ntest \"subtraction\" = assert_eq(1 - 1, 1)\ntest \"more addition\" = assert(true)\n";
    let r = test_code(code, &["--filter", "addition"]);
    assert_eq!(r.code, 0, "{}{}", r.stdout, r.stderr);
    assert!(r.stdout.starts_with("Running 2 of 3 test(s)\n"), "{}", r.stdout);
    assert!(!r.stdout.contains("subtraction"), "{}", r.stdout);
    assert!(r.stdout.contains("2 test(s): 2 passed, 0 failed, 1 left out by the filter ("), "{}", r.stdout);
    let r = test_code(code, &["-f", "SUB", "--filter=works"]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("2 test(s): 1 passed, 1 failed, 1 left out by the filter"), "{}", r.stdout);
    let r = test_code(code, &["--filter", "nothing"]);
    assert_eq!(r.code, 1, "a filter that selects nothing is a mistake");
    assert_eq!(r.stdout, "No test name contains `nothing`.\n");
    let r = common::rythme_lang("fr", &["test", "--filtre", "x"]);
    assert_eq!(r.code, 2, "{}", r.stderr);
}

#[test]
fn options_and_mistakes() {
    let r = test_code("test \"args\" = assert_eq(args(), [\"a\", \"--b\"])\n", &["--", "a", "--b"]);
    assert_eq!(r.code, 0, "{}{}", r.stdout, r.stderr);
    let r = rythme(&["test"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("rythme test ."), "{}", r.stderr);
    let r = test_code("test \"x\" = true\n", &["--wach"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("`--watch`"), "{}", r.stderr);
    let r = test_code("test \"x\" = true\n", &["--seed", "-3"]);
    assert_eq!(r.code, 2);
    let r = test_code("test \"x\" = true\n", &["--seed"]);
    assert_eq!(r.code, 2);
    let r = rythme(&["test", "no-such-file-anywhere.ry"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E017]"), "{}", r.stderr);
    let r = test_code("test \"x\" = true\n", &["--seed", "random"]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.starts_with("Seed: "), "{}", r.stdout);
    let r = common::rythme_lang("fr", &["test", "--seed", "abc", "x.ry"]);
    assert!(r.stderr.contains("la graine doit"), "{}", r.stderr);
}

/// Runs one property test and returns the error output.
fn property(code: &str, extra: &[&str]) -> String {
    let r = test_code(&format!("test \"property\" = {code}\n"), extra);
    assert_eq!(r.code, 1, "{code} should fail:\n{}{}", r.stdout, r.stderr);
    r.stderr
}

#[test]
fn failing_values_are_simplified() {
    let e = property("check_all(fn(x) = x < 10, ints())", &[]);
    assert!(e.contains("error[E015]: the property is false for 10\n"), "{e}");
    assert!(e.contains("seed 42"), "{e}");
    let e = property("check_all(fn(x) = x > -7 or x < -20, ints())", &[]);
    assert!(e.contains("false for -7\n"), "towards 0 from below: {e}");
    let e = property("check_all(fn(x) = x < 50, ints(20, 100))", &[]);
    assert!(e.contains("false for 50\n"), "{e}");
    let e = property("check_all(fn(l) = len(l) < 3, lists_of(ints()))", &[]);
    assert!(e.contains("false for [0, 0, 0]\n"), "{e}");
    let e = property("check_all(fn(l) = not contains(l, 3), lists_of(ints(0, 5)))", &[]);
    assert!(e.contains("false for [3]\n"), "{e}");
    let e = property("check_all(fn(s) = len(s) < 4, texts())", &[]);
    assert!(e.contains("false for \"aaaa\"\n"), "{e}");
    let e = property("check_all(fn(c) = c != \"b\", one_of([\"a\", \"b\", \"c\"]))", &[]);
    assert!(e.contains("false for \"b\"\n"), "{e}");
    let e = property("check_all(fn(b) = not b, bools())", &[]);
    assert!(e.contains("false for true\n"), "{e}");
    let e = property("check_all(fn(r) = r.x < 3 or r.y < 4, records_of({x: ints(0, 9), y: ints(0, 9)}))", &[]);
    assert!(e.contains("false for {x: 3, y: 4}\n"), "{e}");
    let e = property("check_all(fn(x) = x < 0.5, floats(0, 1))", &[]);
    assert!(e.contains("error[E015]: the property is false for 0.5"), "{e}");
    let e = property("check_all(fn(l) = len(l) < 2, lists_of(texts()))", &[]);
    assert!(e.contains("false for [\"\", \"\"]\n"), "{e}");
}

#[test]
fn the_seed_gives_the_same_values() {
    let code = "check_all(fn(x) = x < 1000, ints(0, 1000000))";
    let first = property(code, &["--seed", "5"]);
    assert_eq!(first, property(code, &["--seed", "5"]));
    assert!(first.contains("seed 5"), "{first}");
    let other = property(code, &["--seed", "6"]);
    assert!(other.contains("seed 6"), "{other}");
    // Simplified, it is the same value whatever the seed.
    assert!(first.contains("false for 1000\n") && other.contains("false for 1000\n"), "{first}{other}");
    // `sample` too.
    let dir = Dir::new("sample", &[("s.ry", "test \"s\"\n  print(sample(ints(), 5))\n  true\nend\n")]);
    let run = |seed: &str| rythme(&["test", &dir.path("s.ry"), "--seed", seed]).stdout.lines().nth(1).unwrap_or_default().to_string();
    assert_eq!(run("1"), run("1"));
    assert_ne!(run("1"), run("2"));
}

#[test]
fn errors_inside_a_property_show_the_value() {
    let e = property("check_all(fn(x) = assert_eq(x * 2, x + 2), ints())", &[]);
    assert!(e.contains("error[E015]: the property fails for 0: assertion failed: the values are different"), "{e}");
    assert!(e.contains("got:      0") && e.contains("expected: 2"), "{e}");
    let e = property("check_all(fn(x) = 100 // x > -1000, ints())", &[]);
    assert!(e.contains("the property fails for 0: "), "{e}");
    let e = property("check_all(fn(x) = if x > 3 then fail(\"too big\") else true, ints())", &[]);
    assert!(e.contains("error[E021]: the property fails for 4: too big"), "{e}");
    // `None` and `Err(...)` fail; `Some(...)`, `Ok(...)` and `()` pass.
    let e = property("check_all(fn(x) = if x > 2 then None else Some(x), ints())", &[]);
    assert!(e.contains("the property gives None for 3"), "{e}");
    let r = test_code(
        "test \"p\" = check_all(fn(x) = assert(x == x), ints())\ntest \"q\" = check_all(fn(t) = parse_int(\"{t}\"), ints())\n",
        &[],
    );
    assert_eq!(r.code, 0, "{}{}", r.stdout, r.stderr);
}

#[test]
fn generators_are_checked() {
    ok("print(ints(1, 6), bools())", "{generator: \"ints\", min: 1, max: 6} {generator: \"bools\"}");
    ok("print(len(sample(ints(), 7)), all(sample(ints(3, 5), 200), fn(x) = x >= 3 and x <= 5))", "7 true");
    ok("print(all(sample(floats(-0.5, 0.5), 200), fn(x) = x >= -0.5 and x <= 0.5))", "true");
    ok("print(all(sample(texts(3), 200), fn(t) = len(t) <= 3), all(sample(lists_of(bools(), 2), 100), fn(l) = len(l) <= 2))", "true true");
    ok("print(all(sample(one_of([1, 2]), 50), fn(x) = x == 1 or x == 2), sample(ints(), 0))", "true []");
    ok("print(sample(records_of({a: one_of([7])}), 2))", "[{a: 7}, {a: 7}]");
    ok("check_all(fn(x) = x == x, ints())\nprint(\"ok\")", "ok");
    fails("ints(5, 1)", "E023");
    fails("ints(1)", "E023");
    fails("texts(2000)", "E023");
    fails("lists_of(ints(), -1)", "E023");
    fails("one_of([])", "E023");
    fails("records_of({a: 1})", "E023");
    fails("floats(0, float(\"inf\"))", "E023");
    fails("check_all(fn(x) = true, {a: 1})", "E023");
    fails("check_all(fn(x) = true, {generator: \"ints\", min: 3, max: 1})", "E023");
    fails("check_all(fn(x) = true, ints(), 0)", "E023");
    fails("check_all(fn(x) = 5, ints())", "E005");
    fails("sample(5)", "E005");
    fails("check_all(fn(x) = true, 5)", "E005");
    fails("check_all(fn(a, b) = true, ints())", "E004");
    // Generators inside generators, made by hand, deep.
    fails("g = fold(1..100, ints(), fn(g, _) = {generator: \"lists_of\", of: g, max_length: 1})\ncheck_all(fn(x) = true, g)", "E023");
}

#[test]
fn a_test_with_a_property_can_fail() {
    // The checker knows that `check_all` checks something (no W007).
    let dir = Dir::new("w007", &[("p.ry", "test \"p\" = check_all(fn(x) = x == x, ints())\n")]);
    let r = rythme(&["check", &dir.path("p.ry")]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("no problems found"), "{}{}", r.stdout, r.stderr);
}

#[test]
fn assert_eq_says_where_values_differ() {
    let e = fails("assert_eq([1, 2, {name: \"Bob\", age: 3}], [1, 2, {name: \"Bo\", age: 3}])", "E015").stderr;
    assert!(e.contains("first difference at [2].name: got \"Bob\", expected \"Bo\""), "{e}");
    let e = fails("assert_eq([1, 2, 3], [1, 2])", "E015").stderr;
    assert!(e.contains("the list has 3 element(s), 2 expected") && e.contains("in excess from [2]: 3"), "{e}");
    let e = fails("assert_eq({a: 1}, {a: 1, b: 2})", "E015").stderr;
    assert!(e.contains("missing field(s): b"), "{e}");
    let e = fails("assert_eq({a: [1, \"2\"]}, {a: [1, 2]})", "E015").stderr;
    assert!(e.contains("first difference at a[1]: got \"2\", expected 2, which are of different types: Text and Int"), "{e}");
    let e = fails("assert_eq(\"hello world\", \"hello World\")", "E015").stderr;
    let expected = e.lines().find(|l| l.contains("expected: ")).unwrap();
    let mark = e.lines().find(|l| l.contains("^ first difference: character 7")).unwrap();
    assert_eq!(mark.find('^'), expected.find('W'), "{e}");
    let e = fails("assert_eq(\"a\\nb\\nc\", \"a\\nB\\nc\")", "E015").stderr;
    assert!(e.contains("first different line: line 2\n") && e.contains("got:      \"b\""), "{e}");
    let r = common::rythme_lang("fr", &["-e", "assert_eq([1], [2])"]);
    assert!(r.stderr.contains("première différence en [0] : obtenu 1, attendu 2"), "{}", r.stderr);
}

#[test]
fn watch_runs_the_tests_again_when_a_file_changes() {
    let dir = Dir::new("watch", &[("w.ry", "test \"one\" = assert_eq(1, 1)\n")]);
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["test", "--watch", &dir.path("")])
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("run rythme");
    let (tx, rx) = std::sync::mpsc::channel();
    let stdout = child.stdout.take().unwrap();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let mut seen = Vec::new();
    let mut wait_for = |text: &str| {
        let deadline = Instant::now() + Duration::from_secs(30);
        while Instant::now() < deadline {
            if let Ok(line) = rx.recv_timeout(Duration::from_millis(100)) {
                let found = line.contains(text);
                seen.push(line);
                if found {
                    return true;
                }
            }
        }
        false
    };
    let watching = wait_for("Watching");
    // A change of size is seen even when the clock of the file system is coarse.
    std::thread::sleep(Duration::from_millis(100));
    std::fs::write(Path::new(&dir.path("w.ry")), "test \"one\" = assert_eq(1, 1)\ntest \"two\" = assert_eq(2, 3)\n").unwrap();
    let changed = watching && wait_for("w.ry changed");
    let again = changed && wait_for("1 file(s), 2 test(s): 1 passed, 1 failed");
    let _ = child.kill();
    let _ = child.wait();
    assert!(again, "{}", seen.join("\n"));
}
