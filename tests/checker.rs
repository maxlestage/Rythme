//! Mistakes found by `rythme check` before running, thanks to the types of
//! values: each one is reported before the program runs, with the same code
//! as when it runs; and correct programs that look alike are not reported.

mod common;

use common::rythme;

/// A new file for each call: the tests run at the same time.
fn temp_file(name: &str, content: &str) -> std::path::PathBuf {
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("rythme-checker-{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    path
}

/// `rythme check` finds this error (with this text in the message), without running.
#[track_caller]
fn finds(code: &str, error: &str, text: &str) -> String {
    let file = temp_file("found.ry", &format!("print(\"ran\")\n{code}\n"));
    let r = rythme(&["check", file.to_str().unwrap()]);
    assert_eq!(r.code, 1, "expected {error} for:\n{code}\n{}", r.stderr);
    assert!(!r.stdout.contains("ran"), "check must not run the program");
    assert!(r.stderr.contains(&format!("error[{error}]")), "expected {error} for:\n{code}\n{}", r.stderr);
    assert!(r.stderr.contains(text), "expected `{text}` for:\n{code}\n{}", r.stderr);
    r.stderr
}

/// `rythme check` warns (with this text), and the program still runs.
#[track_caller]
fn warns(code: &str, warning: &str, text: &str) {
    let r = rythme(&["-e", code]);
    assert_eq!(r.code, 0, "code:\n{code}\n{}", r.stderr);
    assert!(r.stderr.contains(&format!("warning[{warning}]")), "expected {warning} for:\n{code}\n{}", r.stderr);
    assert!(r.stderr.contains(text), "expected `{text}` for:\n{code}\n{}", r.stderr);
}

/// The program is checked without a single error or warning, and runs.
#[track_caller]
fn quiet(code: &str) {
    let file = temp_file("quiet.ry", code);
    let r = rythme(&[file.to_str().unwrap()]);
    assert_eq!(r.code, 0, "code:\n{code}\n{}", r.stderr);
    assert!(r.stderr.is_empty(), "code:\n{code}\n{}", r.stderr);
}

#[test]
fn operators_used_with_the_wrong_kinds_of_values() {
    let e = finds("age = 36\nprint(\"age: \" + age)", "E005", "cannot use `+` with Text and Int");
    assert!(e.contains("use braces"), "{e}");
    finds("print(1 + [2])", "E005", "cannot use `+` with Int and List");
    finds("print(-\"a\")", "E005", "cannot use `-` on Text \"a\"");
    finds("print(not 3)", "E005", "a condition must be `true` or `false`, but this is Int 3");
    finds("print(\"ab\" * 2)", "E005", "repeat(x, n)");
    finds("print({a: 1} < {a: 2})", "E005", "cannot use `<` with Record and Record");
    finds("print(true + 1)", "E005", "cannot use `+` with Bool and Int");
    finds("print(len([1]) and true)", "E005", "a condition must be");
    finds("print(1.5..3)", "E005", "a range needs whole numbers, but got Float 1.5 and Int 3");
    let e = finds("type Light = Red | Green\nprint(Red < Green)", "E005", "cannot use `<` with Light and Light");
    assert!(e.contains("give each one a number"), "{e}");
    // The exact message the program would stop with.
    let ran = rythme(&["-e", "print(\"age: \" + 36)"]);
    let checked = finds("print(\"age: \" + 36)", "E005", "Text \"age: \" + Int 36");
    let message = |s: &str| s.lines().find(|l| l.starts_with("error")).unwrap_or_default().to_string();
    assert_eq!(message(&ran.stderr), message(&checked));
}

#[test]
fn conditions_that_are_not_true_or_false() {
    let e = finds("xs = [1, 2]\nprint(if len(xs) then \"some\" else \"none\")", "E005", "a condition must be");
    assert!(e.contains("x != 0"), "{e}");
    let e = finds("fn f(x) if [x] = 1\nfn f(_) = 2\nprint(f(1))", "E005", "a condition must be");
    assert!(e.contains("is_empty"), "{e}");
}

#[test]
fn built_in_functions_given_the_wrong_kind_of_value() {
    finds("print(len(5))", "E005", "`len` expects a text, a list or a record as argument 1, but got Int 5");
    finds("print(upper(3))", "E005", "`upper` expects a text as argument 1");
    finds("print(map(5, fn(x) = x))", "E005", "`map` expects a list, an Option or a Result as argument 1");
    finds("print(map([1], 5))", "E005", "`map` expects a function as argument 2");
    finds("print(\"a,b\".split(1))", "E005", "`split` expects a text as argument 2");
    finds("n = 3\nprint(n |> upper)", "E005", "`upper` expects a text as argument 1, but got a value of type Int");
    finds("print(unwrap(3))", "E005", "an Option or a Result");
    finds("print([1, 2] |> filter(fn(x) = x * 2))", "E005", "the function given to `filter` must return true or false");
    finds("print(fold([1, 2], 0, fn(x) = x))", "E004", "`fold` gives 2 argument(s) to this function, but it takes 1");
    finds("print(pow(\"2\", 3))", "E005", "cannot use `**` with Text and Int");
}

#[test]
fn values_that_are_not_functions() {
    finds("x = 5\nprint(x(1))", "E009", "`x` is not a function");
    finds("r = {a: 1}\nprint(r.a(2))", "E009", "the field `a` is not a function");
    finds("double = fn(x) = x * 2\nprint(double(1, 2))", "E004", "`double` takes 1 argument, but 2 were given");
    finds("print((fn(a, b) = a + b)(1))", "E004", "takes 2 arguments, but 1 was given");
}

#[test]
fn fields_that_records_and_constructors_do_not_have() {
    let e = finds("ada = {name: \"Ada\", born: 1815}\nprint(ada.nmae)", "E010", "this record has no field `nmae`");
    assert!(e.contains("did you mean `name`?") && e.contains("its fields are: `name`, `born`"), "{e}");
    finds("older = {..{name: \"Ada\"}, age: 36}\nprint(older.agee)", "E010", "did you mean `age`?");
    finds("print([{a: 1}, {a: 2}] |> map(fn(r) = r.b))", "E010", "this record has no field `b`");
    finds("print({a: 1}[\"b\"])", "E010", "this record has no field `b`");
    let e = finds("type Shape = Circle(radius) | Rect(width, height)\nprint(Circle(1.0).raduis)", "E010", "`Circle` has no field `raduis`");
    assert!(e.contains("did you mean `radius`?"), "{e}");
    finds(
        "type Shape = Circle(radius) | Rect(width, height)\nfn make(n) = if n > 0 then Circle(n) else Rect(n, n)\nprint(make(1).depth)",
        "E010",
        "no variant of `Shape` has a field `depth`",
    );
    finds("print(\"abc\".upper)", "E010", "add parentheses: `.upper()`");
}

#[test]
fn patterns_that_never_match_the_value() {
    finds("print(match \"abc\"\n  [x, ..rest] -> x\nend)", "E006", "no case of this `match` handles Text \"abc\"");
    finds("[a, b] = 5\nprint(a, b)", "E006", "the value Int 5 does not match this pattern");
    finds("print([1, 2] |> map(fn([a, b]) = a + b))", "E006", "this function receives a value of type Int");
    let e = finds(
        "type Shape = Circle(r) | Rect(w, h)\nfn area(Circle(r)) = r * r\nfn area(Rect(w, h)) = w * h\nprint(area(5))",
        "E006",
        "no definition of `area` accepts these arguments: area(5)",
    );
    assert!(e.contains("does not match"), "{e}");
    warns("print(match \"abc\"\n  [x, ..rest] -> x\n  _ -> \"text\"\nend)", "W004", "a value of type Text never matches this pattern");
    warns("x = Some(1)\nprint(match len(\"a\")\n  Some(n) -> n\n  n -> n\nend, x)", "W004", "this case can never be reached");
}

#[test]
fn question_mark_on_a_value_that_is_neither_an_option_nor_a_result() {
    finds("fn f(r)\n  v = {a: r}?\n  v\nend\nprint(f(1))", "E016", "`?` needs an Option or a Result, but got a value of type Record");
}

#[test]
fn errors_in_code_that_does_not_run_every_time_are_found_too() {
    finds("fn f(x) = if x > 0 then x else \"neg: \" + 0\nprint(f(1))", "E005", "cannot use `+` with Text and Int");
    finds("fn never_called() = len(5)", "E005", "`len` expects");
}

#[test]
fn several_mistakes_are_reported_at_once() {
    let e = finds("print(len(5))\nprint(\"a\" + 1)\nprint({a: 1}.b)", "E005", "`len` expects");
    assert_eq!(e.matches("error[").count(), 3, "{e}");
}

#[test]
fn comparisons_that_are_always_false() {
    warns("x = 1\nprint(x == \"1\")", "W006", "this comparison is always false: Int and Text are never equal");
    warns("x = len(\"ab\")\nprint(x != \"2\")", "W006", "this comparison is always true");
    warns("scores = {ada: 19}\nprint(get(scores, \"ada\") == 19)", "W006", "to look inside an Option");
    warns("n = 0\nprint(n == None)", "W006", "Int and Option are never equal");
}

#[test]
fn cases_handled_twice() {
    warns(
        "x = len(\"a\")\nprint(match x\n  1 -> \"one\"\n  1 -> \"again\"\n  _ -> \"other\"\nend)",
        "W004",
        "this case already handles these values",
    );
    warns("fn f(0) = 1\nfn f(0) = 2\nfn f(_) = 3\nprint(f(0))", "W004", "this definition already accepts these arguments");
    warns("fn f([x, ..]) = x\nfn f([a, b]) = a + b\nfn f(_) = 0\nprint(f([1]))", "W004", "can never be used");
}

#[test]
fn tests_that_check_nothing() {
    let file = temp_file(
        "empty_tests.ry",
        "test \"sum\" = sum([1, 2])\ntest \"shown\"\n  print(1 + 1)\nend\ntest \"good\" = assert_eq(sum([1, 2]), 3)\n",
    );
    let r = rythme(&["check", file.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(r.stderr.matches("warning[W007]").count(), 2, "{}", r.stderr);
}

#[test]
fn results_of_functions_without_effect_that_are_lost() {
    warns("numbers = [1, 2]\npush(numbers, 4)\nprint(numbers)", "W008", "the result of `push` is lost");
    warns("numbers = [3, 1]\nnumbers.sort()\nprint(numbers)", "W008", "`sort(list)` gives a new value");
    warns("fn f(name)\n  name |> upper\n  name\nend\nprint(f(\"a\"))", "W008", "the result of `upper` is lost");
    // The last line gives the value; effects, and conversions that may stop the program, are not lost.
    quiet("fn add(xs) = push(xs, 4)\nfn f(xs)\n  print(len(xs))\n  push(xs, 1)\nend\nprint(add([1]), f([2]))");
    quiet("xs = [1, 2]\nxs |> map(fn(x) = print(x))\nunwrap(first(xs))\nint(\"42\")\neach(xs, fn(x) = push([x], x))\nprint(sort(xs))");
}

#[test]
fn assertions_that_can_never_pass() {
    let e = finds("test \"parse\" = assert_eq(parse_int(\"3\"), 3)", "E015", "this assertion always fails: Result and Int are never equal");
    assert!(e.contains("Ok(3)"), "{e}");
    finds("test \"t\" = assert_eq(len(\"ab\"), \"2\")", "E015", "Int and Text are never equal");
    quiet("test \"numbers\" = assert_eq(1, 1.0)\ntest \"same\" = assert_eq(parse_int(\"3\"), Ok(3))\nfn f(x) = x\ntest \"any\" = assert_eq(f(1), 1)");
}

#[test]
fn functions_shown_instead_of_their_value() {
    warns("fn greeting() = \"hello\"\nprint(\"{greeting}!\")", "W009", "call it: `greeting()`");
    warns("print(len)", "W009", "this shows a function, not a value");
    quiet("fn greeting() = \"hello\"\nprint(\"{greeting()}!\", greeting())\nhelp(greeting)");
}

#[test]
fn every_new_code_is_explained_in_both_languages() {
    for code in ["W006", "W007", "W008", "W009"] {
        let r = rythme(&["explain", code]);
        assert!(r.stdout.starts_with(code), "{code}");
        let r = common::rythme_lang("fr", &["explain", code]);
        assert!(r.stdout.starts_with(code), "{code} (fr)");
    }
    let r = common::rythme_lang("fr", &["-e", "print(\"âge : \" + len(\"ab\"))"]);
    assert!(r.stderr.contains("impossible d'utiliser `+` avec Text et Int"), "{}", r.stderr);
    let r = common::rythme_lang("fr", &["-e", "x = 1\nprint(x == \"1\")"]);
    assert!(r.stderr.contains("cette comparaison est toujours fausse"), "{}", r.stderr);
}

// ----- correct programs that look like mistakes ----------------------------

#[test]
fn names_that_hide_others_have_their_own_type() {
    quiet("x = 5\nfn shout(x) = upper(x) + \"!\"\nprint(shout(\"hey\"), x + 1)");
    quiet("x = 5\ny = do\n  x = \"inner\"\n  x + \"!\"\nend\nprint(y, x * 2)");
    quiet("fn f()\n  len = fn(t) = 0\n  len(5)\nend\nprint(f())");
    quiet("fn f(len) = len(3)\nprint(f(fn(n) = n + 1))");
    // A function of the program that hides a built-in one is not checked like it.
    let r = rythme(&["-e", "fn upper(n) = n * 2\nprint(upper(21), 3.upper())"]);
    assert_eq!(r.stdout.trim(), "42 6", "{}", r.stderr);
    assert!(r.stderr.contains("W003") && !r.stderr.contains("error"), "{}", r.stderr);
}

#[test]
fn parameters_and_closures_stay_unknown() {
    quiet("fn id(x) = x\nprint(id(1) + 1, id(\"a\") + \"b\", id([1]) + [2])");
    quiet("fn adder(n) = fn(v) = v + n\nadd = adder(2)\nprint(add(40), adder(\"a\")(\"b\"))");
    quiet("fn describe(v) = if type_of(v) == \"Int\" then v + 1 else v\nprint(describe(1), describe(\"a\"))");
    quiet("fn first_or(xs, d) = unwrap_or(first(xs), d)\nprint(first_or([1], 0) + 1, first_or([], \"x\") + \"y\")");
}

#[test]
fn recursive_functions_get_their_types() {
    quiet("fn fact(0) = 1\nfn fact(n) = n * fact(n - 1)\nprint(fact(5) + 0.5)");
    quiet("fn even(0) = true\nfn even(n) = odd(n - 1)\nfn odd(0) = false\nfn odd(n) = even(n - 1)\nprint(even(10) and odd(3))");
    quiet("fn countdown(0, acc) = acc\nfn countdown(n, acc) = countdown(n - 1, acc + 1)\nprint(countdown(10, 0) + 1)");
    quiet("fn wrap(0) = []\nfn wrap(n) = [wrap(n - 1)]\nprint(wrap(3), len(wrap(2)))");
    finds("fn fact(0) = 1\nfn fact(n) = n * fact(n - 1)\nprint(fact(3) + \"!\")", "E005", "cannot use `+` with Int/Float and Text");
}

#[test]
fn records_and_their_updates() {
    quiet("ada = {name: \"Ada\"}\nolder = {..ada, born: 1815}\nprint(older.born + 1, older.name)");
    quiet("fn with_age(r) = {..r, age: 36}\nprint(with_age({}).age, with_age({name: 1}).name)");
    quiet("p = put({name: \"Ada\"}, \"city\", \"London\")\nprint(p.city, p.name)");
    quiet("r = {double: fn(x) = x * 2, sum: fn(a, b) = a + b}\nprint(r.double(21), r.sum(1, 2), 21 |> r.double)");
    quiet("g = group_by([1, 2, 3], fn(x) = if is_even(x) then \"even\" else \"odd\")\nprint(g.even, g.odd)");
    quiet("rs = [{a: 1}, {a: 2, b: 3}]\nprint(map(rs, fn(r) = r.a), get(rs[1], \"b\"))");
}

#[test]
fn values_whose_kind_varies_are_not_reported() {
    quiet("x = if len(\"ab\") > 1 then 1 else \"one\"\nprint(if type_of(x) == \"Int\" then x + 1 else x + \"!\")");
    quiet("mixed = [1, \"a\", true]\nprint(map(mixed, fn(m) = type_of(m)))");
    quiet(
        "fn show(v) = match v\n  [a, ..] -> \"list {a}\"\n  \"\" -> \"empty\"\n  n -> \"{n}\"\nend\nprint(show([1]), show(\"\"), show(3))",
    );
    quiet("print(1 == 1.0, 2 < 2.5, \"a\" < \"b\", [1, 2] < [1, 3], Some(1) == None, 1 == \"1\")");
}

#[test]
fn options_results_and_question_marks() {
    quiet("fn add(a, b)\n  x = parse_int(a)?\n  y = parse_int(b)?\n  Ok(x + y)\nend\nprint(add(\"1\", \"2\"), unwrap_or(add(\"x\", \"2\"), 0) + 1)");
    quiet("o = get({a: 1}, \"a\")\nprint(o == Some(1), o != None, is_some(o), map(o, fn(v) = v + 1))");
    quiet("print(match first([1, 2])\n  Some(v) -> v + 1\n  None -> 0\nend)");
    quiet("fn g(x) if parse_int(x)? > 0 = Ok(\"pos\")\nfn g(_) = Ok(\"other\")\nprint(g(\"5\"))");
}

#[test]
fn single_constructor_types_and_unit() {
    quiet("type P = P(a, b)\nfn first_of(P(a, _)) = a\nfn first_of(_) = 0\nprint(first_of(P(1, 2)), first_of(7))");
    quiet("type P = P(a, b)\nprint(match P(1, 2)\n  P(a, b) -> a + b\nend)");
    warns("print(match ()\n  () -> 0\n  _ -> 1\nend)", "W004", "this case already handles everything");
}

#[test]
fn values_of_used_files_are_known_but_not_what_their_functions_give() {
    let dir = temp_file("lib.ry", "fn triple(x) = x * 3\norigin = {x: 0, y: 0}\nfn upper(t) = t\nloud = fn(t) = text(t)\n");
    let dir = dir.parent().unwrap();
    // `text` is defined again by the program: `loud` then gives what it gives.
    let main = "use \"lib.ry\"\nfn text(n) = n\nprint(triple(2), upper(3), loud(5) + 1, origin.x + origin.y)\n";
    std::fs::write(dir.join("main.ry"), main).unwrap();
    let r = rythme(&[dir.join("main.ry").to_str().unwrap()]);
    assert_eq!(r.stdout.trim(), "6 3 6 0", "{}", r.stderr);
    assert!(!r.stderr.contains("error"), "{}", r.stderr);
    std::fs::write(dir.join("main.ry"), "use \"lib.ry\"\nprint(origin.z)\n").unwrap();
    let r = rythme(&["check", dir.join("main.ry").to_str().unwrap()]);
    assert!(r.stderr.contains("error[E010]: this record has no field `z`"), "{}", r.stderr);
}

#[test]
fn the_interactive_mode_knows_values_can_be_defined_again() {
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
    // In the loaded file, `x` is still 5 on the first line: the new definition comes after.
    let file = temp_file("redefine.ry", "print(x + 1)\nx = \"a\"\nprint(x + \"b\")\n");
    let input = format!("x = 5\n:load {}\nx + 1\n:quit\n", file.display());
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stdout.contains("6\nab\n"), "{stdout}{stderr}");
    assert_eq!((stdout.matches("error[").count(), stderr.matches("error[").count()), (0, 1), "{stdout}{stderr}");
}
