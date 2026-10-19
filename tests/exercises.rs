//! The exercises of `docs/exercises/` and `rythme exercise`: every exercise
//! exists in every language (English and French in `docs/exercises/`, the
//! others in their catalog), is well formed, checks without a problem and
//! fails its tests until it is solved; its reference solution, in the hidden
//! folder `docs/exercises/.solutions/<code>/`, passes exactly the same tests.

mod common;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use common::Run;
use rythme::diagnostic::display_width;
use rythme::exercise::{tests_marker, to_do, Exercise, EXERCISES};
use rythme::lang::Lang;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/exercises")
}

/// The words that every exercise uses, in each language: the first word of
/// its first line, and the titles of its statement and of its hints.
const WORDS: &[(&str, &str, &str, &str)] = &[
    ("en", "Exercise", "# Statement", "# Hints"),
    ("fr", "Exercice", "# Énoncé", "# Indices"),
    ("es", "Ejercicio", "# Enunciado", "# Pistas"),
    ("de", "Übung", "# Aufgabe", "# Hinweise"),
    ("pt", "Exercício", "# Enunciado", "# Dicas"),
    ("zh", "练习", "# 题目", "# 提示"),
];

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

/// The version of an exercise in each language: (language, name, text).
fn versions(e: &Exercise) -> Vec<(Lang, &'static str, &'static str)> {
    Lang::all().map(|l| (l, e.name_in(l), e.text_in(l))).collect()
}

fn lang(code: &str) -> Lang {
    Lang::all().find(|l| l.code() == code).unwrap()
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-exercises-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The `.ry` files of a folder, sorted.
fn ry_files(dir: &Path) -> Vec<String> {
    let mut files: Vec<String> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".ry"))
        .collect();
    files.sort();
    files
}

#[test]
fn every_exercise_file_is_built_into_the_program() {
    assert!(EXERCISES.len() >= 30);
    for l in [Lang::En, Lang::Fr] {
        let listed: Vec<String> = EXERCISES.iter().map(|e| format!("{}.ry", e.name_in(l))).collect();
        assert_eq!(
            ry_files(&root().join(l.code())),
            listed,
            "docs/exercises/{} and `EXERCISES` in src/exercise.rs must list the same files, in order",
            l.code()
        );
    }
    // A reference solution for each exercise, in each language.
    for l in Lang::all() {
        let listed: Vec<String> = EXERCISES.iter().map(|e| format!("{}.ry", e.name_in(l))).collect();
        assert_eq!(ry_files(&root().join(".solutions").join(l.code())), listed, "docs/exercises/.solutions/{}", l.code());
    }
}

#[test]
fn every_language_translates_every_exercise() {
    for l in Lang::all() {
        assert!(WORDS.iter().any(|w| w.0 == l.code()), "{}: add the words of its exercises to `WORDS`", l.code());
        if l == Lang::En {
            continue;
        }
        for e in EXERCISES {
            assert_ne!(e.text_in(l), e.en_text, "{}: {} is not translated (see tests/i18n.rs)", l.code(), e.en);
        }
        assert_ne!(tests_marker(l), tests_marker(Lang::En), "{}: the tests marker is not translated", l.code());
    }
}

#[test]
fn the_names_of_the_exercises_are_simple_and_distinct() {
    // A name, or a name without its number, finds one exercise in any language.
    let mut owner: HashMap<&str, usize> = HashMap::new();
    for (i, e) in EXERCISES.iter().enumerate() {
        for (l, name, _) in versions(e) {
            assert!(
                name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{}: `{name}` must be written with a-z, 0-9 and `-`, like the other file names",
                l.code()
            );
            for key in [name, &name[3..]] {
                let first = *owner.entry(key).or_insert(i);
                assert_eq!(first, i, "`{key}` names two exercises: {} and {}", EXERCISES[first].en, e.en);
            }
            assert_eq!(rythme::exercise::find(name).map(|f| f.en), Some(e.en), "{name}");
        }
    }
}

