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
        // src/builtins/numbers.rs
        ("`acosh` needs a number of 1 or more, not {x}", "`acosh` には 1 以上の数が必要（{x} は不可）"),
        ("`atanh` needs a number between -1 and 1 (both excluded), not {x}", "`atanh` には -1 と 1 の間（両端を除く）の数が必要（{x} は不可）"),
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
        ("`sleep` needs a number of seconds of 0 or more, not {secs}", "`sleep` には 0 以上の秒数が必要（{secs} は不可）"),
        ("`sleep({secs})` would wait too long", "`sleep({secs})` では待ち時間が長すぎる"),
        // src/builtins/text_tools.rs
        ("`{f}` writes at most 100 decimals, not {d}", "`{f}` が書ける小数は最大 100 桁（{d} は不可）"),
        ("`{f}` does not know the language \"{l}\"", "`{f}` は言語 \"{l}\" を知らない"),
        ("the languages: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" and \"pt\" (1.234,5)", "使える言語：\"en\"（1,234.5）、\"fr\"（1 234,5）、\"de\"、\"es\"、\"it\"、\"nl\"、\"pt\"（1.234,5）"),
        // src/builtins/web.rs
        ("a web address cannot contain spaces, tabs or line breaks: write a space `%20`", "Web アドレスには空白、タブ、改行を入れられない：空白は `%20` と書く"),
        ("this web address does not start with its scheme: write it like `https://example.com/page`", "この Web アドレスはスキームで始まっていない：`https://example.com/page` のように書く"),
        ("the host `{host_port}` opens a `[` that no `]` closes", "ホスト `{host_port}` で開いた `[` を閉じる `]` がない"),
        ("the host `{host}` cannot contain `{c}`", "ホスト `{host}` には `{c}` を入れられない"),
        ("an address `{scheme}:` names a host: write it like `{scheme}://example.com/page`", "`{scheme}:` のアドレスにはホストが必要：`{scheme}://example.com/page` のように書く"),
        ("the port `{port}` is not a number from 0 to 65535", "ポート `{port}` が 0 から 65535 までの数ではない"),
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
        ("this is a task, the call of an `async fn`: write `await` before it to use its result, like `await total(x) + 1`", "これはタスク（`async fn` の呼び出し）：結果を使うには前に `await` を書く。例：`await total(x) + 1`"),
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
  javascript  coming from JavaScript: its standard library in Rythme
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
  javascript  JavaScript から来た人へ：その標準ライブラリを Rythme で
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
        ("an `async fn` can only be defined at the top level of a file", "`async fn` はファイルの最上位でしか定義できない"),
        ("move it outside of the function or block: its task runs on its own, apart from the values around it", "関数やブロックの外に移す：そのタスクは周りの値から切り離されて単独で動く"),
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
        ("to use another file, write `use other_file`, then `other_file.name`", "別のファイルを使うには `use other_file` と書き、`other_file.name` とする"),
        ("there is no `new`: build values directly, like `{{name: \"Ada\"}}` or `Circle(2.0)`", "`new` はない：`{{name: \"Ada\"}}` や `Circle(2.0)` のように値を直接作る"),
        ("values never change in Rythme: build a new value instead", "Rythme では値は変わらない：代わりに新しい値を作る"),
        (
            r#"Rythme has no loops. Work on the whole list instead:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
or write a recursive function. See `rythme help functional`."#,
            r#"Rythme にループはない。代わりにリスト全体を処理する：
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
または再帰関数を書く。`rythme help functional` を参照。"#,
        ),
        ("Rythme has no loops to interrupt: use `find`, `take`, `filter`, or a recursive function", "Rythme には中断するループがない：`find`、`take`、`filter` か再帰関数を使う"),
        (
            r#"no `return` needed: the last expression of a function is its result.
To stop early with an error, return `Err(...)` and use `?`."#,
            r#"`return` は要らない：関数の最後の式がその結果になる。
エラーで早めに止めるには `Err(...)` を返し、`?` を使う。"#,
        ),
        ("just write `name = value` (values never change, so no keyword is needed)", "`name = value` と書くだけでよい（値は変わらないので、キーワードは要らない）"),
        ("functions are written `fn name(x) = x * 2`, and anonymous ones `fn(x) = x * 2`", "関数は `fn name(x) = x * 2`、無名関数は `fn(x) = x * 2` と書く"),
        (
            r#"Rythme has no classes. Describe your data with `type`:
  type Shape = Circle(radius) | Square(side)
or use a record: {{name: "Ada", age: 36}}"#,
            r#"Rythme にクラスはない。データは `type` で表す：
  type Shape = Circle(radius) | Square(side)
またはレコードを使う：{{name: "Ada", age: 36}}"#,
        ),
        (
            r#"use `match`:
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"`match` を使う：
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
        ),
        (
            r#"errors are values in Rythme: return `Ok(value)` or `Err(message)`,
then handle them with `match` or pass them on with `?`. `fail(message)` stops the program."#,
            r#"Rythme ではエラーは値：`Ok(value)` か `Err(message)` を返し、
`match` で扱うか `?` で呼び出し元に渡す。`fail(message)` はプログラムを止める。"#,
        ),
        // src/tasks.rs
        ("`{f}` expects a Task, the result of a call to an `async fn`, not {shown}", "`{f}` には Task（`async fn` を呼び出した結果）が必要で、{shown} ではない"),
        ("define the function with `async fn`, then `await` what its call gives", "関数を `async fn` で定義し、その呼び出しが返すものに `await` を使う"),
        ("the task stopped without a result", "タスクが結果を返さずに停止した"),
        ("this task was already awaited", "このタスクはすでに await された"),
        ("keep the result of `await` in a name and use that name again", "`await` の結果に名前を付けて、その名前をもう一度使う"),
        ("in the task awaited here", "ここで await したタスクの中"),
        ("`await_all` expects a list of tasks, not {shown}", "`await_all` にはタスクのリストが必要で、{shown} ではない"),
        ("the program could not be loaded to run the task", "タスクを実行するためのプログラムを読み込めなかった"),
        ("an `async fn` cannot give a function as its result", "`async fn` は関数を結果として返せない"),
        // src/testing.rs
        (".", "."),
        ("the test returned `false`", "テストが `false` を返した"),
        ("use `assert_eq(actual, expected)` to see the values", "値を確認するには `assert_eq(actual, expected)` を使う"),
        ("the test ended with {}", "テストが {} で終わった"),
        ("a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`", "テストが成功するのは、エラーなしで終わり、その値が `false`、`Err(...)`、`None` でないとき"),
        (
            r#"No tests found. Write one like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"テストが見つかりません。次のように書いてください：
  test "足し算" = assert_eq(1 + 1, 2)"#,
        ),
        ("Running {n} test(s)", "{n} 個のテストを実行中"),
        ("Running {n} of {total} test(s)", "{total} 個中 {n} 個のテストを実行中"),
        ("Running {n} test(s) of {file}", "{file} の {n} 個のテストを実行中"),
        ("Running {n} of {total} test(s) of {file}", "{file} の {total} 個中 {n} 個のテストを実行中"),
        ("{f} file(s), ", "{f} 個のファイル、"),
        ("{total} test(s): {passed} passed, {failed} failed", "{total} 個のテスト：成功 {passed}、失敗 {failed}"),
        (", {left_out} left out by the filter", "、フィルターで除外 {left_out}"),
        (", {broken} file(s) with errors", "、エラーのあるファイル {broken}"),
        ("No test name contains {}.", "{} を含む名前のテストはありません。"),
        (" or ", " または "),
        ("{red}✗{reset} {}: the file has errors, so its tests could not run", "{red}✗{reset} {}：ファイルにエラーがあるため、テストを実行できなかった"),
        (
            r#"No file with tests in {}. Write a test in a `.ry` file like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"{} にテストのあるファイルがありません。`.ry` ファイルに次のようにテストを書いてください：
  test "足し算" = assert_eq(1 + 1, 2)"#,
        ),
        ("Failed tests:", "失敗したテスト："),
        ("`{name}` needs {what}", "`{name}` には{what}が必要"),
        ("a part of the name of a test", "テスト名の一部"),
        ("a number, or `random`", "数か `random`"),
        ("the seed must be a whole number of 0 or more, or `random`, not `{v}`", "シードは 0 以上の整数か `random` でなければならないが、`{v}` だった"),
        ("unknown option `{name}` for `rythme test`", "`rythme test` に不明なオプション `{name}`"),
        (
            r#"
the options are: --filter NAME, --seed N, --watch, --sandbox, --format json (see `rythme help tests`)"#,
            r#"
オプションは --filter NAME、--seed N、--watch、--sandbox、--format json（`rythme help tests` を参照）"#,
        ),
        ("which file or folder? Example: rythme test my_tests.ry, or rythme test . for the files of this folder", "どのファイルまたはフォルダー？ 例：rythme test my_tests.ry、このフォルダーのファイルなら rythme test ."),
        ("Seed: {}", "シード：{}"),
        (
            r#"
Watching {} file(s): the tests run again when one changes (Ctrl+C to stop)."#,
            r#"
{} 個のファイルを監視中：変更されるとテストをもう一度実行します（止めるには Ctrl+C）。"#,
        ),
        (
            r#"
{} changed: running the tests again
"#,
            r#"
{} が変更されました：テストをもう一度実行します
"#,
        ),
        // src/value.rs
        ("constructor", "コンストラクター"),
        // playground/src/lib.rs
        ("✓ no problems found", "✓ 問題なし"),
        ("✓ no errors, {n} warning(s)", "✓ エラーなし、警告 {n} 件"),
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
            r#"1 分でわかる RYTHME

Rythme はシンプルな言語で、値は決して変わらず、すべてが関数です。
プログラムは .ry で終わるファイルです。

  # コメント
  name = "World"                 # 値に名前を付ける（決して変わらない）
  print("Hello, {name}!")        # {...} はテキストに値を埋め込む

  fn square(x) = x * x           # 関数
  print(square(4))               # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # 偶数だけ残す
    |> map(square)               # それぞれを 2 乗する
    |> sum                       # 合計する
    |> print                     # 20

実行する：          rythme hello.ry
チェックする：      rythme check hello.ry
コードを試す：      rythme          （対話モード）

Rythme はいつでも手助けします：どのエラーも、場所、理由、直し方を
教えてくれます。`rythme explain E003` はエラーコードを詳しく説明します。"#,
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
            r#"構文

値
  42   -7   1_000_000            整数（Int）、大きさの制限なし
  3.14   2.0                     小数（Float）
  "hello"   "total: {x}"         テキスト（Text）。{...} か #{...} で値を埋め込む
  true   false                   真偽値（Bool）
  :ok   :error                   シンボル（Symbol）：値として使う名前
  [1, 2, 3]                      リスト
  {name: "Ada", age: 36}         レコード
  Some(3)   None                 ないかもしれない値（Option）
  Ok(3)   Err("oops")            失敗するかもしれない結果（Result）
  1..5                           リスト [1, 2, 3, 4, 5]

名前
  x = 5                          値に名前を付ける。決して変わらない
  [a, b] = [1, 2]                値を分解する
  {name, age} = person

演算子
  + - * /   //（整数の割り算）   %（余り）   **（べき乗）
  == != < <= > >=   and   or   not
  x |> f(y)                      f(x, y) と同じ：左から右へのステップ
  x.f(y)                         これも f(x, y) と同じ
  x.f                            x のフィールド f。x になければ f(x)：xs.sum
  xs                             チェーンは続けられる（各行は . で始める）
    .select(even?)
    .sum

関数
  fn double(x) = x * 2           1 行なら = で
  fn greet(name)                 複数行なら `end` で終える
    message = "Hello {name}"
    print(message)               最後の行が結果になる
  end
  fn(x) = x + 1                  無名関数
  fn adult?(p) = p.age >= 18     名前の最後に ? も使える（rythme help ruby）

ブロック：呼び出しの最後の引数として渡す関数
  xs.map do |x| x * 2 end        map(xs, fn(x) = x * 2) と同じ
  xs.each do |x|                 複数行なら `end` で終える
    print(x)
  end
  xs.select { |x| x > 2 }        波かっこでも書ける

条件（if には必ず else がある）
  if x > 0 then "positive" else if x == 0 then "zero" else "negative"
  unless x > 0 then "small" else "big"     if の逆

形で選ぶ
  match list
    [] -> "empty"
    [x] -> "one element: {x}"
    [x, ..rest] -> "{x}, then {len(rest)} more"
  end

式の中の複数のステップ
  do
    a = 1
    a + 1
  end

独自の型
  type Shape = Circle(radius) | Rect(width, height)

コードに書く型（省略可、rythme check がチェックする）
  fn area(w: Int, h: Int) -> Int = w * h
  total: Float = 0.0
  type Box(a) = Box(value: a)    型を取る型：Box(Int)、Box(Text)

テストと他のファイル
  test "double works" = assert_eq(double(2), 4)
  use geometry                   その後 geometry.area(2)

コメント
  # 普通のコメント
  ## ドキュメントコメント。関数や型の上に書き、help(...) で表示される"#,
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
            r#"関数

定義と呼び出し
  fn add(a, b) = a + b
  add(2, 3)                      # 5

複数行：最後の式が結果になる
  fn describe(n)
    kind = if n % 2 == 0 then "even" else "odd"
    "{n} is {kind}"
  end

ケースごとに 1 つの定義（順に試される）
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

条件付き（ガード）
  fn sign(n) if n < 0 = "negative"
  fn sign(0) = "zero"
  fn sign(_) = "positive"

関数は値
  double = fn(x) = x * 2
  [1, 2, 3] |> map(double)       # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

パイプは左から右へ読む
  "  Hello  " |> trim |> upper   # "HELLO"
  x |> f(a, b)    は    f(x, a, b) という意味

メソッド風の書き方
  "hello".upper()                # upper("hello") と同じ

ドキュメント
  ## 2 つの数を足す。
  fn add(a, b) = a + b
  help(add)                      # "2 つの数を足す。" と表示する

制限のない再帰
  再帰呼び出しが関数の最後に行う処理なら、何百万回でも
  繰り返せます：
  fn count_down(0) = print("Liftoff!")
  fn count_down(n)
    print(n)
    count_down(n - 1)
  end"#,
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
            r#"リスト

  numbers = [3, 1, 2]
  numbers[0]                     # 3（最初の要素。-1 は最後）
  len(numbers)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numbers]                 # [0, 3, 1, 2]   （.. はリスト全体を挿入する）
  numbers + [9]                  # [3, 1, 2, 9]

リスト全体を処理する（ループは要らない）
  map(numbers, fn(x) = x * 10)            # [30, 10, 20]
  filter(numbers, fn(x) = x > 1)          # [3, 2]
  fold(numbers, 0, fn(acc, x) = acc + x)  # 6
  sort(numbers)                           # [1, 2, 3]
  numbers |> each(fn(x) = print(x))       # 各要素を表示する

安全なアクセス（決してエラーにならない）
  first([])                      # None
  get(numbers, 10)               # None

リストを「変更する」：新しいリストが得られ、元のリストはそのまま
  set_at(numbers, 0, 9)          # [9, 1, 2]
  insert_at(numbers, 1, 7)       # [3, 7, 1, 2]
  remove_at(numbers, -1)         # [3, 1]

リストの一部
  take_while(numbers, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

パターンでリストを分解する
  match numbers
    [] -> "empty"
    [first, ..rest] -> "{first}、ほかに {len(rest)} 個"
  end

すべての関数：rythme help builtins"#,
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
            r#"テキスト

  name = "Ada"
  "Hello, {name}!"               # "Hello, Ada!"  （{...} はどんな式でも埋め込む）
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3（文字数。アクセント付きの文字も 1 文字）
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hello", "ell")       # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14"（小数点以下 2 桁の数）
  pad_left("7", 3, "0")          # "007"  （pad_right もある：列をそろえるため）
  char_code("A")   char(66)      # 65 と "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...)：クラッシュしない変換
  trim_start("  x")   trim_end("x  ")
  words("Hi, you!")              # ["Hi", "you"]
  equals_ignore_case("Ada", "ADA")          # true
  format_number(1234.5, 2)       # "1,234.50"   （"fr" なら "1 234,50"）
  find_all("3 cats, 12 dogs", "[0-9]+")     # ["3", "12"]（rythme help regex）

特殊文字：\n（改行）、\t（タブ）、\r（復帰）、\0（ヌル文字）、\"（引用符）、
\\（バックスラッシュ）、\{ と \}（{ は埋め込みの始まりなので波かっこ）、
\u{e9}（Unicode 番号で表す任意の文字：é）。"#,
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
            r#"レコード

レコードは名前の付いた値をまとめます：
  ada = {name: "Ada", born: 1815}
  ada.name                       # "Ada"
  ada["name"]                    # "Ada"

レコードは決して変わりません：古いレコードから新しいレコードを作ります
  older = {..ada, born: 1816}    # 同じフィールドで、`born` だけ置き換え

分解する
  {name, born} = ada
  fn greet({name}) = "Hello {name}"

便利な関数
  keys(ada)   values(ada)   has(ada, "name")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["name", "Ada"], ["born", 1815]]
  from_entries([["x", 1]])       # {x: 1}

辞書としてのレコード
  counts = {apple: 2}
  put(counts, "pear", 5)         # {apple: 2, pear: 5}
  map_values(counts, fn(n) = n * 10)      # {apple: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}
  名前でないキー（数、リストなど）には辞書を使います：rythme help dicts

JSON（Web のデータ形式）
  parse_json(text)               # Ok(...)：オブジェクトはレコード、配列はリストに
  to_json(value)                 # write_file で保存できるテキスト"#,
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
            r#"辞書と集合

辞書（dict）はキーごとに値を与えます。キーには関数以外のどんな値でも
使えます：数、テキスト、リスト、レコードなど。順番は保たれます。
  ages = dict([["Ada", 36], ["Alan", 41]])
  ages                           # dict("Ada" => 36, "Alan" => 41)
  ages["Ada"]                    # 36（キーがなければエラー）
  get(ages, "Bob")               # None：Option で、決してエラーにならない
  grid = dict([[[0, 0], "start"]])   # リストをキーにする
  dict({x: 1})                   # レコードから：dict("x" => 1)

辞書は決して変わりません：関数が新しい辞書をすばやく（O(log n) で）返します
  put(ages, "Bob", 20)           # 追加（または置き換え）
  remove(ages, "Ada")            # このキーを除いたもの
  has(ages, "Ada")   len(ages)   keys(ages)   values(ages)   entries(ages)
  map_values(ages, fn(n) = n + 1)
  filter(ages, fn(name, age) = age > 40)   # f はキーと値を受け取る
  ages.each do |name, age| print(name, age) end   # reject と select も同様
  union(a, b)                    # 両方。両方にあるキーは b の値が優先

集合は値を重複なしに、順番どおりに持ちます。
  s = set([3, 1, 3])             # set(1, 3)
  has(s, 3)   push(s, 5)   remove(s, 1)   len(s)   values(s)   # values：リスト
  union(a, b)   intersection(a, b)   difference(a, b)   is_subset(a, b)
  filter(s, is_odd)   map(s, fn(x) = x * 10)

辞書と集合は == と < で比較でき、dict(k => v, ...) と set(a, b, ...) の
形で表示されます。`to_json` はキーがすべてテキスト（またはすべてシンボル）の
辞書をオブジェクトとして、集合を配列として書きます。
`dict(parse_json(text)?)` でオブジェクトを読み戻せます。"#,
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
            r#"独自の型

型は値がとりうる形を列挙します：
  type Shape = Circle(radius) | Rect(width, height) | Empty

  c = Circle(2.0)
  c.radius                       # 2.0

match でそれぞれの形を扱う
  fn area(shape) = match shape
    Circle(r) -> pi * r * r
    Rect(w, h) -> w * h
    Empty -> 0
  end

または形ごとに 1 つの定義で
  fn perimeter(Circle(r)) = 2 * pi * r
  fn perimeter(Rect(w, h)) = 2 * (w + h)
  fn perimeter(Empty) = 0

形を 1 つ忘れると、`rythme check` がどれかを教えてくれます。

型は自分自身を含められます（木など）：
  type Tree = Leaf | Node(left, value, right)"#,
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
            r#"MATCH とパターン

`match` は値をパターンと順に比べ、最初に当てはまったものの結果を
返します：

  fn describe(x) = match x
    0 -> "zero"
    1 | 2 | 3 -> "small"
    n if n < 0 -> "negative"
    _ -> "big"
  end

パターン
  42   "text"   true             ちょうどこの値
  1..9   -5..-1   0.0..0.5       最初から 2 番目までの数（両端を含む）
  "Mr " + name                   "Mr " で始まるテキスト。name は残りの部分
  x                              何でもよく、x と名付ける
  _                              何でもよく、無視する
  Some(x)   Circle(r)            コンストラクターとその中身
  []   [a, b]   [first, ..rest]  形で見るリスト
  {name, age}                    これらのフィールドを持つレコード
  p if condition                 パターンと条件（ガード）
  a | b                          どちらかのパターン

パターンは名前を付けるところならどこでも使えます：
  [x, y] = [1, 2]
  fn head_or_zero([]) = 0
  fn head_or_zero([x, ..]) = x"#,
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
            r#"エラーは値（Option、Result、?）

Rythme には null も例外もありません。ないかもしれない値は Option：
Some(value) か None です。失敗するかもしれない操作は Result を返します：
Ok(value) か Err(message) です。扱い忘れることはありません。

  fn divide(a, b) = if b == 0 then Err("ゼロ除算") else Ok(a / b)

  match divide(10, 2)
    Ok(x) -> print("結果：{x}")
    Err(e) -> print("エラー：{e}")
  end

? 演算子：値を取り出すか、止まってエラーを呼び出し元に渡す
  fn average(a, b)
    x = parse_float(a)?          # これが Err(...) なら average はそれを返す
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

手軽な道具
  unwrap_or(get(list, 5), 0)     # 値か、デフォルト値
  unwrap(Some(3))                # 3（None や Err ならプログラムを止める）
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(person, "age"), "no age")   # なければ Err("no age")。? で使える
  and_then(x, f)                 # 失敗するかもしれないもう 1 つのステップ
  map_err(r, fn(e) = "config: {e}")     # エラーをよりよく説明する

プログラム全体を止めるには：fail("message")。"#,
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
            r#"関数型の考え方

Rythme では値は決して変わらず、ループもありません。不思議に思えるかも
しれませんが、プログラムは読みやすく、テストしやすく、信頼しやすくなります：
関数は同じ引数に対していつも同じ結果を返します。

表示するループの代わりに：          1..5 |> each(fn(i) = print(i))
変換するループの代わりに：          map(list, f)
選び出すループの代わりに：          filter(list, f)
積み上げるループの代わりに：        fold(list, start, f)   または sum、product
累計の代わりに：                    scan(list, start, f)
探すループの代わりに：              find、any、all、index_of、min_by、max_by
"break" の代わりに：                take_while(list, f)
"while" の代わりに：                再帰関数

  # Python:  total = 0
  #          for x in numbers: total += x * x
  total = numbers |> map(fn(x) = x * x) |> sum

  # "while" ループを再帰関数で：
  fn collatz(1, steps) = steps
  fn collatz(n, steps) =
    if n % 2 == 0 then collatz(n // 2, steps + 1) else collatz(3 * n + 1, steps + 1)
  collatz(27, 0)                 # 111

値を変える代わりに、新しい値を作ります：
  x2 = x + 1
  person2 = {..person, age: 37}
  list2 = push(list, 4)"#,
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
            r#"テスト

テストはコードのすぐそばに書きます：
  fn double(x) = x * 2

  test "2 の 2 倍は 4" = assert_eq(double(2), 4)

  test "いくつかのチェック"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "負のままのはず")
  end

実行する：
  rythme test my_file.ry            1 つのファイルのテスト
  rythme test .                     フォルダー以下のすべての .ry ファイル
  rythme test . --filter double     名前に "double" を含むテストだけ
  rythme test . --watch             ファイルが変わるたびに再実行（Ctrl+C で止める）
  rythme test my_file.ry -- a b     `args()` が ["a", "b"] を返す状態で
  rythme test                       プロジェクトでは、テストのあるファイル

テスト中は、print(...) のような最上位の式は実行されません：定義が
読み込まれ、それから各テストが実行されます。テストが成功するのは、
エラーなしで終わり、その値が false、Err(...)、None でないときです。
まとめには件数と時間が表示され、すべてのテストが成功すると終了コードは
0 になります。

プロパティテスト：多くの値で何かが成り立つことを確かめる
  test "2 回逆にすると元のリストに戻る" =
    check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

ジェネレーター：ints(min, max)、floats(min, max)、texts(max_length)、bools()、
lists_of(generator, max_length)、one_of(list)、records_of({x: ints(), y: texts()})。
`sample(generator)` はジェネレーターが返す値を見せます。ある値でプロパティが
偽になると、その値は単純化（シュリンク）され、それを生んだシードとともに
表示されます：毎回同じ値が出てくるので、`--seed N` で別の値を試します
（`--seed random` はシードを 1 つ選びます）。"#,
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
            r#"練習問題

Rythme には入門コース向けに、段階的な練習問題が 36 問付いています。最初の
値から再帰、リスト、レコード、型、Option、Result、テストまで進み、最後の
6 問は人工知能がテーマです（31〜36：ベクトル、統計、ニューロン、勾配降下法、
最近傍法、ミニマックス）。それぞれが、問題、ヒント、書くべき関数と
そのテストを含む 1 つのファイルです。

  rythme exercise list              練習問題の一覧。★☆☆（やさしい）から ★★★ まで
  rythme exercise start 7           練習問題 7 を現在のフォルダーにコピーする
  rythme exercise check 07-recursion.ry
                                    テストを実行し、次にすることを教える

ファイルの中のそれぞれの fail("未完成") を自分のコードに置き換えてください。
テストは変えないでください。`rythme test file.ry` でもテストを実行できます。
練習問題はメッセージの言語に従います（`rythme --lang ja exercise start 7`）。
Rythme のリポジトリの docs/exercises フォルダーには英語版とフランス語版が
あります。"#,
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
            r#"複数のファイル

  # geometry.ry
  ## 長方形の面積。
  fn area(w, h) = w * h
  fn _check(x) = x >= 0          # 非公開：`_` で始まる

  # main.ry
  use geometry                    # main.ry の隣のファイル geometry.ry
  print(geometry.area(2, 3))      # 6

`use geometry` は geometry.ry の定義をその名前を通して使えるようにします：
`geometry.area`、`geometry.Circle(1)`、`match` の中なら `geometry.Circle(r)`。
こうして、名前がどこから来たのかがいつでもわかります。フォルダーの中の
ファイルなら `use shapes/circle` の後に `circle.perimeter(1)`。別の名前を
付けるなら `use geometry as geo` の後に `geo.area(2, 3)`。`_` で始まる名前は
そのファイルの中だけのものです。

パスは `use` を含むファイルのフォルダーからの相対パスです。各ファイルは
1 回だけ実行されます。使い忘れた名前がどのファイルで定義されているかは
Rythme が教えてくれます。

古い形：`use "geometry.ry"`（引用符付き）は定義を直接使えるようにします
（`area(2, 3)` のように）。`use geometry` のほうがおすすめです。"#,
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
            r#"ファイルとシステム

システムに触れる関数はどれも Result を返します：Ok(...) か、何がうまく
いかなかったかを伝えるメッセージ付きの Err(message) です。

ファイルとフォルダー（相対パスは現在のフォルダーから始まる）
  read_file("notes.txt")         # Ok("...") か Err("ファイルが見つからない：notes.txt")
  write_file("out.txt", "hi")    # Ok(())。内容を置き換える
  append_file("log.txt", "done\n")
  file_exists("notes.txt")       # true か false
  file_info("notes.txt")         # Ok({is_folder: false, size: 11})。バイト単位
  list_folder(".")               # Ok(["notes.txt", "photos"])。順番どおり
  create_folder("backup/2024")   # Ok(())。足りない上位フォルダーも作る
  delete_file("old.txt")         # Ok(())。フォルダーは決して削除しない

Linux、macOS、Windows で使えるパス
  join_path(["data", "notes.txt"])       # "data/notes.txt"
  split_path("/home/ada/notes.txt")      # ["/", "home", "ada", "notes.txt"]
  split_path("C:\Users\ada")             # ["C:\", "Users", "ada"]

コマンドライン：rythme tool.ry --verbose --name=Ada notes.txt
  args()                         # ["--verbose", "--name=Ada", "notes.txt"]
  options()                      # Ok({args: ["notes.txt"], verbose: true, name: "Ada"})
  opts = unwrap(options())
  name = unwrap_or(get(opts, "name"), "World")   # ないかもしれないオプション
  env("HOME")                    # Some("/home/ada") か None

標準入力：rythme count.ry < names.txt
  input("Name? ")                # 1 行
  input_lines()                  # Ok(["Ada", "Alan"])：終わりまでのすべての行

終了コード（プログラムを実行するスクリプト向け）
  0    すべてうまくいった（プログラムの終わりでも）
  1    プログラムのエラー：構文、チェック、実行中、テストの失敗
  2    コマンドの使い方の誤り（不明なコマンドやオプション）
  101  Rythme の内部エラー（報告すべきバグ）
  exit(code)                     # このコード（0〜255）ですぐに止まる

ブラウザーのプレイグラウンドにはファイルも入力もありません：これらの
関数は Err("... はブラウザーでは使えない ...") を返します。"#,
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
            r#"プロジェクトとパッケージ

  rythme new hello        # 新しいプロジェクトを入れたフォルダー hello/：
                          #   rythme.toml, src/main.ry, src/greeting.ry,
                          #   tests/greeting_test.ry, README.md, .gitignore
  cd hello
  rythme run              # プログラムを実行する：rythme.toml の `entry`
  rythme test             # src/ と tests/ のテストのあるファイルを実行する
  rythme check            # 全ファイルをチェック。`rythme fmt`、`rythme doc` も

rythme.toml はプロジェクトを記述します：

  name = "hello"
  version = "0.1.0"
  entry = "src/main.ry"

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

依存関係は、`tag` か `commit` を指定した git か、フォルダーから取得します。
`rythme install` はそれらを（git で）ダウンロードし、それぞれの正確な
コミットを rythme.lock に書きます：全員が同じコードを使えるよう、
このファイルはプロジェクトと一緒に保管してください。その後は：

  use mathlib/vectors       # mathlib のファイル src/vectors.ry
  use mathlib               # そのプログラム（rythme.toml の `entry`）
  use hello/greeting        # tests/ から使う、プロジェクト自身のファイル

`use` を含むファイルの隣にあるファイルが優先されます。git からの
パッケージは ~/.rythme/packages（または RYTHME_HOME/packages）に保管されます。
名前でパッケージを探したり、自分のパッケージを共有したりするには：
`rythme help packages`。"#,
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
            r#"日付と時刻

日付はレコード {year, month, day} で、1 日の中の時刻には hour、minute、
second を、わかっていれば offset（UTC より進んでいる分数）を加えます。
  today()                        # {year: 2026, month: 10, day: 18}
  now()                          # {year: 2026, ..., hour: 14, minute: 5, second: 9, offset: 120}
  birthday = {year: 1815, month: 12, day: 10}
  birthday.year                  # 1815

テキストとの相互変換（ISO 8601 形式）
  parse_date("2024-02-29")       # Ok({year: 2024, month: 2, day: 29})
  parse_date("2024-02-30")       # Err("... 2024年2月 ...")
  parse_date("2024-02-29T13:05:00+01:00")    # 時刻と時差付き
  format_date({year: 2024, month: 2, day: 29, hour: 9})   # "2024-02-29T09:00:00"

暦の計算（うるう年と月の長さも考慮）
  add_days({year: 2024, month: 2, day: 28}, 1)     # {year: 2024, month: 2, day: 29}
  add_seconds(now(), 3600)       # 1 時間後
  days_between(today(), {year: 2027, month: 1, day: 1})   # 残りの日数
  seconds_between(a, b)          # b が a より前なら負
  day_of_week(today())           # 1 = 月曜日 ... 7 = 日曜日
  days_in_month(2023, 2)         # 28
  is_leap_year(2000)             # true
  is_date({year: 2023, month: 2, day: 29})   # false

存在しない日付を渡すと、関数は E023 で止まり、その理由を示します。
ローカルの時差はシステムから取得します（わからなければ UTC）。time() は
1970 年からの秒数を Float で返すので、時間の計測に使えます。"#,
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
            r#"正規表現：パターンで検索する

正規表現（regex）はテキストの断片を表します："[0-9]+" は「1 つ以上の数字」
という意味です。Rythme は一般的な正規表現を読み、検索にかかる時間は
（テキストの長さ）×（正規表現の大きさ）を超えません。

  matches("2024", "[0-9]+")                  # true：テキスト全体がマッチする
  find_first("tel: 06 12", "[0-9]+")         # Some("06")：最初の断片
  find_all("3 cats, 12 dogs", "[0-9]+")      # ["3", "12"]
  split_by("a, b;c", "[,;] *")               # ["a", "b", "c"]
  replace_all("a cat", "\\b\\w", upper)      # "A Cat"（断片ごとに関数を適用）
  replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1")   # "2024-02-29"
  captures("2024-02", "([0-9]+)-([0-9]+)")   # Some(["2024-02", "2024", "02"])
  named_captures("2024-02", "(?<year>[0-9]+)-(?<month>[0-9]+)")
                                             # Some({year: "2024", month: "02"})

正規表現で表せること
  abc          これらの文字をこの順で
  .            改行以外の任意の文字
  [abc]        これらの文字のどれか。[a-z] は a〜z のどれか。[^0-9] はそれ以外
  \d \w \s     数字（0-9）。英字（アクセント付きも）、数字、_。空白
  \D \W \S     それ以外
  \b           単語の境界
  ^  $         テキストの先頭と末尾
  x|y          x か y
  (...)        グループ：繰り返し、テキストのキャプチャ（$1、`captures`）
  (?:...)      何もキャプチャしないグループ。(?<name>...) は名前付きグループ
  x*  x+  x?   x が 0 回以上、1 回以上、0 回か 1 回（最長一致）
  x{3}  x{2,5}  x{2,}    3 回、2〜5 回、2 回以上
  x*?  x+?     最長ではなく最短一致
  (?i)         先頭に置くと、大文字と小文字を同じとみなす

Rythme のテキストで正規表現を書く
  バックスラッシュは 2 回書きます："\\d+" が正規表現 \d+ です。波かっこは
  \{ と \} と書きます："[0-9]\{4\}" が正規表現 [0-9]{4} です。記号そのものは
  角かっこに入れます："[.]" はドット、"[(]" は丸かっこ、"[$]" はドル記号です。

正規表現に誤りがあると、プログラムは E026 で止まり、その場所を示します。
正規表現がコードに書かれていれば、`rythme check` が先に見つけます。"#,
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
            r#"対話モード

ファイルを指定せずに `rythme` を実行します。式を入力すると値が表示されます：
  ry> 1 + 2
  3
定義は行をまたいで保持されます。行が終わっていなければ（閉じていない
かっこ、`end` のないブロックなど）、次の行に続きます。

コマンド
  :help           このヘルプ             :quit          終了（または Ctrl-D）
  :doc map        名前のヘルプ           :load f.ry     ファイルを読み込む
  :reset          すべて忘れる           :lang ja       言語を変える（en、fr、es、de、pt、zh、it、ja）
  :learn          ガイドツアー           :explain E003  エラーを説明する
  :steps fib(4)   計算を 1 ステップずつ表示する
Tab キーで名前を補完できます。"#,
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
            r#"整形：rythme fmt

`rythme fmt` はコードを Rythme の公式のレイアウトに整えます：どのプログラムも
同じ見た目になり、空白について議論する必要がなくなります。

  rythme fmt hello.ry            ファイルを整形する（書き換えられる）
  rythme fmt .                   フォルダー以下のすべての .ry ファイル
  rythme fmt --check .           チェックだけ：整形が必要なファイルを一覧し、
                                 1 つでもあれば失敗する（終了コード 1）。CI 向け
  rythme fmt - < hello.ry        コードを読み、結果を書く

スタイル
  - インデントは 1 段につき空白 2 つ
  - 演算子の前後に空白：x = a + b * 2   xs |> map(f)   n -> n * 2
  - かっこの内側と .. の前後には空白なし：f(x)   [1, 2]   {name: "Ada"}   1..10
  - 複数行のパイプラインは 1 行に 1 ステップ
      total = prices
        |> filter(fn(p) = p > 10)
        |> sum
  - 複数行の型は 1 行に 1 バリアント
      type Shape =
        | Circle(radius)
        | Rect(width, height)
  - 複数行のリストはカンマで終わる
  - 1 行で書いたブロックは 1 行のまま：
      xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
    複数行なら、その文は `do` より 1 段深くなる
  - 1 行に 1 文（`;` はない）、`end` は単独の行に
  - 複数行の定義の前後には空行を 1 つ。空行を 2 つ続けない
  - コメントはすべて残る。連続する行の末尾のコメントはそろえられる

改行は言語が選択を許す場所では保たれ、長い行は書いたままになります。
整形でプログラムの動作が変わることはありません：構文エラーのある
ファイルはそのままにされ、エラーが表示されます。"#,
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
            r#"ドキュメント：rythme doc

`rythme doc` は、`##` コメント、型、関数の定義からコードのドキュメントを
作ります。

  rythme doc geometry.ry         画面に Markdown で
  rythme doc .                   フォルダー以下のすべての .ry ファイル
  rythme doc --html geometry.ry  他に何も要らない 1 つの Web ページ
  rythme doc . --out doc.html    ファイルに書く（.html なら Web ページ）

`##` コメントは、説明する対象のすぐ上に書きます：

  ## 図形とその面積。             <- 先頭に書き、空行を続けると、
                                     ファイル全体の説明になる
  ## 描く図形。
  type Shape = Circle(radius) | Rect(width, height)

  ## 図形の面積。`len` と `area` も参照。
  fn area(Circle(r)) = pi * r * r
  fn area(Rect(w, h)) = w * h

書かれる内容
  - 各ファイルについて：説明、使っているファイル（それも文書化される）、
    型、関数（各定義の 1 行目）、値、テストの名前
  - `_` で始まる名前は除かれる：そのファイルの中だけのもの
  - コメントの中でバッククォートで囲んだ名前（`area` や `len(list)` など）は
    そのドキュメントへのリンクになる。`use geometry` の後なら
    `geometry.area` は geometry.ry の `area` につながる。このように触れた
    組み込み関数は最後に説明される
  - 見出しはメッセージの言語になる（`--lang fr`）

コメントの中では、空の `##` 行で新しい段落になり、`- ` で始まる行は
箇条書きになります。"#,
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
            r#"ステップ実行：rythme steps

`rythme steps` は、プログラムが値を計算する様子を 1 ステップずつ見せます：
各呼び出しは、引数の値を当てはめた関数の本体に置き換えられ、最後には
結果だけが残ります。

  rythme steps fib.ry              ファイルの各定義と各式
  rythme steps fib.ry "fib(4)"     ファイルの定義の後に、1 つの式
  rythme steps fib.ry --max 50     各式について最大 50 ステップまで表示する
  rythme steps fib.ry --width 80   80 文字より長い行を切る

例：fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

  fib(3)
    → fib(2) + fib(1)
    → fib(1) + fib(0) + fib(1)
    → 1 + fib(0) + fib(1)
    → 1 + 0 + fib(1)
    → 1 + fib(1)
    → 1 + 1
    → 2

1 ステップは 1 回の呼び出しか 1 回の演算です。呼び出された関数の本体は、
他の関数を呼ばずにできるところまで単純化して表示されます。組み込み関数
（と他のファイルの関数）は 1 ステップで結果を返します。プログラムが表示する
ものはステップの間に現れます。表示するステップ（デフォルトで 200）の後は、
表示せずに計算を続けます。

対話モードでは :steps fib(4)。プレイグラウンドではステップボタン。
フランス語では rythme etapes。"#,
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
            r#"エディターで使う RYTHME

Rythme はエディターの中でも手助けします：入力中の問題の表示、マウスを
重ねたときの関数のドキュメント（`fn area(side: Int|Float) -> Int|Float` の
ようにコードから推論したシグネチャ付き）、補完、呼び出しの引数、定義へ
移動、参照、名前の変更、打ち間違いのクイックフィックス、`rythme fmt` と
同じ整形、確実なときの名前の型（と関数のシグネチャ）、名前の種類に応じた
色分け、コードの上の "Run" と "Run test"、ワークスペース全体の名前、
折りたたみ。これは言語サーバーで、エディターが自動的に起動します：
  rythme lsp

VS Code
  editors/vscode の拡張機能：色分け、スニペット、言語サーバー。
  設定：rythme.path（プログラム）、rythme.language（en、fr、es、de、pt、zh、it、ja、auto）。

Neovim、Helix、Zed、Emacs、Sublime Text、Kate など
  Language Server Protocol に対応したエディターならどれでも：.ry ファイルに
  `rythme lsp` を起動するよう設定します。設定例：editors/README.md
  （"Run" と "Run test" はコマンド rythme.runFile、rythme.testFile、
  rythme.runTest を指定し、エディターがそれを実行します：editors/README.md を参照）

Zed
  editors/zed の拡張機能：色分けと言語サーバー。

Neovim、Helix、Zed での色分け
  editors/tree-sitter-rythme の Tree-sitter 文法（editors/README.md を参照）。

メッセージの言語はエディターに従います。`--lang`（en、fr、es、de、pt、zh、it、ja）
か RYTHME_LANG で選ぶこともできます：rythme --lang fr lsp"#,
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
            r#"RUBY から来た人へ

Rythme は Ruby のように読めますが、値は決して変わりません：ミューテーション
も nil も例外もありません。Ruby で知っていることはたいてい使えます：

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

Ruby と同じもの
  呼び出し        puts "hi"   print x, y   assert_eq total, 42
                  （文や結果の先頭で。下を参照）
  ブロック        xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
                  ブロックは関数：fn(x) = x * 2 と同じ
  メソッド        xs.sum   name.upcase   xs.first(2)   xs.include?(3)
  ? 付きの名前    even?   empty?   include?(x)   fn adult?(p) = ...
  シンボル        :ok   :error（== で比較し、match で使う）
  テキスト        "Hello #{name}"（または "Hello {name}"）。"\#{" で #{ と書ける
  unless          unless cond then a else b
  Ruby の名前     puts to_s to_i to_f length size upcase downcase strip
                  select reject reduce inject uniq each_slice each_cons
                  each_with_index include? empty? even? odd? any? all?
                  start_with? end_with? key? lstrip rstrip casecmp?
                  （rythme help builtins を参照）

違うところ
  括弧            式の中の呼び出しには必要：x = len(xs)、
                  x = len xs ではない
  値の変更        x += 1 はない：新しい値に新しい名前を付ける。total2 = total + 1
  nil             ない値は None、ある値は Some(x)
  例外            エラーは値で、Ok(x) か Err(message)。? で呼び出し元に渡す
  if と unless    必ず else があり、値を返す。`return` はなく、
                  関数の最後の行がその結果になる
  クラス          レコード {name: "Ada"} と型：type Shape = Circle(r) | Square(s)
  ループ          each、map、select、reduce、または自分自身を呼ぶ関数
  case/when       match value ... end。1 行に 1 ケース：pattern -> result
  ハッシュ        dict([[:ada, 36]])（シンボルを含め、どんな種類のキーも可）
                  またはレコード {name: "Ada", age: 36}

括弧なしの呼び出し
  名前で始まり、同じ行にその後で値の始まり（数、テキスト、シンボル、名前、
  コンストラクター、not、if、match、fn）が続く文は呼び出しです：print x, y は
  print(x, y)。引数の後の do ブロックは呼び出しに渡されます：
  each xs do |x| ... end。Ruby と同じく、空白の後にあり、直後に空白のない
  [ や - も引数の始まりになります（puts [1, 2]、puts -1）。ただし、コードが
  その名前に値を与えている場合（=、引数、パターンで）は別で、x -1 は x - 1、
  xs [0] は xs[0] のままです。`rythme fmt` はそれらを x - 1、xs[0] と書きます。

  結果も同じように始められます：if や unless の分岐、match のケース、
  関数やテストの = の後の本体。その引数は else、end、}、)、行の終わりで
  終わります：

  fn greet(name) = puts "Hello, #{name}!"
  fn check(ok) = if ok then puts "yes" else puts "no"
  match name
    "Dr " + rest -> puts "doctor", rest
    _ -> puts name
  end
  xs.each { |x| puts x }

x.name の仕組み
  x.name はレコードのフィールド `name` を読みます。x にそのフィールドが
  なければ、x を引数に関数 `name` を呼びます：xs.sum は sum(xs)、
  xs.first(2) は first(xs, 2)。チェーンは複数行に続けられ、各行は `.` で始めます。

? が名前の一部になるとき
  even?(4)、xs.include?(3)、fn adult?(p)：名前の直後、`(` の前。
  `(` のない even?：そういう関数がある（組み込み、またはファイルの中で fn で
  定義した）とき。それ以外では value? は None や Err で止まる ? で、
  (value)? は常にそうなります。"#,
        ),
        (
            r#"COMING FROM JAVASCRIPT

Rythme looks like modern JavaScript without its traps: every value is a
const that never changes (arrays and objects too), there is no null, no
undefined and no exceptions, and == never converts. What you write every
day has a short equivalent:

  names = ["Ada", "Grace", "Alan"]
  names.each(fn(n) = print("Hi {n}!"))

  [3, 1, 2]
    .filter(fn(x) = x > 1)
    .map(fn(x) = x * 10)
    .sort()                      # [20, 30]

  person = {name: "Ada", langs: ["fr"]}
  older = {..person, age: 36}    # {...person, age: 36}

The language
  const x = 1                    x = 1 (a name never changes)
  x => x * 2                     fn(x) = x * 2
  `Hi ${name}!`                  "Hi {name}!"
  a === b                        a == b (never converts: 1 == "1" is false)
  c ? a : b, if/else             if c then a else b (always with else)
  switch                         match value ... end, one case per line
  null, undefined                None, or Some(x)
  try/catch, throw               Ok(x) or Err(message), and ? to pass it on
  async function, await          async fn, await (tasks run on other cores)
  Promise.all(ps)                await_all(tasks)
  [...xs, 3], {...a, b: 2}       [..xs, 3], {..a, b: 2}
  a?.b?.c                        dig(a, ["b", "c"])
  import { f } from "./geo.js"   use geo, then geo.f(x)
  class, new, this               records {name: "Ada"} and types:
                                 type Shape = Circle(r) | Square(s)
  for, while                     each, map, filter, fold, or recursion
  x += 1, push, pop, sort()      new values: total2 = total + 1, push(xs, x)

Methods are functions
  xs.map(f) is map(xs, f), and both are written in Rythme: x.name(...)
  calls the function `name` with x first. Its names are in snake_case, and a
  JavaScript name gives the Rythme one: findLast says "did you mean
  `find_last`?". The most used:

  Math.floor, ceil, round, trunc floor, ceil, round, int
  Math.max(...xs), Math.min      max(xs), min(xs)
  Math.random()                  random(), random(1, 6), random_seed(n)
  Math.atan2(y, x), Math.log2(x) atan(y, x), log(x, 2)
  parseInt, parseFloat           parse_int, parse_float (a Result, never NaN)
  n.toFixed(2)                   text(n, 2)
  s.length, a.length             len(s), len(a)
  s.toUpperCase(), s.trim()      upper(s), trim(s)
  s.includes(x), startsWith      contains, starts_with
  s.padStart(3, "0")             pad_left(s, 3, "0")
  s.replace, s.replaceAll        replace_first, replace
  s.split(","), a.join(", ")     split(s, ","), join(a, ", ")
  a.find, findIndex, findLast    find, find_index, find_last (an Option)
  a.some(f), a.every(f)          any, all
  a.reduce(f, init)              fold(a, init, f)
  a.flat(), a.flatMap(f)         flatten, flat_map
  a.toSorted(), a.toReversed()   sort, reverse
  a.at(-1)                       get(a, -1) (an Option), last(a)
  Object.keys, values, entries   keys, values, entries
  new Map(), get, set            dict(), get(d, k), put(d, k, v)
  new Set(a), s.has(x)           set(a), has(s, x)
  JSON.parse, JSON.stringify     parse_json (a Result), to_json
  new URL(s)                     parse_url(s) (a Result)
  new URLSearchParams(s)         parse_query(s), format_query(record)
  crypto.randomUUID()            uuid()
  setTimeout                     sleep(seconds)
  fetch(url)                     http_get(url), http_post(url, body)
  console.log, console.error     print, print_error

The whole standard library of JavaScript, with what is absent on purpose
and why: docs/javascript.md in the sources of Rythme; on its site, the page
of each function says what JavaScript writes for it."#,
            r#"JAVASCRIPT から来た人へ

Rythme は落とし穴のない現代的な JavaScript のように見えます：どの値も決して
変わらない定数で（配列もオブジェクトも）、null も undefined も例外もなく、
== は決して型を変換しません。毎日書くものには短い書き方があります：

  names = ["Ada", "Grace", "Alan"]
  names.each(fn(n) = print("Hi {n}!"))

  [3, 1, 2]
    .filter(fn(x) = x > 1)
    .map(fn(x) = x * 10)
    .sort()                      # [20, 30]

  person = {name: "Ada", langs: ["fr"]}
  older = {..person, age: 36}    # {...person, age: 36}

言語
  const x = 1                    x = 1（名前は決して変わらない）
  x => x * 2                     fn(x) = x * 2
  `Hi ${name}!`                  "Hi {name}!"
  a === b                        a == b（変換しない：1 == "1" は false）
  c ? a : b, if/else             if c then a else b（必ず else を付ける）
  switch                         match value ... end。1 行に 1 ケース
  null, undefined                None か Some(x)
  try/catch, throw               Ok(x) か Err(message)。? で呼び出し元に渡す
  async function, await          async fn, await（タスクは別のコアで動く）
  Promise.all(ps)                await_all(tasks)
  [...xs, 3], {...a, b: 2}       [..xs, 3], {..a, b: 2}
  a?.b?.c                        dig(a, ["b", "c"])
  import { f } from "./geo.js"   use geo のあと geo.f(x)
  class, new, this               レコード {name: "Ada"} と型：
                                 type Shape = Circle(r) | Square(s)
  for, while                     each, map, filter, fold か再帰
  x += 1, push, pop, sort()      新しい値：total2 = total + 1, push(xs, x)

メソッドは関数
  xs.map(f) は map(xs, f) で、どちらも Rythme で書けます：x.name(...) は x を
  最初の引数にして関数 `name` を呼びます。Rythme の名前は snake_case で、
  JavaScript の名前を書くと Rythme の名前が出ます：findLast には「もしかして
  `find_last`？」と出ます。よく使うもの：

  Math.floor, ceil, round, trunc floor, ceil, round, int
  Math.max(...xs), Math.min      max(xs), min(xs)
  Math.random()                  random(), random(1, 6), random_seed(n)
  Math.atan2(y, x), Math.log2(x) atan(y, x), log(x, 2)
  parseInt, parseFloat           parse_int, parse_float（Result、NaN なし）
  n.toFixed(2)                   text(n, 2)
  s.length, a.length             len(s), len(a)
  s.toUpperCase(), s.trim()      upper(s), trim(s)
  s.includes(x), startsWith      contains, starts_with
  s.padStart(3, "0")             pad_left(s, 3, "0")
  s.replace, s.replaceAll        replace_first, replace
  s.split(","), a.join(", ")     split(s, ","), join(a, ", ")
  a.find, findIndex, findLast    find, find_index, find_last（Option を返す）
  a.some(f), a.every(f)          any, all
  a.reduce(f, init)              fold(a, init, f)
  a.flat(), a.flatMap(f)         flatten, flat_map
  a.toSorted(), a.toReversed()   sort, reverse
  a.at(-1)                       get(a, -1)（Option を返す）, last(a)
  Object.keys, values, entries   keys, values, entries
  new Map(), get, set            dict(), get(d, k), put(d, k, v)
  new Set(a), s.has(x)           set(a), has(s, x)
  JSON.parse, JSON.stringify     parse_json（Result を返す）, to_json
  new URL(s)                     parse_url(s)（Result を返す）
  new URLSearchParams(s)         parse_query(s), format_query(record)
  crypto.randomUUID()            uuid()
  setTimeout                     sleep(seconds)
  fetch(url)                     http_get(url), http_post(url, body)
  console.log, console.error     print, print_error

JavaScript の標準ライブラリ全体と、わざと入れていないものとその理由は、
Rythme のソースの docs/javascript.md にあります。Rythme のサイトでは、各関数
のページに JavaScript での書き方が載っています。"#,
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
            r#"E001 — 構文エラー

この場所のコードが Rythme の文法に従っていません。メッセージは何が
期待されていたかを示します。よくある原因：
  - `)`、`]`、`,` が足りない
  - `if` に `then` や `else` がない
  - ブロックに波かっこ { } を使っている（Rythme は改行と `end` を使う）
  - 他の言語のキーワード（for、while、return、let、def など）

誤り：   if x > 0 { print(x) }
正しい： if x > 0 then print(x) else print("正ではない")"#,
        ),
        (
            r#"E002 — Text never closed

A text starts with " and must end with " on the same text. A `{` inside a
text starts an insertion and must be closed by `}`.

Wrong:   print("hello)
Right:   print("hello")
To write a brace in a text: "\{not inserted\}""#,
            r#"E002 — 閉じられていないテキスト

テキストは " で始まり、同じテキストの中の " で終わらなければなりません。
テキストの中の `{` は埋め込みの始まりで、`}` で閉じる必要があります。

誤り：   print("hello)
正しい： print("hello")
テキストに波かっこを書くには："\{埋め込まない\}""#,
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
            r#"E003 — 不明な名前

この名前はこの場所では定義されていません。つづり（大文字と小文字は区別
されます）と、名前が使う前に定義されているかを確かめてください。関数の
中では、引数と上で定義した名前が見えます。続けて書いたローカル関数どうしは
互いに見えます。最上位では、すべての関数がどこからでも見えます。

誤り：   print(lenght([1, 2]))
正しい： print(len([1, 2]))

Rythme はいちばん近い既存の名前があればそれを提案し、フォルダーの別の
ファイルで定義されていれば、そのファイルを教えます：

  use geometry                   # その後で
  print(geometry.area(2))        # area(2) ではない

`_` で始まる名前はそのファイルの非公開の名前です：他のファイルからは使えません。"#,
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
            r#"E004 — 引数の数が違う

関数が、取るよりも多い（または少ない）値で呼ばれています。`|>` では、
左側の値が最初の引数になることを思い出してください。

誤り：   fn add(a, b) = a + b
         add(1)
正しい： add(1, 2)
これも正しい：  1 |> add(2)

組み込み関数の呼び方は `rythme help NAME` で確認できます。"#,
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
            r#"E005 — 値の種類が違う

この操作はこの種類の値には使えません。Rythme は値を黙って変換しません：
"1" + 1 はエラーで、"11" にも 2 にもなりません。

誤り：   "age: " + 36
正しい： "age: {36}"              （値をテキストに埋め込む）
正しい： "age: " + text(36)       （先に変換する）

条件は true か false でなければなりません：
誤り：   if len(list) then ... else ...
正しい： if len(list) > 0 then ... else ...

値の種類が確実なとき、`rythme check` はこの誤りをプログラムの実行前に
見つけます。毎回は実行されないコードの中でも、関数が使えないものを渡す
呼び出しでも見つけます：`area(side)` が `side * side` を計算するときの
`area("big")` のように。"#,
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
            r#"E006 — どのケースにもマッチしない

`match` のケース（または関数の定義、`[a, b] = list` のような定義の
パターン）のどれもこの値を受け付けません。

誤り：   fn name(1) = "one"
         name(2)
正しい： fn name(1) = "one"
         fn name(_) = "別の数"

`rythme check` は忘れたケースを実行前に警告します（W002）。値の種類が
確実なときは、決して当てはまらないケースも実行前に報告します：Text に
対する `[first, ..rest]`、数に対する `Circle(r)` など。"#,
        ),
        (
            r#"E007 — Index out of range

Positions in a list or a text go from 0 to length - 1 (and -1 is the last).

Wrong:   [10, 20][2]
Right:   [10, 20][1]              # 20
Safe:    get([10, 20], 2)         # None, never an error"#,
            r#"E007 — インデックスが範囲外

リストやテキストの位置は 0 から長さ - 1 まで（-1 は最後）です。

誤り：   [10, 20][2]
正しい： [10, 20][1]              # 20
安全：   get([10, 20], 2)         # None。決してエラーにならない"#,
        ),
        (
            r#"E008 — Division by zero

Dividing by zero has no meaning. Check the divisor before, or return a Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
            r#"E008 — ゼロ除算

ゼロで割ることには意味がありません。先に割る数を確かめるか、
Result を返してください。

  fn divide(a, b) = if b == 0 then Err("ゼロ除算") else Ok(a / b)"#,
        ),
        (
            r#"E009 — Not a function

Parentheses after a value call it, but only functions can be called.

Wrong:   x = 5
         x(3)
Wrong:   None()                   (None is a value)
Right:   None
To multiply, write it:  2 * (x + 1)"#,
            r#"E009 — 関数ではない

値の後の括弧はそれを呼び出しますが、呼び出せるのは関数だけです。

誤り：   x = 5
         x(3)
誤り：   None()                   （None は値）
正しい： None
掛け算なら明示的に書く：  2 * (x + 1)"#,
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
            r#"E010 — 不明なフィールド

レコード（またはコンストラクター）にこの名前のフィールドがありません。
または辞書にそのキーがありません（`dict[key]`）。

誤り：   {name: "Ada"}.nmae
正しい： {name: "Ada"}.name
安全：   get(record, "name")     # Option。決してエラーにならない
         get(dict, key)          # 辞書でも同じ

レコードのフィールドが確実なとき、`rythme check` はこの誤りをプログラムの
実行前に見つけ、いちばん近いフィールドを提案します。"#,
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
            r#"E011 — すでに定義されている

Rythme では、名前はいつも同じ値を指します：同じ場所で変更したり 2 回
定義したりはできません。これがプログラムを追いやすくしています。

誤り：   x = 1
         x = x + 1
正しい： x = 1
         y = x + 1

複数のケースを持つ関数は、定義を続けて並べて書きます：
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

使うファイルに付けた名前（`use geometry`、`use ... as geo`）は、そのファイルの
中で他のものの名前にはできません。また、引用符付きで使う 2 つのファイル
（`use "a.ry"`）が同じ名前を定義することもできません：代わりに名前で使います
（`use a` の後に `a.area`）。"#,
        ),
        (
            r#"E012 — Number too large (no longer given)

Whole numbers (Int) have no limit any more: they grow as needed, and stay
exact. Earlier versions of Rythme stopped with this error beyond 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (a Float is approximate)

Only a whole number of more than 100000 digits stops the program, with
E023: it is usually a calculation that grows without end."#,
            r#"E012 — 数が大きすぎる（現在は出ない）

整数（Int）にはもう上限がありません：必要なだけ大きくなり、正確なままです。
以前の Rythme は 64 ビットを超えるとこのエラーで止まっていました。

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30（Float は近似値）

プログラムが止まるのは 100000 桁を超える整数だけで、E023 になります：
たいていは際限なく大きくなる計算です。"#,
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
            r#"E013 — unwrap が値を見つけられなかった

`unwrap(x)` は Some(...) や Ok(...) から値を取り出し、None や Err(...) なら
プログラムを止めます。代わりに値がない場合を扱ってください：

  match get(list, 5)
    Some(x) -> x
    None -> 0
  end
または単に：  unwrap_or(get(list, 5), 0)"#,
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
            r#"E014 — 入れ子の呼び出しが多すぎる

関数が終わらないまま自分自身（または他の関数）を何度も呼んでいます。
基底ケースに到達しないか、再帰呼び出しが最後に行う処理（末尾呼び出し）に
なるように書き直す必要があります：末尾呼び出しには制限がありません。

制限あり：   fn sum([]) = 0
             fn sum([x, ..rest]) = x + sum(rest)       # + は後で計算される
制限なし：   fn sum([], acc) = acc
             fn sum([x, ..rest], acc) = sum(rest, acc + x)
最も簡単：   sum(list)  または  fold(list, 0, fn(a, x) = a + x)"#,
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
            r#"E015 — アサーション失敗

`assert(condition)` か `assert_eq(actual, expected)` が、何かが期待どおりで
ないことを見つけました。テストの中なら、テストは失敗します：メッセージに
比べる値が表示されます。

決して成功しない `assert_eq` は、2 つの値の種類が決して等しくならない
ものなら、`rythme check` が実行前に見つけます：
誤り：   assert_eq(parse_int("3"), 3)      # Result は決して Int にならない
正しい： assert_eq(parse_int("3"), Ok(3))

`check_all(property, generator)` も、ある値でプロパティが偽になると E015 を
報告します：見つかったいちばん単純な値と、それを生んだシードを表示します
（毎回同じ値が出てきます。`rythme test --seed N` で別の値を試せます）。"#,
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
            r#"E016 — ? の使い方の誤り

`x?` は Some(...) や Ok(...) から値を取り出し、None や Err(...) なら今の関数から
すぐにそれを返します。そのため、関数の中で、Option か Result に対してしか
使えません。

  fn total(a, b)
    x = parse_int(a)?
    y = parse_int(b)?
    Ok(x + y)
  end"#,
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
            r#"E017 — ファイルの問題

ファイルを読めなかったか、`use` に渡したファイルが見つからなかったか、
ファイルどうしが循環して互いを使っています。`use` のパスは、`use` を含む
ファイルのフォルダーからの相対パスです。プログラムは UTF-8（エディターの
普通の設定）で保存したテキストファイルです。

`input()` も読むものがなくなるとこのエラーで止まります：キーボード入力が
閉じられた（Ctrl-D）か、プログラムに送ったテキスト
（`rythme prog.ry < answers.txt`）にもう行がない場合です。"#,
        ),
        (
            r#"E018 — Unexpected character

This character has no meaning in Rythme here. Texts use straight double
quotes "...", powers use **, and comments start with #."#,
            r#"E018 — 予期しない文字

この文字はここでは Rythme で意味を持ちません。テキストにはまっすぐな
二重引用符 "..." を、べき乗には ** を使い、コメントは # で始めます。"#,
        ),
        (
            r#"E019 — if without else

In Rythme, `if` is an expression: it always gives a value, so both cases are
needed. Each branch holds a single expression; use `do ... end` for several.

Wrong:   if x > 0 then print("positive")
Right:   if x > 0 then print("positive") else ()
Right:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
            r#"E019 — else のない if

Rythme の `if` は式です：必ず値を返すので、両方のケースが必要です。各分岐は
1 つの式だけです。複数行なら `do ... end` を使います。

誤り：   if x > 0 then print("正")
正しい： if x > 0 then print("正") else ()
正しい： if x > 0 then "正" else if x == 0 then "ゼロ" else "負""#,
        ),
        (
            r#"E020 — Missing end

A function written on several lines, a `match` or a `do` block must finish
with `end` on its own line.

  fn greet(name)
    print("Hello {name}")
  end"#,
            r#"E020 — end がない

複数行で書いた関数、`match`、`do` ブロックは、単独の行の `end` で
終えなければなりません。

  fn greet(name)
    print("こんにちは {name}")
  end"#,
        ),
        (
            r#"E021 — Program stopped by fail

The program called `fail(message)`, which stops everything. For errors that
the caller could handle, return Err(message) instead."#,
            r#"E021 — fail によるプログラムの停止

プログラムが `fail(message)` を呼び、すべてを止めました。呼び出し側が扱える
エラーには、代わりに Err(message) を返してください。"#,
        ),
        (
            r#"E022 — Impossible conversion

The text does not contain a number. When the text comes from a user or a
file, use the safe versions, which give a Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
            r#"E022 — 変換できない

テキストに数が含まれていません。テキストがユーザーやファイルから来るときは、
Result を返す安全な版を使ってください：

  match parse_int(input("年齢は？ "))
    Ok(age) -> print("来年：{age + 1}")
    Err(e) -> print(e)
  end"#,
        ),
        (
            r#"E023 — Invalid value

The function received a value of the right kind, but outside of what it
accepts: a negative count, the minimum of an empty list, the square root of
a negative number, a date that does not exist (30 February), a range, a
list or a text too large, a whole number of more than 100000 digits... The
message explains the limit."#,
            r#"E023 — 無効な値

関数は正しい種類の値を受け取りましたが、受け付ける範囲の外でした：負の
回数、空のリストの最小値、負の数の平方根、存在しない日付（2 月 30 日）、
大きすぎる範囲・リスト・テキスト、100000 桁を超える整数など。メッセージが
限界を説明します。"#,
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
            r#"E024 — 値が失われる

行が値を計算していますが、何もそれを使っていません：ブロックで値を返すのは
最後の行だけで、値は変わらないので、結果はただ失われます。

誤り：   total = price
         - discount              # 新しい行：別の文になる
正しい： total = price -
           discount              # 続けるには行を演算子で終える
誤り：   x + 1                   # x は変わらない
正しい： y = x + 1
値を表示するには print を使う：  print(x + 1)"#,
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
            r#"E025 — プロジェクトまたはパッケージの問題

プロジェクトの `rythme.toml`（または `rythme install` が書いた `rythme.lock`）
を読めないか、依存関係がインストールされていないか、`rythme install` が
それをダウンロードできないか、パッケージインデックス（`rythme search`、`add`、
`publish`）を読めない、または求めたものがありません。`rythme.toml` は
次のような形です：

  name = "hello"                 # 小文字の Rythme の名前
  version = "0.1.0"              # 3 つの数
  entry = "src/main.ry"          # プログラム（デフォルトはこれ）

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }
  geometry = { version = "^1.2.0" }   # パッケージインデックスから

値はすべて引用符で囲んだテキストです。git からの依存関係には、固定の
バージョン（`tag` か `commit`）とプログラム `git`（https://git-scm.com）が
必要です。`[dependencies]` を変えたら、プロジェクトのフォルダーで
`rythme install` を実行してください。詳しくは `rythme help projects` と
`rythme help packages`。"#,
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
            r#"E026 — 無効な正規表現

正規表現（`matches`、`find_all`、`split_by`、`replace_all` などに渡すもの）が
規則のどれかに反しているか、`replace_all` の置換が存在しないグループを
使っています。メッセージがその場所を示します。

誤り：   find_all(text, "([0-9]+")       # `(` が閉じられていない
正しい： find_all(text, "([0-9]+)")
誤り：   split_by(text, "*")             # `*` は何も繰り返さない
正しい： split_by(text, "[*]")           # 記号 * そのものは角かっこに入れる
誤り：   replace_all(date, "(\\d+)-(\\d+)", "$3")    # グループは 2 つだけ
正しい： replace_all(date, "(\\d+)-(\\d+)", "$2/$1")

Rythme のテキストでは、バックスラッシュは 2 回書き（"\\d"）、波かっこは
\{ と \} と書きます（"[0-9]\{4\}"）。正規表現で表せることはすべて
`rythme help regex` にあります。正規表現がコードに書かれていれば、
`rythme check` が実行前に誤りを見つけます。"#,
        ),
        (
            r#"W001 — Unused name

A name is defined but never used. It is often a typo or a forgotten step.
If it is on purpose, start the name with `_`: `_unused = ...`."#,
            r#"W001 — 使われていない名前

名前が定義されていますが、一度も使われていません。打ち間違いか、忘れた
手順であることが多いです。意図的なら、名前を `_` で始めます：`_unused = ...`。"#,
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
            r#"W002 — 忘れたケース

`match`（または複数の定義を持つ関数）が、ありうる値をすべて扱っていません。
メッセージは足りないものを一覧します。その値が来ると、プログラムは E006 で
止まります。

  type Light = Red | Orange | Green
  fn next(Red) = Green
  fn next(Green) = Orange
  # W002：`next` は `Orange` を扱っていない。足りない定義を加える：
  fn next(Orange) = Red"#,
        ),
        (
            r#"W003 — Built-in function hidden

You defined a name that already belongs to a built-in function. In your file,
the name now means your definition. Choose another name to keep both."#,
            r#"W003 — 組み込み関数を隠している

組み込み関数の名前をあなたが定義しました。あなたのファイルでは、その名前は
以後あなたの定義を指します。両方を使うには別の名前を選んでください。"#,
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
            r#"W004 — 到達しないケース

ケースは順に試され、値は最初に当てはまるケースに進みます。ケース（または
関数の定義）が決して使われないのは次のときです：
  - それより前のケースがすべてを受け付ける（`_` やただの名前）：そのケースを
    最後に置く
  - それより前のケースがすでに同じ値を扱っている：`1 -> ...` が 2 回など
  - 値が決してその形にならない：Text は `[first, ..rest]` のようなリストの
    パターンに決してマッチせず、数は `Some(x)` に決してマッチしない
  - 範囲が空：`9..1` は数を 1 つも含まない。`1..9` と書く

  match name                     # `name` は Text
    [first, ..] -> first         # W004：Text はリストではない
    _ -> "?"
  end
テキストの文字を調べるには `chars(name)` を使います。"#,
        ),
        (
            r#"W005 — Unknown function after a dot

`value.name(args)` calls the function `name(value, args)`, or the field `name`
of a record. No function has this name, and no record of the file has such a
field: it is probably a typo.

Wrong:   [1, 2].lenght()
Right:   [1, 2].len()"#,
            r#"W005 — ドットの後の不明な関数

`value.name(args)` は関数 `name(value, args)` か、レコードのフィールド `name` を
呼び出します。この名前の関数はなく、ファイルのどのレコードにもそのような
フィールドはありません：おそらく打ち間違いです。

誤り：   [1, 2].lenght()
正しい： [1, 2].len()"#,
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
            r#"W006 — 常に偽になる比較

種類の違う値は決して等しくなりません：Rythme は値を黙って変換しません。
`x` が数なら、`x == "1"` は常に偽で、`x != "1"` は常に真です。変換を
忘れたか、Option の中の値を外にあるかのように比べていることが多いです。

誤り：   if count == "3" then ...            （数とテキスト）
正しい： if count == 3 then ...
誤り：   get(scores, "ada") == 19            （Option と数）
正しい： get(scores, "ada") == Some(19)
正しい： unwrap_or(get(scores, "ada"), 0) == 19"#,
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
            r#"W007 — 決して失敗しないテスト

テストが失敗するのは、エラーで止まったときか、`false`、`None`、`Err(...)` で
終わったときだけです。決して止まらない関数で値を計算したり表示したり
するだけで、期待値と比べないテストは、決して失敗しません：何もチェック
していません。

誤り：   test "長さ" = len([1, 2])
誤り：   test "長さ" = print(len([1, 2]))
正しい： test "長さ" = assert_eq(len([1, 2]), 2)
正しい： test "長さ" = len([1, 2]) == 2"#,
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
            r#"W008 — 結果が失われる

Rythme では値は変わりません：`push(list, 4)` は `list` に 4 を加えるのでは
なく、新しいリストを返します。そのような関数を呼ぶだけで、返したものを
何も使わない行は、まったく何もしません。

誤り：   push(numbers, 4)               # `numbers` は変わらない
正しい： more = push(numbers, 4)
誤り：   name.upper()
正しい： print(name.upper())"#,
        ),
        (
            r#"W009 — A function shown instead of its value

A function shown in a text or printed appears as `<fn name>`. To show what
it gives, call it, with parentheses, even when it takes no argument.

  fn greeting() = "hello"
Wrong:   print("{greeting}")          # shows <fn greeting>
Right:   print("{greeting()}")        # shows hello"#,
            r#"W009 — 値ではなく関数を表示している

テキストに入れたり表示したりした関数は `<fn name>` と表示されます。関数が
返す値を表示するには、引数がなくても括弧を付けて呼び出してください。

  fn greeting() = "こんにちは"
誤り：   print("{greeting}")          # <fn greeting> と表示される
正しい： print("{greeting()}")        # こんにちは と表示される"#,
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
            r#"W010 — 重なる範囲

範囲のパターン `a..b` は `a` から `b` まで（両端を含む）の数にマッチします。
`match`（または関数の定義）の 2 つの範囲が数を共有すると、その数はいつも
最初の範囲に進みます：2 つ目の範囲はそれを決して見ません。端を 2 回
書いていることが多いです。

誤り：   match age
           0..18 -> "子ども"
           18..65 -> "大人"          # W010：18 は "子ども" に進む
           _ -> "高齢者"
         end
正しい： 0..17 -> "子ども"
         18..64 -> "大人"
17 と 18 の間に入る小数には、代わりに比較を使います：
  a if a < 18 -> "子ども""#,
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
            r#"W011 — 呼び出しではなくインデックスや引き算になっている

文や結果の先頭では、`show [1, 2]` と `show -1` は括弧なしで `show` を
呼び出します。ただし、ファイルのどこかで `show` に値を与えている（`=` で、
関数やブロックの引数として、またはパターンで）場合は別です。そのときは、
ファイルのどこでも、`show` が関数である場所でも、インデックスと引き算に
なります。値は離れた場所で、別の関数の引数に与えられているかもしれません。

誤り：   fn show(xs) = print(xs)
         fn twice(show) = show * 2
         show [1, 2]                  # W011：関数のインデックスになる
正しい： show([1, 2])
または引数に別の名前を付ける：fn twice(n) = n * 2"#,
        ),
        ("Defines a function: `fn name(x) = expression`, or with its body on the following lines, finished by `end`. Without a name, `fn(x) = x * 2` is an anonymous function.", "関数を定義します：`fn name(x) = expression`、または本体を次の行から書いて `end` で終えます。名前なしの `fn(x) = x * 2` は無名関数です。"),
        ("Before `fn`, at the top level of a file: `async fn name(x) = ...` defines a function whose call starts it at once and gives a Task; `await task` gives its result. Tasks run at the same time, on other cores: start several, then await them. `async` is a keyword only before `fn`; elsewhere it is a name like any other.", "`fn` の前、ファイルの最上位に書きます：`async fn name(x) = ...` は、呼び出すとすぐに動き始めて Task を返す関数を定義します。`await task` はその結果を返します。タスクは別のコアで同時に動きます：いくつか開始してから、それらを await します。`async` は `fn` の前でだけキーワードで、それ以外の場所では普通の名前です。"),
        ("Before a value: `await task` waits for a task (the call of an `async fn`) and gives its result, like `await(task)`. An error in the task stops the program at the `await`. `await_all(tasks)` waits for a list of tasks.", "値の前に書きます：`await task` はタスク（`async fn` の呼び出し）を待ち、その結果を返します。`await(task)` と同じです。タスクの中でエラーが起きると、プログラムはその `await` で停止します。`await_all(tasks)` はタスクのリストを待ちます。"),
        ("Chooses between two values: `if condition then value else other`. Every `if` has an `else`, because an `if` always gives a value.", "2 つの値から選びます：`if condition then value else other`。`if` は必ず値を返すので、どの `if` にも `else` があります。"),
        ("Separates the condition of an `if` from the value it gives when the condition is true.", "`if` の条件と、条件が真のときに返す値を区切ります。"),
        ("Gives the value of an `if` when the condition is false. It is always required; `else if` chains several conditions.", "条件が偽のときの `if` の値を与えます。常に必要です。`else if` で複数の条件をつなげます。"),
        ("The opposite of `if`: `unless condition then a else b` gives `b` when the condition is true, `a` otherwise. Like `if`, it always has an `else`.", "`if` の逆：`unless condition then a else b` は、条件が真なら `b`、そうでなければ `a` を返します。`if` と同じく、必ず `else` があります。"),
        ("Chooses according to the shape of a value: one case per line, `pattern -> result`, finished by `end`. Cases are tried in order, and `rythme check` warns about forgotten cases.", "値の形に応じて選びます：1 行に 1 ケースで `pattern -> result` と書き、`end` で終えます。ケースは順に試され、`rythme check` は忘れたケースを警告します。"),
        ("Closes a block: a function written on several lines, a `match`, a `do` or a `test`.", "ブロックを閉じます：複数行で書いた関数、`match`、`do`、`test`。"),
        ("Groups several steps inside an expression, up to `end`. The last line gives the value. After a call, `do |x| ... end` is a block: a function given as the last argument.", "式の中の複数のステップを `end` までまとめます。最後の行が値になります。呼び出しの後の `do |x| ... end` はブロック、つまり最後の引数として渡す関数です。"),
        ("Defines your own type and its variants. Each variant is a constructor, with or without fields.", "独自の型とそのバリアントを定義します。各バリアントはフィールドのある、またはないコンストラクターです。"),
        ("Defines a test, next to the code. `rythme test file.ry` runs the tests of the file.", "コードのそばにテストを定義します。`rythme test file.ry` でファイルのテストを実行します。"),
        ("Uses another file through its name: `use geometry`, then `geometry.area(2)`. The path is relative to the folder of the file that contains `use`.", "別のファイルを名前を通して使います：`use geometry` の後に `geometry.area(2)`。パスは `use` を含むファイルのフォルダーからの相対パスです。"),
        ("True when both conditions are true. The second one is computed only if the first one is true.", "両方の条件が真のときに真。2 つ目は 1 つ目が真のときだけ計算されます。"),
        ("True when at least one of the conditions is true. The second one is computed only if the first one is false.", "少なくとも 1 つの条件が真のときに真。2 つ目は 1 つ目が偽のときだけ計算されます。"),
        ("Inverts a condition: `not true` is `false`.", "条件を反転します：`not true` は `false`。"),
        ("One of the two values of the type Bool.", "Bool 型の 2 つの値の 1 つ。"),
        ("Shows values on the screen, separated by spaces, then goes to the next line.", "値を空白で区切って画面に表示し、改行します。"),
        ("Shows a value with its position in the code (on the error output) and returns it unchanged. Handy in the middle of a pipeline.", "値をコード内の位置とともに（エラー出力に）表示し、そのまま返します。パイプラインの途中で便利です。"),
        ("Converts any value to text. With a number of decimals, writes a number with exactly that many digits after the point.", "どんな値でもテキストに変換します。小数の桁数を指定すると、小数点以下をちょうどその桁数で書きます。"),
        ("Shows the prompt, then reads a line typed on the keyboard and returns it as text.", "プロンプトを表示し、キーボードで入力された 1 行を読んでテキストとして返します。"),
        ("Reads all the lines of the standard input, until its end: `Ok(list of texts)`, or `Err(message)` if it cannot be read. For `rythme count.ry < names.txt`, or text typed until Ctrl-D (Ctrl-Z then Enter on Windows).", "標準入力のすべての行を終わりまで読みます：`Ok(テキストのリスト)`、読めなければ `Err(message)`。`rythme count.ry < names.txt` や、Ctrl-D（Windows では Ctrl-Z の後に Enter）まで入力したテキストに使います。"),
        ("Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (for `()`), \"Symbol\" (for `:ok`), \"Option\", \"Result\", or the name of your own type.", "値の型の名前を返します：\"Int\"、\"Float\"、\"Text\"、\"Bool\"、\"List\"、\"Record\"、\"Function\"、\"Unit\"（`()` の場合）、\"Symbol\"（`:ok` の場合）、\"Option\"、\"Result\"、または独自の型の名前。"),
        ("Shows help: about a function (`help(map)`), a topic (`help(\"lists\")`), or what you can do with a value (`help([1, 2])`).", "ヘルプを表示します：関数について（`help(map)`）、トピックについて（`help(\"lists\")`）、値でできることについて（`help([1, 2])`）。"),
        ("Stops with an error if the condition is false. Mostly used in tests.", "条件が偽ならエラーで止まります。主にテストで使います。"),
        ("Stops with an error showing both values if they are different. Mostly used in tests.", "2 つの値が違えば、両方の値を示すエラーで止まります。主にテストで使います。"),
        ("Stops the program with an error message. Prefer returning `Err(...)` for errors that can be handled.", "エラーメッセージとともにプログラムを止めます。扱えるエラーには `Err(...)` を返すほうが適しています。"),
        ("Returns the list of the words given after the file name on the command line.", "コマンドラインでファイル名の後に与えた単語のリストを返します。"),
        ("Reads the options of the command line (the words of `args()`, or the list given) into a record, in a Result: `--name=value` gives the text `value`, `--name` and `-n` give `true`, a `-` in a name becomes `_`, and the other words are in the field `args` (all of them after `--`). An option given twice, or a word like `-abc`, gives `Err(message)`. Use `get(opts, \"name\")` or `has(opts, \"name\")` for an option that may be missing.", "コマンドラインのオプション（`args()` の単語、または渡したリスト）をレコードに読み込み、Result で返します：`--name=value` はテキスト `value` を、`--name` と `-n` は `true` を与え、名前の中の `-` は `_` になり、その他の単語はフィールド `args` に入ります（`--` の後はすべて）。2 回指定したオプションや `-abc` のような単語は `Err(message)` になります。ないかもしれないオプションには `get(opts, \"name\")` か `has(opts, \"name\")` を使います。"),
        ("The value of an environment variable of the system, as an Option: `Some(text)`, or `None` if it is not set.", "システムの環境変数の値を Option で返します：`Some(text)`、設定されていなければ `None`。"),
        ("Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not; Rythme itself exits with 1 after an error in the program (see `rythme help system`).", "プログラムをすぐに止めます。コード（デフォルトは 0）は、すべてうまくいったか（0）どうかをシステムに伝えます。プログラムでエラーが起きると Rythme 自身は 1 で終了します（`rythme help system` を参照）。"),
        ("Returns the current time in seconds (a Float), counted from 1 January 1970. Subtract two times to measure a duration.", "1970 年 1 月 1 日から数えた現在の時刻を秒（Float）で返します。2 つの時刻を引き算すると経過時間を測れます。"),
        ("random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.", "random() は 0 から 1 の小数、random(a, b) は a から b まで（両端を含む）の整数、random(list) はランダムな要素を返します。"),
        ("Absolute value: the number without its sign.", "絶対値：符号を取り除いた数。"),
        ("The smallest of the values, or of the elements of a list.", "値、またはリストの要素のうち最小のもの。"),
        ("The largest of the values, or of the elements of a list.", "値、またはリストの要素のうち最大のもの。"),
        ("Square root.", "平方根。"),
        ("x to the power n (same as `x ** n`).", "x の n 乗（`x ** n` と同じ）。"),
        ("Rounds down to a whole number.", "切り捨てて整数にします。"),
        ("Rounds up to a whole number.", "切り上げて整数にします。"),
        ("Rounds to the nearest whole number, or to the given number of decimal digits.", "いちばん近い整数に、または指定した小数の桁数に丸めます。"),
        ("Converts to a whole number (Int): decimals are cut, texts are read.", "整数（Int）に変換します：小数部分は切り捨てられ、テキストは読み取られます。"),
        ("Converts to a decimal number (Float).", "小数（Float）に変換します。"),
        ("Sine (x in radians).", "正弦（x はラジアン）。"),
        ("Cosine (x in radians).", "余弦（x はラジアン）。"),
        ("Tangent (x in radians).", "正接（x はラジアン）。"),
        ("Arc sine: the angle (in radians) whose sine is x, for x from -1 to 1.", "逆正弦：正弦が x になる角度（ラジアン）。x は -1 から 1 まで。"),
        ("Arc cosine: the angle (in radians) whose cosine is x, for x from -1 to 1.", "逆余弦：余弦が x になる角度（ラジアン）。x は -1 から 1 まで。"),
        ("Arc tangent: the angle (in radians) whose tangent is y. With two numbers, the angle of the point (x, y), from -pi to pi.", "逆正接：正接が y になる角度（ラジアン）。2 つの数を渡すと、点 (x, y) の角度（-pi から pi まで）。"),
        ("Logarithm: natural (base e) by default, or in the given base.", "対数：デフォルトは自然対数（底 e）、または指定した底。"),
        ("e to the power x.", "e の x 乗。"),
        ("Is the whole number even?", "整数は偶数か？"),
        ("Is the whole number odd?", "整数は奇数か？"),
        ("Greatest common divisor of two whole numbers: the largest number that divides both.", "2 つの整数の最大公約数：両方を割り切る最大の数。"),
        ("Least common multiple of two whole numbers: the smallest number divisible by both.", "2 つの整数の最小公倍数：両方で割り切れる最小の数。"),
        ("Length of a text (in characters), a list, a record, a dict or a set.", "テキスト（文字数）、リスト、レコード、辞書、集合の長さ。"),
        ("The text in uppercase.", "大文字にしたテキスト。"),
        ("The text in lowercase.", "小文字にしたテキスト。"),
        ("The text with its first character in uppercase (the rest is unchanged).", "最初の文字を大文字にしたテキスト（残りはそのまま）。"),
        ("The text without spaces and line breaks at the start and the end.", "先頭と末尾の空白と改行を取り除いたテキスト。"),
        ("Adds `fill` (a space by default) before the text until it is `width` characters long: numbers line up in columns. A value that is not a text is first shown as `print` shows it.", "テキストが `width` 文字になるまで、前に `fill`（デフォルトは空白）を加えます：数が列にそろいます。テキストでない値は、まず `print` と同じように表示した形にします。"),
        ("Adds `fill` (a space by default) after the text until it is `width` characters long.", "テキストが `width` 文字になるまで、後ろに `fill`（デフォルトは空白）を加えます。"),
        ("Cuts a text into a list of texts. Without separator, cuts at spaces.", "テキストを切ってテキストのリストにします。区切り文字がなければ空白で切ります。"),
        ("Glues the elements of a list into one text, with the separator between them.", "リストの要素を、間に区切り文字を入れて 1 つのテキストにつなげます。"),
        ("Does the text contain this piece of text? Does the list (or the set) contain this element? Does the record have this field, or the dict this key?", "テキストにこの部分テキストが含まれるか？ リスト（または集合）にこの要素が含まれるか？ レコードにこのフィールドが、辞書にこのキーがあるか？"),
        ("Does the text start with this piece of text?", "テキストがこの部分テキストで始まるか？"),
        ("Does the text end with this piece of text?", "テキストがこの部分テキストで終わるか？"),
        ("Replaces every occurrence of `old` by `new`.", "`old` をすべて `new` に置き換えます。"),
        ("The list of the characters of a text.", "テキストの文字のリスト。"),
        ("The list of the lines of a text.", "テキストの行のリスト。"),
        ("The number of a character (its Unicode code point).", "文字の番号（Unicode のコードポイント）。"),
        ("The character with this number (Unicode code point): the opposite of `char_code`.", "この番号（Unicode のコードポイント）の文字：`char_code` の逆。"),
        ("Is the text made only of digits from 0 to 9 (and not empty)?", "テキストが 0〜9 の数字だけでできているか（空でないか）？"),
        ("Is the text made only of letters, accented ones included (and not empty)?", "テキストが文字（アクセント付きも含む）だけでできているか（空でないか）？"),
        ("Is the text made only of spaces, tabs and line breaks (and not empty)?", "テキストが空白、タブ、改行だけでできているか（空でないか）？"),
        ("A text or a list repeated n times.", "n 回繰り返したテキストまたはリスト。"),
        ("Reads a whole number from a text: `Ok(number)`, or `Err(message)` if the text is not a number.", "テキストから整数を読みます：`Ok(数)`、テキストが数でなければ `Err(message)`。"),
        ("Reads a decimal number from a text: `Ok(number)` or `Err(message)`.", "テキストから小数を読みます：`Ok(数)` か `Err(message)`。"),
        ("Reads a JSON text: `Ok(value)`, or `Err(message)` if the text is not valid JSON. Objects become records, arrays lists, and null `None`.", "JSON テキストを読みます：`Ok(値)`、正しい JSON でなければ `Err(message)`。オブジェクトはレコードに、配列はリストに、null は `None` になります。"),
        ("Writes a value as JSON text: records become objects, lists arrays, and `None` null. With `indent`, one element per line, shifted by that many spaces.", "値を JSON テキストとして書きます：レコードはオブジェクトに、リストは配列に、`None` は null になります。`indent` を指定すると、1 行に 1 要素で、その数の空白だけずらします。"),
        ("The text without spaces and line breaks at the start.", "先頭の空白と改行を取り除いたテキスト。"),
        ("The text without spaces and line breaks at the end.", "末尾の空白と改行を取り除いたテキスト。"),
        ("The list of the words of a text: letters and digits, accents included, with `'` or `-` inside a word. Punctuation and spaces separate them.", "テキストの単語のリスト：英字と数字（アクセント付きも含む）で、単語の中の `'` や `-` も含みます。句読点と空白が区切りになります。"),
        ("Are the two texts the same when uppercase and lowercase letters count as equal?", "大文字と小文字を同じとみなしたとき、2 つのテキストは同じか？"),
        ("Writes a number for people: thousands separated, with exactly `decimals` decimals if given, in the style of a language: \"en\" (1,234.5, by default), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" or \"pt\" (1.234,5). To line it up in a column, use `pad_left`.", "人が読むための数を書きます：3 桁ごとに区切り、`decimals` を指定すればちょうどその桁数の小数で、言語の書き方に従います：\"en\"（1,234.5、デフォルト）、\"fr\"（1 234,5）、\"de\"、\"es\"、\"it\"、\"nl\"、\"pt\"（1.234,5）。列にそろえるには `pad_left` を使います。"),
        ("Does the whole text match the regex (a regular expression: see `rythme help regex`)? To look for the regex anywhere in the text, use `find_first`.", "テキスト全体が正規表現（`rythme help regex` を参照）にマッチするか？ テキストのどこかに正規表現があるか探すには `find_first` を使います。"),
        ("The first piece of the text that matches the regex, as an Option: `Some(piece)`, or `None` if there is none.", "正規表現にマッチするテキストの最初の部分を Option で返します：`Some(部分)`、なければ `None`。"),
        ("Every piece of the text that matches the regex, from left to right.", "正規表現にマッチするテキストのすべての部分を、左から右へ。"),
        ("The first match of the regex, as an Option of a list: the whole piece, then the text of each group `(...)`, in order (\"\" for a group that matched nothing).", "正規表現の最初のマッチを、リストの Option で返します：部分全体、続いて各グループ `(...)` のテキストを順に（何にもマッチしなかったグループは \"\"）。"),
        ("Like `captures`, for every match: a list of lists.", "`captures` と同じで、すべてのマッチについて：リストのリスト。"),
        ("The first match of a regex with named groups `(?<name>...)`, as an Option of a record: one field for each name.", "名前付きグループ `(?<name>...)` を持つ正規表現の最初のマッチを、レコードの Option で返します：名前ごとに 1 つのフィールド。"),
        ("Cuts a text at every piece that matches the regex.", "正規表現にマッチする部分ごとにテキストを切ります。"),
        ("Replaces every piece that matches the regex. `new` is a text, where `$1` stands for the text of group 1 (`$<name>` for a named group, `$0` for the whole piece, `$$` for a `$`), or a function that receives each piece and gives its replacement.", "正規表現にマッチする部分をすべて置き換えます。`new` はテキストで、`$1` はグループ 1 のテキストを表します（名前付きグループは `$<name>`、部分全体は `$0`、`$` そのものは `$$`）。または、各部分を受け取って置き換えを返す関数です。"),
        ("Applies the function f to each element and returns the list of the results. Also works on Option and Result, and on a set (giving the set of the results).", "各要素に関数 f を適用し、結果のリストを返します。Option と Result、集合にも使えます（集合なら結果の集合を返す）。"),
        ("Keeps only the elements for which f returns true. Also works on a set, and on a dict, where f receives each key and its value.", "f が true を返す要素だけを残します。集合にも、辞書にも使えます（辞書では f は各キーとその値を受け取る）。"),
        ("Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.", "すべての要素を 1 つの値にまとめます：`start` から始め、各要素について f(結果, 要素) を計算します。"),
        ("Like fold, but gives the list of every intermediate result: running totals, for example.", "fold と同じですが、途中の結果をすべてリストで返します：例えば累計。"),
        ("Calls f on each element, for its effect (like printing). Returns (). Also works on a set, and on a dict, where f receives each key and its value.", "各要素について、その効果（表示など）のために f を呼びます。() を返します。集合にも、辞書にも使えます（辞書では f は各キーとその値を受け取る）。"),
        ("The sum of the numbers of a list; with f, the sum of f(x) for each element.", "リストの数の合計。f を渡すと、各要素の f(x) の合計。"),
        ("The product of the numbers of a list.", "リストの数の積。"),
        ("The first element of a list (or character of a text), as an Option: `Some(x)`, or `None` if there is none. With n, the first n elements (like `take`).", "リストの最初の要素（またはテキストの最初の文字）を Option で返します：`Some(x)`、なければ `None`。n を渡すと、最初の n 個の要素（`take` と同じ）。"),
        ("The last element of a list (or character of a text), as an Option. With n, the last n elements.", "リストの最後の要素（またはテキストの最後の文字）を Option で返します。n を渡すと、最後の n 個の要素。"),
        ("The list (or text) without its first element.", "最初の要素を除いたリスト（またはテキスト）。"),
        ("The element at a position of a list or text, the field of a record, or the value of a key of a dict, as an Option (never an error). Negative positions count from the end.", "リストやテキストのある位置の要素、レコードのフィールド、辞書のキーの値を Option で返します（決してエラーにならない）。負の位置は末尾から数えます。"),
        ("A new list with x added at the end; or a new set with x added.", "x を末尾に加えた新しいリスト。または x を加えた新しい集合。"),
        ("A new list where the element at position i is replaced by x. Negative positions count from the end.", "位置 i の要素を x に置き換えた新しいリスト。負の位置は末尾から数えます。"),
        ("A new list with x inserted so that it is at position i (-1 adds it at the end).", "x が位置 i に来るように挿入した新しいリスト（-1 なら末尾に加える）。"),
        ("A new list without the element at position i. Negative positions count from the end.", "位置 i の要素を除いた新しいリスト。負の位置は末尾から数えます。"),
        ("The list or text in reverse order.", "逆順にしたリストまたはテキスト。"),
        ("The list sorted from smallest to largest (numbers, texts in alphabetical order...). For the largest first: `sort(list) |> reverse`.", "小さい順に並べたリスト（数、テキストはアルファベット順など）。大きい順にするには：`sort(list) |> reverse`。"),
        ("Sorts the list by the value that the function `key` gives for each element. Elements with the same value keep their order.", "関数 `key` が各要素に返す値でリストを並べ替えます。同じ値の要素は元の順序を保ちます。"),
        ("The list without duplicates (the first occurrence is kept).", "重複を取り除いたリスト（最初に出てきたものを残す）。"),
        ("The elements of the list in a random order.", "リストの要素をランダムな順にしたもの。"),
        ("The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.", "リストやテキストの、位置 `start`（含む）から `end`（含まない）までの部分。負の位置は末尾から数えます。"),
        ("The first n elements of a list or text.", "リストやテキストの最初の n 個の要素。"),
        ("The list or text without its first n elements.", "最初の n 個の要素を除いたリストやテキスト。"),
        ("The first elements of a list (or characters of a text), as long as f is true for them.", "f が真である間の、リストの最初の要素（またはテキストの最初の文字）。"),
        ("The list (or text) without its first elements for which f is true.", "f が真である最初の要素を除いたリスト（またはテキスト）。"),
        ("One element out of n of a list (or text), starting with the first. On a range: the numbers with a step.", "リスト（またはテキスト）の要素を、最初から n 個に 1 個取り出します。範囲に使うと、刻み幅付きの数になります。"),
        ("Cuts a list (or text) into pieces of n elements; the last piece may be shorter.", "リスト（またはテキスト）を n 個ずつの部分に切ります。最後の部分は短いこともあります。"),
        ("Every group of n neighbouring elements of a list (or text), in order: handy to compare each element with the next one.", "リスト（またはテキスト）の隣り合う n 個の要素のグループをすべて、順に返します：各要素を次の要素と比べるのに便利です。"),
        ("Pairs each element with its position: [[0, x0], [1, x1], ...].", "各要素をその位置と組にします：[[0, x0], [1, x1], ...]。"),
        ("Is f true for at least one element?", "少なくとも 1 つの要素で f が真か？"),
        ("Is f true for every element?", "すべての要素で f が真か？"),
        ("The first element for which f is true, as an Option.", "f が真になる最初の要素を Option で返します。"),
        ("How many elements make f true; without f, how many elements there are.", "f を真にする要素の数。f がなければ要素の数。"),
        ("The elements for which f is false: the opposite of `filter`. Also works on a set and on a dict, like `filter`.", "f が偽になる要素：`filter` の逆。`filter` と同じく、集合と辞書にも使えます。"),
        ("Calls f(element, position) for each element, in order (positions start at 0). For effects, like `print`.", "各要素について順に f(要素, 位置) を呼びます（位置は 0 から）。`print` のような効果のためのものです。"),
        ("The position of an element in a list (or of a piece of text in a text), as an Option.", "リストの中の要素の位置（またはテキストの中の部分テキストの位置）を Option で返します。"),
        ("The element for which the function `key` gives the smallest value (the first one if there are several).", "関数 `key` が最小の値を返す要素（複数あれば最初のもの）。"),
        ("The element for which the function `key` gives the largest value (the first one if there are several).", "関数 `key` が最大の値を返す要素（複数あれば最初のもの）。"),
        ("Joins a list of lists into one list.", "リストのリストを 1 つのリストにつなげます。"),
        ("Like map, when f returns a list: the lists are joined.", "map と同じで、f がリストを返すとき：それらのリストをつなげます。"),
        ("Splits a list in two: [elements where f is true, elements where f is false].", "リストを 2 つに分けます：[f が真の要素, f が偽の要素]。"),
        ("Groups the elements in a record: the key is the text of what `key` returns, the value the list of matching elements.", "要素をレコードにまとめます：キーは `key` が返すもののテキスト、値は当てはまる要素のリストです。"),
        ("Is the list, text, record, dict or set empty?", "リスト、テキスト、レコード、辞書、集合が空か？"),
        ("The list of the field names of a record, or of the keys of a dict (in order).", "レコードのフィールド名、または辞書のキーのリスト（順番どおり）。"),
        ("The list of the values of a record or of a dict (in the order of its keys), or of the elements of a set (in order).", "レコードや辞書の値のリスト（キーの順）、または集合の要素のリスト（順番どおり）。"),
        ("The list of the [name, value] pairs of a record, or of the [key, value] pairs of a dict.", "レコードの [名前, 値] のペア、または辞書の [キー, 値] のペアのリスト。"),
        ("Builds a record from a list of [name, value] pairs: the opposite of `entries`.", "[名前, 値] のペアのリストからレコードを作ります：`entries` の逆。"),
        ("Does the record have this field? Does the dict have this key, or the set this element?", "レコードにこのフィールドがあるか？ 辞書にこのキーが、集合にこの要素があるか？"),
        ("A new record with the field set to the value (added or replaced); or a new dict with the key set to the value.", "フィールドに値を設定した（追加または置き換えた）新しいレコード。またはキーに値を設定した新しい辞書。"),
        ("A new record without this field; or a new dict without this key, or a new set without this element.", "このフィールドを除いた新しいレコード。またはこのキーを除いた新しい辞書、この要素を除いた新しい集合。"),
        ("A new record (or dict) where f is applied to every value; the names (or keys) stay the same.", "すべての値に f を適用した新しいレコード（または辞書）。名前（またはキー）は同じままです。"),
        ("A dictionary: keys of any kind (but functions), each with a value, kept in the order of the keys. Made from a list of [key, value] pairs (the last value of a key wins), from a record, or empty with `dict()`. Use it with get, put, has, remove, keys, values, entries, len, map_values, filter, union...", "辞書：（関数以外の）どんな種類のキーにもそれぞれ値があり、キーの順に保たれます。[キー, 値] のペアのリスト（同じキーは最後の値が優先）、レコードから作るか、`dict()` で空の辞書を作ります。get、put、has、remove、keys、values、entries、len、map_values、filter、union などと使います。"),
        ("A set: values (but functions) without repetition, kept in order. Made from a list, or empty with `set()`. Use it with has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.", "集合：（関数以外の）値を重複なしに、順番どおりに保ちます。リストから作るか、`set()` で空の集合を作ります。has、push、remove、len、values、filter、map、union、intersection、difference、is_subset と使います。"),
        ("The elements of two sets together; or the pairs of two dicts, where the value of b wins for a key in both.", "2 つの集合の要素を合わせたもの。または 2 つの辞書のペアで、両方にあるキーは b の値が優先されます。"),
        ("The elements of the set a that are also in b; or the pairs of the dict a whose key is in b.", "集合 a の要素のうち b にもあるもの。または辞書 a のペアのうちキーが b にあるもの。"),
        ("The elements of the set a that are not in b; or the pairs of the dict a whose key is not in b.", "集合 a の要素のうち b にないもの。または辞書 a のペアのうちキーが b にないもの。"),
        ("Is every element of the set a also in the set b?", "集合 a のすべての要素が集合 b にもあるか？"),
        ("The value inside `Some(x)` or `Ok(x)`. Stops the program with an error for `None` or `Err(e)`.", "`Some(x)` や `Ok(x)` の中の値。`None` や `Err(e)` ならエラーでプログラムを止めます。"),
        ("The value inside `Some(x)` or `Ok(x)`, or the default value for `None` or `Err(e)`.", "`Some(x)` や `Ok(x)` の中の値。`None` や `Err(e)` ならデフォルト値。"),
        ("Is it `Some(...)`?", "`Some(...)` か？"),
        ("Is it `None`?", "`None` か？"),
        ("Is it `Ok(...)`?", "`Ok(...)` か？"),
        ("Is it `Err(...)`?", "`Err(...)` か？"),
        ("Chains steps that may fail: for `Some(x)` or `Ok(x)`, gives f(x), which must be an Option or a Result; `None` and `Err(e)` stay as they are.", "失敗するかもしれないステップをつなげます：`Some(x)` や `Ok(x)` なら f(x) を返し、f(x) は Option か Result でなければなりません。`None` と `Err(e)` はそのままです。"),
        ("Turns an Option into a Result: `Some(x)` gives `Ok(x)`, and `None` gives `Err(error)`. Handy before `?`.", "Option を Result に変えます：`Some(x)` は `Ok(x)` に、`None` は `Err(error)` になります。`?` の前で便利です。"),
        ("For `Err(e)`, gives `Err(f(e))`; `Ok(x)` stays as it is. Handy to explain an error better.", "`Err(e)` なら `Err(f(e))` を返し、`Ok(x)` はそのままです。エラーをよりよく説明するのに便利です。"),
        ("Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.", "テキストファイル全体を読みます：`Ok(テキスト)`、読めなければ `Err(message)`。"),
        ("Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.", "テキストをファイルに書きます（内容を置き換える）：`Ok(())` か `Err(message)`。"),
        ("Adds a text at the end of a file, creating the file if needed: `Ok(())` or `Err(message)`.", "ファイルの末尾にテキストを加えます。必要ならファイルを作ります：`Ok(())` か `Err(message)`。"),
        ("Does this file (or folder) exist? Always `false` in the browser.", "このファイル（またはフォルダー）があるか？ ブラウザーでは常に `false`。"),
        ("What is known about a file or a folder: `Ok({is_folder, size})`, with its size in bytes (0 for a folder), or `Err(message)` if it does not exist or cannot be read.", "ファイルやフォルダーについてわかること：`Ok({is_folder, size})`（サイズはバイト単位で、フォルダーは 0）、存在しないか読めなければ `Err(message)`。"),
        ("The names of the files and folders inside a folder, in alphabetical order: `Ok(list of texts)`, or `Err(message)`. `list_folder(\".\")` lists the current folder.", "フォルダーの中のファイルとフォルダーの名前をアルファベット順に：`Ok(テキストのリスト)` か `Err(message)`。`list_folder(\".\")` は現在のフォルダーを一覧します。"),
        ("Creates a folder, and the folders around it that are missing: `Ok(())`, also when it already exists, or `Err(message)`.", "フォルダーと、足りない上位フォルダーを作ります：すでにあっても `Ok(())`、または `Err(message)`。"),
        ("Deletes one file (never a folder: that is an error): `Ok(())` or `Err(message)`. A deleted file cannot be brought back.", "ファイルを 1 つ削除します（フォルダーは削除せず、エラーになる）：`Ok(())` か `Err(message)`。削除したファイルは元に戻せません。"),
        ("Joins the parts of a path with `/`, which works on Linux, macOS and Windows (with `\\` if the parts already use only `\\`). Only the first part can start at a root (`/`, `C:`): a part like \"/etc\" after another one is an error.", "パスの部分を `/` でつなげます。これは Linux、macOS、Windows で使えます（部分がすでに `\\` だけを使っていれば `\\` で）。ルート（`/`、`C:`）から始められるのは最初の部分だけです：他の部分の後に \"/etc\" のような部分があるとエラーになります。"),
        ("The parts of a path, split at each `/` or `\\`; the first part of an absolute path is its root (`/`, `C:\\`...). `join_path` puts them back together.", "パスを `/` か `\\` ごとに分けた部分。絶対パスの最初の部分はそのルートです（`/`、`C:\\` など）。`join_path` で元に戻せます。"),
        ("The date and time now, where the program runs: a record {year, month, day, hour, minute, second, offset}, where `offset` is the number of minutes the local time is ahead of UTC (0 when the system does not tell).", "プログラムが動いている場所の現在の日付と時刻：レコード {year, month, day, hour, minute, second, offset}。`offset` はローカル時刻が UTC より進んでいる分数です（システムがわからなければ 0）。"),
        ("The date of today, where the program runs: a record {year, month, day}.", "プログラムが動いている場所の今日の日付：レコード {year, month, day}。"),
        ("Is the value a valid date: a record {year, month, day} (with `hour`, `minute`, `second`, `offset` if needed) for a day that exists?", "値が有効な日付か：存在する日を表すレコード {year, month, day}（必要なら `hour`、`minute`、`second`、`offset` 付き）か？"),
        ("The date n days later (or earlier, when n is negative), at the same time of day.", "n 日後（n が負なら前）の、同じ時刻の日付。"),
        ("The date and time n seconds later (or earlier, when n is negative). Add `60 * minutes` or `3600 * hours` the same way.", "n 秒後（n が負なら前）の日付と時刻。`60 * minutes` や `3600 * hours` も同じように加えます。"),
        ("The number of whole days from date a to date b: negative when b is before a.", "日付 a から日付 b までの丸一日の数：b が a より前なら負。"),
        ("The number of seconds from date a to date b, taking their offsets into account: negative when b is before a.", "時差を考慮した、日付 a から日付 b までの秒数：b が a より前なら負。"),
        ("The day of the week of a date, from 1 (Monday) to 7 (Sunday), as in ISO 8601.", "日付の曜日を、ISO 8601 と同じく 1（月曜日）から 7（日曜日）で返します。"),
        ("The number of days of a month (from 1 to 12) of a year: 28 to 31.", "ある年のある月（1〜12）の日数：28〜31。"),
        ("Does the year have a 29 February? Every 4 years, except every 100 years, except every 400 years.", "その年に 2 月 29 日があるか？ 4 年ごと。ただし 100 年ごとは除き、400 年ごとは含む。"),
        ("Checks that a property holds for many values made by a generator (100, or `cases`): the function is called with each value and must give `true` (or `()`, `Some(...)`, `Ok(...)`). When a value makes it false or stops it with an error, that value is simplified as much as possible (shrinking) and shown, with the seed that made it: the same values come back on every run. The property may also come last, as a block: `check_all(ints()) { |x| x + 0 == x }`.", "ジェネレーターが作る多くの値（100 個、または `cases`）でプロパティが成り立つか確かめます：関数は各値で呼ばれ、`true`（または `()`、`Some(...)`、`Ok(...)`）を返さなければなりません。ある値で偽になるかエラーで止まると、その値はできるだけ単純化（シュリンク）され、それを生んだシードとともに表示されます：毎回同じ値が出てきます。プロパティはブロックとして最後に置くこともできます：`check_all(ints()) { |x| x + 0 == x }`。"),
        ("A list of `n` values (10 if not given) made by a generator, to see what `check_all` will try.", "ジェネレーターが作る `n` 個（指定しなければ 10 個）の値のリスト。`check_all` が何を試すかを見るのに使います。"),
        ("A generator of whole numbers from `min` to `max` (from -1000 to 1000 if not given), often near 0 and at the edges. A failing number is simplified towards 0.", "`min` から `max` まで（指定しなければ -1000 から 1000 まで）の整数のジェネレーター。0 の近くと両端の値がよく出ます。失敗した数は 0 に向けて単純化されます。"),
        ("A generator of decimal numbers from `min` to `max` (from -1000.0 to 1000.0 if not given).", "`min` から `max` まで（指定しなければ -1000.0 から 1000.0 まで）の小数のジェネレーター。"),
        ("A generator of texts of at most `max_length` characters (20 if not given): letters mostly, with digits, spaces, punctuation and accents. A failing text is made shorter and simpler.", "最大 `max_length` 文字（指定しなければ 20 文字）のテキストのジェネレーター：主に英字で、数字、空白、句読点、アクセントも入ります。失敗したテキストは短く、単純にされます。"),
        ("A generator of `true` and `false`.", "`true` と `false` のジェネレーター。"),
        ("A generator of lists of at most `max_length` elements (10 if not given), each made by `generator`. A failing list is made shorter, then its elements simpler.", "各要素を `generator` で作る、最大 `max_length` 個（指定しなければ 10 個）の要素のリストのジェネレーター。失敗したリストはまず短くされ、次に要素が単純にされます。"),
        ("A generator that picks one of the values of a list. A failing value is replaced by one written before it.", "リストの値の 1 つを選ぶジェネレーター。失敗した値はそれより前に書いた値に置き換えられます。"),
        ("A generator of records, with a generator for each field: `records_of({name: texts(), age: ints(0, 120)})`. Handy to give several values to `check_all`.", "フィールドごとにジェネレーターを持つレコードのジェネレーター：`records_of({name: texts(), age: ints(0, 120)})`。`check_all` に複数の値を渡すのに便利です。"),
        ("January", "1月"),
        ("February", "2月"),
        ("March", "3月"),
        ("April", "4月"),
        ("May", "5月"),
        ("June", "6月"),
        ("July", "7月"),
        ("August", "8月"),
        ("September", "9月"),
        ("October", "10月"),
        ("November", "11月"),
        ("December", "12月"),
        // docs/exercises: the exercises of `rythme exercise` (see src/exercise.rs)
        ("# --- Tests (do not change them) ---", "# --- テスト（変更しないでください）---"),
        ("to do", "未完成"),
        ("01-values", "01-values"),
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
            r#"# 練習問題 01 · 最初の値  ★☆☆
#
# 名前は `=` で値を受け取り、それをずっと持ち続けます。Rythme では値は
# 決して変わりません。
#
# 問題
#   `seconds_per_minute` は与えられています。下のそれぞれの `0`（と空の
#   テキスト）を、その前で定義した名前を使う式に置き換えてください：
#   - `seconds_per_hour`：1 時間の秒数
#   - `seconds_per_day`：1 日の秒数
#   - `seconds_per_week`：1 週間の秒数
#   - `summary`：テキスト "1 週間は 604800 秒です。"。ただし数は
#     `seconds_per_week` から取ります（自分で打ち込まないこと）。
#
# ヒント
#   1. 1 時間は 60 分です：`60 * seconds_per_minute`。
#   2. 1 日は 24 時間、1 週間は 7 日です。
#   3. テキストの中の `{name}` は `name` の値に置き換えられます：
#      "りんごを {count} 個持っています"。
#
# 答えをチェックする：  rythme exercise check 01-values.ry
#             または：  rythme test 01-values.ry

seconds_per_minute = 60
seconds_per_hour = 0
seconds_per_day = 0
seconds_per_week = 0
summary = ""

# --- テスト（変更しないでください）---

test "1 時間の秒数" = assert_eq(seconds_per_hour, 3600)

test "1 日の秒数" = assert_eq(seconds_per_day, 86400)

test "1 週間の秒数" = assert_eq(seconds_per_week, 604800)

test "まとめ" = assert_eq(summary, "1 週間は 604800 秒です。")
"#,
        ),
        ("02-texts", "02-texts"),
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
            r#"# 練習問題 02 · テキスト  ★☆☆
#
# 関数は値を受け取り、新しい値を返します：
#   fn double(x) = x * 2
#
# 問題
#   テストが成功するように、下の 4 つの関数を書いてください：
#   - `greet(name)` は "Ada" に対して "こんにちは、Ada！" を返す
#   - `shout(text)` はテキストを大文字にして "!" を付けたものを返す
#   - `initials(first, last)` は "Ada" と "Lovelace" に対して "A.L." を返す
#   - `introduce(name, age)` は "Ada は 36 歳です。" を返す
#
# ヒント
#   1. "こんにちは、{name}！" はテキストに `name` の値を埋め込みます。
#   2. `upper("hi")` は "HI" で、`+` は 2 つのテキストをつなげます："HI" + "!"。
#   3. `word[0]` は `word` の最初の文字です。波かっこの中にも書けます：
#      "{word[0]}."。
#
# 答えをチェックする：  rythme exercise check 02-texts.ry
#             または：  rythme test 02-texts.ry

## `name` へのあいさつ。
fn greet(name) = fail("未完成")

## 大文字にしたテキストに "!" を付けたもの。
fn shout(text) = fail("未完成")

## 人のイニシャル。それぞれの後にドットを付ける。
fn initials(first, last) = fail("未完成")

## 人を紹介する文。
fn introduce(name, age) = fail("未完成")

# --- テスト（変更しないでください）---

test "あいさつ" = assert_eq(greet("Ada"), "こんにちは、Ada！")

test "別の人へのあいさつ" = assert_eq(greet("Alan"), "こんにちは、Alan！")

test "大声"
  assert_eq(shout("hello"), "HELLO!")
  assert_eq(shout("Stop"), "STOP!")
end

test "イニシャル"
  assert_eq(initials("Ada", "Lovelace"), "A.L.")
  assert_eq(initials("grace", "hopper"), "g.h.")
end

test "紹介" = assert_eq(introduce("Ada", 36), "Ada は 36 歳です。")
"#,
        ),
        ("03-arithmetic", "03-arithmetic"),
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
            r#"# 練習問題 03 · 算術  ★☆☆
#
# 数は整数（Int：3）か小数（Float：3.5）です。`/` は常に Float を返し
# （`7 / 2` は 3.5）、`//` は割り算して切り捨て（`7 // 2` は 3）、`%` は
# 余りを返します（`7 % 2` は 1）。
#
# 問題
#   - `square(x)`：x かける x
#   - `average(a, b)`：2 つの数の平均
#   - `fahrenheit(celsius)`：華氏の温度
#     （9 をかけて、5 で割り、32 を足す）
#   - `hypotenuse(a, b)`：残りの 2 辺が a と b である直角三角形の斜辺
#     （a² + b² の平方根）
#   - `minutes_and_seconds(total)`：秒数に対して "2 min 5 s"（125 の場合）
#     のようなテキスト
#
# ヒント
#   1. `sqrt(16)` は 4.0 です。
#   2. `125 // 60` は 2、`125 % 60` は 5 です。
#   3. `3 == 3.0` は真です：同じ値の Int と Float は等しくなります。
#
# 答えをチェックする：  rythme exercise check 03-arithmetic.ry
#             または：  rythme test 03-arithmetic.ry

fn square(x) = fail("未完成")

fn average(a, b) = fail("未完成")

fn fahrenheit(celsius) = fail("未完成")

fn hypotenuse(a, b) = fail("未完成")

fn minutes_and_seconds(total) = fail("未完成")

# --- テスト（変更しないでください）---

test "2 乗"
  assert_eq(square(3), 9)
  assert_eq(square(-4), 16)
  assert_eq(square(1.5), 2.25)
end

test "平均"
  assert_eq(average(2, 4), 3)
  assert_eq(average(1, 2), 1.5)
end

test "華氏"
  assert_eq(fahrenheit(0), 32)
  assert_eq(fahrenheit(100), 212)
  assert_eq(fahrenheit(-40), -40)
end

test "斜辺"
  assert_eq(hypotenuse(3, 4), 5)
  assert_eq(hypotenuse(5, 12), 13)
end

test "分と秒"
  assert_eq(minutes_and_seconds(125), "2 min 5 s")
  assert_eq(minutes_and_seconds(59), "0 min 59 s")
  assert_eq(minutes_and_seconds(600), "10 min 0 s")
end
"#,
        ),
        ("04-choosing", "04-choosing"),
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
            r#"# 練習問題 04 · `if` で選ぶ  ★☆☆
#
# `if condition then value else other_value` は式です：値を返すので、
# `else` は常に必要です。`else if` で複数の条件をつなげます。
#
# 問題
#   組み込み関数 `abs`、`min`、`max` を使わずに：
#   - `absolute(n)`：符号を取り除いた n（`absolute(-5)` は 5）
#   - `larger(a, b)`：2 つの数のうち大きいほう
#   - `grade(score)`：0〜100 の点数に対して、90 以上は "A"、80 以上は "B"、
#     70 以上は "C"、60 以上は "D"、60 未満は "F"
#   - `ticket_price(age)`：4 歳未満は 0、12 歳未満は 5、65 歳以上は 12、
#     それ以外は 10
#
# ヒント
#   1. `if n < 0 then -n else n`
#   2. いちばん高い評価から調べます：`if score >= 90 then "A" else if ...`。
#   3. 長い `if` は次の行に続けられます。各行は `else if` で始めます。
#
# 答えをチェックする：  rythme exercise check 04-choosing.ry
#             または：  rythme test 04-choosing.ry

fn absolute(n) = fail("未完成")

fn larger(a, b) = fail("未完成")

fn grade(score) = fail("未完成")

fn ticket_price(age) = fail("未完成")

# --- テスト（変更しないでください）---

test "絶対値"
  assert_eq(absolute(-5), 5)
  assert_eq(absolute(7), 7)
  assert_eq(absolute(0), 0)
end

test "大きいほう"
  assert_eq(larger(3, 8), 8)
  assert_eq(larger(8, 3), 8)
  assert_eq(larger(-1, -2), -1)
end

test "評価"
  assert_eq(grade(95), "A")
  assert_eq(grade(90), "A")
  assert_eq(grade(85), "B")
  assert_eq(grade(70), "C")
  assert_eq(grade(61), "D")
  assert_eq(grade(12), "F")
end

test "チケットの値段"
  assert_eq(ticket_price(2), 0)
  assert_eq(ticket_price(4), 5)
  assert_eq(ticket_price(11), 5)
  assert_eq(ticket_price(30), 10)
  assert_eq(ticket_price(65), 12)
end
"#,
        ),
        ("05-logic", "05-logic"),
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
            r#"# 練習問題 05 · 真か偽か  ★☆☆
#
# 比較（`==`、`!=`、`<`、`<=`、`>`、`>=`）は Bool、つまり `true` か `false` を
# 返します。`and`、`or`、`not` は Bool を組み合わせます。
#
# 問題
#   どの関数も Bool を返し、どれにも `if` は要りません：
#   - `is_multiple(n, d)`：n は d の倍数か？
#   - `is_between(x, low, high)`：x は low から high まで（両端を含む）か？
#   - `is_leap(year)`：4 の倍数の年はうるう年。ただし 100 の倍数で 400 の
#     倍数でない年は除く（2000 年はうるう年、1900 年は違う）。ライブラリにも
#     `is_leap_year` としてありますが、ここでは規則を自分で書きます
#   - `exclusive_or(a, b)`：a と b のちょうど一方だけが真か？
#
# ヒント
#   1. 余り `n % d` が 0 なら、n は d の倍数です。
#   2. `x >= low and x <= high`
#   3. うるう年の規則は最初の関数を使って書きます：
#      `is_multiple(year, 4) and (...)`。
#   4. 2 つの Bool は `!=` でも比べられます。
#
# 答えをチェックする：  rythme exercise check 05-logic.ry
#             または：  rythme test 05-logic.ry

fn is_multiple(n, d) = fail("未完成")

fn is_between(x, low, high) = fail("未完成")

fn is_leap(year) = fail("未完成")

fn exclusive_or(a, b) = fail("未完成")

# --- テスト（変更しないでください）---

test "倍数"
  assert(is_multiple(12, 3), "12 は 3 の倍数")
  assert(not is_multiple(12, 5), "12 は 5 の倍数ではない")
  assert(is_multiple(0, 7), "0 はどの数の倍数でもある")
end

test "範囲内"
  assert(is_between(5, 1, 10), "5 は 1 と 10 の間")
  assert(is_between(1, 1, 10), "両端を含む")
  assert(is_between(10, 1, 10), "両端を含む")
  assert(not is_between(11, 1, 10), "11 は大きすぎる")
  assert(not is_between(0, 1, 10), "0 は小さすぎる")
end

test "うるう年"
  assert_eq(is_leap(2024), true)
  assert_eq(is_leap(2023), false)
  assert_eq(is_leap(1900), false)
  assert_eq(is_leap(2000), true)
end

test "排他的論理和"
  assert_eq(exclusive_or(true, false), true)
  assert_eq(exclusive_or(false, true), true)
  assert_eq(exclusive_or(true, true), false)
  assert_eq(exclusive_or(false, false), false)
end
"#,
        ),
        ("06-cases", "06-cases"),
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
            r#"# 練習問題 06 · ケースごとに 1 つの定義  ★☆☆
#
# 関数は複数の定義で書くことができ、順に試されます。引数には値を書けて
# （`fn f(0) = ...`）、ガード（`=` の前の `if ...`）で条件を加えられます：
#   fn describe(0) = "ゼロ"
#   fn describe(n) if n < 0 = "負"
#   fn describe(_) = "正"
#
# 問題
#   - `plural(n, word)`：英語の複数形。1 なら "1 cat"、それ以外は "0 cats"、
#     "2 cats" など
#   - `fizzbuzz(n)`：n が 3 と 5 の両方の倍数なら "FizzBuzz"、それ以外の 3 の
#     倍数なら "Fizz"、それ以外の 5 の倍数なら "Buzz"、どれでもなければ数を
#     テキストにしたもの
#   - `day_kind(day)`："土曜日" と "日曜日" なら "週末"、それ以外のテキスト
#     なら "平日"
#
# ヒント
#   1. 特別なケースから始めます：`fn plural(1, word) = ...`、その次の行に
#      一般的なケースを書きます。
#   2. 順番が大事です："Fizz" と "Buzz" より先に "FizzBuzz" を調べます。
#   3. `text(15)` は "15" です。
#   4. `_` は何でも受け付けるので、普通は最後のケースにします。
#
# 答えをチェックする：  rythme exercise check 06-cases.ry
#             または：  rythme test 06-cases.ry

fn plural(n, word) = fail("未完成")

fn fizzbuzz(n) = fail("未完成")

fn day_kind(day) = fail("未完成")

# --- テスト（変更しないでください）---

test "複数形"
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

test "最初の 15 個" = assert_eq(1..15 |> map(fizzbuzz) |> join(" "), "1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz")

test "曜日の種類"
  assert_eq(day_kind("土曜日"), "週末")
  assert_eq(day_kind("日曜日"), "週末")
  assert_eq(day_kind("月曜日"), "平日")
end
"#,
        ),
        ("07-recursion", "07-recursion"),
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
            r#"# 練習問題 07 · 再帰  ★★☆
#
# Rythme にループはありません：繰り返すには、関数が自分自身を呼びます。
# 再帰関数にはどれも、止まる基底ケースと、問題を基底ケースに近づける
# 再帰ケースが必要です：
#   fn countdown(0) = "発射！"
#   fn countdown(n) = "{n} " + countdown(n - 1)
#
# 問題
#   - `factorial(n)`：1 × 2 × ... × n。`factorial(0)` は 1
#   - `sum_to(n)`：1 + 2 + ... + n。`sum_to(0)` は 0
#   - `power(x, n)`：x の n 乗（n ≥ 0）。`**` も `pow` も使わずに
#
# ヒント
#   1. `factorial(5)` は `5 * factorial(4)` です。
#   2. `power(x, 0)` はいくつでしょう？ `power(x, n)` を `power(x, n - 1)` と
#      比べるとどうでしょう？
#   3. 基底ケースに決して到達しない関数はエラー E014 で止まります：
#      呼び出しのたびに n が小さくなっているか確かめてください。
#
# 答えをチェックする：  rythme exercise check 07-recursion.ry
#             または：  rythme test 07-recursion.ry

fn factorial(n) = fail("未完成")

fn sum_to(n) = fail("未完成")

fn power(x, n) = fail("未完成")

# --- テスト（変更しないでください）---

test "階乗"
  assert_eq(factorial(0), 1)
  assert_eq(factorial(1), 1)
  assert_eq(factorial(5), 120)
  assert_eq(factorial(10), 3628800)
end

test "n までの和"
  assert_eq(sum_to(0), 0)
  assert_eq(sum_to(1), 1)
  assert_eq(sum_to(10), 55)
  assert_eq(sum_to(100), 5050)
end

test "べき乗"
  assert_eq(power(2, 0), 1)
  assert_eq(power(2, 10), 1024)
  assert_eq(power(3, 4), 81)
  assert_eq(power(1.5, 2), 2.25)
end
"#,
        ),
        ("08-digits", "08-digits"),
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
            r#"# 練習問題 08 · 数の各桁  ★★☆
#
# `n % 10` は n の最後の桁で、`n // 10` は最後の桁を除いた n です：
# 472 なら 2 と 47 です。
#
# 問題
#   整数 n ≥ 0 について、再帰を使って（数をテキストに変えずに）：
#   - `digit_count(n)`：n の桁数（0 は 1 桁）
#   - `digit_sum(n)`：n の各桁の和
#   - `reverse_number(n)`：桁を逆順にした数（1230 は 321 になる）
#
# ヒント
#   1. 10 未満の数は 1 桁です：これが基底ケースです。
#   2. `digit_sum(472)` は `2 + digit_sum(47)` です。
#   3. `reverse_number` には、引数を 1 つ増やした補助関数を書きます。増やす
#      引数はそれまでに作った数です：`reverse_from(n, built)`。ただし
#      `reverse_from(0, built) = built`。各ステップで n の最後の桁を `built` の
#      末尾に移します：`built * 10 + n % 10`。
#
# 答えをチェックする：  rythme exercise check 08-digits.ry
#             または：  rythme test 08-digits.ry

fn digit_count(n) = fail("未完成")

fn digit_sum(n) = fail("未完成")

fn reverse_number(n) = fail("未完成")

# --- テスト（変更しないでください）---

test "桁数"
  assert_eq(digit_count(0), 1)
  assert_eq(digit_count(7), 1)
  assert_eq(digit_count(472), 3)
  assert_eq(digit_count(1000000), 7)
end

test "各桁の和"
  assert_eq(digit_sum(0), 0)
  assert_eq(digit_sum(472), 13)
  assert_eq(digit_sum(99999), 45)
end

test "逆順の数"
  assert_eq(reverse_number(472), 274)
  assert_eq(reverse_number(1230), 321)
  assert_eq(reverse_number(5), 5)
end
"#,
        ),
        ("09-accumulators", "09-accumulators"),
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
            r#"# 練習問題 09 · 累積引数と末尾呼び出し  ★★☆
#
# `n + sum_to(n - 1)` では、足し算が呼び出しの終わりを待ちます：入れ子の
# 呼び出しが 100 万個になると、エラー E014（「入れ子の呼び出しが多すぎる」）で
# 止まります。再帰呼び出しが関数の最後に行う処理（末尾呼び出し）なら、
# 何も待たず、制限もありません。追加の引数（累積引数）が、それまでに
# 作った結果を運びます：
#   fn sum_from(0, total) = total
#   fn sum_from(n, total) = sum_from(n - 1, total + n)
#
# 問題
#   - `big_sum(n)`：1 + 2 + ... + n。n = 1000000 でも動くこと
#   - `collatz_steps(n)`：n ≥ 1 について、1 に着くまでのステップ数。1 ステップ
#     で、偶数は 2 で割り、奇数は 3 × n + 1 にする
#     （6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 は 8 ステップ）
#   - `fib(n)`：n 番目のフィボナッチ数（0, 1, 1, 2, 3, 5, 8...：それぞれ
#     直前の 2 つの和）。n = 90 でも速いこと
#
# ヒント
#   1. `big_sum(n)` は補助関数を呼ぶだけでよいです：`sum_from(n, 0)`。
#   2. `collatz_from(n, steps)`：n が 1 なら、答えは `steps` です。
#   3. `fib(n) = fib(n - 1) + fib(n - 2)` は正しいですが、同じ数を何度も
#      計算するのでとても遅いです。代わりに最後の 2 つの数を運びます：
#      `fib_from(n, a, b)`。a と b は隣り合うフィボナッチ数で、
#      `fib_from(n, 0, 1)` から始めます。
#
# 答えをチェックする：  rythme exercise check 09-accumulators.ry
#             または：  rythme test 09-accumulators.ry

fn big_sum(n) = fail("未完成")

fn collatz_steps(n) = fail("未完成")

fn fib(n) = fail("未完成")

# --- テスト（変更しないでください）---

test "大きな和"
  assert_eq(big_sum(10), 55)
  assert_eq(big_sum(1000000), 500000500000)
end

test "コラッツのステップ数"
  assert_eq(collatz_steps(1), 0)
  assert_eq(collatz_steps(6), 8)
  assert_eq(collatz_steps(27), 111)
end

test "フィボナッチ"
  assert_eq(0..10 |> map(fib), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55])
  assert_eq(fib(90), 2880067194370816120)
end
"#,
        ),
        ("10-divisors", "10-divisors"),
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
            r#"# 練習問題 10 · 約数と素数  ★★☆
#
# 再帰を使うと、答えが見つかるまで候補を 1 つずつ順に試すこともできます。
#
# 問題
#   - `my_gcd(a, b)`：0 以上の 2 つの整数の最大公約数。ユークリッドの互除法
#     を使う：a と 0 の最大公約数は a、a と b の最大公約数は b と `a % b` の
#     最大公約数（組み込みの `gcd` は使わない）
#   - `is_prime(n)`：n は素数（約数が 1 と自分自身だけの 2 以上の数）か？
#   - `next_prime(n)`：n より真に大きい最小の素数
#
# ヒント
#   1. ユークリッドの互除法はすでに 2 つのケースとして書かれています：
#      それをコードにしてください。
#   2. `is_prime` には、d、d + 1... と試す補助関数 `no_divisor_from(n, d)` を
#      書きます。`d * d > n` になったらすぐに止めてかまいません。
#   3. `next_prime(n)` は n + 1 を試します：素数ならそれが答え、そうでなければ
#      答えは `next_prime(n + 1)` です。
#
# 答えをチェックする：  rythme exercise check 10-divisors.ry
#             または：  rythme test 10-divisors.ry

fn my_gcd(a, b) = fail("未完成")

fn is_prime(n) = fail("未完成")

fn next_prime(n) = fail("未完成")

# --- テスト（変更しないでください）---

test "最大公約数"
  assert_eq(my_gcd(12, 18), 6)
  assert_eq(my_gcd(17, 5), 1)
  assert_eq(my_gcd(7, 0), 7)
  assert_eq(my_gcd(0, 9), 9)
end

test "素数"
  assert_eq(1..30 |> filter(is_prime), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
  assert_eq(is_prime(0), false)
  assert_eq(is_prime(7919), true)
  assert_eq(is_prime(7917), false)
end

test "次の素数"
  assert_eq(next_prime(1), 2)
  assert_eq(next_prime(13), 17)
  assert_eq(next_prime(100), 101)
end
"#,
        ),
        ("11-list-patterns", "11-list-patterns"),
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
            r#"# 練習問題 11 · リストを分解する  ★★☆
#
# リストは空の `[]` か、最初の要素とリストの残りが続く `[first, ..rest]` の
# どちらかです。リストを扱う関数は、普通この 2 つのケースそれぞれに 1 つずつ
# 定義を持ちます：
#   fn count_items([]) = 0
#   fn count_items([_, ..rest]) = 1 + count_items(rest)
#
# 問題
#   再帰を使い、組み込み関数 `len`、`sum`、`max`、`contains` を使わずに：
#   - `total(list)`：リストの数の合計
#   - `largest(list)`：空でないリストのいちばん大きい要素
#   - `has_value(list, x)`：x はリストの要素か？
#   - `last_one(list)`：空でないリストの最後の要素
#
# ヒント
#   1. 空のリストの合計は 0 です。
#   2. 要素が 1 つのリスト `[x]` は、`largest` と `last_one` のよい基底
#      ケースです。
#   3. `[x, ..rest]` のいちばん大きい要素は、x と rest のいちばん大きい要素の
#      うち大きいほうです。
#   4. `has_value`：空のリストは何も含みません。そうでなければ、最初の要素が
#      x か、x が残りにあるかです（`or`）。
#   5. `rythme check` は忘れたケース（ここでは空のリスト）を警告します
#      （W002）。空のリストにはいちばん大きい要素がありません：それを
#      `fn largest([]) = fail("リストが空です")` で伝えます。
#
# 答えをチェックする：  rythme exercise check 11-list-patterns.ry
#             または：  rythme test 11-list-patterns.ry

fn total(list) = fail("未完成")

fn largest(list) = fail("未完成")

fn has_value(list, x) = fail("未完成")

fn last_one(list) = fail("未完成")

# --- テスト（変更しないでください）---

test "合計"
  assert_eq(total([]), 0)
  assert_eq(total([4]), 4)
  assert_eq(total([1, 2, 3, 4]), 10)
end

test "いちばん大きい要素"
  assert_eq(largest([7]), 7)
  assert_eq(largest([3, 9, 2]), 9)
  assert_eq(largest([-3, -1, -2]), -1)
end

test "値を含むか"
  assert_eq(has_value([1, 2, 3], 2), true)
  assert_eq(has_value([1, 2, 3], 5), false)
  assert_eq(has_value([], 1), false)
  assert_eq(has_value(["a", "b"], "b"), true)
end

test "最後の要素"
  assert_eq(last_one([1, 2, 3]), 3)
  assert_eq(last_one(["ひとつ"]), "ひとつ")
end
"#,
        ),
        ("12-building-lists", "12-building-lists"),
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
            r#"# 練習問題 12 · リストを作る  ★★☆
#
# `[x, ..rest]` はリストを作ることもできます：x の後に rest の要素が続きます。
# リストは決して変わりません：関数は新しいリストを返します。
#   fn zeros(0) = []
#   fn zeros(n) = [0, ..zeros(n - 1)]
#
# 問題
#   再帰を使い、`map`、`reverse`、範囲（`1..n`）を使わずに：
#   - `countdown(n)`：リスト [n, n - 1, ..., 1, 0]
#   - `repeat_value(x, n)`：値 x を n 個並べたリスト
#   - `double_all(list)`：リストの各数を 2 倍にしたもの
#   - `reversed(list)`：要素を逆順にしたもの
#
# ヒント
#   1. `countdown(0)` は `[0]` です。
#   2. `double_all([x, ..rest])` は `x * 2` で始まります。
#   3. `reversed` には累積引数を使います：`reversed_onto(list, done)` は、list が
#      空になるまで、list の最初の要素を done の先頭に移します。
#      `reversed_onto([1, 2], [])` → `reversed_onto([2], [1])` →
#      `reversed_onto([], [2, 1])`。
#
# 答えをチェックする：  rythme exercise check 12-building-lists.ry
#             または：  rythme test 12-building-lists.ry

fn countdown(n) = fail("未完成")

fn repeat_value(x, n) = fail("未完成")

fn double_all(list) = fail("未完成")

fn reversed(list) = fail("未完成")

# --- テスト（変更しないでください）---

test "カウントダウン"
  assert_eq(countdown(0), [0])
  assert_eq(countdown(3), [3, 2, 1, 0])
end

test "値の繰り返し"
  assert_eq(repeat_value("a", 3), ["a", "a", "a"])
  assert_eq(repeat_value(1, 0), [])
end

test "すべて 2 倍"
  assert_eq(double_all([1, 2, 3]), [2, 4, 6])
  assert_eq(double_all([]), [])
end

test "逆順"
  assert_eq(reversed([1, 2, 3]), [3, 2, 1])
  assert_eq(reversed([]), [])
  assert_eq(reversed(["x"]), ["x"])
end

test "元のリストは変わらない"
  numbers = [1, 2, 3]
  reversed(numbers)
  assert_eq(numbers, [1, 2, 3])
end
"#,
        ),
        ("13-library", "13-library"),
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
            r#"# 練習問題 13 · リストの関数  ★★☆
#
# リストの処理のほとんどに再帰は要りません：組み込み関数がやってくれます。
# `1..5` はリスト [1, 2, 3, 4, 5]、`map(list, f)` は各要素に f を適用し、
# `filter(list, f)` は f が true を返す要素を残します。`sum`、`len`、`sort`、
# `reverse`、`take` など、`rythme help lists` で一覧できます。
#
# 問題
#   - `squares(n)`：リスト [1, 4, 9, ..., n × n]
#   - `evens(list)`：リストの偶数を同じ順で
#   - `sum_of_multiples(limit)`：1 から limit - 1 までの数のうち、3 か 5 の
#     倍数であるものの合計
#   - `mean(list)`：空でないリストの数の平均
#   - `smallest_three(list)`：小さいほうから 3 つの数
#
# ヒント
#   1. 自分で書いた関数 `square` を使って `map(1..n, square)`、または
#      `map(1..n, fn(x) = x * x)`（無名関数。練習問題 14 を参照）。
#   2. `is_even` は組み込み関数です：`filter(list, is_even)`。
#   3. `1..0` は空のリスト `[]` です。
#   4. `sort` してから `take`。
#
# 答えをチェックする：  rythme exercise check 13-library.ry
#             または：  rythme test 13-library.ry

fn squares(n) = fail("未完成")

fn evens(list) = fail("未完成")

fn sum_of_multiples(limit) = fail("未完成")

fn mean(list) = fail("未完成")

fn smallest_three(list) = fail("未完成")

# --- テスト（変更しないでください）---

test "2 乗"
  assert_eq(squares(5), [1, 4, 9, 16, 25])
  assert_eq(squares(0), [])
end

test "偶数" = assert_eq(evens([5, 2, 8, 1, 4]), [2, 8, 4])

test "倍数の和"
  assert_eq(sum_of_multiples(10), 23)
  assert_eq(sum_of_multiples(1000), 233168)
end

test "平均"
  assert_eq(mean([1, 2, 3, 4]), 2.5)
  assert_eq(mean([7]), 7)
end

test "小さいほうから 3 つ"
  assert_eq(smallest_three([5, 1, 4, 2, 3]), [1, 2, 3])
  assert_eq(smallest_three([9, 8]), [8, 9])
end
"#,
        ),
        ("14-anonymous-functions", "14-anonymous-functions"),
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
            r#"# 練習問題 14 · 無名関数  ★★☆
#
# `fn(x) = x * 2` は名前のない関数で、必要な場所にそのまま書きます。
# 周りの名前を使うこともできます：
#   fn add_to_all(list, n) = map(list, fn(x) = x + n)
#
# 問題
#   - `with_tax(prices, percent)`：各価格を `percent` % 上げたもの
#   - `long_words(words, n)`：n 文字より長い単語
#   - `lengths(words)`：各単語の長さ
#   - `all_positive(list)`：すべての数が 0 より大きいか？
#   - `count_above(list, limit)`：limit より大きい数の個数
#
# ヒント
#   1. 価格 p を 20 % 上げると `p + p * 20 / 100` です。
#   2. `len("word")` は 4 です。
#   3. `all(list, f)` はすべての要素で f が true を返すときに真で、
#      `count(list, f)` は f が true を返す要素を数えます。
#
# 答えをチェックする：  rythme exercise check 14-anonymous-functions.ry
#             または：  rythme test 14-anonymous-functions.ry

fn with_tax(prices, percent) = fail("未完成")

fn long_words(words, n) = fail("未完成")

fn lengths(words) = fail("未完成")

fn all_positive(list) = fail("未完成")

fn count_above(list, limit) = fail("未完成")

# --- テスト（変更しないでください）---

test "税込み"
  assert_eq(with_tax([100, 50], 20), [120, 60])
  assert_eq(with_tax([10], 0), [10])
end

test "長い単語" = assert_eq(long_words(["a", "tree", "is", "green"], 3), ["tree", "green"])

test "長さ" = assert_eq(lengths(["a", "tree", "is"]), [1, 4, 2])

test "すべて正"
  assert_eq(all_positive([1, 5, 3]), true)
  assert_eq(all_positive([1, -5, 3]), false)
  assert_eq(all_positive([]), true)
end

test "より大きい数の個数" = assert_eq(count_above([3, 10, 7, 12], 5), 3)
"#,
        ),
        ("15-functions-as-values", "15-functions-as-values"),
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
            r#"# 練習問題 15 · 関数は値  ★★☆
#
# 関数は別の関数に渡すことができ、関数が新しい関数を返すこともできます：
#   fn twice(f) = fn(x) = f(f(x))
#   add_two = twice(fn(x) = x + 1)      # add_two(5) は 7
#
# 問題
#   - `apply_twice(f, x)`：x に f を適用し、その結果にもう一度 f を適用する
#   - `compose(f, g)`：g を適用してから f を適用する新しい関数
#   - `adder(n)`：引数に n を足す新しい関数
#   - `my_map(list, f)` と `my_filter(list, f)`：再帰で書いた自分版の
#     `map` と `filter`
#
# ヒント
#   1. `compose(f, g)` は `fn(x) = ...` を返します。
#   2. `my_map([x, ..rest], f)` は `f(x)` で始まります。
#   3. `my_filter([x, ..rest], f)`：`f(x)` が真のときだけ x を残し、rest で
#      続けます。
#
# 答えをチェックする：  rythme exercise check 15-functions-as-values.ry
#             または：  rythme test 15-functions-as-values.ry

fn apply_twice(f, x) = fail("未完成")

fn compose(f, g) = fail("未完成")

fn adder(n) = fail("未完成")

fn my_map(list, f) = fail("未完成")

fn my_filter(list, f) = fail("未完成")

# --- テスト（変更しないでください）---

test "2 回適用"
  assert_eq(apply_twice(fn(x) = x * 3, 2), 18)
  assert_eq(apply_twice(upper, "hi"), "HI")
end

test "合成"
  plus_one_then_double = compose(fn(x) = x * 2, fn(x) = x + 1)
  assert_eq(plus_one_then_double(4), 10)
  assert_eq(compose(len, trim)("  abc  "), 3)
end

test "足し算器"
  add10 = adder(10)
  assert_eq(add10(5), 15)
  assert_eq(map([1, 2, 3], adder(100)), [101, 102, 103])
end

test "自分の map"
  assert_eq(my_map([1, 2, 3], fn(x) = x * x), [1, 4, 9])
  assert_eq(my_map([], fn(x) = x), [])
end

test "自分の filter"
  assert_eq(my_filter([1, 2, 3, 4, 5, 6], is_even), [2, 4, 6])
  assert_eq(my_filter(["a", "bb", "ccc"], fn(w) = len(w) > 1), ["bb", "ccc"])
end
"#,
        ),
        ("16-fold", "16-fold"),
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
            r#"# 練習問題 16 · リストを 1 つの値にたたみ込む  ★★☆
#
# `fold(list, start, f)` はすべての要素を 1 つの値にまとめます：`start` から
# 始め、各要素について `f(result, element)` を計算します。
#   fold([1, 2, 3], 0, fn(total, x) = total + x)     # 6
#
# 問題
#   `fold` を使い、再帰を使わずに：
#   - `product_of(list)`：数の積（空のリストなら 1）
#   - `longest_word(words)`：いちばん長い単語（同じ長さが複数あれば最初の
#     もの。空のリストなら ""）
#   - `to_number(digits)`：これらの数字で書いた数（[1, 2, 3] は 123）
#   - `count_where(list, f)`：f で true になる要素の数（`count` も `filter` も
#     使わずに）
#
# ヒント
#   1. `start` をよく選びます：それが空のリストに対する答えです。
#   2. `longest_word` では、途中の結果はそれまでに見たいちばん長い単語です：
#      それを残すか、新しい単語が真に長ければそれに替えます。
#   3. `to_number`：途中の結果が 12 で数字が 3 なら、新しい結果は
#      12 * 10 + 3 です。
#
# 答えをチェックする：  rythme exercise check 16-fold.ry
#             または：  rythme test 16-fold.ry

fn product_of(list) = fail("未完成")

fn longest_word(words) = fail("未完成")

fn to_number(digits) = fail("未完成")

fn count_where(list, f) = fail("未完成")

# --- テスト（変更しないでください）---

test "積"
  assert_eq(product_of([2, 3, 4]), 24)
  assert_eq(product_of([]), 1)
end

test "いちばん長い単語"
  assert_eq(longest_word(["a", "tree", "is", "green"]), "green")
  assert_eq(longest_word(["one", "two"]), "one")
  assert_eq(longest_word([]), "")
end

test "数にする"
  assert_eq(to_number([1, 2, 3]), 123)
  assert_eq(to_number([0, 7]), 7)
  assert_eq(to_number([]), 0)
end

test "条件に合う数"
  assert_eq(count_where([1, 2, 3, 4, 5], is_odd), 3)
  assert_eq(count_where(["a", "bb"], fn(w) = len(w) > 5), 0)
end
"#,
        ),
        ("17-pipelines", "17-pipelines"),
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
            r#"# 練習問題 17 · パイプライン  ★★☆
#
# `x |> f(a)` は `f(x, a)` という意味です：左側の値が最初の引数になります。
# ステップは、起きる順に左から右へ読みます：
#   [3, 1, 2] |> sort |> map(fn(x) = x * 10)        # [10, 20, 30]
# `|>` で始まる行は、前の行の続きです。
#
# 問題
#   それぞれの関数を 1 つのパイプラインで書いてください：
#   - `sum_of_odd_squares(n)`：1 から n までの奇数の 2 乗の和
#   - `top_three(scores)`：最高の 3 つの点数（最高点を先頭に）
#   - `shout_words(sentence)`：各単語を大文字にし、空白 1 つで区切ったもの
#     （"hello  world" は "HELLO WORLD" になる）
#   - `initials_of(sentence)`：各単語の最初の文字を大文字にしたもの
#     （"portable network graphics" は "PNG" になる）
#
# ヒント
#   1. `filter`、次に `map`、次に `sum`。
#   2. `sort`、`reverse`、`take(3)`。
#   3. `split(text)` はテキストを空白で切ります（続いた空白は 1 つとみなす）。
#      `join(words, " ")` は単語をつなぎ直します。
#   4. `word[0]` は `word` の最初の文字です。
#
# 答えをチェックする：  rythme exercise check 17-pipelines.ry
#             または：  rythme test 17-pipelines.ry

fn sum_of_odd_squares(n) = fail("未完成")

fn top_three(scores) = fail("未完成")

fn shout_words(sentence) = fail("未完成")

fn initials_of(sentence) = fail("未完成")

# --- テスト（変更しないでください）---

test "奇数の 2 乗の和"
  assert_eq(sum_of_odd_squares(5), 35)
  assert_eq(sum_of_odd_squares(1), 1)
end

test "上位 3 つ"
  assert_eq(top_three([12, 18, 9, 15, 20]), [20, 18, 15])
  assert_eq(top_three([4]), [4])
end

test "単語を大文字に" = assert_eq(shout_words("hello  world"), "HELLO WORLD")

test "頭文字"
  assert_eq(initials_of("portable network graphics"), "PNG")
  assert_eq(initials_of("Rythme"), "R")
end
"#,
        ),
        ("18-text-processing", "18-text-processing"),
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
            r#"# 練習問題 18 · テキストを扱う  ★★☆
#
# テキストは `chars` で文字のリストに変えられ、あとはリストの関数が
# やってくれます。`lower`、`upper`、`replace`、`split`、`join`、`contains` など
# は `rythme help text` で説明されています。
#
# 問題
#   - `is_palindrome(text)`：大文字と空白を無視して、テキストが前からも
#     後ろからも同じに読めるか？（"Never odd or even" は読める。）
#   - `count_vowels(text)`：母音（大文字か小文字の a、e、i、o、u、y）の文字は
#     いくつあるか？
#   - `title_case(text)`：各単語を大文字で始め、残りを小文字にしたもの
#     （"the LORD of the rings" は "The Lord Of The Rings" になる）
#
# ヒント
#   1. `replace(text, " ", "")` は空白を取り除きます。`reverse` はテキストにも
#      使えます。
#   2. `contains("aeiouy", c)` で、文字 c が母音かどうかがわかります。
#   3. `capitalize("lord")` は "Lord" です。
#
# 答えをチェックする：  rythme exercise check 18-text-processing.ry
#             または：  rythme test 18-text-processing.ry

fn is_palindrome(text) = fail("未完成")

fn count_vowels(text) = fail("未完成")

fn title_case(text) = fail("未完成")

# --- テスト（変更しないでください）---

test "回文"
  assert_eq(is_palindrome("kayak"), true)
  assert_eq(is_palindrome("Never odd or even"), true)
  assert_eq(is_palindrome("rythme"), false)
end

test "母音"
  assert_eq(count_vowels("Rythme"), 2)
  assert_eq(count_vowels("AEIOU"), 5)
  assert_eq(count_vowels("xkcd"), 0)
end

test "単語の頭を大文字に" = assert_eq(title_case("the LORD of the rings"), "The Lord Of The Rings")
"#,
        ),
        ("19-sorting", "19-sorting"),
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
            r#"# 練習問題 19 · 並べ替え  ★★★
#
# リストを並べ替える 2 つの古典的な方法を、再帰で書きます（組み込みの
# `sort` は使わずに）。
#
# 問題
#   - `insert(sorted, x)`：すでに並べ替えたリストの正しい位置に x を入れたもの
#   - `insertion_sort(list)`：空から始めた並べ替え済みのリストに、要素を
#     1 つずつ順に挿入してリストを並べ替える
#   - `merge(a, b)`：並べ替えた 2 つのリストから作った、並べ替えた 1 つのリスト
#   - `merge_sort(list)`：リストを半分に切り、それぞれを並べ替えてから
#     マージする
#
# ヒント
#   1. `insert([], x)` は `[x]` です。`insert([first, ..rest], x)` では、
#      x <= first なら x が前に来て、そうでなければ first が先頭のままです。
#   2. `insertion_sort([x, ..rest])` は `insert(insertion_sort(rest), x)` です
#      （または `fold` と `insert` を使う）。
#   3. `merge` は 2 つのリストの最初の要素を比べます：小さいほうが先に来て、
#      その後に残りをマージしたものが続きます。
#   4. `take(list, n)` と `drop(list, n)` で 2 つの半分が得られます
#      （`n = len(list) // 2`）。要素が 0 個か 1 個のリストはすでに並んでいます。
#
# 答えをチェックする：  rythme exercise check 19-sorting.ry
#             または：  rythme test 19-sorting.ry

fn insert(sorted, x) = fail("未完成")

fn insertion_sort(list) = fail("未完成")

fn merge(a, b) = fail("未完成")

fn merge_sort(list) = fail("未完成")

# --- テスト（変更しないでください）---

test "挿入"
  assert_eq(insert([1, 3, 5], 4), [1, 3, 4, 5])
  assert_eq(insert([], 2), [2])
  assert_eq(insert([1, 2], 9), [1, 2, 9])
end

test "挿入ソート"
  assert_eq(insertion_sort([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(insertion_sort([]), [])
end

test "マージ"
  assert_eq(merge([1, 4, 6], [2, 3, 7]), [1, 2, 3, 4, 6, 7])
  assert_eq(merge([], [1]), [1])
  assert_eq(merge([1], []), [1])
end

test "マージソート"
  assert_eq(merge_sort([5, 2, 9, 1, 5, 6]), [1, 2, 5, 5, 6, 9])
  assert_eq(merge_sort(["pear", "apple", "fig"]), ["apple", "fig", "pear"])
  assert_eq(merge_sort(reverse(1..100)), 1..100)
end
"#,
        ),
        ("20-records", "20-records"),
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
            r#"# 練習問題 20 · レコード  ★★☆
#
# レコードは名前の付いた値をまとめます：`{name: "Ada", born: 1815}`。
# `ada.name` はフィールドを読みます。レコードは決して変わりません：
# `{..ada, born: 1816}` は新しいレコードで、`born` だけが違う ada のコピーです。
# 引数でレコードを分解することもできます：`fn hello({name}) = "こんにちは、{name}"`。
#
# 問題
#   人はレコード `{name, born}`、点はレコード `{x, y}` です。
#   - `age_in(person, year)`：その年の終わりの、その人の年齢
#   - `label(person)`："Ada (1815)" のようなテキスト
#   - `renamed(person, name)`：名前だけ違う同じ人
#   - `moved(point, dx, dy)`：横に dx、縦に dy だけ動かした点
#   - `distance(a, b)`：2 点間の距離
#
# ヒント
#   1. `person.born`、または引数 `{born}`。
#   2. `{..person, name: name}`。これは `{..person, name}` とも書けます。
#   3. 距離は (xb - xa)² + (yb - ya)² の平方根です。
#
# 答えをチェックする：  rythme exercise check 20-records.ry
#             または：  rythme test 20-records.ry

fn age_in(person, year) = fail("未完成")

fn label(person) = fail("未完成")

fn renamed(person, name) = fail("未完成")

fn moved(point, dx, dy) = fail("未完成")

fn distance(a, b) = fail("未完成")

# --- テスト（変更しないでください）---

ada = {name: "Ada", born: 1815}

test "年齢" = assert_eq(age_in(ada, 1852), 37)

test "ラベル" = assert_eq(label(ada), "Ada (1815)")

test "名前の変更"
  assert_eq(renamed(ada, "Augusta"), {name: "Augusta", born: 1815})
  assert_eq(ada.name, "Ada")
end

test "移動"
  assert_eq(moved({x: 1, y: 2}, 3, -1), {x: 4, y: 1})
  assert_eq(moved({x: 0, y: 0}, 0, 0), {x: 0, y: 0})
end

test "距離"
  assert_eq(distance({x: 0, y: 0}, {x: 3, y: 4}), 5)
  assert_eq(distance({x: 1, y: 1}, {x: 1, y: 1}), 0)
end
"#,
        ),
        ("21-lists-of-records", "21-lists-of-records"),
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
            r#"# 練習問題 21 · レコードのリスト  ★★☆
#
# 実際のデータは、生徒のクラスや注文の一覧など、レコードのリストであることが
# 多いです。`map`、`filter`、`sort_by`、`max_by`、`group_by`、`sum` は、
# どんなリストと同じようにそれにも使えます。
#
# 問題
#   生徒はレコード `{name, score, group}`（点数は 0〜100）です。
#   - `names_of_passed(students)`：点数が 50 以上の生徒の名前を同じ順で
#   - `average_score(students)`：点数の平均
#   - `best(students)`：最高点の生徒の名前
#   - `ranking(students)`：最高点から最低点の順に並べた名前
#   - `group_sizes(students)`：グループごとの生徒の数を与えるレコード
#     （`{A: 2, B: 1}`）
#
# ヒント
#   1. `filter` の後に `map(fn(s) = s.name)`。
#   2. `max_by(list, key)` はキーが最大の要素を返します。
#   3. `sort_by(list, fn(s) = -s.score)` は最高点から並べます。
#   4. `group_by(list, key)` はリストのレコードを返し、
#      `map_values(record, f)` はその各値に f を適用します。
#
# 答えをチェックする：  rythme exercise check 21-lists-of-records.ry
#             または：  rythme test 21-lists-of-records.ry

fn names_of_passed(students) = fail("未完成")

fn average_score(students) = fail("未完成")

fn best(students) = fail("未完成")

fn ranking(students) = fail("未完成")

fn group_sizes(students) = fail("未完成")

# --- テスト（変更しないでください）---

classroom = [
  {name: "Ada", score: 92, group: "A"},
  {name: "Alan", score: 45, group: "B"},
  {name: "Grace", score: 78, group: "A"},
  {name: "Edsger", score: 50, group: "B"},
  {name: "Barbara", score: 85, group: "A"},
]

test "合格" = assert_eq(names_of_passed(classroom), ["Ada", "Grace", "Edsger", "Barbara"])

test "平均" = assert_eq(average_score(classroom), 70)

test "最高点" = assert_eq(best(classroom), "Ada")

test "順位" = assert_eq(ranking(classroom), ["Ada", "Barbara", "Grace", "Edsger", "Alan"])

test "グループの人数" = assert_eq(group_sizes(classroom), {A: 3, B: 2})
"#,
        ),
        ("22-own-types", "22-own-types"),
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
            r#"# 練習問題 22 · 独自の型  ★★☆
#
# `type` は、とりうる形ごとに 1 つのコンストラクターを持つ、新しい種類の値を
# 定義します：
#   type Pet = Dog(name) | Cat(name, lives) | Fish
# 関数はコンストラクターごとに 1 つの定義を持てます：
#   fn sound(Dog(_)) = "ワン"
#
# 問題
#   下の型 `Shape` には 3 つの形があります。
#   - `area(shape)`：図形の面積
#   - `perimeter(shape)`：周りの長さ
#   - `name_of(shape)`："円"、"長方形"、"正方形" のどれか
#   - `total_area(shapes)`：図形のリストの面積の合計
#
# ヒント
#   1. `fn area(Circle(r)) = pi * r * r`、その後に他のコンストラクターごとに
#      1 つずつ定義を書きます。
#   2. 円の周りの長さは 2 × π × r です。
#   3. コンストラクターを忘れると、`rythme check` がどれかを教えてくれます
#      （警告 W002）。
#
# 答えをチェックする：  rythme exercise check 22-own-types.ry
#             または：  rythme test 22-own-types.ry

type Shape = Circle(radius) | Rect(width, height) | Square(side)

fn area(shape) = fail("未完成")

fn perimeter(shape) = fail("未完成")

fn name_of(shape) = fail("未完成")

fn total_area(shapes) = fail("未完成")

# --- テスト（変更しないでください）---

test "面積"
  assert_eq(area(Rect(2, 3)), 6)
  assert_eq(area(Square(4)), 16)
  assert_eq(round(area(Circle(1)), 2), 3.14)
end

test "周りの長さ"
  assert_eq(perimeter(Rect(2, 3)), 10)
  assert_eq(perimeter(Square(4)), 16)
  assert_eq(round(perimeter(Circle(1)), 2), 6.28)
end

test "名前" = assert_eq([Circle(1), Rect(1, 2), Square(3)] |> map(name_of), ["円", "長方形", "正方形"])

test "面積の合計" = assert_eq(total_area([Rect(2, 3), Square(2), Rect(1, 1)]), 11)
"#,
        ),
        ("23-match", "23-match"),
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
            r#"# 練習問題 23 · `match` で選ぶ  ★★☆
#
# `match` は値をパターンと順に比べ、最初に当てはまったものの結果を返します：
#   fn describe(n) = match n
#     0 -> "ゼロ"
#     n if n < 0 -> "負"
#     _ -> "正"
#   end
# パターンには、数、テキスト、コンストラクター、リスト（`[]`、`[x]`、
# `[x, ..rest]`）、レコード（`{name}`）を使えます。
#
# 問題
#   - `next_light(light)`：次の信号。Green、Orange、Red の順で、また Green に戻る
#   - `can_go(light)`：Green のときだけ true
#   - `describe(list)`：[] なら "空"、[7] なら "1 つ：7"、[1, 2] なら
#     "2 つ：1 と 2"、それより長いリストなら "たくさん、最初は 5"
#   - `greeting(person)`：フィールド `title` のあるレコードなら
#     "こんにちは、Dr Hopper"。`name` だけなら "こんにちは、Grace"
#
# ヒント
#   1. 3 つの信号のそれぞれが `match` の 1 つのケースです。
#   2. `[a, b]` はちょうど 2 要素のリストに、`[first, ..]` は 1 要素以上の
#      リストにマッチします：より厳密なパターンを先に置きます。
#   3. `{title, last}` は少なくともこれらのフィールドを持つレコードにマッチします。
#
# 答えをチェックする：  rythme exercise check 23-match.ry
#             または：  rythme test 23-match.ry

type Light = Green | Orange | Red

fn next_light(light) = fail("未完成")

fn can_go(light) = fail("未完成")

fn describe(list) = fail("未完成")

fn greeting(person) = fail("未完成")

# --- テスト（変更しないでください）---

test "次の信号"
  assert_eq(next_light(Green), Orange)
  assert_eq(next_light(Orange), Red)
  assert_eq(next_light(Red), Green)
end

test "進めるか"
  assert_eq(can_go(Green), true)
  assert_eq(can_go(Orange), false)
  assert_eq(can_go(Red), false)
end

test "説明"
  assert_eq(describe([]), "空")
  assert_eq(describe([7]), "1 つ：7")
  assert_eq(describe([1, 2]), "2 つ：1 と 2")
  assert_eq(describe([5, 6, 7]), "たくさん、最初は 5")
end

test "あいさつ"
  assert_eq(greeting({title: "Dr", last: "Hopper", name: "Grace"}), "こんにちは、Dr Hopper")
  assert_eq(greeting({name: "Grace"}), "こんにちは、Grace")
end
"#,
        ),
        ("24-trees", "24-trees"),
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
            r#"# 練習問題 24 · 木  ★★★
#
# 型は自分自身を含められます。二分木は、空（`Leaf`）か、値と左右 2 つの
# 小さな木を持つノードです。木を扱う関数は型の形に従います：`Leaf` に 1 つ、
# 両側で自分自身を呼ぶ `Node` に 1 つのケースを書きます。
#
# 問題
#   - `size(tree)`：木が持つ値の数
#   - `total(tree)`：値の合計
#   - `depth(tree)`：段の数（Leaf は 0）
#   - `insert(tree, x)`：探索木とは、ノードの左の値がそのノードの値より
#     小さく、それ以外が右にある木です。x を加えた探索木を返します
#     （Leaf の場所に新しい Node を置く）
#   - `to_list(tree)`：値を左から右へ並べたもの
#
# ヒント
#   1. `fn size(Leaf) = 0` と `fn size(Node(left, _, right)) = ...`。
#   2. `max(a, b)` は 2 つの数のうち大きいほうを返します。
#   3. `insert` は新しい木を返します：x が左に行くなら
#      `Node(insert(left, x), value, right)`。
#   4. ノードの `to_list` は、左のリスト、値、右のリストの順です
#      （`+` はリストをつなげます）。
#
# 答えをチェックする：  rythme exercise check 24-trees.ry
#             または：  rythme test 24-trees.ry

type Tree = Leaf | Node(left, value, right)

fn size(tree) = fail("未完成")

fn total(tree) = fail("未完成")

fn depth(tree) = fail("未完成")

fn insert(tree, x) = fail("未完成")

fn to_list(tree) = fail("未完成")

# --- テスト（変更しないでください）---

#       5
#      / \
#     3   8
#    /
#   1
small = Node(Node(Node(Leaf, 1, Leaf), 3, Leaf), 5, Node(Leaf, 8, Leaf))

test "大きさ"
  assert_eq(size(Leaf), 0)
  assert_eq(size(small), 4)
end

test "合計" = assert_eq(total(small), 17)

test "深さ"
  assert_eq(depth(Leaf), 0)
  assert_eq(depth(small), 3)
end

test "挿入"
  assert_eq(insert(Leaf, 4), Node(Leaf, 4, Leaf))
  assert_eq(insert(Node(Leaf, 4, Leaf), 2), Node(Node(Leaf, 2, Leaf), 4, Leaf))
end

test "リストにする" = assert_eq(to_list(small), [1, 3, 5, 8])

test "木で並べ替え" = assert_eq([7, 2, 9, 4, 1] |> fold(Leaf, insert) |> to_list, [1, 2, 4, 7, 9])
"#,
        ),
        ("25-expressions", "25-expressions"),
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
            r#"# 練習問題 25 · 小さな電卓  ★★★
#
# (1 + 2) × 3 のような計算は木です：掛け算には足し算と 3 という 2 つの部分が
# あります。型を使うと、プログラムは計算を持ち、計算して、表示できます。
#
# 問題
#   - `evaluate(expr)`：計算の値
#   - `show(expr)`：計算をテキストにしたもの。足し算と掛け算はすべて括弧で
#     囲み（"((1 + 2) * 3)"）、符号反転は "-x" と書く（"-4"、"-(1 + 2)"）
#   - `count_numbers(expr)`：計算に含まれる数の個数
#   - `simplify(expr)`：同じ計算で、`Add(e, Num(0))`、`Mul(e, Num(1))`（と、
#     数が左にある同様の形）を、すべての段で e に置き換えたもの
#
# ヒント
#   1. コンストラクターごとに 1 つのケース：`fn evaluate(Num(n)) = n`、
#      `fn evaluate(Add(a, b)) = evaluate(a) + evaluate(b)`...
#   2. `show(Neg(e))` は "-" の後に `show(e)` が続いたものです。
#   3. `simplify` では、まず両方の部分を単純化してから、その結果を `match` で
#      調べます。
#
# 答えをチェックする：  rythme exercise check 25-expressions.ry
#             または：  rythme test 25-expressions.ry

type Expr = Num(value) | Add(left, right) | Mul(left, right) | Neg(inner)

fn evaluate(expr) = fail("未完成")

fn show(expr) = fail("未完成")

fn count_numbers(expr) = fail("未完成")

fn simplify(expr) = fail("未完成")

# --- テスト（変更しないでください）---

calculation = Mul(Add(Num(1), Num(2)), Num(3))

test "評価"
  assert_eq(evaluate(Num(4)), 4)
  assert_eq(evaluate(calculation), 9)
  assert_eq(evaluate(Neg(calculation)), -9)
end

test "表示"
  assert_eq(show(calculation), "((1 + 2) * 3)")
  assert_eq(show(Neg(Num(4))), "-4")
  assert_eq(show(Neg(Add(Num(1), Num(2)))), "-(1 + 2)")
end

test "数の個数" = assert_eq(count_numbers(Add(calculation, Neg(Num(5)))), 4)

test "単純化"
  assert_eq(simplify(Add(Num(7), Num(0))), Num(7))
  assert_eq(simplify(Mul(Num(1), Add(Num(0), Num(2)))), Num(2))
  assert_eq(simplify(calculation), calculation)
  assert_eq(simplify(Neg(Mul(Num(3), Num(1)))), Neg(Num(3)))
end
"#,
        ),
        ("26-option", "26-option"),
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
            r#"# 練習問題 26 · ないかもしれない値：Option  ★★☆
#
# Rythme に `null` はありません。何も見つからないかもしれない関数は Option を
# 返します：値があれば `Some(value)`、なければ `None` です。`match` で両方の
# ケースを扱い、`unwrap_or(option, default)` は値かデフォルト値を返します。
#   fn first_of([]) = None
#   fn first_of([x, ..]) = Some(x)
#
# 問題
#   - `safe_divide(a, b)`：`Some(a / b)`、b が 0 なら `None`
#   - `find_age(people, name)`：この名前の人の年齢を Option で（people は
#     レコード `{name, age}` のリスト）
#   - `age_or_unknown(people, name)`：テキスト。わかっている人なら "36"、
#     そうでなければ "不明"
#   - `first_long_word(words)`：5 文字より長い最初の単語を Option で
#   - `add_options(a, b)`：a が `Some(x)` で b が `Some(y)` なら `Some(x + y)`、
#     そうでなければ `None`
#
# ヒント
#   1. `find(list, f)` は f が真になる最初の要素を Option で返します。
#      `map(option, f)` は中に値があればそれに f を適用します。
#   2. `match find_age(people, name)` で、ケース `Some(age)` と `None` を書きます。
#   3. `match [a, b]` で、ケース `[Some(x), Some(y)]` とケース `_` を書きます。
#
# 答えをチェックする：  rythme exercise check 26-option.ry
#             または：  rythme test 26-option.ry

fn safe_divide(a, b) = fail("未完成")

fn find_age(people, name) = fail("未完成")

fn age_or_unknown(people, name) = fail("未完成")

fn first_long_word(words) = fail("未完成")

fn add_options(a, b) = fail("未完成")

# --- テスト（変更しないでください）---

people = [{name: "Ada", age: 36}, {name: "Alan", age: 41}]

test "安全な割り算"
  assert_eq(safe_divide(10, 4), Some(2.5))
  assert_eq(safe_divide(1, 0), None)
end

test "年齢を探す"
  assert_eq(find_age(people, "Alan"), Some(41))
  assert_eq(find_age(people, "Grace"), None)
end

test "年齢か不明"
  assert_eq(age_or_unknown(people, "Ada"), "36")
  assert_eq(age_or_unknown(people, "Grace"), "不明")
end

test "最初の長い単語"
  assert_eq(first_long_word(["a", "functional", "language"]), Some("functional"))
  assert_eq(first_long_word(["short", "words"]), None)
end

test "Option の足し算"
  assert_eq(add_options(Some(2), Some(3)), Some(5))
  assert_eq(add_options(Some(2), None), None)
  assert_eq(add_options(None, None), None)
end
"#,
        ),
        ("27-result", "27-result"),
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
            r#"# 練習問題 27 · 失敗するかもしれない操作：Result  ★★☆
#
# Rythme に例外はありません。失敗するかもしれない関数は Result を返します：
# うまくいけば `Ok(value)`、そうでなければ `Err(message)` です。呼び出し側は
# 両方のケースを見なければならないので、エラーを忘れることはありません。
#   parse_int("42")       # Ok(42)
#   parse_int("forty")    # Err("\"forty\" は整数ではない")
#
# 問題
#   - `checked_divide(a, b)`：`Ok(a / b)`、または `Err("ゼロ除算")`
#   - `parse_age(text)`：テキストに書かれた年齢を Result で：テキストが整数で
#     なければ `Err("数ではない")`、0 未満なら `Err("年齢は負にならない")`、
#     150 を超えたら `Err("年齢が大きすぎる")`、それ以外は `Ok(age)`
#   - `describe(result)`：`Ok(42)` なら "成功：42"、`Err("年齢が大きすぎる")`
#     なら "エラー：年齢が大きすぎる"
#   - `check_code(code)`：8 文字以上で数字を含めば `Ok(code)`。そうでなければ
#     最初に見つかった問題の Err："短すぎる" か "数字がない"
#
# ヒント
#   1. `match parse_int(text)` で、ケース `Err(_)`、`Ok(n) if n < 0`、
#      `Ok(n) if n > 150`、`Ok(n)` を書きます。
#   2. `any(chars(code), is_digit)` で数字があるかどうかがわかります。
#
# 答えをチェックする：  rythme exercise check 27-result.ry
#             または：  rythme test 27-result.ry

fn checked_divide(a, b) = fail("未完成")

fn parse_age(text) = fail("未完成")

fn describe(result) = fail("未完成")

fn check_code(code) = fail("未完成")

# --- テスト（変更しないでください）---

test "チェック付きの割り算"
  assert_eq(checked_divide(9, 3), Ok(3))
  assert_eq(checked_divide(1, 0), Err("ゼロ除算"))
end

test "年齢を読む"
  assert_eq(parse_age("36"), Ok(36))
  assert_eq(parse_age(" 7 "), Ok(7))
  assert_eq(parse_age("七"), Err("数ではない"))
  assert_eq(parse_age("-1"), Err("年齢は負にならない"))
  assert_eq(parse_age("200"), Err("年齢が大きすぎる"))
end

test "説明"
  assert_eq(describe(Ok(42)), "成功：42")
  assert_eq(describe(Err("年齢が大きすぎる")), "エラー：年齢が大きすぎる")
  assert_eq(describe(parse_age("-5")), "エラー：年齢は負にならない")
end

test "コードのチェック"
  assert_eq(check_code("rythme2024"), Ok("rythme2024"))
  assert_eq(check_code("abc1"), Err("短すぎる"))
  assert_eq(check_code("functional"), Err("数字がない"))
end
"#,
        ),
        ("28-question-mark", "28-question-mark"),
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
            r#"# 練習問題 28 · `?` でエラーを渡す  ★★★
#
# 失敗するかもしれないステップのたびに `match` を書くのは長くなります。
# Result（または Option）の後の `?` は、`Ok(...)`（または `Some(...)`）から値を
# 取り出すか、すぐに関数を止めて `Err(...)`（または `None`）を返します：
#   fn double_text(t)
#     n = parse_int(t)?          # t が数でなければここで止まる
#     Ok(n * 2)
#   end
#
# 問題
#   - `add_texts(a, b)`：a と b に書かれた 2 つの数の和を `Ok(sum)` で返すか、
#     `parse_int` のエラーを返す
#   - `parse_point(text)`："3,4" なら `Ok({x: 3, y: 4})`。テキストがカンマで
#     区切られたちょうど 2 つの部分でなければエラー `Err("x,y の形が必要")`、
#     数でない部分があればその `parse_int` のエラー
#   - `sum_of_texts(texts)`：テキストのリストに書かれた数の和か、最初のエラー
#   - `ratio_of_texts(a, b)`：テキストに書かれた 2 つの数の a / b か、最初の
#     エラー。b が 0 なら `Err("ゼロ除算")`
#
# ヒント
#   1. `Ok(parse_int(a)? + parse_int(b)?)`
#   2. `match split(text, ",")` で、ケース `[x, y]` と `_` を書きます。
#   3. リストには再帰を使います：`sum_of_texts([])` は `Ok(0)`。
#      `[t, ..rest]` なら、t の数と rest の和を、どちらも `?` を付けて取ります。
#   4. `?` は Result を返すどんな関数にも使えます。自分で書いた関数にもです。
#
# 答えをチェックする：  rythme exercise check 28-question-mark.ry
#             または：  rythme test 28-question-mark.ry

fn add_texts(a, b) = fail("未完成")

fn parse_point(text) = fail("未完成")

fn sum_of_texts(texts) = fail("未完成")

fn ratio_of_texts(a, b) = fail("未完成")

# --- テスト（変更しないでください）---

test "テキストの足し算"
  assert_eq(add_texts("2", "40"), Ok(42))
  assert(is_err(add_texts("2", "四十")), "四十 は数ではない")
end

test "点を読む"
  assert_eq(parse_point("3,4"), Ok({x: 3, y: 4}))
  assert_eq(parse_point("3"), Err("x,y の形が必要"))
  assert_eq(parse_point("1,2,3"), Err("x,y の形が必要"))
  assert(is_err(parse_point("3,四")), "四 は数ではない")
end

test "テキストの和"
  assert_eq(sum_of_texts(["1", "2", "3"]), Ok(6))
  assert_eq(sum_of_texts([]), Ok(0))
  assert_eq(sum_of_texts(["1", "x", "y"]), parse_int("x"))
end

test "テキストの比"
  assert_eq(ratio_of_texts("9", "2"), Ok(4.5))
  assert_eq(ratio_of_texts("9", "0"), Err("ゼロ除算"))
  assert(is_err(ratio_of_texts("九", "0")), "九 は数ではない")
end
"#,
        ),
        ("29-word-counts", "29-word-counts"),
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
            r#"# 練習問題 29 · 小さなプロジェクト：単語を数える  ★★★
#
# テキスト、リスト、レコード、パイプライン、Option を全部使って、テキストを
# 分析するプログラムを作ります。
#
# 問題
#   - `words_of(text)`：テキストの単語を小文字にし、句読点 `.`、`,`、`!`、`?` を
#     取り除いたもの
#   - `frequencies(text)`：`[word, count]` のペアのリスト。出てくる回数の多い
#     単語から少ない単語の順で、同じ回数の単語はアルファベット順
#   - `most_frequent(text)`：いちばん多く出てくる単語を Option で（単語のない
#     テキストなら `None`）
#   - `report(text, n)`：出てくる回数の多い n 個の単語を 1 行に 1 つずつ、
#     それぞれ "単語: 回数" の形で
#
# ヒント
#   1. `replace` を 4 回（またはリスト [".", ",", "!", "?"] で `fold`）、その後に
#      `lower` と `split`。
#   2. `group_by(words, fn(w) = w)` はレコード `{word: [word, word]}` を返し、
#      `entries(record)` はその `[name, value]` のペアのリストを返します。
#   3. `sort_by` は安定です：まず単語で並べ、次に回数で並べます（大きい順に
#      するには `fn([_, n]) = -n`）。
#   4. `first(list)` は Option を返し、`map(option, f)` は中の値を変えます。
#   5. `join(lines, "\n")` は各要素を別々の行に置きます。
#
# 答えをチェックする：  rythme exercise check 29-word-counts.ry
#             または：  rythme test 29-word-counts.ry

fn words_of(text) = fail("未完成")

fn frequencies(text) = fail("未完成")

fn most_frequent(text) = fail("未完成")

fn report(text, n) = fail("未完成")

# --- テスト（変更しないでください）---

poem = "The cat sat. The dog sat! Did the cat see the dog?"

test "単語" = assert_eq(words_of("Hello, World! Hello?"), ["hello", "world", "hello"])

test "頻度"
  assert_eq(frequencies("b a b c a b"), [["b", 3], ["a", 2], ["c", 1]])
  assert_eq(frequencies(poem) |> take(3), [["the", 4], ["cat", 2], ["dog", 2]])
end

test "いちばん多い単語"
  assert_eq(most_frequent(poem), Some("the"))
  assert_eq(most_frequent(" ... "), None)
end

test "レポート" = assert_eq(report(poem, 3), "the: 4\ncat: 2\ndog: 2")
"#,
        ),
        ("30-testing", "30-testing"),
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
            r#"# 練習問題 30 · テストでバグを見つける  ★★★
#
# テストはコードの一部をチェックします：`test "name" = assert_eq(actual,
# expected)`。失敗すると、`rythme test` は 2 つの値を表示します：バグは
# その間のどこかにあります。
#
# 問題
#   下の関数はすでに書かれていますが、それぞれにバグがあります。
#   1. テストを実行して、何が失敗するかを読みます。
#   2. すべてのテストが成功するまで、（テストではなく！）関数を直します。
#   3. それから、ファイルの最後に `clamp` のテストを自分で書きます：
#      `clamp(5, 5, 5)` は何を返すべきでしょう？
#
#   - `clamp(x, low, high)`：x を low と high の間に収めたもの
#   - `median(list)`：並べ替えたリストの真ん中の値。リストの長さが偶数なら
#     真ん中の 2 つの値の平均
#   - `count_words(text)`：テキストの単語の数
#
# ヒント
#   1. 最初の失敗の `得られた値：` と `期待した値：` の行を読み、関数がこれらの
#      引数で何をするかを手で計算します。
#   2. `debug(value)` はプログラムの実行中に値を表示し、そのまま返します：
#      `debug(sorted)`。
#   3. 区切り文字なしの `split` は、続いた空白をどう扱うでしょう？
#
# 答えをチェックする：  rythme exercise check 30-testing.ry
#             または：  rythme test 30-testing.ry

fn clamp(x, low, high) = max(high, min(x, low))

fn median(list)
  middle = len(list) // 2
  if is_even(len(list)) then (list[middle - 1] + list[middle]) / 2 else list[middle]
end

fn count_words(text) = len(split(text, " "))

# --- テスト（変更しないでください）---

test "clamp"
  assert_eq(clamp(15, 0, 10), 10)
  assert_eq(clamp(-5, 0, 10), 0)
  assert_eq(clamp(5, 0, 10), 5)
end

test "値の数が奇数のときの中央値"
  assert_eq(median([1, 2, 3]), 2)
  assert_eq(median([3, 1, 2]), 2)
end

test "値の数が偶数のときの中央値" = assert_eq(median([4, 1, 3, 2]), 2.5)

test "単語を数える"
  assert_eq(count_words("one two three"), 3)
  assert_eq(count_words("  one   two "), 2)
end

# 自分のテスト：
"#,
        ),
        // ----- CSV and seeded random (src/builtins/csv.rs, src/builtins.rs, src/main.rs, src/help.rs)
        // src/builtins/csv.rs
        ("line {line}, column {column}: the quote that starts this field is never closed (a quote inside a field is written twice: \"\")", "{line} 行 {column} 列：このフィールドを始める引用符が閉じられていない（フィールド中の引用符は 2 回書く：\"\"）"),
        ("line {line}, column {column}: a quote in the middle of a field: put the whole field between quotes, and write each quote inside it twice (\"5\"\" screen\")", "{line} 行 {column} 列：フィールドの途中に引用符がある：フィールド全体を引用符で囲み、中の引用符はそれぞれ 2 回書く（\"5\"\" screen\"）"),
        ("line {line}, column {column}: after the quote that closes a field comes {sep} or the end of the line, not {c}", "{line} 行 {column} 列：フィールドを閉じる引用符の後には {sep} か行の終わりが来る（{c} ではなく）"),
        ("line {line}: the header names the column \"{name}\" twice", "{line} 行：見出しで列 \"{name}\" が 2 回使われている"),
        ("line {line}: {got} field(s), but the header has {wanted} column(s)", "{line} 行：フィールドが {got} 個あるが、見出しの列は {wanted} 個"),
        ("line {line}: this table has too many rows or fields (more than {MAX_LIST})", "{line} 行：この表は行かフィールドが多すぎる（{MAX_LIST} を超える）"),
        ("a tab", "タブ"),
        ("a space", "空白"),
        ("`{f}` cannot use {c} as separator: it has another meaning in CSV", "`{f}` は {c} を区切り文字に使えない：CSV では別の意味がある"),
        ("the separator of `{f}` is one character, not \"{s}\"", "`{f}` の区切り文字は 1 文字（\"{s}\" は不可）"),
        ("the usual separators: \",\" (by default), \";\" (French spreadsheets), \"\\t\" (a tab)", "よく使う区切り文字：\",\"（デフォルト）、\";\"（フランスの表計算ソフト）、\"\\t\"（タブ）"),
        ("`csv_format`: the field {column} of row {row} must be a text, a number or a Bool, but it is {got}", "`csv_format`：行 {row} のフィールド {column} はテキスト、数、Bool のどれかでなければならないが、{got} だった"),
        ("turn it into a text first, with `text(x)` or `unwrap_or(x, \"\")`", "先に `text(x)` か `unwrap_or(x, \"\")` でテキストに変える"),
        ("`csv_format`: record {number} has the fields {{{these}}}, but the first one has {{{wanted}}}: every record must have the same fields", "`csv_format`：レコード {number} のフィールドは {{{these}}} だが、最初のレコードは {{{wanted}}}：すべてのレコードが同じフィールドを持たなければならない"),
        ("a record, like the first row", "最初の行のようなレコード"),
        ("a list of fields", "フィールドのリスト"),
        ("`csv_format`: row {number} must be {wanted}, but it is {got}", "`csv_format`：行 {number} は{wanted}でなければならないが、{got} だった"),
        ("`csv_format` takes a list of rows: lists of fields ([[\"name\", \"age\"], [\"Ada\", 36]]) or records ([{{name: \"Ada\", age: 36}}])", "`csv_format` は行のリストを取る：フィールドのリスト（[[\"name\", \"age\"], [\"Ada\", 36]]）かレコード（[{{name: \"Ada\", age: 36}}]）"),
        // src/main.rs
        ("the seed must be a whole number, like `rythme run --seed 42 game.ry`, not `{v}`", "シードは整数でなければならないが、`{v}` だった（例：`rythme run --seed 42 game.ry`）"),
        ("`--seed` needs a whole number, like `rythme run --seed 42 game.ry`", "`--seed` には整数が必要（例：`rythme run --seed 42 game.ry`）"),
        // src/builtins.rs
        ("Makes the draws of `random` and `shuffle` that follow the same on every run: the same seed (a whole number) gives the same draws, with the same version of Rythme. For tests and classes; `rythme run --seed N file.ry` does the same from the command line.", "この後の `random` と `shuffle` の結果を、毎回の実行で同じにします：同じシード（整数）からは、同じバージョンの Rythme なら同じ結果が出ます。テストや授業向けです。`rythme run --seed N file.ry` はコマンドラインから同じことをします。"),
        ("Reads a CSV table (a spreadsheet saved as text): `Ok(list of rows)`, each row a list of texts, or `Err(message)` saying at which line and column the text is not valid CSV. A field between quotes may hold separators, line breaks and quotes (written twice). The separator is \",\" unless given: \";\" for French spreadsheets, \"\\t\" for tabs. See `rythme help csv`.", "CSV の表（テキストとして保存した表計算ファイル）を読みます：`Ok(行のリスト)`（各行はテキストのリスト）、または CSV として正しくない行と列を示す `Err(message)`。引用符で囲んだフィールドには区切り文字、改行、引用符（2 回書く）を入れられます。区切り文字は指定しなければ \",\" です：フランスの表計算ソフトなら \";\"、タブなら \"\\t\"。`rythme help csv` を参照してください。"),
        ("Reads a CSV table whose first line names the columns: `Ok(list of records)`, one record for each other line, with one field for each column, in their order, holding a text. `Err(message)` when the text is not valid CSV, when a line does not have as many fields as the header, or when two columns have the same name. To get numbers, use `parse_int` or `parse_float` on the fields.", "1 行目が列の名前になっている CSV の表を読みます：`Ok(レコードのリスト)`。残りの各行が 1 つのレコードになり、列ごとに順番どおり 1 つのフィールドがあって、テキストを持ちます。テキストが CSV として正しくないとき、行のフィールドの数が見出しと違うとき、2 つの列が同じ名前のときは `Err(message)`。数が欲しければ、フィールドに `parse_int` や `parse_float` を使います。"),
        ("Writes rows as a CSV text, each one on its line, ended by a line break. The rows are lists of fields, or records: then a first line names their fields. A field is a text, a number or a Bool, put between quotes only when it needs them (a separator, a quote or a line break inside). The separator is \",\" unless given.", "行を CSV テキストとして書きます。各行は 1 行に書き、改行で終えます。行はフィールドのリストか、レコードです（レコードなら、1 行目にフィールドの名前を書く）。フィールドはテキスト、数、Bool のどれかで、必要なとき（中に区切り文字、引用符、改行があるとき）だけ引用符で囲みます。区切り文字は指定しなければ \",\" です。"),
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
            r#"CSV：表計算ソフトの表

CSV テキストは表です：1 行が 1 つの行で、フィールドはカンマで区切ります。
どの表計算ソフト（LibreOffice、Excel、Google スプレッドシートなど）でも
開いて保存できます。

  csv_parse("name,age\nAda,36")              # Ok([["name", "age"], ["Ada", "36"]])
  csv_records("name,age\nAda,36")            # Ok([{name: "Ada", age: "36"}])
  csv_format([["name", "age"], ["Ada", 36]]) # "name,age\nAda,36\n"
  csv_format([{name: "Ada", age: 36}])       # 同じテキスト：レコードなら見出しが付く

ファイルと使う
  match read_file("grades.csv").and_then(csv_records)
    Ok(rows) -> print(len(rows), "人の生徒")
    Err(e) -> print("エラー：{e}")
  end
  write_file("copy.csv", csv_format(rows))

規則（RFC 4180）
  引用符で囲んだフィールドには、カンマ、改行、引用符（2 回書く）を入れられます：
  "say ""hi""" はフィールド  say "hi"  です。csv_format は必要な場所にだけ
  引用符を付けます。
  行は \n か \r\n（Windows）で終わります。最後の行は改行がなくてもかまいません。
  空の行は飛ばされ、空白はファイルのまま残ります。
  フィールドはすべてテキストです：parse_int(r.age) や parse_float(r.price) で数になります。
  csv_records：1 行目が列の名前で、それがレコードのフィールドになります
  （r.name。空白を含む名前なら get(r, "first name")）。どの行も見出しと
  同じ数のフィールドを持たなければなりません。

別の区切り文字：最後の引数
  ";"（カンマが小数点になるフランス語の表計算ソフト）、"\t"（タブ）、"|" など
  csv_parse("a;b", ";")                      # Ok([["a", "b"]])
  csv_format(rows, ";")

CSV として正しくないテキストは Err になり、場所（行と列）を示します：
  csv_parse("a,b\"c")   # Err("1 行 4 列：フィールドの途中に引用符がある：...")"#,
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
            r#"RANDOM：乱数と、同じ乱数の再現

  random()                   # 0（含む）から 1（含まない）までの小数
  random(1, 6)               # 1 から 6 まで（両端を含む）の整数
  random(["a", "b", "c"])    # リストの要素の 1 つ
  shuffle([1, 2, 3])         # リストをランダムな順にしたもの

実行するたびに違う値が出ます。同じ結果をもう一度出すには（テスト、全員が
同じゲームを見る授業、もう一度見つけたい誤りなど）、シードを与えます：

  random_seed(42)                  # プログラムの中で：この後の結果
  rythme run --seed 42 game.ry     # コマンドラインから：実行全体

同じバージョンの Rythme なら、同じシードからは同じ結果が同じ順に出ます
（別のバージョンでは違う値が出ることがあります）。シードがなければ何も
変わらず、実行のたびに違う結果になります。

テストでは、最初に random_seed を呼びます：そうすれば、その結果は前の
テストに左右されません。
  test "サイコロの目は 1 から 6 の間"
    random_seed(7)
    assert(all(map(1..100, fn(_) = random(1, 6)), fn(d) = d >= 1 and d <= 6))
  end

プロパティテスト（check_all、sample）には専用のシードがあります：
rythme test --seed N（rythme help tests を参照）。"#,
        ),
        // ----- the debugger (src/debug/, src/help.rs)
        ("this file is not part of the program being debugged", "このファイルはデバッグ中のプログラムの一部ではない"),
        ("top level", "最上位"),
        (" [tail call, replaced {by}]", " [末尾呼び出し、{by} を置き換えた]"),
        ("`{name}` gave {}", "`{name}` の結果は {}"),
        ("error: {}", "エラー：{}"),
        ("Locals", "ローカル"),
        ("Globals", "グローバル"),
        ("result of {name}", "{name} の結果"),
        ("no source to give: the files are on the disk", "渡すソースはない：ファイルはディスク上にある"),
        ("unknown request `{cmd}`", "不明なリクエスト `{cmd}`"),
        ("the program is running: pause it first", "プログラムは実行中：先に一時停止する"),
        ("which program? Give its path in `program`, like \"${{file}}\"", "どのプログラム？ `program` にそのパスを指定する。例：\"${{file}}\""),
        ("the program has errors: see the debug console", "プログラムにエラーがある：デバッグコンソールを参照"),
        ("the program is not running: `{cmd}` has nothing to show", "プログラムは実行されていない：`{cmd}` で表示するものがない"),
        ("the program has ended: `{cmd}` has nothing to show", "プログラムは終了した：`{cmd}` で表示するものがない"),
        ("`print` shows the value of one expression, like `print n * 2` or `print len(xs)`", "`print` は 1 つの式の値を表示する。例：`print n * 2` や `print len(xs)`"),
        ("`{name}` has an effect: the debugger only computes values, without changing anything", "`{name}` には効果がある：デバッガーは何も変えずに値を計算するだけ"),
        ("no code where the program can pause on line {line} of {name} or after it", "{name} の {line} 行目以降に、プログラムが一時停止できるコードがない"),
        ("no function named `{name}` in the program", "プログラムに `{name}` という名前の関数はない"),
        (": did you mean `{best}`?", "：もしかして `{best}`？"),
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
            r#"コマンド（括弧内は短縮形）：
  break 12         プログラムの 12 行目で一時停止する（b）
  break file.ry:3  プログラムの別のファイルの 3 行目で一時停止する
  break fact       関数 fact が始まるたびに一時停止する
  delete 2         ブレークポイント 2 を削除する。delete だけならすべて削除する（d）
  breakpoints      ブレークポイントを一覧する
  run              プログラムを開始する（r）
  continue         次のブレークポイントまで進む（c）
  next             呼び出した関数には入らずに次の行へ進む（n）
  step             呼び出した関数に入って次の行へ進む（s）
  finish           この関数が結果を返すまで進む（f）
  where            実行中の呼び出しとその引数（w）
  frame 2          where の 2 番の呼び出しを見る（print と locals がそれを使う）
  print n * 2      現在の呼び出しで計算した式の値（p）
  locals           現在の呼び出しの名前とその値（l）
  list             現在の行の周りの行
  help             この一覧（h）
  quit             プログラムとデバッガーを止める（q）
空の行は next、step、finish、continue を繰り返す。"#,
        ),
        ("unknown command `{word}`", "不明なコマンド `{word}`"),
        (" (`help` lists the commands)", "（`help` でコマンドを一覧できる）"),
        ("break where? A line (`break 12`), a file and a line (`break geometry.ry:3`) or a function (`break fact`)", "どこで止める？ 行（`break 12`）、ファイルと行（`break geometry.ry:3`）、関数（`break fact`）のどれか"),
        ("no file `{name}` in the program (its files: {})", "プログラムにファイル `{name}` はない（プログラムのファイル：{}）"),
        ("breakpoint {n} at {shown_at}", "ブレークポイント {n}：{shown_at}"),
        ("breakpoint {n} at {shown_at} (line {line} has no code where the program can pause)", "ブレークポイント {n}：{shown_at}（{line} 行目にはプログラムが一時停止できるコードがない）"),
        ("breakpoint {n} at the start of `{arg}`", "ブレークポイント {n}：`{arg}` の開始時"),
        ("`{arg}` is not a line number", "`{arg}` は行番号ではない"),
        ("no breakpoints: add one with `break 12` or `break fact`", "ブレークポイントはない：`break 12` や `break fact` で加える"),
        ("start of `{name}`", "`{name}` の開始時"),
        ("all breakpoints removed", "すべてのブレークポイントを削除した"),
        ("breakpoint {n} removed", "ブレークポイント {n} を削除した"),
        ("there is no breakpoint {n} (see `breakpoints`)", "ブレークポイント {n} はない（`breakpoints` を参照）"),
        ("delete which breakpoint? Give its number, like `delete 1`", "どのブレークポイントを削除する？ 番号を指定する。例：`delete 1`"),
        ("● breakpoint {}", "● ブレークポイント {}"),
        ("● breakpoint {n}: `{name}` starts", "● ブレークポイント {n}：`{name}` の開始"),
        ("✗ the program stops on an error: {msg}", "✗ プログラムがエラーで止まる：{msg}"),
        ("  look at the values with where, locals and print; continue shows the whole message", "  where、locals、print で値を見る。continue でメッセージ全体を表示する"),
        (" · depth {depth}", " · 深さ {depth}"),
        (" ({rec} calls of `{name}` in progress)", "（`{name}` の呼び出しが {rec} 個実行中）"),
        ("  (a call in tail position: it took the place of the frame of `{by}`)", "  （末尾位置の呼び出し：`{by}` の呼び出しの場所を引き継いだ）"),
        ("the program is already running: continue goes on, quit stops it", "プログラムはすでに実行中：continue で続け、quit で止める"),
        (" (replaced `{by}`: tail call)", "（`{by}` を置き換えた：末尾呼び出し）"),
        ("`{name}` calls itself: {rec} calls in progress", "`{name}` は自分自身を呼んでいる：{rec} 個の呼び出しが実行中"),
        ("which call? A number from 0 to {}, as `where` shows them", "どの呼び出し？ `where` が示す 0 から {} までの番号"),
        ("no names here yet", "ここにはまだ名前がない"),
        ("print what? An expression, like `print n` or `print len(xs) * 2`", "何を表示する？ 式を書く。例：`print n` や `print len(xs) * 2`"),
        ("debugger stopped", "デバッガーを停止した"),
        ("the program is not running: `run` starts it (`step` pauses at its first line)", "プログラムは実行されていない：`run` で開始する（`step` なら最初の行で一時停止する）"),
        ("which file? Example: rythme debug factorial.ry (see `rythme help debug`)", "どのファイル？ 例：rythme debug factorial.ry（`rythme help debug` を参照）"),
        ("Debugging {shown}. Pause somewhere with `break 12` or `break function`, then `run`; `help` lists the commands.", "{shown} をデバッグ中。`break 12` や `break function` で止める場所を決めてから `run`。`help` でコマンドを一覧できます。"),
        ("The program finished. `run` starts it again, `quit` leaves.", "プログラムが終了しました。`run` でもう一度開始、`quit` で終了します。"),
        ("The program stopped on an error. `run` starts it again, `quit` leaves.", "プログラムがエラーで止まりました。`run` でもう一度開始、`quit` で終了します。"),
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
            r#"デバッガー：rythme debug

`rythme debug` はプログラムを実行し、指定した場所で一時停止して、実行中の
名前の値を調べられるようにします：

  rythme debug factorial.ry          その後プロンプト (debug) でコマンドを入力

  break 12         12 行目で一時停止する（またはコードのある次の行で）
  break geo.ry:3   プログラムが使うファイルの 3 行目で一時停止する
  break fact       関数 fact が始まるたびに一時停止する
  run              プログラムを開始する（step なら最初の行で一時停止）
  next             呼び出した関数には入らずに次の行へ進む
  step             呼び出した関数に入って次の行へ進む
  finish           この関数が結果を返すまで進み、それを表示する
  continue         次のブレークポイントまで進む
  where            実行中の呼び出しとその引数
  frame 1          where の別の呼び出しを見る（print と locals がそれを使う）
  print n * 2      プログラムのいる場所で計算した式の値
  locals           ここで見える名前とその値
  list             現在の行の周りの行
  delete 1         ブレークポイントを削除する（breakpoints で一覧）
  quit             止める

空の行は next、step、finish、continue を繰り返します。

一時停止するたびに、行、呼び出し（fact(n = 2)）、深さが表示されます：深さは
実行中の呼び出しの数と、関数が自分自身を呼ぶときの同じ関数の呼び出しの数
です。末尾位置の呼び出し（関数が最後に行う処理）は呼び出し元の場所を
引き継ぎます：デバッガーはそれを伝え、where にも表示されます。

print は値を計算するだけです：表示、読み込み、ファイルの書き込み、乱数を
行う関数はそこでは拒否されるので、調べてもプログラムは決して変わりません。
エラーでプログラムが止まると、デバッガーはそれが起きた場所で一時停止する
ので、メッセージの前に値を調べられます。

VS Code では：Rythme 拡張機能がデバッガーを追加します（実行 > デバッグの開始、
または .ry ファイルで F5）：余白のブレークポイント、名前の値、ステップ
オーバー、ステップイン、ステップアウト。他のエディターは Debug Adapter
Protocol を話す `rythme dap` を起動できます。
関数 `debug(value)` は別物です：値を表示し、それを返します。"#,
        ),
        // src/check/annot.rs, src/parser.rs: types written in the code
        ("a whole number is an Int, not a Float: write it with a decimal point (`2.0`), or convert it with `float(...)`", "整数は Float ではなく Int：小数点を付けて書く（`2.0`）か、`float(...)` で変換する"),
        ("a decimal number is a Float, not an Int: convert it with `int(...)`, `round(...)` or `floor(...)`", "小数は Int ではなく Float：`int(...)`、`round(...)`、`floor(...)` のどれかで変換する"),
        ("change the value, or the type written for it; types written in the code are checked before running, and are never needed (see `rythme help types`)", "値か、それに書いた型を変える。コードに書く型は実行前にチェックされ、決して必須ではない（`rythme help types` を参照）"),
        ("`{name}` is written `{w}`, but its value is {what}", "`{name}` は `{w}` と書かれているが、値は {what}"),
        ("the type written for `{name}`", "`{name}` に書いた型"),
        ("this value is {what}, but the type written for it is `{w}`", "この値は {what} だが、書かれた型は `{w}`"),
        ("the type written here", "ここに書いた型"),
        ("not `{w}`", "`{w}` ではない"),
        ("`{v}` cannot be both {a} and {b} in this call to `{f}`", "`{f}` のこの呼び出しで、`{v}` が {a} と {b} の両方になることはできない"),
        ("`{v}` is {b} here", "ここでは `{v}` は {b}"),
        ("`{v}` is {a} here", "ここでは `{v}` は {a}"),
        ("a type variable like `{v}` stands for the same type everywhere in the types written for `{f}`", "`{v}` のような型変数は、`{f}` に書いた型の中のどこでも同じ型を表す"),
        ("`{f}` expects `{ws}` for `{param}`, but receives {what}", "`{f}` は `{param}` に `{ws}` を期待しているが、受け取ったのは {what}"),
        ("not `{ws}`", "`{ws}` ではない"),
        ("the type written for `{param}`", "`{param}` に書いた型"),
        ("this function receives {what}, but `{param}` is written `{w}`", "この関数が受け取るのは {what} だが、`{param}` は `{w}` と書かれている"),
        ("`{f}` is written to give `{w}`, but gives {what}", "`{f}` は `{w}` を返すと書かれているが、返すのは {what}"),
        ("the type written for the result of `{f}`", "`{f}` の結果に書いた型"),
        ("`{f}` is written to give `{w}`, but its `?` may give back {what}", "`{f}` は `{w}` を返すと書かれているが、その `?` は {what} を返すことがある"),
        ("`?` gives back the `None` or `Err(...)` it stops on: write the result as an Option or a Result, like `-> Option(Int)`", "`?` は止まったときの `None` や `Err(...)` を返す：結果を Option か Result として書く。例：`-> Option(Int)`"),
        ("unknown type `{name}`", "不明な型 `{name}`"),
        ("in Rythme, write `{best}`", "Rythme では `{best}` と書く"),
        ("the names of types start with an uppercase letter; a type variable, which stands for any type, is one lowercase letter, like `a`", "型の名前は大文字で始まる。任意の型を表す型変数は、`a` のような小文字 1 文字"),
        ("`{name}` takes no type between parentheses", "`{name}` は括弧に入れた型を取らない"),
        ("write `{name}` alone", "`{name}` だけを書く"),
        ("a type is one of Rythme's (`Int`, `Float`, `Text`, `Bool`, `List`, `Dict`...) or one defined with `type`", "型は Rythme のもの（`Int`、`Float`、`Text`、`Bool`、`List`、`Dict` など）か、`type` で定義したもの"),
        ("`{name}` takes one type: `{example}`", "`{name}` は型を 1 つ取る：`{example}`"),
        ("`{name}` takes two types: `{example}`", "`{name}` は型を 2 つ取る：`{example}`"),
        ("to write the type of `{name}`, put a space after `:`: `{name}: {ty} = ...`", "`{name}` の型を書くには、`:` の後に空白を入れる：`{name}: {ty} = ...`"),
        ("`=` and a value", "`=` と値"),
        ("a name with a written type is given its value right away: `total: Float = 0.0`", "型を書いた名前にはすぐに値を与える：`total: Float = 0.0`"),
        ("`,`", "`,`"),
        ("to write the type of a parameter, put `:` right after its name, then a space: `x: {s}`", "引数の型を書くには、名前の直後に `:`、その後に空白を入れる：`x: {s}`"),
        ("`:` and the type of the field", "`:` とフィールドの型"),
        ("`(`", "`(`"),
        ("`->` and the type of the result", "`->` と結果の型"),
        ("a type", "型"),
        ("to write the type of a field, write its name first: `Circle(radius: Float)`", "フィールドの型を書くには、先にその名前を書く：`Circle(radius: Float)`"),
        ("a type is written like `Int`, `Text`, `List(Int)`, `[Int]`, `{{name: Text}}`, `fn(Int) -> Int` or `Shape`; see `rythme help types`", "型は `Int`、`Text`、`List(Int)`、`[Int]`、`{{name: Text}}`、`fn(Int) -> Int`、`Shape` のように書く。`rythme help types` を参照"),
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
            r#"E027 — コードに書いた型が合わない

`:` や `->` の後にコードに書いた型は、`rythme check` が実行前に確かめる
約束です：確実に別の型を持つ値は、それが与えられた場所で報告されます。
型は存在するものでなければなりません。

誤り：   total: Float = 0              # 0 は Int
正しい： total: Float = 0.0
誤り：   fn area(w: Int, h: Int) -> Int = w * h
         area(2, "3")                  # "3" は Text
正しい： area(2, 3)
誤り：   fn label(n: Int) -> Text = n  # Int を返す
正しい： fn label(n: Int) -> Text = text(n)
誤り：   fn twice(x: int) = x * 2      # 型名は大文字で始まる
正しい： fn twice(x: Int) = x * 2
誤り：   fn depth(t: Tree(Int, Text))  # type Tree(a) は型を 1 つ取る
正しい： fn depth(t: Tree(Int))

書いた型は省略できます：取り除いてもプログラムの動作は決して変わりません。
詳しくは `rythme help types`。"#,
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
            r#"コードに書く型（省略可）

Rythme は型を自分で見つけます。コードが読みやすくなる場所に型を書くことも
でき、そのときは `rythme check` がそれを確かめます（E027）：
  fn area(width: Int, height: Int) -> Int = width * height
  total: Float = 0.0
  type Shape = Circle(radius: Float) | Rect(width: Float, height: Float)
  [1, 2].map do |n: Int| n * 2 end

型
  Int  Float  Text  Bool  Unit  Symbol
  List(Int) or [Int]   Set(Text)   Dict(Text, Int)
  Option(Int)   Result(Int, Text)
  {name: Text, age: Int}       少なくともこれらのフィールドを持つレコード
  fn(Int, Int) -> Int          関数
  Shape                        `type` で定義した型（`use geometry` の型なら
                               geometry.Shape）
  Tree(Int)                    パラメーター付きで定義した型と、その型
  a                            任意の型：小文字 1 文字。定義の中では
                               どこでも同じ型
  List、Dict、Set、Option、Result、Record、Function だけなら：中身は任意
  （Tree だけでも同じ）

  fn first_of(xs: List(a)) -> a = xs[0]
  first_of([1, 2]) + 1         # チェッカーは Int だとわかる

型パラメーター付きの型：List(Int) のように、型が型を取れる
  type Tree(a) = Leaf | Node(left: Tree(a), value: a, right: Tree(a))
  type Pair(a, b) = Pair(first: a, second: b)

  fn depth(t: Tree(a)) -> Int = ...      何でも入る Tree
  fn total(t: Tree(Int)) -> Int = ...    整数の Tree
  total(Node(Leaf, "two", Leaf))         E027：これは Tree(Text)

型変数は 1 文字です：`int` や `string` のような小文字の単語は、他の言語の
名前で書いた型とみなされ、`rythme check` が Rythme での名前を教えます。

書いた型は決して必須ではなく、実行時には何も変えません。エディターと
`rythme doc` がそれを表示します。"#,
        ),
        // src/fix.rs, src/machine.rs: fixes and output for programs
        ("`{f}` is not available in the sandbox: a program run by a tool (`--sandbox`) cannot touch files or folders. Run it yourself: rythme file.ry", "`{f}` はサンドボックスでは使えない：ツールが実行するプログラム（`--sandbox`）はファイルにもフォルダーにも触れられない。自分で実行する：rythme file.ry"),
        ("Remove `{}`", "`{}` を取り除く"),
        ("\"not equal\" is written `!=`: `a != b`", "「等しくない」は `!=` と書く：`a != b`"),
        ("an anonymous function is written `fn(x) = x * 2`", "無名関数は `fn(x) = x * 2` と書く"),
        (
            r#"on several lines, `if` is written:
  if x > 0 then
    "positive"
  else if x == 0 then
    "zero"
  else
    "negative"
(no `end`; for several statements in a branch, use `do ... end`)"#,
            r#"複数行の `if` は次のように書く：
  if x > 0 then
    "正"
  else if x == 0 then
    "ゼロ"
  else
    "負"
（`end` はない。分岐に複数の文を書くなら `do ... end` を使う）"#,
        ),
        ("Rythme texts put values in by themselves: write `\"total: {{x}}\"`, without `f` in front", "Rythme のテキストは値を自分で埋め込む：前に `f` を付けずに `\"total: {{x}}\"` と書く"),
        ("`--format` takes `json` or `text`, not `{v}`: `rythme check --format json file.ry`", "`--format` が取るのは `json` か `text`（`{v}` は不可）：`rythme check --format json file.ry`"),
        ("`--watch` does not go with `--format json`: run `rythme test --format json` again after each change", "`--watch` は `--format json` と一緒に使えない：変更のたびに `rythme test --format json` をもう一度実行する"),
        ("unknown option `{bad}` for `rythme fix`; the options are: --dry-run, --all, --format json", "`rythme fix` に不明なオプション `{bad}`。オプションは --dry-run、--all、--format json"),
        ("which file? Example: rythme fix hello.ry (or rythme fix --dry-run hello.ry to see the changes first)", "どのファイル？ 例：rythme fix hello.ry（先に変更を見るなら rythme fix --dry-run hello.ry）"),
        ("cannot write `{shown}`: {e}", "`{shown}` を書き込めない：{e}"),
        ("{shown}: nothing to fix", "{shown}：修正するものはない"),
        ("{shown}: {n} fix(es) to apply", "{shown}：適用する修正が {n} 件"),
        ("{shown}: {n} fix(es) applied", "{shown}：修正を {n} 件適用した"),
        ("; {errors} error(s) and {warnings} warning(s) left: `rythme check {shown}`", "。残りはエラー {errors} 件、警告 {warnings} 件：`rythme check {shown}`"),
        ("here: `new_{name} = {name} {op} {rest}`, then use `new_{name}`; to add up a list, use `sum(list)` or `fold`", "ここでは：`new_{name} = {name} {op} {rest}` と書き、`new_{name}` を使う。リストを合計するには `sum(list)` か `fold` を使う"),
        // src/mcp.rs: the server for AI assistants
        ("`rythme mcp` takes no argument, not `{bad}`", "`rythme mcp` は引数を取らない（`{bad}` は不可）"),
        (
            r#"rythme mcp: a Model Context Protocol server for AI assistants, on the standard input and output.
Its tools run (in a sandbox), check, test, format and fix Rythme code, and explain errors and functions.
Cursor and others: see the manual, section 7.10."#,
            r#"rythme mcp：標準入出力を使う、AI アシスタント向けの Model Context Protocol サーバーです。
そのツールは Rythme のコードを（サンドボックスで）実行し、チェック、テスト、整形、修正を行い、エラーと関数を説明します。
Cursor などはマニュアルの 7.10 節を参照してください。"#,
        ),
        ("the program was stopped after {seconds} seconds: it may never end (a recursion without a base case?)", "{seconds} 秒でプログラムを止めた：終わらないのかもしれない（基底ケースのない再帰？）"),
        ("the program was stopped: it took more than {mb} MB of memory", "プログラムを止めた：{mb} MB を超えるメモリを使った"),
        // src/fix.rs, src/suggest.rs: habits from other languages
        ("an anonymous function is written `fn(x) = x * 2`; a block given to a call is written `xs.map do |x| x * 2 end` or `xs.map {{ |x| x * 2 }}`", "無名関数は `fn(x) = x * 2` と書く。呼び出しに渡すブロックは `xs.map do |x| x * 2 end` か `xs.map {{ |x| x * 2 }}` と書く"),
        ("the parameters of a block are written between bars: `xs.map {{ |x| x * 2 }}`", "ブロックの引数は縦棒で囲んで書く：`xs.map {{ |x| x * 2 }}`"),
        ("to add up a list, write `sum(xs)`; to combine its elements, `fold(xs, 0, fn(acc, x) = acc + x)`", "リストを合計するには `sum(xs)`、要素をまとめるには `fold(xs, 0, fn(acc, x) = acc + x)` と書く"),
        ("a part of a list or a text is taken with a function: `slice(xs, 1, 3)`, `drop(xs, 1)` (all but the first), `take(xs, 2)` (the first two)", "リストやテキストの一部は関数で取る：`slice(xs, 1, 3)`、`drop(xs, 1)`（最初以外すべて）、`take(xs, 2)`（最初の 2 つ）"),
        ("the fields of a record are written with `:`: `{{{field}: value}}`", "レコードのフィールドは `:` で書く：`{{{field}: value}}`"),
        ("Rythme has no named arguments (`name=value`): give the values in order, like `round(x, 2)` (`rythme help NAME` shows the order)", "Rythme に名前付き引数（`name=value`）はない：`round(x, 2)` のように値を順に渡す（順番は `rythme help NAME` で確認できる）"),
        ("texts have no format specifiers like `{{x:.2f}}`: write `{{text(x, 2)}}` for 2 decimals, or use `format_number`", "テキストに `{{x:.2f}}` のような書式指定はない：小数点以下 2 桁なら `{{text(x, 2)}}` と書くか、`format_number` を使う"),
        ("when there is nothing to do otherwise, give the unit value: `if x > 0 then print(x) else ()`", "それ以外に何もすることがないときは、unit 値を与える：`if x > 0 then print(x) else ()`"),
        ("here: `new_{w} = ...`, then use `new_{w}`; to repeat a step, use `fold`, `map` or a recursive function", "ここでは：`new_{w} = ...` と書き、`new_{w}` を使う。ステップを繰り返すには `fold`、`map`、再帰関数のどれかを使う"),
        ("the functions of numbers are built in: write `sqrt(x)`, `abs(x)`, `floor(x)`, `pi`... without `Math.`", "数の関数は組み込み：`Math.` を付けずに `sqrt(x)`、`abs(x)`、`floor(x)`、`pi` などと書く"),
        ("Rythme has no generators: build the whole list (`map`, `filter`, `1..n`), or give a function to call", "Rythme にこの種のジェネレーターはない：リスト全体を作る（`map`、`filter`、`1..n`）か、呼び出す関数を渡す"),
        // src/builtins/ml.rs: vectors, matrices, statistics, chance
        ("`{f}` expects a list of numbers as argument {pos}, but its element {k} is {got}", "`{f}` の第 {pos} 引数には数のリストが必要だが、その要素 {k} は {got}"),
        ("a text is not a number: `parse_float(text)` reads one", "テキストは数ではない：`parse_float(text)` で数を読む"),
        ("`{f}` expects a matrix (a list of rows, each a list of numbers) as argument {pos}, but its row {r} is {got}", "`{f}` の第 {pos} 引数には行列（各行が数のリストである行のリスト）が必要だが、その行 {r} は {got}"),
        ("a matrix of 2 rows and 3 columns: [[1, 2, 3], [4, 5, 6]]", "2 行 3 列の行列：[[1, 2, 3], [4, 5, 6]]"),
        ("`{f}`: argument {pos} is not a matrix: its row 0 has {cols} number(s), but its row {r} has {n}", "`{f}`：第 {pos} 引数は行列ではない：行 0 の数は {cols} 個だが、行 {r} は {n} 個"),
        ("every row of a matrix has the same length", "行列の行はすべて同じ長さ"),
        ("`{f}` expects a matrix of numbers as argument {pos}, but its row {r} holds {got} at position {k}", "`{f}` の第 {pos} 引数には数の行列が必要だが、その行 {r} の位置 {k} に {got} がある"),
        ("a vector of {n} number(s)", "{n} 個の数のベクトル"),
        ("a {n}×{c} matrix", "{n}×{c} の行列"),
        ("a list of {n} element(s)", "{n} 個の要素のリスト"),
        ("`{f}` needs two values of the same shape, but got {a} and {b}", "`{f}` には同じ形の 2 つの値が必要だが、受け取ったのは{a}と{b}"),
        ("`shape(x)` gives the shape of a vector or a matrix: [3] or [2, 3]", "`shape(x)` はベクトルや行列の形を返す：[3] や [2, 3]"),
        ("`{f}`: these lists are nested more than {MAX_NESTING} levels deep: they are not a vector nor a matrix", "`{f}`：これらのリストは {MAX_NESTING} 段を超えて入れ子になっている：ベクトルでも行列でもない"),
        ("`{f}` works on vectors and matrices of numbers, but found {got} in them", "`{f}` は数のベクトルと行列に使えるが、中に {got} があった"),
        ("`{f}` needs two vectors of the same length, but got a vector of {n} number(s) and a vector of {m}", "`{f}` には同じ長さの 2 つのベクトルが必要だが、受け取ったのは {n} 個の数のベクトルと {m} 個の数のベクトル"),
        ("`zip_with` needs two lists of the same length, but got {n} and {m} element(s)", "`zip_with` には同じ長さの 2 つのリストが必要だが、受け取った要素は {n} 個と {m} 個"),
        ("a {r}×{c} matrix", "{r}×{c} の行列"),
        ("a vector of {nr} number(s)", "{nr} 個の数のベクトル"),
        ("a {nr}×{nc} matrix", "{nr}×{nc} の行列"),
        ("`matmul` cannot multiply {first} by {second}: the first one has {c} column(s), so the second one needs {c} row(s), not {nr}", "`matmul` は{first}に{second}を掛けられない：1 つ目の列は {c} 個なので、2 つ目の行は {c} 個必要（{nr} 個ではなく）"),
        ("`shape(x)` gives the shape of each one; `transpose(m)` swaps rows and columns", "`shape(x)` でそれぞれの形がわかる。`transpose(m)` は行と列を入れ替える"),
        ("`matmul`: these matrices are too large to multiply here (more than {} multiplications)", "`matmul`：これらの行列はここで掛けるには大きすぎる（{} 回を超える掛け算）"),
        ("`shape`: the elements of this list do not all have the same shape: element 0 is {a}, element {i} is {b}", "`shape`：このリストの要素がすべて同じ形ではない：要素 0 は{a}、要素 {i} は{b}"),
        ("`{f}` of an empty list", "空のリストの `{f}`"),
        ("`percentile` needs a percentage from 0 to 100, not {p}", "`percentile` には 0 から 100 までの百分率が必要（{p} は不可）"),
        ("`random_normal` needs a standard deviation of 0 or more, not {sd}", "`random_normal` には 0 以上の標準偏差が必要（{sd} は不可）"),
        ("`random_choice_weighted` needs one weight per element: {n} element(s), but {m} weight(s)", "`random_choice_weighted` には要素ごとに 1 つの重みが必要：要素は {n} 個だが、重みは {m} 個"),
        ("`random_choice_weighted`: the weight {i} is {w}; a weight is a number of 0 or more", "`random_choice_weighted`：重み {i} は {w}。重みは 0 以上の数"),
        ("`random_choice_weighted` needs at least one weight above 0", "`random_choice_weighted` には 0 より大きい重みが少なくとも 1 つ必要"),
        ("`train_test_split` needs a share from 0 to 1 for the training part (like 0.8), not {ratio}", "`train_test_split` には訓練用の割合として 0 から 1 までの値（0.8 など）が必要（{ratio} は不可）"),
        ("Machine learning: vectors, matrices, statistics", "機械学習：ベクトル、行列、統計"),
        ("Hyperbolic tangent: a smooth step from -1 to 1, used as the activation of neurons.", "双曲線正接：-1 から 1 へのなめらかな段差で、ニューロンの活性化関数に使います。"),
        ("The logistic function 1 / (1 + e^-x): a smooth step from 0 to 1, read as a probability. Its slope is `sigmoid(x) * (1 - sigmoid(x))`.", "ロジスティック関数 1 / (1 + e^-x)：0 から 1 へのなめらかな段差で、確率として読みます。その傾きは `sigmoid(x) * (1 - sigmoid(x))` です。"),
        ("The rectifier: x when it is positive, 0.0 otherwise. The usual activation of deep networks.", "整流関数：x が正なら x、そうでなければ 0.0。深いネットワークでよく使う活性化関数です。"),
        ("Dot product of two vectors (lists of numbers) of the same length: the sum of the products of their numbers, one by one.", "同じ長さの 2 つのベクトル（数のリスト）の内積：数どうしの積を 1 つずつ足したもの。"),
        ("Length of a vector: the square root of the sum of the squares of its numbers.", "ベクトルの長さ：数の 2 乗の和の平方根。"),
        ("Adds two vectors, or two matrices, of the same shape, number by number.", "同じ形の 2 つのベクトル（または行列）を、数ごとに足します。"),
        ("Subtracts two vectors, or two matrices, of the same shape, number by number.", "同じ形の 2 つのベクトル（または行列）を、数ごとに引きます。"),
        ("Multiplies two vectors, or two matrices, of the same shape, number by number (not the product of matrices: that is `matmul`).", "同じ形の 2 つのベクトル（または行列）を、数ごとに掛けます（行列の積ではありません：それは `matmul`）。"),
        ("Multiplies every number of a vector, or of a matrix, by the number k.", "ベクトル（または行列）のすべての数に数 k を掛けます。"),
        ("Calls f with the elements of two lists of the same length, one pair at a time, and gives the list of the results.", "同じ長さの 2 つのリストの要素を 1 組ずつ f に渡して呼び、結果のリストを返します。"),
        ("Product of two matrices (lists of rows): a matrix of r rows and n columns times a matrix of n rows and c columns gives r rows and c columns. With a vector as `b`, gives a vector.", "2 つの行列（行のリスト）の積：r 行 n 列の行列と n 行 c 列の行列の積は r 行 c 列になります。`b` がベクトルなら、ベクトルを返します。"),
        ("Swaps the rows and the columns of a matrix.", "行列の行と列を入れ替えます。"),
        ("A vector of `rows` zeros, or with `columns`, a matrix full of zeros.", "`rows` 個の 0 のベクトル。`columns` を指定すると、0 で埋めた行列。"),
        ("A vector of `rows` ones, or with `columns`, a matrix full of ones.", "`rows` 個の 1 のベクトル。`columns` を指定すると、1 で埋めた行列。"),
        ("The identity matrix of n rows and n columns: ones on the diagonal, zeros elsewhere.", "n 行 n 列の単位行列：対角線上が 1、それ以外が 0。"),
        ("The shape of a vector or a matrix (of lists nested in lists): its length at each level, `[3]` for a vector of 3 numbers, `[2, 3]` for 2 rows of 3 numbers.", "ベクトルや行列（リストの中に入れ子にしたリスト）の形：各段の長さで、3 つの数のベクトルなら `[3]`、3 つの数の行が 2 つなら `[2, 3]`。"),
        ("A summary of a list of numbers, as a record: `count`, `mean` (the average), `median` (the middle value), `std_dev`, `min` and `max`. `stats(xs).mean` is the mean.", "数のリストのまとめをレコードで返します：`count`、`mean`（平均）、`median`（中央値）、`std_dev`、`min`、`max`。`stats(xs).mean` が平均です。"),
        ("The variance of a list of numbers: the mean of the squares of their distances to their mean (of the whole population: divided by n).", "数のリストの分散：平均からの距離の 2 乗の平均（母集団全体として n で割る）。"),
        ("The standard deviation of a list of numbers: the square root of their variance, how far they usually are from their mean.", "数のリストの標準偏差：分散の平方根で、数がふつう平均からどれだけ離れているかを表します。"),
        ("The number below which p percent of the numbers of the list are (p from 0 to 100), between the two closest ones when needed. `percentile(xs, 50)` is the median, the middle value.", "リストの数の p パーセントがそれより小さくなる数（p は 0 から 100）。必要なら近い 2 つの数の間の値になります。`percentile(xs, 50)` は中央値です。"),
        ("The position of the largest number of a list (the first one if several are equal).", "リストのいちばん大きい数の位置（等しいものが複数あれば最初のもの）。"),
        ("The position of the smallest number of a list (the first one if several are equal).", "リストのいちばん小さい数の位置（等しいものが複数あれば最初のもの）。"),
        ("Turns a list of scores into probabilities: each one becomes e^x divided by the sum of them all, so that they are between 0 and 1 and add up to 1, the largest score keeping the largest share.", "点数のリストを確率に変えます：それぞれ e^x を全体の和で割った値になるので、0 から 1 の間にあって合計は 1 になり、最大の点数が最大の割合を保ちます。"),
        ("Rescales a list of numbers from 0.0 (the smallest) to 1.0 (the largest), keeping their proportions. When they are all equal, gives zeros.", "数のリストを、比率を保ったまま 0.0（最小）から 1.0（最大）の範囲に直します。すべて等しければ 0 を返します。"),
        ("A random decimal number drawn from the normal distribution (the bell curve) of this mean and standard deviation. Follows `random_seed`.", "この平均と標準偏差の正規分布（釣り鐘型の曲線）から引いたランダムな小数。`random_seed` に従います。"),
        ("An element of the list chosen at random, each one with a chance in proportion to its weight (numbers of 0 or more, one per element). Follows `random_seed`.", "重み（0 以上の数で要素ごとに 1 つ）に比例した確率でランダムに選んだリストの要素。`random_seed` に従います。"),
        ("Shuffles a list, then cuts it in two: a record `{train, test}`, where `train` holds this share of the elements (0.8: 80 percent) and `test` the rest. To learn on some examples and check on others. Follows `random_seed`.", "リストをシャッフルしてから 2 つに切ります：レコード `{train, test}` で、`train` は要素のこの割合（0.8 なら 80 パーセント）を、`test` は残りを持ちます。一部の例で学習し、別の例で確かめるためのものです。`random_seed` に従います。"),
        // src/builtins/grad.rs: slopes
        ("the function given to `{f}` must return a number, but it returned {got}", "`{f}` に渡す関数は数を返さなければならないが、{got} を返した"),
        ("The slope of the function f at x (its derivative), computed from f(x + h) and f(x - h) for a tiny h: precise to about 9 significant digits for a smooth function. f takes one number and gives one.", "x における関数 f の傾き（導関数）。ごく小さい h について f(x + h) と f(x - h) から計算します：なめらかな関数なら有効数字約 9 桁の精度です。f は数を 1 つ取り、数を 1 つ返します。"),
        ("The gradient of f at the point xs: the list of the slopes of f along each number of xs (see `derivative`). f takes a list of numbers and gives one number. Going against it lowers f: the heart of gradient descent.", "点 xs における f の勾配：xs の各数の方向の f の傾きのリスト（`derivative` を参照）。f は数のリストを取り、数を 1 つ返します。勾配と逆に進むと f が小さくなります：勾配降下法の中心です。"),
        // src/builtins/plot.rs: charts as text
        ("`{f}` has nothing to draw: the list is empty, or holds no finite number", "`{f}` に描くものがない：リストが空か、有限の数が含まれていない"),
        ("`histogram` draws at most 100 bars, not {bins}", "`histogram` が描く棒は最大 100 本（{bins} は不可）"),
        ("Draws the points (x, y) as a chart in text, 50 columns wide and 11 lines high, with the smallest and largest values on the axes; with one list, draws its numbers at the positions 0, 1, 2... Gives a text: `print(plot(losses))`.", "点 (x, y) を、幅 50 列、高さ 11 行のテキストのグラフとして描き、軸に最小値と最大値を示します。リストが 1 つなら、その数を位置 0, 1, 2... に描きます。テキストを返します：`print(plot(losses))`。"),
        ("Counts how many numbers fall in each of `bars` intervals of the same width (10 if not given) and draws them as horizontal bars of text, with the interval and the count of each. Gives a text: `print(histogram(heights))`.", "同じ幅の `bars` 個（指定しなければ 10 個）の区間それぞれに入る数を数え、区間と個数を添えたテキストの横棒として描きます。テキストを返します：`print(histogram(heights))`。"),
        // src/builtins/http.rs: HTTP requests
        ("`{f}`: the names and the values of the headers are texts, not {got}", "`{f}`：ヘッダーの名前と値はテキスト（{got} は不可）"),
        ("for example: {{\"content-type\": \"application/json\", \"x-api-key\": key}}", "例：{{\"content-type\": \"application/json\", \"x-api-key\": key}}"),
        ("`{f}` is not available in the browser: the playground cannot make requests. Run the program on your computer: rythme file.ry", "`{f}` はブラウザーでは使えない：プレイグラウンドはリクエストを送れない。自分のコンピューターでプログラムを実行する：rythme file.ry"),
        ("a web address starts with http:// or https://, not \"{url}\"", "Web アドレスは http:// か https:// で始まる（\"{url}\" は不可）"),
        ("an address or a header cannot hold a line break or a control character, and a header name is a word without `:` or space", "アドレスやヘッダーに改行や制御文字は入れられない。ヘッダー名は `:` も空白も含まない単語"),
        ("the server answered {code}: {shown}{more}", "サーバーの応答は {code}：{shown}{more}"),
        ("`{f}` needs the program `curl`, which comes with Windows 10, macOS and most Linux systems: install it (on Debian or Ubuntu: sudo apt install curl)", "`{f}` にはプログラム `curl` が必要（Windows 10、macOS、たいていの Linux に付属）：インストールする（Debian や Ubuntu なら sudo apt install curl）"),
        ("server not found: {host} (no connection to the Internet, or a mistake in the address?)", "サーバーが見つからない：{host}（インターネットに接続していないか、アドレスの誤り？）"),
        ("could not connect to {host}: {said}", "{host} に接続できない：{said}"),
        ("no answer from {host} in time (30 seconds)", "{host} から時間内（30 秒）に応答がない"),
        ("the secure connection (TLS) to {host} failed: {said}", "{host} への安全な接続（TLS）に失敗した：{said}"),
        ("the request to {host} failed: {said}", "{host} へのリクエストに失敗した：{said}"),
        ("The web: HTTP requests", "Web：HTTP リクエスト"),
        ("Fetches a web address (http:// or https://): `Ok(text of the answer)`, or `Err(message)` when there is no network, the address is wrong, the server answers with an error (404...) or does not answer within 30 seconds. `headers` is a record of texts, like `{\"accept\": \"application/json\"}`. Uses the program curl; not available in the playground.", "Web アドレス（http:// か https://）を取得します：`Ok(応答のテキスト)`、またはネットワークがない、アドレスが違う、サーバーがエラー（404 など）を返す、30 秒以内に応答しないときは `Err(message)`。`headers` は `{\"accept\": \"application/json\"}` のようなテキストのレコードです。プログラム curl を使います。プレイグラウンドでは使えません。"),
        ("Sends a text (often JSON, made with `to_json`) to a web address and gives `Ok(text of the answer)` or `Err(message)`, like `http_get`. To call a web service or a language model: `http_post(url, to_json(request), {\"content-type\": \"application/json\"})`. Keep keys out of your code: read them with `env`.", "テキスト（多くは `to_json` で作った JSON）を Web アドレスに送り、`http_get` と同じく `Ok(応答のテキスト)` か `Err(message)` を返します。Web サービスや言語モデルを呼ぶには：`http_post(url, to_json(request), {\"content-type\": \"application/json\"})`。キーはコードに書かず、`env` で読みます。"),
        ("The result of a task: the call of an `async fn` starts the function and gives a Task at once; `await` waits until it is done and gives what it returned (an error in the task stops the program here). Tasks run at the same time, on other cores: start several, then await them. A task can be awaited again: it gives the same result.", "タスクの結果です：`async fn` を呼び出すと関数が始まり、すぐに Task が返ります。`await` はそれが終わるまで待ち、関数が返したものを返します（タスクの中でエラーが起きると、プログラムはここで停止します）。タスクは別のコアで同時に動きます：いくつか開始してから、それらを await します。タスクはもう一度 await できます：同じ結果を返します。"),
        ("The results of a list of tasks (calls of `async fn`), in the order of the list, once all are done: the tasks run at the same time, so the whole takes about as long as the slowest one.", "タスク（`async fn` の呼び出し）のリストの結果を、すべて終わった後にリストの順で返します：タスクは同時に動くので、全体にかかる時間はいちばん遅いタスクとほぼ同じです。"),
        // docs/exercises: the exercises about artificial intelligence (31 to 36)
        ("31-vectors", "31-vectors"),
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
            r#"# 練習問題 31 · ベクトルとしての点  ★☆☆
#
# 機械学習では、1 つの例は数のリスト、つまりベクトルです。組み込み関数
# `vector_add`、`vector_sub`、`vector_scale`、`dot`、`norm` はそれを扱い、
# 常に Float を返します。
#   vector_sub([5, 5], [1, 2])     # [4.0, 3.0]
#   norm([3, 4])                   # 5.0
#
# 問題
#   - `distance(a, b)`：点 a と b の距離（一方から他方へのベクトルの長さ）
#   - `midpoint(a, b)`：a と b のちょうど中間の点
#   - `closest(points, p)`：リストのうち p にいちばん近い点
#
# ヒント
#   1. `distance` は差の `norm` です。
#   2. 中間：2 つの点を足してから 0.5 倍します。
#   3. `min_by(list, f)` は f が最小になる要素を返します。
#
# 答えをチェックする：  rythme exercise check 31-vectors.ry
#             または：  rythme test 31-vectors.ry

fn distance(a, b) = fail("未完成")

fn midpoint(a, b) = fail("未完成")

fn closest(points, p) = fail("未完成")

# --- テスト（変更しないでください）---

test "距離"
  assert_eq(distance([0, 0], [3, 4]), 5.0)
  assert_eq(distance([1, 1, 1], [1, 1, 1]), 0.0)
end

test "中点"
  assert_eq(midpoint([0, 0], [4, 2]), [2.0, 1.0])
  assert_eq(midpoint([-1, 3], [1, 3]), [0.0, 3.0])
end

test "いちばん近い点"
  assert_eq(closest([[0, 0], [5, 5], [9, 1]], [6, 4]), [5, 5])
  assert_eq(closest([[2, 2]], [100, -100]), [2, 2])
end
"#,
        ),
        ("32-statistics", "32-statistics"),
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
            r#"# 練習問題 32 · データの統計  ★★☆
#
# データから学ぶ前に、データを見てみましょう。`stats(xs)` は、個数、平均、
# 中央値、標準偏差、最小値、最大値を持つレコードを返します：
#   stats([2, 4, 4, 4, 5, 5, 7, 9]).mean       # 5.0
#   stats([2, 4, 4, 4, 5, 5, 7, 9]).std_dev    # 2.0
#
# 問題
#   - `standardize(xs)`：各数を、平均からの距離を標準偏差の単位で数えたもの
#     にする：(x - mean) / std_dev
#   - `outliers(xs)`：平均から標準偏差の 2 倍より離れている数を順番どおりに
#   - `accuracy(predictions, answers)`：同じ位置の答えと等しい予測の割合
#     （0.0 から 1.0）
#
# ヒント
#   1. `stats(xs)` を 1 回計算して名前を付け、それから `map` します。
#   2. `abs` はどちら側でも距離を返します。
#   3. `zip(a, b)` は要素を組にします。`count` で正しいものを数えます。
#
# 答えをチェックする：  rythme exercise check 32-statistics.ry
#             または：  rythme test 32-statistics.ry

fn standardize(xs) = fail("未完成")

fn outliers(xs) = fail("未完成")

fn accuracy(predictions, answers) = fail("未完成")

# --- テスト（変更しないでください）---

test "標準化"
  assert_eq(standardize([2, 4, 4, 4, 5, 5, 7, 9]), [-1.5, -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 2.0])
end

test "外れ値"
  assert_eq(outliers([10, 11, 9, 10, 10, 12, 10, 9, 11, 10, 50]), [50])
  assert_eq(outliers([1, 2, 3]), [])
end

test "正解率"
  assert_eq(accuracy(["cat", "dog", "cat", "cat"], ["cat", "dog", "dog", "cat"]), 0.75)
  assert_eq(accuracy([1, 0], [0, 1]), 0.0)
end
"#,
        ),
        ("33-neurons", "33-neurons"),
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
            r#"# 練習問題 33 · 人工ニューロン  ★★☆
#
# ニューロンは各入力に重みを掛けて、バイアスとともに足し合わせ、その和を
# `sigmoid` のような活性化関数に通します。`sigmoid` は 0 から 1 の数を返します。
#   sigmoid(0)                     # 0.5
#   dot([1, 2], [3, 4])            # 11.0
#
# 問題
#   - `neuron(weights, bias, inputs)`：sigmoid(weights · inputs + bias)
#   - `fires(weights, bias, inputs)`：ニューロンが 0.5 以上を返すとき true
#   - `layer(matrix, biases, inputs)`：行列の行（重み）とバイアスごとに 1 つの
#     ニューロンを持つ層：それらの出力のリスト
#
# ヒント
#   1. `dot` は重み付きの和を返します。
#   2. `matmul(matrix, inputs)` は層の重み付きの和を一度にすべて計算します。
#      `vector_add` でバイアスを足し、`map` で `sigmoid` を適用します。
#
# 答えをチェックする：  rythme exercise check 33-neurons.ry
#             または：  rythme test 33-neurons.ry

fn neuron(weights, bias, inputs) = fail("未完成")

fn fires(weights, bias, inputs) = fail("未完成")

fn layer(matrix, biases, inputs) = fail("未完成")

# --- テスト（変更しないでください）---

test "ニューロン"
  assert_eq(neuron([1, -1], 0, [2, 2]), 0.5)
  assert_eq(round(neuron([2, 1], -1, [1, 1]), 3), 0.881)
end

test "AND を計算するニューロン"
  assert(fires([10, 10], -15, [1, 1]))
  assert(not fires([10, 10], -15, [1, 0]))
  assert(not fires([10, 10], -15, [0, 0]))
end

test "層"
  outputs = layer([[1, -1], [1, 1]], [0, 1], [2, 2])
  assert_eq(map(outputs, fn(x) = round(x, 3)), [0.5, 0.993])
end
"#,
        ),
        ("34-gradient-descent", "34-gradient-descent"),
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
            r#"# 練習問題 34 · 坂を下る  ★★☆
#
# モデルは誤差を小さくすることで学びます：誤差の傾きに沿って、小さな一歩
# ずつ下っていきます。それが勾配降下法です。
#   derivative(fn(x) = x * x, 3)                      # 6.0、3 での傾き
#   gradient(fn([x, y]) = x * x + y * y, [1, 2])      # [2.0, 4.0]
#
# 問題
#   - `step(f, x, rate)`：x から一歩下る：x - rate × (x における f の傾き)
#   - `descend(f, x, rate, steps)`：x からその回数だけ続けて進んだ、到達点
#   - `descend_all(f, xs, rate, steps)`：複数の数で同じこと：各ステップは
#     xs - rate × (xs における f の勾配)
#
# ヒント
#   1. `descend` は 0 になるまでステップを 1 つ減らして自分自身を呼びます
#      （または `1..steps` で `fold` を使う）。
#   2. `vector_sub` と `vector_scale` は数のリストを扱います。
#
# 答えをチェックする：  rythme exercise check 34-gradient-descent.ry
#             または：  rythme test 34-gradient-descent.ry

fn step(f, x, rate) = fail("未完成")

fn descend(f, x, rate, steps) = fail("未完成")

fn descend_all(f, xs, rate, steps) = fail("未完成")

# --- テスト（変更しないでください）---

fn parabola(x) = (x - 3) ** 2

test "一歩"
  assert_eq(step(parabola, 0, 0.5), 3.0)
  assert_eq(step(parabola, 1, 0.25), 2.0)
end

test "底まで"
  assert_eq(round(descend(parabola, 0, 0.1, 100), 4), 3.0)
  assert_eq(descend(parabola, 10, 0.1, 0), 10)
end

test "複数の数"
  bowl = fn([x, y]) = (x - 1) ** 2 + (y + 2) ** 2
  assert_eq(map(descend_all(bowl, [0, 0], 0.1, 100), fn(v) = round(v, 4)), [1.0, -2.0])
end
"#,
        ),
        ("35-nearest-neighbours", "35-nearest-neighbours"),
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
            r#"# 練習問題 35 · 最近傍法  ★★★
#
# 新しい点のラベルを推測するには、それにいちばん近い k 個の既知の例を見て、
# その多くが持つラベルを選びます。各例はレコードです：
#   {point: [1.0, 2.0], label: "red"}
#
# 問題
#   - `neighbours(examples, point, k)`：点にいちばん近い k 個の例を近い順に
#   - `majority(labels)`：最も多く出てくるラベル（同数が複数あれば、リストの
#     順でその回数に最初に達したもの）
#   - `classify(examples, point, k)`：k 個の最近傍の多くが持つラベル
#
# ヒント
#   1. 距離（`norm(vector_sub(a, b))`）で `sort_by` してから `take`。
#   2. `count(labels, fn(l) = l == label)` で 1 つのラベルを数えます。`max_by`
#      は最大のもののうち最初のものを残します。
#
# 答えをチェックする：  rythme exercise check 35-nearest-neighbours.ry
#             または：  rythme test 35-nearest-neighbours.ry

fn neighbours(examples, point, k) = fail("未完成")

fn majority(labels) = fail("未完成")

fn classify(examples, point, k) = fail("未完成")

# --- テスト（変更しないでください）---

garden = [
  {point: [1, 1], label: "red"},
  {point: [1, 2], label: "red"},
  {point: [2, 1], label: "red"},
  {point: [8, 8], label: "blue"},
  {point: [9, 8], label: "blue"},
  {point: [5, 5], label: "blue"},
]

test "近傍"
  assert_eq(neighbours(garden, [0, 0], 2) |> map(fn(e) = e.point), [[1, 1], [1, 2]])
  assert_eq(len(neighbours(garden, [0, 0], 10)), 6)
end

test "多数決"
  assert_eq(majority(["a", "b", "b", "c"]), "b")
  assert_eq(majority(["x", "y"]), "x")
end

test "分類"
  assert_eq(classify(garden, [2, 2], 3), "red")
  assert_eq(classify(garden, [7, 7], 3), "blue")
  assert_eq(classify(garden, [4, 4], 1), "blue")
end
"#,
        ),
        ("36-minimax", "36-minimax"),
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
            r#"# 練習問題 36 · 完璧に遊ぶ：ミニマックス  ★★★
#
# ニムのゲーム：テーブルにマッチが n 本あり、各プレイヤーは順番に 1、2、3 本の
# どれかを取り、最後のマッチを取った人が勝ちです。ミニマックスはすべての手を
# 想像して、完璧に遊んだときにだれが勝つかを決めます：手番のプレイヤーは、
# 相手を負けの局面に追い込む手が 1 つでもあれば勝ちます。
#
# 問題
#   - `moves(n)`：取れるマッチの本数（1、2、3 のどれかで、n 以下）
#   - `wins(n)`：マッチが n 本のとき、手番のプレイヤーが必ず勝てるなら true
#     （マッチが 0 本なら、手番のプレイヤーはすでに負けている）
#   - `best_move(n)`：勝てる手のうち最小の m を `Some(m)` で、どの手でも
#     負けるなら `None`
#
# ヒント
#   1. `filter(1..3, ...)` で許される手を残します。
#   2. `wins(n)`：その後で相手が勝てなくなる手が 1 つでも（`any`）あるか？
#   3. `find` は当てはまる最初の要素を Option で返します。
#
# 答えをチェックする：  rythme exercise check 36-minimax.ry
#             または：  rythme test 36-minimax.ry

fn moves(n) = fail("未完成")

fn wins(n) = fail("未完成")

fn best_move(n) = fail("未完成")

# --- テスト（変更しないでください）---

test "手"
  assert_eq(moves(10), [1, 2, 3])
  assert_eq(moves(2), [1, 2])
  assert_eq(moves(0), [])
end

test "だれが勝つか"
  assert_eq(map(0..8, wins), [false, true, true, true, false, true, true, true, false])
end

test "最善の手"
  assert_eq(best_move(5), Some(1))
  assert_eq(best_move(7), Some(3))
  assert_eq(best_move(8), None)
end
"#,
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
  Keep API keys out of the code: env("API_KEY").

A mistake of shape names both shapes ("cannot multiply a 2×3 matrix by a
2×2 matrix"). Programs to read: the examples linear_regression, perceptron,
xor_network, kmeans, knn, naive_bayes, markov, tictactoe, pathfinding and
genetic; exercises 31 to 36 (rythme exercise list)."#,
            r#"機械学習と AI

ベクトルは数のリスト、行列は同じ長さの行のリストです。これらの関数は
それを変えず、常に Float の新しいリストを返します。

ベクトルと行列
  dot([1, 2], [3, 4])            # 11.0
  norm([3, 4])                   # 5.0、長さ
  vector_add(a, b)  vector_sub(a, b)  vector_mul(a, b)   # 数ごとに
  vector_scale([1, 2], 3)        # [3.0, 6.0]
  matmul(m, n)  matmul(m, v)     # 行列の積、または行列とベクトルの積
  transpose(m)  shape(m)  zeros(2, 3)  ones(2)  identity_matrix(3)
  zip_with(xs, ys, f)            # 組ごとに f。それ以外のすべてに

ニューロン
  sigmoid(x)  tanh(x)  relu(x)   softmax(scores)   # 点数を確率に

統計
  stats(xs)                      # {count, mean, median, std_dev, min, max}
  variance(xs)  std_dev(xs)  percentile(xs, 90)  argmax(xs)  argmin(xs)
  normalize(xs)                  # 0.0（最小）から 1.0（最大）へ

乱数（random_seed(n) の後は毎回同じ）
  random_normal(0, 1)  random_choice_weighted(xs, weights)
  train_test_split(examples, 0.8)    # {train, test}

傾き（勾配降下法のため。中心差分で約 9 桁）
  derivative(fn(x) = x * x, 3)               # 6.0
  gradient(fn([x, y]) = x * x + y, [2, 5])   # [4.0, 1.0]

テキストのグラフ
  print(plot(losses))  print(plot(xs, ys))  print(histogram(heights))

Web と言語モデル
  http_get(url)  http_post(url, body, headers)   # Ok(テキスト) か Err(message)
  API キーはコードに書かない：env("API_KEY")。

形の誤りは両方の形を示します（「2×3 の行列に 2×2 の行列を掛けられない」）。
読んでみるプログラム：例 linear_regression、perceptron、xor_network、kmeans、
knn、naive_bayes、markov、tictactoe、pathfinding、genetic。練習問題 31〜36
（rythme exercise list）。"#,
        ),
        // Types with parameters: src/check/annot.rs, src/help.rs, src/lsp/features.rs, src/parser.rs
        ("the type variable `{name}` appears twice in `{ty}`", "型変数 `{name}` が `{ty}` に 2 回出てくる"),
        ("give each one its own letter: `type Pair(a, b)`", "それぞれに別の文字を付ける：`type Pair(a, b)`"),
        ("`{name}` cannot be a type variable: a type variable is one lowercase letter", "`{name}` は型変数にできない：型変数は小文字 1 文字"),
        ("write one letter, like `{letter}`, here and in the fields", "`{letter}` のような 1 文字を、こことフィールドに書く"),
        ("`{v}` is not a parameter of `{ty}`", "`{v}` は `{ty}` のパラメーターではない"),
        ("not among `{header}`", "`{header}` の中にない"),
        ("add it to the parameters of the type: `type {with} = ...`", "型のパラメーターに加える：`type {with} = ...`"),
        ("`{name}` takes {count} types: `{example}`", "`{name}` は型を {count} 個取る：`{example}`"),
        (
            r#"Types may also be written in the code, where they help to read it:
  fn area(s: Shape) -> Float
`rythme check` verifies them; they are never needed (rythme help types)."#,
            r#"型は、読みやすくなる場所ではコードに書くこともできます：
  fn area(s: Shape) -> Float
`rythme check` がそれを確かめます。決して必須ではありません（rythme help types）。"#,
        ),
        ("A whole number, as large as needed: `42`, `-7`.", "必要なだけ大きくなる整数：`42`、`-7`。"),
        ("A decimal number: `3.14`, `2.0`.", "小数：`3.14`、`2.0`。"),
        ("A text: `\"hello\"`.", "テキスト：`\"hello\"`。"),
        ("`true` or `false`.", "`true` か `false`。"),
        ("`()`: what a function gives when it gives nothing, like `print`.", "`()`：`print` のように、何も返さない関数が返すもの。"),
        ("A value that is only its name: `:ok`, `:error`.", "名前だけの値：`:ok`、`:error`。"),
        ("A list whose elements all have the type between parentheses; `List` alone: any list.", "要素がすべて括弧の中の型であるリスト。`List` だけなら任意のリスト。"),
        ("A set of values of this type: `set([1, 2])`.", "この型の値の集合：`set([1, 2])`。"),
        ("A dictionary: keys of the first type, values of the second.", "辞書：キーは 1 つ目の型、値は 2 つ目の型。"),
        ("`Some(value)` or `None`: a value that may be missing.", "`Some(value)` か `None`：ないかもしれない値。"),
        ("`Ok(value)` or `Err(error)`: the result of what may fail.", "`Ok(value)` か `Err(error)`：失敗するかもしれないものの結果。"),
        ("Any record; `{{name: Text}}` is a record with at least these fields.", "任意のレコード。`{{name: Text}}` は少なくともこれらのフィールドを持つレコード。"),
        ("Any function; `fn(Int) -> Int` is a function with these types.", "任意の関数。`fn(Int) -> Int` はこれらの型の関数。"),
        ("A type of Rythme: `rythme help types`", "Rythme の型：`rythme help types`"),
        ("type variable", "型変数"),
        ("used file", "使っているファイル"),
        ("a type variable, like `a`", "`a` のような型変数"),
        ("a type without parameters is written without parentheses: `type {name} = ...`", "パラメーターのない型は括弧なしで書く：`type {name} = ...`"),
        ("the parameters of a type are type variables, one lowercase letter each: `type Box(a) = Box(value: a)`", "型のパラメーターは型変数で、それぞれ小文字 1 文字：`type Box(a) = Box(value: a)`"),
        // src/index.rs, src/packages.rs, src/project.rs, src/manifest.rs, src/help.rs: the package index
        ("the address is empty", "アドレスが空"),
        ("`{a}` is not an address that Rythme accepts: no spaces, and no `-` at the start", "`{a}` は Rythme が受け付けるアドレスではない：空白なし、先頭に `-` なし"),
        ("Rythme downloads packages only with `https://`, `ssh://` or from this computer (`file://`), not with `{other}`", "Rythme がパッケージをダウンロードするのは `https://`、`ssh://`、このコンピューター（`file://`）からだけ（`{other}` は不可）"),
        ("`{a}` is not the address of a repository: write `https://...`, `ssh://...` or `file://...`", "`{a}` はリポジトリのアドレスではない：`https://...`、`ssh://...`、`file://...` のどれかを書く"),
        ("set by `index` in `rythme.toml`", "`rythme.toml` の `index` で設定"),
        ("set by the variable RYTHME_INDEX", "環境変数 RYTHME_INDEX で設定"),
        ("`{url}` is not a package index: it has no folder `packages`", "`{url}` はパッケージインデックスではない：フォルダー `packages` がない"),
        ("an index holds one file per package, like `packages/mathlib.toml` (see `rythme help packages`)", "インデックスにはパッケージごとに 1 つのファイルがある。例：`packages/mathlib.toml`（`rythme help packages` を参照）"),
        ("an unknown day", "不明な日"),
        ("cannot update the package index from `{url}`: using the copy of {date}", "`{url}` からパッケージインデックスを更新できない：{date} のコピーを使う"),
        ("cannot download the package index from `{url}`", "`{url}` からパッケージインデックスをダウンロードできない"),
        ("check the connection to the network, or give another index with the variable RYTHME_INDEX (an address or a folder); dependencies from git or from a folder work without the index", "ネットワークの接続を確かめるか、環境変数 RYTHME_INDEX で別のインデックス（アドレスかフォルダー）を指定する。git やフォルダーからの依存関係はインデックスなしで使える"),
        ("no package `{name}` in the package index", "パッケージインデックスにパッケージ `{name}` はない"),
        ("`rythme search WORDS` finds packages by the words of their name and description", "`rythme search WORDS` は名前と説明の単語でパッケージを探す"),
        ("the index: {place}", "インデックス：{place}"),
        ("{url} (copy of {date})", "{url}（{date} のコピー）"),
        ("{} (withdrawn)", "{}（取り下げ済み）"),
        ("none", "なし"),
        ("no version of `{name}` in the package index fits `{range}`", "パッケージインデックスに `{range}` に合う `{name}` のバージョンはない"),
        ("its versions: {}", "バージョン：{}"),
        ("for the newest one, write `{name} = {{ version = \"^{v}\" }}`", "最新のものなら `{name} = {{ version = \"^{v}\" }}` と書く"),
        ("every version of `{name}` was withdrawn", "`{name}` のすべてのバージョンが取り下げられている"),
        ("`{shown}` is too large for an entry of the package index", "`{shown}` はパッケージインデックスの項目には大きすぎる"),
        ("`{key}` is a list of texts", "`{key}` はテキストのリスト"),
        ("this entry of the package index is for `{found}`, but its file is named for `{name}`", "パッケージインデックスのこの項目は `{found}` のものだが、ファイル名は `{name}` のもの"),
        ("the entry of `{name}` has no `repository`", "`{name}` の項目に `repository` がない"),
        ("the entry of `{name}` has no `{key}`", "`{name}` の項目に `{key}` がない"),
        ("each version is written under `[[version]]`", "各バージョンは `[[version]]` の下に書く"),
        ("this version of `{name}` has no `{k}`", "`{name}` のこのバージョンに `{k}` がない"),
        ("`{commit}` is not a full commit: 40 hexadecimal digits in lower case", "`{commit}` は完全なコミットではない：小文字の 16 進数 40 桁"),
        ("`{checksum}` is not a checksum: `sha256:` and 64 hexadecimal digits, as `rythme publish` writes it", "`{checksum}` はチェックサムではない：`rythme publish` が書くように `sha256:` と 16 進数 64 桁"),
        ("`{t}` is not a tag", "`{t}` はタグではない"),
        ("the version {v} is written twice", "バージョン {v} が 2 回書かれている"),
        ("the entry of `{name}` has no version", "`{name}` の項目にバージョンがない"),
        ("expected `,` or `]` here", "ここには `,` か `]` が必要"),
        ("a list is written on one line: `[\"math\", \"vectors\"]`", "リストは 1 行で書く：`[\"math\", \"vectors\"]`"),
        ("like `{key} = {example}`", "例：`{key} = {example}`"),
        ("write `\"^1.2.0\"` (1.2.0 or a newer compatible version, below 2.0.0), `\"~1.2.0\"` (1.2.x), `\"=1.2.0\"` (exactly this one) or `\">=1.2.0, <1.5.0\"`", "`\"^1.2.0\"`（1.2.0 か、2.0.0 未満の互換性のある新しいバージョン）、`\"~1.2.0\"`（1.2.x）、`\"=1.2.0\"`（ちょうどこれ）、`\">=1.2.0, <1.5.0\"` のどれかを書く"),
        ("for a version from the package index, write `{name} = {{ version = \"{v}\" }}`", "パッケージインデックスのバージョンなら `{name} = {{ version = \"{v}\" }}` と書く"),
        ("a dependency comes from the package index (`version`), from git (`git`) or from a folder (`path`): give only one", "依存関係はパッケージインデックス（`version`）、git（`git`）、フォルダー（`path`）のどれか 1 つから取る：1 つだけ指定する"),
        ("`{v}` is not a range of versions", "`{v}` はバージョンの範囲ではない"),
        (
            r#"unknown option `{bad}` for `rythme {command}`
see `rythme help packages`"#,
            r#"`rythme {command}` に不明なオプション `{bad}`
`rythme help packages` を参照"#,
        ),
        ("{bad} file(s) of the package index could not be read", "パッケージインデックスの {bad} 個のファイルを読めなかった"),
        ("{n} package(s) in {place}", "{place} にパッケージが {n} 個"),
        ("No package found for `{query}` in {place}", "{place} に `{query}` のパッケージは見つかりませんでした"),
        ("{n} package(s) found for `{query}` in {place}", "{place} で `{query}` のパッケージが {n} 個見つかりました"),
        ("More about one: `rythme info {name}`; add it to a project: `rythme add {name}`", "詳しくは `rythme info {name}`、プロジェクトに加えるには `rythme add {name}`"),
        ("`rythme search --update{words}` searches the newest copy of the index", "`rythme search --update{words}` はインデックスの最新のコピーを検索する"),
        ("which package? Example: rythme info mathlib", "どのパッケージ？ 例：rythme info mathlib"),
        ("repository", "リポジトリ"),
        ("license", "ライセンス"),
        ("keywords", "キーワード"),
        ("authors", "作者"),
        ("versions", "バージョン"),
        ("  {v} was withdrawn: {}", "  {v} は取り下げられた：{}"),
        ("Add it to a project: `rythme add {name}` (writes `{name} = {{ version = \"^{v}\" }}` in rythme.toml)", "プロジェクトに加えるには：`rythme add {name}`（rythme.toml に `{name} = {{ version = \"^{v}\" }}` を書く）"),
        ("Every version of this package was withdrawn.", "このパッケージのすべてのバージョンが取り下げられています。"),
        ("which package? Example: rythme add mathlib (or rythme add mathlib@1.2)", "どのパッケージ？ 例：rythme add mathlib（または rythme add mathlib@1.2）"),
        ("cannot read `{}`: {e}", "`{}` を読めない：{e}"),
        ("`{range_text}` is not a range of versions", "`{range_text}` はバージョンの範囲ではない"),
        ("`{name}` came from {from}: it now comes from the package index", "`{name}` は {from} からだったが、今はパッケージインデックスから取る"),
        ("`rythme.toml` was left as it was", "`rythme.toml` はそのままにした"),
        ("Added `{line}` to rythme.toml ({version})", "rythme.toml に `{line}` を加えました（{version}）"),
        ("  use it with `use {name}/FILE` (see `rythme info {name}`)", "  `use {name}/FILE` で使う（`rythme info {name}` を参照）"),
        ("which entries? Example: rythme publish --verify packages/mathlib.toml", "どの項目？ 例：rythme publish --verify packages/mathlib.toml"),
        (
            r#"`rythme publish` takes no argument (`{a}`): it checks the package of the current folder
see `rythme help packages`"#,
            r#"`rythme publish` は引数を取らない（`{a}`）：現在のフォルダーのパッケージをチェックする
`rythme help packages` を参照"#,
        ),
        ("`rythme.toml` has no `{key}`", "`rythme.toml` に `{key}` がない"),
        ("the package index shows it, and needs it", "パッケージインデックスはそれを表示し、それを必要とする"),
        ("add a line like `{example}` at the top", "先頭に `{example}` のような行を加える"),
        ("Checking `{name}` {version} before it goes into the package index", "パッケージインデックスに入れる前に `{name}` {version} をチェック中"),
        ("`{version}` is not a version of semantic versioning", "`{version}` はセマンティックバージョニングのバージョンではない"),
        ("like `version = \"1.2.0\"`, without zeros at the start of a number", "例：`version = \"1.2.0\"`（数の先頭に 0 を付けない）"),
        ("description = \"What `{name}` is for, in one sentence\"", "description = \"`{name}` の用途を 1 文で\""),
        ("the `description` is too long: one sentence, at most 200 characters", "`description` が長すぎる：1 文で、最大 200 文字"),
        ("YOU", "あなた"),
        ("`{k}` is not a keyword: a word in lower case, at most 24 characters", "`{k}` はキーワードではない：小文字の単語で、最大 24 文字"),
        ("at most 5 keywords", "キーワードは最大 5 個"),
        ("a published package cannot use a folder of this computer: `{dep_name}` comes from `{p}`", "公開するパッケージはこのコンピューターのフォルダーを使えない：`{dep_name}` は `{p}` から取っている"),
        ("publish `{dep_name}` too, then write `{dep_name} = {{ version = \"^...\" }}`", "`{dep_name}` も公開してから、`{dep_name} = {{ version = \"^...\" }}` と書く"),
        ("`{name}` is not in a git repository", "`{name}` は git リポジトリの中にない"),
        ("the package index gives the commit of each version: a package is published from its git repository", "パッケージインデックスは各バージョンのコミットを示す：パッケージはその git リポジトリから公開する"),
        ("`git init`, `git add .`, `git commit`, then push it to its `repository`", "`git init`、`git add .`、`git commit` の後、その `repository` にプッシュする"),
        ("`rythme.toml` must be at the root of its git repository", "`rythme.toml` は git リポジトリのルートになければならない"),
        ("the root of the repository: {top}", "リポジトリのルート：{top}"),
        ("give the package a repository of its own", "パッケージ専用のリポジトリを用意する"),
        ("some changes are not committed", "コミットされていない変更がある"),
        ("for example: {}", "例：{}"),
        ("commit them (`git add`, `git commit`), or set them aside, then tag the commit", "それらをコミットする（`git add`、`git commit`）か、退避してから、コミットにタグを付ける"),
        ("no tag `v{version}` for the version {version}", "バージョン {version} のタグ `v{version}` がない"),
        ("tag the commit and push the tag: `git tag v{version}`, then `git push origin v{version}`", "コミットにタグを付けてプッシュする：`git tag v{version}` の後に `git push origin v{version}`"),
        ("the tag `{t}` is not the current commit", "タグ `{t}` は現在のコミットではない"),
        ("what is checked must be what is published", "チェックするものと公開するものは同じでなければならない"),
        ("publish from the tag (`git checkout {t}`), or move the tag to this commit (`git tag --force {t}`) before pushing it", "タグから公開する（`git checkout {t}`）か、プッシュする前にタグをこのコミットに移す（`git tag --force {t}`）"),
        ("Running `rythme check`", "`rythme check` を実行中"),
        ("`rythme check` found errors", "`rythme check` がエラーを見つけた"),
        ("fix them first: a package must pass `rythme check`", "先に直す：パッケージは `rythme check` を通らなければならない"),
        ("Running `rythme test`", "`rythme test` を実行中"),
        ("`rythme test` failed", "`rythme test` が失敗した"),
        ("fix them first: a package must pass its tests", "先に直す：パッケージは自分のテストに成功しなければならない"),
        ("the name `{name}` is already taken in the package index", "名前 `{name}` はパッケージインデックスですでに使われている"),
        ("by the repository {repo}", "リポジトリ {repo} によって"),
        ("choose another `name` for your package", "パッケージに別の `name` を選ぶ"),
        ("the version {version} of `{name}` is already in the package index", "`{name}` のバージョン {version} はすでにパッケージインデックスにある"),
        ("a published version never changes: raise `version` in `rythme.toml`, commit and tag again", "公開したバージョンは決して変わらない：`rythme.toml` の `version` を上げ、もう一度コミットしてタグを付ける"),
        ("cannot read the package index `{url}`: it will check the name and the version itself", "パッケージインデックス `{url}` を読めない：名前とバージョンはインデックス側でチェックされる"),
        ("the tag `{t}` is not in {repo}", "タグ `{t}` は {repo} にない"),
        ("push it: `git push origin {t}`", "プッシュする：`git push origin {t}`"),
        ("the tag `{t}` of {repo} is not this commit", "{repo} のタグ `{t}` はこのコミットではない"),
        ("push the tag again: `git push --force origin {t}`", "タグをもう一度プッシュする：`git push --force origin {t}`"),
        ("cannot reach {repo} to check that the tag `{t}` is there", "タグ `{t}` があるか確かめるために {repo} に接続できない"),
        ("cannot read the files of the commit `{commit}`", "コミット `{commit}` のファイルを読めない"),
        ("✓ `{name}` {version} is ready for the package index.", "✓ `{name}` {version} はパッケージインデックスに入れる準備ができました。"),
        ("Rythme never publishes anything by itself: a version goes into the index with a pull request on its repository, {}.", "Rythme が自分で何かを公開することはありません：バージョンはインデックスのリポジトリへのプルリクエストで入ります（{}）。"),
        ("1. Fork the repository of the index and clone your fork. Create the file {file} with:", "1. インデックスのリポジトリをフォークし、そのフォークをクローンします。ファイル {file} を次の内容で作ります："),
        ("1. Fork the repository of the index and clone your fork. Add at the end of {file}:", "1. インデックスのリポジトリをフォークし、そのフォークをクローンします。{file} の末尾に次を加えます："),
        ("2. Commit it on a branch of your fork and push it:", "2. フォークのブランチにコミットしてプッシュします："),
        ("3. Open a pull request on {}: its checks run `rythme publish --verify {file}`, then a maintainer merges it.", "3. {} にプルリクエストを出します：そのチェックで `rythme publish --verify {file}` が実行され、メンテナーがマージします。"),
        (
            r#"
{problems} problem(s) to fix before publishing: nothing was sent anywhere"#,
            r#"
公開する前に直す問題が {problems} 件：何も送信していない"#,
        ),
        ("`{f}` is not an entry of the index: `packages/NAME.toml`", "`{f}` はインデックスの項目ではない：`packages/NAME.toml`"),
        ("✓ {f}: `{stem}`, {count} version(s) verified", "✓ {f}：`{stem}`、{count} 個のバージョンを確認"),
        ("✗ {f}", "✗ {f}"),
        ("{v}: the commit `{commit}` is not in `{url}`", "{v}：コミット `{commit}` は `{url}` にない"),
        ("{v}: the tag `{tag}` of `{url}` is not the commit `{commit}`", "{v}：`{url}` のタグ `{tag}` はコミット `{commit}` ではない"),
        ("{v}: the files of the commit do not have the checksum of the entry", "{v}：コミットのファイルのチェックサムが項目のものと違う"),
        (
            r#"expected: {}
found:    {found}"#,
            r#"期待値：{}
実際：  {found}"#,
        ),
        ("`rythme publish` in the package, at its tag, writes the right one", "パッケージでそのタグに対して `rythme publish` を実行すれば、正しい値が書かれる"),
        ("{v}: the commit has no `rythme.toml` at its root", "{v}：コミットのルートに `rythme.toml` がない"),
        ("{v}: the `rythme.toml` of the commit says `{found_name}` {found_version}", "{v}：コミットの `rythme.toml` は `{found_name}` {found_version} となっている"),
        ("{v}: the dependency `{dep_name}` is a folder of the computer of the author", "{v}：依存関係 `{dep_name}` は作者のコンピューターのフォルダー"),
        ("the package index, version {range}", "パッケージインデックス、バージョン {range}"),
        ("the commit `{commit}` of `{name}` {version} does not exist in `{url}`", "`{name}` {version} のコミット `{commit}` は `{url}` にない"),
        ("the package index gives this commit for this version", "パッケージインデックスはこのバージョンにこのコミットを示している"),
        ("tell the authors of `{name}`: the history of their repository may have been rewritten", "`{name}` の作者に伝える：リポジトリの履歴が書き換えられたのかもしれない"),
        ("cannot read the files of `{name}` {version}", "`{name}` {version} のファイルを読めない"),
        ("the files of `{name}` {version} do not match the checksum of the package index", "`{name}` {version} のファイルがパッケージインデックスのチェックサムと合わない"),
        ("nothing was installed, and nothing of the package ran", "何もインストールしておらず、パッケージのものは何も実行していない"),
        ("do not use this version; tell the maintainers of the package index", "このバージョンは使わない。パッケージインデックスのメンテナーに伝える"),
        ("Updating the dependencies of `{name}`", "`{name}` の依存関係を更新中"),
        ("{range} from the index, commit {short} ({how})", "インデックスの {range}、コミット {short}（{how}）"),
        ("the package index gives `{dep_name}` {wanted}, but its `rythme.toml` says `{found}` {version}", "パッケージインデックスは `{dep_name}` {wanted} を示しているが、その `rythme.toml` は `{found}` {version} となっている"),
        ("tell the maintainers of the package index", "パッケージインデックスのメンテナーに伝える"),
        ("`rythme update` takes the names of dependencies, not `{bad}`: `rythme update` (all of them) or `rythme update mathlib`", "`rythme update` が取るのは依存関係の名前（`{bad}` は不可）：`rythme update`（すべて）か `rythme update mathlib`"),
        ("`{a}` is not a dependency of `{}`", "`{a}` は `{}` の依存関係ではない"),
        ("did you mean `rythme update {best}`?", "もしかして `rythme update {best}`？"),
        ("`rythme add {a}` adds it from the package index", "`rythme add {a}` でパッケージインデックスから加えられる"),
        ("Everything was already up to date", "すべてすでに最新でした"),
        ("Updated:", "更新："),
        ("{} {} (new)", "{} {}（新規）"),
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
            r#"パッケージインデックス

  rythme search vectors      # 名前、説明、キーワードにそれを含むパッケージ
  rythme info mathlib        # 説明、リポジトリ、ライセンス、バージョン
  rythme add mathlib         # rythme.toml に `mathlib = { version = "^1.2.0" }` を加え、
                             #   インストールする（rythme.lock が正確なコミットを保持）
  rythme add mathlib@1.1     # 好きなバージョンの範囲で
  rythme update              # 各依存関係の範囲内で最新のバージョンに
  rythme publish             # パッケージをチェックし、追加の方法を示す

インデックスからの依存関係はバージョンの範囲を指定します：

  [dependencies]
  mathlib = { version = "^1.2.0" }    # 1.2.0 以上、2.0.0 未満（デフォルト）
  geometry = { version = "~0.3.1" }   # 0.3.1 以上の 0.3.x
  tools = { version = "=1.0.0" }      # ちょうどこれ

`rythme install` は範囲内である限り rythme.lock のバージョンを使い、
`rythme update` は最新のものを使います。取り下げられたバージョンは決して
選ばれません。

インデックスは git リポジトリで、パッケージごとに 1 つのファイル
（packages/NAME.toml）があり、名前、説明、リポジトリ、ライセンス、
キーワードと、バージョンごとのタグ、コミット、ファイルのチェックサムを
持ちます。Rythme はそのコピーを ~/.rythme/index に保管し、ネットワークなしで
読みます：`rythme search --update` で最新のものを取得します。別の
インデックスを使うには rythme.toml に `index = "..."`、または環境変数
RYTHME_INDEX（アドレスか、このコンピューターのフォルダー）。デフォルトは
https://github.com/maxlestage/rythme-index です。

安全性：名前は Rythme の名前（`/` も `..` もなし）、アドレスは https、ssh、
file:// を使い、git はフックなしで実行され、各バージョンのファイルは
インデックスのチェックサムと一致しなければならず、パッケージのものは
インストール時に何も実行されません。

公開：バージョンにタグを付けます（git tag v1.2.0、git push origin v1.2.0）。
次に `rythme publish` が rythme.toml（description、license、repository）、
`rythme check`、`rythme test`、タグをチェックし、プルリクエストでインデックスに
加える項目を表示します。Rythme が自分で何かを送ることはありません。"#,
        ),
        ("The index is the folder {url}, where Rythme writes nothing.", "インデックスはフォルダー {url} で、Rythme はそこに何も書きません。"),
        ("Create its file {file} with:", "そのファイル {file} を次の内容で作ります："),
        ("Add at the end of its file {file}:", "そのファイル {file} の末尾に次を加えます："),
        ("the pattern \"{pattern}\" has an unknown directive `%{c}`", "パターン \"{pattern}\" に不明な指示子 `%{c}` がある"),
        ("the directives are %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second), %y (year in 2 digits), %B and %b (month name), %A and %a (day name), %I and %p (12-hour clock), %j (day of the year), %z (offset) and %% (a %)", "指示子は %Y（年）、%m（月）、%d（日）、%H（時）、%M（分）、%S（秒）、%y（2 桁の年）、%B と %b（月の名前）、%A と %a（曜日の名前）、%I と %p（12 時間制）、%j（年内の日）、%z（時差）、%%（% そのもの）"),
        ("\"{text}\" does not follow the pattern \"{pattern}\"", "\"{text}\" はパターン \"{pattern}\" に従っていない"),
        ("the hour must go from 1 to 12 with AM or PM, not {}", "AM か PM 付きの時は 1 から 12 までで、{} は範囲外"),
        ("the day of the year must go from 1 to {len}, not {n}", "年内の日は 1 から {len} までで、{n} は範囲外"),
        ("`{c}` is not a Base64 character", "`{c}` は Base64 の文字ではない"),
        ("this Base64 text is cut short", "この Base64 テキストは途中で切れている"),
        ("the decoded bytes are not a text (UTF-8)", "デコードしたバイトはテキスト（UTF-8）ではない"),
        ("a `%` must be followed by two hexadecimal digits, like `%20`", "`%` の後には 16 進数 2 桁が必要。例：`%20`"),
        ("`{f}` would give {count} lists: too many (at most {MAX_COMBINED} elements in all)", "`{f}` はリストを {count} 個返すことになる：多すぎる（全体で最大 {MAX_COMBINED} 要素）"),
        ("the number of combinations grows very fast: try with a shorter list, or fewer elements in each one", "組み合わせの数はとても速く増える：もっと短いリストか、それぞれの要素を減らして試す"),
        ("`unzip` needs a list of lists, but its element {k} is {}", "`unzip` にはリストのリストが必要だが、その要素 {k} は {}"),
        ("the function given to `unfold` must return `Some([value, next])` or `None`, but it returned Some({})", "`unfold` に渡す関数は `Some([value, next])` か `None` を返さなければならないが、Some({}) を返した"),
        ("`{f}` needs at least two numbers in each list", "`{f}` には各リストに少なくとも 2 つの数が必要"),
        ("`correlation` needs numbers that vary: one of the lists has all its numbers equal", "`correlation` には変化する数が必要：どちらかのリストの数がすべて等しい"),
        ("`{f}` cannot compare {} and {}", "`{f}` は {} と {} を比較できない"),
        ("the value and its bounds must be of the same kind: all numbers, or all texts...", "値とその上下限は同じ種類でなければならない：すべて数、すべてテキストなど"),
        ("`constrain`: the lower bound {} is above the upper bound {}", "`constrain`：下限 {} が上限 {} より大きい"),
        ("`binomial` needs a number of 0 or more, not {n}", "`binomial` には 0 以上の数が必要（{n} は不可）"),
        ("`expect` found `None`: there is no value", "`expect` が `None` を見つけた：値がない"),
        ("`expect` found an error: {}", "`expect` がエラーを見つけた：{}"),
        ("`all_ok` needs a list of Results (Ok or Err), but its element {k} is {}", "`all_ok` には Result（Ok か Err）のリストが必要だが、その要素 {k} は {}"),
        ("to keep the values of a list of Options, use `filter_map(list, fn(x) = x)`", "Option のリストの値を残すには `filter_map(list, fn(x) = x)` を使う"),
        ("`format` cannot use \"{spec}\": {why}", "`format` は \"{spec}\" を使えない：{why}"),
        ("a spec is written [[fill]align][sign][#][0][width][,][.precision][type]: \">8\" (8 wide, on the right), \"^8\" (centered), \"08.2f\" (2 decimals, zeros before), \",d\" (thousands), \".1%\" (a percentage), \"x\" (hexadecimal), \"b\" (binary), \"e\" (exponent)", "書式指定は [[fill]align][sign][#][0][width][,][.precision][type] と書く：\">8\"（幅 8、右寄せ）、\"^8\"（中央）、\"08.2f\"（小数 2 桁、前を 0 で埋める）、\",d\"（3 桁区切り）、\".1%\"（百分率）、\"x\"（16 進数）、\"b\"（2 進数）、\"e\"（指数）"),
        ("the width is at most {MAX_WIDTH}", "幅は最大 {MAX_WIDTH}"),
        ("the precision is at most {MAX_PRECISION}", "精度は最大 {MAX_PRECISION}"),
        ("a number must follow the `.`", "`.` の後には数が必要"),
        ("`{c}` is not a type of format", "`{c}` は書式の型ではない"),
        ("this is not a format spec", "これは書式指定ではない"),
        ("`{kind}` cannot write {what}", "`{kind}` は{what}を書けない"),
        ("a whole number", "整数"),
        ("a whole number has no decimals: write a type like `f` for them", "整数に小数部分はない：小数には `f` のような型を書く"),
        ("`,` groups decimal digits only: use `_`", "`,` は 10 進数の桁しか区切らない：`_` を使う"),
        ("a decimal number", "小数"),
        ("signs, `#`, `0` and `,` are for numbers", "符号、`#`、`0`、`,` は数のためのもの"),
        ("`{f}`: the base must go from 2 to 36, not {b}", "`{f}`：基数は 2 から 36 までで、{b} は範囲外"),
        ("\"{s}\" is not a whole number in base {radix}", "\"{s}\" は {radix} 進数の整数ではない"),
        ("`digits` needs a number of 0 or more, not {}", "`digits` には 0 以上の数が必要（{} は不可）"),
        ("`{f}`: the text to look for is empty", "`{f}`：探すテキストが空"),
        ("`{f}` cannot replace an empty text", "`{f}` は空のテキストを置換できない"),
        ("`edit_distance` compares texts of at most {MAX_DISTANCE_WORK} pairs of characters, not {} by {}", "`edit_distance` が比べられるのは最大 {MAX_DISTANCE_WORK} 組の文字のテキスト（{} × {} は不可）"),
        ("to count a piece of text in a text, use `occurrences(text, piece)`", "テキスト中の部分テキストを数えるには `occurrences(text, piece)` を使う"),
        ("to count the elements equal to a value, use `occurrences(list, x)`", "ある値に等しい要素を数えるには `occurrences(list, x)` を使う"),
        ("the value comes first, then the spec: `format(42, \"05\")`; to put values in a text, use braces", "値が先、書式指定が後：`format(42, \"05\")`。テキストに値を入れるには波かっこを使う"),
        ("return `Some(x)` to keep x, or `None` to leave the element out", "x を残すなら `Some(x)`、要素を除くなら `None` を返す"),
        ("return `Some([value, next])` to go on, or `None` to stop", "続けるなら `Some([value, next])`、止めるなら `None` を返す"),
        ("Pairs the elements of two lists (or more: one element of each list in every group): [[a1, b1], [a2, b2], ...]. It stops at the end of the shortest list.", "2 つ（以上）のリストの要素を組にします（各グループに各リストの要素が 1 つずつ）：[[a1, b1], [a2, b2], ...]。いちばん短いリストの終わりで止まります。"),
        ("Reads a date written as ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", with `Z` or an offset like `+01:00` if needed): `Ok(date)`, or `Err(message)` if it is not a valid date. With a pattern like \"%d/%m/%Y\" (the directives of `format_date`), reads a date written that way.", "ISO 8601 で書いた日付（\"2024-02-29\"、\"2024-02-29T13:05:00\"、必要なら `Z` か `+01:00` のような時差付き）を読みます：`Ok(日付)`、有効な日付でなければ `Err(message)`。\"%d/%m/%Y\" のようなパターン（`format_date` の指示子）を渡すと、その形で書いた日付を読みます。"),
        ("Writes a date as ISO 8601: \"2024-02-29\", with the time if the date has one, and `Z` or the offset if it has one. With a pattern, writes it the way `strftime` does: %Y year, %m month, %d day, %H hour, %M minute, %S second, %y year in two digits, %B and %b the month, %A and %a the day of the week (in English), %I and %p the 12-hour clock, %j the day of the year, %z the offset, %% a %.", "日付を ISO 8601 で書きます：\"2024-02-29\"。日付に時刻があれば時刻も、時差があれば `Z` か時差も書きます。パターンを渡すと `strftime` と同じように書きます：%Y 年、%m 月、%d 日、%H 時、%M 分、%S 秒、%y 2 桁の年、%B と %b 月、%A と %a 曜日（英語）、%I と %p 12 時間制、%j 年内の日、%z 時差、%% は %。"),
        ("The text that shows a value the way the REPL and `debug` show it: a text between quotes, with its special characters written as escapes. `text` gives a text as it is.", "REPL や `debug` と同じように値を表すテキスト：引用符で囲み、特殊文字をエスケープで書いたテキスト。`text` はテキストをそのまま返します。"),
        ("Shows values on the error output (stderr), separated by spaces, then goes to the next line: for messages that must not mix with the results of a program, which go to the standard output.", "値を空白で区切ってエラー出力（stderr）に表示し、改行します：標準出力に出るプログラムの結果と混ざってはいけないメッセージのためのものです。"),
        ("The value kept between two bounds: `low` if it is below, `high` if it is above, itself otherwise. For numbers, texts... any values that can be compared.", "2 つの境界の間に収めた値：下回れば `low`、上回れば `high`、それ以外はそのまま。数、テキストなど、比較できるどんな値にも使えます。"),
        ("The whole quotient and the remainder of a division at once: `[a // b, a % b]`.", "割り算の整数の商と余りを一度に：`[a // b, a % b]`。"),
        ("The binomial coefficient \"n choose k\": the number of ways to choose k elements among n, exact even when it is very large.", "二項係数「n 個から k 個を選ぶ」：n 個の中から k 個を選ぶ方法の数。とても大きくても正確です。"),
        ("The digits of a whole number (0 or more), the most significant first, in base 10 or in the base given (from 2 to 36).", "整数（0 以上）の各桁を、上の桁から、10 進数または指定した基数（2 から 36）で。"),
        ("Writes a whole number in a base from 2 to 36, with the digits 0 to 9 then the letters a to z.", "整数を 2 から 36 までの基数で、0〜9 の数字と a〜z の英字を使って書きます。"),
        ("Reads a whole number written in a base from 2 to 36: `Ok(number)`, or `Err(message)` if the text is not a number in this base. `0x`, `0o` and `0b` may come before a number in base 16, 8 and 2.", "2 から 36 までの基数で書いた整数を読みます：`Ok(数)`、その基数の数でなければ `Err(message)`。16 進数、8 進数、2 進数の前には `0x`、`0o`、`0b` を付けてもかまいません。"),
        ("Is the number greater than 0?", "数は 0 より大きいか？"),
        ("Is the number less than 0?", "数は 0 より小さいか？"),
        ("Is the number 0 (or 0.0)?", "数は 0（または 0.0）か？"),
        ("Is the decimal number `nan` (\"not a number\")? It is never equal to anything, not even to itself.", "小数が `nan`（非数）か？ `nan` は何とも、自分自身とさえ等しくなりません。"),
        ("Is the number finite: neither an infinity nor `nan`? Whole numbers always are.", "数は有限か（無限大でも `nan` でもないか）？ 整数は常に有限です。"),
        ("Adds `fill` (a space by default) on both sides of the text until it is `width` characters long: titles and cells of tables stay centered. A character left over goes to the right.", "テキストが `width` 文字になるまで、両側に `fill`（デフォルトは空白）を加えます：表の見出しやセルが中央にそろいます。余った 1 文字は右に入ります。"),
        ("Writes a value following a format spec, like Python's `format`: \".2f\" for 2 decimals, \">8\" to fill 8 characters on the right (\"<\" on the left, \"^\" centered), \"08.2f\" with zeros before, \",\" with the thousands separated, \"%\" as a percentage, \"x\", \"o\" and \"b\" in hexadecimal, octal and binary (\"#\" adds 0x, 0o, 0b), \"e\" with an exponent. To put values in a text, use braces.", "Python の `format` のように、書式指定に従って値を書きます：\".2f\" は小数 2 桁、\">8\" は右寄せで 8 文字分（\"<\" は左寄せ、\"^\" は中央）、\"08.2f\" は前を 0 で埋める、\",\" は 3 桁区切り、\"%\" は百分率、\"x\"、\"o\"、\"b\" は 16 進数、8 進数、2 進数（\"#\" で 0x、0o、0b を付ける）、\"e\" は指数付き。テキストに値を入れるには波かっこを使います。"),
        ("Replaces the first occurrence of `old` by `new`, and only that one.", "最初の `old` だけを `new` に置き換えます。"),
        ("How many times an element is in a list, or a piece of text in a text (pieces that do not overlap).", "リストの中の要素、またはテキストの中の部分テキスト（重ならないもの）の個数。"),
        ("The position of the last occurrence of an element in a list, or of a piece of text in a text: `Some(position)`, or `None` if it is not there.", "リストの中の要素、またはテキストの中の部分テキストが最後に出てくる位置：`Some(位置)`、なければ `None`。"),
        ("The text without this start, if it starts with it; otherwise the text unchanged.", "テキストがこの部分で始まれば、それを除いたテキスト。そうでなければテキストそのまま。"),
        ("The text without this end, if it ends with it; otherwise the text unchanged.", "テキストがこの部分で終われば、それを除いたテキスト。そうでなければテキストそのまま。"),
        ("The text with the first character of each word in uppercase (the rest is unchanged). Words are separated by spaces.", "各単語の最初の文字を大文字にしたテキスト（残りはそのまま）。単語は空白で区切られます。"),
        ("Writes a name in snake_case: its words in lowercase, joined by `_`. Reads names written in any style: camelCase, PascalCase, kebab-case, with spaces...", "名前を snake_case で書きます：単語を小文字にして `_` でつなぎます。どんな書き方の名前でも読みます：camelCase、PascalCase、kebab-case、空白入りなど。"),
        ("Writes a name in camelCase: its words glued together, each with an uppercase first letter but the first word.", "名前を camelCase で書きます：単語をくっつけ、最初の単語以外は頭文字を大文字にします。"),
        ("Writes a name in kebab-case: its words in lowercase, joined by `-`, as in web addresses.", "名前を kebab-case で書きます：Web アドレスのように、単語を小文字にして `-` でつなぎます。"),
        ("The text with its uppercase letters in lowercase, and its lowercase letters in uppercase.", "大文字を小文字に、小文字を大文字にしたテキスト。"),
        ("Has the text letters, all of them in uppercase? (The other characters do not count.)", "テキストに英字があり、そのすべてが大文字か？（他の文字は数えない。）"),
        ("Has the text letters, all of them in lowercase? (The other characters do not count.)", "テキストに英字があり、そのすべてが小文字か？（他の文字は数えない。）"),
        ("The bytes of a text in UTF-8, as whole numbers from 0 to 255: an accented letter takes two of them, an emoji four.", "テキストの UTF-8 のバイトを、0 から 255 までの整数で：アクセント付きの文字は 2 バイト、絵文字は 4 バイトです。"),
        ("Cuts a text in two at the first separator: `Some([before, after])`, or `None` if the separator is not there. For lines like `key=value`.", "テキストを最初の区切り文字で 2 つに切ります：`Some([前, 後])`、区切り文字がなければ `None`。`key=value` のような行に使います。"),
        ("Cuts the lines of a text between words so that none is longer than `width` characters (a longer word stays whole on its own line).", "どの行も `width` 文字より長くならないように、テキストの行を単語の間で切ります（それより長い単語は 1 行にそのまま置きます）。"),
        ("The edit distance (Levenshtein) between two texts: the fewest characters to add, remove or replace to go from one to the other. For spelling checkers, and to find the closest word.", "2 つのテキストの編集距離（レーベンシュタイン距離）：一方から他方へ変えるのに必要な、文字の追加・削除・置換の最小回数。スペルチェックや、いちばん近い単語を探すのに使います。"),
        ("The position of the first element for which f is true: `Some(position)`, or `None` if there is none.", "f が真になる最初の要素の位置：`Some(位置)`、なければ `None`。"),
        ("The list with the element at position i replaced by f(element). A negative position counts from the end.", "位置 i の要素を f(要素) に置き換えたリスト。負の位置は末尾から数えます。"),
        ("The list or text turned by n places to the left (1 by default): the first elements go to the end. A negative n turns it to the right.", "リストやテキストを n 個（デフォルトは 1）左に回したもの：最初の要素が末尾に行きます。n が負なら右に回します。"),
        ("The list with `x` between every two elements.", "各要素の間に `x` を入れたリスト。"),
        ("The elements of two lists taken in turn, one from each; what is left of the longer list comes at the end.", "2 つのリストから 1 つずつ交互に取った要素。長いほうのリストの残りは最後に来ます。"),
        ("Counts how many times each value is in a list: a dict from each value to its count (like Python's `Counter`).", "リストの中に各値が何回あるかを数えます：各値からその個数への辞書（Python の `Counter` と同じ）。"),
        ("Counts the elements by the key that f gives them: a dict from each key to how many elements have it.", "f が与えるキーで要素を数えます：各キーから、そのキーを持つ要素の数への辞書。"),
        ("The values that come most often, with their counts: a list of `[value, count]` pairs, the most frequent first (on a tie, the one seen first). With n, only the first n.", "最もよく出てくる値とその個数：`[値, 個数]` のペアのリストで、多い順（同数なら先に出てきたもの）。n を指定すると最初の n 個だけ。"),
        ("The list without the elements whose key (given by f) was already seen: the first element of each key stays.", "（f が与える）キーがすでに出てきた要素を除いたリスト：各キーの最初の要素が残ります。"),
        ("The list sorted from the largest to the smallest.", "大きい順に並べたリスト。"),
        ("The list sorted by the key that the function gives, from the largest key to the smallest; elements with the same key keep their order.", "関数が与えるキーで、大きいキーから小さいキーの順に並べたリスト。同じキーの要素は元の順序を保ちます。"),
        ("Cuts a list into runs of neighbour elements that have the same key (given by f).", "リストを、（f が与える）同じキーを持つ隣り合う要素の連なりに切ります。"),
        ("Cuts a list or a text in two at position i: `[before, from i on]`. A negative position counts from the end.", "リストやテキストを位置 i で 2 つに切ります：`[前, i 以降]`。負の位置は末尾から数えます。"),
        ("Every way to choose k elements of the list, each kept in the order of the list.", "リストから k 個の要素を選ぶすべての方法（それぞれリストの順を保つ）。"),
        ("Every order of the elements of the list, or of k of them.", "リストの要素（または k 個の要素）のすべての並べ方。"),
        ("Every way to take one element of each list: the cartesian product.", "各リストから 1 つずつ要素を取るすべての方法：直積。"),
        ("The opposite of `zip`: a list of pairs (or of lists of the same length) becomes the list of the first elements, the list of the second ones...", "`zip` の逆：ペア（または同じ長さのリスト）のリストを、最初の要素のリスト、2 番目の要素のリスト……に変えます。"),
        ("A list of n values: `start`, then f(start), then f of that... For sequences where each value comes from the one before.", "n 個の値のリスト：`start`、次に f(start)、次にその f……。各値が前の値から決まる数列に使います。"),
        ("Builds a list from a starting state: f receives the state and gives `Some([value, next state])` to add a value and go on, or `None` to stop.", "初期状態からリストを作ります：f は状態を受け取り、値を加えて続けるなら `Some([値, 次の状態])`、止めるなら `None` を返します。"),
        ("Applies f to each element and keeps the values of the `Some(...)` it gives, leaving out the `None`: filtering and transforming at once.", "各要素に f を適用し、返された `Some(...)` の値を残して `None` を除きます：絞り込みと変換を一度に。"),
        ("The value found by following a path of keys into records, dicts and lists nested in one another (names of fields, keys, positions): `Some(value)`, or `None` as soon as one is missing. For data read with `parse_json`.", "入れ子になったレコード、辞書、リストの中をキーの道筋（フィールド名、キー、位置）に沿ってたどって見つけた値：`Some(値)`、途中でどれかがなければ `None`。`parse_json` で読んだデータに使います。"),
        ("Merges two records or two dicts; for a key in both, f receives the two values and gives the one kept.", "2 つのレコードか 2 つの辞書をまとめます。両方にあるキーでは、f が 2 つの値を受け取って残す値を返します。"),
        ("A dict where the values of a record or a dict have become the keys, and the keys the values. When a value comes several times, the last key wins.", "レコードや辞書の値をキーに、キーを値にした辞書。同じ値が複数あれば、最後のキーが優先されます。"),
        ("The record or dict with only the keys given (in a list or a set); a key that is missing is ignored.", "指定したキー（リストか集合で）だけを持つレコードや辞書。ないキーは無視されます。"),
        ("The record or dict without the keys given (in a list or a set).", "指定したキー（リストか集合で）を除いたレコードや辞書。"),
        ("The record or dict with each key replaced by f(key); for a record, f must give a text.", "各キーを f(キー) に置き換えたレコードや辞書。レコードなら f はテキストを返さなければなりません。"),
        ("The record or dict where the value of `key` is replaced by f(value); when the key is missing, it is added with `default`. Handy to count: `update(counts, word, 1, fn(n) = n + 1)`.", "`key` の値を f(値) に置き換えたレコードや辞書。キーがなければ `default` で加えます。数えるのに便利です：`update(counts, word, 1, fn(n) = n + 1)`。"),
        ("The elements that are in exactly one of the two sets.", "2 つの集合のちょうど一方にだけある要素。"),
        ("Have the two sets no element in common?", "2 つの集合に共通の要素がないか？"),
        ("The value inside `Some(x)` or `Ok(x)`; otherwise what f gives: f receives nothing for `None`, and the error for `Err(e)`. The default is computed only when it is needed.", "`Some(x)` や `Ok(x)` の中の値。そうでなければ f が返すもの：`None` なら f は何も受け取らず、`Err(e)` ならエラーを受け取ります。デフォルト値は必要なときだけ計算されます。"),
        ("The value inside `Some(x)` or `Ok(x)`, like `unwrap`; otherwise stops the program with your message, which says what was expected.", "`unwrap` と同じく `Some(x)` や `Ok(x)` の中の値。そうでなければ、何を期待していたかを伝えるあなたのメッセージでプログラムを止めます。"),
        ("Makes one Result of a list of Results: `Ok(list of the values)` if they are all `Ok`, or else the first `Err`. When reading many values where one mistake must stop everything.", "Result のリストを 1 つの Result にします：すべて `Ok` なら `Ok(値のリスト)`、そうでなければ最初の `Err`。多くの値を読んで、1 つの誤りで全体を止めたいときに使います。"),
        ("The date n months later (earlier if n is negative). A day that the new month does not have becomes its last day: 31 January plus one month is the end of February.", "n か月後（n が負なら前）の日付。新しい月にない日は、その月の最後の日になります：1 月 31 日の 1 か月後は 2 月の末日です。"),
        ("The number of the day in its year, from 1 (1 January) to 365, or 366 in a leap year.", "その年の中の日の番号：1（1 月 1 日）から 365、うるう年なら 366 まで。"),
        ("n decimal numbers evenly spaced from `start` to `end`, both included: to draw a function with `plot`, or to try values.", "`start` から `end` まで（両端を含む）等間隔に並んだ n 個の小数：`plot` で関数を描いたり、値を試したりするのに使います。"),
        ("The correlation (Pearson) of two lists of numbers of the same length: from -1 (they vary in opposite ways) to 1 (they vary together); about 0 when they are not linearly related.", "同じ長さの 2 つの数のリストの相関（ピアソン）：-1（逆に変化する）から 1（一緒に変化する）まで。線形の関係がなければ 0 に近くなります。"),
        ("The covariance of two lists of numbers of the same length, divided by their length like `variance`.", "同じ長さの 2 つの数のリストの共分散。`variance` と同じく長さで割ります。"),
        ("Encodes a text in Base64 (its UTF-8 bytes), as in emails, web pages and the headers of HTTP requests.", "メール、Web ページ、HTTP リクエストのヘッダーのように、テキスト（その UTF-8 のバイト）を Base64 でエンコードします。"),
        ("Decodes a Base64 text: `Ok(text)`, or `Err(message)` if it is not valid Base64, or not a text once decoded.", "Base64 テキストをデコードします：`Ok(テキスト)`、正しい Base64 でないかデコードしてもテキストにならなければ `Err(message)`。"),
        ("Encodes a text to put it in a web address: every character but the letters, the digits and `-_.~` becomes `%` and two hexadecimal digits (a space is `%20`).", "Web アドレスに入れるためにテキストをエンコードします：英字、数字、`-_.~` 以外の文字は `%` と 16 進数 2 桁になります（空白は `%20`）。"),
        ("Decodes a text of a web address (`%20`...): `Ok(text)`, or `Err(message)` if a `%` is not followed by two hexadecimal digits.", "Web アドレスのテキスト（`%20` など）をデコードします：`Ok(テキスト)`、`%` の後に 16 進数 2 桁がなければ `Err(message)`。"),
        ("Cube root: the number whose cube is x, for negative numbers too.", "立方根：3 乗すると x になる数。負の数でも使えます。"),
        ("The length of the hypotenuse, or the distance from (0, 0) to the point (x, y): `sqrt(x * x + y * y)`, without overflow for very large numbers.", "斜辺の長さ、つまり (0, 0) から点 (x, y) までの距離：`sqrt(x * x + y * y)`。とても大きな数でもオーバーフローしません。"),
        ("Hyperbolic sine.", "双曲線正弦。"),
        ("Hyperbolic cosine.", "双曲線余弦。"),
        ("Inverse hyperbolic sine: the number whose `sinh` is x.", "逆双曲線正弦：`sinh` が x になる数。"),
        ("Inverse hyperbolic cosine: the number (0 or more) whose `cosh` is x, for x of 1 or more.", "逆双曲線余弦：`cosh` が x になる数（0 以上）。x は 1 以上。"),
        ("Inverse hyperbolic tangent: the number whose `tanh` is x, for x between -1 and 1 (both excluded).", "逆双曲線正接：`tanh` が x になる数。x は -1 と 1 の間（両端を除く）。"),
        ("The last element for which f is true: `Some(element)`, or `None` if there is none.", "f が真になる最後の要素：`Some(要素)`、なければ `None`。"),
        ("The position of the last element for which f is true: `Some(position)`, or `None` if there is none.", "f が真になる最後の要素の位置：`Some(位置)`、なければ `None`。"),
        ("Waits `seconds` seconds (0.5 for half a second), then the program goes on: between two requests to a web service, or to show something step by step.", "`seconds` 秒（0.5 なら半秒）待ってから、プログラムを続けます：Web サービスへの 2 つのリクエストの間や、何かを一歩ずつ見せるときに使います。"),
        ("A new random identifier (UUID version 4): a text of 36 characters, to name the records of a database or files. Its numbers come from `random` (`random_seed` makes them repeatable): it is not for secrets.", "新しいランダムな識別子（UUID バージョン 4）：36 文字のテキストで、データベースのレコードやファイルに名前を付けるのに使います。その数字は `random` から来ます（`random_seed` で再現できます）：秘密には使えません。"),
        ("The SHA-256 digest of a text (of its UTF-8 bytes), in 64 hexadecimal digits: to check that a text or a file did not change. Not for passwords, which need a slow hash.", "テキスト（その UTF-8 のバイト）の SHA-256 ダイジェストを 16 進数 64 桁で返します：テキストやファイルが変わっていないことを確かめるためのものです。パスワードには使えません。パスワードには遅いハッシュが必要です。"),
        ("Cuts a web address into its parts: `Ok({scheme, user, password, host, port, path, query, fragment})`, or `Err(message)` if it is not a web address. The parts are texts as written, still encoded (empty when absent: `url_decode` decodes them, `parse_query` reads the query), except `port`, an `Option`; the scheme and the host are in lower case.", "Web アドレスを部分に分けます：`Ok({scheme, user, password, host, port, path, query, fragment})`、Web アドレスでなければ `Err(message)`。各部分は書かれたままのテキストで、エンコードされたままです（ないときは空。`url_decode` でデコードし、`parse_query` でクエリを読みます）。ただし `port` は `Option` です。スキームとホストは小文字になります。"),
        ("Reads the query of a web address (what follows the `?`, like `q=caf%C3%A9&page=2`) into a dict of texts, decoded (`+` is a space). A name given several times keeps its last value. Never fails: what cannot be decoded stays as it is.", "Web アドレスのクエリ（`?` の後の部分、たとえば `q=caf%C3%A9&page=2`）を、デコードしたテキストの辞書として読みます（`+` は空白）。何度も出てくる名前は最後の値になります。失敗はしません：デコードできない部分はそのまま残ります。"),
        ("Writes a record or a dict as the query of a web address: `name=value` joined by `&`, encoded like `url_encode`. A value that is not a text is shown as `print` shows it; a list gives the name once for each element.", "レコードか辞書を Web アドレスのクエリとして書きます：`&` でつないだ `name=value` で、`url_encode` と同じようにエンコードします。テキストでない値は `print` と同じように表示し、リストは要素ごとに 1 回ずつ名前を書きます。"),
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
            r#"リストのその他の関数
  find_index([3, 8, 5], is_even)          # Some(1)
  sort_desc([3, 1, 2])                    # [3, 2, 1]  (sort_by_desc with a key)
  tally(["a", "b", "a"])                  # dict("a" => 2, "b" => 1)
  most_common(words, 3)                   # 最も多い 3 つとその個数
  split_at([1, 2, 3], 1)                  # [[1], [2, 3]]
  rotate_by([1, 2, 3], 1)                 # [2, 3, 1]
  combinations([1, 2, 3], 2)              # [[1, 2], [1, 3], [2, 3]]
  iterate(1, 5, fn(x) = x * 2)            # [1, 2, 4, 8, 16]
  permutations(xs)   cartesian_product(xs, ys)   unzip(pairs)   zip(xs, ys, zs)
  filter_map(xs, f)   chunk_by(xs, f)   unique_by(xs, f)   count_by(xs, f)"#,
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
            r##"テキストのその他の関数
  format(3.14159, ".2f")         # "3.14"  （Python の書式指定：">8"、",d"、".1%"、"#x" など）
  center("hi", 6, "*")           # "**hi**"
  replace_first("a-b-c", "-", "+")          # "a+b-c"
  occurrences("banana", "an")    # 2
  remove_prefix("Mr Smith", "Mr ")          # "Smith"  （remove_suffix も）
  split_once("key=value", "=")   # Some(["key", "value"])
  snake_case("helloWorld")       # "hello_world"  （camel_case、kebab_case も）
  capitalize_words("ada lovelace")          # "Ada Lovelace"
  word_wrap(text, 40)            # 最大 40 文字の行
  to_base(255, 16)   from_base("ff", 16)    # "ff" と Ok(255)
  edit_distance("kitten", "sitting")        # 3"##,
        ),
        (
            r#"More functions on records and dicts
  dig(data, ["user", "langs", 0])           # Some(...), or None: deep inside
  select_keys(person, ["name", "age"])      # only these keys (reject_keys: without them)
  merge_with(a, b, fn(x, y) = x + y)        # f decides for a key in both
  update(counts, "apples", 1, fn(n) = n + 1)  # +1, or 1 when missing
  invert({a: 1, b: 2})                      # dict(1 => "a", 2 => "b")
  map_keys(r, upper)   symmetric_difference(s, t)   is_disjoint(s, t)"#,
            r#"レコードと辞書のその他の関数
  dig(data, ["user", "langs", 0])           # Some(...) か None：奥深くまで
  select_keys(person, ["name", "age"])      # これらのキーだけ（reject_keys：これらを除く）
  merge_with(a, b, fn(x, y) = x + y)        # 両方にあるキーは f が決める
  update(counts, "apples", 1, fn(n) = n + 1)  # +1、なければ 1
  invert({a: 1, b: 2})                      # dict(1 => "a", 2 => "b")
  map_keys(r, upper)   symmetric_difference(s, t)   is_disjoint(s, t)"#,
        ),
        (
            r#"More helpers
  unwrap_or_else(x, fn(e) = ...)   # the default, computed only when it is needed
  expect(x, "a config file")       # like unwrap, with your message if it fails
  all_ok(map(texts, parse_int))    # Ok([...]), or the first Err
  filter_map(xs, f)                # the values of the Some(...) that f gives"#,
            r#"その他の便利な関数
  unwrap_or_else(x, fn(e) = ...)   # デフォルト値。必要なときだけ計算する
  expect(x, "a config file")       # unwrap と同じで、失敗したらあなたのメッセージ
  all_ok(map(texts, parse_int))    # Ok([...]) か最初の Err
  filter_map(xs, f)                # f が返す Some(...) の値"#,
        ),
        (
            r#"More on dates
  add_months(date, 1)                       # 31 January + 1 month: the end of February
  day_of_year(date)                         # from 1 to 366
  format_date(date, "%d/%m/%Y %H:%M")       # with a pattern: %Y %m %d %H %M %S %B %A...
  parse_date("29/02/2024", "%d/%m/%Y")      # Ok({year: 2024, month: 2, day: 29})"#,
            r#"日付のその他の関数
  add_months(date, 1)                       # 1 月 31 日 + 1 か月：2 月の末日
  day_of_year(date)                         # 1 から 366 まで
  format_date(date, "%d/%m/%Y %H:%M")       # パターン付き：%Y %m %d %H %M %S %B %A...
  parse_date("29/02/2024", "%d/%m/%Y")      # Ok({year: 2024, month: 2, day: 29})"#,
        ),
        // src/cli.rs
        ("run a program; the words after the file are its arguments", "プログラムを実行する。ファイルの後の単語はその引数"),
        ("find problems without running the program", "プログラムを実行せずに問題を見つける"),
        ("run the tests of a file or of a folder", "ファイルかフォルダーのテストを実行する"),
        ("give the code its official layout", "コードを公式のレイアウトに整形する"),
        ("write the documentation of a file from its ## comments", "## コメントからファイルのドキュメントを書く"),
        ("apply the safe fixes of the problems that `rythme check` finds", "`rythme check` が見つけた問題の安全な修正を適用する"),
        ("show a calculation step by step", "計算を 1 ステップずつ表示する"),
        ("pause a program where you want and look at its values", "好きな場所でプログラムを一時停止し、値を調べる"),
        ("the debugger for editors (Debug Adapter Protocol)", "エディター向けのデバッガー（Debug Adapter Protocol）"),
        ("exercises checked automatically", "自動でチェックされる練習問題"),
        ("create a project (rythme.toml, src/, tests/)", "プロジェクトを作る（rythme.toml、src/、tests/）"),
        ("download the dependencies of the project", "プロジェクトの依存関係をダウンロードする"),
        ("guided tour of the language, step by step", "言語のガイドツアー（1 ステップずつ）"),
        ("help on a topic, a function or an error code", "トピック、関数、エラーコードのヘルプ"),
        ("explain an error or warning code, like E003", "E003 のようなエラーや警告のコードを説明する"),
        ("run a piece of code given on the command line", "コマンドラインで渡したコードの断片を実行する"),
        ("interactive mode: try code line by line (also `rythme` alone)", "対話モード：コードを 1 行ずつ試す（`rythme` だけでも同じ）"),
        ("language server, for editors (VS Code, Neovim...)", "エディター向けの言語サーバー（VS Code、Neovim...）"),
        ("server for AI assistants (Model Context Protocol)", "AI アシスタント向けのサーバー（Model Context Protocol）"),
        ("the version of Rythme", "Rythme のバージョン"),
        ("completion of the commands in a shell", "シェルでのコマンドの補完"),
        ("the manual page of the command, to install with `man`", "コマンドのマニュアルページ（`man` でインストールする）"),
        ("how to install the latest version of Rythme", "Rythme の最新版のインストール方法"),
        (
            r#"  --seed N        the same draws of random and shuffle on every run
  --sandbox       no files, folders, environment variables nor network
  --format json   the output and the errors as JSON, for tools
  FILE may be `-`: the program is read from the standard input.
  At the root of a project, `rythme run` runs its program."#,
            r#"  --seed N        random と shuffle の結果を毎回同じにする
  --sandbox       ファイル、フォルダー、環境変数、ネットワークなし
  --format json   出力とエラーを JSON で（ツール向け）
  FILE は `-` でもよい：プログラムを標準入力から読む。
  プロジェクトのルートでは、`rythme run` はそのプログラムを実行する。"#,
        ),
        (
            r#"  --format json   the problems as JSON, for tools and AI assistants
  A folder checks every .ry file inside it; at the root of a project,
  `rythme check` checks the files of the project.
  Exit code: 0 without errors (warnings allowed), 1 with errors."#,
            r#"  --format json   問題を JSON で（ツールと AI アシスタント向け）
  フォルダーを指定すると中のすべての .ry ファイルをチェックする。プロジェクトの
  ルートでは、`rythme check` はプロジェクトのファイルをチェックする。
  終了コード：エラーがなければ 0（警告はよい）、エラーがあれば 1。"#,
        ),
        (
            r#"  --filter NAME   only the tests whose name contains NAME
  --seed N        the same draws of random on every run
  --watch         run the tests again whenever a file changes
  --sandbox       no files, folders, environment variables nor network
  --format json   the results as JSON
  Exit code: 0 when every test passes, 1 otherwise."#,
            r#"  --filter NAME   名前に NAME を含むテストだけ
  --seed N        random の結果を毎回同じにする
  --watch         ファイルが変わるたびにテストを再実行する
  --sandbox       ファイル、フォルダー、環境変数、ネットワークなし
  --format json   結果を JSON で
  終了コード：すべてのテストが成功すれば 0、そうでなければ 1。"#,
        ),
        (
            r#"  --check         only check, for continuous integration: exit code 1 when a
                  file must be formatted
  A folder formats all its .ry files; `-` reads the code from the standard
  input and writes it formatted."#,
            r#"  --check         チェックだけ（継続的インテグレーション向け）：整形が必要な
                  ファイルがあれば終了コード 1
  フォルダーを指定するとすべての .ry ファイルを整形する。`-` は標準入力から
  コードを読み、整形して書く。"#,
        ),
        (
            r#"  --dry-run       only show the changes, as a diff
  --all           also apply the fixes that are only likely
  --format json   the fixes as JSON
  Exit code: 0 when no error is left, 1 otherwise (with --dry-run: 1 when
  something would change)."#,
            r#"  --dry-run       変更を diff で表示するだけ
  --all           確実ではない修正も適用する
  --format json   修正を JSON で
  終了コード：エラーが残らなければ 0、そうでなければ 1（--dry-run では、
  何かが変わるなら 1）。"#,
        ),
        (
            r#"  --max N         show at most N steps
  --width N       cut the lines at N characters (20 at least)
  Without an expression, the steps of the last line of the file."#,
            r#"  --max N         最大 N ステップまで表示する
  --width N       行を N 文字で切る（20 以上）
  式がなければ、ファイルの最後の行のステップ。"#,
        ),
        (
            r#"  In the debugger, `help` lists its commands: break, run, next, step,
  finish, where, print, locals, quit."#,
            r#"  デバッガーでは `help` でコマンドを一覧できる：break、run、next、step、
  finish、where、print、locals、quit。"#,
        ),
        ("  Your editor starts it: the VS Code extension does, for the debugger type `rythme`.", "  エディターが起動する：VS Code 拡張機能がデバッガーの種類 `rythme` で起動する。"),
        (
            r#"  list            the exercises, from the easiest
  start NAME      write the file of an exercise, to complete
  check FILE.ry   run its tests and give hints"#,
            r#"  list            練習問題（やさしいものから）
  start NAME      練習問題のファイルを書く（完成させる）
  check FILE.ry   そのテストを実行してヒントを出す"#,
        ),
        (
            r#"  `rythme help` lists the topics, `rythme help builtins` every function,
  `rythme help llms` the whole language in one text (for AI assistants)."#,
            r#"  `rythme help` はトピックを、`rythme help builtins` はすべての関数を、
  `rythme help llms` は言語全体を 1 つのテキストで（AI アシスタント向け）一覧する。"#,
        ),
        ("  Without a code, the list of the codes; --format json gives them all.", "  コードがなければコードの一覧。--format json ですべてを出す。"),
        ("  The value of the last expression is shown, unless it is ().", "  最後の式の値を表示する（() なら表示しない）。"),
        ("  `:help` lists the commands of the interactive mode.", "  `:help` で対話モードのコマンドを一覧できる。"),
        ("  --verbose       also the system, the languages, the functions and the formats", "  --verbose       システム、言語、関数、形式も表示する"),
        ("Then open a new shell.", "その後、新しいシェルを開いてください。"),
        (
            r#"  rythme man > rythme.1 && man ./rythme.1
  To install it: put rythme.1 in a folder of the manual pages, like
  ~/.local/share/man/man1."#,
            r#"  rythme man > rythme.1 && man ./rythme.1
  インストールするには：rythme.1 をマニュアルページのフォルダー
  （~/.local/share/man/man1 など）に置く。"#,
        ),
        (
            r#"
Options of every command: --lang CODE ({codes}), --no-color
"#,
            r#"
すべてのコマンドのオプション：--lang CODE（{codes}）、--no-color
"#,
        ),
        (
            r#"See also `rythme help {topic}`.
"#,
            r#"`rythme help {topic}` も参照してください。
"#,
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
            r#"
ツール
  rythme CMD --help         コマンドのオプション（例：rythme test --help）
  rythme run -              標準入力から読んだプログラムを実行する
  rythme check FOLDER       フォルダーのすべての .ry ファイルをチェックする
  rythme completions SHELL  コマンドの補完：bash、zsh、fish、powershell
  rythme man                コマンドのマニュアルページ（rythme man > rythme.1）
  rythme version --verbose  バージョンと、システムと Rythme の内容
  rythme upgrade            最新版のインストール方法
"#,
        ),
        ("unknown option `{bad}` for `rythme version`: the option is --verbose", "`rythme version` に不明なオプション `{bad}`：オプションは --verbose"),
        (
            r#"system: {os} {arch}
languages of the messages: {codes} (now: {now})
built-in functions: {functions} (and {ruby} Ruby names)
codes: {errors} errors ({e_range}), {warnings} warnings ({w_range})
JSON output (--format json): version {json}
program: {exe}
"#,
            r#"システム：{os} {arch}
メッセージの言語：{codes}（現在：{now}）
組み込み関数：{functions}（と Ruby での名前 {ruby} 個）
コード：エラー {errors} 個（{e_range}）、警告 {warnings} 個（{w_range}）
JSON 出力（--format json）：バージョン {json}
プログラム：{exe}
"#,
        ),
        ("`rythme upgrade` takes no argument, not `{bad}`", "`rythme upgrade` は引数を取らない（`{bad}` は不可）"),
        (
            r#"This is rythme {v} ({exe}).
To install the latest version, type:
  {how}
The versions and what changed: https://github.com/maxlestage/Rythme/releases
Within 1.x, a program that runs keeps running, with the same output.
"#,
            r#"これは rythme {v}（{exe}）です。
最新版をインストールするには、次のように入力します：
  {how}
バージョンと変更点：https://github.com/maxlestage/Rythme/releases
1.x の間は、動くプログラムは同じ出力で動き続けます。
"#,
        ),
        ("unknown shell `{s}`.", "不明なシェル `{s}`。"),
        ("which shell?", "どのシェル？"),
        (
            r#"{what} Write one of bash, zsh, fish, powershell, like `rythme completions bash`
see `rythme completions --help`"#,
            r#"{what} bash、zsh、fish、powershell のどれかを書く。例：`rythme completions bash`
`rythme completions --help` を参照"#,
        ),
        ("`rythme man` takes no argument, not `{bad}`", "`rythme man` は引数を取らない（`{bad}` は不可）"),
        ("Name", "名前"),
        ("rythme - a small, purely functional language with help everywhere", "rythme - どこにでもヘルプがある、小さな純粋関数型言語"),
        ("Synopsis", "書式"),
        ("Description", "説明"),
        ("Rythme runs, checks, tests and formats programs written in Rythme. Without a command, `rythme` starts the interactive mode; `rythme file.ry` runs a program. Every message has a code explained by `rythme explain CODE`, and `rythme help` shows help on the language and its functions.", "Rythme は Rythme で書いたプログラムを実行、チェック、テスト、整形します。コマンドなしの `rythme` は対話モードを始め、`rythme file.ry` はプログラムを実行します。どのメッセージにも `rythme explain CODE` で説明されるコードがあり、`rythme help` は言語とその関数のヘルプを表示します。"),
        ("Commands", "コマンド"),
        ("Options", "オプション"),
        ("the language of the messages: {codes}", "メッセージの言語：{codes}"),
        ("messages without colors", "色なしのメッセージ"),
        ("the help; after a command, its options", "ヘルプ。コマンドの後ならそのオプション"),
        ("Environment", "環境変数"),
        ("the language of the messages, before LC_ALL, LC_MESSAGES and LANG", "メッセージの言語（LC_ALL、LC_MESSAGES、LANG より優先）"),
        ("when set, messages without colors", "設定されていれば、色なしのメッセージ"),
        ("Exit status", "終了ステータス"),
        ("0 when everything went well; 1 after an error in the program, a failed test or a problem found; 2 for a command written wrongly. A program may choose its own code with exit(code).", "すべてうまくいけば 0。プログラムのエラー、テストの失敗、問題の発見の後は 1。コマンドの書き間違いは 2。プログラムは exit(code) で自分のコードを選べます。"),
        ("See also", "関連項目"),
        ("no .ry file in `{a}`", "`{a}` に .ry ファイルがない"),
        ("the standard input cannot be read: {e}", "標準入力を読めない：{e}"),
        // src/cli.rs: the commands of the package index
        ("search the package index", "パッケージインデックスを検索する"),
        ("what the index says of a package: its versions, repository and license", "インデックスにあるパッケージの情報：バージョン、リポジトリ、ライセンス"),
        ("add a package of the index to the project, then install it", "インデックスのパッケージをプロジェクトに加え、インストールする"),
        ("take the newest versions that the ranges of rythme.toml allow", "rythme.toml の範囲が許す最新のバージョンにする"),
        ("check a package and print its entry for the index (sends nothing)", "パッケージをチェックし、インデックス用の項目を表示する（何も送らない）"),
        (
            r#"  --update        download the newest copy of the index first
  --format json   the packages found as JSON
  Without words, every package of the index."#,
            r#"  --update        先にインデックスの最新のコピーをダウンロードする
  --format json   見つかったパッケージを JSON で
  単語がなければ、インデックスのすべてのパッケージ。"#,
        ),
        (
            r#"  --update        download the newest copy of the index first
  --format json   the entry of the package as JSON"#,
            r#"  --update        先にインデックスの最新のコピーをダウンロードする
  --format json   パッケージの項目を JSON で"#,
        ),
        (
            r#"  NAME@RANGE      a range of versions, like mathlib@1.2; without a range,
                  the newest version
  rythme.toml gets the range, rythme.lock the exact commit and its checksum."#,
            r#"  NAME@RANGE      バージョンの範囲（例：mathlib@1.2）。範囲がなければ
                  最新のバージョン
  rythme.toml には範囲が、rythme.lock には正確なコミットとチェックサムが入る。"#,
        ),
        (
            r#"  Without names, every dependency. rythme.toml does not change;
  rythme.lock gets the new versions."#,
            r#"  名前がなければすべての依存関係。rythme.toml は変わらず、
  rythme.lock に新しいバージョンが入る。"#,
        ),
        (
            r#"  Checks rythme.toml, the tag of the version (like v1.2.0), rythme check
  and rythme test, then prints the entry and the steps of a pull request.
  --verify FILE...  the index's own check of its entries (packages/NAME.toml)"#,
            r#"  rythme.toml、バージョンのタグ（v1.2.0 など）、rythme check、rythme test を
  チェックし、項目とプルリクエストの手順を表示する。
  --verify FILE...  インデックス自身による項目のチェック（packages/NAME.toml）"#,
        ),
        ("the argument `{shown}` is not valid UTF-8: Rythme reads only names and words written in UTF-8; rename the file, or give the program its input another way", "引数 `{shown}` は正しい UTF-8 ではありません：Rythme は UTF-8 で書かれた名前と言葉しか読めません。ファイル名を変えるか、別の方法で入力を渡してください"),
    ],
}
