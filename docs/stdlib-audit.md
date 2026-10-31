# The standard library, compared

*En français : l'inventaire de la bibliothèque standard de Rythme, comparée à
celles de Ruby, Python et Elixir : ce qui existe, ce qui a été ajouté après la
1.0, et ce qui manque exprès. Le manuel ([6](reference.fr.md#6-bibliothèque-standard))
décrit chaque fonction.*

What students and professionals expect from Ruby's core (String, Array,
Hash, Enumerable, Integer and Float, Comparable, Range, Time), Python's
builtins, `itertools`, `statistics` and `string`, and Elixir's `Enum`,
`String` and `Map`, against what Rythme has. This audit was made after 1.0:
the functions marked **new** were added then (72 functions, 26 Ruby names);
`rythme help NAME` documents each one, with an example that the tests run.

Legend: a name is the Rythme function; **new** marks one added after 1.0;
*absent* says why Rythme has none on purpose. Rythme is purely functional:
nothing changes a value in place, so `push!`, `pop`, `shift`, `sort!`,
`list.append` have no equivalent, and every function below returns a new
value. Iterators, generators and laziness are absent too: lists are built
whole, bounded (E023 beyond 50 million elements).

## Text (Ruby String, Python str and string, Elixir String)

| Operation | Ruby | Python | Elixir | Rythme |
|---|---|---|---|---|
| length | `length`, `size` | `len` | `length` | `len` (and `length`, `size`) |
| upper, lower case | `upcase`, `downcase` | `upper`, `lower` | `upcase`, `downcase` | `upper`, `lower` (and `upcase`, `downcase`) |
| first letter in uppercase | `capitalize` | `capitalize` | `capitalize` | `capitalize` |
| each word capitalized | (Rails `titleize`) | `title`, `string.capwords` | | **new** `capitalize_words` |
| swap the case | `swapcase` | `swapcase` | | **new** `swap_case` (and `swapcase`) |
| snake, camel, kebab case | (Rails `underscore`, `camelize`, `dasherize`) | | (Macro.underscore) | **new** `snake_case`, `camel_case`, `kebab_case` |
| trim | `strip`, `lstrip`, `rstrip` | `strip`, `lstrip`, `rstrip` | `trim`, `trim_leading`, `trim_trailing` | `trim`, `trim_start`, `trim_end` (and `strip`, `lstrip`, `rstrip`) |
| pad, align | `ljust`, `rjust`, `center` | `ljust`, `rjust`, `center`, `zfill` | `pad_leading`, `pad_trailing` | `pad_left`, `pad_right`, **new** `center` (and `ljust`, `rjust`) |
| format with width and precision | `format`, `%` | `format`, f-strings | (`:io_lib.format`) | **new** `format(value, spec)` (Python's specs), `text(x, decimals)`, `format_number`, `"{x}"` |
| split, join | `split`, `join` | `split`, `join` | `split`, `Enum.join` | `split`, `join` |
| split once | `partition` | `partition`, `split(sep, 1)` | `split(s, sep, parts: 2)` | **new** `split_once` |
| lines, words, characters, bytes | `lines`, `split`, `chars`, `bytes` | `splitlines`, `split`, `list`, `encode` | `split`, `graphemes`, `codepoints` | `lines`, `words`, `chars`, **new** `bytes` |
| contains, starts, ends | `include?`, `start_with?`, `end_with?` | `in`, `startswith`, `endswith` | `contains?`, `starts_with?`, `ends_with?` | `contains`, `starts_with`, `ends_with` (and Ruby's names) |
| first, last position | `index`, `rindex` | `find`, `rfind`, `index` | `:binary.match` | `index_of`, **new** `last_index_of` (and `rindex`) |
| count a piece | `count` (characters) | `count` | | **new** `occurrences` |
| replace all, first | `gsub`, `sub` | `replace`, `replace(old, new, 1)` | `replace`, `replace(global: false)` | `replace`, **new** `replace_first`, `replace_all` (regex) |
| remove a prefix, suffix | `delete_prefix`, `delete_suffix` | `removeprefix`, `removesuffix` | `trim_leading(s, p)` | **new** `remove_prefix`, `remove_suffix` (and Ruby's names) |
| repeat, reverse | `*`, `reverse` | `*`, `[::-1]` | `duplicate`, `reverse` | `repeat`, `reverse` |
| character and code | `ord`, `chr` | `ord`, `chr` | `?a`, `<<c>>` | `char_code`, `char` (and `ord`, `chr`) |
| tests on characters | `match?(/\d/)` | `isdigit`, `isalpha`, `isspace`, `isupper`, `islower` | | `is_digit`, `is_letter`, `is_space`, **new** `is_upper`, `is_lower` |
| compare ignoring the case | `casecmp?` | `casefold` | | `equals_ignore_case` (and `casecmp?`) |
| wrap a paragraph | (Rails `word_wrap`) | `textwrap.fill` | | **new** `word_wrap` |
| distance between words | (DidYouMean) | (difflib) | `jaro_distance` | **new** `edit_distance` |
| regular expressions | `=~`, `scan`, `match` | `re` | `Regex` | `matches`, `find_first`, `find_all`, `captures`, `captures_all`, `named_captures`, `split_by`, `replace_all` |
| text to number, with errors | `Integer(s)`, `Float(s)` | `int(s)`, `float(s)` | `Integer.parse` | `parse_int`, `parse_float` (Results), `int`, `float` |
| any base | `to_s(2)`, `to_i(16)` | `bin`, `hex`, `int(s, 16)` | `Integer.to_string(n, 2)` | **new** `to_base`, `from_base` (Result) |
| code form of a value | `inspect` | `repr` | `inspect` | **new** `inspect` |
| JSON, CSV | `JSON`, `CSV` | `json`, `csv` | (Jason) | `parse_json`, `to_json`, `csv_parse`, `csv_records`, `csv_format` |
| Base64, URL encoding | `Base64`, `CGI.escape` | `base64`, `urllib.parse.quote` | `Base.encode64`, `URI.encode` | **new** `base64_encode`, `base64_decode`, `url_encode`, `url_decode` |
| constants of characters | | `string.ascii_letters`, `digits` | | *absent*: `chars("abc...")` is as short |
| mutation in place | `<<`, `upcase!` | | | *absent*: values never change |

## Numbers (Ruby Integer, Float, Comparable; Python builtins and math; Elixir Integer, Float)

| Operation | Ruby | Python | Elixir | Rythme |
|---|---|---|---|---|
| absolute value, min, max | `abs`, `min`, `max` | `abs`, `min`, `max` | `abs`, `min`, `max` | `abs`, `min`, `max` |
| rounding | `round(n)`, `floor`, `ceil`, `truncate` | `round`, `math.floor`, `math.ceil`, `math.trunc` | `round`, `floor`, `ceil`, `trunc` | `round(x, digits)`, `floor`, `ceil`, `int` |
| whole division, remainder, both | `/`, `%`, `divmod` | `//`, `%`, `divmod` | `div`, `rem` | `//`, `%`, **new** `divmod` |
| powers, roots, logarithms | `**`, `Math.sqrt`, `Math.log` | `**`, `pow`, `math.sqrt`, `math.log` | `:math` | `**`, `pow`, `sqrt`, `log(x, base)`, `exp` |
| trigonometry | `Math.sin`... | `math.sin`... | `:math.sin`... | `sin`, `cos`, `tan`, `asin`, `acos`, `atan(y, x)`, `pi` |
| gcd, lcm | `gcd`, `lcm` | `math.gcd`, `math.lcm` | `Integer.gcd` | `gcd`, `lcm` (any size) |
| keep between two bounds | `clamp`, `between?` | | | **new** `constrain` (and `clamp`) |
| even, odd, sign tests | `even?`, `odd?`, `positive?`, `negative?`, `zero?` | `n % 2` | `Integer.is_even` | `is_even`, `is_odd`, **new** `is_positive`, `is_negative`, `is_zero` (and `even?`, `odd?`, `positive?`, `negative?`, `zero?`) |
| not a number, finite | `nan?`, `finite?`, `infinite?` | `math.isnan`, `math.isfinite` | | **new** `is_nan`, `is_finite` (and `nan?`, `finite?`) |
| digits | `digits` (reversed) | `map(int, str(n))` | `Integer.digits` | **new** `digits(n, base)` (most significant first) |
| binomial coefficient | | `math.comb` | | **new** `binomial` (exact) |
| factorial | | `math.factorial` | | *absent*: `product(1..n)`, and a first exercise of recursion |
| primality | `Prime.prime?` | | | *absent*: an exercise ("divisors"); the examples write it |
| whole numbers of any size | `Integer` | `int` | `integer` | yes, up to 100000 digits |
| sign as a number | `<=>` with 0 | `math.copysign` | | *absent*: `if x < 0 ...`, and a name many programs define |
| random, with a seed | `rand`, `srand` | `random`, `random.seed` | `:rand` | `random`, `random_seed`, `shuffle`, `random_normal`, `random_choice_weighted` |

## Lists (Ruby Array and Enumerable, Python list and itertools, Elixir Enum and List)

| Operation | Ruby | Python | Elixir | Rythme |
|---|---|---|---|---|
| map, filter, reject, reduce | `map`, `select`, `reject`, `reduce` | `map`, `filter`, comprehension, `functools.reduce` | `map`, `filter`, `reject`, `reduce` | `map`, `filter`, `reject`, `fold` (and `collect`, `select`, `reduce`, `inject`) |
| running fold | `each_with_object` | `itertools.accumulate` | `scan` | `scan` |
| filter and map at once | `filter_map` | comprehension | `flat_map` | **new** `filter_map` |
| flat map, flatten | `flat_map`, `flatten` | `itertools.chain` | `flat_map`, `List.flatten` | `flat_map`, `flatten` |
| find, find the position | `find`, `find_index` | `next(...)`, `index` | `find`, `find_index` | `find`, **new** `find_index`, `index_of`, **new** `last_index_of` (and `detect`) |
| any, all, count | `any?`, `all?`, `count` | `any`, `all`, `sum(1 for ...)` | `any?`, `all?`, `count` | `any`, `all`, `count`, **new** `occurrences` |
| sum, product | `sum`, `inject(:*)` | `sum`, `math.prod` | `sum`, `product` | `sum(list, f)`, `product` |
| min, max by a key | `min_by`, `max_by`, `minmax` | `min(key=)`, `max(key=)` | `min_by`, `max_by` | `min_by`, `max_by`, `argmin`, `argmax` |
| sort, by a key, from the largest | `sort`, `sort_by`, `sort.reverse` | `sorted(key=, reverse=)` | `sort`, `sort_by`, `sort(:desc)` | `sort`, `sort_by`, **new** `sort_desc`, `sort_by_desc` |
| sort with a comparison | `sort { \|a, b\| }` | `cmp_to_key` | `sort(fun)` | *absent*: a key (`sort_by`) covers it, lists compare in order |
| unique, by a key | `uniq`, `uniq { }` | `dict.fromkeys` | `uniq`, `uniq_by` | `unique`, **new** `unique_by` (and `uniq`) |
| count each value | `tally`, `tally_by` | `collections.Counter` | `frequencies`, `frequencies_by` | **new** `tally`, `count_by`, `most_common` |
| group | `group_by`, `partition` | `itertools.groupby` (runs) | `group_by`, `split_with` | `group_by`, `partition`, **new** `chunk_by` (runs) |
| first, last, rest | `first`, `last`, `drop(1)` | `[0]`, `[-1]`, `[1:]` | `List.first`, `List.last`, `tl` | `first`, `last`, `rest`, `get` (Options) |
| take, drop, while | `take`, `drop`, `take_while`, `drop_while` | `[:n]`, `itertools.takewhile` | `take`, `drop`, `take_while` | `take`, `drop`, `take_while`, `drop_while`, `slice` |
| split at a position | `take` + `drop` | `[:i]`, `[i:]` | `split` | **new** `split_at` |
| slices, windows | `each_slice`, `each_cons` | `itertools.batched`, `pairwise` | `chunk_every` | `chunks`, `windows` (and `each_slice`, `each_cons`) |
| every n-th | `each_slice(n).map(&:first)` | `[::n]` | `take_every` | `step_by` |
| zip, unzip, more lists | `zip`, `transpose` | `zip`, `zip(*rows)` | `zip`, `unzip` | `zip` (**new**: two lists or more), **new** `unzip`, `zip_with`, `enumerate` |
| insert, remove, replace at | `insert`, `delete_at`, `[]=` | `insert`, `pop(i)`, `[i] =` | `List.insert_at`, `delete_at`, `replace_at`, `update_at` | `insert_at`, `remove_at`, `set_at`, **new** `update_at` |
| rotate | `rotate` | `deque.rotate` | | **new** `rotate_by` (and `rotate`) |
| between, alternate | `zip.flatten` | | `intersperse` | **new** `intersperse`, `interleave` |
| combinations, permutations, product | `combination`, `permutation`, `product` | `itertools.combinations`, `permutations`, `product` | | **new** `combinations`, `permutations`, `cartesian_product` (and `combination`, `permutation`) |
| repeat, cycle | `*`, `cycle` | `*`, `itertools.cycle` | `List.duplicate` | `repeat` |
| sequences from a function | `Enumerator.produce` | generators | `Stream.iterate`, `Stream.unfold` | **new** `iterate(start, n, f)`, `unfold(start, f)` (finite) |
| ranges | `1..10`, `step` | `range` | `1..10//2` | `1..10`, `step_by`, `reverse(1..10)`, **new** `linspace` (Floats) |
| shuffle, sample | `shuffle`, `sample` | `random.shuffle`, `choice` | `shuffle`, `random` | `shuffle`, `random(list)` |
| `none?`, `one?` | `none?`, `one?` | `not any` | | *absent*: `not any(xs, f)`; `none` would read like `None` |
| `pop`, `push!`, `shift` | in place | in place | | *absent*: `push` and `rest` give new lists |
| laziness | `lazy` | iterators | `Stream` | *absent*: lists are whole and bounded |

## Hashes, dictionaries and maps (Ruby Hash, Python dict, Elixir Map)

| Operation | Ruby | Python | Elixir | Rythme |
|---|---|---|---|---|
| build | `{}`, `to_h` | `dict`, `{}` | `%{}`, `Map.new` | records `{a: 1}`, `dict(pairs)` (and `to_h`), `from_entries` |
| get, with a default | `[]`, `fetch`, `dig` | `[]`, `get` | `Map.get`, `get_in` | `get` (an Option), `unwrap_or(get(...), d)`, **new** `dig` |
| put, remove, has | `[]=`, `delete`, `key?` | `[k] =`, `pop`, `in` | `put`, `delete`, `has_key?` | `put`, `remove`, `has` (and `key?`) |
| keys, values, pairs | `keys`, `values`, `to_a` | `keys`, `values`, `items` | `keys`, `values`, `to_list` | `keys`, `values`, `entries` |
| change the values, the keys | `transform_values`, `transform_keys` | comprehension | `Map.new` | `map_values`, **new** `map_keys` (and Ruby's names) |
| change one value with a default | `h[k] = h.fetch(k, d) + 1` | `d[k] = d.get(k, 0) + 1` | `Map.update` | **new** `update(x, key, default, f)` |
| merge, with a function | `merge`, `merge { }` | `\|`, `update` | `Map.merge(a, b, fun)` | `{..a, ..b}`, `union`, **new** `merge_with` |
| keep, leave out keys | `slice`, `except` | comprehension | `Map.take`, `Map.drop` | **new** `select_keys`, `reject_keys` |
| invert | `invert` | comprehension | | **new** `invert` |
| filter, each | `select`, `each_pair` | comprehension | `filter`, `Enum.each` | `filter`, `reject`, `each` (and `each_pair`) |
| default values | `Hash.new(0)` | `defaultdict` | | *absent*: `update` with a default |

## Sets (Ruby Set, Python set, Elixir MapSet)

| Operation | Ruby | Python | Elixir | Rythme |
|---|---|---|---|---|
| build, add, remove, has | `Set.new`, `add`, `delete`, `include?` | `set`, `add`, `discard`, `in` | `MapSet.new`, `put`, `delete`, `member?` | `set`, `push`, `remove`, `has` |
| union, intersection, difference | `\|`, `&`, `-` | `\|`, `&`, `-` | `union`, `intersection`, `difference` | `union`, `intersection`, `difference` |
| symmetric difference | `^` | `^` | `symmetric_difference` | **new** `symmetric_difference` |
| subset, disjoint | `subset?`, `disjoint?` | `issubset`, `isdisjoint` | `subset?`, `disjoint?` | `is_subset`, **new** `is_disjoint` (and `subset?`, `disjoint?`) |

## Option and Result (Rust's names, the habits of Elixir's `{:ok, x}`)

| Operation | Rust | Elixir | Rythme |
|---|---|---|---|
| test the variant | `is_some`, `is_ok` | `match?` | `is_some`, `is_none`, `is_ok`, `is_err` |
| unwrap, with a default, lazily, with a message | `unwrap`, `unwrap_or`, `unwrap_or_else`, `expect` | `case` | `unwrap`, `unwrap_or`, **new** `unwrap_or_else`, **new** `expect` |
| chain, change the value, the error | `and_then`, `map`, `map_err` | `with` | `and_then`, `map`, `map_err`, `?` |
| Option to Result | `ok_or` | | `ok_or` |
| a list of Results into one | `collect::<Result<Vec<_>, _>>()` | `Enum.reduce_while` | **new** `all_ok` |
| keep the `Some` values | `filter_map`, `flatten` | | **new** `filter_map` |
| `transpose`, `zip`, `or_else` | `transpose`, `zip`, `or_else` | | *absent*: `match` is clearer for a first course |

## Dates and times (Ruby Time and Date, Python datetime, Elixir Date and DateTime)

| Operation | Ruby | Python | Elixir | Rythme |
|---|---|---|---|---|
| now, today | `Time.now`, `Date.today` | `datetime.now`, `date.today` | `DateTime.utc_now`, `Date.utc_today` | `now`, `today` (records) |
| ISO 8601 | `iso8601`, `Time.iso8601` | `isoformat`, `fromisoformat` | `to_iso8601`, `from_iso8601` | `format_date`, `parse_date` |
| patterns | `strftime`, `strptime` | `strftime`, `strptime` | `Calendar.strftime` | **new** `format_date(date, pattern)`, `parse_date(text, pattern)` |
| add days, seconds, months | `+`, `>>` | `timedelta`, (`relativedelta`) | `add`, `shift` | `add_days`, `add_seconds`, **new** `add_months` |
| differences | `-` | `-` | `diff` | `days_between`, `seconds_between` |
| calendar | `wday`, `yday`, `leap?` | `weekday`, `isocalendar` | `day_of_week`, `day_of_year`, `leap_year?` | `day_of_week`, **new** `day_of_year`, `days_in_month`, `is_leap_year` |
| time zones | `TZInfo` | `zoneinfo` | `Tzdata` | *absent*: an offset only, no database (no dependency) |

## Statistics (Python statistics, numpy's basics)

| Operation | Python | Rythme |
|---|---|---|
| mean, median, standard deviation, variance | `mean`, `median`, `pstdev`, `pvariance` | `stats(xs)` (count, mean, median, std_dev, min, max), `variance`, `std_dev` |
| quantiles | `quantiles` | `percentile` |
| mode | `mode` | `most_common(xs, 1)` |
| correlation, covariance | `correlation`, `covariance` | **new** `correlation`, `covariance` |
| evenly spaced numbers | `numpy.linspace` | **new** `linspace` |
| vectors and matrices | `numpy` | `dot`, `norm`, `matmul`, `transpose`, `vector_add`... (see `rythme help ml`) |
| `mean` and `median` as functions | `mean`, `median` | *absent*: names that the exercises ask to write; `stats(xs).mean` |

## Input, output and the system

| Operation | Ruby | Python | Elixir | Rythme |
|---|---|---|---|---|
| print, to the error output | `puts`, `warn` | `print`, `print(file=sys.stderr)` | `IO.puts`, `IO.warn` | `print`, **new** `print_error` (and `puts`, `warn`), `debug` |
| read a line, all lines | `gets`, `readlines` | `input`, `sys.stdin` | `IO.gets`, `IO.stream` | `input`, `input_lines` |
| files and folders | `File`, `Dir` | `open`, `os`, `pathlib` | `File` | `read_file`, `write_file`, `append_file`, `file_exists`, `file_info`, `list_folder`, `create_folder`, `delete_file`, `join_path`, `split_path` |
| arguments, environment, exit | `ARGV`, `ENV`, `exit` | `sys.argv`, `os.environ`, `sys.exit` | `System.argv`, `System.get_env`, `System.halt` | `args`, `options`, `env`, `exit` |
| HTTP | `Net::HTTP` | `urllib.request` | (Req) | `http_get`, `http_post` |
| sleep, processes, threads, sockets | `sleep`, `system`, `Thread` | `time.sleep`, `subprocess`, `threading` | `Process`, `Task` | *absent*: a language for learning to compute; effects stay few and visible |
| hashes (MD5, SHA) | `Digest` | `hashlib` | `:crypto` | *absent* for now: a pure function, a candidate for a later version |

## What the tools gained

Outside of the library, the same audit looked at the `rythme` command
itself: `rythme CMD --help` for every command in the six languages, `rythme
run -` (the program on the standard input), `rythme check FOLDER`, `rythme
completions bash|zsh|fish|powershell`, `rythme man`, `rythme version
--verbose` and `rythme upgrade` were added. `rythme fmt --check` already had
its exit codes (0 formatted, 1 to format), and `rythme check` already
checked a whole project from its root.
