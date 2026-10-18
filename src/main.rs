//! The `rythme` command.

use std::io::Write;
use std::path::{Path, PathBuf};

use rythme::diagnostic::Diagnostic;
use rythme::driver::Driver;
use rythme::interp::Mode;
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
                    return usage_error(t!(
                        "{what}: write `--lang fr` or `--lang en`, like `rythme --lang fr hello.ry`",
                        "{what} : écrivez `--lang fr` ou `--lang en`, comme `rythme --lang fr bonjour.ry`"
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
    let rest: Vec<String> = args[1..].to_vec();
    match cmd.as_str() {
        "repl" => repl::run(),
        "help" | "aide" | "--help" | "-h" => {
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
        "run" | "lancer" => match rest.first() {
            Some(file) => run_file(file, rest[1..].to_vec()),
            None => usage_error(t!("which file? Example: rythme run hello.ry", "quel fichier ? Exemple : rythme run bonjour.ry")),
        },
        "test" | "tester" => rythme::testing::command(&rest),
        "check" | "verifier" | "vérifier" => match rest.first() {
            // Every file is checked, even after one with errors.
            Some(_) => rest.iter().map(|file| check_file(file)).fold(0, i32::max),
            None => usage_error(t!("which file? Example: rythme check hello.ry", "quel fichier ? Exemple : rythme check bonjour.ry")),
        },
        "fmt" | "format" | "formater" => rythme::format::command(&rest),
        "doc" | "documenter" => rythme::doc::command(&rest),
        "exercise" | "exercises" | "exercice" | "exercices" => rythme::exercise::command(&rest),
        "lsp" => rythme::lsp::run(&rest, chosen),
        "-e" | "eval" => match rest.first() {
            Some(code) => eval_code(code, rest[1..].to_vec()),
            None => usage_error(t!("which code? Example: rythme -e 'print(1 + 2)'", "quel code ? Exemple : rythme -e 'print(1 + 2)'")),
        },
        _ => {
            if cmd.ends_with(".ry") || Path::new(&cmd).exists() || Path::new(&format!("{cmd}.ry")).is_file() {
                return run_file(&cmd, rest);
            }
            let commands = ["run", "check", "test", "fmt", "doc", "help", "explain", "learn", "repl", "version", "lsp", "exercise"];
            let mut msg = t!("unknown command or file `{cmd}`", "commande ou fichier inconnu `{cmd}`");
            if let Some(best) = suggest::closest(&cmd, commands.iter().copied()) {
                msg.push_str(&t!("\ndid you mean `rythme {best}`?", "\nvouliez-vous dire `rythme {best}` ?"));
            }
            msg.push_str(&t!("\nsee `rythme help`", "\nvoir `rythme help`"));
            usage_error(msg)
        }
    }
}

fn usage_error(msg: String) -> i32 {
    rythme::io::err_line(&msg);
    2
}

/// Finds the program file, adding `.ry` if needed.
fn find_script(name: &str) -> Result<PathBuf, Box<Diagnostic>> {
    let path = PathBuf::from(name);
    if path.is_file() {
        return Ok(path);
    }
    if path.is_dir() {
        return Err(rythme::driver::folder_error(&path).boxed());
    }
    let with_ext = PathBuf::from(format!("{name}.ry"));
    if path.extension().is_none() && with_ext.is_file() {
        return Ok(with_ext);
    }
    let mut d = Diagnostic::error("E017", t!("file `{name}` not found", "fichier `{name}` introuvable"));
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let wanted = path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    let files: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| entries.flatten().map(|e| e.file_name().to_string_lossy().to_string()).filter(|n| n.ends_with(".ry")).collect())
        .unwrap_or_default();
    if let Some(best) = suggest::closest(&wanted, files.iter().map(|s| s.as_str())) {
        d = d.help(t!("did you mean `{}`?", "vouliez-vous dire `{}` ?", dir.join(best).display()));
    } else if let Ok(cwd) = std::env::current_dir() {
        d = d.note(t!("current folder: {}", "dossier actuel : {}", cwd.display()));
    }
    Err(d.boxed())
}

fn run_file(name: &str, script_args: Vec<String>) -> i32 {
    let mut driver = Driver::new();
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
