//! The playground: the real Rythme interpreter, run in the browser. The page itself
//! comes from the server, in the language of its address; this module makes it work.
//! The code being edited stays in `localStorage`; a `.ry` file can be opened, from the
//! button, from the system (the installed app's file handler) or shared as text, and
//! downloaded. Share puts the program in a link (`#code=…`, in the link itself: nothing
//! is sent anywhere). Steps shows the calculation of one expression step by step, one
//! step at a time. Seed, next to Run, makes the same draws of `random` and `shuffle` on
//! every run. Copy for AI copies the program and its output as Markdown, for a chat with
//! an assistant.
//!
//! Programs run in a web worker made from this same bundle ([`worker`]), so that a
//! program that never stops can be stopped; formatting and the help, which always end,
//! run here.

mod links;
pub mod worker;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Element, Event, HtmlButtonElement, HtmlDetailsElement, HtmlElement, HtmlInputElement, HtmlOptionElement, HtmlSelectElement,
    HtmlTextAreaElement, KeyboardEvent, MessageEvent, ScrollBehavior, ScrollIntoViewOptions, ScrollLogicalPosition, Worker, WorkerOptions,
    WorkerType,
};

use crate::assistant::{self, Run};
use crate::dom::{self, id, on, on_capture};
use crate::editor;
use crate::highlight::editor_html;
use crate::js;
use crate::messages::{self, Piece};
use crate::share::LINK_MAX;
use crate::steps::{self, Step};
use links::Asked;

/// The topics of the help, as `rythme help` has them.
const TOPICS: [&str; 12] =
    ["start", "syntax", "functions", "lists", "text", "records", "types", "match", "errors", "functional", "tests", "builtins"];

/// Keys of `localStorage`: the code being edited ("" for an example as it is), the
/// example shown, the name of the opened file, and the seed ("" for none).
const SAVED_CODE: &str = "rythme-code";
const SAVED_EXAMPLE: &str = "rythme-example";
const SAVED_FILE: &str = "rythme-file";
const SAVED_SEED: &str = "rythme-seed";

/// The modes of the interpreter (`rythme_run`).
const RUN: u32 = 0;
const CHECK: u32 = 1;
const TEST: u32 = 2;
const FORMAT: u32 = 3;
const STEPS: u32 = 5;

/// The texts that change while the page runs, written by the server in the language of
/// the page (`data-ui`, from `src/i18n/` of the server).
struct Texts(JsValue);

impl Texts {
    fn get(&self, key: &str) -> String {
        js::string(&self.0, key).unwrap_or_default()
    }

    fn ran_in(&self, ms: f64) -> String {
        self.get("ranIn").replace("{ms}", &ms.to_string())
    }

    fn failed_in(&self, ms: f64) -> String {
        self.get("failedIn").replace("{ms}", &ms.to_string())
    }

    fn lines(&self, n: usize) -> String {
        self.get(if n == 1 { "line" } else { "lines" }).replace("{n}", &n.to_string())
    }

    fn steps_of(&self, n: usize, total: usize) -> String {
        self.get("stepsOf").replace("{n}", &n.to_string()).replace("{total}", &total.to_string())
    }

    fn seed_used(&self, seed: u64) -> String {
        self.get("seedUsed").replace("{seed}", &seed.to_string())
    }
}

/// What the last run gave, for the output and for Copy for AI.
struct Running {
    id: u32,
    mode: u32,
    seed: Option<u64>,
    code: String,
    expr: String,
}

/// The elements of the playground and its state.
struct Playground {
    page: HtmlElement,
    t: Texts,
    /// The position of the language of the page in `rythme::lang::LANGUAGES`: the
    /// language of the interpreter's messages.
    lang: u32,
    /// The welcome program, written in the page by the server.
    welcome: String,
    source: HtmlTextAreaElement,
    highlight: HtmlElement,
    gutter: HtmlElement,
    output: HtmlElement,
    status: HtmlElement,
    run: HtmlButtonElement,
    stop: HtmlButtonElement,
    check: HtmlButtonElement,
    test: HtmlButtonElement,
    format: HtmlButtonElement,
    steps_button: HtmlButtonElement,
    example: HtmlSelectElement,
    help_text: HtmlElement,
    file_name_el: HtmlElement,
    // Share and Copy for AI, and the notice of a program from a link.
    share: HtmlElement,
    share_box: HtmlElement,
    share_msg: HtmlElement,
    share_row: HtmlElement,
    share_link: HtmlInputElement,
    ai: HtmlElement,
    ai_row: HtmlElement,
    ai_text: HtmlTextAreaElement,
    shared_note: HtmlElement,
    shared_text: HtmlElement,
    back: HtmlElement,
    note: String,
    // The seed.
    seed_option: HtmlDetailsElement,
    seed_field: HtmlInputElement,
    seed_shown: HtmlElement,
    seed_used: HtmlElement,
    // The steps.
    stepper: HtmlElement,
    step_nav: HtmlElement,
    step_keys: HtmlElement,
    step_prev: HtmlButtonElement,
    step_next: HtmlButtonElement,
    step_count: HtmlElement,
    step_range: HtmlInputElement,
    step_from: HtmlElement,
    step_now: HtmlElement,
    step_printed: HtmlElement,
    step_print: HtmlElement,
    step_error: HtmlElement,
    step_live: HtmlElement,

    examples: RefCell<JsValue>,
    /// The name of the edited code: main.ry, or the name of the opened file.
    file_name: RefCell<String>,
    current_topic: RefCell<String>,
    worker: RefCell<Option<Worker>>,
    /// The worker listens (it says so once its bundle has started).
    ready: Cell<bool>,
    /// The run asked before the worker was ready.
    pending: RefCell<Option<JsValue>>,
    run_id: Cell<u32>,
    /// What Run does: run the program, or its tests (testing.ry, or a link with
    /// mode=test).
    run_mode: Cell<u32>,
    running: RefCell<Running>,
    last_run: RefCell<Option<Run>>,
    /// The program comes from a link, and is not the draft.
    linked: Cell<bool>,
    /// The button that opened the box of Share and Copy for AI.
    opener: RefCell<HtmlElement>,
    /// The code whose steps are shown ("": the last expression of the program), the
    /// steps, the one shown (from 0), the one to show first (from 1, from a link), and
    /// the errors shown after the last.
    steps_expr: RefCell<String>,
    steps: RefCell<Vec<Step>>,
    step_at: Cell<usize>,
    first_step: Cell<usize>,
    steps_failed: RefCell<String>,
    /// The last tap or key, and whether the run was asked by one.
    last_gesture: Cell<f64>,
    run_asked: Cell<bool>,
}

type Pg = Rc<Playground>;

