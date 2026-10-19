//! The debugger: `rythme debug file.ry` in a terminal ([`term`]) and
//! `rythme dap` for editors ([`dap`], the Debug Adapter Protocol).
//!
//! A program runs under the debugger in the usual interpreter, with
//! differences that cost (almost) nothing when no debugger watches:
//!
//! - its code is prepared with the places where it may pause (`Stmt::Stop`
//!   of `compile`): before each statement of a block, at the start of each
//!   body of a function, of each branch of `if` and each case of `match`,
//!   before each top-level expression. A body that is not a block becomes
//!   one, so the interpreter meets them among the statements of blocks and
//!   nowhere else;
//! - the interpreter's limit of calls stays at 0, so that each call goes
//!   through a hook (`Interp::call_hook`) instead of counting down, and each
//!   result through another (`return_hook`): the frames of the calls are
//!   followed here, and a call in tail position, which replaces the frame
//!   of its caller, is noted as such. An error goes through `error_hook`;
//! - built-in functions run on the stack of the program (`watched`), so that
//!   the frames below a function given to `map` stay visible.
//!
//! Nothing here changes a value: `print` evaluates an expression in a
//! paused frame with the functions without effects only.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;

use crate::ast::{Expr, ExprKind, FnDef, Item, PatKind, Stmt};
use crate::compile::{self, FnCode, StopInfo};
use crate::diagnostic::{Diagnostic, Fallible};
use crate::driver::{Driver, Module};
use crate::interp::{Interp, Mode};
use crate::source::{self, Span};
use crate::symbol::Sym;
use crate::value::{Closure, Value};

#[cfg(not(target_arch = "wasm32"))]
pub mod dap;
#[cfg(not(target_arch = "wasm32"))]
pub mod term;

/// A call in progress, as the debugger follows it.
pub struct Frame {
    /// Different for each call (a call in tail position gets a new one).
    id: u64,
    /// `Interp::depth` while it runs (0 for the top level).
    depth: usize,
    what: What,
    /// The arguments it was called with.
    args: Vec<Value>,
    /// Where it was called.
    call: Option<Span>,
    /// Where its slots start on the stack.
    base: usize,
    /// The last place reached where it may pause, with the names visible there.
    stop: Option<(Span, Rc<StopInfo>)>,
    /// It replaced the frame of this function (a call in tail position).
    replaced: Option<String>,
    /// A breakpoint on the function: pause at its first place.
    entry_break: Option<u32>,
}

enum What {
    Top,
    Func { def: Rc<FnDef>, code: Rc<FnCode>, clo: Rc<Closure> },
    Builtin(&'static crate::builtins::Builtin),
}

/// How the user wants the program to go on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    /// Until a breakpoint.
    Continue,
    /// To the next place, also inside the functions called.
    Step,
    /// To the next place of this function (or of its caller).
    Next,
    /// Until this function gives its result.
    Finish,
}

/// Why the program is paused.
#[derive(Clone)]
pub enum Reason {
    /// Before the first line, as asked.
    Entry,
    /// After `step` or `next`.
    Step,
    /// At breakpoints (their numbers).
    Breakpoint(Vec<u32>),
    /// At the start of a function with a breakpoint (its number).
    Function(u32),
    /// A function gave its result, after `finish`.
    Returned { name: String, value: Value },
    /// An error stopped the program.
    Error(Box<Diagnostic>),
    /// Asked by the editor while the program ran.
    Pause,
}

/// What a debugger shows and asks: the terminal, or an editor.
pub trait Frontend {
    /// The program is paused: answer the user until they say how to go on.
    fn paused(&mut self, p: &mut Paused) -> Resume;
    /// Called at each place where the program may pause: `true` pauses it
    /// now (a pause asked while it runs).
    fn interrupt(&mut self, _breakpoints: &mut Breakpoints) -> bool {
        false
    }
}

/// The state of the debugger while a program runs, kept by the interpreter.
pub struct Session {
    frames: Vec<Frame>,
    next_id: u64,
    pub breakpoints: Breakpoints,
    mode: Resume,
    /// Pause on the next place, whatever the mode (`stopOnEntry`).
    entry: bool,
    /// The function, the file and the line of the last pause: the places
    /// that follow on the same line of the same call do not pause again.
    last: Option<(u64, u32, usize)>,
    frontend: Option<Box<dyn Frontend>>,
    /// The error that stopped the program was already shown in a pause.
    error_seen: bool,
    /// The top-level names of the program, shown with its values.
    globals: Vec<Sym>,
    /// The depth of the frame of the last pause: `next` and `finish` count from it.
    depth_mark: usize,
}

