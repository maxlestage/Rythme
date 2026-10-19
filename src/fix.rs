//! Fixes: the changes of the code that correct a diagnostic.
//!
//! The checker and the parser attach a [`Fix`] where they know the exact
//! change (a misspelt name, a missing `use`, `a ? b : c`); [`complete`] adds
//! those that the text around a mistake gives (`&&` → `and`, `def` → `fn`,
//! `f"..."` → `"..."`). `rythme check --format json`, the language server and
//! the tools of AI assistants show them; `rythme fix` applies the safe ones.

use crate::diagnostic::{Diagnostic, Fix};
use crate::source::{self, Span};
use crate::suggest;

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The last place inside `span` where the name `word` is written.
pub fn word_in(span: Span, word: &str) -> Option<Span> {
    let f = source::get(span.file)?;
    let text = f.text.get(span.start as usize..span.end as usize)?;
    let mut found = None;
    for (i, _) in text.match_indices(word) {
        let before = text[..i].chars().next_back();
        let after = text[i + word.len()..].chars().next();
        if !before.is_some_and(is_name_char) && !after.is_some_and(is_name_char) {
            found = Some(Span::new(span.file, span.start as usize + i, span.start as usize + i + word.len()));
        }
    }
    found
}

/// Adds to a "did you mean `new`?" diagnostic the fix that writes `new`
/// instead of `old` (written inside `span`). It is a guess: not safe.
pub fn guess(d: Diagnostic, span: Span, old: &str, new: &str) -> Diagnostic {
    match word_in(span, old) {
        Some(at) => d.fix(Fix::replace(at, new, false)),
        None => d,
    }
}

/// Adds to an "add `use stem`" diagnostic the fix: the line `use stem` at
/// the top of the file (after its other `use` lines), and `stem.name`
/// instead of `name` at `span`. Safe: only that file defines the name.
pub fn add_use(d: Diagnostic, span: Span, stem: &str, name: &str) -> Diagnostic {
    let Some(file) = source::get(span.file) else { return d };
    let text: &str = &file.text;
    let mut at = 0;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let t = line.trim();
        if t.starts_with("use ") {
            at = offset + line.len();
        } else if !(t.is_empty() || t.starts_with('#')) {
            break;
        }
        offset += line.len();
    }
    let nl = if at > 0 && !text[..at].ends_with('\n') { "\n" } else { "" };
    let Some(name_at) = word_in(span, name) else { return d };
    let (file, written) = (stem, name);
    let fix = Fix {
        title: t!("Add `use {file}` and write `{stem}.{written}`", "Ajouter `use {file}` et écrire `{stem}.{written}`"),
        edits: vec![(Span::new(span.file, at, at), format!("{nl}use {stem}\n")), (name_at, format!("{stem}.{name}"))],
        safe: true,
    };
    d.fix(fix)
}

/// The code of a span, if it is a simple expression that can go inside the
/// braces of a text: `n`, `p.name`, `len(xs)`.
fn simple_code(span: Span) -> Option<String> {
    let f = source::get(span.file)?;
    let code = f.text.get(span.start as usize..span.end as usize)?;
    (!code.is_empty() && !code.contains(['"', '{', '}', '\n', '#'])).then(|| code.to_string())
}

/// The code of a span if it is a text written between `"` on one line.
fn text_code(span: Span) -> Option<String> {
    let f = source::get(span.file)?;
    let code = f.text.get(span.start as usize..span.end as usize)?;
    let ok = code.len() >= 2 && code.starts_with('"') && code.ends_with('"') && !code.starts_with("\"\"\"") && !code.contains('\n');
    ok.then(|| code.to_string())
}

