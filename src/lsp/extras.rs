//! More help in the editor: the types the checker is sure of, shown next to
//! the names (inlay hints); colors that follow what each name is (semantic
//! tokens); "Run" and "Run test" above the code (code lenses); the names of
//! every file of the workspace; folding; and growing a selection.

use std::collections::{HashMap, HashSet};

use super::analysis::Analysis;
use super::features::{pos_json, range_in, Ctx};
use super::index::{Def, DefKind, Index, Target};
use super::json::{obj, Json};
use super::scan;
use super::text::Pos;
use crate::ast::*;
use crate::source::Span;
use crate::types::Ty;

// ----- types, as the hints show them -----------------------------------------

/// Longer labels are shortened to the outermost type (`List` for a list).
const MAX_LABEL: usize = 48;

/// What a type says for sure, written like Rythme writes values: `Int`,
/// `List(Text)`, `Option(Int)`, `{name: Text, age: Int}`, `Shape`. `None`
/// when nothing is certain.
pub fn type_label(t: &Ty) -> Option<String> {
    let full = label(t, 3)?;
    if full.chars().count() <= MAX_LABEL {
        return Some(full);
    }
    label(t, 0)
}

fn inner(t: &Ty, depth: usize) -> String {
    if depth == 0 {
        return "?".into();
    }
    label(t, depth - 1).unwrap_or_else(|| "?".into())
}

fn label(t: &Ty, depth: usize) -> Option<String> {
    Some(match t {
        Ty::Never | Ty::Any => return None,
        Ty::Unit | Ty::Bool | Ty::Int | Ty::Float | Ty::Num | Ty::Text | Ty::Symbol => t.name(),
        Ty::List(e) => match label(e, depth.saturating_sub(1)).filter(|_| depth > 0) {
            Some(e) => format!("List({e})"),
            None => "List".into(),
        },
        Ty::Record(r) => {
            if depth == 0 {
                return Some("Record".into());
            }
            let mut fields: Vec<String> = r.fields.iter().take(4).map(|(k, t)| format!("{k}: {}", inner(t, depth))).collect();
            if r.fields.len() > 4 || !r.exact {
                fields.push("…".into());
            }
            format!("{{{}}}", fields.join(", "))
        }
        Ty::Data(d) => {
            let name = d.name.to_string();
            let field = |ctor: &str| {
                let cs = d.ctors.as_ref()?;
                let (_, fs) = cs.iter().find(|(c, _)| &*c.name() == ctor)?;
                fs.first().and_then(|f| label(f, depth.saturating_sub(1)).filter(|_| depth > 0))
            };
            match name.as_str() {
                "Option" => match field("Some") {
                    Some(x) => format!("Option({x})"),
                    None => name,
                },
                "Result" => match (field("Ok"), field("Err")) {
                    (None, None) => name,
                    (ok, err) => {
                        let q = || "?".to_string();
                        format!("Result({}, {})", ok.unwrap_or_else(q), err.unwrap_or_else(q))
                    }
                },
                _ => name,
            }
        }
        Ty::Func(_) => t.name(),
    })
}

// ----- the syntax tree, read once ---------------------------------------------

/// What the features below need from the syntax tree.
#[derive(Default)]
struct Tree {
    /// Every expression, pattern, definition and line of code: what a
    /// selection grows through.
    nodes: Vec<(u32, u32)>,
    /// What may be folded.
    folds: Vec<(u32, u32)>,
    /// `x = 12`: names whose value is written right there (no hint needed).
    literal: HashSet<Span>,
    /// `name = fn(x) ...`: the end of the head `fn(x)`, by the place of the name.
    lambda_heads: HashMap<Span, u32>,
    /// The symbols written in the code (`:ok`), in expressions and patterns.
    symbols: Vec<Span>,
}

