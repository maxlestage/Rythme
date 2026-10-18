//! `rythme.toml`, the file that describes a project, and `rythme.lock`, the
//! exact commits of its dependencies: the small part of TOML they need,
//! read by hand.
//!
//! ```toml
//! name = "hello"
//! version = "0.1.0"
//! entry = "src/main.ry"
//!
//! [dependencies]
//! mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
//! tools = { path = "../tools" }
//! ```

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::diagnostic::Diagnostic;
use crate::source::{self, Span};
use crate::suggest;

/// The name of the file that describes a project.
pub const MANIFEST: &str = "rythme.toml";
/// The name of the file that holds the exact commits of its dependencies.
pub const LOCK: &str = "rythme.lock";

// ----- the part of TOML that is needed -------------------------------------

/// A value: a text, or a table written on one line (`{ git = "...", tag = "..." }`).
pub enum Value {
    Text(String),
    Table(Vec<Entry>),
}

pub struct Entry {
    pub key: String,
    pub key_span: Span,
    pub value: Value,
    pub span: Span,
}

/// `[name]`, or `[[name]]` (one element of a list of tables), and its lines.
pub struct Section {
    pub name: String,
    pub array: bool,
    pub span: Span,
    pub entries: Vec<Entry>,
}

#[derive(Default)]
pub struct Doc {
    /// The lines before the first section.
    pub top: Vec<Entry>,
    pub sections: Vec<Section>,
}

struct Cursor<'a> {
    line: &'a str,
    i: usize,
    /// Where the line starts in the file.
    base: usize,
    file: u32,
}

fn toml_error(msg: String, span: Span) -> Diagnostic {
    Diagnostic::error("E025", msg).at(span)
}

