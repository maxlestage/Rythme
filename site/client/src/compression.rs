//! Which files of `public/` are sent compressed, shared by the step of `../build.sh`
//! that compresses them once, at their best (`src/bin/precompress.rs`), and by the
//! server that sends them (`src/compress.rs` of the site). Plain Rust, without the
//! browser.
//!
//! Text and WebAssembly shrink to a third or a fifth of their size; images (PNG) are
//! compressed already. Next to `site.css`, the build writes `site.css.br` (Brotli) and
//! `site.css.gz` (gzip), each only when it saves something worth a header.

use std::path::Path;

/// The extensions worth compressing, with the type the server gives them (the same as
/// vitesse's static files, so that a compressed answer only adds `Content-Encoding`).
pub const TYPES: [(&str, &str); 11] = [
    ("css", "text/css; charset=utf-8"),
    ("js", "text/javascript; charset=utf-8"),
    ("wasm", "application/wasm"),
    ("svg", "image/svg+xml"),
    ("json", "application/json"),
    ("txt", "text/plain; charset=utf-8"),
    ("md", "text/markdown; charset=utf-8"),
    ("html", "text/html; charset=utf-8"),
    ("xml", "application/xml"),
    ("webmanifest", "application/manifest+json"),
    ("ico", "image/x-icon"),
];

/// Below this size the headers cost about what compression saves.
pub const MIN_SIZE: usize = 1024;

/// The two compressed forms, by the extension added to the file's name and the name of
/// the encoding (`Content-Encoding`), the preferred first.
pub const ENCODINGS: [(&str, &str); 2] = [("br", "br"), ("gz", "gzip")];

/// The type of a file worth compressing, from its extension; `None` for the others
/// (images, fonts already compressed, the compressed copies themselves).
pub fn content_type(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    TYPES.iter().find(|(e, _)| *e == ext).map(|(_, t)| *t)
}

/// Whether a compressed copy is worth keeping: a file of `MIN_SIZE` or more, and at
/// least 5% smaller.
pub fn worth(original: usize, compressed: usize) -> bool {
    original >= MIN_SIZE && compressed * 20 < original * 19
}

/// The path of a compressed copy: `site.css` and `br`, `site.css.br`.
pub fn copy_path(path: &Path, ext: &str) -> std::path::PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".");
    name.push(ext);
    name.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_and_webassembly_but_not_images() {
        assert_eq!(content_type(Path::new("pkg/site_bg.wasm")), Some("application/wasm"));
        assert_eq!(content_type(Path::new("site.CSS")), Some("text/css; charset=utf-8"));
        assert_eq!(content_type(Path::new("icons/icon-192.png")), None);
        assert_eq!(content_type(Path::new("site.css.br")), None);
        assert_eq!(content_type(Path::new("README")), None);
    }

    #[test]
    fn a_copy_must_save_something() {
        assert!(worth(10_000, 3_000));
        assert!(!worth(10_000, 9_600));
        assert!(!worth(1_000, 100), "too small to be worth a header");
        assert_eq!(copy_path(Path::new("public/site.css"), "br"), Path::new("public/site.css.br"));
    }
}
