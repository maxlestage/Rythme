//! Loads files (and the files they `use`), checks them, runs them, runs tests.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::ast::{FnDef, Item, Stmt, Use};
use crate::check::{self, Knowledge};
use crate::diagnostic::{Diagnostic, Severity};
use crate::interp::{Interp, Mode};
use crate::io::out_line;
use crate::source::Span;
use crate::symbol::{Sym, SymMap};
use crate::value::Value;
use crate::{lexer, parser, source, suggest};

/// The error was already shown to the user.
#[derive(Debug)]
pub struct Reported;

/// The folder `base` followed by the path written in `use`. Its parts,
/// separated by `/` (or `\`), are added one by one, `..` going up a folder,
/// so that the same program works on every system.
pub(crate) fn join_use(base: &Path, p: &str) -> PathBuf {
    if Path::new(p).is_absolute() {
        return PathBuf::from(p);
    }
    let mut path = base.to_path_buf();
    for part in p.split(['/', '\\']) {
        match part {
            "" | "." => {}
            ".." if path.file_name().is_some() => {
                path.pop();
            }
            part => path.push(part),
        }
    }
    path
}

/// `path` is a folder: a message that lists the Rythme files inside it.
pub fn folder_error(path: &Path) -> Diagnostic {
    let shown = path.display();
    let mut d = Diagnostic::error("E017", t!("`{shown}` is a folder, not a file", "`{shown}` est un dossier, pas un fichier"));
    let mut files: Vec<String> = std::fs::read_dir(path)
        .map(|entries| entries.flatten().map(|e| e.file_name().to_string_lossy().to_string()).filter(|n| n.ends_with(".ry")).collect())
        .unwrap_or_default();
    files.sort();
    if !files.is_empty() {
        let list: Vec<String> = files.iter().take(5).map(|f| format!("`{}`", path.join(f).display())).collect();
        d = d.help(t!("give a file inside it, like {}", "donnez un fichier qu'il contient, comme {}", list.join(", ")));
    }
    d
}

/// Reads a source file, explaining in the user's language why it cannot be read.
pub fn read_source(path: &Path) -> Result<String, Box<Diagnostic>> {
    let shown = path.display();
    if path.is_dir() {
        return Err(folder_error(path).boxed());
    }
    let bytes = std::fs::read(path).map_err(|e| {
        let msg = match e.kind() {
            std::io::ErrorKind::NotFound => t!("file `{shown}` not found", "fichier `{shown}` introuvable"),
            std::io::ErrorKind::PermissionDenied => {
                t!("no permission to read the file `{shown}`", "pas la permission de lire le fichier `{shown}`")
            }
            _ => t!("cannot read the file `{shown}`: {e}", "impossible de lire le fichier `{shown}` : {e}"),
        };
        Diagnostic::error("E017", msg).boxed()
    })?;
    String::from_utf8(bytes).map_err(|e| {
        let bytes = e.as_bytes();
        let at = e.utf8_error().valid_up_to();
        let line = bytes[..at].iter().filter(|&&b| b == b'\n').count() + 1;
        let mut d = Diagnostic::error(
            "E017",
            t!(
                "the file `{shown}` is not text in the UTF-8 encoding (unexpected byte on line {line})",
                "le fichier `{shown}` n'est pas du texte dans l'encodage UTF-8 (octet inattendu à la ligne {line})"
            ),
        );
        if bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]) {
            d = d.note(t!("it looks like it was saved in UTF-16", "il semble avoir été enregistré en UTF-16"));
        }
        d.help(t!(
            "save it again with the UTF-8 encoding (most editors have it in \"Save as\" or in the status bar)",
            "enregistrez-le à nouveau avec l'encodage UTF-8 (la plupart des éditeurs le proposent dans « Enregistrer sous » ou dans la barre d'état)"
        ))
        .boxed()
    })
}

/// A file loaded in a namespace (see `Module::ns`), by its canonical path.
pub type UnitKey = (PathBuf, u32);

