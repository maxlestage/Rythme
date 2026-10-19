//! The debugger: `rythme debug` driven by scripts of commands (as typed in a
//! terminal), `rythme dap` driven by the Debug Adapter Protocol, and every
//! example run step by step under the debugger, which must change nothing.

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::rc::Rc;

use rythme::debug::{Breakpoints, Frontend, Paused, Program, Resume, Session};
use rythme::lsp::json::{obj, Json};

/// A new folder with these files.
fn folder(files: &[(&str, &str)]) -> PathBuf {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("rythme-debug-{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for (name, text) in files {
        std::fs::write(dir.join(name), text).unwrap();
    }
    dir
}

/// Runs `rythme debug main.ry` in a folder with these files, typing `input`:
/// what it printed (output and errors together, in order) and its exit code.
fn session(lang: &str, files: &[(&str, &str)], input: &str) -> (String, i32) {
    let dir = folder(files);
    std::fs::write(dir.join("input.txt"), input).unwrap();
    let out = std::fs::File::create(dir.join("output.txt")).unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["debug", "main.ry"])
        .current_dir(&dir)
        .env("RYTHME_LANG", lang)
        .env("NO_COLOR", "1")
        .stdin(std::fs::File::open(dir.join("input.txt")).unwrap())
        .stdout(out.try_clone().unwrap())
        .stderr(out)
        .status()
        .unwrap();
    let printed = std::fs::read_to_string(dir.join("output.txt")).unwrap().replace("\r\n", "\n");
    let _ = std::fs::remove_dir_all(&dir);
    (printed, status.code().unwrap_or(-1))
}

const FACT: &str = "fn fact(0) = 1
fn fact(n) = n * fact(n - 1)

fn total([], acc) = acc
fn total([x, ..rest], acc) = total(rest, acc + x)

print(fact(3))
print(total([1, 2, 3], 0))
";

#[track_caller]
fn has(text: &str, wanted: &[&str]) {
    for w in wanted {
        assert!(text.contains(w), "missing {w:?} in:\n{text}");
    }
}

#[test]
fn breakpoints_where_print_finish_and_tail_calls() {
    let (out, code) = session(
        "en",
        &[("main.ry", FACT)],
        "break 2\nrun\ncontinue\nwhere\nlocals\nprint n * 10\nprint fact(n) + 1\ndelete 1\nfinish\nbreak total\ncontinue\nnext\nwhere\ncontinue\n\n\nquit\n",
    );
    assert_eq!(code, 0, "{out}");
    has(
        &out,
        &[
            "Debugging main.ry.",
            "(debug) break 2\nbreakpoint 1 at main.ry:2\n",
            "● breakpoint 1\n→ main.ry:2  fact(n = 3) · depth 1\n  2 | fn fact(n) = n * fact(n - 1)\n",
            "→ main.ry:2  fact(n = 2) · depth 2 (2 calls of `fact` in progress)",
            "(debug) where\n→ #0 fact(n = 2)  main.ry:2\n  #1 fact(n = 3)  main.ry:2\n  #2 top level  main.ry:7\n`fact` calls itself: 2 calls in progress\n",
            "(debug) locals\n  n = 2\n",
            "(debug) print n * 10\n20\n",
            "(debug) print fact(n) + 1\n3\n",
            // `finish`: the result, where the function was called.
            "(debug) finish\n`fact` gave 2\n→ main.ry:2  fact(n = 3) · depth 1\n",
            "breakpoint 2 at the start of `total`",
            "6\n● breakpoint 2: `total` starts\n→ main.ry:5  total(arg1 = [1, 2, 3], acc = 0) · depth 1\n",
            // A call in tail position takes the place of its caller.
            "(debug) next\n● breakpoint 2: `total` starts\n→ main.ry:5  total(arg1 = [2, 3], acc = 1) · depth 1\n  (a call in tail position: it took the place of the frame of `total`)\n",
            "(debug) where\n→ #0 total(arg1 = [2, 3], acc = 1)  main.ry:5 (replaced `total`: tail call)\n  #1 top level  main.ry:8\n",
            // An empty line repeats `continue`.
            "total(arg1 = [], acc = 6)",
            "6\nThe program finished. `run` starts it again, `quit` leaves.\n(debug) quit\ndebugger stopped\n",
        ],
    );
}

