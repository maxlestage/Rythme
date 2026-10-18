# Changelog

All notable changes to Rythme are written down here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and Rythme follows
[semantic versioning](https://semver.org): from 1.0 on, the
[stability promise](README.md#stability-promise) says what a 1.x version may
and may not change.

*En français : le journal des modifications de Rythme. La version 1.0 réunit
tout ce qui suit ; la [promesse de stabilité](README.fr.md#promesse-de-stabilité)
dit ce qu'une version 1.x peut changer ou non.*

## [Unreleased]

### Performance

- **`find_all`, `split_by` and `replace_all` are no longer quadratic** on
  regexes that read far ahead: their searches skip the ways that an earlier
  search showed to lead nowhere, and share their memory. `a*b|a` on 40 000
  `a`s took about 30 seconds and now takes a few milliseconds; the matches
  are the same (`bench/regex.ry`).

### Messages

- **`condition ? a : b`** (Ruby, C, JavaScript) is explained: the error
  points at the `?` and says to write `if condition then a else b`, in the
  terminal and in the editor. Only code that was already an error gets it.
- **`record[:name]`** says that a symbol is not a text, and how to read the
  field (`r.name`, `r["name"]`, `get(r, "name")`), before running and while
  running (where it said "cannot use `[...]` on Record"). A dict missing the
  key `:a` but holding `"a"` (or the reverse) says so.
- **`:explain` alone** in the REPL shows how to use it and the known codes,
  instead of "unknown code"; an unknown code is named, like with
  `rythme explain`.
- **`rythme steps`**: an unknown option (`--bogus`) is an error that names
  it, with the closest option, like the other commands (it was read as the
  expression `--bogus`); the mistakes in the expression given after the file
  are shown in `<expression>:1:...`, not as one more line of the file.

### Website

- **Sharing links in the playground**: *Share* makes a link that opens the
  program (and its test mode). The program travels in the fragment of the link
  (`#code=`, compressed with deflate, URL-safe), never sent to any server, so
  the links work on GitHub Pages and offline. The link is copied (or handed to
  the share sheet of a phone) and shown in a field to copy by hand; programs too
  long for a link are refused with a message. Opening a link shows the program
  without replacing the saved draft until it is changed, with a notice and
  *Back to my draft*. In the six languages of the site.

## [1.0.0] - 2026-10-18

The first stable version: every item of the [roadmap](ROADMAP.md) is done.
It gathers the pull requests #3 to #10 and the 1.0 release itself.

### Language

- **Modules with names**: `use geometry`, `use shapes/circle`,
  `use "geometry.ry" as geo`, then `geometry.area(2)`; names starting with
  `_` stay private to their file. Plain `use "x.ry"` keeps working.
- **Ruby style**: blocks given to calls (`xs.map do |x| x * 2 end`,
  `xs.select { |x| x > 2 }`), method chains without parentheses and over
  several lines (a field wins over a function), names ending with `?`
  (`even?`, `fn adult?(p)`), `unless ... then ... else`, symbols (`:ok`),
  `"#{x}"` in texts, and the Ruby names of the library (`puts`, `upcase`,
  `select`, `reduce`, `size`...) as real built-in names.
- **Calls without parentheses** at the start of a statement or of a result:
  `puts "Hello"`, `print x, y`, `if ok then puts "yes" else puts "no"`;
  `x [0]` and `x -1` stay an index and a subtraction when `x` has a value.
- **Patterns on ranges and texts** in `match`, function definitions and
  definitions: `1..9`, `-5..-1`, `0.0..0.5` (both ends included) and
  `"Mr " + name`.
- **Whole numbers without limit**: `Int` grows beyond 64 bits instead of
  overflowing (up to 100 000 digits), with the 64-bit fast path kept; `pow`,
  `gcd`, `text`, `parse_int`, JSON and comparisons with `Float` are exact.

### Library

- **Dates and times**: `now()`, `today()`, dates as records, ISO 8601
  parsing and printing, differences and additions in days and seconds, the
  day of the week, UTC and the local offset; no dependency.
- **Dictionaries and sets**: persistent dictionaries whose keys are any
  comparable value (`dict`, `get`, `put`, `has`, `keys`...), and sets
  (`set`, `union`, `intersection`, `difference`), printed and compared like
  the other values.
- **Text tools**: a small, well-explained subset of regular expressions,
  `format_number` (thousands separators, padding), `lines`, `words`,
  `trim_start`, `trim_end`, `equals_ignore_case`.
- **Files and the system**: `list_folder`, `create_folder`, `file_info`,
  `delete_file` (never a folder), `input_lines`, documented `exit` codes,
  `options()` (the command-line arguments as a record), `join_path` and
  `split_path`.
- **Property tests**: `check_all(generator) { |x| ... }` with generators of
  values and shrinking of the failing case, shown with its seed.
- 174 built-in functions in all (from 90 in 0.1), each documented with an
  example that the tests run.

### Checker

- `rythme check` infers the types of values when they are certain and
  reports the operations that would always fail (W006 to W009), with zero
  false positives.
- It infers what the parameters of each function must be and reports the
  calls that always fail (`area("big")` when `area` multiplies).
- W004 for empty ranges and for ranges already handled by the cases before,
  W010 for ranges that overlap.
- 26 error codes (E001 to E026) and 10 warning codes (W001 to W010), each
  explained by `rythme explain`.

### Tools

- `rythme fmt`: the official layout, which never changes the meaning
  (`--check` for CI, folders).
- `rythme lsp`: a language server with problems as you type, documentation
  on hover, completion (after `use ` and `geo.` too), signature help, go to
  definition, rename, formatting, a quick fix adding a missing `use`,
  inferred types as inlay hints, semantic highlighting, code lenses to run a
  test or a file, workspace symbols, folding and selection ranges.
- `rythme test`: folders, `--filter`, `--watch`, a summary with times, and
  property tests.
- `rythme doc`: the documentation of a file or a folder from its `##`
  comments, types and signatures, in Markdown or HTML.
- `rythme steps file.ry ["expression"]` (`:steps` in the REPL, the Steps
  button of the playground): how an expression is reduced, step by step.
- `rythme exercise`: 30 graded exercises for a first course (`list`,
  `start`, `check`), with hints and tests.
- Projects and packages: `rythme new`, the `rythme.toml` manifest,
  dependencies from git or a folder with a `rythme.lock` file
  (`rythme install`), `use "package/file.ry"`, and `rythme run` and
  `rythme test` at the root of a project.
- Installers: `install.sh` (`curl | sh`) and `install.ps1` (PowerShell),
  which check the checksums of the release; a Homebrew formula and winget
  manifests filled in by each release; crates.io metadata.

### Editors

- VS Code: the extension starts the language server, and is published on
  the Marketplace by a workflow when the `VSCE_PAT` secret exists; its
  settings and texts exist in every language of the messages.
- A Tree-sitter grammar (`editors/tree-sitter-rythme`) with its tests, for
  Neovim, Helix and Zed highlighting, indentation and outline.
- The Zed extension: the grammar and the language server.

### Website

- The website is written in Rust with Paquets, mobile first: home page,
  tour, examples, the guide for teachers and the playground, which runs the
  interpreter compiled to WebAssembly.
- Published on GitHub Pages as a static export, and deployable on Heroku in
  one tap (buildpack or container).
- An installable app (PWA) that works offline.
- Color themes, and no page scrolls sideways.
- In six languages (`/en/`, `/fr/`, `/es/`, `/de/`, `/pt/`, `/zh/`), with
  the pieces of code of each page written in that language and run by the
  tests, `Accept-Language`, `hreflang` and a compact language menu.
- The home page shows the version, 1.0.

### Languages of the messages

- Messages in any number of languages: `t!(en, fr)` stays in the code, and
  each other language is a catalog `src/i18n/<code>.rs` that a test checks
  complete.
- **Spanish**, **German**, **Portuguese** (Brazilian) and **simplified
  Chinese**, beside English and French: every message, help topic,
  `rythme learn`, the 30 exercises, the files of `rythme new`, the VS Code
  extension and a README in each language. `--lang es|de|pt|zh`,
  `RYTHME_LANG`, or the language of the system.
- The `^` under a mistake are placed by display width, so Chinese lines
  stay aligned.

### Speed

- Local names become numbered slots before running and most calls allocate
  nothing: the programs of `bench/` run 1.2 to 4.3 times faster than 0.1.
- Building a text with `+` and adding to a list (`push`, `xs + [x]`,
  `[..xs, x]`) take constant amortized time: 100 000 additions take 0.1 s
  instead of 23 s (texts) and 150 s (lists).
- Local functions that call each other make no closure per call (twice as
  fast).

### Documentation

- A reference manual in English and French ([`docs/reference.md`](docs/reference.md),
  [`docs/reference.fr.md`](docs/reference.fr.md)) whose every sample is run
  by the tests and shows its real output.
- This changelog, and the stability promise of 1.x in every README.

### Fixed

- The bugs found by fuzzing (thousands of random programs, none may crash,
  hang or point outside of the code) and by reviews, each with a regression
  test.
- Constructors of two types with the same name in two files are no longer
  mixed at run time: patterns, `==`, order and dict keys tell them apart.
- `check_all` stops at once with E014 when a property recurses too deep,
  instead of running for ever while simplifying.
- The checker no longer refuses dicts given to `d["k"]`, nor a REPL function
  defined again.
- `rythme steps --width` counts terminal columns (Chinese text), and the
  help for names defined further down a block and for `xs.select(m.f?)` is
  right.

## [0.1.0] - 2026-10-17

The first version (pull requests #1 and #2).

### Added

- The Rythme language: an interpreter in Rust, purely functional, with
  `Option` and `Result` instead of `null` and exceptions, algebraic types,
  pattern matching and tail calls that do not grow the stack.
- Rust-style messages with error codes, `rythme explain`, `rythme help`,
  `rythme learn` and help for those coming from other languages, in English
  and French.
- `rythme check`, the interactive mode, tests next to the code
  (`rythme test`), examples and guides for teachers.
- A VS Code extension (colors and snippets), continuous integration,
  release builds for Linux, macOS and Windows, and the web playground.

### Fixed

- Crashes, wrong results, false positives of the checker and problems of
  the interactive mode found by an independent review.

[Unreleased]: https://github.com/maxlestage/Rythme/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/maxlestage/Rythme/releases/tag/v1.0.0
[0.1.0]: https://github.com/maxlestage/Rythme/tree/af427701d94e22addeddef618bde7380c331a20f
