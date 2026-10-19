// The site as real phones show it, in a real Chromium emulating them (`isMobile`,
// `hasTouch`: the meta viewport applies, and an element wider than the screen widens
// the page, which the phone then zooms out), as deployed on Heroku (the server, at the
// root) and on GitHub Pages (the static export behind /Rythme), see README.md:
//
//   node mobile.js <rythme-site binary> [server|export]
//
// `layout.js` measures windows of a computer; phones add what it cannot see: the text
// enlarged by the system or the browser (Android's font size, Safari's and Chrome's
// text zoom: every text bigger, the boxes the same), the page zoomed (200% of a phone of
// 360px is a window of 180px), landscape, the keyboard open, every menu and panel open,
// iOS zooming into a field whose text is under 16px, and fingers (44px targets).
// MOBILE_SHOTS=<folder> saves a screenshot of each page on each phone. The exit code is
// the number of failed checks.
"use strict";

const fs = require("fs");
const path = require("path");
const { launch } = require("./browser");
const { SITE, workspace, freePort, startServer, startExport, binary, checker } = require("./serve");

const BIN = binary(process.argv[2]);
const MODES = process.argv[3] ? [process.argv[3]] : ["server", "export"];
const LANGS = ["fr", "en", "es", "de", "pt", "zh"];
const PAGES = ["", "learn/", "examples/", "playground/", "teach/", "offline/", "nothing-here/"];
const SHOTS = process.env.MOBILE_SHOTS || null;

// The phones: portrait and landscape, the page zoomed (a narrower window, as Chrome on
// Android does with the system's font size, and Safari with its text size), and the
// text alone enlarged (`text`: every font size times this, as WebKit's text zoom and
// Android's font scale do).
const PHONE = { isMobile: true, hasTouch: true, deviceScaleFactor: 2 };
const DEVICES = [
  { name: "320", width: 320, height: 568 },
  { name: "360", width: 360, height: 740 },
  { name: "375", width: 375, height: 667 },
  { name: "390", width: 390, height: 844 },
  { name: "412", width: 412, height: 915 },
  { name: "landscape 844×390", width: 844, height: 390 },
  { name: "landscape 915×412", width: 915, height: 412 },
  { name: "page zoom 150% (390)", width: 260, height: 562, zoomed: true },
  { name: "page zoom 200% (412)", width: 206, height: 457, zoomed: true },
  { name: "page zoom 200% (360)", width: 180, height: 370, zoomed: true },
  { name: "text 150% (320)", width: 320, height: 568, text: 1.5 },
  { name: "text 200% (390)", width: 390, height: 844, text: 2 },
];
const byName = (name) => DEVICES.find((d) => d.name === name);

const checks = checker();
const check = checks.check;

const settle = (page) => page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
const ran = (page) => page.waitForFunction(() => /good|bad/.test(document.getElementById("status").className) && !document.body.classList.contains("running"), null, { timeout: 20000 });

// Every font size times `factor`, as text zoom does (boxes keep their sizes).
function textZoom(factor) {
  const els = [document.documentElement, ...document.querySelectorAll("body, body *")];
  const sizes = els.map((e) => parseFloat(getComputedStyle(e).fontSize));
  els.forEach((e, i) => e.style.setProperty("font-size", `${sizes[i] * factor}px`, "important"));
}

// What a phone would let the finger scroll sideways, measured in the page: the window
// widened by the content (the layout viewport wider than the screen, the page zoomed
// out), the document wider than the window, a scroll that moves sideways, an element
// that sticks out of the screen, an element that scrolls sideways (code, tables…), and
// every text field under 16px (iOS zooms into it, and the page then scrolls sideways).
function phone(width) {
  const root = document.documentElement;
  const name = (el) => {
    let s = el.tagName.toLowerCase();
    if (el.id) s += `#${el.id}`;
    if (typeof el.className === "string" && el.className.trim()) s += `.${el.className.trim().split(/\s+/).join(".")}`;
    const parent = el.parentElement;
    return parent && parent !== document.body && typeof parent.className === "string" && parent.className ? `${parent.className.split(/\s+/)[0]} > ${s}` : s;
  };
  const out = [];
  if (window.innerWidth !== width) out.push(`the window is ${window.innerWidth}px wide on a screen of ${width}px (the page is zoomed out)`);
  if (window.visualViewport && Math.abs(window.visualViewport.scale - 1) > 0.001) out.push(`zoomed to ${window.visualViewport.scale}`);
  if (root.scrollWidth > root.clientWidth) out.push(`the document is ${root.scrollWidth}px wide`);
  const y = window.scrollY;
  window.scrollBy(100, 0);
  if (window.scrollX !== 0) out.push(`scrolls sideways by ${window.scrollX}px`);
  window.scrollTo(0, y);
  const vw = root.clientWidth;
  for (const el of document.body.querySelectorAll("*")) {
    const cs = getComputedStyle(el);
    if (el.matches("input:not([type=range]):not([type=radio]):not([type=checkbox]):not([type=file]):not([type=hidden]), textarea, select, [contenteditable]")
      && parseFloat(cs.fontSize) < 16) out.push(`a field under 16px: ${name(el)} ${cs.fontSize}`);
    if (cs.display === "none" || cs.visibility === "hidden" || el.closest("details:not([open]) > :not(summary)")) continue;
    const r = el.getBoundingClientRect();
    if (r.width === 0 && r.height === 0) continue;
    if (r.right > vw + 0.5 || r.left < -0.5) out.push(`sticks out: ${name(el)} [${Math.round(r.left)}, ${Math.round(r.right)}]`);
    if (el.scrollWidth > el.clientWidth + 1 && /auto|scroll/.test(cs.overflowX)) out.push(`scrolls sideways: ${name(el)} ${el.scrollWidth} > ${el.clientWidth}`);
  }
  return out.slice(0, 12);
}

