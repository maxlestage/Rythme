//! `rythme dap`: the debugger for editors, with the Debug Adapter Protocol
//! on the standard input and output (VS Code starts it for the debugger
//! type `rythme`). Messages are JSON with a `Content-Length` header, as for
//! the language server, whose reader and JSON this module uses.
//!
//! The requests: initialize, launch, setBreakpoints, setFunctionBreakpoints,
//! setExceptionBreakpoints, configurationDone, threads, stackTrace, scopes,
//! variables (lists, records, variants and dictionaries open), evaluate,
//! continue, next, stepIn, stepOut, pause, disconnect and terminate. The
//! program is the only thread. Its output becomes `output` events, and it
//! reads no input: the standard input carries the protocol.

use std::cell::{Cell, RefCell};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, TryRecvError};

use super::{line_of, Breakpoints, FrameView, Frontend, Outcome, Paused, Program, Reason, Resume, Session};
use crate::lsp::json::{obj, Json};
use crate::source;
use crate::value::Value;

/// The children shown for a value at most (the first ones of a long list).
const MAX_CHILDREN: usize = 1000;

/// The connection with the editor.
struct Conn {
    rx: Receiver<Option<Json>>,
    seq: Cell<i64>,
    /// Lines and columns start at 1 (the default), else at 0.
    lines_at_1: Cell<bool>,
    columns_at_1: Cell<bool>,
}

impl Conn {
    fn send(&self, mut msg: Json) {
        let seq = self.seq.get() + 1;
        self.seq.set(seq);
        msg.set("seq", Json::from(seq));
        let body = msg.to_string();
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        let _ = write!(out, "Content-Length: {}\r\n\r\n{body}", body.len());
        let _ = out.flush();
    }

    fn event(&self, event: &str, body: Json) {
        self.send(obj([("type", "event".into()), ("event", event.into()), ("body", body)]));
    }

    fn respond(&self, req: &Json, body: Json) {
        let mut msg = obj([
            ("type", "response".into()),
            ("request_seq", req.get("seq").clone()),
            ("success", true.into()),
            ("command", req.get("command").clone()),
        ]);
        if !body.is_null() {
            msg.set("body", body);
        }
        self.send(msg);
    }

    fn fail(&self, req: &Json, message: &str) {
        self.send(obj([
            ("type", "response".into()),
            ("request_seq", req.get("seq").clone()),
            ("success", false.into()),
            ("command", req.get("command").clone()),
            ("message", message.into()),
            ("body", obj([("error", obj([("id", Json::from(1i64)), ("format", message.into()), ("showUser", true.into())]))])),
        ]));
    }

    fn output(&self, text: &str, category: &str) {
        self.event("output", obj([("category", category.into()), ("output", text.into())]));
    }

    /// The next request: `None` when the editor went away.
    fn next(&self) -> Option<Json> {
        self.rx.recv().ok().flatten()
    }

    fn line_out(&self, line: usize) -> Json {
        Json::from(if self.lines_at_1.get() { line } else { line.saturating_sub(1) })
    }

    fn line_in(&self, v: &Json) -> usize {
        let n = v.as_u64().unwrap_or(1) as usize;
        if self.lines_at_1.get() {
            n
        } else {
            n + 1
        }
    }

    fn column_out(&self, column: usize) -> Json {
        Json::from(if self.columns_at_1.get() { column } else { column.saturating_sub(1) })
    }
}

fn command_of(req: &Json) -> &str {
    req.get("command").as_str().unwrap_or("")
}

/// The path of a file of the program, as the editor knows it.
fn path_of(file: u32) -> Option<PathBuf> {
    let f = source::get(file)?;
    let p = Path::new(&f.name);
    Some(p.canonicalize().unwrap_or_else(|_| p.to_path_buf()))
}

fn source_json(file: u32) -> Json {
    match path_of(file) {
        Some(p) => {
            let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            obj([("name", name.into()), ("path", p.display().to_string().into())])
        }
        None => Json::Null,
    }
}