pub struct Module {
    pub path: Option<PathBuf>,
    pub items: Vec<Item>,
    /// The files it uses without a name (`use "x.ry"`), in its namespace.
    pub deps: Vec<UnitKey>,
    /// The files it uses with a name (`use geometry`), with that name.
    pub named: Vec<(Sym, UnitKey)>,
    /// Where its top-level names live: 0 for the program and the files it
    /// uses without a name; each file used with a name has its own, shared
    /// with the files it uses without a name.
    pub ns: u32,
    /// Its number, for its private names; `None` for the program itself,
    /// whose private names are simply its own.
    pub unit: Option<u32>,
    /// Its path as shown in messages.
    pub file: Rc<str>,
}

pub struct Driver {
    pub interp: Interp,
    modules: HashMap<UnitKey, Rc<Module>>,
    executed: HashSet<UnitKey>,
    /// The files being loaded, each one used by the one before.
    loading: Vec<(PathBuf, Rc<str>)>,
    /// The namespace of each file used with a name.
    namespaces: HashMap<PathBuf, u32>,
    units: u32,
    /// In the REPL, the files used with a name by earlier inputs.
    repl_modules: Vec<(Sym, UnitKey)>,
    pub color: bool,
    pub errors: usize,
    pub warnings: usize,
    /// In the REPL, names can be defined again.
    pub repl: bool,
    /// The function defined by the previous REPL input: typing `fn fib(0) = 0`
    /// then `fn fib(n) = ...` gives one function with two definitions.
    last_repl_fn: Option<Rc<FnDef>>,
    /// The next source read has a tail from somewhere else (see
    /// [`source::add_with_tail`]): where it starts, and its name.
    pub tail: Option<(usize, String)>,
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            interp: Interp::new(),
            modules: HashMap::new(),
            executed: HashSet::new(),
            loading: Vec::new(),
            namespaces: HashMap::new(),
            units: 0,
            repl_modules: Vec::new(),
            color: crate::diagnostic::use_color(),
            errors: 0,
            warnings: 0,
            repl: false,
            last_repl_fn: None,
            tail: None,
        }
    }

    pub fn report(&mut self, d: &Diagnostic) {
        match d.severity {
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
        }
        crate::io::err(&format!("{}\n", d.render(self.color)));
    }

    pub fn parse(&mut self, name: &str, src: &str) -> Result<Vec<Item>, Box<Diagnostic>> {
        let file = match self.tail.take() {
            Some((at, tail)) => source::add_with_tail(name, src, at, &tail),
            None => source::add(name, src),
        };
        let lexed = lexer::lex(src, file)?;
        parser::parse(lexed)
    }

    /// Like `load_file`, but reads the file again even if it was loaded before
    /// (the REPL's `:load` after the file was edited).
    pub fn reload_file(&mut self, path: &Path) -> Option<Rc<Module>> {
        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        self.modules.remove(&(canon.clone(), 0));
        self.executed.remove(&(canon, 0));
        self.load_file(path)
    }

    /// Parses and checks a program and the files it uses, reporting any problem.
    pub fn load_file(&mut self, path: &Path) -> Option<Rc<Module>> {
        self.load_unit(path, 0, false)
    }

    /// Parses and checks a file in the namespace `ns`; `used` when a `use`
    /// asks for it (its private names then stay its own).
    fn load_unit(&mut self, path: &Path, ns: u32, used: bool) -> Option<Rc<Module>> {
        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let key = (canon.clone(), ns);
        if let Some(m) = self.modules.get(&key) {
            return Some(m.clone());
        }
        let src = match read_source(path) {
            Ok(s) => s,
            Err(d) => {
                self.report(&d);
                return None;
            }
        };
        let base = path.parent().map(Path::to_path_buf).unwrap_or_default();
        let shown = path.display().to_string();
        self.loading.push((canon.clone(), Rc::from(shown.as_str())));
        let m = self.load_in(&shown, &src, Some(canon), &base, ns, used);
        self.loading.pop();
        let m = m?;
        self.modules.insert(key, m.clone());
        Some(m)
    }

    /// Parses and checks source code. `base` is the folder used to find `use`d files.
    pub fn load_source(&mut self, name: &str, src: &str, path: Option<PathBuf>, base: &Path) -> Option<Rc<Module>> {
        self.load_in(name, src, path, base, 0, false)
    }

    fn load_in(&mut self, name: &str, src: &str, path: Option<PathBuf>, base: &Path, ns: u32, used: bool) -> Option<Rc<Module>> {
        let mut items = match self.parse(name, src) {
            Ok(items) => items,
            Err(d) => {
                self.report(&d);
                return None;
            }
        };
        let mut deps: Vec<(UnitKey, Span)> = Vec::new();
        let mut named: Vec<(Sym, UnitKey)> = if self.repl { self.repl_modules.clone() } else { Vec::new() };
        let mut named_at: HashMap<Sym, (PathBuf, Rc<str>)> = HashMap::new();
        let mut ok = true;
        for item in &items {
            let Item::Use(u) = item else { continue };
            let Some(dep_path) = self.resolve_use(u, base) else {
                ok = false;
                continue;
            };
            // Modules are known by their canonical path; they are read through the
            // path written in the code (shorter in messages, and on Windows a
            // canonical path does not accept `/` in the paths joined to it).
            let dep = dep_path.canonicalize().unwrap_or_else(|_| dep_path.clone());
            if let Some(i) = self.loading.iter().position(|(p, _)| *p == dep) {
                let chain: Vec<Rc<str>> = self.loading[i..].iter().map(|(_, f)| f.clone()).collect();
                self.report(&use_cycle(u, &chain));
                ok = false;
                continue;
            }
            let Some((n, nspan)) = u.name else {
                match self.load_unit(&dep_path, ns, true) {
                    Some(_) => deps.push(((dep, ns), u.span)),
                    None => ok = false,
                }
                continue;
            };
            let shown: Rc<str> = Rc::from(dep_path.display().to_string());
            if let Some((other, other_shown)) = named_at.get(&n) {
                if *other != dep {
                    self.report(&same_module_name(n, nspan, other_shown, &shown));
                    ok = false;
                    continue;
                }
            }
            named_at.insert(n, (dep.clone(), shown));
            let next = self.namespaces.len() as u32 + 1;
            let dep_ns = *self.namespaces.entry(dep.clone()).or_insert(next);
            match self.load_unit(&dep_path, dep_ns, true) {
                Some(_) => {
                    named.retain(|(m, _)| *m != n);
                    named.push((n, (dep, dep_ns)));
                }
                None => ok = false,
            }
        }
        if !ok {
            return None;
        }
        let names: HashSet<Sym> = named.iter().map(|(n, _)| *n).collect();
        crate::modules::qualify(&mut items, &names);
        let repl_fn = if self.repl { self.merge_repl_definitions(&mut items) } else { None };
        let mut know = Knowledge::from_interp(&self.interp);
        let base_know = know.clone();
        let mut seen = HashSet::new();
        let mut origin: HashMap<Sym, Rc<str>> = HashMap::new();
        let mut twice = Vec::new();
        for (d, span) in &deps {
            let before = seen.clone();
            self.add_dep_knowledge(d, &mut know, &mut seen);
            // The same name from two used files: which one wins would depend on their order.
            for k in seen.difference(&before) {
                let Some(m) = self.modules.get(k) else { continue };
                for (s, _) in crate::modules::top_names(&m.items) {
                    if crate::modules::is_private(s) {
                        continue;
                    }
                    match origin.get(&s) {
                        Some(f) if **f != *m.file => twice.push(defined_twice(s, *span, f, &m.file)),
                        _ => {
                            origin.insert(s, m.file.clone());
                        }
                    }
                }
            }
        }
        for d in &twice {
            self.report(d);
        }
        if !twice.is_empty() {
            return None;
        }
        for (n, key) in &named {
            self.add_module_knowledge(*n, key, &base_know, &mut know);
        }
        let mut diags = check::check(&items, &know, self.repl);
        if diags.iter().any(|d| d.code == "E003") {
            // A name defined by another file of the folder: say which one.
            let mut skip: Vec<PathBuf> = seen.iter().map(|(p, _)| p.clone()).collect();
            skip.extend(named.iter().map(|(_, (p, _))| p.clone()));
            skip.extend(path.clone());
            know.nearby = nearby_names(base, &skip);
            if !know.nearby.is_empty() {
                diags = check::check(&items, &know, self.repl);
            }
        }
        diags.sort_by_key(|d| (d.severity != Severity::Error, d.span.map(|s| (s.file, s.start))));
        let failed = diags.iter().any(|d| d.is_error());
        for d in &diags {
            self.report(d);
        }
        if failed {
            return None;
        }
        if self.repl {
            self.last_repl_fn = repl_fn;
            self.repl_modules = named.clone();
        }
        let unit = used.then(|| {
            self.units += 1;
            self.units
        });
        let deps = deps.into_iter().map(|(k, _)| k).collect();
        Some(Rc::new(Module { path, items, deps, named, ns, unit, file: Rc::from(name) }))
    }

    /// In the REPL, a lone `fn` with the same name and number of parameters as
    /// the function typed just before adds a definition to it instead of
    /// replacing it. Returns the function this input defines, if any.
    fn merge_repl_definitions(&mut self, items: &mut [Item]) -> Option<Rc<FnDef>> {
        let [Item::Stmt(Stmt::Fn(def))] = items else { return None };
        let Some(prev) = &self.last_repl_fn else { return Some(def.clone()) };
        if prev.name != def.name || prev.arity != def.arity {
            return Some(def.clone());
        }
        let mut clauses = prev.clauses.clone();
        clauses.extend(def.clauses.iter().cloned());
        let merged = Rc::new(FnDef {
            name: def.name,
            clauses,
            arity: def.arity,
            doc: prev.doc.clone().or_else(|| def.doc.clone()),
            span: prev.span,
        });
        let (name, n) = (def.name.map(|s| s.to_string()).unwrap_or_default(), merged.clauses.len());
        out_line(&t!(
            "(`{name}` now has {n} definitions, tried in order; :reset to start again)",
            "(`{name}` a maintenant {n} définitions, essayées dans l'ordre ; :reset pour recommencer)"
        ));
        *def = merged.clone();
        Some(merged)
    }

    /// Adds what a file used without a name defines (and the files it uses
    /// without a name), once each.
    fn add_dep_knowledge(&self, dep: &UnitKey, know: &mut Knowledge, seen: &mut HashSet<UnitKey>) {
        if !seen.insert(dep.clone()) {
            return;
        }
        if let Some(m) = self.modules.get(dep) {
            for d in &m.deps {
                self.add_dep_knowledge(d, know, seen);
            }
            know.add_used_items(&m.items, &m.file);
        }
    }

    /// Adds what the file used as `name` defines, known as `name.area`.
    fn add_module_knowledge(&self, name: Sym, key: &UnitKey, base: &Knowledge, know: &mut Knowledge) {
        let Some(m) = self.modules.get(key) else { return };
        let mut inside = base.clone();
        let mut seen = HashSet::new();
        for d in &m.deps {
            self.add_dep_knowledge(d, &mut inside, &mut seen);
        }
        inside.add_items(&m.items);
        let mut items: Vec<&[Item]> = vec![&m.items];
        for k in &seen {
            if let Some(dm) = self.modules.get(k) {
                items.push(&dm.items);
            }
        }
        know.add_module(name, m.file.clone(), &inside, &items);
    }

    fn resolve_use(&mut self, u: &Use, base: &Path) -> Option<PathBuf> {
        match find_use(u, base) {
            Ok(path) => Some(path),
            Err(d) => {
                self.report(&d);
                None
            }
        }
    }

    /// The public top-level names (and type names) of the files of namespace `ns`.
    fn namespace_names(&self, ns: u32) -> Vec<Sym> {
        let mut out = Vec::new();
        for ((_, n), m) in &self.modules {
            if *n != ns {
                continue;
            }
            let names = crate::modules::top_names(&m.items).into_iter().map(|(s, _)| s);
            out.extend(names.filter(|s| !crate::modules::is_private(*s)));
            out.extend(m.items.iter().filter_map(|it| if let Item::Type(t) = it { Some(t.name) } else { None }));
        }
        out
    }

    /// How the interpreter stores the top-level names of a file (see `compile::Names`).
    fn names_of(&self, m: &Module) -> crate::compile::Names {
        let mut map = SymMap::default();
        if m.ns != 0 {
            let prefix = m.ns.to_string();
            for s in self.namespace_names(m.ns) {
                map.insert(s, Sym::stored(&prefix, s));
            }
        }
        if let Some(unit) = m.unit {
            let prefix = format!("p{unit}");
            for (s, _) in crate::modules::top_names(&m.items) {
                if crate::modules::is_private(s) {
                    map.insert(s, Sym::stored(&prefix, s));
                }
            }
        }
        for (n, (_, ns)) in &m.named {
            let prefix = ns.to_string();
            for s in self.namespace_names(*ns) {
                map.insert(Sym::new(&format!("{n}.{s}")), Sym::stored(&prefix, s));
            }
        }
        Rc::new(map)
    }

    /// Runs a checked module, after the modules it uses.
    pub fn execute(&mut self, m: &Rc<Module>, mode: Mode) -> Result<Option<Value>, Reported> {
        self.prepare(m, mode)?;
        match self.interp.exec_items(&m.items, mode) {
            Ok(v) => Ok(v),
            Err(d) => {
                self.report(&d);
                Err(Reported)
            }
        }
    }

    /// Runs the modules that a checked module uses, and makes the interpreter
    /// ready to run its own items (`execute` without the items themselves).
    pub fn prepare(&mut self, m: &Rc<Module>, mode: Mode) -> Result<(), Reported> {
        for dep in m.deps.iter().chain(m.named.iter().map(|(_, k)| k)) {
            if self.executed.contains(dep) {
                continue;
            }
            self.executed.insert(dep.clone());
            if let Some(dm) = self.modules.get(dep).cloned() {
                self.execute(&dm, mode)?;
            }
        }
        if let Some(p) = &m.path {
            self.executed.insert((p.clone(), m.ns));
        }
        self.interp.names = self.names_of(m);
        Ok(())
    }

    /// Runs the collected tests and prints a summary. Returns true if all passed.
    pub fn run_tests(&mut self) -> bool {
        let (_, failed) = self.run_tests_counted();
        failed == 0
    }

    /// Runs the collected tests and prints a summary. Returns the number of
    /// tests and the number of failed ones (see [`crate::testing`]).
    pub fn run_tests_counted(&mut self) -> (usize, usize) {
        let options = crate::testing::Options { seed: self.interp.property_seed, ..Default::default() };
        crate::testing::run_and_summarize(self, &options)
    }

    /// The files loaded so far: the program and the files it uses.
    pub fn loaded_files(&self) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = self.modules.keys().map(|(p, _)| p.clone()).collect();
        files.sort();
        files.dedup();
        files
    }
}

