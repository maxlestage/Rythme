# Insegnare con Rythme

*[English version →](teaching.md) · [Version française →](enseigner.md) · [En español →](teaching.es.md) · [Auf Deutsch →](teaching.de.md) · [Em português →](teaching.pt.md) · [中文 →](teaching.zh.md) · [日本語 →](teaching.ja.md)*

Rythme è stato pensato per la classe: un linguaggio piccolo, con messaggi
che insegnano, perché gli studenti passino il loro tempo sulle idee e non
sulla sintassi. Questa guida copre quello che serve a un corso: un
programma, esercizi controllati e valutati automaticamente, modi di mostrare
come gira il codice, e delle regole per gli assistenti di IA.

## Perché va bene per un corso

- **Poco da imparare prima di scrivere veri programmi.** 15 parole chiave,
  un solo modo di definire una funzione, un solo modo di scegliere
  (`if`/`match`), niente classi, niente cicli, niente mutazioni. Gli
  studenti possono leggere tutta la sintassi in una pagina
  (`rythme help syntax`).
- **L'interprete è un assistente didattico.** Gli errori indicano il punto
  esatto, spiegano il problema e propongono una correzione. Gli errori
  portati da altri linguaggi (`for`, `return`, `x += 1`, `null`...)
  ricevono una spiegazione del modo funzionale. `rythme explain E0xx` dà
  la versione lunga con esempi. `rythme check` trova i problemi senza
  eseguire il programma, comprese le operazioni che fallirebbero sempre,
  come `"età: " + 36`.
- **Pensare in modo funzionale dal primo giorno.** Immutabilità, funzioni
  come valori, ricorsione, pattern matching, tipi algebrici, errori come
  valori (`Option`, `Result`): i concetti di Haskell, OCaml, Elm, Elixir o
  Rust, senza la loro sintassi né un sistema di tipi da combattere il primo
  giorno. I tipi sono facoltativi: il controllo li trova da solo, e quando
  il corso ci arriva, gli studenti li scrivono
  (`fn area(w: Int, h: Int) -> Int`, `type Tree(a)`) e il controllo li
  verifica.
- **I test sono integrati.** `test "..." = assert_eq(actual, expected)`
  accanto al codice, `rythme test file.ry` per eseguirli. Gli esercizi
  possono arrivare con i loro test.
- **Una libreria che gli studenti riconoscono.** Testi, liste, dizionari,
  date, CSV, espressioni regolari, e vettori, statistica e pendenze per
  un'unità di apprendimento automatico (`rythme help ml`). Chi conosce Ruby
  o Python trova molte funzioni con i nomi che conosce (`upcase`, `strip`,
  `each_slice`...): vedi `rythme help ruby` e
  [la libreria a confronto](stdlib-audit.md) (in inglese).
- **Nella lingua dei tuoi studenti**, e **facile da installare, gratuito,
  aperto**: un solo programma, nessuna dipendenza, licenza MIT, per Linux,
  macOS e Windows, o niente da installare con il playground del sito.

## Un possibile corso (12 lezioni)

| # | Tema | Concetti di Rythme | Esercizi | Esempio |
|---|------|--------------------|----------|---------|
| 1 | Espressioni e valori | numeri, testi, `print`, `{...}` nei testi, la REPL | 01, 03 | [`hello.ry`](../examples/hello.ry) |
| 2 | Funzioni | `fn f(x) = ...`, blocchi con `end`, chiamate, `help(f)` | 02 | |
| 3 | Scegliere | `if ... then ... else`, booleani, `and`/`or`/`not` | 04–06 | |
| 4 | Ricorsione | caso base, definizioni per caso `fn fib(0) = 0`, guardie | 07–10 | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Liste | letterali, intervalli `1..10`, pattern `[x, ..rest]` | 11–13 | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Funzioni di ordine superiore | `map`, `filter`, `fold`, funzioni anonime, `\|>` | 14–19 | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Record | `{name: ...}`, accesso ai campi, `{..r, x: 1}` | 20–21 | [`students.ry`](../examples/students.ry) |
| 8 | I tuoi tipi | `type`, costruttori, `match`, casi dimenticati (W002), tipi scritti nel codice | 22–23 | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Tipi ricorsivi | alberi, ricorsione strutturale, tipi con parametri (`type Tree(a)`) | 24–25 | [`tree.ry`](../examples/tree.ry) |
| 10 | Gli errori come valori | `Option`, `Result`, `?`, `unwrap_or` | 26–28 | [`results.ry`](../examples/results.ry) |
| 11 | Test e moduli | `test`, `assert_eq`, `use` | 30 | [`testing.ry`](../examples/testing.ry) |
| 12 | Progetto | elaborazione di testi, analisi di dati, un piccolo gioco | 29 | [`words.ry`](../examples/words.ry) |

