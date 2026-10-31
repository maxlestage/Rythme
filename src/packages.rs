//! The commands of the package index (see `index.rs`): `rythme search`,
//! `rythme info`, `rythme add` and `rythme publish`, with `--format json`
//! for `search` and `info`. `rythme update` is in `project.rs`, beside
//! `rythme install`.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::diagnostic::{Diagnostic, Severity};
use crate::index::{self, clean, Entry, Index};
use crate::io::{err_line, out_line};
use crate::lsp::json::{obj, Json};
use crate::machine::{self, Format};
use crate::manifest::{self, Manifest, Source, MANIFEST};
use crate::project::{self, shown};
use crate::semver::{Range, Version};

fn report(d: &Diagnostic) {
    crate::diagnostic::show(d, crate::diagnostic::use_color());
}

fn usage(msg: String) -> i32 {
    err_line(&msg);
    2
}

/// An option that the command does not know: a usage error.
fn unknown_option(args: &[String], command: &str) -> Option<i32> {
    let bad = args.iter().find(|a| a.starts_with('-'))?;
    Some(usage(t!(
        "unknown option `{bad}` for `rythme {command}`\nsee `rythme help packages`",
        "option inconnue `{bad}` pour `rythme {command}`\nvoir `rythme help packages`"
    )))
}

/// The project of the current folder, when there is one.
fn project() -> Result<Option<Manifest>, Box<Diagnostic>> {
    match project::find_root(Path::new("")) {
        Some(root) => manifest::load(&root).map(Some),
        None => Ok(None),
    }
}

/// The index of the project of the current folder, or of `RYTHME_INDEX`, or the usual one.
fn index_of(m: Option<&Manifest>) -> Result<Index, Box<Diagnostic>> {
    Index::new(m.and_then(|m| m.index.as_deref().map(|i| (i, m.dir.as_path()))))
}

/// The index of the project of the current folder (see [`index_of`]).
fn current_index() -> Result<Index, Box<Diagnostic>> {
    index_of(project()?.as_ref())
}

// ----- JSON -----------------------------------------------------------------------------

/// The fields that start every document (as in `machine.rs`).
fn head(command: &str) -> Vec<(String, Json)> {
    vec![
        ("version".into(), Json::from(machine::VERSION)),
        ("command".into(), command.into()),
        ("rythme".into(), crate::VERSION.into()),
        ("language".into(), crate::lang::code().into()),
    ]
}

fn index_json(index: Option<&Index>) -> Json {
    match index {
        Some(i) => obj([("url", i.url.as_str().into()), ("updated", i.updated().map(|s| index::date(s, true)).into())]),
        None => Json::Null,
    }
}

fn strings(items: &[String]) -> Json {
    Json::Arr(items.iter().map(|s| s.as_str().into()).collect())
}

fn summary_json(e: &Entry) -> Json {
    obj([
        ("name", e.name.as_str().into()),
        ("description", e.description.as_str().into()),
        ("version", e.latest().map(|r| r.version.to_string()).into()),
        ("repository", e.repository.as_str().into()),
        ("license", e.license.as_str().into()),
        ("keywords", strings(&e.keywords)),
        ("authors", strings(&e.authors)),
    ])
}

fn entry_json(e: &Entry) -> Json {
    let versions: Vec<Json> = e
        .releases
        .iter()
        .map(|r| {
            obj([
                ("version", r.version.to_string().into()),
                ("tag", r.tag.clone().into()),
                ("commit", r.commit.as_str().into()),
                ("checksum", r.checksum.as_str().into()),
                ("yanked", r.yanked.clone().into()),
            ])
        })
        .collect();
    let Json::Obj(mut fields) = summary_json(e) else { return Json::Null };
    fields.push(("versions".into(), Json::Arr(versions)));
    Json::Obj(fields)
}

/// Prints the document of `command`, with `ok`, `exit_code`, the index,
/// these fields and the diagnostics; gives the exit code.
fn print_json(command: &str, index: Option<&Index>, more: Vec<(&str, Json)>, mut diags: Vec<Diagnostic>) -> i32 {
    if let Some(i) = index {
        diags.splice(0..0, i.warnings.iter().cloned());
    }
    let ok = !diags.iter().any(|d| d.severity == Severity::Error);
    let code = i32::from(!ok);
    let mut doc = head(command);
    doc.push(("ok".into(), ok.into()));
    doc.push(("exit_code".into(), i64::from(code).into()));
    doc.push(("index".into(), index_json(index)));
    doc.extend(more.into_iter().map(|(k, v)| (k.to_string(), v)));
    doc.push(("diagnostics".into(), Json::Arr(diags.iter().map(machine::diagnostic_json).collect())));
    out_line(&Json::Obj(doc).to_string());
    code
}

// ----- rythme search --------------------------------------------------------------------

/// The options of `search` and `info`: `--format json` and `--update`.
fn options(args: &[String], command: &str) -> Result<(Format, bool, Vec<String>), i32> {
    let mut rest = args.to_vec();
    let format = machine::take_format(&mut rest)?;
    let update = machine::take_flag(&mut rest, "--update");
    if let Some(code) = unknown_option(&rest, command) {
        return Err(code);
    }
    Ok((format, update, rest))
}

