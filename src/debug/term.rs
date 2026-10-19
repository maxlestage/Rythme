//! `rythme debug file.ry`: the debugger in a terminal. The user types
//! commands (`break 12`, `run`, `next`, `print n * 2`...) at the prompt
//! `(debug)`; `help` lists them. When the input is not a terminal (a script
//! of commands), each command is written after the prompt, so that the
//! session reads as it was typed.

use std::io::{BufRead, IsTerminal};
use std::path::{Path, PathBuf};

use super::{line_of, Breakpoints, FrameView, Frontend, Outcome, Paused, Program, Reason, Resume, Session};
use crate::io::{out, out_line};
use crate::source;
use crate::value::Value;

const PROMPT: &str = "(debug) ";

/// The commands, for "did you mean".
const COMMANDS: &[&str] = &[
    "break",
    "delete",
    "breakpoints",
    "run",
    "continue",
    "next",
    "step",
    "finish",
    "where",
    "frame",
    "print",
    "locals",
    "list",
    "help",
    "quit",
];

/// Reads a command: `None` at the end of the input.
fn read_command() -> Option<String> {
    out(PROMPT);
    let mut line = String::new();
    match std::io::stdin().lock().read_line(&mut line) {
        Ok(0) | Err(_) => {
            out_line("");
            None
        }
        Ok(_) => {
            let line = line.trim().to_string();
            if !std::io::stdin().is_terminal() {
                out_line(&line);
            }
            Some(line)
        }
    }
}

/// Splits `print n * 2` into `print` and `n * 2`, and gives the full name of a short command.
fn parse(line: &str) -> (String, &str) {
    let (word, rest) = match line.split_once(char::is_whitespace) {
        Some((w, r)) => (w, r.trim()),
        None => (line, ""),
    };
    let word = match word {
        "b" | "br" => "break",
        "d" | "del" => "delete",
        "info" | "bps" => "breakpoints",
        "r" => "run",
        "c" | "cont" => "continue",
        "n" => "next",
        "s" => "step",
        "f" | "fin" | "out" => "finish",
        "w" | "bt" | "backtrace" => "where",
        "p" => "print",
        "l" | "local" => "locals",
        "h" | "?" => "help",
        "q" | "exit" => "quit",
        other => other,
    };
    (word.to_string(), rest)
}

fn help() {
    out_line(&t!(
        r#"Commands (short forms in parentheses):
  break 12         pause at line 12 of the program (b)
  break file.ry:3  pause at line 3 of another file of the program
  break fact       pause each time the function fact starts
  delete 2         remove breakpoint 2; delete alone removes them all (d)
  breakpoints      list the breakpoints
  run              start the program (r)
  continue         go on until the next breakpoint (c)
  next             go to the next line, without entering the functions called (n)
  step             go to the next line, entering the functions called (s)
  finish           go on until this function gives its result (f)
  where            the calls in progress, with their arguments (w)
  frame 2          look at the call number 2 of where (print and locals use it)
  print n * 2      the value of an expression, computed in the current call (p)
  locals           the names of the current call and their values (l)
  list             the lines around the current one
  help             this list (h)
  quit             stop the program and the debugger (q)
An empty line repeats next, step, finish or continue."#,
        r#"Commandes (formes courtes entre parenthèses) :
  break 12         faire une pause à la ligne 12 du programme (b)
  break fichier.ry:3  faire une pause à la ligne 3 d'un autre fichier du programme
  break fact       faire une pause à chaque début de la fonction fact
  delete 2         enlever le point d'arrêt 2 ; delete seul les enlève tous (d)
  breakpoints      lister les points d'arrêt
  run              lancer le programme (r)
  continue         continuer jusqu'au prochain point d'arrêt (c)
  next             aller à la ligne suivante, sans entrer dans les fonctions appelées (n)
  step             aller à la ligne suivante, en entrant dans les fonctions appelées (s)
  finish           continuer jusqu'à ce que cette fonction donne son résultat (f)
  where            les appels en cours, avec leurs arguments (w)
  frame 2          regarder l'appel numéro 2 de where (print et locals l'utilisent)
  print n * 2      la valeur d'une expression, calculée dans l'appel courant (p)
  locals           les noms de l'appel courant et leurs valeurs (l)
  list             les lignes autour de la ligne courante
  help             cette liste (h)
  quit             arrêter le programme et le débogueur (q)
Une ligne vide répète next, step, finish ou continue."#
    ));
}

