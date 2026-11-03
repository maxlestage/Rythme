//! `async fn` and `await`: tasks that run at the same time as the program.
//!
//! Calling an `async fn` starts it and gives a `Task` at once; `await task`
//! waits for its result. Rythme values never change and functions have no
//! hidden state, so tasks can run on other threads without any lock: each
//! worker thread has an interpreter of its own, with the program loaded, and
//! tasks send it their arguments and get their results back as copies.
//!
//! Where threads are not available (WebAssembly, the REPL, a worker running
//! a task, arguments that are functions), the task runs at once, on the
//! calling thread: the result is the same, only the waiting changes.

// In WebAssembly there are no threads: tasks run when they are called, and
// what sends them elsewhere is not used.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use crate::builtins::{Builtin, Category};
use crate::diagnostic::{Diagnostic, Fallible};
use crate::interp::Interp;
use crate::source::Span;
use crate::symbol::Sym;
use crate::value::{CtorInfo, Fields, Record, Shape, Value, Variant};

/// The name of the hidden built-in that the code of an `async fn` calls to
/// start its task (a name no program can write).
pub const SPAWN_NAME: &str = "#spawn";

/// The name the body of the `async fn` `name` (as stored) is kept under.
pub fn body_name(stored: Sym) -> Sym {
    Sym::new(&format!("#async.{}", stored.name()))
}

/// Called by the code of every `async fn`: `#spawn("#async.name", [args])`.
pub static SPAWN: Builtin = Builtin {
    name: SPAWN_NAME,
    sig: "#spawn(name, args)",
    min: 2,
    max: 2,
    cat: Category::Basics,
    en: "Starts the task of an `async fn` (used by the code of `async fn`).",
    fr: "Démarre la tâche d'une `async fn` (utilisé par le code des `async fn`).",
    example: "",
    func: spawn,
};

// ----- values that travel between threads -----------------------------------

/// A copy of a value that can be sent to another thread: everything but
/// functions (a function may hold values of its own interpreter).
enum Plain {
    Unit,
    Bool(bool),
    Int(i64),
    Big(String),
    Float(f64),
    Symbol(String),
    Text(String),
    List(Vec<Plain>),
    Record(Vec<(String, Plain)>),
    Variant(String, Vec<Plain>),
    Dict(Vec<(Plain, Plain)>),
    Set(Vec<Plain>),
}

/// The value as a `Plain`, or `None` when it holds a function.
fn plain(v: &Value) -> Option<Plain> {
    Some(match v {
        Value::Unit => Plain::Unit,
        Value::Bool(b) => Plain::Bool(*b),
        Value::Int(n) => Plain::Int(*n),
        Value::Big(n) => Plain::Big(n.to_string()),
        Value::Float(x) => Plain::Float(*x),
        Value::Symbol(s) => Plain::Symbol(s.name().to_string()),
        Value::Text(t) => Plain::Text(t.as_str().to_string()),
        Value::List(l) => Plain::List(l.iter().map(plain).collect::<Option<_>>()?),
        Value::Record(r) => Plain::Record(r.iter().map(|(k, v)| Some((k.to_string(), plain(v)?))).collect::<Option<_>>()?),
        Value::Variant(var) => Plain::Variant(var.ctor.id.name().to_string(), var.fields.iter().map(plain).collect::<Option<_>>()?),
        Value::Dict(m) => Plain::Dict(m.iter().map(|(k, v)| Some((plain(k)?, plain(v)?))).collect::<Option<_>>()?),
        Value::Set(m) => Plain::Set(m.keys().map(plain).collect::<Option<_>>()?),
        Value::Builtin(_) | Value::Func(_) | Value::Ctor(_) => return None,
    })
}

