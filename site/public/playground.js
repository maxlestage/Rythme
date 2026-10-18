// The playground: the real Rythme interpreter, compiled to WebAssembly
// (public/rythme.wasm, built by ./build.sh). The page itself comes from the server,
// in the language of its address; this script makes it work.
(() => {
  "use strict";

  const page = document.querySelector(".playground");
  if (!page) return;
  const lang = page.dataset.lang === "fr" ? "fr" : "en";

  // ----- the texts that change while the page runs -----
  const UI = {
    en: {
      loading: "Loading the interpreter…", loadFailed: "The interpreter could not be loaded: ",
      running: "running…", stopped: "Stopped.", ranIn: (ms) => `ran in ${ms} ms`, failedIn: (ms) => `stopped with an error after ${ms} ms`,
      nothing: "The program printed nothing.",
      tooDeep: "The browser ran out of stack: too many nested calls. Make the function tail-recursive, or use fold, map or filter. The installed rythme allows several hundred thousand nested calls.",
      lines: (n) => `${n} line${n === 1 ? "" : "s"}`,
      explain: "explain", custom: "main.ry (edited)",
      formatted: "formatted like rythme fmt", alreadyFormatted: "already formatted", notFormatted: "not formatted: the code has an error",
    },
    fr: {
      loading: "Chargement de l'interpréteur…", loadFailed: "L'interpréteur n'a pas pu être chargé : ",
      running: "en cours…", stopped: "Arrêté.", ranIn: (ms) => `exécuté en ${ms} ms`, failedIn: (ms) => `arrêté par une erreur après ${ms} ms`,
      nothing: "Le programme n'a rien affiché.",
      tooDeep: "Le navigateur manque de pile : trop d'appels imbriqués. Rendez la fonction récursive terminale, ou utilisez fold, map ou filter. Le rythme installé permet plusieurs centaines de milliers d'appels imbriqués.",
      lines: (n) => `${n} ligne${n > 1 ? "s" : ""}`,
      explain: "expliquer", custom: "main.ry (modifié)",
      formatted: "mis en forme comme rythme fmt", alreadyFormatted: "déjà en forme", notFormatted: "pas mis en forme : le code a une erreur",
    },
  }[lang];

  const TOPICS = ["start", "syntax", "functions", "lists", "text", "records", "types", "match", "errors", "functional", "tests", "builtins"];
  const KEYWORDS = new Set(["fn", "if", "then", "else", "match", "end", "do", "type", "test", "use", "and", "or", "not", "true", "false"]);
  const BUILTINS = new Set(("print debug text input type_of help assert assert_eq fail args env exit time random abs min max sqrt pow floor ceil round int float sin cos tan asin acos atan log exp is_even is_odd gcd lcm len upper lower capitalize trim pad_left pad_right split join contains starts_with ends_with replace chars lines char_code char is_digit is_letter is_space repeat parse_int parse_float parse_json to_json map filter fold scan each sum product first last rest get push set_at insert_at remove_at reverse sort sort_by unique shuffle slice take drop take_while drop_while step_by chunks windows zip enumerate any all find count index_of min_by max_by flatten flat_map partition group_by is_empty keys values entries from_entries has put remove map_values unwrap unwrap_or is_some is_none is_ok is_err and_then ok_or map_err read_file write_file append_file file_exists pi").split(" "));

  const $ = (id) => document.getElementById(id);
  const source = $("source"), highlight = $("highlight"), gutter = $("gutter"), output = $("output");
  const statusEl = $("status"), runBtn = $("run"), stopBtn = $("stop"), checkBtn = $("check"), testBtn = $("test"), formatBtn = $("format");
  const exampleSel = $("example"), helpText = $("help-text");
  // The welcome program, written in the page by the server.
  const WELCOME = source.value;

  const store = {
    get(k) { try { return localStorage.getItem(k); } catch { return null; } },
    set(k, v) { try { localStorage.setItem(k, v); } catch { /* storage may be blocked */ } },
  };

  let examples = {};
  let currentTopic = "start";
  let wasmModule = null;
  let worker = null;
  let runId = 0;
  // What Run does: run the program, or its tests (testing.ry, or a link with mode=test).
  let runMode = 0;

  // ----- editor: highlighting, line numbers -----
  const escapeHtml = (s) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

  function colorize(code) {
    let out = "";
    let i = 0;
    const n = code.length;
    const span = (cls, text) => `<span class="${cls}">${escapeHtml(text)}</span>`;
    while (i < n) {
      const c = code[i];
      if (c === "#") {
        let j = code.indexOf("\n", i);
        if (j < 0) j = n;
        out += span("t-com", code.slice(i, j));
        i = j;
      } else if (c === '"') {
        let j = i + 1;
        while (j < n && code[j] !== '"') j += code[j] === "\\" ? 2 : 1;
        j = Math.min(j + 1, n);
        out += span("t-str", code.slice(i, j));
        i = j;
      } else if (/[0-9]/.test(c)) {
        let j = i + 1;
        while (j < n && (/[0-9a-fA-FxX_]/.test(code[j]) || (code[j] === "." && /[0-9]/.test(code[j + 1] || "")))) j++;
        out += span("t-num", code.slice(i, j));
        i = j;
      } else if (/[\p{L}_]/u.test(c)) {
        let j = i + 1;
        while (j < n && /[\p{L}\p{N}_]/u.test(code[j])) j++;
        const word = code.slice(i, j);
        if (KEYWORDS.has(word)) out += span("t-kw", word);
        else if (/^\p{Lu}/u.test(word)) out += span("t-ctor", word);
        else if (BUILTINS.has(word)) out += span("t-fn", word);
        else out += escapeHtml(word);
        i = j;
      } else if ((c === "|" || c === "-") && code[i + 1] === ">") {
        out += span("t-op", c + ">");
        i += 2;
      } else {
        out += escapeHtml(c);
        i += 1;
      }
    }
    // A final new line needs a character after it to be displayed.
    return out + "\n";
  }

  function refreshEditor() {
    const code = source.value;
    highlight.innerHTML = colorize(code);
    const count = code.split("\n").length;
    let nums = "";
    for (let k = 1; k <= count; k++) nums += k + "\n";
    gutter.textContent = nums;
    $("lines").textContent = UI.lines(count);
    syncScroll();
  }

  function syncScroll() {
    highlight.scrollTop = source.scrollTop;
    highlight.scrollLeft = source.scrollLeft;
    gutter.scrollTop = source.scrollTop;
  }

  function insertText(text) {
    const { selectionStart: s, selectionEnd: e, value } = source;
    source.value = value.slice(0, s) + text + value.slice(e);
    source.selectionStart = source.selectionEnd = s + text.length;
    onEdit();
  }

  function setCustom() {
    if (![...exampleSel.options].some((o) => o.value === "custom")) {
      const o = document.createElement("option");
      o.value = "custom";
      o.textContent = UI.custom;
      exampleSel.append(o);
    }
    exampleSel.value = "custom";
    $("file-name").textContent = "main.ry";
  }

  function onEdit() {
    refreshEditor();
    store.set("rythme-code", source.value);
    setCustom();
  }

  source.addEventListener("input", onEdit);
  source.addEventListener("scroll", syncScroll);
  source.addEventListener("keydown", (ev) => {
    if ((ev.ctrlKey || ev.metaKey) && ev.key === "Enter") {
      ev.preventDefault();
      execute(ev.shiftKey ? 2 : 0);
    } else if (ev.shiftKey && ev.altKey && ev.code === "KeyF") {
      ev.preventDefault();
      formatCode();
    } else if (ev.key === "Tab" && !ev.shiftKey && !ev.altKey) {
      ev.preventDefault();
      insertText("  ");
    } else if (ev.key === "Enter" && !ev.ctrlKey && !ev.metaKey) {
      // Keep the indentation, and indent after a line that opens a block.
      const before = source.value.slice(0, source.selectionStart);
      const line = before.slice(before.lastIndexOf("\n") + 1);
      let indent = line.match(/^\s*/)[0];
      if (/^\s*(fn\s+\S+\s*\(.*\)(\s+if\s+.*)?|test\s+".*")\s*$|\b(do|match\s+.+)\s*$|=\s*$/.test(line)) indent += "  ";
      ev.preventDefault();
      insertText("\n" + indent);
    }
  });

  // ----- the interpreter -----
  const imports = { env: { rythme_now: () => Date.now() } };
  function callInterpreter(instance, code, mode) {
    const e = instance.exports;
    const bytes = new TextEncoder().encode(code);
    const ptr = e.rythme_alloc(bytes.length);
    new Uint8Array(e.memory.buffer, ptr, bytes.length).set(bytes);
    const at = e.rythme_run(ptr, bytes.length, mode, lang === "fr" ? 1 : 0);
    return new TextDecoder().decode(new Uint8Array(e.memory.buffer, at, e.rythme_result_len()));
  }

  function newWorker() {
    if (worker) worker.terminate();
    worker = new Worker(page.dataset.worker);
    worker.postMessage({ module: wasmModule });
    worker.onmessage = (msg) => finish(msg.data);
  }

  function setRunning(on) {
    document.body.classList.toggle("running", on);
    runBtn.hidden = on;
    stopBtn.hidden = !on;
    checkBtn.disabled = on;
    testBtn.disabled = on;
  }

  function execute(mode) {
    if (!wasmModule) return;
    runId += 1;
    setRunning(true);
    statusEl.className = "status";
    statusEl.textContent = UI.running;
    worker.postMessage({ id: runId, code: source.value, mode, lang });
  }

  function finish(result) {
    if (result.id !== runId) return;
    setRunning(false);
    const ms = Math.max(1, Math.round(result.ms));
    if (result.trap) {
      renderOutput("\u0001" + UI.tooDeep + (/stack/i.test(result.trap) ? "" : "\n" + result.trap) + "\u0002");
      statusEl.className = "status bad";
      statusEl.textContent = UI.failedIn(ms);
      newWorker();
      return;
    }
    renderOutput(result.text);
    const failed = /\u0001(error|erreur)\[/.test(result.text) || /✗/.test(result.text);
    statusEl.className = failed ? "status bad" : "status good";
    statusEl.textContent = failed ? UI.failedIn(ms) : UI.ranIn(ms);
    // On a phone the output is under the editor: bring it into view.
    if (window.matchMedia("(max-width: 959px)").matches && document.activeElement !== source) {
      output.scrollIntoView({ behavior: "smooth", block: "nearest" });
    }
  }

  stopBtn.addEventListener("click", () => {
    runId += 1;
    newWorker();
    setRunning(false);
    statusEl.className = "status bad";
    statusEl.textContent = UI.stopped;
  });
  runBtn.addEventListener("click", () => execute(runMode));
  checkBtn.addEventListener("click", () => execute(1));
  testBtn.addEventListener("click", () => execute(2));
  formatBtn.addEventListener("click", formatCode);

  // ----- format: the layout of `rythme fmt`, done at once (it cannot loop) -----
  let formatInstance = null;
  async function formatCode() {
    if (!wasmModule) return;
    const code = source.value;
    let result;
    try {
      formatInstance = formatInstance || await WebAssembly.instantiate(wasmModule, imports);
      result = callInterpreter(formatInstance, code, 3);
    } catch (err) {
      formatInstance = null;
      result = "\u0001" + err + "\u0002";
    }
    statusEl.className = "status good";
    if (result.startsWith("\u0001")) {
      renderOutput(result);
      statusEl.className = "status bad";
      statusEl.textContent = UI.notFormatted;
      return;
    }
    if (result === code) {
      statusEl.textContent = UI.alreadyFormatted;
      return;
    }
    // Keep the cursor on the same line, and the view where it was.
    const line = code.slice(0, source.selectionStart).split("\n").length;
    const top = source.scrollTop;
    // With a mouse, through the editing commands so that Ctrl+Z undoes it;
    // on a phone, without opening the keyboard.
    const desktop = window.matchMedia("(hover: hover) and (pointer: fine)").matches;
    if (desktop) {
      source.focus();
      source.select();
    }
    if (!desktop || !document.execCommand("insertText", false, result)) source.value = result;
    const at = result.split("\n").slice(0, line - 1).join("\n").length + (line > 1 ? 1 : 0);
    source.selectionStart = source.selectionEnd = Math.min(at, result.length);
    source.scrollTop = top;
    onEdit();
    statusEl.textContent = UI.formatted;
  }

  // ----- output: messages styled like in a terminal -----
  function lineClass(line) {
    if (/^(error|erreur)(\[|:)/.test(line)) return "err";
    if (/^(warning|avertissement)(\[|:)/.test(line)) return "warn";
    if (/^\s*=\s*(help|aide)/.test(line)) return "help";
    if (/^\s*=\s*/.test(line)) return "note";
    if (/^\s*-->/.test(line)) return "note";
    return null;
  }

  function text(s, cls) {
    if (!cls) return document.createTextNode(s);
    const el = document.createElement("span");
    el.className = cls;
    el.textContent = s;
    return el;
  }

  function renderLine(line, inError, severity) {
    const frag = document.createDocumentFragment();
    let cls = inError ? lineClass(line) : null;
    if (!inError && /^\s*✓/.test(line)) cls = "pass";
    if (!inError && /^\s*✗/.test(line)) cls = "fail";
    // Underline marks: `  |    ^^^^ label`
    const marks = inError && line.match(/^(\s*\|\s*)([\s]*)([\^-]+)(.*)$/);
    if (marks) {
      frag.append(text(marks[1] + marks[2], "note"));
      frag.append(text(marks[3] + marks[4], marks[3][0] === "^" ? (severity === "warn" ? "mark-warn" : "mark-err") : "note"));
      return frag;
    }
    const explain = line.match(/^(.*`)(rythme explain )([EW]\d{3})(`.*)$/);
    if (explain) {
      frag.append(text(explain[1] + explain[2], "note"));
      const b = document.createElement("button");
      b.type = "button";
      b.className = "explain";
      b.textContent = explain[3];
      b.title = UI.explain + " " + explain[3];
      b.addEventListener("click", () => showHelp(explain[3], true));
      frag.append(b, text(explain[4], "note"));
      return frag;
    }
    frag.append(text(line, cls || (inError ? "src" : null)));
    return frag;
  }

  function renderOutput(raw) {
    output.textContent = "";
    if (!raw.replace(/[\u0001\u0002\s]/g, "")) {
      output.append(text(UI.nothing, "empty"));
      return;
    }
    for (const part of raw.split(/(\u0001[\s\S]*?\u0002)/)) {
      if (!part) continue;
      const inError = part.startsWith("\u0001");
      const body = inError ? part.slice(1, -1) : part;
      const severity = /^(warning|avertissement)/.test(body) ? "warn" : "err";
      const lines = body.replace(/\n+$/, "").split("\n");
      lines.forEach((line, k) => {
        output.append(renderLine(line, inError, severity));
        if (k < lines.length - 1 || inError) output.append("\n");
      });
      if (!inError && body.endsWith("\n")) output.append("\n");
    }
  }

  // ----- help, from the interpreter itself -----
  let helpInstance = null;
  async function showHelp(name, scroll) {
    if (!wasmModule) return;
    currentTopic = name;
    for (const b of document.querySelectorAll(".topic")) b.setAttribute("aria-pressed", String(b.dataset.topic === name));
    try {
      helpInstance = helpInstance || await WebAssembly.instantiate(wasmModule, imports);
      const literal = '"' + name.replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\{/g, "\\{").replace(/\n/g, " ") + '"';
      const raw = callInterpreter(helpInstance, `help(${literal})`, 0);
      helpText.textContent = raw.replace(/[\u0001\u0002]/g, "").trim();
    } catch (err) {
      helpInstance = null;
      helpText.textContent = String(err);
    }
    if (scroll) helpText.scrollIntoView({ behavior: "smooth", block: "start" });
  }

  const topicsEl = $("topics");
  for (const t of TOPICS) {
    const b = document.createElement("button");
    b.type = "button";
    b.className = "topic";
    b.dataset.topic = t;
    b.textContent = t;
    b.setAttribute("aria-pressed", "false");
    b.addEventListener("click", () => showHelp(t, false));
    topicsEl.append(b);
  }
  $("search-form").addEventListener("submit", (ev) => {
    ev.preventDefault();
    const q = $("search").value.trim();
    if (q) showHelp(q, false);
  });

  // ----- examples -----
  function loadExample(name) {
    source.value = name === "welcome" ? WELCOME : examples[name] || "";
    exampleSel.value = name;
    $("file-name").textContent = name === "welcome" ? "main.ry" : name + ".ry";
    refreshEditor();
    store.set("rythme-code", "");
    runMode = name === "testing" ? 2 : 0;
    execute(runMode);
  }

  exampleSel.addEventListener("change", () => {
    if (exampleSel.value !== "custom") loadExample(exampleSel.value);
  });

  // `#example=fizzbuzz`, or `#code=…&mode=test`: links from the other pages.
  function fromAddress() {
    const params = new URLSearchParams(location.hash.slice(1));
    const found = { example: params.get("example"), code: params.get("code"), mode: params.get("mode") === "test" ? 2 : 0 };
    if (found.example || found.code !== null) history.replaceState(null, "", location.pathname + location.search);
    return found;
  }

  // ----- start -----
  async function start() {
    const asked = fromAddress();
    const saved = store.get("rythme-code");
    if (asked.code !== null) {
      source.value = asked.code;
      runMode = asked.mode;
      store.set("rythme-code", asked.code);
      setCustom();
    } else if (saved && !asked.example) {
      source.value = saved;
      setCustom();
    }
    refreshEditor();
    output.textContent = "";
    output.append(text(UI.loading, "empty"));
    try {
      const url = page.dataset.wasm;
      try {
        wasmModule = await WebAssembly.compileStreaming(fetch(url));
      } catch {
        const response = await fetch(url);
        if (!response.ok) throw new Error(`${url}: ${response.status}`);
        wasmModule = await WebAssembly.compile(await response.arrayBuffer());
      }
    } catch (err) {
      page.classList.add("no-wasm");
      output.textContent = "";
      output.append(text(UI.loadFailed + err, "err"));
      return;
    }
    for (const b of [runBtn, checkBtn, testBtn, formatBtn]) b.disabled = false;
    newWorker();
    try {
      examples = await (await fetch(page.dataset.examples)).json();
    } catch {
      examples = {};
    }
    if (asked.example && (asked.example === "welcome" || examples[asked.example])) {
      loadExample(asked.example);
    } else {
      execute(runMode);
    }
    showHelp(currentTopic, false);
  }

  start();
})();
