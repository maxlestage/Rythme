//! Projects and packages: `rythme new`, `rythme install`, the files of
//! other packages reached by `use package/file`, and the files that `rythme
//! run`, `test`, `check`, `fmt` and `doc` take at the root of a project.
//!
//! A project is a folder with a `rythme.toml` (see `manifest.rs`), its code
//! in `src/` and its tests in `tests/`. `use mathlib/vectors` is the file
//! `src/vectors.ry` of the package `mathlib`, one of the dependencies of the
//! project, or the project itself when it is its name. A dependency from
//! git is downloaded by `rythme install` into a folder of its own, given by
//! its address and its tag or commit, under `RYTHME_HOME` (`~/.rythme`);
//! `rythme.lock` keeps the exact commit of each one. A dependency from the
//! package index (`index.rs`) is a dependency from git at the commit the
//! index gives for the version chosen, whose files are checked against the
//! checksum of the index.

use std::cell::RefCell;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::diagnostic::Diagnostic;
use crate::driver::join_use;
use crate::index::{self, Index};
use crate::io::out_line;
use crate::manifest::{self, Dep, IndexLock, Locked, Manifest, Rev, Source, LOCK, MANIFEST};
use crate::semver::Version;
use crate::source::Span;

/// A path as shown in messages: relative to the current folder when it is inside it.
pub fn shown(path: &Path) -> String {
    if path.is_absolute() {
        if let Some(rel) = std::env::current_dir().ok().and_then(|cwd| path.strip_prefix(cwd).ok().map(Path::to_path_buf)) {
            if !rel.as_os_str().is_empty() {
                return rel.display().to_string();
            }
        }
    }
    let s = path.display().to_string();
    s.strip_prefix("./").map(str::to_string).unwrap_or(s)
}

/// The folder of the project that contains the folder `start` (the nearest
/// one with a `rythme.toml`), written from `start` (`../..`) so that paths
/// in messages stay short.
pub fn find_root(start: &Path) -> Option<PathBuf> {
    let dir = if start.as_os_str().is_empty() { Path::new(".") } else { start };
    let canon = dir.canonicalize().ok()?;
    let levels = canon.ancestors().position(|a| a.join(MANIFEST).is_file())?;
    let mut p = start.to_path_buf();
    for _ in 0..levels {
        if p.file_name().is_some() {
            p.pop();
        } else {
            p.push("..");
        }
    }
    Some(p)
}

/// `dir` followed by a path written with `/`.
fn join(dir: &Path, p: &str) -> PathBuf {
    join_use(dir, p)
}

// ----- where packages are kept -------------------------------------------------

/// The folder where packages from git are kept: `RYTHME_HOME/packages`,
/// `~/.rythme/packages` by default.
pub fn cache_root() -> Result<PathBuf, Box<Diagnostic>> {
    Ok(rythme_home()?.join("packages"))
}

