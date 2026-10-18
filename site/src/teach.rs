//! The guide for teachers: `docs/teaching.md` and `docs/enseigner.md`, built into the
//! site and rendered to HTML, with the links between files turned into links of the
//! site.

use paquets::prelude::Node;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};

use crate::highlight::{code_block, plain_block};

use crate::i18n::Lang;
use crate::routes::{Page, Route};

const REPO: &str = "https://github.com/maxlestage/Rythme";

pub fn source(lang: Lang) -> &'static str {
    lang.pick([include_str!("../../docs/enseigner.md"), include_str!("../../docs/teaching.md")])
}

pub struct Rendered {
    /// The text of the `# Title`.
    pub title: String,
    /// The `## Sections`: (anchor, text).
    pub sections: Vec<(String, String)>,
    pub html: String,
}

/// `## Pourquoi il convient` → `pourquoi-il-convient`.
pub fn slug(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars().flat_map(char::to_lowercase) {
        let c = match c {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            c => c,
        };
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_owned()
}

/// Where a link of the guide leads on the site.
pub fn rewrite_link(dest: &str, lang: Lang) -> (String, bool) {
    if dest.starts_with("http://") || dest.starts_with("https://") || dest.starts_with('#') {
        return (dest.to_owned(), dest.starts_with("http"));
    }
    let file = dest.trim_start_matches("./").trim_start_matches("../");
    if let Some(name) =
        file.strip_prefix("examples/").and_then(|f| f.strip_suffix(".ry")).filter(|name| crate::examples::find(name).is_some())
    {
        return (format!("{}#{name}", Route::new(lang, Page::Examples).href()), false);
    }
    match file {
        "teaching.md" => (Route::new(Lang::En, Page::Teach).href(), false),
        "enseigner.md" => (Route::new(Lang::Fr, Page::Teach).href(), false),
        // A file of the repository opens on GitHub (`tree/` for a folder).
        _ if file.contains('.') => (format!("{REPO}/blob/master/{file}"), true),
        _ => (format!("{REPO}/tree/master/{file}"), true),
    }
}

pub fn render(lang: Lang) -> Rendered {
    let mut title = String::new();
    let mut sections = Vec::new();
    let mut events: Vec<Event<'_>> = Vec::new();
    let mut heading: Option<(HeadingLevel, String, usize)> = None;
    // The line under the title that links to the other language (the site's
    // language switch does that).
    let mut skip_paragraph = false;
    let mut skipping = false;
    // A code block being read: (its language, its text).
    let mut code: Option<(String, String)> = None;
    // Tables: the texts of the header cells, written on each cell of the body
    // (`data-label`) for the cards they become on phones (`site.css`).
    let mut in_head = false;
    let mut head: Vec<String> = Vec::new();
    let mut column = 0;

    for event in Parser::new_ext(source(lang), Options::ENABLE_TABLES) {
        if skipping {
            if matches!(event, Event::End(TagEnd::Paragraph)) {
                skipping = false;
            }
            continue;
        }
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading = Some((level, String::new(), events.len()));
                events.push(Event::Text(CowStr::Borrowed("")));
            }
            Event::End(TagEnd::Heading(_)) => {
                let Some((level, text, at)) = heading.take() else {
                    continue;
                };
                if level == HeadingLevel::H1 {
                    title = text;
                    events.truncate(at);
                    skip_paragraph = true;
                    continue;
                }
                let id = slug(&text);
                let tag = format!("h{}", level as usize);
                if level == HeadingLevel::H2 {
                    sections.push((id.clone(), text));
                }
                events[at] = Event::Html(format!(r#"<{tag} id="{id}">"#).into());
                events.push(Event::Html(format!("</{tag}>").into()));
            }
            Event::Start(Tag::Paragraph) if skip_paragraph => {
                skip_paragraph = false;
                skipping = true;
            }
            Event::Start(Tag::Link { link_type, dest_url, title, id }) => {
                let (dest, external) = rewrite_link(&dest_url, lang);
                if external {
                    let attr = format!(r#"<a href="{dest}" target="_blank" rel="noopener">"#);
                    events.push(Event::Html(attr.into()));
                } else {
                    events.push(Event::Start(Tag::Link { link_type, dest_url: dest.into(), title, id }));
                }
            }
            Event::End(TagEnd::Link) => match events.iter().rev().find_map(|e| match e {
                Event::Html(h) if h.starts_with("<a ") => Some(true),
                Event::Start(Tag::Link { .. }) => Some(false),
                _ => None,
            }) {
                Some(true) => events.push(Event::Html("</a>".into())),
                _ => events.push(Event::End(TagEnd::Link)),
            },
            // Code wraps line by line under its indentation, colored when it is Rythme.
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(info) => info.split_whitespace().next().unwrap_or("").to_owned(),
                    CodeBlockKind::Indented => String::new(),
                };
                code = Some((lang, String::new()));
            }
            Event::Text(text) if code.is_some() => {
                if let Some((_, body)) = code.as_mut() {
                    body.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((lang, body)) = code.take() {
                    let block = if lang == "rythme" { code_block(&body) } else { plain_block(&body) };
                    events.push(Event::Html(Node::from(block).render().into()));
                }
            }
            Event::Start(Tag::TableHead) => {
                in_head = true;
                head.clear();
                events.push(event);
            }
            Event::End(TagEnd::TableHead) => {
                in_head = false;
                events.push(event);
            }
            Event::Start(Tag::TableRow) => {
                column = 0;
                events.push(event);
            }
            Event::Start(Tag::TableCell) if in_head => {
                head.push(String::new());
                events.push(event);
            }
            Event::Text(text) | Event::Code(text) if in_head => {
                if let Some(cell) = head.last_mut() {
                    cell.push_str(&text);
                }
                events.push(Event::Text(text));
            }
            Event::Start(Tag::TableCell) => {
                let label = head.get(column).map(|h| escape_attr(h)).unwrap_or_default();
                column += 1;
                events.push(Event::Html(format!(r#"<td data-label="{label}">"#).into()));
            }
            Event::End(TagEnd::TableCell) if !in_head => events.push(Event::Html("</td>".into())),
            Event::Start(Tag::Table(_)) => {
                events.push(Event::Html(r#"<div class="table-wrap">"#.into()));
                events.push(event);
            }
            Event::End(TagEnd::Table) => {
                events.push(event);
                events.push(Event::Html("</div>".into()));
            }
            Event::Text(text) | Event::Code(text) if heading.is_some() => {
                if let Some((_, h, _)) = heading.as_mut() {
                    h.push_str(&text);
                }
                events.push(Event::Text(text));
            }
            Event::Start(Tag::Paragraph) => {
                skip_paragraph = false;
                events.push(event);
            }
            other => events.push(other),
        }
    }
    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    Rendered { title, sections, html: out }
}

/// Text in a double-quoted attribute.
fn escape_attr(text: &str) -> String {
    text.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_guides_render() {
        for lang in Lang::ALL {
            let r = render(lang);
            assert!(!r.title.is_empty(), "{lang:?}");
            assert!(r.sections.len() >= 3, "{lang:?}");
            assert!(!r.html.contains("<h1"), "{lang:?}");
            // No link left to a Markdown file next to the guide: only to GitHub.
            for link in r.html.split("href=\"").skip(1).map(|rest| &rest[..rest.find('"').unwrap_or(0)]) {
                assert!(!link.ends_with(".md") || link.starts_with(REPO), "{lang:?}: {link}");
            }
            assert!(r.html.contains("<table>"), "{lang:?}");
            // Each cell of a table carries the title of its column, for phones.
            assert!(r.html.contains(r##"<td data-label="#">"##), "{lang:?}");
            assert!(!r.html.contains("<td>"), "{lang:?}");
            // Code wraps line by line; Rythme is colored.
            assert!(r.html.contains(r#"<pre class="code-block"><code translate="no"><span class="line">"#), "{lang:?}");
            assert!(r.html.contains(r#"<span class="t-kw">"#), "{lang:?}");
            for (id, _) in &r.sections {
                assert!(r.html.contains(&format!(r#"id="{id}""#)), "{id}");
            }
        }
    }

    #[test]
    fn links_lead_to_the_site() {
        assert_eq!(rewrite_link("../examples/hello.ry", Lang::Fr).0, "/fr/examples/#hello");
        assert_eq!(rewrite_link("teaching.md", Lang::Fr).0, "/en/teach/");
        assert_eq!(rewrite_link("../editors/vscode", Lang::En), (format!("{REPO}/tree/master/editors/vscode"), true));
        assert_eq!(slug("Pourquoi il convient à un cours"), "pourquoi-il-convient-a-un-cours");
    }
}
