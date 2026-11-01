//! What the server sends compressed, and how long browsers keep the files.
//!
//! - **The files of `public/`** worth it (the bundles of the browser side, `site.css`…)
//!   are compressed once, at their best, when the site is built: `./build.sh` writes
//!   `site.css.br` (Brotli) and `site.css.gz` (gzip) next to `site.css`
//!   (`client/src/bin/precompress.rs`). [`Precompressed`] sends the copy that the
//!   browser accepts (`Accept-Encoding`: Brotli first), with `Content-Encoding` and
//!   `Vary: Accept-Encoding`. A copy is used only if it decodes to its file (checked at
//!   startup) and the file has not changed since (its size and date, at each request):
//!   a file changed after the build is compressed on the fly, like the pages.
//! - **Everything else** (the pages, their Markdown, `llms-full.txt`, the scripts of the
//!   workers, `examples.json`…) is compressed per request by Paquets' `compress`
//!   middleware (Brotli at level 5, or gzip), from 1 KiB up.
//! - **Caching**: a versioned file (`?v=` of the current version, the links the server
//!   writes) is kept a year without being asked again (`immutable`): its address changes
//!   with its content. Other files are revalidated (`ETag`), pages too (`no-cache`).

use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use paquets::prelude::*;
use paquets::vitesse::{Bytes, Middleware};
use rythme_site_client::compression::{ENCODINGS, content_type, copy_path};

use crate::server::Site;

/// The cache of a versioned file: a year, never revalidated.
pub const IMMUTABLE: &str = "public, max-age=31536000, immutable";
/// The cache of the other files of `public/` (as vitesse's static files): revalidated.
const REVALIDATE: &str = "public, max-age=0";