/// The folder where Rythme keeps what it downloads: `RYTHME_HOME`, `~/.rythme` by default.
pub fn rythme_home() -> Result<PathBuf, Box<Diagnostic>> {
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    let home = var("RYTHME_HOME").or_else(|| var(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(|h| h.join(".rythme")));
    let Some(home) = home else {
        let d = Diagnostic::error("E025", t!("no folder to keep the packages in", "aucun dossier où garder les paquets"));
        return Err(d
            .help(t!(
                "set the variable RYTHME_HOME to a folder, like `RYTHME_HOME=~/.rythme`",
                "donnez un dossier à la variable RYTHME_HOME, comme `RYTHME_HOME=~/.rythme`"
            ))
            .boxed());
    };
    Ok(if home.is_absolute() { home } else { std::env::current_dir().unwrap_or_default().join(home) })
}

/// A short, stable fingerprint of a text (FNV-1a).
pub(crate) fn fingerprint(s: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{:012x}", h & 0xffff_ffff_ffff)
}

/// The address given to git: a folder of this computer written relative to
/// the project becomes a full path.
fn git_address(url: &str, base: &Path) -> String {
    let local = base.join(url);
    if !url.contains("://") && Path::new(url).is_relative() && local.is_dir() {
        if let Ok(full) = local.canonicalize() {
            return plain(&full.display().to_string());
        }
    }
    url.to_string()
}

/// The folder of a package from git, for its address and its version.
fn git_dir(name: &str, url: &str, rev: &Rev) -> Result<PathBuf, Box<Diagnostic>> {
    let rev = match rev {
        Rev::Tag(t) => format!("tag-{t}"),
        Rev::Commit(c) => format!("commit-{c}"),
    };
    let rev: String = rev.chars().map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') { c } else { '_' }).collect();
    Ok(cache_root()?.join(format!("{name}-{}", fingerprint(url))).join(rev))
}

thread_local! {
    /// The last project seen outside of the folder of Rythme (`RYTHME_HOME`):
    /// its `rythme.lock` chooses the versions of the packages from the index,
    /// also for the packages that its packages use.
    static TOP: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

/// The folder whose `rythme.lock` gives the versions of the packages of `m`
/// from the index: `m` itself, or, for a package kept by Rythme, the project
/// that uses it.
fn lock_root(m: &Manifest) -> PathBuf {
    let dir = m.dir.canonicalize().unwrap_or_else(|_| m.dir.clone());
    let kept = rythme_home().ok().and_then(|h| h.canonicalize().ok()).is_some_and(|h| dir.starts_with(h));
    TOP.with(|top| {
        let mut top = top.borrow_mut();
        if kept {
            top.clone().unwrap_or(dir)
        } else {
            *top = Some(dir.clone());
            dir
        }
    })
}

/// The entry of the `rythme.lock` of `root` for a dependency from the index.
fn locked_index(root: &Path, name: &str, range: &crate::semver::Range) -> Option<Locked> {
    let fits = |l: &Locked| l.from_index.as_ref().and_then(|i| Version::parse(&i.version)).is_some_and(|v| range.matches(&v));
    manifest::load_lock(root).ok()?.into_iter().find(|l| l.name == name && fits(l))
}

/// The folder of a dependency of the project `m`, whether it is installed or
/// not; nothing for a dependency from the index that `rythme.lock` does not have yet.
fn dep_dir(m: &Manifest, dep: &Dep) -> Result<Option<PathBuf>, Box<Diagnostic>> {
    match &dep.source {
        Source::Path(p) => Ok(Some(join(&m.dir, p))),
        Source::Git { url, rev } => git_dir(&dep.name, &git_address(url, &m.dir), rev).map(Some),
        Source::Index(range) => match locked_index(&lock_root(m), &dep.name, range) {
            Some(l) => git_dir(&dep.name, &l.url, &Rev::Commit(l.commit)).map(Some),
            None => Ok(None),
        },
    }
}

/// How a dependency is written, for messages: `git https://..., tag v1.0.0`.
fn origin(dep: &Dep) -> String {
    match &dep.source {
        Source::Path(p) => t!("the folder `{p}`", "le dossier `{p}`"),
        Source::Git { url, rev } => format!("git `{url}`, {rev}"),
        Source::Index(range) => t!("the package index, version {range}", "l'index des paquets, version {range}"),
    }
}

/// The folder of an installed dependency, or why it cannot be used.
fn installed_dir(m: &Manifest, dep: &Dep, at: Span) -> Result<PathBuf, Box<Diagnostic>> {
    let found = dep_dir(m, dep)?;
    if let Some(dir) = found.as_ref().filter(|d| d.is_dir()) {
        return Ok(dir.clone());
    }
    let dir = found.unwrap_or_default();
    let (name, root) = (&dep.name, shown(&m.dir));
    let place = if root.is_empty() || root == "." {
        t!("run `rythme install` here, in the folder of the project", "lancez `rythme install` ici, dans le dossier du projet")
    } else {
        t!("run `rythme install` in `{root}`, the folder of the project", "lancez `rythme install` dans `{root}`, le dossier du projet")
    };
    let d = match &dep.source {
        Source::Path(p) => Diagnostic::error(
            "E025",
            t!("the folder `{p}` of the package `{name}` does not exist", "le dossier `{p}` du paquet `{name}` n'existe pas"),
        )
        .note(t!("looked for: {}", "cherché : {}", dir.display()))
        .help(t!("fix its `path` in `rythme.toml`", "corrigez son `path` dans `rythme.toml`")),
        Source::Git { .. } | Source::Index(_) => {
            Diagnostic::error("E025", t!("the package `{name}` is not installed", "le paquet `{name}` n'est pas installé"))
                .note(t!("`rythme.toml` asks for it from {}", "`rythme.toml` le demande depuis {}", origin(dep)))
                .help(place)
        }
    };
    Err(d.at(at).boxed())
}

// ----- `use package/file` ------------------------------------------------------

/// The file named by `use written` (from a file in the folder `base`) when
/// its first part is a package: the project itself or one of its
/// dependencies. Nothing when it is not a package.
pub fn resolve_use(written: &str, base: &Path, at: Span) -> Option<Result<PathBuf, Box<Diagnostic>>> {
    let mut parts = written.split(['/', '\\']).filter(|p| !p.is_empty() && *p != ".");
    let first = parts.next()?;
    let rest: Vec<&str> = parts.collect();
    let package = if rest.is_empty() { first.strip_suffix(".ry").unwrap_or(first) } else { first };
    if !manifest::is_package_name(package) {
        return None;
    }
    let root = find_root(base)?;
    let m = match manifest::load(&root) {
        Ok(m) => m,
        Err(d) => return Some(Err(d)),
    };
    let dir = if package == m.name {
        root
    } else {
        match installed_dir(&m, m.dep(package)?, at) {
            Ok(dir) => dir,
            Err(d) => return Some(Err(d)),
        }
    };
    Some(package_file(package, &dir, &rest, at))
}

/// The file `rest` of the package in `dir` (its `entry` when `rest` is empty).
fn package_file(package: &str, dir: &Path, rest: &[&str], at: Span) -> Result<PathBuf, Box<Diagnostic>> {
    if rest.is_empty() {
        if !dir.join(MANIFEST).is_file() {
            let shown_dir = shown(dir);
            let d = Diagnostic::error(
                "E025",
                t!(
                    "`{shown_dir}` is not a Rythme package: it has no `rythme.toml`",
                    "`{shown_dir}` n'est pas un paquet Rythme : il n'a pas de `rythme.toml`"
                ),
            );
            return Err(d
                .at(at)
                .help(t!("use one of its files: `use {package}/file`", "utilisez un de ses fichiers : `use {package}/fichier`"))
                .boxed());
        }
        let pm = manifest::load(dir)?;
        let path = join(dir, &pm.entry);
        if path.is_file() {
            return Ok(path);
        }
        let entry = &pm.entry;
        let d = Diagnostic::error(
            "E017",
            t!("the package `{package}` has no file `{entry}`", "le paquet `{package}` n'a pas de fichier `{entry}`"),
        );
        return Err(d
            .at(at)
            .note(t!("looked for: {}", "cherché : {}", path.display()))
            .help(t!("use one of its files: `use {package}/file`", "utilisez un de ses fichiers : `use {package}/fichier`"))
            .boxed());
    }
    let src = dir.join("src");
    let mut path = join(&src, &rest.join("/"));
    if path.extension().is_none() {
        path.set_extension("ry");
    }
    if path.is_file() {
        return Ok(path);
    }
    let wanted = rest.join("/");
    let wanted = wanted.trim_end_matches(".ry");
    let mut d = Diagnostic::error(
        "E017",
        t!("the package `{package}` has no file `{wanted}`", "le paquet `{package}` n'a pas de fichier `{wanted}`"),
    )
    .at(at)
    .note(t!("looked for: {}", "cherché : {}", path.display()));
    let folder = path.parent().map(Path::to_path_buf).unwrap_or(src);
    let prefix = match wanted.rsplit_once('/') {
        Some((d, _)) => format!("{package}/{d}/"),
        None => format!("{package}/"),
    };
    let mut files: Vec<String> = std::fs::read_dir(&folder)
        .map(|rd| rd.flatten().filter_map(|e| e.file_name().to_string_lossy().strip_suffix(".ry").map(str::to_string)).collect())
        .unwrap_or_default();
    files.sort();
    let last = wanted.rsplit('/').next().unwrap_or(wanted);
    if let Some(best) = crate::suggest::closest(last, files.iter().map(String::as_str)) {
        d = d.help(t!("did you mean `use {prefix}{best}`?", "vouliez-vous dire `use {prefix}{best}` ?"));
    } else if !files.is_empty() {
        let list: Vec<String> = files.iter().take(8).map(|f| format!("`{prefix}{f}`")).collect();
        d = d.note(t!("its files there are: {}", "ses fichiers à cet endroit sont : {}", list.join(", ")));
    }
    Err(d.boxed())
}

/// The packages that `use name/...` can name from a file of the folder
/// `base`, with the folder of their files (`src/`): for completion.
pub fn packages(base: &Path) -> Vec<(String, PathBuf)> {
    let Some(root) = find_root(base) else { return Vec::new() };
    let Ok(m) = manifest::load(&root) else { return Vec::new() };
    let mut out = vec![(m.name.clone(), root.join("src"))];
    for dep in &m.deps {
        if let Ok(Some(dir)) = dep_dir(&m, dep) {
            out.push((dep.name.clone(), dir.join("src")));
        }
    }
    out
}

/// For a `use` that names nothing, in a project: a package with a close
/// name (`Ok`, a help), or the packages there are (`Err`, a note).
pub fn use_hint(written: &str, base: &Path) -> Option<Result<String, String>> {
    let packages = packages(base);
    if packages.is_empty() {
        return None;
    }
    if let Some((first, rest)) = written.split_once('/') {
        if let Some(best) = crate::suggest::closest(first, packages.iter().map(|(n, _)| n.as_str())) {
            let rest = rest.trim_end_matches(".ry");
            return Some(Ok(t!("did you mean `use {best}/{rest}`?", "vouliez-vous dire `use {best}/{rest}` ?")));
        }
    }
    let names: Vec<String> = packages.into_iter().map(|(n, _)| format!("`{n}/...`")).collect();
    Some(Err(t!(
        "in this project, `use` also reaches the files of the packages {}",
        "dans ce projet, `use` atteint aussi les fichiers des paquets {}",
        names.join(", ")
    )))
}

// ----- git -----------------------------------------------------------------------

pub(crate) enum GitError {
    /// The program `git` cannot be started.
    Missing,
    /// What git said.
    Failed(String),
}

/// A path git accepts: on Windows, a canonical path starts with `\\?\` (or
/// `//?/` once written with slashes), which git cannot use.
fn plain(s: &str) -> String {
    for (verbatim, keep) in [(r"\\?\UNC\", r"\\"), ("//?/UNC/", "//"), (r"\\?\", ""), ("//?/", "")] {
        if let Some(rest) = s.strip_prefix(verbatim) {
            return format!("{keep}{rest}");
        }
    }
    s.to_string()
}

fn git(args: &[&str], dir: &Path) -> Result<String, GitError> {
    run_git(args, dir, false).map(|out| String::from_utf8_lossy(&out).trim().to_string())
}

/// git for the package index and its packages, with the safe options: only
/// the protocols https, ssh and file (a folder of this computer), no hooks,
/// no file system monitor, links checked out as plain files.
pub(crate) fn git_safe(args: &[&str], dir: &Path) -> Result<String, GitError> {
    run_git(args, dir, true).map(|out| String::from_utf8_lossy(&out).trim().to_string())
}

/// The command `git` with these arguments in `dir` (environment variables
/// that would point it at another repository are left out).
pub(crate) fn git_command(args: &[&str], dir: &Path, safe: bool) -> Command {
    let mut all: Vec<String> = Vec::new();
    if safe {
        let no_hooks = if cfg!(windows) { "NUL" } else { "/dev/null" };
        for option in [format!("core.hooksPath={no_hooks}"), "core.fsmonitor=false".into(), "core.symlinks=false".into()] {
            all.push("-c".into());
            all.push(option);
        }
    }
    all.extend(args.iter().map(|a| plain(a)));
    let dir = PathBuf::from(plain(&dir.to_string_lossy()));
    let mut cmd = Command::new("git");
    cmd.args(&all).current_dir(if dir.as_os_str().is_empty() { Path::new(".") } else { &dir }).env("GIT_TERMINAL_PROMPT", "0");
    for var in ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_NAMESPACE"] {
        cmd.env_remove(var);
    }
    if safe {
        cmd.env("GIT_ALLOW_PROTOCOL", "https:ssh:file");
    }
    cmd
}

fn run_git(args: &[&str], dir: &Path, safe: bool) -> Result<Vec<u8>, GitError> {
    let out = git_command(args, dir, safe).stdin(Stdio::null()).output().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            GitError::Missing
        } else {
            GitError::Failed(e.to_string())
        }
    })?;
    if out.status.success() {
        Ok(out.stdout)
    } else {
        Err(GitError::Failed(String::from_utf8_lossy(&out.stderr).trim().to_string()))
    }
}

