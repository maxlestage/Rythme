# The Rythme website

The website of Rythme, written in Rust with [Paquets](https://paquets-90dfc06b6fea.herokuapp.com/):
the server is a `paquets::App`, the pages are `paquets::Page`s built with active views,
rendered on the server with their SEO tags and a strict Content-Security-Policy.
French and English, mobile first.

| Path | Page |
|---|---|
| `/` | redirects to `/fr/` or `/en/` (the language chosen with the switch, else `Accept-Language`) |
| `/{lang}/` | home page |
| `/{lang}/learn/` | the language in two minutes; each piece of code opens in the playground |
| `/{lang}/examples/` | the programs of `examples/` with their output |
| `/{lang}/playground/` | the real interpreter, compiled to WebAssembly |
| `/{lang}/teach/` | the guide for teachers (`docs/teaching.md`, `docs/enseigner.md`) |
| `/examples.json`, `/sitemap.xml`, `/robots.txt` | for the playground and search engines |

| Part | Where |
|---|---|
| Texts in both languages | `src/i18n.rs` (a missing translation does not compile) |
| Pages, header with the burger menu and the language switch | `src/pages.rs` |
| Server and routes | `src/server.rs`, `src/routes.rs` |
| Rythme code colors | `src/highlight.rs` |
| Styles (mobile first), menu, playground | `public/site.css`, `public/site.js`, `public/playground.js` |

The examples and the guide are built into the server (`include_str!`): nothing to copy
when deploying, besides `public/`.

## Build and run

From this folder:

```sh
rustup target add wasm32-unknown-unknown
./build.sh                 # public/rythme.wasm, the interpreter for the playground
cargo run --release        # http://localhost:3000 (PORT on Heroku)
```

Paquets is fetched from its git repository. Without `public/rythme.wasm` the site works,
except the playground. `SITE_URL` sets the public address used in canonical and Open
Graph links (taken from each request otherwise).

## Checks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The tests render every page in both languages through Paquets' test client (status,
`lang`, canonical and `hreflang` links, burger menu, CSP), check the redirections, the
404 page, the sitemap and `/examples.json`, that every program of `examples/` is listed,
and run every piece of code the site shows through the real interpreter, in French and
English: the home page's error message must be exactly the interpreter's.
