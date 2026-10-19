//! `--format json`: every document printed matches
//! `docs/rythme-output.schema.json` (and the schema describes every field
//! printed), and what tools rely on stays true: stable codes, ranges,
//! fixes that correct the code, exit codes.

use std::path::{Path, PathBuf};
use std::process::Command;

use rythme::lsp::json::Json;

/// A folder of its own for each test, with files in it.
fn folder(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-machine-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (f, text) in files {
        std::fs::write(dir.join(f), text).unwrap();
    }
    dir
}

struct Out {
    code: i32,
    stdout: String,
    stderr: String,
}

fn rythme_in(dir: &Path, lang: &str, args: &[&str]) -> Out {
    let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(args)
        .current_dir(dir)
        .env("RYTHME_LANG", lang)
        .env("NO_COLOR", "1")
        .output()
        .expect("run rythme");
    Out {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// Runs a command that prints one JSON document, checks it against the
/// schema definition `def`, and returns it with the exit code.
#[track_caller]
fn json(dir: &Path, lang: &str, args: &[&str], def: &str) -> (Json, i32) {
    let out = rythme_in(dir, lang, args);
    assert_eq!(out.stdout.lines().count(), 1, "one line of JSON for {args:?}:\n{}\nstderr:\n{}", out.stdout, out.stderr);
    let doc = Json::parse(&out.stdout).unwrap_or_else(|e| panic!("{args:?} printed no JSON ({e}):\n{}", out.stdout));
    let schema = schema();
    let node = schema.get("$defs").get(def);
    assert!(!node.is_null(), "no definition `{def}` in the schema");
    if let Err(e) = validate(&doc, node, &schema, "$") {
        panic!("{args:?}: the document does not match `{def}` of the schema: {e}\n{}", out.stdout);
    }
    assert_eq!(doc.get("version").as_u64(), Some(1));
    assert_eq!(doc.get("exit_code").as_i64().unwrap_or(i64::from(out.code)), i64::from(out.code), "exit_code of {args:?}");
    (doc, out.code)
}

fn schema() -> Json {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/rythme-output.schema.json")).unwrap();
    Json::parse(&text).expect("the schema is JSON")
}

// ----- a small validator for the part of JSON Schema the schema uses --------------

fn resolve<'a>(node: &'a Json, root: &'a Json) -> &'a Json {
    match node.get("$ref").as_str() {
        Some(r) => {
            let name = r.strip_prefix("#/$defs/").unwrap_or_else(|| panic!("unsupported $ref {r}"));
            let target = root.get("$defs").get(name);
            assert!(!target.is_null(), "unknown $ref {r}");
            resolve(target, root)
        }
        None => node,
    }
}

fn type_name(v: &Json) -> &'static str {
    match v {
        Json::Null => "null",
        Json::Bool(_) => "boolean",
        Json::Num(n) if n.fract() == 0.0 => "integer",
        Json::Num(_) => "number",
        Json::Str(_) => "string",
        Json::Arr(_) => "array",
        Json::Obj(_) => "object",
    }
}

/// The properties a node describes, with those of its `allOf`.
fn known_properties<'a>(node: &'a Json, root: &'a Json, out: &mut Vec<&'a str>) {
    let node = resolve(node, root);
    if let Json::Obj(props) = node.get("properties") {
        out.extend(props.iter().map(|(k, _)| k.as_str()));
    }
    for part in node.get("allOf").as_array() {
        known_properties(part, root, out);
    }
}

/// `strict`: every field of an object must be described (not for the parts of an `allOf`,
/// which describe only some of them).
fn validate(v: &Json, node: &Json, root: &Json, at: &str) -> Result<(), String> {
    validate_in(v, node, root, at, true)
}

