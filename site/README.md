# The Rythme website

The website of Rythme, written in Rust with [Paquets](https://paquets-90dfc06b6fea.herokuapp.com/):
the server is a `paquets::App`, the pages are `paquets::Page`s built with active views,
rendered on the server with their SEO tags and a strict Content-Security-Policy.
In the six languages of the interpreter (see [Languages](#languages)), mobile first.
It installs as an app and works offline (see
[The installable app](#the-installable-app)), has color themes and never scrolls
sideways (see [Themes and layout](#themes-and-layout)). AI assistants and the tools
that read the web for them find the whole language in `/llms.txt`, every page as
Markdown and structured data (see [For AI assistants](#for-ai-assistants)).

| Path | Page |
|---|---|
| `/` | redirects to `/{lang}/`: the language chosen with the switch, else the first of `Accept-Language` the site has, else French |
| `/{lang}/` | home page |
| `/{lang}/learn/` | the language in two minutes; each piece of code opens in the playground |
| `/{lang}/examples/` | the programs of `examples/` with their output |
| `/{lang}/playground/` | the real interpreter, compiled to WebAssembly |
| `/{lang}/teach/` | the guide for teachers (`docs/teaching.md`, `docs/enseigner.md`) |
| `/{lang}/ai/` | for AI assistants: how to give them Rythme, the MCP server, the loop of `--format json` and `rythme fix` |
| `/{lang}/index.md`, `/{lang}/{page}/index.md` | the home page, *Learn*, the examples, the guide and the AI page as Markdown (`text/markdown`) |
| `/llms.txt`, `/llms-full.txt` | the index and the whole language for AI assistants (`docs/llms.txt`, `docs/llms-full.txt`) |
| `/{lang}/offline/` | shown by the service worker for a page it has not saved, without network |
| `/examples.json`, `/sitemap.xml`, `/robots.txt` | for the playground and search engines |
| `/{lang}/manifest.webmanifest`, `/sw.js` | the manifest of the app in each language, its service worker |

| Part | Where |
|---|---|
| The languages, their texts | `src/i18n.rs`, `src/i18n/<code>.rs` (a missing translation does not compile) |
| Pages, header with the burger menu and the language menu | `src/pages.rs` |
| Server and routes | `src/server.rs`, `src/routes.rs` |
| Rythme code colors, code blocks that wrap line by line | `src/highlight.rs` |
| Color themes: names, `theme-color`, the inline script of the head | `src/themes.rs` (colors in `public/site.css`) |
| Styles (mobile first), menu, playground | `public/site.css`, `public/site.js`, `public/playground.js` |
| Manifests, service worker | `src/pwa.rs`, `src/sw.js` |
| `llms.txt`, the MCP configurations, the AI page's commands | `src/ai.rs` (the page itself in `src/pages.rs`) |
| The pages as Markdown | `src/markdown.rs` |
| Structured data (JSON-LD) | `src/structured.rs`, written by `src/json.rs` (the manifests too) |
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

## Languages

French, English, Spanish, German, Portuguese (Brazilian, "você") and Chinese
(simplified): the languages of the interpreter (`LANGUAGES` in `src/lang.rs`).

- **Texts.** `src/i18n.rs` has `Lang` and the `Texts` structure; each language is a
  `Texts` constant in `src/i18n/<code>.rs`. Everything else that depends on the
  language (the names of the languages, the descriptions of the examples…) is an
  array of `i18n::COUNT` values given to `Lang::pick`: a missing text, or a missing
  language, does not compile. The pieces of code of the home page, of the playground
  and of the tour of *Learn* are written in each language (names, comments, texts):
  the tests run every one through the real interpreter in that language, and the
  error of the home page must be exactly the interpreter's message.
- **Addresses.** `/{code}/…` for each language; the pages have `lang` and `hreflang`
  set to their tag (`pt-BR`, `zh-Hans`: browsers pick the simplified Chinese glyphs),
  canonical and `hreflang` links for every language (`x-default`: French), as in the
  sitemap. `/` follows the cookie of the language switch, else `Accept-Language`
  (`zh-TW` and `pt-PT` go to `zh` and `pt`), else French; the static export does the
  same in the browser.
- **The language menu.** In the bar, a button with the code of the language opens the
  list of languages, each named in itself (a `details`: it works without JavaScript;
  `site.js` closes it with Escape, which gives the focus back to the button, a click
  elsewhere, or the focus leaving it). It hangs under the bar, on the right, never
  wider nor taller than the screen (it scrolls): at 320px the bar keeps one line. The footer lists every language.
- **The playground** gives the interpreter the language of the page (its position in
  `LANGUAGES`, `Lang::interpreter`, checked by the tests), so its messages, its help
  and its steps are in that language; its own texts come from `src/i18n/` (`data-ui`).
  It colors the messages by their first word in every language (`error`, `Fehler`,
  `错误`…), like `highlight.rs`; the tests check both lists against the catalogs.
- **The guide for teachers** exists in English and French. The other languages show
  the English guide (marked `lang="en"`) under a note in their language
  (`Texts::teach_note`, `teach::guide_lang`): a guide added later only needs its line in
  `teach.rs`.
- **Chinese.** The web fonts have no Chinese characters: on the pages in Chinese, a
  list of system fonts (`--cjk` in `site.css`: PingFang, Microsoft YaHei, Noto Sans
  CJK…) comes after the Latin font and before the generic family, in the text, the
  titles and the code; titles lose the tight letter spacing made for Latin letters.

Adding a language: its catalog in the interpreter first, then `src/i18n/<code>.rs`,
its line in `Lang` (and every array the compiler then asks for), the screenshots
(`tests/pwa/images.js`) and the lists of `tests/pwa/*.js`.

## The installable app

The site is a Progressive Web App, on Heroku (at `/`) and on GitHub Pages (behind
`/Rythme`) alike: every link, the manifests and the service worker are generated
behind `BASE_PATH`.

- **Manifest**: each page links the manifest of its language,
  `/{lang}/manifest.webmanifest` (`application/manifest+json`), so the app opens in
  the language it was installed from. One `id` (the base path) for all: it is one
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
  every page in every language, the offline pages, `site.css`, `site.js`,
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
  foreground); pages are revalidated with the server (`no-cache`), so a new version
  is seen at once. An installed app is rarely closed, so the new worker does not wait
  for a "next launch" that may never come: it comes in by itself (the page reloads)
  when it is ready before the visitor has touched anything since the app opened or
  came back, and when the app goes to the background with no field being edited and
  no program running; a new page opened while it waits takes it too. Otherwise a
  banner, "A new version is available — Reload", lets the visitor choose when; nothing
  reloads while something is being typed, and the playground keeps its draft in any
  case. Once activated, the new worker deletes the old caches.
- **Install**: an "Install app" button in the menu when the browser offers it
  (`beforeinstallprompt`: Chrome, Edge…), hidden once installed; on iPhone and iPad,
  a hint (Share, then "Add to Home Screen") until it is closed, never in the app. The
  pages have the tags of iOS (`apple-mobile-web-app-capable`, title, status bar,
  `apple-touch-icon`) and `viewport-fit=cover`: `site.css` keeps the content in the
  safe areas.
- **Playground**: the code being edited, the example shown and the name of the
  opened file stay in `localStorage`; *Open* reads a `.ry` file, *Download* saves the
  program as one. Everything works offline.

### Sharing links

*Share*, in the playground, makes a link that opens the program: the program is in
the fragment of the link itself (`#code=…`), which browsers never send to a server.
Nothing is stored anywhere, so the links work on Heroku, on GitHub Pages and offline
alike.

- **Format.** `#code=1:` then the program compressed with deflate (raw, RFC 1951),
  or `#code=0:` then the program as it is (UTF-8), both in URL-safe base64; *Share*
  takes the shorter. `&seed=N` follows when the program runs with a seed (see
  [Seed](#seed)), then `&mode=test` when Run runs the tests (`testing.ry`),
  `&mode=steps&step=N` when the steps of a calculation are shown (see below), with
  `&expr=…` (percent-encoded) when they are the steps of a selected piece of code.
  `#code=` followed by anything else is the program percent-encoded: the links of
  *Learn* and of the home page, written by `pages::playground_link` (and
  `pages::steps_link`), which stay readable in the address bar.
- **Compression.** The browser's `CompressionStream("deflate-raw")`; without it, the
  plain form. To read a link, `DecompressionStream`, else a small inflate written in
  `playground.js` (stored, fixed and dynamic blocks). No library, no inline script:
  the Content-Security-Policy does not change.
- **Copying.** The Clipboard API; when it says no, the link is shown selected in a
  field to copy by hand (the field is always there, with *Copy* and *Close*;
  Escape closes it). On a phone (`pointer: coarse`) with the Web Share API, the
  share sheet of the system.
- **Sizes.** A link holds at most 8,000 characters of program (packed): past that,
  *Share* says so and suggests *Download*. A link longer than 32,000 characters, a
  program over 200 KB once unpacked (checked while decompressing) or a damaged link
  are refused with a notice, and the draft is shown.
- **The draft.** A program opened from a link is shown and run without touching the
  draft saved in this browser: a notice says so, with *Back to my draft*. The first
  change makes it the draft (and takes the program out of the address); until then
  it stays in the address, so a reload shows it again. A link pasted while the
  playground is open (only the fragment changes) opens too. A text shared with the
  installed app (`share_target`) becomes such a link.

`tests/pwa/check.js` shares, opens the link in another page offline, compares the
program and its output, checks the draft, the reload, *Back to my draft*, a change,
the test mode, a long program, a damaged link, the share sheet of a phone and the
browsers without compression streams (with links made by zlib: stored, fixed and
dynamic blocks); `layout.js` checks the notice and the link in every language,
theme and width.

### Steps

*Steps*, in the playground, shows how one expression is computed, like
`rythme steps`: the last expression of the program, or the selected code (computed
after the definitions of the program, like `rythme steps file.ry "fib(5)"`). The
other expressions of the program are not run. The steps are shown one at a time in
the place of the output: *Previous* and *Next*, a slider, the arrow keys, Home and
End; the step before stays above, faint; what the program printed meanwhile comes
with the step that follows it; an error comes after the last step. A live region
(`aria-live`) says each step ("Step 3 of 14: fib(2) + fib(1) + fib(2)"). Long lines
wrap like code blocks, under their indentation; the colors are the editor's.

- **The interpreter.** Mode 5 of `rythme_run` (`playground/src/lib.rs`): the program,
  U+0000, then the selected code (empty: the last expression). It answers with the
  text of `rythme steps` (lines up to 400 characters rather than 100), or U+0005 when
  there is no expression to show; `playground.js` cuts it into steps.
- **Links.** Run, Check and Test bring the output back. *Share* from the steps makes
  a link with `&mode=steps&step=N` (and `&expr=…`), which opens on that step;
  without `step`, on the first one. One step of *Learn* (`pages::TOUR_STEPS`) opens
  on the steps of `fib(4)` this way.

`tests/pwa/check.js` opens a steps link offline on its step, moves with the buttons,
the keys and the slider, shares from the steps (the mode, the step and the selected
code travel), and checks the selection, what is printed, an error, a program without
expression and the link of *Learn*; `layout.js` shows long and nested steps in every
language, theme and width.

### Seed

*Seed*, next to Run, makes the draws of `random` and `shuffle` repeatable: empty,
they change from run to run, as before; a whole number (up to 15 digits), every Run,
Test and Steps makes the same draws, like `rythme run --seed N`, and Test also the
same values of `check_all` and `sample`, like `rythme test --seed N` (same seed and
same version of Rythme: same draws). Check runs nothing and uses none.

- **The option.** A `details` among the buttons (shown by `playground.js`), whose
  button shows the seed in use; its panel, under the toolbar, holds a labelled
  field (digits only, `inputmode="numeric"`), the dice (*New seed*: a new seed, and
  a run with it) and a hint. Enter in the field runs; Escape or a click elsewhere
  closes it. After a run with a seed, the head of the output says it ("seed 42").
- **The interpreter.** `rythme_seed(seed)` (`playground/src/lib.rs`), called by the
  worker before each `rythme_run`: a whole number from 0 to 2^53, or -1 for none.
  It seeds `random` (`Interp::seed_random`) and, for the tests, `property_seed`.
- **Saved and shared.** The seed is kept in `localStorage` (`rythme-seed`) with
  the draft. *Share* adds `&seed=N` to the link; a link shows its own seed (none
  without `seed`), without touching the one saved here, until the program or the
  seed is changed (then both become the draft) or *Back to my draft*.

`tests/pwa/check.js` runs a program of draws offline: without seed the output
changes, with 42 it does not (after a reload too), Test gives the same failures
with the same seed and others with another, the dice, Share (`&seed=42`), the link
in another page (the same draws, the saved seed untouched), a test link with a
seed, a link without one, and a change of seed on a link; `layout.js` shows the
open option with the longest seed in every language, theme and width.

On GitHub Pages, `sw.js` comes with a cache of 10 minutes rather than `no-cache`:
browsers check service workers past their HTTP cache (`updateViaCache: "none"`).

### Checking it in a browser

`cargo test` checks the manifests (served as `application/manifest+json`, parsed,
URLs right at the root and behind `/Rythme`, every icon and screenshot present with
its size), the service worker (`no-cache`, every URL it saves answers), the tags of
every page and the offline pages, the theme picker and the hash of the inline script
in the policy of every page. The rest needs a real browser: the scripts of
[`tests/pwa/`](tests/pwa/README.md), with Playwright and Chromium, check that the app
is installable, works offline (every page in every language, the playground runs,
checks, tests and formats, with its messages in the language of the page; Copy for AI;
`llms.txt`), shows the offline page, and offers a new version, both with the server and with the static
export; and that no page scrolls sideways, in every language and theme, with the
contrast of each, and that the language menu fits the screen (`layout.js`); and that
real phones never scroll sideways either, text enlarged, page zoomed, in landscape,
keyboard open and every menu open, with fields at 16px and targets at 44px
(`mobile.js`). The `browser` job of CI runs all three.

## Themes and layout

**Themes.** The menu has a theme picker (in the burger panel on a phone, a button of
the bar on a computer): **Auto** (follows the system's light or dark), **Light**,
**Dark**, **High contrast** (white on black with a yellow accent, every text at 7:1
or more, WCAG AAA), **Sepia** (warm paper for long reading) and **Tempo** (a pink
beat on deep violet). Every color of the site is a custom property of `site.css`,
redefined under `:root[data-theme="…"]` (Auto has no attribute and follows
`prefers-color-scheme`): pages, code blocks and the playground's editor, tables,
the offline page, the update and install banners. The picker is a `fieldset` of real
radio buttons with their labels in every language: Tab reaches the chosen one, the
arrow keys choose, and on a computer Escape (or Enter) closes the panel and gives
the focus back to its button (`aria-expanded`). The choice is kept in `localStorage`
(`rythme-theme`, every access in `try`/`catch`) and applied before the first paint by
a tiny inline script of the head (`themes::head_script`), which also sets the
`theme-color` of the browser's bar; Paquets allows it in the Content-Security-Policy
by its hash, the policy stays strict. Its tabs follow each other (`storage` event).

| Theme | Text on paper | Lowest text pair |
|---|---|---|
| Light | 15.22:1 | 4.61:1 |
| Dark | 15.38:1 | 5.01:1 |
| High contrast | 21.00:1 | 7.61:1 |
| Sepia | 13.13:1 | 5.17:1 |
| Tempo | 16.46:1 | 5.64:1 |

**No horizontal scrolling**, at any width, and no element wider than the screen:
code blocks wrap line by line, each wrapped line two columns under its own
indentation (`highlight.rs` writes one block per line with its `--indent`, the hang
at most a third of the screen), runs of spaces wrap too (`break-spaces`); the
playground's editor wraps (the field and its colors under it share the same width and
reserved scrollbar gutter, and each line number takes the height of its line); the
tables of the home page and of the guide become cards on phones, each cell with the
title of its column (`data-label`). Nothing is hidden with `overflow-x: hidden`.

**Mobile first, as real phones show it.** A phone is not only a narrow window: the
system's font size enlarges the text (Android's Chrome zooms the page, so a phone of
360px at 200% is a window of 180px; iOS Safari and Chrome have their text zoom, every
text bigger in the same boxes), landscape leaves 390px of height, the keyboard half of
it, and iOS zooms into a field whose text is under 16px, after which the page scrolls
sideways. So:

- the base rules are for a phone; media queries only add room (`min-width`,
  `min-height`), never `max-width`;
- every word may break where the layout needs it (`overflow-wrap: anywhere` on the
  body: titles, buttons, labels), and every one-column grid is `minmax(0, 1fr)` (a
  grid column otherwise grows to the widest field, select or seed, and widens the
  page); selects and fields have `min-width: 0`; rows of buttons wrap;
- sizes follow the text (`em`, `rem`, `ch`, `min()`, `clamp()`), never a fixed width
  wider than a small phone; heights use `svh`/`dvh` (with a `vh` fallback);
- every field is 16px or more on touch screens, landscape and tablets included (the
  editor is 14px only with a mouse and no touch screen);
- every control is 44×44px or more for a finger (smaller only with `pointer: fine`);
  nothing is shown only on hover;
- the bar sticks to the top only when the screen is 520px high or more; the menu
  panel and the language list start under its real bottom (`--bar-bottom`, measured
  by `site.js`: it wraps when the text is enlarged), the language list hangs under the
  bar and scrolls when the screen is short; titles shrink with the height too, so a
  phone in landscape shows the playground's tools and editor;
- the playground: Run in sight when it opens, the page never scrolls by itself (only
  a run asked by a tap brings its output into view), and with the keyboard open the
  editor is no taller than the part of the screen above it (`--keyboard-free`, from
  the visual viewport).

`tests/pwa/layout.js` checks the widths and themes in Chromium, `tests/pwa/mobile.js`
the phones (see [`tests/pwa/`](tests/pwa/README.md)).

## For AI assistants

Assistants were trained before Rythme existed: the site gives them the language, and
tells people how to give it to them (Rythme's own tools for them are in section 7.10
of the manual).

- **`/llms.txt` and `/llms-full.txt`** (`text/plain; charset=utf-8`): `docs/llms.txt`
  and `docs/llms-full.txt`, built into the server (`include_str!`), at the root of
  the base path. In `/llms.txt` the link to the complete file leads to the site's
  own `/llms-full.txt`, and a *Website* section gives the Markdown of the pages, all
  as absolute addresses on `SITE_URL` (or the address of the request): the tests
  follow every one. Every page links both (`<link rel="alternate" type="text/plain">`),
  `robots.txt` names them, and the service worker saves them, so the AI page works
  offline with its links.
- **Markdown**: `/{lang}/index.md`, `/{lang}/learn/index.md`, `/{lang}/examples/index.md`,
  `/{lang}/teach/index.md` and `/{lang}/ai/index.md` (`text/markdown; charset=utf-8`),
  written by `src/markdown.rs` from the texts, code and examples of the HTML pages:
  one `#` title, the lead as a quote, the code fenced as ```` ```rythme ````, what it
  prints as ```` ```text ````, every link absolute, no HTML. The guide for teachers is
  its own Markdown, its links leading to the site. Each page announces its Markdown
  (`<link rel="alternate" type="text/markdown">`); the server sends the HTML page as
  canonical (`Link: <…>; rel="canonical"`). **Not in the sitemap**: it lists the
  canonical pages, each with its languages; the Markdown is the same content in
  another format, found from the page itself and from `/llms.txt`, and listing it
  would offer search engines duplicates. The playground, the offline and 404 pages
  have none. The static export writes them next to `index.html`.
- **Structured data** (JSON-LD, `src/structured.rs`): on the home page, Rythme as a
  `ComputerLanguage`, its interpreter as a `SoftwareApplication` (version from the
  interpreter's `Cargo.toml`, MIT license, the Releases page as `downloadUrl`,
  operating systems, free) and its `SoftwareSourceCode` (written in Rust, the
  repository); on *Learn*, the examples, the guide and the AI page, a `TechArticle`
  in the language of the page with its Markdown as `encoding`, and on *Learn* and
  the examples each piece of code as a `SoftwareSourceCode` in Rythme. JSON-LD is
  data, not a script: the Content-Security-Policy computes no hash for it and still
  forbids every inline script but the theme's.
- **The AI page** (`/{lang}/ai/`, *AI* in the menu and the footer): the cards of
  `llms-full.txt`, `llms.txt` and the Markdown, a first message to paste into a chat
  (with the address of `llms-full.txt`), the configurations of `rythme mcp` for
  Claude Code, Claude Desktop, Cursor and VS Code (`src/ai.rs`, the same text as the
  manual: a test checks it), the commands of the loop and what `rythme fix` does,
  `rythme new` and its `AGENTS.md`. Each text to copy has its button (`site.js`, as
  the commands of the home page) and wraps like code: nothing scrolls sideways.
- **Copy for AI**, in the playground, next to *Share*: the program and what its last
  run gave as Markdown, after a line in the language of the page that gives the
  address of `llms-full.txt`: the name of the file, the code fenced as `rythme`, then
  the command as in a terminal (`rythme run main.ry`, `rythme check`, `rythme test`,
  `rythme steps … "expr"`, with `--seed N`) and its output or its errors fenced as
  `text` (the program's own output when the code was changed since: none). Copied
  with the Clipboard API; when it says no, the text is shown selected in a field of
  the box of *Share* (Escape closes it and gives the focus back). Everything happens
  in the page: it works offline.

The response of a text (`llms.txt`, Markdown) has a policy of its own: nothing runs
in it, and the browser may style the text it shows. `tests/ai.rs` serves all of it
through Paquets' test client; `tests/pwa/check.js` uses it offline in Chromium.

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
`examples.json`, `sw.js`, the manifests, `llms.txt`, `llms-full.txt`, the Markdown of the
pages (`dist/<path>/index.md`) and `public/`. Every link is behind `BASE_PATH`. `/` picks the language
in the browser. In the repository settings, Pages must use **GitHub Actions** as its
source (with "Deploy from a branch", GitHub shows the README instead).

## Checks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The tests render every page in every language through Paquets' test client (status,
`lang`, canonical and `hreflang` links, burger menu, CSP), check the redirections, the
404 page, the sitemap and `/examples.json`, that every program of `examples/` is listed,
and run every piece of code the site shows through the real interpreter, in the language
of each page: the home page's error message must be exactly the interpreter's.
`tests/ai.rs` checks what the site gives AI assistants: `llms.txt` and its links, the
Markdown of every page (well formed, every link existing, every piece of Rythme code
run through the interpreter, printing exactly the text that follows it), the JSON-LD
of every page, the titles and descriptions, the AI page and Copy for AI.
