#!/bin/sh
# Builds the web playground into playground/web:
#   rythme.wasm     the interpreter compiled to WebAssembly
#   examples.json   the programs of examples/, shown in the page's menu
# Needs: rustup target add wasm32-unknown-unknown
set -e
cd "$(dirname "$0")"
cargo build --release
target="${CARGO_TARGET_DIR:-target}"
cp "$target/wasm32-unknown-unknown/release/rythme_playground.wasm" web/rythme.wasm
python3 - <<'PY'
import json, pathlib
root = pathlib.Path("..")
examples = {}
for path in sorted((root / "examples").glob("*.ry")):
    examples[path.stem] = path.read_text(encoding="utf-8")
pathlib.Path("web/examples.json").write_text(json.dumps(examples, ensure_ascii=False, indent=1), encoding="utf-8")
PY
echo "playground built in $(pwd)/web"
