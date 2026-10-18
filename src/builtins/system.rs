//! Files, folders and the system: folders, file information, the lines of
//! the standard input, paths and the options of the command line. Every
//! function that touches the system returns a Result whose error is a text
//! in the language of the messages; in the browser (the playground), there is
//! no file system and no input, and the error says so.

use super::*;

/// In the browser, the error of `f`, which needs files (or the input when
/// `input`); `None` elsewhere.
pub(crate) fn in_browser(it: &mut Interp, f: &str, input: bool) -> Option<Value> {
    if !cfg!(target_arch = "wasm32") {
        return None;
    }
    let msg = if input {
        t!(
            "`{f}` is not available in the browser: a program here has no input to read. Run it on your computer: rythme file.ry < lines.txt",
            "`{f}` n'est pas disponible dans le navigateur : ici, un programme n'a pas d'entrée à lire. Lancez-le sur votre ordinateur : rythme fichier.ry < lignes.txt"
        )
    } else {
        t!(
            "`{f}` is not available in the browser: a program here has no files or folders. Run it on your computer: rythme file.ry",
            "`{f}` n'est pas disponible dans le navigateur : ici, un programme n'a ni fichiers ni dossiers. Lancez-le sur votre ordinateur : rythme fichier.ry"
        )
    };
    Some(it.err(text_value(msg)))
}

/// The text of the error when `path` cannot be used (`folder`: it names a folder).
pub(crate) fn io_message(e: &std::io::Error, path: &str, folder: bool) -> String {
    if path.is_empty() {
        return t!("the path is empty: write \".\" for the current folder", "le chemin est vide : écrivez \".\" pour le dossier courant");
    }
    match e.kind() {
        std::io::ErrorKind::NotFound if folder => t!("folder not found: {path}", "dossier introuvable : {path}"),
        std::io::ErrorKind::NotFound => t!("file not found: {path}", "fichier introuvable : {path}"),
        std::io::ErrorKind::PermissionDenied => t!("permission denied: {path}", "permission refusée : {path}"),
        _ => t!("cannot access {path}: {e}", "impossible d'accéder à {path} : {e}"),
    }
}

fn done(it: &mut Interp, r: Result<Value, String>) -> Value {
    match r {
        Ok(v) => it.ok(v),
        Err(e) => it.err(text_value(e)),
    }
}

pub(super) fn list_folder(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "list_folder", span)?;
    if let Some(e) = in_browser(it, "list_folder", false) {
        return Ok(e);
    }
    let names = (|| {
        let meta = std::fs::metadata(path).map_err(|e| io_message(&e, path, true))?;
        if !meta.is_dir() {
            return Err(t!(
                "{path} is a file, not a folder: `read_file` reads a file",
                "{path} est un fichier, pas un dossier : `read_file` lit un fichier"
            ));
        }
        let mut names = Vec::new();
        for entry in std::fs::read_dir(path).map_err(|e| io_message(&e, path, true))? {
            let entry = entry.map_err(|e| io_message(&e, path, true))?;
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
        names.sort();
        Ok(Value::list(names.into_iter().map(text_value).collect()))
    })();
    Ok(done(it, names))
}

pub(super) fn create_folder(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "create_folder", span)?;
    if let Some(e) = in_browser(it, "create_folder", false) {
        return Ok(e);
    }
    let made = match std::fs::metadata(path) {
        Ok(m) if m.is_dir() => Ok(Value::Unit),
        Ok(_) => {
            Err(t!("{path} already exists, and it is a file, not a folder", "{path} existe déjà, et c'est un fichier, pas un dossier"))
        }
        Err(_) => std::fs::create_dir_all(path).map(|()| Value::Unit).map_err(|e| io_message(&e, path, true)),
    };
    Ok(done(it, made))
}

pub(super) fn delete_file(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "delete_file", span)?;
    if let Some(e) = in_browser(it, "delete_file", false) {
        return Ok(e);
    }
    // A link is deleted itself, never what it points to.
    let deleted = match std::fs::symlink_metadata(path) {
        Ok(m) if m.is_dir() => Err(t!(
            "{path} is a folder: `delete_file` deletes only files, never folders",
            "{path} est un dossier : `delete_file` supprime seulement des fichiers, jamais des dossiers"
        )),
        Ok(_) => std::fs::remove_file(path).map(|()| Value::Unit).map_err(|e| io_message(&e, path, false)),
        Err(e) => Err(io_message(&e, path, false)),
    };
    Ok(done(it, deleted))
}

