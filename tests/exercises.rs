//! The exercises of `docs/exercises/` and `rythme exercise`: every exercise
//! exists in English and French, is well formed, checks without a problem and
//! fails its tests until it is solved; its reference solution, in the hidden
//! folder `docs/exercises/.solutions/`, passes exactly the same tests.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use common::Run;
use rythme::exercise::{Exercise, EXERCISES, TESTS_MARKER_EN, TESTS_MARKER_FR};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/exercises")
}

/// Runs `rythme` in a folder, with the messages in a language.
fn rythme_in(dir: &Path, lang: &str, args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(args)
        .current_dir(dir)
        .env("RYTHME_LANG", lang)
        .env("NO_COLOR", "1")
        .output()
        .expect("run rythme");
    Run {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code: out.status.code().unwrap_or(-1),
    }
}

/// The two versions of each exercise: (language, name, text, tests marker).
fn versions(e: &Exercise) -> [(&'static str, &'static str, &'static str, &'static str); 2] {
    [("en", e.en, e.en_text, TESTS_MARKER_EN), ("fr", e.fr, e.fr_text, TESTS_MARKER_FR)]
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-exercises-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn every_exercise_file_is_built_into_the_program() {
    assert!(EXERCISES.len() >= 30);
    for (lang, pick) in [("en", (|e: &Exercise| e.en) as fn(&Exercise) -> &'static str), ("fr", |e: &Exercise| e.fr)] {
        let mut files: Vec<String> = std::fs::read_dir(root().join(lang))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".ry"))
            .collect();
        files.sort();
        let listed: Vec<String> = EXERCISES.iter().map(|e| format!("{}.ry", pick(e))).collect();
        assert_eq!(files, listed, "docs/exercises/{lang} and `EXERCISES` in src/exercise.rs must list the same files, in order");
    }
}

#[test]
fn every_exercise_is_well_formed() {
    for (i, e) in EXERCISES.iter().enumerate() {
        let number = format!("{:02}", i + 1);
        for (lang, name, text, marker) in versions(e) {
            assert!(name.starts_with(&format!("{number}-")), "{name}: the number must be {number}");
            let word = if lang == "en" { "Exercise" } else { "Exercice" };
            let first = text.lines().next().unwrap();
            assert!(first.starts_with(&format!("# {word} {number} · ")), "{name}: first line `{first}`");
            assert!(["★☆☆", "★★☆", "★★★"].iter().any(|l| first.ends_with(&format!("  {l}"))), "{name}: level of `{first}`");
            assert_eq!(text.matches(marker).count(), 1, "{name}: the line `{marker}` must appear once");
            let (statement, hints) = if lang == "en" { ("# Statement", "# Hints") } else { ("# Énoncé", "# Indices") };
            assert!(text.contains(statement) && text.contains(hints), "{name}: a statement and hints");
            assert!(text.contains(&format!("rythme exercise check {name}.ry")), "{name}: how to check it");
            assert!(text[text.find(marker).unwrap()..].contains("\ntest \""), "{name}: tests after `{marker}`");
            assert!(rythme::format::format(text).is_ok_and(|f| f == text), "{name}: `rythme fmt` changes it");
        }
        assert_eq!(
            e.en_text.matches("\ntest \"").count(),
            e.fr_text.matches("\ntest \"").count(),
            "{}: same tests in both languages",
            e.en
        );
    }
}

#[test]
fn every_exercise_checks_cleanly_and_fails_until_solved() {
    let dir = root();
    for e in EXERCISES {
        for (lang, name, _, _) in versions(e) {
            let file = format!("{lang}/{name}.ry");
            let r = rythme_in(&dir, lang, &["check", &file]);
            assert_eq!(r.code, 0, "{file}:\n{}", r.stderr);
            assert!(r.stderr.is_empty(), "{file} must check without any warning:\n{}", r.stderr);
            let r = rythme_in(&dir, lang, &["test", &file]);
            assert_eq!(r.code, 1, "{file}: its tests must fail before it is solved\n{}", r.stdout);
        }
    }
}

#[test]
fn every_reference_solution_passes_the_tests_of_its_exercise() {
    let dir = root();
    for e in EXERCISES {
        for (lang, name, text, marker) in versions(e) {
            let file = format!(".solutions/{lang}/{name}.ry");
            let solution = std::fs::read_to_string(dir.join(&file)).unwrap_or_else(|_| panic!("{file} is missing"));
            let tests = text[text.find(marker).unwrap()..].trim_end();
            let given = solution.find(marker).map(|i| &solution[i..]).unwrap_or_default();
            assert!(given.starts_with(tests), "{file}: the tests must be those of the exercise (more may follow)");
            assert!(rythme::format::format(&solution).is_ok_and(|f| f == solution), "{file}: `rythme fmt` changes it");
            let r = rythme_in(&dir, lang, &["check", &file]);
            assert_eq!(r.code, 0, "{file}:\n{}", r.stderr);
            assert!(r.stderr.is_empty(), "{file} must check without any warning:\n{}", r.stderr);
            let r = rythme_in(&dir, lang, &["test", &file]);
            assert_eq!(r.code, 0, "{file} must pass:\n{}\n{}", r.stdout, r.stderr);
        }
    }
}