/// Answers the requests that do not need a paused program; `false` for the others.
fn common(conn: &Conn, req: &Json, bps: &mut Breakpoints) -> bool {
    let args = req.get("arguments");
    match command_of(req) {
        "setBreakpoints" => {
            let path = args.get_path(&["source", "path"]).as_str().unwrap_or("");
            let file = bps.file_of_path(Path::new(path));
            let wanted: Vec<usize> = args.get("breakpoints").as_array().iter().map(|b| conn.line_in(b.get("line"))).collect();
            let mut out = Vec::new();
            match file {
                Some(file) => {
                    bps.clear_file(file);
                    for line in wanted {
                        out.push(match bps.add_line(file, line) {
                            Ok((id, at)) => obj([("id", Json::from(id)), ("verified", true.into()), ("line", conn.line_out(at))]),
                            Err(e) => obj([("verified", false.into()), ("line", conn.line_out(line)), ("message", e.into())]),
                        });
                    }
                }
                None => {
                    let msg =
                        t!("this file is not part of the program being debugged", "ce fichier ne fait pas partie du programme débogué");
                    for line in wanted {
                        out.push(obj([("verified", false.into()), ("line", conn.line_out(line)), ("message", msg.clone().into())]));
                    }
                }
            }
            conn.respond(req, obj([("breakpoints", Json::Arr(out))]));
        }
        "setFunctionBreakpoints" => {
            bps.clear_functions();
            let mut out = Vec::new();
            for b in args.get("breakpoints").as_array() {
                let name = b.get("name").as_str().unwrap_or("").trim();
                out.push(match bps.add_function(name) {
                    Ok(id) => obj([("id", Json::from(id)), ("verified", true.into())]),
                    Err(e) => obj([("verified", false.into()), ("message", e.into())]),
                });
            }
            conn.respond(req, obj([("breakpoints", Json::Arr(out))]));
        }
        "setExceptionBreakpoints" => conn.respond(req, obj([("breakpoints", Json::Arr(Vec::new()))])),
        "threads" => conn.respond(req, obj([("threads", Json::Arr(vec![obj([("id", Json::from(1i64)), ("name", "main".into())])]))])),
        "disconnect" | "terminate" => {
            conn.respond(req, Json::Null);
            if command_of(req) == "terminate" {
                conn.event("terminated", obj([]));
            }
            std::process::exit(0);
        }
        _ => return false,
    }
    true
}

/// What a `variablesReference` stands for, while the program is paused.
enum Var {
    Locals(usize),
    Globals,
    Value(Value),
}

/// The children of a value: the elements of a list, the fields of a record...
fn children(v: &Value) -> Vec<(String, Value)> {
    match v {
        Value::List(l) => l.iter().take(MAX_CHILDREN).enumerate().map(|(i, x)| (format!("[{i}]"), x.clone())).collect(),
        Value::Record(r) => r.iter().map(|(k, x)| (k.to_string(), x.clone())).collect(),
        Value::Variant(var) => var.ctor.fields.iter().zip(&var.fields).map(|(k, x)| (k.name().to_string(), x.clone())).collect(),
        Value::Dict(m) => m.iter().take(MAX_CHILDREN).map(|(k, x)| (k.repr(), x.clone())).collect(),
        Value::Set(m) => m.keys().take(MAX_CHILDREN).enumerate().map(|(i, x)| (format!("[{i}]"), x.clone())).collect(),
        _ => Vec::new(),
    }
}

/// A value, cut if it is very long.
fn shown(v: &Value) -> String {
    let s = v.repr();
    if s.chars().count() > 2000 {
        format!("{}...", s.chars().take(1997).collect::<String>())
    } else {
        s
    }
}

fn frame_name(f: &FrameView) -> String {
    if f.name.is_empty() && !f.builtin {
        return t!("top level", "niveau principal");
    }
    let args: Vec<String> = f.args.iter().map(|(n, v)| if n.is_empty() { shown(v) } else { format!("{n} = {}", shown(v)) }).collect();
    let mut name = format!("{}({})", f.name, args.join(", "));
    if let Some(by) = &f.replaced {
        name.push_str(&t!(" [tail call, replaced {by}]", " [appel terminal, a remplacé {by}]"));
    }
    name
}