pub(super) fn file_info(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "file_info", span)?;
    if let Some(e) = in_browser(it, "file_info", false) {
        return Ok(e);
    }
    let info = std::fs::metadata(path).map_err(|e| io_message(&e, path, false)).map(|m| {
        let mut r = Record::new();
        r.set(Rc::from("is_folder"), Value::Bool(m.is_dir()));
        let size = if m.is_dir() { 0 } else { i64::try_from(m.len()).unwrap_or(i64::MAX) };
        r.set(Rc::from("size"), Value::Int(size));
        Value::Record(Rc::new(r))
    });
    Ok(done(it, info))
}

pub(super) fn input_lines(it: &mut Interp, _: &[Value], span: Span) -> Fallible<Value> {
    use std::io::BufRead;
    if let Some(e) = in_browser(it, "input_lines", true) {
        return Ok(e);
    }
    let mut lines = Vec::new();
    for line in std::io::stdin().lock().split(b'\n') {
        match line {
            Ok(l) => {
                if lines.len() == MAX_LIST {
                    return Err(too_big(true, span));
                }
                let l = String::from_utf8_lossy(&l).into_owned();
                lines.push(text_value(l.strip_suffix('\r').unwrap_or(&l)));
            }
            Err(e) => {
                let msg = t!("cannot read the standard input: {e}", "impossible de lire l'entrée standard : {e}");
                return Ok(it.err(text_value(msg)));
            }
        }
    }
    Ok(it.ok(Value::list(lines)))
}

// ----- paths ---------------------------------------------------------------

fn is_sep(c: char) -> bool {
    c == '/' || c == '\\'
}

/// Where a path starts, if it is absolute: `/`, `\`, `\\` (Windows network
/// paths), or a drive like `C:` with the separator after it if there is one.
fn root(path: &str) -> &str {
    let b = path.as_bytes();
    let drive = if b.len() >= 2 && b[0].is_ascii_alphabetic() && b[1] == b':' { 2 } else { 0 };
    let seps = path[drive..].bytes().take_while(|&c| c == b'/' || c == b'\\').count();
    &path[..drive + seps]
}

pub(super) fn split_path(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "split_path", span)?;
    let start = root(path);
    let mut parts: Vec<Value> = Vec::new();
    if !start.is_empty() {
        parts.push(text_value(start));
    }
    parts.extend(path[start.len()..].split(is_sep).filter(|p| !p.is_empty()).map(text_value));
    Ok(Value::list(parts))
}

pub(super) fn join_path(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "join_path", span)?;
    let mut parts = Vec::with_capacity(l.len());
    for v in l.iter() {
        match v {
            Value::Text(s) => parts.push(&**s),
            other => {
                return Err(Diagnostic::error(
                    "E023",
                    t!(
                        "`join_path` needs a list of texts, but it contains {}",
                        "`join_path` a besoin d'une liste de textes, mais elle contient {}",
                        other.describe()
                    ),
                )
                .at(span)
                .help(t!("`text(value)` turns a value into a text", "`text(valeur)` transforme une valeur en texte"))
                .boxed())
            }
        }
    }
    // Windows paths written with `\` only keep it; `/` works everywhere else, Windows included.
    let sep = if parts.iter().any(|p| p.contains('\\')) && !parts.iter().any(|p| p.contains('/')) { '\\' } else { '/' };
    let mut out = String::new();
    for part in parts.into_iter().filter(|p| !p.is_empty()) {
        let start = root(part);
        if !out.is_empty() {
            if !start.is_empty() {
                return Err(Diagnostic::error(
                    "E023",
                    t!(
                        "`join_path` cannot add \"{part}\" after \"{out}\": only the first part of a path can start at a root (`/`, `\\` or a drive like `C:`)",
                        "`join_path` ne peut pas ajouter \"{part}\" après \"{out}\" : seule la première partie d'un chemin peut partir d'une racine (`/`, `\\` ou un lecteur comme `C:`)"
                    ),
                )
                .at(span)
                .help(t!(
                    "remove the separator at the start of \"{part}\", or put it first in the list",
                    "enlevez le séparateur au début de \"{part}\", ou mettez-le en premier dans la liste"
                ))
                .boxed());
            }
            if !out.ends_with(is_sep) {
                out.push(sep);
            }
        }
        let trimmed = part.trim_end_matches(is_sep);
        out.push_str(if trimmed.len() < start.len() { start } else { trimmed });
        if out.len() > MAX_TEXT {
            return Err(too_big(false, span));
        }
    }
    Ok(text_value(out))
}