impl Session {
    pub fn new(frontend: Box<dyn Frontend>, breakpoints: Breakpoints, first: Resume, entry: bool) -> Session {
        Session {
            frames: Vec::new(),
            next_id: 0,
            breakpoints,
            mode: first,
            entry,
            last: None,
            frontend: Some(frontend),
            error_seen: false,
            globals: Vec::new(),
            depth_mark: 0,
        }
    }

    fn new_frame(&mut self, depth: usize, what: What, args: Vec<Value>, call: Option<Span>, base: usize) -> Frame {
        self.next_id += 1;
        Frame { id: self.next_id, depth, what, args, call, base, stop: None, replaced: None, entry_break: None }
    }

    /// The frame of the top level, at the bottom.
    fn ensure_top(&mut self) {
        if self.frames.first().is_none_or(|f| f.depth != 0) {
            let top = self.new_frame(0, What::Top, Vec::new(), None, 0);
            self.frames.insert(0, top);
        }
    }

    /// Pauses: the frontend talks with the user, then says how to go on.
    fn pause(&mut self, it: &mut Interp, reason: Reason, at: Span) {
        let depth = self.frames.last().map_or(0, |f| f.depth);
        if let Some(f) = self.frames.last() {
            let (file, line) = line_of(at);
            self.last = Some((f.id, file, line));
        }
        let mut frontend = self.frontend.take().expect("a frontend");
        let resume = {
            let mut p = Paused { it, s: self, reason, at };
            frontend.paused(&mut p)
        };
        self.frontend = Some(frontend);
        self.entry = false;
        self.mode = resume;
        self.depth_mark = depth;
    }
}

/// Where the debugger is in the program when it pauses.
pub struct Paused<'a> {
    it: &'a mut Interp,
    s: &'a mut Session,
    pub reason: Reason,
    /// Where the program is paused.
    pub at: Span,
}

/// A frame as shown: innermost first.
pub struct FrameView {
    /// `fact`, `fn` for an anonymous function, `map` for a built-in one; empty for the top level.
    pub name: String,
    pub builtin: bool,
    /// The arguments, with the names of the parameters.
    pub args: Vec<(String, Value)>,
    /// Where it is: the pause for the first one, the call of the next one for the others.
    pub at: Option<Span>,
    /// It replaced the frame of this function (a call in tail position).
    pub replaced: Option<String>,
}

