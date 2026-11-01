# 用 Rythme 教学

*[English version →](teaching.md) · [Version française →](enseigner.md) · [En español →](teaching.es.md) · [Auf Deutsch →](teaching.de.md) · [Em português →](teaching.pt.md) · [In italiano →](teaching.it.md) · [日本語 →](teaching.ja.md)*

Rythme 是为课堂设计的：一门小语言，配上会教人的消息，让学生把时间花在思想上，而不是语法上。本指南涵盖一门课需要的内容：课程大纲、自动检查和评分的练习、展示代码如何运行的方法，以及对待 AI 助手的规则。

## 为什么适合上课

- **写出真正的程序之前，要学的东西很少。** 15 个关键字，定义函数只有一种方式，做选择只有一种方式（`if`/`match`），没有类、没有循环、没有修改。整个语法一页就能写下（`rythme help syntax`）。
- **解释器就是助教。** 错误会指出确切的位置，解释问题，并给出修正建议。从其他语言带来的习惯（`for`、`return`、`x += 1`、`null`……）会得到函数式做法的解释。`rythme explain E0xx` 给出带例子的详细说明。`rythme check` 不运行程序就能找出问题，包括一定会失败的操作，比如 `"年龄：" + 36`。
- **从第一天起就用函数式的方式思考。** 不可变性、函数作为值、递归、模式匹配、代数类型、错误作为值（`Option`、`Result`）：这些是 Haskell、OCaml、Elm、Elixir 和 Rust 的概念，但没有它们的语法，第一天也不用和类型系统较劲。类型是可选的：检查器会自己推断出来；课程讲到时，学生可以写出类型（`fn area(w: Int, h: Int) -> Int`、`type Tree(a)`），由检查器来核对。
- **测试是内置的。** 在代码旁边写 `test "..." = assert_eq(实际值, 期望值)`，用 `rythme test file.ry` 运行。练习可以连同测试一起发给学生。
- **学生熟悉的标准库。** 文本、列表、字典、日期、CSV、正则表达式，还有用于机器学习单元的向量、统计和斜率（`rythme help ml`）。会 Ruby 或 Python 的学生能用熟悉的名字找到许多函数（`upcase`、`strip`、`each_slice`……）：见 `rythme help ruby` 和[标准库对比](stdlib-audit.md)（英文）。
- **使用学生的语言**，而且**易于安装、免费、开放**：一个程序，没有依赖，MIT 许可证，支持 Linux、macOS 和 Windows；用网站上的在线试玩则什么都不用装。

## 一种课程安排（12 节课）

| # | 主题 | Rythme 概念 | 练习 | 示例 |
|---|------|-------------|------|------|
| 1 | 表达式和值 | 数字、文本、`print`、文本中的 `{...}`、交互模式 | 01、03 | [`hello.ry`](../examples/hello.ry) |
| 2 | 函数 | `fn f(x) = ...`、以 `end` 结束的块、调用、`help(f)` | 02 | |
| 3 | 做选择 | `if ... then ... else`、布尔值、`and`/`or`/`not` | 04–06 | |
| 4 | 递归 | 基本情况、按情况定义 `fn fib(0) = 0`、守卫 | 07–10 | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | 列表 | 字面量、区间 `1..10`、模式 `[x, ..rest]` | 11–13 | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | 高阶函数 | `map`、`filter`、`fold`、匿名函数、`\|>` | 14–19 | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | 记录 | `{name: ...}`、访问字段、`{..r, x: 1}` | 20–21 | [`students.ry`](../examples/students.ry) |
| 8 | 自己的类型 | `type`、构造器、`match`、遗漏的情况（W002）、代码中写的类型 | 22–23 | [`shapes.ry`](../examples/shapes.ry) |
| 9 | 递归类型 | 树、结构递归、带参数的类型（`type Tree(a)`） | 24–25 | [`tree.ry`](../examples/tree.ry) |
| 10 | 错误作为值 | `Option`、`Result`、`?`、`unwrap_or` | 26–28 | [`results.ry`](../examples/results.ry) |
| 11 | 测试和模块 | `test`、`assert_eq`、`use` | 30 | [`testing.ry`](../examples/testing.ry) |
| 12 | 项目 | 文本处理、数据分析、一个小游戏 | 29 | [`words.ry`](../examples/words.ry) |

尾递归（第 4 或第 9 节课）值得花一点时间：错误 E014（“嵌套调用太多”）会解释它，`rythme explain E014` 会展示累加器的技巧。

第 7 节课之后，可以用两三节课组成一个**人工智能单元**：向量、统计、人工神经元、梯度下降、最近邻和极小化极大（minimax）算法（练习 31 到 36、`rythme help ml`，以及示例 [`perceptron.ry`](../examples/perceptron.ry) 和 [`knn.ry`](../examples/knn.ry)）。

## 自动检查的练习

给学生一个已经写好测试的文件，每个要写的函数先写成 `fail("待完成")`。下面是一个学生完成的文件，里面有一个典型的错误：

