//! The package index: finding and sharing packages by name, with no server
//! of Rythme's own.
//!
//! The index is a git repository that anyone can read, with one file per
//! package, `packages/NAME.toml` (the same small TOML as `rythme.toml`):
//!
//! ```toml
//! name = "mathlib"
//! description = "Vectors, matrices and a few numbers"
//! repository = "https://github.com/someone/mathlib"
//! license = "MIT"
//! keywords = ["math", "vectors"]
//! authors = ["Someone <someone@example.com>"]
//!
//! [[version]]
//! version = "1.0.0"
//! tag = "v1.0.0"
//! commit = "4d868c2b..."        # the full commit of the tag
//! checksum = "sha256:..."       # of the files of that commit: see `checksum`
//! ```
//!
//! Rythme downloads it with git into `RYTHME_HOME/index/` and reads that
//! copy: `rythme search`, `info`, `add` and `install` work without a network
//! once it is there, and `--update` (or `rythme update`) gets the newest one.
//! A folder of this computer can be an index too, read as it is. Publishing
//! a version is a pull request on the repository of the index: `rythme
//! publish` checks the package and writes its entry, and never sends
//! anything itself.
//!
//! What comes from the index is not trusted: names are checked before they
//! become paths, addresses must use https, ssh or a folder of this computer,
//! git runs with the safe options of `project::git_safe`, the files of a
//! version must have the checksum of the index, and nothing of a package
//! runs when it is installed.

use std::path::{Path, PathBuf};

use crate::diagnostic::Diagnostic;
use crate::manifest::{self, is_package_name, Doc, Value};
use crate::project::{self, GitError};
use crate::semver::{Range, Version};
use crate::sha256::Sha256;

/// The index used when neither `rythme.toml` (`index = "..."`) nor the
/// variable `RYTHME_INDEX` gives another one.
pub const DEFAULT_INDEX: &str = "https://github.com/maxlestage/rythme-index";

/// The longest name of a package, and the largest file of the index read.
const MAX_NAME: usize = 64;
const MAX_ENTRY: u64 = 64 * 1024;

// ----- addresses -------------------------------------------------------------------

/// `git@github.com:someone/mathlib`: a host before `:`, with no `/` before
/// it (`C:\...`, a drive of Windows, is a folder).
fn is_scp(a: &str) -> bool {
    a.split_once(':').is_some_and(|(host, _)| host.len() > 1 && !host.contains(['/', '\\']))
}

/// Is this address a folder of this computer (no `://`, not `host:path`)?
fn is_folder(a: &str) -> bool {
    !a.contains("://") && !is_scp(a)
}

/// Can git be given this address? Only https, ssh and the files of this
/// computer (`file://`); a folder written as a path too when `folder` is true.
/// The message says why not.
pub fn check_address(addr: &str, folder: bool) -> Result<(), String> {
    let a = addr.trim();
    if a.is_empty() {
        return Err(t!("the address is empty", "l'adresse est vide"));
    }
    // A folder may have spaces in its name; an address may not.
    if a.starts_with('-') || a.chars().any(|c| c.is_control() || (c == ' ' && !(folder && is_folder(a)))) {
        return Err(t!(
            "`{a}` is not an address that Rythme accepts: no spaces, and no `-` at the start",
            "`{a}` n'est pas une adresse que Rythme accepte : pas d'espaces, et pas de `-` au début"
        ));
    }
    let scheme = a.split_once("://").map(|(s, _)| s.to_ascii_lowercase()).or_else(|| a.split_once("::").map(|(s, _)| s.to_string()));
    match scheme.as_deref() {
        Some("https" | "ssh" | "file") => Ok(()),
        Some(other) => Err(t!(
            "Rythme downloads packages only with `https://`, `ssh://` or from this computer (`file://`), not with `{other}`",
            "Rythme ne télécharge les paquets qu'avec `https://`, `ssh://` ou depuis cet ordinateur (`file://`), pas avec `{other}`"
        )),
        None if is_scp(a) || folder => Ok(()),
        None => Err(t!(
            "`{a}` is not the address of a repository: write `https://...`, `ssh://...` or `file://...`",
            "`{a}` n'est pas l'adresse d'un dépôt : écrivez `https://...`, `ssh://...` ou `file://...`"
        )),
    }
}

