# Exercises

*[Version française →](README.fr.md)*

Thirty graded exercises for a first programming course, from the first
values to recursion, lists, pipelines, records, types and `match`, `Option`
and `Result`, and tests. Each exercise is one `.ry` file, in
[English](en) and in [French](fr), with:

- a **statement** and **hints**, in comments at the top;
- the **functions to write**, whose body is `fail("to do")` for now;
- **tests** that `rythme test` runs: the exercise is solved when they all pass.

The levels go from ★☆☆ (a few lines, one new idea) to ★★★ (several functions
that work together).

## For students

```sh
rythme exercise list                     # the exercises
rythme exercise start 1                  # copies 01-values.ry into the current folder
rythme exercise check 01-values.ry       # runs its tests and says what to do next
```

Open the file, read the statement, replace each `fail("to do")` with your
code, and check again, until every test passes: `rythme exercise check` then
proposes the next exercise. Do not change the tests. `rythme test
01-values.ry` runs the same tests, and `rythme check 01-values.ry` finds
mistakes without running anything. With `--lang fr` (or when your system is
in French), `rythme exercise` gives the French version of each exercise.

The exercises are built into `rythme`: nothing else to download. Without
installing anything, copy an exercise into the playground of the
[website](https://maxlestage.github.io/Rythme/) and press *Test*.

## The exercises

| # | Exercise | Ideas | Level |
|---|----------|-------|-------|
| 01 | [First values](en/01-values.ry) | names, numbers, `{...}` in texts | ★☆☆ |
| 02 | [Texts](en/02-texts.ry) | functions, `upper`, `+` on texts, `text[0]` | ★☆☆ |
| 03 | [Arithmetic](en/03-arithmetic.ry) | `/`, `//`, `%`, `sqrt`, Int and Float | ★☆☆ |
| 04 | [Choosing with `if`](en/04-choosing.ry) | `if ... then ... else`, `else if` | ★☆☆ |
| 05 | [True or false](en/05-logic.ry) | comparisons, `and`, `or`, `not` | ★☆☆ |
| 06 | [One definition per case](en/06-cases.ry) | definitions by case, guards, `_` | ★☆☆ |
| 07 | [Recursion](en/07-recursion.ry) | base case and recursive case | ★★☆ |
| 08 | [The digits of a number](en/08-digits.ry) | recursion on numbers, a helper function | ★★☆ |
| 09 | [Accumulators and tail calls](en/09-accumulators.ry) | accumulators, tail calls, E014 | ★★☆ |
| 10 | [Divisors and prime numbers](en/10-divisors.ry) | Euclid, trying candidates | ★★☆ |
| 11 | [Taking lists apart](en/11-list-patterns.ry) | `[]`, `[x, ..rest]`, forgotten cases (W002) | ★★☆ |
| 12 | [Building lists](en/12-building-lists.ry) | `[x, ..rest]` to build, values never change | ★★☆ |
| 13 | [The list functions](en/13-library.ry) | ranges, `map`, `filter`, `sum`, `sort`, `take` | ★★☆ |
| 14 | [Anonymous functions](en/14-anonymous-functions.ry) | `fn(x) = ...`, `all`, `count` | ★★☆ |
| 15 | [Functions are values](en/15-functions-as-values.ry) | functions as arguments and results, closures | ★★☆ |
| 16 | [Folding a list into one value](en/16-fold.ry) | `fold` | ★★☆ |
| 17 | [Pipelines](en/17-pipelines.ry) | `\|>`, `split`, `join` | ★★☆ |
| 18 | [Working with texts](en/18-text-processing.ry) | `chars`, `lower`, `replace`, `capitalize` | ★★☆ |
| 19 | [Sorting](en/19-sorting.ry) | insertion sort, merge sort | ★★★ |
| 20 | [Records](en/20-records.ry) | `{name: ...}`, fields, `{..r, x: 1}` | ★★☆ |
| 21 | [Lists of records](en/21-lists-of-records.ry) | `sort_by`, `max_by`, `group_by`, `map_values` | ★★☆ |
| 22 | [Your own types](en/22-own-types.ry) | `type`, constructors, one definition per constructor | ★★☆ |
| 23 | [Choosing with `match`](en/23-match.ry) | `match` on constructors, lists and records | ★★☆ |
| 24 | [Trees](en/24-trees.ry) | recursive types, search trees | ★★★ |
| 25 | [A small calculator](en/25-expressions.ry) | a type for expressions, evaluating, simplifying | ★★★ |
| 26 | [A value that may be missing: Option](en/26-option.ry) | `Some`, `None`, `find`, `match` | ★★☆ |
| 27 | [An operation that may fail: Result](en/27-result.ry) | `Ok`, `Err`, `parse_int`, guards | ★★☆ |
| 28 | [Passing errors on with `?`](en/28-question-mark.ry) | `?` | ★★★ |
| 29 | [A small project: counting words](en/29-word-counts.ry) | everything together | ★★★ |
| 30 | [Tests find bugs](en/30-testing.ry) | reading test failures, fixing bugs, writing a test | ★★★ |

## For teachers

- The exercises follow the [course outline](../teaching.md#a-possible-course-12-sessions):
  01–06 for sessions 1 to 3, 07–10 for recursion, 11–19 for lists and
  higher-order functions, 20–21 for records, 22–25 for types, 26–28 for
  errors as values, 29–30 for tests and a small project.
- Reference solutions are in the hidden folder
  [`.solutions/`](.solutions) (`en/` and `fr/`), with the same tests: the
  tests of Rythme check that each solution passes every test of its exercise,
  and that each exercise fails its tests until it is solved.
- Your own exercises can follow the same pattern: a statement and hints in
  comments, `fail("to do")` in the functions, and the tests after a line
  `# --- Tests (do not change them) ---`. `rythme exercise check` works on
  any file with tests; it also notices when the tests of a built-in exercise
  were changed.
