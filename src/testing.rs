//! `rythme test`: runs the tests of files and folders, with a filter by
//! name, a summary with counts and times, a seed for property tests, and a
//! watch mode that runs them again when a file changes.
//!
//! [`run_paths`] is the reusable part: "test these files and folders", used
//! by the command and by the tools that run tests for the user.

use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use crate::diagnostic::Diagnostic;
use crate::driver::Driver;
use crate::interp::{Mode, TestCase, Wrap};
use crate::io::{err_line, out_line};
use crate::source::Span;
use crate::value::Value;
use crate::{lexer, suggest};

/// How to run tests.
#[derive(Clone, Debug)]
pub struct Options {
    /// Only the tests whose name contains one of these texts (ignoring
    /// case); all the tests when empty.
    pub filters: Vec<String>,
    /// The seed of the values of `check_all` and `sample`.
    pub seed: u64,
    /// What `args()` gives to the programs tested.
    pub script_args: Vec<String>,
    /// `--sandbox`: the tests touch no files, folders or environment variables.
    pub sandbox: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { filters: Vec::new(), seed: crate::builtins::property::DEFAULT_SEED, script_args: Vec::new(), sandbox: false }
    }
}

impl Options {
    /// Does the test named `name` run?
    pub fn selects(&self, name: &str) -> bool {
        let name = name.to_lowercase();
        self.filters.is_empty() || self.filters.iter().any(|f| name.contains(&f.to_lowercase()))
    }
}

/// What one test did.
#[derive(Clone, Debug)]
pub struct TestResult {
    pub name: String,
    /// Where the test is written.
    pub span: Span,
    /// Why it failed, or `None` when it passed.
    pub failure: Option<Diagnostic>,
    pub seconds: f64,
}

impl TestResult {
    pub fn passed(&self) -> bool {
        self.failure.is_none()
    }
}

/// The tests of one file.
#[derive(Clone, Debug)]
pub struct FileReport {
    pub path: PathBuf,
    /// The file (or a file it uses) has errors: its tests could not run.
    pub broken: bool,
    pub results: Vec<TestResult>,
    /// Tests left out by the filter.
    pub filtered_out: usize,
    /// The files it uses (and itself), to watch for changes.
    pub files: Vec<PathBuf>,
}

/// The tests of several files.
#[derive(Clone, Debug, Default)]
pub struct Summary {
    pub files: Vec<FileReport>,
    pub seconds: f64,
}

impl Summary {
    pub fn total(&self) -> usize {
        self.files.iter().map(|f| f.results.len()).sum()
    }

    pub fn failed(&self) -> usize {
        self.files.iter().flat_map(|f| &f.results).filter(|r| !r.passed()).count()
    }

    pub fn broken(&self) -> usize {
        self.files.iter().filter(|f| f.broken).count()
    }

    pub fn filtered_out(&self) -> usize {
        self.files.iter().map(|f| f.filtered_out).sum()
    }

    /// Every test ran and passed, and no file had errors.
    pub fn ok(&self) -> bool {
        self.failed() == 0 && self.broken() == 0
    }
}

fn colors() -> (&'static str, &'static str, &'static str) {
    if std::io::stdout().is_terminal() && crate::diagnostic::terminal_supports_color() {
        ("\x1b[32m", "\x1b[31m", "\x1b[0m")
    } else {
        ("", "", "")
    }
}

/// A duration, as shown in summaries: `12 ms`, `1.25 s`.
pub fn duration(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{} ms", (seconds * 1000.0).round() as u64)
    } else {
        let s = format!("{seconds:.2}");
        // The decimal separator of the language.
        let s = s.replace('.', crate::lang::pick(".", ","));
        format!("{s} s")
    }
}

/// A test that takes longer than this shows its time.
const SLOW: f64 = 0.5;

/// Runs one test of a loaded program.
pub fn run_test(driver: &mut Driver, test: &TestCase) -> TestResult {
    let start = crate::io::now();
    let outcome = driver.interp.eval_test(test);
    let seconds = crate::io::now() - start;
    let failure = match outcome {
        Ok(Value::Bool(false)) => {
            Some(Diagnostic::error("E015", t!("the test returned `false`", "le test a renvoyé `false`")).at(test.span).help(t!(
                "use `assert_eq(actual, expected)` to see the values",
                "utilisez `assert_eq(obtenu, attendu)` pour voir les valeurs"
            )))
        }
        // `Err(...)` or `None`: a `?` stopped the test, or the test gave a failure value.
        Ok(v) if matches!(driver.interp.wrap(&v), Wrap::Err | Wrap::None) => {
            Some(Diagnostic::error("E015", t!("the test ended with {}", "le test s'est terminé avec {}", v.repr())).at(test.span).help(t!(
                "a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`",
                "un test réussit s'il se termine sans erreur, et que sa valeur n'est ni `false`, ni `Err(...)`, ni `None`"
            )))
        }
        Ok(_) => None,
        Err(d) => Some(*d),
    };
    TestResult { name: test.name.to_string(), span: test.span, failure, seconds }
}

