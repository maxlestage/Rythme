//! `workspace/symbol`: the top-level names of every Rythme file of the
//! workspace, open or not, to jump to one by typing a part of its name.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::SystemTime;

use super::analysis::FileIds;
use super::index::{self, DefKind, Index};
use super::json::{obj, Json};
use super::text::{path_to_uri, short_name, Encoding, LineIndex};
use crate::{lexer, parser};

/// Files looked at, at most, under the folders of the workspace.
const MAX_FILES: usize = 2000;
/// Names given, at most, for one search.
const MAX_RESULTS: usize = 256;

/// A top-level name of a file.
#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: u32,
    pub container: Option<String>,
    pub uri: String,
    pub range: Json,
}

/// The names of the files on the disk, kept until a file changes.
#[derive(Default)]
pub struct Cache {
    files: HashMap<PathBuf, (Option<SystemTime>, Rc<Vec<Symbol>>)>,
}

fn kind(k: DefKind) -> Option<u32> {
    Some(match k {
        DefKind::Function => 12,
        DefKind::Value => 13,
        DefKind::Type => 5,
        DefKind::Ctor => 22,
        DefKind::Test => 24,
        _ => return None,
    })
}

/// The top-level names of an index.
pub fn symbols_of(ix: &Index, uri: &str, text: &str, enc: Encoding) -> Vec<Symbol> {
    let lines = LineIndex::new(text);
    let file = short_name(uri);
    ix.defs
        .iter()
        .filter(|d| d.top)
        .filter_map(|d| {
            let kind = kind(d.kind)?;
            let name = if d.kind == DefKind::Test { format!("test {:?}", d.name) } else { d.name.to_string() };
            let container = match (&d.owner, d.kind) {
                (Some(t), DefKind::Ctor) => format!("{t} · {file}"),
                _ => file.clone(),
            };
            let span = if d.kind == DefKind::Test { d.full } else { d.span };
            let range = super::features::range_in(text, &lines, span.start as usize, span.end as usize, enc);
            Some(Symbol { name, kind, container: Some(container), uri: uri.to_string(), range })
        })
        .collect()
}

/// The `.ry` files under a folder, without hidden folders, `target` and `node_modules`.
fn ry_files(root: &Path, out: &mut Vec<PathBuf>) {
    let mut todo = vec![root.to_path_buf()];
    while let Some(dir) = todo.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name());
        for e in entries {
            if out.len() >= MAX_FILES {
                return;
            }
            let name = e.file_name().to_string_lossy().to_string();
            let Ok(ft) = e.file_type() else { continue };
            if ft.is_dir() {
                if !name.starts_with('.') && name != "target" && name != "node_modules" {
                    todo.push(e.path());
                }
            } else if name.ends_with(".ry") {
                out.push(e.path());
            }
        }
    }
}

impl Cache {
    /// The names of a file on the disk (read again when it changed).
    fn file(&mut self, ids: &mut FileIds, path: &Path, enc: Encoding) -> Rc<Vec<Symbol>> {
        let modified = std::fs::metadata(path).and_then(|m| m.modified()).ok();
        if let Some((m, syms)) = self.files.get(path) {
            if *m == modified && modified.is_some() {
                return syms.clone();
            }
        }
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let uri = path_to_uri(path);
        let id = ids.register(&format!("workspace:{}", path.display()), &short_name(&uri), &text);
        let syms = match lexer::lex(&text, id).and_then(parser::parse) {
            Ok(items) => {
                let ix = super::analysis::guard(|| index::build(&items, id, &text, &HashMap::new())).unwrap_or_default();
                symbols_of(&ix, &uri, &text, enc)
            }
            Err(_) => Vec::new(),
        };
        let syms = Rc::new(syms);
        self.files.insert(path.to_path_buf(), (modified, syms.clone()));
        syms
    }

    /// The names of the workspace: those of the open documents (`open`, by
    /// path, as the editor has them), then those of the other files.
    pub fn all(
        &mut self,
        ids: &mut FileIds,
        roots: &[PathBuf],
        open: &HashMap<PathBuf, Vec<Symbol>>,
        others: Vec<Symbol>,
        enc: Encoding,
    ) -> Vec<Symbol> {
        let mut out: Vec<Symbol> = others;
        for syms in open.values() {
            out.extend(syms.iter().cloned());
        }
        let mut files = Vec::new();
        for r in roots {
            ry_files(r, &mut files);
        }
        for f in files {
            let c = super::text::canonical(&f);
            if open.contains_key(&c) {
                continue;
            }
            out.extend(self.file(ids, &c, enc).iter().cloned());
        }
        out
    }
}

/// Is every letter of the query in the name, in order (ignoring case)?
/// The score is lower for better matches: the name itself, then a start.
fn score(name: &str, query: &str) -> Option<u32> {
    if query.is_empty() {
        return Some(3);
    }
    let n = name.to_lowercase();
    let q = query.to_lowercase();
    if n == q {
        return Some(0);
    }
    if n.starts_with(&q) {
        return Some(1);
    }
    if n.contains(&q) {
        return Some(2);
    }
    let mut chars = n.chars();
    q.chars().all(|c| chars.any(|x| x == c)).then_some(3)
}

/// The answer to `workspace/symbol`.
pub fn search(all: Vec<Symbol>, query: &str) -> Json {
    let mut found: Vec<(u32, Symbol)> = all.into_iter().filter_map(|s| Some((score(&s.name, query)?, s))).collect();
    found.sort_by(|a, b| (a.0, &a.1.name, &a.1.uri).cmp(&(b.0, &b.1.name, &b.1.uri)));
    found.truncate(MAX_RESULTS);
    Json::Arr(
        found
            .into_iter()
            .map(|(_, s)| {
                let mut j =
                    obj([("name", s.name.into()), ("kind", s.kind.into()), ("location", obj([("uri", s.uri.into()), ("range", s.range)]))]);
                if let Some(c) = s.container {
                    j.set("containerName", c.into());
                }
                j
            })
            .collect(),
    )
}