/// The editor while the program runs.
struct Editor {
    conn: Rc<Conn>,
    vars: Vec<Var>,
}

impl Editor {
    fn var_ref(&mut self, v: &Value) -> usize {
        if children(v).is_empty() {
            return 0;
        }
        self.vars.push(Var::Value(v.clone()));
        self.vars.len()
    }

    fn variable(&mut self, name: &str, v: &Value) -> Json {
        let reference = self.var_ref(v);
        obj([
            ("name", name.into()),
            ("value", shown(v).into()),
            ("type", v.type_name().into()),
            ("variablesReference", Json::from(reference)),
        ])
    }

    fn stopped(&self, p: &Paused) {
        let (reason, description, text) = match &p.reason {
            Reason::Entry => ("entry", String::new(), None),
            Reason::Step => ("step", String::new(), None),
            Reason::Pause => ("pause", String::new(), None),
            Reason::Breakpoint(_) => ("breakpoint", String::new(), None),
            Reason::Function(_) => ("function breakpoint", String::new(), None),
            Reason::Returned { name, value } => ("step", t!("`{name}` gave {}", "`{name}` a donné {}", shown(value)), None),
            Reason::Error(d) => ("exception", t!("error: {}", "erreur : {}", d.message), Some(d.message.clone())),
        };
        let mut body = obj([("reason", reason.into()), ("threadId", Json::from(1i64)), ("allThreadsStopped", true.into())]);
        if !description.is_empty() {
            body.set("description", description.clone().into());
            self.conn.output(&format!("{description}\n"), "console");
        }
        if let Some(text) = text {
            body.set("text", text.into());
        }
        if let Reason::Breakpoint(ids) = &p.reason {
            body.set("hitBreakpointIds", Json::Arr(ids.iter().map(|id| Json::from(*id)).collect()));
        }
        self.conn.event("stopped", body);
    }

