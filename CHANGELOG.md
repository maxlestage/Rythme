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

### What JavaScript has too

[`docs/javascript.md`](docs/javascript.md) goes through the standard library
of JavaScript (`Math`, `Number`, `String`, `Array`, `Object`, `Map`, `Set`,
`JSON`, `Date`, `RegExp`, `Promise`, and the web APIs `console`,
`setTimeout`, `URL`, `URLSearchParams`, `crypto`): what Rythme has under
another name, and what it leaves out on purpose. The 15 functions it was
missing are added (300 built-in functions now):

- numbers: `cbrt`, `hypot(x, y)`, `sinh`, `cosh`, `asinh`, `acosh`, `atanh`
  (outside their domain, E023 like `asin`);
- lists: `find_last` and `find_last_index`;
- `sleep(seconds)`, and `uuid()` (version 4, from the numbers of `random`:
  `random_seed` makes it repeatable);
- the web: `parse_url` (a Result of the parts of an address), `parse_query`
  (a dict of decoded texts) and `format_query` (a record or a dict as a
  query), `sha256(text)`.

`rythme help javascript` (or `js`, `typescript`) sums it up, and a
JavaScript name gives the Rythme one in the messages: `xs.findLast(f)` says
"in Rythme, this is called `find_last`". Like every built-in function added
after 1.0, a program that defines one of these names gets the warning W003;
`sign` is not added for that reason, since the manual and the help define
their own `sign` as an example.

### Tasks: `async fn` and `await`

Functions that run at the same time, on the other cores: `async fn fetch(url)
= ...` starts when it is called and gives a `Task` at once; `await task`
gives its result, and `await_all(tasks)` the results of a list of tasks.

- No program changes meaning: `async` is a keyword only right before `fn` and
  a function name, and `await` is a built-in function that can also be
  written without parentheses before a name, a constructor or a list
  (`await task`, `await fetch(url) + 1`); elsewhere both stay names.
- Values never change, so tasks need no locks: each one runs on a worker
  thread with an interpreter of its own and the program loaded again (its
  definitions, without its other top-level statements); arguments and
  results are copied. Where that is not possible (the REPL, the playground,
  an argument that is a function), the task runs when it is called, with the
  same result.
- An error in a task stops the program at its `await`, with the task's
  message. `rythme check` knows that a call of an `async fn` gives a `Task`
  (`Task + Int` is E005, with a help that says to write `await`), and what
  `await` gives.
- The formatter, `rythme help async` and `rythme help await`, the manual
  (4.11, EN and FR), the VS Code and Tree-sitter grammars, the site and the
  playground know them.

### A lighter playground

The website's playground downloads its interpreter with English and French
only, and the catalog of the page's language beside it: adding a language no
longer grows the download.

- A feature of the crate, `runtime-catalogs`: `t!` and `lang::pick` find
  the other languages in catalogs registered while running
  (`lang::register`), files written by `lang::catalog`. Without the feature
  (the command, the standalone playground), nothing changes.
  `tests/catalogs.rs` checks that both give the same messages and help, in
  every language.
- The playground's WebAssembly goes from 4.5 MB to 2.1 MB (1.05 MB to
  0.58 MB in Brotli); each catalog is about 90 KB in Brotli, fetched with it,
  cached and kept offline. On a slow phone, the first run comes 2 s sooner.
  If a catalog cannot be fetched, the messages are in English and a note
  says so.

### Italian and Japanese

Rythme now speaks eight languages: Italian (`--lang it`, `it_IT`) and
Japanese (`--lang ja`, `ja_JP`) join English, French, Spanish, German,
Portuguese and Chinese.

- Every message, help page, explanation of a code, documentation of a
  function, page of `rythme learn`, file of `rythme new` and exercise of
  `rythme exercise` (with its reference solution) in both languages; the
  topics of `rythme help` under their Italian and Japanese names
  (`rythme help liste`, `rythme help リスト`). Japanese characters take two
  columns: the `^` stay under the code.
