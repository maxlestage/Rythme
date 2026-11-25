//! `rythme doc`: the documentation of files and folders, in Markdown and HTML.

mod common;

use std::path::PathBuf;

use common::{rythme, rythme_lang};

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-doc-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

const GEOMETRY: &str = r#"## Shapes and their areas.

## A shape to draw.
type Shape = Circle(radius) | Rect(width, height)

## The area of a shape.
##
## - a `Circle` uses `pi`
## - see `len(list)` to count shapes
fn area(Circle(r)) = pi * r * r
fn area(Rect(w, h)) = w * h

## How many sides a square has.
sides = 4

fn _helper(x) = x
_secret = 1

## Doubles a number.
double = fn(x) = x * 2

test "area of a square" = area(Rect(2, 2)) == 4
"#;

const MAIN: &str = r#"use "geometry.ry"

## Sum of the `area` of every shape, with `total_of` <html>.
fn total(shapes) if len(shapes) > 0 = shapes |> map(area) |> sum
fn total(_) = 0

print(total([Circle(1)]))
"#;

fn project(name: &str) -> PathBuf {
    let dir = temp_dir(name);
    std::fs::write(dir.join("geometry.ry"), GEOMETRY).unwrap();
    std::fs::write(dir.join("main.ry"), MAIN).unwrap();
    dir
}

