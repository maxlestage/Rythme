//! The installable app through Paquets' test client, at the root: the manifests, the
//! service worker, the offline pages and the tags of every page (`export.rs` checks
//! them behind `/Rythme`).

use std::sync::Arc;

use paquets::test::TestClient;
use rythme_site::i18n::Lang;
use rythme_site::routes::{Page, Route};
use rythme_site::server::{Site, app};
use serde_json::Value;

mod common;
use common::resolve;

async fn client() -> TestClient {
    app(Arc::new(Site::load(None))).test().await
}

/// The width and height written in a PNG header.
fn png_size(bytes: &[u8]) -> String {
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n", "a PNG file");
    let at = |n: usize| u32::from_be_bytes(bytes[n..n + 4].try_into().unwrap());
    format!("{}x{}", at(16), at(20))
}

/// Every image of a manifest (icons, screenshots, icons of the shortcuts and of the
/// file handler): (src, sizes, type).
fn images(manifest: &Value) -> Vec<(String, String, String)> {
    let mut list: Vec<&Value> = Vec::new();
    for key in ["icons", "screenshots"] {
        list.extend(manifest[key].as_array().unwrap());
    }
    for key in ["shortcuts", "file_handlers"] {
        for item in manifest[key].as_array().unwrap() {
            list.extend(item["icons"].as_array().unwrap());
        }
    }
    list.iter()
        .map(|i| (i["src"].as_str().unwrap().into(), i["sizes"].as_str().unwrap().into(), i["type"].as_str().unwrap().into()))
        .collect()
}

#[tokio::test]
async fn the_manifests_are_served_and_complete() {
    let client = client().await;
    for lang in Lang::ALL {
        let t = lang.texts();
        let code = lang.code();
        let path = format!("/{code}/manifest.webmanifest");
        let res = client.get(&path).await;
        assert_eq!(res.status(), 200, "{path}");
        assert_eq!(res.header("content-type"), Some("application/manifest+json"));
        let m: Value = serde_json::from_str(&res.text()).expect("the manifest is JSON");
        assert_eq!(m["name"], "Rythme");
        assert_eq!(m["short_name"], "Rythme");
        assert_eq!(m["description"], t.app_description);
        assert_eq!(m["lang"], lang.tag());
        assert_eq!(m["display"], "standalone");
        assert_eq!(m["id"], "/");
        assert_eq!(resolve(&path, m["start_url"].as_str().unwrap()), format!("/{code}/"));
        assert_eq!(resolve(&path, m["scope"].as_str().unwrap()), "/");
        assert_eq!(m["theme_color"], "#eef1f6");
        assert!(m["categories"].as_array().unwrap().iter().any(|c| c == "education"));
        let icons = m["icons"].as_array().unwrap();
        for (sizes, purpose) in [("192x192", "any"), ("512x512", "any"), ("512x512", "maskable"), ("any", "any")] {
            assert!(icons.iter().any(|i| i["sizes"] == sizes && i["purpose"] == purpose), "{path}: {sizes} {purpose}");
        }
        let shots = m["screenshots"].as_array().unwrap();
        for form in ["narrow", "wide"] {
            assert!(shots.iter().any(|s| s["form_factor"] == form && s["label"].is_string()), "{path}: {form}");
        }
        let shortcuts: Vec<String> =
            m["shortcuts"].as_array().unwrap().iter().map(|s| resolve(&path, s["url"].as_str().unwrap())).collect();
        let pages = [Page::Playground, Page::Examples, Page::Learn].map(|p| Route::new(lang, p).path());
        assert_eq!(shortcuts, pages);
        let playground = Route::new(lang, Page::Playground).path();
        let handler = &m["file_handlers"][0];
        assert_eq!(resolve(&path, handler["action"].as_str().unwrap()), playground);
        assert_eq!(handler["accept"]["text/x-rythme"][0], ".ry");
        assert_eq!(resolve(&path, m["share_target"]["action"].as_str().unwrap()), playground);
        assert_eq!(m["share_target"]["method"], "GET");
        assert_eq!(m["share_target"]["params"]["text"], "text");
        // Every image exists, with the type and the size the manifest gives.
        for (src, sizes, mime) in images(&m) {
            let url = resolve(&path, &src);
            let res = client.get(&url).await;
            assert_eq!(res.status(), 200, "{url}");
            assert_eq!(res.header("content-type"), Some(mime.as_str()), "{url}");
            if mime == "image/png" {
                assert_eq!(png_size(res.bytes()), sizes, "{url}");
            }
        }
    }
    assert_eq!(client.get("/it/manifest.webmanifest").await.status(), 404);
}

/// The list of a `const NAME = [...]` of the service worker.
fn constant(js: &str, name: &str) -> Value {
    let start = js.find(&format!("const {name} = ")).unwrap_or_else(|| panic!("{name}")) + name.len() + 9;
    let end = start + js[start..].find(";\n").unwrap();
    serde_json::from_str(&js[start..end]).unwrap_or_else(|e| panic!("{name}: {e}"))
}

