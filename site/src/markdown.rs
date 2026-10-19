//! The pages of the site as Markdown, for AI assistants and the tools that read the web
//! for them: `/{lang}/index.md`, `/{lang}/learn/index.md`, `/{lang}/examples/index.md`,
//! `/{lang}/teach/index.md` and `/{lang}/ai/index.md`. Written from the same texts, code
//! and examples as the HTML pages; the code is fenced as ```` ```rythme ````, what it prints
//! as ```` ```text ````, and every link is absolute.

use crate::ai;
use crate::examples::EXAMPLES;
use crate::i18n::Texts;
use crate::pages::{Ctx, TOUR_STEPS, playground_link, steps_link};
use crate::routes::{Page, Route, base, link};

/// The Markdown of a page; `None` for the pages without one (the playground, the
/// offline page).
pub fn render(ctx: &Ctx) -> Option<String> {
    let body = match ctx.route.page {
        Page::Home => home(ctx),
        Page::Learn => learn(ctx),
        Page::Examples => examples(ctx),
        Page::Teach => teach(ctx),
        Page::Ai => ai_page(ctx),
        _ => return None,
    };
    let canonical = ctx.url(ctx.route);
    let origin = &ctx.origin;
    Some(format!(
        "{}\n\n---\n\nRythme {} · <{canonical}> · <{origin}{}> · <{origin}{}>\n",
        body.trim_end(),
        ai::version(),
        ai::LLMS_PATH,
        ai::LLMS_FULL_PATH
    ))
}

// ----- Pieces ---------------------------------------------------------------------------

/// A fenced block: three backticks, or more when the text has some.
pub fn fence(lang: &str, text: &str) -> String {
    let mut longest = 0;
    let mut run = 0;
    for c in text.chars() {
        run = if c == '`' { run + 1 } else { 0 };
        longest = longest.max(run);
    }
    let ticks = "`".repeat((longest + 1).max(3));
    format!("{ticks}{lang}\n{}\n{ticks}\n\n", text.trim_end_matches('\n'))
}

/// A text of the site, its characters that Markdown would read as markup escaped.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '\\' | '*' | '<' | '>' | '[' | ']' | '`' | '|') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Inline code, with enough backticks around it.
fn code_span(text: &str) -> String {
    let ticks = if text.contains('`') { "``" } else { "`" };
    let pad = if text.starts_with('`') || text.ends_with('`') { " " } else { "" };
    format!("{ticks}{pad}{text}{pad}{ticks}")
}

/// A cell of a table: `|` escaped, even in code.
fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

/// A link with an absolute address (in `<…>`: any character but `<`, `>` and a new line).
fn md_link(text: &str, url: &str) -> String {
    format!("[{}](<{url}>)", escape(text))
}

fn title(out: &mut String, title: &str, lead: &str) {
    out.push_str(&format!("# {}\n\n> {}\n\n", escape(title), escape(lead)));
}

/// `href` (behind the base path) as an absolute address.
fn absolute(ctx: &Ctx, href: &str) -> String {
    let origin = ctx.origin.strip_suffix(base()).unwrap_or(&ctx.origin);
    format!("{origin}{href}")
}

fn page_url(ctx: &Ctx, page: Page) -> String {
    ctx.url(Route::new(ctx.route.lang, page))
}

// ----- Pages ------------------------------------------------------------------------------