fn validate_in(v: &Json, node: &Json, root: &Json, at: &str, strict: bool) -> Result<(), String> {
    let node = resolve(node, root);
    for part in node.get("allOf").as_array() {
        validate_in(v, part, root, at, false)?;
    }
    if let [_, ..] = node.get("anyOf").as_array() {
        let errors: Vec<String> = node.get("anyOf").as_array().iter().filter_map(|n| validate(v, n, root, at).err()).collect();
        if errors.len() == node.get("anyOf").as_array().len() {
            return Err(format!("{at}: matches none of anyOf: {}", errors.join(" | ")));
        }
    }
    if let Some(c) = node.get("const").as_f64().map(Json::Num).or_else(|| node.get("const").as_str().map(|s| Json::Str(s.into()))) {
        if *v != c {
            return Err(format!("{at}: expected {c}, got {v}"));
        }
    }
    if let [_, ..] = node.get("enum").as_array() {
        if !node.get("enum").as_array().contains(v) {
            return Err(format!("{at}: {v} is not one of {}", node.get("enum")));
        }
    }
    let types: Vec<&str> = match node.get("type") {
        Json::Str(t) => vec![t.as_str()],
        Json::Arr(ts) => ts.iter().filter_map(Json::as_str).collect(),
        _ => Vec::new(),
    };
    let t = type_name(v);
    if !types.is_empty() && !types.iter().any(|&w| w == t || (w == "number" && t == "integer")) {
        return Err(format!("{at}: expected {types:?}, got {t} {v}"));
    }
    if let (Some(min), Some(n)) = (node.get("minimum").as_f64(), v.as_f64()) {
        if n < min {
            return Err(format!("{at}: {n} < {min}"));
        }
    }
    if let (Some(max), Some(n)) = (node.get("maximum").as_f64(), v.as_f64()) {
        if n > max {
            return Err(format!("{at}: {n} > {max}"));
        }
    }
    if let (Some(p), Some(s)) = (node.get("pattern").as_str(), v.as_str()) {
        assert_eq!(p, "^[EW][0-9]{3}$", "the validator knows only this pattern");
        let ok = s.len() == 4 && s.starts_with(['E', 'W']) && s[1..].chars().all(|c| c.is_ascii_digit());
        if !ok {
            return Err(format!("{at}: `{s}` is not a code"));
        }
    }
    if let Json::Obj(fields) = v {
        for r in node.get("required").as_array() {
            let r = r.as_str().unwrap();
            if !fields.iter().any(|(k, _)| k == r) {
                return Err(format!("{at}: missing field `{r}`"));
            }
        }
        let mut known = Vec::new();
        known_properties(node, root, &mut known);
        for (k, value) in fields {
            // Every field printed is described (only in object nodes that list properties).
            if strict && !known.is_empty() && !known.contains(&k.as_str()) {
                return Err(format!("{at}: the field `{k}` is not in the schema"));
            }
            let prop = node.get("properties").get(k);
            if !prop.is_null() {
                validate(value, prop, root, &format!("{at}.{k}"))?;
            }
        }
    }
    if let Json::Arr(items) = v {
        let schema = node.get("items");
        if !schema.is_null() {
            for (i, item) in items.iter().enumerate() {
                validate(item, schema, root, &format!("{at}[{i}]"))?;
            }
        }
    }
    Ok(())
}

// ----- check -------------------------------------------------------------------------

fn diagnostics(doc: &Json) -> &[Json] {
    doc.get("diagnostics").as_array()
}

