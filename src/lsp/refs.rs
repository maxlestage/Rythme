//! Every use of a name, in its file and in the files that use it, and renaming.
//!
//! A rename is only done when it is sure to be correct: the renamed files are
//! read again, and every name in them must still mean exactly what it meant
//! before (nothing hidden, nothing captured), with no new problem.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::analysis::{Analysis, Module};
use super::features::{location, range_in};
use super::index::{DefId, DefKind, Target};
use super::json::{obj, Json};
use super::scan;
use super::text::{canonical, path_to_uri};
use super::{Reply, RpcError, Server};
use crate::builtins;
use crate::lexer::{KEYWORDS, RESERVED};

/// A definition, the same in every analysis: its file and where its name starts.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct DefKey {
    file: String,
    start: usize,
}

fn file_key(m: &Module) -> String {
    m.path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| m.uri.clone())
}

fn key_of(a: &Analysis, id: DefId) -> Option<DefKey> {
    let m = a.module_of(id.file)?;
    Some(DefKey { file: file_key(m), start: m.index.def(id.idx)?.span.start as usize })
}

/// What the name at the cursor refers to, or why there is nothing to rename there.
fn target_at(a: &Analysis, offset: usize) -> Result<(DefId, usize, usize), String> {
    let Some(occ) = a.index().occ_at(offset) else {
        return Err(t!("there is no name here", "il n'y a pas de nom ici"));
    };
    let name = &occ.name;
    match &occ.target {
        Target::Def(id) => match a.def(*id) {
            Some(d) if d.kind == DefKind::Test => {
                Err(t!("the name of a test is a text: edit it directly", "le nom d'un test est un texte : modifiez-le directement"))
            }
            Some(_) => Ok((*id, occ.span.start as usize, occ.span.end as usize)),
            None => Err(t!("`{name}` cannot be found", "`{name}` est introuvable")),
        },
        Target::Builtin(_) => Err(t!(
            "`{name}` is a built-in function: it cannot be renamed",
            "`{name}` est une fonction intégrée : elle ne peut pas être renommée"
        )),
        Target::Pi | Target::Prelude(_) => {
            Err(t!("`{name}` is part of Rythme: it cannot be renamed", "`{name}` fait partie de Rythme : il ne peut pas être renommé"))
        }
        Target::Unknown => {
            Err(t!("`{name}` is not defined: there is nothing to rename", "`{name}` n'est pas défini : il n'y a rien à renommer"))
        }
    }
}

// ----- the files that see a definition -----------------------------------

/// The `.ry` files of the workspace (a bounded number, skipping hidden and build folders).
fn ry_files(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut todo: Vec<(PathBuf, usize)> = roots.iter().map(|r| (r.clone(), 0)).collect();
    while let Some((dir, depth)) = todo.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            if out.len() >= 5000 {
                return out;
            }
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            let Ok(kind) = e.file_type() else { continue };
            if kind.is_dir() && depth < 16 {
                todo.push((e.path(), depth + 1));
            } else if kind.is_file() && name.ends_with(".ry") {
                out.push(canonical(&e.path()));
            }
        }
    }
    out
}

/// The paths written in the `use` lines of a text.
fn uses_in(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| {
            let rest = l.trim_start().strip_prefix("use")?;
            if !rest.starts_with([' ', '\t']) {
                return None;
            }
            let rest = rest.trim_start();
            match rest.strip_prefix('"') {
                Some(quoted) => quoted.split('"').next().map(str::to_string),
                // `use geometry`, `use shapes/circle as circle`.
                None => rest.split([' ', '\t', '#']).next().filter(|p| !p.is_empty()).map(str::to_string),
            }
        })
        .collect()
}

impl Server {
    /// The text of a file: the editor's version when it is open.
    fn file_text(&self, path: &Path) -> Option<(String, Rc<str>)> {
        if let Some(d) = self.docs.values().find(|d| d.path.as_deref() == Some(path)) {
            return Some((d.uri.clone(), d.text.clone()));
        }
        let text = std::fs::read_to_string(path).ok()?;
        Some((path_to_uri(path), Rc::from(text)))
    }

