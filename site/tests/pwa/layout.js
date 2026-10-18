// The layout and the color themes in a real Chromium, as deployed on Heroku (the server,
// at the root) and on GitHub Pages (the static export behind /Rythme), see README.md:
//
//   node layout.js <rythme-site binary> [server|export]
//
// No horizontal scrolling: every page in every language, at every width, in every
// theme. The themes: contrast, the picker with the keyboard, the choice kept and
// applied before the first paint. The exit code is the number of failed checks.
"use strict";

const fs = require("fs");
const { launch } = require("./browser");
const { workspace, freePort, startServer, startExport, binary, checker } = require("./serve");

const BIN = binary(process.argv[2]);
const MODES = process.argv[3] ? [process.argv[3]] : ["server", "export"];
// Every language of the site (src/i18n.rs).
const LANGS = ["fr", "en", "es", "de", "pt", "zh"];
const PAGES = ["", "learn/", "examples/", "playground/", "teach/", "offline/", "nothing-here/"];
const WIDTHS = [320, 360, 375, 390, 414, 600, 768, 1024, 1280, 1440, 1920];
// The color themes, as `site.css` defines them; `auto` follows the system.
const THEMES = ["auto", "light", "dark", "contrast", "sepia", "tempo"];
// WCAG: 4.5:1 for text (AA), 7:1 (AAA) in the high-contrast theme.
const MIN_CONTRAST = { contrast: 7 };
const STORAGE = "rythme-theme";

const checks = checker();
const check = checks.check;

// What overflows the viewport, measured in the page: the document's own width, every
// element that sticks out of the viewport, and every element that scrolls sideways.
function measure() {
  const root = document.documentElement;
  const vw = root.clientWidth;
  const name = (el) => {
    let s = el.tagName.toLowerCase();
    if (el.id) s += `#${el.id}`;
    if (typeof el.className === "string" && el.className.trim()) s += `.${el.className.trim().split(/\s+/).join(".")}`;
    const parent = el.parentElement && el.parentElement !== document.body ? el.parentElement : null;
    return parent ? `${parent.tagName.toLowerCase()}${parent.className && typeof parent.className === "string" ? "." + parent.className.trim().split(/\s+/)[0] : ""} > ${s}` : s;
  };
  const wide = [];
  const scrollers = [];
  for (const el of document.body.querySelectorAll("*")) {
    const r = el.getBoundingClientRect();
    if (r.width === 0 && r.height === 0) continue;
    const cs = getComputedStyle(el);
    if (cs.display === "none") continue;
    if (r.right > vw + 0.5 || r.left < -0.5) wide.push(`${name(el)} [${Math.round(r.left)}, ${Math.round(r.right)}]`);
    if (/auto|scroll/.test(cs.overflowX) && el.scrollWidth > el.clientWidth + 1) scrollers.push(`${name(el)} ${el.scrollWidth} > ${el.clientWidth}`);
  }
  // The bar keeps one line: the logo, the language switch and the burger (or the links).
  const bar = document.querySelector(".bar__inner");
  if (bar && bar.getBoundingClientRect().height > parseFloat(getComputedStyle(bar).minHeight) + 1) wide.push(`the bar wraps (${Math.round(bar.getBoundingClientRect().height)}px high)`);
  return { scrollWidth: root.scrollWidth, clientWidth: vw, wide: wide.slice(0, 20), scrollers: scrollers.slice(0, 20) };
}

