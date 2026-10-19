//! `rythme mcp`, driven like an AI assistant drives it: JSON-RPC messages,
//! one per line, on its standard input and output (the stdio transport of
//! the Model Context Protocol).

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use rythme::lsp::json::Json;

struct Mcp {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    next: u64,
}

impl Mcp {
    fn start(lang: &str) -> Mcp {
        let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .arg("mcp")
            .env("RYTHME_LANG", lang)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("start rythme mcp");
        let input = child.stdin.take().unwrap();
        let output = BufReader::new(child.stdout.take().unwrap());
        Mcp { child, input, output, next: 0 }
    }

    fn send(&mut self, text: &str) {
        writeln!(self.input, "{text}").unwrap();
        self.input.flush().unwrap();
    }

    fn read(&mut self) -> Json {
        let mut line = String::new();
        self.output.read_line(&mut line).unwrap();
        assert!(line.ends_with('\n'), "one message per line: {line:?}");
        Json::parse(&line).unwrap_or_else(|e| panic!("not JSON ({e}): {line}"))
    }

    /// Sends a request and returns its result (it must not be an error).
    #[track_caller]
    fn request(&mut self, method: &str, params: &str) -> Json {
        self.next += 1;
        let id = self.next;
        self.send(&format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{params}}}"#));
        let reply = self.read();
        assert_eq!(reply.get("jsonrpc").as_str(), Some("2.0"));
        assert_eq!(reply.get("id").as_u64(), Some(id), "{reply}");
        assert!(reply.get("error").is_null(), "{method}: {reply}");
        reply.get("result").clone()
    }

    /// Calls a tool: its texts, and whether it reports an error.
    #[track_caller]
    fn call(&mut self, tool: &str, arguments: &str) -> (Vec<String>, bool) {
        let r = self.request("tools/call", &format!(r#"{{"name":"{tool}","arguments":{arguments}}}"#));
        let texts = r.get("content").as_array().iter().map(|c| {
            assert_eq!(c.get("type").as_str(), Some("text"));
            c.get("text").as_str().unwrap().to_string()
        });
        (texts.collect(), r.get("isError").as_bool().unwrap())
    }

    /// Calls a tool that gives one JSON document.
    #[track_caller]
    fn call_json(&mut self, tool: &str, arguments: &str) -> Json {
        let (texts, error) = self.call(tool, arguments);
        assert!(!error, "{tool}: {texts:?}");
        Json::parse(texts.last().unwrap()).unwrap()
    }

    fn stop(mut self) -> i32 {
        drop(self.input);
        self.child.wait().unwrap().code().unwrap_or(-1)
    }
}

fn init(mcp: &mut Mcp) -> Json {
    let r = mcp.request("initialize", r#"{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}"#);
    // A notification has no answer: the next line answers the next request.
    mcp.send(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
    r
}

fn q(text: &str) -> String {
    Json::Str(text.into()).to_string()
}

#[test]
fn a_session_runs_checks_tests_and_fixes_code() {
    let mut mcp = Mcp::start("en");
    let r = init(&mut mcp);
    assert_eq!(r.get("protocolVersion").as_str(), Some("2025-06-18"));
    assert_eq!(r.get_path(&["serverInfo", "name"]).as_str(), Some("rythme"));
    assert!(!r.get_path(&["capabilities", "tools"]).is_null());
    assert!(r.get("instructions").as_str().unwrap().contains("llms-full.txt"));
    assert_eq!(mcp.request("ping", "{}"), Json::Obj(Vec::new()));

    // The tools, each with a schema of its arguments.
    let tools = mcp.request("tools/list", "{}");
    let names: Vec<&str> = tools.get("tools").as_array().iter().filter_map(|t| t.get("name").as_str()).collect();
    assert_eq!(names, ["run", "check", "test", "fix", "format", "explain", "help", "steps", "reference"]);
    for t in tools.get("tools").as_array() {
        assert_eq!(t.get_path(&["inputSchema", "type"]).as_str(), Some("object"), "{t}");
        assert!(!t.get("description").as_str().unwrap().is_empty());
    }

    // run: the output, the input given, the arguments.
    let doc = mcp.call_json(
        "run",
        &format!(r#"{{"code":{},"stdin":"Ada\n","args":["x"]}}"#, q("name = input(\"\")\nprint(\"hi {name}\", args())\n")),
    );
    assert_eq!(doc.get("stdout").as_str(), Some("hi Ada [\"x\"]\n"));
    assert_eq!(doc.get("exit_code").as_u64(), Some(0));
    // A runtime error is a diagnostic of the document, not a failure of the tool.
    let doc = mcp.call_json("run", &format!(r#"{{"code":{}}}"#, q("print(1 / 0)\n")));
    assert_eq!(doc.get("diagnostics").as_array()[0].get("code").as_str(), Some("E008"));

    // check: the diagnostics and their fixes.
    let doc = mcp.call_json("check", &format!(r#"{{"code":{}}}"#, q("x = 3\nprint(x > 2 && x < 5)\n")));
    assert_eq!(doc.get("ok"), &Json::Bool(false));
    let d = &doc.get("diagnostics").as_array()[0];
    assert_eq!(d.get("file").as_str(), Some("main.ry"));
    assert_eq!(d.get("fixes").as_array()[0].get_path(&["edits"]).as_array()[0].get("new_text").as_str(), Some("and"));

    // fix: the fixed code first, then the report.
    let (texts, error) = mcp.call("fix", &format!(r#"{{"code":{}}}"#, q("let x = 3\nprint(x > 2 ? \"big\" : \"small\")\n")));
    assert!(!error);
    assert_eq!(texts[0], "x = 3\nprint(if x > 2 then \"big\" else \"small\")\n");
    assert_eq!(Json::parse(&texts[1]).unwrap().get("ok"), &Json::Bool(true));

    // test.
    let code = q("fn add(a, b) = a + b\ntest \"adds\" = assert_eq(add(1, 2), 3)\ntest \"wrong\" = assert_eq(add(1, 1), 3)\n");
    let doc = mcp.call_json("test", &format!(r#"{{"code":{code}}}"#));
    assert_eq!(doc.get_path(&["summary", "passed"]).as_u64(), Some(1));
    assert_eq!(doc.get_path(&["summary", "failed"]).as_u64(), Some(1));
    let doc = mcp.call_json("test", &format!(r#"{{"code":{code},"filter":"adds"}}"#));
    assert_eq!(doc.get("ok"), &Json::Bool(true));

    // format, explain, help, steps.
    let (texts, error) = mcp.call("format", &format!(r#"{{"code":{}}}"#, q("fn f(x)=x*2\n")));
    assert_eq!((texts[0].as_str(), error), ("fn f(x) = x * 2\n", false));
    let (texts, error) = mcp.call("format", &format!(r#"{{"code":{}}}"#, q("print(1 +)\n")));
    assert!(error && texts[0].contains("\"E001\""), "{texts:?}");
    let doc = mcp.call_json("explain", r#"{"code":"E011"}"#);
    assert_eq!(doc.get("code").as_str(), Some("E011"));
    let (_, error) = mcp.call("explain", r#"{"code":"E999"}"#);
    assert!(error);
    let doc = mcp.call_json("help", r#"{"name":"upcase"}"#);
    assert_eq!(doc.get_path(&["builtin", "name"]).as_str(), Some("upper"));
    let doc = mcp.call_json("help", "{}");
    assert_eq!(doc.get("builtins").as_array().len(), rythme::builtins::BUILTINS.len());
    let fib = q("fn fib(0) = 0\nfn fib(1) = 1\nfn fib(n) = fib(n - 1) + fib(n - 2)\n");
    let (texts, error) = mcp.call("steps", &format!(r#"{{"code":{fib},"expression":"fib(2)"}}"#));
    assert!(!error);
    assert_eq!(texts[0], "fib(2)\n  → fib(1) + fib(0)\n  → 1 + fib(0)\n  → 1 + 0\n  → 1\n");

    // The reference, as a tool and as a resource.
    let (texts, _) = mcp.call("reference", "{}");
    assert_eq!(texts[0], rythme::llms::full());
    let list = mcp.request("resources/list", "{}");
    let uris: Vec<&str> = list.get("resources").as_array().iter().filter_map(|r| r.get("uri").as_str()).collect();
    assert_eq!(uris, ["rythme://docs/llms-full.txt", "rythme://docs/llms.txt"]);
    let read = mcp.request("resources/read", r#"{"uri":"rythme://docs/llms-full.txt"}"#);
    assert_eq!(read.get("contents").as_array()[0].get("text").as_str(), Some(rythme::llms::full().as_str()));
    assert_eq!(mcp.stop(), 0);
}

#[test]
fn programs_stay_in_the_sandbox_and_within_their_limits() {
    let mut mcp = Mcp::start("en");
    init(&mut mcp);
    let doc = mcp.call_json("run", &format!(r#"{{"code":{}}}"#, q("print(read_file(\"Cargo.toml\"))\nprint(env(\"HOME\"))\n")));
    let out = doc.get("stdout").as_str().unwrap();
    assert!(out.starts_with("Err(\"`read_file` is not available in the sandbox"), "{out}");
    assert!(out.ends_with("\nNone\n"), "{out}");
    let doc = mcp.call_json("test", &format!(r#"{{"code":{}}}"#, q("test \"files\" = assert(not file_exists(\"Cargo.toml\"))\n")));
    assert_eq!(doc.get("ok"), &Json::Bool(true));
    // A program that never ends is stopped, and the server goes on.
    let (texts, error) = mcp.call("run", &format!(r#"{{"code":{},"timeout_seconds":0.5}}"#, q("fn f(n) = f(n + 1)\nf(0)\n")));
    assert!(error);
    assert!(texts[0].contains("stopped after 0.5 seconds"), "{texts:?}");
    if cfg!(target_os = "linux") {
        let (texts, error) =
            mcp.call("run", &format!(r#"{{"code":{},"timeout_seconds":30}}"#, q("fn grow(xs) = grow(xs + xs)\ngrow([1])\n")));
        assert!(error, "{texts:?}");
        assert!(texts[0].contains("MB of memory") || texts[0].contains("too large"), "{texts:?}");
    }
    let doc = mcp.call_json("run", &format!(r#"{{"code":{}}}"#, q("print(\"still here\")\n")));
    assert_eq!(doc.get("stdout").as_str(), Some("still here\n"));
    assert_eq!(mcp.stop(), 0);
}

#[test]
fn protocol_errors_are_answered() {
    let mut mcp = Mcp::start("fr");
    // An older version of the protocol is spoken; an unknown one gets the newest.
    let r = mcp.request("initialize", r#"{"protocolVersion":"2024-11-05","capabilities":{}}"#);
    assert_eq!(r.get("protocolVersion").as_str(), Some("2024-11-05"));
    let r = mcp.request("initialize", r#"{"protocolVersion":"1999-01-01","capabilities":{}}"#);
    assert_eq!(r.get("protocolVersion").as_str(), Some("2025-06-18"));
    mcp.send("{not json");
    let reply = mcp.read();
    assert_eq!(reply.get_path(&["error", "code"]).as_i64(), Some(-32700));
    mcp.send(r#"{"jsonrpc":"2.0","id":"a","method":"nothing/here"}"#);
    let reply = mcp.read();
    assert_eq!((reply.get("id").as_str(), reply.get_path(&["error", "code"]).as_i64()), (Some("a"), Some(-32601)));
    mcp.send(r#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"nope","arguments":{}}}"#);
    assert_eq!(mcp.read().get_path(&["error", "code"]).as_i64(), Some(-32602));
    mcp.send(r#"{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"check","arguments":{}}}"#);
    assert_eq!(mcp.read().get_path(&["error", "code"]).as_i64(), Some(-32602));
    mcp.send(r#"{"jsonrpc":"2.0","id":9,"method":"resources/read","params":{"uri":"rythme://nothing"}}"#);
    assert_eq!(mcp.read().get_path(&["error", "code"]).as_i64(), Some(-32002));
    // A batch gets a batch of answers.
    mcp.send(r#"[{"jsonrpc":"2.0","id":10,"method":"ping"},{"jsonrpc":"2.0","method":"notifications/initialized"},{"jsonrpc":"2.0","id":11,"method":"ping"}]"#);
    let replies = mcp.read();
    let ids: Vec<u64> = replies.as_array().iter().filter_map(|r| r.get("id").as_u64()).collect();
    assert_eq!(ids, [10, 11]);
    // The messages follow the language of the server; the codes do not.
    let doc = mcp.call_json("check", &format!(r#"{{"code":{}}}"#, q("print(nme)\n")));
    let d = &doc.get("diagnostics").as_array()[0];
    assert_eq!((d.get("code").as_str(), d.get("message").as_str()), (Some("E003"), Some("nom inconnu `nme`")));
    assert_eq!(mcp.stop(), 0);
}

#[test]
fn help_and_wrong_arguments() {
    let out = Command::new(env!("CARGO_BIN_EXE_rythme")).args(["mcp", "--help"]).env("RYTHME_LANG", "en").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("claude mcp add rythme -- rythme mcp"));
    let out = Command::new(env!("CARGO_BIN_EXE_rythme")).args(["mcp", "extra"]).env("RYTHME_LANG", "en").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}