#[test]
fn every_exercise_is_well_formed() {
    for (i, e) in EXERCISES.iter().enumerate() {
        let number = format!("{:02}", i + 1);
        let tests = e.en_text.matches("\ntest \"").count();
        let to_dos = e.en_text.matches(&format!("fail({:?})", to_do(Lang::En))).count();
        for (l, name, text) in versions(e) {
            let (_, word, statement, hints) = WORDS.iter().find(|w| w.0 == l.code()).unwrap();
            let marker = tests_marker(l);
            let at = format!("{} {name}", l.code());
            assert!(name.starts_with(&format!("{number}-")), "{at}: the number must be {number}");
            let first = text.lines().next().unwrap();
            assert!(first.starts_with(&format!("# {word} {number} · ")), "{at}: first line `{first}`");
            assert!(["★☆☆", "★★☆", "★★★"].iter().any(|v| first.ends_with(&format!("  {v}"))), "{at}: level of `{first}`");
            assert_eq!(text.matches(marker).count(), 1, "{at}: the line `{marker}` must appear once");
            assert!(text.contains(statement) && text.contains(hints), "{at}: a statement and hints");
            assert!(text.contains(&format!("rythme exercise check {name}.ry")), "{at}: how to check it");
            assert!(text[text.find(marker).unwrap()..].contains("\ntest \""), "{at}: tests after `{marker}`");
            assert!(rythme::format::format(text).is_ok_and(|f| f == text), "{at}: `rythme fmt` changes it");
            assert_eq!(text.matches("\ntest \"").count(), tests, "{at}: the same tests in every language");
            let fail = format!("fail({:?})", to_do(l));
            assert_eq!(text.matches(&fail).count(), to_dos, "{at}: the functions to write say `{fail}`");
        }
    }
}

#[test]
fn every_exercise_checks_cleanly_and_fails_until_solved() {
    let dir = temp_dir("unsolved");
    for e in EXERCISES {
        for (l, name, text) in versions(e) {
            let file = format!("{}-{name}.ry", l.code());
            std::fs::write(dir.join(&file), text).unwrap();
            let r = rythme_in(&dir, l.code(), &["check", &file]);
            assert_eq!(r.code, 0, "{file}:\n{}", r.stderr);
            assert!(r.stderr.is_empty(), "{file} must check without any warning:\n{}", r.stderr);
            let r = rythme_in(&dir, l.code(), &["test", &file]);
            assert_eq!(r.code, 1, "{file}: its tests must fail before it is solved\n{}", r.stdout);
        }
    }
    std::fs::remove_dir_all(&dir).ok();
}

/// Every language × every exercise: the reference solution passes the tests
/// of the exercise, unchanged.
#[test]
fn every_reference_solution_passes_the_tests_of_its_exercise() {
    let dir = root();
    for e in EXERCISES {
        for (l, name, _) in versions(e) {
            let file = format!(".solutions/{}/{name}.ry", l.code());
            let solution = std::fs::read_to_string(dir.join(&file)).unwrap_or_else(|_| panic!("{file} is missing"));
            let tests = e.tests_in(l).unwrap();
            let marker = tests_marker(l);
            let given = solution.find(marker).map(|i| &solution[i..]).unwrap_or_default();
            assert!(given.starts_with(tests), "{file}: the tests must be those of the exercise (more may follow)");
            let first = e.text_in(l).lines().next();
            assert_eq!(solution.lines().next(), first, "{file}: the first line of the exercise");
            assert!(rythme::format::format(&solution).is_ok_and(|f| f == solution), "{file}: `rythme fmt` changes it");
            let r = rythme_in(&dir, l.code(), &["check", &file]);
            assert_eq!(r.code, 0, "{file}:\n{}", r.stderr);
            assert!(r.stderr.is_empty(), "{file} must check without any warning:\n{}", r.stderr);
            let r = rythme_in(&dir, l.code(), &["test", &file]);
            assert_eq!(r.code, 0, "{file} must pass:\n{}\n{}", r.stdout, r.stderr);
        }
    }
}