fn unknown(word: &str) {
    let mut msg = t!("unknown command `{word}`", "commande inconnue `{word}`");
    if let Some(best) = crate::suggest::closest(word, COMMANDS.iter().copied()) {
        msg.push_str(&t!(": did you mean `{best}`?", " : vouliez-vous dire `{best}` ?"));
    }
    msg.push_str(&t!(" (`help` lists the commands)", " (`help` liste les commandes)"));
    out_line(&msg);
}

/// A value, cut if it is very long.
fn shown(v: &Value) -> String {
    let s = v.repr();
    if s.chars().count() > 600 {
        format!("{}...", s.chars().take(597).collect::<String>())
    } else {
        s
    }
}

/// `fact.ry:3` for a span.
fn place(span: crate::source::Span) -> String {
    let (file, line) = line_of(span);
    match source::get(file) {
        Some(f) => {
            let (name, line) = f.shown(line);
            format!("{name}:{line}")
        }
        None => "?".to_string(),
    }
}

/// `fact(n = 2)`, `map([1, 2], <fn>)` or "top level".
fn call_text(f: &FrameView) -> String {
    if f.name.is_empty() && !f.builtin {
        return t!("top level", "niveau principal");
    }
    let args: Vec<String> = f.args.iter().map(|(n, v)| if n.is_empty() { shown(v) } else { format!("{n} = {}", shown(v)) }).collect();
    format!("{}({})", f.name, args.join(", "))
}

/// Shows the line `line` of a file, with its number, and those around it when `around`.
fn show_lines(file: u32, line: usize, around: usize) {
    let Some(f) = source::get(file) else { return };
    let first = line.saturating_sub(around).max(1);
    let last = (line + around).min(f.line_count());
    let width = last.to_string().len();
    for l in first..=last {
        let mark = if l == line && around > 0 { "> " } else { "  " };
        let (_, shown_line) = f.shown(l);
        out_line(&format!("{mark}{shown_line:>width$} | {}", f.line_text(l)));
    }
}

/// Adds a breakpoint from what follows `break`.
fn add_break(bps: &mut Breakpoints, arg: &str) {
    if arg.is_empty() {
        out_line(&t!(
            "break where? A line (`break 12`), a file and a line (`break geometry.ry:3`) or a function (`break fact`)",
            "break où ? Une ligne (`break 12`), un fichier et une ligne (`break geometrie.ry:3`) ou une fonction (`break fact`)"
        ));
        return;
    }
    let (file, line) = match arg.rsplit_once(':') {
        Some((f, l)) if l.trim().parse::<usize>().is_ok() => (Some(f.trim()), l.trim().parse::<usize>().ok()),
        _ => (None, arg.parse::<usize>().ok()),
    };
    match line {
        Some(line) => {
            let id = match file {
                None => bps.files.first().map(|(id, _)| *id),
                Some(name) => bps.find_file(name),
            };
            let Some(id) = id else {
                let name = file.unwrap_or_default();
                let files: Vec<String> = bps.files.iter().map(|(_, n)| format!("`{n}`")).collect();
                out_line(&t!(
                    "no file `{name}` in the program (its files: {})",
                    "pas de fichier `{name}` dans le programme (ses fichiers : {})",
                    files.join(", ")
                ));
                return;
            };
            match bps.add_line(id, line) {
                Ok((n, at)) => {
                    let shown_at = place_of_line(id, at);
                    if at == line {
                        out_line(&t!("breakpoint {n} at {shown_at}", "point d'arrêt {n} à {shown_at}"));
                    } else {
                        out_line(&t!(
                            "breakpoint {n} at {shown_at} (line {line} has no code where the program can pause)",
                            "point d'arrêt {n} à {shown_at} (la ligne {line} n'a pas de code où le programme peut faire une pause)"
                        ));
                    }
                }
                Err(e) => out_line(&e),
            }
        }
        None if file.is_none() => match bps.add_function(arg) {
            Ok(n) => out_line(&t!("breakpoint {n} at the start of `{arg}`", "point d'arrêt {n} au début de `{arg}`")),
            Err(e) => out_line(&e),
        },
        None => out_line(&t!("`{arg}` is not a line number", "`{arg}` n'est pas un numéro de ligne")),
    }
}