/// What a safe git command prints, as bytes.
pub(crate) fn git_bytes(args: &[&str], dir: &Path) -> Result<Vec<u8>, GitError> {
    run_git(args, dir, true)
}

pub(crate) fn git_missing() -> Box<Diagnostic> {
    Diagnostic::error("E025", t!("the program `git` was not found", "le programme `git` est introuvable"))
        .note(t!(
            "Rythme uses git to download the dependencies that come from git",
            "Rythme utilise git pour télécharger les dépendances qui viennent de git"
        ))
        .help(t!(
            "install Git (https://git-scm.com), then run `rythme install` again; dependencies with `path` do not need it",
            "installez Git (https://git-scm.com), puis relancez `rythme install` ; les dépendances avec `path` n'en ont pas besoin"
        ))
        .boxed()
}

pub(crate) fn git_error(e: GitError, msg: String) -> Box<Diagnostic> {
    match e {
        GitError::Missing => git_missing(),
        GitError::Failed(said) => {
            let d = Diagnostic::error("E025", msg);
            let d = if said.is_empty() { d } else { d.note(t!("git says: {said}", "git dit : {said}")) };
            d.boxed()
        }
    }
}

/// The full commit of `rev` in the repository `dir`.
fn resolve(dir: &Path, name: &str, url: &str, rev: &Rev) -> Result<String, Box<Diagnostic>> {
    let wanted = match rev {
        Rev::Tag(t) => format!("refs/tags/{t}^{{commit}}"),
        Rev::Commit(c) => format!("{c}^{{commit}}"),
    };
    match git(&["rev-parse", "--verify", "--quiet", &wanted], dir) {
        Ok(commit) if !commit.is_empty() => Ok(commit),
        Err(GitError::Missing) => Err(git_missing()),
        _ => {
            let mut d = match rev {
                Rev::Tag(t) => Diagnostic::error(
                    "E025",
                    t!("the tag `{t}` of `{name}` does not exist in `{url}`", "le tag `{t}` de `{name}` n'existe pas dans `{url}`"),
                ),
                Rev::Commit(c) => Diagnostic::error(
                    "E025",
                    t!("the commit `{c}` of `{name}` does not exist in `{url}`", "le commit `{c}` de `{name}` n'existe pas dans `{url}`"),
                ),
            };
            if let Rev::Tag(t) = rev {
                let tags = git(&["tag", "--list"], dir).unwrap_or_default();
                let tags: Vec<&str> = tags.lines().collect();
                if let Some(best) = crate::suggest::closest(t, tags.iter().copied()) {
                    d = d.help(t!("did you mean `tag = \"{best}\"`?", "vouliez-vous dire `tag = \"{best}\"` ?"));
                } else if !tags.is_empty() {
                    let list: Vec<String> = tags.iter().rev().take(8).map(|t| format!("`{t}`")).collect();
                    d = d.note(t!("its tags are: {}", "ses tags sont : {}", list.join(", ")));
                } else {
                    d = d.note(t!("it has no tags: give a `commit` instead", "il n'a aucun tag : donnez plutôt un `commit`"));
                }
            }
            Err(d.boxed())
        }
    }
}

