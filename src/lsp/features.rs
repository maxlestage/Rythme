//! What the editor asks for: problems, hover, completion, signature help,
//! go to definition, outline, highlights and quick fixes.

use std::collections::HashSet;
use std::path::Path;

use super::analysis::Analysis;
use super::index::{Def, DefId, DefKind, Occ, Target};
use super::json::{obj, Json};
use super::scan::{self, Place};
use super::text::{Encoding, LineIndex, Pos};
use crate::builtins::{self, Builtin, BUILTINS};
use crate::diagnostic::{Diagnostic, Severity};
use crate::lexer::{KEYWORDS, RESERVED};
use crate::source::{self, Span};
use crate::types::Ty;
use crate::{help, suggest};

/// What the editor can show, learnt at start-up.
#[derive(Clone, Copy)]
pub struct Ctx {
    pub enc: Encoding,
    pub markdown: bool,
    pub snippets: bool,
    pub hierarchical_symbols: bool,
}

// ----- positions ----------------------------------------------------------

pub fn pos_json(p: Pos) -> Json {
    obj([("line", p.line.into()), ("character", p.character.into())])
}

/// A range of bytes of a text, for the editor.
pub fn range_in(text: &str, lines: &LineIndex, start: usize, end: usize, enc: Encoding) -> Json {
    let a = lines.pos(text, start, enc);
    let b = lines.pos(text, end.max(start), enc);
    obj([("start", pos_json(a)), ("end", pos_json(b))])
}

/// The address and range of a span, in this document or in a file it uses.
pub fn location(cx: &Ctx, a: &Analysis, span: Span) -> Option<(String, Json)> {
    if span.file == a.module.file {
        return Some((a.uri.clone(), range_in(&a.text, &a.lines, span.start as usize, span.end as usize, cx.enc)));
    }
    let uri = a.uri_of(span.file)?.to_string();
    let file = source::get(span.file)?;
    let lines = LineIndex::new(&file.text);
    Some((uri, range_in(&file.text, &lines, span.start as usize, span.end as usize, cx.enc)))
}

fn location_json(cx: &Ctx, a: &Analysis, span: Span) -> Option<Json> {
    let (uri, range) = location(cx, a, span)?;
    Some(obj([("uri", uri.into()), ("range", range)]))
}

/// The text of a span, on one line, shortened if long.
fn snippet(span: Span, max: usize) -> String {
    let Some(file) = source::get(span.file) else { return String::new() };
    let text = file.text.get(span.start as usize..span.end as usize).unwrap_or("");
    let one_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.chars().count() > max {
        let cut: String = one_line.chars().take(max).collect();
        format!("{cut}…")
    } else {
        one_line
    }
}

// ----- problems -----------------------------------------------------------

/// A problem, as the editor shows it.
pub fn diagnostic_json(cx: &Ctx, a: &Analysis, d: &Diagnostic) -> Json {
    let mut related = Vec::new();
    let range = match d.span {
        Some(s) if s.file == a.module.file => range_in(&a.text, &a.lines, s.start as usize, s.end as usize, cx.enc),
        other => {
            // A problem placed in another file: shown at the top, with a link.
            if let Some(loc) = other.and_then(|s| location_json(cx, a, s)) {
                related.push(obj([("location", loc), ("message", d.message.clone().into())]));
            }
            range_in(&a.text, &a.lines, 0, 0, cx.enc)
        }
    };
    let mut message = d.message.clone();
    // The label, unless it repeats the message ("expected `)` here").
    let label = d.label.trim_end_matches(" here").trim_end_matches(" ici");
    if !label.is_empty() && !d.message.contains(label) {
        message.push_str(&format!(" ({})", d.label));
    }
    for note in &d.notes {
        message.push_str(&format!("\nnote: {note}"));
    }
    for h in &d.helps {
        message.push_str(&format!("\n{}: {h}", t!("help", "aide")));
    }
    if !d.code.is_empty() {
        message.push_str(&t!("\nmore: `rythme explain {}`", "\nen savoir plus : `rythme explain {}`", d.code));
    }
    for (span, label) in &d.secondary {
        if let Some(loc) = location_json(cx, a, *span) {
            related.push(obj([("location", loc), ("message", label.clone().into())]));
        }
    }
    let severity: u32 = if d.severity == Severity::Error { 1 } else { 2 };
    let mut out = obj([("range", range), ("severity", severity.into()), ("source", "rythme".into()), ("message", message.into())]);
    if !d.code.is_empty() {
        out.set("code", d.code.into());
    }
    if !related.is_empty() {
        out.set("relatedInformation", Json::Arr(related));
    }
    // Unused names and unreachable cases are shown faded.
    if matches!(d.code, "W001" | "W004") {
        out.set("tags", Json::Arr(vec![1u32.into()]));
    }
    out
}

pub fn diagnostics_json(cx: &Ctx, a: &Analysis) -> Json {
    Json::Arr(a.diagnostics().map(|d| diagnostic_json(cx, a, d)).collect())
}

// ----- hover --------------------------------------------------------------

fn code_block(code: &str) -> String {
    format!("```rythme\n{code}\n```")
}

/// Markdown for editors that only show plain text.
fn plain(markdown: &str) -> String {
    markdown.lines().filter(|l| !l.starts_with("```") && *l != "---").map(|l| l.replace("**", "")).collect::<Vec<_>>().join("\n")
}

fn markup(cx: &Ctx, value: String) -> Json {
    if cx.markdown {
        obj([("kind", "markdown".into()), ("value", value.into())])
    } else {
        obj([("kind", "plaintext".into()), ("value", plain(&value).into())])
    }
}

