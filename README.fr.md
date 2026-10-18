# Rythme

**Un langage de programmation simple, purement fonctionnel, avec de l'aide partout.**
Les programmes sont des fichiers qui finissent par `.ry`. L'interpréteur est écrit en Rust.

*[English version →](README.md)*

**Documentation :** le [manuel de référence](docs/reference.fr.md) (tout le
langage, avec précision), le [guide pour les enseignants](docs/enseigner.md),
et [tous les documents](docs/README.md#en-français).

```rythme
# Les valeurs ne changent jamais. Tout est fonction. Il n'y a pas de boucles.
fn carre(x) = x * x

[1, 2, 3, 4]
  |> filter(is_even)      # garder les nombres pairs
  |> map(carre)           # mettre chacun au carré
  |> sum                  # les additionner
  |> print                # 20
```

## Pourquoi Rythme ?

- **Plus simple que Python ou Ruby.** 15 mots-clés (Python en a 35). Une seule
  façon de faire chaque chose. Pas de classes, pas de `null`, pas
  d'exceptions, pas de boucles, pas de modification : moins de concepts à
  apprendre, moins de bugs à traquer.
- **Purement fonctionnel.** Les valeurs sont immuables, les fonctions sont des
  valeurs, chaque `if` et chaque `match` est une expression. La récursion en
  position terminale n'épuise jamais la pile.
- **Sûr comme Rust.** Pas de `null` : une valeur qui peut manquer est une
  `Option` (`Some(x)` ou `None`). Pas d'exceptions : une opération qui peut
  échouer renvoie un `Result` (`Ok(x)` ou `Err(e)`), et `?` transmet les
  erreurs. Types algébriques, filtrage par motifs, un vérificateur qui signale
  les cas oubliés, aucune conversion silencieuse, aucun dépassement d'entier
  silencieux.
- **De l'aide partout, tout le temps.**
  - Chaque erreur dit *où* elle est, *pourquoi*, et *comment la corriger*,
    comme Rust, et propose le bon nom en cas de faute de frappe.
  - Vous venez d'un autre langage ? `for`, `return`, `null`, `x += 1`, `&&`,
    `def`, `class`... chacun reçoit une explication de la façon de faire en
    Rythme.
  - Le programme est vérifié **avant** d'être exécuté : tous les noms
    inconnus, les mauvais nombres d'arguments, les opérations sur de mauvaises
    sortes de valeurs (`"âge : " + 36`, `len(5)`, un champ mal écrit) et les
    cas oubliés sont signalés d'un coup.
  - `rythme explain E003` explique n'importe quel code d'erreur en détail,
    avec des exemples.
  - `rythme help`, `rythme help map`, `help(x)` dans le code, `:doc` dans le
    mode interactif, `rythme learn` pour une visite guidée.
  - Dans votre éditeur : les problèmes pendant la frappe, la documentation
    sous la souris, la complétion, aller à la définition, renommer
    (`rythme lsp`, le serveur de langage, pour VS Code, Neovim, Helix, Emacs...).
  - Chaque message, chaque documentation existe en **français et en
    anglais** (`--lang fr`, ou automatiquement selon la langue du système).

Voici à quoi ressemble une erreur :

```
erreur[E005]: impossible d'utiliser `+` avec Text et Int
 --> rapport.ry:3:16
  |
3 | print("âge : " + 36)
  |                ^ Text "âge : " + Int 36
  |
  = aide: pour construire un texte contenant une valeur, utilisez des accolades : "total : {x}"
      ou convertissez d'abord la valeur : text(x)
  = en savoir plus : `rythme explain E005`
```

## Essayer dans le navigateur

Le site <https://maxlestage.github.io/Rythme/> a un playground qui fait
tourner le vrai interpréteur, compilé en WebAssembly ([`playground/`](playground)) :
modifier du code, le lancer, le vérifier, lancer des tests, le mettre en
forme, lire l'aide, en français ou en anglais, sans rien installer. Le site
s'installe aussi comme une application (une Progressive Web App) et fonctionne
hors ligne, playground compris.

Le dossier [`site/`](site) est le site web de Rythme (accueil, visite du
langage, exemples, playground et guide pour les enseignants, en français et en
anglais), écrit en Rust avec Paquets.

### Déployer le site sur Heroku

[![Déployer sur Heroku](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/maxlestage/Rythme)

1. Touchez le bouton (ça marche depuis un téléphone), choisissez un nom
   d'application, touchez **Deploy app**. Heroku lit [`app.json`](app.json) et
   construit le playground (l'interpréteur en WebAssembly) et le serveur avec le
   [buildpack emk/rust](https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust)
   ([`RustConfig`](RustConfig), [`heroku-build.sh`](heroku-build.sh),
   [`Procfile`](Procfile)). Le premier build prend quelques minutes.
2. Pour déployer chaque fusion dans `master` : dans GitHub, *Settings → Secrets
   and variables → Actions*, ajoutez le secret `HEROKU_API_KEY` (Heroku →
   *Account settings* → *API Key*) et la variable `HEROKU_APP_NAME`. Le workflow
   [`Deploy to Heroku`](.github/workflows/heroku.yml) fait le reste, et crée
   l'application si elle n'existe pas.

En ligne de commande : `heroku create --stack heroku-26 --buildpack emk/rust`,
puis `git push heroku master`. Avec Docker à la place :
`heroku create --stack container` ([`heroku.yml`](heroku.yml),
[`Dockerfile`](Dockerfile)), ou sur n'importe quel hôte,
`docker build -t rythme-site . && docker run --rm -p 8080:8080 rythme-site`.
Détails dans [`site/README.md`](site/README.md#heroku).

## Installer

**Linux et macOS** (x86_64 et arm64) : le script télécharge le programme de
la dernière version, vérifie sa somme SHA-256 et le place dans
`~/.local/bin`.

```sh
curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
```

Options après `sh -s --` : `--dir /usr/local/bin` (un autre dossier),
`--version v0.1.0` (une autre version) ; `--help` les liste.

**Windows** (PowerShell) : le programme va dans
`%LOCALAPPDATA%\Programs\Rythme\bin`, ajouté à votre `PATH`.

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

Définissez `RYTHME_INSTALL_DIR` ou `RYTHME_VERSION` avant, ou lancez le
fichier téléchargé : `.\install.ps1 -Dir C:\Outils -Version v0.1.0 -NoModifyPath`.

**À la main :** chaque version publiée est accompagnée de programmes prêts à
l'emploi pour Linux, macOS et Windows sur la
[page des versions](https://github.com/maxlestage/Rythme/releases), et de
leurs sommes de contrôle dans `SHA256SUMS`. Téléchargez l'archive de votre
système, extrayez `rythme` (`rythme.exe` sous Windows) et placez-le dans un
dossier de votre `PATH`.

**Homebrew** (macOS, Linux) et **winget** (Windows), une fois les paquets
publiés ([`packaging/`](packaging/README.md)) :

```sh
brew install maxlestage/rythme/rythme
winget install Rythme.Rythme
```

**Avec [Rust](https://rustup.rs)** (une version stable récente) :

```sh
cargo install rythme                                    # depuis crates.io, une fois publié
cargo install --git https://github.com/maxlestage/Rythme   # le code le plus récent
```

Ou depuis une copie de ce dépôt :

```sh
cargo build --release        # le programme est target/release/rythme
cargo install --path .       # ou l'installer dans le PATH
```

## Utiliser

| Commande                    | Ce qu'elle fait                                     |
|-----------------------------|-----------------------------------------------------|
| `rythme`                    | mode interactif : tapez du code, voyez le résultat  |
| `rythme bonjour.ry`         | lancer un programme (`rythme run bonjour.ry` marche aussi) |
| `rythme check bonjour.ry`   | trouver les problèmes sans lancer le programme      |
| `rythme test bonjour.ry`    | lancer les tests écrits dans le fichier             |
| `rythme fmt bonjour.ry`     | mettre le code en forme dans le style officiel (aussi un dossier : `rythme fmt .`) |
| `rythme fmt --check .`      | lister les fichiers à mettre en forme, sans les changer (pour l'intégration continue) |
| `rythme doc bonjour.ry`     | sa documentation tirée des commentaires `##`, en Markdown (`--html` : une page web) |
| `rythme help [sujet]`       | aide sur un sujet ou une fonction                   |
| `rythme explain E003`       | expliquer un code d'erreur                          |
| `rythme learn`              | visite guidée du langage                            |
| `rythme exercise`           | [30 exercices gradués](docs/exercises/README.fr.md) : `list`, `start 1`, `check FICHIER` |
| `rythme -e 'print(1 + 2)'`  | exécuter un morceau de code                         |
| `rythme lsp`                | serveur de langage, lancé par votre éditeur ([éditeurs](editors/README.fr.md)) |

Ajoutez `--lang fr` ou `--lang en` avant la commande pour choisir la langue
des messages, ou définissez `RYTHME_LANG`.

## Le langage en deux minutes

```rythme
# Noms et valeurs (ils ne changent jamais)
nom = "Ada"
naissance = 1815
print("{nom} est née en {naissance}")    # {...} insère une valeur

# Fonctions
fn double(x) = x * 2
fn saluer(nom)
  message = "Bonjour, {nom} !"
  print(message)                          # la dernière ligne est le résultat
end

# Une définition par cas, essayées dans l'ordre
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

# Les conditions sont des expressions : chaque if a un else
fn signe(n) = if n < 0 then "négatif" else if n == 0 then "zéro" else "positif"

# Listes, intervalles, enregistrements
nombres = 1..10                           # [1, 2, ..., 10]
ada = {nom: "Ada", naissance: 1815}
autre = {..ada, naissance: 1816}          # un nouvel enregistrement ; ada ne change pas

# Vos propres types, et match
type Forme = Cercle(rayon) | Rect(largeur, hauteur)

fn aire(forme) = match forme
  Cercle(r) -> pi * r * r
  Rect(l, h) -> l * h
end

# Les erreurs sont des valeurs
fn diviser(a, b) = if b == 0 then Err("division par zéro") else Ok(a / b)

fn moyenne(a, b)
  x = parse_float(a)?                     # s'arrête ici et renvoie le Err
  y = parse_float(b)?
  Ok((x + y) / 2)
end

# Les tests vivent à côté du code : rythme test fichier.ry
test "double marche" = assert_eq(double(21), 42)

# Autres fichiers, utilisés par leur nom
use geometrie                             # geometrie.ry, à côté de ce fichier
print(geometrie.aire(2))
```

Les mots-clés et les fonctions intégrées sont en anglais, comme dans tous les
langages répandus (c'est ce qui permet de partager du code dans le monde
entier), mais vos propres noms peuvent être en français, accents compris :
`prénom`, `âge`, `résultat`.

Pour aller plus loin : le [manuel de référence](docs/reference.fr.md),
[`examples/`](examples), `rythme help syntax`, `rythme learn`.

## Exemples

| Fichier | Montre |
|---------|--------|
| [`hello.ry`](examples/hello.ry) | le premier programme classique |
| [`fizzbuzz.ry`](examples/fizzbuzz.ry) | `match` sur une liste, intervalles, `each` |
| [`fibonacci.ry`](examples/fibonacci.ry) | définitions par cas, appels terminaux, documentation `##` |
| [`shapes.ry`](examples/shapes.ry) | vos propres types, gardes, `sort_by` |
| [`quicksort.ry`](examples/quicksort.ry) | motifs de listes, `filter`, `+` sur les listes |
| [`words.ry`](examples/words.ry) | traitement de texte, `group_by`, enregistrements |
| [`cipher.ry`](examples/cipher.ry) | le chiffre de César : `char_code`, `char`, compter les lettres |
| [`results.ry`](examples/results.ry) | `Option`, `Result`, `?` |
| [`students.ry`](examples/students.ry) | enregistrements, mise à jour, regroupements, moyennes |
| [`sales.ry`](examples/sales.ry) | un rapport : `scan`, colonnes alignées avec `pad_left`, `windows`, `chunks`, JSON |
| [`calendar.ry`](examples/calendar.ry) | des dates : la page d'un mois, des textes ISO 8601, `days_between`, `add_days` |
| [`tree.ry`](examples/tree.ry) | un type récursif : arbre binaire de recherche |
| [`primes.ry`](examples/primes.ry) | un crible récursif, `all`, `and` |
| [`testing.ry`](examples/testing.ry) | les blocs `test` |
| [`modules/`](examples/modules) | `use geometrie` puis `geometrie.aire(2)` : un programme en plusieurs fichiers, noms `_privés` |
| [`bonjour.ry`](examples/bonjour.ry) | des noms avec des accents |

## Éditeurs

- **VS Code** : l'extension dans [`editors/vscode`](editors/vscode) : couleurs,
  extraits de code, et le serveur de langage, pour les problèmes pendant la
  frappe, la documentation au survol, la complétion, les paramètres d'un
  appel, aller à la définition, les références, renommer, le plan du
  fichier et la mise en forme, en français ou en anglais.
- **Neovim, Helix, Zed, Emacs, Sublime Text, Kate...** : tout éditeur qui
  parle le Language Server Protocol peut lancer `rythme lsp` ; quelques
  lignes de configuration pour chacun sont dans [`editors/README.fr.md`](editors/README.fr.md).

## Pour les enseignants

[`docs/enseigner.md`](docs/enseigner.md) propose un plan de cours construit
sur Rythme, et explique pourquoi il convient aussi bien à un premier cours de
programmation qu'à un cours de programmation fonctionnelle.
[`docs/exercises/`](docs/exercises/README.fr.md) contient 30 exercices
gradués, en français et en anglais, avec énoncé, indices, tests et solutions,
que `rythme exercise` distribue et vérifie.

## État et feuille de route

Rythme 0.1 est un interpréteur complet et testé, utilisable dès aujourd'hui
pour apprendre et pour de petits programmes. Les limites, honnêtement, et la
suite (la liste complète vers la 1.0 est dans [`ROADMAP.md`](ROADMAP.md)) :

- **Vitesse.** C'est un interpréteur qui parcourt l'arbre du programme : les
  noms locaux deviennent des cases numérotées avant l'exécution, et la
  plupart des appels n'allouent rien. Les programmes de [`bench/`](bench)
  (`bench/run.sh`) tournent 1,2 à 4,3 fois plus vite qu'avec Rythme 0.1 ;
  c'est de l'ordre de Python, loin de Rust compilé. Suite : une machine
  virtuelle à bytecode.
- **Types.** Les valeurs sont vérifiées à l'exécution (strictement : aucune
  conversion silencieuse), et `rythme check` infère leurs types quand ils
  sont certains, pour signaler avant l'exécution les opérations qui
  échoueraient à coup sûr. Suite : les types des paramètres des fonctions,
  pour trouver encore plus d'erreurs avant l'exécution.
- **Outils.** `rythme fmt` met le code en forme dans le style officiel (voir
  `rythme help fmt`), `rythme doc` écrit la documentation de fichiers et de
  dossiers à partir de leurs commentaires `##` (voir `rythme help doc`), et le
  serveur de langage (`rythme lsp`) apporte les
  erreurs et la documentation dans tous les éditeurs (voir
  `rythme help editors`). Le bac à sable en ligne fait tourner l'interpréteur
  compilé en WebAssembly. Suite : un gestionnaire de paquets.
- **Langues.** Les messages existent en anglais et en français ; ajouter une
  langue consiste à traduire les textes de `src/`. Les contributions sont
  bienvenues.
- Les nombres sont sur 64 bits : les entiers s'arrêtent avec une erreur
  au-delà de ±9,2 × 10¹⁸ (utilisez les nombres décimaux au-delà).

## Licence

[MIT](LICENSE).
