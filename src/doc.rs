//! `rythme doc`: the documentation of a file or a folder, written from its
//! `##` comments, its types and the definitions of its functions, as Markdown
//! or as one HTML page that needs nothing else.
//!
//! A file is documented with the files it uses (`use "x.ry"`), each one once.
//! Names starting with `_` are left out: they are details of their file. A
//! name written between backticks in a `##` comment (`area`, `len(list)`)
//! becomes a link to its definition, or to the documentation of the built-in
//! function, which is added at the end.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::ast::{ExprKind, Item, PatKind, Stmt};
use crate::builtins::{self, Builtin};
use crate::diagnostic::{self, Diagnostic};
use crate::lexer::{self, Tok};
use crate::{driver, format, io, parser, source, suggest};

/// One documented file.
struct FileDoc {
    shown: String,
    /// The `##` comment at the top of the file, separated from what follows by a blank line.
    summary: Option<String>,
    /// The files it uses, as shown (each one is documented too).
    uses: Vec<String>,
    types: Vec<Entry>,
    functions: Vec<Entry>,
    values: Vec<Entry>,
    tests: Vec<String>,
}

/// A type, a function or a value: its name, its code (the definition of a
/// type, the first line of each definition of a function), and its `##` comment.
struct Entry {
    name: String,
    code: String,
    doc: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Markdown,
    Html,
}

/// `rythme doc [--html] [--out FILE] FILE_OR_FOLDER...`. Returns the exit code.
pub fn command(args: &[String]) -> i32 {
    let mut html = false;
    let mut out: Option<String> = None;
    let mut targets = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "--html" => html = true,
            "--markdown" | "--md" => html = false,
            "-o" | "--out" | "--sortie" => {
                let Some(file) = args.get(i + 1) else {
                    return usage(t!(
                        "`{a}` needs a file name, like `rythme doc geometry.ry --out geometry.html`",
                        "`{a}` attend un nom de fichier, comme `rythme doc geometrie.ry --out geometrie.html`"
                    ));
                };
                out = Some(file.clone());
                i += 1;
            }
            "--help" | "-h" | "--aide" => {
                io::out_line(crate::help::topic("doc").unwrap_or_default().trim_end());
                return 0;
            }
            o if o.starts_with('-') && o.len() > 1 => {
                let mut msg = t!("unknown option `{o}` for `rythme doc`", "option inconnue `{o}` pour `rythme doc`");
                if let Some(best) = suggest::closest(o, ["--html", "--markdown", "--out"]) {
                    msg.push_str(&t!("\ndid you mean `{best}`?", "\nvouliez-vous dire `{best}` ?"));
                }
                msg.push_str(&t!("\nsee `rythme help doc`", "\nvoir `rythme help doc`"));
                return usage(msg);
            }
            _ => targets.push(a),
        }
        i += 1;
    }
    if targets.is_empty() {
        return usage(t!(
            "which files? Example: rythme doc geometry.ry\n  a folder documents all its .ry files: rythme doc .\n  --html writes one HTML page instead of Markdown\n  --out FILE writes to a file (a name ending with .html gives HTML)\nsee `rythme help doc`",
            "quels fichiers ? Exemple : rythme doc geometrie.ry\n  un dossier documente tous ses fichiers .ry : rythme doc .\n  --html écrit une page HTML au lieu de Markdown\n  --out FICHIER écrit dans un fichier (un nom finissant par .html donne du HTML)\nvoir `rythme help doc`"
        ));
    }
    if out.as_deref().is_some_and(|o| o.ends_with(".html") || o.ends_with(".htm")) {
        html = true;
    }
    let mut docs = Docs { files: Vec::new(), seen: HashSet::new(), errors: 0, color: diagnostic::use_color() };
    for target in &targets {
        // `rythme doc geometry` documents `geometry.ry`, like `rythme run geometry`.
        let with_ext = PathBuf::from(format!("{target}.ry"));
        let path = if !Path::new(target).exists() && Path::new(target).extension().is_none() && with_ext.is_file() {
            with_ext
        } else {
            PathBuf::from(target)
        };
        if path.is_dir() {
            let mut found = Vec::new();
            format::ry_files(&path, &mut found);
            if found.is_empty() {
                io::err_line(&t!("no .ry file in `{target}`", "aucun fichier .ry dans `{target}`"));
            }
            for f in found {
                docs.load(&f);
            }
        } else if path.is_file() {
            docs.load(&path);
        } else {
            docs.report(&format::not_found(&path));
        }
    }
    let title = match targets.as_slice() {
        [_] if docs.files.len() == 1 => docs.files[0].shown.clone(),
        [one] => {
            // `.` is named after the current folder.
            let full = Path::new(one).canonicalize().ok();
            let one = match full.as_deref().and_then(Path::file_name) {
                Some(name) if matches!(*one, "." | "./") => name.to_string_lossy().to_string(),
                _ => one.trim_end_matches(['/', '\\']).to_string(),
            };
            t!("Documentation of {one}", "Documentation de {one}")
        }
        _ => t!("Documentation", "Documentation"),
    };
    let format = if html { Format::Html } else { Format::Markdown };
    let text = render(&docs.files, &title, format);
    match out {
        Some(file) => match std::fs::write(&file, &text) {
            Ok(()) => io::out_line(&t!("written: {file}", "écrit : {file}")),
            Err(e) => {
                docs.report(&Diagnostic::error(
                    "E017",
                    t!("cannot write the file `{file}`: {e}", "impossible d'écrire le fichier `{file}` : {e}"),
                ));
            }
        },
        None if !docs.files.is_empty() => io::out(&text),
        None => {}
    }
    if docs.errors > 0 {
        1
    } else {
        0
    }
}