pub fn builtin_markdown(b: &Builtin) -> String {
    let ruby: Vec<String> = builtins::aliases(b.name).map(|a| format!("`{a}`")).collect();
    let same = if ruby.is_empty() {
        String::new()
    } else {
        t!("\n\nSame as: {} (its Ruby names)", "\n\nPareil que : {} (ses noms Ruby)", ruby.join(", "))
    };
    format!(
        "{}\n\n{}{same}\n\n**{}**\n{}\n\n`rythme help {}`",
        code_block(b.sig),
        b.doc(),
        t!("Example", "Exemple"),
        code_block(b.example),
        b.name
    )
}

/// `upcase` is the Ruby name of `upper`.
fn alias_line(alias: &str, of: &str) -> String {
    t!("**{alias}** — the Ruby name of `{of}`: the same function", "**{alias}** — le nom Ruby de `{of}` : la même fonction")
}

/// The kind of the value written just before `.`, when it is a literal:
/// `"abc".`, `[1, 2].`, `42.`, `:ok.`, `{a: 1}.`.
fn receiver_type(before_dot: &str) -> Option<Ty> {
    let s = before_dot.trim_end();
    let last = s.chars().last()?;
    let opener = |open: char, close: char| {
        let mut depth = 0usize;
        for (i, c) in s.char_indices().rev() {
            if c == close {
                depth += 1;
            } else if c == open {
                depth -= 1;
                if depth == 0 {
                    // A literal, not an index (`xs[0]`) or a call.
                    return !s[..i].trim_end().ends_with(|c: char| scan::is_name_char(c) || matches!(c, ')' | ']' | '}' | '?'));
                }
            }
        }
        false
    };
    match last {
        '"' => Some(Ty::Text),
        ']' if opener('[', ']') => Some(Ty::list(Ty::Any)),
        '}' if opener('{', '}') => Some(Ty::record(Vec::new(), false)),
        c if c.is_ascii_digit() => {
            let word = &s[s.rfind(|c: char| !scan::is_name_char(c) && c != '.').map_or(0, |i| i + 1)..];
            if word.starts_with(|c: char| c.is_ascii_digit()) {
                Some(if word.contains('.') { Ty::Float } else { Ty::Int })
            } else {
                None
            }
        }
        _ => {
            let start = s.rfind(|c: char| !scan::is_name_char(c)).map_or(0, |i| i + 1);
            (start > 0 && s[..start].ends_with(':') && !s[..start - 1].ends_with(':')).then_some(Ty::Symbol)
        }
    }
}

/// Can the built-in function `name` take a value of type `t` first?
fn takes_first(name: &str, min: usize, t: &Ty) -> bool {
    let mut args = vec![Ty::Any; min.max(1)];
    args[0] = t.clone();
    !matches!(crate::types::builtin_call(name, &args), Some(crate::types::Outcome::Wrong { index: 0, .. }))
}

fn prelude_markdown(name: &str) -> String {
    let (sig, doc) = match name {
        "Some" => (
            "Some(value)",
            t!(
                "A value that is there, in an `Option`. The other case is `None`.",
                "Une valeur présente, dans une `Option`. L'autre cas est `None`."
            ),
        ),
        "None" => (
            "None",
            t!(
                "No value, in an `Option`: what `find`, `first` or `get` give when there is nothing. Handle it with `match` or `unwrap_or`.",
                "Pas de valeur, dans une `Option` : ce que donnent `find`, `first` ou `get` quand il n'y a rien. Traitez-la avec `match` ou `unwrap_or`."
            ),
        ),
        "Ok" => (
            "Ok(value)",
            t!(
                "The result of an operation that succeeded, in a `Result`. The other case is `Err(error)`.",
                "Le résultat d'une opération réussie, dans un `Result`. L'autre cas est `Err(erreur)`."
            ),
        ),
        _ => (
            "Err(error)",
            t!(
                "The result of an operation that failed, in a `Result`. Inside a function, `?` stops and passes it on.",
                "Le résultat d'une opération qui a échoué, dans un `Result`. Dans une fonction, `?` s'arrête et le transmet."
            ),
        ),
    };
    format!("{}\n\n{doc}\n\n`rythme help errors`", code_block(sig))
}

fn keyword_markdown(word: &str) -> Option<String> {
    if let Some((doc, example, topic)) = help::keyword_doc(word) {
        return Some(format!(
            "**{word}** — {}\n\n{doc}\n\n**{}**\n{}\n\n`rythme help {topic}`",
            t!("keyword", "mot-clé"),
            t!("Example", "Exemple"),
            code_block(example)
        ));
    }
    if RESERVED.contains(&word) {
        let hint = suggest::foreign_keyword_hint(word).unwrap_or_default();
        return Some(format!("{}\n\n{hint}", t!("**{word}** is not used in Rythme.", "**{word}** n'est pas utilisé en Rythme.")));
    }
    None
}

/// The heads of the definitions of a function: `fn fib(0)`, `fn fib(n)`.
fn heads(def: &Def) -> String {
    let shown: Vec<String> = def.heads.iter().take(8).map(|h| snippet(*h, 120)).collect();
    let mut out = shown.join("\n");
    if def.heads.len() > 8 {
        out.push_str(&format!("\n# … {}", t!("{} more definitions", "{} autres définitions", def.heads.len() - 8)));
    }
    out
}

