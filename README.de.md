# Rythme

**Einfacher als Python und Ruby, so angenehm wie Ruby: eine rein funktionale
Programmiersprache, mit Hilfe überall.**
Programme sind Dateien, die auf `.ry` enden. Der Interpreter ist in Rust geschrieben.

*[English version →](README.md) · [Version française →](README.fr.md) · [En español →](README.es.md) · [Em português →](README.pt.md) · [中文 →](README.zh.md)*

Diese README ist eine Zusammenfassung. Die Meldungen, die Hilfe und die
Erklärungen von Rythme gibt es auf Deutsch (`--lang de`); die vollständige
Dokumentation ist auf Englisch und Französisch: das
[Referenzhandbuch](docs/reference.md) und der
[Leitfaden für Lehrkräfte](docs/teaching.md). Die Website und ihr Playground
gibt es auch auf Deutsch: <https://maxlestage.github.io/Rythme/de/>.

```rythme
# Werte ändern sich nie. Alles ist eine Funktion. Es gibt keine Schleifen.
fn quadrat(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # die geraden Zahlen behalten
  |> map(quadrat)         # jede quadrieren
  |> sum                  # sie addieren
  |> print                # 20
```

## Warum Rythme?

- **Einfacher als Python oder Ruby.** 15 Schlüsselwörter. Ein Weg für jede
  Sache. Keine Klassen, kein `null`, keine Exceptions, keine Schleifen, keine
  Mutation: weniger Begriffe zu lernen, weniger Fehler zu suchen.
- **So angenehm wie Ruby.** Blöcke (`xs.map do |x| x * 2 end`),
  Methodenketten, Aufrufe ohne Klammern (`puts "hallo"`), Namen wie `even?`,
  `unless`, Symbole (`:ok`) und die Ruby-Namen der Bibliothek.
- **Sicher wie Rust.** Ein Wert, der fehlen kann, ist eine `Option`
  (`Some(x)` oder `None`); eine Operation, die scheitern kann, liefert ein
  `Result` (`Ok(x)` oder `Err(e)`), und `?` gibt Fehler nach oben weiter. Das
  Programm wird **vor** dem Ausführen geprüft.
- **Hilfe überall.** Jeder Fehler sagt, *wo* er ist, *warum*, und *wie* man
  ihn behebt. `rythme explain E003` erklärt jeden Fehlercode, `rythme help`
  und `rythme learn` (eine geführte Tour) bringen dir die Sprache bei, und
  der Sprachserver (`rythme lsp`) bringt dieselbe Hilfe in deinen Editor.

So sieht ein Fehler aus, auf Deutsch:

```
Fehler[E005]: `+` kann nicht mit Text und Int verwendet werden
 --> bericht.ry:2:17
  |
2 | print("Alter: " + alter)
  |                 ^ Text "Alter: " + Int
  |
  = Hilfe: um einen Text mit einem Wert darin zu bauen, verwende geschweifte Klammern: "total: {x}"
      oder wandle den Wert zuerst um: text(x)
  = mehr über diesen Fehler: `rythme explain E005`
```

```
Fehler[E001]: `for` wird in Rythme nicht verwendet
 --> schleife.ry:1:1
  |
1 | for x in zahlen
  | ^^^ kommt aus einer anderen Sprache
  |
  = Hilfe: Rythme hat keine Schleifen. Arbeite stattdessen mit der ganzen Liste:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      oder schreib eine rekursive Funktion. Siehe `rythme help functional`.
  = mehr über diesen Fehler: `rythme explain E001`
```

## Installieren

**Linux und macOS:**

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

**Windows** (PowerShell):

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

**Mit [Rust](https://rustup.rs):** `cargo install --git https://github.com/maxlestage/Rythme`.
Weitere Möglichkeiten in der [englischen README](README.md#install).

## Verwenden

| Befehl                      | Was er tut                                        |
|-----------------------------|---------------------------------------------------|
| `rythme`                    | interaktiver Modus: Code tippen, Ergebnis sehen   |
| `rythme hallo.ry`           | ein Programm ausführen                            |
| `rythme check hallo.ry`     | Probleme finden, ohne das Programm auszuführen    |
| `rythme test hallo.ry`      | die Tests in der Datei ausführen                  |
| `rythme fmt hallo.ry`       | dem Code sein offizielles Layout geben            |
| `rythme new hallo`          | ein neues Projekt: `rythme.toml`, `src/`, `tests/` |
| `rythme help [thema]`       | Hilfe zu einem Thema oder einer Funktion (`rythme help listen`) |
| `rythme explain E003`       | einen Fehlercode erklären                         |
| `rythme learn`              | geführte Tour durch die Sprache                   |
| `rythme steps fib.ry "fib(4)"` | die Rechnung Schritt für Schritt              |

Die Meldungen sind auf Deutsch mit `--lang de` vor dem Befehl, mit
`RYTHME_LANG=de`, oder automatisch, wenn die Sprache des Systems Deutsch ist
(`LANG=de_DE.UTF-8`). Die Schlüsselwörter, die Namen der eingebauten
Funktionen und die Befehle sind in allen Sprachen gleich. Im interaktiven
Modus wechselt `:lang de` die Sprache.

## Lizenz

[MIT](LICENSE).
