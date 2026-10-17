//! Understanding a document for the editor: its problems (from the lexer, the
//! parser and the checker, as `rythme check` reports them), the files it
//! `use`s, and the index of its names.
//!
//! Code being typed is often unfinished. The problems come from the real
//! text, but the index (for hover, completion, go to definition) comes from a
//! repaired copy where the lines the parser stumbles on are blanked out: the
//! rest of the file keeps working while one line is being written.

use std::collections::{HashMap, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::index::{self, Def, DefId, Index};
use super::text::{canonical, path_to_uri, short_name, LineIndex};
use crate::ast::Item;
use crate::check::{self, Knowledge};
use crate::diagnostic::Diagnostic;
use crate::source::{self, Span};
use crate::symbol::Sym;
use crate::{driver, lexer, parser};

/// One source id per file, reused at each edit (see `source::replace`).
#[derive(Default)]
pub struct FileIds {
    ids: HashMap<String, u32>,
}

impl FileIds {
    pub fn register(&mut self, key: &str, name: &str, text: &str) -> u32 {
        if let Some(&id) = self.ids.get(key) {
            let same = source::get(id).is_some_and(|f| &*f.text == text && f.name == name);
            if !same {
                source::replace(id, name, text);
            }
            return id;
        }
        let id = source::add(name, text);
        self.ids.insert(key.to_string(), id);
        id
    }
}

/// A file and what is known about it.
pub struct Module {
    pub path: Option<PathBuf>,
    pub uri: String,
    pub file: u32,
    /// The text was read without syntax error.
    pub parsed: bool,
    /// The file, and every file it uses, could be read and parsed.
    pub complete: bool,
    /// The syntax tree; repaired when the text has a syntax error.
    pub items: Vec<Item>,
    /// The folder where the files of `use` are looked for.
    pub base: PathBuf,
    /// Each `use` that names an existing file, and that file.
    pub used: Vec<(Span, PathBuf)>,
    /// The files it uses, found and loaded.
    pub deps: Vec<PathBuf>,
    /// Its own problems.
    pub diags: Vec<Diagnostic>,
    /// On each `use` line, a summary of the errors found in the used file.
    pub use_diags: Vec<Diagnostic>,
    /// The top-level names of the used files, as the checker sees them.
    pub dep_defs: HashMap<Sym, DefId>,
    pub index: Index,
}

/// Where texts come from: the editor's version of a file when it is open
/// (or a version being tried for a rename), else the file on the disk.
pub struct Sources<'a> {
    /// Canonical path → (address given by the editor, text).
    pub open: &'a HashMap<PathBuf, (String, Rc<str>)>,
    pub overlay: &'a HashMap<PathBuf, String>,
}

impl Sources<'_> {
    pub fn read(&self, path: &Path) -> Option<String> {
        if let Some(t) = self.overlay.get(path) {
            return Some(t.clone());
        }
        if let Some((_, t)) = self.open.get(path) {
            return Some(t.to_string());
        }
        std::fs::read_to_string(path).ok()
    }

    pub fn uri(&self, path: &Path) -> String {
        self.open.get(path).map(|(u, _)| u.clone()).unwrap_or_else(|| path_to_uri(path))
    }
}

pub struct Loader<'a> {
    ids: &'a mut FileIds,
    /// Prefix of the keys of `ids`: a rename is tried on separate copies.
    purpose: &'a str,
    sources: Sources<'a>,
    base: &'a Knowledge,
    /// The folder of `use` for documents that are not saved files.
    root: PathBuf,
    pub modules: HashMap<PathBuf, Rc<Module>>,
    loading: Vec<PathBuf>,
}

/// Runs `f`, turning a bug (a panic) into `None` so that the rest of the work goes on.
pub fn guard<T>(f: impl FnOnce() -> T) -> Option<T> {
    catch_unwind(AssertUnwindSafe(f)).ok()
}