pub fn start() {
    let Some(page) = dom::query::<HtmlElement>(".playground") else { return };
    let lang_code = page.get_attribute("data-lang").unwrap_or_else(|| "en".into());
    let lang = rythme::lang::Lang::all().position(|l| l.code() == lang_code).unwrap_or(0) as u32;
    let ui = js_sys::JSON::parse(&page.get_attribute("data-ui").unwrap_or_default()).unwrap_or_else(|_| js_sys::Object::new().into());
    let source: HtmlTextAreaElement = id("source");
    let stepper: HtmlElement = id("stepper");
    let shared_text: HtmlElement = id("shared-text");
    let pg = Rc::new(Playground {
        t: Texts(ui),
        lang,
        welcome: source.value(),
        highlight: id("highlight"),
        gutter: id("gutter"),
        output: id("output"),
        status: id("status"),
        run: id("run"),
        stop: id("stop"),
        check: id("check"),
        test: id("test"),
        format: id("format"),
        steps_button: id("steps"),
        example: id("example"),
        help_text: id("help-text"),
        file_name_el: id("file-name"),
        share: id("share"),
        share_box: id("share-box"),
        share_msg: id("share-msg"),
        share_row: id("share-row"),
        share_link: id("share-link"),
        ai: id("copy-ai"),
        ai_row: id("ai-row"),
        ai_text: id("ai-text"),
        shared_note: id("shared-note"),
        note: shared_text.text_content().unwrap_or_default(),
        shared_text,
        back: id("shared-back"),
        seed_option: id("seed-option"),
        seed_field: id("seed"),
        seed_shown: id("seed-shown"),
        seed_used: id("seed-used"),
        step_nav: dom::query_in(&stepper, ".stepper__nav").expect(".stepper__nav"),
        step_keys: dom::query_in(&stepper, ".stepper__keys").expect(".stepper__keys"),
        step_prev: id("step-prev"),
        step_next: id("step-next"),
        step_count: id("step-count"),
        step_range: id("step-range"),
        step_from: id("step-from"),
        step_now: id("step-now"),
        step_printed: id("step-printed"),
        step_print: id("step-print"),
        step_error: id("step-error"),
        step_live: id("step-live"),
        stepper,
        source,
        page,
        examples: RefCell::new(js_sys::Object::new().into()),
        file_name: RefCell::new("main.ry".into()),
        current_topic: RefCell::new("start".into()),
        worker: RefCell::new(None),
        ready: Cell::new(false),
        pending: RefCell::new(None),
        run_id: Cell::new(0),
        run_mode: Cell::new(RUN),
        running: RefCell::new(Running { id: 0, mode: RUN, seed: None, code: String::new(), expr: String::new() }),
        last_run: RefCell::new(None),
        linked: Cell::new(false),
        opener: RefCell::new(id("share")),
        steps_expr: RefCell::new(String::new()),
        steps: RefCell::new(Vec::new()),
        step_at: Cell::new(0),
        first_step: Cell::new(1),
        steps_failed: RefCell::new(String::new()),
        last_gesture: Cell::new(f64::NEG_INFINITY),
        run_asked: Cell::new(false),
    });
    editor_events(&pg);
    files(&pg);
    buttons(&pg);
    seed_option(&pg);
    stepper_events(&pg);
    help(&pg);
    sharing(&pg);
    keyboard_of_phones(&pg);
    let p = pg.clone();
    wasm_bindgen_futures::spawn_local(async move { p.open().await });
}

impl Playground {
    // ----- editor: highlighting, line numbers -----

    /// Each line number takes the height of its line, wrapped or not.
    fn number_lines(&self) {
        let lines = self.highlight.children();
        let numbers = dom::document().create_document_fragment();
        for k in 0..lines.length() {
            let Some(line) = lines.item(k) else { continue };
            let Ok(number) = dom::document().create_element("div") else { continue };
            number.set_text_content(Some(&(k + 1).to_string()));
            let height = line.get_bounding_client_rect().height();
            let _ = number.unchecked_ref::<HtmlElement>().style().set_property("height", &format!("{height}px"));
            let _ = numbers.append_child(&number);
        }
        self.gutter.set_text_content(None);
        let _ = self.gutter.append_child(&numbers);
        self.sync_scroll();
    }

    fn refresh_editor(&self) {
        let code = self.source.value();
        self.highlight.set_inner_html(&editor_html(&code));
        id::<HtmlElement>("lines").set_text_content(Some(&self.t.lines(code.split('\n').count())));
        self.number_lines();
    }

    fn sync_scroll(&self) {
        let top = self.source.scroll_top();
        self.highlight.set_scroll_top(top);
        self.gutter.set_scroll_top(top);
    }

    /// The selection of the field, as byte offsets of its text.
    fn selection(&self, value: &str) -> (usize, usize) {
        let start = self.source.selection_start().ok().flatten().unwrap_or(0) as usize;
        let end = self.source.selection_end().ok().flatten().unwrap_or(0) as usize;
        (editor::byte_at(value, start), editor::byte_at(value, end))
    }

    fn set_cursor(&self, utf16: usize) {
        let at = utf16 as u32;
        let _ = self.source.set_selection_start(Some(at));
        let _ = self.source.set_selection_end(Some(at));
    }

    fn insert_text(self: &Rc<Self>, text: &str) {
        let value = self.source.value();
        let (start, end) = self.selection(&value);
        let before = &value[..start];
        self.source.set_value(&format!("{before}{text}{}", &value[end..]));
        self.set_cursor(editor::utf16_len(before) + editor::utf16_len(text));
        self.on_edit();
    }

    fn set_custom(&self) {
        let options = self.example.options();
        let has = (0..options.length()).filter_map(|i| options.item(i)).any(|o| o.unchecked_ref::<HtmlOptionElement>().value() == "custom");
        if !has && let Ok(option) = HtmlOptionElement::new_with_text_and_value(&self.t.get("custom"), "custom") {
            let _ = self.example.append_child(&option);
        }
        self.example.set_value("custom");
        self.file_name_el.set_text_content(Some(&self.file_name.borrow()));
    }

    fn close_share_box(&self) {
        if !self.share_box.hidden() {
            self.share_box.set_hidden(true);
            let _ = self.share.set_attribute("aria-expanded", "false");
        }
    }

    fn on_edit(self: &Rc<Self>) {
        self.leave_link();
        self.close_share_box();
        self.refresh_editor();
        dom::save(SAVED_CODE, &self.source.value());
        self.set_custom();
    }

    // ----- .ry files -----

    fn open_text(self: &Rc<Self>, code: &str, name: &str) {
        self.source.set_value(code);
        *self.file_name.borrow_mut() = editor::opened_name(name);
        dom::save(SAVED_FILE, &self.file_name.borrow());
        self.on_edit();
        self.run_mode.set(RUN);
        self.execute(RUN);
    }