    /// Answers a request while the program is paused: `Some` to go on.
    fn paused_request(&mut self, p: &mut Paused, req: &Json) -> Option<Resume> {
        let conn = self.conn.clone();
        let args = req.get("arguments");
        let frames = p.frames();
        let frame_arg = |name: &str| args.get(name).as_u64().map(|n| n as usize).filter(|&i| i < frames.len());
        match command_of(req) {
            "continue" => {
                conn.respond(req, obj([("allThreadsContinued", true.into())]));
                return Some(Resume::Continue);
            }
            "next" => {
                conn.respond(req, Json::Null);
                return Some(Resume::Next);
            }
            "stepIn" => {
                conn.respond(req, Json::Null);
                return Some(Resume::Step);
            }
            "stepOut" => {
                conn.respond(req, Json::Null);
                return Some(Resume::Finish);
            }
            "pause" => conn.respond(req, Json::Null),
            "stackTrace" => {
                let mut out = Vec::new();
                for (i, f) in frames.iter().enumerate() {
                    let mut frame = obj([("id", Json::from(i)), ("name", frame_name(f).into())]);
                    match f.at {
                        Some(at) => {
                            let (file, line) = line_of(at);
                            let column = source::get(file).map_or(1, |s| s.line_col(at.start as usize).1);
                            frame.set("source", source_json(file));
                            frame.set("line", conn.line_out(line));
                            frame.set("column", conn.column_out(column));
                        }
                        None => {
                            frame.set("line", Json::from(0usize));
                            frame.set("column", Json::from(0usize));
                        }
                    }
                    if f.builtin {
                        frame.set("presentationHint", "subtle".into());
                    }
                    out.push(frame);
                }
                let total = out.len();
                conn.respond(req, obj([("stackFrames", Json::Arr(out)), ("totalFrames", Json::from(total))]));
            }
            "scopes" => {
                let i = frame_arg("frameId").unwrap_or(0);
                self.vars.push(Var::Locals(i));
                let locals = self.vars.len();
                self.vars.push(Var::Globals);
                let globals = self.vars.len();
                conn.respond(
                    req,
                    obj([(
                        "scopes",
                        Json::Arr(vec![
                            obj([
                                ("name", t!("Locals", "Locales").into()),
                                ("presentationHint", "locals".into()),
                                ("variablesReference", Json::from(locals)),
                                ("expensive", false.into()),
                            ]),
                            obj([
                                ("name", t!("Globals", "Globales").into()),
                                ("variablesReference", Json::from(globals)),
                                ("expensive", false.into()),
                            ]),
                        ]),
                    )]),
                );
            }
            "variables" => {
                let reference = args.get("variablesReference").as_u64().unwrap_or(0) as usize;
                let list: Vec<(String, Value)> = match reference.checked_sub(1).and_then(|i| self.vars.get(i)) {
                    Some(Var::Locals(i)) => {
                        let mut list = Vec::new();
                        if let (0, Reason::Returned { name, value }) = (*i, &p.reason) {
                            list.push((t!("result of {name}", "résultat de {name}"), value.clone()));
                        }
                        list.extend(p.locals(*i));
                        list
                    }
                    Some(Var::Globals) => p.globals(),
                    Some(Var::Value(v)) => children(v),
                    None => Vec::new(),
                };
                let vars: Vec<Json> = list.iter().map(|(n, v)| self.variable(n, v)).collect();
                conn.respond(req, obj([("variables", Json::Arr(vars))]));
            }
            "evaluate" => {
                let i = frame_arg("frameId").unwrap_or(0);
                let expr = args.get("expression").as_str().unwrap_or("").trim();
                match p.eval(i, expr) {
                    Ok(v) => {
                        let reference = self.var_ref(&v);
                        conn.respond(
                            req,
                            obj([
                                ("result", shown(&v).into()),
                                ("type", v.type_name().into()),
                                ("variablesReference", Json::from(reference)),
                            ]),
                        );
                    }
                    Err(e) => conn.fail(req, &e),
                }
            }
            "source" => conn
                .fail(req, &t!("no source to give: the files are on the disk", "pas de source à donner : les fichiers sont sur le disque")),
            _ => {
                if !common(&conn, req, p.breakpoints()) {
                    let cmd = command_of(req);
                    conn.fail(req, &t!("unknown request `{cmd}`", "requête inconnue `{cmd}`"));
                }
            }
        }
        None
    }
}

impl Frontend for Editor {
    fn paused(&mut self, p: &mut Paused) -> Resume {
        self.vars.clear();
        self.stopped(p);
        loop {
            let Some(req) = self.conn.next() else { std::process::exit(0) };
            if let Some(r) = self.paused_request(p, &req) {
                self.vars.clear();
                return r;
            }
        }
    }

    fn interrupt(&mut self, bps: &mut Breakpoints) -> bool {
        loop {
            let req = match self.conn.rx.try_recv() {
                Ok(Some(req)) => req,
                Ok(None) | Err(TryRecvError::Disconnected) => std::process::exit(0),
                Err(TryRecvError::Empty) => return false,
            };
            if command_of(&req) == "pause" {
                self.conn.respond(&req, Json::Null);
                return true;
            }
            if !common(&self.conn, &req, bps) {
                self.conn.fail(&req, &t!("the program is running: pause it first", "le programme tourne : mettez-le d'abord en pause"));
            }
        }
    }
}