async function open(browser, device) {
  const context = await browser.newContext({ ...PHONE, serviceWorkers: "block", viewport: { width: device.width, height: device.height } });
  const page = await context.newPage();
  page.errors = [];
  page.on("console", (m) => { if (m.type() === "error") page.errors.push(m.text()); });
  page.on("pageerror", (e) => page.errors.push(e.message));
  return { context, page };
}

// The server answers 100 requests a second to one visitor (Paquets' rate limit): a page
// and its files are a dozen, so the pages are visited at a human pace.
let lastVisit = 0;
async function visit(page, url, device) {
  const wait = lastVisit + 250 - Date.now();
  if (wait > 0) await page.waitForTimeout(wait);
  lastVisit = Date.now();
  await page.goto(url);
  await page.evaluate(() => document.fonts.ready);
  if (/playground\/(#|$)/.test(url)) await ran(page).catch(async (err) => {
    const state = await page.evaluate(() => `${document.getElementById("status").className} "${document.getElementById("status").textContent}" ${document.body.className} output="${document.getElementById("output").textContent.slice(0, 80)}" buttons=${document.getElementById("run").disabled} ${typeof WebAssembly}`) + " errors: " + page.errors.join(" | ");
    throw new Error(`${url} on ${device.name}: ${state}: ${err.message}`);
  });
  if (device.text) await page.evaluate(textZoom, device.text);
  await settle(page);
}

// 1. Every page in every language on every phone.
async function pages(browser, base, mode) {
  const failed = [];
  let measured = 0;
  for (const device of DEVICES) {
    const { context, page } = await open(browser, device);
    for (const lang of LANGS) {
      for (const p of PAGES) {
        const url = `${base}/${lang}/${p}`;
        await visit(page, url, device);
        measured += 1;
        for (const what of await page.evaluate(phone, device.width)) failed.push(`${device.name} /${lang}/${p}: ${what}`);
        if (p === "playground/" && !device.text && !device.zoomed) {
          // Run in sight, without scrolling past the editor, and the page did not jump.
          const run = await page.evaluate(() => ({ y: window.scrollY, run: document.getElementById("run").getBoundingClientRect().bottom, h: window.innerHeight }));
          if (run.y !== 0 || run.run > run.h) failed.push(`${device.name} /${lang}/${p}: Run is not in sight when the playground opens (${JSON.stringify(run)})`);
        }
        if (SHOTS && ["fr", "en", "zh"].includes(lang)) {
          await page.screenshot({ path: path.join(SHOTS, `${device.name.replace(/[^\w]+/g, "-")}-${lang}-${(p || "home/").replace(/\/$/, "")}.png`) });
        }
      }
    }
    await context.close();
  }
  check(failed.length === 0, `${mode}: nothing scrolls sideways on ${LANGS.length * PAGES.length} pages × ${DEVICES.length} phones, every field at 16px (${measured} pages)`, `\n    ${failed.slice(0, 60).join("\n    ")}`);
}

