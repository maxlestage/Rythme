# Roadmap to Rythme 1.0

*En français : la feuille de route vers Rythme 1.0. Chaque point est fait,
testé et fusionné l'un après l'autre ; une case cochée est terminée.*

Rythme 0.1 has the language, the checker, the formatter, the language server,
the playground and the manual. This list is what remains for a **complete**
language: one that universities everywhere and teams at work can adopt. Items
are worked on in order, one at a time, each one fully (code, checks, messages
in every supported language, `rythme help`, the reference manual in English
and French, tests, CI green) before it is checked.

The rules of the language stay the rules of every item: simpler than Python,
purely functional, safe like Rust (no `null`, no exceptions), help everywhere.
No new keyword or syntax unless an item says so and nothing simpler works.

## Language

- [ ] **Modules with names.** `use "geometry.ry" as geo`, then `geo.area(2)`;
  names starting with `_` stay private to their file; the checker, the
  formatter, the language server (completion after `geo.`) and the manual
  know it. Plain `use "x.ry"` keeps working.
- [ ] **Patterns on ranges and texts.** `1..9 -> ...` in `match` (Int and
  Float bounds, exhaustiveness knows nothing new), and a text prefix pattern
  only if it stays simple; the checker reports overlapping or empty ranges.
- [ ] **Whole numbers without limit.** Int grows beyond 64 bits instead of
  E012 (the way Python does), with the small-number fast path kept; `pow`,
  `gcd`, `text`, `parse_int`, JSON and comparisons with Float are exact.
- [ ] **Types of parameters, inferred.** `rythme check` infers what a
  function's parameters must be from how it uses them, and reports a call
  that always fails (`area("big")` when `area` multiplies), with zero false
  positives; the language server shows the inferred signature on hover.

## Library

- [x] **Dates and times.** `now()`, `today()`, dates as records
  (`{year, month, day, hour, minute, second}`), ISO 8601 parsing and
  printing, differences and additions in days/seconds, day of the week;
  UTC plus the local offset when the system gives it; no dependency if
  possible.
- [ ] **Dictionaries and sets.** Keys of any comparable value (not only
  texts): `dict(...)`, `get`, `put`, `has`, `keys`... on them, and sets
  (`set(list)`, `union`, `intersection`, `difference`), printed and compared
  like the other values, persistent (no mutation), with the checker's types.
- [ ] **Text tools.** Searching and splitting with simple patterns (wildcards
  or a small, well-explained regular expression subset), `format` of numbers
  (thousands separators, padding), `lines`/`words` helpers if missing,
  `trim_start`/`trim_end`, case-insensitive comparison.
- [ ] **Files and the system.** Listing a folder, creating folders, reading
  lines of stdin until the end, `exit` codes documented, command-line
  options helpers (`args` parsed into a record), paths joined portably.

## Help everywhere, for everyone

- [ ] **Messages in Spanish.** The translation system takes any number of
  languages (catalogs instead of two-argument `t!`), with tests that every
  message exists in every language; then every message, help topic and
  `rythme learn` in Spanish (`--lang es`).
- [ ] **Messages in German, Portuguese and Chinese** (one language per item
  if needed), with the website and the playground offering each language.
- [ ] **Step-by-step evaluation for teaching.** `rythme explain-run file.ry`
  (or a playground button) shows how an expression is reduced step by step
  (`fib(3)` → `fib(2) + fib(1)` → ...), bounded, in every language.
- [ ] **Exercises.** `docs/exercises/` with graded exercises for a first
  course (each with tests that `rythme test` runs and hints), and
  `rythme exercise` (list, start, check) if it stays simple.

## Tools

- [x] **`rythme doc`.** Documentation of a file or a folder from its `##`
  comments, types and signatures, as Markdown and HTML, linked to the
  built-ins' docs.
- [ ] **Better `rythme test`.** A folder of files, a filter by name,
  `--watch`, a summary with times, and property tests
  (`check_all(fn(x) = ..., generator)`) with shrinking of the failing case.
- [ ] **Language server extras.** Inferred types as inlay hints, semantic
  highlighting, code lenses to run a test or a file, workspace symbols.
- [ ] **A Tree-sitter grammar** for Neovim, Helix and Zed highlighting, with
  its tests, and the Zed extension completed.
- [ ] **Projects and packages.** `rythme new name`, a `rythme.toml` manifest,
  dependencies from git or a path with a lock file, `use "package/file.ry"`,
  `rythme run`/`test` at the project root.

## Speed and release

- [ ] **Faster where it still hurts.** Building texts with `+` and adding to
  lists in constant amortized time, cheaper calls between local functions;
  measured with `bench/`, no behavior change.
- [x] **Installers.** A `curl | sh` / PowerShell install script using the
  release binaries, a Homebrew formula and a winget manifest in the
  repository, crates.io metadata, and a workflow publishing the VS Code
  extension when a `VSCE_PAT` secret exists (skipped otherwise).
- [ ] **Rythme 1.0.** Version 1.0.0 everywhere, a CHANGELOG, a stability
  promise in the README, the manual and the website up to date. Opened as a
  pull request; the `v1.0.0` tag is left to the owner.
