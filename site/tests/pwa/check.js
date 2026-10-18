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
// Every language of the site (src/i18n.rs), with the title of its offline page and the
// word that starts the interpreter's error messages in it.
const LANGS = ["fr", "en", "es", "de", "pt", "zh"];
const OFFLINE_TITLE = {
  fr: "Vous êtes hors ligne", en: "You are offline", es: "No tienes conexión",
  de: "Du bist offline", pt: "Você está sem conexão", zh: "你已离线",
};
const ERROR_WORD = { fr: "erreur", en: "error", es: "error", de: "Fehler", pt: "erro", zh: "错误" };

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

// Share, then read the link it made (copied, or shown in its field).
async function share(page) {
  await page.evaluate(() => { document.getElementById("share-msg").textContent = ""; });
  await page.click("#share");
  await page.waitForFunction(() => document.getElementById("share-msg").textContent !== "");
  return page.$eval("#share-link", (e) => e.value);
}

// A new document at `url` (not only a new fragment), once the playground has run.
async function open(page, url) {
  await page.goto("about:blank");
  await page.goto(url);
  await ran(page);
}

const draftOf = (page) => page.evaluate(() => localStorage.getItem("rythme-code"));
const noteShown = (page) => page.isVisible("#shared-note");

// Sharing links, offline: the program travels in the fragment of the link; opening it
// shows and runs the same program, keeps the draft until the program is changed, and
// gives the draft back on demand. Long programs are refused; damaged links are said so;
// browsers without CompressionStream make plain links and read compressed ones.
async function shareLinks(context, page, base) {
  const SHARED = 'fn triple(x) = x * 3\nprint(map([1, 2, 3], triple))\nprint("été, 中文")\n';
  const DRAFT = 'print("my own draft")\n';
  await context.grantPermissions(["clipboard-read", "clipboard-write"], { origin: new URL(base).origin });
  await page.goto(`${base}/fr/playground/`);
  await ran(page);
  check(await page.isVisible("#share") && (await page.getAttribute("#share", "aria-expanded")) === "false", "share: the Share button is shown");
  await setCode(page, SHARED);
  const link = await share(page);
  const shown = await page.evaluate(async () => ({
    msg: document.getElementById("share-msg").textContent,
    copied: await navigator.clipboard.readText().catch(() => null),
    expanded: document.getElementById("share").getAttribute("aria-expanded"),
    ui: JSON.parse(document.querySelector(".playground").dataset.ui).shareCopied,
  }));
  check(link.startsWith(`${base}/fr/playground/#code=1:`) && /^[\w:/.#=-]+$/.test(link), "share: the link holds the program, compressed and URL-safe", link);
  check(shown.copied === link && shown.msg === shown.ui && shown.expanded === "true", "share: the link is copied, and said so", shown);
  await page.keyboard.press("Escape");
  check(!(await page.isVisible("#share-box")) && (await page.evaluate(() => document.activeElement.id)) === "share", "share: Escape closes the box and gives the focus back");
  await setCode(page, DRAFT);
  check((await draftOf(page)) === DRAFT, "share: a draft is saved");

  // The link, in a new page (offline, from the service worker).
  const other = await context.newPage();
  await open(other, link);
  const opened = await other.evaluate(() => ({ code: document.getElementById("source").value, out: document.getElementById("output").textContent }));
  check(opened.code === SHARED && opened.out.includes("[3, 6, 9]") && opened.out.includes("été, 中文"), "share: the link opens the same program, which runs", opened);
  check((await noteShown(other)) && (await draftOf(other)) === DRAFT, "share: the draft is kept, and a notice says so");
  await other.reload();
  await ran(other);
  check((await other.$eval("#source", (e) => e.value)) === SHARED && (await draftOf(other)) === DRAFT, "share: after a reload, still the program of the link and the draft kept");
  await other.click("#shared-back");
  await ran(other);
  check((await other.$eval("#source", (e) => e.value)) === DRAFT && !(await noteShown(other)) && !other.url().includes("#"), "share: Back to my draft", other.url());
  // A link pasted in the address bar of the open playground: only the fragment changes.
  await other.evaluate((url) => { location.hash = new URL(url).hash; }, link);
  await other.waitForFunction((code) => document.getElementById("source").value === code, SHARED);
  await ran(other);
  check(await noteShown(other), "share: a link pasted in the open playground opens its program");
  await other.focus("#source");
  await other.keyboard.type("# changed\n");
  const edited = await draftOf(other);
  check(edited.includes("triple") && edited.includes("# changed") && !(await noteShown(other)) && !other.url().includes("#"), "share: once changed, the program of the link becomes the draft");
  await setCode(other, DRAFT);

  // The test mode travels with the link.
  await page.selectOption("#example", "testing");
  await ran(page);
  const testLink = await share(page);
  await open(other, testLink);
  check(testLink.endsWith("&mode=test") && /✓/.test(await other.textContent("#output")), "share: the test mode travels with the link", testLink);

  // Too long for a link: refused, with a clear message, and no link.
  let x = 2463534242;
  const random = Array.from({ length: 14000 }, () => {
    x ^= x << 13; x ^= x >>> 17; x ^= x << 5;
    return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"[(x >>> 0) % 62];
  }).join("");
  await setCode(page, `# ${random}\nprint(1)\n`);
  await share(page);
  const refused = await page.evaluate(() => ({
    msg: document.getElementById("share-msg").textContent,
    bad: document.getElementById("share-msg").classList.contains("bad"),
    row: document.getElementById("share-row").hidden,
  }));
  check(refused.bad && refused.row && /8\s?000/.test(refused.msg), "share: a program too long for a link is refused, said so", refused);

  // A damaged link: said so, the draft is shown.
  await open(other, `${base}/fr/playground/#code=1:AAAA`);
  const broken = await other.evaluate(() => ({
    note: document.getElementById("shared-text").textContent,
    back: document.getElementById("shared-back").hidden,
    draft: document.getElementById("source").value === localStorage.getItem("rythme-code"),
    ui: JSON.parse(document.querySelector(".playground").dataset.ui).shareBroken,
  }));
  check(broken.note === broken.ui && broken.back && broken.draft, "share: a damaged link is said so, and the draft is shown", broken);
  await other.close();

  // Without CompressionStream and DecompressionStream: the hand-written inflate reads
  // compressed links (stored, fixed and dynamic blocks, made here by zlib), and Share
  // makes plain links.
  const old = await context.newPage();
  await old.addInitScript(() => { delete window.CompressionStream; delete window.DecompressionStream; });
  const zlib = require("zlib");
  const long = `${SHARED}${"print(triple(4))\n".repeat(40)}`;
  for (const [what, code, level] of [["the link of Share", SHARED, null], ["stored", SHARED, 0], ["fixed", SHARED, 1], ["dynamic", long, 9]]) {
    const url = level === null ? link : `${base}/fr/playground/#code=1:${zlib.deflateRawSync(Buffer.from(code), { level }).toString("base64url")}`;
    await open(old, url);
    check((await old.$eval("#source", (e) => e.value)) === code && (await old.textContent("#output")).includes("[3, 6, 9]"), `share: without DecompressionStream, a compressed link opens (${what})`);
  }
  const plain = await share(old);
  check(plain.includes("#code=0:") && Buffer.from(plain.split("#code=0:")[1], "base64url").toString() === long, "share: without CompressionStream, Share makes a plain link", plain);
  await old.close();
  await page.evaluate(() => { localStorage.removeItem("rythme-code"); });
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
  // On a phone, Share hands the link to the share sheet (Web Share API, made by hand:
  // headless Chromium has none).
  await ios.addInitScript(() => { navigator.share = async (data) => { window.sharedData = data; }; });
  await ios.goto(`${base}/fr/playground/`);
  const sheet = await share(ios);
  const handed = await ios.evaluate(() => ({ data: window.sharedData, msg: document.getElementById("share-msg").textContent, ui: JSON.parse(document.querySelector(".playground").dataset.ui).shareShared }));
  check(handed.data && handed.data.url === sheet && sheet.includes("#code=") && handed.msg === handed.ui, "iOS: Share opens the share sheet with the link", handed);
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
      // `pt-BR`, `zh-Hans`: the tag of the page starts with the code of its address.
      check(res && res.ok() && shown.lang.split("-")[0] === lang && shown.h1 && !shown.wide, `offline: ${url}`, shown);
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
  await shareLinks(context, page, base);
  // The playground of every language: its texts, and the interpreter's messages in the
  // language of the page.
  for (const lang of LANGS) {
    await page.goto(`${base}/${lang}/playground/`);
    await ran(page);
    await setCode(page, "fn double(x) = x * 2\nprint(doubel(2))\n");
    const out = await runAndRead(page, "#check");
    const shown = await page.evaluate(() => ({
      status: document.getElementById("status").textContent,
      failedIn: JSON.parse(document.querySelector(".playground").dataset.ui).failedIn.split("{ms}")[0],
      err: (document.querySelector("#output .err") || {}).textContent || "",
      lines: document.getElementById("lines").textContent,
    }));
    check(out.includes(`${ERROR_WORD[lang]}[E003]`) && shown.err.startsWith(`${ERROR_WORD[lang]}[E003]`), `offline playground: the messages in /${lang}/`, { out, shown });
    check(shown.status.startsWith(shown.failedIn) && /\d/.test(shown.lines), `offline playground: the texts of /${lang}/`, shown);
  }
  // The language switch (a menu: its button, then a language), the burger menu.
  await page.goto(`${base}/fr/playground/`);
  await page.click(".lang--bar summary");
  check((await page.locator(".lang--bar .lang__link").count()) === LANGS.length, "offline: the language menu lists every language");
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