impl Paused<'_> {
    /// The frames, innermost first.
    pub fn frames(&self) -> Vec<FrameView> {
        let frames = &self.s.frames;
        let mut out = Vec::with_capacity(frames.len());
        for (i, f) in frames.iter().enumerate().rev() {
            let at = if i + 1 == frames.len() { Some(self.at) } else { frames[i + 1].call };
            let (name, builtin, args) = match &f.what {
                What::Top => (String::new(), false, Vec::new()),
                What::Func { def, .. } => {
                    let names = param_names(def);
                    let args = names.into_iter().zip(f.args.iter().cloned()).collect();
                    (fn_name(def), false, args)
                }
                What::Builtin(b) => (b.name.to_string(), true, f.args.iter().map(|v| (String::new(), v.clone())).collect()),
            };
            out.push(FrameView { name, builtin, args, at, replaced: f.replaced.clone() });
        }
        out
    }

    fn frame(&self, index: usize) -> Option<&Frame> {
        let n = self.s.frames.len();
        index.checked_add(1).and_then(|i| n.checked_sub(i)).map(|i| &self.s.frames[i])
    }

    /// How many calls of functions of the program are in progress.
    pub fn depth(&self) -> usize {
        self.s.frames.iter().filter(|f| matches!(f.what, What::Func { .. })).count()
    }

    /// How many times the function of the innermost frame is in progress
    /// (1 when it does not call itself).
    pub fn recursion(&self) -> usize {
        let Some(What::Func { def, .. }) = self.s.frames.last().map(|f| &f.what) else { return 0 };
        self.s.frames.iter().filter(|f| matches!(&f.what, What::Func { def: d, .. } if Rc::ptr_eq(d, def))).count()
    }

    /// The local names of a frame and their values, in the order they were given.
    pub fn locals(&self, index: usize) -> Vec<(String, Value)> {
        self.local_values(index).into_iter().map(|(s, v)| (s.name().to_string(), v)).collect()
    }

    fn local_values(&self, index: usize) -> Vec<(Sym, Value)> {
        let Some(f) = self.frame(index) else { return Vec::new() };
        let stack = self.it.stack_values();
        let mut out: Vec<(Sym, Value)> = Vec::new();
        if let What::Func { code, clo, .. } = &f.what {
            for (s, v) in code.capture_names.iter().zip(&clo.captured) {
                out.push((*s, v.clone()));
            }
        }
        match &f.stop {
            Some((_, info)) => {
                for &(s, slot) in &info.names {
                    if let Some(v) = stack.get(f.base + slot as usize) {
                        out.retain(|(n, _)| *n != s);
                        out.push((s, v.clone()));
                    }
                }
            }
            None => {
                if let What::Func { def, .. } = &f.what {
                    for (name, v) in param_names(def).into_iter().zip(&f.args) {
                        out.push((Sym::new(&name), v.clone()));
                    }
                }
            }
        }
        out
    }

    /// The top-level names of the program that have a value, other than functions.
    pub fn globals(&self) -> Vec<(String, Value)> {
        let mut out = Vec::new();
        for s in &self.s.globals {
            let stored = self.it.stored(*s);
            if let Some(v) = self.it.globals.get(&stored) {
                if !v.is_callable() {
                    out.push((s.name().to_string(), v.clone()));
                }
            }
        }
        out
    }

    /// Evaluates an expression in a frame, with its local names: only the
    /// functions without effects, and at most a million calls.
    pub fn eval(&mut self, index: usize, text: &str) -> Result<Value, String> {
        let file = source::add("<print>", text);
        let mut items = crate::lexer::lex(text, file).and_then(crate::parser::parse).map_err(|d| d.render(false))?;
        let names = match self.frame(index).and_then(|f| f.stop.as_ref()) {
            Some((_, info)) => info.file_names.clone(),
            None => self.it.names.clone(),
        };
        let modules: HashSet<Sym> = names.keys().filter_map(|k| k.split_module().map(|(m, _)| m)).collect();
        crate::modules::qualify(&mut items, &modules);
        let e = match items.as_slice() {
            [Item::Stmt(Stmt::Expr(e))] => e,
            _ => {
                return Err(t!(
                    "`print` shows the value of one expression, like `print n * 2` or `print len(xs)`",
                    "`print` montre la valeur d'une expression, comme `print n * 2` ou `print len(xs)`"
                ))
            }
        };
        let (syms, values): (Vec<Sym>, Vec<Value>) = self.local_values(index).into_iter().unzip();
        let code = compile::with_locals(e, &names, &syms);
        let steps = std::mem::replace(&mut self.it.steps_left, 1_000_000);
        self.it.pure_only = true;
        let r = self.it.eval_code(&code, values);
        self.it.pure_only = false;
        self.it.steps_left = steps;
        r.map_err(|d| d.render(false).trim_end().to_string())
    }

    pub fn breakpoints(&mut self) -> &mut Breakpoints {
        &mut self.s.breakpoints
    }
}

/// The names of the parameters of a function: those of its definitions, or `arg1`...
fn param_names(def: &FnDef) -> Vec<String> {
    (0..def.arity)
        .map(|i| {
            def.clauses
                .iter()
                .find_map(|c| match &c.params[i].kind {
                    PatKind::Bind(s) => Some(s.name().to_string()),
                    _ => None,
                })
                .unwrap_or_else(|| format!("arg{}", i + 1))
        })
        .collect()
}

fn fn_name(def: &FnDef) -> String {
    def.name.map_or_else(|| "fn".to_string(), |n| n.name().to_string())
}

/// The file and the 1-based line of the start of a span.
pub fn line_of(span: Span) -> (u32, usize) {
    match source::get(span.file) {
        Some(f) => (span.file, f.line_col(span.start as usize).0),
        None => (span.file, 0),
    }
}

// ----- hooks of the interpreter -------------------------------------------

/// Runs `f` with the session taken out of the interpreter (so that both can
/// be changed), when a debugger watches; otherwise gives back `arg`.
fn with_session<A>(it: &mut Interp, arg: A, f: impl FnOnce(&mut Session, &mut Interp, A) -> A) -> A {
    let Some(mut s) = it.debugger.take() else { return arg };
    let r = f(&mut s, it, arg);
    it.debugger = Some(s);
    r
}