    async fn open_file(self: &Rc<Self>, file: web_sys::File) {
        match js::wait(file.text().into()).await {
            Ok(text) => self.open_text(&text.as_string().unwrap_or_default(), &file.name()),
            Err(err) => self.render_output(&format!("\u{1}{}\u{2}", js::describe(&err)), &self.output),
        }
    }

    // ----- the interpreter -----

    /// A new worker, the old one stopped.
    fn new_worker(self: &Rc<Self>) {
        if let Some(old) = self.worker.borrow_mut().take() {
            old.terminate();
        }
        self.ready.set(false);
        let options = WorkerOptions::new();
        options.set_type(WorkerType::Module);
        let url = self.page.get_attribute("data-worker").unwrap_or_default();
        let Ok(worker) = Worker::new_with_options(&url, &options) else {
            self.load_failed("Worker");
            return;
        };
        let p = self.clone();
        let w = worker.clone();
        let listener = Closure::<dyn FnMut(MessageEvent)>::new(move |msg: MessageEvent| {
            // An answer of a worker that was replaced since: nothing to do with it.
            if p.worker.borrow().as_ref() != Some(&w) {
                return;
            }
            let data = msg.data();
            if js::get(&data, "ready").is_truthy() {
                p.ready.set(true);
                if let Some(job) = p.pending.borrow_mut().take() {
                    let _ = w.post_message(&job);
                }
            } else {
                p.finish(&data);
            }
        });
        worker.set_onmessage(Some(listener.as_ref().unchecked_ref()));
        listener.forget();
        // The worker's script could not start (a broken deployment, no WebAssembly…).
        let p = self.clone();
        let w = worker.clone();
        let failed = Closure::<dyn FnMut(Event)>::new(move |ev: Event| {
            ev.prevent_default();
            if p.worker.borrow().as_ref() != Some(&w) {
                return;
            }
            let message = js::string(&ev, "message").unwrap_or_default();
            if p.ready.get() {
                let id = p.running.borrow().id;
                p.finish(&js::object(&[("id", JsValue::from(id)), ("trap", JsValue::from_str(&message)), ("ms", JsValue::from(0))]));
            } else {
                p.load_failed(&message);
            }
        });
        worker.set_onerror(Some(failed.as_ref().unchecked_ref()));
        failed.forget();
        *self.worker.borrow_mut() = Some(worker);
    }

    /// The interpreter cannot start: said in the output.
    fn load_failed(&self, why: &str) {
        let _ = self.page.class_list().add_1("no-wasm");
        self.set_running(false);
        self.output.set_text_content(None);
        let _ = self.output.append_child(&text(&format!("{}{why}", self.t.get("loadFailed")), Some("err")));
    }

    fn set_running(&self, on: bool) {
        let _ = dom::body().class_list().toggle_with_force("running", on);
        self.run.set_hidden(on);
        self.stop.set_hidden(!on);
        self.check.set_disabled(on);
        self.test.set_disabled(on);
        self.steps_button.set_disabled(on);
    }

    fn is_running() -> bool {
        dom::body().class_list().contains("running")
    }

    fn execute(self: &Rc<Self>, mode: u32) {
        self.run_asked.set(dom::now() - self.last_gesture.get() < 2000.0);
        let id = self.run_id.get() + 1;
        self.run_id.set(id);
        // Check runs nothing: no draws.
        let seed = if mode == CHECK { None } else { self.seed() };
        let code = self.source.value();
        let expr = self.steps_expr.borrow().clone();
        // Steps: the program, U+0000, then the expression to compute ("": the last one).
        let job_code = if mode == STEPS { format!("{code}\u{0}{expr}") } else { code.clone() };
        *self.running.borrow_mut() = Running { id, mode, seed, code, expr };
        self.set_running(true);
        self.status.set_class_name("status");
        self.status.set_text_content(Some(&self.t.get("running")));
        let job = js::object(&[
            ("id", JsValue::from(id)),
            ("code", JsValue::from_str(&job_code)),
            ("mode", JsValue::from(mode)),
            ("lang", JsValue::from(self.lang)),
            ("seed", seed.map_or(JsValue::NULL, |s| JsValue::from_f64(s as f64))),
        ]);
        if self.ready.get()
            && let Some(worker) = self.worker.borrow().as_ref()
        {
            let _ = worker.post_message(&job);
        } else {
            *self.pending.borrow_mut() = Some(job);
        }
    }