/// Removes a folder and what it holds, even the files marked read-only (git
/// marks some of its files so on Windows).
pub(crate) fn remove_all(dir: &Path) -> std::io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    if std::fs::remove_dir_all(dir).is_ok() {
        return Ok(());
    }
    fn writable(p: &Path) {
        if let Ok(meta) = std::fs::symlink_metadata(p) {
            let mut perms = meta.permissions();
            #[allow(clippy::permissions_set_readonly_false)]
            perms.set_readonly(false);
            let _ = std::fs::set_permissions(p, perms);
            if meta.is_dir() {
                for e in std::fs::read_dir(p).into_iter().flatten().flatten() {
                    writable(&e.path());
                }
            }
        }
    }
    writable(dir);
    std::fs::remove_dir_all(dir)
}

/// Downloads a dependency from git into its folder, at the commit of the
/// lock file when there is one. Gives the commit, and whether it was
/// downloaded. `fresh`: download it again even when it is there (`rythme
/// update`, for a tag that moved).
fn fetch(name: &str, url: &str, rev: &Rev, base: &Path, locked: Option<&str>, fresh: bool) -> Result<(String, bool), Box<Diagnostic>> {
    let address = git_address(url, base);
    let dir = git_dir(name, &address, rev)?;
    if dir.join(".git").exists() && !fresh {
        if let Ok(head) = git(&["rev-parse", "HEAD"], &dir) {
            let want = match locked {
                Some(c) => Some(c.to_string()),
                None => resolve(&dir, name, url, rev).ok(),
            };
            if want.as_deref() == Some(head.as_str()) {
                return Ok((head, false));
            }
        }
    }
    let parent = dir.parent().map(Path::to_path_buf).unwrap_or_default();
    let cannot = |e: std::io::Error, p: &Path| {
        Diagnostic::error("E025", t!("cannot write in `{}`: {e}", "impossible d'écrire dans `{}` : {e}", p.display())).boxed()
    };
    std::fs::create_dir_all(&parent).map_err(|e| cannot(e, &parent))?;
    let leaf = dir.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    let tmp = parent.join(format!(".download-{leaf}-{}", std::process::id()));
    remove_all(&tmp).map_err(|e| cannot(e, &tmp))?;
    let tmp_text = tmp.display().to_string();
    git(&["clone", "--quiet", "--no-checkout", "--", &address, &tmp_text], base)
        .map_err(|e| git_error(e, t!("cannot download `{name}` from `{url}`", "impossible de télécharger `{name}` depuis `{url}`")))?;
    let result = (|| -> Result<String, Box<Diagnostic>> {
        let commit = match locked.and_then(|c| resolve(&tmp, name, url, &Rev::Commit(c.to_string())).ok()) {
            Some(c) => c,
            None => resolve(&tmp, name, url, rev)?,
        };
        git(&["checkout", "--quiet", "--detach", &commit], &tmp).map_err(|e| {
            git_error(e, t!("cannot take the commit `{commit}` of `{name}`", "impossible de prendre le commit `{commit}` de `{name}`"))
        })?;
        remove_all(&dir).map_err(|e| cannot(e, &dir))?;
        std::fs::rename(&tmp, &dir).map_err(|e| cannot(e, &dir))?;
        Ok(commit)
    })();
    if result.is_err() {
        let _ = remove_all(&tmp);
    }
    Ok((result?, true))
}

/// What the index (or `rythme.lock`) says of the version of a package to install.
pub(crate) struct Pinned {
    pub version: String,
    pub url: String,
    pub tag: Option<String>,
    pub commit: String,
    pub checksum: String,
}

