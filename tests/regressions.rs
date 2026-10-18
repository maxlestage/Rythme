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

#[test]
fn long_chains_and_deep_calls_are_refused_not_a_stack_overflow() {
    // Chains nest without brackets: `((1 + 1) + 1) + ...`.
    let n = 200_000;
    for (name, code, expected) in [
        ("plus.ry", format!("x = {}\n", ["1"; 200_000].join(" + ")), "too long or nested too deeply"),
        ("calls.ry", format!("x = abs{}\n", "(1)".repeat(n)), "too long or nested too deeply"),
        ("question.ry", format!("fn f(x) = x{}\n", "?".repeat(n)), "too long or nested too deeply"),
        ("methods.ry", format!("x = 1{}\n", ".abs()".repeat(n)), "too long or nested too deeply"),
        ("index.ry", format!("x = [1]{}\n", "[0]".repeat(n)), "too long or nested too deeply"),
        ("and.ry", format!("x = {}true\n", "true and ".repeat(n)), "too long or nested too deeply"),
        ("pipes.ry", format!("x = 1{}\n", " |> abs".repeat(n)), "too long or nested too deeply"),
        // Arguments of calls, and `**`, were not counted as nesting.
        ("args.ry", format!("x = {}1{}\n", "abs(".repeat(n), ")".repeat(n)), "nested too deeply"),
        ("power.ry", format!("x = {}\n", ["2"; 200_000].join(" ** ")), "nested too deeply"),
        ("blocks.ry", format!("{}1\n{}", "fn f()\n".repeat(n), "end\n".repeat(n)), "nested too deeply"),
    ] {
        let r = rythme(&["check", temp_file(name, &code).to_str().unwrap()]);
        assert_eq!(r.code, 1, "{name}: {}", r.stderr);
        assert!(r.stderr.contains(expected) && !r.stderr.contains("overflow"), "{name}: {}", r.stderr);
    }
    // A few hundred operators in a row are fine.
    ok(&format!("print({})", ["1"; 500].join(" + ")), "500");
}

