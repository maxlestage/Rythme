//! The standard library: every documented example is true, and every function
//! behaves well on the edge cases (empty values, negative positions, Unicode).

mod common;

use std::path::Path;

use common::{fails, ok};
use rythme::builtins::BUILTINS;

/// Functions whose examples have effects (output, files, randomness, time...):
/// their examples are shown, not checked.
const EFFECTS: &[&str] = &[
    "print",
    "debug",
    "each",
    "input",
    "help",
    "fail",
    "args",
    "exit",
    "time",
    "random",
    "shuffle",
    "env",
    "read_file",
    "write_file",
    "append_file",
    "file_exists",
    "now",
    "today",
];

/// Runs a program inside this process and returns what it printed, or the error.
fn run(code: &str) -> Result<String, String> {
    use rythme::driver::Driver;
    use rythme::interp::Mode;
    rythme::lang::set(rythme::lang::Lang::En);
    rythme::io::capture();
    let mut driver = Driver::new();
    driver.color = false;
    let ok = match driver.load_source("example.ry", code, None, Path::new(".")) {
        Some(m) => driver.execute(&m, Mode::Run).is_ok(),
        None => false,
    };
    let out = rythme::io::take_captured();
    if ok && !out.contains(rythme::io::ERR_START) {
        Ok(out)
    } else {
        Err(out.replace([rythme::io::ERR_START, rythme::io::ERR_END], ""))
    }
}

/// Does `got` match `expected`, where `...` in `expected` stands for any text?
fn matches(got: &str, expected: &str) -> bool {
    let parts: Vec<&str> = expected.split("...").collect();
    if parts.len() == 1 {
        return got == expected;
    }
    let mut rest = got;
    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            let Some(r) = rest.strip_prefix(part) else { return false };
            rest = r;
        } else if i == parts.len() - 1 {
            return rest.ends_with(part);
        } else {
            let Some(at) = rest.find(part) else { return false };
            rest = &rest[at + part.len()..];
        }
    }
    true
}

