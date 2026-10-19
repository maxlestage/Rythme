//! Output for programs: `--format json` of `rythme check`, `run`, `test`,
//! `explain` and `help`, read by editors, scripts and AI assistants
//! that fix their own code in a loop.
//!
//! Every document is one JSON object on the standard output, with
//! `"version": 1` (see [`VERSION`]). Within version 1, fields are only ever
//! added, never removed or changed in meaning. The messages are in the
//! language of the messages (`--lang`); the codes, the keys and the values
//! such as `"error"` never change. `docs/rythme-output.schema.json`
//! describes the documents, and `tests/machine.rs` checks them against it.

use std::path::PathBuf;

use crate::builtins::{self, Builtin, BUILTINS};
use crate::diagnostic::{self, Diagnostic, Fix, Severity};
use crate::driver::{find_script, Driver};
use crate::help;
use crate::interp::Mode;
use crate::lsp::json::{obj, Json};
use crate::source::{self, Span};

/// The version of the JSON documents: it changes only if a field is removed
/// or changes meaning, which 1.x never does.
pub const VERSION: u32 = 1;

/// How a command shows what it found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// For people: the messages of Rythme (the default).
    Text,
    /// For programs: one JSON document on the standard output.
    Json,
}

/// Takes `--format json` (or `--format=json`, `--format text`) out of the
/// arguments. A wrong value is a usage error (exit code 2), reported here.
pub fn take_format(args: &mut Vec<String>) -> Result<Format, i32> {
    let mut format = Format::Text;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--" {
            break;
        }
        let (value, used) = match args[i].strip_prefix("--format=") {
            Some(v) => (Some(v.to_string()), 1),
            None if args[i] == "--format" => (args.get(i + 1).cloned(), 2),
            None => {
                i += 1;
                continue;
            }
        };
        format = match value.as_deref() {
            Some("json") => Format::Json,
            Some("text" | "human") => Format::Text,
            other => {
                let v = other.unwrap_or("");
                crate::io::err_line(&t!(
                    "`--format` takes `json` or `text`, not `{v}`: `rythme check --format json file.ry`",
                    "`--format` prend `json` ou `text`, pas `{v}` : `rythme check --format json fichier.ry`"
                ));
                return Err(2);
            }
        };
        args.drain(i..(i + used).min(args.len()));
    }
    Ok(format)
}

/// Takes the option `name` out of the arguments (before `--`): was it there?
pub fn take_flag(args: &mut Vec<String>, name: &str) -> bool {
    let end = args.iter().position(|a| a == "--").unwrap_or(args.len());
    match args[..end].iter().position(|a| a == name) {
        Some(i) => {
            args.remove(i);
            true
        }
        None => false,
    }
}

fn print(doc: Json) {
    crate::io::out_line(&doc.to_string());
}

/// The fields that start every document.
fn head(command: &str) -> Vec<(String, Json)> {
    vec![
        ("version".into(), Json::from(VERSION)),
        ("command".into(), command.into()),
        ("rythme".into(), crate::VERSION.into()),
        ("language".into(), crate::lang::code().into()),
    ]
}

fn with(mut fields: Vec<(String, Json)>, more: Vec<(&str, Json)>) -> Json {
    fields.extend(more.into_iter().map(|(k, v)| (k.to_string(), v)));
    Json::Obj(fields)
}

// ----- diagnostics -------------------------------------------------------------

/// A place in a file: line and column from 1 (columns count characters, as
/// in the messages), and the offset in bytes from 0.
fn position(f: &source::SourceFile, offset: usize) -> Json {
    let (line, column) = f.line_col(offset);
    obj([("line", f.shown(line).1.into()), ("column", column.into()), ("offset", offset.into())])
}

/// `{"file", "range": {"start", "end"}}` of a span (the end is not included).
fn place(span: Span) -> Option<(Json, Json)> {
    let f = source::get(span.file)?;
    let (start, end) = (span.start as usize, span.end as usize);
    let name = f.shown(f.line_col(start).0).0.to_string();
    Some((name.into(), obj([("start", position(&f, start)), ("end", position(&f, end))])))
}

fn fix_json(fix: &Fix) -> Json {
    let edits: Vec<Json> = fix
        .edits
        .iter()
        .filter_map(|(span, new)| {
            let (file, range) = place(*span)?;
            Some(obj([("file", file), ("range", range), ("new_text", new.as_str().into())]))
        })
        .collect();
    obj([("title", fix.title.as_str().into()), ("safe", fix.safe.into()), ("edits", Json::Arr(edits))])
}

