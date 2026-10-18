// German: each English text of Rythme with its translation. How to add or
// update a language: CONTRIBUTING.md, "Messages in every language".
Language {
    code: "de",
    name: "Deutsch",
    texts: &[
        // src/main.rs
        ("unknown language `{v}`", "unbekannte Sprache `{v}`"),
        (
            r#"Rythme ran into an internal problem. This is a bug in Rythme, not in your program.
Please report it, with your program, at https://github.com/maxlestage/Rythme/issues"#,
            r#"Rythme ist auf ein internes Problem gestoßen. Das ist ein Fehler in Rythme, nicht in deinem Programm.
Bitte melde ihn, zusammen mit deinem Programm, unter https://github.com/maxlestage/Rythme/issues"#,
        ),
        ("`--lang` needs a language", "`--lang` braucht eine Sprache"),
        ("{what}: write `--lang` and one of {codes}, like `rythme --lang fr hello.ry`", "{what}: schreib `--lang` und einen der Codes {codes}, wie `rythme --lang de hallo.ry`"),
        ("usage: rythme explain CODE   (codes: {})", "Verwendung: rythme explain CODE   (Codes: {})"),
        ("unknown code `{code}`. Known codes: {}", "unbekannter Code `{code}`. Bekannte Codes: {}"),
        ("which file? Example: rythme run hello.ry", "welche Datei? Beispiel: rythme run hallo.ry"),
        ("which file? Example: rythme check hello.ry", "welche Datei? Beispiel: rythme check hallo.ry"),
        ("which code? Example: rythme -e 'print(1 + 2)'", "welcher Code? Beispiel: rythme -e 'print(1 + 2)'"),
        ("unknown command or file `{cmd}`", "unbekannter Befehl oder unbekannte Datei `{cmd}`"),
        (
            r#"
did you mean `rythme {best}`?"#,
            r#"
meintest du `rythme {best}`?"#,
        ),
        (
            r#"
see `rythme help`"#,
            r#"
siehe `rythme help`"#,
        ),
        ("current folder: {}", "aktueller Ordner: {}"),
        ("Rythme files usually end with `.ry` ({})", "Rythme-Dateien enden normalerweise auf `.ry` ({})"),
        ("✓ {shown}: no problems found", "✓ {shown}: keine Probleme gefunden"),
        ("✓ {shown}: no errors, {n} warning(s)", "✓ {shown}: keine Fehler, {n} Warnung(en)"),
        ("✗ {shown}: {n} error(s)", "✗ {shown}: {n} Fehler"),
        // src/bigint.rs
        ("this whole number would be too large (more than {MAX_DIGITS} digits)", "diese ganze Zahl wäre zu groß (mehr als {MAX_DIGITS} Ziffern)"),
        ("does it grow without end, in a function that calls itself? For an approximate value, use a decimal number (Float), like `2.0 ** 1000`", "wächst sie ohne Ende, in einer Funktion, die sich selbst aufruft? Für einen ungefähren Wert verwende eine Dezimalzahl (Float), wie `2.0 ** 1000`"),
        // src/builtins/date.rs
        ("the year must go from 1 to 9999, not {year}", "das Jahr muss zwischen 1 und 9999 liegen, nicht {year}"),
        ("the month must go from 1 to 12, not {month}", "der Monat muss zwischen 1 und 12 liegen, nicht {month}"),
        ("the day must go from 1 to {len} in {name} {year}, not {day}", "der Tag muss im {name} {year} zwischen 1 und {len} liegen, nicht {day}"),
        (" ({year} is not a leap year)", " ({year} ist kein Schaltjahr)"),
        ("the hour must go from 0 to 23, not {hour}", "die Stunde muss zwischen 0 und 23 liegen, nicht {hour}"),
        ("the minute must go from 0 to 59, not {minute}", "die Minute muss zwischen 0 und 59 liegen, nicht {minute}"),
        ("the second must go from 0 to 59, not {second}", "die Sekunde muss zwischen 0 und 59 liegen, nicht {second}"),
        ("the offset must go from -1080 to 1080 minutes (-18:00 to +18:00), not {offset}", "der Versatz muss zwischen -1080 und 1080 Minuten liegen (-18:00 bis +18:00), nicht {offset}"),
        ("a date has no field `{k}`", "ein Datum hat kein Feld `{k}`"),
        (" (did you mean `{best}`?)", " (meintest du `{best}`?)"),
        ("the field `{k}` is too large for a date", "das Feld `{k}` ist zu groß für ein Datum"),
        ("the field `{k}` must be a whole number, not {}", "das Feld `{k}` muss eine ganze Zahl sein, nicht {}"),
        ("the field `{k}` is missing", "das Feld `{k}` fehlt"),
        ("\"{text}\" is not a date written as ISO 8601: write it like \"2024-02-29\" or \"2024-02-29T13:05:00\"", "\"{text}\" ist kein Datum im Format ISO 8601: schreib es wie \"2024-02-29\" oder \"2024-02-29T13:05:00\""),
        ("\"{text}\" is not a valid date: {why}", "\"{text}\" ist kein gültiges Datum: {why}"),
        ("`{f}` needs a valid date: {why}", "`{f}` braucht ein gültiges Datum: {why}"),
        ("not a valid date", "kein gültiges Datum"),
        ("a date is a record like `{{year: 2024, month: 2, day: 29}}`, with `hour`, `minute`, `second` and `offset` (minutes ahead of UTC) if needed", "ein Datum ist ein Record wie `{{year: 2024, month: 2, day: 29}}`, bei Bedarf mit `hour`, `minute`, `second` und `offset` (Minuten vor UTC)"),
        ("`{f}` gives a date outside of the years 1 to 9999", "`{f}` ergibt ein Datum außerhalb der Jahre 1 bis 9999"),
        ("the month must go from 1 to 12, not {m}", "der Monat muss zwischen 1 und 12 liegen, nicht {m}"),
        ("the clock of the system gives a date outside of the years 1 to 9999", "die Systemuhr liefert ein Datum außerhalb der Jahre 1 bis 9999"),
        // src/builtins/dicts.rs
        ("`{f}`: {} cannot be a key of a dict, nor an element of a set", "`{f}`: {} kann weder Schlüssel eines Dicts noch Element einer Menge sein"),
        ("it holds {}", "es enthält {}"),
        ("keys and elements must be values that can be compared: anything but functions and `nan`", "Schlüssel und Elemente müssen vergleichbare Werte sein: alles außer Funktionen und `nan`"),
        ("`{f}` needs a list of [key, value] pairs, but it contains {}", "`{f}` braucht eine Liste von [Schlüssel, Wert]-Paaren, aber sie enthält {}"),
        ("each pair is a list of two elements, like `dict([[\"Ada\", 36], [\"Alan\", 41]])`", "jedes Paar ist eine Liste mit zwei Elementen, wie `dict([[\"Ada\", 36], [\"Alan\", 41]])`"),
        // src/builtins/property.rs
        ("this is not a generator: {}", "das ist kein Generator: {}"),
        ("not a generator", "kein Generator"),
        ("make one with `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` or `records_of({{x: g}})`", "erzeuge einen mit `ints()`, `floats()`, `texts()`, `bools()`, `lists_of(g)`, `one_of(list)` oder `records_of({{x: g}})`"),
        ("generators can be put inside each other {MAX_DEPTH} times at most", "Generatoren können höchstens {MAX_DEPTH}-mal ineinander verschachtelt werden"),
        ("its field `{what}` is missing or wrong", "sein Feld `{what}` fehlt oder ist falsch"),
        ("`{f}` takes no argument, or two: the smallest and the largest value, like `{f}(0, 10)`", "`{f}` nimmt kein Argument oder zwei: den kleinsten und den größten Wert, wie `{f}(0, 10)`"),
        ("{f}({}, {}): the minimum is larger than the maximum", "{f}({}, {}): das Minimum ist größer als das Maximum"),
        ("`floats` needs finite numbers, not {} and {}", "`floats` braucht endliche Zahlen, nicht {} und {}"),
        ("`{f}` needs a length from 0 to {MAX_LENGTH}, not {n}", "`{f}` braucht eine Länge von 0 bis {MAX_LENGTH}, nicht {n}"),
        ("`one_of` needs at least one value to choose from", "`one_of` braucht mindestens einen Wert zur Auswahl"),
        ("the field `{k}` given to `records_of` must be a generator, like `ints()`, not {}", "das an `records_of` übergebene Feld `{k}` muss ein Generator sein, wie `ints()`, nicht {}"),
        ("`check_all` tries from 1 to {MAX_CASES} values, not {cases}", "`check_all` probiert 1 bis {MAX_CASES} Werte aus, nicht {cases}"),
        ("the property is false for {v}", "die Eigenschaft ist falsch für {v}"),
        ("false for {v}", "falsch für {v}"),
        ("the property gives {r} for {v}", "die Eigenschaft ergibt {r} für {v}"),
        ("{r} for {v}", "{r} für {v}"),
        ("the property fails for {v}: {}", "die Eigenschaft scheitert für {v}: {}"),
        ("checked here", "hier geprüft"),
        ("found on try {case} of {cases}", "gefunden bei Versuch {case} von {cases}"),
        ("found on try {case} of {cases}, then simplified from {} in {steps} step(s)", "gefunden bei Versuch {case} von {cases}, dann von {} in {steps} Schritt(en) vereinfacht"),
        ("seed {seed}: every run tries the same values; `rythme test --seed N` tries others", "Seed {seed}: jeder Lauf probiert dieselben Werte; `rythme test --seed N` probiert andere"),
        ("the function must hold for every value of the generator: fix the function, or give the generator a smaller range, like `ints(1, 100)`", "die Funktion muss für jeden Wert des Generators gelten: korrigiere die Funktion oder gib dem Generator einen kleineren Bereich, wie `ints(1, 100)`"),
        // src/builtins/regex.rs
        ("for the sign `{c}` itself, write `[{c}]` (in a Rythme text: \"[\\{c}]\")", "für das Zeichen `{c}` selbst schreib `[{c}]` (in einem Rythme-Text: \"[\\{c}]\")"),
        ("for the sign `{c}` itself, write `[{c}]`", "für das Zeichen `{c}` selbst schreib `[{c}]`"),
        ("`{q}` has nothing to repeat", "`{q}` hat nichts zu wiederholen"),
        ("nothing before it to repeat", "nichts davor, das wiederholt werden kann"),
        ("put it after a character or a group, like `a{q}` or `(ab){q}`; {}", "setze es hinter ein Zeichen oder eine Gruppe, wie `a{q}` oder `(ab){q}`; {}"),
        ("`{{` must start a count, like {{3}} or {{2,5}}", "`{{` muss eine Anzahl beginnen, wie {{3}} oder {{2,5}}"),
        ("not a count", "keine Anzahl"),
        ("{}; in a Rythme text, braces are written `\\{{` and `\\}}`, like \"[0-9]\\{{4\\}}\"", "{}; in einem Rythme-Text schreibt man geschweifte Klammern als `\\{{` und `\\}}`, wie \"[0-9]\\{{4\\}}\""),
        ("a count can be at most {MAX_COUNT}", "eine Anzahl darf höchstens {MAX_COUNT} sein"),
        ("too large", "zu groß"),
        ("for more, use `*` or `+`, or {{n,}} without a maximum", "für mehr verwende `*` oder `+`, oder {{n,}} ohne Maximum"),
        ("the count goes backwards: {min} is more than {max}", "die Anzahl läuft rückwärts: {min} ist größer als {max}"),
        ("smallest first", "kleinste zuerst"),
        ("write the smallest number first: {{{max},{min}}}", "schreib die kleinere Zahl zuerst: {{{max},{min}}}"),
        ("a repetition cannot be repeated again", "eine Wiederholung kann nicht noch einmal wiederholt werden"),
        ("repeats a repetition", "wiederholt eine Wiederholung"),
        ("put the repeated part in parentheses, like `(a+)*`, or keep one sign, like `a*`; {}", "setze den wiederholten Teil in Klammern, wie `(a+)*`, oder behalte ein Zeichen, wie `a*`; {}"),
        ("the groups are nested too deeply (more than {MAX_DEPTH})", "die Gruppen sind zu tief verschachtelt (mehr als {MAX_DEPTH})"),
        ("too deep", "zu tief"),
        ("`(?<` must be followed by a name and `>`", "auf `(?<` müssen ein Name und `>` folgen"),
        ("not a group name", "kein Gruppenname"),
        ("a named group looks like `(?<year>[0-9]+)`: the name has letters, digits and `_`", "eine benannte Gruppe sieht so aus: `(?<year>[0-9]+)`; der Name besteht aus Buchstaben, Ziffern und `_`"),
        ("two groups are named `{name}`", "zwei Gruppen heißen `{name}`"),
        ("already used", "schon verwendet"),
        ("give each group its own name", "gib jeder Gruppe ihren eigenen Namen"),
        ("`(?i)` must be at the very start of the regex", "`(?i)` muss ganz am Anfang des regulären Ausdrucks stehen"),
        ("not at the start", "nicht am Anfang"),
        ("`(?i)` ignores the case in the whole regex: write it first", "`(?i)` ignoriert Groß- und Kleinschreibung im ganzen regulären Ausdruck: schreib es zuerst"),
        ("unknown kind of group", "unbekannte Art von Gruppe"),
        ("unknown", "unbekannt"),
        ("the groups are `(...)`, `(?:...)` (a group that captures nothing) and `(?<name>...)` (a named group); `(?i)` at the start ignores the case", "die Gruppen sind `(...)`, `(?:...)` (eine Gruppe, die nichts erfasst) und `(?<name>...)` (eine benannte Gruppe); `(?i)` am Anfang ignoriert Groß- und Kleinschreibung"),
        ("this `(` is never closed", "diese `(` wird nie geschlossen"),
        ("opened here", "hier geöffnet"),
        ("add a `)` where the group ends; {}", "füge eine `)` dort ein, wo die Gruppe endet; {}"),
        ("the regex ends with a lone `\\`", "der reguläre Ausdruck endet mit einem einzelnen `\\`"),
        ("followed by nothing", "gefolgt von nichts"),
        ("a `\\` comes before a letter like d, w, s or b, or before a sign for the sign itself, like `\\.`", "ein `\\` steht vor einem Buchstaben wie d, w, s oder b, oder vor einem Zeichen für das Zeichen selbst, wie `\\.`"),
        ("`\\{c}` means nothing inside `[...]`", "`\\{c}` bedeutet nichts innerhalb von `[...]`"),
        ("not a character", "kein Zeichen"),
        ("`\\b` is the edge of a word: write it outside of the brackets", "`\\b` ist der Rand eines Wortes: schreib es außerhalb der eckigen Klammern"),
        ("`\\{c}` means nothing in a regex", "`\\{c}` bedeutet nichts in einem regulären Ausdruck"),
        (
            r#"the known ones: \d (a digit), \w (a letter, a digit or _), \s (a space), \b (the edge of a word),
their opposites \D, \W, \S, \B, \n, \t, and `\` before a sign for the sign itself, like \. or \("#,
            r#"die bekannten: \d (eine Ziffer), \w (ein Buchstabe, eine Ziffer oder _), \s (ein Leerzeichen), \b (der Rand eines Wortes),
ihre Gegenteile \D, \W, \S, \B, \n, \t, und `\` vor einem Zeichen für das Zeichen selbst, wie \. oder \("#,
        ),
        ("this `[` is never closed", "diese `[` wird nie geschlossen"),
        ("add a `]` after the characters; for the sign `]` itself, put it first: `[]]`", "füge nach den Zeichen eine `]` ein; für das Zeichen `]` selbst setze es an den Anfang: `[]]`"),
        ("a range must end with one character", "ein Bereich muss mit einem einzelnen Zeichen enden"),
        ("not a range", "kein Bereich"),
        ("for the sign `-` itself, put it last: `[a-]`", "für das Zeichen `-` selbst setze es ans Ende: `[a-]`"),
        ("the range `{lo}-{hi}` goes backwards", "der Bereich `{lo}-{hi}` läuft rückwärts"),
        ("backwards", "rückwärts"),
        ("write the first character first: `{hi}-{lo}`", "schreib das erste Zeichen zuerst: `{hi}-{lo}`"),
        ("this `)` closes nothing", "diese `)` schließt nichts"),
        ("no `(` before it", "keine `(` davor"),
        ("the regex is too large once its repetitions are written out", "der reguläre Ausdruck ist zu groß, wenn man seine Wiederholungen ausschreibt"),
        ("use smaller counts in {{n,m}}, or `*` and `+`, which cost nothing", "verwende kleinere Anzahlen in {{n,m}}, oder `*` und `+`, die nichts kosten"),
        ("`$1` is the text of group 1, `$<name>` of a named group, `$0` the whole match; for the sign `$` itself, write `$$`", "`$1` ist der Text von Gruppe 1, `$<name>` der einer benannten Gruppe, `$0` der ganze Treffer; für das Zeichen `$` selbst schreib `$$`"),
        ("the regex has no group", "der reguläre Ausdruck hat keine Gruppe"),
        ("the regex has 1 group", "der reguläre Ausdruck hat 1 Gruppe"),
        ("the regex has {g} groups", "der reguläre Ausdruck hat {g} Gruppen"),
        ("there is no group {n}: {has}", "es gibt keine Gruppe {n}: {has}"),
        ("no such group", "keine solche Gruppe"),
        ("`$<` must be followed by a group name and `>`", "auf `$<` müssen ein Gruppenname und `>` folgen"),
        ("never closed", "nie geschlossen"),
        ("the regex has no named group", "der reguläre Ausdruck hat keine benannte Gruppe"),
        ("its named groups are {}", "seine benannten Gruppen sind {}"),
        ("the regex has no group named `{name}`: {has}", "der reguläre Ausdruck hat keine Gruppe namens `{name}`: {has}"),
        ("unknown group", "unbekannte Gruppe"),
        ("`$` must be followed by a group number, `<name>` or `$`", "auf `$` muss eine Gruppennummer, `<name>` oder `$` folgen"),
        ("a lone `$`", "ein einzelnes `$`"),
        ("invalid replacement: {}", "ungültige Ersetzung: {}"),
        ("invalid regex: {}", "ungültiger regulärer Ausdruck: {}"),
        ("with this replacement", "mit dieser Ersetzung"),
        ("with this regex", "mit diesem regulären Ausdruck"),
        ("here in the replacement:", "hier in der Ersetzung:"),
        ("here in the regex:", "hier im regulären Ausdruck:"),
        ("`named_captures` needs a regex with named groups", "`named_captures` braucht einen regulären Ausdruck mit benannten Gruppen"),
        ("name a group like this: `(?<year>[0-9]+)`; for groups without names, use `captures`", "benenne eine Gruppe so: `(?<year>[0-9]+)`; für Gruppen ohne Namen verwende `captures`"),
        // src/builtins/system.rs
        ("`{f}` is not available in the browser: a program here has no input to read. Run it on your computer: rythme file.ry < lines.txt", "`{f}` gibt es im Browser nicht: ein Programm hat hier keine Eingabe zum Lesen. Führe es auf deinem Computer aus: rythme datei.ry < zeilen.txt"),
        ("`{f}` is not available in the browser: a program here has no files or folders. Run it on your computer: rythme file.ry", "`{f}` gibt es im Browser nicht: ein Programm hat hier keine Dateien oder Ordner. Führe es auf deinem Computer aus: rythme datei.ry"),
        ("the path is empty: write \".\" for the current folder", "der Pfad ist leer: schreib \".\" für den aktuellen Ordner"),
        ("folder not found: {path}", "Ordner nicht gefunden: {path}"),
        ("file not found: {path}", "Datei nicht gefunden: {path}"),
        ("permission denied: {path}", "Zugriff verweigert: {path}"),
        ("cannot access {path}: {e}", "kein Zugriff auf {path}: {e}"),
        ("{path} is a file, not a folder: `read_file` reads a file", "{path} ist eine Datei, kein Ordner: `read_file` liest eine Datei"),
        ("{path} already exists, and it is a file, not a folder", "{path} existiert schon, und es ist eine Datei, kein Ordner"),
        ("{path} is a folder: `delete_file` deletes only files, never folders", "{path} ist ein Ordner: `delete_file` löscht nur Dateien, niemals Ordner"),
        ("cannot read the standard input: {e}", "die Standardeingabe kann nicht gelesen werden: {e}"),
        ("`join_path` needs a list of texts, but it contains {}", "`join_path` braucht eine Liste von Texten, aber sie enthält {}"),
        ("`text(value)` turns a value into a text", "`text(value)` macht aus einem Wert einen Text"),
        ("`join_path` cannot add \"{part}\" after \"{out}\": only the first part of a path can start at a root (`/`, `\\` or a drive like `C:`)", "`join_path` kann \"{part}\" nicht nach \"{out}\" anfügen: nur der erste Teil eines Pfads darf an einer Wurzel beginnen (`/`, `\\` oder ein Laufwerk wie `C:`)"),
        ("remove the separator at the start of \"{part}\", or put it first in the list", "entferne das Trennzeichen am Anfang von \"{part}\", oder setze den Teil an den Anfang der Liste"),
        ("`{w}` is not a valid option: its name starts with a letter, followed by letters, digits, `-` or `_` (write `--` before words that start with `-`)", "`{w}` ist keine gültige Option: ihr Name beginnt mit einem Buchstaben, gefolgt von Buchstaben, Ziffern, `-` oder `_` (schreib `--` vor Wörter, die mit `-` beginnen)"),
        ("`{w}` is not an option that Rythme understands: write `--{name}` for a long name, or `-a -b` for several one-letter options", "`{w}` ist keine Option, die Rythme versteht: schreib `--{name}` für einen langen Namen, oder `-a -b` für mehrere Optionen aus einem Buchstaben"),
        ("`{w}` cannot be an option: the field `args` holds the words that are not options", "`{w}` kann keine Option sein: das Feld `args` enthält die Wörter, die keine Optionen sind"),
        ("the option `{name}` is given twice", "die Option `{name}` wird zweimal angegeben"),
        ("`options` needs a list of texts, like the one of `args()`, but it contains {}", "`options` braucht eine Liste von Texten, wie die von `args()`, aber sie enthält {}"),
        // src/builtins/text_tools.rs
        ("`{f}` writes at most 100 decimals, not {d}", "`{f}` schreibt höchstens 100 Nachkommastellen, nicht {d}"),
        ("`{f}` does not know the language \"{l}\"", "`{f}` kennt die Sprache \"{l}\" nicht"),
        ("the languages: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" and \"pt\" (1.234,5)", "die Sprachen: \"en\" (1,234.5), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" und \"pt\" (1.234,5)"),
        // src/builtins.rs
        ("Basics", "Grundlagen"),
        ("Numbers", "Zahlen"),
        ("Text", "Text"),
        ("Lists", "Listen"),
        ("Records", "Records"),
        ("Dictionaries and sets", "Dicts und Mengen"),
        ("Option and Result", "Option und Result"),
        ("Files", "Dateien"),
        ("Dates and times", "Datum und Uhrzeit"),
        ("Property tests", "Eigenschaftstests"),
        ("usage: `{}`", "Verwendung: `{}`"),
        ("a list", "eine Liste"),
        ("a text", "einen Text"),
        ("a whole number (Int)", "eine ganze Zahl (Int)"),
        ("a number", "eine Zahl"),
        ("a function", "eine Funktion"),
        ("a record", "einen Record"),
        ("true or false", "true oder false"),
        ("a list or a text", "eine Liste oder einen Text"),
        ("a text or a list", "einen Text oder eine Liste"),
        ("a text, a list, a record, a dict or a set", "einen Text, eine Liste, einen Record, ein Dict oder eine Menge"),
        ("a list, a text, a record or a dict", "eine Liste, einen Text, einen Record oder ein Dict"),
        ("a record or a dict", "einen Record oder ein Dict"),
        ("a record, a dict or a set", "einen Record, ein Dict oder eine Menge"),
        ("a list, a dict or a set", "eine Liste, ein Dict oder eine Menge"),
        ("a list or a set", "eine Liste oder eine Menge"),
        ("a dict or a set", "ein Dict oder eine Menge"),
        ("a dict", "ein Dict"),
        ("a set", "eine Menge"),
        ("a list of [key, value] pairs (or a record)", "eine Liste von [Schlüssel, Wert]-Paaren (oder einen Record)"),
        ("a list (or several values)", "eine Liste (oder mehrere Werte)"),
        ("a number or a text", "eine Zahl oder einen Text"),
        ("a list, a set, an Option or a Result", "eine Liste, eine Menge, eine Option oder ein Result"),
        ("an Option or a Result", "eine Option oder ein Result"),
        ("an Option", "eine Option"),
        ("a Result", "ein Result"),
        ("a date (a record {{year, month, day}})", "ein Datum (einen Record {{year, month, day}})"),
        ("a generator (like `ints()`)", "einen Generator (wie `ints()`)"),
        ("a text or a function", "einen Text oder eine Funktion"),
        ("`{f}` expects {expected} as argument {pos}, but got {got}", "`{f}` erwartet {expected} als Argument {pos}, erhalten: {got}"),
        ("wrong kind of value", "falsche Art von Wert"),
        ("a text is not a list: `chars(text)` gives the list of its characters", "ein Text ist keine Liste: `chars(text)` gibt die Liste seiner Zeichen"),
        ("a record is not a list: `entries(record)`, `keys(record)` or `values(record)` give one", "ein Record ist keine Liste: `entries(record)`, `keys(record)` oder `values(record)` geben eine"),
        ("a dict is not a list: `entries(dict)`, `keys(dict)` or `values(dict)` give one", "ein Dict ist keine Liste: `entries(dict)`, `keys(dict)` oder `values(dict)` geben eine"),
        ("a set is not a list: `values(set)` gives the list of its elements, in order", "eine Menge ist keine Liste: `values(set)` gibt die Liste ihrer Elemente, der Reihe nach"),
        ("to change the values of a dict, use `map_values(dict, f)`", "um die Werte eines Dicts zu ändern, verwende `map_values(dict, f)`"),
        ("for a list, use `{cousin}`", "für eine Liste verwende `{cousin}`"),
        ("to read a date from a text like \"2024-02-29\", use `parse_date(text)`", "um ein Datum aus einem Text wie \"2024-02-29\" zu lesen, verwende `parse_date(text)`"),
        ("for a list, use `find(list, f)`", "für eine Liste verwende `find(list, f)`"),
        ("for an Option or a Result, use `and_then(x, f)`", "für eine Option oder ein Result verwende `and_then(x, f)`"),
        ("the function given to `{f}` must return a list, but it returned {got}", "die an `{f}` übergebene Funktion muss eine Liste zurückgeben, hat aber {got} zurückgegeben"),
        ("the function given to `{f}` must return a text, but it returned {got}", "die an `{f}` übergebene Funktion muss einen Text zurückgeben, hat aber {got} zurückgegeben"),
        ("to make a text of any value, use `text(x)`", "um aus einem beliebigen Wert einen Text zu machen, verwende `text(x)`"),
        ("an Option (Some or None)", "eine Option (Some oder None)"),
        ("a Result (Ok or Err)", "ein Result (Ok oder Err)"),
        ("the function given to `{f}` must return {expected}, but it returned {got}", "die an `{f}` übergebene Funktion muss {expected} zurückgeben, hat aber {got} zurückgegeben"),
        ("to change the value inside without another check, use `map(x, f)`", "um den Wert darin ohne weitere Prüfung zu ändern, verwende `map(x, f)`"),
        ("the function given to `{f}` must return true or false, but it returned {got}", "die an `{f}` übergebene Funktion muss true oder false zurückgeben, hat aber {got} zurückgegeben"),
        ("the number {n} is too large for `{f}`", "die Zahl {n} ist zu groß für `{f}`"),
        ("{digits} digits", "{digits} Ziffern"),
        ("`{f}` needs a number of 0 or more, not {n}", "`{f}` braucht eine Zahl ab 0, nicht {n}"),
        ("`{f}` needs a number of 1 or more, not 0", "`{f}` braucht eine Zahl ab 1, nicht 0"),
        ("the list is empty", "die Liste ist leer"),
        ("the list has {len} element(s)", "die Liste hat {len} Element(e)"),
        ("position {n} is out of range: {what}", "Position {n} liegt außerhalb des Bereichs: {what}"),
        ("no element here", "hier ist kein Element"),
        ("valid positions go from 0 to {last} (or from -1 backwards, counting from the end)", "gültige Positionen gehen von 0 bis {last} (oder von -1 rückwärts, vom Ende her gezählt)"),
        ("at least {min}", "mindestens {min}"),
        ("{min} to {max}", "{min} bis {max}"),
        ("`{f}` gives {given} argument(s) to this function, but it takes {expected}", "`{f}` gibt dieser Funktion {given} Argument(e), aber sie nimmt {expected}"),
        ("takes {expected} argument(s)", "nimmt {expected} Argument(e)"),
        ("`nan` (not a number) cannot be converted to a whole number", "`nan` (keine Zahl) kann nicht in eine ganze Zahl umgewandelt werden"),
        ("`nan` is the result of a calculation that has none, like `0.0 * inf`: check where this value comes from", "`nan` ist das Ergebnis einer Rechnung, die keins hat, wie `0.0 * inf`: prüfe, woher dieser Wert kommt"),
        ("`{}` (infinity) cannot be converted to a whole number", "`{}` (unendlich) kann nicht in eine ganze Zahl umgewandelt werden"),
        ("an infinity comes from a calculation too large for decimal numbers, like `1e308 * 10.0`, or from a division by `0.0`", "eine Unendlichkeit kommt von einer Rechnung, die zu groß für Dezimalzahlen ist, wie `1e308 * 10.0`, oder von einer Division durch `0.0`"),
        ("`text` writes at most 100 decimals, not {digits}", "`text` schreibt höchstens 100 Nachkommastellen, nicht {digits}"),
        ("`input` found the end of the input", "`input` hat das Ende der Eingabe erreicht"),
        ("no more lines to read", "keine Zeilen mehr zu lesen"),
        ("the keyboard input was closed (Ctrl-D), or the text sent to the program has ended", "die Tastatureingabe wurde geschlossen (Strg-D), oder der an das Programm geschickte Text ist zu Ende"),
        ("assertion failed: {}", "Zusicherung fehlgeschlagen: {}"),
        ("assertion failed", "Zusicherung fehlgeschlagen"),
        ("this condition is false", "diese Bedingung ist falsch"),
        (
            r#"got:      {got}
expected: {expected}"#,
            r#"erhalten: {got}
erwartet: {expected}"#,
        ),
        ("first difference: character {at}", "erster Unterschied: Zeichen {at}"),
        ("assertion failed: the values are different", "Zusicherung fehlgeschlagen: die Werte sind verschieden"),
        (" at {path}", " bei {path}"),
        ("the list{at} has {} element(s), {} expected", "die Liste{at} hat {} Element(e), erwartet: {}"),
        ("in excess", "zu viel"),
        ("missing", "fehlend"),
        ("{word} from [{from}]: {}{dots}", "{word} ab [{from}]: {}{dots}"),
        ("missing field(s){at}: {}", "fehlende(s) Feld(er){at}: {}"),
        ("field(s) not expected{at}: {}", "nicht erwartete(s) Feld(er){at}: {}"),
        (", which are of different types: {} and {}", ", die verschiedene Typen haben: {} und {}"),
        ("first difference at {path}: got {}, expected {}{kinds}", "erster Unterschied bei {path}: erhalten {}, erwartet {}{kinds}"),
        ("the values are of different types: {} and {}", "die Werte haben verschiedene Typen: {} und {}"),
        ("(no line)", "(keine Zeile)"),
        (
            r#"first different line: line {}
  got:      {}
  expected: {}"#,
            r#"erste unterschiedliche Zeile: Zeile {}
  erhalten: {}
  erwartet: {}"#,
        ),
        ("the program stopped with `fail`", "das Programm wurde mit `fail` beendet"),
        ("the program stopped here", "das Programm wurde hier beendet"),
        ("an exit code goes from 0 to 255, not {code}", "ein Exit-Code liegt zwischen 0 und 255, nicht {code}"),
        ("the program stopped with exit({code})", "das Programm wurde mit exit({code}) beendet"),
        ("cannot pick a random element in an empty list", "aus einer leeren Liste kann kein zufälliges Element gewählt werden"),
        ("random({lo}, {hi}): the minimum is larger than the maximum", "random({lo}, {hi}): das Minimum ist größer als das Maximum"),
        ("`{name}` of an empty list", "`{name}` einer leeren Liste"),
        ("`{name}` cannot compare {} and {}", "`{name}` kann {} und {} nicht vergleichen"),
        ("cannot take the square root of a negative number ({x})", "die Quadratwurzel einer negativen Zahl ({x}) gibt es nicht"),
        ("use `sqrt(abs(x))` if you want the root of its absolute value", "verwende `sqrt(abs(x))`, wenn du die Wurzel ihres Betrags willst"),
        ("cannot convert \"{s}\" to a whole number", "\"{s}\" kann nicht in eine ganze Zahl umgewandelt werden"),
        ("if the text may not be a number, use `parse_int(t)`: it gives `Ok(n)` or `Err(...)` that you can check", "wenn der Text vielleicht keine Zahl ist, verwende `parse_int(t)`: es gibt `Ok(n)` oder `Err(...)`, was du prüfen kannst"),
        ("cannot convert \"{s}\" to a decimal number", "\"{s}\" kann nicht in eine Dezimalzahl umgewandelt werden"),
        ("if the text may not be a number, use `parse_float(t)`, which gives `Ok(x)` or `Err(...)`", "wenn der Text vielleicht keine Zahl ist, verwende `parse_float(t)`, das `Ok(x)` oder `Err(...)` gibt"),
        ("`{name}` needs a number from -1 to 1, not {x}", "`{name}` braucht eine Zahl von -1 bis 1, nicht {x}"),
        ("the logarithm only exists for positive numbers, not {x}", "den Logarithmus gibt es nur für positive Zahlen, nicht für {x}"),
        ("invalid logarithm base: {base}", "ungültige Basis für den Logarithmus: {base}"),
        ("`{f}` needs a single character, but \"{s}\" has {}", "`{f}` braucht ein einzelnes Zeichen, aber \"{s}\" hat {}"),
        ("`replace` cannot replace an empty text", "`replace` kann keinen leeren Text ersetzen"),
        ("{n} is not the number of a character", "{n} ist keine Zeichennummer"),
        ("character numbers go from 0 to 1114111, except 55296 to 57343", "Zeichennummern gehen von 0 bis 1114111, außer 55296 bis 57343"),
        ("the repeated text would be too large", "der wiederholte Text wäre zu groß"),
        ("the repeated list would be too large", "die wiederholte Liste wäre zu groß"),
        ("this number has more than {} digits", "diese Zahl hat mehr als {} Ziffern"),
        ("\"{s}\" is not a whole number", "\"{s}\" ist keine ganze Zahl"),
        ("\"{s}\" is not a number", "\"{s}\" ist keine Zahl"),
        ("JSON, line {line}, column {column}: {msg}", "JSON, Zeile {line}, Spalte {column}: {msg}"),
        ("expected {what}, but found `{c}`", "erwartet: {what}, gefunden: `{c}`"),
        ("expected {what}, but the text ends", "erwartet: {what}, aber der Text endet"),
        ("a value", "ein Wert"),
        ("the end of the text", "das Ende des Textes"),
        ("`,` or `]`", "`,` oder `]`"),
        ("`,` or `}}`", "`,` oder `}}`"),
        ("a name in quotes", "ein Name in Anführungszeichen"),
        ("invalid number: JSON writes numbers like 0, -12, 3.5 or 1e-3", "ungültige Zahl: JSON schreibt Zahlen wie 0, -12, 3.5 oder 1e-3"),
        ("invalid escape: after `\\` comes one of \" \\ / b f n r t u", "ungültiges Escape: nach `\\` kommt eines von \" \\ / b f n r t u"),
        ("a line break or a tab inside a text is written with `\\`: `\\n`, `\\t`", "ein Zeilenumbruch oder ein Tabulator in einem Text wird mit `\\` geschrieben: `\\n`, `\\t`"),
        ("a text is never closed: `\"` is missing", "ein Text wird nie geschlossen: `\"` fehlt"),
        ("`\\u` must be followed by 4 hexadecimal digits", "auf `\\u` müssen 4 Hexadezimalziffern folgen"),
        ("invalid `\\u` escape: half of a character (a surrogate) without its other half", "ungültiges `\\u`-Escape: die Hälfte eines Zeichens (ein Surrogat) ohne seine andere Hälfte"),
        ("`to_json` indents by 16 spaces at most, not {n}", "`to_json` rückt um höchstens 16 Leerzeichen ein, nicht {n}"),
        ("the JSON text would be too large", "der JSON-Text wäre zu groß"),
        ("the values are nested too deeply to be indented: write them on one line, with `to_json(value)`", "die Werte sind zu tief verschachtelt, um eingerückt zu werden: schreib sie in eine Zeile, mit `to_json(value)`"),
        ("`to_json` cannot write {} in JSON", "`to_json` kann {} nicht in JSON schreiben"),
        ("JSON has no infinite numbers and no NaN", "JSON hat keine unendlichen Zahlen und kein NaN"),
        (
            r#"a dict is written as a JSON object only when its keys are all texts, or all symbols:
turn the keys into texts first, or write `entries(dict)`"#,
            r#"ein Dict wird nur dann als JSON-Objekt geschrieben, wenn seine Schlüssel alle Texte oder alle Symbole sind:
mach die Schlüssel zuerst zu Texten, oder schreib `entries(dict)`"#,
        ),
        (
            r#"JSON only holds numbers, texts, true and false, lists, records and null (`None`):
turn your value into a record first, like {{kind: "Circle", radius: r}}"#,
            r#"JSON enthält nur Zahlen, Texte, true und false, Listen, Records und null (`None`):
mach deinen Wert zuerst zu einem Record, wie {{kind: "Circle", radius: r}}"#,
        ),
        ("`sum` needs a list of numbers, but it contains {}", "`sum` braucht eine Liste von Zahlen, aber sie enthält {}"),
        ("`product` needs a list of numbers, but it contains {}", "`product` braucht eine Liste von Zahlen, aber sie enthält {}"),
        ("all the elements must be of the same kind: all numbers, or all texts...", "alle Elemente müssen von derselben Art sein: alles Zahlen, oder alles Texte..."),
        ("the result of `windows` would be too large", "das Ergebnis von `windows` wäre zu groß"),
        ("the function `key` must give values of the same kind for every element: all numbers, or all texts...", "die Funktion `key` muss für jedes Element Werte derselben Art liefern: alles Zahlen, oder alles Texte..."),
        ("`flatten` needs a list of lists, but it contains {}", "`flatten` braucht eine Liste von Listen, aber sie enthält {}"),
        ("`from_entries` needs a list of [name, value] pairs, but it contains {}", "`from_entries` braucht eine Liste von [Name, Wert]-Paaren, aber sie enthält {}"),
        ("each pair is a list of two elements whose name is a text, like [\"age\", 36]", "jedes Paar ist eine Liste mit zwei Elementen, deren Name ein Text ist, wie [\"age\", 36]"),
        ("`unwrap` found `None`: there is no value", "`unwrap` hat `None` gefunden: es gibt keinen Wert"),
        ("handle the missing value with `match`, or give a default: `unwrap_or(x, default)`", "behandle den fehlenden Wert mit `match`, oder gib einen Standardwert an: `unwrap_or(x, default)`"),
        ("`unwrap` found an error: {}", "`unwrap` hat einen Fehler gefunden: {}"),
        ("handle the error with `match`, pass it on with `?`, or give a default: `unwrap_or(x, default)`", "behandle den Fehler mit `match`, gib ihn mit `?` weiter, oder gib einen Standardwert an: `unwrap_or(x, default)`"),
        // src/check/needs.rs
        ("`{name}` stops with this error, every time", "`{name}` endet jedes Mal mit diesem Fehler"),
        ("here", "hier"),
        ("in `{name}`: {label}", "in `{name}`: {label}"),
        ("`{name}` always fails with {}", "`{name}` scheitert immer mit {}"),
        ("fails every time", "scheitert jedes Mal"),
        ("defined here", "hier definiert"),
        ("from how `{name}` uses `{p}`, it needs: `{sig}`", "so wie `{name}` den Parameter `{p}` verwendet, braucht es: `{sig}`"),
        // src/check.rs
        ("the type `{name}` is part of Rythme", "der Typ `{name}` gehört zu Rythme"),
        ("give your type another name", "gib deinem Typ einen anderen Namen"),
        ("the type `{}` is already defined", "der Typ `{}` ist schon definiert"),
        ("first defined here", "zuerst hier definiert"),
        ("give it another name", "gib ihm einen anderen Namen"),
        ("the field `{f}` appears twice in `{}`", "das Feld `{f}` kommt in `{}` zweimal vor"),
        ("`{s}` is already the name of the file `{file}`", "`{s}` ist schon der Name der Datei `{file}`"),
        ("choose another name here, or give the file another one: `use ... as other_name`", "wähle hier einen anderen Namen, oder gib der Datei einen anderen: `use ... as other_name`"),
        ("`{s}` is already defined", "`{s}` ist schon definiert"),
        ("defined again here", "hier noch einmal definiert"),
        ("a constructor belongs to a single type: rename one of them", "ein Konstruktor gehört zu einem einzigen Typ: benenne einen von ihnen um"),
        ("to give several definitions to a function (one per case), write them one right after the other", "um einer Funktion mehrere Definitionen zu geben (eine pro Fall), schreib sie direkt hintereinander"),
        ("`{s}` hides the built-in function `{s}`", "`{s}` verdeckt die eingebaute Funktion `{s}`"),
        ("`{s}` hides the built-in value `{s}`", "`{s}` verdeckt den eingebauten Wert `{s}`"),
        ("in this file, `{s}` will now mean your definition; choose another name to keep the built-in one", "in dieser Datei bedeutet `{s}` jetzt deine Definition; wähle einen anderen Namen, um die eingebaute zu behalten"),
        ("`{s}` is part of Rythme", "`{s}` gehört zu Rythme"),
        ("choose another name", "wähle einen anderen Namen"),
        ("the result of `{name}` is lost", "das Ergebnis von `{name}` geht verloren"),
        ("this value is lost", "dieser Wert geht verloren"),
        (
            r#"values never change: `{sig}` gives a new value, and changes nothing.
To keep it, give it a name: `new = {sig}`"#,
            r#"Werte ändern sich nie: `{sig}` liefert einen neuen Wert und ändert nichts.
Um ihn zu behalten, gib ihm einen Namen: `new = {sig}`"#,
        ),
        ("this test can never fail", "dieser Test kann nie fehlschlagen"),
        ("its result, {}, is never compared with what is expected", "sein Ergebnis, {}, wird nie mit dem Erwarteten verglichen"),
        (
            r#"a test fails only when it stops with an error, or ends with `false`, `None` or `Err(...)`;
nothing in this one can"#,
            r#"ein Test schlägt nur fehl, wenn er mit einem Fehler endet oder `false`, `None` oder `Err(...)` ergibt;
nichts in diesem Test kann das"#,
        ),
        ("compare the result with what you expect: `assert_eq(actual, expected)`", "vergleiche das Ergebnis mit dem, was du erwartest: `assert_eq(actual, expected)`"),
        ("remove `{name}` from the pattern, or write `{name}: _{name}` if this is on purpose", "entferne `{name}` aus dem Muster, oder schreib `{name}: _{name}`, wenn das Absicht ist"),
        ("remove it, or name it `_{name}` if this is on purpose", "entferne es, oder nenne es `_{name}`, wenn das Absicht ist"),
        ("`{name}` is never used", "`{name}` wird nie verwendet"),
        ("`{s}` appears twice in this pattern", "`{s}` kommt in diesem Muster zweimal vor"),
        ("first here", "zuerst hier"),
        ("use two different names, then compare them with `==` in a condition: `[a, b] if a == b -> ...`", "verwende zwei verschiedene Namen und vergleiche sie dann mit `==` in einer Bedingung: `[a, b] if a == b -> ...`"),
        ("`{s}` is used before being defined", "`{s}` wird verwendet, bevor es definiert ist"),
        ("used here", "hier verwendet"),
        ("defined later, here", "später definiert, hier"),
        ("move the definition above this line", "verschiebe die Definition über diese Zeile"),
        ("`{s}` is the name of the file `{file}`, not a value", "`{s}` ist der Name der Datei `{file}`, kein Wert"),
        ("write `{s}.` and the name of one of its definitions, like `{s}.{example}`", "schreib `{s}.` und den Namen einer ihrer Definitionen, wie `{s}.{example}`"),
        ("`{x}` is private to the file `{file}`", "`{x}` ist privat in der Datei `{file}`"),
        ("private", "privat"),
        ("a name starting with `_` can only be used in its own file", "ein Name, der mit `_` beginnt, kann nur in seiner eigenen Datei verwendet werden"),
        ("to use it from other files, rename it without `_` in `{file}`", "um ihn aus anderen Dateien zu verwenden, benenne ihn in `{file}` ohne `_` um"),
        ("the file `{file}` has no `{x}`", "die Datei `{file}` hat kein `{x}`"),
        ("not in `{file}`", "nicht in `{file}`"),
        ("did you mean `{m}.{best}`?", "meintest du `{m}.{best}`?"),
        ("`{file}` defines nothing that other files can use", "`{file}` definiert nichts, was andere Dateien verwenden können"),
        ("`{file}` gives: {}{more}", "`{file}` bietet: {}{more}"),
        ("`{name}` is private to the file `{file}`", "`{name}` ist privat in der Datei `{file}`"),
        ("unknown name `{name}`", "unbekannter Name `{name}`"),
        ("not defined here", "hier nicht definiert"),
        ("`{name}` is defined in `{file}`: write `{m}.{name}`", "`{name}` ist in `{file}` definiert: schreib `{m}.{name}`"),
        ("`{name}` is defined in `{file}`: add `use {stem}` at the top of the file, then write `{stem}.{name}`", "`{name}` ist in `{file}` definiert: füge `use {stem}` oben in die Datei ein, und schreib dann `{stem}.{name}`"),
        ("not defined", "nicht definiert"),
        ("defined here, further down", "hier definiert, weiter unten"),
        ("in a block, a function can be used only after it, unless they are written one after the other with nothing between them: move `{name}` up, next to the function that uses it", "in einem Block kann eine Funktion erst nach ihrer Definition verwendet werden, außer sie stehen direkt hintereinander ohne etwas dazwischen: verschiebe `{name}` nach oben, neben die Funktion, die sie verwendet"),
        ("in a block, a name can be used only after the line that defines it: move `{name} = ...` up", "in einem Block kann ein Name erst nach der Zeile verwendet werden, die ihn definiert: verschiebe `{name} = ...` nach oben"),
        ("did you mean `{best}`?", "meintest du `{best}`?"),
        ("to create your own kind of value, define a type: `type Name = {name}(field) | Other`", "um deine eigene Art von Wert zu erzeugen, definiere einen Typ: `type Name = {name}(field) | Other`"),
        ("define it before using it: `{name} = ...` or `fn {name}(...) = ...`", "definiere es, bevor du es verwendest: `{name} = ...` oder `fn {name}(...) = ...`"),
        ("this definition of `{name}` can never be used", "diese Definition von `{name}` kann nie verwendet werden"),
        ("this definition already accepts every argument", "diese Definition akzeptiert schon jedes Argument"),
        ("definitions are tried in order: put the most general one last", "Definitionen werden der Reihe nach ausprobiert: setze die allgemeinste ans Ende"),
        ("this definition already accepts these arguments", "diese Definition akzeptiert diese Argumente schon"),
        ("definitions are tried in order: remove this one, or change its patterns", "Definitionen werden der Reihe nach ausprobiert: entferne diese, oder ändere ihre Muster"),
        ("this function receives {}, which never matches this pattern", "das Argument dieser Funktion, {}, passt nie zu diesem Muster"),
        ("never matches", "passt nie"),
        ("`{name}` does not handle every possible argument", "`{name}` behandelt nicht jedes mögliche Argument"),
        ("not handled: {}", "nicht behandelt: {}"),
        ("add a definition for it, like `fn {name}({}) = ...`", "füge eine Definition dafür hinzu, wie `fn {name}({}) = ...`"),
        ("the value {} does not match this pattern", "der Wert {} passt nicht zu diesem Muster"),
        ("a value of type {} never matches this pattern", "ein Wert vom Typ {} passt nie zu diesem Muster"),
        ("`_` can only be used in patterns", "`_` kann nur in Mustern verwendet werden"),
        ("`_` means \"anything, ignored\"; give the value a real name to use it", "`_` bedeutet „irgendetwas, ignoriert“; gib dem Wert einen echten Namen, um ihn zu verwenden"),
        ("`?` can only be used inside a function", "`?` kann nur innerhalb einer Funktion verwendet werden"),
        ("outside of a function, handle the value with `match` or `unwrap_or`", "außerhalb einer Funktion behandle den Wert mit `match` oder `unwrap_or`"),
        ("`?` needs an Option or a Result, but got {}", "`?` braucht eine Option oder ein Result, erhalten: {}"),
        ("`?` takes the value out of `Some(x)` / `Ok(x)`, and returns `None` / `Err(e)` from the current function", "`?` holt den Wert aus `Some(x)` / `Ok(x)` heraus und gibt `None` / `Err(e)` aus der aktuellen Funktion zurück"),
        ("the field `{k}` appears twice", "das Feld `{k}` kommt zweimal vor"),
        ("`..` expects {expected} here, but got {}", "`..` erwartet hier {expected}, erhalten: {}"),
        ("this value is not a function", "dieser Wert ist keine Funktion"),
        ("cannot be called", "kann nicht aufgerufen werden"),
        ("to multiply, write `*` explicitly: `2 * (x + 1)`", "zum Multiplizieren schreib `*` ausdrücklich: `2 * (x + 1)`"),
        ("to take one element, use brackets: `x[0]`; between two values, a `,` may be missing", "um ein Element zu nehmen, verwende eckige Klammern: `x[0]`; zwischen zwei Werten fehlt vielleicht ein `,`"),
        ("to read a field, use a dot: `person.name`", "um ein Feld zu lesen, verwende einen Punkt: `person.name`"),
        ("`{s}` is not a function", "`{s}` ist keine Funktion"),
        ("`{s}` is {}", "`{s}` ist {}"),
        ("returns {returned}", "gibt {returned} zurück"),
        ("this assertion always fails: {a} and {b} are never equal", "diese Zusicherung schlägt immer fehl: {a} und {b} sind nie gleich"),
        (
            r#"one value is inside an Option or a Result: compare it with `Some(...)` or `Ok(...)`,
like `assert_eq(parse_int("3"), Ok(3))`"#,
            r#"ein Wert steckt in einer Option oder einem Result: vergleiche ihn mit `Some(...)` oder `Ok(...)`,
wie `assert_eq(parse_int("3"), Ok(3))`"#,
        ),
        ("values of different kinds are never equal: Rythme never converts values silently", "Werte verschiedener Arten sind nie gleich: Rythme wandelt Werte nie stillschweigend um"),
        ("a constructor, shown as `<constructor ...>`", "ein Konstruktor, angezeigt als `<constructor ...>`"),
        ("a function, shown as `<fn ...>`", "eine Funktion, angezeigt als `<fn ...>`"),
        ("this shows a function, not a value", "das zeigt eine Funktion, keinen Wert"),
        ("to show what it gives, call it: `{call}`", "um zu zeigen, was sie liefert, rufe sie auf: `{call}`"),
        ("no definition of `{name}` accepts these arguments: {name}({})", "keine Definition von `{name}` akzeptiert diese Argumente: {name}({})"),
        ("called here", "hier aufgerufen"),
        ("does not match", "passt nicht"),
        ("add a definition that handles this case, for example one that ends with a plain name or `_`", "füge eine Definition hinzu, die diesen Fall behandelt, zum Beispiel eine, die mit einem einfachen Namen oder `_` endet"),
        ("the field `{name}` is not a function", "das Feld `{name}` ist keine Funktion"),
        ("`{name}` is {}", "`{name}` ist {}"),
        ("to read the field, write it without parentheses: `.{name}`", "um das Feld zu lesen, schreib es ohne Klammern: `.{name}`"),
        ("there is no function named `{shown}`", "es gibt keine Funktion namens `{shown}`"),
        ("`value.{shown}(...)` calls `{shown}(value, ...)`, or the field `{shown}` of a record", "`value.{shown}(...)` ruft `{shown}(value, ...)` auf, oder das Feld `{shown}` eines Records"),
        ("`{name}` is a value, not a function", "`{name}` ist ein Wert, keine Funktion"),
        ("write `{name}` without parentheses", "schreib `{name}` ohne Klammern"),
        ("this record has no field `{name}`", "dieser Record hat kein Feld `{name}`"),
        ("unknown field", "unbekanntes Feld"),
        ("the record is empty: {{}}", "der Record ist leer: {{}}"),
        ("its fields are: {}", "seine Felder sind: {}"),
        ("to get an Option instead of an error, use `get(record, \"{name}\")`", "um statt eines Fehlers eine Option zu bekommen, verwende `get(record, \"{name}\")`"),
        ("{} has no fields", "{} hat keine Felder"),
        ("no field `{name}` here", "hier gibt es kein Feld `{name}`"),
        ("did you mean `.{best}`? (`value.{best}` calls `{best}(value)`)", "meintest du `.{best}`? (`value.{best}` ruft `{best}(value)` auf)"),
        ("`{c}` has no field `{name}`", "`{c}` hat kein Feld `{name}`"),
        ("no variant of `{tn}` has a field `{name}`", "keine Variante von `{tn}` hat ein Feld `{name}`"),
        ("`{c}` has no fields", "`{c}` hat keine Felder"),
        ("the fields of `{c}` are: {}", "die Felder von `{c}` sind: {}"),
        ("the variants of `{tn}` have no fields", "die Varianten von `{tn}` haben keine Felder"),
        ("their fields are: {}", "ihre Felder sind: {}"),
        ("a value of type `{tn}` may be another variant: use `match` to handle each case", "ein Wert vom Typ `{tn}` kann eine andere Variante sein: verwende `match`, um jeden Fall zu behandeln"),
        ("cannot use `[...]` on {}", "`[...]` kann nicht auf {} angewendet werden"),
        ("`[...]` works on lists, texts, records and dicts", "`[...]` funktioniert bei Listen, Texten, Records und Dicts"),
        ("an index must be a whole number (Int), but got {}", "ein Index muss eine ganze Zahl (Int) sein, ist aber {}"),
        ("the name of a field must be a text, but got {}", "der Name eines Feldes muss ein Text sein, ist aber {}"),
        ("read a field with `record[\"name\"]` or `record.name`", "lies ein Feld mit `record[\"name\"]` oder `record.name`"),
        ("this record has no field `{k}`", "dieser Record hat kein Feld `{k}`"),
        ("use `get(record, key)` to receive an Option instead", "verwende `get(record, key)`, um stattdessen eine Option zu bekommen"),
        ("{} and {}", "{} und {}"),
        ("a range needs whole numbers, but got {got}", "ein Bereich braucht ganze Zahlen, erhalten: {got}"),
        ("example: `1..10` is the list of the numbers from 1 to 10", "Beispiel: `1..10` ist die Liste der Zahlen von 1 bis 10"),
        ("cannot use `-` on {}", "`-` kann nicht auf {} angewendet werden"),
        ("a condition must be `true` or `false`, but this is {}", "eine Bedingung muss `true` oder `false` sein, aber das hier ist {}"),
        ("not a Bool", "kein Bool"),
        ("compare it explicitly, for example `x != 0` or `x > 0`", "vergleiche ausdrücklich, zum Beispiel `x != 0` oder `x > 0`"),
        ("to test if it is empty, use `is_empty(x)` or `not is_empty(x)`", "um zu prüfen, ob es leer ist, verwende `is_empty(x)` oder `not is_empty(x)`"),
        ("use `is_some(x)`, `is_ok(x)`, or `match` to look inside", "verwende `is_some(x)`, `is_ok(x)` oder `match`, um hineinzuschauen"),
        ("cannot use `{sym}` with {ln} and {rn}", "`{sym}` kann nicht mit {ln} und {rn} verwendet werden"),
        ("cannot use `{sym}` with {ln}", "`{sym}` kann nicht mit {ln} verwendet werden"),
        ("cannot use `{sym}` with {rn}", "`{sym}` kann nicht mit {rn} verwendet werden"),
        ("`{name} -x` subtracts here; to call `{name}`, write `{name}(-x)` (a call without parentheses starts a statement or a result, such as a branch of `if` or the body after the `=` of a function, and its name is not given a value with `=` or as a parameter)", "`{name} -x` subtrahiert hier; um `{name}` aufzurufen, schreib `{name}(-x)` (ein Aufruf ohne Klammern beginnt eine Anweisung oder ein Ergebnis, etwa einen Zweig von `if` oder den Rumpf nach dem `=` einer Funktion, und sein Name bekommt keinen Wert mit `=` oder als Parameter)"),
        ("to join two sets or two dicts, use `union(a, b)`; to take one away from the other, `difference(a, b)`", "um zwei Mengen oder zwei Dicts zu vereinen, verwende `union(a, b)`; um das eine vom anderen abzuziehen, `difference(a, b)`"),
        (
            r#"to build a text with a value inside, use braces: "total: {{x}}"
or convert the value first: text(x)"#,
            r#"um einen Text mit einem Wert darin zu bauen, verwende geschweifte Klammern: "total: {{x}}"
oder wandle den Wert zuerst um: text(x)"#,
        ),
        ("to add one element to a list, use `push(list, x)` or `[..list, x]`", "um einer Liste ein Element hinzuzufügen, verwende `push(list, x)` oder `[..list, x]`"),
        ("to repeat a text or a list, use `repeat(x, n)`", "um einen Text oder eine Liste zu wiederholen, verwende `repeat(x, n)`"),
        ("convert the text to a number first, with `int(t)` or `float(t)`", "wandle den Text zuerst in eine Zahl um, mit `int(t)` oder `float(t)`"),
        ("only values of the same kind can be compared", "nur Werte derselben Art können verglichen werden"),
        ("this comparison is always false: {ln} and {rn} are never equal", "dieser Vergleich ist immer falsch: {ln} und {rn} sind nie gleich"),
        ("this comparison is always true: {ln} and {rn} are never equal", "dieser Vergleich ist immer wahr: {ln} und {rn} sind nie gleich"),
        ("to look inside an Option, use `match`, `unwrap_or(x, default)` or `is_some(x)`", "um in eine Option hineinzuschauen, verwende `match`, `unwrap_or(x, default)` oder `is_some(x)`"),
        ("Rythme never converts values silently: convert one side first, with `text(x)` or `int(t)`", "Rythme wandelt Werte nie stillschweigend um: wandle zuerst eine Seite um, mit `text(x)` oder `int(t)`"),
        ("this case can never be reached", "dieser Fall kann nie erreicht werden"),
        ("this case already handles everything", "dieser Fall behandelt schon alles"),
        ("cases are tried in order: put `_` last", "Fälle werden der Reihe nach ausprobiert: setze `_` ans Ende"),
        ("this case already handles these values", "dieser Fall behandelt diese Werte schon"),
        ("a value goes to the first case that fits: remove this case, or change it", "ein Wert geht in den ersten Fall, der passt: entferne diesen Fall, oder ändere ihn"),
        ("no case of this `match` handles {}", "kein Fall dieses `match` behandelt {}"),
        ("this value", "dieser Wert"),
        ("add a case for this value, or a last case `_ -> ...` that handles everything else", "füge einen Fall für diesen Wert hinzu, oder einen letzten Fall `_ -> ...`, der alles andere behandelt"),
        ("this `match` does not handle every case", "dieses `match` behandelt nicht jeden Fall"),
        ("add a last case `_ -> ...` for all the other values", "füge einen letzten Fall `_ -> ...` für alle anderen Werte hinzu"),
        ("add the missing cases, like `{} -> ...`, or a last case `_ -> ...`", "füge die fehlenden Fälle hinzu, wie `{} -> ...`, oder einen letzten Fall `_ -> ...`"),
        ("unknown constructor `{n}`", "unbekannter Konstruktor `{n}`"),
        ("constructors are defined with `type`: `type Name = {n}(field) | Other`", "Konstruktoren werden mit `type` definiert: `type Name = {n}(field) | Other`"),
        ("`{name}` has {} field(s), but this pattern gives {}", "`{name}` hat {} Feld(er), aber dieses Muster gibt {} an"),
        ("write it like `{example}`", "schreib es wie `{example}`"),
        ("this range never matches", "dieser Bereich passt nie"),
        ("no number goes from {lo} up to {hi}", "keine Zahl geht von {lo} hinauf bis {hi}"),
        ("a range goes from the smaller number to the larger one: `{hi}..{lo}`", "ein Bereich geht von der kleineren Zahl zur größeren: `{hi}..{lo}`"),
        ("patterns joined with `|` cannot give names (here `{s}`)", "mit `|` verbundene Muster können keine Namen vergeben (hier `{s}`)"),
        ("write separate cases instead", "schreib stattdessen getrennte Fälle"),
        ("handles some of these numbers", "behandelt einige dieser Zahlen"),
        ("together, the cases before it already handle all its numbers: remove it, or change it", "zusammen behandeln die Fälle davor schon alle seine Zahlen: entferne ihn, oder ändere ihn"),
        ("these ranges overlap", "diese Bereiche überschneiden sich"),
        ("{both} is also here", "{both} ist auch hier"),
        ("{both} goes here, the first that fits", "{both} geht hierher, in den ersten, der passt"),
        ("both ends of a range are included: make the ranges follow each other without overlapping, like `0..9` then `10..19`", "beide Enden eines Bereichs gehören dazu: lass die Bereiche ohne Überschneidung aufeinander folgen, wie `0..9` und dann `10..19`"),
        ("use `match` to handle the different possible shapes of a value", "verwende `match`, um die verschiedenen möglichen Formen eines Werts zu behandeln"),
        ("a value of type {}", "ein Wert vom Typ {}"),
        ("this line computes a value but does nothing with it", "diese Zeile berechnet einen Wert, macht aber nichts damit"),
        (
            r#"a line that starts with `-` is a new statement. To continue the line above,
end that line with `-` instead:
  total = price -
    discount"#,
            r#"eine Zeile, die mit `-` beginnt, ist eine neue Anweisung. Um die Zeile darüber fortzusetzen,
beende stattdessen jene Zeile mit `-`:
  total = price -
    discount"#,
        ),
        (
            r#"to keep the value, give it a name: `result = ...`; to show it: `print(...)`.
Values never change: `x + 1` does not change `x`."#,
            r#"um den Wert zu behalten, gib ihm einen Namen: `result = ...`; um ihn anzuzeigen: `print(...)`.
Werte ändern sich nie: `x + 1` ändert `x` nicht."#,
        ),
        // src/diagnostic.rs
        ("error", "Fehler"),
        ("warning", "Warnung"),
        ("help", "Hilfe"),
        ("call trace (most recent first):", "Aufrufe (der neueste zuerst):"),
        ("an anonymous function", "eine anonyme Funktion"),
        ("in", "in"),
        ("called at", "aufgerufen bei"),
        ("and {} more calls", "und {} weitere Aufrufe"),
        ("more about this error: `rythme explain {}`", "mehr über diesen Fehler: `rythme explain {}`"),
        // src/doc.rs
        ("`{a}` needs a file name, like `rythme doc geometry.ry --out geometry.html`", "`{a}` braucht einen Dateinamen, wie `rythme doc geometrie.ry --out geometrie.html`"),
        ("unknown option `{o}` for `rythme doc`", "unbekannte Option `{o}` für `rythme doc`"),
        (
            r#"
did you mean `{best}`?"#,
            r#"
meintest du `{best}`?"#,
        ),
        (
            r#"
see `rythme help doc`"#,
            r#"
siehe `rythme help doc`"#,
        ),
        (
            r#"which files? Example: rythme doc geometry.ry
  a folder documents all its .ry files: rythme doc .
  --html writes one HTML page instead of Markdown
  --out FILE writes to a file (a name ending with .html gives HTML)
see `rythme help doc`"#,
            r#"welche Dateien? Beispiel: rythme doc geometrie.ry
  ein Ordner dokumentiert alle seine .ry-Dateien: rythme doc .
  --html schreibt eine HTML-Seite statt Markdown
  --out DATEI schreibt in eine Datei (ein Name, der auf .html endet, ergibt HTML)
siehe `rythme help doc`"#,
        ),
        ("no .ry file in `{target}`", "keine .ry-Datei in `{target}`"),
        ("Documentation of {one}", "Dokumentation von {one}"),
        ("Documentation", "Dokumentation"),
        ("written: {file}", "geschrieben: {file}"),
        ("cannot write the file `{file}`: {e}", "die Datei `{file}` kann nicht geschrieben werden: {e}"),
        ("Types", "Typen"),
        ("Functions", "Funktionen"),
        ("Values", "Werte"),
        ("Tests", "Tests"),
        ("Uses", "Verwendet"),
        ("Contents", "Inhalt"),
        ("Built-in functions mentioned", "Erwähnte eingebaute Funktionen"),
        ("Example", "Beispiel"),
        ("Nothing public to document.", "Nichts Öffentliches zu dokumentieren."),
        ("Documentation written by `rythme doc`", "Dokumentation, geschrieben von `rythme doc`"),
        (" Same as: {} (its Ruby names).", " Dasselbe wie: {} (die Ruby-Namen)."),
        ("Example:", "Beispiel:"),
        (" as {open}{n}{close}", " als {open}{n}{close}"),
        (": ", ": "),
        // src/driver.rs
        ("`{shown}` is a folder, not a file", "`{shown}` ist ein Ordner, keine Datei"),
        ("give a file inside it, like {}", "gib eine Datei darin an, wie {}"),
        ("file `{shown}` not found", "Datei `{shown}` nicht gefunden"),
        ("no permission to read the file `{shown}`", "keine Berechtigung, die Datei `{shown}` zu lesen"),
        ("cannot read the file `{shown}`: {e}", "die Datei `{shown}` kann nicht gelesen werden: {e}"),
        ("the file `{shown}` is not text in the UTF-8 encoding (unexpected byte on line {line})", "die Datei `{shown}` ist kein Text in der Kodierung UTF-8 (unerwartetes Byte in Zeile {line})"),
        ("it looks like it was saved in UTF-16", "sie scheint in UTF-16 gespeichert zu sein"),
        ("save it again with the UTF-8 encoding (most editors have it in \"Save as\" or in the status bar)", "speichere sie noch einmal in der Kodierung UTF-8 (die meisten Editoren haben das unter „Speichern unter“ oder in der Statusleiste)"),
        ("(`{name}` now has {n} definitions, tried in order; :reset to start again)", "(`{name}` hat jetzt {n} Definitionen, die der Reihe nach ausprobiert werden; :reset, um neu anzufangen)"),
        ("file `{p}` not found", "Datei `{p}` nicht gefunden"),
        ("looked for: {}", "gesucht: {}"),
        ("the Rythme files there are: {}{more}", "die Rythme-Dateien dort sind: {}{more}"),
        ("the path is relative to the folder of the file that contains `use`", "der Pfad ist relativ zum Ordner der Datei, die `use` enthält"),
        ("`{p}` uses itself", "`{p}` verwendet sich selbst"),
        ("`{last}` and `{first}` use each other", "`{last}` und `{first}` verwenden sich gegenseitig"),
        ("the circle: {}", "der Kreis: {}"),
        ("files cannot use each other: move what both need into a third file, used by both", "Dateien können sich nicht gegenseitig verwenden: verschiebe, was beide brauchen, in eine dritte Datei, die beide verwenden"),
        ("the name `{n}` is given to two files: `{first}` and `{second}`", "der Name `{n}` wird zwei Dateien gegeben: `{first}` und `{second}`"),
        ("give this one another name: `use ... as other_name`", "gib dieser einen anderen Namen: `use ... as other_name`"),
        ("`{s}` is defined by two used files: `{first}` and `{second}`", "`{s}` wird von zwei verwendeten Dateien definiert: `{first}` und `{second}`"),
        ("use the files with their names, without quotes, and write which one you mean: `use geometry`, then `geometry.{s}`", "verwende die Dateien mit ihren Namen, ohne Anführungszeichen, und schreib, welche du meinst: `use geometry`, dann `geometry.{s}`"),
        // src/exercise.rs
        ("which exercise? Example: rythme exercise start 1   (`rythme exercise list` shows them all)", "welche Übung? Beispiel: rythme exercise start 1   (`rythme exercise list` zeigt alle)"),
        ("which file? Example: rythme exercise check 01-values.ry", "welche Datei? Beispiel: rythme exercise check 01-values.ry"),
        ("unknown action `{other}` for `rythme exercise`", "unbekannte Aktion `{other}` für `rythme exercise`"),
        (
            r#"
did you mean `rythme exercise {best}`?"#,
            r#"
meintest du `rythme exercise {best}`?"#,
        ),
        (
            r#"
the actions are: list, start NAME, check FILE (see `rythme help exercises`)"#,
            r#"
die Aktionen sind: list, start NAME, check DATEI (siehe `rythme help exercises`)"#,
        ),
        ("{} exercises for a first course, from the first values to tests, from ★☆☆ (easy) to ★★★.", "{} Übungen für einen ersten Kurs, von den ersten Werten bis zu Tests, von ★☆☆ (leicht) bis ★★★."),
        (
            r#"Start one:       rythme exercise start 1
Check your work: rythme exercise check {}.ry"#,
            r#"Eine beginnen:   rythme exercise start 1
Arbeit prüfen:   rythme exercise check {}.ry"#,
        ),
        ("there is no exercise `{name}`", "es gibt keine Übung `{name}`"),
        (
            r#"
the exercises are numbered from 1 to {}: `rythme exercise list` shows them"#,
            r#"
die Übungen sind von 1 bis {} nummeriert: `rythme exercise list` zeigt sie"#,
        ),
        ("`{file}` already exists here: open it to continue, or rename it to start again", "`{file}` gibt es hier schon: öffne die Datei, um weiterzumachen, oder benenne sie um, um neu anzufangen"),
        ("cannot write `{file}`: {error}", "`{file}` kann nicht geschrieben werden: {error}"),
        (
            r#"Created {file}: exercise {} · {title}  {level}

Open it, read the statement and the hints at the top, and replace each
`fail("to do")` with your code. Then check your work:

  rythme exercise check {file}"#,
            r#"{file} erstellt: Übung {} · {title}  {level}

Öffne sie, lies die Aufgabe und die Tipps oben, und ersetze jedes
`fail("to do")` durch deinen Code. Prüfe dann deine Arbeit:

  rythme exercise check {file}"#,
        ),
        ("file `{name}` not found", "Datei `{name}` nicht gefunden"),
        (
            r#"
to start this exercise: rythme exercise start {}"#,
            r#"
um diese Übung zu beginnen: rythme exercise start {}"#,
        ),
        (
            r#"✗ The file has errors, so its tests could not run.
  Fix the first error above, then check again."#,
            r#"✗ Die Datei hat Fehler, deshalb konnten ihre Tests nicht laufen.
  Behebe den ersten Fehler oben und prüfe dann noch einmal."#,
        ),
        ("✗ This file has no tests. An exercise comes with its tests: is it the right file?", "✗ Diese Datei hat keine Tests. Eine Übung kommt mit ihren Tests: ist es die richtige Datei?"),
        ("✓ Well done: all {total} tests pass!", "✓ Gut gemacht: alle {total} Tests bestehen!"),
        (
            r#"  Next exercise: {title}  {level}
    rythme exercise start {}"#,
            r#"  Nächste Übung: {title}  {level}
    rythme exercise start {}"#,
        ),
        ("  That was the last exercise: you have finished them all. Congratulations!", "  Das war die letzte Übung: du hast alle geschafft. Herzlichen Glückwunsch!"),
        ("✗ All {total} tests pass, but...", "✗ Alle {total} Tests bestehen, aber..."),
        ("✗ {passed} of {total} tests pass. Keep going!", "✗ {passed} von {total} Tests bestehen. Weiter so!"),
        (
            r#"  - The tests of the exercise were changed: the exercise is solved when its own tests
    pass. `rythme exercise start {number}` gives a fresh copy (rename your file first)."#,
            r#"  - Die Tests der Übung wurden geändert: die Übung ist gelöst, wenn ihre eigenen Tests
    bestehen. `rythme exercise start {number}` gibt eine frische Kopie (benenne deine Datei vorher um)."#,
        ),
        ("  - Some functions are still to do: replace each `fail(\"to do\")` with your code.", "  - Einige Funktionen sind noch zu schreiben: ersetze jedes `fail(\"to do\")` durch deinen Code."),
        (
            r#"  - Start with the first failure above: compare what you got with what was expected.
  - Stuck? Read the hints at the top of the file, or `rythme help NAME` for a function."#,
            r#"  - Fang mit dem ersten Fehlschlag oben an: vergleiche, was du bekommen hast, mit dem Erwarteten.
  - Du steckst fest? Lies die Tipps oben in der Datei, oder `rythme help NAME` für eine Funktion."#,
        ),
        // src/format.rs
        ("`{name}` could not be formatted safely, so it was left as it is", "`{name}` konnte nicht sicher formatiert werden und wurde so gelassen, wie es ist"),
        ("this is a bug in Rythme, not in your program: please report it, with the file, at https://github.com/maxlestage/Rythme/issues", "das ist ein Fehler in Rythme, nicht in deinem Programm: bitte melde ihn, mit der Datei, unter https://github.com/maxlestage/Rythme/issues"),
        ("unknown option `{o}` for `rythme fmt`", "unbekannte Option `{o}` für `rythme fmt`"),
        (
            r#"
see `rythme help fmt`"#,
            r#"
siehe `rythme help fmt`"#,
        ),
        (
            r#"which files? Example: rythme fmt hello.ry
  a folder formats all its .ry files: rythme fmt .
  - reads the code from the standard input
  --check only checks, for continuous integration
see `rythme help fmt`"#,
            r#"welche Dateien? Beispiel: rythme fmt hallo.ry
  ein Ordner formatiert alle seine .ry-Dateien: rythme fmt .
  - liest den Code von der Standardeingabe
  --check prüft nur, für die kontinuierliche Integration
siehe `rythme help fmt`"#,
        ),
        ("needs formatting: {shown}", "muss formatiert werden: {shown}"),
        ("cannot write the file `{shown}`: {e}", "die Datei `{shown}` kann nicht geschrieben werden: {e}"),
        ("formatted: {shown}", "formatiert: {shown}"),
        ("needs formatting: <stdin>", "muss formatiert werden: <stdin>"),
        ("{changed} file(s) formatted, {unchanged} already formatted", "{changed} Datei(en) formatiert, {unchanged} schon formatiert"),
        ("✓ {unchanged} file(s) already formatted", "✓ {unchanged} Datei(en) schon formatiert"),
        ("✗ {changed} file(s) to format, {unchanged} already formatted; `rythme fmt` (without --check) formats them", "✗ {changed} Datei(en) zu formatieren, {unchanged} schon formatiert; `rythme fmt` (ohne --check) formatiert sie"),
        ("✗ {errors} file(s) left as they are because of errors", "✗ {errors} Datei(en) wegen Fehlern unverändert gelassen"),
        ("file or folder `{shown}` not found", "Datei oder Ordner `{shown}` nicht gefunden"),
        ("did you mean `{}`?", "meintest du `{}`?"),
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
            r#"Rythme {v} — Hilfe