/// Downloads a package of the index into its folder: the commit the index
/// gives, whose files must have the checksum of the index; nothing of the
/// package runs. Gives whether it was downloaded.
fn fetch_index(name: &str, p: &Pinned) -> Result<bool, Box<Diagnostic>> {
    let rev = Rev::Commit(p.commit.clone());
    let dir = git_dir(name, &p.url, &rev)?;
    if dir.join(".git").exists() && git_safe(&["rev-parse", "HEAD"], &dir).is_ok_and(|head| head == p.commit) {
        return Ok(false);
    }
    let parent = dir.parent().map(Path::to_path_buf).unwrap_or_default();
    let cannot = |e: std::io::Error, p: &Path| {
        Diagnostic::error("E025", t!("cannot write in `{}`: {e}", "impossible d'écrire dans `{}` : {e}", p.display())).boxed()
    };
    std::fs::create_dir_all(&parent).map_err(|e| cannot(e, &parent))?;
    let leaf = dir.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    let tmp = parent.join(format!(".download-{leaf}-{}", std::process::id()));
    remove_all(&tmp).map_err(|e| cannot(e, &tmp))?;
    let tmp_text = tmp.display().to_string();
    let url = &p.url;
    git_safe(&["clone", "--quiet", "--no-checkout", "--", url, &tmp_text], &parent)
        .map_err(|e| git_error(e, t!("cannot download `{name}` from `{url}`", "impossible de télécharger `{name}` depuis `{url}`")))?;
    let result = (|| -> Result<(), Box<Diagnostic>> {
        let (commit, version) = (&p.commit, &p.version);
        let wanted = format!("{commit}^{{commit}}");
        if !git_safe(&["rev-parse", "--verify", "--quiet", &wanted], &tmp).is_ok_and(|c| c == *commit) {
            let d = Diagnostic::error(
                "E025",
                t!(
                    "the commit `{commit}` of `{name}` {version} does not exist in `{url}`",
                    "le commit `{commit}` de `{name}` {version} n'existe pas dans `{url}`"
                ),
            );
            return Err(d
                .note(t!("the package index gives this commit for this version", "l'index des paquets donne ce commit pour cette version"))
                .help(t!(
                    "tell the authors of `{name}`: the history of their repository may have been rewritten",
                    "prévenez les auteurs de `{name}` : l'historique de leur dépôt a peut-être été réécrit"
                ))
                .boxed());
        }
        let found = index::checksum(&tmp, commit).map_err(|e| {
            git_error(e, t!("cannot read the files of `{name}` {version}", "impossible de lire les fichiers de `{name}` {version}"))
        })?;
        if found != p.checksum {
            let d = Diagnostic::error(
                "E025",
                t!(
                    "the files of `{name}` {version} do not match the checksum of the package index",
                    "les fichiers de `{name}` {version} ne correspondent pas à la somme de contrôle de l'index des paquets"
                ),
            );
            return Err(d
                .note(t!("expected: {}\nfound:    {found}", "attendue : {}\ntrouvée :  {found}", p.checksum))
                .note(t!("nothing was installed, and nothing of the package ran", "rien n'a été installé, et rien du paquet n'a été lancé"))
                .help(t!(
                    "do not use this version; tell the maintainers of the package index",
                    "n'utilisez pas cette version ; prévenez les responsables de l'index des paquets"
                ))
                .boxed());
        }
        git_safe(&["checkout", "--quiet", "--detach", commit], &tmp).map_err(|e| {
            git_error(e, t!("cannot take the commit `{commit}` of `{name}`", "impossible de prendre le commit `{commit}` de `{name}`"))
        })?;
        remove_all(&dir).map_err(|e| cannot(e, &dir))?;
        std::fs::rename(&tmp, &dir).map_err(|e| cannot(e, &dir))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = remove_all(&tmp);
    }
    result.map(|()| true)
}

// ----- `rythme install` ------------------------------------------------------------

fn usage(msg: String) -> i32 {
    crate::io::err_line(&msg);
    2
}

fn report(d: &Diagnostic) {
    crate::diagnostic::show(d, crate::diagnostic::use_color());
}

/// The project of the current folder, or a message saying there is none.
pub(crate) fn current_project() -> Result<PathBuf, Box<Diagnostic>> {
    find_root(Path::new("")).ok_or_else(|| {
        Diagnostic::error("E025", t!("no `rythme.toml` in this folder or above it", "aucun `rythme.toml` dans ce dossier ni au-dessus"))
            .help(t!(
                "go to the folder of a project, or create one with `rythme new NAME`",
                "allez dans le dossier d'un projet, ou créez-en un avec `rythme new NOM`"
            ))
            .boxed()
    })
}

/// `rythme install`: downloads the dependencies of the project (and theirs)
/// and writes `rythme.lock`.
pub fn install_command(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        return usage(t!(
            "`rythme install` takes no argument (`{a}`): it installs the dependencies of `rythme.toml`\nsee `rythme help projects`",
            "`rythme install` ne prend pas d'argument (`{a}`) : il installe les dépendances de `rythme.toml`\nvoir `rythme help projects`"
        ));
    }
    match install(None) {
        Ok(_) => 0,
        Err(d) => {
            report(&d);
            1
        }
    }
}

/// `rythme.lock` before and after `rythme install` or `rythme update`.
pub(crate) struct Installed {
    pub before: Vec<Locked>,
    pub after: Vec<Locked>,
}

/// The package index of an installation, found only when a dependency
/// needs it.
struct LazyIndex {
    /// The `index` of the `rythme.toml` of the project, and its folder.
    setting: (Option<String>, PathBuf),
    index: Option<Index>,
}

impl LazyIndex {
    fn get(&mut self) -> Result<&mut Index, Box<Diagnostic>> {
        if self.index.is_none() {
            let (setting, dir) = &self.setting;
            self.index = Some(Index::new(setting.as_deref().map(|s| (s, dir.as_path())))?);
        }
        Ok(self.index.as_mut().expect("just made"))
    }
}

/// A message about a dependency, at its line of `rythme.toml` when it has no place yet.
fn at_dep(d: Box<Diagnostic>, span: Span) -> Box<Diagnostic> {
    if d.span.is_some() {
        d
    } else {
        Box::new(d.at(span))
    }
}

/// The version of a dependency from the index to install: the one of
/// `rythme.lock` while it is in the range (unless `update`), else the
/// newest one of the index in the range.
fn pin(name: &str, range: &crate::semver::Range, lock: &[Locked], index: &mut LazyIndex, update: bool) -> Result<Pinned, Box<Diagnostic>> {
    let address = index.get()?.url.clone();
    let kept = lock.iter().filter(|_| !update).find_map(|l| {
        let i = l.from_index.as_ref().filter(|i| l.name == name && i.index == address)?;
        Version::parse(&i.version).filter(|v| range.matches(v))?;
        let tag = match &l.rev {
            Rev::Tag(t) => Some(t.clone()),
            Rev::Commit(_) => None,
        };
        Some(Pinned { version: i.version.clone(), url: l.url.clone(), tag, commit: l.commit.clone(), checksum: i.checksum.clone() })
    });
    if let Some(p) = kept {
        return Ok(p);
    }
    let index = index.get()?;
    if update {
        index.refresh()?;
    }
    let entry = index.find(name)?;
    let Some(release) = index::choose(&entry, range) else { return Err(index::no_version(&entry, range)) };
    Ok(Pinned {
        version: release.version.to_string(),
        url: entry.repository.clone(),
        tag: release.tag.clone(),
        commit: release.commit.clone(),
        checksum: release.checksum.clone(),
    })
}