fn home(ctx: &Ctx) -> String {
    let t: &Texts = ctx.route.lang.texts();
    let lang = ctx.route.lang;
    let mut out = String::new();
    title(&mut out, t.hero_title, t.hero_lead);
    out.push_str(&fence("rythme", t.hero_code));
    out.push_str(&format!("**{}**\n\n", escape(t.hero_output)));
    out.push_str(&fence("text", "20"));
    out.push_str(&format!("{}\n\n", md_link(&format!("▶ {}", t.pg_run), &absolute(ctx, &playground_link(lang, t.hero_code, false)))));

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.why_title), escape(t.why_lead)));
    for item in &t.why {
        out.push_str(&format!("- **{}**: {}\n", escape(item.title), escape(item.text)));
    }
    out.push('\n');

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.errors_title), escape(t.errors_lead)));
    out.push_str(&fence("rythme", t.errors_code));
    out.push_str(&fence("text", t.errors_message));
    for point in &t.errors_points {
        out.push_str(&format!("- {}\n", escape(point)));
    }
    out.push('\n');

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.habits_title), escape(t.habits_lead)));
    out.push_str(&format!(
        "| {} | {} | {} |\n|---|---|---|\n",
        cell(&escape(t.habits_head[0])),
        cell(&escape(t.habits_head[1])),
        cell(&escape(t.habits_head[2]))
    ));
    for h in &t.habits {
        out.push_str(&format!("| {} | {} | {} |\n", cell(&code_span(h.other)), cell(&code_span(h.rythme)), cell(&escape(h.note))));
    }
    out.push('\n');

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.install_title), escape(t.install_lead)));
    for (n, step) in t.install_steps.iter().enumerate() {
        out.push_str(&format!("### {}. {}\n\n", n + 1, escape(step.title)));
        if n == 0 {
            out.push_str(&format!("{}\n\n{}\n\n", escape(step.text), md_link("Releases", crate::pages::RELEASES)));
        } else {
            out.push_str(&fence("sh", step.text));
        }
    }
    out.push_str(&format!("### {}\n\n", escape(t.commands_title)));
    for (cmd, what) in &t.commands {
        out.push_str(&format!("- {}: {}\n", code_span(cmd), escape(what)));
    }
    out.push_str(&format!("\n{}\n\n{}\n\n", escape(t.commands_lang), escape(t.install_app)));

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.cta_title), escape(t.cta_text)));
    out.push_str(&format!("- {}\n", md_link(t.cta_try, &page_url(ctx, Page::Playground))));
    for (page, name) in [(Page::Learn, t.nav_learn), (Page::Examples, t.nav_examples), (Page::Teach, t.nav_teach), (Page::Ai, t.ai_title)] {
        out.push_str(&format!("- {}\n", md_link(name, &page_url(ctx, page))));
    }
    out
}

fn learn(ctx: &Ctx) -> String {
    let t = ctx.route.lang.texts();
    let lang = ctx.route.lang;
    let mut out = String::new();
    title(&mut out, t.learn_title, t.learn_lead);
    for (n, step) in t.tour.iter().enumerate() {
        out.push_str(&format!("## {}. {}\n\n{}\n\n", n + 1, escape(step.title), escape(step.text)));
        out.push_str(&fence("rythme", step.code));
        let test = step.code.contains("test \"");
        out.push_str(&md_link(&format!("▶ {}", t.learn_try), &absolute(ctx, &playground_link(lang, step.code, test))));
        if n == TOUR_STEPS.0 {
            let (_, expr) = TOUR_STEPS;
            out.push_str(&format!(
                " · {}",
                md_link(&format!("{}: {expr}", t.learn_steps), &absolute(ctx, &steps_link(lang, step.code, expr)))
            ));
        }
        out.push_str("\n\n");
    }
    out.push_str(&format!("{}\n\n## {}\n\n", escape(t.learn_names), escape(t.learn_next_title)));
    for (n, item) in t.learn_next.iter().enumerate() {
        let name = if n == 2 { md_link(item.title, &page_url(ctx, Page::Examples)) } else { format!("**{}**", code_span(item.title)) };
        out.push_str(&format!("- {name}: {}\n", escape(item.text)));
    }
    out
}

fn examples(ctx: &Ctx) -> String {
    let t = ctx.route.lang.texts();
    let lang = ctx.route.lang;
    let mut out = String::new();
    title(&mut out, t.ex_title, t.ex_lead);
    for e in EXAMPLES {
        out.push_str(&format!("## {}.ry\n\n{}\n\n", e.name, escape(e.about(lang))));
        out.push_str(&fence("rythme", e.code));
        out.push_str(&format!("**{}**\n\n", escape(t.ex_output)));
        out.push_str(&fence("text", e.output.trim_end()));
        let open = absolute(ctx, &format!("{}#example={}", Route::new(lang, Page::Playground).href(), e.name));
        out.push_str(&format!("{}\n\n", md_link(&format!("▶ {}", t.ex_open), &open)));
    }
    out
}