/// One diagnostic, as JSON.
pub fn diagnostic_json(d: &Diagnostic) -> Json {
    let (file, range) = d.span.and_then(place).unwrap_or((Json::Null, Json::Null));
    let code = if d.code.is_empty() { Json::Null } else { d.code.into() };
    let labels: Vec<Json> = d
        .secondary
        .iter()
        .filter_map(|(span, label)| {
            let (file, range) = place(*span)?;
            Some(obj([("file", file), ("range", range), ("message", label.as_str().into())]))
        })
        .collect();
    let trace: Vec<Json> = d
        .trace
        .iter()
        .filter_map(|(name, span)| {
            let (file, range) = place(*span)?;
            let function = if name.is_empty() { Json::Null } else { name.as_str().into() };
            Some(obj([("function", function), ("file", file), ("range", range)]))
        })
        .collect();
    let strings = |v: &[String]| Json::Arr(v.iter().map(|s| s.as_str().into()).collect());
    let explain = if d.code.is_empty() { Json::Null } else { format!("rythme explain {}", d.code).into() };
    obj([
        ("code", code),
        ("severity", if d.is_error() { "error" } else { "warning" }.into()),
        ("message", d.message.as_str().into()),
        ("file", file),
        ("range", range),
        ("label", if d.label.is_empty() { Json::Null } else { d.label.as_str().into() }),
        ("labels", Json::Arr(labels)),
        ("notes", strings(&d.notes)),
        ("help", strings(&d.helps)),
        ("fixes", Json::Arr(d.fixes.iter().map(fix_json).collect())),
        ("trace", Json::Arr(trace)),
        ("explain", explain),
    ])
}

fn counts(diags: &[Diagnostic]) -> (usize, usize) {
    let errors = diags.iter().filter(|d| d.severity == Severity::Error).count();
    (errors, diags.len() - errors)
}

fn diagnostics_json(diags: &[Diagnostic]) -> Json {
    Json::Arr(diags.iter().map(diagnostic_json).collect())
}

/// A file named on the command line, or the diagnostic saying it is missing.
fn script(name: &str) -> Result<PathBuf, Box<Diagnostic>> {
    find_script(name)
}

// ----- rythme check --format json -------------------------------------------------

/// `rythme check --format json FILE...`: exit code 0 without errors, 1 with.
pub fn check_command(files: &[String]) -> i32 {
    diagnostic::keep_shown();
    let mut entries = Vec::new();
    let mut all = Vec::new();
    for name in files {
        let mut driver = Driver::new();
        let (shown, ok) = match script(name) {
            Ok(path) => (path.display().to_string(), driver.load_file(&path).is_some()),
            Err(d) => {
                driver.report(&d);
                (name.clone(), false)
            }
        };
        let found = diagnostic::take_kept();
        let (errors, warnings) = counts(&found);
        entries.push(obj([
            ("file", shown.into()),
            ("ok", (ok && errors == 0).into()),
            ("errors", errors.into()),
            ("warnings", warnings.into()),
        ]));
        all.extend(found);
    }
    let (errors, warnings) = counts(&all);
    let ok = entries.iter().all(|e| e.get("ok").as_bool() == Some(true));
    let fixable = all.iter().filter(|d| d.fixes.iter().any(|f| f.safe)).count();
    let code = i32::from(!ok);
    print(with(
        head("check"),
        vec![
            ("ok", ok.into()),
            ("exit_code", i64::from(code).into()),
            ("files", Json::Arr(entries)),
            (
                "summary",
                obj([("files", files.len().into()), ("errors", errors.into()), ("warnings", warnings.into()), ("fixable", fixable.into())]),
            ),
            ("diagnostics", diagnostics_json(&all)),
        ],
    ));
    code
}

// ----- rythme run --format json ---------------------------------------------------

/// What a program wrote: its standard output, and its error output (the
/// parts of the captured text between `ERR_START` and `ERR_END`).
fn split_output(captured: &str) -> (String, String) {
    let (mut out, mut err) = (String::new(), String::new());
    let mut in_err = false;
    for c in captured.chars() {
        match c {
            crate::io::ERR_START => in_err = true,
            crate::io::ERR_END => in_err = false,
            c if in_err => err.push(c),
            c => out.push(c),
        }
    }
    (out, err)
}

fn run_report(file: &str, code: i32, start: f64) -> Json {
    let (stdout, stderr) = split_output(&crate::io::take_captured());
    let diags = diagnostic::take_kept();
    let (errors, warnings) = counts(&diags);
    with(
        head("run"),
        vec![
            ("file", file.into()),
            ("ok", (code == 0).into()),
            ("exit_code", i64::from(code).into()),
            ("stdout", stdout.into()),
            ("stderr", stderr.into()),
            ("seconds", Json::Num(((crate::io::now() - start) * 1000.0).round() / 1000.0)),
            ("summary", obj([("errors", errors.into()), ("warnings", warnings.into())])),
            ("diagnostics", diagnostics_json(&diags)),
        ],
    )
}