fn def_markdown(a: &Analysis, id: DefId, def: &Def) -> String {
    let mut parts = Vec::new();
    match def.kind {
        DefKind::Function => {
            parts.push(code_block(&heads(def)));
            if !def.top {
                parts.push(t!("local function", "fonction locale"));
            }
        }
        DefKind::Value => parts.push(code_block(&snippet(def.full, 100))),
        DefKind::Param => {
            parts.push(code_block(&def.name));
            parts.push(match &def.owner {
                Some(f) => t!("parameter of `{f}`", "paramètre de `{f}`"),
                None => t!("parameter of an anonymous function", "paramètre d'une fonction anonyme"),
            });
        }
        DefKind::Local => match &def.sig {
            Some(sig) => {
                parts.push(code_block(sig));
                parts.push(t!("local function", "fonction locale"));
            }
            None => {
                parts.push(code_block(&snippet(def.full, 100)));
                parts.push(t!("local value", "valeur locale"));
            }
        },
        DefKind::Case => {
            parts.push(code_block(&def.name));
            let pattern = snippet(def.full, 60);
            parts.push(t!("given by the case `{pattern}` of a `match`", "donné par le cas `{pattern}` d'un `match`"));
        }
        DefKind::Type => parts.push(code_block(&snippet(def.full, 200))),
        DefKind::Ctor => {
            parts.push(code_block(def.sig.as_deref().unwrap_or(&def.name)));
            if let Some(t) = &def.owner {
                parts.push(t!("constructor of the type `{t}`", "constructeur du type `{t}`"));
            }
        }
        DefKind::Test => parts.push(code_block(&format!("test {:?}", def.name))),
    }
    if let Some(doc) = &def.doc {
        parts.push(doc.to_string());
    }
    if id.file != a.module.file {
        if let Some(uri) = a.uri_of(id.file) {
            let name = super::text::short_name(uri);
            parts.push(t!("*defined in `{name}`*", "*défini dans `{name}`*"));
        }
    }
    parts.join("\n\n")
}

fn target_markdown(a: &Analysis, occ: &Occ) -> Option<String> {
    let mut text = match &occ.target {
        Target::Def(id) => def_markdown(a, *id, a.def(*id)?),
        Target::Builtin(name) => {
            let b = builtins::lookup(name)?;
            match builtins::alias_of(&occ.name) {
                Some(of) => format!("{}\n\n{}", alias_line(&occ.name, of), builtin_markdown(b)),
                None => builtin_markdown(b),
            }
        }
        Target::Pi => format!("{}\n\n{}", code_block("pi = 3.141592653589793"), t!("The number π.", "Le nombre π.")),
        Target::Prelude(name) => prelude_markdown(name),
        Target::Unknown => return None,
    };
    if occ.method {
        let n = &occ.name;
        text.push_str(&t!(
            "\n\n`value.{n}(...)` calls `{n}(value, ...)`, or the field `{n}` of a record; `value.{n}` reads the field `{n}`, or calls `{n}(value)`",
            "\n\n`valeur.{n}(...)` appelle `{n}(valeur, ...)`, ou le champ `{n}` d'un enregistrement ; `valeur.{n}` lit le champ `{n}`, ou appelle `{n}(valeur)`"
        ));
    }
    Some(text)
}

/// The explanation of an error code, for a hover.
fn explain_markdown(code: &str) -> Option<String> {
    let text = help::explain(code)?;
    let (title, rest) = text.split_once('\n').unwrap_or((&text, ""));
    Some(format!("**{title}**\n```text\n{}\n```", rest.trim()))
}

/// The file named by a `use` line.
fn used_file(a: &Analysis, span: Span) -> Option<&std::path::PathBuf> {
    a.module.used.iter().find(|(s, _)| *s == span).map(|(_, p)| p)
}

/// What a `use` line brings: the file and its definitions.
fn use_markdown(a: &Analysis, p: &str, span: Span) -> String {
    let Some(path) = used_file(a, span) else {
        return t!("the file `{p}` is not found", "le fichier `{p}` est introuvable");
    };
    let mut out = format!("`{}`", path.display());
    let name = a.module.items.iter().find_map(|it| match it {
        crate::ast::Item::Use(u) if u.span == span => u.name.map(|(n, _)| n),
        _ => None,
    });
    let prefix = name.map(|n| format!("{n}.")).unwrap_or_default();
    if let Some(n) = name {
        out.push_str(&t!(
            "\n\nIts definitions are used through the name `{n}`: `{n}.name`",
            "\n\nSes définitions s'utilisent par le nom `{n}` : `{n}.nom`"
        ));
    }
    if let Some(m) = a.modules.get(path) {
        let defs: Vec<String> = m
            .index
            .tops
            .iter()
            .filter_map(|(_, idx)| m.index.def(*idx))
            .filter(|d| matches!(d.kind, DefKind::Function | DefKind::Value | DefKind::Ctor))
            .filter(|d| !d.name.starts_with('_') || d.name.len() == 1)
            .map(|d| format!("{prefix}{}", d.sig.clone().unwrap_or_else(|| d.name.to_string())))
            .take(40)
            .collect();
        if !defs.is_empty() {
            out.push_str(&format!("\n\n{}\n{}", t!("Defines:", "Définit :"), code_block(&defs.join("\n"))));
        }
    }
    out
}