impl Cursor<'_> {
    fn span(&self, from: usize, to: usize) -> Span {
        Span::new(self.file, self.base + from, self.base + to.max(from))
    }

    fn here(&self) -> Span {
        let end = self.line[self.i..].chars().next().map_or(self.i, |c| self.i + c.len_utf8());
        self.span(self.i, end)
    }

    fn peek(&self) -> Option<char> {
        self.line[self.i..].chars().next()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek().filter(|c| *c == ' ' || *c == '\t') {
            self.i += c.len_utf8();
        }
    }

    fn at_end(&self) -> bool {
        matches!(self.peek(), None | Some('#'))
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.i += c.len_utf8();
            true
        } else {
            false
        }
    }

    /// A key: a bare name (`mathlib`, `my-lib`) or a text.
    fn key(&mut self) -> Result<(String, Span), Box<Diagnostic>> {
        let start = self.i;
        if matches!(self.peek(), Some('"' | '\'')) {
            let s = self.text()?;
            return Ok((s, self.span(start, self.i)));
        }
        while let Some(c) = self.peek().filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-') {
            self.i += c.len_utf8();
        }
        if self.i == start {
            let d = toml_error(t!("expected a name here", "un nom était attendu ici"), self.here());
            let d = Box::new(d.help(t!(
                "each line is `name = \"value\"`, like `version = \"0.1.0\"`",
                "chaque ligne est `nom = \"valeur\"`, comme `version = \"0.1.0\"`"
            )));
            return Err(d);
        }
        Ok((self.line[start..self.i].to_string(), self.span(start, self.i)))
    }

    /// A text between `"` (with `\"`, `\\`, `\n`, `\t`) or between `'` (as written).
    fn text(&mut self) -> Result<String, Box<Diagnostic>> {
        let start = self.i;
        let quote = self.peek().unwrap_or('"');
        self.i += 1;
        let mut out = String::new();
        loop {
            let Some(c) = self.peek() else {
                let d = toml_error(t!("this text is not closed", "ce texte n'est pas fermé"), self.span(start, start + 1));
                return Err(Box::new(d.help(t!("add `{quote}` at the end of the text", "ajoutez `{quote}` à la fin du texte"))));
            };
            self.i += c.len_utf8();
            match c {
                c if c == quote => return Ok(out),
                '\\' if quote == '"' => {
                    let at = self.i - 1;
                    let e = self.peek();
                    self.i += e.map_or(0, char::len_utf8);
                    match e {
                        Some('"') => out.push('"'),
                        Some('\\') => out.push('\\'),
                        Some('n') => out.push('\n'),
                        Some('t') => out.push('\t'),
                        _ => {
                            let d = toml_error(t!("unknown escape in a text", "échappement inconnu dans un texte"), self.span(at, self.i));
                            return Err(Box::new(d.help(t!(
                                "write `\\\\` for a backslash, or use `/` in paths: they work on every system",
                                "écrivez `\\\\` pour une barre oblique inverse, ou utilisez `/` dans les chemins : ils marchent sur tous les systèmes"
                            ))));
                        }
                    }
                }
                c => out.push(c),
            }
        }
    }

    fn value(&mut self) -> Result<Value, Box<Diagnostic>> {
        match self.peek() {
            Some('"' | '\'') => Ok(Value::Text(self.text()?)),
            Some('{') => {
                self.i += 1;
                let mut entries: Vec<Entry> = Vec::new();
                self.skip_ws();
                if self.eat('}') {
                    return Ok(Value::Table(entries));
                }
                loop {
                    self.skip_ws();
                    let start = self.i;
                    let (key, key_span) = self.key()?;
                    self.skip_ws();
                    if !self.eat('=') {
                        return Err(toml_error(t!("expected `=` after `{key}`", "`=` était attendu après `{key}`"), self.here()).boxed());
                    }
                    self.skip_ws();
                    let value = match self.peek() {
                        Some('"' | '\'') => Value::Text(self.text()?),
                        _ => return Err(self.not_a_text()),
                    };
                    if entries.iter().any(|e| e.key == key) {
                        return Err(toml_error(t!("`{key}` is written twice", "`{key}` est écrit deux fois"), key_span).boxed());
                    }
                    entries.push(Entry { key, key_span, value, span: self.span(start, self.i) });
                    self.skip_ws();
                    if self.eat('}') {
                        return Ok(Value::Table(entries));
                    }
                    if !self.eat(',') {
                        let d = toml_error(t!("expected `,` or `}}` here", "`,` ou `}}` était attendu ici"), self.here());
                        return Err(Box::new(d.help(t!(
                            "a table is written on one line: `{{ git = \"...\", tag = \"...\" }}`",
                            "une table s'écrit sur une ligne : `{{ git = \"...\", tag = \"...\" }}`"
                        ))));
                    }
                }
            }
            _ => Err(self.not_a_text()),
        }
    }

    fn not_a_text(&self) -> Box<Diagnostic> {
        let end = self.line[self.i..].find([',', '}', '#']).map_or(self.line.len(), |n| self.i + n);
        let written = self.line[self.i..end].trim();
        let span = if written.is_empty() { self.here() } else { self.span(self.i, self.i + written.len()) };
        let d = toml_error(t!("a value is a text, written in quotes", "une valeur est un texte, écrit entre guillemets"), span);
        let example = if written.is_empty() { "0.1.0" } else { written };
        Box::new(d.help(t!("write `\"{example}\"`", "écrivez `\"{example}\"`")))
    }
}

