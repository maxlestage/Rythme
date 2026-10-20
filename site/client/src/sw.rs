//! The service worker of the site: every page in every language, the playground and
//! its interpreter work offline after the first visit. `sw.js`, written by the server
//! (`src/pwa.rs` and `src/client.rs` of the site), loads this bundle and calls
//! [`serve`] with the current version and the list of the files of the site.
//!
//! - versioned files (`?v=`): the saved copy, without asking the network;
//! - other files of the site: the saved copy at once, refreshed in the background;
//! - pages: the network first, so that updates arrive; the saved copy when the network
//!   is down or slower than [`TIMEOUT`]; the offline page for a page never saved;
//! - Google Fonts: kept in a cache of their own; other sites are not touched.
//!
//! Only complete answers are saved: never an error, a redirection or another site's.
//! A new version waits until a page asks for it (`skip-waiting`: the "Reload" button,
//! or the page seeing that nothing would be lost, `layout.rs`).

use std::rc::Rc;

use js_sys::{Array, Promise};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use web_sys::{
    Cache, CacheQueryOptions, ExtendableEvent, ExtendableMessageEvent, FetchEvent, Request, RequestCache, RequestInit, RequestMode,
    Response, ResponseInit, ResponseType, ServiceWorkerGlobalScope, Url,
};

use crate::js;

/// How long a page waits for the network before its saved copy is shown.
pub const TIMEOUT: i32 = 3000;
const FONTS_CACHE: &str = "rythme-fonts";
const FONT_FILES: &str = "https://fonts.gstatic.com";
/// The cookie of the language switch (`layout.rs`).
const LANG_COOKIE: &str = "rythme-lang";

/// What the server knows: the version of everything saved, the base path, the URLs to
/// save at install, the offline page of each language, the stylesheet of the fonts.
struct Config {
    cache: String,
    base: String,
    precache: Vec<String>,
    offline: Vec<(String, String)>,
    fonts: String,
}

impl Config {
    fn offline_page(&self, lang: &str) -> Option<&str> {
        self.offline.iter().find(|(l, _)| l == lang).map(|(_, url)| url.as_str())
    }
}

fn scope() -> ServiceWorkerGlobalScope {
    js_sys::global().unchecked_into()
}