pub fn hover(cx: &Ctx, a: &Analysis, offset: usize) -> Json {
    let mut parts: Vec<String> = Vec::new();
    let mut range: Option<(usize, usize)> = None;
    if let Some(occ) = a.index().occ_at(offset) {
        range = Some((occ.span.start as usize, occ.span.end as usize));
        parts.extend(target_markdown(a, occ));
    } else if scan::scan(&a.text, offset).place == Place::Code {
        if let Some((s, e)) = scan::word_at(&a.text, offset) {
            let word = &a.text[s..e];
            // Without a syntax tree for this line (it is being typed), a name means what it means at the top.
            let guess = || by_name(a, word).filter(|_| !a.module.parsed).and_then(|t| target_markdown(a, &fake_occ(word, t)));
            let found = keyword_markdown(word).or_else(guess);
            if let Some(text) = found {
                range = Some((s, e));
                parts.push(text);
            }
        }
    }
    let o = offset as u32;
    for (span, p) in &a.index().uses {
        if span.start <= o && o <= span.end {
            parts.push(use_markdown(a, p, *span));
        }
    }
    let mut codes = HashSet::new();
    for d in a.diagnostics() {
        if let Some(s) = d.span.filter(|s| s.file == a.module.file && s.start <= o && o <= s.end.max(s.start + 1)) {
            if !d.code.is_empty() && codes.insert(d.code) {
                parts.extend(explain_markdown(d.code));
                range.get_or_insert((s.start as usize, s.end as usize));
            }
        }
    }
    if parts.is_empty() {
        return Json::Null;
    }
    let mut out = obj([("contents", markup(cx, parts.join("\n\n---\n\n")))]);
    if let Some((s, e)) = range {
        out.set("range", range_in(&a.text, &a.lines, s, e, cx.enc));
    }
    out
}

fn fake_occ(name: &str, target: Target) -> Occ {
    Occ { span: Span::default(), name: name.into(), target, def: false, punned: false, method: false }
}

/// What a name means at the top of the document, when the code around it
/// cannot be read (a line being typed).
fn by_name(a: &Analysis, name: &str) -> Option<Target> {
    let ix = a.index();
    if let Some((_, idx)) = ix.tops.iter().find(|(s, _)| &*s.name() == name) {
        return Some(Target::Def(DefId { file: ix.file, idx: *idx }));
    }
    if let Some((s, id)) = a.module.dep_defs.iter().find(|(s, _)| &*s.name() == name) {
        let _ = s;
        return Some(Target::Def(*id));
    }
    if let Some(b) = builtins::lookup(name) {
        return Some(Target::Builtin(b.name));
    }
    if name == "pi" {
        return Some(Target::Pi);
    }
    super::index::PRELUDE.iter().find(|p| **p == name).map(|p| Target::Prelude(p))
}

// ----- completion ---------------------------------------------------------

/// Kinds of completion items in the protocol.
mod kind {
    pub const FUNCTION: u32 = 3;
    pub const FIELD: u32 = 5;
    pub const VARIABLE: u32 = 6;
    pub const ENUM: u32 = 13;
    pub const KEYWORD: u32 = 14;
    pub const SNIPPET: u32 = 15;
    pub const FILE: u32 = 17;
    pub const FOLDER: u32 = 19;
    pub const ENUM_MEMBER: u32 = 20;
    pub const CONSTANT: u32 = 21;
}

fn item(label: &str, kind: u32, detail: Option<String>, doc: Option<String>, sort: &str, cx: &Ctx) -> Json {
    let mut it = obj([("label", label.into()), ("kind", kind.into()), ("sortText", format!("{sort}{label}").into())]);
    if let Some(d) = detail {
        it.set("detail", d.into());
    }
    if let Some(d) = doc {
        it.set("documentation", markup(cx, d));
    }
    it
}

fn def_item(cx: &Ctx, a: &Analysis, id: DefId, d: &Def, sort: &str) -> Json {
    let k = match d.kind {
        DefKind::Function => kind::FUNCTION,
        DefKind::Ctor => kind::ENUM_MEMBER,
        DefKind::Type => kind::ENUM,
        DefKind::Local if d.sig.is_some() => kind::FUNCTION,
        _ => kind::VARIABLE,
    };
    item(&d.name, k, d.sig.clone(), Some(def_markdown(a, id, d)), sort, cx)
}

fn snippets() -> Vec<(&'static str, &'static str, String, String)> {
    let fr = crate::lang::is_fr();
    let (name, value, other, pattern, result, what, actual, expected, condition, file) = if fr {
        ("nom", "valeur", "autre", "motif", "résultat", "ce qu'il vérifie", "obtenu", "attendu", "condition", "fichier")
    } else {
        ("name", "value", "other", "pattern", "result", "what it checks", "actual", "expected", "condition", "file")
    };
    vec![
        (
            "fn … end",
            "fn",
            format!("fn ${{1:{name}}}(${{2:x}})\n\t$0\nend"),
            t!("a function on several lines", "une fonction sur plusieurs lignes"),
        ),
        ("fn … =", "fn", format!("fn ${{1:{name}}}(${{2:x}}) = ${{0:x}}"), t!("a function on one line", "une fonction sur une ligne")),
        (
            "match … end",
            "match",
            format!("match ${{1:{value}}}\n\t${{2:{pattern}}} -> ${{3:{result}}}\n\t_ -> ${{0:{other}}}\nend"),
            t!("choose according to the shape of a value", "choisir selon la forme d'une valeur"),
        ),
        (
            "test \"…\" = …",
            "test",
            format!("test \"${{1:{what}}}\" = assert_eq(${{2:{actual}}}, ${{0:{expected}}})"),
            t!("a test, run by `rythme test`", "un test, lancé par `rythme test`"),
        ),
        ("test … end", "test", format!("test \"${{1:{what}}}\"\n\t$0\nend"), t!("a test on several lines", "un test sur plusieurs lignes")),
        (
            "if … then … else",
            "if",
            format!("if ${{1:{condition}}} then ${{2:{value}}} else ${{0:{other}}}"),
            t!("choose between two values", "choisir entre deux valeurs"),
        ),
        (
            "type … = …",
            "type",
            if fr {
                "type ${1:Nom} = ${2:Premier}(${3:champ}) | ${0:Second}".to_string()
            } else {
                "type ${1:Name} = ${2:First}(${3:field}) | ${0:Second}".to_string()
            },
            t!("your own type", "votre propre type"),
        ),
        ("do … end", "do", "do\n\t$0\nend".to_string(), t!("several steps inside an expression", "plusieurs étapes dans une expression")),
        (
            "do |x| … end",
            "do",
            "do |${1:x}|\n\t$0\nend".to_string(),
            t!(
                "a block: a function given as the last argument of a call",
                "un bloc : une fonction donnée comme dernier argument d'un appel"
            ),
        ),
        (
            "unless … then … else",
            "unless",
            format!("unless ${{1:{condition}}} then ${{2:{value}}} else ${{0:{other}}}"),
            t!("the opposite of `if`", "le contraire de `if`"),
        ),
        (
            "use …",
            "use",
            format!("use ${{1:{file}}}"),
            t!(
                "use another file: `use geometry`, then `geometry.area(2)`",
                "utiliser un autre fichier : `use geometrie`, puis `geometrie.aire(2)`"
            ),
        ),
    ]
}