/// Installs the dependencies of the project; `update`: the packages to take
/// again without `rythme.lock` (every one when the list is empty).
pub(crate) fn install(update: Option<&[String]>) -> Result<Installed, Box<Diagnostic>> {
    let root = current_project()?;
    let m = manifest::load(&root)?;
    let lock = manifest::load_lock(&root)?;
    let name = &m.name;
    if m.deps.is_empty() {
        out_line(&t!(
            "`{name}` has no dependencies: add them under `[dependencies]` in `rythme.toml`",
            "`{name}` n'a pas de dépendances : ajoutez-les sous `[dependencies]` dans `rythme.toml`"
        ));
    } else if update.is_some() {
        out_line(&t!("Updating the dependencies of `{name}`", "Mise à jour des dépendances de `{name}`"));
    } else {
        out_line(&t!("Installing the dependencies of `{name}`", "Installation des dépendances de `{name}`"));
    }
    let updating = |n: &str| update.is_some_and(|names| names.is_empty() || names.iter().any(|u| u == n));
    let mut index = LazyIndex { setting: (m.index.clone(), m.dir.clone()), index: None };
    let mut queue: Vec<(Manifest, usize)> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut locked: Vec<Locked> = Vec::new();
    let mut count = 0;
    let deps = m.deps.len();
    queue.push((m, 0));
    while let Some((owner, next)) = queue.pop() {
        let Some(dep) = owner.deps.get(next) else { continue };
        let (dep_name, dep_span) = (dep.name.clone(), dep.span);
        let mut from_index = None;
        let (dir, line) = match &dep.source {
            Source::Index(range) => {
                let pinned = pin(&dep_name, range, &lock, &mut index, updating(&dep_name)).map_err(|d| at_dep(d, dep.span))?;
                let downloaded = fetch_index(&dep_name, &pinned).map_err(|d| at_dep(d, dep.span))?;
                let short: String = pinned.commit.chars().take(7).collect();
                let how = if downloaded { t!("downloaded", "téléchargé") } else { t!("already there", "déjà là") };
                let rev = pinned.tag.clone().map_or_else(|| Rev::Commit(pinned.commit.clone()), Rev::Tag);
                let address = index.get()?.url.clone();
                let entry = Locked {
                    name: dep_name.clone(),
                    url: pinned.url.clone(),
                    rev,
                    commit: pinned.commit.clone(),
                    from_index: Some(IndexLock { version: pinned.version.clone(), index: address, checksum: pinned.checksum.clone() }),
                };
                if !locked.contains(&entry) {
                    locked.push(entry);
                }
                let dir = git_dir(&dep_name, &pinned.url, &Rev::Commit(pinned.commit.clone()))?;
                from_index = Some(pinned.version);
                (dir, t!("{range} from the index, commit {short} ({how})", "{range} depuis l'index, commit {short} ({how})"))
            }
            Source::Path(p) => {
                let dir = join(&owner.dir, p);
                if !dir.is_dir() {
                    let d = Diagnostic::error(
                        "E025",
                        t!(
                            "the folder `{p}` of the package `{dep_name}` does not exist",
                            "le dossier `{p}` du paquet `{dep_name}` n'existe pas"
                        ),
                    );
                    return Err(d.at(dep.span).note(t!("looked for: {}", "cherché : {}", dir.display())).boxed());
                }
                (dir, t!("folder {p}", "dossier {p}"))
            }
            Source::Git { url, rev } => {
                let again = updating(&dep_name);
                let wanted = |l: &&Locked| l.url == *url && l.rev == *rev && l.from_index.is_none() && !again;
                let before = lock.iter().find(wanted).map(|l| l.commit.as_str());
                let fresh = again && matches!(rev, Rev::Tag(_));
                let (commit, downloaded) = fetch(&dep_name, url, rev, &owner.dir, before, fresh).map_err(|d| Box::new(d.at(dep.span)))?;
                let short: String = commit.chars().take(7).collect();
                let how = if downloaded { t!("downloaded", "téléchargé") } else { t!("already there", "déjà là") };
                let entry = Locked { name: dep_name.clone(), url: url.clone(), rev: rev.clone(), commit: commit.clone(), from_index: None };
                if !locked.contains(&entry) {
                    locked.push(entry);
                }
                (git_dir(&dep_name, &git_address(url, &owner.dir), rev)?, format!("{rev}, commit {short} ({how})"))
            }
        };
        // The dependencies of the dependency, after the next ones of the same project.
        queue.push((owner, next + 1));
        let canon = dir.canonicalize().unwrap_or_else(|_| dir.clone());
        if !seen.insert(canon) {
            continue;
        }
        if !dir.join(MANIFEST).is_file() {
            let shown_dir = shown(&dir);
            let d = Diagnostic::error(
                "E025",
                t!(
                    "`{dep_name}` is not a Rythme package: `{shown_dir}` has no `rythme.toml`",
                    "`{dep_name}` n'est pas un paquet Rythme : `{shown_dir}` n'a pas de `rythme.toml`"
                ),
            );
            return Err(d
                .help(t!(
                    "a package is a project made by `rythme new`: its `rythme.toml`, and its files in `src/`",
                    "un paquet est un projet fait par `rythme new` : son `rythme.toml`, et ses fichiers dans `src/`"
                ))
                .boxed());
        }
        let sub = manifest::load(&dir)?;
        if let Some(wanted) = from_index.filter(|v| *v != sub.version || sub.name != dep_name) {
            let (found, version) = (&sub.name, &sub.version);
            let d = Diagnostic::error(
                "E025",
                t!(
                    "the package index gives `{dep_name}` {wanted}, but its `rythme.toml` says `{found}` {version}",
                    "l'index des paquets donne `{dep_name}` {wanted}, mais son `rythme.toml` dit `{found}` {version}"
                ),
            );
            return Err(d
                .at(dep_span)
                .help(t!("tell the maintainers of the package index", "prévenez les responsables de l'index des paquets"))
                .boxed());
        }
        count += 1;
        let version = &sub.version;
        out_line(&t!("  ✓ {dep_name} {version}: {line}", "  ✓ {dep_name} {version} : {line}"));
        if !sub.deps.is_empty() {
            queue.push((sub, 0));
        }
    }
    let lock_path = root.join(LOCK);
    if !locked.is_empty() || lock_path.is_file() {
        locked.sort_by(|a, b| (&a.name, &a.url, &a.commit).cmp(&(&b.name, &b.url, &b.commit)));
        let text = manifest::lock_text(&locked);
        // Written again only when something changed (not for the language of its comments).
        if lock != locked || !lock_path.is_file() {
            std::fs::write(&lock_path, text).map_err(|e| {
                Diagnostic::error("E025", t!("cannot write `{}`: {e}", "impossible d'écrire `{}` : {e}", shown(&lock_path))).boxed()
            })?;
        }
    }
    if deps > 0 {
        let more = count - deps.min(count);
        if more == 0 {
            out_line(&t!("{count} package(s) ready", "{count} paquet(s) prêt(s)"));
        } else {
            out_line(&t!(
                "{count} package(s) ready ({more} needed by the other ones)",
                "{count} paquet(s) prêt(s) ({more} demandé(s) par les autres)"
            ));
        }
    }
    Ok(Installed { before: lock, after: locked })
}

// ----- `rythme update` -------------------------------------------------------------