#[test]
fn check_gives_codes_places_and_fixes() {
    let dir = folder(
        "check",
        &[
            ("bad.ry", "x = 3\nprint(pritn)\ny = x > 2 ? \"a\" : \"b\"\n"),
            ("typo.ry", "fn total(xs) = sum(xs)\nprint(totl([1, 2]))\n"),
            ("fine.ry", "print(1 + 2)\n"),
            ("warn.ry", "xs = [1]\nxs.push(2)\nprint(xs)\n"),
        ],
    );
    let (doc, code) = json(&dir, "en", &["check", "--format", "json", "typo.ry", "fine.ry", "warn.ry"], "check");
    assert_eq!(code, 1);
    assert_eq!(doc.get("ok"), &Json::Bool(false));
    let files = doc.get("files").as_array();
    assert_eq!(files.len(), 3);
    assert_eq!(files[1].get("ok"), &Json::Bool(true));
    assert_eq!(files[2].get("warnings").as_u64(), Some(1));
    let d = &diagnostics(&doc)[0];
    assert_eq!(d.get("code").as_str(), Some("E003"));
    assert_eq!(d.get("severity").as_str(), Some("error"));
    assert_eq!(d.get("file").as_str(), Some("typo.ry"));
    assert_eq!(d.get_path(&["range", "start", "line"]).as_u64(), Some(2));
    assert_eq!(d.get_path(&["range", "start", "column"]).as_u64(), Some(7));
    assert_eq!(d.get_path(&["range", "end", "column"]).as_u64(), Some(11));
    let fix = &d.get("fixes").as_array()[0];
    assert_eq!(fix.get("safe"), &Json::Bool(false), "a misspelt name is a guess");
    assert_eq!(fix.get("edits").as_array()[0].get("new_text").as_str(), Some("total"));
    let w = &diagnostics(&doc)[1];
    assert_eq!((w.get("code").as_str(), w.get("severity").as_str()), (Some("W008"), Some("warning")));

    // A syntax error: the parser stops at the first one, with its fix.
    let (doc, code) = json(&dir, "en", &["check", "bad.ry", "--format=json"], "check");
    assert_eq!(code, 1);
    let fix = &diagnostics(&doc)[0].get("fixes").as_array()[0];
    assert_eq!(fix.get("safe"), &Json::Bool(true));
    assert_eq!(fix.get("edits").as_array()[0].get("new_text").as_str(), Some("if x > 2 then \"a\" else \"b\""));
    assert_eq!(doc.get_path(&["summary", "fixable"]).as_u64(), Some(1));

    // Without errors: exit code 0, even with warnings.
    let (doc, code) = json(&dir, "en", &["check", "--format", "json", "fine.ry", "warn.ry"], "check");
    assert_eq!((code, doc.get("ok")), (0, &Json::Bool(true)));

    // A missing file is a diagnostic too, without a place.
    let (doc, code) = json(&dir, "en", &["check", "--format", "json", "nothing.ry"], "check");
    assert_eq!(code, 1);
    assert_eq!(diagnostics(&doc)[0].get("code").as_str(), Some("E017"));
    assert!(diagnostics(&doc)[0].get("range").is_null());
}

#[test]
fn messages_follow_the_language_and_codes_do_not() {
    let dir = folder("lang", &[("a.ry", "print(nme)\n")]);
    let (en, _) = json(&dir, "en", &["check", "--format", "json", "a.ry"], "check");
    let (fr, _) = json(&dir, "fr", &["check", "--format", "json", "a.ry"], "check");
    let (zh, _) = json(&dir, "zh", &["--lang", "zh", "check", "--format", "json", "a.ry"], "check");
    assert_eq!(en.get("language").as_str(), Some("en"));
    assert_eq!(fr.get("language").as_str(), Some("fr"));
    assert_eq!(zh.get("language").as_str(), Some("zh"));
    for doc in [&en, &fr, &zh] {
        assert_eq!(diagnostics(doc)[0].get("code").as_str(), Some("E003"));
        assert_eq!(diagnostics(doc)[0].get("severity").as_str(), Some("error"));
    }
    assert_eq!(diagnostics(&en)[0].get("message").as_str(), Some("unknown name `nme`"));
    assert_eq!(diagnostics(&fr)[0].get("message").as_str(), Some("nom inconnu `nme`"));
}

#[test]
fn a_wrong_format_is_a_usage_error() {
    let dir = folder("usage", &[("a.ry", "print(1)\n")]);
    let out = rythme_in(&dir, "en", &["check", "--format", "xml", "a.ry"]);
    assert_eq!(out.code, 2);
    assert!(out.stderr.contains("`--format` takes `json` or `text`"), "{}", out.stderr);
    assert!(out.stdout.is_empty());
    // Without `--format`, nothing changes: the messages for people.
    let out = rythme_in(&dir, "en", &["check", "a.ry"]);
    assert_eq!((out.code, out.stdout.trim()), (0, "✓ a.ry: no problems found"));
    let out = rythme_in(&dir, "en", &["check", "--format", "text", "a.ry"]);
    assert_eq!((out.code, out.stdout.trim()), (0, "✓ a.ry: no problems found"));
}

// ----- run ----------------------------------------------------------------------------

