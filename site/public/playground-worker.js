// Runs Rythme programs away from the page, so that a program that never stops can be
// stopped (the page terminates this worker and starts a new one).
"use strict";

let module = null;
const imports = { env: { rythme_now: () => Date.now(), rythme_offset: (ms) => -new Date(ms).getTimezoneOffset() } };

// mode: 0 = run, 1 = check, 2 = test, 4 = steps. Error messages come wrapped in U+0001 … U+0002.
// The language of the messages is its position in rythme::lang::LANGUAGES.
const LANGS = ["en", "fr", "es", "de", "pt", "zh"];
function callInterpreter(instance, code, mode, lang) {
  const e = instance.exports;
  const bytes = new TextEncoder().encode(code);
  const ptr = e.rythme_alloc(bytes.length);
  new Uint8Array(e.memory.buffer, ptr, bytes.length).set(bytes);
  const at = e.rythme_run(ptr, bytes.length, mode, Math.max(0, LANGS.indexOf(lang)));
  return new TextDecoder().decode(new Uint8Array(e.memory.buffer, at, e.rythme_result_len()));
}

onmessage = async (msg) => {
  const d = msg.data;
  if (d.module) {
    module = d.module;
    return;
  }
  const start = performance.now();
  try {
    // A fresh instance each time: a program that ran out of stack leaves nothing behind.
    const instance = await WebAssembly.instantiate(module, imports);
    const text = callInterpreter(instance, d.code, d.mode, d.lang);
    postMessage({ id: d.id, text, ms: performance.now() - start });
  } catch (err) {
    postMessage({ id: d.id, trap: String(err), ms: performance.now() - start });
  }
};