impl Tree {
    fn of(items: &[Item]) -> Tree {
        let mut t = Tree::default();
        for item in items {
            match item {
                Item::Stmt(s) => t.stmt(s),
                Item::Type(d) => {
                    let full = d.variants.last().map_or(d.span, |v| d.span.to(v.span));
                    t.node(full);
                    t.fold(full);
                    for v in &d.variants {
                        t.node(v.span);
                    }
                }
                Item::Test(_, body, span) => {
                    t.node(span.to(body.span));
                    t.fold(span.to(body.span));
                    t.expr(body);
                }
                Item::Use(u) => t.node(u.span),
            }
        }
        t
    }

    fn node(&mut self, s: Span) {
        self.nodes.push((s.start, s.end));
    }

    fn fold(&mut self, s: Span) {
        self.folds.push((s.start, s.end));
    }

    fn stmt(&mut self, s: &Stmt) {
        match s {
            Stmt::Let(p, e) => {
                self.node(p.span.to(e.span));
                self.fold(p.span.to(e.span));
                if matches!(p.kind, PatKind::Bind(_)) {
                    if is_literal(e) {
                        self.literal.insert(p.span);
                    }
                    if let ExprKind::Lambda(def) = &e.kind {
                        if let Some(c) = def.clauses.first() {
                            self.lambda_heads.insert(p.span, c.span.end);
                        }
                    }
                }
                self.pattern(p);
                self.expr(e);
            }
            Stmt::Expr(e) => self.expr(e),
            Stmt::Fn(def) => self.func(def),
        }
    }

    fn func(&mut self, def: &FnDef) {
        if let (Some(first), Some(last)) = (def.clauses.first(), def.clauses.last()) {
            let all = first.span.to(last.body.span);
            self.node(all);
            self.fold(all);
        }
        for c in &def.clauses {
            let whole = c.span.to(c.body.span);
            self.node(whole);
            if def.clauses.len() > 1 {
                self.fold(whole);
            }
            self.node(c.span);
            for p in &c.params {
                self.pattern(p);
            }
            if let Some(g) = &c.guard {
                self.expr(g);
            }
            self.expr(&c.body);
        }
    }

    fn pattern(&mut self, p: &Pattern) {
        self.node(p.span);
        match &p.kind {
            PatKind::Symbol(_) => self.symbols.push(p.span),
            PatKind::Ctor(_, ps) | PatKind::List(ps, _) | PatKind::Or(ps) => ps.iter().for_each(|x| self.pattern(x)),
            PatKind::Record(fs) => fs.iter().for_each(|(_, x)| self.pattern(x)),
            _ => {}
        }
    }

    fn expr(&mut self, e: &Expr) {
        self.node(e.span);
        match &e.kind {
            ExprKind::Unit
            | ExprKind::Bool(_)
            | ExprKind::Int(_)
            | ExprKind::Big(_)
            | ExprKind::Float(_)
            | ExprKind::Text(_)
            | ExprKind::Var(_)
            | ExprKind::Ctor(_) => {}
            ExprKind::Symbol(_) => self.symbols.push(e.span),
            ExprKind::Template(parts) => {
                self.fold(e.span);
                for p in parts {
                    if let TemplatePart::Expr(x) = p {
                        self.expr(x);
                    }
                }
            }
            ExprKind::List(items) => {
                self.fold(e.span);
                for it in items {
                    match it {
                        ListItem::One(x) | ListItem::Spread(x) => self.expr(x),
                    }
                }
            }
            ExprKind::Record(items) => {
                self.fold(e.span);
                for it in items {
                    match it {
                        RecordItem::Field(_, ks, x) => {
                            self.node(ks.to(x.span));
                            self.expr(x);
                        }
                        RecordItem::Spread(x) => self.expr(x),
                    }
                }
            }
            ExprKind::Lambda(def) => self.func(def),
            ExprKind::Call(f, args) => {
                self.fold(e.span);
                self.expr(f);
                args.iter().for_each(|a| self.expr(a));
            }
            ExprKind::Method(recv, _, args, name) => {
                self.fold(e.span);
                self.expr(recv);
                self.node(*name);
                args.iter().for_each(|a| self.expr(a));
            }
            ExprKind::Field(x, _, name) => {
                self.expr(x);
                self.node(*name);
            }
            ExprKind::Unary(_, x) | ExprKind::Try(x) => self.expr(x),
            ExprKind::Index(a, b) | ExprKind::Binary(_, a, b, _) | ExprKind::Range(a, b) => {
                self.expr(a);
                self.expr(b);
            }
            ExprKind::If(c, a, b) => {
                self.fold(e.span);
                self.expr(c);
                self.expr(a);
                self.expr(b);
            }
            ExprKind::Match(subject, arms) => {
                self.fold(e.span);
                self.expr(subject);
                for arm in arms {
                    let whole = arm.pat.span.to(arm.body.span);
                    self.node(whole);
                    self.fold(whole);
                    self.pattern(&arm.pat);
                    if let Some(g) = &arm.guard {
                        self.expr(g);
                    }
                    self.expr(&arm.body);
                }
            }
            ExprKind::Block(stmts) => {
                self.fold(e.span);
                stmts.iter().for_each(|s| self.stmt(s));
            }
        }
    }
}

