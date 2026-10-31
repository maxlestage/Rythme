# Unterrichten mit Rythme

*[English version →](teaching.md) · [Version française →](enseigner.md) · [En español →](teaching.es.md) · [Em português →](teaching.pt.md) · [中文 →](teaching.zh.md)*

Rythme wurde für den Unterricht entworfen: eine kleine Sprache mit
Meldungen, die lehren, damit die Lernenden ihre Zeit mit Ideen verbringen
statt mit Syntax. Dieser Leitfaden behandelt, was ein Kurs braucht: einen
Plan, automatisch geprüfte und bewertete Übungen, Wege zu zeigen, wie Code
läuft, und eine Regelung für KI-Assistenten.

## Warum es zu einem Kurs passt

- **Wenig zu lernen, bevor echte Programme entstehen.** 15
  Schlüsselwörter, ein Weg, eine Funktion zu definieren, ein Weg zu wählen
  (`if`/`match`), keine Klassen, keine Schleifen, keine Mutation. Die
  ganze Syntax passt auf eine Seite (`rythme help syntax`).
- **Der Interpreter ist ein Tutor.** Fehler zeigen auf die genaue Stelle,
  erklären das Problem und schlagen eine Korrektur vor. Gewohnheiten aus
  anderen Sprachen (`for`, `return`, `x += 1`, `null`...) bekommen eine
  Erklärung des funktionalen Wegs. `rythme explain E0xx` gibt die lange
  Fassung mit Beispielen. `rythme check` findet Probleme, ohne das Programm
  auszuführen, auch Operationen, die immer scheitern würden, wie
  `"Alter: " + 36`.
- **Funktional denken vom ersten Tag an.** Unveränderlichkeit, Funktionen
  als Werte, Rekursion, Muster, algebraische Typen, Fehler als Werte
  (`Option`, `Result`): die Begriffe von Haskell, OCaml, Elm, Elixir oder
  Rust, ohne ihre Syntax und ohne ein Typsystem, mit dem man am ersten Tag
  kämpft. Typen sind optional: die Prüfung vor dem Ausführen findet sie
  selbst, und wenn der Kurs so weit ist, schreiben die Lernenden sie
  (`fn area(w: Int, h: Int) -> Int`, `type Tree(a)`), und die Prüfung
  kontrolliert sie.
- **Tests sind eingebaut.** `test "..." = assert_eq(erhalten, erwartet)`
  neben dem Code, `rythme test datei.ry` zum Ausführen. Übungen können
  mit ihren Tests ausgeliefert werden.
- **Eine Bibliothek, die die Lernenden wiedererkennen.** Texte, Listen,
  Wörterbücher, Datumsangaben, CSV, reguläre Ausdrücke, und Vektoren,
  Statistik und Steigungen für eine Einheit über maschinelles Lernen
  (`rythme help ml`). Wer Ruby oder Python kennt, findet viele Funktionen
  unter den bekannten Namen (`upcase`, `strip`, `each_slice`...): siehe
  `rythme help ruby` und [die Bibliothek im Vergleich](stdlib-audit.md)
  (auf Englisch).
- **In der Sprache deiner Lernenden**, und **leicht zu installieren,
  kostenlos, offen**: ein einziges Programm, keine Abhängigkeit,
  MIT-Lizenz, für Linux, macOS und Windows, oder gar nichts zu
  installieren mit dem Playground der Website.

## Ein möglicher Kurs (12 Sitzungen)

| # | Thema | Begriffe von Rythme | Übungen | Beispiel |
|---|-------|---------------------|---------|----------|
| 1 | Ausdrücke und Werte | Zahlen, Texte, `print`, `{...}` in Texten, der interaktive Modus | 01, 03 | [`hello.ry`](../examples/hello.ry) |
| 2 | Funktionen | `fn f(x) = ...`, Blöcke mit `end`, Aufrufe, `help(f)` | 02 | |
| 3 | Wählen | `if ... then ... else`, Wahrheitswerte, `and`/`or`/`not` | 04–06 | |
| 4 | Rekursion | Basisfall, Definitionen nach Fällen `fn fib(0) = 0`, Wächter | 07–10 | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Listen | Literale, Bereiche `1..10`, Muster `[x, ..rest]` | 11–13 | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Funktionen höherer Ordnung | `map`, `filter`, `fold`, anonyme Funktionen, `\|>` | 14–19 | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Records | `{name: ...}`, Zugriff auf Felder, `{..r, x: 1}` | 20–21 | [`students.ry`](../examples/students.ry) |
| 8 | Eigene Typen | `type`, Konstruktoren, `match`, vergessene Fälle (W002), im Code geschriebene Typen | 22–23 | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Rekursive Typen | Bäume, strukturelle Rekursion, Typen mit Parametern (`type Tree(a)`) | 24–25 | [`tree.ry`](../examples/tree.ry) |
| 10 | Fehler als Werte | `Option`, `Result`, `?`, `unwrap_or` | 26–28 | [`results.ry`](../examples/results.ry) |
| 11 | Tests und Module | `test`, `assert_eq`, `use` | 30 | [`testing.ry`](../examples/testing.ry) |
| 12 | Projekt | Textverarbeitung, Datenanalyse, ein kleines Spiel | 29 | [`words.ry`](../examples/words.ry) |