#[test]
fn step_into_modules_local_functions_and_blocks() {
    let geometry = "## The area of a square.\nfn area(side)\n  squared = side * side\n  squared\nend\n";
    let main = "use geometry

fn evens(xs)
  fn even(0) = true
  fn even(n) = odd(n - 1)
  fn odd(0) = false
  fn odd(n) = even(n - 1)
  kept = xs |> filter(even)
  {count: len(kept), area: geometry.area(len(kept))}
end

r = evens([1, 2])
print(r)
";
    let (out, code) = session(
        "en",
        &[("main.ry", main), ("geometry.ry", geometry)],
        "step\nstep\nstep\nlocals\nstep\nwhere\nbreak geometry.ry:4\nbreak geometry.ry:9\ncontinue\nlocals\nprint squared + side\nlist\nfinish\nnext\nlocals\ncontinue\nquit\n",
    );
    assert_eq!(code, 0, "{out}");
    has(
        &out,
        &[
            "(debug) step\n→ main.ry:12  top level\n  12 | r = evens([1, 2])\n",
            "(debug) step\n→ main.ry:8  evens(xs = [1, 2]) · depth 1\n  8 |   kept = xs |> filter(even)\n",
            "(debug) step\n→ main.ry:5  even(n = 1) · depth 2\n",
            "(debug) locals\n  n = 1\n",
            "→ main.ry:6  odd(n = 0) · depth 2\n  (a call in tail position: it took the place of the frame of `even`)\n",
            "  #1 filter([1, 2], <fn even>)  main.ry:8\n  #2 evens(xs = [1, 2])  main.ry:8\n  #3 top level  main.ry:12\n",
            "breakpoint 1 at geometry.ry:4",
            "no code where the program can pause on line 9 of geometry.ry or after it",
            "● breakpoint 1\n→ geometry.ry:4  area(side = 1) · depth 2\n",
            "(debug) locals\n  side = 1\n  squared = 1\n",
            "(debug) print squared + side\n2\n",
            "  3 |   squared = side * side\n> 4 |   squared\n  5 | end\n",
            "(debug) finish\n`area` gave 1\n→ main.ry:9  evens(xs = [1, 2]) · depth 1\n",
            "(debug) next\n→ main.ry:13  top level\n",
            "(debug) locals\n  r = {count: 1, area: 1}\n",
            "{count: 1, area: 1}\nThe program finished.",
        ],
    );
}

#[test]
fn an_error_pauses_where_it_happens() {
    let main = "fn ratio(a, b)\n  half = b // 2\n  a / (b - half * 2)\nend\nprint(ratio(10, 4))\n";
    let (out, code) = session("en", &[("main.ry", main)], "run\nwhere\nlocals\nprint b % 2\ncontinue\nquit\n");
    assert_eq!(code, 0, "{out}");
    has(
        &out,
        &[
            "(debug) run\n✗ the program stops on an error: division by zero\n",
            "→ main.ry:3  ratio(a = 10, b = 4) · depth 1\n",
            "(debug) where\n→ #0 ratio(a = 10, b = 4)  main.ry:3\n  #1 top level  main.ry:5\n",
            "(debug) locals\n  a = 10\n  b = 4\n  half = 2\n",
            "(debug) print b % 2\n0\n",
            // The whole message comes once, when the program goes on.
            "(debug) continue\nerror[E008]: division by zero\n",
            "The program stopped on an error. `run` starts it again, `quit` leaves.",
        ],
    );
    assert_eq!(out.matches("error[E008]").count(), 1, "{out}");
}

