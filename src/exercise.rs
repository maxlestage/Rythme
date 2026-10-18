//! `rythme exercise`: the graded exercises of `docs/exercises/`, built into
//! the program. `list` shows them, `start` copies one into the current
//! folder, `check` runs its tests and explains what to do next.

use std::path::{Path, PathBuf};

use crate::driver::Driver;
use crate::interp::Mode;
use crate::io::{err_line, out_line};
use crate::lang::is_fr;
use crate::suggest;

/// One exercise: its file name (without `.ry`) and its text, in each language.
pub struct Exercise {
    pub en: &'static str,
    pub fr: &'static str,
    pub en_text: &'static str,
    pub fr_text: &'static str,
}

macro_rules! exercises {
    ($(($en:literal, $fr:literal)),* $(,)?) => {
        &[$(Exercise {
            en: $en,
            fr: $fr,
            en_text: include_str!(concat!("../docs/exercises/en/", $en, ".ry")),
            fr_text: include_str!(concat!("../docs/exercises/fr/", $fr, ".ry")),
        }),*]
    };
}

/// Every exercise, from the easiest; the number of an exercise is its position.
pub const EXERCISES: &[Exercise] = exercises![
    ("01-values", "01-valeurs"),
    ("02-texts", "02-textes"),
    ("03-arithmetic", "03-calculs"),
    ("04-choosing", "04-choisir"),
    ("05-logic", "05-logique"),
    ("06-cases", "06-cas"),
    ("07-recursion", "07-recursivite"),
    ("08-digits", "08-chiffres"),
    ("09-accumulators", "09-accumulateurs"),
    ("10-divisors", "10-diviseurs"),
    ("11-list-patterns", "11-motifs-de-listes"),
    ("12-building-lists", "12-construire-des-listes"),
    ("13-library", "13-bibliotheque"),
    ("14-anonymous-functions", "14-fonctions-anonymes"),
    ("15-functions-as-values", "15-fonctions-valeurs"),
    ("16-fold", "16-fold"),
    ("17-pipelines", "17-pipelines"),
    ("18-text-processing", "18-traitement-de-textes"),
    ("19-sorting", "19-tris"),
    ("20-records", "20-enregistrements"),
    ("21-lists-of-records", "21-listes-d-enregistrements"),
    ("22-own-types", "22-types"),
    ("23-match", "23-match"),
    ("24-trees", "24-arbres"),
    ("25-expressions", "25-expressions"),
    ("26-option", "26-option"),
    ("27-result", "27-result"),
    ("28-question-mark", "28-point-d-interrogation"),
    ("29-word-counts", "29-compter-les-mots"),
    ("30-testing", "30-tests"),
];

/// The line that starts the tests of an exercise, in each language.
pub const TESTS_MARKER_EN: &str = "# --- Tests (do not change them) ---";
pub const TESTS_MARKER_FR: &str = "# --- Tests (ne les modifiez pas) ---";

impl Exercise {
    /// The file name, in the language of the messages.
    pub fn name(&self) -> &'static str {
        if is_fr() {
            self.fr
        } else {
            self.en
        }
    }

    /// The file, in the language of the messages.
    pub fn text(&self) -> &'static str {
        if is_fr() {
            self.fr_text
        } else {
            self.en_text
        }
    }

    /// The number, as written in the file name: "07".
    pub fn number(&self) -> &'static str {
        &self.en[..2]
    }

    /// The title and the level, from the first line of the file:
    /// `# Exercise 07 · Recursion  ★★☆` gives ("Recursion", "★★☆").
    pub fn title(&self) -> (&'static str, &'static str) {
        let first = self.text().lines().next().unwrap_or("");
        let after = first.split_once("· ").map(|(_, t)| t).unwrap_or(first);
        match after.rsplit_once("  ") {
            Some((title, level)) => (title.trim(), level.trim()),
            None => (after.trim(), ""),
        }
    }

    /// The tests of the exercise, as written in the file given to students.
    fn tests_in(text: &str) -> Option<&str> {
        let start = text.find(TESTS_MARKER_EN).or_else(|| text.find(TESTS_MARKER_FR))?;
        Some(text[start..].trim_end())
    }
}

/// The exercise named by a number (`7`, `07`), or by its file name in either
/// language, with or without `.ry` and with or without its number.
pub fn find(name: &str) -> Option<&'static Exercise> {
    let name = name.trim().trim_end_matches(".ry").to_lowercase();
    if let Ok(n) = name.parse::<usize>() {
        return n.checked_sub(1).and_then(|i| EXERCISES.get(i));
    }
    EXERCISES.iter().find(|e| [e.en, e.fr].iter().any(|n| *n == name || n[3..] == name))
}

