//! A step of `../../build.sh`, run natively: `catalogs <dir>` (`public/pkg/`).
//!
//! The playground's interpreter has its messages and its help in English and French
//! only (the feature `runtime-catalogs` of Rythme): every language compiled in would be
//! most of its download, while a visitor reads one. This step writes the catalog of
//! each other language, `catalog-de.txt`…, from the interpreter compiled with all of
//! them (`rythme::lang::catalog`); the page fetches the one of its language and gives
//! it to the interpreter (`rythme::lang::register`). Their names are those of
//! `CATALOG_FILES` in `src/client.rs` of the site.

use std::path::Path;

use rythme::lang::{Lang, catalog};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args().nth(1).ok_or("usage: catalogs <dir>")?;
    std::fs::create_dir_all(&dir)?;
    for lang in Lang::all() {
        let file = catalog(lang);
        if file.is_empty() {
            continue;
        }
        let path = Path::new(&dir).join(format!("catalog-{}.txt", lang.code()));
        std::fs::write(&path, &file)?;
        println!("{}: {} bytes", path.display(), file.len());
    }
    Ok(())
}
