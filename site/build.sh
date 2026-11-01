#!/bin/sh
# Builds the browser side of the site into public/pkg/: Rust (client/) compiled to
# WebAssembly, with the glue that wasm-bindgen writes, in three bundles:
#   site        every page: menus, themes, the installable app
#   playground  the playground: the same, its editor and the interpreter
#   sw          the service worker
# and the catalogs of the interpreter's messages in the languages it does not compile in
# (catalog-de.txt…), then compresses the files of public/ worth it, once, at their best
# (Brotli and gzip copies next to them, for the server). The examples are built into the
# server itself.
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
out="$target/wasm32-unknown-unknown/wasm-release"
mkdir -p public/pkg
for bundle in site:page:web playground:playground:web sw:sw:no-modules; do
  name=${bundle%%:*}
  rest=${bundle#*:}
  feature=${rest%%:*}
  kind=${rest#*:}
  cargo build -p rythme-site-client --lib --target wasm32-unknown-unknown --profile wasm-release --features "$feature"
  cp "$out/rythme_site_client.wasm" "$out/$name.wasm"
  if [ "$name" = playground ]; then
    # The interpreter's clock, from the bundle itself (see client/src/bin/host_imports.rs).
    cargo run --release -p rythme-site-client --features host-imports --bin host-imports -- "$out/$name.wasm"
  fi
  wasm-bindgen --target "$kind" --no-typescript --out-dir public/pkg "$out/$name.wasm"
  # Optional: binaryen's wasm-opt, when installed, shrinks the bundle a little more
  # (the features are the ones Rust uses for wasm32 by default). Without it, or if it
  # fails, the bundle stays as Rust made it.
  if command -v wasm-opt > /dev/null; then
    wasm="public/pkg/${name}_bg.wasm"
    if wasm-opt -Oz --enable-bulk-memory --enable-mutable-globals --enable-nontrapping-float-to-int --enable-sign-ext \
      --enable-reference-types --enable-multivalue "$wasm" -o "$wasm.opt"; then
      mv "$wasm.opt" "$wasm"
    else
      rm -f "$wasm.opt"
      echo "wasm-opt failed: $wasm kept as it is"
    fi
  fi
done

# The catalogs of the interpreter's messages, one per language besides English and
# French (compiled into the playground's interpreter), which the playground fetches
# (see client/src/bin/catalogs.rs).
cargo run -p rythme-site-client --features catalogs --bin catalogs -- public/pkg

# Brotli and gzip copies of the files worth it, at their best, for the server (see
# client/src/bin/precompress.rs; GitHub Pages compresses with gzip by itself).
cargo run --release -p rythme-site-client --features precompress --bin precompress -- public

echo "public/pkg/ ready: cargo run --release"
