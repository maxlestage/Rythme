# Rythme

**Plus simple que Python et Ruby, aussi agréable que Ruby : un langage de
programmation purement fonctionnel, avec de l'aide partout.**
Les programmes sont des fichiers qui finissent par `.ry`. L'interpréteur est écrit en Rust.

*[English version →](README.md) · [En español →](README.es.md) · [Auf Deutsch →](README.de.md) · [Em português →](README.pt.md) · [中文 →](README.zh.md)*

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

La même chose, à la manière de Ruby (blocs, enchaînements de méthodes,
noms en `?`, appels sans parenthèses) :

```rythme
noms = ["Ada", "Grace", "Alan"]
noms.each do |nom|
  puts "Bonjour, #{nom} !"
end

total = [1, 2, 3, 4]
  .select(even?)
  .map { |x| x * x }
  .sum
puts total                 # 20
```

## Pourquoi Rythme ?

- **Plus simple que Python ou Ruby.** 15 mots-clés (Python en a 35). Une seule
  façon de faire chaque chose. Pas de classes, pas de `null`, pas
  d'exceptions, pas de boucles, pas de modification : moins de concepts à
  apprendre, moins de bugs à traquer.
- **Aussi agréable que Ruby.** Des blocs (`xs.map do |x| x * 2 end`,
  `xs.select { |x| x > 2 }`), des enchaînements de méthodes
  (`xs.filter(is_even).sum`, une étape par ligne), des appels sans
  parenthèses (`puts "salut"`, `print x, y`), des noms comme `even?` et
  `empty?`, `unless`, des symboles (`:ok`), `"Bonjour #{nom}"`, et les noms
  Ruby de la bibliothèque (`puts`, `upcase`, `select`, `reject`, `reduce`,
  `each_with_index`...). `rythme help ruby` montre ce qui est pareil et ce
  qui change.
- **Purement fonctionnel.** Les valeurs sont immuables, les fonctions sont des
  valeurs, chaque `if` et chaque `match` est une expression. La récursion en
  position terminale n'épuise jamais la pile.
- **Sûr comme Rust.** Pas de `null` : une valeur qui peut manquer est une
  `Option` (`Some(x)` ou `None`). Pas d'exceptions : une opération qui peut
  échouer renvoie un `Result` (`Ok(x)` ou `Err(e)`), et `?` transmet les
  erreurs. Types algébriques, filtrage par motifs, un vérificateur qui signale
  les cas oubliés, aucune conversion silencieuse, des nombres entiers qui ne
  débordent jamais.
- **De l'aide partout, tout le temps.**
  - Chaque erreur dit *où* elle est, *pourquoi*, et *comment la corriger*,
    comme Rust, et propose le bon nom en cas de faute de frappe.
  - Vous venez d'un autre langage ? `for`, `return`, `null`, `x += 1`, `&&`,
    `def`, `class`... chacun reçoit une explication de la façon de faire en
    Rythme.
  - Le programme est vérifié **avant** d'être exécuté : tous les noms
    inconnus, les mauvais nombres d'arguments, les opérations sur de mauvaises
    sortes de valeurs (`"âge : " + 36`, `len(5)`, un champ mal écrit), les
    appels qui échouent à coup sûr (`aire("grand")` quand `aire` multiplie son
    paramètre) et les cas oubliés sont signalés d'un coup.
  - `rythme explain E003` explique n'importe quel code d'erreur en détail,
    avec des exemples.
  - `rythme help`, `rythme help map`, `help(x)` dans le code, `:doc` dans le
    mode interactif, `rythme learn` pour une visite guidée.
  - Dans votre éditeur : les problèmes pendant la frappe, la documentation
    sous la souris, la complétion, aller à la définition, renommer
    (`rythme lsp`, le serveur de langage, pour VS Code, Neovim, Helix, Emacs...).
  - Chaque message, chaque documentation existe en **français, en anglais, en
    espagnol, en allemand, en portugais et en chinois** (`--lang fr`,
    `--lang es`, `--lang de`, `--lang pt`, `--lang zh`, ou automatiquement
    selon la langue du système).

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
forme, lire l'aide, en français, anglais, espagnol, allemand, portugais ou chinois,
sans rien installer. Le site
s'installe aussi comme une application (une Progressive Web App) et fonctionne
hors ligne, playground compris. Son menu propose des thèmes de couleurs
(Auto, Clair, Sombre, Contraste élevé, Sépia, Tempo), et aucune page ne défile
de côté, d'un téléphone de 320 px à un grand écran.

Le dossier [`site/`](site) est le site web de Rythme (accueil, visite du
langage, exemples, playground et guide pour les enseignants, en six
langues), écrit en Rust avec Paquets.

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
`--version v1.0.0` (une autre version) ; `--help` les liste.

**Windows** (PowerShell) : le programme va dans
`%LOCALAPPDATA%\Programs\Rythme\bin`, ajouté à votre `PATH`.

