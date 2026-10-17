//! The interactive mode: type code, see the result immediately.

use std::path::PathBuf;

use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

use crate::builtins::BUILTINS;
use crate::driver::Driver;
use crate::interp::Mode;
use crate::lexer::KEYWORDS;
use crate::symbol::Sym;
use crate::value::Value;
use crate::{help, lang, lexer, parser, source, suggest};

const COMMANDS: &[&str] = &[":help", ":quit", ":doc", ":load", ":reset", ":lang", ":learn", ":explain", ":names"];

struct RyHelper {
    names: Vec<String>,
}

impl Completer for RyHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line[..pos]
            .char_indices()
            .rev()
            .find(|(_, c)| !(c.is_alphanumeric() || *c == '_' || *c == ':'))
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or(0);
        let word = &line[start..pos];
        if word.is_empty() {
            return Ok((pos, Vec::new()));
        }
        let mut found: Vec<Pair> =
            self.names.iter().filter(|n| n.starts_with(word)).map(|n| Pair { display: n.clone(), replacement: n.clone() }).collect();
        found.sort_by(|a, b| a.display.cmp(&b.display));
        found.dedup_by(|a, b| a.display == b.display);
        Ok((start, found))
    }
}

impl Hinter for RyHelper {
    type Hint = String;
}

impl Highlighter for RyHelper {}

impl Validator for RyHelper {}

impl Helper for RyHelper {}

fn history_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(PathBuf::from(home).join(".rythme_history"))
}

fn names(driver: &Driver) -> Vec<String> {
    let mut out: Vec<String> = driver.interp.globals.keys().map(|s| s.name().to_string()).collect();
    out.extend(KEYWORDS.iter().map(|k| k.to_string()));
    out.extend(COMMANDS.iter().map(|c| c.to_string()));
    out
}

/// Is this input unfinished (an open bracket, a block without `end`...)?
fn incomplete(code: &str) -> bool {
    let file = source::add("<repl>", code);
    match lexer::lex(code, file) {
        Err(d) => d.incomplete,
        Ok(lexed) => matches!(parser::parse(lexed), Err(d) if d.incomplete),
    }
}

fn new_driver() -> Driver {
    let mut d = Driver::new();
    d.repl = true;
    d
}

enum Next {
    Continue,
    Quit,
}