/// A place where the program may pause: `at`, with the names of `info`, in
/// the frame starting at `base`.
pub(crate) fn stop(it: &mut Interp, at: Span, info: &Rc<StopInfo>, base: usize) {
    with_session(it, (), |s, it, _| {
        let depth = it.call_depth();
        s.ensure_top();
        while s.frames.len() > 1 && s.frames.last().is_some_and(|f| f.depth > depth) {
            s.frames.pop();
        }
        let f = if depth == 0 { &mut s.frames[0] } else { s.frames.last_mut().expect("a frame") };
        if depth == 0 {
            f.base = base;
        }
        f.stop = Some((at, info.clone()));
        let id = f.id;
        let entry = f.entry_break.take();
        let (file, line) = line_of(at);
        let again = s.last == Some((id, file, line));
        if !again {
            s.last = None;
        }
        let asked = {
            let mut frontend = s.frontend.take().expect("a frontend");
            let asked = frontend.interrupt(&mut s.breakpoints);
            s.frontend = Some(frontend);
            asked
        };
        let reason = if asked {
            Some(Reason::Pause)
        } else if let Some(n) = entry {
            Some(Reason::Function(n))
        } else if again {
            None
        } else if s.entry {
            Some(Reason::Entry)
        } else {
            let hits = s.breakpoints.at(file, line);
            if !hits.is_empty() {
                Some(Reason::Breakpoint(hits))
            } else {
                match s.mode {
                    Resume::Step => Some(Reason::Step),
                    Resume::Next if depth <= s.depth_mark => Some(Reason::Step),
                    _ => None,
                }
            }
        };
        if let Some(reason) = reason {
            s.pause(it, reason, at);
        }
    })
}

/// A call starts: `f`, with its arguments from `base` on.
pub(crate) fn call(it: &mut Interp, f: &Value, member: Option<u32>, base: usize, span: Span) -> Fallible<()> {
    with_session(it, Ok(()), |s, it, _| {
        let depth = it.call_depth();
        s.ensure_top();
        let mut replaced = None;
        let mut call = Some(span);
        while s.frames.len() > 1 && s.frames.last().is_some_and(|fr| fr.depth > depth) {
            s.frames.pop();
        }
        // The frame at this depth is still there: a call in tail position
        // replaces it, and gives its result where that one was called.
        if s.frames.len() > 1 && s.frames.last().is_some_and(|fr| fr.depth == depth) {
            let old = s.frames.pop().expect("a frame");
            call = old.call.or(call);
            replaced = match old.what {
                What::Func { def, .. } => Some(fn_name(&def)),
                What::Builtin(b) => Some(b.name.to_string()),
                What::Top => None,
            };
        }
        let args = it.stack_values().get(base..).map(<[Value]>::to_vec).unwrap_or_default();
        let what = match f {
            Value::Func(clo) => {
                let (def, code) = match member {
                    None => (clo.def.clone(), clo.code.clone()),
                    Some(i) => match clo.siblings.as_ref().and_then(|g| g.get(i as usize)) {
                        Some(sib) => (sib.def.clone(), sib.code.clone()),
                        None => return Ok(()),
                    },
                };
                What::Func { def, code, clo: clo.clone() }
            }
            Value::Builtin(b) => What::Builtin(b),
            _ => return Ok(()),
        };
        let mut frame = s.new_frame(depth, what, args, call, base);
        frame.replaced = replaced;
        if let What::Func { def, .. } = &frame.what {
            if let Some(name) = def.name {
                frame.entry_break = s.breakpoints.function(&name.name());
            }
        }
        s.frames.push(frame);
        Ok(())
    })
}

/// A call gives its result (or an error).
pub(crate) fn returned(it: &mut Interp, r: Fallible<Value>) -> Fallible<Value> {
    with_session(it, r, |s, it, r| {
        let depth = it.call_depth();
        let Some(f) = s.frames.last() else { return r };
        if f.depth != depth || matches!(f.what, What::Top) {
            return r;
        }
        let f = s.frames.pop().expect("a frame");
        if let (What::Func { def, .. }, Ok(v), Resume::Finish) = (&f.what, &r, s.mode) {
            if depth <= s.depth_mark {
                let at = f.call.unwrap_or_default();
                s.pause(it, Reason::Returned { name: fn_name(def), value: v.clone() }, at);
            }
        }
        r
    })
}

