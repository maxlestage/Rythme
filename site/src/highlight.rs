//! Colors for Rythme code and for the interpreter's messages, the same classes as the
//! playground's editor (`t-kw`, `t-str`…), so that one stylesheet paints both.

use paquets::prelude::*;

pub const KEYWORDS: &[&str] =
    &["fn", "if", "then", "else", "match", "end", "do", "type", "test", "use", "and", "or", "not", "true", "false", "unless"];

pub const BUILTINS: &[&str] = &[
    "print",
    "debug",
    "text",
    "input",
    "input_lines",
    "type_of",
    "help",
    "assert",
    "assert_eq",
    "fail",
    "args",
    "options",
    "env",
    "exit",
    "time",
    "random",
    "abs",
    "min",
    "max",
    "sqrt",
    "pow",
    "floor",
    "ceil",
    "round",
    "int",
    "float",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "log",
    "exp",
    "is_even",
    "is_odd",
    "gcd",
    "lcm",
    "len",
    "upper",
    "lower",
    "capitalize",
    "trim",
    "pad_left",
    "pad_right",
    "split",
    "join",
    "contains",
    "starts_with",
    "ends_with",
    "replace",
    "chars",
    "lines",
    "char_code",
    "char",
    "is_digit",
    "is_letter",
    "is_space",
    "repeat",
    "parse_int",
    "parse_float",
    "parse_json",
    "to_json",
    "trim_start",
    "trim_end",
    "words",
    "equals_ignore_case",
    "format_number",
    "matches",
    "find_first",
    "find_all",
    "captures",
    "captures_all",
    "named_captures",
    "split_by",
    "replace_all",
    "map",
    "filter",
    "fold",
    "scan",
    "each",
    "sum",
    "product",
    "first",
    "last",
    "rest",
    "get",
    "push",
    "set_at",
    "insert_at",
    "remove_at",
    "reverse",
    "sort",
    "sort_by",
    "unique",
    "shuffle",
    "slice",
    "take",
    "drop",
    "take_while",
    "drop_while",
    "step_by",
    "chunks",
    "windows",
    "zip",
    "enumerate",
    "any",
    "all",
    "find",
    "count",
    "reject",
    "each_with_index",
    "index_of",
    "min_by",
    "max_by",
    "flatten",
    "flat_map",
    "partition",
    "group_by",
    "is_empty",
    "keys",
    "values",
    "entries",
    "from_entries",
    "has",
    "put",
    "remove",
    "map_values",
    "dict",
    "set",
    "union",
    "intersection",
    "difference",
    "is_subset",
    "unwrap",
    "unwrap_or",
    "is_some",
    "is_none",
    "is_ok",
    "is_err",
    "and_then",
    "ok_or",
    "map_err",
    "read_file",
    "write_file",
    "append_file",
    "file_exists",
    "file_info",
    "list_folder",
    "create_folder",
    "delete_file",
    "join_path",
    "split_path",
    "now",
    "today",
    "parse_date",
    "format_date",
    "is_date",
    "add_days",
    "add_seconds",
    "days_between",
    "seconds_between",
    "day_of_week",
    "days_in_month",
    "is_leap_year",
    "check_all",
    "sample",
    "ints",
    "floats",
    "texts",
    "bools",
    "lists_of",
    "one_of",
    "records_of",
    "pi",
];

