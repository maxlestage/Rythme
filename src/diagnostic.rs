//! Error and warning messages, shown the way Rust shows them: with the
//! exact place in the code, an explanation, and help on how to fix it.

use crate::source::{self, Span};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    /// `E003`, `W001`... (`rythme explain E003` explains it in detail).
    pub code: &'static str,
    pub message: String,
    pub span: Option<Span>,
    pub label: String,
    pub secondary: Vec<(Span, String)>,
    pub notes: Vec<String>,
    pub helps: Vec<String>,
    /// Function calls that led to a runtime error, innermost first.
    pub trace: Vec<(String, Span)>,
    pub trace_omitted: usize,
    /// The input ended too early: the REPL should ask for another line.
    pub incomplete: bool,
}

pub type Fallible<T> = Result<T, Box<Diagnostic>>;

const MAX_TRACE: usize = 40;

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            code,
            message: message.into(),
            span: None,
            label: String::new(),
            secondary: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
            trace: Vec::new(),
            trace_omitted: 0,
            incomplete: false,
        }
    }

    pub fn warning(code: &'static str, message: impl Into<String>) -> Diagnostic {
        Diagnostic { severity: Severity::Warning, ..Diagnostic::error(code, message) }
    }

    pub fn at(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.helps.push(help.into());
        self
    }

    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn secondary(mut self, span: Span, label: impl Into<String>) -> Self {
        self.secondary.push((span, label.into()));
        self
    }

    pub fn incomplete(mut self) -> Self {
        self.incomplete = true;
        self
    }

    pub fn boxed(self) -> Box<Diagnostic> {
        Box::new(self)
    }

    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }

    pub fn push_frame(&mut self, function: String, call: Span) {
        if self.trace.len() < MAX_TRACE {
            self.trace.push((function, call));
        } else {
            self.trace_omitted += 1;
        }
    }

    pub fn render(&self, color: bool) -> String {
        let c = Colors::new(color);
        let mut out = String::new();
        let (word, sev_color) = match self.severity {
            Severity::Error => (t!("error", "erreur"), c.red),
            Severity::Warning => (t!("warning", "avertissement"), c.yellow),
        };
        if self.code.is_empty() {
            out.push_str(&format!("{}{}{word}{}", c.bold, sev_color, c.reset));
        } else {
            out.push_str(&format!("{}{}{word}[{}]{}", c.bold, sev_color, self.code, c.reset));
        }
        out.push_str(&format!("{}: {}{}\n", c.bold, self.message, c.reset));

        let mut max_line = 1;
        for span in self.span.iter().chain(self.secondary.iter().map(|(s, _)| s)) {
            if let Some(f) = source::get(span.file) {
                max_line = max_line.max(f.line_col(span.end as usize).0);
            }
        }
        let width = max_line.to_string().len();
        let pad = " ".repeat(width);

        if let Some(span) = self.span {
            if source::get(span.file).is_some() {
                out.push_str(&format!("{pad}{}-->{} {}\n", c.blue, c.reset, source::location(span)));
                out.push_str(&format!("{pad} {}|{}\n", c.blue, c.reset));
                snippet(&mut out, span, '^', &self.label, sev_color, width, &c);
            }
        }
        for (span, label) in &self.secondary {
            if source::get(span.file).is_some() {
                if self.span.map(|s| s.file) != Some(span.file) {
                    out.push_str(&format!("{pad}{}:::{} {}\n", c.blue, c.reset, source::location(*span)));
                }
                out.push_str(&format!("{pad} {}|{}\n", c.blue, c.reset));
                snippet(&mut out, *span, '-', label, c.blue, width, &c);
            }
        }
        if self.span.is_some() || !self.secondary.is_empty() {
            out.push_str(&format!("{pad} {}|{}\n", c.blue, c.reset));
        }
        for note in &self.notes {
            push_block(&mut out, &pad, &format!("{}note{}", c.bold, c.reset), note, &c);
        }
        for help in &self.helps {
            let word = t!("help", "aide");
            push_block(&mut out, &pad, &format!("{}{}{word}{}", c.bold, c.green, c.reset), help, &c);
        }
        if !self.trace.is_empty() {
            out.push_str(&format!(
                "{pad} {}={} {}{}{}\n",
                c.blue,
                c.reset,
                c.bold,
                t!("call trace (most recent first):", "appels en cours (le plus récent d'abord) :"),
                c.reset
            ));
            let mut i = 0;
            while i < self.trace.len() {
                let (name, span) = &self.trace[i];
                let mut n = 1;
                while i + n < self.trace.len() && self.trace[i + n].0 == *name {
                    n += 1;
                }
                let times = if n > 1 { format!(" (×{n})") } else { String::new() };
                out.push_str(&format!(
                    "{pad}     {} `{name}`{times} {} {}\n",
                    t!("in", "dans"),
                    t!("called at", "appelée en"),
                    source::location(*span)
                ));
                i += n;
            }
            if self.trace_omitted > 0 {
                out.push_str(&format!("{pad}     … {}\n", t!("and {} more calls", "et {} autres appels", self.trace_omitted)));
            }
        }
        if self.is_error() && !self.code.is_empty() {
            out.push_str(&format!(
                "{pad} {}={} {}\n",
                c.blue,
                c.reset,
                t!("more about this error: `rythme explain {}`", "en savoir plus : `rythme explain {}`", self.code)
            ));
        }
        out
    }
}

fn push_block(out: &mut String, pad: &str, head: &str, text: &str, c: &Colors) {
    let mut lines = text.lines();
    if let Some(first) = lines.next() {
        out.push_str(&format!("{pad} {}={} {head}: {first}\n", c.blue, c.reset));
    }
    for line in lines {
        out.push_str(&format!("{pad}     {line}\n"));
    }
}

fn snippet(out: &mut String, span: Span, mark: char, label: &str, color: &str, width: usize, c: &Colors) {
    let Some(file) = source::get(span.file) else { return };
    let (line, col) = file.line_col(span.start as usize);
    let (end_line, end_col) = file.line_col(span.end as usize);
    let text = file.line_text(line).replace('\t', " ");
    let line_len = text.chars().count();
    let len = if end_line == line { end_col.saturating_sub(col).max(1) } else { (line_len + 1).saturating_sub(col).max(1) };
    out.push_str(&format!("{}{:>width$} |{} {}\n", c.blue, line, c.reset, text));
    let marks: String = std::iter::repeat_n(mark, len).collect();
    let pad = " ".repeat(col - 1);
    if label.is_empty() {
        out.push_str(&format!("{}{} |{} {pad}{}{marks}{}\n", c.blue, " ".repeat(width), c.reset, color, c.reset));
    } else {
        out.push_str(&format!("{}{} |{} {pad}{}{marks} {label}{}\n", c.blue, " ".repeat(width), c.reset, color, c.reset));
    }
}

struct Colors {
    red: &'static str,
    yellow: &'static str,
    blue: &'static str,
    green: &'static str,
    bold: &'static str,
    reset: &'static str,
}

impl Colors {
    fn new(on: bool) -> Colors {
        if on {
            Colors { red: "\x1b[31m", yellow: "\x1b[33m", blue: "\x1b[34m", green: "\x1b[32m", bold: "\x1b[1m", reset: "\x1b[0m" }
        } else {
            Colors { red: "", yellow: "", blue: "", green: "", bold: "", reset: "" }
        }
    }
}

/// Colors are used when writing to a terminal, unless `NO_COLOR` is set.
pub fn use_color() -> bool {
    use std::io::IsTerminal;
    std::env::var_os("NO_COLOR").is_none() && std::io::stderr().is_terminal()
}
