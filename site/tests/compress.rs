//! What the server sends compressed, and how long browsers keep it (`src/compress.rs`):
//! the pages and the other answers compressed on the fly, the bundles of the browser side
//! from the copies `./build.sh` compressed once, the versioned files kept a year, the
//! pages revalidated with their `ETag`.

use std::io::Read;
use std::sync::Arc;

use paquets::test::TestClient;
use rythme_site::compress::IMMUTABLE;
use rythme_site::server::{Site, app};

async fn client() -> (TestClient, String) {
    let site = Arc::new(Site::load(None));
    let version = site.version.clone();
    (app(site).test().await, version)
}

fn decode(bytes: &[u8], encoding: &str) -> Vec<u8> {
    let mut out = Vec::new();
    match encoding {
        "br" => brotli::Decompressor::new(bytes, 4096).read_to_end(&mut out).unwrap(),
        "gzip" => flate2::read::GzDecoder::new(bytes).read_to_end(&mut out).unwrap(),
        other => panic!("unknown encoding {other}"),
    };
    out
}

/// Pages, Markdown, the texts for assistants, the examples, the scripts of the workers:
/// compressed with Brotli or gzip, whichever the browser accepts, and the same once
/// decoded.
#[tokio::test]
async fn answers_are_compressed_for_the_browsers_that_accept_it() {
    let (client, version) = client().await;
    for path in ["/fr/", "/en/playground/", "/zh/learn/index.md", "/llms-full.txt", "/examples.json", "/sw.js", "/fr/manifest.webmanifest"]
    {
        let plain = client.get(path).await;
        assert_eq!(plain.status(), 200, "{path}");
        assert_eq!(plain.header("content-encoding"), None, "{path}: compressed for a client that did not ask");
        for (accept, encoding) in [("gzip, deflate, br, zstd", "br"), ("gzip, deflate", "gzip")] {
            let res = client.get(path).header("accept-encoding", accept).await;
            assert_eq!(res.header("content-encoding"), Some(encoding), "{path} ({accept})");
            assert!(res.header("vary").is_some_and(|v| v.to_ascii_lowercase().contains("accept-encoding")), "{path}");
            assert!(res.bytes().len() < plain.bytes().len(), "{path}");
            assert_eq!(decode(res.bytes(), encoding), plain.bytes().as_ref(), "{path} ({encoding})");
        }
    }
    let worker = client.get(&format!("/pkg/worker.js?v={version}")).header("accept-encoding", "br").await;
    assert_eq!(worker.status(), 200);
}

/// The files linked with `?v=` of the current version are kept a year; the same files
/// without it, the service worker and the pages are revalidated.
#[tokio::test]
async fn versioned_files_are_kept_a_year() {
    let (client, version) = client().await;
    let css = client.get(&format!("/site.css?v={version}")).await;
    assert_eq!(css.header("cache-control"), Some(IMMUTABLE));
    assert_ne!(client.get("/site.css").await.header("cache-control"), Some(IMMUTABLE));
    assert_ne!(client.get("/site.css?v=0000000000").await.header("cache-control"), Some(IMMUTABLE), "an old version");
    let worker = client.get(&format!("/pkg/worker.js?v={version}")).await;
    assert_eq!(worker.header("cache-control"), Some(IMMUTABLE));
    assert_eq!(client.get("/pkg/worker.js").await.header("cache-control"), Some("no-cache"));
    assert_eq!(client.get("/sw.js").await.header("cache-control"), Some("no-cache"));
    assert_eq!(
        client.get(&format!("/sw.js?v={version}")).await.header("cache-control"),
        Some("no-cache"),
        "the service worker is never kept"
    );
    assert_eq!(client.get(&format!("/fr/?v={version}")).await.header("cache-control"), Some("no-cache"), "nor the pages");
}

/// A page has an `ETag`: asked again with it, `304 Not Modified` and no page.
#[tokio::test]
async fn pages_are_revalidated() {
    let (client, _) = client().await;
    let first = client.get("/en/learn/").await;
    let etag = first.header("etag").expect("an ETag").to_owned();
    assert!(etag.starts_with("W/\""), "{etag}");
    let again = client.get("/en/learn/").header("if-none-match", &etag).await;
    assert_eq!(again.status(), 304);
    assert!(again.bytes().is_empty());
    assert_eq!(again.header("etag"), Some(etag.as_str()));
    assert_eq!(client.get("/fr/learn/").header("if-none-match", &etag).await.status(), 200, "another page");
    assert_eq!(client.get("/en/nothing/").await.header("etag"), None, "not for a page not found");
}