    /// The files that use `target`, directly or through other files.
    fn users_of(&self, target: &Path) -> Vec<PathBuf> {
        let mut files = ry_files(&self.roots);
        files.extend(self.docs.values().filter_map(|d| d.path.clone()));
        files.sort();
        files.dedup();
        let mut users: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
        for f in &files {
            let Some((_, text)) = self.file_text(f) else { continue };
            let base = f.parent().map(Path::to_path_buf).unwrap_or_default();
            for p in uses_in(&text) {
                let mut dep = base.join(&p);
                if dep.extension().is_none() {
                    dep.set_extension("ry");
                }
                if dep.is_file() {
                    users.entry(canonical(&dep)).or_default().push(f.clone());
                }
            }
        }
        let mut seen: HashSet<PathBuf> = HashSet::new();
        let mut todo = vec![target.to_path_buf()];
        while let Some(p) = todo.pop() {
            for u in users.get(&p).into_iter().flatten() {
                if seen.insert(u.clone()) {
                    todo.push(u.clone());
                }
            }
        }
        seen.remove(target);
        let mut out: Vec<PathBuf> = seen.into_iter().collect();
        out.sort();
        out
    }

    /// The other files where a definition can be used: its own file and the
    /// files that use it (only for top-level definitions).
    fn other_files(&self, a: &Analysis, id: DefId) -> Vec<PathBuf> {
        let top = a.def(id).is_some_and(|d| d.top);
        let def_path = a.module_of(id.file).and_then(|m| m.path.clone());
        let (true, Some(def_path)) = (top, def_path) else { return Vec::new() };
        let mut files = self.users_of(&def_path);
        files.push(def_path);
        files.retain(|f| Some(f) != a.path.as_ref());
        files.sort();
        files.dedup();
        files
    }
}

// ----- references ---------------------------------------------------------

fn collect(cx: &super::Ctx, a: &Analysis, key: &DefKey, include_decl: bool, out: &mut Vec<Json>) {
    for occ in &a.index().occs {
        let Target::Def(id) = occ.target else { continue };
        if (include_decl || !occ.def) && key_of(a, id).as_ref() == Some(key) {
            if let Some((uri, range)) = location(cx, a, occ.span) {
                out.push(obj([("uri", uri.into()), ("range", range)]));
            }
        }
    }
}

pub fn references(s: &mut Server, uri: &str, offset: usize, include_decl: bool) -> Reply {
    let cx = s.cx;
    let Some(a) = s.docs.get(uri).and_then(|d| d.analysis.as_ref()) else { return Ok(Json::Null) };
    let Some(Target::Def(id)) = a.index().occ_at(offset).map(|o| o.target.clone()) else { return Ok(Json::Null) };
    let Some(key) = key_of(a, id) else { return Ok(Json::Null) };
    let mut out = Vec::new();
    collect(&cx, a, &key, include_decl, &mut out);
    let others = s.other_files(a, id);
    let mut closed = Vec::new();
    for f in others {
        match s.docs.values().find(|d| d.path.as_ref() == Some(&f)).and_then(|d| d.analysis.as_ref()) {
            Some(b) => collect(&cx, b, &key, include_decl, &mut out),
            None => closed.push(f),
        }
    }
    for f in closed {
        let Some((furi, text)) = s.file_text(&f) else { continue };
        let b = s.load("", Some(&f), &furi, text, &HashMap::new());
        collect(&cx, &b, &key, include_decl, &mut out);
    }
    Ok(Json::Arr(out))
}

// ----- renaming -----------------------------------------------------------

pub fn prepare_rename(s: &mut Server, uri: &str, offset: usize) -> Reply {
    let cx = s.cx;
    let Some(a) = s.docs.get(uri).and_then(|d| d.analysis.as_ref()) else { return Ok(Json::Null) };
    let (id, start, end) = target_at(a, offset).map_err(RpcError::failed)?;
    let name = a.def(id).map(|d| d.name.to_string()).unwrap_or_default();
    Ok(obj([("range", range_in(&a.text, &a.lines, start, end, cx.enc)), ("placeholder", name.into())]))
}

