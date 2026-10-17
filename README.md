# Rythme

**A simple, purely functional programming language, with help everywhere.**
Programs are files ending in `.ry`. The interpreter is written in Rust.

*[Version française →](README.fr.md)*

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

## Why Rythme?

- **Simpler than Python or Ruby.** 15 keywords (Python has 35). One way to do
  each thing. No classes, no `null`, no exceptions, no loops, no mutation:
  fewer concepts to learn, fewer bugs to chase.
- **Purely functional.** Values are immutable, functions are values, every
  `if` and `match` is an expression. Recursion in tail position never runs
  out of stack.
- **Safe like Rust.** No `null`: a missing value is an `Option` (`Some(x)` or
  `None`). No exceptions: an operation that may fail returns a `Result`
  (`Ok(x)` or `Err(e)`), and `?` passes errors on. Algebraic types, pattern
  matching, a checker that warns about forgotten cases, no silent conversion,
  no silent integer overflow.
- **Help everywhere, at all times.**
  - Every error says *where* it is, *why*, and *how to fix it*, the way Rust
    does, and suggests the right name when you make a typo.
  - Coming from another language? `for`, `return`, `null`, `x += 1`, `&&`,
    `def`, `class`... each gets an explanation of the Rythme way.
  - The program is checked **before** it runs: all the unknown names, wrong
    numbers of arguments and forgotten cases are reported at once.
  - `rythme explain E003` explains any error code in detail, with examples.
  - `rythme help`, `rythme help map`, `help(x)` in code, `:doc` in the REPL,
    `rythme learn` for a guided tour.
  - Every message, every piece of documentation exists in **English and
    French** (`--lang fr`, or automatically from your system language).

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
edit code, run it, run tests, read the help, in English or French, without
installing anything.

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

**Without installing anything else:** each published version comes with
ready-to-use programs for Linux, macOS and Windows on the
[Releases page](https://github.com/maxlestage/Rythme/releases). Download the
archive for your system, extract `rythme` (`rythme.exe` on Windows) and put it
in a folder of your `PATH`.

**With [Rust](https://rustup.rs)** (any recent stable version):

```sh
cargo install --git https://github.com/maxlestage/Rythme
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
| `rythme test hello.ry`      | run the tests written in the file              |
| `rythme fmt hello.ry`       | give the code its official layout (also a folder: `rythme fmt .`) |
| `rythme fmt --check .`      | list the files to format, without changing them (for CI) |
| `rythme help [topic]`       | help on a topic or a function                  |
| `rythme explain E003`       | explain an error code                          |
| `rythme learn`              | guided tour of the language                    |
| `rythme -e 'print(1 + 2)'`  | run a piece of code                            |

Add `--lang fr` or `--lang en` before the command to choose the language of
the messages, or set `RYTHME_LANG`.

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

# Lists, ranges, records
numbers = 1..10                           # [1, 2, ..., 10]
ada = {name: "Ada", born: 1815}
older = {..ada, born: 1816}               # a new record; ada is unchanged

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

# Other files
use "geometry.ry"
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
| [`sales.ry`](examples/sales.ry) | a report: `scan`, columns lined up with `pad_left`, `windows`, `chunks`, JSON |
| [`tree.ry`](examples/tree.ry) | a recursive type: binary search tree |
| [`primes.ry`](examples/primes.ry) | a recursive sieve, `all`, `and` |
| [`testing.ry`](examples/testing.ry) | `test` blocks |
| [`modules/`](examples/modules) | `use` to split a program in files |
| [`bonjour.ry`](examples/bonjour.ry) | names with accents |

## Editors

- **VS Code**: colors, indentation and snippets in [`editors/vscode`](editors/vscode).

## For teachers

[`docs/teaching.md`](docs/teaching.md) proposes a course outline built on
Rythme, and explains why it fits a first programming course and a
functional programming course alike.

## How it works

The interpreter has no dependency besides `rustyline` (for the interactive
mode) and is organised as a pipeline:

| File | Role |
|------|------|
| [`src/lexer.rs`](src/lexer.rs) | text → tokens (with `{...}` inside texts) |
| [`src/parser.rs`](src/parser.rs) | tokens → syntax tree, with hints for common mistakes |
| [`src/check.rs`](src/check.rs) | finds errors before running: names, arguments, forgotten cases |
| [`src/interp.rs`](src/interp.rs) | evaluation, with tail calls that do not grow the stack |
| [`src/builtins.rs`](src/builtins.rs) | ~120 built-in functions, documented in two languages, JSON included |
| [`src/help.rs`](src/help.rs) | topics, error explanations, guided tour |
| [`src/diagnostic.rs`](src/diagnostic.rs) | Rust-style error messages |
| [`src/repl.rs`](src/repl.rs) | interactive mode with completion |
| [`src/format.rs`](src/format.rs) | `rythme fmt`: the official layout, which never changes the meaning |

`cargo test` runs the examples, checks that every example of the standard
library gives the value it shows, and runs ~70 tests of the language, the
library, the messages and the commands.

## Status and roadmap

Rythme 0.1 is a complete, tested interpreter, usable today for learning and
for small programs. Honest limits, and what comes next:

- **Speed.** It is a tree-walking interpreter: about as fast as Python, far
  from compiled Rust. Next: a bytecode virtual machine.
- **Types.** Values are checked at run time (strongly: no silent
  conversions). Next: type inference in `rythme check`, so that more mistakes
  are found before running.
- **Tooling.** `rythme fmt` formats the code in the official style (see
  `rythme help fmt`). Next: a language server (errors and documentation
  inside every editor), a package manager, and a web playground (the
  interpreter compiled to WebAssembly) so that anyone can try Rythme without
  installing anything.
- **Languages.** Messages exist in English and French; adding a language means
  translating the texts of `src/`. Contributions are welcome.
- Numbers are 64-bit: whole numbers stop with an error beyond ±9.2 × 10¹⁸
  (use decimal numbers beyond that).

## License

[MIT](LICENSE).