/// The file named by `use`, looked for in the folder `base`, or an error
/// explaining that it is missing (also used by the language server).
pub fn find_use(u: &Use, base: &Path) -> Result<PathBuf, Box<Diagnostic>> {
    let (p, span) = (&*u.path, u.span);
    let mut path = join_use(base, p);
    if path.extension().is_none() {
        path.set_extension("ry");
    }
    if path.is_file() {
        return Ok(path);
    }
    // `use mathlib/vectors`: a file of a package of the project.
    if let Some(found) = crate::project::resolve_use(p, base, u.path_span) {
        return found;
    }
    for dir in [&path, &join_use(base, p)] {
        if dir.is_dir() {
            return Err(folder_error(dir).at(span).boxed());
        }
    }
    let shown = if u.short { p.trim_end_matches(".ry") } else { p };
    let mut d = Diagnostic::error("E017", t!("file `{p}` not found", "fichier `{p}` introuvable")).at(u.path_span).note(t!(
        "looked for: {}",
        "cherché : {}",
        path.display()
    ));
    let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
    let wanted = path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(if dir.as_os_str().is_empty() { Path::new(".") } else { &dir }) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".ry") {
                files.push(name);
            }
        }
    }
    files.sort();
    // In the form it was written: `use geometry` or `use "geometry.ry"`.
    let written = |f: &str| {
        let folder = shown.rsplit_once('/').map(|(d, _)| format!("{d}/")).unwrap_or_default();
        if u.short {
            format!("use {folder}{}", f.trim_end_matches(".ry"))
        } else {
            format!("use \"{folder}{f}\"")
        }
    };
    if let Some(best) = suggest::closest(&wanted, files.iter().map(|s| s.as_str())) {
        let best = written(&best);
        d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
    } else if !files.is_empty() {
        let list: Vec<String> = files.iter().take(8).map(|f| format!("`{f}`")).collect();
        let more = if files.len() > 8 { ", ..." } else { "" };
        d = d.note(t!("the Rythme files there are: {}{more}", "les fichiers Rythme de ce dossier sont : {}{more}", list.join(", ")));
    }
    d = d.help(t!(
        "the path is relative to the folder of the file that contains `use`",
        "le chemin est relatif au dossier du fichier qui contient `use`"
    ));
    match crate::project::use_hint(p, base) {
        Some(Ok(help)) => d = d.help(help),
        Some(Err(note)) => d = d.note(note),
        None => {}
    }
    Err(d.boxed())
}