- The website in both languages (`/it/`, `/ja/`), with its playground, its
  examples and the guide for teachers (`docs/teaching.it.md`,
  `docs/teaching.ja.md`); `README.it.md` and `README.ja.md`; the VS Code
  extension (`package.nls.it.json`, `package.nls.ja.json`, and `it`, `ja` for
  `rythme.language`); `"it"` and `"ja"` for `language` in the JSON output.
- The playground's WebAssembly grows by about 0.85 MB (0.15 MB in Brotli)
  with the two catalogs.

### The package index

Packages are found and shared by name, beside the dependencies from git or
a folder (manual 7.11, `rythme help packages`). The index needs no server:
it is a public git repository with one file per package
(`packages/NAME.toml`: description, repository, license, keywords, authors,
and for each version its tag, its commit and a checksum of its files), by
default `https://github.com/maxlestage/rythme-index` (to be created by the
owner from `packaging/index-template/`, which has its README, an example
entry and the workflow that checks its pull requests).

- **`rythme search WORDS`** and **`rythme info NAME`** read a copy of the
  index kept in `RYTHME_HOME/index`, without a network (`--update` gets the
  newest one; without a network the copy is used, with a warning), with
  `--format json` (`"command": "search"` and `"info"` in
  `docs/rythme-output.schema.json`).
- **`rythme add NAME[@RANGE]`** writes `NAME = { version = "^1.2.0" }` at the
  end of `[dependencies]` (the rest of `rythme.toml` untouched), then
  installs; `rythme.lock` keeps the version, the exact commit and the
  checksum, so `rythme install` needs neither the index nor the network
  afterwards.
- **`rythme update [NAME...]`** takes the newest version within the range of
  each dependency (`^1.2.0`, `~1.2`, `=1.2.0`, `>=1.2.0, <1.5.0`, as in
  Cargo and npm; withdrawn versions are never chosen) and the commit of each
  tag of git now, and lists what changed.
- **`rythme publish`** checks a package (`description`, `license` and
  `repository` in `rythme.toml`, a clean git repository, the tag of the
  version on the current commit and in the repository, `rythme check` and
  `rythme test`, a name and a version free in the index), then prints its
  entry and the steps of the pull request: it never sends anything.
  `rythme publish --verify FILE...` checks entries for the index's CI.
- **Safety**: names of packages are Rythme names (no path in them),
  addresses use https, ssh or file only (git is limited to these
  protocols), git runs without hooks, the files of each version must have
  the checksum of the index, and nothing of a package runs when it is
  installed.
- `rythme.toml` accepts `description`, `license`, `repository`, `keywords`,
  `authors` and `index` (another index: an address, or a folder read as it
  is; or the variable `RYTHME_INDEX`), lists of texts on one line, and
  dependencies `{ version = "..." }`. Files of 1.0 read the same.
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
- **`AGENTS.md`** (and `AGENTS.md`) for the agents of contributors, and in
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
  paste, the MCP configurations for AI assistants, desktop assistants, Cursor and
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
- **Types with parameters**: `type Tree(a) = Leaf | Node(left: Tree(a),
  value: a, right: Tree(a))`, `type Pair(a, b) = Pair(first: a, second: b)`,
  then `fn total(t: Tree(Int)) -> Int` or `p: Pair(Text, Int) = ...`. The
  parameters are type variables (one lowercase letter: a longer word stays
  a type named as in another language, which `rythme check` renames), used
  in the types of the fields; running never looks at them. `rythme check`
  reports, with E027, a value whose contents surely have another type
  (`Node(Leaf, "two", Leaf)` for a `Tree(Int)`, shown as `Tree(Text)`), a
  constructor whose type variable gets two types, a wrong number of types
  (`Tree(Int, Text)`), and parameters that are not one letter, written twice
  or missing (`type Box(a) = Box(value: b)`); it uses them as it uses those
  of `List`: the fields of a `Tree(Int)` are Ints in patterns, a call gives
  the types found in its arguments (`fn first_of(p: Pair(a, b)) -> a`),
  across used files too. `Tree` alone holds anything. Read only where `=`
  was the only thing allowed after the name of a type: every program of the
  repository, the manual and 457 000 generated programs (292 000 that parse)
  keep exactly the same syntax tree. `rythme fmt`, `rythme doc`, the
  Tree-sitter (regenerated) and VS Code grammars, `rythme help types`,
  `rythme help syntax`, `rythme learn`, E027's explanation and the manual
  (4.10, Appendix A) know them.
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
  genetic algorithm) and `examples/ask_model.ry` (a language model asked
  through its HTTP API, with a key read by `env`), and six AI exercises
  (31 to 36) in every language of `rythme exercise`.
