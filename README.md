# Rythme

**Simpler than Python and Ruby, as pleasant as Ruby: a purely functional
programming language, with help everywhere.**
Programs are files ending in `.ry`. The interpreter is written in Rust.

*[Version française →](README.fr.md) · [En español →](README.es.md) · [Auf Deutsch →](README.de.md) · [Em português →](README.pt.md) · [中文 →](README.zh.md)*

**Documentation:** the [reference manual](docs/reference.md) (the whole
language, precisely), the [guide for teachers](docs/teaching.md), and
[all the documents](docs/README.md).

```rythme
# Values never change. Everything is a function. There are no loops.
fn square(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # keep the even numbers
  |> map(square)          # square each one
  |> sum                  # add them up
  |> print                # 20
```

The same, the Ruby way (blocks, method chains, names ending with `?`, calls
without parentheses):

```rythme
names = ["Ada", "Grace", "Alan"]
names.each do |name|
  puts "Hello, #{name}!"
end

total = [1, 2, 3, 4]
  .select(even?)
  .map { |x| x * x }
  .sum
puts total                 # 20
```

## Why Rythme?

- **Simpler than Python or Ruby.** 15 keywords (Python has 35). One way to do
  each thing. No classes, no `null`, no exceptions, no loops, no mutation:
  fewer concepts to learn, fewer bugs to chase.
- **As pleasant as Ruby.** Blocks (`xs.map do |x| x * 2 end`,
  `xs.select { |x| x > 2 }`), method chains (`xs.filter(is_even).sum`, one
  step per line), calls without parentheses (`puts "hi"`, `print x, y`),
  names like `even?` and `empty?`, `unless`, symbols (`:ok`),
  `"Hello #{name}"`, and the Ruby names of the library (`puts`, `upcase`,
  `select`, `reject`, `reduce`, `each_with_index`...). `rythme help ruby`
  shows what is the same and what is different.
- **Purely functional.** Values are immutable, functions are values, every
  `if` and `match` is an expression. Recursion in tail position never runs
  out of stack.
- **Safe like Rust.** No `null`: a missing value is an `Option` (`Some(x)` or
  `None`). No exceptions: an operation that may fail returns a `Result`
  (`Ok(x)` or `Err(e)`), and `?` passes errors on. Algebraic types, pattern
  matching, a checker that warns about forgotten cases, no silent conversion,
  whole numbers that never overflow.
- **Help everywhere, at all times.**
  - Every error says *where* it is, *why*, and *how to fix it*, the way Rust
    does, and suggests the right name when you make a typo.
  - Coming from another language? `for`, `return`, `null`, `x += 1`, `&&`,
    `def`, `class`... each gets an explanation of the Rythme way.
  - The program is checked **before** it runs: all the unknown names, wrong
    numbers of arguments, operations on the wrong kinds of values
    (`"age: " + 36`, `len(5)`, a misspelled field), calls that always fail
    (`area("big")` when `area` multiplies its parameter) and forgotten cases
    are reported at once.
  - `rythme explain E003` explains any error code in detail, with examples.
  - `rythme help`, `rythme help map`, `help(x)` in code, `:doc` in the REPL,
    `rythme learn` for a guided tour.
  - Inside your editor: the problems as you type, the documentation under
    the mouse, completion, go to definition, rename (`rythme lsp`, the
    language server, for VS Code, Neovim, Helix, Emacs...).
  - Every message, every piece of documentation exists in **English, French,
    Spanish, German, Portuguese and Chinese** (`--lang fr`, `--lang es`,
    `--lang de`, `--lang pt`, `--lang zh`, or automatically from your system
    language).

Here is what a mistake looks like:

```
error[E005]: cannot use `+` with Text and Int
 --> report.ry:3:15
  |
3 | print("age: " + 36)
  |               ^ Text "age: " + Int 36
  |
  = help: to build a text with a value inside, use braces: "total: {x}"
      or convert the value first: text(x)
  = more about this error: `rythme explain E005`
```

```
error[E001]: `for` is not used in Rythme
 --> loop.ry:1:1
  |
1 | for x in numbers
  | ^^^ comes from another language
  |
  = help: Rythme has no loops. Work on the whole list instead:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      or write a recursive function. See `rythme help functional`.
```

## Try it in your browser

The website <https://maxlestage.github.io/Rythme/> has a playground that runs
the real interpreter, compiled to WebAssembly ([`playground/`](playground)):
edit code, run it, check it, run tests, format it, read the help, in English
or French, without installing anything. The website also installs as an app
(a Progressive Web App) and works offline, playground included. Its menu offers color
themes (Auto, Light, Dark, High contrast, Sepia, Tempo), and no page scrolls
sideways, from a 320px phone to a wide screen.