    fn finish(self: &Rc<Self>, result: &JsValue) {
        if js::get(result, "id").as_f64() != Some(f64::from(self.run_id.get())) {
            return;
        }
        self.set_running(false);
        let ms = js::get(result, "ms").as_f64().unwrap_or(0.0).round().max(1.0);
        let trap = js::string(result, "trap");
        let (mode, seed, code, expr) = {
            let r = self.running.borrow();
            (r.mode, r.seed, r.code.clone(), r.expr.clone())
        };
        let stepping = mode == STEPS && trap.is_none();
        self.show_stepper(stepping);
        self.seed_used.set_hidden(seed.is_none());
        self.seed_used.set_text_content(Some(&seed.map(|s| self.t.seed_used(s)).unwrap_or_default()));
        if let Some(trap) = trap {
            let mut shown = self.t.get("tooDeep");
            if !trap.to_lowercase().contains("stack") {
                shown.push('\n');
                shown.push_str(&trap);
            }
            *self.last_run.borrow_mut() = Some(Run { mode, seed, expr, code, text: shown.clone() });
            self.render_output(&format!("\u{1}{shown}\u{2}"), &self.output);
            self.status.set_class_name("status bad");
            self.status.set_text_content(Some(&self.t.failed_in(ms)));
            self.new_worker();
            return;
        }
        let text = js::string(result, "text").unwrap_or_default();
        *self.last_run.borrow_mut() = Some(Run { mode, seed, expr, code, text: text.clone() });
        let failed = if stepping {
            self.render_steps(&text)
        } else {
            self.render_output(&text, &self.output);
            messages::failed(&text)
        };
        self.status.set_class_name(if failed { "status bad" } else { "status good" });
        self.status.set_text_content(Some(&if failed { self.t.failed_in(ms) } else { self.t.ran_in(ms) }));
        // On a phone the output is under the editor: bring it into view.
        let typing = dom::active().is_some_and(|a| a == *self.source.unchecked_ref::<Element>());
        if self.run_asked.get() && !dom::media("(min-width: 960px)") && !typing {
            let options = ScrollIntoViewOptions::new();
            options.set_behavior(ScrollBehavior::Smooth);
            options.set_block(ScrollLogicalPosition::Nearest);
            let target: &HtmlElement = if stepping { &self.stepper } else { &self.output };
            target.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }

    /// The interpreter, here in the page, for what always ends (formatting, the help):
    /// called through JavaScript so that a trap comes back as an error.
    fn run_here(&self, code: &str, mode: u32) -> Result<String, String> {
        let lang = self.lang;
        let run = Closure::<dyn Fn(JsValue) -> JsValue>::new(move |code: JsValue| {
            JsValue::from_str(&rythme_playground::run(&code.as_string().unwrap_or_default(), mode, lang, None))
        });
        let function: &Function = run.as_ref().unchecked_ref();
        function.call1(&JsValue::NULL, &JsValue::from_str(code)).map(|v| v.as_string().unwrap_or_default()).map_err(|e| js::describe(&e))
    }

    // ----- the seed of the draws -----
    // Empty: the draws of `random` and `shuffle` change from run to run. A whole number:
    // every run makes the same draws (`rythme run --seed N`), and Test the same values
    // of `check_all` and `sample` too (`rythme test --seed N`). It is kept in this
    // browser with the draft, travels with Share (`&seed=N`) and is shown in the head of
    // the output after a run that used it. The dice picks a new one and runs.

    fn seed(&self) -> Option<u64> {
        let value = self.seed_field.value();
        if links::is_seed(&value) { value.parse().ok() } else { None }
    }

    /// Shows a seed (digits, or nothing), without saving it.
    fn show_seed(&self, value: Option<&str>) {
        let value = value.filter(|v| links::is_seed(v)).and_then(|v| v.parse::<u64>().ok()).map(|n| n.to_string()).unwrap_or_default();
        self.seed_field.set_value(&value);
        self.seed_shown.set_text_content(Some(&value));
    }

    /// The seed was changed by hand: saved, and a program from a link becomes the draft
    /// (with its seed), as when it is edited.
    fn seed_changed(self: &Rc<Self>) {
        self.seed_shown.set_text_content(Some(&self.seed_field.value()));
        if self.linked.get() {
            self.on_edit();
        } else {
            self.close_share_box();
        }
        dom::save(SAVED_SEED, &self.seed_field.value());
    }

    // ----- format: the layout of `rythme fmt`, done at once (it cannot loop) -----

    fn format_code(self: &Rc<Self>) {
        let code = self.source.value();
        let result = self.run_here(&code, FORMAT).unwrap_or_else(|err| format!("\u{1}{err}\u{2}"));
        self.status.set_class_name("status good");
        if result.starts_with('\u{1}') {
            self.render_output(&result, &self.output);
            self.status.set_class_name("status bad");
            self.status.set_text_content(Some(&self.t.get("notFormatted")));
            return;
        }
        if result == code {
            self.status.set_text_content(Some(&self.t.get("alreadyFormatted")));
            return;
        }
        // Keep the cursor on the same line, and the view where it was.
        let (start, _) = self.selection(&code);
        let line = code[..start].split('\n').count();
        let top = self.source.scroll_top();
        // With a mouse, through the editing commands so that Ctrl+Z undoes it; on a
        // phone, without opening the keyboard.
        let desktop = dom::media("(hover: hover) and (pointer: fine)");
        if desktop {
            let _ = self.source.focus();
            self.source.select();
        }
        let inserted = desktop
            && dom::document()
                .dyn_into::<web_sys::HtmlDocument>()
                .ok()
                .and_then(|d| d.exec_command_with_show_ui_and_value("insertText", false, &result).ok())
                .unwrap_or(false);
        if !inserted {
            self.source.set_value(&result);
        }
        let before: Vec<&str> = result.split('\n').take(line - 1).collect();
        let at = editor::utf16_len(&before.join("\n")) + usize::from(line > 1);
        self.set_cursor(at.min(editor::utf16_len(&result)));
        self.source.set_scroll_top(top);
        self.on_edit();
        self.status.set_text_content(Some(&self.t.get("formatted")));
    }

    // ----- output: messages styled like in a terminal -----

    fn render_output(self: &Rc<Self>, raw: &str, into: &HtmlElement) {
        into.set_text_content(None);
        let Some(pieces) = messages::pieces(raw) else {
            let _ = into.append_child(&text(&self.t.get("nothing"), Some("empty")));
            return;
        };
        for piece in pieces {
            match piece {
                Piece::Text(s, class) => {
                    let _ = into.append_child(&text(&s, class));
                }
                Piece::Explain(code) => {
                    let Ok(button) = dom::document().create_element("button") else { continue };
                    let _ = button.set_attribute("type", "button");
                    button.set_class_name("explain");
                    button.set_text_content(Some(&code));
                    let _ = button.set_attribute("title", &format!("{} {code}", self.t.get("explain")));
                    let p = self.clone();
                    on::<Event>(&button, "click", move |_| p.show_help(&code, true));
                    let _ = into.append_child(&button);
                }
            }
        }
    }

    // ----- steps: the calculation of one expression, one step at a time -----

    fn show_steps(self: &Rc<Self>, expr: &str, step: usize) {
        *self.steps_expr.borrow_mut() = expr.to_owned();
        self.first_step.set(step);
        self.execute(STEPS);
    }

    fn show_stepper(&self, on: bool) {
        self.stepper.set_hidden(!on);
        self.output.set_hidden(on);
    }

    fn steps_shown(&self) -> bool {
        !self.stepper.hidden() && !self.steps.borrow().is_empty()
    }

    /// Returns whether the calculation stopped with an error.
    fn render_steps(self: &Rc<Self>, raw: &str) -> bool {
        let (body, failed) = steps::split(raw);
        let none = body.contains('\u{5}');
        *self.steps.borrow_mut() = if none { Vec::new() } else { steps::parse(&body) };
        *self.steps_failed.borrow_mut() = failed.clone();
        let empty = self.steps.borrow().is_empty();
        for el in [&self.step_nav, self.step_range.unchecked_ref(), &self.step_keys] {
            el.set_hidden(empty);
        }
        if !empty {
            self.show_step(self.first_step.get() as i64 - 1);
        } else {
            self.step_from.set_text_content(None);
            self.step_now.set_text_content(None);
            let shown = self.t.get(if none { "stepsNone" } else { "nothing" });
            let _ = self.step_now.append_child(&text(&shown, Some("empty")));
            self.step_now.set_hidden(!none && !failed.is_empty());
            self.step_printed.set_hidden(true);
            self.step_error.set_hidden(failed.is_empty());
            if !failed.is_empty() {
                self.render_output(&failed, &self.step_error);
            }
            self.step_live.set_text_content(None);
        }
        self.first_step.set(1);
        !failed.is_empty()
    }

    fn show_step(self: &Rc<Self>, i: i64) {
        let steps = self.steps.borrow();
        let n = steps.len();
        if n == 0 {
            return;
        }
        let at = i.clamp(0, n as i64 - 1) as usize;
        self.step_at.set(at);
        let step = &steps[at];
        let count = self.t.steps_of(at + 1, n);
        self.step_count.set_text_content(Some(&count));
        self.step_range.set_max(&n.to_string());
        self.step_range.set_value(&(at + 1).to_string());
        let _ = self.step_range.set_attribute("aria-valuetext", &count);
        // A button that gets disabled loses the focus: it goes to the slider.
        let focused = dom::active();
        self.step_prev.set_disabled(at == 0);
        self.step_next.set_disabled(at == n - 1);
        if focused.is_some_and(|f| js::get(&f, "disabled") == JsValue::TRUE) {
            let _ = self.step_range.focus();
        }
        if at == 0 {
            self.step_from.set_text_content(None);
        } else if step.gap {
            self.step_from.set_text_content(Some("…"));
        } else {
            show_term(&self.step_from, &steps[at - 1].lines);
        }
        self.step_now.set_hidden(false);
        show_term(&self.step_now, &step.lines);
        self.step_printed.set_hidden(step.printed.is_empty());
        self.step_print.set_text_content(Some(&step.printed.join("\n")));
        let failed = self.steps_failed.borrow().clone();
        let last = at == n - 1 && !failed.is_empty();
        self.step_error.set_hidden(!last);
        if last {
            self.render_output(&failed, &self.step_error);
        }
        self.step_live.set_text_content(Some(&steps::spoken(&count, step)));
    }

    // ----- help, from the interpreter itself -----

    fn show_help(self: &Rc<Self>, name: &str, scroll: bool) {
        *self.current_topic.borrow_mut() = name.to_owned();
        for button in dom::query_all::<Element>(None, ".topic") {
            let pressed = button.get_attribute("data-topic").as_deref() == Some(name);
            let _ = button.set_attribute("aria-pressed", if pressed { "true" } else { "false" });
        }
        let literal = format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\"").replace('{', "\\{").replace('\n', " "));
        let shown = match self.run_here(&format!("help({literal})"), RUN) {
            Ok(raw) => raw.replace(['\u{1}', '\u{2}'], "").trim().to_owned(),
            Err(err) => err,
        };
        self.help_text.set_text_content(Some(&shown));
        if scroll {
            let options = ScrollIntoViewOptions::new();
            options.set_behavior(ScrollBehavior::Smooth);
            options.set_block(ScrollLogicalPosition::Start);
            self.help_text.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }

    // ----- examples -----

    fn has_example(&self, name: &str) -> bool {
        name == "welcome" || js::string(&self.examples.borrow(), name).is_some()
    }

    fn load_example(self: &Rc<Self>, name: &str) {
        self.leave_link();
        let code = if name == "welcome" { self.welcome.clone() } else { js::string(&self.examples.borrow(), name).unwrap_or_default() };
        self.source.set_value(&code);
        self.example.set_value(name);
        self.file_name_el.set_text_content(Some(&if name == "welcome" { "main.ry".to_owned() } else { format!("{name}.ry") }));
        *self.file_name.borrow_mut() = "main.ry".into();
        self.refresh_editor();
        dom::save(SAVED_CODE, "");
        dom::save(SAVED_EXAMPLE, name);
        dom::save(SAVED_FILE, "main.ry");
        self.run_mode.set(if name == "testing" { TEST } else { RUN });
        self.execute(self.run_mode.get());
    }

    // ----- a program from a link -----
    // It is shown without touching the draft saved in this browser: the notice says so,
    // and its button brings the draft back. The first change makes the program the draft.

    fn set_note(&self, message: &str, back: bool) {
        self.shared_text.set_text_content(Some(message));
        self.back.set_hidden(!back);
        self.shared_note.set_hidden(message.is_empty());
    }

    fn show_linked(&self, asked: &Asked) {
        self.source.set_value(asked.code.as_deref().unwrap_or_default());
        self.run_mode.set(asked.mode);
        *self.file_name.borrow_mut() = "main.ry".into();
        self.linked.set(true);
        // The seed of the link (none: draws that change), without touching the saved one.
        self.show_seed(asked.seed.as_deref());
        self.set_custom();
        self.set_note(&self.note, true);
    }

    fn leave_link(&self) {
        if !self.shared_note.hidden() {
            self.set_note("", false);
        }
        if !self.linked.get() {
            return;
        }
        self.linked.set(false);
        dom::save(SAVED_FILE, &self.file_name.borrow());
        dom::save(SAVED_SEED, &self.seed_field.value());
        let hash = dom::window().location().hash().unwrap_or_default();
        if hash.trim_start_matches('#').split('&').any(|p| p.starts_with("code=")) {
            replace_address_without_hash();
        }
    }

    // ----- Share and Copy for AI -----

    fn show_share(&self, message: &str, bad: bool, link: Option<&str>, text: Option<&str>) {
        self.share_box.set_hidden(false);
        let opener = self.opener.borrow();
        for b in [&self.share, &self.ai] {
            let _ = b.set_attribute("aria-expanded", if b == &*opener { "true" } else { "false" });
        }
        self.share_msg.set_text_content(Some(message));
        let _ = self.share_msg.class_list().toggle_with_force("bad", bad);
        self.share_row.set_hidden(link.is_none());
        if let Some(link) = link {
            self.share_link.set_value(link);
        }
        self.ai_row.set_hidden(text.is_none());
        if let Some(text) = text {
            self.ai_text.set_value(text);
        }
    }

    fn close_share(&self) {
        self.share_box.set_hidden(true);
        for b in [&self.share, &self.ai] {
            let _ = b.set_attribute("aria-expanded", "false");
        }
        let _ = self.opener.borrow().focus();
    }

    async fn copy_link(&self, link: &str) {
        if write_clipboard(link).await {
            self.show_share(&self.t.get("shareCopied"), false, Some(link), None);
        } else {
            self.show_share(&self.t.get("shareSelect"), false, Some(link), None);
            let _ = self.share_link.focus();
            self.share_link.select();
        }
    }

    async fn share_link(self: &Rc<Self>) {
        *self.opener.borrow_mut() = self.share.clone();
        let packed = links::pack(&self.source.value()).await;
        if packed.len() > LINK_MAX {
            let lang = dom::root().get_attribute("lang").filter(|l| !l.is_empty()).unwrap_or_default();
            let n = |x: usize| String::from(js_sys::Number::from(x as f64).to_locale_string(&lang));
            let message = self.t.get("shareTooLong").replace("{n}", &n(packed.len())).replace("{max}", &n(LINK_MAX));
            self.show_share(&message, true, None, None);
            return;
        }
        // From the steps, the link opens on the same step (of the same piece of code).
        let mode = if self.steps_shown() {
            let expr = self.steps_expr.borrow();
            let expr = if expr.is_empty() { String::new() } else { format!("&expr={}", String::from(js_sys::encode_uri_component(&expr))) };
            format!("&mode=steps{expr}&step={}", self.step_at.get() + 1)
        } else if self.run_mode.get() == TEST {
            "&mode=test".to_owned()
        } else {
            String::new()
        };
        let seeded = self.seed().map(|s| format!("&seed={s}")).unwrap_or_default();
        let location = dom::window().location();
        let link =
            format!("{}{}#code={packed}{seeded}{mode}", location.origin().unwrap_or_default(), location.pathname().unwrap_or_default());
        let navigator = dom::window().navigator();
        if js::has(&navigator, "share") && js::get(&navigator, "share").is_truthy() && dom::media("(pointer: coarse)") {
            self.show_share("", false, Some(&link), None);
            let data = js::object(&[("title", JsValue::from_str(&dom::document().title())), ("url", JsValue::from_str(&link))]);
            match js::call_async(&navigator, "share", &[data]).await {
                Ok(_) => {
                    self.show_share(&self.t.get("shareShared"), false, Some(&link), None);
                    return;
                }
                Err(err) if js::string(&err, "name").as_deref() == Some("AbortError") => {
                    self.show_share(&self.t.get("shareSelect"), false, Some(&link), None);
                    return;
                }
                Err(_) => {}
            }
        }
        self.copy_link(&link).await;
    }