/// Runs the tests collected by `Mode::Test` in a loaded program, printing a
/// line for each one and the report of each failure. `title` names the file
/// when several files are tested. Returns the results and the number of
/// tests left out by the filter.
pub fn run_loaded(driver: &mut Driver, options: &Options, title: Option<&str>) -> (Vec<TestResult>, usize) {
    let all = std::mem::take(&mut driver.interp.tests);
    let total = all.len();
    let tests: Vec<TestCase> = all.into_iter().filter(|t| options.selects(&t.name)).collect();
    let left_out = total - tests.len();
    if tests.is_empty() {
        if title.is_none() && total == 0 {
            out_line(&t!(
                "No tests found. Write one like this:\n  test \"addition\" = assert_eq(1 + 1, 2)",
                "Aucun test trouvé. Écrivez-en un comme ceci :\n  test \"addition\" = assert_eq(1 + 1, 2)"
            ));
        }
        return (Vec::new(), left_out);
    }
    let (green, red, reset) = colors();
    let n = tests.len();
    out_line(&match (title, left_out) {
        (None, 0) => t!("Running {n} test(s)", "Exécution de {n} test(s)"),
        (None, _) => t!("Running {n} of {total} test(s)", "Exécution de {n} test(s) sur {total}"),
        (Some(file), 0) => t!("Running {n} test(s) of {file}", "Exécution de {n} test(s) de {file}"),
        (Some(file), _) => t!("Running {n} of {total} test(s) of {file}", "Exécution de {n} test(s) sur {total} de {file}"),
    });
    let mut results = Vec::new();
    for test in &tests {
        let result = run_test(driver, test);
        let time = if result.seconds >= SLOW { format!(" ({})", duration(result.seconds)) } else { String::new() };
        match &result.failure {
            None => out_line(&format!("  {green}✓{reset} {}{time}", test.name)),
            Some(d) => {
                out_line(&format!("  {red}✗{reset} {}{time}", test.name));
                driver.report(d);
            }
        }
        results.push(result);
    }
    (results, left_out)
}

/// The line of counts: `4 test(s): 3 passed, 1 failed (12 ms)`.
fn counts(files: Option<usize>, total: usize, failed: usize, left_out: usize, broken: usize, seconds: f64) -> String {
    let passed = total - failed;
    let mut line = match files {
        Some(f) => t!("{f} file(s), ", "{f} fichier(s), "),
        None => String::new(),
    };
    line.push_str(&t!("{total} test(s): {passed} passed, {failed} failed", "{total} test(s) : {passed} réussi(s), {failed} échoué(s)"));
    if left_out > 0 {
        line.push_str(&t!(", {left_out} left out by the filter", ", {left_out} écarté(s) par le filtre"));
    }
    if broken > 0 {
        line.push_str(&t!(", {broken} file(s) with errors", ", {broken} fichier(s) avec des erreurs"));
    }
    line.push_str(&format!(" ({})", duration(seconds)));
    line
}

/// Runs the tests of a program loaded in `driver` and prints the summary
/// line. Returns the number of tests run and the number of failed ones.
pub fn run_and_summarize(driver: &mut Driver, options: &Options) -> (usize, usize) {
    let start = crate::io::now();
    let (results, left_out) = run_loaded(driver, options, None);
    if results.is_empty() {
        if left_out > 0 {
            no_match(options);
        }
        return (0, 0);
    }
    let failed = results.iter().filter(|r| !r.passed()).count();
    let (green, red, reset) = colors();
    let color = if failed == 0 { green } else { red };
    out_line(&format!("{color}{}{reset}", counts(None, results.len(), failed, left_out, 0, crate::io::now() - start)));
    (results.len(), failed)
}

fn no_match(options: &Options) {
    let filters: Vec<String> = options.filters.iter().map(|f| format!("`{f}`")).collect();
    out_line(&t!("No test name contains {}.", "Aucun nom de test ne contient {}.", filters.join(t!(" or ", " ou ").as_str())));
}