#[test]
fn print_only_computes_values() {
    let main = "fn f(x) = x + 1\nprint(f(1))\n";
    let (out, _) = session(
        "en",
        &[("main.ry", main)],
        "break f\nrun\nprint print(x)\nprint write_file(\"a.txt\", \"x\")\nprint random()\nprint y\nprint x +\nprint\nx = 2\nprint x\nquit\n",
    );
    has(
        &out,
        &[
            "`print` has an effect: the debugger only computes values, without changing anything",
            "`write_file` has an effect",
            "`random` has an effect",
            "unknown name `y`",
            "(debug) print\nprint what? An expression, like `print n` or `print len(xs) * 2`\n",
            "unknown command `x`",
            "(debug) print x\n1\n",
        ],
    );
}

#[test]
fn commands_explain_themselves() {
    let (out, _) = session(
        "en",
        &[("main.ry", FACT)],
        "where\nbreak\nbreak 99\nbreak fcat\nbreak other.ry:1\ndelete 5\nbreakpoints\nnxet\nhelp\nbreak 1\nbreakpoints\nrun\nrun\nquit\n",
    );
    has(
        &out,
        &[
            "(debug) where\nthe program is not running: `run` starts it (`step` pauses at its first line)\n",
            "break where? A line (`break 12`)",
            "no code where the program can pause on line 99 of main.ry or after it",
            "no function named `fcat` in the program: did you mean `fact`?",
            "no file `other.ry` in the program (its files: `main.ry`)",
            "there is no breakpoint 5 (see `breakpoints`)",
            "no breakpoints: add one with `break 12` or `break fact`",
            "unknown command `nxet`: did you mean `next`? (`help` lists the commands)",
            "  finish           go on until this function gives its result (f)",
            "(debug) breakpoints\n  1  main.ry:1\n",
            "the program is already running: continue goes on, quit stops it",
        ],
    );
}

#[test]
fn run_again_after_the_end() {
    let (out, code) = session("en", &[("main.ry", FACT)], "run\nbreak total\nrun\ndelete\ncontinue\n");
    assert_eq!(code, 0, "{out}");
    assert_eq!(out.matches("The program finished.").count(), 2, "{out}");
    has(&out, &["● breakpoint 1: `total` starts"]);
    // The end of the input quits.
    assert!(out.ends_with("debugger stopped\n"), "{out}");
}

#[test]
fn usage_and_missing_files() {
    let r = Command::new(env!("CARGO_BIN_EXE_rythme")).args(["debug"]).env("RYTHME_LANG", "en").output().unwrap();
    assert_eq!(r.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&r.stderr).contains("which file? Example: rythme debug factorial.ry"));
    let r = Command::new(env!("CARGO_BIN_EXE_rythme")).args(["debug", "no-such-file.ry"]).env("RYTHME_LANG", "en").output().unwrap();
    assert_eq!(r.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&r.stderr).contains("error[E017]"));
    let r = Command::new(env!("CARGO_BIN_EXE_rythme")).args(["help", "debug"]).env("RYTHME_LANG", "en").output().unwrap();
    let help = String::from_utf8_lossy(&r.stdout);
    assert!(help.starts_with("THE DEBUGGER: rythme debug"), "{help}");
    // `debug` is also a function: its help follows.
    assert!(help.contains("debug(value)"), "{help}");
}

