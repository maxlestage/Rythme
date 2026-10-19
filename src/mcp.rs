//! `rythme mcp`: a server of the Model Context Protocol, so that AI
//! assistants (Claude Code, Claude Desktop, Cursor...) can run, check, test,
//! format and fix Rythme code, and read its documentation, by themselves.
//!
//! The protocol is JSON-RPC 2.0, one message per line on the standard input
//! and output (the "stdio" transport), written with the JSON of the
//! language server. Each tool that runs code starts `rythme` again as a
//! child process, in the sandbox (`RYTHME_SANDBOX=1`: no files, folders or
//! environment variables), stopped after a time limit and, on Linux, when it
//! takes too much memory: a program that never ends or fills the memory
//! never takes the server down with it.

use std::io::{BufRead, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::lsp::json::{obj, Json};
use crate::machine;

/// The versions of the protocol this server speaks, the newest first.
const PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

/// How long a program may run, by default and at most (seconds).
const DEFAULT_SECONDS: f64 = 10.0;
const MAX_SECONDS: f64 = 60.0;

/// The memory a program may take (on Linux, where the server can see it).
const MAX_MEMORY: u64 = 512 << 20;

/// What a tool gives back to the assistant: some text, and whether it reports a failure.
struct Answer {
    texts: Vec<String>,
    error: bool,
}

impl Answer {
    fn ok(text: impl Into<String>) -> Answer {
        Answer { texts: vec![text.into()], error: false }
    }

    fn failed(text: impl Into<String>) -> Answer {
        Answer { texts: vec![text.into()], error: true }
    }
}

/// `rythme mcp`: serves until the standard input ends.
pub fn run(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        crate::io::out_line(&usage());
        return 0;
    }
    if let Some(bad) = args.first() {
        crate::io::err_line(&t!("`rythme mcp` takes no argument, not `{bad}`", "`rythme mcp` ne prend pas d'argument, pas `{bad}`"));
        crate::io::err_line(&usage());
        return 2;
    }
    if std::io::stdin().is_terminal() {
        crate::io::err_line(&usage());
    }
    let mut server = Server { tmp: 0 };
    let stdin = std::io::stdin();
    let mut line = String::new();
    let mut input = stdin.lock();
    loop {
        line.clear();
        match input.read_line(&mut line) {
            Ok(0) | Err(_) => return 0,
            Ok(_) => {}
        }
        let text = line.trim();
        if text.is_empty() {
            continue;
        }
        let reply = match Json::parse(text) {
            Ok(Json::Arr(batch)) => {
                let replies: Vec<Json> = batch.iter().filter_map(|m| server.handle(m)).collect();
                (!replies.is_empty()).then_some(Json::Arr(replies))
            }
            Ok(message) => server.handle(&message),
            Err(e) => Some(error_reply(Json::Null, -32700, &format!("parse error: {e}"))),
        };
        if let Some(reply) = reply {
            let mut out = std::io::stdout().lock();
            if writeln!(out, "{reply}").and_then(|_| out.flush()).is_err() {
                return 0;
            }
        }
    }
}

/// What `rythme mcp --help` (and a terminal) shows.
fn usage() -> String {
    t!(
        "rythme mcp: a Model Context Protocol server for AI assistants, on the standard input and output.\nIts tools run (in a sandbox), check, test, format and fix Rythme code, and explain errors and functions.\nAdd it to Claude Code: claude mcp add rythme -- rythme mcp\nClaude Desktop, Cursor and others: see the manual, section 7.10.",
        "rythme mcp : un serveur Model Context Protocol pour les assistants d'IA, sur l'entrée et la sortie standard.\nSes outils lancent (dans un bac à sable), vérifient, testent, mettent en forme et corrigent du code Rythme, et expliquent les erreurs et les fonctions.\nL'ajouter à Claude Code : claude mcp add rythme -- rythme mcp\nClaude Desktop, Cursor et les autres : voir le manuel, section 7.10."
    )
}

fn error_reply(id: Json, code: i64, message: &str) -> Json {
    obj([("jsonrpc", "2.0".into()), ("id", id), ("error", obj([("code", code.into()), ("message", message.into())]))])
}

fn reply(id: Json, result: Json) -> Json {
    obj([("jsonrpc", "2.0".into()), ("id", id), ("result", result)])
}

struct Server {
    /// The number of the next temporary folder.
    tmp: u64,
}

