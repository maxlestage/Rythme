//! Where the output of a program goes: the terminal, or a buffer when Rythme
//! runs inside a web page (the playground).

use std::cell::RefCell;
use std::io::Write;

/// Start and end of an error message inside captured output.
pub const ERR_START: char = '\u{1}';
pub const ERR_END: char = '\u{2}';

/// Where the output goes instead of the terminal: the text, and whether it
/// is an error message.
pub type Sink = Box<dyn Fn(&str, bool)>;

thread_local! {
    static CAPTURE: RefCell<Option<String>> = const { RefCell::new(None) };
    static SINK: RefCell<Option<Sink>> = const { RefCell::new(None) };
    static NO_INPUT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// From now on, output goes to `sink` (the debugger of an editor, whose
/// standard output carries its protocol).
pub fn set_sink(sink: Sink) {
    SINK.with(|s| *s.borrow_mut() = Some(sink));
}

fn sunk(text: &str, err: bool) -> bool {
    SINK.with(|s| match s.borrow().as_ref() {
        Some(sink) => {
            sink(text, err);
            true
        }
        None => false,
    })
}

/// `sunk` for a line: the line break is added only when there is a sink.
fn sunk_line(text: &str) -> bool {
    SINK.with(|s| match s.borrow().as_ref() {
        Some(sink) => {
            sink(&format!("{text}\n"), false);
            true
        }
        None => false,
    })
}

/// From now on, the program reads no input: `input` finds its end (the
/// standard input carries the protocol of the debugger).
pub fn close_input() {
    NO_INPUT.with(|n| n.set(true));
}

/// Is the input closed by `close_input`?
pub fn input_closed() -> bool {
    NO_INPUT.with(|n| n.get())
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

/// When the reader of the output went away (`rythme prog.ry | head`), stop
/// quietly instead of computing output that nobody will read.
fn stop_if_closed(result: std::io::Result<()>) {
    if let Err(e) = result {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
    }
}

/// Writes text on the standard output.
pub fn out(text: &str) {
    if !captured(|b| b.push_str(text)) && !sunk(text, false) {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        stop_if_closed(lock.write_all(text.as_bytes()).and_then(|_| lock.flush()));
    }
}

pub fn out_line(text: &str) {
    if !captured(|b| {
        b.push_str(text);
        b.push('\n');
    }) && !sunk_line(text)
    {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        stop_if_closed(writeln!(lock, "{text}"));
    }
}

/// Writes text on the error output. If it cannot be written (closed or full),
/// the text is lost, but the program goes on.
pub fn err(text: &str) {
    if !captured(|b| {
        b.push(ERR_START);
        b.push_str(text);
        b.push(ERR_END);
    }) && !sunk(text, true)
    {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().write_all(text.as_bytes());
    }
}

pub fn err_line(text: &str) {
    err(&format!("{text}\n"));
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

/// How many minutes the local time is ahead of UTC at the moment `secs`
/// (seconds since 1970), if the system tells: on Unix, from its time zone
/// settings (`TZ`, `/etc/localtime`).
#[cfg(all(unix, target_pointer_width = "64"))]
pub fn local_offset(secs: i64) -> Option<i64> {
    use std::os::raw::{c_int, c_long};
    /// The `struct tm` of C: its first nine fields are the same everywhere;
    /// `rest` leaves room for the fields that some systems add after them.
    #[repr(C)]
    struct Tm {
        sec: c_int,
        min: c_int,
        hour: c_int,
        mday: c_int,
        mon: c_int,
        year: c_int,
        wday: c_int,
        yday: c_int,
        isdst: c_int,
        rest: [u64; 8],
    }
    extern "C" {
        // On 64-bit Unix systems, `time_t` is a `long`.
        fn localtime_r(t: *const c_long, tm: *mut Tm) -> *mut Tm;
    }
    let t: c_long = secs;
    let mut tm = Tm { sec: 0, min: 0, hour: 0, mday: 0, mon: 0, year: 0, wday: 0, yday: 0, isdst: 0, rest: [0; 8] };
    // SAFETY: both pointers point to valid values of the right size, and
    // `localtime_r` writes only into `tm`.
    if unsafe { localtime_r(&t, &mut tm) }.is_null() {
        return None;
    }
    let days = crate::builtins::date::days_from_civil(i64::from(tm.year) + 1900, i64::from(tm.mon) + 1, i64::from(tm.mday));
    let local = days * 86_400 + i64::from(tm.hour) * 3600 + i64::from(tm.min) * 60 + i64::from(tm.sec);
    Some((local - secs).div_euclid(60))
}

/// How many minutes the local time is ahead of UTC, given by the web page.
#[cfg(target_arch = "wasm32")]
pub fn local_offset(secs: i64) -> Option<i64> {
    #[link(wasm_import_module = "env")]
    extern "C" {
        fn rythme_offset(ms: f64) -> f64;
    }
    // SAFETY: the playground page provides this function; it takes a number.
    let minutes = unsafe { rythme_offset(secs as f64 * 1000.0) };
    minutes.is_finite().then(|| minutes.round() as i64)
}

/// Elsewhere, the local offset is not known: dates are in UTC.
#[cfg(not(any(all(unix, target_pointer_width = "64"), target_arch = "wasm32")))]
pub fn local_offset(_secs: i64) -> Option<i64> {
    None
}