/// Reads the text of a `rythme.toml` or a `rythme.lock` registered as `file`.
pub fn parse(text: &str, file: u32) -> Result<Doc, Box<Diagnostic>> {
    let mut doc = Doc::default();
    let mut offset = 0;
    for raw in text.split_inclusive('\n') {
        let base = offset;
        offset += raw.len();
        let line = raw.trim_end_matches(['\n', '\r']);
        let mut c = Cursor { line, i: 0, base, file };
        c.skip_ws();
        if c.at_end() {
            continue;
        }
        let start = c.i;
        if c.eat('[') {
            let array = c.eat('[');
            c.skip_ws();
            let (name, _) = c.key()?;
            c.skip_ws();
            if !c.eat(']') || (array && !c.eat(']')) {
                let close = if array { "]]" } else { "]" };
                return Err(toml_error(t!("expected `{close}` here", "`{close}` était attendu ici"), c.here()).boxed());
            }
            doc.sections.push(Section { name, array, span: c.span(start, c.i), entries: Vec::new() });
        } else {
            let (key, key_span) = c.key()?;
            c.skip_ws();
            if !c.eat('=') {
                let d = toml_error(t!("expected `=` after `{key}`", "`=` était attendu après `{key}`"), c.here());
                return Err(Box::new(d.help(t!("write `{key} = \"...\"`", "écrivez `{key} = \"...\"`"))));
            }
            c.skip_ws();
            let value = c.value()?;
            let entry = Entry { key, key_span, value, span: c.span(start, c.i) };
            let entries = match doc.sections.last_mut() {
                Some(s) => &mut s.entries,
                None => &mut doc.top,
            };
            if let Some(first) = entries.iter().find(|e| e.key == entry.key) {
                let key = &entry.key;
                let d = Diagnostic::error("E025", t!("`{key}` is written twice", "`{key}` est écrit deux fois"))
                    .at(entry.key_span)
                    .secondary(first.key_span, t!("first written here", "écrit une première fois ici"));
                return Err(d.boxed());
            }
            entries.push(entry);
        }
        c.skip_ws();
        if !c.at_end() {
            let d = toml_error(t!("unexpected text after the value", "texte inattendu après la valeur"), c.span(c.i, line.len()));
            return Err(Box::new(d.help(t!(
                "write one value per line; a comment starts with `#`",
                "écrivez une valeur par ligne ; un commentaire commence par `#`"
            ))));
        }
    }
    Ok(doc)
}

// ----- registered files ------------------------------------------------------

thread_local! {
    /// The id of each file read here, so that reading it again (the language
    /// server does it at every change) does not register it again.
    static IDS: RefCell<HashMap<PathBuf, u32>> = RefCell::new(HashMap::new());
}

/// Registers the text of `path` for messages, reusing its id.
fn register(path: &Path, text: &str) -> u32 {
    let shown = crate::project::shown(path);
    IDS.with(|ids| {
        let mut ids = ids.borrow_mut();
        match ids.get(path) {
            Some(&id) => {
                if source::get(id).is_none_or(|f| *f.text != *text || f.name != shown) {
                    source::replace(id, &shown, text);
                }
                id
            }
            None => {
                let id = source::add(&shown, text);
                ids.insert(path.to_path_buf(), id);
                id
            }
        }
    })
}

/// Reads and parses a file, with its id.
fn read(path: &Path) -> Result<(Doc, u32), Box<Diagnostic>> {
    let text = crate::driver::read_source(path)?;
    let file = register(path, &text);
    Ok((parse(&text, file)?, file))
}

// ----- rythme.toml -----------------------------------------------------------

/// Where the code of a dependency comes from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    Git {
        url: String,
        rev: Rev,
    },
    /// A folder, relative to the project.
    Path(String),
}

/// The version of a dependency from git: always a fixed one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rev {
    Tag(String),
    Commit(String),
}

impl std::fmt::Display for Rev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rev::Tag(t) => write!(f, "tag {t}"),
            Rev::Commit(c) => write!(f, "commit {c}"),
        }
    }
}

pub struct Dep {
    pub name: String,
    pub source: Source,
    pub span: Span,
}

pub struct Manifest {
    /// The folder of the project.
    pub dir: PathBuf,
    pub name: String,
    pub version: String,
    /// The program, relative to the folder of the project.
    pub entry: String,
    pub deps: Vec<Dep>,
}

impl Manifest {
    pub fn dep(&self, name: &str) -> Option<&Dep> {
        self.deps.iter().find(|d| d.name == name)
    }
}

/// Can this name be used in `use name/file`? A Rythme name in lower case.
pub fn is_package_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase() || c == '_')
        && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && crate::lexer::lex(name, u32::MAX).is_ok_and(|l| matches!(l.tokens.first().map(|t| &t.tok), Some(crate::lexer::Tok::Ident(_))))
}

