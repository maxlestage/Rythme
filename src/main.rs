//! The `rythme` command.

use std::io::Write;
use std::path::Path;

use rythme::diagnostic::Diagnostic;
use rythme::driver::{find_script, Driver};
use rythme::interp::Mode;
use rythme::machine::{self, Format};
use rythme::project::Cmd;
use rythme::value::Value;
use rythme::{help, lang, repl, suggest, t};

/// Deep (non-tail) recursion needs a big stack.
const STACK_SIZE: usize = 1 << 30;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::panic::set_hook(Box::new(|info| {
        // Written directly: the hook must never fail, even if the error output is closed.
        let _ = writeln!(
            std::io::stderr(),
            "{}\n{info}",
            t!(
                "Rythme ran into an internal problem. This is a bug in Rythme, not in your program.\nPlease report it, with your program, at https://github.com/maxlestage/Rythme/issues",
                "Rythme a rencontré un problème interne. C'est un bug de Rythme, pas de votre programme.\nMerci de le signaler, avec votre programme, sur https://github.com/maxlestage/Rythme/issues"
            )
        );
    }));
    let thread_args = args.clone();
    let child = std::thread::Builder::new().name("rythme".into()).stack_size(STACK_SIZE).spawn(move || {
        // Keep a safety margin for the work done between two checks.
        rythme::interp::set_stack_budget(STACK_SIZE - (128 << 20));
        cli(thread_args)
    });
    let code = match child {
        Ok(handle) => handle.join().unwrap_or(101),
        // No big thread available: run here, with the default (small) budget.
        Err(_) => cli(args),
    };
    std::process::exit(code);
}