fn place_of_line(file: u32, line: usize) -> String {
    match source::get(file) {
        Some(f) => {
            let (name, line) = f.shown(line);
            format!("{name}:{line}")
        }
        None => format!("?:{line}"),
    }
}

fn list_breaks(bps: &Breakpoints) {
    if bps.lines.is_empty() && bps.functions.is_empty() {
        out_line(&t!(
            "no breakpoints: add one with `break 12` or `break fact`",
            "aucun point d'arrêt : ajoutez-en un avec `break 12` ou `break fact`"
        ));
        return;
    }
    let mut all: Vec<(u32, String)> = bps.lines.iter().map(|b| (b.id, place_of_line(b.file, b.line))).collect();
    all.extend(bps.functions.iter().map(|(id, name)| (*id, t!("start of `{name}`", "début de `{name}`"))));
    all.sort();
    for (id, what) in all {
        out_line(&format!("  {id}  {what}"));
    }
}

fn delete(bps: &mut Breakpoints, arg: &str) {
    if arg.is_empty() {
        bps.lines.clear();
        bps.functions.clear();
        out_line(&t!("all breakpoints removed", "tous les points d'arrêt sont enlevés"));
        return;
    }
    match arg.trim_start_matches('#').parse::<u32>() {
        Ok(n) if bps.remove(n) => out_line(&t!("breakpoint {n} removed", "point d'arrêt {n} enlevé")),
        Ok(n) => out_line(&t!("there is no breakpoint {n} (see `breakpoints`)", "il n'y a pas de point d'arrêt {n} (voir `breakpoints`)")),
        Err(_) => out_line(&t!(
            "delete which breakpoint? Give its number, like `delete 1`",
            "enlever quel point d'arrêt ? Donnez son numéro, comme `delete 1`"
        )),
    }
}

/// The terminal while the program runs: shows each pause and reads commands.
struct Terminal {
    /// The command an empty line repeats.
    repeat: Option<Resume>,
}

impl Terminal {
    fn show_pause(&self, p: &Paused) {
        let frames = p.frames();
        match &p.reason {
            Reason::Entry | Reason::Step | Reason::Pause => {}
            Reason::Breakpoint(ids) => {
                let ids: Vec<String> = ids.iter().map(u32::to_string).collect();
                out_line(&t!("● breakpoint {}", "● point d'arrêt {}", ids.join(", ")));
            }
            Reason::Function(n) => {
                let name = frames.first().map(|f| f.name.clone()).unwrap_or_default();
                out_line(&t!("● breakpoint {n}: `{name}` starts", "● point d'arrêt {n} : `{name}` commence"));
            }
            Reason::Returned { name, value } => {
                out_line(&t!("`{name}` gave {}", "`{name}` a donné {}", shown(value)));
            }
            Reason::Error(d) => {
                let msg = &d.message;
                out_line(&t!("✗ the program stops on an error: {msg}", "✗ le programme s'arrête sur une erreur : {msg}"));
                out_line(&t!(
                    "  look at the values with where, locals and print; continue shows the whole message",
                    "  regardez les valeurs avec where, locals et print ; continue montre le message complet"
                ));
            }
        }
        let Some(here) = frames.first() else { return };
        let mut line = format!("→ {}  {}", place(p.at), call_text(here));
        let depth = p.depth();
        if depth > 0 {
            line.push_str(&t!(" · depth {depth}", " · profondeur {depth}"));
            let rec = p.recursion();
            if rec > 1 {
                let name = &here.name;
                line.push_str(&t!(" ({rec} calls of `{name}` in progress)", " ({rec} appels de `{name}` en cours)"));
            }
        }
        out_line(&line);
        if let Some(by) = &here.replaced {
            out_line(&t!(
                "  (a call in tail position: it took the place of the frame of `{by}`)",
                "  (un appel terminal : il a pris la place du cadre de `{by}`)"
            ));
        }
        let (file, l) = line_of(p.at);
        show_lines(file, l, 0);
    }
}