/// A name that is not a package name: a message, with a better name when there is one.
pub fn bad_name(name: &str) -> (String, Option<String>) {
    let better: String = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_string();
    let better = if better.starts_with(|c: char| c.is_ascii_digit()) { format!("_{better}") } else { better };
    let msg = t!(
        "`{name}` cannot be the name of a package: it is used in `use {name}/file`, so it must be a Rythme name in lower case (letters, digits, `_`)",
        "`{name}` ne peut pas être le nom d'un paquet : il sert dans `use {name}/fichier`, donc ce doit être un nom Rythme en minuscules (lettres, chiffres, `_`)"
    );
    (msg, (is_package_name(&better) && better != name).then_some(better))
}

fn text_of<'a>(e: &'a Entry, what: &str) -> Result<&'a str, Box<Diagnostic>> {
    match &e.value {
        Value::Text(s) => Ok(s),
        Value::Table(_) => {
            let key = &e.key;
            Err(toml_error(t!("`{key}` is {what}, written in quotes", "`{key}` est {what}, écrit entre guillemets"), e.span).boxed())
        }
    }
}

fn is_version(v: &str) -> bool {
    let core = v.split(['-', '+']).next().unwrap_or("");
    let parts: Vec<&str> = core.split('.').collect();
    parts.len() == 3 && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit())) && !v.ends_with(['-', '+'])
}

fn unknown_key(key: &str, span: Span, known: &[&str]) -> Box<Diagnostic> {
    let mut d = Diagnostic::error("E025", t!("unknown key `{key}`", "clé inconnue `{key}`")).at(span);
    if let Some(best) = suggest::closest(key, known.iter().copied()) {
        d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
    }
    let list: Vec<String> = known.iter().map(|k| format!("`{k}`")).collect();
    d.note(t!("the keys here are {}", "les clés ici sont {}", list.join(", "))).boxed()
}

/// Reads the `rythme.toml` of the folder `dir`.
pub fn load(dir: &Path) -> Result<Manifest, Box<Diagnostic>> {
    let path = dir.join(MANIFEST);
    let (doc, _) = read(&path)?;
    let shown = crate::project::shown(&path);
    let mut name = None;
    let mut version = None;
    let mut entry = None;
    for e in &doc.top {
        match e.key.as_str() {
            "name" => {
                let n = text_of(e, &t!("a text", "un texte"))?;
                if !is_package_name(n) {
                    let (msg, better) = bad_name(n);
                    let mut d = Diagnostic::error("E025", msg).at(e.span);
                    if let Some(b) = better {
                        d = d.help(t!("write `name = \"{b}\"`", "écrivez `name = \"{b}\"`"));
                    }
                    return Err(d.boxed());
                }
                name = Some(n.to_string());
            }
            "version" => {
                let v = text_of(e, &t!("a text", "un texte"))?;
                if !is_version(v) {
                    let d = toml_error(t!("`{v}` is not a version", "`{v}` n'est pas une version"), e.span);
                    return Err(Box::new(d.help(t!(
                        "a version is three numbers: `version = \"1.0.0\"` (major.minor.patch)",
                        "une version est faite de trois nombres : `version = \"1.0.0\"` (majeure.mineure.correctif)"
                    ))));
                }
                version = Some(v.to_string());
            }
            "entry" => {
                let p = text_of(e, &t!("a text", "un texte"))?;
                if !p.ends_with(".ry") || Path::new(p).is_absolute() {
                    let d = toml_error(
                        t!("`entry` is the path of a `.ry` file in the project", "`entry` est le chemin d'un fichier `.ry` du projet"),
                        e.span,
                    );
                    return Err(Box::new(d.help(t!("like `entry = \"src/main.ry\"`", "comme `entry = \"src/main.ry\"`"))));
                }
                entry = Some(p.to_string());
            }
            key => return Err(unknown_key(key, e.key_span, &["name", "version", "entry"])),
        }
    }
    for s in &doc.sections {
        if s.name != "dependencies" || s.array {
            let n = &s.name;
            let mut d = Diagnostic::error("E025", t!("unknown section `{n}`", "section inconnue `{n}`")).at(s.span);
            d = if n == "package" {
                d.help(t!(
                    "write `name`, `version` and `entry` at the top of the file, without a section",
                    "écrivez `name`, `version` et `entry` en haut du fichier, sans section"
                ))
            } else {
                d.help(t!("the only section is `[dependencies]`", "la seule section est `[dependencies]`"))
            };
            return Err(d.boxed());
        }
    }
    let Some(name) = name else {
        let d = Diagnostic::error("E025", t!("`{shown}` has no `name`", "`{shown}` n'a pas de `name`"));
        return Err(d
            .help(t!("add a line like `name = \"hello\"` at the top", "ajoutez une ligne comme `name = \"bonjour\"` en haut"))
            .boxed());
    };
    let Some(version) = version else {
        let d = Diagnostic::error("E025", t!("`{shown}` has no `version`", "`{shown}` n'a pas de `version`"));
        return Err(d
            .help(t!("add a line like `version = \"0.1.0\"` at the top", "ajoutez une ligne comme `version = \"0.1.0\"` en haut"))
            .boxed());
    };
    let mut deps: Vec<Dep> = Vec::new();
    for s in &doc.sections {
        for e in &s.entries {
            deps.push(dependency(e, &name)?);
        }
    }
    Ok(Manifest { dir: dir.to_path_buf(), name, version, entry: entry.unwrap_or_else(|| "src/main.ry".to_string()), deps })
}