fn usage(msg: String) -> i32 {
    io::err_line(&msg);
    2
}

struct Docs {
    files: Vec<FileDoc>,
    seen: HashSet<PathBuf>,
    errors: usize,
    color: bool,
}

impl Docs {
    fn report(&mut self, d: &Diagnostic) {
        if d.is_error() {
            self.errors += 1;
        }
        io::err(&format!("{}\n", d.render(self.color)));
    }

    /// Reads a file and the files it uses, each one once.
    fn load(&mut self, path: &Path) {
        let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if !self.seen.insert(canon) {
            return;
        }
        // `geometry.ry` rather than `./geometry.ry` for the files of the current folder.
        let shown = path.strip_prefix(".").ok().filter(|p| !p.as_os_str().is_empty()).unwrap_or(path).display().to_string();
        let src = match driver::read_source(path) {
            Ok(s) => s,
            Err(d) => return self.report(&d),
        };
        let file = source::add(&shown, &src);
        let lexed = match lexer::lex(&src, file) {
            Ok(l) => l,
            Err(d) => return self.report(&d),
        };
        // The `##` comments by the position of what they document.
        let doc_at: HashMap<u32, String> = lexed.docs.iter().map(|(i, d)| (lexed.tokens[*i].span.start, d.clone())).collect();
        let items = match parser::parse(lexed) {
            Ok(items) => items,
            Err(d) => return self.report(&d),
        };
        let text = |s: crate::source::Span| src.get(s.start as usize..s.end as usize).unwrap_or("").trim().to_string();
        let mut doc = FileDoc {
            shown,
            summary: summary(&src),
            uses: Vec::new(),
            types: Vec::new(),
            functions: Vec::new(),
            values: Vec::new(),
            tests: Vec::new(),
        };
        let mut used = Vec::new();
        for item in &items {
            match item {
                Item::Type(t) => {
                    let variants: Vec<String> = t
                        .variants
                        .iter()
                        .map(|v| {
                            let fields: Vec<String> = v.fields.iter().map(|f| f.name().to_string()).collect();
                            if fields.is_empty() {
                                v.name.name().to_string()
                            } else {
                                format!("{}({})", v.name.name(), fields.join(", "))
                            }
                        })
                        .collect();
                    let one_line = format!("type {} = {}", t.name.name(), variants.join(" | "));
                    let code = if one_line.chars().count() <= 80 {
                        one_line
                    } else {
                        format!("type {} =\n{}", t.name.name(), variants.iter().map(|v| format!("  | {v}")).collect::<Vec<_>>().join("\n"))
                    };
                    doc.types.push(Entry { name: t.name.name().to_string(), code, doc: t.doc.as_deref().map(str::to_string) });
                }
                Item::Stmt(Stmt::Fn(f)) => {
                    let Some(name) = f.name.map(|n| n.name().to_string()).filter(|n| !n.starts_with('_')) else { continue };
                    let code: Vec<String> = f.clauses.iter().map(|c| one_line(&text(c.span))).collect();
                    doc.functions.push(Entry { name, code: code.join("\n"), doc: f.doc.as_deref().map(str::to_string) });
                }
                Item::Stmt(Stmt::Let(p, e)) => {
                    let PatKind::Bind(s) = &p.kind else { continue };
                    let name = s.name().to_string();
                    if name.starts_with('_') {
                        continue;
                    }
                    let comment = doc_at.get(&p.span.start).cloned();
                    if let ExprKind::Lambda(f) = &e.kind {
                        let code: Vec<String> = f.clauses.iter().map(|c| format!("{name} = {}", one_line(&text(c.span)))).collect();
                        doc.functions.push(Entry {
                            name,
                            code: code.join("\n"),
                            doc: comment.or_else(|| f.doc.as_deref().map(str::to_string)),
                        });
                        continue;
                    }
                    let whole = text(p.span.to(e.span));
                    let code = if whole.contains('\n') || whole.chars().count() > 80 { format!("{name} = …") } else { whole };
                    doc.values.push(Entry { name, code, doc: comment });
                }
                Item::Test(name, _, _) => doc.tests.push(name.to_string()),
                Item::Use(p, span) => {
                    let base = path.parent().unwrap_or(Path::new(""));
                    match driver::find_use(p, *span, base) {
                        Ok(found) => {
                            doc.uses.push(found.display().to_string());
                            used.push(found);
                        }
                        Err(d) => self.report(&d),
                    }
                }
                Item::Stmt(Stmt::Expr(_)) => {}
            }
        }
        self.files.push(doc);
        for p in used {
            self.load(&p);
        }
    }
}

