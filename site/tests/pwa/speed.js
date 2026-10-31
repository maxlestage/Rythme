// How fast the site is on a slow phone: Chromium with Lighthouse's mobile profile (the
// CPU 4× slower, 1.6 Mb/s down, 750 kb/s up, 150 ms of latency), a fresh browser for
// each measure (nothing cached, the service worker allowed), as deployed:
//
//   node speed.js <rythme-site binary> [server|export] [runs] [devtools|shared]
//
// `devtools` (by default) slows the network as Chrome's DevTools and Lighthouse do: for
// the page and its workers, but not for the service worker. `shared` puts the site
// behind one link of 1.6 Mb/s (150 ms before each answer) that the page, its workers and
// the service worker share, as a real phone does: what the service worker saves takes
// the network from the page.
//
// Three times, from the start of the navigation (medians printed at the end):
// - home: the home page (`/fr/`) until its menus work (the burger shown: the site's
//   bundle has started);
// - menus: the same on the playground, whose interpreter (3 MB) arrives meanwhile;
// - run: the playground until its first program has run (the status says how it went).
// A first visit warms up the connection to Google Fonts and is not counted. It measures,
// it checks nothing: the exit code is 0 unless the site does not start.
"use strict";

const http = require("http");
const { launch } = require("./browser");
const { workspace, freePort, startServer, startExport, binary, sleep } = require("./serve");

const BIN = binary(process.argv[2]);
const MODE = process.argv[3] || "server";
const RUNS = Number(process.argv[4] || 3);
const SHARED = process.argv[5] === "shared";
const RATE = (1.6 * 1024 * 1024) / 8 / 1000; // bytes per millisecond

// One link in front of the site: each answer waits 150 ms, then its bytes go through the
// link in turns with every other answer's.
async function sharedLink(target) {
  const port = await freePort();
  let free = 0;
  const server = http.createServer((req, res) => {
    const asked = Date.now();
    const upstream = http.request({ host: "localhost", port: target, path: req.url, method: req.method, headers: req.headers }, (answer) => {
      const chunks = [];
      answer.on("data", (chunk) => chunks.push(chunk));
      answer.on("end", async () => {
        const body = Buffer.concat(chunks);
        await sleep(Math.max(0, asked + 150 - Date.now()));
        res.writeHead(answer.statusCode, answer.headers);
        for (let at = 0; at < body.length; at += 8192) {
          const slice = body.subarray(at, at + 8192);
          const now = Date.now();
          free = Math.max(now, free) + slice.length / RATE;
          await sleep(free - now);
          res.write(slice);
        }
        res.end();
      });
    });
    upstream.on("error", () => res.writeHead(502).end());
    req.pipe(upstream);
  });
  await new Promise((resolve) => server.listen(port, resolve));
  return { port, stop: () => new Promise((resolve) => { server.close(resolve); server.closeAllConnections(); }) };
}

// Times taken in the page, from the start of the navigation (performance.now()).
const OBSERVE = () => {
  window.__speed = {};
  const mark = (name) => { if (!(name in window.__speed)) window.__speed[name] = performance.now(); };
  new MutationObserver(() => {
    const burger = document.querySelector(".burger");
    if (burger && !burger.hidden) mark("menus");
    const status = document.getElementById("status");
    if (status && /good|bad/.test(status.className)) mark("run");
  }).observe(document, { subtree: true, attributes: true, childList: true, attributeFilter: ["hidden", "class"] });
};

async function measure(browser, url, wanted) {
  const context = await browser.newContext({ viewport: { width: 412, height: 823 }, isMobile: true, hasTouch: true, deviceScaleFactor: 1.75 });
  await context.addInitScript(OBSERVE);
  const page = await context.newPage();
  const cdp = await context.newCDPSession(page);
  await cdp.send("Network.enable");
  if (!SHARED) await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: 150, downloadThroughput: (1.6 * 1024 * 1024) / 8, uploadThroughput: (750 * 1024) / 8 });
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: 4 });
  let bytes = 0;
  cdp.on("Network.loadingFinished", (e) => { bytes += e.encodedDataLength; });
  await page.goto(url, { waitUntil: "commit" });
  await page.waitForFunction((keys) => keys.every((k) => window.__speed && k in window.__speed), wanted, { timeout: 120000, polling: 50 });
  const times = await page.evaluate(() => window.__speed);
  await context.close();
  return { ...times, kb: Math.round(bytes / 1024) };
}

const median = (xs) => [...xs].sort((a, b) => a - b)[Math.floor(xs.length / 2)];

(async () => {
  const dir = workspace();
  const port = await freePort();
  const site = await (MODE === "server" ? startServer : startExport)(BIN, dir, port);
  const link = SHARED ? await sharedLink(port) : null;
  const base = `http://localhost:${link ? link.port : port}${MODE === "server" ? "" : "/Rythme"}`;
  const browser = await launch();
  const all = { home: [], menus: [], run: [] };
  try {
    await measure(browser, `${base}/fr/`, ["menus"]);
    for (let n = 1; n <= RUNS; n++) {
      // The server answers 100 requests a second to one visitor: a pause between visits.
      await sleep(1500);
      const home = await measure(browser, `${base}/fr/`, ["menus"]);
      await sleep(1500);
      const playground = await measure(browser, `${base}/fr/playground/`, ["menus", "run"]);
      all.home.push(home.menus);
      all.menus.push(playground.menus);
      all.run.push(playground.run);
      console.log(`  ${n}: home ${Math.round(home.menus)} ms (${home.kb} KB), playground menus ${Math.round(playground.menus)} ms, first run ${Math.round(playground.run)} ms (${playground.kb} KB)`);
    }
  } finally {
    await browser.close();
    if (link) await link.stop();
    await site.stop();
  }
  console.log(`${MODE}, ${SHARED ? "shared link" : "DevTools throttling"}, median of ${RUNS}: home ${Math.round(median(all.home))} ms, playground menus ${Math.round(median(all.menus))} ms, first run ${Math.round(median(all.run))} ms`);
})().catch((err) => {
  console.error(err);
  process.exit(1);
});
