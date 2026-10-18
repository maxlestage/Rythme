// The playground: the real Rythme interpreter, compiled to WebAssembly
// (public/rythme.wasm, built by ./build.sh). The page itself comes from the server,
// in the language of its address; this script makes it work. The code being edited
// stays in localStorage; a .ry file can be opened, from the button, from the system
// (the installed app's file handler) or shared as text, and downloaded. Share puts
// the program in a link (`#code=…`, in the link itself: nothing is sent anywhere).
// Steps shows the calculation of one expression step by step, one step at a time.
(() => {
  "use strict";

  const page = document.querySelector(".playground");
  if (!page) return;
  // The language of the page, which is also the language of the interpreter's messages.
  const lang = page.dataset.lang || "en";

  // ----- the texts that change while the page runs -----
  // Written by the server in the language of the page (`data-ui`, from src/i18n/).
  const TEXTS = JSON.parse(page.dataset.ui);
  const UI = {
    ...TEXTS,
    ranIn: (ms) => TEXTS.ranIn.replace("{ms}", ms),
    failedIn: (ms) => TEXTS.failedIn.replace("{ms}", ms),
    lines: (n) => (n === 1 ? TEXTS.line : TEXTS.lines).replace("{n}", n),
    stepsOf: (n, total) => TEXTS.stepsOf.replace("{n}", n).replace("{total}", total),
  };
  // The words that start the interpreter's messages, in every language
  // (rythme::lang::LANGUAGES; the tests of the site check that none is missing).
  const ERROR = "error|erreur|Fehler|erro|错误";
  const WARNING = "warning|avertissement|advertencia|Warnung|aviso|警告";
  const HELP = "help|aide|ayuda|Hilfe|ajuda|帮助";
  const FAILED = new RegExp(`\\u0001(${ERROR})\\[`);
  const ERROR_LINE = new RegExp(`^(${ERROR})(\\[|:)`);
  const WARNING_LINE = new RegExp(`^(${WARNING})(\\[|:)`);
  const HELP_LINE = new RegExp(`^\\s*=\\s*(${HELP})`);

  const TOPICS = ["start", "syntax", "functions", "lists", "text", "records", "types", "match", "errors", "functional", "tests", "builtins"];
  const KEYWORDS = new Set(["fn", "if", "then", "else", "match", "end", "do", "type", "test", "use", "and", "or", "not", "true", "false", "unless"]);
  const BUILTINS = new Set(("print debug text input input_lines type_of help assert assert_eq fail args options env exit time random abs min max sqrt pow floor ceil round int float sin cos tan asin acos atan log exp is_even is_odd gcd lcm len upper lower capitalize trim pad_left pad_right split join contains starts_with ends_with replace chars lines char_code char is_digit is_letter is_space repeat parse_int parse_float parse_json to_json trim_start trim_end words equals_ignore_case format_number matches find_first find_all captures captures_all named_captures split_by replace_all map filter fold scan each sum product first last rest get push set_at insert_at remove_at reverse sort sort_by unique shuffle slice take drop take_while drop_while step_by chunks windows zip enumerate any all find count reject each_with_index index_of min_by max_by flatten flat_map partition group_by is_empty keys values entries from_entries has put remove map_values dict set union intersection difference is_subset unwrap unwrap_or is_some is_none is_ok is_err and_then ok_or map_err read_file write_file append_file file_exists file_info list_folder create_folder delete_file join_path split_path now today parse_date format_date is_date add_days add_seconds days_between seconds_between day_of_week days_in_month is_leap_year check_all sample ints floats texts bools lists_of one_of records_of pi").split(" "));

  const $ = (id) => document.getElementById(id);
  const source = $("source"), highlight = $("highlight"), gutter = $("gutter"), output = $("output");
  const statusEl = $("status"), runBtn = $("run"), stopBtn = $("stop"), checkBtn = $("check"), testBtn = $("test"), formatBtn = $("format");
  const stepsBtn = $("steps");
  const exampleSel = $("example"), helpText = $("help-text");
  // The welcome program, written in the page by the server.
  const WELCOME = source.value;

  const store = {
    get(k) { try { return localStorage.getItem(k); } catch { return null; } },
    set(k, v) { try { localStorage.setItem(k, v); } catch { /* storage may be blocked */ } },
  };

  // Keys of localStorage: the code being edited ("" for an example as it is), the
  // example shown, and the name of the opened file.
  const SAVED = { code: "rythme-code", example: "rythme-example", file: "rythme-file" };

  let examples = {};
  // The name of the edited code: main.ry, or the name of the opened file.
  let fileName = "main.ry";
  let currentTopic = "start";
  let wasmModule = null;
  let worker = null;
  let runId = 0;
  // What Run does: run the program, or its tests (testing.ry, or a link with mode=test).
  let runMode = 0;
  // What the interpreter is doing: one of the modes of `rythme_run` (5: steps).
  let runningMode = 0;

  // ----- editor: highlighting, line numbers -----
  const escapeHtml = (s) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

  function colorize(code) {
    let out = "";
    let i = 0;
    const n = code.length;
    // A piece that spans lines (a text on several lines) is closed at each new line:
    // every line of the result stands alone.
    const span = (cls, text) => text.split("\n").map((part) => `<span class="${cls}">${escapeHtml(part)}</span>`).join("\n");
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
    // One block per line, which wraps like the same line in the field; its new line
    // gives an empty line its height.
    return out.split("\n").map((line) => `<span>${line}\n</span>`).join("");
  }

  // Each line number takes the height of its line, wrapped or not.
  function numberLines() {
    const lines = highlight.children;
    const numbers = document.createDocumentFragment();
    for (let k = 0; k < lines.length; k++) {
      const number = document.createElement("div");
      number.textContent = String(k + 1);
      number.style.height = `${lines[k].getBoundingClientRect().height}px`;
      numbers.append(number);
    }
    gutter.replaceChildren(numbers);
    syncScroll();
  }

  function refreshEditor() {
    const code = source.value;
    highlight.innerHTML = colorize(code);
    $("lines").textContent = UI.lines(code.split("\n").length);
    numberLines();
  }
  // The lines wrap again when the editor changes width, or when its font arrives.
  if ("ResizeObserver" in window) {
    let width = 0;
    new ResizeObserver(() => {
      if (highlight.clientWidth !== width) {
        width = highlight.clientWidth;
        numberLines();
      }
    }).observe(highlight);
  }
  if (document.fonts) document.fonts.ready.then(numberLines);

  function syncScroll() {
    highlight.scrollTop = source.scrollTop;
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
    $("file-name").textContent = fileName;
  }

  function onEdit() {
    leaveLink();
    if (!shareBox.hidden) {
      shareBox.hidden = true;
      shareBtn.setAttribute("aria-expanded", "false");
    }
    refreshEditor();
    store.set(SAVED.code, source.value);
    setCustom();
  }

  // ----- .ry files: open, download -----
  function openText(code, name) {
    source.value = code;
    fileName = /^[^/\\]{1,80}\.ry$/i.test(name || "") ? name : "main.ry";
    store.set(SAVED.file, fileName);
    onEdit();
    runMode = 0;
    execute(runMode);
  }

  async function openFile(file) {
    try {
      openText(await file.text(), file.name);
    } catch (err) {
      renderOutput("\u0001" + err + "\u0002");
    }
  }

  const openBtn = $("open"), saveBtn = $("save"), fileInput = $("open-file");
  openBtn.hidden = false;
  saveBtn.hidden = false;
  openBtn.addEventListener("click", () => fileInput.click());
  fileInput.addEventListener("change", () => {
    if (fileInput.files[0]) openFile(fileInput.files[0]);
    fileInput.value = "";
  });
  saveBtn.addEventListener("click", () => {
    const name = ($("file-name").textContent || "main.ry").replace(/[^\w.-]+/g, "_");
    const url = URL.createObjectURL(new Blob([source.value], { type: "text/plain;charset=utf-8" }));
    const link = document.createElement("a");
    link.href = url;
    link.download = /\.ry$/.test(name) ? name : "main.ry";
    document.body.append(link);
    link.click();
    link.remove();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  });
  // The last keystrokes, if the app is closed before the storage was written.
  window.addEventListener("pagehide", () => {
    if (exampleSel.value === "custom" && !linked) store.set(SAVED.code, source.value);
  });

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
  const imports = { env: { rythme_now: () => Date.now(), rythme_offset: (ms) => -new Date(ms).getTimezoneOffset() } };
  function callInterpreter(instance, code, mode) {
    const e = instance.exports;
    const bytes = new TextEncoder().encode(code);
    const ptr = e.rythme_alloc(bytes.length);
    new Uint8Array(e.memory.buffer, ptr, bytes.length).set(bytes);
    // The language of the messages: its position in rythme::lang::LANGUAGES.
    const at = e.rythme_run(ptr, bytes.length, mode, Math.max(0, ["en", "fr", "es", "de", "pt", "zh"].indexOf(lang)));
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
    stepsBtn.disabled = on;
  }

  function execute(mode) {
    if (!wasmModule) return;
    runId += 1;
    runningMode = mode;
    setRunning(true);
    statusEl.className = "status";
    statusEl.textContent = UI.running;
    // Steps: the program, U+0000, then the expression to compute ("": the last one).
    const code = mode === 5 ? source.value + "\u0000" + stepsExpr : source.value;
    worker.postMessage({ id: runId, code, mode, lang });
  }

  function finish(result) {
    if (result.id !== runId) return;
    setRunning(false);
    const ms = Math.max(1, Math.round(result.ms));
    const stepping = runningMode === 5 && !result.trap;
    showStepper(stepping);
    if (result.trap) {
      renderOutput("\u0001" + UI.tooDeep + (/stack/i.test(result.trap) ? "" : "\n" + result.trap) + "\u0002");
      statusEl.className = "status bad";
      statusEl.textContent = UI.failedIn(ms);
      newWorker();
      return;
    }
    const failed = stepping ? renderSteps(result.text) : (renderOutput(result.text), FAILED.test(result.text) || /✗/.test(result.text));
    statusEl.className = failed ? "status bad" : "status good";
    statusEl.textContent = failed ? UI.failedIn(ms) : UI.ranIn(ms);
    // On a phone the output is under the editor: bring it into view.
    if (window.matchMedia("(max-width: 959px)").matches && document.activeElement !== source) {
      (stepping ? stepper : output).scrollIntoView({ behavior: "smooth", block: "nearest" });
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
  // The calculation of the selected code, or of the last expression, step by step.
  stepsBtn.addEventListener("click", () => {
    const selected = source.value.slice(source.selectionStart, source.selectionEnd).trim();
    showSteps(selected === source.value.trim() ? "" : selected, 1);
  });
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
    if (ERROR_LINE.test(line)) return "err";
    if (WARNING_LINE.test(line)) return "warn";
    if (HELP_LINE.test(line)) return "help";
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

  function renderOutput(raw, into) {
    const out = into || output;
    out.textContent = "";
    if (!raw.replace(/[\u0001\u0002\s]/g, "")) {
      out.append(text(UI.nothing, "empty"));
      return;
    }
    for (const part of raw.split(/(\u0001[\s\S]*?\u0002)/)) {
      if (!part) continue;
      const inError = part.startsWith("\u0001");
      const body = inError ? part.slice(1, -1) : part;
      const severity = WARNING_LINE.test(body) ? "warn" : "err";
      const lines = body.replace(/\n+$/, "").split("\n");
      lines.forEach((line, k) => {
        out.append(renderLine(line, inError, severity));
        if (k < lines.length - 1 || inError) out.append("\n");
      });
      if (!inError && body.endsWith("\n")) out.append("\n");
    }
  }

  // ----- steps: the calculation of one expression, one step at a time -----
  // The interpreter writes the steps like `rythme steps`: the expression, then each
  // step after "  → " (its other lines after four spaces). The other lines are what
  // the program printed meanwhile, shown with the step that follows them, and
  // "  ... (…)" when steps were left out. U+0005: no expression to show.
  const stepper = $("stepper"), stepNav = stepper.querySelector(".stepper__nav"), stepKeys = stepper.querySelector(".stepper__keys");
  const stepPrev = $("step-prev"), stepNext = $("step-next"), stepCount = $("step-count"), stepRange = $("step-range");
  const stepFrom = $("step-from"), stepNow = $("step-now"), stepPrinted = $("step-printed"), stepPrint = $("step-print");
  const stepError = $("step-error"), stepLive = $("step-live");
  // The code whose steps are shown ("": the last expression of the program), the
  // steps, the one shown (from 0), and the one to show first (from 1, from a link).
  let stepsExpr = "";
  let steps = [];
  let stepAt = 0;
  let firstStep = 1;
  let stepsFailed = "";

  function showSteps(expr, step) {
    stepsExpr = expr;
    firstStep = step;
    execute(5);
  }

  function showStepper(on) {
    stepper.hidden = !on;
    output.hidden = on;
  }

  const stepsShown = () => !stepper.hidden && steps.length > 0;

  function parseSteps(body) {
    const list = [];
    let pending = [];
    if (!body.trim()) return list;
    for (const line of body.replace(/\n+$/, "").split("\n")) {
      const current = list[list.length - 1];
      if (line.startsWith("  → ")) {
        list.push({ lines: [line.slice(4)], printed: pending.filter((l) => !/^ {2}\.\.\. \(/.test(l)), gap: pending.some((l) => /^ {2}\.\.\. \(/.test(l)) });
        pending = [];
      } else if (!current) {
        list.push({ lines: [line], printed: [], gap: false });
      } else if (!pending.length && list.length > 1 && line.startsWith("    ")) {
        current.lines.push(line.slice(4));
      } else if (!pending.length && list.length === 1 && /^(\s|end\b)/.test(line) && !/^ {2}\.\.\. \(/.test(line)) {
        current.lines.push(line);
      } else {
        pending.push(line);
      }
    }
    if (pending.length && list.length) list[list.length - 1].printed.push(...pending.filter((l) => !/^ {2}\.\.\. \(/.test(l)));
    return list;
  }

  // Returns whether the calculation stopped with an error.
  function renderSteps(raw) {
    let body = "";
    stepsFailed = "";
    for (const part of raw.split(/(\u0001[\s\S]*?\u0002)/)) {
      if (part.startsWith("\u0001")) {
        // The warnings are for Check; the errors are shown after the last step.
        if (!WARNING_LINE.test(part.slice(1))) stepsFailed += part;
      } else {
        body += part;
      }
    }
    const none = body.includes("\u0005");
    steps = none ? [] : parseSteps(body);
    for (const el of [stepNav, stepRange, stepKeys]) el.hidden = !steps.length;
    if (steps.length) {
      showStep(firstStep - 1);
    } else {
      stepFrom.textContent = "";
      stepNow.textContent = "";
      stepNow.append(text(none ? UI.stepsNone : UI.nothing, "empty"));
      stepNow.hidden = !none && !!stepsFailed;
      stepPrinted.hidden = true;
      stepError.hidden = !stepsFailed;
      if (stepsFailed) renderOutput(stepsFailed, stepError);
      stepLive.textContent = "";
    }
    firstStep = 1;
    return !!stepsFailed;
  }

  // A step, colored like the editor; each line wraps under its indentation.
  function showTerm(el, lines) {
    el.innerHTML = colorize(lines.join("\n"));
    for (const line of el.children) {
      line.className = "line";
      const indent = line.textContent.match(/^ */)[0].length;
      if (indent) line.style.setProperty("--indent", indent);
    }
  }

  function showStep(i) {
    const n = steps.length;
    stepAt = Math.max(0, Math.min(Number.isFinite(i) ? i : 0, n - 1));
    const step = steps[stepAt];
    const count = UI.stepsOf(stepAt + 1, n);
    stepCount.textContent = count;
    stepRange.max = String(n);
    stepRange.value = String(stepAt + 1);
    stepRange.setAttribute("aria-valuetext", count);
    // A button that gets disabled loses the focus: it goes to the slider.
    const focused = document.activeElement;
    stepPrev.disabled = stepAt === 0;
    stepNext.disabled = stepAt === n - 1;
    if (focused && focused.disabled) stepRange.focus();
    if (stepAt === 0) stepFrom.textContent = "";
    else if (step.gap) stepFrom.textContent = "…";
    else showTerm(stepFrom, steps[stepAt - 1].lines);
    stepNow.hidden = false;
    showTerm(stepNow, step.lines);
    stepPrinted.hidden = !step.printed.length;
    stepPrint.textContent = step.printed.join("\n");
    const last = stepAt === n - 1 && !!stepsFailed;
    stepError.hidden = !last;
    if (last) renderOutput(stepsFailed, stepError);
    stepLive.textContent = `${count}: ${step.lines.join(" ").replace(/\s+/g, " ").trim().slice(0, 300)}`;
  }

  stepPrev.addEventListener("click", () => showStep(stepAt - 1));
  stepNext.addEventListener("click", () => showStep(stepAt + 1));
  stepRange.addEventListener("input", () => showStep(Number(stepRange.value) - 1));
  // The keyboard, anywhere in the steps (the slider has these keys of its own).
  stepper.addEventListener("keydown", (ev) => {
    if (ev.target === stepRange || ev.ctrlKey || ev.metaKey || ev.altKey || !steps.length) return;
    const to = { ArrowLeft: stepAt - 1, ArrowRight: stepAt + 1, Home: 0, End: steps.length - 1 }[ev.key];
    if (to === undefined) return;
    ev.preventDefault();
    showStep(to);
  });

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
    leaveLink();
    source.value = name === "welcome" ? WELCOME : examples[name] || "";
    exampleSel.value = name;
    $("file-name").textContent = name === "welcome" ? "main.ry" : name + ".ry";
    fileName = "main.ry";
    refreshEditor();
    store.set(SAVED.code, "");
    store.set(SAVED.example, name);
    store.set(SAVED.file, fileName);
    runMode = name === "testing" ? 2 : 0;
    execute(runMode);
  }

  exampleSel.addEventListener("change", () => {
    if (exampleSel.value !== "custom") loadExample(exampleSel.value);
  });

  // ----- sharing: the program in the link itself -----
  // `#code=1:…` is the program compressed (deflate-raw), `#code=0:…` as it is (UTF-8),
  // both in URL-safe base64 (Share takes the shorter); `#code=` followed by anything
  // else is the program percent-encoded (the links of the other pages). A fragment
  // never leaves the browser: nothing is sent anywhere, and the links work offline and
  // on any host, the static export too.
  const LINK_MAX = 8000; // characters of a shared program, packed
  const CODE_MAX = 200000; // bytes of a program opened from a link
  const PACKED = /^([01]):([A-Za-z0-9_-]*)$/;

  function toBase64(bytes) {
    let s = "";
    for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
    return btoa(s).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
  }

  function fromBase64(text) {
    const s = atob(text.replace(/-/g, "+").replace(/_/g, "/"));
    const bytes = new Uint8Array(s.length);
    for (let i = 0; i < s.length; i++) bytes[i] = s.charCodeAt(i);
    return bytes;
  }

  // A compression stream of the browser, if it knows deflate-raw.
  function browserStream(Kind) {
    try {
      return typeof Kind === "function" ? new Kind("deflate-raw") : null;
    } catch {
      return null;
    }
  }

  async function through(bytes, stream) {
    const reader = new Blob([bytes]).stream().pipeThrough(stream).getReader();
    const chunks = [];
    let size = 0;
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      size += value.length;
      if (size > CODE_MAX) {
        reader.cancel();
        throw new Error("too long");
      }
      chunks.push(value);
    }
    const out = new Uint8Array(size);
    let at = 0;
    for (const chunk of chunks) {
      out.set(chunk, at);
      at += chunk.length;
    }
    return out;
  }

  // Inflate (RFC 1951), for the browsers without DecompressionStream: stored, fixed and
  // dynamic blocks, at most `max` bytes out. Throws on a damaged stream.
  const LEN_BASE = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
  const LEN_EXTRA = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
  const DIST_BASE = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
  const DIST_EXTRA = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
  const ORDER = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

  function inflate(data, max) {
    let pos = 0;
    let bit = 0;
    let out = new Uint8Array(4096);
    let size = 0;
    const damaged = () => new Error("damaged");
    const bits = (n) => {
      let v = 0;
      for (let i = 0; i < n; i++) {
        if (pos >= data.length) throw damaged();
        v |= ((data[pos] >> bit) & 1) << i;
        if (++bit === 8) {
          bit = 0;
          pos++;
        }
      }
      return v;
    };
    const put = (b) => {
      if (size === out.length) {
        if (size >= max) throw new Error("too long");
        const bigger = new Uint8Array(Math.min(max, size * 2));
        bigger.set(out);
        out = bigger;
      }
      out[size++] = b;
    };
    // A canonical Huffman code from its lengths: how many codes of each length, and
    // the symbols in the order of their codes.
    const table = (lengths) => {
      const counts = new Uint16Array(16);
      const offsets = new Uint16Array(16);
      for (const l of lengths) counts[l]++;
      counts[0] = 0;
      for (let l = 1; l < 16; l++) offsets[l] = offsets[l - 1] + counts[l - 1];
      const symbols = new Uint16Array(lengths.length);
      lengths.forEach((l, s) => { if (l) symbols[offsets[l]++] = s; });
      return { counts, symbols };
    };
    const decode = (h) => {
      let code = 0;
      let first = 0;
      let index = 0;
      for (let l = 1; l < 16; l++) {
        code |= bits(1);
        const count = h.counts[l];
        if (code - count < first) return h.symbols[index + (code - first)];
        index += count;
        first = (first + count) << 1;
        code <<= 1;
      }
      throw damaged();
    };
    let fixed = null;
    let last = 0;
    while (!last) {
      last = bits(1);
      const type = bits(2);
      if (type === 0) {
        if (bit) {
          bit = 0;
          pos++;
        }
        if (pos + 4 > data.length) throw damaged();
        const len = data[pos] | (data[pos + 1] << 8);
        if ((len ^ 0xffff) !== (data[pos + 2] | (data[pos + 3] << 8))) throw damaged();
        pos += 4;
        if (pos + len > data.length) throw damaged();
        for (let i = 0; i < len; i++) put(data[pos + i]);
        pos += len;
        continue;
      }
      let lit;
      let dist;
      if (type === 1) {
        if (!fixed) {
          const lengths = new Array(288);
          for (let i = 0; i < 288; i++) lengths[i] = i < 144 ? 8 : i < 256 ? 9 : i < 280 ? 7 : 8;
          fixed = { lit: table(lengths), dist: table(new Array(30).fill(5)) };
        }
        ({ lit, dist } = fixed);
      } else if (type === 2) {
        const nlit = bits(5) + 257;
        const ndist = bits(5) + 1;
        const ncode = bits(4) + 4;
        const codeLengths = new Array(19).fill(0);
        for (let i = 0; i < ncode; i++) codeLengths[ORDER[i]] = bits(3);
        const lengthCode = table(codeLengths);
        const lengths = [];
        while (lengths.length < nlit + ndist) {
          const sym = decode(lengthCode);
          if (sym < 16) lengths.push(sym);
          else {
            if (sym === 16 && !lengths.length) throw damaged();
            const [value, n] = sym === 16 ? [lengths[lengths.length - 1], 3 + bits(2)] : sym === 17 ? [0, 3 + bits(3)] : [0, 11 + bits(7)];
            if (lengths.length + n > nlit + ndist) throw damaged();
            for (let i = 0; i < n; i++) lengths.push(value);
          }
        }
        lit = table(lengths.slice(0, nlit));
        dist = table(lengths.slice(nlit));
      } else {
        throw damaged();
      }
      for (;;) {
        let sym = decode(lit);
        if (sym < 256) put(sym);
        else if (sym === 256) break;
        else {
          sym -= 257;
          if (sym >= 29) throw damaged();
          const len = LEN_BASE[sym] + bits(LEN_EXTRA[sym]);
          const d = decode(dist);
          if (d >= 30) throw damaged();
          const back = DIST_BASE[d] + bits(DIST_EXTRA[d]);
          if (back > size) throw damaged();
          for (let i = 0; i < len; i++) put(out[size - back]);
        }
      }
    }
    return out.subarray(0, size);
  }

  // The program, packed for a link: compressed when the browser can and it is shorter.
  async function pack(code) {
    const bytes = new TextEncoder().encode(code);
    const plain = "0:" + toBase64(bytes);
    const deflate = browserStream(window.CompressionStream);
    if (!deflate || bytes.length > CODE_MAX) return plain;
    try {
      const packed = "1:" + toBase64(await through(bytes, deflate));
      return packed.length < plain.length ? packed : plain;
    } catch {
      return plain;
    }
  }

  async function unpack(value) {
    const [, kind, data] = value.match(PACKED);
    let bytes = fromBase64(data);
    if (kind === "1") {
      const native = browserStream(window.DecompressionStream);
      bytes = native ? await through(bytes, native) : inflate(bytes, CODE_MAX);
    }
    if (bytes.length > CODE_MAX) throw new Error("too long");
    return new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  }

  // `#example=fizzbuzz`, `#code=…&mode=test`, or `#code=…&mode=steps&step=3` (with
  // `&expr=…`, percent-encoded, for the steps of a piece of code rather than of the last
  // expression): links from the other pages and shared
  // links; `?text=…&title=…&url=…`: a text shared with the installed app (its share
  // target), which becomes a `#code=` link. A program from a link stays in the address
  // until it is changed, so that a reload shows it again.
  async function fromAddress() {
    const hash = location.hash.slice(1);
    const params = new URLSearchParams(hash);
    const found = { example: params.get("example"), code: null, mode: params.get("mode") === "test" ? 2 : 0, broken: false };
    found.steps = params.get("mode") === "steps";
    found.expr = (params.get("expr") || "").slice(0, LINK_MAX);
    found.step = Math.max(1, Math.min(parseInt(params.get("step"), 10) || 1, 100000));
    const raw = (hash.match(/(?:^|&)code=([^&]*)/) || [])[1];
    if (raw !== undefined) {
      try {
        if (raw.length > 4 * LINK_MAX) throw new Error("too long");
        found.code = PACKED.test(raw) ? await unpack(raw) : params.get("code");
        if (found.code.length > CODE_MAX) throw new Error("too long");
        found.example = null;
      } catch {
        found.code = null;
        found.broken = true;
      }
    }
    const query = new URLSearchParams(location.search);
    if (["text", "title", "url"].some((k) => query.has(k))) {
      const text = query.get("text") || query.get("title") || "";
      const url = query.get("url");
      found.code = url && !text.includes(url) ? `${text ? text + "\n" : ""}# ${url}\n` : text;
      found.example = null;
      found.broken = false;
      history.replaceState(null, "", `${location.pathname}#code=${encodeURIComponent(found.code)}`);
    } else if (found.example || found.broken) {
      history.replaceState(null, "", location.pathname);
    }
    return found;
  }

  // A program from a link is shown without touching the draft saved in this browser:
  // the notice says so, and its button brings the draft back. The first change makes
  // the program the draft.
  const sharedNote = $("shared-note"), sharedText = $("shared-text"), backBtn = $("shared-back");
  const NOTE = sharedText.textContent;
  let linked = false;

  function setNote(message, back) {
    sharedText.textContent = message;
    backBtn.hidden = !back;
    sharedNote.hidden = !message;
  }

  function showLinked(asked) {
    source.value = asked.code;
    runMode = asked.mode;
    fileName = "main.ry";
    linked = true;
    setCustom();
    setNote(NOTE, true);
  }

  function leaveLink() {
    if (!sharedNote.hidden) setNote("", false);
    if (!linked) return;
    linked = false;
    store.set(SAVED.file, fileName);
    if (/(?:^|&)code=/.test(location.hash.slice(1))) history.replaceState(null, "", location.pathname + location.search);
  }

  backBtn.addEventListener("click", () => {
    linked = false;
    setNote("", false);
    history.replaceState(null, "", location.pathname + location.search);
    const saved = store.get(SAVED.code);
    if (saved) {
      fileName = store.get(SAVED.file) || "main.ry";
      source.value = saved;
      refreshEditor();
      setCustom();
      runMode = 0;
      execute(runMode);
    } else {
      const example = store.get(SAVED.example);
      loadExample(example && examples[example] ? example : "welcome");
    }
    // The button is gone: the focus goes to the top of the playground.
    exampleSel.focus();
  });

  // Share: the link, copied (or handed to the share sheet of a phone), and shown in a
  // field to select by hand when the clipboard says no.
  const shareBtn = $("share"), shareBox = $("share-box"), shareMsg = $("share-msg"), shareRow = $("share-row"), shareLink = $("share-link");
  shareBtn.hidden = false;

  function showShare(message, bad, link) {
    shareBox.hidden = false;
    shareBtn.setAttribute("aria-expanded", "true");
    shareMsg.textContent = message;
    shareMsg.classList.toggle("bad", bad);
    shareRow.hidden = !link;
    if (link) shareLink.value = link;
  }

  function closeShare() {
    shareBox.hidden = true;
    shareBtn.setAttribute("aria-expanded", "false");
    shareBtn.focus();
  }

  async function copyLink(link) {
    try {
      await navigator.clipboard.writeText(link);
      showShare(UI.shareCopied, false, link);
    } catch {
      showShare(UI.shareSelect, false, link);
      shareLink.focus();
      shareLink.select();
    }
  }

  shareBtn.addEventListener("click", async () => {
    const packed = await pack(source.value);
    if (packed.length > LINK_MAX) {
      const n = (x) => x.toLocaleString(document.documentElement.lang || lang);
      showShare(UI.shareTooLong.replace("{n}", n(packed.length)).replace("{max}", n(LINK_MAX)), true, null);
      return;
    }
    // From the steps, the link opens on the same step (of the same piece of code).
    const mode = stepsShown()
      ? `&mode=steps${stepsExpr ? "&expr=" + encodeURIComponent(stepsExpr) : ""}&step=${stepAt + 1}`
      : runMode === 2 ? "&mode=test" : "";
    const link = `${location.origin}${location.pathname}#code=${packed}${mode}`;
    if (navigator.share && window.matchMedia("(pointer: coarse)").matches) {
      showShare("", false, link);
      try {
        await navigator.share({ title: document.title, url: link });
        showShare(UI.shareShared, false, link);
        return;
      } catch (err) {
        if (err && err.name === "AbortError") {
          showShare(UI.shareSelect, false, link);
          return;
        }
      }
    }
    await copyLink(link);
  });
  $("share-copy").addEventListener("click", () => copyLink(shareLink.value));
  $("share-close").addEventListener("click", closeShare);
  // Escape closes the box, from the box or from the button that opened it.
  for (const el of [shareBox, shareBtn]) {
    el.addEventListener("keydown", (ev) => {
      if (ev.key === "Escape" && !shareBox.hidden) closeShare();
    });
  }

  // ----- start -----
  async function start() {
    const asked = await fromAddress();
    const saved = store.get(SAVED.code);
    if (asked.code !== null) {
      showLinked(asked);
    } else if (saved && !asked.example) {
      fileName = store.get(SAVED.file) || fileName;
      source.value = saved;
      setCustom();
    }
    // The example that was open, when nothing else is asked.
    const example = asked.example || (asked.code === null && !saved ? store.get(SAVED.example) : null);
    if (asked.broken) setNote(UI.shareBroken, false);
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
    for (const b of [runBtn, checkBtn, testBtn, stepsBtn, formatBtn]) b.disabled = false;
    newWorker();
    try {
      examples = await (await fetch(page.dataset.examples)).json();
    } catch {
      examples = {};
    }
    if (example && (example === "welcome" || examples[example])) {
      loadExample(example);
    } else if (asked.code !== null && asked.steps) {
      showSteps(asked.expr, asked.step);
    } else {
      execute(runMode);
    }
    showHelp(currentTopic, false);
  }

  start();

  // A link pasted in the address bar while the playground is open changes only the
  // fragment: the page stays, the program of the link comes in.
  window.addEventListener("hashchange", async () => {
    const asked = await fromAddress();
    if (asked.code !== null) {
      showLinked(asked);
      refreshEditor();
      if (asked.steps) showSteps(asked.expr, asked.step);
      else execute(runMode);
    } else if (asked.example && (asked.example === "welcome" || examples[asked.example])) {
      loadExample(asked.example);
    }
    if (asked.broken) setNote(UI.shareBroken, false);
  });

  // A .ry file opened from the system with the installed app (file_handlers of the
  // manifest; Chrome and Edge on computers).
  if ("launchQueue" in window) {
    window.launchQueue.setConsumer(async (params) => {
      if (params.files && params.files.length) openFile(await params.files[0].getFile());
    });
  }
})();