    /// Copy for AI: the program and what its last run gave, as Markdown.
    fn for_assistant(&self) -> String {
        let href = dom::window().location().href().unwrap_or_default();
        let llms = self.page.get_attribute("data-llms").unwrap_or_default();
        let llms = web_sys::Url::new_with_base(&llms, &href).map(|u| u.href()).unwrap_or(llms);
        let t = assistant::Texts { intro: &self.t.get("aiIntro"), nothing: &self.t.get("nothing"), steps_none: &self.t.get("stepsNone") };
        let shown = self.file_name_el.text_content().unwrap_or_default();
        assistant::markdown(&t, &llms, &shown, &self.source.value(), self.last_run.borrow().as_ref())
    }

    async fn copy_for_ai(self: &Rc<Self>) {
        *self.opener.borrow_mut() = self.ai.clone();
        let text = self.for_assistant();
        if write_clipboard(&text).await {
            self.show_share(&self.t.get("aiCopied"), false, None, None);
        } else {
            // No clipboard (refused, or an insecure page): the text, selected, to copy.
            self.show_share(&self.t.get("aiSelect"), false, None, Some(&text));
            let _ = self.ai_text.focus();
            self.ai_text.select();
        }
    }

    // ----- start -----

    async fn open(self: &Rc<Self>) {
        let asked = links::from_address().await;
        let saved = dom::load(SAVED_CODE).filter(|s| !s.is_empty());
        self.show_seed(dom::load(SAVED_SEED).as_deref());
        if asked.code.is_some() {
            self.show_linked(&asked);
        } else if let Some(saved) = saved.as_ref().filter(|_| asked.example.is_none()) {
            if let Some(file) = dom::load(SAVED_FILE).filter(|f| !f.is_empty()) {
                *self.file_name.borrow_mut() = file;
            }
            self.source.set_value(saved);
            self.set_custom();
        }
        // The example that was open, when nothing else is asked.
        let example =
            asked.example.clone().or_else(|| if asked.code.is_none() && saved.is_none() { dom::load(SAVED_EXAMPLE) } else { None });
        if asked.broken {
            self.set_note(&self.t.get("shareBroken"), false);
        }
        self.refresh_editor();
        self.output.set_text_content(None);
        let _ = self.output.append_child(&text(&self.t.get("loading"), Some("empty")));
        for b in [&self.run, &self.check, &self.test, &self.steps_button, &self.format] {
            b.set_disabled(false);
        }
        self.new_worker();
        let url = self.page.get_attribute("data-examples").unwrap_or_default();
        if let Ok(examples) = fetch_json(&url).await {
            *self.examples.borrow_mut() = examples;
        }
        match example.filter(|e| self.has_example(e)) {
            Some(example) => self.load_example(&example),
            None if asked.code.is_some() && asked.steps => self.show_steps(&asked.expr, asked.step),
            None => self.execute(self.run_mode.get()),
        }
        let topic = self.current_topic.borrow().clone();
        self.show_help(&topic, false);
    }

    /// A link pasted in the address bar while the playground is open changes only the
    /// fragment: the page stays, the program of the link comes in.
    async fn address_changed(self: &Rc<Self>) {
        let asked = links::from_address().await;
        if asked.code.is_some() {
            self.show_linked(&asked);
            self.refresh_editor();
            if asked.steps {
                self.show_steps(&asked.expr, asked.step);
            } else {
                self.execute(self.run_mode.get());
            }
        } else if let Some(example) = asked.example.as_ref().filter(|e| self.has_example(e)) {
            self.load_example(example);
        }
        if asked.broken {
            self.set_note(&self.t.get("shareBroken"), false);
        }
    }
}

/// A piece of text: a text node, or a `span` of a class.
fn text(s: &str, class: Option<&str>) -> web_sys::Node {
    let document = dom::document();
    match class {
        None => document.create_text_node(s).into(),
        Some(class) => {
            let el = document.create_element("span").expect("a span");
            el.set_class_name(class);
            el.set_text_content(Some(s));
            el.into()
        }
    }
}

/// A step, colored like the editor; each line wraps under its indentation.
fn show_term(el: &HtmlElement, lines: &[String]) {
    el.set_inner_html(&editor_html(&lines.join("\n")));
    let children = el.children();
    for k in 0..children.length() {
        let Some(line) = children.item(k) else { continue };
        line.set_class_name("line");
        let indent = line.text_content().unwrap_or_default().chars().take_while(|c| *c == ' ').count();
        if indent > 0 {
            let _ = line.unchecked_ref::<HtmlElement>().style().set_property("--indent", &indent.to_string());
        }
    }
}

/// The address of the page without its fragment.
fn replace_address_without_hash() {
    let location = dom::window().location();
    let url = format!("{}{}", location.pathname().unwrap_or_default(), location.search().unwrap_or_default());
    if let Ok(history) = dom::window().history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }
}

/// Writes `text` to the clipboard (the Clipboard API, looked up now: a page may have
/// none, or refuse); whether it worked.
async fn write_clipboard(text: &str) -> bool {
    let clipboard = js::get(&dom::window().navigator(), "clipboard");
    js::call_async(&clipboard, "writeText", &[JsValue::from_str(text)]).await.is_ok()
}

async fn fetch_json(url: &str) -> Result<JsValue, JsValue> {
    let response = js::wait(dom::window().fetch_with_str(url).into()).await?;
    js::call_async(&response, "json", &[]).await
}

// ----- the events of the page -----