/// `rythme update [NAME...]`: takes again the dependencies (all of them, or
/// those named) without `rythme.lock`: for a package of the index, the newest
/// version of its range, from the newest copy of the index; for a tag of git,
/// its commit now. `rythme.toml` does not change.
pub fn update_command(args: &[String]) -> i32 {
    if let Some(bad) = args.iter().find(|a| a.starts_with('-')) {
        return usage(t!(
            "`rythme update` takes the names of dependencies, not `{bad}`: `rythme update` (all of them) or `rythme update mathlib`",
            "`rythme update` prend des noms de dépendances, pas `{bad}` : `rythme update` (toutes) ou `rythme update mathlib`"
        ));
    }
    let result = (|| -> Result<Installed, Box<Diagnostic>> {
        let root = current_project()?;
        let m = manifest::load(&root)?;
        let known: Vec<String> =
            m.deps.iter().map(|d| d.name.clone()).chain(manifest::load_lock(&root)?.into_iter().map(|l| l.name)).collect();
        for a in args {
            if !known.contains(a) {
                let d =
                    Diagnostic::error("E025", t!("`{a}` is not a dependency of `{}`", "`{a}` n'est pas une dépendance de `{}`", m.name));
                let d = match crate::suggest::closest(a, known.iter().map(String::as_str)) {
                    Some(best) => d.help(t!("did you mean `rythme update {best}`?", "vouliez-vous dire `rythme update {best}` ?")),
                    None => d.help(t!(
                        "`rythme add {a}` adds it from the package index",
                        "`rythme add {a}` l'ajoute depuis l'index des paquets"
                    )),
                };
                return Err(d.boxed());
            }
        }
        install(Some(args))
    })();
    match result {
        Ok(done) => {
            let changes = changes(&done.before, &done.after);
            if changes.is_empty() {
                out_line(&t!("Everything was already up to date", "Tout était déjà à jour"));
            } else {
                out_line(&t!("Updated:", "Mis à jour :"));
                for c in changes {
                    out_line(&format!("  {c}"));
                }
            }
            0
        }
        Err(d) => {
            report(&d);
            1
        }
    }
}

/// What changed in `rythme.lock`: `mathlib 1.0.0 → 1.2.0`.
fn changes(before: &[Locked], after: &[Locked]) -> Vec<String> {
    let what = |l: &Locked| match &l.from_index {
        Some(i) => i.version.clone(),
        None => format!("{} ({})", l.rev, l.commit.chars().take(7).collect::<String>()),
    };
    let mut out = Vec::new();
    for a in after {
        let same_kind = |b: &&Locked| b.name == a.name && b.from_index.is_some() == a.from_index.is_some();
        match before.iter().find(same_kind) {
            Some(b) if b.commit == a.commit => {}
            Some(b) => out.push(format!("{} {} → {}", a.name, what(b), what(a))),
            None => out.push(t!("{} {} (new)", "{} {} (nouveau)", a.name, what(a))),
        }
    }
    out
}

// ----- `rythme new` ----------------------------------------------------------------

/// `rythme new NAME`: a folder with a new project.
pub fn new_command(args: &[String]) -> i32 {
    let [target] = args else {
        return usage(match args.first() {
            None => t!("which name? Example: rythme new hello", "quel nom ? Exemple : rythme new bonjour"),
            Some(_) => {
                t!("`rythme new` takes one name, like `rythme new hello`", "`rythme new` prend un seul nom, comme `rythme new bonjour`")
            }
        });
    };
    let dir = PathBuf::from(target);
    let name = dir.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    if !manifest::is_package_name(&name) {
        let (msg, better) = manifest::bad_name(&name);
        let mut d = Diagnostic::error("E025", msg);
        if let Some(b) = better {
            d = d.help(t!("try `rythme new {b}`", "essayez `rythme new {b}`"));
        }
        report(&d);
        return 2;
    }
    let busy = dir.is_file() || std::fs::read_dir(&dir).is_ok_and(|mut rd| rd.next().is_some());
    if busy {
        let shown_dir = shown(&dir);
        let d = Diagnostic::error("E025", t!("`{shown_dir}` already exists", "`{shown_dir}` existe déjà"))
            .help(t!("choose another name, or another folder", "choisissez un autre nom, ou un autre dossier"));
        report(&d);
        return 1;
    }
    let files = new_files(&name);
    for (path, text) in &files {
        let path = join(&dir, path);
        let written = path.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|_| std::fs::write(&path, text));
        if let Err(e) = written {
            report(&Diagnostic::error("E025", t!("cannot write `{}`: {e}", "impossible d'écrire `{}` : {e}", shown(&path))));
            return 1;
        }
    }
    let shown_dir = shown(&dir);
    out_line(&t!("Created the project `{name}`:", "Projet `{name}` créé :"));
    for (path, _) in &files {
        out_line(&format!("  {}", shown(&join(&dir, path))));
    }
    out_line(&t!(
        "Next: `cd {shown_dir}`, then `rythme run` to run it and `rythme test` to test it (see `rythme help projects`)",
        "Ensuite : `cd {shown_dir}`, puis `rythme run` pour le lancer et `rythme test` pour le tester (voir `rythme help projects`)"
    ));
    0
}

/// The files of a new project, in the language of the messages.
pub fn new_files(name: &str) -> Vec<(&'static str, String)> {
    use crate::lang::pick;
    let (module, func) = (pick("greeting", "salutation"), pick("hello", "bonjour"));
    let toml = t!(
        r#"# The project: `rythme run`, `rythme test` and `rythme check` in this folder.
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# From git, at a tag or a commit, then `rythme install`:
# mathlib = {{ git = "https://github.com/someone/mathlib", tag = "v1.0.0" }}
# From a folder:
# tools = {{ path = "../tools" }}
"#,
        r#"# Le projet : `rythme run`, `rythme test` et `rythme check` dans ce dossier.
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# Depuis git, à un tag ou à un commit, puis `rythme install` :
# mathlib = {{ git = "https://github.com/quelqu-un/mathlib", tag = "v1.0.0" }}
# Depuis un dossier :
# outils = {{ path = "../outils" }}
"#
    );
    let main = t!(
        "# The program: `rythme run` in the folder of the project.\nuse {module}\n\nprint({module}.{func}(\"World\"))\n",
        "# Le programme : `rythme run` dans le dossier du projet.\nuse {module}\n\nprint({module}.{func}(\"le monde\"))\n"
    );
    let lib = t!(
        "## A greeting for `name`.\nfn {func}(name) = \"Hello, {{name}}!\"\n",
        "## Une salutation pour `nom`.\nfn {func}(nom) = \"Bonjour, {{nom}} !\"\n"
    );
    let test = t!(
        "# The tests: `rythme test` in the folder of the project.\nuse {name}/{module}\n\ntest \"{func}\" = assert_eq({module}.{func}(\"Ada\"), \"Hello, Ada!\")\n",
        "# Les tests : `rythme test` dans le dossier du projet.\nuse {name}/{module}\n\ntest \"{func}\" = assert_eq({module}.{func}(\"Ada\"), \"Bonjour, Ada !\")\n"
    );
    let readme = t!(
        "# {name}\n\nA [Rythme](https://github.com/maxlestage/Rythme) project.\n\n```sh\nrythme run        # run the program (src/main.ry)\nrythme test       # run the tests (tests/)\nrythme check      # find problems without running\nrythme fmt        # give the code its official layout\nrythme install    # download the dependencies of rythme.toml\n```\n",
        "# {name}\n\nUn projet [Rythme](https://github.com/maxlestage/Rythme).\n\n```sh\nrythme run        # lancer le programme (src/main.ry)\nrythme test       # lancer les tests (tests/)\nrythme check      # trouver les problèmes sans lancer\nrythme fmt        # mettre le code en forme\nrythme install    # télécharger les dépendances de rythme.toml\n```\n"
    );
    let ignore = t!(
        "# Files that git does not keep (keep rythme.lock).\n.DS_Store\nThumbs.db\n*.swp\n*~\n",
        "# Les fichiers que git ne garde pas (gardez rythme.lock).\n.DS_Store\nThumbs.db\n*.swp\n*~\n"
    );
    let lib_path = pick("src/greeting.ry", "src/salutation.ry");
    let test_path = pick("tests/greeting_test.ry", "tests/salutation_test.ry");
    vec![
        (MANIFEST, toml),
        ("src/main.ry", main),
        (lib_path, lib),
        (test_path, test),
        ("README.md", readme),
        (".gitignore", ignore),
        ("AGENTS.md", AGENTS.replace("{name}", name)),
        ("CLAUDE.md", "See [AGENTS.md](AGENTS.md).\n\n@AGENTS.md\n".to_string()),
    ]
}