Befehle
  rythme                    interaktiver Modus (Code Zeile für Zeile ausprobieren)
  rythme datei.ry           ein Programm ausführen
  rythme check datei.ry     Probleme finden, ohne das Programm auszuführen
  rythme test datei.ry      die Tests einer Datei (oder eines Ordners) ausführen
  rythme fmt datei.ry       dem Code sein offizielles Layout geben (--check: nur prüfen)
  rythme doc datei.ry       seine Dokumentation schreiben, in Markdown (--html: eine Webseite)
  rythme steps datei.ry     die Rechnung Schritt für Schritt zeigen (fib(3) → fib(2) + fib(1) → ...)
  rythme exercise           automatisch geprüfte Übungen (list, start, check)
  rythme new NAME           ein Projekt anlegen (rythme.toml, src/, tests/)
  rythme install            die Abhängigkeiten des Projekts herunterladen
  rythme learn              geführte Tour durch die Sprache, Schritt für Schritt
  rythme help THEMA         Hilfe zu einem Thema oder einer Funktion
  rythme explain E003       einen Fehlercode erklären
  rythme -e "code"          ein Stück Code ausführen
  rythme lsp                Sprachserver, für Editoren (VS Code, Neovim...)
  Option --lang CODE        Sprache der Meldungen: en, fr, es, de, pt, zh (oder RYTHME_LANG)