/// A text from the index as it may be shown: no control characters, which
/// could move the cursor or change the colors of the terminal.
pub fn clean(s: &str) -> String {
    s.chars().map(|c| if c.is_control() { ' ' } else { c }).collect()
}

// ----- the index and its copy ------------------------------------------------------

/// Where the address of the index comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setting {
    /// `index = "..."` in `rythme.toml`.
    Project,
    /// The variable `RYTHME_INDEX`.
    Variable,
    Default,
}

pub struct Index {
    /// The address, as given.
    pub url: String,
    pub setting: Setting,
    /// The folder read: the index itself (a folder of this computer), or its copy.
    folder: PathBuf,
    /// Is `folder` the index itself (no copy, no git)?
    local: bool,
    /// The file that holds the time of the last download of the copy.
    stamp: PathBuf,
    /// The copy was downloaded (or tried) by this command.
    refreshed: bool,
    /// Keep the warnings in `warnings` instead of showing them (`--format json`).
    pub quiet: bool,
    pub warnings: Vec<Diagnostic>,
}

/// The time now, in seconds since 1970.
fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// `2026-10-30` (with the time: `2026-10-30T14:05:00Z`), in UTC.
pub fn date(secs: u64, with_time: bool) -> String {
    let days = (secs / 86_400) as i64;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    if with_time {
        let s = secs % 86_400;
        format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", s / 3600, s / 60 % 60, s % 60)
    } else {
        format!("{y:04}-{m:02}-{d:02}")
    }
}

fn said(e: GitError) -> Option<String> {
    match e {
        GitError::Missing => None,
        GitError::Failed(s) => Some(s),
    }
}

impl Index {
    /// The index of a project (its `index` and its folder), or else of the
    /// variable `RYTHME_INDEX`, or else the usual one. Nothing is downloaded.
    pub fn new(project: Option<(&str, &Path)>) -> Result<Index, Box<Diagnostic>> {
        let cwd = std::env::current_dir().unwrap_or_default();
        let variable = std::env::var("RYTHME_INDEX").ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
        let (url, setting, base) = match (project, variable) {
            (Some((s, dir)), _) => (s.trim().to_string(), Setting::Project, dir.to_path_buf()),
            (None, Some(v)) => (v, Setting::Variable, cwd),
            (None, None) => (DEFAULT_INDEX.to_string(), Setting::Default, cwd),
        };
        let url = url.trim_end_matches('/').to_string();
        let set_by = |d: Diagnostic| -> Box<Diagnostic> {
            match setting {
                Setting::Project => d.note(t!("set by `index` in `rythme.toml`", "donnée par `index` dans `rythme.toml`")),
                Setting::Variable => d.note(t!("set by the variable RYTHME_INDEX", "donnée par la variable RYTHME_INDEX")),
                Setting::Default => d,
            }
            .boxed()
        };
        if let Err(msg) = check_address(&url, true) {
            return Err(set_by(Diagnostic::error("E025", msg)));
        }
        if is_folder(&url) {
            let folder = if Path::new(&url).is_absolute() { PathBuf::from(&url) } else { base.join(&url) };
            if !folder.join("packages").is_dir() {
                let d = Diagnostic::error(
                    "E025",
                    t!(
                        "`{url}` is not a package index: it has no folder `packages`",
                        "`{url}` n'est pas un index de paquets : il n'a pas de dossier `packages`"
                    ),
                )
                .help(t!(
                    "an index holds one file per package, like `packages/mathlib.toml` (see `rythme help packages`)",
                    "un index contient un fichier par paquet, comme `packages/mathlib.toml` (voir `rythme help packages`)"
                ));
                return Err(set_by(d));
            }
            return Ok(Index {
                url,
                setting,
                folder,
                local: true,
                stamp: PathBuf::new(),
                refreshed: true,
                quiet: false,
                warnings: Vec::new(),
            });
        }
        let home = project::rythme_home()?.join("index");
        let key = project::fingerprint(&url);
        Ok(Index {
            folder: home.join(&key),
            stamp: home.join(format!("{key}.updated")),
            url,
            setting,
            local: false,
            refreshed: false,
            quiet: false,
            warnings: Vec::new(),
        })
    }