/// `trigger` is the character that made the editor ask, if any.
pub fn completion(cx: &Ctx, a: &Analysis, offset: usize, trigger: Option<&str>) -> Json {
    let text = &a.text;
    let offset = offset.min(text.len());
    let sc = scan::scan(text, offset);
    match sc.place {
        Place::Comment => return Json::Arr(Vec::new()),
        Place::Text => return use_completion(cx, a, offset, sc.text_start),
        Place::Code if short_use_line(text, offset).is_some() => return short_use_completion(cx, a, offset),
        // `"` and `/` ask for file names in `use "..."`; elsewhere they are a text or a division.
        Place::Code if matches!(trigger, Some("\"" | "/")) => return Json::Arr(Vec::new()),
        Place::Code => {}
    }
    let start = scan::word_at(text, offset).map_or(offset, |(s, _)| s.min(offset));
    let before = text[..start].trim_end_matches([' ', '\t']);
    let after_dot = before.ends_with('.') && !before.ends_with("..") && start == before.len();
    if after_dot {
        // `geometry.`: the definitions of the file used as `geometry`.
        let head = &before[..before.len() - 1];
        let m = &head[head.rfind(|c: char| !scan::is_name_char(c)).map_or(0, |i| i + 1)..];
        if a.module.named.iter().any(|(n, _)| &*n.name() == m) {
            let mut items = Vec::new();
            let mut defs: Vec<(DefId, &Def)> = a
                .module
                .dep_defs
                .iter()
                .filter(|(s, _)| s.split_module().is_some_and(|(n, _)| &*n.name() == m))
                .filter_map(|(_, id)| Some((*id, a.def(*id)?)))
                .collect();
            defs.sort_by(|x, y| x.1.name.cmp(&y.1.name));
            for (id, d) in defs {
                items.push(def_item(cx, a, id, d, "0"));
            }
            return Json::Arr(items);
        }
    }
    let mut items = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let wanted = |d: &Def| !after_dot || d.kind == DefKind::Function || d.sig.is_some() && d.kind == DefKind::Local;
    for (idx, d) in a.index().locals_at(offset) {
        if d.kind != DefKind::Test && wanted(d) && seen.insert(d.name.to_string()) {
            items.push(def_item(cx, a, DefId { file: a.module.file, idx }, d, "0"));
        }
    }
    let ix = a.index();
    for (_, idx) in &ix.tops {
        if let Some(d) = ix.def(*idx) {
            if wanted(d) && seen.insert(d.name.to_string()) {
                items.push(def_item(cx, a, DefId { file: ix.file, idx: *idx }, d, "1"));
            }
        }
    }
    for (id, d) in a.dep_tops() {
        if wanted(d) && seen.insert(d.name.to_string()) {
            items.push(def_item(cx, a, id, d, "2"));
        }
    }
    // After `"abc".`, only the functions that take a text first.
    let receiver = if after_dot { receiver_type(&before[..before.len() - 1]) } else { None };
    let fits = |b: &Builtin| !after_dot || (b.max > 0 && receiver.as_ref().is_none_or(|t| takes_first(b.name, b.min, t)));
    for b in BUILTINS {
        if fits(b) && seen.insert(b.name.to_string()) {
            items.push(item(b.name, kind::FUNCTION, Some(b.sig.to_string()), Some(builtin_markdown(b)), "3", cx));
        }
    }
    // The Ruby names: `upcase`, `select`, `empty?`...
    for alias in builtins::ALIASES {
        let Some(b) = builtins::lookup(alias.of) else { continue };
        if fits(b) && seen.insert(alias.name.to_string()) {
            let detail = t!("same as {}", "pareil que {}", b.sig);
            let doc = format!("{}\n\n{}", alias_line(alias.name, alias.of), builtin_markdown(b));
            items.push(item(alias.name, kind::FUNCTION, Some(detail), Some(doc), "4", cx));
        }
    }
    if after_dot {
        for f in &ix.fields {
            if seen.insert(f.to_string()) {
                items.push(item(f, kind::FIELD, Some(t!("field", "champ")), None, "1", cx));
            }
        }
        return Json::Arr(items);
    }
    if seen.insert("pi".into()) {
        items.push(item("pi", kind::CONSTANT, Some("3.141592653589793".into()), Some(t!("The number π.", "Le nombre π.")), "3", cx));
    }
    for p in super::index::PRELUDE {
        if seen.insert(p.to_string()) {
            items.push(item(p, kind::ENUM_MEMBER, None, Some(prelude_markdown(p)), "3", cx));
        }
    }
    for k in KEYWORDS.iter().chain(&["unless"]) {
        items.push(item(k, kind::KEYWORD, Some(t!("keyword", "mot-clé")), keyword_markdown(k), "4", cx));
    }
    if cx.snippets {
        for (label, filter, body, detail) in snippets() {
            let mut it = item(label, kind::SNIPPET, Some(detail), Some(code_block(&body.replace('\t', "  "))), "5", cx);
            it.set("filterText", filter.into());
            it.set("insertText", body.into());
            it.set("insertTextFormat", 2u32.into());
            items.push(it);
        }
    }
    Json::Arr(items)
}

