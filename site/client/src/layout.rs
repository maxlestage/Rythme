//! What every page does: the burger menu, the language menu, the copy buttons, the
//! remembered language, the installable app (the service worker, the "new version"
//! banner, the install button and the hint for iOS), and the color themes. Every page
//! works without it: the links then stay visible under the logo, and the site is a plain
//! website. `/` of the static export picks the language here too ([`pick_language`]).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Element, Event, FocusEvent, HtmlAnchorElement, HtmlDetailsElement, HtmlElement, HtmlInputElement, KeyboardEvent, MediaQueryList,
    RegistrationOptions, ServiceWorker, ServiceWorkerRegistration, ServiceWorkerState, ServiceWorkerUpdateViaCache, StorageEvent,
};

use crate::dom::{self, on, on_capture};
use crate::js;

/// The key of `localStorage` that keeps the chosen theme (read by the inline script of
/// the head too, `themes.rs` of the server).
const THEME_KEY: &str = "rythme-theme";
/// The cookie of the language switch, read by the server at `/`.
const LANG_COOKIE: &str = "rythme-lang";

pub fn start() {
    if dom::query::<Element>("[data-pick-language]").is_some() {
        pick_language();
        return;
    }
    let wide = dom::window().match_media("(min-width: 860px)").ok().flatten();
    let menu = burger_menu(wide.clone());
    language_menu(menu.clone());
    remember_language();
    copy_buttons();
    service_worker();
    install_button();
    ios_hint();
    themes(wide);
}

/// The burger menu: opens and closes the panel of the navigation (`menu-open` on
/// <html>), which starts under the real bottom of the bar (`--bar-bottom`).
#[derive(Clone)]
struct Menu {
    burger: Option<HtmlElement>,
    nav: Option<HtmlElement>,
    wide: Option<MediaQueryList>,
}

impl Menu {
    fn is_open(&self) -> bool {
        dom::root().class_list().contains("menu-open")
    }

    fn set(&self, open: bool) {
        let (Some(burger), Some(nav)) = (&self.burger, &self.nav) else { return };
        if open {
            measure_bar();
        }
        let _ = dom::root().class_list().toggle_with_force("menu-open", open);
        let _ = burger.set_attribute("aria-expanded", if open { "true" } else { "false" });
        // Closed on a phone, the panel's links leave the tab order.
        let wide = self.wide.as_ref().is_some_and(MediaQueryList::matches);
        js::set(nav, "inert", &JsValue::from_bool(!open && !wide));
    }
}

/// The bottom of the bar, where the menu panel and the language list start: the bar
/// wraps on two lines when the text is enlarged, and scrolls away on a short screen.
fn measure_bar() {
    if let Some(bar) = dom::query::<Element>(".bar") {
        let bottom = bar.get_bounding_client_rect().bottom().round().max(0.0);
        let _ = dom::root().style().set_property("--bar-bottom", &format!("{bottom}px"));
    }
}

fn burger_menu(wide: Option<MediaQueryList>) -> Menu {
    let menu = Menu { burger: dom::query(".burger"), nav: dom::by_id("nav"), wide };
    on::<Event>(&dom::window(), "resize", |_| measure_bar());
    let (Some(burger), Some(nav)) = (menu.burger.clone(), menu.nav.clone()) else { return menu };
    burger.set_hidden(false);
    menu.set(false);
    let m = menu.clone();
    on::<Event>(&burger, "click", move |_| m.set(!m.is_open()));
    let m = menu.clone();
    on::<Event>(&nav, "click", move |ev| {
        let in_link = ev.target().and_then(|t| t.dyn_into::<Element>().ok()).is_some_and(|el| el.closest("a").ok().flatten().is_some());
        if in_link {
            m.set(false);
        }
    });
    let m = menu.clone();
    on::<KeyboardEvent>(&dom::document(), "keydown", move |ev| {
        if ev.key() == "Escape" && m.is_open() {
            m.set(false);
            let _ = burger.focus();
        }
    });
    if let Some(wide) = &menu.wide {
        let m = menu.clone();
        on::<Event>(wide, "change", move |_| m.set(false));
    }
    menu
}

