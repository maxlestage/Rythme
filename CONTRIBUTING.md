# Contributing to Rythme

Thank you for helping! Rythme aims to be the simplest language to learn, with
help everywhere. Every change should keep that in mind: if a beginner can be
confused, a message should explain.

## Build and test

```sh
cargo build                 # target/debug/rythme
cargo test                  # examples, language, messages, commands
cargo fmt && cargo clippy --all-targets -- -D warnings
bench/run.sh                # speed: best time of each program of bench/
```

The tests run the real `rythme` program. `tests/examples.rs` checks that every
program in `examples/` prints exactly its `.out` file: when you add an
example, add its expected output too.

`tests/fuzz.rs` runs thousands of random programs: none may crash, hang, or
give a message that points outside of the code. After a change to the
lexer, the parser or the interpreter, run a longer session:
`RYTHME_FUZZ_CASES=1000000 cargo test --release --test fuzz -- --ignored`.

## How the interpreter is organised

```
source text ─▶ lexer.rs ─▶ parser.rs ─▶ check.rs ─▶ compile.rs ─▶ interp.rs
               tokens       syntax tree   errors        slots         values
                                          before running
```

- `driver.rs` loads files and the files they `use`, checks them, runs them,
  and runs tests. `modules.rs` turns `geometry.area` (for `use geometry`)
  into the single name `geometry.area` right after parsing; each file used
  with a name keeps its top-level names apart in the interpreter (see
  `compile::Names`), and so do the private `_names` of used files.
- `project.rs` is `rythme new`, `rythme install` (git through the `git`
  program) and `use package/file`, reached from `driver::find_use`;
  `manifest.rs` reads `rythme.toml` and `rythme.lock` (a small part of
  TOML, by hand). `tests/projects.rs` tests them with git repositories
  created in temporary folders.
- `types.rs` holds what the checker knows for sure about values: their types,
  and what each built-in function accepts and gives back. `check.rs` infers
  the types of expressions to report operations that would fail every time
  they run. A type only says what is certain: when the checker is not sure,
  it says nothing, because a correct program must never be rejected.
- `compile.rs` prepares the syntax tree for `interp.rs`: each local name
  becomes a numbered slot of the frame of its function, or a value captured
  by a closure, so that running never searches for a local name. After a
  change to the interpreter, run `bench/run.sh` before and after: the
  programs of `bench/` print a checksum of their work and their best time.
- `bigint.rs` holds the whole numbers beyond 64 bits (`Value::Big`), written
  by hand. The interpreter computes the usual numbers (`Value::Int`) on the
  spot and only goes there when a result does not fit, so keep it off the
  fast paths: `tests/fuzz.rs` checks it against `i128` and against a simple
  decimal multiplication.
- `diagnostic.rs` prints messages in the style of Rust.
- `format.rs` is `rythme fmt`, the formatter: run `cargo run -- fmt examples`
  after changing an example. Its tests (`tests/format.rs`) check on every
  piece of code of the repository that it never changes the meaning of a
  program nor loses a comment.
- `testing.rs` is `rythme test`: folders, `--filter`, `--watch`, `--seed`
  and the summary (`tests/testing.rs`); `run_paths` tests files and folders
  for any tool. Property tests (`check_all` and its generators) live in
  `builtins/property.rs`.
- `doc.rs` is `rythme doc`: the documentation of files and folders, from
  their `##` comments, as Markdown or one HTML page (`tests/doc.rs`).
- `builtins.rs` holds the standard library; `help.rs` the help topics and the
  explanation of every error code; `suggest.rs` the "did you mean" logic and
  the hints for people coming from other languages.
- `lsp/` is the language server (`rythme lsp`): it reuses the lexer, the
  parser and the checker to give the same help inside editors. Its tests
  (`tests/lsp.rs`) talk to it like an editor does. The VS Code extension
  (`editors/vscode`) is checked with `npm run check`.

## Messages: always in two languages

Every text shown to users exists in English and French:

```rust
t!("unknown name `{name}`", "nom inconnu `{name}`")
```

A good message says **where** (a span), **what** (the message), and **how to
fix it** (a `help`, often with a small example). Errors have a code
(`E001`...); a new code needs an explanation in `help::explain` and an entry
in `help::CODES` (a test checks it).

## Adding a built-in function

1. Write the function in `builtins.rs`:
   `fn name(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value>`.
   Use the helpers (`list`, `text`, `int`, `func`...) to check the arguments:
   they produce clear errors, with `wrong_argument` (and `wrong_return` for
   a function given to it), which the checker uses too.
2. Add a line to `BUILTINS` with its signature, number of arguments,
   category, documentation in English and French, and an example. Each
   example line `code   # value` is run by `tests/stdlib.rs`: it must give
   exactly that value.
3. Add its name to the VS Code grammar (`editors/vscode/syntaxes`), to
   `site/src/highlight.rs`, `site/public/playground.js` and
   `playground/web/index.html`, in the order of `BUILTINS`; tests check that
   the lists match.
4. If other languages call it differently (`padStart`, `rjust`...), add the
   names to `suggest::alias_hint`, and test the edge cases (empty values,
   negative positions, accents) in `tests/stdlib.rs`.
5. Optionally, describe it in `types::builtin_call` (what kinds of values it
   accepts, with the `Expected` of its errors and in the order it checks
   them, and what it gives back), so that `rythme check` finds mistakes like
   `len(5)` before running, with the same message. Without it, the checker
   simply knows nothing about the function. `tests/types.rs` calls every
   described function with values of every kind and checks that the
   description is right.

## Adding an exercise

The exercises of [`docs/exercises/`](docs/exercises/README.md) exist in
`en/` and `fr/`, with a reference solution of each in
`docs/exercises/.solutions/`. A new exercise needs both files, both
solutions (the code, then exactly the tests of the exercise, after the line
`# --- Tests ... ---`), a line in `EXERCISES` (`src/exercise.rs`, which builds
the files into `rythme exercise`) and a line in both index files.
`tests/exercises.rs` checks that every exercise is well formed, checks without
a warning, fails until it is solved, and that its solution passes its tests.

## Design rules

- One way to do each thing. Before adding syntax, check whether a function
  would do.
- Values never change. No feature may mutate a value in place.
- No silent conversion: `"1" + 1` is an error with a helpful message.
- Prefer a warning from the checker over a surprise at run time.
