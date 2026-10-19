//! The site for AI assistants: `/llms.txt` and `/llms-full.txt`, the Markdown of every
//! page with content (its code run through the real interpreter, like the HTML samples),
//! the structured data (JSON-LD) of the pages, the AI page and Copy for AI.

use std::path::Path;
use std::sync::Arc;

use paquets::test::TestClient;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use rythme_site::ai;
use rythme_site::examples::EXAMPLES;
use rythme_site::highlight::{ERROR_WORDS, WARNING_WORDS};
use rythme_site::i18n::Lang;
use rythme_site::routes::{Page, Route};
use rythme_site::server::{Site, app};
use serde_json::Value;

const ORIGIN: &str = "http://example.com";

/// The site without its rate limit: the tests follow every link of every page.
async fn client() -> TestClient {
    let mut app = app(Arc::new(Site::load(None)));
    app.security(None);
    app.test().await
}

async fn get(client: &TestClient, path: &str) -> (u16, String, String) {
    let res = client.get(path).header("host", "example.com").await;
    let kind = res.header("content-type").unwrap_or_default().to_owned();
    (res.status().as_u16(), kind, res.text())
}

/// A link of the site, without its origin and fragment, as the test client asks for it.
fn local(url: &str) -> Option<String> {
    let path = url.strip_prefix(ORIGIN)?;
    Some(path.split('#').next().unwrap_or("/").to_owned()).filter(|p| !p.is_empty())
}

/// The text of a double-quoted attribute, unescaped.
fn unescape(text: &str) -> String {
    text.replace("&quot;", "\"").replace("&#x27;", "'").replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
}

/// Every value of an attribute in a page.
fn attributes(html: &str, name: &str) -> Vec<String> {
    let open = format!(" {name}=\"");
    html.match_indices(&open)
        .map(|(at, _)| {
            let start = at + open.len();
            unescape(&html[start..start + html[start..].find('"').unwrap()])
        })
        .collect()
}

