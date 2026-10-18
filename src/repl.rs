//! The interactive mode: type code, see the result immediately.

use std::borrow::Cow;
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

const COMMANDS: &[&str] = &[":help", ":quit", ":doc", ":load", ":reset", ":lang", ":learn", ":explain", ":names", ":steps"];

struct RyHelper {
    names: Vec<String>,
    color: bool,
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

impl Highlighter for RyHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if self.color {
            Cow::Owned(colorize(line))
        } else {
            Cow::Borrowed(line)
        }
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        self.color
    }
}

/// Colors for the code typed in the REPL: keywords, texts, numbers, constructors, comments.
fn colorize(line: &str) -> String {
    const RESET: &str = "\x1b[0m";
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let at = |i: usize| chars.get(i).map(|(b, _)| *b).unwrap_or(line.len());
    let mut out = String::with_capacity(line.len() * 2);
    let mut i = 0;
    while i < chars.len() {
        let (start, c) = chars[i];
        let mut j = i + 1;
        let color = if c == '#' {
            j = chars.len();
            "\x1b[90m"
        } else if c == '"' {
            while j < chars.len() {
                match chars[j].1 {
                    '\\' => j += 2,
                    '"' => {
                        j += 1;
                        break;
                    }
                    _ => j += 1,
                }
            }
            j = j.min(chars.len());
            "\x1b[32m"
        } else if c.is_ascii_digit() {
            while j < chars.len()
                && (chars[j].1.is_ascii_alphanumeric()
                    || chars[j].1 == '_'
                    || (chars[j].1 == '.' && chars.get(j + 1).is_some_and(|n| n.1.is_ascii_digit())))
            {
                j += 1;
            }
            "\x1b[33m"
        } else if c.is_alphabetic() || c == '_' {
            while j < chars.len() && (chars[j].1.is_alphanumeric() || chars[j].1 == '_') {
                j += 1;
            }
            let word = &line[start..at(j)];
            if KEYWORDS.contains(&word) {
                "\x1b[35m"
            } else if c.is_uppercase() {
                "\x1b[36m"
            } else {
                ""
            }
        } else {
            ""
        };
        let text = &line[start..at(j)];
        if color.is_empty() {
            out.push_str(text);
        } else {
            out.push_str(color);
            out.push_str(text);
            out.push_str(RESET);
        }
        i = j;
    }
    out
}

impl Validator for RyHelper {}

impl Helper for RyHelper {}

fn history_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(PathBuf::from(home).join(".rythme_history"))
}