/// `"n = " + n` and `n + " items"`: the value inside the text, `"n = {n}"`.
pub fn concat(d: Diagnostic, left: Span, left_text: bool, right: Span) -> Diagnostic {
    let new = if left_text {
        text_code(left).zip(simple_code(right)).map(|(t, v)| format!("{}{{{v}}}\"", &t[..t.len() - 1]))
    } else {
        simple_code(left).zip(text_code(right)).map(|(v, t)| format!("\"{{{v}}}{}", &t[1..]))
    };
    match new {
        Some(new) if left.file == right.file && left.end <= right.start => {
            d.fix(Fix::replace(Span::new(left.file, left.start as usize, right.end as usize), new, true))
        }
        _ => d,
    }
}

/// `"hi %s" % name`: the value inside the text, `"hi {name}"`.
pub fn percent(d: Diagnostic, left: Span, right: Span) -> Diagnostic {
    let (Some(t), Some(v)) = (text_code(left), simple_code(right)) else { return d };
    let marks: Vec<usize> = ["%s", "%d", "%i", "%f", "%r"].iter().flat_map(|m| t.match_indices(m).map(|(i, _)| i)).collect();
    if marks.len() != 1 || v.starts_with('(') || v.starts_with('[') || left.file != right.file || left.end > right.start {
        return d;
    }
    let new = format!("{}{{{v}}}{}", &t[..marks[0]], &t[marks[0] + 2..]);
    d.fix(Fix::replace(Span::new(left.file, left.start as usize, right.end as usize), new, true))
}

/// The text of the line around the byte `at`: its start, and its end
/// (without the line break).
fn line_around(text: &str, at: usize) -> (usize, usize) {
    let start = text[..at].rfind('\n').map_or(0, |i| i + 1);
    let end = text[at..].find('\n').map_or(text.len(), |i| at + i);
    (start, end.max(start))
}

/// Adds the help and the fix that the text around a diagnostic gives, when
/// it has none: called on every error of the parser and of the checker.
pub fn complete(d: &mut Diagnostic) {
    let Some(span) = d.span else { return };
    let Some(file) = source::get(span.file) else { return };
    let text: &str = &file.text;
    let (s, e) = (span.start as usize, span.end as usize);
    let Some(w) = text.get(s..e) else { return };
    if !text.is_char_boundary(s) || !text.is_char_boundary(e) {
        return;
    }
    let sp = |a: usize, b: usize| Span::new(span.file, a, b);
    let (before, after) = (&text[..s], &text[e..]);
    match d.code {
        "E001" => {
            if w == ">" && before.ends_with('<') {
                d.helps.push(t!("\"not equal\" is written `!=`: `a != b`", "« différent de » s'écrit `!=` : `a != b`"));
            } else if w == "=>" || (w == "," && arrow_after(text, s)) {
                d.helps.push(t!("an anonymous function is written `fn(x) = x * 2`", "une fonction anonyme s'écrit `fn(x) = x * 2`"));
            } else if f_prefix(w, before) {
                d.helps.push(f_string_help());
            } else if missing_then(text, s, w) {
                d.helps.push(t!(
                    "on several lines, `if` is written:\n  if x > 0 then\n    \"positive\"\n  else if x == 0 then\n    \"zero\"\n  else\n    \"negative\"\n(no `end`; for several statements in a branch, use `do ... end`)",
                    "sur plusieurs lignes, `if` s'écrit :\n  if x > 0 then\n    \"positif\"\n  else if x == 0 then\n    \"zéro\"\n  else\n    \"négatif\"\n(sans `end` ; pour plusieurs instructions dans une branche, utilisez `do ... end`)"
                ));
            }
        }
        "E003" if w == "f" && after.starts_with('"') => d.helps.push(f_string_help()),
        _ => {}
    }
    if !d.fixes.is_empty() {
        return;
    }
    let fix = match d.code {
        "E001" => syntax_fix(text, s, e, w, sp),
        "E003" | "W005" => {
            let renamed = suggest::alias_hint(w).is_some_and(|h| d.helps.contains(&h));
            name_fix(w, before, after, span, renamed)
        }
        // `if a then b elif c then d else e`: the `elif` is shown as a second place.
        "E019" => d.secondary.iter().find_map(|(at, _)| {
            let word = text.get(at.start as usize..at.end as usize)?;
            matches!(word, "elif" | "elsif").then(|| Fix::replace(*at, "else if", true))
        }),
        _ => None,
    };
    d.fixes.extend(fix);
}

