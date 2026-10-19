//! The language for AI assistants: `docs/llms.txt` (an index, in the
//! llmstxt.org format) and `docs/llms-full.txt` (the whole language in one
//! text a model reads before writing Rythme), printed by `rythme help llms`.
//!
//! Both are built here from the sources of truth: the table of built-in
//! functions, the keywords and the explanations of the codes, around a part
//! written by hand (the rules, the syntax by example, the mistakes that
//! habits from other languages cause). `tests/llms.rs` fails when the files
//! of `docs/` differ from what this gives (write them again with
//! `rythme help llms > docs/llms-full.txt` and
//! `rythme help llms.txt > docs/llms.txt`), and runs every example.
//!
//! They are in English, whatever the language of the messages: the
//! language models read best, and the names of Rythme are English.

use crate::builtins::{self, Category, BUILTINS};
use crate::help;
use crate::lang::{self, Lang};

/// The address of the files of the repository, for the links.
const REPO: &str = "https://github.com/maxlestage/Rythme/blob/master";

/// Runs `f` with the messages in English.
fn in_english<T>(f: impl FnOnce() -> T) -> T {
    let before = lang::get();
    lang::set(Lang::En);
    let out = f();
    lang::set(before);
    out
}

/// The first sentence of a documentation: up to the first ". ".
fn first_sentence(doc: &str) -> &str {
    let end = doc.find(". ").map_or(doc.len(), |i| i + 1);
    doc[..end].trim_end()
}

