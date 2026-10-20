// The site as deployed, for the scripts of this folder: the server at the root (as on
// Heroku) or the static export behind /Rythme served like GitHub Pages does. Each run
// works on a copy of public/ in a temporary folder, which a "deployment" may change.
"use strict";

const fs = require("fs");
const http = require("http");
const os = require("os");
const path = require("path");
const { execFileSync, spawn } = require("child_process");

const SITE = path.join(__dirname, "..", "..");

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

function freePort() {
  return new Promise((resolve) => {
    const server = http.createServer().listen(0, () => {
      const { port } = server.address();
      server.close(() => resolve(port));
    });
  });
}

async function waitFor(url) {
  for (let n = 0; n < 200; n++) {
    const ok = await new Promise((resolve) => {
      http.get(url, (res) => { res.resume(); resolve(res.statusCode < 500); }).on("error", () => resolve(false));
    });
    if (ok) return;
    await sleep(100);
  }
  throw new Error(`${url} does not answer`);
}

// A copy of public/.
function workspace() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "rythme-pwa-"));
  fs.cpSync(path.join(SITE, "public"), path.join(dir, "public"), { recursive: true });
  return dir;
}

// The server, as on Heroku.
async function startServer(bin, dir, port) {
  const child = spawn(bin, [], { cwd: dir, env: { ...process.env, PORT: String(port), SITE_DIR: dir }, stdio: "ignore" });
  await waitFor(`http://localhost:${port}/fr/`);
  return { stop: () => new Promise((resolve) => { child.once("exit", resolve); child.kill(); }) };
}

const TYPES = {
  ".html": "text/html; charset=utf-8", ".css": "text/css; charset=utf-8", ".js": "application/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8", ".webmanifest": "application/manifest+json", ".wasm": "application/wasm",
  ".png": "image/png", ".svg": "image/svg+xml", ".ico": "image/x-icon", ".xml": "application/xml", ".txt": "text/plain; charset=utf-8",
  ".md": "text/markdown; charset=utf-8",
};

// The static export behind /Rythme, served like GitHub Pages does (types, 404.html,
// a cache of 10 minutes).
async function startExport(bin, dir, port) {
  const dist = path.join(dir, "dist");
  fs.rmSync(dist, { recursive: true, force: true });
  const env = { ...process.env, SITE_DIR: dir, BASE_PATH: "/Rythme", SITE_URL: `http://localhost:${port}/Rythme` };
  execFileSync(bin, ["export", dist], { cwd: dir, env, stdio: "ignore" });
  const server = http.createServer((req, res) => {
    const url = new URL(req.url, "http://localhost");
    if (url.pathname === "/Rythme") return res.writeHead(301, { location: "/Rythme/" }).end();
    let file = url.pathname.startsWith("/Rythme/") ? path.join(dist, decodeURIComponent(url.pathname.slice(8))) : null;
    if (file && fs.existsSync(file) && fs.statSync(file).isDirectory()) {
      if (!url.pathname.endsWith("/")) return res.writeHead(301, { location: `${url.pathname}/` }).end();
      file = path.join(file, "index.html");
    }
    if (!file || !file.startsWith(dist) || !fs.existsSync(file)) {
      return res.writeHead(404, { "content-type": TYPES[".html"] }).end(fs.readFileSync(path.join(dist, "404.html")));
    }
    res.writeHead(200, { "content-type": TYPES[path.extname(file)] || "application/octet-stream", "cache-control": "max-age=600" });
    res.end(fs.readFileSync(file));
  });
  await new Promise((resolve) => server.listen(port, resolve));
  return { stop: () => new Promise((resolve) => { server.close(resolve); server.closeAllConnections(); }) };
}

// The binary given on the command line, or the release build.
function binary(arg) {
  const bin = path.resolve(arg || path.join(SITE, "target", "release", "rythme-site"));
  if (!fs.existsSync(bin)) {
    console.error(`${bin} is missing: build the site (cargo build --release) or give its path`);
    process.exit(2);
  }
  return bin;
}

// The behavior of the pages is Rust compiled to WebAssembly: it starts once its bundle
// is downloaded and compiled, which can be just after the `load` event that `goto`
// waits for. Its first act is to show the burger menu: wait for that.
function started(page) {
  return page.waitForFunction(() => {
    const burger = document.querySelector(".burger");
    return !!burger && !burger.hidden;
  }, null, { timeout: 20000 });
}

// A check that prints its result and counts the failures.
function checker() {
  const state = { failures: 0 };
  state.check = (ok, what, detail) => {
    console.log(`${ok ? "  ok  " : "  FAIL"} ${what}${!ok && detail !== undefined ? `: ${typeof detail === "string" ? detail : JSON.stringify(detail)}` : ""}`);
    if (!ok) state.failures += 1;
    return ok;
  };
  return state;
}

module.exports = { SITE, sleep, freePort, workspace, startServer, startExport, binary, checker, started };