fn editor_events(pg: &Pg) {
    let p = pg.clone();
    on::<Event>(&pg.source, "input", move |_| p.on_edit());
    let p = pg.clone();
    on::<Event>(&pg.source, "scroll", move |_| p.sync_scroll());
    let p = pg.clone();
    on::<KeyboardEvent>(&pg.source, "keydown", move |ev| {
        if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "Enter" {
            ev.prevent_default();
            p.execute(if ev.shift_key() { TEST } else { RUN });
        } else if ev.shift_key() && ev.alt_key() && ev.code() == "KeyF" {
            ev.prevent_default();
            p.format_code();
        } else if ev.key() == "Tab" && !ev.shift_key() && !ev.alt_key() {
            ev.prevent_default();
            p.insert_text("  ");
        } else if ev.key() == "Enter" && !ev.ctrl_key() && !ev.meta_key() {
            // Keep the indentation, and indent after a line that opens a block.
            let value = p.source.value();
            let (start, _) = p.selection(&value);
            let before = &value[..start];
            let line = &before[before.rfind('\n').map_or(0, |n| n + 1)..];
            let indent = editor::next_indent(line);
            ev.prevent_default();
            p.insert_text(&format!("\n{indent}"));
        }
    });
    // The lines wrap again when the editor changes width, or when its font arrives.
    let p = pg.clone();
    let width = Cell::new(-1);
    let resized = Closure::<dyn FnMut()>::new(move || {
        if p.highlight.client_width() != width.get() {
            width.set(p.highlight.client_width());
            p.number_lines();
        }
    });
    if let Ok(observer) = web_sys::ResizeObserver::new(resized.as_ref().unchecked_ref()) {
        observer.observe(&pg.highlight);
    }
    resized.forget();
    let fonts = js::get(&dom::document(), "fonts");
    if fonts.is_object() {
        let p = pg.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if js::wait(js::get(&fonts, "ready")).await.is_ok() {
                p.number_lines();
            }
        });
    }
}

fn files(pg: &Pg) {
    let open: HtmlElement = id("open");
    let save: HtmlElement = id("save");
    let input: HtmlInputElement = id("open-file");
    open.set_hidden(false);
    save.set_hidden(false);
    let i = input.clone();
    on::<Event>(&open, "click", move |_| i.click());
    let (p, i) = (pg.clone(), input.clone());
    on::<Event>(&input, "change", move |_| {
        if let Some(file) = i.files().and_then(|f| f.get(0)) {
            let p = p.clone();
            wasm_bindgen_futures::spawn_local(async move { p.open_file(file).await });
        }
        i.set_value("");
    });
    let p = pg.clone();
    on::<Event>(&save, "click", move |_| {
        let name = editor::download_name(&p.file_name_el.text_content().unwrap_or_default());
        let parts = js_sys::Array::of1(&JsValue::from_str(&p.source.value()));
        let options = web_sys::BlobPropertyBag::new();
        options.set_type("text/plain;charset=utf-8");
        let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options) else { return };
        let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else { return };
        let Ok(link) = dom::document().create_element("a") else { return };
        let link: web_sys::HtmlAnchorElement = link.unchecked_into();
        link.set_href(&url);
        link.set_download(&name);
        let _ = dom::body().append_child(&link);
        link.click();
        link.remove();
        dom::later(1000, move || {
            let _ = web_sys::Url::revoke_object_url(&url);
        });
    });
    // The last keystrokes, if the app is closed before the storage was written.
    let p = pg.clone();
    on::<Event>(&dom::window(), "pagehide", move |_| {
        if p.example.value() == "custom" && !p.linked.get() {
            dom::save(SAVED_CODE, &p.source.value());
        }
    });
    // A .ry file opened from the system with the installed app (file_handlers of the
    // manifest; Chrome and Edge on computers).
    let queue = js::get(&dom::window(), "launchQueue");
    if js::has(&dom::window(), "launchQueue") && queue.is_object() {
        let p = pg.clone();
        let consumer = Closure::<dyn FnMut(JsValue)>::new(move |params: JsValue| {
            let files = js::get(&params, "files");
            let first = js::get(&files, "0");
            if js::get(&files, "length").as_f64().unwrap_or(0.0) < 1.0 || !first.is_object() {
                return;
            }
            let p = p.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(file) = js::call_async(&first, "getFile", &[]).await {
                    p.open_file(file.unchecked_into()).await;
                }
            });
        });
        let _ = js::call(&queue, "setConsumer", &[consumer.as_ref().clone()]);
        consumer.forget();
    }
}

fn buttons(pg: &Pg) {
    // A run asked by a tap or a key (not the one of the page opening) brings its output
    // into view on a phone; the page does not jump by itself.
    for event in ["pointerdown", "keydown"] {
        let p = pg.clone();
        on_capture::<Event>(&dom::document(), event, move |_| p.last_gesture.set(dom::now()));
    }
    let p = pg.clone();
    on::<Event>(&pg.stop, "click", move |_| {
        p.run_id.set(p.run_id.get() + 1);
        p.new_worker();
        p.set_running(false);
        p.status.set_class_name("status bad");
        p.status.set_text_content(Some(&p.t.get("stopped")));
    });
    let p = pg.clone();
    on::<Event>(&pg.run, "click", move |_| p.execute(p.run_mode.get()));
    let p = pg.clone();
    on::<Event>(&pg.check, "click", move |_| p.execute(CHECK));
    let p = pg.clone();
    on::<Event>(&pg.test, "click", move |_| p.execute(TEST));
    // The calculation of the selected code, or of the last expression, step by step.
    let p = pg.clone();
    on::<Event>(&pg.steps_button, "click", move |_| {
        let value = p.source.value();
        let (start, end) = p.selection(&value);
        let selected = value[start..end.max(start)].trim();
        let expr = if selected == value.trim() { "" } else { selected };
        p.show_steps(expr, 1);
    });
    let p = pg.clone();
    on::<Event>(&pg.format, "click", move |_| p.format_code());
    let p = pg.clone();
    on::<Event>(&pg.example, "change", move |_| {
        let value = p.example.value();
        if value != "custom" {
            p.load_example(&value);
        }
    });
}

fn seed_option(pg: &Pg) {
    pg.seed_option.set_hidden(false);
    let p = pg.clone();
    on::<Event>(&pg.seed_field, "input", move |_| {
        let digits: String = p.seed_field.value().chars().filter(char::is_ascii_digit).take(15).collect();
        if digits != p.seed_field.value() {
            p.seed_field.set_value(&digits);
        }
        p.seed_changed();
    });
    let p = pg.clone();
    on::<KeyboardEvent>(&pg.seed_field, "keydown", move |ev| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            if !Playground::is_running() {
                p.execute(p.run_mode.get());
            }
        }
    });
    let p = pg.clone();
    on::<Event>(&id::<HtmlElement>("seed-new"), "click", move |_| {
        let mut n = 1 + (js_sys::Math::random() * 999_999.0).floor() as u64;
        while Some(n) == p.seed() {
            n = 1 + (js_sys::Math::random() * 999_999.0).floor() as u64;
        }
        p.show_seed(Some(&n.to_string()));
        p.seed_changed();
        if !Playground::is_running() {
            p.execute(p.run_mode.get());
        }
    });
    // Escape closes the option and gives the focus back to its button; so does a click
    // elsewhere.
    let p = pg.clone();
    on::<KeyboardEvent>(&pg.seed_option, "keydown", move |ev| {
        if ev.key() == "Escape" && p.seed_option.open() {
            p.seed_option.set_open(false);
            if let Some(summary) = dom::query_in::<HtmlElement>(&p.seed_option, "summary") {
                let _ = summary.focus();
            }
        }
    });
    let p = pg.clone();
    on::<Event>(&dom::document(), "click", move |ev| {
        if p.seed_option.open() && !dom::holds(&p.seed_option, ev.target()) {
            p.seed_option.set_open(false);
        }
    });
}

