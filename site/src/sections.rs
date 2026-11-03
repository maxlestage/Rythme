//! The five sections that flesh the site out: *Install & tools*, *Reference*, *Packages*,
//! *Rythme at work* and *Exercises*. Their words are in `i18n/sections/`; what they list
//! comes from the interpreter itself (`rythme`, a dependency of the server), so the pages
//! cannot fall behind it: every built-in function of `BUILTINS` with its signature, its
//! documentation and its example, every code of `help::CODES` with its explanation, every
//! exercise of `rythme exercise`, each in the language of the page.

use std::io::Write;

use paquets::prelude::*;
use rythme::builtins::{BUILTINS, Builtin, Category};
use rythme::exercise::{EXERCISES, Exercise};

use crate::highlight::{code_block, code_inline, plain_block};
use crate::i18n::{Lang, Sections};
use crate::pages::{Ctx, RELEASES, REPO, button_link, command, external, layout, page_head, playground_link, section_head, snippet};
use crate::routes::Page as PageKind;

// ----- What the pages show, the same in every language ---------------------------------------

/// The ways to install Rythme, as the README gives them, in the order of
/// `Sections::inst_ways`: their commands (by hand: none, the Releases page).
pub const INSTALL_WAYS: [&[&str]; 6] = [
    &[
        "curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh",
        "curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh -s -- --dir /usr/local/bin",
    ],
    &["irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex"],
    &[],
    &["brew install maxlestage/rythme/rythme", "winget install Rythme.Rythme"],
    &["cargo install --git https://github.com/maxlestage/Rythme"],
    &["git clone https://github.com/maxlestage/Rythme", "cd Rythme && cargo install --path ."],
];

/// Keeping Rythme up to date, and the completions of the shells: in the order of
/// `Sections::inst_keep`.
pub const KEEP: [&str; 6] = [
    "rythme upgrade",
    "rythme version --verbose",
    "rythme completions bash > ~/.local/share/bash-completion/completions/rythme",
    "rythme completions zsh > \"${fpath[1]}/_rythme\"",
    "rythme completions fish > ~/.config/fish/completions/rythme.fish",
    "rythme man > ~/.local/share/man/man1/rythme.1",
];

/// The folders of the editors in the repository, in the order of
/// `Sections::inst_editors`, and the command or configuration of each (if any).
pub const EDITORS: [(&str, Option<&str>); 5] = [
    ("editors/vscode", Some("code --install-extension rythme-1.0.0.vsix")),
    ("editors/zed", None),
    ("editors/helix", Some("hx --health rythme")),
    ("editors/tree-sitter-rythme", None),
    ("editors/README.md", Some("rythme lsp")),
];

/// The configuration of Helix (`editors/README.md`).
pub const HELIX_TOML: &str = "[language-server.rythme]\ncommand = \"rythme\"\nargs = [\"lsp\"]\n\n[[language]]\nname = \"rythme\"\nscope = \"source.rythme\"\nfile-types = [\"ry\"]\ncomment-token = \"#\"\nlanguage-servers = [\"rythme\"]";

/// The debugger, in the order of `Sections::inst_debug`.
pub const DEBUG: [&str; 6] = [
    "rythme debug fib.ry",
    "break 12 · break fact",
    "run · next · step · finish · continue",
    "where · frame · locals · list",
    "print n * 2",
    "rythme dap",
];

/// The MCP server for Claude Code (the other assistants: the AI page).
pub const MCP: &str = crate::ai::CLAUDE_CODE;

/// The commands of the package index, in the order of `Sections::pkg_find`.
pub const PACKAGE_COMMANDS: [&str; 6] =
    ["rythme search vectors", "rythme info mathlib", "rythme add mathlib", "rythme add mathlib@1.1", "rythme update", "rythme install"];

/// A `rythme.toml` ready for the index (the manual, 7.7 and 7.11).
pub const TOML: &str = "name = \"hello\"\nversion = \"0.1.0\"\ndescription = \"Greetings in every language\"\nlicense = \"MIT\"\nrepository = \"https://github.com/someone/hello\"\nkeywords = [\"text\", \"greetings\"]\n\n[dependencies]\nmathlib = { version = \"^1.2.0\" }\ngeometry = { git = \"https://github.com/someone/geometry\", tag = \"v1.0.0\" }\ntools = { path = \"../tools\" }";

/// The ranges of versions, in the order of `Sections::pkg_ranges`.
pub const RANGES: [&str; 5] = ["^1.2.0", "~0.3.1", "=1.0.0", ">=1.2.0, <1.5.0", "* · 1.*"];

/// The command of each step of publishing, in the order of `Sections::pkg_publish`.
pub const PUBLISH: [&str; 5] = [
    "rythme check . && rythme test .",
    "git tag v1.2.0",
    "git push origin v1.2.0",
    "rythme publish",
    "rythme publish --verify packages/hello.toml",
];