Die Endrekursion (Sitzung 4 oder 9) verdient einen Moment: der Fehler E014
(„zu viele verschachtelte Aufrufe“) erklärt sie, und
`rythme explain E014` zeigt die Technik mit dem Akkumulator.

Nach Sitzung 7 können zwei oder drei Sitzungen eine **Einheit über
künstliche Intelligenz** bilden: Vektoren, Statistik, ein künstliches
Neuron, Gradientenabstieg, nächste Nachbarn und Minimax (Übungen 31 bis
36, `rythme help ml`, und die Beispiele
[`perceptron.ry`](../examples/perceptron.ry) und [`knn.ry`](../examples/knn.ry)).

## Übungen mit automatischer Prüfung

Gib den Lernenden eine Datei, in der die Tests schon geschrieben sind, und
jede zu schreibende Funktion als `fail("zu tun")`. Hier ist eine, die
jemand ausgefüllt hat, mit einem klassischen Fehler:

```rythme
## Gibt das größte Element einer nicht leeren Liste zurück.
fn largest(list) = fold(list, 0, max)

test "ein Element" = assert_eq(largest([7]), 7)
test "mehrere" = assert_eq(largest([3, 9, 2]), 9)
test "negativ" = assert_eq(largest([-3, -1, -2]), -1)
```

`rythme test largest.ry` zeigt, was besteht, und für jeden Fehlschlag den
erwarteten und den erhaltenen Wert:

```text
3 Test(s) werden ausgeführt
  ✓ ein Element
  ✓ mehrere
  ✗ negativ
Fehler[E015]: Zusicherung fehlgeschlagen: die Werte sind verschieden
 --> largest.ry:6:18
  |
6 | test "negativ" = assert_eq(largest([-3, -1, -2]), -1)
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: erhalten: 0
      erwartet: -1
  = mehr über diesen Fehler: `rythme explain E015`

3 Test(s): 2 bestanden, 1 fehlgeschlagen (1 ms)
```

Sechsunddreißig solche Übungen sind fertig: [**die Übungen**](exercises/README.md)
(auf Englisch), abgestuft von ★☆☆ bis ★★★ und dem Kurs oben folgend, jede
mit einer Aufgabe, Hinweisen und Tests, in jeder Sprache der Meldungen, mit
Musterlösungen für Lehrkräfte. Sie sind in `rythme` eingebaut:

```sh
rythme exercise list                      # die Übungen
rythme exercise start 7                   # kopiert 07-rekursion.ry in den aktuellen Ordner
rythme exercise check 07-rekursion.ry     # führt ihre Tests aus, mit Ratschlägen, und schlägt die nächste vor
```

`rythme exercise check` sagt auch, wenn die Tests einer eingebauten Übung
verändert wurden: die Übung ist erst gelöst, wenn ihre eigenen Tests
bestehen. Deine eigenen Übungen können demselben Muster folgen (eine
Aufgabe und Hinweise in Kommentaren, `fail("zu tun")` in den Funktionen,
die Tests nach einer Zeile `# --- Tests (nicht verändern) ---`);
`rythme exercise check` funktioniert mit jeder Datei, die Tests hat.

## Bewerten mit `--format json`

`rythme test` nimmt einen Ordner: mit einer Datei (oder einem Ordner) pro
Person führt ein einziger Befehl alle Tests der Klasse aus.

```sh
rythme test --sandbox --format json submissions/ > results.json
```

- **`--format json`** gibt ein einziges JSON-Dokument aus statt der
  Meldungen für Menschen: für jede Datei `tests`, `failed` und `broken`
  (eine Datei, die nicht kompiliert oder vor ihren Tests anhält); für
  jeden Test seinen `name`, seine `file`, `passed` und `failure`; dann eine
  `summary`. Die Schlüssel ändern sich nie ([das Schema](rythme-output.schema.json));
  die Meldungen folgen `--lang`.