/// After `use ` without quotes: what is typed so far (`shapes/ci`).
fn short_use_line(text: &str, offset: usize) -> Option<&str> {
    let line_start = text[..offset].rfind(['\n', '\r']).map_or(0, |i| i + 1);
    let rest = text[line_start..offset].trim_start().strip_prefix("use")?;
    if !rest.starts_with([' ', '\t']) {
        return None;
    }
    let typed = rest.trim_start();
    typed.chars().all(|c| scan::is_name_char(c) || c == '/' || c == '.').then_some(typed)
}

/// The files that `use name` can name: the Rythme files of the folder
/// (without `.ry`) and its folders, after `use `.
fn short_use_completion(cx: &Ctx, a: &Analysis, offset: usize) -> Json {
    let Some(typed) = short_use_line(&a.text, offset) else { return Json::Arr(Vec::new()) };
    let (dir, _) = typed.rsplit_once('/').unwrap_or(("", typed));
    let replace_from = offset - typed.len() + if dir.is_empty() { 0 } else { dir.len() + 1 };
    let folder = a.module.base.join(dir);
    let own = a.path.as_deref().and_then(Path::file_name).map(|f| f.to_string_lossy().to_string());
    let mut entries: Vec<(String, bool)> = std::fs::read_dir(&folder)
        .map(|rd| {
            rd.flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    let is_dir = e.file_type().is_ok_and(|t| t.is_dir());
                    let stem = if is_dir { Some(name.as_str()) } else { name.strip_suffix(".ry") };
                    let ok =
                        stem.is_some_and(|s| s.starts_with(|c: char| c.is_lowercase() || c == '_') && s.chars().all(scan::is_name_char));
                    let own = !is_dir && dir.is_empty() && own.as_deref() == Some(name.as_str());
                    (ok && !own).then(|| (stem.unwrap_or_default().to_string(), is_dir))
                })
                .collect()
        })
        .unwrap_or_default();
    entries.sort();
    let range = range_in(&a.text, &a.lines, replace_from, offset, cx.enc);
    let mut items = Vec::new();
    for (name, is_dir) in entries.into_iter().take(500) {
        let shown = if is_dir { format!("{name}/") } else { name.clone() };
        let detail = if is_dir { t!("folder", "dossier") } else { format!("{name}.ry") };
        let mut it = obj([("label", shown.clone().into()), ("kind", if is_dir { kind::FOLDER } else { kind::FILE }.into())]);
        it.set("detail", detail.into());
        it.set("textEdit", obj([("range", range.clone()), ("newText", shown.into())]));
        items.push(it);
    }
    Json::Arr(items)
}

/// File names inside `use "..."`.
fn use_completion(cx: &Ctx, a: &Analysis, offset: usize, quote: Option<usize>) -> Json {
    let Some(q) = quote else { return Json::Arr(Vec::new()) };
    let line_start = a.text[..q].rfind(['\n', '\r']).map_or(0, |i| i + 1);
    if a.text[line_start..q].trim() != "use" {
        return Json::Arr(Vec::new());
    }
    let typed = &a.text[q + 1..offset];
    let (dir, _) = typed.rsplit_once('/').unwrap_or(("", typed));
    let replace_from = if dir.is_empty() { q + 1 } else { q + 1 + dir.len() + 1 };
    let folder = a.module.base.join(dir);
    let mut items = Vec::new();
    let mut entries: Vec<(String, bool)> = std::fs::read_dir(&folder)
        .map(|rd| {
            rd.flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    let is_dir = e.file_type().is_ok_and(|t| t.is_dir());
                    (!name.starts_with('.') && (is_dir || name.ends_with(".ry"))).then_some((name, is_dir))
                })
                .collect()
        })
        .unwrap_or_default();
    entries.sort();
    let own = a.path.as_deref().and_then(Path::file_name).map(|f| f.to_string_lossy().to_string());
    let range = range_in(&a.text, &a.lines, replace_from, offset, cx.enc);
    for (name, is_dir) in entries.into_iter().take(500) {
        if !is_dir && dir.is_empty() && own.as_deref() == Some(name.as_str()) {
            continue;
        }
        let shown = if is_dir { format!("{name}/") } else { name };
        let mut it = obj([("label", shown.clone().into()), ("kind", if is_dir { kind::FOLDER } else { kind::FILE }.into())]);
        it.set("textEdit", obj([("range", range.clone()), ("newText", shown.into())]));
        items.push(it);
    }
    Json::Arr(items)
}

// ----- signature help -----------------------------------------------------

/// The parameters of a signature like `map(list, f)`, as byte ranges of the label.
fn params_of(label: &str) -> Vec<(usize, usize)> {
    let (Some(open), Some(close)) = (label.find('('), label.rfind(')')) else { return Vec::new() };
    if close <= open + 1 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut start = open + 1;
    for (i, c) in label[open + 1..close].char_indices() {
        if c == ',' {
            out.push((start, open + 1 + i));
            start = open + 1 + i + 1;
            while label[start..].starts_with(' ') {
                start += 1;
            }
        }
    }
    out.push((start, close));
    out
}

