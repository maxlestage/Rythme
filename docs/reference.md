# Rythme reference manual

*[Version française →](reference.fr.md)*

This manual describes the Rythme language and its tools, as implemented by
the `rythme` interpreter, version 1.0.0. It is meant to be complete and
precise: someone who wants to write a second implementation of Rythme should
find every rule here. It is also meant to be read by students: each rule
comes with a short program and the exact output it gives.

For a gentler introduction, run `rythme learn` (a guided tour), read the
[README](../README.md) and the [examples](../examples), or try the
playground on the [website](https://maxlestage.github.io/Rythme/).

**How to read the examples.** Every program in this manual was saved in a
`.ry` file and run with `rythme --lang en`; the block that follows it is the
real output, error messages included, followed by the exit code when it
matters. When a rule and the interpreter disagree, the interpreter is right
and this manual has a bug.

**Stability.** Rythme 1.0 is the first stable version. What this manual
describes (the syntax, the values, the rules of evaluation, the built-in
functions, the error and warning codes, `rythme.toml` and the commands)
holds for every 1.x version, as the [stability
promise](../README.md#stability-promise) says: later versions add to it,
and anything to be removed first gives a warning for a whole minor version.
The wording of the messages shown in the examples may still improve; their
codes stay. What changed from version to version is in the
[changelog](../CHANGELOG.md).

## Contents

1. [Lexical structure](#1-lexical-structure)
2. [Values and types](#2-values-and-types)
3. [Expressions](#3-expressions)
4. [Definitions](#4-definitions)
5. [Errors](#5-errors)
6. [Standard library](#6-standard-library)
7. [Tools](#7-tools)
- [Appendix A. Grammar](#appendix-a-grammar)
- [Appendix B. Limits of the implementation](#appendix-b-limits-of-the-implementation)

---

## 1. Lexical structure

### 1.1 Source files

A Rythme program is a text file encoded in UTF-8. By convention its name
ends with `.ry`; `rythme` runs other files too, after a warning. A byte order
mark at the very start of the file is ignored. Lines end with `\n` or
`\r\n`: a `\r` is treated as a space, and a `\r\n` inside a text becomes
`\n`.

A file is a sequence of *items* separated by new lines. An item is a
definition (`name = ...`, `fn`, `type`, `test`, `use`) or an expression.

### 1.2 Comments

`#` starts a comment that runs to the end of the line. Inside a text, `#` is
an ordinary character.

`##` starts a **documentation comment** when it is the first thing on its line
(only spaces before it). Consecutive `##` lines are joined, one space after
`##` is removed, and the text documents the next `fn` definition: `help(f)`
in a program and `:doc f` in the REPL show it. A blank line between the `##`
lines and the definition detaches them; ordinary `#` comment lines do not.
A `##` that follows code on the same line is an ordinary comment. Doc comments
written above a `type` document the type: `help` shows them for each of its
constructors (see [4.7](#47-types)). `rythme doc` makes the documentation of
whole files from them (see [7.6](#76-documentation-rythme-doc)).

```rythme
# An ordinary comment: from `#` to the end of the line.
answer = 6 * 7  # a comment can follow code

## Area of a rectangle.
## A documentation comment can span several lines.
fn area(width, height) = width * height

help(area)
```

```text
area(width, height)
  Area of a rectangle.
  A documentation comment can span several lines.
  defined at 01-comments.ry:6:4
```

### 1.3 Spaces and indentation

Spaces and tabs separate tokens and are otherwise ignored. **Indentation has
no meaning**: blocks are closed by the keyword `end`, not by indentation. The
examples use two spaces per level, which is the convention.

### 1.4 Names

A name starts with a letter or `_`, followed by letters, digits and `_`.
"Letter" and "digit" are meant in the Unicode sense, so accented letters and
other alphabets are allowed. Names are case-sensitive.

The **first character** decides the kind of name:

- an **uppercase** letter: a type or a constructor (`Shape`, `Circle`, `Some`);
- anything else (a lowercase letter, `_`, or a letter without case such as
  日): an ordinary name, for values, functions, parameters and fields.

`_` alone is the *wildcard*: it may only appear in patterns (`_` used as a
value is error E001). Names that start with `_` are never reported as unused
(W001). A number immediately followed by a letter, such as `2x`, is an error
(E018): Rythme never multiplies implicitly.

```rythme
prénom = "Ada"           # letters with accents are allowed
_draft = 0               # a name may start with `_`
item2 = 2                # digits are allowed after the first character
日本 = "Japan"           # any alphabetic character
type Answer = Yes | No   # an uppercase first letter: a type or a constructor
r = {type: "a keyword as a field name", "full name": "a text as a field name"}
print(prénom, item2, 日本, Yes, r.type, r["full name"])
```

```text
Ada 2 Japan Yes a keyword as a field name a text as a field name
```

**Names ending with `?`.** The name of a function that answers a question
may end with `?`, as in Ruby: `even?`, `empty?`, `fn adult?(person) = ...`.
Since `?` is also the operator that stops on `None` or `Err`
([5.2](#52-the--operator)), the rule is precise: a name directly followed by
`?` (no space between them) is one name ending with `?`

- after `fn` (`fn adult?(p)`), or when `(` directly follows the `?`
  (`adult?(ada)`, `xs.include?(3)`);
- otherwise, when a function of that name exists: a built-in one (`even?`,
  `empty?`... see [6](#6-standard-library)), or one defined with `fn` in the
  same file.

In every other case, `?` is the operator: `value?`, `parse_int(t)?`,
`x |> parse_int?`. To apply the operator to a value whose name is also the
name of such a function, write `(value)?` or `value ?`. Only functions can
have a name ending with `?` (E001 for `ok? = 1`), and `is_even` and the
other names keep working.

```rythme
fn adult?(person) = person.age >= 18
ada = {name: "Ada", age: 36}
print(adult?(ada), ada.adult?, even?(4), [1, 2].include?(2), [].empty?)
print([1, 2, 3, 4].select(even?))
fn first_double(xs)
  x = first(xs)?             # `first` is not followed by `(`: the operator
  Some(x * 2)
end
print(first_double([5]), first_double([]))
```

```text
true true true true true
[2, 4]
Some(10) None
```

### 1.5 Keywords and reserved words

Rythme has 15 keywords:

```text
and  do  else  end  false  fn  if  match  not  or  test  then  true  type  use
```

They cannot be used as names. They can be used as **field names**, after a dot
or before `:` in a record (`r.type`, `{type: 1}`), but not in the short form
`{type}`. `as` is not a keyword: it means something only after the path of
a `use` (`use geometry as geo`). `unless` is not a keyword either: it starts
`unless condition then a else b` ([3.9](#39-if)) only when the start of a
value follows it on the same line (a name, a number, a text, a symbol,
`not`, `if`, `match`, `fn`, or a bracket after a space); `unless(x)` and
`unless[0]`, without a space, use a value named `unless`.

The following words, taken from other languages, are **reserved**: using one
as a name is an error (E001) whose message explains how to do the same thing
in Rythme. Like keywords, they are allowed as field names.

```text
for  while  return  break  continue  let  var  const  def  function  class
elif  elsif  import  try  catch  except  raise  throw  null  nil  lambda  switch
```

### 1.6 Numbers

**Whole numbers (Int)** are written in decimal (`42`), in hexadecimal after
`0x` (`0xFF`, digits in either case) or in binary after `0b` (`0b1010`). The
prefixes are lowercase only; there is no octal notation. `_` may separate
digits anywhere after the first one (`1_000_000`, `0xFF_FF`). Whole numbers
have no size limit (see 2.1): a literal of more than 100000 digits is error
E023, in every base; a hexadecimal or binary literal with a wrong digit or no
digit (`0b12`, `0x`) is error E018.

**Decimal numbers (Float)** have digits on both sides of the point (`3.14`,
`2.0`), an exponent (`1e3`, `1.5e-3`, `6.02E23`), or both. `.5` and `5.` are
not numbers; `5.max(3)` is a method call on the Int 5 (see 3.8). `_` is
allowed in the integer and decimal parts but not in the exponent. A Float
literal too large to represent becomes `inf`.

There are no negative literals: `-7` is the operator `-` applied to `7`.

```rythme
print(42, -7, 1_000_000, 0xFF, 0b1010)
print(3.14, 2.0, 1e3, 1.5e-3, 6.02E23)
print(9223372036854775807, -9223372036854775808, 123456789012345678901234567890)
```

```text
42 -7 1000000 255 10
3.14 2.0 1000.0 0.0015 6.02e23
9223372036854775807 -9223372036854775808 123456789012345678901234567890
```

### 1.7 Texts

A text is written between straight double quotes `"`. It may span several
lines; the line breaks are part of the text. Single quotes, backquotes and
typographic quotes are errors (E018). A text that is never closed is error
E002.

Escape sequences:

| Write | To get |
|-------|--------|
| `\n` | a new line |
| `\t` | a tab |
| `\r` | a carriage return |
| `\0` | the character U+0000 |
| `\\` | a backslash |
| `\"` | a double quote |
| `\{` and `\}` | braces |
| `\#` | `#`; `\#{` gives `#{`, written as it is |
| `\u{e9}` | the character with this hexadecimal code point (here é) |

Any other `\` sequence is error E018.

**Interpolation.** `{expression}` inside a text inserts the value of the
expression, shown as `print` would show it (texts without quotes). Any
expression is allowed, including records and other texts; braces inside it
are matched, and new lines inside the braces are ignored. An empty `{}` is an
error (E001). To write a literal `{`, use `\{`. A `}` that does not close an
interpolation is an ordinary character (`\}` is also accepted). As in Ruby,
`#{expression}` is the same as `{expression}`; `\#{` writes `#{` as it is.

Texts used as a pattern, as the name of a test or as the path of `use` must
be plain texts, without interpolation.

```rythme
name = "Ada"
print("Hello, {name}! 2 + 2 = {2 + 2}")
print("tab:\t| quote: \" | backslash: \\ | braces: \{ and }")
print("\u{e9}t\u{e9} \u{1F3B5}")
print("a text can span
several lines")
print("{ {x: 1}.x } and {"nested {name}"}")
print("Ruby style: #{name}, #{2 * 21}, \#{name}")
```

```text
Hello, Ada! 2 + 2 = 4
tab:	| quote: " | backslash: \ | braces: { and }
été 🎵
a text can span
several lines
1 and nested Ada
Ruby style: Ada, 42, #{name}
```

### 1.8 Operators and punctuation

```text
+  -  *  /  //  %  **  ==  !=  <  <=  >  >=  |>  ..  .  ,  :  =  ->  ?  |
(  )  [  ]  {  }  ;
```

Some symbols of other languages are recognized only to explain what to write
instead, and are always errors: `=>`, `+=`, `-=`, `*=`, `/=`, `++`, `&&`,
`||`, `!` (alone), `&`, `...`, `'`, `` ` ``, `^`, `$`, and typographic quotes.

Characters often copied from a document or a web page are errors too
(E018), with what to do: the lookalikes of operators (`−` `–` `×` `÷` `≠`
`≤` `≥` `→`, full-width punctuation such as `（`) name the character to write
instead; invisible characters (a non-breaking or other special space, a
zero-width space, a byte order mark that is not at the start of the file, a
control character) are named by their code, such as `U+00A0`, since they
cannot be seen in the message.

```rythme
price = 6 × 7
```

```text
error[E018]: unexpected character `×`
 --> 08-lookalike.ry:1:11
  |
1 | price = 6 × 7
  |           ^
  |
  = help: write `*` instead
  = more about this error: `rythme explain E018`
```

### 1.9 New lines, `;` and continuation lines

A new line ends a statement. `;` is exactly equivalent to a new line, so
`a = 1; b = 2` is two statements. Blank lines and lines holding only a
comment count as nothing.

New lines are **ignored inside `( )`, `[ ]` and `{ }`**, except inside a block
(`do`, a function body on several lines, `match`) written within the
brackets, where they count again.

Outside brackets, an expression **continues on the next line** when:

1. the line ends with something that needs a continuation: a binary operator
   (`or and == != < <= > >= |> .. + - * / // % **`), the `=` of a definition,
   the `->` of a `match` case, `then`, `else`, or the `=` and `|` of a `type`;
2. or the next line starts with one of these tokens:
   `|>` `.` `+` `*` `/` `//` `%` `**` `==` `!=` `<` `<=` `>` `>=` `and` `or`
   `..`, or with `then` or `else` (for an `if`), or with `|` (in a `type`).

A line that starts with `-` is **not** a continuation: it is a new statement,
because `-x` is also a valid expression on its own. A call `(...)`, an index
`[...]` and `?` must start on the same line as what they apply to. `not` and
the unary `-` must have their operand on the same line.

```rythme
total = 1 +
  2                       # the line ends with an operator: it continues
result = [1, 2, 3, 4]
  |> filter(is_even)      # the next line starts with |>: it continues
  |> map(fn(x) = x * 10)
point = {
  x: 1,                   # inside ( ), [ ] and { }, new lines do not count
  y: 2,
}
kind = if total > 2
  then "big"              # `then` and `else` may start a line
  else "small"
a = 1; b = 2              # `;` is the same as a new line
print(total, result, point, kind, a + b)
```

```text
3 [20, 40] {x: 1, y: 2} big 3
```

A line that starts with `-` is a separate statement, and its value is lost:

```rythme
price = 10
discount = 3
total = price
  - discount
print(total)
```

```text
error[E024]: this line computes a value but does nothing with it
 --> 06-minus.ry:4:3
  |
4 |   - discount
  |   ^^^^^^^^^^ this value is lost
  |
  = help: a line that starts with `-` is a new statement. To continue the line above,
      end that line with `-` instead:
        total = price -
          discount
  = more about this error: `rythme explain E024`
```

---

## 2. Values and types

All values are **immutable**: no operation changes a value; "changing"
something means building a new value. Rythme is dynamically typed: values
carry their type, and every operation checks the types it receives, without
ever converting a value silently.

| Type (`type_of`) | Values | Examples |
|------------------|--------|----------|
| `Int` | whole numbers, of any size | `42`, `-7`, `0xFF` |
| `Float` | decimal numbers, 64 bits | `3.14`, `2.0`, `1e3` |
| `Text` | texts | `"hello"`, `"total: {x}"` |
| `Bool` | `true`, `false` | |
| `Unit` | `()`, "no interesting value" | `()` |
| `Symbol` | names used as values | `:ok`, `:error` |
| `List` | ordered sequences | `[1, 2, 3]`, `1..5`, `[]` |
| `Record` | named fields | `{name: "Ada", born: 1815}` |
| `Dict` | dictionaries: a value for each key, keys of any kind | `dict([["Ada", 36]])`, `dict()` |
| `Set` | sets: values without repetition | `set([1, 2, 3])`, `set()` |
| `Function` | functions, built-ins, constructors with fields | `fn(x) = x + 1`, `print`, `Some` |
| `Option` | `Some(value)`, `None` | `Some(3)` |
| `Result` | `Ok(value)`, `Err(error)` | `Err("oops")` |
| your types | the variants of a `type` | `Circle(2.0)` |

```rythme
type Shape = Circle(radius) | Square(side)
fn double(x) = x * 2
samples = [42, 3.5, "hi", true, (), [1, 2], 1..3, {a: 1}, Some(1), None, Ok(1), Circle(2), double, print, Circle]
print(map(samples, type_of))
```

```text
["Int", "Float", "Text", "Bool", "Unit", "List", "List", "Record", "Option", "Option", "Result", "Shape", "Function", "Function", "Function"]
```

### 2.1 Int

Whole numbers of any size, always exact: they grow as needed, like in Python
or Ruby, and never wrap around. The usual numbers, from −9223372036854775808
to 9223372036854775807 (64 bits), are computed fastest; beyond, the result is
still an Int, with all its digits. `+`, `-`, `*`, `//`, `%`, `**`, the
comparisons (with Floats too, see 2.11), `abs`, `gcd`, `lcm`, `is_even`,
`text`, `int`, `parse_int`, `to_json` and `parse_json` work exactly on all
of them; a function that needs a position or a count (`take`, `get`,
`repeat`...) refuses a number beyond 64 bits (E023), and `[...]` says it is
out of range (E007).

```rythme
big = 9223372036854775807
print(big + 1, big * big)
fn factorial(n) = if n == 0 then 1 else n * factorial(n - 1)
print(factorial(25), 2 ** 100)
print(-(2 ** 64) // 3, -(2 ** 64) % 3, (2 ** 64) / 3)
print(type_of(2 ** 64), 2 ** 64 - 2 ** 64 + 1)
```

```text
9223372036854775808 85070591730234615847396907784232501249
15511210043330985984000000 1267650600228229401496703205376
-6148914691236517206 2 6.148914691236517e18
Int 1
```

A whole number has at most 100000 digits: beyond, the program stops with
E023 rather than filling the memory (a number that large usually comes from
a calculation that grows without end).

```rythme
print(len(text(10 ** 99999)))
print(10 ** 100000)
```

```text
100000
error[E023]: this whole number would be too large (more than 100000 digits)
 --> 11-int.ry:2:10
  |
2 | print(10 ** 100000)
  |          ^^
  |
  = help: does it grow without end, in a function that calls itself? For an approximate value, use a decimal number (Float), like `2.0 ** 1000`
  = more about this error: `rythme explain E023`
```

Exit code: 1.

### 2.2 Float

IEEE 754 double-precision numbers. Results that are too large become `inf` or
`-inf`, and undefined results are `nan`; but a **division by zero is always
an error** (E008), for Floats as for Ints, and so is `0 ** -n`, which is
`1 / 0 ** n`. A computation that has no value among real numbers is error
E023 rather than `nan`: a negative number to a fractional power
(`(-8.0) ** 0.5`), `sqrt` of a negative number, `log` of 0 or of a negative
number, `asin` and `acos` outside −1...1.

A Float is always printed with a decimal point or an exponent, so that it
cannot be confused with an Int: `2.0`, `0.5`, `1e16`. Values whose absolute
value is at least 10¹⁶ or below 10⁻⁵ (except 0) use the exponent notation;
the others are written in full with the fewest digits that read back as the
same number.

```rythme
print(7 / 2, 6 / 3, 0.1 + 0.2, 1 / 3)
print(1e15, 1e16, 0.00001, 0.000001)
huge = 10.0 ** 400
print(huge, -huge, huge - huge, 2.0 ** 100)
print(1.0 / 0)
```

```text
3.5 2.0 0.30000000000000004 0.3333333333333333
1000000000000000.0 1e16 0.00001 1e-6
inf -inf nan 1.2676506002282294e30
error[E008]: division by zero
 --> 12-float.ry:5:11
  |
5 | print(1.0 / 0)
  |           ^
  |
  = help: check the divisor first: `if b == 0 then ... else a / b`,
      or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`
  = more about this error: `rythme explain E008`
```

### 2.3 Text

A text is a sequence of Unicode characters (code points). `len`, indexes and
`slice` count characters, not bytes: `len("été")` is 3.

A text holds at most 100 MB (100 000 000 bytes) in UTF-8: 1 byte for a
letter without accent, a digit or a space, 2 for `é`, 4 for an emoji, so
100 000 000 characters for a text without accents. Building a longer one,
with `+`, `{...}`, `join`, `replace`, `repeat`, `pad_left` or `pad_right`,
is error E023. This
limit, like the limit of the length of lists (see [2.6](#26-list)), stops a
value that grows without end before the memory of the computer runs out:

```rythme
fn double_text(t) = double_text(t + t)   # never stops
double_text("ab")
```

```text
error[E023]: this text would be too large (more than 100 MB)
 --> 13-too-large.ry:1:35
  |
1 | fn double_text(t) = double_text(t + t)   # never stops
  |                                   ^
  |
  = help: does it grow in a function that calls itself? Check that the function always reaches a case where it stops
  = call trace (most recent first):
      in `double_text` called at 13-too-large.ry:1:21
  = more about this error: `rythme explain E023`
```

### 2.4 Bool

`true` and `false`. Conditions (`if`, guards, `and`, `or`, `not`, `filter`...)
must be Bools: there is no "truthiness", so `if 1 then ...` or
`if list then ...` is error E005.

### 2.5 Unit

`()` is the only value of type `Unit`. It is the result of functions that
only have an effect (`print`, `each`; `write_file` gives `Ok(())`), of an
empty block and of a block that ends with a definition. The REPL and
`rythme -e` do not display it.

### 2.6 List

An ordered, immutable sequence of values of any types (`[1, "two", 3.0]` is
valid). Positions start at 0. Lists are built with `[...]`, with ranges, with
`+` and with the list functions.

A **range** `a..b` is the list of the whole numbers from `a` to `b`, both
included. It is an ordinary list (`type_of(1..3)` is `"List"`). It is empty
when `b < a`, both ends must be Ints (E005 otherwise), and it may hold at most
50 000 000 elements (E023).

The same limit holds for every list: building a list of more than
50 000 000 elements, with `+`, `..xs` inside brackets, `repeat`, `flatten`,
`flat_map` or `chars`, is error E023 too (see [2.3](#23-text)).

### 2.7 Record

A set of fields, each with a name (any text) and a value. Fields keep the
order in which they were written: this order is used for printing and by
`keys`, `values` and `entries`, but not by equality. A field name that is not
a plain name (`"full name"`) is written as a text.

### 2.8 Function

Named functions, anonymous functions, built-in functions and constructors
with fields are all values of type `Function`: they can be passed as
arguments, returned, and stored in lists and records.

### 2.9 Option and Result

Two types are predefined, as if they were written:

```text
type Option = Some(value) | None
type Result = Ok(value) | Err(error)
```

`Option` is for a value that may be missing, `Result` for an operation that
may fail (see [5.1](#51-errors-are-values)). Their fields have names, so
`Some(3).value` is `3` and `Err("oops").error` is `"oops"`. They are part of
the language: defining a type `Option` or `Result`, or a constructor `Some`,
`None`, `Ok` or `Err`, is error E011.

### 2.10 Your own types

`type` defines a new type and its variants (see [4.7](#47-types)). A value of
such a type prints as its constructor followed by its fields:
`Circle(2.0)`, `Leaf`.

### 2.11 Equality

`==` and `!=` accept **any two values** and never fail:

- an Int and a Float are equal when they have the same numeric value
  (`1 == 1.0`), compared exactly: the Int is not rounded to a Float first, so
  `9007199254740993 == 9007199254740992.0` is `false`;
- texts, Bools and `()` are equal when identical;
- lists are equal when they have the same length and equal elements, in order;
- records are equal when they have the same field names with equal values,
  whatever the order of the fields;
- variants are equal when they have the same type, the same constructor and
  equal fields;
- dicts are equal when they have the same keys with equal values, and sets
  when they have the same elements;
- functions are equal only to themselves (the same function value);
- values of different kinds are never equal (`"1" == 1` is `false`);
- `nan` is not equal to anything, not even itself.

```rythme
type Color = Red | Green
print(1 == 1.0, [1, 2] == [1, 2], {a: 1, b: 2} == {b: 2, a: 1})
print(Some(1) == Some(1), Red != Green, "1" == 1, [] == {})
```

```text
true true true
true true false false
```

### 2.12 Order

`<`, `<=`, `>` and `>=` compare:

- numbers, Int and Float mixed, by their exact value;
- texts, character by character by Unicode code point (so `"Z" < "a"`, and
  accented letters come after `z`: this is not dictionary order);
- Bools (`false < true`), and `()` with itself;
- lists, element by element, a shorter list coming first when it is a prefix
  of the other;
- two values of the **same type**: first by constructor, in the order of the
  type's definition (`None < Some(x)`, `Ok(x) < Err(e)`, and for
  `type Size = Small | Medium | Large`, `Small < Medium < Large`), then field
  by field;
- sets, element by element in their order, like lists; and dicts the same
  way, as the lists of their `[key, value]` pairs. This is not inclusion:
  `set([1, 2]) < set([1, 3])` is `true` (use `is_subset` for inclusion).

Anything else (records, functions, values of two different types, a number
and a text...) is error E005. `sort`, `sort_by`, `min` and `max` use the same
order and fail the same way.

```rythme
type Size = Small | Medium | Large
print(1 < 2.5, "Zebra" < "apple", "apple" < "banana", false < true)
print([1, 2] < [1, 3], [1, 2] < [1, 2, 0], Some(1) < Some(2))
print(sort(["pear", "Fig", "apple", "été"]))
print(None < Some(1), Ok(5) < Err(1), Small < Large)
print(sort([Some(2), None, Some(1)]))
```

```text
true true true true
true true true
["Fig", "apple", "pear", "été"]
true true true
[None, Some(1), Some(2)]
```

When the kinds of the values are certain, as here, the error is found before
the program runs (see [5.5](#55-the-check-before-running)):

```rythme
print({a: 1} < {a: 2})
```

```text
error[E005]: cannot use `<` with Record and Record
 --> 14b-ordering.ry:1:14
  |
1 | print({a: 1} < {a: 2})
  |              ^ Record {a: 1} < Record {a: 2}
  |
  = more about this error: `rythme explain E005`
```

### 2.13 How values are shown

Every value has two written forms:

- the **display** form, used by `print`, by `{...}` in texts, by `text(x)` and
  by `join`: a text is shown as is, without quotes;
- the **code** form, used by `debug`, by the REPL and `rythme -e`, inside lists
  and records, and in error messages: a text is shown between quotes, with
  `"`, `\`, new lines, tabs, carriage returns and `{` escaped.

Apart from texts, both forms are the same:

| Value | Shown as |
|-------|----------|
| Int | `42`, `-7` |
| Float | see [2.2](#22-float): `2.0`, `0.5`, `1e16`, `inf`, `-inf`, `nan` |
| Bool | `true`, `false` |
| Unit | `()` |
| List | `[1, "a", 2.0]` |
| Record | `{name: "Ada", "full name": "Ada L."}`: a field name is written as a text when it is not a plain name |
| variant | `None`, `Some(3)`, `Circle(2.0)` |
| Dict | `dict("Ada" => 36, [0, 0] => "start")`, `dict()`: the pairs in the order of the keys |
| Set | `set(1, 2, 3)`, `set()`: the elements in order |
| function | `<fn name>` for a named function or a built-in, `<fn>` for an anonymous one |
| constructor with fields | `<constructor Circle>` (in French: `<constructeur Circle>`) |
| Symbol | `:ok` |

`print` writes its arguments separated by one space, then a new line, on the
standard output. `debug(x)` writes `[debug file:line:column]` and the code form
of `x` on the **error output**, then returns `x` unchanged, so it can be put in
the middle of a pipeline.

```rythme
items = ["a", "b"]
print("text", 42, 2.0, items, {name: "Ada", "full name": "Ada L."}, None, ())
print(text(items), "{items}", text("plain"))
size = debug(items) |> len
print(size)
print([print, fn(v) = v, Some, None])
```

```text
text 42 2.0 ["a", "b"] {name: "Ada", "full name": "Ada L."} None ()
["a", "b"] ["a", "b"] plain
[debug 15-print.ry:4:8] ["a", "b"]
2
[<fn print>, <fn>, <constructor Some>, None]
```

### 2.14 Symbol

A **symbol** is a name used as a value, as in Ruby: `:ok`, `:error`,
`:pending`. It is written `:` directly followed by a name, and shown the same
way; a `?` right after the name is part of it, as in Ruby (`:even?`). Two symbols are equal when their names are; they are ordered by name;
they are never equal to a text (`:ok == "ok"` is false, and the checker warns,
W006). They are used in `match` patterns, and `to_json` writes them as texts.

A `:` is the one of a record when it follows a field name that follows `{`
or `,` (`{status:ok}` is the field `status` with the value of `ok`);
everywhere else, `:name` is a symbol: `{status: :ok}`, `[:a, :b]`,
`f(:ok)`.

```rythme
status = :pending
fn label(s) = match s
  :ok -> "done"
  :pending -> "waiting"
  _ -> "unknown"
end
print(status, label(status), type_of(:ok), :a < :b, :ok == :ok)
print({status: :ok, count: 2}, to_json([:ok, 1]), text(:ok))
```

```text
:pending waiting Symbol true true
{status: :ok, count: 2} ["ok", 1] :ok
```

### 2.15 Dict and Set

A **dict** (dictionary) gives a value for each of its keys; a **set** holds
values without repetition. Like every value, they never change. They have no
syntax of their own: they are built with the functions `dict` and `set`, and
used with the functions of the [standard library](#6-standard-library).

- `dict()` is the empty dict; `dict(pairs)` takes a list of `[key, value]`
  pairs (when a key comes several times, its last value wins);
  `dict(record)` takes the names of the fields as keys (texts).
- `set()` is the empty set; `set(list)` keeps each element once.
- Keys and elements may be any value but a function or `nan` (E023):
  numbers (whole numbers of any size too), texts, symbols, Bools, `()`,
  lists, records, variants, dicts and sets. Two
  keys are the same key when they are equal (`==`, see [2.11](#211-equality)):
  `1` and `1.0` are one key, and so are `{x: 1, y: 2}` and `{y: 2, x: 1}`;
  the key that stays is the one put first.
- They are kept **in the order of their keys**, whatever the order they were
  built in: the order of [2.12](#212-order), extended to all of these values
  (values of different kinds go in the order `()`, Bools, numbers, texts,
  symbols, lists, records, variants, dicts, sets; symbols by their names;
  records by their field names in
  alphabetical order, then by their values). This order is used for
  printing, by `keys`, `values`, `entries`, `filter`, `map_values` and
  `to_json`.
- `dict[key]` gives the value of a key, E010 if there is none (see
  [3.7](#37-indexing)); `get(dict, key)` gives an Option.
- They are persistent balanced trees: `get`, `has`, `[...]`, `put`, `push`
  and `remove` take a time in O(log n), and a new dict or set shares almost
  all of the old one instead of copying it.
- They print as `dict(key => value, ...)` and `set(a, b, ...)`. These forms
  only show what they hold: in code, write `dict([[key, value], ...])` and
  `set([a, b, ...])`.

```rythme
ages = dict([["Bob", 25], ["Ada", 36]])
print(ages, ages["Ada"], get(ages, "Eve"), len(ages))
older = put(ages, "Bob", 26)
print(older, ages)
print(set([3, 1, 3, 2]), set([1, 1.0]), dict({x: 1, y: 2}))
grid = dict([[[0, 0], "start"], [[2, 1], "goal"]])
print(grid[[2, 1]], keys(grid))
print(type_of(ages), type_of(set()), dict() == dict([]))
```

```text
dict("Ada" => 36, "Bob" => 25) 36 None 2
dict("Ada" => 36, "Bob" => 26) dict("Ada" => 36, "Bob" => 25)
set(1, 2, 3) set(1) dict("x" => 1, "y" => 2)
goal [[0, 0], [2, 1]]
Dict Set true
```

```rythme
type Color = Red | Green
print(dict([[1, "a"]]) == dict([[1.0, "a"]]), set([[1, 2], [1]]) == set([[1], [1, 2]]))
print(set([1, 2]) < set([1, 3]), sort([set([2]), set([1, 3]), set()]))
print(set([Green, Red, 2, "x", [1], ()]))
```

```text
true true
true [set(), set(1, 3), set(2)]
set((), 2, "x", [1], Red, Green)
```

In the Ruby style (see [3.14](#314-blocks-given-to-calls)), `each`, `select`
and `reject` give a block the key and the value of each pair of a dict,
`key?` and `include?` are `has` and `contains`, and `size` and `length` are
`len`. Symbols and whole numbers of any size are keys like the others:

```rythme
ages = dict([[:ada, 36], [:alan, 41]])
ages.each do |name, age|
  print(name, age)
end
print(ages.key?(:ada), ages.size, ages.select { |_, age| age > 40 })
seen = set([10 ** 20, 1, 10 ** 20])
print(seen, seen.include?(1), seen.length, to_json(ages))
```

```text
:ada 36
:alan 41
true 2 dict(:alan => 41)
set(1, 100000000000000000000) true 2 {"ada": 36, "alan": 41}
```

A function cannot be a key (functions cannot be compared):

```rythme
fn double(x) = x * 2
print(set([1, double]))
```

```text
error[E023]: `set`: <fn double> cannot be a key of a dict, nor an element of a set
 --> 16-set.ry:2:7
  |
2 | print(set([1, double]))
  |       ^^^^^^^^^^^^^^^^
  |
  = help: keys and elements must be values that can be compared: anything but functions and `nan`
  = more about this error: `rythme explain E023`
```

---

## 3. Expressions

Everything that gives a value is an expression, including `if`, `match` and
blocks. Expressions are evaluated **from left to right**: operands, arguments,
list elements and record fields, in the order they are written.

### 3.1 Operators: precedence and associativity

From the tightest binding to the loosest:

| Level | Operators | Associativity |
|-------|-----------|---------------|
| 1 | call `f(...)`, index `x[i]`, field `x.f`, method `x.f(...)`, `?` | left to right |
| 2 | `**` | right (`2 ** 3 ** 2` is `2 ** 9`); its right operand may start with `-` |
| 3 | unary `-` | prefix (`-a * b` is `(-a) * b`; `-2 ** 2` is `-(2 ** 2)`) |
| 4 | `*` `/` `//` `%` | left |
| 5 | `+` `-` | left |
| 6 | `..` | none: one range per expression |
| 7 | `\|>` | left |
| 8 | `==` `!=` `<` `<=` `>` `>=` | none: `a < b < c` is an error |
| 9 | `not` | prefix (`not a == b` is `not (a == b)`) |
| 10 | `and` | left |
| 11 | `or` | left |

Two extra rules prevent common misreadings, both reported as E001: a
comparison cannot be chained (`0 < x < 10`; write `0 < x and x < 10`), and an
arithmetic operator or `..` cannot directly follow a `|>` step
(`x |> f + 1`; write `(x |> f) + 1`).

```rythme
fn inc(x) = x + 1
print(2 + 3 * 4, (2 + 3) * 4)         # 14 20
print(2 ** 3 ** 2, -2 ** 2, 2 ** -1)  # ** groups to the right and before unary -
print(10 - 3 - 2, 7 // 2 * 2)          # left to right
print(1 + 2 |> inc)                    # |> comes after arithmetic: inc(3)
print(1..3 |> map(inc))                # .. comes before |>
print(5 |> inc == 6)                   # |> comes before comparisons
print(not 1 == 2, not true and false)  # not comes after comparisons, before and
print(true or false and false)         # and comes before or
```

```text
14 20
512 -4 0.5
5 6
4
[2, 3, 4]
true
true false
true
```

```rythme
x = 5
print(0 < x < 10)
```

```text
error[E001]: comparisons cannot be chained
 --> 22-chained.ry:2:13
  |
2 | print(0 < x < 10)
  |             ^
  |
  = help: combine them with `and`: `a < b and b < c`
  = more about this error: `rythme explain E001`
```

```rythme
fn inc(x) = x + 1
print(1 |> inc + 1)
```

```text
error[E001]: `+` cannot follow a `|>` step directly
 --> 24-pipe-error.ry:2:16
  |
2 | print(1 |> inc + 1)
  |                ^
  |
  = help: `|>` comes after the calculations: put the pipeline in parentheses,
      like `(x |> f) + 1`, or continue it with a function: `x |> f |> fn(v) = v + 1`
  = more about this error: `rythme explain E001`
```

### 3.2 Arithmetic

| Operation | Ints | With a Float | Other values |
|-----------|------|--------------|--------------|
| `a + b` | Int, exact at any size | Float | two texts: joined; two lists: joined; otherwise E005 |
| `a - b`, `a * b` | Int, exact at any size | Float | E005 |
| `a / b` | **always a Float** (`6 / 3` is `2.0`) | Float | E005 |
| `a // b` | Int, rounded down (toward −∞) | Float, rounded down | E005 |
| `a % b` | Int, with the sign of `b` | Float: the exact remainder, with the sign of `b` | E005 |
| `a ** b` | Int if `b >= 0`, exact at any size, Float if `b < 0` (`0 ** -n` is E008) | Float (E023 for a negative number to a fractional power) | E005 |
| `-a` | Int | Float | E005 |

`/`, `//` and `%` by zero are error E008, for Ints and Floats. Rythme never
converts between texts and numbers: `"1" + 1` is E005 (write `"{x}"`,
`text(x)`, `int(t)` or `float(t)`).

```rythme
print(7 / 2, 7 // 2, 7 % 3, 2 ** 10, 2 ** -1)
print(-7 // 2, -7 % 3, 7 % -3)
print(7.5 // 2, 7.5 % 2, 1 + 2.0, 9 ** 0.5)
print("ab" + "cd", [1, 2] + [3])
```

```text
3.5 3 1 1024 0.5
-4 2 -2
3.0 1.5 3.0 3.0
abcd [1, 2, 3]
```

### 3.3 Comparisons and logic

`==` and `!=` follow [2.11](#211-equality); `<`, `<=`, `>`, `>=` follow
[2.12](#212-order).

`and`, `or` and `not` need Bools (E005 otherwise). `and` and `or` are
**short-circuit**: the right side is evaluated only when the left side does
not decide the result (`false and ...` is `false` without evaluating `...`).

### 3.4 Ranges

`a..b` builds the list `[a, a + 1, ..., b]`, see [2.6](#26-list). Both sides
are arithmetic expressions: `1..n + 1` is `1..(n + 1)`. In a pattern, `1..9`
is not a list but any number from 1 to 9 (see [3.10](#310-match-and-patterns)).

### 3.5 Calls

`f(a, b)` evaluates `f`, then the arguments from left to right, then calls
the function. A trailing comma is allowed, and new lines inside the
parentheses are ignored. The number of arguments must be exactly the number
of parameters (E004; some built-ins accept a range of counts). Calling a value
that is not a function is error E009, and so is `None()`: a constructor
without fields is a value, not a function. There are no named arguments,
default values or partial application. A statement, or a result such as a
branch of `if`, may also call a function without parentheses: `print x, y` (see [3.15](#315-calls-without-parentheses)).

### 3.6 Pipes `|>`

`x |> step` passes `x` as the **first argument** of a call. The step is a
function, possibly followed by its other arguments:

| Written | Means |
|---------|-------|
| `x \|> f` | `f(x)` |
| `x \|> f(a, b)` | `f(x, a, b)` |
| `x \|> r.f` | `r.f(x)`: the function stored in the field `f` of `r` |
| `x \|> v.m(a)` | `v.m(x, a)`, see [3.8](#38-fields-and-method-calls) |
| `x \|> fn(v) = ...` | the anonymous function applied to `x` |
| `x \|> f?`, `x \|> f[0]`, `x \|> f(a).name` | `f(x)?`, `f(x)[0]`, `f(x, a).name`: what follows the step applies to its result |

A `.name` written right after the function of a step, before any
parentheses, belongs to the function (`x |> r.f` calls the field `f` of `r`);
written after the parentheses, it applies to the result.

The head of a step is a single simple expression: a name, a constructor, an
anonymous function or an expression in parentheses. The body of an anonymous
function extends as far right as possible, so such a step should be the last
one (or be put in parentheses). Pipes have a lower precedence than arithmetic
and `..`, and a higher one than comparisons (see [3.1](#31-operators-precedence-and-associativity)).
A line that starts with `|>` continues the previous line.

```rythme
fn add(a, b) = a + b
tools = {double: fn(x) = x * 2}
print(3 |> add(10))                     # add(3, 10)
print(3 |> tools.double)                # tools.double(3)
print(3 |> Some)                        # Some(3)
print(" Hi " |> trim |> upper)          # upper(trim(" Hi "))
print([1, 2, 3] |> map(fn(x) = x * 2) |> sum)
print([4, 5] |> reverse[0])             # [0] applies to reverse([4, 5])
print(3 |> fn(x) = x * 100)             # an anonymous function as a step
fn total(a, b)
  x = a |> parse_int?                   # ? applies to parse_int(a)
  Ok(x + b)
end
print(total("2", 3), total("two", 3))
```

```text
13
6
Some(3)
HI
12
5
300
Ok(5) Err("\"two\" is not a whole number")
```

### 3.7 Indexing

| Written | Gives | When it fails |
|---------|-------|---------------|
| `list[i]` | the element at position `i` (from 0); `-1` is the last, `-2` the one before... | E007 out of range, E005 if `i` is not an Int |
| `text[i]` | the character at position `i`, as a text | E007, E005 |
| `record["name"]` | the field with this name | E010 if there is none |
| `dict[key]` | the value of this key | E010 if there is none |

`get(x, i)` does the same but returns an Option instead of failing.

### 3.8 Fields and method calls

`value.name` reads a field: the field of a record, or the field of a variant
named in its `type` (`Circle(2).radius`). A missing field is error E010. The
name after the dot may be any name, a keyword or a reserved word.

`value.name(args)` is a **method-style call**:

- if `value` is a record that has a field `name`, it calls the function stored
  in that field with `args` (the record itself is not passed);
- otherwise it calls the function `name` visible at that place with
  `(value, args...)`: `"hi".upper()` is `upper("hi")`, `10.max(20)` is
  `max(10, 20)`.

`value.name` **without parentheses** reads the field `name` when the value
has one (a record field, or a field of a variant); when it has none, it calls
the function `name` with the value: `xs.sum` is `sum(xs)`, `"hi".upcase` is
`upcase("hi")`, `shape.area` is `area(shape)` when `Circle` has no field
`area`. The field always wins, so existing records keep their meaning. A
name ending with `?` works the same way: `xs.empty?`, `n.even?`. When there
is neither a field nor a function, it is error E010.

The checker warns (W005) when no function and no record field of the file has
this name. A line that starts with `.` continues the previous line, so method
calls can be chained over several lines, one step per line, the way Ruby
programs are written (blank lines and comments may come between the steps).

```rythme
type Shape = Circle(radius)
xs = [10, 20, 30]
ada = {name: "Ada", "full name": "Ada Lovelace"}
print(xs[0], xs[-1], "été"[1], ada.name, ada["full name"], Circle(2).radius)
print("hello".upper(), [3, 1, 2].sort(), 10.max(20))
calc = {twice: fn(x) = x * 2}
print(calc.twice(21))                   # the function stored in the field
print(get(xs, 5))
print(xs[5])
```

```text
10 30 t Ada Ada Lovelace 2
HELLO [1, 2, 3] 20
42
None
error[E007]: index 5 is out of range: the list has 3 element(s)
 --> 25-access.ry:9:10
  |
9 | print(xs[5])
  |          ^ no element here
  |
  = help: valid indices go from 0 to 2 (or -1 for the last element)
  = help: to get an Option instead of an error, use `get(x, 5)`
  = more about this error: `rythme explain E007`
```

Without parentheses, and over several lines:

```rythme
xs = [3, 1, 4, 1, 5, 9, 2, 6]
print(xs.sum, xs.size, xs.sort.first, "rythme".upcase, xs.uniq.size)
top = xs
  .sort
  .reverse
  # the three largest
  .first(3)
print(top)
point = {x: 1, sum: "a field"}
print(point.sum)                        # the field wins over the function
```

```text
31 8 Some(1) RYTHME 7
[9, 6, 5]
a field
```

### 3.9 `if`

```text
if condition then value else other_value
```

The condition must be a Bool (E005). Only the chosen branch is evaluated.
**`else` is mandatory** (E019), because `if` always gives a value. Each
branch is a single expression; for several statements use `do ... end`.
`else if` chains conditions. `then` and `else` may start a new line.

```rythme
fn sign(n) = if n < 0 then "negative" else if n == 0 then "zero" else "positive"
print(sign(-2), sign(0), sign(3))
fn describe(n) =
  if n > 100 then do
    print("a big number")
    "big"
  end else "small"
print(describe(500))
```

```text
negative zero positive
a big number
big
```

```rythme
x = 5
if x > 0 then print("positive")
```

```text
error[E019]: this `if` has no `else`
 --> 26b-if-error.ry:2:1
  |
2 | if x > 0 then print("positive")
  | ^^ every `if` must have an `else`
  |
3 | 
  | - expected `else` here
  |
  = help: in Rythme, `if` always gives a value, so it needs both cases:
        if age >= 18 then "adult" else "minor"
  = more about this error: `rythme explain E019`
```

When there is nothing to do in one case, give the unit value:
`if x > 0 then print("positive") else ()`.

**`unless`**, as in Ruby, is the opposite of `if`:
`unless condition then a else b` gives `b` when the condition is true and `a`
when it is false; it is exactly `if condition then b else a`. Like `if`, it
always has an `else` (E019). See [1.5](#15-keywords-and-reserved-words) for
when the word `unless` starts it.

```rythme
fn describe(stock) = unless stock > 0 then "sold out" else "{stock} left"
print(describe(0), describe(3))
```

```text
sold out 3 left
```

### 3.10 `match` and patterns

```text
match subject
  pattern -> result
  pattern if guard -> result
  ...
end
```

The subject is written on the same line as `match`; each case starts on its
own line (or after a `;`), and `end` closes the `match`. There must be at
least one case, and no `|` before a case. The result may start on the line
after `->`.

The cases are tried **in order**. The first case whose pattern matches the
value, and whose guard (if any) is `true`, is chosen; the names bound by the
pattern are visible in its guard and its result. If no case matches, the
program stops with error E006. Before running, the checker warns about
forgotten cases it can prove (W002), about cases placed after a case that
accepts everything or the same values, and empty ranges (W004), and about
ranges that overlap (W010).

| Pattern | Matches |
|---------|---------|
| `_` | anything; binds nothing |
| `name` | anything; binds it to `name` |
| `42`, `-1`, `2.5`, `-0.5` | a number equal to this one (an Int pattern matches an equal Float, and the other way round: `0` matches `0.0`) |
| `1..9`, `-5..-1`, `0.0..0.5` | a number from the first to the second, both included: an Int or a Float (`89.5` matches `0..100` but neither `0..89` nor `90..100`). The bounds are numbers written in the code |
| `"text"` | exactly this text (no `{...}` in a pattern) |
| `"Mr " + name`, `"#" + _` | a text that starts with this text; `name` is bound to the rest of the text (after `+` comes a name or `_`) |
| `true`, `false` | this Bool |
| `()` | the unit value |
| `Leaf`, `None` | this constructor without fields |
| `Circle(p)`, `Rect(p1, p2)` | a variant of this constructor whose fields match the patterns |
| `[]`, `[p]`, `[p1, p2]` | a list of exactly this length whose elements match |
| `[p1, ..rest]`, `[p1, ..]`, `[..rest]` | a list of at least this length; `rest` is bound to the remaining elements. `..` must come last |
| `{name}`, `{name, age: p}`, `{"full name": p}`, `{name, ..}` | a record that has at least these fields (it may have others); `{name}` is short for `{name: name}`; a `..` among the fields changes nothing (`{name, ..}` is `{name}`), and `{name, ..others}` is E001: the other fields cannot be given a name; `{}` matches any record |
| `p1 \| p2 \| p3` | any of the alternatives; alternatives cannot bind names (E001) |
| `(p)` | the same as `p` |

A name may appear only once in a pattern (E011): to compare two parts, use
two names and a guard, `[a, b] if a == b -> ...`. A constructor used in a
pattern must exist (E003) and have the right number of fields (E004).

```rythme
type Shape = Circle(radius) | Rect(width, height) | Dot

fn describe(value) = match value
  0 -> "the number zero"
  -1 | -2 -> "minus one or minus two"
  2.5 -> "two and a half"
  "hi" -> "a greeting"
  true -> "true"
  () -> "the unit value"
  [] -> "an empty list"
  [x] -> "a list with one element, {x}"
  [first, ..rest] if len(rest) > 3 -> "a long list starting with {first}"
  [first, second, ..] -> "a list starting with {first}, {second}"
  {name, age: 36} -> "{name}, 36 years old"
  {name} -> "a record with a name: {name}"
  Circle(r) -> "a circle of radius {r}"
  Rect(w, h) if w == h -> "a square"
  Rect(_, _) | Dot -> "a rectangle or a dot"
  Some(Some(v)) -> "an Option inside an Option: {v}"
  _ -> "something else"
end

samples = [0, -2, 2.5, "hi", true, (), [], [7], 1..10, [1, 2, 3],
  {name: "Ada", age: 36}, {name: "Alan", age: 41}, Circle(1), Rect(2, 2), Dot, Some(Some(4)), 0.0, 99]
samples |> each(fn(v) = print(describe(v)))
```

```text
the number zero
minus one or minus two
two and a half
a greeting
true
the unit value
an empty list
a list with one element, 7
a long list starting with 1
a list starting with 1, 2
Ada, 36 years old
a record with a name: Alan
a circle of radius 1
a square
a rectangle or a dot
an Option inside an Option: 4
the number zero
something else
```

When no case matches:

```rythme
fn name(n) = match n
  1 -> "one"
  2 -> "two"
end
print(name(1))
print(name(3))
```

```text
warning[W002]: this `match` does not handle every case
 --> 27b-nomatch.ry:1:14
  |
1 | fn name(n) = match n
  |              ^^^^^^^
  |
  = note: not handled: `_`
  = help: add a last case `_ -> ...` for all the other values

one
error[E006]: no case of this `match` handles Int 3
 --> 27b-nomatch.ry:1:20
  |
1 | fn name(n) = match n
  |                    ^ this value
  |
  = help: add a case for this value, or a last case `_ -> ...` that handles everything else
  = call trace (most recent first):
      in `name` called at 27b-nomatch.ry:6:7
  = more about this error: `rythme explain E006`
```

Ranges and the start of a text:

```rythme
fn grade(score) = match score
  90..100 -> "A"
  70..89 -> "B"
  0..69 -> "C"
  _ -> "not a score"
end

fn coin(0.0..0.5) = "heads"
fn coin(_) = "tails"

fn polite("Mr " + name) = "Dear Mister {name}"
fn polite("Ms " + name) = "Dear Madam {name}"
fn polite(name) = "Dear {name}"

print([95, 70, 100.0, 89.5, -3] |> map(grade))
print(coin(0.25), coin(0.75), coin(-2))
print(polite("Mr Smith"), "/", polite("Ms Lee"), "/", polite("Ada"))
```

```text
["A", "B", "A", "not a score", "not a score"]
heads tails tails
Dear Mister Smith / Dear Madam Lee / Dear Ada
```

A range never covers every number, so a `match` on numbers still needs a last
case `_` (W002). The checker reports, before running, a range that shares
numbers with a range before it (W010: these numbers always go to the first
one), a case whose numbers are all handled by the cases before it, even by
several of them together (W004), and an empty range like `99..65` (W004):

```rythme
fn age_group(age) = match age
  0..18 -> "young"
  18..64 -> "adult"
  20..30 -> "twenties"
  99..65 -> "senior"
  _ -> "older"
end
print(age_group(18))
```

```text
warning[W010]: these ranges overlap
 --> 54-ranges.ry:3:3
  |
3 |   18..64 -> "adult"
  |   ^^^^^^ 18 is also here
  |
2 |   0..18 -> "young"
  |   ----- 18 goes here, the first that fits
  |
  = help: both ends of a range are included: make the ranges follow each other without overlapping, like `0..9` then `10..19`

warning[W004]: this case can never be reached
 --> 54-ranges.ry:4:3
  |
4 |   20..30 -> "twenties"
  |   ^^^^^^
  |
3 |   18..64 -> "adult"
  |   ------ this case already handles these values
  |
  = help: a value goes to the first case that fits: remove this case, or change it

warning[W004]: this range never matches
 --> 54-ranges.ry:5:3
  |
5 |   99..65 -> "senior"
  |   ^^^^^^ no number goes from 99 up to 65
  |
  = help: a range goes from the smaller number to the larger one: `65..99`

young
```

Only the start of a text can be matched: for its end, or for bounds that are
names, use a guard (`s if ends_with(s, "!") -> ...`,
`n if n >= low -> ...`).

Patterns are also used in definitions (`[a, b] = list`, see
[4.1](#41-names)) and in function parameters (see [4.5](#45-functions)).

### 3.11 Blocks and `do`

A **block** is a sequence of statements, one per line, closed by `end`. The
body of a function written on several lines, the body of a `test` written on
several lines and `do ... end` are blocks. A statement is a definition
(`pattern = expression`, a local `fn`) or an expression.

- The value of a block is the value of its **last statement**, when it is an
  expression. When the last statement is a definition, or the block is empty,
  the value is `()`.
- An expression statement that is not the last one must have an effect: it
  must call a function (or use `?`). Otherwise its value would be lost, and
  this is error E024.
- `do ... end` is an expression: a block written anywhere a value is
  expected. It opens a new scope (see [4.3](#43-scopes-and-shadowing)). It
  may be written on one line: `do 1 + 1 end`.
- `type`, `test` and `use` are only allowed at the top level of a file.

```rythme
fn describe(n)
  kind = if n % 2 == 0 then "even" else "odd"
  "{n} is {kind}"                 # the last line gives the result
end
fn nothing()
end
area = do
  width = 3
  height = 4
  width * height
end
print(describe(7), nothing(), area, do 1 + 1 end)
```

```text
7 is odd () 12 2
```

### 3.12 Anonymous functions

```text
fn(parameters) = expression

fn(parameters)
  statements
end
```

An anonymous function is a value. Its parameters are patterns, like those of
named functions. It can use the names visible where it is written (it is a
**closure**; since values never change, it simply keeps them). The body of
the one-line form extends as far right as possible. An anonymous function has
a single definition and no guard: use `match` inside it to choose between
cases. When it is defined by `name = fn(...) ...`, at the top level or
inside a block, the function can call itself by `name`. A named function (`fn name(...)`) cannot be written
inside an expression (E001).

```rythme
double = fn(x) = x * 2
fn adder(n) = fn(x) = x + n           # the anonymous function keeps n
add10 = adder(10)
show = fn(x)
  y = x * 2
  "value: {y}"
end
print(double(4), add10(5), show(3))
print([[1, 2], [3, 4]] |> map(fn([a, b]) = a * b))
fn countdown(n)
  step = fn(k) = if k == 0 then "liftoff" else "{k} " + step(k - 1)
  step(n)
end
print(countdown(3))
```

```text
8 15 value: 6
[2, 12]
3 2 1 liftoff
```

### 3.13 Lists and records

`[a, b, c]` builds a list. `..xs` inside the brackets inserts all the
elements of the list `xs`, at any position (`[0, ..xs, 9]`); spreading a
value that is not a list is E005.

`{name: value, ...}` builds a record. The field name may be a name, a
keyword, a reserved word, a capitalized name or a text (`{"full name": x}`).
`{name}` is short for `{name: name}`. `..r` copies all the fields of the
record `r`. The items are processed from left to right: a later field with
the same name replaces the earlier value but keeps its position. Writing the
same field name twice explicitly is E011. **This is how a record is
"updated"**: `{..ada, born: 1816}` is a new record; `ada` does not change.

Trailing commas are allowed in both.

```rythme
ada = {name: "Ada", born: 1815}
print({..ada, born: 1816}, {..ada, city: "London"}, {born: 0, ..ada})
name = "Grace"
print({name, born: 1906})
xs = [1, 2]
print([0, ..xs, 3], xs + [9], 1..5, 5..1)
```

```text
{name: "Ada", born: 1816} {name: "Ada", born: 1815, city: "London"} {born: 1815, name: "Ada"}
{name: "Grace", born: 1906}
[0, 1, 2, 3] [1, 2, 9] [1, 2, 3, 4, 5] []
```

### 3.14 Blocks given to calls

As in Ruby, a function can be given to a call as a **block**, written after
the call: `do |parameters| ... end`, or `{ |parameters| ... }`. A block is
the **last argument** of the call, and it is exactly an anonymous function:
`xs.map do |x| x * 2 end` is `map(xs, fn(x) = x * 2)`.

- It follows a call (`map(xs) do ... end`), a method call
  (`xs.reduce(0) do |sum, x| ... end`), a `.name` without parentheses
  (`xs.each do |x| ... end` is `xs.each(fn(x) = ...)`), or a name
  (`run do ... end` is `run(fn() = ...)`).
- `do` (or `{`) must be on the line of the call; the block can then go on over
  several lines, up to `end` (or `}`), or stay on one line.
- The parameters go between bars, separated by commas; they may be patterns
  (`|[key, value]|`, `|{name, ..}|`, `|Some(x)|`); without parameters, write
  `do ... end` or `{ || ... }`.
- `{ |x| ... }` is never a record: a record never starts with `|`. A block
  written alone, not after a call, is error E001.
- The value of the block is its last line, as in a function. The rest of the
  chain applies to the result of the call: `xs.map do |x| x end.sum`.

```rythme
names = ["Ada", "Grace", "Alan"]
names.each do |name|
  print("Hello, #{name}!")
end
print(names.map { |n| n.size })
total = [1, 2, 3].reduce(0) do |sum, x|
  sum + x
end
print(total, [[1, 2], [3, 4]].map do |[a, b]| a * b end)
names.each_with_index do |name, i| print(i, name) end
```

```text
Hello, Ada!
Hello, Grace!
Hello, Alan!
[3, 5, 4]
6 [2, 12]
0 Ada
1 Grace
2 Alan
```

### 3.15 Calls without parentheses

As in Ruby, a **statement** may call a function without parentheses:
`puts "Hello"` is `puts("Hello")`, and `print x, y` is `print(x, y)`.

- The statement starts with a name (`even?` too), followed on the same line
  by the start of its first argument: a number, a text, a symbol, a name, a
  constructor, `true`, `false`, `not`, `if`, `match` or `fn`. The arguments
  are expressions separated by commas, up to the end of the line; a new line
  may follow a comma.
- Only a statement starts this way: a line at the top level, or in a
  function, a block or a test; or a **result**, which holds a whole
  expression the way a statement does: a branch of `if` or `unless`
  (`if ok then puts "yes" else puts "no"`), what a case of `match` gives
  (`"Dr " + name -> puts name`), and the body after the `=` of a function
  or a test (`fn greet(n) = puts "Hi #{n}"`, `fn(x) = puts x`,
  `test "sum" = assert_eq 1 + 1, 2`). The arguments of a result end where
  the expression would: at `else`, `end`, `}`, `)` or a new line (so at the
  next case of `match`). Anywhere else in an expression, the parentheses
  stay: `x = len(xs)`, `print(len(xs))`, `1 + f(x)`.
- A comma after the arguments belongs to the call, inside brackets too:
  `print(if ok then puts 1, 2 else 0)` gives `1, 2` to `puts`. Write
  `puts(1)` to end the call before the comma.
- A `do` block written after the arguments is given to the call itself, as
  its last argument: `each xs do |x| ... end`. A `{ |x| ... }` block belongs
  to the expression right before it, as everywhere.
- `(` keeps its meaning: `puts (1 + 2) * 3` is `puts(1 + 2) * 3`.
- `[` and `-` can be read in two ways. As in Ruby, a `[` after a space
  (`puts [1, 2]`), and a `-` after a space but right before a number or a
  name (`puts -1`), start the arguments, **unless the code gives this name a
  value**: with `=` (`x = 5`, `f = fn(v) = v`, `[a, b] = xs`), as a
  parameter of a function or a block, or in a pattern of `match` (the names
  before `=` or `->` on their line count). Then `x -1` is `x - 1` and
  `xs [0]` is `xs[0]`, as they always were, in a result too. Without a space before
  (`x-1`, `xs[0]`), or with a space after the `-` (`x - 1`), it is never a
  call. The value may be given anywhere in the file, even to a parameter of
  another function: when the name is a function where it is written
  (`show [1, 2]` with `fn show(xs)` and, elsewhere, `fn other(show)`), the
  checker warns (W011) and shows where the name gets its value; write the
  parentheses, `show([1, 2])`, to call it.
- `rythme fmt` keeps the choice of the author, with or without parentheses,
  and writes the other readings as what they are: `x - 1`, `xs[0]`.

```rythme
fn greet(name, greeting) = print("#{greeting}, #{name}!")
greet "Ada", "Hello"
puts "sum:", [1, 2, 3].sum
puts -1
puts [1, 2]
x = 10
xs = [7, 8]
puts x -1, xs [0]
[1, 2].each do |n| puts n * 10 end
each ["a", "b"] do |s|
  print s.upcase, s.size
end
```

```text
Hello, Ada!
sum: 6
-1
[1, 2]
9 7
10
20
A 1
B 1
```

In results:

```rythme
fn greet(name) = puts "Hello, #{name}!"
fn check(ok) = if ok then puts "yes" else puts "no"
fn title(name) =
  match name
    "Dr " + rest -> puts "doctor", rest
    _ -> puts name
  end
greet "Ada"
check 1 > 2
title "Dr Grace"
title "Alan"
[1, 2].each { |n| puts n * 10 }
show = fn(v) = puts "v:", v
show(3)
x = 10
puts(if x > 5 then x -1 else 0)
```

```text
Hello, Ada!
no
doctor Grace
Alan
10
20
v: 3
9
```

A name given a value with `=` is a value, even when it holds a function:

```rythme
double = fn(v) = v * 2
print double(4)
double -3
```

```text
error[E005]: cannot use `-` with Function and Int
 --> sample.ry:3:8
  |
3 | double -3
  |        ^ Function - Int 3
  |
  = help: `double -x` subtracts here; to call `double`, write `double(-x)` (a call without parentheses starts a statement or a result, such as a branch of `if` or the body after the `=` of a function, and its name is not given a value with `=` or as a parameter)
  = more about this error: `rythme explain E005`
```

---

## 4. Definitions

### 4.1 Names

`pattern = expression` gives names to a value, at the top level or in a
block. The left side is usually a name, but may be any pattern of `match`
(see [3.10](#310-match-and-patterns)) that takes the value apart: `_`, a list
pattern (`[a, b]`, `[first, ..rest]`, `[first, ..]`), a record pattern
(`{name, born: year}`, `{name, ..}`), a constructor pattern (`Some(v)`) or a
literal. If the value does not match, the program stops with E006 (found
before running when the kind of the value is certain). The value may start
on the line after `=`.

As in `match`, `{name, ..others}` is E001: the other fields of a record
cannot be given a name (write `{name, ..}`, or use `remove`). A guard is not
allowed (use `match`), and alternatives (`|`) only inside brackets, where
they cannot give names: `Some(1 | 2) = x` only checks the value.

```rythme
[a, b] = [1, 2]
{name, born: year} = {name: "Ada", born: 1815, city: "London"}
[head, ..tail] = 1..4
[top, ..] = ["x", "y", "z"]
{city, ..} = {name: "Ada", city: "London"}
Some(v) = Some(5)
print(a, b, name, year, head, tail, top, city, v)
[x, y] = [1, 2, 3]
```

```text
1 2 Ada 1815 1 [2, 3, 4] x London 5
error[E006]: the value List [1, 2, 3] does not match this pattern
 --> 40-bindings.ry:8:1
  |
8 | [x, y] = [1, 2, 3]
  | ^^^^^^
  |
  = help: use `match` to handle the different possible shapes of a value
  = more about this error: `rythme explain E006`
```

```rythme
ada = {name: "Ada", born: 1815}
{name, ..others} = ada
```

```text
error[E001]: a record pattern cannot give a name to the other fields
 --> 40b-rest.ry:2:10
  |
2 | {name, ..others} = ada
  |          ^^^^^^
  |
  = help: write `{name, ..}` to ignore the other fields; for a record without some fields, use `remove(record, "name")`
  = more about this error: `rythme explain E001`
```

### 4.2 Names never change

A name means one value for its whole scope: defining it again in the same
scope is error E011, found before the program runs. `x += 1` and similar
forms are errors too (E011). Give the new value a new name.

```rythme
total = 1
total = total + 1
```

```text
error[E011]: `total` is already defined
 --> 41-redefine.ry:2:1
  |
2 | total = total + 1
  | ^^^^^ defined again here
  |
1 | total = 1
  | ----- first defined here
  |
  = help: values never change in Rythme: give the new value a new name, like `total2 = total + 1`
  = more about this error: `rythme explain E011`
```

In the REPL, names can be defined again (see [7.2](#72-the-interactive-mode-repl)).

### 4.3 Scopes and shadowing

A scope is: the top level of a file; one definition of a function (its
parameters and the statements of its body share a single scope); each `do`
block; each `match` case; each anonymous function.

- Inside one scope, a name is defined only once (E011), and a pattern cannot
  bind the same name twice (E011).
- An inner scope **may reuse a name** of an outer scope, which then hides the
  outer one: a parameter may have the name of a top-level value, a `do` block
  may define a name that exists outside.
- A local name may hide a built-in function silently; a top-level definition
  that hides a built-in function gives warning W003.
- The names brought in by `use "file.ry"` count as top-level names:
  defining one of them again is E011. With `use geometry`, only the name
  `geometry` is taken (see 4.9).

```rythme
x = 1
fn times_ten(x) = x * 10     # a parameter may reuse an outer name
fn g(n)
  y = n + 1
  z = do
    y = 100                  # a nested block may reuse a name
    y + 1
  end
  [y, z]
end
print(x, times_ten(2), g(1))
```

```text
1 20 [2, 101]
```

### 4.4 The order of the top level

Before a file runs, all its `type` definitions and all its top-level `fn`
definitions are registered. A function or a constructor can therefore be used
anywhere in the file, even above its definition, and top-level functions can
call each other in any order.

Then the definitions `name = ...` and the expressions run **from top to
bottom**. A value can only be used after its definition: directly at the top
level, the checker reports it before running (E003); through a function
called too early, the error happens while running (E003, "not defined yet").

```rythme
print(double(21), Circle(1))     # functions and types are known in the whole file
fn double(x) = x * 2
type Shape = Circle(radius)
```

```text
42 Circle(1)
```

```rythme
print(limit)
limit = 10
```

```text
error[E003]: `limit` is used before being defined
 --> 48b-order.ry:1:7
  |
1 | print(limit)
  |       ^^^^^ used here
  |
2 | limit = 10
  | ----- defined later, here
  |
  = help: move the definition above this line
  = more about this error: `rythme explain E003`
```

```rythme
fn show_limit() = print(limit)
show_limit()
limit = 10
```

```text
error[E003]: `limit` is not defined yet
 --> 48c-order.ry:1:25
  |
1 | fn show_limit() = print(limit)
  |                         ^^^^^ used here
  |
3 | limit = 10
  | ----- defined later, here
  |
  = help: the file runs from top to bottom: move this definition above the line that uses it (directly or through a function)
  = call trace (most recent first):
      in `show_limit` called at 48c-order.ry:2:1
  = more about this error: `rythme explain E003`
```

The value of the last top-level expression is shown by the REPL and by
`rythme -e`, not when a file is run. Every other top-level expression must
have an effect (E024).

### 4.5 Functions

```text
fn name(parameters) = expression

fn name(parameters)
  statements
end

fn name(parameters) if guard = expression
```

- The parentheses are required, even without parameters: `fn hello() = ...`.
- Parameters are **patterns**: `fn head([x, ..]) = x`, `fn greet({name}) = ...`.
- In the block form, the body starts on the line after the parameters (or
  after the guard) and ends with `end`; its last statement gives the result.
- **Definition by cases.** Several definitions of the same function, written
  one right after the other (blank lines and comments may separate them),
  form a single function. They must have the same number of parameters (E004).
  At each call they are tried **in order**; the first one whose parameters
  all match, and whose guard (if any) is `true`, is used. If none matches,
  the program stops with E006. Definitions with the same name that are not
  consecutive are E011.
- A **guard** is evaluated after the parameters matched, with their names
  visible. If a `?` in the guard meets `None` or `Err(...)`, the whole call
  returns that value at once.
- Before running, the checker reports definitions placed after one that
  accepts every argument (W004), and, for functions of one parameter, the
  cases that no definition handles (W002).
- A function written inside a block is **local**: it is visible after its
  definition in that block and can call itself, but it only sees the names
  defined before it. Local functions written **one right after the other**
  (only blank lines and comments between them) form a group, and each
  function of the group sees all the others, in any order, as top-level
  functions do: they can call each other. A value defined between two local
  functions ends the group; to use it in both functions, define it before
  them.
- Calls with the wrong number of arguments are found before running when the
  function is known by its name (E004), otherwise while running.

```rythme
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

fn sign(n) if n < 0 = "negative"
fn sign(0) = "zero"
fn sign(_) = "positive"

fn greet({name}) = "Hello, {name}!"
fn hello() = "hello"

print(fib(10), sign(-5), sign(0), sign(9), greet({name: "Ada", age: 36}), hello())
```

```text
55 negative zero positive Hello, Ada! hello
```

```rythme
fn parity(n)
  fn even(0) = true
  fn even(k) = odd(k - 1)        # odd is written right after: even sees it
  fn odd(0) = false
  fn odd(k) = even(k - 1)
  if even(n) then "even" else "odd"
end
print(parity(10), parity(7))
```

```text
even odd
```

```rythme
fn name(1) = "one"
fn name(2) = "two"
print(name(3))
```

```text
warning[W002]: `name` does not handle every possible argument
 --> 43b-noclause.ry:1:4
  |
1 | fn name(1) = "one"
  |    ^^^^
  |
  = note: not handled: `_`
  = help: add a definition for it, like `fn name(_) = ...`

error[E006]: no definition of `name` accepts these arguments: name(3)
 --> 43b-noclause.ry:3:7
  |
3 | print(name(3))
  |       ^^^^^^^ called here
  |
1 | fn name(1) = "one"
  | ---------- does not match
  |
2 | fn name(2) = "two"
  | ---------- does not match
  |
  = help: add a definition that handles this case, for example one that ends with a plain name or `_`
  = more about this error: `rythme explain E006`
```

### 4.6 Recursion and tail calls

Rythme has no loops: repetition is written with recursion or with the list
functions (`map`, `filter`, `fold`, `each`...).

A call is a **tail call** when it is the very last thing a function does.
Tail calls do not use stack space: a function may call itself (or another
function) in tail position millions of times. A call is in tail position
when it is:

- the body of a function, or the last statement of its block;
- a branch of an `if` in tail position;
- the result of a `match` case in tail position (not its guard);
- the last statement of a `do` block in tail position;
- the right side of an `and` or `or` in tail position, when that right side is
  a call.

A pipe step `x |> f(a)` and a method call `x.f(a)` are calls. A call that is
an argument, an operand (`n * fact(n - 1)`), a condition, or followed by `?`
is not in tail position.

Other calls nest, and nesting is limited by the stack: beyond a limit, the
program stops with E014. The limit depends on the function; with the
`rythme` command it is several hundred thousand nested calls (about 900 000
for `total` below, more than 400 000 even for a function that does much more
in each call), and 600 in the web playground. The list functions (`map`,
`fold`...) do not nest.

```rythme
fn count_down(0, acc) = acc
fn count_down(n, acc) = count_down(n - 1, acc + 1)   # a tail call
fn is_even_n(0) = true
fn is_even_n(n) = is_odd_n(n - 1)                     # tail calls between two functions
fn is_odd_n(0) = false
fn is_odd_n(n) = is_even_n(n - 1)
print(count_down(1000000, 0), is_even_n(1000001))
fn total([]) = 0
fn total([x, ..rest]) = x + total(rest)               # not a tail call: + comes after
print(total(1..5000000))
```

```text
1000000 false
error[E014]: too many nested calls (917503 calls in progress)
 --> 44-tail.ry:9:29
  |
9 | fn total([x, ..rest]) = x + total(rest)               # not a tail call: + comes after
  |                             ^^^^^^^^^^^ this call goes too deep
  |
  = help: a recursive function that never stops? Check that it always reaches a base case.
      Otherwise, make the recursive call the very last thing the function does
      (tail call), for example with an accumulator:
        fn sum([], acc) = acc
        fn sum([x, ..rest], acc) = sum(rest, acc + x)
      or use `fold`, `map`, `filter`, which have no limit.
  = call trace (most recent first):
      in `total` (×40) called at 44-tail.ry:9:29
      … and 917463 more calls
  = more about this error: `rythme explain E014`
```

(The number of calls in progress varies with the function and the machine.)

### 4.7 Types

```text
type Name = Variant | Variant(field, field) | ...
```

- The type name and the variant names start with an uppercase letter; field
  names start with a lowercase letter and are distinct within a variant (E011).
- A `|` may come before the first variant, and the variants may be written on
  several lines that start with `|`.
- A variant **with fields** is a constructor: a function that takes exactly
  one argument per field (E004 otherwise). A variant **without fields** is a
  value (calling it is E009).
- Constructor names are global: two types cannot have a constructor with the
  same name, and a type cannot be defined twice (E011); `Option`, `Result`
  and their constructors cannot be defined again either. The type name itself
  is not a value; it appears in `type_of` and in messages.
- A type may contain itself (trees, lists...). `type` is only allowed at the
  top level.
- Fields are read with `.field` or with patterns. Equality and order: see
  [2.11](#211-equality) and [2.12](#212-order).
- `##` comments written above a `type` document it: `help` shows them for
  each of its constructors, after the name of the type.

```rythme
## A shape to draw.
type Shape =
  | Circle(radius)
  | Rect(width, height)
  | Empty

fn area(Circle(r)) = pi * r * r
fn area(Rect(w, h)) = w * h
fn area(Empty) = 0

shapes = [Circle(1.0), Rect(2, 3), Empty]
print(shapes |> map(area))
print(Rect(2, 3).width, type_of(Empty), Circle, map([1, 2], Circle))
help(Circle)
```

```text
warning[W009]: this shows a function, not a value
  --> sample.ry:13:41
   |
13 | print(Rect(2, 3).width, type_of(Empty), Circle, map([1, 2], Circle))
   |                                         ^^^^^^ a constructor, shown as `<constructor ...>`
   |
   = help: to show what it gives, call it: `Circle(...)`

[3.141592653589793, 6, 0]
2 Shape <constructor Circle> [Circle(1), Circle(2)]
Circle(radius)
  A constructor of the type `Shape`.
  A shape to draw.
```

(The checker warns that `Circle` alone is shown as a function: here, it is on
purpose.)

### 4.8 Tests

```text
test "name" = expression

test "name"
  statements
end
```

The name is a plain text. Tests are only allowed at the top level. They are
ignored when the program is run normally, and run by `rythme test file.ry`:

1. the file and the files it uses are loaded and checked;
2. the definitions `name = ...` run, but **top-level expressions are skipped**
   (in every file), so that a test run does not print or ask for input;
3. the tests run in order: those of the used files first, then those of the
   file.

A test **passes** if it ends without error and its value is not `false`,
`Err(...)` or `None`. `?` may be used in a test body: a `?` that meets `None`
or `Err(...)` ends the test with that value, which makes it fail. The exit
code is 0 when all tests pass (or when there are none) and 1 otherwise.

```rythme
fn double(x) = x * 2

test "double of 2 is 4" = assert_eq(double(2), 4)

test "several checks"
  assert_eq(double(0), 0)
  assert(double(-1) < 0, "should stay negative")
end

test "a parsed number"
  n = parse_int("21")?            # `?` is allowed in a test
  assert_eq(double(n), 42)
end

test "a wrong expectation" = assert_eq(double(3), 7)

print("not shown by rythme test")
```

`rythme test 46-tests.ry`:

```text
Running 4 test(s)
  ✓ double of 2 is 4
  ✓ several checks
  ✓ a parsed number
  ✗ a wrong expectation
error[E015]: assertion failed: the values are different
  --> 46-tests.ry:15:30
   |
15 | test "a wrong expectation" = assert_eq(double(3), 7)
   |                              ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: got:      6
       expected: 7
   = more about this error: `rythme explain E015`

4 test(s): 3 passed, 1 failed (1 ms)
```

Exit code: 1. The time at the end of the summary is the time of the whole
run; a test that takes half a second or more shows its own time too.

**Several files, a filter.** `rythme test` takes files and folders: for a
folder (`rythme test .`), it runs the tests of every `.ry` file inside it and
in its subfolders that has tests, in alphabetical order (hidden folders,
`target` and `node_modules` are skipped). Each file runs on its own, after a
line that names it. A file with errors is reported, and counted in the
summary. At the end, when several files ran or at least two tests failed,
the failed tests are listed with their place. The options may come anywhere
after `test`:

| Option | What it does |
|--------|--------------|
| `--filter TEXT` (`-f`) | runs only the tests whose name contains `TEXT`, ignoring case; may be given several times. When no test is selected, the exit code is 1 |
| `--seed N` | the seed of property tests (below); `--seed random` picks one and prints it |
| `--watch` (`-w`) | runs the tests, then again each time a file changes: a file tested, a file it uses, or a `.ry` file of a folder given. The files are looked at twice a second; Ctrl+C stops |
| `-- ARGS...` | the words after `--` are given to `args()` |

**Property tests.** `check_all(property, generator)` calls the function
`property` with 100 values made by `generator` (`check_all(property,
generator, cases)` with another number, from 1 to 100 000). The property
holds for a value when it gives `true`, `()`, `Some(...)` or `Ok(...)`; it
fails when it gives `false`, `None` or `Err(...)`, or stops with an error
(another value is E005). The first values are small, the next ones use more
and more of the range, and the edges (0, the minimum and the maximum) come
often. When a value makes the property fail, `check_all` **simplifies** it
(shrinking): a number goes toward 0 (or the end of its range closest to 0),
a text or a list gets shorter, then its elements simpler, as long as the
property still fails. It then stops with E015 (or with the error of the
property, its message starting with the value), showing the simplest value
found, the value it came from and the **seed**. A property that recurses too
deep stops at once with E014, without simplifying. The values depend only on
the seed and on the place of the call in its file, so every run, on every
machine, tries the same values; `rythme test --seed N` tries others. The
property may also be given last, as a block (see
[3.14](#314-blocks-given-to-calls)): `check_all(ints(0, 9)) { |x| x * 0 == 0 }`.

The generators are records, made by `ints(min, max)` (whole numbers, from
-1000 to 1000 without arguments), `floats(min, max)` (decimal numbers),
`texts(max_length)` (20 at most without argument: letters, digits, spaces,
punctuation, accents), `bools()`, `lists_of(generator, max_length)` (10 at
most without length), `one_of(list)` (one of the values; a failing value is
replaced by one written before it) and `records_of({field: generator,
...})`, to give several values. `sample(generator, n)` gives `n` values (10
without `n`) to see what a generator makes.

When `assert_eq` fails on lists or records, the message also says where the
values first differ (`[2].name`), what is missing or in excess; on two short
texts, a mark shows the first character that differs.

```rythme
fn my_max(list) = fold(list, 0, fn(best, x) = if x > best then x else best)

test "reversing twice gives the list back" = check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

test "the largest is in the list" = check_all(fn(l) = is_empty(l) or contains(l, my_max(l)), lists_of(ints()))

test "a record" = assert_eq({name: "Ada", langs: ["en", "fr"]}, {name: "Ada", langs: ["en", "de"]})
```

`rythme test 47-properties.ry`:

```text
Running 3 test(s)
  ✓ reversing twice gives the list back
  ✗ the largest is in the list
error[E015]: the property is false for [-1]
 --> 47-properties.ry:5:37
  |
5 | test "the largest is in the list" = check_all(fn(l) = is_empty(l) or contains(l, my_max(l)), lists_of(ints()))
  |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ false for [-1]
  |
  = note: found on try 1 of 100, then simplified from [-3, -1] in 3 step(s)
  = note: seed 42: every run tries the same values; `rythme test --seed N` tries others
  = help: the function must hold for every value of the generator: fix the function, or give the generator a smaller range, like `ints(1, 100)`
  = more about this error: `rythme explain E015`

  ✗ a record
error[E015]: assertion failed: the values are different
 --> 47-properties.ry:7:19
  |
7 | test "a record" = assert_eq({name: "Ada", langs: ["en", "fr"]}, {name: "Ada", langs: ["en", "de"]})
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: got:      {name: "Ada", langs: ["en", "fr"]}
      expected: {name: "Ada", langs: ["en", "de"]}
  = note: first difference at langs[1]: got "fr", expected "de"
  = more about this error: `rythme explain E015`

Failed tests:
  ✗ the largest is in the list (47-properties.ry:5:1)
  ✗ a record (47-properties.ry:7:1)
3 test(s): 1 passed, 2 failed (2 ms)
```

### 4.9 Modules: `use`

```text
use geometry               # the file geometry.ry, used through its name
use shapes/circle          # the file shapes/circle.ry, used as `circle`
use geometry as geo        # the same file, under another name
use "geometry.ry"          # older form: its names are used directly
```

- **The recommended form is `use name`**, without quotes and without `.ry`:
  its definitions are then reached through the name, so that every name
  says where it comes from. `rythme fmt` writes `use "geometry.ry" as geo`
  as `use geometry as geo`.
- `use` is only allowed at the top level. The path is relative to the
  **folder of the file that contains the `use`** (for `rythme -e` and the
  REPL: the current folder). Its parts are names separated by `/` on every
  system, and `..` goes up one folder: `use ../lib/tools`. A file whose
  name is not a Rythme name (`My Tools.ry`) is written as a text, with a
  name: `use "My Tools.ry" as tools`.
- The name is the name of the file (`circle` for `shapes/circle`), or the
  one written after `as`. Then `geometry.square_area(4)` calls a function,
  `geometry.origin` reads a value, `geometry.Square(5)` builds a value of a
  type of the file, `geometry.Square(s)` matches it in a `match`, and
  `x |> geometry.square_area` works as usual. The checker knows them all:
  a name that the file does not have is E003 (with the closest one), and
  the number of arguments is checked (E004).
- A file gives its top-level definitions (values, functions, constructors),
  and those of the files it uses with quotes. **A name starting with `_`
  is private to its file** (`fn _checked(x)`): other files cannot use it
  (E003). The name of the file is not a value (`geometry` alone is E003),
  and it cannot name anything else in the file (E011); two files cannot
  get the same name (E011).
- Each file has its own names: `geometry.area` and `circle.area` are two
  different functions, and a file may define `area` itself. A name used
  without its file, but defined by another file of the folder, gets a help
  that names that file and the `use` to add (the language server adds it).
- `use "path.ry"`, with quotes and without `as`, is the older form: the
  definitions of the file (except the private ones) become visible
  directly, with those of the files it uses that way; a name defined again,
  or defined by two such files, is E011, and the message names both files.
  `.ry` is added when the path has no extension.
- Each file is loaded, checked and run **once** per program, before the file
  that uses it, wherever the `use` is written in that file. Its top-level
  expressions run (a `print` in a used file prints once).
- A missing file is E017, with a suggestion when a close name exists (or the
  list of the Rythme files of the folder), and so is a folder; files that
  use each other in a circle are E017, and the message names both. Messages
  show the path of a used file as it is reached from the current folder.

```rythme
# geometry.ry
print("geometry.ry runs once")
fn rectangle_area(w, h) = w * h
fn square_area(side) = rectangle_area(side, _checked(side))
fn _checked(x) = if x >= 0 then x else fail("negative side")
origin = {x: 0, y: 0}
type Shape = Rectangle(w, h) | Square(side)
```

```rythme
# shapes/circle.ry
fn area(r) = pi * r ** 2
```

```rythme
# main.ry
use geometry
use shapes/circle
print(geometry.rectangle_area(2, 3), geometry.square_area(4), geometry.origin)
print(round(circle.area(1), 2))
fn area(shape) = match shape
  geometry.Rectangle(w, h) -> w * h
  geometry.Square(s) -> s * s
end
print(area(geometry.Square(5)))
```

`rythme main.ry`:

```text
geometry.ry runs once
6 16 {x: 0, y: 0}
3.14
25
```

A file next to `geometry.ry` that forgets the `use`, `rythme forgot.ry`:

```text
error[E003]: unknown name `rectangle_area`
 --> forgot.ry:1:7
  |
1 | print(rectangle_area(2, 3))
  |       ^^^^^^^^^^^^^^ not defined here
  |
  = help: `rectangle_area` is defined in `geometry.ry`: add `use geometry` at the top of the file, then write `geometry.rectangle_area`
  = more about this error: `rythme explain E003`
```

A private name, `rythme private.ry`:

```text
error[E003]: `_checked` is private to the file `geometry.ry`
 --> private.ry:2:7
  |
2 | print(geometry._checked(2))
  |       ^^^^^^^^^^^^^^^^^ private
  |
  = note: a name starting with `_` can only be used in its own file
  = help: to use it from other files, rename it without `_` in `geometry.ry`
  = more about this error: `rythme explain E003`
```

With `ping.ry` containing `use pong` and `pong.ry` containing `use ping`,
`rythme ping.ry` gives:

```text
error[E017]: `pong.ry` and `ping.ry` use each other
 --> pong.ry:1:1
  |
1 | use ping
  | ^^^^^^^^
  |
  = help: files cannot use each other: move what both need into a third file, used by both
  = more about this error: `rythme explain E017`
```

---

## 5. Errors

### 5.1 Errors are values

Rythme has no `null` and no exceptions. A value that may be missing is an
`Option` (`Some(value)` or `None`); an operation that may fail returns a
`Result` (`Ok(value)` or `Err(error)`). The library follows this rule:
`first`, `last`, `get`, `find`, `index_of` and `env` return Options;
`parse_int`, `parse_float`, `parse_json`, `options`, `input_lines` and every
function of files and folders (`read_file`, `write_file`, `append_file`,
`file_info`, `list_folder`, `create_folder`, `delete_file`) return Results.

They are handled with `match`, with `?`, or with the functions `unwrap_or`,
`unwrap`, `is_some`, `is_none`, `is_ok`, `is_err`, `map` (which applies a
function to the value inside `Some` or `Ok`), `and_then`, `ok_or` and
`map_err` (see [6](#6-standard-library)).

### 5.2 The `?` operator

`expression?` looks at the value of the expression:

- `Some(x)` or `Ok(x)`: the result is `x`;
- `None` or `Err(e)`: the current function (named or anonymous) **returns this
  value immediately**; in a test, the test ends with it;
- any other value: error E016.

`?` is a postfix operator of the highest precedence: `parse_int(t)? + 1`
adds 1 to the number. It can only be used inside a function or a test (E016,
found before running). In a guard, it makes the whole call return.

```rythme
fn add_texts(a, b)
  x = parse_int(a)?          # on Err(...), add_texts returns it at once
  y = parse_int(b)?
  Ok(x + y)
end
print(add_texts("1", "2"), add_texts("1", "two"))

fn first_even_doubled(list)
  x = find(list, is_even)?   # on None, the function returns None
  Some(x * 2)
end
print(first_even_doubled([1, 4, 5]), first_even_doubled([1, 3]))
```

```text
Ok(3) Err("\"two\" is not a whole number")
Some(8) None
```

### 5.3 Stopping the program

- `fail(message)` stops the program with error E021 and the message.
- `assert(condition, message)` and `assert_eq(actual, expected)` stop with
  E015 when the check fails; they are mostly used in tests.
- `unwrap(x)` stops with E013 on `None` or `Err(...)`.
- `exit(code)` stops immediately with this exit code (0 to 255, E023
  otherwise; 0 when omitted). The codes Rythme itself exits with are in
  [7.1](#71-the-command-line).

### 5.4 Errors while running

When an error happens while the program runs, Rythme stops the program,
writes a message on the **error output** and exits with code **1**. What the
program printed before stays printed. A message has:

1. a header: `error[CODE]: what happened`;
2. the place: `--> file:line:column`, then the line of code with the exact
   spot marked, and sometimes other related places;
3. notes and help lines (`= note:`, `= help:`), often with a suggestion;
4. for errors inside functions, a **call trace**, the most recent call first:
   repeated calls are grouped (`×40`), and tail calls do not appear, since they
   replace the function that made them;
5. the command to learn more: `rythme explain CODE`.

```rythme
fn average(list) = sum(list) / len(list)
fn report(lists) = map(lists, average)
print("start")
print(report([[1, 2], []]))
print("never shown")
```

```text
start
error[E008]: division by zero
 --> 51-runtime.ry:1:30
  |
1 | fn average(list) = sum(list) / len(list)
  |                              ^
  |
  = help: check the divisor first: `if b == 0 then ... else a / b`,
      or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`
  = call trace (most recent first):
      in `average` called at 51-runtime.ry:2:20
  = more about this error: `rythme explain E008`
```

Exit code: 1.

### 5.5 The check before running

Every program is **read and checked completely before it runs**: by
`rythme file.ry`, `rythme run`, `rythme test`, `rythme -e`, and for each
input of the REPL. `rythme check file.ry` does only this step, and runs
nothing; given several files (`rythme check a.ry b.ry`), it checks each one,
even after a file with errors, and exits with code 1 if any of them has an
error.

- A syntax error stops the reading at the first one.
- Then the checker reports **all** the problems it finds at once, errors
  first. If there is at least one error, nothing runs (exit code 1).
  Warnings are shown, and the program runs anyway.

The checker finds:

- unknown names and constructors, with the closest existing name or the
  Rythme name of a function from another language (E003);
- top-level values used before their definition (E003);
- wrong numbers of arguments to functions and constructors known by name,
  and constructor patterns with the wrong number of fields (E004);
- calls of things that are not functions: `None()`, `2(x + 1)` (E009);
- names, types, constructors and fields defined twice, and names bound twice
  in one pattern (E011);
- `?` outside a function (E016);
- lines whose value is lost (E024), `_` used as a value and alternatives that
  bind names (E001);
- unused local names (W001), forgotten cases (W002), built-in functions
  hidden by a top-level definition (W003), cases that can never be reached
  and empty ranges (W004), unknown functions after a dot (W005), ranges
  that overlap (W010).

The checker also knows the kinds of values when they are certain: the values
written in the code, and what operators, built-in functions and the
functions of the program surely give. It reports, before running, the
operations that would fail every time they run, with the code, the message
and the help of the error the program would stop with: a wrong kind of value
for an operator, a condition, a range, an index or a built-in function
(E005), a field that a record or a constructor does not have (E010), a
pattern that never matches the value (E006), `?` on a value that is neither
an Option nor a Result (E016), a value called as a function (E009), an
`assert_eq` that can never pass (E015), a function given to a built-in
function with the wrong number of parameters (E004). It also warns about
comparisons that are always false or always true (W006), tests that can never
fail (W007), results of functions without effect that are lost, like
`push(list, x)` alone on its line (W008), and functions shown instead of
their value, like `print(f)` (W009). When it is not sure, it says nothing:
the rest (values whose kind varies, indexes out of range, division by
zero...) is found while running.

```rythme
fn area(width, height) = width * heigth
fn main()
  total = area(2)
  print(totl)
end
x = 1
x = 2
```

`rythme check 52-check.ry`:

```text
error[E003]: unknown name `heigth`
 --> 52-check.ry:1:34
  |
1 | fn area(width, height) = width * heigth
  |                                  ^^^^^^ not defined
  |
  = help: did you mean `height`?
  = more about this error: `rythme explain E003`

error[E004]: `area` takes 2 arguments, but 1 was given
 --> 52-check.ry:3:11
  |
3 |   total = area(2)
  |           ^^^^^^^ 1 argument(s) here
  |
1 | fn area(width, height) = width * heigth
  |    ---- defined here
  |
  = help: usage: `area(width, height)`
  = more about this error: `rythme explain E004`

error[E003]: unknown name `totl`
 --> 52-check.ry:4:9
  |
4 |   print(totl)
  |         ^^^^ not defined
  |
  = help: did you mean `total`?
  = more about this error: `rythme explain E003`

error[E011]: `x` is already defined
 --> 52-check.ry:7:1
  |
7 | x = 2
  | ^ defined again here
  |
6 | x = 1
  | - first defined here
  |
  = help: values never change in Rythme: give the new value a new name, like `total2 = total + 1`
  = more about this error: `rythme explain E011`

warning[W001]: `total` is never used
 --> 52-check.ry:3:3
  |
3 |   total = area(2)
  |   ^^^^^
  |
  = help: remove it, or name it `_total` if this is on purpose

✗ 52-check.ry: 4 error(s)
```

Exit code: 1. And the five warnings:

```rythme
type Light = Red | Orange | Green
fn next(Red) = Green
fn next(Green) = Orange
fn label(n) = match n
  _ -> "any"
  0 -> "zero"
end
fn twice(x)
  unused = x * 2
  x + x
end
len = 3
print([1, 2].strlen())
```

`rythme check 53-warnings.ry`:

```text
warning[W002]: `next` does not handle every possible argument
 --> 53-warnings.ry:2:4
  |
2 | fn next(Red) = Green
  |    ^^^^
  |
  = note: not handled: `Orange`
  = help: add a definition for it, like `fn next(Orange) = ...`

warning[W004]: this case can never be reached
 --> 53-warnings.ry:6:3
  |
6 |   0 -> "zero"
  |   ^
  |
5 |   _ -> "any"
  |   - this case already handles everything
  |
  = help: cases are tried in order: put `_` last

warning[W001]: `unused` is never used
 --> 53-warnings.ry:9:3
  |
9 |   unused = x * 2
  |   ^^^^^^
  |
  = help: remove it, or name it `_unused` if this is on purpose

warning[W003]: `len` hides the built-in function `len`
  --> 53-warnings.ry:12:1
   |
12 | len = 3
   | ^^^
   |
   = help: in this file, `len` will now mean your definition; choose another name to keep the built-in one

warning[W005]: there is no function named `strlen`
  --> 53-warnings.ry:13:14
   |
13 | print([1, 2].strlen())
   |              ^^^^^^ unknown
   |
   = note: `value.strlen(...)` calls `strlen(value, ...)`, or the field `strlen` of a record
   = help: in Rythme, this is called `len`

✓ 53-warnings.ry: no errors, 5 warning(s)
```

Exit code: 0.

The checker also infers what the **parameters of each function** must be,
from how its code uses them: `side * side` needs numbers, `upper(t)` a
text, `p.name` a record (or a value whose constructor has a field `name`),
`if c` a Bool, a `match` the kinds of its patterns, a call to another
function what that one needs. It follows every way through the function:
the branches of an `if` or of a `match`, and the definitions of a function
(one per case), add up their kinds; one operation after the other narrows
them down; a `?`, or a call that may end the program (`exit`), keeps what is
known at that point. A call with an argument of another kind fails every
time it runs: the checker reports it, with the code, the message and the
help of the error the function would stop with, where it stops in the
function, and what the function needs. Nothing is reported when the call
may work: a function that works on several kinds of values (`twice` below,
on numbers, texts and lists), definitions chosen by their patterns or their
guards, recursion, functions never called. What the functions of another
file need is not inferred.

```rythme
fn area(side) = side * side
fn shout(t) = upper(t) + "!"
fn twice(x) = x + x
print(area(3), shout("hi"), twice("ab"))
print(area("big"))
print(shout(5))
```

`rythme check 54-params.ry`:

```text
error[E005]: cannot use `*` with Text and Text
 --> 54-params.ry:5:7
  |
5 | print(area("big"))
  |       ^^^^^^^^^^^ `area` stops with this error, every time
  |
1 | fn area(side) = side * side
  |                      - in `area`: Text "big" * Text "big"
  |
  = note: from how `area` uses `side`, it needs: `fn area(side: Int|Float) -> Int|Float`
  = more about this error: `rythme explain E005`

error[E005]: `upper` expects a text as argument 1, but got Int 5
 --> 54-params.ry:6:7
  |
6 | print(shout(5))
  |       ^^^^^^^^ `shout` stops with this error, every time
  |
2 | fn shout(t) = upper(t) + "!"
  |                     - in `shout`: wrong kind of value
  |
  = note: from how `shout` uses `t`, it needs: `fn shout(t: Text) -> Text`
  = help: usage: `upper(text)`
  = more about this error: `rythme explain E005`

✗ 54-params.ry: 2 error(s)
```

In an editor, the hover of a function shows what the checker infers, as a
signature: `fn area(side: Int|Float) -> Int|Float`, the kinds each
parameter must be (separated by `|`) and what a call gives back, when they
are known (see [7.5](#75-editors-and-the-language-server)).

### 5.6 Error and warning codes

Every error and warning has a code. `rythme explain CODE` (or
`rythme help CODE`, or `:explain CODE` in the REPL) gives a long explanation
with examples; the code may be written in lowercase.

| Code | Name | When |
|------|------|------|
| E001 | Syntax error | the code does not follow the grammar: a missing bracket or `then`, braces used for a block, a keyword from another language, chained comparisons, `_` used as a value, `{name, ..others}`, an expression too long or nested too deeply... |
| E002 | Text never closed | a `"` or a `{` inside a text is never closed |
| E003 | Unknown name | a name or constructor that is not defined at this place, a value used before its definition, or a private name (`_name`) of another file |
| E004 | Wrong number of arguments | a call or a constructor pattern with too many or too few values; definitions of one function with different numbers of parameters |
| E005 | Wrong kind of value | an operation or a function received a kind of value it does not accept (`"a" + 1`, a condition that is not a Bool, values that cannot be compared...) |
| E006 | No case matches | no `match` case, no definition of a function, or no definition pattern accepts the value |
| E007 | Index out of range | `list[i]` or `text[i]` outside the valid positions |
| E008 | Division by zero | `/`, `//` or `%` by zero, or `0 ** -n` |
| E009 | Not a function | a value that is not a function is called (`x(3)`, `None()`) |
| E010 | Unknown field | the record or the variant has no field with this name |
| E011 | Already defined | a name, type, constructor or field defined twice in the same place, or an attempt to change a value (`x += 1`) |
| E012 | Number too large | no longer given: whole numbers have no size limit (a number of more than 100000 digits is E023) |
| E013 | `unwrap` found no value | `unwrap` on `None` or `Err(...)` |
| E014 | Too many nested calls | calls nested deeper than the stack allows |
| E015 | Assertion failed | `assert`, `assert_eq`, or a test that ended with `false`, `Err(...)` or `None` |
| E016 | Wrong use of `?` | `?` outside a function, or on a value that is not an Option or a Result |
| E017 | File problem | a file that cannot be read or is not found, a folder given instead of a file, files that use each other in a circle, or `input()` at the end of the input |
| E018 | Unexpected character | a character or a number with no meaning here (`'`, `&`, `2x`, `...`, `++`, an unknown escape sequence, a lookalike such as `×` or an invisible character copied from a document) |
| E019 | `if` without `else` | an `if` with no `else` |
| E020 | Missing `end` | a function, `test`, `match` or `do` block that is never closed |
| E021 | Program stopped by `fail` | `fail(message)` was called |
| E022 | Impossible conversion | `int(t)` or `float(t)` on a text that is not a number |
| E023 | Invalid value | a value of the right kind but outside what is accepted (a negative count, `min` of an empty list, `sqrt(-1)`, a range, a list or a text too large, a whole number of more than 100000 digits, an exit code outside 0–255...) |
| E024 | A value is lost | a line computes a value that nothing uses |
| E025 | Project or package problem | a `rythme.toml` or `rythme.lock` that cannot be read, a dependency not installed, or `rythme install` that cannot get one (see [7.7](#77-projects-and-packages)) |
| E026 | Invalid regex | a regex that breaks a rule (a `(` never closed, `*` with nothing to repeat, an unknown `\q`...), or a replacement of `replace_all` with a group that does not exist |
| W001 | Unused name | a local name is defined but never used |
| W002 | Forgotten case | a `match` or a one-parameter function does not handle every possible value |
| W003 | Built-in function hidden | a top-level definition takes the name of a built-in function |
| W004 | Case never reached | a case or definition comes after one that accepts everything or the same values (or after ranges that together hold all its numbers), can never match the value, or is an empty range (`9..1`) |
| W005 | Unknown function after a dot | `value.name(...)` where no function and no record field has this name |
| W006 | Comparison always false | `==` or `!=` between values of kinds that are never equal (`count == "3"`) |
| W007 | Test that can never fail | a test that only computes or shows a value, and never compares it with what is expected |
| W008 | Result lost | a line that only calls a function without effect, like `push(list, x)`: values never change |
| W009 | Function shown | a function shown in a text or printed (`<fn name>`) instead of what it gives |
| W010 | Ranges that overlap | a range pattern shares numbers with a range of a case or definition before it: they always go to the first one |
| W011 | Index or subtraction instead of a call | `show [1, 2]` or `show -1` starts a statement where `show` is a function, but the file gives `show` a value elsewhere: it is an index or a subtraction ([3.15](#315-calls-without-parentheses)) |

---

## 6. Standard library

The 174 built-in functions are always available, without `use`. Like the
keywords, their names are in English. `rythme help builtins` lists all of
them by category, one line each; `rythme help NAME` (or `help(NAME)` in a
program, or `:doc NAME` in the REPL) shows the documentation of one, with
examples whose values are checked by the tests of Rythme. This section gives
the conventions and what each category holds; `rythme help NAME` is the
reference for the details of each function.

**Conventions.**

- The data comes first: `map(list, f)`, `split(text, separator)`,
  `put(record, name, value)`. This is what makes `|>` and method-style calls
  work: `list |> map(f)`, `list.map(f)`.
- Functions never change their arguments; they return new values.
- A function that may find nothing returns an Option; a function that may
  fail for reasons outside the program (a text typed by a user, a file)
  returns a Result. Other functions stop the program on invalid input: E005
  for a wrong kind of value, E022 for an impossible conversion, E023 for a
  value outside what is accepted.
- Wherever a function is expected, any function works: a named function, an
  anonymous one, a built-in or a constructor (`map([1, 2], Some)`). Functions
  that test elements (`filter`, `any`, `all`, `find`, `count`, `partition`)
  need a function that returns a Bool (E005).
- The texts of `Err(...)` values made by built-ins (for example by
  `parse_int`) are written in the language of the messages.
- Mistakes get a hint: the name of a function of another language gives its
  Rythme name (`ord` → `char_code`, `padStart` → `pad_left`, `takeWhile` →
  `take_while`, `stringify` → `to_json`...), and a value of a neighbouring
  kind points to the right function (a text given where a list is expected:
  `chars`; a record: `entries`, `keys` or `values`; a list given to `put`,
  `remove` or `has`: `set_at`, `remove_at` or `contains`; an Option given to
  `flat_map`: `and_then`). When the kinds of the values are certain, these
  mistakes are found before running (see [5.5](#55-the-check-before-running)).

**Basics.** `print(values...)` and `debug(value)` (see [2.13](#213-how-values-are-shown)),
`text(value)` (display form as a text) and `text(number, decimals)` (a number
with exactly this many decimals, at most 100: `text(5, 2)` is `"5.00"`),
`input(prompt)` (reads a line from the keyboard; at the end of the input,
error E017), `input_lines()` (every line of the standard input until its end,
in a Result: for `rythme count.ry < names.txt`), `type_of(value)`, `help(value)`, `assert(condition, message)`,
`assert_eq(actual, expected)`, `fail(message)`, `args()` (the words given
after the file name on the command line), `options()` (the words of
`args()` read as options, in a Result: see below), `env(name)` (an environment
variable, as an Option), `exit(code)`, `time()` (seconds since 1970, a
Float), `random()` (a Float in [0, 1)), `random(a, b)` (an Int from a to b,
included), `random(list)` (an element).

**Numbers.** `abs`, `min` and `max` (several values, or one list), `sqrt`,
`pow(x, n)` (same as `**`), `floor`, `ceil` and `round(x)` (give an Int),
`round(x, digits)` (gives a Float), `int(x)` (cuts the decimals; reads a text;
`true` is 1), `float(x)`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan(x)` and
`atan(y, x)` (the angle of the point (x, y), from −π to π), `log(x)` and
`log(x, base)`, `exp`, `is_even`, `is_odd`, `gcd` and `lcm` (of two whole
numbers), and the constant `pi`.

**Text.** `len`, `upper`, `lower`, `capitalize` (the first character in
uppercase, or in titlecase where Unicode has one: `ǆ` gives `ǅ`, `ß` gives
`Ss`), `trim`, `pad_left(x, width, fill)` and `pad_right` (to line up
columns; `fill` is a space by default), `split(text)` (at white space) and
`split(text, separator)` (an empty separator cuts into characters),
`join(list, separator)`, `contains`, `starts_with`, `ends_with`,
`replace(text, old, new)`, `chars`, `lines`, `repeat(x, n)`, `char_code(c)`
and `char(code)` (a character and its Unicode number), `is_digit`,
`is_letter` and `is_space` (is the text, not empty, made only of such
characters?), `parse_int(text)` and `parse_float(text)` (Results; spaces
around the number and `_` are accepted), `parse_json(text)` (a Result: JSON
objects become records, arrays lists, `null` `None`; an error gives the line
and the column) and `to_json(value, indent)` (the JSON text of a value, on
one line, or one element per line shifted by `indent` spaces, at most 16),
`trim_start` and `trim_end` (spaces and line breaks of one side only),
`words(text)` (letters and digits, with `'`, `’` or `-` inside a word:
`"it's"`, `"well-known"`), `equals_ignore_case(a, b)` (uppercase and
lowercase count as equal, `ß` as `SS`), and
`format_number(x, decimals, language)` (thousands separated, `decimals`
decimals if given, at most 100, rounded like `round`; `language` is `"en"`
by default: `1,234.5`; `"fr"`: `1 234,5`, with a plain space; `"de"`,
`"es"`, `"it"`, `"nl"`, `"pt"`: `1.234,5`; `"fr-CA"` counts as `"fr"`).

**Regexes.** A regex (regular expression) is a text that describes pieces
of text. `matches(text, regex)` (does the *whole* text match?),
`find_first(text, regex)` (an Option), `find_all(text, regex)` (every
match, left to right, never overlapping; an empty match right after the
previous one is skipped), `captures(text, regex)` (an Option of a list: the
whole match, then the text of each group, `""` for a group that took no
part), `captures_all`, `named_captures(text, regex)` (an Option of a record,
one field per group `(?<name>...)`; E023 without named groups),
`split_by(text, regex)` (an empty match never cuts) and
`replace_all(text, regex, new)` (`new` is a text where `$1`, `$<name>`, `$0`
and `$$` stand for a group, a named group, the whole match and a `$`, or a
function given each match, which must return a text). A regex is made of
characters, `.` (any character but `\n`), classes `[abc]`, `[a-z]`,
`[^...]` (a `]` right after `[` or `[^` and a `-` at an end are themselves),
`\d` (0 to 9), `\w` (a letter, accents included, a digit or `_`), `\s` (a
space, a tab or a line break), their opposites `\D`, `\W`, `\S`, `\b` and
`\B` (at the edge of a word, or not), `^` and `$` (the start and the end of
the text), `\n`, `\t`, `\r`, a backslash before a sign for the sign itself
(`\.`), alternatives `a|b`, groups `(...)`, `(?:...)` (without capture) and
`(?<name>...)` (or `(?P<name>...)`), the repetitions `*`, `+`, `?`, `{n}`,
`{n,}` and `{n,m}` (n and m up to 1000), which take as much as they can,
or as little with a `?` after them (`*?`), and `(?i)` at the start, to
ignore the case. The usual rules choose between several matches: the
leftmost, then the first alternative, then the longest repetition. Searching
takes a time proportional to the length of the text times the size of the
regex, whatever they are: the engine follows every way at once instead of
trying one after the other (no "catastrophic backtracking"), and the
searches of `find_all`, `split_by` and `replace_all` skip the ways that an
earlier one showed to lead nowhere, so that finding every match usually
reads the text about once; for a
repetition of a part that can match an empty text, like `(a*)*`, it may
choose another end than other engines. In a Rythme text, a backslash is
written twice and braces `\{` and `\}`: `"\\d\{4\}"` is the regex
`\d{4}`; a sign in brackets needs no backslash: `"[.]"`, `"[(]"`. A
mistake in a regex stops the program with E026, which points to it in the
regex; when the regex is written in the call, it is found before running,
at its place in the code.

**Lists.** `map`, `filter`, `reject(list, f)` (the elements for which f is
false), `fold(list, start, f)`, `scan(list, start, f)` (every intermediate
result of `fold`), `each`, `each_with_index(list, f)` (calls `f(x, i)`),
`sum` (and `sum(list, f)`, the sum of `f(x)`), `product`, `first` and `last`
(an Option; `first(list, n)` and `last(list, n)` give the first or last n
elements), `rest`, `get(x, key)`, `push(list, x)`, `set_at(list, i, x)`,
`insert_at(list, i, x)` and `remove_at(list, i)` (negative positions count
from the end), `reverse`, `sort`, `sort_by(list, key)` (stable), `unique`,
`shuffle`, `slice(x, start, end)`, `take`, `drop`, `take_while(x, f)` and
`drop_while(x, f)`, `step_by(x, n)` (one element out of n), `chunks(x, n)`
(pieces of n elements) and `windows(x, n)` (every group of n neighbouring
elements), `zip`, `enumerate`, `any`, `all`, `find`, `count` (and
`count(list)`, the number of elements), `index_of`,
`min_by(list, key)` and `max_by(list, key)`, `flatten`, `flat_map`,
`partition`, `group_by(list, key)` (a record whose field names are the keys
as texts), `is_empty`. Many of them also accept texts (`first`, `last`,
`rest`, `get`, `reverse`, `slice`, `take`, `drop`, `take_while`,
`drop_while`, `step_by`, `chunks`, `windows`, `index_of`), records (`len`,
`contains`, `get`, `is_empty`) or Options and Results (`map`).

**Records.** `keys`, `values`, `entries` (a list of `[name, value]`) and
`from_entries` (the opposite), `has(record, name)`,
`put(record, name, value)`, `remove(record, name)`, `map_values(record, f)`.

**Dictionaries and sets.** `dict()`, `dict(pairs)` and `dict(record)`,
`set()` and `set(list)` build them (see [2.15](#215-dict-and-set)). The
functions of lists and records work on them too: `len`, `is_empty`,
`get(dict, key)` (an Option), `has(x, key)` and `contains(x, key)` (a key of
a dict, an element of a set), `put(dict, key, value)`, `push(set, x)`,
`remove(x, key)` (a key of a dict, an element of a set), `keys(dict)`,
`values(x)` (the values of a dict, or the elements of a set, as a list),
`entries(dict)` (a list of `[key, value]`), `map_values(dict, f)`,
`filter(dict, f)` and `reject(dict, f)` (f receives each key and its
value), `filter(set, f)`, `reject(set, f)`, `each(dict, f)` (f receives each
key and its value), `each(set, f)`, `map(set, f)` (the set of the
results). Then `union(a, b)` (two sets, or two
dicts: for a key in both, the value of `b` wins), `intersection(a, b)` and
`difference(a, b)` (for dicts, the pairs of `a` whose key is, or is not, in
`b`) and `is_subset(a, b)` (two sets). `to_json` writes a dict whose keys are
all texts, or all symbols, as a JSON object, and a set as an array; `dict(parse_json(text)?)`
reads an object as a dict. The other list functions do not take them:
`values(x)` or `entries(x)` give a list first.

```rythme
a = set([1, 2, 3])
b = set([2, 3, 4])
print(union(a, b), intersection(a, b), difference(a, b), is_subset(set([2]), a))
print(push(a, 0), remove(a, 2), has(a, 2), values(a), map(a, fn(x) = x % 2))
prices = dict([["apple", 3], ["pear", 5], ["fig", 9]])
print(filter(prices, fn(name, price) = price < 6), map_values(prices, fn(p) = p * 2))
print(union(prices, dict([["fig", 1], ["kiwi", 4]])))
counts = fold(split("a b a c b a"), dict(), fn(d, w) = put(d, w, unwrap_or(get(d, w), 0) + 1))
print(counts, entries(counts))
print(to_json(counts), to_json(set([1, 2])), dict(unwrap(parse_json("\{\"x\": 1}"))))
```

```text
set(1, 2, 3, 4) set(2, 3) set(1) true
set(0, 1, 2, 3) set(1, 3) true [1, 2, 3] set(0, 1)
dict("apple" => 3, "pear" => 5) dict("apple" => 6, "fig" => 18, "pear" => 10)
dict("apple" => 3, "fig" => 1, "kiwi" => 4, "pear" => 5)
dict("a" => 3, "b" => 2, "c" => 1) [["a", 3], ["b", 2], ["c", 1]]
{"a": 3, "b": 2, "c": 1} [1, 2] dict("x" => 1)
```

**Option and Result.** `unwrap`, `unwrap_or(x, default)`, `is_some`,
`is_none`, `is_ok`, `is_err`, `and_then(x, f)` (one more step that may fail:
`f` gives an Option or a Result), `ok_or(option, error)` (an Option made a
Result, ready for `?`), `map_err(result, f)`, and the constructors `Some`,
`None`, `Ok`, `Err`.

**Files.** `read_file(path)` (`Ok(text)` or `Err(message)`),
`write_file(path, value)` (writes the display form, replacing the content:
`Ok(())` or `Err(message)`), `append_file(path, value)` (adds at the end,
creating the file if needed), `file_exists(path)` (a Bool, for a file or a
folder), `file_info(path)` (`Ok({is_folder, size})`, the size in bytes, 0 for
a folder), `list_folder(path)` (the names inside a folder, in alphabetical
order), `create_folder(path)` (with the missing folders around it; `Ok(())`
also when it already exists), `delete_file(path)` (one file, never a folder).
Relative paths are relative to the current folder. Every error is an
`Err(message)` that names the path: `"file not found: notes.txt"`,
`"folder not found: photos"`, `"permission denied: ..."`, a file given
instead of a folder or the reverse, an empty path.

`join_path(parts)` joins a list of texts with `/`, which Linux, macOS and
Windows all understand (with `\` when the parts use only `\`, as Windows
paths do); a separator at the end of a part is not repeated, and empty parts
are skipped. Only the first part may start at a root (`/`, `\`, or a drive
like `C:`): a part like `"/etc"` after another one is E023, so that a path
never leaves its folder by surprise. `split_path(path)` cuts a path at each
`/` or `\`, whatever the system; the root of an absolute path (`/`, `C:\`,
`\\` for a network path) is its first part, and `join_path` puts the parts
back together.

`options()` reads the words of `args()` (or `options(words)`, the list
given) into a record, in a Result. `--name=value` gives the field `name`
with the text `value`; `--name` and `-n` give `true`; a `-` in a name
becomes `_` (`--dry-run` is `dry_run`); the other words, in order, are in
the field `args`, and so are all the words after `--`. A negative number
like `-5` is a word. An option given twice, a name that does not start with
a letter, a word like `-abc` (several letters after one `-`) and `--args`
give `Err(message)`. An option that may be missing is read with
`get(opts, "name")` (an Option) or tested with `has(opts, "name")`.

In the playground, every function of files and folders and `input_lines`
give `Err("`read_file` is not available in the browser: ...")`, and
`file_exists` is always `false`.

**Dates and times.** A date is a record `{year, month, day}`, with `hour`,
`minute` and `second` for a moment of the day (a missing one is 0), and
`offset`, the number of minutes the time is ahead of UTC, when it is known.
Years go from 1 to 9999, in the Gregorian calendar (leap years every 4
years, except every 100 years, except every 400 years). `now()` (the local
date and time with its `offset`: from the time zone of the system on Linux
and macOS, of the browser in the playground, 0 (UTC) elsewhere), `today()`
(`{year, month, day}`), `parse_date(text)` (a Result; ISO 8601:
`"2024-02-29"`, then optionally `T` or a space and a time `13:05` or
`13:05:00`, a fraction of a second, ignored, and `Z` or an offset `+01:00`,
`+0100` or `+01`), `format_date(date)` (back to ISO 8601, with the time and
the offset when the date has them), `is_date(value)`, `add_days(date, n)`
and `add_seconds(date, n)` (the time of day and the offset stay;
`add_seconds` always gives a time), `days_between(a, b)` (whole days from a
to b, rounded toward zero) and `seconds_between(a, b)` (offsets count),
`day_of_week(date)` (1 for Monday to 7 for Sunday, as in ISO 8601),
`days_in_month(year, month)`, `is_leap_year(year)`. A record that is not a
date (an unknown or missing field, a number out of range, a 30 February)
stops these functions with E023, which says what is wrong. An offset is a
fixed number: there are no daylight saving rules, so one day after a moment
is the same time on the next day.

**Property tests.** `check_all(property, generator, cases)`, `sample(generator, n)`,
and the generators `ints(min, max)`, `floats(min, max)`, `texts(max_length)`,
`bools()`, `lists_of(generator, max_length)`, `one_of(list)` and
`records_of(fields)` (see [4.8](#48-tests)). A generator is a record like
`{generator: "ints", min: 1, max: 6}`; one that is not well made stops
`check_all` and `sample` with E023.

**Ruby names.** Some functions have a second name, the one Ruby gives them.
Each one is exactly the function it names (same arguments, same messages):
`rythme help upcase` says so, and `rythme help builtins` lists them by
category. A program may give one of these names to something of its own
without a warning.

| Ruby name | Same as | Ruby name | Same as |
|-----------|---------|-----------|---------|
| `puts` | `print` | `select` | `filter` |
| `to_s` | `text` | `reduce`, `inject` | `fold` |
| `to_i` | `int` | `uniq` | `unique` |
| `to_f` | `float` | `each_slice` | `chunks` |
| `length`, `size` | `len` | `each_cons` | `windows` |
| `upcase` | `upper` | `include?` | `contains` |
| `downcase` | `lower` | `empty?` | `is_empty` |
| `strip` | `trim` | `even?`, `odd?` | `is_even`, `is_odd` |
| `start_with?` | `starts_with` | `any?`, `all?` | `any`, `all` |
| `end_with?` | `ends_with` | `key?` | `has` |
| `lstrip`, `rstrip` | `trim_start`, `trim_end` | `casecmp?` | `equals_ignore_case` |

`sort_by`, `min_by`, `max_by`, `group_by`, `zip`, `flatten`, `sum`, `count`,
`first`, `last`, `find`, `map`, `each`, `reject` and `each_with_index` have the
same name in both languages. With method calls and blocks
([3.8](#38-fields-and-method-calls), [3.14](#314-blocks-given-to-calls)),
Ruby code often works as it is:

```rythme
fruits = ["pear", "fig", "banana", "fig"]
print(fruits.uniq.size, fruits.select { |w| w.size > 3 }, fruits.reject { |w| w.include?("a") })
print(fruits.first(2), fruits.last(1), fruits.count, fruits.sum { |w| w.length })
print(fruits.each_slice(3), "  Rythme ".strip.downcase, 42.to_s + "!", "7".to_i + 1)
print([1, 2, 3].reduce(0) do |acc, x| acc + x end, [3, 1, 2].sort_by { |x| -x })
```

```text
3 ["pear", "banana"] ["fig", "fig"]
["pear", "fig"] ["fig"] 4 16
[["pear", "fig", "banana"], ["fig"]] rythme 42! 8
6 [3, 2, 1]
```

```rythme
numbers = [3, 1, 4, 1, 5, 9, 2, 6]
print(map(numbers, fn(x) = x * 2))
print(filter(numbers, is_even), fold(numbers, 0, fn(acc, x) = acc + x), sum(numbers))
print(sort(numbers), unique(numbers), sort_by(["pear", "fig", "banana"], len))
print(first(numbers), last([]), get(numbers, -1), take(numbers, 3), drop(numbers, 6))
print(find(numbers, fn(x) = x > 4), any(numbers, fn(x) = x > 8), count(numbers, is_odd))
print(zip([1, 2], ["a", "b"]), enumerate(["x", "y"]), group_by(["ant", "bee", "ape"], fn(w) = w[0]))
print(split("a,b,c", ","), join(["a", "b"], "-"), upper("été"), replace("2024", "4", "5"))
print(len("été"), chars("hé"), trim("  hi  "), contains("hello", "ell"), starts_with("rythme", "ry"))
print(parse_int("42"), parse_int("4x"), int("42"), float("2.5"), text(3.0))
print(abs(-3), min(3, 1, 2), max([4, 8]), round(3.14159, 2), floor(2.7), sqrt(16), 2 |> pow(10))
print(keys({a: 1, b: 2}), put({a: 1}, "b", 2), remove({a: 1, b: 2}, "a"), has({a: 1}, "a"))
print(unwrap_or(get([], 0), "default"), map(Some(2), fn(x) = x + 1), is_ok(Err("no")))
```

```text
[6, 2, 8, 2, 10, 18, 4, 12]
[4, 2, 6] 31 31
[1, 1, 2, 3, 4, 5, 6, 9] [3, 1, 4, 5, 9, 2, 6] ["fig", "pear", "banana"]
Some(3) None Some(6) [3, 1, 4] [2, 6]
Some(5) true 5
[[1, "a"], [2, "b"]] [[0, "x"], [1, "y"]] {a: ["ant", "ape"], b: ["bee"]}
["a", "b", "c"] a-b ÉTÉ 2025
3 ["h", "é"] hi true true
Ok(42) Err("\"4x\" is not a whole number") 42 2.5 3.0
3 1 8 3.14 2 4.0 1024
["a", "b"] {a: 1, b: 2} {b: 2} true
default Some(3) false
```

```rythme
print(gcd(12, 18), lcm(4, 6), atan(1, -1), text(3.14159, 2), text(5, 2))
print(capitalize("élise"), pad_left("7", 3, "0"), pad_right("ab", 4, "."), char_code("A"), char(66))
print(is_digit("42"), is_letter("été"), is_space(" \n"), take_while("42 cm", is_digit))
print(scan([1, 2, 3], 0, fn(total, x) = total + x), set_at([1, 2, 3], 0, 9), insert_at([1, 2], 1, 7), remove_at([1, 2, 3], -1))
print(drop_while([1, 3, 4, 5], is_odd), 0..10 |> step_by(5), chunks([1, 2, 3, 4, 5], 2), windows([1, 2, 3], 2))
people = [{name: "Ada", age: 36}, {name: "Alan", age: 41}]
print(min_by(["ccc", "a", "bb"], len), max_by(people, fn(p) = p.age).name)
print(from_entries([["a", 1], ["b", 2]]), map_values({a: 1, b: 2}, fn(x) = x * 10))
print(parse_int("42") |> and_then(fn(n) = if n > 0 then Ok(n) else Err("not positive")))
print(ok_or(None, "missing"), map_err(Err("not found"), fn(e) = "config: {e}"))
print(parse_json("[1, 2.5, null, \{\"ok\": true\}]"), to_json({name: "Ada", tags: ["x"], age: None}))
```

```text
6 12 2.356194490192345 3.14 5.00
Élise 007 ab.. 65 B
true true true 42
[1, 3, 6] [9, 2, 3] [1, 7, 2] [1, 2]
[4, 5] [0, 5, 10] [[1, 2], [3, 4], [5]] [[1, 2], [2, 3]]
a Alan
{a: 1, b: 2} {a: 10, b: 20}
Ok(42)
Err("missing") Err("config: not found")
Ok([1, 2.5, None, {ok: true}]) {"name": "Ada", "tags": ["x"], "age": null}
```

A name from another language and a value of the wrong kind get a hint, here
before the program runs:

```rythme
word = "hello"
print(ord(word[0]))
print(map(word, upper))
```

```text
error[E003]: unknown name `ord`
 --> 62-hints.ry:2:7
  |
2 | print(ord(word[0]))
  |       ^^^ not defined
  |
  = help: in Rythme, this is called `char_code`
  = more about this error: `rythme explain E003`

error[E005]: `map` expects a list, a set, an Option or a Result as argument 1, but got a value of type Text
 --> 62-hints.ry:3:11
  |
3 | print(map(word, upper))
  |           ^^^^ wrong kind of value
  |
  = help: usage: `map(list, f)`
  = help: a text is not a list: `chars(text)` gives the list of its characters
  = more about this error: `rythme explain E005`
```

```rythme
fn save_and_read(path)
  write_file(path, "line 1\nline 2")?
  content = read_file(path)?
  Ok(lines(content))
end
print(save_and_read("notes.txt"))
print(read_file("missing.txt"))
print(file_exists("notes.txt"), args())
```

`rythme 61-files.ry Ada 36`:

```text
Ok(["line 1", "line 2"])
Err("file not found: missing.txt")
true ["Ada", "36"]
```

```rythme
fn backup(name)
  create_folder(join_path(["backup", "2024"]))?
  write_file(join_path(["backup", "2024", name]), "saved")?
  list_folder("backup/2024")
end
print(backup("notes.txt"))
print(file_info("backup/2024/notes.txt"), file_info("backup"))
print(delete_file("backup/2024/notes.txt"), delete_file("backup"))
print(split_path("C:\\Users\\ada\\notes.txt"), split_path("/home/ada/") |> join_path)
print(options(["--dry-run", "--name=Ada", "notes.txt"]))
print(options(["-abc"]))
print(input_lines())
```

```text
Ok(["notes.txt"])
Ok({is_folder: false, size: 5}) Ok({is_folder: true, size: 0})
Ok(()) Err("backup is a folder: `delete_file` deletes only files, never folders")
["C:\\", "Users", "ada", "notes.txt"] /home/ada
Ok({args: ["notes.txt"], dry_run: true, name: "Ada"})
Err("`-abc` is not an option that Rythme understands: write `--abc` for a long name, or `-a -b` for several one-letter options")
Ok([])
```

```rythme
launch = unwrap(parse_date("2024-02-28T22:30:00+01:00"))
print(launch)
print(add_days(launch, 1) |> format_date, add_seconds(launch, 5400) |> format_date)
print(days_between({year: 2024, month: 1, day: 1}, {year: 2025, month: 1, day: 1}), days_in_month(2023, 2), is_leap_year(1900))
print(day_of_week({year: 2024, month: 2, day: 29}), seconds_between(launch, {year: 2024, month: 2, day: 28, hour: 21, minute: 30, offset: 0}))
print(parse_date("2023-02-29"))
```

```text
{year: 2024, month: 2, day: 28, hour: 22, minute: 30, second: 0, offset: 60}
2024-02-29T22:30:00+01:00 2024-02-29T00:00:00+01:00
366 28 false
4 0
Err("\"2023-02-29\" is not a valid date: the day must go from 1 to 28 in February 2023, not 29 (2023 is not a leap year)")
```

A date that does not exist is explained:

```rythme
deadline = {year: 2023, month: 2, day: 29}
print(add_days(deadline, 7))
```

```text
error[E023]: `add_days` needs a valid date: the day must go from 1 to 28 in February 2023, not 29 (2023 is not a leap year)
 --> 65-dates.ry:2:7
  |
2 | print(add_days(deadline, 7))
  |       ^^^^^^^^^^^^^^^^^^^^^ not a valid date
  |
  = help: a date is a record like `{year: 2024, month: 2, day: 29}`, with `hour`, `minute`, `second` and `offset` (minutes ahead of UTC) if needed
  = more about this error: `rythme explain E023`
```

Searching and cutting with regexes, and numbers written for people:

```rythme
journal = "2024-02-29 ERROR disk full\n2024-03-01 INFO ok\n2024-03-02 ERROR fan"
errors = lines(journal) |> filter(fn(line) = matches(line, ".* ERROR .*"))
print(errors |> map(fn(line) = unwrap(find_first(line, "[0-9-]+"))))
print(find_all("3 cats, 12 dogs and 1 fish", "\\d+"), split_by("a, b;c", "[,;] *"))
print(captures("v2.14", "v(\\d+)\\.(\\d+)"), named_captures("2024-02", "(?<year>\\d+)-(?<month>\\d+)"))
print(replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1"), replace_all("hello world", "\\b\\w", upper))
print(matches("AB-12", "[A-Z]\{2\}-[0-9]+"), matches("ab-12", "(?i)[A-Z]\{2\}-[0-9]+"), find_all("x<b>y</b>", "<.+?>"))
print(format_number(1234567.891, 2), format_number(1234567.891, 2, "fr"), format_number(-42000), pad_left(format_number(3.5, 2), 8))
print(words("Hello, world! It's well-known."), trim_start("  hi  "), trim_end("  hi  "), equals_ignore_case("Été", "ÉTÉ"))
```

```text
["2024-02-29", "2024-03-02"]
["3", "12", "1"] ["a", "b", "c"]
Some(["v2.14", "2", "14"]) Some({year: "2024", month: "02"})
2024-02-29 Hello World
true true ["<b>", "</b>"]
1,234,567.89 1 234 567,89 -42,000     3.50
["Hello", "world", "It's", "well-known"] hi     hi true
```

A mistake in a regex written in the code is shown before running:

```rythme
code = "AB-12"
print(matches(code, "[A-Z]+-([0-9]+"))
```

```text
error[E026]: invalid regex: this `(` is never closed
 --> 67-regex.ry:2:29
  |
2 | print(matches(code, "[A-Z]+-([0-9]+"))
  |                             ^ opened here
  |
  = help: add a `)` where the group ends; for the sign `(` itself, write `[(]`
  = more about this error: `rythme explain E026`
```

When there is nothing more to read (the keyboard input was closed with
Ctrl-D, or the text sent to the program has ended; here, the input is
empty), `input` stops the program with E017, so that a program that asks
again and again cannot run forever:

```rythme
print("Your name?")
name = input()
print("Hello, {name}!")
```

```text
Your name?
error[E017]: `input` found the end of the input
 --> 63-input.ry:2:8
  |
2 | name = input()
  |        ^^^^^^^ no more lines to read
  |
  = note: the keyboard input was closed (Ctrl-D), or the text sent to the program has ended
  = more about this error: `rythme explain E017`
```

---

## 7. Tools

### 7.1 The command line

```text
rythme [--lang en|fr|es|de|pt|zh] [--no-color] [COMMAND] [ARGUMENTS...]
```

| Command | Aliases | What it does |
|---------|---------|--------------|
| *(none)* | `repl` | the interactive mode (see [7.2](#72-the-interactive-mode-repl)) |
| `FILE [ARGS...]` | `run FILE [ARGS...]`, `lancer` | checks then runs the program; `ARGS` are given to `args()`. Without a file, in a project: its program (see [7.7](#77-projects-and-packages)), and so for `test`, `check`, `fmt` and `doc` |
| `check FILE...` | `verifier`, `vérifier` | checks each program without running it (see [5.5](#55-the-check-before-running)) |
| `test PATH... [--filter TEXT] [--seed N] [--watch] [-- ARGS...]` | `tester` | runs the tests of files and folders (see [4.8](#48-tests)) |
| `fmt [--check] FILE...` | `format`, `formater` | gives the code its official layout; also a folder, or `-` for the standard input (see [7.4](#74-the-formatter-rythme-fmt)) |
| `doc [--html] [--out FILE] FILE...` | `documenter` | writes the documentation of files or folders, in Markdown or as a web page (see [7.6](#76-documentation-rythme-doc)) |
| `new NAME` | `nouveau` | creates the folder of a new project (see [7.7](#77-projects-and-packages)) |
| `install` | `installer` | downloads the dependencies of the project and writes `rythme.lock` (see [7.7](#77-projects-and-packages)) |
| `exercise [list \| start NAME \| check FILE]` | `exercises`, `exercice`, `exercices` | the built-in exercises (see below) |
| `steps FILE [EXPRESSION] [--max N] [--width N]` | `etapes`, `étapes` | shows the evaluation step by step (see [7.8](#78-step-by-step-rythme-steps)) |
| `-e CODE [ARGS...]` | `eval CODE [ARGS...]` | runs the code given as an argument, and shows the value of its last expression unless it is `()`; `ARGS` are given to `args()` |
| `lsp` | | the language server, started by an editor (see [7.5](#75-editors-and-the-language-server)) |
| `help [NAME]` | `aide`, `--help`, `-h` | the list of topics, or the help of a topic, a built-in function or an error code |
| `explain [CODE]` | `expliquer` | the explanation of an error or warning code; without a code, the list of codes |
| `learn` | `apprendre`, `tutorial`, `tutoriel` | the guided tour: the 13 topics in order, with a pause after each one when run in a terminal |
| `version` | `--version`, `-V` | prints `rythme 1.0.0` |

**Finding the file.** A first argument that is not a command is taken as a
file when it ends with `.ry` or names an existing file; if `name` does not
exist but `name.ry` does, `name.ry` is used. A file that does not end with
`.ry` runs after a warning. A missing file is E017, and so is a folder (the
message names files inside it) or a file that is not text in UTF-8. Anything
else is an unknown command, with a suggestion.

**Options.** `--lang` and `--no-color` must come **before** the command;
anything after the file name goes to the program (for `rythme test`, what
comes after `--`). `--lang` accepts any value
that starts with the code of a language, `en`, `fr`, `es`, `de`, `pt` or `zh`
(`--lang es`, `--lang=en`, `--lang pt_BR`); another value, or `--lang` without
a value, is reported, with exit code 2.

**Languages of the messages.** Every message, help topic, explanation of a
code and page of `rythme learn` exists in English, French, Spanish
(`--lang es`), German (`--lang de`), Brazilian Portuguese (`--lang pt`) and
simplified Chinese (`--lang zh`); the keywords, the names of the built-in functions and the
commands stay the same in every language. The files written by `rythme new`
and the exercises of `rythme exercise` follow the language too. The texts of other languages are kept in
catalogs (`src/i18n/`); CONTRIBUTING.md explains how to add one. In
Chinese, the code in the help stays as written in English (only its comments
are translated), and the `^` under a mistake stay aligned: Chinese characters
take two columns of the terminal, in the code as in the messages.

**Exercises.** `rythme exercise` (or `rythme exercise list`) lists the 30
graded exercises built into the program (the files of
[`docs/exercises/`](exercises/README.md)). `start NAME` writes the exercise
named by its number (`7`) or its file name (`07-recursion`, `recursion`) into
the current folder, in the language of the messages (each language names
the files in its own words, like `07-rekursion` in German: any of these
names finds the exercise); it never replaces an
existing file (exit code 1). `check FILE` runs the tests of the file like
`rythme test`, then explains what to do next; `FILE` may also be the number
of an exercise started in the current folder. Its exit code is 0 when every
test passes and the tests of the exercise were not changed, 1 otherwise. An
unknown action or exercise is exit code 2.

**Help topics.** `rythme help TOPIC` accepts `start`, `syntax`, `functions`,
`lists`, `text`, `records`, `types`, `match`, `errors`, `functional`,
`tests`, `exercises`, `modules`, `system`, `projects`, `dates`, `repl`,
`fmt`, `doc`, `steps`, `editors`, `ruby` and `builtins`, and French or
alternative names: `syntaxe`, `fonctions`, `listes`, `texte`,
`enregistrements`, `motifs`, `erreurs`, `fonctionnel`, `boucles`,
`exercices`, `fichiers`, `système`, `dossiers`, `chemins`, `projets`,
`paquets`, `interactif`, `format`, `documentation`, `etapes`, `lsp`,
`vscode`, `éditeurs`, `blocs`, `symboles`, `bibliothèque`..., Spanish
names: `sintaxis`, `funciones`, `listas`, `registros`, `tipos`, `patrones`,
`errores`, `pruebas`, `ejercicios`, `proyectos`, `fechas`, `pasos`..., German
names: `funktionen`, `listen`, `typen`, `muster`, `fehler`, `übungen`,
`projekte`, `datum`, `schritte`..., Portuguese names: `sintaxe`, `funções`,
`listas`, `tipos`, `padrões`, `erros`, `testes`, `exercícios`, `projetos`,
`datas`, `passos`..., and Chinese names: `语法`, `函数`, `列表`, `文本`,
`记录`, `字典`, `类型`, `模式`, `错误`, `测试`, `练习`, `项目`, `日期`...

**Exit codes.**

| Code | Meaning |
|------|---------|
| 0 | success; `check` without errors (warnings allowed); all tests passed or no tests; `fmt --check` with every file already formatted |
| 1 | an error in the program (syntax, check or while running), a failed test, a file not found, or files to format for `fmt --check` |
| 2 | a wrong use of the command: unknown command or option, missing file name, unknown language, help topic or error code |
| 0 to 255 | the code given to `exit(code)` by the program |
| 101 | an internal error of Rythme (a bug, to be reported) |

**Environment.**

- `RYTHME_LANG`: the language of the messages (`en...`, `fr...`, `es...`,
  `de...`, `pt...` or `zh...`). Without it, the first non-empty variable among
  `LC_ALL`, `LC_MESSAGES`, `LANG` and `LANGUAGE` decides: the language whose
  code starts it (`LANG=es_ES.UTF-8`, `LANG=de_DE.UTF-8`, `LANG=pt_BR.UTF-8`,
  `LANG=zh_CN.UTF-8`), English otherwise.
  `--lang` takes precedence over all of them.
- `NO_COLOR`: when it is set (to any value), messages have no colors;
  `--no-color` does the same. Colors are used only when the error output is a
  terminal (on Windows, only in terminals known to support them).
- `HOME` (`USERPROFILE` on Windows): where the REPL keeps its history, and
  where `rythme install` keeps packages (`~/.rythme/packages`).
- `RYTHME_HOME`: another folder for the packages (`RYTHME_HOME/packages`).

**Outputs.** What the program prints (`print`, the prompt of `input`, the
results of `check`, `test` and `fmt`) goes to the standard output; error and
warning messages and `debug` go to the error output.

```text
$ rythme version
rythme 1.0.0
$ rythme -e '[1, 2, 3] |> map(fn(x) = x * x)'
[1, 4, 9]
$ rythme -e 'print("hello")'
hello
$ rythme -e 'print(args())' Ada 36
["Ada", "36"]
$ rythme chek 01.ry
unknown command or file `chek`
did you mean `rythme check`?
see `rythme help`
$ rythme help lenght
No help found for `lenght`.
in Rythme, this is called `len`
See the list of topics with `rythme help`.
$ rythme --lang it hello.ry
unknown language `it`: write `--lang` and one of en, fr, es, de, pt, zh, like `rythme --lang fr hello.ry`
```

The last three exit with code 2 (the last one is written in the language of
the system).

### 7.2 The interactive mode (REPL)

`rythme` without arguments starts the interactive mode. The prompt is
`ry> `, and `..> ` while an input is not finished. Each input is checked and
run like a small file; the value of its last expression is shown in code form
(unless it is `()`). Definitions are kept from one input to the next.

Differences with a file:

- names may be defined again (no E011);
- a single `fn` with the same name and number of parameters as the function
  defined by the previous input adds a definition (a case) to it;
- the checker does not report forgotten cases of functions (W002);
- `use` paths and `:load` are relative to the current folder.

An input that is not finished (an open bracket, a block without `end`, a text
not closed...) continues on the next line; two empty lines in a row run it
anyway, and at the end of the input (Ctrl-D) an input left unfinished is
checked, so that its error is shown. A line that starts with `|>` or with
`.name` continues the previous input, so a pipeline can be typed (or pasted)
step by step. Ctrl-C cancels
the current input, Ctrl-D leaves. Tab completes names, keywords and commands.
The history is kept in `~/.rythme_history`. Typing `help` (or `aide`) shows
the help of the REPL; `exit`, `quit` (or `quitter`) leaves.

| Command | Aliases | What it does |
|---------|---------|--------------|
| `:help` | `:h`, `:?`, `:aide` | the help of the REPL |
| `:quit` | `:q`, `:exit`, `:quitter` | leave |
| `:doc NAME` | | the documentation of a defined name, a built-in function or a topic |
| `:load FILE` | `:charger` | load (again) a file and run it (`.ry` is added if needed) |
| `:reset` | | forget everything |
| `:lang en\|fr\|es\|de\|pt\|zh` | `:langue` | change the language of the messages |
| `:learn` | `:apprendre` | the guided tour |
| `:explain CODE` | `:expliquer` | explain an error code |
| `:names` | `:noms` | the names defined so far |
| `:steps EXPRESSION` | `:etapes`, `:étapes` | shows the evaluation of the expression step by step (see [7.8](#78-step-by-step-rythme-steps)) |

A session (the inputs are shown after the prompts):

```text
Rythme 1.0.0 — :help for help, :quit to leave
ry> 1 + 2
3
ry> name = "Ada"
ry> "Hello, {name}"
"Hello, Ada"
ry> name = "Grace"
ry> fn fib(0) = 0
ry> fn fib(1) = 1
(`fib` now has 2 definitions, tried in order; :reset to start again)
ry> fn fib(n) = fib(n - 1) + fib(n - 2)
(`fib` now has 3 definitions, tried in order; :reset to start again)
ry> fib(20)
6765
ry> fn greet(who)
..>   message = "Hello, {who}!"
..>   message
..> end
ry> greet(name)
"Hello, Grace!"
ry> [1, 2, 3]
[1, 2, 3]
ry> |> map(fn(x) = x * 10)
[10, 20, 30]
ry> :names
fib  greet  name
ry> :load geometry
geometry.ry runs once
geometry.ry loaded
ry> rectangle_area(2, 5)
10
ry> :reset
Everything was forgotten.
ry> fib(3)
error[E003]: unknown name `fib`
 --> <repl>:1:1
  |
1 | fib(3)
  | ^^^ not defined
  |
  = help: define it before using it: `fib = ...` or `fn fib(...) = ...`
  = more about this error: `rythme explain E003`
```

### 7.3 The web playground

The [website](https://maxlestage.github.io/Rythme/) has a playground that
runs the real interpreter, compiled to WebAssembly, in the browser: the same
language, the same messages and the same help, without installing anything.
It has buttons to run (also Ctrl+Enter), check and test (also
Ctrl+Shift+Enter) the code, a **Format** button (also Shift+Alt+F) that gives
the code the layout of `rythme fmt` (see [7.4](#74-the-formatter-rythme-fmt);
code with an error is left as it is, and the error is shown), a **Steps**
button that shows the evaluation step by step like `rythme steps` (see
[7.8](#78-step-by-step-rythme-steps)), a button to
stop a program that does not end, the choice between English and French, a
menu of examples, and a help panel (topics, search, and the explanation of an
error code by clicking on it).

Differences with the `rythme` command: files cannot be read or written
(`read_file`, `write_file` and `append_file` return `Err(...)`), `input()`
returns an empty text, `args()` is empty, `env(name)` is `None`, `exit(code)`
stops the program with error E021, and nested calls stop at 600 (tail calls
have no limit).

### 7.4 The formatter: `rythme fmt`

```text
rythme fmt [--check] FILE_OR_FOLDER...
rythme fmt [--check] -
```

`rythme fmt` gives code the official layout of Rythme, so that every program
looks the same. It changes only the layout (spaces, blank lines, line
breaks, `;`, the comma after the last element of a list written on several
lines), never what the program does: the result is read again, and must
give the same program with the same comments, otherwise the file is left as
it is.

- `rythme fmt hello.ry` rewrites the file (`hello` is enough when `hello.ry`
  exists, as with `rythme run`) and writes `formatted: hello.ry`. A folder
  stands for every `.ry` file in it and in its subfolders, hidden folders
  such as `.git` aside. Several files and folders may be given; a summary
  follows.
- `--check` changes nothing: it lists the files that need formatting and
  exits with code 1 if there are any, for continuous integration.
- `-` reads the code from the standard input and writes the result on the
  standard output.
- A file with a syntax error is left as it is, and the error is shown (exit
  code 1).
- `format` and `formater` are aliases of `fmt`, `--verifier` of `--check`.

The style, described by `rythme help fmt`: two spaces per level; spaces
around operators, `=`, `->` and `|>`, none inside brackets or around `..`; a
pipeline written on several lines has one step per line, and a type written
on several lines one variant per line; a list or a record written on several
lines ends with a comma; one statement per line, and `end` on its own line;
a blank line around a definition written on several lines, and never two
blank lines in a row; every comment is kept, and the comments at the end of
consecutive lines are aligned; a block given to a call written on one line
stays on one line, with a space around its parts (`xs.map do |x| x * 2 end`,
`xs.map { |x| x * 2 }`), and on several lines its statements are one level
deeper than the line of `do`. Where the language allows a choice, line
breaks are kept, and long lines are kept as written.

With `messy.ry`:

```rythme
type Shape = Circle(radius)|Rect(width,height)
fn area(s)=match s
    Circle(r)->pi*r*r
    Rect(w,h)->w*h
  end
prices=[12,5,30]
total=prices|>filter(fn(p)=p>10)
  |>sum
print( total ) ; print(area(Rect(2,3)))
```

`rythme fmt - < messy.ry` writes:

```text
type Shape = Circle(radius) | Rect(width, height)

fn area(s) = match s
  Circle(r) -> pi * r * r
  Rect(w, h) -> w * h
end

prices = [12, 5, 30]

total = prices
  |> filter(fn(p) = p > 10)
  |> sum

print(total)
print(area(Rect(2, 3)))
```

In a folder that holds `messy.ry` and an already formatted `clean.ry`:

```text
$ rythme fmt --check .
needs formatting: ./messy.ry
✗ 1 file(s) to format, 1 already formatted; `rythme fmt` (without --check) formats them
$ rythme fmt .
formatted: ./messy.ry
1 file(s) formatted, 1 already formatted
$ rythme fmt --check .
✓ 2 file(s) already formatted
```

The first command exits with code 1, the others with code 0.

### 7.5 Editors and the language server

`rythme lsp` is a language server (the Language Server Protocol, on the
standard input and output): an editor starts it by itself and gets, for
`.ry` files, the problems as you type (every error and warning of
`rythme check`, those of the files given to `use` included), documentation
on hover (built-in functions with an example, keywords, your functions and
types with their `##` comments, the signature the checker infers for a
function, like `fn area(side: Int|Float) -> Int|Float`, the explanation of
an error code),
completion, the parameters of a call, go to definition, references, rename,
the outline of the file, quick fixes for typos, formatting like
`rythme fmt`, the types the checker is sure of next to the names (inlay
hints such as `names: List(Text)` or `half(x: Int|Float)`), colors that follow what each name is
(semantic tokens), *Run* and *Run test* above the code (code lenses), the
names of every file of the workspace, folding and growing the selection.
The messages are in the language of the editor, unless a
setting of the editor, `--lang` (`rythme --lang fr lsp`) or `RYTHME_LANG`
chooses it. Started by hand in a terminal, it says what it is for;
`rythme lsp --help` shows the same text as `rythme help editors`.

- **VS Code**: the extension in [`editors/vscode`](../editors/vscode) adds
  colors, indentation, snippets and the language server, with commands to
  run the file, run its tests or explain an error code, and the settings
  `rythme.path` (the program) and `rythme.language`.
- **Other editors** (Neovim, Helix, Zed, Emacs, Sublime Text, Kate...): any
  editor that speaks the Language Server Protocol can start `rythme lsp`;
  [`editors/README.md`](../editors/README.md) gives the configuration for
  each.

### 7.6 Documentation: `rythme doc`

```text
rythme doc [--html] [--out FILE] FILE_OR_FOLDER...
```

`rythme doc` writes the documentation of programs from their `##` comments
(see [1.2](#12-comments)), their types and the definitions of their
functions. Nothing is run.

- `rythme doc geometry.ry` writes Markdown on the standard output (`geometry`
  is enough when `geometry.ry` exists). A folder stands for every `.ry` file
  in it and in its subfolders, hidden folders aside; several files and
  folders may be given.
- A file is documented with the files it uses (`use geometry`, `use
  shapes/circle as circle`, `use "geometry.ry"`), each one once: the
  documentation of a program includes that of its modules.
- `--html` writes one web page instead, with the look of the website, light
  or dark like the system. It needs nothing else (no script, no other
  file): it can be opened from the disk or put on any web server.
- `--out FILE` (`-o`) writes to a file and says so; a name ending with
  `.html` gives the web page.
- `documenter` is an alias of `doc`; the titles (*Functions*, *Values*...)
  are in the language of the messages.
- A file that cannot be read, or that has a syntax error, is reported and
  left out; the others are documented, and the exit code is 1.

For each file come: its description, the `##` lines at its top when a blank
line separates them from what follows; the files it uses, as links (with
the name given by `as` when it is not the name of the file); its
types; its functions, with the first line of **every** definition
(patterns and `if` conditions included); its values (with their definition
when it fits on one line); and the names of its tests. Names starting with
`_` are left out, as details of their file.

In a comment, a name between backticks, like `` `area` `` or
`` `len(list)` ``, becomes a link to its documentation; a constructor leads
to its type; in a file that has `use geometry`, `` `geometry.area` `` leads
to `area` in `geometry.ry`; the built-in functions named this way are
explained at the end. A blank `##` line starts a new paragraph, and lines starting with `- `
make a list. In Markdown, code is in `~~~rythme` blocks, and the links use
the anchors GitHub gives to titles.

With `geometry.ry`:

```rythme
## Shapes and their areas.

## A shape to draw.
type Shape = Circle(radius) | Rect(width, height)

## The area of a shape: for a `Circle`, `pi` times the square
## of its radius. To count shapes, use `len`.
fn area(Circle(r)) = pi * _square(r)
fn area(Rect(w, h)) = w * h

## How many sides a square has.
sides = 4

fn _square(x) = x * x

test "area of a square" = area(Rect(2, 2)) == 4
```

`rythme doc geometry.ry` writes:

```text
# geometry.ry

Shapes and their areas.

## Types

### Shape

~~~rythme
type Shape = Circle(radius) | Rect(width, height)
~~~

A shape to draw.

## Functions

### area

~~~rythme
fn area(Circle(r))
fn area(Rect(w, h))
~~~

The area of a shape: for a [`Circle`](#shape), `pi` times the square
of its radius. To count shapes, use [`len`](#len).

## Values

### sides

~~~rythme
sides = 4
~~~

How many sides a square has.

## Tests

- area of a square

## Built-in functions mentioned

### len

~~~rythme
len(x)
~~~

Length of a text (in characters), a list, a record, a dict or a set. Same as: `length`, `size` (its Ruby names).

Example:

~~~rythme
len("été")                 # 3
len([1, 2])                # 2
~~~
```

`rythme doc --html . --out doc.html` writes the same for every file of the
folder as one page, with a table of contents.

### 7.7 Projects and packages

A **project** is a folder with a `rythme.toml` file, its code in `src/` and
its tests in `tests/`. A project can use other projects, its
**dependencies**, from git or from a folder: they are **packages**.

**Creating a project.** `rythme new NAME` creates the folder `NAME` with a
small program that already works:

```text
$ rythme new hello
Created the project `hello`:
  hello/rythme.toml
  hello/src/main.ry
  hello/src/greeting.ry
  hello/tests/greeting_test.ry
  hello/README.md
  hello/.gitignore
Next: `cd hello`, then `rythme run` to run it and `rythme test` to test it (see `rythme help projects`)
$ cd hello
$ rythme run
Hello, World!
$ rythme test
Running 1 test(s)
  ✓ hello
1 test(s): 1 passed, 0 failed
```

The name of a project is a Rythme name in lower case (letters, digits and
`_`, not a keyword), because it is used in `use`: `rythme new my-app` is
E025, with the help `try rythme new my_app` (exit code 2). A folder that
already exists and is not empty is never touched (exit code 1). The files
are written in the language of the messages (`src/salutation.ry` in
French, `src/saludo.ry` in Spanish, `src/gruss.ry` in German,
`src/saudacao.ry` in Portuguese; in Chinese the names stay those of English
and the texts are in Chinese).

**`rythme.toml`.** Every value is a text in quotes; `#` starts a comment.

```toml
name = "hello"            # required: the name of the package
version = "0.1.0"         # required: three numbers, major.minor.patch
entry = "src/main.ry"     # the program; this one when it is not given

[dependencies]
mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
geometry = { git = "https://github.com/someone/geometry", commit = "4d868c2" }
tools = { path = "../tools" }
```

A dependency from git gives its address and a fixed version: a `tag` or a
`commit` (at least 4 hexadecimal digits). A dependency from a folder gives
its `path`, relative to the project. The name before `=` is the name used in
`use`. This is a small part of TOML, read by Rythme itself: texts with `"`
(with `\"`, `\\`, `\n`, `\t`) or `'`, and tables on one line. Any other key,
section (`[package]` included) or kind of value is E025, shown at its place
in `rythme.toml`, with a suggestion when a close key exists:

```text
error[E025]: unknown key `tga`
 --> rythme.toml:6:30
  |
6 | tools = { path = "../tools", tga = "x" }
  |                              ^^^
  |
  = note: the keys here are `git`, `tag`, `commit`, `path`
  = help: did you mean `tag`?
  = more about this error: `rythme explain E025`
```

**`rythme install`.** In the folder of the project (or one of its folders),
`rythme install` downloads each dependency from git, then the dependencies
of each dependency, and writes `rythme.lock`:

```text
$ rythme install
Installing the dependencies of `hello`
  ✓ mathlib 1.0.0: tag v1.0.0, commit 4d868c2 (downloaded)
  ✓ tools 0.2.0: folder ../tools
2 package(s) ready
```

- Downloading uses the program `git` (`git clone`, then the commit of the
  tag or of the commit). Without it, `rythme install` stops with E025 and
  says where to get it; dependencies from a folder do not need it.
- Each package from git is kept in its own folder under
  `RYTHME_HOME/packages` (`~/.rythme/packages` by default), named after its
  address and its version, and shared by every project of the computer. A
  package already there is not downloaded again.
- Every dependency must be a project itself, with its `rythme.toml`.
- `rythme.lock` holds, for each package from git, its address, its tag (or
  the commit asked), and the **exact commit** that was installed. Keep it
  with the project: `rythme install` on another computer then installs the
  same commits, even if a tag was moved. When `rythme.toml` asks for another
  version, `rythme install` takes it and updates `rythme.lock`; to take the
  new commit of a moved tag, delete `rythme.lock`.
- A tag or a commit that does not exist, an address that git cannot reach,
  a folder that does not exist: E025, with what git said, the line of
  `rythme.toml`, and the closest tag when there is one.

**Using a package.** `use NAME/FILE` is the file `src/FILE.ry` of the
package `NAME`, and is used through the last part of the path, as any
`use` (see [4.9](#49-modules-use)):

```rythme
# src/main.ry
use mathlib/vectors       # the file src/vectors.ry of mathlib
use mathlib               # the program of mathlib: the `entry` of its rythme.toml
use hello/greeting        # a file of the project itself (`hello`), useful in tests/
print(vectors.add({x: 1, y: 2}, {x: 3, y: 4}))
```

- The packages are found through the nearest `rythme.toml` above the file
  that contains the `use`: inside a package, its own dependencies are used.
- A file next to the one that contains the `use` comes first: `use
  mathlib/vectors` is first `mathlib/vectors.ry` in the folder of the file.
- The private names (`_name`) of a package stay private, and its files are
  checked like the others.
- A package that is not installed is E025, at the `use`, with the help to
  run `rythme install`; a file that the package does not have is E017, with
  the closest file. A `use` that names nothing in a project says which
  packages it can reach.
- The language server knows packages too: no false problem on `use
  mathlib/vectors`, completion of the package names after `use `, and of
  their files after `use mathlib/`.

**Commands without a file.** In the folder of a project or in one of its
folders, the commands take the files of the project when no file is given:

| Command | Works on |
|---------|----------|
| `rythme run` | the program, `entry` of `rythme.toml` |
| `rythme test` | every file of `src/` and `tests/` (and their folders) that has tests, one after the other, each one with its name, then one summary, as `rythme test FOLDER` does (`--filter`, `--seed`, `--watch` and `--` work the same); the program when none has |
| `rythme check` | the program, then every file of `src/` and `tests/` |
| `rythme fmt` | the folders `src/` and `tests/` (`rythme fmt --check` too) |
| `rythme doc` | the folder `src/` (`--html`, `--out` work as usual) |

Outside of a project, a missing file is still a usage error (exit code 2).
A file given by name never needs `rythme.toml`, even in a project.

### 7.8 Step by step: `rythme steps`

```text
rythme steps FILE [EXPRESSION] [--max N] [--width N]
```

`rythme steps` (in French `rythme etapes`) shows how a program computes its
values, one step at a time, for learning and teaching. It uses the
**substitution model**: a call is replaced by the body of its function, with
the values of the arguments written in place of the parameters, until only
the result remains.

- Without `EXPRESSION`, every top-level definition (`name = ...`) and
  expression of the file is shown, in order, after its functions and types
  are defined, as when the file runs. With `EXPRESSION` (`rythme steps fib.ry
  "fib(5)"`), the definitions of the file are made without being shown, its
  expressions are skipped, and only the expression is shown.
- The first line is the expression as written (pipes are shown as the calls
  they make); each next line, after `→`, is the expression after one more
  step, written like `rythme fmt` writes code.
- A step is **one call or one operation**, in the order of evaluation of the
  interpreter (see [3](#3-expressions)). The body of a called function is
  shown already simplified as far as it can be without calling another
  function: the operations on known values are done, the branch of `if` and
  the case of `match` are chosen. A name of the program is replaced by its
  value when it is needed. Built-in functions, and the functions of other
  files, give their result in one step.
- What the program prints appears between the steps, where it happens; an
  error stops the evaluation with the same message as when the program runs.
- `--max N` shows at most `N` steps of each expression (200 by default):
  after them, `...` says that the rest is computed without being shown, and
  the last line is the result. `--width N` cuts the lines longer than `N`
  characters (100 by default, 20 at least) with `…`; a step is shown on 24
  lines at most. A calculation that never ends is stopped after 10 000 000
  steps, and one nested more than 10 000 levels deep (1 200 in the playground)
  is stopped too, with a message.

The same is available in the interactive mode with `:steps EXPRESSION`
(see [7.2](#72-the-interactive-mode-repl)) and in the playground with the
**Steps** button (see [7.3](#73-the-web-playground)). For example, with
this program:

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)
fn total([]) = 0
fn total([x, ..rest]) = x + total(rest)
fn describe(n)
  kind = if n % 2 == 0 then "even" else "odd"
  "{n} is {kind}"
end
print(fib(3))
t = total([10, 20, 30])
describe(t // 4)
[1, 2, 3] |> map(fn(x) = x * t)
```

`rythme steps` shows:

```text
print(fib(3))
  → print(fib(2) + fib(1))
  → print(fib(1) + fib(0) + fib(1))
  → print(1 + fib(0) + fib(1))
  → print(1 + 0 + fib(1))
  → print(1 + fib(1))
  → print(1 + 1)
  → print(2)
2
  → ()

t = total([10, 20, 30])
  → 10 + total([20, 30])
  → 10 + (20 + total([30]))
  → 10 + (20 + (30 + total([])))
  → 10 + (20 + (30 + 0))
  → 10 + (20 + 30)
  → 10 + 50
  → 60

describe(t // 4)
  → describe(60 // 4)
  → describe(15)
  → "15 is odd"

map([1, 2, 3], fn(x) = x * t)
  → [60, 120, 180]
```

---

## Appendix A. Grammar

This grammar, in EBNF, describes the syntax accepted by the interpreter.
`{ x }` means zero or more `x`, `[ x ]` means an optional `x`. `NL` is a new
line or `;` (several in a row count as one). Inside `( )`, `[ ]` and `{ }`,
`NL` is ignored, except within a nested block; outside, the continuation rules
of [1.9](#19-new-lines--and-continuation-lines) apply. Trailing commas are
allowed in every comma-separated list.

```text
program      = { NL } { item ( NL | EOF ) } ;
item         = type_def | test_def | use_def | statement ;
statement    = fn_def | binding | command | expression ;
command      = NAME [ "?" ] expression { "," { NL } expression }
               [ "do" [ params ] block_end ] ;     (* 3.15 *)

binding      = pattern_atom "=" expression ;
fn_def       = "fn" NAME [ "?" ] "(" [ patterns ] ")" [ "if" expression ] body ;
body         = "=" result
             | NL block_end ;
result       = command | expression ;              (* 3.15 *)
block_end    = { statement NL } [ statement ] "end" ;
type_def     = "type" CTOR "=" [ "|" ] variant { "|" variant } ;
variant      = CTOR [ "(" [ NAME { "," NAME } ] ")" ] ;
test_def     = "test" PLAIN_TEXT body ;
use_def      = "use" PLAIN_TEXT ;

expression   = or_expr ;
or_expr      = and_expr { "or" and_expr } ;
and_expr     = not_expr { "and" not_expr } ;
not_expr     = "not" not_expr | comparison ;
comparison   = pipe [ ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) pipe ] ;
pipe         = range { "|>" step } ;
step         = primary { "." MEMBER } [ "(" [ arguments ] ")" ] { postfix } ;
range        = sum [ ".." sum ] ;
sum          = product { ( "+" | "-" ) product } ;
product      = unary { ( "*" | "/" | "//" | "%" ) unary } ;
unary        = "-" unary | power ;
power        = primary { postfix } [ "**" unary ] ;
postfix      = "(" [ arguments ] ")" | "[" expression "]" | "?"
             | "." MEMBER [ "(" [ arguments ] ")" ] | block ;
block        = "do" [ params ] block_end
             | "{" params { statement NL } [ statement ] "}" ;
params       = "|" [ pattern_atom { "," pattern_atom } ] "|" | "||" ;
arguments    = expression { "," expression } ;

primary      = INT | FLOAT | TEXT | SYMBOL | "true" | "false" | NAME | CTOR
             | "(" ")" | "(" expression ")"
             | "[" [ list_item { "," list_item } ] "]"
             | "{" [ record_item { "," record_item } ] "}"
             | "fn" "(" [ patterns ] ")" body
             | "if" expression "then" result "else" result
             | "unless" expression "then" result "else" result
             | "match" expression NL case { NL case } NL "end"
             | "do" block_end ;
list_item    = [ ".." ] expression ;
record_item  = ".." expression | KEY ":" expression | NAME ;
case         = pattern [ "if" expression ] "->" result ;

patterns     = pattern { "," pattern } ;
pattern      = pattern_atom { "|" pattern_atom } ;
pattern_atom = "_" | NAME
             | CTOR [ "(" [ patterns ] ")" ]
             | [ "-" ] INT | [ "-" ] FLOAT | PLAIN_TEXT | SYMBOL | "true" | "false"
             | "(" ")" | "(" pattern ")"
             | "[" [ list_patterns ] "]"
             | "{" [ field_pattern { "," field_pattern } ] "}" ;
list_patterns = ".." [ NAME ] [ "," ]
             | pattern [ "," [ list_patterns ] ] ;
field_pattern = KEY ":" pattern | NAME | ".." ;

MEMBER       = NAME [ "?" ] | CTOR | keyword | reserved word ;
SYMBOL       = ":" NAME [ "?" ] ;
KEY          = MEMBER | PLAIN_TEXT ;
```

Notes:

- `binding`: when a statement starts with `(`, `[`, `{` or a constructor
  followed by `(`, and the bracket that closes it is followed by `=`, the left
  side is read as a pattern. Otherwise the statement is read as an
  expression, and, if `=` follows, that expression is turned into a pattern:
  only a name, `_`, a constructor without fields and a literal are accepted
  there (E001 otherwise, with a hint for `f(x) = ...`, `xs[0] = ...` and
  `r.x = ...`).
- `field_pattern`: `..` followed by a name (`{name, ..others}`) is E001.
- `NAME [ "?" ]`: one name only when the `?` touches the name and the rules
  of [1.4](#14-names) say so; otherwise the `?` is the `postfix` operator.
  A `NAME` followed by `?` that is a name (`even?`) is a `primary` too.
- `block`: only after a call, a method call, a `.` MEMBER or a NAME, with
  `do` or `{` on the same line ([3.14](#314-blocks-given-to-calls)); it is
  the last argument of the call. `{` followed by `|` or `||` always starts a
  block, never a record.
- `"unless"`: the word starts an `unless` only as described in
  [1.5](#15-keywords-and-reserved-words); elsewhere it is a NAME.
- `SYMBOL`: `:` directly followed by a name, except the `:` that follows a
  field name after `{` or `,` ([2.14](#214-symbol)); a `?` that touches the
  name is part of the symbol (`:even?`), never the `postfix` operator.
- In `match`, the subject must end on the line of `match`, and the cases may
  be separated by blank lines; `end` may follow the last case on the same
  line. In blocks, `end` may follow the last statement on the same line.
- `then` and `else` may be preceded by new lines; new lines are skipped after
  `then`, `else`, `->`, `=` and every binary operator.
- In `step`, a `.MEMBER (arguments)` makes a method call whose arguments are
  the piped value followed by `arguments` (see [3.6](#36-pipes-)).
- Brackets, blocks and other constructs may be nested at most 256 levels
  deep, and an expression may be at most 1000 levels deep, counting each
  operator, call, index and field (E001, see
  [Appendix B](#appendix-b-limits-of-the-implementation)).

## Appendix B. Limits of the implementation

| Limit | Value | Error |
|-------|-------|-------|
| digits of a whole number (Int), in the code, as a result or read by `int` | 100 000 | E023 |
| elements of a list (built by a range, `+`, `..xs`, `repeat`, `flatten`, `flat_map`, `chars`) | 50 000 000 | E023 |
| length of a text (built by `+`, `{...}`, `join`, `replace`, `repeat`, `pad_left`, `pad_right`) | 100 000 000 bytes in UTF-8 | E023 |
| nesting of brackets, blocks and other constructs in the code | 256 levels | E001 |
| depth of an expression: each operator, call, index or field is one level, so `1 + 1 + ...` holds at most 1000 numbers | 1000 levels | E001 |
| texts nested in `{...}` of texts | 32 levels | E001 |
| nested (non-tail) calls | depends on the function and on the stack: several hundred thousand with `rythme` (about 900 000 for a simple recursion), 600 in the playground | E014 |
| steps of `rythme steps`: shown for each expression; in all; depth of an expression being evaluated | 200 (`--max`); 10 000 000; 10 000 levels, 1 200 in the playground | — |
| digits of `round(x, digits)` | clamped to −15...15 | — |
| decimals of `text(x, decimals)` | 0 to 100 | E023 |
| indentation of `to_json(value, indent)` | 0 to 16 spaces | E023 |
| exit codes | 0 to 255 | E023 |

Tail calls and the list functions (`map`, `fold`, `sort`...) do not count as
nested calls, and `parse_json` and `to_json` read and write data nested
hundreds of thousands of levels deep (indented with `to_json(value, indent)`,
such data would be too large: E023).
