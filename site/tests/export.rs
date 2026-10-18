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
        // No link escapes the base path.
        for attr in [r#"href="/"#, r#"src="/"#] {
            for (at, _) in html.match_indices(attr) {
                assert!(html[at + attr.len() - 1..].starts_with("/Rythme/"), "{path}: {}", &html[at..at + 40]);
            }
        }
    }
    let playground = std::fs::read_to_string(out.join("fr/playground/index.html")).unwrap();
    assert!(playground.contains(r#"data-examples="/Rythme/examples.json""#));
    assert!(playground.contains(r#"data-wasm="/Rythme/rythme.wasm?v="#));
    let root = std::fs::read_to_string(out.join("index.html")).unwrap();
    assert!(root.contains(r#"var b="/Rythme""#));
    assert!(root.contains(r#"url=/Rythme/fr/"#));
    // `/` picks any language of the site, and links them all.
    assert!(root.contains("=(fr|en|es|de|pt|zh)(?:;|$)") && root.contains(r#"k=" fr en es de pt zh ""#), "{root}");
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
        "site.js",
        "playground.js",
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
    let _ = std::fs::remove_dir_all(&out);
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
    assert!(worker.contains(r#"const BASE = "/Rythme";"#));
    let start = worker.find("const PRECACHE = ").unwrap() + 17;
    let end = start + worker[start..].find(";\n").unwrap();
    let precache: Vec<String> = serde_json::from_str(&worker[start..end]).unwrap();
    assert!(precache.contains(&"/Rythme/fr/playground/".to_owned()));
    for url in &precache {
        assert!(file_of(out, url).is_file(), "{url}");
    }
}
