# Rythme for Visual Studio Code

Colors, automatic indentation and snippets for the [Rythme](https://github.com/maxlestage/Rythme) language (`.ry` files).

## Install

Copy this folder into your extensions folder and restart VS Code:

- Linux / macOS: `cp -r editors/vscode ~/.vscode/extensions/rythme`
- Windows (PowerShell): `Copy-Item -Recurse editors\vscode $env:USERPROFILE\.vscode\extensions\rythme`

Or build a package with `npx @vscode/vsce package` and install the `.vsix` file
(Extensions panel → `...` → *Install from VSIX*).

## Snippets

| Type      | You get                                   |
|-----------|-------------------------------------------|
| `fn`      | `fn name(x) = x`                          |
| `fnb`     | a function on several lines, with `end`   |
| `lambda`  | `fn(x) = x`                               |
| `if`      | `if condition then value else other`      |
| `match`   | a `match` with two cases                  |
| `type`    | `type Name = First(field) \| Second`      |
| `test`    | `test "..." = assert_eq(actual, expected)` |
| `pipe`    | a `|> map |> filter` pipeline             |
