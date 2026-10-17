//! Loads files (and the files they `use`), checks them, runs them, runs tests.

use std::collections::{HashMap, HashSet};
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::ast::{FnDef, Item, Stmt};
use crate::check::{self, Knowledge};
use crate::diagnostic::{Diagnostic, Severity};
use crate::interp::{Interp, Mode, Wrap};
use crate::io::out_line;
use crate::source::Span;
use crate::value::Value;
use crate::{lexer, parser, source, suggest};

/// The error was already shown to the user.
#[derive(Debug)]
pub struct Reported;

pub struct Module {
    pub path: Option<PathBuf>,
    pub items: Vec<Item>,
    pub deps: Vec<PathBuf>,
}

pub struct Driver {
    pub interp: Interp,
    modules: HashMap<PathBuf, Rc<Module>>,
    executed: HashSet<PathBuf>,
    loading: Vec<PathBuf>,
    pub color: bool,
    pub errors: usize,
    pub warnings: usize,
    /// In the REPL, names can be defined again.
    pub repl: bool,
    /// The function defined by the previous REPL input: typing `fn fib(0) = 0`
    /// then `fn fib(n) = ...` gives one function with two definitions.
    last_repl_fn: Option<Rc<FnDef>>,
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
            color: crate::diagnostic::use_color(),
            errors: 0,
            warnings: 0,
            repl: false,
            last_repl_fn: None,
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
        let file = source::add(name, src);
        let lexed = lexer::lex(src, file)?;
        parser::parse(lexed)
    }

    /// Like `load_file`, but reads the file again even if it was loaded before
    /// (the REPL's `:load` after the file was edited).
    pub fn reload_file(&mut self, path: &Path) -> Option<Rc<Module>> {
        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        self.modules.remove(&canon);
        self.executed.remove(&canon);
        self.load_file(path)
    }

    /// Parses and checks a file and the files it uses, reporting any problem.
    pub fn load_file(&mut self, path: &Path) -> Option<Rc<Module>> {
        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if let Some(m) = self.modules.get(&canon) {
            return Some(m.clone());
        }
        let src = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                let shown = path.display();
                self.report(&Diagnostic::error(
                    "E017",
                    t!("cannot read the file `{shown}`: {e}", "impossible de lire le fichier `{shown}` : {e}"),
                ));
                return None;
            }
        };
        let base = path.parent().map(Path::to_path_buf).unwrap_or_default();
        self.loading.push(canon.clone());
        let m = self.load_source(&path.display().to_string(), &src, Some(canon.clone()), &base);
        self.loading.pop();
        let m = m?;
        self.modules.insert(canon, m.clone());
        Some(m)
    }

    /// Parses and checks source code. `base` is the folder used to find `use`d files.
    pub fn load_source(&mut self, name: &str, src: &str, path: Option<PathBuf>, base: &Path) -> Option<Rc<Module>> {
        let mut items = match self.parse(name, src) {
            Ok(items) => items,
            Err(d) => {
                self.report(&d);
                return None;
            }
        };
        let repl_fn = if self.repl { self.merge_repl_definitions(&mut items) } else { None };
        let mut deps = Vec::new();
        let mut ok = true;
        for item in &items {
            let Item::Use(p, span) = item else { continue };
            let Some(dep) = self.resolve_use(p, *span, base) else {
                ok = false;
                continue;
            };
            if self.loading.contains(&dep) {
                self.report(
                    &Diagnostic::error(
                        "E017",
                        t!(
                            "`{p}` uses itself (directly or through other files)",
                            "`{p}` s'utilise lui-même (directement ou via d'autres fichiers)"
                        ),
                    )
                    .at(*span)
                    .help(t!(
                        "move what both files need into a third file, used by both",
                        "déplacez ce dont les deux fichiers ont besoin dans un troisième fichier, utilisé par les deux"
                    )),
                );
                ok = false;
                continue;
            }
            match self.load_file(&dep) {
                Some(_) => deps.push(dep),
                None => ok = false,
            }
        }
        if !ok {
            return None;
        }
        let mut know = Knowledge::from_interp(&self.interp);
        let mut seen = HashSet::new();
        for d in &deps {
            self.add_dep_knowledge(d, &mut know, &mut seen);
        }
        let mut diags = check::check(&items, &know, self.repl);
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
        }
        Some(Rc::new(Module { path, items, deps }))
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

    fn add_dep_knowledge(&self, dep: &PathBuf, know: &mut Knowledge, seen: &mut HashSet<PathBuf>) {
        if !seen.insert(dep.clone()) {
            return;
        }
        if let Some(m) = self.modules.get(dep) {
            for d in &m.deps {
                self.add_dep_knowledge(d, know, seen);
            }
            know.add_items(&m.items);
        }
    }

    fn resolve_use(&mut self, p: &str, span: Span, base: &Path) -> Option<PathBuf> {
        let mut path = base.join(p);
        if path.extension().is_none() {
            path.set_extension("ry");
        }
        if path.is_file() {
            return Some(path.canonicalize().unwrap_or(path));
        }
        let mut d = Diagnostic::error("E017", t!("file `{p}` not found", "fichier `{p}` introuvable")).at(span).note(t!(
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
        if let Some(best) = suggest::closest(&wanted, files.iter().map(|s| s.as_str())) {
            d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
        }
        d = d.help(t!(
            "the path is relative to the folder of the file that contains `use`",
            "le chemin est relatif au dossier du fichier qui contient `use`"
        ));
        self.report(&d);
        None
    }

    /// Runs a checked module, after the modules it uses.
    pub fn execute(&mut self, m: &Rc<Module>, mode: Mode) -> Result<Option<Value>, Reported> {
        for dep in &m.deps {
            if self.executed.contains(dep) {
                continue;
            }
            self.executed.insert(dep.clone());
            if let Some(dm) = self.modules.get(dep).cloned() {
                self.execute(&dm, mode)?;
            }
        }
        if let Some(p) = &m.path {
            self.executed.insert(p.clone());
        }
        match self.interp.exec_items(&m.items, mode) {
            Ok(v) => Ok(v),
            Err(d) => {
                self.report(&d);
                Err(Reported)
            }
        }
    }

    /// Runs the collected tests and prints a summary. Returns true if all passed.
    pub fn run_tests(&mut self) -> bool {
        let tests = std::mem::take(&mut self.interp.tests);
        if tests.is_empty() {
            out_line(&t!(
                "No tests found. Write one like this:\n  test \"addition\" = assert_eq(1 + 1, 2)",
                "Aucun test trouvé. Écrivez-en un comme ceci :\n  test \"addition\" = assert_eq(1 + 1, 2)"
            ));
            return true;
        }
        let color = std::io::stdout().is_terminal() && crate::diagnostic::terminal_supports_color();
        let (green, red, reset) = if color { ("\x1b[32m", "\x1b[31m", "\x1b[0m") } else { ("", "", "") };
        let n = tests.len();
        out_line(&t!("Running {n} test(s)", "Exécution de {n} test(s)"));
        let mut failed = 0;
        for test in &tests {
            let outcome = match self.interp.eval_test(&test.body) {
                Ok(Value::Bool(false)) => {
                    Some(Diagnostic::error("E015", t!("the test returned `false`", "le test a renvoyé `false`")).at(test.span).help(t!(
                        "use `assert_eq(actual, expected)` to see the values",
                        "utilisez `assert_eq(obtenu, attendu)` pour voir les valeurs"
                    )))
                }
                // `Err(...)` or `None`: a `?` stopped the test, or the test gave a failure value.
                Ok(v) if matches!(self.interp.wrap(&v), Wrap::Err | Wrap::None) => Some(
                    Diagnostic::error("E015", t!("the test ended with {}", "le test s'est terminé avec {}", v.repr())).at(test.span).help(
                        t!(
                            "a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`",
                            "un test réussit s'il se termine sans erreur, et que sa valeur n'est ni `false`, ni `Err(...)`, ni `None`"
                        ),
                    ),
                ),
                Ok(_) => None,
                Err(d) => Some(*d),
            };
            match outcome {
                None => out_line(&format!("  {green}✓{reset} {}", test.name)),
                Some(d) => {
                    failed += 1;
                    out_line(&format!("  {red}✗{reset} {}", test.name));
                    self.report(&d);
                }
            }
        }
        let passed = n - failed;
        let summary = t!("{n} test(s): {passed} passed, {failed} failed", "{n} test(s) : {passed} réussi(s), {failed} échoué(s)");
        let color = if failed == 0 { green } else { red };
        out_line(&format!("{color}{summary}{reset}"));
        failed == 0
    }
}