    /// Is the index a folder of this computer, read as it is?
    pub fn is_folder(&self) -> bool {
        self.local
    }

    /// Is there something to read: the index itself, or a copy of it?
    fn has_copy(&self) -> bool {
        self.local || (self.folder.join(".git").is_dir() && self.folder.join("packages").is_dir())
    }

    /// When the copy was downloaded, in seconds since 1970 (nothing for a
    /// folder read as it is, or before the first download).
    pub fn updated(&self) -> Option<u64> {
        if self.local {
            return None;
        }
        let text = std::fs::read_to_string(&self.stamp).ok()?;
        let mut lines = text.lines();
        (lines.next() == Some(self.url.as_str())).then_some(())?;
        lines.next()?.trim().parse().ok()
    }

    /// `the copy of 2026-10-30`, for messages.
    pub fn copy_date(&self) -> String {
        match self.updated() {
            Some(secs) => date(secs, false),
            None => t!("an unknown day", "un jour inconnu"),
        }
    }

    fn warn(&mut self, d: Diagnostic) {
        if self.quiet {
            self.warnings.push(d);
        } else {
            crate::diagnostic::show(&d, crate::diagnostic::use_color());
        }
    }

    /// Downloads the copy (or brings it up to date) with git.
    fn download(&self) -> Result<(), Option<String>> {
        let cannot =
            |e: std::io::Error, p: &Path| Some(t!("cannot write in `{}`: {e}", "impossible d'écrire dans `{}` : {e}", p.display()));
        let parent = self.folder.parent().map(Path::to_path_buf).unwrap_or_default();
        std::fs::create_dir_all(&parent).map_err(|e| cannot(e, &parent))?;
        if self.folder.join(".git").is_dir() {
            project::git_safe(&["fetch", "--quiet", "--depth", "1", "--", &self.url, "HEAD"], &self.folder).map_err(said)?;
            project::git_safe(&["checkout", "--quiet", "--force", "--detach", "FETCH_HEAD"], &self.folder).map_err(said)?;
            project::git_safe(&["clean", "--quiet", "--force", "-d", "-x"], &self.folder).map_err(said)?;
        } else {
            let leaf = self.folder.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
            let tmp = parent.join(format!(".download-{leaf}-{}", std::process::id()));
            project::remove_all(&tmp).map_err(|e| cannot(e, &tmp))?;
            let tmp_text = tmp.display().to_string();
            if let Err(e) = project::git_safe(&["clone", "--quiet", "--depth", "1", "--", &self.url, &tmp_text], &parent) {
                let _ = project::remove_all(&tmp);
                return Err(said(e));
            }
            project::remove_all(&self.folder).map_err(|e| cannot(e, &self.folder))?;
            std::fs::rename(&tmp, &self.folder).map_err(|e| cannot(e, &self.folder))?;
        }
        std::fs::write(&self.stamp, format!("{}\n{}\n", self.url, now())).map_err(|e| cannot(e, &self.stamp))?;
        Ok(())
    }

    /// Gets the newest copy of the index (once per command). Without a
    /// network, the copy there is used, with a warning; without a copy, it
    /// is an error.
    pub fn refresh(&mut self) -> Result<(), Box<Diagnostic>> {
        if self.local || self.refreshed {
            return Ok(());
        }
        self.refreshed = true;
        let url = self.url.clone();
        match self.download() {
            Ok(()) => Ok(()),
            Err(why) if self.has_copy() => {
                let date = self.copy_date();
                let mut d = Diagnostic::warning(
                    "",
                    t!(
                        "cannot update the package index from `{url}`: using the copy of {date}",
                        "impossible de mettre à jour l'index des paquets depuis `{url}` : la copie du {date} est utilisée"
                    ),
                );
                if let Some(said) = why.filter(|w| !w.is_empty()) {
                    d = d.note(t!("git says: {said}", "git dit : {said}"));
                }
                self.warn(d);
                Ok(())
            }
            Err(why) => {
                if why.is_none() && git_is_missing() {
                    return Err(project::git_missing());
                }
                let mut d = Diagnostic::error(
                    "E025",
                    t!("cannot download the package index from `{url}`", "impossible de télécharger l'index des paquets depuis `{url}`"),
                );
                if let Some(said) = why.filter(|w| !w.is_empty()) {
                    d = d.note(t!("git says: {said}", "git dit : {said}"));
                }
                Err(d
                    .help(t!(
                        "check the connection to the network, or give another index with the variable RYTHME_INDEX (an address or a folder); dependencies from git or from a folder work without the index",
                        "vérifiez la connexion au réseau, ou donnez un autre index avec la variable RYTHME_INDEX (une adresse ou un dossier) ; les dépendances venant de git ou d'un dossier marchent sans l'index"
                    ))
                    .boxed())
            }
        }
    }

