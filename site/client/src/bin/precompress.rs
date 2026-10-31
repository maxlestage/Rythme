//! A step of `../../build.sh`, run natively once the bundles are built:
//! `precompress <dir>` (`public/`).
//!
//! Next to each file worth it (text and WebAssembly, `rythme_site_client::compression`),
//! it writes a copy compressed with Brotli at its best (`site.css.br`, quality 11) and
//! one with gzip at its best (`site.css.gz`, level 9), which the server sends to the
//! browsers that accept them (`src/compress.rs` of the site). Compressing this well
//! takes seconds (the playground's bundle is 3 MB): done once here, never per request.
//! A copy that would not save enough is not written, and an old one is removed; copies
//! of files that no longer exist are removed too.

use std::io::Write;
use std::path::{Path, PathBuf};

use rythme_site_client::compression::{ENCODINGS, content_type, copy_path, worth};

type Error = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), Error> {
    let dir = std::env::args().nth(1).ok_or("usage: precompress <dir>")?;
    let mut files = Vec::new();
    walk(Path::new(&dir), &mut files)?;
    files.sort();
    let (mut before, mut after) = (0, 0);
    for path in &files {
        let name = path.to_string_lossy();
        // A copy whose file is gone.
        if let Some(stem) = ENCODINGS.iter().find_map(|(ext, _)| name.strip_suffix(&format!(".{ext}")))
            && !Path::new(stem).is_file()
        {
            std::fs::remove_file(path)?;
            continue;
        }
        if content_type(path).is_none() {
            continue;
        }
        let bytes = std::fs::read(path)?;
        let mut sizes = Vec::new();
        for (ext, _) in ENCODINGS {
            let compressed = if ext == "br" { brotli(&bytes)? } else { gzip(&bytes)? };
            let copy = copy_path(path, ext);
            if worth(bytes.len(), compressed.len()) {
                sizes.push(format!("{ext} {}", compressed.len()));
                if ext == "br" {
                    before += bytes.len();
                    after += compressed.len();
                }
                std::fs::write(&copy, compressed)?;
            } else if copy.is_file() {
                std::fs::remove_file(&copy)?;
            }
        }
        if !sizes.is_empty() {
            println!("{name}: {} bytes, {}", bytes.len(), sizes.join(", "));
        }
    }
    println!("{dir}: {before} bytes compressed to {after} with Brotli");
    Ok(())
}

fn walk(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

/// Brotli at its best: quality 11, with a window of 16 MiB (the largest every browser
/// decodes), so that the whole bundle of the playground is one window.
fn brotli(bytes: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(bytes.len() / 3);
    let params = brotli::enc::BrotliEncoderParams { quality: 11, lgwin: 24, size_hint: bytes.len(), ..Default::default() };
    brotli::BrotliCompress(&mut &bytes[..], &mut out, &params)?;
    Ok(out)
}

fn gzip(bytes: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::with_capacity(bytes.len() / 3), flate2::Compression::best());
    encoder.write_all(bytes)?;
    encoder.finish()
}
