// Makes the images of the installable app, in public/ (see README.md):
//   node images.js icons               the icons and favicon.ico, from assets/logo.svg
//   node images.js screenshots <url> [lang...]   the screenshots of the install dialog,
//                                      in every language (or those given), taken on a
//                                      running site (e.g. http://localhost:3000)
"use strict";

const fs = require("fs");
const path = require("path");
const { launch } = require("./browser");

// The languages of the site (src/i18n.rs).
const LANGS = ["fr", "en", "es", "de", "pt", "zh"];
const PUBLIC = path.join(__dirname, "..", "..", "public");
const LOGO = fs.readFileSync(path.join(PUBLIC, "assets", "logo.svg"), "utf8");
const BLUE = LOGO.match(/fill="(#[0-9a-f]{6})"/i)[1];
const DOTS = LOGO.match(/<g[\s\S]*<\/g>/)[0];

// The beats on a full square, scaled around the center: for masks (Android keeps the
// central circle, 80% of the size) and for iOS (which rounds the corners itself).
const square = (scale) =>
  `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><rect width="64" height="64" fill="${BLUE}"/>` +
  `<g transform="translate(32 32) scale(${scale}) translate(-32 -32)">${DOTS}</g></svg>`;

async function render(browser, svg, size) {
  const page = await browser.newPage({ viewport: { width: size, height: size } });
  await page.setContent(`<style>html,body{margin:0;background:transparent}svg{display:block;width:${size}px;height:${size}px}</style>${svg}`);
  const png = await page.screenshot({ omitBackground: true });
  await page.close();
  return png;
}

// An .ico file that holds PNG images (every browser reads them).
function ico(images) {
  const head = Buffer.alloc(6 + 16 * images.length);
  head.writeUInt16LE(1, 2);
  head.writeUInt16LE(images.length, 4);
  let offset = head.length;
  images.forEach(({ size, png }, n) => {
    const at = 6 + 16 * n;
    head.writeUInt8(size % 256, at);
    head.writeUInt8(size % 256, at + 1);
    head.writeUInt16LE(1, at + 4);
    head.writeUInt16LE(32, at + 6);
    head.writeUInt32LE(png.length, at + 8);
    head.writeUInt32LE(offset, at + 12);
    offset += png.length;
  });
  return Buffer.concat([head, ...images.map((i) => i.png)]);
}

async function icons(browser) {
  const dir = path.join(PUBLIC, "icons");
  fs.mkdirSync(dir, { recursive: true });
  const files = {
    "icon-192.png": [LOGO, 192],
    "icon-512.png": [LOGO, 512],
    "icon-maskable-512.png": [square(0.72), 512],
    "apple-touch-icon.png": [square(0.86), 180],
  };
  for (const [name, [svg, size]] of Object.entries(files)) {
    fs.writeFileSync(path.join(dir, name), await render(browser, svg, size));
    console.log(`public/icons/${name}`);
  }
  const small = [];
  for (const size of [16, 32, 48]) small.push({ size, png: await render(browser, LOGO, size) });
  fs.writeFileSync(path.join(PUBLIC, "favicon.ico"), ico(small));
  console.log("public/favicon.ico");
}

async function screenshots(browser, url, langs) {
  const dir = path.join(PUBLIC, "screenshots");
  fs.mkdirSync(dir, { recursive: true });
  const forms = { narrow: { width: 412, height: 915 }, wide: { width: 1280, height: 800 } };
  for (const lang of langs) {
    for (const [form, viewport] of Object.entries(forms)) {
      // Without a service worker: nothing but the page itself.
      const context = await browser.newContext({ viewport, colorScheme: "light", reducedMotion: "reduce", serviceWorkers: "block", locale: lang });
      const page = await context.newPage();
      for (const name of ["home", "playground"]) {
        await page.goto(`${url}/${lang}/${name === "home" ? "" : name + "/"}`, { waitUntil: "networkidle" });
        if (name === "playground") await page.waitForSelector("#status.good", { timeout: 20000 });
        // The playground brings its output into view on a phone: back to the top once
        // it has.
        await page.waitForTimeout(1000);
        await page.evaluate(() => {
          document.documentElement.style.scrollBehavior = "auto";
          window.scrollTo(0, 0);
          return document.fonts.ready;
        });
        await page.waitForFunction(() => { window.scrollTo(0, 0); return window.scrollY === 0; }, null, { polling: 200 });
        await page.waitForTimeout(300);
        await page.screenshot({ path: path.join(dir, `${lang}-${name}-${form}.png`) });
        console.log(`public/screenshots/${lang}-${name}-${form}.png`);
      }
      await context.close();
    }
  }
}

(async () => {
  const [what, url, ...langs] = process.argv.slice(2);
  if (what !== "icons" && !(what === "screenshots" && url)) {
    console.error("usage: node images.js icons | screenshots <url of the site>");
    process.exit(2);
  }
  const browser = await launch();
  try {
    if (what === "icons") await icons(browser);
    else await screenshots(browser, url.replace(/\/$/, ""), langs.length ? langs : LANGS);
  } finally {
    await browser.close();
  }
})();