impl Frontend for Terminal {
    fn paused(&mut self, p: &mut Paused) -> Resume {
        self.show_pause(p);
        let mut frame = 0;
        loop {
            let Some(line) = read_command() else { quit() };
            if line.is_empty() {
                if let Some(r) = self.repeat {
                    return r;
                }
                continue;
            }
            let (cmd, arg) = parse(&line);
            let resume = match cmd.as_str() {
                "continue" => Some(Resume::Continue),
                "next" => Some(Resume::Next),
                "step" => Some(Resume::Step),
                "finish" => Some(Resume::Finish),
                _ => None,
            };
            if let Some(r) = resume {
                self.repeat = Some(r);
                return r;
            }
            match cmd.as_str() {
                "run" => out_line(&t!(
                    "the program is already running: continue goes on, quit stops it",
                    "le programme est déjà lancé : continue le poursuit, quit l'arrête"
                )),
                "where" => {
                    let frames = p.frames();
                    for (i, f) in frames.iter().enumerate() {
                        let mark = if i == frame { "→" } else { " " };
                        let at = f.at.map(place).unwrap_or_default();
                        let mut text = format!("{mark} #{i} {}  {at}", call_text(f));
                        if let Some(by) = &f.replaced {
                            text.push_str(&t!(" (replaced `{by}`: tail call)", " (a remplacé `{by}` : appel terminal)"));
                        }
                        out_line(&text);
                    }
                    let rec = p.recursion();
                    if rec > 1 {
                        let name = frames.first().map(|f| f.name.clone()).unwrap_or_default();
                        out_line(&t!(
                            "`{name}` calls itself: {rec} calls in progress",
                            "`{name}` s'appelle lui-même : {rec} appels en cours"
                        ));
                    }
                }
                "frame" => {
                    let n = p.frames().len();
                    match arg.trim_start_matches('#').parse::<usize>() {
                        Ok(i) if i < n => {
                            frame = i;
                            let f = &p.frames()[i];
                            let at = f.at.map(place).unwrap_or_default();
                            out_line(&format!("#{i} {}  {at}", call_text(f)));
                        }
                        _ => out_line(&t!(
                            "which call? A number from 0 to {}, as `where` shows them",
                            "quel appel ? Un numéro de 0 à {}, comme `where` les montre",
                            n.saturating_sub(1)
                        )),
                    }
                }
                "locals" => {
                    let locals = p.locals(frame);
                    let is_top = p.frames().get(frame).is_some_and(|f| f.name.is_empty() && !f.builtin);
                    let globals = if is_top { p.globals() } else { Vec::new() };
                    if locals.is_empty() && globals.is_empty() {
                        out_line(&t!("no names here yet", "aucun nom ici pour l'instant"));
                    }
                    for (name, v) in locals.iter().chain(&globals) {
                        out_line(&format!("  {name} = {}", shown(v)));
                    }
                }
                "print" if arg.is_empty() => out_line(&t!(
                    "print what? An expression, like `print n` or `print len(xs) * 2`",
                    "print quoi ? Une expression, comme `print n` ou `print len(xs) * 2`"
                )),
                "print" => match p.eval(frame, arg) {
                    Ok(v) => out_line(&shown(&v)),
                    Err(e) => out_line(&e),
                },
                "list" => {
                    let at = p.frames().get(frame).and_then(|f| f.at).unwrap_or(p.at);
                    let (file, l) = line_of(at);
                    show_lines(file, l, 4);
                }
                "break" => add_break(p.breakpoints(), arg),
                "delete" => delete(p.breakpoints(), arg),
                "breakpoints" => list_breaks(p.breakpoints()),
                "help" => help(),
                "quit" => quit(),
                other => unknown(other),
            }
        }
    }
}

