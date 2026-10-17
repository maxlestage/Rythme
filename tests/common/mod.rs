#![allow(dead_code)]

use std::process::{Command, Output};

pub struct Run {
    pub stdout: String,
    pub stderr: String,
    pub code: i32,
}

fn finish(out: Output) -> Run {
    Run {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code: out.status.code().unwrap_or(-1),
    }
}

pub fn rythme_lang(lang: &str, args: &[&str]) -> Run {
    let out =
        Command::new(env!("CARGO_BIN_EXE_rythme")).args(args).env("RYTHME_LANG", lang).env("NO_COLOR", "1").output().expect("run rythme");
    finish(out)
}

pub fn rythme(args: &[&str]) -> Run {
    rythme_lang("en", args)
}

/// Runs code and checks what it prints.
#[track_caller]
pub fn ok(code: &str, expected: &str) {
    let r = rythme(&["-e", code]);
    assert_eq!(r.code, 0, "code:\n{code}\nstderr:\n{}", r.stderr);
    assert_eq!(r.stdout.trim_end(), expected, "code:\n{code}\nstderr:\n{}", r.stderr);
}

/// Runs code and checks that it fails with the given error code.
#[track_caller]
pub fn fails(code: &str, error: &str) -> Run {
    let r = rythme(&["-e", code]);
    assert_eq!(r.code, 1, "code:\n{code}\nstdout:\n{}\nstderr:\n{}", r.stdout, r.stderr);
    assert!(r.stderr.contains(&format!("error[{error}]")), "expected {error} for:\n{code}\nstderr:\n{}", r.stderr);
    r
}

/// Runs code and checks that it warns with the given code but still runs.
#[track_caller]
pub fn warns(code: &str, warning: &str) -> Run {
    let r = rythme(&["-e", code]);
    assert_eq!(r.code, 0, "code:\n{code}\nstderr:\n{}", r.stderr);
    assert!(r.stderr.contains(&format!("warning[{warning}]")), "expected {warning} for:\n{code}\nstderr:\n{}", r.stderr);
    r
}
