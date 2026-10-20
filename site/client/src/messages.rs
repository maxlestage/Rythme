//! The output of the interpreter as the playground shows it, styled like in a terminal:
//! the messages (between U+0001 and U+0002) colored line by line, the code of an error
//! a button that explains it, the results of the tests in green and red.

/// The words that start the interpreter's messages, in every language of
/// `rythme::lang::LANGUAGES` (the site's tests check that none is missing). The server
/// colors the messages of its pages with them too.
pub const ERROR_WORDS: [&str; 5] = ["error", "erreur", "Fehler", "erro", "错误"];
pub const WARNING_WORDS: [&str; 6] = ["warning", "avertissement", "advertencia", "Warnung", "aviso", "警告"];
pub const HELP_WORDS: [&str; 6] = ["help", "aide", "ayuda", "Hilfe", "ajuda", "帮助"];

/// A piece of the output: a text with its class (`None`: plain text), or the code of
/// an error (`E003`), shown as a button that explains it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    Text(String, Option<&'static str>),
    Explain(String),
}

fn text(s: impl Into<String>, class: Option<&'static str>) -> Piece {
    Piece::Text(s.into(), class)
}

/// The line starts with one of `words`, then `[` or `:` (`error[E003]: …`).
fn starts_with_word(line: &str, words: &[&str]) -> bool {
    words.iter().any(|w| line.strip_prefix(w).is_some_and(|rest| rest.starts_with(['[', ':'])))
}

/// After the spaces of the start, `=` then spaces: the rest.
fn after_equals(line: &str) -> Option<&str> {
    line.trim_start().strip_prefix('=').map(str::trim_start)
}

/// A message that is a warning (`warning[W001]: …` in any language).
pub fn is_warning(message: &str) -> bool {
    starts_with_word(message, &WARNING_WORDS)
}

/// The class of a line of a message: `err`, `warn`, `help`, `note`, or none.
pub fn line_class(line: &str) -> Option<&'static str> {
    if starts_with_word(line, &ERROR_WORDS) {
        Some("err")
    } else if is_warning(line) {
        Some("warn")
    } else if after_equals(line).is_some_and(|rest| HELP_WORDS.iter().any(|w| rest.starts_with(w))) {
        Some("help")
    } else if after_equals(line).is_some() || line.trim_start().starts_with("-->") {
        Some("note")
    } else {
        None
    }
}

/// Whether the output of a run says it failed: an error, or a test that failed.
pub fn failed(output: &str) -> bool {
    ERROR_WORDS.iter().any(|w| output.contains(&format!("\u{1}{w}["))) || output.contains('✗')
}

/// The pieces of one line. In a message: the marks under the code (`  |   ^^^^ label`)
/// in the color of an error or of a warning, and `rythme explain E003` with its button.
fn line_pieces(line: &str, in_message: bool, warning: bool, out: &mut Vec<Piece>) {
    if !in_message {
        let start = line.trim_start();
        let class = if start.starts_with('✓') {
            Some("pass")
        } else if start.starts_with('✗') {
            Some("fail")
        } else {
            None
        };
        out.push(text(line, class));
        return;
    }
    // `  |    ^^^^ label`: the spaces, the bar and the spaces, then the marks.
    let after_spaces = line.trim_start();
    if let Some(rest) = after_spaces.strip_prefix('|') {
        let marks_at = rest.len() - rest.trim_start().len();
        let marks = &rest[marks_at..];
        let run = marks.len() - marks.trim_start_matches(['^', '-']).len();
        if run > 0 {
            let head = line.len() - marks.len();
            out.push(text(&line[..head], Some("note")));
            let class = if marks.starts_with('^') { if warning { "mark-warn" } else { "mark-err" } } else { "note" };
            out.push(text(marks, Some(class)));
            return;
        }
    }
    // The last `` `rythme explain E003` `` of the line.
    const EXPLAIN: &str = "`rythme explain ";
    if let Some(at) = line.rfind(EXPLAIN) {
        let code_at = at + EXPLAIN.len();
        let code = line.get(code_at..code_at + 4).unwrap_or("");
        let rest = line.get(code_at + 4..).unwrap_or("");
        let valid = code.len() == 4 && code.starts_with(['E', 'W']) && code[1..].bytes().all(|b| b.is_ascii_digit());
        if valid && rest.starts_with('`') {
            out.push(text(&line[..code_at], Some("note")));
            out.push(Piece::Explain(code.to_owned()));
            out.push(text(rest, Some("note")));
            return;
        }
    }
    out.push(text(line, Some(line_class(line).unwrap_or("src"))));
}

