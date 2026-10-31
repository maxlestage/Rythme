# Enseigner avec Rythme

*[English version →](teaching.md) · [En español →](teaching.es.md) · [Auf Deutsch →](teaching.de.md) · [Em português →](teaching.pt.md) · [中文 →](teaching.zh.md)*

Rythme a été conçu pour la salle de cours : un petit langage, avec des
messages qui enseignent, pour que les étudiants passent leur temps sur les
idées plutôt que sur la syntaxe. Ce guide couvre ce dont un cours a
besoin : un plan, des exercices corrigés et notés automatiquement, des
façons de montrer comment le code s'exécute, et une politique pour les
assistants d'IA.

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
  jour. Les types sont facultatifs : le vérificateur les trouve seul, et
  quand le cours y arrive, les étudiants les écrivent
  (`fn aire(l: Int, h: Int) -> Int`, `type Arbre(a)`) et le vérificateur
  les contrôle.
- **Les tests sont intégrés.** `test "..." = assert_eq(obtenu, attendu)` à
  côté du code, `rythme test fichier.ry` pour les lancer. Les exercices
  peuvent être fournis avec leurs tests.
- **Une bibliothèque que les étudiants reconnaissent.** Textes, listes,
  dictionnaires, dates, CSV, expressions régulières, et vecteurs,
  statistiques et pentes pour une unité sur l'apprentissage automatique
  (`rythme help ml`). Les étudiants qui connaissent Ruby ou Python trouvent
  beaucoup de fonctions sous les noms qu'ils connaissent (`upcase`,
  `strip`, `each_slice`...) : voir `rythme help ruby` et
  [la bibliothèque, comparée](stdlib-audit.md) (en anglais).
- **Dans la langue des étudiants**, et **facile à installer, gratuit,
  ouvert** : un seul programme, aucune dépendance, licence MIT, pour Linux,
  macOS et Windows, ou rien à installer du tout avec le playground du site.

## Un cours possible (12 séances)

| # | Thème | Notions Rythme | Exercices | Exemple |
|---|-------|----------------|-----------|---------|
| 1 | Expressions et valeurs | nombres, textes, `print`, `{...}` dans les textes, le mode interactif | 01, 03 | [`bonjour.ry`](../examples/bonjour.ry) |
| 2 | Fonctions | `fn f(x) = ...`, blocs avec `end`, appels, `help(f)` | 02 | |
| 3 | Choisir | `if ... then ... else`, booléens, `and`/`or`/`not` | 04–06 | |
| 4 | Récursion | cas de base, définitions par cas `fn fib(0) = 0`, gardes | 07–10 | [`fibonacci.ry`](../examples/fibonacci.ry) |
| 5 | Listes | littéraux, intervalles `1..10`, motifs `[x, ..reste]` | 11–13 | [`quicksort.ry`](../examples/quicksort.ry) |
| 6 | Fonctions d'ordre supérieur | `map`, `filter`, `fold`, fonctions anonymes, `\|>` | 14–19 | [`fizzbuzz.ry`](../examples/fizzbuzz.ry) |
| 7 | Enregistrements | `{nom: ...}`, accès aux champs, `{..r, x: 1}` | 20–21 | [`students.ry`](../examples/students.ry) |
| 8 | Vos propres types | `type`, constructeurs, `match`, cas oubliés (W002), types écrits dans le code | 22–23 | [`shapes.ry`](../examples/shapes.ry) |
| 9 | Types récursifs | arbres, récursion structurelle, types à paramètres (`type Arbre(a)`) | 24–25 | [`tree.ry`](../examples/tree.ry) |
| 10 | Les erreurs comme valeurs | `Option`, `Result`, `?`, `unwrap_or` | 26–28 | [`results.ry`](../examples/results.ry) |
| 11 | Tests et modules | `test`, `assert_eq`, `use` | 30 | [`testing.ry`](../examples/testing.ry) |
| 12 | Projet | traitement de texte, analyse de données, petit jeu | 29 | [`words.ry`](../examples/words.ry) |