/// `(class, text)` pieces of `code`; `None` is plain text.
pub fn tokenize(code: &str) -> Vec<(Option<&'static str>, String)> {
    let chars: Vec<char> = code.chars().collect();
    let n = chars.len();
    let mut out: Vec<(Option<&'static str>, String)> = Vec::new();
    let mut push = |class: Option<&'static str>, text: String| match out.last_mut() {
        Some((c, t)) if *c == class => t.push_str(&text),
        _ => out.push((class, text)),
    };
    let take = |from: usize, to: usize| chars[from..to].iter().collect::<String>();
    let mut i = 0;
    while i < n {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if c == '#' {
            let mut j = i;
            while j < n && chars[j] != '\n' {
                j += 1;
            }
            push(Some("t-com"), take(i, j));
            i = j;
        } else if c == '"' {
            let mut j = i + 1;
            while j < n && chars[j] != '"' {
                j += if chars[j] == '\\' { 2 } else { 1 };
            }
            let j = (j + 1).min(n);
            push(Some("t-str"), take(i, j));
            i = j;
        } else if c.is_ascii_digit() {
            let mut j = i + 1;
            while j < n
                && (chars[j].is_ascii_hexdigit()
                    || matches!(chars[j], 'x' | 'X' | '_')
                    || (chars[j] == '.' && chars.get(j + 1).is_some_and(char::is_ascii_digit)))
            {
                j += 1;
            }
            push(Some("t-num"), take(i, j));
            i = j;
        } else if c.is_alphabetic() || c == '_' {
            let mut j = i + 1;
            while j < n && (chars[j].is_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            let word = take(i, j);
            let class = if KEYWORDS.contains(&word.as_str()) {
                Some("t-kw")
            } else if c.is_uppercase() {
                Some("t-ctor")
            } else if BUILTINS.contains(&word.as_str()) {
                Some("t-fn")
            } else {
                None
            };
            push(class, word);
            i = j;
        } else if (c == '|' || c == '-') && next == Some('>') {
            push(Some("t-op"), take(i, i + 2));
            i += 2;
        } else {
            push(None, c.to_string());
            i += 1;
        }
    }
    out
}

fn pieces(parts: Vec<(Option<&'static str>, String)>) -> Vec<Node> {
    parts
        .into_iter()
        .map(|(class, text)| match class {
            Some(class) => span().class(class).text(text).into(),
            None => Node::from(text),
        })
        .collect()
}

/// `<pre class="code-block"><code>…</code></pre>` with colored Rythme code.
pub fn code_block(source: &str) -> Element {
    pre().class("code-block").child(code().attr("translate", "no").children(pieces(tokenize(source.trim_end_matches('\n')))))
}

/// `<code>` with colored Rythme code, inside a sentence or a table.
pub fn code_inline(source: &str) -> Element {
    code().class("code-inline").attr("translate", "no").children(pieces(tokenize(source)))
}

/// The class of a line of an error message, as the playground colors it.
fn line_class(line: &str) -> Option<&'static str> {
    let trimmed = line.trim_start();
    if line.starts_with("error") || line.starts_with("erreur") {
        Some("err")
    } else if line.starts_with("warning") || line.starts_with("avertissement") {
        Some("warn")
    } else if trimmed.starts_with("= help") || trimmed.starts_with("= aide") {
        Some("help")
    } else if trimmed.starts_with('=') || trimmed.starts_with("-->") {
        Some("note")
    } else {
        None
    }
}

/// An error message of the interpreter, colored like in a terminal.
pub fn diagnostic(message: &str) -> Element {
    let mut nodes: Vec<Node> = Vec::new();
    for line in message.trim_end_matches('\n').lines() {
        // Underline marks: `  |       ^^^^^ label`.
        let marks = line.find('|').filter(|_| line.trim_start().starts_with('|')).and_then(|bar| {
            let rest = &line[bar + 1..];
            let at = rest.find(|c: char| c != ' ')?;
            rest[at..].starts_with('^').then(|| (&line[..bar + 1 + at], &rest[at..]))
        });
        match (marks, line_class(line)) {
            (Some((before, after)), _) => {
                nodes.push(span().class("note").text(before.to_owned()).into());
                nodes.push(span().class("mark").text(after.to_owned()).into());
            }
            (None, Some(class)) => nodes.push(span().class(class).text(line.to_owned()).into()),
            (None, None) => nodes.push(span().class("src").text(line.to_owned()).into()),
        }
        nodes.push(Node::from("\n"));
    }
    nodes.pop();
    pre().class("output output--diagnostic").child(code().attr("translate", "no").children(nodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_every_kind() {
        let parts = tokenize("fn f(x) = map(x, Some) |> print # done\n\"a\" 3.5");
        let class_of =
            |text: &str| parts.iter().find(|(_, t)| t == text).map(|(c, _)| *c).unwrap_or_else(|| panic!("{text} not found in {parts:?}"));
        assert_eq!(class_of("fn"), Some("t-kw"));
        assert_eq!(class_of("map"), Some("t-fn"));
        assert_eq!(class_of("Some"), Some("t-ctor"));
        assert_eq!(class_of("|>"), Some("t-op"));
        assert_eq!(class_of("# done"), Some("t-com"));
        assert_eq!(class_of("\"a\""), Some("t-str"));
        assert_eq!(class_of("3.5"), Some("t-num"));
    }

    #[test]
    fn keeps_the_text() {
        let code = "fn carré(x) = x * x # « accents »\nprint(carré(4))";
        let joined: String = tokenize(code).into_iter().map(|(_, t)| t).collect();
        assert_eq!(joined, code);
    }
}
