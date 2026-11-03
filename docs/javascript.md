# Coming from JavaScript: its standard library in Rythme

*En français : l'inventaire de la bibliothèque standard de JavaScript
(ECMAScript, et les API web les plus courantes) comparée à celle de Rythme :
ce qui existe sous un autre nom, ce qui a été ajouté pour elle (**new**), et ce
qui manque exprès. `rythme help javascript` en donne l'essentiel, et la
référence du site a une page par fonction.*

What JavaScript developers reach for every day (the globals of ECMAScript:
`Math`, `Number`, `String`, `Array`, `Object`, `Map`, `Set`, `JSON`, `Date`,
`RegExp`, `Promise`; and the web APIs `console`, `setTimeout`, `URL`,
`URLSearchParams`, `crypto`, `btoa`), against what Rythme has. The functions
marked **new** were added for this audit (15 functions); `rythme help NAME`
documents each one, with an example that the tests run. The names come from
the ECMAScript specification and the habits of the language; the texts here
are Rythme's own.

Rythme is purely functional, which explains most of the differences:

- **Nothing changes a value in place.** `push`, `pop`, `shift`, `unshift`,
  `splice`, `sort`, `reverse`, `fill`, `copyWithin`, `Object.assign` change
  their array or object in JavaScript; in Rythme every function returns a new
  value (`push(xs, x)` is a new list, like `xs.concat([x])` or `toSpliced`).
- **No `null`, `undefined` or exceptions.** What JavaScript gives as
  `undefined`, `-1` or `null` when nothing is found is an `Option`
  (`Some(x)` or `None`); what throws is a `Result` (`Ok(x)` or `Err(message)`).
- **No objects with methods, `this` or prototypes.** Records (`{name: "Ada"}`)
  and types (`type Shape = Circle(r) | Square(s)`); a "method" is a function
  whose first argument is the value: `xs.map(f)` is `map(xs, f)`, and both
  are written in Rythme.