/// `rythme run --format json FILE [ARGS...]`: runs the program with its
/// output kept, then prints one document with the output and the
/// diagnostics. The exit code is the program's.
pub fn run_command(name: &str, script_args: Vec<String>, seed: Option<i64>, sandbox: bool) -> i32 {
    diagnostic::keep_shown();
    crate::io::capture();
    let start = crate::io::now();
    let shown = name.to_string();
    let hook_name = shown.clone();
    crate::io::on_exit(Box::new(move |code| print_captured(run_report(&hook_name, code, start))));
    let code = run_inner(name, script_args, seed, sandbox);
    crate::io::clear_on_exit();
    print_captured(run_report(&shown, code, start));
    code
}

/// Prints a document after the output was captured: capturing stops first.
fn print_captured(doc: Json) {
    let _ = crate::io::take_captured();
    print(doc);
}

fn run_inner(name: &str, script_args: Vec<String>, seed: Option<i64>, sandbox: bool) -> i32 {
    let mut driver = Driver::new();
    if let Some(seed) = seed {
        driver.interp.seed_random(seed);
    }
    driver.interp.sandboxed = sandbox;
    let path = match script(name) {
        Ok(p) => p,
        Err(d) => {
            driver.report(&d);
            return 1;
        }
    };
    driver.interp.script_args = script_args;
    let Some(module) = driver.load_file(&path) else { return 1 };
    i32::from(driver.execute(&module, Mode::Run).is_err())
}

// ----- rythme test --format json --------------------------------------------------

/// The document of `rythme test --format json`, from the summary of the run
/// and the diagnostics kept while it ran.
pub fn test_report(summary: &crate::testing::Summary, code: i32, seed: u64, kept: &[Diagnostic]) -> Json {
    let mut tests = Vec::new();
    let mut files = Vec::new();
    for f in &summary.files {
        let failed = f.results.iter().filter(|r| !r.passed()).count();
        files.push(obj([
            ("file", f.path.display().to_string().into()),
            ("ok", (!f.broken && failed == 0).into()),
            ("broken", f.broken.into()),
            ("tests", f.results.len().into()),
            ("failed", failed.into()),
            ("filtered_out", f.filtered_out.into()),
        ]));
        for r in &f.results {
            let (file, range) = place(r.span).unwrap_or((Json::Null, Json::Null));
            tests.push(obj([
                ("name", r.name.as_str().into()),
                ("file", file),
                ("range", range),
                ("passed", r.passed().into()),
                ("seconds", Json::Num((r.seconds * 1000.0).round() / 1000.0)),
                ("failure", r.failure.as_ref().map_or(Json::Null, diagnostic_json)),
            ]));
        }
    }
    let (errors, warnings) = counts(kept);
    let (total, failed) = (summary.total(), summary.failed());
    with(
        head("test"),
        vec![
            ("ok", (code == 0).into()),
            ("exit_code", i64::from(code).into()),
            ("seed", Json::Num(seed as f64)),
            (
                "summary",
                obj([
                    ("files", summary.files.len().into()),
                    ("tests", total.into()),
                    ("passed", (total - failed).into()),
                    ("failed", failed.into()),
                    ("filtered_out", summary.filtered_out().into()),
                    ("broken_files", summary.broken().into()),
                    ("errors", errors.into()),
                    ("warnings", warnings.into()),
                    ("seconds", Json::Num((summary.seconds * 1000.0).round() / 1000.0)),
                ]),
            ),
            ("files", Json::Arr(files)),
            ("tests", Json::Arr(tests)),
            ("diagnostics", diagnostics_json(kept)),
        ],
    )
}

/// Prints the document of a test run (the human lines were captured and are dropped).
fn print_test_report(summary: &crate::testing::Summary, code: i32, seed: u64) -> i32 {
    let kept = diagnostic::take_kept();
    print_captured(test_report(summary, code, seed, &kept));
    code
}

/// `rythme test --format json [PATH...] [--filter NAME] [--seed N] [--sandbox]`:
/// the tests run as with `rythme test`, then one document tells what each did.
pub fn test_command(args: &[String]) -> i32 {
    use crate::testing::{self, Summary};
    let testing::Args { mut options, mut paths, watch, random_seed } = match testing::parse_args(args) {
        Ok(a) => a,
        Err(code) => return code,
    };
    if watch {
        crate::io::err_line(&t!(
            "`--watch` does not go with `--format json`: run `rythme test --format json` again after each change",
            "`--watch` ne va pas avec `--format json` : relancez `rythme test --format json` après chaque changement"
        ));
        return 2;
    }
    diagnostic::keep_shown();
    crate::io::capture();
    if paths.is_empty() {
        match crate::project::targets(crate::project::Cmd::Test) {
            Some(Ok(files)) => paths = files.into_iter().map(PathBuf::from).collect(),
            Some(Err(())) => return print_test_report(&Summary::default(), 1, options.seed),
            None => {}
        }
    }
    if paths.is_empty() {
        let _ = crate::io::take_captured();
        crate::io::err_line(&testing::which_files());
        return 2;
    }
    if random_seed {
        options.seed = (crate::io::now() * 1e6) as u64 % 1_000_000;
    }
    match testing::run_paths(&paths, &options) {
        Ok(summary) => {
            let code = testing::exit_code(&summary, &options);
            print_test_report(&summary, code, options.seed)
        }
        Err(d) => {
            Driver::new().report(&d);
            print_test_report(&Summary::default(), 1, options.seed)
        }
    }
}

