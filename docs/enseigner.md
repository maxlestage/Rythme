# Enseigner avec Rythme

*[English version →](teaching.md)*

Rythme a été conçu pour la salle de cours : un petit langage, avec des
messages qui enseignent, pour que les étudiants passent leur temps sur les
idées plutôt que sur la syntaxe.

## Pourquoi il convient à un cours

- **Peu à apprendre avant d'écrire de vrais programmes.** 15 mots-clés, une
  seule façon de définir une fonction, une seule façon de choisir
  (`if`/`match`), pas de classes, pas de boucles, pas de modification. Toute
  la syntaxe tient sur une page (`rythme help syntax`).
- **L'interpréteur est un assistant pédagogique.** Les erreurs pointent
  l'endroit exact, expliquent le problème et proposent une correction. Les
  réflexes venus d'autres langages (`for`, `return`, `x += 1`, `null`...)
  reçoivent une explication de la façon fonctionnelle de faire.
  `rythme explain E0xx` donne la version longue avec des exemples.
  `rythme check` trouve les problèmes sans lancer le programme, y compris
  les opérations qui échoueraient à coup sûr, comme `"âge : " + 36`.
- **Penser fonctionnel dès le premier jour.** Immuabilité, fonctions comme
  valeurs, récursion, filtrage par motifs, types algébriques, erreurs comme
  valeurs (`Option`, `Result`) : les concepts de Haskell, OCaml, Elm, Elixir
  ou Rust, sans leur syntaxe ni un système de types à combattre le premier
  jour.
- **Les tests sont intégrés.** `test "..." = assert_eq(obtenu, attendu)` à
  côté du code, `rythme test fichier.ry` pour les lancer. Les exercices
  peuvent être fournis avec leurs tests.
- **Bilingue.** Chaque message existe en français et en anglais (`--lang`).
- **Facile à installer, gratuit, ouvert.** Un seul programme, aucune
  dépendance, licence MIT, fonctionne sous Linux, macOS et Windows.

## Un cours possible (12 séances)

| # | Thème | Notions Rythme | Exemple |
|---|-------|----------------|---------|
| 1 | Expressions et valeurs | nombres, textes, `print`, `{...}` dans les textes, le mode interactif | [`bonjour.ry`](../examples/bonjour.ry) |
| 2 | Fonctions | `fn f(x) = ...`, blocs avec `end`, appels, `help(f)` | |
| 3 | Choisir | `if ... then ... else`, booléens, `and`/`or`/`not` | |
| 4 | Récursion | cas de base, définitions par cas `fn fib(0) = 0`, gardes | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Listes | littéraux, intervalles `1..10`, motifs `[x, ..reste]` | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Fonctions d'ordre supérieur | `map`, `filter`, `fold`, fonctions anonymes, `|>` | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Enregistrements | `{nom: ...}`, accès aux champs, `{..r, x: 1}` | [`students.ry`](../examples/students.ry) |
| 8 | Vos propres types | `type`, constructeurs, `match`, cas oubliés (W002) | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Types récursifs | arbres, récursion structurelle | [`tree.ry`](../examples/tree.ry) |
| 10 | Les erreurs comme valeurs | `Option`, `Result`, `?`, `unwrap_or` | [`results.ry`](../examples/results.ry) |
| 11 | Tests et modules | `test`, `assert_eq`, `use` | [`testing.ry`](../examples/testing.ry) |
| 12 | Projet | traitement de texte, analyse de données, petit jeu | [`words.ry`](../examples/words.ry) |

La récursion terminale (séance 4 ou 9) mérite un moment : l'erreur E014
(« trop d'appels imbriqués ») l'explique, et `rythme explain E014` montre la
technique de l'accumulateur.

## Des exercices corrigés automatiquement

Donnez aux étudiants un fichier où les tests sont déjà écrits :

```rythme
## Renvoie le plus grand élément d'une liste non vide.
fn plus_grand(liste) = fail("à faire")

test "un élément" = assert_eq(plus_grand([7]), 7)
test "plusieurs" = assert_eq(plus_grand([3, 9, 2]), 9)
test "négatifs" = assert_eq(plus_grand([-3, -1, -2]), -1)
```

`rythme test exercice.ry` indique ce qui réussit et, pour chaque échec, les
valeurs attendue et obtenue.

Trente exercices de ce genre sont prêts : [**les exercices**](exercises/README.fr.md),
gradués de ★☆☆ à ★★★ et qui suivent le cours ci-dessus, chacun avec un
énoncé, des indices et des tests, en français et en anglais, avec des
solutions de référence pour les enseignants. Ils sont intégrés à `rythme` :

```sh
rythme exercise list                       # les exercices
rythme exercise start 7                    # copie 07-recursivite.ry dans le dossier actuel
rythme exercise check 07-recursivite.ry    # lance ses tests, avec des conseils, et propose le suivant
```

## Conseils pratiques

- Commencez par le mode interactif (`rythme`) : chaque ligne affiche sa
  valeur. `:doc map` affiche la documentation d'une fonction, Tab complète
  les noms.
- `rythme learn` est une visite guidée que les étudiants peuvent suivre seuls.
- `rythme help builtins` liste toutes les fonctions intégrées par catégorie.
- Installez l'[extension VS Code](../editors/vscode) : les couleurs, des
  extraits de code, et les problèmes affichés pendant la frappe, avec la
  documentation sous la souris. Les autres éditeurs peuvent aussi utiliser
  le serveur de langage ([`editors/README.fr.md`](../editors/README.fr.md)).
- `rythme fmt` donne à tous les fichiers la même mise en forme : les
  étudiants peuvent le lancer (ou `rythme fmt --check .`) avant de rendre
  leur travail.
- `rythme etapes fib.ry "fib(4)"` montre un calcul étape par étape, dans le
  modèle par substitution (`fib(4)` → `fib(3) + fib(2)` → ...) : la
  récursion, les accumulateurs et les appels terminaux deviennent visibles.
  `:etapes` le fait dans le mode interactif, et le bouton **Étapes** dans le
  playground.
- Rien à installer sur les ordinateurs des étudiants ? Le playground du
  [site](https://maxlestage.github.io/Rythme/) fait tourner le vrai
  interpréteur dans le navigateur.