#[test]
fn sessions_in_every_language() {
    let input = "break 2\nrun\nwhere\ndelete\nfinish\nnxet\nquit\n";
    let cases: &[(&str, &[&str])] = &[
        (
            "fr",
            &[
                "Débogage de main.ry.",
                "point d'arrêt 1 à main.ry:2",
                "● point d'arrêt 1\n→ main.ry:2  fact(n = 3) · profondeur 1\n",
                "  #1 niveau principal  main.ry:7\n",
                "`fact` a donné 6",
                "commande inconnue `nxet` : vouliez-vous dire `next` ?",
                "débogueur arrêté",
            ],
        ),
        (
            "es",
            &[
                "Depurando main.ry.",
                "punto de interrupción 1 en main.ry:2",
                "· profundidad 1",
                "nivel principal",
                "`fact` dio 6",
                "depurador detenido",
            ],
        ),
        (
            "de",
            &["main.ry wird debuggt.", "Haltepunkt 1 bei main.ry:2", "· Tiefe 1", "oberste Ebene", "`fact` ergab 6", "Debugger beendet"],
        ),
        (
            "pt",
            &[
                "Depurando main.ry.",
                "ponto de parada 1 em main.ry:2",
                "· profundidade 1",
                "nível principal",
                "`fact` deu 6",
                "depurador encerrado",
            ],
        ),
        ("zh", &["正在调试 main.ry。", "断点 1 位于 main.ry:2", "· 深度 1", "顶层", "`fact` 得出 6", "调试器已停止"]),
    ];
    for (lang, wanted) in cases {
        let (out, code) = session(lang, &[("main.ry", FACT)], input);
        assert_eq!(code, 0, "{lang}: {out}");
        for w in *wanted {
            assert!(out.contains(w), "{lang}: missing {w:?} in:\n{out}");
        }
    }
}

// ----- every example, step by step ------------------------------------------

/// Pauses at every place, and checks there that each local name, evaluated
/// with `print`, has the value the debugger shows.
struct Everywhere {
    pauses: Rc<RefCell<usize>>,
    problems: Rc<RefCell<Vec<String>>>,
}

impl Frontend for Everywhere {
    fn paused(&mut self, p: &mut Paused) -> Resume {
        *self.pauses.borrow_mut() += 1;
        let frames = p.frames();
        for (i, frame) in frames.iter().enumerate() {
            for (name, value) in p.locals(i) {
                match p.eval(i, &name) {
                    Ok(v) if v.repr() == value.repr() => {}
                    other => self.problems.borrow_mut().push(format!(
                        "frame {i} ({}): `{name}` is {} but print gives {:?}",
                        frame.name,
                        value.repr(),
                        other.map(|v| v.repr())
                    )),
                }
            }
        }
        let _ = p.globals();
        Resume::Step
    }
}

fn examples(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            examples(&path, found);
        } else if path.extension().is_some_and(|e| e == "out") {
            found.push(path.with_extension("ry"));
        }
    }
}

#[test]
fn every_example_runs_the_same_step_by_step() {
    let handle = std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(|| {
            rythme::interp::set_stack_budget(200 << 20);
            let mut found = Vec::new();
            examples(&Path::new(env!("CARGO_MANIFEST_DIR")).join("examples"), &mut found);
            found.sort();
            assert!(found.len() >= 10);
            let mut failures = Vec::new();
            for path in found {
                rythme::io::capture();
                let mut program = Program::load(&path, Vec::new(), false).expect("the example loads");
                let mut bps = Breakpoints::default();
                bps.learn(&program.modules());
                let pauses = Rc::new(RefCell::new(0));
                let problems = Rc::new(RefCell::new(Vec::new()));
                let frontend = Everywhere { pauses: pauses.clone(), problems: problems.clone() };
                let (outcome, _) = program.run(Session::new(Box::new(frontend), bps, Resume::Step, true));
                let printed = rythme::io::take_captured();
                let expected = std::fs::read_to_string(path.with_extension("out")).unwrap().replace("\r\n", "\n");
                if outcome != rythme::debug::Outcome::Finished || printed != expected {
                    failures.push(format!("{}: {outcome:?}, printed:\n{printed}", path.display()));
                }
                if *pauses.borrow() < 1 {
                    failures.push(format!("{}: only {} pauses", path.display(), pauses.borrow()));
                }
                failures.extend(problems.borrow().iter().take(5).map(|p| format!("{}: {p}", path.display())));
            }
            assert!(failures.is_empty(), "{}", failures.join("\n"));
        })
        .unwrap();
    if let Err(e) = handle.join() {
        std::panic::resume_unwind(e);
    }
}