/// The value a `Plain` stands for, in the interpreter `it`.
fn value(it: &Interp, p: Plain) -> Value {
    match p {
        Plain::Unit => Value::Unit,
        Plain::Bool(b) => Value::Bool(b),
        Plain::Int(n) => Value::Int(n),
        Plain::Big(s) => match crate::bigint::BigInt::parse(&s, 10) {
            Some(Ok(n)) => Value::Big(Rc::new(n)),
            _ => Value::Unit,
        },
        Plain::Float(x) => Value::Float(x),
        Plain::Symbol(s) => Value::Symbol(Sym::new(&s)),
        Plain::Text(s) => Value::text(&s),
        Plain::List(xs) => Value::list(xs.into_iter().map(|x| value(it, x)).collect()),
        Plain::Record(fields) => {
            let keys = fields.iter().map(|(k, _)| Rc::from(k.as_str())).collect();
            let vals = fields.into_iter().map(|(_, v)| value(it, v)).collect();
            Value::Record(Rc::new(Record::with_shape(Rc::new(Shape::new(keys)), vals)))
        }
        Plain::Variant(id, fields) => {
            let fields: Fields = fields.into_iter().map(|x| value(it, x)).collect();
            match ctor(it, &id) {
                Some(ctor) => Value::Variant(Rc::new(Variant { ctor, fields })),
                None => Value::Unit,
            }
        }
        Plain::Dict(pairs) => {
            Value::Dict(crate::dict::Map::from_pairs(pairs.into_iter().map(|(k, v)| (value(it, k), value(it, v))).collect()))
        }
        Plain::Set(keys) => Value::Set(crate::dict::Map::from_pairs(keys.into_iter().map(|k| (value(it, k), Value::Unit)).collect())),
    }
}

/// The constructor stored as `id` in the interpreter: `Some`, `Ok`, a
/// constructor of the program, or `Task`.
fn ctor(it: &Interp, id: &str) -> Option<Rc<CtorInfo>> {
    if id == "Task" {
        return Some(task_ctor());
    }
    let found = it.types.values().flatten().find(|c| &*c.id.name() == id).cloned();
    found.or_else(|| match it.globals.get(&Sym::new(id))? {
        Value::Ctor(c) => Some(c.clone()),
        Value::Variant(v) if v.fields.is_empty() => Some(v.ctor.clone()),
        _ => None,
    })
}

// ----- tasks ------------------------------------------------------------------

/// What `await` gets from a task that stopped with an error.
struct Failure {
    code: &'static str,
    message: String,
    /// The whole message, with its place in the program.
    shown: String,
}

enum State {
    Done(Value),
    Failed(Failure),
    Running(std::sync::mpsc::Receiver<Result<Plain, Failure>>),
    /// Taken by `await` while it waits.
    Taken,
}

thread_local! {
    static TASKS: RefCell<Vec<State>> = const { RefCell::new(Vec::new()) };
    static TASK_CTOR: RefCell<Option<Rc<CtorInfo>>> = const { RefCell::new(None) };
    /// This thread is a worker running tasks: an `async fn` it calls runs at once.
    static IN_WORKER: Cell<bool> = const { Cell::new(false) };
}

/// The constructor of the values `Task(n)` (not a global name: a task
/// cannot be made by hand).
fn task_ctor() -> Rc<CtorInfo> {
    TASK_CTOR.with(|c| {
        c.borrow_mut()
            .get_or_insert_with(|| {
                let task = Sym::new("Task");
                Rc::new(CtorInfo {
                    type_name: task,
                    name: task,
                    id: task,
                    type_id: task,
                    fields: vec![Sym::new("id")],
                    index: 0,
                    doc: None,
                })
            })
            .clone()
    })
}

fn new_task(state: State) -> Value {
    let id = TASKS.with(|t| {
        let mut t = t.borrow_mut();
        t.push(state);
        t.len() as i64 - 1
    });
    Value::Variant(Rc::new(Variant { ctor: task_ctor(), fields: Fields::one(Value::Int(id)) }))
}

/// `#spawn(name, args)`: runs the body of an `async fn` as a task.
fn spawn(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (Value::Text(name), Value::List(args)) = (&a[0], &a[1]) else {
        return Ok(Value::Unit);
    };
    let name = Sym::new(name.as_str());
    let args: Vec<Value> = args.iter().cloned().collect();
    #[cfg(not(target_arch = "wasm32"))]
    if !IN_WORKER.with(Cell::get) {
        if let Some(sent) = args.iter().map(plain).collect::<Option<Vec<_>>>() {
            if let Some(rx) = pool::submit(&name.name(), sent) {
                return Ok(new_task(State::Running(rx)));
            }
        }
    }
    // No other thread: the task runs now, and `await` gives its result.
    let body = it.globals.get(&name).cloned().unwrap_or(Value::Unit);
    match it.call(body, args, span) {
        Ok(v) => Ok(new_task(State::Done(v))),
        Err(d) => Ok(new_task(State::Failed(failure(&d)))),
    }
}