fn cli(mut args: Vec<String>) -> i32 {
    lang::set(lang::detect());
    // The language asked with `--lang`: the language server keeps it over the editor's.
    let mut chosen = None;
    // Options placed before the command.
    while let Some(first) = args.first() {
        if first == "--lang" || first.starts_with("--lang=") {
            let (value, used) = match first.strip_prefix("--lang=") {
                Some(v) => (Some(v.to_string()), 1),
                None => (args.get(1).cloned(), 2),
            };
            match value.as_deref().and_then(lang::parse) {
                Some(l) => {
                    lang::set(l);
                    chosen = Some(l);
                }
                None => {
                    let what = match value {
                        Some(v) => t!("unknown language `{v}`", "langue inconnue `{v}`"),
                        None => t!("`--lang` needs a language", "`--lang` attend une langue"),
                    };
                    let codes = lang::codes();
                    return usage_error(t!(
                        "{what}: write `--lang` and one of {codes}, like `rythme --lang fr hello.ry`",
                        "{what} : écrivez `--lang` et l'une de {codes}, comme `rythme --lang fr bonjour.ry`"
                    ));
                }
            }
            args.drain(..used.min(args.len()));
        } else if first == "--no-color" {
            std::env::set_var("NO_COLOR", "1");
            args.remove(0);
        } else {
            break;
        }
    }
    let Some(cmd) = args.first().cloned() else { return repl::run() };
    let mut rest: Vec<String> = args[1..].to_vec();
    // `--format json`, anywhere after these commands.
    let format = match cmd.as_str() {
        "check" | "verifier" | "vérifier" | "explain" | "expliquer" | "test" | "tester" => match machine::take_format(&mut rest) {
            Ok(f) => f,
            Err(code) => return code,
        },
        _ => Format::Text,
    };
    match cmd.as_str() {
        "repl" => repl::run(),
        "help" | "aide" | "--help" | "-h" => {
            let mut rest = rest;
            let format = match machine::take_format(&mut rest) {
                Ok(f) => f,
                Err(code) => return code,
            };
            if machine::take_flag(&mut rest, "--llms") {
                rest.insert(0, "llms".into());
            }
            match rest.first().map(String::as_str) {
                // The language for AI assistants, in one text (docs/llms-full.txt).
                Some("llms" | "llms-full" | "llms-full.txt") => {
                    rythme::io::out(&rythme::llms::full());
                    return 0;
                }
                Some("llms.txt") => {
                    rythme::io::out(&rythme::llms::index());
                    return 0;
                }
                _ if format == Format::Json => return machine::help_command(rest.first().map(String::as_str)),
                _ => {}
            }
            match rest.first() {
                None => rythme::io::out(&help::overview()),
                Some(name) => match help::topic(name) {
                    Some(text) => rythme::io::out_line(text.trim_end()),
                    None => {
                        rythme::io::err_line(&help::unknown_topic(name));
                        return 2;
                    }
                },
            }
            0
        }
        "explain" | "expliquer" if format == Format::Json => machine::explain_command(rest.first().map(String::as_str)),
        "explain" | "expliquer" => match rest.first() {
            None => {
                rythme::io::out_line(&t!(
                    "usage: rythme explain CODE   (codes: {})",
                    "utilisation : rythme explain CODE   (codes : {})",
                    help::CODES.join(" ")
                ));
                0
            }
            Some(code) => match help::explain(code) {
                Some(text) => {
                    rythme::io::out_line(&text);
                    0
                }
                None => {
                    rythme::io::err_line(&t!(
                        "unknown code `{code}`. Known codes: {}",
                        "code inconnu `{code}`. Codes connus : {}",
                        help::CODES.join(" ")
                    ));
                    2
                }
            },
        },
        "learn" | "apprendre" | "tutorial" | "tutoriel" => {
            help::learn();
            0
        }
        "version" | "--version" | "-V" => {
            rythme::io::out_line(&format!("rythme {}", rythme::VERSION));
            0
        }
        "run" | "lancer" => {
            let (opts, rest) = match run_options(&rest) {
                Ok(found) => found,
                Err(code) => return code,
            };
            let run = |file: &str, args: Vec<String>| match opts.format {
                Format::Json => machine::run_command(file, args, opts.seed, opts.sandbox),
                Format::Text => run_file(file, args, opts.seed, opts.sandbox),
            };
            match rest.first() {
                Some(file) => run(file, rest[1..].to_vec()),
                None => in_project(Cmd::Run, |files| run(&files[0], Vec::new())).unwrap_or_else(|| {
                    usage_error(t!("which file? Example: rythme run hello.ry", "quel fichier ? Exemple : rythme run bonjour.ry"))
                }),
            }
        }
        // With no file, at the root of a project: the files of the project with tests.
        "test" | "tester" if format == Format::Json => machine::test_command(&rest),
        "test" | "tester" => rythme::testing::command(&rest),
        "check" | "verifier" | "vérifier" if format == Format::Json => match rest.first() {
            Some(_) => machine::check_command(&rest),
            None => in_project(Cmd::Check, machine::check_command).unwrap_or_else(|| {
                usage_error(t!("which file? Example: rythme check hello.ry", "quel fichier ? Exemple : rythme check bonjour.ry"))
            }),
        },
        "check" | "verifier" | "vérifier" => match rest.first() {
            // Every file is checked, even after one with errors.
            Some(_) => rest.iter().map(|file| check_file(file)).fold(0, i32::max),
            None => in_project(Cmd::Check, |files| files.iter().map(|file| check_file(file)).fold(0, i32::max)).unwrap_or_else(|| {
                usage_error(t!("which file? Example: rythme check hello.ry", "quel fichier ? Exemple : rythme check bonjour.ry"))
            }),
        },
        "fmt" | "format" | "formater" if !has_target(&rest, &[]) => {
            in_project(Cmd::Fmt, |files| rythme::format::command(&[rest.clone(), files.to_vec()].concat()))
                .unwrap_or_else(|| rythme::format::command(&rest))
        }
        "fmt" | "format" | "formater" => rythme::format::command(&rest),
        "doc" | "documenter" if !has_target(&rest, &["-o", "--out", "--sortie"]) => {
            in_project(Cmd::Doc, |files| rythme::doc::command(&[rest.clone(), files.to_vec()].concat()))
                .unwrap_or_else(|| rythme::doc::command(&rest))
        }
        "doc" | "documenter" => rythme::doc::command(&rest),
        "new" | "nouveau" => rythme::project::new_command(&rest),
        "fix" | "corriger" => machine::fix_command(&rest),
        "install" | "installer" => rythme::project::install_command(&rest),
        "exercise" | "exercises" | "exercice" | "exercices" => rythme::exercise::command(&rest),
        "steps" | "etapes" | "étapes" => rythme::steps::command(&rest),
        "lsp" => rythme::lsp::run(&rest, chosen),
        "debug" | "deboguer" | "déboguer" => rythme::debug::term::command(&rest),
        "dap" => rythme::debug::dap::command(&rest),
        "-e" | "eval" => match rest.first() {
            Some(code) => eval_code(code, rest[1..].to_vec()),
            None => usage_error(t!("which code? Example: rythme -e 'print(1 + 2)'", "quel code ? Exemple : rythme -e 'print(1 + 2)'")),
        },
        _ => {
            if cmd.ends_with(".ry") || Path::new(&cmd).exists() || Path::new(&format!("{cmd}.ry")).is_file() {
                return run_file(&cmd, rest, None, false);
            }
            let commands = [
                "run", "check", "test", "fmt", "doc", "help", "explain", "learn", "repl", "version", "lsp", "exercise", "new", "install",
                "steps", "debug", "dap", "fix",
            ];
            let mut msg = t!("unknown command or file `{cmd}`", "commande ou fichier inconnu `{cmd}`");
            if let Some(best) = suggest::closest(&cmd, commands.iter().copied()) {
                msg.push_str(&t!("\ndid you mean `rythme {best}`?", "\nvouliez-vous dire `rythme {best}` ?"));
            }
            msg.push_str(&t!("\nsee `rythme help`", "\nvoir `rythme help`"));
            usage_error(msg)
        }
    }
}