/// `rythme exercise ...`.
pub fn command(args: &[String]) -> i32 {
    let sub = args.first().map(String::as_str);
    match sub {
        None | Some("list" | "liste" | "lister" | "ls") => {
            list();
            0
        }
        Some("start" | "commencer" | "new" | "nouveau") => match args.get(1) {
            Some(name) => start(name),
            None => usage(t!(
                "which exercise? Example: rythme exercise start 1   (`rythme exercise list` shows them all)",
                "quel exercice ? Exemple : rythme exercise start 1   (`rythme exercise list` les montre tous)"
            )),
        },
        Some("check" | "verifier" | "vérifier" | "test" | "tester") => match args.get(1) {
            Some(file) => check(file),
            None => usage(t!(
                "which file? Example: rythme exercise check 01-values.ry",
                "quel fichier ? Exemple : rythme exercise check 01-valeurs.ry"
            )),
        },
        Some(other) => {
            let mut msg = t!("unknown action `{other}` for `rythme exercise`", "action inconnue `{other}` pour `rythme exercise`");
            if let Some(best) = suggest::closest(other, ["list", "start", "check"]) {
                msg.push_str(&t!("\ndid you mean `rythme exercise {best}`?", "\nvouliez-vous dire `rythme exercise {best}` ?"));
            }
            msg.push_str(&t!(
                "\nthe actions are: list, start NAME, check FILE (see `rythme help exercises`)",
                "\nles actions sont : list, start NOM, check FICHIER (voir `rythme help exercises`)"
            ));
            usage(msg)
        }
    }
}

fn usage(msg: String) -> i32 {
    err_line(&msg);
    2
}

fn list() {
    out_line(&t!(
        "{} exercises for a first course, from the first values to tests, from ★☆☆ (easy) to ★★★.",
        "{} exercices pour un premier cours, des premières valeurs aux tests, de ★☆☆ (facile) à ★★★.",
        EXERCISES.len()
    ));
    out_line("");
    let width = EXERCISES.iter().map(|e| e.title().0.chars().count()).max().unwrap_or(0);
    for e in EXERCISES {
        let (title, level) = e.title();
        let pad = " ".repeat(width - title.chars().count());
        out_line(&format!("  {}  {title}{pad}  {level}  {}.ry", e.number(), e.name()));
    }
    out_line("");
    out_line(&t!(
        "Start one:       rythme exercise start 1\nCheck your work: rythme exercise check {}.ry",
        "Commencer :      rythme exercise start 1\nVérifier :       rythme exercise check {}.ry",
        EXERCISES[0].name()
    ));
}

fn unknown_exercise(name: &str) -> i32 {
    let mut msg = t!("there is no exercise `{name}`", "il n'y a pas d'exercice `{name}`");
    let names = EXERCISES.iter().flat_map(|e| [e.en, e.fr, &e.en[3..], &e.fr[3..]]);
    if let Some(best) = suggest::closest(name.trim_end_matches(".ry"), names) {
        msg.push_str(&t!("\ndid you mean `{best}`?", "\nvouliez-vous dire `{best}` ?"));
    }
    msg.push_str(&t!(
        "\nthe exercises are numbered from 1 to {}: `rythme exercise list` shows them",
        "\nles exercices sont numérotés de 1 à {} : `rythme exercise list` les montre",
        EXERCISES.len()
    ));
    usage(msg)
}

fn start(name: &str) -> i32 {
    let Some(e) = find(name) else { return unknown_exercise(name) };
    let file = format!("{}.ry", e.name());
    if Path::new(&file).exists() {
        err_line(&t!(
            "`{file}` already exists here: open it to continue, or rename it to start again",
            "`{file}` existe déjà ici : ouvrez-le pour continuer, ou renommez-le pour recommencer"
        ));
        return 1;
    }
    if let Err(error) = std::fs::write(&file, e.text()) {
        err_line(&t!("cannot write `{file}`: {error}", "impossible d'écrire `{file}` : {error}"));
        return 1;
    }
    let (title, level) = e.title();
    out_line(&t!(
        "Created {file}: exercise {} · {title}  {level}\n\nOpen it, read the statement and the hints at the top, and replace each\n`fail(\"to do\")` with your code. Then check your work:\n\n  rythme exercise check {file}",
        "{file} créé : exercice {} · {title}  {level}\n\nOuvrez-le, lisez l'énoncé et les indices en haut, et remplacez chaque\n`fail(\"à faire\")` par votre code. Puis vérifiez votre travail :\n\n  rythme exercise check {file}",
        e.number()
    ));
    0
}

/// The file to check: the name given, with `.ry` added if needed, or the
/// file of the exercise with this number or name in the current folder.
fn find_file(name: &str) -> Option<PathBuf> {
    let candidates = [PathBuf::from(name), PathBuf::from(format!("{name}.ry"))];
    if let Some(p) = candidates.into_iter().find(|p| p.is_file()) {
        return Some(p);
    }
    let e = find(name)?;
    [e.en, e.fr].iter().map(|n| PathBuf::from(format!("{n}.ry"))).find(|p| p.is_file())
}