/// Is `new` a name that can replace `old`?
fn check_name(new: &str, old: &str, top: bool) -> Result<(), String> {
    let mut chars = new.chars();
    let valid = chars.next().is_some_and(scan::is_name_start) && chars.all(scan::is_name_char) && new != "_";
    if !valid {
        return Err(t!("`{new}` is not a valid name", "`{new}` n'est pas un nom valide"));
    }
    if KEYWORDS.contains(&new) || RESERVED.contains(&new) {
        return Err(t!("`{new}` is a keyword", "`{new}` est un mot-clé"));
    }
    let upper = |s: &str| s.chars().next().is_some_and(char::is_uppercase);
    if upper(new) != upper(old) {
        return Err(if upper(old) {
            t!(
                "the names of types and constructors start with an uppercase letter",
                "les noms de types et de constructeurs commencent par une majuscule"
            )
        } else {
            t!(
                "names start with a lowercase letter (uppercase is for types and constructors)",
                "les noms commencent par une minuscule (les majuscules sont pour les types et les constructeurs)"
            )
        });
    }
    if top && builtins::lookup(new).is_some() {
        return Err(t!(
            "`{new}` is a built-in function: choose another name",
            "`{new}` est une fonction intégrée : choisissez un autre nom"
        ));
    }
    Ok(())
}

/// A file to rename in: its address, path, text before and the edits.
struct Change {
    uri: String,
    path: Option<PathBuf>,
    text: Rc<str>,
    old: Analysis,
    /// (start, end, new text), in order.
    edits: Vec<(usize, usize, String)>,
}

impl Change {
    fn new_text(&self) -> String {
        let mut out = String::with_capacity(self.text.len());
        let mut at = 0;
        for (s, e, new) in &self.edits {
            out.push_str(&self.text[at..*s]);
            out.push_str(new);
            at = *e;
        }
        out.push_str(&self.text[at..]);
        out
    }

    /// Where a name starting at a byte offset of the old text starts in the
    /// new text. In `{name}` renamed `{name: new}`, the new name comes after `name: `.
    fn shift(&self, offset: usize) -> usize {
        let mut delta: isize = 0;
        for (s, e, new) in &self.edits {
            if *e <= offset {
                delta += new.len() as isize - (e - s) as isize;
            } else if *s == offset {
                delta += new.rfind(' ').map_or(0, |i| i + 1) as isize;
            }
        }
        (offset as isize + delta) as usize
    }
}

fn fields_seen(a: &Analysis) -> HashSet<Rc<str>> {
    let mut out: HashSet<Rc<str>> = a.index().fields.iter().cloned().collect();
    for m in a.modules.values() {
        out.extend(m.index.fields.iter().cloned());
    }
    out
}