/// Which index, in the order of `Sections::pkg_index`.
pub const INDEXES: [&str; 3] =
    ["https://github.com/maxlestage/rythme-index", "RYTHME_INDEX=https://git.example.org/our-index.git", "index = \"../shared-index\""];

/// A program of *Rythme at work*: its file, its code, what it prints.
pub struct Sample {
    pub file: &'static str,
    pub code: &'static str,
    pub output: &'static str,
}

/// The programs of *Rythme at work*, in the order of `Sections::work_samples`: they run
/// in the playground (no file, no network) and the tests check what they print.
pub const WORK_SAMPLES: [Sample; 4] = [
    Sample {
        file: "hours.ry",
        code: "data = \"name,team,hours\\nAda,core,12\\nAlan,web,7\\nGrace,core,9\\nLinus,web,4\\n\"\n\nfn hours(row) = parse_int(row.hours) |> unwrap_or(0)\n\nmatch csv_records(data, \",\")\n  Ok(rows) ->\n    rows\n      |> group_by(fn(row) = row.team)\n      |> entries\n      |> each(fn([team, people]) = print(team, sum(map(people, hours))))\n  Err(e) -> print_error(e)\nend\n",
        output: "core 21\nweb 11",
    },
    Sample {
        file: "latency.ry",
        code: "body = \"\\{\\\"service\\\": \\\"api\\\", \\\"latency_ms\\\": [120, 95, 310, 88]\\}\"\n\nfn summary(doc) = {\n  service: doc.service,\n  slow: len(filter(doc.latency_ms, fn(ms) = ms > 100)),\n  worst: max(doc.latency_ms),\n}\n\nmatch parse_json(body)\n  Ok(doc) -> print(to_json(summary(doc)))\n  Err(e) -> print_error(e)\nend\n",
        output: "{\"service\": \"api\", \"slow\": 2, \"worst\": 310}",
    },
    Sample {
        file: "journal.ry",
        code: "journal = \"2026-10-30 ERROR db timeout\\n2026-10-30 INFO started\\n2026-10-31 ERROR db timeout\\n2026-10-31 WARN slow disk\\n\"\n\nlines(journal)\n  |> filter(fn(l) = contains(l, \"ERROR\") or contains(l, \"WARN\"))\n  |> map(fn(l) = split(l, \" \") |> drop(1) |> join(\" \"))\n  |> most_common\n  |> each(fn([line, n]) = print(\"{n} × {line}\"))\n",
        output: "2 × ERROR db timeout\n1 × WARN slow disk",
    },
    Sample {
        file: "greet.ry",
        code: "fn greet(opts)\n  name = get(opts, \"name\") |> unwrap_or(\"World\")\n  if has(opts, \"loud\") then upper(\"Hello, {name}!\") else \"Hello, {name}!\"\nend\n\n# In a terminal: options(args()), with rythme greet.ry --name=Ada --loud\nmatch options([\"--name=Ada\", \"--loud\"])\n  Ok(opts) -> print(greet(opts))\n  Err(e) -> print_error(e)\nend\n",
        output: "HELLO, ADA!",
    },
];

/// A file with its tests, for the part of *Rythme at work* about CI: it opens in the
/// playground's test mode.
pub const CI_SAMPLE: Sample = Sample {
    file: "invoice.ry",
    code: "fn total(lines) = lines |> map(fn(l) = l.qty * l.price) |> sum\n\ntest \"an empty invoice\" = assert_eq(total([]), 0)\ntest \"two lines\" = assert_eq(total([{qty: 2, price: 5}, {qty: 1, price: 3}]), 13)\n",
    output: "",
};

/// The commands of a continuous integration, in the order of `Sections::work_ci`.
pub const CI_COMMANDS: [&str; 4] = ["rythme check .", "rythme fmt --check .", "rythme test .", "rythme test --sandbox --format json ."];

/// A workflow of GitHub Actions that runs them.
pub const CI_WORKFLOW: &str = "name: Rythme\non: [push, pull_request]\njobs:\n  check:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - run: curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh\n      - run: echo \"$HOME/.local/bin\" >> \"$GITHUB_PATH\"\n      - run: rythme check . && rythme fmt --check . && rythme test .";

/// `--format json` for tools, in the order of `Sections::work_json`.
pub const JSON_COMMANDS: [&str; 3] =
    ["rythme check --format json src/", "echo 'print(1 + 2)' | rythme run --format json -", "rythme test --format json"];

