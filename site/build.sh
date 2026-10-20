#!/bin/sh
# Builds the browser side of the site into public/pkg/: Rust (client/) compiled to
# WebAssembly, with the glue that wasm-bindgen writes, in three bundles:
#   site        every page: menus, themes, the installable app
#   playground  the playground: the same, its editor and the interpreter
#   sw          the service worker
# And, for the time being, the interpreter of the JavaScript playground
# (public/rythme.wasm). The examples are built into the server itself.
#
# Needs the wasm32-unknown-unknown target of Rust and wasm-bindgen-cli at exactly the
# version of the wasm-bindgen crate in Cargo.lock: both are installed when missing.
set -e
cd "$(dirname "$0")"

# The version of wasm-bindgen in Cargo.lock: the command line tool must be the same.
version=$(sed -n '/^name = "wasm-bindgen"$/{n;s/^version = "\(.*\)"$/\1/p;}' Cargo.lock)
if command -v rustup > /dev/null; then
  rustup target add wasm32-unknown-unknown
fi
if [ "$(wasm-bindgen --version 2> /dev/null)" != "wasm-bindgen $version" ]; then
  echo "Installing wasm-bindgen-cli $version (once)"
  cargo install wasm-bindgen-cli --version "$version" --locked
fi

target="${CARGO_TARGET_DIR:-target}"
mkdir -p public/pkg
for bundle in site:page:web; do
  name=${bundle%%:*}
  rest=${bundle#*:}
  feature=${rest%%:*}
  kind=${rest#*:}
  cargo build -p rythme-site-client --lib --target wasm32-unknown-unknown --profile wasm-release --features "$feature"
  wasm-bindgen --target "$kind" --no-typescript --out-dir public/pkg --out-name "$name" \
    "$target/wasm32-unknown-unknown/wasm-release/rythme_site_client.wasm"
done

(cd ../playground && cargo build --release)
cp "${CARGO_TARGET_DIR:-../playground/target}/wasm32-unknown-unknown/release/rythme_playground.wasm" public/rythme.wasm

echo "public/pkg/ ready: cargo run --release"