/// Every line `code   # value` of an example: `code` gives exactly `value`.
#[test]
fn every_example_is_true() {
    let mut failures = Vec::new();
    for b in BUILTINS.iter().filter(|b| !EFFECTS.contains(&b.name)) {
        let mut program = String::new();
        let mut expected = Vec::new();
        for line in b.example.lines() {
            match line.split_once("  # ") {
                Some((code, value)) => {
                    program.push_str(&format!("print([{}])\n", code.trim()));
                    expected.push(format!("[{}]", value.trim()));
                }
                None => {
                    program.push_str(line);
                    program.push('\n');
                }
            }
        }
        match run(&program) {
            Ok(out) => {
                let got: Vec<&str> = out.lines().collect();
                if got.len() != expected.len() || got.iter().zip(&expected).any(|(g, e)| !matches(g, e)) {
                    failures.push(format!("{}:\n  expected {expected:?}\n  got      {got:?}", b.name));
                }
            }
            Err(e) => failures.push(format!("{}: the example fails:\n{e}", b.name)),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The signature names as many arguments as the function accepts. It may show
/// several forms, like `random(a, b), random(list)`: the longest one counts.
#[test]
fn signatures_match_the_number_of_arguments() {
    for b in BUILTINS {
        assert!(b.min <= b.max, "{}", b.name);
        let forms: Vec<Vec<&str>> = b
            .sig
            .split("), ")
            .map(|form| {
                let inside = form.strip_prefix(b.name).and_then(|s| s.strip_prefix('(')).map(|s| s.strip_suffix(')').unwrap_or(s));
                inside.unwrap_or_else(|| panic!("{}: {}", b.name, b.sig)).split(", ").filter(|p| !p.is_empty()).collect()
            })
            .collect();
        let params = forms.iter().max_by_key(|f| f.len()).unwrap();
        if b.max == usize::MAX {
            assert_eq!(params.last(), Some(&"..."), "{}", b.name);
        } else {
            assert_eq!(params.len(), b.max, "{}: {}", b.name, b.sig);
        }
    }
}

/// The names that the editors, the site and the playground color as built-in.
fn listed(file: &str) -> Vec<String> {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(file)).unwrap();
    let names: Vec<String> = if file.ends_with(".rs") {
        let start = text.find("pub const BUILTINS").expect("BUILTINS in highlight.rs");
        let block = &text[start..start + text[start..].find("];").unwrap()];
        block.lines().filter_map(|l| l.trim().strip_prefix('"')?.strip_suffix("\",").map(str::to_owned)).collect()
    } else {
        let start = text.find("const BUILTINS = new Set((\"").expect("BUILTINS in the page") + 27;
        text[start..start + text[start..].find('"').unwrap()].split(' ').map(str::to_owned).collect()
    };
    names.into_iter().filter(|n| n != "pi").collect()
}

#[test]
fn every_builtin_is_colored_by_the_site_and_the_playground() {
    let expected: Vec<String> = BUILTINS.iter().map(|b| b.name.to_owned()).collect();
    for file in ["site/src/highlight.rs", "site/public/playground.js", "playground/web/index.html"] {
        assert_eq!(listed(file), expected, "{file} must list the built-in functions in the order of BUILTINS");
    }
}

#[test]
fn numbers() {
    ok("print(gcd(12, 18), gcd(-12, 18), gcd(0, 0), gcd(7, 0), lcm(4, 6), lcm(-4, 6), lcm(0, 5))", "6 6 0 7 12 12 0");
    ok("print(lcm(9223372036854775807, 2), gcd(-9223372036854775807 - 1, 0))", "18446744073709551614 9223372036854775808");
    fails("gcd(1.5, 2)", "E005");
    ok("print(asin(1) == pi / 2, acos(1), atan(1) == pi / 4, atan(0, -1) == pi, atan(-1, 0) == -pi / 2)", "true 0.0 true true true");
    fails("asin(2)", "E023");
    fails("acos(-1.5)", "E023");
    ok("print(round(1e300, 15), round(1234.5678, -2), round(-2.5), round(2.5, 0))", "1e300 1200.0 -3 3.0");
}

#[test]
fn whole_numbers_without_limit() {
    // Known values.
    ok("print(2 ** 100, 3 ** 100)", "1267650600228229401496703205376 515377520732011331036461129765621272702107522001");
    ok(
        "print(product(1..100))",
        "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000",
    );
    ok("fn fib(n, a, b) = if n == 0 then a else fib(n - 1, b, a + b)\nprint(fib(100, 0, 1))", "354224848179261915075");
    ok(
        "print(2 ** 127 - 1, pow(2, 64), pow(-2, 65))",
        "170141183460469231731687303715884105727 18446744073709551616 -36893488147419103232",
    );
    ok(
        "print(10 ** 30 // 7, 10 ** 30 % 7, -(10 ** 30) // 7, -(10 ** 30) % 7)",
        "142857142857142857142857142857 1 -142857142857142857142857142858 6",
    );
    ok("print(10 ** 30 % -7, (2 ** 100) // (2 ** 98), 2 ** 100 / 2 ** 99, 10 ** 400 / 10 ** 399, 1 / 10 ** 400)", "-6 4 2.0 10.0 0.0");
    ok(
        "print(gcd(2 ** 100, 10 ** 30), gcd(-(6 ** 40), 4 ** 30), lcm(2 ** 70, 3), lcm(-(10 ** 20), 0))",
        "1073741824 1099511627776 3541774862152233910272 0",
    );
    ok(
        "print(abs(-(10 ** 20)), -(10 ** 20), is_even(10 ** 20), is_odd(10 ** 20 + 1))",
        "100000000000000000000 -100000000000000000000 true true",
    );
    // Exact comparisons with decimal numbers.
    ok("print(2 ** 64 == 2.0 ** 64, 2 ** 64 + 1 == 2.0 ** 64, 2 ** 64 + 1 > 2.0 ** 64, 10 ** 400 < 1e308 * 10.0, -(10 ** 400) > -1e308 * 10.0, 10 ** 400 > 1e308)", "true false true true true true");
    ok(
        "print(max(2 ** 70, 1e300), min([3, -(2 ** 70), 0.5]), sort([2 ** 65, 1.5, -(2 ** 65)]))",
        "1e300 -1180591620717411303424 [-36893488147419103232, 1.5, 36893488147419103232]",
    );
    ok(
        "print(2 ** 70 + 0.5, 2 ** 70 * 1.0, float(2 ** 70), 2 ** 1100 * 1.0)",
        "1.1805916207174113e21 1.1805916207174113e21 1.1805916207174113e21 inf",
    );
    // Converted exactly, both ways.
    ok("print(int(1e30), floor(-1e20), round(2.0 ** 70))", "1000000000000000019884624838656 -100000000000000000000 1180591620717411303424");
    ok(
        "print(int(\"123456789012345678901234567890\"), int(\" -1_000_000_000_000_000_000_000 \"))",
        "123456789012345678901234567890 -1000000000000000000000",
    );
    ok(
        "print(parse_int(\"99999999999999999999\"), text(2 ** 70), text(2 ** 70, 2), \"{2 ** 70}\")",
        "Ok(99999999999999999999) 1180591620717411303424 1180591620717411303424.00 1180591620717411303424",
    );
    ok(
        "print(to_json({n: 2 ** 70}), parse_json(\"[123456789012345678901234567890, -1e400]\"))",
        "{\"n\": 1180591620717411303424} Ok([123456789012345678901234567890, -inf])",
    );
    ok(
        "print(unique([2 ** 70, 2.0 ** 70, 2 ** 70 + 1]), type_of(2 ** 70), 2 ** 70 == 1180591620717411303424)",
        "[1180591620717411303424, 1180591620717411303425] Int true",
    );
    ok("print(match 2 ** 70\n  1180591620717411303424 -> \"yes\"\n  _ -> \"no\"\nend, match -(2 ** 70)\n  -1180591620717411303424 -> \"yes\"\n  _ -> \"no\"\nend)", "yes yes");
    ok(
        "print(10 ** 20 .. 10 ** 20 + 2, len(text(10 ** 99999)), (-1) ** (10 ** 30), 1 ** (10 ** 30), 0 ** (10 ** 30))",
        "[100000000000000000000, 100000000000000000001, 100000000000000000002] 100000 1 1 0",
    );
    // Only absurdly large numbers stop: more than 100000 digits.
    fails("10 ** 100000", "E023");
    fails("2 ** (10 ** 30)", "E023");
    fails("x = 10 ** 60000\nx * x", "E023");
    fails("int(\"1\" + repeat(\"0\", 100000))", "E023");
    ok("print(parse_int(\"1\" + repeat(\"0\", 100000)))", "Err(\"this number has more than 100000 digits\")");
    fails("int(1e308 * 10.0)", "E022");
    fails("[1, 2][10 ** 20]", "E007");
    fails("take([1], 10 ** 20)", "E023");
    fails("2 ** 70 // 0", "E008");
    fails("0 ** -(10 ** 20)", "E008");
}

#[test]
fn numbers_as_text() {
    ok("print(text(3.14159, 2), text(5, 2), text(-7, 0), text(2.5, 0), text(0.125, 2), text(-0.001, 2))", "3.14 5.00 -7 3 0.13 0.00");
    ok(
        "print(text(9007199254740993, 1), text(1e20, 1), text(float(\"inf\"), 2), text(0.1 + 0.2, 1))",
        "9007199254740993.0 100000000000000000000.0 inf 0.3",
    );
    fails("text(\"a\", 2)", "E005");
    fails("text(1.5, -1)", "E023");
    fails("text(1.5, 101)", "E023");
}

#[test]
fn text_functions() {
    ok("print(capitalize(\"élise\"), capitalize(\"\"), capitalize(\"ßa\"), capitalize(\"1abc\"))", "Élise  SSa 1abc");
    ok(
        "print(pad_left(\"été\", 5, \"*\"), pad_right(3.5, 6, \"_\"), pad_left(\"long\", 2), pad_left(\"\", 0) == \"\")",
        "**été 3.5___ long true",
    );
    ok("print(\"[{pad_left(42, 5)}]\", \"[{pad_right(\"ab\", 4)}]\")", "[   42] [ab  ]");
    fails("pad_left(\"a\", 3, \"ab\")", "E023");
    fails("pad_left(\"a\", -1)", "E023");
    ok(
        "print(char_code(\"A\"), char_code(\"é\"), char_code(\"😀\"), char(233), char(128512), char(char_code(\"y\") + 1))",
        "65 233 128512 é 😀 z",
    );
    fails("char_code(\"ab\")", "E023");
    fails("char_code(\"\")", "E023");
    fails("char(55296)", "E023");
    fails("char(-1)", "E023");
    ok("print(is_digit(\"042\"), is_digit(\"\"), is_digit(\"4a\"), is_digit(\"٣\"), is_digit(\"-1\"))", "true false false false false");
    ok("print(is_letter(\"Ωmega\"), is_letter(\"été\"), is_letter(\"a1\"), is_letter(\"\"))", "true true false false");
    ok("print(is_space(\" \\t\\n\"), is_space(\"\"), is_space(\" a \"))", "true false false");
}

#[test]
fn list_changes_make_new_lists() {
    ok("xs = [1, 2, 3]\nprint(set_at(xs, 0, 9), set_at(xs, -1, 0), xs)", "[9, 2, 3] [1, 2, 0] [1, 2, 3]");
    ok(
        "print(insert_at([1, 2], 0, 0), insert_at([1, 2], 2, 3), insert_at([1, 2], -1, 3), insert_at([1, 2], -3, 0), insert_at([], 0, 1))",
        "[0, 1, 2] [1, 2, 3] [1, 2, 3] [0, 1, 2] [1]",
    );
    ok("print(remove_at([1, 2, 3], 0), remove_at([1, 2, 3], -1), remove_at([1], 0))", "[2, 3] [1, 2] []");
    let r = fails("set_at([1, 2], 2, 0)", "E007");
    assert!(r.stderr.contains("from 0 to 1"), "{}", r.stderr);
    fails("insert_at([1, 2], 3, 0)", "E007");
    fails("insert_at([1, 2], -4, 0)", "E007");
    fails("remove_at([], 0)", "E007");
    fails("set_at(\"abc\", 0, \"x\")", "E005");
}

#[test]
fn pieces_of_lists_and_texts() {
    ok(
        "print(take_while([1, 3, 4, 5], is_odd), drop_while([1, 3, 4, 5], is_odd), take_while([], is_odd), drop_while([1, 3], is_odd))",
        "[1, 3] [4, 5] [] []",
    );
    ok("print(take_while(\"42 cm\", is_digit), drop_while(\"  été\", is_space), take_while(\"\", is_digit) == \"\")", "42 été true");
    ok("print(0..10 |> step_by(5), step_by(\"abcdef\", 2), step_by([], 3), step_by(1..3, 9))", "[0, 5, 10] ace [] [1]");
    ok(
        "print(chunks([1, 2, 3, 4, 5], 2), chunks(\"héllo\", 2), chunks([], 3), chunks([1], 4))",
        "[[1, 2], [3, 4], [5]] [\"hé\", \"ll\", \"o\"] [] [[1]]",
    );
    ok(
        "print(windows([1, 2, 3, 4], 2), windows(\"abc\", 2), windows([1, 2], 3), windows([1, 2], 2))",
        "[[1, 2], [2, 3], [3, 4]] [\"ab\", \"bc\"] [] [[1, 2]]",
    );
    fails("step_by([1], 0)", "E023");
    fails("chunks([1], 0)", "E023");
    fails("windows(\"abc\", -1)", "E023");
    fails("take_while(5, is_odd)", "E005");
    fails("take_while([1, 2], fn(x) = x)", "E005");
}

#[test]
fn folding_and_choosing() {
    ok(
        "print(scan([1, 2, 3], 0, fn(total, x) = total + x), scan([], 0, fn(a, x) = a + x), scan([2, 3], 1, fn(a, x) = a * x))",
        "[1, 3, 6] [] [2, 6]",
    );
    ok("print(min_by([\"ccc\", \"a\", \"bb\"], len), max_by([\"bb\", \"aa\", \"c\"], len), max_by([3], fn(x) = -x))", "a bb 3");
    ok(
        "people = [{n: \"Ada\", age: 36}, {n: \"Alan\", age: 41}]\nprint(max_by(people, fn(p) = p.age).n, min_by(people, fn(p) = p.age).n)",
        "Alan Ada",
    );
    fails("min_by([], len)", "E023");
    fails("max_by([1, \"a\"], fn(x) = x)", "E005");
    ok("print(len(shuffle(1..50)), sort(shuffle(1..50)) == 1..50, shuffle([]))", "50 true []");
}

#[test]
fn records_as_dictionaries() {
    ok(
        "print(from_entries([[\"a\", 1], [\"b c\", 2]]), from_entries([]), from_entries([[\"a\", 1], [\"a\", 2]]))",
        "{a: 1, \"b c\": 2} {} {a: 2}",
    );
    ok(
        "r = {x: 1, y: 2}\nprint(from_entries(entries(r)) == r, map_values(r, fn(v) = v * 10), map_values({}, upper))",
        "true {x: 10, y: 20} {}",
    );
    ok("print(group_by(split(\"a b a c a\"), fn(w) = w) |> map_values(len))", "{a: 3, b: 1, c: 1}");
    fails("from_entries([[1, 2]])", "E023");
    fails("from_entries([[\"a\"]])", "E023");
    let r = fails("put([1, 2], 0, 5)", "E005");
    assert!(r.stderr.contains("set_at"), "{}", r.stderr);
    let r = fails("remove([1, 2], 0)", "E005");
    assert!(r.stderr.contains("remove_at"), "{}", r.stderr);
}

#[test]
fn option_and_result_helpers() {
    ok(
        "print(and_then(Some(2), fn(x) = Some(x * 2)), and_then(Some(2), fn(x) = None), and_then(None, fn(x) = Some(x)))",
        "Some(4) None None",
    );
    ok(
        "print(and_then(Ok(2), fn(x) = Err(\"no\")), and_then(Err(\"e\"), fn(x) = Ok(x)), parse_int(\"7\") |> and_then(fn(n) = Ok(n + 1)))",
        "Err(\"no\") Err(\"e\") Ok(8)",
    );
    let r = fails("and_then(Some(1), fn(x) = x + 1)", "E005");
    assert!(r.stderr.contains("map(x, f)"), "{}", r.stderr);
    fails("and_then(Ok(1), fn(x) = Some(x))", "E005");
    fails("and_then([1], fn(x) = Some(x))", "E005");
    ok(
        "print(ok_or(Some(1), \"e\"), ok_or(None, \"missing\"), map_err(Ok(1), upper), map_err(Err(\"e\"), upper))",
        "Ok(1) Err(\"missing\") Ok(1) Err(\"E\")",
    );
    ok("fn age(p)\n  a = ok_or(get(p, \"age\"), \"no age\")?\n  Ok(a + 1)\nend\nprint(age({age: 36}), age({}))", "Ok(37) Err(\"no age\")");
    fails("ok_or(Ok(1), \"e\")", "E005");
    fails("map_err(None, upper)", "E005");
    let r = fails("flat_map(Some(1), fn(x) = Some(x))", "E005");
    assert!(r.stderr.contains("and_then"), "{}", r.stderr);
}

#[test]
fn json_reading() {
    ok("print(parse_json(\"[1, 2.5, -3e2, true, false, null, \\\"x\\\"]\"))", "Ok([1, 2.5, -300.0, true, false, None, \"x\"])");
    ok("print(parse_json(\" \\{\\\"a\\\": \\{\\\"b\\\": [\\{\\}, []]\\}, \\\"c d\\\": 1\\} \"))", "Ok({a: {b: [{}, []]}, \"c d\": 1})");
    ok("print(parse_json(\"\\\"\\\\u00e9\\\\ud83d\\\\ude00\\\\n\\\\\\\"\\\"\") |> unwrap |> len)", "4");
    ok(
        "print(parse_json(\"12345678901234567890\"), parse_json(\"-0\"), parse_json(\"\\u{feff}1\"))",
        "Ok(12345678901234567890) Ok(0) Ok(1)",
    );
    for (json, error) in [
        ("", "line 1, column 1: expected a value"),
        ("[1, 2", "expected `,` or `]`"),
        ("[1,]", "expected a value, but found `]`"),
        ("01", "invalid number"),
        ("1.", "invalid number"),
        ("[1] x", "expected the end of the text, but found `x`"),
        ("tru", "expected a value"),
        ("\\\"a", "never closed"),
        ("\\\"\\\\x\\\"", "invalid escape"),
        ("\\\"\\\\ud800\\\"", "surrogate"),
        ("[\\n1,\\n2,\\n?]", "line 4, column 1"),
    ] {
        let r = common::rythme(&["-e", &format!("print(parse_json(\"{json}\"))")]);
        assert!(r.stdout.starts_with("Err(\"JSON, ") && r.stdout.contains(error), "{json}: {}{}", r.stdout, r.stderr);
    }
    fails("parse_json(42)", "E005");
}

#[test]
fn json_writing() {
    ok(
        "print(to_json({name: \"Ada\", tags: [\"a\"], ok: true, none: None, some: Some(2), x: 1e20, y: 0.5, z: -3}))",
        "{\"name\": \"Ada\", \"tags\": [\"a\"], \"ok\": true, \"none\": null, \"some\": 2, \"x\": 1e20, \"y\": 0.5, \"z\": -3}",
    );
    ok("print(to_json(\"say \\\"hi\\\"\\n\\t\\\\ \\{é}\"))", "\"say \\\"hi\\\"\\n\\t\\\\ {é}\"");
    ok("print(to_json([[], {}, [1, [2]]], 2))", "[\n  [],\n  {},\n  [\n    1,\n    [\n      2\n    ]\n  ]\n]");
    ok("print(to_json({a: [1]}, 0))", "{\n\"a\": [\n1\n]\n}");
    ok(
        "v = {a: [1, 2.5, \"x\", None, {b: true}], c: \"é\"}\nprint(parse_json(to_json(v)) == Ok(v), parse_json(to_json(v, 4)) == Ok(v))",
        "true true",
    );
    fails("to_json(print)", "E005");
    fails("to_json(Ok(1))", "E005");
    fails("to_json([1e308 * 10])", "E023");
    fails("to_json(1, 17)", "E023");
}

#[test]
fn json_nested_very_deeply_does_not_crash() {
    let depth = 200_000;
    let code = format!("j = repeat(\"[\", {depth}) + repeat(\"]\", {depth})\nv = unwrap(parse_json(j))\nprint(to_json(v) == j)");
    ok(&code, "true");
    // Indented, this would take tens of gigabytes: an error, not a crash.
    fails(&format!("j = repeat(\"[\", {depth}) + repeat(\"]\", {depth})\nto_json(unwrap(parse_json(j)), 2)"), "E023");
}

#[test]
fn env_and_files() {
    ok("print(env(\"RYTHME_SURELY_NOT_SET\"), env(\"\"), env(\"A=B\"))", "None None None");
    let r = common::rythme(&["-e", "print(env(\"PATH\") |> is_some)"]);
    assert_eq!(r.stdout.trim(), "true");
    let dir = std::env::temp_dir().join(format!("rythme-stdlib-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("log.txt");
    let path = path.to_str().unwrap().replace('\\', "/");
    ok(
        &format!("p = \"{path}\"\nwrite_file(p, \"a\\n\")\nappend_file(p, \"b\\n\")\nappend_file(p, 3)\nprint(read_file(p))"),
        "Ok(\"a\\nb\\n3\")",
    );
    ok(&format!("print(is_err(append_file(\"{path}/nowhere/x.txt\", \"a\")))"), "true");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn wrong_values_get_a_hint() {
    let r = fails("map(\"abc\", upper)", "E005");
    assert!(r.stderr.contains("chars(text)"), "{}", r.stderr);
    let r = fails("filter({a: 1}, fn(x) = true)", "E005");
    assert!(r.stderr.contains("entries(record)"), "{}", r.stderr);
    let r = fails("xs = [1]\nxs[0] = 2", "E001");
    assert!(r.stderr.contains("set_at"), "{}", r.stderr);
    let r = common::rythme_lang("fr", &["-e", "char_code(\"ab\")"]);
    assert!(r.stderr.contains("a besoin d'un seul caractère"), "{}", r.stderr);
}

#[test]
fn names_from_other_languages() {
    for (name, rythme) in [
        ("padStart", "pad_left"),
        ("rjust", "pad_left"),
        ("ljust", "pad_right"),
        ("ord", "char_code"),
        ("chr", "char"),
        ("isdigit", "is_digit"),
        ("takeWhile", "take_while"),
        ("each_slice", "chunks"),
        ("pairwise", "windows"),
        ("accumulate", "scan"),
        ("maxBy", "max_by"),
        ("insert", "insert_at"),
        ("pop", "remove_at"),
        ("fromEntries", "from_entries"),
        ("mapValues", "map_values"),
        ("andThen", "and_then"),
        ("mapErr", "map_err"),
        ("stringify", "to_json"),
        ("json_parse", "parse_json"),
        ("getenv", "env"),
        ("startsWith", "starts_with"),
        ("pgcd", "gcd"),
    ] {
        let r = fails(&format!("{name}(1)"), "E003");
        assert!(r.stderr.contains(&format!("called `{rythme}`")), "{name}: {}", r.stderr);
    }
    for (name, hint) in [("mod", "`%`"), ("div", "`//`"), ("toFixed", "text(x, 2)"), ("words", "split(text)"), ("range", "step_by")] {
        let r = fails(&format!("{name}(1)"), "E003");
        assert!(r.stderr.contains(hint), "{name}: {}", r.stderr);
    }
}

#[test]
fn help_on_a_value_lists_each_function_once() {
    let r = common::rythme(&["-e", "help([1, 2])"]);
    let names: Vec<&str> = r.stdout.lines().filter_map(|l| l.split_once(": ").map(|p| p.1)).flat_map(|l| l.split(", ")).collect();
    for name in ["map", "chunks", "len", "join"] {
        assert_eq!(names.iter().filter(|n| **n == name).count(), 1, "{name}: {}", r.stdout);
    }
    let r = common::rythme(&["-e", "help({a: 1})"]);
    assert!(r.stdout.contains("map_values") && r.stdout.contains("to_json"), "{}", r.stdout);
}