```rythme
## 返回非空列表中最大的元素。
fn largest(list) = fold(list, 0, max)

test "一个元素" = assert_eq(largest([7]), 7)
test "多个元素" = assert_eq(largest([3, 9, 2]), 9)
test "负数" = assert_eq(largest([-3, -1, -2]), -1)
```

`rythme test largest.ry` 会报告哪些测试通过，并为每个失败的测试给出期望值和实际值：

```text
正在运行 3 个测试
  ✓ 一个元素
  ✓ 多个元素
  ✗ 负数
错误[E015]: 断言失败：两个值不同
 --> largest.ry:6:13
  |
6 | test "负数" = assert_eq(largest([-3, -1, -2]), -1)
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: 实际：0
      期望：-1
  = 关于这个错误的更多信息：`rythme explain E015`

3 个测试：2 个通过，1 个失败 (1 ms)
```

已经准备好三十六个这样的练习：[**练习**](exercises/README.md)（英文），难度从 ★☆☆ 到 ★★★，按上面的课程顺序排列，每个都有题目、提示和测试，支持每一种消息语言，并为教师提供参考答案。它们内置在 `rythme` 中：

```sh
rythme exercise list                     # 练习列表
rythme exercise start 7                  # 把 07-recursion.ry 复制到当前文件夹
rythme exercise check 07-recursion.ry    # 运行它的测试，给出建议，并推荐下一个练习
```

`rythme exercise check` 还会指出内置练习的测试是否被改过：只有练习自己的测试通过，练习才算完成。你自己的练习也可以采用同样的格式（题目和提示写在注释里，函数里写 `fail("待完成")`，测试放在 `# --- 测试（请不要修改）---` 这一行之后）；`rythme exercise check` 适用于任何带测试的文件。

## 用 `--format json` 评分

`rythme test` 可以接受一个文件夹：每个学生一个文件（或一个文件夹），一条命令就能运行全班的所有测试。

```sh
rythme test --sandbox --format json submissions/ > results.json
```

- **`--format json`** 输出一个 JSON 文档，而不是给人看的消息：每个文件有 `tests`、`failed` 和 `broken`（无法编译的文件，或在测试前就停止的文件）；每个测试有它的 `name`、`file`、`passed` 和 `failure`；最后是 `summary`。键名永远不变（[模式](rythme-output.schema.json)）；消息跟随 `--lang`。
- **`--sandbox`** 让程序无法接触文件、文件夹、环境变量和网络。没有时间限制：请在限时下运行这条命令（在 Linux 上用 `timeout 60 rythme test ...`）。
- **学生可以修改自己文件里的测试。** 评分时请用你自己的那份测试：`rythme exercise check` 会指出内置练习的测试是否被改过，而参考答案是公开的（在代码仓库里）：计分的作业请用你自己出的练习。
- `rythme check --format json` 会加上警告，每个警告都有固定的代码（W001 未使用的名字，W002 遗漏的情况……），`rythme fmt --check` 会说明代码是否符合官方格式。

Rythme 自己就能读 JSON。下面的程序把结果变成分数：

```rythme
# grades.ry
fn grade(f) = if f.broken then "{f.file}：无法运行" else "{f.file}：{f.tests - f.failed}/{f.tests}"

results = read_file(args()[0]) |> unwrap |> parse_json |> unwrap
each(results.files, fn(f) = print(grade(f)))
```

`rythme grades.ry results.json` 输出：

```text
submissions/ada.ry：3/3
submissions/alan.ry：2/3
submissions/grace.ry：无法运行
```

## 展示代码如何运行

`rythme steps` 用代换模型一步一步地展示计算过程：每个调用都被替换成它的函数体，直到只剩下结果。递归、累加器和尾调用都变得看得见。

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

fib(4)
```

`rythme steps fib.ry` 显示：

```text
fib(4)
  → fib(3) + fib(2)
  → fib(2) + fib(1) + fib(2)
  → fib(1) + fib(0) + fib(1) + fib(2)
  → 1 + fib(0) + fib(1) + fib(2)
  → 1 + 0 + fib(1) + fib(2)
  → 1 + fib(1) + fib(2)
  → 1 + 1 + fib(2)
  → 2 + fib(2)
  → 2 + (fib(1) + fib(0))
  → 2 + (1 + fib(0))
  → 2 + (1 + 0)
  → 2 + 1
  → 3