// ----- options of the command line -------------------------------------------

/// Is this word a negative number (`-5`, `-0.5`), not an option?
fn is_negative_number(w: &str) -> bool {
    w.strip_prefix('-').is_some_and(|n| n.starts_with(|c: char| c.is_ascii_digit() || c == '.') && n.parse::<f64>().is_ok())
}

/// The record of the options in `words`: `--name=value` gives the text
/// `value`, `--name` and `-n` give `true`, and the other words are in `args`.
fn parse_options(words: &[String]) -> Result<Record, String> {
    let mut rest = Vec::new();
    let mut found: Vec<(String, Value)> = Vec::new();
    let mut only_words = false;
    for w in words {
        if only_words || w == "-" || !w.starts_with('-') || is_negative_number(w) {
            rest.push(text_value(w.as_str()));
            continue;
        }
        if w == "--" {
            only_words = true;
            continue;
        }
        let long = w.starts_with("--");
        let body = if long { &w[2..] } else { &w[1..] };
        let (name, value) = match body.split_once('=') {
            Some((n, v)) => (n, text_value(v)),
            None => (body, Value::Bool(true)),
        };
        let valid = name.starts_with(char::is_alphabetic) && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_');
        if !valid {
            return Err(t!(
                "`{w}` is not a valid option: its name starts with a letter, followed by letters, digits, `-` or `_` (write `--` before words that start with `-`)",
                "`{w}` n'est pas une option valide : son nom commence par une lettre, suivie de lettres, de chiffres, de `-` ou de `_` (écrivez `--` avant les mots qui commencent par `-`)"
            ));
        }
        if !long && name.chars().count() != 1 {
            return Err(t!(
                "`{w}` is not an option that Rythme understands: write `--{name}` for a long name, or `-a -b` for several one-letter options",
                "`{w}` n'est pas une option que Rythme comprend : écrivez `--{name}` pour un nom long, ou `-a -b` pour plusieurs options d'une lettre"
            ));
        }
        let field = name.replace('-', "_");
        if field == "args" {
            return Err(t!(
                "`{w}` cannot be an option: the field `args` holds the words that are not options",
                "`{w}` ne peut pas être une option : le champ `args` contient les mots qui ne sont pas des options"
            ));
        }
        if found.iter().any(|(f, _)| *f == field) {
            return Err(t!("the option `{name}` is given twice", "l'option `{name}` est donnée deux fois"));
        }
        found.push((field, value));
    }
    let mut r = Record::new();
    r.set(Rc::from("args"), Value::list(rest));
    for (k, v) in found {
        r.set(Rc::from(k.as_str()), v);
    }
    Ok(r)
}

pub(super) fn options(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let words: Vec<String> = match a.first() {
        None => it.script_args.clone(),
        Some(_) => {
            let l = list(a, 0, "options", span)?;
            let mut words = Vec::with_capacity(l.len());
            for v in l.iter() {
                match v {
                    Value::Text(s) => words.push(s.to_string()),
                    other => {
                        return Err(Diagnostic::error(
                            "E023",
                            t!(
                                "`options` needs a list of texts, like the one of `args()`, but it contains {}",
                                "`options` a besoin d'une liste de textes, comme celle de `args()`, mais elle contient {}",
                                other.describe()
                            ),
                        )
                        .at(span)
                        .boxed())
                    }
                }
            }
            words
        }
    };
    let parsed = parse_options(&words).map(|r| Value::Record(Rc::new(r)));
    Ok(done(it, parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roots_of_paths() {
        for (path, start) in [
            ("/home/ada", "/"),
            ("home/ada", ""),
            ("C:\\Users\\ada", "C:\\"),
            ("c:/Users", "c:/"),
            ("D:notes.txt", "D:"),
            ("\\\\server\\share", "\\\\"),
            ("", ""),
            ("é/x", ""),
        ] {
            assert_eq!(root(path), start, "{path}");
        }
    }

    #[test]
    fn negative_numbers_are_words() {
        assert!(is_negative_number("-5") && is_negative_number("-0.5") && is_negative_number("-.5"));
        assert!(!is_negative_number("-v") && !is_negative_number("-") && !is_negative_number("-inf"));
    }
}