The [`site/`](site) folder is the website of Rythme (home, tour of the
language, examples, playground and guide for teachers, in English and French),
written in Rust with Paquets.

### Deploy the website on Heroku

[![Deploy to Heroku](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/maxlestage/Rythme)

1. Tap the button (it works from a phone), choose an app name, tap **Deploy
   app**. Heroku reads [`app.json`](app.json) and builds the playground (the
   interpreter in WebAssembly) and the server with the
   [emk/rust buildpack](https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust)
   ([`RustConfig`](RustConfig), [`heroku-build.sh`](heroku-build.sh),
   [`Procfile`](Procfile)). The first build takes a few minutes.
2. To deploy every merge into `master`: in GitHub, *Settings → Secrets and
   variables → Actions*, add the secret `HEROKU_API_KEY` (Heroku → *Account
   settings* → *API Key*) and the variable `HEROKU_APP_NAME`. The
   [`Deploy to Heroku`](.github/workflows/heroku.yml) workflow does the rest,
   and creates the app if it does not exist.

Command line: `heroku create --stack heroku-26 --buildpack emk/rust`, then
`git push heroku master`. With Docker instead: `heroku create --stack container`
([`heroku.yml`](heroku.yml), [`Dockerfile`](Dockerfile)), or on any host,
`docker build -t rythme-site . && docker run --rm -p 8080:8080 rythme-site`.
Details in [`site/README.md`](site/README.md#heroku).

## Install

**Linux and macOS** (x86_64 and arm64): the script downloads the program of
the latest version, checks its SHA-256 checksum and puts it in `~/.local/bin`.

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

Options after `sh -s --`: `--dir /usr/local/bin` (another folder),
`--version v0.1.0` (another version); `--help` lists them.

**Windows** (PowerShell): the program goes in
`%LOCALAPPDATA%\Programs\Rythme\bin`, added to your `PATH`.

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

Set `RYTHME_INSTALL_DIR` or `RYTHME_VERSION` before, or run the downloaded
file: `.\install.ps1 -Dir C:\Tools -Version v0.1.0 -NoModifyPath`.

**By hand:** each published version comes with ready-to-use programs for
Linux, macOS and Windows on the
[Releases page](https://github.com/maxlestage/Rythme/releases), and their
checksums in `SHA256SUMS`. Download the archive for your system, extract
`rythme` (`rythme.exe` on Windows) and put it in a folder of your `PATH`.

**Homebrew** (macOS, Linux) and **winget** (Windows), once the packages are
published ([`packaging/`](packaging/README.md)):

```sh
brew install maxlestage/rythme/rythme
winget install Rythme.Rythme
```

**With [Rust](https://rustup.rs)** (any recent stable version):

```sh
cargo install rythme                                    # from crates.io, once published
cargo install --git https://github.com/maxlestage/Rythme   # the latest code
```

Or from a clone of this repository:

```sh
cargo build --release        # the program is target/release/rythme
cargo install --path .       # or install it in your PATH
```

## Use

| Command                     | What it does                                   |
|-----------------------------|------------------------------------------------|
| `rythme`                    | interactive mode: type code, see the result    |
| `rythme hello.ry`           | run a program (`rythme run hello.ry` also works) |
| `rythme check hello.ry`     | find problems without running the program      |
| `rythme test hello.ry`      | run the tests written in the file (also a folder: `rythme test .`, with `--filter NAME`, `--watch`) |
| `rythme fmt hello.ry`       | give the code its official layout (also a folder: `rythme fmt .`) |
| `rythme fmt --check .`      | list the files to format, without changing them (for CI) |
| `rythme doc hello.ry`       | its documentation from the `##` comments, in Markdown (`--html`: one web page) |
| `rythme new hello`          | a new project: `rythme.toml`, `src/`, `tests/`; then `rythme run`, `rythme test`... without a file |
| `rythme install`            | download the dependencies of `rythme.toml` (git or folders), and write `rythme.lock` |
| `rythme help [topic]`       | help on a topic or a function                  |
| `rythme explain E003`       | explain an error code                          |
| `rythme learn`              | guided tour of the language                    |
| `rythme exercise`           | [30 graded exercises](docs/exercises/README.md): `list`, `start 1`, `check FILE` |
| `rythme steps fib.ry "fib(4)"` | the calculation step by step: `fib(4)` → `fib(3) + fib(2)` → ... |
| `rythme -e 'print(1 + 2)'`  | run a piece of code                            |
| `rythme lsp`                | language server, started by your editor ([editors](editors)) |

Add `--lang` and a language (`en`, `fr`, `es`, `de`, `pt` or `zh`) before the
command to choose the language of the messages, or set `RYTHME_LANG`.

## The language in two minutes

```rythme
# Names and values (they never change)
name = "Ada"
born = 1815
print("{name} was born in {born}")       # {...} inserts a value

# Functions
fn double(x) = x * 2
fn greet(name)
  message = "Hello, {name}!"
  print(message)                          # the last line is the result
end

# One definition per case, tried in order
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

# Conditions are expressions: every if has an else
fn sign(n) = if n < 0 then "negative" else if n == 0 then "zero" else "positive"

# Lists, ranges, records, dicts and sets
numbers = 1..10                           # [1, 2, ..., 10]
ada = {name: "Ada", born: 1815}
older = {..ada, born: 1816}               # a new record; ada is unchanged
ages = dict([["Ada", 36], ["Alan", 41]])  # a dictionary: keys of any kind
seen = set([1, 2, 2])                     # a set: set(1, 2)

# Your own types, and match
type Shape = Circle(radius) | Rect(width, height)

fn area(shape) = match shape
  Circle(r) -> pi * r * r
  Rect(w, h) -> w * h
end

# Errors are values
fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

fn average(a, b)
  x = parse_float(a)?                     # stops here and returns the Err
  y = parse_float(b)?
  Ok((x + y) / 2)
end

# Tests live next to the code: rythme test file.ry
test "double works" = assert_eq(double(21), 42)

# Other files, used through their names
use geometry                              # geometry.ry, next to this file
print(geometry.area(2))
```

More: the [reference manual](docs/reference.md), [`examples/`](examples),
`rythme help syntax`, `rythme learn`.

## Examples

| File | Shows |
|------|-------|
| [`hello.ry`](examples/hello.ry) | the classic first program |
| [`fizzbuzz.ry`](examples/fizzbuzz.ry) | `match` on a list, ranges, `each` |
| [`fibonacci.ry`](examples/fibonacci.ry) | definitions by case, tail calls, `##` docs |
| [`shapes.ry`](examples/shapes.ry) | your own types, guards, `sort_by` |
| [`quicksort.ry`](examples/quicksort.ry) | list patterns, `filter`, `+` on lists |
| [`words.ry`](examples/words.ry) | text processing, `group_by`, records |
| [`cipher.ry`](examples/cipher.ry) | the Caesar cipher: `char_code`, `char`, counting letters |
| [`results.ry`](examples/results.ry) | `Option`, `Result`, `?` |
| [`students.ry`](examples/students.ry) | records, record update, grouping, averages |
| [`library.ry`](examples/library.ry) | the Ruby style: `do \|x\| ... end` blocks, method chains, names ending with `?`, `unless`, symbols |
| [`sales.ry`](examples/sales.ry) | a report: `scan`, columns lined up with `pad_left`, `windows`, `chunks`, JSON |
| [`calendar.ry`](examples/calendar.ry) | dates: the page of a month, ISO 8601 texts, `days_between`, `add_days` |
| [`logs.ry`](examples/logs.ry) | reading a log with regexes: `named_captures`, `matches`, `find_all`, `replace_all`, `format_number` |
| [`tree.ry`](examples/tree.ry) | a recursive type: binary search tree |
| [`primes.ry`](examples/primes.ry) | a recursive sieve, `all`, `and` |
| [`testing.ry`](examples/testing.ry) | `test` blocks |
| [`modules/`](examples/modules) | `use geometry` then `geometry.area(2)`: a program in several files, private `_names` |
| [`bonjour.ry`](examples/bonjour.ry) | names with accents |

## Editors

- **VS Code**: the extension in [`editors/vscode`](editors/vscode): colors,
  snippets, and the language server, for the problems as you type, the
  documentation on hover, completion, the parameters of a call, go to
  definition, references, rename, an outline, formatting, the types of the
  names, *Run* and *Run test* above the code, in English or French.
- **Neovim, Helix, Zed, Emacs, Sublime Text, Kate...**: every editor that
  speaks the Language Server Protocol can start `rythme lsp`; a few lines of
  configuration for each are in [`editors/README.md`](editors/README.md).

## For teachers

[`docs/teaching.md`](docs/teaching.md) proposes a course outline built on
Rythme, and explains why it fits a first programming course and a
functional programming course alike.
[`docs/exercises/`](docs/exercises/README.md) holds 30 graded exercises, with
statements, hints, tests and solutions, that `rythme exercise` hands out and
checks in every language of the messages.

## How it works

The interpreter has no dependency besides `rustyline` (for the interactive
mode) and is organised as a pipeline:

| File | Role |
|------|------|
| [`src/lexer.rs`](src/lexer.rs) | text → tokens (with `{...}` inside texts) |
| [`src/parser.rs`](src/parser.rs) | tokens → syntax tree, with hints for common mistakes |
| [`src/check.rs`](src/check.rs) | finds errors before running: names, arguments, kinds of values, forgotten cases |
| [`src/types.rs`](src/types.rs) | what the checker knows for sure about values: their types |
| [`src/check/needs.rs`](src/check/needs.rs) | what the parameters of each function must be, and the calls that always fail |
| [`src/compile.rs`](src/compile.rs) | prepares functions: each local name becomes a numbered slot |
| [`src/interp.rs`](src/interp.rs) | evaluation, with tail calls that do not grow the stack |
| [`src/bigint.rs`](src/bigint.rs) | whole numbers beyond 64 bits, without a dependency |
| [`src/dict.rs`](src/dict.rs) | the persistent ordered tree behind dictionaries and sets, and the order of their keys |
| [`src/builtins.rs`](src/builtins.rs) | 174 built-in functions (and their Ruby names), documented in two languages, JSON included |
| [`src/builtins/`](src/builtins) | dates, files and the system, dictionaries and sets, regexes and text tools, generators of property tests |
| [`src/help.rs`](src/help.rs) | topics, error explanations, guided tour |
| [`src/diagnostic.rs`](src/diagnostic.rs) | Rust-style error messages |
| [`src/repl.rs`](src/repl.rs) | interactive mode with completion |
| [`src/project.rs`](src/project.rs), [`src/manifest.rs`](src/manifest.rs) | projects and packages: `rythme new`, `rythme install`, `rythme.toml`, `use package/file` |
| [`src/testing.rs`](src/testing.rs) | `rythme test`: files and folders, `--filter`, `--watch`, the summary |
| [`src/format.rs`](src/format.rs) | `rythme fmt`: the official layout, which never changes the meaning |
| [`src/steps.rs`](src/steps.rs) | `rythme steps`: the evaluation shown step by step, for teaching |
| [`src/lsp/`](src/lsp) | the language server (`rythme lsp`): help inside editors |

`cargo test` runs the examples, checks that every example of the standard
library gives the value it shows and that every sample of the
[reference manual](docs/reference.md) prints what it shows, and runs about
200 tests of the language, the library, the checker, the messages, the
formatter, the language server and the commands, fuzzing included.

## Status and roadmap

Rythme 0.1 is a complete, tested interpreter, usable today for learning and
for small programs. Honest limits, and what comes next (the complete list
towards 1.0 is in [`ROADMAP.md`](ROADMAP.md)):

- **Speed.** It is a tree-walking interpreter: local names become numbered
  slots before running, and most calls allocate nothing. The programs of
  [`bench/`](bench) (`bench/run.sh`) run 1.2 to 4.3 times faster than with
  Rythme 0.1; that is in the range of Python, far from compiled Rust.
  Building a text with `+` or `"{acc}..."` and adding to a list (`push`,
  `xs + [x]`, `[..xs, x]`) take constant time per addition: 100 000 of them
  take 0.1 s, instead of 23 s (texts) and 150 s (lists) before. Local
  functions that call each other make no closure per call (twice as fast).
  Next: a bytecode virtual machine.
- **Types.** Values are checked at run time (strongly: no silent
  conversions), and `rythme check` infers their types when they are certain,
  to report before running the operations that would always fail; it also
  infers what the parameters of each function must be, to report the calls
  that always fail, and the editor shows these signatures on hover.
- **Tooling.** `rythme fmt` formats the code in the official style (see
  `rythme help fmt`), `rythme doc` writes the documentation of files and
  folders from their `##` comments (see `rythme help doc`), and the language server (`rythme lsp`) brings errors and
  documentation inside every editor (see `rythme help editors`). The web
  playground runs the interpreter compiled to WebAssembly. Next: a package
  manager.
- **Languages.** Messages exist in English, French, Spanish, German,
  Portuguese and simplified Chinese; adding a language means writing one
  catalog file, `src/i18n/<code>.rs`, which a test checks against every text
  of the program (see [CONTRIBUTING](CONTRIBUTING.md#messages-in-every-language)).
  Contributions are welcome.
- **Numbers.** Whole numbers have no size limit and stay exact (up to 100000
  digits), the usual 64-bit ones computed fastest; decimal numbers are 64-bit.

## License

[MIT](LICENSE).
