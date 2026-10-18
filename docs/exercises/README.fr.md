# Exercices

*[English version →](README.md)*

Trente exercices gradués pour un premier cours de programmation, des
premières valeurs à la récursivité, aux listes, aux tuyaux, aux
enregistrements, aux types et à `match`, à `Option` et `Result`, et aux
tests. Chaque exercice est un fichier `.ry`, en [français](fr) et en
[anglais](en), avec :

- un **énoncé** et des **indices**, en commentaires en haut ;
- les **fonctions à écrire**, dont le corps est `fail("à faire")` pour
  l'instant ;
- des **tests** que `rythme test` lance : l'exercice est réussi quand ils
  passent tous.

Les niveaux vont de ★☆☆ (quelques lignes, une idée nouvelle) à ★★★ (plusieurs
fonctions qui travaillent ensemble).

## Pour les élèves

```sh
rythme exercise list                     # les exercices
rythme exercise start 1                  # copie 01-valeurs.ry dans le dossier actuel
rythme exercise check 01-valeurs.ry      # lance ses tests et dit quoi faire ensuite
```

Ouvrez le fichier, lisez l'énoncé, remplacez chaque `fail("à faire")` par
votre code, et vérifiez de nouveau, jusqu'à ce que tous les tests passent :
`rythme exercise check` propose alors l'exercice suivant. Ne modifiez pas les
tests. `rythme test 01-valeurs.ry` lance les mêmes tests, et
`rythme check 01-valeurs.ry` trouve les erreurs sans rien lancer.
`rythme exercise` donne la version française de chaque exercice quand le
système est en français, ou avec `--lang fr`
(`rythme --lang fr exercise start 1`).

Les exercices sont intégrés à `rythme` : rien d'autre à télécharger. Sans
rien installer, copiez un exercice dans le playground du
[site](https://maxlestage.github.io/Rythme/) et appuyez sur *Tester*.

## Les exercices

| # | Exercice | Idées | Niveau |
|---|----------|-------|--------|
| 01 | [Premières valeurs](fr/01-valeurs.ry) | noms, nombres, `{...}` dans les textes | ★☆☆ |
| 02 | [Textes](fr/02-textes.ry) | fonctions, `upper`, `+` sur les textes, `texte[0]` | ★☆☆ |
| 03 | [Calculs](fr/03-calculs.ry) | `/`, `//`, `%`, `sqrt`, Int et Float | ★☆☆ |
| 04 | [Choisir avec `if`](fr/04-choisir.ry) | `if ... then ... else`, `else if` | ★☆☆ |
| 05 | [Vrai ou faux](fr/05-logique.ry) | comparaisons, `and`, `or`, `not` | ★☆☆ |
| 06 | [Une définition par cas](fr/06-cas.ry) | définitions par cas, gardes, `_` | ★☆☆ |
| 07 | [Récursivité](fr/07-recursivite.ry) | cas de base et cas récursif | ★★☆ |
| 08 | [Les chiffres d'un nombre](fr/08-chiffres.ry) | récursivité sur les nombres, une fonction auxiliaire | ★★☆ |
| 09 | [Accumulateurs et appels terminaux](fr/09-accumulateurs.ry) | accumulateurs, appels terminaux, E014 | ★★☆ |
| 10 | [Diviseurs et nombres premiers](fr/10-diviseurs.ry) | Euclide, essayer des candidats | ★★☆ |
| 11 | [Démonter une liste](fr/11-motifs-de-listes.ry) | `[]`, `[x, ..reste]`, cas oubliés (W002) | ★★☆ |
| 12 | [Construire des listes](fr/12-construire-des-listes.ry) | `[x, ..reste]` pour construire, les valeurs ne changent pas | ★★☆ |
| 13 | [Les fonctions de listes](fr/13-bibliotheque.ry) | intervalles, `map`, `filter`, `sum`, `sort`, `take` | ★★☆ |
| 14 | [Fonctions anonymes](fr/14-fonctions-anonymes.ry) | `fn(x) = ...`, `all`, `count` | ★★☆ |
| 15 | [Les fonctions sont des valeurs](fr/15-fonctions-valeurs.ry) | fonctions en argument et en résultat, fermetures | ★★☆ |
| 16 | [Réduire une liste à une valeur](fr/16-fold.ry) | `fold` | ★★☆ |
| 17 | [Les tuyaux `\|>`](fr/17-pipelines.ry) | `\|>`, `split`, `join` | ★★☆ |
| 18 | [Travailler sur des textes](fr/18-traitement-de-textes.ry) | `chars`, `lower`, `replace`, `capitalize` | ★★☆ |
| 19 | [Trier](fr/19-tris.ry) | tri par insertion, tri fusion | ★★★ |
| 20 | [Enregistrements](fr/20-enregistrements.ry) | `{nom: ...}`, champs, `{..e, x: 1}` | ★★☆ |
| 21 | [Listes d'enregistrements](fr/21-listes-d-enregistrements.ry) | `sort_by`, `max_by`, `group_by`, `map_values` | ★★☆ |
| 22 | [Vos propres types](fr/22-types.ry) | `type`, constructeurs, une définition par constructeur | ★★☆ |
| 23 | [Choisir avec `match`](fr/23-match.ry) | `match` sur des constructeurs, des listes et des enregistrements | ★★☆ |
| 24 | [Arbres](fr/24-arbres.ry) | types récursifs, arbres de recherche | ★★★ |
| 25 | [Une petite calculatrice](fr/25-expressions.ry) | un type pour les expressions, évaluer, simplifier | ★★★ |
| 26 | [Une valeur qui peut manquer : Option](fr/26-option.ry) | `Some`, `None`, `find`, `match` | ★★☆ |
| 27 | [Une opération qui peut échouer : Result](fr/27-result.ry) | `Ok`, `Err`, `parse_int`, gardes | ★★☆ |
| 28 | [Transmettre les erreurs avec `?`](fr/28-point-d-interrogation.ry) | `?` | ★★★ |
| 29 | [Un petit projet : compter les mots](fr/29-compter-les-mots.ry) | tout ensemble | ★★★ |
| 30 | [Les tests trouvent les bugs](fr/30-tests.ry) | lire les échecs des tests, corriger des bugs, écrire un test | ★★★ |

## Pour les enseignants

- Les exercices suivent le [plan de cours](../enseigner.md#un-cours-possible-12-séances) :
  01–06 pour les séances 1 à 3, 07–10 pour la récursivité, 11–19 pour les
  listes et les fonctions d'ordre supérieur, 20–21 pour les enregistrements,
  22–25 pour les types, 26–28 pour les erreurs comme valeurs, 29–30 pour les
  tests et un petit projet.
- Les solutions de référence sont dans le dossier caché
  [`.solutions/`](.solutions) (`fr/` et `en/`), avec les mêmes tests : les
  tests de Rythme vérifient que chaque solution passe tous les tests de son
  exercice, et que chaque exercice échoue à ses tests tant qu'il n'est pas
  résolu.
- Vos propres exercices peuvent suivre le même modèle : un énoncé et des
  indices en commentaires, `fail("à faire")` dans les fonctions, et les tests
  après une ligne `# --- Tests (ne les modifiez pas) ---`.
  `rythme exercise check` marche sur tout fichier qui a des tests ; il
  remarque aussi quand les tests d'un exercice intégré ont été modifiés.