/// Runs `f` on the files a command takes at the root of a project, when the
/// current folder is in one (`rythme run` runs its program).
fn in_project(cmd: Cmd, f: impl FnOnce(&[String]) -> i32) -> Option<i32> {
    match rythme::project::targets(cmd)? {
        Ok(files) => Some(f(&files)),
        Err(()) => Some(1),
    }
}

/// Is a file or a folder given to the command (not only options)? `valued`
/// are the options followed by a value.
fn has_target(args: &[String], valued: &[&str]) -> bool {
    let mut skip = false;
    for a in args {
        if std::mem::take(&mut skip) {
            continue;
        }
        if valued.contains(&a.as_str()) {
            skip = true;
        } else if a == "-" || !a.starts_with('-') {
            return true;
        }
    }
    false
}

fn usage_error(msg: String) -> i32 {
    rythme::io::err_line(&msg);
    2
}

/// `rythme run --seed N file.ry`: the seed of `random` and `shuffle`, given
/// before the file (what follows the file is for the program), and the other words.
fn run_seed(args: &[String]) -> Result<(Option<i64>, &[String]), i32> {
    let (value, used) = match args.first().map(String::as_str) {
        Some("--seed" | "--graine") => (args.get(1).map(String::as_str), 2),
        Some(first) => match first.strip_prefix("--seed=").or_else(|| first.strip_prefix("--graine=")) {
            Some(v) => (Some(v), 1),
            None => return Ok((None, args)),
        },
        None => return Ok((None, args)),
    };
    match value.map(|v| (v, v.parse::<i64>())) {
        Some((_, Ok(n))) => Ok((Some(n), &args[used.min(args.len())..])),
        Some((v, Err(_))) => Err(usage_error(t!(
            "the seed must be a whole number, like `rythme run --seed 42 game.ry`, not `{v}`",
            "la graine doit être un nombre entier, comme `rythme run --seed 42 jeu.ry`, pas `{v}`"
        ))),
        None => Err(usage_error(t!(
            "`--seed` needs a whole number, like `rythme run --seed 42 game.ry`",
            "`--seed` attend un nombre entier, comme `rythme run --seed 42 jeu.ry`"
        ))),
    }
}

