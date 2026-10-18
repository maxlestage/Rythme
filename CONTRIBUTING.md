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
- `buffer.rs` holds the memory of texts and lists: a value made by adding at
  the end of another one (`acc + "x"`, `push(xs, x)`, `[..xs, x]`) is written
  in the room left after it, when nothing else was written there, so that
  building a text or a list piece by piece takes constant time per piece.
  Its `unsafe` code relies on one rule: what is written never changes.
- `bigint.rs` holds the whole numbers beyond 64 bits (`Value::Big`), written
  by hand. The interpreter computes the usual numbers (`Value::Int`) on the
  spot and only goes there when a result does not fit, so keep it off the
  fast paths: `tests/fuzz.rs` checks it against `i128` and against a simple
  decimal multiplication.
- `diagnostic.rs` prints messages in the style of Rust.
- `dict.rs` is the persistent ordered map (a weight-balanced tree) behind
  dictionaries and sets, and `key_cmp`, the order of their keys; their
  built-in functions are in `builtins/dicts.rs`, and `tests/dicts.rs` checks
  the tree against a simple model.
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

## Messages in every language

Every text shown to users is written in English and French where it is used:

```rust
t!("unknown name `{name}`", "nom inconnu `{name}`")
```

The other languages (Spanish, German, Portuguese and Chinese today) have a
**catalog**, `src/i18n/<code>.rs`:
each English text of the program with its translation, compiled into the
program. Fixed texts that are not formatted (help pages, explanations of
codes, documentation of functions) go through `lang::pick(en, fr)` instead of
`t!`, and find their translation the same way.

```rust
// src/i18n/es.rs
Language {
    code: "es",
    name: "español",
    texts: &[
        ("unknown name `{name}`", "nombre desconocido `{name}`"),
        // ...
    ],
}
```

- **How it works.** `t!` is a small macro (the crate `macros/`, without
  dependencies) that looks the English text up in every catalog while
  compiling and adds a `format!` for each translation: the compiler checks
  its `{names}` like those of the two texts of the call. A text missing from
  a catalog is shown in English.
- **The check.** `tests/i18n.rs` reads every `t!` and `pick` of the sources
  (and the fixed texts given by `help::fixed_texts`), prints how many texts
  each language translates, and fails when a complete language (`COMPLETE`
  in that test) misses one, when a catalog keeps a text the program no
  longer has (the English text changed: translate the new one), or when a
  translation uses a `{name}` its English text does not have.
- **Changing an English text** therefore means updating its entry in each
  catalog: the test names it.
- **Adding a language** (Italian, Japanese...): add a line to
  `LANGUAGES` in `src/lang.rs` (`include!("i18n/it.rs")`), create
  `src/i18n/it.rs` with `code`, `name` and an empty list, then run
  `RYTHME_I18N_MISSING=missing.rs cargo test --test i18n`: `missing.rs` lists
  every text to translate, in the order of the sources, with the French text
  as a guide, as entries `("English", "")` ready to copy into the catalog. An
  empty translation counts as missing, so the catalog can be filled little by
  little; add the code to `COMPLETE` once it is done. The code picks the
  language for `--lang it`, `RYTHME_LANG` and `LANG=it_IT.UTF-8`, and the
  playground's language number is its position in `LANGUAGES` (add the code
  to the lists of `site/public/playground.js` and `playground-worker.js`, a
  test checks them). Add it to the VS Code setting too (`package.json` and
  `package.nls.*.json`), and to the topic names of `help::TOPICS`. Keywords,
  built-in names and commands stay in English in every language.
- **Writing translations.** Keep every `{name}`, `{}`, backtick and example of
  code; translate the names and texts inside examples when the French text
  does (or keep the code as it is, like the Chinese catalog); keep the
  alignment of columns in help pages: Chinese, Japanese and Korean characters
  take two columns of the terminal. Run
  `rythme --lang es help`, `rythme --lang es explain E003` and a few
  programs with errors to read the result.

A good message says **where** (a span), **what** (the message), and **how to
fix it** (a `help`, often with a small example). Errors have a code
(`E001`...); a new code needs an explanation in `help::explain` and an entry
in `help::CODES` (a test checks it), and its translation in each catalog.

## Adding a built-in function

