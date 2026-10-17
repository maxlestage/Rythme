#!/bin/sh
# Builds what the site serves besides its own files: the Rythme interpreter compiled
# to WebAssembly (public/rythme.wasm), for the playground. The examples are built
# into the server itself.
# Needs: rustup target add wasm32-unknown-unknown
set -e
cd "$(dirname "$0")"
../playground/build.sh
cp ../playground/web/rythme.wasm public/rythme.wasm
echo "public/rythme.wasm ready: cargo run --release"
