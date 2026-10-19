//! `rythme-site export <dir>`: the whole site as static files, for GitHub Pages. Every
//! page comes from the server itself, through Paquets' in-memory test client
//! (`app.test()`), so both are identical.
//!
//! What a static host cannot do is replaced: `/` picks the language in the browser
//! instead of a redirection, and `404.html` is the French "not found" page. GitHub
//! Pages serves `sw.js` with a short cache rather than `no-cache`: browsers check a
//! service worker past their HTTP cache anyway. The Markdown of the pages
//! (`index.md`) and `llms.txt` are files like the others; GitHub Pages gives them their
//! types (`text/markdown`, `text/plain`).

use std::io;
use std::path::Path;
use std::sync::Arc;

use crate::i18n::Lang;
use crate::routes::{Page, Route, base, link, set_base};
use crate::server::{LANG_COOKIE, Site, app, site_dir};

/// Writes the site into `out`, links behind `base_path` (`""` or `/Rythme`), absolute
/// URLs (canonical, sitemap, Open Graph) on `site_url`. Returns the number of files.
pub async fn export(out: &Path, base_path: &str, site_url: Option<String>) -> io::Result<usize> {
    set_base(base_path);
    let out = std::path::absolute(out)?;
    let public = std::path::absolute(site_dir().join("public"))?;
    if !public.join("rythme.wasm").is_file() {
        eprintln!("warning: public/rythme.wasm is missing: run ./build.sh for the playground");
    }
    let site_url = site_url.filter(|u| !u.trim().is_empty()).unwrap_or_else(|| format!("https://maxlestage.github.io{}", base()));
    // Site::load reads the static files from the site folder to version their links.
    std::env::set_current_dir(public.parent().unwrap_or(Path::new(".")))?;
    let mut app = app(Arc::new(Site::load(Some(site_url))));
    // Static files have no server to protect, and the export asks for every page at
    // once (the rate limit would stop it).
    app.security(None);
    let client = app.test().await;

    let mut written = 0;
    let mut write = |path: &str, bytes: &[u8]| -> io::Result<()> {
        let file = out.join(path.trim_start_matches('/'));
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(file, bytes)?;
        written += 1;
        Ok(())
    };
    let get = |path: String, expected: u16| {
        let client = &client;
        async move {
            let res = client.get(&path).await;
            if res.status().as_u16() != expected {
                return Err(io::Error::other(format!("GET {path}: {} instead of {expected}", res.status())));
            }
            Ok(res.text())
        }
    };

    for route in Route::all() {
        write(&format!("{}index.html", route.path()), get(route.path(), 200).await?.as_bytes())?;
        // The page as Markdown, for AI assistants: `/{lang}/learn/index.md`.
        if route.page.has_markdown() {
            write(&route.markdown_path(), get(route.markdown_path(), 200).await?.as_bytes())?;
        }
    }
    write("404.html", get("/fr/404/".into(), 404).await?.as_bytes())?;
    // The installable app: the offline pages and the manifests of every language, and
    // the service worker at the root of the base path (its scope).
    for lang in Lang::ALL {
        let offline = Route::new(lang, Page::Offline).path();
        write(&format!("{offline}index.html"), get(offline, 200).await?.as_bytes())?;
        let manifest = crate::pwa::manifest_path(lang);
        write(&manifest, get(manifest.clone(), 200).await?.as_bytes())?;
    }
    for file in ["/examples.json", "/sitemap.xml", "/robots.txt", "/sw.js", crate::ai::LLMS_PATH, crate::ai::LLMS_FULL_PATH] {
        write(file, get(file.into(), 200).await?.as_bytes())?;
    }
    write("index.html", language_picker().as_bytes())?;
    // GitHub Pages: serve the files as they are, without Jekyll.
    write(".nojekyll", b"")?;
    copy_dir(&public, &mut write)?;
    Ok(written)
}

/// `/` on a static host: goes to the language chosen with the switch, else the
/// browser's, else French; links every language without JavaScript.
pub fn language_picker() -> String {
    let fr = link("/fr/");
    let links: Vec<String> = Lang::ALL
        .iter()
        .map(|l| format!(r#"<a href="{}" hreflang="{}" lang="{}">{}</a>"#, link(&format!("/{}/", l.code())), l.tag(), l.tag(), l.name()))
        .collect();
    let codes: Vec<&str> = Lang::ALL.iter().map(|l| l.code()).collect();
    format!(
        concat!(
            r#"<!DOCTYPE html><html lang="fr"><head><meta charset="utf-8"/>"#,
            r#"<meta name="viewport" content="width=device-width, initial-scale=1"/>"#,
            r#"<title>Rythme</title><link rel="canonical" href="{fr}"/>"#,
            r#"<script>(function(){{var b="{base}",k=" {list} ",c=(document.cookie.match(/(?:^|; ){cookie}=({alt})(?:;|$)/)||[])[1];"#,
            r#"var l=navigator.languages||[navigator.language||""];"#,
            r#"for(var i=0;i<l.length&&!c;i++){{var x=String(l[i]).slice(0,2).toLowerCase();if(x.length==2&&k.indexOf(" "+x+" ")>=0)c=x}}"#,
            r#"location.replace(b+"/"+(c||"fr")+"/")}})()</script>"#,
            r#"<noscript><meta http-equiv="refresh" content="0; url={fr}"/></noscript>"#,
            r#"</head><body><p>{links}</p></body></html>"#,
        ),
        fr = fr,
        base = base(),
        list = codes.join(" "),
        alt = codes.join("|"),
        cookie = LANG_COOKIE,
        links = links.join(" · "),
    )
}

/// Every file of `from`, at the same place in the export.
fn copy_dir(from: &Path, write: &mut impl FnMut(&str, &[u8]) -> io::Result<()>) -> io::Result<()> {
    fn walk(root: &Path, dir: &Path, write: &mut impl FnMut(&str, &[u8]) -> io::Result<()>) -> io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                walk(root, &path, write)?;
            } else {
                let rel = path.strip_prefix(root).map_err(io::Error::other)?;
                write(&rel.to_string_lossy(), &std::fs::read(&path)?)?;
            }
        }
        Ok(())
    }
    walk(from, from, write)
}