/// An error stops the program: pause where it happened, once.
pub(crate) fn error(it: &mut Interp, d: Box<Diagnostic>) -> Box<Diagnostic> {
    with_session(it, d, |s, it, d| {
        if s.error_seen {
            return d;
        }
        s.error_seen = true;
        s.ensure_top();
        let depth = it.call_depth();
        while s.frames.len() > 1 && s.frames.last().is_some_and(|f| f.depth > depth) {
            s.frames.pop();
        }
        let at = d.span.or_else(|| s.frames.last().and_then(|f| f.stop.as_ref().map(|(sp, _)| *sp))).unwrap_or_default();
        s.pause(it, Reason::Error(d.clone()), at);
        d
    })
}

/// The built-in functions that a debugger's `print` refuses: they would
/// change something outside of the values (output, files, input, chance).
pub(crate) fn has_effect(name: &str) -> bool {
    matches!(
        name,
        "print"
            | "debug"
            | "input"
            | "input_lines"
            | "exit"
            | "write_file"
            | "append_file"
            | "create_folder"
            | "delete_file"
            | "random"
            | "shuffle"
            | "random_seed"
            | "random_normal"
            | "random_choice_weighted"
            | "train_test_split"
            | "http_get"
            | "http_post"
    )
}

pub(crate) fn effect_refused(name: &str, span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "",
        t!(
            "`{name}` has an effect: the debugger only computes values, without changing anything",
            "`{name}` a un effet : le débogueur ne fait que calculer des valeurs, sans rien changer"
        ),
    )
    .at(span)
    .boxed()
}

// ----- breakpoints -----------------------------------------------------------

/// A breakpoint on a line.
pub struct LineBreak {
    pub id: u32,
    /// The file, by the name it was loaded with (kept when the program is loaded again).
    pub file_name: String,
    pub file: u32,
    pub line: usize,
}

/// The breakpoints, and the lines where the program may pause.
#[derive(Default)]
pub struct Breakpoints {
    pub lines: Vec<LineBreak>,
    pub functions: Vec<(u32, String)>,
    next: u32,
    /// The lines where the program may pause, by file.
    stop_lines: HashMap<u32, BTreeSet<usize>>,
    /// The names of the functions of the program.
    function_names: BTreeSet<String>,
    /// The files of the program: id and name, the program first.
    pub files: Vec<(u32, String)>,
}

impl Breakpoints {
    /// Learns the places where the program may pause, from its files (the
    /// program first); the breakpoints kept from a previous run follow
    /// their files.
    pub fn learn(&mut self, modules: &[Rc<Module>]) {
        self.stop_lines.clear();
        self.function_names.clear();
        self.files.clear();
        let mut lines = BTreeSet::new();
        for m in modules {
            for item in &m.items {
                walk_item(item, &mut lines, &mut self.function_names);
            }
            if let Some(file) = module_file(m) {
                if !self.files.iter().any(|(f, _)| *f == file) {
                    self.files.push((file, m.file.to_string()));
                }
            }
        }
        for (file, line) in lines {
            self.stop_lines.entry(file).or_default().insert(line);
        }
        for b in &mut self.lines {
            if let Some((id, _)) = self.files.iter().find(|(_, n)| *n == b.file_name) {
                b.file = *id;
            }
        }
    }

    /// The file named `name` (or whose name ends with it): `geometry.ry`.
    pub fn find_file(&self, name: &str) -> Option<u32> {
        let name = name.replace('\\', "/");
        let found = self.files.iter().find(|(_, n)| n.replace('\\', "/") == name).or_else(|| {
            self.files.iter().find(|(_, n)| {
                let n = n.replace('\\', "/");
                n.ends_with(&format!("/{name}")) || Path::new(&n).file_name().is_some_and(|f| f.to_string_lossy() == name)
            })
        });
        found.map(|(id, _)| *id)
    }

    /// The file whose path is `path` (an editor gives absolute paths).
    pub fn file_of_path(&self, path: &Path) -> Option<u32> {
        let wanted = path.canonicalize().ok()?;
        self.files.iter().find(|(_, n)| Path::new(n).canonicalize().ok().as_deref() == Some(&wanted)).map(|(id, _)| *id)
    }

    fn file_name(&self, file: u32) -> String {
        self.files.iter().find(|(f, _)| *f == file).map(|(_, n)| n.clone()).unwrap_or_default()
    }

