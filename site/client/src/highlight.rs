//! The colors of Rythme code, the same on both sides: the server paints the code of the
//! pages with them (`rythme-site`'s `highlight.rs`), and the playground its editor and
//! the steps of a calculation. Each piece of code gets a class of `site.css`: `t-kw`
//! (keywords), `t-str` (texts), `t-num` (numbers), `t-com` (comments), `t-ctor`
//! (constructors), `t-fn` (built-in functions), `t-op` (`|>`, `->`).

/// The keywords of Rythme.
pub const KEYWORDS: &[&str] =
    &["fn", "if", "then", "else", "match", "end", "do", "type", "test", "use", "and", "or", "not", "true", "false", "unless"];

/// The built-in functions, in the order of the interpreter's `BUILTINS` (a test of the
/// interpreter checks it), colored as `t-fn`.
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
    "random_seed",
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
    "csv_parse",
    "csv_records",
    "csv_format",
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
    "tanh",
    "sigmoid",
    "relu",
    "dot",
    "norm",
    "vector_add",
    "vector_sub",
    "vector_mul",
    "vector_scale",
    "zip_with",
    "matmul",
    "transpose",
    "zeros",
    "ones",
    "identity_matrix",
    "shape",
    "stats",
    "variance",
    "std_dev",
    "percentile",
    "argmax",
    "argmin",
    "softmax",
    "normalize",
    "random_normal",
    "random_choice_weighted",
    "train_test_split",
    "derivative",
    "gradient",
    "plot",
    "histogram",
    "http_get",
    "http_post",
    "pi",
];

/// `(class, text)` pieces of `code`; `None` is plain text. Neighbours of the same class
/// are joined; joined, the pieces give `code` back.
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

/// `text` with `&`, `<` and `>` escaped, for HTML.
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// The HTML of the playground's editor (under its text field) for `code`: one
/// `<span>` per line, ending with its new line (which gives an empty line its height),
/// so that each line wraps like the same line of the field; a piece that spans lines (a
/// text on several lines) is closed at each new line.
pub fn editor_html(code: &str) -> String {
    let mut out = String::with_capacity(code.len() * 2);
    for (class, text) in tokenize(code) {
        for (k, part) in text.split('\n').enumerate() {
            if k > 0 {
                out.push('\n');
            }
            match class {
                Some(class) => {
                    out.push_str("<span class=\"");
                    out.push_str(class);
                    out.push_str("\">");
                    out.push_str(&escape(part));
                    out.push_str("</span>");
                }
                None => out.push_str(&escape(part)),
            }
        }
    }
    out.split('\n').map(|line| format!("<span>{line}\n</span>")).collect()
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

    #[test]
    fn the_editor_has_one_block_per_line() {
        let html = editor_html("x = \"a\nb\" < 2\n");
        assert_eq!(
            html,
            "<span>x = <span class=\"t-str\">\"a</span>\n</span><span><span class=\"t-str\">b\"</span> &lt; <span class=\"t-num\">2</span>\n</span><span>\n</span>"
        );
        assert_eq!(editor_html(""), "<span>\n</span>");
    }
}
