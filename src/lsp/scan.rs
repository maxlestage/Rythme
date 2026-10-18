//! What surrounds the cursor, read from the raw text: inside a comment, a
//! text or code, and the calls whose parentheses are still open. Unlike the
//! parser, this works on unfinished code (`print(map(xs, `).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    Code,
    Comment,
    /// Inside a text, between its quotes (not inside a `{...}`).
    Text,
}

/// A call whose `(` is open before the cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenCall {
    pub name: String,
    /// Byte offset of the name.
    pub at: usize,
    /// Commas already written between the parentheses.
    pub commas: usize,
    /// `x |> f(...)` or `x.f(...)`: the value on the left is the first argument.
    pub first_given: bool,
}

pub struct Scan {
    pub place: Place,
    /// Byte offset of the `"` that starts the text the cursor is in.
    pub text_start: Option<usize>,
    /// Innermost last.
    pub calls: Vec<OpenCall>,
}

enum Frame {
    Paren(Option<OpenCall>),
    Bracket,
    Brace,
    Text(usize),
    Insert,
}

pub fn is_name_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Reads `text` up to `offset`.
pub fn scan(text: &str, offset: usize) -> Scan {
    let mut end = offset.min(text.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let src = &text[..end];
    let mut stack: Vec<Frame> = Vec::new();
    let mut comment = false;
    // The last name read, if nothing but spaces follows it.
    let mut last: Option<OpenCall> = None;
    let mut chars = src.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if comment {
            if c == '\n' || c == '\r' {
                comment = false;
            }
            continue;
        }
        if let Some(Frame::Text(_)) = stack.last() {
            match c {
                '\\' => {
                    // `\#{` is written as it is.
                    if chars.next().is_some_and(|(_, e)| e == '#') && chars.peek().is_some_and(|&(_, b)| b == '{') {
                        chars.next();
                    }
                }
                '"' => {
                    stack.pop();
                }
                '{' => stack.push(Frame::Insert),
                _ => {}
            }
            continue;
        }
        if c.is_whitespace() {
            continue;
        }
        let name = last.take();
        match c {
            '#' => comment = true,
            '"' => stack.push(Frame::Text(i)),
            '(' => stack.push(Frame::Paren(name.map(|mut call| {
                call.commas = 0;
                call
            }))),
            '[' => stack.push(Frame::Bracket),
            '{' => stack.push(Frame::Brace),
            ')' => {
                if matches!(stack.last(), Some(Frame::Paren(_))) {
                    stack.pop();
                }
            }
            ']' => {
                if matches!(stack.last(), Some(Frame::Bracket)) {
                    stack.pop();
                }
            }
            '}' => {
                if matches!(stack.last(), Some(Frame::Brace | Frame::Insert)) {
                    stack.pop();
                }
            }
            ',' => {
                if let Some(Frame::Paren(Some(call))) = stack.last_mut() {
                    call.commas += 1;
                }
            }
            c if is_name_start(c) => {
                let mut j = i + c.len_utf8();
                while let Some(&(k, d)) = chars.peek() {
                    if !is_name_char(d) {
                        break;
                    }
                    j = k + d.len_utf8();
                    chars.next();
                }
                // `even?(`: the `?` is a part of the name.
                if src[j..].starts_with("?(") {
                    chars.next();
                    j += 1;
                }
                let before = src[..i].trim_end();
                let first_given = before.ends_with("|>") || (before.ends_with('.') && !before.ends_with(".."));
                last = Some(OpenCall { name: src[i..j].to_string(), at: i, commas: 0, first_given });
            }
            c if c.is_ascii_digit() => {
                while chars.peek().is_some_and(|&(_, d)| is_name_char(d)) {
                    chars.next();
                }
            }
            _ => {}
        }
    }
    let place = if comment {
        Place::Comment
    } else if matches!(stack.last(), Some(Frame::Text(_))) {
        Place::Text
    } else {
        Place::Code
    };
    let text_start = stack.iter().rev().find_map(|f| if let Frame::Text(i) = f { Some(*i) } else { None });
    let calls = stack.into_iter().filter_map(|f| if let Frame::Paren(call) = f { call } else { None }).collect();
    Scan { place, text_start, calls }
}

/// The name around a byte offset (or ending there), and its byte range.
pub fn word_at(text: &str, offset: usize) -> Option<(usize, usize)> {
    let mut offset = offset.min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    let start = text[..offset].char_indices().rev().take_while(|(_, c)| is_name_char(*c)).last().map_or(offset, |(i, _)| i);
    let end = text[offset..].char_indices().find(|(_, c)| !is_name_char(*c)).map_or(text.len(), |(i, _)| offset + i);
    let word = &text[start..end];
    (start < end && word.chars().next().is_some_and(is_name_start)).then_some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_calls() {
        let s = scan("print(map(xs, ", 14);
        assert_eq!(s.place, Place::Code);
        let names: Vec<(&str, usize)> = s.calls.iter().map(|c| (c.name.as_str(), c.commas)).collect();
        assert_eq!(names, [("print", 0), ("map", 1)]);
        let s = scan("xs |> filter(", 13);
        assert!(s.calls[0].first_given);
        let s = scan("f([1, 2], ", 10);
        assert_eq!(s.calls[0].commas, 1);
        let s = scan("print(\"a, {g(1, ", 16);
        assert_eq!(s.calls.last().map(|c| (c.name.as_str(), c.commas)), Some(("g", 1)));
    }

    #[test]
    fn places() {
        assert_eq!(scan("x = 1 # note", 12).place, Place::Comment);
        assert_eq!(scan("x = \"abc", 8).place, Place::Text);
        assert_eq!(scan("x = \"a{b", 8).place, Place::Code);
        assert_eq!(scan("x = \"a\\\"b\" + ", 13).place, Place::Code);
        assert_eq!(scan("é = \"è", 7).text_start, Some(5));
    }

    #[test]
    fn words() {
        let t = "été + x2";
        assert_eq!(word_at(t, 2), Some((0, 5)));
        assert_eq!(word_at(t, 5), Some((0, 5)));
        assert_eq!(word_at(t, 9), Some((8, 10)));
        assert_eq!(word_at("12 + ", 1), None);
    }
}
