//! The bundle of the playground runs the interpreter, whose stack lives in the
//! WebAssembly memory: room for it, as `playground/.cargo/config.toml` gives the
//! standalone playground. The other bundles keep the default stack (1 MiB): every page
//! loads them.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|arch| arch == "wasm32");
    if wasm && std::env::var_os("CARGO_FEATURE_PLAYGROUND").is_some() {
        println!("cargo:rustc-link-arg-cdylib=-zstack-size=16777216");
    }
}
