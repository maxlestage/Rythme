# Teaching with Rythme

*[Version française →](enseigner.md) · [En español →](teaching.es.md) · [Auf Deutsch →](teaching.de.md) · [Em português →](teaching.pt.md) · [中文 →](teaching.zh.md)*

Rythme was designed for the classroom: a small language, with messages that
teach, so that students spend their time on ideas rather than on syntax.
This guide covers what a course needs: an outline, exercises checked and
graded automatically, ways to show how code runs, and a policy for AI
assistants.

## Why it fits a course

- **Little to learn before writing real programs.** 15 keywords, one way to
  define a function, one way to choose (`if`/`match`), no classes, no loops,
  no mutation. Students can read the whole syntax on one page
  (`rythme help syntax`).
- **The interpreter is a teaching assistant.** Errors point at the exact
  place, explain the problem and propose a fix. Mistakes carried over from
  other languages (`for`, `return`, `x += 1`, `null`...) get an explanation
  of the functional way. `rythme explain E0xx` gives the long version with
  examples. `rythme check` finds problems without running the program,
  including operations that would always fail, such as `"age: " + 36`.
- **Functional thinking from day one.** Immutability, functions as values,
  recursion, pattern matching, algebraic types, errors as values (`Option`,
  `Result`): the concepts of Haskell, OCaml, Elm, Elixir or Rust, without
  their syntax or a type system to fight on the first day. Types are
  optional: the checker finds them by itself, and once the course gets
  there, students write them (`fn area(w: Int, h: Int) -> Int`,
  `type Tree(a)`) and the checker verifies them.
- **Testing is built in.** `test "..." = assert_eq(actual, expected)` next to
  the code, `rythme test file.ry` to run. Exercises can ship with their tests.
- **A library students recognize.** Texts, lists, dictionaries, dates, CSV,
  regular expressions, and vectors, statistics and slopes for a unit on
  machine learning (`rythme help ml`). Students who know Ruby or Python find
  many functions under the names they know (`upcase`, `strip`,
  `each_slice`...): see `rythme help ruby` and
  [the library, compared](stdlib-audit.md).
- **In your students' language**, and **easy to install, free, open**: one
  program, no dependency, MIT license, for Linux, macOS and Windows, or
  nothing to install at all with the playground of the website.

## A possible course (12 sessions)

| # | Theme | Rythme concepts | Exercises | Example |
|---|-------|-----------------|-----------|---------|
| 1 | Expressions and values | numbers, texts, `print`, `{...}` in texts, the REPL | 01, 03 | [`hello.ry`](../examples/hello.ry) |
| 2 | Functions | `fn f(x) = ...`, blocks with `end`, calling, `help(f)` | 02 | |
| 3 | Choosing | `if ... then ... else`, booleans, `and`/`or`/`not` | 04–06 | |
| 4 | Recursion | base case, definitions by case `fn fib(0) = 0`, guards | 07–10 | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Lists | literals, ranges `1..10`, `[x, ..rest]` patterns | 11–13 | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Higher-order functions | `map`, `filter`, `fold`, anonymous functions, `\|>` | 14–19 | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Records | `{name: ...}`, field access, `{..r, x: 1}` | 20–21 | [`students.ry`](../examples/students.ry) |
| 8 | Your own types | `type`, constructors, `match`, forgotten cases (W002), types written in the code | 22–23 | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Recursive types | trees, structural recursion, types with parameters (`type Tree(a)`) | 24–25 | [`tree.ry`](../examples/tree.ry) |
| 10 | Errors as values | `Option`, `Result`, `?`, `unwrap_or` | 26–28 | [`results.ry`](../examples/results.ry) |
| 11 | Tests and modules | `test`, `assert_eq`, `use` | 30 | [`testing.ry`](../examples/testing.ry) |
| 12 | Project | text processing, data analysis, a small game | 29 | [`words.ry`](../examples/words.ry) |

Tail recursion (session 4 or 9) deserves a moment: the error E014
("too many nested calls") explains it, and `rythme explain E014` shows the
accumulator technique.

After session 7, two or three sessions can make a **unit on artificial
intelligence**: vectors, statistics, an artificial neuron, gradient
descent, nearest neighbours and minimax (exercises 31 to 36,
`rythme help ml`, and the examples
[`perceptron.ry`](../examples/perceptron.ry) and [`knn.ry`](../examples/knn.ry)).

## Exercises with automatic checking