/// A value written as it is: `12`, `-1.5`, `"hello"`, `true`, `()`.
fn is_literal(e: &Expr) -> bool {
    match &e.kind {
        ExprKind::Unit
        | ExprKind::Bool(_)
        | ExprKind::Int(_)
        | ExprKind::Big(_)
        | ExprKind::Float(_)
        | ExprKind::Text(_)
        | ExprKind::Symbol(_) => true,
        ExprKind::Unary(UnOp::Neg, x) => matches!(x.kind, ExprKind::Int(_) | ExprKind::Big(_) | ExprKind::Float(_)),
        _ => false,
    }
}

// ----- inlay hints --------------------------------------------------------------

/// `textDocument/inlayHint`: after a name, its type, when the checker is
/// sure of it (`total: Int`); after a parameter, what the code of its
/// function needs it to be (`side: Int|Float`, from [`crate::check::signatures`]);
/// after the head of a function, what it gives back (`fn area(r) → Float`).
/// Nothing for a name whose value is written right there (`x = 12`), nor
/// when the file has errors that stop the checker.
pub fn inlay_hints(cx: &Ctx, a: &Analysis, start: usize, end: usize) -> Json {
    let types = &a.module.types;
    let signatures = &a.module.signatures;
    if types.is_empty() && signatures.is_empty() {
        return Json::Arr(Vec::new());
    }
    let tree = Tree::of(&a.module.items);
    let defs = &a.index().defs;
    // The signature of the function whose definition holds a parameter: the innermost one.
    let functions: Vec<&Def> =
        defs.iter().filter(|d| d.kind == DefKind::Function || (d.kind == DefKind::Local && d.sig.is_some())).collect();
    let param_need = |d: &Def| -> Option<String> {
        let f = functions
            .iter()
            .filter(|f| f.full.start <= d.span.start && d.span.end <= f.full.end)
            .min_by_key(|f| f.full.end - f.full.start)?;
        let sig = signatures.get(&f.span)?;
        sig.params.iter().find(|(n, _)| **n == *d.name).and_then(|(_, t)| t.clone())
    };
    let mut hints: Vec<(usize, String, bool)> = Vec::new();
    for d in defs {
        let t = types.get(&d.span);
        // What a function gives back: the type the checker is sure of, or else the inferred one.
        let ret = || t.and_then(|t| type_label(&t.ret())).or_else(|| signatures.get(&d.span).and_then(|s| s.ret.clone()));
        match d.kind {
            DefKind::Function => {
                let Some(head) = d.heads.first() else { continue };
                if let Some(ret) = ret() {
                    hints.push((head.end as usize, format!("→ {ret}"), true));
                }
            }
            DefKind::Local if d.sig.is_some() => {
                let Some(&head) = tree.lambda_heads.get(&d.span) else { continue };
                if let Some(ret) = ret() {
                    hints.push((head as usize, format!("→ {ret}"), true));
                }
            }
            DefKind::Param => {
                if d.punned {
                    continue;
                }
                if let Some(l) = t.and_then(type_label).or_else(|| param_need(d)) {
                    hints.push((d.span.end as usize, format!(": {l}"), false));
                }
            }
            DefKind::Value | DefKind::Local | DefKind::Case => {
                let Some(t) = t else { continue };
                // `{age}` in a record pattern: `{age: Int}` would read as another name.
                if d.punned || tree.literal.contains(&d.span) {
                    continue;
                }
                if let Some(l) = type_label(t) {
                    hints.push((d.span.end as usize, format!(": {l}"), false));
                }
            }
            DefKind::Type | DefKind::Ctor | DefKind::Test => {}
        }
    }
    hints.sort();
    hints.dedup_by(|x, y| x.0 == y.0);
    let out = hints
        .into_iter()
        .filter(|(at, _, _)| start <= *at && *at <= end)
        .map(|(at, label, padding)| {
            let mut h = obj([("position", pos_json(a.lines.pos(&a.text, at, cx.enc))), ("label", label.into()), ("kind", 1u32.into())]);
            if padding {
                h.set("paddingLeft", true.into());
            }
            h
        })
        .collect();
    Json::Arr(out)
}