/// `rythme dap`: runs the adapter until the editor disconnects.
pub fn command(_args: &[String]) -> i32 {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut input = stdin.lock();
        loop {
            match crate::lsp::read_message(&mut input) {
                Ok(Some(body)) => {
                    if let Ok(json) = Json::parse(&String::from_utf8_lossy(&body)) {
                        if tx.send(Some(json)).is_err() {
                            return;
                        }
                    }
                }
                Ok(None) => {
                    let _ = tx.send(None);
                    return;
                }
                Err(e) => eprintln!("rythme dap: {e}"),
            }
        }
    });
    let conn = Rc::new(Conn { rx, seq: Cell::new(0), lines_at_1: Cell::new(true), columns_at_1: Cell::new(true) });
    crate::io::close_input();
    let sink_conn = conn.clone();
    crate::io::set_sink(Box::new(move |text, err| sink_conn.output(text, if err { "stderr" } else { "stdout" })));

    // Until `launch`, then until `configurationDone`.
    let program: RefCell<Option<Program>> = RefCell::new(None);
    let mut bps = Breakpoints::default();
    let mut stop_on_entry = false;
    loop {
        let Some(req) = conn.next() else { return 0 };
        let args = req.get("arguments");
        match command_of(&req) {
            "initialize" => {
                conn.lines_at_1.set(args.get("linesStartAt1").as_bool().unwrap_or(true));
                conn.columns_at_1.set(args.get("columnsStartAt1").as_bool().unwrap_or(true));
                conn.respond(
                    &req,
                    obj([
                        ("supportsConfigurationDoneRequest", true.into()),
                        ("supportsFunctionBreakpoints", true.into()),
                        ("supportsEvaluateForHovers", true.into()),
                        ("supportsTerminateRequest", true.into()),
                    ]),
                );
            }
            "launch" => {
                if let Some(cwd) = args.get("cwd").as_str().filter(|c| !c.is_empty()) {
                    let _ = std::env::set_current_dir(cwd);
                }
                let Some(path) = args.get("program").as_str().filter(|p| !p.is_empty()) else {
                    conn.fail(
                        &req,
                        &t!(
                            "which program? Give its path in `program`, like \"${{file}}\"",
                            "quel programme ? Donnez son chemin dans `program`, comme \"${{file}}\""
                        ),
                    );
                    continue;
                };
                stop_on_entry = args.get("stopOnEntry").as_bool().unwrap_or(false);
                let script_args: Vec<String> = args.get("args").as_array().iter().filter_map(|a| a.as_str().map(String::from)).collect();
                match Program::load(Path::new(path), script_args, false) {
                    Some(p) => {
                        bps.learn(&p.modules());
                        *program.borrow_mut() = Some(p);
                        conn.respond(&req, Json::Null);
                        conn.event("initialized", obj([]));
                    }
                    None => {
                        conn.fail(
                            &req,
                            &t!(
                                "the program has errors: see the debug console",
                                "le programme a des erreurs : voir la console de débogage"
                            ),
                        );
                        conn.event("exited", obj([("exitCode", Json::from(1i64))]));
                        conn.event("terminated", obj([]));
                    }
                }
            }
            "configurationDone" => {
                conn.respond(&req, Json::Null);
                if program.borrow().is_some() {
                    break;
                }
            }
            _ => {
                if !common(&conn, &req, &mut bps) {
                    let cmd = command_of(&req);
                    conn.fail(
                        &req,
                        &t!(
                            "the program is not running: `{cmd}` has nothing to show",
                            "le programme n'est pas lancé : `{cmd}` n'a rien à montrer"
                        ),
                    );
                }
            }
        }
    }

    let mut program = program.into_inner().expect("a program");
    let editor = Editor { conn: conn.clone(), vars: Vec::new() };
    let session = Session::new(Box::new(editor), bps, Resume::Continue, stop_on_entry);
    let (outcome, mut session) = program.run(session);
    let code = if outcome == Outcome::Finished { 0i64 } else { 1 };
    conn.event("exited", obj([("exitCode", Json::from(code))]));
    conn.event("terminated", obj([]));
    loop {
        let Some(req) = conn.next() else { return 0 };
        if !common(&conn, &req, &mut session.breakpoints) {
            let cmd = command_of(&req);
            conn.fail(
                &req,
                &t!("the program has ended: `{cmd}` has nothing to show", "le programme est terminé : `{cmd}` n'a rien à montrer"),
            );
        }
    }
}
