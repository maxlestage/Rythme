# Rythme

**比 Python 和 Ruby 更简单，和 Ruby 一样好用：一门纯函数式编程语言，处处都有帮助。**
程序是以 `.ry` 结尾的文件。解释器用 Rust 编写。

*[English version →](README.md) · [Version française →](README.fr.md) · [En español →](README.es.md) · [Auf Deutsch →](README.de.md) · [Em português →](README.pt.md)*

这份 README 是一个概要。Rythme 的消息、帮助和错误解释都有简体中文版
（`--lang zh`）；完整的文档有英文版和法文版：[参考手册](docs/reference.md)
和[教师指南](docs/teaching.md)。网站和在线试玩也有中文版：
<https://maxlestage.github.io/Rythme/zh/>。

```rythme
# 值永远不会改变。一切都是函数。没有循环。
fn square(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # 保留偶数
  |> map(square)          # 把每个数平方
  |> sum                  # 把它们加起来
  |> print                # 20
```

## 为什么选择 Rythme？

- **比 Python 或 Ruby 更简单。** 15 个关键字。每件事只有一种做法。没有类、
  没有 `null`、没有异常、没有循环、没有修改：要学的概念更少，要找的错误也更少。
- **和 Ruby 一样好用。** 代码块（`xs.map do |x| x * 2 end`）、方法链、不带括号
  的调用（`puts "hello"`）、像 `even?` 这样的名字、`unless`、符号（`:ok`），
  以及标准库中的 Ruby 名字。
- **像 Rust 一样安全。** 可能缺失的值是 `Option`（`Some(x)` 或 `None`）；可能
  失败的操作返回 `Result`（`Ok(x)` 或 `Err(e)`），`?` 会把错误向上传递。程序在
  运行**之前**就会被检查。
- **处处都有帮助。** 每个错误都会说明它在*哪里*、*为什么*，以及*如何*修正。
  `rythme explain E003` 解释每个错误代码，`rythme help` 和 `rythme learn`
  （一次导览）教你这门语言，语言服务器（`rythme lsp`）把同样的帮助带到你的
  编辑器中。

错误看起来是这样的（简体中文）：

```
错误[E005]: 不能对 Text 和 Int 使用 `+`
 --> report.ry:2:13
  |
2 | print("年龄：" + age)
  |                ^ Text "年龄：" + Int
  |
  = 帮助: 要构造一个包含值的文本，请使用花括号："total: {x}"
      或者先转换这个值：text(x)
  = 关于这个错误的更多信息：`rythme explain E005`
```

```
错误[E001]: Rythme 中不使用 `for`
 --> loop.ry:1:1
  |
1 | for x in numbers
  | ^^^ 来自其他语言
  |
  = 帮助: Rythme 没有循环。请改为处理整个列表：
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      或者编写一个递归函数。参见 `rythme help functional`。
  = 关于这个错误的更多信息：`rythme explain E001`
```

中文字符在终端中占两列：`^` 始终对准出错的位置，代码和消息中有中文时也是如此。

## 安装

**Linux 和 macOS：**

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

**Windows**（PowerShell）：

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

**使用 [Rust](https://rustup.rs)：** `cargo install --git https://github.com/maxlestage/Rythme`。
更多方式见[英文 README](README.md#install)。

## 使用

| 命令                        | 作用                                             |
|-----------------------------|--------------------------------------------------|
| `rythme`                    | 交互模式：输入代码，查看结果                     |
| `rythme hello.ry`           | 运行程序                                         |
| `rythme check hello.ry`     | 不运行程序，找出问题                             |
| `rythme test hello.ry`      | 运行文件中写的测试                               |
| `rythme fmt hello.ry`       | 把代码整理成官方格式                             |
| `rythme new hello`          | 新项目：`rythme.toml`、`src/`、`tests/`          |
| `rythme help [主题]`        | 某个主题或函数的帮助（`rythme help 列表`）       |
| `rythme explain E003`       | 解释错误代码                                     |
| `rythme learn`              | 语言导览                                         |
| `rythme steps fib.ry "fib(4)"` | 一步一步显示计算过程                          |

在命令前加上 `--lang zh`、设置 `RYTHME_LANG=zh`，或者系统语言是中文时
（`LANG=zh_CN.UTF-8`），消息会自动使用简体中文。关键字、内置函数的名字和命令
在所有语言中都一样。在交互模式中，`:lang zh` 可以切换语言。

## 稳定性承诺

Rythme 1.0 遵循[语义化版本](https://semver.org/lang/zh-CN/)。在所有 1.x
版本中：能运行的程序会继续运行，输出相同（输入相同时），退出码也相同；错误和
警告代码（`E003`、`W004`……）的含义不变；要移除的功能会先给出警告，至少持续
一个完整的次版本，并且只会在 2.0 中移除。可能变化的是消息的措辞、性能，以及
`rythme check` 的新警告。详情见[英文 README](README.md#stability-promise)，
1.0 的内容见 [`CHANGELOG.md`](CHANGELOG.md)。

## 许可证

[MIT](LICENSE).