fn on<E: JsCast + 'static>(event: &str, handler: impl FnMut(E) + 'static) {
    let mut handler = handler;
    let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |ev: web_sys::Event| handler(ev.unchecked_into()));
    let _ = scope().add_event_listener_with_callback(event, closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Listens to the events of the worker, now: a service worker that has no listener
/// when its script ends gets no event.
pub fn serve(config: &JsValue) {
    let strings = |key: &str| -> Vec<String> { Array::from(&js::get(config, key)).iter().filter_map(|v| v.as_string()).collect() };
    let offline = js::get(config, "offline");
    let config = Rc::new(Config {
        cache: format!("rythme-{}", js::string(config, "version").unwrap_or_default()),
        base: js::string(config, "base").unwrap_or_default(),
        precache: strings("precache"),
        offline: js_sys::Object::keys(offline.unchecked_ref::<js_sys::Object>())
            .iter()
            .filter_map(|k| k.as_string())
            .map(|k| {
                let url = js::string(&offline, &k).unwrap_or_default();
                (k, url)
            })
            .collect(),
        fonts: js::string(config, "fonts").unwrap_or_default(),
    });

    let c = config.clone();
    on::<ExtendableEvent>("install", move |event| {
        // Revalidated with the server: a stale copy of the HTTP cache is never saved.
        let (c, c2) = (c.clone(), c.clone());
        let files = future_to_promise(async move {
            let cache = open(&c2.cache).await?;
            let requests: Array = c2.precache.iter().filter_map(|url| request(url, Some(RequestCache::NoCache), None)).collect();
            JsFuture::from(cache.add_all_with_request_sequence(&requests)).await
        });
        let fonts = future_to_promise(async move {
            let _ = save_fonts(&c).await;
            Ok(JsValue::UNDEFINED)
        });
        let _ = event.wait_until(&Promise::all(&Array::of2(&files, &fonts)));
    });

    on::<ExtendableMessageEvent>("message", |event| {
        if event.data().as_string().as_deref() == Some("skip-waiting") {
            let _ = scope().skip_waiting();
        }
    });

    let c = config.clone();
    on::<ExtendableEvent>("activate", move |event| {
        let c = c.clone();
        let done = future_to_promise(async move {
            let caches = scope().caches()?;
            let keys: Array = JsFuture::from(caches.keys()).await?.unchecked_into();
            for key in keys.iter().filter_map(|k| k.as_string()) {
                if key.starts_with("rythme-") && key != c.cache && key != FONTS_CACHE {
                    JsFuture::from(caches.delete(&key)).await?;
                }
            }
            JsFuture::from(scope().clients().claim()).await
        });
        let _ = event.wait_until(&done);
    });

    on::<FetchEvent>("fetch", move |event| {
        let request = event.request();
        if request.method() != "GET" {
            return;
        }
        let Ok(url) = Url::new(&request.url()) else { return };
        let own_origin = js::string(&js::get(&scope(), "location"), "origin").unwrap_or_default();
        let c = config.clone();
        let e = event.clone();
        let answer = if url.href() == c.fonts || url.origin() == FONT_FILES {
            future_to_promise(async move { font(&e, &url).await.map(JsValue::from) })
        } else if url.origin() == own_origin
            && url.pathname().starts_with(&format!("{}/", c.base))
            && url.pathname() != format!("{}/sw.js", c.base)
        {
            if request.mode() == RequestMode::Navigate {
                future_to_promise(async move { page(&e, &url, &c).await.map(JsValue::from) })
            } else {
                future_to_promise(async move { file(&e, &url, &c).await.map(JsValue::from) })
            }
        } else {
            return;
        };
        let _ = event.respond_with(&answer);
    });
}

async fn open(name: &str) -> Result<Cache, JsValue> {
    Ok(JsFuture::from(scope().caches()?.open(name)).await?.unchecked_into())
}

/// A request for `url`, with its cache mode and its mode.
fn request(url: &str, cache: Option<RequestCache>, mode: Option<RequestMode>) -> Option<JsValue> {
    let init = RequestInit::new();
    if let Some(cache) = cache {
        init.set_cache(cache);
    }
    if let Some(mode) = mode {
        init.set_mode(mode);
    }
    Request::new_with_str_and_init(url, &init).ok().map(Into::into)
}

/// The stylesheet of the fonts and the font files it names, for the first offline
/// visit.
async fn save_fonts(c: &Config) -> Result<(), JsValue> {
    let cache = open(FONTS_CACHE).await?;
    if !JsFuture::from(cache.match_with_str(&c.fonts)).await?.is_undefined() {
        return Ok(());
    }
    let init = RequestInit::new();
    init.set_mode(RequestMode::Cors);
    let css: Response = JsFuture::from(scope().fetch_with_str_and_init(&c.fonts, &init)).await?.unchecked_into();
    if !css.ok() {
        return Ok(());
    }
    let text = JsFuture::from(css.clone()?.text()?).await?.as_string().unwrap_or_default();
    let requests: Array = font_urls(&text).iter().filter_map(|url| request(url, None, Some(RequestMode::Cors))).collect();
    JsFuture::from(cache.add_all_with_request_sequence(&requests)).await?;
    JsFuture::from(cache.put_with_str(&c.fonts, &css)).await?;
    Ok(())
}

/// The font files a stylesheet of Google Fonts names: `url(https://fonts.gstatic.com/…)`.
fn font_urls(css: &str) -> Vec<String> {
    css.match_indices("url(")
        .filter_map(|(at, _)| {
            let rest = &css[at + 4..];
            let end = rest.find(|c: char| c == ')' || c.is_whitespace())?;
            let url = &rest[..end];
            url.starts_with("https://fonts.gstatic.com/").then(|| url.to_owned())
        })
        .collect()
}

/// A complete answer of the site: not an error, a redirection or another site's.
fn complete(response: &Response) -> bool {
    response.ok() && response.type_() == ResponseType::Basic && !response.redirected()
}

fn ignore_vary() -> CacheQueryOptions {
    let options = CacheQueryOptions::new();
    options.set_ignore_vary(true);
    options
}

/// `promise`, whose failure is ignored (`promise.catch(() => {})`).
fn quietly(promise: &Promise) -> Promise {
    let promise = promise.clone();
    future_to_promise(async move {
        let _ = JsFuture::from(promise).await;
        Ok(JsValue::UNDEFINED)
    })
}

/// A file of the site.
async fn file(event: &FetchEvent, url: &Url, c: &Config) -> Result<Response, JsValue> {
    let cache = open(&c.cache).await?;
    let request = event.request();
    let cached = JsFuture::from(cache.match_with_request_and_options(&request, &ignore_vary())).await?;
    if !cached.is_undefined() && url.search_params().has("v") {
        return Ok(cached.unchecked_into());
    }
    let network = future_to_promise({
        let (cache, request) = (cache.clone(), Clone::clone(&request));
        async move {
            let response: Response = JsFuture::from(scope().fetch_with_request(&request)).await?.unchecked_into();
            if complete(&response) {
                JsFuture::from(cache.put_with_request(&request, &response.clone()?)).await?;
            }
            Ok(response.into())
        }
    });
    if cached.is_undefined() {
        return Ok(JsFuture::from(network).await?.unchecked_into());
    }
    let _ = event.wait_until(&quietly(&network));
    Ok(cached.unchecked_into())
}

/// The language (`fr`, `en`, `es`…): from the address, else the one chosen with the
/// switch (the cookie of the language switch, where the browser lets a worker read it),
/// else the browser's.
async fn lang(url: &Url, c: &Config) -> String {
    let path = url.pathname();
    let first = path.get(c.base.len()..).unwrap_or("").split('/').nth(1).unwrap_or("").to_owned();
    if c.offline_page(&first).is_some() {
        return first;
    }
    let store = js::get(&scope(), "cookieStore");
    if store.is_object()
        && let Ok(chosen) = js::call_async(&store, "get", &[JsValue::from_str(LANG_COOKIE)]).await
        && let Some(value) = js::string(&chosen, "value")
        && c.offline_page(&value).is_some()
    {
        return value;
    }
    let languages = scope().navigator().languages();
    languages
        .iter()
        .filter_map(|l| l.as_string())
        .map(|l| l.chars().take(2).collect::<String>().to_lowercase())
        .find(|l| c.offline_page(l).is_some())
        .unwrap_or_else(|| "fr".into())
}

/// A page: the network first; the saved copy without network, or when it is slower
/// than `TIMEOUT`; else the offline page of its language.
async fn page(event: &FetchEvent, url: &Url, c: &Config) -> Result<Response, JsValue> {
    let cache = open(&c.cache).await?;
    let path = url.pathname();
    // Saved without its query: `?text=…` of a shared text is the same playground.
    // Revalidated with the server, never from the HTTP cache (GitHub Pages keeps pages 10
    // minutes): a new deployment shows at once, and its new worker is found at once.
    let network = future_to_promise({
        let (cache, request, path) = (cache.clone(), event.request(), path.clone());
        async move {
            let init = RequestInit::new();
            init.set_cache(RequestCache::NoCache);
            let response: Response = JsFuture::from(scope().fetch_with_request_and_init(&request, &init)).await?.unchecked_into();
            if complete(&response) {
                JsFuture::from(cache.put_with_str(&path, &response.clone()?)).await?;
            }
            Ok(response.into())
        }
    });
    let _ = event.wait_until(&quietly(&network));
    let late = Promise::new(&mut |resolve, _| {
        let _ = scope().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, TIMEOUT);
    });
    let fresh = JsFuture::from(Promise::race(&Array::of2(&network, &late))).await.unwrap_or(JsValue::NULL);
    if fresh.is_object() {
        return Ok(fresh.unchecked_into());
    }
    let cached = JsFuture::from(cache.match_with_str_and_options(&path, &ignore_vary())).await?;
    if !cached.is_undefined() {
        return Ok(cached.unchecked_into());
    }
    // `/fr/learn`: `/fr/learn/`, as the server redirects.
    if !path.ends_with('/') && !JsFuture::from(cache.match_with_str(&format!("{path}/"))).await?.is_undefined() {
        return Response::redirect_with_status(&format!("{path}/{}{}", url.search(), url.hash()), 301);
    }
    // A slow network and nothing saved: its answer after all.
    if let Ok(answer) = JsFuture::from(network).await {
        return Ok(answer.unchecked_into());
    }
    // `/`: the home page in the browser's language, as the server or the static page does.
    if path == format!("{}/", c.base) || path == format!("{}/index.html", c.base) {
        return Response::redirect_with_status(&format!("{}/{}/", c.base, lang(url, c).await), 302);
    }
    let lang = lang(url, c).await;
    if let Some(offline) = c.offline_page(&lang) {
        let saved = JsFuture::from(cache.match_with_str(offline)).await?;
        if !saved.is_undefined() {
            return Ok(saved.unchecked_into());
        }
    }
    Ok(Response::error())
}