/// The error for a `use` when files use each other in a circle: `chain` is
/// the file used, then each file used by the one before, up to the file that
/// contains this `use`.
pub fn use_cycle(u: &Use, chain: &[Rc<str>]) -> Diagnostic {
    let d = match chain {
        [] | [_] => {
            let p = &u.path;
            Diagnostic::error("E017", t!("`{p}` uses itself", "`{p}` s'utilise lui-même"))
        }
        [first, .., last] => {
            let d =
                Diagnostic::error("E017", t!("`{last}` and `{first}` use each other", "`{last}` et `{first}` s'utilisent l'un l'autre"));
            if chain.len() > 2 {
                let circle: Vec<String> = chain.iter().chain([first]).map(|f| format!("`{f}`")).collect();
                d.note(t!("the circle: {}", "le cercle : {}", circle.join(" → ")))
            } else {
                d
            }
        }
    };
    d.at(u.span).help(t!(
        "files cannot use each other: move what both need into a third file, used by both",
        "des fichiers ne peuvent pas s'utiliser l'un l'autre : déplacez ce dont les deux ont besoin dans un troisième fichier, utilisé par les deux"
    ))
}

/// Two different files used with the same name.
fn same_module_name(n: Sym, span: Span, first: &str, second: &str) -> Diagnostic {
    Diagnostic::error(
        "E011",
        t!(
            "the name `{n}` is given to two files: `{first}` and `{second}`",
            "le nom `{n}` est donné à deux fichiers : `{first}` et `{second}`"
        ),
    )
    .at(span)
    .help(t!("give this one another name: `use ... as other_name`", "donnez un autre nom à celui-ci : `use ... as autre_nom`"))
}