impl Server {
    /// Handles one message: the reply to a request, nothing for a notification.
    fn handle(&mut self, message: &Json) -> Option<Json> {
        let id = message.get("id").clone();
        let Some(method) = message.get("method").as_str() else {
            // A reply from the client (the server asks nothing): ignored.
            return (!id.is_null() && message.get("result").is_null() && message.get("error").is_null())
                .then(|| error_reply(id, -32600, "invalid request: no method"));
        };
        let notification = id.is_null();
        let params = message.get("params");
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.request(method, params)));
        if notification {
            return None;
        }
        Some(match result {
            Ok(Ok(result)) => reply(id, result),
            Ok(Err((code, msg))) => error_reply(id, code, &msg),
            Err(_) => error_reply(id, -32603, "internal error of Rythme (a bug: please report it)"),
        })
    }

    fn request(&mut self, method: &str, params: &Json) -> Result<Json, (i64, String)> {
        match method {
            "initialize" => {
                let asked = params.get("protocolVersion").as_str().unwrap_or("");
                let version = PROTOCOL_VERSIONS.iter().find(|v| **v == asked).unwrap_or(&PROTOCOL_VERSIONS[0]);
                Ok(obj([
                    ("protocolVersion", (*version).into()),
                    (
                        "capabilities",
                        obj([
                            ("tools", obj([("listChanged", false.into())])),
                            ("resources", obj([("listChanged", false.into()), ("subscribe", false.into())])),
                        ]),
                    ),
                    ("serverInfo", obj([("name", "rythme".into()), ("title", "Rythme".into()), ("version", crate::VERSION.into())])),
                    ("instructions", INSTRUCTIONS.into()),
                ]))
            }
            "ping" => Ok(obj([])),
            "tools/list" => Ok(obj([("tools", Json::Arr(tools()))])),
            "tools/call" => {
                let name = params.get("name").as_str().unwrap_or("");
                let args = params.get("arguments");
                let answer = self.call(name, args).map_err(|e| (-32602, e))?;
                let content: Vec<Json> = answer.texts.into_iter().map(|t| obj([("type", "text".into()), ("text", t.into())])).collect();
                Ok(obj([("content", Json::Arr(content)), ("isError", answer.error.into())]))
            }
            "resources/list" => Ok(obj([("resources", Json::Arr(RESOURCES.iter().map(resource).collect()))])),
            "resources/templates/list" => Ok(obj([("resourceTemplates", Json::Arr(Vec::new()))])),
            "resources/read" => {
                let uri = params.get("uri").as_str().unwrap_or("");
                let r = RESOURCES.iter().find(|r| r.0 == uri).ok_or((-32002, format!("resource not found: {uri}")))?;
                let text = (r.3)();
                Ok(obj([(
                    "contents",
                    Json::Arr(vec![obj([("uri", uri.into()), ("mimeType", "text/plain".into()), ("text", text.into())])]),
                )]))
            }
            "prompts/list" => Ok(obj([("prompts", Json::Arr(Vec::new()))])),
            m if m.starts_with("notifications/") => Ok(Json::Null),
            _ => Err((-32601, format!("method not found: {method}"))),
        }
    }

    /// A folder of its own for the code given to a tool, removed after the call.
    fn scratch(&mut self) -> Result<Scratch, String> {
        self.tmp += 1;
        let dir = std::env::temp_dir().join(format!("rythme-mcp-{}-{}", std::process::id(), self.tmp));
        std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create a temporary folder: {e}"))?;
        Ok(Scratch { dir })
    }

    /// The file a tool works on: `code` written in a temporary `main.ry`, or
    /// an existing `file`. Returns the folder to run in and the file to name.
    fn target(&mut self, args: &Json, keep: &mut Option<Scratch>) -> Result<(PathBuf, String), String> {
        if let Some(code) = args.get("code").as_str() {
            let s = self.scratch()?;
            std::fs::write(s.dir.join("main.ry"), code).map_err(|e| format!("cannot write the code: {e}"))?;
            let dir = s.dir.clone();
            *keep = Some(s);
            return Ok((dir, "main.ry".into()));
        }
        if let Some(file) = args.get("file").or_path() {
            return Ok((std::env::current_dir().unwrap_or_default(), file.to_string()));
        }
        Err("give `code` (Rythme code) or `file` (the path of a .ry file)".into())
    }

    fn call(&mut self, name: &str, args: &Json) -> Result<Answer, String> {
        let mut scratch = None;
        let lang = crate::lang::code();
        let base = |dir: &Path| {
            let mut c = Command::new(std::env::current_exe().unwrap_or_else(|_| PathBuf::from("rythme")));
            c.current_dir(dir).env("RYTHME_SANDBOX", "1").env("NO_COLOR", "1").args(["--lang", lang]);
            c
        };
        let seconds = args.get("timeout_seconds").as_f64().unwrap_or(DEFAULT_SECONDS).clamp(0.1, MAX_SECONDS);
        let answer = match name {
            "run" => {
                let (dir, file) = self.target(args, &mut scratch)?;
                let mut c = base(&dir);
                c.args(["run", "--format", "json", "--sandbox"]);
                if let Some(seed) = args.get("seed").as_i64() {
                    c.args(["--seed", &seed.to_string()]);
                }
                c.arg(&file);
                c.args(args.get("args").as_array().iter().filter_map(Json::as_str));
                let stdin = args.get("stdin").as_str().unwrap_or("");
                json_answer(child(c, stdin, seconds))
            }
            "check" => {
                let (dir, file) = self.target(args, &mut scratch)?;
                let mut c = base(&dir);
                c.args(["check", "--format", "json", &file]);
                json_answer(child(c, "", seconds))
            }
            "test" => {
                let (dir, file) = self.target(args, &mut scratch)?;
                let mut c = base(&dir);
                c.args(["test", "--format", "json", "--sandbox", &file]);
                if let Some(f) = args.get("filter").as_str() {
                    c.args(["--filter", f]);
                }
                json_answer(child(c, "", seconds))
            }
            "fix" => {
                let code = args.get("code").as_str().ok_or("give `code`: the Rythme code to fix")?;
                let s = self.scratch()?;
                std::fs::write(s.dir.join("main.ry"), code).map_err(|e| format!("cannot write the code: {e}"))?;
                let mut c = base(&s.dir);
                c.args(["fix", "--format", "json"]);
                if args.get("all").as_bool() == Some(true) {
                    c.arg("--all");
                }
                c.arg("main.ry");
                let report = child(c, "", seconds);
                let fixed = std::fs::read_to_string(s.dir.join("main.ry")).unwrap_or_default();
                scratch = Some(s);
                let mut a = json_answer(report);
                a.texts.insert(0, fixed);
                a
            }
            "format" => {
                let code = args.get("code").as_str().ok_or("give `code`: the Rythme code to format")?;
                match crate::format::format_named("main.ry", code) {
                    Ok(out) => Answer::ok(out),
                    Err(diags) => Answer::failed(Json::Arr(diags.iter().map(machine::diagnostic_json).collect()).to_string()),
                }
            }
            "explain" => match machine::explain_json(args.get("code").as_str()) {
                Some(doc) => Answer::ok(doc.to_string()),
                None => {
                    let code = args.get("code").as_str().unwrap_or("");
                    Answer::failed(t!(
                        "unknown code `{code}`. Known codes: {}",
                        "code inconnu `{code}`. Codes connus : {}",
                        crate::help::CODES.join(" ")
                    ))
                }
            },
            "help" => {
                // An unknown name is an answer too: `"kind": "unknown"`, with what to do instead.
                Answer::ok(machine::help_json(args.get("name").as_str()).0.to_string())
            }
            "steps" => {
                let (dir, file) = self.target(args, &mut scratch)?;
                let mut c = base(&dir);
                c.args(["steps", &file]);
                if let Some(e) = args.get("expression").as_str() {
                    c.arg(e);
                }
                if let Some(n) = args.get("max_steps").as_u64() {
                    c.args(["--max", &n.to_string()]);
                }
                let r = child(c, "", seconds);
                let mut text = r.stdout.clone();
                if !r.stderr.is_empty() {
                    text.push_str(&r.stderr);
                }
                match r.stopped {
                    Some(why) => Answer::failed(why),
                    None => Answer::ok(text),
                }
            }
            "reference" => Answer::ok(crate::llms::full()),
            _ => return Err(format!("unknown tool `{name}`")),
        };
        drop(scratch);
        Ok(answer)
    }
}