    /// The first line, from `line` on, where the program may pause.
    pub fn line_with_code(&self, file: u32, line: usize) -> Option<usize> {
        self.stop_lines.get(&file)?.range(line..).next().copied()
    }

    /// Adds a breakpoint on a line, or on the next one with code: its
    /// number and its line.
    pub fn add_line(&mut self, file: u32, line: usize) -> Result<(u32, usize), String> {
        let name = self.file_name(file);
        let Some(at) = self.line_with_code(file, line) else {
            return Err(t!(
                "no code where the program can pause on line {line} of {name} or after it",
                "pas de code où le programme peut faire une pause à la ligne {line} de {name} ni après"
            ));
        };
        if let Some(b) = self.lines.iter().find(|b| b.file == file && b.line == at) {
            return Ok((b.id, at));
        }
        self.next += 1;
        self.lines.push(LineBreak { id: self.next, file_name: name, file, line: at });
        Ok((self.next, at))
    }

    /// Adds a breakpoint at the start of the function `name`.
    pub fn add_function(&mut self, name: &str) -> Result<u32, String> {
        let own = name.rsplit('.').next().unwrap_or(name);
        if !self.function_names.contains(own) {
            let mut msg = t!("no function named `{name}` in the program", "aucune fonction nommée `{name}` dans le programme");
            if let Some(best) = crate::suggest::closest(own, self.function_names.iter().map(String::as_str)) {
                msg.push_str(&t!(": did you mean `{best}`?", " : vouliez-vous dire `{best}` ?"));
            }
            return Err(msg);
        }
        if let Some((id, _)) = self.functions.iter().find(|(_, n)| n == own) {
            return Ok(*id);
        }
        self.next += 1;
        self.functions.push((self.next, own.to_string()));
        Ok(self.next)
    }

    /// Removes a breakpoint by its number.
    pub fn remove(&mut self, id: u32) -> bool {
        let before = self.lines.len() + self.functions.len();
        self.lines.retain(|b| b.id != id);
        self.functions.retain(|(n, _)| *n != id);
        before != self.lines.len() + self.functions.len()
    }

    pub fn clear_file(&mut self, file: u32) {
        self.lines.retain(|b| b.file != file);
    }

    pub fn clear_functions(&mut self) {
        self.functions.clear();
    }

    fn at(&self, file: u32, line: usize) -> Vec<u32> {
        self.lines.iter().filter(|b| b.file == file && b.line == line).map(|b| b.id).collect()
    }

    fn function(&self, name: &str) -> Option<u32> {
        self.functions.iter().find(|(_, n)| n == name).map(|(id, _)| *id)
    }
}

/// The id of the source of a module (a span of its items tells it, or its name).
fn module_file(m: &Module) -> Option<u32> {
    let mut id = 0;
    let mut found = None;
    while let Some(f) = source::get(id) {
        if *f.name == *m.file {
            found = Some(id);
        }
        id += 1;
    }
    found
}

// The places where the program may pause, as `compile` puts them (see `Compiler::stop_here`).

fn walk_item(item: &Item, lines: &mut BTreeSet<(u32, usize)>, fns: &mut BTreeSet<String>) {
    if let Item::Stmt(s) = item {
        match s {
            Stmt::Let(_, e) | Stmt::Expr(e) => {
                stop_at(e, e.span, lines);
                walk(e, lines, fns);
            }
            Stmt::Fn(def) => walk_fn(def, lines, fns),
        }
    }
}

fn stop_at(e: &Expr, at: Span, lines: &mut BTreeSet<(u32, usize)>) {
    if !matches!(e.kind, ExprKind::Block(_)) {
        lines.insert(line_of(at));
    }
}

fn walk_fn(def: &FnDef, lines: &mut BTreeSet<(u32, usize)>, fns: &mut BTreeSet<String>) {
    if let Some(name) = def.name {
        fns.insert(name.name().to_string());
    }
    for c in &def.clauses {
        stop_at(&c.body, c.body.span, lines);
        if let Some(g) = &c.guard {
            walk(g, lines, fns);
        }
        walk(&c.body, lines, fns);
    }
}