// ----- semantic tokens ------------------------------------------------------------

/// The kinds of names, in the order of the legend sent at start-up.
pub const TOKEN_TYPES: [&str; 5] = ["function", "variable", "parameter", "type", "enumMember"];
/// Bits: 1 = where the name is defined, 2 = a name of Rythme itself.
pub const TOKEN_MODIFIERS: [&str; 2] = ["declaration", "defaultLibrary"];

fn def_token(d: &Def) -> Option<u32> {
    Some(match d.kind {
        DefKind::Function => 0,
        DefKind::Local if d.sig.is_some() => 0,
        DefKind::Value | DefKind::Local | DefKind::Case => 1,
        DefKind::Param => 2,
        DefKind::Type => 3,
        DefKind::Ctor => 4,
        DefKind::Test => return None,
    })
}

/// `textDocument/semanticTokens/full`: each name with what it is, encoded as
/// the protocol says (five numbers per name, relative to the one before).
/// Symbols (`:ok`) are values named like constructors (`enumMember`).
/// Keywords, numbers, texts and comments keep the colors of the grammar.
pub fn semantic_tokens(cx: &Ctx, a: &Analysis) -> Vec<u32> {
    let ix: &Index = a.index();
    let mut found: Vec<(Span, u32, u32)> = Vec::new();
    for occ in &ix.occs {
        let (kind, mut mods) = match &occ.target {
            Target::Def(id) => match a.def(*id).and_then(def_token) {
                Some(k) => (k, 0),
                None => continue,
            },
            Target::Builtin(_) => (0, 2),
            Target::Pi => (1, 2),
            Target::Prelude(_) => (4, 2),
            Target::Unknown => continue,
        };
        if occ.def {
            mods |= 1;
        }
        found.push((occ.span, kind, mods));
    }
    found.extend(Tree::of(&a.module.items).symbols.into_iter().map(|s| (s, 4, 0)));
    found.sort_by_key(|(s, ..)| (s.start, s.end));
    let mut out = Vec::new();
    let mut prev = Pos { line: 0, character: 0 };
    let mut last_end = 0u32;
    for (span, kind, mods) in found {
        if span.start < last_end || span.end <= span.start || span.file != a.module.file {
            continue;
        }
        let (s, e) = (span.start as usize, span.end as usize);
        let Some(text) = a.text.get(s..e) else { continue };
        if text.contains(['\n', '\r']) {
            continue;
        }
        let p = a.lines.pos(&a.text, s, cx.enc);
        let delta_line = p.line - prev.line;
        let delta_char = if delta_line == 0 { p.character - prev.character } else { p.character };
        out.extend([delta_line, delta_char, cx.enc.len(text) as u32, kind, mods]);
        prev = p;
        last_end = span.end;
    }
    out
}