/// What `echo 'print(1 + 2)' | rythme run --format json -` prints.
pub const JSON_EXAMPLE: &str = "{\"version\":1,\"command\":\"run\",\"rythme\":\"1.0.0\",\"language\":\"en\",\"file\":\"-\",\"ok\":true,\"exit_code\":0,\"stdout\":\"3\\n\",\"stderr\":\"\",\"seconds\":0,\"summary\":{\"errors\":0,\"warnings\":0},\"diagnostics\":[]}";

/// The schema of the JSON output.
pub const SCHEMA: &str = "https://github.com/maxlestage/Rythme/blob/master/docs/rythme-output.schema.json";
/// The stability promise.
pub const PROMISE: &str = "https://github.com/maxlestage/Rythme/blob/master/README.md#stability-promise";
/// The benchmarks.
pub const BENCH: &str = "https://github.com/maxlestage/Rythme/tree/master/bench";
/// The manual, at the package index.
pub const MANUAL_INDEX: &str = "https://github.com/maxlestage/Rythme/blob/master/docs/reference.md#711-the-package-index";
/// The template of the index.
pub const INDEX_TEMPLATE: &str = "https://github.com/maxlestage/Rythme/tree/master/packaging/index-template";
/// The exercises, with their solutions.
pub const EXERCISES_DIR: &str = "https://github.com/maxlestage/Rythme/tree/master/docs/exercises";

/// The anchors of the groups of built-in functions, in the order of `Category::ALL`.
const CATEGORY_IDS: [&str; 12] =
    ["basics", "numbers", "text", "lists", "records", "dicts", "options", "files", "dates", "tests", "ml", "web"];

// ----- From the interpreter -------------------------------------------------------------------

/// The interpreter's language of a language of the site.
pub fn interpreter(lang: Lang) -> rythme::lang::Lang {
    rythme::lang::Lang::all().nth(lang.interpreter()).expect("a language of the interpreter")
}

/// The groups of built-in functions: (the anchor, the category, its functions), each
/// function once, in the order of `BUILTINS`.
pub fn groups() -> Vec<(&'static str, usize, Vec<&'static Builtin>)> {
    Category::ALL
        .iter()
        .enumerate()
        .map(|(n, cat)| (CATEGORY_IDS[n], n, BUILTINS.iter().filter(|b| b.cat == *cat).collect()))
        .filter(|(_, _, fns): &(_, _, Vec<_>)| !fns.is_empty())
        .collect()
}

/// The documentation of a built-in function in a language, without changing the
/// interpreter's current language (pages of every language are rendered at once).
pub fn doc(b: &Builtin, lang: Lang) -> &'static str {
    rythme::lang::pick_in(interpreter(lang), b.en, b.fr)
}

/// The anchor of a built-in function: `fn-map`.
pub fn fn_id(b: &Builtin) -> String {
    format!("fn-{}", b.name)
}

/// The explanation of a code in a language: its title (after ` — ` on the first line)
/// and the rest.
pub fn explanation(code: &str, lang: Lang) -> (String, String) {
    let text = rythme::help::explain_in(interpreter(lang), code).unwrap_or_default();
    let (first, rest) = text.split_once('\n').unwrap_or((text, ""));
    let title = first.split_once(" — ").map_or(first, |(_, t)| t).trim().to_owned();
    (title, rest.trim_matches('\n').to_owned())
}

/// An exercise in a language: its number, title, level (1 to 3), the comments of its
/// head (statement and hints) and its file name.
pub struct Exo {
    pub number: &'static str,
    pub title: String,
    pub level: usize,
    pub statement: String,
    pub file: String,
    pub text: &'static str,
}

pub fn exercise(e: &Exercise, lang: Lang) -> Exo {
    let l = interpreter(lang);
    let text = e.text_in(l);
    let mut lines = text.lines();
    let first = lines.next().unwrap_or("");
    let after = first.split_once("· ").map_or(first, |(_, t)| t);
    let (title, stars) = after.rsplit_once("  ").unwrap_or((after, ""));
    let statement: Vec<&str> =
        lines.take_while(|l| l.starts_with('#')).map(|l| l.strip_prefix("# ").or_else(|| l.strip_prefix('#')).unwrap_or(l)).collect();
    Exo {
        number: e.number(),
        title: title.trim().to_owned(),
        level: stars.matches('★').count().clamp(1, 3),
        statement: statement.join("\n").trim_matches('\n').to_owned(),
        file: format!("{}.ry", e.name_in(l)),
        text,
    }
}

/// A link that opens `code` in the playground compressed, as *Share* makes it
/// (`#code=1:` then the program in raw deflate and URL-safe base64): an exercise is a
/// few kilobytes, a third of that once compressed.
pub fn packed_link(lang: Lang, code: &str, test: bool) -> String {
    let mut deflate = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
    let packed = deflate.write_all(code.as_bytes()).and_then(|()| deflate.finish());
    let Ok(bytes) = packed else { return playground_link(lang, code, test) };
    let mut href =
        format!("{}#code=1:{}", crate::routes::Route::new(lang, PageKind::Playground).href(), rythme_site_client::share::to_base64(&bytes));
    if test {
        href.push_str("&mode=test");
    }
    href
}

