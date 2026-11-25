//! `rythme lsp`: the language server. Editors (VS Code, Neovim, Helix, Emacs...)
//! start it and talk to it with the Language Server Protocol, on its standard
//! input and output, to show help everywhere while code is being written:
//! problems as they are typed, documentation on hover, completion, the
//! parameters of a call, go to definition, references, renaming, an outline,
//! the types of names, colors that follow what names are, "Run" and "Run
//! test" above the code, the names of the whole workspace, folding.
//!
//! Messages are JSON, each preceded by a `Content-Length` header. The server
//! answers requests in order; a bug while handling one becomes an error
//! answer, never a dead server.

mod analysis;
mod extras;
mod features;
mod index;
pub mod json;
mod refs;
mod scan;
pub mod text;
mod workspace;

use std::collections::HashMap;
use std::io::{BufRead, IsTerminal, Read, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::rc::Rc;

use analysis::{Analysis, FileIds, Loader, Sources};
use features::Ctx;
use json::{obj, Json};
use text::{uri_to_path, Encoding, Pos};

use crate::check::Knowledge;
use crate::interp::Interp;
use crate::lang;

/// Error codes of JSON-RPC and of the protocol.
mod code {
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;
    pub const SERVER_NOT_INITIALIZED: i64 = -32002;
    pub const REQUEST_FAILED: i64 = -32803;
}

/// Bigger messages are refused (and skipped).
const MAX_MESSAGE: usize = 64 << 20;

pub struct RpcError {
    code: i64,
    message: String,
}

impl RpcError {
    fn new(code: i64, message: impl Into<String>) -> RpcError {
        RpcError { code, message: message.into() }
    }

    fn failed(message: impl Into<String>) -> RpcError {
        RpcError::new(code::REQUEST_FAILED, message)
    }
}

type Reply = Result<Json, RpcError>;

pub struct Doc {
    uri: String,
    /// Canonical path, for documents that are files.
    path: Option<PathBuf>,
    text: Rc<str>,
    version: Option<i64>,
    analysis: Option<Analysis>,
    /// The last semantic tokens sent, and their id (for `full/delta`).
    tokens: Option<(String, Vec<u32>)>,
}

pub struct Server {
    out: Box<dyn Write>,
    initialized: bool,
    shutdown: bool,
    cx: Ctx,
    prepare_rename: bool,
    /// The language asked on the command line (`--lang`).
    chosen_lang: Option<lang::Lang>,
    /// The language of the editor, if it is English or French.
    editor_lang: Option<lang::Lang>,
    roots: Vec<PathBuf>,
    ids: FileIds,
    base: Knowledge,
    docs: HashMap<String, Doc>,
    /// The names of the files of the workspace that are not open.
    symbols: workspace::Cache,
    /// Ids of the semantic tokens sent.
    next_tokens: u64,
}

/// Runs the server on the standard input and output. `chosen` is the
/// language asked with `--lang`. Returns the exit code.
pub fn run(args: &[String], mut chosen: Option<lang::Lang>) -> i32 {
    let mut args = args.iter();
    while let Some(a) = args.next() {
        match a.as_str() {
            // Added by some editors; standard input and output are the only transport.
            "--stdio" => {}
            "--lang" => {
                chosen = args.next().and_then(|v| lang::parse(v)).or(chosen);
                lang::set(chosen.unwrap_or_else(lang::get));
            }
            "--help" | "-h" => {
                crate::io::out_line(crate::help::topic("editors").unwrap_or_default().trim_end());
                return 0;
            }
            other => {
                eprintln!("{}", t!("rythme lsp: unknown option `{other}` (ignored)", "rythme lsp : option inconnue `{other}` (ignorée)"))
            }
        }
    }
    if std::io::stdin().is_terminal() {
        eprintln!(
            "{}",
            t!(
                "rythme lsp is the language server: your editor starts it and talks to it. See `rythme help editors`. (Ctrl-C to quit)",
                "rythme lsp est le serveur de langage : votre éditeur le lance et dialogue avec lui. Voir `rythme help editors`. (Ctrl-C pour quitter)"
            )
        );
    }
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut server = Server::new(Box::new(std::io::stdout()));
    server.chosen_lang = chosen;
    loop {
        match read_message(&mut input) {
            Ok(Some(body)) => {
                if let Some(code) = server.handle_bytes(&body) {
                    return code;
                }
            }
            // The editor went away without `exit`.
            Ok(None) => return if server.shutdown { 0 } else { 1 },
            Err(e) => eprintln!("rythme lsp: {e}"),
        }
    }
}

/// Reads one message: headers, an empty line, then `Content-Length` bytes.
/// `Ok(None)` at the end of the input.
pub fn read_message(input: &mut impl BufRead) -> Result<Option<Vec<u8>>, String> {
    let mut length: Option<usize> = None;
    let mut any = false;
    loop {
        let mut line = Vec::new();
        let n = input.by_ref().take(8192).read_until(b'\n', &mut line).map_err(|e| e.to_string())?;
        if n == 0 {
            return if any { Err("the input ended in the middle of a message".into()) } else { Ok(None) };
        }
        any = true;
        let line = String::from_utf8_lossy(&line);
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("content-length") {
                length = value.trim().parse().ok();
            }
        }
    }
    let Some(length) = length else { return Err("a message without a valid Content-Length header was skipped".into()) };
    if length > MAX_MESSAGE {
        std::io::copy(&mut input.by_ref().take(length as u64), &mut std::io::sink()).map_err(|e| e.to_string())?;
        return Err(format!("a message of {length} bytes is too big and was skipped"));
    }
    let mut body = vec![0; length];
    match input.read_exact(&mut body) {
        Ok(()) => Ok(Some(body)),
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

fn panic_message(e: &Box<dyn std::any::Any + Send>) -> String {
    e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap_or_default()
}

fn uint(v: &Json) -> u32 {
    v.as_u64().map_or(0, |n| n.min(u32::MAX as u64) as u32)
}

impl Server {
    pub fn new(out: Box<dyn Write>) -> Server {
        Server {
            out,
            initialized: false,
            shutdown: false,
            cx: Ctx { enc: Encoding::Utf16, markdown: true, snippets: false, hierarchical_symbols: false },
            prepare_rename: false,
            chosen_lang: None,
            editor_lang: None,
            roots: Vec::new(),
            ids: FileIds::default(),
            base: Knowledge::from_interp(&Interp::new()),
            docs: HashMap::new(),
            symbols: workspace::Cache::default(),
            next_tokens: 0,
        }
    }

    // ----- sending ------------------------------------------------------

    fn send(&mut self, msg: &Json) {
        let body = msg.to_string();
        let framed = format!("Content-Length: {}\r\n\r\n{body}", body.len());
        // If the editor went away, the next read ends the server.
        let _ = self.out.write_all(framed.as_bytes()).and_then(|_| self.out.flush());
    }

    fn respond(&mut self, id: Json, reply: Reply) {
        let msg = match reply {
            Ok(result) => obj([("jsonrpc", "2.0".into()), ("id", id), ("result", result)]),
            Err(e) => {
                obj([("jsonrpc", "2.0".into()), ("id", id), ("error", obj([("code", e.code.into()), ("message", e.message.into())]))])
            }
        };
        self.send(&msg);
    }

    fn notify(&mut self, method: &str, params: Json) {
        self.send(&obj([("jsonrpc", "2.0".into()), ("method", method.into()), ("params", params)]));
    }

    fn log_error(&mut self, message: String) {
        eprintln!("rythme lsp: {message}");
        self.notify("window/logMessage", obj([("type", 1u32.into()), ("message", message.into())]));
    }

    // ----- receiving ----------------------------------------------------

    /// Handles one message. Returns the exit code when the server must stop.
    pub fn handle_bytes(&mut self, body: &[u8]) -> Option<i32> {
        let parsed = std::str::from_utf8(body).map_err(|e| e.to_string()).and_then(Json::parse);
        let msg = match parsed {
            Ok(m) => m,
            Err(e) => {
                self.respond(Json::Null, Err(RpcError::new(code::PARSE_ERROR, format!("invalid JSON: {e}"))));
                return None;
            }
        };
        if !matches!(msg, Json::Obj(_)) {
            self.respond(Json::Null, Err(RpcError::new(code::INVALID_REQUEST, "a message must be a JSON object")));
            return None;
        }
        let id = msg.get("id").clone();
        let Some(method) = msg.get("method").as_str().map(str::to_string) else {
            // An answer to a request of the server: none are sent, nothing to do.
            return None;
        };
        let params = msg.get("params").clone();
        let is_request = !matches!(msg.get("id"), Json::Null);
        if !is_request {
            if method == "exit" {
                return Some(if self.shutdown { 0 } else { 1 });
            }
            if self.initialized && !self.shutdown {
                let r = catch_unwind(AssertUnwindSafe(|| self.notification(&method, &params)));
                if let Err(e) = r {
                    let m = panic_message(&e);
                    self.log_error(format!("internal error while handling `{method}`: {m}"));
                }
            }
            return None;
        }
        let reply = if method != "initialize" && !self.initialized {
            Err(RpcError::new(
                code::SERVER_NOT_INITIALIZED,
                t!("the server is not initialized yet", "le serveur n'est pas encore initialisé"),
            ))
        } else if self.shutdown {
            Err(RpcError::new(code::INVALID_REQUEST, t!("the server is shutting down", "le serveur est en train de s'arrêter")))
        } else {
            match catch_unwind(AssertUnwindSafe(|| self.request(&method, &params))) {
                Ok(r) => r,
                Err(e) => Err(RpcError::new(
                    code::INTERNAL_ERROR,
                    t!(
                        "internal error in Rythme (please report it): {}",
                        "erreur interne de Rythme (merci de la signaler) : {}",
                        panic_message(&e)
                    ),
                )),
            }
        };
        self.respond(id, reply);
        None
    }

    fn request(&mut self, method: &str, params: &Json) -> Reply {
        match method {
            "initialize" => self.initialize(params),
            "shutdown" => {
                self.shutdown = true;
                Ok(Json::Null)
            }
            "textDocument/hover" => self.at(params, features::hover),
            "textDocument/completion" => {
                let trigger = params.get_path(&["context", "triggerCharacter"]).as_str().map(str::to_string);
                self.at(params, |cx, a, offset| features::completion(cx, a, offset, trigger.as_deref()))
            }
            "textDocument/signatureHelp" => self.at(params, features::signature_help),
            "textDocument/definition" => self.at(params, features::definition),
            "textDocument/documentHighlight" => self.at(params, features::highlights),
            "textDocument/documentSymbol" => {
                let cx = self.cx;
                Ok(self.analysis(params).map_or(Json::Null, |a| features::document_symbols(&cx, a)))
            }
            "textDocument/codeAction" => {
                let cx = self.cx;
                let Some(a) = self.analysis(params) else { return Ok(Json::Null) };
                let range = params.get("range");
                let start = a.lines.offset(&a.text, pos_of(range.get("start")), cx.enc);
                let end = a.lines.offset(&a.text, pos_of(range.get("end")), cx.enc);
                Ok(features::code_actions(&cx, a, start, end))
            }
            "textDocument/references" => {
                let include = params.get_path(&["context", "includeDeclaration"]).as_bool().unwrap_or(true);
                let Some((uri, offset)) = self.position(params) else { return Ok(Json::Null) };
                refs::references(self, &uri, offset, include)
            }
            "textDocument/prepareRename" => {
                let Some((uri, offset)) = self.position(params) else { return Ok(Json::Null) };
                refs::prepare_rename(self, &uri, offset)
            }
            "textDocument/rename" => {
                let Some(new_name) = params.get("newName").as_str() else {
                    return Err(RpcError::new(code::INVALID_PARAMS, "newName is missing"));
                };
                let Some((uri, offset)) = self.position(params) else { return Ok(Json::Null) };
                refs::rename(self, &uri, offset, new_name)
            }
            "textDocument/formatting" => self.formatting(params),
            "textDocument/inlayHint" => {
                let cx = self.cx;
                let Some(a) = self.analysis(params) else { return Ok(Json::Null) };
                let range = params.get("range");
                let start = a.lines.offset(&a.text, pos_of(range.get("start")), cx.enc);
                let end = a.lines.offset(&a.text, pos_of(range.get("end")), cx.enc);
                Ok(extras::inlay_hints(&cx, a, start, end))
            }
            "textDocument/semanticTokens/full" => Ok(self.semantic_tokens(params, None)),
            "textDocument/semanticTokens/full/delta" => {
                let previous = params.get("previousResultId").as_str().map(str::to_string);
                Ok(self.semantic_tokens(params, previous.as_deref()))
            }
            "textDocument/semanticTokens/range" => {
                let cx = self.cx;
                let Some(a) = self.analysis(params) else { return Ok(Json::Null) };
                let range = params.get("range");
                let data = extras::tokens_in_lines(
                    &extras::semantic_tokens(&cx, a),
                    uint(range.get_path(&["start", "line"])),
                    uint(range.get_path(&["end", "line"])),
                );
                Ok(obj([("data", Json::Arr(data.into_iter().map(Json::from).collect()))]))
            }
            "textDocument/codeLens" => {
                let cx = self.cx;
                Ok(self.analysis(params).map_or(Json::Null, |a| extras::code_lenses(&cx, a)))
            }
            "textDocument/foldingRange" => Ok(self.analysis(params).map_or(Json::Null, extras::folding_ranges)),
            "textDocument/selectionRange" => {
                let cx = self.cx;
                let Some(a) = self.analysis(params) else { return Ok(Json::Null) };
                let offsets: Vec<usize> =
                    params.get("positions").as_array().iter().map(|p| a.lines.offset(&a.text, pos_of(p), cx.enc)).collect();
                Ok(extras::selection_ranges(&cx, a, &offsets))
            }
            "workspace/symbol" => Ok(self.workspace_symbols(params.get("query").as_str().unwrap_or(""))),
            _ => Err(RpcError::new(code::METHOD_NOT_FOUND, format!("unknown method `{method}`"))),
        }
    }

    fn notification(&mut self, method: &str, params: &Json) {
        match method {
            "textDocument/didOpen" => {
                let td = params.get("textDocument");
                let Some(uri) = td.get("uri").as_str() else { return };
                let text: Rc<str> = Rc::from(td.get("text").as_str().unwrap_or(""));
                let path = uri_to_path(uri).map(|p| text::canonical(&p));
                let doc = Doc { uri: uri.to_string(), path, text, version: td.get("version").as_i64(), analysis: None, tokens: None };
                self.docs.insert(uri.to_string(), doc);
                self.refresh(uri);
            }
            "textDocument/didChange" => {
                let uri = params.get_path(&["textDocument", "uri"]).as_str().unwrap_or("").to_string();
                let enc = self.cx.enc;
                let Some(doc) = self.docs.get_mut(&uri) else { return };
                let mut text = doc.text.to_string();
                for change in params.get("contentChanges").as_array() {
                    let new = change.get("text").as_str().unwrap_or("");
                    let range = change.get("range");
                    if range.is_null() {
                        text = new.to_string();
                    } else {
                        let lines = text::LineIndex::new(&text);
                        let start = lines.offset(&text, pos_of(range.get("start")), enc);
                        let end = lines.offset(&text, pos_of(range.get("end")), enc).max(start);
                        text.replace_range(start..end, new);
                    }
                }
                doc.text = Rc::from(text);
                doc.version = params.get_path(&["textDocument", "version"]).as_i64();
                self.refresh(&uri);
            }
            "textDocument/didClose" => {
                let Some(uri) = params.get_path(&["textDocument", "uri"]).as_str() else { return };
                if let Some(doc) = self.docs.remove(uri) {
                    self.notify("textDocument/publishDiagnostics", obj([("uri", uri.into()), ("diagnostics", Json::Arr(Vec::new()))]));
                    // Files that use it now read it from the disk.
                    if let Some(path) = doc.path {
                        self.refresh_users(&path, uri);
                    }
                }
            }
            // Saved files, or files changed outside of the editor: the files
            // that use them may see something else now.
            "textDocument/didSave" | "workspace/didChangeWatchedFiles" => self.refresh_all(),
            "workspace/didChangeConfiguration" => {
                let l = params.get_path(&["settings", "rythme", "language"]).as_str();
                if let Some(l) = l {
                    lang::set(lang::parse(l).or(self.chosen_lang).or(env_lang()).or(self.editor_lang).unwrap_or_else(lang::detect));
                    self.refresh_all();
                }
            }
            _ => {}
        }
    }

    // ----- start-up ------------------------------------------------------

    fn initialize(&mut self, params: &Json) -> Reply {
        if self.initialized {
            return Err(RpcError::new(code::INVALID_REQUEST, "initialize was already received"));
        }
        let caps = params.get("capabilities");
        // The first encoding the editor offers; UTF-16 when it offers none.
        let offered = caps.get_path(&["general", "positionEncodings"]).as_array();
        self.cx.enc = offered.iter().find_map(|e| e.as_str().and_then(Encoding::parse)).unwrap_or(Encoding::Utf16);
        let formats = caps.get_path(&["textDocument", "hover", "contentFormat"]).as_array();
        self.cx.markdown = formats.is_empty() || formats.iter().any(|f| f.as_str() == Some("markdown"));
        self.cx.snippets = caps.get_path(&["textDocument", "completion", "completionItem", "snippetSupport"]).as_bool() == Some(true);
        self.cx.hierarchical_symbols =
            caps.get_path(&["textDocument", "documentSymbol", "hierarchicalDocumentSymbolSupport"]).as_bool() == Some(true);
        self.prepare_rename = caps.get_path(&["textDocument", "rename", "prepareSupport"]).as_bool() == Some(true);

        // The language of the messages: the editor's setting, then `--lang`,
        // then RYTHME_LANG, then the language of the editor, then the system's.
        let setting = params.get_path(&["initializationOptions", "language"]).as_str().and_then(lang::parse);
        self.editor_lang = params.get("locale").as_str().and_then(lang::parse);
        lang::set(setting.or(self.chosen_lang).or(env_lang()).or(self.editor_lang).unwrap_or_else(lang::detect));

        for f in params.get("workspaceFolders").as_array() {
            self.roots.extend(f.get("uri").as_str().and_then(uri_to_path));
        }
        if self.roots.is_empty() {
            let root = params.get("rootUri").as_str().and_then(uri_to_path).or_else(|| params.get("rootPath").as_str().map(PathBuf::from));
            self.roots.extend(root);
        }
        self.initialized = true;

        let caps = obj([
            ("positionEncoding", self.cx.enc.name().into()),
            (
                "textDocumentSync",
                obj([("openClose", true.into()), ("change", 1u32.into()), ("save", obj([("includeText", false.into())]))]),
            ),
            ("hoverProvider", true.into()),
            ("completionProvider", obj([("triggerCharacters", Json::Arr(vec![".".into(), "\"".into(), "/".into()]))])),
            (
                "signatureHelpProvider",
                // A space: the arguments of a call without parentheses (`puts `).
                obj([
                    ("triggerCharacters", Json::Arr(vec!["(".into(), ",".into(), " ".into()])),
                    ("retriggerCharacters", Json::Arr(vec![",".into()])),
                ]),
            ),
            ("definitionProvider", true.into()),
            ("referencesProvider", true.into()),
            ("documentHighlightProvider", true.into()),
            ("documentSymbolProvider", true.into()),
            ("codeActionProvider", obj([("codeActionKinds", Json::Arr(vec!["quickfix".into()]))])),
            ("renameProvider", if self.prepare_rename { obj([("prepareProvider", true.into())]) } else { true.into() }),
            ("documentFormattingProvider", true.into()),
            ("inlayHintProvider", true.into()),
            (
                "semanticTokensProvider",
                obj([
                    (
                        "legend",
                        obj([
                            ("tokenTypes", Json::Arr(extras::TOKEN_TYPES.iter().map(|t| (*t).into()).collect())),
                            ("tokenModifiers", Json::Arr(extras::TOKEN_MODIFIERS.iter().map(|t| (*t).into()).collect())),
                        ]),
                    ),
                    ("full", obj([("delta", true.into())])),
                    ("range", true.into()),
                ]),
            ),
            ("codeLensProvider", obj([("resolveProvider", false.into())])),
            ("foldingRangeProvider", true.into()),
            ("selectionRangeProvider", true.into()),
            ("workspaceSymbolProvider", true.into()),
        ]);
        Ok(obj([("capabilities", caps), ("serverInfo", obj([("name", "rythme".into()), ("version", crate::VERSION.into())]))]))
    }

    // ----- documents -----------------------------------------------------

    /// The open documents that are files: path → (address, text).
    fn open_files(&self) -> HashMap<PathBuf, (String, Rc<str>)> {
        self.docs.values().filter_map(|d| Some((d.path.clone()?, (d.uri.clone(), d.text.clone())))).collect()
    }

    /// The folder of `use` in documents that are not saved files.
    fn root(&self) -> PathBuf {
        self.roots.first().cloned().or_else(|| std::env::current_dir().ok()).unwrap_or_default()
    }

    /// Analyzes a file, open or not, with `overlay` texts replacing some files.
    fn load(
        &mut self,
        purpose: &str,
        path: Option<&std::path::Path>,
        uri: &str,
        text: Rc<str>,
        overlay: &HashMap<PathBuf, String>,
    ) -> Analysis {
        let open = self.open_files();
        let root = self.root();
        let mut loader = Loader::new(&mut self.ids, purpose, Sources { open: &open, overlay }, &self.base, root);
        let module = loader.load(path, uri, &text);
        Analysis::new(loader, module, text)
    }

    fn analyze(&mut self, uri: &str) {
        let Some(doc) = self.docs.get(uri) else { return };
        let (path, text) = (doc.path.clone(), doc.text.clone());
        let analysis = analysis::guard(|| self.load("", path.as_deref(), uri, text, &HashMap::new()));
        if analysis.is_none() {
            self.log_error(format!("internal error while reading {uri}"));
        }
        if let Some(doc) = self.docs.get_mut(uri) {
            doc.analysis = analysis;
        }
    }

    fn publish(&mut self, uri: &str) {
        let cx = self.cx;
        let Some(doc) = self.docs.get(uri) else { return };
        let Some(a) = &doc.analysis else { return };
        let mut params = obj([("uri", uri.into()), ("diagnostics", features::diagnostics_json(&cx, a))]);
        if let Some(v) = doc.version {
            params.set("version", v.into());
        }
        self.notify("textDocument/publishDiagnostics", params);
    }

    /// Analyzes a document again, then the open documents that use it.
    fn refresh(&mut self, uri: &str) {
        self.analyze(uri);
        self.publish(uri);
        if let Some(path) = self.docs.get(uri).and_then(|d| d.path.clone()) {
            self.refresh_users(&path, uri);
        }
    }

    fn refresh_users(&mut self, path: &std::path::Path, except: &str) {
        let users: Vec<String> = self
            .docs
            .values()
            .filter(|d| d.uri != except && d.analysis.as_ref().is_some_and(|a| a.modules.contains_key(path)))
            .map(|d| d.uri.clone())
            .collect();
        for u in users {
            self.analyze(&u);
            self.publish(&u);
        }
    }

    fn refresh_all(&mut self) {
        let uris: Vec<String> = self.docs.keys().cloned().collect();
        for u in uris {
            self.analyze(&u);
            self.publish(&u);
        }
    }

    fn analysis(&self, params: &Json) -> Option<&Analysis> {
        let uri = params.get_path(&["textDocument", "uri"]).as_str()?;
        self.docs.get(uri)?.analysis.as_ref()
    }

    /// The document and byte offset of `textDocument` and `position`.
    fn position(&self, params: &Json) -> Option<(String, usize)> {
        let a = self.analysis(params)?;
        let offset = a.lines.offset(&a.text, pos_of(params.get("position")), self.cx.enc);
        Some((a.uri.clone(), offset))
    }

    /// Runs a feature at the position given in `params` (`null` for unknown documents).
    fn at(&mut self, params: &Json, f: impl Fn(&Ctx, &Analysis, usize) -> Json) -> Reply {
        let cx = self.cx;
        let Some(a) = self.analysis(params) else { return Ok(Json::Null) };
        let offset = a.lines.offset(&a.text, pos_of(params.get("position")), cx.enc);
        Ok(f(&cx, a, offset))
    }

    /// `textDocument/formatting`: one edit replacing the whole document by its
    /// formatted text (`rythme fmt`). Code with a syntax error is left as it is.
    fn formatting(&mut self, params: &Json) -> Reply {
        let cx = self.cx;
        let Some(a) = self.analysis(params) else { return Ok(Json::Null) };
        match crate::format::format(&a.text) {
            Err(errors) => {
                let first = errors.first().map(|d| d.message.clone()).unwrap_or_default();
                Err(RpcError::failed(t!(
                    "not formatted: the code has an error ({first})",
                    "pas mis en forme : le code a une erreur ({first})"
                )))
            }
            Ok(new) if new == *a.text => Ok(Json::Arr(Vec::new())),
            Ok(new) => {
                let range = features::range_in(&a.text, &a.lines, 0, a.text.len(), cx.enc);
                Ok(Json::Arr(vec![obj([("range", range), ("newText", new.into())])]))
            }
        }
    }
}

impl Server {
    /// `semanticTokens/full`, or `full/delta` from the tokens of `previous`.
    fn semantic_tokens(&mut self, params: &Json, previous: Option<&str>) -> Json {
        let cx = self.cx;
        let Some(uri) = params.get_path(&["textDocument", "uri"]).as_str() else { return Json::Null };
        let Some(doc) = self.docs.get_mut(uri) else { return Json::Null };
        let Some(a) = &doc.analysis else { return Json::Null };
        let data = extras::semantic_tokens(&cx, a);
        self.next_tokens += 1;
        let id = self.next_tokens.to_string();
        let old = doc.tokens.replace((id.clone(), data.clone()));
        match (previous, old) {
            (Some(p), Some((old_id, old_data))) if p == old_id => {
                obj([("resultId", id.into()), ("edits", Json::Arr(extras::tokens_edit(&old_data, &data)))])
            }
            _ => obj([("resultId", id.into()), ("data", Json::Arr(data.into_iter().map(Json::from).collect()))]),
        }
    }

    /// `workspace/symbol`: the names of the open documents and of the files of the workspace.
    fn workspace_symbols(&mut self, query: &str) -> Json {
        let enc = self.cx.enc;
        let mut open = HashMap::new();
        let mut others = Vec::new();
        for d in self.docs.values() {
            let Some(a) = &d.analysis else { continue };
            let syms = workspace::symbols_of(a.index(), &d.uri, &d.text, enc);
            match &d.path {
                Some(p) => {
                    open.insert(p.clone(), syms);
                }
                None => others.extend(syms),
            }
        }
        let all = self.symbols.all(&mut self.ids, &self.roots, &open, others, enc);
        workspace::search(all, query)
    }
}

fn pos_of(p: &Json) -> Pos {
    Pos { line: uint(p.get("line")), character: uint(p.get("character")) }
}

fn env_lang() -> Option<lang::Lang> {
    std::env::var("RYTHME_LANG").ok().as_deref().and_then(lang::parse)
}