// Every piece of text against the background it is drawn on: the ratios under the
// minimum. Hidden, decorative (`aria-hidden`) and disabled elements are left out, as
// WCAG does.
function contrast(min) {
  const parse = (c) => {
    const m = c.match(/rgba?\(([^)]+)\)/);
    if (!m) return null;
    const p = m[1].split(/[\s,/]+/).filter(Boolean).map(Number);
    return { r: p[0], g: p[1], b: p[2], a: p.length > 3 ? p[3] : 1 };
  };
  const over = (top, under) => ({
    r: top.r * top.a + under.r * (1 - top.a), g: top.g * top.a + under.g * (1 - top.a), b: top.b * top.a + under.b * (1 - top.a), a: 1,
  });
  const lum = (c) => [c.r, c.g, c.b].map((v) => { v /= 255; return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; })
    .reduce((s, v, i) => s + v * [0.2126, 0.7152, 0.0722][i], 0);
  const ratio = (a, b) => { const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p); return (x + 0.05) / (y + 0.05); };
  // The color behind an element: its own background and its ancestors', blended.
  const background = (el) => {
    const layers = [];
    for (let e = el; e; e = e.parentElement) {
      const cs = getComputedStyle(e);
      if (cs.backgroundImage !== "none") return null;
      const c = parse(cs.backgroundColor);
      if (c && c.a > 0) {
        layers.push(c);
        if (c.a >= 1) break;
      }
    }
    let color = { r: 255, g: 255, b: 255, a: 1 };
    for (const layer of layers.reverse()) color = over(layer, color);
    return color;
  };
  const opacity = (el) => {
    let o = 1;
    for (let e = el; e; e = e.parentElement) o *= Number(getComputedStyle(e).opacity);
    return o;
  };
  const bad = [];
  let lowest = Infinity;
  const seen = new Set();
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  for (let node = walker.nextNode(); node; node = walker.nextNode()) {
    const el = node.parentElement;
    if (!node.textContent.trim() || seen.has(el)) continue;
    seen.add(el);
    if (el.closest("[aria-hidden=true], [hidden], noscript, :disabled, .skip")) continue;
    const r = el.getBoundingClientRect();
    if (r.width === 0 || r.height === 0) continue;
    const cs = getComputedStyle(el);
    if (cs.visibility === "hidden") continue;
    const bg = background(el);
    const fg = parse(cs.color);
    // The editor's field is transparent: its colors are drawn under it.
    if (!bg || !fg || fg.a === 0) continue;
    const shown = over({ ...fg, a: fg.a * opacity(el) }, bg);
    const value = ratio(shown, bg);
    lowest = Math.min(lowest, value);
    if (value < min) bad.push(`${el.tagName.toLowerCase()}.${el.className} "${node.textContent.trim().slice(0, 24)}" ${value.toFixed(2)}`);
  }
  return { lowest, bad: bad.slice(0, 6) };
}