La ricorsione in coda (lezione 4 o 9) merita un momento: l'errore E014
("troppe chiamate annidate") la spiega, e `rythme explain E014` mostra la
tecnica dell'accumulatore.

Dopo la lezione 7, due o tre lezioni possono formare un'**unità
sull'intelligenza artificiale**: vettori, statistica, un neurone
artificiale, discesa del gradiente, vicini più prossimi e minimax (esercizi
da 31 a 36, `rythme help ml`, e gli esempi
[`perceptron.ry`](../examples/perceptron.ry) e [`knn.ry`](../examples/knn.ry)).

## Esercizi con correzione automatica

Dai agli studenti un file con i test già scritti, e ogni funzione da
scrivere come `fail("da fare")`. Eccone uno completato da uno studente, con
un errore classico:

```rythme
## Restituisce l'elemento più grande di una lista non vuota.
fn largest(list) = fold(list, 0, max)

test "un elemento" = assert_eq(largest([7]), 7)
test "più elementi" = assert_eq(largest([3, 9, 2]), 9)
test "negativi" = assert_eq(largest([-3, -1, -2]), -1)
```

`rythme test largest.ry` dice cosa passa e, per ogni fallimento, il valore
atteso e quello ottenuto:

```text
Esecuzione di 3 test
  ✓ un elemento
  ✓ più elementi
  ✗ negativi
errore[E015]: asserzione fallita: i valori sono diversi
 --> largest.ry:6:19
  |
6 | test "negativi" = assert_eq(largest([-3, -1, -2]), -1)
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: ottenuto: 0
      atteso:   -1
  = altro su questo errore: `rythme explain E015`

3 test: 2 superati, 1 falliti (1 ms)
```

Trentasei esercizi di questo tipo sono pronti: [**gli esercizi**](exercises/README.md),
graduati da ★☆☆ a ★★★ e allineati al corso qui sopra, ognuno con una
consegna, dei suggerimenti e dei test, in ogni lingua dei messaggi, con le
soluzioni di riferimento per gli insegnanti. Sono integrati in `rythme`:

```sh
rythme exercise list                     # gli esercizi
rythme exercise start 7                  # copia 07-recursion.ry nella cartella corrente
rythme exercise check 07-recursion.ry    # esegue i suoi test, con consigli, e propone il successivo
```

`rythme exercise check` dice anche quando i test di un esercizio integrato
sono stati cambiati: l'esercizio è risolto solo quando passano i suoi test.
I tuoi esercizi possono seguire lo stesso schema (una consegna e dei
suggerimenti nei commenti, `fail("da fare")` nelle funzioni, i test dopo
una riga `# --- Test (non cambiarli) ---`); `rythme exercise check`
funziona su qualsiasi file con dei test.

## Valutare con `--format json`

`rythme test` accetta una cartella: con un file (o una cartella) per
studente, un solo comando esegue tutti i test della classe.

```sh
rythme test --sandbox --format json submissions/ > results.json
```

- **`--format json`** stampa un documento JSON invece dei messaggi per le
  persone: per ogni file, `tests`, `failed` e `broken` (un file che non
  compila, o si ferma prima dei suoi test); per ogni test, il suo `name`,
  `file`, `passed` e `failure`; poi un `summary`. Le chiavi non cambiano
  mai ([lo schema](rythme-output.schema.json)); i messaggi seguono
  `--lang`.