/// `quit`: stops the program and the debugger at once.
fn quit() -> ! {
    out_line(&t!("debugger stopped", "débogueur arrêté"));
    std::process::exit(0)
}

/// What to do when the program is not running.
enum Start {
    Run,
    Step,
}

/// Reads commands while the program is not running (before it starts or after it ended).
fn idle(bps: &mut Breakpoints) -> Start {
    loop {
        let Some(line) = read_command() else { quit() };
        if line.is_empty() {
            continue;
        }
        let (cmd, arg) = parse(&line);
        match cmd.as_str() {
            "run" | "continue" => return Start::Run,
            "step" | "next" => return Start::Step,
            "break" => add_break(bps, arg),
            "delete" => delete(bps, arg),
            "breakpoints" => list_breaks(bps),
            "list" => {
                let file = bps.files.first().map(|(id, _)| *id).unwrap_or_default();
                let line = arg.parse::<usize>().unwrap_or(1);
                show_lines(file, line, 4);
            }
            "help" => help(),
            "quit" => quit(),
            "where" | "print" | "locals" | "finish" | "frame" => out_line(&t!(
                "the program is not running: `run` starts it (`step` pauses at its first line)",
                "le programme n'est pas lancé : `run` le lance (`step` fait une pause à sa première ligne)"
            )),
            other => unknown(other),
        }
    }
}

/// `rythme debug file.ry [arguments of the program]`.
pub fn command(args: &[String]) -> i32 {
    let Some(name) = args.first() else {
        crate::io::err_line(&t!(
            "which file? Example: rythme debug factorial.ry (see `rythme help debug`)",
            "quel fichier ? Exemple : rythme debug factorielle.ry (voir `rythme help debug`)"
        ));
        return 2;
    };
    let mut path = PathBuf::from(name);
    if !path.is_file() && path.extension().is_none() {
        path.set_extension("ry");
    }
    let color = crate::diagnostic::use_color();
    let mut bps = Breakpoints::default();
    let mut first = true;
    let mut start = None;
    loop {
        // Loaded afresh at each run: the program may have changed.
        let Some(mut program) = Program::load(&path, args[1..].to_vec(), color) else { return 1 };
        bps.learn(&program.modules());
        if first {
            let shown = Path::new(&*program.module.file).display().to_string();
            out_line(&t!(
                "Debugging {shown}. Pause somewhere with `break 12` or `break function`, then `run`; `help` lists the commands.",
                "Débogage de {shown}. Faites une pause quelque part avec `break 12` ou `break fonction`, puis `run` ; `help` liste les commandes."
            ));
            first = false;
        }
        let start_now = match start.take() {
            Some(s) => s,
            None => idle(&mut bps),
        };
        let entry = matches!(start_now, Start::Step);
        let session = Session::new(Box::new(Terminal { repeat: None }), bps, Resume::Continue, entry);
        let (outcome, session) = program.run(session);
        bps = session.breakpoints;
        match outcome {
            Outcome::Finished => out_line(&t!(
                "The program finished. `run` starts it again, `quit` leaves.",
                "Le programme est terminé. `run` le relance, `quit` quitte."
            )),
            Outcome::Failed => out_line(&t!(
                "The program stopped on an error. `run` starts it again, `quit` leaves.",
                "Le programme s'est arrêté sur une erreur. `run` le relance, `quit` quitte."
            )),
        }
        start = Some(idle(&mut bps));
    }
}