// ----- Pieces -------------------------------------------------------------------------------

/// A text of the interpreter's documentation: what is between backticks is code.
fn doc_text(text: &str) -> Vec<Node> {
    text.split('`')
        .enumerate()
        .filter(|(_, part)| !part.is_empty())
        .map(|(n, part)| if n % 2 == 1 { code_inline(part).into() } else { Node::from(part.to_owned()) })
        .collect()
}

/// A piece of a text written for a terminal (an explanation of `rythme explain`, the
/// head of an exercise), whose lines are cut at about 75 columns.
#[derive(Debug, PartialEq)]
pub enum Block {
    /// Lines of prose, joined again: they wrap at the width of the screen.
    Prose(String),
    /// An item of a list (`- …` or `1. …`), its lines joined.
    Item(bool, String),
    /// Code, or text lined up in columns (`Wrong:   …`), kept as it is.
    Code(String),
}

/// Whether a line belongs to code: indented, or lined up with runs of spaces.
fn is_code(line: &str) -> bool {
    line.starts_with(' ') || line.trim().contains("   ")
}

/// `- item` or `12. item`: whether it is numbered, and the text after the marker.
fn list_item(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return Some((false, rest));
    }
    let digits = trimmed.chars().take_while(char::is_ascii_digit).count();
    (digits > 0).then(|| trimmed[digits..].strip_prefix(". ").map(|rest| (true, rest))).flatten()
}

/// Two lines of one sentence, joined: with a space, but between two characters of
/// Chinese or Japanese, which have none.
fn join(a: &mut String, b: &str) {
    let wide = |c: Option<char>| c.is_some_and(|c| c as u32 >= 0x2E80);
    if !a.is_empty() && !(wide(a.chars().last()) && wide(b.chars().next())) {
        a.push(' ');
    }
    a.push_str(b);
}

/// The blocks of a text for a terminal: prose and list items joined again, code kept.
pub fn blocks(text: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    for paragraph in text.split("\n\n") {
        // The column where the text of the last item starts.
        let mut item_indent: Option<usize> = None;
        let mut fresh = true;
        for line in paragraph.lines().filter(|l| !l.trim().is_empty()) {
            let indent = line.len() - line.trim_start().len();
            if let Some((numbered, rest)) = list_item(line) {
                out.push(Block::Item(numbered, rest.trim().to_owned()));
                // The column of its text: the lines that go on with it start there.
                item_indent = Some(line.len() - rest.len());
            } else if let (Some(Block::Item(_, text)), Some(at)) = (out.last_mut(), item_indent)
                && indent == at
                && !line.trim().contains("   ")
            {
                join(text, line.trim());
            } else if is_code(line) {
                item_indent = None;
                match out.last_mut() {
                    Some(Block::Code(code)) if !fresh => {
                        code.push('\n');
                        code.push_str(line.trim_end());
                    }
                    _ => out.push(Block::Code(line.trim_end().to_owned())),
                }
            } else {
                item_indent = None;
                match out.last_mut() {
                    Some(Block::Prose(prose)) if !fresh => join(prose, line.trim()),
                    _ => out.push(Block::Prose(line.trim().to_owned())),
                }
            }
            fresh = false;
        }
    }
    // Code keeps its indentation, less what all its lines share.
    for block in &mut out {
        if let Block::Code(code) = block {
            let common = code.lines().map(|l| l.len() - l.trim_start().len()).min().unwrap_or(0);
            *code = code.lines().map(|l| &l[common..]).collect::<Vec<_>>().join("\n");
        }
    }
    out
}

/// The blocks as HTML: paragraphs (code between backticks), lists, code.
fn rich(text: &str) -> Vec<Element> {
    let mut out: Vec<Element> = Vec::new();
    let mut list: Option<(bool, Vec<Element>)> = None;
    let flush = |list: &mut Option<(bool, Vec<Element>)>, out: &mut Vec<Element>| {
        if let Some((numbered, items)) = list.take() {
            out.push(if numbered { ol() } else { ul() }.class("rich__list").children(items));
        }
    };
    for block in blocks(text) {
        match block {
            Block::Item(numbered, text) => {
                if list.as_ref().is_some_and(|(n, _)| *n != numbered) {
                    flush(&mut list, &mut out);
                }
                list.get_or_insert_with(|| (numbered, Vec::new())).1.push(li().children(doc_text(&text)));
            }
            Block::Prose(text) => {
                flush(&mut list, &mut out);
                out.push(p().children(doc_text(&text)));
            }
            Block::Code(code) => {
                flush(&mut list, &mut out);
                out.push(plain_block(&code));
            }
        }
    }
    flush(&mut list, &mut out);
    out
}