- **`--sandbox`** tiene i programmi lontani da file, cartelle, variabili
  d'ambiente e rete. Non c'è limite di tempo: esegui il comando sotto un
  limite (su Linux, `timeout 60 rythme test ...`).
- **Gli studenti possono cambiare i test** del loro file. Per un voto,
  esegui una tua copia: `rythme exercise check` dice quando i test di un
  esercizio integrato sono stati cambiati, e le soluzioni di riferimento
  sono pubbliche (nel repository): per i lavori valutati, scrivi esercizi
  tuoi.
- `rythme check --format json` aggiunge gli avvisi, ognuno con il suo codice
  stabile (W001 un nome non usato, W002 un caso dimenticato...), e
  `rythme fmt --check` dice se il codice ha il formato ufficiale.

Rythme legge il JSON da sé. Questo programma trasforma i risultati in voti:

```rythme
# grades.ry
fn grade(f) = if f.broken then "{f.file}: non funziona" else "{f.file}: {f.tests - f.failed}/{f.tests}"

results = read_file(args()[0]) |> unwrap |> parse_json |> unwrap
each(results.files, fn(f) = print(grade(f)))
```

`rythme grades.ry results.json` stampa:

```text
submissions/ada.ry: 3/3
submissions/alan.ry: 2/3
submissions/grace.ry: non funziona
```

## Mostrare come gira il codice

`rythme steps` mostra un calcolo un passo alla volta, nel modello di
sostituzione: ogni chiamata viene sostituita dal corpo della sua funzione,
finché resta solo il risultato. Ricorsione, accumulatori e chiamate in coda
diventano visibili.

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