// ----- the Debug Adapter Protocol --------------------------------------------

struct Dap {
    child: Child,
    input: ChildStdin,
    output: BufReader<std::process::ChildStdout>,
    seq: i64,
    /// Messages read while waiting for another one.
    pending: Vec<Json>,
}

impl Dap {
    fn start(dir: &Path) -> Dap {
        let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .arg("dap")
            .current_dir(dir)
            .env("RYTHME_LANG", "en")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = BufReader::new(child.stdout.take().unwrap());
        Dap { child, input, output, seq: 0, pending: Vec::new() }
    }

    fn read(&mut self) -> Json {
        let mut length = 0;
        loop {
            let mut line = String::new();
            assert!(self.output.read_line(&mut line).unwrap() > 0, "the adapter stopped");
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            if let Some(n) = line.strip_prefix("Content-Length:") {
                length = n.trim().parse().unwrap();
            }
        }
        let mut body = vec![0; length];
        self.output.read_exact(&mut body).unwrap();
        Json::parse(&String::from_utf8(body).unwrap()).unwrap()
    }

    fn wait(&mut self, test: impl Fn(&Json) -> bool) -> Json {
        if let Some(i) = self.pending.iter().position(&test) {
            return self.pending.remove(i);
        }
        loop {
            let m = self.read();
            if test(&m) {
                return m;
            }
            self.pending.push(m);
        }
    }

    fn event(&mut self, name: &str) -> Json {
        self.wait(|m| m.get("type").as_str() == Some("event") && m.get("event").as_str() == Some(name))
    }

    fn request(&mut self, command: &str, args: Json) -> Json {
        self.seq += 1;
        let seq = self.seq;
        let body =
            obj([("seq", Json::from(seq)), ("type", "request".into()), ("command", command.into()), ("arguments", args)]).to_string();
        write!(self.input, "Content-Length: {}\r\n\r\n{body}", body.len()).unwrap();
        self.input.flush().unwrap();
        self.wait(|m| m.get("type").as_str() == Some("response") && m.get("request_seq").as_i64() == Some(seq))
    }

    fn outputs(&self, category: &str) -> String {
        self.pending
            .iter()
            .filter(|m| m.get("event").as_str() == Some("output") && m.get_path(&["body", "category"]).as_str() == Some(category))
            .filter_map(|m| m.get_path(&["body", "output"]).as_str())
            .collect()
    }
}

fn names_and_values(response: &Json) -> Vec<(String, String)> {
    response
        .get_path(&["body", "variables"])
        .as_array()
        .iter()
        .map(|v| (v.get("name").as_str().unwrap().to_string(), v.get("value").as_str().unwrap().to_string()))
        .collect()
}