#[tokio::test]
async fn the_service_worker_saves_only_urls_that_exist() {
    let client = client().await;
    let res = client.get("/sw.js").await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.header("content-type"), Some("text/javascript; charset=utf-8"));
    assert_eq!(res.header("cache-control"), Some("no-cache"));
    let js = res.text();
    assert!(!js.contains("__"), "every placeholder is replaced");
    assert_eq!(constant(&js, "BASE"), "");
    let version = constant(&js, "VERSION");
    assert_eq!(version.as_str().unwrap().len(), 10);
    let precache: Vec<String> = constant(&js, "PRECACHE").as_array().unwrap().iter().map(|u| u.as_str().unwrap().to_owned()).collect();
    let site = Site::load(None);
    // Every page in both languages, the offline pages, the files of the playground.
    for route in Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline))) {
        assert!(precache.contains(&route.path()), "{}", route.path());
    }
    assert!(precache.contains(&format!("/site.css?v={}", site.version)));
    // The browser side, once built (`./build.sh`): the bundles of the pages, with the
    // interpreter, and the playground's worker.
    if std::path::Path::new("public/pkg/playground_bg.wasm").is_file() {
        for file in ["/pkg/site.js", "/pkg/site_bg.wasm", "/pkg/playground.js", "/pkg/playground_bg.wasm", "/pkg/worker.js"] {
            assert!(precache.contains(&format!("{file}?v={}", site.version)), "{file}");
        }
    }
    // The AI page links llms-full.txt and llms.txt: offline too.
    for file in [
        "/examples.json",
        "/llms.txt",
        "/llms-full.txt",
        "/fr/ai/",
        "/zh/ai/",
        "/fr/manifest.webmanifest",
        "/en/manifest.webmanifest",
        "/icons/icon-512.png",
        "/favicon.ico",
    ] {
        assert!(precache.contains(&file.to_owned()), "{file}");
    }
    // A URL that fails would cancel the installation of the worker.
    for url in &precache {
        assert_eq!(client.get(url).await.status(), 200, "{url}");
    }
    let offline = constant(&js, "OFFLINE");
    for lang in Lang::ALL {
        assert_eq!(offline[lang.code()], Route::new(lang, Page::Offline).path());
    }
}

#[tokio::test]
async fn a_new_version_makes_a_new_worker() {
    let site = Site::load(None);
    let other = Site { site_url: None, version: "0000000000".into(), worker: String::new(), worker_wasm: String::new() };
    let worker = rythme_site::pwa::worker(&other);
    assert_ne!(constant(&worker, "VERSION"), constant(&site.worker, "VERSION"));
    assert!(worker.contains("/site.css?v=0000000000"));
}

#[tokio::test]
async fn every_page_is_installable() {
    let client = client().await;
    let routes = Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline)));
    for (path, lang) in routes.map(|r| (r.path(), r.lang)).chain([("/en/nothing/".to_owned(), Lang::En)]) {
        let res = client.get(&path).await;
        let html = res.text();
        let code = lang.code();
        assert!(html.contains(&format!(r#"<link rel="manifest" href="/{code}/manifest.webmanifest"/>"#)), "{path}");
        for tag in [
            r#"<link rel="apple-touch-icon" href="/icons/apple-touch-icon.png"/>"#,
            r#"<link rel="icon" href="/favicon.ico" sizes="32x32"/>"#,
            r#"<meta name="apple-mobile-web-app-capable" content="yes"/>"#,
            r#"<meta name="mobile-web-app-capable" content="yes"/>"#,
            r#"<meta name="apple-mobile-web-app-title" content="Rythme"/>"#,
            r#"<meta name="apple-mobile-web-app-status-bar-style" content="default"/>"#,
            r#"<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"/>"#,
            r#"data-sw="/sw.js" data-scope="/""#,
            r#"id="app-update""#,
            r#"id="app-install""#,
            r#"id="ios-hint""#,
        ] {
            assert!(html.contains(tag), "{path}: {tag}");
        }
        assert_eq!(html.matches(r#"name="viewport""#).count(), 1, "{path}");
        // The policy is kept: no inline script without its hash.
        let csp = res.header("content-security-policy").unwrap_or_default();
        let scripts = csp.split(';').find(|d| d.trim().starts_with("script-src")).unwrap_or_default();
        assert!(scripts.contains("'sha256-") && !scripts.contains("unsafe-inline"), "{path}: {csp}");
    }
}

#[tokio::test]
async fn the_offline_page_exists_in_both_languages() {
    let client = client().await;
    for lang in Lang::ALL {
        let path = Route::new(lang, Page::Offline).path();
        let res = client.get(&path).await;
        assert_eq!(res.status(), 200, "{path}");
        let html = res.text();
        assert!(html.starts_with(&format!("<!DOCTYPE html><html lang=\"{}\">", lang.tag())), "{path}");
        assert!(html.contains(&lang.texts().offline_title.replace('\'', "&#x27;")), "{path}");
        assert!(html.contains("noindex"), "{path}");
    }
    assert!(!client.get("/sitemap.xml").await.text().contains("offline"));
}

#[tokio::test]
async fn the_playground_opens_and_downloads_files() {
    let client = client().await;
    for lang in Lang::ALL {
        let html = client.get(&Route::new(lang, Page::Playground).path()).await.text();
        let t = lang.texts();
        for id in [r#"id="open""#, r#"id="save""#, r#"id="open-file""#, r#"accept=".ry,text/plain""#] {
            assert!(html.contains(id), "{id}");
        }
        assert!(html.contains(t.pg_save) && html.contains(t.pg_open));
    }
}