/// Rows of commands and what they do.
fn commands(rows: impl IntoIterator<Item = (String, String)>) -> Element {
    dl().class("commands").children(
        rows.into_iter().map(|(cmd, what)| {
            div().class("commands__row").child(dt().child(code().attr("translate", "no").text(cmd))).child(dd().text(what))
        }),
    )
}

/// A section of a page: its title, and its lead when it has one.
fn part(id: &str, title: &str, lead: &str) -> Element {
    let head = if lead.is_empty() {
        header().class("section__head").child(h2().id(format!("{id}-title")).text(title.to_owned()))
    } else {
        section_head(id, title, lead)
    };
    section().class("section").id(id.to_owned()).attr("aria-labelledby", format!("{id}-title")).child(head)
}

fn bullets(items: &[&str]) -> Element {
    ul().class("checks").children(items.iter().map(|i| li().text(i.to_string())))
}

pub fn repo_link(path: &str) -> String {
    let kind = if path.ends_with(".md") { "blob" } else { "tree" };
    format!("{REPO}/{kind}/master/{path}")
}

fn s(ctx: &Ctx) -> &'static Sections {
    ctx.lang().sections()
}

fn title(t: &str) -> String {
    format!("{t} — Rythme")
}

// ----- Install & tools ----------------------------------------------------------------------

pub fn install(ctx: &Ctx) -> Page {
    let (t, x) = (ctx.t(), s(ctx));
    let ways = x.inst_ways.iter().zip(INSTALL_WAYS).enumerate().map(|(n, (item, cmds))| {
        let body = div().class("way__body").child(p().text(item.text)).children(cmds.iter().map(|c| command(t, c)));
        let body = if n == 2 { body.child(external(a().class("btn btn--ghost").href(RELEASES).text("Releases →"))) } else { body };
        li().class("step way").child(span().class("step__num").text(format!("{}", n + 1))).child(h3().text(item.title)).child(body)
    });
    let get = part("get", x.inst_get_title, x.inst_get_lead).child(ol().class("steps steps--ways").children(ways));
    let keep = part("keep", x.inst_keep_title, x.inst_keep_lead)
        .child(commands(KEEP.iter().zip(x.inst_keep).map(|(c, w)| (c.to_string(), w.to_owned()))));
    let editors = part("editors", x.inst_editors_title, x.inst_editors_lead).child(ul().class("cards cards--editors").children(
        x.inst_editors.iter().zip(EDITORS).enumerate().map(|(n, (item, (path, cmd)))| {
            let card =
                li().class("card").child(h3().child(external(a().href(repo_link(path)).text(item.title)))).child(p().text(item.text));
            let card = if n == 2 { card.child(snippet(t, "languages.toml", None, HELIX_TOML)) } else { card };
            match cmd {
                Some(c) => card.child(command(t, c)),
                None => card,
            }
        }),
    ));
    let lsp = part("lsp", x.inst_lsp_title, x.inst_lsp_lead).child(command(t, "rythme lsp")).child(bullets(&x.inst_lsp));
    let debug = part("debug", x.inst_debug_title, x.inst_debug_lead)
        .child(commands(DEBUG.iter().zip(x.inst_debug).map(|(c, w)| (c.to_string(), w.to_owned()))));
    let ai = part("assistants", x.inst_ai_title, x.inst_ai_lead).child(command(t, MCP)).child(button_link(
        ctx.href(PageKind::Ai),
        "btn--ghost",
        &format!("{} →", t.ai_title),
    ));
    let content = main()
        .class("page more")
        .child(page_head(x.inst_title, x.inst_lead))
        .child(get)
        .child(keep)
        .child(editors)
        .child(lsp)
        .child(debug)
        .child(ai);
    layout(ctx, &title(x.inst_title), x.inst_lead, content)
}

// ----- Reference ---------------------------------------------------------------------------

