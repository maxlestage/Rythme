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

## How the interpreter is organised

```
source text ─▶ lexer.rs ─▶ parser.rs ─▶ check.rs ─▶ interp.rs
               tokens       syntax tree   errors        values
                                          before running
```

- `driver.rs` loads files and the files they `use`, checks them, runs them,
  and runs tests.
- `diagnostic.rs` prints messages in the style of Rust.
- `builtins.rs` holds the standard library; `help.rs` the help topics and the
  explanation of every error code; `suggest.rs` the "did you mean" logic and
  the hints for people coming from other languages.

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
   they produce clear errors.
2. Add a line to `BUILTINS` with its signature, number of arguments,
   category, documentation in English and French, and an example.
3. Add its name to the VS Code grammar (`editors/vscode/syntaxes`); a test
   checks that the two lists match.

## Design rules

- One way to do each thing. Before adding syntax, check whether a function
  would do.
- Values never change. No feature may mutate a value in place.
- No silent conversion: `"1" + 1` is an error with a helpful message.
- Prefer a warning from the checker over a surprise at run time.
