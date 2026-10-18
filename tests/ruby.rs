//! The Ruby style: blocks, method chains, names ending with `?`, `unless`,
//! symbols, `#{}` in texts and the Ruby names of the library; and the cases
//! where these could be read in two ways.

mod common;

use common::{fails, ok, rythme, warns};

#[test]
fn blocks_are_the_last_argument_of_a_call() {
    ok("print([1, 2, 3].map do |x| x * 2 end)", "[2, 4, 6]");
    ok("print(map([1, 2]) do |x| x + 1 end)", "[2, 3]");
    ok("[1, 2].each do |x|\n  print(x)\nend", "1\n2");
    ok("print([1, 2, 3].select { |x| x > 1 })", "[2, 3]");
    ok("print([1, 2, 3].reduce(0) do |acc, x| acc + x end)", "6");
    ok("print([[1, 2], [3, 4]].map do |[a, b]| a * b end)", "[2, 12]");
    ok("print([{n: 1}].map { |{n}| n })", "[1]");
    ok("fn run(f) = f()\nprint(run do 42 end, run { || 7 })", "42 7");
    // On several lines, with statements, and the chain goes on after `end`.
    ok("print([1, 2].map do |x|\n  y = x * 10\n  y + 1\nend.sum)", "32");
    ok("xs = [3, 1, 2]\n  .sort_by { |x| -x }\n  .map do |x| x * x end\n  .sum\nprint(xs)", "14");
    // Inside a text and inside brackets.
    ok("print(\"{[1, 2].map { |x| x * 3 }}\")", "[3, 6]");
    ok("print(len([1, 2].select do |x| x > 1 end))", "1");
    // A block is a function: a pipe step too.
    ok("print([1, 2] |> map do |x| -x end)", "[-1, -2]");
}

#[test]
fn a_block_is_never_a_record_and_needs_a_call() {
    // `{` followed by `|` is a block; anything else a record.
    ok("r = {a: 1}\nprint(r, [{b: 2}])", "{a: 1} [{b: 2}]");
    let r = fails("xs = [1]\nb = do |x| x end", "E001");
    assert!(r.stderr.contains("a block must follow a call"), "{}", r.stderr);
    let r = fails("f = { |x| x }", "E001");
    assert!(r.stderr.contains("a block must follow a call"), "{}", r.stderr);
    // `do` on the next line is not a block of the call above.
    fails("xs = [1].map\ndo |x| x end", "E001");
    // After a value that is not a call, a field or a name: no block.
    let r = fails("print(1 do 2 end)", "E001");
    assert!(r.stdout.is_empty());
    let r = fails("x = 1\nx + 1 do print(1) end", "E001");
    assert!(r.stderr.contains("given to a call"), "{}", r.stderr);
    // The plain `do ... end` block keeps its meaning.
    ok("x = do\n  a = 1\n  a + 1\nend\nprint(x)", "2");
    ok("print(if true then do 1 end else 2)", "1");
}

#[test]
fn blocks_are_checked_like_functions() {
    let r = fails("print([1, 2].map do |a, b| a end)", "E004");
    assert!(r.stderr.contains("gives 1 argument(s) to this function, but it takes 2"), "{}", r.stderr);
    fails("print([1].select { |x| x + 1 })", "E005");
}

#[test]
fn a_dot_without_parentheses_reads_a_field_or_calls_a_function() {
    ok("print([1, 2, 3].sum, [3, 1].sort, \"hi\".upcase, [1, 2].first, [].empty?)", "6 [1, 3] HI Some(1) true");
    // The field wins over the function.
    ok("r = {sum: \"field\", len: 3}\nprint(r.sum, r.len, r.size)", "field 3 2");
    ok("type Shape = Circle(radius)\nfn area(s) = 3 * s.radius * s.radius\nprint(Circle(2).area, Circle(2).radius)", "12 2");
    // A function of the file, a local one, a function value.
    ok("fn double(x) = x * 2\nprint(21.double)", "42");
    ok("fn f(xs)\n  twice = fn(v) = v + v\n  xs.len.twice\nend\nprint(f([1, 2]))", "4");
    // Neither a field nor a function: E010, before running when certain.
    let r = fails("print([1].summ)", "E010");
    assert!(r.stderr.contains("did you mean `.sum`?"), "{}", r.stderr);
    let r = fails("print({a: 1}.b)", "E010");
    assert!(r.stderr.contains("this record has no field `b`"), "{}", r.stderr);
    // The checker knows the type of the result.
    let r = fails("print([1, 2].sum + \"a\")", "E005");
    assert!(r.stderr.contains("cannot use `+` with Int and Text"), "{}", r.stderr);
    // A record that may have more fields: no error.
    ok("fn get_total(r) = r.sum\nprint(get_total({sum: 1}), get_total([1, 2]))", "1 3");
}