/// Loads a file and runs its tests. `title`: the name shown for the file
/// when several are tested.
pub fn run_file(path: &Path, options: &Options, title: Option<&str>) -> FileReport {
    let mut driver = Driver::new();
    driver.interp.script_args = options.script_args.clone();
    driver.interp.property_seed = options.seed;
    driver.interp.sandboxed = options.sandbox;
    let mut report = FileReport { path: path.to_path_buf(), broken: true, results: Vec::new(), filtered_out: 0, files: Vec::new() };
    if let Some(module) = driver.load_file(path) {
        if driver.execute(&module, Mode::Test).is_ok() {
            let (results, left_out) = run_loaded(&mut driver, options, title);
            report = FileReport { broken: false, results, filtered_out: left_out, ..report };
        }
    }
    if report.broken && title.is_some() {
        let (_, red, reset) = colors();
        out_line(&t!(
            "{red}✗{reset} {}: the file has errors, so its tests could not run",
            "{red}✗{reset} {} : le fichier a des erreurs, ses tests n'ont pas pu être lancés",
            path.display()
        ));
    }
    report.files = driver.loaded_files();
    if !report.files.iter().any(|f| same_file(f, path)) {
        report.files.push(path.to_path_buf());
    }
    report
}

fn same_file(a: &Path, b: &Path) -> bool {
    a == b || matches!((a.canonicalize(), b.canonicalize()), (Ok(x), Ok(y)) if x == y)
}

/// Does this source have tests? A file that cannot be read as Rythme code is
/// kept when it seems to have some, so that its errors are shown.
fn has_tests(src: &str) -> bool {
    match lexer::lex(src, u32::MAX) {
        Ok(lexed) => lexed.tokens.iter().any(|t| t.tok == lexer::Tok::Test),
        Err(_) => src.lines().any(|l| l.trim_start().starts_with("test ")),
    }
}

/// The `.ry` files with tests inside a folder and its subfolders, in order.
/// Hidden folders (`.git`), `target` and `node_modules` are skipped.
fn files_in(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if p.is_dir() {
            if !name.starts_with('.') && name != "target" && name != "node_modules" {
                files_in(&p, out);
            }
        } else if name.ends_with(".ry") && std::fs::read_to_string(&p).is_ok_and(|src| has_tests(&src)) {
            out.push(p);
        }
    }
}

/// The files to test: the files given, and the files with tests inside the
/// folders given. A name without `.ry` finds the file with it.
pub fn test_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>, Box<Diagnostic>> {
    let mut out = Vec::new();
    for p in paths {
        if p.is_dir() {
            files_in(p, &mut out);
        } else if p.is_file() {
            out.push(p.clone());
        } else {
            let with_ext = PathBuf::from(format!("{}.ry", p.display()));
            if p.extension().is_none() && with_ext.is_file() {
                out.push(with_ext);
            } else {
                return Err(not_found(p));
            }
        }
    }
    let mut seen = Vec::new();
    out.retain(|p| {
        let key = p.canonicalize().unwrap_or_else(|_| p.clone());
        let new = !seen.contains(&key);
        seen.push(key);
        new
    });
    Ok(out)
}

fn not_found(path: &Path) -> Box<Diagnostic> {
    let shown = path.display();
    let mut d = Diagnostic::error("E017", t!("file or folder `{shown}` not found", "fichier ou dossier `{shown}` introuvable"));
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let wanted = path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    let names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().is_dir() || e.file_name().to_string_lossy().ends_with(".ry"))
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default();
    if let Some(best) = suggest::closest(&wanted, names.iter().map(String::as_str)) {
        d = d.help(t!("did you mean `{}`?", "vouliez-vous dire `{}` ?", dir.join(best).display()));
    }
    d.boxed()
}

