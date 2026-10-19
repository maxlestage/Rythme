// The service worker of the Rythme website, written by the server (src/pwa.rs) with the
// current version and the list of the files of the site: every page in every language,
// the playground and its interpreter work offline after the first visit.
//
// - versioned files (`?v=`): the saved copy, without asking the network;
// - other files of the site: the saved copy at once, refreshed in the background;
// - pages: the network first, so that updates arrive; the saved copy when the network
//   is down or slower than TIMEOUT; the offline page for a page never saved;
// - Google Fonts: kept in a cache of their own; other sites are not touched.
// Only complete answers are saved: never an error, a redirection or another site's.
"use strict";

const VERSION = __VERSION__;
const CACHE = `rythme-${VERSION}`;
const FONTS_CACHE = "rythme-fonts";
const BASE = __BASE__;
const PRECACHE = __PRECACHE__;
const OFFLINE = __OFFLINE__;
const FONTS = __FONTS__;
const FONT_FILES = "https://fonts.gstatic.com";
const TIMEOUT = 3000;

self.addEventListener("install", (event) => {
  // Revalidated with the server: a stale copy of the HTTP cache is never saved. The new
  // version waits until the page asks for it (the "Reload" button), see site.js.
  const files = caches.open(CACHE).then((cache) => cache.addAll(PRECACHE.map((url) => new Request(url, { cache: "no-cache" }))));
  event.waitUntil(Promise.all([files, saveFonts().catch(() => {})]));
});

// The stylesheet of the fonts and the font files it names, for the first offline visit.
async function saveFonts() {
  const cache = await caches.open(FONTS_CACHE);
  if (await cache.match(FONTS)) return;
  const css = await fetch(FONTS, { mode: "cors" });
  if (!css.ok) return;
  const urls = [...(await css.clone().text()).matchAll(/url\((https:\/\/fonts\.gstatic\.com\/[^)\s]+)\)/g)].map((m) => m[1]);
  await cache.addAll(urls.map((url) => new Request(url, { mode: "cors" })));
  await cache.put(FONTS, css);
}

self.addEventListener("message", (event) => {
  if (event.data === "skip-waiting") self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys()
      .then((keys) => Promise.all(keys.filter((k) => k.startsWith("rythme-") && k !== CACHE && k !== FONTS_CACHE).map((k) => caches.delete(k))))
      .then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  if (request.method !== "GET") return;
  const url = new URL(request.url);
  if (url.href === FONTS || url.origin === FONT_FILES) {
    event.respondWith(font(event, url));
  } else if (url.origin === self.location.origin && url.pathname.startsWith(`${BASE}/`) && url.pathname !== `${BASE}/sw.js`) {
    event.respondWith(request.mode === "navigate" ? page(event, url) : file(event, url));
  }
});

function complete(response) {
  return response.ok && response.type === "basic" && !response.redirected;
}

async function file(event, url) {
  const cache = await caches.open(CACHE);
  const cached = await cache.match(event.request, { ignoreVary: true });
  if (cached && url.searchParams.has("v")) return cached;
  const network = fetch(event.request).then(async (response) => {
    if (complete(response)) await cache.put(event.request, response.clone());
    return response;
  });
  if (!cached) return network;
  event.waitUntil(network.catch(() => {}));
  return cached;
}

// The language (`fr`, `en`, `es`...): from the address, else the one chosen with the
// switch (the cookie of site.js, where the browser lets a worker read it), else the
// browser's.
async function lang(url) {
  const first = url.pathname.slice(BASE.length).split("/")[1];
  if (first in OFFLINE) return first;
  const chosen = self.cookieStore ? await self.cookieStore.get("rythme-lang").catch(() => null) : null;
  if (chosen && chosen.value in OFFLINE) return chosen.value;
  const asked = (self.navigator.languages || []).map((l) => l.slice(0, 2).toLowerCase()).find((l) => l in OFFLINE);
  return asked || "fr";
}

async function page(event, url) {
  const cache = await caches.open(CACHE);
  // Saved without its query: `?text=…` of a shared text is the same playground.
  // Revalidated with the server, never from the HTTP cache (GitHub Pages keeps pages 10
  // minutes): a new deployment shows at once, and its new worker is found at once.
  const network = fetch(event.request, { cache: "no-cache" }).then(async (response) => {
    if (complete(response)) await cache.put(url.pathname, response.clone());
    return response;
  });
  event.waitUntil(network.catch(() => {}));
  const late = new Promise((resolve) => setTimeout(resolve, TIMEOUT, null));
  const fresh = await Promise.race([network, late]).catch(() => null);
  if (fresh) return fresh;
  const cached = await cache.match(url.pathname, { ignoreVary: true });
  if (cached) return cached;
  // `/fr/learn`: `/fr/learn/`, as the server redirects.
  if (!url.pathname.endsWith("/") && (await cache.match(`${url.pathname}/`))) {
    return Response.redirect(`${url.pathname}/${url.search}${url.hash}`, 301);
  }
  // A slow network and nothing saved: its answer after all.
  const answer = await network.catch(() => null);
  if (answer) return answer;
  // `/`: the home page in the browser's language, as the server or the static page does.
  if (url.pathname === `${BASE}/` || url.pathname === `${BASE}/index.html`) return Response.redirect(`${BASE}/${await lang(url)}/`, 302);
  return (await cache.match(OFFLINE[await lang(url)])) || Response.error();
}

async function font(event, url) {
  const cache = await caches.open(FONTS_CACHE);
  const cached = await cache.match(event.request, { ignoreVary: true });
  const network = fetch(event.request).then(async (response) => {
    // The page asks for the stylesheet without CORS: an opaque answer, whose status is
    // unknown, so it is replaced at the next visit.
    if (response.ok || response.type === "opaque") await cache.put(event.request, response.clone());
    return response;
  });
  if (cached) {
    if (url.origin !== FONT_FILES) event.waitUntil(network.catch(() => {}));
    return cached;
  }
  // Offline, never saved: no stylesheet rather than an error; the site has its own fonts.
  return network.catch(() => (url.origin === FONT_FILES ? Response.error() : new Response("", { headers: { "content-type": "text/css" } })));
}
