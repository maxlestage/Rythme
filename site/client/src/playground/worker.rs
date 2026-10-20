//! The worker of the playground: the same bundle as the page, started in a web worker
//! (`/pkg/worker.js`), runs the programs away from the page, so that a program that
//! never stops can be stopped (the page ends this worker and starts a new one).
//!
//! It receives `{id, code, mode, lang, seed}` (the modes of `rythme_run`: 0 runs,
//! 1 checks, 2 tests, 5 shows the steps of one expression; `lang` is the position of the
//! language of the messages in `rythme::lang::LANGUAGES`; `seed` a whole number or
//! `null`) and answers `{id, text, ms}`, or `{id, trap, ms}` when the run broke the
//! WebAssembly itself (no more stack…): the page then starts a new worker, so that
//! nothing is left of it. It says `{ready: true}` once it listens.

use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};

use crate::js;

pub fn start() {
    let scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
    // The run is called through JavaScript: a trap of the WebAssembly (the stack of the
    // browser exhausted…) comes back as an error rather than ending this code.
    let run = Closure::<dyn Fn(JsValue) -> JsValue>::new(|job: JsValue| {
        let code = js::string(&job, "code").unwrap_or_default();
        let mode = js::get(&job, "mode").as_f64().unwrap_or(0.0) as u32;
        let lang = js::get(&job, "lang").as_f64().unwrap_or(0.0) as u32;
        let seed = js::get(&job, "seed").as_f64().and_then(rythme_playground::seed_of);
        JsValue::from_str(&rythme_playground::run(&code, mode, lang, seed))
    });
    let run: Function = run.into_js_value().unchecked_into();
    let s = scope.clone();
    let listener = Closure::<dyn FnMut(MessageEvent)>::new(move |msg: MessageEvent| {
        let job = msg.data();
        let id = js::get(&job, "id");
        let start = js_sys::Date::now();
        let answer = match run.call1(&JsValue::NULL, &job) {
            Ok(text) => js::object(&[("id", id), ("text", text), ("ms", JsValue::from_f64(js_sys::Date::now() - start))]),
            Err(trap) => js::object(&[
                ("id", id),
                ("trap", JsValue::from_str(&js::describe(&trap))),
                ("ms", JsValue::from_f64(js_sys::Date::now() - start)),
            ]),
        };
        let _ = s.post_message(&answer);
    });
    scope.set_onmessage(Some(listener.as_ref().unchecked_ref()));
    listener.forget();
    let _ = scope.post_message(&js::object(&[("ready", JsValue::TRUE)]));
}

/// The clock of the interpreter (`now`, `today`…), which the browser gives: the
/// interpreter asks for these two functions of the host (`rythme::io`), defined here.
#[unsafe(no_mangle)]
pub extern "C" fn rythme_now() -> f64 {
    js_sys::Date::now()
}

/// How many minutes the local time is ahead of UTC at `ms` (milliseconds since 1970).
#[unsafe(no_mangle)]
pub extern "C" fn rythme_offset(ms: f64) -> f64 {
    -js_sys::Date::new(&JsValue::from_f64(ms)).get_timezone_offset()
}