    /// Makes sure there is something to read: downloads the copy the first time.
    pub fn ready(&mut self) -> Result<(), Box<Diagnostic>> {
        if self.has_copy() {
            Ok(())
        } else {
            self.refresh()
        }
    }

    fn entry_path(&self, name: &str) -> PathBuf {
        self.folder.join("packages").join(format!("{name}.toml"))
    }

    /// The entry of a package. When the copy does not have it, the newest
    /// copy is downloaded once (the package may be new).
    pub fn find(&mut self, name: &str) -> Result<Entry, Box<Diagnostic>> {
        check_name(name)?;
        self.ready()?;
        if !self.entry_path(name).is_file() {
            self.refresh()?;
        }
        let path = self.entry_path(name);
        if path.is_file() {
            return read_entry(&path, name, false);
        }
        let names = self.names();
        let mut d =
            Diagnostic::error("E025", t!("no package `{name}` in the package index", "aucun paquet `{name}` dans l'index des paquets"))
                .note(self.described());
        if let Some(best) = crate::suggest::closest(name, names.iter().map(String::as_str)) {
            d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
        } else {
            d = d.help(t!(
                "`rythme search WORDS` finds packages by the words of their name and description",
                "`rythme search MOTS` trouve des paquets par les mots de leur nom et de leur description"
            ));
        }
        Err(d.boxed())
    }

    /// `the index: https://... (copy of 2026-10-30)`, for messages.
    pub fn described(&self) -> String {
        let place = self.place();
        t!("the index: {place}", "l'index : {place}")
    }

    /// `https://... (copy of 2026-10-30)`: the index and the day of its copy.
    pub fn place(&self) -> String {
        let url = &self.url;
        if self.local {
            url.clone()
        } else {
            let date = self.copy_date();
            t!("{url} (copy of {date})", "{url} (copie du {date})")
        }
    }

    /// The entry of a package, or nothing when the index does not have it.
    pub fn lookup(&mut self, name: &str) -> Result<Option<Entry>, Box<Diagnostic>> {
        check_name(name)?;
        self.ready()?;
        let path = self.entry_path(name);
        if path.is_file() {
            read_entry(&path, name, false).map(Some)
        } else {
            Ok(None)
        }
    }

    /// The names of the packages of the index.
    fn names(&self) -> Vec<String> {
        let Ok(rd) = std::fs::read_dir(self.folder.join("packages")) else { return Vec::new() };
        let mut names: Vec<String> = rd
            .flatten()
            .filter_map(|e| e.file_name().to_string_lossy().strip_suffix(".toml").map(str::to_string))
            .filter(|n| is_package_name(n) && n.len() <= MAX_NAME)
            .collect();
        names.sort();
        names
    }

    /// Every entry of the index, by name, and the number of files that could not be read.
    pub fn entries(&mut self) -> Result<(Vec<Entry>, usize), Box<Diagnostic>> {
        self.ready()?;
        let mut out = Vec::new();
        let mut bad = 0;
        for name in self.names() {
            match read_entry(&self.entry_path(&name), &name, false) {
                Ok(e) => out.push(e),
                Err(_) => bad += 1,
            }
        }
        Ok((out, bad))
    }
}

/// Is `git` missing? (A failed download said nothing.)
fn git_is_missing() -> bool {
    matches!(project::git_safe(&["--version"], Path::new("")), Err(GitError::Missing))
}

