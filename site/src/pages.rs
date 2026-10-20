//! Every page of the site, written with active (through Paquets' prelude). Mobile
//! first: one column, a burger menu, and more room as the screen grows (`site.css`).

use paquets::prelude::*;

use crate::client::Bundle;
use crate::examples::{EXAMPLES, json_string};
use crate::highlight::{code_block, code_inline, diagnostic, plain_block};
use crate::i18n::{Lang, Texts};
use crate::pwa;
use crate::routes::{Page as PageKind, Route, link};
use crate::themes::{self, AUTO, THEMES};

pub const REPO: &str = "https://github.com/maxlestage/Rythme";
pub const RELEASES: &str = "https://github.com/maxlestage/Rythme/releases";
const PAQUETS: &str = "https://paquets-90dfc06b6fea.herokuapp.com/";
pub const FONTS: &str = "https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:opsz,wght@12..96,600;12..96,800&family=IBM+Plex+Sans:wght@400;500;600&family=JetBrains+Mono:wght@400;600&display=swap";

/// What a request needs to render a page.
pub struct Ctx {
    pub route: Route,
    /// Absolute origin of the site, for canonical and Open Graph URLs.
    pub origin: String,
    /// Short hash of `site.css` and of the browser side, for `?v=` in their links.
    pub version: String,
}

impl Ctx {
    fn t(&self) -> &'static Texts {
        self.route.lang.texts()
    }

    fn lang(&self) -> Lang {
        self.route.lang
    }

    fn href(&self, page: PageKind) -> String {
        Route::new(self.lang(), page).href()
    }

    fn asset(&self, path: &str) -> String {
        format!("{}?v={}", link(path), self.version)
    }

    /// The absolute address of a page.
    pub fn url(&self, route: Route) -> String {
        format!("{}{}", self.origin, route.path())
    }
}

// ----- Small pieces ---------------------------------------------------------------------

/// The four beats of the logo; they play while a program runs.
fn beats() -> Element {
    span().class("beats").attr("aria-hidden", "true").children((0..4).map(|_| i()))
}

fn external(link: Element) -> Element {
    link.attr("target", "_blank").attr("rel", "noopener")
}

fn button_link(href: impl Into<String>, class: &str, text: &str) -> Element {
    a().class("btn").class(class.to_owned()).href(href.into()).text(text.to_owned())
}

/// The heading of a section: a small label, a title and a lead.
fn section_head(id: &str, title: &str, lead: &str) -> Element {
    header()
        .class("section__head")
        .child(h2().id(format!("{id}-title")).text(title.to_owned()))
        .child(p().class("lead").text(lead.to_owned()))
}

/// A shell command with a copy button (made to work by the browser side, `client/`).
fn command(t: &Texts, text: &str) -> Element {
    div()
        .class("command")
        .child(
            code()
                .attr("translate", "no")
                .child(span().class("command__prompt").attr("aria-hidden", "true").text("$"))
                .text(text.to_owned()),
        )
        .child(
            button()
                .class("command__copy")
                .attr("type", "button")
                .attr("data-copy", text.to_owned())
                .attr("data-copied", t.copied)
                .attr("hidden", "")
                .text(t.copy),
        )
}

/// A link that opens `code` in the playground (`#code=…`).
pub fn playground_link(lang: Lang, code: &str, test: bool) -> String {
    let mut href = format!("{}#code={}", Route::new(lang, PageKind::Playground).href(), percent_encode(code));
    if test {
        href.push_str("&mode=test");
    }
    href
}

/// A link that opens `code` in the playground on the steps of the calculation of
/// `expr`, computed after the definitions of `code` (`#code=…&mode=steps&expr=…`).
pub fn steps_link(lang: Lang, code: &str, expr: &str) -> String {
    format!("{}&mode=steps&expr={}", playground_link(lang, code, false), percent_encode(expr))
}

/// The step of the tour (from 0) whose code also opens on the steps of a
/// calculation, and that calculation.
pub const TOUR_STEPS: (usize, &str) = (2, "fib(4)");

/// The same as JavaScript's `encodeURIComponent`.
pub fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

// ----- Layout ---------------------------------------------------------------------------

/// The links to the same page in every language, each named in its own language.
fn lang_links(ctx: &Ctx) -> [Element; crate::i18n::COUNT] {
    Lang::ALL.map(|l| {
        let route = if ctx.route.page == PageKind::NotFound { Route::new(l, PageKind::Home) } else { ctx.route.with_lang(l) };
        let link = a()
            .class("lang__link")
            .href(route.href())
            .attr("hreflang", l.tag())
            .attr("lang", l.tag())
            .attr("data-lang", l.code())
            .child(span().class("lang__name").text(l.name()))
            .child(span().class("lang__code").attr("aria-hidden", "true").text(l.code().to_uppercase()));
        let link = if l == ctx.lang() { link.class("is-active").attr("aria-current", "true") } else { link };
        li().child(link)
    })
}

/// The language switch of the bar: a compact menu (`details`, which opens without
/// script; the browser side closes it with Escape or a click elsewhere) whose button shows
/// the code of the language.
fn lang_menu(ctx: &Ctx) -> Element {
    let t = ctx.t();
    details()
        .class("lang lang--bar")
        .child(
            summary()
                .class("lang__toggle")
                .attr("aria-label", format!("{} — {}", t.lang_aria, ctx.lang().name()))
                .attr("title", t.lang_aria)
                .child(globe_icon())
                .child(span().class("lang__current").text(ctx.lang().code().to_uppercase())),
        )
        .child(ul().class("lang__menu").children(lang_links(ctx)))
}

/// The language switch of the footer: every language, in a line that wraps.
fn lang_list(ctx: &Ctx) -> Element {
    nav().class("lang lang--foot").attr("aria-label", ctx.t().lang_aria).child(ul().class("lang__list").children(lang_links(ctx)))
}