/// The numbers of a range of lines, from those of the whole document.
pub fn tokens_in_lines(data: &[u32], first: u32, last: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let (mut line, mut character) = (0u32, 0u32);
    let mut kept_prev: Option<(u32, u32)> = None;
    for t in data.chunks_exact(5) {
        if t[0] > 0 {
            line += t[0];
            character = t[1];
        } else {
            character += t[1];
        }
        if line < first || line > last {
            continue;
        }
        let (dl, dc) = match kept_prev {
            None => (line, character),
            Some((l, c)) if l == line => (0, character - c),
            Some((l, _)) => (line - l, character),
        };
        out.extend([dl, dc, t[2], t[3], t[4]]);
        kept_prev = Some((line, character));
    }
    out
}

/// `semanticTokens/full/delta`: one edit from the previous numbers to the new ones.
pub fn tokens_edit(old: &[u32], new: &[u32]) -> Vec<Json> {
    if old == new {
        return Vec::new();
    }
    let prefix = old.iter().zip(new).take_while(|(a, b)| a == b).count();
    let max_suffix = old.len().min(new.len()) - prefix;
    let suffix = old.iter().rev().zip(new.iter().rev()).take(max_suffix).take_while(|(a, b)| a == b).count();
    let data: Vec<Json> = new[prefix..new.len() - suffix].iter().map(|n| (*n).into()).collect();
    vec![obj([("start", prefix.into()), ("deleteCount", (old.len() - prefix - suffix).into()), ("data", Json::Arr(data))])]
}

// ----- code lenses ------------------------------------------------------------------

fn lens(cx: &Ctx, a: &Analysis, at: usize, title: String, command: &str, args: Vec<Json>) -> Json {
    let range = range_in(&a.text, &a.lines, at, at, cx.enc);
    let command = obj([("title", title.into()), ("command", command.into()), ("arguments", Json::Arr(args))]);
    obj([("range", range), ("command", command)])
}

/// `textDocument/codeLens`: "Run" above the first line that does something,
/// "Run test" above each test, and "Run every test" above the first one when
/// there are several. The commands (`rythme.runFile`, `rythme.runTest`,
/// `rythme.testFile`) are carried out by the editor: the VS Code extension
/// runs them in a terminal; `rythme help editors` shows how elsewhere.
pub fn code_lenses(cx: &Ctx, a: &Analysis) -> Json {
    let uri: Json = a.uri.clone().into();
    let items = &a.module.items;
    let mut out = Vec::new();
    let first_expr = items.iter().find_map(|it| match it {
        Item::Stmt(Stmt::Expr(e)) => Some(e.span.start as usize),
        _ => None,
    });
    if let Some(at) = first_expr {
        out.push(lens(cx, a, at, t!("▶ Run", "▶ Lancer"), "rythme.runFile", vec![uri.clone()]));
    }
    let tests: Vec<(&str, usize)> = items
        .iter()
        .filter_map(|it| match it {
            Item::Test(name, _, span) => Some((&**name, span.start as usize)),
            _ => None,
        })
        .collect();
    for (i, (name, at)) in tests.iter().enumerate() {
        if i == 0 && tests.len() > 1 {
            let n = tests.len();
            out.push(lens(cx, a, *at, t!("▶ Run the {n} tests", "▶ Lancer les {n} tests"), "rythme.testFile", vec![uri.clone()]));
        }
        out.push(lens(cx, a, *at, t!("▶ Run test", "▶ Lancer le test"), "rythme.runTest", vec![uri.clone(), (*name).into()]));
    }
    Json::Arr(out)
}

// ----- folding --------------------------------------------------------------------------

/// The last line of a fold: the line of a closing `end`, `)`, `]` or `}` stays visible.
fn fold_end(a: &Analysis, end: usize) -> u32 {
    let p = a.lines.pos(&a.text, end, super::text::Encoding::Utf8);
    let (s, e) = a.lines.line_range(&a.text, p.line as usize);
    let line = a.text[s..e].trim_start();
    let closing = (line.starts_with("end") && !line[3..].starts_with(scan::is_name_char)) || line.starts_with([')', ']', '}']);
    if closing && p.line > 0 {
        p.line - 1
    } else {
        p.line
    }
}