1. Write the function in `builtins.rs`:
   `fn name(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value>`.
   Use the helpers (`list`, `text`, `int`, `func`...) to check the arguments:
   they produce clear errors, with `wrong_argument` (and `wrong_return` for
   a function given to it), which the checker uses too.
2. Add a line to `BUILTINS` with its signature, number of arguments,
   category, documentation in English and French (and its translation in
   each catalog of `src/i18n/`), and an example. Each
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

The other languages translate each exercise in their catalog, like a help
page: an entry for its file name (ASCII, `07-rekursion`; Chinese keeps the
English names and identifiers) and one for the whole file, whose English text
is the key, so changing an English exercise shows its translations as stale
in `tests/i18n.rs`. Translate the statement, the hints, the names of the
tests and `fail("to do")` (an entry of its own), and the names of the code
the way the French file does; keep the numbers and the structure, so the
same tests check the same things. Each language has its reference solutions
in `docs/exercises/.solutions/<code>/`, and `tests/exercises.rs` runs every
exercise in every language.

## Design rules

- One way to do each thing. Before adding syntax, check whether a function
  would do.
- Values never change. No feature may mutate a value in place.
- No silent conversion: `"1" + 1` is an error with a helpful message.
- Prefer a warning from the checker over a surprise at run time.

## Releasing

Only the owner of the repository tags versions. A release is one commit on
`master` and one tag `vX.Y.Z`; the workflows do the rest.

Before tagging (here for 1.0.0; the pull request of a release does it):

1. The version is the same everywhere: `Cargo.toml` (and its dependency on
   `rythme-macros`), `macros/Cargo.toml`, `playground/Cargo.toml`,
   `site/Cargo.toml`, the lock files, `editors/vscode/package.json` and its
   `package-lock.json`, `editors/tree-sitter-rythme/package.json`,
   `package-lock.json` and `tree-sitter.json`, `editors/zed/Cargo.toml` and
   `extension.toml`. `rythme --version` prints the version of `Cargo.toml`,
   and the home page of the website shows the major and minor numbers
   (`hero_eyebrow` in `site/src/i18n/*.rs`).
2. `CHANGELOG.md` has a section for the version, dated the day of the tag,
   with what moved from `[Unreleased]`, and the link at the bottom.
3. In `editors/zed/extension.toml`, `rev` is a commit of `master` that has
   the current grammar.
4. CI is green on `master`.

Then tag the merge commit and push the tag:

```sh
git checkout master && git pull
git tag -a v1.0.0 -m "Rythme 1.0.0"
git push origin v1.0.0
```

What the tag starts:

- [`release.yml`](.github/workflows/release.yml) first checks that the tag
  is `v` followed by the version of `Cargo.toml`, and stops otherwise. It
  then builds `rythme` for Linux (x86_64, arm64), macOS (arm64, x86_64) and
  Windows (x86_64), and publishes the GitHub release `v1.0.0` with the
  archives, `SHA256SUMS`, `install.sh`, `install.ps1`, the Homebrew formula
  `rythme.rb` and the winget manifests filled in by
  [`packaging/render.sh`](packaging/render.sh). With the secret
  `HOMEBREW_TAP_TOKEN`, it also pushes the formula to the tap
  `maxlestage/homebrew-rythme`. From then on, the install scripts, which
  take the latest release by default, install 1.0.0.
- [`vscode.yml`](.github/workflows/vscode.yml) builds `rythme`, checks the
  extension, packages it (the artifact `rythme-vscode`) and, with the secret
  `VSCE_PAT`, publishes it on the Visual Studio Marketplace (a version
  already there is skipped).
- The website is not tied to tags: [`pages.yml`](.github/workflows/pages.yml)
  and [`heroku.yml`](.github/workflows/heroku.yml) publish every push to
  `master`.

By hand, after the release ([`packaging/README.md`](packaging/README.md)
gives the details): the winget manifests go to `microsoft/winget-pkgs`, and
crates.io gets `rythme-macros` first, then `rythme`
(`cargo publish -p rythme-macros`, then `cargo publish`), since `rythme`
depends on that exact version of the macros.

A patch version (1.0.1) fixes bugs; a minor version (1.1.0) adds things.
Both keep the [stability promise](README.md#stability-promise): nothing is
removed in 1.x; what is deprecated keeps working with a warning during at
least a whole minor version, and may only go in 2.0.