fn failure(d: &Diagnostic) -> Failure {
    Failure { code: d.code, message: d.message.clone(), shown: d.render(false) }
}

/// The task a value stands for, or an error saying what `await` expects.
fn task_id(v: &Value, f: &str, span: Span) -> Fallible<usize> {
    if let Value::Variant(var) = v {
        if var.ctor.id == Sym::new("Task") {
            if let Some(Value::Int(n)) = var.fields.first() {
                return Ok(*n as usize);
            }
        }
    }
    let shown = v.describe();
    Err(Diagnostic::error(
        "E006",
        t!(
            "`{f}` expects a Task, the result of a call to an `async fn`, not {shown}",
            "`{f}` attend une Task, le résultat de l'appel d'une `async fn`, pas {shown}"
        ),
    )
    .at(span)
    .help(t!(
        "define the function with `async fn`, then `await` what its call gives",
        "définissez la fonction avec `async fn`, puis faites `await` sur ce que donne son appel"
    ))
    .boxed())
}

/// Waits for the task `id` and gives its result.
fn wait(it: &Interp, id: usize, span: Span) -> Fallible<Value> {
    let state = TASKS.with(|t| t.borrow_mut().get_mut(id).map(|s| std::mem::replace(s, State::Taken)));
    let done = match state {
        Some(State::Done(v)) => Ok(v),
        Some(State::Failed(f)) => Err(f),
        Some(State::Running(rx)) => match rx.recv() {
            Ok(Ok(p)) => Ok(value(it, p)),
            Ok(Err(f)) => Err(f),
            Err(_) => Err(Failure {
                code: "E023",
                message: t!("the task stopped without a result", "la tâche s'est arrêtée sans résultat"),
                shown: String::new(),
            }),
        },
        Some(State::Taken) | None => {
            // Not reached: an awaited task keeps its result (below), and a
            // `Task` cannot be made by hand; kept so that nothing can panic.
            return Err(Diagnostic::error("E023", t!("this task was already awaited", "cette tâche a déjà été attendue"))
                .at(span)
                .help(t!(
                    "keep the result of `await` in a name and use that name again",
                    "gardez le résultat de `await` dans un nom et réutilisez ce nom"
                ))
                .boxed());
        }
    };
    // A task can be awaited again: its result stays.
    TASKS.with(|t| {
        if let Some(s) = t.borrow_mut().get_mut(id) {
            *s = match &done {
                Ok(v) => State::Done(v.clone()),
                Err(f) => State::Failed(Failure { code: f.code, message: f.message.clone(), shown: f.shown.clone() }),
            };
        }
    });
    done.map_err(|f| {
        let mut d = Diagnostic::error(f.code, f.message).at(span).label(t!("in the task awaited here", "dans la tâche attendue ici"));
        if !f.shown.is_empty() {
            d = d.note(f.shown.trim_end().to_string());
        }
        d.boxed()
    })
}

/// `await task`: the result of the task.
pub(crate) fn await_task(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let id = task_id(&a[0], "await", span)?;
    wait(it, id, span)
}

/// `await_all(tasks)`: the results of the tasks of a list, in its order.
pub(crate) fn await_all(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let Value::List(tasks) = &a[0] else {
        let shown = a[0].describe();
        return Err(Diagnostic::error(
            "E006",
            t!("`await_all` expects a list of tasks, not {shown}", "`await_all` attend une liste de tâches, pas {shown}"),
        )
        .at(span)
        .boxed());
    };
    let ids = tasks.iter().map(|t| task_id(t, "await_all", span)).collect::<Fallible<Vec<_>>>()?;
    let values = ids.into_iter().map(|id| wait(it, id, span)).collect::<Fallible<Vec<_>>>()?;
    Ok(Value::list(values))
}

// ----- the program the workers load --------------------------------------------

/// What a worker needs to load the program: its source and where it is.
#[derive(Clone)]
pub struct Program {
    pub name: String,
    pub src: String,
    pub path: Option<PathBuf>,
    pub base: PathBuf,
}

thread_local! {
    static PROGRAM: RefCell<Option<Program>> = const { RefCell::new(None) };
}