Themen (rythme help THEMA)
  start       die Sprache in einer Minute
  syntax      die ganze Syntax auf einer Seite
  functions   Funktionen, |>, Rekursion
  lists       Listen und ihre Funktionen
  text        Texte
  records     Records {{name: value}}
  dicts       Dicts und Mengen: dict(...), set(...)
  types       eigene Typen
  match       je nach Form eines Werts entscheiden
  errors      Option, Result und ?
  functional  keine Schleifen: was man stattdessen tut
  tests       Tests schreiben und ausführen
  exercises   Übungen zum Lernen, mit ihren Tests
  modules     mehrere Dateien verwenden
  system      Dateien, Ordner, Pfade, Optionen, Eingabe, Exit-Codes
  projects    Projekte und Pakete: rythme new, rythme.toml, rythme install
  dates       Datum und Uhrzeit: now(), today(), Kalender, ISO 8601
  regex       mit regulären Ausdrücken suchen, zerlegen und ersetzen
  repl        der interaktive Modus
  fmt         das offizielle Layout des Codes (rythme fmt)
  doc         Dokumentation aus den ##-Kommentaren (rythme doc)
  steps       die Rechnung Schritt für Schritt (rythme steps)
  editors     Rythme in deinem Editor (VS Code, Neovim, Helix...)
  ruby        von Ruby kommend: Blöcke, Symbole, Methodenketten
  builtins    alle eingebauten Funktionen