pub fn reference(ctx: &Ctx) -> Page {
    let (lang, x) = (ctx.lang(), s(ctx));
    let groups = groups();
    let total = BUILTINS.len();
    let jump = nav().class("chips").attr("aria-label", x.ref_jump).children(
        groups
            .iter()
            .map(|(id, n, _)| a().class("chip").href(format!("#{id}")).text(x.ref_categories[*n]))
            .chain([a().class("chip").href("#codes").text(x.ref_codes_title)]),
    );
    // The filter, shown by the browser side: without it, the whole list is there.
    let filter = div()
        .class("filter")
        .id("ref-filter")
        .attr("hidden", "")
        .attr("data-count", x.ref_filter_count)
        .child(label().class("filter__label").attr("for", "ref-search").text(x.ref_filter_label))
        .child(
            input()
                .id("ref-search")
                .attr("type", "search")
                .attr("spellcheck", "false")
                .attr("autocomplete", "off")
                .attr("autocapitalize", "off")
                .attr("placeholder", x.ref_filter_placeholder)
                .attr("aria-describedby", "ref-count"),
        )
        .child(
            p().class("filter__count")
                .id("ref-count")
                .attr("aria-live", "polite")
                .text(x.ref_filter_count.replace("{n}", &total.to_string()).replace("{total}", &total.to_string())),
        );
    let none = p().class("note note--box").id("ref-none").attr("hidden", "").text(x.ref_filter_none);
    let lists = groups.iter().map(|(id, n, fns)| {
        section()
            .class("ref-group")
            .id(id.to_string())
            .attr("aria-labelledby", format!("{id}-title"))
            .child(h3().class("ref-group__title").id(format!("{id}-title")).text(x.ref_categories[*n]))
            .child(ul().class("fns").children(fns.iter().map(|b| {
                li().class("fn")
                    .id(fn_id(b))
                    .attr("data-name", b.name)
                    .child(
                        h4().class("fn__name").child(a().href(format!("#{}", fn_id(b))).child(code().attr("translate", "no").text(b.name))),
                    )
                    .child(p().class("fn__sig").child(code_inline(b.sig)))
                    .child(p().class("fn__doc").children(doc_text(doc(b, lang))))
                    .child(div().class("fn__example").attr("aria-label", x.ref_example).child(code_block(b.example)))
            })))
    });
    let fns = part("functions", x.ref_fns_title, &x.ref_fns_lead.replace("{n}", &total.to_string()))
        .child(filter)
        .child(jump)
        .child(none)
        .child(div().class("ref-groups").id("ref-groups").children(lists));
    let codes = rythme::help::CODES;
    let code_list = |warnings: bool| {
        div().class("codes").children(codes.iter().filter(|c| c.starts_with('W') == warnings).map(|id| {
            let id: &str = id;
            let (name, text) = explanation(id, lang);
            article()
                .class("code-entry")
                .id(id.to_owned())
                .attr("aria-labelledby", format!("{id}-title"))
                .child(
                    h4().class("code-entry__title")
                        .id(format!("{id}-title"))
                        .child(a().href(format!("#{id}")).child(code_el(id)))
                        .text(format!(" {name}")),
                )
                .child(div().class("rich").children(rich(&text)))
                .child(p().class("code-entry__cmd").child(code_el(&format!("rythme explain {id}"))))
        }))
    };
    let codes_section = part("codes", x.ref_codes_title, &x.ref_codes_lead.replace("{n}", &codes.len().to_string()))
        .child(h3().class("subhead").text(x.ref_errors))
        .child(code_list(false))
        .child(h3().class("subhead").text(x.ref_warnings))
        .child(code_list(true));
    let content = main()
        .class("page more reference")
        .child(page_head(x.ref_title, x.ref_lead))
        .child(p().class("note").text(x.ref_generated.replace("{version}", crate::ai::version())))
        .child(fns)
        .child(codes_section);
    layout(ctx, &title(x.ref_title), x.ref_lead, content)
}

fn code_el(text: &str) -> Element {
    code().attr("translate", "no").text(text.to_owned())
}

// ----- Packages ----------------------------------------------------------------------------

pub fn packages(ctx: &Ctx) -> Page {
    let (t, x) = (ctx.t(), s(ctx));
    let find = part("find", x.pkg_find_title, x.pkg_find_lead)
        .child(commands(PACKAGE_COMMANDS.iter().zip(x.pkg_find).map(|(c, w)| (c.to_string(), w.to_owned()))));
    let toml = part("manifest", x.pkg_toml_title, x.pkg_toml_lead)
        .child(snippet(t, "rythme.toml", None, TOML))
        .child(commands(RANGES.iter().zip(x.pkg_ranges).map(|(c, w)| (c.to_string(), w.to_owned()))))
        .child(p().class("note note--box").text(x.pkg_lock));
    let steps = x.pkg_publish.iter().zip(PUBLISH).enumerate().map(|(n, (item, cmd))| {
        li().class("step")
            .child(span().class("step__num").text(format!("{}", n + 1)))
            .child(h3().text(item.title))
            .child(div().child(p().text(item.text)).child(command(t, cmd)))
    });
    let publish = part("publish", x.pkg_publish_title, x.pkg_publish_lead).child(ol().class("steps steps--ways").children(steps)).child(
        div()
            .class("more__links")
            .child(external(button_link(MANUAL_INDEX, "btn--ghost", "docs/reference.md §7.11 →")))
            .child(external(button_link(INDEX_TEMPLATE, "btn--ghost", "packaging/index-template →"))),
    );
    let index = part("index", x.pkg_index_title, x.pkg_index_lead)
        .child(commands(INDEXES.iter().zip(x.pkg_index).map(|(c, w)| (c.to_string(), w.to_owned()))));
    let safety = part("safety", x.pkg_safety_title, "").child(bullets(&x.pkg_safety));
    let content = main()
        .class("page more")
        .child(page_head(x.pkg_title, x.pkg_lead))
        .child(find)
        .child(toml)
        .child(publish)
        .child(index)
        .child(safety);
    layout(ctx, &title(x.pkg_title), x.pkg_lead, content)
}

