//! Bugs found by reviewing the interpreter: each test keeps one of them fixed.

mod common;

use std::io::Write;
use std::process::{Command, Stdio};

use common::{fails, ok, rythme, warns};

fn temp_file(name: &str, content: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-regressions-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    path
}

fn repl(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .env("HOME", std::env::temp_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr)
}

// ----- crashes and hangs -------------------------------------------------

#[test]
fn sorting_mixed_values_is_an_error_not_a_crash() {
    let mut items: Vec<String> = (0..30).map(|i| ((i * 37) % 101).to_string()).collect();
    items[8] = "\"x\"".into();
    fails(&format!("print(sort([{}]))", items.join(", ")), "E005");
    items[8] = "float(\"nan\")".into();
    fails(&format!("print(sort([{}]))", items.join(", ")), "E005");
    fails(
        &format!(
            "print(sort_by([{}], fn(x) = if x == 5 then \"a\" else x))",
            (0..30).map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
        ),
        "E005",
    );
    ok(
        "print(sort([3, 1.5, 2]), sort([[2, 1], [1, 5], [1, 2]]), sort([\"b\", \"a\"]))",
        "[1.5, 2, 3] [[1, 2], [1, 5], [2, 1]] [\"a\", \"b\"]",
    );
}

#[test]
fn very_deep_values_can_be_built_shown_compared_and_freed() {
    ok(
        "type L = Nil | Cons(head, tail)\nfn build(0, acc) = acc\nfn build(n, acc) = build(n - 1, Cons(n, acc))\nxs = build(300000, Nil)\nys = build(300000, Nil)\nprint(len(text(xs)), xs == ys, len(unique([xs, ys])))",
        "4088898 true 1",
    );
    ok("fn b(0, acc) = acc\nfn b(n, acc) = b(n - 1, [n, acc])\nprint(len(text(b(300000, []))))", "2888897");
}