/// The `AGENTS.md` of a new project: how an AI coding agent works in a
/// Rythme project. In English, the language these tools read best, whatever
/// the language of the messages.
pub const AGENTS: &str = r#"# AGENTS.md

This is **{name}**, a project in [Rythme](https://github.com/maxlestage/Rythme),
a small, purely functional language. AI assistants do not know Rythme well:
before writing code, read the whole language in one text with

```sh
rythme help llms
```

(also at https://github.com/maxlestage/Rythme/blob/master/docs/llms-full.txt).
Rythme is not Python, JavaScript or Ruby: no loops (use `map`, `filter`,
`fold`, `each` or recursion), values never change (no `+=`, no `x = ...`
twice), no `return` (the last expression is the result), no null (`Some`,
`None`), no exceptions (`Ok`, `Err`, `?`), and `if` always has an `else`.
Never guess a function name: `rythme help NAME` or `rythme help builtins`.

## The loop

After each change, until there is nothing left:

```sh
rythme check --format json        # every error and warning, as JSON, without running
rythme fix                        # apply the safe fixes it found (--dry-run to see them first)
rythme test --format json         # run the tests of tests/
rythme run --format json          # run src/main.ry; its output and errors as JSON
```

Each diagnostic has a stable `code` (`rythme explain E003` explains it), a
`range`, a `help` that names the fix in code, and often `fixes` with exact
text edits. Read the help before changing the code. `rythme fmt` gives the
code its official layout.

## The project

- `rythme.toml`: the name, the version, the entry point and the dependencies
  (`rythme install` downloads them).
- `src/main.ry`: the program; the other files of `src/` are modules: for
  `src/shapes.ry`, write `use shapes`, then `shapes.area(2)`. Names starting
  with `_` stay private to their file.
- `tests/`: tests, written `test "name" = assert_eq(actual, expected)`; add
  one for each new function.

`rythme mcp` gives these commands to AI assistants through the Model Context
Protocol: `claude mcp add rythme -- rythme mcp` for Claude Code.
"#;

// ----- commands without a file, at the root of a project -----------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cmd {
    Run,
    Test,
    Check,
    Fmt,
    Doc,
}

/// The Rythme files of a folder and of its folders, in order.
fn ry_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let hidden = p.file_name().is_some_and(|f| f.to_string_lossy().starts_with('.'));
        if hidden {
            continue;
        }
        if p.is_dir() {
            ry_files(&p, out);
        } else if p.extension().is_some_and(|e| e == "ry") {
            out.push(p);
        }
    }
}

/// Does this file have tests?
fn has_tests(path: &Path) -> bool {
    let Ok(src) = std::fs::read_to_string(path) else { return false };
    match crate::lexer::lex(&src, u32::MAX).and_then(crate::parser::parse) {
        Ok(items) => items.iter().any(|i| matches!(i, crate::ast::Item::Test(..))),
        // A file that does not parse: running it shows the error.
        Err(_) => src.contains("test "),
    }
}

/// What a command without a file works on in the project of the current
/// folder: the program for `run`, the files with tests for `test`, every
/// file for `check`, the folders `src` and `tests` for `fmt`, `src` for
/// `doc`. Nothing outside of a project; `Err` after a reported error.
pub fn targets(cmd: Cmd) -> Option<Result<Vec<String>, ()>> {
    let root = find_root(Path::new(""))?;
    let m = match manifest::load(&root) {
        Ok(m) => m,
        Err(d) => {
            report(&d);
            return Some(Err(()));
        }
    };
    let entry = join(&root, &m.entry);
    let text = |p: &Path| shown(p);
    let folders: Vec<PathBuf> = ["src", "tests"].iter().map(|f| root.join(f)).filter(|p| p.is_dir()).collect();
    let mut files = Vec::new();
    for f in &folders {
        ry_files(f, &mut files);
    }
    let list = match cmd {
        Cmd::Run => vec![text(&entry)],
        Cmd::Check => {
            let first = text(&entry);
            let mut all: Vec<String> = files.iter().map(|p| text(p)).filter(|p| *p != first).collect();
            all.insert(0, first);
            all
        }
        Cmd::Test => {
            let with: Vec<String> = files.iter().filter(|p| has_tests(p)).map(|p| text(p)).collect();
            if with.is_empty() {
                vec![text(&entry)]
            } else {
                with
            }
        }
        Cmd::Fmt => {
            if folders.is_empty() {
                vec![text(&entry)]
            } else {
                folders.iter().map(|p| text(p)).collect()
            }
        }
        Cmd::Doc => {
            let src = root.join("src");
            vec![text(if src.is_dir() { &src } else { &entry })]
        }
    };
    Some(Ok(list))
}

#[cfg(test)]
mod tests {
    use super::plain;

    #[test]
    fn git_gets_paths_without_the_windows_verbatim_prefix() {
        assert_eq!(plain(r"\\?\C:\Users\a"), r"C:\Users\a");
        assert_eq!(plain("//?/C:/Users/a"), "C:/Users/a");
        assert_eq!(plain(r"\\?\UNC\server\share"), r"\\server\share");
        assert_eq!(plain("/home/a"), "/home/a");
        assert_eq!(plain("https://example.com/x.git"), "https://example.com/x.git");
    }
}