/// A temporary folder, removed when dropped.
struct Scratch {
    dir: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

trait OrPath {
    fn or_path(&self) -> Option<&str>;
}

impl OrPath for Json {
    /// A non-empty text.
    fn or_path(&self) -> Option<&str> {
        self.as_str().filter(|s| !s.trim().is_empty())
    }
}

/// What a child process did.
struct Child {
    stdout: String,
    stderr: String,
    code: Option<i32>,
    /// Why it was stopped (time or memory), if it was.
    stopped: Option<String>,
}

/// Runs `rythme` as a child, with `stdin` as its input, stopped after
/// `seconds` or (on Linux) when it takes more than [`MAX_MEMORY`].
fn child(mut c: Command, stdin: &str, seconds: f64) -> Child {
    c.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut proc = match c.spawn() {
        Ok(p) => p,
        Err(e) => {
            return Child { stdout: String::new(), stderr: String::new(), code: None, stopped: Some(format!("cannot start rythme: {e}")) }
        }
    };
    let input = stdin.to_string();
    let mut pipe_in = proc.stdin.take();
    let feeder = std::thread::spawn(move || {
        if let Some(p) = pipe_in.as_mut() {
            let _ = p.write_all(input.as_bytes());
        }
    });
    let read = |pipe: Option<Box<dyn Read + Send>>| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(mut p) = pipe {
                // At most 16 MB of output are kept.
                let _ = p.by_ref().take(16 << 20).read_to_end(&mut buf);
                let _ = std::io::copy(&mut p, &mut std::io::sink());
            }
            String::from_utf8_lossy(&buf).into_owned()
        })
    };
    let out = read(proc.stdout.take().map(|p| Box::new(p) as Box<dyn Read + Send>));
    let err = read(proc.stderr.take().map(|p| Box::new(p) as Box<dyn Read + Send>));
    let start = Instant::now();
    let mut stopped = None;
    let code = loop {
        match proc.try_wait() {
            Ok(Some(status)) => break status.code(),
            Ok(None) => {}
            Err(_) => break None,
        }
        if start.elapsed().as_secs_f64() > seconds {
            stopped = Some(t!(
                "the program was stopped after {seconds} seconds: it may never end (a recursion without a base case?)",
                "le programme a été arrêté après {seconds} secondes : il ne finit peut-être jamais (une récursion sans cas de base ?)"
            ));
        } else if memory(proc.id()).is_some_and(|m| m > MAX_MEMORY) {
            let mb = MAX_MEMORY >> 20;
            stopped = Some(t!(
                "the program was stopped: it took more than {mb} MB of memory",
                "le programme a été arrêté : il prenait plus de {mb} Mo de mémoire"
            ));
        }
        if stopped.is_some() {
            let _ = proc.kill();
            let _ = proc.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let _ = feeder.join();
    Child { stdout: out.join().unwrap_or_default(), stderr: err.join().unwrap_or_default(), code, stopped }
}

/// The memory a process takes, in bytes, where the system tells (Linux).
fn memory(pid: u32) -> Option<u64> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    let line = status.lines().find(|l| l.starts_with("VmRSS:"))?;
    let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb * 1024)
}