/// Tests files and folders (see [`test_files`]): prints what each test does,
/// then a summary. With one file, prints what `rythme test file.ry` always
/// printed; with several, the name of each file and, at the end, the list of
/// the failed tests.
pub fn run_paths(paths: &[PathBuf], options: &Options) -> Result<Summary, Box<Diagnostic>> {
    let start = crate::io::now();
    let files = test_files(paths)?;
    let several = files.len() > 1 || paths.iter().any(|p| p.is_dir());
    let mut summary = Summary::default();
    if files.is_empty() {
        let shown: Vec<String> = paths.iter().map(|p| p.display().to_string()).collect();
        out_line(&t!(
            "No file with tests in {}. Write a test in a `.ry` file like this:\n  test \"addition\" = assert_eq(1 + 1, 2)",
            "Aucun fichier avec des tests dans {}. Écrivez un test dans un fichier `.ry` comme ceci :\n  test \"addition\" = assert_eq(1 + 1, 2)",
            shown.join(", ")
        ));
        return Ok(summary);
    }
    for file in &files {
        let shown = file.display().to_string();
        summary.files.push(run_file(file, options, several.then_some(shown.as_str())));
    }
    summary.seconds = crate::io::now() - start;
    let (total, failed, broken, left_out) = (summary.total(), summary.failed(), summary.broken(), summary.filtered_out());
    if !several {
        if broken > 0 {
            return Ok(summary);
        }
        if total == 0 {
            if left_out > 0 {
                no_match(options);
            }
            return Ok(summary);
        }
    }
    if total == 0 && left_out > 0 {
        no_match(options);
    }
    let (green, red, reset) = colors();
    if failed >= 2 || (several && failed > 0) {
        out_line(&format!("{red}{}{reset}", t!("Failed tests:", "Tests échoués :")));
        for f in &summary.files {
            for r in f.results.iter().filter(|r| !r.passed()) {
                let at = crate::source::location(r.span);
                out_line(&format!("  {red}✗{reset} {} ({at})", r.name));
            }
        }
    }
    let color = if summary.ok() { green } else { red };
    let files = several.then_some(summary.files.len());
    out_line(&format!("{color}{}{reset}", counts(files, total, failed, left_out, broken, summary.seconds)));
    Ok(summary)
}

/// The exit code of a summary: 0 when all is well, 1 otherwise.
pub fn exit_code(summary: &Summary, options: &Options) -> i32 {
    // A filter that selects nothing is a mistake (a misspelt name).
    let nothing = summary.total() == 0 && summary.filtered_out() > 0 && !options.filters.is_empty();
    i32::from(!summary.ok() || nothing)
}

// ----- the command ---------------------------------------------------------

fn usage(msg: String) -> i32 {
    err_line(&msg);
    2
}

/// The words of `rythme test`, read.
pub struct Args {
    pub options: Options,
    pub paths: Vec<PathBuf>,
    pub watch: bool,
    /// `--seed random`.
    pub random_seed: bool,
}

/// `rythme test PATH... [--filter NAME] [--seed N] [--watch] [-- ARGS...]`;
/// without a path, in a project, the files of the project that have tests.
pub fn command(args: &[String]) -> i32 {
    let Args { mut options, mut paths, watch, random_seed } = match parse_args(args) {
        Ok(a) => a,
        Err(code) => return code,
    };
    if paths.is_empty() {
        // At the root of a project (or in one of its folders): its files with tests.
        match crate::project::targets(crate::project::Cmd::Test) {
            Some(Ok(files)) => paths = files.into_iter().map(PathBuf::from).collect(),
            Some(Err(())) => return 1,
            None => {}
        }
    }
    if paths.is_empty() {
        return usage(which_files());
    }
    if random_seed {
        options.seed = (crate::io::now() * 1e6) as u64 % 1_000_000;
        out_line(&t!("Seed: {}", "Graine : {}", options.seed));
    }
    if watch {
        return watch_paths(&paths, &options);
    }
    match run_paths(&paths, &options) {
        Ok(summary) => exit_code(&summary, &options),
        Err(d) => {
            Driver::new().report(&d);
            1
        }
    }
}

/// The message when `rythme test` has nothing to test.
pub fn which_files() -> String {
    t!(
        "which file or folder? Example: rythme test my_tests.ry, or rythme test . for the files of this folder",
        "quel fichier ou dossier ? Exemple : rythme test mes_tests.ry, ou rythme test . pour les fichiers de ce dossier"
    )
}