/// `docs/llms.txt`: what Rythme is, and links to read (llmstxt.org format).
pub fn index() -> String {
    format!(
        r#"# Rythme

> Rythme is a small, purely functional programming language with help everywhere: simpler than Python, safe like Rust (no null, no exceptions, values never change), with Rust-style error messages that name the fix. Version {version}. Models do not know it yet: read `llms-full.txt` (the whole language in one file) before writing Rythme, then check the code with `rythme check --format json` and fix it until it is clean.

How to work with Rythme as an AI assistant:

- Read the full reference first: everything differs from Python, JavaScript and Ruby in small ways (no loops, no `return`, no mutation, `if` always has `else`).
- After writing code, run `rythme check --format json FILE.ry`: each diagnostic has a stable code, a place, help in words and code, and often fixes as text edits.
- `rythme fix FILE.ry` applies the safe fixes; `rythme explain CODE` explains a code; `rythme help NAME --format json` documents a built-in function.
- Run `rythme test --format json` and `rythme run --format json FILE.ry`; repeat until there are no errors.
- `rythme mcp` is a Model Context Protocol server with these tools (run in a sandbox, check, test, format, fix, explain, help, steps).

## Docs

- [Rythme for language models, complete]({REPO}/docs/llms-full.txt): the syntax, every construct with an example, every built-in function, common mistakes from other languages and every error code, in one file
- [Reference manual]({REPO}/docs/reference.md): the precise definition of the language and its tools
- [Rythme and AI assistants]({REPO}/docs/reference.md#710-rythme-and-ai-assistants): JSON output, `rythme fix`, the MCP server and how to add it to Claude Code, Claude Desktop or Cursor
- [JSON output schema]({REPO}/docs/rythme-output.schema.json): the documents printed by `--format json`

## Optional

- [README]({REPO}/README.md): what Rythme is, installation
- [Exercises]({REPO}/docs/exercises/README.md): 30 graded exercises with tests
- [Examples]({REPO}/examples): small complete programs
- [AGENTS.md]({REPO}/AGENTS.md): how to work on Rythme itself (the interpreter, in Rust)
"#,
        version = crate::VERSION
    )
}

/// `docs/llms-full.txt`: the whole language for a model, in one text.
pub fn full() -> String {
    in_english(|| {
        let mut out = HEAD.replace("{version}", crate::VERSION);
        out.push_str(SYNTAX);
        out.push_str(&builtin_section());
        out.push_str(&keyword_section());
        out.push_str(MISTAKES);
        out.push_str(&code_section());
        out.push_str(TOOLS);
        out
    })
}

fn builtin_section() -> String {
    let mut out = String::from(
        "\n## Built-in functions\n\nAlways available, no import. `f(x, y)` can also be written `x.f(y)` or `x |> f(y)`. One line each: signature, then what it does. `rythme help NAME` shows the details and an example.\n",
    );
    for cat in Category::ALL {
        out.push_str(&format!("\n### {}\n\n", cat.title()));
        for b in BUILTINS.iter().filter(|b| b.cat == cat) {
            out.push_str(&format!("- `{}`: {}\n", b.sig, first_sentence(b.en)));
        }
        if cat == Category::Numbers {
            out.push_str("- `pi`: the number π (a value, not a function)\n");
        }
        if cat == Category::Options {
            out.push_str("- `Some(x)`, `None`, `Ok(x)`, `Err(e)`: build an Option or a Result\n");
        }
    }
    out.push_str("\nRuby names, exactly the same functions: ");
    let ruby: Vec<String> = builtins::ALIASES.iter().map(|a| format!("`{}` = `{}`", a.name, a.of)).collect();
    out.push_str(&ruby.join(", "));
    out.push_str(".\n");
    out
}

fn keyword_section() -> String {
    let mut out = String::from("\n## Keywords\n\n");
    for word in help::keywords() {
        if let Some((doc, _, _)) = help::keyword_doc(word) {
            out.push_str(&format!("- `{word}`: {doc}\n"));
        }
    }
    let reserved: Vec<String> = crate::lexer::RESERVED.iter().map(|w| format!("`{w}`")).collect();
    out.push_str(&format!(
        "\nReserved because other languages use them (each gives a message saying what to write instead): {}.\n",
        reserved.join(", ")
    ));
    out
}

fn code_section() -> String {
    let mut out = String::from(
        "\n## Error and warning codes\n\nEvery diagnostic has a stable code; `rythme explain CODE` gives the long explanation with a wrong and a right example. Errors stop the program (or `rythme check`); warnings do not.\n\n",
    );
    for code in help::CODES {
        let title = help::code_title(code).unwrap_or_default();
        out.push_str(&format!("- {code}: {title}\n"));
    }
    out
}

const HEAD: &str = r#"# Rythme {version}: the complete language for AI assistants

> Rythme is a small, purely functional language: values never change, there are no loops, no `return`, no null and no exceptions, and `if` always has an `else`. Files end with `.ry`; `rythme file.ry` runs one. This file is the whole language in one text: read it all before writing Rythme. Every example here runs as shown (`rythme help llms` prints this file; it is generated from the sources of Rythme and tested).

Work in a loop: write the code, run `rythme check --format json file.ry`, apply the fixes it gives (or `rythme fix file.ry`), read the help of each diagnostic, run `rythme test --format json`, and repeat until there are no errors. Never guess a function name: they are all listed below.

## The rules that differ from Python, JavaScript and Ruby

1. No loops. Work on whole lists with `map`, `filter`, `fold`, `each`, `sum`, ranges (`1..10`), or write a recursive function. `for`, `while`, `break` do not exist.
2. Values never change. A name is given a value once in a scope (`x = 1`, then `x = 2` is error E011). No `+=`, no `x++`, no `xs[0] = 1`. Functions such as `push(xs, 4)`, `put(record, "k", v)` or `set_at(xs, 0, 9)` give a new value and leave the old one as it was: give the result a name.
3. No `return`: the last expression of a function is its result. To stop early with an error, give back `Err(...)` and use `?`.
4. No null: a value that may be missing is `Some(x)` or `None` (an Option); an operation that may fail gives `Ok(x)` or `Err(message)` (a Result). Handle them with `match`, `unwrap_or`, or `?`.
5. No exceptions: errors are values (`Err(...)`); `fail("message")` stops the whole program.
6. `if` is an expression and always has an `else`: `if x > 0 then "positive" else "other"`. Chain with `else if` (not `elif`). For several statements in a branch, use `do ... end`.
7. Functions: `fn name(x) = expression` on one line, or `fn name(x)` with the body on the next lines, finished by `end`. Anonymous: `fn(x) = x * 2`. Not `def`, `function`, `lambda` or `=>`.
8. Texts are written with `"` and put values in with braces: `"total: {x}"`. No f-strings, no `%` or `.format`, and `"a" + 1` is an error: write `"a{1}"` or `"a" + text(1)`.
9. Logic is written with words: `and`, `or`, `not`. Equality: `==` and `!=` (not `===`, `<>`). Booleans are `true` and `false` (lowercase).
10. No braces or colons for blocks: a function body, `match`, `do` and blocks given to calls end with `end`.
11. `value.f(args)` calls `f(value, args)`: `xs.map(f)` is `map(xs, f)`, `"hi".upper()` is `upper("hi")`, `xs.len` is `len(xs)`. Pipes too: `xs |> map(f)`.
12. Choose by shape with `match value ... end` and cases `pattern -> result`.
13. Records are `{name: "Ada", age: 36}`; read with `p.name`; a changed copy is `{..p, age: 37}`. Your own types: `type Shape = Circle(radius) | Rect(w, h)`. No classes, no `self`, no `new`.
14. Tests are part of the language: `test "name" = assert_eq(actual, expected)`, run by `rythme test`.
15. Several files: `use geometry` (the file `geometry.ry`), then `geometry.area(2)`. Names starting with `_` stay private to their file. No `import`.
16. Comments start with `#`; `##` above a function or a type documents it.
"#;

const SYNTAX: &str = r#"
## The language by example

Each example is a whole program; the block after it is what it prints.

### Values, names and texts

```rythme
name = "Ada"
age = 36
print("Hello, {name}! Next year: {age + 1}")
big = 2 ** 100
print(big, 7 / 2, 7 // 2, 7 % 2)
print(type_of(3.5), type_of("x"), type_of([1]), type_of(:ok))
```

```text
Hello, Ada! Next year: 37
1267650600228229401496703205376 3.5 3 1
Float Text List Symbol
```

Whole numbers (`Int`) have no limit; `/` gives a `Float`, `//` a whole division. Texts are `Text`, with `{...}` inside to insert any expression (`\{` for a brace). A text may span several lines.

### Functions

```rythme
fn square(x) = x * x
fn describe(n)
  kind = if n % 2 == 0 then "even" else "odd"
  "{n} is {kind}"
end
print(square(4), describe(7))
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)
fn sign(n) if n < 0 = "negative"
fn sign(0) = "zero"
fn sign(_) = "positive"
print(fib(10), sign(-3), sign(0))
double = fn(x) = x * 2
print(double(21))
fn adult?(person) = person.age >= 18
print(adult?({age: 20}))
```

```text
16 7 is odd
55 negative zero
42
true
```

Several definitions of a function are tried in order (patterns as parameters, `if` for a condition). A name may end with `?`.

### Lists, pipes and blocks

```rythme
xs = [3, 1, 2]
print(xs[0], xs[-1], len(xs), xs + [9], [0, ..xs])
print(xs |> map(fn(x) = x * 10) |> sort)
print(xs.filter(fn(x) = x > 1).sum)
print(fold(xs, 0, fn(acc, x) = acc + x))
print(1..5, (1..10).step_by(3))
squares = xs.map do |x| x * x end
print(squares)
xs.each do |x|
  print("item {x}")
end
print(xs.select { |x| x >= 2 })
```

```text
3 2 3 [3, 1, 2, 9] [0, 3, 1, 2]
[10, 20, 30]
5
6
[1, 2, 3, 4, 5] [1, 4, 7, 10]
[9, 1, 4]
item 3
item 1
item 2
[3, 2]
```

`x |> f(y)` is `f(x, y)`. A block `do |x| ... end` (or `{ |x| ... }`) is a function given as the last argument.

### Conditions, `match` and patterns

```rythme
fn size(n) = if n < 10 then "small" else if n < 100 then "medium" else "large"
print(size(5), size(50), size(500))
fn describe(x) = match x
  0 -> "zero"
  1 | 2 | 3 -> "small"
  4..9 -> "a few"
  n if n < 0 -> "negative"
  _ -> "many"
end
print(describe(2), describe(7), describe(-1), describe(40))
fn first_two(list) = match list
  [] -> "empty"
  [x] -> "only {x}"
  [x, y, ..rest] -> "{x} and {y}, then {len(rest)} more"
end
print(first_two([]), first_two([1]), first_two([1, 2, 3, 4]))
fn greet(name)
  if name == "" then
    "nobody"
  else
    "hi {name}"
end
print(greet("Bo"))
```

```text
small medium large
small a few negative many
empty only 1 1 and 2, then 2 more
hi Bo
```

### Records and your own types

```rythme
ada = {name: "Ada", born: 1815}
older = {..ada, born: 1816}
print(ada.name, older, keys(ada))
{name, born} = ada
print("{name} was born in {born}")
type Shape = Circle(radius) | Rect(width, height)
fn area(shape) = match shape
  Circle(r) -> 3 * r * r
  Rect(w, h) -> w * h
end
print([Circle(1), Rect(2, 3)] |> map(area))
print(Circle(2).radius)
```

```text
Ada {name: "Ada", born: 1816} ["name", "born"]
Ada was born in 1815
[3, 6]
2
```

`rythme check` reports a `match` that forgets a shape.

### Errors are values: Option, Result and `?`

```rythme
fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)
match divide(10, 4)
  Ok(x) -> print("result: {x}")
  Err(e) -> print("error: {e}")
end
fn average(a, b)
  x = parse_float(a)?
  y = parse_float(b)?
  Ok((x + y) / 2)
end
print(average("1", "2"), average("1", "x"))
print(unwrap_or(first([]), 0), get([1, 2], 5), parse_int("42"))
```

```text
result: 2.5
Ok(1.5) Err("\"x\" is not a number")
0 None Ok(42)
```

`?` takes the value out of `Ok(x)` or `Some(x)`, or stops the function and gives back the `Err`/`None`.

### Several steps, recursion

```rythme
total = do
  a = 10
  b = 20
  a + b
end
print(total)
fn count_down(0) = "liftoff"
fn count_down(n) = count_down(n - 1)
print(count_down(100000))
fn sum_to(n, acc) = if n == 0 then acc else sum_to(n - 1, acc + n)
print(sum_to(1000, 0))
```

```text
30
liftoff
500500
```

A recursive call as the very last step can repeat without limit.

### Tests

```rythme
fn add(a, b) = a + b
test "adds" = assert_eq(add(1, 2), 3)
test "several checks"
  assert_eq(add(0, 0), 0)
  assert(add(1, 1) > 1, "positive")
end
print(add(2, 2))
```

```text
4
```

`rythme test file.ry` (or `rythme test` in a project, or `rythme test folder/`) runs the tests; running the file does not.

### Several files and projects

In `geometry.ry`: `fn area(r) = r * r` (names starting with `_` stay private). In `main.ry`: `use geometry` then `print(geometry.area(2))`. Also `use shapes/circle` (a folder) and `use "lib.ry" as l`. `rythme new name` creates a project (`rythme.toml`, `src/main.ry`, `tests/`); `rythme run`, `rythme test` and `rythme check` then work without a file name.

### Written types (optional)

`fn area(width: Int, height: Int) -> Int = width * height`, `total: Float = 0.0`, `List(Int)` or `[Int]`, `Option(Text)`, `Result(Int, Text)`, `{name: Text}`, `fn(Int) -> Int`. They are never needed; `rythme check` verifies them (E027).

### Input and output

`print(a, b)` (or `puts`) writes a line; `input("prompt")` reads one; `input_lines()` reads all of the standard input; `args()` and `options()` give the command line; `read_file(path)` and `write_file(path, text)` give a `Result`; `exit(code)` stops.
"#;

const MISTAKES: &str = r#"
## Habits from other languages, and the Rythme way

| Wrong (other languages) | Right in Rythme |
|---|---|
| `for x in xs: print(x)` / `xs.forEach(...)` | `xs.each do \|x\| print(x) end` or `each(xs, fn(x) = print(x))` |
| `for i in range(10)` | `(0..9).each do \|i\| ... end` (`1..10` includes both ends) |
| `total = 0; for x in xs: total += x` | `total = sum(xs)` or `fold(xs, 0, fn(acc, x) = acc + x)` |
| `while n > 0: ...` | a recursive function: `fn countdown(n) = if n == 0 then ... else countdown(n - 1)` |
| `x = x + 1` (again in the same scope), `x += 1`, `x++` | a new name: `next_x = x + 1` |
| `xs.append(4)` / `xs.push(4)` alone on a line | `ys = push(xs, 4)` (the result is a new list; W008 warns when it is lost) |
| `xs[0] = 9`, `p.age = 37` | `set_at(xs, 0, 9)`, `{..p, age: 37}` |
| `return x` | `x` as the last line of the function |
| `def f(x):` / `function f(x) {` / `const f = (x) => ...` | `fn f(x) = ...` or `fn f(x)` ... `end` |
| `lambda x: x * 2` / `x => x * 2` | `fn(x) = x * 2` |
| `null`, `nil`, `None` as "nothing", `undefined` | `None` (with `Some(x)`): an Option |
| `try: ... except: ...`, `throw` | `Ok(x)` / `Err(e)` and `match`, `?`; `fail("...")` stops the program |
| `if x > 0:` / `if (x > 0) { ... }` | `if x > 0 then ... else ...` |
| `elif` / `elsif` | `else if` |
| `a ? b : c` | `if a then b else c` |
| `&&`, `\|\|`, `!x` | `and`, `or`, `not x` |
| `===`, `<>` | `==`, `!=` |
| `True`, `False` | `true`, `false` |
| `f"hi {name}"`, `"hi %s" % name`, `"hi " + name + "!"` with numbers | `"hi {name}"` |
| `len(xs)` / `xs.length` / `xs.size()` | `len(xs)` or `xs.len` (`length` and `size` work too) |
| `str(3)` / `String(3)` / `3.toString()` | `text(3)` |
| `int("3")` that may fail | `parse_int("3")` gives `Ok(3)` or `Err(...)`; `int(3.7)` cuts a number |
| `s.toUpperCase()`, `s.strip()` | `upper(s)`, `trim(s)` (`upcase`, `strip` work too) |
| `", ".join(xs)` | `join(xs, ", ")` |
| `s.split(",")` | `split(s, ",")` |
| `d["key"]` on a missing key, `d.get("k", 0)` | `get(d, "key")` gives an Option: `unwrap_or(get(d, "key"), 0)` |
| `{"a": 1}` as a dictionary with any keys | a record `{a: 1}` for names, or `dict([["a", 1]])` |
| `class Point: ...` | `type Point = Point(x, y)` or a record `{x: 1, y: 2}` |
| `import math` / `require "x"` | `use math` (a file `math.ry`), then `math.f(...)` |
| `print("a", end="")` | `print("a")` writes a line; build the text first |
| `switch x { case 1: ... }` | `match x` with `1 -> ...` cases, then `end` |
| `{ ... }` for a block | `do ... end` |
"#;

const TOOLS: &str = r#"
## Commands for the loop

- `rythme file.ry [args]` or `rythme run file.ry`: check, then run. Add `--format json` for one JSON document with `stdout`, `stderr`, `exit_code` and the `diagnostics`; `--sandbox` keeps the program away from files, folders and environment variables.
- `rythme check --format json file.ry ...`: every error and warning, without running. Each diagnostic: `code` (stable), `severity`, `message`, `file`, `range` (lines and columns from 1, end excluded), `labels`, `notes`, `help` (what to write), `fixes` (`title`, `safe`, `edits` with `range` and `new_text`). Exit code 0 when there are no errors.
- `rythme fix file.ry`: applies the safe fixes until none is left; `--dry-run` shows the diff; `--all` also applies the guesses (misspelt names).
- `rythme test --format json [file.ry | folder]`: each test with `passed` and its `failure`.
- `rythme explain E003` (`--format json`): the long explanation of a code.
- `rythme help map` (`--format json`): the documentation of a built-in function, a keyword or a topic; `rythme help builtins` lists them all.
- `rythme fmt file.ry`: the official layout. `rythme steps file.ry "expr"`: the evaluation step by step.
- `rythme mcp`: the same tools for AI assistants, as a Model Context Protocol server (stdio).

Exit codes: 0 success, 1 an error in the program or a failed test, 2 a wrong use of the command, or the code given to `exit(code)`.
"#;