#[test]
fn chains_go_on_over_several_lines() {
    ok("total = [1, 2, 3, 4]\n  .select(even?)\n  # the squares\n  .map { |x| x * x }\n\n  .sum\nprint(total)", "20");
    ok("print(\"  Ada  \"\n  .strip\n  .downcase)", "ada");
}

#[test]
fn names_ending_with_a_question_mark() {
    ok("fn adult?(p) = p.age >= 18\nada = {age: 36}\nprint(adult?(ada), ada.adult?, even?(4), odd?(4))", "true true true false");
    ok("print([1, 2, 3, 4].select(even?), [1, 2].include?(2), \"abc\".start_with?(\"a\"), {a: 1}.key?(\"a\"))", "[2, 4] true true true");
    ok("fn small?(n) = n < 3\nprint([1, 5].reject(small?), [1, 5] |> filter(small?))", "[5] [1]");
    // A local function defined with `fn` and a name ending with `?`.
    ok("fn f(xs)\n  fn big?(n) = n > 1\n  xs.select(big?)\nend\nprint(f([1, 2, 3]))", "[2, 3]");
    // `is_even` and the other names keep working.
    ok("print(is_even(2), is_empty([]))", "true true");
    // Only functions may have such a name.
    let r = fails("ok? = 1", "E001");
    assert!(r.stderr.contains("only a function can have a name that ends with `?`"), "{}", r.stderr);
}

#[test]
fn the_question_mark_operator_keeps_its_meaning() {
    // `value?` and `call(x)?`: not a name, since no such function exists.
    ok("fn f(r)\n  v = r?\n  Some(v + 1)\nend\nprint(f(Some(1)), f(None))", "Some(2) None");
    ok("fn f(t)\n  n = parse_int(t)?\n  Ok(n * 2)\nend\nprint(f(\"21\"), f(\"x\") |> is_err)", "Ok(42) true");
    ok("fn f(t)\n  n = t |> parse_int?\n  Ok(n)\nend\nprint(f(\"4\"))", "Ok(4)");
    ok("fn f(r)\n  v = r.port?\n  Some(v)\nend\nprint(f({port: Some(80)}), f({port: None}))", "Some(80) None");
    // A value named like a predicate (`empty` and `empty?`): `(value)?` or a space.
    ok("fn f(empty)\n  a = (empty)?\n  b = empty ?\n  Some(a + b)\nend\nprint(f(Some(1)))", "Some(2)");
    // Double `?`.
    ok("fn f(r)\n  v = r??\n  Some(v)\nend\nprint(f(Some(Some(3))))", "Some(3)");
}

#[test]
fn unless_is_the_opposite_of_if() {
    ok("print(unless 1 > 2 then \"small\" else \"big\")", "small");
    ok("x = 5\nprint(unless x > 2 then \"small\" else \"big\")", "big");
    ok("print(unless not true then 1 else 2, unless (1 > 2) then :a else :b)", "1 :a");
    let r = fails("print(unless true then 1)", "E019");
    assert!(r.stderr.contains("this `unless` has no `else`"), "{}", r.stderr);
    // A value or a function named `unless` still works.
    ok("unless = 3\nprint(unless + 1, [unless])", "4 [3]");
    ok("fn unless(x) = x * 2\nprint(unless(4), if unless(1) == 2 then \"yes\" else \"no\")", "8 yes");
    ok("unless = [7]\nprint(unless[0])", "7");
}

