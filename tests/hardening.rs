//! Bugs found by auditing the tools around the interpreter (checker,
//! built-in functions, command line, package index...): each one, kept from
//! coming back.

mod common;

use common::{fails, ok, rythme};

/// The program is checked without a single error or warning, and runs.
#[track_caller]
fn quiet(code: &str, expected: &str) {
    let r = rythme(&["-e", code]);
    assert_eq!(r.code, 0, "code:\n{code}\n{}", r.stderr);
    assert!(r.stderr.is_empty(), "code:\n{code}\n{}", r.stderr);
    assert_eq!(r.stdout.trim_end(), expected, "code:\n{code}");
}

// ----- the checker: no false positives ----------------------------------------

/// A function given for the elements of something known to be empty is never
/// called: like with `[]`, the checker says nothing about it.
#[test]
fn functions_never_called_on_empty_values_are_not_reported() {
    quiet("print(map_keys({}, len))", "{}");
    quiet("print(take_while(\"\", len))", "");
    quiet("print(drop_while(\"\", fn(c) = c + 1))", "");
    // Records without a name in common: nothing to merge.
    quiet("print(merge_with({a: 1}, {b: 2}, is_even))", "{a: 1, b: 2}");
    // The same mistakes are still found when the function is called.
    fails("print(map_keys({a: 1}, len))", "E005");
    fails("print(take_while(\"ab\", len))", "E005");
    fails("print(merge_with({a: 1}, {a: 2}, is_even))", "E004");
}

/// `sum(list, f)` needs numbers from `f`: the checker said "true or false".
#[test]
fn the_function_given_to_sum_must_return_a_number() {
    let r = fails("print(sum([1, 2], fn(x) = \"a\"))", "E023");
    assert!(r.stderr.contains("the function given to `sum` must return a number, but it returned"), "{}", r.stderr);
    ok("print(sum([1, 2], fn(x) = x * 10))", "30");
}

// ----- built-in functions: errors, never a crash -------------------------------

/// `from_base` cut a text inside a character of several bytes.
#[test]
fn from_base_with_characters_of_several_bytes() {
    ok("print(from_base(\"0é\", 16))", "Err(\"\\\"0é\\\" is not a whole number in base 16\")");
    ok("print(from_base(\"-日本\", 2))", "Err(\"\\\"-日本\\\" is not a whole number in base 2\")");
    ok("print(from_base(\"0xff\", 16))", "Ok(255)");
}

/// `zeros(0, n)` made a row of `n` numbers before seeing there are no rows.
#[test]
fn zeros_and_ones_with_a_huge_number_of_columns() {
    fails("print(zeros(0, 10 ** 18))", "E023");
    fails("print(ones(0, 10 ** 18))", "E023");
    ok("print(zeros(0, 3))", "[]");
    ok("print(ones(2, 2))", "[[1.0, 1.0], [1.0, 1.0]]");
}

// ----- fixes: each one removes its diagnostic --------------------------------

/// A new folder for each test: they run at the same time.
fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-hardening-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// `rythme fix --all` on `code`: the file after it.
fn fixed(name: &str, code: &str) -> String {
    let dir = temp_dir(name);
    let file = dir.join("a.ry");
    std::fs::write(&file, code).unwrap();
    rythme(&["fix", "--all", file.to_str().unwrap()]);
    let out = std::fs::read_to_string(&file).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    out
}

/// `else ()` went to the end of the line, after the `)` of the call.
#[test]
fn the_missing_else_goes_where_it_was_expected() {
    assert_eq!(fixed("else", "x = 1\nprint(if x > 0 then \"yes\")\n"), "x = 1\nprint(if x > 0 then \"yes\" else ())\n");
    assert_eq!(fixed("else-line", "x = 1\nif x > 0 then print(x)\n"), "x = 1\nif x > 0 then print(x) else ()\n");
    assert_eq!(fixed("else-comment", "x = 1\nif x > 0 then print(x) # why\n"), "x = 1\nif x > 0 then print(x) else () # why\n");
}

