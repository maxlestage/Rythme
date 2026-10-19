//! The structured data of the pages (JSON-LD, schema.org), for search engines and the
//! assistants that read the web: on the home page, Rythme as a `ComputerLanguage`, its
//! interpreter as a `SoftwareApplication` (version, license, download) and its source
//! code (`SoftwareSourceCode`, written in Rust); on the pages that teach, a
//! `TechArticle` with each piece of code as a `SoftwareSourceCode` in Rythme. JSON-LD is
//! data, not a script: the Content-Security-Policy does not run it.

use crate::ai;
use crate::examples::EXAMPLES;
use crate::json::{Json, s};
use crate::pages::{Ctx, RELEASES, REPO};
use crate::routes::{Page, Route};

/// The license of Rythme.
pub const LICENSE: &str = "https://github.com/maxlestage/Rythme/blob/master/LICENSE";

fn rythme(ctx: &Ctx) -> Json {
    Json::Obj(vec![("@type", s("ComputerLanguage")), ("@id", s(format!("{}/#language", ctx.origin))), ("name", s("Rythme"))])
}

fn website(ctx: &Ctx) -> Json {
    Json::Obj(vec![
        ("@type", s("WebSite")),
        ("@id", s(format!("{}/#website", ctx.origin))),
        ("name", s("Rythme")),
        ("url", s(format!("{}/", ctx.origin))),
    ])
}

/// A piece of Rythme code.
fn code(ctx: &Ctx, name: &str, description: &str, text: &str, kind: &str, url: Option<String>) -> Json {
    let mut fields = vec![
        ("@type", s("SoftwareSourceCode")),
        ("name", s(name)),
        ("description", s(description)),
        ("programmingLanguage", rythme(ctx)),
        ("codeSampleType", s(kind)),
        ("runtimePlatform", s(format!("Rythme {}", ai::version()))),
        ("text", s(text)),
    ];
    if let Some(url) = url {
        fields.push(("url", s(url)));
    }
    Json::Obj(fields)
}

/// The JSON-LD of a page, if it has some.
pub fn json_ld(ctx: &Ctx) -> Option<String> {
    let t = ctx.route.lang.texts();
    let lang = ctx.route.lang;
    let url = ctx.url(ctx.route);
    let article = |headline: &str, description: &str, parts: Vec<Json>| {
        let mut fields = vec![
            ("@context", s("https://schema.org")),
            ("@type", s("TechArticle")),
            ("headline", s(headline)),
            ("description", s(description)),
            ("url", s(url.clone())),
            ("inLanguage", s(lang.tag())),
            ("isPartOf", website(ctx)),
            ("about", rythme(ctx)),
            ("license", s(LICENSE)),
            (
                "encoding",
                Json::Obj(vec![
                    ("@type", s("MediaObject")),
                    ("encodingFormat", s("text/markdown")),
                    ("contentUrl", s(format!("{}{}", ctx.origin, ctx.route.markdown_path()))),
                ]),
            ),
        ];
        if !parts.is_empty() {
            fields.push(("hasPart", Json::Arr(parts)));
        }
        Json::Obj(fields)
    };
    let json = match ctx.route.page {
        Page::Home => home(ctx),
        Page::Learn => {
            let parts = t.tour.iter().map(|step| code(ctx, step.title, step.text, step.code, "code snippet", None)).collect();
            let mut json = article(t.learn_title, t.learn_lead, parts);
            if let Json::Obj(fields) = &mut json {
                fields.push(("proficiencyLevel", s("Beginner")));
            }
            json
        }
        Page::Examples => {
            let parts = EXAMPLES
                .iter()
                .map(|e| code(ctx, &format!("{}.ry", e.name), e.about(lang), e.code, "full solution", Some(format!("{url}#{}", e.name))))
                .collect();
            article(t.ex_title, t.ex_lead, parts)
        }
        Page::Teach => article(&crate::teach::render(lang).title, t.teach_lead, Vec::new()),
        Page::Ai => article(t.ai_title, t.ai_lead, Vec::new()),
        _ => return None,
    };
    Some(json.to_string())
}

/// Rythme, its interpreter and its source code.
fn home(ctx: &Ctx) -> Json {
    let t = ctx.route.lang.texts();
    let lang = ctx.route.lang;
    let home = ctx.url(ctx.route);
    let version = ai::version();
    let language = Json::Obj(vec![
        ("@type", s("ComputerLanguage")),
        ("@id", s(format!("{}/#language", ctx.origin))),
        ("name", s("Rythme")),
        ("description", s(t.meta_description)),
        ("url", s(home.clone())),
        ("version", s(version)),
        ("license", s(LICENSE)),
        ("sameAs", Json::Arr(vec![s(REPO)])),
    ]);
    let interpreter = Json::Obj(vec![
        ("@type", s("SoftwareApplication")),
        ("@id", s(format!("{}/#interpreter", ctx.origin))),
        ("name", s("Rythme")),
        ("alternateName", s("rythme")),
        ("description", s(t.footer_tagline)),
        ("url", s(home.clone())),
        ("applicationCategory", s("DeveloperApplication")),
        ("applicationSubCategory", s("Programming language")),
        ("operatingSystem", s("Linux, macOS, Windows")),
        ("softwareVersion", s(version)),
        ("license", s(LICENSE)),
        ("downloadUrl", s(RELEASES)),
        ("installUrl", s(format!("{home}#install"))),
        ("softwareHelp", s(ctx.url(Route::new(lang, Page::Learn)))),
        ("inLanguage", s(lang.tag())),
        ("isAccessibleForFree", Json::Bool(true)),
        ("offers", Json::Obj(vec![("@type", s("Offer")), ("price", s("0")), ("priceCurrency", s("USD"))])),
        ("about", Json::Obj(vec![("@id", s(format!("{}/#language", ctx.origin)))])),
    ]);
    let source = Json::Obj(vec![
        ("@type", s("SoftwareSourceCode")),
        ("@id", s(format!("{}/#source", ctx.origin))),
        ("name", s("Rythme")),
        ("codeRepository", s(REPO)),
        ("programmingLanguage", Json::Obj(vec![("@type", s("ComputerLanguage")), ("name", s("Rust"))])),
        ("version", s(version)),
        ("license", s(LICENSE)),
        ("targetProduct", Json::Obj(vec![("@id", s(format!("{}/#interpreter", ctx.origin)))])),
    ]);
    let mut site = website(ctx);
    if let Json::Obj(fields) = &mut site {
        fields.push(("inLanguage", s(lang.tag())));
    }
    Json::Obj(vec![("@context", s("https://schema.org")), ("@graph", Json::Arr(vec![language, interpreter, source, site]))])
}