fn names(driver: &Driver) -> Vec<String> {
    let mut out: Vec<String> = driver.interp.globals.keys().filter(|s| !s.is_stored()).map(|s| s.name().to_string()).collect();
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
    crate::io::out_line(&format!(
        "Rythme {} — {}",
        crate::VERSION,
        t!(":help for help, :quit to leave", ":help pour l'aide, :quit pour quitter")
    ));
    let mut rl: Editor<RyHelper, DefaultHistory> = match Editor::new() {
        Ok(rl) => rl,
        Err(e) => {
            crate::io::err_line(&e.to_string());
            return 1;
        }
    };
    rl.set_helper(Some(RyHelper { names: names(&driver), color: driver.color }));
    let history = history_path();
    if let Some(h) = &history {
        let _ = rl.load_history(h);
    }
    let mut buffer = String::new();
    // The last code run, so that a pasted `|> step` line can continue it.
    let mut last_input: Option<String> = None;
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
                            crate::io::out_line(&help::topic("repl").unwrap_or_default());
                            continue;
                        }
                        _ => {}
                    }
                    // A line starting with `|>` or `.name` continues the previous code
                    // (a multi-line pipeline pasted line by line).
                    let continues =
                        trimmed.starts_with("|>") || (trimmed.starts_with('.') && trimmed[1..].starts_with(char::is_alphabetic));
                    if let (true, Some(prev)) = (continues, &last_input) {
                        buffer.push_str(prev);
                    }
                }
                buffer.push_str(&line);
                buffer.push('\n');
                // Wait for the end of an unfinished block; blank lines are kept,
                // and two blank lines in a row run the code anyway.
                if incomplete(&buffer) && !buffer.ends_with("\n\n\n") {
                    continue;
                }
                let _ = rl.add_history_entry(buffer.trim_end());
                eval(&mut driver, &buffer);
                last_input = Some(std::mem::take(&mut buffer));
            }
            Err(ReadlineError::Interrupted) => {
                if buffer.is_empty() {
                    crate::io::out_line(&t!("(type :quit to leave)", "(tapez :quit pour quitter)"));
                } else {
                    buffer.clear();
                    crate::io::out_line(&t!("(cancelled)", "(annulé)"));
                }
            }
            Err(ReadlineError::Eof) => {
                // Unfinished code at the end of the input: show what is missing.
                if !buffer.trim().is_empty() {
                    eval(&mut driver, &buffer);
                }
                break;
            }
            Err(e) => {
                crate::io::err_line(&t!("cannot read what is typed: {e}", "impossible de lire ce qui est tapé : {e}"));
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
                crate::io::out_line(&format!("\x1b[36m{}\x1b[0m", v.repr()));
            } else {
                crate::io::out_line(&v.repr());
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
        ":help" | ":h" | ":?" | ":aide" => crate::io::out_line(&help::topic("repl").unwrap_or_default()),
        ":doc" => {
            if arg.is_empty() {
                crate::io::out_line(&t!("usage: :doc NAME (a function, a topic...)", "utilisation : :doc NOM (une fonction, un sujet...)"));
            } else if let Some(v) = driver.interp.globals.get(&Sym::new(arg)).cloned() {
                crate::io::out_line(help::help_for_value(&driver.interp, Some(&v)).trim_end());
            } else {
                crate::io::out_line(help::topic(arg).unwrap_or_else(|| help::unknown_topic(arg)).trim_end());
            }
        }
        ":load" | ":charger" => {
            if arg.is_empty() {
                crate::io::out_line(&t!("usage: :load file.ry", "utilisation : :load fichier.ry"));
            } else {
                let mut path = PathBuf::from(arg);
                // Like `rythme file`: `.ry` is added only if needed.
                if path.extension().is_none() && !path.is_file() {
                    path.set_extension("ry");
                }
                if let Some(m) = driver.reload_file(&path) {
                    if driver.execute(&m, Mode::Run).is_ok() {
                        crate::io::out_line(&t!("{} loaded", "{} chargé", path.display()));
                    }
                }
            }
        }
        ":reset" => {
            *driver = new_driver();
            crate::io::out_line(&t!("Everything was forgotten.", "Tout a été oublié."));
        }
        ":lang" | ":langue" => match lang::parse(arg) {
            Some(l) => {
                lang::set(l);
                crate::io::out_line(&t!("Messages are now in English.", "Les messages sont maintenant en français."));
            }
            None => crate::io::out_line(&t!("usage: :lang fr  or  :lang en", "utilisation : :lang fr  ou  :lang en")),
        },
        ":learn" | ":apprendre" => help::learn(),
        ":explain" | ":expliquer" => match help::explain(arg.split_whitespace().next().unwrap_or("")) {
            Some(text) => crate::io::out_line(&text),
            None => crate::io::out_line(&t!(
                "unknown code; codes look like E003 or W001",
                "code inconnu ; les codes ressemblent à E003 ou W001"
            )),
        },
        ":steps" | ":etapes" | ":étapes" => {
            if arg.is_empty() {
                crate::io::out_line(&t!(
                    "usage: :steps EXPRESSION, like :steps fib(4)",
                    "utilisation : :etapes EXPRESSION, comme :etapes fib(4)"
                ));
            } else {
                let cwd = std::env::current_dir().unwrap_or_default();
                if let Some(m) = driver.load_source("<repl>", arg, None, &cwd) {
                    let _ = crate::steps::run_module(driver, &m, crate::steps::Options::default(), None);
                }
            }
        }
        ":names" | ":noms" => {
            let builtin: std::collections::HashSet<&str> =
                BUILTINS.iter().map(|b| b.name).chain(crate::builtins::ALIASES.iter().map(|a| a.name)).collect();
            let mut own: Vec<String> = driver
                .interp
                .globals
                .keys()
                .filter(|s| !s.is_stored())
                .map(|s| s.name().to_string())
                .filter(|n| !builtin.contains(n.as_str()) && !matches!(n.as_str(), "pi" | "Some" | "None" | "Ok" | "Err"))
                .collect();
            own.sort();
            if own.is_empty() {
                crate::io::out_line(&t!("Nothing defined yet.", "Rien n'est encore défini."));
            } else {
                crate::io::out_line(&own.join("  "));
            }
        }
        other => {
            let mut msg = t!("unknown command `{other}`", "commande inconnue `{other}`");
            if let Some(best) = suggest::closest(other, COMMANDS.iter().copied()) {
                msg.push_str(&t!(" — did you mean `{best}`?", " — vouliez-vous dire `{best}` ?"));
            }
            crate::io::out_line(&format!("{msg}  ({})", t!(":help lists the commands", ":help liste les commandes")));
        }
    }
    Next::Continue
}

#[cfg(test)]
mod tests {
    use super::colorize;

    fn strip(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                for c in chars.by_ref() {
                    if c == 'm' {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    #[test]
    fn colorize_keeps_the_text_unchanged() {
        for line in [
            "",
            "fn f(x) = x * 2",
            "print(\"été {x}\") # comment",
            "\"unclosed",
            "\"ends with \\",
            "1.5e3 + 0xff",
            "Some(élément)",
            "日本 = 1",
        ] {
            assert_eq!(strip(&colorize(line)), line);
        }
        assert!(colorize("fn").contains("\x1b[35m"));
    }
}