#[test]
fn symbols() {
    ok("print(:ok, :ok == :ok, :a < :b, type_of(:error), [:a, :b])", ":ok true true Symbol [:a, :b]");
    ok(
        "fn label(s) = match s\n  :ok -> \"fine\"\n  :error -> \"bad\"\n  _ -> \"?\"\nend\nprint(label(:ok), label(:error), label(:x))",
        "fine bad ?",
    );
    ok("print(text(:ok), \"{:ok}\", to_json({s: :ok}), sort([:b, :a]))", ":ok :ok {\"s\": \"ok\"} [:a, :b]");
    ok("[s, n] = [:k, 1]\nprint(s, n)", ":k 1");
    // The `:` of a record stays the `:` of a record, with or without spaces.
    ok("v = 2\nprint({a:v}, {a: :v}, {\"b\":v}, {type:v}, {x: 1, y:v})", "{a: 2} {a: :v} {b: 2} {type: 2} {x: 1, y: 2}");
    ok("v = 3\nr = {\n  a:v,\n  b:\n    v,\n}\nprint(r)", "{a: 3, b: 3}");
    ok("v = 4\nprint(match {a: 1}\n  {a:x} -> x + v\nend)", "5");
    ok("v = 1\nprint(\"{ {a:v}.a }\")", "1");
    warns("s = :ok\nprint(s == \"ok\")", "W006");
    fails("print(:a + 1)", "E005");
}

#[test]
fn ruby_interpolation_in_texts() {
    ok("name = \"Ada\"\nprint(\"Hello, #{name}! #{1 + 1} {name}\")", "Hello, Ada! 2 Ada");
    ok("print(\"a \\#{b} c\", \"# not a comment\", \"#\", \"\\#\")", "a #{b} c # not a comment # #");
    ok("print(\"#{\"in #{1}\"}\")", "in 1");
    fails("print(\"#{\")", "E002");
}

#[test]
fn ruby_names_of_the_library() {
    ok("puts(\"hi\")", "hi");
    ok("print([1, 2, 2].uniq, [1, 2].size, [1].length, \" x \".strip, \"a\".upcase, \"B\".downcase)", "[1, 2] 2 1 x A b");
    ok("print(42.to_s + \"!\", \"7\".to_i + 1, \"2.5\".to_f, [1, 2, 3].inject(0) do |a, x| a + x end)", "42! 8 2.5 6");
    ok(
        "print([1, 2, 3, 4].each_slice(2), [1, 2, 3].each_cons(2), [1, 2].any?(even?), [2].all?(even?))",
        "[[1, 2], [3, 4]] [[1, 2], [2, 3]] true true",
    );
    ok("print(\"abc\".end_with?(\"c\"), [1, 2].include?(3), \"\".empty?)", "true false true");
    // New functions and new forms.
    ok(
        "print(reject([1, 2, 3], is_even), first([1, 2, 3], 2), last([1, 2, 3], 2), count([1, 2]), sum([\"ab\", \"c\"], len))",
        "[1, 3] [1, 2] [2, 3] 2 3",
    );
    ok("each_with_index([\"a\", \"b\"], fn(x, i) = print(i, x))", "0 a\n1 b");
    ok("print(first(\"abc\", 2), last(\"abc\", 5))", "ab abc");
    // An error says the name of the function used.
    let r = fails("print(upcase(1))", "E005");
    assert!(r.stderr.contains("`upper` expects"), "{}", r.stderr);
    // A program may use these names for itself, without a warning.
    let r = rythme(&["-e", "fn size(x) = 99\nselect = 1\nprint(size([1]), select, [1, 2].size)"]);
    assert_eq!(r.stdout, "99 1 99\n", "{}", r.stderr);
    assert!(!r.stderr.contains("W003"), "{}", r.stderr);
}

