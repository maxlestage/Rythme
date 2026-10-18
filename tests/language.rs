//! The language itself: values, functions, patterns, errors.

mod common;

use common::{fails, ok, warns};

#[test]
fn numbers() {
    ok("print(1 + 2 * 3, (1 + 2) * 3)", "7 9");
    ok("print(7 / 2, 7 // 2, -7 // 2, 7 % 3, -7 % 3)", "3.5 3 -4 1 2");
    ok("print(2 ** 10, 2 ** -1, -2 ** 2)", "1024 0.5 -4");
    ok("print(1_000_000, 0xff, 0b101, 1.5e3)", "1000000 255 5 1500.0");
    ok("print(0.1 + 0.2, 3.0, 1 + 0.5)", "0.30000000000000004 3.0 1.5");
    ok("print(1 == 1.0, 2 < 3, \"a\" < \"b\", [1, 2] < [1, 3])", "true true true true");
    ok("print(abs(-3), min(3, 1, 2), max([4, 8]), floor(2.7), ceil(2.1), round(2.5), sqrt(16))", "3 1 8 2 3 3 4.0");
    ok("print(int(\"42\"), int(3.9), float(\"2.5\"), text(42) + \"!\")", "42 3 2.5 42!");
}

#[test]
fn texts() {
    ok("name = \"Ada\"\nprint(\"Hello, {name}! {1 + 1}\")", "Hello, Ada! 2");
    ok("print(\"a\\tb\\n\\{c\\}\")", "a\tb\n{c}");
    ok("print(len(\"été\"), upper(\"abc\"), trim(\"  x \"), reverse(\"abc\"))", "3 ABC x cba");
    ok("print(split(\"a,b\", \",\"), join([\"a\", \"b\"], \"-\"), \"hello\"[1], \"hello\"[-1])", "[\"a\", \"b\"] a-b e o");
    ok("print(\"{[1, \"x\"]}\")", "[1, \"x\"]");
}

#[test]
fn lists() {
    ok("print(1..5, 5..1, [0, ..[1, 2], 3])", "[1, 2, 3, 4, 5] [] [0, 1, 2, 3]");
    ok("xs = [3, 1, 2]\nprint(xs[0], xs[-1], len(xs), sort(xs), xs)", "3 2 3 [1, 2, 3] [3, 1, 2]");
    ok("print([1, 2, 3] |> map(fn(x) = x * 2) |> filter(fn(x) = x > 2) |> sum)", "10");
    ok("print(fold(1..4, 1, fn(acc, x) = acc * x), first([]), last([1, 2]), get([1], 3))", "24 None Some(2) None");
    ok("print(unique([1, 2, 1]), take([1, 2, 3], 2), drop([1, 2, 3], 2), push([1], 2))", "[1, 2] [1, 2] [3] [1, 2]");
    ok("print(any([1, 2], is_even), all([1, 2], is_even), count([1, 2, 4], is_even), find([1, 4], is_even))", "true false 2 Some(4)");
    ok("[1, 2] |> each(fn(x) = print(x))", "1\n2");
}

#[test]
fn records() {
    ok("p = {name: \"Ada\", age: 36}\nprint(p.name, p[\"age\"], {..p, age: 37})", "Ada 36 {name: \"Ada\", age: 37}");
    ok("{name, age} = {name: \"Ada\", age: 36}\nprint(name, age)", "Ada 36");
    ok("r = {a: 1}\nprint(keys(put(r, \"b\", 2)), has(r, \"b\"), remove({a: 1, b: 2}, \"a\"))", "[\"a\", \"b\"] false {b: 2}");
    ok("print({type: 1, \"full name\": 2})", "{type: 1, \"full name\": 2}");
    ok("r = {double: fn(x) = x * 2}\nprint(r.double(21))", "42");
}

#[test]
fn functions() {
    ok("fn add(a, b) = a + b\nprint(add(2, 3))", "5");
    ok("fn f(x)\n  y = x + 1\n  y * 2\nend\nprint(f(4))", "10");
    ok("fn fib(0) = 0\nfn fib(1) = 1\nfn fib(n) = fib(n - 1) + fib(n - 2)\nprint(fib(15))", "610");
    ok("fn sign(n) if n < 0 = \"neg\"\nfn sign(0) = \"zero\"\nfn sign(_) = \"pos\"\nprint(sign(-1), sign(0), sign(1))", "neg zero pos");
    ok("fn adder(n) = fn(x) = x + n\nadd2 = adder(2)\nprint(add2(40))", "42");
    ok("fn outer(n)\n  fn helper(0) = 0\n  fn helper(k) = k + helper(k - 1)\n  helper(n)\nend\nprint(outer(4))", "10");
    ok("print(\"  hi \" |> trim |> upper, \"abc\".upper(), [3, 1].sort())", "HI ABC [1, 3]");
    ok("print(10 |> fn(x) = x + 1)", "11");
}

