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
const { workspace, freePort, startServer, startExport, binary, checker, started } = require("./serve");

const BIN = binary(process.argv[2]);
const MODES = process.argv[3] ? [process.argv[3]] : ["server", "export"];
const PAGES = ["", "learn/", "examples/", "playground/", "teach/", "ai/"];
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

// What the steps show: the count, the step before, the step, what it printed, the error,
// the live region, the slider, and whether the output is hidden.
const stepsOf = (page) => page.evaluate(() => {
  const $ = (id) => document.getElementById(id);
  return {
    shown: !$("stepper").hidden && $("output").hidden,
    count: $("step-count").textContent,
    from: $("step-from").textContent.trim(),
    now: $("step-now").textContent.trim(),
    printed: $("step-printed").hidden ? null : $("step-print").textContent,
    error: $("step-error").hidden ? null : $("step-error").textContent,
    live: $("step-live").textContent,
    range: [$("step-range").value, $("step-range").max],
    prev: $("step-prev").disabled,
    next: $("step-next").disabled,
  };
});

// The steps of a calculation, offline: a link with `&mode=steps&step=3` opens on that
// step; the buttons, the keys and the slider move; Share keeps the mode, the step and
// the selected code; the selection, print, errors and a program without expression.
async function stepsLinks(context, page, base) {
  const FIB = 'fn fib(0) = 0\nfn fib(1) = 1\nfn fib(n) = fib(n - 1) + fib(n - 2)\n\nprint("before")\nfib(4)\n';
  // `rythme steps` on FIB: fib(4) → fib(3) + fib(2) → ... → 3 (14 in all).
  const STEPS = ["fib(4)", "fib(3) + fib(2)", "fib(2) + fib(1) + fib(2)", "fib(1) + fib(0) + fib(1) + fib(2)", "1 + fib(0) + fib(1) + fib(2)"];
  const of = (n, total) => `Step ${n} of ${total}`;
  await open(page, `${base}/en/playground/#code=${encodeURIComponent(FIB)}&mode=steps&step=3`);
  let s = await stepsOf(page);
  check(s.shown && s.count === of(3, 14) && s.now === STEPS[2] && s.from === STEPS[1] && s.range.join() === "3,14", "steps: a link with mode=steps&step=3 opens on step 3 of the last expression", s);
  check(s.live.startsWith(of(3, 14)) && s.live.includes(STEPS[2]) && (await page.getAttribute("#step-live", "aria-live")) === "polite", "steps: the live region says the step", s.live);
  check(s.printed === null && !(await page.textContent("#step-now")).includes("before"), "steps: only that expression, the others of the program are not run", s);
  await page.click("#step-next");
  s = await stepsOf(page);
  check(s.count === of(4, 14) && s.now === STEPS[3], "steps: Next", s);
  await page.click("#step-prev");
  await page.click("#step-prev");
  s = await stepsOf(page);
  check(s.count === of(2, 14) && s.now === STEPS[1], "steps: Previous", s);
  await page.focus("#step-now");
  await page.keyboard.press("ArrowRight");
  check((await stepsOf(page)).now === STEPS[2], "steps: the right arrow key");
  await page.keyboard.press("ArrowLeft");
  check((await stepsOf(page)).now === STEPS[1], "steps: the left arrow key");
  await page.keyboard.press("End");
  s = await stepsOf(page);
  check(s.count === of(14, 14) && s.now === "3" && s.next && !s.prev, "steps: End shows the result, Next is disabled", s);
  await page.keyboard.press("Home");
  s = await stepsOf(page);
  check(s.count === of(1, 14) && s.now === STEPS[0] && s.from === "" && s.prev, "steps: Home shows the expression, Previous is disabled", s);
  await page.$eval("#step-range", (e) => { e.value = "5"; e.dispatchEvent(new Event("input", { bubbles: true })); });
  check((await stepsOf(page)).now === STEPS[4], "steps: the slider");
  await page.focus("#step-range");
  await page.keyboard.press("ArrowRight");
  check((await stepsOf(page)).count === of(6, 14), "steps: the slider with the keyboard");

  // Share from the steps: the link keeps the mode and the step, and opens on it.
  await context.grantPermissions(["clipboard-read", "clipboard-write"], { origin: new URL(base).origin });
  const link = await share(page);
  check(link.includes("#code=") && link.endsWith("&mode=steps&step=6"), "steps: Share keeps the steps mode and the step", link);
  await page.keyboard.press("Escape");
  const other = await context.newPage();
  await open(other, link);
  s = await stepsOf(other);
  check(s.shown && s.count === of(6, 14) && (await other.$eval("#source", (e) => e.value)) === FIB, "steps: the shared link opens on the same step", s);

  // Run goes back to the output; Share then makes a link without the steps.
  await runAndRead(other, "#run");
  s = await stepsOf(other);
  check(!s.shown && (await other.textContent("#output")).includes("before"), "steps: Run shows the output again", s);
  check(!(await share(other)).includes("mode=steps"), "steps: Share from the output makes a link without the steps");

  // The selected code: its steps, after the definitions (an error when it uses a name
  // it cannot see); Share keeps it.
  await setCode(other, FIB);
  await other.$eval("#source", (e) => { const at = e.value.indexOf("fib(n - 2)"); e.focus(); e.setSelectionRange(at, at + 10); });
  await runAndRead(other, "#steps");
  s = await stepsOf(other);
  check(s.shown && s.error && /error\[/.test(s.error) && (await other.isHidden(".stepper__nav")), "steps: the selected code, with an error said so", s);
  await setCode(other, `${FIB}fib(5)
`);
  await other.$eval("#source", (e) => { const at = e.value.lastIndexOf("fib(4)"); e.focus(); e.setSelectionRange(at, at + 6); });
  await runAndRead(other, "#steps");
  s = await stepsOf(other);
  check(s.count === of(1, 14) && s.now === "fib(4)", "steps: the selected expression rather than the last one", s);
  await other.click("#step-next");
  const selected = await share(other);
  check(selected.endsWith("&mode=steps&expr=fib(4)&step=2"), "steps: Share keeps the selected code", selected);
  await other.keyboard.press("Escape");
  await open(other, selected.replace("/en/", "/fr/"));
  s = await stepsOf(other);
  check(s.count === "Étape 2 sur 14" && s.now === STEPS[1], "steps: the link of a selected code opens on its steps, in French too", s);

  // What a step prints is shown with the step that follows; an error after the last step.
  await setCode(other, 'fn twice(t) = t + t\nprint(twice("ab"))\n');
  await runAndRead(other, "#steps");
  await other.focus("#step-now");
  await other.keyboard.press("End");
  s = await stepsOf(other);
  check(s.now === "()" && s.printed === "abab", "steps: what the program prints", s);
  await setCode(other, "fn half(n) = 10 / n\nhalf(5 - 5)\n");
  await runAndRead(other, "#steps");
  await other.focus("#step-now");
  await other.keyboard.press("End");
  s = await stepsOf(other);
  check(s.error && /erreur\[/.test(s.error) && (await other.getAttribute("#status", "class")).includes("bad"), "steps: an error is shown after the last step", s);
  await setCode(other, "fn half(n) = n / 2\n");
  await runAndRead(other, "#steps");
  s = await stepsOf(other);
  const none = await other.evaluate(() => JSON.parse(document.querySelector(".playground").dataset.ui).stepsNone);
  check(s.shown && s.now === none && (await other.isHidden(".stepper__nav")), "steps: a program without expression says what to do", s);
  await other.close();

  // The tour of the language opens the playground on the steps of fib(4).
  await page.goto(`${base}/en/learn/`);
  await Promise.all([page.waitForURL(/mode=steps/), page.click(".tour__try--steps")]);
  await ran(page);
  s = await stepsOf(page);
  check(s.shown && s.count === of(1, 14) && s.now === "fib(4)", "steps: the link of the tour", s);
  await page.evaluate(() => { localStorage.removeItem("rythme-code"); });
}

// The Seed option, offline: empty, the draws change from run to run; a number, every
// run makes the same draws (Test too, with check_all's values), shown in the head of
// the output and kept after a reload; the dice picks another; Share puts it in the
// link, which reproduces the draws in another page without changing the seed saved
// there, until the program is changed.
async function seedOption(context, page, base) {
  const DRAWS = "print(random(1, 1000000), random(1, 1000000))\nprint(shuffle([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]))\n";
  const TESTS = 'test "draws" = assert_eq([random(1, 1000000), sample(ints(0, 1000000), 1)[0]], [])\n';
  // The field, typed into (the option opened first).
  const fillSeed = async (p, value) => {
    await p.evaluate(() => { document.getElementById("seed-option").open = true; });
    await p.fill("#seed", value);
  };
  const seedState = (p) => p.evaluate(() => ({
    field: document.getElementById("seed").value,
    shown: document.getElementById("seed-shown").textContent,
    used: document.getElementById("seed-used").hidden ? null : document.getElementById("seed-used").textContent,
    saved: localStorage.getItem("rythme-seed"),
  }));
  // The welcome program first: Run runs the program (not the tests of testing.ry).
  await page.goto(`${base}/en/playground/#example=welcome`);
  await ran(page);
  check(await page.isVisible("#seed-option") && !(await page.isVisible("#seed")), "seed: the option is shown, closed");
  await setCode(page, DRAWS);
  const free = [];
  for (let i = 0; i < 3; i++) free.push(await runAndRead(page, "#run"));
  check(new Set(free).size > 1 && (await seedState(page)).used === null, "seed: empty, the draws change from run to run", free);

  await page.click(".seed__toggle");
  check(await page.isVisible("#seed") && (await page.getAttribute("#seed", "placeholder")) === "random", "seed: the option opens on its field");
  await fillSeed(page, "4x2");
  let state = await seedState(page);
  check(state.field === "42" && state.shown === "42" && state.saved === "42", "seed: only digits, shown on the button and saved", state);
  const first = await runAndRead(page, "#run");
  const second = await runAndRead(page, "#run");
  state = await seedState(page);
  check(first === second && !free.includes(first) && state.used === "seed 42", "seed: 42, two runs make the same draws, said in the head of the output", { first, second, state });
  // A click elsewhere (Run) closed it; Escape too, from its field.
  check(!(await page.isVisible("#seed")), "seed: a click elsewhere closes the option");
  await page.click(".seed__toggle");
  await page.focus("#seed");
  await page.keyboard.press("Escape");
  check(!(await page.isVisible("#seed")) && (await page.evaluate(() => document.activeElement.classList.contains("seed__toggle"))), "seed: Escape closes the option and gives the focus back");
  await page.reload();
  await ran(page);
  state = await seedState(page);
  check(state.field === "42" && (await page.textContent("#output")) === first, "seed: kept after a reload, with the same draws", state);

  // Test: the same draws, and the same values of check_all and sample (the times of
  // the summary aside).
  const timeless = (s) => s.replace(/\(\d+(\.\d+)? m?s\)/g, "");
  await setCode(page, TESTS);
  const tested = timeless(await runAndRead(page, "#test"));
  check(/✗/.test(tested) && tested === timeless(await runAndRead(page, "#test")) && (await seedState(page)).used === "seed 42", "seed: Test makes the same draws each time", tested);
  await fillSeed(page, "7");
  check(timeless(await runAndRead(page, "#test")) !== tested, "seed: another seed, other draws in the tests");
  // Check runs nothing: no seed in the head of the output.
  await runAndRead(page, "#check");
  check((await seedState(page)).used === null, "seed: not shown after Check");

  // The dice: another seed, and a run with it.
  await setCode(page, DRAWS);
  await page.evaluate(() => { document.getElementById("seed-option").open = true; document.getElementById("status").className = "status"; });
  await page.click("#seed-new");
  await ran(page);
  state = await seedState(page);
  check(/^\d+$/.test(state.field) && state.field !== "7" && state.saved === state.field && state.used === `seed ${state.field}`, "seed: the dice picks a new seed and runs", state);
  await fillSeed(page, "42");
  check((await runAndRead(page, "#run")) === first, "seed: back to 42, the same draws as before");

  // Share: the seed travels in the link, before the mode.
  await context.grantPermissions(["clipboard-read", "clipboard-write"], { origin: new URL(base).origin });
  const link = await share(page);
  check(/#code=[01]:[\w-]+&seed=42$/.test(link), "seed: Share puts the seed in the link", link);
  await page.keyboard.press("Escape");
  await fillSeed(page, "");
  const other = await context.newPage();
  await open(other, link);
  state = await seedState(other);
  check((await other.textContent("#output")) === first && state.field === "42" && state.used === "seed 42" && state.saved === "", "seed: the link reproduces the draws, without changing the seed saved here", state);
  const again = await runAndRead(other, "#run");
  check(again === first, "seed: and every run of the link too");
  await other.click("#shared-back");
  await ran(other);
  check((await seedState(other)).field === "", "seed: Back to my draft brings its seed back (none)");
  // A link of the tests with a seed, made by hand: the seed comes before the mode.
  await open(other, `${base}/en/playground/#code=${encodeURIComponent(TESTS)}&seed=42&mode=test`);
  check(timeless(await other.textContent("#output")) === tested, "seed: a test link with a seed reproduces the draws", await other.textContent("#output"));
  // A link without seed: draws that change, even with a seed saved here.
  await fillSeed(page, "42");
  await open(other, `${base}/en/playground/#code=${encodeURIComponent(DRAWS)}`);
  const unseeded = [await other.textContent("#output"), await runAndRead(other, "#run"), await runAndRead(other, "#run")];
  check(new Set(unseeded).size > 1 && (await seedState(other)).field === "" && (await seedState(other)).saved === "42", "seed: a link without seed, draws that change; the saved seed is kept", unseeded);
  // Changing the seed of a link makes its program the draft, with that seed.
  await fillSeed(other, "5");
  state = await seedState(other);
  check(state.saved === "5" && !(await noteShown(other)) && (await draftOf(other)) === DRAWS, "seed: changing the seed of a link makes it the draft", state);
  await other.close();
  await fillSeed(page, "");
  await page.evaluate(() => { localStorage.removeItem("rythme-code"); localStorage.removeItem("rythme-seed"); });
}

// A clipboard made by hand, which keeps what is copied (`window.copied`), or refuses.
const fakeClipboard = (page, refuse) => page.evaluate((no) => {
  window.copied = [];
  const writeText = async (text) => {
    if (no) throw new Error("refused");
    window.copied.push(text);
  };
  Object.defineProperty(navigator, "clipboard", { configurable: true, value: { writeText } });
}, refuse);

// For AI assistants, offline: llms.txt and llms-full.txt; on the AI page, the copy
// buttons of the configurations of the MCP server; in the playground, Copy for AI: the
// program and what its last run gave (Run, Check, Test), as Markdown, with the address
// of llms-full.txt; the output left out once the code is changed; without a clipboard,
// the text in a field, selected; Escape closes the box.
async function forAssistants(page, base) {
  const full = await page.goto(`${base}/llms-full.txt`);
  check(full && full.ok() && (await full.text()).startsWith("# Rythme"), "offline: /llms-full.txt");
  const index = await page.goto(`${base}/llms.txt`);
  const indexText = index && index.ok() ? await index.text() : "";
  check(indexText.startsWith("# Rythme") && indexText.includes(`(${base}/llms-full.txt)`) && indexText.includes(`(${base}/en/learn/index.md)`), "offline: /llms.txt, with the addresses of the site", indexText.slice(0, 400));

  await page.goto(`${base}/en/ai/`);
  await fakeClipboard(page, false);
  const button = page.locator('.snippet__copy[data-copy*="mcpServers"]');
  await button.click();
  await page.waitForFunction(() => window.copied.length === 1);
  const mcp = await page.evaluate(() => window.copied[0]);
  check(JSON.parse(mcp).mcpServers.rythme.command === "rythme" && (await button.textContent()) === "Copied", "offline AI page: the configuration of the MCP server is copied", mcp);

  await page.goto(`${base}/en/playground/`);
  await ran(page);
  await fakeClipboard(page, false);
  const intro = `Here is a Rythme program and what it gives. Rythme is a new language: its whole reference is at ${base}/llms-full.txt.`;
  const copy = async () => {
    const before = await page.evaluate(() => window.copied.length);
    await page.click("#copy-ai");
    await page.waitForFunction((n) => window.copied.length > n || !document.getElementById("ai-row").hidden, before);
    return page.evaluate(() => ({
      text: window.copied[window.copied.length - 1],
      name: document.getElementById("file-name").textContent,
      msg: document.getElementById("share-msg").textContent,
      ui: JSON.parse(document.querySelector(".playground").dataset.ui),
      expanded: document.getElementById("copy-ai").getAttribute("aria-expanded"),
      row: !document.getElementById("ai-row").hidden,
    }));
  };
  const program = "fn double(x) = x * 2\nprint(map([1, 2, 3], double))\n";
  await setCode(page, program);
  await runAndRead(page, "#run");
  let got = await copy();
  const fence = (kind, text) => `\`\`\`${kind}\n${text}\n\`\`\``;
  let expected = `${intro}\n\n\`${got.name}\`:\n\n${fence("rythme", program.trimEnd())}\n\n\`rythme run ${got.name}\`:\n\n${fence("text", "[2, 4, 6]")}\n`;
  check(got.text === expected && got.msg === got.ui.aiCopied && got.expanded === "true" && !got.row, "offline playground: Copy for AI copies the program and its output as Markdown", got);
  await setCode(page, "fn double(x) = x * 2\nprint(doubel(2))\n");
  await runAndRead(page, "#check");
  got = await copy();
  check(got.text.includes(`\`rythme check ${got.name}\`:\n\n\`\`\`text\nerror[E003]`) && !/[\u0001\u0002]/.test(got.text), "offline playground: Copy for AI after Check: the error", got.text);
  await setCode(page, 'fn double(x) = x * 2\n\ntest "double" = assert_eq(double(2), 4)\n');
  await runAndRead(page, "#test");
  got = await copy();
  check(got.text.includes(`\`rythme test ${got.name}\`:`) && got.text.includes("✓"), "offline playground: Copy for AI after Test: the tests", got.text);
  await page.fill("#source", program);
  got = await copy();
  check(got.text === `${intro}\n\n\`${got.name}\`:\n\n${fence("rythme", program.trimEnd())}\n`, "offline playground: Copy for AI leaves out the output of other code", got.text);
  await fakeClipboard(page, true);
  got = await copy();
  const field = await page.evaluate(() => {
    const el = document.getElementById("ai-text");
    return { value: el.value, focused: document.activeElement === el, selected: el.selectionStart === 0 && el.selectionEnd === el.value.length };
  });
  check(got.row && got.msg === got.ui.aiSelect && field.value.startsWith(intro) && field.value.includes("```rythme\n") && field.focused && field.selected,
    "offline playground: without a clipboard, Copy for AI shows the text, selected", { got, field });
  await page.keyboard.press("Escape");
  check(await page.evaluate(() => document.getElementById("share-box").hidden && document.activeElement.id === "copy-ai" && document.getElementById("copy-ai").getAttribute("aria-expanded") === "false"),
    "offline playground: Escape closes the box and gives the focus back to Copy for AI");
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
  await started(ios);
  check(await ios.isVisible("#ios-hint"), "iOS: the hint to add the site to the home screen is shown");
  await ios.click("#ios-hint-close");
  await ios.reload();
  await started(ios);
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
  await started(page);
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
  await stepsLinks(context, page, base);
  await seedOption(context, page, base);
  await forAssistants(page, base);
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

  // (e2) The installed app opens, or comes back to the foreground, and finds a new
  // version before anything is touched: it comes in by itself, the code kept.
  const css = () => page.getAttribute('link[href*="site.css"]', "href");
  const deploy = async () => {
    await site.stop();
    deployChange(dir);
    site = await start(dir, port);
  };
  const lookForUpdate = () => page.evaluate(async () => (await navigator.serviceWorker.getRegistration()).update());
  // The app goes to the background (as when another app is opened).
  const hide = () => page.evaluate(() => {
    Object.defineProperty(document, "visibilityState", { configurable: true, get: () => "hidden" });
    document.dispatchEvent(new Event("visibilitychange"));
  });
  let before = await css();
  await deploy();
  await page.evaluate(() => { window.stillHere = true; });
  const reloaded = await Promise.all([page.waitForEvent("load", { timeout: 30000 }), lookForUpdate()]).then(() => true, () => false);
  await ran(page);
  check(reloaded && (await css()) !== before && !(await page.evaluate(() => window.stillHere)) && !(await page.isVisible("#app-update")),
    "update: found before anything is touched, the new version comes in by itself");
  check((await page.inputValue("#source")).includes("typing"), "update: … and the code is kept");

  // (e3) Touched, nothing being edited: the banner; then the app goes to the background
  // and the new version comes in (an installed app is rarely closed).
  before = await css();
  await deploy();
  await page.click("h1");
  await page.evaluate(() => { window.stillHere = true; });
  await lookForUpdate();
  await page.waitForSelector("#app-update:not([hidden])", { timeout: 30000 }).catch(() => null);
  check(await page.isVisible("#app-update") && await page.evaluate(() => window.stillHere === true), "update: once the page is touched, the banner and no reload");
  await Promise.all([page.waitForEvent("load", { timeout: 30000 }), hide()]).catch(() => null);
  await ran(page);
  check((await css()) !== before && !(await page.evaluate(() => window.stillHere)), "update: the app going to the background takes the new version");

  // (e4) A program being typed when the app goes to the background: nothing reloads, the
  // banner stays. The next launch (a new page) takes the new version by itself.
  before = await css();
  await deploy();
  await page.click("#source");
  await page.keyboard.type("# still typing\n");
  await page.evaluate(() => { window.stillHere = true; });
  await lookForUpdate();
  await page.waitForSelector("#app-update:not([hidden])", { timeout: 30000 }).catch(() => null);
  await hide();
  await page.waitForTimeout(2000);
  check(await page.evaluate(() => window.stillHere === true) && await page.isVisible("#app-update") && (await page.inputValue("#source")).includes("still typing"),
    "update: a field being edited when the app leaves: no reload, the banner stays");
  const launch2 = await context.newPage();
  await launch2.goto(`${base}/fr/playground/`);
  const waiting = async () => {
    for (let n = 0; n < 100; n++) {
      const left = await launch2.evaluate(async () => !!(await navigator.serviceWorker.getRegistration()).waiting).catch(() => true);
      if (!left) return false;
      await launch2.waitForTimeout(200);
    }
    return true;
  };
  const stillWaiting = await waiting();
  await launch2.waitForLoadState("load");
  await ran(launch2);
  check(!stillWaiting && (await launch2.getAttribute('link[href*="site.css"]', "href")) !== before && (await launch2.inputValue("#source")).includes("still typing"),
    "update: the next launch takes the waiting version by itself, with the code typed");
  check(await page.evaluate(() => window.stillHere === true) && (await page.inputValue("#source")).includes("still typing"), "update: … and the page being edited is left alone");
  await launch2.close();

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