// 2. Every menu and panel open: the burger menu (with the theme picker), the language
// menu, the banners of the app, and in the playground the Seed option, the box of a
// shared link, the steps of a calculation, long output lines and a long error, a topic
// of the help.
async function panels(browser, base, mode) {
  const failed = [];
  const long = "a_long_word_without_spaces_that_goes_on_and_on_".repeat(4);
  const program = `# ${"https://example.com/a/long/address/".repeat(3)}\nprint("${long}")\nprint("${"word ".repeat(40)}")\nprint(${long})\n`;
  const steps = 'fn twice(t) = t + t\nfn grow(t)\n  doubled = twice(t)\n  print(len(doubled))\n  doubled + "!"\nend\n\ngrow(twice("a_long_word_without_spaces_that_goes_on_and_on_and_on_and_on_"))\n';
  const measure = async (page, device, what) => {
    await settle(page);
    for (const line of await page.evaluate(phone, device.width)) failed.push(`${device.name} ${what}: ${line}`);
  };
  for (const device of ["320", "390", "landscape 844×390", "page zoom 200% (360)", "text 200% (390)"].map(byName)) {
    const { context, page } = await open(browser, device);
    for (const lang of LANGS) {
      await visit(page, `${base}/${lang}/learn/`, device);
      await page.click(".burger");
      // The panel slides in (0.2s).
      await page.waitForTimeout(300);
      await measure(page, device, `/${lang}/ burger menu`);
      const menu = await page.evaluate(() => {
        const nav = document.getElementById("nav").getBoundingClientRect();
        const bar = document.querySelector(".bar").getBoundingClientRect();
        return { top: nav.top, bar: bar.bottom, themes: getComputedStyle(document.getElementById("theme-panel")).display !== "none" };
      });
      if (Math.abs(menu.top - Math.max(0, menu.bar)) > 1 || !menu.themes) failed.push(`${device.name} /${lang}/: the menu does not start under the bar, or has no theme picker: ${JSON.stringify(menu)}`);
      await page.click(".burger");
      await page.click(".lang--bar summary");
      await measure(page, device, `/${lang}/ language menu`);
      const list = await page.evaluate(() => {
        const ul = document.querySelector(".lang__menu");
        const r = ul.getBoundingClientRect();
        return { inside: r.left >= 0 && r.right <= document.documentElement.clientWidth, reachable: r.bottom <= window.innerHeight + 1 || ul.scrollHeight > ul.clientHeight };
      });
      if (!list.inside || !list.reachable) failed.push(`${device.name} /${lang}/: the language menu leaves the screen ${JSON.stringify(list)}`);
      await page.evaluate(() => { document.querySelector(".lang--bar").open = false; });
      // The banners of the app, all shown at once.
      await page.evaluate(() => {
        for (const id of ["app-update", "ios-hint"]) document.getElementById(id).hidden = false;
        document.getElementById("app-install").closest("li").hidden = false;
      });
      await measure(page, device, `/${lang}/ banners`);

      await visit(page, `${base}/${lang}/playground/#code=${encodeURIComponent(program)}&seed=999999999999999`, device);
      await page.evaluate(() => { document.getElementById("seed-option").open = true; });
      await measure(page, device, `/${lang}/playground/ seed and long output`);
      await page.evaluate(() => { document.getElementById("seed-option").open = false; });
      await page.click("#share");
      await page.waitForFunction(() => document.getElementById("share-msg").textContent !== "");
      await measure(page, device, `/${lang}/playground/ shared link`);
      await page.click(".topic >> nth=3");
      await measure(page, device, `/${lang}/playground/ help`);
      await visit(page, `${base}/${lang}/playground/#code=${encodeURIComponent(steps)}&mode=steps&step=4`, device);
      await measure(page, device, `/${lang}/playground/ steps`);
      await visit(page, `${base}/${lang}/playground/#code=${encodeURIComponent(`fn f(x) = x +\nprint(${long} + 1)\n`)}`, device);
      await measure(page, device, `/${lang}/playground/ long error`);
    }
    await context.close();
  }
  check(failed.length === 0, `${mode}: every menu and panel fits on phones, page zoom and text zoom included`, `\n    ${failed.slice(0, 40).join("\n    ")}`);
}

// 3. The keyboard: the editor stays inside the part of the screen left above it, and
// nothing scrolls sideways while typing; the editor is back to its height once closed.
async function keyboard(browser, base, mode) {
  for (const [w, h, kb] of [[390, 844, 450], [844, 390, 160]]) {
    const device = { name: `${w}×${h}`, width: w, height: h };
    const { context, page } = await open(browser, device);
    await visit(page, `${base}/en/playground/`, device);
    const before = await page.evaluate(() => document.querySelector(".editor").getBoundingClientRect().height);
    await page.tap("#source");
    // The keyboard takes the bottom of the screen.
    await page.setViewportSize({ width: w, height: h - kb });
    await settle(page);
    await page.keyboard.type("print(1)\n");
    await settle(page);
    const open_ = await page.evaluate(() => ({
      keyboard: document.documentElement.classList.contains("keyboard"),
      editor: document.querySelector(".editor").getBoundingClientRect().height,
      visible: window.visualViewport.height,
    }));
    const wide = await page.evaluate(phone, w);
    check(open_.keyboard && open_.editor <= open_.visible && wide.length === 0, `${mode}: ${device.name} with the keyboard open, the editor fits above it and nothing scrolls sideways`, { open_, wide });
    await page.evaluate(() => document.activeElement.blur());
    await page.setViewportSize({ width: w, height: h });
    await settle(page);
    const after = await page.evaluate(() => ({ keyboard: document.documentElement.classList.contains("keyboard"), editor: document.querySelector(".editor").getBoundingClientRect().height }));
    check(!after.keyboard && Math.abs(after.editor - before) < 1, `${mode}: ${device.name} the keyboard closed, the editor is back to its height`, { before, after });
    await context.close();
  }
}