#[test]
fn the_debug_adapter_protocol_end_to_end() {
    let main = "use geometry\n\nfn twice(x)\n  y = x * 2\n  y\nend\n\nshapes = [{side: 2}, {side: 3}]\nareas = shapes |> map(fn(s) = geometry.area(s.side) + 0)\nprint(areas)\nprint(twice(5))\nprint(10 / (twice(1) - 2))\n";
    let geometry = "fn area(side)\n  squared = side * side\n  squared\nend\n";
    let dir = folder(&[("main.ry", main), ("geometry.ry", geometry)]);
    let program = dir.join("main.ry").display().to_string();
    let mut dap = Dap::start(&dir);
    let init = dap.request("initialize", obj([("adapterID", "rythme".into()), ("linesStartAt1", true.into())]));
    assert_eq!(init.get_path(&["body", "supportsFunctionBreakpoints"]).as_bool(), Some(true));
    let launch = dap.request("launch", obj([("program", program.clone().into()), ("stopOnEntry", true.into())]));
    assert_eq!(launch.get("success").as_bool(), Some(true));
    dap.event("initialized");
    let geometry_path = dir.join("geometry.ry").display().to_string();
    let set = dap.request(
        "setBreakpoints",
        obj([("source", obj([("path", geometry_path.into())])), ("breakpoints", Json::Arr(vec![obj([("line", Json::from(3usize))])]))]),
    );
    assert_eq!(set.get_path(&["body", "breakpoints"]).as_array()[0].get("verified").as_bool(), Some(true));
    let elsewhere = dap.request(
        "setBreakpoints",
        obj([
            ("source", obj([("path", "/nowhere/else.ry".into())])),
            ("breakpoints", Json::Arr(vec![obj([("line", Json::from(1usize))])])),
        ]),
    );
    let unverified = &elsewhere.get_path(&["body", "breakpoints"]).as_array()[0];
    assert_eq!(unverified.get("verified").as_bool(), Some(false));
    assert!(unverified.get("message").as_str().unwrap().contains("not part of the program"));
    let fns = dap.request(
        "setFunctionBreakpoints",
        obj([("breakpoints", Json::Arr(vec![obj([("name", "twice".into())]), obj([("name", "twcie".into())])]))]),
    );
    let fns = fns.get_path(&["body", "breakpoints"]).as_array().to_vec();
    assert_eq!(fns[0].get("verified").as_bool(), Some(true));
    assert!(fns[1].get("message").as_str().unwrap().contains("did you mean `twice`?"));
    dap.request("configurationDone", obj([]));

    // Paused before the first line.
    let stopped = dap.event("stopped");
    assert_eq!(stopped.get_path(&["body", "reason"]).as_str(), Some("entry"));
    let threads = dap.request("threads", obj([]));
    assert_eq!(threads.get_path(&["body", "threads"]).as_array().len(), 1);
    dap.request("next", obj([("threadId", Json::from(1i64))]));
    dap.event("stopped");
    let trace = dap.request("stackTrace", obj([("threadId", Json::from(1i64))]));
    assert_eq!(trace.get_path(&["body", "stackFrames"]).as_array()[0].get("line").as_u64(), Some(9));

    // The breakpoint in geometry.ry, through `map` and an anonymous function.
    dap.request("continue", obj([("threadId", Json::from(1i64))]));
    let stopped = dap.event("stopped");
    assert_eq!(stopped.get_path(&["body", "reason"]).as_str(), Some("breakpoint"));
    let trace = dap.request("stackTrace", obj([("threadId", Json::from(1i64))]));
    let frames = trace.get_path(&["body", "stackFrames"]).as_array().to_vec();
    let names: Vec<&str> = frames.iter().map(|f| f.get("name").as_str().unwrap()).collect();
    assert_eq!(names, ["area(side = 2)", "fn(s = {side: 2})", "map([{side: 2}, {side: 3}], <fn>)", "top level"]);
    assert!(frames[0].get_path(&["source", "path"]).as_str().unwrap().ends_with("geometry.ry"));
    assert_eq!(frames[0].get("line").as_u64(), Some(3));
    assert_eq!(frames[1].get("line").as_u64(), Some(9));
    let scopes = dap.request("scopes", obj([("frameId", Json::from(1usize))]));
    let locals = scopes.get_path(&["body", "scopes"]).as_array()[0].get("variablesReference").clone();
    let vars = dap.request("variables", obj([("variablesReference", locals)]));
    assert_eq!(names_and_values(&vars), [("s".to_string(), "{side: 2}".to_string())]);
    let record = vars.get_path(&["body", "variables"]).as_array()[0].get("variablesReference").clone();
    let fields = dap.request("variables", obj([("variablesReference", record)]));
    assert_eq!(names_and_values(&fields), [("side".to_string(), "2".to_string())]);
    let value = dap.request("evaluate", obj([("expression", "squared * 10".into()), ("frameId", Json::from(0usize))]));
    assert_eq!(value.get_path(&["body", "result"]).as_str(), Some("40"));
    let wrong = dap.request("evaluate", obj([("expression", "sqared".into()), ("frameId", Json::from(0usize))]));
    assert_eq!(wrong.get("success").as_bool(), Some(false));
    assert!(wrong.get("message").as_str().unwrap().contains("did you mean `squared`?"));

    // Out of `area`: its result.
    dap.request(
        "setBreakpoints",
        obj([("source", obj([("path", dir.join("geometry.ry").display().to_string().into())])), ("breakpoints", Json::Arr(Vec::new()))]),
    );
    dap.request("stepOut", obj([("threadId", Json::from(1i64))]));
    let stopped = dap.event("stopped");
    assert!(stopped.get_path(&["body", "description"]).as_str().unwrap().contains("`area` gave 4"));

    // The function breakpoint, then into it, and the values of its scope.
    dap.request("continue", obj([("threadId", Json::from(1i64))]));
    let stopped = dap.event("stopped");
    assert_eq!(stopped.get_path(&["body", "reason"]).as_str(), Some("function breakpoint"));
    dap.request("stepIn", obj([("threadId", Json::from(1i64))]));
    dap.event("stopped");
    let scopes = dap.request("scopes", obj([("frameId", Json::from(0usize))]));
    let locals = scopes.get_path(&["body", "scopes"]).as_array()[0].get("variablesReference").clone();
    let globals = scopes.get_path(&["body", "scopes"]).as_array()[1].get("variablesReference").clone();
    let vars = dap.request("variables", obj([("variablesReference", locals)]));
    assert_eq!(names_and_values(&vars), [("x".to_string(), "5".to_string()), ("y".to_string(), "10".to_string())]);
    let vars = dap.request("variables", obj([("variablesReference", globals)]));
    assert_eq!(
        names_and_values(&vars),
        [("shapes".to_string(), "[{side: 2}, {side: 3}]".to_string()), ("areas".to_string(), "[4, 9]".to_string())]
    );

    // The error stops the program where it happens: an exception.
    dap.request("setFunctionBreakpoints", obj([("breakpoints", Json::Arr(Vec::new()))]));
    dap.request("continue", obj([("threadId", Json::from(1i64))]));
    let stopped = dap.event("stopped");
    assert_eq!(stopped.get_path(&["body", "reason"]).as_str(), Some("exception"));
    assert_eq!(stopped.get_path(&["body", "text"]).as_str(), Some("division by zero"));
    dap.request("continue", obj([("threadId", Json::from(1i64))]));
    let exited = dap.event("exited");
    assert_eq!(exited.get_path(&["body", "exitCode"]).as_i64(), Some(1));
    dap.event("terminated");
    assert_eq!(dap.outputs("stdout"), "[4, 9]\n10\n");
    assert!(dap.outputs("stderr").contains("error[E008]: division by zero"), "{}", dap.outputs("stderr"));
    let after = dap.request("stackTrace", obj([("threadId", Json::from(1i64))]));
    assert_eq!(after.get("success").as_bool(), Some(false));
    dap.request("disconnect", obj([]));
    assert_eq!(dap.child.wait().unwrap().code(), Some(0));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_adapter_reports_a_program_with_errors() {
    let dir = folder(&[("main.ry", "print(lenght([1]))\n")]);
    let mut dap = Dap::start(&dir);
    dap.request("initialize", obj([("adapterID", "rythme".into())]));
    let launch = dap.request("launch", obj([("program", dir.join("main.ry").display().to_string().into())]));
    assert_eq!(launch.get("success").as_bool(), Some(false));
    assert_eq!(dap.event("exited").get_path(&["body", "exitCode"]).as_i64(), Some(1));
    dap.event("terminated");
    assert!(dap.outputs("stderr").contains("unknown name `lenght`"), "{}", dap.outputs("stderr"));
    let missing = dap.request("launch", obj([]));
    assert!(missing.get("message").as_str().unwrap().contains("which program?"));
    dap.request("disconnect", obj([]));
    assert_eq!(dap.child.wait().unwrap().code(), Some(0));
    let _ = std::fs::remove_dir_all(&dir);
}
