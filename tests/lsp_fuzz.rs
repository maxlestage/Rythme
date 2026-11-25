//! Fuzzing the language server: random edits (incomplete code, characters of
//! several bytes, positions past the end of a line or inside a character),
//! each followed by random requests. The server must never fail inside: no
//! "internal error" answer, no error logged while handling a notification.
//!
//! A longer run, in release mode:
//!
//! ```sh
//! RYTHME_LSP_FUZZ_CASES=20000 cargo test --release --test lsp_fuzz
//! ```

use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use rythme::lsp::json::Json;
use rythme::lsp::Server;

struct Rng(u64);

impl Rng {
    /// splitmix64.
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    fn pick<'a>(&mut self, items: &[&'a str]) -> &'a str {
        items[self.below(items.len())]
    }
}

/// What the server writes, kept to be read by the test.
#[derive(Clone, Default)]
struct Out(Rc<RefCell<Vec<u8>>>);

impl Write for Out {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Pieces of code typed at random places: incomplete constructs, characters
/// of 2, 3 and 4 bytes (and of 2 UTF-16 units), line breaks of every kind.
const PIECES: &[&str] = &[
    "é",
    "👍🏽",
    "日本",
    "ß",
    "e\u{301}",
    "\r\n",
    "\n",
    "\r",
    "\t",
    " ",
    "fn ",
    "fn f(x) = ",
    "(",
    ")",
    "[",
    "]",
    "{",
    "}",
    "\"",
    "\"{",
    "#",
    "## doc\n",
    "|>",
    ".",
    ",",
    "=",
    "->",
    "match x\n",
    "  [a, ..t] -> ",
    "end",
    "do\n",
    "if ",
    " then ",
    " else ",
    "type Tree(a) = Leaf | Node(l: Tree(a), v: a, r: Tree(a))\n",
    "type ",
    "(a)",
    "use \"",
    "use \"lib\"\n",
    "lib.",
    "test \"t\" = ",
    "x: Int",
    ": List(",
    "zip(",
    "zip([1], [\"a\"], [true])",
    "format(\"{}\", ",
    "word_wrap(\"héllo wörld\", 3)",
    "center(\"日本\", 9, \"·\")",
    "edit_distance(\"é\", \"👍\")",
    "snake_case(\"HéllöWörld\")",
    "print(",
    "map",
    "xs",
    "x",
    "1",
    "-",
    "1.5",
    "Some(",
    "Ok(1)",
    "?",
    "|v|",
    ":ok",
];

const SEED: &str = "## Adds one.\nfn add(x: Int) -> Int = x + 1\n\ntype Tree(a) = Leaf | Node(l: Tree(a), v: a, r: Tree(a))\n\nxs = [1, 2, 3] |> map(add)\ns = \"héllo 👍🏽 日本\"\nprint(\"{s} {xs}\")\nmatch xs\n  [a, ..rest] -> print(a)\n  [] -> print(0)\nend\ntest \"add\" = assert_eq(add(1), 2)\n";

const ENCODINGS: &[&str] = &["utf-8", "utf-16", "utf-32"];

fn pos(rng: &mut Rng, text: &str) -> Json {
    let lines = text.split('\n').count();
    let line = rng.below(lines + 2);
    let width = text.split('\n').nth(line).map_or(3, |l| l.len() + 3);
    let character = if rng.below(20) == 0 { u32::MAX as usize } else { rng.below(width) };
    obj(&[("line", Json::from(line)), ("character", Json::from(character))])
}

fn range(rng: &mut Rng, text: &str) -> Json {
    obj(&[("start", pos(rng, text)), ("end", pos(rng, text))])
}

fn obj(fields: &[(&str, Json)]) -> Json {
    Json::Obj(fields.iter().map(|(k, v)| (k.to_string(), v.clone())).collect())
}

fn doc_id(uri: &str) -> Json {
    obj(&[("uri", Json::Str(uri.to_string()))])
}

struct Session {
    server: Server,
    out: Out,
    next_id: u64,
    /// Answers with something in them: the requests reach the features.
    answers: usize,
}

impl Session {
    fn send(&mut self, method: &str, params: Json, request: bool) -> Result<(), String> {
        let mut fields = vec![("jsonrpc", Json::Str("2.0".into())), ("method", Json::Str(method.into())), ("params", params)];
        if request {
            self.next_id += 1;
            fields.push(("id", Json::from(self.next_id as usize)));
        }
        let body = obj(&fields).to_string();
        self.out.0.borrow_mut().clear();
        self.server.handle_bytes(body.as_bytes());
        let written = String::from_utf8_lossy(&self.out.0.borrow()).to_string();
        if written.contains("internal error") {
            return Err(format!("{method}: {body}\n=> {written}"));
        }
        if written.contains("\"result\":{") || written.contains("\"result\":[{") {
            self.answers += 1;
        }
        Ok(())
    }
}

/// One editing session: returns the first internal error, with the edits that led to it.
fn session(seed: u64, steps: usize) -> Result<usize, String> {
    let mut rng = Rng(seed);
    let enc = rng.pick(ENCODINGS);
    let out = Out::default();
    let mut s = Session { server: Server::new(Box::new(out.clone())), out, next_id: 0, answers: 0 };
    let caps = obj(&[(
        "capabilities",
        obj(&[
            ("general", obj(&[("positionEncodings", Json::Arr(vec![Json::Str(enc.into())]))])),
            ("textDocument", obj(&[("rename", obj(&[("prepareSupport", Json::Bool(true))]))])),
        ]),
    )]);
    s.send("initialize", caps, true)?;
    s.send("initialized", obj(&[]), false)?;
    let dir = std::env::temp_dir().join("rythme-lsp-fuzz-does-not-exist");
    let uri = rythme::lsp::text::path_to_uri(&dir.join("main.ry"));
    let lib = rythme::lsp::text::path_to_uri(&dir.join("lib.ry"));
    let mut text = SEED.to_string();
    let mut log = format!("encoding {enc}\n");
    let open = |uri: &str, text: &str| {
        obj(&[(
            "textDocument",
            obj(&[
                ("uri", Json::Str(uri.into())),
                ("languageId", Json::Str("rythme".into())),
                ("version", Json::from(1usize)),
                ("text", Json::Str(text.into())),
            ]),
        )])
    };
    s.send("textDocument/didOpen", open(&lib, "## A tree.\ntype Tree(a) = Leaf | Node(v: a)\nfn double(x) = x * 2\n"), false)?;
    s.send("textDocument/didOpen", open(&uri, &text), false)?;
    for version in 2..steps as u64 + 2 {
        // An edit: a piece typed somewhere, a range deleted, or both.
        let r = range(&mut rng, &text);
        let mut new = String::new();
        for _ in 0..rng.below(3) {
            new.push_str(rng.pick(PIECES));
        }
        let change = obj(&[("range", r.clone()), ("text", Json::Str(new.clone()))]);
        log.push_str(&format!("change {r} {new:?}\n"));
        let params = obj(&[
            ("textDocument", obj(&[("uri", Json::Str(uri.clone())), ("version", Json::from(version as usize))])),
            ("contentChanges", Json::Arr(vec![change])),
        ]);
        s.send("textDocument/didChange", params, false).map_err(|e| format!("{log}{e}"))?;
        // Keep the text the server has, the way it applies the change.
        text = apply(&text, &r, &new, enc);
        for _ in 0..1 + rng.below(3) {
            let (method, params) = request(&mut rng, &uri, &text);
            log.push_str(&format!("{method} {params}\n"));
            s.send(method, params, true).map_err(|e| format!("{log}{e}\ntext: {text:?}"))?;
        }
    }
    Ok(s.answers)
}

fn request(rng: &mut Rng, uri: &str, text: &str) -> (&'static str, Json) {
    let at = |rng: &mut Rng| obj(&[("textDocument", doc_id(uri)), ("position", pos(rng, text))]);
    let in_range = |rng: &mut Rng| obj(&[("textDocument", doc_id(uri)), ("range", range(rng, text))]);
    let whole = || obj(&[("textDocument", doc_id(uri))]);
    match rng.below(19) {
        0 => ("textDocument/hover", at(rng)),
        1 | 2 => ("textDocument/completion", at(rng)),
        3 => ("textDocument/signatureHelp", at(rng)),
        4 => ("textDocument/definition", at(rng)),
        5 => ("textDocument/documentHighlight", at(rng)),
        6 => ("textDocument/documentSymbol", whole()),
        7 => {
            let mut p = in_range(rng);
            if let Json::Obj(fields) = &mut p {
                fields.push(("context".into(), obj(&[("diagnostics", Json::Arr(Vec::new()))])));
            }
            ("textDocument/codeAction", p)
        }
        8 => {
            let mut p = at(rng);
            if let Json::Obj(fields) = &mut p {
                fields.push(("context".into(), obj(&[("includeDeclaration", Json::Bool(true))])));
            }
            ("textDocument/references", p)
        }
        9 => ("textDocument/prepareRename", at(rng)),
        10 => {
            let mut p = at(rng);
            let name = rng.pick(&["y", "é", "new_name", "fn", "", "日本", "a b"]);
            if let Json::Obj(fields) = &mut p {
                fields.push(("newName".into(), Json::Str(name.into())));
            }
            ("textDocument/rename", p)
        }
        11 => ("textDocument/formatting", obj(&[("textDocument", doc_id(uri)), ("options", obj(&[("tabSize", Json::from(2usize))]))])),
        12 => ("textDocument/inlayHint", in_range(rng)),
        13 => ("textDocument/semanticTokens/full", whole()),
        14 => ("textDocument/semanticTokens/range", in_range(rng)),
        15 => ("textDocument/codeLens", whole()),
        16 => ("textDocument/foldingRange", whole()),
        17 => {
            let positions = (0..1 + rng.below(3)).map(|_| pos(rng, text)).collect();
            ("textDocument/selectionRange", obj(&[("textDocument", doc_id(uri)), ("positions", Json::Arr(positions))]))
        }
        _ => ("workspace/symbol", obj(&[("query", Json::Str(rng.pick(&["", "a", "é", "Tree"]).into()))])),
    }
}

/// The text after a change, as the protocol says: positions past the end of
/// a line mean its end, inside a character mean after it.
fn apply(text: &str, r: &Json, new: &str, enc: &str) -> String {
    let lines = rythme::lsp::text::LineIndex::new(text);
    let enc = rythme::lsp::text::Encoding::parse(enc).unwrap();
    let p = |j: &Json| rythme::lsp::text::Pos {
        line: j.get("line").as_u64().unwrap_or(0).min(u32::MAX as u64) as u32,
        character: j.get("character").as_u64().unwrap_or(0).min(u32::MAX as u64) as u32,
    };
    let start = lines.offset(text, p(r.get("start")), enc);
    let end = lines.offset(text, p(r.get("end")), enc).max(start);
    let mut t = text.to_string();
    t.replace_range(start..end, new);
    t
}

#[test]
fn the_language_server_never_fails_inside() {
    rythme::lang::set(rythme::lang::Lang::En);
    let cases: u64 = std::env::var("RYTHME_LSP_FUZZ_CASES").ok().and_then(|s| s.parse().ok()).unwrap_or(150);
    let (mut failures, mut answers) = (Vec::new(), 0);
    for i in 0..cases {
        match session(0x4C53_5000 + i, 25) {
            Ok(n) => answers += n,
            Err(e) => failures.push(e),
        }
    }
    failures.sort_by_key(String::len);
    assert!(failures.is_empty(), "{} failing session(s); the shortest:\n{}", failures.len(), failures[0]);
    assert!(answers > cases as usize * 10, "only {answers} answers with something in them");
}