#[test]
fn tail_calls_do_not_grow_the_stack() {
    ok("fn count(0, acc) = acc\nfn count(n, acc) = count(n - 1, acc + 1)\nprint(count(1000000, 0))", "1000000");
    ok("fn even(0) = true\nfn even(n) = odd(n - 1)\nfn odd(0) = false\nfn odd(n) = even(n - 1)\nprint(even(100001))", "false");
    ok("fn go(n) = if n == 0 then \"done\" else match n\n  _ -> go(n - 1)\nend\nprint(go(500000))", "done");
}

#[test]
fn deep_recursion_is_an_error_not_a_crash() {
    let r = fails("fn f(n) = f(n - 1) + 1\nprint(f(1))", "E014");
    assert!(r.stderr.contains("tail call"), "{}", r.stderr);
    ok("fn depth(0) = 0\nfn depth(n) = 1 + depth(n - 1)\nprint(depth(20000))", "20000");
}

#[test]
fn patterns() {
    ok(
        "fn d(x) = match x\n  0 -> \"zero\"\n  1 | 2 -> \"small\"\n  n if n < 0 -> \"neg\"\n  _ -> \"big\"\nend\nprint(map([0, 2, -1, 9], d))",
        "[\"zero\", \"small\", \"neg\", \"big\"]",
    );
    ok("fn s([]) = 0\nfn s([x, ..rest]) = x + s(rest)\nprint(s([1, 2, 3]))", "6");
    ok("[a, b, ..rest] = [1, 2, 3, 4]\nprint(a, b, rest)", "1 2 [3, 4]");
    ok("fn name({name}) = name\nprint(name({name: \"Ada\", age: 1}))", "Ada");
    ok("print(match Some([1, 2])\n  Some([x, y]) -> x + y\n  _ -> 0\nend)", "3");
}

#[test]
fn patterns_on_ranges_and_texts() {
    let grade = "fn grade(n) = match n\n  90..100 -> \"A\"\n  -10..-1 | 0.0..0.5 -> \"low\"\n  _ -> \"other\"\nend\n";
    // Both ends are included; an Int range matches a Float between its ends, and the other way round.
    ok(
        &format!("{grade}print([90, 100, 95.5, 100.5, -10, -1, 0, 0.5, 0.6, -0.0, \"90\", [90]] |> map(grade))"),
        "[\"A\", \"A\", \"A\", \"other\", \"low\", \"low\", \"low\", \"low\", \"other\", \"low\", \"other\", \"other\"]",
    );
    ok(
        "fn digit(0..9) = true\nfn digit(_) = false\nprint(digit(0), digit(9), digit(10), digit(-1), digit(4.5))",
        "true true false false true",
    );
    ok("print(match 9223372036854775807\n  -9223372036854775808..9223372036854775807 -> \"all\"\n  _ -> \"none\"\nend)", "all");
    ok("print(match 1.0e300\n  0..9223372036854775807 -> \"in\"\n  _ -> \"out\"\nend)", "out");
    ok("print(match [3, 50]\n  [1..9, 10..99] -> \"digit, then two digits\"\n  _ -> \"?\"\nend)", "digit, then two digits");
    // The start of a text; the rest is given a name.
    let polite = "fn polite(\"Mr \" + name) = \"Mister {name}\"\nfn polite(\"Mr\" + _) = \"Mr?\"\nfn polite(other) = other\n";
    ok(
        &format!("{polite}print(polite(\"Mr Smith\"), polite(\"Mr \"), polite(\"Mrs\"), polite(\"Ms Lee\"), polite(\"\"))"),
        "Mister Smith Mister  Mr? Ms Lee",
    );
    ok("print(match \"éte\"\n  \"é\" + rest -> rest\n  _ -> \"?\"\nend)", "te");
    ok("print(match \"x\"\n  \"\" + all -> all\nend)", "x");
    ok("print(match 5\n  \"5\" + _ -> \"text\"\n  _ -> \"number\"\nend)", "number");
    // In definitions.
    ok("\"Dr \" + who = \"Dr Who\"\nprint(who)", "Who");
    ok("1..9 = 5\nprint(\"ok\")", "ok");
    let r = fails("\"Dr \" + who = \"Mr X\"\nprint(who)", "E006");
    assert!(r.stderr.contains("Text \"Mr X\""), "{}", r.stderr);
    fails("fn f(1..9) = 1\nprint(f(10))", "E006");
}