Eine bestimmte Funktion: rythme help map
"#,
        ),
        ("{alias}  (the Ruby name of `{of}`: the same function)", "{alias}  (der Ruby-Name von `{of}`: dieselbe Funktion)"),
        (
            r#"pi
  The number π = 3.141592653589793."#,
            r#"pi
  Die Zahl π = 3.141592653589793."#,
        ),
        ("The function:", "Die Funktion:"),
        ("No help found for `{name}`.", "Keine Hilfe zu `{name}` gefunden."),
        (
            r#"
Did you mean `{best}`?"#,
            r#"
Meintest du `{best}`?"#,
        ),
        (
            r#"
See the list of topics with `rythme help`."#,
            r#"
Die Liste der Themen zeigt `rythme help`."#,
        ),
        ("keyword", "Schlüsselwort"),
        (
            r#"  More: rythme help {topic}
"#,
            r#"  Mehr: rythme help {topic}
"#,
        ),
        (
            r#"  Same as: {} (its Ruby names)
"#,
            r#"  Dasselbe wie: {} (die Ruby-Namen)
"#,
        ),
        (
            r#"BUILT-IN FUNCTIONS
"#,
            r#"EINGEBAUTE FUNKTIONEN
"#,
        ),
        ("the number π", "die Zahl π"),
        ("{} (same as {})", "{} (dasselbe wie {})"),
        ("  Ruby names:", "  Ruby-Namen:"),
        ("build an Option or a Result", "eine Option oder ein Result bauen"),
        (
            r#"
Details and example for one function: rythme help NAME (or help(NAME) in code)
"#,
            r#"
Details und Beispiel zu einer Funktion: rythme help NAME (oder help(NAME) im Code)
"#,
        ),
        (
            r#"help(x) explains x: a function (help(map)), a topic (help("lists")), or a value (help([1, 2])).
Topics: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
            r#"help(x) erklärt x: eine Funktion (help(map)), ein Thema (help("lists")) oder einen Wert (help([1, 2])).
Themen: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."#,
        ),
        (
            r#"  (no documentation: write `## description` on the line above `fn`)
"#,
            r#"  (keine Dokumentation: schreib `## Beschreibung` in die Zeile über `fn`)
"#,
        ),
        ("defined at", "definiert in"),
        (
            r#"{}({})
  A constructor of the type `{}`.
"#,
            r#"{}({})
  Ein Konstruktor des Typs `{}`.
"#,
        ),
        (
            r#"This is a value of type {ty}: {}
"#,
            r#"Das ist ein Wert vom Typ {ty}: {}
"#,
        ),
        (
            r#"Useful functions: {}
"#,
            r#"Nützliche Funktionen: {}
"#,
        ),
        (
            r#"Also: {also}
"#,
            r#"Außerdem: {also}
"#,
        ),
        (
            r#"Use `match` to handle each variant of `{}`.
"#,
            r#"Verwende `match`, um jede Variante von `{}` zu behandeln.
"#,
        ),
        ("[Enter] next   [q] quit > ", "[Enter] weiter   [q] beenden > "),
        ("To practice, run `rythme` and type code, or do the exercises: `rythme exercise list`. `rythme help builtins` lists every function.", "Zum Üben starte `rythme` und tippe Code ein, oder mach die Übungen: `rythme exercise list`. `rythme help builtins` listet alle Funktionen auf."),
        // src/interp.rs
        ("this list would be too large (more than {MAX_LIST} elements)", "diese Liste wäre zu groß (mehr als {MAX_LIST} Elemente)"),
        ("this text would be too large (more than {mb} MB)", "dieser Text wäre zu groß (mehr als {mb} MB)"),
        ("does it grow in a function that calls itself? Check that the function always reaches a case where it stops", "wächst er in einer Funktion, die sich selbst aufruft? Prüfe, ob die Funktion immer einen Fall erreicht, in dem sie aufhört"),
        ("no function named `{}`", "keine Funktion namens `{}`"),
        ("`value.{0}(...)` calls `{0}(value, ...)`", "`value.{0}(...)` ruft `{0}(value, ...)` auf"),
        ("`{name}` is not defined yet", "`{name}` ist noch nicht definiert"),
        ("the file runs from top to bottom: move this definition above the line that uses it (directly or through a function)", "die Datei läuft von oben nach unten: verschiebe diese Definition über die Zeile, die sie verwendet (direkt oder über eine Funktion)"),
        ("`{ctor}` has no field `{name}`", "`{ctor}` hat kein Feld `{name}`"),
        ("`{ctor}` has no fields", "`{ctor}` hat keine Felder"),
        ("the fields of `{ctor}` are: {}", "die Felder von `{ctor}` sind: {}"),
        ("a value of type `{}` may be another variant: use `match` to handle each case", "ein Wert vom Typ `{}` kann eine andere Variante sein: verwende `match`, um jeden Fall zu behandeln"),
        ("too many nested calls ({depth} calls in progress)", "zu viele verschachtelte Aufrufe ({depth} Aufrufe laufen)"),
        ("this call goes too deep", "dieser Aufruf geht zu tief"),
        (
            r#"a recursive function that never stops? Check that it always reaches a base case.
Otherwise, make the recursive call the very last thing the function does
(tail call), for example with an accumulator:
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
or use `fold`, `map`, `filter`, which have no limit."#,
            r#"eine rekursive Funktion, die nie aufhört? Prüfe, ob sie immer einen Basisfall erreicht.
Sonst mach den rekursiven Aufruf zum allerletzten Schritt der Funktion
(Endaufruf), zum Beispiel mit einem Akkumulator:
  fn sum([], acc) = acc
  fn sum([x, ..rest], acc) = sum(rest, acc + x)
oder verwende `fold`, `map`, `filter`, die keine Grenze haben."#,
        ),
        ("the program was stopped: it made more calls than allowed for this run", "das Programm wurde angehalten: es hat mehr Aufrufe gemacht, als für diesen Lauf erlaubt sind"),
        ("does it call itself forever? Check that it always reaches a base case", "ruft es sich endlos selbst auf? Prüfe, ob es immer einen Basisfall erreicht"),
        ("{} is not a function", "{} ist keine Funktion"),
        ("`{0}` is a value, not a function: write `{0}` without parentheses", "`{0}` ist ein Wert, keine Funktion: schreib `{0}` ohne Klammern"),
        ("`..` expects {what} here, but got {}", "`..` erwartet hier {what}, erhalten: {}"),
        ("division by zero", "Division durch null"),
        (
            r#"check the divisor first: `if b == 0 then ... else a / b`,
or return a Result: `if b == 0 then Err("division by zero") else Ok(a / b)`"#,
            r#"prüfe zuerst den Teiler: `if b == 0 then ... else a / b`,
oder gib ein Result zurück: `if b == 0 then Err("Division durch null") else Ok(a / b)`"#,
        ),
        ("`0 ** -n` is `1 / 0 ** n`", "`0 ** -n` ist `1 / 0 ** n`"),
        ("{min} argument{s}", "{min} Argument(e)"),
        ("at least {min} argument{s}", "mindestens {min} Argument(e)"),
        ("{min} to {max} arguments", "{min} bis {max} Argumente"),
        ("{got} {verb}", "{got}"),
        ("`{name}` takes {expected}, but {given}", "`{name}` nimmt {expected}, hat aber {given} bekommen"),
        ("{got} argument(s) here", "hier {got} Argument(e)"),
        ("usage: `{sig}`", "Verwendung: `{sig}`"),
        ("cannot use `{sym}` with {lt} and {rt}", "`{sym}` kann nicht mit {lt} und {rt} verwendet werden"),
        ("`nan` (not a number) cannot be compared", "`nan` (keine Zahl) kann nicht verglichen werden"),
        ("a negative number ({}) to a fractional power ({}) has no value among real numbers", "eine negative Zahl ({}) hoch einen gebrochenen Exponenten ({}) hat keinen Wert unter den reellen Zahlen"),
        ("use `abs(x) ** p` for the power of its absolute value", "verwende `abs(x) ** p` für die Potenz ihres Betrags"),
        ("this range is too large ({} elements)", "dieser Bereich ist zu groß ({} Elemente)"),
        ("ranges are lists: keep them under {MAX_RANGE} elements", "Bereiche sind Listen: halte sie unter {MAX_RANGE} Elementen"),
        ("a range needs whole numbers, but got {} and {}", "ein Bereich braucht ganze Zahlen, erhalten: {} und {}"),
        ("index {n} is out of range: {what}", "Index {n} liegt außerhalb des Bereichs: {what}"),
        ("valid indices go from 0 to {} (or -1 for the last element)", "gültige Indizes gehen von 0 bis {} (oder -1 für das letzte Element)"),
        ("to get an Option instead of an error, use `get(x, {n})`", "um statt eines Fehlers eine Option zu bekommen, verwende `get(x, {n})`"),
        ("the text is empty", "der Text ist leer"),
        ("the text has {len} character(s)", "der Text hat {len} Zeichen"),
        ("the list has {} element(s)", "die Liste hat {} Element(e)"),
        ("this dict has no key {}", "dieses Dict hat keinen Schlüssel {}"),
        ("use `get(dict, key)` to receive an Option instead, or `has(dict, key)` to check first", "verwende `get(dict, key)`, um stattdessen eine Option zu bekommen, oder `has(dict, key)`, um es vorher zu prüfen"),
        // src/lexer.rs
        ("`...` is not an operator in Rythme", "`...` ist kein Operator in Rythme"),
        ("write `1..10` for a range (10 included), or `[first, ..rest]` in a list", "schreib `1..10` für einen Bereich (10 inklusive), oder `[first, ..rest]` in einer Liste"),
        ("`++` is not an operator in Rythme", "`++` ist kein Operator in Rythme"),
        (
            r#"to add one, write `x + 1` and give the result a new name (values never change);
to join two lists or two texts, use `+`: `[1] + [2]`"#,
            r#"um eins zu addieren, schreib `x + 1` und gib dem Ergebnis einen neuen Namen (Werte ändern sich nie);
um zwei Listen oder zwei Texte zu verbinden, verwende `+`: `[1] + [2]`"#,
        ),
        ("unexpected character `&`", "unerwartetes Zeichen `&`"),
        ("use `and` to combine two conditions", "verwende `and`, um zwei Bedingungen zu verbinden"),
        ("this `{{` in a text is never closed", "diese `{{` in einem Text wird nie geschlossen"),
        ("starts a value inserted in the text", "beginnt einen in den Text eingefügten Wert"),
        ("close it with `}}`, or write `\\{{` to get a literal brace: \"\\{{\"", "schließe sie mit `}}`, oder schreib `\\{{` für eine echte geschweifte Klammer: \"\\{{\""),
        ("unexpected invisible character {code}", "unerwartetes unsichtbares Zeichen {code}"),
        ("this is a special space (often copied from a document or a web page): replace it with a normal space", "das ist ein besonderes Leerzeichen (oft aus einem Dokument oder einer Webseite kopiert): ersetze es durch ein normales Leerzeichen"),
        ("this character cannot be seen (it is often copied from a document or a web page): delete it", "dieses Zeichen ist unsichtbar (es wird oft aus einem Dokument oder einer Webseite kopiert): lösche es"),
        ("unexpected character `{c}`", "unerwartetes Zeichen `{c}`"),
        ("write `{s}` instead", "schreib stattdessen `{s}`"),
        ("texts use double quotes: \"hello\"", "Texte stehen in doppelten Anführungszeichen: \"hallo\""),
        ("use `**` for powers: `2 ** 10`", "verwende `**` für Potenzen: `2 ** 10`"),
        ("use straight double quotes `\"` for texts", "verwende gerade doppelte Anführungszeichen `\"` für Texte"),
        ("to insert a value in a text, use braces: \"total: {{x}}\"", "um einen Wert in einen Text einzufügen, verwende geschweifte Klammern: \"total: {{x}}\""),
        ("invalid number `{text}`", "ungültige Zahl `{text}`"),
        ("hexadecimal numbers use the digits 0 to 9 and a to f, like `0xff`; binary numbers use 0 and 1, like `0b101`", "Hexadezimalzahlen verwenden die Ziffern 0 bis 9 und a bis f, wie `0xff`; Binärzahlen verwenden 0 und 1, wie `0b101`"),
        ("a name cannot start with a digit: `{}`", "ein Name kann nicht mit einer Ziffer beginnen: `{}`"),
        ("to multiply, write it explicitly: `{num} * {word}`", "zum Multiplizieren schreib es ausdrücklich: `{num} * {word}`"),
        ("texts are nested too deeply inside `{{...}}`", "Texte sind zu tief in `{{...}}` verschachtelt"),
        ("this text is never closed", "dieser Text wird nie geschlossen"),
        ("the text starts here", "der Text beginnt hier"),
        ("add a `\"` at the end of the text", "füge am Ende des Textes ein `\"` hinzu"),
        ("invalid unicode escape", "ungültiges Unicode-Escape"),
        ("write it like `\\u{{e9}}` for é", "schreib es wie `\\u{{e9}}` für é"),
        ("unknown escape sequence in text", "unbekannte Escape-Sequenz im Text"),
        ("in a regex, write the backslash twice: \"\\\\{c}\" is the text \\{c}", "in einem regulären Ausdruck schreib den Backslash doppelt: \"\\\\{c}\" ist der Text \\{c}"),
        (
            r#"valid escapes: \n (new line), \t (tab), \r (carriage return), \0 (null character), \\ (backslash),
\" (quote), \{{ and \}} (braces), \# (a `#` before a brace), \u{{e9}} (a character by its Unicode number: é)"#,
            r#"gültige Escapes: \n (neue Zeile), \t (Tabulator), \r (Wagenrücklauf), \0 (Nullzeichen), \\ (Backslash),
\" (Anführungszeichen), \{{ und \}} (geschweifte Klammern), \# (ein `#` vor einer geschweiften Klammer), \u{{e9}} (ein Zeichen über seine Unicode-Nummer: é)"#,
        ),
        ("inside it, this text is never closed", "darin wird dieser Text nie geschlossen"),
        ("empty `{{}}` in a text", "leeres `{{}}` in einem Text"),
        ("put a value inside, like \"{{name}}\", or write `\\{{` for a literal brace", "setze einen Wert hinein, wie \"{{name}}\", oder schreib `\\{{` für eine echte geschweifte Klammer"),
        ("inside a text, `{{` starts an inserted value: to write braces, use `\\{{` and `\\}}`, like \"\\{{\\\"a\\\": 1\\}}\"", "in einem Text beginnt `{{` einen eingefügten Wert: um geschweifte Klammern zu schreiben, verwende `\\{{` und `\\}}`, wie \"\\{{\\\"a\\\": 1\\}}\""),
        ("the number `{n}`", "die Zahl `{n}`"),
        ("the number `{f}`", "die Zahl `{f}`"),
        ("the name `{s}`", "der Name `{s}`"),
        ("the symbol `:{s}`", "das Symbol `:{s}`"),
        ("the end of the line", "das Ende der Zeile"),
        ("the end of the code", "das Ende des Codes"),
        ("the keyword `{k}`", "das Schlüsselwort `{k}`"),
        // src/lsp/analysis.rs
        ("cannot read the file `{shown}`", "die Datei `{shown}` kann nicht gelesen werden"),
        ("the file `{p}` has {n} error(s)", "die Datei `{p}` hat {n} Fehler"),
        ("first: {}{place}", "erster: {}{place}"),
        ("open `{p}` to see and fix them", "öffne `{p}`, um sie zu sehen und zu beheben"),
        // src/lsp/extras.rs
        ("▶ Run", "▶ Ausführen"),
        ("▶ Run the {n} tests", "▶ Die {n} Tests ausführen"),
        ("▶ Run test", "▶ Test ausführen"),
        // src/lsp/features.rs
        (
            r#"
more: `rythme explain {}`"#,
            r#"
mehr: `rythme explain {}`"#,
        ),
        (
            r#"

Same as: {} (its Ruby names)"#,
            r#"

Dasselbe wie: {} (die Ruby-Namen)"#,
        ),
        ("**{alias}** — the Ruby name of `{of}`: the same function", "**{alias}** — der Ruby-Name von `{of}`: dieselbe Funktion"),
        ("A value that is there, in an `Option`. The other case is `None`.", "Ein Wert, der da ist, in einer `Option`. Der andere Fall ist `None`."),
        ("No value, in an `Option`: what `find`, `first` or `get` give when there is nothing. Handle it with `match` or `unwrap_or`.", "Kein Wert, in einer `Option`: was `find`, `first` oder `get` liefern, wenn nichts da ist. Behandle es mit `match` oder `unwrap_or`."),
        ("The result of an operation that succeeded, in a `Result`. The other case is `Err(error)`.", "Das Ergebnis einer Operation, die gelungen ist, in einem `Result`. Der andere Fall ist `Err(error)`."),
        ("The result of an operation that failed, in a `Result`. Inside a function, `?` stops and passes it on.", "Das Ergebnis einer Operation, die gescheitert ist, in einem `Result`. In einer Funktion hält `?` an und gibt es weiter."),
        ("**{word}** is not used in Rythme.", "**{word}** wird in Rythme nicht verwendet."),
        ("{} more definitions", "{} weitere Definitionen"),
        ("Inferred from its code:", "Aus seinem Code abgeleitet:"),
        ("local function", "lokale Funktion"),
        ("parameter of `{f}`", "Parameter von `{f}`"),
        ("parameter of an anonymous function", "Parameter einer anonymen Funktion"),
        ("local value", "lokaler Wert"),
        ("given by the case `{pattern}` of a `match`", "gegeben durch den Fall `{pattern}` eines `match`"),
        ("constructor of the type `{t}`", "Konstruktor des Typs `{t}`"),
        ("*defined in `{name}`*", "*definiert in `{name}`*"),
        ("The number π.", "Die Zahl π."),
        (
            r#"

`value.{n}(...)` calls `{n}(value, ...)`, or the field `{n}` of a record; `value.{n}` reads the field `{n}`, or calls `{n}(value)`"#,
            r#"

`value.{n}(...)` ruft `{n}(value, ...)` auf, oder das Feld `{n}` eines Records; `value.{n}` liest das Feld `{n}`, oder ruft `{n}(value)` auf"#,
        ),
        ("the file `{p}` is not found", "die Datei `{p}` wurde nicht gefunden"),
        (
            r#"

Its definitions are used through the name `{n}`: `{n}.name`"#,
            r#"

Ihre Definitionen werden über den Namen `{n}` verwendet: `{n}.name`"#,
        ),
        ("Defines:", "Definiert:"),
        ("name", "name"),
        ("value", "wert"),
        ("other", "anderes"),
        ("pattern", "muster"),
        ("result", "ergebnis"),
        ("what it checks", "was er prüft"),
        ("actual", "erhalten"),
        ("expected", "erwartet"),
        ("condition", "bedingung"),
        ("file", "datei"),
        ("a function on several lines", "eine Funktion über mehrere Zeilen"),
        ("a function on one line", "eine Funktion in einer Zeile"),
        ("choose according to the shape of a value", "je nach Form eines Werts entscheiden"),
        ("a test, run by `rythme test`", "ein Test, ausgeführt von `rythme test`"),
        ("a test on several lines", "ein Test über mehrere Zeilen"),
        ("choose between two values", "zwischen zwei Werten wählen"),
        ("type ${{1:Name}} = ${{2:First}}(${{3:field}}) | ${{0:Second}}", "type ${{1:Name}} = ${{2:Erste}}(${{3:feld}}) | ${{0:Zweite}}"),
        ("your own type", "dein eigener Typ"),
        ("several steps inside an expression", "mehrere Schritte innerhalb eines Ausdrucks"),
        ("a block: a function given as the last argument of a call", "ein Block: eine Funktion, die als letztes Argument eines Aufrufs übergeben wird"),
        ("the opposite of `if`", "das Gegenteil von `if`"),
        ("use another file: `use geometry`, then `geometry.area(2)`", "eine andere Datei verwenden: `use geometry`, dann `geometry.area(2)`"),
        ("same as {}", "dasselbe wie {}"),
        ("field", "Feld"),
        ("package", "Paket"),
        ("folder", "Ordner"),
        ("Add `use {file}` and write `{stem}.{written}`", "`use {file}` hinzufügen und `{stem}.{written}` schreiben"),
        ("Replace with `{new}`", "Durch `{new}` ersetzen"),
        // src/lsp/mod.rs
        ("rythme lsp: unknown option `{other}` (ignored)", "rythme lsp: unbekannte Option `{other}` (ignoriert)"),
        ("rythme lsp is the language server: your editor starts it and talks to it. See `rythme help editors`. (Ctrl-C to quit)", "rythme lsp ist der Sprachserver: dein Editor startet ihn und spricht mit ihm. Siehe `rythme help editors`. (Strg-C zum Beenden)"),
        ("the server is not initialized yet", "der Server ist noch nicht initialisiert"),
        ("the server is shutting down", "der Server wird heruntergefahren"),
        ("internal error in Rythme (please report it): {}", "interner Fehler in Rythme (bitte melde ihn): {}"),
        ("not formatted: the code has an error ({first})", "nicht formatiert: der Code hat einen Fehler ({first})"),
        // src/lsp/refs.rs
        ("there is no name here", "hier ist kein Name"),
        ("the name of a test is a text: edit it directly", "der Name eines Tests ist ein Text: bearbeite ihn direkt"),
        ("`{name}` cannot be found", "`{name}` wurde nicht gefunden"),
        ("`{name}` is a built-in function: it cannot be renamed", "`{name}` ist eine eingebaute Funktion: sie kann nicht umbenannt werden"),
        ("`{name}` is part of Rythme: it cannot be renamed", "`{name}` gehört zu Rythme: es kann nicht umbenannt werden"),
        ("`{name}` is not defined: there is nothing to rename", "`{name}` ist nicht definiert: es gibt nichts umzubenennen"),
        ("`{new}` is not a valid name", "`{new}` ist kein gültiger Name"),
        ("`{new}` is a keyword", "`{new}` ist ein Schlüsselwort"),
        ("the names of types and constructors start with an uppercase letter", "die Namen von Typen und Konstruktoren beginnen mit einem Großbuchstaben"),
        ("names start with a lowercase letter (uppercase is for types and constructors)", "Namen beginnen mit einem Kleinbuchstaben (Großbuchstaben sind für Typen und Konstruktoren)"),
        ("`{new}` is a built-in function: choose another name", "`{new}` ist eine eingebaute Funktion: wähle einen anderen Namen"),
        ("`{shown}` has a syntax error: fix it before renaming", "`{shown}` hat einen Syntaxfehler: behebe ihn vor dem Umbenennen"),
        ("`{old_name}` is also the name of a record field, and `value.{old_name}(...)` may call that field: rename it by hand", "`{old_name}` ist auch der Name eines Record-Feldes, und `value.{old_name}(...)` könnte dieses Feld aufrufen: benenne es von Hand um"),
        ("cannot rename `{old_name}` to `{new_name}`: {why}", "`{old_name}` kann nicht in `{new_name}` umbenannt werden: {why}"),
        ("some names would then refer to something else", "einige Namen würden dann auf etwas anderes verweisen"),
        ("the code would not be valid", "der Code wäre nicht gültig"),
        // src/manifest.rs
        ("expected a name here", "hier wird ein Name erwartet"),
        ("each line is `name = \"value\"`, like `version = \"0.1.0\"`", "jede Zeile ist `name = \"value\"`, wie `version = \"0.1.0\"`"),
        ("this text is not closed", "dieser Text wird nicht geschlossen"),
        ("add `{quote}` at the end of the text", "füge am Ende des Textes `{quote}` hinzu"),
        ("unknown escape in a text", "unbekanntes Escape in einem Text"),
        ("write `\\\\` for a backslash, or use `/` in paths: they work on every system", "schreib `\\\\` für einen Backslash, oder verwende `/` in Pfaden: das funktioniert auf jedem System"),
        ("expected `=` after `{key}`", "nach `{key}` wird `=` erwartet"),
        ("`{key}` is written twice", "`{key}` wird zweimal geschrieben"),
        ("expected `,` or `}}` here", "hier wird `,` oder `}}` erwartet"),
        ("a table is written on one line: `{{ git = \"...\", tag = \"...\" }}`", "eine Tabelle wird in einer Zeile geschrieben: `{{ git = \"...\", tag = \"...\" }}`"),
        ("a value is a text, written in quotes", "ein Wert ist ein Text in Anführungszeichen"),
        ("write `\"{example}\"`", "schreib `\"{example}\"`"),
        ("expected `{close}` here", "hier wird `{close}` erwartet"),
        ("write `{key} = \"...\"`", "schreib `{key} = \"...\"`"),
        ("first written here", "zuerst hier geschrieben"),
        ("unexpected text after the value", "unerwarteter Text nach dem Wert"),
        ("write one value per line; a comment starts with `#`", "schreib einen Wert pro Zeile; ein Kommentar beginnt mit `#`"),
        ("`{name}` cannot be the name of a package: it is used in `use {name}/file`, so it must be a Rythme name in lower case (letters, digits, `_`)", "`{name}` kann nicht der Name eines Pakets sein: er wird in `use {name}/file` verwendet, also muss er ein Rythme-Name in Kleinbuchstaben sein (Buchstaben, Ziffern, `_`)"),
        ("`{key}` is {what}, written in quotes", "`{key}` erwartet {what}, in Anführungszeichen geschrieben"),
        ("unknown key `{key}`", "unbekannter Schlüssel `{key}`"),
        ("the keys here are {}", "die Schlüssel hier sind {}"),
        ("write `name = \"{b}\"`", "schreib `name = \"{b}\"`"),
        ("`{v}` is not a version", "`{v}` ist keine Version"),
        ("a version is three numbers: `version = \"1.0.0\"` (major.minor.patch)", "eine Version besteht aus drei Zahlen: `version = \"1.0.0\"` (Major.Minor.Patch)"),
        ("`entry` is the path of a `.ry` file in the project", "`entry` ist der Pfad einer `.ry`-Datei im Projekt"),
        ("like `entry = \"src/main.ry\"`", "wie `entry = \"src/main.ry\"`"),
        ("unknown section `{n}`", "unbekannter Abschnitt `{n}`"),
        ("write `name`, `version` and `entry` at the top of the file, without a section", "schreib `name`, `version` und `entry` oben in die Datei, ohne Abschnitt"),
        ("the only section is `[dependencies]`", "der einzige Abschnitt ist `[dependencies]`"),
        ("`{shown}` has no `name`", "`{shown}` hat keinen `name`"),
        ("add a line like `name = \"hello\"` at the top", "füge oben eine Zeile wie `name = \"hallo\"` hinzu"),
        ("`{shown}` has no `version`", "`{shown}` hat keine `version`"),
        ("add a line like `version = \"0.1.0\"` at the top", "füge oben eine Zeile wie `version = \"0.1.0\"` hinzu"),
        ("write `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (or `commit = \"...\"`), or `{name} = {{ path = \"../folder\" }}`", "schreib `{name} = {{ git = \"https://...\", tag = \"v1.0.0\" }}` (oder `commit = \"...\"`), oder `{name} = {{ path = \"../ordner\" }}`"),
        ("call it `{b}`", "nenne es `{b}`"),
        ("the project `{own}` cannot depend on itself", "das Projekt `{own}` kann nicht von sich selbst abhängen"),
        ("`use {own}/file` already reaches its own files", "`use {own}/file` erreicht schon seine eigenen Dateien"),
        ("the dependency `{name}` is written as a table", "die Abhängigkeit `{name}` wird als Tabelle geschrieben"),
        ("the dependency `{name}` has both `git` and `path`", "die Abhängigkeit `{name}` hat sowohl `git` als auch `path`"),
        ("the dependency `{name}` needs `git` or `path`", "die Abhängigkeit `{name}` braucht `git` oder `path`"),
        ("`tag` and `commit` are for a dependency from git, not from a folder", "`tag` und `commit` sind für eine Abhängigkeit aus git, nicht aus einem Ordner"),
        ("`{c}` is not a commit", "`{c}` ist kein Commit"),
        ("give a `tag` or a `commit`, not both", "gib ein `tag` oder einen `commit` an, nicht beides"),
        ("the dependency `{name}` needs a fixed version: a `tag` or a `commit`", "die Abhängigkeit `{name}` braucht eine feste Version: ein `tag` oder einen `commit`"),
        ("the `git` address of `{name}` is empty", "die `git`-Adresse von `{name}` ist leer"),
        ("this file is written by `rythme install`: delete it and run `rythme install` again", "diese Datei wird von `rythme install` geschrieben: lösche sie und führe `rythme install` noch einmal aus"),
        ("unexpected line in `rythme.lock`", "unerwartete Zeile in `rythme.lock`"),
        ("this entry of `rythme.lock` is not complete", "dieser Eintrag von `rythme.lock` ist nicht vollständig"),
        (
            r#"# The exact commit of each dependency from git, written by `rythme install`.
# Keep this file with the project, so that everyone gets the same code.
"#,
            r#"# Der genaue Commit jeder Abhängigkeit aus git, geschrieben von `rythme install`.
# Behalte diese Datei im Projekt, damit alle denselben Code bekommen.
"#,
        ),
        // src/parser.rs
        ("all the definitions of `{name}` must have the same number of parameters", "alle Definitionen von `{name}` müssen gleich viele Parameter haben"),
        ("{} parameter(s) here", "hier {} Parameter"),
        ("expected {expected}, found {found}", "erwartet: {expected}, gefunden: {found}"),
        ("expected {expected} here", "hier wird {expected} erwartet"),
        ("to compare two values, use `==`", "um zwei Werte zu vergleichen, verwende `==`"),
        ("use `->` in `match` cases; a dict is built with `dict([[key, value], ...])`", "verwende `->` in den Fällen von `match`; ein Dict wird mit `dict([[key, value], ...])` gebaut"),
        ("use `and` to combine conditions", "verwende `and`, um Bedingungen zu verbinden"),
        ("use `or` to combine conditions", "verwende `or`, um Bedingungen zu verbinden"),
        ("use `not` to negate a condition: `not done`", "verwende `not`, um eine Bedingung zu verneinen: `not done`"),
        ("Rythme does not use braces for blocks: write `do` ... `end`, or put a function body on the following lines and finish with `end`", "Rythme verwendet keine geschweiften Klammern für Blöcke: schreib `do` ... `end`, oder setze den Rumpf einer Funktion in die folgenden Zeilen und schließe mit `end`"),
        ("Rythme does not use `:` to start a block; see `rythme help syntax`", "Rythme beginnt einen Block nicht mit `:`; siehe `rythme help syntax`"),
        ("`{word}` is not used in Rythme", "`{word}` wird in Rythme nicht verwendet"),
        ("comes from another language", "kommt aus einer anderen Sprache"),
        ("unexpected here: {found}", "hier unerwartet: {found}"),
        ("the statement should end before this", "die Anweisung sollte vorher enden"),
        ("to call a function, use parentheses: `{name}(...)`", "um eine Funktion aufzurufen, verwende Klammern: `{name}(...)`"),
        ("a block `do |x| ... end` is given to a call, on its line: `names.each do |name| print(name) end`", "ein Block `do |x| ... end` wird einem Aufruf in seiner Zeile übergeben: `names.each do |name| print(name) end`"),
        ("`else` must follow `if condition then value` (an `if` branch holds one expression; use `do ... end` for several lines)", "`else` muss auf `if Bedingung then Wert` folgen (ein Zweig von `if` enthält einen Ausdruck; verwende `do ... end` für mehrere Zeilen)"),
        ("put each statement on its own line", "setze jede Anweisung in eine eigene Zeile"),
        ("`{word}` can only be used at the top level of a file", "`{word}` kann nur auf der obersten Ebene einer Datei verwendet werden"),
        ("move it outside of the function or block", "verschiebe es aus der Funktion oder dem Block heraus"),
        ("before `=` comes a name or a pattern, like `[first, ..rest]`, `{{name, ..}}` or `Some(x)`", "vor `=` steht ein Name oder ein Muster, wie `[first, ..rest]`, `{{name, ..}}` oder `Some(x)`"),
        ("`=`", "`=`"),
        ("values cannot be changed in Rythme", "Werte können in Rythme nicht geändert werden"),
        ("a record pattern cannot give a name to the other fields", "ein Record-Muster kann den anderen Feldern keinen Namen geben"),
        ("write `{{name, ..}}` to ignore the other fields; for a record without some fields, use `remove(record, \"name\")`", "schreib `{{name, ..}}`, um die anderen Felder zu ignorieren; für einen Record ohne bestimmte Felder verwende `remove(record, \"name\")`"),
        ("only a function can have a name that ends with `?`", "nur eine Funktion kann einen Namen haben, der auf `?` endet"),
        ("define it with `fn`: `fn {s}(x) = ...`; for a value, use a name without `?`", "definiere sie mit `fn`: `fn {s}(x) = ...`; für einen Wert verwende einen Namen ohne `?`"),
        ("invalid `..` in this pattern", "ungültiges `..` in diesem Muster"),
        ("`..name` must be last: `[first, ..rest]`", "`..name` muss am Ende stehen: `[first, ..rest]`"),
        ("cannot give a value to this", "dem kann kein Wert gegeben werden"),
        ("this is not a name", "das ist kein Name"),
        ("`=` gives a name to a value", "`=` gibt einem Wert einen Namen"),
        ("to define a function, write `fn name(x) = ...`", "um eine Funktion zu definieren, schreib `fn name(x) = ...`"),
        ("records cannot be changed; build a new one: `new = {{..old, {name}: value}}`", "Records können nicht geändert werden; bau einen neuen: `new = {{..old, {name}: value}}`"),
        ("values cannot be changed; build a new one: `set_at(list, i, x)` for a list, `put(record, name, x)` for a record", "Werte können nicht geändert werden; bau einen neuen: `set_at(list, i, x)` für eine Liste, `put(record, name, x)` für einen Record"),
        ("a function name", "ein Funktionsname"),
        ("`(` and the parameters", "`(` und die Parameter"),
        ("even without parameters, write the parentheses: `fn greet() = print(\"hi\")`", "auch ohne Parameter schreib die Klammern: `fn greet() = print(\"hi\")`"),
        ("functions use `=`, not `{arrow}`", "Funktionen verwenden `=`, nicht `{arrow}`"),
        ("write `fn(x) = x * 2`", "schreib `fn(x) = x * 2`"),
        ("`=` or a new line", "`=` oder eine neue Zeile"),
        (
            r#"a function is written `fn name(x) = expression`,
or with its body on the following lines, finished by `end`"#,
            r#"eine Funktion wird `fn name(x) = Ausdruck` geschrieben,
oder mit ihrem Rumpf in den folgenden Zeilen, abgeschlossen mit `end`"#,
        ),
        ("type names start with an uppercase letter", "Typnamen beginnen mit einem Großbuchstaben"),
        ("write `type {cap} = ...`", "schreib `type {cap} = ...`"),
        ("a type name", "ein Typname"),
        ("example: `type Shape = Circle(radius) | Square(side)`", "Beispiel: `type Shape = Circle(radius) | Square(side)`"),
        ("variant names start with an uppercase letter", "Variantennamen beginnen mit einem Großbuchstaben"),
        ("write `{cap}`", "schreib `{cap}`"),
        ("a variant name, like `Circle`", "ein Variantenname, wie `Circle`"),
        ("a field name", "ein Feldname"),
        ("fields are named in lowercase: `Circle(radius)`", "Felder werden kleingeschrieben: `Circle(radius)`"),
        ("the name of the test, as a text", "der Name des Tests, als Text"),
        ("example: `test \"addition works\" = assert_eq(1 + 1, 2)`", "Beispiel: `test \"Addition funktioniert\" = assert_eq(1 + 1, 2)`"),
        ("write the name of the file without `.ry`", "schreib den Namen der Datei ohne `.ry`"),
        ("example: `use {path}`", "Beispiel: `use {path}`"),
        ("a name for the file, in lowercase", "ein Name für die Datei, in Kleinbuchstaben"),
        ("example: `use geometry as geo`, then `geo.area(2)`", "Beispiel: `use geometry as geo`, dann `geo.area(2)`"),
        ("the name of a file", "der Name einer Datei"),
        ("example: `use geometry` for the file `geometry.ry`, then `geometry.area(2)`", "Beispiel: `use geometry` für die Datei `geometry.ry`, dann `geometry.area(2)`"),
        ("for a file whose name starts with an uppercase letter, write its path as a text: `use \"Shapes.ry\" as shapes`", "für eine Datei, deren Name mit einem Großbuchstaben beginnt, schreib ihren Pfad als Text: `use \"Shapes.ry\" as shapes`"),
        ("the end of the expression", "das Ende des Ausdrucks"),
        ("comparisons cannot be chained", "Vergleiche können nicht verkettet werden"),
        ("combine them with `and`: `a < b and b < c`", "verbinde sie mit `and`: `a < b and b < c`"),
        ("`{sym}` cannot follow a `|>` step directly", "`{sym}` kann nicht direkt auf einen `|>`-Schritt folgen"),
        (
            r#"`|>` comes after the calculations: put the pipeline in parentheses,
like `(x |> f) {sym} 1`, or continue it with a function: `x |> f |> fn(v) = v {sym} 1`"#,
            r#"`|>` kommt nach den Rechnungen: setze die Pipeline in Klammern,
wie `(x |> f) {sym} 1`, oder setze sie mit einer Funktion fort: `x |> f |> fn(v) = v {sym} 1`"#,
        ),
        ("a block must follow a call", "ein Block muss auf einen Aufruf folgen"),
        ("not given to a function", "keiner Funktion übergeben"),
        (
            r#"write the block on the line of the call it is given to: `xs.map do |x| x * 2 end`;
for a function as a value, write `fn(x) = x * 2`"#,
            r#"schreib den Block in die Zeile des Aufrufs, dem er übergeben wird: `xs.map do |x| x * 2 end`;
für eine Funktion als Wert schreib `fn(x) = x * 2`"#,
        ),
        ("`,` or `|`", "`,` oder `|`"),
        ("the parameters of a block go between bars: `do |name, age| ... end`", "die Parameter eines Blocks stehen zwischen senkrechten Strichen: `do |name, age| ... end`"),
        ("`}}`", "`}}`"),
        ("the block starts here", "der Block beginnt hier"),
        ("a field or function name after `.`", "ein Feld- oder Funktionsname nach `.`"),
        ("an expression", "ein Ausdruck"),
        ("`]`", "`]`"),
        ("`,` or `{closing}`", "`,` oder `{closing}`"),
        ("separate the elements with commas", "trenne die Elemente durch Kommas"),
        ("this code is nested too deeply (more than {MAX_NESTING} levels)", "dieser Code ist zu tief verschachtelt (mehr als {MAX_NESTING} Ebenen)"),
        ("split it into several named values or functions", "teile ihn in mehrere benannte Werte oder Funktionen auf"),
        ("this expression is too long or nested too deeply (more than {MAX_DEPTH} levels)", "dieser Ausdruck ist zu lang oder zu tief verschachtelt (mehr als {MAX_DEPTH} Ebenen)"),
        (
            r#"split it into several named values or functions;
to combine many values, put them in a list: `sum([a, b, c])`, `join([x, y, z])`"#,
            r#"teile ihn in mehrere benannte Werte oder Funktionen auf;
um viele Werte zu verbinden, setze sie in eine Liste: `sum([a, b, c])`, `join([x, y, z])`"#,
        ),
        ("Rythme has no tuples", "Rythme hat keine Tupel"),
        ("group values in a list `[a, b]` or a record `{{x: a, y: b}}`", "fasse Werte in einer Liste `[a, b]` oder einem Record `{{x: a, y: b}}` zusammen"),
        ("`)`", "`)`"),
        ("a named function cannot be used inside an expression", "eine benannte Funktion kann nicht innerhalb eines Ausdrucks verwendet werden"),
        ("define it on its own line, or use an anonymous function: `fn(x) = x * 2`", "definiere sie in einer eigenen Zeile, oder verwende eine anonyme Funktion: `fn(x) = x * 2`"),
        ("comments start with `#`", "Kommentare beginnen mit `#`"),
        ("`else` must follow `if condition then value`; a branch holds a single expression (use `do ... end` for several lines)", "`else` muss auf `if Bedingung then Wert` folgen; ein Zweig enthält einen einzigen Ausdruck (verwende `do ... end` für mehrere Zeilen)"),
        ("a record is written `{{name: \"Ada\", age: 36}}`; for a block of code, use `do ... end`", "ein Record wird `{{name: \"Ada\", age: 36}}` geschrieben; für einen Codeblock verwende `do ... end`"),
        ("`:` and a value", "`:` und ein Wert"),
        ("`then`", "`then`"),
        ("an `{word}` is written: `{word} condition then value else other_value`", "ein `{word}` wird so geschrieben: `{word} Bedingung then Wert else anderer_Wert`"),
        ("this `{word}` has no `else`", "dieses `{word}` hat kein `else`"),
        ("every `{word}` must have an `else`", "jedes `{word}` braucht ein `else`"),
        ("expected `else` here", "hier wird `else` erwartet"),
        ("write `else if`", "schreib `else if`"),
        (
            r#"in Rythme, `unless` always gives a value, so it needs both cases:
  unless age >= 18 then "minor" else "adult""#,
            r#"in Rythme liefert `unless` immer einen Wert, also braucht es beide Fälle:
  unless age >= 18 then "minderjährig" else "volljährig""#,
        ),
        (
            r#"in Rythme, `if` always gives a value, so it needs both cases:
  if age >= 18 then "adult" else "minor""#,
            r#"in Rythme liefert `if` immer einen Wert, also braucht es beide Fälle:
  if age >= 18 then "volljährig" else "minderjährig""#,
        ),
        ("a branch holds a single expression; for several lines, use `do ... end`", "ein Zweig enthält einen einzigen Ausdruck; für mehrere Zeilen verwende `do ... end`"),
        ("a new line", "eine neue Zeile"),
        (
            r#"write each case on its own line, then `end`:
  match x
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"schreib jeden Fall in eine eigene Zeile, dann `end`:
  match x
    0 -> "null"
    _ -> "anderes"
  end"#,
        ),
        ("no `|` is needed before a case", "vor einem Fall ist kein `|` nötig"),
        ("write each case as `pattern -> result`", "schreib jeden Fall als `Muster -> Ergebnis`"),
        ("`->`", "`->`"),
        ("each case is written `pattern -> result`", "jeder Fall wird `Muster -> Ergebnis` geschrieben"),
        ("put each case on its own line", "setze jeden Fall in eine eigene Zeile"),
        ("a `match` needs at least one case", "ein `match` braucht mindestens einen Fall"),
        ("a number after `..`", "eine Zahl nach `..`"),
        ("only a text can come before `+` in a pattern", "in einem Muster kann nur ein Text vor `+` stehen"),
        ("to look at the end of a text, use a guard: `s if ends_with(s, \"!\") -> ...`", "um das Ende eines Textes zu prüfen, verwende eine Bedingung: `s if ends_with(s, \"!\") -> ...`"),
        ("after `+`, a pattern takes a name or `_`", "nach `+` nimmt ein Muster einen Namen oder `_`"),
        ("a pattern cannot contain `{{...}}`", "ein Muster kann kein `{{...}}` enthalten"),
        ("use a guard instead: `s if s == \"...\" -> ...`", "verwende stattdessen eine Bedingung: `s if s == \"...\" -> ...`"),
        ("`..rest` must be the last element: `[first, ..rest]`", "`..rest` muss das letzte Element sein: `[first, ..rest]`"),
        ("`:` and a pattern", "`:` und ein Muster"),
        ("a pattern", "ein Muster"),
        ("patterns look like: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`", "Muster sehen so aus: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`"),
        ("the ends of a range in a pattern are whole numbers of 64 bits at most", "die Enden eines Bereichs in einem Muster sind ganze Zahlen mit höchstens 64 Bit"),
        ("too large for a range", "zu groß für einen Bereich"),
        ("compare with a guard instead: `n if n >= low and n <= high -> ...`; a range still matches large numbers between its ends, and a decimal end like `1.0e30` works too", "vergleiche stattdessen mit einer Bedingung: `n if n >= low and n <= high -> ...`; ein Bereich passt trotzdem auf große Zahlen zwischen seinen Enden, und ein Dezimalende wie `1.0e30` funktioniert auch"),
        ("a range in a pattern goes from a number to a number", "ein Bereich in einem Muster geht von einer Zahl zu einer Zahl"),
        ("not a number", "keine Zahl"),
        ("the arguments of a call without parentheses are separated by commas: `print a, b`", "die Argumente eines Aufrufs ohne Klammern werden durch Kommas getrennt: `print a, b`"),
        ("write the numbers themselves, like `1..9` or `0.0..0.5`; to compare with names, use a guard: `n if n >= low and n <= high -> ...`", "schreib die Zahlen selbst, wie `1..9` oder `0.0..0.5`; um mit Namen zu vergleichen, verwende eine Bedingung: `n if n >= low and n <= high -> ...`"),
        ("a text that starts with a text is written `\"Mr \" + name`: `name` is the rest of the text", "ein Text, der mit einem Text beginnt, wird `\"Herr \" + name` geschrieben: `name` ist der Rest des Textes"),
        ("missing `end` to close this `{what}`", "es fehlt `end`, um dieses `{what}` zu schließen"),
        ("this `{what}` is never closed", "dieses `{what}` wird nie geschlossen"),
        ("the code ends here", "der Code endet hier"),
        ("add `end` on its own line after the last line of the block", "füge `end` in einer eigenen Zeile nach der letzten Zeile des Blocks hinzu"),
        ("values never change in Rythme: give the new value a new name, like `total2 = total + 1`", "Werte ändern sich in Rythme nie: gib dem neuen Wert einen neuen Namen, wie `total2 = total + 1`"),
        // src/project.rs
        ("no folder to keep the packages in", "kein Ordner, um die Pakete aufzubewahren"),
        ("set the variable RYTHME_HOME to a folder, like `RYTHME_HOME=~/.rythme`", "setze die Variable RYTHME_HOME auf einen Ordner, wie `RYTHME_HOME=~/.rythme`"),
        ("the folder `{p}`", "der Ordner `{p}`"),
        ("run `rythme install` here, in the folder of the project", "führe `rythme install` hier aus, im Ordner des Projekts"),
        ("run `rythme install` in `{root}`, the folder of the project", "führe `rythme install` in `{root}` aus, dem Ordner des Projekts"),
        ("the folder `{p}` of the package `{name}` does not exist", "der Ordner `{p}` des Pakets `{name}` existiert nicht"),
        ("fix its `path` in `rythme.toml`", "korrigiere seinen `path` in `rythme.toml`"),
        ("the package `{name}` is not installed", "das Paket `{name}` ist nicht installiert"),
        ("`rythme.toml` asks for it from {}", "`rythme.toml` verlangt es von {}"),
        ("`{shown_dir}` is not a Rythme package: it has no `rythme.toml`", "`{shown_dir}` ist kein Rythme-Paket: es hat keine `rythme.toml`"),
        ("use one of its files: `use {package}/file`", "verwende eine seiner Dateien: `use {package}/file`"),
        ("the package `{package}` has no file `{entry}`", "das Paket `{package}` hat keine Datei `{entry}`"),
        ("the package `{package}` has no file `{wanted}`", "das Paket `{package}` hat keine Datei `{wanted}`"),
        ("did you mean `use {prefix}{best}`?", "meintest du `use {prefix}{best}`?"),
        ("its files there are: {}", "seine Dateien dort sind: {}"),
        ("did you mean `use {best}/{rest}`?", "meintest du `use {best}/{rest}`?"),
        ("in this project, `use` also reaches the files of the packages {}", "in diesem Projekt erreicht `use` auch die Dateien der Pakete {}"),
        ("the program `git` was not found", "das Programm `git` wurde nicht gefunden"),
        ("Rythme uses git to download the dependencies that come from git", "Rythme verwendet git, um die Abhängigkeiten herunterzuladen, die aus git kommen"),
        ("install Git (https://git-scm.com), then run `rythme install` again; dependencies with `path` do not need it", "installiere Git (https://git-scm.com) und führe dann `rythme install` noch einmal aus; Abhängigkeiten mit `path` brauchen es nicht"),
        ("git says: {said}", "git sagt: {said}"),
        ("the tag `{t}` of `{name}` does not exist in `{url}`", "das Tag `{t}` von `{name}` existiert nicht in `{url}`"),
        ("the commit `{c}` of `{name}` does not exist in `{url}`", "der Commit `{c}` von `{name}` existiert nicht in `{url}`"),
        ("did you mean `tag = \"{best}\"`?", "meintest du `tag = \"{best}\"`?"),
        ("its tags are: {}", "seine Tags sind: {}"),
        ("it has no tags: give a `commit` instead", "es hat keine Tags: gib stattdessen einen `commit` an"),
        ("cannot write in `{}`: {e}", "in `{}` kann nicht geschrieben werden: {e}"),
        ("cannot download `{name}` from `{url}`", "`{name}` kann nicht von `{url}` heruntergeladen werden"),
        ("cannot take the commit `{commit}` of `{name}`", "der Commit `{commit}` von `{name}` kann nicht genommen werden"),
        ("no `rythme.toml` in this folder or above it", "keine `rythme.toml` in diesem Ordner oder darüber"),
        ("go to the folder of a project, or create one with `rythme new NAME`", "geh in den Ordner eines Projekts, oder lege eines an mit `rythme new NAME`"),
        (
            r#"`rythme install` takes no argument (`{a}`): it installs the dependencies of `rythme.toml`
see `rythme help projects`"#,
            r#"`rythme install` nimmt kein Argument (`{a}`): es installiert die Abhängigkeiten aus `rythme.toml`
siehe `rythme help projects`"#,
        ),
        ("`{name}` has no dependencies: add them under `[dependencies]` in `rythme.toml`", "`{name}` hat keine Abhängigkeiten: füge sie unter `[dependencies]` in `rythme.toml` hinzu"),
        ("Installing the dependencies of `{name}`", "Installiere die Abhängigkeiten von `{name}`"),
        ("the folder `{p}` of the package `{dep_name}` does not exist", "der Ordner `{p}` des Pakets `{dep_name}` existiert nicht"),
        ("folder {p}", "Ordner {p}"),
        ("downloaded", "heruntergeladen"),
        ("already there", "schon vorhanden"),
        ("`{dep_name}` is not a Rythme package: `{shown_dir}` has no `rythme.toml`", "`{dep_name}` ist kein Rythme-Paket: `{shown_dir}` hat keine `rythme.toml`"),
        ("a package is a project made by `rythme new`: its `rythme.toml`, and its files in `src/`", "ein Paket ist ein Projekt, das mit `rythme new` angelegt wurde: seine `rythme.toml` und seine Dateien in `src/`"),
        ("  ✓ {dep_name} {version}: {line}", "  ✓ {dep_name} {version}: {line}"),
        ("cannot write `{}`: {e}", "`{}` kann nicht geschrieben werden: {e}"),
        ("{count} package(s) ready", "{count} Paket(e) bereit"),
        ("{count} package(s) ready ({more} needed by the other ones)", "{count} Paket(e) bereit ({more} davon von den anderen gebraucht)"),
        ("which name? Example: rythme new hello", "welcher Name? Beispiel: rythme new hallo"),
        ("`rythme new` takes one name, like `rythme new hello`", "`rythme new` nimmt einen Namen, wie `rythme new hallo`"),
        ("try `rythme new {b}`", "versuch es mit `rythme new {b}`"),
        ("`{shown_dir}` already exists", "`{shown_dir}` existiert schon"),
        ("choose another name, or another folder", "wähle einen anderen Namen oder einen anderen Ordner"),
        ("Created the project `{name}`:", "Projekt `{name}` angelegt:"),
        ("Next: `cd {shown_dir}`, then `rythme run` to run it and `rythme test` to test it (see `rythme help projects`)", "Als Nächstes: `cd {shown_dir}`, dann `rythme run`, um es auszuführen, und `rythme test`, um es zu testen (siehe `rythme help projects`)"),
        ("greeting", "gruss"),
        ("hello", "hallo"),
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
            r#"# Das Projekt: `rythme run`, `rythme test` und `rythme check` in diesem Ordner.
