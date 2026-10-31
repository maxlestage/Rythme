//! The package index: `rythme search`, `info`, `add`, `update` and
//! `publish`, with indexes and packages in local git repositories (reached
//! through `file://`) or in folders, the checksums of the files of each
//! version, and what happens without a network.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Run {
    stdout: String,
    stderr: String,
    code: i32,
}

/// A folder removed at the end of the test, with its own `RYTHME_HOME` and index.
struct Temp {
    root: PathBuf,
    index: RefCell<String>,
}

impl Temp {
    fn new(name: &str) -> Temp {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("rythme-index-{name}-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Temp { root: dir.canonicalize().unwrap(), index: RefCell::new(String::new()) }
    }

    fn path(&self, p: &str) -> PathBuf {
        self.root.join(p)
    }

    fn write(&self, files: &[(&str, &str)]) {
        for (p, text) in files {
            let path = self.path(p);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
    }

    fn read(&self, p: &str) -> String {
        std::fs::read_to_string(self.path(p)).unwrap()
    }

    fn home(&self) -> PathBuf {
        self.path("home")
    }

    /// The index the next commands use (`RYTHME_INDEX`).
    fn use_index(&self, address: &str) {
        *self.index.borrow_mut() = address.to_string();
    }

    fn rythme(&self, lang: &str, dir: &str, args: &[&str]) -> Run {
        let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .args(args)
            .current_dir(self.path(dir))
            .env("RYTHME_LANG", lang)
            .env("NO_COLOR", "1")
            .env("RYTHME_HOME", self.home())
            .env("RYTHME_INDEX", self.index.borrow().as_str())
            .output()
            .expect("run rythme");
        let text = |bytes: &[u8]| {
            let s = String::from_utf8_lossy(bytes).replace("\r\n", "\n");
            if cfg!(windows) {
                s.replace('\\', "/")
            } else {
                s
            }
        };
        Run { stdout: text(&out.stdout), stderr: text(&out.stderr), code: out.status.code().unwrap_or(-1) }
    }

    /// A git repository in `dir` with these files, committed; gives the commit.
    fn repo(&self, dir: &str, files: &[(&str, &str)]) -> String {
        std::fs::create_dir_all(self.path(dir)).unwrap();
        git(&self.path(dir), &["init", "--quiet"]);
        self.commit(dir, files)
    }

    fn commit(&self, dir: &str, files: &[(&str, &str)]) -> String {
        let files: Vec<(String, &str)> = files.iter().map(|(p, t)| (format!("{dir}/{p}"), *t)).collect();
        let files: Vec<(&str, &str)> = files.iter().map(|(p, t)| (p.as_str(), *t)).collect();
        self.write(&files);
        let repo = self.path(dir);
        git(&repo, &["add", "--all"]);
        git(&repo, &["commit", "--quiet", "--message", "work"]);
        git(&repo, &["rev-parse", "HEAD"])
    }

    /// The address of a folder of the test, as `file://`.
    fn url(&self, dir: &str) -> String {
        file_url(&self.path(dir))
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        if std::fs::remove_dir_all(&self.root).is_err() {
            make_writable(&self.root);
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
}

fn make_writable(p: &Path) {
    if let Ok(meta) = std::fs::symlink_metadata(p) {
        let mut perms = meta.permissions();
        #[allow(clippy::permissions_set_readonly_false)]
        perms.set_readonly(false);
        let _ = std::fs::set_permissions(p, perms);
        if meta.is_dir() {
            for e in std::fs::read_dir(p).into_iter().flatten().flatten() {
                make_writable(&e.path());
            }
        }
    }
}

fn git_raw(dir: &Path, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .args(["-c", "user.name=Test", "-c", "user.email=test@example.com", "-c", "init.defaultBranch=main", "-c", "commit.gpgsign=false"])
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git is needed by these tests");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}

fn git(dir: &Path, args: &[&str]) -> String {
    String::from_utf8_lossy(&git_raw(dir, args)).trim().to_string()
}

/// `file:///...` for a folder, on every system (without the `\\?\` of Windows).
fn file_url(p: &Path) -> String {
    let s = p.display().to_string();
    let s = s.strip_prefix(r"\\?\").unwrap_or(&s).replace('\\', "/");
    if s.starts_with('/') {
        format!("file://{s}")
    } else {
        format!("file:///{s}")
    }
}

/// The checksum of the files of a commit, computed here as the manual and
/// `packaging/index-template/README.md` describe it: for each file in the
/// order of the bytes of its path, its mode, a space, its path and a zero
/// byte, then its size, a zero byte and its content.
fn checksum(repo: &Path, commit: &str) -> String {
    let listing = git_raw(repo, &["ls-tree", "-r", "-z", "--full-tree", commit]);
    let mut files: Vec<(String, String, String, Vec<u8>)> = Vec::new();
    for item in listing.split(|b| *b == 0).filter(|i| !i.is_empty()) {
        let tab = item.iter().position(|b| *b == b'\t').unwrap();
        let head = String::from_utf8(item[..tab].to_vec()).unwrap();
        let mut parts = head.split(' ').map(str::to_string);
        files.push((parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap(), item[tab + 1..].to_vec()));
    }
    files.sort_by(|a, b| a.3.cmp(&b.3));
    let mut data = Vec::new();
    for (mode, kind, object, path) in files {
        data.extend_from_slice(mode.as_bytes());
        data.push(b' ');
        data.extend_from_slice(&path);
        data.push(0);
        if kind == "blob" {
            let content = git_raw(repo, &["cat-file", "blob", &object]);
            data.extend_from_slice(content.len().to_string().as_bytes());
            data.push(0);
            data.extend_from_slice(&content);
        } else {
            data.extend_from_slice(object.as_bytes());
            data.push(0);
        }
    }
    format!("sha256:{}", rythme::sha256::hex(&data))
}

#[track_caller]
fn ok(r: &Run) {
    assert_eq!(r.code, 0, "stdout:\n{}\nstderr:\n{}", r.stdout, r.stderr);
}

/// A version of a package: its files, committed and tagged `v{version}` in
/// the repository `dir`. Gives the line of the index for it.
fn release(t: &Temp, dir: &str, name: &str, version: &str, code: &str) -> String {
    release_with(t, dir, name, version, code, "")
}

/// [`release`], with more lines at the end of its `rythme.toml` (its dependencies).
fn release_with(t: &Temp, dir: &str, name: &str, version: &str, code: &str, more: &str) -> String {
    let toml = format!(
        "name = \"{name}\"\nversion = \"{version}\"\nentry = \"src/{name}.ry\"\ndescription = \"The package {name}\"\nlicense = \"MIT\"\nrepository = \"{}\"\nkeywords = [\"math\", \"vectors\"]\n{more}",
        t.url(dir)
    );
    let files = [
        ("rythme.toml", toml.as_str()),
        (&*format!("src/{name}.ry"), &*format!("version = \"{version}\"\n")),
        ("src/vectors.ry", code),
        ("tests/vectors_test.ry", &*format!("use {name}/vectors\ntest \"add\" = assert_eq(vectors.add(1, 2), 3)\n")),
    ];
    let commit = if t.path(dir).join(".git").exists() { t.commit(dir, &files) } else { t.repo(dir, &files) };
    git(&t.path(dir), &["tag", &format!("v{version}")]);
    let sum = checksum(&t.path(dir), &commit);
    format!("\n[[version]]\nversion = \"{version}\"\ntag = \"v{version}\"\ncommit = \"{commit}\"\nchecksum = \"{sum}\"\n")
}

/// The head of the entry of a package of the index.
fn entry_head(t: &Temp, dir: &str, name: &str, description: &str) -> String {
    format!(
        "name = \"{name}\"\ndescription = \"{description}\"\nrepository = \"{}\"\nlicense = \"MIT\"\nkeywords = [\"math\", \"vectors\"]\nauthors = [\"Ada <ada@example.com>\"]\n",
        t.url(dir)
    )
}

const ADD: &str = "## Adds two numbers.\nfn add(a, b) = a + b\n";

/// An index in a git repository with the package `mathlib` 1.0.0, used through `file://`.
fn index_with_mathlib(t: &Temp) -> String {
    let v1 = release(t, "mathlib", "mathlib", "1.0.0", ADD);
    let text = format!("{}{v1}", entry_head(t, "mathlib", "mathlib", "Vectors and a few numbers"));
    t.repo("index", &[("packages/mathlib.toml", &text), ("README.md", "An index.\n")]);
    t.use_index(&t.url("index"));
    text
}

fn new_app(t: &Temp) {
    ok(&t.rythme("en", "", &["new", "app"]));
    t.write(&[("app/src/main.ry", "use mathlib/vectors\nuse mathlib\nprint(vectors.add(1, 2), mathlib.version)\n")]);
}

#[test]
fn add_installs_a_package_of_the_index_and_locks_its_commit() {
    let t = Temp::new("add");
    let entry = index_with_mathlib(&t);
    new_app(&t);
    let r = t.rythme("en", "app", &["add", "mathlib"]);
    ok(&r);
    assert!(r.stdout.contains("✓ mathlib 1.0.0: ^1.0.0 from the index, commit"), "{}", r.stdout);
    assert!(r.stdout.contains("Added `mathlib = { version = \"^1.0.0\" }` to rythme.toml (1.0.0)"), "{}", r.stdout);
    let toml = t.read("app/rythme.toml");
    // At the end of [dependencies], after its comments, which stay.
    assert!(toml.ends_with("# tools = { path = \"../tools\" }\nmathlib = { version = \"^1.0.0\" }\n"), "{toml}");
    let lock = t.read("app/rythme.lock");
    let commit = git(&t.path("mathlib"), &["rev-parse", "v1.0.0"]);
    let sum = checksum(&t.path("mathlib"), &commit);
    let wanted = format!(
        "[[package]]\nname = \"mathlib\"\nversion = \"1.0.0\"\nindex = \"{}\"\ngit = \"{}\"\ntag = \"v1.0.0\"\ncommit = \"{commit}\"\nchecksum = \"{sum}\"\n",
        t.url("index"),
        t.url("mathlib")
    );
    assert!(lock.contains(&wanted), "{lock}\n---\n{wanted}");
    assert!(entry.contains(&sum));
    let r = t.rythme("en", "app", &["run"]);
    ok(&r);
    assert_eq!(r.stdout, "3 1.0.0\n");
    ok(&t.rythme("en", "app", &["check"]));
    // Installing again finds it there, without the index.
    std::fs::rename(t.path("index"), t.path("index-away")).unwrap();
    let r = t.rythme("fr", "app", &["install"]);
    ok(&r);
    assert!(r.stdout.contains("^1.0.0 depuis l'index, commit") && r.stdout.contains("(déjà là)"), "{}", r.stdout);
    assert_eq!(t.read("app/rythme.lock"), lock);
    std::fs::rename(t.path("index-away"), t.path("index")).unwrap();

    // A range, an unknown package, a bad name, the project itself.
    let r = t.rythme("en", "app", &["add", "mathlib@=1.0.0"]);
    ok(&r);
    assert!(t.read("app/rythme.toml").contains("mathlib = { version = \"=1.0.0\" }\n"));
    assert_eq!(t.read("app/rythme.toml").lines().filter(|l| l.starts_with("mathlib =")).count(), 1, "replaced, not added twice");
    let r = t.rythme("en", "app", &["add", "mathlab"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E025]: no package `mathlab` in the package index"), "{}", r.stderr);
    assert!(r.stderr.contains("did you mean `mathlib`?"), "{}", r.stderr);
    let r = t.rythme("en", "app", &["add", "mathlib@^2"]);
    assert_eq!(r.code, 1);
    assert!(
        r.stderr.contains("no version of `mathlib` in the package index fits `^2`") && r.stderr.contains("its versions: 1.0.0"),
        "{}",
        r.stderr
    );
    for bad in ["../evil", "a/b", "Math", "..", "x y"] {
        let r = t.rythme("en", "app", &["add", bad]);
        assert_eq!(r.code, 1, "{bad}");
        assert!(r.stderr.contains("cannot be the name of a package"), "{bad}: {}", r.stderr);
    }
    let r = t.rythme("en", "app", &["add", "app"]);
    assert!(r.stderr.contains("cannot depend on itself"), "{}", r.stderr);
    let r = t.rythme("en", "app", &["add"]);
    assert_eq!(r.code, 2);
    assert!(t.read("app/rythme.toml").contains("mathlib = { version = \"=1.0.0\" }\n"), "unchanged by the failures");
    // Outside of a project.
    let r = t.rythme("en", "", &["add", "mathlib"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("no `rythme.toml` in this folder or above it"), "{}", r.stderr);
}

#[test]
fn update_takes_the_newest_version_in_the_range() {
    let t = Temp::new("update");
    let mut entry = index_with_mathlib(&t);
    new_app(&t);
    ok(&t.rythme("en", "app", &["add", "mathlib"]));
    // New versions: 1.1.0 fits ^1.0.0, 2.0.0 does not.
    entry.push_str(&release(&t, "mathlib", "mathlib", "1.1.0", ADD));
    entry.push_str(&release(&t, "mathlib", "mathlib", "2.0.0", "fn add(a, b) = a * b\n"));
    t.commit("index", &[("packages/mathlib.toml", &entry)]);
    // `rythme install` keeps the version of rythme.lock.
    let r = t.rythme("en", "app", &["install"]);
    ok(&r);
    assert!(r.stdout.contains("✓ mathlib 1.0.0"), "{}", r.stdout);
    let r = t.rythme("en", "app", &["update"]);
    ok(&r);
    assert!(r.stdout.contains("Updating the dependencies of `app`"), "{}", r.stdout);
    assert!(r.stdout.contains("✓ mathlib 1.1.0: ^1.0.0 from the index"), "{}", r.stdout);
    assert!(r.stdout.contains("Updated:\n  mathlib 1.0.0 → 1.1.0"), "{}", r.stdout);
    assert!(t.read("app/rythme.lock").contains("version = \"1.1.0\""));
    assert_eq!(t.rythme("en", "app", &["run"]).stdout, "3 1.1.0\n");
    let r = t.rythme("fr", "app", &["update", "mathlib"]);
    ok(&r);
    assert!(r.stdout.contains("Tout était déjà à jour"), "{}", r.stdout);
    let r = t.rythme("en", "app", &["update", "mathlb"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("`mathlb` is not a dependency of `app`") && r.stderr.contains("rythme update mathlib"), "{}", r.stderr);
    // A withdrawn version is no longer chosen.
    let withdrawn = entry.replace("version = \"1.1.0\"\n", "version = \"1.1.0\"\nyanked = \"add gave wrong results: use 1.0.0\"\n");
    t.commit("index", &[("packages/mathlib.toml", &withdrawn)]);
    let r = t.rythme("en", "app", &["update"]);
    ok(&r);
    assert!(r.stdout.contains("mathlib 1.1.0 → 1.0.0"), "{}", r.stdout);
    let r = t.rythme("en", "app", &["info", "--update", "mathlib"]);
    ok(&r);
    assert!(r.stdout.contains("1.1.0 was withdrawn: add gave wrong results"), "{}", r.stdout);
    assert!(r.stdout.contains("versions    2.0.0, 1.1.0 (withdrawn), 1.0.0"), "{}", r.stdout);
    // A wider range in rythme.toml, then `rythme update`.
    let toml = t.read("app/rythme.toml").replace("\"^1.0.0\"", "\">=1.0.0, <3.0.0\"");
    t.write(&[("app/rythme.toml", &toml)]);
    let r = t.rythme("en", "app", &["update"]);
    ok(&r);
    assert_eq!(t.rythme("en", "app", &["run"]).stdout, "2 2.0.0\n");
}

#[test]
fn search_and_info_read_a_copy_of_the_index() {
    let t = Temp::new("search");
    let mut entry = index_with_mathlib(&t);
    entry.push_str(&release(&t, "mathlib", "mathlib", "1.1.0", ADD));
    let geometry = format!(
        "{}{}",
        entry_head(&t, "geometry", "geometry", "Shapes, areas and perimeters, with vectors\u{1b}[31m")
            .replace("[\"math\", \"vectors\"]", "[\"geometry\", \"shapes\"]"),
        release(&t, "geometry", "geometry", "0.3.1", ADD)
    );
    t.commit("index", &[("packages/mathlib.toml", &entry), ("packages/geometry.toml", &geometry)]);
    let r = t.rythme("en", "", &["search", "vectors"]);
    ok(&r);
    let lines: Vec<&str> = r.stdout.lines().collect();
    // The name first, then the words; control characters of the index are not printed.
    assert_eq!(lines[0], "mathlib 1.1.0    Vectors and a few numbers", "{}", r.stdout);
    assert_eq!(lines[1], "geometry 0.3.1   Shapes, areas and perimeters, with vectors [31m", "{}", r.stdout);
    assert!(!r.stdout.contains('\u{1b}'));
    assert!(r.stdout.contains(&format!("2 package(s) found for `vectors` in {} (copy of ", t.url("index"))), "{}", r.stdout);
    assert!(r.stdout.contains("More about one: `rythme info mathlib`"), "{}", r.stdout);
    let r = t.rythme("en", "", &["search", "area"]);
    assert!(r.stdout.starts_with("geometry 0.3.1") && r.stdout.contains("1 package(s) found"), "{}", r.stdout);
    let r = t.rythme("en", "", &["search", "nothing"]);
    ok(&r);
    assert!(r.stdout.contains("No package found for `nothing`") && r.stdout.contains("rythme search --update nothing"), "{}", r.stdout);
    let r = t.rythme("en", "", &["search"]);
    assert!(r.stdout.contains("2 package(s) in"), "{}", r.stdout);
    let r = t.rythme("en", "", &["info", "mathlib"]);
    ok(&r);
    assert!(r.stdout.starts_with("mathlib 1.1.0\nVectors and a few numbers\n"), "{}", r.stdout);
    for line in [
        "  repository  file://",
        "  license     MIT",
        "  keywords    math, vectors",
        "  authors     Ada <ada@example.com>",
        "  versions    1.1.0, 1.0.0",
    ] {
        assert!(r.stdout.contains(line), "{line}: {}", r.stdout);
    }
    assert!(r.stdout.contains("`rythme add mathlib` (writes `mathlib = { version = \"^1.1.0\" }`"), "{}", r.stdout);
    let r = t.rythme("en", "", &["info", "../../etc/passwd"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("cannot be the name of a package"), "{}", r.stderr);
    assert_eq!(t.rythme("en", "", &["info"]).code, 2);
    assert_eq!(t.rythme("en", "", &["search", "--fast"]).code, 2);

    // Without a network: the copy is used, and says so with --update.
    std::fs::rename(t.path("index"), t.path("index-away")).unwrap();
    let r = t.rythme("en", "", &["search", "vectors"]);
    ok(&r);
    assert!(r.stdout.contains("2 package(s) found"), "{}", r.stdout);
    let r = t.rythme("en", "", &["search", "--update", "vectors"]);
    ok(&r);
    assert!(r.stderr.contains("warning: cannot update the package index from") && r.stderr.contains("using the copy of"), "{}", r.stderr);
    assert!(r.stdout.contains("2 package(s) found"), "{}", r.stdout);
    // A package missing from the copy: the newest copy is asked for, then the copy answers.
    let r = t.rythme("en", "", &["info", "newthing"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("cannot update the package index") && r.stderr.contains("no package `newthing`"), "{}", r.stderr);
    // Without a copy, it is an error that says what to do.
    let _ = std::fs::remove_dir_all(t.home());
    let r = t.rythme("en", "", &["search", "vectors"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E025]: cannot download the package index from"), "{}", r.stderr);
    assert!(
        r.stderr.contains("RYTHME_INDEX") && r.stderr.contains("dependencies from git or from a folder work without the index"),
        "{}",
        r.stderr
    );
    std::fs::rename(t.path("index-away"), t.path("index")).unwrap();

    // A folder of this computer is an index too, read as it is.
    t.write(&[("folder/packages/geometry.toml", &geometry)]);
    t.use_index(&t.path("folder").display().to_string());
    let r = t.rythme("en", "", &["search", "shapes"]);
    ok(&r);
    assert!(r.stdout.contains("1 package(s) found for `shapes` in ") && !r.stdout.contains("copy of"), "{}", r.stdout);
    t.use_index(&t.path("nothing").display().to_string());
    let r = t.rythme("en", "", &["search"]);
    assert!(r.stderr.contains("is not a package index: it has no folder `packages`") && r.stderr.contains("RYTHME_INDEX"), "{}", r.stderr);
    t.use_index("http://example.com/index");
    let r = t.rythme("en", "", &["search"]);
    assert!(r.stderr.contains("only with `https://`, `ssh://` or from this computer (`file://`), not with `http`"), "{}", r.stderr);
}

#[test]
fn what_comes_from_the_index_is_checked() {
    let t = Temp::new("checked");
    let entry = index_with_mathlib(&t);
    new_app(&t);
    let before = t.read("app/rythme.toml");
    // Files that do not have the checksum of the index: nothing is installed.
    let sum = checksum(&t.path("mathlib"), &git(&t.path("mathlib"), &["rev-parse", "v1.0.0"]));
    let wrong = entry.replace(&sum, &format!("sha256:{}", "0".repeat(64)));
    assert_ne!(wrong, entry);
    t.commit("index", &[("packages/mathlib.toml", &wrong)]);
    let r = t.rythme("en", "app", &["add", "mathlib"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E025]: the files of `mathlib` 1.0.0 do not match the checksum of the package index"), "{}", r.stderr);
    assert!(r.stderr.contains("nothing was installed, and nothing of the package ran"), "{}", r.stderr);
    assert!(r.stderr.contains("`rythme.toml` was left as it was"), "{}", r.stderr);
    assert_eq!(t.read("app/rythme.toml"), before);
    let kept: Vec<_> = std::fs::read_dir(t.home().join("packages")).map(|rd| rd.flatten().collect()).unwrap_or_default();
    let inside: Vec<_> = kept.iter().flat_map(|e| std::fs::read_dir(e.path()).into_iter().flatten().flatten().map(|f| f.path())).collect();
    assert!(inside.is_empty(), "nothing kept: {inside:?}");
    // A commit that the repository does not have.
    let absent = entry.replace(&git(&t.path("mathlib"), &["rev-parse", "v1.0.0"]), &"a".repeat(40));
    t.commit("index", &[("packages/mathlib.toml", &absent)]);
    let r = t.rythme("en", "app", &["add", "mathlib"]);
    assert!(r.stderr.contains(&format!("the commit `{}` of `mathlib` 1.0.0 does not exist in", "a".repeat(40))), "{}", r.stderr);
    // Addresses outside of https, ssh and file are refused.
    let http = entry.replace(&t.url("mathlib"), "http://example.com/mathlib");
    t.commit("index", &[("packages/mathlib.toml", &http)]);
    let r = t.rythme("en", "app", &["info", "--update", "mathlib"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("Rythme downloads packages only with `https://`"), "{}", r.stderr);
    let ext = entry.replace(&t.url("mathlib"), "ext::sh");
    t.commit("index", &[("packages/mathlib.toml", &ext)]);
    assert!(t.rythme("en", "app", &["add", "mathlib"]).stderr.contains("not with `ext`"));
    // An entry whose name is not the one of its file.
    t.commit("index", &[("packages/mathlib.toml", &entry.replace("name = \"mathlib\"", "name = \"other\""))]);
    let r = t.rythme("en", "app", &["info", "--update", "mathlib"]);
    assert!(r.stderr.contains("this entry of the package index is for `other`, but its file is named for `mathlib`"), "{}", r.stderr);
    // A version whose rythme.toml says something else than the index.
    let other = release(&t, "liar", "liar", "1.0.0", ADD);
    let lie = format!("{}{}", entry_head(&t, "liar", "mathlib", "A lie"), other);
    t.commit("index", &[("packages/mathlib.toml", &lie)]);
    let r = t.rythme("en", "app", &["add", "mathlib"]);
    assert!(r.stderr.contains("the package index gives `mathlib` 1.0.0, but its `rythme.toml` says `liar` 1.0.0"), "{}", r.stderr);
    assert_eq!(t.read("app/rythme.toml"), before);
}

#[test]
fn publish_checks_the_package_then_prints_its_entry() {
    let t = Temp::new("publish");
    t.repo("index", &[("packages/.keep", "")]);
    t.use_index(&t.url("index"));
    t.repo(
        "shapes",
        &[
            ("rythme.toml", "name = \"shapes\"\nversion = \"0.1.0\"\n"),
            ("src/main.ry", "print(1)\n"),
            ("src/circle.ry", "fn area(r) = 3 * r * r\n"),
            ("tests/circle_test.ry", "use shapes/circle\ntest \"area\" = assert_eq(circle.area(1), 3)\n"),
        ],
    );
    let r = t.rythme("en", "shapes", &["publish"]);
    assert_eq!(r.code, 1);
    for wanted in [
        "error[E025]: `rythme.toml` has no `description`",
        "`rythme.toml` has no `license`",
        "add a line like `license = \"MIT\"` at the top",
        "`rythme.toml` has no `repository`",
        "no tag `v0.1.0` for the version 0.1.0",
        "git tag v0.1.0",
        "problem(s) to fix before publishing: nothing was sent anywhere",
    ] {
        assert!(r.stderr.contains(wanted), "{wanted}: {}", r.stderr);
    }
    let toml = format!(
        "name = \"shapes\"\nversion = \"0.1.0\"\ndescription = \"Circles and squares\"\nlicense = \"MIT\"\nrepository = \"{}\"\nkeywords = [\"geometry\"]\nauthors = [\"Ada <ada@example.com>\"]\n",
        t.url("shapes")
    );
    let commit = t.commit("shapes", &[("rythme.toml", &toml)]);
    git(&t.path("shapes"), &["tag", "v0.1.0"]);
    t.write(&[("shapes/notes.txt", "draft\n")]);
    let r = t.rythme("en", "shapes", &["publish"]);
    assert!(r.stderr.contains("some changes are not committed") && r.stderr.contains("notes.txt"), "{}", r.stderr);
    std::fs::remove_file(t.path("shapes/notes.txt")).unwrap();
    // Failing tests stop it too.
    t.write(&[("shapes/tests/circle_test.ry", "use shapes/circle\ntest \"area\" = assert_eq(circle.area(1), 4)\n")]);
    let r = t.rythme("en", "shapes", &["publish"]);
    assert!(r.stderr.contains("`rythme test` failed"), "{}", r.stderr);
    git(&t.path("shapes"), &["checkout", "--", "tests"]);

    let r = t.rythme("en", "shapes", &["publish"]);
    ok(&r);
    let sum = checksum(&t.path("shapes"), &commit);
    let text = format!(
        "    name = \"shapes\"\n    description = \"Circles and squares\"\n    repository = \"{}\"\n    license = \"MIT\"\n    keywords = [\"geometry\"]\n    authors = [\"Ada <ada@example.com>\"]\n\n    [[version]]\n    version = \"0.1.0\"\n    tag = \"v0.1.0\"\n    commit = \"{commit}\"\n    checksum = \"{sum}\"\n",
        t.url("shapes")
    );
    assert!(r.stdout.contains(&text), "{}\n---\n{text}", r.stdout);
    assert!(r.stdout.contains("✓ `shapes` 0.1.0 is ready for the package index."), "{}", r.stdout);
    assert!(r.stdout.contains("Rythme never publishes anything by itself"), "{}", r.stdout);
    assert!(r.stdout.contains("Create the file packages/shapes.toml with:"), "{}", r.stdout);
    assert!(r.stdout.contains("its checks run `rythme publish --verify packages/shapes.toml`"), "{}", r.stdout);
    assert_eq!(git(&t.path("index"), &["log", "--oneline"]).lines().count(), 1, "nothing sent to the index");

    // The entry, in the index: `--verify` agrees, and the version cannot be published twice.
    let entry: String = text.lines().map(|l| l.strip_prefix("    ").unwrap_or(l)).collect::<Vec<_>>().join("\n") + "\n";
    t.commit("index", &[("packages/shapes.toml", &entry)]);
    let r = t.rythme("en", "index", &["publish", "--verify", "packages/shapes.toml"]);
    ok(&r);
    assert!(r.stdout.contains("✓ packages/shapes.toml: `shapes`, 1 version(s) verified"), "{}", r.stdout);
    let r = t.rythme("en", "shapes", &["publish"]);
    assert!(r.stderr.contains("the version 0.1.0 of `shapes` is already in the package index"), "{}", r.stderr);
    // A next version: only its block, to add at the end of the file.
    let toml2 = toml.replace("0.1.0", "0.2.0");
    t.commit("shapes", &[("rythme.toml", &toml2)]);
    git(&t.path("shapes"), &["tag", "v0.2.0"]);
    let r = t.rythme("en", "shapes", &["publish"]);
    ok(&r);
    assert!(r.stdout.contains("Add at the end of packages/shapes.toml:\n\n    [[version]]\n    version = \"0.2.0\""), "{}", r.stdout);
    assert!(!r.stdout.contains("    name = \"shapes\""), "{}", r.stdout);
    // An index in a folder: the text to write there, and nothing written.
    t.write(&[("folder/packages/.keep", "")]);
    t.use_index(&t.path("folder").display().to_string());
    let r = t.rythme("en", "shapes", &["publish"]);
    ok(&r);
    assert!(r.stdout.contains("where Rythme writes nothing.\nCreate its file packages/shapes.toml with:"), "{}", r.stdout);
    assert!(r.stdout.contains("    name = \"shapes\"") && !r.stdout.contains("pull request"), "{}", r.stdout);
    assert!(!t.path("folder/packages/shapes.toml").exists());
    t.use_index(&t.url("index"));

    // `--verify` finds what is wrong.
    let cases: &[(&str, &str, &str)] = &[
        (&sum, "sha256:xyz", "is not a checksum"),
        ("tag = \"v0.1.0\"", "tag = \"v9\"", "the tag `v9` of"),
        ("license = \"MIT\"\n", "", "has no `license`"),
        ("version = \"0.1.0\"\n", "version = \"0.1.0\"\nyankd = \"x\"\n", "unknown key `yankd`"),
        ("name = \"shapes\"", "name = \"other\"", "is for `other`"),
    ];
    for (from, to, wanted) in cases {
        let changed = entry.replacen(from, to, 1);
        t.write(&[("index/packages/shapes.toml", &changed)]);
        let r = t.rythme("en", "index", &["publish", "--verify", "packages/shapes.toml"]);
        assert_eq!(r.code, 1, "{wanted}: {}", r.stdout);
        assert!(r.stderr.contains(wanted), "{wanted}: {}", r.stderr);
        assert!(r.stdout.contains("✗ packages/shapes.toml"), "{}", r.stdout);
    }
    let other_sum = format!("sha256:{}", "0".repeat(64));
    t.write(&[("index/packages/shapes.toml", &entry.replace(&sum, &other_sum))]);
    let r = t.rythme("en", "index", &["publish", "--verify", "packages/shapes.toml"]);
    assert!(r.stderr.contains("0.1.0: the files of the commit do not have the checksum of the entry"), "{}", r.stderr);
    assert_eq!(t.rythme("en", "index", &["publish", "--verify"]).code, 2);
    assert_eq!(t.rythme("en", "shapes", &["publish", "now"]).code, 2);
}

#[test]
fn rythme_toml_knows_the_keys_of_the_index() {
    let t = Temp::new("toml");
    let cases: &[(&str, &[&str])] = &[
        ("keywords = \"math\"\n", &["`keywords` is a list of texts", "keywords = [\"math\", \"vectors\"]"]),
        ("keywords = [\"math\" \"x\"]\n", &["expected `,` or `]` here"]),
        ("keywords = [\"math\",\n", &["expected `,` or `]` here"]),
        ("index = \"git://example.com/index\"\n", &["not with `git`"]),
        ("repository = \"../here\"\n", &["is not the address of a repository"]),
        ("[dependencies]\nlib = { version = \"abc\" }\n", &["`abc` is not a range of versions", "\"^1.2.0\""]),
        ("[dependencies]\nlib = { version = \"1.0\", git = \"x\" }\n", &["give only one"]),
        ("[dependencies]\nlib = \"1.0\"\n", &["is written as a table", "write `lib = { version = \"1.0\" }`"]),
        ("[dependencies]\nlib = { versoin = \"1.0\" }\n", &["unknown key `versoin`", "did you mean `version`?"]),
        ("[dependencies]\nlib = { tag = \"v1\" }\n", &["needs `version`, `git` or `path`"]),
        ("licence = \"MIT\"\n", &["unknown key `licence`", "did you mean `license`?"]),
    ];
    for (more, wanted) in cases {
        let toml = format!("name = \"app\"\nversion = \"0.1.0\"\n{more}");
        t.write(&[("app/rythme.toml", &toml), ("app/src/main.ry", "print(1)\n")]);
        let r = t.rythme("en", "app", &["run"]);
        assert_eq!(r.code, 1, "{toml}\n{}", r.stderr);
        for w in *wanted {
            assert!(r.stderr.contains(w), "{toml}: expected `{w}` in\n{}", r.stderr);
        }
    }
    // Every key together, and a dependency of the index not installed yet.
    t.write(&[(
        "app/rythme.toml",
        "name = \"app\"\nversion = \"0.1.0\"\ndescription = \"An app\"\nlicense = \"MIT\"\nrepository = \"https://example.com/app\"\nkeywords = [\"one\", 'two',]\nauthors = []\nindex = \"../folder\"\n\n[dependencies]\nlib = { version = \"~1.2\" }\n",
    )]);
    t.write(&[("app/src/main.ry", "use lib/x\nprint(1)\n")]);
    let r = t.rythme("en", "app", &["run"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("the package `lib` is not installed"), "{}", r.stderr);
    assert!(r.stderr.contains("asks for it from the package index, version ~1.2"), "{}", r.stderr);
    // The index of rythme.toml wins over RYTHME_INDEX.
    t.write(&[("folder/packages/.keep", "")]);
    let r = t.rythme("en", "app", &["install"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("no package `lib` in the package index") && r.stderr.contains("the index: ../folder"), "{}", r.stderr);
}

#[test]
fn packages_of_the_index_use_packages_of_the_index() {
    let t = Temp::new("nested");
    let mut tools = format!("{}{}", entry_head(&t, "tools", "tools", "Tools"), release(&t, "tools", "tools", "1.0.0", ADD));
    let lib = release_with(
        &t,
        "mathlib",
        "mathlib",
        "1.0.0",
        "use tools/vectors as t\nfn add(a, b) = t.add(a, b) * 10\n",
        "\n[dependencies]\ntools = { version = \"^1.0.0\" }\n",
    );
    let mathlib = format!("{}{lib}", entry_head(&t, "mathlib", "mathlib", "Maths"));
    t.repo("index", &[("packages/tools.toml", &tools), ("packages/mathlib.toml", &mathlib)]);
    t.use_index(&t.url("index"));
    new_app(&t);
    t.write(&[("app/src/main.ry", "use mathlib/vectors\nprint(vectors.add(1, 2))\n")]);
    let r = t.rythme("en", "app", &["add", "mathlib"]);
    ok(&r);
    assert!(r.stdout.contains("✓ mathlib 1.0.0") && r.stdout.contains("✓ tools 1.0.0: ^1.0.0 from the index"), "{}", r.stdout);
    assert!(r.stdout.contains("2 package(s) ready (1 needed by the other ones)"), "{}", r.stdout);
    assert_eq!(t.rythme("en", "app", &["run"]).stdout, "30\n");
    ok(&t.rythme("en", "app", &["check"]));
    // A newer tools: the lock of the project keeps 1.0.0 for mathlib, until `rythme update`.
    tools.push_str(&release(&t, "tools", "tools", "1.1.0", "fn add(a, b) = a + b + 1\n"));
    t.commit("index", &[("packages/tools.toml", &tools)]);
    ok(&t.rythme("en", "app", &["install"]));
    assert_eq!(t.rythme("en", "app", &["run"]).stdout, "30\n");
    let r = t.rythme("en", "app", &["update", "tools"]);
    ok(&r);
    assert!(r.stdout.contains("tools 1.0.0 → 1.1.0"), "{}", r.stdout);
    assert_eq!(t.rythme("en", "app", &["run"]).stdout, "40\n");
}

#[test]
fn the_template_of_the_index_is_an_index() {
    let t = Temp::new("template");
    let template = Path::new(env!("CARGO_MANIFEST_DIR")).join("packaging/index-template");
    t.use_index(&template.display().to_string());
    let r = t.rythme("en", "", &["info", "mathlib"]);
    ok(&r);
    assert!(r.stdout.contains("  versions    1.1.1, 1.1.0 (withdrawn), 1.0.0"), "{}", r.stdout);
    let r = t.rythme("en", "", &["search", "matrices"]);
    assert!(r.stdout.starts_with("mathlib 1.1.1"), "{}", r.stdout);
    // The example has every field that `rythme publish --verify` asks for.
    let entry = rythme::index::read_entry(&template.join("packages/mathlib.toml"), "mathlib", true);
    assert!(entry.is_ok(), "{:?}", entry.err().map(|d| d.message));
    let workflow = std::fs::read_to_string(template.join(".github/workflows/check.yml")).unwrap();
    assert!(workflow.contains("xargs rythme publish --verify < changed.txt"), "{workflow}");
    let readme = std::fs::read_to_string(template.join("README.md")).unwrap();
    assert!(readme.contains(rythme::index::DEFAULT_INDEX), "the README names the default index");
}

#[test]
fn messages_of_the_index_in_every_language() {
    let t = Temp::new("languages");
    t.write(&[("folder/packages/.keep", "")]);
    t.use_index(&t.path("folder").display().to_string());
    let wanted = [
        ("en", "error[E025]: no package `nothing` in the package index", "0 package(s) in "),
        ("fr", "erreur[E025]: aucun paquet `nothing` dans l'index des paquets", "0 paquet(s) dans "),
        ("es", "error[E025]: ningún paquete `nothing` en el índice de paquetes", "0 paquete(s) en "),
        ("de", "Fehler[E025]: kein Paket `nothing` im Paketindex", "0 Paket(e) in "),
        ("pt", "erro[E025]: nenhum pacote `nothing` no índice de pacotes", "0 pacote(s) em "),
        ("zh", "错误[E025]: 包索引中没有包 `nothing`", " 中有 0 个包"),
        ("it", "errore[E025]: nessun pacchetto `nothing` nell'indice dei pacchetti", "0 pacchetto/i in "),
        ("ja", "エラー[E025]: パッケージインデックスにパッケージ `nothing` はない", " にパッケージが 0 個"),
    ];
    for (lang, error, count) in wanted {
        let r = t.rythme(lang, "", &["info", "nothing"]);
        assert_eq!(r.code, 1);
        assert!(r.stderr.contains(error), "{lang}: {}", r.stderr);
        let r = t.rythme(lang, "", &["search"]);
        ok(&r);
        assert!(r.stdout.contains(count), "{lang}: {}", r.stdout);
        let r = t.rythme(lang, "", &["help", "packages"]);
        ok(&r);
        assert!(r.stdout.contains("rythme publish") && r.stdout.contains("RYTHME_INDEX"), "{lang}: {}", r.stdout);
    }
}