pub fn rename(s: &mut Server, uri: &str, offset: usize, new_name: &str) -> Reply {
    let cx = s.cx;
    let Some(doc) = s.docs.get(uri) else { return Ok(Json::Null) };
    let Some(a) = doc.analysis.as_ref() else { return Ok(Json::Null) };
    let (id, _, _) = target_at(a, offset).map_err(RpcError::failed)?;
    let (Some(def), Some(key)) = (a.def(id), key_of(a, id)) else { return Ok(Json::Null) };
    let old_name = def.name.to_string();
    check_name(new_name, &old_name, def.top).map_err(RpcError::failed)?;
    if new_name == old_name {
        return Ok(obj([("changes", Json::Obj(Vec::new()))]));
    }
    let mut targets: Vec<(Option<PathBuf>, String, Rc<str>)> = vec![(doc.path.clone(), doc.uri.clone(), doc.text.clone())];
    for f in s.other_files(a, id) {
        if let Some((furi, text)) = s.file_text(&f) {
            targets.push((Some(f), furi, text));
        }
    }

    // Each file as it is now, and what changes in it.
    let mut changes: Vec<Change> = Vec::new();
    for (path, furi, text) in targets {
        let old = s.load("rename-old:", path.as_deref(), &furi, text.clone(), &HashMap::new());
        if !old.module.parsed {
            let shown = super::text::short_name(&furi);
            return Err(RpcError::failed(t!(
                "`{shown}` has a syntax error: fix it before renaming",
                "`{shown}` contient une erreur de syntaxe : corrigez-la avant de renommer"
            )));
        }
        let mut edits = Vec::new();
        let fields = fields_seen(&old);
        for occ in &old.index().occs {
            let Target::Def(oid) = occ.target else { continue };
            if key_of(&old, oid).as_ref() != Some(&key) {
                continue;
            }
            if occ.method && fields.contains(&*old_name) {
                return Err(RpcError::failed(t!(
                    "`{old_name}` is also the name of a record field, and `value.{old_name}(...)` may call that field: rename it by hand",
                    "`{old_name}` est aussi le nom d'un champ d'enregistrement, et `valeur.{old_name}(...)` peut appeler ce champ : renommez-le à la main"
                )));
            }
            let (st, en) = (occ.span.start as usize, occ.span.end as usize);
            // `{name}` reads the field `name`: it becomes `{name: new}`.
            let new = if occ.punned { format!("{old_name}: {new_name}") } else { new_name.to_string() };
            edits.push((st, en, new));
        }
        edits.sort_by_key(|e| e.0);
        edits.dedup_by_key(|e| e.0);
        if !edits.is_empty() {
            changes.push(Change { uri: furi, path, text, old, edits });
        }
    }

    // The files after the rename must mean the same thing.
    let overlay: HashMap<PathBuf, String> = changes.iter().filter_map(|c| Some((c.path.clone()?, c.new_text()))).collect();
    let shifts: HashMap<String, usize> = changes.iter().enumerate().map(|(i, c)| (file_key(&c.old.module), i)).collect();
    let moved = |k: &DefKey| -> DefKey {
        match shifts.get(&k.file) {
            Some(&i) => DefKey { file: k.file.clone(), start: changes[i].shift(k.start) },
            None => k.clone(),
        }
    };
    for c in &changes {
        let new_text: Rc<str> = Rc::from(c.new_text());
        let new = s.load("rename-new:", c.path.as_deref(), &c.uri, new_text, &overlay);
        if let Err(why) = same_meaning(&c.old, &new, c, &moved) {
            return Err(RpcError::failed(t!(
                "cannot rename `{old_name}` to `{new_name}`: {why}",
                "impossible de renommer `{old_name}` en `{new_name}` : {why}"
            )));
        }
    }

    let mut files = Vec::new();
    for c in &changes {
        let lines = &c.old.lines;
        let edits: Vec<Json> = c
            .edits
            .iter()
            .map(|(st, en, new)| obj([("range", range_in(&c.text, lines, *st, *en, cx.enc)), ("newText", new.clone().into())]))
            .collect();
        files.push((c.uri.clone(), Json::Arr(edits)));
    }
    Ok(obj([("changes", Json::Obj(files))]))
}

/// Checks that the renamed file means the same as before.
fn same_meaning(old: &Analysis, new: &Analysis, c: &Change, moved: &dyn Fn(&DefKey) -> DefKey) -> Result<(), String> {
    let changed = || t!("some names would then refer to something else", "certains noms désigneraient alors autre chose");
    if !new.module.parsed {
        return Err(t!("the code would not be valid", "le code ne serait plus valide"));
    }
    let (o, n) = (&old.index().occs, &new.index().occs);
    if o.len() != n.len() {
        return Err(changed());
    }
    for (x, y) in o.iter().zip(n.iter()) {
        if c.shift(x.span.start as usize) != y.span.start as usize {
            return Err(changed());
        }
        let same = match (&x.target, &y.target) {
            (Target::Def(i), Target::Def(j)) => key_of(old, *i).map(|k| moved(&k)) == key_of(new, *j),
            (Target::Unknown, Target::Unknown) => x.name == y.name,
            (a, b) => a == b,
        };
        if !same {
            return Err(changed());
        }
    }
    // No new problem (a name defined twice, a hidden built-in...).
    let mut before: HashMap<(&str, bool), usize> = HashMap::new();
    for d in old.diagnostics() {
        *before.entry((d.code, d.is_error())).or_default() += 1;
    }
    for d in new.diagnostics() {
        let n = before.entry((d.code, d.is_error())).or_default();
        if *n == 0 {
            return Err(d.message.clone());
        }
        *n -= 1;
    }
    Ok(())
}