// ----- rythme explain / help --format json ------------------------------------------

fn code_json(code: &str) -> Json {
    let severity = if code.starts_with('W') { "warning" } else { "error" };
    obj([
        ("code", code.into()),
        ("severity", severity.into()),
        ("title", help::code_title(code).into()),
        ("explanation", help::explain(code).into()),
    ])
}

/// `rythme explain [CODE] --format json`.
pub fn explain_command(code: Option<&str>) -> i32 {
    match code {
        None => {
            let codes: Vec<Json> = help::CODES.iter().map(|c| code_json(c)).collect();
            print(with(head("explain"), vec![("codes", Json::Arr(codes))]));
            0
        }
        Some(code) => match help::explain(code) {
            Some(_) => {
                let c = code.trim().to_uppercase();
                let Json::Obj(fields) = code_json(&c) else { return 1 };
                let mut doc = head("explain");
                doc.extend(fields);
                print(Json::Obj(doc));
                0
            }
            None => {
                crate::io::err_line(&t!(
                    "unknown code `{code}`. Known codes: {}",
                    "code inconnu `{code}`. Codes connus : {}",
                    help::CODES.join(" ")
                ));
                2
            }
        },
    }
}

/// A built-in function, as JSON.
pub fn builtin_json(b: &Builtin) -> Json {
    let max = if b.max == usize::MAX { Json::Null } else { b.max.into() };
    obj([
        ("name", b.name.into()),
        ("signature", b.sig.into()),
        ("category", format!("{:?}", b.cat).to_lowercase().into()),
        ("category_title", b.cat.title().into()),
        ("doc", b.doc().into()),
        ("example", b.example.into()),
        ("aliases", Json::Arr(builtins::aliases(b.name).map(Json::from).collect())),
        ("min_args", b.min.into()),
        ("max_args", max),
    ])
}

fn keyword_json(word: &str) -> Option<Json> {
    let (doc, example, topic) = help::keyword_doc(word)?;
    Some(obj([("word", word.into()), ("doc", doc.into()), ("example", example.into()), ("topic", topic.into())]))
}

/// `rythme help [NAME] --format json`: everything, or one built-in function,
/// keyword, topic or code. An unknown name gives `"kind": "unknown"` and exit code 2.
pub fn help_command(name: Option<&str>) -> i32 {
    let Some(name) = name else {
        let topics: Vec<Json> = help::TOPICS
            .iter()
            .map(|(n, aliases)| obj([("name", (*n).into()), ("aliases", Json::Arr(aliases.iter().map(|a| (*a).into()).collect()))]))
            .collect();
        let aliases: Vec<Json> = builtins::ALIASES.iter().map(|a| obj([("name", a.name.into()), ("of", a.of.into())])).collect();
        print(with(
            head("help"),
            vec![
                ("builtins", Json::Arr(BUILTINS.iter().map(builtin_json).collect())),
                ("aliases", Json::Arr(aliases)),
                ("keywords", Json::Arr(help::keywords().filter_map(keyword_json).collect())),
                ("topics", Json::Arr(topics)),
                ("codes", Json::Arr(help::CODES.iter().map(|c| code_json(c)).collect())),
            ],
        ));
        return 0;
    };
    let one = |kind: &str, value: Json| {
        let mut doc = head("help");
        doc.push(("kind".into(), kind.into()));
        doc.push((kind.into(), value));
        print(Json::Obj(doc));
        0
    };
    if let Some(b) = builtins::lookup(name) {
        return one("builtin", builtin_json(b));
    }
    if let Some(k) = keyword_json(name) {
        return one("keyword", k);
    }
    if help::explain(name).is_some() {
        return one("code", code_json(&name.trim().to_uppercase()));
    }
    if let Some(text) = help::topic(name) {
        return one("topic", obj([("name", name.into()), ("text", text.trim_end().into())]));
    }
    let mut doc = head("help");
    doc.push(("kind".into(), "unknown".into()));
    doc.push(("name".into(), name.into()));
    doc.push(("message".into(), help::unknown_topic(name).trim_end().into()));
    print(Json::Obj(doc));
    2
}