fn walk(e: &Expr, lines: &mut BTreeSet<(u32, usize)>, fns: &mut BTreeSet<String>) {
    use crate::ast::{ListItem, RecordItem, TemplatePart};
    match &e.kind {
        ExprKind::Unit
        | ExprKind::Bool(_)
        | ExprKind::Int(_)
        | ExprKind::Big(_)
        | ExprKind::Float(_)
        | ExprKind::Text(_)
        | ExprKind::Symbol(_)
        | ExprKind::Var(_)
        | ExprKind::Ctor(_) => {}
        ExprKind::Template(parts) => {
            for p in parts {
                if let TemplatePart::Expr(x) = p {
                    walk(x, lines, fns);
                }
            }
        }
        ExprKind::List(items) => {
            for it in items {
                let (ListItem::One(x) | ListItem::Spread(x)) = it;
                walk(x, lines, fns);
            }
        }
        ExprKind::Record(items) => {
            for it in items {
                let (RecordItem::Field(_, _, x) | RecordItem::Spread(x)) = it;
                walk(x, lines, fns);
            }
        }
        ExprKind::Lambda(def) => walk_fn(def, lines, fns),
        ExprKind::Call(f, args) | ExprKind::Method(f, _, args, _) => {
            walk(f, lines, fns);
            args.iter().for_each(|a| walk(a, lines, fns));
        }
        ExprKind::Field(x, _, _) | ExprKind::Unary(_, x) | ExprKind::Try(x) => walk(x, lines, fns),
        ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => {
            walk(a, lines, fns);
            walk(b, lines, fns);
        }
        ExprKind::If(c, a, b) => {
            walk(c, lines, fns);
            for x in [a, b] {
                stop_at(x, x.span, lines);
                walk(x, lines, fns);
            }
        }
        ExprKind::Match(subject, arms) => {
            walk(subject, lines, fns);
            for arm in arms {
                if let Some(g) = &arm.guard {
                    walk(g, lines, fns);
                }
                stop_at(&arm.body, arm.body.span, lines);
                walk(&arm.body, lines, fns);
            }
        }
        ExprKind::Block(stmts) => {
            for s in stmts {
                match s {
                    Stmt::Let(p, x) => {
                        let is_fn = matches!((&p.kind, &x.kind), (PatKind::Bind(_), ExprKind::Lambda(_)));
                        if !is_fn {
                            stop_at(x, p.span.to(x.span), lines);
                        }
                        if let (PatKind::Bind(name), ExprKind::Lambda(_)) = (&p.kind, &x.kind) {
                            fns.insert(name.name().to_string());
                        }
                        walk(x, lines, fns);
                    }
                    Stmt::Expr(x) => {
                        stop_at(x, x.span, lines);
                        walk(x, lines, fns);
                    }
                    Stmt::Fn(def) => walk_fn(def, lines, fns),
                }
            }
        }
    }
}

// ----- running a program ---------------------------------------------------

/// How a run under the debugger ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// To its end.
    Finished,
    /// Stopped by an error (already reported).
    Failed,
}

/// A program loaded for the debugger.
pub struct Program {
    pub driver: Driver,
    pub module: Rc<Module>,
}

impl Program {
    /// Loads and checks a program and the files it uses (problems are reported).
    pub fn load(path: &Path, args: Vec<String>, color: bool) -> Option<Program> {
        let mut driver = Driver::new();
        driver.color = color;
        driver.interp.script_args = args;
        let module = driver.load_file(path)?;
        Some(Program { driver, module })
    }

    /// The modules of the program, the program first.
    pub fn modules(&self) -> Vec<Rc<Module>> {
        let mut out = vec![self.module.clone()];
        for m in self.driver.modules() {
            if !Rc::ptr_eq(&m, &self.module) {
                out.push(m);
            }
        }
        out
    }

    /// Runs the program under the debugger; gives back the session (with
    /// its breakpoints) and how it ended.
    pub fn run(&mut self, mut session: Session) -> (Outcome, Session) {
        session.globals = crate::modules::top_names(&self.module.items).into_iter().map(|(s, _)| s).collect();
        let it = &mut self.driver.interp;
        it.watched = true;
        it.steps_left = 0;
        it.debugger = Some(Box::new(session));
        let outcome = match self.driver.prepare(&self.module, Mode::Run) {
            Err(_) => Outcome::Failed,
            Ok(()) => match self.driver.interp.exec_items(&self.module.items, Mode::Run) {
                Ok(_) => Outcome::Finished,
                Err(d) => {
                    self.driver.report(&d);
                    Outcome::Failed
                }
            },
        };
        let it = &mut self.driver.interp;
        it.watched = false;
        it.steps_left = u64::MAX;
        let session = *it.debugger.take().expect("the session");
        (outcome, session)
    }
}
