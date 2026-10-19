# Checking the installable app in a browser

These scripts drive Chromium with [Playwright](https://playwright.dev). They are not
run by `cargo test`: they need Node.js, Playwright and its Chromium
(`npm install playwright && npx playwright install chromium`, anywhere; `NODE_PATH`
tells Node where). They use `HTTPS_PROXY` when it is set, for the Google Fonts.

## `check.js`: the whole app, in a real browser

```sh
cd site
cargo build --release
NODE_PATH=/path/to/node_modules node tests/pwa/check.js target/release/rythme-site
```

It runs twice: the server at the root (as on Heroku), then the static export behind
`/Rythme` served like GitHub Pages; add `server` or `export` to run one of them. Each
run works on a copy of `public/` in a temporary folder and checks:

1. on an iPhone, the hint to add the site to the home screen, until it is closed;
   *Share* hands the link to the share sheet (a Web Share API made by hand);
2. the service worker is activated, controls the page and has saved the site;
3. `Page.getInstallabilityErrors` gives no error and `Page.getAppManifest` parses,
   in every language; the install button appears when the browser offers to install
   (an event made by hand: headless Chromium never offers it);
4. offline (the site stopped and the browser offline): every page in every
   language, with its fonts and no horizontal scroll; the playground runs, checks,
   tests and formats, its examples menu and help work, a `.ry` file opens and
   downloads, the code survives a reload, a shared text opens; *Share* makes a link
   (copied, compressed, URL-safe) that opens the same program in another page, which
   runs, without replacing the draft until it is changed, after a reload too, with
   *Back to my draft*; the test mode travels with it, a program too long is refused,
   a damaged link is said so, and without `CompressionStream` and
   `DecompressionStream` compressed links still open (made by Share and by zlib) and
   Share makes plain ones; a link with `&mode=steps&step=3` opens on that step of
   the calculation, which moves with the buttons, the arrow keys, Home, End and the
   slider, says each step in its live region, and travels with *Share* (the step and
   the selected code), with what is printed, an error, a program without expression
   and the link of *Learn*; *Seed*: without it the draws change, with it two runs
   (and two test runs) give the same output, after a reload too, the dice, a link
   with `&seed=` reproduces the draws in another page without changing the seed
   saved there; in every language the
   playground shows its texts and the interpreter's messages in that language; the
   language menu (it lists every language) and the burger menu; an unknown page
   shows the offline page of its language;
5. a new version deployed while the playground is open: the banner, no reload on
   its own, *Reload* brings the new version and the old cache is gone;
6. no console error and no Content-Security-Policy violation on any page.

The exit code is the number of failed checks.

## `layout.js`: no horizontal scroll, the themes and their contrast

```sh
NODE_PATH=/path/to/node_modules node tests/pwa/layout.js target/release/rythme-site
```

The same two deployments (`server` or `export` to run one), with service workers
blocked. It checks:

1. no horizontal scrolling: every page in every language (home, learn, examples,
   playground, teach, offline, 404), at 320, 360, 375, 390, 414, 600, 768, 1024,
   1280, 1440 and 1920px, in each of the six themes: the document is no wider than
   the window, no element sticks out of it, none scrolls sideways, and the bar keeps
   one line;
2. the language menu of the bar, open in every language at 320, 375, 768 and
   1280px: it lists every language and stays inside the screen; Enter opens it,
   Escape closes it and gives the focus back to its button, a click elsewhere closes
   it;
3. the notice of a program opened from a link and the box of the link made by
   *Share*, open together in every language, at 320 to 1280px in every theme: no
   horizontal scroll, every text at the contrast of the theme;
   the steps of a calculation (long lines without spaces, a nested block, the step
   before and what it printed), opened from a link in every language, at 320 to
   1280px in every theme, on three steps: no horizontal scroll, every text at the
   contrast of the theme; the *Seed* option open with a seed of 15 digits, shown in
   the head of the output, the same way: its panel inside the screen;
4. the playground's editor wraps a long line, a long word and a long address, and
   each line number stays level with its wrapped line;
5. contrast: every pair of colors of each theme (text on paper, on cards and on the
   soft accent; buttons, banners and commands on their background) at 4.5:1, 7:1 in
   High contrast; then every visible text of every page, against the background it
   is drawn on, at 390 and 1280px;
6. the picker with the keyboard only, in the menu of a phone and in the bar of a
   computer: the focus on the chosen theme, the arrow keys choose, Escape closes and
   gives the focus back; the page and its `theme-color` follow; every option has its
   label in French; the choice is kept from page to page;
7. no flash: the stored theme (and its `theme-color`) is on `<html>` as soon as
   `<body>` exists, before any deferred script, on every kind of page; Auto follows
   the system; an unknown stored value falls back to Auto.

`serve.js` starts the site for both scripts.

## `images.js`: icons and screenshots

```sh
NODE_PATH=… node tests/pwa/images.js icons                  # public/icons/, public/favicon.ico
NODE_PATH=… node tests/pwa/images.js screenshots http://localhost:3000 [fr en …]
```

The icons come from `public/assets/logo.svg` (the maskable one keeps the beats in
the central circle); the screenshots of the install dialog are taken on a running
site, in every language (or the languages given), narrow (412×915) and wide
(1280×800). Commit what they make.
