//! The whole site through Paquets' in-memory test client, and every piece of Rythme
//! code it shows through the real interpreter.

use std::path::Path;
use std::sync::Arc;

use paquets::test::TestClient;
use rythme_site::examples::EXAMPLES;
use rythme_site::i18n::Lang;
use rythme_site::routes::{Page, Route};
use rythme_site::server::{Site, app};

async fn client() -> TestClient {
    app(Arc::new(Site::load(None))).test().await
}

#[tokio::test]
async fn every_page_renders_in_both_languages() {
    let client = client().await;
    for route in Route::all() {
        let path = route.path();
        let res = client.get(&path).header("host", "example.com").await;
        assert_eq!(res.status(), 200, "{path}");
        let html = res.text();
        let code = route.lang.code();
        assert!(html.starts_with(&format!("<!DOCTYPE html><html lang=\"{code}\">")), "{path}");
        assert!(html.contains(&format!(r#"<link rel="canonical" href="http://example.com{path}"/>"#)), "{path}");
        for lang in Lang::ALL {
            let other = route.with_lang(lang).path();
            // The language switch, and the alternate link for search engines.
            assert!(html.contains(&format!(r#"href="{other}" hreflang="{}""#, lang.code())), "{path}: {other}");
            assert!(html.contains(&format!(r#"hreflang="{}" href="http://example.com{other}""#, lang.code())), "{path}");
        }
        // The burger menu and the navigation.
        assert!(html.contains(r#"class="burger""#), "{path}");
        assert!(html.contains(r#"aria-controls="nav""#), "{path}");
        assert!(html.contains("/site.js?v="), "{path}");
        let csp = res.header("content-security-policy").unwrap_or_default().to_owned();
        assert!(csp.contains("'wasm-unsafe-eval'"), "{path}: {csp}");
        assert!(csp.contains("'sha256-"), "{path}: {csp}");
    }
}

#[tokio::test]
async fn the_pages_hold_their_content() {
    let client = client().await;
    for lang in Lang::ALL {
        let t = lang.texts();
        let page = |p| async move { client.get(&Route::new(lang, p).path()).await.text() };
        let home = page(Page::Home).await;
        for item in &t.why {
            assert!(home.contains(&item.title.replace('\'', "&#x27;")), "{}", item.title);
        }
        assert!(home.contains(r#"id="install""#));
        let examples = page(Page::Examples).await;
        for e in EXAMPLES {
            assert!(examples.contains(&format!(r#"id="{}""#, e.name)), "{}", e.name);
            assert!(examples.contains(&format!("#example={}", e.name)), "{}", e.name);
        }
        let playground = page(Page::Playground).await;
        assert!(playground.contains(r#"id="source""#));
        assert!(playground.contains(r#"id="format""#) && playground.contains(t.pg_format), "the Format button");
        assert!(playground.contains(r#"id="steps""#) && playground.contains(t.pg_steps), "the Steps button");
        assert!(playground.contains("/playground.js?v="));
        assert!(playground.contains(r#"data-wasm="/rythme.wasm?v="#));
        let learn = page(Page::Learn).await;
        assert_eq!(learn.matches("class=\"tour__try\"").count(), t.tour.len());
        let teach = page(Page::Teach).await;
        assert!(teach.contains("<table>"));
    }
}

#[tokio::test]
async fn redirects_and_not_found() {
    let client = client().await;
    let res = client.get("/").header("accept-language", "en-US,en;q=0.9").await;
    assert_eq!(res.status(), 302);
    assert_eq!(res.header("location"), Some("/en/"));
    let res = client.get("/").header("accept-language", "de").await;
    assert_eq!(res.header("location"), Some("/fr/"));
    // The language chosen with the switch wins over the browser's.
    let res = client.get("/").header("accept-language", "fr").header("cookie", "rythme-lang=en").await;
    assert_eq!(res.header("location"), Some("/en/"));
    let res = client.get("/en/learn").await;
    assert_eq!(res.status(), 301);
    assert_eq!(res.header("location"), Some("/en/learn/"));
    for path in ["/en/nothing/", "/fr/learn/more/", "/de/", "/nothing"] {
        let res = client.get(path).await;
        assert_eq!(res.status(), 404, "{path}");
        assert!(res.text().contains("noindex"), "{path}");
    }
    assert!(client.get("/en/nothing/").await.text().contains("<html lang=\"en\">"));
}

#[tokio::test]
async fn examples_json_sitemap_and_robots() {
    let client = client().await;
    let res = client.get("/examples.json").await;
    assert_eq!(res.status(), 200);
    let json = res.text();
    for e in EXAMPLES {
        assert!(json.contains(&format!("\"{}\":", e.name)), "{}", e.name);
    }
    let sitemap = client.get("/sitemap.xml").header("host", "example.com").await.text();
    assert_eq!(sitemap.matches("<loc>").count(), Route::all().count());
    assert!(client.get("/robots.txt").header("host", "example.com").await.text().contains("http://example.com/sitemap.xml"));
}

/// Every top-level program of `examples/` is on the examples page and in the
/// playground's menu.
#[test]
fn every_example_is_listed() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../examples");
    let mut files: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| {
            let name = e.ok()?.file_name().into_string().ok()?;
            name.strip_suffix(".ry").map(str::to_owned)
        })
        .collect();
    files.sort();
    let mut listed: Vec<String> = EXAMPLES.iter().map(|e| e.name.to_owned()).collect();
    listed.sort();
    assert_eq!(files, listed);
}

/// Runs `code` like the playground does; returns the output and whether it failed.
fn run(code: &str, lang: Lang, test: bool) -> (String, bool) {
    use rythme::driver::Driver;
    use rythme::interp::Mode;
    rythme::lang::set(match lang {
        Lang::Fr => rythme::lang::Lang::Fr,
        Lang::En => rythme::lang::Lang::En,
    });
    rythme::io::capture();
    let mut driver = Driver::new();
    driver.color = false;
    let mut ok = false;
    if let Some(m) = driver.load_source("main.ry", code, None, Path::new(".")) {
        let mode = if test { Mode::Test } else { Mode::Run };
        ok = driver.execute(&m, mode).is_ok() && (!test || driver.run_tests());
    }
    let out = rythme::io::take_captured();
    let failed = !ok || out.contains("error[") || out.contains("erreur[");
    (out, failed)
}

#[test]
fn every_piece_of_code_runs() {
    for lang in Lang::ALL {
        let t = lang.texts();
        let (out, failed) = run(t.hero_code, lang, false);
        assert!(!failed, "{lang:?} hero: {out}");
        assert_eq!(out.trim(), "20", "{lang:?}: the hero shows 20 as its output");
        let (out, failed) = run(t.pg_welcome_code, lang, false);
        assert!(!failed && !out.contains("warning[") && !out.contains("avertissement["), "{lang:?} welcome: {out}");
        for step in &t.tour {
            let test = step.code.contains("test \"");
            let (out, failed) = run(step.code, lang, test);
            assert!(!failed, "{lang:?} {}: {out}", step.title);
            // The samples of the site are good examples: not even a warning.
            assert!(!out.contains("warning[") && !out.contains("avertissement["), "{lang:?} {}: {out}", step.title);
        }
        // The error of the home page is the interpreter's own message.
        let (out, failed) = run(t.errors_code, lang, false);
        assert!(failed);
        let shown: Vec<&str> = t.errors_message.lines().skip(2).collect();
        let real: Vec<&str> = out.trim_matches(['\u{1}', '\u{2}', '\n']).lines().skip(2).collect();
        assert_eq!(shown, real, "{lang:?}");
    }
}

/// The theme picker of every page, and the inline script that applies the theme before
/// the first paint, allowed by its hash in the Content-Security-Policy.
#[tokio::test]
async fn the_theme_picker_and_its_script() {
    use paquets::securite::headers::ContentSecurityPolicy;
    use rythme_site::themes::{self, THEMES};

    let client = client().await;
    let script = format!("<script>{}</script>", themes::head_script());
    let hash = ContentSecurityPolicy::hashes_for_inline_scripts(&script).pop().expect("a hash");
    let routes = Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline)));
    for (path, lang) in routes.map(|r| (r.path(), r.lang)).chain([("/en/nothing/".to_owned(), Lang::En)]) {
        let res = client.get(&path).await;
        let html = res.text();
        let t = lang.texts();
        // The script runs in the head, after the theme-color tags it updates.
        let at = html.find(&script).unwrap_or_else(|| panic!("{path}: the theme script"));
        assert!(at < html.find("<body").unwrap(), "{path}");
        assert!(html.find(r#"<meta name="theme-color""#).unwrap() < at, "{path}");
        assert_eq!(html.matches("<script>").count(), 1, "{path}: one inline script");
        let csp = res.header("content-security-policy").unwrap_or_default();
        assert!(csp.contains(&hash), "{path}: {csp}");
        for inline in ContentSecurityPolicy::hashes_for_inline_scripts(&html) {
            assert!(csp.contains(&inline), "{path}: {inline}");
        }
        // The picker: hidden without JavaScript, a button for computers, a fieldset of
        // radio buttons with their labels in the language of the page.
        assert!(html.contains(r#"<li class="nav__item--theme" hidden="">"#), "{path}");
        assert!(html.contains(r#"id="theme-toggle" aria-expanded="false" aria-controls="theme-panel""#), "{path}");
        assert!(html.contains(&format!(r#"title="{}""#, t.theme_title)), "{path}");
        let legend = format!(r#"<fieldset class="theme__panel" id="theme-panel"><legend class="theme__legend">{}</legend>"#, t.theme_label);
        assert!(html.contains(&legend), "{path}");
        assert_eq!(html.matches(r#"type="radio" name="theme""#).count(), THEMES.len(), "{path}");
        assert_eq!(html.matches(" checked=\"\"").count(), 1, "{path}: Auto is chosen by default");
        assert!(html.contains(r##"value="auto" id="theme-auto" data-color="#eef1f6" checked="" data-dark="#0e1118""##), "{path}");
        for theme in THEMES {
            let id = theme.id;
            assert!(html.contains(&format!(r#"<label class="theme__option" for="theme-{id}">"#)), "{path}: {id}");
            assert!(html.contains(&format!(r#"value="{id}" id="theme-{id}" data-color="{}""#, theme.color)), "{path}: {id}");
            assert!(html.contains(&format!(r#"<span class="theme__name">{}</span>"#, theme.name(t))), "{path}: {id}");
        }
    }
}