- **The library, completed** after an audit against Ruby's core, Python's
  builtins, `itertools`, `statistics` and `string`, and Elixir's `Enum`,
  `String` and `Map` ([`docs/stdlib-audit.md`](docs/stdlib-audit.md)): 72
  new functions, all pure.
  - Text: `center`, `format(value, spec)` (the format specs of Python:
    `".2f"`, `">8"`, `"08,.2f"`, `"#x"`, `".1%"`...), `replace_first`,
    `occurrences`, `last_index_of`, `remove_prefix`, `remove_suffix`,
    `capitalize_words`, `snake_case`, `camel_case`, `kebab_case`,
    `swap_case`, `is_upper`, `is_lower`, `bytes`, `split_once`, `word_wrap`,
    `edit_distance`, `inspect`, `print_error`.
  - Numbers: `constrain` (Ruby's `clamp`), `divmod`, `binomial` (exact),
    `digits`, `to_base` and `from_base` (bases 2 to 36, any size),
    `is_positive`, `is_negative`, `is_zero`, `is_nan`, `is_finite`.
  - Lists: `find_index`, `update_at`, `rotate_by`, `intersperse`,
    `interleave`, `tally`, `count_by`, `most_common`, `unique_by`,
    `sort_desc`, `sort_by_desc`, `chunk_by`, `split_at`, `combinations`,
    `permutations`, `cartesian_product` (bounded: E023 beyond 10 million
    elements), `unzip`, `iterate`, `unfold`, `filter_map`; `zip` takes two
    lists or more.
  - Records and dicts: `dig`, `merge_with`, `invert`, `select_keys`,
    `reject_keys`, `map_keys`, `update(x, key, default, f)`; sets:
    `symmetric_difference`, `is_disjoint`.
  - Option and Result: `unwrap_or_else`, `expect`, `all_ok`.
  - Dates: `add_months`, `day_of_year`, and patterns for `format_date` and
    `parse_date` (`%Y %m %d %H %M %S %y %B %b %A %a %I %p %j %z %%`).
  - The web and statistics: `base64_encode`, `base64_decode`, `url_encode`,
    `url_decode`, `linspace`, `correlation`, `covariance`.
  - 26 more Ruby names: `clamp`, `swapcase`, `ljust`, `rjust`, `ord`, `chr`,
    `delete_prefix`, `delete_suffix`, `rindex`, `detect`, `collect`,
    `rotate`, `combination`, `permutation`, `transform_values`,
    `transform_keys`, `each_pair`, `to_h`, `warn`, `positive?`,
    `negative?`, `zero?`, `nan?`, `finite?`, `subset?`, `disjoint?`.
  - Each one is known to the checker (an argument that never fits is
    reported before running), documented with an example run by the tests,
    in the six languages, in `rythme help` (a block at the end of the topics
    `lists`, `text`, `dicts`, `errors` and `dates`), the manual (6, *More of
    the library*), the lists of the site, the playground and VS Code, and
    `docs/llms-full.txt`. Names that the examples and the exercises define
    (`is_prime`, `mean`, `median`, `clamp`, `merge`, `title_case`,
    `frequencies`, `factorial`, `sign`...) were left free.
- 283 built-in functions. Names from other languages point to the new ones
  (`parse_csv`, `DictReader`, `to_csv`, `srand`, `median`, `stdev`, `eye`,
  `randn`, `logistic`, `removeprefix`, `Counter`, `levenshtein`, `btoa`,
  `encodeURIComponent`, `strptime`...). As the stability promise says, a
  function of a program with one of these names stays its own (W003).

### Tools

- **`rythme CMD --help`** (or `-h`) for every command: its usage, its
  options and its exit codes, in the six languages (before the file for
  `run`, `steps`, `debug` and `-e`, whose following words belong to the
  program).
- **`rythme run -`** and `rythme -` run the program read from the standard
  input (also with `--format json`); **`rythme check FOLDER`** checks every `.ry` file of a folder.
- **`rythme completions bash|zsh|fish|powershell`** writes a completion
  script (commands, options, topics of `help`, codes of `explain`,
  languages, `.ry` files), and **`rythme man`** the manual page, in roff.
- **`rythme version --verbose`** adds the system, the languages, the numbers
  of functions and codes, and the version of the JSON output;
  **`rythme upgrade`** says which command installs the latest version,
  guessed from where Rythme is installed (nothing is downloaded).
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
- **The debugger names the parameters written as patterns** by their
  pattern in the definition that runs: `total([x, ..rest] = [1, 2], acc =
  0)` instead of `total(arg1 = [1, 2], acc = 0)`, in `where`, the pauses,
  `locals` and the call stack of `rythme dap`.
- **The language server completes the names of types** where a type is
  written: after the `:` of a parameter, of a name given a value, of a field
  of a variant or of a block's parameter, after `->`, inside `List(` or
  `Dict(Text, `, after `geometry.` for the types of a used file, with the
  type variables of the type being defined; it shows what a type of Rythme
  is, or the definition of a type of the program, on hover, goes to that
  definition, renames it and colors it (semantic tokens). The hover of a
  constructor shows the types of its fields and its type with its
  parameters (`constructor of the type `Tree(a)``), and so does
  `help(Node)` in a program.

### Documentation

- **The guide for teachers, up to date and in six languages**: grading a
  class with `rythme test --sandbox --format json` (and a Rythme program
  that turns the results into grades), the playground for students without
  an installation, `rythme steps` and the debugger to show how code runs,
  the messages in the students' language, the AI unit (exercises 31 to 36),
  types written in the code, and a policy for AI assistants in class.
  [`docs/teaching.md`](docs/teaching.md) and
  [`docs/enseigner.md`](docs/enseigner.md) are translated into Spanish,
  German, Portuguese and Chinese (`docs/teaching.es.md`, `.de.md`, `.pt.md`,
  `.zh.md`); the website shows each language its own guide, and each README
  links its own. The tests run the samples of every guide in its language.

### Fixed

- `rythme fix` and the suggestions for `x => ...` and `{name = ...}` no
  longer stop with an internal error when a character of several bytes
  comes right before the name (found by fuzzing).
- **`capitalize` uses the titlecase of Unicode** where it differs from the
  uppercase: `capitalize("ǆungla")` gives `ǅungla` (not `Ǆungla`), and so do
  `ǈ ǋ ǲ`; `ß` gives `Ss`, the ligatures `ﬁ ﬂ ﬀ...` give `Fi Fl Ff...`, the
  Greek letters with iota subscript keep it (`ᾳ` gives `ᾼ`), Armenian `և`
  gives `Եւ`, and Georgian letters stay as they are. A program that
  capitalizes one of these characters prints another text: a bug fix.
  `upper` does not change.
- **Found by auditing the tools around the interpreter** (with new fuzzers
  of the checker, the formatter, the fixes, the language server and every
  built-in function on edge values):
  - `from_base("0é", 16)` and `zeros(0, 10 ** 18)` (also `ones`) stopped
    Rythme with an internal error; they give `Err(...)` and E023.
  - The checker no longer reports a function that is never called on an
    empty value written in the code: `map_keys({}, len)`,
    `take_while("", len)`, `merge_with` of two records without a name in
    common (as it already did for `[]`). For `sum(list, f)`, it says that
    `f` must return a number (it said "true or false").
  - `rythme fix`: `else ()` goes where `else` was expected (inside
    `print(if x then y)`, not after the `)`), and `else ()` and `then` go
    before a comment that ends the line, not inside it; `fn f"..."` is no
    longer taken for an f-string.
  - A command-line word that is not UTF-8 (a file name made on another
    system) is a usage error (exit code 2) instead of a crash; `rythme fmt`
    says why a file cannot be read like the other commands (not UTF-8, saved
    in UTF-16...) and refuses unknown options written with one `-`.
  - `rythme.lock` must give a commit in hexadecimal and an address that git
    cannot take as an option.

### Performance

- **A bytecode virtual machine runs programs** instead of the interpreter
  that walked the syntax tree: functions are compiled once to instructions
  on registers (`src/vm/`), and the programs of `bench/` run up to about
  twice as fast (`count`, `fib`, `match`, `records`; 1.1 to 1.8 times for
  the others, as fast for `regex` and `ml`, which spend their time in
  built-in functions), printing exactly the same (output, errors with their
  places and traces, what the debugger and `rythme steps` show, the draws
  of `random_seed`). Calls are cheap: built-in functions are called
  directly, functions given to `map`, `fold`, `sort_by`... are called in
  place, and the instructions that read or update a field keep its
  position for records with the same fields.
- Every program of the repository (examples, exercises, benchmarks, the
  samples of the manual, the code in the tests) and millions of generated
  ones print the same as with 1.0, with `rythme run` and `rythme test`, in
  the debugger, with `rythme steps` and in the REPL; every kind of
  instruction runs under Miri (`cargo +nightly miri test --lib vm`).
- The limit of nested calls (E014) is now the same for every function:
  917 504 with `rythme` (it was about 900 000, fewer for functions that do
  more in each call), still 600 in the playground, which needs less of the
  browser's stack than 1.0, also when functions given to `map` or `fold`
  call them again.
- **The virtual machine is faster still**, printing the same: a value is
  16 bytes instead of 24 (a text or a list takes the 15 bytes after its
  tag); a value of a type keeps up to three fields in place and a record up
  to four, so making `Node(l, v, r)` or `{..r, a: 1}` takes one block of
  memory instead of two; the lists of Floats made by `matmul`,
  `transpose`, `vector_scale`, `softmax`... keep their numbers as they are
  (8 bytes each), read as they are by these functions; the value stack
  keeps its room, so that a function given to `map` or `fold` no longer
  writes its registers again at each call; a constructor called last, and
  a function of a group calling another one last, skip the path of calls;
  `None` and `Leaf` are kept by the instructions that read them. `bench/`
  (instructions, against 1.0): `local` −32 %, `bst` −20 %, `ml` −15 %,
  `records` −14 %, `match` and `sort` −13 %, `pipeline` −10 %, `closures`
  −9 %, `count` −5 %, `fib` −4 %, `append` −3 %; in time, `bst` 0.91 s to
  0.57 s, `local` 0.28 s to 0.17 s. The limits of nested calls are the same
  (917 504; 600 in the playground, which needs no more of the browser's
  stack than 1.0).
- **`find_all`, `split_by` and `replace_all` are no longer quadratic** on
  regexes that read far ahead: their searches skip the ways that an earlier
  search showed to lead nowhere, and share their memory. `a*b|a` on 40 000
  `a`s took about 30 seconds and now takes a few milliseconds; the matches
  are the same (`bench/regex.ry`).
- **Regexes about twice as fast**: a search tries the ways of a match one
  after the other, in order of preference, remembering each step and place
  it reached (so it never takes forever, and finds exactly the same matches
  and groups as before); it goes straight to the characters that can start
  a match, and reads classes like `[a-z]` from a table for ASCII characters.
  `bench/regex.ry`: 2.8 to 1.1 billion instructions (0.23 s to 0.11 s).
- **Fewer copies in the built-in functions**: numbers are written in texts
  without the machinery of Rust's `format!` (`bench/textbuild.ry`: 13 %
  fewer instructions), `take`, `drop`, `slice`, `first`, `last` and `rest`
  of a text copy the part once (`last` read the whole text), `chars` is
  20 % faster, `upper` and `lower` of short ASCII texts skip a copy.
