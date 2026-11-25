//! Source files and positions inside them.

use std::cell::RefCell;
use std::rc::Rc;

/// A range of bytes in a source file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Span {
    pub file: u32,
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(file: u32, start: usize, end: usize) -> Span {
        Span { file, start: start as u32, end: end as u32 }
    }

    /// The smallest span covering both `self` and `other`.
    pub fn to(self, other: Span) -> Span {
        if other.file != self.file {
            return self;
        }
        Span { file: self.file, start: self.start.min(other.start), end: self.end.max(other.end) }
    }
}

pub struct SourceFile {
    pub name: String,
    pub text: Rc<str>,
    line_starts: Vec<usize>,
    /// The lines from this one on (0-based) come from somewhere else, named
    /// so in messages: the expression of `rythme steps file.ry "fib(4)"`,
    /// read after the definitions of the file.
    tail: Option<(usize, String)>,
}

impl SourceFile {
    /// 1-based line and column (in characters) of a byte offset.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let mut offset = offset.min(self.text.len());
        while !self.text.is_char_boundary(offset) {
            offset -= 1;
        }
        let line = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let start = self.line_starts[line];
        let col = self.text[start..offset].chars().count() + 1;
        (line + 1, col)
    }

    /// Text of a 1-based line, without its line break.
    pub fn line_text(&self, line: usize) -> &str {
        let start = self.line_starts[line - 1];
        let end = self.line_starts.get(line).copied().unwrap_or(self.text.len());
        self.text[start..end].trim_end_matches(['\n', '\r'])
    }

    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// The name and the 1-based line shown for a 1-based line: the lines of
    /// an appended tail are counted from its start, under its own name.
    pub fn shown(&self, line: usize) -> (&str, usize) {
        match &self.tail {
            Some((first, name)) if line > *first => (name, line - first),
            _ => (&self.name, line),
        }
    }
}

thread_local! {
    static FILES: RefCell<Vec<Rc<SourceFile>>> = const { RefCell::new(Vec::new()) };
}

fn new_file(name: &str, text: &str) -> SourceFile {
    let mut line_starts = vec![0];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            line_starts.push(i + 1);
        }
    }
    SourceFile { name: name.to_string(), text: Rc::from(text), line_starts, tail: None }
}

/// Registers a source file whose text from byte `at` on (at the start of a
/// line) comes from somewhere else, named `tail` in messages.
pub fn add_with_tail(name: &str, text: &str, at: usize, tail: &str) -> u32 {
    let mut file = new_file(name, text);
    let first = file.line_starts.partition_point(|&s| s < at);
    file.tail = Some((first, tail.to_string()));
    FILES.with(|f| {
        let mut f = f.borrow_mut();
        f.push(Rc::new(file));
        (f.len() - 1) as u32
    })
}

/// Registers a source file and returns its id.
pub fn add(name: &str, text: &str) -> u32 {
    let file = new_file(name, text);
    FILES.with(|f| {
        let mut f = f.borrow_mut();
        f.push(Rc::new(file));
        (f.len() - 1) as u32
    })
}

/// Gives a registered file a new text. The language server keeps one id per
/// file and calls this at each edit, instead of registering a new file.
pub fn replace(id: u32, name: &str, text: &str) {
    let file = Rc::new(new_file(name, text));
    FILES.with(|f| {
        if let Some(slot) = f.borrow_mut().get_mut(id as usize) {
            *slot = file;
        }
    })
}

pub fn get(id: u32) -> Option<Rc<SourceFile>> {
    FILES.with(|f| f.borrow().get(id as usize).cloned())
}

/// The name of the file (or of its tail) where a span starts.
pub fn name_at(span: Span) -> Option<String> {
    let f = get(span.file)?;
    let line = f.line_col(span.start as usize).0;
    Some(f.shown(line).0.to_string())
}

/// `file.ry:3:7` for a span.
pub fn location(span: Span) -> String {
    match get(span.file) {
        Some(f) => {
            let (l, c) = f.line_col(span.start as usize);
            let (name, l) = f.shown(l);
            format!("{name}:{l}:{c}")
        }
        None => "?".to_string(),
    }
}