/// The pieces of the output `raw` (`None` when it shows nothing at all). Each message
/// (U+0001 … U+0002) ends with a new line; the output keeps its own.
pub fn pieces(raw: &str) -> Option<Vec<Piece>> {
    if raw.chars().all(|c| c == '\u{1}' || c == '\u{2}' || c.is_whitespace()) {
        return None;
    }
    let mut out = Vec::new();
    let mut rest = raw;
    while !rest.is_empty() {
        // The next message, or the plain text before it.
        let (part, in_message, next) = match rest.find('\u{1}') {
            Some(0) => match rest.find('\u{2}') {
                Some(end) => (&rest[1..end], true, &rest[end + 1..]),
                None => (rest, false, ""),
            },
            Some(at) if rest[at..].contains('\u{2}') => (&rest[..at], false, &rest[at..]),
            _ => (rest, false, ""),
        };
        rest = next;
        if part.is_empty() {
            continue;
        }
        let warning = is_warning(part);
        let body = part.trim_end_matches('\n');
        let lines: Vec<&str> = body.split('\n').collect();
        for (k, line) in lines.iter().enumerate() {
            line_pieces(line, in_message, warning, &mut out);
            if k + 1 < lines.len() || in_message {
                out.push(text("\n", None));
            }
        }
        if !in_message && part.ends_with('\n') {
            out.push(text("\n", None));
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shown(raw: &str) -> String {
        pieces(raw)
            .unwrap()
            .iter()
            .map(|p| match p {
                Piece::Text(t, Some(c)) => format!("[{c}:{t}]"),
                Piece::Text(t, None) => t.clone(),
                Piece::Explain(code) => format!("<{code}>"),
            })
            .collect()
    }

    #[test]
    fn colors_the_lines_of_a_message() {
        let raw = "hi\n\u{1}error[E003]: name\n --> main.ry:2:7\n  |\n2 | print(x)\n  |       ^ here\n  = help: did you mean `y`?\n  = note: `rythme explain E003` tells more\n\u{2}";
        assert_eq!(
            shown(raw),
            "hi\n[err:error[E003]: name]\n[note: --> main.ry:2:7]\n[src:  |]\n[src:2 | print(x)]\n[note:  |       ][mark-err:^ here]\n[help:  = help: did you mean `y`?]\n[note:  = note: `rythme explain ]<E003>[note:` tells more]\n"
        );
    }

    #[test]
    fn warnings_tests_and_nothing() {
        assert_eq!(shown("\u{1}Warnung[W001]: x\n  |  -- ^^\n\u{2}"), "[warn:Warnung[W001]: x]\n[note:  |  ][note:-- ^^]\n");
        assert_eq!(shown("\u{1}Warnung[W001]: x\n  |  ^^\n\u{2}"), "[warn:Warnung[W001]: x]\n[note:  |  ][mark-warn:^^]\n");
        assert_eq!(shown("  ✓ a\n  ✗ b\nend"), "[pass:  ✓ a]\n[fail:  ✗ b]\nend");
        assert_eq!(pieces("\u{1}\u{2}\n "), None);
        assert!(failed("\u{1}erreur[E001]: x\u{2}") && failed("✗ 1 test") && !failed("error[E001] in a text"));
        assert_eq!(line_class("aide: x"), None);
        assert_eq!(line_class("  = aide: x"), Some("help"));
        assert_eq!(line_class("errors are fine"), None);
    }
}