- **A faster start**: the names of the built-ins are made with a faster
  hasher, and what the checker knows of them is no longer copied. `rythme
  hello.ry` runs 1.5 million instructions instead of 2.1 (most of the time
  left is the system starting the program).
- **Long programs are checked and formatted much faster**: reading a field
  (`p.name`) made the checker look at every constructor of the program
  (quadratic), it searched the list of built-ins at each call, and `rythme
  fmt` compared the syntax trees again when the code was already formatted.
  On a program of 25 000 lines: `rythme check` 3.6 to 1.6 billion
  instructions, `rythme fmt --check` 3.6 to 1.4, `rythme run` 3.8 to 1.7; in
  the language server, the diagnostics after opening it come in 0.34 s
  instead of 0.65 s, and formatting it takes 0.23 s instead of 0.48 s.

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
  44px at least for a finger; the bar gets compact on short screens,
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
- **The website is fast on a slow phone**: on Lighthouse's mobile profile (CPU
  4× slower, 1.6 Mb/s, 150 ms), the home page works in 0.8 s instead of 1.5,
  the playground's menus in 0.9 s instead of 2.1, and its first run comes in
  5.7 s instead of 18.4 (5.0 instead of 28.5 when the service worker shares
  the network, as on a real phone). The server sends everything compressed:
  the bundles from Brotli and gzip copies made once at build time
  (`site/build.sh`, the interpreter 3.2 MB → 0.8 MB), the pages and the rest
  on the fly; files linked with `?v=` are kept a year
  (`immutable`), pages are revalidated with an `ETag`. The playground compiles
  the interpreter once and hands it to its worker (no second download, a
  worker started again after *Stop* is ready at once), fetches its examples
  and its worker's script while the interpreter downloads, and lets the
  service worker save the site only after its first run. On GitHub Pages,
  GitHub's gzip does the compression. `site/tests/pwa/speed.js` measures it.
