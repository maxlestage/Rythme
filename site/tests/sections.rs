//! The five sections: *Install & tools*, *Reference*, *Packages*, *Rythme at work* and
//! *Exercises*. The reference must list every built-in function and every code of the
//! interpreter, the exercises page every exercise, each opening in the playground with
//! its own file; every page reaches them from its menu and its footer.

use std::sync::Arc;

use paquets::test::TestClient;
use rythme::builtins::BUILTINS;
use rythme::exercise::EXERCISES;
use rythme::help::CODES;
use rythme_site::i18n::Lang;
use rythme_site::routes::{Page, Route};
use rythme_site::sections;
use rythme_site::server::{Site, app};

async fn client() -> TestClient {
    let mut app = app(Arc::new(Site::load(None)));
    app.security(None);
    app.test().await
}

async fn get(client: &TestClient, path: &str) -> String {
    let res = client.get(path).await;
    assert_eq!(res.status(), 200, "{path}");
    res.text()
}

/// The text of a page, its entities read.
fn unescape(text: &str) -> String {
    text.replace("&quot;", "\"").replace("&#x27;", "'").replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
}

/// The text of a page without its tags (the colors of the code), its entities read.
fn text_of(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    unescape(&out)
}

const SECTIONS: [Page; 5] = [Page::Install, Page::Reference, Page::Packages, Page::Work, Page::Exercises];