/// The first line of a definition written on several lines.
fn one_line(code: &str) -> String {
    code.lines().map(str::trim).collect::<Vec<_>>().join(" ")
}

/// The `##` lines that start a file, when a blank line separates them from what follows.
fn summary(src: &str) -> Option<String> {
    let mut lines = src.trim_start_matches('\u{feff}').lines().skip_while(|l| l.trim().is_empty());
    let mut found = Vec::new();
    for line in lines.by_ref() {
        let l = line.trim();
        if !l.starts_with("##") {
            // A blank line ends the summary; anything else means the comment documents a definition.
            return (l.is_empty() && !found.is_empty()).then(|| found.join("\n"));
        }
        let text = l.trim_start_matches('#');
        found.push(text.strip_prefix(' ').unwrap_or(text).trim_end().to_string());
    }
    None
}

// ----- rendering -----------------------------------------------------------------

/// Labels, in the language of the messages.
fn label(what: &str) -> String {
    match what {
        "types" => t!("Types", "Types"),
        "functions" => t!("Functions", "Fonctions"),
        "values" => t!("Values", "Valeurs"),
        "tests" => t!("Tests", "Tests"),
        "uses" => t!("Uses", "Utilise"),
        "contents" => t!("Contents", "Sommaire"),
        "builtins" => t!("Built-in functions mentioned", "Fonctions intégrées citées"),
        "example" => t!("Example", "Exemple"),
        "empty" => t!("Nothing public to document.", "Rien de public à documenter."),
        "generated" => t!("Documentation written by `rythme doc`", "Documentation écrite par `rythme doc`"),
        _ => what.to_string(),
    }
}

/// The anchors of the headings, computed like GitHub does for Markdown, so
/// that links work on GitHub and in the HTML page alike.
#[derive(Default)]
struct Anchors {
    used: HashMap<String, usize>,
}

impl Anchors {
    fn make(&mut self, heading: &str) -> String {
        let base: String = heading
            .to_lowercase()
            .chars()
            .filter_map(|c| match c {
                ' ' => Some('-'),
                c if c.is_alphanumeric() || c == '-' || c == '_' => Some(c),
                _ => None,
            })
            .collect();
        let n = self.used.entry(base.clone()).or_insert(0);
        let anchor = if *n == 0 { base.clone() } else { format!("{base}-{n}") };
        *n += 1;
        anchor
    }
}

/// What the page is made of, with the anchor of every heading.
struct Plan<'a> {
    files: Vec<PlannedFile<'a>>,
    builtins: Vec<(&'static Builtin, String)>,
    builtins_anchor: String,
    /// Where each name is documented, for the links of the comments.
    targets: HashMap<String, String>,
}

/// A part of a file (its types, its functions...): its title, its anchor, and
/// its entries with their anchors.
type Section<'a> = (String, String, Vec<(&'a Entry, String)>);

struct PlannedFile<'a> {
    doc: &'a FileDoc,
    anchor: String,
    sections: Vec<Section<'a>>,
    tests_anchor: Option<String>,
}