Give students a file with the tests already written, and each function to
write as `fail("to do")`. Here is one a student completed, with a classic
mistake:

```rythme
## Returns the largest element of a non-empty list.
fn largest(list) = fold(list, 0, max)

test "one element" = assert_eq(largest([7]), 7)
test "several" = assert_eq(largest([3, 9, 2]), 9)
test "negative" = assert_eq(largest([-3, -1, -2]), -1)
```

`rythme test largest.ry` reports what passes and, for each failure, the
expected and obtained values:

```text
Running 3 test(s)
  ✓ one element
  ✓ several
  ✗ negative
error[E015]: assertion failed: the values are different
 --> largest.ry:6:19
  |
6 | test "negative" = assert_eq(largest([-3, -1, -2]), -1)
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: got:      0
      expected: -1
  = more about this error: `rythme explain E015`

3 test(s): 2 passed, 1 failed (1 ms)
```

Thirty-six such exercises are ready: [**the exercises**](exercises/README.md),
graded from ★☆☆ to ★★★ and following the course above, each with a
statement, hints and tests, in every language of the messages, with
reference solutions for teachers. They are built into `rythme`:

```sh
rythme exercise list                     # the exercises
rythme exercise start 7                  # copies 07-recursion.ry into the current folder
rythme exercise check 07-recursion.ry    # runs its tests, with advice, and proposes the next one
```

`rythme exercise check` also says when the tests of a built-in exercise
were changed: the exercise is solved only when its own tests pass. Your
own exercises can follow the same pattern (a statement and hints in
comments, `fail("to do")` in the functions, the tests after a line
`# --- Tests (do not change them) ---`); `rythme exercise check` works on
any file with tests.

## Grading with `--format json`

`rythme test` takes a folder: with one file (or one folder) per student, a
single command runs every test of the class.

```sh
rythme test --sandbox --format json submissions/ > results.json
```

- **`--format json`** prints one JSON document instead of the messages for
  people: for each file, `tests`, `failed` and `broken` (a file that does
  not compile, or stops before its tests); for each test, its `name`,
  `file`, `passed` and `failure`; then a `summary`. The keys never change
  ([the schema](rythme-output.schema.json)); the messages follow `--lang`.
- **`--sandbox`** keeps the programs away from files, folders, environment
  variables and the network. There is no time limit: run the command under
  one (on Linux, `timeout 60 rythme test ...`).
- **Students can change the tests** of their file. For a grade, run your
  own copy: `rythme exercise check` says when the tests of a built-in
  exercise were changed, and the reference solutions are public (in the
  repository): for graded work, write exercises of your own.
- `rythme check --format json` adds the warnings, each with its stable code
  (W001 an unused name, W002 a forgotten case...), and `rythme fmt --check`
  says whether the code has the official layout.

Rythme reads JSON itself. This program turns the results into grades:

```rythme
# grades.ry
fn grade(f) = if f.broken then "{f.file}: does not run" else "{f.file}: {f.tests - f.failed}/{f.tests}"

results = read_file(args()[0]) |> unwrap |> parse_json |> unwrap
each(results.files, fn(f) = print(grade(f)))
```

`rythme grades.ry results.json` prints:

```text
submissions/ada.ry: 3/3
submissions/alan.ry: 2/3
submissions/grace.ry: does not run
```

## Showing how code runs

`rythme steps` shows a calculation one step at a time, in the substitution
model: each call is replaced by the body of its function, until only the
result is left. Recursion, accumulators and tail calls become visible.

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

fib(4)
```

`rythme steps fib.ry` shows:

```text
fib(4)
  → fib(3) + fib(2)
  → fib(2) + fib(1) + fib(2)
  → fib(1) + fib(0) + fib(1) + fib(2)
  → 1 + fib(0) + fib(1) + fib(2)
  → 1 + 0 + fib(1) + fib(2)
  → 1 + fib(1) + fib(2)
  → 1 + 1 + fib(2)
  → 2 + fib(2)
  → 2 + (fib(1) + fib(0))
  → 2 + (1 + fib(0))
  → 2 + (1 + 0)
  → 2 + 1
  → 3