#[tokio::test]
async fn the_llms_files_are_served() {
    let client = client().await;
    let (status, kind, full) = get(&client, "/llms-full.txt").await;
    assert_eq!((status, kind.as_str()), (200, "text/plain; charset=utf-8"));
    assert_eq!(full, ai::LLMS_FULL);
    // Nothing runs in them: a policy of their own, which lets the browser show them.
    for path in ["/llms.txt", "/llms-full.txt", "/en/learn/index.md"] {
        let res = client.get(path).await;
        assert_eq!(res.header("content-security-policy"), Some(rythme_site::server::TEXT_POLICY), "{path}");
    }
    let (status, kind, index) = get(&client, "/llms.txt").await;
    assert_eq!((status, kind.as_str()), (200, "text/plain; charset=utf-8"));
    assert!(index.starts_with("# Rythme\n\n> "), "{index}");
    // Every address of the site in the index exists; the others are those of docs/.
    let links: Vec<String> = Parser::new(&index)
        .filter_map(|e| match e {
            Event::Start(Tag::Link { dest_url, .. }) => Some(dest_url.to_string()),
            _ => None,
        })
        .collect();
    assert!(links.contains(&format!("{ORIGIN}/llms-full.txt")), "{links:?}");
    for page in [Page::Home, Page::Learn, Page::Examples, Page::Teach, Page::Ai] {
        assert!(links.contains(&format!("{ORIGIN}{}", Route::new(Lang::En, page).markdown_path())), "{page:?}");
    }
    for url in &links {
        assert!(url.starts_with("https://") || url.starts_with(ORIGIN), "{url}");
        match local(url) {
            Some(path) => assert_eq!(client.get(&path).await.status(), 200, "{url}"),
            None => assert!(ai::LLMS.contains(url.as_str()), "{url}: not in docs/llms.txt"),
        }
    }
    let robots = get(&client, "/robots.txt").await.2;
    assert!(robots.contains(&format!("{ORIGIN}/llms.txt")) && robots.contains(&format!("{ORIGIN}/llms-full.txt")), "{robots}");
    assert!(robots.contains(&format!("Sitemap: {ORIGIN}/sitemap.xml")), "{robots}");
    // Every page links them, and its Markdown when it has one.
    for route in Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline))) {
        let html = get(&client, &route.path()).await.2;
        assert!(html.contains(r#"<link rel="alternate" type="text/plain" href="/llms.txt" title="llms.txt"/>"#), "{route:?}");
        assert!(html.contains(r#"<link rel="alternate" type="text/plain" href="/llms-full.txt" title="llms-full.txt"/>"#), "{route:?}");
        let markdown = format!(r#"<link rel="alternate" type="text/markdown" href="{}" title="Markdown"/>"#, route.markdown_path());
        assert_eq!(html.contains(&markdown), route.page.has_markdown(), "{route:?}");
    }
}

/// The blocks of code of a Markdown text: (language, text).
fn code_blocks(markdown: &str) -> Vec<(String, String)> {
    let mut blocks = Vec::new();
    let mut current: Option<(String, String)> = None;
    for event in Parser::new_ext(markdown, Options::ENABLE_TABLES) {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(info) => info.to_string(),
                    CodeBlockKind::Indented => panic!("an indented code block"),
                };
                current = Some((lang, String::new()));
            }
            Event::Text(text) => {
                if let Some((_, body)) = current.as_mut() {
                    body.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => blocks.push(current.take().unwrap()),
            _ => {}
        }
    }
    blocks
}

#[tokio::test]
async fn every_page_has_its_markdown() {
    let client = client().await;
    for route in Route::all().filter(|r| r.page.has_markdown()) {
        let path = route.markdown_path();
        let res = client.get(&path).header("host", "example.com").await;
        assert_eq!(res.status(), 200, "{path}");
        assert_eq!(res.header("content-type"), Some("text/markdown; charset=utf-8"), "{path}");
        assert_eq!(res.header("link"), Some(format!("<{ORIGIN}{}>; rel=\"canonical\"", route.path()).as_str()), "{path}");
        let markdown = res.text();
        assert!(markdown.starts_with("# "), "{path}");
        // Clean Markdown: no HTML, every link absolute and, on the site, an existing page.
        let mut headings = 0;
        for event in Parser::new_ext(&markdown, Options::ENABLE_TABLES) {
            match event {
                Event::Html(html) | Event::InlineHtml(html) => panic!("{path}: HTML in the Markdown: {html}"),
                Event::Start(Tag::Heading { level: pulldown_cmark::HeadingLevel::H1, .. }) => headings += 1,
                Event::Start(Tag::Link { dest_url, .. }) => {
                    assert!(dest_url.starts_with("https://") || dest_url.starts_with(ORIGIN), "{path}: {dest_url}");
                    if let Some(local) = local(&dest_url) {
                        assert_eq!(client.get(&local).await.status(), 200, "{path}: {dest_url}");
                    }
                }
                _ => {}
            }
        }
        assert_eq!(headings, 1, "{path}: one title");
        let blocks = code_blocks(&markdown);
        for (lang, _) in &blocks {
            assert!(["rythme", "text", "sh", "json"].contains(&lang.as_str()), "{path}: ```{lang}");
        }
        let rythme = blocks.iter().filter(|(l, _)| l == "rythme").count();
        let t = route.lang.texts();
        match route.page {
            Page::Learn => assert_eq!(rythme, t.tour.len(), "{path}"),
            Page::Examples => assert_eq!(rythme, EXAMPLES.len(), "{path}"),
            Page::Home => assert_eq!(rythme, 2, "{path}"),
            _ => {}
        }
        assert!(markdown.contains(&format!("<{ORIGIN}{}>", route.path())), "{path}: the address of the page");
    }
    // Only the pages with content, and never in the sitemap (it lists the canonical pages).
    for path in ["/en/playground/index.md", "/en/offline/index.md", "/en/nothing/index.md", "/it/index.md", "/en/learn/more/index.md"] {
        assert_eq!(client.get(path).await.status(), 404, "{path}");
    }
    assert!(!get(&client, "/sitemap.xml").await.2.contains("index.md"));
}

/// Runs `code` like the playground does, in the folder of the examples; returns the
/// output and whether it failed.
fn run(code: &str, lang: Lang, test: bool) -> (String, bool) {
    use rythme::driver::Driver;
    use rythme::interp::Mode;
    let interpreter = rythme::lang::Lang::all().nth(lang.interpreter()).expect("a language of the interpreter");
    rythme::lang::set(interpreter);
    rythme::io::capture();
    let mut driver = Driver::new();
    driver.color = false;
    let mut ok = false;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../examples");
    if let Some(m) = driver.load_source("main.ry", code, None, &dir) {
        let mode = if test { Mode::Test } else { Mode::Run };
        ok = driver.execute(&m, mode).is_ok() && (!test || driver.run_tests());
    }
    let out = rythme::io::take_captured();
    let failed = !ok || ERROR_WORDS.iter().any(|w| out.contains(&format!("{w}[")));
    (out, failed)
}

/// Every piece of Rythme code of the Markdown runs in the language of its page (the
/// examples in English, like their `.out` files): followed
/// by a block of text, it prints exactly that text (the error of the home page too);
/// otherwise it runs without an error or a warning.
#[tokio::test]
async fn the_code_of_the_markdown_runs() {
    let client = client().await;
    for lang in Lang::ALL {
        for page in [Page::Home, Page::Learn, Page::Examples, Page::Ai] {
            let path = Route::new(lang, page).markdown_path();
            let blocks = code_blocks(&get(&client, &path).await.2);
            for (n, (kind, code)) in blocks.iter().enumerate() {
                if kind != "rythme" {
                    continue;
                }
                // `rythme test` for the tour's tests; the examples as `rythme file.ry` runs them.
                let test = page != Page::Examples && code.contains("test \"");
                // The examples print what their `.out` files hold: `rythme` in English.
                let (out, failed) = run(code, if page == Page::Examples { Lang::En } else { lang }, test);
                let out = out.trim_matches(['\u{1}', '\u{2}', '\n']).to_owned();
                match blocks.get(n + 1) {
                    Some((next, expected)) if next == "text" => {
                        assert_eq!(out.lines().collect::<Vec<_>>(), expected.trim_end().lines().collect::<Vec<_>>(), "{path}: {code}")
                    }
                    _ => {
                        assert!(!failed, "{path}: {code}\n{out}");
                        assert!(!WARNING_WORDS.iter().any(|w| out.contains(&format!("{w}["))), "{path}: {code}\n{out}");
                    }
                }
            }
        }
    }
}

/// The JSON-LD blocks of a page, parsed.
fn json_ld(html: &str) -> Vec<Value> {
    const OPEN: &str = r#"<script type="application/ld+json">"#;
    html.match_indices(OPEN)
        .map(|(at, _)| {
            let start = at + OPEN.len();
            let text = &html[start..start + html[start..].find("</script>").unwrap()];
            serde_json::from_str(text).unwrap_or_else(|e| panic!("{e}: {text}"))
        })
        .collect()
}

#[tokio::test]
async fn the_structured_data_parses() {
    let client = client().await;
    assert_eq!(ai::version(), rythme::VERSION);
    for route in Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline))) {
        let html = get(&client, &route.path()).await.2;
        let data = json_ld(&html);
        let t = route.lang.texts();
        let path = route.path();
        match route.page {
            Page::Home => {
                assert_eq!(data.len(), 1, "{path}");
                let graph = data[0]["@graph"].as_array().expect("a graph");
                let node = |kind: &str| graph.iter().find(|n| n["@type"] == kind).unwrap_or_else(|| panic!("{path}: {kind}"));
                let language = node("ComputerLanguage");
                assert_eq!(language["name"], "Rythme");
                assert_eq!(language["version"], rythme::VERSION);
                let app = node("SoftwareApplication");
                assert_eq!(app["softwareVersion"], rythme::VERSION);
                assert_eq!(app["downloadUrl"], "https://github.com/maxlestage/Rythme/releases");
                assert_eq!(app["license"], rythme_site::structured::LICENSE);
                assert_eq!(app["inLanguage"], route.lang.tag());
                assert_eq!(app["installUrl"], format!("{ORIGIN}{path}#install"));
                let source = node("SoftwareSourceCode");
                assert_eq!(source["programmingLanguage"]["name"], "Rust");
                assert_eq!(source["codeRepository"], "https://github.com/maxlestage/Rythme");
            }
            Page::Learn | Page::Examples | Page::Teach | Page::Ai => {
                assert_eq!(data.len(), 1, "{path}");
                let article = &data[0];
                assert_eq!(article["@type"], "TechArticle", "{path}");
                assert_eq!(article["url"], format!("{ORIGIN}{path}"));
                assert_eq!(article["inLanguage"], route.lang.tag());
                assert_eq!(article["encoding"]["contentUrl"], format!("{ORIGIN}{}", route.markdown_path()));
                let parts = article["hasPart"].as_array().map(Vec::as_slice).unwrap_or_default();
                let codes: Vec<&str> = parts.iter().map(|p| p["text"].as_str().unwrap()).collect();
                match route.page {
                    Page::Learn => assert_eq!(codes, t.tour.iter().map(|s| s.code).collect::<Vec<_>>(), "{path}"),
                    Page::Examples => assert_eq!(codes, EXAMPLES.iter().map(|e| e.code).collect::<Vec<_>>(), "{path}"),
                    _ => assert!(codes.is_empty(), "{path}"),
                }
                for part in parts {
                    assert_eq!(part["@type"], "SoftwareSourceCode");
                    assert_eq!(part["programmingLanguage"]["name"], "Rythme");
                }
            }
            _ => assert!(data.is_empty(), "{path}"),
        }
    }
}