#[test]
fn the_formatter_keeps_the_ruby_style() {
    let fmt = |code: &str| rythme::format::format(code).unwrap_or_else(|d| panic!("{code}: {}", d[0].render(false)));
    assert_eq!(fmt("xs.map do |x|x*2 end\n"), "xs.map do |x| x * 2 end\n");
    assert_eq!(fmt("xs.map {|x,y|x}\n"), "xs.map { |x, y| x }\n");
    assert_eq!(fmt("xs.each do |x|\nprint(x)\nend\n"), "xs.each do |x|\n  print(x)\nend\n");
    assert_eq!(fmt("xs.each do |x| print(x)\nend\n"), "xs.each do |x|\n  print(x)\nend\n");
    assert_eq!(fmt("f() {||1}\n"), "f() { || 1 }\n");
    assert_eq!(fmt("x = unless a then :b else  :c\n"), "x = unless a then :b else :c\n");
    assert_eq!(fmt("fn adult?(p)=p.age>18\n"), "fn adult?(p) = p.age > 18\n");
    assert_eq!(fmt("r = {a::ok}\n"), "r = {a: :ok}\n");
    // A `?` that is the operator stays apart from a name it would join.
    assert_eq!(fmt("fn f(empty)\n  v = empty   ?\n  Some(v)\nend\n"), "fn f(empty)\n  v = empty ?\n  Some(v)\nend\n");
    assert_eq!(fmt("fn f(g)\n  v = g ?(1)\n  v\nend\n"), "fn f(g)\n  v = g ?(1)\n  v\nend\n");
    assert_eq!(fmt("fn f(r)\n  v = r  ?\n  v\nend\n"), "fn f(r)\n  v = r?\n  v\nend\n");
}

#[test]
fn help_knows_the_ruby_style() {
    let r = rythme(&["help", "syntax"]);
    assert!(r.stdout.contains("xs.map do |x| x * 2 end") && r.stdout.contains(":ok   :error"), "{}", r.stdout);
    let r = rythme(&["help", "unless"]);
    assert!(r.stdout.contains("The opposite of `if`"), "{}", r.stdout);
    let r = rythme(&["help", "empty?"]);
    assert!(r.stdout.contains("the Ruby name of `is_empty`"), "{}", r.stdout);
    let r = common::rythme_lang("fr", &["help", "upcase"]);
    assert!(r.stdout.contains("le nom Ruby de `upper`"), "{}", r.stdout);
}