fn plan<'a>(files: &'a [FileDoc], title: &str) -> Plan<'a> {
    let mut anchors = Anchors::default();
    // Several files: the Markdown text starts with the title and the contents.
    if files.len() > 1 {
        anchors.make(title);
        anchors.make(&label("contents"));
    }
    let mut targets = HashMap::new();
    let mut planned = Vec::new();
    for f in files {
        let anchor = anchors.make(&f.shown);
        let mut sections = Vec::new();
        for (key, entries) in [("types", &f.types), ("functions", &f.functions), ("values", &f.values)] {
            if entries.is_empty() {
                continue;
            }
            let section = anchors.make(&label(key));
            let mut list = Vec::new();
            for e in entries {
                let a = anchors.make(&e.name);
                targets.entry(e.name.clone()).or_insert_with(|| a.clone());
                list.push((e, a));
            }
            sections.push((label(key), section, list));
        }
        // A constructor leads to its type.
        for t in &f.types {
            let a = targets.get(&t.name).cloned().unwrap_or_default();
            for ctor in t.code.split(['=', '|']).skip(1) {
                let name: String = ctor.trim().chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if !name.is_empty() {
                    targets.entry(name).or_insert_with(|| a.clone());
                }
            }
        }
        let tests_anchor = (!f.tests.is_empty()).then(|| anchors.make(&label("tests")));
        planned.push(PlannedFile { doc: f, anchor, sections, tests_anchor });
    }
    // Built-in functions named in the comments, in the order they are first named.
    let mut builtins = Vec::new();
    let mut named = HashSet::new();
    let comments = files
        .iter()
        .flat_map(|f| f.summary.iter().chain(f.types.iter().chain(&f.functions).chain(&f.values).filter_map(|e| e.doc.as_ref())));
    for text in comments {
        for (code, _) in code_spans(text) {
            let name = leading_name(code);
            if targets.contains_key(name) || !named.insert(name.to_string()) {
                continue;
            }
            if let Some(b) = builtins::lookup(name) {
                builtins.push(b);
            }
        }
    }
    let builtins_anchor = if builtins.is_empty() { String::new() } else { anchors.make(&label("builtins")) };
    let builtins: Vec<(&'static Builtin, String)> = builtins.into_iter().map(|b| (b, anchors.make(b.name))).collect();
    for (b, a) in &builtins {
        targets.insert(b.name.to_string(), a.clone());
    }
    Plan { files: planned, builtins, builtins_anchor, targets }
}

/// The parts of a comment written between backticks: (code, byte range of the whole span).
fn code_spans(text: &str) -> Vec<(&str, (usize, usize))> {
    let mut out = Vec::new();
    let mut rest = 0;
    while let Some(open) = text[rest..].find('`').map(|i| rest + i) {
        let Some(close) = text[open + 1..].find('`').map(|i| open + 1 + i) else { break };
        out.push((&text[open + 1..close], (open, close + 1)));
        rest = close + 1;
    }
    out
}

/// `len` for `len(list)`: the name a code span is about, when it starts with one.
fn leading_name(code: &str) -> &str {
    let end = code.find(|c: char| !(c.is_alphanumeric() || c == '_')).unwrap_or(code.len());
    let (name, after) = code.split_at(end);
    if after.is_empty() || after.starts_with('(') {
        name
    } else {
        ""
    }
}

fn render(files: &[FileDoc], title: &str, format: Format) -> String {
    let plan = plan(files, title);
    match format {
        Format::Markdown => markdown(&plan, title),
        Format::Html => html(&plan, title),
    }
}

// ----- Markdown ------------------------------------------------------------------