/// The language menu of the bar: a `details`, which opens and closes without this
/// code; this closes it with Escape (the focus goes back to its button), a click
/// elsewhere, the focus leaving it, or the burger menu opening.
fn language_menu(menu: Menu) {
    let Some(details) = dom::query::<HtmlDetailsElement>(".lang--bar") else { return };
    let Some(toggle) = dom::query_in::<HtmlElement>(&details, "summary") else { return };
    let d = details.clone();
    on::<KeyboardEvent>(&details, "keydown", move |ev| {
        if ev.key() == "Escape" && d.open() {
            ev.stop_propagation();
            d.set_open(false);
            let _ = toggle.focus();
        }
    });
    let d = details.clone();
    on::<Event>(&dom::document(), "click", move |ev| {
        if d.open() && !dom::holds(&d, ev.target()) {
            d.set_open(false);
        }
    });
    let d = details.clone();
    on::<FocusEvent>(&details, "focusout", move |ev| {
        if let Some(next) = ev.related_target()
            && !dom::holds(&d, Some(next))
        {
            d.set_open(false);
        }
    });
    let d = details.clone();
    let m = menu.clone();
    on::<Event>(&details, "toggle", move |_| {
        if d.open() {
            measure_bar();
            if m.burger.is_some() && m.is_open() {
                m.set(false);
            }
        }
    });
    if let Some(burger) = &menu.burger {
        on::<Event>(burger, "click", move |_| details.set_open(false));
    }
}

/// The language chosen with the switch, for the next visit of `/`: a cookie under the
/// site's base path (`/Rythme` on GitHub Pages).
fn remember_language() {
    for link in dom::query_all::<HtmlAnchorElement>(None, ".lang__link[data-lang]") {
        let l = link.clone();
        on::<Event>(&link, "click", move |_| {
            let Some(lang) = l.get_attribute("data-lang") else { return };
            let pathname = l.pathname();
            let path = match pathname.rfind(&format!("/{lang}/")) {
                Some(0) | None => "/".to_owned(),
                Some(at) => pathname[..at].to_owned(),
            };
            if let Ok(doc) = dom::document().dyn_into::<web_sys::HtmlDocument>() {
                let _ = doc.set_cookie(&format!("{LANG_COOKIE}={lang}; Path={path}; Max-Age=31536000; SameSite=Lax"));
            }
        });
    }
}

/// The copy buttons of commands and texts (`data-copy`), shown when the browser has a
/// clipboard: "Copied" for a moment. When the clipboard says no, the text stays
/// selectable.
fn copy_buttons() {
    let navigator = dom::window().navigator();
    if js::get(&navigator, "clipboard").is_falsy() {
        return;
    }
    for button in dom::query_all::<HtmlElement>(None, "[data-copy]") {
        let label = button.text_content().unwrap_or_default();
        button.set_hidden(false);
        let b = button.clone();
        on::<Event>(&button, "click", move |_| {
            let (b, label) = (b.clone(), label.clone());
            wasm_bindgen_futures::spawn_local(async move {
                let text = b.get_attribute("data-copy").unwrap_or_default();
                let clipboard = js::get(&dom::window().navigator(), "clipboard");
                if js::call_async(&clipboard, "writeText", &[JsValue::from_str(&text)]).await.is_ok() {
                    b.set_text_content(b.get_attribute("data-copied").as_deref());
                    dom::later(1600, move || b.set_text_content(Some(&label)));
                }
            });
        });
    }
}

/// Whether the site runs as an installed app.
fn standalone() -> bool {
    dom::media("(display-mode: standalone)") || js::get(&dom::window().navigator(), "standalone") == JsValue::TRUE
}