#[test]
#[cfg(target_os = "linux")]
fn a_full_error_output_does_not_crash_rythme() {
    let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["-e", "print(nope)"])
        .stderr(std::fs::File::create("/dev/full").unwrap())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn input_stops_at_the_end_of_the_input_instead_of_asking_forever() {
    let file = temp_file("ask.ry", "fn ask()\n  line = input(\"> \")\n  if line == \"quit\" then \"bye\" else ask()\nend\nprint(ask())\n");
    let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .arg(&file)
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("error[E017]") && stderr.contains("end of the input"), "{stderr}");
    let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .arg(&file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all("a\nb\nquit".as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(String::from_utf8_lossy(&out.stdout), "> > > bye\n");
}

#[test]
fn texts_that_grow_without_end_stop_with_an_error_not_out_of_memory() {
    for code in [
        "fn grow(s) = grow(s + s)\nprint(grow(\"ab\"))",
        "fn grow(s) = grow(\"{s}{s}\")\nprint(grow(\"ab\"))",
        "print(len(join(repeat([repeat(\"x\", 60000000)], 2))))",
        "print(len(replace(repeat(\"a\", 30000000), \"a\", \"abcd\")))",
    ] {
        let r = fails(code, "E023");
        assert!(r.stderr.contains("this text would be too large"), "{}", r.stderr);
    }
}

#[test]
fn fail_with_an_empty_message_still_says_something() {
    let r = fails("fail(\"\")", "E021");
    assert!(r.stderr.contains("stopped with `fail`"), "{}", r.stderr);
}

// ----- wrong results -----------------------------------------------------

#[test]
fn the_negation_of_the_smallest_whole_number_overflows() {
    fails("print(-(-9223372036854775808))", "E012");
    fails("print(- -9223372036854775808)", "E012");
    ok("print(match -9223372036854775808\n  -9223372036854775808 -> \"min\"\n  _ -> \"other\"\nend)", "min");
}

#[test]
fn powers_without_a_value_are_errors() {
    fails("print(0 ** -1)", "E008");
    fails("print(0.0 ** -1)", "E008");
    let r = fails("print((-8) ** (1 / 3))", "E023");
    assert!(r.stderr.contains("fractional power"), "{}", r.stderr);
    ok("print(2 ** -1, (-2.0) ** 3, (-8.0) ** 2.0)", "0.5 -8.0 64.0");
}

#[test]
fn decimal_remainders_are_exact_and_follow_the_divisor() {
    ok("print(5.0 % 1e400, -5.0 % 1e400, 1e20 % 3.0, -7.5 % 2.0, 7.5 % -2.0, 5 % 3.0)", "5.0 inf 1.0 0.5 -0.5 2.0");
}

#[test]
fn nan_is_explained_and_rounding_huge_numbers_keeps_them() {
    let r = fails("print(float(\"nan\") < 1)", "E005");
    assert!(r.stderr.contains("`nan` (not a number) cannot be compared"), "{}", r.stderr);
    for code in ["print(int(float(\"nan\")))", "print(int(\"nan\"))", "print(round(float(\"nan\")))"] {
        let r = fails(code, "E022");
        assert!(r.stderr.contains("`nan`"), "{}", r.stderr);
    }
    ok("print(round(1e307, 2), round(-1.7e308, 1), round(2.345, 2))", "1e307 -1.7e308 2.35");
}

#[test]
fn whole_and_decimal_numbers_compare_exactly() {
    ok("print(9007199254740993 == 9007199254740992.0, 9007199254740993 > 9007199254740992.0, 1 == 1.0)", "false true true");
    ok("print(match 9007199254740992.0\n  9007199254740993 -> \"wrong\"\n  _ -> \"right\"\nend)", "right");
    // `sort` follows `<`: [1.0] < [1, 2].
    ok("print(sort([[1, 2], [1.0]]), [1.0] < [1, 2])", "[[1.0], [1, 2]] true");
}

#[test]
fn and_or_check_that_a_call_on_their_right_gives_a_bool() {
    for code in [
        "fn identity(x) = x\nfn t() = true and identity(5)\nprint(t())",
        "fn identity(x) = x\nprint(true and identity(5))",
        "fn identity(x) = x\nprint(if true then false or identity(5) else false)",
    ] {
        fails(code, "E005");
    }
    ok("fn identity(x) = x\nfn t() = false or identity(true)\nprint(t())", "true");
}

#[test]
fn values_of_a_type_are_ordered_by_constructor() {
    ok(
        "print(sort([Some(2), None, Some(1)]), sort([Err(1), Ok(2)]), None < Some(0), max([None, Some(3)]))",
        "[None, Some(1), Some(2)] [Ok(2), Err(1)] true Some(3)",
    );
    ok("type Size = Small | Medium | Large\nprint(sort([Large, Small, Medium]), Small < Large)", "[Small, Medium, Large] true");
}

#[test]
fn a_record_pattern_cannot_name_the_other_fields() {
    for code in [
        "{name, ..others} = {name: \"Ada\", born: 1815}\nprint(others)",
        "print(match {name: \"Ada\"}\n  {name, ..others} -> name\nend)",
        "Some({name, ..rest}) = Some({name: 1})",
    ] {
        let r = fails(code, "E001");
        assert!(r.stderr.contains("cannot give a name to the other fields"), "{}", r.stderr);
    }
}

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

#[test]
fn definitions_accept_the_patterns_of_match() {
    ok("[a, ..] = [1, 2]\n{name, ..} = {name: \"Ada\", age: 36}\n[b, .._] = [3]\nprint(a, name, b)", "1 Ada 3");
    ok("fn f()\n  [x, ..] = [7, 8]\n  {y, ..} = {y: 9, z: 0}\n  x + y\nend\nprint(f())", "16");
    ok("[1, x] = [1, 2]\nSome(-1 | 0) = Some(0)\n(z) = 4\nprint(x, z)", "2 4");
}

#[test]
fn functions_written_one_after_the_other_in_a_block_see_each_other() {
    ok("fn outer(n)\n  fn a(0) = \"done\"\n  fn a(k) = b(k - 1)\n  fn b(k) = a(k)\n  a(n)\nend\nprint(outer(5))", "done");
    let r = rythme(&[
        "-e",
        "fn outer(n)\n  fn even(0) = true\n  fn even(k) = odd(k - 1)\n  fn odd(0) = false\n  fn odd(k) = even(k - 1)\n  even(n)\nend\nprint(outer(100001))",
    ]);
    assert_eq!(r.stdout, "false\n", "{}", r.stderr);
    assert!(r.stderr.is_empty(), "{}", r.stderr);
}

// ----- mistakes the checker now finds -------------------------------------

#[test]
fn the_types_of_rythme_cannot_be_redefined() {
    let r = fails("type Option = Foo | Bar\nprint(1)", "E011");
    assert!(r.stderr.contains("part of Rythme"), "{}", r.stderr);
    fails("type Int = Big", "E011");
}

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

#[test]
fn an_unclosed_brace_in_a_text_is_shown_where_it_is() {
    for code in ["print(\"{\")", "print(\"{x\")", "print(\"{ # not a comment }\")"] {
        let r = fails(code, "E002");
        assert!(r.stderr.contains("`{` in a text is never closed") && r.stderr.contains("<code>:1:8"), "{}", r.stderr);
        assert!(r.stderr.contains("\\{"), "{}", r.stderr);
    }
}

#[test]
fn mistakes_inside_an_insertion_explain_how_to_write_braces() {
    for (code, error) in [("print(\"[1, {\\\"a\\\": true}]\")", "E018"), ("print(\"{a: 1}\")", "E001"), ("print(\"{\"{x: 1}\"}\")", "E001")]
    {
        let r = fails(code, error);
        assert_eq!(r.stderr.matches("use `\\{` and `\\}`").count(), 1, "{}", r.stderr);
        assert!(!r.stderr.contains("block"), "{}", r.stderr);
    }
}

#[test]
fn characters_copied_from_documents_are_explained() {
    for (code, expected) in [
        ("x = 1\u{a0}+ 2", "U+00A0"),
        ("x = 1\u{200b}+ 2", "U+200B"),
        ("x = 1\n\u{feff}y = 2", "U+FEFF"),
        ("x = 1\u{1}", "U+0001"),
        ("x = 5 − 3", "write `-` instead"),
        ("x = 5 × 3", "write `*` instead"),
        ("x = 1 ≠ 2", "write `!=` instead"),
        ("print（1）", "write `(` instead"),
        ("x = ‘a’", "double quotes"),
    ] {
        let r = fails(code, "E018");
        assert!(r.stderr.contains(expected), "{code:?}: {}", r.stderr);
    }
    let r = common::rythme_lang("fr", &["-e", "x = 1\u{a0}+ 2"]);
    assert!(r.stderr.contains("espace spéciale"), "{}", r.stderr);
}

#[test]
fn every_file_given_to_check_is_checked_and_code_gets_its_arguments() {
    let good = temp_file("good.ry", "print(1)\n");
    let bad = temp_file("bad.ry", "print(nope)\n");
    let r = rythme(&["check", bad.to_str().unwrap(), good.to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stdout.contains("bad.ry: 1 error") && r.stdout.contains("good.ry: no problems"), "{}", r.stdout);
    let r = rythme(&["-e", "print(args())", "a", "b c"]);
    assert_eq!(r.stdout, "[\"a\", \"b c\"]\n");
}

#[test]
fn number_errors_say_what_is_wrong() {
    let r = fails("print(0xFFFFFFFFFFFFFFFF)", "E012");
    assert!(r.stderr.contains("too large"), "{}", r.stderr);
    let r = fails("print(0xg)", "E018");
    assert!(r.stderr.contains("0xff"), "{}", r.stderr);
    let r = fails("print(\"a\\q\")", "E018");
    assert!(r.stderr.contains("\\u{e9}") && r.stderr.contains("\\r") && r.stderr.contains("\\0"), "{}", r.stderr);
}

#[test]
fn snippets_are_aligned_and_cut_around_the_mistake() {
    // Wide characters take two columns, accents written as two characters one.
    let r = fails("x = \"日本\" + nope", "E003");
    assert!(r.stderr.contains(&format!("  | {}^^^^ not defined", " ".repeat(13))), "{}", r.stderr);
    let r = fails("x = \"e\u{301}\" + nope", "E003");
    assert!(r.stderr.contains(&format!("  | {}^^^^ not defined", " ".repeat(10))), "{}", r.stderr);
    let r = fails(&format!("x = \"{}\" + nope", "a".repeat(5000)), "E003");
    assert!(r.stderr.lines().all(|l| l.chars().count() < 200), "{}", r.stderr);
    assert!(r.stderr.contains("...") && r.stderr.contains("^^^^ not defined"), "{}", r.stderr);
}

#[test]
fn files_that_cannot_be_read_are_explained() {
    let file = temp_file("latin1.ry", "");
    std::fs::write(&file, b"print(1)\nprint(\"caf\xe9\")\n").unwrap();
    let r = rythme(&[file.to_str().unwrap()]);
    assert!(r.stderr.contains("UTF-8") && r.stderr.contains("line 2"), "{}", r.stderr);
    let r = common::rythme_lang("fr", &[file.to_str().unwrap()]);
    assert!(r.stderr.contains("ligne 2"), "{}", r.stderr);
    let folder = file.parent().unwrap().join("folder.ry");
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("inner.ry"), "").unwrap();
    let r = rythme(&[folder.to_str().unwrap()]);
    assert!(r.stderr.contains("is a folder"), "{}", r.stderr);
    let r = rythme(&[temp_file("use_folder.ry", "use \"folder.ry\"\n").to_str().unwrap()]);
    assert!(r.stderr.contains("is a folder") && r.stderr.contains("inner.ry"), "{}", r.stderr);
}

#[test]
fn use_finds_files_in_nested_folders_on_every_system() {
    // main.ry -> app/screen.ry -> app/parts/button.ry -> ../../shared.ry, and app/screen.ry -> ../shared.ry.
    let root = temp_file("anchor.txt", "").parent().unwrap().join("nested");
    std::fs::create_dir_all(root.join("app/parts")).unwrap();
    std::fs::write(root.join("main.ry"), "use \"app/screen.ry\"\nprint(screen())\n").unwrap();
    std::fs::write(root.join("shared.ry"), "fn shared() = \"shared\"\nprint(\"shared loaded\")\n").unwrap();
    std::fs::write(
        root.join("app/screen.ry"),
        "use \"parts/button\"\nuse \"../shared.ry\"\nfn screen() = \"screen with {button()} and {shared()}\"\n",
    )
    .unwrap();
    std::fs::write(root.join("app/parts/button.ry"), "use \"../../shared.ry\"\nfn button() = \"button ({shared()})\"\n").unwrap();
    let r = rythme(&[root.join("main.ry").to_str().unwrap()]);
    assert_eq!(r.stdout, "shared loaded\nscreen with button (shared) and shared\n", "{}", r.stderr);
    let r = rythme(&["check", root.join("app/screen.ry").to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
}

#[test]
fn a_wrong_language_option_is_reported() {
    for args in [&["--lang", "xx", "help"][..], &["--lang"], &["--lang=", "help"]] {
        let r = rythme(args);
        assert_eq!(r.code, 2, "{args:?}");
        assert!(r.stderr.contains("--lang fr"), "{args:?}: {}", r.stderr);
    }
}

#[test]
fn suggestions_are_not_far_fetched() {
    let r = fails("print(Foo)", "E003");
    assert!(!r.stderr.contains("did you mean"), "{}", r.stderr);
    let r = fails("print(lenn([1]))", "E003");
    assert!(r.stderr.contains("did you mean `len`?"), "{}", r.stderr);
    let r = rythme(&["sbu"]);
    assert!(!r.stderr.contains("did you mean"), "{}", r.stderr);
}

#[test]
fn hints_fit_the_mistake() {
    let r = fails("print(\"a\"(2))", "E009");
    assert!(!r.stderr.contains("multiply") && r.stderr.contains("x[0]"), "{}", r.stderr);
    assert!(fails("print(2(3))", "E009").stderr.contains("multiply"));
    let r = fails("fn greet = print(\"hi\")", "E001");
    assert!(!r.stderr.contains("==") && r.stderr.contains("fn greet()"), "{}", r.stderr);
    let r = warns("pi = 3", "W003");
    assert!(r.stderr.contains("built-in value `pi`"), "{}", r.stderr);
    let r = warns("print(match 5\n  5 -> 1\nend)", "W002");
    assert_eq!(r.stderr.matches("`_ -> ...`").count(), 1, "{}", r.stderr);
    let r = fails("x = Some(Some(1))??", "E016");
    assert_eq!(r.stderr.matches("error[E016]").count(), 1, "{}", r.stderr);
    let r = fails("print(random(1, 2, 3))", "E004");
    assert!(r.stderr.contains("random(list)"), "{}", r.stderr);
}

#[test]
fn help_shows_the_documentation_of_types() {
    ok("## Something to draw.\ntype Shape = Circle(r) | Dot\nhelp(Circle)\nhelp(Dot)", "Circle(r)\n  A constructor of the type `Shape`.\n  Something to draw.\nThis is a value of type Shape: Dot\n  Something to draw.\nUse `match` to handle each variant of `Shape`.");
    let r = rythme(&["help", "type_of"]);
    assert!(r.stdout.contains("\"Unit\"") && r.stdout.contains("\"Option\""), "{}", r.stdout);
    let r = rythme(&["help", "text"]);
    assert!(r.stdout.contains("\\u{e9}") && r.stdout.contains("\\0"), "{}", r.stdout);
}

// ----- REPL ---------------------------------------------------------------

#[test]
fn repl_commands_accept_what_the_command_line_accepts() {
    let file = temp_file("plain_name", "fn twice(x) = 2 * x\n");
    let out = repl(&format!(":load {}\ntwice(4)\n:explain E003 please\n", file.display()));
    assert!(out.contains("\n8\n") && out.contains("E003 — Unknown name"), "{out}");
}

#[test]
fn repl_shows_the_error_of_code_left_unfinished_at_the_end() {
    let out = repl("print(\"unclosed\n");
    assert!(out.contains("error[E002]"), "{out}");
    let out = repl("do\n  1\n");
    assert!(out.contains("error[E020]"), "{out}");
}

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

#[test]
fn a_name_defined_further_down_in_a_block_is_explained() {
    // Not "in Rythme, this is called `is_odd`": `odd` is the program's own function.
    let r = fails("fn parity(n) = do\n  fn even(0) = true\n  fn even(k) = odd(k - 1)\n  limit = 10\n  fn odd(0) = false\n  fn odd(k) = even(k - 1)\n  even(n + limit)\nend\nprint(parity(4))", "E003");
    assert!(
        !r.stderr.contains("is_odd") && r.stderr.contains("defined here, further down") && r.stderr.contains("move `odd` up"),
        "{}",
        r.stderr
    );
    let r = fails("fn f() = do\n  print(y)\n  y = 1\n  y\nend\nf()", "E003");
    assert!(r.stderr.contains("move `y = ...` up"), "{}", r.stderr);
}

#[test]
fn padding_counts_the_size_of_the_text_like_every_text() {
    // `é` takes 2 bytes: 99 999 999 of them are more than 100 MB.
    let r = fails("print(len(pad_left(\"a\", 99999999, \"é\")))", "E023");
    assert!(r.stderr.contains("more than 100 MB"), "{}", r.stderr);
    ok("print(len(pad_left(\"a\", 3, \"é\")))", "3");
}