#[test]
fn a_closed_output_stops_the_program_quietly() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["-e", "1..2000000 |> each(print)"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut first = [0u8; 4];
    std::io::Read::read_exact(child.stdout.as_mut().unwrap(), &mut first).unwrap();
    drop(child.stdout.take());
    let out = child.wait_with_output().unwrap();
    assert!(!String::from_utf8_lossy(&out.stderr).contains("internal problem"));
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn repeating_an_empty_list_a_huge_number_of_times_is_instant() {
    ok("print(repeat([], 9223372036854775807), len(repeat(\"\", 9223372036854775807)))", "[] 0");
}

#[test]
fn absurdly_nested_code_is_refused() {
    let deep = format!("x = {}{}", "[".repeat(100_000), "]".repeat(100_000));
    let r = rythme(&["check", temp_file("deep.ry", &deep).to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("nested too deeply"), "{}", r.stderr);
    let r = rythme(&["-e", &format!("x = {}1{}", "(".repeat(300), ")".repeat(300))]);
    assert!(r.stderr.contains("nested too deeply"), "{}", r.stderr);
}

// ----- wrong results -----------------------------------------------------

#[test]
fn a_test_stopped_by_question_mark_or_ending_with_err_fails() {
    let file = temp_file(
        "tests_q.ry",
        "test \"q\"\n  x = parse_int(\"abc\")?\n  assert_eq(x, 1)\nend\ntest \"err\" = Err(\"bad\")\ntest \"none\" = None\ntest \"ok\" = assert_eq(1, 1)\n",
    );
    let r = rythme(&["test", file.to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("1 passed, 3 failed"), "{}", r.stdout);
}

#[test]
fn a_line_starting_with_minus_is_not_silently_dropped() {
    let r = fails("fn net(gross, tax)\n  result = gross\n    + 100\n    - tax\n  result\nend\nprint(net(1000, 300))", "E024");
    assert!(r.stderr.contains("starts with `-`"), "{}", r.stderr);
    fails("total = 10\n- 3\nprint(total)", "E024");
    fails("fn f(x)\n  x + 1\n  x\nend", "E024");
    ok("fn neg(y)\n  z = y * 2\n  -z\nend\nprint(neg(2))\nprint(\"a\")\n1 + 1", "-4\na\n2");
}

#[test]
fn question_mark_index_and_field_apply_to_the_result_of_a_pipe_step() {
    ok("fn parse(s)\n  n = s |> trim |> parse_int?\n  Ok(n * 2)\nend\nprint(parse(\" 21 \"), parse(\"x\") |> is_err)", "Ok(42) true");
    ok("fn f(s) = Ok((s |> trim |> parse_int()?) + 1)\nprint(f(\" 1 \"))", "Ok(2)");
    let r = fails("fn f(s) = Ok(s |> trim |> parse_int()? + 1)", "E001");
    assert!(r.stderr.contains("in parentheses"), "{}", r.stderr);
    ok("print([3, 1, 2] |> sort()[0], [3, 1, 2] |> sort |> reverse()[0], {a: {b: 7}} |> fn(r) = r.a |> fn(x) = x.b)", "1 3 7");
    ok("r = {double: fn(x) = x * 2}\nprint(21 |> r.double)", "42");
}

#[test]
fn unique_uses_the_same_equality_as_double_equals() {
    ok(
        "print(len(unique([fn(x) = x + 1, fn(x) = x * 100])), unique([1, 1.0, 2]), unique([{a: 1, b: 2}, {b: 2, a: 1}]))",
        "2 [1, 2] [{a: 1, b: 2}]",
    );
}

#[test]
fn calls_on_the_right_of_and_or_are_tail_calls() {
    ok(
        "fn all_pos([]) = true\nfn all_pos([x, ..rest]) = x > 0 and all_pos(rest)\nprint(all_pos(1..300000), all_pos([1, -1, 2]))",
        "true false",
    );
    ok("fn any_neg([]) = false\nfn any_neg([x, ..rest]) = x < 0 or any_neg(rest)\nprint(any_neg(1..300000))", "false");
}

#[test]
fn whole_number_edge_cases() {
    ok(
        "print((-9223372036854775807 - 1) % -1, 1 ** 5000000000, 0 ** 5000000000, (-1) ** 5000000001, -9223372036854775808)",
        "0 1 0 -1 -9223372036854775808",
    );
    fails("print(9223372036854775808)", "E012");
    fails("exit(256)", "E023");
}

// ----- valid programs that were rejected ----------------------------------

#[test]
fn record_fields_can_be_called_even_when_a_function_has_the_same_name() {
    ok("calc = {sum: fn(a, b) = a + b}\nprint(calc.sum(1, 2))", "3");
    ok("fn helper(a) = a\nobj = {helper: fn(a, b) = a * b}\nprint(obj.helper(6, 7), helper(1))", "42 1");
}

#[test]
fn nested_patterns_count_as_handled() {
    let r = rythme(&["-e", "print(match Some(Some(3))\n  Some(Some(n)) -> n\n  Some(None) -> 0\n  None -> -1\nend)"]);
    assert!(r.stderr.is_empty(), "{}", r.stderr);
    let r = rythme(&["-e", "print(match Ok([1])\n  Ok([first, ..]) -> first\n  Ok([]) -> 0\n  Err(_) -> -1\nend)"]);
    assert!(r.stderr.is_empty(), "{}", r.stderr);
    let r = warns("print(match Some(Some(3))\n  Some(Some(n)) -> n\n  None -> -1\nend)", "W002");
    assert!(r.stderr.contains("Some(None)"), "{}", r.stderr);
}

#[test]
fn record_and_unit_patterns_do_not_match_everything() {
    let r = rythme(&["-e", "fn d({name}) = name\nfn d(_) = \"anonymous\"\nprint(d({id: 3}), d(42), d({name: \"Ada\"}))"]);
    assert_eq!(r.stdout.trim(), "anonymous anonymous Ada");
    assert!(r.stderr.is_empty(), "{}", r.stderr);
    let r = rythme(&["-e", "fn f(x) = match x\n  () -> 0\n  _ -> 1\nend\nprint(f(()), f(1))"]);
    assert_eq!(r.stdout.trim(), "0 1");
    assert!(r.stderr.is_empty(), "{}", r.stderr);
    // A single-constructor pattern only accepts everything when the value is of its type.
    let r = rythme(&["-e", "type P = P(a, b)\nfn f(P(a, b)) = a + b\nfn f(_) = 0\nprint(f(P(1, 2)), f(5))"]);
    assert_eq!(r.stdout.trim(), "3 0");
    assert!(r.stderr.is_empty(), "{}", r.stderr);
}

#[test]
fn keywords_of_other_languages_can_be_field_names() {
    ok("s = {name: \"Ada\", class: \"3A\", for: 1}\nprint(s.class, s[\"for\"], match s\n  {class: c} -> c\nend)", "3A 1 3A");
    fails("x = {class}", "E001");
}

#[test]
fn record_patterns_accept_text_and_uppercase_keys() {
    ok("r = {\"my key\": 1, Name: 2}\nprint(match r\n  {\"my key\": v, Name: w} -> v + w\nend)", "3");
}

#[test]
fn files_may_start_with_a_byte_order_mark() {
    let file = temp_file("bom.ry", "\u{feff}print(\"ok\")\n");
    let r = rythme(&[file.to_str().unwrap()]);
    assert_eq!(r.stdout, "ok\n", "{}", r.stderr);
}

#[test]
fn a_match_can_be_written_on_several_lines_inside_a_text() {
    ok("x = 3\nprint(\"{match x\n  3 -> \"three\"\n  _ -> \"other\"\nend}\")", "three");
}

// ----- mistakes the checker now finds -------------------------------------

#[test]
fn local_functions_are_checked_before_running() {
    let r = rythme(&[
        "check",
        temp_file("local.ry", "fn f()\n  fn g(a) = a\n  h = fn(a, b) = a + b\n  g(1, 2) + h(1)\nend\n").to_str().unwrap(),
    ]);
    assert_eq!(r.code, 1);
    assert_eq!(r.stderr.matches("error[E004]").count(), 2, "{}", r.stderr);
}

#[test]
fn misspelled_method_names_are_reported_before_running() {
    ok("print([1, 2].len())", "2");
    let r = rythme(&["check", temp_file("method.ry", "print([1, 2].lenght())\n").to_str().unwrap()]);
    assert!(r.stderr.contains("warning[W005]") && r.stderr.contains("`len`"), "{}", r.stderr);
}

#[test]
fn question_mark_in_a_guard_returns_from_the_function() {
    ok(
        "fn g(x) if parse_int(x)? > 0 = Ok(\"pos\")\nfn g(_) = Ok(\"other\")\nprint(g(\"5\"), g(\"-1\"), is_err(g(\"z\")))",
        "Ok(\"pos\") Ok(\"other\") true",
    );
}

// ----- messages -------------------------------------------------------------

#[test]
fn messages_give_the_right_advice() {
    let r = fails("print(\"a\" * 3)", "E005");
    assert!(r.stderr.contains("repeat"), "{}", r.stderr);
    let r = fails("x = 1\nx++", "E018");
    assert!(r.stderr.contains("x + 1"), "{}", r.stderr);
    let r = fails("fn f(x) => x", "E001");
    assert!(r.stderr.contains("functions use `=`"), "{}", r.stderr);
    let r = warns("fn f(r)\n  {name, age} = r\n  name\nend\nprint(f({name: 1, age: 2}))", "W001");
    assert!(r.stderr.contains("age: _age"), "{}", r.stderr);
    let r = fails("type A = X | Y\ntype B = X | Z", "E011");
    assert!(r.stderr.contains("constructor belongs to a single type"), "{}", r.stderr);
    let r = fails("print(match Some(1)\n  Some(x) | Ok(x) -> x\n  _ -> 0\nend)", "E001");
    assert_eq!(r.stderr.matches("error[").count(), 1, "{}", r.stderr);
    ok("print(type_of(Some), parse_float(\"1_000.5\"), float(\"2_000.5\"))", "Function Ok(1000.5) 2000.5");
}

#[test]
fn doc_comments_only_document_what_follows_them() {
    let file =
        temp_file("docs.ry", "## Section\n\nfn f() = 1\n## Real doc\nfn g() = 2\nx = 1 ## remark\nfn h() = 3\nhelp(f)\nhelp(g)\nhelp(h)\n");
    let r = rythme(&[file.to_str().unwrap()]);
    assert_eq!(r.stdout.matches("no documentation").count(), 2, "{}", r.stdout);
    assert!(r.stdout.contains("Real doc"));
}

#[test]
fn windows_line_ends_do_not_end_up_inside_texts() {
    let file = temp_file("crlf.ry", "x = \"a\r\nb\"\r\nprint(len(x))\r\n");
    assert_eq!(rythme(&[file.to_str().unwrap()]).stdout, "3\n");
}

// ----- REPL ---------------------------------------------------------------

#[test]
fn repl_definitions_typed_one_per_line_add_up() {
    let out = repl("fn fib(0) = 0\nfn fib(1) = 1\nfn fib(n) = fib(n - 1) + fib(n - 2)\nfib(10)\n:quit\n");
    assert!(out.contains("\n55\n"), "{out}");
    assert!(!out.contains("W002"), "{out}");
}

#[test]
fn repl_blank_lines_inside_a_block_are_kept() {
    let out = repl("fn f(x)\n  y = x\n\n  y + 1\nend\nf(1)\n:quit\n");
    assert!(out.contains("\n2\n") && !out.contains("error"), "{out}");
}

#[test]
fn repl_pipelines_can_be_pasted_line_by_line() {
    let out = repl("[1, 2, 3, 4]\n  |> filter(is_even)\n  |> map(fn(x) = x * x)\n:quit\n");
    assert!(out.contains("[4, 16]") && !out.contains("error"), "{out}");
}
