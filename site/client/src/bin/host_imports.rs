//! A step of `../../build.sh`, between Cargo and wasm-bindgen, for the playground's
//! bundle: `host-imports <file.wasm>`.
//!
//! The interpreter asks its host for the clock as two WebAssembly imports of the module
//! `env` (`rythme_now` and `rythme_offset`, in `rythme::io`): the standalone playground
//! (`playground/web`) gives them from its page. In the site's bundle they are Rust
//! (`src/playground/worker.rs`), exported under the same names; but the linker keeps
//! the imports (Rust gives imports of a module names of their own), and wasm-bindgen
//! would make them an import of a JavaScript module `env`, which does not exist. This
//! replaces each import of `env` by a call to the export of the same name, with
//! walrus, the library that wasm-bindgen itself uses to read and write WebAssembly.

use walrus::{ExportItem, ImportKind, Module};

type Error = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), Error> {
    let path = std::env::args().nth(1).ok_or("usage: host-imports <file.wasm>")?;
    let mut module = Module::from_file(&path)?;
    let imports: Vec<(String, ImportKind)> =
        module.imports.iter().filter(|i| i.module == "env").map(|i| (i.name.clone(), i.kind.clone())).collect();
    for (name, kind) in imports {
        let ImportKind::Function(import) = kind else {
            return Err(format!("env.{name} is not a function").into());
        };
        let export = module.exports.iter().find(|e| e.name == name).map(|e| e.item);
        let Some(ExportItem::Function(target)) = export else {
            return Err(format!("env.{name}: no function exported under that name").into());
        };
        module.replace_imported_func(import, |(body, args)| {
            for arg in args {
                body.local_get(*arg);
            }
            body.call(target);
        })?;
        println!("{path}: env.{name} is now the function of the bundle");
    }
    Ok(module.emit_wasm_file(&path)?)
}