/// `rythme search [WORDS...] [--update] [--format json]`: the packages of the
/// index whose name, description or keywords have every word (all of them
/// without words), from the copy of the index.
pub fn search_command(args: &[String]) -> i32 {
    let (format, update, words) = match options(args, "search") {
        Ok(found) => found,
        Err(code) => return code,
    };
    let mut index = match current_index() {
        Ok(i) => i,
        Err(d) => return fail(format, "search", None, *d, vec![("query", strings(&words)), ("packages", Json::Arr(Vec::new()))]),
    };
    index.quiet = format == Format::Json;
    let read = (|| {
        if update {
            index.refresh()?;
        }
        index.entries()
    })();
    let (entries, bad) = match read {
        Ok(found) => found,
        Err(d) => return fail(format, "search", Some(&index), *d, vec![("query", strings(&words)), ("packages", Json::Arr(Vec::new()))]),
    };
    let found = index::search(&entries, &words);
    let mut warnings = Vec::new();
    if bad > 0 {
        warnings.push(Diagnostic::warning(
            "",
            t!("{bad} file(s) of the package index could not be read", "{bad} fichier(s) de l'index des paquets n'ont pas pu être lus"),
        ));
    }
    if format == Format::Json {
        let packages = Json::Arr(found.iter().map(|e| summary_json(e)).collect());
        return print_json("search", Some(&index), vec![("query", strings(&words)), ("packages", packages)], warnings);
    }
    for w in &warnings {
        report(w);
    }
    let lines: Vec<(String, String)> = found
        .iter()
        .map(|e| {
            let version = e.latest().map(|r| r.version.to_string()).unwrap_or_else(|| "-".into());
            (format!("{} {version}", e.name), clean(&e.description))
        })
        .collect();
    let width = lines.iter().map(|(l, _)| crate::diagnostic::display_width(l)).max().unwrap_or(0).min(32);
    for (left, right) in &lines {
        let pad = width.saturating_sub(crate::diagnostic::display_width(left));
        out_line(format!("{left}{}   {right}", " ".repeat(pad)).trim_end());
    }
    let (n, place, query) = (found.len(), index.place(), words.join(" "));
    if words.is_empty() {
        out_line(&t!("{n} package(s) in {place}", "{n} paquet(s) dans {place}"));
    } else if n == 0 {
        out_line(&t!("No package found for `{query}` in {place}", "Aucun paquet trouvé pour `{query}` dans {place}"));
    } else {
        out_line(&t!("{n} package(s) found for `{query}` in {place}", "{n} paquet(s) trouvé(s) pour `{query}` dans {place}"));
    }
    if let Some(first) = found.first() {
        let name = &first.name;
        out_line(&t!(
            "More about one: `rythme info {name}`; add it to a project: `rythme add {name}`",
            "Plus sur l'un d'eux : `rythme info {name}` ; l'ajouter à un projet : `rythme add {name}`"
        ));
    }
    if !update && index.updated().is_some() {
        let words = if query.is_empty() { String::new() } else { format!(" {query}") };
        out_line(&t!(
            "`rythme search --update{words}` searches the newest copy of the index",
            "`rythme search --update{words}` cherche dans la copie la plus récente de l'index"
        ));
    }
    0
}

/// A command that could not go on: its message, or its JSON document.
fn fail(format: Format, command: &str, index: Option<&Index>, d: Diagnostic, more: Vec<(&str, Json)>) -> i32 {
    match format {
        Format::Json => print_json(command, index, more, vec![d]),
        Format::Text => {
            report(&d);
            1
        }
    }
}

// ----- rythme info ----------------------------------------------------------------------

/// `rythme info NAME [--update] [--format json]`: what the index says of a package.
pub fn info_command(args: &[String]) -> i32 {
    let (format, update, rest) = match options(args, "info") {
        Ok(found) => found,
        Err(code) => return code,
    };
    let [name] = rest.as_slice() else {
        return usage(t!("which package? Example: rythme info mathlib", "quel paquet ? Exemple : rythme info mathlib"));
    };
    let mut index = match current_index() {
        Ok(i) => i,
        Err(d) => return fail(format, "info", None, *d, vec![("package", Json::Null)]),
    };
    index.quiet = format == Format::Json;
    let found = (|| {
        if update {
            index.refresh()?;
        }
        index.find(name)
    })();
    let entry = match found {
        Ok(e) => e,
        Err(d) => return fail(format, "info", Some(&index), *d, vec![("package", Json::Null)]),
    };
    if format == Format::Json {
        return print_json("info", Some(&index), vec![("package", entry_json(&entry))], Vec::new());
    }
    let latest = entry.latest();
    let version = latest.map(|r| r.version.to_string()).unwrap_or_default();
    out_line(format!("{} {version}", entry.name).trim_end());
    if !entry.description.is_empty() {
        out_line(&entry.description);
    }
    out_line("");
    let rows = [
        (t!("repository", "dépôt"), entry.repository.clone()),
        (t!("license", "licence"), entry.license.clone()),
        (t!("keywords", "mots-clés"), entry.keywords.join(", ")),
        (t!("authors", "auteurs"), entry.authors.join(", ")),
        (t!("versions", "versions"), index::versions_text(&entry)),
    ];
    let width = rows.iter().map(|(l, _)| crate::diagnostic::display_width(l)).max().unwrap_or(0);
    for (label, value) in rows.iter().filter(|(_, v)| !v.is_empty()) {
        let pad = width - crate::diagnostic::display_width(label);
        out_line(&format!("  {label}{}  {}", " ".repeat(pad), clean(value)));
    }
    for r in &entry.releases {
        if let Some(why) = &r.yanked {
            let v = &r.version;
            out_line(&t!("  {v} was withdrawn: {}", "  {v} a été retirée : {}", clean(why)));
        }
    }
    out_line("");
    match latest {
        Some(r) => {
            let (name, v) = (&entry.name, &r.version);
            out_line(&t!(
                "Add it to a project: `rythme add {name}` (writes `{name} = {{ version = \"^{v}\" }}` in rythme.toml)",
                "L'ajouter à un projet : `rythme add {name}` (écrit `{name} = {{ version = \"^{v}\" }}` dans rythme.toml)"
            ));
        }
        None => out_line(&t!("Every version of this package was withdrawn.", "Toutes les versions de ce paquet ont été retirées.")),
    }
    0
}

