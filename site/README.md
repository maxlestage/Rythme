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

## Heroku

The repository deploys this site, with its playground, on Heroku in two ways:

| Way | Files | Stack |
|---|---|---|
| emk/rust buildpack (default) | `app.json`, `RustConfig`, `heroku-build.sh`, `Procfile` | `heroku-26` |
| Docker | `heroku.yml`, `Dockerfile` | `container` |

`heroku-build.sh` (run by the buildpack once Rust is installed, or by hand from the
repository) compiles `playground/` to `site/public/rythme.wasm`, then the server to
`site/target/release/rythme-site`; the `Procfile` starts it from `site/`. The
`Dockerfile` does the same in an image of about 120 MB, run as a non-root user.
The server listens on `PORT`, set by Heroku.

- **One tap:** the *Deploy to Heroku* button of the main README
  (`https://heroku.com/deploy?template=https://github.com/maxlestage/Rythme`).
- **Every merge into `master`:** the secret `HEROKU_API_KEY` and the variable
  `HEROKU_APP_NAME` in *Settings → Secrets and variables → Actions*; the workflow
  `.github/workflows/heroku.yml` builds through the Heroku Platform API and creates the
  app if needed. Optional variables: `HEROKU_REGION` (`us` or `eu`), `HEROKU_BUILD`
  (`buildpack` or `docker`). Without the secret, the workflow does nothing.
- **Command line:** `heroku create --stack heroku-26 --buildpack emk/rust` (or
  `--stack container` for Docker), then `git push heroku master`.

Config vars, all optional: `SITE_URL` (the public address, e.g. a custom domain;
taken from each request otherwise), `REPOS_TOKEN` (a GitHub token, only if some git
dependencies of Paquets are private; buildpack only). Heroku has no free plan: an Eco
dyno costs about $5 a month.

## GitHub Pages

The site is published at <https://maxlestage.github.io/Rythme/> by the `Website`
workflow (`.github/workflows/pages.yml`), as static files:

```sh
./build.sh
BASE_PATH=/Rythme SITE_URL=https://maxlestage.github.io/Rythme cargo run --release -- export dist
```

Every page is rendered by the app itself through Paquets' test client and written to
`dist/<path>/index.html`, with `404.html`, `sitemap.xml`, `robots.txt`,
`examples.json` and `public/`. Every link is behind `BASE_PATH`. `/` picks the language
in the browser. In the repository settings, Pages must use **GitHub Actions** as its
source (with "Deploy from a branch", GitHub shows the README instead).

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