#[test]
fn run_keeps_the_output_and_the_errors_apart() {
    let dir = folder(
        "run",
        &[
            ("ok.ry", "print(\"hello\")\ndebug(2)\n"),
            ("crash.ry", "fn inv(n) = 1 / n\nprint(\"before\")\nprint(inv(0))\n"),
            ("stop.ry", "print(\"bye\")\nexit(3)\n"),
            ("args.ry", "print(args())\n"),
            ("files.ry", "print(read_file(\"ok.ry\"))\nprint(file_exists(\"ok.ry\"))\n"),
            ("broken.ry", "print(1 +)\n"),
        ],
    );
    let (doc, code) = json(&dir, "en", &["run", "--format", "json", "ok.ry"], "run");
    assert_eq!(code, 0);
    assert_eq!(doc.get("stdout").as_str(), Some("hello\n"));
    assert_eq!(doc.get("stderr").as_str(), Some("[debug ok.ry:2:1] 2\n"));
    assert!(diagnostics(&doc).is_empty());

    let (doc, code) = json(&dir, "en", &["run", "--format", "json", "crash.ry"], "run");
    assert_eq!(code, 1);
    assert_eq!(doc.get("stdout").as_str(), Some("before\n"));
    let d = &diagnostics(&doc)[0];
    assert_eq!(d.get("code").as_str(), Some("E008"));
    assert_eq!(d.get("trace").as_array()[0].get("function").as_str(), Some("inv"));

    let (doc, code) = json(&dir, "en", &["run", "--format", "json", "stop.ry"], "run");
    assert_eq!((code, doc.get("stdout").as_str()), (3, Some("bye\n")));

    let (doc, _) = json(&dir, "en", &["run", "--seed", "4", "--format", "json", "args.ry", "a", "--format"], "run");
    assert_eq!(doc.get("stdout").as_str(), Some("[\"a\", \"--format\"]\n"));

    let (doc, code) = json(&dir, "en", &["run", "--format", "json", "broken.ry"], "run");
    assert_eq!((code, diagnostics(&doc)[0].get("code").as_str()), (1, Some("E001")));

    // In the sandbox, a program cannot see the files.
    let (doc, code) = json(&dir, "en", &["run", "--sandbox", "--format", "json", "files.ry"], "run");
    assert_eq!(code, 0);
    let out = doc.get("stdout").as_str().unwrap();
    assert!(out.starts_with("Err(\"`read_file` is not available in the sandbox"), "{out}");
    assert!(out.ends_with("\nfalse\n"), "{out}");
    let out = rythme_in(&dir, "en", &["run", "--sandbox", "files.ry"]);
    assert!(out.stdout.contains("not available in the sandbox"), "{}", out.stdout);
    let out = rythme_in(&dir, "en", &["run", "files.ry"]);
    assert!(out.stdout.starts_with("Ok(\"print"), "{}", out.stdout);
}

// ----- test ---------------------------------------------------------------------------

#[test]
fn test_reports_each_test() {
    let dir = folder(
        "test",
        &[
            ("math_test.ry", "fn add(a, b) = a + b\ntest \"adds\" = assert_eq(add(1, 2), 3)\ntest \"wrong\" = assert_eq(add(1, 2), 4)\n"),
            ("good_test.ry", "test \"fine\" = assert(true)\n"),
            ("broken_test.ry", "test \"x\" = assert_eq(1, \n"),
        ],
    );
    let (doc, code) = json(&dir, "en", &["test", "--format", "json", "math_test.ry"], "test");
    assert_eq!(code, 1);
    assert_eq!(doc.get_path(&["summary", "tests"]).as_u64(), Some(2));
    assert_eq!(doc.get_path(&["summary", "failed"]).as_u64(), Some(1));
    let tests = doc.get("tests").as_array();
    assert_eq!(tests[0].get("passed"), &Json::Bool(true));
    assert!(tests[0].get("failure").is_null());
    assert_eq!(tests[1].get("name").as_str(), Some("wrong"));
    assert_eq!(tests[1].get_path(&["failure", "code"]).as_str(), Some("E015"));
    assert_eq!(tests[1].get_path(&["failure", "notes"]).as_array()[0].as_str(), Some("got:      3\nexpected: 4"));

    let (doc, code) = json(&dir, "en", &["test", "good_test.ry", "--format", "json", "--filter", "fine"], "test");
    assert_eq!((code, doc.get("ok")), (0, &Json::Bool(true)));

    let (doc, code) = json(&dir, "en", &["test", "--format", "json", "."], "test");
    assert_eq!(code, 1);
    assert_eq!(doc.get_path(&["summary", "files"]).as_u64(), Some(3));
    assert_eq!(doc.get_path(&["summary", "broken_files"]).as_u64(), Some(1));
    assert!(diagnostics(&doc).iter().any(|d| d.get("code").as_str() == Some("E001")));

    let out = rythme_in(&dir, "en", &["test", "--format", "json", "--watch", "good_test.ry"]);
    assert_eq!(out.code, 2);
}

