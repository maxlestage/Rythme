//! Projects and packages: `rythme new`, `rythme.toml`, `rythme install` with
//! dependencies from local git repositories and folders, `rythme.lock`,
//! `use package/file`, and the commands without a file at the root of a
//! project.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Run {
    stdout: String,
    stderr: String,
    code: i32,
}

/// A folder removed at the end of the test.
struct Temp(PathBuf);

impl Temp {
    fn new(name: &str) -> Temp {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("rythme-projects-{name}-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Temp(dir.canonicalize().unwrap())
    }

    fn path(&self, p: &str) -> PathBuf {
        self.0.join(p)
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

    /// The folder where `rythme install` keeps the packages of this test.
    fn home(&self) -> PathBuf {
        self.path("home")
    }

    /// Runs `rythme` in the folder `dir`, with its own `RYTHME_HOME`.
    fn rythme(&self, lang: &str, dir: &str, args: &[&str]) -> Run {
        let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .args(args)
            .current_dir(self.path(dir))
            .env("RYTHME_LANG", lang)
            .env("NO_COLOR", "1")
            .env("RYTHME_HOME", self.home())
            .output()
            .expect("run rythme");
        Run {
            stdout: String::from_utf8_lossy(&out.stdout).replace("\r\n", "\n"),
            stderr: String::from_utf8_lossy(&out.stderr).replace("\r\n", "\n"),
            code: out.status.code().unwrap_or(-1),
        }
    }

    /// A git repository in `dir` with these files, committed.
    fn repo(&self, dir: &str, files: &[(&str, &str)]) -> String {
        std::fs::create_dir_all(self.path(dir)).unwrap();
        git(&self.path(dir), &["init", "--quiet"]);
        self.commit(dir, files)
    }

    /// Writes these files in the repository `dir` and commits them; gives the commit.
    fn commit(&self, dir: &str, files: &[(&str, &str)]) -> String {
        let files: Vec<(String, &str)> = files.iter().map(|(p, t)| (format!("{dir}/{p}"), *t)).collect();
        let files: Vec<(&str, &str)> = files.iter().map(|(p, t)| (p.as_str(), *t)).collect();
        self.write(&files);
        let repo = self.path(dir);
        git(&repo, &["add", "--all"]);
        git(&repo, &["commit", "--quiet", "--message", "work"]);
        git(&repo, &["rev-parse", "HEAD"])
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        if std::fs::remove_dir_all(&self.0).is_err() {
            // git marks some of its files read-only on Windows.
            make_writable(&self.0);
            let _ = std::fs::remove_dir_all(&self.0);
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

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(["-c", "user.name=Test", "-c", "user.email=test@example.com", "-c", "init.defaultBranch=main", "-c", "commit.gpgsign=false"])
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git is needed by these tests");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A path written in `rythme.toml`, with `/` on every system.
fn toml_path(p: &Path) -> String {
    p.display().to_string().replace('\\', "/")
}

#[track_caller]
fn ok(r: &Run) {
    assert_eq!(r.code, 0, "stdout:\n{}\nstderr:\n{}", r.stdout, r.stderr);
}

#[test]
fn a_new_project_runs_tests_checks_formats_and_documents_itself() {
    for lang in ["en", "fr"] {
        let t = Temp::new(&format!("new-{lang}"));
        let r = t.rythme(lang, "", &["new", "hello"]);
        ok(&r);
        let (lib, test) =
            if lang == "fr" { ("src/salutation.ry", "tests/salutation_test.ry") } else { ("src/greeting.ry", "tests/greeting_test.ry") };
        for f in ["rythme.toml", "src/main.ry", lib, test, "README.md", ".gitignore"] {
            assert!(t.path("hello").join(f).is_file(), "{lang}: {f} is missing");
            assert!(r.stdout.contains(&format!("hello/{f}")), "{lang}: {}", r.stdout);
        }
        assert!(t.read("hello/rythme.toml").contains("name = \"hello\"\nversion = \"0.1.0\"\nentry = \"src/main.ry\""));
        let r = t.rythme(lang, "hello", &["run"]);
        ok(&r);
        assert_eq!(r.stdout, if lang == "fr" { "Bonjour, le monde !\n" } else { "Hello, World!\n" });
        let r = t.rythme(lang, "hello", &["test"]);
        ok(&r);
        assert!(r.stdout.contains(if lang == "fr" { "1 réussi(s), 0 échoué(s)" } else { "1 passed, 0 failed" }), "{}", r.stdout);
        let r = t.rythme(lang, "hello", &["check"]);
        ok(&r);
        assert_eq!(r.stdout.lines().count(), 3, "{}", r.stdout);
        assert!(r.stderr.is_empty(), "no warning in a new project: {}", r.stderr);
        ok(&t.rythme(lang, "hello", &["fmt", "--check"]));
        let r = t.rythme(lang, "hello", &["doc"]);
        ok(&r);
        assert!(r.stdout.contains(&format!("## {lib}")), "{}", r.stdout);
        // From a folder of the project, too.
        let r = t.rythme(lang, "hello/tests", &["run"]);
        ok(&r);
        let r = t.rythme(lang, "hello/tests", &["test"]);
        ok(&r);
        // No dependencies yet.
        let r = t.rythme(lang, "hello", &["install"]);
        ok(&r);
        assert!(!t.path("hello/rythme.lock").exists());
    }
}

#[test]
fn new_refuses_bad_names_and_existing_folders() {
    let t = Temp::new("new-bad");
    let r = t.rythme("en", "", &["new", "My-App"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("error[E025]") && r.stderr.contains("try `rythme new my_app`"), "{}", r.stderr);
    let r = t.rythme("fr", "", &["new"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("quel nom ?"), "{}", r.stderr);
    ok(&t.rythme("en", "", &["new", "app"]));
    let before = t.read("app/src/main.ry");
    t.write(&[("app/src/main.ry", "print(1)\n")]);
    let r = t.rythme("en", "", &["new", "app"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("`app` already exists"), "{}", r.stderr);
    assert_eq!(t.read("app/src/main.ry"), "print(1)\n", "never replaces a file");
    assert_ne!(before, "print(1)\n");
}

const MATHLIB_TOML: &str = "name = \"mathlib\"\nversion = \"1.0.0\"\nentry = \"src/mathlib.ry\"\n";

#[test]
fn dependencies_from_git_are_installed_locked_and_used() {
    let t = Temp::new("git");
    let first = t.repo(
        "mathlib",
        &[
            ("rythme.toml", MATHLIB_TOML),
            ("src/mathlib.ry", "## The version.\nversion = \"one\"\n"),
            ("src/vectors.ry", "## Adds two vectors.\nfn add(a, b) = {x: a.x + b.x, y: a.y + b.y}\nfn _hidden(x) = x\n"),
        ],
    );
    git(&t.path("mathlib"), &["tag", "v1.0.0"]);
    let url = toml_path(&t.path("mathlib"));
    t.write(&[
        (
            "app/rythme.toml",
            &format!("name = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nmathlib = {{ git = \"{url}\", tag = \"v1.0.0\" }}\n"),
        ),
        ("app/src/main.ry", "use mathlib/vectors\nuse mathlib\nprint(vectors.add({x: 1, y: 2}, {x: 3, y: 4}), mathlib.version)\n"),
    ]);
    // Not installed yet.
    let r = t.rythme("en", "app", &["run"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E025]: the package `mathlib` is not installed"), "{}", r.stderr);
    assert!(r.stderr.contains("src/main.ry:1:5") && r.stderr.contains("run `rythme install`"), "{}", r.stderr);
    let r = t.rythme("fr", "app", &["run"]);
    assert!(r.stderr.contains("le paquet `mathlib` n'est pas installé"), "{}", r.stderr);

    let r = t.rythme("en", "app", &["install"]);
    ok(&r);
    let short = &first[..7];
    assert!(r.stdout.contains(&format!("✓ mathlib 1.0.0: tag v1.0.0, commit {short} (downloaded)")), "{}", r.stdout);
    let lock = t.read("app/rythme.lock");
    assert!(
        lock.contains(&format!("[[package]]\nname = \"mathlib\"\ngit = \"{url}\"\ntag = \"v1.0.0\"\ncommit = \"{first}\"\n")),
        "{lock}"
    );
    let r = t.rythme("en", "app", &["run"]);
    ok(&r);
    assert_eq!(r.stdout, "{x: 4, y: 6} one\n");
    let r = t.rythme("en", "app", &["check"]);
    ok(&r);
    // A second install finds it there.
    let r = t.rythme("fr", "app", &["install"]);
    ok(&r);
    assert!(r.stdout.contains("(déjà là)"), "{}", r.stdout);
    assert_eq!(t.read("app/rythme.lock"), lock);

    // A private name stays private, and a missing file gets a suggestion.
    t.write(&[("app/src/main.ry", "use mathlib/vectors\nprint(vectors._hidden(1))\n")]);
    let r = t.rythme("en", "app", &["run"]);
    assert!(r.stderr.contains("error[E003]") && r.stderr.contains("private"), "{}", r.stderr);
    t.write(&[("app/src/main.ry", "use mathlib/vector\nprint(vector.add)\n")]);
    let r = t.rythme("en", "app", &["run"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E017]: the package `mathlib` has no file `vector`"), "{}", r.stderr);
    assert!(r.stderr.contains("did you mean `use mathlib/vectors`?"), "{}", r.stderr);

    // The lock keeps the commit, even when the tag moves.
    let second = t.commit("mathlib", &[("src/mathlib.ry", "version = \"two\"\n")]);
    git(&t.path("mathlib"), &["tag", "--force", "v1.0.0"]);
    let _ = std::fs::remove_dir_all(t.home());
    let r = t.rythme("en", "app", &["install"]);
    ok(&r);
    assert!(r.stdout.contains(short), "{}", r.stdout);
    t.write(&[("app/src/main.ry", "use mathlib\nprint(mathlib.version)\n")]);
    assert_eq!(t.rythme("en", "app", &["run"]).stdout, "one\n");
    // Without the lock, the tag gives the new commit.
    std::fs::remove_file(t.path("app/rythme.lock")).unwrap();
    let _ = std::fs::remove_dir_all(t.home());
    ok(&t.rythme("en", "app", &["install"]));
    assert!(t.read("app/rythme.lock").contains(&second));
    assert_eq!(t.rythme("en", "app", &["run"]).stdout, "two\n");
}

#[test]
fn dependencies_of_dependencies_by_commit_and_from_folders() {
    let t = Temp::new("deep");
    let tools = t.repo(
        "tools",
        &[("rythme.toml", "name = \"tools\"\nversion = \"0.2.0\"\n"), ("src/text/shout.ry", "fn shout(s) = upper(s) + \"!\"\n")],
    );
    let tools_url = toml_path(&t.path("tools"));
    t.repo(
        "mathlib",
        &[
            (
                "rythme.toml",
                &format!("{MATHLIB_TOML}\n[dependencies]\ntools = {{ git = \"{tools_url}\", commit = \"{}\" }}\n", &tools[..10]),
            ),
            ("src/mathlib.ry", "use tools/text/shout\nuse mathlib/inner\nfn loud(n) = shout.shout(text(inner.twice(n)))\n"),
            ("src/inner.ry", "fn twice(n) = 2 * n\n"),
        ],
    );
    git(&t.path("mathlib"), &["tag", "v1.0.0"]);
    let url = toml_path(&t.path("mathlib"));
    t.write(&[
        ("local/rythme.toml", "name = \"local\"\nversion = \"0.1.0\"\n"),
        ("local/src/hello.ry", "fn hi() = \"hi\"\n"),
        (
            "app/rythme.toml",
            &format!(
                "# My app\nname = \"app\"   # its name\nversion = \"0.1.0\"\n\n[dependencies]\nmathlib = {{ git = \"{url}\", tag = \"v1.0.0\" }}\nlocal = {{ path = \"../local\" }}\n"
            ),
        ),
        ("app/src/main.ry", "use mathlib\nuse local/hello\nprint(mathlib.loud(21), hello.hi())\n"),
    ]);
    let r = t.rythme("en", "app", &["install"]);
    ok(&r);
    assert!(r.stdout.contains("✓ mathlib 1.0.0: tag v1.0.0"), "{}", r.stdout);
    assert!(r.stdout.contains(&format!("✓ tools 0.2.0: commit {}, commit {}", &tools[..10], &tools[..7])), "{}", r.stdout);
    assert!(r.stdout.contains("✓ local 0.1.0: folder ../local"), "{}", r.stdout);
    assert!(r.stdout.contains("3 package(s) ready (1 needed by the other ones)"), "{}", r.stdout);
    let lock = t.read("app/rythme.lock");
    assert!(
        lock.contains(&format!("name = \"tools\"\ngit = \"{tools_url}\"\nasked = \"{}\"\ncommit = \"{tools}\"", &tools[..10])),
        "{lock}"
    );
    assert!(!lock.contains("local"), "{lock}");
    let r = t.rythme("en", "app", &["run"]);
    ok(&r);
    assert_eq!(r.stdout, "42! hi\n");
}

#[test]
fn install_explains_what_goes_wrong() {
    let t = Temp::new("errors");
    t.repo("lib", &[("rythme.toml", "name = \"lib\"\nversion = \"1.0.0\"\n"), ("src/a.ry", "x = 1\n")]);
    git(&t.path("lib"), &["tag", "v1.0.0"]);
    git(&t.path("lib"), &["tag", "v1.1.0"]);
    let url = toml_path(&t.path("lib"));
    let project = |deps: &str| t.write(&[("app/rythme.toml", &format!("name = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\n{deps}\n"))]);

    project(&format!("lib = {{ git = \"{url}\", tag = \"v1.0.1\" }}"));
    let r = t.rythme("en", "app", &["install"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E025]: the tag `v1.0.1` of `lib` does not exist in"), "{}", r.stderr);
    assert!(r.stderr.contains("--> rythme.toml:5:1") && r.stderr.contains("did you mean `tag = \"v1.0.0\"`?"), "{}", r.stderr);

    project(&format!("lib = {{ git = \"{url}/nothing\", tag = \"v1.0.0\" }}"));
    let r = t.rythme("fr", "app", &["install"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("impossible de télécharger `lib`") && r.stderr.contains("git dit :"), "{}", r.stderr);

    // Without git.
    project(&format!("lib = {{ git = \"{url}\", tag = \"v1.0.0\" }}"));
    let empty = t.path("no-programs");
    std::fs::create_dir_all(&empty).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .arg("install")
        .current_dir(t.path("app"))
        .env("PATH", &empty)
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .env("RYTHME_HOME", t.home())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr.contains("error[E025]: the program `git` was not found") && stderr.contains("https://git-scm.com"), "{stderr}");

    // A repository that is not a package.
    t.repo("plain", &[("a.ry", "x = 1\n")]);
    git(&t.path("plain"), &["tag", "v1"]);
    project(&format!("plain = {{ git = \"{}\", tag = \"v1\" }}", toml_path(&t.path("plain"))));
    let r = t.rythme("en", "app", &["install"]);
    assert!(r.stderr.contains("`plain` is not a Rythme package"), "{}", r.stderr);

    // A folder that does not exist.
    project("near = { path = \"../near\" }");
    let r = t.rythme("en", "app", &["install"]);
    assert!(r.stderr.contains("the folder `../near` of the package `near` does not exist"), "{}", r.stderr);

    // Outside of a project.
    let r = t.rythme("en", "", &["install"]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("no `rythme.toml` in this folder or above it") && r.stderr.contains("rythme new NAME"), "{}", r.stderr);
    let r = t.rythme("en", "app", &["install", "lib"]);
    assert_eq!(r.code, 2);
}

#[test]
fn mistakes_in_rythme_toml_are_shown_where_they_are() {
    let t = Temp::new("toml");
    let cases: &[(&str, &[&str])] = &[
        ("nmae = \"app\"\nversion = \"0.1.0\"\n", &["unknown key `nmae`", "rythme.toml:1:1", "did you mean `name`?"]),
        ("name = app\nversion = \"0.1.0\"\n", &["a value is a text, written in quotes", "write `\"app\"`"]),
        ("name = \"app\nversion = \"0.1.0\"\n", &["this text is not closed", "rythme.toml:1:8"]),
        ("[package]\nname = \"app\"\nversion = \"0.1.0\"\n", &["unknown section `package`", "at the top of the file, without a section"]),
        ("name = \"app\"\nversion = \"1.0\"\n", &["`1.0` is not a version", "three numbers"]),
        ("name = \"My App\"\nversion = \"0.1.0\"\n", &["cannot be the name of a package", "write `name = \"my_app\"`"]),
        ("version = \"0.1.0\"\n", &["has no `name`"]),
        ("name = \"app\"\nname = \"other\"\nversion = \"0.1.0\"\n", &["`name` is written twice", "first written here"]),
        ("name = \"app\"\nversion = \"0.1.0\"\n[dependencies]\nlib = { git = \"x\" }\n", &["needs a fixed version: a `tag` or a `commit`"]),
        ("name = \"app\"\nversion = \"0.1.0\"\n[dependencies]\nlib = \"1.0\"\n", &["is written as a table", "lib = { git = "]),
        (
            "name = \"app\"\nversion = \"0.1.0\"\n[dependencies]\nlib = { path = \"../lib\", tag = \"v1\" }\n",
            &["`tag` and `commit` are for a dependency from git"],
        ),
        ("name = \"app\"\nversion = \"0.1.0\"\n[dependencies]\napp = { path = \".\" }\n", &["cannot depend on itself"]),
        (
            "name = \"app\"\nversion = \"0.1.0\"\n[dependencies]\nlib = { gti = \"x\", tag = \"v1\" }\n",
            &["unknown key `gti`", "did you mean `git`?"],
        ),
    ];
    for (toml, wanted) in cases {
        t.write(&[("app/rythme.toml", toml), ("app/src/main.ry", "print(1)\n")]);
        let r = t.rythme("en", "app", &["run"]);
        assert_eq!(r.code, 1, "{toml}\n{}", r.stderr);
        assert!(r.stderr.contains("error[E025]"), "{toml}\n{}", r.stderr);
        for w in *wanted {
            assert!(r.stderr.contains(w), "{toml}: expected `{w}` in\n{}", r.stderr);
        }
    }
    // In French too.
    t.write(&[("app/rythme.toml", "nom = \"app\"\n")]);
    let r = t.rythme("fr", "app", &["run"]);
    assert!(r.stderr.contains("erreur[E025]: clé inconnue `nom`"), "{}", r.stderr);
    // A file given by name does not need the project.
    let r = t.rythme("en", "app", &["run", "src/main.ry"]);
    ok(&r);
    // A broken lock file.
    t.write(&[("app/rythme.toml", "name = \"app\"\nversion = \"0.1.0\"\n"), ("app/rythme.lock", "[[package]]\nname = \"x\"\n")]);
    let r = t.rythme("en", "app", &["install"]);
    assert!(r.stderr.contains("this entry of `rythme.lock` is not complete") && r.stderr.contains("delete it"), "{}", r.stderr);
}

#[test]
fn test_at_the_root_runs_every_file_with_tests() {
    let t = Temp::new("tests");
    t.write(&[
        ("p/rythme.toml", "name = \"p\"\nversion = \"0.1.0\"\n"),
        ("p/src/main.ry", "use p/maths\nprint(maths.double(2))\n"),
        ("p/src/maths.ry", "fn double(x) = 2 * x\ntest \"inside\" = assert_eq(double(1), 2)\n"),
        ("p/tests/a_test.ry", "use p/maths\ntest \"a\" = assert_eq(maths.double(2), 4)\n"),
        ("p/tests/more/b_test.ry", "use p/maths\ntest \"b\" = assert_eq(maths.double(3), 5)\n"),
        ("p/tests/helper.ry", "fn nothing() = 0\n"),
    ]);
    let r = t.rythme("en", "p", &["test"]);
    assert_eq!(r.code, 1, "{}", r.stdout);
    let headers: Vec<&str> = r.stdout.lines().filter(|l| l.starts_with("── ")).collect();
    assert_eq!(headers, ["── src/maths.ry ──", "── tests/a_test.ry ──", "── tests/more/b_test.ry ──"], "{}", r.stdout);
    assert!(r.stderr.contains("error[E015]"), "{}", r.stderr);
    // `rythme check` checks them all, the program first.
    let r = t.rythme("en", "p", &["check"]);
    ok(&r);
    let first: Vec<&str> = r.stdout.lines().map(|l| l.split(':').next().unwrap()).collect();
    assert_eq!(first, ["✓ src/main.ry", "✓ src/maths.ry", "✓ tests/a_test.ry", "✓ tests/helper.ry", "✓ tests/more/b_test.ry"]);
    // Outside of a project, a file is still needed.
    let r = t.rythme("en", "", &["test"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("which file?"), "{}", r.stderr);
}

#[test]
fn rythme_toml_is_found_by_the_language_server() {
    // `use package/file` through the language server: no problem reported,
    // and completion offers the packages.
    let t = Temp::new("lsp");
    t.write(&[
        ("p/rythme.toml", "name = \"p\"\nversion = \"0.1.0\"\n\n[dependencies]\nlocal = { path = \"../local\" }\n"),
        ("local/rythme.toml", "name = \"local\"\nversion = \"0.1.0\"\n"),
        ("local/src/hello.ry", "fn hi() = \"hi\"\n"),
        ("p/src/main.ry", "use local/hello\nprint(hello.hi())\n"),
    ]);
    let main = t.path("p/src/main.ry");
    let uri = rythme::lsp::text::path_to_uri(&main);
    let text = "use local/hello\nprint(hello.hi())\nuse \n";
    let messages = [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}"#.to_string(),
        r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#.to_string(),
        format!(
            r#"{{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{{"textDocument":{{"uri":{},"languageId":"rythme","version":1,"text":{}}}}}}}"#,
            rythme::lsp::json::Json::Str(uri.clone()),
            rythme::lsp::json::Json::Str(text.to_string())
        ),
        format!(
            r#"{{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{{"textDocument":{{"uri":{}}},"position":{{"line":2,"character":4}}}}}}"#,
            rythme::lsp::json::Json::Str(uri.clone())
        ),
        r#"{"jsonrpc":"2.0","id":3,"method":"shutdown"}"#.to_string(),
        r#"{"jsonrpc":"2.0","method":"exit"}"#.to_string(),
    ];
    let mut input = String::new();
    for m in &messages {
        input.push_str(&format!("Content-Length: {}\r\n\r\n{m}", m.len()));
    }
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .arg("lsp")
        .env("RYTHME_LANG", "en")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    let out = String::from_utf8_lossy(&out.stdout);
    let diags: Vec<&str> = out.split("Content-Length").filter(|m| m.contains("publishDiagnostics")).collect();
    assert!(!diags.is_empty(), "{out}");
    let last = diags.last().unwrap();
    assert!(!last.contains("E017") && !last.contains("E025"), "{last}");
    let completion = out.split("Content-Length").find(|m| m.contains("\"id\":2")).unwrap_or("");
    assert!(completion.contains("\"label\":\"local/\"") && completion.contains("\"label\":\"p/\""), "{completion}");
}
