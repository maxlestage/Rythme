//! Types with parameters: `type Tree(a) = Leaf | Node(left: Tree(a), value:
//! a, right: Tree(a))`, then `Tree(Int)` in the types written in the code.
//! The parser reads the parameters where `=` was the only thing allowed
//! before (every program written before them keeps its syntax tree), running
//! never looks at them, and the checker verifies and uses them (E027).

mod common;

use common::{rythme, rythme_lang};
use rythme::ast::Item;
use rythme::format::format;
use rythme::{lexer, parser};

fn parse(src: &str) -> Vec<Item> {
    lexer::lex(src, 9).and_then(parser::parse).unwrap_or_else(|d| panic!("{src}: {}", d.message))
}

fn parse_error(src: &str) -> String {
    match lexer::lex(src, 9).and_then(parser::parse) {
        Ok(_) => panic!("{src}: parsed"),
        Err(d) => format!("{}\n{}", d.message, d.helps.join("\n")),
    }
}

fn temp_file(name: &str, content: &str) -> std::path::PathBuf {
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("rythme-params-{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    path
}

const TREE: &str = "type Tree(a) = Leaf | Node(left: Tree(a), value: a, right: Tree(a))\n";

/// `rythme check` reports E027 with these texts, without running the program.
#[track_caller]
fn finds(code: &str, texts: &[&str]) -> String {
    let file = temp_file("found.ry", &format!("print(\"ran\")\n{code}\n"));
    let r = rythme(&["check", file.to_str().unwrap()]);
    assert_eq!(r.code, 1, "expected E027 for:\n{code}\n{}", r.stderr);
    assert!(r.stderr.contains("error[E027]"), "expected E027 for:\n{code}\n{}", r.stderr);
    for t in texts {
        assert!(r.stderr.contains(t), "expected `{t}` for:\n{code}\n{}", r.stderr);
    }
    r.stderr
}

/// The program is checked without a single error or warning, and prints this.
#[track_caller]
fn runs(code: &str, printed: &str) {
    let file = temp_file("quiet.ry", code);
    let r = rythme(&[file.to_str().unwrap()]);
    assert_eq!(r.code, 0, "code:\n{code}\n{}", r.stderr);
    assert!(r.stderr.is_empty(), "code:\n{code}\n{}", r.stderr);
    assert_eq!(r.stdout, printed, "code:\n{code}");
}

// ----- syntax ---------------------------------------------------------------

#[test]
fn the_parameters_of_a_type_are_read_after_its_name() {
    let items = parse("type Pair(a, b) = Pair(first: a, second: b)\ntype Shape = Dot\n");
    let Item::Type(pair) = &items[0] else { panic!() };
    let params: Vec<String> = pair.params.iter().map(|(p, _)| p.to_string()).collect();
    assert_eq!(params, ["a", "b"]);
    assert_eq!(pair.header(), "Pair(a, b)");
    assert_eq!(pair.variants[0].signature(), "Pair(first: a, second: b)");
    let Item::Type(shape) = &items[1] else { panic!() };
    assert!(shape.params.is_empty());
    assert_eq!(shape.header(), "Shape");
    // A type without parameters has the syntax tree it always had.
    assert!(!format!("{items:?}").contains("params: []"));
    assert!(format!("{items:?}").contains("params: [(`a`"));
    // On several lines, and with a written type of the program.
    let items = parse("type Many(a, b) =\n  | One(x: a)\n  | Two(x: a, y: List(b))\nfn f(m: Many(Int, Tree(Text))) = m\n");
    let Item::Type(many) = &items[0] else { panic!() };
    assert_eq!(many.variants.len(), 2);
}

#[test]
fn mistakes_in_the_parameters_are_explained() {
    let e = parse_error("type Tree() = Leaf");
    assert!(e.contains("expected a type variable, like `a`") && e.contains("written without parentheses: `type Tree = ...`"), "{e}");
    let e = parse_error("type Box(Int) = Box(value: Int)");
    assert!(e.contains("expected a type variable, like `a`") && e.contains("one lowercase letter each"), "{e}");
    let e = parse_error("type Box(1) = Box(x)");
    assert!(e.contains("expected a type variable"), "{e}");
    // A field is named first, even in a type with parameters.
    let e = parse_error("type Tree(a) = Leaf | Node(Tree(a), a, Tree(a))");
    assert!(e.contains("write its name first: `Circle(radius: Float)`"), "{e}");
}

#[test]
fn the_formatter_writes_the_parameters_like_the_types_a_type_takes() {
    let src = "type Tree( a ) =   Leaf |   Node(left :Tree( a ),value:a, right: Tree(a))\ntype Pair(a,b)=Pair(first:a,second:b)\nfn f( t:Tree( Int ) )->Pair( Int,Text ) = Pair(1,\"x\")\n";
    let want = "type Tree(a) = Leaf | Node(left: Tree(a), value: a, right: Tree(a))\ntype Pair(a, b) = Pair(first: a, second: b)\nfn f(t: Tree(Int)) -> Pair(Int, Text) = Pair(1, \"x\")\n";
    assert_eq!(format(src).unwrap(), want);
    assert_eq!(format(want).unwrap(), want);
    let multi = "type Many(a, b) =\n  | One(x: a)\n  | Two(x: a, y: b)\n";
    assert_eq!(format(multi).unwrap(), multi);
}

// ----- running ---------------------------------------------------------------

#[test]
fn parameters_change_nothing_when_the_program_runs() {
    let typed = format!(
        "{TREE}fn total(t: Tree(Int)) -> Int = match t\n  Leaf -> 0\n  Node(l, v, r) -> total(l) + v + total(r)\nend\nt: Tree(Int) = Node(Node(Leaf, 1, Leaf), 2, Leaf)\nprint(total(t), t.value, t)\n"
    );
    let plain = "type Tree = Leaf | Node(left, value, right)\nfn total(t) = match t\n  Leaf -> 0\n  Node(l, v, r) -> total(l) + v + total(r)\nend\nt = Node(Node(Leaf, 1, Leaf), 2, Leaf)\nprint(total(t), t.value, t)\n";
    let printed = "3 2 Node(Node(Leaf, 1, Leaf), 2, Leaf)\n";
    runs(&typed, printed);
    runs(plain, printed);
}

// ----- the checker ---------------------------------------------------------------

#[test]
fn values_of_another_type_of_contents_are_reported() {
    let total = format!("{TREE}fn total(t: Tree(Int)) -> Int = match t\n  Leaf -> 0\n  Node(l, v, r) -> total(l) + v + total(r)\nend\n");
    let e = finds(
        &format!("{total}print(total(Node(Leaf, \"two\", Leaf)))"),
        &["`total` expects `Tree(Int)` for `t`, but receives a value of type Tree(Text)", "the type written for `t`"],
    );
    assert!(!e.contains("ran"));
    finds(
        "type Pair(a, b) = Pair(first: a, second: b)\np: Pair(Int, Text) = Pair(1, 2)",
        &["`p` is written `Pair(Int, Text)`, but its value is a value of type Pair(Int, Int)"],
    );
    finds(
        &format!("{TREE}fn leaf() -> Tree(Int) = Node(Leaf, 2.5, Leaf)\nprint(leaf())"),
        &["`leaf` is written to give `Tree(Int)`, but gives"],
    );
    // A constructor keeps its type variable to one type.
    finds(
        &format!("{TREE}t = Node(Leaf, 1, Node(Leaf, \"x\", Leaf))\nprint(t)"),
        &["`a` cannot be both Int and Text in this call to `Node`", "`a` is Int here", "`a` is Text here"],
    );
    // The fields of a value of a written type have their types.
    let e = rythme(&[
        "check",
        temp_file(
            "fields.ry",
            &format!("{TREE}fn f(t: Tree(Int)) = match t\n  Leaf -> \"\"\n  Node(_, v, _) -> v + \"!\"\nend\nprint(f(Leaf))\n"),
        )
        .to_str()
        .unwrap(),
    ])
    .stderr;
    assert!(e.contains("error[E005]") && e.contains("Int + Text"), "{e}");
}

#[test]
fn what_a_call_gives_follows_the_types_of_the_contents() {
    finds(
        "type Pair(a, b) = Pair(first: a, second: b)\nfn first_of(p: Pair(a, b)) -> a = p.first\nn: Text = first_of(Pair(1, \"x\"))\nprint(n)",
        &["`n` is written `Text`, but its value is a value of type Int"],
    );
    let e = rythme(&["check", temp_file("root.ry", &format!("{TREE}fn root(t: Tree(a)) -> Option(a) = match t\n  Leaf -> None\n  Node(_, v, _) -> Some(v)\nend\nprint(unwrap(root(Node(Leaf, 3, Leaf))) + \"!\")\n")).to_str().unwrap()]).stderr;
    assert!(e.contains("error[E005]") && e.contains("Int + Text"), "{e}");
    runs(
        &format!("{TREE}type Pair(a, b) = Pair(first: a, second: b)\nfn first_of(p: Pair(a, b)) -> a = p.first\nfn depth(t: Tree(a)) -> Int = match t\n  Leaf -> 0\n  Node(l, _, r) -> 1 + max(depth(l), depth(r))\nend\nprint(first_of(Pair(1, \"x\")) + 1, first_of(Pair(\"a\", 2)) + \"b\", depth(Node(Leaf, \"x\", Node(Leaf, \"y\", Leaf))))\n"),
        "2 ab 2\n",
    );
}

#[test]
fn types_that_take_a_wrong_number_of_types_are_reported() {
    finds(&format!("{TREE}fn f(t: Tree(Int, Text)) = t\nprint(f(Leaf))"), &["`Tree` takes one type: `Tree(Int)`"]);
    finds(
        "type Pair(a, b) = Pair(first: a, second: b)\nfn f(p: Pair(Int)) = p\nprint(f(Pair(1, 2)))",
        &["`Pair` takes two types: `Pair(Int, Text)`"],
    );
    finds(
        "type T3(a, b, c) = T3(x: a, y: b, z: c)\nfn f(t: T3(Int)) = t\nprint(f(T3(1, 2, 3)))",
        &["`T3` takes 3 types: `T3(Int, Text, Bool)`"],
    );
    finds(
        "type Shape = Dot\nfn f(s: Shape(Int)) = s\nprint(f(Dot))",
        &["`Shape` takes no type between parentheses", "write `Shape` alone"],
    );
    // Alone, like `List`, a type with parameters holds anything.
    runs(&format!("{TREE}fn size(t: Tree) -> Int = match t\n  Leaf -> 0\n  Node(l, _, r) -> size(l) + 1 + size(r)\nend\nprint(size(Node(Leaf, \"a\", Leaf)))\n"), "1\n");
}

#[test]
fn the_parameters_of_a_type_are_checked() {
    finds(
        "type Box(item) = Box(value: item)\nprint(Box(1))",
        &["`item` cannot be a type variable: a type variable is one lowercase letter", "write one letter, like `i`"],
    );
    finds("type Two(a, a) = Two(x: a)\nprint(Two(1))", &["the type variable `a` appears twice in `Two`"]);
    finds(
        "type Box(a) = Box(value: b)\nprint(Box(1))",
        &["`b` is not a parameter of `Box`", "not among `Box(a)`", "`type Box(a, b) = ...`"],
    );
    // Without parameters, a type variable in a field still means any type.
    runs("type Old = Old(x: List(a))\nprint(Old([1]))\n", "Old([1])\n");
}

#[test]
fn types_with_parameters_of_used_files_are_checked_too() {
    let trees = temp_file(
        "trees.ry",
        &format!("{TREE}fn total(t: Tree(Int)) -> Int = match t\n  Leaf -> 0\n  Node(l, v, r) -> total(l) + v + total(r)\nend\n"),
    );
    let dir = trees.parent().unwrap();
    std::fs::write(dir.join("main.ry"), "use trees\n\ntype Shape = Dot\n\nprint(trees.total(trees.Node(trees.Leaf, \"x\", trees.Leaf)))\nfn bad(t: trees.Tree(Text, Int)) = t\nx: trees.Tree(Shape) = trees.Node(trees.Leaf, Dot, trees.Leaf)\ny: trees.Tree(Shape) = trees.Node(trees.Leaf, 1, trees.Leaf)\n").unwrap();
    let r = rythme(&["check", dir.join("main.ry").to_str().unwrap()]);
    assert!(
        r.stderr.contains("`trees.total` expects `trees.Tree(Int)` for `t`, but receives a value of type trees.Tree(Text)"),
        "{}",
        r.stderr
    );
    assert!(r.stderr.contains("`trees.Tree` takes one type: `trees.Tree(Int)`"), "{}", r.stderr);
    assert!(r.stderr.contains("`y` is written `trees.Tree(Shape)`"), "{}", r.stderr);
    assert_eq!(r.stderr.matches("error[").count(), 3, "{}", r.stderr);
    std::fs::write(dir.join("plain.ry"), "use \"trees.ry\"\n\nprint(total(Node(Leaf, 2, Leaf)))\nprint(total(Node(Leaf, 2.5, Leaf)))\n")
        .unwrap();
    let r = rythme(&["check", dir.join("plain.ry").to_str().unwrap()]);
    assert!(r.stderr.contains("`total` expects `Tree(Int)` for `t`, but receives a value of type Tree(Float)"), "{}", r.stderr);
    assert_eq!(r.stderr.matches("error[").count(), 1, "{}", r.stderr);
}

#[test]
fn the_repl_accepts_the_types_of_its_earlier_lines() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .env("HOME", std::env::temp_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let input = format!("{TREE}fn total(t: Tree(Int)) -> Int = match t\n  Leaf -> 0\n  Node(l, v, r) -> total(l) + v + total(r)\nend\ntotal(Node(Leaf, 21, Leaf))\n:quit\n");
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    let (stdout, stderr) = (String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("21") && !stderr.contains("error"), "{stdout}\n{stderr}");
}

#[test]
fn messages_in_every_language() {
    let file = temp_file("lang.ry", &format!("{TREE}fn f(t: Tree(Int, Text)) = t\nprint(f(Leaf))\n"));
    let fr = rythme_lang("fr", &["check", file.to_str().unwrap()]).stderr;
    assert!(fr.contains("`Tree` prend un seul type : `Tree(Int)`"), "{fr}");
    let file = temp_file("lang.ry", "type Box(a) = Box(value: b)\nprint(Box(1))\n");
    for lang in ["es", "de", "pt", "zh", "it", "ja"] {
        let out = rythme_lang(lang, &["check", file.to_str().unwrap()]).stderr;
        assert!(out.contains("E027") && !out.contains("is not a parameter"), "{lang}: {out}");
    }
}

// ----- help, `rythme doc` -------------------------------------------------------

#[test]
fn help_shows_the_types_of_the_fields_of_a_constructor() {
    let file =
        temp_file("help.ry", &format!("{TREE}type Shape = Circle(radius: Float) | Rect(w, h)\nhelp(Node)\nhelp(Circle)\nhelp(Rect)\n"));
    let r = rythme(&[file.to_str().unwrap()]);
    assert_eq!(
        r.stdout,
        "Node(left: Tree(a), value: a, right: Tree(a))\n  A constructor of the type `Tree(a)`.\nCircle(radius: Float)\n  A constructor of the type `Shape`.\nRect(w, h)\n  A constructor of the type `Shape`.\n",
        "{}",
        r.stderr
    );
    for topic in ["types", "syntax"] {
        let r = rythme(&["help", topic]);
        assert!(r.stdout.contains("type Tree(a)") || r.stdout.contains("type Box(a)"), "{topic}: {}", r.stdout);
    }
    assert!(rythme(&["help"]).stdout.contains("types written in the code"));
}

#[test]
fn rythme_doc_shows_the_parameters() {
    let file = temp_file("trees.ry", &format!("## A tree.\n{TREE}"));
    let r = rythme(&["doc", file.to_str().unwrap()]);
    assert!(r.stdout.contains("type Tree(a) = Leaf | Node(left: Tree(a), value: a, right: Tree(a))"), "{}", r.stdout);
}
