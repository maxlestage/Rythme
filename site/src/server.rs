//! The server: a `paquets::App` with the pages in every language, the examples for the
//! playground, the sitemap and `robots.txt`, the manifests and the service worker of
//! the installable app (`pwa.rs`). Static files (styles, scripts, the
//! interpreter compiled to WebAssembly) come from `public/`, found by Paquets in the
//! current directory.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use paquets::prelude::*;

use crate::i18n::{Lang, pick_lang};
use crate::pages::{self, Ctx};
use crate::routes::{Page as PageKind, Route, base, link};

/// The styles, scripts and interpreter of `public/`, linked with `?v=` and their version.
pub const ASSETS: [&str; 5] = ["/site.css", "/site.js", "/playground.js", "/playground-worker.js", "/rythme.wasm"];

/// The cookie that remembers the language chosen with the switch (set by `site.js`).
pub const LANG_COOKIE: &str = "rythme-lang";

pub struct Site {
    /// The public address, with the base path (`SITE_URL`); `None`: taken from each
    /// request.
    pub site_url: Option<String>,
    /// Short hash of the styles and scripts, for `?v=` in their links.
    pub version: String,
    /// The service worker, with the version of everything it saves.
    pub worker: String,
}

/// The `site/` folder: `SITE_DIR`, the current directory when it has `public/`, else
/// the crate folder.
pub fn site_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("SITE_DIR") {
        // Absolute: `run` moves into it, and a relative `SITE_DIR` would then point
        // elsewhere.
        return std::path::absolute(&dir).unwrap_or_else(|_| PathBuf::from(dir));
    }
    if Path::new("public").is_dir() {
        return PathBuf::from(".");
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// FNV-1a, enough to tell versions of a file apart.
pub fn version(bytes: &[u8]) -> String {
    let h = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3));
    format!("{h:016x}")[..10].to_owned()
}

impl Site {
    /// Reads the static files of `site_dir()/public` to version their links.
    pub fn load(site_url: Option<String>) -> Site {
        let public = site_dir().join("public");
        let mut bytes = Vec::new();
        for file in ASSETS {
            bytes.extend(std::fs::read(public.join(file.trim_start_matches('/'))).unwrap_or_default());
        }
        let site_url = site_url.map(|u| u.trim().trim_end_matches('/').to_owned()).filter(|u| !u.is_empty());
        let mut site = Site { site_url, version: version(&bytes), worker: String::new() };
        site.worker = crate::pwa::worker(&site);
        site
    }

    /// The public address of the site.
    fn origin(&self, req: &Request) -> String {
        if let Some(url) = &self.site_url {
            return url.clone();
        }
        let header = |name: &str| req.header(name).map(|v| v.split(',').next().unwrap_or("").trim().to_owned());
        let host = header("x-forwarded-host").or_else(|| header("host")).unwrap_or_else(|| "localhost".into());
        if host.is_empty() || !host.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ':')) {
            return format!("http://localhost{}", base());
        }
        let proto = if header("x-forwarded-proto").as_deref() == Some("https") { "https" } else { "http" };
        format!("{proto}://{host}{}", base())
    }

    fn ctx(&self, req: &Request, route: Route) -> Ctx {
        Ctx { route, origin: self.origin(req), version: self.version.clone() }
    }
}

/// The language of a visitor without one in the address: the one they chose with the
/// switch, else their browser's, else French.
fn visitor_lang(req: &Request) -> Lang {
    req.cookie(LANG_COOKIE).and_then(Lang::parse).unwrap_or_else(|| pick_lang(req.header("accept-language").unwrap_or("")))
}

/// `/`: the home page in the visitor's language.
fn root(_site: &Site, req: &Request) -> Response {
    Redirect::to(link(&format!("/{}/", visitor_lang(req).code())))
        .into_response()
        .header("vary", "Accept-Language, Cookie")
        .header("cache-control", "no-cache")
}

/// A page of the site: `/{lang}/` and `/{lang}/{page}/`.
fn page(site: &Site, req: &Request) -> Response {
    let path = req.path();
    let Some(route) = Route::parse(path).filter(|r| r.page != PageKind::NotFound) else {
        return not_found(site, req);
    };
    if !path.ends_with('/') {
        let query = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        return Redirect::permanent(format!("{}/{query}", link(path))).into_response();
    }
    let ctx = site.ctx(req, route);
    pages::respond(|| pages::render(&ctx)).header("cache-control", "no-cache")
}