fn dependency_help(name: &str) -> String {
    t!(
        "write `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (or `commit = \"...\"`), or `{name} = {{ path = \"../folder\" }}`",
        "écrivez `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (ou `commit = \"...\"`), ou `{name} = {{ path = \"../dossier\" }}`"
    )
}

fn dependency(e: &Entry, own: &str) -> Result<Dep, Box<Diagnostic>> {
    let name = &e.key;
    if !is_package_name(name) {
        let (msg, better) = bad_name(name);
        let mut d = Diagnostic::error("E025", msg).at(e.key_span);
        if let Some(b) = better {
            d = d.help(t!("call it `{b}`", "appelez-la `{b}`"));
        }
        return Err(d.boxed());
    }
    if name == own {
        let d = Diagnostic::error(
            "E025",
            t!("the project `{own}` cannot depend on itself", "le projet `{own}` ne peut pas dépendre de lui-même"),
        );
        return Err(d
            .at(e.key_span)
            .help(t!("`use {own}/file` already reaches its own files", "`use {own}/fichier` atteint déjà ses propres fichiers"))
            .boxed());
    }
    let Value::Table(fields) = &e.value else {
        let d = toml_error(t!("the dependency `{name}` is written as a table", "la dépendance `{name}` s'écrit comme une table"), e.span);
        return Err(Box::new(d.help(dependency_help(name))));
    };
    let mut get = HashMap::new();
    for f in fields {
        if !["git", "tag", "commit", "path"].contains(&f.key.as_str()) {
            return Err(unknown_key(&f.key, f.key_span, &["git", "tag", "commit", "path"]));
        }
        get.insert(f.key.as_str(), text_of(f, &t!("a text", "un texte"))?.to_string());
    }
    let wrong = |msg: String| -> Box<Diagnostic> { Box::new(toml_error(msg, e.span).help(dependency_help(name))) };
    let source = match (get.remove("git"), get.remove("path")) {
        (Some(_), Some(_)) => {
            return Err(wrong(t!(
                "the dependency `{name}` has both `git` and `path`",
                "la dépendance `{name}` a à la fois `git` et `path`"
            )))
        }
        (None, None) => {
            return Err(wrong(t!("the dependency `{name}` needs `git` or `path`", "la dépendance `{name}` a besoin de `git` ou de `path`")))
        }
        (None, Some(path)) => {
            if !get.is_empty() {
                return Err(wrong(t!(
                    "`tag` and `commit` are for a dependency from git, not from a folder",
                    "`tag` et `commit` sont pour une dépendance venant de git, pas d'un dossier"
                )));
            }
            Source::Path(path)
        }
        (Some(url), None) => {
            let rev = match (get.remove("tag"), get.remove("commit")) {
                (Some(t), None) if !t.is_empty() => Rev::Tag(t),
                (None, Some(c)) if c.len() >= 4 && c.chars().all(|c| c.is_ascii_hexdigit()) => Rev::Commit(c.to_lowercase()),
                (None, Some(c)) => return Err(wrong(t!("`{c}` is not a commit", "`{c}` n'est pas un commit"))),
                (Some(_), Some(_)) => {
                    return Err(wrong(t!("give a `tag` or a `commit`, not both", "donnez un `tag` ou un `commit`, pas les deux")))
                }
                _ => {
                    return Err(wrong(t!(
                        "the dependency `{name}` needs a fixed version: a `tag` or a `commit`",
                        "la dépendance `{name}` a besoin d'une version fixe : un `tag` ou un `commit`"
                    )))
                }
            };
            if url.trim().is_empty() {
                return Err(wrong(t!("the `git` address of `{name}` is empty", "l'adresse `git` de `{name}` est vide")));
            }
            Source::Git { url, rev }
        }
    };
    Ok(Dep { name: name.clone(), source, span: e.span })
}

