//! The static export for GitHub Pages, behind `/Rythme` (its own test binary: the base
//! path is set once for the whole process).

use rythme_site::routes::Route;

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
    for file in
        ["404.html", ".nojekyll", "examples.json", "sitemap.xml", "robots.txt", "site.css", "site.js", "playground.js", "assets/logo.svg"]
    {
        assert!(out.join(file).is_file(), "{file}");
    }
    assert!(std::fs::read_to_string(out.join("sitemap.xml")).unwrap().contains("<loc>https://example.com/Rythme/en/learn/</loc>"));
    let _ = std::fs::remove_dir_all(&out);
}