- **The bar stays at hand in landscape**: on a short screen it is compact and
  still sticks to the top, so the burger is always reachable; with the keyboard
  open in the playground it scrolls away, leaving the editor the height above
  the keyboard.
- **GitHub Pages**: the `Website` workflow says in its summary whether the
  source of Pages is "GitHub Actions", and while it is "Deploy from a branch"
  waits for GitHub's own build of the README to end, so that the site always
  deploys last instead of sometimes being replaced (a 404).
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
- **Five new sections of the website**, in the eight languages, each with its
  Markdown, structured data, place in the sitemap and in the offline app:
  *Install & tools* (`/{lang}/install/`: the ways to install of the README,
  `rythme upgrade`, the completions and `rythme man`, VS Code, Zed, Helix and
  Tree-sitter, the language server, the debugger and `rythme dap`, `rythme
  mcp`); *Reference* (`/{lang}/reference/`: every built-in function with its
  signature, documentation and example, and every error and warning code
  with its explanation, one anchor each, like `/en/reference/#E014`, made from
  `BUILTINS` and `help::CODES` at build time, with a filter that leaves the
  whole list without the browser side); *Packages* (`/{lang}/packages/`: the
  index, `rythme.toml` and ranges of versions, publishing, safety); *Rythme at
  work* (`/{lang}/work/`: CSV, JSON, logs and a command-line tool that run in
  the playground, tests and CI, `--format json`, the stability promise, the
  speed of the virtual machine as `bench/` measures it); *Exercises*
  (`/{lang}/exercises/`: the 36 exercises of `rythme exercise` by level, each
  opening in the playground with its tests, in the language of the page). The
  site's server now depends on the interpreter, so these pages cannot fall
  behind it.
- **The menu**: the five sections follow the other links in the burger menu
  of a phone, and open from *More* in the bar of a computer; without the
  browser side they are plain links. The footer lists every page.
- **Up to date with the language**: *Learn* has a ninth idea, types written
  in the code with a type with parameters (`type Tree(a)`) and E027; the
  commands of the home page add `rythme search`, `rythme exercise`, `rythme
  debug` and `rythme upgrade`; the AI page shows `run --format json -` from
  the standard input; `/llms.txt` lists the new pages and the eight
  languages; every description for search engines has 160 characters or
  fewer.

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
