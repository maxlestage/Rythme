//! `rythme steps`: the evaluation shown step by step. Its evaluator is a
//! second one, next to the interpreter: on every example, every solution of
//! the exercises and every sample of the manual, it must give exactly the
//! results of the interpreter.

mod common;

use std::path::{Path, PathBuf};

use common::{rythme, rythme_lang};
use rythme::driver::Driver;
use rythme::interp::Mode;
use rythme::io;
use rythme::steps::{self, Options};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Runs `f` on a thread with a big stack, like the `rythme` command.
fn big_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(move || {
            rythme::interp::set_stack_budget(256 << 20);
            f()
        })
        .unwrap()
        .join()
        .unwrap()
}

fn silent() -> Options {
    Options { show: false, limit: 50_000_000, ..Options::default() }
}

/// What a program prints when it runs (`stepped`: with the stepper), with
/// the first line of each error message.
fn outcome(path: &Path, stepped: bool) -> String {
    io::capture();
    let mut driver = Driver::new();
    driver.color = false;
    if let Some(m) = driver.load_file(path) {
        if stepped {
            let _ = steps::run_module(&mut driver, &m, silent(), None);
        } else {
            let _ = driver.execute(&m, Mode::Run);
        }
    }
    shown(&io::take_captured())
}

/// The standard output, and the first line of each error (the call traces
/// of the two evaluators may differ: a substitution has no call stack).
fn shown(captured: &str) -> String {
    let mut out = String::new();
    let mut rest = captured;
    while let Some(start) = rest.find(io::ERR_START) {
        out.push_str(&rest[..start]);
        let end = rest[start..].find(io::ERR_END).map(|e| start + e).unwrap_or(rest.len());
        let err = &rest[start + 1..end];
        if let Some(first) = err.lines().next().filter(|l| l.starts_with("error")) {
            // Both stop a calculation too deep for the stack, at different depths.
            let first = if first.contains("[E014]") || first.contains("too deep to be shown") { "error: too deep" } else { first };
            out.push_str(&format!("<{first}>\n"));
        }
        rest = &rest[(end + 1).min(rest.len())..];
    }
    out.push_str(rest);
    out
}

fn ry_files(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            ry_files(&path, found);
        } else if path.extension().is_some_and(|e| e == "ry") {
            found.push(path);
        }
    }
}

#[test]
fn the_stepper_gives_the_results_of_the_interpreter_on_every_example() {
    big_stack(|| {
        let mut files = Vec::new();
        ry_files(&root().join("examples"), &mut files);
        files.sort();
        assert!(files.len() >= 15);
        for path in files {
            let real = outcome(&path, false);
            assert_eq!(outcome(&path, true), real, "{}", path.display());
            let expected = path.with_extension("out");
            if expected.is_file() {
                assert_eq!(real, std::fs::read_to_string(expected).unwrap().replace("\r\n", "\n"), "{}", path.display());
            }
        }
    });
}

#[test]
fn the_stepper_gives_the_results_of_the_interpreter_on_the_exercise_solutions() {
    big_stack(|| {
        let mut files = Vec::new();
        ry_files(&root().join("docs/exercises/.solutions"), &mut files);
        files.sort();
        assert!(files.len() >= 60);
        for path in files {
            // The program itself.
            assert_eq!(outcome(&path, true), outcome(&path, false), "{}", path.display());
            // Its tests, one by one.
            let mut results = Vec::new();
            for stepped in [false, true] {
                io::capture();
                let mut driver = Driver::new();
                driver.color = false;
                let m = driver.load_file(&path).expect("a solution loads");
                driver.execute(&m, Mode::Test).expect("a solution runs");
                let tests = std::mem::take(&mut driver.interp.tests);
                assert!(!tests.is_empty(), "{}", path.display());
                let mut got = Vec::new();
                for test in &tests {
                    let r = if stepped { steps::eval_test(&mut driver.interp, test, silent()) } else { driver.interp.eval_test(test) };
                    got.push(match r {
                        Ok(v) => format!("{}: {}", test.name, v.repr()),
                        Err(d) => format!("{}: error {}", test.name, d.message),
                    });
                }
                got.push(shown(&io::take_captured()));
                results.push(got);
            }
            assert_eq!(results[1], results[0], "{}", path.display());
        }
    });
}