#[test]
fn patterns_on_ranges_with_large_numbers_and_blocks() {
    // A large whole number is beyond every Int end, and compared exactly with a Float end.
    let size = "fn size(n) = match n\n  1..9 -> \"digit\"\n  0.0..1.0e40 -> \"in\"\n  _ -> \"out\"\nend\n";
    ok(&format!("{size}print(size(10 ** 30), size(10 ** 50), size(-(10 ** 30)), size(5))"), "in out out digit");
    ok("print(match 10 ** 30\n  -9223372036854775808..9223372036854775807 -> \"small\"\n  _ -> \"large\"\nend)", "large");
    let r = fails("print(match 1\n  0..100000000000000000000 -> 1\n  _ -> 2\nend)", "E001");
    assert!(r.stderr.contains("64 bits"), "{}", r.stderr);
    // The parameters of blocks are patterns too.
    ok("[[1, \"Mr A\"], [2, \"Mr B\"]].each do |[1..9, \"Mr \" + who]| print(who) end", "A\nB");
    fails("[[20, \"Mr B\"]].each do |[1..9, \"Mr \" + who]| print(who) end", "E006");
    ok("print([3, 30].map { |n| match n\n  1..9 -> \"one\"\n  _ -> \"more\"\nend })", "[\"one\", \"more\"]");
}

#[test]
fn patterns_on_ranges_and_texts_badly_written() {
    let r = fails("print(match 1\n  x..5 -> 1\n  _ -> 2\nend)", "E001");
    assert!(r.stderr.contains("a range in a pattern goes from a number to a number"), "{}", r.stderr);
    assert!(r.stderr.contains("use a guard"), "{}", r.stderr);
    let r = fails("print(match 1\n  1.. -> 1\n  _ -> 2\nend)", "E001");
    assert!(r.stderr.contains("expected a number after `..`"), "{}", r.stderr);
    let r = fails("print(match \"a\"\n  1..\"z\" -> 1\n  _ -> 2\nend)", "E001");
    assert!(r.stderr.contains("expected a number after `..`"), "{}", r.stderr);
    let r = fails("print(match \"a\"\n  n + \"!\" -> 1\n  _ -> 2\nend)", "E001");
    assert!(r.stderr.contains("only a text can come before `+` in a pattern"), "{}", r.stderr);
    assert!(r.stderr.contains("ends_with"), "{}", r.stderr);
    let r = fails("print(match \"a\"\n  \"a\" + Some(x) -> 1\n  _ -> 2\nend)", "E001");
    assert!(r.stderr.contains("after `+`, a pattern takes a name or `_`"), "{}", r.stderr);
    fails("print(match \"a\"\n  \"{1}\" + x -> 1\n  _ -> 2\nend)", "E001");
    fails("fn f(\"a\" + x + y) = 1", "E001");
    // Alternatives cannot give names.
    fails("print(match \"a\"\n  \"a\" + x | \"b\" + x -> x\n  _ -> \"\"\nend)", "E001");
}

#[test]
fn types() {
    ok(
        "type Shape = Circle(r) | Square(side) | Empty\nfn area(Circle(r)) = 3 * r * r\nfn area(Square(s)) = s * s\nfn area(Empty) = 0\nprint(map([Circle(1), Square(2), Empty], area))",
        "[3, 4, 0]",
    );
    ok("type P = P(x, y)\np = P(1, 2)\nprint(p.x, p.y, p, type_of(p))", "1 2 P(1, 2) P");
    ok("type T = Leaf | Node(l, v, r)\nprint(Node(Leaf, 1, Leaf) == Node(Leaf, 1, Leaf))", "true");
}