impl<'a> Loader<'a> {
    pub fn new(ids: &'a mut FileIds, purpose: &'a str, sources: Sources<'a>, base: &'a Knowledge, root: PathBuf) -> Loader<'a> {
        Loader { ids, purpose, sources, base, root, modules: HashMap::new(), loading: Vec::new() }
    }

    /// Loads a document: its text, and the files it uses.
    pub fn load(&mut self, path: Option<&Path>, uri: &str, text: &str) -> Module {
        let key = match path {
            Some(p) => format!("{}{}", self.purpose, p.display()),
            None => format!("{}{uri}", self.purpose),
        };
        let file = self.ids.register(&key, &short_name(uri), text);
        let (items, parsed, mut diags) = match lexer::lex(text, file).and_then(parser::parse) {
            Ok(items) => (items, true, Vec::new()),
            Err(d) => (guard(|| recover(text, file)).flatten().unwrap_or_default(), false, vec![*d]),
        };
        let base = path.and_then(Path::parent).map(Path::to_path_buf).unwrap_or_else(|| self.root.clone());
        if let Some(p) = path {
            self.loading.push(p.to_path_buf());
        }
        let mut deps = Vec::new();
        let mut used = Vec::new();
        let mut use_diags = Vec::new();
        let mut complete = parsed;
        for item in &items {
            let Item::Use(p, span) = item else { continue };
            let dep = match driver::find_use(p, *span, &base) {
                // The same form of path as the open documents (see `text::canonical`).
                Ok(dep) => canonical(&dep),
                Err(d) => {
                    complete = false;
                    if parsed {
                        diags.push(*d);
                    }
                    continue;
                }
            };
            used.push((*span, dep.clone()));
            if self.loading.contains(&dep) {
                complete = false;
                if parsed {
                    diags.push(driver::use_cycle(p, *span));
                }
                continue;
            }
            let m = self.load_dep(&dep);
            complete &= m.complete;
            let errors = self.errors_in(&dep);
            if parsed && !errors.is_empty() {
                use_diags.push(summary(p, *span, &errors));
            }
            if !deps.contains(&dep) {
                deps.push(dep);
            }
        }
        if path.is_some() {
            self.loading.pop();
        }
        let order = self.dep_order(&deps);
        if complete {
            // Like the driver: the built-ins, then what the used files define.
            let mut know = self.base.clone();
            for m in &order {
                know.add_items(&m.items);
            }
            if let Some(found) = guard(|| check::check(&items, &know, false)) {
                diags.extend(found);
            }
        }
        let mut tops: HashMap<Sym, DefId> = HashMap::new();
        for m in &order {
            for (s, idx) in &m.index.tops {
                tops.insert(*s, DefId { file: m.file, idx: *idx });
            }
        }
        let index = guard(|| index::build(&items, file, text, &tops)).unwrap_or_default();
        Module {
            path: path.map(Path::to_path_buf),
            uri: uri.to_string(),
            file,
            parsed,
            complete,
            items,
            base,
            used,
            deps,
            diags,
            use_diags,
            dep_defs: tops,
            index,
        }
    }

    /// Loads a used file, once.
    fn load_dep(&mut self, path: &Path) -> Rc<Module> {
        if let Some(m) = self.modules.get(path) {
            return m.clone();
        }
        let uri = self.sources.uri(path);
        let m = match self.sources.read(path) {
            Some(text) => self.load(Some(path), &uri, &text),
            None => {
                let shown = path.display();
                let key = format!("{}{shown}", self.purpose);
                let file = self.ids.register(&key, &short_name(&uri), "");
                let d = Diagnostic::error("E017", t!("cannot read the file `{shown}`", "impossible de lire le fichier `{shown}`"));
                Module {
                    path: Some(path.to_path_buf()),
                    uri,
                    file,
                    parsed: false,
                    complete: false,
                    items: Vec::new(),
                    base: path.parent().map(Path::to_path_buf).unwrap_or_default(),
                    used: Vec::new(),
                    deps: Vec::new(),
                    diags: vec![d],
                    use_diags: Vec::new(),
                    dep_defs: HashMap::new(),
                    index: Index { file, ..Index::default() },
                }
            }
        };
        let m = Rc::new(m);
        self.modules.insert(path.to_path_buf(), m.clone());
        m
    }

    /// The errors of a used file and of the files it uses.
    fn errors_in(&self, path: &Path) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        let mut todo = vec![path.to_path_buf()];
        while let Some(p) = todo.pop() {
            if !seen.insert(p.clone()) {
                continue;
            }
            if let Some(m) = self.modules.get(&p) {
                out.extend(m.diags.iter().filter(|d| d.is_error()).cloned());
                todo.extend(m.deps.iter().rev().cloned());
            }
        }
        out
    }

    /// The used files in the order the checker learns their definitions
    /// (the files a file uses come before it), each once.
    fn dep_order(&self, deps: &[PathBuf]) -> Vec<Rc<Module>> {
        fn visit(l: &Loader, p: &PathBuf, seen: &mut HashSet<PathBuf>, out: &mut Vec<Rc<Module>>) {
            if !seen.insert(p.clone()) {
                return;
            }
            if let Some(m) = l.modules.get(p) {
                for d in &m.deps {
                    visit(l, d, seen, out);
                }
                out.push(m.clone());
            }
        }
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for d in deps {
            visit(self, d, &mut seen, &mut out);
        }
        out
    }
}