```powershell
irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex
```

Définissez `RYTHME_INSTALL_DIR` ou `RYTHME_VERSION` avant, ou lancez le
fichier téléchargé : `.\install.ps1 -Dir C:\Outils -Version v1.0.0 -NoModifyPath`.

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
| `rythme bonjour.ry`         | lancer un programme (`rythme run bonjour.ry` marche aussi ; `rythme run --seed 42 bonjour.ry` : les mêmes tirages de `random` à chaque fois) |
| `rythme check bonjour.ry`   | trouver les problèmes sans lancer le programme      |
| `rythme test bonjour.ry`    | lancer les tests écrits dans le fichier (aussi un dossier : `rythme test .`, avec `--filter NOM`, `--watch`) |
| `rythme fmt bonjour.ry`     | mettre le code en forme dans le style officiel (aussi un dossier : `rythme fmt .`) |
| `rythme fmt --check .`      | lister les fichiers à mettre en forme, sans les changer (pour l'intégration continue) |
| `rythme doc bonjour.ry`     | sa documentation tirée des commentaires `##`, en Markdown (`--html` : une page web) |
| `rythme help [sujet]`       | aide sur un sujet ou une fonction                   |
| `rythme explain E003`       | expliquer un code d'erreur                          |
| `rythme learn`              | visite guidée du langage                            |
| `rythme new bonjour`        | un nouveau projet : `rythme.toml`, `src/`, `tests/` ; puis `rythme run`, `rythme test`... sans fichier |
| `rythme install`            | télécharger les dépendances de `rythme.toml` (git ou dossiers), et écrire `rythme.lock` |
| `rythme exercise`           | [30 exercices gradués](docs/exercises/README.fr.md) : `list`, `start 1`, `check FICHIER` |
| `rythme etapes fib.ry "fib(4)"` | le calcul étape par étape : `fib(4)` → `fib(3) + fib(2)` → ... |
| `rythme -e 'print(1 + 2)'`  | exécuter un morceau de code                         |
| `rythme lsp`                | serveur de langage, lancé par votre éditeur ([éditeurs](editors/README.fr.md)) |

Ajoutez `--lang` et une langue (`fr`, `en`, `es`, `de`, `pt` ou `zh`) avant la
commande pour choisir la langue des messages, ou définissez `RYTHME_LANG`.

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

# Listes, intervalles, enregistrements, dictionnaires et ensembles
nombres = 1..10                           # [1, 2, ..., 10]
ada = {nom: "Ada", naissance: 1815}
autre = {..ada, naissance: 1816}          # un nouvel enregistrement ; ada ne change pas
ages = dict([["Ada", 36], ["Alan", 41]])  # un dictionnaire : des clés de toute sorte
vus = set([1, 2, 2])                      # un ensemble : set(1, 2)

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
| [`library.ry`](examples/library.ry) | le style Ruby : blocs `do \|x\| ... end`, enchaînements de méthodes, noms en `?`, `unless`, symboles |
| [`sales.ry`](examples/sales.ry) | un rapport : `scan`, colonnes alignées avec `pad_left`, `windows`, `chunks`, JSON |
| [`calendar.ry`](examples/calendar.ry) | des dates : la page d'un mois, des textes ISO 8601, `days_between`, `add_days` |
| [`logs.ry`](examples/logs.ry) | lire un journal avec des regex : `named_captures`, `matches`, `find_all`, `replace_all`, `format_number` |
| [`grades.ry`](examples/grades.ry) | les notes d'un tableur : `csv_records` avec `;`, `csv_format`, un tirage répétable avec `random_seed` |
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
  fichier, la mise en forme, les types des noms, *Lancer* et *Lancer le
  test* au-dessus du code, en français ou en anglais.
- **Neovim, Helix, Zed, Emacs, Sublime Text, Kate...** : tout éditeur qui
  parle le Language Server Protocol peut lancer `rythme lsp` ; quelques
  lignes de configuration pour chacun sont dans [`editors/README.fr.md`](editors/README.fr.md).

## Pour les enseignants

[`docs/enseigner.md`](docs/enseigner.md) propose un plan de cours construit
sur Rythme, et explique pourquoi il convient aussi bien à un premier cours de
programmation qu'à un cours de programmation fonctionnelle.
[`docs/exercises/`](docs/exercises/README.fr.md) contient 30 exercices
gradués, avec énoncé, indices, tests et solutions, que `rythme exercise`
distribue et vérifie dans chaque langue des messages.

## État

Rythme 1.0 est un langage complet et testé : l'interpréteur, le vérificateur,
le formateur, le serveur de langage, les éditeurs, le bac à sable, le manuel
et des messages en six langues. Son contenu est décrit dans
[`CHANGELOG.md`](CHANGELOG.md) ; des idées pour la suite sont à la fin de
[`ROADMAP.md`](ROADMAP.md#after-10). Les limites, honnêtement :

- **Vitesse.** C'est un interpréteur qui parcourt l'arbre du programme : les
  noms locaux deviennent des cases numérotées avant l'exécution, et la
  plupart des appels n'allouent rien. Les programmes de [`bench/`](bench)
  (`bench/run.sh`) tournent 1,2 à 4,3 fois plus vite qu'avec la première
  version, Rythme 0.1 ;
  c'est de l'ordre de Python, loin de Rust compilé. Construire un texte avec
  `+` ou `"{acc}..."` et ajouter à une liste (`push`, `xs + [x]`,
  `[..xs, x]`) prennent un temps constant par ajout : 100 000 ajouts
  prennent 0,1 s, au lieu de 23 s (textes) et 150 s (listes) avant. Les
  fonctions locales qui s'appellent entre elles ne créent plus de fermeture
  à chaque appel (deux fois plus rapide).
- **Types.** Les valeurs sont vérifiées à l'exécution (strictement : aucune
  conversion silencieuse), et `rythme check` infère leurs types quand ils
  sont certains, pour signaler avant l'exécution les opérations qui
  échoueraient à coup sûr ; il déduit aussi ce que doivent être les
  paramètres de chaque fonction, pour signaler les appels qui échouent à
  coup sûr, et l'éditeur montre ces signatures au survol.
- **Outils.** `rythme fmt` met le code en forme dans le style officiel (voir
  `rythme help fmt`), `rythme doc` écrit la documentation de fichiers et de
  dossiers à partir de leurs commentaires `##` (voir `rythme help doc`),
  `rythme new` et `rythme install` créent des projets avec leurs dépendances
  (voir `rythme help projets`), et le serveur de langage (`rythme lsp`)
  apporte les erreurs et la documentation dans tous les éditeurs (voir
  `rythme help editors`). Le bac à sable en ligne fait tourner l'interpréteur
  compilé en WebAssembly.
- **Langues.** Les messages existent en anglais, en français, en espagnol, en
  allemand, en portugais et en chinois simplifié ; ajouter une langue consiste
  à écrire un fichier de catalogue, `src/i18n/<code>.rs`, qu'un test compare à
  tous les textes du programme (voir
  [CONTRIBUTING](CONTRIBUTING.md#messages-in-every-language)). Les
  contributions sont bienvenues.
- **Nombres.** Les nombres entiers n'ont pas de limite de taille et restent
  exacts (jusqu'à 100000 chiffres), les habituels sur 64 bits calculés le plus
  vite ; les nombres décimaux sont sur 64 bits.

## Promesse de stabilité

Rythme suit le [versionnage sémantique](https://semver.org/lang/fr/). Pour
toutes les versions 1.x (1.0, 1.1, 1.2...) :

- **Un programme qui fonctionne continue de fonctionner, avec la même
  sortie.** Le même programme, avec les mêmes entrées (arguments, fichiers,
  entrée standard, horloge), affiche la même chose sur la sortie standard (à
  part ce que tirent `random` et `shuffle`) et se termine avec le même code
  de sortie. La syntaxe, les mots-clés, les
  fonctions intégrées (leurs noms, leurs paramètres et leurs résultats), les
  formats de `rythme.toml` et de `rythme.lock`, et les commandes et options
  de `rythme` continuent de fonctionner. Aucun mot ne devient un mot-clé ;
  une nouvelle fonction intégrée ne remplace jamais une fonction à vous du
  même nom (votre définition l'emporte, et `rythme check` le signale avec
  W003).
- **Les codes d'erreur et d'avertissement sont stables.** `E003` ou `W004`
  gardent leur sens, et `rythme explain` continue de les expliquer ; de
  nouveaux codes peuvent s'ajouter, jamais être réutilisés.
- **Rien ne disparaît sans prévenir.** Ce qui doit être retiré est d'abord
  *déprécié* : cela continue de fonctionner et donne un avertissement, avec
  la façon de le remplacer, pendant au moins une version mineure entière
  (déprécié en 1.3, encore là et signalé pendant toute la 1.4). Les retraits
  n'arrivent que dans une 2.0.

Ce qui peut changer dans une version 1.x :

- la formulation des messages, des conseils et des pages d'aide, dans toutes
  les langues (comptez sur les codes, pas sur le texte) ;
- les performances, dans un sens ou dans l'autre pour certains programmes,
  même si le but est toujours d'aller plus vite ;
- de nouveaux avertissements de `rythme check`, de nouvelles fonctions
  intégrées, commandes, options et pages d'aide ;
- la mise en forme choisie par `rythme fmt` pour du code qu'il ne savait pas
  mettre en forme, ou mettait mal en forme ;
- la sortie d'un programme qui dépendait d'un bogue (un plantage, un résultat
  faux) : le corriger ne rompt pas la promesse.

## Licence

[MIT](LICENSE).
