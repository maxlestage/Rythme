// Japanese: each English text of Rythme with its translation. How to add or
// update a language: CONTRIBUTING.md, "Messages in every language".
Language {
    code: "ja",
    name: "日本語",
    texts: &[
        // src/main.rs
        ("unknown language `{v}`", "不明な言語 `{v}`"),
        (
            r#"Rythme ran into an internal problem. This is a bug in Rythme, not in your program.
Please report it, with your program, at https://github.com/maxlestage/Rythme/issues"#,
            r#"Rythme で内部の問題が起きました。これは Rythme のバグで、あなたのプログラムの誤りではありません。
プログラムを添えて https://github.com/maxlestage/Rythme/issues に報告してください"#,
        ),
        ("`--lang` needs a language", "`--lang` には言語が必要"),
        ("{what}: write `--lang` and one of {codes}, like `rythme --lang fr hello.ry`", "{what}：`--lang` の後に {codes} のどれかを書く（例：`rythme --lang ja hello.ry`）"),
        ("usage: rythme explain CODE   (codes: {})", "使い方：rythme explain CODE   （コード：{}）"),
        ("unknown code `{code}`. Known codes: {}", "不明なコード `{code}`。既知のコード：{}"),
        ("which file? Example: rythme run hello.ry", "どのファイル？ 例：rythme run hello.ry"),
        ("which file? Example: rythme check hello.ry", "どのファイル？ 例：rythme check hello.ry"),
        ("which code? Example: rythme -e 'print(1 + 2)'", "どのコード？ 例：rythme -e 'print(1 + 2)'"),
        ("unknown command or file `{cmd}`", "不明なコマンドまたはファイル `{cmd}`"),
        (
            r#"
did you mean `rythme {best}`?"#,
            r#"
もしかして `rythme {best}`？"#,
        ),
        (
            r#"
see `rythme help`"#,
            r#"
`rythme help` を参照"#,
        ),
        ("current folder: {}", "現在のフォルダー：{}"),
        ("Rythme files usually end with `.ry` ({})", "Rythme のファイル名は普通 `.ry` で終わる（{}）"),
        ("✓ {shown}: no problems found", "✓ {shown}：問題なし"),
        ("✓ {shown}: no errors, {n} warning(s)", "✓ {shown}：エラーなし、警告 {n} 件"),
        ("✗ {shown}: {n} error(s)", "✗ {shown}：エラー {n} 件"),
        // src/bigint.rs
        ("this whole number would be too large (more than {MAX_DIGITS} digits)", "この整数は大きすぎる（{MAX_DIGITS} 桁を超える）"),
        ("does it grow without end, in a function that calls itself? For an approximate value, use a decimal number (Float), like `2.0 ** 1000`", "自分自身を呼ぶ関数の中で、際限なく大きくなっていないか？ 近似値でよければ小数（Float）を使う。例：`2.0 ** 1000`"),
        // src/builtins/date.rs
        ("the year must go from 1 to 9999, not {year}", "年は 1 から 9999 までで、{year} は範囲外"),
        ("the month must go from 1 to 12, not {month}", "月は 1 から 12 までで、{month} は範囲外"),
        ("the day must go from 1 to {len} in {name} {year}, not {day}", "{year}年{name}の日は 1 から {len} までで、{day} は範囲外"),
        (" ({year} is not a leap year)", "（{year} 年はうるう年ではない）"),
        ("the hour must go from 0 to 23, not {hour}", "時は 0 から 23 までで、{hour} は範囲外"),
        ("the minute must go from 0 to 59, not {minute}", "分は 0 から 59 までで、{minute} は範囲外"),
        ("the second must go from 0 to 59, not {second}", "秒は 0 から 59 までで、{second} は範囲外"),
        ("the offset must go from -1080 to 1080 minutes (-18:00 to +18:00), not {offset}", "UTC との時差は -1080 から 1080 分（-18:00 から +18:00）までで、{offset} は範囲外"),
        ("a date has no field `{k}`", "日付にフィールド `{k}` はない"),
        (" (did you mean `{best}`?)", "（もしかして `{best}`？）"),
        ("the field `{k}` is too large for a date", "フィールド `{k}` は日付には大きすぎる"),
        ("the field `{k}` must be a whole number, not {}", "フィールド `{k}` は整数でなければならないが、{} だった"),
        ("the field `{k}` is missing", "フィールド `{k}` がない"),
        ("\"{text}\" is not a date written as ISO 8601: write it like \"2024-02-29\" or \"2024-02-29T13:05:00\"", "\"{text}\" は ISO 8601 形式の日付ではない。\"2024-02-29\" や \"2024-02-29T13:05:00\" のように書く"),
        ("\"{text}\" is not a valid date: {why}", "\"{text}\" は有効な日付ではない：{why}"),
        ("`{f}` needs a valid date: {why}", "`{f}` には有効な日付が必要：{why}"),
        ("not a valid date", "有効な日付ではない"),
        ("a date is a record like `{{year: 2024, month: 2, day: 29}}`, with `hour`, `minute`, `second` and `offset` (minutes ahead of UTC) if needed", "日付は `{{year: 2024, month: 2, day: 29}}` のようなレコードで、必要なら `hour`、`minute`、`second`、`offset`（UTC より進んでいる分数）を加える"),
        ("`{f}` gives a date outside of the years 1 to 9999", "`{f}` の結果が 1〜9999 年の範囲外の日付になる"),
        ("the month must go from 1 to 12, not {m}", "月は 1 から 12 までで、{m} は範囲外"),
        ("the clock of the system gives a date outside of the years 1 to 9999", "システムの時計が 1〜9999 年の範囲外の日付を返している"),
        // src/builtins/dicts.rs
        ("`{f}`: {} cannot be a key of a dict, nor an element of a set", "`{f}`：{} は辞書のキーにも集合の要素にもできない"),
        ("it holds {}", "{} を含んでいる"),
        ("keys and elements must be values that can be compared: anything but functions and `nan`", "キーと要素は比較できる値でなければならない：関数と `nan` 以外なら何でもよい"),
        ("`{f}` needs a list of [key, value] pairs, but it contains {}", "`{f}` には [キー, 値] のペアのリストが必要だが、{} が含まれている"),
        ("each pair is a list of two elements, like `dict([[\"Ada\", 36], [\"Alan\", 41]])`", "各ペアは 2 つの要素のリスト。例：`dict([[\"Ada\", 36], [\"Alan\", 41]])`"),
        // src/builtins/property.rs
        ("this is not a generator: {}", "これはジェネレーターではない：{}"),
        ("not a generator", "ジェネレーターではない"),
        ("make one with `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` or `records_of({{x: g}})`", "`ints()`、`floats()`、`texts()`、`bools()`、`lists_of(g)`、`one_of(list)`、`records_of({{x: g}})` で作る"),
        ("generators can be put inside each other {MAX_DEPTH} times at most", "ジェネレーターの入れ子は最大 {MAX_DEPTH} 段まで"),
        ("its field `{what}` is missing or wrong", "フィールド `{what}` がないか、間違っている"),
        ("`{f}` takes no argument, or two: the smallest and the largest value, like `{f}(0, 10)`", "`{f}` は引数を取らないか、2 つ（最小値と最大値）取る。例：`{f}(0, 10)`"),
        ("{f}({}, {}): the minimum is larger than the maximum", "{f}({}, {})：最小値が最大値より大きい"),
        ("`floats` needs finite numbers, not {} and {}", "`floats` には有限の数値が必要（{} と {} は不可）"),
        ("`{f}` needs a length from 0 to {MAX_LENGTH}, not {n}", "`{f}` の長さは 0 から {MAX_LENGTH} までで、{n} は範囲外"),
        ("`one_of` needs at least one value to choose from", "`one_of` には選ぶ値が少なくとも 1 つ必要"),
        ("the field `{k}` given to `records_of` must be a generator, like `ints()`, not {}", "`records_of` に渡すフィールド `{k}` は `ints()` のようなジェネレーターでなければならないが、{} だった"),
        ("`check_all` tries from 1 to {MAX_CASES} values, not {cases}", "`check_all` が試す値の数は 1 から {MAX_CASES} までで、{cases} は範囲外"),
        ("the property is false for {v}", "プロパティが {v} で偽になる"),
        ("false for {v}", "{v} で偽"),
        ("the property gives {r} for {v}", "プロパティが {v} で {r} を返す"),
        ("{r} for {v}", "{v} で {r}"),
        ("the property fails for {v}: {}", "プロパティが {v} で失敗する：{}"),
        ("checked here", "ここでチェック"),
        ("found on try {case} of {cases}", "{cases} 回中 {case} 回目の試行で見つかった"),
        ("found on try {case} of {cases}, then simplified from {} in {steps} step(s)", "{cases} 回中 {case} 回目の試行で見つかり、{} から {steps} ステップで単純化された"),
        ("seed {seed}: every run tries the same values; `rythme test --seed N` tries others", "シード {seed}：毎回同じ値を試す。`rythme test --seed N` で別の値を試せる"),
        ("the function must hold for every value of the generator: fix the function, or give the generator a smaller range, like `ints(1, 100)`", "関数はジェネレーターのすべての値で成り立たなければならない：関数を直すか、`ints(1, 100)` のようにジェネレーターの範囲を狭める"),
        // src/builtins/regex.rs
        ("for the sign `{c}` itself, write `[{c}]` (in a Rythme text: \"[\\{c}]\")", "記号 `{c}` そのものには `[{c}]` と書く（Rythme のテキストでは \"[\\{c}]\"）"),
        ("for the sign `{c}` itself, write `[{c}]`", "記号 `{c}` そのものには `[{c}]` と書く"),
        ("`{q}` has nothing to repeat", "`{q}` の前に繰り返すものがない"),
        ("nothing before it to repeat", "前に繰り返すものがない"),
        ("put it after a character or a group, like `a{q}` or `(ab){q}`; {}", "`a{q}` や `(ab){q}` のように、文字かグループの後に置く。{}"),
        ("`{{` must start a count, like {{3}} or {{2,5}}", "`{{` は {{3}} や {{2,5}} のような回数指定の始まりでなければならない"),
        ("not a count", "回数指定ではない"),
        ("{}; in a Rythme text, braces are written `\\{{` and `\\}}`, like \"[0-9]\\{{4\\}}\"", "{}。Rythme のテキストでは波かっこを `\\{{` と `\\}}` と書く。例：\"[0-9]\\{{4\\}}\""),
        ("a count can be at most {MAX_COUNT}", "回数は最大 {MAX_COUNT}"),
        ("too large", "大きすぎる"),
        ("for more, use `*` or `+`, or {{n,}} without a maximum", "それ以上なら `*` か `+`、または上限なしの {{n,}} を使う"),
        ("the count goes backwards: {min} is more than {max}", "回数の順序が逆：{min} は {max} より大きい"),
        ("smallest first", "小さい方を先に"),
        ("write the smallest number first: {{{max},{min}}}", "小さい数を先に書く：{{{max},{min}}}"),
        ("a repetition cannot be repeated again", "繰り返しをさらに繰り返すことはできない"),
        ("repeats a repetition", "繰り返しを繰り返している"),
        ("put the repeated part in parentheses, like `(a+)*`, or keep one sign, like `a*`; {}", "繰り返す部分を `(a+)*` のように括弧で囲むか、`a*` のように記号を 1 つにする。{}"),
        ("the groups are nested too deeply (more than {MAX_DEPTH})", "グループの入れ子が深すぎる（{MAX_DEPTH} 段を超える）"),
        ("too deep", "深すぎる"),
        ("`(?<` must be followed by a name and `>`", "`(?<` の後には名前と `>` が必要"),
        ("not a group name", "グループ名ではない"),
        ("a named group looks like `(?<year>[0-9]+)`: the name has letters, digits and `_`", "名前付きグループは `(?<year>[0-9]+)` のように書く。名前には英字、数字、`_` を使う"),
        ("two groups are named `{name}`", "`{name}` という名前のグループが 2 つある"),
        ("already used", "使用済み"),
        ("give each group its own name", "グループごとに別の名前を付ける"),
        ("`(?i)` must be at the very start of the regex", "`(?i)` は正規表現のいちばん最初に置く"),
        ("not at the start", "先頭ではない"),
        ("`(?i)` ignores the case in the whole regex: write it first", "`(?i)` は正規表現全体で大文字と小文字を区別しない。最初に書く"),
        ("unknown kind of group", "不明な種類のグループ"),
        ("unknown", "不明"),
        ("the groups are `(...)`, `(?:...)` (a group that captures nothing) and `(?<name>...)` (a named group); `(?i)` at the start ignores the case", "グループは `(...)`、`(?:...)`（何もキャプチャしないグループ）、`(?<name>...)`（名前付きグループ）。先頭の `(?i)` は大文字と小文字を区別しない"),
        ("this `(` is never closed", "この `(` が閉じられていない"),
        ("opened here", "ここで開いている"),
        ("add a `)` where the group ends; {}", "グループの終わりに `)` を加える。{}"),
        ("the regex ends with a lone `\\`", "正規表現が単独の `\\` で終わっている"),
        ("followed by nothing", "後に何もない"),
        ("a `\\` comes before a letter like d, w, s or b, or before a sign for the sign itself, like `\\.`", "`\\` は d、w、s、b などの英字の前か、記号そのものを表すときに記号の前（`\\.` など）に置く"),
        ("`\\{c}` means nothing inside `[...]`", "`[...]` の中の `\\{c}` には意味がない"),
        ("not a character", "文字ではない"),
        ("`\\b` is the edge of a word: write it outside of the brackets", "`\\b` は単語の境界：角かっこの外に書く"),
        ("`\\{c}` means nothing in a regex", "`\\{c}` は正規表現では意味がない"),
        (
            r#"the known ones: \d (a digit), \w (a letter, a digit or _), \s (a space), \b (the edge of a word),
their opposites \D, \W, \S, \B, \n, \t, and `\` before a sign for the sign itself, like \. or \("#,
            r#"使えるもの：\d（数字）、\w（英字、数字、_）、\s（空白）、\b（単語の境界）、
その逆の \D、\W、\S、\B、\n、\t、そして記号そのものを表す `\`＋記号（\. や \( など）"#,
        ),
        ("this `[` is never closed", "この `[` が閉じられていない"),
        ("add a `]` after the characters; for the sign `]` itself, put it first: `[]]`", "文字の後に `]` を加える。記号 `]` そのものは先頭に置く：`[]]`"),
        ("a range must end with one character", "範囲は 1 文字で終わらなければならない"),
        ("not a range", "範囲ではない"),
        ("for the sign `-` itself, put it last: `[a-]`", "記号 `-` そのものは最後に置く：`[a-]`"),
        ("the range `{lo}-{hi}` goes backwards", "範囲 `{lo}-{hi}` が逆順"),
        ("backwards", "逆順"),
        ("write the first character first: `{hi}-{lo}`", "先の文字を先に書く：`{hi}-{lo}`"),
        ("this `)` closes nothing", "この `)` は何も閉じていない"),
        ("no `(` before it", "前に `(` がない"),
        ("the regex is too large once its repetitions are written out", "繰り返しを展開すると正規表現が大きくなりすぎる"),
        ("use smaller counts in {{n,m}}, or `*` and `+`, which cost nothing", "{{n,m}} の回数を小さくするか、コストのかからない `*` や `+` を使う"),
        ("`$1` is the text of group 1, `$<name>` of a named group, `$0` the whole match; for the sign `$` itself, write `$$`", "`$1` はグループ 1 のテキスト、`$<name>` は名前付きグループのテキスト、`$0` はマッチ全体。記号 `$` そのものは `$$` と書く"),
        ("the regex has no group", "正規表現にグループがない"),
        ("the regex has 1 group", "正規表現のグループは 1 つ"),
        ("the regex has {g} groups", "正規表現のグループは {g} 個"),
        ("there is no group {n}: {has}", "グループ {n} はない：{has}"),
        ("no such group", "そのグループはない"),
        ("`$<` must be followed by a group name and `>`", "`$<` の後にはグループ名と `>` が必要"),
        ("never closed", "閉じられていない"),
        ("the regex has no named group", "正規表現に名前付きグループがない"),
        ("its named groups are {}", "名前付きグループは {}"),
        ("the regex has no group named `{name}`: {has}", "正規表現に `{name}` という名前のグループはない：{has}"),
        ("unknown group", "不明なグループ"),
        ("`$` must be followed by a group number, `<name>` or `$`", "`$` の後にはグループ番号、`<name>`、`$` のどれかが必要"),
        ("a lone `$`", "単独の `$`"),
        ("invalid replacement: {}", "置換が無効：{}"),
        ("invalid regex: {}", "正規表現が無効：{}"),
        ("with this replacement", "この置換で"),
        ("with this regex", "この正規表現で"),
        ("here in the replacement:", "置換のここ："),
        ("here in the regex:", "正規表現のここ："),
        ("`named_captures` needs a regex with named groups", "`named_captures` には名前付きグループのある正規表現が必要"),
        ("name a group like this: `(?<year>[0-9]+)`; for groups without names, use `captures`", "グループには `(?<year>[0-9]+)` のように名前を付ける。名前のないグループには `captures` を使う"),
        // src/builtins/system.rs
        ("`{f}` is not available in the browser: a program here has no input to read. Run it on your computer: rythme file.ry < lines.txt", "`{f}` はブラウザーでは使えない：ここではプログラムに読み込む入力がない。自分のコンピューターで実行する：rythme file.ry < lines.txt"),
        ("`{f}` is not available in the browser: a program here has no files or folders. Run it on your computer: rythme file.ry", "`{f}` はブラウザーでは使えない：ここではプログラムにファイルもフォルダーもない。自分のコンピューターで実行する：rythme file.ry"),
        ("the path is empty: write \".\" for the current folder", "パスが空：現在のフォルダーには \".\" と書く"),
        ("folder not found: {path}", "フォルダーが見つからない：{path}"),
        ("file not found: {path}", "ファイルが見つからない：{path}"),
        ("permission denied: {path}", "アクセスが拒否された：{path}"),
        ("cannot access {path}: {e}", "{path} にアクセスできない：{e}"),
        ("{path} is a file, not a folder: `read_file` reads a file", "{path} はフォルダーではなくファイル：`read_file` はファイルを読む"),
        ("{path} already exists, and it is a file, not a folder", "{path} はすでにあり、フォルダーではなくファイル"),
        ("{path} is a folder: `delete_file` deletes only files, never folders", "{path} はフォルダー：`delete_file` はファイルだけを削除し、フォルダーは決して削除しない"),
        ("cannot read the standard input: {e}", "標準入力を読めない：{e}"),
        ("`join_path` needs a list of texts, but it contains {}", "`join_path` にはテキストのリストが必要だが、{} が含まれている"),
        ("`text(value)` turns a value into a text", "`text(value)` は値をテキストに変える"),
        ("`join_path` cannot add \"{part}\" after \"{out}\": only the first part of a path can start at a root (`/`, `\\` or a drive like `C:`)", "`join_path` は \"{out}\" の後に \"{part}\" を追加できない：ルート（`/`、`\\`、`C:` のようなドライブ）から始められるのはパスの最初の部分だけ"),
        ("remove the separator at the start of \"{part}\", or put it first in the list", "\"{part}\" の先頭の区切り文字を取るか、それをリストの最初に置く"),
        ("`{w}` is not a valid option: its name starts with a letter, followed by letters, digits, `-` or `_` (write `--` before words that start with `-`)", "`{w}` は有効なオプションではない：名前は英字で始まり、英字、数字、`-`、`_` が続く（`-` で始まる単語の前には `--` を書く）"),
        ("`{w}` is not an option that Rythme understands: write `--{name}` for a long name, or `-a -b` for several one-letter options", "`{w}` は Rythme が理解できるオプションではない：長い名前は `--{name}`、1 文字のオプションを複数なら `-a -b` と書く"),
        ("`{w}` cannot be an option: the field `args` holds the words that are not options", "`{w}` はオプションにできない：フィールド `args` にはオプションでない単語が入る"),
        ("the option `{name}` is given twice", "オプション `{name}` が 2 回指定されている"),
        ("`options` needs a list of texts, like the one of `args()`, but it contains {}", "`options` には `args()` が返すようなテキストのリストが必要だが、{} が含まれている"),
        // src/builtins/text_tools.rs
        ("`{f}` writes at most 100 decimals, not {d}", "`{f}` が書ける小数は最大 100 桁（{d} は不可）"),
        ("`{f}` does not know the language \"{l}\"", "`{f}` は言語 \"{l}\" を知らない"),
        ("the languages: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" and \"pt\" (1.234,5)", "使える言語：\"en\"（1,234.5）、\"fr\"（1 234,5）、\"de\"、\"es\"、\"it\"、\"nl\"、\"pt\"（1.234,5）"),
        // src/builtins.rs
        ("Basics", "基本"),
        ("Numbers", "数値"),
        ("Text", "テキスト"),
        ("Lists", "リスト"),
        ("Records", "レコード"),
        ("Dictionaries and sets", "辞書と集合"),
        ("Option and Result", "Option と Result"),
        ("Files", "ファイル"),
        ("Dates and times", "日付と時刻"),
        ("Property tests", "プロパティテスト"),
        ("usage: `{}`", "使い方：`{}`"),
        ("a list", "リスト"),
        ("a text", "テキスト"),
        ("a whole number (Int)", "整数（Int）"),
        ("a number", "数値"),
        ("a function", "関数"),
        ("a record", "レコード"),
        ("true or false", "true か false"),
        ("a list or a text", "リストかテキスト"),
        ("a text or a list", "テキストかリスト"),
        ("a text, a list, a record, a dict or a set", "テキスト、リスト、レコード、辞書、集合のいずれか"),
        ("a list, a text, a record or a dict", "リスト、テキスト、レコード、辞書のいずれか"),
        ("a record or a dict", "レコードか辞書"),
        ("a record, a dict or a set", "レコード、辞書、集合のいずれか"),
        ("a list, a dict or a set", "リスト、辞書、集合のいずれか"),
        ("a list or a set", "リストか集合"),
        ("a dict or a set", "辞書か集合"),
        ("a dict", "辞書"),
        ("a set", "集合"),
        ("a list of [key, value] pairs (or a record)", "[キー, 値] のペアのリスト（またはレコード）"),
        ("a list (or several values)", "リスト（または複数の値）"),
        ("a number or a text", "数値かテキスト"),
        ("a list, a set, an Option or a Result", "リスト、集合、Option、Result のいずれか"),
        ("an Option or a Result", "Option か Result"),
        ("an Option", "Option"),
        ("a Result", "Result"),
        ("a date (a record {{year, month, day}})", "日付（レコード {{year, month, day}}）"),
        ("a generator (like `ints()`)", "ジェネレーター（`ints()` など）"),
        ("a text or a function", "テキストか関数"),
        ("`{f}` expects {expected} as argument {pos}, but got {got}", "`{f}` の第 {pos} 引数には{expected}が必要だが、{got} を受け取った"),
        ("wrong kind of value", "値の種類が違う"),
        ("a text is not a list: `chars(text)` gives the list of its characters", "テキストはリストではない：`chars(text)` で文字のリストが得られる"),
        ("a record is not a list: `entries(record)`, `keys(record)` or `values(record)` give one", "レコードはリストではない：`entries(record)`、`keys(record)`、`values(record)` でリストが得られる"),
        ("a dict is not a list: `entries(dict)`, `keys(dict)` or `values(dict)` give one", "辞書はリストではない：`entries(dict)`、`keys(dict)`、`values(dict)` でリストが得られる"),
        ("a set is not a list: `values(set)` gives the list of its elements, in order", "集合はリストではない：`values(set)` で要素のリストが順番に得られる"),
        ("to change the values of a dict, use `map_values(dict, f)`", "辞書の値を変えるには `map_values(dict, f)` を使う"),
        ("for a list, use `{cousin}`", "リストには `{cousin}` を使う"),
        ("to read a date from a text like \"2024-02-29\", use `parse_date(text)`", "\"2024-02-29\" のようなテキストから日付を読むには `parse_date(text)` を使う"),
        ("for a list, use `find(list, f)`", "リストには `find(list, f)` を使う"),
        ("for an Option or a Result, use `and_then(x, f)`", "Option や Result には `and_then(x, f)` を使う"),
        ("the function given to `{f}` must return a list, but it returned {got}", "`{f}` に渡す関数はリストを返さなければならないが、{got} を返した"),
        ("the function given to `{f}` must return a text, but it returned {got}", "`{f}` に渡す関数はテキストを返さなければならないが、{got} を返した"),
        ("to make a text of any value, use `text(x)`", "どんな値からでもテキストを作るには `text(x)` を使う"),
        ("an Option (Some or None)", "Option（Some か None）"),
        ("a Result (Ok or Err)", "Result（Ok か Err）"),
        ("the function given to `{f}` must return {expected}, but it returned {got}", "`{f}` に渡す関数は{expected}を返さなければならないが、{got} を返した"),
        ("to change the value inside without another check, use `map(x, f)`", "再チェックせずに中の値を変えるには `map(x, f)` を使う"),
        ("the function given to `{f}` must return true or false, but it returned {got}", "`{f}` に渡す関数は true か false を返さなければならないが、{got} を返した"),
        ("the number {n} is too large for `{f}`", "数値 {n} は `{f}` には大きすぎる"),
        ("{digits} digits", "{digits} 桁"),
        ("`{f}` needs a number of 0 or more, not {n}", "`{f}` には 0 以上の数値が必要（{n} は不可）"),
        ("`{f}` needs a number of 1 or more, not 0", "`{f}` には 1 以上の数値が必要（0 は不可）"),
        ("the list is empty", "リストが空"),
        ("the list has {len} element(s)", "リストの要素は {len} 個"),
        ("position {n} is out of range: {what}", "位置 {n} は範囲外：{what}"),
        ("no element here", "ここに要素はない"),
        ("valid positions go from 0 to {last} (or from -1 backwards, counting from the end)", "有効な位置は 0 から {last} まで（または末尾から数えて -1 から逆向き）"),
        ("at least {min}", "{min} 以上"),
        ("{min} to {max}", "{min} から {max}"),
        ("`{f}` gives {given} argument(s) to this function, but it takes {expected}", "`{f}` はこの関数に引数を {given} 個渡すが、この関数は {expected} 個取る"),
        ("takes {expected} argument(s)", "引数を {expected} 個取る"),
        ("`nan` (not a number) cannot be converted to a whole number", "`nan`（非数）は整数に変換できない"),
        ("`nan` is the result of a calculation that has none, like `0.0 * inf`: check where this value comes from", "`nan` は `0.0 * inf` のように結果を持たない計算の結果：この値がどこから来たか確かめる"),
        ("`{}` (infinity) cannot be converted to a whole number", "`{}`（無限大）は整数に変換できない"),
        ("an infinity comes from a calculation too large for decimal numbers, like `1e308 * 10.0`, or from a division by `0.0`", "無限大は `1e308 * 10.0` のような小数には大きすぎる計算か、`0.0` による除算から生じる"),
        ("`text` writes at most 100 decimals, not {digits}", "`text` が書ける小数は最大 100 桁（{digits} は不可）"),
        ("`input` found the end of the input", "`input` が入力の終わりに達した"),
        ("no more lines to read", "読む行がもうない"),
        ("the keyboard input was closed (Ctrl-D), or the text sent to the program has ended", "キーボード入力が閉じられた（Ctrl-D）か、プログラムに送られたテキストが終わった"),
        ("assertion failed: {}", "アサーション失敗：{}"),
        ("assertion failed", "アサーション失敗"),
        ("this condition is false", "この条件は偽"),
        (
            r#"got:      {got}
expected: {expected}"#,
            r#"得られた値：{got}
期待した値：{expected}"#,
        ),
        ("first difference: character {at}", "最初の違い：{at} 文字目"),
        ("assertion failed: the values are different", "アサーション失敗：値が違う"),
        (" at {path}", "（位置 {path}）"),
        ("the list{at} has {} element(s), {} expected", "リスト{at}の要素は {} 個で、期待は {} 個"),
        ("in excess", "余分"),
        ("missing", "不足"),
        ("{word} from [{from}]: {}{dots}", "[{from}] 以降が{word}：{}{dots}"),
        ("missing field(s){at}: {}", "不足しているフィールド{at}：{}"),
        ("field(s) not expected{at}: {}", "予期しないフィールド{at}：{}"),
        (", which are of different types: {} and {}", "（型が違う：{} と {}）"),
        ("first difference at {path}: got {}, expected {}{kinds}", "最初の違い（{path}）：得られた値 {}、期待した値 {}{kinds}"),
        ("the values are of different types: {} and {}", "値の型が違う：{} と {}"),
        ("(no line)", "（行なし）"),
        (
            r#"first different line: line {}
  got:      {}
  expected: {}"#,
            r#"最初に異なる行：{} 行目
  得られた値：{}
  期待した値：{}"#,
        ),
        ("the program stopped with `fail`", "プログラムが `fail` で停止した"),
        ("the program stopped here", "プログラムはここで停止した"),
        ("an exit code goes from 0 to 255, not {code}", "終了コードは 0 から 255 までで、{code} は範囲外"),
        ("the program stopped with exit({code})", "プログラムが exit({code}) で停止した"),
        ("cannot pick a random element in an empty list", "空のリストからランダムに要素を選ぶことはできない"),
        ("random({lo}, {hi}): the minimum is larger than the maximum", "random({lo}, {hi})：最小値が最大値より大きい"),
        ("`{name}` of an empty list", "空のリストの `{name}`"),
        ("`{name}` cannot compare {} and {}", "`{name}` は {} と {} を比較できない"),
        ("cannot take the square root of a negative number ({x})", "負の数の平方根は計算できない（{x}）"),
        ("use `sqrt(abs(x))` if you want the root of its absolute value", "絶対値の平方根が欲しいなら `sqrt(abs(x))` を使う"),
        ("cannot convert \"{s}\" to a whole number", "\"{s}\" を整数に変換できない"),
        ("if the text may not be a number, use `parse_int(t)`: it gives `Ok(n)` or `Err(...)` that you can check", "テキストが数値でない可能性があるなら `parse_int(t)` を使う。結果はチェックできる `Ok(n)` か `Err(...)` になる"),
        ("cannot convert \"{s}\" to a decimal number", "\"{s}\" を小数に変換できない"),
        ("if the text may not be a number, use `parse_float(t)`, which gives `Ok(x)` or `Err(...)`", "テキストが数値でない可能性があるなら、`Ok(x)` か `Err(...)` を返す `parse_float(t)` を使う"),
        ("`{name}` needs a number from -1 to 1, not {x}", "`{name}` には -1 から 1 までの数値が必要（{x} は不可）"),
        ("the logarithm only exists for positive numbers, not {x}", "対数は正の数にしかない（{x} は不可）"),
        ("invalid logarithm base: {base}", "対数の底が無効：{base}"),
        ("`{f}` needs a single character, but \"{s}\" has {}", "`{f}` には 1 文字が必要だが、\"{s}\" は {} 文字ある"),
        ("`replace` cannot replace an empty text", "`replace` は空のテキストを置換できない"),
        ("{n} is not the number of a character", "{n} は文字の番号ではない"),
        ("character numbers go from 0 to 1114111, except 55296 to 57343", "文字の番号は 0 から 1114111 まで（55296 から 57343 を除く）"),
        ("the repeated text would be too large", "繰り返したテキストが大きくなりすぎる"),
        ("the repeated list would be too large", "繰り返したリストが大きくなりすぎる"),
        ("this number has more than {} digits", "この数値は {} 桁を超えている"),
        ("\"{s}\" is not a whole number", "\"{s}\" は整数ではない"),
        ("\"{s}\" is not a number", "\"{s}\" は数値ではない"),
        ("JSON, line {line}, column {column}: {msg}", "JSON、{line} 行 {column} 列：{msg}"),
        ("expected {what}, but found `{c}`", "{what}があるはずだが、`{c}` が見つかった"),
        ("expected {what}, but the text ends", "{what}があるはずだが、テキストが終わっている"),
        ("a value", "値"),
        ("the end of the text", "テキストの終わり"),
        ("`,` or `]`", "`,` か `]`"),
        ("`,` or `}}`", "`,` か `}}`"),
        ("a name in quotes", "引用符で囲んだ名前"),
        ("invalid number: JSON writes numbers like 0, -12, 3.5 or 1e-3", "無効な数値：JSON の数値は 0、-12、3.5、1e-3 のように書く"),
        ("invalid escape: after `\\` comes one of \" \\ / b f n r t u", "無効なエスケープ：`\\` の後には \" \\ / b f n r t u のどれかが来る"),
        ("a line break or a tab inside a text is written with `\\`: `\\n`, `\\t`", "テキスト中の改行やタブは `\\` を使って `\\n`、`\\t` と書く"),
        ("a text is never closed: `\"` is missing", "テキストが閉じられていない：`\"` がない"),
        ("`\\u` must be followed by 4 hexadecimal digits", "`\\u` の後には 16 進数 4 桁が必要"),
        ("invalid `\\u` escape: half of a character (a surrogate) without its other half", "無効な `\\u` エスケープ：文字の半分（サロゲート）だけで、もう半分がない"),
        ("`to_json` indents by 16 spaces at most, not {n}", "`to_json` のインデントは最大 16 スペース（{n} は不可）"),
        ("the JSON text would be too large", "JSON テキストが大きくなりすぎる"),
        ("the values are nested too deeply to be indented: write them on one line, with `to_json(value)`", "値の入れ子が深すぎてインデントできない：`to_json(value)` で 1 行に書く"),
        ("`to_json` cannot write {} in JSON", "`to_json` は {} を JSON に書けない"),
        ("JSON has no infinite numbers and no NaN", "JSON には無限大も NaN もない"),
        (
            r#"a dict is written as a JSON object only when its keys are all texts, or all symbols:
turn the keys into texts first, or write `entries(dict)`"#,
            r#"辞書が JSON のオブジェクトとして書かれるのは、キーがすべてテキストか、すべてシンボルのときだけ：
先にキーをテキストに変えるか、`entries(dict)` と書く"#,
        ),
        (
            r#"JSON only holds numbers, texts, true and false, lists, records and null (`None`):
turn your value into a record first, like {{kind: "Circle", radius: r}}"#,
            r#"JSON に入るのは数値、テキスト、true と false、リスト、レコード、null（`None`）だけ：
先に値をレコードに変える。例：{{kind: "Circle", radius: r}}"#,
        ),
        ("`sum` needs a list of numbers, but it contains {}", "`sum` には数値のリストが必要だが、{} が含まれている"),
        ("`product` needs a list of numbers, but it contains {}", "`product` には数値のリストが必要だが、{} が含まれている"),
        ("all the elements must be of the same kind: all numbers, or all texts...", "要素はすべて同じ種類でなければならない：すべて数値、すべてテキストなど"),
        ("the result of `windows` would be too large", "`windows` の結果が大きくなりすぎる"),
        ("the function `key` must give values of the same kind for every element: all numbers, or all texts...", "関数 `key` はどの要素にも同じ種類の値を返さなければならない：すべて数値、すべてテキストなど"),
        ("`flatten` needs a list of lists, but it contains {}", "`flatten` にはリストのリストが必要だが、{} が含まれている"),
        ("`from_entries` needs a list of [name, value] pairs, but it contains {}", "`from_entries` には [名前, 値] のペアのリストが必要だが、{} が含まれている"),
        ("each pair is a list of two elements whose name is a text, like [\"age\", 36]", "各ペアは名前がテキストである 2 要素のリスト。例：[\"age\", 36]"),
        ("`unwrap` found `None`: there is no value", "`unwrap` が `None` を見つけた：値がない"),
        ("handle the missing value with `match`, or give a default: `unwrap_or(x, default)`", "値がない場合を `match` で扱うか、デフォルト値を与える：`unwrap_or(x, default)`"),
        ("`unwrap` found an error: {}", "`unwrap` がエラーを見つけた：{}"),
        ("handle the error with `match`, pass it on with `?`, or give a default: `unwrap_or(x, default)`", "エラーを `match` で扱うか、`?` で呼び出し元に渡すか、デフォルト値を与える：`unwrap_or(x, default)`"),
        // src/check/needs.rs
        ("`{name}` stops with this error, every time", "`{name}` は毎回このエラーで停止する"),
        ("here", "ここ"),
        ("in `{name}`: {label}", "`{name}` の中：{label}"),
        ("`{name}` always fails with {}", "`{name}` は必ず {} で失敗する"),
        ("fails every time", "毎回失敗する"),
        ("defined here", "ここで定義"),
        ("from how `{name}` uses `{p}`, it needs: `{sig}`", "`{name}` での `{p}` の使い方から、次が必要：`{sig}`"),
        // src/check.rs
        ("`{s} [...]` is read as an index, not as a call", "`{s} [...]` は呼び出しではなくインデックスとして読まれる"),
        ("to call `{s}`, put its argument in parentheses: `{s}([...])`; for an index, write `{s}[...]` without a space", "`{s}` を呼ぶには引数を括弧に入れる：`{s}([...])`。インデックスなら空白なしで `{s}[...]` と書く"),
        ("`{s} -...` is read as a subtraction, not as a call", "`{s} -...` は呼び出しではなく引き算として読まれる"),
        ("to call `{s}`, put its argument in parentheses: `{s}(-x)`; for a subtraction, write `{s} - x`", "`{s}` を呼ぶには引数を括弧に入れる：`{s}(-x)`。引き算なら `{s} - x` と書く"),
        ("`{s}` is not a value here", "ここでは `{s}` は値ではない"),
        ("`{s}` is a function here", "ここでは `{s}` は関数"),
        ("because `{s}` is given a value here", "ここで `{s}` に値が与えられているため"),
        ("a call without parentheses, like `{s} [1, 2]` or `{s} -1`, is read so only when the file never gives `{s}` a value", "`{s} [1, 2]` や `{s} -1` のような括弧なしの呼び出しがそう読まれるのは、ファイルの中で `{s}` に一度も値が与えられないときだけ"),
        ("the type `{name}` is part of Rythme", "型 `{name}` は Rythme に組み込まれている"),
        ("give your type another name", "自分の型には別の名前を付ける"),
        ("the type `{}` is already defined", "型 `{}` はすでに定義されている"),
        ("first defined here", "最初の定義はここ"),
        ("give it another name", "別の名前を付ける"),
        ("the field `{f}` appears twice in `{}`", "フィールド `{f}` が `{}` に 2 回出てくる"),
        ("`{s}` is already the name of the file `{file}`", "`{s}` はすでにファイル `{file}` の名前"),
        ("choose another name here, or give the file another one: `use ... as other_name`", "ここで別の名前を選ぶか、ファイルに別の名前を付ける：`use ... as other_name`"),
        ("`{s}` is already defined", "`{s}` はすでに定義されている"),
        ("defined again here", "ここで再定義"),
        ("a constructor belongs to a single type: rename one of them", "コンストラクターは 1 つの型にだけ属する：どちらかの名前を変える"),
        ("to give several definitions to a function (one per case), write them one right after the other", "関数に複数の定義（ケースごとに 1 つ）を与えるには、続けて並べて書く"),
        ("`{s}` hides the built-in function `{s}`", "`{s}` が組み込み関数 `{s}` を隠している"),
        ("`{s}` hides the built-in value `{s}`", "`{s}` が組み込みの値 `{s}` を隠している"),
        ("in this file, `{s}` will now mean your definition; choose another name to keep the built-in one", "このファイルでは、`{s}` は以後あなたの定義を指す。組み込みのものを残すには別の名前を選ぶ"),
        ("`{s}` is part of Rythme", "`{s}` は Rythme に組み込まれている"),
        ("choose another name", "別の名前を選ぶ"),
        ("the result of `{name}` is lost", "`{name}` の結果が失われる"),
        ("this value is lost", "この値は失われる"),
        (
            r#"values never change: `{sig}` gives a new value, and changes nothing.
To keep it, give it a name: `new = {sig}`"#,
            r#"値は変わらない：`{sig}` は新しい値を返すだけで、何も変えない。
残すには名前を付ける：`new = {sig}`"#,
        ),
        ("this test can never fail", "このテストは決して失敗しない"),
        ("its result, {}, is never compared with what is expected", "結果の {} が期待値と比較されていない"),
        (
            r#"a test fails only when it stops with an error, or ends with `false`, `None` or `Err(...)`;
nothing in this one can"#,
            r#"テストが失敗するのは、エラーで停止したときか、`false`、`None`、`Err(...)` で終わったときだけ。
このテストにはそうなるものがない"#,
        ),
        ("compare the result with what you expect: `assert_eq(actual, expected)`", "結果を期待値と比較する：`assert_eq(actual, expected)`"),
        ("remove `{name}` from the pattern, or write `{name}: _{name}` if this is on purpose", "パターンから `{name}` を取り除くか、意図的なら `{name}: _{name}` と書く"),
        ("remove it, or name it `_{name}` if this is on purpose", "取り除くか、意図的なら `_{name}` という名前にする"),
        ("`{name}` is never used", "`{name}` が一度も使われていない"),
        ("`{s}` appears twice in this pattern", "`{s}` がこのパターンに 2 回出てくる"),
        ("first here", "最初はここ"),
        ("use two different names, then compare them with `==` in a condition: `[a, b] if a == b -> ...`", "2 つの別の名前を使い、条件の中で `==` で比較する：`[a, b] if a == b -> ...`"),
        ("`{s}` is used before being defined", "`{s}` が定義より前に使われている"),
        ("used here", "ここで使用"),
        ("defined later, here", "後のここで定義"),
        ("move the definition above this line", "定義をこの行より上に移す"),
        ("`{s}` is the name of the file `{file}`, not a value", "`{s}` はファイル `{file}` の名前で、値ではない"),
        ("write `{s}.` and the name of one of its definitions, like `{s}.{example}`", "`{s}.` の後にその定義の名前を書く。例：`{s}.{example}`"),
        ("`{x}` is private to the file `{file}`", "`{x}` はファイル `{file}` の非公開の名前"),
        ("private", "非公開"),
        ("a name starting with `_` can only be used in its own file", "`_` で始まる名前は、それを定義したファイルの中でしか使えない"),
        ("to use it from other files, rename it without `_` in `{file}`", "他のファイルから使うには、`{file}` の中で `_` を外した名前に変える"),
        ("the file `{file}` has no `{x}`", "ファイル `{file}` に `{x}` はない"),
        ("`{m}.{x}?` is a function of `{file}`, but without `(` right after it, `?` is the operator: call it, `{m}.{x}?(...)`, or give it as `fn(v) = {m}.{x}?(v)`", "`{m}.{x}?` は `{file}` の関数だが、直後に `(` がないと `?` は演算子になる：`{m}.{x}?(...)` と呼ぶか、`fn(v) = {m}.{x}?(v)` として渡す"),
        ("not in `{file}`", "`{file}` にはない"),
        ("did you mean `{m}.{best}`?", "もしかして `{m}.{best}`？"),
        ("`{file}` defines nothing that other files can use", "`{file}` は他のファイルが使えるものを何も定義していない"),
        ("`{file}` gives: {}{more}", "`{file}` が提供するもの：{}{more}"),
        ("`{name}` is private to the file `{file}`", "`{name}` はファイル `{file}` の非公開の名前"),
        ("unknown name `{name}`", "不明な名前 `{name}`"),
        ("not defined here", "ここでは未定義"),
        ("`{name}` is defined in `{file}`: write `{m}.{name}`", "`{name}` は `{file}` で定義されている：`{m}.{name}` と書く"),
        ("`{name}` is defined in `{file}`: add `use {stem}` at the top of the file, then write `{stem}.{name}`", "`{name}` は `{file}` で定義されている：ファイルの先頭に `use {stem}` を加え、`{stem}.{name}` と書く"),
        ("not defined", "未定義"),
        ("defined here, further down", "もっと下のここで定義"),
        ("in a block, a function can be used only after it, unless they are written one after the other with nothing between them: move `{name}` up, next to the function that uses it", "ブロックの中では、関数はその定義の後でしか使えない（間に何も挟まずに続けて書いた関数どうしは別）：`{name}` を上に移し、それを使う関数の隣に置く"),
        ("in a block, a name can be used only after the line that defines it: move `{name} = ...` up", "ブロックの中では、名前はそれを定義する行の後でしか使えない：`{name} = ...` を上に移す"),
        ("did you mean `{best}`?", "もしかして `{best}`？"),
        ("to create your own kind of value, define a type: `type Name = {name}(field) | Other`", "独自の種類の値を作るには型を定義する：`type Name = {name}(field) | Other`"),
        ("define it before using it: `{name} = ...` or `fn {name}(...) = ...`", "使う前に定義する：`{name} = ...` または `fn {name}(...) = ...`"),
        ("this definition of `{name}` can never be used", "この `{name}` の定義は決して使われない"),
        ("this definition already accepts every argument", "この定義はすでにすべての引数を受け付けている"),
        ("definitions are tried in order: put the most general one last", "定義は順に試される：最も一般的なものを最後に置く"),
        ("this definition already accepts these arguments", "この定義はすでにこれらの引数を受け付けている"),
        ("definitions are tried in order: remove this one, or change its patterns", "定義は順に試される：これを取り除くか、パターンを変える"),
        ("this function receives {}, which never matches this pattern", "この関数が受け取るのは {} で、このパターンには決してマッチしない"),
        ("never matches", "決してマッチしない"),
        ("`{name}` does not handle every possible argument", "`{name}` がありうる引数をすべて扱っていない"),
        ("not handled: {}", "扱われていない：{}"),
        ("add a definition for it, like `fn {name}({}) = ...`", "それを扱う定義を加える。例：`fn {name}({}) = ...`"),
        ("the value {} does not match this pattern", "値 {} はこのパターンにマッチしない"),
        ("a value of type {} never matches this pattern", "{} 型の値はこのパターンに決してマッチしない"),
        ("`_` can only be used in patterns", "`_` はパターンの中でしか使えない"),
        ("`_` means \"anything, ignored\"; give the value a real name to use it", "`_` は「何でもよく、無視する」という意味。値を使うにはちゃんとした名前を付ける"),
        ("`?` can only be used inside a function", "`?` は関数の中でしか使えない"),
        ("outside of a function, handle the value with `match` or `unwrap_or`", "関数の外では、値を `match` か `unwrap_or` で扱う"),
        ("`?` needs an Option or a Result, but got {}", "`?` には Option か Result が必要だが、{} を受け取った"),
        ("`?` takes the value out of `Some(x)` / `Ok(x)`, and returns `None` / `Err(e)` from the current function", "`?` は `Some(x)` / `Ok(x)` から値を取り出し、`None` / `Err(e)` なら今の関数からそれを返す"),
        ("the field `{k}` appears twice", "フィールド `{k}` が 2 回出てくる"),
        ("`..` expects {expected} here, but got {}", "ここで `..` には{expected}が必要だが、{} を受け取った"),
        ("this value is not a function", "この値は関数ではない"),
        ("cannot be called", "呼び出せない"),
        ("to multiply, write `*` explicitly: `2 * (x + 1)`", "掛け算には `*` を明示的に書く：`2 * (x + 1)`"),
        ("to take one element, use brackets: `x[0]`; between two values, a `,` may be missing", "要素を 1 つ取り出すには角かっこを使う：`x[0]`。2 つの値の間なら `,` が抜けているかもしれない"),
        ("to read a field, use a dot: `person.name`", "フィールドを読むにはドットを使う：`person.name`"),
        ("`{s}` is not a function", "`{s}` は関数ではない"),
        ("`{s}` is {}", "`{s}` は {}"),
        ("returns {returned}", "{returned} を返す"),
        ("this assertion always fails: {a} and {b} are never equal", "このアサーションは必ず失敗する：{a} と {b} は決して等しくならない"),
        (
            r#"one value is inside an Option or a Result: compare it with `Some(...)` or `Ok(...)`,
like `assert_eq(parse_int("3"), Ok(3))`"#,
            r#"片方の値が Option か Result の中にある：`Some(...)` か `Ok(...)` と比較する。
例：`assert_eq(parse_int("3"), Ok(3))`"#,
        ),
        ("values of different kinds are never equal: Rythme never converts values silently", "種類の違う値は決して等しくならない：Rythme は値を黙って変換しない"),
        ("these are two different types named `{name}`, defined in two files: their values are never equal", "これは 2 つのファイルで定義された、`{name}` という名前の別々の型：その値は決して等しくならない"),
        ("a constructor, shown as `<constructor ...>`", "コンストラクター（`<constructor ...>` と表示される）"),
        ("a function, shown as `<fn ...>`", "関数（`<fn ...>` と表示される）"),
        ("this shows a function, not a value", "これは値ではなく関数を表示する"),
        ("to show what it gives, call it: `{call}`", "返す値を表示するには呼び出す：`{call}`"),
        ("no definition of `{name}` accepts these arguments: {name}({})", "これらの引数を受け付ける `{name}` の定義がない：{name}({})"),
        ("called here", "ここで呼び出し"),
        ("does not match", "マッチしない"),
        ("add a definition that handles this case, for example one that ends with a plain name or `_`", "このケースを扱う定義を加える。例えば、ただの名前か `_` で終わる定義"),
        ("the field `{name}` is not a function", "フィールド `{name}` は関数ではない"),
        ("`{name}` is {}", "`{name}` は {}"),
        ("to read the field, write it without parentheses: `.{name}`", "フィールドを読むには括弧なしで書く：`.{name}`"),
        ("there is no function named `{shown}`", "`{shown}` という名前の関数はない"),
        ("`value.{shown}(...)` calls `{shown}(value, ...)`, or the field `{shown}` of a record", "`value.{shown}(...)` は `{shown}(value, ...)` かレコードのフィールド `{shown}` を呼び出す"),
        ("`{name}` is a value, not a function", "`{name}` は関数ではなく値"),
        ("write `{name}` without parentheses", "`{name}` は括弧なしで書く"),
        ("this record has no field `{name}`", "このレコードにフィールド `{name}` はない"),
        ("unknown field", "不明なフィールド"),
        ("the record is empty: {{}}", "レコードが空：{{}}"),
        ("its fields are: {}", "フィールドは次のとおり：{}"),
        ("to get an Option instead of an error, use `get(record, \"{name}\")`", "エラーではなく Option を得るには `get(record, \"{name}\")` を使う"),
        ("{} has no fields", "{} にはフィールドがない"),
        ("no field `{name}` here", "ここにフィールド `{name}` はない"),
        ("did you mean `.{best}`? (`value.{best}` calls `{best}(value)`)", "もしかして `.{best}`？（`value.{best}` は `{best}(value)` を呼び出す）"),
        ("did you mean `.{best}`? (a field of this record)", "もしかして `.{best}`？（このレコードのフィールド）"),
        ("`{c}` has no field `{name}`", "`{c}` にフィールド `{name}` はない"),
        ("no variant of `{tn}` has a field `{name}`", "`{tn}` のどのバリアントにもフィールド `{name}` はない"),
        ("`{c}` has no fields", "`{c}` にはフィールドがない"),
        ("the fields of `{c}` are: {}", "`{c}` のフィールドは次のとおり：{}"),
        ("the variants of `{tn}` have no fields", "`{tn}` のバリアントにはフィールドがない"),
        ("their fields are: {}", "それらのフィールドは次のとおり：{}"),
        ("a value of type `{tn}` may be another variant: use `match` to handle each case", "`{tn}` 型の値は別のバリアントかもしれない：`match` を使って各ケースを扱う"),
        ("cannot use `[...]` on {}", "{} には `[...]` を使えない"),
        ("`[...]` works on lists, texts, records and dicts", "`[...]` はリスト、テキスト、レコード、辞書に使える"),
        ("an index must be a whole number (Int), but got {}", "インデックスは整数（Int）でなければならないが、{} を受け取った"),
        ("the name of a field must be a text, but got {}", "フィールドの名前はテキストでなければならないが、{} を受け取った"),
        ("read a field with `record[\"name\"]` or `record.name`", "フィールドは `record[\"name\"]` か `record.name` で読む"),
        ("this record has no field `{k}`", "このレコードにフィールド `{k}` はない"),
        ("use `get(record, key)` to receive an Option instead", "代わりに Option を受け取るには `get(record, key)` を使う"),
        ("{} and {}", "{} と {}"),
        ("a range needs whole numbers, but got {got}", "範囲には整数が必要だが、{got} を受け取った"),
        ("example: `1..10` is the list of the numbers from 1 to 10", "例：`1..10` は 1 から 10 までの数のリスト"),
        ("cannot use `-` on {}", "{} には `-` を使えない"),
        ("a condition must be `true` or `false`, but this is {}", "条件は `true` か `false` でなければならないが、これは {}"),
        ("not a Bool", "Bool ではない"),
        ("compare it explicitly, for example `x != 0` or `x > 0`", "明示的に比較する。例：`x != 0` や `x > 0`"),
        ("to test if it is empty, use `is_empty(x)` or `not is_empty(x)`", "空かどうか調べるには `is_empty(x)` か `not is_empty(x)` を使う"),
        ("use `is_some(x)`, `is_ok(x)`, or `match` to look inside", "中を見るには `is_some(x)`、`is_ok(x)`、`match` のどれかを使う"),
        ("cannot use `{sym}` with {ln} and {rn}", "`{sym}` は {ln} と {rn} には使えない"),
        ("cannot use `{sym}` with {ln}", "`{sym}` は {ln} には使えない"),
        ("cannot use `{sym}` with {rn}", "`{sym}` は {rn} には使えない"),
        ("`{name} -x` subtracts here; to call `{name}`, write `{name}(-x)` (a call without parentheses starts a statement or a result, such as a branch of `if` or the body after the `=` of a function, and its name is not given a value with `=` or as a parameter)", "ここでは `{name} -x` は引き算になる。`{name}` を呼ぶには `{name}(-x)` と書く（括弧なしの呼び出しになるのは、`if` の分岐や関数の `=` の後の本体のように文や結果の先頭にあり、その名前が `=` や引数として値を与えられていないときだけ）"),
        ("to join two sets or two dicts, use `union(a, b)`; to take one away from the other, `difference(a, b)`", "2 つの集合や辞書をまとめるには `union(a, b)` を、一方から他方を取り除くには `difference(a, b)` を使う"),
        (
            r#"to build a text with a value inside, use braces: "total: {{x}}"
or convert the value first: text(x)"#,
            r#"値を含むテキストを作るには波かっこを使う："total: {{x}}"
または先に値を変換する：text(x)"#,
        ),
        ("to add one element to a list, use `push(list, x)` or `[..list, x]`", "リストに要素を 1 つ加えるには `push(list, x)` か `[..list, x]` を使う"),
        ("to repeat a text or a list, use `repeat(x, n)`", "テキストやリストを繰り返すには `repeat(x, n)` を使う"),
        ("convert the text to a number first, with `int(t)` or `float(t)`", "先にテキストを数値に変換する：`int(t)` か `float(t)`"),
        ("only values of the same kind can be compared", "比較できるのは同じ種類の値どうしだけ"),
        ("this comparison is always false: {ln} and {rn} are never equal", "この比較は常に偽：{ln} と {rn} は決して等しくならない"),
        ("this comparison is always true: {ln} and {rn} are never equal", "この比較は常に真：{ln} と {rn} は決して等しくならない"),
        ("to look inside an Option, use `match`, `unwrap_or(x, default)` or `is_some(x)`", "Option の中を見るには `match`、`unwrap_or(x, default)`、`is_some(x)` のどれかを使う"),
        ("Rythme never converts values silently: convert one side first, with `text(x)` or `int(t)`", "Rythme は値を黙って変換しない：`text(x)` や `int(t)` で先に片方を変換する"),
        ("this case can never be reached", "このケースには決して到達しない"),
        ("this case already handles everything", "このケースはすでにすべてを扱っている"),
        ("cases are tried in order: put `_` last", "ケースは順に試される：`_` を最後に置く"),
        ("this case already handles these values", "このケースはすでにこれらの値を扱っている"),
        ("a value goes to the first case that fits: remove this case, or change it", "値は最初に当てはまるケースに進む：このケースを取り除くか、変更する"),
        ("no case of this `match` handles {}", "この `match` のどのケースも {} を扱っていない"),
        ("this value", "この値"),
        ("add a case for this value, or a last case `_ -> ...` that handles everything else", "この値のケースを加えるか、それ以外をすべて扱う最後のケース `_ -> ...` を加える"),
        ("this `match` does not handle every case", "この `match` はすべてのケースを扱っていない"),
        ("add a last case `_ -> ...` for all the other values", "他のすべての値のために最後のケース `_ -> ...` を加える"),
        ("add the missing cases, like `{} -> ...`, or a last case `_ -> ...`", "`{} -> ...` のような足りないケースか、最後のケース `_ -> ...` を加える"),
        ("unknown constructor `{n}`", "不明なコンストラクター `{n}`"),
        ("constructors are defined with `type`: `type Name = {n}(field) | Other`", "コンストラクターは `type` で定義する：`type Name = {n}(field) | Other`"),
        ("`{name}` has {} field(s), but this pattern gives {}", "`{name}` のフィールドは {} 個だが、このパターンは {} 個与えている"),
        ("write it like `{example}`", "`{example}` のように書く"),
        ("this range never matches", "この範囲は決してマッチしない"),
        ("no number goes from {lo} up to {hi}", "{lo} から {hi} まで上がっていく数はない"),
        ("a range goes from the smaller number to the larger one: `{hi}..{lo}`", "範囲は小さい数から大きい数へ進む：`{hi}..{lo}`"),
        ("patterns joined with `|` cannot give names (here `{s}`)", "`|` でつないだパターンは名前を与えられない（ここでは `{s}`）"),
        ("write separate cases instead", "代わりに別々のケースを書く"),
        ("handles some of these numbers", "これらの数の一部を扱っている"),
        ("together, the cases before it already handle all its numbers: remove it, or change it", "これより前のケースだけで、そのすべての数がすでに扱われている：取り除くか、変更する"),
        ("these ranges overlap", "これらの範囲が重なっている"),
        ("{both} is also here", "{both} はここにも入る"),
        ("{both} goes here, the first that fits", "{both} は最初に当てはまるここに進む"),
        ("both ends of a range are included: make the ranges follow each other without overlapping, like `0..9` then `10..19`", "範囲は両端を含む：`0..9` の次に `10..19` のように、範囲が重ならずに続くようにする"),
        ("use `match` to handle the different possible shapes of a value", "値のさまざまな形を扱うには `match` を使う"),
        ("a value of type {}", "{} 型の値"),
        ("this line computes a value but does nothing with it", "この行は値を計算しているが、何もしていない"),
        (
            r#"a line that starts with `-` is a new statement. To continue the line above,
end that line with `-` instead:
  total = price -
    discount"#,
            r#"`-` で始まる行は新しい文になる。上の行を続けるには、
代わりにその行を `-` で終える：
  total = price -
    discount"#,
        ),
        (
            r#"to keep the value, give it a name: `result = ...`; to show it: `print(...)`.
Values never change: `x + 1` does not change `x`."#,
            r#"値を残すには名前を付ける：`result = ...`。表示するには：`print(...)`。
値は変わらない：`x + 1` は `x` を変えない。"#,
        ),
        // src/diagnostic.rs
        ("error", "エラー"),
        ("warning", "警告"),
        ("help", "ヘルプ"),
        ("call trace (most recent first):", "呼び出し履歴（新しいものから）："),
        ("an anonymous function", "無名関数"),
        ("in", "関数"),
        ("called at", "の呼び出し位置"),
        ("and {} more calls", "ほか {} 件の呼び出し"),
        ("more about this error: `rythme explain {}`", "このエラーについて詳しく：`rythme explain {}`"),
        // src/doc.rs
        ("`{a}` needs a file name, like `rythme doc geometry.ry --out geometry.html`", "`{a}` にはファイル名が必要。例：`rythme doc geometry.ry --out geometry.html`"),
        ("unknown option `{o}` for `rythme doc`", "`rythme doc` に不明なオプション `{o}`"),
        (
            r#"
did you mean `{best}`?"#,
            r#"
もしかして `{best}`？"#,
        ),
        (
            r#"
see `rythme help doc`"#,
            r#"
`rythme help doc` を参照"#,
        ),
        (
            r#"which files? Example: rythme doc geometry.ry
  a folder documents all its .ry files: rythme doc .
  --html writes one HTML page instead of Markdown
  --out FILE writes to a file (a name ending with .html gives HTML)
see `rythme help doc`"#,
            r#"どのファイル？ 例：rythme doc geometry.ry
  フォルダーを指定すると、その中のすべての .ry ファイルを文書化する：rythme doc .
  --html は Markdown の代わりに HTML ページを 1 つ書く
  --out FILE はファイルに書く（.html で終わる名前なら HTML）
`rythme help doc` を参照"#,
        ),
        ("no .ry file in `{target}`", "`{target}` に .ry ファイルがない"),
        ("Documentation of {one}", "{one} のドキュメント"),
        ("Documentation", "ドキュメント"),
        ("written: {file}", "書き込み完了：{file}"),
        ("cannot write the file `{file}`: {e}", "ファイル `{file}` を書き込めない：{e}"),
        ("Types", "型"),
        ("Functions", "関数"),
        ("Values", "値"),
        ("Tests", "テスト"),
        ("Uses", "使用ファイル"),
        ("Contents", "目次"),
        ("Built-in functions mentioned", "言及している組み込み関数"),
        ("Example", "例"),
        ("Nothing public to document.", "文書化する公開の定義はありません。"),
        ("Documentation written by `rythme doc`", "`rythme doc` で作成したドキュメント"),
        (" Same as: {} (its Ruby names).", " {} と同じ（Ruby での名前）。"),
        ("Example:", "例："),
        (" as {open}{n}{close}", " （{open}{n}{close} として）"),
        (": ", "："),
        // src/driver.rs
        ("`{shown}` is a folder, not a file", "`{shown}` はファイルではなくフォルダー"),
        ("give a file inside it, like {}", "その中のファイルを指定する。例：{}"),
        ("file `{shown}` not found", "ファイル `{shown}` が見つからない"),
        ("no permission to read the file `{shown}`", "ファイル `{shown}` を読む権限がない"),
        ("cannot read the file `{shown}`: {e}", "ファイル `{shown}` を読めない：{e}"),
        ("the file `{shown}` is not text in the UTF-8 encoding (unexpected byte on line {line})", "ファイル `{shown}` は UTF-8 エンコーディングのテキストではない（{line} 行目に予期しないバイト）"),
        ("it looks like it was saved in UTF-16", "UTF-16 で保存されているようだ"),
        ("save it again with the UTF-8 encoding (most editors have it in \"Save as\" or in the status bar)", "UTF-8 エンコーディングで保存し直す（たいていのエディターでは「名前を付けて保存」かステータスバーにある）"),
        ("(`{name}` now has {n} definitions, tried in order; :reset to start again)", "（`{name}` の定義は {n} 個になり、順に試される。やり直すには :reset）"),
        ("file `{p}` not found", "ファイル `{p}` が見つからない"),
        ("looked for: {}", "探した場所：{}"),
        ("the Rythme files there are: {}{more}", "そこにある Rythme ファイル：{}{more}"),
        ("the path is relative to the folder of the file that contains `use`", "パスは `use` を含むファイルのフォルダーからの相対パス"),
        ("`{p}` uses itself", "`{p}` が自分自身を使っている"),
        ("`{last}` and `{first}` use each other", "`{last}` と `{first}` が互いを使っている"),
        ("the circle: {}", "循環：{}"),
        ("files cannot use each other: move what both need into a third file, used by both", "ファイルどうしが互いを使うことはできない：両方に必要なものを 3 つ目のファイルに移し、両方からそれを使う"),
        ("the name `{n}` is given to two files: `{first}` and `{second}`", "名前 `{n}` が 2 つのファイルに付けられている：`{first}` と `{second}`"),
        ("give this one another name: `use ... as other_name`", "こちらに別の名前を付ける：`use ... as other_name`"),
        ("`{s}` is defined by two used files: `{first}` and `{second}`", "`{s}` は使っている 2 つのファイルで定義されている：`{first}` と `{second}`"),
        ("use the files with their names, without quotes, and write which one you mean: `use geometry`, then `geometry.{s}`", "ファイルは引用符なしの名前で使い、どちらのものかを書く：`use geometry` の後に `geometry.{s}`"),
        // src/exercise.rs
        ("which exercise? Example: rythme exercise start 1   (`rythme exercise list` shows them all)", "どの練習問題？ 例：rythme exercise start 1   （`rythme exercise list` で一覧を表示）"),
        ("which file? Example: rythme exercise check 01-values.ry", "どのファイル？ 例：rythme exercise check 01-values.ry"),
        ("unknown action `{other}` for `rythme exercise`", "`rythme exercise` に不明な操作 `{other}`"),
        (
            r#"
did you mean `rythme exercise {best}`?"#,
            r#"
もしかして `rythme exercise {best}`？"#,
        ),
        (
            r#"
the actions are: list, start NAME, check FILE (see `rythme help exercises`)"#,
            r#"
操作は list、start NAME、check FILE（`rythme help exercises` を参照）"#,
        ),
        ("{} exercises for a first course, from the first values to tests, from ★☆☆ (easy) to ★★★.", "入門コース向けの練習問題 {} 問。最初の値からテストまで、★☆☆（やさしい）から ★★★ まで。"),
        (
            r#"Start one:       rythme exercise start 1
Check your work: rythme exercise check {}.ry"#,
            r#"始める：            rythme exercise start 1
答えをチェックする：rythme exercise check {}.ry"#,
        ),
        ("there is no exercise `{name}`", "練習問題 `{name}` はない"),
        (
            r#"
the exercises are numbered from 1 to {}: `rythme exercise list` shows them"#,
            r#"
練習問題には 1 から {} までの番号が付いている：`rythme exercise list` で表示"#,
        ),
        ("`{file}` already exists here: open it to continue, or rename it to start again", "`{file}` はすでにここにある：続けるならそれを開き、最初からやり直すなら名前を変える"),
        ("cannot write `{file}`: {error}", "`{file}` を書き込めない：{error}"),
        (
            r#"Created {file}: exercise {} · {title}  {level}

Open it, read the statement and the hints at the top, and replace each
`fail("to do")` with your code. Then check your work:

  rythme exercise check {file}"#,
            r#"{file} を作成しました：練習問題 {} · {title}  {level}

ファイルを開いて先頭の問題とヒントを読み、それぞれの `fail("未完成")` を
自分のコードに置き換えてください。それから答えをチェックします：

  rythme exercise check {file}"#,
        ),
        ("file `{name}` not found", "ファイル `{name}` が見つからない"),
        (
            r#"
to start this exercise: rythme exercise start {}"#,
            r#"
この練習問題を始めるには：rythme exercise start {}"#,
        ),
        (
            r#"✗ The file has errors, so its tests could not run.
  Fix the first error above, then check again."#,
            r#"✗ ファイルにエラーがあるため、テストを実行できませんでした。
  上の最初のエラーを直してから、もう一度チェックしてください。"#,
        ),
        ("✗ This file has no tests. An exercise comes with its tests: is it the right file?", "✗ このファイルにはテストがありません。練習問題にはテストが付いています。正しいファイルですか？"),
        ("✓ Well done: all {total} tests pass!", "✓ よくできました：{total} 個のテストがすべて成功しました！"),
        (
            r#"  Next exercise: {title}  {level}
    rythme exercise start {}"#,
            r#"  次の練習問題：{title}  {level}
    rythme exercise start {}"#,
        ),
        ("  That was the last exercise: you have finished them all. Congratulations!", "  これが最後の練習問題でした：すべて解き終わりました。おめでとうございます！"),
        ("✗ All {total} tests pass, but...", "✗ {total} 個のテストはすべて成功しましたが……"),
        ("✗ {passed} of {total} tests pass. Keep going!", "✗ {total} 個中 {passed} 個のテストが成功。その調子です！"),
        (
            r#"  - The tests of the exercise were changed: the exercise is solved when its own tests
    pass. `rythme exercise start {number}` gives a fresh copy (rename your file first)."#,
            r#"  - 練習問題のテストが変更されています：練習問題が解けたことになるのは、
    元のテストが成功したときです。`rythme exercise start {number}` で
    新しいコピーが手に入ります（先に自分のファイルの名前を変えてください）。"#,
        ),
        ("  - Some functions are still to do: replace each `fail(\"to do\")` with your code.", "  - まだ未完成の関数があります：それぞれの `fail(\"未完成\")` を自分のコードに置き換えてください。"),
        (
            r#"  - Start with the first failure above: compare what you got with what was expected.
  - Stuck? Read the hints at the top of the file, or `rythme help NAME` for a function."#,
            r#"  - 上の最初の失敗から始めましょう：得られた値と期待した値を比べてください。
  - 行き詰まったら、ファイル先頭のヒントを読むか、関数について
    `rythme help NAME` を見てください。"#,
        ),
        // src/format.rs
        ("`{name}` could not be formatted safely, so it was left as it is", "`{name}` は安全に整形できなかったので、そのままにした"),
        ("this is a bug in Rythme, not in your program: please report it, with the file, at https://github.com/maxlestage/Rythme/issues", "これは Rythme のバグで、あなたのプログラムの誤りではありません：ファイルを添えて https://github.com/maxlestage/Rythme/issues に報告してください"),
        ("unknown option `{o}` for `rythme fmt`", "`rythme fmt` に不明なオプション `{o}`"),
        (
            r#"
see `rythme help fmt`"#,
            r#"
`rythme help fmt` を参照"#,
        ),
        (
            r#"which files? Example: rythme fmt hello.ry
  a folder formats all its .ry files: rythme fmt .
  - reads the code from the standard input
  --check only checks, for continuous integration
see `rythme help fmt`"#,
            r#"どのファイル？ 例：rythme fmt hello.ry
  フォルダーを指定すると、その中のすべての .ry ファイルを整形する：rythme fmt .
  - は標準入力からコードを読む
  --check はチェックだけする（継続的インテグレーション向け）
`rythme help fmt` を参照"#,
        ),
        ("needs formatting: {shown}", "整形が必要：{shown}"),
        ("cannot write the file `{shown}`: {e}", "ファイル `{shown}` を書き込めない：{e}"),
        ("formatted: {shown}", "整形済み：{shown}"),
        ("needs formatting: <stdin>", "整形が必要：<stdin>"),
        ("{changed} file(s) formatted, {unchanged} already formatted", "{changed} 個のファイルを整形、{unchanged} 個は整形済み"),
        ("✓ {unchanged} file(s) already formatted", "✓ {unchanged} 個のファイルは整形済み"),
        ("✗ {changed} file(s) to format, {unchanged} already formatted; `rythme fmt` (without --check) formats them", "✗ 整形が必要なファイル {changed} 個、整形済み {unchanged} 個。`rythme fmt`（--check なし）で整形できる"),
        ("✗ {errors} file(s) left as they are because of errors", "✗ エラーのため {errors} 個のファイルをそのままにした"),
        ("file or folder `{shown}` not found", "ファイルまたはフォルダー `{shown}` が見つからない"),
        ("did you mean `{}`?", "もしかして `{}`？"),
        // src/help.rs
        (
            r#"Rythme {v} — help

Commands
  rythme                    interactive mode (try code line by line)
  rythme file.ry            run a program
  rythme run --seed 42 f.ry run it with the same draws of random every time
  rythme check file.ry      find problems without running the program
  rythme fix file.ry        apply the safe fixes of those problems (--dry-run: only show them)
  rythme test file.ry       run the tests of a file (or of a folder)
  rythme fmt file.ry        give the code its official layout (--check: only check)
  rythme doc file.ry        write its documentation, in Markdown (--html: a web page)
  rythme steps file.ry      show the calculation step by step (fib(3) → fib(2) + fib(1) → ...)
  rythme debug file.ry      pause the program where you want and look at its values
  rythme exercise           exercises checked automatically (list, start, check)
  rythme new NAME           create a project (rythme.toml, src/, tests/)
  rythme install            download the dependencies of the project
  rythme add NAME           add a package of the index to the project (rythme search WORDS finds them)
  rythme learn              guided tour of the language, step by step
  rythme help TOPIC         help on a topic or a function
  rythme explain E003       explain an error code
  rythme help llms          the whole language in one text, for AI assistants
  rythme -e "code"          run a piece of code
  rythme lsp                language server, for editors (VS Code, Neovim...)
  rythme mcp                server for AI assistants (Model Context Protocol)
  option --lang CODE        language of the messages: en, fr, es, de, pt, zh, it, ja (or RYTHME_LANG)

Topics (rythme help TOPIC)
  start       the language in one minute
  syntax      the whole syntax on one page
  functions   functions, |>, recursion
  lists       lists and their functions
  text        texts
  records     records {{name: value}}
  dicts       dictionaries and sets: dict(...), set(...)
  types       your own types, and types written in the code
  match       choosing according to the shape of a value
  errors      Option, Result and ?
  functional  no loops: what to do instead
  tests       writing and running tests
  exercises   exercises to learn, with their tests
  modules     using several files
  system      files, folders, paths, options, input, exit codes
  projects    projects and packages: rythme new, rythme.toml, rythme install
  packages    the package index: rythme search, add, update, publish
  dates       dates and times: now(), today(), calendar, ISO 8601
  regex       searching, cutting and replacing with regular expressions
  csv         the tables of spreadsheets: csv_parse, csv_records, csv_format
  random      chance, and the same draws again with a seed
  ml          machine learning and AI: vectors, statistics, slopes, charts, HTTP
  repl        the interactive mode
  fmt         the official layout of the code (rythme fmt)
  doc         documentation from the ## comments (rythme doc)
  steps       the calculation shown step by step (rythme steps)
  debug       the debugger: breakpoints, line by line, values (rythme debug)
  editors     Rythme in your editor (VS Code, Neovim, Helix...)
  ruby        coming from Ruby: blocks, symbols, method chains
  builtins    every built-in function

A specific function: rythme help map
"#,
            r#"Rythme {v} — ヘルプ

コマンド
  rythme                    対話モード（コードを 1 行ずつ試す）
  rythme file.ry            プログラムを実行する
  rythme run --seed 42 f.ry random の結果を毎回同じにして実行する
  rythme check file.ry      プログラムを実行せずに問題を見つける
  rythme fix file.ry        それらの問題の安全な修正を適用する（--dry-run：表示だけ）
  rythme test file.ry       ファイル（またはフォルダー）のテストを実行する
  rythme fmt file.ry        コードを公式のレイアウトに整形する（--check：チェックだけ）
  rythme doc file.ry        ドキュメントを Markdown で書く（--html：Web ページ）
  rythme steps file.ry      計算を 1 ステップずつ表示する（fib(3) → fib(2) + fib(1) → ...）
  rythme debug file.ry      好きな場所でプログラムを一時停止し、値を調べる
  rythme exercise           自動でチェックされる練習問題（list、start、check）
  rythme new NAME           プロジェクトを作る（rythme.toml、src/、tests/）
  rythme install            プロジェクトの依存関係をダウンロードする
  rythme add NAME           インデックスのパッケージを追加（rythme search WORDS で探す）
  rythme learn              言語のガイドツアー（1 ステップずつ）
  rythme help TOPIC         トピックや関数のヘルプ
  rythme explain E003       エラーコードを説明する
  rythme help llms          AI アシスタント向けに、言語全体を 1 つのテキストで
  rythme -e "code"          コードの断片を実行する
  rythme lsp                エディター向けの言語サーバー（VS Code、Neovim...）
  rythme mcp                AI アシスタント向けのサーバー（Model Context Protocol）
  オプション --lang CODE    メッセージの言語：en、fr、es、de、pt、zh、it、ja（または RYTHME_LANG）

トピック（rythme help TOPIC）
  start       1 分でわかる Rythme
  syntax      構文のすべてを 1 ページで
  functions   関数、|>、再帰
  lists       リストとその関数
  text        テキスト
  records     レコード {{name: value}}
  dicts       辞書と集合：dict(...)、set(...)
  types       独自の型と、コードに書く型
  match       値の形に応じて選ぶ
  errors      Option、Result、?
  functional  ループなしでどうするか
  tests       テストを書いて実行する
  exercises   テスト付きの学習用練習問題
  modules     複数のファイルを使う
  system      ファイル、フォルダー、パス、オプション、入力、終了コード
  projects    プロジェクトとパッケージ：rythme new、rythme.toml、rythme install
  packages    パッケージインデックス：rythme search、add、update、publish
  dates       日付と時刻：now()、today()、カレンダー、ISO 8601
  regex       正規表現による検索、分割、置換
  csv         表計算ソフトの表：csv_parse、csv_records、csv_format
  random      乱数と、シードによる同じ結果の再現
  ml          機械学習と AI：ベクトル、統計、傾き、グラフ、HTTP
  repl        対話モード
  fmt         コードの公式のレイアウト（rythme fmt）
  doc         ## コメントから作るドキュメント（rythme doc）
  steps       1 ステップずつ表示される計算（rythme steps）
  debug       デバッガー：ブレークポイント、1 行ずつの実行、値（rythme debug）
  editors     エディターで使う Rythme（VS Code、Neovim、Helix...）
  ruby        Ruby から来た人へ：ブロック、シンボル、メソッドチェーン
  builtins    すべての組み込み関数

特定の関数について：rythme help map
"#,
        ),
        ("{alias}  (the Ruby name of `{of}`: the same function)", "{alias}  （`{of}` の Ruby での名前：同じ関数）"),
        (
            r#"pi
  The number π = 3.141592653589793."#,
            r#"pi
  数 π = 3.141592653589793。"#,
        ),
        ("The function:", "関数："),
        ("No help found for `{name}`.", "`{name}` のヘルプは見つかりませんでした。"),
        (
            r#"
Did you mean `{best}`?"#,
            r#"
もしかして `{best}`？"#,
        ),
        (
            r#"
See the list of topics with `rythme help`."#,
            r#"
トピックの一覧は `rythme help` で見られます。"#,
        ),
        ("keyword", "キーワード"),
        (
            r#"  More: rythme help {topic}
"#,
            r#"  詳しく：rythme help {topic}
"#,
        ),
        (
            r#"  Same as: {} (its Ruby names)
"#,
            r#"  {} と同じ（Ruby での名前）
"#,
        ),
        (
            r#"BUILT-IN FUNCTIONS
"#,
            r#"組み込み関数
"#,
        ),
        ("the number π", "数 π"),
        ("{} (same as {})", "{}（{} と同じ）"),
        ("  Ruby names:", "  Ruby での名前："),
        ("build an Option or a Result", "Option か Result を作る"),
        (
            r#"
Details and example for one function: rythme help NAME (or help(NAME) in code)
"#,
            r#"
1 つの関数の詳細と例：rythme help NAME（コードでは help(NAME)）
"#,
        ),
        (
            r#"help(x) explains x: a function (help(map)), a topic (help("lists")), or a value (help([1, 2])).
Topics: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
            r#"help(x) は x を説明します：関数（help(map)）、トピック（help("lists")）、値（help([1, 2])）。
トピック：start、syntax、functions、lists、text、records、types、match、errors、functional、tests、modules、builtins。"#,
        ),
        (
            r#"  (no documentation: write `## description` on the line above `fn`)
"#,
            r#"  （ドキュメントなし：`fn` の上の行に `## 説明` と書いてください）
"#,
        ),
        ("defined at", "定義場所"),
        (
            r#"{}({})
  A constructor of the type `{}`.
"#,
            r#"{}({})
  型 `{}` のコンストラクター。
"#,
        ),
        (
            r#"This is a value of type {ty}: {}
"#,
            r#"これは {ty} 型の値です：{}
"#,
        ),
        (
            r#"Useful functions: {}
"#,
            r#"便利な関数：{}
"#,
        ),
        (
            r#"Also: {also}
"#,
            r#"関連：{also}
"#,
        ),
        (
            r#"Use `match` to handle each variant of `{}`.
"#,
            r#"`{}` の各バリアントを扱うには `match` を使います。
"#,
        ),
        ("[Enter] next   [q] quit > ", "[Enter] 次へ   [q] 終了 > "),
        ("To practice, run `rythme` and type code, or do the exercises: `rythme exercise list`. `rythme help builtins` lists every function.", "練習するには、`rythme` を起動してコードを入力するか、練習問題に取り組みましょう：`rythme exercise list`。`rythme help builtins` ですべての関数を一覧できます。"),
        // src/interp.rs
        ("record", "レコード"),
        ("`:{field}` is a symbol, not a text: write `{r}.{field}` or `{r}[\"{field}\"]`, or `get({r}, \"{field}\")` to receive an Option", "`:{field}` はテキストではなくシンボル：`{r}.{field}` か `{r}[\"{field}\"]` と書くか、Option を受け取るなら `get({r}, \"{field}\")` を使う"),
        ("the dict has the key {shown}, which is not the same key as {key}: write `[{shown}]`", "辞書のキーは {shown} で、{key} とは別のキー：`[{shown}]` と書く"),
        ("this list would be too large (more than {MAX_LIST} elements)", "このリストは大きくなりすぎる（{MAX_LIST} 要素を超える）"),
        ("this text would be too large (more than {mb} MB)", "このテキストは大きくなりすぎる（{mb} MB を超える）"),
        ("does it grow in a function that calls itself? Check that the function always reaches a case where it stops", "自分自身を呼ぶ関数の中で大きくなっていないか？ 関数が停止するケースに必ず到達するか確かめる"),
        ("no function named `{}`", "`{}` という名前の関数はない"),
        ("`value.{0}(...)` calls `{0}(value, ...)`", "`value.{0}(...)` は `{0}(value, ...)` を呼び出す"),
        ("`{name}` is not defined yet", "`{name}` はまだ定義されていない"),
        ("the file runs from top to bottom: move this definition above the line that uses it (directly or through a function)", "ファイルは上から下へ実行される：この定義を、それを（直接または関数を通して）使う行より上に移す"),
        ("`{ctor}` has no field `{name}`", "`{ctor}` にフィールド `{name}` はない"),
        ("`{ctor}` has no fields", "`{ctor}` にはフィールドがない"),
        ("the fields of `{ctor}` are: {}", "`{ctor}` のフィールドは次のとおり：{}"),
        ("a value of type `{}` may be another variant: use `match` to handle each case", "`{}` 型の値は別のバリアントかもしれない：`match` を使って各ケースを扱う"),
        ("too many nested calls ({depth} calls in progress)", "入れ子の呼び出しが多すぎる（実行中の呼び出しが {depth} 個）"),
        ("this call goes too deep", "この呼び出しは深くなりすぎる"),
        (
            r#"a recursive function that never stops? Check that it always reaches a base case.
Otherwise, make the recursive call the very last thing the function does
(tail call), for example with an accumulator:
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
or use `fold`, `map`, `filter`, which have no limit."#,
            r#"止まらない再帰関数？ 必ず基底ケースに到達するか確かめる。
そうでなければ、再帰呼び出しを関数が最後に行う処理（末尾呼び出し）にする。
例えば累積引数を使って：
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
または制限のない `fold`、`map`、`filter` を使う。"#,
        ),
        ("the program was stopped: it made more calls than allowed for this run", "プログラムを停止した：この実行で許された回数より多く呼び出しを行った"),
        ("does it call itself forever? Check that it always reaches a base case", "自分自身を永遠に呼んでいないか？ 必ず基底ケースに到達するか確かめる"),
        ("{} is not a function", "{} は関数ではない"),
        ("`{0}` is a value, not a function: write `{0}` without parentheses", "`{0}` は関数ではなく値：`{0}` は括弧なしで書く"),
        ("`..` expects {what} here, but got {}", "ここで `..` には{what}が必要だが、{} を受け取った"),
        ("division by zero", "ゼロ除算"),
        (
            r#"check the divisor first: `if b == 0 then ... else a / b`,
or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`"#,
            r#"先に割る数を確かめる：`if b == 0 then ... else a / b`
または Result を返す：`if b == 0 then Err("ゼロ除算") else Ok(a / b)`"#,
        ),
        ("`0 ** -n` is `1 / 0 ** n`", "`0 ** -n` は `1 / 0 ** n`"),
        ("{min} argument{s}", "{min} 個の引数"),
        ("at least {min} argument{s}", "{min} 個以上の引数"),
        ("{min} to {max} arguments", "{min}〜{max} 個の引数"),
        ("{got} {verb}", "{got}"),
        ("`{name}` takes {expected}, but {given}", "`{name}` は {expected}を取るが、受け取ったのは {given} 個"),
        ("{got} argument(s) here", "ここの引数は {got} 個"),
        ("usage: `{sig}`", "使い方：`{sig}`"),
        ("cannot use `{sym}` with {lt} and {rt}", "`{sym}` は {lt} と {rt} には使えない"),
        ("`nan` (not a number) cannot be compared", "`nan`（非数）は比較できない"),
        ("a negative number ({}) to a fractional power ({}) has no value among real numbers", "負の数（{}）の分数乗（{}）は実数の範囲に値がない"),
        ("use `abs(x) ** p` for the power of its absolute value", "絶対値のべき乗なら `abs(x) ** p` を使う"),
        ("this range is too large ({} elements)", "この範囲は大きすぎる（{} 要素）"),
        ("ranges are lists: keep them under {MAX_RANGE} elements", "範囲はリスト：{MAX_RANGE} 要素未満にする"),
        ("a range needs whole numbers, but got {} and {}", "範囲には整数が必要だが、{} と {} を受け取った"),
        ("index {n} is out of range: {what}", "インデックス {n} は範囲外：{what}"),
        ("valid indices go from 0 to {} (or -1 for the last element)", "有効なインデックスは 0 から {} まで（最後の要素は -1）"),
        ("to get an Option instead of an error, use `get(x, {n})`", "エラーではなく Option を得るには `get(x, {n})` を使う"),
        ("the text is empty", "テキストが空"),
        ("the text has {len} character(s)", "テキストは {len} 文字"),
        ("the list has {} element(s)", "リストの要素は {} 個"),
        ("this dict has no key {}", "この辞書にキー {} はない"),
        ("use `get(dict, key)` to receive an Option instead, or `has(dict, key)` to check first", "代わりに Option を受け取るには `get(dict, key)` を、先に確かめるには `has(dict, key)` を使う"),
        // src/lexer.rs
        ("`...` is not an operator in Rythme", "`...` は Rythme の演算子ではない"),
        ("write `1..10` for a range (10 included), or `[first, ..rest]` in a list", "範囲なら `1..10`（10 を含む）、リストの中なら `[first, ..rest]` と書く"),
        ("`++` is not an operator in Rythme", "`++` は Rythme の演算子ではない"),
        (
            r#"to add one, write `x + 1` and give the result a new name (values never change);
to join two lists or two texts, use `+`: `[1] + [2]`"#,
            r#"1 を足すには `x + 1` と書き、結果に新しい名前を付ける（値は変わらない）。
2 つのリストやテキストをつなげるには `+` を使う：`[1] + [2]`"#,
        ),
        ("unexpected character `&`", "予期しない文字 `&`"),
        ("use `and` to combine two conditions", "2 つの条件を組み合わせるには `and` を使う"),
        ("this `{{` in a text is never closed", "テキスト中のこの `{{` が閉じられていない"),
        ("starts a value inserted in the text", "テキストに埋め込む値の始まり"),
        ("close it with `}}`, or write `\\{{` to get a literal brace: \"\\{{\"", "`}}` で閉じるか、波かっこそのものなら `\\{{` と書く：\"\\{{\""),
        ("unexpected invisible character {code}", "予期しない見えない文字 {code}"),
        ("this is a special space (often copied from a document or a web page): replace it with a normal space", "これは特殊な空白（文書や Web ページからコピーされることが多い）：普通の空白に置き換える"),
        ("this character cannot be seen (it is often copied from a document or a web page): delete it", "この文字は見えない（文書や Web ページからコピーされることが多い）：削除する"),
        ("unexpected character `{c}`", "予期しない文字 `{c}`"),
        ("write `{s}` instead", "代わりに `{s}` と書く"),
        ("texts use double quotes: \"hello\"", "テキストには二重引用符を使う：\"hello\""),
        ("use `**` for powers: `2 ** 10`", "べき乗には `**` を使う：`2 ** 10`"),
        ("use straight double quotes `\"` for texts", "テキストにはまっすぐな二重引用符 `\"` を使う"),
        ("to insert a value in a text, use braces: \"total: {{x}}\"", "テキストに値を埋め込むには波かっこを使う：\"total: {{x}}\""),
        ("invalid number `{text}`", "無効な数値 `{text}`"),
        ("hexadecimal numbers use the digits 0 to 9 and a to f, like `0xff`; binary numbers use 0 and 1, like `0b101`", "16 進数は 0〜9 と a〜f の数字を使う（`0xff` など）。2 進数は 0 と 1 を使う（`0b101` など）"),
        ("a name cannot start with a digit: `{}`", "名前は数字で始められない：`{}`"),
        ("to multiply, write it explicitly: `{num} * {word}`", "掛け算は明示的に書く：`{num} * {word}`"),
        ("texts are nested too deeply inside `{{...}}`", "`{{...}}` の中のテキストの入れ子が深すぎる"),
        ("this text is never closed", "このテキストが閉じられていない"),
        ("the text starts here", "テキストはここから始まる"),
        ("add a `\"` at the end of the text", "テキストの終わりに `\"` を加える"),
        ("invalid unicode escape", "無効な Unicode エスケープ"),
        ("write it like `\\u{{e9}}` for é", "é なら `\\u{{e9}}` のように書く"),
        ("unknown escape sequence in text", "テキスト中の不明なエスケープシーケンス"),
        ("in a regex, write the backslash twice: \"\\\\{c}\" is the text \\{c}", "正規表現ではバックスラッシュを 2 回書く：\"\\\\{c}\" がテキスト \\{c} になる"),
        (
            r#"valid escapes: \n (new line), \t (tab), \r (carriage return), \0 (null character), \\ (backslash),
\" (quote), \{{ and \}} (braces), \# (a `#` before a brace), \u{{e9}} (a character by its Unicode number: é)"#,
            r#"有効なエスケープ：\n（改行）、\t（タブ）、\r（復帰）、\0（ヌル文字）、\\（バックスラッシュ）、
\"（引用符）、\{{ と \}}（波かっこ）、\#（波かっこの前の `#`）、\u{{e9}}（Unicode 番号で表す文字：é）"#,
        ),
        ("inside it, this text is never closed", "その中で、このテキストが閉じられていない"),
        ("empty `{{}}` in a text", "テキスト中に空の `{{}}`"),
        ("put a value inside, like \"{{name}}\", or write `\\{{` for a literal brace", "\"{{name}}\" のように中に値を入れるか、波かっこそのものなら `\\{{` と書く"),
        ("inside a text, `{{` starts an inserted value: to write braces, use `\\{{` and `\\}}`, like \"\\{{\\\"a\\\": 1\\}}\"", "テキストの中では `{{` は埋め込む値の始まり：波かっこを書くには `\\{{` と `\\}}` を使う。例：\"\\{{\\\"a\\\": 1\\}}\""),
        ("the number `{n}`", "数値 `{n}`"),
        ("the number `{f}`", "数値 `{f}`"),
        ("the name `{s}`", "名前 `{s}`"),
        ("the symbol `:{s}`", "シンボル `:{s}`"),
        ("the end of the line", "行の終わり"),
        ("the end of the code", "コードの終わり"),
        ("the keyword `{k}`", "キーワード `{k}`"),
        // src/lsp/analysis.rs
        ("cannot read the file `{shown}`", "ファイル `{shown}` を読めない"),
        ("the file `{p}` has {n} error(s)", "ファイル `{p}` にエラーが {n} 件ある"),
        ("first: {}{place}", "最初のエラー：{}{place}"),
        ("open `{p}` to see and fix them", "`{p}` を開いて確認し、修正する"),
        // src/lsp/extras.rs
        ("▶ Run", "▶ 実行"),
        ("▶ Run the {n} tests", "▶ {n} 個のテストを実行"),
        ("▶ Run test", "▶ テストを実行"),
        // src/lsp/features.rs
        (
            r#"
more: `rythme explain {}`"#,
            r#"
詳しく：`rythme explain {}`"#,
        ),
        (
            r#"

Same as: {} (its Ruby names)"#,
            r#"

{} と同じ（Ruby での名前）"#,
        ),
        ("**{alias}** — the Ruby name of `{of}`: the same function", "**{alias}** — `{of}` の Ruby での名前：同じ関数"),
        ("A value that is there, in an `Option`. The other case is `None`.", "`Option` の中にある値。もう一方のケースは `None`。"),
        ("No value, in an `Option`: what `find`, `first` or `get` give when there is nothing. Handle it with `match` or `unwrap_or`.", "`Option` で値がないこと：何もないときに `find`、`first`、`get` が返すもの。`match` か `unwrap_or` で扱う。"),
        ("The result of an operation that succeeded, in a `Result`. The other case is `Err(error)`.", "`Result` の中の、成功した操作の結果。もう一方のケースは `Err(error)`。"),
        ("The result of an operation that failed, in a `Result`. Inside a function, `?` stops and passes it on.", "`Result` の中の、失敗した操作の結果。関数の中では `?` がそこで止まり、それを呼び出し元に渡す。"),
        ("**{word}** is not used in Rythme.", "**{word}** は Rythme では使わない。"),
        ("{} more definitions", "ほか {} 個の定義"),
        ("Inferred from its code:", "コードから推論："),
        ("local function", "ローカル関数"),
        ("parameter of `{f}`", "`{f}` の引数"),
        ("parameter of an anonymous function", "無名関数の引数"),
        ("local value", "ローカルな値"),
        ("given by the case `{pattern}` of a `match`", "`match` のケース `{pattern}` で与えられる"),
        ("constructor of the type `{t}`", "型 `{t}` のコンストラクター"),
        ("*defined in `{name}`*", "*`{name}` で定義*"),
        ("The number π.", "数 π。"),
        (
            r#"

`value.{n}(...)` calls `{n}(value, ...)`, or the field `{n}` of a record; `value.{n}` reads the field `{n}`, or calls `{n}(value)`"#,
            r#"

`value.{n}(...)` は `{n}(value, ...)` かレコードのフィールド `{n}` を呼び出す。`value.{n}` はフィールド `{n}` を読むか、`{n}(value)` を呼び出す"#,
        ),
        ("the file `{p}` is not found", "ファイル `{p}` が見つからない"),
        (
            r#"

Its definitions are used through the name `{n}`: `{n}.name`"#,
            r#"

定義は名前 `{n}` を通して使う：`{n}.name`"#,
        ),
        ("Defines:", "定義しているもの："),
        ("name", "名前"),
        ("value", "値"),
        ("other", "その他"),
        ("pattern", "パターン"),
        ("result", "結果"),
        ("what it checks", "チェックする内容"),
        ("actual", "実際の値"),
        ("expected", "期待値"),
        ("condition", "条件"),
        ("file", "ファイル"),
        ("a function on several lines", "複数行の関数"),
        ("a function on one line", "1 行の関数"),
        ("choose according to the shape of a value", "値の形に応じて選ぶ"),
        ("a test, run by `rythme test`", "`rythme test` で実行するテスト"),
        ("a test on several lines", "複数行のテスト"),
        ("choose between two values", "2 つの値から選ぶ"),
        ("type ${{1:Name}} = ${{2:First}}(${{3:field}}) | ${{0:Second}}", "type ${{1:Name}} = ${{2:First}}(${{3:field}}) | ${{0:Second}}"),
        ("your own type", "独自の型"),
        ("several steps inside an expression", "式の中の複数のステップ"),
        ("a block: a function given as the last argument of a call", "ブロック：呼び出しの最後の引数として渡す関数"),
        ("the opposite of `if`", "`if` の逆"),
        ("use another file: `use geometry`, then `geometry.area(2)`", "別のファイルを使う：`use geometry` の後に `geometry.area(2)`"),
        ("same as {}", "{} と同じ"),
        ("field", "フィールド"),
        ("package", "パッケージ"),
        ("folder", "フォルダー"),
        ("Add `use {file}` and write `{stem}.{written}`", "`use {file}` を加えて `{stem}.{written}` と書く"),
        ("Replace with `{new}`", "`{new}` に置き換える"),
        // src/lsp/mod.rs
        ("rythme lsp: unknown option `{other}` (ignored)", "rythme lsp：不明なオプション `{other}`（無視する）"),
        ("rythme lsp is the language server: your editor starts it and talks to it. See `rythme help editors`. (Ctrl-C to quit)", "rythme lsp は言語サーバーです：エディターがこれを起動して通信します。`rythme help editors` を参照してください。（終了は Ctrl-C）"),
        ("the server is not initialized yet", "サーバーはまだ初期化されていない"),
        ("the server is shutting down", "サーバーは終了処理中"),
        ("internal error in Rythme (please report it): {}", "Rythme の内部エラー（報告をお願いします）：{}"),
        ("not formatted: the code has an error ({first})", "整形していない：コードにエラーがある（{first}）"),
        // src/lsp/refs.rs
        ("there is no name here", "ここに名前はない"),
        ("the name of a test is a text: edit it directly", "テストの名前はテキスト：直接編集する"),
        ("`{name}` cannot be found", "`{name}` が見つからない"),
        ("`{name}` is a built-in function: it cannot be renamed", "`{name}` は組み込み関数：名前を変えられない"),
        ("`{name}` is part of Rythme: it cannot be renamed", "`{name}` は Rythme に組み込まれている：名前を変えられない"),
        ("`{name}` is not defined: there is nothing to rename", "`{name}` は定義されていない：名前を変えるものがない"),
        ("`{new}` is not a valid name", "`{new}` は有効な名前ではない"),
        ("`{new}` is a keyword", "`{new}` はキーワード"),
        ("the names of types and constructors start with an uppercase letter", "型とコンストラクターの名前は大文字で始まる"),
        ("names start with a lowercase letter (uppercase is for types and constructors)", "名前は小文字で始まる（大文字は型とコンストラクター用）"),
        ("`{new}` is a built-in function: choose another name", "`{new}` は組み込み関数：別の名前を選ぶ"),
        ("`{shown}` has a syntax error: fix it before renaming", "`{shown}` に構文エラーがある：名前を変える前に直す"),
        ("`{old_name}` is also the name of a record field, and `value.{old_name}(...)` may call that field: rename it by hand", "`{old_name}` はレコードのフィールドの名前でもあり、`value.{old_name}(...)` はそのフィールドを呼び出すかもしれない：手で名前を変える"),
        ("cannot rename `{old_name}` to `{new_name}`: {why}", "`{old_name}` の名前を `{new_name}` に変えられない：{why}"),
        ("some names would then refer to something else", "いくつかの名前が別のものを指すようになってしまう"),
        ("the code would not be valid", "コードが正しくなくなってしまう"),
        // src/manifest.rs
        ("the section `[{name}]` is written twice", "セクション `[{name}]` が 2 回書かれている"),
        ("put all the lines of `[{name}]` under one `[{name}]`", "`[{name}]` の行をすべて 1 つの `[{name}]` の下にまとめる"),
        ("`{name}` is written both as `[{name}]` and as `[[{name}]]`", "`{name}` が `[{name}]` としても `[[{name}]]` としても書かれている"),
        ("expected a name here", "ここには名前が必要"),
        ("each line is `name = \"value\"`, like `version = \"0.1.0\"`", "各行は `name = \"value\"` の形。例：`version = \"0.1.0\"`"),
        ("this text is not closed", "このテキストが閉じられていない"),
        ("add `{quote}` at the end of the text", "テキストの終わりに `{quote}` を加える"),
        ("unknown escape in a text", "テキスト中の不明なエスケープ"),
        ("write `\\\\` for a backslash, or use `/` in paths: they work on every system", "バックスラッシュは `\\\\` と書くか、パスでは `/` を使う（どのシステムでも動く）"),
        ("expected `=` after `{key}`", "`{key}` の後には `=` が必要"),
        ("`{key}` is written twice", "`{key}` が 2 回書かれている"),
        ("expected `,` or `}}` here", "ここには `,` か `}}` が必要"),
        ("a table is written on one line: `{{ git = \"...\", tag = \"...\" }}`", "テーブルは 1 行で書く：`{{ git = \"...\", tag = \"...\" }}`"),
        ("a value is a text, written in quotes", "値は引用符で囲んだテキスト"),
        ("write `\"{example}\"`", "`\"{example}\"` と書く"),
        ("expected `{close}` here", "ここには `{close}` が必要"),
        ("write `{key} = \"...\"`", "`{key} = \"...\"` と書く"),
        ("first written here", "最初はここに書かれている"),
        ("unexpected text after the value", "値の後に予期しないテキスト"),
        ("write one value per line; a comment starts with `#`", "1 行に 1 つの値を書く。コメントは `#` で始める"),
        ("`{name}` cannot be the name of a package: it is used in `use {name}/file`, so it must be a Rythme name in lower case (letters, digits, `_`)", "`{name}` はパッケージの名前にできない：`use {name}/file` で使うので、小文字の Rythme の名前（英字、数字、`_`）でなければならない"),
        ("`{key}` is {what}, written in quotes", "`{key}` は引用符で囲んだ{what}"),
        ("unknown key `{key}`", "不明なキー `{key}`"),
        ("the keys here are {}", "ここで使えるキーは {}"),
        ("write `name = \"{b}\"`", "`name = \"{b}\"` と書く"),
        ("`{v}` is not a version", "`{v}` はバージョンではない"),
        ("a version is three numbers: `version = \"1.0.0\"` (major.minor.patch)", "バージョンは 3 つの数：`version = \"1.0.0\"`（major.minor.patch）"),
        ("`entry` is the path of a `.ry` file in the project", "`entry` はプロジェクトの `.ry` ファイルのパス"),
        ("like `entry = \"src/main.ry\"`", "例：`entry = \"src/main.ry\"`"),
        ("unknown section `{n}`", "不明なセクション `{n}`"),
        ("write `name`, `version` and `entry` at the top of the file, without a section", "`name`、`version`、`entry` はファイルの先頭にセクションなしで書く"),
        ("the only section is `[dependencies]`", "セクションは `[dependencies]` だけ"),
        ("`{shown}` has no `name`", "`{shown}` に `name` がない"),
        ("add a line like `name = \"hello\"` at the top", "先頭に `name = \"hello\"` のような行を加える"),
        ("`{shown}` has no `version`", "`{shown}` に `version` がない"),
        ("add a line like `version = \"0.1.0\"` at the top", "先頭に `version = \"0.1.0\"` のような行を加える"),
        ("write `{name} = {{ version = \"^1.0.0\" }}` (from the package index), `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (or `commit = \"...\"`), or `{name} = {{ path = \"../folder\" }}`", "`{name} = {{ version = \"^1.0.0\" }}`（パッケージインデックスから）、`{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}`（または `commit = \"...\"`）、`{name} = {{ path = \"../folder\" }}` のどれかを書く"),
        ("call it `{b}`", "`{b}` という名前にする"),
        ("the project `{own}` cannot depend on itself", "プロジェクト `{own}` は自分自身に依存できない"),
        ("`use {own}/file` already reaches its own files", "`use {own}/file` で自分のファイルにはすでに届く"),
        ("the dependency `{name}` is written as a table", "依存関係 `{name}` はテーブルとして書く"),
        ("the dependency `{name}` has both `git` and `path`", "依存関係 `{name}` に `git` と `path` の両方がある"),
        ("the dependency `{name}` needs `version`, `git` or `path`", "依存関係 `{name}` には `version`、`git`、`path` のどれかが必要"),
        ("`tag` and `commit` are for a dependency from git, not from a folder", "`tag` と `commit` は git からの依存関係用で、フォルダーからの依存関係には使えない"),
        ("`{c}` is not a commit", "`{c}` はコミットではない"),
        ("give a `tag` or a `commit`, not both", "`tag` か `commit` のどちらか一方を指定する"),
        ("the dependency `{name}` needs a fixed version: a `tag` or a `commit`", "依存関係 `{name}` には固定のバージョン（`tag` か `commit`）が必要"),
        ("the `git` address of `{name}` is empty", "`{name}` の `git` アドレスが空"),
        ("this file is written by `rythme install`: delete it and run `rythme install` again", "このファイルは `rythme install` が書くもの：削除して `rythme install` をもう一度実行する"),
        ("unexpected line in `rythme.lock`", "`rythme.lock` に予期しない行"),
        ("this entry of `rythme.lock` is not complete", "`rythme.lock` のこの項目は不完全"),
        (
            r#"# The exact commit of each dependency from git, written by `rythme install`.
# Keep this file with the project, so that everyone gets the same code.
"#,
            r#"# git からの各依存関係の正確なコミット。`rythme install` が書きます。
# 全員が同じコードを使えるよう、このファイルはプロジェクトと一緒に保管します。
"#,
        ),
        // src/parser.rs
        (
            r#"a case of `match` gives one expression; for a result on several lines, write `do` after `->`, then the lines, then `end`:
  Ok(x) -> do
    y = x + 1
    y * 2
  end
or move the lines into a function: `Ok(x) -> handle(x)`"#,
            r#"`match` のケースは 1 つの式を返す。複数行の結果には、`->` の後に `do`、その行、最後に `end` を書く：
  Ok(x) -> do
    y = x + 1
    y * 2
  end
または行を関数に移す：`Ok(x) -> handle(x)`"#,
        ),
        ("`{name}` is read as a value here, not called, because the code gives it a value (line {line}): to call it, put its argument in parentheses, like `{name}([1, 2])`", "コードが `{name}` に値を与えているので（{line} 行目）、ここでは `{name}` は呼び出しではなく値として読まれる：呼び出すには `{name}([1, 2])` のように引数を括弧に入れる"),
        ("Rythme has no `condition ? a : b`", "Rythme に `condition ? a : b` はない"),
        ("`?` does not choose between two values", "`?` は 2 つの値から選ぶものではない"),
        ("write `if condition then a else b`, like `if x > 2 then \"big\" else \"small\"`", "`if condition then a else b` と書く。例：`if x > 2 then \"big\" else \"small\"`"),
        ("after a value, `?` takes it out of `Some(x)` or `Ok(x)` and stops on `None` or `Err(e)`; at the end of a name, it is part of the name (`even?`)", "値の後の `?` は `Some(x)` や `Ok(x)` から値を取り出し、`None` や `Err(e)` ならそこで止まる。名前の末尾では名前の一部になる（`even?`）"),
        ("all the definitions of `{name}` must have the same number of parameters", "`{name}` の定義はすべて同じ数の引数を持たなければならない"),
        ("{} parameter(s) here", "ここの引数は {} 個"),
        ("expected {expected}, found {found}", "{expected}があるはずだが、{found} が見つかった"),
        ("expected {expected} here", "ここには{expected}が必要"),
        ("to compare two values, use `==`", "2 つの値を比べるには `==` を使う"),
        ("use `->` in `match` cases; a dict is built with `dict([[key, value], ...])`", "`match` のケースには `->` を使う。辞書は `dict([[key, value], ...])` で作る"),
        ("use `and` to combine conditions", "条件を組み合わせるには `and` を使う"),
        ("use `or` to combine conditions", "条件を組み合わせるには `or` を使う"),
        ("use `not` to negate a condition: `not done`", "条件を否定するには `not` を使う：`not done`"),
        ("Rythme does not use braces for blocks: write `do` ... `end`, or put a function body on the following lines and finish with `end`", "Rythme はブロックに波かっこを使わない：`do` ... `end` と書くか、関数の本体を次の行から書いて `end` で終える"),
        ("Rythme does not use `:` to start a block; see `rythme help syntax`", "Rythme はブロックを始めるのに `:` を使わない。`rythme help syntax` を参照"),
        ("`{word}` is not used in Rythme", "`{word}` は Rythme では使わない"),
        ("comes from another language", "別の言語のもの"),
        ("unexpected here: {found}", "ここでは予期しないもの：{found}"),
        ("the statement should end before this", "文はこの前で終わるはず"),
        ("to call a function, use parentheses: `{name}(...)`", "関数を呼ぶには括弧を使う：`{name}(...)`"),
        ("a block `do |x| ... end` is given to a call, on its line: `names.each do |name| print(name) end`", "ブロック `do |x| ... end` は同じ行の呼び出しに渡す：`names.each do |name| print(name) end`"),
        ("`else` must follow `if condition then value` (an `if` branch holds one expression; use `do ... end` for several lines)", "`else` は `if condition then value` の後に続く（`if` の分岐は 1 つの式。複数行なら `do ... end` を使う）"),
        ("put each statement on its own line", "文は 1 行に 1 つずつ書く"),
        ("`{word}` can only be used at the top level of a file", "`{word}` はファイルの最上位でしか使えない"),
        ("move it outside of the function or block", "関数やブロックの外に移す"),
        ("before `=` comes a name or a pattern, like `[first, ..rest]`, `{{name, ..}}` or `Some(x)`", "`=` の前には名前かパターンを書く。例：`[first, ..rest]`、`{{name, ..}}`、`Some(x)`"),
        ("`=`", "`=`"),
        ("values cannot be changed in Rythme", "Rythme では値を変更できない"),
        ("a record pattern cannot give a name to the other fields", "レコードのパターンでは残りのフィールドに名前を付けられない"),
        ("write `{{name, ..}}` to ignore the other fields; for a record without some fields, use `remove(record, \"name\")`", "残りのフィールドを無視するには `{{name, ..}}` と書く。一部のフィールドを除いたレコードには `remove(record, \"name\")` を使う"),
        ("only a function can have a name that ends with `?`", "名前が `?` で終わってよいのは関数だけ"),
        ("define it with `fn`: `fn {s}(x) = ...`; for a value, use a name without `?`", "`fn` で定義する：`fn {s}(x) = ...`。値なら `?` なしの名前を使う"),
        ("invalid `..` in this pattern", "このパターンの `..` は無効"),
        ("`..name` must be last: `[first, ..rest]`", "`..name` は最後に置く：`[first, ..rest]`"),
        ("cannot give a value to this", "これに値を与えることはできない"),
        ("this is not a name", "これは名前ではない"),
        ("`=` gives a name to a value", "`=` は値に名前を付ける"),
        ("to define a function, write `fn name(x) = ...`", "関数を定義するには `fn name(x) = ...` と書く"),
        ("records cannot be changed; build a new one: `new = {{..old, {name}: value}}`", "レコードは変更できない。新しいレコードを作る：`new = {{..old, {name}: value}}`"),
        ("values cannot be changed; build a new one: `set_at(list, i, x)` for a list, `put(record, name, x)` for a record", "値は変更できない。新しい値を作る：リストなら `set_at(list, i, x)`、レコードなら `put(record, name, x)`"),
        ("a function name", "関数名"),
        ("`(` and the parameters", "`(` と引数"),
        ("even without parameters, write the parentheses: `fn greet() = print(\"hi\")`", "引数がなくても括弧は書く：`fn greet() = print(\"hi\")`"),
        ("functions use `=`, not `{arrow}`", "関数には `{arrow}` ではなく `=` を使う"),
        ("write `fn(x) = x * 2`", "`fn(x) = x * 2` と書く"),
        ("`=` or a new line", "`=` か改行"),
        (
            r#"a function is written `fn name(x) = expression`,
or with its body on the following lines, finished by `end`"#,
            r#"関数は `fn name(x) = expression` と書くか、
本体を次の行から書いて `end` で終える"#,
        ),
        ("type names start with an uppercase letter", "型名は大文字で始まる"),
        ("write `type {cap} = ...`", "`type {cap} = ...` と書く"),
        ("a type name", "型名"),
        ("example: `type Shape = Circle(radius) | Square(side)`", "例：`type Shape = Circle(radius) | Square(side)`"),
        ("variant names start with an uppercase letter", "バリアント名は大文字で始まる"),
        ("write `{cap}`", "`{cap}` と書く"),
        ("a variant name, like `Circle`", "`Circle` のようなバリアント名"),
        ("a field name", "フィールド名"),
        ("fields are named in lowercase: `Circle(radius)`", "フィールド名は小文字で書く：`Circle(radius)`"),
        ("the name of the test, as a text", "テキストで書いたテストの名前"),
        ("example: `test \"addition works\" = assert_eq(1 + 1, 2)`", "例：`test \"足し算が正しい\" = assert_eq(1 + 1, 2)`"),
        ("write the name of the file without `.ry`", "ファイル名を `.ry` なしで書く"),
        ("example: `use {path}`", "例：`use {path}`"),
        ("a name for the file, in lowercase", "ファイルに付ける小文字の名前"),
        ("example: `use geometry as geo`, then `geo.area(2)`", "例：`use geometry as geo` の後に `geo.area(2)`"),
        ("the name of a file", "ファイル名"),
        ("example: `use geometry` for the file `geometry.ry`, then `geometry.area(2)`", "例：ファイル `geometry.ry` なら `use geometry` の後に `geometry.area(2)`"),
        ("for a file whose name starts with an uppercase letter, write its path as a text: `use \"Shapes.ry\" as shapes`", "名前が大文字で始まるファイルは、パスをテキストで書く：`use \"Shapes.ry\" as shapes`"),
        ("the end of the expression", "式の終わり"),
        ("comparisons cannot be chained", "比較を連ねることはできない"),
        ("combine them with `and`: `a < b and b < c`", "`and` で組み合わせる：`a < b and b < c`"),
        ("`{sym}` cannot follow a `|>` step directly", "`|>` のステップの直後に `{sym}` は置けない"),
        (
            r#"`|>` comes after the calculations: put the pipeline in parentheses,
like `(x |> f) {sym} 1`, or continue it with a function: `x |> f |> fn(v) = v {sym} 1`"#,
            r#"`|>` は計算の後に来る：パイプラインを括弧で囲んで `(x |> f) {sym} 1` とするか、
関数で続ける：`x |> f |> fn(v) = v {sym} 1`"#,
        ),
        ("a block must follow a call", "ブロックは呼び出しの後に置く"),
        ("not given to a function", "関数に渡されていない"),
        (
            r#"write the block on the line of the call it is given to: `xs.map do |x| x * 2 end`;
for a function as a value, write `fn(x) = x * 2`"#,
            r#"ブロックは渡す先の呼び出しと同じ行に書く：`xs.map do |x| x * 2 end`。
関数を値として使うなら `fn(x) = x * 2` と書く"#,
        ),
        ("`,` or `|`", "`,` か `|`"),
        ("the parameters of a block go between bars: `do |name, age| ... end`", "ブロックの引数は縦棒で囲む：`do |name, age| ... end`"),
        ("`}}`", "`}}`"),
        ("the block starts here", "ブロックはここから始まる"),
        ("a field or function name after `.`", "`.` の後のフィールド名か関数名"),
        ("an expression", "式"),
        ("`]`", "`]`"),
        ("`,` or `{closing}`", "`,` か `{closing}`"),
        ("separate the elements with commas", "要素はカンマで区切る"),
        ("this code is nested too deeply (more than {MAX_NESTING} levels)", "このコードは入れ子が深すぎる（{MAX_NESTING} 段を超える）"),
        ("split it into several named values or functions", "名前を付けた複数の値や関数に分ける"),
        ("this expression is too long or nested too deeply (more than {MAX_DEPTH} levels)", "この式は長すぎるか、入れ子が深すぎる（{MAX_DEPTH} 段を超える）"),
        (
            r#"split it into several named values or functions;
to combine many values, put them in a list: `sum([a, b, c])`, `join([x, y, z])`"#,
            r#"名前を付けた複数の値や関数に分ける。
多くの値をまとめるならリストに入れる：`sum([a, b, c])`、`join([x, y, z])`"#,
        ),
        ("Rythme has no tuples", "Rythme にタプルはない"),
        ("group values in a list `[a, b]` or a record `{{x: a, y: b}}`", "値はリスト `[a, b]` かレコード `{{x: a, y: b}}` にまとめる"),
        ("`)`", "`)`"),
        ("a named function cannot be used inside an expression", "名前付きの関数は式の中では使えない"),
        ("define it on its own line, or use an anonymous function: `fn(x) = x * 2`", "独立した行で定義するか、無名関数を使う：`fn(x) = x * 2`"),
        ("comments start with `#`", "コメントは `#` で始める"),
        ("`else` must follow `if condition then value`; a branch holds a single expression (use `do ... end` for several lines)", "`else` は `if condition then value` の後に続く。分岐は 1 つの式だけ（複数行なら `do ... end` を使う）"),
        ("a record is written `{{name: \"Ada\", age: 36}}`; for a block of code, use `do ... end`", "レコードは `{{name: \"Ada\", age: 36}}` と書く。コードのブロックには `do ... end` を使う"),
        ("`:` and a value", "`:` と値"),
        ("`then`", "`then`"),
        ("an `{word}` is written: `{word} condition then value else other_value`", "`{word}` は `{word} condition then value else other_value` と書く"),
        ("this `{word}` has no `else`", "この `{word}` に `else` がない"),
        ("every `{word}` must have an `else`", "`{word}` には必ず `else` が必要"),
        ("expected `else` here", "ここには `else` が必要"),
        ("write `else if`", "`else if` と書く"),
        (
            r#"in Rythme, `unless` always gives a value, so it needs both cases:
  unless age >= 18 then "minor" else "adult""#,
            r#"Rythme の `unless` は必ず値を返すので、両方のケースが必要：
  unless age >= 18 then "未成年" else "成人""#,
        ),
        (
            r#"in Rythme, `if` always gives a value, so it needs both cases:
  if age >= 18 then "adult" else "minor""#,
            r#"Rythme の `if` は必ず値を返すので、両方のケースが必要：
  if age >= 18 then "成人" else "未成年""#,
        ),
        ("a branch holds a single expression; for several lines, use `do ... end`", "分岐は 1 つの式だけ。複数行なら `do ... end` を使う"),
        ("a new line", "改行"),
        (
            r#"write each case on its own line, then `end`:
  match x
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"各ケースを 1 行ずつ書き、最後に `end`：
  match x
    0 -> "zero"
    _ -> "other"
  end"#,
        ),
        ("no `|` is needed before a case", "ケースの前に `|` は要らない"),
        ("write each case as `pattern -> result`", "各ケースは `pattern -> result` と書く"),
        ("`->`", "`->`"),
        ("each case is written `pattern -> result`", "各ケースは `pattern -> result` と書く"),
        ("put each case on its own line", "ケースは 1 行に 1 つずつ書く"),
        ("a `match` needs at least one case", "`match` には少なくとも 1 つのケースが必要"),
        ("a number after `..`", "`..` の後の数"),
        ("only a text can come before `+` in a pattern", "パターンの `+` の前に置けるのはテキストだけ"),
        ("to look at the end of a text, use a guard: `s if ends_with(s, \"!\") -> ...`", "テキストの終わりを調べるにはガードを使う：`s if ends_with(s, \"!\") -> ...`"),
        ("after `+`, a pattern takes a name or `_`", "パターンの `+` の後には名前か `_` を置く"),
        ("a pattern cannot contain `{{...}}`", "パターンに `{{...}}` は入れられない"),
        ("use a guard instead: `s if s == \"...\" -> ...`", "代わりにガードを使う：`s if s == \"...\" -> ...`"),
        ("`..rest` must be the last element: `[first, ..rest]`", "`..rest` は最後の要素にする：`[first, ..rest]`"),
        ("`:` and a pattern", "`:` とパターン"),
        ("a pattern", "パターン"),
        ("patterns look like: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`", "パターンの例：`x`、`_`、`0`、`\"text\"`、`:ok`、`Some(x)`、`[first, ..rest]`、`{{name}}`"),
        ("the ends of a range in a pattern are whole numbers of 64 bits at most", "パターン中の範囲の両端は 64 ビット以下の整数"),
        ("too large for a range", "範囲には大きすぎる"),
        ("compare with a guard instead: `n if n >= low and n <= high -> ...`; a range still matches large numbers between its ends, and a decimal end like `1.0e30` works too", "代わりにガードで比較する：`n if n >= low and n <= high -> ...`。範囲は両端の間の大きな数にもマッチし、`1.0e30` のような小数の端も使える"),
        ("a range in a pattern goes from a number to a number", "パターン中の範囲は数から数まで"),
        ("not a number", "数ではない"),
        ("the arguments of a call without parentheses are separated by commas: `print a, b`", "括弧なしの呼び出しの引数はカンマで区切る：`print a, b`"),
        ("write the numbers themselves, like `1..9` or `0.0..0.5`; to compare with names, use a guard: `n if n >= low and n <= high -> ...`", "`1..9` や `0.0..0.5` のように数そのものを書く。名前と比べるならガードを使う：`n if n >= low and n <= high -> ...`"),
        ("a text that starts with a text is written `\"Mr \" + name`: `name` is the rest of the text", "テキストで始まるテキストは `\"Mr \" + name` と書く：`name` はテキストの残り"),
        ("missing `end` to close this `{what}`", "この `{what}` を閉じる `end` がない"),
        ("this `{what}` is never closed", "この `{what}` が閉じられていない"),
        ("the code ends here", "コードはここで終わっている"),
        ("add `end` on its own line after the last line of the block", "ブロックの最後の行の後に、`end` だけの行を加える"),
        ("values never change in Rythme: give the new value a new name, like `total2 = total + 1`", "Rythme では値は変わらない：新しい値には新しい名前を付ける。例：`total2 = total + 1`"),
        // src/project.rs
        ("no folder to keep the packages in", "パッケージを保管するフォルダーがない"),
        ("set the variable RYTHME_HOME to a folder, like `RYTHME_HOME=~/.rythme`", "環境変数 RYTHME_HOME にフォルダーを設定する。例：`RYTHME_HOME=~/.rythme`"),
        ("the folder `{p}`", "フォルダー `{p}`"),
        ("run `rythme install` here, in the folder of the project", "ここ（プロジェクトのフォルダー）で `rythme install` を実行する"),
        ("run `rythme install` in `{root}`, the folder of the project", "プロジェクトのフォルダー `{root}` で `rythme install` を実行する"),
        ("the folder `{p}` of the package `{name}` does not exist", "パッケージ `{name}` のフォルダー `{p}` がない"),
        ("fix its `path` in `rythme.toml`", "`rythme.toml` の `path` を直す"),
        ("the package `{name}` is not installed", "パッケージ `{name}` がインストールされていない"),
        ("`rythme.toml` asks for it from {}", "`rythme.toml` は {} からそれを求めている"),
        ("`{shown_dir}` is not a Rythme package: it has no `rythme.toml`", "`{shown_dir}` は Rythme のパッケージではない：`rythme.toml` がない"),
        ("use one of its files: `use {package}/file`", "そのファイルの 1 つを使う：`use {package}/file`"),
        ("the package `{package}` has no file `{entry}`", "パッケージ `{package}` にファイル `{entry}` はない"),
        ("the package `{package}` has no file `{wanted}`", "パッケージ `{package}` にファイル `{wanted}` はない"),
        ("did you mean `use {prefix}{best}`?", "もしかして `use {prefix}{best}`？"),
        ("its files there are: {}", "そこにあるファイル：{}"),
        ("did you mean `use {best}/{rest}`?", "もしかして `use {best}/{rest}`？"),
        ("in this project, `use` also reaches the files of the packages {}", "このプロジェクトでは、`use` はパッケージ {} のファイルにも届く"),
        ("the program `git` was not found", "プログラム `git` が見つからない"),
        ("Rythme uses git to download the dependencies that come from git", "Rythme は git からの依存関係をダウンロードするのに git を使う"),
        ("install Git (https://git-scm.com), then run `rythme install` again; dependencies with `path` do not need it", "Git（https://git-scm.com）をインストールしてから、もう一度 `rythme install` を実行する。`path` の依存関係には不要"),
        ("git says: {said}", "git のメッセージ：{said}"),
        ("the tag `{t}` of `{name}` does not exist in `{url}`", "`{name}` のタグ `{t}` は `{url}` にない"),
        ("the commit `{c}` of `{name}` does not exist in `{url}`", "`{name}` のコミット `{c}` は `{url}` にない"),
        ("did you mean `tag = \"{best}\"`?", "もしかして `tag = \"{best}\"`？"),
        ("its tags are: {}", "タグは次のとおり：{}"),
        ("it has no tags: give a `commit` instead", "タグがない：代わりに `commit` を指定する"),
        ("cannot write in `{}`: {e}", "`{}` に書き込めない：{e}"),
        ("cannot download `{name}` from `{url}`", "`{url}` から `{name}` をダウンロードできない"),
        ("cannot take the commit `{commit}` of `{name}`", "`{name}` のコミット `{commit}` を取得できない"),
        ("no `rythme.toml` in this folder or above it", "このフォルダーにもその上にも `rythme.toml` がない"),
        ("go to the folder of a project, or create one with `rythme new NAME`", "プロジェクトのフォルダーに移動するか、`rythme new NAME` でプロジェクトを作る"),
        (
            r#"`rythme install` takes no argument (`{a}`): it installs the dependencies of `rythme.toml`
see `rythme help projects`"#,
            r#"`rythme install` は引数を取らない（`{a}`）：`rythme.toml` の依存関係をインストールする
`rythme help projects` を参照"#,
        ),
        ("`{name}` has no dependencies: add them under `[dependencies]` in `rythme.toml`", "`{name}` に依存関係はない：`rythme.toml` の `[dependencies]` の下に加える"),
        ("Installing the dependencies of `{name}`", "`{name}` の依存関係をインストール中"),
        ("the folder `{p}` of the package `{dep_name}` does not exist", "パッケージ `{dep_name}` のフォルダー `{p}` がない"),
        ("folder {p}", "フォルダー {p}"),
        ("downloaded", "ダウンロード済み"),
        ("already there", "インストール済み"),
        ("`{dep_name}` is not a Rythme package: `{shown_dir}` has no `rythme.toml`", "`{dep_name}` は Rythme のパッケージではない：`{shown_dir}` に `rythme.toml` がない"),
        ("a package is a project made by `rythme new`: its `rythme.toml`, and its files in `src/`", "パッケージとは `rythme new` で作ったプロジェクト（`rythme.toml` と `src/` のファイル）"),
        ("  ✓ {dep_name} {version}: {line}", "  ✓ {dep_name} {version}：{line}"),
        ("cannot write `{}`: {e}", "`{}` を書き込めない：{e}"),
        ("{count} package(s) ready", "{count} 個のパッケージの準備完了"),
        ("{count} package(s) ready ({more} needed by the other ones)", "{count} 個のパッケージの準備完了（うち {more} 個は他のパッケージが必要とするもの）"),
        ("which name? Example: rythme new hello", "名前は？ 例：rythme new hello"),
        ("`rythme new` takes one name, like `rythme new hello`", "`rythme new` は名前を 1 つ取る。例：`rythme new hello`"),
        ("try `rythme new {b}`", "`rythme new {b}` を試す"),
        ("`{shown_dir}` already exists", "`{shown_dir}` はすでにある"),
        ("choose another name, or another folder", "別の名前か別のフォルダーを選ぶ"),
        ("Created the project `{name}`:", "プロジェクト `{name}` を作成しました："),
        ("Next: `cd {shown_dir}`, then `rythme run` to run it and `rythme test` to test it (see `rythme help projects`)", "次は `cd {shown_dir}` してから、`rythme run` で実行、`rythme test` でテストします（`rythme help projects` を参照）"),
        ("greeting", "greeting"),
        ("hello", "hello"),
        (
            r#"# The project: `rythme run`, `rythme test` and `rythme check` in this folder.
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# From git, at a tag or a commit, then `rythme install`:
# mathlib = {{ git = "https://github.com/someone/mathlib", tag = "v1.0.0" }}
# From a folder:
# tools = {{ path = "../tools" }}
"#,
            r#"# プロジェクト：このフォルダーで `rythme run`、`rythme test`、`rythme check` を使います。
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# git から、タグかコミットを指定して。その後 `rythme install`：
# mathlib = {{ git = "https://github.com/someone/mathlib", tag = "v1.0.0" }}
# フォルダーから：
# tools = {{ path = "../tools" }}
"#,
        ),
        (
            r#"# The program: `rythme run` in the folder of the project.
use {module}

print({module}.{func}("World"))
"#,
            r#"# プログラム：プロジェクトのフォルダーで `rythme run` を使います。
use {module}

print({module}.{func}("世界"))
"#,
        ),
        (
            r#"## A greeting for `name`.
fn {func}(name) = "Hello, {{name}}!"
"#,
            r#"## `name` へのあいさつ。
fn {func}(name) = "こんにちは、{{name}}！"
"#,
        ),
        (
            r#"# The tests: `rythme test` in the folder of the project.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "Hello, Ada!")
"#,
            r#"# テスト：プロジェクトのフォルダーで `rythme test` を使います。
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "こんにちは、Ada！")
"#,
        ),
        (
            r#"# {name}

A [Rythme](https://github.com/maxlestage/Rythme) project.

```sh
rythme run        # run the program (src/main.ry)
rythme test       # run the tests (tests/)
rythme check      # find problems without running
rythme fmt        # give the code its official layout
rythme install    # download the dependencies of rythme.toml
```
"#,
            r#"# {name}

[Rythme](https://github.com/maxlestage/Rythme) のプロジェクトです。

```sh
rythme run        # プログラムを実行する（src/main.ry）
rythme test       # テストを実行する（tests/）
rythme check      # 実行せずに問題を見つける
rythme fmt        # コードを公式のレイアウトに整形する
rythme install    # rythme.toml の依存関係をダウンロードする
```
"#,
        ),
        (
            r#"# Files that git does not keep (keep rythme.lock).
.DS_Store
Thumbs.db
*.swp
*~
"#,
            r#"# git が保存しないファイル（rythme.lock は保存する）。
.DS_Store
Thumbs.db
*.swp
*~
"#,
        ),
        ("src/greeting.ry", "src/greeting.ry"),
        ("tests/greeting_test.ry", "tests/greeting_test.ry"),
        // src/repl.rs
        ("usage: :explain CODE, like :explain E003   (codes: {})", "使い方：:explain CODE（例：:explain E003）   （コード：{}）"),
        (":help for help, :quit to leave", "ヘルプは :help、終了は :quit"),
        ("(type :quit to leave)", "（終了するには :quit と入力）"),
        ("(cancelled)", "（キャンセルしました）"),
        ("cannot read what is typed: {e}", "入力を読めない：{e}"),
        ("usage: :doc NAME (a function, a topic...)", "使い方：:doc NAME（関数、トピックなど）"),
        ("usage: :load file.ry", "使い方：:load file.ry"),
        ("{} loaded", "{} を読み込みました"),
        ("Everything was forgotten.", "すべて忘れました。"),
        ("Messages are now in English.", "メッセージを日本語に切り替えました。"),
        ("usage: :lang fr  (or another of {codes})", "使い方：:lang ja  （または {codes} のどれか）"),
        ("usage: :steps EXPRESSION, like :steps fib(4)", "使い方：:steps EXPRESSION（例：:steps fib(4)）"),
        ("Nothing defined yet.", "まだ何も定義されていません。"),
        ("unknown command `{other}`", "不明なコマンド `{other}`"),
        (" — did you mean `{best}`?", " — もしかして `{best}`？"),
        (":help lists the commands", ":help でコマンドを一覧できる"),
        // src/steps.rs
        ("unknown option `{flag}` for `rythme steps`", "`rythme steps` に不明なオプション `{flag}`"),
        (
            r#"
the options are: --max N, --width N (see `rythme help steps`)"#,
            r#"
オプションは --max N、--width N（`rythme help steps` を参照）"#,
        ),
        ("  ... (more steps, not shown: {n} are shown at most, change it with --max)", "  ...（以降のステップは省略：表示は最大 {n} 個。--max で変更できる）"),
        ("this calculation became too deep to be shown step by step (more than {MAX_DEPTH} operations inside each other)", "この計算は深くなりすぎて、ステップごとに表示できない（{MAX_DEPTH} 個を超える演算が入れ子になっている）"),
        ("try it with smaller values, or run the program normally: `rythme file.ry`", "もっと小さい値で試すか、普通にプログラムを実行する：`rythme file.ry`"),
        ("the evaluation was stopped after {limit} steps: does it ever end?", "{limit} ステップで評価を止めた：いつか終わるのか？"),
        ("a function that calls itself must always reach a case where it stops", "自分自身を呼ぶ関数は、止まるケースに必ず到達しなければならない"),
        ("Nothing to show: this file only has definitions. Give an expression to compute, like: rythme steps file.ry \"fib(5)\"", "表示するものがありません：このファイルには定義しかありません。計算する式を指定してください。例：rythme steps file.ry \"fib(5)\""),
        ("`{flag}` needs a number, like `--max 50` or `--width 80` (20 at least)", "`{flag}` には数が必要。例：`--max 50` や `--width 80`（20 以上）"),
        (
            r#"usage: rythme steps file.ry [expression] [--max N] [--width N]
example: rythme steps fibonacci.ry "fib(4)""#,
            r#"使い方：rythme steps file.ry [expression] [--max N] [--width N]
例：rythme steps fibonacci.ry "fib(4)""#,
        ),
        // src/suggest.rs
        ("to join two lists or two texts, use `+`: `[1, 2] + [3]`; for a list of lists, use `flatten`", "2 つのリストやテキストをつなげるには `+` を使う：`[1, 2] + [3]`。リストのリストには `flatten` を使う"),
        ("in Rythme, this is called `{rythme}`", "Rythme ではこれを `{rythme}` と呼ぶ"),
        ("booleans are written in lowercase: `true`", "真偽値は小文字で書く：`true`"),
        ("booleans are written in lowercase: `false`", "真偽値は小文字で書く：`false`"),
        ("Rythme has no null: use `None` for \"no value\" and `Some(x)` for a value", "Rythme に null はない：「値がない」には `None`、値があるなら `Some(x)` を使う"),
        ("in Rythme, this is called `Some`", "Rythme ではこれを `Some` と呼ぶ"),
        ("in Rythme, an error value is written `Err(...)`", "Rythme ではエラーの値を `Err(...)` と書く"),
        ("in Rythme, a success value is written `Ok(...)`", "Rythme では成功の値を `Ok(...)` と書く"),
        ("Rythme has no objects: functions receive their data as arguments, like `area(shape)` or `shape.area()`", "Rythme にオブジェクトはない：関数はデータを引数として受け取る。例：`area(shape)` や `shape.area()`"),
        ("write a range with `..`: `1..10` is the list from 1 to 10 (included), and `0..10 |> step_by(2)` counts by 2", "範囲は `..` で書く：`1..10` は 1 から 10 まで（10 を含む）のリストで、`0..10 |> step_by(2)` は 2 ずつ数える"),
        ("use the operator `//` for a whole division: `7 // 2` is 3", "整数の割り算には演算子 `//` を使う：`7 // 2` は 3"),
        ("use the operator `%` for the remainder: `7 % 3` is 1", "余りには演算子 `%` を使う：`7 % 3` は 1"),
        ("to put values in a text, use braces: \"total: {{x}}\"; to write a number with 2 decimals: `text(x, 2)`", "テキストに値を入れるには波かっこを使う：\"total: {{x}}\"。数を小数点以下 2 桁で書くには `text(x, 2)`"),
        ("read the whole file with `read_file(path)`, then cut it with `lines(text)`; the lines of the standard input are `input_lines()`", "`read_file(path)` でファイル全体を読み、`lines(text)` で行に分ける。標準入力の行は `input_lines()`"),
        ("to merge two records, write `{{..a, ..b}}`: the fields of `b` replace those of `a`; for two dicts or two sets, `union(a, b)`", "2 つのレコードをまとめるには `{{..a, ..b}}` と書く（`b` のフィールドが `a` のものを置き換える）。2 つの辞書や集合なら `union(a, b)`"),
        ("write `unwrap_or(get(x, key), default)`: the value, or the default if it is missing", "`unwrap_or(get(x, key), default)` と書く：値か、なければデフォルト値"),
        ("compute the average with `sum(list) / len(list)`", "平均は `sum(list) / len(list)` で計算する"),
        ("`n.times do |i| ... end` is written `(0..n - 1).each do |i| ... end`", "`n.times do |i| ... end` は `(0..n - 1).each do |i| ... end` と書く"),
        ("to use another file, write `use other_file`, then `other_file.name`", ""),
        ("there is no `new`: build values directly, like `{{name: \"Ada\"}}` or `Circle(2.0)`", ""),
        ("values never change in Rythme: build a new value instead", ""),
        (
            r#"Rythme has no loops. Work on the whole list instead:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
or write a recursive function. See `rythme help functional`."#,
            "",
        ),
        ("Rythme has no loops to interrupt: use `find`, `take`, `filter`, or a recursive function", ""),
        (
            r#"no `return` needed: the last expression of a function is its result.
To stop early with an error, return `Err(...)` and use `?`."#,
            "",
        ),
        ("just write `name = value` (values never change, so no keyword is needed)", ""),
        ("functions are written `fn name(x) = x * 2`, and anonymous ones `fn(x) = x * 2`", ""),
        (
            r#"Rythme has no classes. Describe your data with `type`:
  type Shape = Circle(radius) | Square(side)
or use a record: {{name: "Ada", age: 36}}"#,
            "",
        ),
        (
            r#"use `match`:
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
            "",
        ),
        (
            r#"errors are values in Rythme: return `Ok(value)` or `Err(message)`,
then handle them with `match` or pass them on with `?`. `fail(message)` stops the program."#,
            "",
        ),
        // src/testing.rs
        (".", ""),
        ("the test returned `false`", ""),
        ("use `assert_eq(actual, expected)` to see the values", ""),
        ("the test ended with {}", ""),
        ("a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`", ""),
        (
            r#"No tests found. Write one like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            "",
        ),
        ("Running {n} test(s)", ""),
        ("Running {n} of {total} test(s)", ""),
        ("Running {n} test(s) of {file}", ""),
        ("Running {n} of {total} test(s) of {file}", ""),
        ("{f} file(s), ", ""),
        ("{total} test(s): {passed} passed, {failed} failed", ""),
        (", {left_out} left out by the filter", ""),
        (", {broken} file(s) with errors", ""),
        ("No test name contains {}.", ""),
        (" or ", ""),
        ("{red}✗{reset} {}: the file has errors, so its tests could not run", ""),
        (
            r#"No file with tests in {}. Write a test in a `.ry` file like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            "",
        ),
        ("Failed tests:", ""),
        ("`{name}` needs {what}", ""),
        ("a part of the name of a test", ""),
        ("a number, or `random`", ""),
        ("the seed must be a whole number of 0 or more, or `random`, not `{v}`", ""),
        ("unknown option `{name}` for `rythme test`", ""),
        (
            r#"
the options are: --filter NAME, --seed N, --watch, --sandbox, --format json (see `rythme help tests`)"#,
            "",
        ),
        ("which file or folder? Example: rythme test my_tests.ry, or rythme test . for the files of this folder", ""),
        ("Seed: {}", ""),
        (
            r#"
Watching {} file(s): the tests run again when one changes (Ctrl+C to stop)."#,
            "",
        ),
        (
            r#"
{} changed: running the tests again
"#,
            "",
        ),
        // src/value.rs
        ("constructor", ""),
        // playground/src/lib.rs
        ("✓ no problems found", ""),
        ("✓ no errors, {n} warning(s)", ""),
        // help pages, explanations, keywords, functions, months
        (
            r#"RYTHME IN ONE MINUTE

Rythme is a simple language where values never change and everything is a
function. Programs are files ending with .ry.

  # A comment
  name = "World"                 # a name for a value (it never changes)
  print("Hello, {name}!")        # {...} inserts a value in a text

  fn square(x) = x * x           # a function
  print(square(4))               # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # keep the even numbers
    |> map(square)               # square each one
    |> sum                       # add them up
    |> print                     # 20

Run it:     rythme hello.ry
Check it:   rythme check hello.ry
Try code:   rythme          (interactive mode)

Rythme helps you all the time: every error says where it is, why, and how
to fix it. `rythme explain E003` explains an error code in detail."#,
            "",
        ),
        (
            r#"SYNTAX

Values
  42   -7   1_000_000            whole numbers (Int), of any size
  3.14   2.0                     decimal numbers (Float)
  "hello"   "total: {x}"         texts (Text); {...} (or #{...}) inserts a value
  true   false                   booleans (Bool)
  :ok   :error                   symbols (Symbol): a name used as a value
  [1, 2, 3]                      lists
  {name: "Ada", age: 36}         records
  Some(3)   None                 a value that may be missing (Option)
  Ok(3)   Err("oops")            a result that may fail (Result)
  1..5                           the list [1, 2, 3, 4, 5]

Names
  x = 5                          a name for a value; it never changes
  [a, b] = [1, 2]                take a value apart
  {name, age} = person

Operators
  + - * /   // (whole division)   % (remainder)   ** (power)
  == != < <= > >=   and   or   not
  x |> f(y)                      same as f(x, y): steps from left to right
  x.f(y)                         also the same as f(x, y)
  x.f                            the field f of x; if x has none, f(x): xs.sum
  xs                             a chain may go on, each line starting with .
    .select(even?)
    .sum

Functions
  fn double(x) = x * 2           on one line, with =
  fn greet(name)                 on several lines, finished by `end`
    message = "Hello {name}"
    print(message)               the last line gives the result
  end
  fn(x) = x + 1                  an anonymous function
  fn adult?(p) = p.age >= 18     a name may end with ? (see rythme help ruby)

Blocks: a function given as the last argument of a call
  xs.map do |x| x * 2 end        same as map(xs, fn(x) = x * 2)
  xs.each do |x|                 on several lines, finished by `end`
    print(x)
  end
  xs.select { |x| x > 2 }        with braces

Conditions (every if has an else)
  if x > 0 then "positive" else if x == 0 then "zero" else "negative"
  unless x > 0 then "small" else "big"     the opposite of if

Choosing by shape
  match list
    [] -> "empty"
    [x] -> "one element: {x}"
    [x, ..rest] -> "{x}, then {len(rest)} more"
  end

Several steps inside an expression
  do
    a = 1
    a + 1
  end

Your own types
  type Shape = Circle(radius) | Rect(width, height)

Types written in the code (optional, checked by rythme check)
  fn area(w: Int, h: Int) -> Int = w * h
  total: Float = 0.0
  type Box(a) = Box(value: a)    a type that takes a type: Box(Int), Box(Text)

Tests and other files
  test "double works" = assert_eq(double(2), 4)
  use geometry                   then geometry.area(2)

Comments
  # an ordinary comment
  ## a documentation comment, above a function or a type, shown by help(...)"#,
            "",
        ),
        (
            r#"FUNCTIONS

Define and call
  fn add(a, b) = a + b
  add(2, 3)                      # 5

On several lines: the last expression is the result
  fn describe(n)
    kind = if n % 2 == 0 then "even" else "odd"
    "{n} is {kind}"
  end

One definition per case (tried in order)
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

With a condition (a guard)
  fn sign(n) if n < 0 = "negative"
  fn sign(0) = "zero"
  fn sign(_) = "positive"

Functions are values
  double = fn(x) = x * 2
  [1, 2, 3] |> map(double)       # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

Pipes read from left to right
  "  Hello  " |> trim |> upper   # "HELLO"
  x |> f(a, b)    means    f(x, a, b)

Method style
  "hello".upper()                # same as upper("hello")

Documentation
  ## Adds two numbers.
  fn add(a, b) = a + b
  help(add)                      # shows "Adds two numbers."

Recursion without limit
  When the recursive call is the very last thing a function does, it can
  repeat millions of times:
  fn count_down(0) = print("Liftoff!")
  fn count_down(n)
    print(n)
    count_down(n - 1)
  end"#,
            "",
        ),
        (
            r#"LISTS

  numbers = [3, 1, 2]
  numbers[0]                     # 3 (the first; -1 is the last)
  len(numbers)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numbers]                 # [0, 3, 1, 2]   (.. inserts a whole list)
  numbers + [9]                  # [3, 1, 2, 9]

Work on the whole list (no loops needed)
  map(numbers, fn(x) = x * 10)            # [30, 10, 20]
  filter(numbers, fn(x) = x > 1)          # [3, 2]
  fold(numbers, 0, fn(acc, x) = acc + x)  # 6
  sort(numbers)                           # [1, 2, 3]
  numbers |> each(fn(x) = print(x))       # prints each element

Safe access (never an error)
  first([])                      # None
  get(numbers, 10)               # None

"Change" a list: you get a new list, the old one stays the same
  set_at(numbers, 0, 9)          # [9, 1, 2]
  insert_at(numbers, 1, 7)       # [3, 7, 1, 2]
  remove_at(numbers, -1)         # [3, 1]

Pieces of a list
  take_while(numbers, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

Take a list apart with patterns
  match numbers
    [] -> "empty"
    [first, ..rest] -> "{first}, then {len(rest)} more"
  end

All the functions: rythme help builtins"#,
            "",
        ),
        (
            r#"TEXT

  name = "Ada"
  "Hello, {name}!"               # "Hello, Ada!"  ({...} inserts any expression)
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3 (characters, accents included)
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hello", "ell")       # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14" (a number with 2 decimals)
  pad_left("7", 3, "0")          # "007"  (and pad_right: to line up columns)
  char_code("A")   char(66)      # 65 and "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...): a conversion that cannot crash
  trim_start("  x")   trim_end("x  ")
  words("Hi, you!")              # ["Hi", "you"]
  equals_ignore_case("Ada", "ADA")          # true
  format_number(1234.5, 2)       # "1,234.50"   (with "fr": "1 234,50")
  find_all("3 cats, 12 dogs", "[0-9]+")     # ["3", "12"]  (regexes: rythme help regex)

Special characters: \n (new line), \t (tab), \r (carriage return),
\0 (null character), \" (quote), \\ (backslash), \{ and \} (braces, because
{ starts an insertion), \u{e9} (any character by its Unicode number: é)."#,
            "",
        ),
        (
            r#"RECORDS

A record groups named values:
  ada = {name: "Ada", born: 1815}
  ada.name                       # "Ada"
  ada["name"]                    # "Ada"

Records never change: build a new one from an old one
  older = {..ada, born: 1816}    # same fields, `born` replaced

Take apart
  {name, born} = ada
  fn greet({name}) = "Hello {name}"

Useful functions
  keys(ada)   values(ada)   has(ada, "name")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["name", "Ada"], ["born", 1815]]
  from_entries([["x", 1]])       # {x: 1}

Records as dictionaries
  counts = {apple: 2}
  put(counts, "pear", 5)         # {apple: 2, pear: 5}
  map_values(counts, fn(n) = n * 10)      # {apple: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}
  For keys that are not names (numbers, lists...), use a dict: rythme help dicts

JSON, the format of data on the web
  parse_json(text)               # Ok(...): objects become records, arrays lists
  to_json(value)                 # the text, ready to save with write_file"#,
            "",
        ),
        (
            r#"DICTIONARIES AND SETS

A dict (dictionary) gives a value for each key. Keys can be any value but
functions: numbers, texts, lists, records... They are kept in order.
  ages = dict([["Ada", 36], ["Alan", 41]])
  ages                           # dict("Ada" => 36, "Alan" => 41)
  ages["Ada"]                    # 36 (an error if the key is missing)
  get(ages, "Bob")               # None: an Option, never an error
  grid = dict([[[0, 0], "start"]])   # a list as a key
  dict({x: 1})                   # from a record: dict("x" => 1)

Dicts never change: functions give a new one, quickly (in O(log n))
  put(ages, "Bob", 20)           # added (or replaced)
  remove(ages, "Ada")            # without this key
  has(ages, "Ada")   len(ages)   keys(ages)   values(ages)   entries(ages)
  map_values(ages, fn(n) = n + 1)
  filter(ages, fn(name, age) = age > 40)   # f receives the key and the value
  ages.each do |name, age| print(name, age) end   # reject and select too
  union(a, b)                    # both; for a key in both, b's value wins

A set holds values without repetition, in order.
  s = set([3, 1, 3])             # set(1, 3)
  has(s, 3)   push(s, 5)   remove(s, 1)   len(s)   values(s)   # values: a list
  union(a, b)   intersection(a, b)   difference(a, b)   is_subset(a, b)
  filter(s, is_odd)   map(s, fn(x) = x * 10)

Dicts and sets compare with == and <, and print as dict(k => v, ...) and
set(a, b, ...). `to_json` writes a dict whose keys are all texts (or all symbols) as an
object, a set as an array; `dict(parse_json(text)?)` reads an object back."#,
            "",
        ),
        (
            r#"YOUR OWN TYPES

A type lists the possible shapes of a value:
  type Shape = Circle(radius) | Rect(width, height) | Empty

  c = Circle(2.0)
  c.radius                       # 2.0

Handle each shape with match
  fn area(shape) = match shape
    Circle(r) -> pi * r * r
    Rect(w, h) -> w * h
    Empty -> 0
  end

Or with one definition per shape
  fn perimeter(Circle(r)) = 2 * pi * r
  fn perimeter(Rect(w, h)) = 2 * (w + h)
  fn perimeter(Empty) = 0

If you forget a shape, `rythme check` tells you which one.

A type can contain itself (trees...):
  type Tree = Leaf | Node(left, value, right)"#,
            "",
        ),
        (
            r#"MATCH AND PATTERNS

`match` compares a value with patterns, in order, and gives the result of the
first one that fits:

  fn describe(x) = match x
    0 -> "zero"
    1 | 2 | 3 -> "small"
    n if n < 0 -> "negative"
    _ -> "big"
  end

Patterns
  42   "text"   true             exactly this value
  1..9   -5..-1   0.0..0.5       a number from the first to the second, both included
  "Mr " + name                   a text that starts with "Mr "; name is the rest
  x                              anything, named x
  _                              anything, ignored
  Some(x)   Circle(r)            a constructor and what it contains
  []   [a, b]   [first, ..rest]  lists, by shape
  {name, age}                    a record with these fields
  p if condition                 a pattern and a condition (guard)
  a | b                          one pattern or the other

Patterns work everywhere a name is given:
  [x, y] = [1, 2]
  fn head_or_zero([]) = 0
  fn head_or_zero([x, ..]) = x"#,
            "",
        ),
        (
            r#"ERRORS ARE VALUES (Option, Result, ?)

Rythme has no null and no exceptions. A value that may be missing is an
Option: Some(value) or None. An operation that may fail gives a Result:
Ok(value) or Err(message). You cannot forget to handle them.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

  match divide(10, 2)
    Ok(x) -> print("result: {x}")
    Err(e) -> print("error: {e}")
  end

The ? operator: take the value, or stop and pass the error on
  fn average(a, b)
    x = parse_float(a)?          # if this is Err(...), average returns it
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

Quick tools
  unwrap_or(get(list, 5), 0)     # the value, or a default
  unwrap(Some(3))                # 3 (stops the program on None or Err)
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(person, "age"), "no age")   # Err("no age") if missing: ready for ?
  and_then(x, f)                 # one more step that may fail
  map_err(r, fn(e) = "config: {e}")     # explain an error better

To stop the whole program: fail("message")."#,
            "",
        ),
        (
            r#"THINKING FUNCTIONALLY

In Rythme, values never change and there are no loops. It may seem strange,
but programs become easier to read, test and trust: a function always gives
the same result for the same arguments.

Instead of a loop that prints:       1..5 |> each(fn(i) = print(i))
Instead of a loop that transforms:   map(list, f)
Instead of a loop that selects:      filter(list, f)
Instead of a loop that accumulates:  fold(list, start, f)   or sum, product
Instead of running totals:           scan(list, start, f)
Instead of a loop that searches:     find, any, all, index_of, min_by, max_by
Instead of "break":                  take_while(list, f)
Instead of "while":                  a recursive function

  # Python:  total = 0
  #          for x in numbers: total += x * x
  total = numbers |> map(fn(x) = x * x) |> sum

  # A "while" loop as a recursive function:
  fn collatz(1, steps) = steps
  fn collatz(n, steps) =
    if n % 2 == 0 then collatz(n // 2, steps + 1) else collatz(3 * n + 1, steps + 1)
  collatz(27, 0)                 # 111

Instead of changing a value, make a new one:
  x2 = x + 1
  person2 = {..person, age: 37}
  list2 = push(list, 4)"#,
            "",
        ),
        (
            r#"TESTS

Write tests next to your code:
  fn double(x) = x * 2

  test "double of 2 is 4" = assert_eq(double(2), 4)

  test "several checks"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "should stay negative")
  end

Run them:
  rythme test my_file.ry            the tests of one file
  rythme test .                     of every .ry file of a folder (and below)
  rythme test . --filter double     only the tests whose name contains "double"
  rythme test . --watch             again each time a file changes (Ctrl+C stops)
  rythme test my_file.ry -- a b     with `args()` giving ["a", "b"]
  rythme test                       in a project: its files with tests

While testing, top-level expressions such as print(...) are not run: the
definitions are loaded, then each test runs. A test passes if it ends without
error, and its value is not false, Err(...) or None. The summary gives the
counts and the time; the exit code is 0 when every test passes.

Property tests: check that something holds for many values
  test "reversing twice gives the list back" =
    check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

Generators: ints(min, max), floats(min, max), texts(max_length), bools(),
lists_of(generator, max_length), one_of(list), records_of({x: ints(), y: texts()}).
`sample(generator)` shows what one gives. When a value makes the property
false, it is simplified (shrinking) and shown with the seed that made it: the
same values come back on every run, and `--seed N` tries others
(`--seed random` picks one)."#,
            "",
        ),
        (
            r#"EXERCISES

Rythme comes with 36 graded exercises for a first course, from the first
values to recursion, lists, records, types, Option, Result and tests, then six
about artificial intelligence (31 to 36: vectors, statistics, a neuron,
gradient descent, nearest neighbours, minimax). Each one is a file with a
statement, hints, the functions to write and their tests.

  rythme exercise list              the exercises, from ★☆☆ (easy) to ★★★
  rythme exercise start 7           copies exercise 7 into the current folder
  rythme exercise check 07-recursion.ry
                                    runs its tests, and says what to do next

In the file, replace each fail("to do") with your code; do not change the
tests. `rythme test file.ry` runs the tests too. The exercises follow the
language of the messages (`rythme --lang es exercise start 7`); the folder
docs/exercises of the repository of Rythme has them in English and French."#,
            "",
        ),
        (
            r#"SEVERAL FILES

  # geometry.ry
  ## Area of a rectangle.
  fn area(w, h) = w * h
  fn _check(x) = x >= 0          # private: starts with `_`

  # main.ry
  use geometry                    # the file geometry.ry, next to main.ry
  print(geometry.area(2, 3))      # 6

`use geometry` gives the definitions of geometry.ry through its name:
`geometry.area`, `geometry.Circle(1)`, and in a `match`, `geometry.Circle(r)`.
So you always see where a name comes from. A file in a folder:
`use shapes/circle`, then `circle.perimeter(1)`. Another name: `use
geometry as geo`, then `geo.area(2, 3)`. Names starting with `_` stay
private to their file.

The path is relative to the folder of the file that contains `use`. Each file
runs only once. Rythme says which file defines a name you forgot to use.

Older form: `use "geometry.ry"` (with quotes) gives its definitions directly,
as `area(2, 3)`; prefer `use geometry`."#,
            "",
        ),
        (
            r#"FILES AND THE SYSTEM

Every function that touches the system returns a Result: Ok(...), or
Err(message) with a message that says what went wrong.

Files and folders (relative paths start in the current folder)
  read_file("notes.txt")         # Ok("...") or Err("file not found: notes.txt")
  write_file("out.txt", "hi")    # Ok(()), replaces the content
  append_file("log.txt", "done\n")
  file_exists("notes.txt")       # true or false
  file_info("notes.txt")         # Ok({is_folder: false, size: 11}), in bytes
  list_folder(".")               # Ok(["notes.txt", "photos"]), in order
  create_folder("backup/2024")   # Ok(()), with the missing folders around it
  delete_file("old.txt")         # Ok(()); never a folder

Paths that work on Linux, macOS and Windows
  join_path(["data", "notes.txt"])       # "data/notes.txt"
  split_path("/home/ada/notes.txt")      # ["/", "home", "ada", "notes.txt"]
  split_path("C:\Users\ada")             # ["C:\", "Users", "ada"]

The command line: rythme tool.ry --verbose --name=Ada notes.txt
  args()                         # ["--verbose", "--name=Ada", "notes.txt"]
  options()                      # Ok({args: ["notes.txt"], verbose: true, name: "Ada"})
  opts = unwrap(options())
  name = unwrap_or(get(opts, "name"), "World")   # an option that may be missing
  env("HOME")                    # Some("/home/ada") or None

The standard input: rythme count.ry < names.txt
  input("Name? ")                # one line
  input_lines()                  # Ok(["Ada", "Alan"]): every line until the end

Exit codes, for the scripts that run your program
  0    everything went well (also at the end of the program)
  1    an error in the program: syntax, check, while running, failed test
  2    a wrong use of the command (unknown command or option)
  101  an internal error of Rythme (a bug, to report)
  exit(code)                     # stops now with this code, from 0 to 255

In the playground, in the browser, there are no files and no input: these
functions give Err("... is not available in the browser ...")."#,
            "",
        ),
        (
            r#"PROJECTS AND PACKAGES

  rythme new hello        # a folder hello/ with a new project:
                          #   rythme.toml, src/main.ry, src/greeting.ry,
                          #   tests/greeting_test.ry, README.md, .gitignore
  cd hello
  rythme run              # runs the program: `entry` of rythme.toml
  rythme test             # runs the files with tests of src/ and tests/
  rythme check            # checks every file; also `rythme fmt`, `rythme doc`

rythme.toml describes the project:

  name = "hello"
  version = "0.1.0"
  entry = "src/main.ry"

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

A dependency comes from git, at a `tag` or a `commit`, or from a folder.
`rythme install` downloads them (with git) and writes rythme.lock, the exact
commit of each one: keep it with the project, so that everyone gets the
same code. Then:

  use mathlib/vectors       # the file src/vectors.ry of mathlib
  use mathlib               # its program, the `entry` of its rythme.toml
  use hello/greeting        # a file of the project itself, from tests/

A file next to the one that contains `use` comes first. Packages from git
are kept in ~/.rythme/packages (or RYTHME_HOME/packages). Finding packages
by name and sharing yours: `rythme help packages`."#,
            "",
        ),
        (
            r#"DATES AND TIMES

A date is a record: {year, month, day}, with hour, minute and second for a
moment of the day, and offset (minutes ahead of UTC) when it is known.
  today()                        # {year: 2026, month: 10, day: 18}
  now()                          # {year: 2026, ..., hour: 14, minute: 5, second: 9, offset: 120}
  birthday = {year: 1815, month: 12, day: 10}
  birthday.year                  # 1815

Text and back, in the ISO 8601 form
  parse_date("2024-02-29")       # Ok({year: 2024, month: 2, day: 29})
  parse_date("2024-02-30")       # Err("... February 2024 ...")
  parse_date("2024-02-29T13:05:00+01:00")    # with a time and an offset
  format_date({year: 2024, month: 2, day: 29, hour: 9})   # "2024-02-29T09:00:00"

Calendar arithmetic (leap years and month lengths included)
  add_days({year: 2024, month: 2, day: 28}, 1)     # {year: 2024, month: 2, day: 29}
  add_seconds(now(), 3600)       # in one hour
  days_between(today(), {year: 2027, month: 1, day: 1})   # days left
  seconds_between(a, b)          # negative when b is before a
  day_of_week(today())           # 1 = Monday ... 7 = Sunday
  days_in_month(2023, 2)         # 28
  is_leap_year(2000)             # true
  is_date({year: 2023, month: 2, day: 29})   # false

A date that does not exist stops the functions with E023, which says why.
The local offset comes from the system (UTC when it does not tell); time()
gives seconds since 1970 as a Float, to measure durations."#,
            "",
        ),
        (
            r#"REGEX: SEARCHING WITH A PATTERN

A regex (regular expression) describes pieces of text: "[0-9]+" means "one
digit or more". Rythme reads the usual regexes, and a search never takes
more than (length of the text) × (size of the regex).

  matches("2024", "[0-9]+")                  # true: the WHOLE text matches
  find_first("tel: 06 12", "[0-9]+")         # Some("06"): the first piece
  find_all("3 cats, 12 dogs", "[0-9]+")      # ["3", "12"]
  split_by("a, b;c", "[,;] *")               # ["a", "b", "c"]
  replace_all("a cat", "\\b\\w", upper)      # "A Cat" (a function for each piece)
  replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1")   # "2024-02-29"
  captures("2024-02", "([0-9]+)-([0-9]+)")   # Some(["2024-02", "2024", "02"])
  named_captures("2024-02", "(?<year>[0-9]+)-(?<month>[0-9]+)")
                                             # Some({year: "2024", month: "02"})

What a regex can say
  abc          these characters, in this order
  .            any character but a line break
  [abc]        one of these characters; [a-z] one from a to z; [^0-9] any other
  \d \w \s     a digit (0-9); a letter (accents included), a digit or _; a space
  \D \W \S     anything else
  \b           the edge of a word
  ^  $         the start and the end of the text
  x|y          x or y
  (...)        a group: to repeat it, or to capture its text ($1, `captures`)
  (?:...)      a group that captures nothing; (?<name>...) a group with a name
  x*  x+  x?   x zero times or more, once or more, zero or one time (the longest)
  x{3}  x{2,5}  x{2,}    3 times, 2 to 5 times, 2 times or more
  x*?  x+?     the shortest instead of the longest
  (?i)         at the start: uppercase and lowercase letters count as equal

Writing a regex in a Rythme text
  A backslash is written twice: "\\d+" is the regex \d+. Braces are written
  \{ and \}: "[0-9]\{4\}" is the regex [0-9]{4}. For a sign itself, put it in
  brackets: "[.]" is a dot, "[(]" a parenthesis, "[$]" a dollar.

A mistake in a regex stops the program with E026, which shows where it is;
when the regex is written in the code, `rythme check` finds it before."#,
            "",
        ),
        (
            r#"THE INTERACTIVE MODE

Run `rythme` without a file. Type an expression and see its value:
  ry> 1 + 2
  3
Definitions are kept from one line to the next. If a line is not finished
(open bracket, block without `end`...), it continues on the next line.

Commands
  :help           this help              :quit          leave (or Ctrl-D)
  :doc map        help on a name         :load f.ry     load a file
  :reset          forget everything      :lang fr       change the language (en, fr, es, de, pt, zh, it, ja)
  :learn          guided tour            :explain E003  explain an error
  :steps fib(4)   show the calculation step by step
The Tab key completes names."#,
            "",
        ),
        (
            r#"FORMATTING: rythme fmt

`rythme fmt` gives code the official layout of Rythme: every program looks
the same, and nobody has to argue about spaces.

  rythme fmt hello.ry            format a file (it is rewritten)
  rythme fmt .                   every .ry file of a folder and its subfolders
  rythme fmt --check .           only check: lists the files to format and
                                 fails (exit code 1) if there are any, for CI
  rythme fmt - < hello.ry        reads the code, writes the result

The style
  - 2 spaces per level of indentation
  - spaces around operators: x = a + b * 2   xs |> map(f)   n -> n * 2
  - none inside brackets or around ..: f(x)   [1, 2]   {name: "Ada"}   1..10
  - a pipeline written on several lines: one step per line
      total = prices
        |> filter(fn(p) = p > 10)
        |> sum
  - a type written on several lines: one variant per line
      type Shape =
        | Circle(radius)
        | Rect(width, height)
  - a list written on several lines ends with a comma
  - a block written on one line stays on one line:
      xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
    on several lines, its statements go one level deeper than `do`
  - one statement per line (no `;`), and `end` on its own line
  - a blank line around a definition written on several lines; never two
    blank lines in a row
  - every comment is kept; the comments at the end of consecutive lines
    are aligned

Line breaks are kept where the language allows a choice, and long lines are
kept as written. The formatter never changes what a program does: a file with
a syntax error is left as it is, and the error is shown."#,
            "",
        ),
        (
            r#"DOCUMENTATION: rythme doc

`rythme doc` writes the documentation of your code from its `##` comments,
its types and the definitions of its functions.

  rythme doc geometry.ry         Markdown on the screen
  rythme doc .                   every .ry file of a folder and its subfolders
  rythme doc --html geometry.ry  one web page, which needs nothing else
  rythme doc . --out doc.html    written to a file (.html gives the web page)

Write a `##` comment just above what it documents:

  ## Shapes and their areas.      <- at the top, followed by a blank line:
                                     it describes the whole file
  ## A shape to draw.
  type Shape = Circle(radius) | Rect(width, height)

  ## The area of a shape. See `len` and `area`.
  fn area(Circle(r)) = pi * r * r
  fn area(Rect(w, h)) = w * h

What is written
  - for each file: its description, the files it uses (documented too),
    its types, its functions (the first line of every definition), its
    values and the names of its tests
  - names starting with `_` are left out: they stay inside their file
  - a name between backticks in a comment, like `area` or `len(list)`,
    becomes a link to its documentation; after `use geometry`,
    `geometry.area` leads to `area` in geometry.ry; the built-in functions
    named this way are explained at the end
  - the titles are in the language of the messages (`--lang fr`)

In a comment, a blank `##` line starts a new paragraph, and lines starting
with `- ` make a list."#,
            "",
        ),
        (
            r#"STEP BY STEP: rythme steps

`rythme steps` shows how a program computes its values, one step at a time:
each call is replaced by the body of its function, with the values of the
arguments in place, until only the result remains.

  rythme steps fib.ry              each definition and expression of the file
  rythme steps fib.ry "fib(4)"     an expression, after the definitions of the file
  rythme steps fib.ry --max 50     show at most 50 steps of each expression
  rythme steps fib.ry --width 80   cut the lines longer than 80 characters

With  fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

  fib(3)
    → fib(2) + fib(1)
    → fib(1) + fib(0) + fib(1)
    → 1 + fib(0) + fib(1)
    → 1 + 0 + fib(1)
    → 1 + fib(1)
    → 1 + 1
    → 2

A step is one call or one operation. The body of a called function is shown
already simplified as far as it can be without calling another function.
Built-in functions (and the functions of other files) give their result in
one step. What the program prints appears between the steps. After the
steps shown (200 by default), the rest is computed without being shown.

In the interactive mode: :steps fib(4). In the playground: the Steps button.
In French: rythme etapes."#,
            "",
        ),
        (
            r#"RYTHME IN YOUR EDITOR

Rythme helps inside the editor too: problems as you type, the documentation
of a function under the mouse (with the signature inferred from its code,
like `fn area(side: Int|Float) -> Int|Float`), completion, the parameters of
a call, go to definition, references, rename, quick fixes for typos,
formatting like `rythme fmt`, the types of the names when they are certain
(and the signatures of the functions), colors that follow what each name is,
"Run" and "Run test" above the code, the names of the whole workspace,
folding. This is the language server, that the editor
starts by itself:
  rythme lsp

VS Code
  the extension in editors/vscode: colors, snippets and the language server.
  Settings: rythme.path (the program), rythme.language (en, fr, es, de, pt, zh, it, ja or auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  every editor that speaks the Language Server Protocol: tell it to start
  `rythme lsp` for .ry files. Ready-made configurations: editors/README.md
  ("Run" and "Run test" name the commands rythme.runFile, rythme.testFile and
  rythme.runTest, that the editor carries out: see editors/README.md)

Zed
  the extension in editors/zed: colors and the language server.

Colors in Neovim, Helix and Zed
  the Tree-sitter grammar in editors/tree-sitter-rythme (see editors/README.md).

The language of the messages follows the editor, unless `--lang` (en, fr, es, de, pt, zh, it, ja) or
RYTHME_LANG chooses it: rythme --lang fr lsp"#,
            "",
        ),
        (
            r#"COMING FROM RUBY

Rythme reads like Ruby, but its values never change: no mutation, no nil,
no exceptions. What you know from Ruby mostly works:

  names = ["Ada", "Grace", "Alan"]
  names.each do |name|
    puts "Hello, #{name}!"
  end

  [1, 2, 3, 4, 5, 6]
    .select(even?)
    .map { |x| x * x }
    .sum                         # 56

  fn adult?(person) = person.age >= 18
  kind = unless adult?(ada) then :minor else :adult

The same as in Ruby
  calls           puts "hi"   print x, y   assert_eq total, 42
                  (at the start of a statement or a result; see below)
  blocks          xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
                  a block is a function: the same as fn(x) = x * 2
  method calls    xs.sum   name.upcase   xs.first(2)   xs.include?(3)
  names with ?    even?   empty?   include?(x)   fn adult?(p) = ...
  symbols         :ok   :error (compared with ==, used in match)
  texts           "Hello #{name}" (or "Hello {name}"); "\#{" writes #{
  unless          unless cond then a else b
  Ruby names      puts to_s to_i to_f length size upcase downcase strip
                  select reject reduce inject uniq each_slice each_cons
                  each_with_index include? empty? even? odd? any? all?
                  start_with? end_with? key? lstrip rstrip casecmp?
                  (see rythme help builtins)

What is different
  parentheses     a call inside an expression needs them: x = len(xs),
                  not x = len xs
  no mutation     no x += 1: give the new value a new name, total2 = total + 1
  no nil          a missing value is None, a present one Some(x)
  no exceptions   errors are values, Ok(x) or Err(message), passed on with ?
  if and unless   always have an else: they give a value; no `return`, the
                  last line of a function is its result
  no classes      records {name: "Ada"} and types: type Shape = Circle(r) | Square(s)
  no loops        each, map, select, reduce, or a function that calls itself
  case/when       match value ... end, one case per line: pattern -> result
  hashes          dict([[:ada, 36]]) (keys of any kind, symbols too), or
                  records {name: "Ada", age: 36}

Calls without parentheses
  A statement that starts with a name, followed on its line by the start of
  a value (a number, a text, a symbol, a name, a constructor, not, if,
  match, fn), is a call: print x, y is print(x, y). A do block after the
  arguments is given to the call: each xs do |x| ... end. Like Ruby, a [ or
  a - after a space, without a space after the -, starts the arguments too
  (puts [1, 2], puts -1), except when the code gives the name a value (with
  =, as a parameter or in a pattern): then x -1 is x - 1 and xs [0] is
  xs[0], as before. `rythme fmt` writes those as x - 1 and xs[0].

  A result may start the same way: a branch of if or unless, a case of
  match, the body after the = of a function or a test. Its arguments end
  at else, end, }, ) or the end of the line:

  fn greet(name) = puts "Hello, #{name}!"
  fn check(ok) = if ok then puts "yes" else puts "no"
  match name
    "Dr " + rest -> puts "doctor", rest
    _ -> puts name
  end
  xs.each { |x| puts x }

How x.name works
  x.name reads the field `name` of a record; when x has no such field, it
  calls the function `name` with x: xs.sum is sum(xs), xs.first(2) is
  first(xs, 2). A chain may go on over several lines, each starting with `.`.

When ? is part of a name
  even?(4), xs.include?(3), fn adult?(p): right after a name, before `(`.
  even? without `(`: when such a function exists (built-in, or defined with
  fn in the file). Otherwise value? is the ? that stops on None or Err, and
  (value)? always is."#,
            "",
        ),
        (
            r#"E001 — Syntax error

The code does not follow the grammar of Rythme at this place. The message
says what was expected. Frequent causes:
  - a missing `)`, `]` or `,`
  - `then` or `else` missing in an `if`
  - braces { } used for a block (Rythme uses a new line and `end`)
  - a keyword from another language (for, while, return, let, def...)

Wrong:   if x > 0 { print(x) }
Right:   if x > 0 then print(x) else print("not positive")"#,
            "",
        ),
        (
            r#"E002 — Text never closed

A text starts with " and must end with " on the same text. A `{` inside a
text starts an insertion and must be closed by `}`.

Wrong:   print("hello)
Right:   print("hello")
To write a brace in a text: "\{not inserted\}""#,
            "",
        ),
        (
            r#"E003 — Unknown name

This name is not defined at this place. Check the spelling (uppercase and
lowercase letters are different), and that the name is defined before it is
used. Inside a function, the parameters and the names defined above are
visible, and local functions written one right after the other see each
other; at the top level, all functions are visible everywhere.

Wrong:   print(lenght([1, 2]))
Right:   print(len([1, 2]))

Rythme suggests the closest existing name when there is one, and the file
that defines it when it is another file of the folder:

  use geometry                   # then
  print(geometry.area(2))        # not area(2)

A name starting with `_` is private to its file: other files cannot use it."#,
            "",
        ),
        (
            r#"E004 — Wrong number of arguments

The function is called with more or fewer values than it takes. Remember that
with `|>`, the value on the left is the first argument.

Wrong:   fn add(a, b) = a + b
         add(1)
Right:   add(1, 2)
Also right:  1 |> add(2)

`rythme help NAME` shows how to call a built-in function."#,
            "",
        ),
        (
            r#"E005 — Wrong kind of value

The operation does not work with this kind of value. Rythme never converts
values silently: "1" + 1 is an error, not "11" or 2.

Wrong:   "age: " + 36
Right:   "age: {36}"              (insert the value in the text)
Right:   "age: " + text(36)       (convert it first)

Conditions must be true or false:
Wrong:   if len(list) then ... else ...
Right:   if len(list) > 0 then ... else ...

When the kinds of the values are certain, `rythme check` finds this mistake
before the program runs, even in code that does not run every time, and in
the calls that give a function what it cannot use: `area("big")` when
`area(side)` computes `side * side`."#,
            "",
        ),
        (
            r#"E006 — No case matches

None of the cases of a `match` (or of the definitions of a function, or the
pattern of a definition like `[a, b] = list`) accepts this value.

Wrong:   fn name(1) = "one"
         name(2)
Right:   fn name(1) = "one"
         fn name(_) = "another number"

`rythme check` warns about forgotten cases before running (W002). When the
kind of the value is certain, it also reports, before running, a case that
can never fit it: `[first, ..rest]` for a Text, `Circle(r)` for a number."#,
            "",
        ),
        (
            r#"E007 — Index out of range

Positions in a list or a text go from 0 to length - 1 (and -1 is the last).

Wrong:   [10, 20][2]
Right:   [10, 20][1]              # 20
Safe:    get([10, 20], 2)         # None, never an error"#,
            "",
        ),
        (
            r#"E008 — Division by zero

Dividing by zero has no meaning. Check the divisor before, or return a Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
            "",
        ),
        (
            r#"E009 — Not a function

Parentheses after a value call it, but only functions can be called.

Wrong:   x = 5
         x(3)
Wrong:   None()                   (None is a value)
Right:   None
To multiply, write it:  2 * (x + 1)"#,
            "",
        ),
        (
            r#"E010 — Unknown field

The record (or the constructor) has no field with this name; or the dict
has no such key (`dict[key]`).

Wrong:   {name: "Ada"}.nmae
Right:   {name: "Ada"}.name
Safe:    get(record, "name")     # an Option, never an error
         get(dict, key)          # the same for a dict

When the fields of the record are certain, `rythme check` finds this mistake
before the program runs, and suggests the closest field."#,
            "",
        ),
        (
            r#"E011 — Already defined

In Rythme a name always means the same value: it cannot be changed or defined
twice in the same place. This is what makes programs easy to follow.

Wrong:   x = 1
         x = x + 1
Right:   x = 1
         y = x + 1

For a function with several cases, write the definitions one right after the
other:
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

The name given to a used file (`use geometry`, `use ... as geo`) cannot name
anything else in the file; and two files used with quotes (`use "a.ry"`)
cannot define the same name: use them by their names instead
(`use a`, then `a.area`)."#,
            "",
        ),
        (
            r#"E012 — Number too large (no longer given)

Whole numbers (Int) have no limit any more: they grow as needed, and stay
exact. Earlier versions of Rythme stopped with this error beyond 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (a Float is approximate)

Only a whole number of more than 100000 digits stops the program, with
E023: it is usually a calculation that grows without end."#,
            "",
        ),
        (
            r#"E013 — unwrap found no value

`unwrap(x)` takes the value out of Some(...) or Ok(...), and stops the
program on None or Err(...). Handle the missing case instead:

  match get(list, 5)
    Some(x) -> x
    None -> 0
  end
or simply:  unwrap_or(get(list, 5), 0)"#,
            "",
        ),
        (
            r#"E014 — Too many nested calls

A function calls itself (or others) too many times without finishing.
Either it never reaches its base case, or it needs to be written so that the
recursive call is the very last thing it does (a tail call): those calls
have no limit.

Limited:     fn sum([]) = 0
             fn sum([x, ..rest]) = x + sum(rest)       # + happens after
No limit:    fn sum([], acc) = acc
             fn sum([x, ..rest], acc) = sum(rest, acc + x)
Simplest:    sum(list)  or  fold(list, 0, fn(a, x) = a + x)"#,
            "",
        ),
        (
            r#"E015 — Assertion failed

`assert(condition)` or `assert_eq(actual, expected)` found that something is
not as expected. In a test, this means the test fails: the message shows the
values to compare.

`rythme check` finds before running an `assert_eq` that can never pass,
because its two values are of kinds that are never equal:
Wrong:   assert_eq(parse_int("3"), 3)      # a Result is never an Int
Right:   assert_eq(parse_int("3"), Ok(3))

`check_all(property, generator)` reports E015 too when a value makes the
property false: it shows the simplest value found, and the seed that gave it
(the same values come back on every run; `rythme test --seed N` tries others)."#,
            "",
        ),
        (
            r#"E016 — Wrong use of ?

`x?` takes the value out of Some(...) or Ok(...); on None or Err(...), it makes
the current function return it immediately. So it only works inside a
function, on an Option or a Result.

  fn total(a, b)
    x = parse_int(a)?
    y = parse_int(b)?
    Ok(x + y)
  end"#,
            "",
        ),
        (
            r#"E017 — File problem

A file could not be read, or a file given to `use` was not found, or files
use each other in a circle. Paths in `use` are relative to the folder of the
file that contains the `use`. Programs are text files saved in UTF-8 (the
usual choice of editors).

`input()` also stops with this error when there is nothing more to read:
the keyboard input was closed (Ctrl-D), or the text sent to the program
(`rythme prog.ry < answers.txt`) has no more lines."#,
            "",
        ),
        (
            r#"E018 — Unexpected character

This character has no meaning in Rythme here. Texts use straight double
quotes "...", powers use **, and comments start with #."#,
            "",
        ),
        (
            r#"E019 — if without else

In Rythme, `if` is an expression: it always gives a value, so both cases are
needed. Each branch holds a single expression; use `do ... end` for several.

Wrong:   if x > 0 then print("positive")
Right:   if x > 0 then print("positive") else ()
Right:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
            "",
        ),
        (
            r#"E020 — Missing end

A function written on several lines, a `match` or a `do` block must finish
with `end` on its own line.

  fn greet(name)
    print("Hello {name}")
  end"#,
            "",
        ),
        (
            r#"E021 — Program stopped by fail

The program called `fail(message)`, which stops everything. For errors that
the caller could handle, return Err(message) instead."#,
            "",
        ),
        (
            r#"E022 — Impossible conversion

The text does not contain a number. When the text comes from a user or a
file, use the safe versions, which give a Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
            "",
        ),
        (
            r#"E023 — Invalid value

The function received a value of the right kind, but outside of what it
accepts: a negative count, the minimum of an empty list, the square root of
a negative number, a date that does not exist (30 February), a range, a
list or a text too large, a whole number of more than 100000 digits... The
message explains the limit."#,
            "",
        ),
        (
            r#"E024 — A value is lost

A line computes a value, but nothing uses it: only the last line of a block
gives its value, and values never change, so the result is simply lost.

Wrong:   total = price
         - discount              # a new line: this is a separate statement
Right:   total = price -
           discount              # end the line with the operator to continue
Wrong:   x + 1                   # does not change x
Right:   y = x + 1
To show a value, print it:  print(x + 1)"#,
            "",
        ),
        (
            r#"E025 — Project or package problem

The `rythme.toml` of the project (or the `rythme.lock` written by
`rythme install`) cannot be read, a dependency is not installed,
`rythme install` cannot download one, or the package index (`rythme
search`, `add`, `publish`) cannot be read or does not have what was asked.
A `rythme.toml` looks like this:

  name = "hello"                 # a Rythme name in lower case
  version = "0.1.0"              # three numbers
  entry = "src/main.ry"          # the program (this one by default)

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }
  geometry = { version = "^1.2.0" }   # from the package index

Every value is a text, in quotes. A dependency from git needs a fixed
version, a `tag` or a `commit`, and the program `git`
(https://git-scm.com). After a change of `[dependencies]`, run
`rythme install` in the folder of the project. More: `rythme help projects`
and `rythme help packages`."#,
            "",
        ),
        (
            r#"E026 — Invalid regex

A regex (a regular expression, given to `matches`, `find_all`, `split_by`,
`replace_all`...) breaks one of its rules, or the replacement of
`replace_all` uses a group that does not exist. The message shows where.

Wrong:   find_all(text, "([0-9]+")       # the `(` is never closed
Right:   find_all(text, "([0-9]+)")
Wrong:   split_by(text, "*")             # `*` repeats nothing
Right:   split_by(text, "[*]")           # the sign * itself, in brackets
Wrong:   replace_all(date, "(\\d+)-(\\d+)", "$3")    # only 2 groups
Right:   replace_all(date, "(\\d+)-(\\d+)", "$2/$1")

In a Rythme text, a backslash is written twice ("\\d") and braces are
written \{ and \} ("[0-9]\{4\}"). `rythme help regex` shows everything a
regex can say. When the regex is written in the code, `rythme check` finds
the mistake before running."#,
            "",
        ),
        (
            r#"W001 — Unused name

A name is defined but never used. It is often a typo or a forgotten step.
If it is on purpose, start the name with `_`: `_unused = ...`."#,
            "",
        ),
        (
            r#"W002 — Forgotten case

A `match` (or a function with several definitions) does not handle every
possible value. The message lists what is missing. If that value comes, the
program will stop with E006.

  type Light = Red | Orange | Green
  fn next(Red) = Green
  fn next(Green) = Orange
  # W002: `next` does not handle `Orange`. Add the missing definition:
  fn next(Orange) = Red"#,
            "",
        ),
        (
            r#"W003 — Built-in function hidden

You defined a name that already belongs to a built-in function. In your file,
the name now means your definition. Choose another name to keep both."#,
            "",
        ),
        (
            r#"W004 — Case never reached

Cases are tried in order, and a value goes to the first one that fits. A
case (or a definition of a function) can never be used when:
  - a case before it accepts everything (`_`, or a plain name): put that
    one last;
  - a case before it already handles the same values: `1 -> ...` twice;
  - the value can never have this shape: a Text never matches a list
    pattern like `[first, ..rest]`, a number never matches `Some(x)`;
  - a range is empty: `9..1` holds no number, write `1..9`.

  match name                     # `name` is a Text
    [first, ..] -> first         # W004: a Text is not a list
    _ -> "?"
  end
To look at the characters of a text, use `chars(name)`."#,
            "",
        ),
        (
            r#"W005 — Unknown function after a dot

`value.name(args)` calls the function `name(value, args)`, or the field `name`
of a record. No function has this name, and no record of the file has such a
field: it is probably a typo.

Wrong:   [1, 2].lenght()
Right:   [1, 2].len()"#,
            "",
        ),
        (
            r#"W006 — Comparison always false

Values of different kinds are never equal: Rythme never converts them
silently. When `x` is a number, `x == "1"` is always false and `x != "1"`
always true. It is often a forgotten conversion, or a value inside an Option
compared as if it were outside.

Wrong:   if count == "3" then ...            (a number and a text)
Right:   if count == 3 then ...
Wrong:   get(scores, "ada") == 19            (an Option and a number)
Right:   get(scores, "ada") == Some(19)
Right:   unwrap_or(get(scores, "ada"), 0) == 19"#,
            "",
        ),
        (
            r#"W007 — Test that can never fail

A test fails only when it stops with an error, or when it ends with `false`,
`None` or `Err(...)`. A test that only computes or prints a value with
functions that never stop, and never compares it with what is expected,
can never fail: it checks nothing.

Wrong:   test "length" = len([1, 2])
Wrong:   test "length" = print(len([1, 2]))
Right:   test "length" = assert_eq(len([1, 2]), 2)
Right:   test "length" = len([1, 2]) == 2"#,
            "",
        ),
        (
            r#"W008 — Result lost

Values never change in Rythme: `push(list, 4)` does not add 4 to `list`, it
gives a new list. A line that only calls such a function, and does nothing
with what it gives, does nothing at all.

Wrong:   push(numbers, 4)               # `numbers` is unchanged
Right:   more = push(numbers, 4)
Wrong:   name.upper()
Right:   print(name.upper())"#,
            "",
        ),
        (
            r#"W009 — A function shown instead of its value

A function shown in a text or printed appears as `<fn name>`. To show what
it gives, call it, with parentheses, even when it takes no argument.

  fn greeting() = "hello"
Wrong:   print("{greeting}")          # shows <fn greeting>
Right:   print("{greeting()}")        # shows hello"#,
            "",
        ),
        (
            r#"W010 — Ranges that overlap

A range pattern `a..b` matches the numbers from `a` to `b`, both included.
When two ranges of a `match` (or of the definitions of a function) share
numbers, these numbers always go to the first one: the second never sees
them. It is often an end written twice.

Wrong:   match age
           0..18 -> "young"
           18..65 -> "adult"         # W010: 18 goes to "young"
           _ -> "senior"
         end
Right:   0..17 -> "young"
         18..64 -> "adult"
For decimal numbers, which fall between 17 and 18, compare instead:
  a if a < 18 -> "young""#,
            "",
        ),
        (
            r#"W011 — Index or subtraction instead of a call

At the start of a statement or of a result, `show [1, 2]` and `show -1`
call `show` without parentheses, unless the file gives `show` a value
somewhere: with `=`, as a parameter of a function or a block, or in a
pattern. Then they are an index and a subtraction, everywhere in the file,
even where `show` is a function. The value may be given far away, to a
parameter of another function.

Wrong:   fn show(xs) = print(xs)
         fn twice(show) = show * 2
         show [1, 2]                  # W011: an index of the function
Right:   show([1, 2])
Or give the parameter another name: fn twice(n) = n * 2"#,
            "",
        ),
        ("Defines a function: `fn name(x) = expression`, or with its body on the following lines, finished by `end`. Without a name, `fn(x) = x * 2` is an anonymous function.", ""),
        ("Chooses between two values: `if condition then value else other`. Every `if` has an `else`, because an `if` always gives a value.", ""),
        ("Separates the condition of an `if` from the value it gives when the condition is true.", ""),
        ("Gives the value of an `if` when the condition is false. It is always required; `else if` chains several conditions.", ""),
        ("The opposite of `if`: `unless condition then a else b` gives `b` when the condition is true, `a` otherwise. Like `if`, it always has an `else`.", ""),
        ("Chooses according to the shape of a value: one case per line, `pattern -> result`, finished by `end`. Cases are tried in order, and `rythme check` warns about forgotten cases.", ""),
        ("Closes a block: a function written on several lines, a `match`, a `do` or a `test`.", ""),
        ("Groups several steps inside an expression, up to `end`. The last line gives the value. After a call, `do |x| ... end` is a block: a function given as the last argument.", ""),
        ("Defines your own type and its variants. Each variant is a constructor, with or without fields.", ""),
        ("Defines a test, next to the code. `rythme test file.ry` runs the tests of the file.", ""),
        ("Uses another file through its name: `use geometry`, then `geometry.area(2)`. The path is relative to the folder of the file that contains `use`.", ""),
        ("True when both conditions are true. The second one is computed only if the first one is true.", ""),
        ("True when at least one of the conditions is true. The second one is computed only if the first one is false.", ""),
        ("Inverts a condition: `not true` is `false`.", ""),
        ("One of the two values of the type Bool.", ""),
        ("Shows values on the screen, separated by spaces, then goes to the next line.", ""),
        ("Shows a value with its position in the code (on the error output) and returns it unchanged. Handy in the middle of a pipeline.", ""),
        ("Converts any value to text. With a number of decimals, writes a number with exactly that many digits after the point.", ""),
        ("Shows the prompt, then reads a line typed on the keyboard and returns it as text.", ""),
        ("Reads all the lines of the standard input, until its end: `Ok(list of texts)`, or `Err(message)` if it cannot be read. For `rythme count.ry < names.txt`, or text typed until Ctrl-D (Ctrl-Z then Enter on Windows).", ""),
        ("Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (for `()`), \"Symbol\" (for `:ok`), \"Option\", \"Result\", or the name of your own type.", ""),
        ("Shows help: about a function (`help(map)`), a topic (`help(\"lists\")`), or what you can do with a value (`help([1, 2])`).", ""),
        ("Stops with an error if the condition is false. Mostly used in tests.", ""),
        ("Stops with an error showing both values if they are different. Mostly used in tests.", ""),
        ("Stops the program with an error message. Prefer returning `Err(...)` for errors that can be handled.", ""),
        ("Returns the list of the words given after the file name on the command line.", ""),
        ("Reads the options of the command line (the words of `args()`, or the list given) into a record, in a Result: `--name=value` gives the text `value`, `--name` and `-n` give `true`, a `-` in a name becomes `_`, and the other words are in the field `args` (all of them after `--`). An option given twice, or a word like `-abc`, gives `Err(message)`. Use `get(opts, \"name\")` or `has(opts, \"name\")` for an option that may be missing.", ""),
        ("The value of an environment variable of the system, as an Option: `Some(text)`, or `None` if it is not set.", ""),
        ("Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not; Rythme itself exits with 1 after an error in the program (see `rythme help system`).", ""),
        ("Returns the current time in seconds (a Float), counted from 1 January 1970. Subtract two times to measure a duration.", ""),
        ("random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.", ""),
        ("Absolute value: the number without its sign.", ""),
        ("The smallest of the values, or of the elements of a list.", ""),
        ("The largest of the values, or of the elements of a list.", ""),
        ("Square root.", ""),
        ("x to the power n (same as `x ** n`).", ""),
        ("Rounds down to a whole number.", ""),
        ("Rounds up to a whole number.", ""),
        ("Rounds to the nearest whole number, or to the given number of decimal digits.", ""),
        ("Converts to a whole number (Int): decimals are cut, texts are read.", ""),
        ("Converts to a decimal number (Float).", ""),
        ("Sine (x in radians).", ""),
        ("Cosine (x in radians).", ""),
        ("Tangent (x in radians).", ""),
        ("Arc sine: the angle (in radians) whose sine is x, for x from -1 to 1.", ""),
        ("Arc cosine: the angle (in radians) whose cosine is x, for x from -1 to 1.", ""),
        ("Arc tangent: the angle (in radians) whose tangent is y. With two numbers, the angle of the point (x, y), from -pi to pi.", ""),
        ("Logarithm: natural (base e) by default, or in the given base.", ""),
        ("e to the power x.", ""),
        ("Is the whole number even?", ""),
        ("Is the whole number odd?", ""),
        ("Greatest common divisor of two whole numbers: the largest number that divides both.", ""),
        ("Least common multiple of two whole numbers: the smallest number divisible by both.", ""),
        ("Length of a text (in characters), a list, a record, a dict or a set.", ""),
        ("The text in uppercase.", ""),
        ("The text in lowercase.", ""),
        ("The text with its first character in uppercase (the rest is unchanged).", ""),
        ("The text without spaces and line breaks at the start and the end.", ""),
        ("Adds `fill` (a space by default) before the text until it is `width` characters long: numbers line up in columns. A value that is not a text is first shown as `print` shows it.", ""),
        ("Adds `fill` (a space by default) after the text until it is `width` characters long.", ""),
        ("Cuts a text into a list of texts. Without separator, cuts at spaces.", ""),
        ("Glues the elements of a list into one text, with the separator between them.", ""),
        ("Does the text contain this piece of text? Does the list (or the set) contain this element? Does the record have this field, or the dict this key?", ""),
        ("Does the text start with this piece of text?", ""),
        ("Does the text end with this piece of text?", ""),
        ("Replaces every occurrence of `old` by `new`.", ""),
        ("The list of the characters of a text.", ""),
        ("The list of the lines of a text.", ""),
        ("The number of a character (its Unicode code point).", ""),
        ("The character with this number (Unicode code point): the opposite of `char_code`.", ""),
        ("Is the text made only of digits from 0 to 9 (and not empty)?", ""),
        ("Is the text made only of letters, accented ones included (and not empty)?", ""),
        ("Is the text made only of spaces, tabs and line breaks (and not empty)?", ""),
        ("A text or a list repeated n times.", ""),
        ("Reads a whole number from a text: `Ok(number)`, or `Err(message)` if the text is not a number.", ""),
        ("Reads a decimal number from a text: `Ok(number)` or `Err(message)`.", ""),
        ("Reads a JSON text: `Ok(value)`, or `Err(message)` if the text is not valid JSON. Objects become records, arrays lists, and null `None`.", ""),
        ("Writes a value as JSON text: records become objects, lists arrays, and `None` null. With `indent`, one element per line, shifted by that many spaces.", ""),
        ("The text without spaces and line breaks at the start.", ""),
        ("The text without spaces and line breaks at the end.", ""),
        ("The list of the words of a text: letters and digits, accents included, with `'` or `-` inside a word. Punctuation and spaces separate them.", ""),
        ("Are the two texts the same when uppercase and lowercase letters count as equal?", ""),
        ("Writes a number for people: thousands separated, with exactly `decimals` decimals if given, in the style of a language: \"en\" (1,234.5, by default), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" or \"pt\" (1.234,5). To line it up in a column, use `pad_left`.", ""),
        ("Does the whole text match the regex (a regular expression: see `rythme help regex`)? To look for the regex anywhere in the text, use `find_first`.", ""),
        ("The first piece of the text that matches the regex, as an Option: `Some(piece)`, or `None` if there is none.", ""),
        ("Every piece of the text that matches the regex, from left to right.", ""),
        ("The first match of the regex, as an Option of a list: the whole piece, then the text of each group `(...)`, in order (\"\" for a group that matched nothing).", ""),
        ("Like `captures`, for every match: a list of lists.", ""),
        ("The first match of a regex with named groups `(?<name>...)`, as an Option of a record: one field for each name.", ""),
        ("Cuts a text at every piece that matches the regex.", ""),
        ("Replaces every piece that matches the regex. `new` is a text, where `$1` stands for the text of group 1 (`$<name>` for a named group, `$0` for the whole piece, `$$` for a `$`), or a function that receives each piece and gives its replacement.", ""),
        ("Applies the function f to each element and returns the list of the results. Also works on Option and Result, and on a set (giving the set of the results).", ""),
        ("Keeps only the elements for which f returns true. Also works on a set, and on a dict, where f receives each key and its value.", ""),
        ("Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.", ""),
        ("Like fold, but gives the list of every intermediate result: running totals, for example.", ""),
        ("Calls f on each element, for its effect (like printing). Returns (). Also works on a set, and on a dict, where f receives each key and its value.", ""),
        ("The sum of the numbers of a list; with f, the sum of f(x) for each element.", ""),
        ("The product of the numbers of a list.", ""),
        ("The first element of a list (or character of a text), as an Option: `Some(x)`, or `None` if there is none. With n, the first n elements (like `take`).", ""),
        ("The last element of a list (or character of a text), as an Option. With n, the last n elements.", ""),
        ("The list (or text) without its first element.", ""),
        ("The element at a position of a list or text, the field of a record, or the value of a key of a dict, as an Option (never an error). Negative positions count from the end.", ""),
        ("A new list with x added at the end; or a new set with x added.", ""),
        ("A new list where the element at position i is replaced by x. Negative positions count from the end.", ""),
        ("A new list with x inserted so that it is at position i (-1 adds it at the end).", ""),
        ("A new list without the element at position i. Negative positions count from the end.", ""),
        ("The list or text in reverse order.", ""),
        ("The list sorted from smallest to largest (numbers, texts in alphabetical order...). For the largest first: `sort(list) |> reverse`.", ""),
        ("Sorts the list by the value that the function `key` gives for each element. Elements with the same value keep their order.", ""),
        ("The list without duplicates (the first occurrence is kept).", ""),
        ("The elements of the list in a random order.", ""),
        ("The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.", ""),
        ("The first n elements of a list or text.", ""),
        ("The list or text without its first n elements.", ""),
        ("The first elements of a list (or characters of a text), as long as f is true for them.", ""),
        ("The list (or text) without its first elements for which f is true.", ""),
        ("One element out of n of a list (or text), starting with the first. On a range: the numbers with a step.", ""),
        ("Cuts a list (or text) into pieces of n elements; the last piece may be shorter.", ""),
        ("Every group of n neighbouring elements of a list (or text), in order: handy to compare each element with the next one.", ""),
        ("Pairs each element with its position: [[0, x0], [1, x1], ...].", ""),
        ("Is f true for at least one element?", ""),
        ("Is f true for every element?", ""),
        ("The first element for which f is true, as an Option.", ""),
        ("How many elements make f true; without f, how many elements there are.", ""),
        ("The elements for which f is false: the opposite of `filter`. Also works on a set and on a dict, like `filter`.", ""),
        ("Calls f(element, position) for each element, in order (positions start at 0). For effects, like `print`.", ""),
        ("The position of an element in a list (or of a piece of text in a text), as an Option.", ""),
        ("The element for which the function `key` gives the smallest value (the first one if there are several).", ""),
        ("The element for which the function `key` gives the largest value (the first one if there are several).", ""),
        ("Joins a list of lists into one list.", ""),
        ("Like map, when f returns a list: the lists are joined.", ""),
        ("Splits a list in two: [elements where f is true, elements where f is false].", ""),
        ("Groups the elements in a record: the key is the text of what `key` returns, the value the list of matching elements.", ""),
        ("Is the list, text, record, dict or set empty?", ""),
        ("The list of the field names of a record, or of the keys of a dict (in order).", ""),
        ("The list of the values of a record or of a dict (in the order of its keys), or of the elements of a set (in order).", ""),
        ("The list of the [name, value] pairs of a record, or of the [key, value] pairs of a dict.", ""),
        ("Builds a record from a list of [name, value] pairs: the opposite of `entries`.", ""),
        ("Does the record have this field? Does the dict have this key, or the set this element?", ""),
        ("A new record with the field set to the value (added or replaced); or a new dict with the key set to the value.", ""),
        ("A new record without this field; or a new dict without this key, or a new set without this element.", ""),
        ("A new record (or dict) where f is applied to every value; the names (or keys) stay the same.", ""),
        ("A dictionary: keys of any kind (but functions), each with a value, kept in the order of the keys. Made from a list of [key, value] pairs (the last value of a key wins), from a record, or empty with `dict()`. Use it with get, put, has, remove, keys, values, entries, len, map_values, filter, union...", ""),
        ("A set: values (but functions) without repetition, kept in order. Made from a list, or empty with `set()`. Use it with has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.", ""),
        ("The elements of two sets together; or the pairs of two dicts, where the value of b wins for a key in both.", ""),
        ("The elements of the set a that are also in b; or the pairs of the dict a whose key is in b.", ""),
        ("The elements of the set a that are not in b; or the pairs of the dict a whose key is not in b.", ""),
        ("Is every element of the set a also in the set b?", ""),
        ("The value inside `Some(x)` or `Ok(x)`. Stops the program with an error for `None` or `Err(e)`.", ""),
        ("The value inside `Some(x)` or `Ok(x)`, or the default value for `None` or `Err(e)`.", ""),
        ("Is it `Some(...)`?", ""),
        ("Is it `None`?", ""),
        ("Is it `Ok(...)`?", ""),
        ("Is it `Err(...)`?", ""),
        ("Chains steps that may fail: for `Some(x)` or `Ok(x)`, gives f(x), which must be an Option or a Result; `None` and `Err(e)` stay as they are.", ""),
        ("Turns an Option into a Result: `Some(x)` gives `Ok(x)`, and `None` gives `Err(error)`. Handy before `?`.", ""),
        ("For `Err(e)`, gives `Err(f(e))`; `Ok(x)` stays as it is. Handy to explain an error better.", ""),
        ("Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.", ""),
        ("Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.", ""),
        ("Adds a text at the end of a file, creating the file if needed: `Ok(())` or `Err(message)`.", ""),
        ("Does this file (or folder) exist? Always `false` in the browser.", ""),
        ("What is known about a file or a folder: `Ok({is_folder, size})`, with its size in bytes (0 for a folder), or `Err(message)` if it does not exist or cannot be read.", ""),
        ("The names of the files and folders inside a folder, in alphabetical order: `Ok(list of texts)`, or `Err(message)`. `list_folder(\".\")` lists the current folder.", ""),
        ("Creates a folder, and the folders around it that are missing: `Ok(())`, also when it already exists, or `Err(message)`.", ""),
        ("Deletes one file (never a folder: that is an error): `Ok(())` or `Err(message)`. A deleted file cannot be brought back.", ""),
        ("Joins the parts of a path with `/`, which works on Linux, macOS and Windows (with `\\` if the parts already use only `\\`). Only the first part can start at a root (`/`, `C:`): a part like \"/etc\" after another one is an error.", ""),
        ("The parts of a path, split at each `/` or `\\`; the first part of an absolute path is its root (`/`, `C:\\`...). `join_path` puts them back together.", ""),
        ("The date and time now, where the program runs: a record {year, month, day, hour, minute, second, offset}, where `offset` is the number of minutes the local time is ahead of UTC (0 when the system does not tell).", ""),
        ("The date of today, where the program runs: a record {year, month, day}.", ""),
        ("Is the value a valid date: a record {year, month, day} (with `hour`, `minute`, `second`, `offset` if needed) for a day that exists?", ""),
        ("The date n days later (or earlier, when n is negative), at the same time of day.", ""),
        ("The date and time n seconds later (or earlier, when n is negative). Add `60 * minutes` or `3600 * hours` the same way.", ""),
        ("The number of whole days from date a to date b: negative when b is before a.", ""),
        ("The number of seconds from date a to date b, taking their offsets into account: negative when b is before a.", ""),
        ("The day of the week of a date, from 1 (Monday) to 7 (Sunday), as in ISO 8601.", ""),
        ("The number of days of a month (from 1 to 12) of a year: 28 to 31.", ""),
        ("Does the year have a 29 February? Every 4 years, except every 100 years, except every 400 years.", ""),
        ("Checks that a property holds for many values made by a generator (100, or `cases`): the function is called with each value and must give `true` (or `()`, `Some(...)`, `Ok(...)`). When a value makes it false or stops it with an error, that value is simplified as much as possible (shrinking) and shown, with the seed that made it: the same values come back on every run. The property may also come last, as a block: `check_all(ints()) { |x| x + 0 == x }`.", ""),
        ("A list of `n` values (10 if not given) made by a generator, to see what `check_all` will try.", ""),
        ("A generator of whole numbers from `min` to `max` (from -1000 to 1000 if not given), often near 0 and at the edges. A failing number is simplified towards 0.", ""),
        ("A generator of decimal numbers from `min` to `max` (from -1000.0 to 1000.0 if not given).", ""),
        ("A generator of texts of at most `max_length` characters (20 if not given): letters mostly, with digits, spaces, punctuation and accents. A failing text is made shorter and simpler.", ""),
        ("A generator of `true` and `false`.", ""),
        ("A generator of lists of at most `max_length` elements (10 if not given), each made by `generator`. A failing list is made shorter, then its elements simpler.", ""),
        ("A generator that picks one of the values of a list. A failing value is replaced by one written before it.", ""),
        ("A generator of records, with a generator for each field: `records_of({name: texts(), age: ints(0, 120)})`. Handy to give several values to `check_all`.", ""),
        ("January", ""),
        ("February", ""),
        ("March", ""),
        ("April", ""),
        ("May", ""),
        ("June", ""),
        ("July", ""),
        ("August", ""),
        ("September", ""),
        ("October", ""),
        ("November", ""),
        ("December", ""),
        // docs/exercises: the exercises of `rythme exercise` (see src/exercise.rs)
        ("# --- Tests (do not change them) ---", ""),
        ("to do", ""),
        ("01-values", ""),
        (
            r#"# Exercise 01 · First values  ★☆☆
#
# A name receives a value with `=` and keeps it forever: in Rythme, values
# never change.
#
# Statement
#   `seconds_per_minute` is given. Replace each `0` (and the empty text)
#   below with an expression that uses the names defined before it:
#   - `seconds_per_hour`: the number of seconds in one hour;
#   - `seconds_per_day`: the number of seconds in one day;
#   - `seconds_per_week`: the number of seconds in one week;
#   - `summary`: the text "A week has 604800 seconds.", where the number
#     comes from `seconds_per_week` (do not type it yourself).
#
# Hints
#   1. An hour has 60 minutes: `60 * seconds_per_minute`.
#   2. A day has 24 hours, a week has 7 days.
#   3. Inside a text, `{name}` is replaced by the value of `name`:
#      "I have {count} apples".
#
# Check your work:  rythme exercise check 01-values.ry
#              or:  rythme test 01-values.ry

seconds_per_minute = 60
seconds_per_hour = 0
seconds_per_day = 0
seconds_per_week = 0
summary = ""

# --- Tests (do not change them) ---

test "seconds in an hour" = assert_eq(seconds_per_hour, 3600)

test "seconds in a day" = assert_eq(seconds_per_day, 86400)

test "seconds in a week" = assert_eq(seconds_per_week, 604800)

test "the summary" = assert_eq(summary, "A week has 604800 seconds.")
"#,
            "",
        ),
        ("02-texts", ""),
        (
            r#"# Exercise 02 · Texts  ★☆☆
#
# A function takes values and gives back a new value:
#   fn double(x) = x * 2
#
# Statement
#   Write the four functions below so that their tests pass:
#   - `greet(name)` gives "Hello, Ada!" for "Ada";
#   - `shout(text)` gives the text in uppercase followed by "!";
#   - `initials(first, last)` gives "A.L." for "Ada" and "Lovelace";
#   - `introduce(name, age)` gives "Ada is 36 years old.".
#
# Hints
#   1. "Hello, {name}!" inserts the value of `name` into the text.
#   2. `upper("hi")` is "HI", and `+` joins two texts: "HI" + "!".
#   3. `word[0]` is the first character of `word`; it can go inside braces
#      too: "{word[0]}.".
#
# Check your work:  rythme exercise check 02-texts.ry
#              or:  rythme test 02-texts.ry

## A greeting for `name`.
fn greet(name) = fail("to do")

## The text in uppercase, followed by "!".
fn shout(text) = fail("to do")

## The initials of a person, each followed by a dot.
fn initials(first, last) = fail("to do")

## A sentence that introduces a person.
fn introduce(name, age) = fail("to do")

# --- Tests (do not change them) ---

test "greet" = assert_eq(greet("Ada"), "Hello, Ada!")

test "greet someone else" = assert_eq(greet("Alan"), "Hello, Alan!")

test "shout"
  assert_eq(shout("hello"), "HELLO!")
  assert_eq(shout("Stop"), "STOP!")
end

test "initials"
  assert_eq(initials("Ada", "Lovelace"), "A.L.")
  assert_eq(initials("grace", "hopper"), "g.h.")
end

test "introduce" = assert_eq(introduce("Ada", 36), "Ada is 36 years old.")
"#,
            "",
        ),
        ("03-arithmetic", ""),
        (
            r#"# Exercise 03 · Arithmetic  ★☆☆
#
# Numbers are whole (Int: 3) or decimal (Float: 3.5). `/` always gives a
# Float (`7 / 2` is 3.5); `//` divides and rounds down (`7 // 2` is 3) and
# `%` gives the remainder (`7 % 2` is 1).
#
# Statement
#   - `square(x)`: x times x;
#   - `average(a, b)`: the average of two numbers;
#   - `fahrenheit(celsius)`: the temperature in degrees Fahrenheit
#     (multiply by 9, divide by 5, add 32);
#   - `hypotenuse(a, b)`: the long side of a right triangle whose two other
#     sides are a and b (the square root of a² + b²);
#   - `minutes_and_seconds(total)`: for a number of seconds, a text like
#     "2 min 5 s" (for 125).
#
# Hints
#   1. `sqrt(16)` is 4.0.
#   2. `125 // 60` is 2 and `125 % 60` is 5.
#   3. `3 == 3.0` is true: an Int and a Float with the same value are equal.
#
# Check your work:  rythme exercise check 03-arithmetic.ry
#              or:  rythme test 03-arithmetic.ry

fn square(x) = fail("to do")

fn average(a, b) = fail("to do")

fn fahrenheit(celsius) = fail("to do")

fn hypotenuse(a, b) = fail("to do")

fn minutes_and_seconds(total) = fail("to do")

# --- Tests (do not change them) ---

test "square"
  assert_eq(square(3), 9)
  assert_eq(square(-4), 16)
  assert_eq(square(1.5), 2.25)
end

test "average"
  assert_eq(average(2, 4), 3)
  assert_eq(average(1, 2), 1.5)
end

test "fahrenheit"
  assert_eq(fahrenheit(0), 32)
  assert_eq(fahrenheit(100), 212)
  assert_eq(fahrenheit(-40), -40)
end

test "hypotenuse"
  assert_eq(hypotenuse(3, 4), 5)
  assert_eq(hypotenuse(5, 12), 13)
end

test "minutes and seconds"
  assert_eq(minutes_and_seconds(125), "2 min 5 s")
  assert_eq(minutes_and_seconds(59), "0 min 59 s")
  assert_eq(minutes_and_seconds(600), "10 min 0 s")
end
"#,
            "",
        ),
        ("04-choosing", ""),
        (
            r#"# Exercise 04 · Choosing with `if`  ★☆☆
#
# `if condition then value else other_value` is an expression: it gives a
# value, so `else` is always needed. `else if` chains several conditions.
#
# Statement
#   Without using the built-in functions `abs`, `min` and `max`:
#   - `absolute(n)`: n without its sign (`absolute(-5)` is 5);
#   - `larger(a, b)`: the larger of two numbers;
#   - `grade(score)`: for a score from 0 to 100, "A" from 90, "B" from 80,
#     "C" from 70, "D" from 60, and "F" below 60;
#   - `ticket_price(age)`: 0 under 4 years, 5 under 12, 12 from 65, and 10
#     otherwise.
#
# Hints
#   1. `if n < 0 then -n else n`
#   2. Test the highest grade first: `if score >= 90 then "A" else if ...`.
#   3. A long `if` can continue on the next lines, each starting with
#      `else if`.
#
# Check your work:  rythme exercise check 04-choosing.ry
#              or:  rythme test 04-choosing.ry

fn absolute(n) = fail("to do")

fn larger(a, b) = fail("to do")

fn grade(score) = fail("to do")

fn ticket_price(age) = fail("to do")

# --- Tests (do not change them) ---

test "absolute"
  assert_eq(absolute(-5), 5)
  assert_eq(absolute(7), 7)
  assert_eq(absolute(0), 0)
end

test "larger"
  assert_eq(larger(3, 8), 8)
  assert_eq(larger(8, 3), 8)
  assert_eq(larger(-1, -2), -1)
end

test "grade"
  assert_eq(grade(95), "A")
  assert_eq(grade(90), "A")
  assert_eq(grade(85), "B")
  assert_eq(grade(70), "C")
  assert_eq(grade(61), "D")
  assert_eq(grade(12), "F")
end

test "ticket price"
  assert_eq(ticket_price(2), 0)
  assert_eq(ticket_price(4), 5)
  assert_eq(ticket_price(11), 5)
  assert_eq(ticket_price(30), 10)
  assert_eq(ticket_price(65), 12)
end
"#,
            "",
        ),
        ("05-logic", ""),
        (
            r#"# Exercise 05 · True or false  ★☆☆
#
# Comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) give a Bool: `true` or
# `false`. `and`, `or` and `not` combine Bools.
#
# Statement
#   Each function gives a Bool, and none of them needs `if`:
#   - `is_multiple(n, d)`: is n a multiple of d?
#   - `is_between(x, low, high)`: is x from low to high (both included)?
#   - `is_leap(year)`: a year is a leap year when it is a multiple of
#     4, except the multiples of 100 that are not multiples of 400 (2000 is
#     a leap year, 1900 is not); the library has it too, as
#     `is_leap_year`, but here you write the rule yourself;
#   - `exclusive_or(a, b)`: is exactly one of a and b true?
#
# Hints
#   1. n is a multiple of d when the remainder `n % d` is 0.
#   2. `x >= low and x <= high`
#   3. Write the rule for leap years with the first function:
#      `is_multiple(year, 4) and (...)`.
#   4. Two Bools can be compared with `!=` too.
#
# Check your work:  rythme exercise check 05-logic.ry
#              or:  rythme test 05-logic.ry

fn is_multiple(n, d) = fail("to do")

fn is_between(x, low, high) = fail("to do")

fn is_leap(year) = fail("to do")

fn exclusive_or(a, b) = fail("to do")

# --- Tests (do not change them) ---

test "multiples"
  assert(is_multiple(12, 3), "12 is a multiple of 3")
  assert(not is_multiple(12, 5), "12 is not a multiple of 5")
  assert(is_multiple(0, 7), "0 is a multiple of every number")
end

test "between"
  assert(is_between(5, 1, 10), "5 is between 1 and 10")
  assert(is_between(1, 1, 10), "the bounds are included")
  assert(is_between(10, 1, 10), "the bounds are included")
  assert(not is_between(11, 1, 10), "11 is too big")
  assert(not is_between(0, 1, 10), "0 is too small")
end

test "leap years"
  assert_eq(is_leap(2024), true)
  assert_eq(is_leap(2023), false)
  assert_eq(is_leap(1900), false)
  assert_eq(is_leap(2000), true)
end

test "exclusive or"
  assert_eq(exclusive_or(true, false), true)
  assert_eq(exclusive_or(false, true), true)
  assert_eq(exclusive_or(true, true), false)
  assert_eq(exclusive_or(false, false), false)
end
"#,
            "",
        ),
        ("06-cases", ""),
        (
            r#"# Exercise 06 · One definition per case  ★☆☆
#
# A function can be written as several definitions, tried in order. A
# parameter can be a value (`fn f(0) = ...`), and a guard (`if ...` before
# the `=`) adds a condition:
#   fn describe(0) = "zero"
#   fn describe(n) if n < 0 = "negative"
#   fn describe(_) = "positive"
#
# Statement
#   - `plural(n, word)`: "1 cat" for 1, but "0 cats", "2 cats"...
#   - `fizzbuzz(n)`: "FizzBuzz" if n is a multiple of 3 and of 5, "Fizz"
#     for the other multiples of 3, "Buzz" for the other multiples of 5,
#     and the number as a text otherwise;
#   - `day_kind(day)`: "weekend" for "Saturday" and "Sunday", "weekday"
#     for any other text.
#
# Hints
#   1. Start with the special case: `fn plural(1, word) = ...`, then the
#      general case on the next line.
#   2. The order matters: test "FizzBuzz" before "Fizz" and "Buzz".
#   3. `text(15)` is "15".
#   4. `_` accepts anything, and is usually the last case.
#
# Check your work:  rythme exercise check 06-cases.ry
#              or:  rythme test 06-cases.ry

fn plural(n, word) = fail("to do")

fn fizzbuzz(n) = fail("to do")

fn day_kind(day) = fail("to do")

# --- Tests (do not change them) ---

test "plural"
  assert_eq(plural(1, "cat"), "1 cat")
  assert_eq(plural(2, "cat"), "2 cats")
  assert_eq(plural(0, "dog"), "0 dogs")
end

test "fizzbuzz"
  assert_eq(fizzbuzz(1), "1")
  assert_eq(fizzbuzz(3), "Fizz")
  assert_eq(fizzbuzz(10), "Buzz")
  assert_eq(fizzbuzz(30), "FizzBuzz")
  assert_eq(fizzbuzz(31), "31")
end

test "the first fifteen" = assert_eq(1..15 |> map(fizzbuzz) |> join(" "), "1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz")

test "day kind"
  assert_eq(day_kind("Saturday"), "weekend")
  assert_eq(day_kind("Sunday"), "weekend")
  assert_eq(day_kind("Monday"), "weekday")
end
"#,
            "",
        ),
        ("07-recursion", ""),
        (
            r#"# Exercise 07 · Recursion  ★★☆
#
# Rythme has no loops: to repeat, a function calls itself. Every recursive
# function needs a base case, which stops, and a recursive case, which
# brings the problem closer to the base case:
#   fn countdown(0) = "liftoff!"
#   fn countdown(n) = "{n} " + countdown(n - 1)
#
# Statement
#   - `factorial(n)`: 1 × 2 × ... × n, and `factorial(0)` is 1;
#   - `sum_to(n)`: 1 + 2 + ... + n, and `sum_to(0)` is 0;
#   - `power(x, n)`: x to the power n, for n ≥ 0, without `**` or `pow`.
#
# Hints
#   1. `factorial(5)` is `5 * factorial(4)`.
#   2. What is `power(x, 0)`? And `power(x, n)` compared with
#      `power(x, n - 1)`?
#   3. A function that never reaches its base case stops with error E014:
#      check that n gets smaller at each call.
#
# Check your work:  rythme exercise check 07-recursion.ry
#              or:  rythme test 07-recursion.ry

fn factorial(n) = fail("to do")

fn sum_to(n) = fail("to do")

fn power(x, n) = fail("to do")

# --- Tests (do not change them) ---

test "factorial"
  assert_eq(factorial(0), 1)
  assert_eq(factorial(1), 1)
  assert_eq(factorial(5), 120)
  assert_eq(factorial(10), 3628800)
end

test "sum to"
  assert_eq(sum_to(0), 0)
  assert_eq(sum_to(1), 1)
  assert_eq(sum_to(10), 55)
  assert_eq(sum_to(100), 5050)
end

test "power"
  assert_eq(power(2, 0), 1)
  assert_eq(power(2, 10), 1024)
  assert_eq(power(3, 4), 81)
  assert_eq(power(1.5, 2), 2.25)
end
"#,
            "",
        ),
        ("08-digits", ""),
        (
            r#"# Exercise 08 · The digits of a number  ★★☆
#
# `n % 10` is the last digit of n, and `n // 10` is n without its last
# digit: for 472, they are 2 and 47.
#
# Statement
#   For whole numbers n ≥ 0, and with recursion (without turning the number
#   into a text):
#   - `digit_count(n)`: how many digits n has (0 has one digit);
#   - `digit_sum(n)`: the sum of the digits of n;
#   - `reverse_number(n)`: the digits in the opposite order (1230 gives 321).
#
# Hints
#   1. A number below 10 has a single digit: that is the base case.
#   2. `digit_sum(472)` is `2 + digit_sum(47)`.
#   3. For `reverse_number`, write a helper with one more parameter, the
#      number built so far: `reverse_from(n, built)`, with
#      `reverse_from(0, built) = built`. Each step moves the last digit of n
#      to the end of `built`: `built * 10 + n % 10`.
#
# Check your work:  rythme exercise check 08-digits.ry
#              or:  rythme test 08-digits.ry

fn digit_count(n) = fail("to do")

fn digit_sum(n) = fail("to do")

fn reverse_number(n) = fail("to do")

# --- Tests (do not change them) ---

test "digit count"
  assert_eq(digit_count(0), 1)
  assert_eq(digit_count(7), 1)
  assert_eq(digit_count(472), 3)
  assert_eq(digit_count(1000000), 7)
end

test "digit sum"
  assert_eq(digit_sum(0), 0)
  assert_eq(digit_sum(472), 13)
  assert_eq(digit_sum(99999), 45)
end

test "reverse number"
  assert_eq(reverse_number(472), 274)
  assert_eq(reverse_number(1230), 321)
  assert_eq(reverse_number(5), 5)
end
"#,
            "",
        ),
        ("09-accumulators", ""),
        (
            r#"# Exercise 09 · Accumulators and tail calls  ★★☆
#
# In `n + sum_to(n - 1)`, the addition waits for the call to finish: a
# million nested calls stop with error E014 ("too many nested calls"). When
# the recursive call is the very last thing the function does (a tail
# call), nothing waits, and there is no limit. An extra parameter, the
# accumulator, carries the result built so far:
#   fn sum_from(0, total) = total
#   fn sum_from(n, total) = sum_from(n - 1, total + n)
#
# Statement
#   - `big_sum(n)`: 1 + 2 + ... + n, which must work for n = 1000000;
#   - `collatz_steps(n)`: for n ≥ 1, the number of steps to reach 1, where
#     a step divides an even number by 2 and turns an odd number into
#     3 × n + 1 (6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 takes 8 steps);
#   - `fib(n)`: the n-th Fibonacci number (0, 1, 1, 2, 3, 5, 8...: each
#     one is the sum of the two before), fast even for n = 90.
#
# Hints
#   1. `big_sum(n)` can simply call a helper: `sum_from(n, 0)`.
#   2. `collatz_from(n, steps)`: when n is 1, the answer is `steps`.
#   3. `fib(n) = fib(n - 1) + fib(n - 2)` is correct but very slow, because
#      it computes the same numbers again and again. Carry the two last
#      numbers instead: `fib_from(n, a, b)`, where a and b are two
#      neighbouring Fibonacci numbers, starting with `fib_from(n, 0, 1)`.
#
# Check your work:  rythme exercise check 09-accumulators.ry
#              or:  rythme test 09-accumulators.ry

fn big_sum(n) = fail("to do")

fn collatz_steps(n) = fail("to do")

fn fib(n) = fail("to do")

# --- Tests (do not change them) ---

test "big sum"
  assert_eq(big_sum(10), 55)
  assert_eq(big_sum(1000000), 500000500000)
end

test "collatz steps"
  assert_eq(collatz_steps(1), 0)
  assert_eq(collatz_steps(6), 8)
  assert_eq(collatz_steps(27), 111)
end

test "fibonacci"
  assert_eq(0..10 |> map(fib), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55])
  assert_eq(fib(90), 2880067194370816120)
end
"#,
            "",
        ),
        ("10-divisors", ""),
        (
            r#"# Exercise 10 · Divisors and prime numbers  ★★☆
#
# Recursion can also try every candidate, one after the other, until an
# answer is found.
#
# Statement
#   - `my_gcd(a, b)`: the greatest common divisor of two whole numbers
#     ≥ 0, with Euclid's method: the gcd of a and 0 is a, and the gcd of a
#     and b is the gcd of b and `a % b` (without the built-in `gcd`);
#   - `is_prime(n)`: is n a prime number (a number ≥ 2 whose only divisors
#     are 1 and itself)?
#   - `next_prime(n)`: the smallest prime number strictly greater than n.
#
# Hints
#   1. Euclid's method is already written as two cases: translate it.
#   2. For `is_prime`, write a helper `no_divisor_from(n, d)` that tries d,
#      then d + 1... It can stop as soon as `d * d > n`.
#   3. `next_prime(n)` tries n + 1: if it is prime, that is the answer;
#      otherwise, the answer is `next_prime(n + 1)`.
#
# Check your work:  rythme exercise check 10-divisors.ry
#              or:  rythme test 10-divisors.ry

fn my_gcd(a, b) = fail("to do")

fn is_prime(n) = fail("to do")

fn next_prime(n) = fail("to do")

# --- Tests (do not change them) ---

test "gcd"
  assert_eq(my_gcd(12, 18), 6)
  assert_eq(my_gcd(17, 5), 1)
  assert_eq(my_gcd(7, 0), 7)
  assert_eq(my_gcd(0, 9), 9)
end

test "prime numbers"
  assert_eq(1..30 |> filter(is_prime), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
  assert_eq(is_prime(0), false)
  assert_eq(is_prime(7919), true)
  assert_eq(is_prime(7917), false)
end

test "next prime"
  assert_eq(next_prime(1), 2)
  assert_eq(next_prime(13), 17)
  assert_eq(next_prime(100), 101)
end
"#,
            "",
        ),
        ("11-list-patterns", ""),
        (
            r#"# Exercise 11 · Taking lists apart  ★★☆
#
# A list is either empty, `[]`, or a first element followed by the rest of
# the list, `[first, ..rest]`. A function on lists usually has one
# definition for each of these two cases:
#   fn count_items([]) = 0
#   fn count_items([_, ..rest]) = 1 + count_items(rest)
#
# Statement
#   With recursion, and without the built-in functions `len`, `sum`, `max`
#   and `contains`:
#   - `total(list)`: the sum of the numbers of the list;
#   - `largest(list)`: the largest element of a list that is not empty;
#   - `has_value(list, x)`: is x an element of the list?
#   - `last_one(list)`: the last element of a list that is not empty.
#
# Hints
#   1. The total of an empty list is 0.
#   2. A list of one element, `[x]`, is a good base case for `largest` and
#      `last_one`.
#   3. The largest of `[x, ..rest]` is the larger of x and of the largest of
#      rest.
#   4. `has_value`: an empty list contains nothing; otherwise, either the
#      first element is x, or x is in the rest (`or`).
#   5. `rythme check` warns (W002) about a forgotten case, here the empty
#      list. It has no largest element: say so with
#      `fn largest([]) = fail("the list is empty")`.
#
# Check your work:  rythme exercise check 11-list-patterns.ry
#              or:  rythme test 11-list-patterns.ry

fn total(list) = fail("to do")

fn largest(list) = fail("to do")

fn has_value(list, x) = fail("to do")

fn last_one(list) = fail("to do")

# --- Tests (do not change them) ---

test "total"
  assert_eq(total([]), 0)
  assert_eq(total([4]), 4)
  assert_eq(total([1, 2, 3, 4]), 10)
end

test "largest"
  assert_eq(largest([7]), 7)
  assert_eq(largest([3, 9, 2]), 9)
  assert_eq(largest([-3, -1, -2]), -1)
end

test "has value"
  assert_eq(has_value([1, 2, 3], 2), true)
  assert_eq(has_value([1, 2, 3], 5), false)
  assert_eq(has_value([], 1), false)
  assert_eq(has_value(["a", "b"], "b"), true)
end

test "last one"
  assert_eq(last_one([1, 2, 3]), 3)
  assert_eq(last_one(["only"]), "only")
end
"#,
            "",
        ),
        ("12-building-lists", ""),
        (
            r#"# Exercise 12 · Building lists  ★★☆
#
# `[x, ..rest]` also builds a list: x followed by the elements of rest.
# Lists never change: a function gives back a new list.
#   fn zeros(0) = []
#   fn zeros(n) = [0, ..zeros(n - 1)]
#
# Statement
#   With recursion, and without `map`, `reverse` or ranges (`1..n`):
#   - `countdown(n)`: the list [n, n - 1, ..., 1, 0];
#   - `repeat_value(x, n)`: a list with n times the value x;
#   - `double_all(list)`: each number of the list doubled;
#   - `reversed(list)`: the elements in the opposite order.
#
# Hints
#   1. `countdown(0)` is `[0]`.
#   2. `double_all([x, ..rest])` starts with `x * 2`.
#   3. For `reversed`, use an accumulator: `reversed_onto(list, done)` moves
#      the first element of list to the front of done, until list is empty.
#      `reversed_onto([1, 2], [])` → `reversed_onto([2], [1])` →
#      `reversed_onto([], [2, 1])`.
#
# Check your work:  rythme exercise check 12-building-lists.ry
#              or:  rythme test 12-building-lists.ry

fn countdown(n) = fail("to do")

fn repeat_value(x, n) = fail("to do")

fn double_all(list) = fail("to do")

fn reversed(list) = fail("to do")

# --- Tests (do not change them) ---

test "countdown"
  assert_eq(countdown(0), [0])
  assert_eq(countdown(3), [3, 2, 1, 0])
end

test "repeat value"
  assert_eq(repeat_value("a", 3), ["a", "a", "a"])
  assert_eq(repeat_value(1, 0), [])
end

test "double all"
  assert_eq(double_all([1, 2, 3]), [2, 4, 6])
  assert_eq(double_all([]), [])
end

test "reversed"
  assert_eq(reversed([1, 2, 3]), [3, 2, 1])
  assert_eq(reversed([]), [])
  assert_eq(reversed(["x"]), ["x"])
end

test "the original list does not change"
  numbers = [1, 2, 3]
  reversed(numbers)
  assert_eq(numbers, [1, 2, 3])
end
"#,
            "",
        ),
        ("13-library", ""),
        (
            r#"# Exercise 13 · The list functions  ★★☆
#
# Most work on lists needs no recursion: the built-in functions do it.
# `1..5` is the list [1, 2, 3, 4, 5]; `map(list, f)` applies f to each
# element; `filter(list, f)` keeps the elements for which f gives true;
# `sum`, `len`, `sort`, `reverse`, `take`... `rythme help lists` shows them.
#
# Statement
#   - `squares(n)`: the list [1, 4, 9, ..., n × n];
#   - `evens(list)`: the even numbers of the list, in the same order;
#   - `sum_of_multiples(limit)`: the sum of the numbers from 1 to limit - 1
#     that are multiples of 3 or of 5;
#   - `mean(list)`: the average of the numbers of a list that is not empty;
#   - `smallest_three(list)`: the three smallest numbers, from the smallest.
#
# Hints
#   1. `map(1..n, square)` with a function `square` written by you, or
#      `map(1..n, fn(x) = x * x)` (an anonymous function, see exercise 14).
#   2. `is_even` is a built-in function: `filter(list, is_even)`.
#   3. `1..0` is the empty list `[]`.
#   4. `sort` then `take`.
#
# Check your work:  rythme exercise check 13-library.ry
#              or:  rythme test 13-library.ry

fn squares(n) = fail("to do")

fn evens(list) = fail("to do")

fn sum_of_multiples(limit) = fail("to do")

fn mean(list) = fail("to do")

fn smallest_three(list) = fail("to do")

# --- Tests (do not change them) ---

test "squares"
  assert_eq(squares(5), [1, 4, 9, 16, 25])
  assert_eq(squares(0), [])
end

test "evens" = assert_eq(evens([5, 2, 8, 1, 4]), [2, 8, 4])

test "sum of multiples"
  assert_eq(sum_of_multiples(10), 23)
  assert_eq(sum_of_multiples(1000), 233168)
end

test "mean"
  assert_eq(mean([1, 2, 3, 4]), 2.5)
  assert_eq(mean([7]), 7)
end

test "smallest three"
  assert_eq(smallest_three([5, 1, 4, 2, 3]), [1, 2, 3])
  assert_eq(smallest_three([9, 8]), [8, 9])
end
"#,
            "",
        ),
        ("14-anonymous-functions", ""),
        (
            r#"# Exercise 14 · Anonymous functions  ★★☆
#
# `fn(x) = x * 2` is a function without a name, written right where it is
# needed. It can use the names around it:
#   fn add_to_all(list, n) = map(list, fn(x) = x + n)
#
# Statement
#   - `with_tax(prices, percent)`: each price increased by `percent` %;
#   - `long_words(words, n)`: the words that have more than n characters;
#   - `lengths(words)`: the length of each word;
#   - `all_positive(list)`: are all the numbers greater than 0?
#   - `count_above(list, limit)`: how many numbers are greater than limit.
#
# Hints
#   1. A price p increased by 20 % is `p + p * 20 / 100`.
#   2. `len("word")` is 4.
#   3. `all(list, f)` is true when f gives true for every element, and
#      `count(list, f)` counts the elements for which f gives true.
#
# Check your work:  rythme exercise check 14-anonymous-functions.ry
#              or:  rythme test 14-anonymous-functions.ry

fn with_tax(prices, percent) = fail("to do")

fn long_words(words, n) = fail("to do")

fn lengths(words) = fail("to do")

fn all_positive(list) = fail("to do")

fn count_above(list, limit) = fail("to do")

# --- Tests (do not change them) ---

test "with tax"
  assert_eq(with_tax([100, 50], 20), [120, 60])
  assert_eq(with_tax([10], 0), [10])
end

test "long words" = assert_eq(long_words(["a", "tree", "is", "green"], 3), ["tree", "green"])

test "lengths" = assert_eq(lengths(["a", "tree", "is"]), [1, 4, 2])

test "all positive"
  assert_eq(all_positive([1, 5, 3]), true)
  assert_eq(all_positive([1, -5, 3]), false)
  assert_eq(all_positive([]), true)
end

test "count above" = assert_eq(count_above([3, 10, 7, 12], 5), 3)
"#,
            "",
        ),
        ("15-functions-as-values", ""),
        (
            r#"# Exercise 15 · Functions are values  ★★☆
#
# A function can be given to another function, and a function can give
# back a new function:
#   fn twice(f) = fn(x) = f(f(x))
#   add_two = twice(fn(x) = x + 1)      # add_two(5) is 7
#
# Statement
#   - `apply_twice(f, x)`: f applied to x, then f applied to the result;
#   - `compose(f, g)`: a new function that applies g, then f;
#   - `adder(n)`: a new function that adds n to its argument;
#   - `my_map(list, f)` and `my_filter(list, f)`: your own versions of
#     `map` and `filter`, with recursion.
#
# Hints
#   1. `compose(f, g)` gives `fn(x) = ...`.
#   2. `my_map([x, ..rest], f)` starts with `f(x)`.
#   3. `my_filter([x, ..rest], f)`: keep x only if `f(x)` is true, then
#      continue with rest.
#
# Check your work:  rythme exercise check 15-functions-as-values.ry
#              or:  rythme test 15-functions-as-values.ry

fn apply_twice(f, x) = fail("to do")

fn compose(f, g) = fail("to do")

fn adder(n) = fail("to do")

fn my_map(list, f) = fail("to do")

fn my_filter(list, f) = fail("to do")

# --- Tests (do not change them) ---

test "apply twice"
  assert_eq(apply_twice(fn(x) = x * 3, 2), 18)
  assert_eq(apply_twice(upper, "hi"), "HI")
end

test "compose"
  plus_one_then_double = compose(fn(x) = x * 2, fn(x) = x + 1)
  assert_eq(plus_one_then_double(4), 10)
  assert_eq(compose(len, trim)("  abc  "), 3)
end

test "adder"
  add10 = adder(10)
  assert_eq(add10(5), 15)
  assert_eq(map([1, 2, 3], adder(100)), [101, 102, 103])
end

test "my map"
  assert_eq(my_map([1, 2, 3], fn(x) = x * x), [1, 4, 9])
  assert_eq(my_map([], fn(x) = x), [])
end

test "my filter"
  assert_eq(my_filter([1, 2, 3, 4, 5, 6], is_even), [2, 4, 6])
  assert_eq(my_filter(["a", "bb", "ccc"], fn(w) = len(w) > 1), ["bb", "ccc"])
end
"#,
            "",
        ),
        ("16-fold", ""),
        (
            r#"# Exercise 16 · Folding a list into one value  ★★☆
#
# `fold(list, start, f)` combines all the elements into one value: it
# starts with `start`, then computes `f(result, element)` for each element.
#   fold([1, 2, 3], 0, fn(total, x) = total + x)     # 6
#
# Statement
#   With `fold`, and without recursion:
#   - `product_of(list)`: the product of the numbers (1 for an empty list);
#   - `longest_word(words)`: the longest word (the first one if several
#     have the same length; "" for an empty list);
#   - `to_number(digits)`: the number written with these digits
#     ([1, 2, 3] gives 123);
#   - `count_where(list, f)`: how many elements give true with f (without
#     `count` or `filter`).
#
# Hints
#   1. Choose `start` well: it is the answer for the empty list.
#   2. For `longest_word`, the result so far is the longest word seen so
#      far: keep it, or take the new word if it is strictly longer.
#   3. `to_number`: with 12 so far and the digit 3, the new result is
#      12 * 10 + 3.
#
# Check your work:  rythme exercise check 16-fold.ry
#              or:  rythme test 16-fold.ry

fn product_of(list) = fail("to do")

fn longest_word(words) = fail("to do")

fn to_number(digits) = fail("to do")

fn count_where(list, f) = fail("to do")

# --- Tests (do not change them) ---

test "product"
  assert_eq(product_of([2, 3, 4]), 24)
  assert_eq(product_of([]), 1)
end

test "longest word"
  assert_eq(longest_word(["a", "tree", "is", "green"]), "green")
  assert_eq(longest_word(["one", "two"]), "one")
  assert_eq(longest_word([]), "")
end

test "to number"
  assert_eq(to_number([1, 2, 3]), 123)
  assert_eq(to_number([0, 7]), 7)
  assert_eq(to_number([]), 0)
end

test "count where"
  assert_eq(count_where([1, 2, 3, 4, 5], is_odd), 3)
  assert_eq(count_where(["a", "bb"], fn(w) = len(w) > 5), 0)
end
"#,
            "",
        ),
        ("17-pipelines", ""),
        (
            r#"# Exercise 17 · Pipelines  ★★☆
#
# `x |> f(a)` means `f(x, a)`: the value on the left becomes the first
# argument. Steps are read from left to right, in the order they happen:
#   [3, 1, 2] |> sort |> map(fn(x) = x * 10)        # [10, 20, 30]
# A line that starts with `|>` continues the line before.
#
# Statement
#   Write each function as one pipeline:
#   - `sum_of_odd_squares(n)`: the sum of the squares of the odd numbers
#     from 1 to n;
#   - `top_three(scores)`: the three best scores, the best first;
#   - `shout_words(sentence)`: each word in uppercase, separated by one
#     space ("hello  world" gives "HELLO WORLD");
#   - `initials_of(sentence)`: the first letter of each word, in uppercase
#     ("portable network graphics" gives "PNG").
#
# Hints
#   1. `filter`, then `map`, then `sum`.
#   2. `sort`, `reverse`, `take(3)`.
#   3. `split(text)` cuts a text at the spaces (several spaces count as
#      one); `join(words, " ")` glues words back together.
#   4. `word[0]` is the first character of `word`.
#
# Check your work:  rythme exercise check 17-pipelines.ry
#              or:  rythme test 17-pipelines.ry

fn sum_of_odd_squares(n) = fail("to do")

fn top_three(scores) = fail("to do")

fn shout_words(sentence) = fail("to do")

fn initials_of(sentence) = fail("to do")

# --- Tests (do not change them) ---

test "sum of odd squares"
  assert_eq(sum_of_odd_squares(5), 35)
  assert_eq(sum_of_odd_squares(1), 1)
end

test "top three"
  assert_eq(top_three([12, 18, 9, 15, 20]), [20, 18, 15])
  assert_eq(top_three([4]), [4])
end

test "shout words" = assert_eq(shout_words("hello  world"), "HELLO WORLD")

test "initials of"
  assert_eq(initials_of("portable network graphics"), "PNG")
  assert_eq(initials_of("Rythme"), "R")
end
"#,
            "",
        ),
        ("18-text-processing", ""),
        (
            r#"# Exercise 18 · Working with texts  ★★☆
#
# A text can be turned into the list of its characters with `chars`, and
# the list functions do the rest. `lower`, `upper`, `replace`, `split`,
# `join`, `contains`... are described by `rythme help text`.
#
# Statement
#   - `is_palindrome(text)`: does the text read the same from both ends,
#     ignoring uppercase and spaces? ("Never odd or even" does.)
#   - `count_vowels(text)`: how many characters are vowels (a, e, i, o, u
#     or y, in lowercase or uppercase)?
#   - `title_case(text)`: each word starts with an uppercase letter, the
#     rest in lowercase ("the LORD of the rings" gives
#     "The Lord Of The Rings").
#
# Hints
#   1. `replace(text, " ", "")` removes the spaces, and `reverse` works on
#      texts too.
#   2. `contains("aeiouy", c)` tells whether the character c is a vowel.
#   3. `capitalize("lord")` is "Lord".
#
# Check your work:  rythme exercise check 18-text-processing.ry
#              or:  rythme test 18-text-processing.ry

fn is_palindrome(text) = fail("to do")

fn count_vowels(text) = fail("to do")

fn title_case(text) = fail("to do")

# --- Tests (do not change them) ---

test "palindromes"
  assert_eq(is_palindrome("kayak"), true)
  assert_eq(is_palindrome("Never odd or even"), true)
  assert_eq(is_palindrome("rythme"), false)
end

test "vowels"
  assert_eq(count_vowels("Rythme"), 2)
  assert_eq(count_vowels("AEIOU"), 5)
  assert_eq(count_vowels("xkcd"), 0)
end

test "title case" = assert_eq(title_case("the LORD of the rings"), "The Lord Of The Rings")
"#,
            "",
        ),
        ("19-sorting", ""),
        (
            r#"# Exercise 19 · Sorting  ★★★
#
# Two classic ways to sort a list, written with recursion (without the
# built-in `sort`).
#
# Statement
#   - `insert(sorted, x)`: x placed at its position in a list that is
#     already sorted;
#   - `insertion_sort(list)`: sorts the list by inserting each element, one
#     after the other, into a sorted list that starts empty;
#   - `merge(a, b)`: one sorted list from two sorted lists;
#   - `merge_sort(list)`: cuts the list in two halves, sorts each half,
#     and merges them.
#
# Hints
#   1. `insert([], x)` is `[x]`. For `insert([first, ..rest], x)`: if
#      x <= first, x goes in front; otherwise first stays first.
#   2. `insertion_sort([x, ..rest])` is `insert(insertion_sort(rest), x)`
#      (or use `fold` with `insert`).
#   3. `merge` compares the first elements of the two lists: the smaller one
#      comes first, followed by the merge of what remains.
#   4. `take(list, n)` and `drop(list, n)` give the two halves, with
#      `n = len(list) // 2`. A list of 0 or 1 element is already sorted.
#
# Check your work:  rythme exercise check 19-sorting.ry
#              or:  rythme test 19-sorting.ry

fn insert(sorted, x) = fail("to do")

fn insertion_sort(list) = fail("to do")

fn merge(a, b) = fail("to do")

fn merge_sort(list) = fail("to do")

# --- Tests (do not change them) ---

test "insert"
  assert_eq(insert([1, 3, 5], 4), [1, 3, 4, 5])
  assert_eq(insert([], 2), [2])
  assert_eq(insert([1, 2], 9), [1, 2, 9])
end

test "insertion sort"
  assert_eq(insertion_sort([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(insertion_sort([]), [])
end

test "merge"
  assert_eq(merge([1, 4, 6], [2, 3, 7]), [1, 2, 3, 4, 6, 7])
  assert_eq(merge([], [1]), [1])
  assert_eq(merge([1], []), [1])
end

test "merge sort"
  assert_eq(merge_sort([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(merge_sort(["pear", "apple", "fig"]), ["apple", "fig", "pear"])
  assert_eq(merge_sort(reverse(1..100)), 1..100)
end
"#,
            "",
        ),
        ("20-records", ""),
        (
            r#"# Exercise 20 · Records  ★★☆
#
# A record groups named values: `{name: "Ada", born: 1815}`. `ada.name`
# reads a field. A record never changes: `{..ada, born: 1816}` is a new
# record, a copy of ada with another `born`. A parameter can take a record
# apart: `fn hello({name}) = "Hello, {name}"`.
#
# Statement
#   A person is a record `{name, born}`; a point is a record `{x, y}`.
#   - `age_in(person, year)`: the age of the person at the end of the year;
#   - `label(person)`: a text like "Ada (1815)";
#   - `renamed(person, name)`: the same person with another name;
#   - `moved(point, dx, dy)`: the point moved by dx horizontally and dy
#     vertically;
#   - `distance(a, b)`: the distance between two points.
#
# Hints
#   1. `person.born`, or a parameter `{born}`.
#   2. `{..person, name: name}`, which can be written `{..person, name}`.
#   3. The distance is the square root of (xb - xa)² + (yb - ya)².
#
# Check your work:  rythme exercise check 20-records.ry
#              or:  rythme test 20-records.ry

fn age_in(person, year) = fail("to do")

fn label(person) = fail("to do")

fn renamed(person, name) = fail("to do")

fn moved(point, dx, dy) = fail("to do")

fn distance(a, b) = fail("to do")

# --- Tests (do not change them) ---

ada = {name: "Ada", born: 1815}

test "age" = assert_eq(age_in(ada, 1852), 37)

test "label" = assert_eq(label(ada), "Ada (1815)")

test "renamed"
  assert_eq(renamed(ada, "Augusta"), {name: "Augusta", born: 1815})
  assert_eq(ada.name, "Ada")
end

test "moved"
  assert_eq(moved({x: 1, y: 2}, 3, -1), {x: 4, y: 1})
  assert_eq(moved({x: 0, y: 0}, 0, 0), {x: 0, y: 0})
end

test "distance"
  assert_eq(distance({x: 0, y: 0}, {x: 3, y: 4}), 5)
  assert_eq(distance({x: 1, y: 1}, {x: 1, y: 1}), 0)
end
"#,
            "",
        ),
        ("21-lists-of-records", ""),
        (
            r#"# Exercise 21 · Lists of records  ★★☆
#
# Real data is often a list of records: a class of students, a list of
# orders... `map`, `filter`, `sort_by`, `max_by`, `group_by` and `sum` work
# on them like on any list.
#
# Statement
#   A student is a record `{name, score, group}` (a score from 0 to 100).
#   - `names_of_passed(students)`: the names of the students whose score is
#     at least 50, in the same order;
#   - `average_score(students)`: the average of the scores;
#   - `best(students)`: the name of the student with the best score;
#   - `ranking(students)`: the names, from the best score to the lowest;
#   - `group_sizes(students)`: a record that gives, for each group, how
#     many students are in it (`{A: 2, B: 1}`).
#
# Hints
#   1. `filter` then `map(fn(s) = s.name)`.
#   2. `max_by(list, key)` gives the element whose key is the largest.
#   3. `sort_by(list, fn(s) = -s.score)` sorts from the best score.
#   4. `group_by(list, key)` gives a record of lists, and
#      `map_values(record, f)` applies f to each of its values.
#
# Check your work:  rythme exercise check 21-lists-of-records.ry
#              or:  rythme test 21-lists-of-records.ry

fn names_of_passed(students) = fail("to do")

fn average_score(students) = fail("to do")

fn best(students) = fail("to do")

fn ranking(students) = fail("to do")

fn group_sizes(students) = fail("to do")

# --- Tests (do not change them) ---

classroom = [
  {name: "Ada", score: 92, group: "A"},
  {name: "Alan", score: 45, group: "B"},
  {name: "Grace", score: 78, group: "A"},
  {name: "Edsger", score: 50, group: "B"},
  {name: "Barbara", score: 85, group: "A"},
]

test "passed" = assert_eq(names_of_passed(classroom), ["Ada", "Grace", "Edsger", "Barbara"])

test "average" = assert_eq(average_score(classroom), 70)

test "best" = assert_eq(best(classroom), "Ada")

test "ranking" = assert_eq(ranking(classroom), ["Ada", "Barbara", "Grace", "Edsger", "Alan"])

test "group sizes" = assert_eq(group_sizes(classroom), {A: 3, B: 2})
"#,
            "",
        ),
        ("22-own-types", ""),
        (
            r#"# Exercise 22 · Your own types  ★★☆
#
# `type` defines a new kind of value, with one constructor for each
# possible form:
#   type Pet = Dog(name) | Cat(name, lives) | Fish
# A function can have one definition per constructor:
#   fn sound(Dog(_)) = "woof"
#
# Statement
#   The type `Shape` below has three forms.
#   - `area(shape)`: the area of the shape;
#   - `perimeter(shape)`: the length of its border;
#   - `name_of(shape)`: "circle", "rectangle" or "square";
#   - `total_area(shapes)`: the sum of the areas of a list of shapes.
#
# Hints
#   1. `fn area(Circle(r)) = pi * r * r`, then one definition for each
#      other constructor.
#   2. The perimeter of a circle is 2 × π × r.
#   3. If you forget a constructor, `rythme check` tells you which one
#      (warning W002).
#
# Check your work:  rythme exercise check 22-own-types.ry
#              or:  rythme test 22-own-types.ry

type Shape = Circle(radius) | Rect(width, height) | Square(side)

fn area(shape) = fail("to do")

fn perimeter(shape) = fail("to do")

fn name_of(shape) = fail("to do")

fn total_area(shapes) = fail("to do")

# --- Tests (do not change them) ---

test "area"
  assert_eq(area(Rect(2, 3)), 6)
  assert_eq(area(Square(4)), 16)
  assert_eq(round(area(Circle(1)), 2), 3.14)
end

test "perimeter"
  assert_eq(perimeter(Rect(2, 3)), 10)
  assert_eq(perimeter(Square(4)), 16)
  assert_eq(round(perimeter(Circle(1)), 2), 6.28)
end

test "name" = assert_eq([Circle(1), Rect(1, 2), Square(3)] |> map(name_of), ["circle", "rectangle", "square"])

test "total area" = assert_eq(total_area([Rect(2, 3), Square(2), Rect(1, 1)]), 11)
"#,
            "",
        ),
        ("23-match", ""),
        (
            r#"# Exercise 23 · Choosing with `match`  ★★☆
#
# `match` compares a value with patterns, in order, and gives the result of
# the first one that fits:
#   fn describe(n) = match n
#     0 -> "zero"
#     n if n < 0 -> "negative"
#     _ -> "positive"
#   end
# Patterns can be numbers, texts, constructors, lists (`[]`, `[x]`,
# `[x, ..rest]`) or records (`{name}`).
#
# Statement
#   - `next_light(light)`: the light that comes after: Green, then Orange,
#     then Red, then Green again;
#   - `can_go(light)`: true only for Green;
#   - `describe(list)`: "empty" for [], "one: 7" for [7], "two: 1 and 2"
#     for [1, 2], and "many, starting with 5" for longer lists;
#   - `greeting(person)`: for a record with a `title` field,
#     "Hello, Dr Hopper"; otherwise, with only `name`, "Hello, Grace".
#
# Hints
#   1. Each of the three lights is one case of the `match`.
#   2. `[a, b]` matches lists of exactly two elements, `[first, ..]` lists
#      of at least one element: put the most precise patterns first.
#   3. `{title, last}` matches records that have at least these fields.
#
# Check your work:  rythme exercise check 23-match.ry
#              or:  rythme test 23-match.ry

type Light = Green | Orange | Red

fn next_light(light) = fail("to do")

fn can_go(light) = fail("to do")

fn describe(list) = fail("to do")

fn greeting(person) = fail("to do")

# --- Tests (do not change them) ---

test "next light"
  assert_eq(next_light(Green), Orange)
  assert_eq(next_light(Orange), Red)
  assert_eq(next_light(Red), Green)
end

test "can go"
  assert_eq(can_go(Green), true)
  assert_eq(can_go(Orange), false)
  assert_eq(can_go(Red), false)
end

test "describe"
  assert_eq(describe([]), "empty")
  assert_eq(describe([7]), "one: 7")
  assert_eq(describe([1, 2]), "two: 1 and 2")
  assert_eq(describe([5, 6, 7]), "many, starting with 5")
end

test "greeting"
  assert_eq(greeting({title: "Dr", last: "Hopper", name: "Grace"}), "Hello, Dr Hopper")
  assert_eq(greeting({name: "Grace"}), "Hello, Grace")
end
"#,
            "",
        ),
        ("24-trees", ""),
        (
            r#"# Exercise 24 · Trees  ★★★
#
# A type can contain itself. A binary tree is either empty (`Leaf`), or a
# node with a value and two smaller trees, on its left and on its right.
# Functions on trees follow the shape of the type: one case for `Leaf`, one
# for `Node`, which calls itself on both sides.
#
# Statement
#   - `size(tree)`: how many values the tree holds;
#   - `total(tree)`: the sum of its values;
#   - `depth(tree)`: the number of levels (0 for a Leaf);
#   - `insert(tree, x)`: a search tree is a tree where the values on the
#     left of a node are smaller than its value, and the others on the
#     right; give the search tree with x added (a new Node in place of a
#     Leaf);
#   - `to_list(tree)`: the values, from left to right.
#
# Hints
#   1. `fn size(Leaf) = 0` and `fn size(Node(left, _, right)) = ...`.
#   2. `max(a, b)` gives the larger of two numbers.
#   3. `insert` gives a new tree: `Node(insert(left, x), value, right)`
#      when x goes to the left.
#   4. `to_list` of a node is: the list of the left, then the value, then
#      the list of the right (`+` joins lists).
#
# Check your work:  rythme exercise check 24-trees.ry
#              or:  rythme test 24-trees.ry

type Tree = Leaf | Node(left, value, right)

fn size(tree) = fail("to do")

fn total(tree) = fail("to do")

fn depth(tree) = fail("to do")

fn insert(tree, x) = fail("to do")

fn to_list(tree) = fail("to do")

# --- Tests (do not change them) ---

#       5
#      / \
#     3   8
#    /
#   1
small = Node(Node(Node(Leaf, 1, Leaf), 3, Leaf), 5, Node(Leaf, 8, Leaf))

test "size"
  assert_eq(size(Leaf), 0)
  assert_eq(size(small), 4)
end

test "total" = assert_eq(total(small), 17)

test "depth"
  assert_eq(depth(Leaf), 0)
  assert_eq(depth(small), 3)
end

test "insert"
  assert_eq(insert(Leaf, 4), Node(Leaf, 4, Leaf))
  assert_eq(insert(Node(Leaf, 4, Leaf), 2), Node(Node(Leaf, 2, Leaf), 4, Leaf))
end

test "to list" = assert_eq(to_list(small), [1, 3, 5, 8])

test "sorting with a tree" = assert_eq([7, 2, 9, 4, 1] |> fold(Leaf, insert) |> to_list, [1, 2, 4, 7, 9])
"#,
            "",
        ),
        ("25-expressions", ""),
        (
            r#"# Exercise 25 · A small calculator  ★★★
#
# A calculation like (1 + 2) × 3 is a tree: the multiplication has two
# parts, the addition and 3. With a type, a program can hold calculations,
# compute them and show them.
#
# Statement
#   - `evaluate(expr)`: the value of the calculation;
#   - `show(expr)`: the calculation as a text, with parentheses around every
#     addition and multiplication: "((1 + 2) * 3)", and a negation written
#     "-x" ("-4", "-(1 + 2)");
#   - `count_numbers(expr)`: how many numbers the calculation contains;
#   - `simplify(expr)`: the same calculation, where `Add(e, Num(0))`,
#     `Mul(e, Num(1))` (and the same with the number on the left) are
#     replaced by e, at every level.
#
# Hints
#   1. One case per constructor: `fn evaluate(Num(n)) = n`,
#      `fn evaluate(Add(a, b)) = evaluate(a) + evaluate(b)`...
#   2. `show(Neg(e))` is "-" followed by `show(e)`.
#   3. In `simplify`, first simplify both parts, then look at the result
#      with a `match`.
#
# Check your work:  rythme exercise check 25-expressions.ry
#              or:  rythme test 25-expressions.ry

type Expr = Num(value) | Add(left, right) | Mul(left, right) | Neg(inner)

fn evaluate(expr) = fail("to do")

fn show(expr) = fail("to do")

fn count_numbers(expr) = fail("to do")

fn simplify(expr) = fail("to do")

# --- Tests (do not change them) ---

calculation = Mul(Add(Num(1), Num(2)), Num(3))

test "evaluate"
  assert_eq(evaluate(Num(4)), 4)
  assert_eq(evaluate(calculation), 9)
  assert_eq(evaluate(Neg(calculation)), -9)
end

test "show"
  assert_eq(show(calculation), "((1 + 2) * 3)")
  assert_eq(show(Neg(Num(4))), "-4")
  assert_eq(show(Neg(Add(Num(1), Num(2)))), "-(1 + 2)")
end

test "count numbers" = assert_eq(count_numbers(Add(calculation, Neg(Num(5)))), 4)

test "simplify"
  assert_eq(simplify(Add(Num(7), Num(0))), Num(7))
  assert_eq(simplify(Mul(Num(1), Add(Num(0), Num(2)))), Num(2))
  assert_eq(simplify(calculation), calculation)
  assert_eq(simplify(Neg(Mul(Num(3), Num(1)))), Neg(Num(3)))
end
"#,
            "",
        ),
        ("26-option", ""),
        (
            r#"# Exercise 26 · A value that may be missing: Option  ★★☆
#
# Rythme has no `null`. A function that may find nothing returns an Option:
# `Some(value)` when there is a value, `None` otherwise. `match` handles
# both cases, and `unwrap_or(option, default)` gives the value or a default.
#   fn first_of([]) = None
#   fn first_of([x, ..]) = Some(x)
#
# Statement
#   - `safe_divide(a, b)`: `Some(a / b)`, or `None` when b is 0;
#   - `find_age(people, name)`: the age of the person with this name, as an
#     Option (people is a list of records `{name, age}`);
#   - `age_or_unknown(people, name)`: a text, "36" for a known person and
#     "unknown" otherwise;
#   - `first_long_word(words)`: the first word with more than 5
#     characters, as an Option;
#   - `add_options(a, b)`: `Some(x + y)` when a is `Some(x)` and b is
#     `Some(y)`, `None` otherwise.
#
# Hints
#   1. `find(list, f)` gives the first element for which f is true, as an
#      Option; `map(option, f)` applies f to the value inside, if any.
#   2. `match find_age(people, name)` with the cases `Some(age)` and `None`.
#   3. `match [a, b]` with a case `[Some(x), Some(y)]` and a case `_`.
#
# Check your work:  rythme exercise check 26-option.ry
#              or:  rythme test 26-option.ry

fn safe_divide(a, b) = fail("to do")

fn find_age(people, name) = fail("to do")

fn age_or_unknown(people, name) = fail("to do")

fn first_long_word(words) = fail("to do")

fn add_options(a, b) = fail("to do")

# --- Tests (do not change them) ---

people = [{name: "Ada", age: 36}, {name: "Alan", age: 41}]

test "safe divide"
  assert_eq(safe_divide(10, 4), Some(2.5))
  assert_eq(safe_divide(1, 0), None)
end

test "find age"
  assert_eq(find_age(people, "Alan"), Some(41))
  assert_eq(find_age(people, "Grace"), None)
end

test "age or unknown"
  assert_eq(age_or_unknown(people, "Ada"), "36")
  assert_eq(age_or_unknown(people, "Grace"), "unknown")
end

test "first long word"
  assert_eq(first_long_word(["a", "functional", "language"]), Some("functional"))
  assert_eq(first_long_word(["short", "words"]), None)
end

test "add options"
  assert_eq(add_options(Some(2), Some(3)), Some(5))
  assert_eq(add_options(Some(2), None), None)
  assert_eq(add_options(None, None), None)
end
"#,
            "",
        ),
        ("27-result", ""),
        (
            r#"# Exercise 27 · An operation that may fail: Result  ★★☆
#
# Rythme has no exceptions. A function that may fail returns a Result:
# `Ok(value)` when it worked, `Err(message)` otherwise. The caller must
# look at both cases: no error can be forgotten.
#   parse_int("42")       # Ok(42)
#   parse_int("forty")    # Err("\"forty\" is not a whole number")
#
# Statement
#   - `checked_divide(a, b)`: `Ok(a / b)`, or `Err("division by zero")`;
#   - `parse_age(text)`: the age written in the text, as a Result:
#     `Err("not a number")` if the text is not a whole number,
#     `Err("an age cannot be negative")` below 0,
#     `Err("too old")` above 150, and `Ok(age)` otherwise;
#   - `describe(result)`: "ok: 42" for `Ok(42)` and "error: too old" for
#     `Err("too old")`;
#   - `check_code(code)`: `Ok(code)` if it has at least 8
#     characters and contains a digit; otherwise an Err with the first
#     problem found: "too short" or "no digit".
#
# Hints
#   1. `match parse_int(text)` with the cases `Err(_)`, `Ok(n) if n < 0`,
#      `Ok(n) if n > 150` and `Ok(n)`.
#   2. `any(chars(code), is_digit)` tells whether there is a digit.
#
# Check your work:  rythme exercise check 27-result.ry
#              or:  rythme test 27-result.ry

fn checked_divide(a, b) = fail("to do")

fn parse_age(text) = fail("to do")

fn describe(result) = fail("to do")

fn check_code(code) = fail("to do")

# --- Tests (do not change them) ---

test "checked divide"
  assert_eq(checked_divide(9, 3), Ok(3))
  assert_eq(checked_divide(1, 0), Err("division by zero"))
end

test "parse age"
  assert_eq(parse_age("36"), Ok(36))
  assert_eq(parse_age(" 7 "), Ok(7))
  assert_eq(parse_age("seven"), Err("not a number"))
  assert_eq(parse_age("-1"), Err("an age cannot be negative"))
  assert_eq(parse_age("200"), Err("too old"))
end

test "describe"
  assert_eq(describe(Ok(42)), "ok: 42")
  assert_eq(describe(Err("too old")), "error: too old")
  assert_eq(describe(parse_age("-5")), "error: an age cannot be negative")
end

test "check code"
  assert_eq(check_code("rythme2024"), Ok("rythme2024"))
  assert_eq(check_code("abc1"), Err("too short"))
  assert_eq(check_code("functional"), Err("no digit"))
end
"#,
            "",
        ),
        ("28-question-mark", ""),
        (
            r#"# Exercise 28 · Passing errors on with `?`  ★★★
#
# Writing a `match` after each step that may fail is long. `?` after a
# Result (or an Option) takes the value out of `Ok(...)` (or `Some(...)`),
# or stops the function at once and returns the `Err(...)` (or `None`):
#   fn double_text(t)
#     n = parse_int(t)?          # stops here if t is not a number
#     Ok(n * 2)
#   end
#
# Statement
#   - `add_texts(a, b)`: the sum of the two numbers written in a and b,
#     as `Ok(sum)`, or the error of `parse_int`;
#   - `parse_point(text)`: for "3,4", `Ok({x: 3, y: 4})`; the error
#     `Err("expected x,y")` if the text does not have exactly two parts
#     separated by a comma, or the error of `parse_int` for a part that is
#     not a number;
#   - `sum_of_texts(texts)`: the sum of the numbers written in a list of
#     texts, or the first error;
#   - `ratio_of_texts(a, b)`: a / b for two numbers written in texts, or the
#     first error, with `Err("division by zero")` when b is 0.
#
# Hints
#   1. `Ok(parse_int(a)? + parse_int(b)?)`
#   2. `match split(text, ",")` with the cases `[x, y]` and `_`.
#   3. For a list, use recursion: `sum_of_texts([])` is `Ok(0)`; for
#      `[t, ..rest]`, take the number of t and the sum of rest, both with
#      `?`.
#   4. `?` can be used on any function that gives a Result, your own ones
#      too.
#
# Check your work:  rythme exercise check 28-question-mark.ry
#              or:  rythme test 28-question-mark.ry

fn add_texts(a, b) = fail("to do")

fn parse_point(text) = fail("to do")

fn sum_of_texts(texts) = fail("to do")

fn ratio_of_texts(a, b) = fail("to do")

# --- Tests (do not change them) ---

test "add texts"
  assert_eq(add_texts("2", "40"), Ok(42))
  assert(is_err(add_texts("2", "forty")), "forty is not a number")
end

test "parse point"
  assert_eq(parse_point("3,4"), Ok({x: 3, y: 4}))
  assert_eq(parse_point("3"), Err("expected x,y"))
  assert_eq(parse_point("1,2,3"), Err("expected x,y"))
  assert(is_err(parse_point("3,four")), "four is not a number")
end

test "sum of texts"
  assert_eq(sum_of_texts(["1", "2", "3"]), Ok(6))
  assert_eq(sum_of_texts([]), Ok(0))
  assert_eq(sum_of_texts(["1", "x", "y"]), parse_int("x"))
end

test "ratio of texts"
  assert_eq(ratio_of_texts("9", "2"), Ok(4.5))
  assert_eq(ratio_of_texts("9", "0"), Err("division by zero"))
  assert(is_err(ratio_of_texts("nine", "0")), "nine is not a number")
end
"#,
            "",
        ),
        ("29-word-counts", ""),
        (
            r#"# Exercise 29 · A small project: counting words  ★★★
#
# Everything together: texts, lists, records, pipelines and Options, for a
# program that analyses a text.
#
# Statement
#   - `words_of(text)`: the words of the text, in lowercase, without the
#     punctuation `.`, `,`, `!` and `?`;
#   - `frequencies(text)`: a list of `[word, count]` pairs, from the most
#     frequent word to the least frequent, and in alphabetical order for
#     words that appear the same number of times;
#   - `most_frequent(text)`: the most frequent word, as an Option (`None`
#     for a text without words);
#   - `report(text, n)`: the n most frequent words, one per line, each as
#     "word: count".
#
# Hints
#   1. `replace` four times (or `fold` over the list [".", ",", "!", "?"]),
#      then `lower` and `split`.
#   2. `group_by(words, fn(w) = w)` gives a record `{word: [word, word]}`,
#      and `entries(record)` the list of its `[name, value]` pairs.
#   3. `sort_by` is stable: sort by word first, then by count (the largest
#      first, with `fn([_, n]) = -n`).
#   4. `first(list)` gives an Option, and `map(option, f)` changes the value
#      inside.
#   5. `join(lines, "\n")` puts each element on its own line.
#
# Check your work:  rythme exercise check 29-word-counts.ry
#              or:  rythme test 29-word-counts.ry

fn words_of(text) = fail("to do")

fn frequencies(text) = fail("to do")

fn most_frequent(text) = fail("to do")

fn report(text, n) = fail("to do")

# --- Tests (do not change them) ---

poem = "The cat sat. The dog sat! Did the cat see the dog?"

test "words" = assert_eq(words_of("Hello, World! Hello?"), ["hello", "world", "hello"])

test "frequencies"
  assert_eq(frequencies("b a b c a b"), [["b", 3], ["a", 2], ["c", 1]])
  assert_eq(frequencies(poem) |> take(3), [["the", 4], ["cat", 2], ["dog", 2]])
end

test "most frequent"
  assert_eq(most_frequent(poem), Some("the"))
  assert_eq(most_frequent(" ... "), None)
end

test "report" = assert_eq(report(poem, 3), "the: 4\ncat: 2\ndog: 2")
"#,
            "",
        ),
        ("30-testing", ""),
        (
            r#"# Exercise 30 · Tests find bugs  ★★★
#
# A test checks one piece of code: `test "name" = assert_eq(actual,
# expected)`. When it fails, `rythme test` shows the two values: the bug is
# somewhere between them.
#
# Statement
#   The functions below are already written, but each one has a bug.
#   1. Run the tests and read what fails.
#   2. Fix the functions (not the tests!) until every test passes.
#   3. Then write a test of your own for `clamp`, at the end of the file:
#      what should `clamp(5, 5, 5)` give?
#
#   - `clamp(x, low, high)`: x brought back between low and high;
#   - `median(list)`: the middle value of the sorted list, or the average of
#     the two middle values when the list has an even length;
#   - `count_words(text)`: how many words the text has.
#
# Hints
#   1. Read the `got:` and `expected:` lines of the first failure, and
#      compute by hand what the function does with these arguments.
#   2. `debug(value)` shows a value while the program runs, and gives it
#      back unchanged: `debug(sorted)`.
#   3. What does `split` without a separator do with several spaces?
#
# Check your work:  rythme exercise check 30-testing.ry
#              or:  rythme test 30-testing.ry

fn clamp(x, low, high) = max(high, min(x, low))

fn median(list)
  middle = len(list) // 2
  if is_even(len(list)) then (list[middle - 1] + list[middle]) / 2 else list[middle]
end

fn count_words(text) = len(split(text, " "))

# --- Tests (do not change them) ---

test "clamp"
  assert_eq(clamp(15, 0, 10), 10)
  assert_eq(clamp(-5, 0, 10), 0)
  assert_eq(clamp(5, 0, 10), 5)
end

test "median of an odd number of values"
  assert_eq(median([1, 2, 3]), 2)
  assert_eq(median([3, 1, 2]), 2)
end

test "median of an even number of values" = assert_eq(median([4, 1, 3, 2]), 2.5)

test "count words"
  assert_eq(count_words("one two three"), 3)
  assert_eq(count_words("  one   two "), 2)
end

# Your test:
"#,
            "",
        ),
        // ----- CSV and seeded random (src/builtins/csv.rs, src/builtins.rs, src/main.rs, src/help.rs)
        // src/builtins/csv.rs
        ("line {line}, column {column}: the quote that starts this field is never closed (a quote inside a field is written twice: \"\")", ""),
        ("line {line}, column {column}: a quote in the middle of a field: put the whole field between quotes, and write each quote inside it twice (\"5\"\" screen\")", ""),
        ("line {line}, column {column}: after the quote that closes a field comes {sep} or the end of the line, not {c}", ""),
        ("line {line}: the header names the column \"{name}\" twice", ""),
        ("line {line}: {got} field(s), but the header has {wanted} column(s)", ""),
        ("line {line}: this table has too many rows or fields (more than {MAX_LIST})", ""),
        ("a tab", ""),
        ("a space", ""),
        ("`{f}` cannot use {c} as separator: it has another meaning in CSV", ""),
        ("the separator of `{f}` is one character, not \"{s}\"", ""),
        ("the usual separators: \",\" (by default), \";\" (French spreadsheets), \"\\t\" (a tab)", ""),
        ("`csv_format`: the field {column} of row {row} must be a text, a number or a Bool, but it is {got}", ""),
        ("turn it into a text first, with `text(x)` or `unwrap_or(x, \"\")`", ""),
        ("`csv_format`: record {number} has the fields {{{these}}}, but the first one has {{{wanted}}}: every record must have the same fields", ""),
        ("a record, like the first row", ""),
        ("a list of fields", ""),
        ("`csv_format`: row {number} must be {wanted}, but it is {got}", ""),
        ("`csv_format` takes a list of rows: lists of fields ([[\"name\", \"age\"], [\"Ada\", 36]]) or records ([{{name: \"Ada\", age: 36}}])", ""),
        // src/main.rs
        ("the seed must be a whole number, like `rythme run --seed 42 game.ry`, not `{v}`", ""),
        ("`--seed` needs a whole number, like `rythme run --seed 42 game.ry`", ""),
        // src/builtins.rs
        ("Makes the draws of `random` and `shuffle` that follow the same on every run: the same seed (a whole number) gives the same draws, with the same version of Rythme. For tests and classes; `rythme run --seed N file.ry` does the same from the command line.", ""),
        ("Reads a CSV table (a spreadsheet saved as text): `Ok(list of rows)`, each row a list of texts, or `Err(message)` saying at which line and column the text is not valid CSV. A field between quotes may hold separators, line breaks and quotes (written twice). The separator is \",\" unless given: \";\" for French spreadsheets, \"\\t\" for tabs. See `rythme help csv`.", ""),
        ("Reads a CSV table whose first line names the columns: `Ok(list of records)`, one record for each other line, with one field for each column, in their order, holding a text. `Err(message)` when the text is not valid CSV, when a line does not have as many fields as the header, or when two columns have the same name. To get numbers, use `parse_int` or `parse_float` on the fields.", ""),
        ("Writes rows as a CSV text, each one on its line, ended by a line break. The rows are lists of fields, or records: then a first line names their fields. A field is a text, a number or a Bool, put between quotes only when it needs them (a separator, a quote or a line break inside). The separator is \",\" unless given.", ""),
        // src/help.rs
        (
            r#"CSV: THE TABLES OF SPREADSHEETS

A CSV text is a table: one row per line, its fields separated by commas.
Every spreadsheet (LibreOffice, Excel, Google Sheets...) opens and saves it.

  csv_parse("name,age\nAda,36")              # Ok([["name", "age"], ["Ada", "36"]])
  csv_records("name,age\nAda,36")            # Ok([{name: "Ada", age: "36"}])
  csv_format([["name", "age"], ["Ada", 36]]) # "name,age\nAda,36\n"
  csv_format([{name: "Ada", age: 36}])       # the same text: records give a header

With files
  match read_file("grades.csv").and_then(csv_records)
    Ok(rows) -> print(len(rows), "students")
    Err(e) -> print("Error: {e}")
  end
  write_file("copy.csv", csv_format(rows))

The rules (RFC 4180)
  A field between quotes may hold commas, line breaks and quotes, written
  twice: "say ""hi""" is the field  say "hi"  ; csv_format adds the quotes
  only where they are needed.
  Lines end with \n or \r\n (Windows); the last one may have no line break.
  Empty lines are skipped; spaces are kept as they are in the file.
  Every field is a text: parse_int(r.age) or parse_float(r.price) give numbers.
  csv_records: the first line names the columns, which become the fields of
  the records (r.name; get(r, "first name") for a name with spaces); every
  line must have as many fields as the header.

Another separator: the last argument
  ";" (spreadsheets in French, where the comma is the decimal point), "\t"
  (tabs), "|"...
  csv_parse("a;b", ";")                      # Ok([["a", "b"]])
  csv_format(rows, ";")

A text that is not valid CSV gives Err, which says where (line and column):
  csv_parse("a,b\"c")   # Err("line 1, column 4: a quote in the middle of a field: ...")"#,
            "",
        ),
        (
            r#"RANDOM: CHANCE, AND THE SAME CHANCE AGAIN

  random()                   # a decimal number from 0 (included) to 1 (excluded)
  random(1, 6)               # a whole number from 1 to 6, both included
  random(["a", "b", "c"])    # one element of the list
  shuffle([1, 2, 3])         # the list in a random order

Every run draws other values. To get the same draws again (a test, a class
where everyone sees the same game, a mistake to find again), give a seed:

  random_seed(42)                  # in the program: the draws that follow
  rythme run --seed 42 game.ry     # from the command line: the whole run

The same seed gives the same draws, in the same order, with the same version
of Rythme (another version may draw other values). Without a seed, nothing
changes: every run is different.

In a test, call random_seed at its start: its draws then do not depend on
the tests before it.
  test "a dice roll stays between 1 and 6"
    random_seed(7)
    assert(all(map(1..100, fn(_) = random(1, 6)), fn(d) = d >= 1 and d <= 6))
  end

The property tests (check_all, sample) have their own seed:
rythme test --seed N (see rythme help tests)."#,
            "",
        ),
        // ----- the debugger (src/debug/, src/help.rs)
        ("this file is not part of the program being debugged", ""),
        ("top level", ""),
        (" [tail call, replaced {by}]", ""),
        ("`{name}` gave {}", ""),
        ("error: {}", ""),
        ("Locals", ""),
        ("Globals", ""),
        ("result of {name}", ""),
        ("no source to give: the files are on the disk", ""),
        ("unknown request `{cmd}`", ""),
        ("the program is running: pause it first", ""),
        ("which program? Give its path in `program`, like \"${{file}}\"", ""),
        ("the program has errors: see the debug console", ""),
        ("the program is not running: `{cmd}` has nothing to show", ""),
        ("the program has ended: `{cmd}` has nothing to show", ""),
        ("`print` shows the value of one expression, like `print n * 2` or `print len(xs)`", ""),
        ("`{name}` has an effect: the debugger only computes values, without changing anything", ""),
        ("no code where the program can pause on line {line} of {name} or after it", ""),
        ("no function named `{name}` in the program", ""),
        (": did you mean `{best}`?", ""),
        (
            r#"Commands (short forms in parentheses):
  break 12         pause at line 12 of the program (b)
  break file.ry:3  pause at line 3 of another file of the program
  break fact       pause each time the function fact starts
  delete 2         remove breakpoint 2; delete alone removes them all (d)
  breakpoints      list the breakpoints
  run              start the program (r)
  continue         go on until the next breakpoint (c)
  next             go to the next line, without entering the functions called (n)
  step             go to the next line, entering the functions called (s)
  finish           go on until this function gives its result (f)
  where            the calls in progress, with their arguments (w)
  frame 2          look at the call number 2 of where (print and locals use it)
  print n * 2      the value of an expression, computed in the current call (p)
  locals           the names of the current call and their values (l)
  list             the lines around the current one
  help             this list (h)
  quit             stop the program and the debugger (q)
An empty line repeats next, step, finish or continue."#,
            "",
        ),
        ("unknown command `{word}`", ""),
        (" (`help` lists the commands)", ""),
        ("break where? A line (`break 12`), a file and a line (`break geometry.ry:3`) or a function (`break fact`)", ""),
        ("no file `{name}` in the program (its files: {})", ""),
        ("breakpoint {n} at {shown_at}", ""),
        ("breakpoint {n} at {shown_at} (line {line} has no code where the program can pause)", ""),
        ("breakpoint {n} at the start of `{arg}`", ""),
        ("`{arg}` is not a line number", ""),
        ("no breakpoints: add one with `break 12` or `break fact`", ""),
        ("start of `{name}`", ""),
        ("all breakpoints removed", ""),
        ("breakpoint {n} removed", ""),
        ("there is no breakpoint {n} (see `breakpoints`)", ""),
        ("delete which breakpoint? Give its number, like `delete 1`", ""),
        ("● breakpoint {}", ""),
        ("● breakpoint {n}: `{name}` starts", ""),
        ("✗ the program stops on an error: {msg}", ""),
        ("  look at the values with where, locals and print; continue shows the whole message", ""),
        (" · depth {depth}", ""),
        (" ({rec} calls of `{name}` in progress)", ""),
        ("  (a call in tail position: it took the place of the frame of `{by}`)", ""),
        ("the program is already running: continue goes on, quit stops it", ""),
        (" (replaced `{by}`: tail call)", ""),
        ("`{name}` calls itself: {rec} calls in progress", ""),
        ("which call? A number from 0 to {}, as `where` shows them", ""),
        ("no names here yet", ""),
        ("print what? An expression, like `print n` or `print len(xs) * 2`", ""),
        ("debugger stopped", ""),
        ("the program is not running: `run` starts it (`step` pauses at its first line)", ""),
        ("which file? Example: rythme debug factorial.ry (see `rythme help debug`)", ""),
        ("Debugging {shown}. Pause somewhere with `break 12` or `break function`, then `run`; `help` lists the commands.", ""),
        ("The program finished. `run` starts it again, `quit` leaves.", ""),
        ("The program stopped on an error. `run` starts it again, `quit` leaves.", ""),
        (
            r#"THE DEBUGGER: rythme debug

`rythme debug` runs a program and pauses it where you ask, to look at the
values of its names while it runs:

  rythme debug factorial.ry          then type commands at the prompt (debug)

  break 12         pause at line 12 (or at the next line with code)
  break geo.ry:3   pause at line 3 of a file the program uses
  break fact       pause each time the function fact starts
  run              start the program (step: pause at its first line)
  next             go to the next line, without entering the functions called
  step             go to the next line, entering the functions called
  finish           go on until this function gives its result, and show it
  continue         go on until the next breakpoint
  where            the calls in progress, with their arguments
  frame 1          look at another call of where (print and locals use it)
  print n * 2      the value of an expression, computed where the program is
  locals           the names visible here and their values
  list             the lines around the current one
  delete 1         remove a breakpoint (breakpoints lists them)
  quit             stop

An empty line repeats next, step, finish or continue.

Each pause shows the line, the call (fact(n = 2)) and the depth: how many
calls are in progress, and how many of the same function when it calls
itself. A call in tail position (the last thing a function does) takes the
place of the frame of its caller: the debugger says so, and where shows it.

print computes only values: the functions that print, read, write files or
draw random numbers are refused there, so that looking never changes the
program. When an error stops the program, the debugger pauses where it
happened, so that you can look at the values before the message.

In VS Code: the Rythme extension adds the debugger (Run > Start Debugging,
or F5 on a .ry file): breakpoints in the margin, the values of the names,
step over, step into, step out. Other editors can start `rythme dap`, which
speaks the Debug Adapter Protocol.
The function `debug(value)` is something else: it prints a value and gives it back."#,
            "",
        ),
        // src/check/annot.rs, src/parser.rs: types written in the code
        ("a whole number is an Int, not a Float: write it with a decimal point (`2.0`), or convert it with `float(...)`", ""),
        ("a decimal number is a Float, not an Int: convert it with `int(...)`, `round(...)` or `floor(...)`", ""),
        ("change the value, or the type written for it; types written in the code are checked before running, and are never needed (see `rythme help types`)", ""),
        ("`{name}` is written `{w}`, but its value is {what}", ""),
        ("the type written for `{name}`", ""),
        ("this value is {what}, but the type written for it is `{w}`", ""),
        ("the type written here", ""),
        ("not `{w}`", ""),
        ("`{v}` cannot be both {a} and {b} in this call to `{f}`", ""),
        ("`{v}` is {b} here", ""),
        ("`{v}` is {a} here", ""),
        ("a type variable like `{v}` stands for the same type everywhere in the types written for `{f}`", ""),
        ("`{f}` expects `{ws}` for `{param}`, but receives {what}", ""),
        ("not `{ws}`", ""),
        ("the type written for `{param}`", ""),
        ("this function receives {what}, but `{param}` is written `{w}`", ""),
        ("`{f}` is written to give `{w}`, but gives {what}", ""),
        ("the type written for the result of `{f}`", ""),
        ("`{f}` is written to give `{w}`, but its `?` may give back {what}", ""),
        ("`?` gives back the `None` or `Err(...)` it stops on: write the result as an Option or a Result, like `-> Option(Int)`", ""),
        ("unknown type `{name}`", ""),
        ("in Rythme, write `{best}`", ""),
        ("the names of types start with an uppercase letter; a type variable, which stands for any type, is one lowercase letter, like `a`", ""),
        ("`{name}` takes no type between parentheses", ""),
        ("write `{name}` alone", ""),
        ("a type is one of Rythme's (`Int`, `Float`, `Text`, `Bool`, `List`, `Dict`...) or one defined with `type`", ""),
        ("`{name}` takes one type: `{example}`", ""),
        ("`{name}` takes two types: `{example}`", ""),
        ("to write the type of `{name}`, put a space after `:`: `{name}: {ty} = ...`", ""),
        ("`=` and a value", ""),
        ("a name with a written type is given its value right away: `total: Float = 0.0`", ""),
        ("`,`", ""),
        ("to write the type of a parameter, put `:` right after its name, then a space: `x: {s}`", ""),
        ("`:` and the type of the field", ""),
        ("`(`", ""),
        ("`->` and the type of the result", ""),
        ("a type", ""),
        ("to write the type of a field, write its name first: `Circle(radius: Float)`", ""),
        ("a type is written like `Int`, `Text`, `List(Int)`, `[Int]`, `{{name: Text}}`, `fn(Int) -> Int` or `Shape`; see `rythme help types`", ""),
        (
            r#"E027 — A type written in the code does not fit

A type written in the code, after `:` or `->`, is a promise that
`rythme check` verifies before running: a value that surely has another type
is reported where it is given. The type must also exist.

Wrong:   total: Float = 0              # 0 is an Int
Right:   total: Float = 0.0
Wrong:   fn area(w: Int, h: Int) -> Int = w * h
         area(2, "3")                  # "3" is a Text
Right:   area(2, 3)
Wrong:   fn label(n: Int) -> Text = n  # gives an Int
Right:   fn label(n: Int) -> Text = text(n)
Wrong:   fn twice(x: int) = x * 2      # type names start with an uppercase letter
Right:   fn twice(x: Int) = x * 2
Wrong:   fn depth(t: Tree(Int, Text))  # type Tree(a) takes one type
Right:   fn depth(t: Tree(Int))

Written types are optional: removing one never changes what a program does.
More: `rythme help types`."#,
            "",
        ),
        (
            r#"TYPES WRITTEN IN THE CODE (optional)

Rythme finds the types by itself. You may also write them where they help
to read the code; `rythme check` then verifies them (E027):
  fn area(width: Int, height: Int) -> Int = width * height
  total: Float = 0.0
  type Shape = Circle(radius: Float) | Rect(width: Float, height: Float)
  [1, 2].map do |n: Int| n * 2 end

The types
  Int  Float  Text  Bool  Unit  Symbol
  List(Int) or [Int]   Set(Text)   Dict(Text, Int)
  Option(Int)   Result(Int, Text)
  {name: Text, age: Int}       a record with at least these fields
  fn(Int, Int) -> Int          a function
  Shape                        a type defined with `type` (geometry.Shape
                               for a type of `use geometry`)
  Tree(Int)                    a type defined with parameters, with its types
  a                            any type: one lowercase letter, the same type
                               everywhere in a definition
  List, Dict, Set, Option, Result, Record, Function alone: any content
  (Tree alone too)

  fn first_of(xs: List(a)) -> a = xs[0]
  first_of([1, 2]) + 1         # the checker knows it is an Int

Types with parameters: a type can take types, like List(Int)
  type Tree(a) = Leaf | Node(left: Tree(a), value: a, right: Tree(a))
  type Pair(a, b) = Pair(first: a, second: b)

  fn depth(t: Tree(a)) -> Int = ...      a Tree of anything
  fn total(t: Tree(Int)) -> Int = ...    a Tree of whole numbers
  total(Node(Leaf, "two", Leaf))         E027: this is a Tree(Text)

A type variable is one letter: a lowercase word, like `int` or `string`, is
taken for a type named as in another language, and `rythme check` gives its
Rythme name.

Written types are never needed and change nothing when the program runs;
the editors and `rythme doc` show them."#,
            "",
        ),
        // src/fix.rs, src/machine.rs: fixes and output for programs
        ("`{f}` is not available in the sandbox: a program run by a tool (`--sandbox`) cannot touch files or folders. Run it yourself: rythme file.ry", ""),
        ("Remove `{}`", ""),
        ("\"not equal\" is written `!=`: `a != b`", ""),
        ("an anonymous function is written `fn(x) = x * 2`", ""),
        (
            r#"on several lines, `if` is written:
  if x > 0 then
    "positive"
  else if x == 0 then
    "zero"
  else
    "negative"
(no `end`; for several statements in a branch, use `do ... end`)"#,
            "",
        ),
        ("Rythme texts put values in by themselves: write `\"total: {{x}}\"`, without `f` in front", ""),
        ("`--format` takes `json` or `text`, not `{v}`: `rythme check --format json file.ry`", ""),
        ("`--watch` does not go with `--format json`: run `rythme test --format json` again after each change", ""),
        ("unknown option `{bad}` for `rythme fix`; the options are: --dry-run, --all, --format json", ""),
        ("which file? Example: rythme fix hello.ry (or rythme fix --dry-run hello.ry to see the changes first)", ""),
        ("cannot write `{shown}`: {e}", ""),
        ("{shown}: nothing to fix", ""),
        ("{shown}: {n} fix(es) to apply", ""),
        ("{shown}: {n} fix(es) applied", ""),
        ("; {errors} error(s) and {warnings} warning(s) left: `rythme check {shown}`", ""),
        ("here: `new_{name} = {name} {op} {rest}`, then use `new_{name}`; to add up a list, use `sum(list)` or `fold`", ""),
        // src/mcp.rs: the server for AI assistants
        ("`rythme mcp` takes no argument, not `{bad}`", ""),
        (
            r#"rythme mcp: a Model Context Protocol server for AI assistants, on the standard input and output.
Its tools run (in a sandbox), check, test, format and fix Rythme code, and explain errors and functions.
Add it to Claude Code: claude mcp add rythme -- rythme mcp
Claude Desktop, Cursor and others: see the manual, section 7.10."#,
            "",
        ),
        ("the program was stopped after {seconds} seconds: it may never end (a recursion without a base case?)", ""),
        ("the program was stopped: it took more than {mb} MB of memory", ""),
        // src/fix.rs, src/suggest.rs: habits from other languages
        ("an anonymous function is written `fn(x) = x * 2`; a block given to a call is written `xs.map do |x| x * 2 end` or `xs.map {{ |x| x * 2 }}`", ""),
        ("the parameters of a block are written between bars: `xs.map {{ |x| x * 2 }}`", ""),
        ("to add up a list, write `sum(xs)`; to combine its elements, `fold(xs, 0, fn(acc, x) = acc + x)`", ""),
        ("a part of a list or a text is taken with a function: `slice(xs, 1, 3)`, `drop(xs, 1)` (all but the first), `take(xs, 2)` (the first two)", ""),
        ("the fields of a record are written with `:`: `{{{field}: value}}`", ""),
        ("Rythme has no named arguments (`name=value`): give the values in order, like `round(x, 2)` (`rythme help NAME` shows the order)", ""),
        ("texts have no format specifiers like `{{x:.2f}}`: write `{{text(x, 2)}}` for 2 decimals, or use `format_number`", ""),
        ("when there is nothing to do otherwise, give the unit value: `if x > 0 then print(x) else ()`", ""),
        ("here: `new_{w} = ...`, then use `new_{w}`; to repeat a step, use `fold`, `map` or a recursive function", ""),
        ("the functions of numbers are built in: write `sqrt(x)`, `abs(x)`, `floor(x)`, `pi`... without `Math.`", ""),
        ("Rythme has no generators: build the whole list (`map`, `filter`, `1..n`), or give a function to call", ""),
        // src/builtins/ml.rs: vectors, matrices, statistics, chance
        ("`{f}` expects a list of numbers as argument {pos}, but its element {k} is {got}", ""),
        ("a text is not a number: `parse_float(text)` reads one", ""),
        ("`{f}` expects a matrix (a list of rows, each a list of numbers) as argument {pos}, but its row {r} is {got}", ""),
        ("a matrix of 2 rows and 3 columns: [[1, 2, 3], [4, 5, 6]]", ""),
        ("`{f}`: argument {pos} is not a matrix: its row 0 has {cols} number(s), but its row {r} has {n}", ""),
        ("every row of a matrix has the same length", ""),
        ("`{f}` expects a matrix of numbers as argument {pos}, but its row {r} holds {got} at position {k}", ""),
        ("a vector of {n} number(s)", ""),
        ("a {n}×{c} matrix", ""),
        ("a list of {n} element(s)", ""),
        ("`{f}` needs two values of the same shape, but got {a} and {b}", ""),
        ("`shape(x)` gives the shape of a vector or a matrix: [3] or [2, 3]", ""),
        ("`{f}`: these lists are nested more than {MAX_NESTING} levels deep: they are not a vector nor a matrix", ""),
        ("`{f}` works on vectors and matrices of numbers, but found {got} in them", ""),
        ("`{f}` needs two vectors of the same length, but got a vector of {n} number(s) and a vector of {m}", ""),
        ("`zip_with` needs two lists of the same length, but got {n} and {m} element(s)", ""),
        ("a {r}×{c} matrix", ""),
        ("a vector of {nr} number(s)", ""),
        ("a {nr}×{nc} matrix", ""),
        ("`matmul` cannot multiply {first} by {second}: the first one has {c} column(s), so the second one needs {c} row(s), not {nr}", ""),
        ("`shape(x)` gives the shape of each one; `transpose(m)` swaps rows and columns", ""),
        ("`matmul`: these matrices are too large to multiply here (more than {} multiplications)", ""),
        ("`shape`: the elements of this list do not all have the same shape: element 0 is {a}, element {i} is {b}", ""),
        ("`{f}` of an empty list", ""),
        ("`percentile` needs a percentage from 0 to 100, not {p}", ""),
        ("`random_normal` needs a standard deviation of 0 or more, not {sd}", ""),
        ("`random_choice_weighted` needs one weight per element: {n} element(s), but {m} weight(s)", ""),
        ("`random_choice_weighted`: the weight {i} is {w}; a weight is a number of 0 or more", ""),
        ("`random_choice_weighted` needs at least one weight above 0", ""),
        ("`train_test_split` needs a share from 0 to 1 for the training part (like 0.8), not {ratio}", ""),
        ("Machine learning: vectors, matrices, statistics", ""),
        ("Hyperbolic tangent: a smooth step from -1 to 1, used as the activation of neurons.", ""),
        ("The logistic function 1 / (1 + e^-x): a smooth step from 0 to 1, read as a probability. Its slope is `sigmoid(x) * (1 - sigmoid(x))`.", ""),
        ("The rectifier: x when it is positive, 0.0 otherwise. The usual activation of deep networks.", ""),
        ("Dot product of two vectors (lists of numbers) of the same length: the sum of the products of their numbers, one by one.", ""),
        ("Length of a vector: the square root of the sum of the squares of its numbers.", ""),
        ("Adds two vectors, or two matrices, of the same shape, number by number.", ""),
        ("Subtracts two vectors, or two matrices, of the same shape, number by number.", ""),
        ("Multiplies two vectors, or two matrices, of the same shape, number by number (not the product of matrices: that is `matmul`).", ""),
        ("Multiplies every number of a vector, or of a matrix, by the number k.", ""),
        ("Calls f with the elements of two lists of the same length, one pair at a time, and gives the list of the results.", ""),
        ("Product of two matrices (lists of rows): a matrix of r rows and n columns times a matrix of n rows and c columns gives r rows and c columns. With a vector as `b`, gives a vector.", ""),
        ("Swaps the rows and the columns of a matrix.", ""),
        ("A vector of `rows` zeros, or with `columns`, a matrix full of zeros.", ""),
        ("A vector of `rows` ones, or with `columns`, a matrix full of ones.", ""),
        ("The identity matrix of n rows and n columns: ones on the diagonal, zeros elsewhere.", ""),
        ("The shape of a vector or a matrix (of lists nested in lists): its length at each level, `[3]` for a vector of 3 numbers, `[2, 3]` for 2 rows of 3 numbers.", ""),
        ("A summary of a list of numbers, as a record: `count`, `mean` (the average), `median` (the middle value), `std_dev`, `min` and `max`. `stats(xs).mean` is the mean.", ""),
        ("The variance of a list of numbers: the mean of the squares of their distances to their mean (of the whole population: divided by n).", ""),
        ("The standard deviation of a list of numbers: the square root of their variance, how far they usually are from their mean.", ""),
        ("The number below which p percent of the numbers of the list are (p from 0 to 100), between the two closest ones when needed. `percentile(xs, 50)` is the median, the middle value.", ""),
        ("The position of the largest number of a list (the first one if several are equal).", ""),
        ("The position of the smallest number of a list (the first one if several are equal).", ""),
        ("Turns a list of scores into probabilities: each one becomes e^x divided by the sum of them all, so that they are between 0 and 1 and add up to 1, the largest score keeping the largest share.", ""),
        ("Rescales a list of numbers from 0.0 (the smallest) to 1.0 (the largest), keeping their proportions. When they are all equal, gives zeros.", ""),
        ("A random decimal number drawn from the normal distribution (the bell curve) of this mean and standard deviation. Follows `random_seed`.", ""),
        ("An element of the list chosen at random, each one with a chance in proportion to its weight (numbers of 0 or more, one per element). Follows `random_seed`.", ""),
        ("Shuffles a list, then cuts it in two: a record `{train, test}`, where `train` holds this share of the elements (0.8: 80 percent) and `test` the rest. To learn on some examples and check on others. Follows `random_seed`.", ""),
        // src/builtins/grad.rs: slopes
        ("the function given to `{f}` must return a number, but it returned {got}", ""),
        ("The slope of the function f at x (its derivative), computed from f(x + h) and f(x - h) for a tiny h: precise to about 9 significant digits for a smooth function. f takes one number and gives one.", ""),
        ("The gradient of f at the point xs: the list of the slopes of f along each number of xs (see `derivative`). f takes a list of numbers and gives one number. Going against it lowers f: the heart of gradient descent.", ""),
        // src/builtins/plot.rs: charts as text
        ("`{f}` has nothing to draw: the list is empty, or holds no finite number", ""),
        ("`histogram` draws at most 100 bars, not {bins}", ""),
        ("Draws the points (x, y) as a chart in text, 50 columns wide and 11 lines high, with the smallest and largest values on the axes; with one list, draws its numbers at the positions 0, 1, 2... Gives a text: `print(plot(losses))`.", ""),
        ("Counts how many numbers fall in each of `bars` intervals of the same width (10 if not given) and draws them as horizontal bars of text, with the interval and the count of each. Gives a text: `print(histogram(heights))`.", ""),
        // src/builtins/http.rs: HTTP requests
        ("`{f}`: the names and the values of the headers are texts, not {got}", ""),
        ("for example: {{\"content-type\": \"application/json\", \"x-api-key\": key}}", ""),
        ("`{f}` is not available in the browser: the playground cannot make requests. Run the program on your computer: rythme file.ry", ""),
        ("a web address starts with http:// or https://, not \"{url}\"", ""),
        ("an address or a header cannot hold a line break or a control character, and a header name is a word without `:` or space", ""),
        ("the server answered {code}: {shown}{more}", ""),
        ("`{f}` needs the program `curl`, which comes with Windows 10, macOS and most Linux systems: install it (on Debian or Ubuntu: sudo apt install curl)", ""),
        ("server not found: {host} (no connection to the Internet, or a mistake in the address?)", ""),
        ("could not connect to {host}: {said}", ""),
        ("no answer from {host} in time (30 seconds)", ""),
        ("the secure connection (TLS) to {host} failed: {said}", ""),
        ("the request to {host} failed: {said}", ""),
        ("The web: HTTP requests", ""),
        ("Fetches a web address (http:// or https://): `Ok(text of the answer)`, or `Err(message)` when there is no network, the address is wrong, the server answers with an error (404...) or does not answer within 30 seconds. `headers` is a record of texts, like `{\"accept\": \"application/json\"}`. Uses the program curl; not available in the playground.", ""),
        ("Sends a text (often JSON, made with `to_json`) to a web address and gives `Ok(text of the answer)` or `Err(message)`, like `http_get`. To call a web service or a language model: `http_post(url, to_json(request), {\"content-type\": \"application/json\"})`. Keep keys out of your code: read them with `env`.", ""),
        // docs/exercises: the exercises about artificial intelligence (31 to 36)
        ("31-vectors", ""),
        (
            r#"# Exercise 31 · Points as vectors  ★☆☆
#
# In machine learning, an example is a list of numbers: a vector. The
# built-in functions `vector_add`, `vector_sub`, `vector_scale`, `dot` and
# `norm` work on them, and always give Floats.
#   vector_sub([5, 5], [1, 2])     # [4.0, 3.0]
#   norm([3, 4])                   # 5.0
#
# Statement
#   - `distance(a, b)`: the distance between the points a and b (the length
#     of the vector from one to the other);
#   - `midpoint(a, b)`: the point halfway between a and b;
#   - `closest(points, p)`: the point of the list nearest to p.
#
# Hints
#   1. `distance` is the `norm` of a difference.
#   2. Halfway: add the two points, then scale by 0.5.
#   3. `min_by(list, f)` gives the element for which f is the smallest.
#
# Check your work:  rythme exercise check 31-vectors.ry
#              or:  rythme test 31-vectors.ry

fn distance(a, b) = fail("to do")

fn midpoint(a, b) = fail("to do")

fn closest(points, p) = fail("to do")

# --- Tests (do not change them) ---

test "distance"
  assert_eq(distance([0, 0], [3, 4]), 5.0)
  assert_eq(distance([1, 1, 1], [1, 1, 1]), 0.0)
end

test "midpoint"
  assert_eq(midpoint([0, 0], [4, 2]), [2.0, 1.0])
  assert_eq(midpoint([-1, 3], [1, 3]), [0.0, 3.0])
end

test "closest"
  assert_eq(closest([[0, 0], [5, 5], [9, 1]], [6, 4]), [5, 5])
  assert_eq(closest([[2, 2]], [100, -100]), [2, 2])
end
"#,
            "",
        ),
        ("32-statistics", ""),
        (
            r#"# Exercise 32 · Statistics on data  ★★☆
#
# Before learning from data, look at it. `stats(xs)` gives a record with the
# count, the mean, the median, the standard deviation, the min and the max:
#   stats([2, 4, 4, 4, 5, 5, 7, 9]).mean       # 5.0
#   stats([2, 4, 4, 4, 5, 5, 7, 9]).std_dev    # 2.0
#
# Statement
#   - `standardize(xs)`: each number becomes its distance to the mean,
#     counted in standard deviations: (x - mean) / std_dev;
#   - `outliers(xs)`: the numbers more than 2 standard deviations away from
#     the mean, in their order;
#   - `accuracy(predictions, answers)`: the share (from 0.0 to 1.0) of the
#     predictions equal to the answer at the same position.
#
# Hints
#   1. Compute `stats(xs)` once, give it a name, then `map`.
#   2. `abs` gives the distance whatever the side.
#   3. `zip(a, b)` pairs the elements; `count` counts those that are right.
#
# Check your work:  rythme exercise check 32-statistics.ry
#              or:  rythme test 32-statistics.ry

fn standardize(xs) = fail("to do")

fn outliers(xs) = fail("to do")

fn accuracy(predictions, answers) = fail("to do")

# --- Tests (do not change them) ---

test "standardize"
  assert_eq(standardize([2, 4, 4, 4, 5, 5, 7, 9]), [-1.5, -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 2.0])
end

test "outliers"
  assert_eq(outliers([10, 11, 9, 10, 10, 12, 10, 9, 11, 10, 50]), [50])
  assert_eq(outliers([1, 2, 3]), [])
end

test "accuracy"
  assert_eq(accuracy(["cat", "dog", "cat", "cat"], ["cat", "dog", "dog", "cat"]), 0.75)
  assert_eq(accuracy([1, 0], [0, 1]), 0.0)
end
"#,
            "",
        ),
        ("33-neurons", ""),
        (
            r#"# Exercise 33 · An artificial neuron  ★★☆
#
# A neuron multiplies each input by a weight, adds them up with a bias, and
# passes the sum through an activation function such as `sigmoid`, which
# gives a number between 0 and 1.
#   sigmoid(0)                     # 0.5
#   dot([1, 2], [3, 4])            # 11.0
#
# Statement
#   - `neuron(weights, bias, inputs)`: sigmoid(weights · inputs + bias);
#   - `fires(weights, bias, inputs)`: true when the neuron gives 0.5 or more;
#   - `layer(matrix, biases, inputs)`: a layer of neurons, one per row of the
#     matrix (its weights) and per bias: the list of their outputs.
#
# Hints
#   1. `dot` gives the weighted sum.
#   2. `matmul(matrix, inputs)` computes every weighted sum of a layer at
#      once; `vector_add` adds the biases; `map` applies `sigmoid`.
#
# Check your work:  rythme exercise check 33-neurons.ry
#              or:  rythme test 33-neurons.ry

fn neuron(weights, bias, inputs) = fail("to do")

fn fires(weights, bias, inputs) = fail("to do")

fn layer(matrix, biases, inputs) = fail("to do")

# --- Tests (do not change them) ---

test "neuron"
  assert_eq(neuron([1, -1], 0, [2, 2]), 0.5)
  assert_eq(round(neuron([2, 1], -1, [1, 1]), 3), 0.881)
end

test "a neuron that computes AND"
  assert(fires([10, 10], -15, [1, 1]))
  assert(not fires([10, 10], -15, [1, 0]))
  assert(not fires([10, 10], -15, [0, 0]))
end

test "layer"
  outputs = layer([[1, -1], [1, 1]], [0, 1], [2, 2])
  assert_eq(map(outputs, fn(x) = round(x, 3)), [0.5, 0.993])
end
"#,
            "",
        ),
        ("34-gradient-descent", ""),
        (
            r#"# Exercise 34 · Going down the slope  ★★☆
#
# A model learns by making its error smaller: it follows the slope of the
# error downwards, one small step at a time. That is gradient descent.
#   derivative(fn(x) = x * x, 3)                      # 6.0, the slope at 3
#   gradient(fn([x, y]) = x * x + y * y, [1, 2])      # [2.0, 4.0]
#
# Statement
#   - `step(f, x, rate)`: one step down from x: x - rate × (slope of f at x);
#   - `descend(f, x, rate, steps)`: that many steps from x, one after the
#     other; the point reached;
#   - `descend_all(f, xs, rate, steps)`: the same with several numbers: each
#     step is xs - rate × (gradient of f at xs).
#
# Hints
#   1. `descend` calls itself with one step less, until 0 (or use `fold`
#      over `1..steps`).
#   2. `vector_sub` and `vector_scale` work on the list of numbers.
#
# Check your work:  rythme exercise check 34-gradient-descent.ry
#              or:  rythme test 34-gradient-descent.ry

fn step(f, x, rate) = fail("to do")

fn descend(f, x, rate, steps) = fail("to do")

fn descend_all(f, xs, rate, steps) = fail("to do")

# --- Tests (do not change them) ---

fn parabola(x) = (x - 3) ** 2

test "one step"
  assert_eq(step(parabola, 0, 0.5), 3.0)
  assert_eq(step(parabola, 1, 0.25), 2.0)
end

test "down to the bottom"
  assert_eq(round(descend(parabola, 0, 0.1, 100), 4), 3.0)
  assert_eq(descend(parabola, 10, 0.1, 0), 10)
end

test "several numbers"
  bowl = fn([x, y]) = (x - 1) ** 2 + (y + 2) ** 2
  assert_eq(map(descend_all(bowl, [0, 0], 0.1, 100), fn(v) = round(v, 4)), [1.0, -2.0])
end
"#,
            "",
        ),
        ("35-nearest-neighbours", ""),
        (
            r#"# Exercise 35 · Nearest neighbours  ★★★
#
# To guess the label of a new point, look at the k known examples closest to
# it, and take the label most of them have. Each example is a record:
#   {point: [1.0, 2.0], label: "red"}
#
# Statement
#   - `neighbours(examples, point, k)`: the k examples nearest to the point,
#     from the nearest;
#   - `majority(labels)`: the label that comes most often (the first one to
#     reach that count, in the order of the list, when several tie);
#   - `classify(examples, point, k)`: the label most of the k nearest
#     neighbours have.
#
# Hints
#   1. `sort_by` with the distance (`norm(vector_sub(a, b))`), then `take`.
#   2. `count(labels, fn(l) = l == label)` counts one label; `max_by` keeps
#      the first of the largest.
#
# Check your work:  rythme exercise check 35-nearest-neighbours.ry
#              or:  rythme test 35-nearest-neighbours.ry

fn neighbours(examples, point, k) = fail("to do")

fn majority(labels) = fail("to do")

fn classify(examples, point, k) = fail("to do")

# --- Tests (do not change them) ---

garden = [
  {point: [1, 1], label: "red"},
  {point: [1, 2], label: "red"},
  {point: [2, 1], label: "red"},
  {point: [8, 8], label: "blue"},
  {point: [9, 8], label: "blue"},
  {point: [5, 5], label: "blue"},
]

test "neighbours"
  assert_eq(neighbours(garden, [0, 0], 2) |> map(fn(e) = e.point), [[1, 1], [1, 2]])
  assert_eq(len(neighbours(garden, [0, 0], 10)), 6)
end

test "majority"
  assert_eq(majority(["a", "b", "b", "c"]), "b")
  assert_eq(majority(["x", "y"]), "x")
end

test "classify"
  assert_eq(classify(garden, [2, 2], 3), "red")
  assert_eq(classify(garden, [7, 7], 3), "blue")
  assert_eq(classify(garden, [4, 4], 1), "blue")
end
"#,
            "",
        ),
        ("36-minimax", ""),
        (
            r#"# Exercise 36 · Playing perfectly: minimax  ★★★
#
# A game of Nim: there are n matches on the table; each player in turn takes
# 1, 2 or 3 of them, and whoever takes the last match wins. Minimax decides
# who wins with perfect play by imagining every move: the player to move
# wins if one of their moves leaves the other player in a losing position.
#
# Statement
#   - `moves(n)`: the possible numbers of matches to take (1, 2 or 3, but no
#     more than n);
#   - `wins(n)`: true when the player to move can always win from n matches
#     (with 0 matches left, the player to move has already lost);
#   - `best_move(n)`: `Some(m)`, where m is the smallest move that wins, or
#     `None` when every move loses.
#
# Hints
#   1. `filter(1..3, ...)` keeps the moves allowed.
#   2. `wins(n)`: is there `any` move after which the other player does
#      not win?
#   3. `find` gives the first element that fits, as an Option.
#
# Check your work:  rythme exercise check 36-minimax.ry
#              or:  rythme test 36-minimax.ry

fn moves(n) = fail("to do")

fn wins(n) = fail("to do")

fn best_move(n) = fail("to do")

# --- Tests (do not change them) ---

test "moves"
  assert_eq(moves(10), [1, 2, 3])
  assert_eq(moves(2), [1, 2])
  assert_eq(moves(0), [])
end

test "who wins"
  assert_eq(map(0..8, wins), [false, true, true, true, false, true, true, true, false])
end

test "best move"
  assert_eq(best_move(5), Some(1))
  assert_eq(best_move(7), Some(3))
  assert_eq(best_move(8), None)
end
"#,
            "",
        ),
        // src/help.rs: rythme help ml
        (
            r#"MACHINE LEARNING AND AI

A vector is a list of numbers, a matrix a list of rows of the same length.
These functions never change them: they give new lists, always of Floats.

Vectors and matrices
  dot([1, 2], [3, 4])            # 11.0
  norm([3, 4])                   # 5.0, the length
  vector_add(a, b)  vector_sub(a, b)  vector_mul(a, b)   # number by number
  vector_scale([1, 2], 3)        # [3.0, 6.0]
  matmul(m, n)  matmul(m, v)     # product of matrices, or matrix by vector
  transpose(m)  shape(m)  zeros(2, 3)  ones(2)  identity_matrix(3)
  zip_with(xs, ys, f)            # f on the pairs, for anything else

Neurons
  sigmoid(x)  tanh(x)  relu(x)   softmax(scores)   # scores into probabilities

Statistics
  stats(xs)                      # {count, mean, median, std_dev, min, max}
  variance(xs)  std_dev(xs)  percentile(xs, 90)  argmax(xs)  argmin(xs)
  normalize(xs)                  # from 0.0 (smallest) to 1.0 (largest)

Chance, the same on every run after random_seed(n)
  random_normal(0, 1)  random_choice_weighted(xs, weights)
  train_test_split(examples, 0.8)    # {train, test}

Slopes, for gradient descent (central differences, about 9 digits)
  derivative(fn(x) = x * x, 3)               # 6.0
  gradient(fn([x, y]) = x * x + y, [2, 5])   # [4.0, 1.0]

Charts as text
  print(plot(losses))  print(plot(xs, ys))  print(histogram(heights))

The web and language models
  http_get(url)  http_post(url, body, headers)   # Ok(text) or Err(message)
  Keep API keys out of the code: env("ANTHROPIC_API_KEY"). The example
  examples/ask_claude.ry asks Claude a question with http_post and JSON.

A mistake of shape names both shapes ("cannot multiply a 2×3 matrix by a
2×2 matrix"). Programs to read: the examples linear_regression, perceptron,
xor_network, kmeans, knn, naive_bayes, markov, tictactoe, pathfinding and
genetic; exercises 31 to 36 (rythme exercise list)."#,
            "",
        ),
        // Types with parameters: src/check/annot.rs, src/help.rs, src/lsp/features.rs, src/parser.rs
        ("the type variable `{name}` appears twice in `{ty}`", ""),
        ("give each one its own letter: `type Pair(a, b)`", ""),
        ("`{name}` cannot be a type variable: a type variable is one lowercase letter", ""),
        ("write one letter, like `{letter}`, here and in the fields", ""),
        ("`{v}` is not a parameter of `{ty}`", ""),
        ("not among `{header}`", ""),
        ("add it to the parameters of the type: `type {with} = ...`", ""),
        ("`{name}` takes {count} types: `{example}`", ""),
        (
            r#"Types may also be written in the code, where they help to read it:
  fn area(s: Shape) -> Float
`rythme check` verifies them; they are never needed (rythme help types)."#,
            "",
        ),
        ("A whole number, as large as needed: `42`, `-7`.", ""),
        ("A decimal number: `3.14`, `2.0`.", ""),
        ("A text: `\"hello\"`.", ""),
        ("`true` or `false`.", ""),
        ("`()`: what a function gives when it gives nothing, like `print`.", ""),
        ("A value that is only its name: `:ok`, `:error`.", ""),
        ("A list whose elements all have the type between parentheses; `List` alone: any list.", ""),
        ("A set of values of this type: `set([1, 2])`.", ""),
        ("A dictionary: keys of the first type, values of the second.", ""),
        ("`Some(value)` or `None`: a value that may be missing.", ""),
        ("`Ok(value)` or `Err(error)`: the result of what may fail.", ""),
        ("Any record; `{{name: Text}}` is a record with at least these fields.", ""),
        ("Any function; `fn(Int) -> Int` is a function with these types.", ""),
        ("A type of Rythme: `rythme help types`", ""),
        ("type variable", ""),
        ("used file", ""),
        ("a type variable, like `a`", ""),
        ("a type without parameters is written without parentheses: `type {name} = ...`", ""),
        ("the parameters of a type are type variables, one lowercase letter each: `type Box(a) = Box(value: a)`", ""),
        // src/index.rs, src/packages.rs, src/project.rs, src/manifest.rs, src/help.rs: the package index
        ("the address is empty", ""),
        ("`{a}` is not an address that Rythme accepts: no spaces, and no `-` at the start", ""),
        ("Rythme downloads packages only with `https://`, `ssh://` or from this computer (`file://`), not with `{other}`", ""),
        ("`{a}` is not the address of a repository: write `https://...`, `ssh://...` or `file://...`", ""),
        ("set by `index` in `rythme.toml`", ""),
        ("set by the variable RYTHME_INDEX", ""),
        ("`{url}` is not a package index: it has no folder `packages`", ""),
        ("an index holds one file per package, like `packages/mathlib.toml` (see `rythme help packages`)", ""),
        ("an unknown day", ""),
        ("cannot update the package index from `{url}`: using the copy of {date}", ""),
        ("cannot download the package index from `{url}`", ""),
        ("check the connection to the network, or give another index with the variable RYTHME_INDEX (an address or a folder); dependencies from git or from a folder work without the index", ""),
        ("no package `{name}` in the package index", ""),
        ("`rythme search WORDS` finds packages by the words of their name and description", ""),
        ("the index: {place}", ""),
        ("{url} (copy of {date})", ""),
        ("{} (withdrawn)", ""),
        ("none", ""),
        ("no version of `{name}` in the package index fits `{range}`", ""),
        ("its versions: {}", ""),
        ("for the newest one, write `{name} = {{ version = \"^{v}\" }}`", ""),
        ("every version of `{name}` was withdrawn", ""),
        ("`{shown}` is too large for an entry of the package index", ""),
        ("`{key}` is a list of texts", ""),
        ("this entry of the package index is for `{found}`, but its file is named for `{name}`", ""),
        ("the entry of `{name}` has no `repository`", ""),
        ("the entry of `{name}` has no `{key}`", ""),
        ("each version is written under `[[version]]`", ""),
        ("this version of `{name}` has no `{k}`", ""),
        ("`{commit}` is not a full commit: 40 hexadecimal digits in lower case", ""),
        ("`{checksum}` is not a checksum: `sha256:` and 64 hexadecimal digits, as `rythme publish` writes it", ""),
        ("`{t}` is not a tag", ""),
        ("the version {v} is written twice", ""),
        ("the entry of `{name}` has no version", ""),
        ("expected `,` or `]` here", ""),
        ("a list is written on one line: `[\"math\", \"vectors\"]`", ""),
        ("like `{key} = {example}`", ""),
        ("write `\"^1.2.0\"` (1.2.0 or a newer compatible version, below 2.0.0), `\"~1.2.0\"` (1.2.x), `\"=1.2.0\"` (exactly this one) or `\">=1.2.0, <1.5.0\"`", ""),
        ("for a version from the package index, write `{name} = {{ version = \"{v}\" }}`", ""),
        ("a dependency comes from the package index (`version`), from git (`git`) or from a folder (`path`): give only one", ""),
        ("`{v}` is not a range of versions", ""),
        (
            r#"unknown option `{bad}` for `rythme {command}`
see `rythme help packages`"#,
            "",
        ),
        ("{bad} file(s) of the package index could not be read", ""),
        ("{n} package(s) in {place}", ""),
        ("No package found for `{query}` in {place}", ""),
        ("{n} package(s) found for `{query}` in {place}", ""),
        ("More about one: `rythme info {name}`; add it to a project: `rythme add {name}`", ""),
        ("`rythme search --update{words}` searches the newest copy of the index", ""),
        ("which package? Example: rythme info mathlib", ""),
        ("repository", ""),
        ("license", ""),
        ("keywords", ""),
        ("authors", ""),
        ("versions", ""),
        ("  {v} was withdrawn: {}", ""),
        ("Add it to a project: `rythme add {name}` (writes `{name} = {{ version = \"^{v}\" }}` in rythme.toml)", ""),
        ("Every version of this package was withdrawn.", ""),
        ("which package? Example: rythme add mathlib (or rythme add mathlib@1.2)", ""),
        ("cannot read `{}`: {e}", ""),
        ("`{range_text}` is not a range of versions", ""),
        ("`{name}` came from {from}: it now comes from the package index", ""),
        ("`rythme.toml` was left as it was", ""),
        ("Added `{line}` to rythme.toml ({version})", ""),
        ("  use it with `use {name}/FILE` (see `rythme info {name}`)", ""),
        ("which entries? Example: rythme publish --verify packages/mathlib.toml", ""),
        (
            r#"`rythme publish` takes no argument (`{a}`): it checks the package of the current folder
see `rythme help packages`"#,
            "",
        ),
        ("`rythme.toml` has no `{key}`", ""),
        ("the package index shows it, and needs it", ""),
        ("add a line like `{example}` at the top", ""),
        ("Checking `{name}` {version} before it goes into the package index", ""),
        ("`{version}` is not a version of semantic versioning", ""),
        ("like `version = \"1.2.0\"`, without zeros at the start of a number", ""),
        ("description = \"What `{name}` is for, in one sentence\"", ""),
        ("the `description` is too long: one sentence, at most 200 characters", ""),
        ("YOU", ""),
        ("`{k}` is not a keyword: a word in lower case, at most 24 characters", ""),
        ("at most 5 keywords", ""),
        ("a published package cannot use a folder of this computer: `{dep_name}` comes from `{p}`", ""),
        ("publish `{dep_name}` too, then write `{dep_name} = {{ version = \"^...\" }}`", ""),
        ("`{name}` is not in a git repository", ""),
        ("the package index gives the commit of each version: a package is published from its git repository", ""),
        ("`git init`, `git add .`, `git commit`, then push it to its `repository`", ""),
        ("`rythme.toml` must be at the root of its git repository", ""),
        ("the root of the repository: {top}", ""),
        ("give the package a repository of its own", ""),
        ("some changes are not committed", ""),
        ("for example: {}", ""),
        ("commit them (`git add`, `git commit`), or set them aside, then tag the commit", ""),
        ("no tag `v{version}` for the version {version}", ""),
        ("tag the commit and push the tag: `git tag v{version}`, then `git push origin v{version}`", ""),
        ("the tag `{t}` is not the current commit", ""),
        ("what is checked must be what is published", ""),
        ("publish from the tag (`git checkout {t}`), or move the tag to this commit (`git tag --force {t}`) before pushing it", ""),
        ("Running `rythme check`", ""),
        ("`rythme check` found errors", ""),
        ("fix them first: a package must pass `rythme check`", ""),
        ("Running `rythme test`", ""),
        ("`rythme test` failed", ""),
        ("fix them first: a package must pass its tests", ""),
        ("the name `{name}` is already taken in the package index", ""),
        ("by the repository {repo}", ""),
        ("choose another `name` for your package", ""),
        ("the version {version} of `{name}` is already in the package index", ""),
        ("a published version never changes: raise `version` in `rythme.toml`, commit and tag again", ""),
        ("cannot read the package index `{url}`: it will check the name and the version itself", ""),
        ("the tag `{t}` is not in {repo}", ""),
        ("push it: `git push origin {t}`", ""),
        ("the tag `{t}` of {repo} is not this commit", ""),
        ("push the tag again: `git push --force origin {t}`", ""),
        ("cannot reach {repo} to check that the tag `{t}` is there", ""),
        ("cannot read the files of the commit `{commit}`", ""),
        ("✓ `{name}` {version} is ready for the package index.", ""),
        ("Rythme never publishes anything by itself: a version goes into the index with a pull request on its repository, {}.", ""),
        ("1. Fork the repository of the index and clone your fork. Create the file {file} with:", ""),
        ("1. Fork the repository of the index and clone your fork. Add at the end of {file}:", ""),
        ("2. Commit it on a branch of your fork and push it:", ""),
        ("3. Open a pull request on {}: its checks run `rythme publish --verify {file}`, then a maintainer merges it.", ""),
        (
            r#"
{problems} problem(s) to fix before publishing: nothing was sent anywhere"#,
            "",
        ),
        ("`{f}` is not an entry of the index: `packages/NAME.toml`", ""),
        ("✓ {f}: `{stem}`, {count} version(s) verified", ""),
        ("✗ {f}", ""),
        ("{v}: the commit `{commit}` is not in `{url}`", ""),
        ("{v}: the tag `{tag}` of `{url}` is not the commit `{commit}`", ""),
        ("{v}: the files of the commit do not have the checksum of the entry", ""),
        (
            r#"expected: {}
found:    {found}"#,
            "",
        ),
        ("`rythme publish` in the package, at its tag, writes the right one", ""),
        ("{v}: the commit has no `rythme.toml` at its root", ""),
        ("{v}: the `rythme.toml` of the commit says `{found_name}` {found_version}", ""),
        ("{v}: the dependency `{dep_name}` is a folder of the computer of the author", ""),
        ("the package index, version {range}", ""),
        ("the commit `{commit}` of `{name}` {version} does not exist in `{url}`", ""),
        ("the package index gives this commit for this version", ""),
        ("tell the authors of `{name}`: the history of their repository may have been rewritten", ""),
        ("cannot read the files of `{name}` {version}", ""),
        ("the files of `{name}` {version} do not match the checksum of the package index", ""),
        ("nothing was installed, and nothing of the package ran", ""),
        ("do not use this version; tell the maintainers of the package index", ""),
        ("Updating the dependencies of `{name}`", ""),
        ("{range} from the index, commit {short} ({how})", ""),
        ("the package index gives `{dep_name}` {wanted}, but its `rythme.toml` says `{found}` {version}", ""),
        ("tell the maintainers of the package index", ""),
        ("`rythme update` takes the names of dependencies, not `{bad}`: `rythme update` (all of them) or `rythme update mathlib`", ""),
        ("`{a}` is not a dependency of `{}`", ""),
        ("did you mean `rythme update {best}`?", ""),
        ("`rythme add {a}` adds it from the package index", ""),
        ("Everything was already up to date", ""),
        ("Updated:", ""),
        ("{} {} (new)", ""),
        (
            r#"THE PACKAGE INDEX

  rythme search vectors      # the packages whose name, description or keywords say it
  rythme info mathlib        # its description, repository, license, versions
  rythme add mathlib         # adds `mathlib = { version = "^1.2.0" }` to rythme.toml,
                             #   then installs it (rythme.lock keeps the exact commit)
  rythme add mathlib@1.1     # a range of versions of your choice
  rythme update              # the newest version in the range of each dependency
  rythme publish             # checks your package, then says how to add it

A dependency from the index gives a range of versions:

  [dependencies]
  mathlib = { version = "^1.2.0" }    # 1.2.0 or newer, below 2.0.0 (the default)
  geometry = { version = "~0.3.1" }   # 0.3.x, from 0.3.1
  tools = { version = "=1.0.0" }      # exactly this one

`rythme install` takes the version of rythme.lock while it is in the range,
and `rythme update` the newest one; a withdrawn version is never chosen.

The index is a git repository, one file per package (packages/NAME.toml):
its name, description, repository, license, keywords, and for each version
its tag, its commit and the checksum of its files. Rythme keeps a copy in
~/.rythme/index and reads it without a network: `rythme search --update`
gets the newest one. Another index: `index = "..."` in rythme.toml, or the
variable RYTHME_INDEX (an address, or a folder of this computer); by
default https://github.com/maxlestage/rythme-index.

Safety: names are Rythme names (no `/`, no `..`), addresses use https, ssh
or file://, git runs without hooks, the files of each version must have the
checksum of the index, and nothing of a package runs when it is installed.

Publishing: tag the version (git tag v1.2.0, git push origin v1.2.0), then
`rythme publish` checks rythme.toml (description, license, repository),
`rythme check`, `rythme test` and the tag, and prints the entry to add to
the index with a pull request. Rythme never sends anything by itself."#,
            "",
        ),
        ("The index is the folder {url}, where Rythme writes nothing.", ""),
        ("Create its file {file} with:", ""),
        ("Add at the end of its file {file}:", ""),
        ("the pattern \"{pattern}\" has an unknown directive `%{c}`", ""),
        ("the directives are %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second), %y (year in 2 digits), %B and %b (month name), %A and %a (day name), %I and %p (12-hour clock), %j (day of the year), %z (offset) and %% (a %)", ""),
        ("\"{text}\" does not follow the pattern \"{pattern}\"", ""),
        ("the hour must go from 1 to 12 with AM or PM, not {}", ""),
        ("the day of the year must go from 1 to {len}, not {n}", ""),
        ("`{c}` is not a Base64 character", ""),
        ("this Base64 text is cut short", ""),
        ("the decoded bytes are not a text (UTF-8)", ""),
        ("a `%` must be followed by two hexadecimal digits, like `%20`", ""),
        ("`{f}` would give {count} lists: too many (at most {MAX_COMBINED} elements in all)", ""),
        ("the number of combinations grows very fast: try with a shorter list, or fewer elements in each one", ""),
        ("`unzip` needs a list of lists, but its element {k} is {}", ""),
        ("the function given to `unfold` must return `Some([value, next])` or `None`, but it returned Some({})", ""),
        ("`{f}` needs at least two numbers in each list", ""),
        ("`correlation` needs numbers that vary: one of the lists has all its numbers equal", ""),
        ("`{f}` cannot compare {} and {}", ""),
        ("the value and its bounds must be of the same kind: all numbers, or all texts...", ""),
        ("`constrain`: the lower bound {} is above the upper bound {}", ""),
        ("`binomial` needs a number of 0 or more, not {n}", ""),
        ("`expect` found `None`: there is no value", ""),
        ("`expect` found an error: {}", ""),
        ("`all_ok` needs a list of Results (Ok or Err), but its element {k} is {}", ""),
        ("to keep the values of a list of Options, use `filter_map(list, fn(x) = x)`", ""),
        ("`format` cannot use \"{spec}\": {why}", ""),
        ("a spec is written [[fill]align][sign][#][0][width][,][.precision][type]: \">8\" (8 wide, on the right), \"^8\" (centered), \"08.2f\" (2 decimals, zeros before), \",d\" (thousands), \".1%\" (a percentage), \"x\" (hexadecimal), \"b\" (binary), \"e\" (exponent)", ""),
        ("the width is at most {MAX_WIDTH}", ""),
        ("the precision is at most {MAX_PRECISION}", ""),
        ("a number must follow the `.`", ""),
        ("`{c}` is not a type of format", ""),
        ("this is not a format spec", ""),
        ("`{kind}` cannot write {what}", ""),
        ("a whole number", ""),
        ("a whole number has no decimals: write a type like `f` for them", ""),
        ("`,` groups decimal digits only: use `_`", ""),
        ("a decimal number", ""),
        ("signs, `#`, `0` and `,` are for numbers", ""),
        ("`{f}`: the base must go from 2 to 36, not {b}", ""),
        ("\"{s}\" is not a whole number in base {radix}", ""),
        ("`digits` needs a number of 0 or more, not {}", ""),
        ("`{f}`: the text to look for is empty", ""),
        ("`{f}` cannot replace an empty text", ""),
        ("`edit_distance` compares texts of at most {MAX_DISTANCE_WORK} pairs of characters, not {} by {}", ""),
        ("to count a piece of text in a text, use `occurrences(text, piece)`", ""),
        ("to count the elements equal to a value, use `occurrences(list, x)`", ""),
        ("the value comes first, then the spec: `format(42, \"05\")`; to put values in a text, use braces", ""),
        ("return `Some(x)` to keep x, or `None` to leave the element out", ""),
        ("return `Some([value, next])` to go on, or `None` to stop", ""),
        ("Pairs the elements of two lists (or more: one element of each list in every group): [[a1, b1], [a2, b2], ...]. It stops at the end of the shortest list.", ""),
        ("Reads a date written as ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", with `Z` or an offset like `+01:00` if needed): `Ok(date)`, or `Err(message)` if it is not a valid date. With a pattern like \"%d/%m/%Y\" (the directives of `format_date`), reads a date written that way.", ""),
        ("Writes a date as ISO 8601: \"2024-02-29\", with the time if the date has one, and `Z` or the offset if it has one. With a pattern, writes it the way `strftime` does: %Y year, %m month, %d day, %H hour, %M minute, %S second, %y year in two digits, %B and %b the month, %A and %a the day of the week (in English), %I and %p the 12-hour clock, %j the day of the year, %z the offset, %% a %.", ""),
        ("The text that shows a value the way the REPL and `debug` show it: a text between quotes, with its special characters written as escapes. `text` gives a text as it is.", ""),
        ("Shows values on the error output (stderr), separated by spaces, then goes to the next line: for messages that must not mix with the results of a program, which go to the standard output.", ""),
        ("The value kept between two bounds: `low` if it is below, `high` if it is above, itself otherwise. For numbers, texts... any values that can be compared.", ""),
        ("The whole quotient and the remainder of a division at once: `[a // b, a % b]`.", ""),
        ("The binomial coefficient \"n choose k\": the number of ways to choose k elements among n, exact even when it is very large.", ""),
        ("The digits of a whole number (0 or more), the most significant first, in base 10 or in the base given (from 2 to 36).", ""),
        ("Writes a whole number in a base from 2 to 36, with the digits 0 to 9 then the letters a to z.", ""),
        ("Reads a whole number written in a base from 2 to 36: `Ok(number)`, or `Err(message)` if the text is not a number in this base. `0x`, `0o` and `0b` may come before a number in base 16, 8 and 2.", ""),
        ("Is the number greater than 0?", ""),
        ("Is the number less than 0?", ""),
        ("Is the number 0 (or 0.0)?", ""),
        ("Is the decimal number `nan` (\"not a number\")? It is never equal to anything, not even to itself.", ""),
        ("Is the number finite: neither an infinity nor `nan`? Whole numbers always are.", ""),
        ("Adds `fill` (a space by default) on both sides of the text until it is `width` characters long: titles and cells of tables stay centered. A character left over goes to the right.", ""),
        ("Writes a value following a format spec, like Python's `format`: \".2f\" for 2 decimals, \">8\" to fill 8 characters on the right (\"<\" on the left, \"^\" centered), \"08.2f\" with zeros before, \",\" with the thousands separated, \"%\" as a percentage, \"x\", \"o\" and \"b\" in hexadecimal, octal and binary (\"#\" adds 0x, 0o, 0b), \"e\" with an exponent. To put values in a text, use braces.", ""),
        ("Replaces the first occurrence of `old` by `new`, and only that one.", ""),
        ("How many times an element is in a list, or a piece of text in a text (pieces that do not overlap).", ""),
        ("The position of the last occurrence of an element in a list, or of a piece of text in a text: `Some(position)`, or `None` if it is not there.", ""),
        ("The text without this start, if it starts with it; otherwise the text unchanged.", ""),
        ("The text without this end, if it ends with it; otherwise the text unchanged.", ""),
        ("The text with the first character of each word in uppercase (the rest is unchanged). Words are separated by spaces.", ""),
        ("Writes a name in snake_case: its words in lowercase, joined by `_`. Reads names written in any style: camelCase, PascalCase, kebab-case, with spaces...", ""),
        ("Writes a name in camelCase: its words glued together, each with an uppercase first letter but the first word.", ""),
        ("Writes a name in kebab-case: its words in lowercase, joined by `-`, as in web addresses.", ""),
        ("The text with its uppercase letters in lowercase, and its lowercase letters in uppercase.", ""),
        ("Has the text letters, all of them in uppercase? (The other characters do not count.)", ""),
        ("Has the text letters, all of them in lowercase? (The other characters do not count.)", ""),
        ("The bytes of a text in UTF-8, as whole numbers from 0 to 255: an accented letter takes two of them, an emoji four.", ""),
        ("Cuts a text in two at the first separator: `Some([before, after])`, or `None` if the separator is not there. For lines like `key=value`.", ""),
        ("Cuts the lines of a text between words so that none is longer than `width` characters (a longer word stays whole on its own line).", ""),
        ("The edit distance (Levenshtein) between two texts: the fewest characters to add, remove or replace to go from one to the other. For spelling checkers, and to find the closest word.", ""),
        ("The position of the first element for which f is true: `Some(position)`, or `None` if there is none.", ""),
        ("The list with the element at position i replaced by f(element). A negative position counts from the end.", ""),
        ("The list or text turned by n places to the left (1 by default): the first elements go to the end. A negative n turns it to the right.", ""),
        ("The list with `x` between every two elements.", ""),
        ("The elements of two lists taken in turn, one from each; what is left of the longer list comes at the end.", ""),
        ("Counts how many times each value is in a list: a dict from each value to its count (like Python's `Counter`).", ""),
        ("Counts the elements by the key that f gives them: a dict from each key to how many elements have it.", ""),
        ("The values that come most often, with their counts: a list of `[value, count]` pairs, the most frequent first (on a tie, the one seen first). With n, only the first n.", ""),
        ("The list without the elements whose key (given by f) was already seen: the first element of each key stays.", ""),
        ("The list sorted from the largest to the smallest.", ""),
        ("The list sorted by the key that the function gives, from the largest key to the smallest; elements with the same key keep their order.", ""),
        ("Cuts a list into runs of neighbour elements that have the same key (given by f).", ""),
        ("Cuts a list or a text in two at position i: `[before, from i on]`. A negative position counts from the end.", ""),
        ("Every way to choose k elements of the list, each kept in the order of the list.", ""),
        ("Every order of the elements of the list, or of k of them.", ""),
        ("Every way to take one element of each list: the cartesian product.", ""),
        ("The opposite of `zip`: a list of pairs (or of lists of the same length) becomes the list of the first elements, the list of the second ones...", ""),
        ("A list of n values: `start`, then f(start), then f of that... For sequences where each value comes from the one before.", ""),
        ("Builds a list from a starting state: f receives the state and gives `Some([value, next state])` to add a value and go on, or `None` to stop.", ""),
        ("Applies f to each element and keeps the values of the `Some(...)` it gives, leaving out the `None`: filtering and transforming at once.", ""),
        ("The value found by following a path of keys into records, dicts and lists nested in one another (names of fields, keys, positions): `Some(value)`, or `None` as soon as one is missing. For data read with `parse_json`.", ""),
        ("Merges two records or two dicts; for a key in both, f receives the two values and gives the one kept.", ""),
        ("A dict where the values of a record or a dict have become the keys, and the keys the values. When a value comes several times, the last key wins.", ""),
        ("The record or dict with only the keys given (in a list or a set); a key that is missing is ignored.", ""),
        ("The record or dict without the keys given (in a list or a set).", ""),
        ("The record or dict with each key replaced by f(key); for a record, f must give a text.", ""),
        ("The record or dict where the value of `key` is replaced by f(value); when the key is missing, it is added with `default`. Handy to count: `update(counts, word, 1, fn(n) = n + 1)`.", ""),
        ("The elements that are in exactly one of the two sets.", ""),
        ("Have the two sets no element in common?", ""),
        ("The value inside `Some(x)` or `Ok(x)`; otherwise what f gives: f receives nothing for `None`, and the error for `Err(e)`. The default is computed only when it is needed.", ""),
        ("The value inside `Some(x)` or `Ok(x)`, like `unwrap`; otherwise stops the program with your message, which says what was expected.", ""),
        ("Makes one Result of a list of Results: `Ok(list of the values)` if they are all `Ok`, or else the first `Err`. When reading many values where one mistake must stop everything.", ""),
        ("The date n months later (earlier if n is negative). A day that the new month does not have becomes its last day: 31 January plus one month is the end of February.", ""),
        ("The number of the day in its year, from 1 (1 January) to 365, or 366 in a leap year.", ""),
        ("n decimal numbers evenly spaced from `start` to `end`, both included: to draw a function with `plot`, or to try values.", ""),
        ("The correlation (Pearson) of two lists of numbers of the same length: from -1 (they vary in opposite ways) to 1 (they vary together); about 0 when they are not linearly related.", ""),
        ("The covariance of two lists of numbers of the same length, divided by their length like `variance`.", ""),
        ("Encodes a text in Base64 (its UTF-8 bytes), as in emails, web pages and the headers of HTTP requests.", ""),
        ("Decodes a Base64 text: `Ok(text)`, or `Err(message)` if it is not valid Base64, or not a text once decoded.", ""),
        ("Encodes a text to put it in a web address: every character but the letters, the digits and `-_.~` becomes `%` and two hexadecimal digits (a space is `%20`).", ""),
        ("Decodes a text of a web address (`%20`...): `Ok(text)`, or `Err(message)` if a `%` is not followed by two hexadecimal digits.", ""),
        (
            r#"More functions on lists
  find_index([3, 8, 5], is_even)          # Some(1)
  sort_desc([3, 1, 2])                    # [3, 2, 1]  (sort_by_desc with a key)
  tally(["a", "b", "a"])                  # dict("a" => 2, "b" => 1)
  most_common(words, 3)                   # the 3 most frequent, with their counts
  split_at([1, 2, 3], 1)                  # [[1], [2, 3]]
  rotate_by([1, 2, 3], 1)                 # [2, 3, 1]
  combinations([1, 2, 3], 2)              # [[1, 2], [1, 3], [2, 3]]
  iterate(1, 5, fn(x) = x * 2)            # [1, 2, 4, 8, 16]
  permutations(xs)   cartesian_product(xs, ys)   unzip(pairs)   zip(xs, ys, zs)
  filter_map(xs, f)   chunk_by(xs, f)   unique_by(xs, f)   count_by(xs, f)"#,
            "",
        ),
        (
            r##"More functions on texts
  format(3.14159, ".2f")         # "3.14"  (the format specs of Python: ">8", ",d", ".1%", "#x"...)
  center("hi", 6, "*")           # "**hi**"
  replace_first("a-b-c", "-", "+")          # "a+b-c"
  occurrences("banana", "an")    # 2
  remove_prefix("Mr Smith", "Mr ")          # "Smith"  (and remove_suffix)
  split_once("key=value", "=")   # Some(["key", "value"])
  snake_case("helloWorld")       # "hello_world"  (and camel_case, kebab_case)
  capitalize_words("ada lovelace")          # "Ada Lovelace"
  word_wrap(text, 40)            # lines of at most 40 characters
  to_base(255, 16)   from_base("ff", 16)    # "ff" and Ok(255)
  edit_distance("kitten", "sitting")        # 3"##,
            "",
        ),
        (
            r#"More functions on records and dicts
  dig(data, ["user", "langs", 0])           # Some(...), or None: deep inside
  select_keys(person, ["name", "age"])      # only these keys (reject_keys: without them)
  merge_with(a, b, fn(x, y) = x + y)        # f decides for a key in both
  update(counts, "apples", 1, fn(n) = n + 1)  # +1, or 1 when missing
  invert({a: 1, b: 2})                      # dict(1 => "a", 2 => "b")
  map_keys(r, upper)   symmetric_difference(s, t)   is_disjoint(s, t)"#,
            "",
        ),
        (
            r#"More helpers
  unwrap_or_else(x, fn(e) = ...)   # the default, computed only when it is needed
  expect(x, "a config file")       # like unwrap, with your message if it fails
  all_ok(map(texts, parse_int))    # Ok([...]), or the first Err
  filter_map(xs, f)                # the values of the Some(...) that f gives"#,
            "",
        ),
        (
            r#"More on dates
  add_months(date, 1)                       # 31 January + 1 month: the end of February
  day_of_year(date)                         # from 1 to 366
  format_date(date, "%d/%m/%Y %H:%M")       # with a pattern: %Y %m %d %H %M %S %B %A...
  parse_date("29/02/2024", "%d/%m/%Y")      # Ok({year: 2024, month: 2, day: 29})"#,
            "",
        ),
        // src/cli.rs
        ("run a program; the words after the file are its arguments", ""),
        ("find problems without running the program", ""),
        ("run the tests of a file or of a folder", ""),
        ("give the code its official layout", ""),
        ("write the documentation of a file from its ## comments", ""),
        ("apply the safe fixes of the problems that `rythme check` finds", ""),
        ("show a calculation step by step", ""),
        ("pause a program where you want and look at its values", ""),
        ("the debugger for editors (Debug Adapter Protocol)", ""),
        ("exercises checked automatically", ""),
        ("create a project (rythme.toml, src/, tests/)", ""),
        ("download the dependencies of the project", ""),
        ("guided tour of the language, step by step", ""),
        ("help on a topic, a function or an error code", ""),
        ("explain an error or warning code, like E003", ""),
        ("run a piece of code given on the command line", ""),
        ("interactive mode: try code line by line (also `rythme` alone)", ""),
        ("language server, for editors (VS Code, Neovim...)", ""),
        ("server for AI assistants (Model Context Protocol)", ""),
        ("the version of Rythme", ""),
        ("completion of the commands in a shell", ""),
        ("the manual page of the command, to install with `man`", ""),
        ("how to install the latest version of Rythme", ""),
        (
            r#"  --seed N        the same draws of random and shuffle on every run
  --sandbox       no files, folders, environment variables nor network
  --format json   the output and the errors as JSON, for tools
  FILE may be `-`: the program is read from the standard input.
  At the root of a project, `rythme run` runs its program."#,
            "",
        ),
        (
            r#"  --format json   the problems as JSON, for tools and AI assistants
  A folder checks every .ry file inside it; at the root of a project,
  `rythme check` checks the files of the project.
  Exit code: 0 without errors (warnings allowed), 1 with errors."#,
            "",
        ),
        (
            r#"  --filter NAME   only the tests whose name contains NAME
  --seed N        the same draws of random on every run
  --watch         run the tests again whenever a file changes
  --sandbox       no files, folders, environment variables nor network
  --format json   the results as JSON
  Exit code: 0 when every test passes, 1 otherwise."#,
            "",
        ),
        (
            r#"  --check         only check, for continuous integration: exit code 1 when a
                  file must be formatted
  A folder formats all its .ry files; `-` reads the code from the standard
  input and writes it formatted."#,
            "",
        ),
        (
            r#"  --dry-run       only show the changes, as a diff
  --all           also apply the fixes that are only likely
  --format json   the fixes as JSON
  Exit code: 0 when no error is left, 1 otherwise (with --dry-run: 1 when
  something would change)."#,
            "",
        ),
        (
            r#"  --max N         show at most N steps
  --width N       cut the lines at N characters (20 at least)
  Without an expression, the steps of the last line of the file."#,
            "",
        ),
        (
            r#"  In the debugger, `help` lists its commands: break, run, next, step,
  finish, where, print, locals, quit."#,
            "",
        ),
        ("  Your editor starts it: the VS Code extension does, for the debugger type `rythme`.", ""),
        (
            r#"  list            the exercises, from the easiest
  start NAME      write the file of an exercise, to complete
  check FILE.ry   run its tests and give hints"#,
            "",
        ),
        (
            r#"  `rythme help` lists the topics, `rythme help builtins` every function,
  `rythme help llms` the whole language in one text (for AI assistants)."#,
            "",
        ),
        ("  Without a code, the list of the codes; --format json gives them all.", ""),
        ("  The value of the last expression is shown, unless it is ().", ""),
        ("  `:help` lists the commands of the interactive mode.", ""),
        ("  --verbose       also the system, the languages, the functions and the formats", ""),
        ("Then open a new shell.", ""),
        (
            r#"  rythme man > rythme.1 && man ./rythme.1
  To install it: put rythme.1 in a folder of the manual pages, like
  ~/.local/share/man/man1."#,
            "",
        ),
        (
            r#"
Options of every command: --lang CODE ({codes}), --no-color
"#,
            "",
        ),
        (
            r#"See also `rythme help {topic}`.
"#,
            "",
        ),
        (
            r#"
Tools
  rythme CMD --help         the options of a command, like rythme test --help
  rythme run -              run the program read from the standard input
  rythme check FOLDER       check every .ry file of a folder
  rythme completions SHELL  completion of the commands: bash, zsh, fish, powershell
  rythme man                the manual page of the command (rythme man > rythme.1)
  rythme version --verbose  the version, with the system and what Rythme contains
  rythme upgrade            how to install the latest version
"#,
            "",
        ),
        ("unknown option `{bad}` for `rythme version`: the option is --verbose", ""),
        (
            r#"system: {os} {arch}
languages of the messages: {codes} (now: {now})
built-in functions: {functions} (and {ruby} Ruby names)
codes: {errors} errors ({e_range}), {warnings} warnings ({w_range})
JSON output (--format json): version {json}
program: {exe}
"#,
            "",
        ),
        ("`rythme upgrade` takes no argument, not `{bad}`", ""),
        (
            r#"This is rythme {v} ({exe}).
To install the latest version, type:
  {how}
The versions and what changed: https://github.com/maxlestage/Rythme/releases
Within 1.x, a program that runs keeps running, with the same output.
"#,
            "",
        ),
        ("unknown shell `{s}`", ""),
        ("which shell?", ""),
        (
            r#"{what} Write one of bash, zsh, fish, powershell, like `rythme completions bash`
see `rythme completions --help`"#,
            "",
        ),
        ("`rythme man` takes no argument, not `{bad}`", ""),
        ("Name", ""),
        ("rythme - a small, purely functional language with help everywhere", ""),
        ("Synopsis", ""),
        ("Description", ""),
        ("Rythme runs, checks, tests and formats programs written in Rythme. Without a command, `rythme` starts the interactive mode; `rythme file.ry` runs a program. Every message has a code explained by `rythme explain CODE`, and `rythme help` shows help on the language and its functions.", ""),
        ("Commands", ""),
        ("Options", ""),
        ("the language of the messages: {codes}", ""),
        ("messages without colors", ""),
        ("the help; after a command, its options", ""),
        ("Environment", ""),
        ("the language of the messages, before LC_ALL, LC_MESSAGES and LANG", ""),
        ("when set, messages without colors", ""),
        ("Exit status", ""),
        ("0 when everything went well; 1 after an error in the program, a failed test or a problem found; 2 for a command written wrongly. A program may choose its own code with exit(code).", ""),
        ("See also", ""),
        ("no .ry file in `{a}`", ""),
        ("the standard input cannot be read: {e}", ""),
        // src/cli.rs: the commands of the package index
        ("search the package index", ""),
        ("what the index says of a package: its versions, repository and license", ""),
        ("add a package of the index to the project, then install it", ""),
        ("take the newest versions that the ranges of rythme.toml allow", ""),
        ("check a package and print its entry for the index (sends nothing)", ""),
        (
            r#"  --update        download the newest copy of the index first
  --format json   the packages found as JSON
  Without words, every package of the index."#,
            "",
        ),
        (
            r#"  --update        download the newest copy of the index first
  --format json   the entry of the package as JSON"#,
            "",
        ),
        (
            r#"  NAME@RANGE      a range of versions, like mathlib@1.2; without a range,
                  the newest version
  rythme.toml gets the range, rythme.lock the exact commit and its checksum."#,
            "",
        ),
        (
            r#"  Without names, every dependency. rythme.toml does not change;
  rythme.lock gets the new versions."#,
            "",
        ),
        (
            r#"  Checks rythme.toml, the tag of the version (like v1.2.0), rythme check
  and rythme test, then prints the entry and the steps of a pull request.
  --verify FILE...  the index's own check of its entries (packages/NAME.toml)"#,
            "",
        ),
    ],
}