/// Checks a name of package given on the command line: it becomes the name
/// of a file of the index, so only Rythme names are accepted (no `/`, no `..`).
pub fn check_name(name: &str) -> Result<(), Box<Diagnostic>> {
    if is_package_name(name) && name.len() <= MAX_NAME {
        return Ok(());
    }
    let (msg, better) = manifest::bad_name(name);
    let mut d = Diagnostic::error("E025", msg);
    if let Some(best) = better.filter(|b| b.len() <= MAX_NAME) {
        d = d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?"));
    }
    Err(d.boxed())
}

// ----- entries -----------------------------------------------------------------------

/// A package of the index.
pub struct Entry {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub license: String,
    pub keywords: Vec<String>,
    pub authors: Vec<String>,
    /// Newest first.
    pub releases: Vec<Release>,
}

/// One version of a package of the index.
pub struct Release {
    pub version: Version,
    pub tag: Option<String>,
    /// The full commit, in hexadecimal.
    pub commit: String,
    /// `sha256:` and 64 hexadecimal digits: see [`checksum`].
    pub checksum: String,
    /// Why this version should no longer be chosen, when it should not.
    pub yanked: Option<String>,
}

impl Entry {
    /// The newest version that is not withdrawn.
    pub fn latest(&self) -> Option<&Release> {
        self.releases
            .iter()
            .find(|r| r.yanked.is_none() && !r.version.is_prerelease())
            .or_else(|| self.releases.iter().find(|r| r.yanked.is_none()))
    }
}

/// The newest version of the entry in the range, not withdrawn.
pub fn choose<'a>(entry: &'a Entry, range: &Range) -> Option<&'a Release> {
    entry.releases.iter().find(|r| r.yanked.is_none() && range.matches(&r.version))
}

/// The versions of an entry, for messages: `1.2.0, 1.1.0 (withdrawn), 1.0.0`.
pub fn versions_text(entry: &Entry) -> String {
    let list: Vec<String> = entry
        .releases
        .iter()
        .map(|r| match r.yanked {
            Some(_) => t!("{} (withdrawn)", "{} (retirée)", r.version),
            None => r.version.to_string(),
        })
        .collect();
    if list.is_empty() {
        t!("none", "aucune")
    } else {
        list.join(", ")
    }
}

/// No version of the entry is in the range.
pub fn no_version(entry: &Entry, range: &Range) -> Box<Diagnostic> {
    let name = &entry.name;
    Diagnostic::error(
        "E025",
        t!(
            "no version of `{name}` in the package index fits `{range}`",
            "aucune version de `{name}` de l'index des paquets ne va avec `{range}`"
        ),
    )
    .note(t!("its versions: {}", "ses versions : {}", versions_text(entry)))
    .help(match entry.latest() {
        Some(r) => {
            let v = &r.version;
            t!(
                "for the newest one, write `{name} = {{ version = \"^{v}\" }}`",
                "pour la plus récente, écrivez `{name} = {{ version = \"^{v}\" }}`"
            )
        }
        None => t!("every version of `{name}` was withdrawn", "toutes les versions de `{name}` ont été retirées"),
    })
    .boxed()
}