/// A file of `public/` with its compressed copies.
struct Entry {
    path: PathBuf,
    len: u64,
    modified: Option<SystemTime>,
    content_type: &'static str,
    /// The copies that decode to the file, the preferred first: (encoding, bytes).
    copies: Vec<(&'static str, Bytes)>,
}

/// The compressed copies of `public/`, by path (`/pkg/site_bg.wasm`).
#[derive(Default)]
pub struct Precompressed {
    files: HashMap<String, Entry>,
}

impl Precompressed {
    /// Finds the compressed copies of the files of `public`, and keeps those that decode
    /// to their file as it is now.
    pub fn scan(public: &Path) -> Precompressed {
        let mut files = HashMap::new();
        let mut paths = Vec::new();
        walk(public, &mut paths);
        for path in paths {
            let Some(content_type) = content_type(&path) else { continue };
            let Ok(meta) = std::fs::metadata(&path) else { continue };
            let Ok(original) = std::fs::read(&path) else { continue };
            let copies: Vec<(&'static str, Bytes)> = ENCODINGS
                .iter()
                .filter_map(|(ext, encoding)| {
                    let copy = std::fs::read(copy_path(&path, ext)).ok()?;
                    decodes_to(&copy, encoding, &original).then(|| (*encoding, Bytes::from(copy)))
                })
                .collect();
            let Ok(rel) = path.strip_prefix(public) else { continue };
            if copies.is_empty() {
                continue;
            }
            let key = format!("/{}", rel.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/"));
            files.insert(key, Entry { len: meta.len(), modified: meta.modified().ok(), content_type, copies, path });
        }
        Precompressed { files }
    }

    /// The paths that have compressed copies, sorted.
    pub fn paths(&self) -> Vec<&str> {
        let mut paths: Vec<&str> = self.files.keys().map(String::as_str).collect();
        paths.sort_unstable();
        paths
    }

    /// The answer for a file that has a copy the browser accepts, unless the file
    /// changed since the copy was made. `None`: the static files answer as usual.
    fn answer(&self, req: &Request, cache: &'static str) -> Option<Response> {
        let head = *req.method() == Method::HEAD;
        if *req.method() != Method::GET && !head {
            return None;
        }
        let entry = self.files.get(req.path())?;
        let meta = std::fs::metadata(&entry.path).ok()?;
        if meta.len() != entry.len || meta.modified().ok() != entry.modified {
            return None;
        }
        let accept = req.header("accept-encoding").unwrap_or("");
        let (encoding, bytes) = entry.copies.iter().find(|(encoding, _)| accepts(accept, encoding))?;
        let mtime = entry.modified.and_then(|m| m.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs());
        let etag = format!("W/\"{:x}-{mtime:x}-{encoding}\"", entry.len);
        let res = Response::new()
            .header("content-type", entry.content_type)
            .header("content-encoding", *encoding)
            .header("vary", "Accept-Encoding")
            .header("cache-control", cache)
            .header("etag", etag.as_str());
        let fresh = req.header("if-none-match").is_some_and(|tags| tags.split(',').any(|t| t.trim() == "*" || t.trim() == etag));
        Some(if fresh {
            res.status(304)
        } else if head {
            res.header("content-length", bytes.len())
        } else {
            res.send(bytes.clone())
        })
    }

    fn has(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }
}

/// Every file under `dir`.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Whether `copy`, compressed with `encoding`, gives exactly `original` (and nothing
/// more: a damaged copy cannot grow without end).
fn decodes_to(copy: &[u8], encoding: &str, original: &[u8]) -> bool {
    let mut out = Vec::with_capacity(original.len());
    let limit = original.len() as u64 + 1;
    let read = match encoding {
        "br" => brotli::Decompressor::new(copy, 4096).take(limit).read_to_end(&mut out),
        "gzip" => flate2::read::GzDecoder::new(copy).take(limit).read_to_end(&mut out),
        _ => return false,
    };
    read.is_ok() && out == original
}

/// Whether an `Accept-Encoding` header accepts `encoding`: named with a quality above
/// zero, or `*` for gzip (as vitesse's `compress` reads it).
pub fn accepts(header: &str, encoding: &str) -> bool {
    let mut star = false;
    for part in header.split(',') {
        let mut pieces = part.split(';');
        let name = pieces.next().unwrap_or("").trim();
        let q = pieces.find_map(|p| p.trim().strip_prefix("q=")).and_then(|q| q.trim().parse::<f32>().ok()).unwrap_or(1.0);
        if name.eq_ignore_ascii_case(encoding) {
            return q > 0.0;
        }
        if name == "*" {
            star = q > 0.0;
        }
    }
    star && encoding == "gzip"
}

/// Whether the request asks for a versioned file at the current version: one of the
/// files the pages link with `?v=` (`server::VERSIONED`).
pub fn versioned(req: &Request, version: &str) -> bool {
    crate::server::VERSIONED.contains(&req.path()) && req.query("v").is_some_and(|v| v == version)
}

/// The middleware of the files of `public/`: their compressed copies, and the cache of
/// a year for the versioned ones (whoever answers them: a file, or a script written by
/// the server).
pub fn files(site: Arc<Site>, precompressed: Precompressed) -> impl Middleware {
    let precompressed = Arc::new(precompressed);
    move |req: Request, next: Next| {
        let (site, precompressed) = (site.clone(), precompressed.clone());
        async move {
            let immutable = versioned(&req, &site.version);
            if let Some(res) = precompressed.answer(&req, if immutable { IMMUTABLE } else { REVALIDATE }) {
                return res;
            }
            let compressed = precompressed.has(req.path());
            let mut res = next.run(req).await;
            if res.status_code() == StatusCode::OK {
                if immutable {
                    res.set_header("cache-control", IMMUTABLE);
                }
                // The same address has compressed answers for other browsers.
                if compressed && res.get_header("vary").is_none() {
                    res.set_header("vary", "Accept-Encoding");
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_accept_encoding_as_browsers_write_it() {
        let chrome = "gzip, deflate, br, zstd";
        assert!(accepts(chrome, "br") && accepts(chrome, "gzip"));
        assert!(!accepts("gzip, deflate", "br"), "Chrome sends no br over plain HTTP");
        assert!(!accepts("br;q=0, gzip", "br"));
        assert!(accepts("BR;q=0.5", "br"));
        assert!(accepts("*", "gzip") && !accepts("*", "br"));
        assert!(!accepts("", "gzip") && !accepts("identity", "gzip"));
    }

    #[test]
    fn keeps_only_copies_that_decode_to_their_file() {
        let dir = std::env::temp_dir().join(format!("rythme-site-compress-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("pkg")).unwrap();
        let css = "body { color: black }\n".repeat(200);
        std::fs::write(dir.join("site.css"), &css).unwrap();
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        std::io::Write::write_all(&mut gz, css.as_bytes()).unwrap();
        std::fs::write(dir.join("site.css.gz"), gz.finish().unwrap()).unwrap();
        // A copy of an older file: never sent.
        let mut br = Vec::new();
        brotli::BrotliCompress(&mut "old".as_bytes(), &mut br, &Default::default()).unwrap();
        std::fs::write(dir.join("site.css.br"), br).unwrap();
        std::fs::write(dir.join("pkg/app.js"), "let a = 1;\n".repeat(200)).unwrap();
        let found = Precompressed::scan(&dir);
        assert_eq!(found.paths(), ["/site.css"]);
        let entry = &found.files["/site.css"];
        assert_eq!(entry.copies.iter().map(|c| c.0).collect::<Vec<_>>(), ["gzip"]);
        assert_eq!(entry.content_type, "text/css; charset=utf-8");
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// The copy the browser accepts, with its headers; the file itself otherwise, and as
    /// soon as the file changes.
    #[tokio::test]
    async fn sends_the_copy_until_the_file_changes() {
        let dir = std::env::temp_dir().join(format!("rythme-site-serve-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let css = "body { color: black }\n".repeat(200);
        std::fs::write(dir.join("site.css"), &css).unwrap();
        let mut br = Vec::new();
        brotli::BrotliCompress(&mut css.as_bytes(), &mut br, &Default::default()).unwrap();
        std::fs::write(dir.join("site.css.br"), &br).unwrap();
        let site = Arc::new(Site { site_url: None, version: "abc".into(), worker: String::new(), worker_wasm: String::new() });
        let mut app = paquets::vitesse::App::new();
        app.middleware(files(site, Precompressed::scan(&dir)));
        app.middleware(paquets::vitesse::ServeDir::new(&dir));
        let client = paquets::vitesse::test::TestClient::new(app);

        let res = client.get("/site.css?v=abc").header("accept-encoding", "gzip, deflate, br").await;
        assert_eq!(res.status(), 200);
        assert_eq!(res.header("content-encoding"), Some("br"));
        assert_eq!(res.header("content-type"), Some("text/css; charset=utf-8"));
        assert_eq!(res.header("vary"), Some("Accept-Encoding"));
        assert_eq!(res.header("cache-control"), Some(IMMUTABLE));
        assert_eq!(res.bytes().as_ref(), &br[..]);
        let etag = res.header("etag").unwrap().to_owned();
        let again = client.get("/site.css?v=abc").header("accept-encoding", "br").header("if-none-match", &etag).await;
        assert_eq!(again.status(), 304);
        let head = client.request(Method::HEAD, "/site.css").header("accept-encoding", "br").await;
        assert_eq!(head.header("content-length"), Some(br.len().to_string().as_str()));
        assert_eq!(head.header("cache-control"), Some(REVALIDATE), "another version: revalidated");

        // A browser without Brotli: the file, which could be compressed for others.
        let plain = client.get("/site.css?v=abc").header("accept-encoding", "gzip").await;
        assert_eq!(plain.header("content-encoding"), None);
        assert_eq!(plain.header("vary"), Some("Accept-Encoding"));
        assert_eq!(plain.header("cache-control"), Some(IMMUTABLE));
        assert_eq!(plain.text(), css);

        // Changed after the build: the copy is old, the file is sent.
        let changed = format!("{css}/* deployed */\n");
        std::fs::write(dir.join("site.css"), &changed).unwrap();
        let res = client.get("/site.css").header("accept-encoding", "br").await;
        assert_eq!(res.header("content-encoding"), None);
        assert_eq!(res.text(), changed);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
