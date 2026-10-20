//! The installable app (a Progressive Web App): the web app manifest of each language,
//! and the service worker that keeps the whole site, the playground and its interpreter
//! available offline. Both are generated with the current version of the files and
//! behind the base path, so the server and the static export serve the same ones.
//!
//! The manifest of `/fr/` is `/fr/manifest.webmanifest`: the app opens in the language
//! it was installed from. Its URLs are relative to it, so they hold under any base path;
//! only `id`, resolved against the origin, is written with the base path.

use crate::i18n::Lang;
use crate::json::{Json, js_string, s};
use crate::pages::{self, Ctx};
use crate::routes::{Page, Route, base, link};
use crate::server::{ASSETS, Site, site_dir, version};

/// The paper of the light and dark themes (`site.css`): the splash screen and the bar of
/// the installed app.
pub use crate::themes::{DARK, LIGHT};

/// The icons of `public/`, made from `assets/logo.svg` by `tests/pwa/images.js`:
/// (path, sizes, purpose). The maskable one keeps the beats in the central circle.
pub const ICONS: [(&str, &str, &str); 3] = [
    ("/icons/icon-192.png", "192x192", "any"),
    ("/icons/icon-512.png", "512x512", "any"),
    ("/icons/icon-maskable-512.png", "512x512", "maskable"),
];
pub const APPLE_ICON: &str = "/icons/apple-touch-icon.png";
pub const FAVICON: &str = "/favicon.ico";
pub const LOGO: &str = "/assets/logo.svg";

/// The screenshots of the install dialog, in `public/screenshots/{lang}-{page}-{form}.png`
/// (taken by `tests/pwa/images.js`): (page, form factor, size).
pub const SCREENSHOTS: [(Page, &str, &str); 4] = [
    (Page::Playground, "narrow", "412x915"),
    (Page::Home, "narrow", "412x915"),
    (Page::Home, "wide", "1280x800"),
    (Page::Playground, "wide", "1280x800"),
];

/// Where a page of `lang` finds its manifest.
pub fn manifest_path(lang: Lang) -> String {
    format!("/{}/manifest.webmanifest", lang.code())
}

/// A screenshot of the manifest.
pub fn screenshot_path(lang: Lang, page: Page, form: &str) -> String {
    let name = if page == Page::Home { "home" } else { page.slug() };
    format!("/screenshots/{}-{name}-{form}.png", lang.code())
}

/// A path of the site, relative to the manifests (which are one folder down).
fn rel(path: &str) -> String {
    format!("..{path}")
}

/// The manifest of the app in `lang`.
pub fn manifest(lang: Lang) -> String {
    let t = lang.texts();
    let page = |p: Page| rel(&Route::new(lang, p).path());
    let icon = |(src, sizes, purpose): (&str, &str, &str)| {
        Json::Obj(vec![("src", s(rel(src))), ("sizes", s(sizes)), ("type", s("image/png")), ("purpose", s(purpose))])
    };
    let mut icons: Vec<Json> = ICONS.into_iter().map(icon).collect();
    icons.push(Json::Obj(vec![("src", s(rel(LOGO))), ("sizes", s("any")), ("type", s("image/svg+xml")), ("purpose", s("any"))]));
    let screenshots = SCREENSHOTS.map(|(p, form, sizes)| {
        let label = if p == Page::Home { t.app_shots[0] } else { t.app_shots[1] };
        Json::Obj(vec![
            ("src", s(rel(&screenshot_path(lang, p, form)))),
            ("sizes", s(sizes)),
            ("type", s("image/png")),
            ("form_factor", s(form)),
            ("label", s(label)),
        ])
    });
    let shortcuts = [
        (Page::Playground, t.nav_playground, t.pg_lead),
        (Page::Examples, t.nav_examples, t.ex_lead),
        (Page::Learn, t.nav_learn, t.learn_lead),
    ]
    .map(|(p, name, description)| {
        Json::Obj(vec![("name", s(name)), ("description", s(description)), ("url", s(page(p))), ("icons", Json::Arr(vec![icon(ICONS[0])]))])
    });
    let manifest = Json::Obj(vec![
        ("name", s("Rythme")),
        ("short_name", s("Rythme")),
        ("description", s(t.app_description)),
        ("lang", s(lang.tag())),
        ("dir", s("ltr")),
        // One app in every language: it opens in the language of the manifest it was
        // installed from (or last updated from).
        ("id", s(link("/"))),
        ("start_url", s(page(Page::Home))),
        ("scope", s(rel("/"))),
        ("display", s("standalone")),
        ("display_override", Json::Arr(vec![s("standalone"), s("minimal-ui")])),
        ("theme_color", s(LIGHT)),
        ("background_color", s(LIGHT)),
        ("categories", Json::Arr(vec![s("education"), s("developer tools"), s("developer")])),
        ("icons", Json::Arr(icons)),
        ("screenshots", Json::Arr(screenshots.into())),
        ("shortcuts", Json::Arr(shortcuts.into())),
        // A `.ry` file opened from the system goes to the playground (`launchQueue`).
        (
            "file_handlers",
            Json::Arr(vec![Json::Obj(vec![
                ("action", s(page(Page::Playground))),
                ("accept", Json::Obj(vec![("text/x-rythme", Json::Arr(vec![s(".ry")]))])),
                ("icons", Json::Arr(vec![icon(ICONS[0])])),
                ("launch_type", s("single-client")),
            ])]),
        ),
        // Text shared with the app opens in the playground: `?title=…&text=…&url=…`.
        (
            "share_target",
            Json::Obj(vec![
                ("action", s(page(Page::Playground))),
                ("method", s("GET")),
                ("enctype", s("application/x-www-form-urlencoded")),
                ("params", Json::Obj(vec![("title", s("title")), ("text", s("text")), ("url", s("url"))])),
            ]),
        ),
    ]);
    manifest.to_string()
}