fn is_hex(s: &str, lens: &[usize]) -> bool {
    lens.contains(&s.len()) && s.chars().all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

/// Is this a checksum as the index writes it: `sha256:` and 64 hexadecimal digits?
pub fn is_checksum(s: &str) -> bool {
    s.strip_prefix("sha256:").is_some_and(|h| is_hex(h, &[64]))
}

/// Is this a full commit: 40 hexadecimal digits (64 in a repository of SHA-256)?
pub fn is_commit(s: &str) -> bool {
    is_hex(s, &[40, 64])
}

const ENTRY_KEYS: &[&str] = &["name", "description", "repository", "license", "keywords", "authors"];
const VERSION_KEYS: &[&str] = &["version", "tag", "commit", "checksum", "yanked"];

/// Reads the file of the index of the package `name`. `strict` (for `rythme
/// publish --verify`): every key known, every field there, nothing written twice.
pub fn read_entry(path: &Path, name: &str, strict: bool) -> Result<Entry, Box<Diagnostic>> {
    let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    if size > MAX_ENTRY {
        let shown = project::shown(path);
        return Err(Diagnostic::error(
            "E025",
            t!(
                "`{shown}` is too large for an entry of the package index",
                "`{shown}` est trop grand pour une entrée de l'index des paquets"
            ),
        )
        .boxed());
    }
    let (doc, file) = manifest::read(path)?;
    entry_from(&doc, file, name, strict)
}

fn problem(msg: String, span: crate::source::Span) -> Box<Diagnostic> {
    Diagnostic::error("E025", msg).at(span).boxed()
}

fn entry_from(doc: &Doc, file: u32, name: &str, strict: bool) -> Result<Entry, Box<Diagnostic>> {
    let whole = crate::source::Span::new(file, 0, 0);
    let mut e = Entry {
        name: String::new(),
        description: String::new(),
        repository: String::new(),
        license: String::new(),
        keywords: Vec::new(),
        authors: Vec::new(),
        releases: Vec::new(),
    };
    let text = |x: &manifest::Entry| manifest::text_of(x, &t!("a text", "un texte")).map(clean);
    let list = |x: &manifest::Entry| match &x.value {
        Value::List(items) => Ok(items.iter().map(|s| clean(s)).collect::<Vec<_>>()),
        _ => {
            let key = &x.key;
            Err(problem(t!("`{key}` is a list of texts", "`{key}` est une liste de textes"), x.span))
        }
    };
    for x in &doc.top {
        match x.key.as_str() {
            "name" => e.name = text(x)?,
            "description" => e.description = text(x)?,
            "repository" => {
                let r = text(x)?;
                check_address(&r, false).map_err(|msg| problem(msg, x.span))?;
                e.repository = r;
            }
            "license" => e.license = text(x)?,
            "keywords" => e.keywords = list(x)?,
            "authors" => e.authors = list(x)?,
            key if strict => return Err(manifest::unknown_key(key, x.key_span, ENTRY_KEYS)),
            _ => {}
        }
    }
    if e.name != name {
        let found = &e.name;
        return Err(problem(
            t!(
                "this entry of the package index is for `{found}`, but its file is named for `{name}`",
                "cette entrée de l'index des paquets est pour `{found}`, mais son fichier porte le nom de `{name}`"
            ),
            doc.top.iter().find(|x| x.key == "name").map_or(whole, |x| x.span),
        ));
    }
    if e.repository.is_empty() {
        return Err(problem(t!("the entry of `{name}` has no `repository`", "l'entrée de `{name}` n'a pas de `repository`"), whole));
    }
    if strict {
        for (key, empty) in [("description", e.description.is_empty()), ("license", e.license.is_empty())] {
            if empty {
                return Err(problem(t!("the entry of `{name}` has no `{key}`", "l'entrée de `{name}` n'a pas de `{key}`"), whole));
            }
        }
    }
    for s in &doc.sections {
        if s.name != "version" || !s.array {
            if strict {
                let n = &s.name;
                let d = Diagnostic::error("E025", t!("unknown section `{n}`", "section inconnue `{n}`"))
                    .at(s.span)
                    .help(t!("each version is written under `[[version]]`", "chaque version s'écrit sous `[[version]]`"));
                return Err(d.boxed());
            }
            continue;
        }
        let mut get: Vec<(&str, String, crate::source::Span)> = Vec::new();
        for x in &s.entries {
            if !VERSION_KEYS.contains(&x.key.as_str()) {
                if strict {
                    return Err(manifest::unknown_key(&x.key, x.key_span, VERSION_KEYS));
                }
                continue;
            }
            get.push((VERSION_KEYS.iter().find(|k| **k == x.key).copied().unwrap_or(""), text(x)?, x.span));
        }
        let field = |k: &str| get.iter().find(|(key, _, _)| *key == k).map(|(_, v, span)| (v.clone(), *span));
        let missing = |k: &str| problem(t!("this version of `{name}` has no `{k}`", "cette version de `{name}` n'a pas de `{k}`"), s.span);
        let (v, at) = field("version").ok_or_else(|| missing("version"))?;
        let Some(version) = Version::parse(&v) else {
            return Err(problem(t!("`{v}` is not a version", "`{v}` n'est pas une version"), at));
        };
        let (commit, at) = field("commit").ok_or_else(|| missing("commit"))?;
        if !is_commit(&commit) {
            return Err(problem(
                t!(
                    "`{commit}` is not a full commit: 40 hexadecimal digits in lower case",
                    "`{commit}` n'est pas un commit complet : 40 chiffres hexadécimaux en minuscules"
                ),
                at,
            ));
        }
        let (checksum, at) = field("checksum").ok_or_else(|| missing("checksum"))?;
        if !is_checksum(&checksum) {
            return Err(problem(
                t!(
                    "`{checksum}` is not a checksum: `sha256:` and 64 hexadecimal digits, as `rythme publish` writes it",
                    "`{checksum}` n'est pas une somme de contrôle : `sha256:` et 64 chiffres hexadécimaux, comme l'écrit `rythme publish`"
                ),
                at,
            ));
        }
        let tag = match field("tag") {
            Some((t, at)) if t.is_empty() || t.starts_with('-') || t.contains(' ') => {
                return Err(problem(t!("`{t}` is not a tag", "`{t}` n'est pas un tag"), at));
            }
            Some((t, _)) => Some(t),
            None if strict => return Err(missing("tag")),
            None => None,
        };
        if e.releases.iter().any(|r: &Release| r.version == version) {
            return Err(problem(t!("the version {v} is written twice", "la version {v} est écrite deux fois"), s.span));
        }
        e.releases.push(Release { version, tag, commit, checksum, yanked: field("yanked").map(|(y, _)| y) });
    }
    if strict && e.releases.is_empty() {
        let d = Diagnostic::error("E025", t!("the entry of `{name}` has no version", "l'entrée de `{name}` n'a pas de version"))
            .at(whole)
            .help(t!("each version is written under `[[version]]`", "chaque version s'écrit sous `[[version]]`"));
        return Err(d.boxed());
    }
    e.releases.sort_by(|a, b| b.version.cmp(&a.version));
    Ok(e)
}

// ----- searching -----------------------------------------------------------------------

/// The entries that have every word in their name, description or keywords,
/// the best first: a word that is the name, then one in the name, then a
/// keyword, then a word of the description.
pub fn search<'a>(entries: &'a [Entry], words: &[String]) -> Vec<&'a Entry> {
    let words: Vec<String> = words.iter().map(|w| w.trim().to_lowercase()).filter(|w| !w.is_empty()).collect();
    let mut found: Vec<(u32, &Entry)> = entries
        .iter()
        .filter_map(|e| {
            let description = e.description.to_lowercase();
            let mut score = 0;
            for w in &words {
                let keyword = |exact: bool| {
                    e.keywords.iter().any(|k| if exact { k.to_lowercase() == *w } else { k.to_lowercase().contains(w.as_str()) })
                };
                score += match () {
                    _ if e.name == *w => 100,
                    _ if e.name.contains(w.as_str()) => 50,
                    _ if keyword(true) => 30,
                    _ if keyword(false) => 20,
                    _ if description.contains(w.as_str()) => 10,
                    _ => return None,
                };
            }
            Some((score, e))
        })
        .collect();
    found.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.name.cmp(&b.1.name)));
    found.into_iter().map(|(_, e)| e).collect()
}

