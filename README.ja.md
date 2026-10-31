# Rythme

**Python や Ruby よりシンプルで、Ruby のように楽しい：どこにでもヘルプがある純粋関数型プログラミング言語。**
プログラムは `.ry` で終わるファイルです。インタープリターは Rust で書かれています。

*[English version →](README.md) · [Version française →](README.fr.md) · [En español →](README.es.md) · [Auf Deutsch →](README.de.md) · [Em português →](README.pt.md) · [中文 →](README.zh.md) · [In italiano →](README.it.md)*

この README は概要です。Rythme のメッセージ、ヘルプ、エラーの説明は日本語でも読めます（`--lang ja`）。[教師向けガイド](docs/teaching.ja.md)も日本語版があります。完全な[リファレンスマニュアル](docs/reference.md)は英語版とフランス語版です。Web サイトとプレイグラウンドも日本語で使えます：<https://maxlestage.github.io/Rythme/ja/>。

```rythme
# 値は決して変わらない。すべてが関数。ループはない。
fn square(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # 偶数だけ残す
  |> map(square)          # それぞれを 2 乗する
  |> sum                  # 合計する
  |> print                # 20
```

## なぜ Rythme？

- **Python や Ruby よりシンプル。** キーワードは 15 個。それぞれのことをする方法は 1 つだけ。クラスも `null` も例外もループもミューテーションもありません：覚える概念も、探すバグも少なくなります。
- **Ruby のように楽しい。** ブロック（`xs.map do |x| x * 2 end`）、メソッドチェーン、括弧なしの呼び出し（`puts "hello"`）、`even?` のような名前、`unless`、シンボル（`:ok`）、そして標準ライブラリにある Ruby の名前。
- **Rust のように安全。** ないかもしれない値は `Option`（`Some(x)` か `None`）、失敗するかもしれない操作は `Result`（`Ok(x)` か `Err(e)`）を返し、`?` がエラーを呼び出し元に渡します。プログラムは実行**前**にチェックされます。
- **どこにでもヘルプ。** どのエラーも、*どこで*、*なぜ*起きたか、*どう*直すかを伝えます。`rythme explain E003` はどのエラーコードも説明し、`rythme help` と `rythme learn`（ガイドツアー）が言語を教え、言語サーバー（`rythme lsp`）が同じヘルプをエディターに届けます。

エラーはこのように表示されます（日本語）：

```
エラー[E005]: `+` は Text と Int には使えない
 --> report.ry:2:13
  |
2 | print("年齢：" + age)
  |                ^ Text "年齢：" + Int
  |
  = ヘルプ: 値を含むテキストを作るには波かっこを使う："total: {x}"
      または先に値を変換する：text(x)
  = このエラーについて詳しく：`rythme explain E005`
```

```
エラー[E001]: `for` は Rythme では使わない
 --> loop.ry:1:1
  |
1 | for x in numbers
  | ^^^ 別の言語のもの
  |
  = ヘルプ: Rythme にループはない。代わりにリスト全体を処理する：
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      または再帰関数を書く。`rythme help functional` を参照。
  = このエラーについて詳しく：`rythme explain E001`
```

日本語の文字はターミナルで 2 列分を使います：コードやメッセージに日本語があっても、`^` はいつも誤りの場所の真下にそろいます。

## インストール

**Linux と macOS：**

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

**Windows**（PowerShell）：

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

**[Rust](https://rustup.rs) で：** `cargo install --git https://github.com/maxlestage/Rythme`。
その他の方法は[英語の README](README.md#install)を参照してください。

## 使い方

| コマンド                    | すること                                         |
|-----------------------------|--------------------------------------------------|
| `rythme`                    | 対話モード：コードを入力して結果を見る           |
| `rythme hello.ry`           | プログラムを実行する                             |
| `rythme check hello.ry`     | プログラムを実行せずに問題を見つける             |
| `rythme test hello.ry`      | ファイルに書いたテストを実行する                 |
| `rythme fmt hello.ry`       | コードを公式のレイアウトに整形する               |
| `rythme new hello`          | 新しいプロジェクト：`rythme.toml`、`src/`、`tests/` |
| `rythme add mathlib`        | [パッケージインデックス](docs/reference.md#711-the-package-index)のパッケージを加える（`rythme search WORDS` で探す） |
| `rythme help [トピック]`    | トピックや関数のヘルプ（`rythme help リスト`）   |
| `rythme explain E003`       | エラーコードを説明する                           |
| `rythme learn`              | 言語のガイドツアー                               |
| `rythme steps fib.ry "fib(4)"` | 計算を 1 ステップずつ表示する                 |
| `rythme debug fib.ry`          | デバッガー：ブレークポイント、1 行ずつの実行、値 |

コマンドの前に `--lang ja` を付けるか、`RYTHME_LANG=ja` を設定するか、システムの言語が日本語なら（`LANG=ja_JP.UTF-8`）、メッセージは日本語になります。キーワード、組み込み関数の名前、コマンドはどの言語でも同じです。対話モードでは `:lang ja` で言語を切り替えられます。

## 安定性の約束

Rythme 1.0 は[セマンティックバージョニング](https://semver.org/lang/ja/)に従います。1.x のすべてのバージョンで：動くプログラムは同じ出力（入力が同じなら）と同じ終了コードで動き続けます。エラーと警告のコード（`E003`、`W004` など）の意味は変わりません。取り除く機能には、少なくともマイナーバージョン 1 つの間、先に警告が出され、取り除かれるのは 2.0 だけです。変わりうるのは、メッセージの言い回し、性能、`rythme check` の新しい警告です。詳しくは[英語の README](README.md#stability-promise)を、1.0 の内容は [`CHANGELOG.md`](CHANGELOG.md) を参照してください。

## ライセンス

[MIT](LICENSE).
