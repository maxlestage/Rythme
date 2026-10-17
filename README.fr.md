# Rythme

**Un langage de programmation simple, purement fonctionnel, avec de l'aide partout.**
Les programmes sont des fichiers qui finissent par `.ry`. L'interpréteur est écrit en Rust.

*[English version →](README.md)*

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
    inconnus, les mauvais nombres d'arguments et les cas oubliés sont signalés
    d'un coup.
  - `rythme explain E003` explique n'importe quel code d'erreur en détail,
    avec des exemples.
  - `rythme help`, `rythme help map`, `help(x)` dans le code, `:doc` dans le
    mode interactif, `rythme learn` pour une visite guidée.
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

## Installer

**Sans rien installer d'autre :** chaque version publiée est accompagnée de
programmes prêts à l'emploi pour Linux, macOS et Windows sur la
[page des versions](https://github.com/maxlestage/Rythme/releases).
Téléchargez l'archive de votre système, extrayez `rythme` (`rythme.exe` sous
Windows) et placez-le dans un dossier de votre `PATH`.

**Avec [Rust](https://rustup.rs)** (une version stable récente) :

```sh
cargo install --git https://github.com/maxlestage/Rythme
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
| `rythme help [sujet]`       | aide sur un sujet ou une fonction                   |
| `rythme explain E003`       | expliquer un code d'erreur                          |
| `rythme learn`              | visite guidée du langage                            |
| `rythme -e 'print(1 + 2)'`  | exécuter un morceau de code                         |

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

# Autres fichiers
use "geometrie.ry"
```

Les mots-clés et les fonctions intégrées sont en anglais, comme dans tous les
langages répandus (c'est ce qui permet de partager du code dans le monde
entier), mais vos propres noms peuvent être en français, accents compris :
`prénom`, `âge`, `résultat`.

Pour aller plus loin : [`examples/`](examples), `rythme help syntax`, `rythme learn`.

## Éditeurs

- **VS Code** : couleurs, indentation et extraits de code dans [`editors/vscode`](editors/vscode).

## Pour les enseignants

[`docs/enseigner.md`](docs/enseigner.md) propose un plan de cours construit
sur Rythme, et explique pourquoi il convient aussi bien à un premier cours de
programmation qu'à un cours de programmation fonctionnelle.

## État et feuille de route

Rythme 0.1 est un interpréteur complet et testé, utilisable dès aujourd'hui
pour apprendre et pour de petits programmes. Les limites, honnêtement, et la
suite :

- **Vitesse.** C'est un interpréteur qui parcourt l'arbre du programme :
  à peu près aussi rapide que Python, loin de Rust compilé. Suite : une
  machine virtuelle à bytecode.
- **Types.** Les valeurs sont vérifiées à l'exécution (strictement : aucune
  conversion silencieuse). Suite : l'inférence de types dans `rythme check`,
  pour trouver plus d'erreurs avant l'exécution.
- **Outils.** Suite : un formateur (`rythme fmt`), un serveur de langage
  (erreurs et documentation dans tous les éditeurs), un gestionnaire de
  paquets, et un bac à sable en ligne (l'interpréteur compilé en WebAssembly)
  pour essayer Rythme sans rien installer.
- **Langues.** Les messages existent en anglais et en français ; ajouter une
  langue consiste à traduire les textes de `src/`. Les contributions sont
  bienvenues.
- Les nombres sont sur 64 bits : les entiers s'arrêtent avec une erreur
  au-delà de ±9,2 × 10¹⁸ (utilisez les nombres décimaux au-delà).

## Licence

[MIT](LICENSE).