fn markdown(plan: &Plan, title: &str) -> String {
    let mut out = String::new();
    let md_comment = |text: &str| link_comment(text, &plan.targets, Format::Markdown);
    let code = |code: &str| format!("~~~rythme\n{code}\n~~~\n\n");
    if plan.files.len() > 1 {
        out.push_str(&format!("# {title}\n\n## {}\n\n", label("contents")));
        for f in &plan.files {
            out.push_str(&format!("- [{}](#{})\n", f.doc.shown, f.anchor));
        }
        out.push('\n');
    }
    let level = if plan.files.len() > 1 { "##" } else { "#" };
    for f in &plan.files {
        out.push_str(&format!("{level} {}\n\n", f.doc.shown));
        if let Some(s) = &f.doc.summary {
            out.push_str(&format!("{}\n\n", md_comment(s)));
        }
        if !f.doc.uses.is_empty() {
            let links: Vec<String> = f.doc.uses.iter().map(|u| format!("[`{u}`](#{})", file_anchor(plan, u))).collect();
            out.push_str(&format!("{}{}{}\n\n", label("uses"), sep(), links.join(", ")));
        }
        if f.sections.is_empty() && f.tests_anchor.is_none() {
            out.push_str(&format!("*{}*\n\n", label("empty")));
        }
        for (name, _, entries) in &f.sections {
            out.push_str(&format!("{level}# {name}\n\n"));
            for (e, _) in entries {
                out.push_str(&format!("{level}## {}\n\n", e.name));
                out.push_str(&code(&e.code));
                if let Some(d) = &e.doc {
                    out.push_str(&format!("{}\n\n", md_comment(d)));
                }
            }
        }
        if f.tests_anchor.is_some() {
            out.push_str(&format!("{level}# {}\n\n", label("tests")));
            for t in &f.doc.tests {
                out.push_str(&format!("- {t}\n"));
            }
            out.push('\n');
        }
    }
    if !plan.builtins.is_empty() {
        out.push_str(&format!("## {}\n\n", label("builtins")));
        for (b, _) in &plan.builtins {
            out.push_str(&format!("### {}\n\n", b.name));
            out.push_str(&code(b.sig));
            out.push_str(&format!("{}\n\n{}\n\n", b.doc(), t!("Example:", "Exemple :")));
            out.push_str(&code(b.example));
        }
    }
    format!("{}\n", out.trim_end())
}

fn file_anchor(plan: &Plan, shown: &str) -> String {
    plan.files.iter().find(|f| f.doc.shown == shown).map(|f| f.anchor.clone()).unwrap_or_default()
}

/// A `##` comment, its names between backticks made links. In HTML, the text
/// is also escaped, a blank line separates paragraphs and lines starting with
/// `- ` make a list.
fn link_comment(text: &str, targets: &HashMap<String, String>, format: Format) -> String {
    let mut out = String::new();
    let mut last = 0;
    for (code, (start, end)) in code_spans(text) {
        out.push_str(&plain(&text[last..start], format));
        let target = targets.get(leading_name(code)).filter(|_| !leading_name(code).is_empty());
        out.push_str(&match (format, target) {
            (Format::Markdown, Some(a)) => format!("[`{code}`](#{a})"),
            (Format::Markdown, None) => format!("`{code}`"),
            (Format::Html, Some(a)) => format!("<a href=\"#{a}\"><code>{}</code></a>", escape(code)),
            (Format::Html, None) => format!("<code>{}</code>", escape(code)),
        });
        last = end;
    }
    out.push_str(&plain(&text[last..], format));
    match format {
        Format::Markdown => out,
        Format::Html => paragraphs(&out),
    }
}

fn plain(text: &str, format: Format) -> String {
    match format {
        Format::Markdown => text.to_string(),
        Format::Html => escape(text),
    }
}

/// Paragraphs and lists of a comment already escaped.
fn paragraphs(text: &str) -> String {
    let mut out = String::new();
    for block in text.split("\n\n").map(str::trim).filter(|b| !b.is_empty()) {
        let mut para = Vec::new();
        let mut items: Vec<String> = Vec::new();
        let flush_para = |para: &mut Vec<&str>, out: &mut String| {
            if !para.is_empty() {
                out.push_str(&format!("<p>{}</p>\n", para.join("\n")));
                para.clear();
            }
        };
        for line in block.lines() {
            if let Some(item) = line.trim_start().strip_prefix("- ") {
                flush_para(&mut para, &mut out);
                items.push(item.to_string());
            } else if !items.is_empty() && line.starts_with(' ') {
                let last = items.last_mut().expect("an item");
                last.push(' ');
                last.push_str(line.trim());
            } else {
                if !items.is_empty() {
                    out.push_str(&list_html(&items));
                    items.clear();
                }
                para.push(line);
            }
        }
        flush_para(&mut para, &mut out);
        if !items.is_empty() {
            out.push_str(&list_html(&items));
        }
    }
    out
}

fn list_html(items: &[String]) -> String {
    let lis: String = items.iter().map(|i| format!("<li>{i}</li>")).collect();
    format!("<ul>{lis}</ul>\n")
}

// ----- HTML ----------------------------------------------------------------------

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