/// The ```rythme samples of the manual that do not depend on the outside
/// world (files, time, randomness, the input, the end of the program).
fn manual_samples(manual: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let lines: Vec<&str> = manual.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if lines[i] == "```rythme" {
            let start = i + 1;
            let mut j = start;
            while lines[j] != "```" {
                j += 1;
            }
            let code = lines[start..j].join("\n");
            let outside = ["file", "random", "shuffle", "now(", "today(", "time(", "input(", "args(", "env(", "exit(", "use "];
            if !outside.iter().any(|w| code.contains(w)) {
                out.push((start, code));
            }
            i = j;
        }
        i += 1;
    }
    out
}

#[test]
fn the_stepper_gives_the_results_of_the_interpreter_on_the_samples_of_the_manual() {
    big_stack(|| {
        let dir = std::env::temp_dir().join(format!("rythme-steps-manual-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut count = 0;
        for manual in ["docs/reference.md", "docs/reference.fr.md"] {
            let text = std::fs::read_to_string(root().join(manual)).unwrap();
            for (line, code) in manual_samples(&text) {
                let path = dir.join(format!("sample-{line}.ry"));
                std::fs::write(&path, &code).unwrap();
                assert_eq!(outcome(&path, true), outcome(&path, false), "{manual}, line {line}:\n{code}");
                count += 1;
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
        assert!(count > 100, "only {count} samples");
    });
}

fn program(name: &str, code: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-steps-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{name}.ry"));
    std::fs::write(&path, code).unwrap();
    path
}

#[test]
fn fib_is_shown_in_the_substitution_model() {
    let path = program("fib", "fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)\nfib(3)\n");
    let r = rythme(&["steps", path.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(
        r.stdout,
        "fib(3)\n  → fib(2) + fib(1)\n  → fib(1) + fib(0) + fib(1)\n  → 1 + fib(0) + fib(1)\n  → 1 + 0 + fib(1)\n  → 1 + fib(1)\n  → 1 + 1\n  → 2\n"
    );
    // In French, with the French name of the command.
    let r = rythme_lang("fr", &["etapes", path.to_str().unwrap(), "fib(2)"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(r.stdout, "fib(2)\n  → fib(1) + fib(0)\n  → 1 + fib(0)\n  → 1 + 0\n  → 1\n");
}

#[test]
fn recursion_on_lists_and_definitions() {
    let code = "fn total([]) = 0\nfn total([x, ..rest]) = x + total(rest)\nsum3 = total([1, 2, 3])\nprint(sum3 * 2)\n";
    let path = program("lists", code);
    let r = rythme(&["steps", path.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(
        r.stdout,
        "sum3 = total([1, 2, 3])\n  → 1 + total([2, 3])\n  → 1 + (2 + total([3]))\n  → 1 + (2 + (3 + total([])))\n  → 1 + (2 + (3 + 0))\n  → 1 + (2 + 3)\n  → 1 + 5\n  → 6\n\nprint(sum3 * 2)\n  → print(6 * 2)\n  → print(12)\n12\n  → ()\n"
    );
}

#[test]
fn tail_calls_closures_and_question_marks() {
    let code = r#"fn count(n, acc) = if n == 0 then acc else count(n - 1, acc + n)
fn adder(n) = fn(x) = x + n
fn half(n) = if n % 2 == 0 then Ok(n // 2) else Err("odd")
fn quarter(n)
  h = half(n)?
  half(h)
end
count(3, 0)
adder(10)(5)
quarter(6)
"#;
    let path = program("closures", code);
    let r = rythme(&["steps", path.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    let expected = "count(3, 0)\n  → count(2, 3)\n  → count(1, 5)\n  → count(0, 6)\n  → 6\n\n\
adder(10)(5)\n  → (fn(x) = x + 10)(5)\n  → 15\n\n\
quarter(6)\n  → do\n      h = half(6)?\n      half(h)\n    end\n  → do\n      h = Ok(3)?\n      half(h)\n    end\n  → do\n      h = 3\n      half(h)\n    end\n  → half(3)\n  → Err(\"odd\")\n";
    assert_eq!(r.stdout, expected);
}

#[test]
fn steps_are_bounded() {
    let code = "fn count(n) = if n == 0 then 0 else count(n - 1)\ncount(50)\nprint(\"after\")\n";
    let path = program("bounded", code);
    let r = rythme(&["steps", path.to_str().unwrap(), "--max", "3"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.starts_with("count(50)\n  → count(49)\n  → count(48)\n  → count(47)\n  ... (more steps"), "{}", r.stdout);
    assert!(r.stdout.contains("\n  → 0\n\nprint(\"after\")\n"), "{}", r.stdout);
    // Long lines are cut.
    let path = program("wide", "xs = 1..100\nlen(xs)\n");
    let r = rythme(&["steps", path.to_str().unwrap(), "--width", "30"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.lines().all(|l| l.chars().count() <= 30), "{}", r.stdout);
    assert!(r.stdout.contains("\n  → [1, 2, 3, 4, 5, 6, 7, 8, …\n"), "{}", r.stdout);
    // A program that never ends is stopped.
    let path = program("forever", "fn forever(n) = forever(n + 1)\nforever(0)\n");
    let r = rythme(&["steps", path.to_str().unwrap(), "--max", "2"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("stopped after"), "{}", r.stderr);
}

#[test]
fn errors_are_the_errors_of_the_interpreter() {
    let path = program("errors", "fn inverse(x) = 1 / x\nprint(inverse(0))\n");
    let r = rythme(&["steps", path.to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("print(inverse(0))\n  → print(1 / 0)\n"), "{}", r.stdout);
    assert!(r.stderr.contains("error[E008]: division by zero"), "{}", r.stderr);
    assert!(r.stderr.contains("in `inverse` called at"), "{}", r.stderr);
}

#[test]
fn the_command_explains_its_usage() {
    let r = rythme(&["steps"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("usage: rythme steps file.ry"), "{}", r.stderr);
    let r = rythme_lang("fr", &["etapes"]);
    assert!(r.stderr.contains("utilisation : rythme etapes"), "{}", r.stderr);
    let r = rythme(&["steps", "x.ry", "--max", "zero"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("`--max` needs a number"), "{}", r.stderr);
    let r = rythme(&["steps", "missing_file.ry"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("not found"), "{}", r.stderr);
    let path = program("defs", "fn double(x) = x * 2\n");
    let r = rythme(&["steps", path.to_str().unwrap()]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("Nothing to show"), "{}", r.stdout);
    let r = rythme(&["help", "steps"]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("rythme steps"), "{}", r.stdout);
    let r = rythme_lang("fr", &["help", "etapes"]);
    assert!(r.stdout.contains("rythme etapes"), "{}", r.stdout);
}

/// The steps are written like `rythme fmt` writes code.
#[test]
fn steps_are_written_in_the_style_of_the_formatter() {
    let code = r#"type Shape = Circle(radius) | Rect(width, height)
fn area(s) = match s
  Circle(r) -> 3 * r * r
  Rect(w, h) -> w * h
end
fn describe(n)
  kind = if n % 2 == 0 then "even" else "odd"
  "{n} is {kind}"
end
fn check(n) = n > 0 and n < 10 or n == -1
fn power(x) = -x ** 2 + (x - 1) * 3 // 2
shapes = [Circle(1), Rect(2, 3)]
print(shapes |> map(area) |> sum, describe(area(Rect(1, 3))))
{name: "Ada", "full name": [1, ..[2, 3]], ok: check(-1), p: power(-2)}
do
  a = power(2)
  b = [a, a + 1]
  b[1]
end
"#;
    let path = program("style", code);
    let r = rythme(&["steps", path.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    // Each step is code: the formatter leaves it as it is.
    let mut steps: Vec<String> = Vec::new();
    let mut in_step = false;
    for line in r.stdout.lines() {
        if let Some(step) = line.strip_prefix("  → ") {
            steps.push(step.to_string());
            in_step = true;
        } else if let (true, Some(more)) = (in_step, line.strip_prefix("    ")) {
            let last = steps.last_mut().unwrap();
            last.push('\n');
            last.push_str(more);
        } else {
            in_step = false;
        }
    }
    assert!(steps.len() > 10, "{}", r.stdout);
    for step in steps {
        let code = format!("{step}\n");
        assert_eq!(rythme::format::format(&code).ok().as_deref(), Some(code.as_str()), "{}", r.stdout);
    }
}