/// `then` went inside the comment that ends the line.
#[test]
fn the_missing_then_goes_before_a_comment() {
    let then = "x = 1\nif x > 0 then # positive\n  print(1)\nelse\n  print(2)\n";
    assert_eq!(fixed("then", "x = 1\nif x > 0 # positive\n  print(1)\nelse\n  print(2)\n"), then);
    let colon = "x = 1\nif x > 0:\n  print(1)\nelse\n  print(2)\n";
    assert_eq!(fixed("then-colon", colon), "x = 1\nif x > 0 then\n  print(1)\nelse\n  print(2)\n");
}

/// `fn f"..."`: the `f` is the name of the function, not the prefix of a text.
#[test]
fn a_function_named_f_is_not_an_f_string() {
    let code = "fn f\"x\"(n) = n\n";
    assert_eq!(fixed("fn-f", code), code);
    assert_eq!(fixed("f-string", "x = 1\nprint(f\"{x}\")\n"), "x = 1\nprint(\"{x}\")\n");
}

// ----- the command line ------------------------------------------------------

/// A word that is not UTF-8 made `rythme` panic (exit code 101).
#[cfg(unix)]
#[test]
fn arguments_that_are_not_utf8() {
    use std::os::unix::ffi::OsStrExt;
    let name = std::ffi::OsStr::from_bytes(b"n\xffm.ry");
    for cmd in ["run", "check", "fmt"] {
        let out = std::process::Command::new(env!("CARGO_BIN_EXE_rythme")).arg(cmd).arg(name).env("RYTHME_LANG", "en").output().unwrap();
        let err = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(2), "{cmd}: {err}");
        assert!(err.contains("is not valid UTF-8") && !err.contains("panicked"), "{cmd}: {err}");
    }
}

/// `rythme fmt` reads files like the other commands, and `-x` is an unknown option.
#[test]
fn fmt_reads_files_like_the_other_commands() {
    let dir = temp_dir("fmt");
    let file = dir.join("utf16.ry");
    std::fs::write(&file, b"\xff\xfep\x00").unwrap();
    let r = rythme(&["fmt", file.to_str().unwrap()]);
    assert_eq!(r.code, 1, "{}", r.stderr);
    assert!(r.stderr.contains("is not text in the UTF-8 encoding") && r.stderr.contains("UTF-16"), "{}", r.stderr);
    let r = rythme(&["fmt", "-x"]);
    assert_eq!(r.code, 2, "{}", r.stderr);
    assert!(r.stderr.contains("unknown option `-x`"), "{}", r.stderr);
    let r = rythme(&["completions", "tcsh"]);
    assert!(r.stderr.contains("unknown shell `tcsh`. Write one of"), "{}", r.stderr);
    let _ = std::fs::remove_dir_all(&dir);
}

// ----- the package index -------------------------------------------------------

/// `rythme.lock` gives git a commit and an address: a commit that is not in
/// hexadecimal, or an address that git would take as an option, is refused.
#[test]
fn a_lock_file_cannot_give_git_options() {
    let dir = temp_dir("lock");
    std::fs::write(dir.join("rythme.toml"), "name = \"app\"\nversion = \"0.1.0\"\n").unwrap();
    for (url, commit) in [("https://example.com/a", "--output=x"), ("--upload-pack=touch x", "0123abcd")] {
        let lock = format!("[[package]]\nname = \"a\"\ngit = \"{url}\"\ncommit = \"{commit}\"\n");
        std::fs::write(dir.join("rythme.lock"), lock).unwrap();
        assert!(rythme::manifest::load_lock(&dir).is_err(), "{url} {commit}");
    }
    let lock = "[[package]]\nname = \"a\"\ngit = \"https://example.com/a\"\ncommit = \"0123abcd\"\n";
    std::fs::write(dir.join("rythme.lock"), lock).unwrap();
    assert_eq!(rythme::manifest::load_lock(&dir).unwrap().len(), 1);
    let _ = std::fs::remove_dir_all(&dir);
}