/// The options of `rythme run`, written before the file.
struct RunOptions {
    seed: Option<i64>,
    format: Format,
    /// `--sandbox`: no files, folders or environment variables (for tools).
    sandbox: bool,
}

/// `rythme run [--seed N] [--format json] [--sandbox] file.ry ARGS...`: the
/// options before the file, in any order, and the other words.
fn run_options(args: &[String]) -> Result<(RunOptions, &[String]), i32> {
    let mut opts = RunOptions { seed: None, format: Format::Text, sandbox: false };
    let mut rest = args;
    loop {
        match rest.first().map(String::as_str) {
            Some("--sandbox") => {
                opts.sandbox = true;
                rest = &rest[1..];
            }
            Some(first) if first == "--format" || first.starts_with("--format=") => {
                let used = if first == "--format" { 2 } else { 1 };
                let mut words = rest[..used.min(rest.len())].to_vec();
                opts.format = machine::take_format(&mut words)?;
                rest = &rest[used.min(rest.len())..];
            }
            _ => {
                let (seed, after) = run_seed(rest)?;
                if after.len() == rest.len() {
                    return Ok((opts, rest));
                }
                opts.seed = seed;
                rest = after;
            }
        }
    }
}

fn run_file(name: &str, script_args: Vec<String>, seed: Option<i64>, sandbox: bool) -> i32 {
    let mut driver = Driver::new();
    driver.interp.sandboxed = sandbox;
    if let Some(seed) = seed {
        driver.interp.seed_random(seed);
    }
    let path = match find_script(name) {
        Ok(p) => p,
        Err(d) => {
            driver.report(&d);
            return 1;
        }
    };
    if path.extension().and_then(|e| e.to_str()) != Some("ry") {
        driver.report(&Diagnostic::warning(
            "",
            t!("Rythme files usually end with `.ry` ({})", "les fichiers Rythme finissent d'habitude par `.ry` ({})", path.display()),
        ));
    }
    driver.interp.script_args = script_args;
    let Some(module) = driver.load_file(&path) else { return 1 };
    if driver.execute(&module, Mode::Run).is_err() {
        return 1;
    }
    0
}

fn check_file(name: &str) -> i32 {
    let mut driver = Driver::new();
    let path = match find_script(name) {
        Ok(p) => p,
        Err(d) => {
            driver.report(&d);
            return 1;
        }
    };
    let ok = driver.load_file(&path).is_some();
    let shown = path.display();
    if ok {
        if driver.warnings == 0 {
            rythme::io::out_line(&t!("✓ {shown}: no problems found", "✓ {shown} : aucun problème trouvé"));
        } else {
            let n = driver.warnings;
            rythme::io::out_line(&t!("✓ {shown}: no errors, {n} warning(s)", "✓ {shown} : aucune erreur, {n} avertissement(s)"));
        }
        0
    } else {
        let n = driver.errors;
        rythme::io::out_line(&t!("✗ {shown}: {n} error(s)", "✗ {shown} : {n} erreur(s)"));
        1
    }
}

fn eval_code(code: &str, script_args: Vec<String>) -> i32 {
    let mut driver = Driver::new();
    driver.interp.script_args = script_args;
    let cwd = std::env::current_dir().unwrap_or_default();
    let Some(module) = driver.load_source("<code>", code, None, &cwd) else { return 1 };
    match driver.execute(&module, Mode::Run) {
        Ok(Some(v)) => {
            if !matches!(v, Value::Unit) {
                rythme::io::out_line(&v.repr());
            }
            0
        }
        Ok(None) => 0,
        Err(_) => 1,
    }
}
