//! Every program in `examples/` must print exactly its `.out` file.

mod common;

use std::path::{Path, PathBuf};

fn outputs(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("examples folder") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            outputs(&path, found);
        } else if path.extension().is_some_and(|e| e == "out") {
            found.push(path);
        }
    }
}

#[test]
fn examples_print_their_expected_output() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let mut found = Vec::new();
    outputs(&dir, &mut found);
    found.sort();
    assert!(found.len() >= 10, "examples are missing");
    for out in found {
        let program = out.with_extension("ry");
        let expected = std::fs::read_to_string(&out).unwrap().replace("\r\n", "\n");
        let r = common::rythme(&[program.to_str().unwrap()]);
        assert_eq!(r.code, 0, "{} failed:\n{}", program.display(), r.stderr);
        assert_eq!(r.stdout.replace("\r\n", "\n"), expected, "{} printed something else", program.display());
        assert!(r.stderr.is_empty(), "{} printed warnings:\n{}", program.display(), r.stderr);
    }
}

#[test]
fn examples_have_no_warnings() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "ry") {
            let r = common::rythme(&["check", path.to_str().unwrap()]);
            assert_eq!(r.code, 0, "{}", r.stderr);
            assert!(r.stdout.contains("no problems found"), "{}: {}{}", path.display(), r.stdout, r.stderr);
        }
    }
}

#[test]
fn example_tests_pass() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/testing.ry");
    let r = common::rythme(&["test", path.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}{}", r.stdout, r.stderr);
    assert!(r.stdout.contains("3 passed, 0 failed"), "{}", r.stdout);
    assert!(!r.stdout.contains("42"), "top-level expressions must not run in test mode");
}