fib(4)
```

`rythme steps fib.ry` mostra:

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

`rythme steps fib.ry "fib(6)"` mostra un'altra espressione, `:steps fib(6)`
lo fa nella modalità interattiva, e così il pulsante **Passi** del
playground, dove *Condividi* crea un link che si apre allo stesso passo: un
solo link da proiettare, o da mandare alla classe.

`rythme debug file.ry` mette in pausa un programma dove chiedi
(`break 12`, `break fib`) e mostra i valori dei suoi nomi. `where` elenca
le chiamate in corso con i loro argomenti, e dice quante chiamate della
stessa funzione sono in attesa: lo stack di una ricorsione, dal vivo. Una
chiamata in coda dice che ha preso il posto di chi l'ha chiamata. In
VS Code, `F5` su un file `.ry` avvia lo stesso debugger, con i punti di
interruzione nel margine. Il [manuale](reference.md#79-the-debugger-rythme-debug)
(in inglese) mostra una sessione intera.

## Senza installare niente: il playground

Il playground del [sito](https://maxlestage.github.io/Rythme/) esegue il
vero interprete nel browser, con gli stessi messaggi e lo stesso aiuto: gli
studenti possono iniziare su qualsiasi computer, tablet o telefono.

- **Esegui**, **Controlla**, **Test**, **Formatta** e **Passi** funzionano
  come i comandi; incolla un esercizio e premi **Test**.
- **Condividi** crea un link che contiene il programma stesso (niente viene
  inviato a un server): distribuisci un punto di partenza, o il programma
  della lezione. **Seme** dà a ogni studente le stesse estrazioni di
  `random`.
- Installato come app (**Installa l'app** nel menu, o *Aggiungi alla
  schermata Home* su iPhone e iPad), il sito e il suo playground funzionano
  offline, il che aiuta in un'aula con una rete scarsa. Il codice che si
  sta scrivendo resta nel browser.
- Nel browser non si possono leggere né scrivere file, `input()` dà un
  testo vuoto, e le chiamate annidate si fermano a 600 (le chiamate in coda
  non hanno limiti).

## Nella lingua dei tuoi studenti

- Ogni messaggio, argomento di aiuto, spiegazione e pagina di
  `rythme learn` esiste in inglese, francese, spagnolo, tedesco, portoghese,
  cinese, italiano e giapponese. Rythme segue la lingua del sistema;
  `--lang it` o la variabile `RYTHME_LANG=it` ne scelgono un'altra.
- Parole chiave, nomi delle funzioni e comandi restano gli stessi in ogni
  lingua: un programma scritto in una classe gira ovunque, e `rythme help`
  accetta i nomi degli argomenti in ogni lingua (`rythme help liste`).
- Anche gli esercizi seguono la lingua: `rythme --lang de exercise start 7`
  scrive `07-rekursion.ry`, in tedesco.
- Il sito, il suo playground e questa guida esistono nelle stesse lingue.

## Gli assistenti di IA in classe

Gli studenti hanno gli assistenti a portata di mano. Rythme non cerca di
scoprirli; ecco cosa offre, e delle regole che un corso può adattare.

**Cosa offre Rythme.** Gli assistenti sono stati addestrati prima che
Rythme esistesse: lasciati da soli, scrivono abitudini di Python o di
JavaScript (`for`, `return`, `x += 1`), che Rythme rifiuta con un messaggio
che dice cosa scrivere. Con `rythme help llms`, tutto il linguaggio in un
solo testo (anche nella pagina *IA* del sito), scrivono Rythme corretto.
`rythme mcp` permette a un assistente di eseguire, controllare e testare il
codice da solo, in una sandbox. Nel playground, *Copia per l'IA* copia il
programma e il suo output o i suoi errori, pronti per una chat.
`rythme fix` applica le correzioni certe (`&&` → `and`, `let x = 3` →
`x = 3`), e `rythme fix --dry-run` si limita a mostrarle.

**Delle regole possibili.**

- **Prime settimane: il codice lo scrivono gli studenti.** Lo scopo è
  costruire i riflessi, e i messaggi e `rythme explain` spiegano già ogni
  errore. Un assistente può spiegare un messaggio o un'idea, non scrivere
  la soluzione.
- **`rythme fix` una volta letto il messaggio**: cambia solo quello che è
  certo, la stessa correzione che propone il messaggio.
- **Progetti: permesso, e dichiarato.** Dai all'assistente
  `rythme help llms`, chiedi agli studenti di dire per cosa l'hanno usato,
  e di spiegare il loro codice.
- **Valuta la comprensione, non solo i risultati.** I test controllano che
  un programma funzioni, non chi l'ha scritto. Aggiungi brevi verifiche in
  classe: prevedere i passi di `rythme steps`, l'output di un programma, o
  la correzione di un messaggio, senza computer; o un esercizio fatto sul
  momento nel playground.
- **Per il tuo lavoro**, un assistente a cui dai `rythme help llms` può
  preparare nuovi esercizi nel formato qui sopra. Controllali uno per uno
  con una tua soluzione e `rythme test` prima di distribuirli.

## Consigli pratici

- Comincia con la modalità interattiva (`rythme`): ogni riga mostra il suo
  valore. `:doc map` mostra la documentazione di una funzione, Tab completa
  i nomi.
- `rythme learn` è una visita guidata che gli studenti possono seguire da
  soli.
- `rythme help builtins` elenca tutte le funzioni predefinite per
  categoria, e ogni comando risponde a `--help` (`rythme test --help`).
  `rythme completions bash` (o `zsh`, `fish`, `powershell`) completa i
  comandi nel terminale.
- Installa l'[estensione di VS Code](../editors/vscode): colori, snippet, i
  problemi mostrati mentre gli studenti scrivono con le loro correzioni
  rapide, la documentazione sotto il mouse, e il debugger. Anche altri
  editor possono usare il server del linguaggio
  ([`editors/README.md`](../editors/README.md)).
- `rythme fmt` dà a ogni file lo stesso formato: gli studenti possono
  eseguirlo (o `rythme fmt --check .`) prima di consegnare il loro lavoro.
- Per un progetto più grande, `rythme new NOME` crea una cartella con
  `src/`, `tests/` e `rythme.toml`; `rythme test` da solo esegue allora
  tutti i test del progetto, e `rythme add` porta un pacchetto
  dell'[indice dei pacchetti](reference.md#711-the-package-index).