/// Every URL the service worker saves when it installs: every page in every language
/// with the offline pages, the styles and the browser side (with the interpreter) at
/// their current version, the playground's worker, the examples, the manifests and the
/// icons. Only files that exist (`public/pkg/` is built by `./build.sh`).
pub fn precache(site: &Site) -> Vec<String> {
    let public = site_dir().join("public");
    let exists = |path: &str| public.join(path.trim_start_matches('/')).is_file();
    let mut urls: Vec<String> = Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline))).map(|r| r.href()).collect();
    urls.extend(ASSETS.iter().filter(|a| exists(a)).map(|a| format!("{}?v={}", link(a), site.version)));
    if exists(crate::client::PAGE_FILES[3]) {
        urls.push(crate::client::worker_url(&site.version));
    }
    urls.push(link("/examples.json"));
    // The whole language for AI assistants, linked by the AI page: offline too.
    urls.extend([crate::ai::LLMS_PATH, crate::ai::LLMS_FULL_PATH].map(link));
    urls.extend(Lang::ALL.map(|l| link(&manifest_path(l))));
    let files = ICONS.iter().map(|i| i.0).chain([APPLE_ICON, FAVICON, LOGO]);
    urls.extend(files.filter(|f| exists(f)).map(link));
    urls
}

/// The service worker (`src/sw.js` with its constants). Its version is a hash of
/// everything it saves (the pages as rendered, the files, the manifests): each
/// deployment that changes one of them installs a new worker, which shows the visitors
/// the "new version" banner and, once activated, deletes the old caches.
pub fn worker(site: &Site) -> String {
    let origin = site.site_url.clone().unwrap_or_else(|| format!("http://localhost{}", base()));
    let mut parts: Vec<Vec<u8>> = vec![
        site.version.clone().into_bytes(),
        crate::examples::json().into_bytes(),
        crate::ai::llms_txt(&origin).into_bytes(),
        crate::ai::LLMS_FULL.as_bytes().to_vec(),
    ];
    for lang in Lang::ALL {
        parts.push(manifest(lang).into_bytes());
        for page in Page::ALL.into_iter().chain(Page::EXTRA) {
            let ctx = Ctx { route: Route::new(lang, page), origin: origin.clone(), version: site.version.clone() };
            parts.push(pages::render(&ctx).render().into_bytes());
        }
    }
    let public = site_dir().join("public");
    for file in ICONS.iter().map(|i| i.0).chain([APPLE_ICON, FAVICON, LOGO]) {
        parts.push(std::fs::read(public.join(file.trim_start_matches('/'))).unwrap_or_default());
    }
    let list = |urls: &[String]| format!("[{}]", urls.iter().map(|u| js_string(u)).collect::<Vec<_>>().join(", "));
    let offline: Vec<String> =
        Lang::ALL.iter().map(|l| format!("{}: {}", js_string(l.code()), js_string(&Route::new(*l, Page::Offline).href()))).collect();
    include_str!("sw.js")
        .replace("__VERSION__", &js_string(&version(&parts.concat())))
        .replace("__BASE__", &js_string(base()))
        .replace("__PRECACHE__", &list(&precache(site)))
        .replace("__OFFLINE__", &format!("{{ {} }}", offline.join(", ")))
        .replace("__FONTS__", &js_string(pages::FONTS))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_urls_are_relative_to_the_manifest() {
        let json = manifest(Lang::En);
        assert!(json.contains(r#""start_url":"../en/""#), "{json}");
        assert!(json.contains(r#""scope":"../""#));
        assert!(json.contains(r#""id":"/""#));
        assert!(json.contains(r#""url":"../en/playground/""#));
        assert!(json.contains(r#""src":"../screenshots/en-home-wide.png""#));
    }
}