#[test]
fn option_result_and_question_mark() {
    ok(
        "fn f(a, b)\n  x = parse_int(a)?\n  y = parse_int(b)?\n  Ok(x + y)\nend\nprint(f(\"1\", \"2\"), is_err(f(\"1\", \"z\")))",
        "Ok(3) true",
    );
    ok("print(unwrap_or(None, 0), unwrap(Ok(3)), map(Some(2), fn(x) = x * 10), is_some(Some(1)))", "0 3 Some(20) true");
    fails("unwrap(None)", "E013");
    fails("x = Some(1)?", "E016");
}

#[test]
fn values_never_change() {
    let r = fails("x = 1\nx = x + 1", "E011");
    assert!(r.stderr.contains("first defined here"));
    fails("x = 1\nx += 1", "E011");
    fails("fn f(x)\n  x = 2\n  x\nend", "E011");
    fails("p = {a: 1}\np.a = 2", "E001");
}

#[test]
fn helpful_errors() {
    let r = fails("print(lenght([1]))", "E003");
    assert!(r.stderr.contains("`len`"), "{}", r.stderr);
    let r = fails("fn square(x) = x * x\nprint(sqaure(2))", "E003");
    assert!(r.stderr.contains("did you mean `square`?"), "{}", r.stderr);
    let r = fails("for x in [1, 2]", "E001");
    assert!(r.stderr.contains("no loops"), "{}", r.stderr);
    fails("return 1", "E001");
    // `print "hi"` is a call without parentheses; a record after the name is not.
    ok("print \"hi\"", "hi");
    let r = fails("print {a: 1}", "E001");
    assert!(r.stderr.contains("print(...)"), "{}", r.stderr);
    fails("fn add(a, b) = a + b\nadd(1)", "E004");
    fails("len(1, 2)", "E004");
    let r = fails("print(\"age: \" + 36)", "E005");
    assert!(r.stderr.contains("{x}"), "{}", r.stderr);
    fails("if 1 then 2 else 3", "E005");
    fails("fn f(0) = 0\nf(1)", "E006");
    fails("[1, 2][5]", "E007");
    fails("1 / 0", "E008");
    fails("None()", "E009");
    fails("{a: 1}.b", "E010");
    fails("10 ** 100000", "E023");
    fails("assert_eq(1, 2)", "E015");
    fails("print(\"oops)", "E002");
    fails("x = if true then 1", "E019");
    fails("fn f(x)\n  x", "E020");
    fails("fail(\"stop\")", "E021");
    fails("int(\"abc\")", "E022");
    fails("y = x + 1\nx = 2", "E003");
    let r = fails("x = f()\nfn f() = y + 1\ny = 2", "E003");
    assert!(r.stderr.contains("not defined yet") && r.stderr.contains("defined later"), "{}", r.stderr);
    fails("True", "E003");
    fails("x = 1 < 2 < 3", "E001");
}

#[test]
fn errors_point_at_the_right_place() {
    let r = fails("x = 1\ny = [1, 2]\nz = y[7]", "E007");
    assert!(r.stderr.contains("<code>:3:7"), "{}", r.stderr);
    let r = fails("fn f(x) = x / 0\nfn g(x) = f(x) + 1\ng(1)", "E008");
    assert!(r.stderr.contains("in `f`") && r.stderr.contains("in `g`"), "{}", r.stderr);
}

#[test]
fn warnings_do_not_stop_the_program() {
    let r = warns("type Light = Red | Green\nfn next(Red) = Green\nprint(next(Red))", "W002");
    assert_eq!(r.stdout.trim(), "Green");
    warns("fn f(x)\n  unused = 1\n  x\nend\nprint(f(1))", "W001");
    warns("fn sum(x) = x\nprint(sum(1))", "W003");
    warns("print(match 1\n  _ -> 0\n  1 -> 1\nend)", "W004");
    warns("print(match Some(1)\n  Some(x) -> x\nend)", "W002");
    warns("print(match [1]\n  [x, ..rest] -> x + len(rest)\nend)", "W002");
}

#[test]
fn french_messages() {
    let r = common::rythme_lang("fr", &["-e", "print(lenght([1]))"]);
    assert!(r.stderr.contains("erreur[E003]: nom inconnu `lenght`"), "{}", r.stderr);
    assert!(r.stderr.contains("aide"), "{}", r.stderr);
    let r = common::rythme(&["--lang", "fr", "-e", "x = 1\nx = 2"]);
    assert!(r.stderr.contains("déjà défini"), "{}", r.stderr);
}
