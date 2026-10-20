#!/usr/bin/env bash
# Builds the website of Rythme (site/), as the Dockerfile does: its browser side (Rust
# compiled to WebAssembly, with the interpreter for the playground) and its server. Run
# on Heroku by RustConfig once the emk/rust buildpack has installed Rust; works by hand
# too, from the repository: ./heroku-build.sh. The Procfile starts
# site/target/release/rythme-site.
set -euo pipefail

# Cargo fetches the site's git dependencies (Paquets and the repositories it uses)
# through git. If some of them are private, the REPOS_TOKEN config var, a GitHub
# token that can read them, is given to git by a credential helper (the buildpack
# hands config vars over in ENV_DIR). The token stays out of every URL.
if [ -z "${REPOS_TOKEN:-}" ] && [ -n "${ENV_DIR:-}" ] && [ -f "$ENV_DIR/REPOS_TOKEN" ]; then
  REPOS_TOKEN=$(<"$ENV_DIR/REPOS_TOKEN")
fi
if [ -n "${REPOS_TOKEN:-}" ]; then
  export REPOS_TOKEN CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
    GIT_CONFIG_KEY_0=credential.https://github.com.helper \
    GIT_CONFIG_VALUE_0='!f() { test "$1" = get && echo username=x-access-token && echo "password=$REPOS_TOKEN"; }; f'
fi
cd "$(dirname "$0")"
# On Heroku, Cargo builds into the buildpack's cache (CARGO_TARGET_DIR), shared by
# the two builds below; here, each crate has its own target/.
if [ -n "${CARGO_TARGET_DIR:-}" ]; then
  CARGO_TARGET_DIR=$(realpath -m "$CARGO_TARGET_DIR")
  export CARGO_TARGET_DIR
fi

echo "-----> Building the browser side (Rust compiled to WebAssembly)"
# site/build.sh installs the wasm32 target and wasm-bindgen-cli (the version of
# site/Cargo.lock) when they are missing: a few minutes, once, then cached.
site/build.sh
echo "       site/public/pkg/ ready"

echo "-----> Building the server"
(cd site && cargo build --release --bin rythme-site)
mkdir -p site/target/release
if [ -n "${CARGO_TARGET_DIR:-}" ]; then
  cp "$CARGO_TARGET_DIR/release/rythme-site" site/target/release/
fi
echo "       site/target/release/rythme-site ready"
