//! The fast paths of the interpreter give the same results, and the same
//! errors, as the general ones.

mod common;

use common::{fails, ok};

#[test]
fn local_names_are_found_in_the_right_scope() {
    ok("fn f(x)\n  y = x + 1\n  g = fn(z) = fn(w) = x + y + z + w\n  g(10)(100)\nend\nprint(f(1))", "113");
    ok("fn f(f) = f + 1\nprint(f(1))", "2");
    ok("fn outer()\n  fn inner(n) = if n == 0 then 0 else n + inner(n - 1)\n  inner(4)\nend\nprint(outer())", "10");
    ok("fn outer()\n  fn inner(inner) = inner * 2\n  inner(21)\nend\nprint(outer())", "42");
    ok("fn f()\n  g = fn(n) = if n == 0 then 0 else n + g(n - 1)\n  g\nend\nprint(f()(4))", "10");
    ok("fn mk() = 1..3 |> map(fn(i) = fn(x) = x * i)\nprint(mk() |> map(fn(f) = f(10)))", "[10, 20, 30]");
    ok("x = 5\nfn f() = x + 1\nprint(f())", "6");
    ok("fn f(n)\n  a = match n\n    0 -> \"zero\"\n    k -> \"k{k}\"\n  end\n  b = match n + 1\n    k -> k\n  end\n  \"{a} {b}\"\nend\nprint(f(0), f(4))", "zero 1 k4 5");
}

#[test]
fn methods_and_messages_see_local_names() {
    // A local name hides the function of the same name.
    fails("fn f(len) = [1].len()\nprint(f(3))", "E009");
    ok("fn f(x)\n  foo = fn(a, b) = a * b\n  3.foo(x)\nend\nprint(f(4))", "12");
    let r = fails("fn f(totalx) = [1].totalz()\nprint(f(1))", "E003");
    assert!(r.stderr.contains("did you mean `totalx`?"), "{}", r.stderr);
}

#[test]
fn operations_on_whole_numbers_still_check_overflow() {
    let r = fails("fn g(x) = x\nfn f(n) = g(n + 1)\nprint(f(9223372036854775807))", "E012");
    assert!(r.stderr.contains("<code>:2:15") && r.stderr.contains("in `f`"), "{}", r.stderr);
    fails("fn f(n) = n * 2\nprint(map([9223372036854775807], f))", "E012");
    fails("print(sum([9223372036854775807, 1]))", "E012");
    ok("fn f(n) = if n < 2 then n else n - 1\nprint(f(1), f(5), f(1.5), 7 // 2, -7 // 2, -7 % 3, 7 % -3)", "1 4 1.5 3 -4 2 -2");
    ok("fn f(0) = \"zero\"\nfn f(n) = \"other\"\nprint(f(0), f(0.0), f(1))", "zero zero other");
}

#[test]
fn records_keep_their_fields_in_order() {
    ok(
        "p = {name: \"Ada\", age: 36}\nprint({..p, age: 37}, {age: 1, ..p}, {x: 0, ..p, y: 2}, p)",
        "{name: \"Ada\", age: 37} {age: 36, name: \"Ada\"} {x: 0, name: \"Ada\", age: 36, y: 2} {name: \"Ada\", age: 36}",
    );
    ok(
        "fn build(0, r) = r\nfn build(n, r) = build(n - 1, put(r, \"k{n}\", n))\nbig = build(20, {})\nprint(len(big), big.k1, big[\"k20\"], has(big, \"k21\"), keys(remove(big, \"k3\")) |> len, {..big, k1: 0}.k1)",
        "20 1 20 false 19 0",
    );
    ok("print({a: 1, b: 2} == {b: 2, a: 1}, {a: 1} == {a: 1, b: 2})", "true false");
}

#[test]
fn values_built_last_and_texts() {
    ok("type T = L | N(l, r)\nfn build(0) = L\nfn build(n) = N(build(n - 1), L)\nfn size(L) = 0\nfn size(N(l, r)) = 1 + size(l) + size(r)\nprint(size(build(1000)))", "1000");
    fails("type T = N(a, b)\nfn f(g) = g(1)\nprint(f(N))", "E004");
    ok(
        "x = 1.5\nprint(\"{1}|{-2}|{x}|{[1, \"a\"]}|{Some(\"s\")}|{()}\", join([1, \"a\", 2.5], \", \"))",
        "1|-2|1.5|[1, \"a\"]|Some(\"s\")|() 1, a, 2.5",
    );
    ok(
        "print(split(\"a€b€c\", \"€\"), replace(\"été\", \"é\", \"e\"), sort([3, 1, 2]), sort([2, 1.5, 1]))",
        "[\"a\", \"b\", \"c\"] ete [1, 2, 3] [1, 1.5, 2]",
    );
}

#[test]
fn functions_written_one_after_the_other_share_what_they_capture() {
    ok(
        "fn outer(n, k)\n  base = 100\n  fn even(0) = base + k\n  fn even(m) = odd(m - 1)\n  fn odd(0) = 0 - k\n  fn odd(m) = even(m - 1)\n  [even(n), odd(n), map([1, 2], fn(x) = odd(x) + base)]\nend\nprint(outer(10, 7), outer(11, 3))",
        "[107, -7, [207, 93]] [-3, 103, [203, 97]]",
    );
    // A function of the group used from a closure inside another one, and as a value.
    ok(
        "fn outer(n)\n  fn a(x) = (fn(y) = b(x + y))(1)\n  fn b(x) = x * n\n  fn c() = [a, b] |> map(fn(f) = f(2))\n  c()\nend\nprint(outer(10))",
        "[30, 20]",
    );
    // The parameters hide the functions of the group; tail calls between them do not grow the stack.
    ok("fn f(n)\n  fn a(b) = b + 1\n  fn b(a) = a * 10\n  [a(n), b(n)]\nend\nprint(f(4))", "[5, 40]");
    ok(
        "fn f(n)\n  fn ping(0, acc) = acc\n  fn ping(k, acc) = pong(k - 1, acc + 1)\n  fn pong(0, acc) = acc\n  fn pong(k, acc) = ping(k - 1, acc + 2)\n  ping(n, 0)\nend\nprint(f(1000000))",
        "1500000",
    );
    fails("fn f(n)\n  fn yes(0) = true\n  fn yes(k) = true and no(k - 1)\n  fn no(0) = 5\n  fn no(k) = false or yes(k - 1)\n  yes(n)\nend\nprint(f(3))", "E005");
}