/// The pages of the navigation, with their names.
fn nav_pages(t: &Texts) -> [(PageKind, &'static str); 5] {
    [
        (PageKind::Learn, t.nav_learn),
        (PageKind::Examples, t.nav_examples),
        (PageKind::Playground, t.nav_playground),
        (PageKind::Teach, t.nav_teach),
        (PageKind::Ai, t.nav_ai),
    ]
}

fn github_icon() -> Element {
    svg()
        .attr("class", "icon")
        .attr("viewBox", "0 0 16 16")
        .attr("width", "18")
        .attr("height", "18")
        .attr("aria-hidden", "true")
        .child(path().attr("fill", "currentColor").attr(
            "d",
            "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z",
        ))
}

fn install_icon() -> Element {
    svg().attr("class", "icon").attr("viewBox", "0 0 16 16").attr("width", "18").attr("height", "18").attr("aria-hidden", "true").child(
        path()
            .attr("fill", "none")
            .attr("stroke", "currentColor")
            .attr("stroke-width", "1.6")
            .attr("stroke-linecap", "round")
            .attr("stroke-linejoin", "round")
            .attr("d", "M8 2v8m-3.5-3.5L8 10l3.5-3.5M3 13.5h10"),
    )
}

/// The share button of Safari, in the hint that explains how to install on iOS.
fn share_icon() -> Element {
    svg().attr("class", "icon").attr("viewBox", "0 0 16 16").attr("width", "18").attr("height", "18").attr("aria-hidden", "true").child(
        path()
            .attr("fill", "none")
            .attr("stroke", "currentColor")
            .attr("stroke-width", "1.4")
            .attr("stroke-linecap", "round")
            .attr("stroke-linejoin", "round")
            .attr("d", "M8 1.5v8.5M5.5 4 8 1.5 10.5 4M5 6.5H3.5v8h9v-8H11"),
    )
}

/// A globe: the language switch.
fn globe_icon() -> Element {
    svg().attr("class", "icon").attr("viewBox", "0 0 16 16").attr("width", "16").attr("height", "16").attr("aria-hidden", "true").child(
        path()
            .attr("fill", "none")
            .attr("stroke", "currentColor")
            .attr("stroke-width", "1.3")
            .attr("d", "M8 1.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13Zm-6.5 6.5h13M8 1.5c1.8 1.8 2.6 4 2.6 6.5S9.8 12.7 8 14.5C6.2 12.7 5.4 10.5 5.4 8S6.2 3.3 8 1.5Z"),
    )
}

/// A circle, half ink and half paper: the theme picker.
fn theme_icon() -> Element {
    svg()
        .attr("class", "icon")
        .attr("viewBox", "0 0 16 16")
        .attr("width", "18")
        .attr("height", "18")
        .attr("aria-hidden", "true")
        .child(path().attr("fill", "currentColor").attr("d", "M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1Zm0 1.5v11a5.5 5.5 0 0 1 0-11Z"))
}

/// The color themes (`themes.rs`), shown by the browser side: real radio buttons in a
/// `fieldset`, so the arrow keys choose and screen readers announce "Theme, 3 of 6".
/// In the menu of a phone the choices are listed; on a computer they open from a button
/// of the bar.
fn theme_picker(t: &Texts) -> Element {
    let options = THEMES.map(|theme| {
        let radio = input()
            .attr("type", "radio")
            .attr("name", "theme")
            .attr("value", theme.id)
            .id(format!("theme-{}", theme.id))
            .attr("data-color", theme.color);
        let radio = if theme.id == AUTO { radio.attr("checked", "").attr("data-dark", themes::DARK) } else { radio };
        label()
            .class("theme__option")
            .attr("for", format!("theme-{}", theme.id))
            .child(radio)
            .child(span().class("theme__swatch").class(format!("theme__swatch--{}", theme.id)).attr("aria-hidden", "true"))
            .child(span().class("theme__name").text(theme.name(t)))
    });
    li().class("nav__item--theme").attr("hidden", "").child(
        div()
            .class("theme")
            .child(
                button()
                    .class("nav__link theme__toggle")
                    .attr("type", "button")
                    .id("theme-toggle")
                    .attr("aria-expanded", "false")
                    .attr("aria-controls", "theme-panel")
                    .attr("title", t.theme_title)
                    .child(theme_icon())
                    .child(span().class("theme__toggle-text").text(t.theme_label)),
            )
            .child(
                fieldset()
                    .class("theme__panel")
                    .id("theme-panel")
                    .child(legend().class("theme__legend").text(t.theme_label))
                    .child(div().class("theme__options").children(options)),
            ),
    )
}

/// The banners of the installed app, shown by the browser side: a new version is ready, and on
/// iPhone and iPad how to install the site.
fn notices(t: &Texts) -> Element {
    div()
        .class("notices")
        .attr("role", "status")
        .child(
            div().class("notice").id("app-update").attr("hidden", "").child(p().class("notice__text").text(t.app_update)).child(
                div()
                    .class("notice__actions")
                    .child(button().class("btn btn--primary btn--sm").attr("type", "button").id("app-reload").text(t.app_reload))
                    .child(button().class("btn btn--ghost btn--sm").attr("type", "button").id("app-later").text(t.app_later)),
            ),
        )
        .child(
            div()
                .class("notice notice--hint")
                .id("ios-hint")
                .attr("hidden", "")
                .child(share_icon())
                .child(p().class("notice__text").text(t.app_ios))
                .child(
                    button()
                        .class("notice__close")
                        .attr("type", "button")
                        .id("ios-hint-close")
                        .attr("aria-label", t.app_close)
                        .attr("title", t.app_close)
                        .text("×"),
                ),
        )
}

fn site_header(ctx: &Ctx) -> Element {
    let t = ctx.t();
    let links = nav_pages(t).map(|(page, text)| {
        let link = a().class("nav__link").href(ctx.href(page)).text(text);
        let link = if page == ctx.route.page { link.class("is-active").attr("aria-current", "page") } else { link };
        li().child(link)
    });
    header().class("bar").id("top").child(
        div()
            .class("bar__inner")
            .child(
                a().class("brand")
                    .href(ctx.href(PageKind::Home))
                    .attr("aria-label", t.brand_aria)
                    .child(span().class("brand__word").text("Rythme"))
                    .child(beats()),
            )
            .child(
                nav().class("nav").id("nav").attr("aria-label", t.nav_aria).child(
                    ul().class("nav__list")
                        .children(links)
                        .child(li().class("nav__item--github").child(external(
                            a().class("nav__link nav__link--github").href(REPO).child(github_icon()).child(span().text("GitHub")),
                        )))
                        // Shown by the browser side when the browser offers to install the site.
                        .child(
                            li().class("nav__item--app").attr("hidden", "").child(
                                button()
                                    .class("nav__link nav__install")
                                    .attr("type", "button")
                                    .id("app-install")
                                    .attr("title", t.app_install_title)
                                    .child(install_icon())
                                    .child(span().text(t.app_install)),
                            ),
                        )
                        .child(theme_picker(t)),
                ),
            )
            .child(lang_menu(ctx))
            .child(
                button()
                    .class("burger")
                    .attr("type", "button")
                    .attr("aria-controls", "nav")
                    .attr("aria-expanded", "false")
                    .attr("aria-label", t.menu)
                    .attr("hidden", "")
                    .child(span().class("burger__box").attr("aria-hidden", "true").children((0..3).map(|_| span().class("burger__line")))),
            ),
    )
}

fn site_footer(ctx: &Ctx) -> Element {
    let t = ctx.t();
    footer().class("foot").child(
        div()
            .class("foot__inner")
            .child(
                div()
                    .class("foot__brand")
                    .child(
                        a().class("brand")
                            .href(ctx.href(PageKind::Home))
                            .attr("aria-label", t.brand_aria)
                            .child(span().class("brand__word").text("Rythme"))
                            .child(beats()),
                    )
                    .child(p().text(t.footer_tagline)),
            )
            .child(
                nav()
                    .class("foot__links")
                    .attr("aria-label", t.nav_aria)
                    .children(nav_pages(t).map(|(page, text)| a().href(ctx.href(page)).text(text))),
            )
            .child(
                div()
                    .class("foot__meta")
                    .child(external(a().href(REPO).text(t.footer_source)))
                    .child(external(a().href(format!("{REPO}/blob/master/LICENSE")).text(t.footer_license)))
                    .child(external(a().href(PAQUETS).text(t.footer_made)))
                    .child(lang_list(ctx)),
            ),
    )
}

/// The page around `content`: head tags, header with the burger menu, footer.
fn layout(ctx: &Ctx, title: &str, description: &str, content: Element) -> Page {
    let lang = ctx.lang();
    let route = ctx.route;
    let mut seo =
        Seo::new().site_name("Rythme").base_url(ctx.origin.clone()).image("/assets/og-image.png").og_type("website").locale(lang.locale());
    if matches!(route.page, PageKind::NotFound | PageKind::Offline) {
        seo = seo.noindex(true);
    } else {
        seo = seo.canonical(route.path());
        for l in Lang::ALL {
            seo = seo.alternate(l.tag(), route.with_lang(l).path());
        }
        seo = seo.alternate("x-default", route.with_lang(Lang::Fr).path());
    }
    if let Some(json) = crate::structured::json_ld(ctx) {
        seo = seo.json_ld(json);
    }
    // For AI assistants: the whole language in one text, and this page as Markdown.
    let mut alternates = format!(
        concat!(
            r#"<link rel="alternate" type="text/plain" href="{llms}" title="llms.txt"/>"#,
            r#"<link rel="alternate" type="text/plain" href="{full}" title="llms-full.txt"/>"#,
        ),
        llms = link(crate::ai::LLMS_PATH),
        full = link(crate::ai::LLMS_FULL_PATH),
    );
    if route.page.has_markdown() {
        alternates
            .push_str(&format!(r#"<link rel="alternate" type="text/markdown" href="{}" title="Markdown"/>"#, crate::markdown::href(route)));
    }
    // Before the first paint (`themes::head_script`): `js` on <html>, the menu folds
    // behind the burger only when the code that opens it can run; and the theme
    // chosen, with its color for the browser's bar. Then the browser side, in Rust
    // compiled to WebAssembly (`client.rs`): the site's bundle, and on the playground
    // its own too (with the interpreter: the menus work before it has arrived).
    let mut client = Bundle::Site.head(&ctx.version);
    if route.page == PageKind::Playground {
        client.push_str(&Bundle::Playground.head(&ctx.version));
    }
    let head = format!(
        concat!(
            r#"<meta name="color-scheme" content="light dark"/>"#,
            r#"<meta name="theme-color" content="{light}" media="(prefers-color-scheme: light)"/>"#,
            r#"<meta name="theme-color" content="{dark}" media="(prefers-color-scheme: dark)"/>"#,
            r#"<link rel="icon" href="{favicon}" sizes="32x32"/>"#,
            r#"<link rel="icon" href="{icon}" type="image/svg+xml"/>"#,
            r#"<link rel="manifest" href="{manifest}"/>"#,
            r#"<link rel="apple-touch-icon" href="{apple}"/>"#,
            r#"<meta name="application-name" content="Rythme"/>"#,
            r#"<meta name="mobile-web-app-capable" content="yes"/>"#,
            r#"<meta name="apple-mobile-web-app-capable" content="yes"/>"#,
            r#"<meta name="apple-mobile-web-app-title" content="Rythme"/>"#,
            r#"<meta name="apple-mobile-web-app-status-bar-style" content="default"/>"#,
            r#"<link rel="preconnect" href="https://fonts.googleapis.com"/>"#,
            r#"<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>"#,
            r#"{alternates}"#,
            r#"<script>{theme}</script>"#,
            r#"{client}"#,
        ),
        client = client,
        alternates = alternates,
        theme = themes::head_script(),
        icon = link(pwa::LOGO),
        favicon = link(pwa::FAVICON),
        light = pwa::LIGHT,
        dark = pwa::DARK,
        manifest = link(&pwa::manifest_path(lang)),
        apple = link(pwa::APPLE_ICON),
    );
    // The browser side registers the service worker from these.
    let body = div()
        .class("site")
        .attr("data-sw", link("/sw.js"))
        .attr("data-scope", link("/"))
        .child(a().class("skip").href("#main").text(ctx.t().skip))
        .child(site_header(ctx))
        .child(content.id("main"))
        .child(site_footer(ctx))
        .child(notices(ctx.t()));
    Page::new(title)
        .lang(lang.tag())
        .description(description)
        .seo(seo)
        .plain()
        .stylesheet(FONTS)
        .stylesheet(ctx.asset("/site.css"))
        .head(head)
        .body(body)
}

// ----- Home -------------------------------------------------------------------------------

pub fn home(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let content = main().class("home").child(hero(ctx)).child(why(t)).child(errors(t)).child(habits(t)).child(install(t)).child(cta(ctx));
    layout(ctx, t.meta_title, t.meta_description, content)
}

fn hero(ctx: &Ctx) -> Element {
    let t = ctx.t();
    section()
        .class("hero")
        .attr("aria-labelledby", "hero-title")
        .child(
            div()
                .class("hero__text")
                .child(p().class("eyebrow").text(t.hero_eyebrow))
                .child(h1().class("hero__title").id("hero-title").text(t.hero_title))
                .child(p().class("hero__lead").text(t.hero_lead))
                .child(
                    div()
                        .class("hero__actions")
                        .child(button_link(ctx.href(PageKind::Playground), "btn--primary btn--lg", t.cta_try))
                        .child(button_link("#install", "btn--ghost btn--lg", t.cta_install)),
                ),
        )
        .child(
            figure()
                .class("score")
                .child(
                    div().class("score__head").child(span().class("file").text("main.ry")).child(
                        a().class("score__run").href(playground_link(ctx.lang(), t.hero_code, false)).text(format!("▶ {}", t.pg_run)),
                    ),
                )
                .child(code_block(t.hero_code))
                .child(figcaption().class("score__out").child(span().class("score__label").text(t.hero_output)).child(code().text("20"))),
        )
}

fn why(t: &Texts) -> Element {
    section().class("section").id("why").attr("aria-labelledby", "why-title").child(section_head("why", t.why_title, t.why_lead)).child(
        ul().class("cards").children(t.why.iter().enumerate().map(|(n, item)| {
            li().class("card")
                .child(span().class("card__num").text(format!("0{}", n + 1)))
                .child(h3().text(item.title))
                .child(p().text(item.text))
        })),
    )
}

fn errors(t: &Texts) -> Element {
    section()
        .class("section section--split")
        .id("errors")
        .attr("aria-labelledby", "errors-title")
        .child(
            div()
                .class("split__text")
                .child(section_head("errors", t.errors_title, t.errors_lead))
                .child(ul().class("checks").children(t.errors_points.iter().map(|point| li().text(*point)))),
        )
        .child(
            div()
                .class("terminal")
                .child(
                    div()
                        .class("terminal__head")
                        .attr("aria-hidden", "true")
                        .child(i())
                        .child(i())
                        .child(i())
                        .child(span().text("rythme main.ry")),
                )
                .child(code_block(t.errors_code))
                .child(diagnostic(t.errors_message)),
        )
}

fn habits(t: &Texts) -> Element {
    section()
        .class("section")
        .id("habits")
        .attr("aria-labelledby", "habits-title")
        .child(section_head("habits", t.habits_title, t.habits_lead))
        .child(
            div().class("table-wrap").child(
                table()
                    .class("habits")
                    .child(thead().child(tr().children(t.habits_head.iter().map(|h| th().attr("scope", "col").text(*h)))))
                    .child(tbody().children(t.habits.iter().map(|h| {
                        tr().child(td().attr("data-label", t.habits_head[0]).child(code().class("habit__other").text(h.other)))
                            .child(td().attr("data-label", t.habits_head[1]).child(code_inline(h.rythme)))
                            .child(td().attr("data-label", t.habits_head[2]).text(h.note))
                    }))),
            ),
        )
}

fn install(t: &Texts) -> Element {
    let steps = t.install_steps.iter().enumerate().map(|(n, step)| {
        let body = if n == 0 {
            div().child(p().text(step.text)).child(external(a().class("btn btn--ghost").href(RELEASES).text("Releases →")))
        } else {
            div().child(command(t, step.text))
        };
        li().class("step").child(span().class("step__num").text(format!("{}", n + 1))).child(h3().text(step.title)).child(body)
    });
    section()
        .class("section")
        .id("install")
        .attr("aria-labelledby", "install-title")
        .child(section_head("install", t.install_title, t.install_lead))
        .child(ol().class("steps").children(steps))
        .child(h3().class("subhead").text(t.commands_title))
        .child(dl().class("commands").children(t.commands.iter().map(|(cmd, what)| {
            div().class("commands__row").child(dt().child(code().attr("translate", "no").text(*cmd))).child(dd().text(*what))
        })))
        .child(p().class("note").text(t.commands_lang))
        .child(p().class("note note--box").text(t.install_app))
}

fn cta(ctx: &Ctx) -> Element {
    let t = ctx.t();
    section()
        .class("cta")
        .attr("aria-labelledby", "cta-title")
        .child(beats())
        .child(h2().id("cta-title").text(t.cta_title))
        .child(p().text(t.cta_text))
        .child(button_link(ctx.href(PageKind::Playground), "btn--primary btn--lg", t.cta_try))
}

// ----- Learn ------------------------------------------------------------------------------

/// The top of an inner page.
fn page_head(title: &str, lead: &str) -> Element {
    header().class("page-head").child(h1().text(title.to_owned())).child(p().class("lead").text(lead.to_owned()))
}

pub fn learn(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let lang = ctx.lang();
    let steps = t.tour.iter().enumerate().map(|(n, step)| {
        let test = step.code.contains("test \"");
        li().class("tour__step")
            .id(format!("step-{}", n + 1))
            .child(
                div()
                    .class("tour__text")
                    .child(span().class("card__num").text(format!("0{}", n + 1)))
                    .child(h2().text(step.title))
                    .child(p().text(step.text)),
            )
            .child(
                div().class("tour__code").child(code_block(step.code)).child(
                    div()
                        .class("tour__links")
                        .child(a().class("tour__try").href(playground_link(lang, step.code, test)).text(format!("▶ {}", t.learn_try)))
                        .children((n == TOUR_STEPS.0).then(|| {
                            a().class("tour__try tour__try--steps")
                                .href(steps_link(lang, step.code, TOUR_STEPS.1))
                                .text(format!("→ {}: {}", t.learn_steps, TOUR_STEPS.1))
                        })),
                ),
            )
    });
    let next = t.learn_next.iter().enumerate().map(|(n, item)| {
        let card = li().class("card");
        let title =
            if n == 2 { h3().child(a().href(ctx.href(PageKind::Examples)).text(item.title)) } else { h3().child(code().text(item.title)) };
        card.child(title).child(p().text(item.text))
    });
    let content = main()
        .class("page")
        .child(page_head(t.learn_title, t.learn_lead))
        .child(ol().class("tour").children(steps))
        .child(p().class("note note--box").text(t.learn_names))
        .child(
            section()
                .class("section")
                .attr("aria-labelledby", "next-title")
                .child(h2().id("next-title").text(t.learn_next_title))
                .child(ul().class("cards cards--3").children(next)),
        );
    layout(ctx, &format!("{} — Rythme", t.learn_title), t.learn_lead, content)
}

// ----- Examples ---------------------------------------------------------------------------

pub fn examples(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let lang = ctx.lang();
    let jump = nav()
        .class("chips")
        .attr("aria-label", t.ex_jump)
        .children(EXAMPLES.iter().map(|e| a().class("chip").href(format!("#{}", e.name)).text(format!("{}.ry", e.name))));
    let list = EXAMPLES.iter().map(|e| {
        article()
            .class("example")
            .id(e.name)
            .attr("aria-labelledby", format!("{}-title", e.name))
            .child(
                header()
                    .class("example__head")
                    .child(
                        div()
                            .child(h2().id(format!("{}-title", e.name)).class("file").text(format!("{}.ry", e.name)))
                            .child(p().text(e.about(lang))),
                    )
                    .child(button_link(
                        format!("{}#example={}", ctx.href(PageKind::Playground), e.name),
                        "btn--ghost btn--sm",
                        &format!("▶ {}", t.ex_open),
                    )),
            )
            .child(
                div().class("example__body").child(code_block(e.code)).child(
                    div()
                        .class("example__out")
                        .child(span().class("score__label").text(t.ex_output))
                        .child(pre().class("output").child(code().text(e.output.trim_end().to_owned()))),
                ),
            )
    });
    let content = main().class("page").child(page_head(t.ex_title, t.ex_lead)).child(jump).child(div().class("examples").children(list));
    layout(ctx, &format!("{} — Rythme", t.ex_title), t.ex_lead, content)
}

// ----- Playground -------------------------------------------------------------------------

/// The texts that the playground writes while it runs (`client/src/playground/`), as a
/// JSON object (`data-ui`).
pub fn playground_ui(t: &Texts) -> String {
    let fields = [
        ("loading", t.pg_loading),
        ("loadFailed", t.pg_load_failed),
        ("running", t.pg_running),
        ("stopped", t.pg_stopped),
        ("ranIn", t.pg_ran_in),
        ("failedIn", t.pg_failed_in),
        ("nothing", t.pg_nothing),
        ("tooDeep", t.pg_too_deep),
        ("line", t.pg_lines[0]),
        ("lines", t.pg_lines[1]),
        ("explain", t.pg_explain),
        ("custom", t.pg_custom),
        ("formatted", t.pg_formatted),
        ("alreadyFormatted", t.pg_already_formatted),
        ("notFormatted", t.pg_not_formatted),
        ("shareCopied", t.pg_share_copied),
        ("shareShared", t.pg_share_shared),
        ("shareSelect", t.pg_share_select),
        ("shareTooLong", t.pg_share_too_long),
        ("shareBroken", t.pg_share_broken),
        ("stepsOf", t.pg_steps_of),
        ("stepsNone", t.pg_steps_none),
        ("seedUsed", t.pg_seed_used),
        ("aiCopied", t.pg_ai_copied),
        ("aiSelect", t.pg_ai_select),
        ("aiIntro", t.pg_ai_intro),
    ];
    let pairs: Vec<String> = fields.iter().map(|(k, v)| format!("{}:{}", json_string(k), json_string(v))).collect();
    format!("{{{}}}", pairs.join(","))
}

/// The steps of a calculation, shown one at a time in the place of the output (Steps,
/// or a link with `&mode=steps`): the previous step and the next, a slider, the step
/// before (faint) and the step itself, what it printed. The browser side fills it; a
/// live region says each step.
fn stepper(t: &Texts) -> Element {
    let nav = div()
        .class("stepper__nav")
        .child(button().class("btn btn--sm").attr("type", "button").id("step-prev").text(format!("← {}", t.pg_steps_prev)))
        .child(span().class("stepper__count").id("step-count"))
        .child(button().class("btn btn--sm").attr("type", "button").id("step-next").text(format!("{} →", t.pg_steps_next)));
    div()
        .class("stepper output--live")
        .id("stepper")
        .attr("hidden", "")
        .attr("tabindex", "-1")
        .attr("aria-labelledby", "output-title")
        .child(nav)
        .child(
            input()
                .class("stepper__range")
                .attr("type", "range")
                .id("step-range")
                .attr("min", "1")
                .attr("max", "1")
                .attr("value", "1")
                .attr("aria-label", t.pg_steps_slider),
        )
        .child(p().class("stepper__keys").text(t.pg_steps_keys))
        .child(pre().class("output stepper__from").id("step-from").attr("aria-hidden", "true"))
        .child(pre().class("output stepper__now").id("step-now").attr("tabindex", "0"))
        .child(
            div()
                .class("stepper__printed")
                .id("step-printed")
                .attr("hidden", "")
                .child(span().class("score__label").text(t.pg_steps_printed))
                .child(pre().class("output").id("step-print")),
        )
        .child(pre().class("output").id("step-error").attr("hidden", ""))
        .child(p().class("sr-only").id("step-live").attr("aria-live", "polite").attr("aria-atomic", "true"))
}

pub fn playground(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let toolbar = div()
        .class("toolbar")
        .child(
            div().class("picker").child(label().attr("for", "example").text(t.pg_example)).child(
                select()
                    .id("example")
                    .child(option().attr("value", "welcome").text(t.pg_welcome))
                    .children(EXAMPLES.iter().map(|e| option().attr("value", e.name).text(format!("{}.ry", e.name)))),
            ),
        )
        .child(
            div()
                .class("actions")
                .child(button().class("btn").attr("type", "button").id("check").attr("disabled", "").text(t.pg_check))
                .child(button().class("btn").attr("type", "button").id("test").attr("disabled", "").text(t.pg_test))
                .child(
                    button()
                        .class("btn")
                        .attr("type", "button")
                        .id("steps")
                        .attr("disabled", "")
                        .attr("title", t.pg_steps_title)
                        .text(t.pg_steps),
                )
                .child(
                    button()
                        .class("btn")
                        .attr("type", "button")
                        .id("format")
                        .attr("disabled", "")
                        .attr("title", t.pg_format_title)
                        .text(t.pg_format),
                )
                .child(seed_option(t))
                .child(
                    button()
                        .class("btn btn--primary")
                        .attr("type", "button")
                        .id("run")
                        .attr("disabled", "")
                        .child(span().text(t.pg_run))
                        .child(span().class("kbd").attr("aria-hidden", "true").text("Ctrl+Enter")),
                )
                .child(button().class("btn btn--stop").attr("type", "button").id("stop").attr("hidden", "").text(t.pg_stop)),
        );
    let workspace = div()
        .class("workspace")
        .child(
            section()
                .class("panel")
                .attr("aria-labelledby", "file-name")
                .child(
                    div().class("panel__head").child(span().class("file").id("file-name").text("main.ry")).child(
                        div()
                            .class("panel__tools")
                            .child(
                                button()
                                    .class("btn btn--ghost btn--sm")
                                    .attr("type", "button")
                                    .id("open")
                                    .attr("title", t.pg_open_title)
                                    .attr("hidden", "")
                                    .text(t.pg_open),
                            )
                            .child(
                                button()
                                    .class("btn btn--ghost btn--sm")
                                    .attr("type", "button")
                                    .id("save")
                                    .attr("title", t.pg_save_title)
                                    .attr("hidden", "")
                                    .text(t.pg_save),
                            )
                            .child(
                                button()
                                    .class("btn btn--ghost btn--sm")
                                    .attr("type", "button")
                                    .id("share")
                                    .attr("title", t.pg_share_title)
                                    .attr("aria-controls", "share-box")
                                    .attr("aria-expanded", "false")
                                    .attr("hidden", "")
                                    .text(t.pg_share),
                            )
                            .child(
                                button()
                                    .class("btn btn--ghost btn--sm")
                                    .attr("type", "button")
                                    .id("copy-ai")
                                    .attr("title", t.pg_ai_title)
                                    .attr("aria-controls", "share-box")
                                    .attr("aria-expanded", "false")
                                    .attr("hidden", "")
                                    .text(t.pg_ai),
                            )
                            .child(
                                input()
                                    .attr("type", "file")
                                    .id("open-file")
                                    .attr("accept", ".ry,text/plain")
                                    .attr("hidden", "")
                                    .attr("aria-label", t.pg_open_title),
                            )
                            .child(span().class("status").id("lines")),
                    ),
                )
                .child(
                    div().class("editor").child(div().class("gutter").id("gutter").attr("aria-hidden", "true").text("1")).child(
                        div().class("editor__code").child(pre().id("highlight").attr("aria-hidden", "true")).child(
                            textarea()
                                .id("source")
                                .attr("spellcheck", "false")
                                .attr("autocapitalize", "off")
                                .attr("autocomplete", "off")
                                .attr("autocorrect", "off")
                                .attr("aria-label", t.pg_code)
                                .text(t.pg_welcome_code),
                        ),
                    ),
                ),
        )
        .child(
            section()
                .class("panel")
                .attr("aria-labelledby", "output-title")
                .child(
                    div()
                        .class("panel__head")
                        .child(span().class("file").id("output-title").text(t.pg_output))
                        .child(span().class("seed-used").id("seed-used").attr("hidden", ""))
                        .child(span().class("status").id("status").attr("role", "status")),
                )
                .child(pre().class("output output--live").id("output").attr("tabindex", "0"))
                .child(stepper(t)),
        );
    let help = section()
        .class("help")
        .attr("aria-labelledby", "help-title")
        .child(
            div()
                .class("help__nav")
                .child(h2().id("help-title").text(t.pg_help))
                .child(p().text(t.pg_help_intro))
                .child(div().class("topics").id("topics"))
                .child(
                    form()
                        .class("search")
                        .id("search-form")
                        .child(
                            input()
                                .id("search")
                                .attr("type", "search")
                                .attr("spellcheck", "false")
                                .attr("autocomplete", "off")
                                .attr("placeholder", t.pg_search)
                                .attr("aria-label", t.pg_search),
                        )
                        .child(button().class("btn").attr("type", "submit").text(t.pg_search_go)),
                ),
        )
        .child(pre().class("help__text").id("help-text").attr("tabindex", "0"));
    let content = main()
        .class("page page--wide playground")
        .attr("data-lang", ctx.lang().code())
        .attr("data-ui", playground_ui(t))
        .attr("data-worker", crate::client::worker_url(&ctx.version))
        .attr("data-examples", link("/examples.json"))
        .attr("data-llms", link(crate::ai::LLMS_FULL_PATH))
        .child(page_head(t.pg_title, t.pg_lead))
        .child(noscript().child(p().class("note note--box").text(t.pg_noscript)))
        .child(toolbar)
        .child(shared_note(t))
        .child(share_box(t))
        .child(workspace)
        .child(help)
        .child(p().class("note").text(t.pg_limits))
        .child(p().class("note").text(t.pg_offline));
    layout(ctx, &format!("{} — Rythme", t.pg_title), t.pg_lead, content)
}

/// The Seed option, next to Run (a `details`, shown by the browser side): empty, the
/// draws of `random` and `shuffle` change from run to run; a whole number, every run
/// makes the same draws (`rythme run --seed N`; with Test, `rythme test --seed N`).
/// The dice picks a new seed and runs. The seed stays in this browser, and travels
/// with *Share* (`&seed=N`).
fn seed_option(t: &Texts) -> Element {
    details()
        .class("seed")
        .id("seed-option")
        .attr("hidden", "")
        .child(
            summary()
                .class("btn seed__toggle")
                .attr("title", t.pg_seed_title)
                .child(span().class("seed__name").text(t.pg_seed))
                .child(span().class("seed__value").id("seed-shown")),
        )
        .child(
            div()
                .class("seed__panel")
                .child(label().class("seed__label").attr("for", "seed").text(t.pg_seed_label))
                .child(
                    div()
                        .class("seed__row")
                        .child(
                            input()
                                .id("seed")
                                .attr("type", "text")
                                .attr("inputmode", "numeric")
                                .attr("pattern", "[0-9]*")
                                .attr("maxlength", "15")
                                .attr("spellcheck", "false")
                                .attr("autocomplete", "off")
                                .attr("placeholder", t.pg_seed_random)
                                .attr("aria-describedby", "seed-hint"),
                        )
                        .child(
                            button()
                                .class("btn btn--sm")
                                .attr("type", "button")
                                .id("seed-new")
                                .child(span().attr("aria-hidden", "true").text("⚄"))
                                .child(span().text(t.pg_seed_new)),
                        ),
                )
                .child(p().class("seed__hint").id("seed-hint").text(t.pg_seed_hint)),
        )
}

/// Shown by the browser side when the program comes from a link (`#code=…`): the draft
/// saved in this browser is kept until the program is changed; the button brings it
/// back.
fn shared_note(t: &Texts) -> Element {
    div()
        .class("note note--box shared")
        .id("shared-note")
        .attr("role", "status")
        .attr("hidden", "")
        .child(p().id("shared-text").text(t.pg_shared_note))
        .child(button().class("btn btn--sm").attr("type", "button").id("shared-back").text(t.pg_shared_back))
}

/// The link made by Share (the browser side): what happened to it (copied, shared, to
/// copy by hand, too long), and the link itself in a field, ready to select.
fn share_box(t: &Texts) -> Element {
    div()
        .class("note note--box share")
        .id("share-box")
        .attr("hidden", "")
        .child(p().class("share__msg").id("share-msg").attr("role", "status").attr("aria-live", "polite"))
        .child(
            div()
                .class("share__row")
                .id("share-row")
                .child(label().class("share__label").attr("for", "share-link").text(t.pg_share_link))
                .child(
                    input()
                        .id("share-link")
                        .attr("type", "url")
                        .attr("readonly", "")
                        .attr("spellcheck", "false")
                        .attr("autocomplete", "off"),
                )
                .child(button().class("btn btn--sm").attr("type", "button").id("share-copy").text(t.pg_share_copy)),
        )
        // Copy for AI, when the clipboard says no: the text, ready to select.
        .child(
            div()
                .class("share__row share__ai")
                .id("ai-row")
                .attr("hidden", "")
                .child(label().class("share__label").attr("for", "ai-text").text(t.pg_ai_label))
                .child(
                    textarea()
                        .id("ai-text")
                        .attr("readonly", "")
                        .attr("rows", "8")
                        .attr("spellcheck", "false")
                        .attr("autocomplete", "off")
                        .attr("translate", "no"),
                ),
        )
        .child(button().class("btn btn--ghost btn--sm share__close").attr("type", "button").id("share-close").text(t.pg_share_close))
}

// ----- Teach ------------------------------------------------------------------------------

pub fn teach(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let guide = crate::teach::render(ctx.lang());
    // A language without a guide of its own: the English one, marked as English, under
    // a note in the language of the page.
    let guide_lang = crate::teach::guide_lang(ctx.lang());
    let mark = |el: Element| if guide_lang == ctx.lang() { el } else { el.attr("lang", guide_lang.tag()) };
    let sections = ol().children(guide.sections.iter().map(|(id, text)| li().child(a().href(format!("#{id}")).text(text.clone()))));
    let toc = nav()
        .class("toc")
        .attr("aria-labelledby", "toc-title")
        .child(p().class("toc__title").id("toc-title").text(t.teach_toc))
        .child(mark(sections));
    let article = mark(article().class("prose").html(guide.html.clone()));
    let head = header().class("page-head").child(mark(h1().text(guide.title.clone()))).child(p().class("lead").text(t.teach_lead));
    let head = match t.teach_note {
        Some(note) => head.child(p().class("note note--box").id("teach-note").text(note)),
        None => head,
    };
    let content = main().class("page").child(head).child(div().class("doc").child(toc).child(article));
    layout(ctx, &format!("{} — Rythme", guide.title), t.teach_lead, content)
}

// ----- For AI assistants --------------------------------------------------------------------

/// A piece of text to copy as it is (a configuration, a message for a chat): its name,
/// where it goes, a copy button (made to work by the browser side) and the text, which wraps.
fn snippet(t: &Texts, name: &str, place: Option<&str>, text: &str) -> Element {
    let head = div().class("snippet__head").child(span().class("snippet__name file").text(name.to_owned()));
    let head = match place {
        Some(place) => head.child(span().class("snippet__where").text(place.to_owned())),
        None => head,
    };
    div()
        .class("snippet")
        .child(
            head.child(
                button()
                    .class("command__copy snippet__copy")
                    .attr("type", "button")
                    .attr("data-copy", text.to_owned())
                    .attr("data-copied", t.copied)
                    .attr("title", format!("{} — {name}", t.copy))
                    .attr("hidden", "")
                    .text(t.copy),
            ),
        )
        .child(plain_block(text))
}

pub fn ai(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let lang = ctx.lang();
    let targets = [link(crate::ai::LLMS_FULL_PATH), link(crate::ai::LLMS_PATH), crate::markdown::href(Route::new(lang, PageKind::Learn))];
    let give = section()
        .class("section")
        .id("give")
        .attr("aria-labelledby", "give-title")
        .child(section_head("give", t.ai_give_title, t.ai_give_lead))
        .child(ul().class("cards cards--3").children(t.ai_give.iter().zip(targets).map(|(item, href)| {
            li().class("card").child(h3().child(a().href(href).attr("translate", "no").text(item.title))).child(p().text(item.text))
        })))
        .child(snippet(t, t.ai_prompt_label, None, &crate::ai::prompt(lang, &ctx.origin)));
    let mcp = section()
        .class("section")
        .id("mcp")
        .attr("aria-labelledby", "mcp-title")
        .child(section_head("mcp", t.ai_mcp_title, t.ai_mcp_lead))
        .child(
            div()
                .class("snippets")
                .children(crate::ai::MCP.iter().zip(t.ai_mcp_where).map(|((name, _, text), place)| snippet(t, name, Some(place), text))),
        )
        .child(p().class("note").text(t.ai_mcp_path));
    let fix = section()
        .class("section")
        .id("loop")
        .attr("aria-labelledby", "loop-title")
        .child(section_head("loop", t.ai_loop_title, t.ai_loop_lead))
        .child(dl().class("commands").children(crate::ai::LOOP.iter().zip(t.ai_loop).map(|(cmd, what)| {
            div().class("commands__row").child(dt().child(code().attr("translate", "no").text(*cmd))).child(dd().text(what))
        })))
        .child(div().class("terminal").child(plain_block(crate::ai::FIX_EXAMPLE)));
    let agents = section()
        .class("section")
        .id("agents")
        .attr("aria-labelledby", "agents-title")
        .child(section_head("agents", t.ai_agents_title, t.ai_agents_lead))
        .child(command(t, crate::ai::NEW_PROJECT));
    let play = section()
        .class("section")
        .id("from-playground")
        .attr("aria-labelledby", "from-playground-title")
        .child(section_head("from-playground", t.ai_play_title, t.ai_play_lead))
        .child(button_link(ctx.href(PageKind::Playground), "btn--primary", t.cta_try));
    let content =
        main().class("page ai").child(page_head(t.ai_title, t.ai_lead)).child(give).child(mcp).child(fix).child(agents).child(play);
    layout(ctx, &format!("{} — Rythme", t.ai_title), t.ai_lead, content)
}

// ----- Not found ------------------------------------------------------------------------

pub fn not_found(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let content = main()
        .class("page nf")
        .child(p().class("eyebrow").text("404"))
        .child(h1().text(t.nf_title))
        .child(p().class("lead").text(t.nf_text))
        .child(
            div()
                .class("hero__actions")
                .child(button_link(ctx.href(PageKind::Home), "btn--primary btn--lg", t.nf_home))
                .child(button_link(ctx.href(PageKind::Playground), "btn--ghost btn--lg", t.nav_playground)),
        );
    layout(ctx, &format!("404 — {}", t.nf_title), t.nf_text, content).status(404)
}

// ----- Offline ----------------------------------------------------------------------------

/// Shown by the service worker, without network, for a page it has not saved.
pub fn offline(ctx: &Ctx) -> Page {
    let t = ctx.t();
    let content = main()
        .class("page nf")
        .child(p().class("eyebrow").child(beats()))
        .child(h1().text(t.offline_title))
        .child(p().class("lead").text(t.offline_text))
        .child(
            div()
                .class("hero__actions")
                // The address of the missing page: the service worker shows this page in
                // its place.
                .child(a().class("btn btn--primary btn--lg").href("").text(t.offline_retry))
                .child(button_link(ctx.href(PageKind::Playground), "btn--ghost btn--lg", t.nav_playground))
                .child(button_link(ctx.href(PageKind::Home), "btn--ghost btn--lg", t.nf_home)),
        );
    layout(ctx, &format!("{} — Rythme", t.offline_title), t.offline_text, content)
}

const VIEWPORT: &str = r#"<meta name="viewport" content="width=device-width, initial-scale=1"/>"#;
/// The installed app draws under the notch and the rounded corners: `site.css` keeps
/// the content in the safe areas.
const VIEWPORT_COVER: &str = r#"<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"/>"#;

/// The response of a page: its HTML with the viewport of the installed app, and its
/// Content-Security-Policy (Paquets computes it from the same page).
pub fn respond(page: impl Fn() -> Page) -> Response {
    let res = page().into_response();
    let html = page().render().replacen(VIEWPORT, VIEWPORT_COVER, 1);
    let out = Response::new().status(res.status_code()).html(html);
    match res.get_header("content-security-policy") {
        Some(csp) => out.header("content-security-policy", csp.to_owned()),
        None => out,
    }
}

/// The page of a route.
pub fn render(ctx: &Ctx) -> Page {
    match ctx.route.page {
        PageKind::Home => home(ctx),
        PageKind::Learn => learn(ctx),
        PageKind::Examples => examples(ctx),
        PageKind::Playground => playground(ctx),
        PageKind::Teach => teach(ctx),
        PageKind::Ai => ai(ctx),
        PageKind::Offline => offline(ctx),
        PageKind::NotFound => not_found(ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_like_javascript() {
        assert_eq!(percent_encode("a b\n|>é"), "a%20b%0A%7C%3E%C3%A9");
        assert_eq!(percent_encode("f(x)*2"), "f(x)*2");
    }

    /// Every text given to the playground's code (`data-ui`, read by
    /// `client/src/playground/`) is used by it, the texts of sharing too; and the
    /// elements it looks for are in the page.
    #[test]
    fn the_playground_code_finds_its_texts_and_elements() {
        let code = include_str!("../client/src/playground/mod.rs");
        let ui = playground_ui(Lang::En.texts());
        let keys: Vec<&str> =
            ui.trim_matches(['{', '}']).split("\",\"").map(|pair| pair.trim_matches('"').split('"').next().unwrap()).collect();
        assert!(keys.contains(&"shareTooLong") && keys.contains(&"shareBroken"), "{keys:?}");
        for key in keys {
            assert!(code.contains(&format!("\"{key}\"")), "the playground does not use {key}");
        }
        let page =
            playground(&Ctx { route: Route::new(Lang::En, PageKind::Playground), origin: String::new(), version: String::new() }).render();
        for id in [
            "share",
            "share-box",
            "share-msg",
            "share-row",
            "share-link",
            "share-copy",
            "share-close",
            "shared-note",
            "shared-text",
            "shared-back",
            "seed-option",
            "seed-shown",
            "seed",
            "seed-new",
            "seed-used",
            "copy-ai",
            "ai-row",
            "ai-text",
        ] {
            assert!(code.contains(&format!("(\"{id}\")")), "the playground does not use #{id}");
            assert!(page.contains(&format!("id=\"{id}\"")), "the playground has no #{id}");
        }
    }
}