/// The "not found" page, in the language of the address or of the visitor.
fn not_found(site: &Site, req: &Request) -> Response {
    let lang = Route::parse(req.path()).map_or_else(|| visitor_lang(req), |r| r.lang);
    let ctx = site.ctx(req, Route::new(lang, PageKind::NotFound));
    pages::respond(|| pages::not_found(&ctx))
}

/// `/{lang}/manifest.webmanifest`: the manifest of the app in that language.
fn manifest(site: &Site, req: &Request) -> Response {
    match req.param("lang").and_then(Lang::parse) {
        Some(lang) => Response::new()
            .header("content-type", "application/manifest+json")
            .header("cache-control", "no-cache")
            .send(crate::pwa::manifest(lang)),
        None => not_found(site, req),
    }
}

/// `/sw.js`: the service worker, checked by the browser at each visit. Its own policy
/// lets it fetch the Google Fonts the pages use, to keep them offline.
fn service_worker(site: &Site, _req: &Request) -> Response {
    Response::new()
        .header("content-type", "text/javascript; charset=utf-8")
        .header("cache-control", "no-cache")
        .header(
            "content-security-policy",
            "default-src 'self'; connect-src 'self' https://fonts.googleapis.com https://fonts.gstatic.com; object-src 'none'",
        )
        .send(site.worker.clone())
}

/// `/examples.json`: the programs of the playground's menu.
fn examples_json(_site: &Site, _req: &Request) -> Response {
    Response::new()
        .header("content-type", "application/json; charset=utf-8")
        .header("cache-control", "public, max-age=300")
        .send(crate::examples::json())
}

fn sitemap(site: &Site, req: &Request) -> Response {
    let origin = site.origin(req);
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
    );
    for route in Route::all() {
        xml.push_str(&format!("<url><loc>{origin}{}</loc>", route.path()));
        for lang in Lang::ALL {
            xml.push_str(&format!(
                r#"<xhtml:link rel="alternate" hreflang="{}" href="{origin}{}"/>"#,
                lang.tag(),
                route.with_lang(lang).path()
            ));
        }
        xml.push_str("</url>\n");
    }
    xml.push_str("</urlset>\n");
    Response::new().header("content-type", "application/xml; charset=utf-8").send(xml)
}

fn robots(site: &Site, req: &Request) -> Response {
    Response::new()
        .header("content-type", "text/plain; charset=utf-8")
        .send(format!("User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n", site.origin(req)))
}

/// Routes a handler that only needs the site and the request.
fn route(app: &mut App, path: &str, site: &Arc<Site>, handler: fn(&Site, &Request) -> Response) {
    let site = site.clone();
    app.get(path, move |req: Request| {
        let site = site.clone();
        async move { handler(&site, &req) }
    });
}

/// The application, ready to run or to test.
pub fn app(site: Arc<Site>) -> App {
    let mut app = App::new();
    route(&mut app, "/", &site, root);
    route(&mut app, "/examples.json", &site, examples_json);
    route(&mut app, "/sitemap.xml", &site, sitemap);
    route(&mut app, "/robots.txt", &site, robots);
    route(&mut app, "/sw.js", &site, service_worker);
    route(&mut app, "/:lang/manifest.webmanifest", &site, manifest);
    for path in ["/:lang", "/:lang/:page"] {
        route(&mut app, path, &site, page);
    }
    let fallback = site.clone();
    app.fallback(move |req: Request| {
        let site = fallback.clone();
        async move { not_found(&site, &req) }
    });
    app
}

/// `cargo run`: the server, on `PORT` (Heroku) or 3000.
pub fn run() -> io::Result<()> {
    // Paquets serves `public/` from the current directory.
    let dir = site_dir();
    if dir != Path::new(".") {
        std::env::set_current_dir(&dir)?;
    }
    if !Path::new("public/rythme.wasm").is_file() {
        println!("   public/rythme.wasm is missing: run ./build.sh for the playground");
    }
    app(Arc::new(Site::load(std::env::var("SITE_URL").ok()))).run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_change_with_the_content() {
        assert_eq!(version(b"a").len(), 10);
        assert_ne!(version(b"a"), version(b"b"));
    }
}