```

`rythme steps fib.ry "fib(6)"` 展示另一个表达式，交互模式中用 `:steps fib(6)`，在线试玩中用 **步骤** 按钮；在那里，**分享** 会生成一个打开在同一步的链接：可以投影，也可以发给全班。

`rythme debug file.ry` 会在你指定的地方暂停程序（`break 12`、`break fib`），并显示其中名字的值。`where` 列出正在进行的调用及其参数，并说明同一个函数有多少个调用在等待：递归的调用栈，实时可见。尾位置的调用会说明它取代了调用者的位置。在 VS Code 中，对 `.ry` 文件按 `F5` 会启动同一个调试器，断点设在页边。[参考手册](reference.md#79-the-debugger-rythme-debug)（英文）展示了一次完整的调试过程。

## 什么都不用装：在线试玩

[网站](https://maxlestage.github.io/Rythme/zh/)上的在线试玩在浏览器中运行真正的解释器，消息和帮助都一样：学生在任何电脑、平板或手机上都能开始。

- **运行**、**检查**、**测试**、**格式化** 和 **步骤** 与对应的命令作用相同；粘贴一个练习，然后按 **测试**。
- **分享** 会生成一个包含程序本身的链接（不会向任何服务器发送内容）：可以分发起始代码，或者这节课的程序。**种子** 让每个学生得到相同的 `random` 抽取结果。
- 安装成应用后（菜单中的 **安装应用**，或在 iPhone 和 iPad 上选 *添加到主屏幕*），网站和在线试玩离线也能用，在网络不好的教室里很有帮助。正在写的代码保存在浏览器中。
- 在浏览器中，无法读写文件，`input()` 返回空文本，嵌套调用最多 600 层（尾调用没有限制）。

## 使用学生的语言

- 每条消息、每个帮助主题、每个解释和 `rythme learn` 的每一页都有英语、法语、西班牙语、德语、葡萄牙语、中文、意大利语和日语版本。Rythme 跟随系统的语言；`--lang es` 或环境变量 `RYTHME_LANG=es` 可以选择另一种。
- 关键字、函数名和命令在每种语言中都相同：一个班写的程序在哪里都能运行，`rythme help` 也接受各种语言的主题名（`rythme help 列表`）。
- 练习也跟随语言：`rythme --lang de exercise start 7` 会写出德语的 `07-rekursion.ry`。
- 网站、在线试玩和本指南都有同样的这些语言版本。

## 课堂上的 AI 助手

学生手边都有 AI 助手。Rythme 不尝试检测它们；下面是 Rythme 提供的东西，以及一门课可以调整采用的规则。

**Rythme 提供什么。** AI 助手是在 Rythme 出现之前训练的：任其自由发挥时，它们会写出 Python 或 JavaScript 的习惯（`for`、`return`、`x += 1`），Rythme 会拒绝这些代码，并用消息说明应该怎么写。给它们 `rythme help llms`（一份文本包含整个语言，网站的 *AI* 页面上也有），它们就能写出正确的 Rythme。`rythme mcp` 让 AI 助手在沙箱中自己运行、检查和测试代码。在线试玩中的 *复制给 AI* 会复制程序及其输出或错误，可以直接粘贴到对话里。`rythme fix` 会应用安全修复（`&&` → `and`、`let x = 3` → `x = 3`），`rythme fix --dry-run` 只显示这些修复。

**一种可行的规则。**

- **最初几周：代码由学生自己写。** 目的是培养习惯，而 Rythme 的消息和 `rythme explain` 已经会解释每个错误。AI 助手可以解释一条消息或一个概念，但不要替学生写答案。
- **读完消息之后再用 `rythme fix`**：它只修改确定无疑的地方，也就是消息本身建议的修正。
- **项目：允许使用，但要说明。** 把 `rythme help llms` 交给 AI 助手，要求学生说明用它做了什么，并讲解自己的代码。
- **评估理解，而不只是结果。** 测试检查的是程序能否工作，而不是谁写的。在课堂上加一些简短的检查：不用电脑，预测 `rythme steps` 的步骤、一个程序的输出，或者一条消息的修正方法；或者当场在在线试玩里做一个练习。
- **用于你自己的工作**：拿到 `rythme help llms` 的 AI 助手可以按上面的格式起草新练习。分发之前，用你自己写的答案和 `rythme test` 检查每一个练习。

## 实用建议

- 从交互模式开始（`rythme`）：每一行都会显示它的值。`:doc map` 显示一个函数的文档，按 Tab 补全名字。
- `rythme learn` 是一次语言导览，学生可以自己跟着学。
- `rythme help builtins` 按类别列出所有内置函数，每个命令都支持 `--help`（`rythme test --help`）。`rythme completions bash`（或 `zsh`、`fish`、`powershell`）可以在终端中补全命令。
- 安装 [VS Code 扩展](../editors/vscode)：语法着色、代码片段、输入时显示的问题及其快速修复、鼠标悬停时的文档，以及调试器。其他编辑器也可以使用语言服务器（[`editors/README.md`](../editors/README.md)，英文）。
- `rythme fmt` 让所有文件都使用相同的格式：学生可以在交作业之前运行它（或 `rythme fmt --check .`）。
- 对于更大的项目，`rythme new NAME` 会创建一个包含 `src/`、`tests/` 和 `rythme.toml` 的文件夹；这时单独运行 `rythme test` 就会运行项目的所有测试，`rythme add` 可以从[包索引](reference.md#711-the-package-index)（英文）中添加一个包。