// ----- rythme add -----------------------------------------------------------------------

/// `rythme add NAME[@RANGE]...`: adds packages of the index to the
/// dependencies of `rythme.toml` (`{ version = "^1.2.0" }`, the newest
/// version by default), then installs them; `rythme.lock` keeps the exact commit.
pub fn add_command(args: &[String]) -> i32 {
    if let Some(code) = unknown_option(args, "add") {
        return code;
    }
    if args.is_empty() {
        return usage(t!(
            "which package? Example: rythme add mathlib (or rythme add mathlib@1.2)",
            "quel paquet ? Exemple : rythme add mathlib (ou rythme add mathlib@1.2)"
        ));
    }
    match add(args) {
        Ok(code) => code,
        Err(d) => {
            report(&d);
            1
        }
    }
}

/// `{ version = "^1.2.0" }` for `1.2.0`; a range with an operator stays as written.
fn written_range(given: &str) -> String {
    if given.starts_with(|c: char| c.is_ascii_digit()) && !given.contains(',') {
        format!("^{given}")
    } else {
        given.to_string()
    }
}

fn add(args: &[String]) -> Result<i32, Box<Diagnostic>> {
    let root = project::current_project()?;
    let m = manifest::load(&root)?;
    let mut index = index_of(Some(&m))?;
    let path = root.join(MANIFEST);
    let before = std::fs::read_to_string(&path)
        .map_err(|e| Diagnostic::error("E025", t!("cannot read `{}`: {e}", "impossible de lire `{}` : {e}", shown(&path))).boxed())?;
    let mut text = before.clone();
    let mut added = Vec::new();
    for arg in args {
        let (name, asked) = match arg.split_once('@') {
            Some((n, r)) => (n, Some(r)),
            None => (arg.as_str(), None),
        };
        index::check_name(name)?;
        if name == m.name {
            let own = &m.name;
            return Err(Diagnostic::error(
                "E025",
                t!("the project `{own}` cannot depend on itself", "le projet `{own}` ne peut pas dépendre de lui-même"),
            )
            .boxed());
        }
        // The newest copy of the index (the copy there without a network): `add` takes the newest version.
        index.refresh()?;
        let entry = index.find(name)?;
        let range_text = match asked {
            Some(r) => written_range(r.trim()),
            None => match entry.latest() {
                Some(r) => format!("^{}", r.version),
                None => return Err(index::no_version(&entry, &Range::parse("*").expect("a range"))),
            },
        };
        let Some(range) = Range::parse(&range_text) else {
            let d = Diagnostic::error(
                "E025",
                t!("`{range_text}` is not a range of versions", "`{range_text}` n'est pas un intervalle de versions"),
            );
            return Err(d.help(manifest::range_help()).boxed());
        };
        let Some(release) = index::choose(&entry, &range) else { return Err(index::no_version(&entry, &range)) };
        let line = format!("{name} = {{ version = {} }}", manifest::quote(&range_text));
        // The text of `rythme.toml` as it is now (earlier names may have been added).
        let now = manifest::read(&path).ok();
        text = with_dependency(&text, now.as_ref().map(|(doc, _)| doc), name, &line);
        std::fs::write(&path, &text)
            .map_err(|e| Diagnostic::error("E025", t!("cannot write `{}`: {e}", "impossible d'écrire `{}` : {e}", shown(&path))).boxed())?;
        if let Some(old) = m.dep(name).filter(|d| !matches!(d.source, Source::Index(_))) {
            let from = match &old.source {
                Source::Path(p) => t!("the folder `{p}`", "le dossier `{p}`"),
                Source::Git { url, .. } => format!("git `{url}`"),
                Source::Index(_) => String::new(),
            };
            out_line(&t!(
                "`{name}` came from {from}: it now comes from the package index",
                "`{name}` venait de {from} : il vient maintenant de l'index des paquets"
            ));
        }
        added.push((line, release.version.to_string(), clean(&entry.description)));
    }
    // What was written must be read back without a problem before installing.
    if let Err(d) = manifest::load(&root) {
        let _ = std::fs::write(&path, &before);
        return Err(d);
    }
    let restore = |d: Box<Diagnostic>| -> Box<Diagnostic> {
        let _ = std::fs::write(&path, &before);
        Box::new(d.note(t!("`rythme.toml` was left as it was", "`rythme.toml` a été laissé tel quel")))
    };
    project::install(None).map_err(restore)?;
    for (line, version, description) in &added {
        let name = line.split(' ').next().unwrap_or("");
        out_line(&t!("Added `{line}` to rythme.toml ({version})", "Ajouté `{line}` à rythme.toml ({version})"));
        if !description.is_empty() {
            out_line(&format!("  {name}: {description}"));
        }
        out_line(&t!(
            "  use it with `use {name}/FILE` (see `rythme info {name}`)",
            "  utilisez-le avec `use {name}/FICHIER` (voir `rythme info {name}`)"
        ));
    }
    Ok(0)
}