/// The answer of a tool that prints one JSON document: the document, an
/// error when the command failed (exit code other than 0).
fn json_answer(r: Child) -> Answer {
    if let Some(why) = r.stopped {
        return Answer::failed(why);
    }
    let doc = r.stdout.trim();
    // The command did its work: whether the code is right is in the document (`ok`).
    if doc.starts_with('{') {
        return Answer::ok(doc);
    }
    let text = format!("{}{}", r.stdout, r.stderr).trim().to_string();
    Answer { texts: vec![text], error: r.code != Some(0) }
}

/// What the server tells the assistant when it connects.
const INSTRUCTIONS: &str = "Rythme is a small, purely functional language that you probably do not know well: read the resource rythme://docs/llms-full.txt (or call the tool `reference`) before writing Rythme. Then work in a loop: `check` the code, apply its fixes (or call `fix`), `test` it, `run` it, until there are no errors. Every diagnostic has a stable code (`explain` it), a range and a help that names the fix. `help` documents a built-in function. Programs run in a sandbox: no files, folders or environment variables.";

/// The resources: uri, name, description, text.
type Resource = (&'static str, &'static str, &'static str, fn() -> String);

const RESOURCES: &[Resource] = &[
    (
        "rythme://docs/llms-full.txt",
        "llms-full.txt",
        "The whole Rythme language in one text, for AI assistants: rules, syntax by example, every built-in function, habits from other languages, error codes.",
        crate::llms::full,
    ),
    ("rythme://docs/llms.txt", "llms.txt", "What Rythme is, and where to read more (llmstxt.org index).", crate::llms::index),
];

fn resource(r: &Resource) -> Json {
    obj([("uri", r.0.into()), ("name", r.1.into()), ("description", r.2.into()), ("mimeType", "text/plain".into())])
}

/// A property of the input of a tool.
fn prop(kind: &str, description: &str) -> Json {
    obj([("type", kind.into()), ("description", description.into())])
}

fn tool(name: &str, title: &str, description: &str, props: Vec<(&str, Json)>, required: &[&str]) -> Json {
    let properties = Json::Obj(props.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    obj([
        ("name", name.into()),
        ("title", title.into()),
        ("description", description.into()),
        (
            "inputSchema",
            obj([
                ("type", "object".into()),
                ("properties", properties),
                ("required", Json::Arr(required.iter().map(|r| (*r).into()).collect())),
            ]),
        ),
    ])
}

const CODE: &str = "Rythme code (a whole program, as the content of a .ry file)";
const FILE: &str = "or: the path of a .ry file, from the folder where the server runs";

fn tools() -> Vec<Json> {
    let timeout = || prop("number", "the time limit in seconds (10 by default, 60 at most)");
    vec![
        tool(
            "run",
            "Run Rythme code",
            "Checks then runs a Rythme program in a sandbox (no files, folders or environment variables; time and memory limited). Returns one JSON document: stdout, stderr, exit_code and the diagnostics (code, message, range, help, fixes).",
            vec![
                ("code", prop("string", CODE)),
                ("file", prop("string", FILE)),
                ("stdin", prop("string", "what the program reads with input() and input_lines()")),
                ("args", obj([("type", "array".into()), ("items", obj([("type", "string".into())])), ("description", "what args() gives".into())])),
                ("seed", prop("integer", "the seed of random and shuffle, for the same draws every time")),
                ("timeout_seconds", timeout()),
            ],
            &[],
        ),
        tool(
            "check",
            "Check Rythme code",
            "Finds every error and warning without running the code. Returns one JSON document whose diagnostics have a stable code, a message, a range (lines and columns from 1), labels, help that names the fix in code, and fixes as text edits (safe ones can be applied as they are).",
            vec![("code", prop("string", CODE)), ("file", prop("string", FILE)), ("timeout_seconds", timeout())],
            &[],
        ),
        tool(
            "test",
            "Run the tests of Rythme code",
            "Runs the tests (`test \"name\" = ...`) of the code, a file or a folder, in the sandbox. Returns one JSON document: each test with passed and its failure (expected and actual values in the notes).",
            vec![
                ("code", prop("string", CODE)),
                ("file", prop("string", "or: the path of a .ry file or of a folder with tests")),
                ("filter", prop("string", "only the tests whose name contains this text")),
                ("timeout_seconds", timeout()),
            ],
            &[],
        ),
        tool(
            "fix",
            "Fix Rythme code",
            "Applies the safe fixes of the diagnostics (`&&` to `and`, `a ? b : c` to `if`, `def` to `fn`, `return` removed, a missing `then`...) until none is left. Returns the fixed code, then a JSON report of the fixes applied and of the diagnostics left.",
            vec![("code", prop("string", CODE)), ("all", prop("boolean", "also apply the guesses, such as misspelt names")), ("timeout_seconds", timeout())],
            &["code"],
        ),
        tool(
            "format",
            "Format Rythme code",
            "Gives Rythme code its official layout (as `rythme fmt`). Returns the formatted code, or the syntax errors as JSON.",
            vec![("code", prop("string", CODE))],
            &["code"],
        ),
        tool(
            "explain",
            "Explain an error code",
            "The long explanation of an error or warning code (E001...E027, W001...W011), with wrong and right examples. Without a code: the list of all codes with their titles.",
            vec![("code", prop("string", "the code, like E005"))],
            &[],
        ),
        tool(
            "help",
            "Look up a built-in function",
            "The documentation of a built-in function (signature, doc, example, Ruby names), a keyword, a code or a help topic, as JSON. Without a name: every built-in function, keyword, topic and code.",
            vec![("name", prop("string", "a function (map), a keyword (match), a code (E003) or a topic (lists)"))],
            &[],
        ),
        tool(
            "steps",
            "Show an evaluation step by step",
            "Shows how Rythme reduces an expression step by step (fib(3) -> fib(2) + fib(1) -> ...): the expression given, or the last line of the code.",
            vec![
                ("code", prop("string", CODE)),
                ("file", prop("string", FILE)),
                ("expression", prop("string", "the expression to show, using the definitions of the code")),
                ("max_steps", prop("integer", "the number of steps shown at most")),
                ("timeout_seconds", timeout()),
            ],
            &[],
        ),
        tool(
            "reference",
            "Read the Rythme reference",
            "The whole Rythme language in one text (the same as the resource rythme://docs/llms-full.txt): read it before writing Rythme.",
            Vec::new(),
            &[],
        ),
    ]
}
