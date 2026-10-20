//! The page: finding its elements, listening to them, the browser's storage.

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Document, Element, Event, EventTarget, HtmlElement, Storage, Window};

pub fn window() -> Window {
    web_sys::window().expect("a window")
}

pub fn document() -> Document {
    window().document().expect("a document")
}

/// `<html>`.
pub fn root() -> HtmlElement {
    document().document_element().expect("<html>").unchecked_into()
}

pub fn body() -> HtmlElement {
    document().body().expect("<body>")
}

/// The element with this id, as the type asked for.
pub fn by_id<T: JsCast>(id: &str) -> Option<T> {
    document().get_element_by_id(id).and_then(|e| e.dyn_into().ok())
}

/// The element with this id, which the page has (the server writes it).
pub fn id<T: JsCast>(id: &str) -> T {
    by_id(id).unwrap_or_else(|| panic!("#{id}"))
}

/// The first element that matches `selector`.
pub fn query<T: JsCast>(selector: &str) -> Option<T> {
    document().query_selector(selector).ok().flatten().and_then(|e| e.dyn_into().ok())
}

/// The first element under `parent` that matches `selector`.
pub fn query_in<T: JsCast>(parent: &Element, selector: &str) -> Option<T> {
    parent.query_selector(selector).ok().flatten().and_then(|e| e.dyn_into().ok())
}

/// Every element under `parent` (the document when `None`) that matches `selector`.
pub fn query_all<T: JsCast>(parent: Option<&Element>, selector: &str) -> Vec<T> {
    let list = match parent {
        Some(p) => p.query_selector_all(selector),
        None => document().query_selector_all(selector),
    };
    let Ok(list) = list else { return Vec::new() };
    (0..list.length()).filter_map(|i| list.item(i)).filter_map(|n| n.dyn_into().ok()).collect()
}

/// Calls `handler` at each `event` of `target`, for as long as the page lives.
pub fn on<E: JsCast + 'static>(target: &EventTarget, event: &str, handler: impl FnMut(E) + 'static) {
    listen(target, event, handler, false);
}

/// The same, in the capture phase (before the element's own listeners).
pub fn on_capture<E: JsCast + 'static>(target: &EventTarget, event: &str, handler: impl FnMut(E) + 'static) {
    listen(target, event, handler, true);
}

fn listen<E: JsCast + 'static>(target: &EventTarget, event: &str, mut handler: impl FnMut(E) + 'static, capture: bool) {
    let closure = Closure::<dyn FnMut(Event)>::new(move |ev: Event| handler(ev.unchecked_into()));
    let _ = target.add_event_listener_with_callback_and_bool(event, closure.as_ref().unchecked_ref(), capture);
    // The listener stays for the life of the page.
    closure.forget();
}

/// Runs `f` after `ms` milliseconds.
pub fn later(ms: i32, f: impl FnOnce() + 'static) {
    let closure = Closure::once_into_js(f);
    let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), ms);
}

/// Whether the media query matches (`(min-width: 860px)`…).
pub fn media(query: &str) -> bool {
    window().match_media(query).ok().flatten().is_some_and(|m| m.matches())
}

fn storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

/// A value of `localStorage`; none when the storage is blocked.
pub fn load(key: &str) -> Option<String> {
    storage()?.get_item(key).ok().flatten()
}

/// Keeps a value in `localStorage`, if the browser lets it.
pub fn save(key: &str, value: &str) {
    if let Some(storage) = storage() {
        let _ = storage.set_item(key, value);
    }
}

/// `el.hidden = hidden`, for any element.
pub fn set_hidden(el: &Element, hidden: bool) {
    if let Some(el) = el.dyn_ref::<HtmlElement>() {
        el.set_hidden(hidden);
    } else if hidden {
        let _ = el.set_attribute("hidden", "");
    } else {
        let _ = el.remove_attribute("hidden");
    }
}

pub fn is_hidden(el: &Element) -> bool {
    el.has_attribute("hidden")
}

/// The element that has the focus.
pub fn active() -> Option<Element> {
    document().active_element()
}

/// Whether `container` holds the target of an event (or is it).
pub fn holds(container: &Element, target: Option<EventTarget>) -> bool {
    let node = target.and_then(|t| t.dyn_into::<web_sys::Node>().ok());
    container.contains(node.as_ref())
}

/// A milliseconds clock, for the times of the runs.
pub fn now() -> f64 {
    window().performance().map_or(0.0, |p| p.now())
}