/// The bundles of the browser side, once built (`./build.sh`): their copies compressed
/// at build time, which decode to the files, with the type that lets the browser
/// compile WebAssembly while it downloads; `HEAD` and `304` too.
#[tokio::test]
async fn the_bundles_come_from_their_compressed_copies() {
    let public = std::path::Path::new("public");
    if !public.join("pkg/playground_bg.wasm.br").is_file() {
        eprintln!("public/pkg/ is not built with its copies (./build.sh): nothing to check");
        return;
    }
    let (client, version) = client().await;
    for (file, content_type) in [
        ("/pkg/playground_bg.wasm", "application/wasm"),
        ("/pkg/site_bg.wasm", "application/wasm"),
        ("/pkg/site.js", "text/javascript; charset=utf-8"),
    ] {
        let original = std::fs::read(public.join(&file[1..])).unwrap();
        let url = format!("{file}?v={version}");
        for (accept, encoding) in [("gzip, deflate, br, zstd", "br"), ("gzip", "gzip")] {
            let res = client.get(&url).header("accept-encoding", accept).await;
            assert_eq!(res.status(), 200, "{url}");
            assert_eq!(res.header("content-encoding"), Some(encoding), "{url}");
            assert_eq!(res.header("content-type"), Some(content_type), "{url}");
            assert_eq!(res.header("cache-control"), Some(IMMUTABLE), "{url}");
            assert_eq!(decode(res.bytes(), encoding), original, "{url} ({encoding})");
            let etag = res.header("etag").unwrap().to_owned();
            assert_eq!(client.get(&url).header("accept-encoding", accept).header("if-none-match", &etag).await.status(), 304);
        }
        let plain = client.get(&url).await;
        assert_eq!(plain.header("content-encoding"), None);
        assert_eq!(plain.bytes().as_ref(), &original[..]);
        assert_eq!(plain.header("vary"), Some("Accept-Encoding"));
    }
    let head = client.request(paquets::vitesse::Method::HEAD, "/pkg/playground_bg.wasm").header("accept-encoding", "br").await;
    let copy = std::fs::metadata(public.join("pkg/playground_bg.wasm.br")).unwrap().len();
    assert_eq!(head.header("content-length"), Some(copy.to_string().as_str()));
}

/// The catalogs of the interpreter's messages, once built (`./build.sh`): one per
/// language not compiled into the playground's interpreter, the very texts of the
/// interpreter (`rythme::lang::catalog`), compressed and kept a year at their version.
#[tokio::test]
async fn the_catalogs_are_those_of_the_interpreter() {
    use rythme::lang::{Lang, catalog};
    let public = std::path::Path::new("public");
    if !public.join("pkg/playground_bg.wasm").is_file() {
        eprintln!("public/pkg/ is not built (./build.sh): nothing to check");
        return;
    }
    let files: Vec<String> =
        Lang::all().filter(|l| *l != Lang::En && *l != Lang::Fr).map(|l| format!("/pkg/catalog-{}.txt", l.code())).collect();
    assert_eq!(files, rythme_site::client::CATALOG_FILES, "a catalog for each language of rythme::lang besides English and French");
    let (client, version) = client().await;
    for l in Lang::all().filter(|l| *l != Lang::En && *l != Lang::Fr) {
        let file = format!("/pkg/catalog-{}.txt", l.code());
        let text = catalog(l);
        assert_eq!(std::fs::read_to_string(public.join(&file[1..])).unwrap(), text, "{file}: run ./build.sh again");
        let url = format!("{file}?v={version}");
        let res = client.get(&url).header("accept-encoding", "br").await;
        assert_eq!(res.status(), 200, "{url}");
        assert_eq!(res.header("content-encoding"), Some("br"), "{url}");
        assert_eq!(res.header("content-type"), Some("text/plain; charset=utf-8"), "{url}");
        assert_eq!(res.header("cache-control"), Some(IMMUTABLE), "{url}");
        assert_eq!(decode(res.bytes(), "br"), text.as_bytes(), "{url}");
    }
}

/// The playground fetches what its first run needs while the interpreter downloads: its
/// bundle preloaded, the examples and the worker's script prefetched; the other pages
/// load the site's bundle only.
#[tokio::test]
async fn the_playground_fetches_early_what_its_first_run_needs() {
    let (client, version) = client().await;
    let html = client.get("/fr/playground/").await.text();
    for tag in [
        format!(r#"<link rel="modulepreload" href="/pkg/playground.js?v={version}"/>"#),
        format!(r#"<link rel="preload" href="/pkg/playground_bg.wasm?v={version}" as="fetch" type="application/wasm" crossorigin=""/>"#),
        r#"<link rel="prefetch" href="/examples.json"/>"#.to_owned(),
        format!(r#"<link rel="prefetch" href="/pkg/worker.js?v={version}"/>"#),
    ] {
        assert!(html.contains(&tag), "{tag}");
    }
    // French and English are compiled into the interpreter; another language fetches its
    // catalog, and the page says where it is.
    assert!(!html.contains("catalog-") && !html.contains("data-catalog"));
    let de = client.get("/de/playground/").await.text();
    let catalog = format!("/pkg/catalog-de.txt?v={version}");
    assert!(de.contains(&format!(r#"<link rel="prefetch" href="{catalog}"/>"#)), "{de}");
    assert!(de.contains(&format!(r#"data-catalog="{catalog}""#)));
    assert!(de.contains(r#"id="catalog-note""#));
    let home = client.get("/fr/").await.text();
    assert!(home.contains(&format!(r#"<link rel="preload" href="/pkg/site_bg.wasm?v={version}""#)));
    assert!(!home.contains("playground_bg.wasm") && !home.contains(r#"rel="prefetch""#));
}