// ----- checksums -----------------------------------------------------------------------

/// The checksum of the files of a commit of the repository `dir`, as the
/// index writes it: `sha256:` and the SHA-256 of, for each file in the order
/// of the bytes of its path, its mode and its path followed by a zero byte,
/// then its size in bytes followed by a zero byte and its content (for a
/// submodule: its commit followed by a zero byte). It is computed from what
/// git keeps, so it is the same on every system, whatever git does to the
/// ends of lines when it writes the files.
pub(crate) fn checksum(dir: &Path, commit: &str) -> Result<String, GitError> {
    /// One line of `git ls-tree`: `mode type object\tpath`.
    struct File<'a> {
        mode: &'a [u8],
        blob: bool,
        object: &'a [u8],
        path: &'a [u8],
    }
    let listing = project::git_bytes(&["ls-tree", "-r", "-z", "--full-tree", commit], dir)?;
    // Each line ends with a zero byte.
    let mut files: Vec<File> = Vec::new();
    for item in listing.split(|b| *b == 0).filter(|i| !i.is_empty()) {
        let Some(tab) = item.iter().position(|b| *b == b'\t') else { continue };
        let (head, path) = (&item[..tab], &item[tab + 1..]);
        let mut parts = head.split(|b| *b == b' ');
        let (Some(mode), Some(kind), Some(object)) = (parts.next(), parts.next(), parts.next()) else { continue };
        files.push(File { mode, blob: kind == b"blob", object, path });
    }
    files.sort_by(|a, b| a.path.cmp(b.path));
    let blobs: Vec<&[u8]> = files.iter().filter(|f| f.blob).map(|f| f.object).collect();
    let contents = cat_files(dir, &blobs)?;
    let mut h = Sha256::new();
    let mut next = contents.iter();
    for f in &files {
        h.update(f.mode);
        h.update(b" ");
        h.update(f.path);
        h.update(b"\0");
        if f.blob {
            let content = next.next().ok_or_else(|| GitError::Failed("cat-file".into()))?;
            h.update(content.len().to_string().as_bytes());
            h.update(b"\0");
            h.update(content);
        } else {
            h.update(f.object);
            h.update(b"\0");
        }
    }
    Ok(format!("sha256:{}", h.hex()))
}

