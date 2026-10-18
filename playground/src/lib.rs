//! The Rythme interpreter as a WebAssembly module, for the web playground.
//!
//! The page writes the code in memory obtained with `rythme_alloc`, calls
//! `rythme_run`, then reads `rythme_result_len()` bytes at the returned
//! address. Error messages are wrapped in \u{1} ... \u{2}.

use std::cell::RefCell;
use std::path::Path;

use rythme::ast::{Item, Stmt};
use rythme::driver::Driver;
use rythme::interp::Mode;
use rythme::lang::{self, Lang};
use rythme::value::Value;
use rythme::{io, t};

thread_local! {
    static RESULT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Memory for `len` bytes of code.
#[no_mangle]
pub extern "C" fn rythme_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len.max(1));
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Runs the code. `mode`: 0 = run, 1 = check, 2 = test, 3 = format (the
/// result is the formatted code, or error messages), 4 = steps (the
/// calculation shown step by step, like `rythme steps`), 5 = the steps of one
/// expression (the code is the program, U+0000, then the expression: the last
/// expression of the program when it is empty; U+0005 alone when there is no
/// expression to show). `language`: the
/// position of the language of the messages in `rythme::lang::LANGUAGES`
/// (0 = English, 1 = French, 2 = Spanish, 3 = German, 4 = Portuguese,
/// 5 = Chinese); English if there is none.
///
/// # Safety
/// `ptr` must come from `rythme_alloc(len)` and hold `len` bytes of UTF-8.
#[no_mangle]
pub unsafe extern "C" fn rythme_run(ptr: *mut u8, len: usize, mode: u32, language: u32) -> *const u8 {
    // SAFETY: the page allocated this buffer with `rythme_alloc(len)` and filled `len` bytes.
    let bytes = unsafe { Vec::from_raw_parts(ptr, len, len.max(1)) };
    let code = String::from_utf8_lossy(&bytes).into_owned();
    lang::set(Lang::all().nth(language as usize).unwrap_or(Lang::En));
    let output = run(&code, mode);
    RESULT.with(|r| {
        *r.borrow_mut() = output.into_bytes();
        r.borrow().as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn rythme_result_len() -> usize {
    RESULT.with(|r| r.borrow().len())
}

fn run(code: &str, mode: u32) -> String {
    if mode == 3 {
        return match rythme::format::format_named("main.ry", code) {
            Ok(formatted) => formatted,
            Err(diags) => {
                io::capture();
                for d in diags {
                    io::err(&format!("{}\n", d.render(false)));
                }
                io::take_captured()
            }
        };
    }
    if mode == 5 {
        let (program, expr) = code.rsplit_once('\0').unwrap_or((code, ""));
        return steps_of_one(program, expr);
    }
    // The browser's own call stack is the limit here, not the memory: about
    // 760 nested calls in a Chrome worker. Stop before, with a clear message.
    rythme::interp::set_max_depth(600);
    rythme::interp::set_stack_budget(12 << 20);
    io::capture();
    let mut driver = Driver::new();
    driver.color = false;
    let module = driver.load_source("main.ry", code, None, Path::new("."));
    match (module, mode) {
        (Some(_), 1) => {
            let n = driver.warnings;
            io::out_line(&if n == 0 {
                t!("✓ no problems found", "✓ aucun problème trouvé")
            } else {
                t!("✓ no errors, {n} warning(s)", "✓ aucune erreur, {n} avertissement(s)")
            });
        }
        (Some(m), 2) => {
            if driver.execute(&m, Mode::Test).is_ok() {
                driver.run_tests();
            }
        }
        (Some(m), 4) => {
            let _ = rythme::steps::run_module(&mut driver, &m, rythme::steps::Options::default(), None);
        }
        (Some(m), _) => {
            if let Ok(Some(v)) = driver.execute(&m, Mode::Run) {
                if !matches!(v, Value::Unit) {
                    io::out_line(&v.repr());
                }
            }
        }
        (None, _) => {}
    }
    io::take_captured()
}

/// The steps of one expression: `expr` computed after the definitions of the
/// program (like `rythme steps file.ry "expr"`), or the last expression of the
/// program. Only that expression is shown; the others of the program are
/// skipped, and its definitions are made first.
fn steps_of_one(program: &str, expr: &str) -> String {
    rythme::interp::set_max_depth(600);
    rythme::interp::set_stack_budget(12 << 20);
    io::capture();
    let mut driver = Driver::new();
    driver.color = false;
    let expr = expr.trim();
    let whole = if expr.is_empty() { program.to_string() } else { format!("{}\n{expr}\n", program.trim_end()) };
    if let Some(m) = driver.load_source("main.ry", &whole, None, Path::new(".")) {
        let last = m.items.iter().rposition(|item| matches!(item, Item::Stmt(Stmt::Let(..) | Stmt::Expr(_))));
        match last {
            Some(i) => {
                // Long lines wrap in the page, rather than being cut at 100 characters.
                let opts = rythme::steps::Options { width: 400, ..rythme::steps::Options::default() };
                let _ = rythme::steps::run_module(&mut driver, &m, opts, Some(m.items.len() - i));
            }
            None => io::out("\u{5}"),
        }
    }
    io::take_captured()
}