/// The text of `rythme.toml` with the line of a dependency: in place of the
/// one of the same name, or at the end of `[dependencies]` (made when there
/// is none). The rest of the file, comments included, stays as it was.
fn with_dependency(text: &str, doc: Option<&manifest::Doc>, name: &str, line: &str) -> String {
    let nl = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let section = doc.and_then(|d| d.sections.iter().find(|s| s.name == "dependencies" && !s.array));
    if let Some(old) = section.and_then(|s| s.entries.iter().find(|e| e.key == name)) {
        let (start, end) = (old.span.start as usize, old.span.end as usize);
        return format!("{}{line}{}", &text[..start], &text[end..]);
    }
    let Some(section) = section else {
        let sep = if text.is_empty() || text.ends_with('\n') { "" } else { nl };
        return format!("{text}{sep}{nl}[dependencies]{nl}{line}{nl}");
    };
    // The end of the last line of the section that is not empty.
    let mut at = text[section.span.start as usize..].find('\n').map_or(text.len(), |n| section.span.start as usize + n + 1);
    let mut pos = at;
    for l in text[at..].split_inclusive('\n') {
        if l.trim_start().starts_with('[') {
            break;
        }
        pos += l.len();
        if !l.trim().is_empty() {
            at = pos;
        }
    }
    let (head, tail) = text.split_at(at);
    let sep = if head.ends_with('\n') { "" } else { nl };
    format!("{head}{sep}{line}{nl}{tail}")
}

// ----- rythme publish -------------------------------------------------------------------

/// `rythme publish`: checks that the package of the current folder can go
/// into the index, and prints its entry and the steps of the pull request.
/// `rythme publish --verify FILE...`: checks entries of the index (for the
/// checks of its pull requests). Neither sends anything anywhere.
pub fn publish_command(args: &[String]) -> i32 {
    let mut rest = args.to_vec();
    if machine::take_flag(&mut rest, "--verify") {
        if let Some(code) = unknown_option(&rest, "publish --verify") {
            return code;
        }
        if rest.is_empty() {
            return usage(t!(
                "which entries? Example: rythme publish --verify packages/mathlib.toml",
                "quelles entrées ? Exemple : rythme publish --verify packages/mathlib.toml"
            ));
        }
        return verify(&rest);
    }
    if let Some(a) = rest.first() {
        return usage(t!(
            "`rythme publish` takes no argument (`{a}`): it checks the package of the current folder\nsee `rythme help packages`",
            "`rythme publish` ne prend pas d'argument (`{a}`) : il vérifie le paquet du dossier courant\nvoir `rythme help packages`"
        ));
    }
    match publish() {
        Ok(code) => code,
        Err(d) => {
            report(&d);
            1
        }
    }
}

/// The problems found before publishing, shown as they are found.
#[derive(Default)]
struct Problems(usize);

impl Problems {
    fn add(&mut self, d: Diagnostic) {
        self.0 += 1;
        report(&d);
    }
}

/// A command of Rythme run on the project, in its own process (the tests of
/// the package are its code: they may stop with `exit`).
fn rythme_in(root: &Path, args: &[&str]) -> bool {
    let Ok(exe) = std::env::current_exe() else { return false };
    Command::new(exe).args(args).current_dir(absolute(root)).env("RYTHME_LANG", crate::lang::code()).status().is_ok_and(|s| s.success())
}

/// The full path of a folder (`find_root` gives `""` for the current one).
fn absolute(dir: &Path) -> PathBuf {
    let dir = if dir.as_os_str().is_empty() { Path::new(".") } else { dir };
    dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf())
}

/// A key to write in `rythme.toml` before publishing.
fn missing_key(problems: &mut Problems, key: &str, example: String) {
    let d = Diagnostic::error("E025", t!("`rythme.toml` has no `{key}`", "`rythme.toml` n'a pas de `{key}`"));
    let d = d.note(t!("the package index shows it, and needs it", "l'index des paquets le montre, et en a besoin"));
    problems.add(d.help(t!("add a line like `{example}` at the top", "ajoutez une ligne comme `{example}` en haut")));
}