fn check(name: &str) -> i32 {
    let Some(path) = find_file(name) else {
        let mut msg = t!("file `{name}` not found", "fichier `{name}` introuvable");
        if let Some(e) = find(name) {
            msg.push_str(&t!(
                "\nto start this exercise: rythme exercise start {}",
                "\npour commencer cet exercice : rythme exercise start {}",
                e.number()
            ));
        } else if let Ok(entries) = std::fs::read_dir(".") {
            let files: Vec<String> =
                entries.flatten().map(|e| e.file_name().to_string_lossy().to_string()).filter(|n| n.ends_with(".ry")).collect();
            if let Some(best) = suggest::closest(name, files.iter().map(String::as_str)) {
                msg.push_str(&t!("\ndid you mean `{best}`?", "\nvouliez-vous dire `{best}` ?"));
            }
        }
        err_line(&msg);
        return 1;
    };
    let source = std::fs::read_to_string(&path).unwrap_or_default();
    let stem = path.file_stem().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default();
    let position = EXERCISES.iter().position(|e| e.en == stem || e.fr == stem);
    let exercise = position.map(|i| &EXERCISES[i]);

    let mut driver = Driver::new();
    let counts = match driver.load_file(&path) {
        Some(module) if driver.execute(&module, Mode::Test).is_ok() => Some(driver.run_tests_counted()),
        _ => None,
    };
    out_line("");
    let Some((total, failed)) = counts else {
        out_line(&t!(
            "✗ The file has errors, so its tests could not run.\n  Fix the first error above, then check again.",
            "✗ Le fichier a des erreurs : ses tests n'ont pas pu être lancés.\n  Corrigez la première erreur ci-dessus, puis vérifiez de nouveau."
        ));
        return 1;
    };
    // The tests of the exercise must still be there, unchanged.
    let changed =
        exercise.is_some_and(|e| [e.en_text, e.fr_text].iter().filter_map(|t| Exercise::tests_in(t)).all(|t| !source.contains(t)));
    if total == 0 {
        out_line(&t!(
            "✗ This file has no tests. An exercise comes with its tests: is it the right file?",
            "✗ Ce fichier n'a pas de tests. Un exercice est livré avec ses tests : est-ce le bon fichier ?"
        ));
        return 1;
    }
    let passed = total - failed;
    if failed == 0 && !changed {
        out_line(&t!("✓ Well done: all {total} tests pass!", "✓ Bravo : les {total} tests passent !"));
        match position.map(|i| EXERCISES.get(i + 1)) {
            Some(Some(next)) => {
                let (title, level) = next.title();
                out_line(&t!(
                    "  Next exercise: {title}  {level}\n    rythme exercise start {}",
                    "  Exercice suivant : {title}  {level}\n    rythme exercise start {}",
                    next.number()
                ));
            }
            Some(None) => out_line(&t!(
                "  That was the last exercise: you have finished them all. Congratulations!",
                "  C'était le dernier exercice : vous les avez tous finis. Félicitations !"
            )),
            None => {}
        }
        return 0;
    }
    if failed == 0 {
        out_line(&t!("✗ All {total} tests pass, but...", "✗ Les {total} tests passent, mais..."));
    } else {
        out_line(&t!("✗ {passed} of {total} tests pass. Keep going!", "✗ {passed} test(s) sur {total} passent. Continuez !"));
    }
    if changed {
        let number = exercise.map(Exercise::number).unwrap_or_default();
        out_line(&t!(
            "  - The tests of the exercise were changed: the exercise is solved when its own tests\n    pass. `rythme exercise start {number}` gives a fresh copy (rename your file first).",
            "  - Les tests de l'exercice ont été modifiés : l'exercice est réussi quand ses propres\n    tests passent. `rythme exercise start {number}` en donne une copie neuve (renommez d'abord votre fichier)."
        ));
    }
    if failed > 0 {
        if source.contains("fail(\"to do\")") || source.contains("fail(\"à faire\")") {
            out_line(&t!(
                "  - Some functions are still to do: replace each `fail(\"to do\")` with your code.",
                "  - Des fonctions restent à faire : remplacez chaque `fail(\"à faire\")` par votre code."
            ));
        }
        out_line(&t!(
            "  - Start with the first failure above: compare what you got with what was expected.\n  - Stuck? Read the hints at the top of the file, or `rythme help NAME` for a function.",
            "  - Commencez par le premier échec ci-dessus : comparez ce que vous obtenez à ce qui est attendu.\n  - Bloqué ? Lisez les indices en haut du fichier, ou `rythme help NOM` pour une fonction."
        ));
    }
    1
}