La récursion terminale (séance 4 ou 9) mérite un moment : l'erreur E014
(« trop d'appels imbriqués ») l'explique, et `rythme explain E014` montre la
technique de l'accumulateur.

Après la séance 7, deux ou trois séances peuvent former une **unité sur
l'intelligence artificielle** : vecteurs, statistiques, un neurone
artificiel, la descente de gradient, les plus proches voisins et le
minimax (exercices 31 à 36, `rythme help ml`, et les exemples
[`perceptron.ry`](../examples/perceptron.ry) et [`knn.ry`](../examples/knn.ry)).

## Des exercices corrigés automatiquement

Donnez aux étudiants un fichier où les tests sont déjà écrits, et chaque
fonction à écrire sous la forme `fail("à faire")`. En voici un qu'un
étudiant a complété, avec une erreur classique :

```rythme
## Renvoie le plus grand élément d'une liste non vide.
fn plus_grand(liste) = fold(liste, 0, max)

test "un élément" = assert_eq(plus_grand([7]), 7)
test "plusieurs" = assert_eq(plus_grand([3, 9, 2]), 9)
test "négatifs" = assert_eq(plus_grand([-3, -1, -2]), -1)
```

`rythme test plus_grand.ry` indique ce qui réussit et, pour chaque échec,
les valeurs attendue et obtenue :

```text
Exécution de 3 test(s)
  ✓ un élément
  ✓ plusieurs
  ✗ négatifs
erreur[E015]: assertion échouée : les valeurs sont différentes
 --> plus_grand.ry:6:19
  |
6 | test "négatifs" = assert_eq(plus_grand([-3, -1, -2]), -1)
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: obtenu :  0
      attendu : -1
  = en savoir plus : `rythme explain E015`

3 test(s) : 2 réussi(s), 1 échoué(s) (1 ms)
```

Trente-six exercices de ce genre sont prêts : [**les exercices**](exercises/README.fr.md),
gradués de ★☆☆ à ★★★ et qui suivent le cours ci-dessus, chacun avec un
énoncé, des indices et des tests, dans chaque langue des messages, avec des
solutions de référence pour les enseignants. Ils sont intégrés à
`rythme` :

```sh
rythme exercise list                       # les exercices
rythme exercise start 7                    # copie 07-recursivite.ry dans le dossier actuel
rythme exercise check 07-recursivite.ry    # lance ses tests, avec des conseils, et propose le suivant
```

`rythme exercise check` dit aussi quand les tests d'un exercice intégré ont
été modifiés : l'exercice n'est résolu que quand ses propres tests
réussissent. Vos propres exercices peuvent suivre le même modèle (un
énoncé et des indices en commentaires, `fail("à faire")` dans les
fonctions, les tests après une ligne
`# --- Tests (ne les modifiez pas) ---`) ; `rythme exercise check`
fonctionne sur tout fichier qui a des tests.

## Noter avec `--format json`

`rythme test` accepte un dossier : avec un fichier (ou un dossier) par
étudiant, une seule commande lance tous les tests de la classe.

```sh
rythme test --sandbox --format json rendus/ > resultats.json
```

- **`--format json`** affiche un seul document JSON au lieu des messages
  pour les personnes : pour chaque fichier, `tests`, `failed` et `broken`
  (un fichier qui ne compile pas, ou s'arrête avant ses tests) ; pour
  chaque test, son `name`, son `file`, `passed` et `failure` ; puis un
  `summary`. Les clés ne changent jamais ([le schéma](rythme-output.schema.json)) ;
  les messages suivent `--lang`.
- **`--sandbox`** tient les programmes à l'écart des fichiers, des
  dossiers, des variables d'environnement et du réseau. Il n'y a pas de
  limite de temps : lancez la commande sous une limite (sous Linux,
  `timeout 60 rythme test ...`).
- **Les étudiants peuvent modifier les tests** de leur fichier. Pour une
  note, lancez votre propre copie : `rythme exercise check` dit quand les
  tests d'un exercice intégré ont été modifiés, et les solutions de
  référence sont publiques (dans le dépôt) : pour un travail noté, écrivez
  vos propres exercices.
- `rythme check --format json` ajoute les avertissements, chacun avec son
  code stable (W001 un nom inutilisé, W002 un cas oublié...), et
  `rythme fmt --check` dit si le code a la mise en forme officielle.

Rythme lit lui-même le JSON. Ce programme transforme les résultats en
notes :

```rythme
# notes.ry
fn note(f) = if f.broken then "{f.file} : ne fonctionne pas" else "{f.file} : {f.tests - f.failed}/{f.tests}"

resultats = read_file(args()[0]) |> unwrap |> parse_json |> unwrap
each(resultats.files, fn(f) = print(note(f)))
```

`rythme notes.ry resultats.json` affiche :

```text
rendus/ada.ry : 3/3
rendus/alan.ry : 2/3
rendus/grace.ry : ne fonctionne pas
```

## Montrer comment le code s'exécute

`rythme etapes` montre un calcul étape par étape, dans le modèle par
substitution : chaque appel est remplacé par le corps de sa fonction,
jusqu'à ce qu'il ne reste que le résultat. La récursion, les accumulateurs
et les appels terminaux deviennent visibles.

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)

fib(4)
```

`rythme etapes fib.ry` affiche :

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

`rythme etapes fib.ry "fib(6)"` montre une autre expression, `:etapes fib(6)`
le fait dans le mode interactif, et le bouton **Étapes** dans le
playground, où *Partager* crée un lien qui s'ouvre sur la même étape : un
lien à projeter, ou à envoyer à la classe.

`rythme debug fichier.ry` met un programme en pause là où vous le demandez
(`break 12`, `break fib`) et montre les valeurs de ses noms. `where` liste
les appels en cours avec leurs arguments, et dit combien d'appels de la
même fonction attendent : la pile d'une récursion, en direct. Un appel en
position terminale dit qu'il a pris la place de son appelant. Dans
VS Code, `F5` sur un fichier `.ry` lance le même débogueur, avec les points
d'arrêt dans la marge. Le [manuel](reference.fr.md#79-le-débogueur--rythme-debug)
montre une session complète.

## Sans rien installer : le playground

Le playground du [site](https://maxlestage.github.io/Rythme/) fait tourner
le vrai interpréteur dans le navigateur, avec les mêmes messages et la même
aide : les étudiants peuvent commencer sur n'importe quel ordinateur,
tablette ou téléphone.

- **Lancer**, **Vérifier**, **Tester**, **Formater** et **Étapes**
  fonctionnent comme les commandes ; collez un exercice et appuyez sur
  **Tester**.
- **Partager** crée un lien qui contient le programme lui-même (rien n'est
  envoyé à un serveur) : distribuez un point de départ, ou le programme du
  cours. **Graine** donne à chaque étudiant les mêmes tirages de `random`.
- Installé comme une application (**Installer l'app** dans le menu, ou
  *Sur l'écran d'accueil* sur iPhone et iPad), le site et son playground
  fonctionnent hors ligne, ce qui aide dans une salle au réseau faible. Le
  code en cours d'écriture reste dans le navigateur.
- Dans le navigateur, les fichiers ne peuvent pas être lus ni écrits,
  `input()` renvoie un texte vide, et les appels imbriqués s'arrêtent à 600
  (les appels terminaux n'ont pas de limite).

## Dans la langue des étudiants

- Chaque message, sujet d'aide, explication et page de `rythme learn`
  existe en français, en anglais, en espagnol, en allemand, en portugais et
  en chinois. Rythme suit la langue du système ; `--lang es` ou la variable
  `RYTHME_LANG=es` en choisissent une autre.
- Les mots-clés, les noms des fonctions et les commandes restent les mêmes
  dans toutes les langues : un programme écrit dans une classe fonctionne
  partout, et `rythme help` accepte les noms des sujets dans chaque langue
  (`rythme help listes`).
- Les exercices suivent aussi la langue :
  `rythme --lang de exercise start 7` écrit `07-rekursion.ry`, en allemand.
- Le site, son playground et ce guide existent dans les mêmes langues.

## Les assistants d'IA en cours

Les étudiants ont des assistants sous la main. Rythme n'essaie pas de les
détecter ; voici ce qu'il offre, et une politique qu'un cours peut adapter.

**Ce que Rythme offre.** Les assistants ont été entraînés avant que Rythme
existe : livrés à eux-mêmes, ils écrivent des habitudes de Python ou de
JavaScript (`for`, `return`, `x += 1`), que Rythme refuse avec un message
qui dit quoi écrire. Avec `rythme help llms`, tout le langage en un texte
(aussi sur la page *IA* du site), ils écrivent du Rythme correct.
`rythme mcp` permet à un assistant de lancer, vérifier et tester du code
lui-même, dans un bac à sable. Dans le playground, *Copier pour l'IA*
copie le programme et sa sortie ou ses erreurs, prêts pour une
conversation. `rythme fix` applique les corrections certaines
(`&&` → `and`, `let x = 3` → `x = 3`), et `rythme fix --dry-run` se
contente de les montrer.

**Une politique possible.**

- **Premières semaines : les étudiants écrivent le code.** Il s'agit de
  construire les réflexes, et les messages et `rythme explain` expliquent
  déjà chaque erreur. Un assistant peut expliquer un message ou une idée,
  pas écrire la solution.
- **`rythme fix` une fois le message lu** : il ne change que ce qui est
  certain, la correction même que le message propose.
- **Projets : autorisé, et dit.** Donnez `rythme help llms` à l'assistant,
  demandez aux étudiants de dire à quoi il leur a servi, et d'expliquer
  leur code.
- **Évaluer la compréhension, pas seulement les résultats.** Les tests
  vérifient qu'un programme fonctionne, pas qui l'a écrit. Ajoutez de
  courtes vérifications en classe : prévoir les étapes de `rythme etapes`,
  la sortie d'un programme, ou la correction d'un message, sans
  ordinateur ; ou un exercice fait sur place dans le playground.
- **Pour votre propre travail**, un assistant à qui l'on a donné
  `rythme help llms` peut ébaucher de nouveaux exercices dans le format
  ci-dessus. Vérifiez chacun avec une solution à vous et `rythme test`
  avant de le distribuer.

## Conseils pratiques

- Commencez par le mode interactif (`rythme`) : chaque ligne affiche sa
  valeur. `:doc map` affiche la documentation d'une fonction, Tab complète
  les noms.
- `rythme learn` est une visite guidée que les étudiants peuvent suivre seuls.
- `rythme help builtins` liste toutes les fonctions intégrées par
  catégorie, et chaque commande répond à `--help` (`rythme test --help`).
  `rythme completions bash` (ou `zsh`, `fish`, `powershell`) complète les
  commandes dans le terminal.
- Installez l'[extension VS Code](../editors/vscode) : les couleurs, des
  extraits de code, les problèmes affichés pendant la frappe avec leurs
  corrections rapides, la documentation sous la souris, et le débogueur.
  Les autres éditeurs peuvent aussi utiliser le serveur de langage
  ([`editors/README.fr.md`](../editors/README.fr.md)).
- `rythme fmt` donne à tous les fichiers la même mise en forme : les
  étudiants peuvent le lancer (ou `rythme fmt --check .`) avant de rendre
  leur travail.
- Pour un projet plus grand, `rythme new NOM` crée un dossier avec `src/`,
  `tests/` et `rythme.toml` ; `rythme test` seul lance alors tous les tests
  du projet, et `rythme add` ajoute un paquet de
  l'[index des paquets](reference.fr.md#711-lindex-des-paquets).