/// The contents of these objects of git, in order (`git cat-file --batch`).
fn cat_files(dir: &Path, objects: &[&[u8]]) -> Result<Vec<Vec<u8>>, GitError> {
    use std::io::{Read, Write};
    if objects.is_empty() {
        return Ok(Vec::new());
    }
    let mut child = project::git_command(&["cat-file", "--batch"], dir, true)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound { GitError::Missing } else { GitError::Failed(e.to_string()) })?;
    let mut input = Vec::new();
    for o in objects {
        input.extend_from_slice(o);
        input.push(b'\n');
    }
    let mut stdin = child.stdin.take().ok_or_else(|| GitError::Failed("stdin".into()))?;
    // Written by another thread: git answers while it reads, and its answer could fill the pipe.
    let writer = std::thread::spawn(move || stdin.write_all(&input));
    let mut out = Vec::new();
    child
        .stdout
        .take()
        .ok_or_else(|| GitError::Failed("stdout".into()))?
        .read_to_end(&mut out)
        .map_err(|e| GitError::Failed(e.to_string()))?;
    let _ = writer.join();
    let status = child.wait().map_err(|e| GitError::Failed(e.to_string()))?;
    if !status.success() {
        return Err(GitError::Failed("git cat-file".into()));
    }
    let mut contents = Vec::new();
    let mut rest = &out[..];
    for _ in objects {
        // `object blob size\n`, the content, then `\n`.
        let end = rest.iter().position(|b| *b == b'\n').ok_or_else(|| GitError::Failed("cat-file".into()))?;
        let head = String::from_utf8_lossy(&rest[..end]).to_string();
        let size: usize = head.rsplit(' ').next().and_then(|s| s.parse().ok()).ok_or_else(|| GitError::Failed(head.clone()))?;
        let start = end + 1;
        if rest.len() < start + size + 1 {
            return Err(GitError::Failed(head));
        }
        contents.push(rest[start..start + size].to_vec());
        rest = &rest[start + size + 1..];
    }
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::{check_address, date, is_folder};

    #[test]
    fn addresses_are_checked() {
        for ok in ["https://github.com/a/b", "ssh://git@host/a.git", "git@github.com:a/b.git", "file:///tmp/a", "HTTPS://x/y"] {
            assert!(check_address(ok, false).is_ok(), "{ok}");
        }
        for bad in ["http://x/y", "git://x/y", "ext::sh -c touch% /tmp/x", "-u", "fd::17", "../local", "/abs/path", "", "a b"] {
            assert!(check_address(bad, false).is_err(), "{bad}");
        }
        assert!(check_address("../index", true).is_ok());
        assert!(check_address(r"C:\Users\a\index", true).is_ok() && is_folder(r"C:\Users\a\index"));
        assert!(!is_folder("git@github.com:a/b") && !is_folder("https://x/y") && is_folder("/srv/index"));
    }

    #[test]
    fn dates_in_utc() {
        assert_eq!(date(0, true), "1970-01-01T00:00:00Z");
        assert_eq!(date(951_782_400, false), "2000-02-29");
        assert_eq!(date(1_793_440_000, true), "2026-10-31T09:46:40Z");
    }
}
