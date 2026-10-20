//! The static export for GitHub Pages, behind `/Rythme` (its own test binary: the base
//! path is set once for the whole process).

use std::path::Path;

use rythme_site::i18n::Lang;
use rythme_site::routes::Route;
use serde_json::Value;

mod common;
use common::resolve;

#[tokio::test]
async fn exports_every_page_behind_the_base_path() {
    let out = std::env::temp_dir().join(format!("rythme-site-export-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    let files = rythme_site::export::export(&out, "/Rythme", Some("https://example.com/Rythme".into())).await.unwrap();
    assert!(files > 15, "{files}");
    for route in Route::all() {
        let file = out.join(format!("{}index.html", route.path().trim_start_matches('/')));
        let html = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        let path = route.path();
        assert!(html.contains(&format!(r#"<link rel="canonical" href="https://example.com/Rythme{path}"/>"#)), "{path}");
        assert!(html.contains("/Rythme/site.css?v="), "{path}");
        // The browser side, behind the base path too.
        assert!(html.contains(r#"<script type="module">import init from "/Rythme/pkg/"#), "{path}");
        assert!(html.contains(r#"init({module_or_path:"/Rythme/pkg/"#), "{path}");
        // No link escapes the base path.
        for attr in [r#"href="/"#, r#"src="/"#] {
            for (at, _) in html.match_indices(attr) {
                assert!(html[at + attr.len() - 1..].starts_with("/Rythme/"), "{path}: {}", &html[at..at + 40]);
            }
        }
    }
    let playground = std::fs::read_to_string(out.join("fr/playground/index.html")).unwrap();
    assert!(playground.contains(r#"data-examples="/Rythme/examples.json""#));
    assert!(playground.contains(r#"data-worker="/Rythme/pkg/worker.js?v="#));
    assert!(playground.contains(r#"<script type="module">import init from "/Rythme/pkg/playground.js?v="#));
    let root = std::fs::read_to_string(out.join("index.html")).unwrap();
    assert!(root.contains(r#"data-base="/Rythme""#));
    assert!(root.contains(r#"url=/Rythme/fr/"#));
    // `/` picks any language of the site (the browser side, which it loads), and links
    // them all.
    assert!(root.contains(r#"data-pick-language="fr en es de pt zh""#), "{root}");
    assert!(root.contains(r#"<script type="module">import init from "/Rythme/pkg/site.js?v="#), "{root}");
    assert!(!root.contains("<script>"), "no inline script but the bundle's start: {root}");
    for lang in Lang::ALL {
        assert!(root.contains(&format!(r#"<a href="/Rythme/{}/" hreflang="{}""#, lang.code(), lang.tag())), "{lang:?}");
    }
    for file in [
        "404.html",
        ".nojekyll",
        "examples.json",
        "sitemap.xml",
        "robots.txt",
        "site.css",
        "pkg/worker.js",
        "assets/logo.svg",
        "sw.js",
        "favicon.ico",
        "fr/offline/index.html",
        "zh/offline/index.html",
        "zh/teach/index.html",
        "en/manifest.webmanifest",
        "pt/manifest.webmanifest",
    ] {
        assert!(out.join(file).is_file(), "{file}");
    }
    assert!(std::fs::read_to_string(out.join("sitemap.xml")).unwrap().contains("<loc>https://example.com/Rythme/en/learn/</loc>"));
    installable(&out);
    for_assistants(&out);
    let _ = std::fs::remove_dir_all(&out);
}

/// For AI assistants, on GitHub Pages too: `llms.txt` and `llms-full.txt` at the root of
/// the base path, every page with content as Markdown next to its HTML, and every
/// address of the site they give (on `SITE_URL`) is a file of the export.
fn for_assistants(out: &Path) {
    let index = std::fs::read_to_string(out.join("llms.txt")).unwrap();
    assert!(index.contains("](https://example.com/Rythme/llms-full.txt)"), "{index}");
    assert_eq!(std::fs::read_to_string(out.join("llms-full.txt")).unwrap(), rythme_site::ai::LLMS_FULL);
    let robots = std::fs::read_to_string(out.join("robots.txt")).unwrap();
    assert!(robots.contains("https://example.com/Rythme/llms.txt"), "{robots}");
    let mut texts = vec![("llms.txt".to_owned(), index)];
    for route in Route::all().filter(|r| r.page.has_markdown()) {
        let file = route.markdown_path().trim_start_matches('/').to_owned();
        let text = std::fs::read_to_string(out.join(&file)).unwrap_or_else(|e| panic!("{file}: {e}"));
        let html = std::fs::read_to_string(out.join(format!("{}index.html", route.path().trim_start_matches('/')))).unwrap();
        assert!(html.contains(&format!(r#"type="text/markdown" href="/Rythme{}""#, route.markdown_path())), "{file}");
        texts.push((file, text));
    }
    for (file, text) in texts {
        for (at, _) in text.match_indices("https://example.com/") {
            let url: String =
                text[at..].chars().take_while(|c| c.is_ascii_graphic() && !matches!(c, ')' | '>' | ' ' | '\n' | '#')).collect();
            let url = url.trim_end_matches([':', '.', ',']);
            assert!(url.starts_with("https://example.com/Rythme/"), "{file}: {url}");
            assert!(file_of(out, &url["https://example.com".len()..]).is_file(), "{file}: {url}");
        }
    }
}

/// The file of the export served at `url` (behind `/Rythme`).
fn file_of(out: &Path, url: &str) -> std::path::PathBuf {
    let path = url.strip_prefix("/Rythme/").unwrap_or_else(|| panic!("{url} escapes the base path"));
    let path = path.split('?').next().unwrap();
    if path.is_empty() || path.ends_with('/') { out.join(path).join("index.html") } else { out.join(path) }
}

/// The app installs from GitHub Pages too: its manifests, worker and offline pages
/// are behind the base path, and every URL they give is a file of the export.
fn installable(out: &Path) {
    for lang in Lang::ALL {
        let code = lang.code();
        let path = format!("/Rythme/{code}/manifest.webmanifest");
        let m: Value = serde_json::from_str(&std::fs::read_to_string(file_of(out, &path)).unwrap()).unwrap();
        assert_eq!(m["id"], "/Rythme/");
        assert_eq!(resolve(&path, m["start_url"].as_str().unwrap()), format!("/Rythme/{code}/"));
        assert_eq!(resolve(&path, m["scope"].as_str().unwrap()), "/Rythme/");
        let mut urls: Vec<&str> = Vec::new();
        for key in ["icons", "screenshots"] {
            urls.extend(m[key].as_array().unwrap().iter().map(|i| i["src"].as_str().unwrap()));
        }
        urls.extend(m["shortcuts"].as_array().unwrap().iter().map(|s| s["url"].as_str().unwrap()));
        urls.push(m["file_handlers"][0]["action"].as_str().unwrap());
        urls.push(m["share_target"]["action"].as_str().unwrap());
        for url in urls {
            let url = resolve(&path, url);
            assert!(file_of(out, &url).is_file(), "{path}: {url}");
        }
        let offline = std::fs::read_to_string(out.join(format!("{code}/offline/index.html"))).unwrap();
        assert!(offline.contains(lang.texts().offline_retry), "{code}");
        let home = std::fs::read_to_string(out.join(format!("{code}/index.html"))).unwrap();
        assert!(home.contains(&format!(r#"<link rel="manifest" href="{path}"/>"#)), "{code}");
        assert!(home.contains(r#"data-sw="/Rythme/sw.js" data-scope="/Rythme/""#), "{code}");
    }
    let worker = std::fs::read_to_string(out.join("sw.js")).unwrap();
    // Its bundle and what the server knows, behind the base path too.
    assert!(worker.starts_with(r#"importScripts("/Rythme/pkg/sw.js?v="#), "{worker}");
    assert!(worker.contains(r#","/Rythme/pkg/sw_wasm.js?v="#), "{worker}");
    let start = worker.find("wasm_bindgen.serve(").unwrap() + 19;
    let end = start + worker[start..].find(");\n").unwrap();
    let config: Value = serde_json::from_str(&worker[start..end]).unwrap();
    assert_eq!(config["base"], "/Rythme");
    let precache: Vec<String> = serde_json::from_value(config["precache"].clone()).unwrap();
    if out.join("pkg/sw_bg.wasm").is_file() {
        assert!(out.join("pkg/sw_wasm.js").is_file());
    }
    assert!(precache.contains(&"/Rythme/fr/playground/".to_owned()));
    for url in &precache {
        assert!(file_of(out, url).is_file(), "{url}");
    }
}