/// `textDocument/foldingRange`: functions, types, tests, blocks, `match`
/// and their cases, long lists, records and calls; groups of comment lines
/// and of `use` lines.
pub fn folding_ranges(a: &Analysis) -> Json {
    let lines = &a.lines;
    let line_of = |o: usize| lines.pos(&a.text, o, super::text::Encoding::Utf8).line;
    let mut found: HashMap<u32, (u32, Option<&str>)> = HashMap::new();
    let mut add = |start: u32, end: u32, kind: Option<&'static str>| {
        if end > start {
            let e = found.entry(start).or_insert((end, kind));
            if end > e.0 {
                *e = (end, kind);
            }
        }
    };
    for (s, e) in Tree::of(&a.module.items).folds {
        add(line_of(s as usize), fold_end(a, e as usize), None);
    }
    // Comment lines one after the other, and `use` lines.
    let mut run: Option<(u32, u32, &'static str)> = None;
    for line in 0..lines.line_count() {
        let (s, e) = lines.line_range(&a.text, line);
        let text = a.text[s..e].trim_start();
        let kind = if text.starts_with('#') {
            Some("comment")
        } else if text.starts_with("use ") || text.starts_with("use\"") {
            Some("imports")
        } else {
            None
        };
        let line = line as u32;
        match (run, kind) {
            (Some((first, _, k)), Some(now)) if k == now => run = Some((first, line, k)),
            _ => {
                if let Some((first, last, k)) = run {
                    add(first, last, Some(k));
                }
                run = kind.map(|k| (line, line, k));
            }
        }
    }
    if let Some((first, last, k)) = run {
        add(first, last, Some(k));
    }
    let mut ranges: Vec<(u32, u32, Option<&str>)> = found.into_iter().map(|(s, (e, k))| (s, e, k)).collect();
    ranges.sort();
    Json::Arr(
        ranges
            .into_iter()
            .map(|(s, e, k)| {
                let mut r = obj([("startLine", s.into()), ("endLine", e.into())]);
                if let Some(k) = k {
                    r.set("kind", k.into());
                }
                r
            })
            .collect(),
    )
}

// ----- selection ranges -------------------------------------------------------------------

/// `textDocument/selectionRange`: for each position, the name there, then
/// each piece of code around it, out to the whole document.
pub fn selection_ranges(cx: &Ctx, a: &Analysis, offsets: &[usize]) -> Json {
    let tree = Tree::of(&a.module.items);
    let len = a.text.len() as u32;
    let out = offsets
        .iter()
        .map(|&o| {
            let at = o as u32;
            let mut spans: Vec<(u32, u32)> = tree.nodes.iter().copied().filter(|&(s, e)| s <= at && at <= e && e <= len).collect();
            if let Some((s, e)) = scan::word_at(&a.text, o) {
                spans.push((s as u32, e as u32));
            }
            spans.push((0, len));
            spans.sort_by_key(|&(s, e)| (e - s, std::cmp::Reverse(s)));
            // Each range must hold the one before.
            let mut chain: Vec<(u32, u32)> = Vec::new();
            for sp in spans {
                match chain.last() {
                    Some(&(s, e)) if sp == (s, e) || !(sp.0 <= s && e <= sp.1) => {}
                    _ => chain.push(sp),
                }
            }
            if chain.is_empty() {
                chain.push((at, at));
            }
            let mut node: Option<Json> = None;
            for &(s, e) in chain.iter().rev() {
                let mut r = obj([("range", range_in(&a.text, &a.lines, s as usize, e as usize, cx.enc))]);
                if let Some(parent) = node.take() {
                    r.set("parent", parent);
                }
                node = Some(r);
            }
            node.unwrap_or(Json::Null)
        })
        .collect();
    Json::Arr(out)
}