/// Rythme code with colors, using the lexer of the language.
fn highlight(code: &str) -> String {
    let Ok(lexed) = lexer::lex(code, u32::MAX) else { return escape(code) };
    let mut out = String::new();
    let mut last = 0;
    let mut after_fn = false;
    for t in &lexed.tokens {
        if matches!(t.tok, Tok::Newline | Tok::Eof) {
            continue;
        }
        let (start, end) = (t.span.start as usize, t.span.end as usize);
        if start < last || end > code.len() {
            continue;
        }
        out.push_str(&gap(&code[last..start]));
        let class = match &t.tok {
            Tok::Int(_) | Tok::Float(_) | Tok::IntMinMagnitude => Some("num"),
            Tok::Str(_) => Some("str"),
            Tok::Ctor(_) | Tok::True | Tok::False => Some("ctor"),
            Tok::Ident(_) if after_fn => Some("fn"),
            tok if lexer::keyword_text(tok).is_some() => Some("kw"),
            _ => None,
        };
        let text = escape(&code[start..end]);
        match class {
            Some(c) => out.push_str(&format!("<span class=\"{c}\">{text}</span>")),
            None => out.push_str(&text),
        }
        after_fn = matches!(t.tok, Tok::Fn);
        last = end;
    }
    out.push_str(&gap(&code[last..]));
    out
}

/// The text between two tokens: spaces, and maybe a comment.
fn gap(text: &str) -> String {
    match text.find('#') {
        Some(i) => {
            let end = text[i..].find('\n').map_or(text.len(), |j| i + j);
            format!("{}<span class=\"com\">{}</span>{}", escape(&text[..i]), escape(&text[i..end]), gap(&text[end..]))
        }
        None => escape(text),
    }
}

fn html(plan: &Plan, title: &str) -> String {
    let lang = if crate::lang::is_fr() { "fr" } else { "en" };
    let comment = |text: &str| link_comment(text, &plan.targets, Format::Html);
    let code = |code: &str| format!("<pre><code>{}</code></pre>\n", highlight(code));
    let mut nav = String::new();
    let mut main = String::new();
    for f in &plan.files {
        nav.push_str(&format!("<li><a href=\"#{}\">{}</a>", f.anchor, escape(&f.doc.shown)));
        main.push_str(&format!("<section class=\"file\" id=\"{}\">\n<h2>{}</h2>\n", f.anchor, escape(&f.doc.shown)));
        if let Some(s) = &f.doc.summary {
            main.push_str(&format!("<div class=\"summary\">{}</div>\n", comment(s)));
        }
        if !f.doc.uses.is_empty() {
            let links: Vec<String> =
                f.doc.uses.iter().map(|u| format!("<a href=\"#{}\"><code>{}</code></a>", file_anchor(plan, u), escape(u))).collect();
            main.push_str(&format!("<p class=\"uses\">{}{}{}</p>\n", escape(&label("uses")), sep(), links.join(", ")));
        }
        if f.sections.is_empty() && f.tests_anchor.is_none() {
            main.push_str(&format!("<p class=\"empty\">{}</p>\n", escape(&label("empty"))));
        }
        let mut sub = String::new();
        for (name, anchor, entries) in &f.sections {
            sub.push_str(&format!("<li><a href=\"#{anchor}\">{}</a><ul>", escape(name)));
            main.push_str(&format!("<h3 id=\"{anchor}\">{}</h3>\n", escape(name)));
            for (e, a) in entries {
                sub.push_str(&format!("<li><a href=\"#{a}\"><code>{}</code></a></li>", escape(&e.name)));
                main.push_str(&format!("<article class=\"entry\" id=\"{a}\">\n<h4><code>{}</code></h4>\n", escape(&e.name)));
                main.push_str(&code(&e.code));
                if let Some(d) = &e.doc {
                    main.push_str(&comment(d));
                }
                main.push_str("</article>\n");
            }
            sub.push_str("</ul></li>");
        }
        if let Some(a) = &f.tests_anchor {
            sub.push_str(&format!("<li><a href=\"#{a}\">{}</a></li>", escape(&label("tests"))));
            main.push_str(&format!("<h3 id=\"{a}\">{}</h3>\n<ul class=\"tests\">", escape(&label("tests"))));
            for t in &f.doc.tests {
                main.push_str(&format!("<li>{}</li>", escape(t)));
            }
            main.push_str("</ul>\n");
        }
        if !sub.is_empty() {
            nav.push_str(&format!("<ul>{sub}</ul>"));
        }
        nav.push_str("</li>");
        main.push_str("</section>\n");
    }
    if !plan.builtins.is_empty() {
        let a = &plan.builtins_anchor;
        nav.push_str(&format!("<li><a href=\"#{a}\">{}</a><ul>", escape(&label("builtins"))));
        main.push_str(&format!("<section class=\"file\" id=\"{a}\">\n<h2>{}</h2>\n", escape(&label("builtins"))));
        for (b, a) in &plan.builtins {
            nav.push_str(&format!("<li><a href=\"#{a}\"><code>{}</code></a></li>", b.name));
            main.push_str(&format!("<article class=\"entry\" id=\"{a}\">\n<h4><code>{}</code></h4>\n", b.name));
            main.push_str(&code(b.sig));
            main.push_str(&format!("<p>{}</p>\n<p class=\"label\">{}</p>\n", escape(b.doc()), escape(&label("example"))));
            main.push_str(&code(b.example));
            main.push_str("</article>\n");
        }
        nav.push_str("</ul></li>");
        main.push_str("</section>\n");
    }
    let footer = link_comment(&label("generated"), &HashMap::new(), Format::Html);
    let footer = footer.trim().trim_start_matches("<p>").trim_end_matches("</p>");
    let version = crate::VERSION;
    let title = escape(title);
    format!(
        r#"<!doctype html>
<html lang="{lang}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="generator" content="rythme doc {version}">
<title>{title}</title>
<style>
{STYLE}
</style>
</head>
<body>
<header class="bar"><span class="brand"><span class="beats" aria-hidden="true"><i></i><i></i><i></i><i></i></span>Rythme</span><span class="title">{title}</span></header>
<div class="layout">
<nav class="toc" aria-label="{contents}"><p class="label">{contents}</p><ul>{nav}</ul></nav>
<main>
{main}</main>
</div>
<footer>{footer} · Rythme {version}</footer>
</body>
</html>
"#,
        contents = escape(&label("contents")),
    )
}