// ----- rythme.lock -----------------------------------------------------------

/// The exact commit of a dependency from git.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Locked {
    pub name: String,
    pub url: String,
    pub rev: Rev,
    pub commit: String,
}

/// Reads `rythme.lock` in `dir`; nothing when there is none.
pub fn load_lock(dir: &Path) -> Result<Vec<Locked>, Box<Diagnostic>> {
    let path = dir.join(LOCK);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let (doc, _) = read(&path)?;
    let fix = || {
        t!(
            "this file is written by `rythme install`: delete it and run `rythme install` again",
            "ce fichier est écrit par `rythme install` : supprimez-le et relancez `rythme install`"
        )
    };
    if let Some(e) = doc.top.first() {
        return Err(Box::new(
            toml_error(t!("unexpected line in `rythme.lock`", "ligne inattendue dans `rythme.lock`"), e.span).help(fix()),
        ));
    }
    let mut out = Vec::new();
    for s in &doc.sections {
        let mut get: HashMap<&str, &str> = HashMap::new();
        for e in &s.entries {
            if let Value::Text(v) = &e.value {
                get.insert(e.key.as_str(), v);
            }
        }
        let rev = match (get.get("tag"), get.get("commit")) {
            (Some(t), Some(_)) => Some(Rev::Tag(t.to_string())),
            (None, Some(c)) => Some(Rev::Commit(get.get("asked").unwrap_or(c).to_string())),
            _ => None,
        };
        match (s.name.as_str(), s.array, get.get("name"), get.get("git"), rev, get.get("commit")) {
            ("package", true, Some(name), Some(url), Some(rev), Some(commit)) => {
                out.push(Locked { name: name.to_string(), url: url.to_string(), rev, commit: commit.to_string() })
            }
            _ => {
                let d = toml_error(
                    t!("this entry of `rythme.lock` is not complete", "cette entrée de `rythme.lock` n'est pas complète"),
                    s.span,
                );
                return Err(Box::new(d.help(fix())));
            }
        }
    }
    Ok(out)
}

fn quoted(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// The text of `rythme.lock`.
pub fn lock_text(entries: &[Locked]) -> String {
    let mut out = t!(
        "# The exact commit of each dependency from git, written by `rythme install`.\n# Keep this file with the project, so that everyone gets the same code.\n",
        "# Le commit exact de chaque dépendance venant de git, écrit par `rythme install`.\n# Gardez ce fichier avec le projet, pour que tout le monde ait le même code.\n"
    );
    for e in entries {
        out.push_str(&format!("\n[[package]]\nname = {}\ngit = {}\n", quoted(&e.name), quoted(&e.url)));
        match &e.rev {
            Rev::Tag(t) => out.push_str(&format!("tag = {}\n", quoted(t))),
            Rev::Commit(c) => out.push_str(&format!("asked = {}\n", quoted(c))),
        }
        out.push_str(&format!("commit = {}\n", quoted(&e.commit)));
    }
    out
}