#[test]
fn a_file_is_documented_in_markdown_with_the_files_it_uses() {
    let dir = project("markdown");
    let main = dir.join("main.ry");
    let r = rythme(&["doc", main.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    let md = &r.stdout;
    // The file given first, then the file it uses, each once.
    let first = md.find("main.ry\n").unwrap();
    let second = md.find("geometry.ry\n\nShapes and their areas.").expect(md);
    assert!(first < second, "{md}");
    assert!(md.contains("## Contents"), "{md}");
    // Every definition of a function, with its guard.
    assert!(md.contains("~~~rythme\nfn total(shapes) if len(shapes) > 0\nfn total(_)\n~~~"), "{md}");
    assert!(md.contains("~~~rythme\nfn area(Circle(r))\nfn area(Rect(w, h))\n~~~"), "{md}");
    assert!(md.contains("~~~rythme\ntype Shape = Circle(radius) | Rect(width, height)\n~~~\n\nA shape to draw."), "{md}");
    assert!(md.contains("~~~rythme\nsides = 4\n~~~\n\nHow many sides a square has."), "{md}");
    assert!(md.contains("~~~rythme\ndouble = fn(x)\n~~~\n\nDoubles a number."), "{md}");
    assert!(md.contains("### Tests\n\n- area of a square"), "{md}");
    // Private names stay out.
    assert!(!md.contains("_helper") && !md.contains("_secret"), "{md}");
    // Links: to a definition, to the type of a constructor, to a built-in explained at the end.
    assert!(md.contains("[`area`](#area)"), "{md}");
    assert!(md.contains("[`Circle`](#shape)"), "{md}");
    assert!(md.contains("[`len(list)`](#len)"), "{md}");
    assert!(md.contains("`total_of`") && !md.contains("[`total_of`]"), "an unknown name is not a link:\n{md}");
    assert!(md.contains("## Built-in functions mentioned\n\n### len\n\n~~~rythme\nlen(x)\n~~~"), "{md}");
    assert!(md.contains("Uses: [`"), "{md}");
    // The program is not run.
    assert!(!md.contains("3.14"), "{md}");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_folder_gives_one_html_page() {
    let dir = project("html");
    let out = dir.join("doc.html");
    let r = rythme(&["doc", dir.to_str().unwrap(), "--out", out.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.starts_with("written: "), "{}", r.stdout);
    let html = std::fs::read_to_string(&out).unwrap();
    assert!(html.starts_with("<!doctype html>\n<html lang=\"en\">"), "{html}");
    // One page that needs nothing else: no script, no external file.
    assert!(!html.contains("<script") && !html.contains("<link") && !html.contains("http"), "{html}");
    assert!(html.contains("prefers-color-scheme: dark"));
    assert_eq!(html.matches("<section class=\"file\"").count(), 3, "two files and the built-ins:\n{html}");
    assert!(html.contains("<span class=\"kw\">fn</span> <span class=\"fn\">area</span>(<span class=\"ctor\">Circle</span>(r))"), "{html}");
    assert!(html.contains("<a href=\"#len\"><code>len(list)</code></a>"), "{html}");
    assert!(html.contains("id=\"len\""), "{html}");
    assert!(html.contains("<ul><li>a <a href=\"#shape\"><code>Circle</code></a> uses <code>pi</code></li>"), "{html}");
    assert!(html.contains("&lt;html&gt;") && !html.contains("<html>."), "comments are escaped:\n{html}");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn labels_follow_the_language_and_doc_has_a_french_name() {
    let dir = project("french");
    let r = rythme_lang("fr", &["documenter", "--html", dir.join("main.ry").to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.contains("<html lang=\"fr\">"), "{}", r.stdout);
    assert!(r.stdout.contains("Fonctions intégrées citées") && r.stdout.contains("Utilise : "), "{}", r.stdout);
    assert!(r.stdout.contains("Longueur d'un texte"), "the built-ins are explained in French:\n{}", r.stdout);
    let r = rythme_lang("fr", &["doc", dir.join("geometry").to_str().unwrap()]);
    assert_eq!(r.code, 0, "`geometry` stands for `geometry.ry`: {}", r.stderr);
    assert!(r.stdout.contains("## Fonctions") && r.stdout.contains("## Valeurs"), "{}", r.stdout);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn doc_reports_mistakes() {
    let r = rythme(&["doc"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("which files?"), "{}", r.stderr);
    let r = rythme(&["doc", "--htm", "x.ry"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("did you mean `--html`?"), "{}", r.stderr);
    let r = rythme(&["doc", "x.ry", "--out"]);
    assert_eq!(r.code, 2);
    let dir = temp_dir("errors");
    std::fs::write(dir.join("bad.ry"), "fn f(x) =\nprint(\"hello)\n").unwrap();
    std::fs::write(dir.join("good.ry"), "## Twice.\nfn twice(x) = x * 2\n").unwrap();
    std::fs::write(dir.join("lost.ry"), "use \"missing.ry\"\n").unwrap();
    let r = rythme(&["doc", dir.to_str().unwrap()]);
    assert_eq!(r.code, 1, "{}", r.stdout);
    assert!(r.stderr.contains("error[E002]") && r.stderr.contains("bad.ry:2:7"), "{}", r.stderr);
    assert!(r.stderr.contains("`missing.ry` not found"), "{}", r.stderr);
    assert!(
        r.stdout.contains("fn twice(x)") && r.stdout.contains("Nothing public to document."),
        "the other files are documented:\n{}",
        r.stdout
    );
    let r = rythme(&["doc", dir.join("nothing.ry").to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E017]"), "{}", r.stderr);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn doc_has_a_help_topic() {
    for (lang, args) in [("en", ["help", "doc"]), ("fr", ["aide", "documentation"]), ("en", ["doc", "--help"])] {
        let r = rythme_lang(lang, &args);
        assert_eq!(r.code, 0);
        assert!(r.stdout.starts_with("DOCUMENTATION"), "{}", r.stdout);
    }
    assert!(rythme(&["help"]).stdout.contains("rythme doc file.ry"));
}

#[test]
fn doc_follows_named_modules() {
    let dir = temp_dir("modules");
    std::fs::create_dir_all(dir.join("shapes")).unwrap();
    std::fs::write(dir.join("geometry.ry"), GEOMETRY).unwrap();
    std::fs::write(dir.join("shapes/circle.ry"), "## Perimeter of a circle.\nfn perimeter(r) = 2 * pi * r\nfn _half(x) = x / 2\n").unwrap();
    std::fs::write(
        dir.join("main.ry"),
        "use geometry\nuse shapes/circle as round\n\n## The `geometry.area` of a shape plus the `round.perimeter(1)`.\nfn both(s) = geometry.area(s) + round.perimeter(1)\n",
    )
    .unwrap();
    let r = rythme(&["doc", dir.join("main.ry").to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    let md = &r.stdout;
    // Both used files are documented, each with its own name.
    assert!(md.contains("Shapes and their areas.") && md.contains("Perimeter of a circle."), "{md}");
    assert!(md.contains("geometry.ry`](#") && md.contains("circle.ry`](#") && md.contains(") as `round`"), "{md}");
    assert!(!md.contains("as `geometry`"), "{md}");
    // `geometry.area` leads to `area` in geometry.ry.
    assert!(md.contains("[`geometry.area`](#area)") && md.contains("[`round.perimeter(1)`](#perimeter)"), "{md}");
    assert!(!md.contains("_half"), "{md}");
    let r = rythme_lang("fr", &["doc", "--html", dir.join("main.ry").to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.contains("</a> sous le nom <code>round</code>"), "{}", r.stdout);
    std::fs::remove_dir_all(&dir).ok();
}
