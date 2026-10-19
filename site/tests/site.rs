//! The whole site through Paquets' in-memory test client, and every piece of Rythme
//! code it shows through the real interpreter.

use std::path::Path;
use std::sync::Arc;

use paquets::test::TestClient;
use rythme_site::examples::EXAMPLES;
use rythme_site::highlight::{ERROR_WORDS, HELP_WORDS, WARNING_WORDS};
use rythme_site::i18n::Lang;
use rythme_site::pages::{TOUR_STEPS, steps_link};
use rythme_site::routes::{Page, Route};
use rythme_site::server::{Site, app};

async fn client() -> TestClient {
    app(Arc::new(Site::load(None))).test().await
}

#[tokio::test]
async fn every_page_renders_in_both_languages() {
    let client = client().await;
    for route in Route::all() {
        let path = route.path();
        let res = client.get(&path).header("host", "example.com").await;
        assert_eq!(res.status(), 200, "{path}");
        let html = res.text();
        let code = route.lang.tag();
        assert!(html.starts_with(&format!("<!DOCTYPE html><html lang=\"{code}\">")), "{path}");
        assert!(html.contains(&format!(r#"<link rel="canonical" href="http://example.com{path}"/>"#)), "{path}");
        for lang in Lang::ALL {
            let other = route.with_lang(lang).path();
            // The language switch, and the alternate link for search engines.
            assert!(html.contains(&format!(r#"href="{other}" hreflang="{}""#, lang.tag())), "{path}: {other}");
            assert!(html.contains(&format!(r#"hreflang="{}" href="http://example.com{other}""#, lang.tag())), "{path}");
        }
        // The burger menu and the navigation.
        assert!(html.contains(r#"class="burger""#), "{path}");
        assert!(html.contains(r#"aria-controls="nav""#), "{path}");
        assert!(html.contains("/site.js?v="), "{path}");
        let csp = res.header("content-security-policy").unwrap_or_default().to_owned();
        assert!(csp.contains("'wasm-unsafe-eval'"), "{path}: {csp}");
        assert!(csp.contains("'sha256-"), "{path}: {csp}");
    }
}

#[tokio::test]
async fn the_pages_hold_their_content() {
    let client = client().await;
    for lang in Lang::ALL {
        let t = lang.texts();
        let page = |p| async move { client.get(&Route::new(lang, p).path()).await.text() };
        let home = page(Page::Home).await;
        for item in &t.why {
            assert!(home.contains(&item.title.replace('\'', "&#x27;")), "{}", item.title);
        }
        assert!(home.contains(r#"id="install""#));
        let examples = page(Page::Examples).await;
        for e in EXAMPLES {
            assert!(examples.contains(&format!(r#"id="{}""#, e.name)), "{}", e.name);
            assert!(examples.contains(&format!("#example={}", e.name)), "{}", e.name);
        }
        let playground = page(Page::Playground).await;
        assert!(playground.contains(r#"id="source""#));
        assert!(playground.contains(r#"id="format""#) && playground.contains(t.pg_format), "the Format button");
        assert!(playground.contains(r#"id="steps""#) && playground.contains(t.pg_steps), "the Steps button");
        // Share: its button (shown by playground.js), the box of the link with its
        // labelled field, the notice of a program opened from a link and its button.
        let attr = |s: &str| s.replace('\'', "&#x27;");
        assert!(playground.contains(r#"id="share""#) && playground.contains(&attr(t.pg_share_title)), "the Share button");
        assert!(playground.contains(r#"aria-controls="share-box""#), "{lang:?}");
        assert!(
            playground.contains(r#"id="share-box""#) && playground.contains(r#"<label class="share__label" for="share-link">"#),
            "{lang:?}"
        );
        assert!(
            playground.contains(&attr(t.pg_share_link))
                && playground.contains(&attr(t.pg_share_copy))
                && playground.contains(&attr(t.pg_share_close)),
            "{lang:?}"
        );
        assert!(
            playground.contains(r#"id="shared-note""#)
                && playground.contains(&attr(t.pg_shared_note))
                && playground.contains(&attr(t.pg_shared_back)),
            "{lang:?}"
        );
        for text in [t.pg_share_copied, t.pg_share_shared, t.pg_share_select, t.pg_share_too_long, t.pg_share_broken] {
            assert!(playground.contains(&attr(text).replace('"', "&quot;")), "{lang:?}: {text}");
        }
        assert!(t.pg_share_too_long.contains("{n}") && t.pg_share_too_long.contains("{max}"), "{lang:?}");
        assert!(playground.contains("/playground.js?v="));
        assert!(playground.contains(r#"data-wasm="/rythme.wasm?v="#));
        // The steps of a calculation, one at a time: hidden until Steps (or a link with
        // `&mode=steps`), with its buttons, its labelled slider, its keys and a live
        // region, in the language of the page.
        assert!(playground.contains(r#"class="stepper output--live" id="stepper" hidden="""#), "{lang:?}");
        assert!(playground.contains(&format!(r#"id="step-prev">← {}</button>"#, attr(t.pg_steps_prev))), "{lang:?}");
        assert!(playground.contains(&format!(r#"id="step-next">{} →</button>"#, attr(t.pg_steps_next))), "{lang:?}");
        assert!(
            playground
                .contains(&format!(r#"type="range" id="step-range" min="1" max="1" value="1" aria-label="{}""#, attr(t.pg_steps_slider))),
            "{lang:?}"
        );
        assert!(playground.contains(&attr(t.pg_steps_keys)) && playground.contains(&attr(t.pg_steps_printed)), "{lang:?}");
        assert!(playground.contains(r#"id="step-live" aria-live="polite" aria-atomic="true""#), "{lang:?}");
        assert!(playground.contains(r#"id="step-now" tabindex="0""#) && playground.contains(r#"id="step-error" hidden="""#), "{lang:?}");
        for text in [t.pg_steps_of, t.pg_steps_none] {
            assert!(playground.contains(&attr(text).replace('"', "&quot;")), "{lang:?}: {text}");
        }
        assert!(t.pg_steps_of.contains("{n}") && t.pg_steps_of.contains("{total}"), "{lang:?}");
        // The Seed option next to Run: hidden until playground.js shows it, a labelled
        // field for digits described by its hint, the dice, and the seed in the head of
        // the output, in the language of the page.
        assert!(playground.contains(r#"<details class="seed" id="seed-option" hidden="">"#), "{lang:?}");
        assert!(
            playground.contains(&format!(r#"<summary class="btn seed__toggle" title="{}">"#, attr(t.pg_seed_title)))
                && playground.contains(&format!(r#"<span class="seed__name">{}</span>"#, attr(t.pg_seed))),
            "{lang:?}"
        );
        assert!(playground.contains(&format!(r#"<label class="seed__label" for="seed">{}</label>"#, attr(t.pg_seed_label))), "{lang:?}");
        assert!(
            playground.contains(&format!(
                r#"id="seed" type="text" inputmode="numeric" pattern="[0-9]*" maxlength="15" spellcheck="false" autocomplete="off" placeholder="{}" aria-describedby="seed-hint""#,
                attr(t.pg_seed_random)
            )),
            "{lang:?}"
        );
        assert!(playground.contains(&format!(r#"id="seed-hint">{}</p>"#, attr(t.pg_seed_hint))), "{lang:?}");
        assert!(playground.contains(&format!("<span>{}</span></button>", attr(t.pg_seed_new))), "{lang:?}");
        assert!(playground.contains(r#"id="seed-used" hidden="""#), "{lang:?}");
        assert!(t.pg_seed_used.contains("{seed}") && playground.contains(&attr(t.pg_seed_used)), "{lang:?}");
        assert!(t.pg_seed_hint.contains("--seed 42") && t.pg_seed_hint.contains("check_all"), "{lang:?}");
        let learn = page(Page::Learn).await;
        assert_eq!(learn.matches("class=\"tour__try\"").count(), t.tour.len());
        // One step of the tour also opens on the steps of a calculation.
        let (n, expr) = TOUR_STEPS;
        let steps = steps_link(lang, t.tour[n].code, expr).replace('&', "&amp;");
        assert!(steps.contains("#code=fn%20fib(") && steps.ends_with("&amp;mode=steps&amp;expr=fib(4)"), "{steps}");
        assert!(learn.contains(&format!(r#"class="tour__try tour__try--steps" href="{steps}""#)), "{lang:?}: {steps}");
        assert!(learn.contains(&format!("→ {}: {expr}", attr(t.learn_steps))), "{lang:?}");
        assert_eq!(learn.matches("tour__try--steps").count(), 1);
        // The texts that playground.js writes, in the language of the page.
        assert!(playground.contains("data-ui=\""));
        assert!(playground.contains(&t.pg_loading.replace('\'', "&#x27;")), "{lang:?}");
        assert!(playground.contains(&format!(r#"data-lang="{}""#, lang.code())));
        let teach = page(Page::Teach).await;
        assert!(teach.contains("<table>"));
        // Without a guide of its own: the English one, marked as English, and a note.
        match t.teach_note {
            Some(note) => {
                assert!(teach.contains(&note.replace('\'', "&#x27;")), "{lang:?}");
                assert!(teach.contains(r#"<article class="prose" lang="en">"#), "{lang:?}");
            }
            None => assert!(!teach.contains("teach-note") && teach.contains(r#"<article class="prose">"#), "{lang:?}"),
        }
    }
}

#[tokio::test]
async fn redirects_and_not_found() {
    let client = client().await;
    let res = client.get("/").header("accept-language", "en-US,en;q=0.9").await;
    assert_eq!(res.status(), 302);
    assert_eq!(res.header("location"), Some("/en/"));
    // Every language of the site, from the browser's; French when none is known.
    for (header, lang) in [
        ("de", "de"),
        ("es-MX,es;q=0.9,en;q=0.5", "es"),
        ("pt-BR", "pt"),
        ("zh-CN,zh;q=0.9", "zh"),
        ("zh-Hant-TW", "zh"),
        ("it, en;q=0.4", "en"),
        ("it, ja", "fr"),
        ("", "fr"),
    ] {
        let res = client.get("/").header("accept-language", header).await;
        assert_eq!(res.header("location"), Some(format!("/{lang}/").as_str()), "{header}");
    }
    let res = client.get("/").header("accept-language", "en").header("cookie", "rythme-lang=zh").await;
    assert_eq!(res.header("location"), Some("/zh/"));
    // The language chosen with the switch wins over the browser's.
    let res = client.get("/").header("accept-language", "fr").header("cookie", "rythme-lang=en").await;
    assert_eq!(res.header("location"), Some("/en/"));
    let res = client.get("/en/learn").await;
    assert_eq!(res.status(), 301);
    assert_eq!(res.header("location"), Some("/en/learn/"));
    for path in ["/en/nothing/", "/fr/learn/more/", "/it/", "/nothing"] {
        let res = client.get(path).await;
        assert_eq!(res.status(), 404, "{path}");
        assert!(res.text().contains("noindex"), "{path}");
    }
    assert!(client.get("/en/nothing/").await.text().contains("<html lang=\"en\">"));
    assert!(client.get("/zh/nothing/").await.text().contains("<html lang=\"zh-Hans\">"));
}

#[tokio::test]
async fn examples_json_sitemap_and_robots() {
    let client = client().await;
    let res = client.get("/examples.json").await;
    assert_eq!(res.status(), 200);
    let json = res.text();
    for e in EXAMPLES {
        assert!(json.contains(&format!("\"{}\":", e.name)), "{}", e.name);
    }
    let sitemap = client.get("/sitemap.xml").header("host", "example.com").await.text();
    assert_eq!(sitemap.matches("<loc>").count(), Route::all().count());
    assert!(client.get("/robots.txt").header("host", "example.com").await.text().contains("http://example.com/sitemap.xml"));
}

/// Every top-level program of `examples/` is on the examples page and in the
/// playground's menu.
#[test]
fn every_example_is_listed() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../examples");
    let mut files: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| {
            let name = e.ok()?.file_name().into_string().ok()?;
            name.strip_suffix(".ry").map(str::to_owned)
        })
        .collect();
    files.sort();
    let mut listed: Vec<String> = EXAMPLES.iter().map(|e| e.name.to_owned()).collect();
    listed.sort();
    assert_eq!(files, listed);
}

/// Runs `code` like the playground does; returns the output and whether it failed.
fn run(code: &str, lang: Lang, test: bool) -> (String, bool) {
    use rythme::driver::Driver;
    use rythme::interp::Mode;
    rythme::lang::set(interpreter_lang(lang));
    rythme::io::capture();
    let mut driver = Driver::new();
    driver.color = false;
    let mut ok = false;
    if let Some(m) = driver.load_source("main.ry", code, None, Path::new(".")) {
        let mode = if test { Mode::Test } else { Mode::Run };
        ok = driver.execute(&m, mode).is_ok() && (!test || driver.run_tests());
    }
    let out = rythme::io::take_captured();
    let failed = !ok || ERROR_WORDS.iter().any(|w| out.contains(&format!("{w}[")));
    (out, failed)
}

/// The interpreter's language that the playground of `lang` asks for (its position in
/// `rythme::lang::LANGUAGES`).
fn interpreter_lang(lang: Lang) -> rythme::lang::Lang {
    rythme::lang::Lang::all().nth(lang.interpreter()).expect("a language of the interpreter")
}

/// A warning in the output, in any language.
fn warns(out: &str) -> bool {
    WARNING_WORDS.iter().any(|w| out.contains(&format!("{w}[")))
}

/// Each language of the site is the interpreter's language of the same code, at the
/// position the playground gives it; the site has every language of the interpreter.
#[test]
fn the_languages_are_the_interpreters() {
    for lang in Lang::ALL {
        assert_eq!(interpreter_lang(lang).code(), lang.code(), "{lang:?}");
    }
    assert_eq!(rythme::lang::Lang::all().count(), Lang::ALL.len());
    // `playground.js` and its worker pass the language by its position.
    let codes: Vec<String> = rythme::lang::Lang::all().map(|l| format!("{:?}", l.code())).collect();
    let list = format!("[{}]", codes.join(", "));
    for file in ["public/playground.js", "public/playground-worker.js"] {
        let js = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(file)).unwrap();
        assert!(js.contains(&list), "{file}: {list}");
    }
}

/// The words that start the interpreter's messages, in every language: the site colors
/// them (`highlight.rs`), the playground too (`playground.js`).
#[test]
fn every_word_of_the_messages_is_known() {
    let js = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("public/playground.js")).unwrap();
    for lang in rythme::lang::Lang::all() {
        for (en, fr, words) in
            [("error", "erreur", &ERROR_WORDS[..]), ("warning", "avertissement", &WARNING_WORDS[..]), ("help", "aide", &HELP_WORDS[..])]
        {
            // Without `rythme::lang::set`: the other tests run at the same time.
            let word = match lang {
                rythme::lang::Lang::En => en,
                rythme::lang::Lang::Fr => fr,
                other => rythme::lang::translation(other, en).unwrap_or(en),
            };
            assert!(words.contains(&word), "{}: {word}", lang.code());
            let list = words.join("|");
            assert!(js.contains(&format!("\"{list}\"")), "playground.js: {list}");
        }
    }
}

#[test]
fn every_piece_of_code_runs() {
    for lang in Lang::ALL {
        let t = lang.texts();
        let (out, failed) = run(t.hero_code, lang, false);
        assert!(!failed, "{lang:?} hero: {out}");
        assert_eq!(out.trim(), "20", "{lang:?}: the hero shows 20 as its output");
        let (out, failed) = run(t.pg_welcome_code, lang, false);
        assert!(!failed && !warns(&out), "{lang:?} welcome: {out}");
        for step in &t.tour {
            let test = step.code.contains("test \"");
            let (out, failed) = run(step.code, lang, test);
            assert!(!failed, "{lang:?} {}: {out}", step.title);
            // The samples of the site are good examples: not even a warning.
            assert!(!warns(&out), "{lang:?} {}: {out}", step.title);
        }
        // The error of the home page is the interpreter's own message, in the language
        // of the page.
        let (out, failed) = run(t.errors_code, lang, false);
        assert!(failed);
        let shown: Vec<&str> = t.errors_message.lines().collect();
        let real: Vec<&str> = out.trim_matches(['\u{1}', '\u{2}', '\n']).lines().collect();
        assert_eq!(shown, real, "{lang:?}");
    }
}

/// The theme picker of every page, and the inline script that applies the theme before
/// the first paint, allowed by its hash in the Content-Security-Policy.
#[tokio::test]
async fn the_theme_picker_and_its_script() {
    use paquets::securite::headers::ContentSecurityPolicy;
    use rythme_site::themes::{self, THEMES};

    let client = client().await;
    let script = format!("<script>{}</script>", themes::head_script());
    let hash = ContentSecurityPolicy::hashes_for_inline_scripts(&script).pop().expect("a hash");
    let routes = Route::all().chain(Lang::ALL.map(|l| Route::new(l, Page::Offline)));
    for (path, lang) in routes.map(|r| (r.path(), r.lang)).chain([("/en/nothing/".to_owned(), Lang::En)]) {
        let res = client.get(&path).await;
        let html = res.text();
        let t = lang.texts();
        // The script runs in the head, after the theme-color tags it updates.
        let at = html.find(&script).unwrap_or_else(|| panic!("{path}: the theme script"));
        assert!(at < html.find("<body").unwrap(), "{path}");
        assert!(html.find(r#"<meta name="theme-color""#).unwrap() < at, "{path}");
        assert_eq!(html.matches("<script>").count(), 1, "{path}: one inline script");
        let csp = res.header("content-security-policy").unwrap_or_default();
        assert!(csp.contains(&hash), "{path}: {csp}");
        // Every script that runs is allowed by its hash; the structured data (JSON-LD) is
        // not a script that runs: the policy leaves it out, and the browser reads it.
        let (scripts, data) = split_json_ld(&html);
        for inline in ContentSecurityPolicy::hashes_for_inline_scripts(&scripts) {
            assert!(csp.contains(&inline), "{path}: {inline}");
        }
        for block in data {
            let hash = ContentSecurityPolicy::hashes_for_inline_scripts(&format!("<script>{block}</script>")).pop().unwrap();
            assert!(!csp.contains(&hash), "{path}: the JSON-LD needs no hash");
        }
        let scripts_src = csp.split(';').find(|d| d.trim().starts_with("script-src")).unwrap_or_default();
        assert!(!scripts_src.contains("unsafe-inline"), "{path}: {csp}");
        // The picker: hidden without JavaScript, a button for computers, a fieldset of
        // radio buttons with their labels in the language of the page.
        assert!(html.contains(r#"<li class="nav__item--theme" hidden="">"#), "{path}");
        assert!(html.contains(r#"id="theme-toggle" aria-expanded="false" aria-controls="theme-panel""#), "{path}");
        assert!(html.contains(&format!(r#"title="{}""#, t.theme_title)), "{path}");
        let legend = format!(r#"<fieldset class="theme__panel" id="theme-panel"><legend class="theme__legend">{}</legend>"#, t.theme_label);
        assert!(html.contains(&legend), "{path}");
        assert_eq!(html.matches(r#"type="radio" name="theme""#).count(), THEMES.len(), "{path}");
        assert_eq!(html.matches(" checked=\"\"").count(), 1, "{path}: Auto is chosen by default");
        assert!(html.contains(r##"value="auto" id="theme-auto" data-color="#eef1f6" checked="" data-dark="#0e1118""##), "{path}");
        for theme in THEMES {
            let id = theme.id;
            assert!(html.contains(&format!(r#"<label class="theme__option" for="theme-{id}">"#)), "{path}: {id}");
            assert!(html.contains(&format!(r#"value="{id}" id="theme-{id}" data-color="{}""#, theme.color)), "{path}: {id}");
            assert!(html.contains(&format!(r#"<span class="theme__name">{}</span>"#, theme.name(t))), "{path}: {id}");
        }
    }
}

/// The page without its JSON-LD blocks, and the text of those blocks.
fn split_json_ld(html: &str) -> (String, Vec<String>) {
    const OPEN: &str = r#"<script type="application/ld+json">"#;
    let (mut rest, mut scripts, mut data) = (html, String::new(), Vec::new());
    while let Some(start) = rest.find(OPEN) {
        scripts.push_str(&rest[..start]);
        let end = rest[start..].find("</script>").expect("a closed JSON-LD block") + start;
        data.push(rest[start + OPEN.len()..end].to_owned());
        rest = &rest[end + "</script>".len()..];
    }
    scripts.push_str(rest);
    (scripts, data)
}

/// The calculation that a step of the tour opens on (`&mode=steps&expr=…`) is shown
/// step by step, as the playground does it (`rythme_run`, mode 5): the expression
/// computed after the definitions of the code of that step, alone.
#[test]
fn the_steps_of_the_tour_are_shown() {
    use rythme::driver::Driver;
    let (n, expr) = TOUR_STEPS;
    for lang in Lang::ALL {
        let t = lang.texts();
        rythme::lang::set(interpreter_lang(lang));
        rythme::io::capture();
        let mut driver = Driver::new();
        driver.color = false;
        let whole = format!("{}\n{expr}\n", t.tour[n].code.trim_end());
        let m = driver.load_source("main.ry", &whole, None, Path::new(".")).expect("the code of the step loads");
        rythme::steps::run_module(&mut driver, &m, rythme::steps::Options::default(), Some(1)).expect("its steps");
        let out = rythme::io::take_captured();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.first(), Some(&expr), "{lang:?}: {out}");
        assert_eq!(lines.get(1), Some(&"  → fib(3) + fib(2)"), "{lang:?}: {out}");
        assert_eq!(lines.last(), Some(&"  → 3"), "{lang:?}: {out}");
        // Only that expression: the print of the code is not run.
        assert!(!out.contains('[') && lines.len() > 4, "{lang:?}: {out}");
    }
}
