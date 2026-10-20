//! The browser side of the site: Rust too (`client/`), compiled to WebAssembly by
//! `./build.sh` into `public/pkg/`, with the glue that wasm-bindgen writes. This module
//! knows those files and writes the few lines that start them: a browser runs
//! WebAssembly only when a script asks for it.
//!
//! - every page loads `pkg/site.js` and `pkg/site_bg.wasm` (the menus, the themes, the
//!   installable app); the playground loads `pkg/playground.js` and
//!   `pkg/playground_bg.wasm` too (its editor and the interpreter: 3 MB, while its
//!   menus already work). The page starts each bundle with one line, `import init from …;
//!   init(…)`, the line that Paquets writes for its islands, allowed by its hash in
//!   the Content-Security-Policy;
//! - the playground runs programs in a web worker made from the same bundle, so that a
//!   program that never stops can be stopped: `/pkg/worker.js` is that same line (a
//!   worker starts from a script file, never from inline code);
//! - the service worker is the `sw` bundle: `/sw.js` loads its glue and its WebAssembly
//!   (`/pkg/sw_wasm.js`, the bytes of `pkg/sw_bg.wasm` in base64, compiled at once:
//!   a service worker must have its listeners when its script ends, so it cannot wait
//!   for a download) and calls `serve` with what the server knows (`pwa.rs`).
//!
//! Every URL is behind the base path and versioned (`?v=`), like the other files.

use crate::json::js_string;
use crate::routes::link;

/// A bundle of the pages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bundle {
    /// Every page but the playground.
    Site,
    /// The playground: its page and its worker.
    Playground,
}

impl Bundle {
    pub fn name(self) -> &'static str {
        match self {
            Bundle::Site => "site",
            Bundle::Playground => "playground",
        }
    }

    /// The glue written by wasm-bindgen, versioned.
    pub fn js(self, version: &str) -> String {
        format!("{}?v={version}", link(&format!("/pkg/{}.js", self.name())))
    }

    /// The WebAssembly of the bundle, versioned.
    pub fn wasm(self, version: &str) -> String {
        format!("{}?v={version}", link(&format!("/pkg/{}_bg.wasm", self.name())))
    }

    /// The line that starts the bundle: the glue's `init`, with the WebAssembly to load.
    pub fn start(self, version: &str) -> String {
        format!("import init from {};init({{module_or_path:{}}});", js_string(&self.js(version)), js_string(&self.wasm(version)))
    }

    /// The tags of the head that load and start the bundle, as Paquets writes them for
    /// islands: the glue and the WebAssembly downloaded while the page is read, then
    /// started.
    pub fn head(self, version: &str) -> String {
        format!(
            r#"<link rel="modulepreload" href="{js}"/><link rel="preload" href="{wasm}" as="fetch" type="application/wasm" crossorigin=""/><script type="module">{start}</script>"#,
            js = self.js(version),
            wasm = self.wasm(version),
            start = self.start(version),
        )
    }
}

/// The files of `public/` that the pages link with `?v=` (and that the service worker
/// saves): the styles and the bundles of the pages.
pub const PAGE_FILES: [&str; 4] = ["/pkg/site.js", "/pkg/site_bg.wasm", "/pkg/playground.js", "/pkg/playground_bg.wasm"];

/// The service worker's own bundle, loaded by `/sw.js`.
pub const SW_FILES: [&str; 2] = ["/pkg/sw.js", "/pkg/sw_bg.wasm"];

/// Where the playground's worker starts (served by the server, written by the export).
pub const WORKER_PATH: &str = "/pkg/worker.js";
/// The WebAssembly of the service worker, as a script (served by the server, written by
/// the export).
pub const SW_WASM_PATH: &str = "/pkg/sw_wasm.js";

/// The playground's worker, versioned.
pub fn worker_url(version: &str) -> String {
    format!("{}?v={version}", link(WORKER_PATH))
}

/// `/pkg/worker.js`: the playground's bundle, started in the worker.
pub fn worker_script(version: &str) -> String {
    Bundle::Playground.start(version)
}

/// `/sw.js`: the service worker's bundle, loaded at once, then `serve(config)`.
pub fn service_worker(version: &str, config: &str) -> String {
    let glue = format!("{}?v={version}", link("/pkg/sw.js"));
    let wasm = format!("{}?v={version}", link(SW_WASM_PATH));
    format!("importScripts({},{});wasm_bindgen.serve({config});\n", js_string(&glue), js_string(&wasm))
}

/// `/pkg/sw_wasm.js`: the WebAssembly of the service worker, compiled at once (see the
/// module's documentation). Empty when the bundle is not built.
pub fn service_worker_wasm(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    format!("wasm_bindgen.initSync({{module:Uint8Array.from(atob(\"{}\"),c=>c.charCodeAt(0))}});\n", base64(bytes))
}

/// Standard base64, with padding, as `atob` reads it.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (u32::from(chunk[0]) << 16) | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8) | u32::from(*chunk.get(2).unwrap_or(&0));
        for k in 0..4 {
            out.push(if k <= chunk.len() { ALPHABET[(n >> (18 - 6 * k)) as usize & 63] as char } else { '=' });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lines_that_start_the_bundles() {
        assert_eq!(Bundle::Site.start("abc"), r#"import init from "/pkg/site.js?v=abc";init({module_or_path:"/pkg/site_bg.wasm?v=abc"});"#);
        assert!(Bundle::Playground.head("v").ends_with(r#"<script type="module">import init from "/pkg/playground.js?v=v";init({module_or_path:"/pkg/playground_bg.wasm?v=v"});</script>"#));
        assert_eq!(service_worker("v", "{}"), "importScripts(\"/pkg/sw.js?v=v\",\"/pkg/sw_wasm.js?v=v\");wasm_bindgen.serve({});\n");
    }

    #[test]
    fn base64_as_atob_reads_it() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"a"), "YQ==");
        assert_eq!(base64(b"ab"), "YWI=");
        assert_eq!(base64(b"\0asm\xff"), "AGFzbf8=");
        assert!(service_worker_wasm(b"\0asm").starts_with("wasm_bindgen.initSync({module:Uint8Array.from(atob(\"AGFzbQ==\")"));
        assert_eq!(service_worker_wasm(b""), "");
    }
}
