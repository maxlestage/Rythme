//! The steps of a calculation, as the playground shows them one at a time. The
//! interpreter writes them like `rythme steps`: the expression, then each step after
//! `  → ` (its other lines after four spaces). The other lines are what the program
//! printed meanwhile, shown with the step that follows them, and `  ... (…)` when steps
//! were left out. U+0005: no expression to show.

/// One step: its lines, what the program printed just before it, and whether steps were
/// left out between it and the one before.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Step {
    pub lines: Vec<String>,
    pub printed: Vec<String>,
    pub gap: bool,
}

/// `  ... (12 steps)`: steps left out.
fn is_gap(line: &str) -> bool {
    line.starts_with("  ... (")
}

/// A line that goes on the expression (the first step) rather than being printed: an
/// indented line, or `end`.
fn continues_expression(line: &str) -> bool {
    line.starts_with(char::is_whitespace)
        || line.strip_prefix("end").is_some_and(|rest| !rest.starts_with(|c: char| c.is_alphanumeric() || c == '_'))
}

/// The steps of `body` (the output of the interpreter without its messages).
pub fn parse(body: &str) -> Vec<Step> {
    let mut list: Vec<Step> = Vec::new();
    let mut pending: Vec<String> = Vec::new();
    if body.trim().is_empty() {
        return list;
    }
    let printed = |pending: &[String]| pending.iter().filter(|l| !is_gap(l)).cloned().collect::<Vec<_>>();
    for line in body.trim_end_matches('\n').split('\n') {
        if let Some(step) = line.strip_prefix("  → ") {
            list.push(Step { lines: vec![step.to_owned()], printed: printed(&pending), gap: pending.iter().any(|l| is_gap(l)) });
            pending.clear();
        } else if list.is_empty() {
            list.push(Step { lines: vec![line.to_owned()], ..Step::default() });
        } else if pending.is_empty() && list.len() > 1 && line.starts_with("    ") {
            list.last_mut().expect("a step").lines.push(line[4..].to_owned());
        } else if pending.is_empty() && list.len() == 1 && continues_expression(line) && !is_gap(line) {
            list[0].lines.push(line.to_owned());
        } else {
            pending.push(line.to_owned());
        }
    }
    if !pending.is_empty()
        && let Some(last) = list.last_mut()
    {
        last.printed.extend(printed(&pending));
    }
    list
}

/// The output of a run of the steps, cut in two: what is not a message (the steps and
/// what the program printed), and the errors, kept as messages (U+0001 … U+0002) to be
/// shown after the last step. The warnings are for Check: left out.
pub fn split(raw: &str) -> (String, String) {
    let mut body = String::new();
    let mut errors = String::new();
    let mut rest = raw;
    while !rest.is_empty() {
        match rest.find('\u{1}') {
            Some(at) if rest[at..].contains('\u{2}') => {
                body.push_str(&rest[..at]);
                let end = at + rest[at..].find('\u{2}').expect("the end of the message");
                let message = &rest[at..=end];
                if !crate::messages::is_warning(&message[1..]) {
                    errors.push_str(message);
                }
                rest = &rest[end + 1..];
            }
            _ => {
                body.push_str(rest);
                rest = "";
            }
        }
    }
    (body, errors)
}

/// What the live region says of a step: "Step 3 of 14: fib(2) + fib(1) + fib(2)".
pub fn spoken(count: &str, step: &Step) -> String {
    let joined = step.lines.join(" ");
    let words: Vec<&str> = joined.split_whitespace().collect();
    let text: String = words.join(" ").chars().take(300).collect();
    format!("{count}: {text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_with_what_is_printed_and_gaps() {
        let body = "fib(2)\n  → fib(1) + fib(0)\nhello\n  ... (3 steps)\n  → 1 + 0\n  → 1\n";
        let steps = parse(body);
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[0].lines, ["fib(2)"]);
        assert_eq!(steps[2], Step { lines: vec!["1 + 0".into()], printed: vec!["hello".into()], gap: true });
        assert_eq!(steps[3].lines, ["1"]);
        assert!(parse(" \n").is_empty());
    }

    #[test]
    fn steps_on_several_lines() {
        let body = "match x\n  1 -> a\nend\n  → do\n      b\n    end\nprinted\n";
        let steps = parse(body);
        assert_eq!(steps[0].lines, ["match x", "  1 -> a", "end"]);
        assert_eq!(steps[1].lines, ["do", "  b", "end"]);
        assert_eq!(steps[1].printed, ["printed"]);
    }

    #[test]
    fn errors_come_apart_and_warnings_go() {
        let (body, errors) = split("a\n\u{1}warning[W001]: w\u{2}b\n\u{1}error[E001]: e\n\u{2}");
        assert_eq!(body, "a\nb\n");
        assert_eq!(errors, "\u{1}error[E001]: e\n\u{2}");
        let step = Step { lines: vec!["fib(2)  +".into(), "  fib(1)".into()], ..Step::default() };
        assert_eq!(spoken("Step 1 of 3", &step), "Step 1 of 3: fib(2) + fib(1)");
    }
}
