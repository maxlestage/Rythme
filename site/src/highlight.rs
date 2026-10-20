//! Colors for Rythme code and for the interpreter's messages, the same classes as the
//! playground's editor (`t-kw`, `t-str`…), so that one stylesheet paints both. The
//! pieces of code come from the browser side's own tokenizer (`rythme-site-client`):
//! the pages and the playground color code the same way.

use paquets::prelude::*;

pub use rythme_site_client::highlight::{BUILTINS, KEYWORDS, tokenize};
pub use rythme_site_client::messages::{ERROR_WORDS, HELP_WORDS, WARNING_WORDS};

fn pieces(parts: Vec<(Option<&'static str>, String)>) -> Vec<Node> {
    parts
        .into_iter()
        .map(|(class, text)| match class {
            Some(class) => span().class(class).text(text).into(),
            None => Node::from(text),
        })
        .collect()
}

/// The lines of `parts`, each a `<span class="line">` that wraps under its own
/// indentation (`--indent`, its leading spaces, read by `site.css`): code never scrolls
/// sideways, and a wrapped line still shows where it belongs. Each line but the last
/// ends with its `\n`, so that copying the code gives it back as written.
fn lines(source: &str, parts: Vec<(Option<&'static str>, String)>) -> Vec<Node> {
    let mut lines: Vec<Vec<Node>> = vec![Vec::new()];
    for (class, text) in parts {
        for (k, piece) in text.split('\n').enumerate() {
            if k > 0 {
                lines.push(Vec::new());
            }
            if piece.is_empty() {
                continue;
            }
            let lines = lines.last_mut().expect("one line at least");
            lines.push(match class {
                Some(class) => span().class(class).text(piece.to_owned()).into(),
                None => Node::from(piece.to_owned()),
            });
        }
    }
    let last = lines.len() - 1;
    lines
        .into_iter()
        .zip(source.split('\n'))
        .enumerate()
        .map(|(n, (nodes, text))| {
            let indent = text.chars().take_while(|c| *c == ' ').count();
            let line = span().class("line").children(nodes);
            let line = if indent > 0 { line.attr("style", format!("--indent: {indent}")) } else { line };
            if n < last { line.text("\n").into() } else { line.into() }
        })
        .collect()
}

/// `<pre class="code-block"><code>…</code></pre>` with colored Rythme code.
pub fn code_block(source: &str) -> Element {
    let source = source.trim_end_matches('\n');
    pre().class("code-block").child(code().attr("translate", "no").children(lines(source, tokenize(source))))
}

/// The same block for code of another language (a shell…), without colors.
pub fn plain_block(source: &str) -> Element {
    let source = source.trim_end_matches('\n');
    pre().class("code-block").child(code().attr("translate", "no").children(lines(source, vec![(None, source.to_owned())])))
}

/// `<code>` with colored Rythme code, inside a sentence or a table.
pub fn code_inline(source: &str) -> Element {
    code().class("code-inline").attr("translate", "no").children(pieces(tokenize(source)))
}

/// The class of a line of an error message, as the playground colors it.
fn line_class(line: &str) -> Option<&'static str> {
    let trimmed = line.trim_start();
    let help = trimmed.strip_prefix('=').map(str::trim_start);
    if ERROR_WORDS.iter().any(|w| line.starts_with(w)) {
        Some("err")
    } else if WARNING_WORDS.iter().any(|w| line.starts_with(w)) {
        Some("warn")
    } else if help.is_some_and(|h| HELP_WORDS.iter().any(|w| h.starts_with(w))) {
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
    fn blocks_wrap_line_by_line_under_their_indentation() {
        let html = Node::from(code_block("fn f(x) =\n  \"a\nb\"\n\n    x # done\n")).render();
        assert_eq!(html.matches(r#"<span class="line""#).count(), 5, "{html}");
        assert!(html.contains(r#"<span class="line" style="--indent: 2">  <span class="t-str">&quot;a"#), "{html}");
        assert!(html.contains(r#"<span class="line" style="--indent: 4">    x"#), "{html}");
        assert!(html.contains("<span class=\"line\">\n</span>"), "an empty line: {html}");
        assert!(html.ends_with("done</span></span></code></pre>"), "no new line after the last line: {html}");
        let plain = Node::from(plain_block("cargo install\n  --path .")).render();
        assert!(plain.contains(r#"<span class="line" style="--indent: 2">  --path .</span>"#), "{plain}");
    }
}