// 4. Fingers: every control at 44px or more, both ways (links in running text are
// exempt, as WCAG says).
async function targets(browser, base, mode) {
  const failed = [];
  const device = byName("390");
  const { context, page } = await open(browser, device);
  for (const lang of ["fr", "en", "de", "zh"]) {
    for (const p of PAGES) {
      await visit(page, `${base}/${lang}/${p}`, device);
      for (const menu of [false, true]) {
        if (menu) await page.click(".burger");
        const small = await page.evaluate(() => {
          const out = [];
          for (const el of document.querySelectorAll("a[href], button, summary, select, input:not([type=hidden]):not([type=file]), textarea, label.theme__option")) {
            if (el.closest("p, td, dd, .prose li, .lead, .note, .skip, .sr-only, [hidden], details:not([open]) > :not(summary)") || el.matches(".theme__option input")) continue;
            const cs = getComputedStyle(el);
            if (cs.display === "none" || cs.visibility === "hidden") continue;
            const r = el.getBoundingClientRect();
            if (r.width === 0 || r.bottom < 0 || r.top > window.innerHeight * 40) continue;
            if (r.height < 43.5 || r.width < 43.5) out.push(`${el.tagName.toLowerCase()}${el.id ? "#" + el.id : ""}.${el.className} "${el.textContent.trim().slice(0, 20)}" ${Math.round(r.width)}×${Math.round(r.height)}`);
          }
          return [...new Set(out)];
        });
        for (const s of small) failed.push(`/${lang}/${p}${menu ? " (menu)" : ""}: ${s}`);
        if (menu) await page.click(".burger");
      }
    }
  }
  await context.close();
  check(failed.length === 0, `${mode}: every control is at least 44×44px on a phone`, `\n    ${[...new Set(failed)].slice(0, 40).join("\n    ")}`);
}

// 5. The stylesheet itself, mobile first: the rules are for phones and the media
// queries only add (`min-width`, `min-height`, never `max-width` nor `max-height`); no
// width of `100vw` (wider than the page when a scrollbar is shown); no content shown
// only on hover; no text kept on one line unless what overflows is hidden.
function stylesheet() {
  const css = fs.readFileSync(path.join(SITE, "public", "site.css"), "utf8").replace(/\/\*[\s\S]*?\*\//g, "");
  const media = [...css.matchAll(/@media([^{]+)\{/g)].map((m) => m[1].trim());
  check(media.every((m) => !/max-(width|height)/.test(m)), "site.css: media queries only add room (no max-width nor max-height)", media.filter((m) => /max-(width|height)/.test(m)));
  check(!/(^|[^-])width:\s*100vw/.test(css), "site.css: no width of 100vw");
  const rules = [...css.matchAll(/([^{}]+)\{([^{}]*)\}/g)].map((m) => ({ selector: m[1].trim(), body: m[2] }));
  const hover = rules.filter((r) => /:hover/.test(r.selector) && /(^|;|\s)(display|visibility|opacity|height|max-height)\s*:/.test(r.body));
  check(hover.length === 0, "site.css: nothing is shown only on hover", hover.map((r) => r.selector));
  const nowrap = rules.filter((r) => /white-space:\s*(nowrap|pre)\s*(;|$)/.test(r.body) && !/overflow:\s*hidden/.test(r.body));
  check(nowrap.length === 0, "site.css: no text kept on one line that could stick out", nowrap.map((r) => r.selector));
}

async function verify(mode) {
  console.log(`\n== ${mode === "server" ? "the server, at the root" : "the static export, behind /Rythme"}`);
  const dir = workspace();
  const port = await freePort();
  const site = await (mode === "server" ? startServer : startExport)(BIN, dir, port);
  const base = `http://localhost:${port}${mode === "server" ? "" : "/Rythme"}`;
  const browser = await launch();
  try {
    await pages(browser, base, mode);
    await panels(browser, base, mode);
    await keyboard(browser, base, mode);
    await targets(browser, base, mode);
  } finally {
    await browser.close();
    await site.stop();
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

(async () => {
  if (SHOTS) fs.mkdirSync(SHOTS, { recursive: true });
  stylesheet();
  for (const mode of MODES) {
    try {
      await verify(mode);
    } catch (err) {
      check(false, `${mode}: ${err.message.split("\n")[0]}`, err.stack);
    }
  }
  console.log(checks.failures ? `\n${checks.failures} check(s) failed` : "\nevery check passed");
  process.exit(checks.failures ? 1 : 0);
})();
