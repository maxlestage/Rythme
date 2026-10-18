# Teaching with Rythme

*[Version française →](enseigner.md)*

Rythme was designed for the classroom: a small language, with messages that
teach, so that students spend their time on ideas rather than on syntax.

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
  their syntax or a type system to fight on the first day.
- **Testing is built in.** `test "..." = assert_eq(actual, expected)` next to
  the code, `rythme test file.ry` to run. Exercises can ship with their tests.
- **In your students' language.** Every message exists in English, French and
  Spanish (`--lang fr`, `--lang es`).
- **Easy to install, free, open.** One program, no dependency, MIT license,
  runs on Linux, macOS and Windows.

## A possible course (12 sessions)

| # | Theme | Rythme concepts | Example |
|---|-------|-----------------|---------|
| 1 | Expressions and values | numbers, texts, `print`, `{...}` in texts, the REPL | [`hello.ry`](../examples/hello.ry) |
| 2 | Functions | `fn f(x) = ...`, blocks with `end`, calling, `help(f)` | |
| 3 | Choosing | `if ... then ... else`, booleans, `and`/`or`/`not` | |
| 4 | Recursion | base case, definitions by case `fn fib(0) = 0`, guards | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Lists | literals, ranges `1..10`, `[x, ..rest]` patterns | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Higher-order functions | `map`, `filter`, `fold`, anonymous functions, `|>` | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Records | `{name: ...}`, field access, `{..r, x: 1}` | [`students.ry`](../examples/students.ry) |
| 8 | Your own types | `type`, constructors, `match`, forgotten cases (W002) | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Recursive types | trees, structural recursion | [`tree.ry`](../examples/tree.ry) |
| 10 | Errors as values | `Option`, `Result`, `?`, `unwrap_or` | [`results.ry`](../examples/results.ry) |
| 11 | Tests and modules | `test`, `assert_eq`, `use` | [`testing.ry`](../examples/testing.ry) |
| 12 | Project | text processing, data analysis, a small game | [`words.ry`](../examples/words.ry) |

Tail recursion (session 4 or 9) deserves a moment: the error E014
("too many nested calls") explains it, and `rythme explain E014` shows the
accumulator technique.

## Exercises with automatic checking

Give students a file with the tests already written:

```rythme
## Returns the largest element of a non-empty list.
fn largest(list) = fail("to do")

test "one element" = assert_eq(largest([7]), 7)
test "several" = assert_eq(largest([3, 9, 2]), 9)
test "negative" = assert_eq(largest([-3, -1, -2]), -1)
```

`rythme test exercise.ry` reports what passes and, for each failure, the
expected and obtained values.

Thirty such exercises are ready: [**the exercises**](exercises/README.md),
graded from ★☆☆ to ★★★ and following the course above, each with a
statement, hints and tests, in English and French, with reference solutions
for teachers. They are built into `rythme`:

```sh
rythme exercise list                     # the exercises
rythme exercise start 7                  # copies 07-recursion.ry into the current folder
rythme exercise check 07-recursion.ry    # runs its tests, with advice, and proposes the next one
```

## Practical tips

- Start with the interactive mode (`rythme`): every line shows its value.
  `:doc map` shows the documentation of a function, Tab completes names.
- `rythme learn` is a guided tour that students can follow alone.
- `rythme help builtins` lists every built-in function by category.
- Install the [VS Code extension](../editors/vscode): colors, snippets, and
  the problems shown as students type, with the documentation under the
  mouse. Other editors can use the language server too
  ([`editors/README.md`](../editors/README.md)).
- `rythme fmt` gives every file the same layout: students can run it (or
  `rythme fmt --check .`) before handing in their work.
- `rythme steps fib.ry "fib(4)"` shows a calculation step by step, in the
  substitution model (`fib(4)` → `fib(3) + fib(2)` → ...): recursion,
  accumulators and tail calls become visible. `:steps` does it in the
  interactive mode, and the **Steps** button in the playground.
- Nothing to install on the students' computers? The playground of the
  [website](https://maxlestage.github.io/Rythme/) runs the real interpreter
  in the browser.
