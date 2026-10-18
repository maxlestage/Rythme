// Checks the installable app in a real Chromium, as deployed on Heroku (the server, at
// the root) and on GitHub Pages (the static export behind /Rythme), see README.md:
//
//   node check.js <rythme-site binary> [server|export]
//
// Each run works on a copy of public/ in a temporary folder: it starts the site, lets
// the service worker install, cuts the network (offline mode, and the site stopped),
// uses every page and the playground, then "deploys" a new version and expects the
// update banner. It ends with the number of failed checks as its exit code.
"use strict";

const fs = require("fs");
const path = require("path");
const { launch } = require("./browser");
const { workspace, freePort, startServer, startExport, binary, checker } = require("./serve");

const BIN = binary(process.argv[2]);
const MODES = process.argv[3] ? [process.argv[3]] : ["server", "export"];
const PAGES = ["", "learn/", "examples/", "playground/", "teach/"];
const LANGS = ["fr", "en"];
const OFFLINE_TITLE = { fr: "Vous êtes hors ligne", en: "You are offline" };

const checks = checker();
const check = checks.check;

function deployChange(dir) {
  fs.appendFileSync(path.join(dir, "public", "site.css"), `\n/* deployed ${Date.now()} */\n`);
}

// The playground has run something: its status says how it went.
const ran = (page) => page.waitForFunction(() => /good|bad/.test(document.getElementById("status").className) && !document.body.classList.contains("running"), null, { timeout: 20000 });

async function setCode(page, code) {
  await page.fill("#source", code);
}

async function runAndRead(page, button) {
  await page.evaluate(() => { document.getElementById("status").className = "status"; });
  await page.click(button);
  await ran(page);
  return page.$eval("#output", (e) => e.textContent);
}