/// What a host (a web framework...) does to the interpreter of each worker
/// before it loads the program: the same functions of its own as on its threads.
static WORKER_SETUP: std::sync::OnceLock<fn(&mut Interp)> = std::sync::OnceLock::new();

/// Sets what a host does to the interpreter of each worker (see
/// `WORKER_SETUP`); the first call wins.
pub fn on_worker_start(setup: fn(&mut Interp)) {
    let _ = WORKER_SETUP.set(setup);
}

/// Notes the program run by this thread, for the workers of its tasks.
pub fn set_program(p: Option<Program>) {
    PROGRAM.with(|c| *c.borrow_mut() = p);
    #[cfg(not(target_arch = "wasm32"))]
    pool::reset();
}

#[cfg(not(target_arch = "wasm32"))]
mod pool {
    use super::*;
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::sync::{Arc, Mutex};

    struct Job {
        name: String,
        args: Vec<Plain>,
        reply: Sender<Result<Plain, Failure>>,
    }

    thread_local! {
        static POOL: RefCell<Option<Sender<Job>>> = const { RefCell::new(None) };
    }

    pub(super) fn reset() {
        POOL.with(|p| *p.borrow_mut() = None);
    }

    /// Sends a task to the workers of this thread's program (started the first
    /// time); `None` when there is no program to load (the REPL...).
    pub(super) fn submit(name: &str, args: Vec<Plain>) -> Option<Receiver<Result<Plain, Failure>>> {
        let tx = POOL.with(|p| {
            let mut p = p.borrow_mut();
            if p.is_none() {
                *p = start();
            }
            p.clone()
        })?;
        let (reply, rx) = channel();
        tx.send(Job { name: name.to_string(), args, reply }).ok()?;
        Some(rx)
    }

    fn start() -> Option<Sender<Job>> {
        let program = PROGRAM.with(|c| c.borrow().clone())?;
        let (tx, rx) = channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let workers = std::thread::available_parallelism().map_or(2, |n| n.get()).clamp(2, 8);
        let lang = crate::lang::get();
        let stack = crate::interp::stack_budget() + (128 << 20);
        for i in 0..workers {
            let (rx, program) = (rx.clone(), program.clone());
            let spawned =
                std::thread::Builder::new().name(format!("rythme-task-{i}")).stack_size(stack).spawn(move || work(program, lang, rx));
            if spawned.is_err() {
                return None;
            }
        }
        Some(tx)
    }

    /// A worker: loads the program once (without running its statements),
    /// then runs the tasks it is given.
    fn work(program: Program, lang: crate::lang::Lang, jobs: Arc<Mutex<Receiver<Job>>>) {
        crate::lang::set(lang);
        IN_WORKER.with(|w| w.set(true));
        let mut driver = crate::driver::Driver::new();
        driver.quiet = true;
        if let Some(setup) = WORKER_SETUP.get() {
            setup(&mut driver.interp);
        }
        let loaded = driver
            .load_source(&program.name, &program.src, program.path.clone(), &program.base)
            .filter(|m| driver.prepare_definitions(m).is_ok());
        loop {
            let job = match jobs.lock() {
                Ok(rx) => rx.recv(),
                Err(_) => return,
            };
            let Ok(job) = job else { return };
            let result = if loaded.is_none() {
                Err(Failure {
                    code: "E023",
                    message: t!(
                        "the program could not be loaded to run the task",
                        "le programme n'a pas pu être chargé pour lancer la tâche"
                    ),
                    shown: String::new(),
                })
            } else {
                run(&mut driver.interp, &job.name, job.args)
            };
            let _ = job.reply.send(result);
        }
    }

    fn run(it: &mut Interp, name: &str, args: Vec<Plain>) -> Result<Plain, Failure> {
        let body = it.globals.get(&Sym::new(name)).cloned().unwrap_or(Value::Unit);
        let args = args.into_iter().map(|a| value(it, a)).collect();
        match it.call(body, args, Span::default()) {
            Ok(v) => plain(&v).ok_or_else(|| Failure {
                code: "E023",
                message: t!(
                    "an `async fn` cannot give a function as its result",
                    "une `async fn` ne peut pas donner une fonction comme résultat"
                ),
                shown: String::new(),
            }),
            Err(d) => Err(failure(&d)),
        }
    }
}
