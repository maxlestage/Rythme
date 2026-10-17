//! Where the output of a program goes: the terminal, or a buffer when Rythme
//! runs inside a web page (the playground).

use std::cell::RefCell;
use std::io::Write;

/// Start and end of an error message inside captured output.
pub const ERR_START: char = '\u{1}';
pub const ERR_END: char = '\u{2}';

thread_local! {
    static CAPTURE: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// From now on, output is kept in a buffer instead of being printed.
pub fn capture() {
    CAPTURE.with(|c| *c.borrow_mut() = Some(String::new()));
}

/// Stops capturing and returns what was written. Error messages are wrapped
/// in `ERR_START` ... `ERR_END`.
pub fn take_captured() -> String {
    CAPTURE.with(|c| c.borrow_mut().take().unwrap_or_default())
}

fn captured(f: impl FnOnce(&mut String)) -> bool {
    CAPTURE.with(|c| match c.borrow_mut().as_mut() {
        Some(buf) => {
            f(buf);
            true
        }
        None => false,
    })
}

/// Writes text on the standard output.
pub fn out(text: &str) {
    if !captured(|b| b.push_str(text)) {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        let _ = lock.write_all(text.as_bytes());
        let _ = lock.flush();
    }
}

pub fn out_line(text: &str) {
    if !captured(|b| {
        b.push_str(text);
        b.push('\n');
    }) {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        let _ = writeln!(lock, "{text}");
    }
}

/// Writes text on the error output.
pub fn err(text: &str) {
    if !captured(|b| {
        b.push(ERR_START);
        b.push_str(text);
        b.push(ERR_END);
    }) {
        let _ = std::io::stdout().flush();
        eprint!("{text}");
    }
}

/// Current time in seconds.
#[cfg(not(target_arch = "wasm32"))]
pub fn now() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

/// Current time in seconds, given by the web page.
#[cfg(target_arch = "wasm32")]
pub fn now() -> f64 {
    #[link(wasm_import_module = "env")]
    extern "C" {
        fn rythme_now() -> f64;
    }
    // SAFETY: the playground page provides this function; it takes no argument.
    unsafe { rythme_now() / 1000.0 }
}