// The colors of a theme (its custom properties) and the ratios of their pairs: every
// text color on the paper, on the cards and on the soft accent; the text of buttons
// and banners on their background.
function palette() {
  const cs = getComputedStyle(document.documentElement);
  const v = (name) => cs.getPropertyValue(`--${name}`).trim();
  const hex = (h) => { const n = parseInt(h.slice(1), 16); return { r: (n >> 16) & 255, g: (n >> 8) & 255, b: n & 255 }; };
  const lum = (c) => [c.r, c.g, c.b].map((x) => { x /= 255; return x <= 0.03928 ? x / 12.92 : ((x + 0.055) / 1.055) ** 2.4; })
    .reduce((s, x, i) => s + x * [0.2126, 0.7152, 0.0722][i], 0);
  const ratio = (a, b) => { const [x, y] = [lum(hex(v(a))), lum(hex(v(b)))].sort((p, q) => q - p); return (x + 0.05) / (y + 0.05); };
  const pairs = [];
  for (const fg of ["ink", "ink-soft", "accent", "err", "warn", "ok", "hint", "tok-kw", "tok-str", "tok-num", "tok-ctor", "tok-com", "tok-fn"]) {
    for (const bg of ["paper", "staff", "accent-soft"]) pairs.push([fg, bg]);
  }
  pairs.push(["accent-ink", "accent"], ["notice-ink", "notice-bg"], ["cmd-ink", "cmd-bg"], ["err-ink", "err"]);
  return pairs.map(([fg, bg]) => ({ pair: `${fg}/${bg}`, ratio: ratio(fg, bg), ok: /^#[0-9a-f]{6}$/i.test(v(fg)) && /^#[0-9a-f]{6}$/i.test(v(bg)) }));
}

const settle = (page) => page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));

async function setTheme(page, theme) {
  await page.evaluate((t) => {
    if (t === "auto") document.documentElement.removeAttribute("data-theme");
    else document.documentElement.setAttribute("data-theme", t);
  }, theme);
}

// The playground has run its welcome program.
const ran = (page) => page.waitForFunction(() => /good|bad/.test(document.getElementById("status").className) && !document.body.classList.contains("running"), null, { timeout: 20000 });

async function noHorizontalScroll(browser, base, mode) {
  const context = await browser.newContext({ serviceWorkers: "block", viewport: { width: WIDTHS[0], height: 800 } });
  const page = await context.newPage();
  const failed = [];
  let measured = 0;
  for (const lang of LANGS) {
    for (const p of PAGES) {
      const url = `${base}/${lang}/${p}`;
      await page.setViewportSize({ width: WIDTHS[0], height: 800 });
      await page.goto(url);
      await page.evaluate(() => document.fonts.ready);
      if (p === "playground/") await ran(page);
      for (const theme of THEMES) {
        await setTheme(page, theme);
        for (const width of WIDTHS) {
          await page.setViewportSize({ width, height: 800 });
          await settle(page);
          const m = await page.evaluate(measure);
          measured += 1;
          if (m.scrollWidth > m.clientWidth) failed.push(`${url} ${theme} ${width}px: the page is ${m.scrollWidth}px wide`);
          for (const what of m.wide) failed.push(`${url} ${theme} ${width}px: sticks out: ${what}`);
          for (const what of m.scrollers) failed.push(`${url} ${theme} ${width}px: scrolls sideways: ${what}`);
        }
      }
    }
  }
  // One line per element and page: the narrowest width and theme where it fails.
  const unique = new Map();
  for (const line of failed) {
    const key = line.replace(/ \S+ \d+px: /, ": ").replace(/\[-?\d+, -?\d+\]|\d+ > \d+|\d+px wide/g, "");
    if (!unique.has(key)) unique.set(key, line);
  }
  check(failed.length === 0, `${mode}: no horizontal scroll on ${LANGS.length * PAGES.length} pages × ${THEMES.length} themes × ${WIDTHS.length} widths (${measured} layouts)`, `\n    ${[...unique.values()].slice(0, 60).join("\n    ")}`);
  await context.close();
}

// The language menu of the bar: open, it stays inside the screen at every width and
// lists every language; with the keyboard, Enter opens it, Escape closes it and gives
// the focus back to its button.
async function languageMenu(browser, base, mode) {
  const context = await browser.newContext({ serviceWorkers: "block", viewport: { width: 320, height: 800 } });
  const page = await context.newPage();
  for (const lang of LANGS) {
    await page.goto(`${base}/${lang}/learn/`);
    for (const width of [320, 375, 768, 1280]) {
      await page.setViewportSize({ width, height: 800 });
      await page.evaluate(() => { document.querySelector(".lang--bar").open = true; });
      await settle(page);
      const m = await page.evaluate(measure);
      const menu = await page.evaluate(() => {
        const links = [...document.querySelectorAll(".lang--bar .lang__link")];
        const vw = document.documentElement.clientWidth;
        return {
          count: links.length,
          inside: links.every((a) => { const r = a.getBoundingClientRect(); return r.left >= 0 && r.right <= vw && r.width > 0; }),
          current: (document.querySelector('.lang--bar .lang__link[aria-current="true"]') || {}).hreflang,
        };
      });
      check(m.scrollWidth <= m.clientWidth && !m.wide.length && !m.scrollers.length && menu.count === LANGS.length && menu.inside && menu.current.split("-")[0] === lang,
        `${mode}: /${lang}/ the open language menu fits at ${width}px`, { m, menu });
      await page.evaluate(() => { document.querySelector(".lang--bar").open = false; });
    }
  }
  await page.setViewportSize({ width: 390, height: 800 });
  await page.goto(`${base}/de/`);
  await page.focus(".lang--bar summary");
  await page.keyboard.press("Enter");
  const opened = await page.evaluate(() => document.querySelector(".lang--bar").open);
  await page.keyboard.press("Tab");
  const focused = await page.evaluate(() => document.activeElement.classList.contains("lang__link"));
  await page.keyboard.press("Escape");
  const closed = await page.evaluate(() => ({ open: document.querySelector(".lang--bar").open, focus: document.activeElement.tagName }));
  check(opened && focused && !closed.open && closed.focus === "SUMMARY", `${mode}: the language menu with the keyboard`, { opened, focused, closed });
  await page.click(".lang--bar summary");
  // On the left of the page, away from the menu, which hangs on the right.
  await page.mouse.click(8, 600);
  check(!(await page.evaluate(() => document.querySelector(".lang--bar").open)), `${mode}: a click elsewhere closes the language menu`);
  await context.close();
}

// The editor wraps long lines; each line number stays level with its line.
async function editorWraps(browser, base) {
  const context = await browser.newContext({ serviceWorkers: "block", viewport: { width: 320, height: 800 } });
  const page = await context.newPage();
  await page.goto(`${base}/en/playground/`);
  await ran(page);
  const long = `# ${"a-very-long-comment ".repeat(8)}\nprint("${"x".repeat(160)}")\n\nfn add(a, b) =\n  a + b\nprint(add(1, 2)) # ${"https://example.com/".repeat(4)}\n`;
  await page.fill("#source", long);
  for (const width of [320, 390, 768, 1280]) {
    await page.setViewportSize({ width, height: 800 });
    await settle(page);
    const m = await page.evaluate(measure);
    check(m.scrollWidth <= m.clientWidth && !m.wide.length && !m.scrollers.length, `the editor wraps long lines at ${width}px`, m);
    const aligned = await page.evaluate(() => {
      const nums = [...document.querySelectorAll("#gutter > *")];
      const lines = [...document.querySelectorAll("#highlight > *")];
      const source = document.getElementById("source");
      const same = source.clientWidth === document.getElementById("highlight").clientWidth;
      return {
        count: nums.length === lines.length && lines.length === source.value.split("\n").length,
        tops: nums.every((n, k) => Math.abs(n.getBoundingClientRect().top - lines[k].getBoundingClientRect().top) < 1),
        wrapped: lines[1].getBoundingClientRect().height > 2 * parseFloat(getComputedStyle(lines[0]).lineHeight),
        same,
      };
    });
    check(aligned.count && aligned.tops && aligned.wrapped && aligned.same, `the line numbers stay level with the wrapped lines at ${width}px`, aligned);
  }
  // The text typed lies exactly on its colors: same lines in the field and under it.
  const caret = await page.evaluate(() => {
    const source = document.getElementById("source");
    const pre = document.getElementById("highlight");
    return Math.abs(source.scrollHeight - pre.scrollHeight) <= 1;
  });
  check(caret, "the field and its colors wrap the same way");
  await context.close();
}

// Sharing in the playground, in every language: the notice of a program opened from a
// link and the box of the link made by Share, open together; no horizontal scroll at
// any width, in any theme, and their texts at the contrast of each theme.
async function shareLayout(browser, base, mode) {
  const context = await browser.newContext({ serviceWorkers: "block", viewport: { width: 320, height: 800 } });
  const page = await context.newPage();
  const failed = [];
  for (const lang of LANGS) {
    const code = encodeURIComponent(`print("${"x".repeat(200)}")\n`);
    await page.goto(`${base}/${lang}/playground/#code=${code}`);
    await ran(page);
    await page.click("#share");
    await page.waitForFunction(() => document.getElementById("share-msg").textContent !== "");
    const open = await page.evaluate(() => !document.getElementById("shared-note").hidden && !document.getElementById("share-box").hidden && !!document.getElementById("share-link").value);
    if (!open) failed.push(`/${lang}/: the notice and the box of the link are not both open`);
    for (const theme of THEMES) {
      await setTheme(page, theme);
      for (const width of [320, 360, 390, 768, 1280]) {
        await page.setViewportSize({ width, height: 800 });
        await settle(page);
        const m = await page.evaluate(measure);
        if (m.scrollWidth > m.clientWidth || m.wide.length || m.scrollers.length) failed.push(`/${lang}/ ${theme} ${width}px: ${JSON.stringify(m)}`);
        if (theme !== "auto" && (width === 320 || width === 1280)) {
          await page.waitForTimeout(300);
          const result = await page.evaluate(contrast, MIN_CONTRAST[theme] || 4.5);
          if (result.bad.length) failed.push(`/${lang}/ ${theme} ${width}px: contrast ${result.bad.join("; ")}`);
        }
      }
    }
    await page.setViewportSize({ width: 320, height: 800 });
  }
  check(failed.length === 0, `${mode}: sharing (the notice and the link) fits and reads well in every language, theme and width`, `\n    ${failed.slice(0, 20).join("\n    ")}`);
  await context.close();
}

async function themes(browser, base, mode) {
  // The contrast of every theme: its palette, then the text as drawn on every page.
  const context = await browser.newContext({ serviceWorkers: "block", viewport: { width: 390, height: 800 } });
  const page = await context.newPage();
  await page.goto(`${base}/en/`);
  const table = [];
  for (const theme of THEMES.slice(1)) {
    await setTheme(page, theme);
    const pairs = await page.evaluate(palette);
    const min = MIN_CONTRAST[theme] || 4.5;
    const low = pairs.filter((p) => !p.ok || p.ratio < min);
    const lowest = Math.min(...pairs.map((p) => p.ratio));
    const ink = pairs.find((p) => p.pair === "ink/paper").ratio;
    table.push(`${theme}: text ${ink.toFixed(2)}:1, lowest pair ${lowest.toFixed(2)}:1`);
    check(low.length === 0, `${mode}: the palette of ${theme} has every pair at ${min}:1 or more (text ${ink.toFixed(2)}:1, lowest ${lowest.toFixed(2)}:1)`, low.map((p) => `${p.pair} ${p.ratio.toFixed(2)}`).join(", "));
  }
  if (mode === "server") console.log(`  info ${table.join("; ")}`);
  let low = 0;
  for (const width of [390, 1280]) {
    await page.setViewportSize({ width, height: 800 });
    for (const lang of LANGS) {
      for (const p of PAGES) {
        await page.goto(`${base}/${lang}/${p}`);
        if (p === "playground/") await ran(page);
        for (const theme of THEMES.slice(1)) {
          await setTheme(page, theme);
          // The colors of buttons and links change with a short transition.
          await page.waitForTimeout(300);
          const min = MIN_CONTRAST[theme] || 4.5;
          const result = await page.evaluate(contrast, min);
          if (result.bad.length) {
            low += 1;
            check(false, `${mode}: contrast of /${lang}/${p} in ${theme} at ${width}px (≥ ${min}:1)`, result.bad.join("; "));
          }
        }
      }
    }
  }
  check(low === 0, `${mode}: every text of every page reaches its contrast in every theme, at 390 and 1280px`);
  await context.close();

  // The picker, with the keyboard only, in the menu of a phone and in the bar of a
  // computer; the choice is kept, and applied before the first paint.
  for (const width of [390, 1280]) {
    const ctx = await browser.newContext({ serviceWorkers: "block", viewport: { width, height: 800 }, colorScheme: "light" });
    const tab = await ctx.newPage();
    await tab.goto(`${base}/fr/`);
    const wide = width >= 860;
    if (wide) {
      await tab.focus("#theme-toggle");
      await tab.keyboard.press("Enter");
      check(await tab.getAttribute("#theme-toggle", "aria-expanded") === "true" && await tab.isVisible("#theme-panel"), `${width}px: Enter opens the theme picker`);
    } else {
      await tab.focus(".burger");
      await tab.keyboard.press("Enter");
      check(await tab.isVisible("#theme-panel"), `${width}px: the theme picker is in the menu`);
      await tab.focus("#theme-panel input:checked");
    }
    const focused = await tab.evaluate(() => document.activeElement && document.activeElement.name === "theme" ? document.activeElement.value : null);
    check(focused === "auto", `${width}px: the focus is on the chosen theme (Auto)`, focused);
    await tab.keyboard.press("ArrowDown");
    await tab.keyboard.press("ArrowDown");
    const chosen = await tab.evaluate((key) => ({
      theme: document.documentElement.dataset.theme,
      stored: localStorage.getItem(key),
      checked: document.querySelector('#theme-panel input:checked').value,
      color: document.querySelector('meta[name="theme-color"]').content,
      bg: getComputedStyle(document.body).backgroundColor,
    }), STORAGE);
    check(chosen.theme === "dark" && chosen.stored === "dark" && chosen.checked === "dark", `${width}px: the arrow keys choose a theme (Dark)`, chosen);
    check(chosen.color === "#0e1118" && chosen.bg === "rgb(14, 17, 24)", `${width}px: the page and the theme-color follow the theme`, chosen);
    const labels = await tab.evaluate(() => [...document.querySelectorAll("#theme-panel input")].map((i) => i.labels[0] && i.labels[0].textContent.trim()));
    check(labels.length === THEMES.length && labels.every(Boolean) && labels.includes("Sombre"), `${width}px: every theme has a label, in French`, labels);
    if (wide) {
      await tab.keyboard.press("Escape");
      const closed = await tab.evaluate(() => ({ expanded: document.getElementById("theme-toggle").getAttribute("aria-expanded"), focus: document.activeElement.id }));
      check(closed.expanded === "false" && closed.focus === "theme-toggle" && !(await tab.isVisible("#theme-panel")), `${width}px: Escape closes the picker and gives the focus back`, closed);
    }
    // Kept for the next page, the other language, and a new visit.
    await tab.goto(`${base}/en/learn/`);
    const kept = await tab.evaluate(() => ({ theme: document.documentElement.dataset.theme, checked: document.querySelector("#theme-panel input:checked").value }));
    check(kept.theme === "dark" && kept.checked === "dark", `${width}px: the theme is kept from page to page`, kept);
    await ctx.close();
  }

  // Before the first paint: the theme is on <html> as soon as <body> exists, before any
  // deferred script, and the system's choice is followed in Auto.
  const ctx = await browser.newContext({ serviceWorkers: "block", viewport: { width: 390, height: 800 }, colorScheme: "dark" });
  await ctx.addInitScript(() => {
    window.firstPaint = null;
    new MutationObserver((records, observer) => {
      if (document.body && !window.firstPaint) {
        window.firstPaint = { theme: document.documentElement.getAttribute("data-theme"), js: document.documentElement.classList.contains("js"), color: (document.querySelector('meta[name="theme-color"]') || {}).content };
        observer.disconnect();
      }
    }).observe(document, { childList: true, subtree: true });
  });
  const tab = await ctx.newPage();
  await tab.goto(`${base}/fr/`);
  const auto = await tab.evaluate(() => ({ first: window.firstPaint, bg: getComputedStyle(document.body).backgroundColor }));
  check(auto.first && auto.first.theme === null && auto.first.js && auto.bg === "rgb(14, 17, 24)", "Auto follows the system (dark here), without an attribute", auto);
  for (const theme of THEMES.slice(1)) {
    await tab.evaluate(([key, t]) => localStorage.setItem(key, t), [STORAGE, theme]);
    for (const p of ["", "playground/", "offline/", "nothing-here/"]) {
      await tab.goto(`${base}/en/${p}`);
      const first = await tab.evaluate(() => window.firstPaint);
      const expected = await tab.evaluate((t) => document.querySelector(`#theme-panel input[value="${t}"]`).dataset.color, theme);
      check(first && first.theme === theme && first.color === expected, `${mode}: ${theme} is set before the first paint on /en/${p}`, first);
    }
  }
  // A value that is not a theme, or blocked storage, falls back to Auto.
  await tab.evaluate((key) => localStorage.setItem(key, "constructor"), STORAGE);
  await tab.goto(`${base}/en/`);
  check(await tab.evaluate(() => window.firstPaint.theme) === null, "an unknown stored value falls back to Auto");
  await ctx.close();
}

async function verify(mode) {
  console.log(`\n== ${mode === "server" ? "the server, at the root" : "the static export, behind /Rythme"}`);
  const dir = workspace();
  const port = await freePort();
  const site = await (mode === "server" ? startServer : startExport)(BIN, dir, port);
  const base = `http://localhost:${port}${mode === "server" ? "" : "/Rythme"}`;
  const browser = await launch();
  try {
    await noHorizontalScroll(browser, base, mode);
    await languageMenu(browser, base, mode);
    await editorWraps(browser, base);
    await shareLayout(browser, base, mode);
    await themes(browser, base, mode);
  } finally {
    await browser.close();
    await site.stop();
    fs.rmSync(dir, { recursive: true, force: true });
  }
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
