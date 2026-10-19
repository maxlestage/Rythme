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
   playground shows its texts and the interpreter's messages in that language;
   `llms.txt` and `llms-full.txt` (with the addresses of the site), the copy button
   of an MCP configuration on the AI page, and *Copy for AI*: after Run, Check and
   Test, the program and its output or errors as Markdown with the address of
   `llms-full.txt`, no output once the code is changed, and without a clipboard the
   text shown selected, Escape closing the box; the
   language menu (it lists every language) and the burger menu; an unknown page
   shows the offline page of its language;
5. a new version deployed while the playground is open and being typed in: the
   banner, no reload on its own, *Reload* brings the new version and the old cache is
   gone; found before anything is touched (the app opening or coming back), it comes
   in by itself with the code kept; touched, the banner, then the app going to the
   background takes it; a field being edited when the app leaves: no reload, and the
   next launch (a new page) takes the waiting version by itself, leaving the page
   being edited alone;
6. no console error and no Content-Security-Policy violation on any page.

The exit code is the number of failed checks.

## `layout.js`: no horizontal scroll, the themes and their contrast

```sh
NODE_PATH=/path/to/node_modules node tests/pwa/layout.js target/release/rythme-site
```

The same two deployments (`server` or `export` to run one), with service workers
blocked. It checks:

1. no horizontal scrolling: every page in every language (home, learn, examples,
   playground, teach, AI, offline, 404), at 320, 360, 375, 390, 414, 600, 768, 1024,
   1280, 1440 and 1920px, in each of the six themes: the document is no wider than
   the window, no element sticks out of it, none scrolls sideways, the bar keeps
   one line, and under 700px every text field is at 16px or more (iOS zooms into a
   smaller one, and the page then scrolls sideways);
2. the language menu of the bar, open in every language at 320, 375, 768 and
   1280px: it lists every language and stays inside the screen; Enter opens it,
   Escape closes it and gives the focus back to its button, a click elsewhere closes
   it;
3. the notice of a program opened from a link and the box of the link made by
   *Share*, open together in every language, at 320 to 1280px in every theme: no
   horizontal scroll, every text at the contrast of the theme; *Copy for AI* when the
   clipboard says no (the Markdown shown, focused, in its field), in every language,
   at 320 to 1280px in every theme, the same way;
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

## `mobile.js`: as real phones show it

```sh
NODE_PATH=/path/to/node_modules node tests/pwa/mobile.js target/release/rythme-site
MOBILE_SHOTS=/some/folder NODE_PATH=… node tests/pwa/mobile.js …   # with screenshots
```

Chromium emulating phones (`isMobile`, `hasTouch`: the meta viewport applies, and an
element wider than the screen widens the window, which the phone then zooms out), the
same two deployments, service workers blocked. Before them, it reads `site.css`:
media queries only add room (`min-width`, `min-height`), no width of `100vw`, nothing
shown only on hover, no text kept on one line unless its overflow is hidden. Then:

1. every page in every language on twelve phones: 320, 360, 375, 390 and 412px
   portrait, 844×390 and 915×412 landscape, the page zoomed (150% of 390px, 200% of
   412 and of 360px: windows of 260, 206 and 180px, as Chrome on Android does with the
   system's font size) and the text alone enlarged (150% at 320px, 200% at 390px: every
   font size times that, as WebKit's text zoom does): the window as wide as the screen
   (not zoomed out), the document no wider, `scrollBy` does not move sideways, no
   element sticks out or scrolls sideways, and every text field (shown or not) at 16px
   or more, else iOS zooms into it; the playground opens with Run in sight and the
   page not scrolled;
2. every menu and panel open, in every language, at 320 and 390px, in landscape, at
   200% page zoom and 200% text: the burger menu (under the real bottom of the bar,
   with the theme picker), the language menu (inside the screen, reachable), the
   banners of the app all shown, and in the playground the Seed option with a seed of
   15 digits and long output lines, the box of a shared link, the text of *Copy for
   AI* to copy by hand, a topic of the help, the
   steps of a calculation, a long error;
3. the keyboard, at 390×844 and 844×390: the visible height shrinks, the editor fits
   above it while typing, nothing scrolls sideways, and it gets its height back once
   closed;
4. every control (links outside running text, buttons, fields, the options of the
   theme picker) at 44×44px or more, in French, English, German and Chinese, with the
   menu closed and open.

The pages are visited at a human pace: the server answers 100 requests a second to
one visitor.

## `images.js`: icons and screenshots

```sh
NODE_PATH=… node tests/pwa/images.js icons                  # public/icons/, public/favicon.ico
NODE_PATH=… node tests/pwa/images.js screenshots http://localhost:3000 [fr en …]
```

The icons come from `public/assets/logo.svg` (the maskable one keeps the beats in
the central circle); the screenshots of the install dialog are taken on a running
site, in every language (or the languages given), narrow (412×915) and wide
(1280×800). Commit what they make.
