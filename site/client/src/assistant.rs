//! *Copy for AI*, in the playground: the program and what its last run gave, as
//! Markdown for a chat with an assistant, after a line that gives the address of the
//! whole language (`llms-full.txt`): the code fenced as `rythme`, the command run (as
//! in a terminal) and its output or its errors fenced as `text`.

/// What the last run gave: its mode (of `rythme_run`), its seed, the expression of its
/// steps, the code it ran and its output.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Run {
    pub mode: u32,
    pub seed: Option<u64>,
    pub expr: String,
    pub code: String,
    pub text: String,
}

/// `text` fenced as `kind`, with more backticks than any run of them inside.
pub fn fenced(kind: &str, text: &str) -> String {
    let mut longest = 0;
    let mut run = 0;
    for c in text.chars() {
        run = if c == '`' { run + 1 } else { 0 };
        longest = longest.max(run);
    }
    let ticks = "`".repeat((longest + 1).max(3));
    format!("{ticks}{kind}\n{}\n{ticks}", text.trim_end_matches('\n'))
}

/// A JSON string, as JavaScript's `JSON.stringify` writes it.
pub fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The name of the program in the text: the name shown above the editor when it is one
/// of a `.ry` file (an example, an opened file), else `main.ry`.
pub fn file_name(shown: &str) -> String {
    let shown = shown.trim();
    let stem = shown.strip_suffix(".ry").unwrap_or("");
    let fits = (1..=80).contains(&stem.encode_utf16().count()) && !stem.contains(|c: char| c.is_whitespace() || c == '/' || c == '\\');
    if fits { shown.to_owned() } else { "main.ry".to_owned() }
}

/// The texts of the page it needs: the first line (with `{url}`), what says that
/// nothing was printed, and that there is no expression to show the steps of.
pub struct Texts<'a> {
    pub intro: &'a str,
    pub nothing: &'a str,
    pub steps_none: &'a str,
}

/// The Markdown for the assistant: `code` (named `shown`), and the output of `last`
/// when it ran this very code.
pub fn markdown(t: &Texts, llms: &str, shown: &str, code: &str, last: Option<&Run>) -> String {
    let name = file_name(shown);
    let mut parts = vec![t.intro.replace("{url}", llms), String::new(), format!("`{name}`:"), String::new(), fenced("rythme", code)];
    if let Some(run) = last.filter(|run| run.code == code) {
        let seed = run.seed.map(|s| format!(" --seed {s}")).unwrap_or_default();
        let command = match run.mode {
            1 => format!("rythme check {name}"),
            2 => format!("rythme test{seed} {name}"),
            5 if run.expr.is_empty() => format!("rythme steps {name}"),
            5 => format!("rythme steps {name} {}", json_string(&run.expr)),
            _ => format!("rythme run{seed} {name}"),
        };
        let text = if run.mode == 5 && run.text.contains('\u{5}') { t.steps_none } else { &run.text };
        let out: String = text.chars().filter(|c| !matches!(c, '\u{1}' | '\u{2}' | '\u{5}')).collect();
        let out = out.trim_end_matches('\n');
        parts.extend([String::new(), format!("`{command}`:"), String::new(), fenced("text", if out.is_empty() { t.nothing } else { out })]);
    }
    parts.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: Texts = Texts { intro: "Read {url}.", nothing: "(nothing)", steps_none: "none" };

    #[test]
    fn the_program_and_its_output() {
        let run = Run { mode: 2, seed: Some(42), code: "x\n".into(), text: "\u{1}error[E1]: no\u{2}\n\n".into(), ..Run::default() };
        assert_eq!(
            markdown(&T, "https://x/llms-full.txt", "a.ry", "x\n", Some(&run)),
            "Read https://x/llms-full.txt.\n\n`a.ry`:\n\n```rythme\nx\n```\n\n`rythme test --seed 42 a.ry`:\n\n```text\nerror[E1]: no\n```\n"
        );
        // Another program than the one that ran: no output.
        assert_eq!(markdown(&T, "u", "my file.ry", "y", Some(&run)), "Read u.\n\n`main.ry`:\n\n```rythme\ny\n```\n");
        let steps = Run { mode: 5, expr: "f(\"a\")".into(), code: "y".into(), text: "\u{5}".into(), seed: None };
        assert!(
            markdown(&T, "u", "main.ry", "y", Some(&steps)).ends_with("`rythme steps main.ry \"f(\\\"a\\\")\"`:\n\n```text\nnone\n```\n")
        );
    }

    #[test]
    fn fences_hold_backticks() {
        assert_eq!(fenced("text", "a ```` b\n\n"), "`````text\na ```` b\n`````");
        assert_eq!(fenced("text", ""), "```text\n\n```");
    }
}