/// Reads the words of `rythme test` (a wrong use is reported: exit code 2).
pub fn parse_args(args: &[String]) -> Result<Args, i32> {
    let mut options = Options::default();
    let mut paths = Vec::new();
    let mut watch = false;
    let mut random_seed = false;
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        i += 1;
        let (name, inline) = match arg.split_once('=') {
            Some((n, v)) if n.starts_with("--") => (n, Some(v.to_string())),
            _ => (arg.as_str(), None),
        };
        let mut value = |what: &str| -> Result<String, i32> {
            if let Some(v) = &inline {
                return Ok(v.clone());
            }
            match args.get(i) {
                Some(v) => {
                    i += 1;
                    Ok(v.clone())
                }
                None => Err(usage(t!("`{name}` needs {what}", "`{name}` attend {what}"))),
            }
        };
        match name {
            "--" => {
                options.script_args = args[i..].to_vec();
                break;
            }
            "--filter" | "--filtre" | "-f" => {
                options.filters.push(value(&t!("a part of the name of a test", "une partie du nom d'un test"))?)
            }
            "--seed" | "--graine" => match value(&t!("a number, or `random`", "un nombre, ou `random`")) {
                Ok(v) if v == "random" || v == "hasard" => random_seed = true,
                Ok(v) => match v.parse::<u64>() {
                    Ok(n) => options.seed = n,
                    Err(_) => {
                        return Err(usage(t!(
                            "the seed must be a whole number of 0 or more, or `random`, not `{v}`",
                            "la graine doit être un nombre entier positif ou nul, ou `random`, pas `{v}`"
                        )))
                    }
                },
                Err(code) => return Err(code),
            },
            "--sandbox" => options.sandbox = true,
            "--watch" | "--surveiller" | "-w" => watch = true,
            _ if name.starts_with('-') && name.len() > 1 => {
                let mut msg = t!("unknown option `{name}` for `rythme test`", "option inconnue `{name}` pour `rythme test`");
                if let Some(best) = suggest::closest(name, ["--filter", "--seed", "--watch", "--sandbox", "--format"]) {
                    msg.push_str(&t!("\ndid you mean `{best}`?", "\nvouliez-vous dire `{best}` ?"));
                }
                msg.push_str(&t!(
                    "\nthe options are: --filter NAME, --seed N, --watch, --sandbox, --format json (see `rythme help tests`)",
                    "\nles options sont : --filter NOM, --seed N, --watch, --sandbox, --format json (voir `rythme help tests`)"
                ));
                return Err(usage(msg));
            }
            _ => paths.push(PathBuf::from(arg)),
        }
    }
    Ok(Args { options, paths, watch, random_seed })
}

/// What a file looks like on the disk, to see that it changed.
type Stamp = Vec<(PathBuf, Option<(std::time::SystemTime, u64)>)>;

fn stamp(files: &[PathBuf]) -> Stamp {
    let mut out: Stamp =
        files.iter().map(|f| (f.clone(), std::fs::metadata(f).ok().and_then(|m| Some((m.modified().ok()?, m.len()))))).collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out.dedup_by(|a, b| a.0 == b.0);
    out
}

/// The files to watch: those tested and the files they use, and every `.ry`
/// file of the folders given (a new test file starts a new run).
fn watched(paths: &[PathBuf], summary: Option<&Summary>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = summary.map(|s| s.files.iter().flat_map(|f| f.files.clone()).collect()).unwrap_or_default();
    fn all_ry(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for p in entries.flatten().map(|e| e.path()) {
            let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if p.is_dir() {
                if !name.starts_with('.') && name != "target" && name != "node_modules" {
                    all_ry(&p, out);
                }
            } else if name.ends_with(".ry") {
                out.push(p);
            }
        }
    }
    for p in paths {
        if p.is_dir() {
            all_ry(p, &mut files);
        } else {
            files.push(p.clone());
        }
    }
    files.into_iter().map(|f| f.canonicalize().unwrap_or(f)).collect()
}

/// `--watch`: runs the tests, then again each time a file changes, until
/// the user stops it (Ctrl+C). Checks the files twice a second: no
/// dependency, and it works the same on every system.
fn watch_paths(paths: &[PathBuf], options: &Options) -> i32 {
    let mut first = true;
    loop {
        if !first && std::io::stdout().is_terminal() {
            // Clears the screen: each run starts at the top.
            crate::io::out("\x1b[2J\x1b[H");
        }
        first = false;
        let summary = match run_paths(paths, options) {
            Ok(s) => Some(s),
            Err(d) => {
                Driver::new().report(&d);
                None
            }
        };
        let files = watched(paths, summary.as_ref());
        let before = stamp(&files);
        out_line(&t!(
            "\nWatching {} file(s): the tests run again when one changes (Ctrl+C to stop).",
            "\nSurveillance de {} fichier(s) : les tests sont relancés quand l'un d'eux change (Ctrl+C pour arrêter).",
            before.len()
        ));
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            let now = stamp(&watched(paths, summary.as_ref()));
            if now != before {
                let changed: Vec<String> = now
                    .iter()
                    .filter(|f| !before.contains(f))
                    .chain(before.iter().filter(|f| !now.iter().any(|n| n.0 == f.0)))
                    .map(|f| f.0.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default())
                    .collect();
                out_line(&t!(
                    "\n{} changed: running the tests again\n",
                    "\n{} a changé : nouvelle exécution des tests\n",
                    changed.join(", ")
                ));
                break;
            }
        }
    }
}