#[test]
fn rythme_doc_knows_names_ending_with_a_question_mark_and_ruby_names() {
    let dir = std::env::temp_dir().join(format!("rythme-ruby-doc-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("people.ry");
    std::fs::write(&file, "## Is the person an adult? Uses `upcase` nowhere.\nfn adult?(person) = person.age >= 18\n").unwrap();
    let r = rythme(&["doc", file.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.contains("### adult?") && r.stdout.contains("fn adult?(person)"), "{}", r.stdout);
    assert!(r.stdout.contains("[`upcase`](#upper)") && r.stdout.contains("Same as: `upcase` (its Ruby names)."), "{}", r.stdout);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_call_without_parentheses_may_be_a_statement() {
    // `"Ada".greet` calls `greet`: its value is not lost for nothing (no E024).
    ok(
        "fn greet(n) = print(\"Hello, {n}!\")\n\"Ada\".greet\nfn main()\n  \"Bob\".greet\n  1\nend\nprint(main())",
        "Hello, Ada!\nHello, Bob!\n1",
    );
    // A field read for nothing still is.
    fails("r = {a: 1}\nr.a\nprint(r)", "E024");
}

#[test]
fn calls_without_parentheses() {
    ok("puts \"Hello\"\nprint 1, \"a\", :ok", "Hello\n1 a :ok");
    ok("x = 3\ny = 4\nprint x, y\ntotal = x + y\nassert_eq total, 7\nputs total", "3 4\n7");
    ok("[1, 2].each do |x| puts x end\n[3].each { |x| puts x * 2 }", "1\n2\n6");
    // The arguments are whole expressions; a new line may follow a comma.
    ok("xs = [1, 2]\nprint xs.map { |x| x * 10 }, xs |> sum, if true then \"y\" else \"n\"\nprint 1,\n  2", "[10, 20] 3 y\n1 2");
    ok("puts not false\nputs Some(1)\nputs fn(x) = x\nputs match 2\n  2 -> \"two\"\n  _ -> \"?\"\nend", "true\nSome(1)\n<fn>\ntwo");
    ok("fn twice(f, x) = f(f(x))\nputs twice(fn(x) = x * 3, 2)\nfn show(a, b) = print(a, b)\nshow \"sum\", 1 + 2", "18\nsum 3");
    // Names ending with `?` and Ruby names.
    ok("fn small?(n) = n < 10\nprint small?(3)\nputs \"x\".upcase\nassert even?(4)\neven? 4\nputs :done", "true\nX\n:done");
    // A `do` block after the arguments is given to the call itself.
    ok("fn each_twice(xs, f) = xs.each do |x|\n  f(x)\n  f(x)\nend\neach_twice [1, 2] do |v|\n  puts v * 10\nend", "10\n10\n20\n20");
    ok("each [1, 2].map { |x| x + 1 } do |v| puts v end", "2\n3");
    // Statements only: inside blocks and functions too.
    ok("fn main()\n  puts \"in\"\n  1\nend\nprint main()\ntest \"t\"\n  assert_eq 1, 1\nend", "in\n1");
    // The parentheses stay the same call.
    ok("puts(\"a\")\nputs (\"b\")", "a\nb");
}

#[test]
fn calls_without_parentheses_and_values_that_look_like_them() {
    // `-` after a space and right before a number or a name, and `[` after a
    // space: a call when the name is not a value of the code...
    ok("puts -1\nputs [1, 2]\nfn show(v) = print(v)\nshow -5\nshow [0]", "-1\n[1, 2]\n-5\n[0]");
    // Not inside an argument: there, a name and `-` subtract.
    let r = fails("fn neg(v) = -v\nputs neg -5", "E005");
    assert!(r.stderr.contains("to call `neg`, write `neg(-x)`"), "{}", r.stderr);
    // ...but a subtraction or an index when it is, as before.
    ok("fn dec(n) = do\n  n -1\nend\nfn first(xs) = do\n  xs [0]\nend\nprint dec(5), first([4, 5])", "4 4");
    ok("x = 10\nxs = [7]\nputs x -1, xs [0]\nputs x - 1, x-1", "9 7\n9 9");
    // Parameters, block parameters, names of patterns and `=` all make values.
    ok("fn f(g) = do\n  g -1\nend\nprint f(5)", "4");
    ok("[5].each do |v| print(do\n  v -1\nend) end", "4");
    ok("print(match 5\n  v -> do\n    v -1\n  end\nend)", "4");
    // A function defined with `=` is a value too: `double -3` subtracts.
    let r = fails("double = fn(x) = x * 2\ndouble -3", "E005");
    assert!(r.stderr.contains("to call `double`, write `double(-x)`"), "{}", r.stderr);
    // Without a space before, or with one after, never a call.
    fails("puts-1", "E005");
    fails("puts - 1", "E005");
    fails("puts[0]", "E005");
    // `(` keeps its meaning: the usual call, then the rest of the expression.
    fails("puts (1 + 2) * 3", "E005");
}

#[test]
fn calls_without_parentheses_are_checked_and_explained() {
    let r = fails("fn add(a, b) = a + b\nadd 1", "E004");
    assert!(r.stderr.contains("add 1"), "{}", r.stderr);
    fails("upper 5", "E005");
    let r = fails("x = 1\nprint x x", "E001");
    assert!(r.stderr.contains("separated by commas: `print a, b`"), "{}", r.stderr);
    // Only at the start of a statement.
    fails("x = len \"abc\"", "E001");
    fails("print(len \"abc\")", "E001");
    // The formatter keeps them as they are written.
    let fmt = |code: &str| rythme::format::format(code).unwrap_or_else(|d| panic!("{code}: {}", d[0].render(false)));
    assert_eq!(fmt("puts  \"a\"\nprint(1)\nprint 1 ,2\n"), "puts \"a\"\nprint(1)\nprint 1, 2\n");
    assert_eq!(fmt("x = 1\nputs x -1\nxs = [x]\nputs xs [0]\n"), "x = 1\nputs x - 1\nxs = [x]\nputs xs[0]\n");
    let r = rythme(&["help", "ruby"]);
    assert!(r.stdout.contains("puts \"hi\"   print x, y") && r.stdout.contains("Calls without parentheses"), "{}", r.stdout);
    let r = common::rythme_lang("fr", &["help", "ruby"]);
    assert!(r.stdout.contains("Appels sans parenthèses"), "{}", r.stdout);
}