/// The guide for teachers: its own Markdown (`docs/teaching.md`, `docs/enseigner.md`),
/// with its links leading to the site or to GitHub, and the note of a language without
/// a guide of its own.
fn teach(ctx: &Ctx) -> String {
    let lang = ctx.route.lang;
    let canonical = ctx.url(ctx.route);
    let mut out = String::new();
    let mut fenced: Option<String> = None;
    let mut titled = false;
    for line in crate::teach::source(lang).lines() {
        let trimmed = line.trim_start();
        match &fenced {
            Some(ticks) => {
                if trimmed.starts_with(ticks.as_str()) && trimmed.trim_start_matches('`').trim().is_empty() {
                    fenced = None;
                }
                out.push_str(line);
            }
            None if trimmed.starts_with("```") => {
                fenced = Some(trimmed.chars().take_while(|c| *c == '`').collect());
                out.push_str(line);
            }
            None => out.push_str(&rewrite_links(line, |dest| {
                if let Some(anchor) = dest.strip_prefix('#') {
                    return format!("{canonical}#{anchor}");
                }
                let (href, external) = crate::teach::rewrite_link(dest, lang);
                if external { href } else { absolute(ctx, &href) }
            })),
        }
        out.push('\n');
        if !titled && line.starts_with("# ") {
            titled = true;
            if let Some(note) = lang.texts().teach_note {
                out.push_str(&format!("\n> {}\n", escape(note)));
            }
        }
    }
    out
}

/// The destinations of the links of a line (`[text](dest)`) through `to`.
fn rewrite_links(line: &str, to: impl Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(at) = rest.find("](") {
        let after = &rest[at + 2..];
        match after.find(')').filter(|end| !after[..*end].contains([' ', '<', '>'])) {
            Some(end) => {
                out.push_str(&rest[..at + 2]);
                out.push_str(&to(&after[..end]));
                out.push(')');
                rest = &after[end + 1..];
            }
            None => {
                out.push_str(&rest[..at + 2]);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn ai_page(ctx: &Ctx) -> String {
    let t = ctx.route.lang.texts();
    let origin = &ctx.origin;
    let mut out = String::new();
    title(&mut out, t.ai_title, t.ai_lead);

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.ai_give_title), escape(t.ai_give_lead)));
    let urls = [
        format!("{origin}{}", ai::LLMS_FULL_PATH),
        format!("{origin}{}", ai::LLMS_PATH),
        format!("{origin}{}", Route::new(ctx.route.lang, Page::Learn).markdown_path()),
    ];
    for (item, url) in t.ai_give.iter().zip(&urls) {
        out.push_str(&format!("- {}: {}\n", md_link(item.title, url), escape(item.text)));
    }
    out.push_str(&format!("\n**{}**\n\n", escape(t.ai_prompt_label)));
    out.push_str(&fence("text", &ai::prompt(ctx.route.lang, origin)));

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.ai_mcp_title), escape(t.ai_mcp_lead)));
    for ((name, kind, snippet), place) in ai::MCP.iter().zip(t.ai_mcp_where) {
        out.push_str(&format!("### {}\n\n{}\n\n", escape(name), escape(place)));
        out.push_str(&fence(kind, snippet));
    }
    out.push_str(&format!("{}\n\n", escape(t.ai_mcp_path)));

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.ai_loop_title), escape(t.ai_loop_lead)));
    for (cmd, what) in ai::LOOP.iter().zip(t.ai_loop) {
        out.push_str(&format!("- {}: {}\n", code_span(cmd), escape(what)));
    }
    out.push('\n');
    out.push_str(&fence("text", ai::FIX_EXAMPLE));

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.ai_agents_title), escape(t.ai_agents_lead)));
    out.push_str(&fence("sh", ai::NEW_PROJECT));

    out.push_str(&format!("## {}\n\n{}\n\n", escape(t.ai_play_title), escape(t.ai_play_lead)));
    out.push_str(&format!("{}\n", md_link(t.nav_playground, &page_url(ctx, Page::Playground))));
    out
}

/// The `<link>` of a page to its Markdown version (behind the base path).
pub fn href(route: Route) -> String {
    link(&route.markdown_path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fences_are_longer_than_the_code() {
        assert_eq!(fence("rythme", "x = 1\n"), "```rythme\nx = 1\n```\n\n");
        assert_eq!(fence("text", "a ```` b"), "`````text\na ```` b\n`````\n\n");
    }

    #[test]
    fn links_are_rewritten_outside_code() {
        let line = "See [the exercises](exercises/README.md) and [x](#a), not [y](a b).";
        assert_eq!(rewrite_links(line, |d| format!("<{d}>")), "See [the exercises](<exercises/README.md>) and [x](<#a>), not [y](a b).");
    }

    #[test]
    fn markup_is_escaped() {
        assert_eq!(escape("a * b <c> [d] `e` | f"), "a \\* b \\<c\\> \\[d\\] \\`e\\` \\| f");
        assert_eq!(code_span("a | b"), "`a | b`");
        assert_eq!(cell(&code_span("a | b")), "`a \\| b`");
    }
}