#[test]
fn exercise_list_and_start() {
    let dir = temp_dir("start");
    let r = rythme_in(&dir, "en", &["exercise", "list"]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("01  First values") && r.stdout.contains("30-testing.ry"), "{}", r.stdout);
    let r = rythme_in(&dir, "fr", &["exercice"]);
    assert!(r.stdout.contains("01  Premières valeurs") && r.stdout.contains("30-tests.ry"), "{}", r.stdout);

    let r = rythme_in(&dir, "en", &["exercise", "start", "7"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.contains("rythme exercise check 07-recursion.ry"), "{}", r.stdout);
    assert_eq!(std::fs::read_to_string(dir.join("07-recursion.ry")).unwrap(), EXERCISES[6].en_text);
    let r = rythme_in(&dir, "en", &["exercise", "start", "recursion"]);
    assert_eq!(r.code, 1, "an existing file is kept");
    assert!(r.stderr.contains("already exists"), "{}", r.stderr);
    let r = rythme_in(&dir, "fr", &["exercise", "start", "07-recursion.ry"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(std::fs::read_to_string(dir.join("07-recursivite.ry")).unwrap(), EXERCISES[6].fr_text);

    let r = rythme_in(&dir, "en", &["exercise", "start", "31"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("from 1 to 30"), "{}", r.stderr);
    let r = rythme_in(&dir, "en", &["exercise", "start", "recursoin"]);
    assert!(r.stderr.contains("did you mean `recursion`?"), "{}", r.stderr);
    let r = rythme_in(&dir, "en", &["exercise", "strat"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("did you mean `rythme exercise start`?"), "{}", r.stderr);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn exercise_check_gives_friendly_feedback() {
    let dir = temp_dir("check");
    // Not solved yet.
    rythme_in(&dir, "en", &["exercise", "start", "1"]);
    let r = rythme_in(&dir, "en", &["exercise", "check", "01-values.ry"]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("✗ 0 of 4 tests pass. Keep going!"), "{}", r.stdout);
    let r = rythme_in(&dir, "fr", &["exercise", "start", "2"]);
    assert_eq!(r.code, 0);
    let r = rythme_in(&dir, "fr", &["exercise", "check", "2"]);
    assert!(r.stdout.contains("remplacez chaque `fail(\"à faire\")`"), "{}", r.stdout);

    // Solved: the next exercise is proposed.
    let solution = root().join(".solutions/en/01-values.ry");
    std::fs::copy(solution, dir.join("01-values.ry")).unwrap();
    let r = rythme_in(&dir, "en", &["exercise", "check", "01-values"]);
    assert_eq!(r.code, 0, "{}", r.stdout);
    assert!(r.stdout.contains("✓ Well done: all 4 tests pass!") && r.stdout.contains("rythme exercise start 02"), "{}", r.stdout);
    let solution = root().join(".solutions/fr/30-tests.ry");
    std::fs::copy(solution, dir.join("30-tests.ry")).unwrap();
    let r = rythme_in(&dir, "fr", &["exercise", "check", "30-tests.ry"]);
    assert_eq!(r.code, 0, "{}", r.stdout);
    assert!(r.stdout.contains("C'était le dernier exercice"), "{}", r.stdout);

    // Tests changed to pass: not solved.
    let text = std::fs::read_to_string(dir.join("01-values.ry")).unwrap().replace("3600)", "seconds_per_hour)");
    std::fs::write(dir.join("01-values.ry"), text).unwrap();
    let r = rythme_in(&dir, "en", &["exercise", "check", "01-values.ry"]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("The tests of the exercise were changed"), "{}", r.stdout);

    // Errors before the tests.
    std::fs::write(dir.join("broken.ry"), "fn f(x) = \n").unwrap();
    let r = rythme_in(&dir, "en", &["exercise", "check", "broken.ry"]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("The file has errors"), "{}", r.stdout);
    let r = rythme_in(&dir, "en", &["exercise", "check", "missing.ry"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("not found"), "{}", r.stderr);
    std::fs::remove_dir_all(&dir).ok();
}