fn f_string_help() -> String {
    t!(
        "Rythme texts put values in by themselves: write `\"total: {{x}}\"`, without `f` in front",
        "les textes de Rythme insèrent les valeurs d'eux-mêmes : écrivez `\"total : {{x}}\"`, sans `f` devant"
    )
}

/// `f"..."`: a Python format text, its `f` right before the text `w`.
fn f_prefix(w: &str, before: &str) -> bool {
    w.starts_with('"') && before.ends_with('f') && !before[..before.len() - 1].ends_with(is_name_char)
}

/// The line of `at` starts an `if` (or an `else if`, an `unless`) and has no
/// `then`: the error is at its end (or at a `:` there).
fn missing_then(text: &str, at: usize, w: &str) -> bool {
    let (start, end) = line_around(text, at);
    let line = text[start..end].trim();
    let starts = ["if ", "unless ", "else if ", "elif ", "elsif "].iter().any(|k| line.starts_with(k));
    let at_end = text[at..end].trim_start_matches(':').trim().is_empty();
    starts && at_end && (w.is_empty() || w == ":" || w.trim().is_empty()) && !line.split_whitespace().any(|t| t == "then")
}

fn syntax_fix(text: &str, s: usize, e: usize, w: &str, sp: impl Fn(usize, usize) -> Span) -> Option<Fix> {
    let (before, after) = (&text[..s], &text[e..]);
    let spaces_after = after.len() - after.trim_start_matches([' ', '\t']).len();
    Some(match w {
        "&&" => Fix::replace(sp(s, e), "and", true),
        "||" => Fix::replace(sp(s, e), "or", true),
        "!" if !after.starts_with('=') => {
            let new = if after.starts_with(char::is_whitespace) { "not" } else { "not " };
            Fix::replace(sp(s, e), new, true)
        }
        // `===` and `!==`: one `=` too many.
        "=" if before.ends_with("==") || before.ends_with("!=") => Fix::replace(sp(s, e), "", true),
        ">" if before.ends_with('<') => Fix::replace(sp(s - 1, e), "!=", true),
        "=>" => return arrow_fix(text, s, e, sp),
        "let" | "var" | "const" => Fix::replace(sp(s, e + spaces_after), "", true),
        "def" | "function" => Fix::replace(sp(s, e), "fn", true),
        "elif" | "elsif" => Fix::replace(sp(s, e), "else if", true),
        "null" | "nil" => Fix::replace(sp(s, e), "None", false),
        "return" => {
            // Safe when it is the last line of a function or of a branch.
            let next = text[e..].find('\n').map(|i| &text[e + i + 1..]);
            let rest = next.and_then(|n| n.lines().map(str::trim).find(|l| !l.is_empty() && !l.starts_with('#')));
            let last = rest.is_none_or(|l| l == "end" || l.starts_with("end ") || l.starts_with("else"));
            let has_value = !text[e..].lines().next().unwrap_or("").trim().is_empty();
            Fix::replace(sp(s, e + spaces_after), "", last && has_value)
        }
        "lambda" => {
            let (_, end) = line_around(text, s);
            let colon = text[e..end].find(':')?;
            let params = text[e..e + colon].trim();
            if !params.chars().all(|c| is_name_char(c) || c == ',' || c == ' ') {
                return None;
            }
            Fix::replace(sp(s, e + colon + 1), format!("fn({params}) ="), true)
        }
        _ if f_prefix(w, before) => Fix::replace(sp(s - 1, s), "", true),
        _ if missing_then(text, s, w) => {
            let (_, end) = line_around(text, s);
            let line_end = text[..end].trim_end().len();
            match text[..line_end].strip_suffix(':') {
                Some(head) => Fix::replace(sp(head.trim_end().len(), line_end), " then", true),
                None => Fix::replace(sp(line_end, line_end), " then", true),
            }
        }
        // `(a, b) => a + b`: the error is inside the parameters, before the `=>`.
        _ => {
            let (_, end) = line_around(text, s);
            let arrow = s + text[s..end].find("=>")?;
            let fix = arrow_fix(text, arrow, arrow + 2, sp)?;
            return fix.edits.iter().all(|(span, _)| span.start as usize <= s).then_some(fix);
        }
    })
}

