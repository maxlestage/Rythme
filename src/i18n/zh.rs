// Chinese: each English text of Rythme with its translation. How to add or
// update a language: CONTRIBUTING.md, "Messages in every language".
Language {
    code: "zh",
    name: "简体中文",
    texts: &[
        // src/main.rs
        ("unknown language `{v}`", "未知语言 `{v}`"),
        (
            r#"Rythme ran into an internal problem. This is a bug in Rythme, not in your program.
Please report it, with your program, at https://github.com/maxlestage/Rythme/issues"#,
            r#"Rythme 遇到了一个内部问题。这是 Rythme 的缺陷，不是你的程序的问题。
请把问题连同你的程序一起报告到 https://github.com/maxlestage/Rythme/issues"#,
        ),
        ("`--lang` needs a language", "`--lang` 后面需要一种语言"),
        ("{what}: write `--lang` and one of {codes}, like `rythme --lang fr hello.ry`", "{what}：写 `--lang` 加上 {codes} 中的一个，例如 `rythme --lang zh hello.ry`"),
        ("usage: rythme explain CODE   (codes: {})", "用法：rythme explain 代码   （代码：{}）"),
        ("unknown code `{code}`. Known codes: {}", "未知代码 `{code}`。已知代码：{}"),
        ("which file? Example: rythme run hello.ry", "哪个文件？例如：rythme run hello.ry"),
        ("which file? Example: rythme check hello.ry", "哪个文件？例如：rythme check hello.ry"),
        ("which code? Example: rythme -e 'print(1 + 2)'", "哪段代码？例如：rythme -e 'print(1 + 2)'"),
        ("unknown command or file `{cmd}`", "未知命令或文件 `{cmd}`"),
        (
            r#"
did you mean `rythme {best}`?"#,
            r#"
你是想写 `rythme {best}` 吗？"#,
        ),
        (
            r#"
see `rythme help`"#,
            r#"
请参阅 `rythme help`"#,
        ),
        ("current folder: {}", "当前文件夹：{}"),
        ("Rythme files usually end with `.ry` ({})", "Rythme 文件通常以 `.ry` 结尾（{}）"),
        ("✓ {shown}: no problems found", "✓ {shown}：没有发现问题"),
        ("✓ {shown}: no errors, {n} warning(s)", "✓ {shown}：没有错误，{n} 个警告"),
        ("✗ {shown}: {n} error(s)", "✗ {shown}：{n} 个错误"),
        // src/bigint.rs
        ("this whole number would be too large (more than {MAX_DIGITS} digits)", "这个整数会太大（超过 {MAX_DIGITS} 位）"),
        ("does it grow without end, in a function that calls itself? For an approximate value, use a decimal number (Float), like `2.0 ** 1000`", "它是不是在一个调用自身的函数里无限增长？如果只需要近似值，请使用小数（Float），例如 `2.0 ** 1000`"),
        // src/builtins/date.rs
        ("the year must go from 1 to 9999, not {year}", "年份必须在 1 到 9999 之间，不能是 {year}"),
        ("the month must go from 1 to 12, not {month}", "月份必须在 1 到 12 之间，不能是 {month}"),
        ("the day must go from 1 to {len} in {name} {year}, not {day}", "{year}年{name}的日期必须在 1 到 {len} 之间，不能是 {day}"),
        (" ({year} is not a leap year)", " （{year} 年不是闰年）"),
        ("the hour must go from 0 to 23, not {hour}", "小时必须在 0 到 23 之间，不能是 {hour}"),
        ("the minute must go from 0 to 59, not {minute}", "分钟必须在 0 到 59 之间，不能是 {minute}"),
        ("the second must go from 0 to 59, not {second}", "秒必须在 0 到 59 之间，不能是 {second}"),
        ("the offset must go from -1080 to 1080 minutes (-18:00 to +18:00), not {offset}", "时差必须在 -1080 到 1080 分钟之间（-18:00 到 +18:00），不能是 {offset}"),
        ("a date has no field `{k}`", "日期没有字段 `{k}`"),
        (" (did you mean `{best}`?)", " （你是想写 `{best}` 吗？）"),
        ("the field `{k}` is too large for a date", "字段 `{k}` 对日期来说太大了"),
        ("the field `{k}` must be a whole number, not {}", "字段 `{k}` 必须是整数，不能是 {}"),
        ("the field `{k}` is missing", "缺少字段 `{k}`"),
        ("\"{text}\" is not a date written as ISO 8601: write it like \"2024-02-29\" or \"2024-02-29T13:05:00\"", "\"{text}\" 不是按 ISO 8601 书写的日期：请写成 \"2024-02-29\" 或 \"2024-02-29T13:05:00\" 这样的形式"),
        ("\"{text}\" is not a valid date: {why}", "\"{text}\" 不是有效的日期：{why}"),
        ("`{f}` needs a valid date: {why}", "`{f}` 需要一个有效的日期：{why}"),
        ("not a valid date", "不是有效的日期"),
        ("a date is a record like `{{year: 2024, month: 2, day: 29}}`, with `hour`, `minute`, `second` and `offset` (minutes ahead of UTC) if needed", "日期是一个记录，例如 `{{year: 2024, month: 2, day: 29}}`，需要时还可以有 `hour`、`minute`、`second` 和 `offset`（比 UTC 快的分钟数）"),
        ("`{f}` gives a date outside of the years 1 to 9999", "`{f}` 得到的日期超出了 1 到 9999 年的范围"),
        ("the month must go from 1 to 12, not {m}", "月份必须在 1 到 12 之间，不能是 {m}"),
        ("the clock of the system gives a date outside of the years 1 to 9999", "系统时钟给出的日期超出了 1 到 9999 年的范围"),
        // src/builtins/dicts.rs
        ("`{f}`: {} cannot be a key of a dict, nor an element of a set", "`{f}`：{} 不能作为字典的键，也不能作为集合的元素"),
        ("it holds {}", "它包含 {}"),
        ("keys and elements must be values that can be compared: anything but functions and `nan`", "键和元素必须是可以比较的值：除了函数和 `nan` 之外都可以"),
        ("`{f}` needs a list of [key, value] pairs, but it contains {}", "`{f}` 需要一个由 [键, 值] 对组成的列表，但其中包含 {}"),
        ("each pair is a list of two elements, like `dict([[\"Ada\", 36], [\"Alan\", 41]])`", "每一对都是包含两个元素的列表，例如 `dict([[\"Ada\", 36], [\"Alan\", 41]])`"),
        // src/builtins/property.rs
        ("this is not a generator: {}", "这不是生成器：{}"),
        ("not a generator", "不是生成器"),
        ("make one with `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` or `records_of({{x: g}})`", "用 `ints()`、`floats()`、`texts()`、`bools()`、`lists_of(g)`、`one_of(list)` 或 `records_of({{x: g}})` 创建一个"),
        ("generators can be put inside each other {MAX_DEPTH} times at most", "生成器最多只能互相嵌套 {MAX_DEPTH} 层"),
        ("its field `{what}` is missing or wrong", "它的字段 `{what}` 缺失或有误"),
        ("`{f}` takes no argument, or two: the smallest and the largest value, like `{f}(0, 10)`", "`{f}` 不接受参数，或者接受两个参数：最小值和最大值，例如 `{f}(0, 10)`"),
        ("{f}({}, {}): the minimum is larger than the maximum", "{f}({}, {})：最小值大于最大值"),
        ("`floats` needs finite numbers, not {} and {}", "`floats` 需要有限的数，不能是 {} 和 {}"),
        ("`{f}` needs a length from 0 to {MAX_LENGTH}, not {n}", "`{f}` 需要 0 到 {MAX_LENGTH} 之间的长度，不能是 {n}"),
        ("`one_of` needs at least one value to choose from", "`one_of` 至少需要一个可供选择的值"),
        ("the field `{k}` given to `records_of` must be a generator, like `ints()`, not {}", "传给 `records_of` 的字段 `{k}` 必须是生成器，例如 `ints()`，不能是 {}"),
        ("`check_all` tries from 1 to {MAX_CASES} values, not {cases}", "`check_all` 尝试的值的个数必须在 1 到 {MAX_CASES} 之间，不能是 {cases}"),
        ("the property is false for {v}", "对于 {v}，该性质不成立"),
        ("false for {v}", "对 {v} 不成立"),
        ("the property gives {r} for {v}", "对于 {v}，该性质给出 {r}"),
        ("{r} for {v}", "对 {v} 给出 {r}"),
        ("the property fails for {v}: {}", "对于 {v}，该性质失败：{}"),
        ("checked here", "在这里检查"),
        ("found on try {case} of {cases}", "在第 {case} 次尝试（共 {cases} 次）时发现"),
        ("found on try {case} of {cases}, then simplified from {} in {steps} step(s)", "在第 {case} 次尝试（共 {cases} 次）时发现，然后从 {} 经过 {steps} 步简化而来"),
        ("seed {seed}: every run tries the same values; `rythme test --seed N` tries others", "种子 {seed}：每次运行都尝试相同的值；`rythme test --seed N` 会尝试其他值"),
        ("the function must hold for every value of the generator: fix the function, or give the generator a smaller range, like `ints(1, 100)`", "函数必须对生成器的每个值都成立：修正函数，或者给生成器一个更小的范围，例如 `ints(1, 100)`"),
        // src/builtins/regex.rs
        ("for the sign `{c}` itself, write `[{c}]` (in a Rythme text: \"[\\{c}]\")", "要表示符号 `{c}` 本身，请写 `[{c}]`（在 Rythme 文本中写 \"[\\{c}]\"）"),
        ("for the sign `{c}` itself, write `[{c}]`", "要表示符号 `{c}` 本身，请写 `[{c}]`"),
        ("`{q}` has nothing to repeat", "`{q}` 没有可重复的内容"),
        ("nothing before it to repeat", "前面没有可重复的内容"),
        ("put it after a character or a group, like `a{q}` or `(ab){q}`; {}", "把它放在一个字符或一个分组后面，例如 `a{q}` 或 `(ab){q}`；{}"),
        ("`{{` must start a count, like {{3}} or {{2,5}}", "`{{` 必须开始一个次数，例如 {{3}} 或 {{2,5}}"),
        ("not a count", "不是次数"),
        ("{}; in a Rythme text, braces are written `\\{{` and `\\}}`, like \"[0-9]\\{{4\\}}\"", "{}；在 Rythme 文本中，花括号要写成 `\\{{` 和 `\\}}`，例如 \"[0-9]\\{{4\\}}\""),
        ("a count can be at most {MAX_COUNT}", "次数最多为 {MAX_COUNT}"),
        ("too large", "太大"),
        ("for more, use `*` or `+`, or {{n,}} without a maximum", "如果要更多，请使用 `*` 或 `+`，或者不带最大值的 {{n,}}"),
        ("the count goes backwards: {min} is more than {max}", "次数顺序反了：{min} 大于 {max}"),
        ("smallest first", "小的在前"),
        ("write the smallest number first: {{{max},{min}}}", "先写较小的数：{{{max},{min}}}"),
        ("a repetition cannot be repeated again", "重复不能再次被重复"),
        ("repeats a repetition", "重复了一个重复"),
        ("put the repeated part in parentheses, like `(a+)*`, or keep one sign, like `a*`; {}", "把要重复的部分放在括号里，例如 `(a+)*`，或者只保留一个符号，例如 `a*`；{}"),
        ("the groups are nested too deeply (more than {MAX_DEPTH})", "分组嵌套太深（超过 {MAX_DEPTH} 层）"),
        ("too deep", "太深"),
        ("`(?<` must be followed by a name and `>`", "`(?<` 后面必须跟一个名字和 `>`"),
        ("not a group name", "不是分组名"),
        ("a named group looks like `(?<year>[0-9]+)`: the name has letters, digits and `_`", "命名分组的样子是 `(?<year>[0-9]+)`：名字由字母、数字和 `_` 组成"),
        ("two groups are named `{name}`", "有两个分组都叫 `{name}`"),
        ("already used", "已被使用"),
        ("give each group its own name", "给每个分组一个不同的名字"),
        ("`(?i)` must be at the very start of the regex", "`(?i)` 必须位于正则表达式的最开头"),
        ("not at the start", "不在开头"),
        ("`(?i)` ignores the case in the whole regex: write it first", "`(?i)` 让整个正则表达式忽略大小写：请把它写在最前面"),
        ("unknown kind of group", "未知的分组类型"),
        ("unknown", "未知"),
        ("the groups are `(...)`, `(?:...)` (a group that captures nothing) and `(?<name>...)` (a named group); `(?i)` at the start ignores the case", "分组有 `(...)`、`(?:...)`（不捕获任何内容的分组）和 `(?<name>...)`（命名分组）；开头的 `(?i)` 表示忽略大小写"),
        ("this `(` is never closed", "这个 `(` 一直没有闭合"),
        ("opened here", "在这里打开"),
        ("add a `)` where the group ends; {}", "在分组结束的地方加上 `)`；{}"),
        ("the regex ends with a lone `\\`", "正则表达式以单独的 `\\` 结尾"),
        ("followed by nothing", "后面什么都没有"),
        ("a `\\` comes before a letter like d, w, s or b, or before a sign for the sign itself, like `\\.`", "`\\` 应放在 d、w、s、b 这样的字母前面，或者放在符号前面表示符号本身，例如 `\\.`"),
        ("`\\{c}` means nothing inside `[...]`", "`\\{c}` 在 `[...]` 中没有意义"),
        ("not a character", "不是字符"),
        ("`\\b` is the edge of a word: write it outside of the brackets", "`\\b` 表示单词的边界：请把它写在方括号外面"),
        ("`\\{c}` means nothing in a regex", "`\\{c}` 在正则表达式中没有意义"),
        (
            r#"the known ones: \d (a digit), \w (a letter, a digit or _), \s (a space), \b (the edge of a word),
their opposites \D, \W, \S, \B, \n, \t, and `\` before a sign for the sign itself, like \. or \("#,
            r#"已知的有：\d（一个数字）、\w（一个字母、数字或 _）、\s（一个空白）、\b（单词边界），
它们的反面 \D、\W、\S、\B，以及 \n、\t，还有放在符号前面表示符号本身的 `\`，例如 \. 或 \("#,
        ),
        ("this `[` is never closed", "这个 `[` 一直没有闭合"),
        ("add a `]` after the characters; for the sign `]` itself, put it first: `[]]`", "在字符后面加上 `]`；如果要表示符号 `]` 本身，把它放在最前面：`[]]`"),
        ("a range must end with one character", "范围必须以单个字符结束"),
        ("not a range", "不是范围"),
        ("for the sign `-` itself, put it last: `[a-]`", "如果要表示符号 `-` 本身，把它放在最后：`[a-]`"),
        ("the range `{lo}-{hi}` goes backwards", "范围 `{lo}-{hi}` 的顺序反了"),
        ("backwards", "顺序反了"),
        ("write the first character first: `{hi}-{lo}`", "先写前面的字符：`{hi}-{lo}`"),
        ("this `)` closes nothing", "这个 `)` 没有闭合任何东西"),
        ("no `(` before it", "前面没有 `(`"),
        ("the regex is too large once its repetitions are written out", "正则表达式展开重复之后太大了"),
        ("use smaller counts in {{n,m}}, or `*` and `+`, which cost nothing", "在 {{n,m}} 中使用更小的次数，或者使用不占空间的 `*` 和 `+`"),
        ("`$1` is the text of group 1, `$<name>` of a named group, `$0` the whole match; for the sign `$` itself, write `$$`", "`$1` 是第 1 个分组的文本，`$<name>` 是命名分组的文本，`$0` 是整个匹配；要表示符号 `$` 本身，请写 `$$`"),
        ("the regex has no group", "这个正则表达式没有分组"),
        ("the regex has 1 group", "这个正则表达式有 1 个分组"),
        ("the regex has {g} groups", "这个正则表达式有 {g} 个分组"),
        ("there is no group {n}: {has}", "没有第 {n} 个分组：{has}"),
        ("no such group", "没有这个分组"),
        ("`$<` must be followed by a group name and `>`", "`$<` 后面必须跟一个分组名和 `>`"),
        ("never closed", "一直没有闭合"),
        ("the regex has no named group", "这个正则表达式没有命名分组"),
        ("its named groups are {}", "它的命名分组有 {}"),
        ("the regex has no group named `{name}`: {has}", "这个正则表达式没有名为 `{name}` 的分组：{has}"),
        ("unknown group", "未知分组"),
        ("`$` must be followed by a group number, `<name>` or `$`", "`$` 后面必须跟分组编号、`<name>` 或 `$`"),
        ("a lone `$`", "单独的 `$`"),
        ("invalid replacement: {}", "无效的替换文本：{}"),
        ("invalid regex: {}", "无效的正则表达式：{}"),
        ("with this replacement", "在这个替换文本中"),
        ("with this regex", "在这个正则表达式中"),
        ("here in the replacement:", "替换文本中的这里："),
        ("here in the regex:", "正则表达式中的这里："),
        ("`named_captures` needs a regex with named groups", "`named_captures` 需要一个带命名分组的正则表达式"),
        ("name a group like this: `(?<year>[0-9]+)`; for groups without names, use `captures`", "像这样给分组命名：`(?<year>[0-9]+)`；对于没有名字的分组，请使用 `captures`"),
        // src/builtins/system.rs
        ("`{f}` is not available in the browser: a program here has no input to read. Run it on your computer: rythme file.ry < lines.txt", "`{f}` 在浏览器中不可用：这里的程序没有可读取的输入。请在你的电脑上运行：rythme file.ry < lines.txt"),
        ("`{f}` is not available in the browser: a program here has no files or folders. Run it on your computer: rythme file.ry", "`{f}` 在浏览器中不可用：这里的程序没有文件和文件夹。请在你的电脑上运行：rythme file.ry"),
        ("the path is empty: write \".\" for the current folder", "路径为空：当前文件夹请写 \".\""),
        ("folder not found: {path}", "找不到文件夹：{path}"),
        ("file not found: {path}", "找不到文件：{path}"),
        ("permission denied: {path}", "没有权限：{path}"),
        ("cannot access {path}: {e}", "无法访问 {path}：{e}"),
        ("{path} is a file, not a folder: `read_file` reads a file", "{path} 是文件，不是文件夹：`read_file` 用来读取文件"),
        ("{path} already exists, and it is a file, not a folder", "{path} 已经存在，而且它是文件，不是文件夹"),
        ("{path} is a folder: `delete_file` deletes only files, never folders", "{path} 是文件夹：`delete_file` 只删除文件，从不删除文件夹"),
        ("cannot read the standard input: {e}", "无法读取标准输入：{e}"),
        ("`join_path` needs a list of texts, but it contains {}", "`join_path` 需要一个文本列表，但其中包含 {}"),
        ("`text(value)` turns a value into a text", "`text(value)` 把一个值变成文本"),
        ("`join_path` cannot add \"{part}\" after \"{out}\": only the first part of a path can start at a root (`/`, `\\` or a drive like `C:`)", "`join_path` 不能把 \"{part}\" 加在 \"{out}\" 后面：只有路径的第一部分可以从根开始（`/`、`\\` 或像 `C:` 这样的驱动器）"),
        ("remove the separator at the start of \"{part}\", or put it first in the list", "去掉 \"{part}\" 开头的分隔符，或者把它放在列表的最前面"),
        ("`{w}` is not a valid option: its name starts with a letter, followed by letters, digits, `-` or `_` (write `--` before words that start with `-`)", "`{w}` 不是有效的选项：选项名以字母开头，后面是字母、数字、`-` 或 `_`（以 `-` 开头的单词前面要写 `--`）"),
        ("`{w}` is not an option that Rythme understands: write `--{name}` for a long name, or `-a -b` for several one-letter options", "`{w}` 不是 Rythme 能理解的选项：长名字写成 `--{name}`，多个单字母选项写成 `-a -b`"),
        ("`{w}` cannot be an option: the field `args` holds the words that are not options", "`{w}` 不能作为选项：字段 `args` 存放的是不是选项的单词"),
        ("the option `{name}` is given twice", "选项 `{name}` 给了两次"),
        ("`options` needs a list of texts, like the one of `args()`, but it contains {}", "`options` 需要一个文本列表，比如 `args()` 给出的那种，但其中包含 {}"),
        // src/builtins/text_tools.rs
        ("`{f}` writes at most 100 decimals, not {d}", "`{f}` 最多写 100 位小数，不能是 {d}"),
        ("`{f}` does not know the language \"{l}\"", "`{f}` 不认识语言 \"{l}\""),
        ("the languages: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" and \"pt\" (1.234,5)", "可用的语言：\"en\"（1,234.5）、\"fr\"（1 234,5）、\"de\"、\"es\"、\"it\"、\"nl\" 和 \"pt\"（1.234,5）"),
        // src/builtins.rs
        ("Basics", "基础"),
        ("Numbers", "数字"),
        ("Text", "文本"),
        ("Lists", "列表"),
        ("Records", "记录"),
        ("Dictionaries and sets", "字典和集合"),
        ("Option and Result", "Option 和 Result"),
        ("Files", "文件"),
        ("Dates and times", "日期和时间"),
        ("Property tests", "属性测试"),
        ("usage: `{}`", "用法：`{}`"),
        ("a list", "列表"),
        ("a text", "文本"),
        ("a whole number (Int)", "整数（Int）"),
        ("a number", "数字"),
        ("a function", "函数"),
        ("a record", "记录"),
        ("true or false", "true 或 false"),
        ("a list or a text", "列表或文本"),
        ("a text or a list", "文本或列表"),
        ("a text, a list, a record, a dict or a set", "文本、列表、记录、字典或集合"),
        ("a list, a text, a record or a dict", "列表、文本、记录或字典"),
        ("a record or a dict", "记录或字典"),
        ("a record, a dict or a set", "记录、字典或集合"),
        ("a list, a dict or a set", "列表、字典或集合"),
        ("a list or a set", "列表或集合"),
        ("a dict or a set", "字典或集合"),
        ("a dict", "字典"),
        ("a set", "集合"),
        ("a list of [key, value] pairs (or a record)", "由 [键, 值] 对组成的列表（或记录）"),
        ("a list (or several values)", "列表（或多个值）"),
        ("a number or a text", "数字或文本"),
        ("a list, a set, an Option or a Result", "列表、集合、Option 或 Result"),
        ("an Option or a Result", "Option 或 Result"),
        ("an Option", "Option"),
        ("a Result", "Result"),
        ("a date (a record {{year, month, day}})", "日期（记录 {{year, month, day}}）"),
        ("a generator (like `ints()`)", "生成器（例如 `ints()`）"),
        ("a text or a function", "文本或函数"),
        ("`{f}` expects {expected} as argument {pos}, but got {got}", "`{f}` 的第 {pos} 个参数应该是{expected}，但得到的是 {got}"),
        ("wrong kind of value", "值的类型不对"),
        ("a text is not a list: `chars(text)` gives the list of its characters", "文本不是列表：`chars(text)` 会给出它的字符列表"),
        ("a record is not a list: `entries(record)`, `keys(record)` or `values(record)` give one", "记录不是列表：`entries(record)`、`keys(record)` 或 `values(record)` 会给出一个列表"),
        ("a dict is not a list: `entries(dict)`, `keys(dict)` or `values(dict)` give one", "字典不是列表：`entries(dict)`、`keys(dict)` 或 `values(dict)` 会给出一个列表"),
        ("a set is not a list: `values(set)` gives the list of its elements, in order", "集合不是列表：`values(set)` 会按顺序给出它的元素列表"),
        ("to change the values of a dict, use `map_values(dict, f)`", "要改变字典的值，请使用 `map_values(dict, f)`"),
        ("for a list, use `{cousin}`", "对于列表，请使用 `{cousin}`"),
        ("to read a date from a text like \"2024-02-29\", use `parse_date(text)`", "要从 \"2024-02-29\" 这样的文本中读取日期，请使用 `parse_date(text)`"),
        ("for a list, use `find(list, f)`", "对于列表，请使用 `find(list, f)`"),
        ("for an Option or a Result, use `and_then(x, f)`", "对于 Option 或 Result，请使用 `and_then(x, f)`"),
        ("the function given to `{f}` must return a list, but it returned {got}", "传给 `{f}` 的函数必须返回列表，但它返回了 {got}"),
        ("the function given to `{f}` must return a text, but it returned {got}", "传给 `{f}` 的函数必须返回文本，但它返回了 {got}"),
        ("to make a text of any value, use `text(x)`", "要把任意值变成文本，请使用 `text(x)`"),
        ("an Option (Some or None)", "Option（Some 或 None）"),
        ("a Result (Ok or Err)", "Result（Ok 或 Err）"),
        ("the function given to `{f}` must return {expected}, but it returned {got}", "传给 `{f}` 的函数必须返回{expected}，但它返回了 {got}"),
        ("to change the value inside without another check, use `map(x, f)`", "如果只想改变里面的值而不再检查，请使用 `map(x, f)`"),
        ("the function given to `{f}` must return true or false, but it returned {got}", "传给 `{f}` 的函数必须返回 true 或 false，但它返回了 {got}"),
        ("the number {n} is too large for `{f}`", "数字 {n} 对 `{f}` 来说太大了"),
        ("{digits} digits", "{digits} 位"),
        ("`{f}` needs a number of 0 or more, not {n}", "`{f}` 需要大于或等于 0 的数，不能是 {n}"),
        ("`{f}` needs a number of 1 or more, not 0", "`{f}` 需要大于或等于 1 的数，不能是 0"),
        ("the list is empty", "列表是空的"),
        ("the list has {len} element(s)", "列表有 {len} 个元素"),
        ("position {n} is out of range: {what}", "位置 {n} 超出范围：{what}"),
        ("no element here", "这里没有元素"),
        ("valid positions go from 0 to {last} (or from -1 backwards, counting from the end)", "有效位置是 0 到 {last}（或者从 -1 开始倒着数，从末尾算起）"),
        ("at least {min}", "至少 {min} 个"),
        ("{min} to {max}", "{min} 到 {max} 个"),
        ("`{f}` gives {given} argument(s) to this function, but it takes {expected}", "`{f}` 给这个函数传了 {given} 个参数，但它需要 {expected} 个"),
        ("takes {expected} argument(s)", "需要 {expected} 个参数"),
        ("`nan` (not a number) cannot be converted to a whole number", "`nan`（不是数字）不能转换成整数"),
        ("`nan` is the result of a calculation that has none, like `0.0 * inf`: check where this value comes from", "`nan` 是没有结果的计算的结果，例如 `0.0 * inf`：请检查这个值从哪里来"),
        ("`{}` (infinity) cannot be converted to a whole number", "`{}`（无穷大）不能转换成整数"),
        ("an infinity comes from a calculation too large for decimal numbers, like `1e308 * 10.0`, or from a division by `0.0`", "无穷大来自对小数来说太大的计算，例如 `1e308 * 10.0`，或者来自除以 `0.0`"),
        ("`text` writes at most 100 decimals, not {digits}", "`text` 最多写 100 位小数，不能是 {digits}"),
        ("`input` found the end of the input", "`input` 读到了输入的末尾"),
        ("no more lines to read", "没有更多可读的行了"),
        ("the keyboard input was closed (Ctrl-D), or the text sent to the program has ended", "键盘输入已关闭（Ctrl-D），或者发送给程序的文本已经结束"),
        ("assertion failed: {}", "断言失败：{}"),
        ("assertion failed", "断言失败"),
        ("this condition is false", "这个条件为假"),
        (
            r#"got:      {got}
expected: {expected}"#,
            r#"实际：{got}
期望：{expected}"#,
        ),
        ("first difference: character {at}", "第一个不同之处：第 {at} 个字符"),
        ("assertion failed: the values are different", "断言失败：两个值不同"),
        (" at {path}", " （位于 {path}）"),
        ("the list{at} has {} element(s), {} expected", "列表{at}有 {} 个元素，期望 {} 个"),
        ("in excess", "多出的"),
        ("missing", "缺少的"),
        ("{word} from [{from}]: {}{dots}", "从 [{from}] 开始{word}：{}{dots}"),
        ("missing field(s){at}: {}", "缺少的字段{at}：{}"),
        ("field(s) not expected{at}: {}", "不应有的字段{at}：{}"),
        (", which are of different types: {} and {}", "，它们的类型不同：{} 和 {}"),
        ("first difference at {path}: got {}, expected {}{kinds}", "第一个不同之处在 {path}：实际为 {}，期望为 {}{kinds}"),
        ("the values are of different types: {} and {}", "两个值的类型不同：{} 和 {}"),
        ("(no line)", "（没有这一行）"),
        (
            r#"first different line: line {}
  got:      {}
  expected: {}"#,
            r#"第一个不同的行：第 {} 行
  实际：{}
  期望：{}"#,
        ),
        ("the program stopped with `fail`", "程序因 `fail` 而停止"),
        ("the program stopped here", "程序在这里停止"),
        ("an exit code goes from 0 to 255, not {code}", "退出码必须在 0 到 255 之间，不能是 {code}"),
        ("the program stopped with exit({code})", "程序以 exit({code}) 停止"),
        ("cannot pick a random element in an empty list", "不能从空列表中随机选取元素"),
        ("random({lo}, {hi}): the minimum is larger than the maximum", "random({lo}, {hi})：最小值大于最大值"),
        ("`{name}` of an empty list", "空列表的 `{name}`"),
        ("`{name}` cannot compare {} and {}", "`{name}` 无法比较 {} 和 {}"),
        ("cannot take the square root of a negative number ({x})", "不能对负数（{x}）开平方根"),
        ("use `sqrt(abs(x))` if you want the root of its absolute value", "如果你想要它的绝对值的平方根，请使用 `sqrt(abs(x))`"),
        ("cannot convert \"{s}\" to a whole number", "无法把 \"{s}\" 转换成整数"),
        ("if the text may not be a number, use `parse_int(t)`: it gives `Ok(n)` or `Err(...)` that you can check", "如果文本可能不是数字，请使用 `parse_int(t)`：它给出 `Ok(n)` 或 `Err(...)`，你可以检查结果"),
        ("cannot convert \"{s}\" to a decimal number", "无法把 \"{s}\" 转换成小数"),
        ("if the text may not be a number, use `parse_float(t)`, which gives `Ok(x)` or `Err(...)`", "如果文本可能不是数字，请使用 `parse_float(t)`，它给出 `Ok(x)` 或 `Err(...)`"),
        ("`{name}` needs a number from -1 to 1, not {x}", "`{name}` 需要 -1 到 1 之间的数，不能是 {x}"),
        ("the logarithm only exists for positive numbers, not {x}", "对数只对正数有定义，不能是 {x}"),
        ("invalid logarithm base: {base}", "无效的对数底数：{base}"),
        ("`{f}` needs a single character, but \"{s}\" has {}", "`{f}` 需要单个字符，但 \"{s}\" 有 {} 个"),
        ("`replace` cannot replace an empty text", "`replace` 不能替换空文本"),
        ("{n} is not the number of a character", "{n} 不是字符编号"),
        ("character numbers go from 0 to 1114111, except 55296 to 57343", "字符编号的范围是 0 到 1114111，但不包括 55296 到 57343"),
        ("the repeated text would be too large", "重复后的文本会太大"),
        ("the repeated list would be too large", "重复后的列表会太大"),
        ("this number has more than {} digits", "这个数超过了 {} 位"),
        ("\"{s}\" is not a whole number", "\"{s}\" 不是整数"),
        ("\"{s}\" is not a number", "\"{s}\" 不是数字"),
        ("JSON, line {line}, column {column}: {msg}", "JSON，第 {line} 行，第 {column} 列：{msg}"),
        ("expected {what}, but found `{c}`", "期望 {what}，但遇到了 `{c}`"),
        ("expected {what}, but the text ends", "期望 {what}，但文本结束了"),
        ("a value", "一个值"),
        ("the end of the text", "文本的结尾"),
        ("`,` or `]`", "`,` 或 `]`"),
        ("`,` or `}}`", "`,` 或 `}}`"),
        ("a name in quotes", "带引号的名字"),
        ("invalid number: JSON writes numbers like 0, -12, 3.5 or 1e-3", "无效的数字：JSON 中的数字写作 0、-12、3.5 或 1e-3"),
        ("invalid escape: after `\\` comes one of \" \\ / b f n r t u", "无效的转义：`\\` 后面只能是 \" \\ / b f n r t u 之一"),
        ("a line break or a tab inside a text is written with `\\`: `\\n`, `\\t`", "文本中的换行符或制表符要用 `\\` 来写：`\\n`、`\\t`"),
        ("a text is never closed: `\"` is missing", "文本一直没有闭合：缺少 `\"`"),
        ("`\\u` must be followed by 4 hexadecimal digits", "`\\u` 后面必须跟 4 个十六进制数字"),
        ("invalid `\\u` escape: half of a character (a surrogate) without its other half", "无效的 `\\u` 转义：只有半个字符（代理项），缺少另一半"),
        ("`to_json` indents by 16 spaces at most, not {n}", "`to_json` 最多缩进 16 个空格，不能是 {n}"),
        ("the JSON text would be too large", "JSON 文本会太大"),
        ("the values are nested too deeply to be indented: write them on one line, with `to_json(value)`", "这些值嵌套太深，无法缩进：请用 `to_json(value)` 把它们写在一行里"),
        ("`to_json` cannot write {} in JSON", "`to_json` 无法把 {} 写成 JSON"),
        ("JSON has no infinite numbers and no NaN", "JSON 中没有无穷大，也没有 NaN"),
        (
            r#"a dict is written as a JSON object only when its keys are all texts, or all symbols:
turn the keys into texts first, or write `entries(dict)`"#,
            r#"只有当字典的键全是文本或全是符号时，它才会写成 JSON 对象：
请先把键变成文本，或者写 `entries(dict)`"#,
        ),
        (
            r#"JSON only holds numbers, texts, true and false, lists, records and null (`None`):
turn your value into a record first, like {{kind: "Circle", radius: r}}"#,
            r#"JSON 只能包含数字、文本、true 和 false、列表、记录以及 null（`None`）：
请先把你的值变成记录，例如 {{kind: "Circle", radius: r}}"#,
        ),
        ("`sum` needs a list of numbers, but it contains {}", "`sum` 需要一个数字列表，但其中包含 {}"),
        ("`product` needs a list of numbers, but it contains {}", "`product` 需要一个数字列表，但其中包含 {}"),
        ("all the elements must be of the same kind: all numbers, or all texts...", "所有元素必须是同一种类型：全是数字，或者全是文本……"),
        ("the result of `windows` would be too large", "`windows` 的结果会太大"),
        ("the function `key` must give values of the same kind for every element: all numbers, or all texts...", "函数 `key` 必须为每个元素给出同一种类型的值：全是数字，或者全是文本……"),
        ("`flatten` needs a list of lists, but it contains {}", "`flatten` 需要一个由列表组成的列表，但其中包含 {}"),
        ("`from_entries` needs a list of [name, value] pairs, but it contains {}", "`from_entries` 需要一个由 [名字, 值] 对组成的列表，但其中包含 {}"),
        ("each pair is a list of two elements whose name is a text, like [\"age\", 36]", "每一对都是包含两个元素的列表，其中名字是文本，例如 [\"age\", 36]"),
        ("`unwrap` found `None`: there is no value", "`unwrap` 遇到了 `None`：没有值"),
        ("handle the missing value with `match`, or give a default: `unwrap_or(x, default)`", "用 `match` 处理缺失的值，或者给一个默认值：`unwrap_or(x, default)`"),
        ("`unwrap` found an error: {}", "`unwrap` 遇到了一个错误：{}"),
        ("handle the error with `match`, pass it on with `?`, or give a default: `unwrap_or(x, default)`", "用 `match` 处理这个错误，用 `?` 把它传递出去，或者给一个默认值：`unwrap_or(x, default)`"),
        // src/check/needs.rs
        ("`{name}` stops with this error, every time", "`{name}` 每次都会因这个错误而停止"),
        ("here", "这里"),
        ("in `{name}`: {label}", "在 `{name}` 中：{label}"),
        ("`{name}` always fails with {}", "`{name}` 总是以 {} 失败"),
        ("fails every time", "每次都失败"),
        ("defined here", "在这里定义"),
        ("from how `{name}` uses `{p}`, it needs: `{sig}`", "从 `{name}` 使用 `{p}` 的方式来看，它需要：`{sig}`"),
        // src/check.rs
        ("the type `{name}` is part of Rythme", "类型 `{name}` 是 Rythme 自带的"),
        ("give your type another name", "给你的类型换个名字"),
        ("the type `{}` is already defined", "类型 `{}` 已经定义过了"),
        ("first defined here", "第一次在这里定义"),
        ("give it another name", "给它换个名字"),
        ("the field `{f}` appears twice in `{}`", "字段 `{f}` 在 `{}` 中出现了两次"),
        ("`{s}` is already the name of the file `{file}`", "`{s}` 已经是文件 `{file}` 的名字了"),
        ("choose another name here, or give the file another one: `use ... as other_name`", "在这里换一个名字，或者给文件起另一个名字：`use ... as other_name`"),
        ("`{s}` is already defined", "`{s}` 已经定义过了"),
        ("defined again here", "在这里再次定义"),
        ("a constructor belongs to a single type: rename one of them", "一个构造器只属于一个类型：请重命名其中一个"),
        ("to give several definitions to a function (one per case), write them one right after the other", "要给一个函数写多个定义（每种情况一个），请把它们一个接一个地写在一起"),
        ("`{s}` hides the built-in function `{s}`", "`{s}` 遮蔽了内置函数 `{s}`"),
        ("`{s}` hides the built-in value `{s}`", "`{s}` 遮蔽了内置值 `{s}`"),
        ("in this file, `{s}` will now mean your definition; choose another name to keep the built-in one", "在这个文件中，`{s}` 现在表示你的定义；如果想保留内置的那个，请换个名字"),
        ("`{s}` is part of Rythme", "`{s}` 是 Rythme 自带的"),
        ("choose another name", "换个名字"),
        ("the result of `{name}` is lost", "`{name}` 的结果丢失了"),
        ("this value is lost", "这个值丢失了"),
        (
            r#"values never change: `{sig}` gives a new value, and changes nothing.
To keep it, give it a name: `new = {sig}`"#,
            r#"值永远不会改变：`{sig}` 给出一个新值，不会改变任何东西。
要保留它，请给它一个名字：`new = {sig}`"#,
        ),
        ("this test can never fail", "这个测试永远不会失败"),
        ("its result, {}, is never compared with what is expected", "它的结果 {} 从未与期望值比较"),
        (
            r#"a test fails only when it stops with an error, or ends with `false`, `None` or `Err(...)`;
nothing in this one can"#,
            r#"测试只有在因错误而停止，或者以 `false`、`None` 或 `Err(...)` 结束时才会失败；
这个测试里没有任何东西能做到这一点"#,
        ),
        ("compare the result with what you expect: `assert_eq(actual, expected)`", "把结果和你期望的值进行比较：`assert_eq(actual, expected)`"),
        ("remove `{name}` from the pattern, or write `{name}: _{name}` if this is on purpose", "把 `{name}` 从模式中去掉，或者如果是故意的，写成 `{name}: _{name}`"),
        ("remove it, or name it `_{name}` if this is on purpose", "去掉它，或者如果是故意的，把它命名为 `_{name}`"),
        ("`{name}` is never used", "`{name}` 从未被使用"),
        ("`{s}` appears twice in this pattern", "`{s}` 在这个模式中出现了两次"),
        ("first here", "第一次在这里"),
        ("use two different names, then compare them with `==` in a condition: `[a, b] if a == b -> ...`", "使用两个不同的名字，然后在条件中用 `==` 比较它们：`[a, b] if a == b -> ...`"),
        ("`{s}` is used before being defined", "`{s}` 在定义之前就被使用了"),
        ("used here", "在这里使用"),
        ("defined later, here", "在后面这里定义"),
        ("move the definition above this line", "把定义移到这一行的上面"),
        ("`{s}` is the name of the file `{file}`, not a value", "`{s}` 是文件 `{file}` 的名字，不是一个值"),
        ("write `{s}.` and the name of one of its definitions, like `{s}.{example}`", "写 `{s}.` 再加上它的某个定义的名字，例如 `{s}.{example}`"),
        ("`{x}` is private to the file `{file}`", "`{x}` 是文件 `{file}` 的私有名字"),
        ("private", "私有"),
        ("a name starting with `_` can only be used in its own file", "以 `_` 开头的名字只能在它自己的文件中使用"),
        ("to use it from other files, rename it without `_` in `{file}`", "如果要在其他文件中使用它，请在 `{file}` 中把它重命名为不带 `_` 的名字"),
        ("the file `{file}` has no `{x}`", "文件 `{file}` 中没有 `{x}`"),
        ("not in `{file}`", "不在 `{file}` 中"),
        ("did you mean `{m}.{best}`?", "你是想写 `{m}.{best}` 吗？"),
        ("`{file}` defines nothing that other files can use", "`{file}` 没有定义任何可供其他文件使用的东西"),
        ("`{file}` gives: {}{more}", "`{file}` 提供：{}{more}"),
        ("`{name}` is private to the file `{file}`", "`{name}` 是文件 `{file}` 的私有名字"),
        ("unknown name `{name}`", "未知名字 `{name}`"),
        ("not defined here", "这里没有定义"),
        ("`{name}` is defined in `{file}`: write `{m}.{name}`", "`{name}` 定义在 `{file}` 中：请写 `{m}.{name}`"),
        ("`{name}` is defined in `{file}`: add `use {stem}` at the top of the file, then write `{stem}.{name}`", "`{name}` 定义在 `{file}` 中：在文件开头加上 `use {stem}`，然后写 `{stem}.{name}`"),
        ("not defined", "未定义"),
        ("defined here, further down", "在下面这里定义"),
        ("in a block, a function can be used only after it, unless they are written one after the other with nothing between them: move `{name}` up, next to the function that uses it", "在代码块中，函数只能在它之后使用，除非它们一个接一个地写在一起、中间没有别的东西：请把 `{name}` 移上去，放在使用它的函数旁边"),
        ("in a block, a name can be used only after the line that defines it: move `{name} = ...` up", "在代码块中，名字只能在定义它的那一行之后使用：请把 `{name} = ...` 移上去"),
        ("did you mean `{best}`?", "你是想写 `{best}` 吗？"),
        ("to create your own kind of value, define a type: `type Name = {name}(field) | Other`", "要创建你自己的值的种类，请定义一个类型：`type Name = {name}(field) | Other`"),
        ("define it before using it: `{name} = ...` or `fn {name}(...) = ...`", "先定义再使用：`{name} = ...` 或 `fn {name}(...) = ...`"),
        ("this definition of `{name}` can never be used", "`{name}` 的这个定义永远不会被用到"),
        ("this definition already accepts every argument", "这个定义已经接受所有参数"),
        ("definitions are tried in order: put the most general one last", "定义是按顺序尝试的：请把最通用的放在最后"),
        ("this definition already accepts these arguments", "这个定义已经接受这些参数"),
        ("definitions are tried in order: remove this one, or change its patterns", "定义是按顺序尝试的：去掉这个定义，或者修改它的模式"),
        ("this function receives {}, which never matches this pattern", "这个函数收到的是 {}，永远不会匹配这个模式"),
        ("never matches", "永远不匹配"),
        ("`{name}` does not handle every possible argument", "`{name}` 没有处理所有可能的参数"),
        ("not handled: {}", "未处理：{}"),
        ("add a definition for it, like `fn {name}({}) = ...`", "为它添加一个定义，例如 `fn {name}({}) = ...`"),
        ("the value {} does not match this pattern", "值 {} 不匹配这个模式"),
        ("a value of type {} never matches this pattern", "类型为 {} 的值永远不会匹配这个模式"),
        ("`_` can only be used in patterns", "`_` 只能在模式中使用"),
        ("`_` means \"anything, ignored\"; give the value a real name to use it", "`_` 的意思是“任意值，忽略掉”；要使用这个值，请给它起一个真正的名字"),
        ("`?` can only be used inside a function", "`?` 只能在函数内部使用"),
        ("outside of a function, handle the value with `match` or `unwrap_or`", "在函数外部，请用 `match` 或 `unwrap_or` 处理这个值"),
        ("`?` needs an Option or a Result, but got {}", "`?` 需要 Option 或 Result，但得到的是 {}"),
        ("`?` takes the value out of `Some(x)` / `Ok(x)`, and returns `None` / `Err(e)` from the current function", "`?` 从 `Some(x)` / `Ok(x)` 中取出值，或者让当前函数返回 `None` / `Err(e)`"),
        ("the field `{k}` appears twice", "字段 `{k}` 出现了两次"),
        ("`..` expects {expected} here, but got {}", "`..` 在这里需要{expected}，但得到的是 {}"),
        ("this value is not a function", "这个值不是函数"),
        ("cannot be called", "不能被调用"),
        ("to multiply, write `*` explicitly: `2 * (x + 1)`", "乘法要明确写出 `*`：`2 * (x + 1)`"),
        ("to take one element, use brackets: `x[0]`; between two values, a `,` may be missing", "要取一个元素，请用方括号：`x[0]`；两个值之间可能缺少一个 `,`"),
        ("to read a field, use a dot: `person.name`", "要读取字段，请用点：`person.name`"),
        ("`{s}` is not a function", "`{s}` 不是函数"),
        ("`{s}` is {}", "`{s}` 是 {}"),
        ("returns {returned}", "返回 {returned}"),
        ("this assertion always fails: {a} and {b} are never equal", "这个断言总是失败：{a} 和 {b} 永远不相等"),
        (
            r#"one value is inside an Option or a Result: compare it with `Some(...)` or `Ok(...)`,
like `assert_eq(parse_int("3"), Ok(3))`"#,
            r#"其中一个值在 Option 或 Result 里面：请把它和 `Some(...)` 或 `Ok(...)` 比较，
例如 `assert_eq(parse_int("3"), Ok(3))`"#,
        ),
        ("values of different kinds are never equal: Rythme never converts values silently", "不同种类的值永远不相等：Rythme 从不悄悄地转换值"),
        ("a constructor, shown as `<constructor ...>`", "一个构造器，显示为 `<constructor ...>`"),
        ("a function, shown as `<fn ...>`", "一个函数，显示为 `<fn ...>`"),
        ("this shows a function, not a value", "这里显示的是一个函数，而不是一个值"),
        ("to show what it gives, call it: `{call}`", "要显示它给出的结果，请调用它：`{call}`"),
        ("no definition of `{name}` accepts these arguments: {name}({})", "`{name}` 的定义都不接受这些参数：{name}({})"),
        ("called here", "在这里调用"),
        ("does not match", "不匹配"),
        ("add a definition that handles this case, for example one that ends with a plain name or `_`", "添加一个处理这种情况的定义，例如以普通名字或 `_` 结尾的定义"),
        ("the field `{name}` is not a function", "字段 `{name}` 不是函数"),
        ("`{name}` is {}", "`{name}` 是 {}"),
        ("to read the field, write it without parentheses: `.{name}`", "要读取这个字段，请不带括号地写：`.{name}`"),
        ("there is no function named `{shown}`", "没有名为 `{shown}` 的函数"),
        ("`value.{shown}(...)` calls `{shown}(value, ...)`, or the field `{shown}` of a record", "`value.{shown}(...)` 会调用 `{shown}(value, ...)`，或者记录的字段 `{shown}`"),
        ("`{name}` is a value, not a function", "`{name}` 是一个值，不是函数"),
        ("write `{name}` without parentheses", "写 `{name}` 时不要加括号"),
        ("this record has no field `{name}`", "这个记录没有字段 `{name}`"),
        ("unknown field", "未知字段"),
        ("the record is empty: {{}}", "这个记录是空的：{{}}"),
        ("its fields are: {}", "它的字段有：{}"),
        ("to get an Option instead of an error, use `get(record, \"{name}\")`", "如果想得到 Option 而不是错误，请使用 `get(record, \"{name}\")`"),
        ("{} has no fields", "{} 没有字段"),
        ("no field `{name}` here", "这里没有字段 `{name}`"),
        ("did you mean `.{best}`? (`value.{best}` calls `{best}(value)`)", "你是想写 `.{best}` 吗？（`value.{best}` 会调用 `{best}(value)`）"),
        ("`{c}` has no field `{name}`", "`{c}` 没有字段 `{name}`"),
        ("no variant of `{tn}` has a field `{name}`", "`{tn}` 的所有变体都没有字段 `{name}`"),
        ("`{c}` has no fields", "`{c}` 没有字段"),
        ("the fields of `{c}` are: {}", "`{c}` 的字段有：{}"),
        ("the variants of `{tn}` have no fields", "`{tn}` 的变体都没有字段"),
        ("their fields are: {}", "它们的字段有：{}"),
        ("a value of type `{tn}` may be another variant: use `match` to handle each case", "类型为 `{tn}` 的值可能是另一个变体：请用 `match` 处理每种情况"),
        ("cannot use `[...]` on {}", "不能对 {} 使用 `[...]`"),
        ("`[...]` works on lists, texts, records and dicts", "`[...]` 适用于列表、文本、记录和字典"),
        ("an index must be a whole number (Int), but got {}", "索引必须是整数（Int），但得到的是 {}"),
        ("the name of a field must be a text, but got {}", "字段名必须是文本，但得到的是 {}"),
        ("read a field with `record[\"name\"]` or `record.name`", "用 `record[\"name\"]` 或 `record.name` 读取字段"),
        ("this record has no field `{k}`", "这个记录没有字段 `{k}`"),
        ("use `get(record, key)` to receive an Option instead", "如果想得到 Option，请使用 `get(record, key)`"),
        ("{} and {}", "{} 和 {}"),
        ("a range needs whole numbers, but got {got}", "范围需要整数，但得到的是 {got}"),
        ("example: `1..10` is the list of the numbers from 1 to 10", "例如：`1..10` 是从 1 到 10 的数字列表"),
        ("cannot use `-` on {}", "不能对 {} 使用 `-`"),
        ("a condition must be `true` or `false`, but this is {}", "条件必须是 `true` 或 `false`，但这里是 {}"),
        ("not a Bool", "不是 Bool"),
        ("compare it explicitly, for example `x != 0` or `x > 0`", "请明确地进行比较，例如 `x != 0` 或 `x > 0`"),
        ("to test if it is empty, use `is_empty(x)` or `not is_empty(x)`", "要判断是否为空，请使用 `is_empty(x)` 或 `not is_empty(x)`"),
        ("use `is_some(x)`, `is_ok(x)`, or `match` to look inside", "用 `is_some(x)`、`is_ok(x)` 或 `match` 查看里面的内容"),
        ("cannot use `{sym}` with {ln} and {rn}", "不能对 {ln} 和 {rn} 使用 `{sym}`"),
        ("cannot use `{sym}` with {ln}", "不能对 {ln} 使用 `{sym}`"),
        ("cannot use `{sym}` with {rn}", "不能对 {rn} 使用 `{sym}`"),
        ("`{name} -x` subtracts here; to call `{name}`, write `{name}(-x)` (a call without parentheses starts a statement or a result, such as a branch of `if` or the body after the `=` of a function, and its name is not given a value with `=` or as a parameter)", "`{name} -x` 在这里表示减法；要调用 `{name}`，请写 `{name}(-x)`（不带括号的调用出现在语句或结果的开头，例如 `if` 的分支或函数 `=` 后面的函数体，而且它的名字没有通过 `=` 或作为参数获得值）"),
        ("to join two sets or two dicts, use `union(a, b)`; to take one away from the other, `difference(a, b)`", "要合并两个集合或两个字典，请使用 `union(a, b)`；要从一个中去掉另一个，请使用 `difference(a, b)`"),
        (
            r#"to build a text with a value inside, use braces: "total: {{x}}"
or convert the value first: text(x)"#,
            r#"要构造一个包含值的文本，请使用花括号："total: {{x}}"
或者先转换这个值：text(x)"#,
        ),
        ("to add one element to a list, use `push(list, x)` or `[..list, x]`", "要向列表添加一个元素，请使用 `push(list, x)` 或 `[..list, x]`"),
        ("to repeat a text or a list, use `repeat(x, n)`", "要重复一个文本或列表，请使用 `repeat(x, n)`"),
        ("convert the text to a number first, with `int(t)` or `float(t)`", "请先用 `int(t)` 或 `float(t)` 把文本转换成数字"),
        ("only values of the same kind can be compared", "只有同一种类的值才能比较"),
        ("this comparison is always false: {ln} and {rn} are never equal", "这个比较总是为假：{ln} 和 {rn} 永远不相等"),
        ("this comparison is always true: {ln} and {rn} are never equal", "这个比较总是为真：{ln} 和 {rn} 永远不相等"),
        ("to look inside an Option, use `match`, `unwrap_or(x, default)` or `is_some(x)`", "要查看 Option 里面的内容，请使用 `match`、`unwrap_or(x, default)` 或 `is_some(x)`"),
        ("Rythme never converts values silently: convert one side first, with `text(x)` or `int(t)`", "Rythme 从不悄悄地转换值：请先用 `text(x)` 或 `int(t)` 转换其中一边"),
        ("this case can never be reached", "这个情况永远不会被执行到"),
        ("this case already handles everything", "这个情况已经处理了所有值"),
        ("cases are tried in order: put `_` last", "情况是按顺序尝试的：请把 `_` 放在最后"),
        ("this case already handles these values", "这个情况已经处理了这些值"),
        ("a value goes to the first case that fits: remove this case, or change it", "值会进入第一个匹配的情况：去掉这个情况，或者修改它"),
        ("no case of this `match` handles {}", "这个 `match` 中没有情况处理 {}"),
        ("this value", "这个值"),
        ("add a case for this value, or a last case `_ -> ...` that handles everything else", "为这个值添加一个情况，或者添加最后一个情况 `_ -> ...` 来处理其他所有值"),
        ("this `match` does not handle every case", "这个 `match` 没有处理所有情况"),
        ("add a last case `_ -> ...` for all the other values", "为其他所有值添加最后一个情况 `_ -> ...`"),
        ("add the missing cases, like `{} -> ...`, or a last case `_ -> ...`", "添加缺少的情况，例如 `{} -> ...`，或者最后一个情况 `_ -> ...`"),
        ("unknown constructor `{n}`", "未知构造器 `{n}`"),
        ("constructors are defined with `type`: `type Name = {n}(field) | Other`", "构造器用 `type` 定义：`type Name = {n}(field) | Other`"),
        ("`{name}` has {} field(s), but this pattern gives {}", "`{name}` 有 {} 个字段，但这个模式给了 {} 个"),
        ("write it like `{example}`", "请写成 `{example}` 这样"),
        ("this range never matches", "这个范围永远不会匹配"),
        ("no number goes from {lo} up to {hi}", "没有数字能从 {lo} 增加到 {hi}"),
        ("a range goes from the smaller number to the larger one: `{hi}..{lo}`", "范围是从较小的数到较大的数：`{hi}..{lo}`"),
        ("patterns joined with `|` cannot give names (here `{s}`)", "用 `|` 连接的模式不能绑定名字（这里是 `{s}`）"),
        ("write separate cases instead", "请改为写成分开的情况"),
        ("handles some of these numbers", "处理了其中的一些数字"),
        ("together, the cases before it already handle all its numbers: remove it, or change it", "前面的情况合起来已经处理了它的所有数字：去掉它，或者修改它"),
        ("these ranges overlap", "这些范围有重叠"),
        ("{both} is also here", "{both} 也在这里"),
        ("{both} goes here, the first that fits", "{both} 会进入这里，即第一个匹配的情况"),
        ("both ends of a range are included: make the ranges follow each other without overlapping, like `0..9` then `10..19`", "范围的两端都包含在内：让范围一个接一个而不重叠，例如先 `0..9` 再 `10..19`"),
        ("use `match` to handle the different possible shapes of a value", "用 `match` 处理一个值可能的不同形式"),
        ("a value of type {}", "类型为 {} 的值"),
        ("this line computes a value but does nothing with it", "这一行计算了一个值，但没有使用它"),
        (
            r#"a line that starts with `-` is a new statement. To continue the line above,
end that line with `-` instead:
  total = price -
    discount"#,
            r#"以 `-` 开头的行是一条新语句。如果要接着上一行写，
请改为让上一行以 `-` 结尾：
  total = price -
    discount"#,
        ),
        (
            r#"to keep the value, give it a name: `result = ...`; to show it: `print(...)`.
Values never change: `x + 1` does not change `x`."#,
            r#"要保留这个值，请给它一个名字：`result = ...`；要显示它：`print(...)`。
值永远不会改变：`x + 1` 不会改变 `x`。"#,
        ),
        // src/diagnostic.rs
        ("error", "错误"),
        ("warning", "警告"),
        ("help", "帮助"),
        ("call trace (most recent first):", "调用栈（最近的在前）："),
        ("an anonymous function", "一个匿名函数"),
        ("in", "在"),
        ("called at", "中，调用位置"),
        ("and {} more calls", "还有 {} 次调用"),
        ("more about this error: `rythme explain {}`", "关于这个错误的更多信息：`rythme explain {}`"),
        // src/doc.rs
        ("`{a}` needs a file name, like `rythme doc geometry.ry --out geometry.html`", "`{a}` 需要一个文件名，例如 `rythme doc geometry.ry --out geometry.html`"),
        ("unknown option `{o}` for `rythme doc`", "`rythme doc` 的未知选项 `{o}`"),
        (
            r#"
did you mean `{best}`?"#,
            r#"
你是想写 `{best}` 吗？"#,
        ),
        (
            r#"
see `rythme help doc`"#,
            r#"
请参阅 `rythme help doc`"#,
        ),
        (
            r#"which files? Example: rythme doc geometry.ry
  a folder documents all its .ry files: rythme doc .
  --html writes one HTML page instead of Markdown
  --out FILE writes to a file (a name ending with .html gives HTML)
see `rythme help doc`"#,
            r#"哪些文件？例如：rythme doc geometry.ry
  对文件夹会为其中所有 .ry 文件生成文档：rythme doc .
  --html 生成一个 HTML 页面，而不是 Markdown
  --out FILE 写入一个文件（以 .html 结尾的名字会生成 HTML）
请参阅 `rythme help doc`"#,
        ),
        ("no .ry file in `{target}`", "`{target}` 中没有 .ry 文件"),
        ("Documentation of {one}", "{one} 的文档"),
        ("Documentation", "文档"),
        ("written: {file}", "已写入：{file}"),
        ("cannot write the file `{file}`: {e}", "无法写入文件 `{file}`：{e}"),
        ("Types", "类型"),
        ("Functions", "函数"),
        ("Values", "值"),
        ("Tests", "测试"),
        ("Uses", "使用"),
        ("Contents", "目录"),
        ("Built-in functions mentioned", "提到的内置函数"),
        ("Example", "示例"),
        ("Nothing public to document.", "没有可以生成文档的公开内容。"),
        ("Documentation written by `rythme doc`", "由 `rythme doc` 生成的文档"),
        (" Same as: {} (its Ruby names).", " 等同于：{}（它们的 Ruby 名字）。"),
        ("Example:", "示例："),
        (" as {open}{n}{close}", " （名为 {open}{n}{close}）"),
        (": ", ": "),
        // src/driver.rs
        ("`{shown}` is a folder, not a file", "`{shown}` 是文件夹，不是文件"),
        ("give a file inside it, like {}", "请指定其中的一个文件，例如 {}"),
        ("file `{shown}` not found", "找不到文件 `{shown}`"),
        ("no permission to read the file `{shown}`", "没有读取文件 `{shown}` 的权限"),
        ("cannot read the file `{shown}`: {e}", "无法读取文件 `{shown}`：{e}"),
        ("the file `{shown}` is not text in the UTF-8 encoding (unexpected byte on line {line})", "文件 `{shown}` 不是 UTF-8 编码的文本（第 {line} 行有意外的字节）"),
        ("it looks like it was saved in UTF-16", "它看起来是用 UTF-16 保存的"),
        ("save it again with the UTF-8 encoding (most editors have it in \"Save as\" or in the status bar)", "请用 UTF-8 编码重新保存（大多数编辑器在“另存为”或状态栏里都有这个选项）"),
        ("(`{name}` now has {n} definitions, tried in order; :reset to start again)", "（`{name}` 现在有 {n} 个定义，按顺序尝试；输入 :reset 可以重新开始）"),
        ("file `{p}` not found", "找不到文件 `{p}`"),
        ("looked for: {}", "查找过：{}"),
        ("the Rythme files there are: {}{more}", "那里的 Rythme 文件有：{}{more}"),
        ("the path is relative to the folder of the file that contains `use`", "路径是相对于包含 `use` 的文件所在的文件夹"),
        ("`{p}` uses itself", "`{p}` 使用了它自己"),
        ("`{last}` and `{first}` use each other", "`{last}` 和 `{first}` 互相使用"),
        ("the circle: {}", "循环：{}"),
        ("files cannot use each other: move what both need into a third file, used by both", "文件不能互相使用：把两者都需要的内容移到第三个文件中，让两者都使用它"),
        ("the name `{n}` is given to two files: `{first}` and `{second}`", "名字 `{n}` 被同时给了两个文件：`{first}` 和 `{second}`"),
        ("give this one another name: `use ... as other_name`", "给这个文件换个名字：`use ... as other_name`"),
        ("`{s}` is defined by two used files: `{first}` and `{second}`", "`{s}` 在两个被使用的文件中都有定义：`{first}` 和 `{second}`"),
        ("use the files with their names, without quotes, and write which one you mean: `use geometry`, then `geometry.{s}`", "用名字（不带引号）使用这些文件，并写明你指的是哪一个：`use geometry`，然后 `geometry.{s}`"),
        // src/exercise.rs
        ("which exercise? Example: rythme exercise start 1   (`rythme exercise list` shows them all)", "哪个练习？例如：rythme exercise start 1   （`rythme exercise list` 会列出全部练习）"),
        ("which file? Example: rythme exercise check 01-values.ry", "哪个文件？例如：rythme exercise check 01-values.ry"),
        ("unknown action `{other}` for `rythme exercise`", "`rythme exercise` 的未知操作 `{other}`"),
        (
            r#"
did you mean `rythme exercise {best}`?"#,
            r#"
你是想写 `rythme exercise {best}` 吗？"#,
        ),
        (
            r#"
the actions are: list, start NAME, check FILE (see `rythme help exercises`)"#,
            r#"
可用的操作有：list、start NAME、check FILE（参见 `rythme help exercises`）"#,
        ),
        ("{} exercises for a first course, from the first values to tests, from ★☆☆ (easy) to ★★★.", "适合入门课程的 {} 个练习，从最初的值到测试，难度从 ★☆☆（简单）到 ★★★。"),
        (
            r#"Start one:       rythme exercise start 1
Check your work: rythme exercise check {}.ry"#,
            r#"开始一个练习：rythme exercise start 1
检查你的作业：rythme exercise check {}.ry"#,
        ),
        ("there is no exercise `{name}`", "没有练习 `{name}`"),
        (
            r#"
the exercises are numbered from 1 to {}: `rythme exercise list` shows them"#,
            r#"
练习的编号是 1 到 {}：`rythme exercise list` 会列出它们"#,
        ),
        ("`{file}` already exists here: open it to continue, or rename it to start again", "`{file}` 在这里已经存在：打开它继续做，或者把它改名后重新开始"),
        ("cannot write `{file}`: {error}", "无法写入 `{file}`：{error}"),
        (
            r#"Created {file}: exercise {} · {title}  {level}

Open it, read the statement and the hints at the top, and replace each
`fail("to do")` with your code. Then check your work:

  rythme exercise check {file}"#,
            r#"已创建 {file}：练习 {} · {title}  {level}

打开它，阅读顶部的题目和提示，然后把每个
`fail("to do")` 替换成你的代码。之后检查你的作业：

  rythme exercise check {file}"#,
        ),
        ("file `{name}` not found", "找不到文件 `{name}`"),
        (
            r#"
to start this exercise: rythme exercise start {}"#,
            r#"
要开始这个练习：rythme exercise start {}"#,
        ),
        (
            r#"✗ The file has errors, so its tests could not run.
  Fix the first error above, then check again."#,
            r#"✗ 文件有错误，所以它的测试无法运行。
  请先修正上面的第一个错误，然后再检查一次。"#,
        ),
        ("✗ This file has no tests. An exercise comes with its tests: is it the right file?", "✗ 这个文件没有测试。练习都带有自己的测试：这是正确的文件吗？"),
        ("✓ Well done: all {total} tests pass!", "✓ 做得好：全部 {total} 个测试都通过了！"),
        (
            r#"  Next exercise: {title}  {level}
    rythme exercise start {}"#,
            r#"  下一个练习：{title}  {level}
    rythme exercise start {}"#,
        ),
        ("  That was the last exercise: you have finished them all. Congratulations!", "  这是最后一个练习：你已经全部完成了。恭喜！"),
        ("✗ All {total} tests pass, but...", "✗ 全部 {total} 个测试都通过了，但是……"),
        ("✗ {passed} of {total} tests pass. Keep going!", "✗ {total} 个测试中通过了 {passed} 个。继续加油！"),
        (
            r#"  - The tests of the exercise were changed: the exercise is solved when its own tests
    pass. `rythme exercise start {number}` gives a fresh copy (rename your file first)."#,
            r#"  - 练习的测试被修改过：只有练习自带的测试通过，练习才算完成。
    `rythme exercise start {number}` 会给你一份新的副本（请先把你的文件改名）。"#,
        ),
        ("  - Some functions are still to do: replace each `fail(\"to do\")` with your code.", "  - 还有一些函数没有完成：把每个 `fail(\"to do\")` 替换成你的代码。"),
        (
            r#"  - Start with the first failure above: compare what you got with what was expected.
  - Stuck? Read the hints at the top of the file, or `rythme help NAME` for a function."#,
            r#"  - 从上面的第一个失败开始：比较你得到的结果和期望的结果。
  - 卡住了？阅读文件顶部的提示，或者用 `rythme help NAME` 查看某个函数。"#,
        ),
        // src/format.rs
        ("`{name}` could not be formatted safely, so it was left as it is", "无法安全地格式化 `{name}`，所以保持原样"),
        ("this is a bug in Rythme, not in your program: please report it, with the file, at https://github.com/maxlestage/Rythme/issues", "这是 Rythme 的缺陷，不是你的程序的问题：请把问题连同文件一起报告到 https://github.com/maxlestage/Rythme/issues"),
        ("unknown option `{o}` for `rythme fmt`", "`rythme fmt` 的未知选项 `{o}`"),
        (
            r#"
see `rythme help fmt`"#,
            r#"
请参阅 `rythme help fmt`"#,
        ),
        (
            r#"which files? Example: rythme fmt hello.ry
  a folder formats all its .ry files: rythme fmt .
  - reads the code from the standard input
  --check only checks, for continuous integration
see `rythme help fmt`"#,
            r#"哪些文件？例如：rythme fmt hello.ry
  对文件夹会格式化其中所有 .ry 文件：rythme fmt .
  - 从标准输入读取代码
  --check 只检查，用于持续集成
请参阅 `rythme help fmt`"#,
        ),
        ("needs formatting: {shown}", "需要格式化：{shown}"),
        ("cannot write the file `{shown}`: {e}", "无法写入文件 `{shown}`：{e}"),
        ("formatted: {shown}", "已格式化：{shown}"),
        ("needs formatting: <stdin>", "需要格式化：<stdin>"),
        ("{changed} file(s) formatted, {unchanged} already formatted", "格式化了 {changed} 个文件，{unchanged} 个已经是规范格式"),
        ("✓ {unchanged} file(s) already formatted", "✓ {unchanged} 个文件已经是规范格式"),
        ("✗ {changed} file(s) to format, {unchanged} already formatted; `rythme fmt` (without --check) formats them", "✗ {changed} 个文件需要格式化，{unchanged} 个已经是规范格式；`rythme fmt`（不带 --check）会格式化它们"),
        ("✗ {errors} file(s) left as they are because of errors", "✗ {errors} 个文件因为有错误而保持原样"),
        ("file or folder `{shown}` not found", "找不到文件或文件夹 `{shown}`"),
        ("did you mean `{}`?", "你是想写 `{}` 吗？"),
        // src/help.rs
        (
            r#"Rythme {v} — help

Commands
  rythme                    interactive mode (try code line by line)
  rythme file.ry            run a program
  rythme check file.ry      find problems without running the program
  rythme test file.ry       run the tests of a file (or of a folder)
  rythme fmt file.ry        give the code its official layout (--check: only check)
  rythme doc file.ry        write its documentation, in Markdown (--html: a web page)
  rythme steps file.ry      show the calculation step by step (fib(3) → fib(2) + fib(1) → ...)
  rythme exercise           exercises checked automatically (list, start, check)
  rythme new NAME           create a project (rythme.toml, src/, tests/)
  rythme install            download the dependencies of the project
  rythme learn              guided tour of the language, step by step
  rythme help TOPIC         help on a topic or a function
  rythme explain E003       explain an error code
  rythme -e "code"          run a piece of code
  rythme lsp                language server, for editors (VS Code, Neovim...)
  option --lang CODE        language of the messages: en, fr, es, de, pt, zh (or RYTHME_LANG)

Topics (rythme help TOPIC)
  start       the language in one minute
  syntax      the whole syntax on one page
  functions   functions, |>, recursion
  lists       lists and their functions
  text        texts
  records     records {{name: value}}
  dicts       dictionaries and sets: dict(...), set(...)
  types       your own types
  match       choosing according to the shape of a value
  errors      Option, Result and ?
  functional  no loops: what to do instead
  tests       writing and running tests
  exercises   exercises to learn, with their tests
  modules     using several files
  system      files, folders, paths, options, input, exit codes
  projects    projects and packages: rythme new, rythme.toml, rythme install
  dates       dates and times: now(), today(), calendar, ISO 8601
  regex       searching, cutting and replacing with regular expressions
  repl        the interactive mode
  fmt         the official layout of the code (rythme fmt)
  doc         documentation from the ## comments (rythme doc)
  steps       the calculation shown step by step (rythme steps)
  editors     Rythme in your editor (VS Code, Neovim, Helix...)
  ruby        coming from Ruby: blocks, symbols, method chains
  builtins    every built-in function

A specific function: rythme help map
"#,
            r#"Rythme {v} — 帮助

命令
  rythme                    交互模式（逐行试验代码）
  rythme file.ry            运行程序
  rythme check file.ry      不运行程序，找出问题
  rythme test file.ry       运行一个文件（或一个文件夹）的测试
  rythme fmt file.ry        把代码整理成官方格式（--check：只检查）
  rythme doc file.ry        生成它的文档，Markdown 格式（--html：网页）
  rythme steps file.ry      一步一步显示计算过程（fib(3) → fib(2) + fib(1) → ...）
  rythme exercise           自动检查的练习（list、start、check）
  rythme new NAME           创建项目（rythme.toml、src/、tests/）
  rythme install            下载项目的依赖
  rythme learn              一步一步的语言导览
  rythme help TOPIC         某个主题或函数的帮助
  rythme explain E003       解释错误代码
  rythme -e "code"          运行一段代码
  rythme lsp                语言服务器，供编辑器使用（VS Code、Neovim……）
  选项 --lang CODE          消息的语言：en、fr、es、de、pt、zh（或 RYTHME_LANG）

主题（rythme help TOPIC）
  start       一分钟了解这门语言
  syntax      一页纸看完全部语法
  functions   函数、|>、递归
  lists       列表及其函数
  text        文本
  records     记录 {{name: value}}
  dicts       字典和集合：dict(...)、set(...)
  types       你自己的类型
  match       根据值的形状进行选择
  errors      Option、Result 和 ?
  functional  没有循环：该怎么做
  tests       编写和运行测试
  exercises   用来学习的练习，带有测试
  modules     使用多个文件
  system      文件、文件夹、路径、选项、输入、退出码
  projects    项目和包：rythme new、rythme.toml、rythme install
  dates       日期和时间：now()、today()、日历、ISO 8601
  regex       用正则表达式搜索、切分和替换
  repl        交互模式
  fmt         代码的官方格式（rythme fmt）
  doc         根据 ## 注释生成文档（rythme doc）
  steps       一步一步显示计算过程（rythme steps）
  editors     在你的编辑器中使用 Rythme（VS Code、Neovim、Helix……）
  ruby        从 Ruby 转过来：代码块、符号、方法链
  builtins    所有内置函数

查看某个函数：rythme help map
"#,
        ),
        ("{alias}  (the Ruby name of `{of}`: the same function)", "{alias}  （`{of}` 的 Ruby 名字：同一个函数）"),
        (
            r#"pi
  The number π = 3.141592653589793."#,
            r#"pi
  圆周率 π = 3.141592653589793。"#,
        ),
        ("The function:", "函数："),
        ("No help found for `{name}`.", "没有找到 `{name}` 的帮助。"),
        (
            r#"
Did you mean `{best}`?"#,
            r#"
你是想找 `{best}` 吗？"#,
        ),
        (
            r#"
See the list of topics with `rythme help`."#,
            r#"
用 `rythme help` 查看主题列表。"#,
        ),
        ("keyword", "关键字"),
        (
            r#"  More: rythme help {topic}
"#,
            r#"  更多：rythme help {topic}
"#,
        ),
        (
            r#"  Same as: {} (its Ruby names)
"#,
            r#"  等同于：{}（它们的 Ruby 名字）
"#,
        ),
        (
            r#"BUILT-IN FUNCTIONS
"#,
            r#"内置函数
"#,
        ),
        ("the number π", "圆周率 π"),
        ("{} (same as {})", "{}（同 {}）"),
        ("  Ruby names:", "  Ruby 名字："),
        ("build an Option or a Result", "构造 Option 或 Result"),
        (
            r#"
Details and example for one function: rythme help NAME (or help(NAME) in code)
"#,
            r#"
查看某个函数的详细说明和示例：rythme help NAME（或在代码中写 help(NAME)）
"#,
        ),
        (
            r#"help(x) explains x: a function (help(map)), a topic (help("lists")), or a value (help([1, 2])).
Topics: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
            r#"help(x) 解释 x：一个函数（help(map)）、一个主题（help("lists")）或一个值（help([1, 2])）。
主题：start、syntax、functions、lists、text、records、types、match、errors、functional、tests、modules、builtins。"#,
        ),
        (
            r#"  (no documentation: write `## description` on the line above `fn`)
"#,
            r#"  （没有文档：在 `fn` 的上一行写 `## 说明`）
"#,
        ),
        ("defined at", "定义于"),
        (
            r#"{}({})
  A constructor of the type `{}`.
"#,
            r#"{}({})
  类型 `{}` 的一个构造器。
"#,
        ),
        (
            r#"This is a value of type {ty}: {}
"#,
            r#"这是一个类型为 {ty} 的值：{}
"#,
        ),
        (
            r#"Useful functions: {}
"#,
            r#"有用的函数：{}
"#,
        ),
        (
            r#"Also: {also}
"#,
            r#"另见：{also}
"#,
        ),
        (
            r#"Use `match` to handle each variant of `{}`.
"#,
            r#"用 `match` 处理 `{}` 的每个变体。
"#,
        ),
        ("[Enter] next   [q] quit > ", "[Enter] 下一步   [q] 退出 > "),
        ("To practice, run `rythme` and type code, or do the exercises: `rythme exercise list`. `rythme help builtins` lists every function.", "要练习的话，运行 `rythme` 并输入代码，或者做练习：`rythme exercise list`。`rythme help builtins` 会列出所有函数。"),
        // src/interp.rs
        ("this list would be too large (more than {MAX_LIST} elements)", "这个列表会太大（超过 {MAX_LIST} 个元素）"),
        ("this text would be too large (more than {mb} MB)", "这个文本会太大（超过 {mb} MB）"),
        ("does it grow in a function that calls itself? Check that the function always reaches a case where it stops", "它是不是在一个调用自身的函数里不断增长？请检查函数是否总能到达停止的情况"),
        ("no function named `{}`", "没有名为 `{}` 的函数"),
        ("`value.{0}(...)` calls `{0}(value, ...)`", "`value.{0}(...)` 会调用 `{0}(value, ...)`"),
        ("`{name}` is not defined yet", "`{name}` 还没有定义"),
        ("the file runs from top to bottom: move this definition above the line that uses it (directly or through a function)", "文件是从上到下运行的：请把这个定义移到使用它的那一行（直接使用或通过函数使用）的上面"),
        ("`{ctor}` has no field `{name}`", "`{ctor}` 没有字段 `{name}`"),
        ("`{ctor}` has no fields", "`{ctor}` 没有字段"),
        ("the fields of `{ctor}` are: {}", "`{ctor}` 的字段有：{}"),
        ("a value of type `{}` may be another variant: use `match` to handle each case", "类型为 `{}` 的值可能是另一个变体：请用 `match` 处理每种情况"),
        ("too many nested calls ({depth} calls in progress)", "嵌套调用太多（正在进行 {depth} 层调用）"),
        ("this call goes too deep", "这个调用太深了"),
        (
            r#"a recursive function that never stops? Check that it always reaches a base case.
Otherwise, make the recursive call the very last thing the function does
(tail call), for example with an accumulator:
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
or use `fold`, `map`, `filter`, which have no limit."#,
            r#"是不是一个永不停止的递归函数？请检查它是否总能到达基本情况。
否则，让递归调用成为函数做的最后一件事
（尾调用），例如使用累加器：
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
或者使用没有限制的 `fold`、`map`、`filter`。"#,
        ),
        ("the program was stopped: it made more calls than allowed for this run", "程序被停止了：它的调用次数超过了这次运行允许的上限"),
        ("does it call itself forever? Check that it always reaches a base case", "它是不是在无限地调用自己？请检查它是否总能到达基本情况"),
        ("{} is not a function", "{} 不是函数"),
        ("`{0}` is a value, not a function: write `{0}` without parentheses", "`{0}` 是一个值，不是函数：写 `{0}` 时不要加括号"),
        ("`..` expects {what} here, but got {}", "`..` 在这里需要{what}，但得到的是 {}"),
        ("division by zero", "除以零"),
        (
            r#"check the divisor first: `if b == 0 then ... else a / b`,
or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`"#,
            r#"请先检查除数：`if b == 0 then ... else a / b`，
或者返回一个 Result：`if b == 0 then Err("division by zero") else Ok(a / b)`"#,
        ),
        ("`0 ** -n` is `1 / 0 ** n`", "`0 ** -n` 等于 `1 / 0 ** n`"),
        ("{min} argument{s}", "{min} 个参数"),
        ("at least {min} argument{s}", "至少 {min} 个参数"),
        ("{min} to {max} arguments", "{min} 到 {max} 个参数"),
        ("{got} {verb}", "{got}"),
        ("`{name}` takes {expected}, but {given}", "`{name}` 需要 {expected}，但收到了 {given} 个"),
        ("{got} argument(s) here", "这里有 {got} 个参数"),
        ("usage: `{sig}`", "用法：`{sig}`"),
        ("cannot use `{sym}` with {lt} and {rt}", "不能对 {lt} 和 {rt} 使用 `{sym}`"),
        ("`nan` (not a number) cannot be compared", "`nan`（不是数字）不能比较"),
        ("a negative number ({}) to a fractional power ({}) has no value among real numbers", "负数（{}）的分数次幂（{}）在实数范围内没有值"),
        ("use `abs(x) ** p` for the power of its absolute value", "如果要计算它的绝对值的幂，请使用 `abs(x) ** p`"),
        ("this range is too large ({} elements)", "这个范围太大了（{} 个元素）"),
        ("ranges are lists: keep them under {MAX_RANGE} elements", "范围就是列表：请让它少于 {MAX_RANGE} 个元素"),
        ("a range needs whole numbers, but got {} and {}", "范围需要整数，但得到的是 {} 和 {}"),
        ("index {n} is out of range: {what}", "索引 {n} 超出范围：{what}"),
        ("valid indices go from 0 to {} (or -1 for the last element)", "有效索引是 0 到 {}（或者用 -1 表示最后一个元素）"),
        ("to get an Option instead of an error, use `get(x, {n})`", "如果想得到 Option 而不是错误，请使用 `get(x, {n})`"),
        ("the text is empty", "文本是空的"),
        ("the text has {len} character(s)", "文本有 {len} 个字符"),
        ("the list has {} element(s)", "列表有 {} 个元素"),
        ("this dict has no key {}", "这个字典没有键 {}"),
        ("use `get(dict, key)` to receive an Option instead, or `has(dict, key)` to check first", "如果想得到 Option，请使用 `get(dict, key)`；或者先用 `has(dict, key)` 检查"),
        // src/lexer.rs
        ("`...` is not an operator in Rythme", "`...` 不是 Rythme 的运算符"),
        ("write `1..10` for a range (10 included), or `[first, ..rest]` in a list", "范围写作 `1..10`（包括 10），在列表中写作 `[first, ..rest]`"),
        ("`++` is not an operator in Rythme", "`++` 不是 Rythme 的运算符"),
        (
            r#"to add one, write `x + 1` and give the result a new name (values never change);
to join two lists or two texts, use `+`: `[1] + [2]`"#,
            r#"要加一，请写 `x + 1` 并给结果起一个新名字（值永远不会改变）；
要连接两个列表或两个文本，请使用 `+`：`[1] + [2]`"#,
        ),
        ("unexpected character `&`", "意外的字符 `&`"),
        ("use `and` to combine two conditions", "用 `and` 连接两个条件"),
        ("this `{{` in a text is never closed", "文本中的这个 `{{` 一直没有闭合"),
        ("starts a value inserted in the text", "开始一个插入到文本中的值"),
        ("close it with `}}`, or write `\\{{` to get a literal brace: \"\\{{\"", "用 `}}` 闭合它，或者写 `\\{{` 表示花括号本身：\"\\{{\""),
        ("unexpected invisible character {code}", "意外的不可见字符 {code}"),
        ("this is a special space (often copied from a document or a web page): replace it with a normal space", "这是一个特殊的空格（通常是从文档或网页中复制来的）：请把它换成普通空格"),
        ("this character cannot be seen (it is often copied from a document or a web page): delete it", "这个字符看不见（通常是从文档或网页中复制来的）：请删除它"),
        ("unexpected character `{c}`", "意外的字符 `{c}`"),
        ("write `{s}` instead", "请改写为 `{s}`"),
        ("texts use double quotes: \"hello\"", "文本使用英文双引号：\"hello\""),
        ("use `**` for powers: `2 ** 10`", "幂运算用 `**`：`2 ** 10`"),
        ("use straight double quotes `\"` for texts", "文本请使用英文直双引号 `\"`"),
        ("to insert a value in a text, use braces: \"total: {{x}}\"", "要在文本中插入一个值，请使用花括号：\"total: {{x}}\""),
        ("invalid number `{text}`", "无效的数字 `{text}`"),
        ("hexadecimal numbers use the digits 0 to 9 and a to f, like `0xff`; binary numbers use 0 and 1, like `0b101`", "十六进制数使用数字 0 到 9 和字母 a 到 f，例如 `0xff`；二进制数使用 0 和 1，例如 `0b101`"),
        ("a name cannot start with a digit: `{}`", "名字不能以数字开头：`{}`"),
        ("to multiply, write it explicitly: `{num} * {word}`", "乘法要明确写出来：`{num} * {word}`"),
        ("texts are nested too deeply inside `{{...}}`", "`{{...}}` 中的文本嵌套太深"),
        ("this text is never closed", "这个文本一直没有闭合"),
        ("the text starts here", "文本从这里开始"),
        ("add a `\"` at the end of the text", "在文本末尾加上 `\"`"),
        ("invalid unicode escape", "无效的 Unicode 转义"),
        ("write it like `\\u{{e9}}` for é", "写成 `\\u{{e9}}` 这样表示 é"),
        ("unknown escape sequence in text", "文本中有未知的转义序列"),
        ("in a regex, write the backslash twice: \"\\\\{c}\" is the text \\{c}", "在正则表达式中，反斜杠要写两次：\"\\\\{c}\" 表示文本 \\{c}"),
        (
            r#"valid escapes: \n (new line), \t (tab), \r (carriage return), \0 (null character), \\ (backslash),
\" (quote), \{{ and \}} (braces), \# (a `#` before a brace), \u{{e9}} (a character by its Unicode number: é)"#,
            r#"有效的转义：\n（换行）、\t（制表符）、\r（回车）、\0（空字符）、\\（反斜杠）、
\"（引号）、\{{ 和 \}}（花括号）、\#（花括号前面的 `#`）、\u{{e9}}（用 Unicode 编号表示的字符：é）"#,
        ),
        ("inside it, this text is never closed", "在它里面，这个文本一直没有闭合"),
        ("empty `{{}}` in a text", "文本中有空的 `{{}}`"),
        ("put a value inside, like \"{{name}}\", or write `\\{{` for a literal brace", "在里面放一个值，例如 \"{{name}}\"，或者写 `\\{{` 表示花括号本身"),
        ("inside a text, `{{` starts an inserted value: to write braces, use `\\{{` and `\\}}`, like \"\\{{\\\"a\\\": 1\\}}\"", "在文本中，`{{` 会开始一个插入的值：要写花括号，请使用 `\\{{` 和 `\\}}`，例如 \"\\{{\\\"a\\\": 1\\}}\""),
        ("the number `{n}`", "数字 `{n}`"),
        ("the number `{f}`", "数字 `{f}`"),
        ("the name `{s}`", "名字 `{s}`"),
        ("the symbol `:{s}`", "符号 `:{s}`"),
        ("the end of the line", "行尾"),
        ("the end of the code", "代码结尾"),
        ("the keyword `{k}`", "关键字 `{k}`"),
        // src/lsp/analysis.rs
        ("cannot read the file `{shown}`", "无法读取文件 `{shown}`"),
        ("the file `{p}` has {n} error(s)", "文件 `{p}` 有 {n} 个错误"),
        ("first: {}{place}", "第一个：{}{place}"),
        ("open `{p}` to see and fix them", "打开 `{p}` 查看并修正它们"),
        // src/lsp/extras.rs
        ("▶ Run", "▶ 运行"),
        ("▶ Run the {n} tests", "▶ 运行 {n} 个测试"),
        ("▶ Run test", "▶ 运行测试"),
        // src/lsp/features.rs
        (
            r#"
more: `rythme explain {}`"#,
            r#"
更多信息：`rythme explain {}`"#,
        ),
        (
            r#"

Same as: {} (its Ruby names)"#,
            r#"

等同于：{}（它们的 Ruby 名字）"#,
        ),
        ("**{alias}** — the Ruby name of `{of}`: the same function", "**{alias}** — `{of}` 的 Ruby 名字：同一个函数"),
        ("A value that is there, in an `Option`. The other case is `None`.", "`Option` 中存在的值。另一种情况是 `None`。"),
        ("No value, in an `Option`: what `find`, `first` or `get` give when there is nothing. Handle it with `match` or `unwrap_or`.", "`Option` 中的“没有值”：`find`、`first` 或 `get` 在什么都没有时给出的结果。请用 `match` 或 `unwrap_or` 处理它。"),
        ("The result of an operation that succeeded, in a `Result`. The other case is `Err(error)`.", "`Result` 中成功操作的结果。另一种情况是 `Err(error)`。"),
        ("The result of an operation that failed, in a `Result`. Inside a function, `?` stops and passes it on.", "`Result` 中失败操作的结果。在函数内部，`?` 会停止并把它传递出去。"),
        ("**{word}** is not used in Rythme.", "Rythme 中不使用 **{word}**。"),
        ("{} more definitions", "还有 {} 个定义"),
        ("Inferred from its code:", "根据它的代码推断："),
        ("local function", "局部函数"),
        ("parameter of `{f}`", "`{f}` 的参数"),
        ("parameter of an anonymous function", "匿名函数的参数"),
        ("local value", "局部值"),
        ("given by the case `{pattern}` of a `match`", "由 `match` 的情况 `{pattern}` 给出"),
        ("constructor of the type `{t}`", "类型 `{t}` 的构造器"),
        ("*defined in `{name}`*", "*定义于 `{name}`*"),
        ("The number π.", "圆周率 π。"),
        (
            r#"

`value.{n}(...)` calls `{n}(value, ...)`, or the field `{n}` of a record; `value.{n}` reads the field `{n}`, or calls `{n}(value)`"#,
            r#"

`value.{n}(...)` 会调用 `{n}(value, ...)`，或者记录的字段 `{n}`；`value.{n}` 会读取字段 `{n}`，或者调用 `{n}(value)`"#,
        ),
        ("the file `{p}` is not found", "找不到文件 `{p}`"),
        (
            r#"

Its definitions are used through the name `{n}`: `{n}.name`"#,
            r#"

它的定义通过名字 `{n}` 使用：`{n}.name`"#,
        ),
        ("Defines:", "定义了："),
        ("name", "名字"),
        ("value", "值"),
        ("other", "其他"),
        ("pattern", "模式"),
        ("result", "结果"),
        ("what it checks", "检查的内容"),
        ("actual", "实际值"),
        ("expected", "期望值"),
        ("condition", "条件"),
        ("file", "文件"),
        ("a function on several lines", "多行函数"),
        ("a function on one line", "单行函数"),
        ("choose according to the shape of a value", "根据值的形状进行选择"),
        ("a test, run by `rythme test`", "一个测试，由 `rythme test` 运行"),
        ("a test on several lines", "多行测试"),
        ("choose between two values", "在两个值之间选择"),
        ("type ${{1:Name}} = ${{2:First}}(${{3:field}}) | ${{0:Second}}", "type ${{1:Name}} = ${{2:First}}(${{3:field}}) | ${{0:Second}}"),
        ("your own type", "你自己的类型"),
        ("several steps inside an expression", "表达式中的多个步骤"),
        ("a block: a function given as the last argument of a call", "代码块：作为调用的最后一个参数传入的函数"),
        ("the opposite of `if`", "`if` 的反面"),
        ("use another file: `use geometry`, then `geometry.area(2)`", "使用另一个文件：`use geometry`，然后 `geometry.area(2)`"),
        ("same as {}", "同 {}"),
        ("field", "字段"),
        ("package", "包"),
        ("folder", "文件夹"),
        ("Add `use {file}` and write `{stem}.{written}`", "添加 `use {file}` 并写 `{stem}.{written}`"),
        ("Replace with `{new}`", "替换为 `{new}`"),
        // src/lsp/mod.rs
        ("rythme lsp: unknown option `{other}` (ignored)", "rythme lsp：未知选项 `{other}`（已忽略）"),
        ("rythme lsp is the language server: your editor starts it and talks to it. See `rythme help editors`. (Ctrl-C to quit)", "rythme lsp 是语言服务器：由你的编辑器启动并与它通信。参见 `rythme help editors`。（按 Ctrl-C 退出）"),
        ("the server is not initialized yet", "服务器尚未初始化"),
        ("the server is shutting down", "服务器正在关闭"),
        ("internal error in Rythme (please report it): {}", "Rythme 内部错误（请报告此问题）：{}"),
        ("not formatted: the code has an error ({first})", "未格式化：代码中有错误（{first}）"),
        // src/lsp/refs.rs
        ("there is no name here", "这里没有名字"),
        ("the name of a test is a text: edit it directly", "测试的名字是文本：请直接编辑它"),
        ("`{name}` cannot be found", "找不到 `{name}`"),
        ("`{name}` is a built-in function: it cannot be renamed", "`{name}` 是内置函数：不能重命名"),
        ("`{name}` is part of Rythme: it cannot be renamed", "`{name}` 是 Rythme 自带的：不能重命名"),
        ("`{name}` is not defined: there is nothing to rename", "`{name}` 没有定义：没有可重命名的东西"),
        ("`{new}` is not a valid name", "`{new}` 不是有效的名字"),
        ("`{new}` is a keyword", "`{new}` 是关键字"),
        ("the names of types and constructors start with an uppercase letter", "类型和构造器的名字以大写字母开头"),
        ("names start with a lowercase letter (uppercase is for types and constructors)", "名字以小写字母开头（大写字母用于类型和构造器）"),
        ("`{new}` is a built-in function: choose another name", "`{new}` 是内置函数：请换个名字"),
        ("`{shown}` has a syntax error: fix it before renaming", "`{shown}` 有语法错误：请在重命名之前修正它"),
        ("`{old_name}` is also the name of a record field, and `value.{old_name}(...)` may call that field: rename it by hand", "`{old_name}` 也是某个记录字段的名字，而 `value.{old_name}(...)` 可能会调用那个字段：请手动重命名"),
        ("cannot rename `{old_name}` to `{new_name}`: {why}", "无法把 `{old_name}` 重命名为 `{new_name}`：{why}"),
        ("some names would then refer to something else", "有些名字将会指向别的东西"),
        ("the code would not be valid", "代码将会无效"),
        // src/manifest.rs
        ("expected a name here", "这里需要一个名字"),
        ("each line is `name = \"value\"`, like `version = \"0.1.0\"`", "每一行都是 `name = \"value\"`，例如 `version = \"0.1.0\"`"),
        ("this text is not closed", "这个文本没有闭合"),
        ("add `{quote}` at the end of the text", "在文本末尾加上 `{quote}`"),
        ("unknown escape in a text", "文本中有未知的转义"),
        ("write `\\\\` for a backslash, or use `/` in paths: they work on every system", "反斜杠请写成 `\\\\`，或者在路径中使用 `/`：它在所有系统上都能用"),
        ("expected `=` after `{key}`", "`{key}` 后面需要 `=`"),
        ("`{key}` is written twice", "`{key}` 写了两次"),
        ("expected `,` or `}}` here", "这里需要 `,` 或 `}}`"),
        ("a table is written on one line: `{{ git = \"...\", tag = \"...\" }}`", "表格要写在一行里：`{{ git = \"...\", tag = \"...\" }}`"),
        ("a value is a text, written in quotes", "值是写在引号里的文本"),
        ("write `\"{example}\"`", "请写 `\"{example}\"`"),
        ("expected `{close}` here", "这里需要 `{close}`"),
        ("write `{key} = \"...\"`", "请写 `{key} = \"...\"`"),
        ("first written here", "第一次在这里写"),
        ("unexpected text after the value", "值后面有意外的文本"),
        ("write one value per line; a comment starts with `#`", "每行写一个值；注释以 `#` 开头"),
        ("`{name}` cannot be the name of a package: it is used in `use {name}/file`, so it must be a Rythme name in lower case (letters, digits, `_`)", "`{name}` 不能作为包的名字：它会在 `use {name}/file` 中使用，所以必须是小写的 Rythme 名字（字母、数字、`_`）"),
        ("`{key}` is {what}, written in quotes", "`{key}` 应该是{what}，写在引号里"),
        ("unknown key `{key}`", "未知的键 `{key}`"),
        ("the keys here are {}", "这里的键有 {}"),
        ("write `name = \"{b}\"`", "请写 `name = \"{b}\"`"),
        ("`{v}` is not a version", "`{v}` 不是版本号"),
        ("a version is three numbers: `version = \"1.0.0\"` (major.minor.patch)", "版本号由三个数字组成：`version = \"1.0.0\"`（主版本.次版本.修订号）"),
        ("`entry` is the path of a `.ry` file in the project", "`entry` 是项目中一个 `.ry` 文件的路径"),
        ("like `entry = \"src/main.ry\"`", "例如 `entry = \"src/main.ry\"`"),
        ("unknown section `{n}`", "未知的节 `{n}`"),
        ("write `name`, `version` and `entry` at the top of the file, without a section", "把 `name`、`version` 和 `entry` 写在文件开头，不放在任何节中"),
        ("the only section is `[dependencies]`", "唯一的节是 `[dependencies]`"),
        ("`{shown}` has no `name`", "`{shown}` 没有 `name`"),
        ("add a line like `name = \"hello\"` at the top", "在开头加上一行，例如 `name = \"hello\"`"),
        ("`{shown}` has no `version`", "`{shown}` 没有 `version`"),
        ("add a line like `version = \"0.1.0\"` at the top", "在开头加上一行，例如 `version = \"0.1.0\"`"),
        ("write `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (or `commit = \"...\"`), or `{name} = {{ path = \"../folder\" }}`", "请写 `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}`（或 `commit = \"...\"`），或者 `{name} = {{ path = \"../folder\" }}`"),
        ("call it `{b}`", "把它命名为 `{b}`"),
        ("the project `{own}` cannot depend on itself", "项目 `{own}` 不能依赖它自己"),
        ("`use {own}/file` already reaches its own files", "`use {own}/file` 已经可以访问它自己的文件"),
        ("the dependency `{name}` is written as a table", "依赖 `{name}` 写成了表格"),
        ("the dependency `{name}` has both `git` and `path`", "依赖 `{name}` 同时有 `git` 和 `path`"),
        ("the dependency `{name}` needs `git` or `path`", "依赖 `{name}` 需要 `git` 或 `path`"),
        ("`tag` and `commit` are for a dependency from git, not from a folder", "`tag` 和 `commit` 用于来自 git 的依赖，而不是来自文件夹的依赖"),
        ("`{c}` is not a commit", "`{c}` 不是提交"),
        ("give a `tag` or a `commit`, not both", "请给出 `tag` 或 `commit`，不要两个都给"),
        ("the dependency `{name}` needs a fixed version: a `tag` or a `commit`", "依赖 `{name}` 需要固定的版本：`tag` 或 `commit`"),
        ("the `git` address of `{name}` is empty", "`{name}` 的 `git` 地址是空的"),
        ("this file is written by `rythme install`: delete it and run `rythme install` again", "这个文件由 `rythme install` 生成：请删除它，然后重新运行 `rythme install`"),
        ("unexpected line in `rythme.lock`", "`rythme.lock` 中有意外的行"),
        ("this entry of `rythme.lock` is not complete", "`rythme.lock` 的这个条目不完整"),
        (
            r#"# The exact commit of each dependency from git, written by `rythme install`.
# Keep this file with the project, so that everyone gets the same code.
"#,
            r#"# 每个来自 git 的依赖的确切提交，由 `rythme install` 写入。
# 请把这个文件和项目放在一起，让每个人都得到相同的代码。
"#,
        ),
        // src/parser.rs
        ("all the definitions of `{name}` must have the same number of parameters", "`{name}` 的所有定义必须有相同数量的参数"),
        ("{} parameter(s) here", "这里有 {} 个参数"),
        ("expected {expected}, found {found}", "期望 {expected}，却遇到了 {found}"),
        ("expected {expected} here", "这里需要 {expected}"),
        ("to compare two values, use `==`", "要比较两个值，请使用 `==`"),
        ("use `->` in `match` cases; a dict is built with `dict([[key, value], ...])`", "在 `match` 的情况中使用 `->`；字典用 `dict([[key, value], ...])` 创建"),
        ("use `and` to combine conditions", "用 `and` 连接条件"),
        ("use `or` to combine conditions", "用 `or` 连接条件"),
        ("use `not` to negate a condition: `not done`", "用 `not` 对条件取反：`not done`"),
        ("Rythme does not use braces for blocks: write `do` ... `end`, or put a function body on the following lines and finish with `end`", "Rythme 不用花括号表示代码块：请写 `do` ... `end`，或者把函数体写在后面几行并以 `end` 结束"),
        ("Rythme does not use `:` to start a block; see `rythme help syntax`", "Rythme 不用 `:` 开始代码块；参见 `rythme help syntax`"),
        ("`{word}` is not used in Rythme", "Rythme 中不使用 `{word}`"),
        ("comes from another language", "来自其他语言"),
        ("unexpected here: {found}", "这里不应出现：{found}"),
        ("the statement should end before this", "语句应该在这之前结束"),
        ("to call a function, use parentheses: `{name}(...)`", "要调用函数，请使用括号：`{name}(...)`"),
        ("a block `do |x| ... end` is given to a call, on its line: `names.each do |name| print(name) end`", "代码块 `do |x| ... end` 要在调用所在的那一行传给调用：`names.each do |name| print(name) end`"),
        ("`else` must follow `if condition then value` (an `if` branch holds one expression; use `do ... end` for several lines)", "`else` 必须跟在 `if 条件 then 值` 后面（`if` 的分支只包含一个表达式；多行请用 `do ... end`）"),
        ("put each statement on its own line", "每条语句单独占一行"),
        ("`{word}` can only be used at the top level of a file", "`{word}` 只能在文件的最外层使用"),
        ("move it outside of the function or block", "把它移到函数或代码块外面"),
        ("before `=` comes a name or a pattern, like `[first, ..rest]`, `{{name, ..}}` or `Some(x)`", "`=` 前面是一个名字或一个模式，例如 `[first, ..rest]`、`{{name, ..}}` 或 `Some(x)`"),
        ("`=`", "`=`"),
        ("values cannot be changed in Rythme", "在 Rythme 中值不能被修改"),
        ("a record pattern cannot give a name to the other fields", "记录模式不能给其他字段命名"),
        ("write `{{name, ..}}` to ignore the other fields; for a record without some fields, use `remove(record, \"name\")`", "写 `{{name, ..}}` 来忽略其他字段；如果想要去掉某些字段的记录，请使用 `remove(record, \"name\")`"),
        ("only a function can have a name that ends with `?`", "只有函数的名字才能以 `?` 结尾"),
        ("define it with `fn`: `fn {s}(x) = ...`; for a value, use a name without `?`", "用 `fn` 定义它：`fn {s}(x) = ...`；如果是值，请使用不带 `?` 的名字"),
        ("invalid `..` in this pattern", "这个模式中的 `..` 无效"),
        ("`..name` must be last: `[first, ..rest]`", "`..name` 必须放在最后：`[first, ..rest]`"),
        ("cannot give a value to this", "不能给这个赋值"),
        ("this is not a name", "这不是一个名字"),
        ("`=` gives a name to a value", "`=` 给一个值起名字"),
        ("to define a function, write `fn name(x) = ...`", "要定义函数，请写 `fn name(x) = ...`"),
        ("records cannot be changed; build a new one: `new = {{..old, {name}: value}}`", "记录不能被修改；请创建一个新的：`new = {{..old, {name}: value}}`"),
        ("values cannot be changed; build a new one: `set_at(list, i, x)` for a list, `put(record, name, x)` for a record", "值不能被修改；请创建一个新的：列表用 `set_at(list, i, x)`，记录用 `put(record, name, x)`"),
        ("a function name", "函数名"),
        ("`(` and the parameters", "`(` 和参数"),
        ("even without parameters, write the parentheses: `fn greet() = print(\"hi\")`", "即使没有参数，也要写括号：`fn greet() = print(\"hi\")`"),
        ("functions use `=`, not `{arrow}`", "函数使用 `=`，而不是 `{arrow}`"),
        ("write `fn(x) = x * 2`", "请写 `fn(x) = x * 2`"),
        ("`=` or a new line", "`=` 或换行"),
        (
            r#"a function is written `fn name(x) = expression`,
or with its body on the following lines, finished by `end`"#,
            r#"函数写作 `fn name(x) = 表达式`，
或者把函数体写在后面几行，并以 `end` 结束"#,
        ),
        ("type names start with an uppercase letter", "类型名以大写字母开头"),
        ("write `type {cap} = ...`", "请写 `type {cap} = ...`"),
        ("a type name", "类型名"),
        ("example: `type Shape = Circle(radius) | Square(side)`", "例如：`type Shape = Circle(radius) | Square(side)`"),
        ("variant names start with an uppercase letter", "变体名以大写字母开头"),
        ("write `{cap}`", "请写 `{cap}`"),
        ("a variant name, like `Circle`", "变体名，例如 `Circle`"),
        ("a field name", "字段名"),
        ("fields are named in lowercase: `Circle(radius)`", "字段名用小写：`Circle(radius)`"),
        ("the name of the test, as a text", "测试的名字，写成文本"),
        ("example: `test \"addition works\" = assert_eq(1 + 1, 2)`", "例如：`test \"addition works\" = assert_eq(1 + 1, 2)`"),
        ("write the name of the file without `.ry`", "写文件名时不要带 `.ry`"),
        ("example: `use {path}`", "例如：`use {path}`"),
        ("a name for the file, in lowercase", "文件的名字，用小写"),
        ("example: `use geometry as geo`, then `geo.area(2)`", "例如：`use geometry as geo`，然后 `geo.area(2)`"),
        ("the name of a file", "文件的名字"),
        ("example: `use geometry` for the file `geometry.ry`, then `geometry.area(2)`", "例如：对文件 `geometry.ry` 写 `use geometry`，然后 `geometry.area(2)`"),
        ("for a file whose name starts with an uppercase letter, write its path as a text: `use \"Shapes.ry\" as shapes`", "如果文件名以大写字母开头，请把它的路径写成文本：`use \"Shapes.ry\" as shapes`"),
        ("the end of the expression", "表达式的结尾"),
        ("comparisons cannot be chained", "比较不能连写"),
        ("combine them with `and`: `a < b and b < c`", "用 `and` 把它们连起来：`a < b and b < c`"),
        ("`{sym}` cannot follow a `|>` step directly", "`{sym}` 不能直接跟在 `|>` 步骤后面"),
        (
            r#"`|>` comes after the calculations: put the pipeline in parentheses,
like `(x |> f) {sym} 1`, or continue it with a function: `x |> f |> fn(v) = v {sym} 1`"#,
            r#"`|>` 在计算之后进行：请把管道放在括号里，
例如 `(x |> f) {sym} 1`，或者用一个函数继续它：`x |> f |> fn(v) = v {sym} 1`"#,
        ),
        ("a block must follow a call", "代码块必须跟在调用后面"),
        ("not given to a function", "没有传给任何函数"),
        (
            r#"write the block on the line of the call it is given to: `xs.map do |x| x * 2 end`;
for a function as a value, write `fn(x) = x * 2`"#,
            r#"把代码块写在它所传给的调用的那一行：`xs.map do |x| x * 2 end`；
如果要把函数当作值，请写 `fn(x) = x * 2`"#,
        ),
        ("`,` or `|`", "`,` 或 `|`"),
        ("the parameters of a block go between bars: `do |name, age| ... end`", "代码块的参数写在两条竖线之间：`do |name, age| ... end`"),
        ("`}}`", "`}}`"),
        ("the block starts here", "代码块从这里开始"),
        ("a field or function name after `.`", "`.` 后面的字段名或函数名"),
        ("an expression", "一个表达式"),
        ("`]`", "`]`"),
        ("`,` or `{closing}`", "`,` 或 `{closing}`"),
        ("separate the elements with commas", "用逗号分隔元素"),
        ("this code is nested too deeply (more than {MAX_NESTING} levels)", "这段代码嵌套太深（超过 {MAX_NESTING} 层）"),
        ("split it into several named values or functions", "把它拆成几个有名字的值或函数"),
        ("this expression is too long or nested too deeply (more than {MAX_DEPTH} levels)", "这个表达式太长或嵌套太深（超过 {MAX_DEPTH} 层）"),
        (
            r#"split it into several named values or functions;
to combine many values, put them in a list: `sum([a, b, c])`, `join([x, y, z])`"#,
            r#"把它拆成几个有名字的值或函数；
要组合许多值，请把它们放进列表：`sum([a, b, c])`、`join([x, y, z])`"#,
        ),
        ("Rythme has no tuples", "Rythme 没有元组"),
        ("group values in a list `[a, b]` or a record `{{x: a, y: b}}`", "把值放进列表 `[a, b]` 或记录 `{{x: a, y: b}}` 中"),
        ("`)`", "`)`"),
        ("a named function cannot be used inside an expression", "有名字的函数不能在表达式内部使用"),
        ("define it on its own line, or use an anonymous function: `fn(x) = x * 2`", "把它定义在单独的一行，或者使用匿名函数：`fn(x) = x * 2`"),
        ("comments start with `#`", "注释以 `#` 开头"),
        ("`else` must follow `if condition then value`; a branch holds a single expression (use `do ... end` for several lines)", "`else` 必须跟在 `if 条件 then 值` 后面；一个分支只包含一个表达式（多行请用 `do ... end`）"),
        ("a record is written `{{name: \"Ada\", age: 36}}`; for a block of code, use `do ... end`", "记录写作 `{{name: \"Ada\", age: 36}}`；代码块请用 `do ... end`"),
        ("`:` and a value", "`:` 和一个值"),
        ("`then`", "`then`"),
        ("an `{word}` is written: `{word} condition then value else other_value`", "`{word}` 的写法是：`{word} 条件 then 值 else 另一个值`"),
        ("this `{word}` has no `else`", "这个 `{word}` 没有 `else`"),
        ("every `{word}` must have an `else`", "每个 `{word}` 都必须有 `else`"),
        ("expected `else` here", "这里需要 `else`"),
        ("write `else if`", "请写 `else if`"),
        (
            r#"in Rythme, `unless` always gives a value, so it needs both cases:
  unless age >= 18 then "minor" else "adult""#,
            r#"在 Rythme 中，`unless` 总是给出一个值，所以两种情况都需要：
  unless age >= 18 then "minor" else "adult""#,
        ),
        (
            r#"in Rythme, `if` always gives a value, so it needs both cases:
  if age >= 18 then "adult" else "minor""#,
            r#"在 Rythme 中，`if` 总是给出一个值，所以两种情况都需要：
  if age >= 18 then "adult" else "minor""#,
        ),
        ("a branch holds a single expression; for several lines, use `do ... end`", "一个分支只包含一个表达式；多行请用 `do ... end`"),
        ("a new line", "换行"),
        (
            r#"write each case on its own line, then `end`:
  match x
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"每种情况单独写一行，最后写 `end`：
  match x
    0 -> "zero"
    _ -> "other"
  end"#,
        ),
        ("no `|` is needed before a case", "情况前面不需要 `|`"),
        ("write each case as `pattern -> result`", "每种情况写成 `模式 -> 结果`"),
        ("`->`", "`->`"),
        ("each case is written `pattern -> result`", "每种情况写作 `模式 -> 结果`"),
        ("put each case on its own line", "每种情况单独写一行"),
        ("a `match` needs at least one case", "`match` 至少需要一种情况"),
        ("a number after `..`", "`..` 后面的数字"),
        ("only a text can come before `+` in a pattern", "在模式中，`+` 前面只能是文本"),
        ("to look at the end of a text, use a guard: `s if ends_with(s, \"!\") -> ...`", "要查看文本的结尾，请使用守卫条件：`s if ends_with(s, \"!\") -> ...`"),
        ("after `+`, a pattern takes a name or `_`", "在 `+` 后面，模式接受一个名字或 `_`"),
        ("a pattern cannot contain `{{...}}`", "模式中不能包含 `{{...}}`"),
        ("use a guard instead: `s if s == \"...\" -> ...`", "请改用守卫条件：`s if s == \"...\" -> ...`"),
        ("`..rest` must be the last element: `[first, ..rest]`", "`..rest` 必须是最后一个元素：`[first, ..rest]`"),
        ("`:` and a pattern", "`:` 和一个模式"),
        ("a pattern", "一个模式"),
        ("patterns look like: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`", "模式的样子有：`x`、`_`、`0`、`\"text\"`、`:ok`、`Some(x)`、`[first, ..rest]`、`{{name}}`"),
        ("the ends of a range in a pattern are whole numbers of 64 bits at most", "模式中范围的两端是最多 64 位的整数"),
        ("too large for a range", "对范围来说太大"),
        ("compare with a guard instead: `n if n >= low and n <= high -> ...`; a range still matches large numbers between its ends, and a decimal end like `1.0e30` works too", "请改用守卫条件比较：`n if n >= low and n <= high -> ...`；范围仍然能匹配两端之间的大数，像 `1.0e30` 这样的小数端点也可以"),
        ("a range in a pattern goes from a number to a number", "模式中的范围是从一个数到另一个数"),
        ("not a number", "不是数字"),
        ("the arguments of a call without parentheses are separated by commas: `print a, b`", "不带括号的调用，参数之间用逗号分隔：`print a, b`"),
        ("write the numbers themselves, like `1..9` or `0.0..0.5`; to compare with names, use a guard: `n if n >= low and n <= high -> ...`", "请直接写数字，例如 `1..9` 或 `0.0..0.5`；如果要和名字比较，请使用守卫条件：`n if n >= low and n <= high -> ...`"),
        ("a text that starts with a text is written `\"Mr \" + name`: `name` is the rest of the text", "以某个文本开头的文本写作 `\"Mr \" + name`：`name` 是文本的剩余部分"),
        ("missing `end` to close this `{what}`", "缺少用来闭合这个 `{what}` 的 `end`"),
        ("this `{what}` is never closed", "这个 `{what}` 一直没有闭合"),
        ("the code ends here", "代码在这里结束"),
        ("add `end` on its own line after the last line of the block", "在代码块最后一行的后面单独写一行 `end`"),
        ("values never change in Rythme: give the new value a new name, like `total2 = total + 1`", "在 Rythme 中值永远不会改变：给新值起一个新名字，例如 `total2 = total + 1`"),
        // src/project.rs
        ("no folder to keep the packages in", "没有存放包的文件夹"),
        ("set the variable RYTHME_HOME to a folder, like `RYTHME_HOME=~/.rythme`", "把变量 RYTHME_HOME 设置为一个文件夹，例如 `RYTHME_HOME=~/.rythme`"),
        ("the folder `{p}`", "文件夹 `{p}`"),
        ("run `rythme install` here, in the folder of the project", "在这里，也就是项目的文件夹中，运行 `rythme install`"),
        ("run `rythme install` in `{root}`, the folder of the project", "在项目的文件夹 `{root}` 中运行 `rythme install`"),
        ("the folder `{p}` of the package `{name}` does not exist", "包 `{name}` 的文件夹 `{p}` 不存在"),
        ("fix its `path` in `rythme.toml`", "修正 `rythme.toml` 中它的 `path`"),
        ("the package `{name}` is not installed", "包 `{name}` 没有安装"),
        ("`rythme.toml` asks for it from {}", "`rythme.toml` 要求从 {} 获取它"),
        ("`{shown_dir}` is not a Rythme package: it has no `rythme.toml`", "`{shown_dir}` 不是 Rythme 包：它没有 `rythme.toml`"),
        ("use one of its files: `use {package}/file`", "使用它的某个文件：`use {package}/file`"),
        ("the package `{package}` has no file `{entry}`", "包 `{package}` 中没有文件 `{entry}`"),
        ("the package `{package}` has no file `{wanted}`", "包 `{package}` 中没有文件 `{wanted}`"),
        ("did you mean `use {prefix}{best}`?", "你是想写 `use {prefix}{best}` 吗？"),
        ("its files there are: {}", "它在那里的文件有：{}"),
        ("did you mean `use {best}/{rest}`?", "你是想写 `use {best}/{rest}` 吗？"),
        ("in this project, `use` also reaches the files of the packages {}", "在这个项目中，`use` 也可以访问这些包的文件：{}"),
        ("the program `git` was not found", "找不到程序 `git`"),
        ("Rythme uses git to download the dependencies that come from git", "Rythme 使用 git 下载来自 git 的依赖"),
        ("install Git (https://git-scm.com), then run `rythme install` again; dependencies with `path` do not need it", "请安装 Git（https://git-scm.com），然后重新运行 `rythme install`；使用 `path` 的依赖不需要它"),
        ("git says: {said}", "git 的输出：{said}"),
        ("the tag `{t}` of `{name}` does not exist in `{url}`", "`{name}` 的标签 `{t}` 在 `{url}` 中不存在"),
        ("the commit `{c}` of `{name}` does not exist in `{url}`", "`{name}` 的提交 `{c}` 在 `{url}` 中不存在"),
        ("did you mean `tag = \"{best}\"`?", "你是想写 `tag = \"{best}\"` 吗？"),
        ("its tags are: {}", "它的标签有：{}"),
        ("it has no tags: give a `commit` instead", "它没有标签：请改为给出 `commit`"),
        ("cannot write in `{}`: {e}", "无法写入 `{}`：{e}"),
        ("cannot download `{name}` from `{url}`", "无法从 `{url}` 下载 `{name}`"),
        ("cannot take the commit `{commit}` of `{name}`", "无法获取 `{name}` 的提交 `{commit}`"),
        ("no `rythme.toml` in this folder or above it", "这个文件夹及其上级文件夹中都没有 `rythme.toml`"),
        ("go to the folder of a project, or create one with `rythme new NAME`", "请进入某个项目的文件夹，或者用 `rythme new NAME` 创建一个"),
        (
            r#"`rythme install` takes no argument (`{a}`): it installs the dependencies of `rythme.toml`
see `rythme help projects`"#,
            r#"`rythme install` 不接受参数（`{a}`）：它会安装 `rythme.toml` 中的依赖
请参阅 `rythme help projects`"#,
        ),
        ("`{name}` has no dependencies: add them under `[dependencies]` in `rythme.toml`", "`{name}` 没有依赖：请在 `rythme.toml` 的 `[dependencies]` 下添加"),
        ("Installing the dependencies of `{name}`", "正在安装 `{name}` 的依赖"),
        ("the folder `{p}` of the package `{dep_name}` does not exist", "包 `{dep_name}` 的文件夹 `{p}` 不存在"),
        ("folder {p}", "文件夹 {p}"),
        ("downloaded", "已下载"),
        ("already there", "已存在"),
        ("`{dep_name}` is not a Rythme package: `{shown_dir}` has no `rythme.toml`", "`{dep_name}` 不是 Rythme 包：`{shown_dir}` 没有 `rythme.toml`"),
        ("a package is a project made by `rythme new`: its `rythme.toml`, and its files in `src/`", "包就是用 `rythme new` 创建的项目：包括它的 `rythme.toml` 和 `src/` 中的文件"),
        ("  ✓ {dep_name} {version}: {line}", "  ✓ {dep_name} {version}：{line}"),
        ("cannot write `{}`: {e}", "无法写入 `{}`：{e}"),
        ("{count} package(s) ready", "{count} 个包已就绪"),
        ("{count} package(s) ready ({more} needed by the other ones)", "{count} 个包已就绪（其中 {more} 个是其他包需要的）"),
        ("which name? Example: rythme new hello", "什么名字？例如：rythme new hello"),
        ("`rythme new` takes one name, like `rythme new hello`", "`rythme new` 只接受一个名字，例如 `rythme new hello`"),
        ("try `rythme new {b}`", "试试 `rythme new {b}`"),
        ("`{shown_dir}` already exists", "`{shown_dir}` 已经存在"),
        ("choose another name, or another folder", "换一个名字，或者换一个文件夹"),
        ("Created the project `{name}`:", "已创建项目 `{name}`："),
        ("Next: `cd {shown_dir}`, then `rythme run` to run it and `rythme test` to test it (see `rythme help projects`)", "下一步：`cd {shown_dir}`，然后用 `rythme run` 运行它，用 `rythme test` 测试它（参见 `rythme help projects`）"),
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
            r#"# 项目：在这个文件夹中使用 `rythme run`、`rythme test` 和 `rythme check`。
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# 来自 git，指定一个标签或一个提交，然后运行 `rythme install`：
# mathlib = {{ git = "https://github.com/someone/mathlib", tag = "v1.0.0" }}
# 来自一个文件夹：
# tools = {{ path = "../tools" }}
"#,
        ),
        (
            r#"# The program: `rythme run` in the folder of the project.
use {module}

print({module}.{func}("World"))
"#,
            r#"# 程序：在项目文件夹中运行 `rythme run`。
use {module}

print({module}.{func}("世界"))
"#,
        ),
        (
            r#"## A greeting for `name`.
fn {func}(name) = "Hello, {{name}}!"
"#,
            r#"## 向 `name` 问好。
fn {func}(name) = "你好，{{name}}！"
"#,
        ),
        (
            r#"# The tests: `rythme test` in the folder of the project.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "Hello, Ada!")
"#,
            r#"# 测试：在项目文件夹中运行 `rythme test`。
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "你好，Ada！")
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

一个 [Rythme](https://github.com/maxlestage/Rythme) 项目。

```sh
rythme run        # 运行程序（src/main.ry）
rythme test       # 运行测试（tests/）
rythme check      # 不运行，找出问题
rythme fmt        # 把代码整理成官方格式
rythme install    # 下载 rythme.toml 中的依赖
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
            r#"# git 不保存的文件（请保留 rythme.lock）。
.DS_Store
Thumbs.db
*.swp
*~
"#,
        ),
        ("src/greeting.ry", "src/greeting.ry"),
        ("tests/greeting_test.ry", "tests/greeting_test.ry"),
        // src/repl.rs
        (":help for help, :quit to leave", "输入 :help 查看帮助，:quit 退出"),
        ("(type :quit to leave)", "（输入 :quit 退出）"),
        ("(cancelled)", "（已取消）"),
        ("cannot read what is typed: {e}", "无法读取输入：{e}"),
        ("usage: :doc NAME (a function, a topic...)", "用法：:doc NAME（一个函数、一个主题……）"),
        ("usage: :load file.ry", "用法：:load file.ry"),
        ("{} loaded", "已加载 {}"),
        ("Everything was forgotten.", "所有内容都已清除。"),
        ("Messages are now in English.", "消息现在使用简体中文。"),
        ("usage: :lang fr  (or another of {codes})", "用法：:lang zh  （或 {codes} 中的另一个）"),
        ("unknown code; codes look like E003 or W001", "未知代码；代码的样子是 E003 或 W001"),
        ("usage: :steps EXPRESSION, like :steps fib(4)", "用法：:steps 表达式，例如 :steps fib(4)"),
        ("Nothing defined yet.", "还没有定义任何东西。"),
        ("unknown command `{other}`", "未知命令 `{other}`"),
        (" — did you mean `{best}`?", " — 你是想输入 `{best}` 吗？"),
        (":help lists the commands", ":help 会列出所有命令"),
        // src/steps.rs
        ("  ... (more steps, not shown: {n} are shown at most, change it with --max)", "  ...（更多步骤未显示：最多显示 {n} 步，可以用 --max 修改）"),
        ("this calculation became too deep to be shown step by step (more than {MAX_DEPTH} operations inside each other)", "这个计算太深了，无法一步一步显示（超过 {MAX_DEPTH} 层嵌套的运算）"),
        ("try it with smaller values, or run the program normally: `rythme file.ry`", "试试更小的值，或者正常运行程序：`rythme file.ry`"),
        ("the evaluation was stopped after {limit} steps: does it ever end?", "求值在 {limit} 步后被停止：它会结束吗？"),
        ("a function that calls itself must always reach a case where it stops", "调用自身的函数必须总能到达停止的情况"),
        ("Nothing to show: this file only has definitions. Give an expression to compute, like: rythme steps file.ry \"fib(5)\"", "没有可显示的内容：这个文件只有定义。请给出一个要计算的表达式，例如：rythme steps file.ry \"fib(5)\""),
        ("`{flag}` needs a number, like `--max 50` or `--width 80` (20 at least)", "`{flag}` 需要一个数字，例如 `--max 50` 或 `--width 80`（至少 20）"),
        (
            r#"usage: rythme steps file.ry [expression] [--max N] [--width N]
example: rythme steps fibonacci.ry "fib(4)""#,
            r#"用法：rythme steps file.ry [表达式] [--max N] [--width N]
例如：rythme steps fibonacci.ry "fib(4)""#,
        ),
        // src/suggest.rs
        ("to join two lists or two texts, use `+`: `[1, 2] + [3]`; for a list of lists, use `flatten`", "要连接两个列表或两个文本，请使用 `+`：`[1, 2] + [3]`；对于由列表组成的列表，请使用 `flatten`"),
        ("in Rythme, this is called `{rythme}`", "在 Rythme 中，它叫 `{rythme}`"),
        ("booleans are written in lowercase: `true`", "布尔值用小写：`true`"),
        ("booleans are written in lowercase: `false`", "布尔值用小写：`false`"),
        ("Rythme has no null: use `None` for \"no value\" and `Some(x)` for a value", "Rythme 没有 null：用 `None` 表示“没有值”，用 `Some(x)` 表示有值"),
        ("in Rythme, this is called `Some`", "在 Rythme 中，它叫 `Some`"),
        ("in Rythme, an error value is written `Err(...)`", "在 Rythme 中，错误值写作 `Err(...)`"),
        ("in Rythme, a success value is written `Ok(...)`", "在 Rythme 中，成功值写作 `Ok(...)`"),
        ("Rythme has no objects: functions receive their data as arguments, like `area(shape)` or `shape.area()`", "Rythme 没有对象：函数通过参数接收数据，例如 `area(shape)` 或 `shape.area()`"),
        ("write a range with `..`: `1..10` is the list from 1 to 10 (included), and `0..10 |> step_by(2)` counts by 2", "范围用 `..` 来写：`1..10` 是从 1 到 10 的列表（包括 10），`0..10 |> step_by(2)` 则以 2 为步长计数"),
        ("use the operator `//` for a whole division: `7 // 2` is 3", "整数除法请使用运算符 `//`：`7 // 2` 等于 3"),
        ("use the operator `%` for the remainder: `7 % 3` is 1", "求余数请使用运算符 `%`：`7 % 3` 等于 1"),
        ("use the operators `//` and `%`: `7 // 2` is 3 and `7 % 2` is 1", "请使用运算符 `//` 和 `%`：`7 // 2` 等于 3，`7 % 2` 等于 1"),
        ("to put values in a text, use braces: \"total: {{x}}\"; to write a number with 2 decimals: `text(x, 2)`", "要把值放进文本，请使用花括号：\"total: {{x}}\"；要把数字写成 2 位小数：`text(x, 2)`"),
        ("read the whole file with `read_file(path)`, then cut it with `lines(text)`; the lines of the standard input are `input_lines()`", "用 `read_file(path)` 读取整个文件，再用 `lines(text)` 切分；标准输入的各行是 `input_lines()`"),
        ("to merge two records, write `{{..a, ..b}}`: the fields of `b` replace those of `a`; for two dicts or two sets, `union(a, b)`", "要合并两个记录，请写 `{{..a, ..b}}`：`b` 的字段会替换 `a` 的字段；对于两个字典或两个集合，请用 `union(a, b)`"),
        ("write `unwrap_or(get(x, key), default)`: the value, or the default if it is missing", "请写 `unwrap_or(get(x, key), default)`：得到这个值，如果缺失则得到默认值"),
        ("pair the lists, then combine each pair: `zip(a, b) |> map(fn([x, y]) = x + y)`", "先把列表配对，再组合每一对：`zip(a, b) |> map(fn([x, y]) = x + y)`"),
        ("sort from the largest to the smallest with `sort(list) |> reverse`", "用 `sort(list) |> reverse` 从大到小排序"),
        ("compute the average with `sum(list) / len(list)`", "用 `sum(list) / len(list)` 计算平均值"),
        ("keep x between low and high with `max(low, min(x, high))`", "用 `max(low, min(x, high))` 把 x 限制在 low 和 high 之间"),
        ("count how many times each value appears with `group_by(list, fn(x) = x) |> map_values(len)`", "用 `group_by(list, fn(x) = x) |> map_values(len)` 统计每个值出现的次数"),
        ("a decimal number x is NaN (not a number) when `x != x`", "当 `x != x` 时，小数 x 是 NaN（不是数字）"),
        ("`n.times do |i| ... end` is written `(0..n - 1).each do |i| ... end`", "`n.times do |i| ... end` 写作 `(0..n - 1).each do |i| ... end`"),
        ("to use another file, write `use other_file`, then `other_file.name`", "要使用另一个文件，请写 `use other_file`，然后 `other_file.name`"),
        ("there is no `new`: build values directly, like `{{name: \"Ada\"}}` or `Circle(2.0)`", "没有 `new`：直接构造值，例如 `{{name: \"Ada\"}}` 或 `Circle(2.0)`"),
        ("values never change in Rythme: build a new value instead", "在 Rythme 中值永远不会改变：请改为构造一个新值"),
        (
            r#"Rythme has no loops. Work on the whole list instead:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
or write a recursive function. See `rythme help functional`."#,
            r#"Rythme 没有循环。请改为处理整个列表：
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
或者编写一个递归函数。参见 `rythme help functional`。"#,
        ),
        ("Rythme has no loops to interrupt: use `find`, `take`, `filter`, or a recursive function", "Rythme 没有可以中断的循环：请使用 `find`、`take`、`filter` 或递归函数"),
        (
            r#"no `return` needed: the last expression of a function is its result.
To stop early with an error, return `Err(...)` and use `?`."#,
            r#"不需要 `return`：函数的最后一个表达式就是它的结果。
如果要因错误而提前停止，请返回 `Err(...)` 并使用 `?`。"#,
        ),
        ("just write `name = value` (values never change, so no keyword is needed)", "直接写 `name = value` 就行（值永远不会改变，所以不需要关键字）"),
        ("functions are written `fn name(x) = x * 2`, and anonymous ones `fn(x) = x * 2`", "函数写作 `fn name(x) = x * 2`，匿名函数写作 `fn(x) = x * 2`"),
        (
            r#"Rythme has no classes. Describe your data with `type`:
  type Shape = Circle(radius) | Square(side)
or use a record: {{name: "Ada", age: 36}}"#,
            r#"Rythme 没有类。请用 `type` 描述你的数据：
  type Shape = Circle(radius) | Square(side)
或者使用记录：{{name: "Ada", age: 36}}"#,
        ),
        (
            r#"use `match`:
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"请使用 `match`：
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
        ),
        (
            r#"errors are values in Rythme: return `Ok(value)` or `Err(message)`,
then handle them with `match` or pass them on with `?`. `fail(message)` stops the program."#,
            r#"在 Rythme 中错误也是值：返回 `Ok(value)` 或 `Err(message)`，
然后用 `match` 处理它们，或者用 `?` 把它们传递出去。`fail(message)` 会停止程序。"#,
        ),
        // src/testing.rs
        (".", "."),
        ("the test returned `false`", "测试返回了 `false`"),
        ("use `assert_eq(actual, expected)` to see the values", "用 `assert_eq(actual, expected)` 查看这些值"),
        ("the test ended with {}", "测试以 {} 结束"),
        ("a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`", "测试在没有错误地结束、并且它的值不是 `false`、`Err(...)` 或 `None` 时通过"),
        (
            r#"No tests found. Write one like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"没有找到测试。可以这样写一个：
  test "addition" = assert_eq(1 + 1, 2)"#,
        ),
        ("Running {n} test(s)", "正在运行 {n} 个测试"),
        ("Running {n} of {total} test(s)", "正在运行 {n} 个测试（共 {total} 个）"),
        ("Running {n} test(s) of {file}", "正在运行 {file} 的 {n} 个测试"),
        ("Running {n} of {total} test(s) of {file}", "正在运行 {file} 的 {n} 个测试（共 {total} 个）"),
        ("{f} file(s), ", "{f} 个文件， "),
        ("{total} test(s): {passed} passed, {failed} failed", "{total} 个测试：{passed} 个通过，{failed} 个失败"),
        (", {left_out} left out by the filter", "，{left_out} 个被过滤掉"),
        (", {broken} file(s) with errors", "，{broken} 个文件有错误"),
        ("No test name contains {}.", "没有测试的名字包含 {}。"),
        (" or ", " 或 "),
        ("{red}✗{reset} {}: the file has errors, so its tests could not run", "{red}✗{reset} {}：文件有错误，所以它的测试无法运行"),
        (
            r#"No file with tests in {}. Write a test in a `.ry` file like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"{} 中没有包含测试的文件。可以在一个 `.ry` 文件中这样写一个测试：
  test "addition" = assert_eq(1 + 1, 2)"#,
        ),
        ("Failed tests:", "失败的测试："),
        ("`{name}` needs {what}", "`{name}` 需要{what}"),
        ("a part of the name of a test", "测试名字的一部分"),
        ("a number, or `random`", "一个数字，或者 `random`"),
        ("the seed must be a whole number of 0 or more, or `random`, not `{v}`", "种子必须是大于或等于 0 的整数，或者 `random`，不能是 `{v}`"),
        ("unknown option `{name}` for `rythme test`", "`rythme test` 的未知选项 `{name}`"),
        (
            r#"
the options are: --filter NAME, --seed N, --watch (see `rythme help tests`)"#,
            r#"
可用的选项有：--filter NAME、--seed N、--watch（参见 `rythme help tests`）"#,
        ),
        ("which file or folder? Example: rythme test my_tests.ry, or rythme test . for the files of this folder", "哪个文件或文件夹？例如：rythme test my_tests.ry，或者用 rythme test . 测试这个文件夹中的文件"),
        ("Seed: {}", "种子：{}"),
        (
            r#"
Watching {} file(s): the tests run again when one changes (Ctrl+C to stop)."#,
            r#"
正在监视 {} 个文件：其中任何一个改变时，测试都会重新运行（按 Ctrl+C 停止）。"#,
        ),
        (
            r#"
{} changed: running the tests again
"#,
            r#"
{} 已改变：重新运行测试
"#,
        ),
        // src/value.rs
        ("constructor", "构造器"),
        // playground/src/lib.rs
        ("✓ no problems found", "✓ 没有发现问题"),
        ("✓ no errors, {n} warning(s)", "✓ 没有错误，{n} 个警告"),
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
            r#"一分钟了解 RYTHME

Rythme 是一门简单的语言，其中的值永远不会改变，一切都是函数。
程序是以 .ry 结尾的文件。

  # 一条注释
  name = "World"                 # 给一个值起名字（它永远不会改变）
  print("Hello, {name}!")        # {...} 把一个值插入文本中

  fn square(x) = x * x           # 一个函数
  print(square(4))               # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # 保留偶数
    |> map(square)               # 把每个数平方
    |> sum                       # 把它们加起来
    |> print                     # 20

运行：      rythme hello.ry
检查：      rythme check hello.ry
试验代码：  rythme          （交互模式）

Rythme 一直在帮助你：每个错误都会说明它在哪里、为什么出错，以及如何修正。
`rythme explain E003` 会详细解释一个错误代码。"#,
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

Tests and other files
  test "double works" = assert_eq(double(2), 4)
  use geometry                   then geometry.area(2)

Comments
  # an ordinary comment
  ## a documentation comment, above a function or a type, shown by help(...)"#,
            r#"语法

值
  42   -7   1_000_000            整数（Int），任意大小
  3.14   2.0                     小数（Float）
  "hello"   "total: {x}"         文本（Text）；{...}（或 #{...}）插入一个值
  true   false                   布尔值（Bool）
  :ok   :error                   符号（Symbol）：当作值使用的名字
  [1, 2, 3]                      列表
  {name: "Ada", age: 36}         记录
  Some(3)   None                 可能缺失的值（Option）
  Ok(3)   Err("oops")            可能失败的结果（Result）
  1..5                           列表 [1, 2, 3, 4, 5]

名字
  x = 5                          给一个值起名字；它永远不会改变
  [a, b] = [1, 2]                拆开一个值
  {name, age} = person

运算符
  + - * /   //（整数除法）   %（余数）   **（幂）
  == != < <= > >=   and   or   not
  x |> f(y)                      等同于 f(x, y)：从左到右的步骤
  x.f(y)                         也等同于 f(x, y)
  x.f                            x 的字段 f；如果 x 没有这个字段，则是 f(x)：xs.sum
  xs                             链式调用可以继续，每行以 . 开头
    .select(even?)
    .sum

函数
  fn double(x) = x * 2           写在一行，用 =
  fn greet(name)                 写在多行，以 `end` 结束
    message = "Hello {name}"
    print(message)               最后一行给出结果
  end
  fn(x) = x + 1                  匿名函数
  fn adult?(p) = p.age >= 18     名字可以以 ? 结尾（参见 rythme help ruby）

代码块：作为调用的最后一个参数传入的函数
  xs.map do |x| x * 2 end        等同于 map(xs, fn(x) = x * 2)
  xs.each do |x|                 写在多行，以 `end` 结束
    print(x)
  end
  xs.select { |x| x > 2 }        用花括号

条件（每个 if 都有 else）
  if x > 0 then "positive" else if x == 0 then "zero" else "negative"
  unless x > 0 then "small" else "big"     if 的反面

根据形状选择
  match list
    [] -> "empty"
    [x] -> "one element: {x}"
    [x, ..rest] -> "{x}, then {len(rest)} more"
  end

表达式中的多个步骤
  do
    a = 1
    a + 1
  end

你自己的类型
  type Shape = Circle(radius) | Rect(width, height)

测试和其他文件
  test "double works" = assert_eq(double(2), 4)
  use geometry                   然后 geometry.area(2)

注释
  # 普通注释
  ## 文档注释，写在函数或类型上面，由 help(...) 显示"#,
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
            r#"函数

定义和调用
  fn add(a, b) = a + b
  add(2, 3)                      # 5

写在多行：最后一个表达式就是结果
  fn describe(n)
    kind = if n % 2 == 0 then "even" else "odd"
    "{n} is {kind}"
  end

每种情况一个定义（按顺序尝试）
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

带条件（守卫）
  fn sign(n) if n < 0 = "negative"
  fn sign(0) = "zero"
  fn sign(_) = "positive"

函数也是值
  double = fn(x) = x * 2
  [1, 2, 3] |> map(double)       # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

管道从左往右读
  "  Hello  " |> trim |> upper   # "HELLO"
  x |> f(a, b)    表示    f(x, a, b)

方法风格
  "hello".upper()                # 等同于 upper("hello")

文档
  ## 把两个数相加。
  fn add(a, b) = a + b
  help(add)                      # 显示“把两个数相加。”

没有上限的递归
  当递归调用是函数做的最后一件事时，它可以重复数百万次：
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
            r#"列表

  numbers = [3, 1, 2]
  numbers[0]                     # 3（第一个；-1 是最后一个）
  len(numbers)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numbers]                 # [0, 3, 1, 2]   （.. 插入整个列表）
  numbers + [9]                  # [3, 1, 2, 9]

处理整个列表（不需要循环）
  map(numbers, fn(x) = x * 10)            # [30, 10, 20]
  filter(numbers, fn(x) = x > 1)          # [3, 2]
  fold(numbers, 0, fn(acc, x) = acc + x)  # 6
  sort(numbers)                           # [1, 2, 3]
  numbers |> each(fn(x) = print(x))       # 打印每个元素

安全访问（永远不会出错）
  first([])                      # None
  get(numbers, 10)               # None

“修改”列表：你会得到一个新列表，旧列表保持不变
  set_at(numbers, 0, 9)          # [9, 1, 2]
  insert_at(numbers, 1, 7)       # [3, 7, 1, 2]
  remove_at(numbers, -1)         # [3, 1]

列表的片段
  take_while(numbers, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

用模式拆开列表
  match numbers
    [] -> "empty"
    [first, ..rest] -> "{first}, then {len(rest)} more"
  end

所有函数：rythme help builtins"#,
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
            r#"文本

  name = "Ada"
  "Hello, {name}!"               # "Hello, Ada!"  （{...} 可以插入任何表达式）
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3（字符数，带重音的字母也算一个）
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hello", "ell")       # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14"（保留 2 位小数的数字）
  pad_left("7", 3, "0")          # "007"  （还有 pad_right：用来对齐列）
  char_code("A")   char(66)      # 65 和 "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...)：不会崩溃的转换
  trim_start("  x")   trim_end("x  ")
  words("Hi, you!")              # ["Hi", "you"]
  equals_ignore_case("Ada", "ADA")          # true
  format_number(1234.5, 2)       # "1,234.50"   （用 "fr"："1 234,50"）
  find_all("3 cats, 12 dogs", "[0-9]+")     # ["3", "12"]  （正则表达式：rythme help regex）

特殊字符：\n（换行）、\t（制表符）、\r（回车）、\0（空字符）、\"（引号）、
\\（反斜杠）、\{ 和 \}（花括号，因为 { 会开始一个插入）、
\u{e9}（用 Unicode 编号表示的任意字符：é）。"#,
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
            r#"记录

记录把有名字的值组合在一起：
  ada = {name: "Ada", born: 1815}
  ada.name                       # "Ada"
  ada["name"]                    # "Ada"

记录永远不会改变：从旧记录构造一个新记录
  older = {..ada, born: 1816}    # 相同的字段，替换了 `born`

拆开
  {name, born} = ada
  fn greet({name}) = "Hello {name}"

有用的函数
  keys(ada)   values(ada)   has(ada, "name")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["name", "Ada"], ["born", 1815]]
  from_entries([["x", 1]])       # {x: 1}

把记录当作字典
  counts = {apple: 2}
  put(counts, "pear", 5)         # {apple: 2, pear: 5}
  map_values(counts, fn(n) = n * 10)      # {apple: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}
  如果键不是名字（数字、列表……），请使用字典：rythme help dicts

JSON，网络上的数据格式
  parse_json(text)               # Ok(...)：对象变成记录，数组变成列表
  to_json(value)                 # 文本，可以直接用 write_file 保存"#,
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
            r#"字典和集合

字典（dict）为每个键给出一个值。键可以是除函数以外的任何值：数字、文本、
列表、记录……它们会保持顺序。
  ages = dict([["Ada", 36], ["Alan", 41]])
  ages                           # dict("Ada" => 36, "Alan" => 41)
  ages["Ada"]                    # 36（如果键不存在则出错）
  get(ages, "Bob")               # None：一个 Option，永远不会出错
  grid = dict([[[0, 0], "start"]])   # 用列表作为键
  dict({x: 1})                   # 从记录创建：dict("x" => 1)

字典永远不会改变：函数会很快地给出一个新字典（O(log n)）
  put(ages, "Bob", 20)           # 添加（或替换）
  remove(ages, "Ada")            # 去掉这个键
  has(ages, "Ada")   len(ages)   keys(ages)   values(ages)   entries(ages)
  map_values(ages, fn(n) = n + 1)
  filter(ages, fn(name, age) = age > 40)   # f 接收键和值
  ages.each do |name, age| print(name, age) end   # reject 和 select 也一样
  union(a, b)                    # 两者合并；两边都有的键取 b 的值

集合保存不重复的值，并保持顺序。
  s = set([3, 1, 3])             # set(1, 3)
  has(s, 3)   push(s, 5)   remove(s, 1)   len(s)   values(s)   # values：一个列表
  union(a, b)   intersection(a, b)   difference(a, b)   is_subset(a, b)
  filter(s, is_odd)   map(s, fn(x) = x * 10)

字典和集合可以用 == 和 < 比较，打印出来是 dict(k => v, ...) 和 set(a, b, ...)。
`to_json` 会把键全是文本（或全是符号）的字典写成对象，把集合写成数组；
`dict(parse_json(text)?)` 可以把对象读回来。"#,
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
            r#"你自己的类型

类型列出一个值可能的形状：
  type Shape = Circle(radius) | Rect(width, height) | Empty

  c = Circle(2.0)
  c.radius                       # 2.0

用 match 处理每种形状
  fn area(shape) = match shape
    Circle(r) -> pi * r * r
    Rect(w, h) -> w * h
    Empty -> 0
  end

或者每种形状一个定义
  fn perimeter(Circle(r)) = 2 * pi * r
  fn perimeter(Rect(w, h)) = 2 * (w + h)
  fn perimeter(Empty) = 0

如果你漏掉了某种形状，`rythme check` 会告诉你是哪一种。

类型可以包含它自己（树……）：
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
            r#"MATCH 和模式

`match` 把一个值按顺序与各个模式比较，给出第一个匹配的模式的结果：

  fn describe(x) = match x
    0 -> "zero"
    1 | 2 | 3 -> "small"
    n if n < 0 -> "negative"
    _ -> "big"
  end

模式
  42   "text"   true             正好是这个值
  1..9   -5..-1   0.0..0.5       从第一个数到第二个数之间的数，两端都包括
  "Mr " + name                   以 "Mr " 开头的文本；name 是剩下的部分
  x                              任意值，命名为 x
  _                              任意值，忽略
  Some(x)   Circle(r)            一个构造器及其内容
  []   [a, b]   [first, ..rest]  列表，按形状匹配
  {name, age}                    带有这些字段的记录
  p if condition                 一个模式加一个条件（守卫）
  a | b                          这个模式或那个模式

凡是给出名字的地方都可以使用模式：
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
            r#"错误也是值（Option、Result、?）

Rythme 没有 null，也没有异常。可能缺失的值是 Option：Some(value) 或 None。
可能失败的操作给出 Result：Ok(value) 或 Err(message)。你不会忘记处理它们。

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

  match divide(10, 2)
    Ok(x) -> print("result: {x}")
    Err(e) -> print("error: {e}")
  end

? 运算符：取出值，或者停下来把错误传递出去
  fn average(a, b)
    x = parse_float(a)?          # 如果这是 Err(...)，average 就返回它
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

便捷工具
  unwrap_or(get(list, 5), 0)     # 这个值，或者一个默认值
  unwrap(Some(3))                # 3（遇到 None 或 Err 时停止程序）
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(person, "age"), "no age")   # 缺失时为 Err("no age")：可以直接用 ?
  and_then(x, f)                 # 再加一个可能失败的步骤
  map_err(r, fn(e) = "config: {e}")     # 把错误解释得更清楚

要停止整个程序：fail("message")。"#,
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
            r#"函数式思维

在 Rythme 中，值永远不会改变，也没有循环。这可能看起来很奇怪，但程序会变得
更容易阅读、测试和信任：同样的参数，函数总是给出同样的结果。

代替打印的循环：          1..5 |> each(fn(i) = print(i))
代替转换的循环：          map(list, f)
代替筛选的循环：          filter(list, f)
代替累加的循环：          fold(list, start, f)   或 sum、product
代替累计和：              scan(list, start, f)
代替查找的循环：          find, any, all, index_of, min_by, max_by
代替 "break"：            take_while(list, f)
代替 "while"：            递归函数

  # Python:  total = 0
  #          for x in numbers: total += x * x
  total = numbers |> map(fn(x) = x * x) |> sum

  # 用递归函数实现 "while" 循环：
  fn collatz(1, steps) = steps
  fn collatz(n, steps) =
    if n % 2 == 0 then collatz(n // 2, steps + 1) else collatz(3 * n + 1, steps + 1)
  collatz(27, 0)                 # 111

不要修改值，而是创建一个新值：
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
            r#"测试

把测试写在代码旁边：
  fn double(x) = x * 2

  test "double of 2 is 4" = assert_eq(double(2), 4)

  test "several checks"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "should stay negative")
  end

运行测试：
  rythme test my_file.ry            一个文件的测试
  rythme test .                     一个文件夹（及其子文件夹）中所有 .ry 文件的测试
  rythme test . --filter double     只运行名字包含 "double" 的测试
  rythme test . --watch             每次有文件改变时重新运行（按 Ctrl+C 停止）
  rythme test my_file.ry -- a b     让 `args()` 给出 ["a", "b"]
  rythme test                       在项目中：运行项目里带测试的文件

测试时，最外层的表达式（例如 print(...)）不会运行：先加载定义，然后运行每个
测试。测试在没有错误地结束、并且它的值不是 false、Err(...) 或 None 时通过。
摘要会给出数量和时间；所有测试都通过时退出码为 0。

属性测试：检查某个性质对许多值都成立
  test "reversing twice gives the list back" =
    check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

生成器：ints(min, max)、floats(min, max)、texts(max_length)、bools()、
lists_of(generator, max_length)、one_of(list)、records_of({x: ints(), y: texts()})。
`sample(generator)` 会显示一个生成器给出的值。当某个值让性质不成立时，它会被
简化（shrinking），并和产生它的种子一起显示：每次运行都会得到同样的值，
`--seed N` 会尝试其他值（`--seed random` 随机选一个）。"#,
        ),
        (
            r#"EXERCISES

Rythme comes with 30 graded exercises for a first course, from the first
values to recursion, lists, records, types, Option, Result and tests. Each one
is a file with a statement, hints, the functions to write and their tests.

  rythme exercise list              the exercises, from ★☆☆ (easy) to ★★★
  rythme exercise start 7           copies exercise 7 into the current folder
  rythme exercise check 07-recursion.ry
                                    runs its tests, and says what to do next

In the file, replace each fail("to do") with your code; do not change the
tests. `rythme test file.ry` runs the tests too. The exercises are also in the
folder docs/exercises of the repository of Rythme, in English and French."#,
            r#"练习

Rythme 自带 30 个分级练习，适合入门课程，内容从最初的值到递归、列表、记录、
类型、Option、Result 和测试。每个练习都是一个文件，包含题目、提示、要编写的
函数以及它们的测试。

  rythme exercise list              列出练习，难度从 ★☆☆（简单）到 ★★★
  rythme exercise start 7           把练习 7 复制到当前文件夹
  rythme exercise check 07-recursion.ry
                                    运行它的测试，并告诉你下一步做什么

在文件中，把每个 fail("to do") 替换成你的代码；不要修改测试。
`rythme test file.ry` 也会运行这些测试。这些练习也在 Rythme 仓库的
docs/exercises 文件夹中，有英文版和法文版。"#,
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
            r#"多个文件

  # geometry.ry
  ## Area of a rectangle.
  fn area(w, h) = w * h
  fn _check(x) = x >= 0          # 私有：以 `_` 开头

  # main.ry
  use geometry                    # 文件 geometry.ry，和 main.ry 在同一位置
  print(geometry.area(2, 3))      # 6

`use geometry` 通过名字提供 geometry.ry 的定义：`geometry.area`、
`geometry.Circle(1)`，在 `match` 中则是 `geometry.Circle(r)`。这样你总能
看出一个名字来自哪里。文件夹中的文件：`use shapes/circle`，然后
`circle.perimeter(1)`。换一个名字：`use geometry as geo`，然后
`geo.area(2, 3)`。以 `_` 开头的名字只在它自己的文件中可见。

路径是相对于包含 `use` 的文件所在的文件夹。每个文件只运行一次。
如果你忘了 `use` 某个文件，Rythme 会告诉你是哪个文件定义了那个名字。

旧的写法：`use "geometry.ry"`（带引号）会直接提供它的定义，例如 `area(2, 3)`；
推荐使用 `use geometry`。"#,
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
            r#"文件和系统

所有接触系统的函数都返回 Result：Ok(...)，或者带有说明出错原因的
消息的 Err(message)。

文件和文件夹（相对路径从当前文件夹开始）
  read_file("notes.txt")         # Ok("...") 或 Err("file not found: notes.txt")
  write_file("out.txt", "hi")    # Ok(())，替换原有内容
  append_file("log.txt", "done\n")
  file_exists("notes.txt")       # true 或 false
  file_info("notes.txt")         # Ok({is_folder: false, size: 11})，单位是字节
  list_folder(".")               # Ok(["notes.txt", "photos"])，按顺序排列
  create_folder("backup/2024")   # Ok(())，同时创建缺少的上级文件夹
  delete_file("old.txt")         # Ok(())；从不删除文件夹

在 Linux、macOS 和 Windows 上都能用的路径
  join_path(["data", "notes.txt"])       # "data/notes.txt"
  split_path("/home/ada/notes.txt")      # ["/", "home", "ada", "notes.txt"]
  split_path("C:\Users\ada")             # ["C:\", "Users", "ada"]

命令行：rythme tool.ry --verbose --name=Ada notes.txt
  args()                         # ["--verbose", "--name=Ada", "notes.txt"]
  options()                      # Ok({args: ["notes.txt"], verbose: true, name: "Ada"})
  opts = unwrap(options())
  name = unwrap_or(get(opts, "name"), "World")   # 一个可能缺失的选项
  env("HOME")                    # Some("/home/ada") 或 None

标准输入：rythme count.ry < names.txt
  input("Name? ")                # 一行
  input_lines()                  # Ok(["Ada", "Alan"])：直到结尾的所有行

退出码，供运行你的程序的脚本使用
  0    一切正常（程序结束时也是如此）
  1    程序中有错误：语法、检查、运行时错误、测试失败
  2    命令用错了（未知命令或选项）
  101  Rythme 的内部错误（一个缺陷，请报告）
  exit(code)                     # 立即以这个代码停止，范围是 0 到 255

在浏览器中的在线试玩（playground）里，没有文件也没有输入：这些函数会给出
Err("... is not available in the browser ...")。"#,
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
are kept in ~/.rythme/packages (or RYTHME_HOME/packages)."#,
            r#"项目和包

  rythme new hello        # 创建一个文件夹 hello/，其中是一个新项目：
                          #   rythme.toml、src/main.ry、src/greeting.ry、
                          #   tests/greeting_test.ry、README.md、.gitignore
  cd hello
  rythme run              # 运行程序：rythme.toml 中的 `entry`
  rythme test             # 运行 src/ 和 tests/ 中带测试的文件
  rythme check            # 检查所有文件；还有 `rythme fmt`、`rythme doc`

rythme.toml 描述了这个项目：

  name = "hello"
  version = "0.1.0"
  entry = "src/main.ry"

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

依赖来自 git（指定一个 `tag` 或一个 `commit`），或者来自一个文件夹。
`rythme install` 会（用 git）下载它们，并写入 rythme.lock，记录每个依赖的
确切提交：请把它和项目放在一起，让每个人都得到相同的代码。然后：

  use mathlib/vectors       # mathlib 的文件 src/vectors.ry
  use mathlib               # 它的程序，即它的 rythme.toml 中的 `entry`
  use hello/greeting        # 项目自己的一个文件，在 tests/ 中使用

与包含 `use` 的文件在同一位置的文件优先。来自 git 的包保存在
~/.rythme/packages（或 RYTHME_HOME/packages）中。"#,
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
            r#"日期和时间

日期是一个记录：{year, month, day}，表示一天中的某个时刻时还有 hour、minute
和 second，已知时差时还有 offset（比 UTC 快的分钟数）。
  today()                        # {year: 2026, month: 10, day: 18}
  now()                          # {year: 2026, ..., hour: 14, minute: 5, second: 9, offset: 120}
  birthday = {year: 1815, month: 12, day: 10}
  birthday.year                  # 1815

与 ISO 8601 格式的文本互相转换
  parse_date("2024-02-29")       # Ok({year: 2024, month: 2, day: 29})
  parse_date("2024-02-30")       # Err("... 2024年2月 ...")
  parse_date("2024-02-29T13:05:00+01:00")    # 带时间和时差
  format_date({year: 2024, month: 2, day: 29, hour: 9})   # "2024-02-29T09:00:00"

日历计算（考虑闰年和每月天数）
  add_days({year: 2024, month: 2, day: 28}, 1)     # {year: 2024, month: 2, day: 29}
  add_seconds(now(), 3600)       # 一小时后
  days_between(today(), {year: 2027, month: 1, day: 1})   # 剩余天数
  seconds_between(a, b)          # b 在 a 之前时为负数
  day_of_week(today())           # 1 = 星期一 ... 7 = 星期日
  days_in_month(2023, 2)         # 28
  is_leap_year(2000)             # true
  is_date({year: 2023, month: 2, day: 29})   # false

不存在的日期会让函数以 E023 停止，并说明原因。
本地时差来自系统（系统没有提供时为 UTC）；time() 以 Float 给出从 1970 年
开始的秒数，用来测量时长。"#,
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
            r#"正则表达式：用模式搜索

正则表达式（regex）描述文本的片段："[0-9]+" 表示“一个或多个数字”。
Rythme 能读懂常见的正则表达式，而且一次搜索的耗时不会超过
（文本长度）×（正则表达式的大小）。

  matches("2024", "[0-9]+")                  # true：整个文本都匹配
  find_first("tel: 06 12", "[0-9]+")         # Some("06")：第一个片段
  find_all("3 cats, 12 dogs", "[0-9]+")      # ["3", "12"]
  split_by("a, b;c", "[,;] *")               # ["a", "b", "c"]
  replace_all("a cat", "\\b\\w", upper)      # "A Cat"（对每个片段调用一个函数）
  replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1")   # "2024-02-29"
  captures("2024-02", "([0-9]+)-([0-9]+)")   # Some(["2024-02", "2024", "02"])
  named_captures("2024-02", "(?<year>[0-9]+)-(?<month>[0-9]+)")
                                             # Some({year: "2024", month: "02"})

正则表达式能表达什么
  abc          按这个顺序出现的这些字符
  .            除换行符以外的任意字符
  [abc]        这些字符之一；[a-z] 从 a 到 z 中的一个；[^0-9] 其他任意字符
  \d \w \s     一个数字（0-9）；一个字母（包括带重音的）、数字或 _；一个空白
  \D \W \S     其他任何东西
  \b           单词的边界
  ^  $         文本的开头和结尾
  x|y          x 或 y
  (...)        分组：用来重复它，或者捕获它的文本（$1、`captures`）
  (?:...)      不捕获任何内容的分组；(?<name>...) 是带名字的分组
  x*  x+  x?   x 出现零次或多次、一次或多次、零次或一次（尽可能长）
  x{3}  x{2,5}  x{2,}    3 次、2 到 5 次、2 次或更多
  x*?  x+?     尽可能短，而不是尽可能长
  (?i)         放在开头：大写和小写字母视为相同

在 Rythme 文本中书写正则表达式
  反斜杠要写两次："\\d+" 是正则表达式 \d+。花括号要写成 \{ 和 \}：
  "[0-9]\{4\}" 是正则表达式 [0-9]{4}。要表示符号本身，请把它放在方括号里：
  "[.]" 是一个点，"[(]" 是一个括号，"[$]" 是一个美元符号。

正则表达式中的错误会让程序以 E026 停止，并显示错误的位置；
如果正则表达式写在代码中，`rythme check` 会提前发现它。"#,
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
  :reset          forget everything      :lang fr       change the language (en, fr, es, de, pt, zh)
  :learn          guided tour            :explain E003  explain an error
  :steps fib(4)   show the calculation step by step
The Tab key completes names."#,
            r#"交互模式

不带文件运行 `rythme`。输入一个表达式，就能看到它的值：
  ry> 1 + 2
  3
定义会从一行保留到下一行。如果一行没有写完（括号没闭合、代码块没有 `end`……），
它会在下一行继续。

命令
  :help           这份帮助               :quit          退出（或 Ctrl-D）
  :doc map        某个名字的帮助         :load f.ry     加载一个文件
  :reset          清除所有内容           :lang zh       切换语言（en、fr、es、de、pt、zh）
  :learn          导览                   :explain E003  解释一个错误
  :steps fib(4)   一步一步显示计算过程
Tab 键可以补全名字。"#,
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
            r#"格式化：rythme fmt

`rythme fmt` 把代码整理成 Rythme 的官方格式：所有程序看起来都一样，
谁也不用再为空格争论。

  rythme fmt hello.ry            格式化一个文件（会改写这个文件）
  rythme fmt .                   一个文件夹及其子文件夹中的所有 .ry 文件
  rythme fmt --check .           只检查：列出需要格式化的文件，如果有
                                 则失败（退出码 1），用于 CI
  rythme fmt - < hello.ry        读取代码，输出结果

风格
  - 每级缩进 2 个空格
  - 运算符两边有空格：x = a + b * 2   xs |> map(f)   n -> n * 2
  - 括号内侧和 .. 两边没有空格：f(x)   [1, 2]   {name: "Ada"}   1..10
  - 写成多行的管道：每行一个步骤
      total = prices
        |> filter(fn(p) = p > 10)
        |> sum
  - 写成多行的类型：每行一个变体
      type Shape =
        | Circle(radius)
        | Rect(width, height)
  - 写成多行的列表以逗号结尾
  - 写在一行的代码块保持在一行：
      xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
    写成多行时，其中的语句比 `do` 深一级
  - 每行一条语句（没有 `;`），`end` 单独占一行
  - 写成多行的定义前后各有一个空行；从不出现连续两个空行
  - 所有注释都会保留；连续几行末尾的注释会对齐

在语言允许选择的地方，换行会被保留，长行也保持原样。格式化工具从不改变
程序的行为：有语法错误的文件会保持原样，并显示错误。"#,
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
            r#"文档：rythme doc

`rythme doc` 根据代码中的 `##` 注释、类型和函数定义生成文档。

  rythme doc geometry.ry         在屏幕上输出 Markdown
  rythme doc .                   一个文件夹及其子文件夹中的所有 .ry 文件
  rythme doc --html geometry.ry  一个不依赖其他任何东西的网页
  rythme doc . --out doc.html    写入一个文件（.html 会生成网页）

把 `##` 注释写在它所说明的内容的正上方：

  ## Shapes and their areas.      <- 在文件顶部，后面跟一个空行：
                                     它描述整个文件
  ## A shape to draw.
  type Shape = Circle(radius) | Rect(width, height)

  ## The area of a shape. See `len` and `area`.
  fn area(Circle(r)) = pi * r * r
  fn area(Rect(w, h)) = w * h

生成的内容
  - 每个文件：它的说明、它使用的文件（同样生成文档）、它的类型、它的函数
    （每个定义的第一行）、它的值以及它的测试名字
  - 以 `_` 开头的名字会被略去：它们只在自己的文件中使用
  - 注释中用反引号括起来的名字，例如 `area` 或 `len(list)`，会变成指向其文档
    的链接；在 `use geometry` 之后，`geometry.area` 会指向 geometry.ry 中的
    `area`；以这种方式提到的内置函数会在最后加以解释
  - 标题使用消息的语言（`--lang zh`）

在注释中，空的 `##` 行会开始一个新段落，以 `- ` 开头的行会组成一个列表。"#,
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
            r#"一步一步：rythme steps

`rythme steps` 一步一步地显示程序如何计算它的值：每次调用都被替换成它的
函数体，参数的值放在相应的位置，直到只剩下结果。

  rythme steps fib.ry              文件中的每个定义和表达式
  rythme steps fib.ry "fib(4)"     一个表达式，在文件的定义之后计算
  rythme steps fib.ry --max 50     每个表达式最多显示 50 步
  rythme steps fib.ry --width 80   截断超过 80 个字符的行

对于  fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

  fib(3)
    → fib(2) + fib(1)
    → fib(1) + fib(0) + fib(1)
    → 1 + fib(0) + fib(1)
    → 1 + 0 + fib(1)
    → 1 + fib(1)
    → 1 + 1
    → 2

一步就是一次调用或一次运算。被调用函数的函数体在显示时，已经在不调用其他
函数的前提下尽可能地化简。内置函数（以及其他文件中的函数）一步就给出结果。
程序打印的内容会出现在步骤之间。显示的步骤（默认 200 步）之后，剩下的部分
会直接计算而不显示。

在交互模式中：:steps fib(4)。在在线试玩中：Steps 按钮。
法语中：rythme etapes。"#,
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
  Settings: rythme.path (the program), rythme.language (en, fr, es, de, pt, zh or auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  every editor that speaks the Language Server Protocol: tell it to start
  `rythme lsp` for .ry files. Ready-made configurations: editors/README.md
  ("Run" and "Run test" name the commands rythme.runFile, rythme.testFile and
  rythme.runTest, that the editor carries out: see editors/README.md)

Zed
  the extension in editors/zed: colors and the language server.

Colors in Neovim, Helix and Zed
  the Tree-sitter grammar in editors/tree-sitter-rythme (see editors/README.md).

The language of the messages follows the editor, unless `--lang` (en, fr, es, de, pt, zh) or
RYTHME_LANG chooses it: rythme --lang fr lsp"#,
            r#"在你的编辑器中使用 RYTHME

Rythme 在编辑器中也能提供帮助：输入时显示问题，鼠标悬停时显示函数的文档
（带有根据代码推断出的签名，例如 `fn area(side: Int|Float) -> Int|Float`），
自动补全，调用的参数提示，跳转到定义，查找引用，重命名，快速修正拼写错误，
像 `rythme fmt` 一样格式化，确定时显示名字的类型（以及函数的签名），
按名字的种类着色，代码上方的 "Run" 和 "Run test"，整个工作区的名字，
代码折叠。这就是语言服务器，编辑器会自动启动它：
  rythme lsp

VS Code
  editors/vscode 中的扩展：着色、代码片段和语言服务器。
  设置：rythme.path（程序），rythme.language（en、fr、es、de、pt、zh 或 auto）。

Neovim、Helix、Zed、Emacs、Sublime Text、Kate……
  所有支持 Language Server Protocol 的编辑器：让它为 .ry 文件启动
  `rythme lsp`。现成的配置：editors/README.md
  （"Run" 和 "Run test" 对应命令 rythme.runFile、rythme.testFile 和
  rythme.runTest，由编辑器执行：参见 editors/README.md）

Zed
  editors/zed 中的扩展：着色和语言服务器。

Neovim、Helix 和 Zed 中的着色
  editors/tree-sitter-rythme 中的 Tree-sitter 语法（参见 editors/README.md）。

消息的语言跟随编辑器，除非用 `--lang`（en、fr、es、de、pt、zh）或
RYTHME_LANG 指定：rythme --lang zh lsp"#,
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
            r#"从 RUBY 转过来

Rythme 读起来像 Ruby，但它的值永远不会改变：没有修改，没有 nil，没有异常。
你在 Ruby 中学到的大部分东西都能用：

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

和 Ruby 一样的地方
  调用            puts "hi"   print x, y   assert_eq total, 42
                  （在语句或结果的开头；见下文）
  代码块          xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
                  代码块就是函数：等同于 fn(x) = x * 2
  方法调用        xs.sum   name.upcase   xs.first(2)   xs.include?(3)
  带 ? 的名字     even?   empty?   include?(x)   fn adult?(p) = ...
  符号            :ok   :error（用 == 比较，在 match 中使用）
  文本            "Hello #{name}"（或 "Hello {name}"）；"\#{" 写出 #{
  unless          unless cond then a else b
  Ruby 名字       puts to_s to_i to_f length size upcase downcase strip
                  select reject reduce inject uniq each_slice each_cons
                  each_with_index include? empty? even? odd? any? all?
                  start_with? end_with? key? lstrip rstrip casecmp?
                  （参见 rythme help builtins）

不同的地方
  括号            表达式内部的调用需要括号：x = len(xs)，
                  而不是 x = len xs
  没有修改        没有 x += 1：给新值起一个新名字，total2 = total + 1
  没有 nil        缺失的值是 None，存在的值是 Some(x)
  没有异常        错误也是值，Ok(x) 或 Err(message)，用 ? 传递出去
  if 和 unless    总是有 else：它们给出一个值；没有 `return`，
                  函数的最后一行就是它的结果
  没有类          记录 {name: "Ada"} 和类型：type Shape = Circle(r) | Square(s)
  没有循环        each、map、select、reduce，或者调用自身的函数
  case/when       match value ... end，每行一种情况：模式 -> 结果
  哈希            dict([[:ada, 36]])（任何种类的键，包括符号），或者
                  记录 {name: "Ada", age: 36}

不带括号的调用
  如果一条语句以一个名字开头，同一行后面跟着一个值的开头（数字、文本、符号、
  名字、构造器、not、if、match、fn），它就是一次调用：print x, y 就是
  print(x, y)。参数后面的 do 代码块会传给这次调用：each xs do |x| ... end。
  和 Ruby 一样，空格后面的 [ 或 -（- 后面没有空格）也会开始参数
  （puts [1, 2]、puts -1），除非代码给这个名字赋了值（用 =、作为参数或在
  模式中）：这时 x -1 就是 x - 1，xs [0] 就是 xs[0]，和以前一样。
  `rythme fmt` 会把它们写成 x - 1 和 xs[0]。

  结果也可以这样开始：if 或 unless 的分支、match 的情况、函数或测试的 =
  后面的部分。它的参数在 else、end、}、) 或行尾结束：

  fn greet(name) = puts "Hello, #{name}!"
  fn check(ok) = if ok then puts "yes" else puts "no"
  match name
    "Dr " + rest -> puts "doctor", rest
    _ -> puts name
  end
  xs.each { |x| puts x }

x.name 是怎么工作的
  x.name 读取记录的字段 `name`；如果 x 没有这个字段，就用 x 调用函数 `name`：
  xs.sum 就是 sum(xs)，xs.first(2) 就是 first(xs, 2)。链式调用可以跨越
  多行，每行以 `.` 开头。

什么时候 ? 是名字的一部分
  even?(4)、xs.include?(3)、fn adult?(p)：紧跟在名字后面、在 `(` 前面。
  不带 `(` 的 even?：当存在这样的函数时（内置的，或在文件中用 fn 定义的）。
  否则 value? 中的 ? 是遇到 None 或 Err 就停止的 ?，而 (value)? 总是如此。"#,
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
            r#"E001 — 语法错误

这里的代码不符合 Rythme 的语法。消息会说明这里需要什么。常见原因：
  - 缺少 `)`、`]` 或 `,`
  - `if` 中缺少 `then` 或 `else`
  - 用花括号 { } 表示代码块（Rythme 使用换行和 `end`）
  - 使用了其他语言的关键字（for、while、return、let、def……）

错误：   if x > 0 { print(x) }
正确：   if x > 0 then print(x) else print("not positive")"#,
        ),
        (
            r#"E002 — Text never closed

A text starts with " and must end with " on the same text. A `{` inside a
text starts an insertion and must be closed by `}`.

Wrong:   print("hello)
Right:   print("hello")
To write a brace in a text: "\{not inserted\}""#,
            r#"E002 — 文本没有闭合

文本以 " 开始，并且必须在同一个文本中以 " 结束。文本中的 `{` 会开始一个
插入，必须用 `}` 闭合。

错误：   print("hello)
正确：   print("hello")
在文本中写花括号："\{not inserted\}""#,
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
            r#"E003 — 未知名字

这个名字在这里没有定义。请检查拼写（大写和小写字母是不同的），以及这个名字
是否在使用之前就已经定义。在函数内部，参数和上面定义的名字都是可见的，
一个接一个写在一起的局部函数也能互相看到；在最外层，所有函数在任何地方
都可见。

错误：   print(lenght([1, 2]))
正确：   print(len([1, 2]))

如果有最接近的已有名字，Rythme 会建议它；如果名字定义在同一文件夹的另一个
文件中，Rythme 会告诉你是哪个文件：

  use geometry                   # 然后
  print(geometry.area(2))        # 而不是 area(2)

以 `_` 开头的名字是它所在文件的私有名字：其他文件不能使用它。"#,
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
            r#"E004 — 参数个数不对

调用函数时给的值比它需要的多或少。请记住，使用 `|>` 时，左边的值是第一个
参数。

错误：   fn add(a, b) = a + b
         add(1)
正确：   add(1, 2)
也正确：  1 |> add(2)

`rythme help NAME` 会显示如何调用一个内置函数。"#,
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
            r#"E005 — 值的类型不对

这个操作不能用于这种类型的值。Rythme 从不悄悄地转换值："1" + 1 是错误，
而不是 "11" 或 2。

错误：   "age: " + 36
正确：   "age: {36}"              （把值插入文本中）
正确：   "age: " + text(36)       （先转换它）

条件必须是 true 或 false：
错误：   if len(list) then ... else ...
正确：   if len(list) > 0 then ... else ...

当值的类型确定时，`rythme check` 会在程序运行之前发现这个错误，即使是在
不会每次都运行的代码中，以及在给函数传入它无法使用的值的调用中：
`area(side)` 计算 `side * side` 时的 `area("big")`。"#,
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
            r#"E006 — 没有匹配的情况

`match` 的所有情况（或者函数的所有定义，或者像 `[a, b] = list` 这样的定义
中的模式）都不接受这个值。

错误：   fn name(1) = "one"
         name(2)
正确：   fn name(1) = "one"
         fn name(_) = "another number"

`rythme check` 会在运行之前对遗漏的情况发出警告（W002）。当值的类型确定时，
它还会在运行之前指出永远无法匹配的情况：对 Text 使用 `[first, ..rest]`，
对数字使用 `Circle(r)`。"#,
        ),
        (
            r#"E007 — Index out of range

Positions in a list or a text go from 0 to length - 1 (and -1 is the last).

Wrong:   [10, 20][2]
Right:   [10, 20][1]              # 20
Safe:    get([10, 20], 2)         # None, never an error"#,
            r#"E007 — 索引超出范围

列表或文本中的位置是从 0 到长度 - 1（-1 表示最后一个）。

错误：   [10, 20][2]
正确：   [10, 20][1]              # 20
安全：   get([10, 20], 2)         # None，永远不会出错"#,
        ),
        (
            r#"E008 — Division by zero

Dividing by zero has no meaning. Check the divisor before, or return a Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
            r#"E008 — 除以零

除以零没有意义。请先检查除数，或者返回一个 Result。

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
        ),
        (
            r#"E009 — Not a function

Parentheses after a value call it, but only functions can be called.

Wrong:   x = 5
         x(3)
Wrong:   None()                   (None is a value)
Right:   None
To multiply, write it:  2 * (x + 1)"#,
            r#"E009 — 不是函数

值后面的括号表示调用它，但只有函数才能被调用。

错误：   x = 5
         x(3)
错误：   None()                   （None 是一个值）
正确：   None
要做乘法，请写出来：  2 * (x + 1)"#,
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
            r#"E010 — 未知字段

记录（或构造器）没有这个名字的字段；或者字典没有这个键（`dict[key]`）。

错误：   {name: "Ada"}.nmae
正确：   {name: "Ada"}.name
安全：   get(record, "name")     # 一个 Option，永远不会出错
         get(dict, key)          # 字典也一样

当记录的字段确定时，`rythme check` 会在程序运行之前发现这个错误，并建议
最接近的字段。"#,
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
            r#"E011 — 已经定义过了

在 Rythme 中，一个名字总是表示同一个值：它不能被修改，也不能在同一个地方
定义两次。正是这一点让程序容易理解。

错误：   x = 1
         x = x + 1
正确：   x = 1
         y = x + 1

对于有多种情况的函数，请把各个定义一个接一个地写在一起：
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

给被使用的文件起的名字（`use geometry`、`use ... as geo`）不能再在文件中
表示别的东西；用引号使用的两个文件（`use "a.ry"`）不能定义同一个名字：
请改为通过名字使用它们（`use a`，然后 `a.area`）。"#,
        ),
        (
            r#"E012 — Number too large (no longer given)

Whole numbers (Int) have no limit any more: they grow as needed, and stay
exact. Earlier versions of Rythme stopped with this error beyond 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (a Float is approximate)

Only a whole number of more than 100000 digits stops the program, with
E023: it is usually a calculation that grows without end."#,
            r#"E012 — 数字太大（已不再出现）

整数（Int）不再有上限：它们会根据需要增长，并且保持精确。Rythme 的早期版本
在超过 64 位时会因这个错误而停止。

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30（Float 是近似值）

只有超过 100000 位的整数才会让程序停止，错误是 E023：这通常是一个无限增长
的计算。"#,
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
            r#"E013 — unwrap 没有找到值

`unwrap(x)` 从 Some(...) 或 Ok(...) 中取出值，遇到 None 或 Err(...) 时
会停止程序。请改为处理值缺失的情况：

  match get(list, 5)
    Some(x) -> x
    None -> 0
  end
或者简单地写：  unwrap_or(get(list, 5), 0)"#,
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
            r#"E014 — 嵌套调用太多

一个函数调用自己（或其他函数）的次数太多，一直没有结束。要么它永远到达不了
基本情况，要么需要把它写成让递归调用成为它做的最后一件事（尾调用）：这样的
调用没有上限。

有限制：     fn sum([]) = 0
             fn sum([x, ..rest]) = x + sum(rest)       # + 在调用之后进行
无限制：     fn sum([], acc) = acc
             fn sum([x, ..rest], acc) = sum(rest, acc + x)
最简单：     sum(list)  或  fold(list, 0, fn(a, x) = a + x)"#,
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
            r#"E015 — 断言失败

`assert(condition)` 或 `assert_eq(actual, expected)` 发现有东西和期望的
不一样。在测试中，这意味着测试失败：消息会显示要比较的值。

`rythme check` 会在运行之前发现永远不可能通过的 `assert_eq`，因为它的两个值
属于永远不会相等的类型：
错误：   assert_eq(parse_int("3"), 3)      # Result 永远不是 Int
正确：   assert_eq(parse_int("3"), Ok(3))

当某个值让性质不成立时，`check_all(property, generator)` 也会报告 E015：
它会显示找到的最简单的值，以及产生它的种子（每次运行都会得到同样的值；
`rythme test --seed N` 会尝试其他值）。"#,
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
            r#"E016 — ? 用错了

`x?` 从 Some(...) 或 Ok(...) 中取出值；遇到 None 或 Err(...) 时，它会让当前
函数立即返回这个值。所以它只能在函数内部、对 Option 或 Result 使用。

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
            r#"E017 — 文件问题

无法读取某个文件，或者找不到传给 `use` 的文件，或者文件之间循环使用。`use`
中的路径是相对于包含 `use` 的文件所在的文件夹。程序是以 UTF-8 保存的文本文件
（编辑器通常的选择）。

当没有更多内容可读时，`input()` 也会因这个错误而停止：键盘输入已关闭
（Ctrl-D），或者发送给程序的文本（`rythme prog.ry < answers.txt`）没有更多行了。"#,
        ),
        (
            r#"E018 — Unexpected character

This character has no meaning in Rythme here. Texts use straight double
quotes "...", powers use **, and comments start with #."#,
            r#"E018 — 意外的字符

这个字符在 Rythme 的这个位置没有意义。文本使用英文直双引号 "..."，幂运算
使用 **，注释以 # 开头。"#,
        ),
        (
            r#"E019 — if without else

In Rythme, `if` is an expression: it always gives a value, so both cases are
needed. Each branch holds a single expression; use `do ... end` for several.

Wrong:   if x > 0 then print("positive")
Right:   if x > 0 then print("positive") else ()
Right:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
            r#"E019 — if 没有 else

在 Rythme 中，`if` 是一个表达式：它总是给出一个值，所以两种情况都需要。每个
分支只包含一个表达式；多个请用 `do ... end`。

错误：   if x > 0 then print("positive")
正确：   if x > 0 then print("positive") else ()
正确：   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
        ),
        (
            r#"E020 — Missing end

A function written on several lines, a `match` or a `do` block must finish
with `end` on its own line.

  fn greet(name)
    print("Hello {name}")
  end"#,
            r#"E020 — 缺少 end

写成多行的函数、`match` 或 `do` 代码块必须以单独一行的 `end` 结束。

  fn greet(name)
    print("Hello {name}")
  end"#,
        ),
        (
            r#"E021 — Program stopped by fail

The program called `fail(message)`, which stops everything. For errors that
the caller could handle, return Err(message) instead."#,
            r#"E021 — 程序被 fail 停止

程序调用了 `fail(message)`，它会停止一切。对于调用者可以处理的错误，请改为
返回 Err(message)。"#,
        ),
        (
            r#"E022 — Impossible conversion

The text does not contain a number. When the text comes from a user or a
file, use the safe versions, which give a Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
            r#"E022 — 无法转换

文本中不包含数字。当文本来自用户或文件时，请使用安全的版本，它们会给出
Result：

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
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
            r#"E023 — 无效的值

函数收到的值类型正确，但超出了它接受的范围：负数的个数、空列表的最小值、
负数的平方根、不存在的日期（2 月 30 日）、太大的范围、列表或文本、超过
100000 位的整数……消息会说明限制是什么。"#,
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
            r#"E024 — 一个值丢失了

某一行计算了一个值，但没有任何东西使用它：只有代码块的最后一行会给出它的值，
而值永远不会改变，所以这个结果就丢失了。

错误：   total = price
         - discount              # 新的一行：这是一条单独的语句
正确：   total = price -
           discount              # 用运算符结束这一行，才能接着写
错误：   x + 1                   # 不会改变 x
正确：   y = x + 1
要显示一个值，请打印它：  print(x + 1)"#,
        ),
        (
            r#"E025 — Project or package problem

The `rythme.toml` of the project (or the `rythme.lock` written by
`rythme install`) cannot be read, a dependency is not installed, or
`rythme install` cannot download one. A `rythme.toml` looks like this:

  name = "hello"                 # a Rythme name in lower case
  version = "0.1.0"              # three numbers
  entry = "src/main.ry"          # the program (this one by default)

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

Every value is a text, in quotes. A dependency from git needs a fixed
version, a `tag` or a `commit`, and the program `git`
(https://git-scm.com). After a change of `[dependencies]`, run
`rythme install` in the folder of the project. More: `rythme help projects`."#,
            r#"E025 — 项目或包的问题

无法读取项目的 `rythme.toml`（或者由 `rythme install` 写入的 `rythme.lock`），
某个依赖没有安装，或者 `rythme install` 无法下载某个依赖。`rythme.toml` 的
样子是这样的：

  name = "hello"                 # 小写的 Rythme 名字
  version = "0.1.0"              # 三个数字
  entry = "src/main.ry"          # 程序（默认是这个）

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

每个值都是写在引号里的文本。来自 git 的依赖需要固定的版本（`tag` 或
`commit`），还需要程序 `git`（https://git-scm.com）。修改 `[dependencies]`
之后，请在项目文件夹中运行 `rythme install`。更多信息：`rythme help projects`。"#,
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
            r#"E026 — 无效的正则表达式

正则表达式（传给 `matches`、`find_all`、`split_by`、`replace_all` 等的
regex）违反了它的某条规则，或者 `replace_all` 的替换文本使用了不存在的分组。
消息会显示问题在哪里。

错误：   find_all(text, "([0-9]+")       # `(` 一直没有闭合
正确：   find_all(text, "([0-9]+)")
错误：   split_by(text, "*")             # `*` 没有可重复的内容
正确：   split_by(text, "[*]")           # 符号 * 本身，放在方括号里
错误：   replace_all(date, "(\\d+)-(\\d+)", "$3")    # 只有 2 个分组
正确：   replace_all(date, "(\\d+)-(\\d+)", "$2/$1")

在 Rythme 文本中，反斜杠要写两次（"\\d"），花括号要写成 \{ 和 \}
（"[0-9]\{4\}"）。`rythme help regex` 列出了正则表达式能表达的一切。
如果正则表达式写在代码中，`rythme check` 会在运行之前发现错误。"#,
        ),
        (
            r#"W001 — Unused name

A name is defined but never used. It is often a typo or a forgotten step.
If it is on purpose, start the name with `_`: `_unused = ...`."#,
            r#"W001 — 未使用的名字

一个名字被定义了，但从未使用。这通常是拼写错误或者漏掉了某个步骤。
如果是故意的，请让名字以 `_` 开头：`_unused = ...`。"#,
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
            r#"W002 — 遗漏的情况

`match`（或者有多个定义的函数）没有处理所有可能的值。消息会列出缺少了什么。
如果出现了这个值，程序会以 E006 停止。

  type Light = Red | Orange | Green
  fn next(Red) = Green
  fn next(Green) = Orange
  # W002：`next` 没有处理 `Orange`。添加缺少的定义：
  fn next(Orange) = Red"#,
        ),
        (
            r#"W003 — Built-in function hidden

You defined a name that already belongs to a built-in function. In your file,
the name now means your definition. Choose another name to keep both."#,
            r#"W003 — 遮蔽了内置函数

你定义的名字已经属于一个内置函数。在你的文件中，这个名字现在表示你的定义。
如果想两个都保留，请换一个名字。"#,
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
            r#"W004 — 永远不会执行到的情况

情况是按顺序尝试的，值会进入第一个匹配的情况。在以下情况下，一个情况（或
函数的一个定义）永远不会被用到：
  - 它前面的某个情况接受一切（`_` 或一个普通名字）：请把那个情况放在最后；
  - 它前面的某个情况已经处理了同样的值：`1 -> ...` 写了两次；
  - 值永远不可能是这种形状：Text 永远不会匹配 `[first, ..rest]` 这样的
    列表模式，数字永远不会匹配 `Some(x)`；
  - 范围是空的：`9..1` 不包含任何数字，请写 `1..9`。

  match name                     # `name` 是一个 Text
    [first, ..] -> first         # W004：Text 不是列表
    _ -> "?"
  end
要查看文本中的字符，请使用 `chars(name)`。"#,
        ),
        (
            r#"W005 — Unknown function after a dot

`value.name(args)` calls the function `name(value, args)`, or the field `name`
of a record. No function has this name, and no record of the file has such a
field: it is probably a typo.

Wrong:   [1, 2].lenght()
Right:   [1, 2].len()"#,
            r#"W005 — 点后面是未知的函数

`value.name(args)` 会调用函数 `name(value, args)`，或者记录的字段 `name`。
没有函数叫这个名字，文件中也没有记录有这样的字段：很可能是拼写错误。

错误：   [1, 2].lenght()
正确：   [1, 2].len()"#,
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
            r#"W006 — 比较总是为假

不同种类的值永远不相等：Rythme 从不悄悄地转换它们。当 `x` 是数字时，
`x == "1"` 总是为假，`x != "1"` 总是为真。这通常是忘了转换，或者把 Option
里面的值当成在外面来比较。

错误：   if count == "3" then ...            （一个数字和一个文本）
正确：   if count == 3 then ...
错误：   get(scores, "ada") == 19            （一个 Option 和一个数字）
正确：   get(scores, "ada") == Some(19)
正确：   unwrap_or(get(scores, "ada"), 0) == 19"#,
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
            r#"W007 — 永远不会失败的测试

测试只有在因错误而停止，或者以 `false`、`None` 或 `Err(...)` 结束时才会失败。
如果一个测试只是用永远不会停止的函数计算或打印一个值，却从不把它和期望值
比较，它就永远不会失败：它什么也没有检查。

错误：   test "length" = len([1, 2])
错误：   test "length" = print(len([1, 2]))
正确：   test "length" = assert_eq(len([1, 2]), 2)
正确：   test "length" = len([1, 2]) == 2"#,
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
            r#"W008 — 结果丢失了

在 Rythme 中值永远不会改变：`push(list, 4)` 不会把 4 加到 `list` 中，它会
给出一个新列表。如果一行只调用这样的函数，却不使用它给出的结果，这一行就
什么也没做。

错误：   push(numbers, 4)               # `numbers` 没有改变
正确：   more = push(numbers, 4)
错误：   name.upper()
正确：   print(name.upper())"#,
        ),
        (
            r#"W009 — A function shown instead of its value

A function shown in a text or printed appears as `<fn name>`. To show what
it gives, call it, with parentheses, even when it takes no argument.

  fn greeting() = "hello"
Wrong:   print("{greeting}")          # shows <fn greeting>
Right:   print("{greeting()}")        # shows hello"#,
            r#"W009 — 显示的是函数而不是它的值

在文本中显示或打印出来的函数会显示为 `<fn name>`。要显示它给出的结果，请
带括号调用它，即使它不接受参数。

  fn greeting() = "hello"
错误：   print("{greeting}")          # 显示 <fn greeting>
正确：   print("{greeting()}")        # 显示 hello"#,
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
            r#"W010 — 范围有重叠

范围模式 `a..b` 匹配从 `a` 到 `b` 的数字，两端都包括。当 `match`（或函数的
定义）中的两个范围有共同的数字时，这些数字总是进入第一个范围：第二个范围
永远看不到它们。这通常是某个端点写了两次。

错误：   match age
           0..18 -> "young"
           18..65 -> "adult"         # W010：18 会进入 "young"
           _ -> "senior"
         end
正确：   0..17 -> "young"
         18..64 -> "adult"
对于落在 17 和 18 之间的小数，请改为比较：
  a if a < 18 -> "young""#,
        ),
        ("Defines a function: `fn name(x) = expression`, or with its body on the following lines, finished by `end`. Without a name, `fn(x) = x * 2` is an anonymous function.", "定义一个函数：`fn name(x) = expression`，或者把函数体写在后面几行，并以 `end` 结束。没有名字的 `fn(x) = x * 2` 是匿名函数。"),
        ("Chooses between two values: `if condition then value else other`. Every `if` has an `else`, because an `if` always gives a value.", "在两个值之间选择：`if condition then value else other`。每个 `if` 都有 `else`，因为 `if` 总是给出一个值。"),
        ("Separates the condition of an `if` from the value it gives when the condition is true.", "把 `if` 的条件和条件为真时给出的值分开。"),
        ("Gives the value of an `if` when the condition is false. It is always required; `else if` chains several conditions.", "给出条件为假时 `if` 的值。它总是必需的；`else if` 可以串联多个条件。"),
        ("The opposite of `if`: `unless condition then a else b` gives `b` when the condition is true, `a` otherwise. Like `if`, it always has an `else`.", "`if` 的反面：`unless condition then a else b` 在条件为真时给出 `b`，否则给出 `a`。和 `if` 一样，它总是有 `else`。"),
        ("Chooses according to the shape of a value: one case per line, `pattern -> result`, finished by `end`. Cases are tried in order, and `rythme check` warns about forgotten cases.", "根据值的形状进行选择：每行一种情况，`pattern -> result`，以 `end` 结束。情况按顺序尝试，`rythme check` 会对遗漏的情况发出警告。"),
        ("Closes a block: a function written on several lines, a `match`, a `do` or a `test`.", "结束一个代码块：写成多行的函数、`match`、`do` 或 `test`。"),
        ("Groups several steps inside an expression, up to `end`. The last line gives the value. After a call, `do |x| ... end` is a block: a function given as the last argument.", "把表达式中的多个步骤组合在一起，直到 `end`。最后一行给出值。在调用之后，`do |x| ... end` 是一个代码块：作为最后一个参数传入的函数。"),
        ("Defines your own type and its variants. Each variant is a constructor, with or without fields.", "定义你自己的类型及其变体。每个变体都是一个构造器，可以带字段，也可以不带。"),
        ("Defines a test, next to the code. `rythme test file.ry` runs the tests of the file.", "在代码旁边定义一个测试。`rythme test file.ry` 会运行文件中的测试。"),
        ("Uses another file through its name: `use geometry`, then `geometry.area(2)`. The path is relative to the folder of the file that contains `use`.", "通过名字使用另一个文件：`use geometry`，然后 `geometry.area(2)`。路径是相对于包含 `use` 的文件所在的文件夹。"),
        ("True when both conditions are true. The second one is computed only if the first one is true.", "当两个条件都为真时为真。只有第一个条件为真时，才会计算第二个。"),
        ("True when at least one of the conditions is true. The second one is computed only if the first one is false.", "当至少一个条件为真时为真。只有第一个条件为假时，才会计算第二个。"),
        ("Inverts a condition: `not true` is `false`.", "对条件取反：`not true` 是 `false`。"),
        ("One of the two values of the type Bool.", "Bool 类型的两个值之一。"),
        ("Shows values on the screen, separated by spaces, then goes to the next line.", "在屏幕上显示值，用空格分隔，然后换行。"),
        ("Shows a value with its position in the code (on the error output) and returns it unchanged. Handy in the middle of a pipeline.", "显示一个值及其在代码中的位置（输出到错误输出），并原样返回它。在管道中间很方便。"),
        ("Converts any value to text. With a number of decimals, writes a number with exactly that many digits after the point.", "把任意值转换成文本。如果给出小数位数，会把数字写成小数点后正好有这么多位。"),
        ("Shows the prompt, then reads a line typed on the keyboard and returns it as text.", "显示提示语，然后读取键盘输入的一行，并把它作为文本返回。"),
        ("Reads all the lines of the standard input, until its end: `Ok(list of texts)`, or `Err(message)` if it cannot be read. For `rythme count.ry < names.txt`, or text typed until Ctrl-D (Ctrl-Z then Enter on Windows).", "读取标准输入的所有行，直到结尾：`Ok(list of texts)`，如果无法读取则为 `Err(message)`。用于 `rythme count.ry < names.txt`，或者一直输入到 Ctrl-D（Windows 上是 Ctrl-Z 再按回车）的文本。"),
        ("Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (for `()`), \"Symbol\" (for `:ok`), \"Option\", \"Result\", or the name of your own type.", "返回值的类型名：\"Int\"、\"Float\"、\"Text\"、\"Bool\"、\"List\"、\"Record\"、\"Function\"、\"Unit\"（表示 `()`）、\"Symbol\"（表示 `:ok`）、\"Option\"、\"Result\"，或者你自己的类型的名字。"),
        ("Shows help: about a function (`help(map)`), a topic (`help(\"lists\")`), or what you can do with a value (`help([1, 2])`).", "显示帮助：关于一个函数（`help(map)`）、一个主题（`help(\"lists\")`），或者一个值能做什么（`help([1, 2])`）。"),
        ("Stops with an error if the condition is false. Mostly used in tests.", "如果条件为假，就以错误停止。主要用于测试。"),
        ("Stops with an error showing both values if they are different. Mostly used in tests.", "如果两个值不同，就以显示这两个值的错误停止。主要用于测试。"),
        ("Stops the program with an error message. Prefer returning `Err(...)` for errors that can be handled.", "以一条错误消息停止程序。对于可以处理的错误，最好返回 `Err(...)`。"),
        ("Returns the list of the words given after the file name on the command line.", "返回命令行中文件名后面给出的单词列表。"),
        ("Reads the options of the command line (the words of `args()`, or the list given) into a record, in a Result: `--name=value` gives the text `value`, `--name` and `-n` give `true`, a `-` in a name becomes `_`, and the other words are in the field `args` (all of them after `--`). An option given twice, or a word like `-abc`, gives `Err(message)`. Use `get(opts, \"name\")` or `has(opts, \"name\")` for an option that may be missing.", "把命令行的选项（`args()` 的单词，或者给出的列表）读入一个记录，放在 Result 中：`--name=value` 给出文本 `value`，`--name` 和 `-n` 给出 `true`，名字中的 `-` 变成 `_`，其他单词放在字段 `args` 中（`--` 之后的所有单词也是）。给了两次的选项，或者像 `-abc` 这样的单词，会给出 `Err(message)`。对于可能缺失的选项，请使用 `get(opts, \"name\")` 或 `has(opts, \"name\")`。"),
        ("The value of an environment variable of the system, as an Option: `Some(text)`, or `None` if it is not set.", "系统中某个环境变量的值，作为 Option：`Some(text)`，如果没有设置则为 `None`。"),
        ("Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not; Rythme itself exits with 1 after an error in the program (see `rythme help system`).", "立即停止程序。代码（默认为 0）告诉系统一切是否正常（0）；程序中出错后，Rythme 本身以 1 退出（参见 `rythme help system`）。"),
        ("Returns the current time in seconds (a Float), counted from 1 January 1970. Subtract two times to measure a duration.", "返回当前时间，以秒为单位（一个 Float），从 1970 年 1 月 1 日开始计算。把两个时间相减可以测量时长。"),
        ("random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.", "random() 给出 0 到 1 之间的小数；random(a, b) 给出从 a 到 b（包括两端）的整数；random(list) 给出一个随机元素。"),
        ("Absolute value: the number without its sign.", "绝对值：去掉符号的数。"),
        ("The smallest of the values, or of the elements of a list.", "这些值中，或者列表的元素中最小的一个。"),
        ("The largest of the values, or of the elements of a list.", "这些值中，或者列表的元素中最大的一个。"),
        ("Square root.", "平方根。"),
        ("x to the power n (same as `x ** n`).", "x 的 n 次幂（等同于 `x ** n`）。"),
        ("Rounds down to a whole number.", "向下取整。"),
        ("Rounds up to a whole number.", "向上取整。"),
        ("Rounds to the nearest whole number, or to the given number of decimal digits.", "四舍五入到最接近的整数，或者到给定的小数位数。"),
        ("Converts to a whole number (Int): decimals are cut, texts are read.", "转换成整数（Int）：小数部分被截掉，文本会被读取。"),
        ("Converts to a decimal number (Float).", "转换成小数（Float）。"),
        ("Sine (x in radians).", "正弦（x 的单位是弧度）。"),
        ("Cosine (x in radians).", "余弦（x 的单位是弧度）。"),
        ("Tangent (x in radians).", "正切（x 的单位是弧度）。"),
        ("Arc sine: the angle (in radians) whose sine is x, for x from -1 to 1.", "反正弦：正弦为 x 的角（单位是弧度），x 的范围是 -1 到 1。"),
        ("Arc cosine: the angle (in radians) whose cosine is x, for x from -1 to 1.", "反余弦：余弦为 x 的角（单位是弧度），x 的范围是 -1 到 1。"),
        ("Arc tangent: the angle (in radians) whose tangent is y. With two numbers, the angle of the point (x, y), from -pi to pi.", "反正切：正切为 y 的角（单位是弧度）。给出两个数时，是点 (x, y) 的角，范围是 -pi 到 pi。"),
        ("Logarithm: natural (base e) by default, or in the given base.", "对数：默认是自然对数（以 e 为底），或者以给定的数为底。"),
        ("e to the power x.", "e 的 x 次幂。"),
        ("Is the whole number even?", "这个整数是偶数吗？"),
        ("Is the whole number odd?", "这个整数是奇数吗？"),
        ("Greatest common divisor of two whole numbers: the largest number that divides both.", "两个整数的最大公约数：能同时整除两者的最大的数。"),
        ("Least common multiple of two whole numbers: the smallest number divisible by both.", "两个整数的最小公倍数：能被两者同时整除的最小的数。"),
        ("Length of a text (in characters), a list, a record, a dict or a set.", "文本（按字符计）、列表、记录、字典或集合的长度。"),
        ("The text in uppercase.", "转换成大写的文本。"),
        ("The text in lowercase.", "转换成小写的文本。"),
        ("The text with its first character in uppercase (the rest is unchanged).", "第一个字符变成大写的文本（其余部分不变）。"),
        ("The text without spaces and line breaks at the start and the end.", "去掉开头和结尾的空格和换行的文本。"),
        ("Adds `fill` (a space by default) before the text until it is `width` characters long: numbers line up in columns. A value that is not a text is first shown as `print` shows it.", "在文本前面添加 `fill`（默认是空格），直到它有 `width` 个字符：数字会按列对齐。不是文本的值会先按 `print` 显示的样子转换。"),
        ("Adds `fill` (a space by default) after the text until it is `width` characters long.", "在文本后面添加 `fill`（默认是空格），直到它有 `width` 个字符。"),
        ("Cuts a text into a list of texts. Without separator, cuts at spaces.", "把文本切分成文本列表。没有分隔符时，按空白切分。"),
        ("Glues the elements of a list into one text, with the separator between them.", "把列表的元素粘成一个文本，元素之间放上分隔符。"),
        ("Does the text contain this piece of text? Does the list (or the set) contain this element? Does the record have this field, or the dict this key?", "文本中包含这段文本吗？列表（或集合）中包含这个元素吗？记录有这个字段吗，或者字典有这个键吗？"),
        ("Does the text start with this piece of text?", "文本以这段文本开头吗？"),
        ("Does the text end with this piece of text?", "文本以这段文本结尾吗？"),
        ("Replaces every occurrence of `old` by `new`.", "把每一处 `old` 替换成 `new`。"),
        ("The list of the characters of a text.", "文本的字符列表。"),
        ("The list of the lines of a text.", "文本的行列表。"),
        ("The number of a character (its Unicode code point).", "字符的编号（它的 Unicode 码位）。"),
        ("The character with this number (Unicode code point): the opposite of `char_code`.", "具有这个编号（Unicode 码位）的字符：`char_code` 的反操作。"),
        ("Is the text made only of digits from 0 to 9 (and not empty)?", "文本是否只由 0 到 9 的数字组成（并且不为空）？"),
        ("Is the text made only of letters, accented ones included (and not empty)?", "文本是否只由字母组成，包括带重音的字母（并且不为空）？"),
        ("Is the text made only of spaces, tabs and line breaks (and not empty)?", "文本是否只由空格、制表符和换行组成（并且不为空）？"),
        ("A text or a list repeated n times.", "重复 n 次的文本或列表。"),
        ("Reads a whole number from a text: `Ok(number)`, or `Err(message)` if the text is not a number.", "从文本中读取整数：`Ok(number)`，如果文本不是数字则为 `Err(message)`。"),
        ("Reads a decimal number from a text: `Ok(number)` or `Err(message)`.", "从文本中读取小数：`Ok(number)` 或 `Err(message)`。"),
        ("Reads a JSON text: `Ok(value)`, or `Err(message)` if the text is not valid JSON. Objects become records, arrays lists, and null `None`.", "读取 JSON 文本：`Ok(value)`，如果文本不是有效的 JSON 则为 `Err(message)`。对象变成记录，数组变成列表，null 变成 `None`。"),
        ("Writes a value as JSON text: records become objects, lists arrays, and `None` null. With `indent`, one element per line, shifted by that many spaces.", "把一个值写成 JSON 文本：记录变成对象，列表变成数组，`None` 变成 null。带 `indent` 时，每行一个元素，缩进这么多个空格。"),
        ("The text without spaces and line breaks at the start.", "去掉开头的空格和换行的文本。"),
        ("The text without spaces and line breaks at the end.", "去掉结尾的空格和换行的文本。"),
        ("The list of the words of a text: letters and digits, accents included, with `'` or `-` inside a word. Punctuation and spaces separate them.", "文本中的单词列表：字母和数字，包括带重音的字母，单词内部可以有 `'` 或 `-`。标点和空白把它们分开。"),
        ("Are the two texts the same when uppercase and lowercase letters count as equal?", "在大写和小写字母视为相同时，两个文本是否相同？"),
        ("Writes a number for people: thousands separated, with exactly `decimals` decimals if given, in the style of a language: \"en\" (1,234.5, by default), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" or \"pt\" (1.234,5). To line it up in a column, use `pad_left`.", "把数字写成方便人阅读的形式：千位分隔，如果给出 `decimals` 则正好保留这么多位小数，按某种语言的习惯：\"en\"（1,234.5，默认）、\"fr\"（1 234,5）、\"de\"、\"es\"、\"it\"、\"nl\" 或 \"pt\"（1.234,5）。要按列对齐，请使用 `pad_left`。"),
        ("Does the whole text match the regex (a regular expression: see `rythme help regex`)? To look for the regex anywhere in the text, use `find_first`.", "整个文本是否匹配这个正则表达式（参见 `rythme help regex`）？要在文本中的任何位置查找，请使用 `find_first`。"),
        ("The first piece of the text that matches the regex, as an Option: `Some(piece)`, or `None` if there is none.", "文本中第一个匹配正则表达式的片段，作为 Option：`Some(piece)`，如果没有则为 `None`。"),
        ("Every piece of the text that matches the regex, from left to right.", "文本中每一个匹配正则表达式的片段，从左到右。"),
        ("The first match of the regex, as an Option of a list: the whole piece, then the text of each group `(...)`, in order (\"\" for a group that matched nothing).", "正则表达式的第一个匹配，作为列表的 Option：先是整个片段，然后按顺序是每个分组 `(...)` 的文本（没有匹配到内容的分组为 \"\"）。"),
        ("Like `captures`, for every match: a list of lists.", "和 `captures` 一样，但针对每一个匹配：一个由列表组成的列表。"),
        ("The first match of a regex with named groups `(?<name>...)`, as an Option of a record: one field for each name.", "带命名分组 `(?<name>...)` 的正则表达式的第一个匹配，作为记录的 Option：每个名字一个字段。"),
        ("Cuts a text at every piece that matches the regex.", "在每个匹配正则表达式的片段处切分文本。"),
        ("Replaces every piece that matches the regex. `new` is a text, where `$1` stands for the text of group 1 (`$<name>` for a named group, `$0` for the whole piece, `$$` for a `$`), or a function that receives each piece and gives its replacement.", "替换每个匹配正则表达式的片段。`new` 是一个文本，其中 `$1` 表示第 1 个分组的文本（`$<name>` 表示命名分组，`$0` 表示整个片段，`$$` 表示一个 `$`），或者是一个函数，它接收每个片段并给出替换后的内容。"),
        ("Applies the function f to each element and returns the list of the results. Also works on Option and Result, and on a set (giving the set of the results).", "对每个元素应用函数 f，返回结果组成的列表。也适用于 Option 和 Result，以及集合（得到结果组成的集合）。"),
        ("Keeps only the elements for which f returns true. Also works on a set, and on a dict, where f receives each key and its value.", "只保留让 f 返回 true 的元素。也适用于集合，以及字典（这时 f 接收每个键和它的值）。"),
        ("Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.", "把所有元素合并成一个值：从 `start` 开始，然后对每个元素计算 f(result, element)。"),
        ("Like fold, but gives the list of every intermediate result: running totals, for example.", "和 fold 一样，但给出每个中间结果组成的列表：例如累计和。"),
        ("Calls f on each element, for its effect (like printing). Returns (). Also works on a set, and on a dict, where f receives each key and its value.", "对每个元素调用 f，为了它的效果（比如打印）。返回 ()。也适用于集合，以及字典（这时 f 接收每个键和它的值）。"),
        ("The sum of the numbers of a list; with f, the sum of f(x) for each element.", "列表中数字的和；带 f 时，是每个元素的 f(x) 的和。"),
        ("The product of the numbers of a list.", "列表中数字的积。"),
        ("The first element of a list (or character of a text), as an Option: `Some(x)`, or `None` if there is none. With n, the first n elements (like `take`).", "列表的第一个元素（或文本的第一个字符），作为 Option：`Some(x)`，如果没有则为 `None`。带 n 时，是前 n 个元素（同 `take`）。"),
        ("The last element of a list (or character of a text), as an Option. With n, the last n elements.", "列表的最后一个元素（或文本的最后一个字符），作为 Option。带 n 时，是最后 n 个元素。"),
        ("The list (or text) without its first element.", "去掉第一个元素的列表（或文本）。"),
        ("The element at a position of a list or text, the field of a record, or the value of a key of a dict, as an Option (never an error). Negative positions count from the end.", "列表或文本中某个位置的元素、记录的字段，或字典中某个键的值，作为 Option（永远不会出错）。负数位置从末尾开始数。"),
        ("A new list with x added at the end; or a new set with x added.", "在末尾添加了 x 的新列表；或者添加了 x 的新集合。"),
        ("A new list where the element at position i is replaced by x. Negative positions count from the end.", "把位置 i 的元素替换成 x 的新列表。负数位置从末尾开始数。"),
        ("A new list with x inserted so that it is at position i (-1 adds it at the end).", "插入 x 使它位于位置 i 的新列表（-1 表示添加到末尾）。"),
        ("A new list without the element at position i. Negative positions count from the end.", "去掉位置 i 的元素的新列表。负数位置从末尾开始数。"),
        ("The list or text in reverse order.", "顺序反转的列表或文本。"),
        ("The list sorted from smallest to largest (numbers, texts in alphabetical order...). For the largest first: `sort(list) |> reverse`.", "从小到大排序的列表（数字、按字母顺序排列的文本……）。如果要从大到小：`sort(list) |> reverse`。"),
        ("Sorts the list by the value that the function `key` gives for each element. Elements with the same value keep their order.", "按函数 `key` 为每个元素给出的值对列表排序。值相同的元素保持原来的顺序。"),
        ("The list without duplicates (the first occurrence is kept).", "去掉重复元素的列表（保留第一次出现的元素）。"),
        ("The elements of the list in a random order.", "随机排列的列表元素。"),
        ("The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.", "列表或文本从位置 `start`（包括）到 `end`（不包括）的部分。负数位置从末尾开始数。"),
        ("The first n elements of a list or text.", "列表或文本的前 n 个元素。"),
        ("The list or text without its first n elements.", "去掉前 n 个元素的列表或文本。"),
        ("The first elements of a list (or characters of a text), as long as f is true for them.", "列表的开头元素（或文本的开头字符），只要 f 对它们为真。"),
        ("The list (or text) without its first elements for which f is true.", "去掉开头那些让 f 为真的元素之后的列表（或文本）。"),
        ("One element out of n of a list (or text), starting with the first. On a range: the numbers with a step.", "列表（或文本）中每 n 个取一个元素，从第一个开始。对于范围：按步长取数字。"),
        ("Cuts a list (or text) into pieces of n elements; the last piece may be shorter.", "把列表（或文本）切成每段 n 个元素；最后一段可能更短。"),
        ("Every group of n neighbouring elements of a list (or text), in order: handy to compare each element with the next one.", "列表（或文本）中每组 n 个相邻的元素，按顺序排列：方便把每个元素和下一个比较。"),
        ("Pairs the elements of two lists: [[a1, b1], [a2, b2], ...]. It stops at the end of the shorter list.", "把两个列表的元素配对：[[a1, b1], [a2, b2], ...]。在较短的列表结束时停止。"),
        ("Pairs each element with its position: [[0, x0], [1, x1], ...].", "把每个元素和它的位置配对：[[0, x0], [1, x1], ...]。"),
        ("Is f true for at least one element?", "f 是否至少对一个元素为真？"),
        ("Is f true for every element?", "f 是否对每个元素都为真？"),
        ("The first element for which f is true, as an Option.", "第一个让 f 为真的元素，作为 Option。"),
        ("How many elements make f true; without f, how many elements there are.", "有多少个元素让 f 为真；不带 f 时，是元素的个数。"),
        ("The elements for which f is false: the opposite of `filter`. Also works on a set and on a dict, like `filter`.", "让 f 为假的元素：`filter` 的反面。和 `filter` 一样，也适用于集合和字典。"),
        ("Calls f(element, position) for each element, in order (positions start at 0). For effects, like `print`.", "按顺序对每个元素调用 f(element, position)（位置从 0 开始）。用于产生效果，比如 `print`。"),
        ("The position of an element in a list (or of a piece of text in a text), as an Option.", "元素在列表中的位置（或一段文本在文本中的位置），作为 Option。"),
        ("The element for which the function `key` gives the smallest value (the first one if there are several).", "函数 `key` 给出最小值的元素（如果有多个，取第一个）。"),
        ("The element for which the function `key` gives the largest value (the first one if there are several).", "函数 `key` 给出最大值的元素（如果有多个，取第一个）。"),
        ("Joins a list of lists into one list.", "把由列表组成的列表连接成一个列表。"),
        ("Like map, when f returns a list: the lists are joined.", "和 map 一样，但 f 返回列表：这些列表会被连接起来。"),
        ("Splits a list in two: [elements where f is true, elements where f is false].", "把列表分成两部分：[让 f 为真的元素，让 f 为假的元素]。"),
        ("Groups the elements in a record: the key is the text of what `key` returns, the value the list of matching elements.", "把元素分组放进一个记录：键是 `key` 返回值的文本，值是对应元素组成的列表。"),
        ("Is the list, text, record, dict or set empty?", "列表、文本、记录、字典或集合是否为空？"),
        ("The list of the field names of a record, or of the keys of a dict (in order).", "记录的字段名列表，或字典的键列表（按顺序）。"),
        ("The list of the values of a record or of a dict (in the order of its keys), or of the elements of a set (in order).", "记录或字典的值列表（按键的顺序），或集合的元素列表（按顺序）。"),
        ("The list of the [name, value] pairs of a record, or of the [key, value] pairs of a dict.", "记录的 [名字, 值] 对列表，或字典的 [键, 值] 对列表。"),
        ("Builds a record from a list of [name, value] pairs: the opposite of `entries`.", "用 [名字, 值] 对的列表构造一个记录：`entries` 的反操作。"),
        ("Does the record have this field? Does the dict have this key, or the set this element?", "记录有这个字段吗？字典有这个键吗，或者集合有这个元素吗？"),
        ("A new record with the field set to the value (added or replaced); or a new dict with the key set to the value.", "把字段设为这个值（添加或替换）的新记录；或者把键设为这个值的新字典。"),
        ("A new record without this field; or a new dict without this key, or a new set without this element.", "去掉这个字段的新记录；或者去掉这个键的新字典，或者去掉这个元素的新集合。"),
        ("A new record (or dict) where f is applied to every value; the names (or keys) stay the same.", "对每个值应用 f 的新记录（或字典）；名字（或键）保持不变。"),
        ("A dictionary: keys of any kind (but functions), each with a value, kept in the order of the keys. Made from a list of [key, value] pairs (the last value of a key wins), from a record, or empty with `dict()`. Use it with get, put, has, remove, keys, values, entries, len, map_values, filter, union...", "字典：任何种类的键（函数除外），每个键对应一个值，按键的顺序保存。可以由 [键, 值] 对的列表创建（同一个键以最后一个值为准），也可以由记录创建，或者用 `dict()` 创建空字典。可以配合 get、put、has、remove、keys、values、entries、len、map_values、filter、union 等使用。"),
        ("A set: values (but functions) without repetition, kept in order. Made from a list, or empty with `set()`. Use it with has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.", "集合：不重复的值（函数除外），按顺序保存。可以由列表创建，或者用 `set()` 创建空集合。可以配合 has、push、remove、len、values、filter、map、union、intersection、difference、is_subset 使用。"),
        ("The elements of two sets together; or the pairs of two dicts, where the value of b wins for a key in both.", "两个集合的所有元素；或者两个字典的所有键值对，两边都有的键取 b 的值。"),
        ("The elements of the set a that are also in b; or the pairs of the dict a whose key is in b.", "集合 a 中也在 b 中的元素；或者字典 a 中键在 b 中的键值对。"),
        ("The elements of the set a that are not in b; or the pairs of the dict a whose key is not in b.", "集合 a 中不在 b 中的元素；或者字典 a 中键不在 b 中的键值对。"),
        ("Is every element of the set a also in the set b?", "集合 a 的每个元素是否也都在集合 b 中？"),
        ("The value inside `Some(x)` or `Ok(x)`. Stops the program with an error for `None` or `Err(e)`.", "`Some(x)` 或 `Ok(x)` 里面的值。遇到 `None` 或 `Err(e)` 时以错误停止程序。"),
        ("The value inside `Some(x)` or `Ok(x)`, or the default value for `None` or `Err(e)`.", "`Some(x)` 或 `Ok(x)` 里面的值，遇到 `None` 或 `Err(e)` 时则为默认值。"),
        ("Is it `Some(...)`?", "是 `Some(...)` 吗？"),
        ("Is it `None`?", "是 `None` 吗？"),
        ("Is it `Ok(...)`?", "是 `Ok(...)` 吗？"),
        ("Is it `Err(...)`?", "是 `Err(...)` 吗？"),
        ("Chains steps that may fail: for `Some(x)` or `Ok(x)`, gives f(x), which must be an Option or a Result; `None` and `Err(e)` stay as they are.", "串联可能失败的步骤：对于 `Some(x)` 或 `Ok(x)`，给出 f(x)，它必须是 Option 或 Result；`None` 和 `Err(e)` 保持不变。"),
        ("Turns an Option into a Result: `Some(x)` gives `Ok(x)`, and `None` gives `Err(error)`. Handy before `?`.", "把 Option 变成 Result：`Some(x)` 给出 `Ok(x)`，`None` 给出 `Err(error)`。在 `?` 之前很方便。"),
        ("For `Err(e)`, gives `Err(f(e))`; `Ok(x)` stays as it is. Handy to explain an error better.", "对于 `Err(e)`，给出 `Err(f(e))`；`Ok(x)` 保持不变。方便把错误解释得更清楚。"),
        ("Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.", "读取整个文本文件：`Ok(text)`，如果无法读取则为 `Err(message)`。"),
        ("Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.", "把文本写入文件（替换原有内容）：`Ok(())` 或 `Err(message)`。"),
        ("Adds a text at the end of a file, creating the file if needed: `Ok(())` or `Err(message)`.", "在文件末尾添加文本，需要时创建文件：`Ok(())` 或 `Err(message)`。"),
        ("Does this file (or folder) exist? Always `false` in the browser.", "这个文件（或文件夹）存在吗？在浏览器中总是 `false`。"),
        ("What is known about a file or a folder: `Ok({is_folder, size})`, with its size in bytes (0 for a folder), or `Err(message)` if it does not exist or cannot be read.", "关于文件或文件夹的信息：`Ok({is_folder, size})`，大小以字节为单位（文件夹为 0），如果不存在或无法读取则为 `Err(message)`。"),
        ("The names of the files and folders inside a folder, in alphabetical order: `Ok(list of texts)`, or `Err(message)`. `list_folder(\".\")` lists the current folder.", "文件夹中的文件和文件夹的名字，按字母顺序排列：`Ok(list of texts)` 或 `Err(message)`。`list_folder(\".\")` 列出当前文件夹。"),
        ("Creates a folder, and the folders around it that are missing: `Ok(())`, also when it already exists, or `Err(message)`.", "创建一个文件夹，以及缺少的上级文件夹：`Ok(())`（已经存在时也是），或 `Err(message)`。"),
        ("Deletes one file (never a folder: that is an error): `Ok(())` or `Err(message)`. A deleted file cannot be brought back.", "删除一个文件（从不删除文件夹：那是错误）：`Ok(())` 或 `Err(message)`。删除的文件无法恢复。"),
        ("Joins the parts of a path with `/`, which works on Linux, macOS and Windows (with `\\` if the parts already use only `\\`). Only the first part can start at a root (`/`, `C:`): a part like \"/etc\" after another one is an error.", "用 `/` 连接路径的各个部分，在 Linux、macOS 和 Windows 上都能用（如果各部分已经只使用 `\\`，则用 `\\`）。只有第一部分可以从根开始（`/`、`C:`）：在其他部分之后出现像 \"/etc\" 这样的部分是错误。"),
        ("The parts of a path, split at each `/` or `\\`; the first part of an absolute path is its root (`/`, `C:\\`...). `join_path` puts them back together.", "路径的各个部分，在每个 `/` 或 `\\` 处切分；绝对路径的第一部分是它的根（`/`、`C:\\`……）。`join_path` 可以把它们重新组合起来。"),
        ("The date and time now, where the program runs: a record {year, month, day, hour, minute, second, offset}, where `offset` is the number of minutes the local time is ahead of UTC (0 when the system does not tell).", "程序运行所在地的当前日期和时间：一个记录 {year, month, day, hour, minute, second, offset}，其中 `offset` 是当地时间比 UTC 快的分钟数（系统没有提供时为 0）。"),
        ("The date of today, where the program runs: a record {year, month, day}.", "程序运行所在地的今天的日期：一个记录 {year, month, day}。"),
        ("Reads a date written as ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", with `Z` or an offset like `+01:00` if needed): `Ok(date)`, or `Err(message)` if it is not a valid date.", "读取按 ISO 8601 书写的日期（\"2024-02-29\"、\"2024-02-29T13:05:00\"，需要时带 `Z` 或像 `+01:00` 这样的时差）：`Ok(date)`，如果不是有效的日期则为 `Err(message)`。"),
        ("Writes a date as ISO 8601: \"2024-02-29\", with the time if the date has one, and `Z` or the offset if it has one.", "把日期写成 ISO 8601 格式：\"2024-02-29\"，如果日期带有时间则包括时间，如果带有时差则包括 `Z` 或时差。"),
        ("Is the value a valid date: a record {year, month, day} (with `hour`, `minute`, `second`, `offset` if needed) for a day that exists?", "这个值是否是有效的日期：一个表示存在的某一天的记录 {year, month, day}（需要时带 `hour`、`minute`、`second`、`offset`）？"),
        ("The date n days later (or earlier, when n is negative), at the same time of day.", "n 天之后（n 为负数时是之前）的日期，一天中的时间不变。"),
        ("The date and time n seconds later (or earlier, when n is negative). Add `60 * minutes` or `3600 * hours` the same way.", "n 秒之后（n 为负数时是之前）的日期和时间。用同样的方法可以加上 `60 * minutes` 或 `3600 * hours`。"),
        ("The number of whole days from date a to date b: negative when b is before a.", "从日期 a 到日期 b 的整天数：b 在 a 之前时为负数。"),
        ("The number of seconds from date a to date b, taking their offsets into account: negative when b is before a.", "从日期 a 到日期 b 的秒数，考虑了它们的时差：b 在 a 之前时为负数。"),
        ("The day of the week of a date, from 1 (Monday) to 7 (Sunday), as in ISO 8601.", "日期是星期几，从 1（星期一）到 7（星期日），和 ISO 8601 一样。"),
        ("The number of days of a month (from 1 to 12) of a year: 28 to 31.", "某年某月（1 到 12）的天数：28 到 31。"),
        ("Does the year have a 29 February? Every 4 years, except every 100 years, except every 400 years.", "这一年有 2 月 29 日吗？每 4 年一次，但每 100 年除外，而每 400 年又不除外。"),
        ("Checks that a property holds for many values made by a generator (100, or `cases`): the function is called with each value and must give `true` (or `()`, `Some(...)`, `Ok(...)`). When a value makes it false or stops it with an error, that value is simplified as much as possible (shrinking) and shown, with the seed that made it: the same values come back on every run. The property may also come last, as a block: `check_all(ints()) { |x| x + 0 == x }`.", "检查某个性质对生成器产生的许多值（100 个，或 `cases` 个）都成立：用每个值调用函数，它必须给出 `true`（或 `()`、`Some(...)`、`Ok(...)`）。当某个值让它为假或以错误停止时，这个值会被尽可能地简化（shrinking）并显示出来，同时显示产生它的种子：每次运行都会得到同样的值。性质也可以作为代码块放在最后：`check_all(ints()) { |x| x + 0 == x }`。"),
        ("A list of `n` values (10 if not given) made by a generator, to see what `check_all` will try.", "由生成器产生的 `n` 个值（没有给出时为 10 个）组成的列表，用来查看 `check_all` 会尝试什么。"),
        ("A generator of whole numbers from `min` to `max` (from -1000 to 1000 if not given), often near 0 and at the edges. A failing number is simplified towards 0.", "从 `min` 到 `max` 的整数生成器（没有给出时为 -1000 到 1000），经常接近 0 和边界。导致失败的数会向 0 简化。"),
        ("A generator of decimal numbers from `min` to `max` (from -1000.0 to 1000.0 if not given).", "从 `min` 到 `max` 的小数生成器（没有给出时为 -1000.0 到 1000.0）。"),
        ("A generator of texts of at most `max_length` characters (20 if not given): letters mostly, with digits, spaces, punctuation and accents. A failing text is made shorter and simpler.", "最多 `max_length` 个字符（没有给出时为 20）的文本生成器：大多是字母，也有数字、空格、标点和带重音的字母。导致失败的文本会被缩短并简化。"),
        ("A generator of `true` and `false`.", "`true` 和 `false` 的生成器。"),
        ("A generator of lists of at most `max_length` elements (10 if not given), each made by `generator`. A failing list is made shorter, then its elements simpler.", "最多 `max_length` 个元素（没有给出时为 10）的列表生成器，每个元素由 `generator` 产生。导致失败的列表会先被缩短，然后它的元素会被简化。"),
        ("A generator that picks one of the values of a list. A failing value is replaced by one written before it.", "从列表的值中选取一个的生成器。导致失败的值会被替换成写在它前面的值。"),
        ("A generator of records, with a generator for each field: `records_of({name: texts(), age: ints(0, 120)})`. Handy to give several values to `check_all`.", "记录的生成器，每个字段一个生成器：`records_of({name: texts(), age: ints(0, 120)})`。方便给 `check_all` 提供多个值。"),
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
    ],
}