name = "{name}"
version = "0.1.0"
entry = "src/main.ry"

[dependencies]
# Aus git, bei einem Tag oder einem Commit, dann `rythme install`:
# mathlib = {{ git = "https://github.com/jemand/mathlib", tag = "v1.0.0" }}
# Aus einem Ordner:
# werkzeuge = {{ path = "../werkzeuge" }}
"#,
        ),
        (
            r#"# The program: `rythme run` in the folder of the project.
use {module}

print({module}.{func}("World"))
"#,
            r#"# Das Programm: `rythme run` im Ordner des Projekts.
use {module}

print({module}.{func}("Welt"))
"#,
        ),
        (
            r#"## A greeting for `name`.
fn {func}(name) = "Hello, {{name}}!"
"#,
            r#"## Ein Gruß für `name`.
fn {func}(name) = "Hallo, {{name}}!"
"#,
        ),
        (
            r#"# The tests: `rythme test` in the folder of the project.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "Hello, Ada!")
"#,
            r#"# Die Tests: `rythme test` im Ordner des Projekts.
use {name}/{module}

test "{func}" = assert_eq({module}.{func}("Ada"), "Hallo, Ada!")
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

Ein [Rythme](https://github.com/maxlestage/Rythme)-Projekt.

```sh
rythme run        # das Programm ausführen (src/main.ry)
rythme test       # die Tests ausführen (tests/)
rythme check      # Probleme finden, ohne auszuführen
rythme fmt        # dem Code sein offizielles Layout geben
rythme install    # die Abhängigkeiten aus rythme.toml herunterladen
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
            r#"# Dateien, die git nicht aufbewahrt (behalte rythme.lock).
.DS_Store
Thumbs.db
*.swp
*~
"#,
        ),
        ("src/greeting.ry", "src/gruss.ry"),
        ("tests/greeting_test.ry", "tests/gruss_test.ry"),
        // src/repl.rs
        (":help for help, :quit to leave", ":help für Hilfe, :quit zum Beenden"),
        ("(type :quit to leave)", "(tippe :quit zum Beenden)"),
        ("(cancelled)", "(abgebrochen)"),
        ("cannot read what is typed: {e}", "die Eingabe kann nicht gelesen werden: {e}"),
        ("usage: :doc NAME (a function, a topic...)", "Verwendung: :doc NAME (eine Funktion, ein Thema...)"),
        ("usage: :load file.ry", "Verwendung: :load datei.ry"),
        ("{} loaded", "{} geladen"),
        ("Everything was forgotten.", "Alles wurde vergessen."),
        ("Messages are now in English.", "Die Meldungen sind jetzt auf Deutsch."),
        ("usage: :lang fr  (or another of {codes})", "Verwendung: :lang de  (oder ein anderer von {codes})"),
        ("unknown code; codes look like E003 or W001", "unbekannter Code; Codes sehen aus wie E003 oder W001"),
        ("usage: :steps EXPRESSION, like :steps fib(4)", "Verwendung: :steps AUSDRUCK, wie :steps fib(4)"),
        ("Nothing defined yet.", "Noch nichts definiert."),
        ("unknown command `{other}`", "unbekannter Befehl `{other}`"),
        (" — did you mean `{best}`?", " — meintest du `{best}`?"),
        (":help lists the commands", ":help listet die Befehle auf"),
        // src/steps.rs
        ("  ... (more steps, not shown: {n} are shown at most, change it with --max)", "  ... (weitere Schritte, nicht gezeigt: höchstens {n} werden gezeigt, ändere das mit --max)"),
        ("this calculation became too deep to be shown step by step (more than {MAX_DEPTH} operations inside each other)", "diese Rechnung wurde zu tief, um Schritt für Schritt gezeigt zu werden (mehr als {MAX_DEPTH} ineinander verschachtelte Operationen)"),
        ("try it with smaller values, or run the program normally: `rythme file.ry`", "versuch es mit kleineren Werten, oder führe das Programm normal aus: `rythme datei.ry`"),
        ("the evaluation was stopped after {limit} steps: does it ever end?", "die Auswertung wurde nach {limit} Schritten angehalten: endet sie überhaupt?"),
        ("a function that calls itself must always reach a case where it stops", "eine Funktion, die sich selbst aufruft, muss immer einen Fall erreichen, in dem sie aufhört"),
        ("Nothing to show: this file only has definitions. Give an expression to compute, like: rythme steps file.ry \"fib(5)\"", "Nichts zu zeigen: diese Datei hat nur Definitionen. Gib einen Ausdruck zum Berechnen an, wie: rythme steps datei.ry \"fib(5)\""),
        ("`{flag}` needs a number, like `--max 50` or `--width 80` (20 at least)", "`{flag}` braucht eine Zahl, wie `--max 50` oder `--width 80` (mindestens 20)"),
        (
            r#"usage: rythme steps file.ry [expression] [--max N] [--width N]
example: rythme steps fibonacci.ry "fib(4)""#,
            r#"Verwendung: rythme steps datei.ry [Ausdruck] [--max N] [--width N]
Beispiel: rythme steps fibonacci.ry "fib(4)""#,
        ),
        // src/suggest.rs
        ("to join two lists or two texts, use `+`: `[1, 2] + [3]`; for a list of lists, use `flatten`", "um zwei Listen oder zwei Texte zu verbinden, verwende `+`: `[1, 2] + [3]`; für eine Liste von Listen verwende `flatten`"),
        ("in Rythme, this is called `{rythme}`", "in Rythme heißt das `{rythme}`"),
        ("booleans are written in lowercase: `true`", "Wahrheitswerte werden kleingeschrieben: `true`"),
        ("booleans are written in lowercase: `false`", "Wahrheitswerte werden kleingeschrieben: `false`"),
        ("Rythme has no null: use `None` for \"no value\" and `Some(x)` for a value", "Rythme hat kein null: verwende `None` für „kein Wert“ und `Some(x)` für einen Wert"),
        ("in Rythme, this is called `Some`", "in Rythme heißt das `Some`"),
        ("in Rythme, an error value is written `Err(...)`", "in Rythme wird ein Fehlerwert `Err(...)` geschrieben"),
        ("in Rythme, a success value is written `Ok(...)`", "in Rythme wird ein Erfolgswert `Ok(...)` geschrieben"),
        ("Rythme has no objects: functions receive their data as arguments, like `area(shape)` or `shape.area()`", "Rythme hat keine Objekte: Funktionen bekommen ihre Daten als Argumente, wie `area(shape)` oder `shape.area()`"),
        ("write a range with `..`: `1..10` is the list from 1 to 10 (included), and `0..10 |> step_by(2)` counts by 2", "schreib einen Bereich mit `..`: `1..10` ist die Liste von 1 bis 10 (inklusive), und `0..10 |> step_by(2)` zählt in Zweierschritten"),
        ("use the operator `//` for a whole division: `7 // 2` is 3", "verwende den Operator `//` für eine ganzzahlige Division: `7 // 2` ist 3"),
        ("use the operator `%` for the remainder: `7 % 3` is 1", "verwende den Operator `%` für den Rest: `7 % 3` ist 1"),
        ("use the operators `//` and `%`: `7 // 2` is 3 and `7 % 2` is 1", "verwende die Operatoren `//` und `%`: `7 // 2` ist 3 und `7 % 2` ist 1"),
        ("to put values in a text, use braces: \"total: {{x}}\"; to write a number with 2 decimals: `text(x, 2)`", "um Werte in einen Text zu setzen, verwende geschweifte Klammern: \"total: {{x}}\"; um eine Zahl mit 2 Nachkommastellen zu schreiben: `text(x, 2)`"),
        ("read the whole file with `read_file(path)`, then cut it with `lines(text)`; the lines of the standard input are `input_lines()`", "lies die ganze Datei mit `read_file(path)` und zerlege sie dann mit `lines(text)`; die Zeilen der Standardeingabe sind `input_lines()`"),
        ("to merge two records, write `{{..a, ..b}}`: the fields of `b` replace those of `a`; for two dicts or two sets, `union(a, b)`", "um zwei Records zusammenzuführen, schreib `{{..a, ..b}}`: die Felder von `b` ersetzen die von `a`; für zwei Dicts oder zwei Mengen `union(a, b)`"),
        ("write `unwrap_or(get(x, key), default)`: the value, or the default if it is missing", "schreib `unwrap_or(get(x, key), default)`: der Wert, oder der Standardwert, wenn er fehlt"),
        ("pair the lists, then combine each pair: `zip(a, b) |> map(fn([x, y]) = x + y)`", "bilde Paare aus den Listen und verbinde dann jedes Paar: `zip(a, b) |> map(fn([x, y]) = x + y)`"),
        ("sort from the largest to the smallest with `sort(list) |> reverse`", "sortiere vom größten zum kleinsten mit `sort(list) |> reverse`"),
        ("compute the average with `sum(list) / len(list)`", "berechne den Durchschnitt mit `sum(list) / len(list)`"),
        ("keep x between low and high with `max(low, min(x, high))`", "halte x zwischen low und high mit `max(low, min(x, high))`"),
        ("count how many times each value appears with `group_by(list, fn(x) = x) |> map_values(len)`", "zähle, wie oft jeder Wert vorkommt, mit `group_by(list, fn(x) = x) |> map_values(len)`"),
        ("a decimal number x is NaN (not a number) when `x != x`", "eine Dezimalzahl x ist NaN (keine Zahl), wenn `x != x`"),
        ("`n.times do |i| ... end` is written `(0..n - 1).each do |i| ... end`", "`n.times do |i| ... end` wird `(0..n - 1).each do |i| ... end` geschrieben"),
        ("to use another file, write `use other_file`, then `other_file.name`", "um eine andere Datei zu verwenden, schreib `use other_file`, dann `other_file.name`"),
        ("there is no `new`: build values directly, like `{{name: \"Ada\"}}` or `Circle(2.0)`", "es gibt kein `new`: baue Werte direkt, wie `{{name: \"Ada\"}}` oder `Circle(2.0)`"),
        ("values never change in Rythme: build a new value instead", "Werte ändern sich in Rythme nie: baue stattdessen einen neuen Wert"),
        (
            r#"Rythme has no loops. Work on the whole list instead:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
or write a recursive function. See `rythme help functional`."#,
            r#"Rythme hat keine Schleifen. Arbeite stattdessen mit der ganzen Liste:
  1..5 |> each(fn(i) = print(i))
  [1, 2, 3] |> map(fn(x) = x * 2)
oder schreib eine rekursive Funktion. Siehe `rythme help functional`."#,
        ),
        ("Rythme has no loops to interrupt: use `find`, `take`, `filter`, or a recursive function", "Rythme hat keine Schleifen, die man unterbrechen könnte: verwende `find`, `take`, `filter` oder eine rekursive Funktion"),
        (
            r#"no `return` needed: the last expression of a function is its result.
To stop early with an error, return `Err(...)` and use `?`."#,
            r#"kein `return` nötig: der letzte Ausdruck einer Funktion ist ihr Ergebnis.
Um früh mit einem Fehler aufzuhören, gib `Err(...)` zurück und verwende `?`."#,
        ),
        ("just write `name = value` (values never change, so no keyword is needed)", "schreib einfach `name = value` (Werte ändern sich nie, also ist kein Schlüsselwort nötig)"),
        ("functions are written `fn name(x) = x * 2`, and anonymous ones `fn(x) = x * 2`", "Funktionen werden `fn name(x) = x * 2` geschrieben, anonyme `fn(x) = x * 2`"),
        (
            r#"Rythme has no classes. Describe your data with `type`:
  type Shape = Circle(radius) | Square(side)
or use a record: {{name: "Ada", age: 36}}"#,
            r#"Rythme hat keine Klassen. Beschreibe deine Daten mit `type`:
  type Shape = Circle(radius) | Square(side)
oder verwende einen Record: {{name: "Ada", age: 36}}"#,
        ),
        (
            r#"use `match`:
  match value
    0 -> "zero"
    _ -> "other"
  end"#,
            r#"verwende `match`:
  match value
    0 -> "null"
    _ -> "anderes"
  end"#,
        ),
        (
            r#"errors are values in Rythme: return `Ok(value)` or `Err(message)`,
then handle them with `match` or pass them on with `?`. `fail(message)` stops the program."#,
            r#"Fehler sind in Rythme Werte: gib `Ok(value)` oder `Err(message)` zurück,
und behandle sie dann mit `match` oder gib sie mit `?` weiter. `fail(message)` hält das Programm an."#,
        ),
        // src/testing.rs
        (".", ","),
        ("the test returned `false`", "der Test hat `false` zurückgegeben"),
        ("use `assert_eq(actual, expected)` to see the values", "verwende `assert_eq(actual, expected)`, um die Werte zu sehen"),
        ("the test ended with {}", "der Test endete mit {}"),
        ("a test passes when it ends without error, and its value is not `false`, `Err(...)` or `None`", "ein Test besteht, wenn er ohne Fehler endet und sein Wert nicht `false`, `Err(...)` oder `None` ist"),
        (
            r#"No tests found. Write one like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"Keine Tests gefunden. Schreib einen so:
  test "Addition" = assert_eq(1 + 1, 2)"#,
        ),
        ("Running {n} test(s)", "{n} Test(s) werden ausgeführt"),
        ("Running {n} of {total} test(s)", "{n} von {total} Test(s) werden ausgeführt"),
        ("Running {n} test(s) of {file}", "{n} Test(s) von {file} werden ausgeführt"),
        ("Running {n} of {total} test(s) of {file}", "{n} von {total} Test(s) von {file} werden ausgeführt"),
        ("{f} file(s), ", "{f} Datei(en), "),
        ("{total} test(s): {passed} passed, {failed} failed", "{total} Test(s): {passed} bestanden, {failed} fehlgeschlagen"),
        (", {left_out} left out by the filter", ", {left_out} vom Filter ausgelassen"),
        (", {broken} file(s) with errors", ", {broken} Datei(en) mit Fehlern"),
        ("No test name contains {}.", "Kein Testname enthält {}."),
        (" or ", " oder "),
        ("{red}✗{reset} {}: the file has errors, so its tests could not run", "{red}✗{reset} {}: die Datei hat Fehler, deshalb konnten ihre Tests nicht laufen"),
        (
            r#"No file with tests in {}. Write a test in a `.ry` file like this:
  test "addition" = assert_eq(1 + 1, 2)"#,
            r#"Keine Datei mit Tests in {}. Schreib einen Test in eine `.ry`-Datei, so:
  test "Addition" = assert_eq(1 + 1, 2)"#,
        ),
        ("Failed tests:", "Fehlgeschlagene Tests:"),
        ("`{name}` needs {what}", "`{name}` braucht {what}"),
        ("a part of the name of a test", "einen Teil des Namens eines Tests"),
        ("a number, or `random`", "eine Zahl, oder `random`"),
        ("the seed must be a whole number of 0 or more, or `random`, not `{v}`", "der Seed muss eine ganze Zahl ab 0 sein, oder `random`, nicht `{v}`"),
        ("unknown option `{name}` for `rythme test`", "unbekannte Option `{name}` für `rythme test`"),
        (
            r#"
the options are: --filter NAME, --seed N, --watch (see `rythme help tests`)"#,
            r#"
die Optionen sind: --filter NAME, --seed N, --watch (siehe `rythme help tests`)"#,
        ),
        ("which file or folder? Example: rythme test my_tests.ry, or rythme test . for the files of this folder", "welche Datei oder welcher Ordner? Beispiel: rythme test meine_tests.ry, oder rythme test . für die Dateien dieses Ordners"),
        ("Seed: {}", "Seed: {}"),
        (
            r#"
Watching {} file(s): the tests run again when one changes (Ctrl+C to stop)."#,
            r#"
Beobachte {} Datei(en): die Tests laufen erneut, wenn sich eine ändert (Strg+C zum Beenden)."#,
        ),
        (
            r#"
{} changed: running the tests again
"#,
            r#"
{} hat sich geändert: die Tests laufen erneut
"#,
        ),
        // src/value.rs
        ("constructor", "Konstruktor"),
        // playground/src/lib.rs
        ("✓ no problems found", "✓ keine Probleme gefunden"),
        ("✓ no errors, {n} warning(s)", "✓ keine Fehler, {n} Warnung(en)"),
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
            r#"RYTHME IN EINER MINUTE

Rythme ist eine einfache Sprache, in der sich Werte nie ändern und alles eine
Funktion ist. Programme sind Dateien, die auf .ry enden.

  # Ein Kommentar
  name = "Welt"                  # ein Name für einen Wert (er ändert sich nie)
  print("Hallo, {name}!")        # {...} fügt einen Wert in einen Text ein

  fn quadrat(x) = x * x          # eine Funktion
  print(quadrat(4))              # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # die geraden Zahlen behalten
    |> map(quadrat)              # jede quadrieren
    |> sum                       # sie addieren
    |> print                     # 20

Ausführen:      rythme hallo.ry
Prüfen:         rythme check hallo.ry
Ausprobieren:   rythme          (interaktiver Modus)

Rythme hilft dir die ganze Zeit: jeder Fehler sagt, wo er ist, warum, und wie
man ihn behebt. `rythme explain E003` erklärt einen Fehlercode im Detail."#,
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
            r#"SYNTAX

Werte
  42   -7   1_000_000            ganze Zahlen (Int), beliebig groß
  3.14   2.0                     Dezimalzahlen (Float)
  "hallo"   "total: {x}"         Texte (Text); {...} (oder #{...}) fügt einen Wert ein
  true   false                   Wahrheitswerte (Bool)
  :ok   :error                   Symbole (Symbol): ein Name, der als Wert dient
  [1, 2, 3]                      Listen
  {name: "Ada", age: 36}         Records
  Some(3)   None                 ein Wert, der fehlen kann (Option)
  Ok(3)   Err("hoppla")          ein Ergebnis, das scheitern kann (Result)
  1..5                           die Liste [1, 2, 3, 4, 5]

Namen
  x = 5                          ein Name für einen Wert; er ändert sich nie
  [a, b] = [1, 2]                einen Wert zerlegen
  {name, age} = person

Operatoren
  + - * /   // (ganzzahlige Division)   % (Rest)   ** (Potenz)
  == != < <= > >=   and   or   not
  x |> f(y)                      dasselbe wie f(x, y): Schritte von links nach rechts
  x.f(y)                         auch dasselbe wie f(x, y)
  x.f                            das Feld f von x; wenn x keins hat, f(x): xs.sum
  xs                             eine Kette kann weitergehen, jede Zeile beginnt mit .
    .select(even?)
    .sum

Funktionen
  fn doppelt(x) = x * 2          in einer Zeile, mit =
  fn gruesse(name)               über mehrere Zeilen, abgeschlossen mit `end`
    nachricht = "Hallo {name}"
    print(nachricht)             die letzte Zeile liefert das Ergebnis
  end
  fn(x) = x + 1                  eine anonyme Funktion
  fn adult?(p) = p.age >= 18     ein Name darf auf ? enden (siehe rythme help ruby)

Blöcke: eine Funktion als letztes Argument eines Aufrufs
  xs.map do |x| x * 2 end        dasselbe wie map(xs, fn(x) = x * 2)
  xs.each do |x|                 über mehrere Zeilen, abgeschlossen mit `end`
    print(x)
  end
  xs.select { |x| x > 2 }        mit geschweiften Klammern

Bedingungen (jedes if hat ein else)
  if x > 0 then "positiv" else if x == 0 then "null" else "negativ"
  unless x > 0 then "klein" else "groß"     das Gegenteil von if

Nach Form entscheiden
  match list
    [] -> "leer"
    [x] -> "ein Element: {x}"
    [x, ..rest] -> "{x}, dann {len(rest)} weitere"
  end

Mehrere Schritte innerhalb eines Ausdrucks
  do
    a = 1
    a + 1
  end

Eigene Typen
  type Shape = Circle(radius) | Rect(width, height)

Tests und andere Dateien
  test "doppelt funktioniert" = assert_eq(doppelt(2), 4)
  use geometry                   dann geometry.area(2)

Kommentare
  # ein gewöhnlicher Kommentar
  ## ein Dokumentationskommentar, über einer Funktion oder einem Typ, angezeigt von help(...)"#,
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
            r#"FUNKTIONEN

Definieren und aufrufen
  fn addiere(a, b) = a + b
  addiere(2, 3)                  # 5

Über mehrere Zeilen: der letzte Ausdruck ist das Ergebnis
  fn beschreibe(n)
    art = if n % 2 == 0 then "gerade" else "ungerade"
    "{n} ist {art}"
  end

Eine Definition pro Fall (der Reihe nach ausprobiert)
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

Mit einer Bedingung (einem Guard)
  fn vorzeichen(n) if n < 0 = "negativ"
  fn vorzeichen(0) = "null"
  fn vorzeichen(_) = "positiv"

Funktionen sind Werte
  doppelt = fn(x) = x * 2
  [1, 2, 3] |> map(doppelt)      # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

Pipes liest man von links nach rechts
  "  Hallo  " |> trim |> upper   # "HALLO"
  x |> f(a, b)    bedeutet    f(x, a, b)

Methodenstil
  "hallo".upper()                # dasselbe wie upper("hallo")

Dokumentation
  ## Addiert zwei Zahlen.
  fn addiere(a, b) = a + b
  help(addiere)                  # zeigt "Addiert zwei Zahlen."

Rekursion ohne Grenze
  Wenn der rekursive Aufruf das Allerletzte ist, was eine Funktion tut, kann
  sie sich millionenfach wiederholen:
  fn countdown(0) = print("Abheben!")
  fn countdown(n)
    print(n)
    countdown(n - 1)
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
            r#"LISTEN

  numbers = [3, 1, 2]
  numbers[0]                     # 3 (das erste; -1 ist das letzte)
  len(numbers)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numbers]                 # [0, 3, 1, 2]   (.. fügt eine ganze Liste ein)
  numbers + [9]                  # [3, 1, 2, 9]

Mit der ganzen Liste arbeiten (keine Schleifen nötig)
  map(numbers, fn(x) = x * 10)            # [30, 10, 20]
  filter(numbers, fn(x) = x > 1)          # [3, 2]
  fold(numbers, 0, fn(acc, x) = acc + x)  # 6
  sort(numbers)                           # [1, 2, 3]
  numbers |> each(fn(x) = print(x))       # gibt jedes Element aus

Sicherer Zugriff (nie ein Fehler)
  first([])                      # None
  get(numbers, 10)               # None

Eine Liste „ändern“: du bekommst eine neue Liste, die alte bleibt gleich
  set_at(numbers, 0, 9)          # [9, 1, 2]
  insert_at(numbers, 1, 7)       # [3, 7, 1, 2]
  remove_at(numbers, -1)         # [3, 1]

Teile einer Liste
  take_while(numbers, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

Eine Liste mit Mustern zerlegen
  match numbers
    [] -> "empty"
    [first, ..rest] -> "{first}, then {len(rest)} more"
  end

Alle Funktionen: rythme help builtins"#,
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
            r#"TEXT

  name = "Ada"
  "Hello, {name}!"               # "Hello, Ada!"  ({...} fügt einen beliebigen Ausdruck ein)
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3 (Zeichen, Akzente mitgezählt)
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hello", "ell")       # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14" (eine Zahl mit 2 Nachkommastellen)
  pad_left("7", 3, "0")          # "007"  (und pad_right: um Spalten auszurichten)
  char_code("A")   char(66)      # 65 und "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...): eine Umwandlung, die nicht abstürzen kann
  trim_start("  x")   trim_end("x  ")
  words("Hi, you!")              # ["Hi", "you"]
  equals_ignore_case("Ada", "ADA")          # true
  format_number(1234.5, 2)       # "1,234.50"   (mit "de": "1.234,50")
  find_all("3 cats, 12 dogs", "[0-9]+")     # ["3", "12"]  (reguläre Ausdrücke: rythme help regex)

Sonderzeichen: \n (neue Zeile), \t (Tabulator), \r (Wagenrücklauf),
\0 (Nullzeichen), \" (Anführungszeichen), \\ (Backslash), \{ und \} (geschweifte
Klammern, weil { eine Einfügung beginnt), \u{e9} (ein beliebiges Zeichen über
seine Unicode-Nummer: é)."#,
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
            r#"RECORDS

Ein Record fasst benannte Werte zusammen:
  ada = {name: "Ada", born: 1815}
  ada.name                       # "Ada"
  ada["name"]                    # "Ada"

Records ändern sich nie: bau einen neuen aus einem alten
  older = {..ada, born: 1816}    # dieselben Felder, `born` ersetzt

Zerlegen
  {name, born} = ada
  fn greet({name}) = "Hello {name}"

Nützliche Funktionen
  keys(ada)   values(ada)   has(ada, "name")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["name", "Ada"], ["born", 1815]]
  from_entries([["x", 1]])       # {x: 1}

Records als Wörterbücher
  counts = {apple: 2}
  put(counts, "pear", 5)         # {apple: 2, pear: 5}
  map_values(counts, fn(n) = n * 10)      # {apple: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}
  Für Schlüssel, die keine Namen sind (Zahlen, Listen...), verwende ein Dict: rythme help dicts

JSON, das Datenformat des Webs
  parse_json(text)               # Ok(...): Objekte werden Records, Arrays Listen
  to_json(value)                 # der Text, bereit zum Speichern mit write_file"#,
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
            r#"DICTS UND MENGEN

Ein Dict (Wörterbuch) gibt für jeden Schlüssel einen Wert. Schlüssel können
alle Werte außer Funktionen sein: Zahlen, Texte, Listen, Records... Sie
bleiben in ihrer Reihenfolge.
  ages = dict([["Ada", 36], ["Alan", 41]])
  ages                           # dict("Ada" => 36, "Alan" => 41)
  ages["Ada"]                    # 36 (ein Fehler, wenn der Schlüssel fehlt)
  get(ages, "Bob")               # None: eine Option, nie ein Fehler
  grid = dict([[[0, 0], "start"]])   # eine Liste als Schlüssel
  dict({x: 1})                   # aus einem Record: dict("x" => 1)

Dicts ändern sich nie: Funktionen liefern schnell ein neues (in O(log n))
  put(ages, "Bob", 20)           # hinzugefügt (oder ersetzt)
  remove(ages, "Ada")            # ohne diesen Schlüssel
  has(ages, "Ada")   len(ages)   keys(ages)   values(ages)   entries(ages)
  map_values(ages, fn(n) = n + 1)
  filter(ages, fn(name, age) = age > 40)   # f bekommt den Schlüssel und den Wert
  ages.each do |name, age| print(name, age) end   # reject und select auch
  union(a, b)                    # beide; für einen Schlüssel in beiden gewinnt der Wert von b

Eine Menge enthält Werte ohne Wiederholung, der Reihe nach.
  s = set([3, 1, 3])             # set(1, 3)
  has(s, 3)   push(s, 5)   remove(s, 1)   len(s)   values(s)   # values: eine Liste
  union(a, b)   intersection(a, b)   difference(a, b)   is_subset(a, b)
  filter(s, is_odd)   map(s, fn(x) = x * 10)

Dicts und Mengen lassen sich mit == und < vergleichen und werden als
dict(k => v, ...) und set(a, b, ...) ausgegeben. `to_json` schreibt ein Dict,
dessen Schlüssel alle Texte (oder alle Symbole) sind, als Objekt, eine Menge als
Array; `dict(parse_json(text)?)` liest ein Objekt wieder ein."#,
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
            r#"EIGENE TYPEN

Ein Typ zählt die möglichen Formen eines Werts auf:
  type Shape = Circle(radius) | Rect(width, height) | Empty

  c = Circle(2.0)
  c.radius                       # 2.0

Jede Form mit match behandeln
  fn area(shape) = match shape
    Circle(r) -> pi * r * r
    Rect(w, h) -> w * h
    Empty -> 0
  end

Oder mit einer Definition pro Form
  fn perimeter(Circle(r)) = 2 * pi * r
  fn perimeter(Rect(w, h)) = 2 * (w + h)
  fn perimeter(Empty) = 0

Wenn du eine Form vergisst, sagt dir `rythme check`, welche.

Ein Typ kann sich selbst enthalten (Bäume...):
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
            r#"MATCH UND MUSTER

`match` vergleicht einen Wert der Reihe nach mit Mustern und liefert das
Ergebnis des ersten, das passt:

  fn describe(x) = match x
    0 -> "zero"
    1 | 2 | 3 -> "small"
    n if n < 0 -> "negative"
    _ -> "big"
  end

Muster
  42   "text"   true             genau dieser Wert
  1..9   -5..-1   0.0..0.5       eine Zahl von der ersten bis zur zweiten, beide inklusive
  "Mr " + name                   ein Text, der mit "Mr " beginnt; name ist der Rest
  x                              irgendetwas, x genannt
  _                              irgendetwas, ignoriert
  Some(x)   Circle(r)            ein Konstruktor und was er enthält
  []   [a, b]   [first, ..rest]  Listen, nach ihrer Form
  {name, age}                    ein Record mit diesen Feldern
  p if condition                 ein Muster und eine Bedingung (Guard)
  a | b                          das eine Muster oder das andere

Muster funktionieren überall, wo ein Name vergeben wird:
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
            r#"FEHLER SIND WERTE (Option, Result, ?)

Rythme hat kein null und keine Exceptions. Ein Wert, der fehlen kann, ist eine
Option: Some(value) oder None. Eine Operation, die scheitern kann, liefert ein
Result: Ok(value) oder Err(message). Du kannst nicht vergessen, sie zu behandeln.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

  match divide(10, 2)
    Ok(x) -> print("result: {x}")
    Err(e) -> print("error: {e}")
  end

Der Operator ?: den Wert nehmen, oder anhalten und den Fehler weitergeben
  fn average(a, b)
    x = parse_float(a)?          # wenn das Err(...) ist, gibt average es zurück
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

Schnelle Werkzeuge
  unwrap_or(get(list, 5), 0)     # der Wert, oder ein Standardwert
  unwrap(Some(3))                # 3 (hält das Programm bei None oder Err an)
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(person, "age"), "no age")   # Err("no age"), wenn es fehlt: bereit für ?
  and_then(x, f)                 # ein weiterer Schritt, der scheitern kann
  map_err(r, fn(e) = "config: {e}")     # einen Fehler besser erklären

Um das ganze Programm anzuhalten: fail("message")."#,
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
            r#"FUNKTIONAL DENKEN

In Rythme ändern sich Werte nie, und es gibt keine Schleifen. Das mag seltsam
wirken, aber Programme werden leichter zu lesen, zu testen und zu verstehen:
eine Funktion liefert für dieselben Argumente immer dasselbe Ergebnis.

Statt einer Schleife, die ausgibt:      1..5 |> each(fn(i) = print(i))
Statt einer Schleife, die umwandelt:    map(list, f)
Statt einer Schleife, die auswählt:     filter(list, f)
Statt einer Schleife, die aufsummiert:  fold(list, start, f)   oder sum, product
Statt laufender Summen:                 scan(list, start, f)
Statt einer Schleife, die sucht:        find, any, all, index_of, min_by, max_by
Statt "break":                          take_while(list, f)
Statt "while":                          eine rekursive Funktion

  # Python:  total = 0
  #          for x in numbers: total += x * x
  total = numbers |> map(fn(x) = x * x) |> sum

  # Eine "while"-Schleife als rekursive Funktion:
  fn collatz(1, steps) = steps
  fn collatz(n, steps) =
    if n % 2 == 0 then collatz(n // 2, steps + 1) else collatz(3 * n + 1, steps + 1)
  collatz(27, 0)                 # 111

Statt einen Wert zu ändern, erzeuge einen neuen:
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
            r#"TESTS

Schreib Tests direkt neben deinen Code:
  fn double(x) = x * 2

  test "double of 2 is 4" = assert_eq(double(2), 4)

  test "several checks"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "should stay negative")
  end

Ausführen:
  rythme test my_file.ry            die Tests einer Datei
  rythme test .                     aller .ry-Dateien eines Ordners (und darunter)
  rythme test . --filter double     nur die Tests, deren Name "double" enthält
  rythme test . --watch             erneut, sobald sich eine Datei ändert (Strg+C beendet)
  rythme test my_file.ry -- a b     mit `args()`, das ["a", "b"] liefert
  rythme test                       in einem Projekt: seine Dateien mit Tests

Beim Testen werden Ausdrücke auf oberster Ebene wie print(...) nicht ausgeführt:
die Definitionen werden geladen, dann läuft jeder Test. Ein Test besteht, wenn
er ohne Fehler endet und sein Wert nicht false, Err(...) oder None ist. Die
Zusammenfassung zeigt die Anzahlen und die Zeit; der Exit-Code ist 0, wenn
alle Tests bestehen.

Eigenschaftstests: prüfen, dass etwas für viele Werte gilt
  test "reversing twice gives the list back" =
    check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

Generatoren: ints(min, max), floats(min, max), texts(max_length), bools(),
lists_of(generator, max_length), one_of(list), records_of({x: ints(), y: texts()}).
`sample(generator)` zeigt, was einer liefert. Wenn ein Wert die Eigenschaft
falsch macht, wird er vereinfacht (Shrinking) und mit dem Seed gezeigt, der ihn
erzeugt hat: dieselben Werte kommen bei jedem Lauf wieder, und `--seed N`
probiert andere (`--seed random` wählt einen)."#,
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
            r#"ÜBUNGEN

Rythme bringt 30 abgestufte Übungen für einen ersten Kurs mit, von den ersten
Werten bis zu Rekursion, Listen, Records, Typen, Option, Result und Tests. Jede
ist eine Datei mit einer Aufgabe, Tipps, den zu schreibenden Funktionen und
ihren Tests.

  rythme exercise list              die Übungen, von ★☆☆ (leicht) bis ★★★
  rythme exercise start 7           kopiert Übung 7 in den aktuellen Ordner
  rythme exercise check 07-recursion.ry
                                    führt ihre Tests aus und sagt, was als Nächstes zu tun ist

Ersetze in der Datei jedes fail("to do") durch deinen Code; ändere die Tests
nicht. `rythme test datei.ry` führt die Tests ebenfalls aus. Die Übungen liegen
auch im Ordner docs/exercises des Rythme-Repositorys, auf Englisch und
Französisch."#,
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
            r#"MEHRERE DATEIEN

  # geometry.ry
  ## Area of a rectangle.
  fn area(w, h) = w * h
  fn _check(x) = x >= 0          # privat: beginnt mit `_`

  # main.ry
  use geometry                    # die Datei geometry.ry, neben main.ry
  print(geometry.area(2, 3))      # 6

`use geometry` gibt die Definitionen von geometry.ry über ihren Namen:
`geometry.area`, `geometry.Circle(1)`, und in einem `match` `geometry.Circle(r)`.
So siehst du immer, woher ein Name kommt. Eine Datei in einem Ordner:
`use shapes/circle`, dann `circle.perimeter(1)`. Ein anderer Name: `use
geometry as geo`, dann `geo.area(2, 3)`. Namen, die mit `_` beginnen, bleiben
privat in ihrer Datei.

Der Pfad ist relativ zum Ordner der Datei, die `use` enthält. Jede Datei läuft
nur einmal. Rythme sagt dir, welche Datei einen Namen definiert, den du zu
verwenden vergessen hast.

Ältere Form: `use "geometry.ry"` (mit Anführungszeichen) gibt ihre Definitionen
direkt, wie `area(2, 3)`; besser ist `use geometry`."#,
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
            r#"DATEIEN UND DAS SYSTEM

Jede Funktion, die das System berührt, gibt ein Result zurück: Ok(...), oder
Err(message) mit einer Meldung, die sagt, was schiefgegangen ist.

Dateien und Ordner (relative Pfade beginnen im aktuellen Ordner)
  read_file("notes.txt")         # Ok("...") oder Err("file not found: notes.txt")
  write_file("out.txt", "hi")    # Ok(()), ersetzt den Inhalt
  append_file("log.txt", "done\n")
  file_exists("notes.txt")       # true oder false
  file_info("notes.txt")         # Ok({is_folder: false, size: 11}), in Bytes
  list_folder(".")               # Ok(["notes.txt", "photos"]), sortiert
  create_folder("backup/2024")   # Ok(()), mit den fehlenden Ordnern drumherum
  delete_file("old.txt")         # Ok(()); niemals ein Ordner

Pfade, die unter Linux, macOS und Windows funktionieren
  join_path(["data", "notes.txt"])       # "data/notes.txt"
  split_path("/home/ada/notes.txt")      # ["/", "home", "ada", "notes.txt"]
  split_path("C:\Users\ada")             # ["C:\", "Users", "ada"]

Die Kommandozeile: rythme tool.ry --verbose --name=Ada notes.txt
  args()                         # ["--verbose", "--name=Ada", "notes.txt"]
  options()                      # Ok({args: ["notes.txt"], verbose: true, name: "Ada"})
  opts = unwrap(options())
  name = unwrap_or(get(opts, "name"), "World")   # eine Option, die fehlen kann
  env("HOME")                    # Some("/home/ada") oder None

Die Standardeingabe: rythme count.ry < names.txt
  input("Name? ")                # eine Zeile
  input_lines()                  # Ok(["Ada", "Alan"]): alle Zeilen bis zum Ende

Exit-Codes, für die Skripte, die dein Programm ausführen
  0    alles ist gut gegangen (auch am Ende des Programms)
  1    ein Fehler im Programm: Syntax, Prüfung, beim Ausführen, fehlgeschlagener Test
  2    eine falsche Verwendung des Befehls (unbekannter Befehl oder unbekannte Option)
  101  ein interner Fehler von Rythme (ein Bug, bitte melden)
  exit(code)                     # hält sofort mit diesem Code an, von 0 bis 255

Im Playground, im Browser, gibt es keine Dateien und keine Eingabe: diese
Funktionen liefern Err("... is not available in the browser ...")."#,
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
            r#"PROJEKTE UND PAKETE

  rythme new hello        # ein Ordner hello/ mit einem neuen Projekt:
                          #   rythme.toml, src/main.ry, src/gruss.ry,
                          #   tests/gruss_test.ry, README.md, .gitignore
  cd hello
  rythme run              # führt das Programm aus: `entry` aus rythme.toml
  rythme test             # führt die Dateien mit Tests aus src/ und tests/ aus
  rythme check            # prüft jede Datei; auch `rythme fmt`, `rythme doc`

rythme.toml beschreibt das Projekt:

  name = "hello"
  version = "0.1.0"
  entry = "src/main.ry"

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

Eine Abhängigkeit kommt aus git, bei einem `tag` oder einem `commit`, oder aus
einem Ordner. `rythme install` lädt sie herunter (mit git) und schreibt
rythme.lock, den genauen Commit jeder einzelnen: behalte die Datei im Projekt,
damit alle denselben Code bekommen. Dann:

  use mathlib/vectors       # die Datei src/vectors.ry von mathlib
  use mathlib               # sein Programm, der `entry` seiner rythme.toml
  use hello/gruss           # eine Datei des Projekts selbst, aus tests/

Eine Datei neben der Datei, die `use` enthält, hat Vorrang. Pakete aus git
werden in ~/.rythme/packages aufbewahrt (oder RYTHME_HOME/packages)."#,
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
            r#"DATUM UND UHRZEIT

Ein Datum ist ein Record: {year, month, day}, mit hour, minute und second für
einen Zeitpunkt des Tages, und offset (Minuten vor UTC), wenn er bekannt ist.
  today()                        # {year: 2026, month: 10, day: 18}
  now()                          # {year: 2026, ..., hour: 14, minute: 5, second: 9, offset: 120}
  birthday = {year: 1815, month: 12, day: 10}
  birthday.year                  # 1815

Als Text und zurück, im Format ISO 8601
  parse_date("2024-02-29")       # Ok({year: 2024, month: 2, day: 29})
  parse_date("2024-02-30")       # Err("... Februar 2024 ...")
  parse_date("2024-02-29T13:05:00+01:00")    # mit Uhrzeit und Versatz
  format_date({year: 2024, month: 2, day: 29, hour: 9})   # "2024-02-29T09:00:00"

Kalenderrechnung (Schaltjahre und Monatslängen inklusive)
  add_days({year: 2024, month: 2, day: 28}, 1)     # {year: 2024, month: 2, day: 29}
  add_seconds(now(), 3600)       # in einer Stunde
  days_between(today(), {year: 2027, month: 1, day: 1})   # verbleibende Tage
  seconds_between(a, b)          # negativ, wenn b vor a liegt
  day_of_week(today())           # 1 = Montag ... 7 = Sonntag
  days_in_month(2023, 2)         # 28
  is_leap_year(2000)             # true
  is_date({year: 2023, month: 2, day: 29})   # false

Ein Datum, das es nicht gibt, hält die Funktionen mit E023 an, der sagt, warum.
Der lokale Versatz kommt vom System (UTC, wenn es nichts sagt); time() liefert
die Sekunden seit 1970 als Float, um Dauern zu messen."#,
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
            r#"REGEX: MIT EINEM MUSTER SUCHEN

Ein regulärer Ausdruck (Regex) beschreibt Textstücke: "[0-9]+" bedeutet „eine
Ziffer oder mehr“. Rythme liest die üblichen regulären Ausdrücke, und eine Suche
dauert nie länger als (Länge des Textes) × (Größe des Ausdrucks).

  matches("2024", "[0-9]+")                  # true: der GANZE Text passt
  find_first("tel: 06 12", "[0-9]+")         # Some("06"): das erste Stück
  find_all("3 cats, 12 dogs", "[0-9]+")      # ["3", "12"]
  split_by("a, b;c", "[,;] *")               # ["a", "b", "c"]
  replace_all("a cat", "\\b\\w", upper)      # "A Cat" (eine Funktion für jedes Stück)
  replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1")   # "2024-02-29"
  captures("2024-02", "([0-9]+)-([0-9]+)")   # Some(["2024-02", "2024", "02"])
  named_captures("2024-02", "(?<year>[0-9]+)-(?<month>[0-9]+)")
                                             # Some({year: "2024", month: "02"})

Was ein regulärer Ausdruck sagen kann
  abc          diese Zeichen, in dieser Reihenfolge
  .            jedes Zeichen außer einem Zeilenumbruch
  [abc]        eines dieser Zeichen; [a-z] eines von a bis z; [^0-9] jedes andere
  \d \w \s     eine Ziffer (0-9); ein Buchstabe (mit Akzenten), eine Ziffer oder _; ein Leerzeichen
  \D \W \S     alles andere
  \b           der Rand eines Wortes
  ^  $         der Anfang und das Ende des Textes
  x|y          x oder y
  (...)        eine Gruppe: um sie zu wiederholen oder ihren Text zu erfassen ($1, `captures`)
  (?:...)      eine Gruppe, die nichts erfasst; (?<name>...) eine Gruppe mit Namen
  x*  x+  x?   x null Mal oder öfter, einmal oder öfter, null oder einmal (das Längste)
  x{3}  x{2,5}  x{2,}    3 Mal, 2 bis 5 Mal, 2 Mal oder öfter
  x*?  x+?     das Kürzeste statt des Längsten
  (?i)         am Anfang: Groß- und Kleinbuchstaben gelten als gleich

Einen regulären Ausdruck in einem Rythme-Text schreiben
  Ein Backslash wird doppelt geschrieben: "\\d+" ist der Ausdruck \d+. Geschweifte
  Klammern werden \{ und \} geschrieben: "[0-9]\{4\}" ist der Ausdruck [0-9]{4}.
  Für ein Zeichen selbst setze es in eckige Klammern: "[.]" ist ein Punkt, "[(]"
  eine Klammer, "[$]" ein Dollar.

Ein Fehler in einem regulären Ausdruck hält das Programm mit E026 an, der zeigt,
wo er ist; wenn der Ausdruck im Code steht, findet `rythme check` ihn vorher."#,
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
            r#"DER INTERAKTIVE MODUS

Starte `rythme` ohne Datei. Tippe einen Ausdruck und sieh seinen Wert:
  ry> 1 + 2
  3
Definitionen bleiben von einer Zeile zur nächsten erhalten. Wenn eine Zeile
nicht fertig ist (offene Klammer, Block ohne `end`...), geht sie in der
nächsten Zeile weiter.

Befehle
  :help           diese Hilfe            :quit          beenden (oder Strg-D)
  :doc map        Hilfe zu einem Namen   :load f.ry     eine Datei laden
  :reset          alles vergessen        :lang de       die Sprache ändern (en, fr, es, de, pt, zh)
  :learn          geführte Tour          :explain E003  einen Fehler erklären
  :steps fib(4)   die Rechnung Schritt für Schritt zeigen
Die Tab-Taste vervollständigt Namen."#,
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
            r#"FORMATIEREN: rythme fmt

`rythme fmt` gibt dem Code das offizielle Layout von Rythme: jedes Programm
sieht gleich aus, und niemand muss über Leerzeichen streiten.

  rythme fmt hello.ry            eine Datei formatieren (sie wird neu geschrieben)
  rythme fmt .                   alle .ry-Dateien eines Ordners und seiner Unterordner
  rythme fmt --check .           nur prüfen: listet die zu formatierenden Dateien und
                                 schlägt fehl (Exit-Code 1), wenn es welche gibt, für CI
  rythme fmt - < hello.ry        liest den Code, schreibt das Ergebnis

Der Stil
  - 2 Leerzeichen pro Einrückungsebene
  - Leerzeichen um Operatoren: x = a + b * 2   xs |> map(f)   n -> n * 2
  - keine in Klammern oder um ..: f(x)   [1, 2]   {name: "Ada"}   1..10
  - eine Pipeline über mehrere Zeilen: ein Schritt pro Zeile
      total = prices
        |> filter(fn(p) = p > 10)
        |> sum
  - ein Typ über mehrere Zeilen: eine Variante pro Zeile
      type Shape =
        | Circle(radius)
        | Rect(width, height)
  - eine Liste über mehrere Zeilen endet mit einem Komma
  - ein Block in einer Zeile bleibt in einer Zeile:
      xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
    über mehrere Zeilen stehen seine Anweisungen eine Ebene tiefer als `do`
  - eine Anweisung pro Zeile (kein `;`), und `end` in einer eigenen Zeile
  - eine Leerzeile um eine Definition über mehrere Zeilen; nie zwei
    Leerzeilen hintereinander
  - jeder Kommentar bleibt erhalten; die Kommentare am Ende aufeinanderfolgender
    Zeilen werden ausgerichtet

Zeilenumbrüche bleiben dort, wo die Sprache eine Wahl lässt, und lange Zeilen
bleiben, wie sie geschrieben sind. Der Formatierer ändert nie, was ein Programm
tut: eine Datei mit einem Syntaxfehler bleibt, wie sie ist, und der Fehler wird
angezeigt."#,
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
            r#"DOKUMENTATION: rythme doc

`rythme doc` schreibt die Dokumentation deines Codes aus seinen `##`-Kommentaren,
seinen Typen und den Definitionen seiner Funktionen.

  rythme doc geometry.ry         Markdown auf dem Bildschirm
  rythme doc .                   alle .ry-Dateien eines Ordners und seiner Unterordner
  rythme doc --html geometry.ry  eine Webseite, die nichts anderes braucht
  rythme doc . --out doc.html    in eine Datei geschrieben (.html ergibt die Webseite)

Schreib einen `##`-Kommentar direkt über das, was er dokumentiert:

  ## Shapes and their areas.      <- ganz oben, gefolgt von einer Leerzeile:
                                     er beschreibt die ganze Datei
  ## A shape to draw.
  type Shape = Circle(radius) | Rect(width, height)

  ## The area of a shape. See `len` and `area`.
  fn area(Circle(r)) = pi * r * r
  fn area(Rect(w, h)) = w * h

Was geschrieben wird
  - für jede Datei: ihre Beschreibung, die Dateien, die sie verwendet (ebenfalls
    dokumentiert), ihre Typen, ihre Funktionen (die erste Zeile jeder
    Definition), ihre Werte und die Namen ihrer Tests
  - Namen, die mit `_` beginnen, werden ausgelassen: sie bleiben in ihrer Datei
  - ein Name in Backticks in einem Kommentar, wie `area` oder `len(list)`,
    wird ein Link zu seiner Dokumentation; nach `use geometry` führt
    `geometry.area` zu `area` in geometry.ry; die so genannten eingebauten
    Funktionen werden am Ende erklärt
  - die Überschriften sind in der Sprache der Meldungen (`--lang de`)

In einem Kommentar beginnt eine leere `##`-Zeile einen neuen Absatz, und Zeilen,
die mit `- ` beginnen, bilden eine Liste."#,
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
            r#"SCHRITT FÜR SCHRITT: rythme steps

`rythme steps` zeigt, wie ein Programm seine Werte berechnet, einen Schritt nach
dem anderen: jeder Aufruf wird durch den Rumpf seiner Funktion ersetzt, mit den
Werten der Argumente an ihrer Stelle, bis nur noch das Ergebnis übrig ist.

  rythme steps fib.ry              jede Definition und jeder Ausdruck der Datei
  rythme steps fib.ry "fib(4)"     ein Ausdruck, nach den Definitionen der Datei
  rythme steps fib.ry --max 50     höchstens 50 Schritte jedes Ausdrucks zeigen
  rythme steps fib.ry --width 80   Zeilen kürzen, die länger als 80 Zeichen sind

Mit  fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

  fib(3)
    → fib(2) + fib(1)
    → fib(1) + fib(0) + fib(1)
    → 1 + fib(0) + fib(1)
    → 1 + 0 + fib(1)
    → 1 + fib(1)
    → 1 + 1
    → 2

Ein Schritt ist ein Aufruf oder eine Operation. Der Rumpf einer aufgerufenen
Funktion wird schon so weit vereinfacht gezeigt, wie es geht, ohne eine andere
Funktion aufzurufen. Eingebaute Funktionen (und die Funktionen anderer Dateien)
liefern ihr Ergebnis in einem Schritt. Was das Programm ausgibt, erscheint
zwischen den Schritten. Nach den gezeigten Schritten (standardmäßig 200) wird
der Rest berechnet, ohne gezeigt zu werden.

Im interaktiven Modus: :steps fib(4). Im Playground: der Knopf Steps.
Auf Französisch: rythme etapes."#,
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
            r#"RYTHME IN DEINEM EDITOR

Rythme hilft auch im Editor: Probleme beim Tippen, die Dokumentation einer
Funktion unter der Maus (mit der aus ihrem Code abgeleiteten Signatur, wie
`fn area(side: Int|Float) -> Int|Float`), Vervollständigung, die Parameter
eines Aufrufs, Gehe zu Definition, Verweise, Umbenennen, schnelle Korrekturen
für Tippfehler, Formatieren wie `rythme fmt`, die Typen der Namen, wenn sie
sicher sind (und die Signaturen der Funktionen), Farben, die zeigen, was jeder
Name ist, "Run" und "Run test" über dem Code, die Namen des ganzen
Arbeitsbereichs, Einklappen. Das ist der Sprachserver, den der Editor selbst
startet:
  rythme lsp

VS Code
  die Erweiterung in editors/vscode: Farben, Snippets und der Sprachserver.
  Einstellungen: rythme.path (das Programm), rythme.language (en, fr, es, de, pt, zh oder auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  jeder Editor, der das Language Server Protocol spricht: sag ihm, dass er
  `rythme lsp` für .ry-Dateien starten soll. Fertige Konfigurationen: editors/README.md
  ("Run" und "Run test" nennen die Befehle rythme.runFile, rythme.testFile und
  rythme.runTest, die der Editor ausführt: siehe editors/README.md)

Zed
  die Erweiterung in editors/zed: Farben und der Sprachserver.

Farben in Neovim, Helix und Zed
  die Tree-sitter-Grammatik in editors/tree-sitter-rythme (siehe editors/README.md).

Die Sprache der Meldungen folgt dem Editor, außer wenn `--lang` (en, fr, es, de, pt, zh)
oder RYTHME_LANG sie festlegt: rythme --lang de lsp"#,
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
            r#"VON RUBY KOMMEND

Rythme liest sich wie Ruby, aber seine Werte ändern sich nie: keine Mutation,
kein nil, keine Exceptions. Was du aus Ruby kennst, funktioniert meistens:

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

Dasselbe wie in Ruby
  Aufrufe         puts "hi"   print x, y   assert_eq total, 42
                  (am Anfang einer Anweisung oder eines Ergebnisses; siehe unten)
  Blöcke          xs.map do |x| x * 2 end      xs.map { |x| x * 2 }
                  ein Block ist eine Funktion: dasselbe wie fn(x) = x * 2
  Methoden        xs.sum   name.upcase   xs.first(2)   xs.include?(3)
  Namen mit ?     even?   empty?   include?(x)   fn adult?(p) = ...
  Symbole         :ok   :error (mit == verglichen, in match verwendet)
  Texte           "Hello #{name}" (oder "Hello {name}"); "\#{" schreibt #{
  unless          unless cond then a else b
  Ruby-Namen      puts to_s to_i to_f length size upcase downcase strip
                  select reject reduce inject uniq each_slice each_cons
                  each_with_index include? empty? even? odd? any? all?
                  start_with? end_with? key? lstrip rstrip casecmp?
                  (siehe rythme help builtins)

Was anders ist
  Klammern        ein Aufruf innerhalb eines Ausdrucks braucht sie: x = len(xs),
                  nicht x = len xs
  ohne Mutation   kein x += 1: gib dem neuen Wert einen neuen Namen, total2 = total + 1
  ohne nil        ein fehlender Wert ist None, ein vorhandener Some(x)
  ohne Ausnahmen  Fehler sind Werte, Ok(x) oder Err(message), weitergegeben mit ?
  if und unless   haben immer ein else: sie liefern einen Wert; kein `return`, die
                  letzte Zeile einer Funktion ist ihr Ergebnis
  ohne Klassen    Records {name: "Ada"} und Typen: type Shape = Circle(r) | Square(s)
  ohne Schleifen  each, map, select, reduce, oder eine Funktion, die sich selbst aufruft
  case/when       match value ... end, ein Fall pro Zeile: Muster -> Ergebnis
  Hashes          dict([[:ada, 36]]) (Schlüssel jeder Art, auch Symbole), oder
                  Records {name: "Ada", age: 36}

Aufrufe ohne Klammern
  Eine Anweisung, die mit einem Namen beginnt, gefolgt in ihrer Zeile vom Anfang
  eines Werts (eine Zahl, ein Text, ein Symbol, ein Name, ein Konstruktor, not,
  if, match, fn), ist ein Aufruf: print x, y ist print(x, y). Ein do-Block nach
  den Argumenten wird dem Aufruf übergeben: each xs do |x| ... end. Wie in Ruby
  beginnen auch ein [ oder ein - nach einem Leerzeichen, ohne Leerzeichen nach
  dem -, die Argumente (puts [1, 2], puts -1), außer wenn der Code dem Namen einen
  Wert gibt (mit =, als Parameter oder in einem Muster): dann ist x -1 gleich
  x - 1 und xs [0] gleich xs[0], wie bisher. `rythme fmt` schreibt diese als
  x - 1 und xs[0].

  Ein Ergebnis kann genauso beginnen: ein Zweig von if oder unless, ein Fall von
  match, der Rumpf nach dem = einer Funktion oder eines Tests. Seine Argumente
  enden bei else, end, }, ) oder am Zeilenende:

  fn greet(name) = puts "Hello, #{name}!"
  fn check(ok) = if ok then puts "yes" else puts "no"
  match name
    "Dr " + rest -> puts "doctor", rest
    _ -> puts name
  end
  xs.each { |x| puts x }

Wie x.name funktioniert
  x.name liest das Feld `name` eines Records; wenn x kein solches Feld hat, ruft
  es die Funktion `name` mit x auf: xs.sum ist sum(xs), xs.first(2) ist
  first(xs, 2). Eine Kette kann über mehrere Zeilen gehen, jede beginnt mit `.`.

Wann ? Teil eines Namens ist
  even?(4), xs.include?(3), fn adult?(p): direkt nach einem Namen, vor `(`.
  even? ohne `(`: wenn es eine solche Funktion gibt (eingebaut, oder mit fn in
  der Datei definiert). Sonst ist value? das ?, das bei None oder Err anhält,
  und (value)? ist es immer."#,
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
            r#"E001 — Syntaxfehler

Der Code folgt an dieser Stelle nicht der Grammatik von Rythme. Die Meldung
sagt, was erwartet wurde. Häufige Ursachen:
  - eine fehlende `)`, `]` oder `,`
  - `then` oder `else` fehlt in einem `if`
  - geschweifte Klammern { } für einen Block (Rythme verwendet eine neue Zeile und `end`)
  - ein Schlüsselwort aus einer anderen Sprache (for, while, return, let, def...)

Falsch:  if x > 0 { print(x) }
Richtig: if x > 0 then print(x) else print("not positive")"#,
        ),
        (
            r#"E002 — Text never closed

A text starts with " and must end with " on the same text. A `{` inside a
text starts an insertion and must be closed by `}`.

Wrong:   print("hello)
Right:   print("hello")
To write a brace in a text: "\{not inserted\}""#,
            r#"E002 — Text nie geschlossen

Ein Text beginnt mit " und muss im selben Text mit " enden. Eine `{` in einem
Text beginnt eine Einfügung und muss mit `}` geschlossen werden.

Falsch:  print("hello)
Richtig: print("hello")
Eine geschweifte Klammer in einem Text schreiben: "\{nicht eingefügt\}""#,
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
            r#"E003 — Unbekannter Name

Dieser Name ist an dieser Stelle nicht definiert. Prüfe die Schreibweise
(Groß- und Kleinbuchstaben sind verschieden) und ob der Name definiert ist,
bevor er verwendet wird. In einer Funktion sind die Parameter und die weiter
oben definierten Namen sichtbar, und lokale Funktionen, die direkt
hintereinander stehen, sehen sich gegenseitig; auf oberster Ebene sind alle
Funktionen überall sichtbar.

Falsch:  print(lenght([1, 2]))
Richtig: print(len([1, 2]))

Rythme schlägt den ähnlichsten vorhandenen Namen vor, wenn es einen gibt, und
die Datei, die ihn definiert, wenn es eine andere Datei des Ordners ist:

  use geometry                   # dann
  print(geometry.area(2))        # nicht area(2)

Ein Name, der mit `_` beginnt, ist privat in seiner Datei: andere Dateien
können ihn nicht verwenden."#,
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
            r#"E004 — Falsche Anzahl von Argumenten

Die Funktion wird mit mehr oder weniger Werten aufgerufen, als sie nimmt. Denk
daran, dass bei `|>` der Wert links das erste Argument ist.

Falsch:  fn add(a, b) = a + b
         add(1)
Richtig: add(1, 2)
Auch richtig:  1 |> add(2)

`rythme help NAME` zeigt, wie man eine eingebaute Funktion aufruft."#,
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
            r#"E005 — Falsche Art von Wert

Die Operation funktioniert nicht mit dieser Art von Wert. Rythme wandelt Werte
nie stillschweigend um: "1" + 1 ist ein Fehler, nicht "11" oder 2.

Falsch:  "age: " + 36
Richtig: "age: {36}"              (den Wert in den Text einfügen)
Richtig: "age: " + text(36)       (ihn zuerst umwandeln)

Bedingungen müssen true oder false sein:
Falsch:  if len(list) then ... else ...
Richtig: if len(list) > 0 then ... else ...

Wenn die Arten der Werte sicher sind, findet `rythme check` diesen Fehler, bevor
das Programm läuft, auch in Code, der nicht jedes Mal läuft, und in den
Aufrufen, die einer Funktion etwas geben, das sie nicht verwenden kann:
`area("big")`, wenn `area(side)` `side * side` berechnet."#,
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
            r#"E006 — Kein Fall passt

Keiner der Fälle eines `match` (oder der Definitionen einer Funktion, oder das
Muster einer Definition wie `[a, b] = list`) akzeptiert diesen Wert.

Falsch:  fn name(1) = "one"
         name(2)
Richtig: fn name(1) = "one"
         fn name(_) = "another number"

`rythme check` warnt vor dem Ausführen vor vergessenen Fällen (W002). Wenn die
Art des Werts sicher ist, meldet es vor dem Ausführen auch einen Fall, der nie
passen kann: `[first, ..rest]` für einen Text, `Circle(r)` für eine Zahl."#,
        ),
        (
            r#"E007 — Index out of range

Positions in a list or a text go from 0 to length - 1 (and -1 is the last).

Wrong:   [10, 20][2]
Right:   [10, 20][1]              # 20
Safe:    get([10, 20], 2)         # None, never an error"#,
            r#"E007 — Index außerhalb des Bereichs

Positionen in einer Liste oder einem Text gehen von 0 bis Länge - 1 (und -1 ist
die letzte).

Falsch:  [10, 20][2]
Richtig: [10, 20][1]              # 20
Sicher:  get([10, 20], 2)         # None, nie ein Fehler"#,
        ),
        (
            r#"E008 — Division by zero

Dividing by zero has no meaning. Check the divisor before, or return a Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
            r#"E008 — Division durch null

Eine Division durch null hat keine Bedeutung. Prüfe den Teiler vorher, oder gib
ein Result zurück.

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
            r#"E009 — Keine Funktion

Klammern nach einem Wert rufen ihn auf, aber nur Funktionen können aufgerufen
werden.

Falsch:  x = 5
         x(3)
Falsch:  None()                   (None ist ein Wert)
Richtig: None
Zum Multiplizieren schreib es:  2 * (x + 1)"#,
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
            r#"E010 — Unbekanntes Feld

Der Record (oder der Konstruktor) hat kein Feld mit diesem Namen; oder das Dict
hat keinen solchen Schlüssel (`dict[key]`).

Falsch:  {name: "Ada"}.nmae
Richtig: {name: "Ada"}.name
Sicher:  get(record, "name")     # eine Option, nie ein Fehler
         get(dict, key)          # dasselbe für ein Dict

Wenn die Felder des Records sicher sind, findet `rythme check` diesen Fehler,
bevor das Programm läuft, und schlägt das ähnlichste Feld vor."#,
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
            r#"E011 — Schon definiert

In Rythme bedeutet ein Name immer denselben Wert: er kann nicht geändert oder
zweimal an derselben Stelle definiert werden. Das macht Programme leicht
nachvollziehbar.

Falsch:  x = 1
         x = x + 1
Richtig: x = 1
         y = x + 1

Für eine Funktion mit mehreren Fällen schreib die Definitionen direkt
hintereinander:
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

Der Name, der einer verwendeten Datei gegeben wird (`use geometry`, `use ... as
geo`), kann in der Datei nichts anderes bezeichnen; und zwei Dateien, die mit
Anführungszeichen verwendet werden (`use "a.ry"`), können nicht denselben Namen
definieren: verwende sie stattdessen über ihre Namen (`use a`, dann `a.area`)."#,
        ),
        (
            r#"E012 — Number too large (no longer given)

Whole numbers (Int) have no limit any more: they grow as needed, and stay
exact. Earlier versions of Rythme stopped with this error beyond 64 bits.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (a Float is approximate)

Only a whole number of more than 100000 digits stops the program, with
E023: it is usually a calculation that grows without end."#,
            r#"E012 — Zahl zu groß (gibt es nicht mehr)

Ganze Zahlen (Int) haben keine Grenze mehr: sie wachsen nach Bedarf und bleiben
genau. Frühere Versionen von Rythme hielten jenseits von 64 Bit mit diesem
Fehler an.

  2 ** 100        # 1267650600228229401496703205376
  2.0 ** 100      # 1.2676506002282294e30 (ein Float ist ungefähr)

Nur eine ganze Zahl mit mehr als 100000 Ziffern hält das Programm an, mit E023:
meistens ist es eine Rechnung, die ohne Ende wächst."#,
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
            r#"E013 — unwrap hat keinen Wert gefunden

`unwrap(x)` holt den Wert aus Some(...) oder Ok(...) heraus und hält das
Programm bei None oder Err(...) an. Behandle stattdessen den fehlenden Fall:

  match get(list, 5)
    Some(x) -> x
    None -> 0
  end
oder einfach:  unwrap_or(get(list, 5), 0)"#,
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
            r#"E014 — Zu viele verschachtelte Aufrufe

Eine Funktion ruft sich selbst (oder andere) zu oft auf, ohne fertig zu werden.
Entweder erreicht sie nie ihren Basisfall, oder sie muss so geschrieben werden,
dass der rekursive Aufruf das Allerletzte ist, was sie tut (ein Endaufruf):
solche Aufrufe haben keine Grenze.

Begrenzt:    fn sum([]) = 0
             fn sum([x, ..rest]) = x + sum(rest)       # + kommt danach
Unbegrenzt:  fn sum([], acc) = acc
             fn sum([x, ..rest], acc) = sum(rest, acc + x)
Einfacher:   sum(list)  oder  fold(list, 0, fn(a, x) = a + x)"#,
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
            r#"E015 — Zusicherung fehlgeschlagen

`assert(condition)` oder `assert_eq(actual, expected)` hat festgestellt, dass
etwas nicht wie erwartet ist. In einem Test heißt das, dass der Test
fehlschlägt: die Meldung zeigt die zu vergleichenden Werte.

`rythme check` findet vor dem Ausführen ein `assert_eq`, das nie bestehen kann,
weil seine beiden Werte von Arten sind, die nie gleich sind:
Falsch:  assert_eq(parse_int("3"), 3)      # ein Result ist nie ein Int
Richtig: assert_eq(parse_int("3"), Ok(3))

`check_all(property, generator)` meldet E015 auch, wenn ein Wert die Eigenschaft
falsch macht: es zeigt den einfachsten gefundenen Wert und den Seed, der ihn
geliefert hat (dieselben Werte kommen bei jedem Lauf wieder; `rythme test --seed N`
probiert andere)."#,
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
            r#"E016 — Falsche Verwendung von ?

`x?` holt den Wert aus Some(...) oder Ok(...) heraus; bei None oder Err(...)
lässt es die aktuelle Funktion diesen sofort zurückgeben. Es funktioniert also
nur innerhalb einer Funktion, bei einer Option oder einem Result.

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
            r#"E017 — Problem mit einer Datei

Eine Datei konnte nicht gelesen werden, oder eine an `use` gegebene Datei wurde
nicht gefunden, oder Dateien verwenden sich gegenseitig im Kreis. Pfade in `use`
sind relativ zum Ordner der Datei, die das `use` enthält. Programme sind
Textdateien, in UTF-8 gespeichert (die übliche Wahl der Editoren).

`input()` hält ebenfalls mit diesem Fehler an, wenn es nichts mehr zu lesen gibt:
die Tastatureingabe wurde geschlossen (Strg-D), oder der an das Programm
geschickte Text (`rythme prog.ry < antworten.txt`) hat keine Zeilen mehr."#,
        ),
        (
            r#"E018 — Unexpected character

This character has no meaning in Rythme here. Texts use straight double
quotes "...", powers use **, and comments start with #."#,
            r#"E018 — Unerwartetes Zeichen

Dieses Zeichen hat hier in Rythme keine Bedeutung. Texte verwenden gerade
doppelte Anführungszeichen "...", Potenzen **, und Kommentare beginnen mit #."#,
        ),
        (
            r#"E019 — if without else

In Rythme, `if` is an expression: it always gives a value, so both cases are
needed. Each branch holds a single expression; use `do ... end` for several.

Wrong:   if x > 0 then print("positive")
Right:   if x > 0 then print("positive") else ()
Right:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
            r#"E019 — if ohne else

In Rythme ist `if` ein Ausdruck: es liefert immer einen Wert, also werden beide
Fälle gebraucht. Jeder Zweig enthält einen einzigen Ausdruck; verwende
`do ... end` für mehrere.

Falsch:  if x > 0 then print("positive")
Richtig: if x > 0 then print("positive") else ()
Richtig: if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
        ),
        (
            r#"E020 — Missing end

A function written on several lines, a `match` or a `do` block must finish
with `end` on its own line.

  fn greet(name)
    print("Hello {name}")
  end"#,
            r#"E020 — Fehlendes end

Eine Funktion über mehrere Zeilen, ein `match` oder ein `do`-Block muss mit
`end` in einer eigenen Zeile enden.

  fn greet(name)
    print("Hello {name}")
  end"#,
        ),
        (
            r#"E021 — Program stopped by fail

The program called `fail(message)`, which stops everything. For errors that
the caller could handle, return Err(message) instead."#,
            r#"E021 — Programm durch fail angehalten

Das Programm hat `fail(message)` aufgerufen, was alles anhält. Für Fehler, die
der Aufrufer behandeln könnte, gib stattdessen Err(message) zurück."#,
        ),
        (
            r#"E022 — Impossible conversion

The text does not contain a number. When the text comes from a user or a
file, use the safe versions, which give a Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
            r#"E022 — Unmögliche Umwandlung

Der Text enthält keine Zahl. Wenn der Text von einem Benutzer oder aus einer
Datei kommt, verwende die sicheren Versionen, die ein Result liefern:

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
            r#"E023 — Ungültiger Wert

Die Funktion hat einen Wert der richtigen Art bekommen, aber außerhalb dessen,
was sie akzeptiert: eine negative Anzahl, das Minimum einer leeren Liste, die
Quadratwurzel einer negativen Zahl, ein Datum, das es nicht gibt (30. Februar),
ein Bereich, eine Liste oder ein Text, die zu groß sind, eine ganze Zahl mit
mehr als 100000 Ziffern... Die Meldung erklärt die Grenze."#,
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
            r#"E024 — Ein Wert geht verloren

Eine Zeile berechnet einen Wert, aber nichts verwendet ihn: nur die letzte Zeile
eines Blocks liefert ihren Wert, und Werte ändern sich nie, also geht das
Ergebnis einfach verloren.

Falsch:  total = price
         - discount              # eine neue Zeile: das ist eine eigene Anweisung
Richtig: total = price -
           discount              # beende die Zeile mit dem Operator, um fortzufahren
Falsch:  x + 1                   # ändert x nicht
Richtig: y = x + 1
Um einen Wert anzuzeigen, gib ihn aus:  print(x + 1)"#,
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
            r#"E025 — Problem mit dem Projekt oder einem Paket

Die `rythme.toml` des Projekts (oder die von `rythme install` geschriebene
`rythme.lock`) kann nicht gelesen werden, eine Abhängigkeit ist nicht
installiert, oder `rythme install` kann eine nicht herunterladen. Eine
`rythme.toml` sieht so aus:

  name = "hello"                 # ein Rythme-Name in Kleinbuchstaben
  version = "0.1.0"              # drei Zahlen
  entry = "src/main.ry"          # das Programm (standardmäßig dieses)

  [dependencies]
  mathlib = { git = "https://github.com/someone/mathlib", tag = "v1.0.0" }
  tools = { path = "../tools" }

Jeder Wert ist ein Text in Anführungszeichen. Eine Abhängigkeit aus git braucht
eine feste Version, ein `tag` oder einen `commit`, und das Programm `git`
(https://git-scm.com). Nach einer Änderung von `[dependencies]` führe
`rythme install` im Ordner des Projekts aus. Mehr: `rythme help projects`."#,
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
            r#"E026 — Ungültiger regulärer Ausdruck

Ein regulärer Ausdruck (Regex, übergeben an `matches`, `find_all`, `split_by`,
`replace_all`...) verletzt eine seiner Regeln, oder die Ersetzung von
`replace_all` verwendet eine Gruppe, die es nicht gibt. Die Meldung zeigt, wo.

Falsch:  find_all(text, "([0-9]+")       # die `(` wird nie geschlossen
Richtig: find_all(text, "([0-9]+)")
Falsch:  split_by(text, "*")             # `*` wiederholt nichts
Richtig: split_by(text, "[*]")           # das Zeichen * selbst, in eckigen Klammern
Falsch:  replace_all(date, "(\\d+)-(\\d+)", "$3")    # nur 2 Gruppen
Richtig: replace_all(date, "(\\d+)-(\\d+)", "$2/$1")

In einem Rythme-Text wird ein Backslash doppelt geschrieben ("\\d"), und
geschweifte Klammern werden \{ und \} geschrieben ("[0-9]\{4\}"). `rythme help regex`
zeigt alles, was ein regulärer Ausdruck sagen kann. Wenn der Ausdruck im Code
steht, findet `rythme check` den Fehler vor dem Ausführen."#,
        ),
        (
            r#"W001 — Unused name

A name is defined but never used. It is often a typo or a forgotten step.
If it is on purpose, start the name with `_`: `_unused = ...`."#,
            r#"W001 — Unbenutzter Name

Ein Name wird definiert, aber nie verwendet. Oft ist es ein Tippfehler oder ein
vergessener Schritt. Wenn es Absicht ist, beginne den Namen mit `_`: `_unused = ...`."#,
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
            r#"W002 — Vergessener Fall

Ein `match` (oder eine Funktion mit mehreren Definitionen) behandelt nicht jeden
möglichen Wert. Die Meldung listet auf, was fehlt. Wenn dieser Wert kommt, hält
das Programm mit E006 an.

  type Light = Red | Orange | Green
  fn next(Red) = Green
  fn next(Green) = Orange
  # W002: `next` behandelt `Orange` nicht. Füge die fehlende Definition hinzu:
  fn next(Orange) = Red"#,
        ),
        (
            r#"W003 — Built-in function hidden

You defined a name that already belongs to a built-in function. In your file,
the name now means your definition. Choose another name to keep both."#,
            r#"W003 — Eingebaute Funktion verdeckt

Du hast einen Namen definiert, der schon zu einer eingebauten Funktion gehört.
In deiner Datei bedeutet der Name jetzt deine Definition. Wähle einen anderen
Namen, um beide zu behalten."#,
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
            r#"W004 — Fall nie erreicht

Fälle werden der Reihe nach ausprobiert, und ein Wert geht in den ersten, der
passt. Ein Fall (oder eine Definition einer Funktion) kann nie verwendet werden,
wenn:
  - ein Fall davor alles akzeptiert (`_`, oder ein einfacher Name): setze jenen
    ans Ende;
  - ein Fall davor schon dieselben Werte behandelt: zweimal `1 -> ...`;
  - der Wert nie diese Form haben kann: ein Text passt nie zu einem
    Listenmuster wie `[first, ..rest]`, eine Zahl nie zu `Some(x)`;
  - ein Bereich leer ist: `9..1` enthält keine Zahl, schreib `1..9`.

  match name                     # `name` ist ein Text
    [first, ..] -> first         # W004: ein Text ist keine Liste
    _ -> "?"
  end
Um die Zeichen eines Textes anzuschauen, verwende `chars(name)`."#,
        ),
        (
            r#"W005 — Unknown function after a dot

`value.name(args)` calls the function `name(value, args)`, or the field `name`
of a record. No function has this name, and no record of the file has such a
field: it is probably a typo.

Wrong:   [1, 2].lenght()
Right:   [1, 2].len()"#,
            r#"W005 — Unbekannte Funktion nach einem Punkt

`value.name(args)` ruft die Funktion `name(value, args)` auf, oder das Feld
`name` eines Records. Keine Funktion hat diesen Namen, und kein Record der Datei
hat ein solches Feld: wahrscheinlich ist es ein Tippfehler.

Falsch:  [1, 2].lenght()
Richtig: [1, 2].len()"#,
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
            r#"W006 — Vergleich immer falsch

Werte verschiedener Arten sind nie gleich: Rythme wandelt sie nie stillschweigend
um. Wenn `x` eine Zahl ist, ist `x == "1"` immer falsch und `x != "1"` immer
wahr. Oft ist es eine vergessene Umwandlung, oder ein Wert in einer Option, der
verglichen wird, als stünde er außerhalb.

Falsch:  if count == "3" then ...            (eine Zahl und ein Text)
Richtig: if count == 3 then ...
Falsch:  get(scores, "ada") == 19            (eine Option und eine Zahl)
Richtig: get(scores, "ada") == Some(19)
Richtig: unwrap_or(get(scores, "ada"), 0) == 19"#,
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
            r#"W007 — Test, der nie fehlschlagen kann

Ein Test schlägt nur fehl, wenn er mit einem Fehler endet oder `false`, `None`
oder `Err(...)` ergibt. Ein Test, der nur mit Funktionen, die nie anhalten,
einen Wert berechnet oder ausgibt und ihn nie mit dem Erwarteten vergleicht,
kann nie fehlschlagen: er prüft nichts.

Falsch:  test "length" = len([1, 2])
Falsch:  test "length" = print(len([1, 2]))
Richtig: test "length" = assert_eq(len([1, 2]), 2)
Richtig: test "length" = len([1, 2]) == 2"#,
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
            r#"W008 — Ergebnis geht verloren

Werte ändern sich in Rythme nie: `push(list, 4)` fügt `list` nicht 4 hinzu, es
liefert eine neue Liste. Eine Zeile, die nur eine solche Funktion aufruft und
nichts mit ihrem Ergebnis macht, tut überhaupt nichts.

Falsch:  push(numbers, 4)               # `numbers` bleibt unverändert
Richtig: more = push(numbers, 4)
Falsch:  name.upper()
Richtig: print(name.upper())"#,
        ),
        (
            r#"W009 — A function shown instead of its value

A function shown in a text or printed appears as `<fn name>`. To show what
it gives, call it, with parentheses, even when it takes no argument.

  fn greeting() = "hello"
Wrong:   print("{greeting}")          # shows <fn greeting>
Right:   print("{greeting()}")        # shows hello"#,
            r#"W009 — Eine Funktion statt ihres Werts angezeigt

Eine Funktion, die in einem Text angezeigt oder ausgegeben wird, erscheint als
`<fn name>`. Um zu zeigen, was sie liefert, rufe sie auf, mit Klammern, auch
wenn sie kein Argument nimmt.

  fn greeting() = "hello"
Falsch:  print("{greeting}")          # zeigt <fn greeting>
Richtig: print("{greeting()}")        # zeigt hello"#,
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
            r#"W010 — Bereiche, die sich überschneiden

Ein Bereichsmuster `a..b` passt auf die Zahlen von `a` bis `b`, beide inklusive.
Wenn zwei Bereiche eines `match` (oder der Definitionen einer Funktion) Zahlen
gemeinsam haben, gehen diese Zahlen immer in den ersten: der zweite sieht sie
nie. Oft ist es ein Ende, das doppelt geschrieben wurde.

Falsch:  match age
           0..18 -> "young"
           18..65 -> "adult"         # W010: 18 geht zu "young"
           _ -> "senior"
         end
Richtig: 0..17 -> "young"
         18..64 -> "adult"
Für Dezimalzahlen, die zwischen 17 und 18 liegen, vergleiche stattdessen:
  a if a < 18 -> "young""#,
        ),
        ("Defines a function: `fn name(x) = expression`, or with its body on the following lines, finished by `end`. Without a name, `fn(x) = x * 2` is an anonymous function.", "Definiert eine Funktion: `fn name(x) = expression`, oder mit ihrem Rumpf in den folgenden Zeilen, abgeschlossen mit `end`. Ohne Namen ist `fn(x) = x * 2` eine anonyme Funktion."),
        ("Chooses between two values: `if condition then value else other`. Every `if` has an `else`, because an `if` always gives a value.", "Wählt zwischen zwei Werten: `if condition then value else other`. Jedes `if` hat ein `else`, weil ein `if` immer einen Wert liefert."),
        ("Separates the condition of an `if` from the value it gives when the condition is true.", "Trennt die Bedingung eines `if` von dem Wert, den es liefert, wenn die Bedingung wahr ist."),
        ("Gives the value of an `if` when the condition is false. It is always required; `else if` chains several conditions.", "Gibt den Wert eines `if` an, wenn die Bedingung falsch ist. Es ist immer nötig; `else if` verkettet mehrere Bedingungen."),
        ("The opposite of `if`: `unless condition then a else b` gives `b` when the condition is true, `a` otherwise. Like `if`, it always has an `else`.", "Das Gegenteil von `if`: `unless condition then a else b` liefert `b`, wenn die Bedingung wahr ist, sonst `a`. Wie `if` hat es immer ein `else`."),
        ("Chooses according to the shape of a value: one case per line, `pattern -> result`, finished by `end`. Cases are tried in order, and `rythme check` warns about forgotten cases.", "Entscheidet nach der Form eines Werts: ein Fall pro Zeile, `pattern -> result`, abgeschlossen mit `end`. Fälle werden der Reihe nach ausprobiert, und `rythme check` warnt vor vergessenen Fällen."),
        ("Closes a block: a function written on several lines, a `match`, a `do` or a `test`.", "Schließt einen Block: eine Funktion über mehrere Zeilen, ein `match`, ein `do` oder ein `test`."),
        ("Groups several steps inside an expression, up to `end`. The last line gives the value. After a call, `do |x| ... end` is a block: a function given as the last argument.", "Fasst mehrere Schritte innerhalb eines Ausdrucks zusammen, bis `end`. Die letzte Zeile liefert den Wert. Nach einem Aufruf ist `do |x| ... end` ein Block: eine Funktion, die als letztes Argument übergeben wird."),
        ("Defines your own type and its variants. Each variant is a constructor, with or without fields.", "Definiert deinen eigenen Typ und seine Varianten. Jede Variante ist ein Konstruktor, mit oder ohne Felder."),
        ("Defines a test, next to the code. `rythme test file.ry` runs the tests of the file.", "Definiert einen Test, direkt neben dem Code. `rythme test datei.ry` führt die Tests der Datei aus."),
        ("Uses another file through its name: `use geometry`, then `geometry.area(2)`. The path is relative to the folder of the file that contains `use`.", "Verwendet eine andere Datei über ihren Namen: `use geometry`, dann `geometry.area(2)`. Der Pfad ist relativ zum Ordner der Datei, die `use` enthält."),
        ("True when both conditions are true. The second one is computed only if the first one is true.", "Wahr, wenn beide Bedingungen wahr sind. Die zweite wird nur berechnet, wenn die erste wahr ist."),
        ("True when at least one of the conditions is true. The second one is computed only if the first one is false.", "Wahr, wenn mindestens eine der Bedingungen wahr ist. Die zweite wird nur berechnet, wenn die erste falsch ist."),
        ("Inverts a condition: `not true` is `false`.", "Kehrt eine Bedingung um: `not true` ist `false`."),
        ("One of the two values of the type Bool.", "Einer der beiden Werte des Typs Bool."),
        ("Shows values on the screen, separated by spaces, then goes to the next line.", "Zeigt Werte auf dem Bildschirm an, durch Leerzeichen getrennt, und geht dann in die nächste Zeile."),
        ("Shows a value with its position in the code (on the error output) and returns it unchanged. Handy in the middle of a pipeline.", "Zeigt einen Wert mit seiner Position im Code an (auf der Fehlerausgabe) und gibt ihn unverändert zurück. Praktisch mitten in einer Pipeline."),
        ("Converts any value to text. With a number of decimals, writes a number with exactly that many digits after the point.", "Wandelt einen beliebigen Wert in Text um. Mit einer Anzahl von Nachkommastellen schreibt es eine Zahl mit genau so vielen Ziffern nach dem Punkt."),
        ("Shows the prompt, then reads a line typed on the keyboard and returns it as text.", "Zeigt die Eingabeaufforderung, liest dann eine auf der Tastatur getippte Zeile und gibt sie als Text zurück."),
        ("Reads all the lines of the standard input, until its end: `Ok(list of texts)`, or `Err(message)` if it cannot be read. For `rythme count.ry < names.txt`, or text typed until Ctrl-D (Ctrl-Z then Enter on Windows).", "Liest alle Zeilen der Standardeingabe bis zu ihrem Ende: `Ok(list of texts)`, oder `Err(message)`, wenn sie nicht gelesen werden kann. Für `rythme count.ry < names.txt`, oder Text, der bis Strg-D getippt wird (Strg-Z und dann Enter unter Windows)."),
        ("Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (for `()`), \"Symbol\" (for `:ok`), \"Option\", \"Result\", or the name of your own type.", "Gibt den Namen des Typs eines Werts zurück: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (für `()`), \"Symbol\" (für `:ok`), \"Option\", \"Result\", oder den Namen deines eigenen Typs."),
        ("Shows help: about a function (`help(map)`), a topic (`help(\"lists\")`), or what you can do with a value (`help([1, 2])`).", "Zeigt Hilfe an: zu einer Funktion (`help(map)`), einem Thema (`help(\"lists\")`), oder dazu, was man mit einem Wert machen kann (`help([1, 2])`)."),
        ("Stops with an error if the condition is false. Mostly used in tests.", "Hält mit einem Fehler an, wenn die Bedingung falsch ist. Meistens in Tests verwendet."),
        ("Stops with an error showing both values if they are different. Mostly used in tests.", "Hält mit einem Fehler an, der beide Werte zeigt, wenn sie verschieden sind. Meistens in Tests verwendet."),
        ("Stops the program with an error message. Prefer returning `Err(...)` for errors that can be handled.", "Hält das Programm mit einer Fehlermeldung an. Für Fehler, die behandelt werden können, gib besser `Err(...)` zurück."),
        ("Returns the list of the words given after the file name on the command line.", "Gibt die Liste der Wörter zurück, die auf der Kommandozeile nach dem Dateinamen stehen."),
        ("Reads the options of the command line (the words of `args()`, or the list given) into a record, in a Result: `--name=value` gives the text `value`, `--name` and `-n` give `true`, a `-` in a name becomes `_`, and the other words are in the field `args` (all of them after `--`). An option given twice, or a word like `-abc`, gives `Err(message)`. Use `get(opts, \"name\")` or `has(opts, \"name\")` for an option that may be missing.", "Liest die Optionen der Kommandozeile (die Wörter von `args()`, oder die übergebene Liste) in einen Record, in einem Result: `--name=value` ergibt den Text `value`, `--name` und `-n` ergeben `true`, ein `-` in einem Namen wird zu `_`, und die anderen Wörter stehen im Feld `args` (alle nach `--`). Eine doppelt angegebene Option, oder ein Wort wie `-abc`, ergibt `Err(message)`. Verwende `get(opts, \"name\")` oder `has(opts, \"name\")` für eine Option, die fehlen kann."),
        ("The value of an environment variable of the system, as an Option: `Some(text)`, or `None` if it is not set.", "Der Wert einer Umgebungsvariable des Systems, als Option: `Some(text)`, oder `None`, wenn sie nicht gesetzt ist."),
        ("Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not; Rythme itself exits with 1 after an error in the program (see `rythme help system`).", "Hält das Programm sofort an. Der Code (standardmäßig 0) sagt dem System, ob alles gut gegangen ist (0) oder nicht; Rythme selbst endet nach einem Fehler im Programm mit 1 (siehe `rythme help system`)."),
        ("Returns the current time in seconds (a Float), counted from 1 January 1970. Subtract two times to measure a duration.", "Gibt die aktuelle Zeit in Sekunden zurück (ein Float), gezählt ab dem 1. Januar 1970. Ziehe zwei Zeiten voneinander ab, um eine Dauer zu messen."),
        ("random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.", "random() liefert eine Dezimalzahl zwischen 0 und 1; random(a, b) eine ganze Zahl von a bis b (inklusive); random(list) ein zufälliges Element."),
        ("Absolute value: the number without its sign.", "Betrag: die Zahl ohne ihr Vorzeichen."),
        ("The smallest of the values, or of the elements of a list.", "Der kleinste der Werte, oder der Elemente einer Liste."),
        ("The largest of the values, or of the elements of a list.", "Der größte der Werte, oder der Elemente einer Liste."),
        ("Square root.", "Quadratwurzel."),
        ("x to the power n (same as `x ** n`).", "x hoch n (dasselbe wie `x ** n`)."),
        ("Rounds down to a whole number.", "Rundet auf eine ganze Zahl ab."),
        ("Rounds up to a whole number.", "Rundet auf eine ganze Zahl auf."),
        ("Rounds to the nearest whole number, or to the given number of decimal digits.", "Rundet auf die nächste ganze Zahl, oder auf die angegebene Anzahl von Nachkommastellen."),
        ("Converts to a whole number (Int): decimals are cut, texts are read.", "Wandelt in eine ganze Zahl (Int) um: Nachkommastellen werden abgeschnitten, Texte werden gelesen."),
        ("Converts to a decimal number (Float).", "Wandelt in eine Dezimalzahl (Float) um."),
        ("Sine (x in radians).", "Sinus (x im Bogenmaß)."),
        ("Cosine (x in radians).", "Kosinus (x im Bogenmaß)."),
        ("Tangent (x in radians).", "Tangens (x im Bogenmaß)."),
        ("Arc sine: the angle (in radians) whose sine is x, for x from -1 to 1.", "Arkussinus: der Winkel (im Bogenmaß), dessen Sinus x ist, für x von -1 bis 1."),
        ("Arc cosine: the angle (in radians) whose cosine is x, for x from -1 to 1.", "Arkuskosinus: der Winkel (im Bogenmaß), dessen Kosinus x ist, für x von -1 bis 1."),
        ("Arc tangent: the angle (in radians) whose tangent is y. With two numbers, the angle of the point (x, y), from -pi to pi.", "Arkustangens: der Winkel (im Bogenmaß), dessen Tangens y ist. Mit zwei Zahlen der Winkel des Punkts (x, y), von -pi bis pi."),
        ("Logarithm: natural (base e) by default, or in the given base.", "Logarithmus: standardmäßig der natürliche (Basis e), oder zur angegebenen Basis."),
        ("e to the power x.", "e hoch x."),
        ("Is the whole number even?", "Ist die ganze Zahl gerade?"),
        ("Is the whole number odd?", "Ist die ganze Zahl ungerade?"),
        ("Greatest common divisor of two whole numbers: the largest number that divides both.", "Größter gemeinsamer Teiler zweier ganzer Zahlen: die größte Zahl, die beide teilt."),
        ("Least common multiple of two whole numbers: the smallest number divisible by both.", "Kleinstes gemeinsames Vielfaches zweier ganzer Zahlen: die kleinste Zahl, die durch beide teilbar ist."),
        ("Length of a text (in characters), a list, a record, a dict or a set.", "Länge eines Textes (in Zeichen), einer Liste, eines Records, eines Dicts oder einer Menge."),
        ("The text in uppercase.", "Der Text in Großbuchstaben."),
        ("The text in lowercase.", "Der Text in Kleinbuchstaben."),
        ("The text with its first character in uppercase (the rest is unchanged).", "Der Text mit seinem ersten Zeichen in Großschreibung (der Rest bleibt unverändert)."),
        ("The text without spaces and line breaks at the start and the end.", "Der Text ohne Leerzeichen und Zeilenumbrüche am Anfang und am Ende."),
        ("Adds `fill` (a space by default) before the text until it is `width` characters long: numbers line up in columns. A value that is not a text is first shown as `print` shows it.", "Fügt `fill` (standardmäßig ein Leerzeichen) vor dem Text hinzu, bis er `width` Zeichen lang ist: Zahlen stehen in Spalten untereinander. Ein Wert, der kein Text ist, wird zuerst so dargestellt, wie `print` ihn zeigt."),
        ("Adds `fill` (a space by default) after the text until it is `width` characters long.", "Fügt `fill` (standardmäßig ein Leerzeichen) nach dem Text hinzu, bis er `width` Zeichen lang ist."),
        ("Cuts a text into a list of texts. Without separator, cuts at spaces.", "Zerlegt einen Text in eine Liste von Texten. Ohne Trennzeichen wird an Leerzeichen zerlegt."),
        ("Glues the elements of a list into one text, with the separator between them.", "Klebt die Elemente einer Liste zu einem Text zusammen, mit dem Trennzeichen dazwischen."),
        ("Does the text contain this piece of text? Does the list (or the set) contain this element? Does the record have this field, or the dict this key?", "Enthält der Text dieses Textstück? Enthält die Liste (oder die Menge) dieses Element? Hat der Record dieses Feld, oder das Dict diesen Schlüssel?"),
        ("Does the text start with this piece of text?", "Beginnt der Text mit diesem Textstück?"),
        ("Does the text end with this piece of text?", "Endet der Text mit diesem Textstück?"),
        ("Replaces every occurrence of `old` by `new`.", "Ersetzt jedes Vorkommen von `old` durch `new`."),
        ("The list of the characters of a text.", "Die Liste der Zeichen eines Textes."),
        ("The list of the lines of a text.", "Die Liste der Zeilen eines Textes."),
        ("The number of a character (its Unicode code point).", "Die Nummer eines Zeichens (sein Unicode-Codepunkt)."),
        ("The character with this number (Unicode code point): the opposite of `char_code`.", "Das Zeichen mit dieser Nummer (Unicode-Codepunkt): das Gegenteil von `char_code`."),
        ("Is the text made only of digits from 0 to 9 (and not empty)?", "Besteht der Text nur aus Ziffern von 0 bis 9 (und ist nicht leer)?"),
        ("Is the text made only of letters, accented ones included (and not empty)?", "Besteht der Text nur aus Buchstaben, auch solchen mit Akzent (und ist nicht leer)?"),
        ("Is the text made only of spaces, tabs and line breaks (and not empty)?", "Besteht der Text nur aus Leerzeichen, Tabulatoren und Zeilenumbrüchen (und ist nicht leer)?"),
        ("A text or a list repeated n times.", "Ein Text oder eine Liste, n-mal wiederholt."),
        ("Reads a whole number from a text: `Ok(number)`, or `Err(message)` if the text is not a number.", "Liest eine ganze Zahl aus einem Text: `Ok(number)`, oder `Err(message)`, wenn der Text keine Zahl ist."),
        ("Reads a decimal number from a text: `Ok(number)` or `Err(message)`.", "Liest eine Dezimalzahl aus einem Text: `Ok(number)` oder `Err(message)`."),
        ("Reads a JSON text: `Ok(value)`, or `Err(message)` if the text is not valid JSON. Objects become records, arrays lists, and null `None`.", "Liest einen JSON-Text: `Ok(value)`, oder `Err(message)`, wenn der Text kein gültiges JSON ist. Objekte werden Records, Arrays Listen, und null wird `None`."),
        ("Writes a value as JSON text: records become objects, lists arrays, and `None` null. With `indent`, one element per line, shifted by that many spaces.", "Schreibt einen Wert als JSON-Text: Records werden Objekte, Listen Arrays, und `None` wird null. Mit `indent` ein Element pro Zeile, um so viele Leerzeichen eingerückt."),
        ("The text without spaces and line breaks at the start.", "Der Text ohne Leerzeichen und Zeilenumbrüche am Anfang."),
        ("The text without spaces and line breaks at the end.", "Der Text ohne Leerzeichen und Zeilenumbrüche am Ende."),
        ("The list of the words of a text: letters and digits, accents included, with `'` or `-` inside a word. Punctuation and spaces separate them.", "Die Liste der Wörter eines Textes: Buchstaben und Ziffern, Akzente inklusive, mit `'` oder `-` innerhalb eines Wortes. Satzzeichen und Leerzeichen trennen sie."),
        ("Are the two texts the same when uppercase and lowercase letters count as equal?", "Sind die beiden Texte gleich, wenn Groß- und Kleinbuchstaben als gleich gelten?"),
        ("Writes a number for people: thousands separated, with exactly `decimals` decimals if given, in the style of a language: \"en\" (1,234.5, by default), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" or \"pt\" (1.234,5). To line it up in a column, use `pad_left`.", "Schreibt eine Zahl für Menschen: Tausender getrennt, mit genau `decimals` Nachkommastellen, falls angegeben, im Stil einer Sprache: \"en\" (1,234.5, standardmäßig), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" oder \"pt\" (1.234,5). Um sie in einer Spalte auszurichten, verwende `pad_left`."),
        ("Does the whole text match the regex (a regular expression: see `rythme help regex`)? To look for the regex anywhere in the text, use `find_first`.", "Passt der ganze Text auf den regulären Ausdruck (siehe `rythme help regex`)? Um den Ausdruck irgendwo im Text zu suchen, verwende `find_first`."),
        ("The first piece of the text that matches the regex, as an Option: `Some(piece)`, or `None` if there is none.", "Das erste Stück des Textes, das auf den regulären Ausdruck passt, als Option: `Some(piece)`, oder `None`, wenn es keines gibt."),
        ("Every piece of the text that matches the regex, from left to right.", "Jedes Stück des Textes, das auf den regulären Ausdruck passt, von links nach rechts."),
        ("The first match of the regex, as an Option of a list: the whole piece, then the text of each group `(...)`, in order (\"\" for a group that matched nothing).", "Der erste Treffer des regulären Ausdrucks, als Option einer Liste: das ganze Stück, dann der Text jeder Gruppe `(...)`, der Reihe nach (\"\" für eine Gruppe, die nichts getroffen hat)."),
        ("Like `captures`, for every match: a list of lists.", "Wie `captures`, für jeden Treffer: eine Liste von Listen."),
        ("The first match of a regex with named groups `(?<name>...)`, as an Option of a record: one field for each name.", "Der erste Treffer eines regulären Ausdrucks mit benannten Gruppen `(?<name>...)`, als Option eines Records: ein Feld für jeden Namen."),
        ("Cuts a text at every piece that matches the regex.", "Zerlegt einen Text an jedem Stück, das auf den regulären Ausdruck passt."),
        ("Replaces every piece that matches the regex. `new` is a text, where `$1` stands for the text of group 1 (`$<name>` for a named group, `$0` for the whole piece, `$$` for a `$`), or a function that receives each piece and gives its replacement.", "Ersetzt jedes Stück, das auf den regulären Ausdruck passt. `new` ist ein Text, in dem `$1` für den Text der Gruppe 1 steht (`$<name>` für eine benannte Gruppe, `$0` für das ganze Stück, `$$` für ein `$`), oder eine Funktion, die jedes Stück bekommt und seine Ersetzung liefert."),
        ("Applies the function f to each element and returns the list of the results. Also works on Option and Result, and on a set (giving the set of the results).", "Wendet die Funktion f auf jedes Element an und gibt die Liste der Ergebnisse zurück. Funktioniert auch mit Option und Result, und mit einer Menge (ergibt die Menge der Ergebnisse)."),
        ("Keeps only the elements for which f returns true. Also works on a set, and on a dict, where f receives each key and its value.", "Behält nur die Elemente, für die f true zurückgibt. Funktioniert auch mit einer Menge, und mit einem Dict, wobei f jeden Schlüssel und seinen Wert bekommt."),
        ("Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.", "Fasst alle Elemente zu einem Wert zusammen: beginnt mit `start` und berechnet dann f(result, element) für jedes Element."),
        ("Like fold, but gives the list of every intermediate result: running totals, for example.", "Wie fold, aber liefert die Liste aller Zwischenergebnisse: zum Beispiel laufende Summen."),
        ("Calls f on each element, for its effect (like printing). Returns (). Also works on a set, and on a dict, where f receives each key and its value.", "Ruft f für jedes Element auf, wegen seiner Wirkung (wie etwas auszugeben). Gibt () zurück. Funktioniert auch mit einer Menge, und mit einem Dict, wobei f jeden Schlüssel und seinen Wert bekommt."),
        ("The sum of the numbers of a list; with f, the sum of f(x) for each element.", "Die Summe der Zahlen einer Liste; mit f die Summe von f(x) für jedes Element."),
        ("The product of the numbers of a list.", "Das Produkt der Zahlen einer Liste."),
        ("The first element of a list (or character of a text), as an Option: `Some(x)`, or `None` if there is none. With n, the first n elements (like `take`).", "Das erste Element einer Liste (oder Zeichen eines Textes), als Option: `Some(x)`, oder `None`, wenn es keines gibt. Mit n die ersten n Elemente (wie `take`)."),
        ("The last element of a list (or character of a text), as an Option. With n, the last n elements.", "Das letzte Element einer Liste (oder Zeichen eines Textes), als Option. Mit n die letzten n Elemente."),
        ("The list (or text) without its first element.", "Die Liste (oder der Text) ohne ihr erstes Element."),
        ("The element at a position of a list or text, the field of a record, or the value of a key of a dict, as an Option (never an error). Negative positions count from the end.", "Das Element an einer Position einer Liste oder eines Textes, das Feld eines Records, oder der Wert eines Schlüssels eines Dicts, als Option (nie ein Fehler). Negative Positionen zählen vom Ende her."),
        ("A new list with x added at the end; or a new set with x added.", "Eine neue Liste mit x am Ende angefügt; oder eine neue Menge mit x hinzugefügt."),
        ("A new list where the element at position i is replaced by x. Negative positions count from the end.", "Eine neue Liste, in der das Element an Position i durch x ersetzt ist. Negative Positionen zählen vom Ende her."),
        ("A new list with x inserted so that it is at position i (-1 adds it at the end).", "Eine neue Liste, in die x so eingefügt ist, dass es an Position i steht (-1 fügt es am Ende an)."),
        ("A new list without the element at position i. Negative positions count from the end.", "Eine neue Liste ohne das Element an Position i. Negative Positionen zählen vom Ende her."),
        ("The list or text in reverse order.", "Die Liste oder der Text in umgekehrter Reihenfolge."),
        ("The list sorted from smallest to largest (numbers, texts in alphabetical order...). For the largest first: `sort(list) |> reverse`.", "Die Liste vom kleinsten zum größten sortiert (Zahlen, Texte alphabetisch...). Für das Größte zuerst: `sort(list) |> reverse`."),
        ("Sorts the list by the value that the function `key` gives for each element. Elements with the same value keep their order.", "Sortiert die Liste nach dem Wert, den die Funktion `key` für jedes Element liefert. Elemente mit demselben Wert behalten ihre Reihenfolge."),
        ("The list without duplicates (the first occurrence is kept).", "Die Liste ohne Duplikate (das erste Vorkommen bleibt)."),
        ("The elements of the list in a random order.", "Die Elemente der Liste in zufälliger Reihenfolge."),
        ("The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.", "Der Teil einer Liste oder eines Textes von Position `start` (inklusive) bis `end` (exklusive). Negative Positionen zählen vom Ende her."),
        ("The first n elements of a list or text.", "Die ersten n Elemente einer Liste oder eines Textes."),
        ("The list or text without its first n elements.", "Die Liste oder der Text ohne die ersten n Elemente."),
        ("The first elements of a list (or characters of a text), as long as f is true for them.", "Die ersten Elemente einer Liste (oder Zeichen eines Textes), solange f für sie wahr ist."),
        ("The list (or text) without its first elements for which f is true.", "Die Liste (oder der Text) ohne ihre ersten Elemente, für die f wahr ist."),
        ("One element out of n of a list (or text), starting with the first. On a range: the numbers with a step.", "Ein Element von n einer Liste (oder eines Textes), beginnend mit dem ersten. Bei einem Bereich: die Zahlen mit einer Schrittweite."),
        ("Cuts a list (or text) into pieces of n elements; the last piece may be shorter.", "Zerlegt eine Liste (oder einen Text) in Stücke von n Elementen; das letzte Stück kann kürzer sein."),
        ("Every group of n neighbouring elements of a list (or text), in order: handy to compare each element with the next one.", "Jede Gruppe von n benachbarten Elementen einer Liste (oder eines Textes), der Reihe nach: praktisch, um jedes Element mit dem nächsten zu vergleichen."),
        ("Pairs the elements of two lists: [[a1, b1], [a2, b2], ...]. It stops at the end of the shorter list.", "Bildet Paare aus den Elementen zweier Listen: [[a1, b1], [a2, b2], ...]. Hört am Ende der kürzeren Liste auf."),
        ("Pairs each element with its position: [[0, x0], [1, x1], ...].", "Paart jedes Element mit seiner Position: [[0, x0], [1, x1], ...]."),
        ("Is f true for at least one element?", "Ist f für mindestens ein Element wahr?"),
        ("Is f true for every element?", "Ist f für jedes Element wahr?"),
        ("The first element for which f is true, as an Option.", "Das erste Element, für das f wahr ist, als Option."),
        ("How many elements make f true; without f, how many elements there are.", "Wie viele Elemente f wahr machen; ohne f, wie viele Elemente es gibt."),
        ("The elements for which f is false: the opposite of `filter`. Also works on a set and on a dict, like `filter`.", "Die Elemente, für die f falsch ist: das Gegenteil von `filter`. Funktioniert auch mit einer Menge und einem Dict, wie `filter`."),
        ("Calls f(element, position) for each element, in order (positions start at 0). For effects, like `print`.", "Ruft f(element, position) für jedes Element der Reihe nach auf (Positionen beginnen bei 0). Für Wirkungen, wie `print`."),
        ("The position of an element in a list (or of a piece of text in a text), as an Option.", "Die Position eines Elements in einer Liste (oder eines Textstücks in einem Text), als Option."),
        ("The element for which the function `key` gives the smallest value (the first one if there are several).", "Das Element, für das die Funktion `key` den kleinsten Wert liefert (das erste, wenn es mehrere gibt)."),
        ("The element for which the function `key` gives the largest value (the first one if there are several).", "Das Element, für das die Funktion `key` den größten Wert liefert (das erste, wenn es mehrere gibt)."),
        ("Joins a list of lists into one list.", "Verbindet eine Liste von Listen zu einer Liste."),
        ("Like map, when f returns a list: the lists are joined.", "Wie map, wenn f eine Liste zurückgibt: die Listen werden verbunden."),
        ("Splits a list in two: [elements where f is true, elements where f is false].", "Teilt eine Liste in zwei: [Elemente, für die f wahr ist, Elemente, für die f falsch ist]."),
        ("Groups the elements in a record: the key is the text of what `key` returns, the value the list of matching elements.", "Gruppiert die Elemente in einem Record: der Schlüssel ist der Text dessen, was `key` zurückgibt, der Wert die Liste der passenden Elemente."),
        ("Is the list, text, record, dict or set empty?", "Ist die Liste, der Text, der Record, das Dict oder die Menge leer?"),
        ("The list of the field names of a record, or of the keys of a dict (in order).", "Die Liste der Feldnamen eines Records, oder der Schlüssel eines Dicts (der Reihe nach)."),
        ("The list of the values of a record or of a dict (in the order of its keys), or of the elements of a set (in order).", "Die Liste der Werte eines Records oder eines Dicts (in der Reihenfolge seiner Schlüssel), oder der Elemente einer Menge (der Reihe nach)."),
        ("The list of the [name, value] pairs of a record, or of the [key, value] pairs of a dict.", "Die Liste der [Name, Wert]-Paare eines Records, oder der [Schlüssel, Wert]-Paare eines Dicts."),
        ("Builds a record from a list of [name, value] pairs: the opposite of `entries`.", "Baut einen Record aus einer Liste von [Name, Wert]-Paaren: das Gegenteil von `entries`."),
        ("Does the record have this field? Does the dict have this key, or the set this element?", "Hat der Record dieses Feld? Hat das Dict diesen Schlüssel, oder die Menge dieses Element?"),
        ("A new record with the field set to the value (added or replaced); or a new dict with the key set to the value.", "Ein neuer Record, in dem das Feld auf den Wert gesetzt ist (hinzugefügt oder ersetzt); oder ein neues Dict, in dem der Schlüssel auf den Wert gesetzt ist."),
        ("A new record without this field; or a new dict without this key, or a new set without this element.", "Ein neuer Record ohne dieses Feld; oder ein neues Dict ohne diesen Schlüssel, oder eine neue Menge ohne dieses Element."),
        ("A new record (or dict) where f is applied to every value; the names (or keys) stay the same.", "Ein neuer Record (oder ein neues Dict), in dem f auf jeden Wert angewendet ist; die Namen (oder Schlüssel) bleiben gleich."),
        ("A dictionary: keys of any kind (but functions), each with a value, kept in the order of the keys. Made from a list of [key, value] pairs (the last value of a key wins), from a record, or empty with `dict()`. Use it with get, put, has, remove, keys, values, entries, len, map_values, filter, union...", "Ein Wörterbuch: Schlüssel jeder Art (außer Funktionen), jeder mit einem Wert, in der Reihenfolge der Schlüssel gehalten. Aus einer Liste von [Schlüssel, Wert]-Paaren gebaut (der letzte Wert eines Schlüssels gewinnt), aus einem Record, oder leer mit `dict()`. Verwende es mit get, put, has, remove, keys, values, entries, len, map_values, filter, union..."),
        ("A set: values (but functions) without repetition, kept in order. Made from a list, or empty with `set()`. Use it with has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.", "Eine Menge: Werte (außer Funktionen) ohne Wiederholung, der Reihe nach gehalten. Aus einer Liste gebaut, oder leer mit `set()`. Verwende sie mit has, push, remove, len, values, filter, map, union, intersection, difference, is_subset."),
        ("The elements of two sets together; or the pairs of two dicts, where the value of b wins for a key in both.", "Die Elemente zweier Mengen zusammen; oder die Paare zweier Dicts, wobei für einen Schlüssel in beiden der Wert von b gewinnt."),
        ("The elements of the set a that are also in b; or the pairs of the dict a whose key is in b.", "Die Elemente der Menge a, die auch in b sind; oder die Paare des Dicts a, deren Schlüssel in b ist."),
        ("The elements of the set a that are not in b; or the pairs of the dict a whose key is not in b.", "Die Elemente der Menge a, die nicht in b sind; oder die Paare des Dicts a, deren Schlüssel nicht in b ist."),
        ("Is every element of the set a also in the set b?", "Ist jedes Element der Menge a auch in der Menge b?"),
        ("The value inside `Some(x)` or `Ok(x)`. Stops the program with an error for `None` or `Err(e)`.", "Der Wert in `Some(x)` oder `Ok(x)`. Hält das Programm bei `None` oder `Err(e)` mit einem Fehler an."),
        ("The value inside `Some(x)` or `Ok(x)`, or the default value for `None` or `Err(e)`.", "Der Wert in `Some(x)` oder `Ok(x)`, oder der Standardwert bei `None` oder `Err(e)`."),
        ("Is it `Some(...)`?", "Ist es `Some(...)`?"),
        ("Is it `None`?", "Ist es `None`?"),
        ("Is it `Ok(...)`?", "Ist es `Ok(...)`?"),
        ("Is it `Err(...)`?", "Ist es `Err(...)`?"),
        ("Chains steps that may fail: for `Some(x)` or `Ok(x)`, gives f(x), which must be an Option or a Result; `None` and `Err(e)` stay as they are.", "Verkettet Schritte, die scheitern können: für `Some(x)` oder `Ok(x)` liefert es f(x), das eine Option oder ein Result sein muss; `None` und `Err(e)` bleiben, wie sie sind."),
        ("Turns an Option into a Result: `Some(x)` gives `Ok(x)`, and `None` gives `Err(error)`. Handy before `?`.", "Macht aus einer Option ein Result: `Some(x)` ergibt `Ok(x)`, und `None` ergibt `Err(error)`. Praktisch vor `?`."),
        ("For `Err(e)`, gives `Err(f(e))`; `Ok(x)` stays as it is. Handy to explain an error better.", "Für `Err(e)` liefert es `Err(f(e))`; `Ok(x)` bleibt, wie es ist. Praktisch, um einen Fehler besser zu erklären."),
        ("Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.", "Liest eine ganze Textdatei: `Ok(text)`, oder `Err(message)`, wenn sie nicht gelesen werden kann."),
        ("Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.", "Schreibt einen Text in eine Datei (ersetzt ihren Inhalt): `Ok(())` oder `Err(message)`."),
        ("Adds a text at the end of a file, creating the file if needed: `Ok(())` or `Err(message)`.", "Fügt einen Text am Ende einer Datei an und legt die Datei bei Bedarf an: `Ok(())` oder `Err(message)`."),
        ("Does this file (or folder) exist? Always `false` in the browser.", "Gibt es diese Datei (oder diesen Ordner)? Im Browser immer `false`."),
        ("What is known about a file or a folder: `Ok({is_folder, size})`, with its size in bytes (0 for a folder), or `Err(message)` if it does not exist or cannot be read.", "Was über eine Datei oder einen Ordner bekannt ist: `Ok({is_folder, size})`, mit der Größe in Bytes (0 für einen Ordner), oder `Err(message)`, wenn sie nicht existiert oder nicht gelesen werden kann."),
        ("The names of the files and folders inside a folder, in alphabetical order: `Ok(list of texts)`, or `Err(message)`. `list_folder(\".\")` lists the current folder.", "Die Namen der Dateien und Ordner in einem Ordner, alphabetisch sortiert: `Ok(list of texts)`, oder `Err(message)`. `list_folder(\".\")` listet den aktuellen Ordner auf."),
        ("Creates a folder, and the folders around it that are missing: `Ok(())`, also when it already exists, or `Err(message)`.", "Legt einen Ordner an, und die fehlenden Ordner drumherum: `Ok(())`, auch wenn er schon existiert, oder `Err(message)`."),
        ("Deletes one file (never a folder: that is an error): `Ok(())` or `Err(message)`. A deleted file cannot be brought back.", "Löscht eine Datei (nie einen Ordner: das ist ein Fehler): `Ok(())` oder `Err(message)`. Eine gelöschte Datei kann nicht zurückgeholt werden."),
        ("Joins the parts of a path with `/`, which works on Linux, macOS and Windows (with `\\` if the parts already use only `\\`). Only the first part can start at a root (`/`, `C:`): a part like \"/etc\" after another one is an error.", "Verbindet die Teile eines Pfads mit `/`, was unter Linux, macOS und Windows funktioniert (mit `\\`, wenn die Teile schon nur `\\` verwenden). Nur der erste Teil darf an einer Wurzel beginnen (`/`, `C:`): ein Teil wie \"/etc\" nach einem anderen ist ein Fehler."),
        ("The parts of a path, split at each `/` or `\\`; the first part of an absolute path is its root (`/`, `C:\\`...). `join_path` puts them back together.", "Die Teile eines Pfads, an jedem `/` oder `\\` getrennt; der erste Teil eines absoluten Pfads ist seine Wurzel (`/`, `C:\\`...). `join_path` setzt sie wieder zusammen."),
        ("The date and time now, where the program runs: a record {year, month, day, hour, minute, second, offset}, where `offset` is the number of minutes the local time is ahead of UTC (0 when the system does not tell).", "Datum und Uhrzeit jetzt, dort, wo das Programm läuft: ein Record {year, month, day, hour, minute, second, offset}, wobei `offset` die Anzahl der Minuten ist, die die Ortszeit UTC voraus ist (0, wenn das System es nicht sagt)."),
        ("The date of today, where the program runs: a record {year, month, day}.", "Das heutige Datum, dort, wo das Programm läuft: ein Record {year, month, day}."),
        ("Reads a date written as ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", with `Z` or an offset like `+01:00` if needed): `Ok(date)`, or `Err(message)` if it is not a valid date.", "Liest ein im Format ISO 8601 geschriebenes Datum (\"2024-02-29\", \"2024-02-29T13:05:00\", bei Bedarf mit `Z` oder einem Versatz wie `+01:00`): `Ok(date)`, oder `Err(message)`, wenn es kein gültiges Datum ist."),
        ("Writes a date as ISO 8601: \"2024-02-29\", with the time if the date has one, and `Z` or the offset if it has one.", "Schreibt ein Datum im Format ISO 8601: \"2024-02-29\", mit der Uhrzeit, wenn das Datum eine hat, und `Z` oder dem Versatz, wenn es einen hat."),
        ("Is the value a valid date: a record {year, month, day} (with `hour`, `minute`, `second`, `offset` if needed) for a day that exists?", "Ist der Wert ein gültiges Datum: ein Record {year, month, day} (bei Bedarf mit `hour`, `minute`, `second`, `offset`) für einen Tag, den es gibt?"),
        ("The date n days later (or earlier, when n is negative), at the same time of day.", "Das Datum n Tage später (oder früher, wenn n negativ ist), zur selben Tageszeit."),
        ("The date and time n seconds later (or earlier, when n is negative). Add `60 * minutes` or `3600 * hours` the same way.", "Datum und Uhrzeit n Sekunden später (oder früher, wenn n negativ ist). Addiere `60 * minutes` oder `3600 * hours` auf dieselbe Weise."),
        ("The number of whole days from date a to date b: negative when b is before a.", "Die Anzahl ganzer Tage von Datum a bis Datum b: negativ, wenn b vor a liegt."),
        ("The number of seconds from date a to date b, taking their offsets into account: negative when b is before a.", "Die Anzahl der Sekunden von Datum a bis Datum b, mit ihren Versätzen berücksichtigt: negativ, wenn b vor a liegt."),
        ("The day of the week of a date, from 1 (Monday) to 7 (Sunday), as in ISO 8601.", "Der Wochentag eines Datums, von 1 (Montag) bis 7 (Sonntag), wie in ISO 8601."),
        ("The number of days of a month (from 1 to 12) of a year: 28 to 31.", "Die Anzahl der Tage eines Monats (von 1 bis 12) eines Jahres: 28 bis 31."),
        ("Does the year have a 29 February? Every 4 years, except every 100 years, except every 400 years.", "Hat das Jahr einen 29. Februar? Alle 4 Jahre, außer alle 100 Jahre, außer alle 400 Jahre."),
        ("Checks that a property holds for many values made by a generator (100, or `cases`): the function is called with each value and must give `true` (or `()`, `Some(...)`, `Ok(...)`). When a value makes it false or stops it with an error, that value is simplified as much as possible (shrinking) and shown, with the seed that made it: the same values come back on every run. The property may also come last, as a block: `check_all(ints()) { |x| x + 0 == x }`.", "Prüft, dass eine Eigenschaft für viele von einem Generator erzeugte Werte gilt (100, oder `cases`): die Funktion wird mit jedem Wert aufgerufen und muss `true` liefern (oder `()`, `Some(...)`, `Ok(...)`). Wenn ein Wert sie falsch macht oder mit einem Fehler anhält, wird dieser Wert so weit wie möglich vereinfacht (Shrinking) und gezeigt, mit dem Seed, der ihn erzeugt hat: dieselben Werte kommen bei jedem Lauf wieder. Die Eigenschaft kann auch als Block am Ende stehen: `check_all(ints()) { |x| x + 0 == x }`."),
        ("A list of `n` values (10 if not given) made by a generator, to see what `check_all` will try.", "Eine Liste von `n` Werten (10, wenn nicht angegeben), von einem Generator erzeugt, um zu sehen, was `check_all` ausprobieren wird."),
        ("A generator of whole numbers from `min` to `max` (from -1000 to 1000 if not given), often near 0 and at the edges. A failing number is simplified towards 0.", "Ein Generator ganzer Zahlen von `min` bis `max` (von -1000 bis 1000, wenn nicht angegeben), oft nahe 0 und an den Rändern. Eine fehlschlagende Zahl wird in Richtung 0 vereinfacht."),
        ("A generator of decimal numbers from `min` to `max` (from -1000.0 to 1000.0 if not given).", "Ein Generator von Dezimalzahlen von `min` bis `max` (von -1000.0 bis 1000.0, wenn nicht angegeben)."),
        ("A generator of texts of at most `max_length` characters (20 if not given): letters mostly, with digits, spaces, punctuation and accents. A failing text is made shorter and simpler.", "Ein Generator von Texten mit höchstens `max_length` Zeichen (20, wenn nicht angegeben): meistens Buchstaben, mit Ziffern, Leerzeichen, Satzzeichen und Akzenten. Ein fehlschlagender Text wird kürzer und einfacher gemacht."),
        ("A generator of `true` and `false`.", "Ein Generator von `true` und `false`."),
        ("A generator of lists of at most `max_length` elements (10 if not given), each made by `generator`. A failing list is made shorter, then its elements simpler.", "Ein Generator von Listen mit höchstens `max_length` Elementen (10, wenn nicht angegeben), jedes von `generator` erzeugt. Eine fehlschlagende Liste wird kürzer gemacht, dann ihre Elemente einfacher."),
        ("A generator that picks one of the values of a list. A failing value is replaced by one written before it.", "Ein Generator, der einen der Werte einer Liste wählt. Ein fehlschlagender Wert wird durch einen ersetzt, der davor steht."),
        ("A generator of records, with a generator for each field: `records_of({name: texts(), age: ints(0, 120)})`. Handy to give several values to `check_all`.", "Ein Generator von Records, mit einem Generator für jedes Feld: `records_of({name: texts(), age: ints(0, 120)})`. Praktisch, um `check_all` mehrere Werte zu geben."),
        ("January", "Januar"),
        ("February", "Februar"),
        ("March", "März"),
        ("April", "April"),
        ("May", "Mai"),
        ("June", "Juni"),
        ("July", "Juli"),
        ("August", "August"),
        ("September", "September"),
        ("October", "Oktober"),
        ("November", "November"),
        ("December", "Dezember"),
    ],
}
