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
//! `rythme.lock` keeps the exact commit of each one.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::diagnostic::Diagnostic;
use crate::driver::join_use;
use crate::io::out_line;
use crate::manifest::{self, Dep, Locked, Manifest, Rev, Source, LOCK, MANIFEST};
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
    let home = if home.is_absolute() { home } else { std::env::current_dir().unwrap_or_default().join(home) };
    Ok(home.join("packages"))
}

/// A short, stable fingerprint of a text (FNV-1a).
fn fingerprint(s: &str) -> String {
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

/// The folder of a dependency of the project `m`, whether it is installed or not.
fn dep_dir(m: &Manifest, dep: &Dep) -> Result<PathBuf, Box<Diagnostic>> {
    match &dep.source {
        Source::Path(p) => Ok(join(&m.dir, p)),
        Source::Git { url, rev } => git_dir(&dep.name, &git_address(url, &m.dir), rev),
    }
}

/// How a dependency is written, for messages: `git https://..., tag v1.0.0`.
fn origin(dep: &Dep) -> String {
    match &dep.source {
        Source::Path(p) => t!("the folder `{p}`", "le dossier `{p}`"),
        Source::Git { url, rev } => format!("git `{url}`, {rev}"),
    }
}

/// The folder of an installed dependency, or why it cannot be used.
fn installed_dir(m: &Manifest, dep: &Dep, at: Span) -> Result<PathBuf, Box<Diagnostic>> {
    let dir = dep_dir(m, dep)?;
    if dir.is_dir() {
        return Ok(dir);
    }
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
        Source::Git { .. } => {
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
        if let Ok(dir) = dep_dir(&m, dep) {
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

enum GitError {
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
    let args: Vec<String> = args.iter().map(|a| plain(a)).collect();
    let dir = PathBuf::from(plain(&dir.to_string_lossy()));
    let out = Command::new("git")
        .args(&args)
        .current_dir(if dir.as_os_str().is_empty() { Path::new(".") } else { &dir })
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::null())
        .output()
        .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound { GitError::Missing } else { GitError::Failed(e.to_string()) })?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(GitError::Failed(String::from_utf8_lossy(&out.stderr).trim().to_string()))
    }
}

fn git_missing() -> Box<Diagnostic> {
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

fn git_error(e: GitError, msg: String) -> Box<Diagnostic> {
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
fn remove_all(dir: &Path) -> std::io::Result<()> {
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
/// lock file when there is one. Gives the commit, and whether it was downloaded.
fn fetch(name: &str, url: &str, rev: &Rev, base: &Path, locked: Option<&str>) -> Result<(String, bool), Box<Diagnostic>> {
    let address = git_address(url, base);
    let dir = git_dir(name, &address, rev)?;
    if dir.join(".git").exists() {
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

// ----- `rythme install` ------------------------------------------------------------

fn usage(msg: String) -> i32 {
    crate::io::err_line(&msg);
    2
}

fn report(d: &Diagnostic) {
    crate::diagnostic::show(d, crate::diagnostic::use_color());
}

/// The project of the current folder, or a message saying there is none.
fn current_project() -> Result<PathBuf, Box<Diagnostic>> {
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
    match install() {
        Ok(()) => 0,
        Err(d) => {
            report(&d);
            1
        }
    }
}

fn install() -> Result<(), Box<Diagnostic>> {
    let root = current_project()?;
    let m = manifest::load(&root)?;
    let lock = manifest::load_lock(&root)?;
    let name = &m.name;
    if m.deps.is_empty() {
        out_line(&t!(
            "`{name}` has no dependencies: add them under `[dependencies]` in `rythme.toml`",
            "`{name}` n'a pas de dépendances : ajoutez-les sous `[dependencies]` dans `rythme.toml`"
        ));
    } else {
        out_line(&t!("Installing the dependencies of `{name}`", "Installation des dépendances de `{name}`"));
    }
    let mut queue: Vec<(Manifest, usize)> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut locked: Vec<Locked> = Vec::new();
    let mut count = 0;
    let deps = m.deps.len();
    queue.push((m, 0));
    while let Some((owner, next)) = queue.pop() {
        let Some(dep) = owner.deps.get(next) else { continue };
        let dep_name = dep.name.clone();
        let (dir, line) = match &dep.source {
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
                let wanted = |l: &&Locked| l.url == *url && l.rev == *rev;
                let before = lock.iter().find(wanted).map(|l| l.commit.as_str());
                let (commit, downloaded) = fetch(&dep_name, url, rev, &owner.dir, before).map_err(|d| Box::new(d.at(dep.span)))?;
                let short: String = commit.chars().take(7).collect();
                let how = if downloaded { t!("downloaded", "téléchargé") } else { t!("already there", "déjà là") };
                let entry = Locked { name: dep_name.clone(), url: url.clone(), rev: rev.clone(), commit: commit.clone() };
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
    Ok(())
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
    vec![(MANIFEST, toml), ("src/main.ry", main), (lib_path, lib), (test_path, test), ("README.md", readme), (".gitignore", ignore)]
}

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