// ----- explain and help -------------------------------------------------------------

#[test]
fn explain_and_help_describe_everything() {
    let dir = folder("help", &[]);
    let (doc, code) = json(&dir, "en", &["explain", "E005", "--format", "json"], "explain_one");
    assert_eq!(code, 0);
    assert_eq!(doc.get("title").as_str(), Some("Wrong kind of value"));
    assert!(doc.get("explanation").as_str().unwrap().starts_with("E005 — Wrong kind of value"));
    let (doc, _) = json(&dir, "fr", &["explain", "--format", "json", "w001"], "explain_one");
    assert_eq!(doc.get("code").as_str(), Some("W001"));
    assert_eq!(doc.get("severity").as_str(), Some("warning"));
    let (doc, _) = json(&dir, "en", &["explain", "--format", "json"], "explain_all");
    assert_eq!(doc.get("codes").as_array().len(), rythme::help::CODES.len());
    assert_eq!(rythme_in(&dir, "en", &["explain", "--format", "json", "E999"]).code, 2);

    let (doc, _) = json(&dir, "en", &["help", "--format", "json"], "help_all");
    let names: Vec<&str> = doc.get("builtins").as_array().iter().filter_map(|b| b.get("name").as_str()).collect();
    let all: Vec<&str> = rythme::builtins::BUILTINS.iter().map(|b| b.name).collect();
    assert_eq!(names, all, "every built-in function, in order");
    assert!(doc.get("keywords").as_array().iter().any(|k| k.get("word").as_str() == Some("match")));

    let (doc, _) = json(&dir, "en", &["help", "upcase", "--format", "json"], "help_one");
    assert_eq!(doc.get("kind").as_str(), Some("builtin"));
    assert_eq!(doc.get_path(&["builtin", "name"]).as_str(), Some("upper"));
    assert_eq!(doc.get_path(&["builtin", "aliases"]).as_array()[0].as_str(), Some("upcase"));
    let (doc, _) = json(&dir, "en", &["help", "--format", "json", "print"], "help_one");
    assert!(doc.get_path(&["builtin", "max_args"]).is_null());
    let (doc, _) = json(&dir, "en", &["help", "--format", "json", "fn"], "help_one");
    assert_eq!(doc.get("kind").as_str(), Some("keyword"));
    let (doc, _) = json(&dir, "en", &["help", "--format", "json", "E003"], "help_one");
    assert_eq!(doc.get("kind").as_str(), Some("code"));
    let (doc, _) = json(&dir, "en", &["help", "--format", "json", "lists"], "help_one");
    assert_eq!(doc.get("kind").as_str(), Some("topic"));
    let (doc, code) = json(&dir, "en", &["help", "--format", "json", "lenght"], "help_one");
    assert_eq!((code, doc.get("kind").as_str()), (2, Some("unknown")));
    assert!(doc.get("message").as_str().unwrap().contains("`len`"));
}

#[test]
fn the_schema_knows_every_code_and_language() {
    let schema = schema();
    let langs = schema.get_path(&["$defs", "head", "properties", "language", "enum"]).as_array();
    let codes: Vec<&str> = rythme::lang::LANGUAGES.iter().map(|l| l.code).collect();
    assert_eq!(langs.iter().filter_map(Json::as_str).collect::<Vec<_>>(), codes);
    // Every definition is reachable from the top.
    let tops: Vec<&str> = schema.get("anyOf").as_array().iter().filter_map(|r| r.get("$ref").as_str()).collect();
    for name in ["check", "run", "test", "fix_report", "explain_one", "explain_all", "help_all", "help_one"] {
        assert!(tops.contains(&format!("#/$defs/{name}").as_str()), "{name}");
    }
}
