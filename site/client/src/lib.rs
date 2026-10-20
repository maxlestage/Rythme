//! The browser side of the Rythme website, in Rust compiled to WebAssembly: what the
//! pages do once the server has sent them. The server (the `rythme-site` crate) renders
//! every page with Paquets; this crate gives them their behavior, without a line of
//! hand-written JavaScript.
//!
//! Bundles come out of it (`../build.sh`), each with its feature:
//!
//! - `page` (`pkg/site.js`, `pkg/site_bg.wasm`), loaded by every page but the
//!   playground: the burger menu, the language menu, the copy buttons, the color
//!   themes, the installable app (the service worker, the "new version" banner, the
//!   install button and the hint for iOS), and the choice of a language at `/` of the
//!   static export ([`layout`]);
//! - `playground` (`pkg/playground.js`, `pkg/playground_bg.wasm`), loaded by the
//!   playground: the same, its editor, and the interpreter itself, which runs in a web
//!   worker made from the same bundle so that a program that never stops can be
//!   stopped (`playground`).
//!
//! The parts that need no browser are plain Rust, compiled without any feature: the
//! colors of Rythme code ([`highlight`], which the server uses too), the classes of the
//! interpreter's messages ([`messages`]), the steps of a calculation ([`steps`]), the
//! links that hold a program ([`share`]), the text of *Copy for AI* ([`assistant`])
//! and the indentation and file names of the editor ([`editor`]). The site's tests run
//! them natively.

pub mod assistant;
pub mod editor;
pub mod highlight;
pub mod messages;
pub mod share;
pub mod steps;

// Helpers that the bundles share: each uses a part of them.
#[cfg(feature = "page")]
#[allow(dead_code)]
mod dom;
#[cfg(any(feature = "page", feature = "sw"))]
#[allow(dead_code)]
mod js;

#[cfg(feature = "page")]
pub mod layout;
#[cfg(feature = "playground")]
mod playground;

/// Runs when the bundle is loaded (`init()` of the glue that wasm-bindgen writes): in a
/// page, its behavior; in the playground's worker, the interpreter waiting for programs.
#[cfg(all(target_arch = "wasm32", feature = "page"))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    use wasm_bindgen::JsCast;
    if js_sys::global().is_instance_of::<web_sys::Window>() {
        layout::start();
        #[cfg(feature = "playground")]
        playground::start();
    } else {
        #[cfg(feature = "playground")]
        playground::worker::start();
    }
}
