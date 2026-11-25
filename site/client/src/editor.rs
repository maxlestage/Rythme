//! What the playground's editor needs besides its colors: the indentation of a new
//! line, the positions of the text field (counted in UTF-16, as the browser counts
//! them), and the names of the files it opens and saves.

/// Whether a line opens a block, so that the next line is indented by two more spaces:
/// a function's head without its body (`fn f(x)`, `fn f(x) if x > 0`), a test's name
/// (`test "a"`), a line that ends with `do`, a `match` on something, or `=`.
pub fn opens_block(line: &str) -> bool {
    let end = line.trim_end();
    if end.ends_with('=') {
        return true;
    }
    if ends_with_word(end, "do") || has_match(line) {
        return true;
    }
    let start = line.trim_start();
    if let Some(rest) = after_keyword(start, "test") {
        let rest = rest.trim_end();
        return rest.len() >= 2 && rest.starts_with('"') && rest.ends_with('"');
    }
    after_keyword(start, "fn").is_some_and(function_head)
}

/// ASCII letters, digits and `_`: the word characters of a regular expression.
fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// `text` ends with `word`, a word of its own.
fn ends_with_word(text: &str, word: &str) -> bool {
    text.strip_suffix(word).is_some_and(|before| !before.chars().next_back().is_some_and(is_word))
}

/// `match` then spaces then something, anywhere in the line, `match` a word of its own.
fn has_match(line: &str) -> bool {
    line.match_indices("match").any(|(at, _)| {
        let before = line[..at].chars().next_back();
        let rest = &line[at + 5..];
        !before.is_some_and(is_word) && rest.starts_with(char::is_whitespace) && rest.chars().count() >= 2
    })
}

/// The text after `keyword` and at least one space.
fn after_keyword<'a>(text: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = text.strip_prefix(keyword)?;
    let trimmed = rest.trim_start();
    (trimmed.len() < rest.len()).then_some(trimmed)
}

/// A function's head after `fn `: a name, `(`, anything, `)`, then nothing or
/// ` if …`.
fn function_head(rest: &str) -> bool {
    for (open, _) in rest.match_indices('(') {
        let name = rest[..open].trim_end();
        if name.is_empty() || name.contains(char::is_whitespace) {
            continue;
        }
        for (close, _) in rest.match_indices(')').filter(|(c, _)| *c > open) {
            let after = &rest[close + 1..];
            if after.trim().is_empty() {
                return true;
            }
            let guard = after.trim_start();
            if guard.len() < after.len() && guard.strip_prefix("if").is_some_and(|g| g.starts_with(char::is_whitespace)) {
                return true;
            }
        }
    }
    false
}

/// The indentation for the line after `line`: its own, two spaces more after a line
/// that opens a block.
pub fn next_indent(line: &str) -> String {
    let own: String = line.chars().take_while(|c| c.is_whitespace()).collect();
    if opens_block(line) { own + "  " } else { own }
}

/// The byte offset of a position counted in UTF-16 units (the browser's way), clamped
/// to the text.
pub fn byte_at(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (at, c) in text.char_indices() {
        if units >= utf16 {
            return at;
        }
        units += c.len_utf16();
    }
    text.len()
}

/// The length of a text in UTF-16 units.
pub fn utf16_len(text: &str) -> usize {
    text.encode_utf16().count()
}

/// The name of an opened file, kept when it is a `.ry` file's (80 characters at most,
/// no folder), else `main.ry`.
pub fn opened_name(name: &str) -> String {
    let lower = name.to_lowercase();
    let stem_len = utf16_len(name).saturating_sub(3);
    if lower.ends_with(".ry") && (1..=80).contains(&stem_len) && !name.contains(['/', '\\']) { name.to_owned() } else { "main.ry".into() }
}

/// The name of a downloaded file: the name shown, its odd characters as `_`; `main.ry`
/// when it is not a `.ry` file.
pub fn download_name(shown: &str) -> String {
    let mut name = String::new();
    let mut odd = false;
    for c in shown.chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-') {
            name.push(c);
            odd = false;
        } else if !odd {
            name.push('_');
            odd = true;
        }
    }
    if name.ends_with(".ry") { name } else { "main.ry".into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_open_after_these_lines() {
        for line in ["fn f(x)", "  fn carré(x, y) if x > 0", "fn f(x) =", "test \"a b\"", "x = do", "  match x", "y = match  ", "fn f() "]
        {
            assert!(opens_block(line), "{line}");
        }
        for line in ["fn f(x) = x", "fn (x)", "test \"a", "undo", "rematch x", "print(1)", "fn f(x) if", "match"] {
            assert!(!opens_block(line), "{line}");
        }
        assert_eq!(next_indent("  fn f(x)"), "    ");
        assert_eq!(next_indent("\tprint(1)"), "\t");
    }

    #[test]
    fn positions_in_utf16() {
        let text = "é😀a";
        assert_eq!(byte_at(text, 0), 0);
        assert_eq!(byte_at(text, 1), 2);
        assert_eq!(byte_at(text, 3), 6);
        assert_eq!(byte_at(text, 99), text.len());
        assert_eq!(utf16_len(text), 4);
    }

    #[test]
    fn file_names() {
        assert_eq!(opened_name("hello.RY"), "hello.RY");
        assert_eq!(opened_name("a/b.ry"), "main.ry");
        assert_eq!(opened_name(".ry"), "main.ry");
        assert_eq!(opened_name("notes.txt"), "main.ry");
        assert_eq!(download_name("my file!!.ry"), "my_file_.ry");
        assert_eq!(download_name("hello.ry"), "hello.ry");
        assert_eq!(download_name(""), "main.ry");
    }
}