pub fn signature_help(cx: &Ctx, a: &Analysis, offset: usize) -> Json {
    let sc = scan::scan(&a.text, offset);
    if sc.place == Place::Comment {
        return Json::Null;
    }
    // `fn(`, `if (`: not calls; the call around them is.
    let Some(call) = sc.calls.iter().rev().find(|c| !KEYWORDS.contains(&c.name.as_str())) else { return Json::Null };
    let target = match a.index().occ_at(call.at) {
        Some(o) if *o.name == *call.name => o.target.clone(),
        _ => match a.index().locals_at(offset).into_iter().find(|(_, d)| *d.name == *call.name) {
            Some((idx, _)) => Target::Def(DefId { file: a.module.file, idx }),
            None => by_name(a, &call.name).unwrap_or(Target::Unknown),
        },
    };
    let (label, doc): (String, Option<String>) = match &target {
        Target::Builtin(name) => match builtins::lookup(name) {
            Some(b) => (b.sig.to_string(), Some(b.doc().to_string())),
            None => return Json::Null,
        },
        Target::Def(id) => match a.def(*id) {
            Some(d) => match &d.sig {
                Some(sig) => (sig.clone(), d.doc.as_ref().map(|s| s.to_string())),
                None => return Json::Null,
            },
            None => return Json::Null,
        },
        Target::Prelude(p) if *p != "None" => {
            let sig = match *p {
                "Some" => "Some(value)",
                "Ok" => "Ok(value)",
                _ => "Err(error)",
            };
            (sig.to_string(), None)
        }
        _ => return Json::Null,
    };
    // A built-in may show several forms: `random(a, b), random(list)`.
    let forms: Vec<String> = label.split("), ").map(|f| if f.ends_with(')') { f.to_string() } else { format!("{f})") }).collect();
    let given = call.commas + usize::from(call.first_given);
    // The first form with room for the arguments given, else the longest.
    let chosen = forms
        .iter()
        .position(|f| params_of(f).len() > given)
        .unwrap_or_else(|| forms.iter().enumerate().max_by_key(|(_, f)| params_of(f).len()).map(|(i, _)| i).unwrap_or(0));
    let params = params_of(&forms[chosen]);
    let mut active = given;
    if !params.is_empty() {
        active = active.min(params.len() - 1);
    }
    let enc = cx.enc;
    let signatures = forms
        .iter()
        .map(|form| {
            let param_json: Vec<Json> = params_of(form)
                .iter()
                .map(|&(s, e)| obj([("label", Json::Arr(vec![enc.len(&form[..s]).into(), enc.len(&form[..e]).into()]))]))
                .collect();
            let mut sig = obj([("label", form.clone().into()), ("parameters", Json::Arr(param_json))]);
            if let Some(d) = &doc {
                sig.set("documentation", markup(cx, d.clone()));
            }
            sig
        })
        .collect();
    obj([("signatures", Json::Arr(signatures)), ("activeSignature", chosen.into()), ("activeParameter", active.into())])
}

// ----- definition, highlights, outline ------------------------------------

pub fn definition(cx: &Ctx, a: &Analysis, offset: usize) -> Json {
    let o = offset as u32;
    for (span, _) in &a.index().uses {
        if span.start <= o && o <= span.end {
            if let Some(path) = used_file(a, *span) {
                let uri = a.modules.get(path).map(|m| m.uri.clone()).unwrap_or_else(|| super::text::path_to_uri(path));
                let zero = obj([("start", pos_json(Pos { line: 0, character: 0 })), ("end", pos_json(Pos { line: 0, character: 0 }))]);
                return obj([("uri", uri.into()), ("range", zero)]);
            }
        }
    }
    let target = match a.index().occ_at(offset) {
        Some(occ) => occ.target.clone(),
        None => match scan::word_at(&a.text, offset).filter(|_| !a.module.parsed) {
            Some((s, e)) => by_name(a, &a.text[s..e]).unwrap_or(Target::Unknown),
            None => Target::Unknown,
        },
    };
    match target {
        Target::Def(id) => a.def(id).and_then(|d| location_json(cx, a, d.span)).unwrap_or(Json::Null),
        _ => Json::Null,
    }
}

pub fn highlights(cx: &Ctx, a: &Analysis, offset: usize) -> Json {
    let Some(occ) = a.index().occ_at(offset) else { return Json::Null };
    let Target::Def(id) = occ.target else { return Json::Null };
    let out = a
        .index()
        .occs
        .iter()
        .filter(|o| o.target == Target::Def(id))
        .map(|o| {
            let range = range_in(&a.text, &a.lines, o.span.start as usize, o.span.end as usize, cx.enc);
            obj([("range", range), ("kind", if o.def { 3u32 } else { 2u32 }.into())])
        })
        .collect();
    Json::Arr(out)
}

/// Symbol kinds of the protocol.
fn symbol_kind(d: &Def) -> u32 {
    match d.kind {
        DefKind::Function => 12,
        DefKind::Type => 10,
        DefKind::Ctor => 22,
        DefKind::Test => 24,
        _ => 13,
    }
}