/// `: ` in English, ` : ` in French.
fn sep() -> &'static str {
    crate::lang::pick(": ", " : ")
}

/// The look of the website: paper, ink and a blue accent, light or dark like the system.
const STYLE: &str = r#":root {
  --paper: #eef1f6; --staff: #fafbfd; --ink: #161b26; --ink-soft: #505a6e; --rule: #d4dae5;
  --accent: #3245c2; --accent-soft: #e1e5fa;
  --tok-kw: #8a2e9a; --tok-str: #1d7446; --tok-num: #a1520a; --tok-ctor: #1d6a8e; --tok-com: #6b7487; --tok-fn: #3245c2;
  --body: "IBM Plex Sans", "Segoe UI", system-ui, sans-serif;
  --display: "Bricolage Grotesque", "Avenir Next", "Segoe UI", system-ui, sans-serif;
  --mono: "JetBrains Mono", ui-monospace, "SFMono-Regular", Menlo, Consolas, monospace;
  color-scheme: light;
}
@media (prefers-color-scheme: dark) {
  :root {
    --paper: #0e1118; --staff: #151a24; --ink: #e4e8f0; --ink-soft: #9ba4b6; --rule: #283043;
    --accent: #8d9dff; --accent-soft: #222a4b;
    --tok-kw: #d68cf0; --tok-str: #8fd6a8; --tok-num: #f0a65e; --tok-ctor: #7cc0e8; --tok-com: #7a8396; --tok-fn: #8d9dff;
    color-scheme: dark;
  }
}
*, *::before, *::after { box-sizing: border-box; }
body { margin: 0; background: var(--paper); color: var(--ink); font: 16px/1.6 var(--body); overflow-x: hidden; }
a { color: var(--accent); text-decoration: none; }
a:hover { text-decoration: underline; }
code, pre { font-family: var(--mono); font-size: 13.5px; font-variant-ligatures: none; }
.bar { position: sticky; top: 0; z-index: 1; display: flex; gap: 16px; align-items: center; padding: 12px 16px;
  background: var(--paper); border-bottom: 1px solid var(--rule); }