fn publish() -> Result<i32, Box<Diagnostic>> {
    let root = project::current_project()?;
    let m = manifest::load(&root)?;
    let (name, version) = (m.name.clone(), m.version.clone());
    out_line(&t!(
        "Checking `{name}` {version} before it goes into the package index",
        "Vérification de `{name}` {version} avant son entrée dans l'index des paquets"
    ));
    let mut problems = Problems::default();
    let at = |key: &str| m.span(key);

    // rythme.toml
    if Version::parse(&version).is_none() {
        let d = Diagnostic::error(
            "E025",
            t!(
                "`{version}` is not a version of semantic versioning",
                "`{version}` n'est pas une version de la gestion sémantique de versions"
            ),
        );
        let d = match at("version") {
            Some(s) => d.at(s),
            None => d,
        };
        problems.add(d.help(t!(
            "like `version = \"1.2.0\"`, without zeros at the start of a number",
            "comme `version = \"1.2.0\"`, sans zéros au début d'un nombre"
        )));
    }
    let origin = project::git(&["remote", "get-url", "origin"], &root).ok().filter(|u| index::check_address(u, false).is_ok());
    match &m.description {
        None => missing_key(
            &mut problems,
            "description",
            t!("description = \"What `{name}` is for, in one sentence\"", "description = \"À quoi sert `{name}`, en une phrase\""),
        ),
        Some(d) if d.chars().count() > 200 || d.contains('\n') => {
            let e = Diagnostic::error(
                "E025",
                t!(
                    "the `description` is too long: one sentence, at most 200 characters",
                    "la `description` est trop longue : une phrase, au plus 200 caractères"
                ),
            );
            problems.add(match at("description") {
                Some(s) => e.at(s),
                None => e,
            });
        }
        Some(_) => {}
    }
    if m.license.is_none() {
        missing_key(&mut problems, "license", "license = \"MIT\"".into());
    }
    if m.repository.is_none() {
        let example = origin.clone().unwrap_or_else(|| format!("https://github.com/{}/{name}", t!("YOU", "VOUS")));
        missing_key(&mut problems, "repository", format!("repository = \"{example}\""));
    }
    for k in &m.keywords {
        if k.is_empty() || k.chars().count() > 24 || !k.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '-') {
            let e = Diagnostic::error(
                "E025",
                t!(
                    "`{k}` is not a keyword: a word in lower case, at most 24 characters",
                    "`{k}` n'est pas un mot-clé : un mot en minuscules, au plus 24 caractères"
                ),
            );
            problems.add(match at("keywords") {
                Some(s) => e.at(s),
                None => e,
            });
        }
    }
    if m.keywords.len() > 5 {
        let e = Diagnostic::error("E025", t!("at most 5 keywords", "au plus 5 mots-clés"));
        problems.add(match at("keywords") {
            Some(s) => e.at(s),
            None => e,
        });
    }
    for dep in &m.deps {
        if let Source::Path(p) = &dep.source {
            let dep_name = &dep.name;
            let d = Diagnostic::error(
                "E025",
                t!(
                    "a published package cannot use a folder of this computer: `{dep_name}` comes from `{p}`",
                    "un paquet publié ne peut pas utiliser un dossier de cet ordinateur : `{dep_name}` vient de `{p}`"
                ),
            )
            .at(dep.span)
            .help(t!(
                "publish `{dep_name}` too, then write `{dep_name} = {{ version = \"^...\" }}`",
                "publiez aussi `{dep_name}`, puis écrivez `{dep_name} = {{ version = \"^...\" }}`"
            ));
            problems.add(d);
        }
    }

    // git: the package at the root of a repository, everything committed, a tag on the commit.
    let mut commit = None;
    let mut tag = None;
    match project::git(&["rev-parse", "--show-toplevel"], &root) {
        Err(project::GitError::Missing) => return Err(project::git_missing()),
        Err(_) => {
            let d = Diagnostic::error("E025", t!("`{name}` is not in a git repository", "`{name}` n'est pas dans un dépôt git"))
                .note(t!(
                    "the package index gives the commit of each version: a package is published from its git repository",
                    "l'index des paquets donne le commit de chaque version : un paquet se publie depuis son dépôt git"
                ))
                .help(t!(
                    "`git init`, `git add .`, `git commit`, then push it to its `repository`",
                    "`git init`, `git add .`, `git commit`, puis poussez-le vers son `repository`"
                ));
            problems.add(d);
        }
        Ok(top) => {
            let same = absolute(Path::new(&top)) == absolute(&root);
            if !same {
                let d = Diagnostic::error(
                    "E025",
                    t!("`rythme.toml` must be at the root of its git repository", "`rythme.toml` doit être à la racine de son dépôt git"),
                )
                .note(t!("the root of the repository: {top}", "la racine du dépôt : {top}"))
                .help(t!("give the package a repository of its own", "donnez au paquet un dépôt à lui"));
                problems.add(d);
            }
            let status = project::git(&["status", "--porcelain"], &root).unwrap_or_default();
            if !status.is_empty() {
                let files: Vec<&str> = status.lines().take(5).map(|l| l.get(3..).unwrap_or(l)).collect();
                let d = Diagnostic::error("E025", t!("some changes are not committed", "des changements ne sont pas validés (commit)"))
                    .note(t!("for example: {}", "par exemple : {}", files.join(", ")))
                    .help(t!(
                        "commit them (`git add`, `git commit`), or set them aside, then tag the commit",
                        "validez-les (`git add`, `git commit`), ou mettez-les de côté, puis étiquetez le commit"
                    ));
                problems.add(d);
            }
            let head = project::git(&["rev-parse", "HEAD"], &root).ok();
            let found = [format!("v{version}"), version.clone()].into_iter().find_map(|t| {
                let c = project::git(&["rev-parse", "--verify", "--quiet", &format!("refs/tags/{t}^{{commit}}")], &root).ok()?;
                Some((t, c))
            });
            match found {
                None => {
                    let d = Diagnostic::error(
                        "E025",
                        t!("no tag `v{version}` for the version {version}", "aucun tag `v{version}` pour la version {version}"),
                    )
                    .help(t!(
                        "tag the commit and push the tag: `git tag v{version}`, then `git push origin v{version}`",
                        "étiquetez le commit et poussez le tag : `git tag v{version}`, puis `git push origin v{version}`"
                    ));
                    problems.add(d);
                }
                Some((t, c)) if head.as_deref() != Some(c.as_str()) => {
                    let d = Diagnostic::error("E025", t!("the tag `{t}` is not the current commit", "le tag `{t}` n'est pas le commit courant"))
                        .note(t!("what is checked must be what is published", "ce qui est vérifié doit être ce qui est publié"))
                        .help(t!("publish from the tag (`git checkout {t}`), or move the tag to this commit (`git tag --force {t}`) before pushing it", "publiez depuis le tag (`git checkout {t}`), ou déplacez le tag sur ce commit (`git tag --force {t}`) avant de le pousser"));
                    problems.add(d);
                }
                Some((t, c)) => {
                    tag = Some(t);
                    commit = Some(c);
                }
            }
        }
    }

    // `rythme check` and `rythme test`, as anyone would run them.
    out_line(&t!("Running `rythme check`", "Lancement de `rythme check`"));
    if !rythme_in(&root, &["check"]) {
        problems.add(
            Diagnostic::error("E025", t!("`rythme check` found errors", "`rythme check` a trouvé des erreurs")).help(t!(
                "fix them first: a package must pass `rythme check`",
                "corrigez-les d'abord : un paquet doit passer `rythme check`"
            )),
        );
    }
    out_line(&t!("Running `rythme test`", "Lancement de `rythme test`"));
    if !rythme_in(&root, &["test"]) {
        problems.add(
            Diagnostic::error("E025", t!("`rythme test` failed", "`rythme test` a échoué"))
                .help(t!("fix them first: a package must pass its tests", "corrigez-les d'abord : un paquet doit réussir ses tests")),
        );
    }

    // The index: a name not taken by another repository, a version not there yet.
    let mut index = index_of(Some(&m))?;
    let mut existing = None;
    match index.refresh().and_then(|()| index.lookup(&name)) {
        Ok(Some(entry)) => {
            if m.repository.as_deref().is_some_and(|r| r.trim_end_matches('/') != entry.repository.trim_end_matches('/')) {
                let repo = &entry.repository;
                let d = Diagnostic::error(
                    "E025",
                    t!("the name `{name}` is already taken in the package index", "le nom `{name}` est déjà pris dans l'index des paquets"),
                )
                .note(t!("by the repository {repo}", "par le dépôt {repo}"))
                .help(t!("choose another `name` for your package", "choisissez un autre `name` pour votre paquet"));
                problems.add(d);
            }
            if entry.releases.iter().any(|r| r.version.to_string() == version) {
                let d = Diagnostic::error(
                    "E025",
                    t!(
                        "the version {version} of `{name}` is already in the package index",
                        "la version {version} de `{name}` est déjà dans l'index des paquets"
                    ),
                )
                .help(t!(
                    "a published version never changes: raise `version` in `rythme.toml`, commit and tag again",
                    "une version publiée ne change jamais : augmentez `version` dans `rythme.toml`, validez et étiquetez à nouveau"
                ));
                problems.add(match at("version") {
                    Some(s) => d.at(s),
                    None => d,
                });
            }
            existing = Some(entry);
        }
        Ok(None) => {}
        Err(d) => {
            let url = &index.url;
            let w = Diagnostic::warning(
                "",
                t!(
                    "cannot read the package index `{url}`: it will check the name and the version itself",
                    "impossible de lire l'index des paquets `{url}` : il vérifiera lui-même le nom et la version"
                ),
            );
            report(&w.note(d.message.clone()));
        }
    }

    // The tag must be in the repository that the index will give.
    if let (Some(t), Some(c), Some(repo)) = (&tag, &commit, &m.repository) {
        let wanted = format!("refs/tags/{t}");
        match project::git_safe(&["ls-remote", "--tags", "--", repo, &wanted], &root) {
            Ok(listing) => {
                let mut there = None;
                for l in listing.lines() {
                    if let Some((oid, r)) = l.split_once('\t') {
                        if r == format!("{wanted}^{{}}") || (r == wanted && there.is_none()) {
                            there = Some(oid.to_string());
                        }
                    }
                }
                match there {
                    None => problems.add(
                        Diagnostic::error("E025", t!("the tag `{t}` is not in {repo}", "le tag `{t}` n'est pas dans {repo}"))
                            .help(t!("push it: `git push origin {t}`", "poussez-le : `git push origin {t}`")),
                    ),
                    Some(oid) if oid != *c => problems.add(
                        Diagnostic::error(
                            "E025",
                            t!("the tag `{t}` of {repo} is not this commit", "le tag `{t}` de {repo} n'est pas ce commit"),
                        )
                        .help(t!(
                            "push the tag again: `git push --force origin {t}`",
                            "poussez à nouveau le tag : `git push --force origin {t}`"
                        )),
                    ),
                    Some(_) => {}
                }
            }
            Err(e) => {
                let w = Diagnostic::warning(
                    "",
                    t!(
                        "cannot reach {repo} to check that the tag `{t}` is there",
                        "impossible de joindre {repo} pour vérifier que le tag `{t}` y est"
                    ),
                );
                report(&match e {
                    project::GitError::Failed(said) if !said.is_empty() => w.note(t!("git says: {said}", "git dit : {said}")),
                    _ => w,
                });
            }
        }
    }

    let checksum = match &commit {
        Some(commit) => Some(index::checksum(&root, commit).map_err(|e| {
            project::git_error(
                e,
                t!("cannot read the files of the commit `{commit}`", "impossible de lire les fichiers du commit `{commit}`"),
            )
        })?),
        None => None,
    };
    let (Some(tag), Some(commit), Some(checksum)) = (tag, commit, checksum) else {
        problems.0 = problems.0.max(1);
        return Ok(finish(problems.0));
    };
    if problems.0 > 0 {
        return Ok(finish(problems.0));
    }
    let release = format!(
        "[[version]]\nversion = {}\ntag = {}\ncommit = {}\nchecksum = {}\n",
        manifest::quote(&version),
        manifest::quote(&tag),
        manifest::quote(&commit),
        manifest::quote(&checksum)
    );
    let whole = existing.is_none();
    let text = if whole {
        let list = |items: &[String]| format!("[{}]", items.iter().map(|s| manifest::quote(s)).collect::<Vec<_>>().join(", "));
        let mut s = format!("name = {}\n", manifest::quote(&name));
        for (key, value) in [("description", &m.description), ("repository", &m.repository), ("license", &m.license)] {
            if let Some(v) = value {
                s.push_str(&format!("{key} = {}\n", manifest::quote(v)));
            }
        }
        if !m.keywords.is_empty() {
            s.push_str(&format!("keywords = {}\n", list(&m.keywords)));
        }
        if !m.authors.is_empty() {
            s.push_str(&format!("authors = {}\n", list(&m.authors)));
        }
        format!("{s}\n{release}")
    } else {
        format!("\n{release}")
    };
    let file = format!("packages/{name}.toml");
    out_line("");
    out_line(&t!("✓ `{name}` {version} is ready for the package index.", "✓ `{name}` {version} est prêt pour l'index des paquets."));
    if index.is_folder() {
        // An index in a folder (a school, a team): its file is written by hand, without git.
        let url = &index.url;
        out_line(&t!(
            "The index is the folder {url}, where Rythme writes nothing.",
            "L'index est le dossier {url}, où Rythme n'écrit rien."
        ));
        if whole {
            out_line(&t!("Create its file {file} with:", "Créez son fichier {file} avec :"));
        } else {
            out_line(&t!("Add at the end of its file {file}:", "Ajoutez à la fin de son fichier {file} :"));
        }
        out_line("");
        for l in text.trim_start_matches('\n').lines() {
            out_line(format!("    {l}").trim_end());
        }
        return Ok(0);
    }
    out_line(&t!(
        "Rythme never publishes anything by itself: a version goes into the index with a pull request on its repository, {}.",
        "Rythme ne publie jamais rien lui-même : une version entre dans l'index par une pull request sur son dépôt, {}.",
        index.url
    ));
    out_line("");
    if whole {
        out_line(&t!(
            "1. Fork the repository of the index and clone your fork. Create the file {file} with:",
            "1. Faites une copie (fork) du dépôt de l'index et clonez-la. Créez le fichier {file} avec :"
        ));
    } else {
        out_line(&t!(
            "1. Fork the repository of the index and clone your fork. Add at the end of {file}:",
            "1. Faites une copie (fork) du dépôt de l'index et clonez-la. Ajoutez à la fin de {file} :"
        ));
    }
    out_line("");
    for l in text.trim_start_matches('\n').lines() {
        out_line(format!("    {l}").trim_end());
    }
    out_line("");
    out_line(&t!("2. Commit it on a branch of your fork and push it:", "2. Validez-le sur une branche de votre copie et poussez-la :"));
    out_line(&format!("    git checkout -b {name}-{version}\n    git add {file}\n    git commit -m \"{name} {version}\"\n    git push origin {name}-{version}"));
    out_line(&t!(
        "3. Open a pull request on {}: its checks run `rythme publish --verify {file}`, then a maintainer merges it.",
        "3. Ouvrez une pull request sur {} : ses vérifications lancent `rythme publish --verify {file}`, puis un responsable la fusionne.",
        index.url
    ));
    Ok(0)
}