/// Each page has its own title and a description, in its language.
#[tokio::test]
async fn titles_and_descriptions() {
    let client = client().await;
    for lang in Lang::ALL {
        let mut titles = Vec::new();
        for page in Page::ALL {
            let html = get(&client, &Route::new(lang, page).path()).await.2;
            let title = &html[html.find("<title>").unwrap() + 7..html.find("</title>").unwrap()];
            assert!(title.contains("Rythme"), "{lang:?} {page:?}: {title}");
            assert!(html.contains(r#"<meta name="description" content=""#), "{lang:?} {page:?}");
            let at = html.find(r#"<meta name="description" content=""#).unwrap() + 34;
            let text = &html[at..at + html[at..].find('"').unwrap()];
            assert!(text.chars().count() >= 20 && text.chars().count() <= 300, "{lang:?} {page:?}: {text}");
            titles.push(title.to_owned());
        }
        let mut unique = titles.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), titles.len(), "{lang:?}: {titles:?}");
    }
}

#[tokio::test]
async fn the_ai_page() {
    let client = client().await;
    // The configurations are those of the manual.
    let manual = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/reference.md")).unwrap();
    for snippet in [ai::CLAUDE_CODE, ai::MCP_JSON, ai::VSCODE_JSON, ai::FIX_EXAMPLE] {
        assert!(manual.contains(snippet), "not in docs/reference.md: {snippet}");
    }
    for lang in Lang::ALL {
        let t = lang.texts();
        let code = lang.code();
        let html = get(&client, &format!("/{code}/ai/")).await.2;
        // In the menu and the footer of every page.
        let home = get(&client, &format!("/{code}/")).await.2;
        assert_eq!(home.matches(&format!(r#"href="/{code}/ai/""#)).count(), 2, "{lang:?}");
        assert!(html.contains(&format!(r#"href="/{code}/ai/" aria-current="page""#)), "{lang:?}");
        // How to give an assistant the language, with the copy buttons of the message and
        // of the configurations of the MCP server.
        for href in ["/llms-full.txt", "/llms.txt", &format!("/{code}/learn/index.md")] {
            assert!(html.contains(&format!(r#"href="{href}""#)), "{lang:?}: {href}");
        }
        let copies = attributes(&html, "data-copy");
        let prompt = ai::prompt(lang, ORIGIN);
        assert!(prompt.contains(&format!("{ORIGIN}/llms-full.txt")), "{prompt}");
        for text in [prompt.as_str(), ai::CLAUDE_CODE, ai::MCP_JSON, ai::VSCODE_JSON, ai::NEW_PROJECT] {
            assert!(copies.iter().any(|c| c == text), "{lang:?}: {text}");
        }
        for text in [t.ai_title, t.ai_mcp_title, t.ai_loop_title, t.ai_agents_title, t.ai_play_title] {
            assert!(unescape(&html).contains(text), "{lang:?}: {text}");
        }
        for cmd in ai::LOOP {
            assert!(html.contains(cmd), "{lang:?}: {cmd}");
        }
        assert!(t.ai_prompt.contains("{url}") && t.pg_ai_intro.contains("{url}"), "{lang:?}");
        // Copy for AI in the playground: hidden until playground.js shows it, with the
        // address of llms-full.txt and its texts.
        let playground = get(&client, &format!("/{code}/playground/")).await.2;
        assert!(
            playground.contains(&format!(
                r#"id="copy-ai" title="{}" aria-controls="share-box" aria-expanded="false" hidden="">{}</button>"#,
                t.pg_ai_title.replace('\'', "&#x27;"),
                t.pg_ai.replace('\'', "&#x27;")
            )),
            "{lang:?}"
        );
        assert!(playground.contains(r#"data-llms="/llms-full.txt""#), "{lang:?}");
        assert!(playground.contains(&format!(r#"<label class="share__label" for="ai-text">{}</label>"#, t.pg_ai_label)), "{lang:?}");
        let ui: Value = serde_json::from_str(&attributes(&playground, "data-ui")[0]).unwrap();
        assert_eq!(ui["aiCopied"], t.pg_ai_copied);
        assert_eq!(ui["aiSelect"], t.pg_ai_select);
        assert_eq!(ui["aiIntro"], t.pg_ai_intro);
    }
}
