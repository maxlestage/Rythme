# Rythme

**Più semplice di Python e di Ruby, piacevole come Ruby: un linguaggio di programmazione puramente funzionale, con aiuto ovunque.**
I programmi sono file che finiscono con `.ry`. L'interprete è scritto in Rust.

*[English version →](README.md) · [Version française →](README.fr.md) · [En español →](README.es.md) · [Auf Deutsch →](README.de.md) · [Em português →](README.pt.md) · [中文 →](README.zh.md) · [日本語 →](README.ja.md)*

Questo README è un riassunto. I messaggi, l'aiuto e le spiegazioni degli
errori di Rythme esistono in italiano (`--lang it`), e così la
[guida per gli insegnanti](docs/teaching.it.md); il
[manuale di riferimento](docs/reference.md) completo è in inglese e in
francese. Il sito e il playground esistono anche in italiano:
<https://maxlestage.github.io/Rythme/it/>.

```rythme
# I valori non cambiano mai. Tutto è una funzione. Niente cicli.
fn square(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # tieni i numeri pari
  |> map(square)          # eleva ognuno al quadrato
  |> sum                  # sommali
  |> print                # 20
```

## Perché Rythme?

- **Più semplice di Python o di Ruby.** 15 parole chiave. Un solo modo di
  fare ogni cosa. Niente classi, niente `null`, niente eccezioni, niente
  cicli, niente mutazioni: meno concetti da imparare, meno bug da cercare.
- **Piacevole come Ruby.** Blocchi (`xs.map do |x| x * 2 end`), catene di
  metodi, chiamate senza parentesi (`puts "ciao"`), nomi come `even?`,
  `unless`, simboli (`:ok`), e i nomi di Ruby nella libreria standard.
- **Sicuro come Rust.** Un valore che può mancare è un `Option` (`Some(x)`
  o `None`); un'operazione che può fallire restituisce un `Result` (`Ok(x)`
  o `Err(e)`), e `?` passa avanti l'errore. Il programma viene controllato
  **prima** di partire.
- **Aiuto ovunque.** Ogni errore dice *dove* si trova, *perché*, e *come*
  correggerlo. `rythme explain E003` spiega ogni codice di errore,
  `rythme help` e `rythme learn` (una visita guidata) insegnano il
  linguaggio, e il server del linguaggio (`rythme lsp`) porta lo stesso aiuto
  nel tuo editor.

Ecco come appaiono gli errori (in italiano):

```
errore[E005]: impossibile usare `+` con Text e Int
 --> report.ry:2:15
  |
2 | print("età: " + age)
  |               ^ Text "età: " + Int
  |
  = aiuto: per costruire un testo con dentro un valore, usa le graffe: "totale: {x}"
      o converti prima il valore: text(x)
  = altro su questo errore: `rythme explain E005`
```

```
errore[E001]: `for` non si usa in Rythme
 --> loop.ry:1:1
  |
1 | for x in numbers
  | ^^^ viene da un altro linguaggio
  |
  = aiuto: Rythme non ha cicli. Lavora invece sull'intera lista:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      o scrivi una funzione ricorsiva. Vedi `rythme help functional`.
  = altro su questo errore: `rythme explain E001`
```

## Installazione

**Linux e macOS:**

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

**Windows** (PowerShell):

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

**Con [Rust](https://rustup.rs):** `cargo install --git https://github.com/maxlestage/Rythme`.
Altri modi nel [README in inglese](README.md#install).

## Uso

| Comando                     | Cosa fa                                          |
|-----------------------------|--------------------------------------------------|
| `rythme`                    | modalità interattiva: scrivi codice, guarda il risultato |
| `rythme hello.ry`           | esegue un programma                              |
| `rythme check hello.ry`     | trova i problemi senza eseguire il programma     |
| `rythme test hello.ry`      | esegue i test scritti nel file                   |
| `rythme fmt hello.ry`       | dà al codice il suo formato ufficiale            |
| `rythme new hello`          | nuovo progetto: `rythme.toml`, `src/`, `tests/`  |
| `rythme add mathlib`        | aggiunge un pacchetto dell'[indice dei pacchetti](docs/reference.md#711-the-package-index) (`rythme search PAROLE` li trova) |
| `rythme help [argomento]`   | aiuto su un argomento o una funzione (`rythme help liste`) |
| `rythme explain E003`       | spiega un codice di errore                       |
| `rythme learn`              | visita guidata del linguaggio                    |
| `rythme steps fib.ry "fib(4)"` | mostra il calcolo passo per passo             |
| `rythme debug fib.ry`          | il debugger: punti di interruzione, riga per riga, valori |

I messaggi sono in italiano con `--lang it` prima del comando, con
`RYTHME_LANG=it`, o quando la lingua del sistema è l'italiano
(`LANG=it_IT.UTF-8`). Parole chiave, nomi delle funzioni predefinite e
comandi sono gli stessi in tutte le lingue. Nella modalità interattiva,
`:lang it` cambia lingua.

## Promessa di stabilità

Rythme 1.0 segue il [versionamento semantico](https://semver.org/lang/it/).
In tutte le versioni 1.x: un programma che funziona continua a funzionare,
con lo stesso output (a parità di input) e lo stesso codice di uscita; i
codici di errore e di avviso (`E003`, `W004`...) mantengono il loro
significato; una funzionalità da togliere riceve prima un avviso per almeno
una versione minore intera, e viene tolta solo nella 2.0. Possono cambiare
la formulazione dei messaggi, le prestazioni e i nuovi avvisi di
`rythme check`. Dettagli nel [README in inglese](README.md#stability-promise),
il contenuto della 1.0 in [`CHANGELOG.md`](CHANGELOG.md).

## Licenza

[MIT](LICENSE).
