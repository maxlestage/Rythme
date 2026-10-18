//! `rythme lsp`: the language server, spoken to like an editor does.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use rythme::lsp::json::Json;

struct Lsp {
    child: Child,
    stdin: Option<ChildStdin>,
    rx: Receiver<Option<Json>>,
    next_id: i64,
    /// Notifications received while waiting for an answer.
    notes: Vec<Json>,
}

fn s(text: &str) -> String {
    Json::Str(text.to_string()).to_string()
}

fn uri(path: &Path) -> String {
    rythme::lsp::text::path_to_uri(path)
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-lsp-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.canonicalize().unwrap()
}

impl Lsp {
    fn spawn(lang: &str) -> Lsp {
        Lsp::spawn_with(lang, &["lsp"])
    }

    fn spawn_with(lang: &str, args: &[&str]) -> Lsp {
        let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .args(args)
            .env("RYTHME_LANG", lang)
            .env("NO_COLOR", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("start rythme lsp");
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let mut r = BufReader::new(stdout);
            loop {
                let mut len = None;
                loop {
                    let mut line = String::new();
                    if r.read_line(&mut line).unwrap_or(0) == 0 {
                        let _ = tx.send(None);
                        return;
                    }
                    let line = line.trim_end();
                    if line.is_empty() {
                        break;
                    }
                    if let Some(v) = line.strip_prefix("Content-Length:") {
                        len = v.trim().parse::<usize>().ok();
                    }
                }
                let mut body = vec![0; len.expect("Content-Length")];
                r.read_exact(&mut body).unwrap();
                let msg = Json::parse(std::str::from_utf8(&body).unwrap()).expect("the server sends valid JSON");
                if tx.send(Some(msg)).is_err() {
                    return;
                }
            }
        });
        Lsp { child, stdin: Some(stdin), rx, next_id: 0, notes: Vec::new() }
    }

    /// Starts a server and initializes it with these parameters.
    fn start(lang: &str, init: &str) -> (Lsp, Json) {
        let mut lsp = Lsp::spawn(lang);
        let r = lsp.request("initialize", init);
        lsp.notify("initialized", "{}");
        (lsp, r)
    }

    fn raw(&mut self, bytes: &[u8]) {
        let stdin = self.stdin.as_mut().expect("the input is open");
        stdin.write_all(bytes).unwrap();
        stdin.flush().unwrap();
    }

    fn send(&mut self, body: &str) {
        self.raw(format!("Content-Length: {}\r\n\r\n{body}", body.len()).as_bytes());
    }

    fn recv(&mut self) -> Json {
        match self.rx.recv_timeout(Duration::from_secs(20)) {
            Ok(Some(m)) => m,
            Ok(None) => panic!("the server stopped"),
            Err(_) => panic!("no answer from the server"),
        }
    }

    /// Sends a request and returns the whole answer (`result` or `error`).
    fn request(&mut self, method: &str, params: &str) -> Json {
        self.next_id += 1;
        let id = self.next_id;
        self.send(&format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{params}}}"#));
        loop {
            let m = self.recv();
            if m.get("id").as_i64() == Some(id) && m.get("method").is_null() {
                return m;
            }
            self.notes.push(m);
        }
    }

    fn result(&mut self, method: &str, params: &str) -> Json {
        let r = self.request(method, params);
        assert!(r.get("error").is_null(), "{method} failed: {r}");
        r.get("result").clone()
    }

    fn notify(&mut self, method: &str, params: &str) {
        self.send(&format!(r#"{{"jsonrpc":"2.0","method":"{method}","params":{params}}}"#));
    }

    /// The latest diagnostics published for a document. The server handles
    /// messages in order: what it publishes after a change comes before the
    /// answer to the next request.
    fn diagnostics(&mut self, uri: &str) -> Vec<Json> {
        let r = self.request("rythme/sync", "null");
        assert_eq!(r.get_path(&["error", "code"]).as_i64(), Some(-32601));
        let last = self.notes.iter().rposition(|n| is_diag_for(n, uri)).unwrap_or_else(|| panic!("no diagnostics for {uri}"));
        let found = self.notes[last].get_path(&["params", "diagnostics"]).as_array().to_vec();
        self.notes.retain(|n| !is_diag_for(n, uri));
        found
    }

    fn open(&mut self, uri: &str, text: &str) -> Vec<Json> {
        self.notify(
            "textDocument/didOpen",
            &format!(r#"{{"textDocument":{{"uri":{},"languageId":"rythme","version":1,"text":{}}}}}"#, s(uri), s(text)),
        );
        self.diagnostics(uri)
    }

    fn change(&mut self, uri: &str, version: i64, text: &str) -> Vec<Json> {
        self.notify(
            "textDocument/didChange",
            &format!(r#"{{"textDocument":{{"uri":{},"version":{version}}},"contentChanges":[{{"text":{}}}]}}"#, s(uri), s(text)),
        );
        self.diagnostics(uri)
    }

    fn at(&mut self, method: &str, uri: &str, line: u32, character: u32) -> Json {
        self.result(method, &at(uri, line, character))
    }

    /// `shutdown` then `exit`: returns the exit code.
    fn stop(mut self) -> i32 {
        assert!(self.result("shutdown", "null").is_null());
        self.notify("exit", "null");
        self.wait()
    }

    fn wait(mut self) -> i32 {
        for _ in 0..200 {
            if let Some(status) = self.child.try_wait().unwrap() {
                return status.code().unwrap_or(-1);
            }
            std::thread::sleep(Duration::from_millis(25));
        }
        let _ = self.child.kill();
        panic!("the server did not stop");
    }
}

fn is_diag_for(m: &Json, uri: &str) -> bool {
    m.get("method").as_str() == Some("textDocument/publishDiagnostics") && m.get_path(&["params", "uri"]).as_str() == Some(uri)
}

fn at(uri: &str, line: u32, character: u32) -> String {
    format!(r#"{{"textDocument":{{"uri":{}}},"position":{{"line":{line},"character":{character}}}}}"#, s(uri))
}

fn range(d: &Json) -> (u64, u64, u64, u64) {
    let r = d.get("range");
    let n = |p: &[&str]| r.get_path(p).as_u64().unwrap();
    (n(&["start", "line"]), n(&["start", "character"]), n(&["end", "line"]), n(&["end", "character"]))
}

fn hover_text(h: &Json) -> String {
    h.get_path(&["contents", "value"]).as_str().unwrap_or("").to_string()
}

fn labels(items: &Json) -> Vec<String> {
    items.as_array().iter().map(|i| i.get("label").as_str().unwrap().to_string()).collect()
}

const INIT: &str = r#"{"processId":null,"rootUri":null,"capabilities":{}}"#;

#[test]
fn initialize_announces_the_features() {
    let (lsp, r) = Lsp::start("en", INIT);
    let caps = r.get_path(&["result", "capabilities"]);
    assert_eq!(caps.get("positionEncoding").as_str(), Some("utf-16"));
    for feature in [
        "hoverProvider",
        "completionProvider",
        "signatureHelpProvider",
        "definitionProvider",
        "referencesProvider",
        "renameProvider",
        "documentSymbolProvider",
        "documentFormattingProvider",
        "inlayHintProvider",
        "semanticTokensProvider",
        "codeLensProvider",
        "workspaceSymbolProvider",
        "foldingRangeProvider",
        "selectionRangeProvider",
    ] {
        assert!(!caps.get(feature).is_null(), "{feature}");
    }
    assert_eq!(caps.get_path(&["textDocumentSync", "change"]).as_u64(), Some(1));
    assert_eq!(r.get_path(&["result", "serverInfo", "name"]).as_str(), Some("rythme"));
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn diagnostics_have_utf16_ranges_on_accented_text() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/accents.ry";
    // `é` is one UTF-16 unit (two bytes), `🎉` two units (four bytes).
    let diags = lsp.open(u, "x = \"é🎉\" + inconnu\nprint(x)\n");
    assert_eq!(diags.len(), 1, "{diags:?}");
    let d = &diags[0];
    assert_eq!(range(d), (0, 12, 0, 19));
    assert_eq!(d.get("code").as_str(), Some("E003"));
    assert_eq!(d.get("severity").as_u64(), Some(1));
    assert_eq!(d.get("source").as_str(), Some("rythme"));
    let msg = d.get("message").as_str().unwrap();
    assert!(msg.contains("unknown name `inconnu`") && msg.contains("rythme explain E003"), "{msg}");
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn utf8_positions_when_the_editor_prefers_them() {
    let init = r#"{"processId":null,"capabilities":{"general":{"positionEncodings":["utf-8","utf-16"]}}}"#;
    let (mut lsp, r) = Lsp::start("en", init);
    assert_eq!(r.get_path(&["result", "capabilities", "positionEncoding"]).as_str(), Some("utf-8"));
    let u = "file:///nowhere/bytes.ry";
    let diags = lsp.open(u, "x = \"é🎉\" + inconnu\nprint(x)\n");
    assert_eq!(range(&diags[0]), (0, 15, 0, 22));
    // Hover positions are bytes too: `inconnu` starts at byte 15.
    let h = lsp.at("textDocument/hover", u, 0, 16);
    assert!(hover_text(&h).contains("E003"), "{h}");
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn every_problem_at_once_with_help_and_related_places() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/many.ry";
    let diags = lsp.open(u, "fn f(x) = x\nfn g()\n  unused = 1\n  2\nend\nf = 2\nprint(pirnt(1), f(1, 2), g())\n");
    let codes: Vec<&str> = diags.iter().filter_map(|d| d.get("code").as_str()).collect();
    for code in ["E011", "E003", "E004", "W001"] {
        assert!(codes.contains(&code), "{code} missing in {codes:?}");
    }
    let dup = diags.iter().find(|d| d.get("code").as_str() == Some("E011")).unwrap();
    let related = dup.get("relatedInformation").as_array();
    assert_eq!(related.len(), 1);
    assert_eq!(range(&related[0].get("location").clone()), (0, 3, 0, 4));
    let typo = diags.iter().find(|d| d.get("code").as_str() == Some("E003")).unwrap();
    assert!(typo.get("message").as_str().unwrap().contains("did you mean `print`?"));
    let unused = diags.iter().find(|d| d.get("code").as_str() == Some("W001")).unwrap();
    assert_eq!(unused.get("severity").as_u64(), Some(2));
    // A quick fix for the typo.
    let params = format!(
        r#"{{"textDocument":{{"uri":{}}},"range":{{"start":{{"line":6,"character":6}},"end":{{"line":6,"character":6}}}},"context":{{"diagnostics":[]}}}}"#,
        s(u)
    );
    let actions = lsp.result("textDocument/codeAction", &params);
    let fix = &actions.as_array()[0];
    assert_eq!(fix.get("title").as_str(), Some("Replace with `print`"));
    let edit = &fix.get_path(&["edit", "changes", u]).as_array()[0];
    assert_eq!(edit.get("newText").as_str(), Some("print"));
    assert_eq!(range(edit), (6, 6, 6, 11));
    assert_eq!(unused.get("tags").as_array().len(), 1);
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn messages_follow_the_editor_language() {
    // The language of the editor...
    let (mut lsp, _) = Lsp::start("", r#"{"processId":null,"locale":"fr-CA","capabilities":{}}"#);
    let diags = lsp.open("file:///nowhere/fr.ry", "print(inconnu)\n");
    assert!(diags[0].get("message").as_str().unwrap().starts_with("nom inconnu"), "{:?}", diags[0]);
    let h = lsp.at("textDocument/hover", "file:///nowhere/fr.ry", 0, 2);
    assert!(hover_text(&h).contains("Affiche des valeurs"), "{h}");
    assert_eq!(lsp.stop(), 0);
    // ...unless the editor's setting says otherwise.
    let (mut lsp, _) = Lsp::start("fr", r#"{"processId":null,"locale":"fr","initializationOptions":{"language":"en"},"capabilities":{}}"#);
    let diags = lsp.open("file:///nowhere/en.ry", "print(inconnu)\n");
    assert!(diags[0].get("message").as_str().unwrap().starts_with("unknown name"), "{:?}", diags[0]);
    lsp.notify("workspace/didChangeConfiguration", r#"{"settings":{"rythme":{"language":"fr"}}}"#);
    let diags = lsp.diagnostics("file:///nowhere/en.ry");
    assert!(diags[0].get("message").as_str().unwrap().starts_with("nom inconnu"), "{:?}", diags[0]);
    assert_eq!(lsp.stop(), 0);
    // `--lang` on the command line wins over the language of the editor.
    let mut lsp = Lsp::spawn_with("", &["--lang", "fr", "lsp", "--stdio"]);
    lsp.result("initialize", r#"{"processId":null,"locale":"en-US","capabilities":{}}"#);
    let diags = lsp.open("file:///nowhere/cli.ry", "print(inconnu)\n");
    assert!(diags[0].get("message").as_str().unwrap().starts_with("nom inconnu"), "{:?}", diags[0]);
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn hover_shows_documentation() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/hover.ry";
    let text = "## Doubles a number.\n## Works on Int and Float.\nfn double(x) = x * 2\n\ntype Shape = Circle(radius) | Square(side)\n\nfn fib(0) = 0\nfn fib(n) = fib(n - 1)\n\nprint(map([1, 2], double), Circle(1), fib(3))\nprint(if true then Some(1) else None)\n";
    assert!(lsp.open(u, text).is_empty());
    // A built-in function: signature, documentation, example.
    let h = hover_text(&lsp.at("textDocument/hover", u, 9, 7));
    assert!(h.contains("map(list, f)") && h.contains("Example") && h.contains("rythme help map"), "{h}");
    // A function of the file, with its `##` documentation.
    let h = hover_text(&lsp.at("textDocument/hover", u, 9, 20));
    assert!(h.contains("fn double(x)") && h.contains("Doubles a number.\nWorks on Int and Float."), "{h}");
    // With what the checker infers from its code.
    assert!(h.contains("Inferred from its code:") && h.contains("fn double(x: Int|Float) -> Int|Float"), "{h}");
    // Every definition of a function.
    let h = hover_text(&lsp.at("textDocument/hover", u, 9, 40));
    assert!(h.contains("fn fib(0)\nfn fib(n)"), "{h}");
    // A constructor and its type.
    let h = hover_text(&lsp.at("textDocument/hover", u, 9, 29));
    assert!(h.contains("Circle(radius)") && h.contains("constructor of the type `Shape`"), "{h}");
    // A keyword.
    let h = hover_text(&lsp.at("textDocument/hover", u, 10, 15));
    assert!(h.contains("**then**") && h.contains("keyword"), "{h}");
    // `Some`, `None`.
    let h = hover_text(&lsp.at("textDocument/hover", u, 10, 34));
    assert!(h.contains("`Option`"), "{h}");
    // A parameter.
    let h = hover_text(&lsp.at("textDocument/hover", u, 2, 15));
    assert!(h.contains("parameter of `double`"), "{h}");
    // Nothing on a number or in a comment.
    assert!(lsp.at("textDocument/hover", u, 7, 21).is_null());
    assert!(lsp.at("textDocument/hover", u, 0, 5).is_null());
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn hover_on_a_problem_explains_its_code() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/explain.ry";
    lsp.open(u, "x = 1\nx + 1\nprint(x)\n");
    let h = hover_text(&lsp.at("textDocument/hover", u, 1, 2));
    assert!(h.contains("E024"), "{h}");
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn completion_offers_names_in_scope_keywords_and_snippets() {
    let init = r#"{"processId":null,"capabilities":{"textDocument":{"completion":{"completionItem":{"snippetSupport":true}}}}}"#;
    let (mut lsp, _) = Lsp::start("en", init);
    let u = "file:///nowhere/complete.ry";
    let text = "type Color = Red | Green\nfn area(width, height)\n  surface = width * height\n  \nend\nfn other(z) = z\n";
    lsp.open(u, text);
    let items = lsp.at("textDocument/completion", u, 3, 2);
    let l = labels(&items);
    for name in ["surface", "width", "height", "area", "other", "Red", "map", "print", "match", "fn … end", "match … end", "test \"…\" = …"]
    {
        assert!(l.iter().any(|x| x == name), "{name} missing in {l:?}");
    }
    assert!(!l.iter().any(|x| x == "z"), "`z` is not visible here");
    let map = items.as_array().iter().find(|i| i.get("label").as_str() == Some("map")).unwrap();
    assert_eq!(map.get("detail").as_str(), Some("map(list, f)"));
    assert!(map.get_path(&["documentation", "value"]).as_str().unwrap().contains("Example"));
    let snippet = items.as_array().iter().find(|i| i.get("label").as_str() == Some("fn … end")).unwrap();
    assert_eq!(snippet.get("insertTextFormat").as_u64(), Some(2));
    assert!(snippet.get("insertText").as_str().unwrap().ends_with("\nend"));
    // Local names come first.
    let first = items.as_array().iter().min_by_key(|i| i.get("sortText").as_str().unwrap().to_string()).unwrap();
    assert!(["height", "surface", "width"].contains(&first.get("label").as_str().unwrap()), "{first}");
    // Nothing in a comment.
    lsp.change(u, 2, "# pri");
    assert!(lsp.at("textDocument/completion", u, 0, 5).as_array().is_empty());
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn dicts_and_sets_in_the_editor() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/dicts.ry";
    let diags = lsp.open(u, "ages = dict([[\"Ada\", 36]])\nprint(union(set([1]), ages), has(ages, \"Ada\"))\n");
    assert_eq!(diags.len(), 1, "{diags:?}");
    let message = diags[0].get("message").as_str().unwrap();
    assert!(message.starts_with("`union` expects a set as argument 2, but got"), "{message}");
    let h = hover_text(&lsp.at("textDocument/hover", u, 1, 8));
    assert!(h.contains("union(a, b)") && h.contains("rythme help union"), "{h}");
    let l = labels(&lsp.at("textDocument/completion", u, 1, 0));
    for name in ["dict", "set", "union", "intersection", "difference", "is_subset", "ages"] {
        assert!(l.iter().any(|x| x == name), "{name} missing in {l:?}");
    }
    assert!(lsp.change(u, 2, "d = dict([[1, \"a\"]])\nprint(d[1], filter(d, fn(k, v) = k > 0))\n").is_empty());
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn completion_works_on_unfinished_code() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/typing.ry";
    let diags = lsp.open(u, "fn greet(name)\n  message = \"Hello {name}\"\n  print(mes\nend\n");
    assert_eq!(diags.len(), 1, "a syntax error: {diags:?}");
    let l = labels(&lsp.at("textDocument/completion", u, 2, 11));
    assert!(l.contains(&"message".to_string()) && l.contains(&"name".to_string()), "{l:?}");
    // Signature help while typing the arguments.
    let sig = lsp.at("textDocument/signatureHelp", u, 2, 11);
    assert_eq!(sig.get_path(&["signatures"]).as_array()[0].get("label").as_str(), Some("print(value, ...)"));
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn signature_help_counts_the_arguments() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/sig.ry";
    lsp.open(u, "## Adds.\nfn add(a, b) = a + b\nx = add(1, \ny = [1, 2] |> map(\n");
    let sig = lsp.at("textDocument/signatureHelp", u, 2, 11);
    let first = &sig.get("signatures").as_array()[0];
    assert_eq!(first.get("label").as_str(), Some("add(a, b)"));
    assert_eq!(sig.get("activeParameter").as_u64(), Some(1));
    let params: Vec<(u64, u64)> = first
        .get("parameters")
        .as_array()
        .iter()
        .map(|p| (p.get("label").as_array()[0].as_u64().unwrap(), p.get("label").as_array()[1].as_u64().unwrap()))
        .collect();
    assert_eq!(params, [(4, 5), (7, 8)]);
    // After `|>`, the list is already the first argument.
    let sig = lsp.at("textDocument/signatureHelp", u, 3, 18);
    assert_eq!(sig.get("signatures").as_array()[0].get("label").as_str(), Some("map(list, f)"));
    assert_eq!(sig.get("activeParameter").as_u64(), Some(1));
    // A built-in with several forms: one signature each.
    lsp.change(u, 2, "z = random(1, \n");
    let sig = lsp.at("textDocument/signatureHelp", u, 0, 14);
    let labels: Vec<&str> = sig.get("signatures").as_array().iter().filter_map(|s| s.get("label").as_str()).collect();
    assert_eq!(labels, ["random(a, b)", "random(list)"]);
    assert_eq!(sig.get("activeSignature").as_u64(), Some(0));
    assert_eq!(sig.get("activeParameter").as_u64(), Some(1));
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn signature_help_for_calls_without_parentheses() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/command.ry";
    lsp.open(
        u,
        "fn add(a, b) = a + b
add 1, 
assert_eq 
x = 1
",
    );
    let sig = lsp.at("textDocument/signatureHelp", u, 1, 7);
    assert_eq!(sig.get("signatures").as_array()[0].get("label").as_str(), Some("add(a, b)"));
    assert_eq!(sig.get("activeParameter").as_u64(), Some(1));
    // Right after the space that follows the name.
    let sig = lsp.at("textDocument/signatureHelp", u, 2, 10);
    assert_eq!(sig.get("signatures").as_array()[0].get("label").as_str(), Some("assert_eq(actual, expected)"));
    assert_eq!(sig.get("activeParameter").as_u64(), Some(0));
    // A definition is not a call.
    assert!(lsp.at("textDocument/signatureHelp", u, 3, 4).is_null());
    // In a result: a branch, a case, the body of a function, a block.
    lsp.change(u, 2, "fn add(a, b) = a + b\ny = if x > 0 then add 1, 2 else add 3, \nfn f(v) = assert_eq \n");
    let sig = lsp.at("textDocument/signatureHelp", u, 1, 39);
    assert_eq!(sig.get("signatures").as_array()[0].get("label").as_str(), Some("add(a, b)"));
    assert_eq!(sig.get("activeParameter").as_u64(), Some(1));
    let sig = lsp.at("textDocument/signatureHelp", u, 2, 20);
    assert_eq!(sig.get("signatures").as_array()[0].get("label").as_str(), Some("assert_eq(actual, expected)"));
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn definition_symbols_and_highlights() {
    let init = r#"{"processId":null,"capabilities":{"textDocument":{"documentSymbol":{"hierarchicalDocumentSymbolSupport":true}}}}"#;
    let (mut lsp, _) = Lsp::start("en", init);
    let u = "file:///nowhere/def.ry";
    let text = "type Shape = Circle(r) | Dot\nfn area(Circle(r)) = r * r\nfn area(Dot) = 0\nfn twice(x)\n  y = x * 2\n  y + y\nend\nprint(area(Dot), twice(1))\ntest \"area\" = assert_eq(area(Dot), 0)\n";
    assert!(lsp.open(u, text).is_empty());
    // A function, from a call.
    let d = lsp.at("textDocument/definition", u, 7, 7);
    assert_eq!(d.get("uri").as_str(), Some(u));
    assert_eq!(range(&d), (1, 3, 1, 7));
    // A constructor, from a pattern.
    let d = lsp.at("textDocument/definition", u, 2, 9);
    assert_eq!(range(&d), (0, 25, 0, 28));
    // A local name.
    let d = lsp.at("textDocument/definition", u, 5, 6);
    assert_eq!(range(&d), (4, 2, 4, 3));
    // Built-ins have no definition to go to.
    assert!(lsp.at("textDocument/definition", u, 7, 2).is_null());
    // Highlights: the definition is written, the uses are read.
    let h = lsp.at("textDocument/documentHighlight", u, 4, 2);
    let kinds: Vec<u64> = h.as_array().iter().map(|x| x.get("kind").as_u64().unwrap()).collect();
    assert_eq!(kinds, [3, 2, 2]);
    // The outline.
    let syms = lsp.result("textDocument/documentSymbol", &format!(r#"{{"textDocument":{{"uri":{}}}}}"#, s(u)));
    let names: Vec<&str> = syms.as_array().iter().map(|x| x.get("name").as_str().unwrap()).collect();
    assert_eq!(names, ["Shape", "area", "twice", "test \"area\""]);
    let shape = &syms.as_array()[0];
    let ctors: Vec<&str> = shape.get("children").as_array().iter().map(|x| x.get("name").as_str().unwrap()).collect();
    assert_eq!(ctors, ["Circle", "Dot"]);
    assert_eq!(range(&syms.as_array()[1]), (1, 0, 2, 16));
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn ranges_and_texts_in_patterns() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/ranges.ry";
    let text = "fn title(\"Mr \" + name) = upper(name)\nfn title(other) = other\nfn size(n) = match n\n  0..10 -> \"small\"\n  10..99 -> \"big\"\n  _ -> \"huge\"\nend\nprint(title(\"Mr X\"), size(3))\n";
    let diags = lsp.open(u, text);
    assert_eq!(diags.len(), 1, "{diags:?}");
    assert_eq!(diags[0].get("code").as_str(), Some("W010"));
    assert_eq!(range(&diags[0]), (4, 2, 4, 8));
    // The rest of the text is a name like the others.
    let d = lsp.at("textDocument/definition", u, 0, 32);
    assert_eq!(range(&d), (0, 17, 0, 21));
    let h = lsp.at("textDocument/documentHighlight", u, 0, 18);
    assert_eq!(h.as_array().len(), 2);
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn local_functions_and_mistakes_found_by_types() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/types.ry";
    let text = "fn outer(n)\n  fn even(0) = true\n  fn even(k) = odd(k - 1)\n  fn odd(0) = false\n  fn odd(k) = even(k - 1)\n  even(n)\nend\nprint(outer(4))\n";
    assert!(lsp.open(u, text).is_empty());
    // Functions written one after the other see each other: `odd`, from `even`.
    let d = lsp.at("textDocument/definition", u, 2, 15);
    assert_eq!(range(&d), (3, 5, 3, 8));
    // A mistake found before running, with the help the run time gives.
    let diags = lsp.change(u, 2, "print(map(\"abc\", upper), None < Some(1))\n");
    assert_eq!(diags.len(), 1, "{diags:?}");
    assert_eq!(diags[0].get("code").as_str(), Some("E005"));
    let message = diags[0].get("message").as_str().unwrap();
    assert!(message.starts_with("`map` expects a list, a set, an Option or a Result as argument 1, but got Text \"abc\""), "{message}");
    assert!(message.contains("help: a text is not a list: `chars(text)`"), "{message}");
    assert_eq!(range(&diags[0]), (0, 10, 0, 15));
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn several_files() {
    let dir = temp_dir("files");
    std::fs::write(dir.join("geometry.ry"), "## Area of a rectangle.\nfn area(w, h) = w * h\nfn square(s) = area(s, s)\n").unwrap();
    std::fs::write(dir.join("broken.ry"), "fn oops(x) = x +\n").unwrap();
    let main = dir.join("main.ry");
    let main_text = "use \"geometry.ry\"\nprint(area(2, 3), square(2))\n";
    std::fs::write(&main, main_text).unwrap();
    let init = format!(r#"{{"processId":null,"rootUri":{},"capabilities":{{}}}}"#, s(&uri(&dir)));
    let (mut lsp, _) = Lsp::start("en", &init);
    let mu = uri(&main);
    let gu = uri(&dir.join("geometry.ry"));
    assert!(lsp.open(&mu, main_text).is_empty());
    // Hover and definition reach the other file.
    let h = hover_text(&lsp.at("textDocument/hover", &mu, 1, 7));
    assert!(h.contains("Area of a rectangle.") && h.contains("geometry.ry"), "{h}");
    assert!(h.contains("fn area(w: Int|Float, h: Int|Float) -> Int|Float"), "{h}");
    let d = lsp.at("textDocument/definition", &mu, 1, 7);
    assert_eq!(d.get("uri").as_str(), Some(gu.as_str()));
    assert_eq!(range(&d), (1, 3, 1, 7));
    let d = lsp.at("textDocument/definition", &mu, 0, 8);
    assert_eq!(d.get("uri").as_str(), Some(gu.as_str()));
    // References in both files.
    let refs = lsp.result(
        "textDocument/references",
        &format!(
            r#"{{"textDocument":{{"uri":{}}},"position":{{"line":1,"character":7}},"context":{{"includeDeclaration":true}}}}"#,
            s(&mu)
        ),
    );
    let mut found: Vec<(String, u64)> = refs.as_array().iter().map(|r| (r.get("uri").as_str().unwrap().to_string(), range(r).0)).collect();
    found.sort();
    assert_eq!(found, [(gu.clone(), 1), (gu.clone(), 2), (mu.clone(), 1)]);
    // File names inside `use "..."`, but not the file itself.
    let typed = |line: u32, character: u32, trigger: &str| {
        format!(
            r#"{{"textDocument":{{"uri":{}}},"position":{{"line":{line},"character":{character}}},"context":{{"triggerKind":2,"triggerCharacter":{}}}}}"#,
            s(&mu),
            s(trigger)
        )
    };
    lsp.change(&mu, 2, "use \"\nprint(area(2, 3), square(2))\n");
    assert_eq!(labels(&lsp.result("textDocument/completion", &typed(0, 5, "\""))), ["broken.ry", "geometry.ry"]);
    lsp.change(&mu, 3, "x = 6 /\n");
    assert!(lsp.result("textDocument/completion", &typed(0, 7, "/")).as_array().is_empty(), "no list after a division");
    lsp.change(&mu, 4, main_text);
    // Rename across files.
    let edit = lsp.result(
        "textDocument/rename",
        &format!(r#"{{"textDocument":{{"uri":{}}},"position":{{"line":1,"character":7}},"newName":"surface"}}"#, s(&mu)),
    );
    assert_eq!(edit.get_path(&["changes", &mu]).as_array().len(), 1);
    assert_eq!(edit.get_path(&["changes", &gu]).as_array().len(), 2);
    // The other file's problems are summed up on the `use` line, with links.
    let diags = lsp.change(&mu, 2, "use \"broken.ry\"\nuse \"missing.ry\"\nprint(oops(1))\n");
    assert_eq!(diags.len(), 2, "{diags:?}");
    let summary = diags.iter().find(|d| range(d).0 == 0).unwrap();
    assert!(summary.get("message").as_str().unwrap().contains("the file `broken.ry` has 1 error(s)"), "{summary}");
    let link = &summary.get("relatedInformation").as_array()[0];
    assert_eq!(link.get_path(&["location", "uri"]).as_str(), Some(uri(&dir.join("broken.ry")).as_str()));
    let missing = diags.iter().find(|d| range(d).0 == 1).unwrap();
    assert_eq!(missing.get("code").as_str(), Some("E017"));
    // Opening the used file shows its problem; fixing it clears the summary.
    let bu = uri(&dir.join("broken.ry"));
    assert_eq!(lsp.open(&bu, "fn oops(x) = x +\n").len(), 1);
    lsp.change(&mu, 3, "use \"broken.ry\"\nprint(oops(1))\n");
    assert!(lsp.change(&bu, 2, "fn oops(x) = x + 1\n").is_empty());
    assert!(lsp.diagnostics(&mu).is_empty());
    lsp.notify("textDocument/didClose", &format!(r#"{{"textDocument":{{"uri":{}}}}}"#, s(&bu)));
    assert!(lsp.diagnostics(&bu).is_empty());
    assert_eq!(lsp.stop(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn rename_keeps_the_meaning_or_refuses() {
    let init = r#"{"processId":null,"capabilities":{"textDocument":{"rename":{"prepareSupport":true}}}}"#;
    let (mut lsp, r) = Lsp::start("en", init);
    assert_eq!(r.get_path(&["result", "capabilities", "renameProvider", "prepareProvider"]).as_bool(), Some(true));
    let u = "file:///nowhere/rename.ry";
    let text =
        "fn f(age)\n  name = \"Ada\"\n  person = {name, age}\n  {name: n, age: a} = person\n  \"{n} {a} {name}\"\nend\nprint(f(3))\n";
    assert!(lsp.open(u, text).is_empty());
    let rename = |lsp: &mut Lsp, line: u32, ch: u32, new: &str| {
        lsp.request(
            "textDocument/rename",
            &format!(r#"{{"textDocument":{{"uri":{}}},"position":{{"line":{line},"character":{ch}}},"newName":{}}}"#, s(u), s(new)),
        )
    };
    // `{name}` keeps its field name: it becomes `{name: nom}`; the use inside the text is renamed too.
    let r = rename(&mut lsp, 1, 3, "nom");
    let edits = r.get_path(&["result", "changes", u]).as_array().to_vec();
    let new: Vec<&str> = edits.iter().map(|e| e.get("newText").as_str().unwrap()).collect();
    assert_eq!(new, ["nom", "name: nom", "nom"]);
    // A name that would be captured or defined twice is refused, with the reason.
    let r = rename(&mut lsp, 1, 3, "person");
    assert!(r.get_path(&["error", "message"]).as_str().unwrap().starts_with("cannot rename `name` to `person`"), "{r}");
    let r = rename(&mut lsp, 1, 3, "age");
    assert!(!r.get("error").is_null(), "{r}");
    // Not a valid name, a keyword, a built-in.
    for bad in ["2x", "match", "Name"] {
        let r = rename(&mut lsp, 1, 3, bad);
        assert!(!r.get("error").is_null(), "{bad}: {r}");
    }
    let r = lsp.request("textDocument/prepareRename", &at(u, 6, 1));
    assert!(r.get_path(&["error", "message"]).as_str().unwrap().contains("built-in"), "{r}");
    let p = lsp.at("textDocument/prepareRename", u, 0, 3);
    assert_eq!(p.get("placeholder").as_str(), Some("f"));
    // The function and its call.
    let r = rename(&mut lsp, 6, 7, "describe");
    assert_eq!(r.get_path(&["result", "changes", u]).as_array().len(), 2);
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn formatting_gives_the_layout_of_rythme_fmt() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/format.ry";
    let text = "fn double(x)=x*2   # twice\nprint( double(21) )\n";
    lsp.open(u, text);
    let params = format!(r#"{{"textDocument": {{"uri": "{u}"}}, "options": {{"tabSize": 2, "insertSpaces": true}}}}"#);
    let edits = lsp.result("textDocument/formatting", &params);
    let expected = rythme::format::format(text).unwrap();
    let [edit] = edits.as_array() else { panic!("one edit expected: {edits}") };
    assert_eq!(edit.get("newText").as_str(), Some(expected.as_str()), "{edits}");
    assert_eq!(edit.get_path(&["range", "start", "line"]).as_u64(), Some(0));
    assert_eq!(edit.get_path(&["range", "end", "line"]).as_u64(), Some(2));
    // Already formatted: no edit.
    lsp.change(u, 2, &expected);
    let edits = lsp.result("textDocument/formatting", &params);
    assert!(edits.as_array().is_empty(), "{edits}");
    // A syntax error: an error answer, and the code is left as it is.
    lsp.change(u, 3, "print(1 +\n");
    let r = lsp.request("textDocument/formatting", &params);
    assert!(r.get_path(&["error", "message"]).as_str().unwrap_or("").contains("not formatted"), "{r}");
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn shutdown_and_exit_codes() {
    // `exit` after `shutdown`: 0.
    let (lsp, _) = Lsp::start("en", INIT);
    assert_eq!(lsp.stop(), 0);
    // `exit` without `shutdown`: 1.
    let (mut lsp, _) = Lsp::start("en", INIT);
    lsp.notify("exit", "null");
    assert_eq!(lsp.wait(), 1);
    // The editor closes the input: 1 without `shutdown`.
    let (mut lsp, _) = Lsp::start("en", INIT);
    lsp.stdin = None;
    assert_eq!(lsp.wait(), 1);
    // After `shutdown`, requests are refused.
    let (mut lsp, _) = Lsp::start("en", INIT);
    assert!(lsp.result("shutdown", "null").is_null());
    let r = lsp.request("textDocument/hover", &at("file:///x.ry", 0, 0));
    assert_eq!(r.get_path(&["error", "code"]).as_i64(), Some(-32600));
    lsp.notify("exit", "null");
    assert_eq!(lsp.wait(), 0);
}

#[test]
fn malformed_messages_never_stop_the_server() {
    let mut lsp = Lsp::spawn("en");
    // Before `initialize`.
    let r = lsp.request("textDocument/hover", &at("file:///x.ry", 0, 0));
    assert_eq!(r.get_path(&["error", "code"]).as_i64(), Some(-32002));
    let r = lsp.request("initialize", INIT);
    assert!(r.get("error").is_null());
    lsp.notify("initialized", "{}");
    // Not JSON, not an object, not UTF-8.
    for body in [&b"{not json"[..], b"[1, 2]", b"\"text\"", b"{\"id\": 1, \"method\": \"x\", \"params\": \xff}"] {
        lsp.raw(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
        lsp.raw(body);
        let r = lsp.recv();
        assert!(r.get("id").is_null(), "{r}");
        let code = r.get_path(&["error", "code"]).as_i64().unwrap();
        assert!(code == -32700 || code == -32600, "{r}");
    }
    // A header without a length, then a valid message.
    lsp.raw(b"Content-Type: nothing\r\n\r\n");
    let r = lsp.request("unknown/method", "{}");
    assert_eq!(r.get_path(&["error", "code"]).as_i64(), Some(-32601));
    // Missing or wrong parameters, unknown documents, absurd positions.
    assert!(lsp.result("textDocument/hover", "{}").is_null());
    assert!(lsp.result("textDocument/hover", "null").is_null());
    assert!(lsp.result("textDocument/completion", &at("file:///never/opened.ry", 3, 3)).is_null());
    let u = "file:///nowhere/odd.ry";
    lsp.open(u, "x = 1\n");
    for (line, ch) in [(0, 99999), (99999, 0), (4294967295, 4294967295)] {
        lsp.at("textDocument/hover", u, line, ch);
        lsp.at("textDocument/completion", u, line, ch);
        lsp.at("textDocument/definition", u, line, ch);
    }
    lsp.notify("textDocument/didChange", r#"{"textDocument":{"uri":"file:///nowhere/odd.ry"},"contentChanges":[{"range":{"start":{"line":9,"character":9},"end":{"line":0,"character":0}},"text":"y"}]}"#);
    lsp.notify("textDocument/didOpen", r#"{"textDocument":{}}"#);
    lsp.notify("$/cancelRequest", r#"{"id":1}"#);
    let r = lsp.request("textDocument/rename", &at(u, 0, 0));
    assert_eq!(r.get_path(&["error", "code"]).as_i64(), Some(-32602));
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn incremental_changes_are_applied() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/incremental.ry";
    lsp.open(u, "é = 1\nprint(é)\n");
    // Replace `é` (one UTF-16 unit) in `print(é)` by `zz`.
    lsp.notify(
        "textDocument/didChange",
        &format!(
            r#"{{"textDocument":{{"uri":{},"version":2}},"contentChanges":[{{"range":{{"start":{{"line":1,"character":6}},"end":{{"line":1,"character":7}}}},"text":"zz"}}]}}"#,
            s(u)
        ),
    );
    let diags = lsp.diagnostics(u);
    let codes: Vec<&str> = diags.iter().filter_map(|d| d.get("code").as_str()).collect();
    assert!(codes.contains(&"E003"), "{diags:?}");
    let e003 = diags.iter().find(|d| d.get("code").as_str() == Some("E003")).unwrap();
    assert_eq!(range(e003), (1, 6, 1, 8));
    assert_eq!(lsp.stop(), 0);
}

/// Every prefix of every example, as if typed one piece at a time: the server
/// answers every request and never stops.
#[test]
fn typing_the_examples_never_breaks_the_server() {
    let init = r#"{"processId":null,"capabilities":{"textDocument":{"completion":{"completionItem":{"snippetSupport":true}},"documentSymbol":{"hierarchicalDocumentSymbolSupport":true}}}}"#;
    let (mut lsp, _) = Lsp::start("en", init);
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    let mut files: Vec<PathBuf> = [dir.clone(), dir.join("modules")]
        .iter()
        .flat_map(|d| std::fs::read_dir(d).unwrap().flatten().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "ry"))
        .collect();
    files.sort();
    for (n, file) in files.iter().enumerate() {
        let text = std::fs::read_to_string(file).unwrap();
        let u = uri(file);
        lsp.open(&u, "");
        let chars: Vec<(usize, char)> = text.char_indices().collect();
        let step = (chars.len() / 25).max(1);
        let mut version = 1;
        for k in (0..chars.len()).step_by(step).chain([chars.len()]) {
            let cut = chars.get(k).map_or(text.len(), |(i, _)| *i);
            let prefix = &text[..cut];
            version += 1;
            lsp.change(&u, version, prefix);
            let lines = prefix.lines().count() as u32;
            let last = prefix.lines().last().map_or(0, |l| l.encode_utf16().count()) as u32;
            for method in [
                "textDocument/hover",
                "textDocument/completion",
                "textDocument/signatureHelp",
                "textDocument/definition",
                "textDocument/documentHighlight",
            ] {
                let r = lsp.request(method, &at(&u, lines.saturating_sub(1), last));
                assert!(r.get("error").is_null(), "{method} on {} ({cut} bytes): {r}", file.display());
                let r = lsp.request(method, &at(&u, (k as u32 * 7) % (lines + 1), (k as u32 * 13) % 40));
                assert!(r.get("error").is_null(), "{method} on {}: {r}", file.display());
            }
            for method in
                ["textDocument/documentSymbol", "textDocument/semanticTokens/full", "textDocument/codeLens", "textDocument/foldingRange"]
            {
                let r = lsp.request(method, &doc(&u));
                assert!(r.get("error").is_null(), "{method} on {} ({cut} bytes): {r}", file.display());
            }
            let r = lsp.request("textDocument/inlayHint", &lines_range(&u, 0, lines));
            assert!(r.get("error").is_null(), "{r}");
            let positions = format!(
                r#"{{"textDocument":{{"uri":{}}},"positions":[{{"line":{},"character":{last}}}]}}"#,
                s(&u),
                lines.saturating_sub(1)
            );
            let r = lsp.request("textDocument/selectionRange", &positions);
            assert!(r.get("error").is_null(), "{r}");
        }
        // The whole example has no error.
        let errors: Vec<Json> = lsp.change(&u, version + 1, &text).into_iter().filter(|d| d.get("severity").as_u64() == Some(1)).collect();
        assert!(errors.is_empty(), "{}: {errors:?}", file.display());
        // Its colors are in order, one name after the other, on its lines.
        let tokens = decode(lsp.result("textDocument/semanticTokens/full", &doc(&u)).get("data"));
        let line_texts: Vec<&str> = text.lines().collect();
        let mut previous = (0, 0);
        for t in &tokens {
            assert!((t[0], t[1]) >= previous, "{}: {tokens:?}", file.display());
            let line = line_texts[t[0] as usize];
            assert!(t[1] + t[2] <= line.encode_utf16().count() as u64, "{}: {t:?}", file.display());
            previous = (t[0], t[1] + t[2]);
        }
        assert!(!tokens.is_empty(), "{}", file.display());
        for f in lsp.result("textDocument/foldingRange", &doc(&u)).as_array() {
            assert!(f.get("startLine").as_u64() < f.get("endLine").as_u64(), "{}: {f}", file.display());
        }
        if n % 3 == 0 {
            lsp.notify("textDocument/didClose", &format!(r#"{{"textDocument":{{"uri":{}}}}}"#, s(&u)));
            lsp.diagnostics(&u);
        }
    }
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn help_when_started_by_hand() {
    let out = Command::new(env!("CARGO_BIN_EXE_rythme")).args(["lsp", "--lang", "fr", "--help"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("RYTHME DANS VOTRE ÉDITEUR") && text.contains("rythme lsp"), "{text}");
}

#[test]
fn files_used_with_a_name() {
    let dir = temp_dir("named");
    std::fs::create_dir_all(dir.join("shapes")).unwrap();
    std::fs::write(
        dir.join("geometry.ry"),
        "## Area of a square.\nfn area(side) = side * _square(side) // side\nfn _square(x) = x * x\ntype Shape = Circle(r) | Dot\n",
    )
    .unwrap();
    std::fs::write(dir.join("shapes/circle.ry"), "fn perimeter(r) = 2 * pi * r\n").unwrap();
    let main = dir.join("main.ry");
    let main_text = "use geometry\nprint(geometry.area(2))\n";
    std::fs::write(&main, main_text).unwrap();
    let init = format!(r#"{{"processId":null,"rootUri":{},"capabilities":{{}}}}"#, s(&uri(&dir)));
    let (mut lsp, _) = Lsp::start("en", &init);
    let mu = uri(&main);
    let gu = uri(&dir.join("geometry.ry"));
    assert!(lsp.open(&mu, main_text).is_empty());
    // Hover says which file the name comes from; definition jumps into it.
    let h = hover_text(&lsp.at("textDocument/hover", &mu, 1, 16));
    assert!(h.contains("Area of a square.") && h.contains("geometry.ry"), "{h}");
    let d = lsp.at("textDocument/definition", &mu, 1, 16);
    assert_eq!(d.get("uri").as_str(), Some(gu.as_str()));
    assert_eq!(range(&d), (1, 3, 1, 7));
    let h = hover_text(&lsp.at("textDocument/hover", &mu, 0, 6));
    assert!(h.contains("geometry.area(side)") && !h.contains("_square"), "{h}");
    let typed = |line: u32, character: u32, trigger: &str| {
        format!(
            r#"{{"textDocument":{{"uri":{}}},"position":{{"line":{line},"character":{character}}},"context":{{"triggerKind":2,"triggerCharacter":{}}}}}"#,
            s(&mu),
            s(trigger)
        )
    };
    // After `geometry.`: what the file gives, not its private names.
    lsp.change(&mu, 2, "use geometry\nprint(geometry.)\n");
    let names = labels(&lsp.result("textDocument/completion", &typed(1, 15, ".")));
    assert!(names.contains(&"area".to_string()) && names.contains(&"Circle".to_string()), "{names:?}");
    assert!(!names.contains(&"_square".to_string()) && !names.contains(&"print".to_string()), "{names:?}");
    // After `use `: the files and folders nearby, without `.ry`.
    lsp.change(&mu, 3, "use \nprint(1)\n");
    let names = labels(&lsp.result("textDocument/completion", &typed(0, 4, " ")));
    assert_eq!(names, ["geometry", "shapes/"]);
    lsp.change(&mu, 4, "use shapes/\nprint(1)\n");
    let names = labels(&lsp.result("textDocument/completion", &typed(0, 11, "/")));
    assert_eq!(names, ["circle"]);
    // A name of another file, without `use`: a quick fix adds it.
    let diags = lsp.change(&mu, 5, "print(area(2))\n");
    assert_eq!(diags.len(), 1, "{diags:?}");
    assert!(diags[0].get("message").as_str().unwrap().contains("add `use geometry`"), "{:?}", diags[0]);
    let params = format!(
        r#"{{"textDocument":{{"uri":{}}},"range":{{"start":{{"line":0,"character":7}},"end":{{"line":0,"character":7}}}},"context":{{"diagnostics":[]}}}}"#,
        s(&mu)
    );
    let actions = lsp.result("textDocument/codeAction", &params);
    let fix = &actions.as_array()[0];
    assert_eq!(fix.get("title").as_str(), Some("Add `use geometry` and write `geometry.area`"));
    let edits = fix.get_path(&["edit", "changes", &mu]).as_array();
    assert_eq!(edits[0].get("newText").as_str(), Some("use geometry\n"));
    assert_eq!(range(&edits[0]), (0, 0, 0, 0));
    assert_eq!(edits[1].get("newText").as_str(), Some("geometry.area"));
    assert_eq!(range(&edits[1]), (0, 6, 0, 10));
    // A private name, and a name the file does not have.
    let diags = lsp.change(&mu, 6, "use geometry\nprint(geometry._square(2), geometry.aera(1))\n");
    let messages: Vec<&str> = diags.iter().filter_map(|d| d.get("message").as_str()).collect();
    assert!(messages.iter().any(|m| m.contains("`_square` is private to the file `geometry.ry`")), "{messages:?}");
    assert!(messages.iter().any(|m| m.contains("did you mean `geometry.area`?")), "{messages:?}");
    // Rename from the file used: `geometry.area` follows.
    lsp.change(&mu, 7, main_text);
    let edit = lsp.result(
        "textDocument/rename",
        &format!(r#"{{"textDocument":{{"uri":{}}},"position":{{"line":1,"character":16}},"newName":"surface"}}"#, s(&mu)),
    );
    let main_edits = edit.get_path(&["changes", &mu]).as_array();
    assert_eq!(main_edits.len(), 1);
    assert_eq!(range(&main_edits[0]), (1, 15, 1, 19));
    assert_eq!(lsp.stop(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

fn doc(u: &str) -> String {
    format!(r#"{{"textDocument":{{"uri":{}}}}}"#, s(u))
}

fn lines_range(u: &str, first: u32, last: u32) -> String {
    format!(
        r#"{{"textDocument":{{"uri":{}}},"range":{{"start":{{"line":{first},"character":0}},"end":{{"line":{last},"character":0}}}}}}"#,
        s(u)
    )
}

/// The inlay hints: (line, character, label).
fn hints(lsp: &mut Lsp, u: &str) -> Vec<(u64, u64, String)> {
    let r = lsp.result("textDocument/inlayHint", &lines_range(u, 0, 999));
    r.as_array()
        .iter()
        .map(|h| {
            let p = h.get("position");
            (p.get("line").as_u64().unwrap(), p.get("character").as_u64().unwrap(), h.get("label").as_str().unwrap().to_string())
        })
        .collect()
}

#[test]
fn inlay_hints_show_the_types_the_checker_is_sure_of() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/hints.ry";
    let text = "fn area(r) = 3.14 * r * r\nnames = [\"Ada\", \"Alan\"]\nlengths = map(names, fn(n) = len(n))\nperson = {name: \"Ada\", age: 36}\nfn greet(who)\n  message = \"Hello {who}\"\n  message\nend\nfound = first(names)\nr = match found\n  Some(v) -> v\n  None -> \"nobody\"\nend\nx = 12\n{age} = person\nprint(area(2), lengths, greet(1), r, x, age)\n";
    assert!(lsp.open(u, text).is_empty());
    let expected = [
        // What the code of `area` needs its parameter to be (`check::signatures`).
        (0, 9, ": Int|Float"),
        (0, 10, "→ Float"),
        (1, 5, ": List(Text)"),
        (2, 7, ": List(Int)"),
        // The parameter of a function given to `map`: the elements of the list.
        (2, 25, ": Text"),
        (3, 6, ": {name: Text, age: Int}"),
        (4, 13, "→ Text"),
        (5, 9, ": Text"),
        (8, 5, ": Option(Text)"),
        (9, 1, ": Text"),
        (10, 8, ": Text"),
    ];
    let expected: Vec<(u64, u64, String)> = expected.iter().map(|(l, c, t)| (*l, *c, t.to_string())).collect();
    // Nothing for `who` (it may be anything), `x = 12` (written right there), `{age}`.
    assert_eq!(hints(&mut lsp, u), expected);
    // Only the hints of the lines asked.
    assert_eq!(lsp.result("textDocument/inlayHint", &lines_range(u, 1, 2)).as_array().len(), 1);
    // A file the checker cannot read: no hints rather than wrong ones.
    lsp.change(u, 2, "names = [\"Ada\"]\nprint(names\n");
    assert!(hints(&mut lsp, u).is_empty());
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn inlay_hints_show_the_inferred_signatures_of_functions() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/signatures.ry";
    let text = "fn half(x) = x // 2\nfn shout(t, n) = upper(t) + repeat(\"!\", n)\nprint(half(4), shout(\"a\", 2))\n";
    assert!(lsp.open(u, text).is_empty());
    let found = hints(&mut lsp, u);
    for want in [(0, 9, ": Int|Float"), (1, 10, ": Text"), (1, 13, ": Int"), (1, 14, "→ Text")] {
        assert!(found.contains(&(want.0, want.1, want.2.to_string())), "{want:?} in {found:?}");
    }
    assert_eq!(lsp.stop(), 0);
}

/// Semantic tokens, decoded: (line, character, length, type, modifiers).
fn decode(data: &Json) -> Vec<[u64; 5]> {
    let n: Vec<u64> = data.as_array().iter().map(|x| x.as_u64().unwrap()).collect();
    let (mut line, mut ch) = (0, 0);
    n.chunks(5)
        .map(|t| {
            if t[0] > 0 {
                line += t[0];
                ch = t[1];
            } else {
                ch += t[1];
            }
            [line, ch, t[2], t[3], t[4]]
        })
        .collect()
}

#[test]
fn semantic_tokens_tell_what_each_name_is() {
    let (mut lsp, r) = Lsp::start("en", INIT);
    let legend = r.get_path(&["result", "capabilities", "semanticTokensProvider", "legend"]);
    let types: Vec<&str> = legend.get("tokenTypes").as_array().iter().filter_map(|t| t.as_str()).collect();
    assert_eq!(types, ["function", "variable", "parameter", "type", "enumMember"]);
    let u = "file:///nowhere/tokens.ry";
    let text = "type Shape = Dot | Circle(r)\nfn size(Circle(r)) = r\nfn size(Dot) = 0\nété = size(Some(Dot)) + len(\"ab\")\nprint(été)\n";
    lsp.open(u, text);
    let full = lsp.result("textDocument/semanticTokens/full", &doc(u));
    let tokens = decode(full.get("data"));
    let expected: [[u64; 5]; 16] = [
        [0, 5, 5, 3, 1],  // Shape: a type, defined here
        [0, 13, 3, 4, 1], // Dot: a constructor
        [0, 19, 6, 4, 1],
        [1, 3, 4, 0, 1],  // size: a function
        [1, 8, 6, 4, 0],  // Circle in a pattern
        [1, 15, 1, 2, 1], // r: a parameter
        [1, 21, 1, 2, 0],
        [2, 3, 4, 0, 1],
        [2, 8, 3, 4, 0],
        [3, 0, 3, 1, 1], // été: a value (3 UTF-16 units)
        [3, 6, 4, 0, 0],
        [3, 11, 4, 4, 2], // Some: part of Rythme
        [3, 16, 3, 4, 0],
        [3, 24, 3, 0, 2], // len: a built-in function
        [4, 0, 5, 0, 2],
        [4, 6, 3, 1, 0],
    ];
    assert_eq!(tokens, expected);
    // After a change, only what changed.
    let id = full.get("resultId").as_str().unwrap().to_string();
    lsp.change(u, 2, &text.replace("print(été)", "print(été, été)"));
    let params = format!(r#"{{"textDocument":{{"uri":{}}},"previousResultId":{}}}"#, s(u), s(&id));
    let delta = lsp.result("textDocument/semanticTokens/full/delta", &params);
    let edits = delta.get("edits").as_array();
    assert_eq!(edits.len(), 1, "{delta}");
    assert_eq!(edits[0].get("start").as_u64(), Some(80), "{delta}");
    assert_eq!(edits[0].get("deleteCount").as_u64(), Some(0), "{delta}");
    assert_eq!(edits[0].get("data").as_array().len(), 5, "{delta}");
    assert_ne!(delta.get("resultId").as_str(), Some(id.as_str()));
    // An unknown previous id: every token again.
    let params = format!(r#"{{"textDocument":{{"uri":{}}},"previousResultId":"old"}}"#, s(u));
    assert_eq!(decode(lsp.result("textDocument/semanticTokens/full/delta", &params).get("data")).len(), 17);
    // Some lines only.
    let in_range = decode(lsp.result("textDocument/semanticTokens/range", &lines_range(u, 3, 3)).get("data"));
    assert_eq!(in_range.len(), 5);
    assert_eq!(in_range[0], [3, 0, 3, 1, 1]);
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn code_lenses_run_the_file_and_each_test() {
    let (mut lsp, _) = Lsp::start("fr", INIT);
    let u = "file:///nowhere/lenses.ry";
    lsp.open(
        u,
        "fn double(x) = x * 2\nprint(double(2))\ntest \"double\" = assert_eq(double(2), 4)\ntest \"zéro\" = assert_eq(double(0), 0)\n",
    );
    let lenses = lsp.result("textDocument/codeLens", &doc(u));
    let found: Vec<(u64, String, String, Vec<String>)> = lenses
        .as_array()
        .iter()
        .map(|l| {
            let c = l.get("command");
            let args = c.get("arguments").as_array().iter().map(|a| a.as_str().unwrap().to_string()).collect();
            (range(l).0, c.get("title").as_str().unwrap().to_string(), c.get("command").as_str().unwrap().to_string(), args)
        })
        .collect();
    let v = |a: &[&str]| a.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let expected: Vec<(u64, String, String, Vec<String>)> = vec![
        (1, "▶ Lancer".into(), "rythme.runFile".into(), v(&[u])),
        (2, "▶ Lancer les 2 tests".into(), "rythme.testFile".into(), v(&[u])),
        (2, "▶ Lancer le test".into(), "rythme.runTest".into(), v(&[u, "double"])),
        (3, "▶ Lancer le test".into(), "rythme.runTest".into(), v(&[u, "zéro"])),
    ];
    assert_eq!(found, expected);
    // Only definitions: nothing to run.
    lsp.change(u, 2, "fn double(x) = x * 2\n");
    assert!(lsp.result("textDocument/codeLens", &doc(u)).as_array().is_empty());
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn workspace_symbols_find_names_in_every_file() {
    let dir = temp_dir("symbols");
    std::fs::create_dir_all(dir.join("lib")).unwrap();
    std::fs::create_dir_all(dir.join(".hidden")).unwrap();
    std::fs::write(dir.join("lib/geometry.ry"), "type Shape = Circle(r) | Dot\nfn area(shape) = 0\n").unwrap();
    std::fs::write(dir.join(".hidden/secret.ry"), "fn area_secret() = 0\n").unwrap();
    std::fs::write(dir.join("broken.ry"), "fn area_broken(\n").unwrap();
    let main = dir.join("main.ry");
    std::fs::write(&main, "fn old() = 1\n").unwrap();
    let init = format!(r#"{{"processId":null,"rootUri":{},"capabilities":{{}}}}"#, s(&uri(&dir)));
    let (mut lsp, _) = Lsp::start("en", &init);
    // The open document wins over the file on the disk.
    let mu = uri(&main);
    lsp.open(&mu, "fn surface(x) = x\ntest \"surface\" = assert_eq(surface(1), 1)\n");
    let search = |lsp: &mut Lsp, q: &str| -> Vec<(String, u64, String, String)> {
        let r = lsp.result("workspace/symbol", &format!(r#"{{"query":{}}}"#, s(q)));
        r.as_array()
            .iter()
            .map(|x| {
                (
                    x.get("name").as_str().unwrap().to_string(),
                    x.get("kind").as_u64().unwrap(),
                    x.get_path(&["location", "uri"]).as_str().unwrap().to_string(),
                    x.get("containerName").as_str().unwrap_or("").to_string(),
                )
            })
            .collect()
    };
    let gu = uri(&dir.join("lib/geometry.ry"));
    assert_eq!(search(&mut lsp, "area"), [("area".to_string(), 12, gu.clone(), "geometry.ry".to_string())]);
    // Letters in order, whatever the case: `Circle` for `CLe`.
    assert_eq!(search(&mut lsp, "CLe"), [("Circle".to_string(), 22, gu.clone(), "Shape · geometry.ry".to_string())]);
    let names: Vec<String> = search(&mut lsp, "").into_iter().map(|x| x.0).collect();
    assert_eq!(names, ["Circle", "Dot", "Shape", "area", "surface", "test \"surface\""]);
    // A file changed on the disk is read again.
    std::fs::write(dir.join("lib/geometry.ry"), "fn perimeter(shape) = 0\n").unwrap();
    let later = std::time::SystemTime::now() + Duration::from_secs(5);
    std::fs::File::options().write(true).open(dir.join("lib/geometry.ry")).unwrap().set_modified(later).unwrap();
    assert_eq!(search(&mut lsp, "per").len(), 1);
    assert!(search(&mut lsp, "area").is_empty());
    assert_eq!(lsp.stop(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn folding_and_growing_the_selection() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/fold.ry";
    let text = "# A comment\n# on two lines.\nfn greet(who)\n  message = \"Hello {who}\"\n  match who\n    \"Ada\" -> message\n    _ -> \"?\"\n  end\nend\nxs = [\n  1,\n  2,\n]\nprint(greet(xs))\n";
    lsp.open(u, text);
    let folds = lsp.result("textDocument/foldingRange", &doc(u));
    let found: Vec<(u64, u64, Option<&str>)> = folds
        .as_array()
        .iter()
        .map(|f| (f.get("startLine").as_u64().unwrap(), f.get("endLine").as_u64().unwrap(), f.get("kind").as_str()))
        .collect();
    // The line of `end` or `]` stays visible.
    assert_eq!(found, [(0, 1, Some("comment")), (2, 7, None), (4, 6, None), (9, 11, None)]);
    // From `who` in the text, out to the whole document.
    let params = format!(r#"{{"textDocument":{{"uri":{}}},"positions":[{{"line":3,"character":21}}]}}"#, s(u));
    let sel = lsp.result("textDocument/selectionRange", &params);
    let mut chain = Vec::new();
    let mut node = &sel.as_array()[0];
    while !node.is_null() {
        chain.push(range(node));
        node = node.get("parent");
    }
    assert_eq!(chain, [(3, 20, 3, 23), (3, 12, 3, 25), (3, 2, 3, 25), (2, 0, 8, 3), (0, 0, 14, 0)]);
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn ruby_style_in_hints_and_semantic_tokens() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/ruby_extras.ry";
    let text = "fn adult?(p) = p.age >= 18\nfn status(n) = if n > 0 then :ok else :error\nfn name(:ok) = \"fine\"\nfn name(:error) = \"bad\"\ns = status(2)\ndoubled = [1, 2].map { |x| x * 2 }\nprint(adult?({age: 20}), name(s), doubled)\n";
    let diags = lsp.open(u, text);
    assert!(diags.is_empty(), "{diags:?}");
    let found = hints(&mut lsp, u);
    for want in [(0, 11, ": Record"), (1, 12, "→ Symbol"), (4, 1, ": Symbol"), (5, 7, ": List(Int)"), (5, 25, ": Int")] {
        assert!(found.contains(&(want.0, want.1, want.2.to_string())), "{want:?} in {found:?}");
    }
    let tokens = decode(lsp.result("textDocument/semanticTokens/full", &doc(u)).get("data"));
    // `adult?` is one name, `?` included; `:ok` is colored like a constructor.
    assert!(tokens.contains(&[0, 3, 6, 0, 1]), "{tokens:?}");
    assert!(tokens.contains(&[1, 29, 3, 4, 0]) && tokens.contains(&[2, 8, 3, 4, 0]), "{tokens:?}");
    // The parameter of a block.
    assert!(tokens.contains(&[5, 24, 1, 2, 1]), "{tokens:?}");
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn ruby_style_names_blocks_and_completion_after_a_dot() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/ruby.ry";
    let text = "xs = [1, 2, 3]\nprint(xs.sum, \"hi\".upcase, xs.select do |x| x.even? end)\nfn adult?(p) = p.age >= 18\nprint(adult?({age: 20}), :ok)\n";
    let diags = lsp.open(u, text);
    assert!(diags.is_empty(), "{diags:?}");
    // `xs.sum` calls `sum`; `upcase` is the Ruby name of `upper`.
    let h = hover_text(&lsp.at("textDocument/hover", u, 1, 10));
    assert!(h.contains("sum(list, f)") && h.contains("`value.sum` reads the field `sum`, or calls `sum(value)`"), "{h}");
    let h = hover_text(&lsp.at("textDocument/hover", u, 1, 22));
    assert!(h.contains("the Ruby name of `upper`") && h.contains("upper(text)"), "{h}");
    // `even?` is one name; `adult?` a function of the file.
    let h = hover_text(&lsp.at("textDocument/hover", u, 1, 51));
    assert!(h.contains("the Ruby name of `is_even`"), "{h}");
    let h = hover_text(&lsp.at("textDocument/hover", u, 3, 8));
    assert!(h.contains("fn adult?(p)"), "{h}");
    // After `"text".`: the functions that take a text first, with their Ruby names.
    lsp.change(u, 2, "\"abc\".");
    let l = labels(&lsp.at("textDocument/completion", u, 0, 6));
    for name in ["upper", "upcase", "split", "len", "size", "empty?"] {
        assert!(l.iter().any(|x| x == name), "{name} missing in {l:?}");
    }
    assert!(!l.iter().any(|x| x == "sum" || x == "select"), "a text has no sum: {l:?}");
    // After a list, the functions on lists.
    lsp.change(u, 3, "[1, 2].");
    let l = labels(&lsp.at("textDocument/completion", u, 0, 7));
    assert!(l.iter().any(|x| x == "sum") && l.iter().any(|x| x == "select") && !l.iter().any(|x| x == "upcase"), "{l:?}");
    assert_eq!(lsp.stop(), 0);
}

#[test]
fn csv_and_random_seed_in_the_editor() {
    let (mut lsp, _) = Lsp::start("en", INIT);
    let u = "file:///nowhere/csv.ry";
    lsp.open(u, "rows = csv_parse(\"a;b\", \nrandom_seed(1)\n");
    let sig = lsp.at("textDocument/signatureHelp", u, 0, 24);
    assert_eq!(sig.get("signatures").as_array()[0].get("label").as_str(), Some("csv_parse(text, separator)"));
    assert_eq!(sig.get("activeParameter").as_u64(), Some(1));
    let h = hover_text(&lsp.at("textDocument/hover", u, 1, 3));
    assert!(h.contains("random_seed(n)") && h.contains("the same draws"), "{h}");
    assert_eq!(lsp.stop(), 0);
    // The documentation in the language of the editor.
    for (lang, words) in [
        ("fr", "Lit un tableau CSV"),
        ("es", "Lee una tabla CSV"),
        ("de", "Liest eine CSV-Tabelle"),
        ("pt", "Lê uma tabela CSV"),
        ("zh", "读取一个 CSV 表格"),
    ] {
        let (mut lsp, _) = Lsp::start(lang, INIT);
        lsp.open(u, "x = csv_parse(\"a\")\n");
        let h = hover_text(&lsp.at("textDocument/hover", u, 0, 6));
        assert!(h.contains(words), "{lang}: {h}");
        assert_eq!(lsp.stop(), 0);
    }
}
