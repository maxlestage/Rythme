//! The Rythme interpreter as a WebAssembly module, for the web playground.
//!
//! The page writes the code in memory obtained with `rythme_alloc`, calls
//! `rythme_run`, then reads `rythme_result_len()` bytes at the returned
//! address. Error messages are wrapped in \u{1} ... \u{2}.

use std::cell::RefCell;
use std::path::Path;

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

/// Runs the code. `mode`: 0 = run, 1 = check, 2 = test. `language`: 0 = English, 1 = French.
///
/// # Safety
/// `ptr` must come from `rythme_alloc(len)` and hold `len` bytes of UTF-8.
#[no_mangle]
pub unsafe extern "C" fn rythme_run(ptr: *mut u8, len: usize, mode: u32, language: u32) -> *const u8 {
    // SAFETY: the page allocated this buffer with `rythme_alloc(len)` and filled `len` bytes.
    let bytes = unsafe { Vec::from_raw_parts(ptr, len, len.max(1)) };
    let code = String::from_utf8_lossy(&bytes).into_owned();
    lang::set(if language == 1 { Lang::Fr } else { Lang::En });
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