async function verify(mode) {
  console.log(`\n== ${mode === "server" ? "the server, at the root" : "the static export, behind /Rythme"}`);
  const dir = workspace();
  const port = await freePort();
  const start = (d, p) => (mode === "server" ? startServer : startExport)(BIN, d, p);
  let site = await start(dir, port);
  const base = `http://localhost:${port}${mode === "server" ? "" : "/Rythme"}`;
  const browser = await launch();
  const errors = [];
  const watch = (page) => {
    page.on("console", (m) => { if (m.type() === "error") errors.push(`${page.url()}: ${m.text()}`); });
    page.on("pageerror", (e) => errors.push(`${page.url()}: ${e.message}`));
  };

  // iPhone: no install prompt, a hint shown until closed.
  const iphone = await browser.newContext({
    viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true, serviceWorkers: "block",
    userAgent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
  });
  const ios = await iphone.newPage();
  await ios.goto(`${base}/fr/`);
  check(await ios.isVisible("#ios-hint"), "iOS: the hint to add the site to the home screen is shown");
  await ios.click("#ios-hint-close");
  await ios.reload();
  check(!(await ios.isVisible("#ios-hint")), "iOS: once closed, the hint stays closed");
  await iphone.close();

  const context = await browser.newContext({ viewport: { width: 390, height: 844 }, locale: "fr-FR", acceptDownloads: true });
  context.on("page", watch);
  const page = await context.newPage();

  // (b) The service worker installs, activates and controls the page.
  await page.goto(`${base}/fr/`);
  const sw = await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    const worker = registration.active;
    if (worker.state !== "activated") {
      await new Promise((resolve) => worker.addEventListener("statechange", () => worker.state === "activated" && resolve()));
    }
    if (!navigator.serviceWorker.controller) {
      await new Promise((resolve) => navigator.serviceWorker.addEventListener("controllerchange", resolve, { once: true }));
    }
    return { state: registration.active.state, scope: registration.scope, controlled: !!navigator.serviceWorker.controller };
  });
  check(sw.state === "activated" && sw.controlled, "the service worker is activated and controls the page", sw);
  check(sw.scope === `${base}/`, `its scope is ${base}/`, sw.scope);
  const saved = await page.evaluate(async () => {
    const keys = await caches.keys();
    const counts = {};
    for (const key of keys) counts[key] = (await (await caches.open(key)).keys()).length;
    return counts;
  });
  check(Object.keys(saved).some((k) => /^rythme-[0-9a-f]{10}$/.test(k) && saved[k] >= 26), "every file of the site is saved", saved);

  // (a) Installable, with a manifest that parses, in both languages.
  for (const lang of LANGS) {
    await page.goto(`${base}/${lang}/`);
    const cdp = await context.newCDPSession(page);
    const { installabilityErrors } = await cdp.send("Page.getInstallabilityErrors");
    check(installabilityErrors.length === 0, `/${lang}/: no installability error`, installabilityErrors);
    const manifest = await cdp.send("Page.getAppManifest");
    check(manifest.errors.length === 0 && manifest.url === `${base}/${lang}/manifest.webmanifest`, `/${lang}/: the manifest parses`, manifest.errors);
    const data = JSON.parse(manifest.data);
    check(new URL(data.start_url, manifest.url).href === `${base}/${lang}/`, `/${lang}/: the app starts on ${base}/${lang}/`);
    await cdp.detach();
  }
  const install = await page.evaluate(() => !document.getElementById("app-install").closest("li").hidden);
  console.log(`  info the install button is ${install ? "shown (beforeinstallprompt fired)" : "hidden (no beforeinstallprompt in this browser)"}`);
  // Headless Chromium never offers to install: the same event, made by hand.
  await page.evaluate(() => {
    const offer = new Event("beforeinstallprompt", { cancelable: true });
    offer.prompt = () => { window.prompted = true; };
    offer.userChoice = Promise.resolve({ outcome: "accepted" });
    window.dispatchEvent(offer);
  });
  await page.click(".burger");
  check(await page.isVisible("#app-install"), "the install button appears when the browser offers to install");
  await page.click("#app-install");
  await page.waitForFunction(() => document.getElementById("app-install").closest("li").hidden);
  check(await page.evaluate(() => window.prompted === true), "the install button opens the browser's install dialog, then hides");

  // (c) Offline: the site is stopped and the browser has no network.
  await site.stop();
  await context.setOffline(true);
  for (const lang of LANGS) {
    for (const p of PAGES) {
      const url = `${base}/${lang}/${p}`;
      const res = await page.goto(url);
      const shown = await page.evaluate(() => ({
        lang: document.documentElement.lang,
        h1: (document.querySelector("h1") || {}).textContent,
        wide: document.documentElement.scrollWidth > window.innerWidth,
        fonts: [...document.fonts].some((f) => f.family.includes("Plex") && f.status === "loaded"),
      }));
      check(res && res.ok() && shown.lang === lang && shown.h1 && !shown.wide, `offline: ${url}`, shown);
      if (p === "") check(shown.fonts, `offline: ${url} has its fonts`);
    }
  }

  // The playground runs, checks, tests and formats, from the saved interpreter.
  await page.goto(`${base}/en/playground/`);
  await ran(page);
  check(/\[1, 9, 25\]/.test(await page.$eval("#output", (e) => e.textContent)), "offline playground: the welcome program runs");
  await setCode(page, "fn double(x) = x * 2\nprint(map([1, 2, 3], double))\n");
  check((await runAndRead(page, "#run")).includes("[2, 4, 6]"), "offline playground: Run");
  await setCode(page, "fn double(x) = x * 2\nprint(doubel(2))\n");
  check(/error\[/.test(await runAndRead(page, "#check")), "offline playground: Check finds the error");
  await setCode(page, 'fn double(x) = x * 2\n\ntest "double" = assert_eq(double(2), 4)\n\ntest "wrong" = assert_eq(double(2), 5)\n');
  const tested = await runAndRead(page, "#test");
  check(/✓/.test(tested) && /✗/.test(tested), "offline playground: Test runs the tests", tested);
  await setCode(page, "fn  double( x )=x*2\nprint( map([1,2,3],double) )\n");
  await page.click("#format");
  const formatted = await page.$eval("#source", (e) => e.value);
  check(formatted.startsWith("fn double(x) = x * 2\n"), "offline playground: Format", formatted);
  await page.selectOption("#example", "testing");
  await ran(page);
  check(/✓/.test(await page.$eval("#output", (e) => e.textContent)), "offline playground: the examples menu (testing.ry)");
  await page.click('.topic[data-topic="lists"]');
  await page.waitForFunction(() => /map/.test(document.getElementById("help-text").textContent));
  check(true, "offline playground: the help topics");
  // A .ry file opened, the program downloaded, the code kept after closing.
  await page.setInputFiles("#open-file", { name: "hello.ry", mimeType: "text/plain", buffer: Buffer.from('print("hello from a file")\n') });
  // The status still shows the previous run until the file is read: wait for its output.
  await page
    .waitForFunction(() => document.getElementById("output").textContent.includes("hello from a file"), null, { timeout: 20000 })
    .catch(() => {});
  await ran(page);
  check((await page.textContent("#file-name")) === "hello.ry" && (await page.textContent("#output")).includes("hello from a file"), "offline playground: Open a .ry file");
  const [download] = await Promise.all([page.waitForEvent("download"), page.click("#save")]);
  const content = fs.readFileSync(await download.path(), "utf8");
  check(download.suggestedFilename() === "hello.ry" && content.includes("hello from a file"), "offline playground: Download as a .ry file");
  await page.reload();
  await ran(page);
  check((await page.$eval("#source", (e) => e.value)).includes("hello from a file") && (await page.textContent("#file-name")) === "hello.ry", "offline playground: the code survives a reload");
  // A shared text (the share target).
  await page.goto(`${base}/fr/playground/?title=x&text=${encodeURIComponent('print("partagé")')}`);
  await ran(page);
  check((await page.textContent("#output")).includes("partagé") && !page.url().includes("?"), "offline playground: a shared text opens in the playground");
  // The language switch, the burger menu.
  await page.click('.lang--bar a[hreflang="en"]');
  await page.waitForURL(`${base}/en/playground/`);
  check((await page.getAttribute("html", "lang")) === "en", "offline: the language switch");
  await page.click(".burger");
  await page.click('#nav a[href$="/en/examples/"]');
  await page.waitForURL(`${base}/en/examples/`);
  check(await page.isVisible("#fizzbuzz"), "offline: the burger menu");

  // (d) A page never saved: the offline page, in the language of the address.
  for (const lang of LANGS) {
    await page.goto(`${base}/${lang}/nothing-here/`);
    check((await page.textContent("h1")) === OFFLINE_TITLE[lang], `offline: an unknown page shows the ${lang} offline page`);
  }
  // The language switch above chose English: / remembers it, as the server does.
  await page.goto(`${base}/`);
  check(page.url() === `${base}/en/`, "offline: / goes to the home page in the language chosen", page.url());

  // (e) The app is open on the playground, someone types; a new version is deployed and
  // the app looks for it (as site.js does when the app comes back to the foreground):
  // the banner, and no reload on its own.
  await page.goto(`${base}/fr/playground/`);
  await ran(page);
  const oldCss = await page.getAttribute('link[href*="site.css"]', "href");
  await setCode(page, 'print("typing")\n');
  await page.evaluate(() => { window.stillHere = true; });
  deployChange(dir);
  site = await start(dir, port);
  await context.setOffline(false);
  await page.evaluate(async () => (await navigator.serviceWorker.getRegistration()).update());
  await page.waitForSelector("#app-update:not([hidden])", { timeout: 30000 }).catch(() => null);
  check(await page.isVisible("#app-update"), "update: the banner offers the new version");
  check(await page.$eval("#app-update", (e) => e.closest("[role=status]") !== null), "update: the banner is announced (role=status)");
  check(await page.evaluate(() => window.stillHere === true), "update: no reload on its own");
  await Promise.all([page.waitForEvent("load"), page.click("#app-reload")]);
  await ran(page);
  const newCss = await page.getAttribute('link[href*="site.css"]', "href");
  const after = await page.evaluate(async () => ({ keys: await caches.keys(), code: document.getElementById("source").value }));
  check(newCss !== oldCss, "update: Reload loads the new version", { oldCss, newCss });
  check(after.keys.filter((k) => /^rythme-[0-9a-f]{10}$/.test(k)).length === 1, "update: the old cache is deleted", after.keys);
  check(after.code.includes("typing"), "update: the code being typed is kept");

  // (f) No console error, no CSP violation, on any page.
  check(errors.length === 0, "no console error", errors.join("\n"));
  await browser.close();
  await site.stop();
  fs.rmSync(dir, { recursive: true, force: true });
}

(async () => {
  for (const mode of MODES) {
    try {
      await verify(mode);
    } catch (err) {
      check(false, `${mode}: ${err.message.split("\n")[0]}`);
    }
  }
  console.log(checks.failures ? `\n${checks.failures} check(s) failed` : "\nevery check passed");
  process.exit(checks.failures ? 1 : 0);
})();
