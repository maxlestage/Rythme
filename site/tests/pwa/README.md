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
2. the service worker is activated, controls the page and has saved the site;
3. `Page.getInstallabilityErrors` gives no error and `Page.getAppManifest` parses,
   in both languages; the install button appears when the browser offers to install
   (an event made by hand: headless Chromium never offers it);
4. offline (the site stopped and the browser offline): every page in both
   languages, with its fonts and no horizontal scroll; the playground runs, checks,
   tests and formats, its examples menu and help work, a `.ry` file opens and
   downloads, the code survives a reload, a shared text opens; the language switch
   and the burger menu; an unknown page shows the offline page of its language;
5. a new version deployed while the playground is open: the banner, no reload on
   its own, *Reload* brings the new version and the old cache is gone;
6. no console error and no Content-Security-Policy violation on any page.

The exit code is the number of failed checks.

## `images.js`: icons and screenshots

```sh
NODE_PATH=… node tests/pwa/images.js icons                  # public/icons/, public/favicon.ico
NODE_PATH=… node tests/pwa/images.js screenshots http://localhost:3000
```

The icons come from `public/assets/logo.svg` (the maskable one keeps the beats in
the central circle); the screenshots of the install dialog are taken on a running
site, in both languages, narrow (412×915) and wide (1280×800). Commit what they
make.
