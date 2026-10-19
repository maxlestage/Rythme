# AGENTS.md: working on Rythme with an AI coding agent

Instructions for AI coding agents (Claude Code, Codex, Copilot, Cursor...)
that change this repository: the Rust implementation of the Rythme language,
its tools, editors and website. People: [CONTRIBUTING.md](CONTRIBUTING.md)
says the same at length. To *write Rythme programs* instead, read
[`docs/llms-full.txt`](docs/llms-full.txt) (`rythme help llms`).

## What Rythme is

A small, purely functional language with help everywhere: simpler than
Python, safe like Rust (no null, no exceptions, values never change), with
Rust-style diagnostics that have a stable code (`E001`...`E027`,
`W001`...`W011`) explained by `rythme explain CODE`. Version 1.x has a
[stability promise](README.md#stability-promise): add, never break.

## Build and test

```sh
cargo build                                   # target/debug/rythme
cargo test                                    # every test (runs the real binary)
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Before you finish, everything CI runs must pass:

```sh
cargo fmt --check && cargo clippy --all-targets -- -D warnings
cargo test && cargo test --release
(cd playground && cargo build --release)      # the interpreter in WebAssembly
./heroku-build.sh && (cd site && cargo test)  # the website and its playground
(cd editors/vscode && npm ci && npm run check)            # after cargo build
(cd editors/tree-sitter-rythme && npm ci && npm test)     # if the syntax changed
```

Useful targeted runs: `cargo test --test checker`, `--test lsp`,
`--test i18n`, `--test manual`, `--test machine`, `--test llms`. After a
change to the lexer, parser or interpreter, a longer fuzzing session:
`RYTHME_FUZZ_CASES=1000000 cargo test --release --test fuzz -- --ignored`.
After a change to the interpreter: `bench/run.sh` before and after.

## Where things are

| Path | What |
|---|---|
| `src/lexer.rs` → `parser.rs` → `check.rs` (+ `types.rs`, `check/`) → `compile.rs` → `interp.rs` | the pipeline; `driver.rs` loads files, checks, runs, tests |
| `src/diagnostic.rs` | diagnostics (`Diagnostic`, `Fix`) and their Rust-style rendering |
| `src/fix.rs`, `src/machine.rs` | fixes (`rythme fix`) and `--format json` |
| `src/help.rs` | help topics, keyword docs, every code's explanation (`CODES`, `explain`) |
| `src/builtins.rs`, `src/builtins/` | the standard library (`BUILTINS`: signature, docs, example) |
| `src/suggest.rs` | "did you mean", names and keywords from other languages |
| `src/lang.rs`, `src/i18n/*.rs` | languages: `t!(en, fr)` plus a catalog per other language |
| `src/lsp/` | the language server; `src/debug/` the debugger (terminal and DAP) |
| `src/llms.rs` | generates `docs/llms.txt` and `docs/llms-full.txt` |
| `src/format.rs`, `doc.rs`, `steps.rs`, `testing.rs`, `project.rs`, `exercise.rs` | `fmt`, `doc`, `steps`, `test`, `new`/`install`, `exercise` |
| `docs/reference.md`, `docs/reference.fr.md` | the manual (EN and FR); samples are run by `tests/manual.rs` |
| `tests/` | integration tests; `tests/common/mod.rs` runs the binary |
| `editors/` | VS Code, Tree-sitter, Zed, Helix |
| `playground/`, `site/` | the WebAssembly playground and the website |

## Conventions

- **Every user-facing text in six languages.** Write
  `t!("English {x}", "Français {x}")` at the call (or `lang::pick(en, fr)` for
  fixed texts), then add the translation of the English text to each catalog
  `src/i18n/{es,de,pt,zh}.rs`. `tests/i18n.rs` fails on a missing or stale
  entry; `RYTHME_I18N_MISSING=missing.rs cargo test --test i18n` writes the
  missing ones with the French text as a guide. Changing an English text
  means changing its key in every catalog.
- **Messages help.** A diagnostic says where (a span), what (the message)
  and how to fix it (a `help` with code; a `Fix` when the change is certain).
  A new code needs an explanation in `help::explain`, an entry in
  `help::CODES` and its translations.
- **The checker has zero false positives.** When it is not sure, it says
  nothing: a correct program is never rejected.
- **Stability (1.x).** No change of behavior or of existing output; only new
  commands, options, functions, fields of the JSON output (version 1) and
  messages. No new keyword or syntax unless nothing simpler works.
- **No new dependencies** unless clearly justified (the interpreter depends
  only on `rythme-macros` and `rustyline`).
- **Everything that should know about a change does:** interpreter, checker,
  formatter, language server, `rythme help`, the VS Code and Tree-sitter
  grammars and the lists of built-ins of the site and the playground (tests
  check they match `BUILTINS`), the manual in English and French, README
  tables, `docs/llms-full.txt` (regenerate with
  `cargo run -- help llms > docs/llms-full.txt`), CHANGELOG `[Unreleased]`.
- Style: `rustfmt.toml` (long lines); comments explain why, in plain English.

## Commits

Small, focused commits with a message that says what and why. Do not tag
versions: the owner does.