/// The service worker keeps the site offline. A new version installs in the background
/// (looked for at each visit, and each time the app comes back to the foreground) and
/// waits. It comes in by itself only when nobody can lose anything: when the app opens
/// or comes back and the new version is ready before the visitor has touched anything,
/// or when the app goes to the background with no field being edited and no program
/// running (an installed app is rarely closed, so "next launch" would otherwise never
/// come). Otherwise the banner offers to reload, and nothing reloads on its own: the
/// playground may be in use.
fn service_worker() {
    let Some(site) = dom::query::<Element>(".site[data-sw]") else { return };
    let Some(update) = dom::by_id::<HtmlElement>("app-update") else { return };
    let navigator = dom::window().navigator();
    if !js::has(&navigator, "serviceWorker") {
        return;
    }
    let container = navigator.service_worker();
    let state = Rc::new(Updates { registration: RefCell::new(None), reloading: Cell::new(false), touched: Cell::new(false), update });
    for event in ["pointerdown", "keydown", "input"] {
        let s = state.clone();
        on_capture::<Event>(&dom::document(), event, move |_| s.touched.set(true));
    }
    let s = state.clone();
    let register = move || {
        let s = s.clone();
        let site = site.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let options = RegistrationOptions::new();
            options.set_scope(&site.get_attribute("data-scope").unwrap_or_default());
            options.set_update_via_cache(ServiceWorkerUpdateViaCache::Imports);
            let url = site.get_attribute("data-sw").unwrap_or_default();
            let container = dom::window().navigator().service_worker();
            let Ok(registration) = js::wait(container.register_with_options(&url, &options).into()).await else { return };
            let registration: ServiceWorkerRegistration = registration.unchecked_into();
            *s.registration.borrow_mut() = Some(registration.clone());
            s.settle();
            if let Some(worker) = registration.installing() {
                s.watch(worker);
            }
            let (s2, r) = (s.clone(), registration.clone());
            on::<Event>(&registration, "updatefound", move |_| {
                if let Some(worker) = r.installing() {
                    s2.watch(worker);
                }
            });
            // An app left open looks for a new version when it comes back, and takes the
            // one waiting when it leaves.
            let s3 = s.clone();
            on::<Event>(&dom::document(), "visibilitychange", move |_| {
                if dom::document().visibility_state() == web_sys::VisibilityState::Visible {
                    s3.touched.set(false);
                    if let Some(r) = s3.registration.borrow().as_ref()
                        && let Ok(promise) = r.update()
                    {
                        wasm_bindgen_futures::spawn_local(async move {
                            let _ = js::wait(promise.into()).await;
                        });
                    }
                } else {
                    s3.settle();
                }
            });
        });
    };
    // The bundle may start after the page has loaded.
    if dom::document().ready_state() == "complete" {
        register();
    } else {
        let mut register = Some(register);
        on::<Event>(&dom::window(), "load", move |_| {
            if let Some(register) = register.take() {
                register();
            }
        });
    }
    on::<Event>(&container, "controllerchange", {
        let s = state.clone();
        move |_| {
            if s.reloading.get() {
                let _ = dom::window().location().reload();
            }
        }
    });
    if let Some(reload) = dom::by_id::<HtmlElement>("app-reload") {
        let s = state.clone();
        on::<Event>(&reload, "click", move |_| {
            s.reloading.set(true);
            match s.registration.borrow().as_ref().and_then(ServiceWorkerRegistration::waiting) {
                Some(waiting) => {
                    let _ = waiting.post_message(&JsValue::from_str("skip-waiting"));
                }
                None => {
                    let _ = dom::window().location().reload();
                }
            }
        });
    }
    if let Some(later) = dom::by_id::<HtmlElement>("app-later") {
        let s = state.clone();
        on::<Event>(&later, "click", move |_| s.update.set_hidden(true));
    }
}

/// The state of the updates of the app.
struct Updates {
    registration: RefCell<Option<ServiceWorkerRegistration>>,
    reloading: Cell<bool>,
    /// Touched since the page opened or came back to the foreground.
    touched: Cell<bool>,
    update: HtmlElement,
}

