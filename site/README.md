# The Rythme website

The website of Rythme, written in Rust with [Paquets](https://paquets-90dfc06b6fea.herokuapp.com/):
the server is a `paquets::App`, the pages are `paquets::Page`s built with active views,
rendered on the server with their SEO tags and a strict Content-Security-Policy.
French and English, mobile first. It installs as an app and works offline (see
[The installable app](#the-installable-app)).

| Path | Page |
|---|---|
| `/` | redirects to `/fr/` or `/en/` (the language chosen with the switch, else `Accept-Language`) |
| `/{lang}/` | home page |
| `/{lang}/learn/` | the language in two minutes; each piece of code opens in the playground |
| `/{lang}/examples/` | the programs of `examples/` with their output |
| `/{lang}/playground/` | the real interpreter, compiled to WebAssembly |
| `/{lang}/teach/` | the guide for teachers (`docs/teaching.md`, `docs/enseigner.md`) |
| `/{lang}/offline/` | shown by the service worker for a page it has not saved, without network |
| `/examples.json`, `/sitemap.xml`, `/robots.txt` | for the playground and search engines |
| `/{lang}/manifest.webmanifest`, `/sw.js` | the manifest of the app in each language, its service worker |

| Part | Where |
|---|---|
| Texts in both languages | `src/i18n.rs` (a missing translation does not compile) |
| Pages, header with the burger menu and the language switch | `src/pages.rs` |
| Server and routes | `src/server.rs`, `src/routes.rs` |
| Rythme code colors | `src/highlight.rs` |
| Styles (mobile first), menu, playground | `public/site.css`, `public/site.js`, `public/playground.js` |
| Manifests, service worker | `src/pwa.rs`, `src/sw.js` |
| Icons, `favicon.ico`, screenshots of the install dialog | `public/icons/`, `public/screenshots/`, made by `tests/pwa/images.js` |

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

## The installable app

The site is a Progressive Web App, on Heroku (at `/`) and on GitHub Pages (behind
`/Rythme`) alike: every link, the manifests and the service worker are generated
behind `BASE_PATH`.

- **Manifest**: each page links the manifest of its language,
  `/{lang}/manifest.webmanifest` (`application/manifest+json`), so the app opens in
  the language it was installed from. One `id` (the base path) for both: it is one
  app. Its URLs are relative to it (`start_url` `../fr/`, `scope` `../`), so they
  hold under any base path. It has the icons (PNG 192 and 512, a maskable 512 and the
  SVG), screenshots (narrow and wide, for the rich install dialog of Chrome),
  shortcuts (Playground, Examples, Learn), a `file_handlers` for `.ry` files (the
  playground opens them through `launchQueue`, Chrome and Edge on computers) and a
  `share_target` (a text shared with the app opens in the playground:
  `?title=…&text=…&url=…`). Browsers that do not know a field ignore it.
- **Service worker** (`/sw.js`, `no-cache`): `src/sw.js` with the constants the
  server writes: the version (a hash of everything it saves: the pages as rendered,
  the files, the manifests), the base path and the list of URLs to save at install:
  every page in both languages, the offline pages, `site.css`, `site.js`,
  `playground.js`, `playground-worker.js` and `rythme.wasm` at their `?v=` version,
  `examples.json`, the manifests and the icons. Versioned files come from the cache
  without asking the network; other files of the site from the cache, refreshed in
  the background; pages from the network first, from the cache after 3 seconds or
  without network, else the offline page of their language (`/` goes to the home
  page of the language chosen with the switch, else the browser's). Errors,
  redirections and other sites are never saved, except the Google Fonts of the
  pages, kept in a cache of their own (`rythme-fonts`) so that the site looks the
  same offline; without them it falls back to system fonts. Its own
  Content-Security-Policy lets it fetch them; the pages keep theirs.
- **Updates**: a new deployment changes the version, so the browser installs the new
  worker in the background (at each visit, and when the app comes back to the
  foreground). It waits: a banner, "A new version is available — Reload", lets the
  visitor choose when; nothing reloads on its own, so nobody loses what they were
  typing. Once activated, the new worker deletes the old caches.
- **Install**: an "Install app" button in the menu when the browser offers it
  (`beforeinstallprompt`: Chrome, Edge…), hidden once installed; on iPhone and iPad,
  a hint (Share, then "Add to Home Screen") until it is closed, never in the app. The
  pages have the tags of iOS (`apple-mobile-web-app-capable`, title, status bar,
  `apple-touch-icon`) and `viewport-fit=cover`: `site.css` keeps the content in the
  safe areas.
- **Playground**: the code being edited, the example shown and the name of the
  opened file stay in `localStorage`; *Open* reads a `.ry` file, *Download* saves the
  program as one. Everything works offline.

On GitHub Pages, `sw.js` comes with a cache of 10 minutes rather than `no-cache`:
browsers check service workers past their HTTP cache (`updateViaCache: "none"`).

### Checking it in a browser

`cargo test` checks the manifests (served as `application/manifest+json`, parsed,
URLs right at the root and behind `/Rythme`, every icon and screenshot present with
its size), the service worker (`no-cache`, every URL it saves answers), the tags of
every page and the offline pages. The rest needs a real browser: the scripts of
[`tests/pwa/`](tests/pwa/README.md), with Playwright and Chromium, check that the app
is installable, works offline (every page in both languages, the playground runs,
checks, tests and formats), shows the offline page, and offers a new version, both
with the server and with the static export.

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
`examples.json`, `sw.js`, the manifests and `public/`. Every link is behind `BASE_PATH`. `/` picks the language
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