```

`rythme steps fib.ry "fib(6)"` shows another expression, `:steps fib(6)`
does it in the interactive mode, and the **Steps** button in the
playground, where *Share* makes a link that opens on the same step: one
link to project, or to send to the class.

`rythme debug file.ry` pauses a program where you ask (`break 12`,
`break fib`) and shows the values of its names. `where` lists the calls in
progress with their arguments, and says how many calls of the same
function are waiting: the stack of a recursion, live. A call in tail
position says that it took the place of its caller. In VS Code, `F5` on a
`.ry` file starts the same debugger, with breakpoints in the margin. The
[manual](reference.md#79-the-debugger-rythme-debug) shows a whole session.

## Without installing anything: the playground

The playground of the [website](https://maxlestage.github.io/Rythme/) runs
the real interpreter in the browser, with the same messages and help:
students can start on any computer, tablet or phone.

- **Run**, **Check**, **Test**, **Format** and **Steps** work as the
  commands do; paste an exercise and press **Test**.
- **Share** makes a link that holds the program itself (nothing is sent to
  a server): hand out a starting point, or the program of the lesson.
  **Seed** gives every student the same draws of `random`.
- Installed as an app (**Install app** in the menu, or *Add to Home Screen*
  on iPhone and iPad), the site and its playground work offline, which
  helps in a room with a poor network. The code being written stays in
  the browser.
- In the browser, files cannot be read or written, `input()` gives an
  empty text, and nested calls stop at 600 (tail calls have no limit).

## In your students' language

- Every message, help topic, explanation and page of `rythme learn` exists
  in English, French, Spanish, German, Portuguese and Chinese. Rythme
  follows the language of the system; `--lang es` or the variable
  `RYTHME_LANG=es` choose another one.
- Keywords, names of functions and commands stay the same in every
  language: a program written in one class runs everywhere, and
  `rythme help` accepts the names of topics in each language
  (`rythme help listas`).
- The exercises follow the language too: `rythme --lang de exercise start 7`
  writes `07-rekursion.ry`, in German.
- The website, its playground and this guide exist in the same languages.

## AI assistants in class

Students have assistants at hand. Rythme does not try to detect them; here
is what it offers, and a policy that a course can adapt.

**What Rythme offers.** Assistants were trained before Rythme existed:
left alone, they write habits of Python or JavaScript (`for`, `return`,
`x += 1`), which Rythme rejects with a message that says what to write.
Given `rythme help llms`, the whole language in one text (also on the
website's *AI* page), they write correct Rythme. `rythme mcp` lets an
assistant run, check and test code by itself, in a sandbox. In the
playground, *Copy for AI* copies the program and its output or its errors,
ready for a chat. `rythme fix` applies the fixes that are certain
(`&&` → `and`, `let x = 3` → `x = 3`), and `rythme fix --dry-run` only
shows them.

**A possible policy.**

- **First weeks: students write the code.** The point is to build the
  reflexes, and the messages and `rythme explain` already explain each
  mistake. An assistant may explain a message or an idea, not write the
  solution.
- **`rythme fix` once the message is read**: it changes only what is
  certain, the same fix the message proposes.
- **Projects: allowed, and said.** Give the assistant `rythme help llms`,
  ask students to say what they used it for, and to explain their code.
- **Assess understanding, not only results.** Tests check that a program
  works, not who wrote it. Add short checks in class: predict the steps of
  `rythme steps`, the output of a program, or the fix of a message, without
  a computer; or an exercise done on the spot in the playground.
- **For your own work**, an assistant given `rythme help llms` can draft new
  exercises in the format above. Check each one with a solution of your own
  and `rythme test` before handing it out.

## Practical tips

- Start with the interactive mode (`rythme`): every line shows its value.
  `:doc map` shows the documentation of a function, Tab completes names.
- `rythme learn` is a guided tour that students can follow alone.
- `rythme help builtins` lists every built-in function by category, and
  every command answers `--help` (`rythme test --help`).
  `rythme completions bash` (or `zsh`, `fish`, `powershell`) completes the
  commands in the terminal.
- Install the [VS Code extension](../editors/vscode): colors, snippets, the
  problems shown as students type with their quick fixes, the
  documentation under the mouse, and the debugger. Other editors can use
  the language server too ([`editors/README.md`](../editors/README.md)).
- `rythme fmt` gives every file the same layout: students can run it (or
  `rythme fmt --check .`) before handing in their work.
- For a larger project, `rythme new NAME` creates a folder with `src/`,
  `tests/` and `rythme.toml`; `rythme test` alone then runs every test of
  the project, and `rythme add` brings in a package of the
  [package index](reference.md#711-the-package-index).