impl Updates {
    /// A field being edited, or a program running.
    fn editing() -> bool {
        let field = dom::active().is_some_and(|el| {
            let editable = el.dyn_ref::<HtmlElement>().is_some_and(HtmlElement::is_content_editable);
            editable || matches!(el.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT")
        });
        field || dom::body().class_list().contains("running")
    }

    /// A new version is waiting: in by itself if that loses nothing, else the banner.
    fn settle(&self) {
        let registration = self.registration.borrow();
        let Some(waiting) = registration.as_ref().and_then(ServiceWorkerRegistration::waiting) else { return };
        if dom::window().navigator().service_worker().controller().is_none() || self.reloading.get() {
            return;
        }
        let hidden = dom::document().visibility_state() == web_sys::VisibilityState::Hidden;
        if !Self::editing() && (hidden || !self.touched.get()) {
            self.reloading.set(true);
            let _ = waiting.post_message(&JsValue::from_str("skip-waiting"));
        } else {
            self.update.set_hidden(false);
        }
    }

    fn watch(self: &Rc<Self>, worker: ServiceWorker) {
        let s = self.clone();
        let w = worker.clone();
        on::<Event>(&worker, "statechange", move |_| {
            if w.state() == ServiceWorkerState::Installed {
                s.settle();
            }
        });
    }
}

/// The install button, when the browser offers it (Chrome, Edge…), until installed.
fn install_button() {
    let Some(install) = dom::by_id::<HtmlElement>("app-install") else { return };
    let Some(item) = install.closest("li").ok().flatten().and_then(|li| li.dyn_into::<HtmlElement>().ok()) else { return };
    let offer: Rc<RefCell<Option<Event>>> = Rc::new(RefCell::new(None));
    let (o, i) = (offer.clone(), item.clone());
    on::<Event>(&dom::window(), "beforeinstallprompt", move |ev| {
        ev.prevent_default();
        *o.borrow_mut() = Some(ev);
        i.set_hidden(standalone());
    });
    let (o, i) = (offer.clone(), item.clone());
    on::<Event>(&install, "click", move |_| {
        let Some(offer) = o.borrow().clone() else { return };
        let (o, i) = (o.clone(), i.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let _ = js::call(&offer, "prompt", &[]);
            let _ = js::wait(js::get(&offer, "userChoice")).await;
            *o.borrow_mut() = None;
            i.set_hidden(true);
        });
    });
    on::<Event>(&dom::window(), "appinstalled", move |_| {
        *offer.borrow_mut() = None;
        item.set_hidden(true);
    });
}

/// iPhone and iPad have no install prompt: once, until closed, how to add the site to
/// the home screen.
fn ios_hint() {
    let Some(hint) = dom::by_id::<HtmlElement>("ios-hint") else { return };
    let navigator = dom::window().navigator();
    let agent = navigator.user_agent().unwrap_or_default();
    let touch_points = js::get(&navigator, "maxTouchPoints").as_f64().unwrap_or(0.0);
    let ios = ["iPad", "iPhone", "iPod"].iter().any(|d| agent.contains(d)) || (agent.contains("Macintosh") && touch_points > 1.0);
    if !ios || standalone() || dom::load("rythme-ios-hint").is_some() {
        return;
    }
    hint.set_hidden(false);
    if let Some(close) = dom::by_id::<HtmlElement>("ios-hint-close") {
        on::<Event>(&close, "click", move |_| {
            hint.set_hidden(true);
            dom::save("rythme-ios-hint", "closed");
        });
    }
}

