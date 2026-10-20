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

### Embedding

- **Programs that embed Rythme can give records of functions** (`db.all(q)`,
  `mw.cors(...)`, the way [Facile](https://github.com/maxlestage/Facile) does):
  the checker knows the fields of a record found among the interpreter's
  names, so calling one checks its number of arguments (E004), and a
  misspelled one is reported (W005) with the closest field suggested. A
  function given under the name of one of the language (`db.all`) is not
  taken for it. `tests/embedding.rs` shows how.
- **A function given by the embedding program may be defined again**
  without W003: a web page's own `fn header(...)` hides the framework's
  `header` element on purpose. The language's own functions still warn.

### Rythme and AI assistants

AI assistants and coding agents can now learn Rythme, write it and fix their
own mistakes in a loop (manual 7.10, README "AI assistants"):

- **`--format json`** for `rythme check`, `run`, `test`, `fix`, `explain` and
  `help`: one JSON document (`"version": 1`, described by
  `docs/rythme-output.schema.json`) with each diagnostic's stable code,
  severity, message in the chosen language, file, line/column and byte
  ranges, labels, notes, help, call trace and fixes as text edits; the
  output of the program for `run`, each test for `test`, every built-in
  function (signature, doc, example, Ruby names, arity), keyword, topic and
  code for `help` and `explain`. The messages for people stay the default.
- **Fixes**: diagnostics carry the exact change when it is known (a misspelt
  name, a missing `use`, `a ? b : c`, `&&`, `===`, `<>`, `let`, `def`,
  `elif`, `return`, `lambda`, `=>`, `|x| ...`, `f"..."`, `"n = " + n`,
  `"%s" % x`, `xs[1:]`, `Math.sqrt`, a missing `then`...), marked safe or a
  guess. **`rythme fix FILE`** applies the safe ones until none is left
  (`--dry-run` prints a diff, `--all` also applies the guesses); the
  language server offers them all as quick fixes.
- **`rythme help llms`** prints `docs/llms-full.txt`, the whole language in
  one text for a model (rules, syntax by example, every built-in function,
  habits from other languages, every code), and `rythme help llms.txt` its
  index `docs/llms.txt`; both are generated from the sources and tested.
- **`rythme mcp`**, a Model Context Protocol server (stdio): run in a
  sandbox with a time and memory limit, check, test, fix, format, explain,
  help, steps, and the reference as a resource. `rythme run --sandbox`,
  `rythme test --sandbox` and `RYTHME_SANDBOX=1` keep a program away from
  files, folders and environment variables.
- **`AGENTS.md`** (and `CLAUDE.md`) for the agents of contributors, and in
  every project made by `rythme new`.
- **Messages for the habits of other languages**: named arguments, `n = n +
  1`, `x += 1`, slices, `|x|` and `->` lambdas, Kotlin blocks, format
  specifiers, `Math.`, `yield`, `case`/`when`, `{name = 1}`, an `if` without
  `else` on one line... now say exactly what to write, in every language.
- **The website for assistants**: `/llms.txt` (with the addresses of the
  site) and `/llms-full.txt`, linked from every page and from `robots.txt`,
  saved for offline use; every page with content also as clean Markdown
  (`/en/learn/index.md`: the code fenced as `rythme`, run by the tests
  through the real interpreter), advertised by a `<link rel="alternate">`;
  structured data (JSON-LD: Rythme, its interpreter with version, license
  and download, the code of *Learn* and of the examples); a page *For AI
  assistants* (`/{lang}/ai/`, in the six languages) with the message to
  paste, the MCP configurations for Claude Code, Claude Desktop, Cursor and
  VS Code with copy buttons, the loop of `--format json` and `rythme fix`,
  and `AGENTS.md`; and **Copy for AI** in the playground, which copies the
  program and its output or errors as Markdown, ready for a chat.

### Language

- **Types written in the code, optional and checked**:
  `fn area(w: Int, h: Int) -> Int = w * h`, `total: Float = 0.0`,
  `type Shape = Circle(radius: Float) | Dot`, `xs.map do |n: Int| ... end`.
  The types are those of the messages (`Int`, `Float`, `Text`, `Bool`,
  `Unit`, `Symbol`, `List(Int)` or `[Int]`, `Set(T)`, `Dict(K, V)`,
  `Option(T)`, `Result(T, E)`, `Record`, `Function`), records with at least
  some fields (`{name: Text}`), functions (`fn(Int) -> Bool`), the types of
  the program (`Shape`, `geometry.Shape`) and type variables (`a`). Running
  never looks at them. `rythme check` reports a value, an argument or a
  result that surely has another type, and a type that does not exist, with
  the new code **E027** (explained in every language); it also uses them: a
  parameter has its type inside its function, a call gives the written type
  of its result (`first_of(xs: List(a)) -> a`), across used files too. They
  are read only where nothing could be written before: every program written
  for 1.0 keeps its syntax tree (checked on the examples, the manual, the
  tests and 260 000 generated programs). `rythme fmt`, hover and signature
  help of the language server, `rythme doc`, the Tree-sitter, Helix, Zed
  and VS Code grammars, `rythme help types` and the manual (4.10) know them.
- **A symbol may end with `?`**, as in Ruby: `:even?` is one symbol. It was
  read as `:even` followed by the operator `?`, which the checker always
  refuses (E016) before running, so no program that ran changes meaning (a
  fuzz test puts the old reading in thousands of programs made from the
  examples and the tests: every one is refused). `rythme fmt` keeps the space
  in `:even ?`; the VS Code and Tree-sitter grammars know the new form.

### Library

- **CSV**: `csv_parse(text, separator)` (`Ok(rows)`, lists of texts),
  `csv_records(text, separator)` (`Ok(records)`, one record per line with the
  names of the header as fields, in the order of the columns) and
  `csv_format(rows, separator)` (lists of fields or records; quotes only where
  they are needed). RFC 4180: fields between quotes with separators, line
  breaks and doubled quotes inside, `\n` or `\r\n`, an optional final line
  break, empty lines skipped, a BOM skipped. The separator is `","` by default,
  `";"` for French spreadsheets, `"\t"` for tabs. A text that is not valid CSV
  is an `Err` with the line and the column of the mistake, in every language
  of the messages; a header with two columns of the same name or a line
  without as many fields as the header too. `rythme help csv`, the manual
  (6, *CSV*) and `examples/grades.ry`.
- **Draws that can be repeated**: `random_seed(n)` makes the draws of
  `random` and `shuffle` that follow the same on every run, and
  `rythme run --seed N file.ry` does it for the whole run. Same seed, same
  version of Rythme: same draws. Without a seed nothing changes.
  `rythme help random`, the manual (6, *Chance and seeds*, and 7.1).
- **Numbers for machine learning**, over lists: vectors are lists of
  numbers, matrices lists of rows. `dot`, `norm`, `vector_add`,
  `vector_sub`, `vector_mul`, `vector_scale` (vectors or matrices of the same
  shape), `zip_with`, `matmul` (by a matrix or a vector), `transpose`,
  `zeros`, `ones`, `identity_matrix`, `shape`; `sigmoid`, `tanh`, `relu`,
  `softmax`; `stats` (a record: count, mean, median, std_dev, min, max),
  `variance`, `std_dev`, `percentile`, `argmax`, `argmin`, `normalize`;
  `random_normal`, `random_choice_weighted` and `train_test_split`, which
  follow `random_seed`. Computed in Rust (a product of 100×100 matrices
  takes about 0.4 ms), given back as Floats. A mistake of shape names both
  shapes; empty or ragged lists, `nan` and deep nesting never crash.
  `mean`, `median` and `average`, which exercises and examples define, stay
  free: `stats(xs).mean`, `percentile(xs, 50)`.
- **Slopes**: `derivative(f, x)` and `gradient(f, xs)`, by central
  differences, keeping only the digits the calculation makes sure of
  (`derivative(fn(x) = x * x, 3)` is `6.0`).
- **Charts as text**: `plot(ys)`, `plot(xs, ys)` and `histogram(xs, bars)`
  give a text to print, lined up by display width, the same everywhere.
- **HTTP requests**: `http_get(url, headers)` and `http_post(url, body,
  headers)` give `Ok(text)` or `Err(message)` (no network, a wrong address,
  an error status, a 30-second timeout, TLS). They run the program curl, as
  `rythme install` runs git: no new dependency; the address, the headers and
  the body go through its standard input, never its command line. In the
  playground, an `Err` says they are not available.
- `rythme help ml` (also `ai`, `ia`, `ki`...), the manual (6, *Machine
  learning*, *Charts as text*, *The web*), `bench/ml.ry`, ten AI examples
  (linear regression, perceptron, a network learning XOR, k-means, k nearest
  neighbours, naive Bayes, a Markov chain, minimax, breadth-first search, a
  genetic algorithm) and `examples/ask_claude.ry` (a language model asked
  through its HTTP API, with a key read by `env`), and six AI exercises
  (31 to 36) in every language of `rythme exercise`.
- 211 built-in functions. Names from other languages point to the new ones
  (`parse_csv`, `DictReader`, `to_csv`, `srand`, `median`, `stdev`, `eye`,
  `randn`, `logistic`...). As the stability promise says, a function of a
  program with one of these names stays its own (W003).

### Tools

- **A debugger: `rythme debug file.ry`.** Breakpoints on lines (`break 12`,
  `break geometry.ry:3`; a line without code moves to the next one) and on
  functions (`break fact`), `run`, `next`, `step`, `finish` (which shows the
  result), `continue`, `where` (the calls in progress with their arguments,
  `map` and the other built-in functions that call yours included), `frame`,
  `print` (any expression, computed in the paused call with values only:
  output, files, input, `exit` and chance are refused), `locals`, `list`,
  `help`, `quit`. Each pause shows the depth and how many calls of the same
  function are in progress; a call in tail position says that it replaced
  the frame of its caller. An error pauses where it happens, before its
  message. It works across files (`use`), blocks and local functions written
  one after the other. Messages in the six languages; `rythme help debug`,
  the manual (7.9, with a session checked by the tests).
- **`rythme dap`**: the same debugger for editors, with the Debug Adapter
  Protocol (launch, breakpoints on lines and functions, call stack, scopes
  and values that open, evaluate, continue, next, step in, step out, pause).
  The **VS Code extension** contributes the debugger type `rythme`: `F5` on a
  `.ry` file, a *Debug this file* button, a default launch configuration.
- Without the debugger, programs run at the same speed: the places where it
  may pause are only added to the code of a program it runs, and calls are
  followed through a check that the interpreter already makes (the
  instructions counted on `bench/` differ by 0.0 to 1.2 %).

### Fixed

- **`capitalize` uses the titlecase of Unicode** where it differs from the
  uppercase: `capitalize("ǆungla")` gives `ǅungla` (not `Ǆungla`), and so do
  `ǈ ǋ ǲ`; `ß` gives `Ss`, the ligatures `ﬁ ﬂ ﬀ...` give `Fi Fl Ff...`, the
  Greek letters with iota subscript keep it (`ᾳ` gives `ᾼ`), Armenian `և`
  gives `Եւ`, and Georgian letters stay as they are. A program that
  capitalizes one of these characters prints another text: a bug fix.
  `upper` does not change.

### Performance

- **`find_all`, `split_by` and `replace_all` are no longer quadratic** on
  regexes that read far ahead: their searches skip the ways that an earlier
  search showed to lead nowhere, and share their memory. `a*b|a` on 40 000
  `a`s took about 30 seconds and now takes a few milliseconds; the matches
  are the same (`bench/regex.ry`).

### Messages

- **A case of `match` on several indented lines** (`Ok(x) ->` then the
  lines, as in Ruby or Python) is explained: the error says that a case
  gives one expression and shows `Ok(x) -> do ... end`, instead of a hint
  about `==` or "put each case on its own line".
- **W011, index or subtraction instead of a call**: `show [1, 2]` and
  `show -1` at the start of a statement are an index and a subtraction as
  soon as the file gives `show` a value anywhere, even to a parameter of
  another function. The reading does not change, but where `show` is a
  function, `rythme check` (and the editor) now warns, shows where `show`
  gets its value, and asks for parentheses: `show([1, 2])`. A syntax error
  in such a line says it too. `rythme explain W011`, manual §3.15.
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
- **`rythme.toml` with a section written twice** (two `[dependencies]`, or
  `[dependencies]` and `[[dependencies]]`, or a key and a section of the same
  name) is an error E025 at the second one, with the first, as TOML wants
  (keys written twice already were); it was read without a word.

### Website

- **Sharing links in the playground**: *Share* makes a link that opens the
  program (and its test mode). The program travels in the fragment of the link
  (`#code=`, compressed with deflate, URL-safe), never sent to any server, so
  the links work on GitHub Pages and offline. The link is copied (or handed to
  the share sheet of a phone) and shown in a field to copy by hand; programs too
  long for a link are refused with a message. Opening a link shows the program
  without replacing the saved draft until it is changed, with a notice and
  *Back to my draft*. In the six languages of the site.
- **A seed in the playground**: *Seed*, next to Run, takes a whole number:
  every run then makes the same draws of `random` and `shuffle` (like
  `rythme run --seed N`), and Test the same values of `check_all` and
  `sample` too (like `rythme test --seed N`). Empty, the draws change as
  before. A dice picks a new seed; the seed is kept with the draft, shown in
  the head of the output, and *Share* puts it in the link (`&seed=42`), so a
  teacher's link gives every student the same draws. In the six languages of
  the site.
- **Steps in the playground, and links that open on them**: *Steps* shows the
  calculation of the last expression (or of the selected code) one step at a
  time, with *Previous* and *Next*, a slider and the keyboard, the step before,
  what the program printed and the error if any, announced to screen readers.
  *Share* from the steps makes a link that opens on the same step
  (`&mode=steps&step=3`, with `&expr=` for a selected piece of code), and the
  tour of *Learn* opens on the steps of `fib(4)`. Offline too, in the six
  languages of the site.
- **Truly mobile first, as real phones show it**: nothing scrolls sideways any
  more with the text enlarged (Android's font size, Safari's and Chrome's text
  zoom), the page zoomed (down to a window of 180px), in landscape or with the
  keyboard open. Every word may break where the layout needs it (titles and
  buttons too), every one-column grid is `minmax(0, 1fr)` (a select, a field
  or a long seed widened the page), the language menu hangs under the bar and
  scrolls when the screen is short, the tools of the editor and the banners
  wrap, runs of spaces in an output wrap, deep code lines keep room to wrap.
  The editor's text is 16px on every touch screen, landscape included: iOS
  zoomed into it at 14px and the page then scrolled sideways. Every control is
  44px at least for a finger; the bar stops sticking on short screens,
  titles shrink with the height in landscape, the editor fits above the
  keyboard, and opening the playground no longer scrolls the page by itself.
  The media queries only add room (`min-width`, `min-height`).
  `tests/pwa/mobile.js` (run by CI) checks all of it on emulated phones, in
  every language and page, with every menu and panel open.
- **The installed app updates itself**: a new version found when the app opens
  or comes back, before anything is touched, comes in by itself; so does a new
  version waiting when the app goes to the background with no field being
  edited, and at the next launch. The banner stays for the other cases, and
  nothing reloads while something is being typed. Pages are revalidated with
  the server by the service worker (GitHub Pages kept them 10 minutes).
- **The website is Rust in the browser too**: its menus, themes, installable
  app, service worker and playground were hand-written JavaScript (about 1,700
  lines); they are now Rust compiled to WebAssembly (`site/client`), and the
  playground runs the interpreter from Rust, in a web worker, rather than
  through the C functions of `rythme.wasm`. Same pages, same behavior, same
  strict Content-Security-Policy (`'wasm-unsafe-eval'` for WebAssembly); the
  only JavaScript left is what wasm-bindgen generates, the lines that start
  the bundles, and the one line that applies the theme before the first paint.
  Building the site needs `wasm-bindgen-cli` 0.2.129 (`site/build.sh`
  installs it).

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
