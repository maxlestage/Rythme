//! The browser side of the Rythme website, in Rust compiled to WebAssembly: what the
//! pages do once the server has sent them. The server (the `rythme-site` crate) renders
//! every page with Paquets; this crate gives them their behavior, without a line of
//! hand-written JavaScript.
//!
//! The bundle `site` (`pkg/site.js`, `pkg/site_bg.wasm`, built by `../build.sh` with
//! the feature `page`) is loaded by every page: the burger menu, the language menu, the
//! copy buttons, the color themes, the installable app (the service worker, the "new
//! version" banner, the install button and the hint for iOS), and the choice of a
//! language at `/` of the static export ([`layout`]).
//!
//! The parts that need no browser are plain Rust, compiled without any feature: the
//! colors of Rythme code ([`highlight`], which the server uses too), the classes of the
//! interpreter's messages ([`messages`]), the steps of a calculation ([`steps`]), the
//! links that hold a program ([`share`]) and the text of *Copy for AI*
//! ([`assistant`]). The site's tests run them natively.

pub mod assistant;
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

/// Runs when the bundle is loaded (`init()` of the glue that wasm-bindgen writes): the
/// behavior of the page.
#[cfg(all(target_arch = "wasm32", feature = "page"))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    layout::start();
}