#[tokio::test]
async fn the_reference_lists_every_function_and_every_code() {
    let client = client().await;
    for lang in Lang::ALL {
        let route = Route::new(lang, Page::Reference);
        let html = get(&client, &route.path()).await;
        let markdown = get(&client, &route.markdown_path()).await;
        let text = text_of(&html);
        assert_eq!(html.matches(r#"<li class="fn" id="fn-"#).count(), BUILTINS.len(), "{lang:?}");
        for b in BUILTINS {
            assert!(html.contains(&format!(r#"id="fn-{}" data-name="{}""#, b.name, b.name)), "{lang:?}: {}", b.name);
            assert!(text.contains(b.sig), "{lang:?}: {}", b.sig);
            // In the language of the page: the documentation `rythme help` gives in it.
            let doc = sections::doc(b, lang);
            let first = doc.split('`').next().unwrap_or(doc).trim();
            assert!(text.contains(first), "{lang:?}: {}: {first}", b.name);
            assert!(markdown.contains(&format!("#### `{}`", b.name)), "{lang:?}: {}", b.name);
        }
        for code in CODES {
            assert!(html.contains(&format!(r#"<article class="code-entry" id="{code}""#)), "{lang:?}: {code}");
            assert!(html.contains(&format!(r##"href="#{code}""##)), "{lang:?}: {code}");
            let (title, explanation) = sections::explanation(code, lang);
            assert!(!title.is_empty() && !explanation.is_empty(), "{lang:?}: {code}");
            assert!(text.contains(&title), "{lang:?}: {code} {title}");
            assert!(markdown.contains(&format!("### {code} ")), "{lang:?}: {code}");
        }
        // The filter: hidden until the browser side shows it, so that the whole list is
        // there without it.
        assert!(html.contains(r#"<div class="filter" id="ref-filter" hidden="""#), "{lang:?}");
        assert!(html.contains(r#"id="ref-search" type="search""#), "{lang:?}");
        assert!(html.contains(r#"id="ref-none" hidden="""#), "{lang:?}");
        let x = lang.sections();
        assert!(x.ref_filter_count.contains("{n}") && x.ref_filter_count.contains("{total}"), "{lang:?}");
        assert!(x.ref_fns_lead.contains("{n}") && x.ref_codes_lead.contains("{n}") && x.ref_generated.contains("{version}"), "{lang:?}");
    }
}

/// The client finds what it looks for in the pages.
#[test]
fn the_browser_side_finds_its_elements() {
    let code = include_str!("../client/src/layout.rs");
    for id in ["ref-filter", "ref-search", "ref-count", "ref-none", "nav-more-toggle"] {
        assert!(code.contains(&format!("\"{id}\"")), "the browser side does not use #{id}");
    }
    for class in [".fn", ".ref-group", ".nav__item--more"] {
        assert!(code.contains(&format!("\"{class}\"")), "the browser side does not use {class}");
    }
}

#[tokio::test]
async fn every_exercise_opens_in_the_playground() {
    let client = client().await;
    for lang in Lang::ALL {
        let route = Route::new(lang, Page::Exercises);
        let html = get(&client, &route.path()).await;
        let markdown = get(&client, &route.markdown_path()).await;
        let interpreter = sections::interpreter(lang);
        for e in EXERCISES {
            let number = e.number();
            assert!(html.contains(&format!(r#"id="exercise-{number}""#)), "{lang:?}: {number}");
            let exo = sections::exercise(e, lang);
            assert!(unescape(&html).contains(&exo.title), "{lang:?}: {}", exo.title);
            // The link holds the file of the exercise in the language of the page, as
            // Share packs it, and runs its tests.
            let link = sections::packed_link(lang, e.text_in(interpreter), true);
            assert!(html.contains(&link.replace('&', "&amp;")), "{lang:?}: {number}");
            let data = link.split("#code=1:").nth(1).unwrap().strip_suffix("&mode=test").unwrap();
            let bytes = rythme_site_client::share::inflate(&rythme_site_client::share::from_base64(data).unwrap(), 1 << 20).unwrap();
            assert_eq!(String::from_utf8(bytes).unwrap(), e.text_in(interpreter), "{lang:?}: {number}");
            assert!(link.len() < 4 * rythme_site_client::share::LINK_MAX, "{lang:?}: {number}: the playground would refuse the link");
            assert!(markdown.contains(&format!("### {number} · ")), "{lang:?}: {number}");
        }
        // Every level has exercises, and the commands name the file in the language.
        for level in 1..=3 {
            assert!(EXERCISES.iter().any(|e| sections::exercise(e, lang).level == level), "{lang:?}: {level}");
        }
        let check = &sections::exercise_commands(lang)[2];
        assert!(check.ends_with(&format!("{}.ry", EXERCISES[6].name_in(interpreter))), "{check}");
    }
}

#[tokio::test]
async fn the_sections_are_in_the_menu_the_footer_and_the_sitemap() {
    let client = client().await;
    for route in Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline))) {
        let html = get(&client, &route.path()).await;
        let x = route.lang.sections();
        // "More" opens them on a computer: hidden until the browser side shows it.
        assert!(
            html.contains(r#"id="nav-more-toggle" aria-expanded="false" aria-controls="nav-more" hidden="""#),
            "{route:?}: the button of More"
        );
        assert!(html.contains(&format!(">{}</button>", unescape(x.nav_more))) || html.contains(x.nav_more), "{route:?}");
        let footer = &html[html.find(r#"<nav class="foot__links""#).expect("the links of the footer")..];
        let footer = &footer[..footer.find("</nav>").unwrap()];
        for page in SECTIONS {
            let path = Route::new(route.lang, page).path();
            // In the menu (under More) and in the footer.
            let menu = html.find(r#"<ul class="nav__sub" id="nav-more">"#).expect("the list of More");
            assert!(html[menu..].starts_with(r#"<ul class="nav__sub""#), "{route:?}");
            assert!(html.contains(&format!(r#"<a class="nav__link" href="{path}""#)) || route.page == page, "{route:?}: {page:?}");
            assert!(footer.contains(&format!(r#"href="{path}""#)), "{route:?}: {page:?}");
        }
        if SECTIONS.contains(&route.page) {
            assert!(html.contains(&format!(r#"href="{}" aria-current="page""#, route.path())), "{route:?}");
            assert!(html.contains("nav__link nav__more-toggle is-active"), "{route:?}: More shows where the page is");
        }
    }
    let sitemap = get(&client, "/sitemap.xml").await;
    for lang in Lang::ALL {
        for page in SECTIONS {
            assert!(sitemap.contains(&format!("<loc>http://localhost{}</loc>", Route::new(lang, page).path())), "{lang:?} {page:?}");
        }
    }
}

/// The descriptions of the new pages fit in what search engines show, and every text
/// with a value to fill has it.
#[test]
fn the_texts_of_the_sections() {
    for lang in Lang::ALL {
        let x = lang.sections();
        for page in SECTIONS {
            let (title, lead) = sections::head(x, page).unwrap();
            assert!(!title.is_empty(), "{lang:?} {page:?}");
            assert!(lead.chars().count() <= 160, "{lang:?} {page:?}: {} characters", lead.chars().count());
        }
        assert!(x.ai_description.chars().count() <= 160, "{lang:?}");
    }
}