// ----- Rythme at work ------------------------------------------------------------------------

fn sample(ctx: &Ctx, item: &crate::i18n::Item, sample: &Sample) -> Element {
    let t = ctx.t();
    let name = sample.file.trim_end_matches(".ry");
    article()
        .class("example")
        .id(name.to_owned())
        .attr("aria-labelledby", format!("{name}-title"))
        .child(
            header()
                .class("example__head")
                .child(div().child(h3().id(format!("{name}-title")).text(item.title)).child(p().text(item.text)))
                .child(button_link(playground_link(ctx.lang(), sample.code, false), "btn--ghost btn--sm", &format!("▶ {}", t.ex_open))),
        )
        .child(
            div().class("example__body").child(code_block(sample.code)).child(
                div()
                    .class("example__out")
                    .child(span().class("score__label").text(t.ex_output))
                    .child(pre().class("output").child(code().text(sample.output.to_owned()))),
            ),
        )
}

pub fn work(ctx: &Ctx) -> Page {
    let (t, x) = (ctx.t(), s(ctx));
    let data = part("data", x.work_data_title, x.work_data_lead)
        .child(div().class("examples").children(x.work_samples.iter().zip(&WORK_SAMPLES).map(|(item, s)| sample(ctx, item, s))));
    let ci = part("ci", x.work_ci_title, x.work_ci_lead)
        .child(commands(CI_COMMANDS.iter().zip(x.work_ci).map(|(c, w)| (c.to_string(), w.to_owned()))))
        .child(
            div().class("tour__code").child(code_block(CI_SAMPLE.code)).child(
                div().class("tour__links").child(
                    a().class("tour__try")
                        .href(playground_link(ctx.lang(), CI_SAMPLE.code, true))
                        .text(format!("▶ {} · {}", t.pg_test, CI_SAMPLE.file)),
                ),
            ),
        )
        .child(snippet(t, ".github/workflows/rythme.yml", Some(x.work_ci_file), CI_WORKFLOW));
    let json = part("json", x.work_json_title, x.work_json_lead)
        .child(commands(JSON_COMMANDS.iter().zip(x.work_json).map(|(c, w)| (c.to_string(), w.to_owned()))))
        .child(div().class("terminal").child(plain_block(JSON_EXAMPLE)))
        .child(external(button_link(SCHEMA, "btn--ghost", "rythme-output.schema.json →")));
    let stable = part("stability", x.work_stable_title, x.work_stable_lead).child(bullets(&x.work_stable)).child(external(button_link(
        PROMISE,
        "btn--ghost",
        "README · Stability promise →",
    )));
    let speed = part("speed", x.work_speed_title, x.work_speed_lead)
        .child(bullets(&x.work_speed))
        .child(command(t, "bench/run.sh"))
        .child(external(button_link(BENCH, "btn--ghost", "bench/ →")));
    let content =
        main().class("page more").child(page_head(x.work_title, x.work_lead)).child(data).child(ci).child(json).child(stable).child(speed);
    layout(ctx, &title(x.work_title), x.work_lead, content)
}

// ----- Exercises ----------------------------------------------------------------------------

/// The commands of `rythme exercise`, in the order of `Sections::exo_how`: the file to
/// check is named in the language of the page.
pub fn exercise_commands(lang: Lang) -> [String; 3] {
    let seventh = exercise(&EXERCISES[6], lang);
    ["rythme exercise list".into(), "rythme exercise start 7".into(), format!("rythme exercise check {}", seventh.file)]
}