/// On a `use` line: the used file has errors.
fn summary(p: &str, span: Span, errors: &[Diagnostic]) -> Diagnostic {
    let n = errors.len();
    let mut d = Diagnostic::error("", t!("the file `{p}` has {n} error(s)", "le fichier `{p}` contient {n} erreur(s)")).at(span);
    if let Some(first) = errors.first() {
        let place = first.span.map(|s| format!(" ({})", source::location(s))).unwrap_or_default();
        d = d.note(t!("first: {}{place}", "la première : {}{place}", first.message));
    }
    for e in errors.iter().take(20) {
        if let Some(s) = e.span {
            let code = if e.code.is_empty() { String::new() } else { format!("[{}]", e.code) };
            d = d.secondary(s, format!("{}{code}: {}", t!("error", "erreur"), e.message));
        }
    }
    d.help(t!("open `{p}` to see and fix them", "ouvrez `{p}` pour les voir et les corriger"))
}

/// A syntax tree for a text with syntax errors: the lines where the parser
/// stops are blanked out (keeping every offset), and missing `end`s are
/// added at the end, until the rest can be read.
fn recover(text: &str, file: u32) -> Option<Vec<Item>> {
    let mut t = text.to_string();
    for _ in 0..32 {
        let d = match lexer::lex(&t, file).and_then(parser::parse) {
            Ok(items) => return Some(items),
            Err(d) => d,
        };
        if d.code == "E020" {
            t.push_str("\nend");
            continue;
        }
        let at = match (d.incomplete, d.secondary.first()) {
            (true, Some((s, _))) => s.start as usize,
            _ => d.span.map_or(t.len(), |s| s.start as usize),
        };
        if !blank_line(&mut t, at.min(text.len())) && !blank_last_line_before(&mut t, at.min(text.len())) {
            return None;
        }
    }
    None
}

/// Replaces a line by spaces. False if there was nothing to blank.
fn blank_line(t: &mut String, mut at: usize) -> bool {
    while !t.is_char_boundary(at) {
        at -= 1;
    }
    let start = t[..at].rfind(['\n', '\r']).map_or(0, |i| i + 1);
    let end = t[at..].find(['\n', '\r']).map_or(t.len(), |i| at + i);
    if t[start..end].trim().is_empty() {
        return false;
    }
    let spaces = " ".repeat(end - start);
    t.replace_range(start..end, &spaces);
    true
}

fn blank_last_line_before(t: &mut String, at: usize) -> bool {
    let mut pos = at.min(t.len());
    while let Some(i) = t[..pos].rfind(|c: char| !c.is_whitespace()) {
        if blank_line(t, i) {
            return true;
        }
        pos = i;
    }
    false
}

/// Everything known about an open document.
pub struct Analysis {
    pub uri: String,
    pub path: Option<PathBuf>,
    pub text: Rc<str>,
    pub lines: LineIndex,
    pub module: Module,
    /// The files it uses, directly or not.
    pub modules: HashMap<PathBuf, Rc<Module>>,
}

impl Analysis {
    pub fn new(loader: Loader, module: Module, text: Rc<str>) -> Analysis {
        Analysis { uri: module.uri.clone(), path: module.path.clone(), lines: LineIndex::new(&text), text, modules: loader.modules, module }
    }

    pub fn index(&self) -> &Index {
        &self.module.index
    }

    /// The module of a source id: this document or a file it uses.
    pub fn module_of(&self, file: u32) -> Option<&Module> {
        if file == self.module.file {
            return Some(&self.module);
        }
        self.modules.values().find(|m| m.file == file).map(|m| &**m)
    }

    pub fn def(&self, id: DefId) -> Option<&Def> {
        self.module_of(id.file)?.index.def(id.idx)
    }

    pub fn uri_of(&self, file: u32) -> Option<&str> {
        self.module_of(file).map(|m| m.uri.as_str())
    }

    /// Every problem to show in this document.
    pub fn diagnostics(&self) -> impl Iterator<Item = &Diagnostic> {
        self.module.diags.iter().chain(self.module.use_diags.iter())
    }

    /// The top-level definitions of the files this document uses.
    pub fn dep_tops(&self) -> Vec<(DefId, &Def)> {
        let mut out: Vec<(DefId, &Def)> = self.module.dep_defs.values().filter_map(|id| Some((*id, self.def(*id)?))).collect();
        out.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        out
    }
}
