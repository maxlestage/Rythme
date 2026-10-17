//! Positions in documents and file addresses.
//!
//! Rythme counts bytes; editors count lines and "characters". What a
//! character is depends on the encoding agreed at start-up: UTF-16 code units
//! (the default of the protocol), UTF-8 bytes, or Unicode scalar values.
//! An accented letter is one UTF-16 unit but two bytes; an emoji is two
//! UTF-16 units but four bytes.

use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Utf16,
    Utf32,
}

impl Encoding {
    pub fn name(self) -> &'static str {
        match self {
            Encoding::Utf8 => "utf-8",
            Encoding::Utf16 => "utf-16",
            Encoding::Utf32 => "utf-32",
        }
    }

    pub fn parse(s: &str) -> Option<Encoding> {
        match s {
            "utf-8" => Some(Encoding::Utf8),
            "utf-16" => Some(Encoding::Utf16),
            "utf-32" => Some(Encoding::Utf32),
            _ => None,
        }
    }

    fn width(self, c: char) -> usize {
        match self {
            Encoding::Utf8 => c.len_utf8(),
            Encoding::Utf16 => c.len_utf16(),
            Encoding::Utf32 => 1,
        }
    }

    /// Length of a text in this encoding.
    pub fn len(self, s: &str) -> usize {
        match self {
            Encoding::Utf8 => s.len(),
            Encoding::Utf16 => s.chars().map(char::len_utf16).sum(),
            Encoding::Utf32 => s.chars().count(),
        }
    }
}

/// A position as the protocol sees it: 0-based line and character.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub line: u32,
    pub character: u32,
}

/// Where each line of a text starts. Lines end with `\n`, `\r\n` or `\r`, as
/// in the protocol.
pub struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> LineIndex {
        let mut starts = vec![0];
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'\n' => starts.push(i + 1),
                b'\r' if bytes.get(i + 1) != Some(&b'\n') => starts.push(i + 1),
                _ => {}
            }
            i += 1;
        }
        LineIndex { starts }
    }

    pub fn line_count(&self) -> usize {
        self.starts.len()
    }

    /// Byte range of a 0-based line, without its line break.
    pub fn line_range(&self, text: &str, line: usize) -> (usize, usize) {
        let start = self.starts.get(line).copied().unwrap_or(text.len()).min(text.len());
        let mut end = self.starts.get(line + 1).copied().unwrap_or(text.len()).min(text.len());
        let bytes = text.as_bytes();
        if end > start && bytes[end - 1] == b'\n' {
            end -= 1;
        }
        if end > start && bytes[end - 1] == b'\r' {
            end -= 1;
        }
        (start, end)
    }

    /// The position of a byte offset (moved back to a character boundary if needed).
    pub fn pos(&self, text: &str, offset: usize, enc: Encoding) -> Pos {
        let mut offset = offset.min(text.len());
        while !text.is_char_boundary(offset) {
            offset -= 1;
        }
        let line = match self.starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let (start, end) = self.line_range(text, line);
        // An offset inside a `\r\n` counts as the end of the line.
        let character = enc.len(&text[start..offset.min(end)]);
        Pos { line: line as u32, character: character as u32 }
    }

    /// The byte offset of a position. A character past the end of its line
    /// means the end of the line; a line past the end means the end of the text.
    pub fn offset(&self, text: &str, pos: Pos, enc: Encoding) -> usize {
        let line = pos.line as usize;
        if line >= self.starts.len() {
            return text.len();
        }
        let (start, end) = self.line_range(text, line);
        let mut units = 0usize;
        for (i, c) in text[start..end].char_indices() {
            if units >= pos.character as usize {
                return start + i;
            }
            units += enc.width(c);
            // In the middle of a character (half of an emoji in UTF-16): after it.
        }
        end
    }
}