pub fn document_symbols(cx: &Ctx, a: &Analysis) -> Json {
    let ix = a.index();
    let range = |s: Span| range_in(&a.text, &a.lines, s.start as usize, s.end as usize, cx.enc);
    let name_of = |d: &Def| if d.kind == DefKind::Test { format!("test {:?}", d.name) } else { d.name.to_string() };
    let shown = |d: &Def| d.top && d.kind != DefKind::Ctor || d.kind == DefKind::Function;
    if !cx.hierarchical_symbols {
        let out = ix
            .defs
            .iter()
            .filter(|d| shown(d) || d.kind == DefKind::Ctor)
            .map(|d| {
                let mut s = obj([
                    ("name", name_of(d).into()),
                    ("kind", symbol_kind(d).into()),
                    ("location", obj([("uri", a.uri.clone().into()), ("range", range(d.full))])),
                ]);
                if let Some(owner) = d.owner.as_ref().filter(|_| d.kind == DefKind::Ctor) {
                    s.set("containerName", owner.to_string().into());
                }
                s
            })
            .collect();
        return Json::Arr(out);
    }
    let symbol = |d: &Def, children: Vec<Json>| {
        let mut s = obj([
            ("name", name_of(d).into()),
            ("kind", symbol_kind(d).into()),
            ("range", range(d.full)),
            ("selectionRange", range(d.span)),
        ]);
        if let Some(sig) = &d.sig {
            s.set("detail", sig.clone().into());
        }
        if !children.is_empty() {
            s.set("children", Json::Arr(children));
        }
        s
    };
    let mut out = Vec::new();
    for (i, d) in ix.defs.iter().enumerate() {
        if !d.top || d.kind == DefKind::Ctor {
            continue;
        }
        let children: Vec<Json> = match d.kind {
            DefKind::Type => ix.defs[i + 1..].iter().take_while(|c| c.kind == DefKind::Ctor).map(|c| symbol(c, Vec::new())).collect(),
            DefKind::Function => ix
                .defs
                .iter()
                .filter(|c| !c.top && c.kind == DefKind::Function && d.full.start <= c.span.start && c.span.end <= d.full.end)
                .map(|c| symbol(c, Vec::new()))
                .collect(),
            _ => Vec::new(),
        };
        out.push(symbol(d, children));
    }
    Json::Arr(out)
}

// ----- quick fixes --------------------------------------------------------

/// The name proposed by a "did you mean `x`?" help, in either language.
fn proposed_name(help: &str) -> Option<&str> {
    let rest = help.strip_prefix("did you mean `").or_else(|| help.strip_prefix("vouliez-vous dire `"))?;
    rest.split('`').next()
}

/// The file proposed by an "add `use geometry`" help, in either language.
fn proposed_use(help: &str) -> Option<&str> {
    let (_, rest) = help.split_once(": add `use ").or_else(|| help.split_once(" : ajoutez `use "))?;
    rest.split('`').next()
}

/// Where a new `use` line goes: after the last `use` line at the top, or at the start.
fn new_use_offset(a: &Analysis) -> usize {
    let last = a.index().uses.iter().map(|(span, _)| span.end as usize).max();
    last.map_or(0, |end| a.text[end..].find('\n').map_or(a.text.len(), |i| end + i + 1))
}

pub fn code_actions(cx: &Ctx, a: &Analysis, start: usize, end: usize) -> Json {
    let mut out = Vec::new();
    for d in a.diagnostics() {
        let Some(span) = d.span.filter(|s| s.file == a.module.file) else { continue };
        let (s, e) = (span.start as usize, span.end as usize);
        if e < start || s > end {
            continue;
        }
        let Some(written) = a.text.get(s..e) else { continue };
        for h in &d.helps {
            if let Some(file) = proposed_use(h) {
                let at = new_use_offset(a);
                let nl = if at > 0 && !a.text[..at].ends_with('\n') { "\n" } else { "" };
                let stem = file.rsplit('/').next().unwrap_or(file);
                let add = obj([("range", range_in(&a.text, &a.lines, at, at, cx.enc)), ("newText", format!("{nl}use {file}\n").into())]);
                let name = obj([("range", range_in(&a.text, &a.lines, s, e, cx.enc)), ("newText", format!("{stem}.{written}").into())]);
                let changes = Json::Obj(vec![(a.uri.clone(), Json::Arr(vec![add, name]))]);
                out.push(obj([
                    (
                        "title",
                        t!("Add `use {file}` and write `{stem}.{written}`", "Ajouter `use {file}` et écrire `{stem}.{written}`").into(),
                    ),
                    ("kind", "quickfix".into()),
                    ("diagnostics", Json::Arr(vec![diagnostic_json(cx, a, d)])),
                    ("isPreferred", true.into()),
                    ("edit", obj([("changes", changes)])),
                ]));
                continue;
            }
            let Some(name) = proposed_name(h) else { continue };
            // Only a name is replaced by a name; `use "file"` gets its file name replaced.
            let (from, to, new) = if !written.is_empty() && written.chars().all(|c| scan::is_name_char(c) || c == '.') {
                (s, e, name.to_string())
            } else if let (Some(q1), Some(q2)) = (written.find('"'), written.rfind('"')) {
                if q2 <= q1 || !written.trim_start().starts_with("use") {
                    continue;
                }
                (s + q1 + 1, s + q2, name.to_string())
            } else {
                continue;
            };
            let edit = obj([("range", range_in(&a.text, &a.lines, from, to, cx.enc)), ("newText", new.clone().into())]);
            let changes = Json::Obj(vec![(a.uri.clone(), Json::Arr(vec![edit]))]);
            out.push(obj([
                ("title", t!("Replace with `{new}`", "Remplacer par `{new}`").into()),
                ("kind", "quickfix".into()),
                ("diagnostics", Json::Arr(vec![diagnostic_json(cx, a, d)])),
                ("isPreferred", true.into()),
                ("edit", obj([("changes", changes)])),
            ]));
        }
    }
    Json::Arr(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameters_of_a_signature() {
        let label = "map(list, f)";
        let p: Vec<&str> = params_of(label).into_iter().map(|(s, e)| &label[s..e]).collect();
        assert_eq!(p, ["list", "f"]);
        let label = "print(value, ...)";
        let p: Vec<&str> = params_of(label).into_iter().map(|(s, e)| &label[s..e]).collect();
        assert_eq!(p, ["value", "..."]);
        assert!(params_of("time()").is_empty());
        assert_eq!(proposed_name("vouliez-vous dire `print` ?"), Some("print"));
    }
}