.brand { display: inline-flex; gap: 10px; align-items: center; font: 700 20px/1 var(--display); }
.beats { display: inline-flex; gap: 3px; align-items: flex-end; height: 18px; }
.beats i { display: block; width: 4px; border-radius: 2px; background: var(--accent); }
.beats i:nth-child(1) { height: 18px; } .beats i:nth-child(2) { height: 8px; }
.beats i:nth-child(3) { height: 13px; } .beats i:nth-child(4) { height: 8px; }
.title { color: var(--ink-soft); font-family: var(--mono); font-size: 14px; overflow-wrap: anywhere; }
.layout { display: grid; gap: 24px; max-width: 1100px; margin: 0 auto; padding: 24px 16px; }
.toc { font-size: 14px; }
.toc ul { list-style: none; margin: 0; padding-left: 12px; }
.toc > ul { padding: 0; }
.label { margin: 0 0 6px; color: var(--ink-soft); font-size: 12px; font-weight: 600; letter-spacing: .06em; text-transform: uppercase; }
main { min-width: 0; }
.file { margin-bottom: 32px; padding: 20px 16px; background: var(--staff); border: 1px solid var(--rule); border-radius: 12px; }
h2, h3, h4 { font-family: var(--display); line-height: 1.25; }
h2 { margin: 0 0 12px; font-size: 26px; overflow-wrap: anywhere; }
h3 { margin: 28px 0 8px; padding-bottom: 4px; border-bottom: 1px solid var(--rule); font-size: 19px; }
h4 { margin: 0 0 8px; font-size: 17px; }
.entry { padding: 14px 0; border-bottom: 1px dashed var(--rule); }
.entry:last-child { border-bottom: 0; }
pre { margin: 0 0 10px; padding: 12px 14px; overflow-x: auto; background: var(--paper); border-left: 3px solid var(--accent); border-radius: 8px; }
p code, li code { padding: 1px 5px; background: var(--accent-soft); border-radius: 5px; }
.entry p, .summary p { margin: 0 0 8px; white-space: pre-line; }
.empty { color: var(--ink-soft); font-style: italic; }
.kw { color: var(--tok-kw); font-weight: 600; } .str { color: var(--tok-str); } .num { color: var(--tok-num); }
.ctor { color: var(--tok-ctor); } .com { color: var(--tok-com); font-style: italic; } .fn { color: var(--tok-fn); font-weight: 600; }
footer { padding: 16px; color: var(--ink-soft); font-size: 13px; text-align: center; }
@media (min-width: 860px) {
  .layout { grid-template-columns: 230px minmax(0, 1fr); }
  .toc { position: sticky; top: 72px; align-self: start; max-height: calc(100vh - 96px); overflow-y: auto; }
  .file { padding: 24px 28px; }
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchors_are_those_of_github() {
        let mut a = Anchors::default();
        assert_eq!(a.make("geometry.ry"), "geometryry");
        assert_eq!(a.make("Built-in functions mentioned"), "built-in-functions-mentioned");
        assert_eq!(a.make("Fonctions"), "fonctions");
        assert_eq!(a.make("fonctions"), "fonctions-1");
        assert_eq!(a.make("Fonctions intégrées citées"), "fonctions-intégrées-citées");
    }

    #[test]
    fn the_summary_is_the_comment_at_the_top_followed_by_a_blank_line() {
        assert_eq!(summary("## Shapes.\n## And areas.\n\nfn f(x) = x").as_deref(), Some("Shapes.\nAnd areas."));
        assert_eq!(summary("## Documents f.\nfn f(x) = x"), None);
        assert_eq!(summary("x = 1\n"), None);
    }

    #[test]
    fn names_between_backticks() {
        assert_eq!(leading_name("len(list)"), "len");
        assert_eq!(leading_name("area"), "area");
        assert_eq!(leading_name("a + b"), "");
        assert_eq!(code_spans("use `len` and `map(f)`").iter().map(|(c, _)| *c).collect::<Vec<_>>(), ["len", "map(f)"]);
    }

    #[test]
    fn code_is_colored_and_escaped() {
        let h = highlight("fn area(Circle(r)) if r > 0  # \"big\" <3");
        assert!(h.contains("<span class=\"kw\">fn</span> <span class=\"fn\">area</span>"), "{h}");
        assert!(h.contains("<span class=\"ctor\">Circle</span>"), "{h}");
        assert!(h.contains("&gt;"), "{h}");
        assert!(h.contains("<span class=\"com\"># &quot;big&quot; &lt;3</span>"), "{h}");
    }

    #[test]
    fn comments_become_paragraphs_and_lists() {
        let t = HashMap::from([("len".to_string(), "len".to_string())]);
        let h = link_comment("Counts with `len`.\n\n- one\n- two <b>", &t, Format::Html);
        assert_eq!(h, "<p>Counts with <a href=\"#len\"><code>len</code></a>.</p>\n<ul><li>one</li><li>two &lt;b&gt;</li></ul>\n");
        assert_eq!(link_comment("See `len(x)` and `y`.", &t, Format::Markdown), "See [`len(x)`](#len) and `y`.");
    }
}