/// `file:///home/ada/x.ry` → `/home/ada/x.ry`; `file:///c%3A/x.ry` → `c:/x.ry`.
pub fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    // `file://host/share/x.ry` (a Windows network share) or `file:///path`.
    let (host, path) = rest.split_at(rest.find('/')?);
    let path = percent_decode(path)?;
    if cfg!(windows) {
        let p = path.trim_start_matches('/');
        let is_drive = p.len() >= 2 && p.as_bytes()[1] == b':' && p.as_bytes()[0].is_ascii_alphabetic();
        if !host.is_empty() && host != "localhost" {
            return Some(PathBuf::from(format!("\\\\{host}{}", path.replace('/', "\\"))));
        }
        if is_drive {
            return Some(PathBuf::from(p.replace('/', "\\")));
        }
        return Some(PathBuf::from(path.replace('/', "\\")));
    }
    if !host.is_empty() && host != "localhost" {
        return None;
    }
    Some(PathBuf::from(path))
}

/// The `file://` address of a path.
pub fn path_to_uri(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    let mut out = String::from("file://");
    // `\\host\share` becomes `file://host/share`.
    let s = match s.strip_prefix("//") {
        Some(rest) => rest.to_string(),
        None if s.starts_with('/') => s,
        None => format!("/{s}"),
    };
    for (i, b) in s.bytes().enumerate() {
        let keep = b.is_ascii_alphanumeric() || matches!(b, b'/' | b'-' | b'_' | b'.' | b'~') || (b == b':' && i == 2);
        if keep {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn percent_decode(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// The last part of an address, for messages: `geometry.ry`.
pub fn short_name(uri: &str) -> String {
    let path = uri.split(['?', '#']).next().unwrap_or(uri);
    let last = path.rsplit('/').next().unwrap_or(path);
    percent_decode(last).unwrap_or_else(|| last.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_in_every_encoding() {
        let text = "x = \"é🎉\" + y\r\nsecond\rthird\n";
        let idx = LineIndex::new(text);
        assert_eq!(idx.line_count(), 4);
        let y = text.find('y').unwrap();
        assert_eq!(idx.pos(text, y, Encoding::Utf16), Pos { line: 0, character: 12 });
        assert_eq!(idx.pos(text, y, Encoding::Utf8), Pos { line: 0, character: 15 });
        assert_eq!(idx.pos(text, y, Encoding::Utf32), Pos { line: 0, character: 11 });
        for enc in [Encoding::Utf8, Encoding::Utf16, Encoding::Utf32] {
            let p = idx.pos(text, y, enc);
            assert_eq!(idx.offset(text, p, enc), y);
        }
        let third = text.find("third").unwrap();
        assert_eq!(idx.pos(text, third, Encoding::Utf16), Pos { line: 2, character: 0 });
        // Past the end of a line, past the end of the text.
        assert_eq!(idx.offset(text, Pos { line: 1, character: 99 }, Encoding::Utf16), text.find("\rthird").unwrap());
        assert_eq!(idx.offset(text, Pos { line: 99, character: 0 }, Encoding::Utf16), text.len());
        // Inside the emoji (half of a UTF-16 pair): after it.
        let emoji = text.find('🎉').unwrap();
        assert_eq!(idx.offset(text, Pos { line: 0, character: 7 }, Encoding::Utf16), emoji + 4);
    }

    #[test]
    fn uris() {
        if !cfg!(windows) {
            assert_eq!(uri_to_path("file:///home/ada/caf%C3%A9%20x.ry"), Some(PathBuf::from("/home/ada/café x.ry")));
            assert_eq!(path_to_uri(Path::new("/home/ada/café x.ry")), "file:///home/ada/caf%C3%A9%20x.ry");
        }
        assert_eq!(uri_to_path("untitled:Untitled-1"), None);
        assert_eq!(uri_to_path("file:///bad%zz"), None);
        assert_eq!(short_name("file:///a/b/g%C3%A9o.ry"), "géo.ry");
    }
}