pub fn run() -> i32 {
    let mut driver = new_driver();
    println!("Rythme {} — {}", crate::VERSION, t!(":help for help, :quit to leave", ":help pour l'aide, :quit pour quitter"));
    let mut rl: Editor<RyHelper, DefaultHistory> = match Editor::new() {
        Ok(rl) => rl,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    rl.set_helper(Some(RyHelper { names: names(&driver) }));
    let history = history_path();
    if let Some(h) = &history {
        let _ = rl.load_history(h);
    }
    let mut buffer = String::new();
    loop {
        if let Some(h) = rl.helper_mut() {
            h.names = names(&driver);
        }
        let prompt = if buffer.is_empty() { "ry> " } else { "..> " };
        match rl.readline(prompt) {
            Ok(line) => {
                if buffer.is_empty() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    if trimmed.starts_with(':') {
                        let _ = rl.add_history_entry(trimmed);
                        match command(&mut driver, trimmed) {
                            Next::Quit => break,
                            Next::Continue => continue,
                        }
                    }
                    match trimmed {
                        "exit" | "quit" | "quitter" => break,
                        "help" | "aide" => {
                            println!("{}", help::topic("repl").unwrap_or_default());
                            continue;
                        }
                        _ => {}
                    }
                }
                buffer.push_str(&line);
                buffer.push('\n');
                if !line.trim().is_empty() && incomplete(&buffer) {
                    continue;
                }
                let _ = rl.add_history_entry(buffer.trim_end());
                eval(&mut driver, &buffer);
                buffer.clear();
            }
            Err(ReadlineError::Interrupted) => {
                if buffer.is_empty() {
                    println!("{}", t!("(type :quit to leave)", "(tapez :quit pour quitter)"));
                } else {
                    buffer.clear();
                    println!("{}", t!("(cancelled)", "(annulé)"));
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
    }
    if let Some(h) = &history {
        let _ = rl.save_history(h);
    }
    0
}

fn eval(driver: &mut Driver, code: &str) {
    let cwd = std::env::current_dir().unwrap_or_default();
    let Some(m) = driver.load_source("<repl>", code, None, &cwd) else { return };
    if let Ok(Some(v)) = driver.execute(&m, Mode::Repl) {
        if !matches!(v, Value::Unit) {
            if driver.color {
                println!("\x1b[36m{}\x1b[0m", v.repr());
            } else {
                println!("{}", v.repr());
            }
        }
    }
}

fn command(driver: &mut Driver, input: &str) -> Next {
    let mut parts = input.splitn(2, char::is_whitespace);
    let cmd = parts.next().unwrap_or("");
    let arg = parts.next().unwrap_or("").trim();
    match cmd {
        ":quit" | ":q" | ":exit" | ":quitter" => return Next::Quit,
        ":help" | ":h" | ":?" | ":aide" => println!("{}", help::topic("repl").unwrap_or_default()),
        ":doc" => {
            if arg.is_empty() {
                println!("{}", t!("usage: :doc NAME (a function, a topic...)", "utilisation : :doc NOM (une fonction, un sujet...)"));
            } else if let Some(v) = driver.interp.globals.get(&Sym::new(arg)).cloned() {
                println!("{}", help::help_for_value(&driver.interp, Some(&v)).trim_end());
            } else {
                println!("{}", help::topic(arg).unwrap_or_else(|| help::unknown_topic(arg)).trim_end());
            }
        }
        ":load" | ":charger" => {
            if arg.is_empty() {
                println!("{}", t!("usage: :load file.ry", "utilisation : :load fichier.ry"));
            } else {
                let mut path = PathBuf::from(arg);
                if path.extension().is_none() {
                    path.set_extension("ry");
                }
                if let Some(m) = driver.load_file(&path) {
                    if driver.execute(&m, Mode::Run).is_ok() {
                        println!("{}", t!("{} loaded", "{} chargé", path.display()));
                    }
                }
            }
        }
        ":reset" => {
            *driver = new_driver();
            println!("{}", t!("Everything was forgotten.", "Tout a été oublié."));
        }
        ":lang" | ":langue" => match lang::parse(arg) {
            Some(l) => {
                lang::set(l);
                println!("{}", t!("Messages are now in English.", "Les messages sont maintenant en français."));
            }
            None => println!("{}", t!("usage: :lang fr  or  :lang en", "utilisation : :lang fr  ou  :lang en")),
        },
        ":learn" | ":apprendre" => help::learn(),
        ":explain" | ":expliquer" => match help::explain(arg) {
            Some(text) => println!("{text}"),
            None => println!("{}", t!("unknown code; codes look like E003 or W001", "code inconnu ; les codes ressemblent à E003 ou W001")),
        },
        ":names" | ":noms" => {
            let builtin: std::collections::HashSet<&str> = BUILTINS.iter().map(|b| b.name).collect();
            let mut own: Vec<String> = driver
                .interp
                .globals
                .keys()
                .map(|s| s.name().to_string())
                .filter(|n| !builtin.contains(n.as_str()) && !matches!(n.as_str(), "pi" | "Some" | "None" | "Ok" | "Err"))
                .collect();
            own.sort();
            if own.is_empty() {
                println!("{}", t!("Nothing defined yet.", "Rien n'est encore défini."));
            } else {
                println!("{}", own.join("  "));
            }
        }
        other => {
            let mut msg = t!("unknown command `{other}`", "commande inconnue `{other}`");
            if let Some(best) = suggest::closest(other, COMMANDS.iter().copied()) {
                msg.push_str(&t!(" — did you mean `{best}`?", " — vouliez-vous dire `{best}` ?"));
            }
            println!("{msg}  ({})", t!(":help lists the commands", ":help liste les commandes"));
        }
    }
    Next::Continue
}