/// The color themes. The inline script of the head has already applied the theme kept
/// (before the first paint); this shows the picker and keeps the next choice. Auto has
/// no `data-theme`: the system's light or dark applies.
fn themes(wide: Option<MediaQueryList>) {
    let Some(picker) = dom::query::<HtmlElement>(".nav__item--theme") else { return };
    let Some(toggle) = dom::by_id::<HtmlElement>("theme-toggle") else { return };
    let radios: Rc<Vec<HtmlInputElement>> = Rc::new(dom::query_all(Some(&picker), "input[name=\"theme\"]"));
    if radios.is_empty() {
        return;
    }
    let metas: Rc<Vec<Element>> = Rc::new(dom::query_all(None, "meta[name=\"theme-color\"]"));
    let apply = {
        let (radios, metas) = (radios.clone(), metas.clone());
        Rc::new(move |theme: &str| {
            let radio = radios.iter().find(|r| r.value() == theme).unwrap_or(&radios[0]);
            radio.set_checked(true);
            let color = radio.get_attribute("data-color").unwrap_or_default();
            let root = dom::root();
            if radio.value() == "auto" {
                let _ = root.remove_attribute("data-theme");
                let dark = radio.get_attribute("data-dark").unwrap_or_default();
                for meta in metas.iter() {
                    let media = meta.get_attribute("media").unwrap_or_default();
                    let _ = meta.set_attribute("content", if media.contains("dark") { &dark } else { &color });
                }
            } else {
                let _ = root.set_attribute("data-theme", &radio.value());
                for meta in metas.iter() {
                    let _ = meta.set_attribute("content", &color);
                }
            }
        })
    };
    picker.set_hidden(false);
    apply(&dom::root().get_attribute("data-theme").unwrap_or_else(|| "auto".into()));
    for radio in radios.iter() {
        let (r, apply) = (radio.clone(), apply.clone());
        on::<Event>(radio, "change", move |_| {
            if r.checked() {
                apply(&r.value());
                dom::save(THEME_KEY, &r.value());
            }
        });
    }
    // The same choice in the other tabs of the site.
    let a = apply.clone();
    on::<StorageEvent>(&dom::window(), "storage", move |ev| {
        if ev.key().as_deref() == Some(THEME_KEY) {
            a(&ev.new_value().unwrap_or_else(|| "auto".into()));
        }
    });

    // On a computer the choices open from the button of the bar.
    let set_open = {
        let (toggle, picker) = (toggle.clone(), picker.clone());
        Rc::new(move |open: bool| {
            let _ = toggle.set_attribute("aria-expanded", if open { "true" } else { "false" });
            let _ = picker.class_list().toggle_with_force("is-open", open);
        })
    };
    let (t, s, r) = (toggle.clone(), set_open.clone(), radios.clone());
    on::<Event>(&toggle, "click", move |_| {
        let open = t.get_attribute("aria-expanded").as_deref() != Some("true");
        s(open);
        if open {
            let _ = r.iter().find(|r| r.checked()).unwrap_or(&r[0]).focus();
        }
    });
    let (p, s, t) = (picker.clone(), set_open.clone(), toggle.clone());
    on::<KeyboardEvent>(&picker, "keydown", move |ev| {
        if !p.class_list().contains("is-open") {
            return;
        }
        let on_radio = ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).is_some_and(|i| i.name() == "theme");
        if ev.key() == "Escape" || (ev.key() == "Enter" && on_radio) {
            ev.prevent_default();
            ev.stop_propagation();
            s(false);
            let _ = t.focus();
        }
    });
    let (p, s) = (picker.clone(), set_open.clone());
    on::<Event>(&dom::document(), "click", move |ev| {
        if !dom::holds(&p, ev.target()) {
            s(false);
        }
    });
    let (p, s) = (picker.clone(), set_open.clone());
    on::<FocusEvent>(&picker, "focusout", move |ev| {
        if let Some(next) = ev.related_target()
            && !dom::holds(&p, Some(next))
        {
            s(false);
        }
    });
    if let Some(wide) = wide {
        on::<Event>(&wide, "change", move |_| set_open(false));
    }
}

/// `/` of the static export (GitHub Pages has no server to redirect): the language
/// chosen with the switch (its cookie), else the browser's, else French. The page lists
/// every language, and goes to French without the bundle (`<noscript>`).
pub fn pick_language() {
    let Some(page) = dom::query::<Element>("[data-pick-language]") else { return };
    let codes: Vec<String> = page.get_attribute("data-pick-language").unwrap_or_default().split(' ').map(str::to_owned).collect();
    let base = page.get_attribute("data-base").unwrap_or_default();
    let cookies = dom::document().dyn_into::<web_sys::HtmlDocument>().ok().and_then(|d| d.cookie().ok()).unwrap_or_default();
    let chosen = cookies.split("; ").find_map(|c| c.strip_prefix(&format!("{LANG_COOKIE}="))).filter(|c| codes.iter().any(|k| k == c));
    let navigator = dom::window().navigator();
    let asked = || {
        let languages: Vec<String> = navigator.languages().iter().filter_map(|l| l.as_string()).collect();
        let languages = if languages.is_empty() { vec![navigator.language().unwrap_or_default()] } else { languages };
        languages.into_iter().find_map(|l| {
            let short: String = l.chars().take(2).collect::<String>().to_lowercase();
            codes.iter().find(|k| short.len() == 2 && **k == short).cloned()
        })
    };
    let lang = chosen.map(str::to_owned).or_else(asked).unwrap_or_else(|| "fr".into());
    let _ = dom::window().location().replace(&format!("{base}/{lang}/"));
}
