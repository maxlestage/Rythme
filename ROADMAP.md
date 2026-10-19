# Roadmap to Rythme 1.0

*En français : la feuille de route vers Rythme 1.0. Chaque point a été fait,
testé et fusionné l'un après l'autre ; toutes les cases sont cochées : c'est
Rythme 1.0 (voir [`CHANGELOG.md`](CHANGELOG.md)). Des idées pour la suite,
sans promesse, sont à la fin.*

**Every item is done: this is Rythme 1.0.** What it contains is in
[`CHANGELOG.md`](CHANGELOG.md), what 1.x guarantees in the
[stability promise](README.md#stability-promise), and ideas for what comes
next, without promises, under [After 1.0](#after-10).

Rythme 0.1 had the language, the checker, the formatter, the language server,
the playground and the manual. This list was what remained for a **complete**
language: one that universities everywhere and teams at work can adopt. Items
were worked on in order, one at a time, each one fully (code, checks, messages
in every supported language, `rythme help`, the reference manual in English
and French, tests, CI green) before it was checked.

The rules of the language stay the rules of every item: simpler than Python,
purely functional, safe like Rust (no `null`, no exceptions), help everywhere.
No new keyword or syntax unless an item says so and nothing simpler works.

## Language

- [x] **Modules with names.** `use geometry` (or `use shapes/circle`,
  `use "geometry.ry" as geo`), then `geometry.area(2)`; names starting with
  `_` stay private to their file; the checker, the formatter, the language
  server (completion after `use ` and `geo.`, a quick fix adding a missing
  `use`) and the manual know it. Plain `use "x.ry"` keeps working.
- [x] **Ruby style.** Blocks given to calls (`xs.map do |x| x * 2 end`,
  `xs.select { |x| x > 2 }`), method chains without parentheses and over
  several lines (`xs.select(even?).sum`; a field wins over a function),
  names ending with `?` (`even?`, `fn adult?(p)`) with a precise rule
  against the `?` operator, `unless ... then ... else`, symbols (`:ok`),
  `"#{x}"` in texts, and the Ruby names of the library (`puts`, `upcase`,
  `select`, `reduce`, `size`...) as real built-in names; known by the
  checker, the formatter, the language server, the editors' grammars,
  `rythme help ruby` and the manual. Every existing program keeps working.
  - [x] **Calls without parentheses.** `puts "Hello"`, `print x, y`,
    `assert_eq total, 42`, `each xs do |x| ... end` at the start of a
    statement or of a result (a branch of `if`/`unless`, a case of `match`,
    the body after the `=` of a function, a lambda or a test:
    `if ok then puts "yes" else puts "no"`); `puts [1]` and `puts -1` call too, but `x [0]` and `x -1` stay
    an index and a subtraction when the code gives `x` a value (as in Ruby),
    and `rythme fmt` writes those `x[0]` and `x - 1`. Interpreter, checker,
    formatter, language server (signature help), Tree-sitter and VS Code
    grammars, `rythme steps`, `rythme help ruby` and the manual (3.15).
- [x] **Patterns on ranges and texts.** `1..9 -> ...` in `match` (Int and
  Float bounds, exhaustiveness knows nothing new), and a text prefix pattern
  only if it stays simple; the checker reports overlapping or empty ranges.
  Done as `1..9`, `-5..-1`, `0.0..0.5` (both ends included, Int or Float
  values) and `"Mr " + name`, in `match`, function definitions and
  definitions; W004 for empty ranges and ranges handled by the cases before,
  W010 for ranges that overlap.
- [x] **Whole numbers without limit.** Int grows beyond 64 bits instead of
  E012 (the way Python does), with the small-number fast path kept; `pow`,
  `gcd`, `text`, `parse_int`, JSON and comparisons with Float are exact.
- [x] **Types of parameters, inferred.** `rythme check` infers what a
  function's parameters must be from how it uses them, and reports a call
  that always fails (`area("big")` when `area` multiplies), with zero false
  positives; the language server shows the inferred signature on hover.

## Library

- [x] **Dates and times.** `now()`, `today()`, dates as records
  (`{year, month, day, hour, minute, second}`), ISO 8601 parsing and
  printing, differences and additions in days/seconds, day of the week;
  UTC plus the local offset when the system gives it; no dependency if
  possible.
- [x] **Dictionaries and sets.** Keys of any comparable value (not only
  texts): `dict(...)`, `get`, `put`, `has`, `keys`... on them, and sets
  (`set(list)`, `union`, `intersection`, `difference`), printed and compared
  like the other values, persistent (no mutation), with the checker's types.
- [x] **Text tools.** Searching and splitting with simple patterns (wildcards
  or a small, well-explained regular expression subset), `format` of numbers
  (thousands separators, padding), `lines`/`words` helpers if missing,
  `trim_start`/`trim_end`, case-insensitive comparison.
- [x] **Files and the system.** Listing a folder (`list_folder`), creating
  folders (`create_folder`), file information (`file_info`), deleting one
  file and never a folder (`delete_file`), reading lines of stdin until the
  end (`input_lines`), `exit` codes documented (`rythme help system`),
  command-line options (`options()`: `args` parsed into a record), paths
  joined and split portably (`join_path`, `split_path`).

## Help everywhere, for everyone

- [x] **Messages in Spanish.** The translation system takes any number of
  languages (catalogs instead of two-argument `t!`), with tests that every
  message exists in every language; then every message, help topic and
  `rythme learn` in Spanish (`--lang es`). Done: `t!(en, fr)` stays, each
  other language is a catalog `src/i18n/<code>.rs` checked by
  `tests/i18n.rs` (see CONTRIBUTING); the 30 exercises too (statement,
  hints, test names, the file `rythme exercise start` writes, with a
  reference solution in each language checked by `tests/exercises.rs`).
- [x] **Messages in German, Portuguese and Chinese.** `src/i18n/de.rs`,
  `pt.rs` (Brazilian, "você") and `zh.rs` (simplified Chinese) translate all
  the texts, checked complete by `tests/i18n.rs`; `--lang de|pt|zh`,
  `LANG=de_*|pt_*|zh_*`, topic names in each language, the files of
  `rythme new`, the VS Code setting and its texts, README.de.md, README.pt.md
  and README.zh.md. The `^` under a mistake are placed by display width, so
  Chinese lines stay aligned.
- [x] **The website and the playground in every language.** The site has
  `/es/`, `/de/`, `/pt/` and `/zh/` beside `/fr/` and `/en/`: every text
  (`site/src/i18n/<code>.rs`, a missing one does not compile), the pieces of
  code of the home page, the tour and the playground written in each language
  and run by the tests through the interpreter in that language, a compact
  language menu, `Accept-Language`, manifests, screenshots, service worker,
  sitemap and `hreflang`, offline pages and the static export. The playground
  speaks the language of its page; the guide for teachers is shown in English,
  with a note, where it has no translation; Chinese falls back on the system's
  fonts.
- [x] **Step-by-step evaluation for teaching.** `rythme steps file.ry
  ["expression"]` (`rythme etapes`), `:steps` in the REPL and the Steps
  button of the playground show how an expression is reduced step by step
  (`fib(3)` → `fib(2) + fib(1)` → ...), bounded, in every language.
- [x] **Exercises.** `docs/exercises/` with graded exercises for a first
  course (each with tests that `rythme test` runs and hints), and
  `rythme exercise` (list, start, check) if it stays simple, in every
  language of the messages.

## Tools

- [x] **`rythme doc`.** Documentation of a file or a folder from its `##`
  comments, types and signatures, as Markdown and HTML, linked to the
  built-ins' docs.
- [x] **Better `rythme test`.** A folder of files, a filter by name,
  `--watch`, a summary with times, and property tests
  (`check_all(fn(x) = ..., generator)`) with shrinking of the failing case.
- [x] **Language server extras.** Inferred types as inlay hints, semantic
  highlighting, code lenses to run a test or a file, workspace symbols
  (and folding, selection ranges).
- [x] **A Tree-sitter grammar** for Neovim, Helix and Zed highlighting, with
  its tests, and the Zed extension completed.
- [x] **Projects and packages.** `rythme new name`, a `rythme.toml` manifest,
  dependencies from git or a path with a lock file, `use "package/file.ry"`,
  `rythme run`/`test` at the project root.

## Speed and release

- [x] **Faster where it still hurts.** Building texts with `+` and adding to
  lists in constant amortized time, cheaper calls between local functions;
  measured with `bench/`, no behavior change.
- [x] **Installers.** A `curl | sh` / PowerShell install script using the
  release binaries, a Homebrew formula and a winget manifest in the
  repository, crates.io metadata, and a workflow publishing the VS Code
  extension when a `VSCE_PAT` secret exists (skipped otherwise).
- [x] **Rythme 1.0.** Version 1.0.0 everywhere, a CHANGELOG, a stability
  promise in the README, the manual and the website up to date. Opened as a
  pull request; the `v1.0.0` tag is left to the owner. Done: every package
  (interpreter, macros, playground, site, VS Code, Tree-sitter, Zed) is at
  1.0.0, [`CHANGELOG.md`](CHANGELOG.md) lists what 1.0 contains, the READMEs
  give the stability promise of 1.x, and
  [CONTRIBUTING](CONTRIBUTING.md#releasing) explains how to tag `v1.0.0` and
  what the release workflows then do.

## After 1.0

Ideas, not promises: each one would be weighed against the rules above, and
none may break the [stability promise](README.md#stability-promise) of 1.x.

- **A bytecode virtual machine**, for programs several times faster that
  print exactly the same thing.
- [x] **Types written by hand where they help**: optional, checked by
  `rythme check`, built on the inference that exists; code without them
  stays as it is. Done: `fn area(w: Int, h: Int) -> Int`, `total: Float =
  0.0`, `Circle(radius: Float)`, `do |n: Int|`, with type variables (`a`);
  E027 for a value that surely does not fit or a type that does not exist;
  every 1.0 program keeps its syntax tree (manual 4.10, `rythme help types`).
- **A package index**: finding and sharing packages by name, beside the
  dependencies from a repository or a folder of `rythme.toml`.
- [x] **A debugger in the editors** (breakpoints, values of the names), building
  on `rythme steps` and the language server. Done: `rythme debug file.ry`
  in a terminal (breakpoints on lines and functions, `next`, `step`,
  `finish`, `where` with the arguments and the depth of recursion, `print`
  of any expression with values only, `locals`, a pause on errors, tail
  calls shown as such; `rythme help debug`), and `rythme dap`, the Debug
  Adapter Protocol, used by the VS Code extension for the debugger type
  `rythme`. Without the debugger, programs run at the same speed.
- **More languages of the messages**, and the guide for teachers translated
  where it is not yet.
- **More of the library**: CSV, HTTP requests, `random` with a seed that a
  program can repeat, with the same care for errors as values.
  - [x] **CSV**: `csv_parse`, `csv_records`, `csv_format` (RFC 4180, any
    one-character separator, errors as `Err` values with the line and the
    column; `rythme help csv`).
  - [x] **A seed for `random`**: `random_seed(n)` and `rythme run --seed N`
    (same seed, same version: same draws of `random` and `shuffle`;
    `rythme help random`), and the *Seed* option of the playground (every
    Run, Test and Steps with the same draws, kept with the draft, in sharing
    links as `&seed=N`; see [site/README.md](site/README.md#seed)).
  - [x] **HTTP requests**: `http_get` and `http_post`, `Ok(text)` or
    `Err(message)`, through the program curl (no dependency; keys never on a
    command line); an `Err` in the playground.
- [x] **Artificial intelligence in class**: what a course needs to write
  machine learning, neural networks, search, games and calls to language
  models, pure and readable. Done: vectors and matrices over lists (`dot`,
  `matmul`, `transpose`...), statistics (`stats`, `percentile`, `softmax`...),
  activations, chance that follows `random_seed` (`random_normal`,
  `train_test_split`...), numerical slopes (`derivative`, `gradient`), charts
  as text (`plot`, `histogram`), HTTP (see above); `rythme help ml`, the
  manual, ten examples and `examples/ask_claude.ry`, six exercises (31 to
  36) in every language.
  - [ ] Exact derivatives (automatic differentiation), if it can stay
    invisible: dual numbers would have to go through every operation and
    `print`.
  - [ ] Faster numbers for large data (a compact list of Floats inside the
    interpreter, invisible to programs), if courses need more than
    thousands of examples.
- [x] **Sharing in the playground**: a link to a program with its steps and its
  tests, for classrooms. Done: the Share button of the playground, a link that
  holds the program and its test mode (see [site/README.md](site/README.md#sharing-links)),
  and the steps of a calculation shown one at a time, with links that open on a
  step (`&mode=steps&step=3`, see [site/README.md](site/README.md#steps)).