fn finish(problems: usize) -> i32 {
    err_line(&t!(
        "\n{problems} problem(s) to fix before publishing: nothing was sent anywhere",
        "\n{problems} problème(s) à corriger avant de publier : rien n'a été envoyé nulle part"
    ));
    1
}

// ----- rythme publish --verify ----------------------------------------------------------

/// Checks entries of the index: their form, their name, and for each version
/// that its commit is in the repository, that its tag is that commit, that
/// its files have the checksum, and that its `rythme.toml` has this name and
/// this version. The code of the packages never runs.
fn verify(files: &[String]) -> i32 {
    let mut failed = 0;
    for (n, f) in files.iter().enumerate() {
        let path = PathBuf::from(f);
        let stem = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let result = (|| -> Result<usize, Box<Diagnostic>> {
            if path.extension().is_none_or(|e| e != "toml") {
                return Err(Diagnostic::error(
                    "E025",
                    t!(
                        "`{f}` is not an entry of the index: `packages/NAME.toml`",
                        "`{f}` n'est pas une entrée de l'index : `packages/NOM.toml`"
                    ),
                )
                .boxed());
            }
            index::check_name(&stem)?;
            let entry = index::read_entry(&path, &stem, true)?;
            verify_versions(&entry, n)
        })();
        match result {
            Ok(count) => out_line(&t!("✓ {f}: `{stem}`, {count} version(s) verified", "✓ {f} : `{stem}`, {count} version(s) vérifiée(s)")),
            Err(d) => {
                report(&d);
                out_line(&t!("✗ {f}", "✗ {f}"));
                failed += 1;
            }
        }
    }
    i32::from(failed > 0)
}