#[test]
fn exercise_list_and_start() {
    let dir = temp_dir("start");
    let r = rythme_in(&dir, "en", &["exercise", "list"]);
    assert_eq!(r.code, 0);
    assert!(
        r.stdout.contains("01  First values") && r.stdout.contains("30-testing.ry") && r.stdout.contains("36-minimax.ry"),
        "{}",
        r.stdout
    );
    let r = rythme_in(&dir, "fr", &["exercice"]);
    assert!(r.stdout.contains("01  Premières valeurs") && r.stdout.contains("30-tests.ry"), "{}", r.stdout);
    let r = rythme_in(&dir, "de", &["exercise", "list"]);
    assert!(r.stdout.contains("01  Erste Werte") && r.stdout.contains("28-fragezeichen.ry"), "{}", r.stdout);
    // The levels line up, also when the titles are in Chinese.
    for code in ["en", "zh"] {
        let r = rythme_in(&dir, code, &["exercise", "list"]);
        let columns: Vec<usize> = r
            .stdout
            .lines()
            .filter(|line| line.starts_with("  ") && line.as_bytes().get(2).is_some_and(u8::is_ascii_digit))
            .filter_map(|line| line.find('★').map(|i| display_width(&line[..i])))
            .collect();
        assert_eq!(columns.len(), EXERCISES.len(), "{code}:\n{}", r.stdout);
        assert!(columns.windows(2).all(|w| w[0] == w[1]), "{code}: the levels must line up\n{}", r.stdout);
    }

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

    // The other languages: their file, under their name, found by any name.
    let r = rythme_in(&dir, "es", &["exercise", "start", "textos"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.contains("fail(\"por hacer\")") && r.stdout.contains("rythme exercise check 02-textos.ry"), "{}", r.stdout);
    assert_eq!(std::fs::read_to_string(dir.join("02-textos.ry")).unwrap(), EXERCISES[1].text_in(lang("es")));
    let r = rythme_in(&dir, "pt", &["exercise", "start", "02-texte.ry"]);
    assert_eq!(r.code, 1, "the Portuguese file has the same name: the Spanish one is kept");
    assert_eq!(std::fs::read_to_string(dir.join("02-textos.ry")).unwrap(), EXERCISES[1].text_in(lang("es")));
    let r = rythme_in(&dir, "de", &["exercise", "start", "24"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(std::fs::read_to_string(dir.join("24-baeume.ry")).unwrap().starts_with("# Übung 24 · Bäume  ★★★\n"));
    let r = rythme_in(&dir, "zh", &["exercise", "start", "1"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(std::fs::read_to_string(dir.join("01-values.ry")).unwrap().starts_with("# 练习 01 · 最初的值  ★☆☆\n"));

    let r = rythme_in(&dir, "en", &["exercise", "start", "37"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("from 1 to 36"), "{}", r.stderr);
    let r = rythme_in(&dir, "en", &["exercise", "start", "recursoin"]);
    assert!(r.stderr.contains("did you mean `recursion`?"), "{}", r.stderr);
    let r = rythme_in(&dir, "en", &["exercise", "start", "rekursoin"]);
    assert!(r.stderr.contains("did you mean `rekursion`?"), "{}", r.stderr);
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
    for (code, hint) in
        [("es", "`fail(\"por hacer\")`"), ("de", "`fail(\"zu tun\")`"), ("pt", "`fail(\"a fazer\")`"), ("zh", "`fail(\"待完成\")`")]
    {
        let here = dir.join(code);
        std::fs::create_dir_all(&here).unwrap();
        let r = rythme_in(&here, code, &["exercise", "start", "3"]);
        assert_eq!(r.code, 0, "{code}: {}", r.stderr);
        let r = rythme_in(&here, code, &["exercise", "check", "3"]);
        assert_eq!(r.code, 1, "{code}");
        assert!(r.stdout.contains(hint), "{code}: {}", r.stdout);
    }

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
    assert!(r.stdout.contains("rythme exercise start 31"), "{}", r.stdout);
    let solution = root().join(".solutions/fr/36-minimax.ry");
    std::fs::copy(solution, dir.join("36-minimax.ry")).unwrap();
    let r = rythme_in(&dir, "fr", &["exercise", "check", "36-minimax.ry"]);
    assert_eq!(r.code, 0, "{}", r.stdout);
    assert!(r.stdout.contains("C'était le dernier exercice"), "{}", r.stdout);
    let solution = root().join(".solutions/de/07-rekursion.ry");
    std::fs::copy(solution, dir.join("07-rekursion.ry")).unwrap();
    let r = rythme_in(&dir, "de", &["exercise", "check", "7"]);
    assert_eq!(r.code, 0, "{}", r.stdout);
    assert!(r.stdout.contains("rythme exercise start 08"), "{}", r.stdout);

    // Tests changed to pass: not solved.
    let text = std::fs::read_to_string(dir.join("01-values.ry")).unwrap().replace("3600)", "seconds_per_hour)");
    std::fs::write(dir.join("01-values.ry"), text).unwrap();
    let r = rythme_in(&dir, "en", &["exercise", "check", "01-values.ry"]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("The tests of the exercise were changed"), "{}", r.stdout);
    let text = std::fs::read_to_string(dir.join("07-rekursion.ry")).unwrap().replace("3628800)", "fakultaet(10))");
    std::fs::write(dir.join("07-rekursion.ry"), text).unwrap();
    let r = rythme_in(&dir, "de", &["exercise", "check", "07-rekursion.ry"]);
    assert_eq!(r.code, 1, "{}", r.stdout);

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
