//! Types written in the code: `fn area(w: Int, h: Int) -> Int`, `total:
//! Float = 0.0`, `Circle(radius: Float)`. They are optional: the parser reads
//! them where nothing else could be written (every program written before
//! them keeps its syntax tree), running never looks at them, the checker
//! verifies them (E027) and uses them, and the formatter, `rythme doc` and
//! the help know them.

mod common;

use common::{rythme, rythme_lang};
use rythme::ast::{Item, PatKind, Stmt, TypeKind};
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

/// The syntax tree without positions.
fn tree(src: &str) -> String {
    let debug = format!("{:?}", parse(src));
    let mut out = String::new();
    let mut rest = debug.as_str();
    while let Some(i) = rest.find("Span {") {
        out.push_str(&rest[..i]);
        let end = rest[i..].find('}').unwrap() + i + 1;
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

fn temp_file(name: &str, content: &str) -> std::path::PathBuf {
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("rythme-written-{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    path
}

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
fn types_are_read_where_they_are_written() {
    let items = parse(
        "fn area(w: Int, h: Int) -> Int = w * h\ntotal: Float = 0.0\ntype Shape = Circle(radius: Float) | Dot\nxs = [1].map do |n: Int| n end\nf = fn(x: [Int]) -> {a: Int} = {a: 1}\n",
    );
    let Item::Stmt(Stmt::Fn(area)) = &items[0] else { panic!() };
    assert_eq!(area.signature(), "area(w: Int, h: Int) -> Int");
    assert_eq!(area.param_type(1).unwrap().to_string(), "Int");
    let Item::Stmt(Stmt::Let(p, _)) = &items[1] else { panic!() };
    assert!(matches!(p.kind, PatKind::Bind(_)));
    assert_eq!(p.ty.as_ref().unwrap().to_string(), "Float");
    let Item::Type(t) = &items[2] else { panic!() };
    assert_eq!(t.variants[0].signature(), "Circle(radius: Float)");
    assert_eq!(t.variants[1].signature(), "Dot");
    let tree_of_f = tree("f = fn(x: [Int]) -> {a: Int} = {a: 1}");
    assert!(tree_of_f.contains("ret: TypeExpr { kind: Record("), "{tree_of_f}");
    // Every kind of type, written back as the formatter writes it.
    for t in [
        "Int",
        "List(Int)",
        "[Text]",
        "Dict(Text, Int)",
        "{name: Text, age: Int}",
        "fn(Int, Int) -> Bool",
        "fn() -> Unit",
        "geo.Shape",
        "a",
        "Result(List(a), Text)",
    ] {
        let items = parse(&format!("x: {t} = 1"));
        let Item::Stmt(Stmt::Let(p, _)) = &items[0] else { panic!() };
        assert_eq!(p.ty.as_ref().unwrap().to_string(), t);
    }
    let items = parse("x: geo.Shape = 1");
    let Item::Stmt(Stmt::Let(p, _)) = &items[0] else { panic!() };
    assert!(matches!(&p.ty.as_ref().unwrap().kind, TypeKind::Named(n, _) if &*n.name() == "geo.Shape"));
}

#[test]
fn a_colon_that_touches_the_name_starts_a_type_only_where_a_symbol_cannot_be() {
    // In the parameters of a function or a block: a type.
    assert_eq!(tree("fn f(x:Int, y:Int) = x"), tree("fn f(x: Int, y: Int) = x"));
    assert_eq!(tree("xs.map do |x:Int| x end"), tree("xs.map do |x: Int| x end"));
    assert_eq!(tree("xs.map { |x:Int, y:Int| x }"), tree("xs.map { |x: Int, y: Int| x }"));
    // Elsewhere, symbols keep their meaning: `puts:ok` calls `puts` with `:ok`.
    assert!(tree("puts:ok").contains("Symbol"));
    assert!(tree("xs.map do |x| puts:ok end").contains("Symbol"));
    assert!(tree("r = {a:b}").contains("Var"));
    assert_eq!(tree("fn f(x :Int, y :Int) = x"), tree("fn f(x: Int, y: Int) = x"));
    // After a bracket, a `:` touching the type makes a symbol: explained.
    let e = parse_error("fn f([x] :List(Int)) = x");
    assert!(e.contains("put `:` right after its name, then a space: `x: List`"), "{e}");
    let e = parse_error("total:Float = 0.0");
    assert!(e.contains("put a space after `:`: `total: Float = ...`"), "{e}");
}

#[test]
fn an_arrow_after_the_parameters_is_the_type_of_the_result_only_before_the_body() {
    assert!(tree("fn f(x) -> Int = x").contains("ret: TypeExpr"));
    assert!(tree("fn f(x) -> Int\n  x\nend").contains("ret: TypeExpr"));
    assert!(tree("fn f(x) -> Int if x > 0 = x\nfn f(x) = 0").contains("ret: TypeExpr"));
    assert!(tree("g = fn(x) -> fn(Int) -> Int = fn(y) = x + y").contains("Func("));
    // `fn(x) -> x * 2` is not a type: the usual message.
    let e = parse_error("f = fn(x) -> x * 2");
    assert!(e.contains("functions use `=`, not `->`"), "{e}");
    let e = parse_error("xs.map(fn(x) -> x)");
    assert!(e.contains("functions use `=`, not `->`"), "{e}");
}

#[test]
fn a_name_with_a_type_is_given_its_value_right_away() {
    let e = parse_error("total: Float\ntotal = 1.0");
    assert!(e.contains("`=` and a value") && e.contains("total: Float = 0.0"), "{e}");
    let e = parse_error("x: = 1");
    assert!(e.contains("a type") && e.contains("rythme help types"), "{e}");
    let e = parse_error("type Shape = Circle(Float)");
    assert!(e.contains("write its name first: `Circle(radius: Float)`"), "{e}");
    // Patterns too.
    assert!(tree("[a, b]: List(Int) = [1, 2]").contains("ty: TypeExpr"));
    assert!(tree("Some(x): Option(Int) = Some(1)").contains("ty: TypeExpr"));
}

// ----- running, formatting ---------------------------------------------------------

#[test]
fn written_types_change_nothing_when_the_program_runs() {
    let typed = "type Shape = Circle(radius: Float) | Rect(w: Float, h: Float)\nfn area(s: Shape) -> Float = match s\n  Circle(r) -> 3.0 * r * r\n  Rect(w, h) -> w * h\nend\ntotal: Float = 0.5\nxs: List(Int) = [1, 2, 3].map do |n: Int| n * 2 end\nf: fn(Int) -> Int = fn(x: Int) -> Int = x + 1\nprint(area(Circle(1.0)) + total, xs, f(1))\n";
    let plain = "type Shape = Circle(radius) | Rect(w, h)\nfn area(s) = match s\n  Circle(r) -> 3.0 * r * r\n  Rect(w, h) -> w * h\nend\ntotal = 0.5\nxs = [1, 2, 3].map do |n| n * 2 end\nf = fn(x) = x + 1\nprint(area(Circle(1.0)) + total, xs, f(1))\n";
    runs(typed, "3.5 [2, 4, 6] 2\n");
    runs(plain, "3.5 [2, 4, 6] 2\n");
}

#[test]
fn the_formatter_keeps_written_types_and_spaces_them() {
    let src = "fn area( w:Int,h : Int )->Int = w*h\ntotal:  Float=0.0\nr : {name :Text, age:Int} = {name: \"Ada\", age: 36}\nf: fn( Int )->[ Int ] = fn(x) = [x]\ntype Shape = Circle( radius:Float )|Dot\nys = [1].map do |n:Int| n end\n";
    let want = "fn area(w: Int, h: Int) -> Int = w * h\ntotal: Float = 0.0\nr: {name: Text, age: Int} = {name: \"Ada\", age: 36}\nf: fn(Int) -> [Int] = fn(x) = [x]\ntype Shape = Circle(radius: Float) | Dot\nys = [1].map do |n: Int| n end\n";
    assert_eq!(format(src).unwrap(), want);
    assert_eq!(format(want).unwrap(), want);
    let multi = "fn pick(xs: List(a)) -> a\n  xs[0]\nend\n";
    assert_eq!(format(multi).unwrap(), multi);
}

// ----- the checker ---------------------------------------------------------------

#[test]
fn values_that_do_not_fit_their_written_type_are_reported() {
    let e = finds(
        "total: Float = 0",
        &["`total` is written `Float`, but its value is Int 0", "the type written for `total`", "write it with a decimal point"],
    );
    assert!(!e.contains("ran"));
    finds("count: Int = 2.5", &["`count` is written `Int`, but its value is Float 2.5", "convert it with `int(...)`"]);
    finds("names: List(Text) = [1, 2]", &["`names` is written `List(Text)`, but its value is List [1, 2]"]);
    finds("p: {name: Text} = {age: 3}", &["but its value is"]);
    finds("o: Option(Int) = Some(\"a\")", &["`o` is written `Option(Int)`"]);
    finds("fn f()\n  x: Text = 1\n  x\nend\nprint(f())", &["`x` is written `Text`"]);
    finds("[a, b]: List(Text) = [1, 2]\nprint(a, b)", &["this value is List [1, 2], but the type written for it is `List(Text)`"]);
}

#[test]
fn arguments_are_checked_against_the_types_of_the_parameters() {
    let e = finds(
        "fn area(w: Int, h: Int) -> Int = w * h\nprint(area(2, \"big\"))",
        &["`area` expects `Int` for `h`, but receives Text \"big\"", "the type written for `h`"],
    );
    assert!(!e.contains("E005"), "{e}");
    // Even when the function would work: the written type is a promise.
    finds("fn show(x: Int) = print(x)\nshow(\"a\")", &["`show` expects `Int` for `x`"]);
    finds("type Shape = Circle(radius: Float) | Dot\nprint(Circle(1))", &["`Circle` expects `Float` for `radius`, but receives Int 1"]);
    finds("fn apply(f: fn(Int) -> Int, x: Int) -> Int = f(x)\nprint(apply(print, 1))", &["`apply` expects `fn(Int) -> Int` for `f`"]);
    finds("fn total(xs: List(Int)) -> Int = sum(xs)\nprint(total([\"a\"]))", &["`total` expects `List(Int)` for `xs`"]);
    finds("print([1, 2].map do |x: Text| x end)", &["this function receives a value of type Int, but `x` is written `Text`"]);
    finds("fn outer()\n  fn inner(n: Int) = n + 1\n  inner(\"a\")\nend\nprint(outer())", &["`inner` expects `Int` for `n`"]);
}

#[test]
fn type_variables_stand_for_the_same_type_in_a_definition() {
    finds(
        "fn pair(a: t, b: t) -> List(t) = [a, b]\nprint(pair(1, \"x\"))",
        &["`t` cannot be both Int and Text in this call to `pair`", "`t` is Int here"],
    );
    // What a call gives follows from its arguments.
    let e = finds(
        "fn first_of(xs: List(a)) -> a = xs[0]\nn: Text = first_of([1, 2])\nprint(n)",
        &["`n` is written `Text`, but its value is a value of type Int"],
    );
    assert!(!e.contains("first_of` expects"), "{e}");
    runs("fn first_of(xs: List(a)) -> a = xs[0]\nprint(first_of([1, 2]) + 1, first_of([\"a\"]) + \"b\")\n", "2 ab\n");
}

#[test]
fn results_are_checked_against_the_written_result_type() {
    finds(
        "fn label(n: Int) -> Text = n\nprint(label(1))",
        &["`label` is written to give `Text`, but gives a value of type Int", "the type written for the result of `label`"],
    );
    finds(
        "fn half(n: Int) -> Int\n  h = n / 2\n  h\nend\nprint(half(4))",
        &["`half` is written to give `Int`, but gives a value of type Float"],
    );
    finds(
        "fn get_n(t: Text) -> Int = parse_int(t)?\nprint(get_n(\"1\"))",
        &["`get_n` is written to give `Int`, but its `?` may give back", "-> Option(Int)"],
    );
    // Inside the function, a parameter has its written type.
    let e = rythme(&["check", temp_file("inside.ry", "fn shout(t: Text) = t + 1\nprint(shout(\"a\"))\n").to_str().unwrap()]).stderr;
    assert!(e.contains("error[E005]") && e.contains("Text + Int"), "{e}");
    // Outside, a call gives the written type of the result.
    finds("fn seven() -> Int = 7\nt: Text = seven()\nprint(t)", &["`t` is written `Text`, but its value is a value of type Int"]);
}

#[test]
fn types_that_do_not_exist_are_reported() {
    finds("fn f(x: int) = x\nprint(f(1))", &["unknown type `int`", "in Rythme, write `Int`"]);
    finds("fn f(x: String) = x\nprint(f(1))", &["unknown type `String`", "in Rythme, write `Text`"]);
    finds("fn f(x: Shap) = x\ntype Shape = Dot\nprint(f(Dot))", &["unknown type `Shap`", "did you mean `Shape`?"]);
    finds("fn f(x: item) = x\nprint(f(1))", &["unknown type `item`", "one lowercase letter, like `a`"]);
    finds("fn f(x: List(Int, Text)) = x\nprint(f([]))", &["`List` takes one type: `List(Int)`"]);
    finds("fn f(x: Dict(Text)) = x\nprint(f(dict([])))", &["`Dict` takes two types: `Dict(Text, Int)`"]);
    finds("fn f(x: Int(Text)) = x\nprint(f(1))", &["`Int` takes no type between parentheses"]);
    finds("type Shape = Dot\nfn f(x: Shape(Int)) = x\nprint(f(Dot))", &["`Shape` takes no type between parentheses"]);
}

#[test]
fn correct_programs_with_written_types_are_not_reported() {
    runs(
        "fn fact(n: Int) -> Int = if n == 0 then 1 else n * fact(n - 1)
fn half(x: Int) -> Float = x / 2
fn avg(xs: List(Float)) -> Float = sum(xs) / len(xs)
fn first_one(xs: List(Int)) -> Option(Int) = first(xs)
fn name_of(p: {name: Text}) -> Text = p.name
fn twice(f: fn(Int) -> Int, x: Int) -> Int = f(f(x))
fn double(x: Int) -> Int = x * 2
fn parse(t: Text) -> Result(Int, Text) = parse_int(t)
fn next_of(xs: List(Int)) -> Option(Int)
  n = first(xs)?
  Some(n + 1)
end
fn count_of(d: Dict(Text, Int), k: Text) -> Int = get(d, k).unwrap_or(0)
fn evens(xs: [Int]) -> [Int] = xs.select do |x: Int| x % 2 == 0 end
fn sizes(xs: List(Text)) -> List(Int) = map(xs, len)
fn same(a: t) -> t = a
xs: List(Float) = [1.0, 2.5]
e: List(Int) = []
none: Option(Int) = None
z: Int = same(3) + 1
s: Set(Int) = set([1, 2])
f: fn(Int) -> Int = fn(x) = x + 1
h: Record = {a: 1}
print(fact(5), half(3), avg([1.0, 2.0]), first_one([1]), name_of({name: \"Ada\", age: 3}), twice(double, 3))
print(parse(\"4\"), next_of([]), count_of(dict([[\"a\", 1]]), \"a\"), evens([1, 2, 3, 4]), sizes([\"ab\"]), xs, e, none, z, same(\"a\"), s, f(1), h)
",
        "120 1.5 1.5 Some(1) Ada 12\nOk(4) None 1 [2, 4] [2] [1.0, 2.5] [] None 4 a set(1, 2) 2 {a: 1}\n",
    );
    // A value the checker does not know is never reported.
    runs("n: Int = unwrap(parse_json(\"1\"))\nprint(n)\n", "1\n");
    // The fields of a constructor have their written types in patterns.
    let e = rythme(&["check", temp_file("fields.ry", "type Shape = Circle(radius: Float) | Dot\nfn f(s: Shape) = match s\n  Circle(r) -> r + \"cm\"\n  Dot -> \"\"\nend\nprint(f(Dot))\n").to_str().unwrap()]).stderr;
    assert!(e.contains("error[E005]") && e.contains("Float + Text"), "{e}");
}

#[test]
fn types_written_in_used_files_are_checked_too() {
    let dir = temp_file("geometry.ry", "type Shape = Circle(radius: Float) | Square(side: Float)\n\nfn area(s: Shape) -> Float =\n  match s\n    Circle(r) -> 3.0 * r * r\n    Square(c) -> c * c\n  end\n\nfn twice(x: Int) -> Int = x * 2\n");
    let dir = dir.parent().unwrap();
    std::fs::write(dir.join("main.ry"), "use geometry\n\ntype Shape = Dot\n\nprint(geometry.area(geometry.Circle(1.0)))\nprint(geometry.area(Dot))\nprint(geometry.twice(\"a\"))\nx = geometry.twice(2) + \"a\"\n").unwrap();
    let r = rythme(&["check", dir.join("main.ry").to_str().unwrap()]);
    assert!(r.stderr.contains("`geometry.area` expects `geometry.Shape` for `s`, but receives a value of type Shape"), "{}", r.stderr);
    assert!(r.stderr.contains("`geometry.twice` expects `Int` for `x`, but receives Text \"a\""), "{}", r.stderr);
    assert!(r.stderr.contains("geometry.ry:9:13"), "{}", r.stderr);
    assert!(r.stderr.contains("cannot use `+` with Int and Text"), "{}", r.stderr);
    assert!(!r.stderr.contains("Circle(1.0)"), "{}", r.stderr);
    std::fs::write(dir.join("plain.ry"), "use \"geometry.ry\"\n\nprint(area(Circle(1.0)))\nprint(twice(2.5))\n").unwrap();
    let r = rythme(&["check", dir.join("plain.ry").to_str().unwrap()]);
    assert!(r.stderr.contains("`twice` expects `Int` for `x`, but receives Float 2.5"), "{}", r.stderr);
    assert_eq!(r.stderr.matches("error[").count(), 1, "{}", r.stderr);
    // The types of a private name stay in its file.
    std::fs::write(dir.join("tools.ry"), "fn _clean(x: Int) -> Int = x\nfn cleaned(x: Int) -> Int = _clean(x)\n").unwrap();
    std::fs::write(dir.join("own.ry"), "use \"tools.ry\"\n\nfn _clean(t) = trim(t)\nprint(_clean(\" a \"), cleaned(1))\n").unwrap();
    let r = rythme(&[dir.join("own.ry").to_str().unwrap()]);
    assert_eq!((r.code, r.stdout.as_str()), (0, "a 1\n"), "{}", r.stderr);
}

#[test]
fn messages_in_every_language() {
    let file = temp_file("lang.ry", "total: Float = 0\n");
    let fr = rythme_lang("fr", &["check", file.to_str().unwrap()]).stderr;
    assert!(fr.contains("`total` est écrit `Float`, mais sa valeur est Int 0"), "{fr}");
    for lang in ["es", "de", "pt", "zh"] {
        let out = rythme_lang(lang, &["check", file.to_str().unwrap()]).stderr;
        assert!(out.contains("E027") && !out.contains("is written"), "{lang}: {out}");
    }
}

// ----- the help, `rythme doc` ------------------------------------------------------

#[test]
fn the_help_explains_written_types() {
    let r = rythme(&["explain", "E027"]);
    assert!(r.stdout.contains("A type written in the code does not fit"), "{}", r.stdout);
    for topic in ["types", "annotations"] {
        let r = rythme(&["help", topic]);
        assert!(r.stdout.contains("YOUR OWN TYPES") && r.stdout.contains("TYPES WRITTEN IN THE CODE"), "{}", r.stdout);
    }
    let r = rythme_lang("fr", &["help", "types"]);
    assert!(r.stdout.contains("TYPES ÉCRITS DANS LE CODE"), "{}", r.stdout);
}

#[test]
fn rythme_doc_shows_written_types() {
    let file = temp_file("shapes.ry", "## A shape.\ntype Shape = Circle(radius: Float) | Dot\n\n## Its area.\nfn area(s: Shape) -> Float = 1.0\n\n## The unit.\nunit: Float = 1.0\n");
    let r = rythme(&["doc", file.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    for t in ["type Shape = Circle(radius: Float) | Dot", "fn area(s: Shape) -> Float", "unit: Float = 1.0"] {
        assert!(r.stdout.contains(t), "`{t}` in:\n{}", r.stdout);
    }
}