/// The `=>` of `(a, b) => ...` after the byte `at`, on its line.
fn arrow_after(text: &str, at: usize) -> bool {
    let (_, end) = line_around(text, at);
    text[at..end].contains("=>")
}

/// `x => x + 1` and `(a, b) => a + b`: `fn(x) = x + 1`.
fn arrow_fix(text: &str, s: usize, e: usize, sp: impl Fn(usize, usize) -> Span) -> Option<Fix> {
    let head = text[..s].trim_end();
    if text[e..].trim_start().starts_with('{') {
        return None;
    }
    let (start, params) = if let Some(inner) = head.strip_suffix(')') {
        let open = inner.rfind('(')?;
        if inner[..open].trim_end().ends_with(is_name_char) {
            return None;
        }
        (open, inner[open + 1..].trim())
    } else {
        let start = head.rfind(|c: char| !is_name_char(c)).map_or(0, |i| i + 1);
        let name = &head[start..];
        if name.is_empty() || head[..start].trim_end().ends_with(|c: char| is_name_char(c) || c == '.') {
            return None;
        }
        (start, name)
    };
    if !params.chars().all(|c| is_name_char(c) || c == ',' || c == ' ') {
        return None;
    }
    Some(Fix::replace(sp(start, e), format!("fn({params}) ="), true))
}

/// An unknown name (E003) or function (W005): its Rythme spelling.
fn name_fix(w: &str, before: &str, after: &str, span: Span, renamed: bool) -> Option<Fix> {
    Some(match w {
        "True" => Fix::replace(span, "true", true),
        "False" => Fix::replace(span, "false", true),
        "Just" => Fix::replace(span, "Some", true),
        "NULL" | "Nil" | "undefined" | "nullptr" | "Nothing" => Fix::replace(span, "None", false),
        "f" if after.starts_with('"') => Fix::replace(span, "", true),
        "console" if after.starts_with(".log(") => Fix::replace(Span { end: span.end + 4, ..span }, "print", true),
        _ if renamed && !after.starts_with('.') => {
            let new = suggest::rythme_name(w).filter(|n| *n != "+")?;
            // Called (`str(3)`, `xs.append(4)`): the function of Rythme is meant.
            let called = after.trim_start().starts_with('(') || before.ends_with('.');
            Fix::replace(span, new, called)
        }
        _ => return None,
    })
}

// ----- applying fixes --------------------------------------------------------

/// An edit of a text: the bytes from the start to the end are replaced by the new text.
pub type Edit = (usize, usize, String);

/// Applies edits (byte ranges and new texts) to a text, skipping those that
/// overlap an edit already taken.
pub fn apply(text: &str, edits: &[Edit]) -> String {
    let mut sorted: Vec<&Edit> = edits.iter().collect();
    sorted.sort_by_key(|(s, e, _)| (*s, *e));
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    for (s, e, new) in sorted {
        if *s < at || *e > text.len() || !text.is_char_boundary(*s) || !text.is_char_boundary(*e) {
            continue;
        }
        out.push_str(&text[at..*s]);
        out.push_str(new);
        at = *e;
    }
    out.push_str(&text[at..]);
    out
}