/// A name defined by two files used without a name.
fn defined_twice(s: Sym, span: Span, first: &str, second: &str) -> Diagnostic {
    Diagnostic::error(
        "E011",
        t!(
            "`{s}` is defined by two used files: `{first}` and `{second}`",
            "`{s}` est défini par deux fichiers utilisés : `{first}` et `{second}`"
        ),
    )
    .at(span)
    .help(t!(
        "use the files with their names, without quotes, and write which one you mean: `use geometry`, then `geometry.{s}`",
        "utilisez les fichiers par leur nom, sans guillemets, et écrivez celui que vous voulez dire : `use geometrie`, puis `geometrie.{s}`"
    ))
}

/// Can `use name` (without quotes) name this file?
fn short_name(file: &str) -> Option<&str> {
    let stem = file.strip_suffix(".ry")?;
    let mut chars = stem.chars();
    let ok = chars.next().is_some_and(|c| c.is_lowercase() || c == '_')
        && stem.chars().all(|c| c.is_alphanumeric() || c == '_')
        && crate::lexer::lex(stem, u32::MAX).is_ok_and(|l| matches!(l.tokens.first().map(|t| &t.tok), Some(crate::lexer::Tok::Ident(_))));
    ok.then_some(stem)
}

/// The public names defined by the Rythme files of the folder `base` (not
/// those of `skip`), with how to use each file: to say where an unknown
/// name is defined.
pub fn nearby_names(base: &Path, skip: &[PathBuf]) -> HashMap<Sym, (Rc<str>, Rc<str>)> {
    let mut out = HashMap::new();
    let dir = if base.as_os_str().is_empty() { Path::new(".") } else { base };
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    let mut files: Vec<PathBuf> = entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "ry")).collect();
    files.sort();
    for path in files.into_iter().take(64) {
        let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
        if skip.contains(&canon) {
            continue;
        }
        let file = path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
        let Some(stem) = short_name(&file) else { continue };
        let Ok(src) = std::fs::read_to_string(&path) else { continue };
        let Ok(items) = lexer::lex(&src, u32::MAX).and_then(parser::parse) else { continue };
        let (stem, file): (Rc<str>, Rc<str>) = (Rc::from(stem), Rc::from(file));
        for (s, _) in crate::modules::top_names(&items) {
            if !crate::modules::is_private(s) {
                out.entry(s).or_insert_with(|| (stem.clone(), file.clone()));
            }
        }
    }
    out
}