- **`--sandbox`** hält die Programme von Dateien, Ordnern,
  Umgebungsvariablen und dem Netz fern. Es gibt keine Zeitgrenze: führe
  den Befehl mit einer aus (unter Linux `timeout 60 rythme test ...`).
- **Die Lernenden können die Tests** ihrer Datei verändern. Für eine Note
  verwende deine eigene Kopie: `rythme exercise check` sagt, wenn die Tests
  einer eingebauten Übung verändert wurden, und die Musterlösungen sind
  öffentlich (im Repository): für bewertete Arbeiten schreib eigene
  Übungen.
- `rythme check --format json` fügt die Warnungen hinzu, jede mit ihrem
  festen Code (W001 ein ungenutzter Name, W002 ein vergessener Fall...),
  und `rythme fmt --check` sagt, ob der Code das offizielle Layout hat.

Rythme liest JSON selbst. Dieses Programm macht aus den Ergebnissen Noten:

```rythme
# grades.ry
fn grade(f) = if f.broken then "{f.file}: läuft nicht" else "{f.file}: {f.tests - f.failed}/{f.tests}"

results = read_file(args()[0]) |> unwrap |> parse_json |> unwrap
each(results.files, fn(f) = print(grade(f)))
```

`rythme grades.ry results.json` gibt aus:

```text
submissions/ada.ry: 3/3
submissions/alan.ry: 2/3
submissions/grace.ry: läuft nicht
```

## Zeigen, wie Code läuft

`rythme steps` zeigt eine Berechnung Schritt für Schritt, im
Ersetzungsmodell: jeder Aufruf wird durch den Rumpf seiner Funktion
ersetzt, bis nur das Ergebnis bleibt. Rekursion, Akkumulatoren und
Endaufrufe werden sichtbar.

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