/// The fixes of `diags` that apply to the file `file` (all their edits are
/// in it), safe ones only unless `all`, without two that overlap: the
/// title of each one and its edits.
pub fn chosen(diags: &[Diagnostic], file: u32, all: bool) -> Vec<(String, Vec<Edit>)> {
    let mut taken: Vec<(usize, usize)> = Vec::new();
    let mut out = Vec::new();
    for d in diags {
        // One fix per diagnostic: the first that can be applied.
        for fix in d.fixes.iter().filter(|f| f.safe || all) {
            if fix.edits.is_empty() || fix.edits.iter().any(|(sp, _)| sp.file != file) {
                continue;
            }
            let edits: Vec<Edit> = fix.edits.iter().map(|(sp, new)| (sp.start as usize, sp.end as usize, new.clone())).collect();
            // Two insertions at the same place, or an edit inside another, do not mix.
            let clash = edits.iter().any(|&(s, e, _)| taken.iter().any(|&(ts, te)| s < te && ts < e || s == ts));
            if clash {
                continue;
            }
            taken.extend(edits.iter().map(|&(s, e, _)| (s, e)));
            out.push((fix.title.clone(), edits));
            break;
        }
    }
    out
}

/// A unified diff of two texts, line by line, for `rythme fix --dry-run`.
pub fn diff(name: &str, old: &str, new: &str) -> String {
    let a: Vec<&str> = old.lines().collect();
    let b: Vec<&str> = new.lines().collect();
    // The longest common subsequence of lines (files are small).
    let (n, m) = (a.len(), b.len());
    let mut lcs = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if a[i] == b[j] { lcs[i + 1][j + 1] + 1 } else { lcs[i + 1][j].max(lcs[i][j + 1]) };
        }
    }
    // Each line: ' ', '-' or '+', with its line numbers in the old and new texts.
    let mut ops: Vec<(char, usize, usize)> = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < n || j < m {
        if i < n && j < m && a[i] == b[j] {
            ops.push((' ', i, j));
            i += 1;
            j += 1;
        } else if i < n && (j == m || lcs[i + 1][j] >= lcs[i][j + 1]) {
            ops.push(('-', i, j));
            i += 1;
        } else {
            ops.push(('+', i, j));
            j += 1;
        }
    }
    let mut out = format!("--- {name}\n+++ {name}\n");
    let context = 3;
    let mut k = 0;
    while k < ops.len() {
        if ops[k].0 == ' ' {
            k += 1;
            continue;
        }
        // A hunk: the changes from here, with what is between them when it is short.
        let start = k.saturating_sub(context);
        let mut end = k;
        while end < ops.len() {
            if ops[end].0 != ' ' {
                end += 1;
                continue;
            }
            let run = ops[end..].iter().take_while(|o| o.0 == ' ').count();
            if end + run < ops.len() && run <= 2 * context {
                end += run;
            } else {
                end = (end + context).min(ops.len());
                break;
            }
        }
        let hunk = &ops[start..end];
        let old_len = hunk.iter().filter(|o| o.0 != '+').count();
        let new_len = hunk.iter().filter(|o| o.0 != '-').count();
        let old_start = hunk.first().map_or(0, |o| o.1) + usize::from(old_len > 0);
        let new_start = hunk.first().map_or(0, |o| o.2) + usize::from(new_len > 0);
        out.push_str(&format!("@@ -{old_start},{old_len} +{new_start},{new_len} @@\n"));
        for &(op, i, j) in hunk {
            let line = if op == '+' { b[j] } else { a[i] };
            out.push_str(&format!("{op}{line}\n"));
        }
        k = end;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edits_apply_from_the_start_and_skip_overlaps() {
        let edits = vec![(0, 2, "and".to_string()), (5, 6, "X".to_string()), (1, 3, "no".to_string())];
        assert_eq!(apply("&& abc", &edits), "and abX");
    }

    #[test]
    fn a_diff_shows_the_changed_lines() {
        let d = diff("a.ry", "x = 1\nprint(x && y)\nend\n", "x = 1\nprint(x and y)\nend\n");
        assert_eq!(d, "--- a.ry\n+++ a.ry\n@@ -1,3 +1,3 @@\n x = 1\n-print(x && y)\n+print(x and y)\n end\n");
        assert_eq!(diff("a.ry", "a\n", "a\n"), "--- a.ry\n+++ a.ry\n");
    }
}
