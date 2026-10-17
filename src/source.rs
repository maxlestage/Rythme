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
}

impl SourceFile {
    /// 1-based line and column (in characters) of a byte offset.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.text.len());
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
}

thread_local! {
    static FILES: RefCell<Vec<Rc<SourceFile>>> = const { RefCell::new(Vec::new()) };
}

/// Registers a source file and returns its id.
pub fn add(name: &str, text: &str) -> u32 {
    let mut line_starts = vec![0];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            line_starts.push(i + 1);
        }
    }
    let file = SourceFile { name: name.to_string(), text: Rc::from(text), line_starts };
    FILES.with(|f| {
        let mut f = f.borrow_mut();
        f.push(Rc::new(file));
        (f.len() - 1) as u32
    })
}

pub fn get(id: u32) -> Option<Rc<SourceFile>> {
    FILES.with(|f| f.borrow().get(id as usize).cloned())
}

/// `file.ry:3:7` for a span.
pub fn location(span: Span) -> String {
    match get(span.file) {
        Some(f) => {
            let (l, c) = f.line_col(span.start as usize);
            format!("{}:{}:{}", f.name, l, c)
        }
        None => "?".to_string(),
    }
}