fib(4)
```

`rythme steps fib.ry` zeigt:

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

`rythme steps fib.ry "fib(6)"` zeigt einen anderen Ausdruck, `:steps fib(6)`
tut es im interaktiven Modus, und der Knopf **Schritte** im Playground, wo
*Teilen* einen Link erzeugt, der beim selben Schritt öffnet: ein Link zum
Projizieren oder zum Verschicken an die Klasse.

`rythme debug datei.ry` hält ein Programm an, wo du es verlangst
(`break 12`, `break fib`), und zeigt die Werte seiner Namen. `where` listet
die laufenden Aufrufe mit ihren Argumenten und sagt, wie viele Aufrufe
derselben Funktion warten: der Stapel einer Rekursion, live. Ein Aufruf in
Endposition sagt, dass er den Platz seines Aufrufers eingenommen hat. In
VS Code startet `F5` auf einer `.ry`-Datei denselben Debugger, mit den
Haltepunkten am Rand. Das
[Handbuch](reference.md#79-the-debugger-rythme-debug) (auf Englisch) zeigt
eine ganze Sitzung.

## Ohne Installation: der Playground

Der Playground der [Website](https://maxlestage.github.io/Rythme/de/)
führt den echten Interpreter im Browser aus, mit denselben Meldungen und
derselben Hilfe: die Lernenden können auf jedem Computer, Tablet oder
Telefon anfangen.

- **Ausführen**, **Prüfen**, **Testen**, **Formatieren** und **Schritte**
  arbeiten wie die Befehle; füge eine Übung ein und drücke **Testen**.
- **Teilen** erzeugt einen Link, der das Programm selbst enthält (nichts
  wird an einen Server geschickt): verteile einen Ausgangspunkt oder das
  Programm der Stunde. **Startwert** gibt allen Lernenden dieselben
  Ziehungen von `random`.
- Als App installiert (**App installieren** im Menü, oder *Zum
  Home-Bildschirm* auf iPhone und iPad) funktionieren die Website und ihr
  Playground offline, was in einem Raum mit schlechtem Netz hilft. Der
  Code, an dem geschrieben wird, bleibt im Browser.
- Im Browser können keine Dateien gelesen oder geschrieben werden,
  `input()` gibt einen leeren Text zurück, und verschachtelte Aufrufe
  enden bei 600 (Endaufrufe haben keine Grenze).

## In der Sprache deiner Lernenden

- Jede Meldung, jedes Hilfethema, jede Erklärung und jede Seite von
  `rythme learn` gibt es auf Englisch, Französisch, Spanisch, Deutsch,
  Portugiesisch und Chinesisch. Rythme folgt der Sprache des Systems;
  `--lang es` oder die Variable `RYTHME_LANG=es` wählen eine andere.
- Schlüsselwörter, Namen der Funktionen und Befehle bleiben in jeder
  Sprache gleich: ein Programm aus einer Klasse läuft überall, und
  `rythme help` versteht die Namen der Themen in jeder Sprache
  (`rythme help listen`).
- Auch die Übungen folgen der Sprache:
  `rythme --lang de exercise start 7` schreibt `07-rekursion.ry`, auf
  Deutsch.
- Die Website, ihren Playground und diesen Leitfaden gibt es in denselben
  Sprachen.

## KI-Assistenten im Unterricht

Die Lernenden haben Assistenten zur Hand. Rythme versucht nicht, sie zu
erkennen; hier steht, was es bietet, und eine Regelung, die ein Kurs
anpassen kann.

**Was Rythme bietet.** Die Assistenten wurden trainiert, bevor es Rythme
gab: allein schreiben sie Gewohnheiten aus Python oder JavaScript (`for`,
`return`, `x += 1`), die Rythme mit einer Meldung ablehnt, die sagt, was
zu schreiben ist. Mit `rythme help llms`, der ganzen Sprache in einem Text
(auch auf der Seite *KI* der Website), schreiben sie korrektes Rythme.
`rythme mcp` lässt einen Assistenten Code selbst ausführen, prüfen und
testen, in einer Sandbox. Im Playground kopiert *Für KI kopieren* das
Programm und seine Ausgabe oder seine Fehler, bereit für einen Chat.
`rythme fix` wendet die sicheren Korrekturen an (`&&` → `and`,
`let x = 3` → `x = 3`), und `rythme fix --dry-run` zeigt sie nur.

**Eine mögliche Regelung.**

- **Erste Wochen: die Lernenden schreiben den Code.** Es geht darum,
  Reflexe aufzubauen, und die Meldungen und `rythme explain` erklären
  schon jeden Fehler. Ein Assistent darf eine Meldung oder eine Idee
  erklären, nicht die Lösung schreiben.
- **`rythme fix`, sobald die Meldung gelesen ist**: es ändert nur, was
  sicher ist, genau die Korrektur, die die Meldung vorschlägt.
- **Projekte: erlaubt, und gesagt.** Gib dem Assistenten
  `rythme help llms`, bitte die Lernenden zu sagen, wofür sie ihn benutzt
  haben, und ihren Code zu erklären.
- **Verständnis bewerten, nicht nur Ergebnisse.** Tests prüfen, dass ein
  Programm funktioniert, nicht wer es geschrieben hat. Füge kurze
  Kontrollen im Unterricht hinzu: die Schritte von `rythme steps`, die
  Ausgabe eines Programms oder die Korrektur einer Meldung vorhersagen,
  ohne Computer; oder eine Übung, die vor Ort im Playground gemacht wird.
- **Für deine eigene Arbeit** kann ein Assistent, dem du
  `rythme help llms` gegeben hast, neue Übungen im Format oben entwerfen.
  Prüfe jede mit einer eigenen Lösung und `rythme test`, bevor du sie
  verteilst.

## Praktische Tipps

- Fang mit dem interaktiven Modus an (`rythme`): jede Zeile zeigt ihren
  Wert. `:doc map` zeigt die Dokumentation einer Funktion, Tab ergänzt
  Namen.
- `rythme learn` ist eine geführte Tour, der die Lernenden allein folgen
  können.
- `rythme help builtins` listet alle eingebauten Funktionen nach
  Kategorie, und jeder Befehl antwortet auf `--help`
  (`rythme test --help`). `rythme completions bash` (oder `zsh`, `fish`,
  `powershell`) ergänzt die Befehle im Terminal.
- Installiere die [VS-Code-Erweiterung](../editors/vscode): Farben,
  Snippets, die Probleme beim Tippen mit ihren Schnellkorrekturen, die
  Dokumentation unter der Maus, und den Debugger. Andere Editoren können
  den Sprachserver auch verwenden
  ([`editors/README.md`](../editors/README.md), auf Englisch).
- `rythme fmt` gibt allen Dateien dasselbe Layout: die Lernenden können
  es (oder `rythme fmt --check .`) vor der Abgabe ausführen.
- Für ein größeres Projekt erzeugt `rythme new NAME` einen Ordner mit
  `src/`, `tests/` und `rythme.toml`; `rythme test` allein führt dann alle
  Tests des Projekts aus, und `rythme add` holt ein Paket aus dem
  [Paketindex](reference.md#711-the-package-index) (auf Englisch).