fn stepper_events(pg: &Pg) {
    let p = pg.clone();
    on::<Event>(&pg.step_prev, "click", move |_| p.show_step(p.step_at.get() as i64 - 1));
    let p = pg.clone();
    on::<Event>(&pg.step_next, "click", move |_| p.show_step(p.step_at.get() as i64 + 1));
    let p = pg.clone();
    on::<Event>(&pg.step_range, "input", move |_| p.show_step(p.step_range.value().parse::<i64>().unwrap_or(1) - 1));
    // The keyboard, anywhere in the steps (the slider has these keys of its own).
    let p = pg.clone();
    on::<KeyboardEvent>(&pg.stepper, "keydown", move |ev| {
        let on_range = ev.target().is_some_and(|t| t == *p.step_range.unchecked_ref::<web_sys::EventTarget>());
        let count = p.steps.borrow().len() as i64;
        if on_range || ev.ctrl_key() || ev.meta_key() || ev.alt_key() || count == 0 {
            return;
        }
        let at = p.step_at.get() as i64;
        let to = match ev.key().as_str() {
            "ArrowLeft" => at - 1,
            "ArrowRight" => at + 1,
            "Home" => 0,
            "End" => count - 1,
            _ => return,
        };
        ev.prevent_default();
        p.show_step(to);
    });
}

fn help(pg: &Pg) {
    let topics: HtmlElement = id("topics");
    for topic in TOPICS {
        let Ok(button) = dom::document().create_element("button") else { continue };
        let _ = button.set_attribute("type", "button");
        button.set_class_name("topic");
        let _ = button.set_attribute("data-topic", topic);
        button.set_text_content(Some(topic));
        let _ = button.set_attribute("aria-pressed", "false");
        let p = pg.clone();
        on::<Event>(&button, "click", move |_| p.show_help(topic, false));
        let _ = topics.append_child(&button);
    }
    let p = pg.clone();
    on::<Event>(&id::<HtmlElement>("search-form"), "submit", move |ev| {
        ev.prevent_default();
        let query = id::<HtmlInputElement>("search").value();
        let query = query.trim();
        if !query.is_empty() {
            p.show_help(query, false);
        }
    });
}

fn sharing(pg: &Pg) {
    let p = pg.clone();
    on::<Event>(&pg.back, "click", move |_| {
        p.linked.set(false);
        p.set_note("", false);
        replace_address_without_hash();
        p.show_seed(dom::load(SAVED_SEED).as_deref());
        match dom::load(SAVED_CODE).filter(|s| !s.is_empty()) {
            Some(saved) => {
                *p.file_name.borrow_mut() = dom::load(SAVED_FILE).filter(|f| !f.is_empty()).unwrap_or_else(|| "main.ry".into());
                p.source.set_value(&saved);
                p.refresh_editor();
                p.set_custom();
                p.run_mode.set(RUN);
                p.execute(RUN);
            }
            None => {
                let example = dom::load(SAVED_EXAMPLE).filter(|e| e != "welcome" && p.has_example(e));
                p.load_example(example.as_deref().unwrap_or("welcome"));
            }
        }
        // The button is gone: the focus goes to the top of the playground.
        let _ = p.example.focus();
    });
    pg.share.set_hidden(false);
    pg.ai.set_hidden(false);
    let p = pg.clone();
    on::<Event>(&pg.share, "click", move |_| {
        let p = p.clone();
        wasm_bindgen_futures::spawn_local(async move { p.share_link().await });
    });
    let p = pg.clone();
    on::<Event>(&id::<HtmlElement>("share-copy"), "click", move |_| {
        let p = p.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let link = p.share_link.value();
            p.copy_link(&link).await;
        });
    });
    let p = pg.clone();
    on::<Event>(&id::<HtmlElement>("share-close"), "click", move |_| p.close_share());
    let p = pg.clone();
    on::<Event>(&pg.ai, "click", move |_| {
        let p = p.clone();
        wasm_bindgen_futures::spawn_local(async move { p.copy_for_ai().await });
    });
    // Escape closes the box, from the box or from the button that opened it.
    for el in [&pg.share_box, &pg.share, &pg.ai] {
        let p = pg.clone();
        on::<KeyboardEvent>(el, "keydown", move |ev| {
            if ev.key() == "Escape" && !p.share_box.hidden() {
                p.close_share();
            }
        });
    }
    let p = pg.clone();
    on::<Event>(&dom::window(), "hashchange", move |_| {
        let p = p.clone();
        wasm_bindgen_futures::spawn_local(async move { p.address_changed().await });
    });
}

/// The keyboard of a phone: while it is open (the visible part of the screen is much
/// shorter than without it, at the same width: iOS and Chrome shrink the visual
/// viewport, other browsers the window), the editor is no taller than the part of the
/// screen left above it (`--keyboard-free`, see site.css), so the line being typed
/// stays in sight.
fn keyboard_of_phones(pg: &Pg) {
    let Some(viewport) = dom::window().visual_viewport() else { return };
    let tallest = Rc::new(Cell::new((0.0_f64, 0.0_f64)));
    let fit = {
        let (p, v, tallest) = (pg.clone(), viewport.clone(), tallest.clone());
        Rc::new(move || {
            let typing = dom::active().is_some_and(|a| a == *p.source.unchecked_ref::<Element>());
            let (width, height) = (v.width(), v.height());
            let (mut tall_width, mut tall_height) = tallest.get();
            if width != tall_width {
                tall_width = width;
                tall_height = 0.0;
            }
            if !typing || height > tall_height {
                tall_height = tall_height.max(height);
            }
            tallest.set((tall_width, tall_height));
            let open = typing && height < tall_height * 0.8;
            let root = dom::root();
            let _ = root.class_list().toggle_with_force("keyboard", open);
            if open {
                let _ = root.style().set_property("--keyboard-free", &format!("{}px", height.round()));
            }
        })
    };
    fit();
    for (target, event) in [(viewport.unchecked_ref::<web_sys::EventTarget>(), "resize"), (&pg.source, "focus"), (&pg.source, "blur")] {
        let fit = fit.clone();
        on::<Event>(target, event, move |_| fit());
    }
}
