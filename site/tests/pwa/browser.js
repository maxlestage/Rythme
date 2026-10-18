// Chromium for the scripts of this folder, through HTTPS_PROXY when there is one (for
// the Google Fonts of the pages); the local site itself is reached directly.
"use strict";

const { chromium } = require("playwright");

exports.launch = () => {
  const proxy = process.env.HTTPS_PROXY || process.env.https_proxy;
  if (!proxy) return chromium.launch();
  // Playwright sends the local addresses to the proxy too, unless told not to.
  process.env.PLAYWRIGHT_DISABLE_FORCED_CHROMIUM_PROXIED_LOOPBACK = "1";
  return chromium.launch({ proxy: { server: proxy, bypass: "localhost,127.0.0.1" } });
};