- **snake_case names**: `to_upper_case` is not needed, `upper` is the name; the
  JavaScript names are known to the messages (`toUpperCase` gives "did you mean
  `upper`?") and listed below.
- **Promises** are tasks: `async fn` starts a function on another core and
  `await` gives its result ([manual 4.11](reference.md#411-tasks-async-fn-and-await)).

Legend: the Rythme column is a function or an expression; **new** marks a
function added for this audit; *absent* says why there is none on purpose.

## Math

| JavaScript | Rythme |
|---|---|
| `Math.abs`, `ceil`, `floor`, `round` | `abs`, `ceil`, `floor`, `round` (`round(x, 2)` keeps 2 decimals) |
| `Math.trunc(x)` | `int(x)` (toward zero) |
| `Math.sign(x)` | `if x > 0 then 1 else if x < 0 then -1 else 0`: `sign` is a name left to programs (the manual defines its own) |
| `Math.max(...xs)`, `Math.min(...xs)` | `max(xs)`, `min(xs)`, or `max(a, b)` |
| `Math.pow(x, y)`, `x ** y` | `pow(x, y)`, `x ** y` |
| `Math.sqrt`, `Math.cbrt` | `sqrt`, **new** `cbrt` (negative numbers too) |
| `Math.hypot(x, y)` | **new** `hypot(x, y)` (no overflow for big values) |
| `Math.exp`, `Math.log` | `exp`, `log` |
| `Math.log2(x)`, `Math.log10(x)` | `log(x, 2)`, `log(x, 10)` |
| `Math.sin`, `cos`, `tan`, `asin`, `acos`, `atan` | the same names |
| `Math.atan2(y, x)` | `atan(y, x)` |
| `Math.sinh`, `cosh`, `tanh` | **new** `sinh`, **new** `cosh`, `tanh` |
| `Math.asinh`, `acosh`, `atanh` | **new** `asinh`, **new** `acosh`, **new** `atanh` |
| `Math.PI` | `pi` |
| `Math.E` | `exp(1)` (a global `e` would take the name of countless `Err(e)`) |
| `Math.random()` | `random()`, `random(1, 6)` for a whole number; `random_seed(n)` makes it repeatable |
| `Math.clz32`, `fround`, `imul`, `expm1`, `log1p` | *absent*: 32-bit and floating-point tricks; whole numbers never overflow in Rythme |

## Number

| JavaScript | Rythme |
|---|---|
| `parseInt(s)`, `parseFloat(s)`, `Number(s)` | `parse_int(s)`, `parse_float(s)`: `Ok(n)` or `Err(message)`, never `NaN` |
| `parseInt(s, 16)`, `n.toString(16)` | `from_base(s, 16)`, `to_base(n, 16)` |
| `n.toFixed(2)` | `text(x, 2)` |
| `n.toPrecision`, `Intl.NumberFormat` | `format(x, ".3g")`, `format_number(x)` |
| `Number.isNaN`, `Number.isFinite` | `is_nan`, `is_finite` |
| `Number.isInteger(x)` | `type_of(x) == "Int"` (Int and Float are two types) |
| `Number.MAX_SAFE_INTEGER`, `BigInt` | *absent*: whole numbers grow as needed (`2 ** 100` is exact) |

## String

| JavaScript | Rythme |
|---|---|
| `s.length` | `len(s)` (characters, not UTF-16 units) |
| `s.at(i)`, `s.charAt(i)`, `s[i]` | `get(s, i)` (an `Option`; `-1` is the last), `s[i]` |
| `s.charCodeAt(i)`, `codePointAt`, `String.fromCharCode` | `char_code(c)`, `char(n)` |
| `s.toUpperCase()`, `toLowerCase()` | `upper(s)`, `lower(s)` |
| `s.trim()`, `trimStart()`, `trimEnd()` | `trim`, `trim_start`, `trim_end` |
| `s.padStart(n, c)`, `padEnd` | `pad_left(s, n, c)`, `pad_right` |
| `s.includes(x)`, `startsWith`, `endsWith` | `contains`, `starts_with`, `ends_with` |
| `s.indexOf(x)`, `lastIndexOf(x)` | `index_of`, `last_index_of` (an `Option`, not `-1`) |
| `s.slice(a, b)`, `substring(a, b)` | `slice(s, a, b)` |
| `s.split(sep)`, `arr.join(sep)` | `split(s, sep)`, `join(xs, sep)` |
| `s.repeat(n)` | `repeat(s, n)` |
| `s.replace(a, b)`, `replaceAll(a, b)` | `replace_first(s, a, b)`, `replace(s, a, b)` |
| `s.replace(/re/g, f)` | `replace_all(s, pattern, f)` |
| `s.match(re)`, `matchAll(re)`, `re.test(s)`, `re.exec(s)` | `find_first`, `find_all`, `matches`, `captures`, `named_captures` |
| `s.concat(t)`, `` `${a} ${b}` `` | `s + t`, `"{a} {b}"` |
| `s.localeCompare(t)` | `s < t` (by code points), `equals_ignore_case` |
| `s.normalize()` | *absent*: texts are kept as written (no Unicode tables in the interpreter) |
| `[...s]`, `Array.from(s)` | `chars(s)` |
| `new TextEncoder().encode(s)` | `bytes(s)` |
| `encodeURIComponent`, `decodeURIComponent` | `url_encode`, `url_decode` |
| `btoa`, `atob` | `base64_encode`, `base64_decode` (UTF-8 texts too) |

## Array

| JavaScript | Rythme |
|---|---|
| `a.length` | `len(a)` |
| `a.at(i)`, `a[i]` | `get(a, i)` (an `Option`; `-1` is the last), `a[i]` |
| `a.map(f)`, `filter(f)`, `forEach(f)` | `map`, `filter`, `each` |
| `a.reduce(f, init)` | `fold(a, init, f)` |
| `a.reduceRight(f, init)` | `fold(reverse(a), init, f)` |
| `a.find(f)`, `findIndex(f)` | `find`, `find_index` (an `Option`) |
| `a.findLast(f)`, `findLastIndex(f)` | **new** `find_last`, **new** `find_last_index` |
| `a.some(f)`, `a.every(f)` | `any`, `all` |
| `a.includes(x)`, `indexOf(x)`, `lastIndexOf(x)` | `contains`, `index_of`, `last_index_of` |
| `a.flat()`, `flatMap(f)` | `flatten`, `flat_map` |
| `a.slice(i, j)` | `slice(a, i, j)` |
| `a.concat(b)`, `[...a, ...b]` | `a + b`, `[..a, ..b]` |
| `a.toSorted()`, `a.sort(cmp)` | `sort`, `sort_by(a, key)`, `sort_desc`, `sort_by_desc` |
| `a.toReversed()`, `a.reverse()` | `reverse` |
| `a.with(i, x)` | `set_at(a, i, x)` |
| `a.toSpliced(i, n, ...xs)` | `remove_at`, `insert_at`, `slice` and `+` |
| `a.push(x)`, `a.unshift(x)` | `push(a, x)`, `[x, ..a]`: new lists |
| `a.pop()`, `a.shift()` | `last(a)`/`slice(a, 0, -1)`, `first(a)`/`rest(a)` |
| `a.keys()`, `a.entries()` | `0..len(a) - 1`, `enumerate(a)` |
| `Array.from({length: n}, f)`, `Array(n).fill(x)` | `map(0..n - 1, f)`, `repeat([x], n)` |
| `Array.isArray(x)` | `type_of(x) == "List"` |
| `Object.groupBy(a, f)` | `group_by(a, f)` |
| `a.join(", ")` | `join(a, ", ")` |
| `a.fill`, `copyWithin`, `splice` (in place) | *absent*: values never change; see `toSpliced` |

## Object and Map

| JavaScript | Rythme |
|---|---|
| `{a: 1}`, `obj.a`, `obj["a"]` | records `{a: 1}`, `r.a`; dicts `dict([["a", 1]])`, `get(d, "a")` |
| `Object.keys`, `values`, `entries`, `fromEntries` | `keys`, `values`, `entries`, `from_entries` |
| `Object.hasOwn(o, k)`, `map.has(k)`, `k in o` | `has(x, k)` |
| `{...a, ...b}`, `Object.assign({}, a, b)` | `{..a, ..b}`, `union(a, b)` for dicts |
| `delete o.k`, `map.delete(k)` | `remove(x, k)` (a new value) |
| `new Map()`, `map.set(k, v)`, `map.get(k)`, `map.size` | `dict()`, `put(d, k, v)`, `get(d, k)`, `len(d)` |
| `structuredClone(x)`, `Object.freeze(x)` | *absent*: values never change, a copy is the value itself |
| optional chaining `a?.b?.c` | `dig(a, ["b", "c"])` |

## Set

| JavaScript | Rythme |
|---|---|
| `new Set(a)`, `s.has(x)`, `s.size` | `set(a)`, `has(s, x)`, `len(s)` |
| `s.add(x)`, `s.delete(x)` | `union(s, set([x]))`, `remove(s, x)` |
| `union`, `intersection`, `difference`, `symmetricDifference` | the same names in snake_case |
| `s.isSubsetOf(t)`, `s.isSupersetOf(t)` | `is_subset(s, t)`, `is_subset(t, s)` |
| `s.isDisjointFrom(t)` | `is_disjoint(s, t)` |

## JSON

| JavaScript | Rythme |
|---|---|
| `JSON.parse(s)` | `parse_json(s)`: `Ok(value)` or `Err(message)` |
| `JSON.stringify(x)`, `JSON.stringify(x, null, 2)` | `to_json(x)`, `to_json(x, 2)` |

## Date

| JavaScript | Rythme |
|---|---|
| `new Date()`, `Date.now()` | `now()` (a record `{year, month, day, hour, ...}`), `time()` (seconds) |
| `d.getFullYear()`, `getMonth()`, `getDate()`, `getHours()` | `d.year`, `d.month` (1 to 12), `d.day`, `d.hour` |
| `d.getDay()` | `day_of_week(d)` |
| `new Date("2024-02-29")`, `Date.parse` | `parse_date("2024-02-29")`: a `Result` |
| `d.toISOString()`, `toLocaleDateString` | `format_date(d)`, `format_date(d, "%d/%m/%Y")` |
| `d.setDate(d.getDate() + n)` | `add_days(d, n)`, `add_months`, `add_seconds` |
| `(b - a) / 86400000` | `days_between(a, b)`, `seconds_between(a, b)` |

## RegExp

| JavaScript | Rythme |
|---|---|
| `/re/.test(s)` | `matches(s, "re")` |
| `s.match(/re/)`, `/re/.exec(s)` | `find_first`, `captures` |
| `s.matchAll(/re/g)` | `find_all`, `captures_all` |
| named groups `(?<year>\d+)` | `named_captures` (a dict) |
| `s.split(/re/)` | `split_by(s, "re")` |
| `s.replace(/re/g, ...)` | `replace_all(s, "re", ...)` |

## Promise, timers and console

| JavaScript | Rythme |
|---|---|
| `async function f()`, `await p` | `async fn f()`, `await task` (tasks run on other cores) |
| `Promise.all(ps)` | `await_all(tasks)` |
| `Promise.allSettled`, `Promise.any`, `Promise.race` | *absent*: a task returns a `Result` when it may fail, and `await_all` gives every result |
| `setTimeout(f, ms)`, `await new Promise(r => setTimeout(r, ms))` | **new** `sleep(seconds)` (waits, then the program goes on) |
| `setInterval` | *absent*: no callbacks; `each` over a list, with `sleep` between |
| `console.log`, `console.error`, `console.debug` | `print`, `print_error`, `debug` |
| `console.assert` | `assert`, `assert_eq` |

## URL, crypto and encodings

| JavaScript | Rythme |
|---|---|
| `new URL(s)` (`protocol`, `hostname`, `port`, `pathname`, `search`, `hash`) | **new** `parse_url(s)`: `Ok({scheme, user, host, port, path, query, fragment})` or `Err(message)` |
| `new URLSearchParams(s)`, `params.get(k)` | **new** `parse_query(s)`: a dict of texts, then `get(d, k)` |
| `new URLSearchParams(obj).toString()` | **new** `format_query(x)`: a record or dict as `a=1&b=x%20y` |
| `crypto.randomUUID()` | **new** `uuid()`: a random UUID (version 4); `random_seed` makes it repeatable |
| `crypto.subtle.digest("SHA-256", data)` | **new** `sha256(text)`: the hash in hexadecimal |
| `fetch(url)` | `http_get(url)`, `http_post(url, body)`: a `Result` (in an `async fn` to run several at once) |
| `process.argv`, `process.env`, `process.exit` | `args()`, `env(name)`, `exit(code)` |
| `fs.readFileSync`, `writeFileSync` | `read_file`, `write_file`: a `Result` |

## What else differs

| JavaScript | Rythme |
|---|---|
| `let`, `const`, `var` | `x = 1`: a name for a value that never changes |
| `x += 1`, `i++` | a new name: `total2 = total + 1` |
| `for`, `while` | `each`, `map`, `fold`, or a function that calls itself (tail calls do not grow the stack) |
| `if (...) {} else {}`, `c ? a : b` | `if c then a else b`: an expression, always with `else` |
| `switch` | `match value ... end`, with patterns |
| `try`/`catch`, `throw` | `Result` values, `?` to pass an error on |
| `class`, `new` | records and types: `type Shape = Circle(r) \| Square(s)` |
| `===`, `!==` | `==`, `!=`, never with conversions: `1 == "1"` is `false` |
| `import`, `export` | `use geometry`; names starting with `_` stay private |
| arrow functions `x => x * 2` | `fn(x) = x * 2`, or a block `do \|x\| x * 2 end` |