pub fn exercises(ctx: &Ctx) -> Page {
    let (t, x, lang) = (ctx.t(), s(ctx), ctx.lang());
    let all: Vec<Exo> = EXERCISES.iter().map(|e| exercise(e, lang)).collect();
    let how = part("how", x.exo_how_title, "")
        .child(commands(exercise_commands(lang).into_iter().zip(x.exo_how).map(|(c, w)| (c, w.to_owned()))));
    let levels = (1..=3).map(|level| {
        let item = &x.exo_levels[level - 1];
        let id = format!("level-{level}");
        let list = all.iter().filter(|e| e.level == level).map(|e| {
            li().class("card exo")
                .id(format!("exercise-{}", e.number))
                .child(span().class("card__num").text(format!("{} · {}", e.number, stars(e.level))))
                .child(h4().class("exo__title").text(e.title.clone()))
                .child(
                    details()
                        .class("exo__details")
                        .child(summary().text(x.exo_statement))
                        .child(div().class("rich").children(rich(&e.statement))),
                )
                .child(
                    div()
                        .class("exo__links")
                        .child(a().class("btn btn--ghost btn--sm").href(packed_link(lang, e.text, true)).text(format!("▶ {}", x.exo_try)))
                        .child(code().class("exo__cmd").attr("translate", "no").text(format!("rythme exercise start {}", e.number))),
                )
        });
        section()
            .class("section")
            .id(id.clone())
            .attr("aria-labelledby", format!("{id}-title"))
            .child(section_head(&id, &format!("{} {}", stars(level), item.title), item.text))
            .child(ul().class("cards cards--exos").children(list))
    });
    let content = main()
        .class("page more")
        .child(page_head(x.exo_title, x.exo_lead))
        .child(how)
        .children(levels)
        .child(p().class("note note--box").text(x.exo_note))
        .child(
            div().class("more__links").child(external(button_link(EXERCISES_DIR, "btn--ghost", "docs/exercises/ →"))).child(button_link(
                ctx.href(PageKind::Teach),
                "btn--ghost",
                &format!("{} →", t.nav_teach),
            )),
        );
    layout(ctx, &title(x.exo_title), x.exo_lead, content)
}

/// ★☆☆, ★★☆ or ★★★.
pub fn stars(level: usize) -> String {
    (1..=3).map(|n| if n <= level { '★' } else { '☆' }).collect()
}

/// The texts of the menu for the five sections.
pub fn nav_pages(x: &Sections) -> [(PageKind, &'static str); 5] {
    [
        (PageKind::Install, x.nav_install),
        (PageKind::Reference, x.nav_reference),
        (PageKind::Packages, x.nav_packages),
        (PageKind::Work, x.nav_work),
        (PageKind::Exercises, x.nav_exercises),
    ]
}

/// The title and the lead of a section's page (for the structured data and llms.txt).
pub fn head(x: &Sections, page: PageKind) -> Option<(&'static str, &'static str)> {
    Some(match page {
        PageKind::Install => (x.inst_title, x.inst_lead),
        PageKind::Reference => (x.ref_title, x.ref_lead),
        PageKind::Packages => (x.pkg_title, x.pkg_lead),
        PageKind::Work => (x.work_title, x.work_lead),
        PageKind::Exercises => (x.exo_title, x.exo_lead),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_function_is_in_a_group() {
        let listed: usize = groups().iter().map(|(_, _, f)| f.len()).sum();
        assert_eq!(listed, BUILTINS.len());
    }

    #[test]
    fn exercises_have_a_title_and_a_level() {
        for lang in Lang::ALL {
            for e in EXERCISES {
                let x = exercise(e, lang);
                assert!(!x.title.is_empty() && !x.title.contains('★') && !x.statement.is_empty(), "{lang:?} {}", x.number);
            }
        }
        assert_eq!(exercise(&EXERCISES[6], Lang::En).level, 2);
        assert_eq!(exercise(&EXERCISES[6], Lang::Fr).file, "07-recursivite.ry");
    }

    #[test]
    fn texts_for_a_terminal_wrap_again() {
        let text = "One line\nand its end.\n\nWrong:   x\nRight:   y\n\nCauses:\n  - a missing `)`,\n    or `]`\n  - another\n\n1. first\n   still first\n2. second\n  fn f(x) = x\n    + 1\n\n第一行\n第二行";
        assert_eq!(
            blocks(text),
            vec![
                Block::Prose("One line and its end.".into()),
                Block::Code("Wrong:   x\nRight:   y".into()),
                Block::Prose("Causes:".into()),
                Block::Item(false, "a missing `)`, or `]`".into()),
                Block::Item(false, "another".into()),
                Block::Item(true, "first still first".into()),
                Block::Item(true, "second".into()),
                Block::Code("fn f(x) = x\n  + 1".into()),
                Block::Prose("第一行第二行".into()),
            ]
        );
    }

    #[test]
    fn packed_links_read_back() {
        let link = packed_link(Lang::En, "print(1)\n", true);
        let data = link.split("#code=1:").nth(1).unwrap().trim_end_matches("&mode=test");
        let bytes = rythme_site_client::share::from_base64(data).unwrap();
        let code = rythme_site_client::share::inflate(&bytes, 1000).unwrap();
        assert_eq!(code, b"print(1)\n");
    }
}
