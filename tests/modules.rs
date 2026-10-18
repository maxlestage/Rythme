//! Files used with a name: `use geometry`, then `geometry.area(2)`.

mod common;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use common::{rythme, rythme_lang};

/// A folder with these files, removed at the end of the test.
struct Folder(PathBuf);

impl Folder {
    fn new(name: &str, files: &[(&str, &str)]) -> Folder {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("rythme-modules-{name}-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for (path, text) in files {
            let path = dir.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        Folder(dir)
    }

    fn path(&self, name: &str) -> String {
        self.0.join(name).to_str().unwrap().to_string()
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

const GEOMETRY: &str = "\
## Area of a square.
fn area(side) = _square(side)
fn _square(x) = x * x
type Shape = Circle(radius) | Square(side) | Dot
fn describe(s) = match s
  Circle(r) -> \"circle {r}\"
  Square(c) -> \"square {c}\"
  Dot -> \"dot\"
end
unit = 1
print(\"geometry runs\")
test \"area\" = assert_eq(area(3), 9)
";

fn folder(name: &str, main: &str) -> Folder {
    Folder::new(
        name,
        &[("geometry.ry", GEOMETRY), ("shapes/circle.ry", "fn perimeter(r) = 2 * r\nfn area(r) = 3 * r * r\n"), ("main.ry", main)],
    )
}

#[track_caller]
fn runs(main: &str, expected: &str) {
    let f = folder("run", main);
    let r = rythme(&[&f.path("main.ry")]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(r.stdout, expected);
    let r = rythme(&["check", &f.path("main.ry")]);
    assert_eq!(r.code, 0, "{}{}", r.stdout, r.stderr);
}

#[track_caller]
fn refused(main: &str, expected: &[&str]) -> String {
    let f = folder("refused", main);
    let r = rythme(&[&f.path("main.ry")]);
    assert_eq!(r.code, 1, "{}{}", r.stdout, r.stderr);
    for e in expected {
        assert!(r.stderr.contains(e), "`{e}` missing in:\n{}", r.stderr);
    }
    assert!(!r.stdout.contains("geometry runs"), "nothing runs: {}", r.stdout);
    r.stderr
}

#[test]
fn definitions_through_the_name_of_the_file() {
    runs(
        "use geometry\nprint(geometry.area(3), geometry.unit, geometry.Dot)\nprint(geometry.describe(geometry.Circle(2)))\n",
        "geometry runs\n9 1 Dot\ncircle 2\n",
    );
    // A file in a folder; another name; pipes; patterns; constructors as functions.
    runs(
        "use shapes/circle\nuse geometry as geo\nprint(circle.perimeter(1), 3 |> geo.area, [1, 2] |> map(geo.area), map([1], geo.Square))\n",
        "geometry runs\n2 9 [1, 4] [Square(1)]\n",
    );
    runs(
        "use geometry\nfn size(s) = match s\n  geometry.Circle(r) -> r\n  geometry.Square(c) -> c * 10\n  geometry.Dot -> 0\nend\nprint(size(geometry.Square(4)))\n",
        "geometry runs\n40\n",
    );
    // The old form with a name, and the same file twice: it runs once.
    runs("use \"geometry.ry\" as geo\nuse geometry\nprint(geo.area(2), geometry.area(2))\n", "geometry runs\n4 4\n");
}

#[test]
fn each_file_keeps_its_own_names() {
    // `area` of two files and of the program, and private names that do not meet.
    runs(
        "use geometry\nuse shapes/circle\nfn area(x) = \"mine\"\nfn _square(x) = \"my square\"\nprint(geometry.area(2), circle.area(2), area(2), _square(2))\n",
        "geometry runs\n4 12 mine my square\n",
    );
    // A file used with a name sees its own definitions, not those of the program.
    let f = Folder::new(
        "own",
        &[
            ("lib.ry", "fn twice(x) = helper(x) * 2\nfn helper(x) = x + 1\n"),
            ("main.ry", "use lib\nfn helper(x) = 100\nprint(lib.twice(1), helper(1))\n"),
        ],
    );
    let r = rythme(&[&f.path("main.ry")]);
    assert_eq!(r.stdout, "4 100\n", "{}", r.stderr);
    // Private names of a file used with quotes stay private too.
    let f = Folder::new(
        "quoted",
        &[
            ("lib.ry", "fn twice(x) = _helper(x) * 2\nfn _helper(x) = x + 1\n"),
            ("main.ry", "use \"lib.ry\"\nfn _helper(x) = 100\nprint(twice(1), _helper(1))\n"),
        ],
    );
    let r = rythme(&[&f.path("main.ry")]);
    assert_eq!(r.stdout, "4 100\n", "{}", r.stderr);
    let f = Folder::new("quoted-private", &[("lib.ry", "fn _helper(x) = x + 1\n"), ("main.ry", "use \"lib.ry\"\nprint(_helper(1))\n")]);
    let r = rythme(&[&f.path("main.ry")]);
    assert!(r.stderr.contains("error[E003]: `_helper` is private to the file"), "{}", r.stderr);
}

#[test]
fn mistakes_through_a_name_are_found_before_running() {
    refused("use geometry\nprint(geometry.aera(2))\n", &["error[E003]: the file", "has no `aera`", "did you mean `geometry.area`?"]);
    refused("use geometry\nprint(geometry.Circel(2))\n", &["has no `Circel`", "did you mean `geometry.Circle`?"]);
    refused(
        "use geometry\nprint(geometry._square(2))\n",
        &["error[E003]: `_square` is private to the file", "a name starting with `_` can only be used in its own file"],
    );
    refused(
        "use geometry\nprint(geometry.area(1, 2))\n",
        &["error[E004]: `geometry.area` takes 1 argument, but 2 were given", "usage: `geometry.area(side)`"],
    );
    refused("use geometry\nprint(geometry)\n", &["error[E003]: `geometry` is the name of the file", "not a value", "like `geometry.area`"]);
    refused("use geometry\ngeometry = 2\n", &["error[E011]: `geometry` is already the name of the file"]);
    refused("use geometry\nfn f(geometry) = geometry.area\n", &["error[E011]: `geometry` is already the name of the file"]);
    refused("use geometry\nprint(match 1\n  geometry.Circel(r) -> r\n  _ -> 0\nend)\n", &["has no `Circel`"]);
    // Every case of a type of the file, in a `match`.
    let f = folder("cases", "use geometry\nprint(match geometry.Dot\n  geometry.Circle(r) -> r\n  geometry.Dot -> 0\nend)\n");
    let r = rythme(&["check", &f.path("main.ry")]);
    assert!(r.stderr.contains("warning[W002]") && r.stderr.contains("`geometry.Square(_)`"), "{}", r.stderr);
}

#[test]
fn unknown_names_say_which_file_defines_them() {
    refused("use geometry\nprint(area(2))\n", &["unknown name `area`", "geometry.ry`: write `geometry.area`"]);
    // Not used yet: the file of the folder that defines it, and the `use` to add.
    refused(
        "print(describe(1))\n",
        &[
            "unknown name `describe`",
            "`describe` is defined in `geometry.ry`: add `use geometry` at the top of the file, then write `geometry.describe`",
        ],
    );
    let f = folder("fr", "print(describe(1))\n");
    let r = rythme_lang("fr", &[&f.path("main.ry")]);
    assert!(r.stderr.contains("`describe` est défini dans `geometry.ry` : ajoutez `use geometry` en haut du fichier"), "{}", r.stderr);
}

#[test]
fn missing_files_name_files_and_circles() {
    let e = refused("use geometri\n", &["error[E017]: file `geometri.ry` not found", "did you mean `use geometry`?"]);
    assert!(!e.contains('"'), "the suggestion keeps the form written: {e}");
    refused("use shapes/circel\n", &["did you mean `use shapes/circle`?"]);
    refused("use \"geometri.ry\"\n", &["did you mean `use \"geometry.ry\"`?"]);
    refused("use zzzzzzzzzz\n", &["the Rythme files there are: `geometry.ry`, `main.ry`"]);
    refused("use geometry.ry\n", &["write the name of the file without `.ry`", "example: `use geometry`"]);
    refused("use Geometry\n", &["for a file whose name starts with an uppercase letter"]);
    let f = Folder::new("circle", &[("a.ry", "use b\n"), ("b.ry", "use c\n"), ("c.ry", "use a\n"), ("d.ry", "use d\n")]);
    let r = rythme(&[&f.path("a.ry")]);
    assert!(
        r.stderr.contains("error[E017]: `") && r.stderr.contains("c.ry` and `") && r.stderr.contains("a.ry` use each other"),
        "{}",
        r.stderr
    );
    assert!(r.stderr.contains("the circle:"), "{}", r.stderr);
    let r = rythme_lang("fr", &[&f.path("d.ry")]);
    assert!(r.stderr.contains("erreur[E017]: `d.ry` s'utilise lui-même"), "{}", r.stderr);
}

#[test]
fn two_files_for_one_name() {
    let f = Folder::new(
        "twice",
        &[
            ("a.ry", "fn area(x) = 1\n"),
            ("b.ry", "fn area(x) = 2\n"),
            ("quoted.ry", "use \"a.ry\"\nuse \"b.ry\"\nprint(area(1))\n"),
            ("named.ry", "use a as lib\nuse b as lib\n"),
            ("both.ry", "use a\nuse b\nprint(a.area(1), b.area(1))\n"),
        ],
    );
    let r = rythme(&[&f.path("quoted.ry")]);
    assert!(r.stderr.contains("error[E011]: `area` is defined by two used files:"), "{}", r.stderr);
    assert!(
        r.stderr.contains("a.ry") && r.stderr.contains("b.ry") && r.stderr.contains("`use geometry`, then `geometry.area`"),
        "{}",
        r.stderr
    );
    let r = rythme(&[&f.path("named.ry")]);
    assert!(r.stderr.contains("error[E011]: the name `lib` is given to two files:"), "{}", r.stderr);
    let r = rythme(&[&f.path("both.ry")]);
    assert_eq!(r.stdout, "1 2\n", "{}", r.stderr);
}

#[test]
fn tests_of_used_files_and_the_repl() {
    let f = folder("tests", "use geometry\ntest \"through the name\" = assert_eq(geometry.area(2), 4)\n");
    let r = rythme(&["test", &f.path("main.ry")]);
    assert!(r.stdout.contains("2 passed, 0 failed"), "{}{}", r.stdout, r.stderr);
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["repl"])
        .current_dir(&f.0)
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .take()
                .unwrap()
                .write_all(b"use geometry\ngeometry.area(5)\nuse shapes/circle as c\nc.perimeter(2) + geometry.unit\n")?;
            child.wait_with_output()
        })
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("25") && stdout.contains('5'), "{stdout}{}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn the_formatter_writes_the_recommended_form() {
    let f = Folder::new(
        "fmt",
        &[(
            "main.ry",
            "use   \"geometry.ry\"   as geo  # comment\nuse \"shapes/circle.ry\" as circle\nuse shapes / circle as c\nuse geometry as geometry\nuse \"geometry.ry\"\nuse \"My File.ry\" as mine\nprint(match geo.Dot\n  geo.Circle( r ) -> r\n  _ -> 1\nend)\n",
        )],
    );
    let r = rythme(&["fmt", &f.path("main.ry")]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    let text = std::fs::read_to_string(Path::new(&f.path("main.ry"))).unwrap();
    assert_eq!(
        text,
        "use geometry as geo  # comment\nuse shapes/circle\nuse shapes/circle as c\nuse geometry\nuse \"geometry.ry\"\nuse \"My File.ry\" as mine\n\nprint(match geo.Dot\n  geo.Circle(r) -> r\n  _ -> 1\nend)\n"
    );
}

#[test]
fn help_explains_use() {
    let r = rythme(&["help", "use"]);
    assert!(r.stdout.contains("use geometry") && r.stdout.contains("geometry.area"), "{}", r.stdout);
    let r = rythme(&["help", "modules"]);
    assert!(r.stdout.contains("use shapes/circle") && r.stdout.contains("private"), "{}", r.stdout);
    let r = rythme_lang("fr", &["help", "modules"]);
    assert!(r.stdout.contains("use geometrie") && r.stdout.contains("privés"), "{}", r.stdout);
}
