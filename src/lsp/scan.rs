//! What surrounds the cursor, read from the raw text: inside a comment, a
//! text or code, and the calls whose parentheses are still open, or whose
//! line is, for a call without parentheses (`print x, `). Unlike the
//! parser, this works on unfinished code (`print(map(xs, `).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    Code,
    Comment,
    /// Inside a text, between its quotes (not inside a `{...}`).
    Text,
}

/// A call whose `(` is open before the cursor, or a call without
/// parentheses on the line of the cursor (`puts "a", `).
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
    /// A `(`, after the name of a call, and whether it holds the parameters
    /// of a function (`fn greet(`, `fn(`).
    Paren(Option<OpenCall>, bool),
    /// A call without parentheses, up to the end of its line.
    Command(OpenCall),
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
    // Nothing but spaces before, on this line (outside of brackets).
    let mut line_start = true;
    // A name that starts a line, followed by spaces: a call without
    // parentheses if the start of a value follows.
    let mut command: Option<OpenCall> = None;
    let mut last_starts_line = false;
    // Just read: the `)` of the parameters of a function, the name of a test,
    // `do` or `{` (the `|` of a block may follow), the `|x|` of a block.
    let mut fn_head = false;
    let mut test_name = false;
    let mut block_open = false;
    let mut block_params = false;
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
            if c == '\n' {
                if matches!(stack.last(), Some(Frame::Command(_))) {
                    stack.pop();
                }
                line_start = !stack.iter().any(|f| matches!(f, Frame::Paren(..) | Frame::Bracket));
                command = None;
            } else if last_starts_line && command.is_none() {
                // `puts ` at the start of a line.
                command = last.clone();
            }
            continue;
        }
        let starts_line = std::mem::replace(&mut line_start, false);
        let prev_starts_line = std::mem::replace(&mut last_starts_line, false);
        let name = last.take();
        let next = src[i + c.len_utf8()..].chars().next();
        // A result, where a call without parentheses may start as at the start
        // of a line: after `then`, `else`, `->`, the `=` of a function or a
        // test, and the parameters of a block (`do |x| puts x end`).
        let fn_eq = std::mem::replace(&mut fn_head, false) | std::mem::replace(&mut test_name, false);
        let block_bar = std::mem::replace(&mut block_open, false) && c == '|';
        if c == '=' && next != Some('=') && fn_eq {
            line_start = true;
        }
        if c == '-' && next == Some('>') {
            chars.next();
            command = None;
            line_start = true;
            continue;
        }
        if block_bar || (block_params && c == '|') {
            if next == Some('|') && block_bar {
                // `do || ...`: no parameters.
                chars.next();
                line_start = true;
            } else {
                block_params = !block_params;
                line_start = !block_params;
            }
            continue;
        }
        // `)`, `}`, `]`, `else` and `end` end the arguments of a call without parentheses.
        let word_end = src[i..].find(|d: char| !is_name_char(d)).map_or(src.len(), |k| i + k);
        if matches!(c, ')' | '}' | ']') || matches!(&src[i..word_end], "then" | "else" | "end" | "do") {
            while matches!(stack.last(), Some(Frame::Command(_))) {
                stack.pop();
            }
        }
        if let Some(call) = command.take() {
            let value = c == '"'
                || c == '['
                || c.is_ascii_digit()
                || is_name_start(c)
                || (c == ':' && next.is_some_and(is_name_start))
                || (c == '-' && next.is_some_and(|n| !n.is_whitespace()));
            if value {
                stack.push(Frame::Command(call));
            }
        }
        match c {
            '#' => comment = true,
            '"' => {
                // `test "name" = `: its name is read as a text.
                test_name = prev_starts_line && name.as_ref().is_some_and(|call| call.name == "test");
                stack.push(Frame::Text(i))
            }
            '(' => {
                let before = name.as_ref().map_or(&src[..i], |call| &src[..call.at]).trim_end();
                let fn_params = matches!(&name, Some(call) if call.name == "fn")
                    || (before.ends_with("fn") && !before[..before.len() - 2].ends_with(is_name_char));
                stack.push(Frame::Paren(
                    name.map(|mut call| {
                        call.commas = 0;
                        call
                    }),
                    fn_params,
                ))
            }
            '[' => stack.push(Frame::Bracket),
            '{' => {
                block_open = true;
                stack.push(Frame::Brace)
            }
            ')' => {
                if let Some(Frame::Paren(_, fn_params)) = stack.last() {
                    fn_head = *fn_params;
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
                if let Some(Frame::Paren(Some(call), _) | Frame::Command(call)) = stack.last_mut() {
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
                // `even? 4`: the `?` of a name at the start of a line.
                if starts_line && src[j..].starts_with("? ") {
                    chars.next();
                    if let Some(call) = last.as_mut() {
                        call.name.push('?');
                    }
                }
                // Only a name that starts a line (or a result) may be called
                // without parentheses.
                last_starts_line = starts_line;
                match &src[i..j] {
                    "then" | "else" => line_start = true,
                    "do" => {
                        line_start = true;
                        block_open = true;
                    }
                    _ => {}
                }
                if matches!(&src[i..j], "then" | "else" | "do" | "end") {
                    last = None;
                    last_starts_line = false;
                }
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
    // `puts ` with nothing after it yet.
    if let Some(call) = command {
        stack.push(Frame::Command(call));
    }
    let calls = stack
        .into_iter()
        .filter_map(|f| match f {
            Frame::Paren(call, _) => call,
            Frame::Command(call) => Some(call),
            _ => None,
        })
        .collect();
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
    fn calls_without_parentheses() {
        let names = |s: &Scan| s.calls.iter().map(|c| (c.name.clone(), c.commas)).collect::<Vec<_>>();
        assert_eq!(names(&scan("print x, ", 9)), [("print".to_string(), 1)]);
        assert_eq!(names(&scan("puts ", 5)), [("puts".to_string(), 0)]);
        assert_eq!(names(&scan("x = puts ", 9)), []);
        assert_eq!(names(&scan("print x\ny", 9)), []);
        assert_eq!(names(&scan("even? 4, ", 9)), [("even?".to_string(), 1)]);
        assert_eq!(names(&scan("puts f(1, ", 10)), [("puts".to_string(), 0), ("f".to_string(), 1)]);
        assert_eq!(names(&scan("puts (1, ", 9)), [("puts".to_string(), 1)]);
        // In a result: a branch, a case, the body of a function or a test, a block.
        let last = |text: &str| scan(text, text.len()).calls.last().map(|c| (c.name.clone(), c.commas));
        assert_eq!(last("if ok then puts 1, "), Some(("puts".to_string(), 1)));
        assert_eq!(last("if ok then puts 1 else print "), Some(("print".to_string(), 0)));
        assert_eq!(last("if ok then puts 1 else "), None);
        assert_eq!(last("  \"Dr \" + n -> puts "), Some(("puts".to_string(), 0)));
        assert_eq!(last("fn greet(n) = puts "), Some(("puts".to_string(), 0)));
        assert_eq!(last("f = fn(x) = puts x, "), Some(("puts".to_string(), 1)));
        assert_eq!(last("test \"a\" = assert_eq 1, "), Some(("assert_eq".to_string(), 1)));
        assert_eq!(last("xs.each { |x| puts "), Some(("puts".to_string(), 0)));
        assert_eq!(last("xs.each do |x, y| print x, "), Some(("print".to_string(), 1)));
        assert_eq!(last("xs.each { |x| puts x }"), None);
        assert_eq!(last("print(if ok then puts 1)"), None);
        // Not after a name given a value, nor in the middle of an expression.
        assert_eq!(last("x = puts "), None);
        assert_eq!(last("y = f(x) + puts "), None);
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
