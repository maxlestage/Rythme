# Manuel de référence de Rythme

*[English version →](reference.md)*

Ce manuel décrit le langage Rythme et ses outils, tels que les réalise
l'interpréteur `rythme`, version 0.1.0. Il se veut complet et précis :
quelqu'un qui voudrait écrire une seconde implémentation de Rythme doit y
trouver toutes les règles. Il se veut aussi lisible par des étudiants :
chaque règle est accompagnée d'un petit programme et de ce qu'il affiche
exactement.

Pour une entrée en matière plus douce, lancez `rythme learn` (une visite
guidée), lisez le [README](../README.fr.md) et les [exemples](../examples),
ou essayez le playground du [site](https://maxlestage.github.io/Rythme/).

**Lire les exemples.** Chaque programme de ce manuel a été enregistré dans un
fichier `.ry` et lancé avec `rythme --lang fr` ; le bloc qui le suit est ce
qu'il affiche réellement, messages d'erreur compris, suivi du code de sortie
quand il compte. Si une règle et l'interpréteur ne sont pas d'accord, c'est
l'interpréteur qui a raison et ce manuel qui a un bug.

## Sommaire

1. [Structure lexicale](#1-structure-lexicale)
2. [Valeurs et types](#2-valeurs-et-types)
3. [Expressions](#3-expressions)
4. [Définitions](#4-définitions)
5. [Erreurs](#5-erreurs)
6. [Bibliothèque standard](#6-bibliothèque-standard)
7. [Outils](#7-outils)
- [Annexe A. Grammaire](#annexe-a-grammaire)
- [Annexe B. Limites de l'implémentation](#annexe-b-limites-de-limplémentation)

---

## 1. Structure lexicale

### 1.1 Fichiers source

Un programme Rythme est un fichier texte encodé en UTF-8. Par convention, son
nom finit par `.ry` ; `rythme` lance aussi les autres fichiers, après un
avertissement. Une marque d'ordre des octets (BOM) au tout début du fichier
est ignorée. Les lignes finissent par `\n` ou `\r\n` : un `\r` compte comme
une espace, et un `\r\n` à l'intérieur d'un texte devient `\n`.

Un fichier est une suite d'*éléments* séparés par des retours à la ligne. Un
élément est une définition (`nom = ...`, `fn`, `type`, `test`, `use`) ou une
expression.

### 1.2 Commentaires

`#` commence un commentaire qui va jusqu'à la fin de la ligne. Dans un texte,
`#` est un caractère comme un autre.

`##` commence un **commentaire de documentation** quand c'est la première
chose sur la ligne (seulement des espaces avant). Les lignes `##`
consécutives sont réunies, une espace après `##` est retirée, et le texte
documente la définition `fn` qui suit : `help(f)` dans un programme et
`:doc f` dans le mode interactif l'affichent. Une ligne vide entre les lignes `##` et
la définition les détache ; des lignes de commentaire ordinaire `#`, non. Un
`##` placé après du code sur la même ligne est un commentaire ordinaire. Les
commentaires de documentation écrits au-dessus d'un `type` documentent le
type : `help` les affiche pour chacun de ses constructeurs (voir
[4.7](#47-types)).

```rythme
# Un commentaire ordinaire : de `#` jusqu'à la fin de la ligne.
reponse = 6 * 7  # un commentaire peut suivre du code

## Aire d'un rectangle.
## Un commentaire de documentation peut tenir sur plusieurs lignes.
fn aire(largeur, hauteur) = largeur * hauteur

help(aire)
```

```text
aire(largeur, hauteur)
  Aire d'un rectangle.
  Un commentaire de documentation peut tenir sur plusieurs lignes.
  définie en 01-commentaires.ry:6:4
```

### 1.3 Espaces et indentation

Les espaces et les tabulations séparent les éléments du code et sont ignorés
pour le reste. **L'indentation n'a aucun sens** : les blocs se ferment par le
mot-clé `end`, pas par l'indentation. Les exemples utilisent deux espaces par
niveau, c'est la convention.

### 1.4 Noms

Un nom commence par une lettre ou `_`, suivie de lettres, de chiffres et de
`_`. « Lettre » et « chiffre » s'entendent au sens d'Unicode : les lettres
accentuées et les autres alphabets sont permis. Majuscules et minuscules sont
différentes.

Le **premier caractère** décide de la sorte de nom :

- une **majuscule** : un type ou un constructeur (`Forme`, `Cercle`, `Some`) ;
- tout le reste (une minuscule, `_`, ou une lettre sans casse comme 日) : un
  nom ordinaire, pour les valeurs, les fonctions, les paramètres et les
  champs.

`_` seul est le *joker* : il ne peut apparaître que dans les motifs (`_`
utilisé comme valeur est l'erreur E001). Les noms qui commencent par `_` ne
sont jamais signalés comme inutilisés (W001). Un nombre immédiatement suivi
d'une lettre, comme `2x`, est une erreur (E018) : Rythme ne multiplie jamais
implicitement.

```rythme
prénom = "Ada"           # les lettres accentuées sont permises
_brouillon = 0           # un nom peut commencer par `_`
élément2 = 2             # des chiffres après le premier caractère
日本 = "Japon"           # n'importe quel caractère alphabétique
type Reponse = Oui | Non # une majuscule au début : un type ou un constructeur
e = {type: "un mot-clé comme nom de champ", "nom complet": "un texte comme nom de champ"}
print(prénom, élément2, 日本, Oui, e.type, e["nom complet"])
```

```text
Ada 2 Japon Oui un mot-clé comme nom de champ un texte comme nom de champ
```

Les mots-clés et les fonctions intégrées sont en anglais, mais vos propres
noms peuvent être en français, accents compris.

### 1.5 Mots-clés et mots réservés

Rythme a 15 mots-clés :

```text
and  do  else  end  false  fn  if  match  not  or  test  then  true  type  use
```

Ils ne peuvent pas servir de noms. Ils peuvent servir de **noms de champs**,
après un point ou devant `:` dans un enregistrement (`e.type`, `{type: 1}`),
mais pas dans la forme courte `{type}`.

Les mots suivants, venus d'autres langages, sont **réservés** : en utiliser
un comme nom est une erreur (E001) dont le message explique comment faire la
même chose en Rythme. Comme les mots-clés, ils sont permis comme noms de
champs.

```text
for  while  return  break  continue  let  var  const  def  function  class
elif  elsif  import  try  catch  except  raise  throw  null  nil  lambda  switch
```

### 1.6 Nombres

Les **nombres entiers (Int)** s'écrivent en décimal (`42`), en hexadécimal
après `0x` (`0xFF`, chiffres en majuscules ou minuscules) ou en binaire après
`0b` (`0b1010`). Les préfixes s'écrivent en minuscules ; il n'y a pas de
notation octale. `_` peut séparer les chiffres partout après le premier
(`1_000_000`, `0xFF_FF`). Un littéral doit tenir sur 64 bits :
`9223372036854775807` (`0x7fffffffffffffff`) est le plus grand ;
`9223372036854775808` n'est accepté qu'en décimal, juste après un signe moins
(c'est le plus petit Int). Un littéral plus grand est l'erreur E012, dans
toutes les bases ; un littéral hexadécimal ou binaire avec un mauvais chiffre
ou sans chiffre (`0b12`, `0x`) est l'erreur E018.

Les **nombres décimaux (Float)** ont des chiffres des deux côtés du point
(`3.14`, `2.0`), un exposant (`1e3`, `1.5e-3`, `6.02E23`), ou les deux. `.5`
et `5.` ne sont pas des nombres ; `5.max(3)` est un appel de méthode sur
l'Int 5 (voir 3.8). `_` est permis dans la partie entière et la partie
décimale, pas dans l'exposant. Un littéral Float trop grand pour être
représenté devient `inf`.

Il n'y a pas de littéraux négatifs : `-7` est l'opérateur `-` appliqué à `7`.

```rythme
print(42, -7, 1_000_000, 0xFF, 0b1010)
print(3.14, 2.0, 1e3, 1.5e-3, 6.02E23)
print(9223372036854775807, -9223372036854775808)
```

```text
42 -7 1000000 255 10
3.14 2.0 1000.0 0.0015 6.02e23
9223372036854775807 -9223372036854775808
```

### 1.7 Textes

Un texte s'écrit entre guillemets droits `"`. Il peut tenir sur plusieurs
lignes ; les retours à la ligne font alors partie du texte. Les apostrophes,
les accents graves et les guillemets typographiques (« », “ ”) sont des
erreurs (E018). Un texte jamais fermé est l'erreur E002.

Séquences d'échappement :

| Écrire | Pour obtenir |
|--------|--------------|
| `\n` | un retour à la ligne |
| `\t` | une tabulation |
| `\r` | un retour chariot |
| `\0` | le caractère U+0000 |
| `\\` | une barre oblique inverse |
| `\"` | un guillemet |
| `\{` et `\}` | des accolades |
| `\u{e9}` | le caractère de ce code hexadécimal (ici é) |

Toute autre séquence `\` est l'erreur E018.

**Insertion.** `{expression}` dans un texte insère la valeur de
l'expression, affichée comme le ferait `print` (les textes sans guillemets).
Toute expression est permise, enregistrements et autres textes compris ; les
accolades qu'elle contient sont appariées, et les retours à la ligne entre
les accolades sont ignorés. Un `{}` vide est une erreur (E001). Pour écrire
une `{`, utilisez `\{`. Une `}` qui ne ferme pas d'insertion est un caractère
ordinaire (`\}` est aussi accepté).

Les textes utilisés comme motif, comme nom de test ou comme chemin de `use`
doivent être des textes simples, sans insertion.

```rythme
nom = "Ada"
print("Bonjour, {nom} ! 2 + 2 = {2 + 2}")
print("tab :\t| guillemet : \" | barre : \\ | accolades : \{ et }")
print("\u{e9}t\u{e9} \u{1F3B5}")
print("un texte peut tenir
sur plusieurs lignes")
print("{ {x: 1}.x } et {"imbriqué {nom}"}")
```

```text
Bonjour, Ada ! 2 + 2 = 4
tab :	| guillemet : " | barre : \ | accolades : { et }
été 🎵
un texte peut tenir
sur plusieurs lignes
1 et imbriqué Ada
```

### 1.8 Opérateurs et ponctuation

```text
+  -  *  /  //  %  **  ==  !=  <  <=  >  >=  |>  ..  .  ,  :  =  ->  ?  |
(  )  [  ]  {  }  ;
```

Certains symboles d'autres langages ne sont reconnus que pour expliquer quoi
écrire à la place, et sont toujours des erreurs : `=>`, `+=`, `-=`, `*=`,
`/=`, `++`, `&&`, `||`, `!` (seul), `&`, `...`, `'`, `` ` ``, `^`, `$`, et les
guillemets typographiques.

Les caractères souvent copiés depuis un document ou une page web sont aussi
des erreurs (E018), avec ce qu'il faut faire : les sosies des opérateurs
(`−` `–` `×` `÷` `≠` `≤` `≥` `→`, la ponctuation pleine chasse comme `（`)
donnent le caractère à écrire à la place ; les caractères invisibles (une
espace insécable ou une autre espace spéciale, une espace sans largeur, une
marque d'ordre des octets ailleurs qu'au début du fichier, un caractère de
contrôle) sont désignés par leur code, comme `U+00A0`, puisqu'on ne les
verrait pas dans le message.

```rythme
prix = 6 × 7
```

```text
erreur[E018]: caractère inattendu `×`
 --> 08-sosie.ry:1:10
  |
1 | prix = 6 × 7
  |          ^
  |
  = aide: écrivez plutôt `*`
  = en savoir plus : `rythme explain E018`
```

### 1.9 Retours à la ligne, `;` et lignes de continuation

Un retour à la ligne termine une instruction. `;` équivaut exactement à un
retour à la ligne : `a = 1; b = 2` fait deux instructions. Les lignes vides et
les lignes qui ne contiennent qu'un commentaire ne comptent pas.

Les retours à la ligne sont **ignorés dans `( )`, `[ ]` et `{ }`**, sauf à
l'intérieur d'un bloc (`do`, corps de fonction sur plusieurs lignes, `match`)
écrit entre ces parenthèses ou crochets, où ils comptent à nouveau.

Hors des parenthèses, une expression **continue à la ligne suivante**
quand :

1. la ligne finit par quelque chose qui attend une suite : un opérateur
   binaire (`or and == != < <= > >= |> .. + - * / // % **`), le `=` d'une
   définition, le `->` d'un cas de `match`, `then`, `else`, ou le `=` et le
   `|` d'un `type` ;
2. ou la ligne suivante commence par l'un de ces éléments :
   `|>` `.` `+` `*` `/` `//` `%` `**` `==` `!=` `<` `<=` `>` `>=` `and` `or`
   `..`, ou par `then` ou `else` (pour un `if`), ou par `|` (dans un `type`).

Une ligne qui commence par `-` n'est **pas** une continuation : c'est une
nouvelle instruction, car `-x` est aussi une expression valable à elle seule.
Un appel `(...)`, un indice `[...]` et `?` doivent commencer sur la même ligne
que ce à quoi ils s'appliquent. `not` et le `-` unaire doivent avoir leur
opérande sur la même ligne.

```rythme
total = 1 +
  2                       # la ligne finit par un opérateur : elle continue
resultat = [1, 2, 3, 4]
  |> filter(is_even)      # la ligne suivante commence par |> : elle continue
  |> map(fn(x) = x * 10)
point = {
  x: 1,                   # dans ( ), [ ] et { }, les retours à la ligne ne comptent pas
  y: 2,
}
taille = if total > 2
  then "grand"            # `then` et `else` peuvent commencer une ligne
  else "petit"
a = 1; b = 2              # `;` équivaut à un retour à la ligne
print(total, resultat, point, taille, a + b)
```

```text
3 [20, 40] {x: 1, y: 2} grand 3
```

Une ligne qui commence par `-` est une instruction à part, et sa valeur est
perdue :

```rythme
prix = 10
remise = 3
total = prix
  - remise
print(total)
```

```text
erreur[E024]: cette ligne calcule une valeur mais n'en fait rien
 --> 06-moins.ry:4:3
  |
4 |   - remise
  |   ^^^^^^^^ cette valeur est perdue
  |
  = aide: une ligne qui commence par `-` est une nouvelle instruction. Pour continuer la ligne
      précédente, terminez plutôt cette ligne par `-` :
        total = prix -
          remise
  = en savoir plus : `rythme explain E024`
```

---

## 2. Valeurs et types

Toutes les valeurs sont **immuables** : aucune opération ne modifie une
valeur ; « changer » quelque chose, c'est construire une nouvelle valeur.
Rythme est typé dynamiquement : les valeurs portent leur type, et chaque
opération vérifie les types qu'elle reçoit, sans jamais convertir une valeur
en silence.

| Type (`type_of`) | Valeurs | Exemples |
|------------------|---------|----------|
| `Int` | nombres entiers, 64 bits | `42`, `-7`, `0xFF` |
| `Float` | nombres décimaux, 64 bits | `3.14`, `2.0`, `1e3` |
| `Text` | textes | `"bonjour"`, `"total : {x}"` |
| `Bool` | `true`, `false` | |
| `Unit` | `()`, « pas de valeur intéressante » | `()` |
| `List` | suites ordonnées | `[1, 2, 3]`, `1..5`, `[]` |
| `Record` | enregistrements : des champs nommés | `{nom: "Ada", naissance: 1815}` |
| `Function` | fonctions, fonctions intégrées, constructeurs avec champs | `fn(x) = x + 1`, `print`, `Some` |
| `Option` | `Some(valeur)`, `None` | `Some(3)` |
| `Result` | `Ok(valeur)`, `Err(erreur)` | `Err("oups")` |
| vos types | les variantes d'un `type` | `Cercle(2.0)` |

```rythme
type Forme = Cercle(rayon) | Carre(cote)
fn double(x) = x * 2
exemples = [42, 3.5, "salut", true, (), [1, 2], 1..3, {a: 1}, Some(1), None, Ok(1), Cercle(2), double, print, Cercle]
print(map(exemples, type_of))
```

```text
["Int", "Float", "Text", "Bool", "Unit", "List", "List", "Record", "Option", "Option", "Result", "Forme", "Function", "Function", "Function"]
```

### 2.1 Int

Nombres entiers signés sur 64 bits, de −9223372036854775808 à
9223372036854775807. Une opération dont le résultat exact ne tient pas est
une erreur (E012) : Rythme ne « fait jamais le tour » en silence.

```rythme
grand = 9223372036854775807
print(grand)
print(grand + 1)
```

```text
9223372036854775807
erreur[E012]: le résultat est trop grand pour un nombre entier (Int)
 --> 11-entier.ry:3:13
  |
3 | print(grand + 1)
  |             ^
  |
  = aide: les nombres entiers vont de -9223372036854775808 à 9223372036854775807 ;
      pour des valeurs plus grandes, utilisez des nombres décimaux (Float), comme `2.0 ** 100`
  = en savoir plus : `rythme explain E012`
```

Code de sortie : 1.

### 2.2 Float

Nombres à virgule flottante en double précision (IEEE 754). Les résultats
trop grands deviennent `inf` ou `-inf`, les résultats indéfinis `nan` ; mais
**une division par zéro est toujours une erreur** (E008), pour les Float
comme pour les Int, et `0 ** -n` aussi, qui vaut `1 / 0 ** n`. Un calcul qui
n'a pas de valeur parmi les nombres réels est l'erreur E023 plutôt que
`nan` : un nombre négatif à une puissance fractionnaire (`(-8.0) ** 0.5`),
`sqrt` d'un nombre négatif, `log` de 0 ou d'un nombre négatif, `asin` et
`acos` hors de −1...1.

Un Float s'affiche toujours avec un point ou un exposant, pour qu'on ne le
confonde pas avec un Int : `2.0`, `0.5`, `1e16`. Les valeurs dont la valeur
absolue vaut au moins 10¹⁶ ou est inférieure à 10⁻⁵ (sauf 0) s'écrivent avec
un exposant ; les autres s'écrivent en entier, avec le moins de chiffres
possible pour retrouver le même nombre.

```rythme
print(7 / 2, 6 / 3, 0.1 + 0.2, 1 / 3)
print(1e15, 1e16, 0.00001, 0.000001)
enorme = 10.0 ** 400
print(enorme, -enorme, enorme - enorme, 2.0 ** 100)
print(1.0 / 0)
```

```text
3.5 2.0 0.30000000000000004 0.3333333333333333
1000000000000000.0 1e16 0.00001 1e-6
inf -inf nan 1.2676506002282294e30
erreur[E008]: division par zéro
 --> 12-decimal.ry:5:11
  |
5 | print(1.0 / 0)
  |           ^
  |
  = aide: vérifiez d'abord le diviseur : `if b == 0 then ... else a / b`,
      ou renvoyez un Result : `if b == 0 then Err("division par zéro") else Ok(a / b)`
  = en savoir plus : `rythme explain E008`
```

### 2.3 Text

Un texte est une suite de caractères Unicode (des points de code). `len`, les
indices et `slice` comptent des caractères, pas des octets : `len("été")`
vaut 3.

Un texte contient au plus 100 000 000 octets en UTF-8 (1 octet pour une
lettre sans accent, un chiffre ou une espace, 2 pour `é`, 4 pour un émoji) :
pour un texte sans accents, 100 000 000 caractères, comme le dit le message.
En construire un plus long, avec `+`, `{...}`, `join`, `replace` ou
`repeat`, est l'erreur E023. Cette limite, comme celle de la longueur
des listes (voir [2.6](#26-list)), arrête une valeur qui grandit sans fin
avant que la mémoire de l'ordinateur ne soit épuisée :

```rythme
fn doubler_texte(t) = doubler_texte(t + t)   # ne s'arrête jamais
doubler_texte("ab")
```

```text
erreur[E023]: ce texte serait trop grand (plus de 100000000 caractères)
 --> 13-trop-grand.ry:1:39
  |
1 | fn doubler_texte(t) = doubler_texte(t + t)   # ne s'arrête jamais
  |                                       ^
  |
  = aide: grandit-il dans une fonction qui s'appelle elle-même ? Vérifiez que la fonction atteint toujours un cas où elle s'arrête
  = appels en cours (le plus récent d'abord) :
      dans `doubler_texte` appelée en 13-trop-grand.ry:1:23
  = en savoir plus : `rythme explain E023`
```

### 2.4 Bool

`true` et `false`. Les conditions (`if`, gardes, `and`, `or`, `not`,
`filter`...) doivent être des Bool : rien n'est « vrai » ou « faux » par
défaut, donc `if 1 then ...` ou `if liste then ...` est l'erreur E005.

### 2.5 Unit

`()` est la seule valeur du type `Unit`. C'est le résultat des fonctions qui
n'ont qu'un effet (`print`, `each` ; `write_file` donne `Ok(())`), d'un bloc
vide et d'un bloc qui finit par une définition. Le mode interactif et
`rythme -e` ne l'affichent pas.

### 2.6 List

Une suite ordonnée et immuable de valeurs de n'importe quels types
(`[1, "deux", 3.0]` est valable). Les positions commencent à 0. Les listes se
construisent avec `[...]`, avec les intervalles, avec `+` et avec les
fonctions de listes.

Un **intervalle** `a..b` est la liste des nombres entiers de `a` à `b`, tous
deux inclus. C'est une liste ordinaire (`type_of(1..3)` vaut `"List"`). Il
est vide quand `b < a`, ses deux bornes doivent être des Int (sinon E005), et
il contient au plus 50 000 000 éléments (E023).

La même limite vaut pour toutes les listes : construire une liste de plus de
50 000 000 éléments, avec `+`, `..xs` entre crochets, `repeat`, `flatten`,
`flat_map` ou `chars`, est aussi l'erreur E023 (voir [2.3](#23-text)).

### 2.7 Record

Un enregistrement est un ensemble de champs, chacun avec un nom (n'importe
quel texte) et une valeur. Les champs gardent l'ordre dans lequel ils ont été
écrits : cet ordre sert à l'affichage et à `keys`, `values` et `entries`,
mais pas à l'égalité. Un nom de champ qui n'est pas un nom simple
(`"nom complet"`) s'écrit comme un texte.

### 2.8 Function

Les fonctions nommées, les fonctions anonymes, les fonctions intégrées et les
constructeurs avec champs sont tous des valeurs de type `Function` : on peut
les passer en argument, les renvoyer, les ranger dans des listes et des
enregistrements.

### 2.9 Option et Result

Deux types sont prédéfinis, comme s'ils étaient écrits :

```text
type Option = Some(value) | None
type Result = Ok(value) | Err(error)
```

`Option` sert pour une valeur qui peut manquer, `Result` pour une opération
qui peut échouer (voir [5.1](#51-les-erreurs-sont-des-valeurs)). Leurs champs
ont des noms : `Some(3).value` vaut `3` et `Err("oups").error` vaut `"oups"`.
Ils font partie du langage : définir un type `Option` ou `Result`, ou un
constructeur `Some`, `None`, `Ok` ou `Err`, est l'erreur E011.

### 2.10 Vos propres types

`type` définit un nouveau type et ses variantes (voir [4.7](#47-types)). Une
valeur d'un tel type s'affiche comme son constructeur suivi de ses champs :
`Cercle(2.0)`, `Feuille`.

### 2.11 Égalité

`==` et `!=` acceptent **deux valeurs quelconques** et n'échouent jamais :

- un Int et un Float sont égaux quand ils ont la même valeur numérique
  (`1 == 1.0`), comparée exactement : l'Int n'est pas d'abord arrondi en
  Float, donc `9007199254740993 == 9007199254740992.0` vaut `false` ;
- les textes, les Bool et `()` sont égaux quand ils sont identiques ;
- deux listes sont égales quand elles ont la même longueur et des éléments
  égaux, dans l'ordre ;
- deux enregistrements sont égaux quand ils ont les mêmes noms de champs avec
  des valeurs égales, quel que soit l'ordre des champs ;
- deux variantes sont égales quand elles ont le même type, le même
  constructeur et des champs égaux ;
- une fonction n'est égale qu'à elle-même (la même valeur de fonction) ;
- des valeurs de sortes différentes ne sont jamais égales (`"1" == 1` vaut
  `false`) ;
- `nan` n'est égal à rien, pas même à lui-même.

```rythme
type Couleur = Rouge | Vert
print(1 == 1.0, [1, 2] == [1, 2], {a: 1, b: 2} == {b: 2, a: 1})
print(Some(1) == Some(1), Rouge != Vert, "1" == 1, [] == {})
```

```text
true true true
true true false false
```

### 2.12 Ordre

`<`, `<=`, `>` et `>=` comparent :

- les nombres, Int et Float mélangés, selon leur valeur exacte ;
- les textes, caractère par caractère selon le point de code Unicode (donc
  `"Z" < "a"`, et les lettres accentuées viennent après `z` : ce n'est pas
  l'ordre du dictionnaire) ;
- les Bool (`false < true`), et `()` avec lui-même ;
- les listes, élément par élément, une liste plus courte venant d'abord quand
  elle est le début de l'autre ;
- deux valeurs **du même type** : d'abord selon le constructeur, dans l'ordre
  de la définition du type (`None < Some(x)`, `Ok(x) < Err(e)`, et pour
  `type Taille = Petit | Moyen | Grand`, `Petit < Moyen < Grand`), puis champ
  par champ.

Tout le reste (enregistrements, fonctions, valeurs de deux types différents,
un nombre et un texte...) est l'erreur E005. `sort`, `sort_by`, `min` et `max`
utilisent le même ordre et échouent de la même façon.

```rythme
type Taille = Petit | Moyen | Grand
print(1 < 2.5, "Zèbre" < "abeille", "abeille" < "banane", false < true)
print([1, 2] < [1, 3], [1, 2] < [1, 2, 0], Some(1) < Some(2))
print(sort(["poire", "Figue", "abricot", "été"]))
print(None < Some(1), Ok(5) < Err(1), Petit < Grand)
print(sort([Some(2), None, Some(1)]))
```

```text
true true true true
true true true
["Figue", "abricot", "poire", "été"]
true true true
[None, Some(1), Some(2)]
```

Quand la nature des valeurs est certaine, comme ici, l'erreur est trouvée
avant que le programme ne s'exécute (voir
[5.5](#55-la-vérification-avant-lexécution)) :

```rythme
print({a: 1} < {a: 2})
```

```text
erreur[E005]: impossible d'utiliser `<` avec Record et Record
 --> 14b-ordre.ry:1:14
  |
1 | print({a: 1} < {a: 2})
  |              ^ Record {a: 1} < Record {a: 2}
  |
  = en savoir plus : `rythme explain E005`
```

### 2.13 Affichage des valeurs

Chaque valeur a deux formes écrites :

- la forme **affichée**, utilisée par `print`, par `{...}` dans les textes,
  par `text(x)` et par `join` : un texte y apparaît tel quel, sans
  guillemets ;
- la forme **code**, utilisée par `debug`, par le mode interactif et
  `rythme -e`, à l'intérieur des listes et des enregistrements, et dans les
  messages d'erreur : un texte y apparaît entre guillemets, avec `"`, `\`, les
  retours à la ligne, les tabulations, les retours chariot et `{` échappés.

En dehors des textes, les deux formes sont identiques :

| Valeur | Affichée comme |
|--------|----------------|
| Int | `42`, `-7` |
| Float | voir [2.2](#22-float) : `2.0`, `0.5`, `1e16`, `inf`, `-inf`, `nan` |
| Bool | `true`, `false` |
| Unit | `()` |
| List | `[1, "a", 2.0]` |
| Record | `{nom: "Ada", "nom complet": "Ada L."}` : un nom de champ s'écrit comme un texte quand ce n'est pas un nom simple |
| variante | `None`, `Some(3)`, `Cercle(2.0)` |
| fonction | `<fn nom>` pour une fonction nommée ou intégrée, `<fn>` pour une fonction anonyme |
| constructeur avec champs | `<constructeur Cercle>` (en anglais : `<constructor Cercle>`) |

`print` écrit ses arguments séparés par une espace, puis un retour à la
ligne, sur la sortie standard. `debug(x)` écrit `[debug fichier:ligne:colonne]`
et la forme code de `x` sur la **sortie d'erreur**, puis renvoie `x` tel
quel : on peut donc le glisser au milieu d'un enchaînement.

```rythme
elements = ["a", "b"]
print("texte", 42, 2.0, elements, {nom: "Ada", "nom complet": "Ada L."}, None, ())
print(text(elements), "{elements}", text("simple"))
taille = debug(elements) |> len
print(taille)
print([print, fn(v) = v, Some, None])
```

```text
texte 42 2.0 ["a", "b"] {nom: "Ada", "nom complet": "Ada L."} None ()
["a", "b"] ["a", "b"] simple
[debug 15-affichage.ry:4:10] ["a", "b"]
2
[<fn print>, <fn>, <constructeur Some>, None]
```

---

## 3. Expressions

Tout ce qui donne une valeur est une expression, `if`, `match` et blocs
compris. Les expressions s'évaluent **de gauche à droite** : opérandes,
arguments, éléments de listes et champs d'enregistrements, dans l'ordre où
ils sont écrits.

### 3.1 Opérateurs : priorité et associativité

Du plus prioritaire au moins prioritaire :

| Niveau | Opérateurs | Associativité |
|--------|------------|---------------|
| 1 | appel `f(...)`, indice `x[i]`, champ `x.f`, méthode `x.f(...)`, `?` | de gauche à droite |
| 2 | `**` | à droite (`2 ** 3 ** 2` vaut `2 ** 9`) ; son opérande de droite peut commencer par `-` |
| 3 | `-` unaire | préfixe (`-a * b` vaut `(-a) * b` ; `-2 ** 2` vaut `-(2 ** 2)`) |
| 4 | `*` `/` `//` `%` | à gauche |
| 5 | `+` `-` | à gauche |
| 6 | `..` | aucune : un seul intervalle par expression |
| 7 | `\|>` | à gauche |
| 8 | `==` `!=` `<` `<=` `>` `>=` | aucune : `a < b < c` est une erreur |
| 9 | `not` | préfixe (`not a == b` vaut `not (a == b)`) |
| 10 | `and` | à gauche |
| 11 | `or` | à gauche |

Deux règles supplémentaires évitent des lectures trompeuses, toutes deux
signalées par E001 : une comparaison ne s'enchaîne pas (`0 < x < 10` ; écrivez
`0 < x and x < 10`), et un opérateur de calcul ou `..` ne peut pas suivre
directement une étape `|>` (`x |> f + 1` ; écrivez `(x |> f) + 1`).

```rythme
fn plus1(x) = x + 1
print(2 + 3 * 4, (2 + 3) * 4)          # 14 20
print(2 ** 3 ** 2, -2 ** 2, 2 ** -1)   # ** se groupe à droite, et avant le - unaire
print(10 - 3 - 2, 7 // 2 * 2)           # de gauche à droite
print(1 + 2 |> plus1)                   # |> passe après les calculs : plus1(3)
print(1..3 |> map(plus1))               # .. passe avant |>
print(5 |> plus1 == 6)                  # |> passe avant les comparaisons
print(not 1 == 2, not true and false)   # not passe après les comparaisons, avant and
print(true or false and false)          # and passe avant or
```

```text
14 20
512 -4 0.5
5 6
4
[2, 3, 4]
true
true false
true
```

```rythme
x = 5
print(0 < x < 10)
```

```text
erreur[E001]: les comparaisons ne peuvent pas s'enchaîner
 --> 22-enchaine.ry:2:13
  |
2 | print(0 < x < 10)
  |             ^
  |
  = aide: combinez-les avec `and` : `a < b and b < c`
  = en savoir plus : `rythme explain E001`
```

```rythme
fn plus1(x) = x + 1
print(1 |> plus1 + 1)
```

```text
erreur[E001]: `+` ne peut pas suivre directement une étape `|>`
 --> 24-erreur-enchainement.ry:2:18
  |
2 | print(1 |> plus1 + 1)
  |                  ^
  |
  = aide: `|>` passe après les calculs : mettez l'enchaînement entre parenthèses,
      comme `(x |> f) + 1`, ou continuez-le avec une fonction : `x |> f |> fn(v) = v + 1`
  = en savoir plus : `rythme explain E001`
```

### 3.2 Calcul

| Opération | Entre Int | Avec un Float | Autres valeurs |
|-----------|-----------|---------------|----------------|
| `a + b` | Int (E012 en cas de dépassement) | Float | deux textes : mis bout à bout ; deux listes : mises bout à bout ; sinon E005 |
| `a - b`, `a * b` | Int (E012 en cas de dépassement) | Float | E005 |
| `a / b` | **toujours un Float** (`6 / 3` vaut `2.0`) | Float | E005 |
| `a // b` | Int, arrondi vers le bas (vers −∞) | Float, arrondi vers le bas | E005 |
| `a % b` | Int, du signe de `b` | Float : le reste exact, du signe de `b` | E005 |
| `a ** b` | Int si `b >= 0` (E012 en cas de dépassement), Float si `b < 0` (`0 ** -n` est E008) | Float (E023 pour un nombre négatif à une puissance fractionnaire) | E005 |
| `-a` | Int (E012 pour le plus petit Int) | Float | E005 |

`/`, `//` et `%` par zéro donnent l'erreur E008, pour les Int comme pour les
Float. Rythme ne convertit jamais entre textes et nombres : `"1" + 1` est
E005 (écrivez `"{x}"`, `text(x)`, `int(t)` ou `float(t)`).

```rythme
print(7 / 2, 7 // 2, 7 % 3, 2 ** 10, 2 ** -1)
print(-7 // 2, -7 % 3, 7 % -3)
print(7.5 // 2, 7.5 % 2, 1 + 2.0, 9 ** 0.5)
print("ab" + "cd", [1, 2] + [3])
```

```text
3.5 3 1 1024 0.5
-4 2 -2
3.0 1.5 3.0 3.0
abcd [1, 2, 3]
```

### 3.3 Comparaisons et logique

`==` et `!=` suivent [2.11](#211-égalité) ; `<`, `<=`, `>` et `>=` suivent
[2.12](#212-ordre).

`and`, `or` et `not` demandent des Bool (sinon E005). `and` et `or` sont
**paresseux** : la partie droite n'est évaluée que si la partie gauche ne
décide pas du résultat (`false and ...` vaut `false` sans évaluer `...`).

### 3.4 Intervalles

`a..b` construit la liste `[a, a + 1, ..., b]`, voir [2.6](#26-list). Les
deux côtés sont des expressions de calcul : `1..n + 1` vaut `1..(n + 1)`.

### 3.5 Appels

`f(a, b)` évalue `f`, puis les arguments de gauche à droite, puis appelle la
fonction. Une virgule finale est permise, et les retours à la ligne entre les
parenthèses sont ignorés. Le nombre d'arguments doit être exactement le
nombre de paramètres (E004 ; certaines fonctions intégrées acceptent un
nombre variable d'arguments). Appeler une valeur qui n'est pas une fonction
est l'erreur E009, et `None()` aussi : un constructeur sans champs est une
valeur, pas une fonction. Il n'y a ni arguments nommés, ni valeurs par
défaut, ni application partielle.

### 3.6 Enchaînements `|>`

`x |> étape` passe `x` comme **premier argument** d'un appel. L'étape est
une fonction, éventuellement suivie de ses autres arguments :

| Écrit | Signifie |
|-------|----------|
| `x \|> f` | `f(x)` |
| `x \|> f(a, b)` | `f(x, a, b)` |
| `x \|> e.f` | `e.f(x)` : la fonction rangée dans le champ `f` de `e` |
| `x \|> v.m(a)` | `v.m(x, a)`, voir [3.8](#38-champs-et-appels-de-méthode) |
| `x \|> fn(v) = ...` | la fonction anonyme appliquée à `x` |
| `x \|> f?`, `x \|> f[0]`, `x \|> f(a).nom` | `f(x)?`, `f(x)[0]`, `f(x, a).nom` : ce qui suit l'étape s'applique à son résultat |

Un `.nom` écrit juste après la fonction d'une étape, avant toute parenthèse,
fait partie de la fonction (`x |> e.f` appelle le champ `f` de `e`) ; écrit
après les parenthèses, il s'applique au résultat.

La tête d'une étape est une seule expression simple : un nom, un
constructeur, une fonction anonyme ou une expression entre parenthèses. Le
corps d'une fonction anonyme s'étend aussi loin que possible vers la droite :
une telle étape doit donc être la dernière (ou être mise entre parenthèses).
Les enchaînements passent après les calculs et `..`, et avant les
comparaisons (voir [3.1](#31-opérateurs--priorité-et-associativité)). Une
ligne qui commence par `|>` continue la ligne précédente.

```rythme
fn ajouter(a, b) = a + b
outils = {double: fn(x) = x * 2}
print(3 |> ajouter(10))                 # ajouter(3, 10)
print(3 |> outils.double)               # outils.double(3)
print(3 |> Some)                        # Some(3)
print(" Salut " |> trim |> upper)       # upper(trim(" Salut "))
print([1, 2, 3] |> map(fn(x) = x * 2) |> sum)
print([4, 5] |> reverse[0])             # [0] s'applique à reverse([4, 5])
print(3 |> fn(x) = x * 100)             # une fonction anonyme comme étape
fn total(a, b)
  x = a |> parse_int?                   # ? s'applique à parse_int(a)
  Ok(x + b)
end
print(total("2", 3), total("deux", 3))
```

```text
13
6
Some(3)
SALUT
12
5
300
Ok(5) Err("\"deux\" n'est pas un nombre entier")
```

### 3.7 Indices

| Écrit | Donne | Quand ça échoue |
|-------|-------|-----------------|
| `liste[i]` | l'élément à la position `i` (à partir de 0) ; `-1` est le dernier, `-2` l'avant-dernier... | E007 hors limites, E005 si `i` n'est pas un Int |
| `texte[i]` | le caractère à la position `i`, sous forme de texte | E007, E005 |
| `enregistrement["nom"]` | le champ qui a ce nom | E010 s'il n'existe pas |

`get(x, i)` fait la même chose, mais renvoie une Option au lieu d'échouer.

### 3.8 Champs et appels de méthode

`valeur.nom` lit un champ : le champ d'un enregistrement, ou le champ d'une
variante nommé dans son `type` (`Cercle(2).rayon`). Un champ absent est
l'erreur E010. Le nom après le point peut être n'importe quel nom, un
mot-clé ou un mot réservé.

`valeur.nom(args)` est un **appel en style méthode** :

- si `valeur` est un enregistrement qui a un champ `nom`, il appelle la
  fonction rangée dans ce champ avec `args` (l'enregistrement lui-même n'est
  pas passé) ;
- sinon il appelle la fonction `nom` visible à cet endroit avec
  `(valeur, args...)` : `"salut".upper()` vaut `upper("salut")`, `10.max(20)`
  vaut `max(10, 20)`.

Le vérificateur avertit (W005) quand aucune fonction ni aucun champ
d'enregistrement du fichier ne porte ce nom. Une ligne qui commence par `.`
continue la ligne précédente : on peut donc enchaîner les appels de méthode
sur plusieurs lignes.

```rythme
type Forme = Cercle(rayon)
xs = [10, 20, 30]
ada = {nom: "Ada", "nom complet": "Ada Lovelace"}
print(xs[0], xs[-1], "été"[1], ada.nom, ada["nom complet"], Cercle(2).rayon)
print("salut".upper(), [3, 1, 2].sort(), 10.max(20))
calcul = {deux_fois: fn(x) = x * 2}
print(calcul.deux_fois(21))             # la fonction rangée dans le champ
print(get(xs, 5))
print(xs[5])
```

```text
10 30 t Ada Ada Lovelace 2
SALUT [1, 2, 3] 20
42
None
erreur[E007]: l'indice 5 est hors limites : la liste a 3 élément(s)
 --> 25-acces.ry:9:10
  |
9 | print(xs[5])
  |          ^ aucun élément ici
  |
  = aide: les indices valides vont de 0 à 2 (ou -1 pour le dernier élément)
  = aide: pour obtenir une Option au lieu d'une erreur, utilisez `get(x, 5)`
  = en savoir plus : `rythme explain E007`
```

### 3.9 `if`

```text
if condition then valeur else autre_valeur
```

La condition doit être un Bool (E005). Seule la branche choisie est évaluée.
**Le `else` est obligatoire** (E019), car `if` donne toujours une valeur.
Chaque branche est une seule expression ; pour plusieurs instructions,
utilisez `do ... end`. `else if` enchaîne les conditions. `then` et `else`
peuvent commencer une nouvelle ligne.

```rythme
fn signe(n) = if n < 0 then "négatif" else if n == 0 then "zéro" else "positif"
print(signe(-2), signe(0), signe(3))
fn decrire(n) =
  if n > 100 then do
    print("un grand nombre")
    "grand"
  end else "petit"
print(decrire(500))
```

```text
négatif zéro positif
un grand nombre
grand
```

```rythme
x = 5
if x > 0 then print("positif")
```

```text
erreur[E019]: ce `if` n'a pas de `else`
 --> 26b-si-erreur.ry:2:1
  |
2 | if x > 0 then print("positif")
  | ^^ chaque `if` doit avoir un `else`
  |
3 | 
  | - `else` attendu ici
  |
  = aide: en Rythme, `if` donne toujours une valeur, il lui faut donc les deux cas :
        if age >= 18 then "majeur" else "mineur"
  = en savoir plus : `rythme explain E019`
```

Quand il n'y a rien à faire dans un des cas, donnez la valeur unité :
`if x > 0 then print("positif") else ()`.

### 3.10 `match` et motifs

```text
match valeur
  motif -> résultat
  motif if condition -> résultat
  ...
end
```

La valeur examinée s'écrit sur la même ligne que `match` ; chaque cas
commence sur sa propre ligne (ou après un `;`), et `end` ferme le `match`. Il
faut au moins un cas, et pas de `|` devant un cas. Le résultat peut commencer
à la ligne qui suit `->`.

Les cas sont essayés **dans l'ordre**. Le premier cas dont le motif
correspond à la valeur, et dont la garde (s'il y en a une) vaut `true`, est
choisi ; les noms donnés par le motif sont visibles dans sa garde et dans son
résultat. Si aucun cas ne correspond, le programme s'arrête avec l'erreur
E006. Avant l'exécution, le vérificateur signale les cas oubliés qu'il peut
prouver (W002) et les cas placés après un cas qui accepte tout (W004).

| Motif | Correspond à |
|-------|--------------|
| `_` | n'importe quoi ; ne donne aucun nom |
| `nom` | n'importe quoi ; le nomme `nom` |
| `42`, `-1`, `2.5`, `-0.5` | un nombre égal à celui-ci (un motif Int correspond à un Float égal, et inversement : `0` correspond à `0.0`) |
| `"texte"` | exactement ce texte (pas de `{...}` dans un motif) |
| `true`, `false` | ce Bool |
| `()` | la valeur unité |
| `Feuille`, `None` | ce constructeur sans champs |
| `Cercle(m)`, `Rect(m1, m2)` | une variante de ce constructeur dont les champs correspondent aux motifs |
| `[]`, `[m]`, `[m1, m2]` | une liste d'exactement cette longueur dont les éléments correspondent |
| `[m1, ..reste]`, `[m1, ..]`, `[..reste]` | une liste d'au moins cette longueur ; `reste` désigne les éléments restants. `..` doit venir en dernier |
| `{nom}`, `{nom, age: m}`, `{"nom complet": m}`, `{nom, ..}` | un enregistrement qui a au moins ces champs (il peut en avoir d'autres) ; `{nom}` abrège `{nom: nom}` ; un `..` parmi les champs ne change rien (`{nom, ..}` vaut `{nom}`), et `{nom, ..autres}` est E001 : on ne peut pas donner de nom aux autres champs ; `{}` correspond à n'importe quel enregistrement |
| `m1 \| m2 \| m3` | l'une des possibilités ; elles ne peuvent pas donner de noms (E001) |
| `(m)` | la même chose que `m` |

Un nom ne peut apparaître qu'une fois dans un motif (E011) : pour comparer deux
parties, utilisez deux noms et une garde, `[a, b] if a == b -> ...`. Un
constructeur utilisé dans un motif doit exister (E003) et avoir le bon nombre
de champs (E004).

```rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur) | Point

fn decrire(valeur) = match valeur
  0 -> "le nombre zéro"
  -1 | -2 -> "moins un ou moins deux"
  2.5 -> "deux et demi"
  "salut" -> "une salutation"
  true -> "vrai"
  () -> "la valeur unité"
  [] -> "une liste vide"
  [x] -> "une liste à un élément, {x}"
  [premier, ..reste] if len(reste) > 3 -> "une longue liste qui commence par {premier}"
  [premier, second, ..] -> "une liste qui commence par {premier}, {second}"
  {nom, age: 36} -> "{nom}, 36 ans"
  {nom} -> "un enregistrement avec un nom : {nom}"
  Cercle(r) -> "un cercle de rayon {r}"
  Rect(l, h) if l == h -> "un carré"
  Rect(_, _) | Point -> "un rectangle ou un point"
  Some(Some(v)) -> "une Option dans une Option : {v}"
  _ -> "autre chose"
end

exemples = [0, -2, 2.5, "salut", true, (), [], [7], 1..10, [1, 2, 3],
  {nom: "Ada", age: 36}, {nom: "Alan", age: 41}, Cercle(1), Rect(2, 2), Point, Some(Some(4)), 0.0, 99]
exemples |> each(fn(v) = print(decrire(v)))
```

```text
le nombre zéro
moins un ou moins deux
deux et demi
une salutation
vrai
la valeur unité
une liste vide
une liste à un élément, 7
une longue liste qui commence par 1
une liste qui commence par 1, 2
Ada, 36 ans
un enregistrement avec un nom : Alan
un cercle de rayon 1
un carré
un rectangle ou un point
une Option dans une Option : 4
le nombre zéro
autre chose
```

Quand aucun cas ne correspond :

```rythme
fn nom(n) = match n
  1 -> "un"
  2 -> "deux"
end
print(nom(1))
print(nom(3))
```

```text
avertissement[W002]: ce `match` ne traite pas tous les cas
 --> 27b-aucun-cas.ry:1:13
  |
1 | fn nom(n) = match n
  |             ^^^^^^^
  |
  = note: non traité : `_`
  = aide: ajoutez un dernier cas `_ -> ...` pour toutes les autres valeurs

un
erreur[E006]: aucun cas de ce `match` ne traite Int 3
 --> 27b-aucun-cas.ry:1:19
  |
1 | fn nom(n) = match n
  |                   ^ cette valeur
  |
  = aide: ajoutez un cas pour cette valeur, ou un dernier cas `_ -> ...` qui traite tout le reste
  = appels en cours (le plus récent d'abord) :
      dans `nom` appelée en 27b-aucun-cas.ry:6:7
  = en savoir plus : `rythme explain E006`
```

Les motifs servent aussi dans les définitions (`[a, b] = liste`, voir
[4.1](#41-noms)) et dans les paramètres des fonctions (voir
[4.5](#45-fonctions)).

### 3.11 Blocs et `do`

Un **bloc** est une suite d'instructions, une par ligne, fermée par `end`. Le
corps d'une fonction écrite sur plusieurs lignes, le corps d'un `test` écrit
sur plusieurs lignes et `do ... end` sont des blocs. Une instruction est une
définition (`motif = expression`, une `fn` locale) ou une expression.

- La valeur d'un bloc est celle de sa **dernière instruction**, quand c'est
  une expression. Quand la dernière instruction est une définition, ou que le
  bloc est vide, la valeur est `()`.
- Une instruction-expression qui n'est pas la dernière doit avoir un effet :
  elle doit appeler une fonction (ou utiliser `?`). Sinon sa valeur serait
  perdue, et c'est l'erreur E024.
- `do ... end` est une expression : un bloc écrit partout où une valeur est
  attendue. Il ouvre une nouvelle portée (voir
  [4.3](#43-portées-et-masquage)). Il peut tenir sur une ligne :
  `do 1 + 1 end`.
- `type`, `test` et `use` ne sont permis qu'au premier niveau d'un fichier.

```rythme
fn decrire(n)
  sorte = if n % 2 == 0 then "pair" else "impair"
  "{n} est {sorte}"               # la dernière ligne donne le résultat
end
fn rien()
end
aire = do
  largeur = 3
  hauteur = 4
  largeur * hauteur
end
print(decrire(7), rien(), aire, do 1 + 1 end)
```

```text
7 est impair () 12 2
```

### 3.12 Fonctions anonymes

```text
fn(paramètres) = expression

fn(paramètres)
  instructions
end
```

Une fonction anonyme est une valeur. Ses paramètres sont des motifs, comme
ceux des fonctions nommées. Elle peut utiliser les noms visibles là où elle
est écrite (c'est une **fermeture** ; comme les valeurs ne changent jamais,
elle les garde simplement). Le corps de la forme sur une ligne s'étend aussi
loin que possible vers la droite. Une fonction anonyme n'a qu'une définition
et pas de garde : utilisez `match` à l'intérieur pour choisir entre des cas.
Quand elle est définie par `nom = fn(...) ...`, au premier niveau ou dans un
bloc, la fonction peut s'appeler elle-même par `nom`. Une fonction nommée
(`fn nom(...)`) ne peut pas s'écrire à l'intérieur d'une expression (E001).

```rythme
double = fn(x) = x * 2
fn ajouteur(n) = fn(x) = x + n        # la fonction anonyme garde n
plus10 = ajouteur(10)
montrer = fn(x)
  y = x * 2
  "valeur : {y}"
end
print(double(4), plus10(5), montrer(3))
print([[1, 2], [3, 4]] |> map(fn([a, b]) = a * b))
fn compte_a_rebours(n)
  etape = fn(k) = if k == 0 then "décollage" else "{k} " + etape(k - 1)
  etape(n)
end
print(compte_a_rebours(3))
```

```text
8 15 valeur : 6
[2, 12]
3 2 1 décollage
```

### 3.13 Listes et enregistrements

`[a, b, c]` construit une liste. `..xs` entre les crochets insère tous les
éléments de la liste `xs`, à n'importe quelle position (`[0, ..xs, 9]`) ;
insérer ainsi une valeur qui n'est pas une liste est E005.

`{nom: valeur, ...}` construit un enregistrement. Le nom de champ peut être
un nom, un mot-clé, un mot réservé, un nom avec majuscule ou un texte
(`{"nom complet": x}`). `{nom}` abrège `{nom: nom}`. `..e` recopie tous les
champs de l'enregistrement `e`. Les éléments sont traités de gauche à
droite : un champ écrit plus loin avec le même nom remplace la valeur
précédente mais garde sa position. Écrire deux fois explicitement le même nom
de champ est E011. **C'est ainsi qu'on « met à jour » un enregistrement** :
`{..ada, naissance: 1816}` est un nouvel enregistrement ; `ada` ne change pas.

La virgule finale est permise dans les deux cas.

```rythme
ada = {nom: "Ada", naissance: 1815}
print({..ada, naissance: 1816}, {..ada, ville: "Londres"}, {naissance: 0, ..ada})
nom = "Grace"
print({nom, naissance: 1906})
xs = [1, 2]
print([0, ..xs, 3], xs + [9], 1..5, 5..1)
```

```text
{nom: "Ada", naissance: 1816} {nom: "Ada", naissance: 1815, ville: "Londres"} {naissance: 1815, nom: "Ada"}
{nom: "Grace", naissance: 1906}
[0, 1, 2, 3] [1, 2, 9] [1, 2, 3, 4, 5] []
```

---

## 4. Définitions

### 4.1 Noms

`motif = expression` donne des noms à une valeur, au premier niveau ou dans
un bloc. La partie gauche est le plus souvent un nom, mais ce peut être
n'importe quel motif de `match` (voir [3.10](#310-match-et-motifs)) qui
décompose la valeur : `_`, un motif de liste (`[a, b]`, `[premier, ..reste]`,
`[premier, ..]`), un motif d'enregistrement (`{nom, naissance: annee}`,
`{nom, ..}`), un motif de constructeur (`Some(v)`) ou un littéral. Si la
valeur ne correspond pas, le programme s'arrête avec E006 (trouvée avant
l'exécution quand la nature de la valeur est certaine). La valeur peut
commencer à la ligne qui suit `=`.

Comme dans `match`, `{nom, ..autres}` est E001 : on ne peut pas donner de
nom aux autres champs d'un enregistrement (écrivez `{nom, ..}`, ou utilisez
`remove`). Une garde n'est pas permise (utilisez `match`), et les
possibilités (`|`) seulement à l'intérieur de parenthèses, de crochets ou
d'accolades, où elles ne peuvent pas donner de noms : `Some(1 | 2) = x` ne fait que vérifier la
valeur.

```rythme
[a, b] = [1, 2]
{nom, naissance: annee} = {nom: "Ada", naissance: 1815, ville: "Londres"}
[tete, ..queue] = 1..4
[premier, ..] = ["x", "y", "z"]
{ville, ..} = {nom: "Ada", ville: "Londres"}
Some(v) = Some(5)
print(a, b, nom, annee, tete, queue, premier, ville, v)
[x, y] = [1, 2, 3]
```

```text
1 2 Ada 1815 1 [2, 3, 4] x Londres 5
erreur[E006]: la valeur List [1, 2, 3] ne correspond pas à ce motif
 --> 40-noms.ry:8:1
  |
8 | [x, y] = [1, 2, 3]
  | ^^^^^^
  |
  = aide: utilisez `match` pour traiter les différentes formes possibles d'une valeur
  = en savoir plus : `rythme explain E006`
```

```rythme
ada = {nom: "Ada", naissance: 1815}
{nom, ..autres} = ada
```

```text
erreur[E001]: un motif d'enregistrement ne peut pas nommer les autres champs
 --> 40b-reste.ry:2:9
  |
2 | {nom, ..autres} = ada
  |         ^^^^^^
  |
  = aide: écrivez `{nom, ..}` pour ignorer les autres champs ; pour un enregistrement sans certains champs, utilisez `remove(enregistrement, "nom")`
  = en savoir plus : `rythme explain E001`
```

### 4.2 Les noms ne changent jamais

Un nom désigne une seule valeur dans toute sa portée : le définir à nouveau
dans la même portée est l'erreur E011, trouvée avant l'exécution. `x += 1` et
les formes semblables sont aussi des erreurs (E011). Donnez un nouveau nom à
la nouvelle valeur.

```rythme
total = 1
total = total + 1
```

```text
erreur[E011]: `total` est déjà défini
 --> 41-redefinir.ry:2:1
  |
2 | total = total + 1
  | ^^^^^ défini à nouveau ici
  |
1 | total = 1
  | ----- défini une première fois ici
  |
  = aide: les valeurs ne changent jamais en Rythme : donnez un nouveau nom à la nouvelle valeur, comme `total2 = total + 1`
  = en savoir plus : `rythme explain E011`
```

Dans le mode interactif, on peut redéfinir un nom (voir
[7.2](#72-le-mode-interactif-repl)).

### 4.3 Portées et masquage

Une portée, c'est : le premier niveau d'un fichier ; une définition d'une
fonction (ses paramètres et les instructions de son corps partagent une seule
portée) ; chaque bloc `do` ; chaque cas d'un `match` ; chaque fonction
anonyme.

- Dans une même portée, un nom n'est défini qu'une fois (E011), et un motif
  ne peut pas donner deux fois le même nom (E011).
- Une portée intérieure **peut reprendre un nom** d'une portée extérieure,
  qui est alors masqué : un paramètre peut porter le nom d'une valeur du
  premier niveau, un bloc `do` peut définir un nom qui existe déjà dehors.
- Un nom local peut masquer une fonction intégrée sans rien dire ; une
  définition du premier niveau qui masque une fonction intégrée donne
  l'avertissement W003.
- Les noms des fichiers apportés par `use` comptent comme des noms du premier
  niveau : en redéfinir un est E011.

```rythme
x = 1
fn fois_dix(x) = x * 10      # un paramètre peut reprendre un nom extérieur
fn g(n)
  y = n + 1
  z = do
    y = 100                  # un bloc imbriqué peut reprendre un nom
    y + 1
  end
  [y, z]
end
print(x, fois_dix(2), g(1))
```

```text
1 20 [2, 101]
```

### 4.4 L'ordre du premier niveau

Avant l'exécution d'un fichier, toutes ses définitions `type` et toutes ses
définitions `fn` du premier niveau sont enregistrées. Une fonction ou un
constructeur peut donc être utilisé n'importe où dans le fichier, même
au-dessus de sa définition, et les fonctions du premier niveau peuvent
s'appeler les unes les autres dans n'importe quel ordre.

Ensuite, les définitions `nom = ...` et les expressions s'exécutent **de haut
en bas**. Une valeur ne peut être utilisée qu'après sa définition :
directement au premier niveau, le vérificateur le signale avant l'exécution
(E003) ; à travers une fonction appelée trop tôt, l'erreur arrive pendant
l'exécution (E003, « pas encore défini »).

```rythme
print(double(21), Cercle(1))     # fonctions et types sont connus dans tout le fichier
fn double(x) = x * 2
type Forme = Cercle(rayon)
```

```text
42 Cercle(1)
```

```rythme
print(limite)
limite = 10
```

```text
erreur[E003]: `limite` est utilisé avant d'être défini
 --> 48b-ordre.ry:1:7
  |
1 | print(limite)
  |       ^^^^^^ utilisé ici
  |
2 | limite = 10
  | ------ défini plus loin, ici
  |
  = aide: déplacez la définition au-dessus de cette ligne
  = en savoir plus : `rythme explain E003`
```

```rythme
fn montrer_limite() = print(limite)
montrer_limite()
limite = 10
```

```text
erreur[E003]: `limite` n'est pas encore défini
 --> 48c-ordre.ry:1:29
  |
1 | fn montrer_limite() = print(limite)
  |                             ^^^^^^ utilisé ici
  |
3 | limite = 10
  | ------ défini plus loin, ici
  |
  = aide: le fichier s'exécute de haut en bas : déplacez cette définition au-dessus de la ligne qui l'utilise (directement ou via une fonction)
  = appels en cours (le plus récent d'abord) :
      dans `montrer_limite` appelée en 48c-ordre.ry:2:1
  = en savoir plus : `rythme explain E003`
```

La valeur de la dernière expression du premier niveau est affichée par le
mode interactif et par `rythme -e`, pas quand on lance un fichier. Toute
autre expression du premier niveau doit avoir un effet (E024).

### 4.5 Fonctions

```text
fn nom(paramètres) = expression

fn nom(paramètres)
  instructions
end

fn nom(paramètres) if garde = expression
```

- Les parenthèses sont obligatoires, même sans paramètres :
  `fn bonjour() = ...`.
- Les paramètres sont des **motifs** : `fn tete([x, ..]) = x`,
  `fn saluer({nom}) = ...`.
- Dans la forme en bloc, le corps commence à la ligne qui suit les paramètres
  (ou la garde) et se termine par `end` ; sa dernière instruction donne le
  résultat.
- **Définition par cas.** Plusieurs définitions de la même fonction, écrites
  les unes juste après les autres (des lignes vides et des commentaires
  peuvent les séparer), forment une seule fonction. Elles doivent avoir le
  même nombre de paramètres (E004). À chaque appel, elles sont essayées
  **dans l'ordre** ; la première dont tous les paramètres correspondent, et
  dont la garde (s'il y en a une) vaut `true`, est utilisée. Si aucune ne
  correspond, le programme s'arrête avec E006. Des définitions du même nom
  qui ne se suivent pas sont E011.
- Une **garde** est évaluée une fois que les paramètres correspondent, avec
  leurs noms visibles. Si un `?` dans la garde rencontre `None` ou `Err(...)`,
  l'appel tout entier renvoie aussitôt cette valeur.
- Avant l'exécution, le vérificateur signale les définitions placées après
  une définition qui accepte tous les arguments (W004), et, pour les
  fonctions à un paramètre, les cas qu'aucune définition ne traite (W002).
- Une fonction écrite dans un bloc est **locale** : elle est visible après sa
  définition dans ce bloc et peut s'appeler elle-même, mais elle ne voit que
  les noms définis avant elle. Des fonctions locales écrites **les unes juste
  après les autres** (seulement des lignes vides et des commentaires entre
  elles) forment un groupe, et chaque fonction du groupe voit toutes les
  autres, dans n'importe quel ordre, comme les fonctions du premier niveau :
  elles peuvent s'appeler les unes les autres. Une valeur définie entre deux
  fonctions locales termine le groupe ; pour s'en servir dans les deux
  fonctions, définissez-la avant elles.
- Les appels avec un mauvais nombre d'arguments sont trouvés avant
  l'exécution quand la fonction est connue par son nom (E004), sinon pendant
  l'exécution.

```rythme
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

fn signe(n) if n < 0 = "négatif"
fn signe(0) = "zéro"
fn signe(_) = "positif"

fn saluer({nom}) = "Bonjour, {nom} !"
fn bonjour() = "bonjour"

print(fib(10), signe(-5), signe(0), signe(9), saluer({nom: "Ada", age: 36}), bonjour())
```

```text
55 négatif zéro positif Bonjour, Ada ! bonjour
```

```rythme
fn parite(n)
  fn pair(0) = true
  fn pair(k) = impair(k - 1)     # impair est écrite juste après : pair la voit
  fn impair(0) = false
  fn impair(k) = pair(k - 1)
  if pair(n) then "pair" else "impair"
end
print(parite(10), parite(7))
```

```text
pair impair
```

```rythme
fn nom(1) = "un"
fn nom(2) = "deux"
print(nom(3))
```

```text
avertissement[W002]: `nom` ne traite pas tous les arguments possibles
 --> 43b-aucune-definition.ry:1:4
  |
1 | fn nom(1) = "un"
  |    ^^^
  |
  = note: non traité : `_`
  = aide: ajoutez une définition pour ce cas, comme `fn nom(_) = ...`

erreur[E006]: aucune définition de `nom` n'accepte ces arguments : nom(3)
 --> 43b-aucune-definition.ry:3:7
  |
3 | print(nom(3))
  |       ^^^^^^ appelée ici
  |
1 | fn nom(1) = "un"
  | --------- ne correspond pas
  |
2 | fn nom(2) = "deux"
  | --------- ne correspond pas
  |
  = aide: ajoutez une définition qui traite ce cas, par exemple une qui se termine par un simple nom ou `_`
  = en savoir plus : `rythme explain E006`
```

### 4.6 Récursion et appels terminaux

Rythme n'a pas de boucles : la répétition s'écrit par récursion ou avec les
fonctions de listes (`map`, `filter`, `fold`, `each`...).

Un appel est un **appel terminal** quand c'est la toute dernière chose que
fait une fonction. Les appels terminaux n'utilisent pas de pile : une
fonction peut s'appeler elle-même (ou appeler une autre fonction) en position
terminale des millions de fois. Un appel est en position terminale quand
c'est :

- le corps d'une fonction, ou la dernière instruction de son bloc ;
- une branche d'un `if` en position terminale ;
- le résultat d'un cas de `match` en position terminale (pas sa garde) ;
- la dernière instruction d'un bloc `do` en position terminale ;
- la partie droite d'un `and` ou d'un `or` en position terminale, quand cette
  partie droite est un appel.

Une étape `x |> f(a)` et un appel de méthode `x.f(a)` sont des appels. Un
appel qui sert d'argument, d'opérande (`n * fact(n - 1)`), de condition, ou
qui est suivi de `?`, n'est pas en position terminale.

Les autres appels s'imbriquent, et l'imbrication est limitée par la pile :
au-delà d'une limite, le programme s'arrête avec E014. La limite dépend de la
fonction ; avec la commande `rythme`, c'est plusieurs centaines de milliers
d'appels imbriqués (environ 900 000 pour `somme` ci-dessous, plus de 400 000
même pour une fonction qui en fait bien plus à chaque appel), et 600 dans le
playground. Les fonctions de listes (`map`, `fold`...) ne s'imbriquent pas.

```rythme
fn decompte(0, acc) = acc
fn decompte(n, acc) = decompte(n - 1, acc + 1)       # un appel terminal
fn est_pair(0) = true
fn est_pair(n) = est_impair(n - 1)                    # appels terminaux entre deux fonctions
fn est_impair(0) = false
fn est_impair(n) = est_pair(n - 1)
print(decompte(1000000, 0), est_pair(1000001))
fn somme([]) = 0
fn somme([x, ..reste]) = x + somme(reste)             # pas un appel terminal : + vient après
print(somme(1..5000000))
```

```text
1000000 false
erreur[E014]: trop d'appels imbriqués (917503 appels en cours)
 --> 44-terminal.ry:9:30
  |
9 | fn somme([x, ..reste]) = x + somme(reste)             # pas un appel terminal : + vient après
  |                              ^^^^^^^^^^^^ cet appel va trop loin
  |
  = aide: une fonction récursive qui ne s'arrête jamais ? Vérifiez qu'elle atteint toujours un cas de base.
      Sinon, faites de l'appel récursif la toute dernière chose que fait la fonction
      (appel terminal), par exemple avec un accumulateur :
        fn somme([], acc) = acc
        fn somme([x, ..reste], acc) = somme(reste, acc + x)
      ou utilisez `fold`, `map`, `filter`, qui n'ont pas de limite.
  = appels en cours (le plus récent d'abord) :
      dans `somme` (×40) appelée en 44-terminal.ry:9:30
      … et 917463 autres appels
  = en savoir plus : `rythme explain E014`
```

(Le nombre d'appels en cours varie selon la fonction et la machine.)

### 4.7 Types

```text
type Nom = Variante | Variante(champ, champ) | ...
```

- Le nom du type et les noms des variantes commencent par une majuscule ; les
  noms des champs commencent par une minuscule et sont distincts dans une
  même variante (E011).
- Un `|` peut précéder la première variante, et les variantes peuvent
  s'écrire sur plusieurs lignes qui commencent par `|`.
- Une variante **avec champs** est un constructeur : une fonction qui prend
  exactement un argument par champ (sinon E004). Une variante **sans champs**
  est une valeur (l'appeler est E009).
- Les noms de constructeurs sont globaux : deux types ne peuvent pas avoir un
  constructeur du même nom, et un type ne peut pas être défini deux fois
  (E011) ; `Option`, `Result` et leurs constructeurs ne peuvent pas non plus
  être redéfinis. Le nom du type n'est pas une valeur ; il apparaît dans
  `type_of` et dans les messages.
- Un type peut se contenir lui-même (arbres, listes...). `type` n'est permis
  qu'au premier niveau.
- On lit les champs avec `.champ` ou avec des motifs. Égalité et ordre : voir
  [2.11](#211-égalité) et [2.12](#212-ordre).
- Les commentaires `##` écrits au-dessus d'un `type` le documentent : `help`
  les affiche pour chacun de ses constructeurs, après le nom du type.

```rythme
## Une forme à dessiner.
type Forme =
  | Cercle(rayon)
  | Rect(largeur, hauteur)
  | Vide

fn aire(Cercle(r)) = pi * r * r
fn aire(Rect(l, h)) = l * h
fn aire(Vide) = 0

formes = [Cercle(1.0), Rect(2, 3), Vide]
print(formes |> map(aire))
print(Rect(2, 3).largeur, type_of(Vide), Cercle, map([1, 2], Cercle))
help(Cercle)
```

```text
avertissement[W009]: ceci affiche une fonction, pas une valeur
  --> sample.ry:13:42
   |
13 | print(Rect(2, 3).largeur, type_of(Vide), Cercle, map([1, 2], Cercle))
   |                                          ^^^^^^ un constructeur, affiché comme `<constructeur ...>`
   |
   = aide: pour afficher ce qu'elle donne, appelez-la : `Cercle(...)`

[3.141592653589793, 6, 0]
2 Forme <constructeur Cercle> [Cercle(1), Cercle(2)]
Cercle(rayon)
  Un constructeur du type `Forme`.
  Une forme à dessiner.
```

(Le vérificateur prévient que `Cercle` seul est affiché comme une fonction :
ici, c'est voulu.)

### 4.8 Tests

```text
test "nom" = expression

test "nom"
  instructions
end
```

Le nom est un texte simple. Les tests ne sont permis qu'au premier niveau.
Ils sont ignorés quand le programme est lancé normalement, et exécutés par
`rythme test fichier.ry` :

1. le fichier et les fichiers qu'il utilise sont chargés et vérifiés ;
2. les définitions `nom = ...` s'exécutent, mais **les expressions du premier
   niveau sont sautées** (dans tous les fichiers), pour qu'un passage de tests
   n'affiche rien et ne demande rien ;
3. les tests s'exécutent dans l'ordre : ceux des fichiers utilisés d'abord,
   puis ceux du fichier.

Un test **réussit** s'il se termine sans erreur et que sa valeur n'est ni
`false`, ni `Err(...)`, ni `None`. On peut utiliser `?` dans un test : un `?`
qui rencontre `None` ou `Err(...)` termine le test avec cette valeur, ce qui
le fait échouer. Le code de sortie vaut 0 quand tous les tests réussissent
(ou qu'il n'y en a aucun) et 1 sinon.

```rythme
fn double(x) = x * 2

test "le double de 2 est 4" = assert_eq(double(2), 4)

test "plusieurs vérifications"
  assert_eq(double(0), 0)
  assert(double(-1) < 0, "doit rester négatif")
end

test "un nombre lu"
  n = parse_int("21")?            # `?` est permis dans un test
  assert_eq(double(n), 42)
end

test "une attente fausse" = assert_eq(double(3), 7)

print("pas affiché par rythme test")
```

`rythme test 46-tests.ry` :

```text
Exécution de 4 test(s)
  ✓ le double de 2 est 4
  ✓ plusieurs vérifications
  ✓ un nombre lu
  ✗ une attente fausse
erreur[E015]: assertion échouée : les valeurs sont différentes
  --> 46-tests.ry:15:29
   |
15 | test "une attente fausse" = assert_eq(double(3), 7)
   |                             ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: obtenu :  6
       attendu : 7
   = en savoir plus : `rythme explain E015`

4 test(s) : 3 réussi(s), 1 échoué(s)
```

Code de sortie : 1.

### 4.9 Modules : `use`

```text
use "chemin/fichier.ry"
```

- `use` n'est permis qu'au premier niveau ; le chemin est un texte simple.
- Le chemin est relatif au **dossier du fichier qui contient le `use`** (pour
  `rythme -e` et le mode interactif : le dossier actuel). Quand il n'a pas
  d'extension, `.ry` est ajouté. Ses parties sont séparées par `/` (ou `\`,
  écrit `\\` dans un texte) sur tous les systèmes, et `..` remonte d'un
  dossier : `use "../outils/calculs.ry"`.
- Tout ce qui est défini au premier niveau du fichier utilisé (valeurs,
  fonctions, types et constructeurs) devient visible dans le fichier qui
  l'utilise, ainsi que tout ce que ce fichier utilise lui-même. Il n'y a ni
  espaces de noms ni définitions privées : un nom défini dans deux fichiers
  est E011.
- Chaque fichier est chargé, vérifié et exécuté **une seule fois** par
  programme, avant le fichier qui l'utilise, où que le `use` soit écrit dans
  ce fichier. Ses expressions du premier niveau s'exécutent (un `print` dans
  un fichier utilisé affiche une fois).
- Un fichier introuvable est E017 (avec une suggestion quand un nom proche
  existe), un dossier aussi ; des fichiers qui s'utilisent en cercle sont
  E017. Les messages montrent le chemin d'un fichier utilisé tel qu'on
  l'atteint depuis le dossier actuel.

```rythme
# geometrie.ry
print("geometrie.ry ne s'exécute qu'une fois")
fn aire_rectangle(l, h) = l * h
origine = {x: 0, y: 0}
```

```rythme
# carres.ry
use "geometrie.ry"
fn aire_carre(cote) = aire_rectangle(cote, cote)
```

```rythme
# principal.ry
use "geometrie.ry"
use "carres"               # `.ry` est ajouté quand le chemin n'a pas d'extension
print(aire_rectangle(2, 3), aire_carre(4), origine)
```

`rythme principal.ry` :

```text
geometrie.ry ne s'exécute qu'une fois
6 16 {x: 0, y: 0}
```

Avec `ping.ry` qui contient `use "pong.ry"` et `pong.ry` qui contient
`use "ping.ry"`, `rythme ping.ry` donne :

```text
erreur[E017]: `ping.ry` s'utilise lui-même (directement ou via d'autres fichiers)
 --> pong.ry:1:1
  |
1 | use "ping.ry"
  | ^^^^^^^^^^^^^
  |
  = aide: déplacez ce dont les deux fichiers ont besoin dans un troisième fichier, utilisé par les deux
  = en savoir plus : `rythme explain E017`
```

---

## 5. Erreurs

### 5.1 Les erreurs sont des valeurs

Rythme n'a ni `null` ni exceptions. Une valeur qui peut manquer est une
`Option` (`Some(valeur)` ou `None`) ; une opération qui peut échouer renvoie
un `Result` (`Ok(valeur)` ou `Err(erreur)`). La bibliothèque suit cette
règle : `first`, `last`, `get`, `find`, `index_of` et `env` renvoient des
Options ; `parse_int`, `parse_float`, `parse_json`, `read_file`, `write_file`
et `append_file` renvoient des Results.

On les traite avec `match`, avec `?`, ou avec les fonctions `unwrap_or`,
`unwrap`, `is_some`, `is_none`, `is_ok`, `is_err`, `map` (qui applique une
fonction à la valeur contenue dans `Some` ou `Ok`), `and_then`, `ok_or` et
`map_err` (voir [6](#6-bibliothèque-standard)).

### 5.2 L'opérateur `?`

`expression?` regarde la valeur de l'expression :

- `Some(x)` ou `Ok(x)` : le résultat est `x` ;
- `None` ou `Err(e)` : la fonction en cours (nommée ou anonyme) **renvoie
  aussitôt cette valeur** ; dans un test, le test se termine avec elle ;
- toute autre valeur : erreur E016.

`?` est un opérateur postfixe de la plus haute priorité : `parse_int(t)? + 1`
ajoute 1 au nombre. On ne peut l'utiliser que dans une fonction ou un test
(E016, trouvée avant l'exécution). Dans une garde, il fait se terminer
l'appel tout entier.

```rythme
fn additionner_textes(a, b)
  x = parse_int(a)?          # sur Err(...), la fonction le renvoie aussitôt
  y = parse_int(b)?
  Ok(x + y)
end
print(additionner_textes("1", "2"), additionner_textes("1", "deux"))

fn premier_pair_double(liste)
  x = find(liste, is_even)?  # sur None, la fonction renvoie None
  Some(x * 2)
end
print(premier_pair_double([1, 4, 5]), premier_pair_double([1, 3]))
```

```text
Ok(3) Err("\"deux\" n'est pas un nombre entier")
Some(8) None
```

### 5.3 Arrêter le programme

- `fail(message)` arrête le programme avec l'erreur E021 et le message.
- `assert(condition, message)` et `assert_eq(obtenu, attendu)` arrêtent avec
  E015 quand la vérification échoue ; on s'en sert surtout dans les tests.
- `unwrap(x)` arrête avec E013 sur `None` ou `Err(...)`.
- `exit(code)` arrête immédiatement avec ce code de sortie (de 0 à 255, E023
  sinon ; 0 s'il est omis).

### 5.4 Erreurs pendant l'exécution

Quand une erreur survient pendant l'exécution, Rythme arrête le programme,
écrit un message sur la **sortie d'erreur** et se termine avec le code
**1**. Ce que le programme a affiché avant reste affiché. Un message
comporte :

1. un en-tête : `erreur[CODE]: ce qui s'est passé` ;
2. l'endroit : `--> fichier:ligne:colonne`, puis la ligne de code avec
   l'endroit exact marqué, et parfois d'autres endroits liés ;
3. des notes et des lignes d'aide (`= note:`, `= aide:`), souvent avec une
   suggestion ;
4. pour les erreurs dans des fonctions, les **appels en cours**, le plus
   récent d'abord : les appels répétés sont regroupés (`×40`), et les appels
   terminaux n'apparaissent pas, puisqu'ils remplacent la fonction qui les a
   faits ;
5. la commande pour en savoir plus : `rythme explain CODE`.

```rythme
fn moyenne(liste) = sum(liste) / len(liste)
fn rapport(listes) = map(listes, moyenne)
print("début")
print(rapport([[1, 2], []]))
print("jamais affiché")
```

```text
début
erreur[E008]: division par zéro
 --> 51-execution.ry:1:32
  |
1 | fn moyenne(liste) = sum(liste) / len(liste)
  |                                ^
  |
  = aide: vérifiez d'abord le diviseur : `if b == 0 then ... else a / b`,
      ou renvoyez un Result : `if b == 0 then Err("division par zéro") else Ok(a / b)`
  = appels en cours (le plus récent d'abord) :
      dans `moyenne` appelée en 51-execution.ry:2:22
  = en savoir plus : `rythme explain E008`
```

Code de sortie : 1.

### 5.5 La vérification avant l'exécution

Chaque programme est **lu et vérifié en entier avant d'être exécuté** : par
`rythme fichier.ry`, `rythme run`, `rythme test`, `rythme -e`, et pour chaque
saisie du mode interactif. `rythme check fichier.ry` ne fait que cette étape,
et n'exécute rien ; avec plusieurs fichiers (`rythme check a.ry b.ry`), il
vérifie chacun d'eux, même après un fichier qui a des erreurs, et se termine
avec le code 1 si l'un d'eux a une erreur.

- Une erreur de syntaxe arrête la lecture à la première.
- Ensuite, le vérificateur signale **tous** les problèmes qu'il trouve d'un
  coup, les erreurs d'abord. S'il y a au moins une erreur, rien ne s'exécute
  (code de sortie 1). Les avertissements sont affichés, et le programme
  s'exécute quand même.

Le vérificateur trouve :

- les noms et constructeurs inconnus, avec le nom existant le plus proche ou
  le nom Rythme d'une fonction venue d'un autre langage (E003) ;
- les valeurs du premier niveau utilisées avant leur définition (E003) ;
- les mauvais nombres d'arguments donnés aux fonctions et aux constructeurs
  connus par leur nom, et les motifs de constructeur avec un mauvais nombre
  de champs (E004) ;
- les appels de choses qui ne sont pas des fonctions : `None()`, `2(x + 1)`
  (E009) ;
- les noms, types, constructeurs et champs définis deux fois, et les noms
  donnés deux fois dans un motif (E011) ;
- `?` hors d'une fonction (E016) ;
- les lignes dont la valeur est perdue (E024), `_` utilisé comme valeur et
  les possibilités de motifs qui donnent des noms (E001) ;
- les noms locaux inutilisés (W001), les cas oubliés (W002), les fonctions
  intégrées masquées par une définition du premier niveau (W003), les cas qui
  ne peuvent jamais être atteints (W004), les fonctions inconnues après un
  point (W005).

Le vérificateur connaît aussi la nature des valeurs quand elle est certaine :
les valeurs écrites dans le code, et ce que donnent à coup sûr les
opérateurs, les fonctions intégrées et les fonctions du programme. Il
signale, avant l'exécution, les opérations qui échoueraient à chaque fois
qu'elles s'exécutent, avec le code, le message et l'aide de l'erreur sur
laquelle le programme s'arrêterait : une mauvaise sorte de valeur pour un
opérateur, une condition, un intervalle, un indice ou une fonction intégrée
(E005), un champ qu'un enregistrement ou un constructeur n'a pas (E010), un
motif qui ne correspond jamais à la valeur (E006), `?` sur une valeur qui
n'est ni une Option ni un Result (E016), une valeur appelée comme une
fonction (E009), un `assert_eq` qui ne peut jamais réussir (E015), une
fonction donnée à une fonction intégrée avec un mauvais nombre de paramètres
(E004). Il prévient aussi des comparaisons toujours fausses ou toujours
vraies (W006), des tests qui ne peuvent jamais échouer (W007), des résultats
perdus de fonctions sans effet, comme `push(liste, x)` seul sur sa ligne
(W008), et des fonctions affichées au lieu de leur valeur, comme `print(f)`
(W009). Quand il n'est pas sûr, il ne dit rien : le reste (valeurs dont la
nature varie, indices hors limites, division par zéro...) est trouvé pendant
l'exécution.

```rythme
fn aire(largeur, hauteur) = largeur * hauteru
fn principal()
  total = aire(2)
  print(totl)
end
x = 1
x = 2
```

`rythme check 52-verifier.ry` :

```text
erreur[E003]: nom inconnu `hauteru`
 --> 52-verifier.ry:1:39
  |
1 | fn aire(largeur, hauteur) = largeur * hauteru
  |                                       ^^^^^^^ non défini
  |
  = aide: vouliez-vous dire `hauteur` ?
  = en savoir plus : `rythme explain E003`

erreur[E004]: `aire` attend 2 arguments, mais en a reçu 1
 --> 52-verifier.ry:3:11
  |
3 |   total = aire(2)
  |           ^^^^^^^ 1 argument(s) ici
  |
1 | fn aire(largeur, hauteur) = largeur * hauteru
  |    ---- définie ici
  |
  = aide: utilisation : `aire(largeur, hauteur)`
  = en savoir plus : `rythme explain E004`

erreur[E003]: nom inconnu `totl`
 --> 52-verifier.ry:4:9
  |
4 |   print(totl)
  |         ^^^^ non défini
  |
  = aide: vouliez-vous dire `total` ?
  = en savoir plus : `rythme explain E003`

erreur[E011]: `x` est déjà défini
 --> 52-verifier.ry:7:1
  |
7 | x = 2
  | ^ défini à nouveau ici
  |
6 | x = 1
  | - défini une première fois ici
  |
  = aide: les valeurs ne changent jamais en Rythme : donnez un nouveau nom à la nouvelle valeur, comme `total2 = total + 1`
  = en savoir plus : `rythme explain E011`

avertissement[W001]: `total` n'est jamais utilisé
 --> 52-verifier.ry:3:3
  |
3 |   total = aire(2)
  |   ^^^^^
  |
  = aide: supprimez-le, ou nommez-le `_total` si c'est voulu

✗ 52-verifier.ry : 4 erreur(s)
```

Code de sortie : 1. Et les cinq avertissements :

```rythme
type Feu = Rouge | Orange | Vert
fn suivant(Rouge) = Vert
fn suivant(Vert) = Orange
fn etiquette(n) = match n
  _ -> "n'importe"
  0 -> "zéro"
end
fn deux_fois(x)
  inutile = x * 2
  x + x
end
len = 3
print([1, 2].taille())
```

`rythme check 53-avertissements.ry` :

```text
avertissement[W002]: `suivant` ne traite pas tous les arguments possibles
 --> 53-avertissements.ry:2:4
  |
2 | fn suivant(Rouge) = Vert
  |    ^^^^^^^
  |
  = note: non traité : `Orange`
  = aide: ajoutez une définition pour ce cas, comme `fn suivant(Orange) = ...`

avertissement[W004]: ce cas ne peut jamais être atteint
 --> 53-avertissements.ry:6:3
  |
6 |   0 -> "zéro"
  |   ^
  |
5 |   _ -> "n'importe"
  |   - ce cas traite déjà tout
  |
  = aide: les cas sont essayés dans l'ordre : mettez `_` en dernier

avertissement[W001]: `inutile` n'est jamais utilisé
 --> 53-avertissements.ry:9:3
  |
9 |   inutile = x * 2
  |   ^^^^^^^
  |
  = aide: supprimez-le, ou nommez-le `_inutile` si c'est voulu

avertissement[W003]: `len` masque la fonction intégrée `len`
  --> 53-avertissements.ry:12:1
   |
12 | len = 3
   | ^^^
   |
   = aide: dans ce fichier, `len` désignera désormais votre définition ; choisissez un autre nom pour garder celle intégrée

avertissement[W005]: il n'existe pas de fonction nommée `taille`
  --> 53-avertissements.ry:13:14
   |
13 | print([1, 2].taille())
   |              ^^^^^^ inconnue
   |
   = note: `valeur.taille(...)` appelle `taille(valeur, ...)`, ou le champ `taille` d'un enregistrement
   = aide: en Rythme, cela s'appelle `len`

✓ 53-avertissements.ry : aucune erreur, 5 avertissement(s)
```

Code de sortie : 0.

### 5.6 Codes d'erreur et d'avertissement

Chaque erreur et chaque avertissement a un code. `rythme explain CODE` (ou
`rythme help CODE`, ou `:explain CODE` dans le mode interactif) donne une
explication détaillée avec des exemples ; le code peut s'écrire en
minuscules.

| Code | Nom | Quand |
|------|-----|-------|
| E001 | Erreur de syntaxe | le code ne suit pas la grammaire : une parenthèse ou un `then` manquant, des accolades pour un bloc, un mot-clé d'un autre langage, des comparaisons enchaînées, `_` utilisé comme valeur, `{nom, ..autres}`, une expression trop longue ou imbriquée trop profondément... |
| E002 | Texte jamais fermé | un `"` ou une `{` dans un texte n'est jamais fermé |
| E003 | Nom inconnu | un nom ou un constructeur qui n'est pas défini à cet endroit, ou une valeur utilisée avant sa définition |
| E004 | Mauvais nombre d'arguments | un appel ou un motif de constructeur avec trop ou trop peu de valeurs ; des définitions d'une même fonction avec des nombres de paramètres différents |
| E005 | Mauvaise sorte de valeur | une opération ou une fonction a reçu une sorte de valeur qu'elle n'accepte pas (`"a" + 1`, une condition qui n'est pas un Bool, des valeurs qu'on ne peut pas comparer...) |
| E006 | Aucun cas ne correspond | aucun cas d'un `match`, aucune définition d'une fonction, ou le motif d'une définition n'accepte pas la valeur |
| E007 | Indice hors limites | `liste[i]` ou `texte[i]` hors des positions valides |
| E008 | Division par zéro | `/`, `//` ou `%` par zéro, ou `0 ** -n` |
| E009 | Pas une fonction | on appelle une valeur qui n'est pas une fonction (`x(3)`, `None()`) |
| E010 | Champ inconnu | l'enregistrement ou la variante n'a pas de champ de ce nom |
| E011 | Déjà défini | un nom, un type, un constructeur ou un champ défini deux fois au même endroit, ou une tentative de modifier une valeur (`x += 1`) |
| E012 | Nombre trop grand | un nombre entier hors de l'intervalle des 64 bits, dans le code (dans n'importe quelle base) ou comme résultat (`-x` et `abs(x)` du plus petit Int compris) |
| E013 | `unwrap` n'a pas trouvé de valeur | `unwrap` sur `None` ou `Err(...)` |
| E014 | Trop d'appels imbriqués | des appels imbriqués plus profondément que la pile ne le permet |
| E015 | Assertion échouée | `assert`, `assert_eq`, ou un test qui s'est terminé avec `false`, `Err(...)` ou `None` |
| E016 | Mauvaise utilisation de `?` | `?` hors d'une fonction, ou sur une valeur qui n'est ni une Option ni un Result |
| E017 | Problème de fichier | un fichier illisible ou introuvable, un dossier donné à la place d'un fichier, des fichiers qui s'utilisent en cercle, ou `input()` à la fin de l'entrée |
| E018 | Caractère inattendu | un caractère ou un nombre qui n'a pas de sens ici (`'`, `&`, `2x`, `...`, `++`, une séquence d'échappement inconnue, un sosie comme `×` ou un caractère invisible copié depuis un document) |
| E019 | `if` sans `else` | un `if` sans `else` |
| E020 | Il manque `end` | une fonction, un `test`, un `match` ou un bloc `do` jamais fermé |
| E021 | Programme arrêté par `fail` | `fail(message)` a été appelé |
| E022 | Conversion impossible | `int(t)` ou `float(t)` sur un texte qui n'est pas un nombre |
| E023 | Valeur invalide | une valeur de la bonne sorte mais hors de ce qui est accepté (un nombre négatif d'éléments, `min` d'une liste vide, `sqrt(-1)`, un intervalle, une liste ou un texte trop grand, un code de sortie hors de 0–255...) |
| E024 | Une valeur est perdue | une ligne calcule une valeur dont rien ne se sert |
| W001 | Nom inutilisé | un nom local est défini mais jamais utilisé |
| W002 | Cas oublié | un `match` ou une fonction à un paramètre ne traite pas toutes les valeurs possibles |
| W003 | Fonction intégrée masquée | une définition du premier niveau prend le nom d'une fonction intégrée |
| W004 | Cas jamais atteint | un cas ou une définition vient après un autre qui accepte tout ou les mêmes valeurs, ou ne peut jamais correspondre à la valeur |
| W005 | Fonction inconnue après un point | `valeur.nom(...)` alors qu'aucune fonction ni aucun champ d'enregistrement ne porte ce nom |
| W006 | Comparaison toujours fausse | `==` ou `!=` entre des valeurs de natures qui ne sont jamais égales (`nombre == "3"`) |
| W007 | Test qui ne peut jamais échouer | un test qui ne fait que calculer ou afficher une valeur, sans jamais la comparer à ce qui est attendu |
| W008 | Résultat perdu | une ligne qui ne fait qu'appeler une fonction sans effet, comme `push(liste, x)` : les valeurs ne changent jamais |
| W009 | Fonction affichée | une fonction affichée dans un texte ou par `print` (`<fn nom>`) au lieu de ce qu'elle donne |

---

## 6. Bibliothèque standard

Les 124 fonctions intégrées sont toujours disponibles, sans `use`. Comme les
mots-clés, leurs noms sont en anglais. `rythme help builtins` les liste
toutes par catégorie, une ligne chacune ; `rythme help NOM` (ou `help(NOM)`
dans un programme, ou `:doc NOM` dans le mode interactif) affiche la
documentation de l'une d'elles, avec des exemples dont les valeurs sont
vérifiées par les tests de Rythme. Cette partie donne les conventions et ce
que contient chaque catégorie ; `rythme help NOM` est la référence pour le
détail de chaque fonction.

**Conventions.**

- La donnée vient en premier : `map(liste, f)`, `split(texte, separateur)`,
  `put(enregistrement, nom, valeur)`. C'est ce qui fait marcher `|>` et les
  appels en style méthode : `liste |> map(f)`, `liste.map(f)`.
- Les fonctions ne modifient jamais leurs arguments ; elles renvoient de
  nouvelles valeurs.
- Une fonction qui peut ne rien trouver renvoie une Option ; une fonction qui
  peut échouer pour des raisons extérieures au programme (un texte tapé par
  un utilisateur, un fichier) renvoie un Result. Les autres fonctions
  arrêtent le programme sur une entrée invalide : E005 pour une mauvaise
  sorte de valeur, E022 pour une conversion impossible, E023 pour une valeur
  hors de ce qui est accepté.
- Partout où une fonction est attendue, n'importe quelle fonction convient :
  une fonction nommée, anonyme, intégrée ou un constructeur
  (`map([1, 2], Some)`). Les fonctions qui testent des éléments (`filter`,
  `any`, `all`, `find`, `count`, `partition`) demandent une fonction qui
  renvoie un Bool (E005).
- Les textes des valeurs `Err(...)` fabriquées par les fonctions intégrées
  (par exemple par `parse_int`) sont écrits dans la langue des messages.
- Les erreurs reçoivent une indication : le nom d'une fonction d'un autre
  langage donne son nom en Rythme (`ord` → `char_code`, `padStart` →
  `pad_left`, `takeWhile` → `take_while`, `stringify` → `to_json`...), et
  une valeur d'une sorte voisine renvoie vers la bonne fonction (un texte
  donné là où une liste est attendue : `chars` ; un enregistrement :
  `entries`, `keys` ou `values` ; une liste donnée à `put`, `remove` ou
  `has` : `set_at`, `remove_at` ou `contains` ; une Option donnée à
  `flat_map` : `and_then`). Quand la nature des valeurs est certaine, ces
  erreurs sont trouvées avant l'exécution (voir
  [5.5](#55-la-vérification-avant-lexécution)).

**Bases.** `print(valeurs...)` et `debug(valeur)` (voir
[2.13](#213-affichage-des-valeurs)), `text(valeur)` (la forme affichée, sous
forme de texte) et `text(nombre, decimales)` (un nombre avec exactement ce
nombre de décimales, au plus 100 : `text(5, 2)` vaut `"5.00"`),
`input(invite)` (lit une ligne au clavier ; à la fin de l'entrée, l'erreur
E017), `type_of(valeur)`, `help(valeur)`, `assert(condition, message)`,
`assert_eq(obtenu, attendu)`, `fail(message)`, `args()` (les mots donnés
après le nom du fichier sur la ligne de commande), `env(nom)` (une variable
d'environnement, sous forme d'Option), `exit(code)`, `time()` (les secondes
depuis 1970, un Float), `random()` (un Float dans [0, 1[), `random(a, b)`
(un Int de a à b, inclus), `random(liste)` (un élément).

**Nombres.** `abs`, `min` et `max` (plusieurs valeurs, ou une liste), `sqrt`,
`pow(x, n)` (comme `**`), `floor`, `ceil` et `round(x)` (donnent un Int),
`round(x, chiffres)` (donne un Float), `int(x)` (coupe les décimales ; lit un
texte ; `true` vaut 1), `float(x)`, `sin`, `cos`, `tan`, `asin`, `acos`,
`atan(x)` et `atan(y, x)` (l'angle du point (x, y), de −π à π), `log(x)` et
`log(x, base)`, `exp`, `is_even`, `is_odd`, `gcd` et `lcm` (le PGCD et le
PPCM de deux nombres entiers), et la constante `pi`.

**Texte.** `len`, `upper`, `lower`, `capitalize` (le premier caractère en
majuscule), `trim`, `pad_left(x, largeur, remplissage)` et `pad_right` (pour
aligner des colonnes ; le remplissage est une espace par défaut),
`split(texte)` (aux espaces) et `split(texte, separateur)` (un séparateur
vide découpe en caractères), `join(liste, separateur)`, `contains`,
`starts_with`, `ends_with`, `replace(texte, ancien, nouveau)`, `chars`,
`lines`, `repeat(x, n)`, `char_code(c)` et `char(code)` (un caractère et son
numéro Unicode), `is_digit`, `is_letter` et `is_space` (le texte, non vide,
n'est-il fait que de tels caractères ?), `parse_int(texte)` et
`parse_float(texte)` (des Results ; les espaces autour du nombre et les `_`
sont acceptés), `parse_json(texte)` (un Result : les objets JSON deviennent
des enregistrements, les tableaux des listes, `null` `None` ; une erreur
donne la ligne et la colonne) et `to_json(valeur, indentation)` (le texte
JSON d'une valeur, sur une ligne, ou un élément par ligne décalé de
`indentation` espaces, au plus 16).

**Listes.** `map`, `filter`, `fold(liste, depart, f)`,
`scan(liste, depart, f)` (chaque résultat intermédiaire de `fold`), `each`,
`sum`, `product`, `first`, `last`, `rest`, `get(x, cle)`, `push(liste, x)`,
`set_at(liste, i, x)`, `insert_at(liste, i, x)` et `remove_at(liste, i)`
(les positions négatives partent de la fin), `reverse`, `sort`,
`sort_by(liste, cle)` (stable), `unique`, `shuffle`, `slice(x, debut, fin)`,
`take`, `drop`, `take_while(x, f)` et `drop_while(x, f)`, `step_by(x, n)`
(un élément sur n), `chunks(x, n)` (des morceaux de n éléments) et
`windows(x, n)` (chaque groupe de n éléments voisins), `zip`, `enumerate`,
`any`, `all`, `find`, `count`, `index_of`, `min_by(liste, cle)` et
`max_by(liste, cle)`, `flatten`, `flat_map`, `partition`,
`group_by(liste, cle)` (un enregistrement dont les noms de champs sont les
clés sous forme de textes), `is_empty`. Beaucoup acceptent aussi des textes
(`first`, `last`, `rest`, `get`, `reverse`, `slice`, `take`, `drop`,
`take_while`, `drop_while`, `step_by`, `chunks`, `windows`, `index_of`), des
enregistrements (`len`, `contains`, `get`, `is_empty`) ou des Options et des
Results (`map`).

**Enregistrements.** `keys`, `values`, `entries` (une liste de
`[nom, valeur]`) et `from_entries` (l'inverse), `has(enregistrement, nom)`,
`put(enregistrement, nom, valeur)`, `remove(enregistrement, nom)`,
`map_values(enregistrement, f)`.

**Option et Result.** `unwrap`, `unwrap_or(x, defaut)`, `is_some`,
`is_none`, `is_ok`, `is_err`, `and_then(x, f)` (une étape de plus qui peut
échouer : `f` donne une Option ou un Result), `ok_or(option, erreur)` (une
Option transformée en Result, prête pour `?`), `map_err(result, f)`, et les
constructeurs `Some`, `None`, `Ok`, `Err`.

**Fichiers.** `read_file(chemin)` (`Ok(texte)` ou `Err(message)`),
`write_file(chemin, valeur)` (écrit la forme affichée en remplaçant le
contenu : `Ok(())` ou `Err(message)`), `append_file(chemin, valeur)` (ajoute
à la fin, en créant le fichier si besoin), `file_exists(chemin)`. Les
chemins relatifs partent du dossier actuel.

```rythme
nombres = [3, 1, 4, 1, 5, 9, 2, 6]
print(map(nombres, fn(x) = x * 2))
print(filter(nombres, is_even), fold(nombres, 0, fn(acc, x) = acc + x), sum(nombres))
print(sort(nombres), unique(nombres), sort_by(["poire", "figue", "banane"], len))
print(first(nombres), last([]), get(nombres, -1), take(nombres, 3), drop(nombres, 6))
print(find(nombres, fn(x) = x > 4), any(nombres, fn(x) = x > 8), count(nombres, is_odd))
print(zip([1, 2], ["a", "b"]), enumerate(["x", "y"]), group_by(["ane", "bec", "abeille"], fn(m) = m[0]))
print(split("a,b,c", ","), join(["a", "b"], "-"), upper("été"), replace("2024", "4", "5"))
print(len("été"), chars("hé"), trim("  salut  "), contains("bonjour", "jour"), starts_with("rythme", "ry"))
print(parse_int("42"), parse_int("4x"), int("42"), float("2.5"), text(3.0))
print(abs(-3), min(3, 1, 2), max([4, 8]), round(3.14159, 2), floor(2.7), sqrt(16), 2 |> pow(10))
print(keys({a: 1, b: 2}), put({a: 1}, "b", 2), remove({a: 1, b: 2}, "a"), has({a: 1}, "a"))
print(unwrap_or(get([], 0), "défaut"), map(Some(2), fn(x) = x + 1), is_ok(Err("non")))
```

```text
[6, 2, 8, 2, 10, 18, 4, 12]
[4, 2, 6] 31 31
[1, 1, 2, 3, 4, 5, 6, 9] [3, 1, 4, 5, 9, 2, 6] ["poire", "figue", "banane"]
Some(3) None Some(6) [3, 1, 4] [2, 6]
Some(5) true 5
[[1, "a"], [2, "b"]] [[0, "x"], [1, "y"]] {a: ["ane", "abeille"], b: ["bec"]}
["a", "b", "c"] a-b ÉTÉ 2025
3 ["h", "é"] salut true true
Ok(42) Err("\"4x\" n'est pas un nombre entier") 42 2.5 3.0
3 1 8 3.14 2 4.0 1024
["a", "b"] {a: 1, b: 2} {b: 2} true
défaut Some(3) false
```

```rythme
print(gcd(12, 18), lcm(4, 6), atan(1, -1), text(3.14159, 2), text(5, 2))
print(capitalize("élise"), pad_left("7", 3, "0"), pad_right("ab", 4, "."), char_code("A"), char(66))
print(is_digit("42"), is_letter("été"), is_space(" \n"), take_while("42 cm", is_digit))
print(scan([1, 2, 3], 0, fn(total, x) = total + x), set_at([1, 2, 3], 0, 9), insert_at([1, 2], 1, 7), remove_at([1, 2, 3], -1))
print(drop_while([1, 3, 4, 5], is_odd), 0..10 |> step_by(5), chunks([1, 2, 3, 4, 5], 2), windows([1, 2, 3], 2))
personnes = [{nom: "Ada", age: 36}, {nom: "Alan", age: 41}]
print(min_by(["ccc", "a", "bb"], len), max_by(personnes, fn(p) = p.age).nom)
print(from_entries([["a", 1], ["b", 2]]), map_values({a: 1, b: 2}, fn(x) = x * 10))
print(parse_int("42") |> and_then(fn(n) = if n > 0 then Ok(n) else Err("pas positif")))
print(ok_or(None, "absent"), map_err(Err("introuvable"), fn(e) = "config : {e}"))
print(parse_json("[1, 2.5, null, \{\"ok\": true\}]"), to_json({nom: "Ada", etiquettes: ["x"], age: None}))
```

```text
6 12 2.356194490192345 3.14 5.00
Élise 007 ab.. 65 B
true true true 42
[1, 3, 6] [9, 2, 3] [1, 7, 2] [1, 2]
[4, 5] [0, 5, 10] [[1, 2], [3, 4], [5]] [[1, 2], [2, 3]]
a Alan
{a: 1, b: 2} {a: 10, b: 20}
Ok(42)
Err("absent") Err("config : introuvable")
Ok([1, 2.5, None, {ok: true}]) {"nom": "Ada", "etiquettes": ["x"], "age": null}
```

Un nom venu d'un autre langage et une valeur de la mauvaise sorte reçoivent
une indication, ici avant que le programme ne s'exécute :

```rythme
mot = "bonjour"
print(ord(mot[0]))
print(map(mot, upper))
```

```text
erreur[E003]: nom inconnu `ord`
 --> 62-indices.ry:2:7
  |
2 | print(ord(mot[0]))
  |       ^^^ non défini
  |
  = aide: en Rythme, cela s'appelle `char_code`
  = en savoir plus : `rythme explain E003`

erreur[E005]: `map` attend une liste, une Option ou un Result comme argument 1, mais a reçu une valeur de type Text
 --> 62-indices.ry:3:11
  |
3 | print(map(mot, upper))
  |           ^^^ mauvaise sorte de valeur
  |
  = aide: utilisation : `map(list, f)`
  = aide: un texte n'est pas une liste : `chars(texte)` donne la liste de ses caractères
  = en savoir plus : `rythme explain E005`
```

```rythme
fn ecrire_puis_lire(chemin)
  write_file(chemin, "ligne 1\nligne 2")?
  contenu = read_file(chemin)?
  Ok(lines(contenu))
end
print(ecrire_puis_lire("notes.txt"))
print(read_file("absent.txt"))
print(file_exists("notes.txt"), args())
```

`rythme 61-fichiers.ry Ada 36` :

```text
Ok(["ligne 1", "ligne 2"])
Err("fichier introuvable : absent.txt")
true ["Ada", "36"]
```

Quand il n'y a plus rien à lire (la saisie au clavier a été fermée par
Ctrl-D, ou le texte envoyé au programme est terminé ; ici, l'entrée est
vide), `input` arrête le programme avec E017, pour qu'un programme qui
redemande sans cesse ne tourne pas indéfiniment :

```rythme
print("Votre nom ?")
nom = input()
print("Bonjour, {nom} !")
```

```text
Votre nom ?
erreur[E017]: `input` a atteint la fin de l'entrée
 --> 63-saisie.ry:2:7
  |
2 | nom = input()
  |       ^^^^^^^ plus aucune ligne à lire
  |
  = note: la saisie au clavier a été fermée (Ctrl-D), ou le texte envoyé au programme est terminé
  = en savoir plus : `rythme explain E017`
```

---

## 7. Outils

### 7.1 La ligne de commande

```text
rythme [--lang fr|en] [--no-color] [COMMANDE] [ARGUMENTS...]
```

| Commande | Autres noms | Ce qu'elle fait |
|----------|-------------|-----------------|
| *(aucune)* | `repl` | le mode interactif (voir [7.2](#72-le-mode-interactif-repl)) |
| `FICHIER [ARGS...]` | `run FICHIER [ARGS...]`, `lancer` | vérifie puis exécute le programme ; `ARGS` est donné à `args()` |
| `check FICHIER...` | `verifier`, `vérifier` | vérifie chaque programme sans l'exécuter (voir [5.5](#55-la-vérification-avant-lexécution)) |
| `test FICHIER [ARGS...]` | `tester` | lance les tests (voir [4.8](#48-tests)) |
| `fmt [--check] FICHIER...` | `format`, `formater` | donne au code sa mise en forme officielle ; aussi un dossier, ou `-` pour l'entrée standard (voir [7.4](#74-la-mise-en-forme--rythme-fmt)) |
| `-e CODE [ARGS...]` | `eval CODE [ARGS...]` | exécute le code donné en argument, et affiche la valeur de sa dernière expression sauf si c'est `()` ; `ARGS` est donné à `args()` |
| `lsp` | | le serveur de langage, lancé par un éditeur (voir [7.5](#75-éditeurs-et-serveur-de-langage)) |
| `help [NOM]` | `aide`, `--help`, `-h` | la liste des sujets, ou l'aide d'un sujet, d'une fonction intégrée ou d'un code d'erreur |
| `explain [CODE]` | `expliquer` | l'explication d'un code d'erreur ou d'avertissement ; sans code, la liste des codes |
| `learn` | `apprendre`, `tutorial`, `tutoriel` | la visite guidée : les 13 sujets dans l'ordre, avec une pause après chacun quand elle tourne dans un terminal |
| `version` | `--version`, `-V` | affiche `rythme 0.1.0` |

**Trouver le fichier.** Un premier argument qui n'est pas une commande est
pris pour un fichier quand il finit par `.ry` ou qu'il désigne un fichier
existant ; si `nom` n'existe pas mais que `nom.ry` existe, c'est `nom.ry` qui
est utilisé. Un fichier qui ne finit pas par `.ry` s'exécute après un
avertissement. Un fichier introuvable est E017, un dossier aussi (le message
nomme des fichiers qu'il contient), ainsi qu'un fichier qui n'est pas du
texte en UTF-8. Tout le reste est une commande inconnue, avec une
suggestion.

**Options.** `--lang` et `--no-color` doivent venir **avant** la commande ;
tout ce qui suit le nom du fichier va au programme. `--lang` accepte toute
valeur qui commence par `fr` ou `en` (`--lang fr`, `--lang=en`,
`--lang fr_CA`) ; une autre valeur, ou `--lang` sans valeur, est signalée,
avec le code de sortie 2.

**Sujets d'aide.** `rythme help SUJET` accepte `start`, `syntax`,
`functions`, `lists`, `text`, `records`, `types`, `match`, `errors`,
`functional`, `tests`, `modules`, `repl`, `fmt`, `editors` et `builtins`,
ainsi que des noms français ou alternatifs : `syntaxe`, `fonctions`,
`listes`, `texte`, `enregistrements`, `motifs`, `erreurs`, `fonctionnel`,
`boucles`, `fichiers`, `interactif`, `formater`, `lsp`, `vscode`,
`éditeurs`, `bibliothèque`...

**Codes de sortie.**

| Code | Signification |
|------|---------------|
| 0 | succès ; `check` sans erreur (avertissements permis) ; tous les tests réussis, ou aucun test ; `fmt --check` avec tous les fichiers déjà en forme |
| 1 | une erreur dans le programme (syntaxe, vérification ou exécution), un test échoué, un fichier introuvable, ou des fichiers à mettre en forme pour `fmt --check` |
| 2 | une mauvaise utilisation de la commande : commande ou option inconnue, nom de fichier manquant, langue, sujet d'aide ou code d'erreur inconnu |
| 0 à 255 | le code donné à `exit(code)` par le programme |
| 101 | une erreur interne de Rythme (un bug, à signaler) |

**Environnement.**

- `RYTHME_LANG` : la langue des messages (`fr...` ou `en...`). Sans elle, la
  première variable non vide parmi `LC_ALL`, `LC_MESSAGES`, `LANG` et
  `LANGUAGE` décide : le français si elle commence par `fr`, l'anglais sinon.
  `--lang` l'emporte sur toutes.
- `NO_COLOR` : quand elle est définie (avec n'importe quelle valeur), les
  messages sont sans couleurs ; `--no-color` fait de même. Les couleurs ne
  sont utilisées que quand la sortie d'erreur est un terminal (sous Windows,
  seulement dans les terminaux connus pour les accepter).
- `HOME` (`USERPROFILE` sous Windows) : là où le mode interactif garde son
  historique.

**Sorties.** Ce que le programme affiche (`print`, l'invite de `input`, les
résultats de `check`, de `test` et de `fmt`) va sur la sortie standard ; les
messages d'erreur et d'avertissement et `debug` vont sur la sortie d'erreur.

```text
$ rythme version
rythme 0.1.0
$ rythme -e '[1, 2, 3] |> map(fn(x) = x * x)'
[1, 4, 9]
$ rythme -e 'print("bonjour")'
bonjour
$ rythme -e 'print(args())' Ada 36
["Ada", "36"]
$ rythme chek 01.ry
commande ou fichier inconnu `chek`
vouliez-vous dire `rythme check` ?
voir `rythme help`
$ rythme help lenght
Aucune aide trouvée pour `lenght`.
en Rythme, cela s'appelle `len`
Voyez la liste des sujets avec `rythme help`.
$ rythme --lang es bonjour.ry
langue inconnue `es` : écrivez `--lang fr` ou `--lang en`, comme `rythme --lang fr bonjour.ry`
```

Les trois dernières se terminent avec le code 2 (la dernière est écrite dans
la langue du système).

### 7.2 Le mode interactif (REPL)

`rythme` sans argument lance le mode interactif. L'invite est `ry> `, et
`..> ` tant qu'une saisie n'est pas finie. Chaque saisie est vérifiée et
exécutée comme un petit fichier ; la valeur de sa dernière expression est
affichée sous forme code (sauf si c'est `()`). Les définitions sont gardées
d'une saisie à l'autre.

Différences avec un fichier :

- on peut redéfinir un nom (pas de E011) ;
- une `fn` seule, avec le même nom et le même nombre de paramètres que la
  fonction définie par la saisie précédente, lui ajoute une définition (un
  cas) ;
- le vérificateur ne signale pas les cas oubliés des fonctions (W002) ;
- les chemins de `use` et de `:load` partent du dossier actuel.

Une saisie qui n'est pas finie (une parenthèse ouverte, un bloc sans `end`,
un texte non fermé...) continue à la ligne suivante ; deux lignes vides de
suite l'exécutent quand même, et à la fin de l'entrée (Ctrl-D) une saisie
restée inachevée est vérifiée, pour que son erreur soit affichée. Une ligne
qui commence par `|>` ou par `.nom` continue la saisie précédente : on peut
donc taper (ou coller) un enchaînement étape par étape. Ctrl-C annule la saisie en cours, Ctrl-D fait
quitter. La touche Tab complète les noms, les mots-clés et les commandes.
L'historique est gardé dans `~/.rythme_history`. Taper `help` (ou `aide`)
affiche l'aide du mode interactif ; `exit`, `quit` (ou `quitter`) fait
quitter.

| Commande | Autres noms | Ce qu'elle fait |
|----------|-------------|-----------------|
| `:help` | `:h`, `:?`, `:aide` | l'aide du mode interactif |
| `:quit` | `:q`, `:exit`, `:quitter` | quitter |
| `:doc NOM` | | la documentation d'un nom défini, d'une fonction intégrée ou d'un sujet |
| `:load FICHIER` | `:charger` | charger (à nouveau) un fichier et l'exécuter (`.ry` est ajouté si besoin) |
| `:reset` | | tout oublier |
| `:lang fr\|en` | `:langue` | changer la langue des messages |
| `:learn` | `:apprendre` | la visite guidée |
| `:explain CODE` | `:expliquer` | expliquer un code d'erreur |
| `:names` | `:noms` | les noms définis jusqu'ici |

Une session (les saisies apparaissent après les invites) :

```text
Rythme 0.1.0 — :help pour l'aide, :quit pour quitter
ry> 1 + 2
3
ry> nom = "Ada"
ry> "Bonjour, {nom}"
"Bonjour, Ada"
ry> nom = "Grace"
ry> fn fib(0) = 0
ry> fn fib(1) = 1
(`fib` a maintenant 2 définitions, essayées dans l'ordre ; :reset pour recommencer)
ry> fn fib(n) = fib(n - 1) + fib(n - 2)
(`fib` a maintenant 3 définitions, essayées dans l'ordre ; :reset pour recommencer)
ry> fib(20)
6765
ry> fn saluer(qui)
..>   message = "Bonjour, {qui} !"
..>   message
..> end
ry> saluer(nom)
"Bonjour, Grace !"
ry> [1, 2, 3]
[1, 2, 3]
ry> |> map(fn(x) = x * 10)
[10, 20, 30]
ry> :names
fib  nom  saluer
ry> :load geometrie
geometrie.ry ne s'exécute qu'une fois
geometrie.ry chargé
ry> aire_rectangle(2, 5)
10
ry> :reset
Tout a été oublié.
ry> fib(3)
erreur[E003]: nom inconnu `fib`
 --> <repl>:1:1
  |
1 | fib(3)
  | ^^^ non défini
  |
  = aide: définissez-le avant de l'utiliser : `fib = ...` ou `fn fib(...) = ...`
  = en savoir plus : `rythme explain E003`
```

### 7.3 Le playground web

Le [site](https://maxlestage.github.io/Rythme/) propose un playground (un bac
à sable en ligne) qui fait tourner le vrai interpréteur, compilé en
WebAssembly, dans le navigateur : le même langage, les mêmes messages et la
même aide, sans rien installer. Il a des boutons pour lancer (aussi
Ctrl+Entrée), vérifier et tester (aussi Ctrl+Maj+Entrée) le code, un bouton
**Formater** (aussi Maj+Alt+F) qui donne au code la mise en forme de
`rythme fmt` (voir [7.4](#74-la-mise-en-forme--rythme-fmt) ; un code qui a
une erreur est laissé tel quel, et l'erreur est affichée), un bouton pour
arrêter un programme qui ne s'arrête pas, le choix entre français et anglais,
un menu d'exemples, et un panneau d'aide (sujets, recherche, et explication
d'un code d'erreur en cliquant dessus).

Différences avec la commande `rythme` : les fichiers ne peuvent être ni lus
ni écrits (`read_file`, `write_file` et `append_file` renvoient `Err(...)`),
`input()` renvoie un texte vide, `args()` est vide, `env(nom)` vaut `None`,
`exit(code)` arrête le programme avec l'erreur E021, et les appels imbriqués
s'arrêtent à 600 (les appels terminaux n'ont pas de limite).

### 7.4 La mise en forme : `rythme fmt`

```text
rythme fmt [--check] FICHIER_OU_DOSSIER...
rythme fmt [--check] -
```

`rythme fmt` donne au code la mise en forme officielle de Rythme, pour que
tous les programmes se ressemblent. Il ne change que la disposition (les
espaces, les lignes vides, les retours à la ligne, les `;`, la virgule après
le dernier élément d'une liste écrite sur plusieurs lignes), jamais ce que
fait le programme : le résultat est relu, et doit donner le même programme
avec les mêmes commentaires, sinon le fichier est laissé tel quel.

- `rythme fmt bonjour.ry` réécrit le fichier (`bonjour` suffit quand
  `bonjour.ry` existe, comme avec `rythme run`) et écrit
  `mis en forme : bonjour.ry`. Un dossier désigne tous les fichiers `.ry`
  qu'il contient, ainsi que ses sous-dossiers, sauf les dossiers cachés comme
  `.git`. On peut donner plusieurs fichiers et dossiers ; un bilan suit.
- `--check` ne change rien : il liste les fichiers à mettre en forme et se
  termine avec le code 1 s'il y en a, pour l'intégration continue.
- `-` lit le code sur l'entrée standard et écrit le résultat sur la sortie
  standard.
- Un fichier qui a une erreur de syntaxe est laissé tel quel, et l'erreur est
  affichée (code de sortie 1).
- `format` et `formater` sont d'autres noms de `fmt`, `--verifier` de
  `--check`.

Le style, décrit par `rythme help fmt` : deux espaces par niveau ; des
espaces autour des opérateurs, de `=`, de `->` et de `|>`, aucun dans les
parenthèses et crochets ni autour de `..` ; un enchaînement écrit sur
plusieurs lignes a une étape par ligne, et un type écrit sur plusieurs lignes
une variante par ligne ; une liste ou un enregistrement écrit sur plusieurs
lignes finit par une virgule ; une instruction par ligne, et `end` sur sa
propre ligne ; une ligne vide autour d'une définition écrite sur plusieurs
lignes, et jamais deux lignes vides de suite ; chaque commentaire est gardé,
et les commentaires en fin de lignes consécutives sont alignés. Là où le
langage laisse le choix, les retours à la ligne sont gardés, et les lignes
longues sont gardées telles quelles.

Avec `brouillon.ry` :

```rythme
type Forme = Cercle(rayon)|Rect(largeur,hauteur)
fn aire(f)=match f
    Cercle(r)->pi*r*r
    Rect(l,h)->l*h
  end
prix=[12,5,30]
total=prix|>filter(fn(p)=p>10)
  |>sum
print( total ) ; print(aire(Rect(2,3)))
```

`rythme fmt - < brouillon.ry` écrit :

```text
type Forme = Cercle(rayon) | Rect(largeur, hauteur)

fn aire(f) = match f
  Cercle(r) -> pi * r * r
  Rect(l, h) -> l * h
end

prix = [12, 5, 30]

total = prix
  |> filter(fn(p) = p > 10)
  |> sum

print(total)
print(aire(Rect(2, 3)))
```

Dans un dossier qui contient `brouillon.ry` et un `propre.ry` déjà en forme :

```text
$ rythme fmt --check .
à mettre en forme : ./brouillon.ry
✗ 1 fichier(s) à mettre en forme, 1 déjà en forme ; `rythme fmt` (sans --check) les met en forme
$ rythme fmt .
mis en forme : ./brouillon.ry
1 fichier(s) mis en forme, 1 déjà en forme
$ rythme fmt --check .
✓ 2 fichier(s) déjà en forme
```

La première commande se termine avec le code 1, les autres avec le code 0.

### 7.5 Éditeurs et serveur de langage

`rythme lsp` est un serveur de langage (le Language Server Protocol, sur
l'entrée et la sortie standard) : un éditeur le lance tout seul et obtient,
pour les fichiers `.ry`, les problèmes pendant la frappe (toutes les erreurs
et tous les avertissements de `rythme check`, y compris ceux des fichiers
donnés à `use`), la documentation au survol (les fonctions intégrées avec un
exemple, les mots-clés, vos fonctions et vos types avec leurs commentaires
`##`, l'explication d'un code d'erreur), la complétion, les paramètres d'un
appel, aller à la définition, les références, renommer, le plan du fichier,
les corrections rapides des fautes de frappe, et la mise en forme comme
`rythme fmt`. Les messages sont dans la langue de l'éditeur, sauf si un
réglage de l'éditeur, `--lang` (`rythme --lang fr lsp`) ou `RYTHME_LANG` la
choisit. Lancé à la main dans un terminal, il dit à quoi il sert ;
`rythme lsp --help` affiche le même texte que `rythme help editors`.

- **VS Code** : l'extension dans [`editors/vscode`](../editors/vscode) ajoute
  les couleurs, l'indentation, des extraits de code et le serveur de langage,
  avec des commandes pour lancer le fichier, lancer ses tests ou expliquer un
  code d'erreur, et les réglages `rythme.path` (le programme) et
  `rythme.language`.
- **Les autres éditeurs** (Neovim, Helix, Zed, Emacs, Sublime Text,
  Kate...) : tout éditeur qui parle le Language Server Protocol peut lancer
  `rythme lsp` ; [`editors/README.fr.md`](../editors/README.fr.md) donne la
  configuration pour chacun.

---

## Annexe A. Grammaire

Cette grammaire, en EBNF, décrit la syntaxe acceptée par l'interpréteur.
`{ x }` signifie zéro ou plusieurs `x`, `[ x ]` un `x` facultatif. `NL` est
un retour à la ligne ou un `;` (plusieurs de suite comptent pour un). Dans
`( )`, `[ ]` et `{ }`, `NL` est ignoré, sauf dans un bloc imbriqué ; ailleurs,
les règles de continuation de
[1.9](#19-retours-à-la-ligne--et-lignes-de-continuation) s'appliquent. Une
virgule finale est permise dans toute liste séparée par des virgules.

```text
program      = { NL } { item ( NL | EOF ) } ;
item         = type_def | test_def | use_def | statement ;
statement    = fn_def | binding | expression ;

binding      = pattern_atom "=" expression ;
fn_def       = "fn" NAME "(" [ patterns ] ")" [ "if" expression ] body ;
body         = "=" expression
             | NL block_end ;
block_end    = { statement NL } [ statement ] "end" ;
type_def     = "type" CTOR "=" [ "|" ] variant { "|" variant } ;
variant      = CTOR [ "(" [ NAME { "," NAME } ] ")" ] ;
test_def     = "test" PLAIN_TEXT body ;
use_def      = "use" PLAIN_TEXT ;

expression   = or_expr ;
or_expr      = and_expr { "or" and_expr } ;
and_expr     = not_expr { "and" not_expr } ;
not_expr     = "not" not_expr | comparison ;
comparison   = pipe [ ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) pipe ] ;
pipe         = range { "|>" step } ;
step         = primary { "." MEMBER } [ "(" [ arguments ] ")" ] { postfix } ;
range        = sum [ ".." sum ] ;
sum          = product { ( "+" | "-" ) product } ;
product      = unary { ( "*" | "/" | "//" | "%" ) unary } ;
unary        = "-" unary | power ;
power        = primary { postfix } [ "**" unary ] ;
postfix      = "(" [ arguments ] ")" | "[" expression "]" | "?"
             | "." MEMBER [ "(" [ arguments ] ")" ] ;
arguments    = expression { "," expression } ;

primary      = INT | FLOAT | TEXT | "true" | "false" | NAME | CTOR
             | "(" ")" | "(" expression ")"
             | "[" [ list_item { "," list_item } ] "]"
             | "{" [ record_item { "," record_item } ] "}"
             | "fn" "(" [ patterns ] ")" body
             | "if" expression "then" expression "else" expression
             | "match" expression NL case { NL case } NL "end"
             | "do" block_end ;
list_item    = [ ".." ] expression ;
record_item  = ".." expression | KEY ":" expression | NAME ;
case         = pattern [ "if" expression ] "->" expression ;

patterns     = pattern { "," pattern } ;
pattern      = pattern_atom { "|" pattern_atom } ;
pattern_atom = "_" | NAME
             | CTOR [ "(" [ patterns ] ")" ]
             | [ "-" ] INT | [ "-" ] FLOAT | PLAIN_TEXT | "true" | "false"
             | "(" ")" | "(" pattern ")"
             | "[" [ list_patterns ] "]"
             | "{" [ field_pattern { "," field_pattern } ] "}" ;
list_patterns = ".." [ NAME ] [ "," ]
             | pattern [ "," [ list_patterns ] ] ;
field_pattern = KEY ":" pattern | NAME | ".." ;

MEMBER       = NAME | CTOR | mot-clé | mot réservé ;
KEY          = MEMBER | PLAIN_TEXT ;
```

Remarques :

- `binding` : quand une instruction commence par `(`, `[`, `{` ou un
  constructeur suivi de `(`, et que la parenthèse, le crochet ou l'accolade
  qui le ferme est suivi de `=`, la partie gauche est lue comme un motif.
  Sinon, l'instruction est lue comme une expression et, si `=` suit, cette
  expression est transformée en motif : seuls un nom, `_`, un constructeur
  sans champs et un littéral y sont acceptés (sinon E001, avec une
  indication pour `f(x) = ...`, `xs[0] = ...` et `e.x = ...`).
- `field_pattern` : `..` suivi d'un nom (`{nom, ..autres}`) est E001.
- Dans `match`, la valeur examinée doit finir sur la ligne de `match`, et les
  cas peuvent être séparés par des lignes vides ; `end` peut suivre le dernier
  cas sur la même ligne. Dans les blocs, `end` peut suivre la dernière
  instruction sur la même ligne.
- `then` et `else` peuvent être précédés de retours à la ligne ; les retours
  à la ligne sont sautés après `then`, `else`, `->`, `=` et chaque opérateur
  binaire.
- Dans `step`, un `.MEMBER (arguments)` fait un appel de méthode dont les
  arguments sont la valeur enchaînée suivie de `arguments` (voir
  [3.6](#36-enchaînements-)).
- Parenthèses, blocs et autres constructions s'imbriquent sur au plus 256
  niveaux, et une expression a au plus 1000 niveaux de profondeur, chaque
  opérateur, appel, indice et champ comptant pour un (E001, voir
  [l'annexe B](#annexe-b-limites-de-limplémentation)).

## Annexe B. Limites de l'implémentation

| Limite | Valeur | Erreur |
|--------|--------|--------|
| nombres entiers (Int) | de −2⁶³ à 2⁶³ − 1 | E012 |
| éléments d'une liste (construite par un intervalle, `+`, `..xs`, `repeat`, `flatten`, `flat_map`, `chars`) | 50 000 000 | E023 |
| longueur d'un texte (construit par `+`, `{...}`, `join`, `replace`, `repeat`) | 100 000 000 octets en UTF-8 | E023 |
| largeur donnée à `pad_left` et `pad_right` | 100 000 000 caractères | E023 |
| imbrication des parenthèses, crochets, blocs et autres constructions dans le code | 256 niveaux | E001 |
| profondeur d'une expression : chaque opérateur, appel, indice ou champ compte pour un niveau, donc `1 + 1 + ...` contient au plus 1000 nombres | 1000 niveaux | E001 |
| textes imbriqués dans des `{...}` de textes | 32 niveaux | E001 |
| appels imbriqués (non terminaux) | selon la fonction et la pile : plusieurs centaines de milliers avec `rythme` (environ 900 000 pour une récursion simple), 600 dans le playground | E014 |
| chiffres de `round(x, chiffres)` | ramenés entre −15 et 15 | — |
| décimales de `text(x, decimales)` | de 0 à 100 | E023 |
| indentation de `to_json(valeur, indentation)` | de 0 à 16 espaces | E023 |
| codes de sortie | de 0 à 255 | E023 |

Les appels terminaux et les fonctions de listes (`map`, `fold`, `sort`...) ne
comptent pas comme des appels imbriqués, et `parse_json` et `to_json` lisent
et écrivent des données imbriquées sur des centaines de milliers de niveaux
(indentées par `to_json(valeur, indentation)`, de telles données seraient
trop grandes : E023).