/// The Google Fonts of the pages: the saved copy, else the network (and saved).
async fn font(event: &FetchEvent, url: &Url) -> Result<Response, JsValue> {
    let cache = open(FONTS_CACHE).await?;
    let request = event.request();
    let cached = JsFuture::from(cache.match_with_request_and_options(&request, &ignore_vary())).await?;
    let network = future_to_promise({
        let (cache, request) = (cache.clone(), Clone::clone(&request));
        async move {
            let response: Response = JsFuture::from(scope().fetch_with_request(&request)).await?.unchecked_into();
            // The page asks for the stylesheet without CORS: an opaque answer, whose
            // status is unknown, so it is replaced at the next visit.
            if response.ok() || response.type_() == ResponseType::Opaque {
                JsFuture::from(cache.put_with_request(&request, &response.clone()?)).await?;
            }
            Ok(response.into())
        }
    });
    if !cached.is_undefined() {
        if url.origin() != FONT_FILES {
            let _ = event.wait_until(&quietly(&network));
        }
        return Ok(cached.unchecked_into());
    }
    match JsFuture::from(network).await {
        Ok(response) => Ok(response.unchecked_into()),
        // Offline, never saved: no stylesheet rather than an error; the site has its own
        // fonts.
        Err(_) if url.origin() == FONT_FILES => Ok(Response::error()),
        Err(_) => {
            let init = ResponseInit::new();
            let headers = web_sys::Headers::new()?;
            headers.set("content-type", "text/css")?;
            init.set_headers(&headers);
            Response::new_with_opt_str_and_init(Some(""), &init)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fonts_of_a_stylesheet() {
        let css = "@font-face { src: url(https://fonts.gstatic.com/s/a.woff2) format('woff2'); }\nsrc: url(https://other.example/b.woff2);url(https://fonts.gstatic.com/s/c.woff2)";
        assert_eq!(font_urls(css), ["https://fonts.gstatic.com/s/a.woff2", "https://fonts.gstatic.com/s/c.woff2"]);
    }
}