fn verify_versions(entry: &Entry, n: usize) -> Result<usize, Box<Diagnostic>> {
    let home = project::rythme_home()?;
    let tmp = home.join("index").join(format!(".verify-{}-{n}", std::process::id()));
    let _ = project::remove_all(&tmp);
    std::fs::create_dir_all(&tmp).map_err(|e| {
        Diagnostic::error("E025", t!("cannot write in `{}`: {e}", "impossible d'écrire dans `{}` : {e}", tmp.display())).boxed()
    })?;
    let result = (|| {
        let (name, url) = (&entry.name, &entry.repository);
        let repo = tmp.join("repo");
        let repo_text = repo.display().to_string();
        project::git_safe(&["clone", "--quiet", "--no-checkout", "--", url, &repo_text], &tmp).map_err(|e| {
            project::git_error(e, t!("cannot download `{name}` from `{url}`", "impossible de télécharger `{name}` depuis `{url}`"))
        })?;
        for r in &entry.releases {
            let (v, commit) = (&r.version, &r.commit);
            let wanted = format!("{commit}^{{commit}}");
            if !project::git_safe(&["rev-parse", "--verify", "--quiet", &wanted], &repo).is_ok_and(|c| c == *commit) {
                return Err(Diagnostic::error(
                    "E025",
                    t!("{v}: the commit `{commit}` is not in `{url}`", "{v} : le commit `{commit}` n'est pas dans `{url}`"),
                )
                .boxed());
            }
            if let Some(tag) = &r.tag {
                let at = project::git_safe(&["rev-parse", "--verify", "--quiet", &format!("refs/tags/{tag}^{{commit}}")], &repo).ok();
                if at.as_deref() != Some(commit.as_str()) {
                    return Err(Diagnostic::error(
                        "E025",
                        t!(
                            "{v}: the tag `{tag}` of `{url}` is not the commit `{commit}`",
                            "{v} : le tag `{tag}` de `{url}` n'est pas le commit `{commit}`"
                        ),
                    )
                    .boxed());
                }
            }
            let found = index::checksum(&repo, commit).map_err(|e| {
                project::git_error(
                    e,
                    t!("cannot read the files of the commit `{commit}`", "impossible de lire les fichiers du commit `{commit}`"),
                )
            })?;
            if found != r.checksum {
                let d = Diagnostic::error(
                    "E025",
                    t!(
                        "{v}: the files of the commit do not have the checksum of the entry",
                        "{v} : les fichiers du commit n'ont pas la somme de contrôle de l'entrée"
                    ),
                )
                .note(t!("expected: {}\nfound:    {found}", "attendue : {}\ntrouvée :  {found}", r.checksum));
                return Err(d
                    .help(t!(
                        "`rythme publish` in the package, at its tag, writes the right one",
                        "`rythme publish` dans le paquet, à son tag, écrit la bonne"
                    ))
                    .boxed());
            }
            // Its rythme.toml, read like any other (never run).
            let toml = project::git_bytes(&["cat-file", "blob", &format!("{commit}:{MANIFEST}")], &repo).map_err(|_| {
                Diagnostic::error(
                    "E025",
                    t!("{v}: the commit has no `rythme.toml` at its root", "{v} : le commit n'a pas de `rythme.toml` à sa racine"),
                )
                .boxed()
            })?;
            let dir = tmp.join(format!("manifest-{v}"));
            std::fs::create_dir_all(&dir).and_then(|()| std::fs::write(dir.join(MANIFEST), &toml)).map_err(|e| {
                Diagnostic::error("E025", t!("cannot write in `{}`: {e}", "impossible d'écrire dans `{}` : {e}", dir.display())).boxed()
            })?;
            let m = manifest::load(&dir)?;
            if m.name != *name || m.version != v.to_string() {
                let (found_name, found_version) = (&m.name, &m.version);
                return Err(Diagnostic::error(
                    "E025",
                    t!(
                        "{v}: the `rythme.toml` of the commit says `{found_name}` {found_version}",
                        "{v} : le `rythme.toml` du commit dit `{found_name}` {found_version}"
                    ),
                )
                .boxed());
            }
            if let Some(dep) = m.deps.iter().find(|d| matches!(d.source, Source::Path(_))) {
                let dep_name = &dep.name;
                return Err(Diagnostic::error(
                    "E025",
                    t!(
                        "{v}: the dependency `{dep_name}` is a folder of the computer of the author",
                        "{v} : la dépendance `{dep_name}` est un dossier de l'ordinateur de l'auteur"
                    ),
                )
                .boxed());
            }
        }
        Ok(entry.releases.len())
    })();
    let _ = project::remove_all(&tmp);
    result
}
