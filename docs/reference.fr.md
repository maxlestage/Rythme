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
[4.7](#47-types)). `rythme doc` en tire la documentation de fichiers entiers
(voir [7.6](#76-la-documentation--rythme-doc)).

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

**Noms qui finissent par `?`.** Le nom d'une fonction qui répond à une
question peut finir par `?`, comme en Ruby : `even?`, `empty?`,
`fn majeur?(personne) = ...`. Comme `?` est aussi l'opérateur qui s'arrête
sur `None` ou `Err` ([5.2](#52-lopérateur-)), la règle est précise : un nom
suivi directement de `?` (sans espace entre les deux) est un seul nom qui
finit par `?`

- après `fn` (`fn majeur?(p)`), ou quand `(` suit directement le `?`
  (`majeur?(ada)`, `xs.include?(3)`) ;
- sinon, quand une fonction de ce nom existe : une fonction intégrée
  (`even?`, `empty?`... voir [6](#6-bibliothèque-standard)), ou une fonction
  définie avec `fn` dans le même fichier.

Dans tous les autres cas, `?` est l'opérateur : `valeur?`, `parse_int(t)?`,
`x |> parse_int?`. Pour appliquer l'opérateur à une valeur dont le nom est
aussi celui d'une telle fonction, écrivez `(valeur)?` ou `valeur ?`. Seules
les fonctions peuvent avoir un nom qui finit par `?` (E001 pour `ok? = 1`),
et `is_even` et les autres noms marchent toujours.

```rythme
fn majeur?(personne) = personne.age >= 18
ada = {nom: "Ada", age: 36}
print(majeur?(ada), ada.majeur?, even?(4), [1, 2].include?(2), [].empty?)
print([1, 2, 3, 4].select(even?))
fn premier_double(xs)
  x = first(xs)?             # `first` n'est pas suivi de `(` : l'opérateur
  Some(x * 2)
end
print(premier_double([5]), premier_double([]))
```

```text
true true true true true
[2, 4]
Some(10) None
```

### 1.5 Mots-clés et mots réservés

Rythme a 15 mots-clés :

```text
and  do  else  end  false  fn  if  match  not  or  test  then  true  type  use
```

Ils ne peuvent pas servir de noms. Ils peuvent servir de **noms de champs**,
après un point ou devant `:` dans un enregistrement (`e.type`, `{type: 1}`),
mais pas dans la forme courte `{type}`. `as` n'est pas un mot-clé : il n'a
de sens qu'après le chemin d'un `use` (`use geometrie as geo`). `unless`
n'en est pas un non plus : il commence `unless condition then a else b`
([3.9](#39-if)) seulement quand le début d'une valeur le suit sur la même
ligne (un nom, un nombre, un texte, un symbole, `not`, `if`, `match`, `fn`,
ou un crochet après une espace) ; `unless(x)` et `unless[0]`, sans espace,
utilisent une valeur nommée `unless`.

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
(`1_000_000`, `0xFF_FF`). Les nombres entiers n'ont pas de limite de taille
(voir 2.1) : un littéral de plus de 100000 chiffres est l'erreur E023, dans
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
print(9223372036854775807, -9223372036854775808, 123456789012345678901234567890)
```

```text
42 -7 1000000 255 10
3.14 2.0 1000.0 0.0015 6.02e23
9223372036854775807 -9223372036854775808 123456789012345678901234567890
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
| `\#` | `#` ; `\#{` donne `#{`, écrit tel quel |
| `\u{e9}` | le caractère de ce code hexadécimal (ici é) |

Toute autre séquence `\` est l'erreur E018.

**Insertion.** `{expression}` dans un texte insère la valeur de
l'expression, affichée comme le ferait `print` (les textes sans guillemets).
Toute expression est permise, enregistrements et autres textes compris ; les
accolades qu'elle contient sont appariées, et les retours à la ligne entre
les accolades sont ignorés. Un `{}` vide est une erreur (E001). Pour écrire
une `{`, utilisez `\{`. Une `}` qui ne ferme pas d'insertion est un caractère
ordinaire (`\}` est aussi accepté). Comme en Ruby, `#{expression}` est
pareil que `{expression}` ; `\#{` écrit `#{` tel quel.

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
print("style Ruby : #{nom}, #{2 * 21}, \#{nom}")
```

```text
Bonjour, Ada ! 2 + 2 = 4
tab :	| guillemet : " | barre : \ | accolades : { et }
été 🎵
un texte peut tenir
sur plusieurs lignes
1 et imbriqué Ada
style Ruby : Ada, 42, #{nom}
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
| `Int` | nombres entiers, de toute taille | `42`, `-7`, `0xFF` |
| `Float` | nombres décimaux, 64 bits | `3.14`, `2.0`, `1e3` |
| `Text` | textes | `"bonjour"`, `"total : {x}"` |
| `Bool` | `true`, `false` | |
| `Unit` | `()`, « pas de valeur intéressante » | `()` |
| `Symbol` | des noms utilisés comme valeurs | `:ok`, `:erreur` |
| `List` | suites ordonnées | `[1, 2, 3]`, `1..5`, `[]` |
| `Record` | enregistrements : des champs nommés | `{nom: "Ada", naissance: 1815}` |
| `Dict` | dictionnaires : une valeur pour chaque clé, des clés de toute sorte | `dict([["Ada", 36]])`, `dict()` |
| `Set` | ensembles : des valeurs sans répétition | `set([1, 2, 3])`, `set()` |
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

Des nombres entiers de toute taille, toujours exacts : ils grandissent autant
que nécessaire, comme en Python ou en Ruby, et ne « font jamais le tour ».
Les nombres habituels, de −9223372036854775808 à 9223372036854775807
(64 bits), sont calculés le plus vite ; au-delà, le résultat est encore un
Int, avec tous ses chiffres. `+`, `-`, `*`, `//`, `%`, `**`, les comparaisons
(avec les Floats aussi, voir 2.11), `abs`, `gcd`, `lcm`, `is_even`, `text`,
`int`, `parse_int`, `to_json` et `parse_json` sont exacts sur tous ; une
fonction qui a besoin d'une position ou d'un nombre d'éléments (`take`,
`get`, `repeat`...) refuse un nombre au-delà de 64 bits (E023), et `[...]`
dit qu'il est hors limites (E007).

```rythme
grand = 9223372036854775807
print(grand + 1, grand * grand)
fn factorielle(n) = if n == 0 then 1 else n * factorielle(n - 1)
print(factorielle(25), 2 ** 100)
print(-(2 ** 64) // 3, -(2 ** 64) % 3, (2 ** 64) / 3)
print(type_of(2 ** 64), 2 ** 64 - 2 ** 64 + 1)
```

```text
9223372036854775808 85070591730234615847396907784232501249
15511210043330985984000000 1267650600228229401496703205376
-6148914691236517206 2 6.148914691236517e18
Int 1
```

Un nombre entier a au plus 100000 chiffres : au-delà, le programme s'arrête
avec E023 plutôt que de remplir la mémoire (un nombre aussi grand vient en
général d'un calcul qui grandit sans fin).

```rythme
print(len(text(10 ** 99999)))
print(10 ** 100000)
```

```text
100000
erreur[E023]: ce nombre entier serait trop grand (plus de 100000 chiffres)
 --> 11-entier.ry:2:10
  |
2 | print(10 ** 100000)
  |          ^^
  |
  = aide: grandit-il sans fin, dans une fonction qui s'appelle elle-même ? Pour une valeur approchée, utilisez un nombre décimal (Float), comme `2.0 ** 1000`
  = en savoir plus : `rythme explain E023`
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

Un texte contient au plus 100 Mo (100 000 000 octets) en UTF-8 : 1 octet
pour une lettre sans accent, un chiffre ou une espace, 2 pour `é`, 4 pour un
émoji, donc 100 000 000 caractères pour un texte sans accents. En construire
un plus long, avec `+`, `{...}`, `join`, `replace`, `repeat`, `pad_left` ou
`pad_right`, est l'erreur E023. Cette limite, comme celle de la longueur
des listes (voir [2.6](#26-list)), arrête une valeur qui grandit sans fin
avant que la mémoire de l'ordinateur ne soit épuisée :

```rythme
fn doubler_texte(t) = doubler_texte(t + t)   # ne s'arrête jamais
doubler_texte("ab")
```

```text
erreur[E023]: ce texte serait trop grand (plus de 100 Mo)
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
- deux dictionnaires sont égaux quand ils ont les mêmes clés avec des valeurs
  égales, et deux ensembles quand ils ont les mêmes éléments ;
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
  par champ ;
- les ensembles, élément par élément dans leur ordre, comme des listes ; et
  les dictionnaires de même, comme les listes de leurs paires
  `[clé, valeur]`. Ce n'est pas l'inclusion : `set([1, 2]) < set([1, 3])`
  vaut `true` (utilisez `is_subset` pour l'inclusion).

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
| Dict | `dict("Ada" => 36, [0, 0] => "départ")`, `dict()` : les paires dans l'ordre des clés |
| Set | `set(1, 2, 3)`, `set()` : les éléments dans l'ordre |
| fonction | `<fn nom>` pour une fonction nommée ou intégrée, `<fn>` pour une fonction anonyme |
| constructeur avec champs | `<constructeur Cercle>` (en anglais : `<constructor Cercle>`) |
| Symbol | `:ok` |

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

### 2.14 Symbol

Un **symbole** est un nom utilisé comme valeur, comme en Ruby : `:ok`,
`:erreur`, `:en_attente`. Il s'écrit `:` suivi directement d'un nom, et
s'affiche de la même façon. Deux symboles sont égaux quand leurs noms le
sont ; ils sont ordonnés par nom ; ils ne sont jamais égaux à un texte
(`:ok == "ok"` est faux, et le vérificateur avertit, W006). On les utilise
dans les motifs de `match`, et `to_json` les écrit comme des textes.

Un `:` est celui d'un enregistrement quand il suit un nom de champ qui suit
`{` ou `,` (`{etat:ok}` est le champ `etat` avec la valeur de `ok`) ;
partout ailleurs, `:nom` est un symbole : `{etat: :ok}`, `[:a, :b]`,
`f(:ok)`.

```rythme
etat = :en_attente
fn etiquette(e) = match e
  :ok -> "fait"
  :en_attente -> "en attente"
  _ -> "inconnu"
end
print(etat, etiquette(etat), type_of(:ok), :a < :b, :ok == :ok)
print({etat: :ok, nombre: 2}, to_json([:ok, 1]), text(:ok))
```

```text
:en_attente en attente Symbol true true
{etat: :ok, nombre: 2} ["ok", 1] :ok
```

### 2.15 Dict et Set

Un **dict** (dictionnaire) donne une valeur pour chacune de ses clés ; un
**set** (ensemble) contient des valeurs sans répétition. Comme toutes les
valeurs, ils ne changent jamais. Ils n'ont pas de syntaxe propre : ils se
construisent avec les fonctions `dict` et `set`, et s'utilisent avec les
fonctions de la [bibliothèque standard](#6-bibliothèque-standard).

- `dict()` est le dictionnaire vide ; `dict(paires)` prend une liste de
  paires `[clé, valeur]` (quand une clé revient plusieurs fois, sa dernière
  valeur l'emporte) ; `dict(enregistrement)` prend les noms des champs comme
  clés (des textes).
- `set()` est l'ensemble vide ; `set(liste)` garde chaque élément une fois.
- Les clés et les éléments peuvent être n'importe quelle valeur sauf une
  fonction ou `nan` (E023) : nombres (entiers de toute taille aussi), textes,
  symboles, Bool, `()`, listes, enregistrements, variantes, dictionnaires et
  ensembles. Deux clés sont la
  même clé quand elles sont égales (`==`, voir [2.11](#211-égalité)) : `1` et
  `1.0` sont une seule clé, tout comme `{x: 1, y: 2}` et `{y: 2, x: 1}` ; la
  clé qui reste est celle mise en premier.
- Ils sont gardés **dans l'ordre de leurs clés**, quel que soit l'ordre dans
  lequel ils ont été construits : l'ordre de [2.12](#212-ordre), étendu à
  toutes ces valeurs (des valeurs de sortes différentes vont dans l'ordre
  `()`, Bool, nombres, textes, symboles, listes, enregistrements, variantes,
  dictionnaires, ensembles ; les symboles selon leurs noms ; les
  enregistrements selon leurs noms de champs
  dans l'ordre alphabétique, puis selon leurs valeurs). Cet ordre sert à
  l'affichage, à `keys`, `values`, `entries`, `filter`, `map_values` et
  `to_json`.
- `dico[clé]` donne la valeur d'une clé, E010 s'il n'y en a pas (voir
  [3.7](#37-indices)) ; `get(dico, clé)` donne une Option.
- Ce sont des arbres équilibrés persistants : `get`, `has`, `[...]`, `put`,
  `push` et `remove` prennent un temps en O(log n), et un nouveau
  dictionnaire ou ensemble partage presque tout l'ancien au lieu de le
  copier.
- Ils s'affichent comme `dict(clé => valeur, ...)` et `set(a, b, ...)`. Ces
  formes montrent seulement ce qu'ils contiennent : dans le code, écrivez
  `dict([[clé, valeur], ...])` et `set([a, b, ...])`.

```rythme
ages = dict([["Bob", 25], ["Ada", 36]])
print(ages, ages["Ada"], get(ages, "Eve"), len(ages))
plus_vieux = put(ages, "Bob", 26)
print(plus_vieux, ages)
print(set([3, 1, 3, 2]), set([1, 1.0]), dict({x: 1, y: 2}))
grille = dict([[[0, 0], "départ"], [[2, 1], "arrivée"]])
print(grille[[2, 1]], keys(grille))
print(type_of(ages), type_of(set()), dict() == dict([]))
```

```text
dict("Ada" => 36, "Bob" => 25) 36 None 2
dict("Ada" => 36, "Bob" => 26) dict("Ada" => 36, "Bob" => 25)
set(1, 2, 3) set(1) dict("x" => 1, "y" => 2)
arrivée [[0, 0], [2, 1]]
Dict Set true
```

```rythme
type Couleur = Rouge | Vert
print(dict([[1, "a"]]) == dict([[1.0, "a"]]), set([[1, 2], [1]]) == set([[1], [1, 2]]))
print(set([1, 2]) < set([1, 3]), sort([set([2]), set([1, 3]), set()]))
print(set([Vert, Rouge, 2, "x", [1], ()]))
```

```text
true true
true [set(), set(1, 3), set(2)]
set((), 2, "x", [1], Rouge, Vert)
```

Dans le style Ruby (voir [3.14](#314-blocs-donnés-aux-appels)), `each`, `select`
et `reject` donnent à un bloc la clé et la valeur de chaque paire d'un
dictionnaire, `key?` et `include?` sont `has` et `contains`, et `size` et
`length` sont `len`. Les symboles et les entiers de toute taille sont des
clés comme les autres :

```rythme
ages = dict([[:ada, 36], [:alan, 41]])
ages.each do |nom, age|
  print(nom, age)
end
print(ages.key?(:ada), ages.size, ages.select { |_, age| age > 40 })
vus = set([10 ** 20, 1, 10 ** 20])
print(vus, vus.include?(1), vus.length, to_json(ages))
```

```text
:ada 36
:alan 41
true 2 dict(:alan => 41)
set(1, 100000000000000000000) true 2 {"ada": 36, "alan": 41}
```

Une fonction ne peut pas être une clé (les fonctions ne se comparent pas) :

```rythme
fn double(x) = x * 2
print(set([1, double]))
```

```text
erreur[E023]: `set` : <fn double> ne peut pas être une clé de dictionnaire, ni un élément d'ensemble
 --> 16-ensemble.ry:2:7
  |
2 | print(set([1, double]))
  |       ^^^^^^^^^^^^^^^^
  |
  = aide: les clés et les éléments doivent être des valeurs comparables : tout sauf les fonctions et `nan`
  = en savoir plus : `rythme explain E023`
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
| `a + b` | Int, exact à toute taille | Float | deux textes : mis bout à bout ; deux listes : mises bout à bout ; sinon E005 |
| `a - b`, `a * b` | Int, exact à toute taille | Float | E005 |
| `a / b` | **toujours un Float** (`6 / 3` vaut `2.0`) | Float | E005 |
| `a // b` | Int, arrondi vers le bas (vers −∞) | Float, arrondi vers le bas | E005 |
| `a % b` | Int, du signe de `b` | Float : le reste exact, du signe de `b` | E005 |
| `a ** b` | Int si `b >= 0`, exact à toute taille, Float si `b < 0` (`0 ** -n` est E008) | Float (E023 pour un nombre négatif à une puissance fractionnaire) | E005 |
| `-a` | Int | Float | E005 |

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
Dans un motif, `1..9` n'est pas une liste mais n'importe quel nombre de 1 à 9
(voir [3.10](#310-match-et-motifs)).

### 3.5 Appels

`f(a, b)` évalue `f`, puis les arguments de gauche à droite, puis appelle la
fonction. Une virgule finale est permise, et les retours à la ligne entre les
parenthèses sont ignorés. Le nombre d'arguments doit être exactement le
nombre de paramètres (E004 ; certaines fonctions intégrées acceptent un
nombre variable d'arguments). Appeler une valeur qui n'est pas une fonction
est l'erreur E009, et `None()` aussi : un constructeur sans champs est une
valeur, pas une fonction. Il n'y a ni arguments nommés, ni valeurs par
défaut, ni application partielle. Une instruction, ou un résultat comme une
branche de `if`, peut aussi appeler une fonction sans parenthèses : `print x, y` (voir [3.15](#315-appels-sans-parenthèses)).

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
| `dico[clé]` | la valeur de cette clé | E010 si elle n'existe pas |

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

`valeur.nom` **sans parenthèses** lit le champ `nom` quand la valeur en a un
(un champ d'enregistrement, ou un champ d'une variante) ; quand elle n'en a
pas, il appelle la fonction `nom` avec la valeur : `xs.sum` vaut `sum(xs)`,
`"salut".upcase` vaut `upcase("salut")`, `forme.aire` vaut `aire(forme)`
quand `Cercle` n'a pas de champ `aire`. Le champ gagne toujours : les
enregistrements gardent leur sens. Un nom qui finit par `?` marche de la même
façon : `xs.empty?`, `n.even?`. Quand il n'y a ni champ ni fonction, c'est
l'erreur E010.

Le vérificateur avertit (W005) quand aucune fonction ni aucun champ
d'enregistrement du fichier ne porte ce nom. Une ligne qui commence par `.`
continue la ligne précédente : on peut donc enchaîner les appels de méthode
sur plusieurs lignes, une étape par ligne, comme on écrit en Ruby (des lignes
vides et des commentaires peuvent venir entre les étapes).

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

Sans parenthèses, et sur plusieurs lignes :

```rythme
xs = [3, 1, 4, 1, 5, 9, 2, 6]
print(xs.sum, xs.size, xs.sort.first, "rythme".upcase, xs.uniq.size)
top = xs
  .sort
  .reverse
  # les trois plus grands
  .first(3)
print(top)
point = {x: 1, sum: "un champ"}
print(point.sum)                        # le champ gagne sur la fonction
```

```text
31 8 Some(1) RYTHME 7
[9, 6, 5]
un champ
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

**`unless`**, comme en Ruby, est le contraire de `if` :
`unless condition then a else b` donne `b` quand la condition est vraie et
`a` quand elle est fausse ; c'est exactement `if condition then b else a`.
Comme `if`, il a toujours un `else` (E019). Voir
[1.5](#15-mots-clés-et-mots-réservés) pour savoir quand le mot `unless` le
commence.

```rythme
fn decrire(stock) = unless stock > 0 then "épuisé" else "{stock} restant(s)"
print(decrire(0), decrire(3))
```

```text
épuisé 3 restant(s)
```

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
prouver (W002), les cas placés après un cas qui accepte tout ou les mêmes
valeurs, et les intervalles vides (W004), et les intervalles qui se
chevauchent (W010).

| Motif | Correspond à |
|-------|--------------|
| `_` | n'importe quoi ; ne donne aucun nom |
| `nom` | n'importe quoi ; le nomme `nom` |
| `42`, `-1`, `2.5`, `-0.5` | un nombre égal à celui-ci (un motif Int correspond à un Float égal, et inversement : `0` correspond à `0.0`) |
| `1..9`, `-5..-1`, `0.0..0.5` | un nombre du premier au second, les deux inclus : un Int ou un Float (`89.5` correspond à `0..100` mais ni à `0..89` ni à `90..100`). Les bornes sont des nombres écrits dans le code |
| `"texte"` | exactement ce texte (pas de `{...}` dans un motif) |
| `"M. " + nom`, `"#" + _` | un texte qui commence par ce texte ; `nom` désigne la suite du texte (après `+` vient un nom ou `_`) |
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

Les intervalles et le début d'un texte :

```rythme
fn note(points) = match points
  90..100 -> "A"
  70..89 -> "B"
  0..69 -> "C"
  _ -> "pas une note"
end

fn piece(0.0..0.5) = "pile"
fn piece(_) = "face"

fn poli("M. " + nom) = "Cher Monsieur {nom}"
fn poli("Mme " + nom) = "Chère Madame {nom}"
fn poli(nom) = "Bonjour {nom}"

print([95, 70, 100.0, 89.5, -3] |> map(note))
print(piece(0.25), piece(0.75), piece(-2))
print(poli("M. Martin"), "/", poli("Mme Leroy"), "/", poli("Ada"))
```

```text
["A", "B", "A", "pas une note", "pas une note"]
pile face face
Cher Monsieur Martin / Chère Madame Leroy / Bonjour Ada
```

Un intervalle ne couvre jamais tous les nombres : un `match` sur des nombres
a donc toujours besoin d'un dernier cas `_` (W002). Le vérificateur signale,
avant l'exécution, un intervalle qui a des nombres en commun avec un
intervalle placé avant lui (W010 : ces nombres vont toujours au premier), un
cas dont tous les nombres sont déjà traités par les cas placés avant lui,
même par plusieurs ensemble (W004), et un intervalle vide comme `99..65`
(W004) :

```rythme
fn tranche(age) = match age
  0..18 -> "jeune"
  18..64 -> "adulte"
  20..30 -> "vingtaine"
  99..65 -> "senior"
  _ -> "plus âgé"
end
print(tranche(18))
```

```text
avertissement[W010]: ces intervalles se chevauchent
 --> 54-intervalles.ry:3:3
  |
3 |   18..64 -> "adulte"
  |   ^^^^^^ 18 est aussi ici
  |
2 |   0..18 -> "jeune"
  |   ----- 18 va ici, au premier qui convient
  |
  = aide: les deux bouts d'un intervalle sont inclus : faites se suivre les intervalles sans qu'ils se chevauchent, comme `0..9` puis `10..19`

avertissement[W004]: ce cas ne peut jamais être atteint
 --> 54-intervalles.ry:4:3
  |
4 |   20..30 -> "vingtaine"
  |   ^^^^^^
  |
3 |   18..64 -> "adulte"
  |   ------ ce cas traite déjà ces valeurs
  |
  = aide: une valeur va au premier cas qui lui convient : supprimez ce cas, ou modifiez-le

avertissement[W004]: cet intervalle ne correspond jamais
 --> 54-intervalles.ry:5:3
  |
5 |   99..65 -> "senior"
  |   ^^^^^^ aucun nombre ne va de 99 jusqu'à 65
  |
  = aide: un intervalle va du plus petit nombre au plus grand : `65..99`

jeune
```

Seul le début d'un texte peut servir de motif : pour sa fin, ou pour des
bornes qui sont des noms, utilisez une garde (`t if ends_with(t, "!") -> ...`,
`n if n >= bas -> ...`).

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

### 3.14 Blocs donnés aux appels

Comme en Ruby, on peut donner une fonction à un appel sous forme de **bloc**,
écrit après l'appel : `do |paramètres| ... end`, ou `{ |paramètres| ... }`.
Un bloc est le **dernier argument** de l'appel, et c'est exactement une
fonction anonyme : `xs.map do |x| x * 2 end` vaut `map(xs, fn(x) = x * 2)`.

- Il suit un appel (`map(xs) do ... end`), un appel de méthode
  (`xs.reduce(0) do |somme, x| ... end`), un `.nom` sans parenthèses
  (`xs.each do |x| ... end` vaut `xs.each(fn(x) = ...)`), ou un nom
  (`lancer do ... end` vaut `lancer(fn() = ...)`).
- `do` (ou `{`) doit être sur la ligne de l'appel ; le bloc peut ensuite
  continuer sur plusieurs lignes, jusqu'à `end` (ou `}`), ou tenir sur une
  ligne.
- Les paramètres vont entre deux barres, séparés par des virgules ; ce
  peuvent être des motifs (`|[cle, valeur]|`, `|{nom, ..}|`, `|Some(x)|`) ;
  sans paramètres, écrivez `do ... end` ou `{ || ... }`.
- `{ |x| ... }` n'est jamais un enregistrement : un enregistrement ne
  commence jamais par `|`. Un bloc écrit seul, pas après un appel, est
  l'erreur E001.
- La valeur du bloc est sa dernière ligne, comme dans une fonction. La suite
  de l'enchaînement s'applique au résultat de l'appel :
  `xs.map do |x| x end.sum`.

```rythme
noms = ["Ada", "Grace", "Alan"]
noms.each do |nom|
  print("Bonjour, #{nom} !")
end
print(noms.map { |n| n.size })
total = [1, 2, 3].reduce(0) do |somme, x|
  somme + x
end
print(total, [[1, 2], [3, 4]].map do |[a, b]| a * b end)
noms.each_with_index do |nom, i| print(i, nom) end
```

```text
Bonjour, Ada !
Bonjour, Grace !
Bonjour, Alan !
[3, 5, 4]
6 [2, 12]
0 Ada
1 Grace
2 Alan
```

### 3.15 Appels sans parenthèses

Comme en Ruby, une **instruction** peut appeler une fonction sans
parenthèses : `puts "Bonjour"` est `puts("Bonjour")`, et `print x, y` est
`print(x, y)`.

- L'instruction commence par un nom (`even?` aussi), suivi sur la même
  ligne du début de son premier argument : un nombre, un texte, un symbole,
  un nom, un constructeur, `true`, `false`, `not`, `if`, `match` ou `fn`.
  Les arguments sont des expressions séparées par des virgules, jusqu'à la
  fin de la ligne ; un retour à la ligne peut suivre une virgule.
- Seule une instruction commence ainsi : une ligne du premier niveau, ou
  d'une fonction, d'un bloc ou d'un test ; ou un **résultat**, qui contient
  une expression entière comme une instruction : une branche de `if` ou de
  `unless` (`if ok then puts "oui" else puts "non"`), ce que donne un cas de
  `match` (`"Dr " + nom -> puts nom`), et le corps après le `=` d'une
  fonction ou d'un test (`fn saluer(n) = puts "Salut #{n}"`,
  `fn(x) = puts x`, `test "somme" = assert_eq 1 + 1, 2`). Les arguments
  d'un résultat s'arrêtent là où l'expression s'arrêterait : à `else`,
  `end`, `}`, `)` ou à un retour à la ligne (donc au cas suivant d'un
  `match`). Partout ailleurs dans une expression, les parenthèses restent :
  `x = len(xs)`, `print(len(xs))`, `1 + f(x)`.
- Une virgule après les arguments appartient à l'appel, entre parenthèses
  aussi : `print(if ok then puts 1, 2 else 0)` donne `1, 2` à `puts`.
  Écrivez `puts(1)` pour terminer l'appel avant la virgule.
- Un bloc `do` écrit après les arguments est donné à l'appel lui-même,
  comme dernier argument : `each xs do |x| ... end`. Un bloc `{ |x| ... }`
  appartient à l'expression juste avant lui, comme partout.
- `(` garde son sens : `puts (1 + 2) * 3` est `puts(1 + 2) * 3`.
- `[` et `-` peuvent se lire de deux façons. Comme en Ruby, un `[` après une
  espace (`puts [1, 2]`), et un `-` après une espace mais juste avant un
  nombre ou un nom (`puts -1`), commencent les arguments, **sauf si le code
  donne une valeur à ce nom** : avec `=` (`x = 5`, `f = fn(v) = v`,
  `[a, b] = xs`), comme paramètre d'une fonction ou d'un bloc, ou dans un
  motif de `match` (les noms avant `=` ou `->` sur leur ligne comptent).
  Alors `x -1` est `x - 1` et `xs [0]` est `xs[0]`, comme toujours, dans un
  résultat aussi. Sans
  espace avant (`x-1`, `xs[0]`), ou avec une espace après le `-` (`x - 1`),
  ce n'est jamais un appel.
- `rythme fmt` garde le choix de l'auteur, avec ou sans parenthèses, et
  écrit les autres lectures comme ce qu'elles sont : `x - 1`, `xs[0]`.

```rythme
fn saluer(nom, salut) = print("#{salut}, #{nom} !")
saluer "Ada", "Bonjour"
puts "somme :", [1, 2, 3].sum
puts -1
puts [1, 2]
x = 10
xs = [7, 8]
puts x -1, xs [0]
[1, 2].each do |n| puts n * 10 end
each ["a", "b"] do |s|
  print s.upcase, s.size
end
```

```text
Bonjour, Ada !
somme : 6
-1
[1, 2]
9 7
10
20
A 1
B 1
```

Dans des résultats :

```rythme
fn saluer(nom) = puts "Bonjour, #{nom} !"
fn verifier(ok) = if ok then puts "oui" else puts "non"
fn titre(nom) =
  match nom
    "Dr " + reste -> puts "docteur", reste
    _ -> puts nom
  end
saluer "Ada"
verifier 1 > 2
titre "Dr Grace"
titre "Alan"
[1, 2].each { |n| puts n * 10 }
montrer = fn(v) = puts "v :", v
montrer(3)
x = 10
puts(if x > 5 then x -1 else 0)
```

```text
Bonjour, Ada !
non
docteur Grace
Alan
10
20
v : 3
9
```

Un nom qui reçoit une valeur avec `=` est une valeur, même quand il contient
une fonction :

```rythme
double = fn(v) = v * 2
print double(4)
double -3
```

```text
erreur[E005]: impossible d'utiliser `-` avec Function et Int
 --> sample.ry:3:8
  |
3 | double -3
  |        ^ Function - Int 3
  |
  = aide: `double -x` soustrait ici ; pour appeler `double`, écrivez `double(-x)` (un appel sans parenthèses commence une instruction ou un résultat, comme une branche de `if` ou le corps après le `=` d'une fonction, et son nom ne reçoit pas de valeur avec `=` ni comme paramètre)
  = en savoir plus : `rythme explain E005`
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
- Les noms apportés par `use "fichier.ry"` comptent comme des noms du
  premier niveau : en redéfinir un est E011. Avec `use geometrie`, seul le
  nom `geometrie` est pris (voir 4.9).

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

4 test(s) : 3 réussi(s), 1 échoué(s) (1 ms)
```

Code de sortie : 1. Le temps à la fin du résumé est celui de tout le
passage ; un test qui dure une demi-seconde ou plus montre aussi son temps.

**Plusieurs fichiers, un filtre.** `rythme test` accepte des fichiers et des
dossiers : pour un dossier (`rythme test .`), il lance les tests de chaque
fichier `.ry` qu'il contient, sous-dossiers compris, qui a des tests, dans
l'ordre alphabétique (les dossiers cachés, `target` et `node_modules` sont
sautés). Chaque fichier s'exécute à part, après une ligne qui le nomme. Un
fichier avec des erreurs est signalé, et compté dans le résumé. À la fin,
quand plusieurs fichiers ont été lancés ou qu'au moins deux tests ont échoué,
les tests échoués sont listés avec leur position. Les options peuvent venir
n'importe où après `test` :

| Option | Ce qu'elle fait |
|--------|-----------------|
| `--filter TEXTE` (`-f`) | lance seulement les tests dont le nom contient `TEXTE`, sans tenir compte des majuscules ; peut être donnée plusieurs fois. Quand aucun test n'est choisi, le code de sortie est 1 |
| `--seed N` | la graine des tests de propriétés (ci-dessous) ; `--seed random` en choisit une et l'affiche |
| `--watch` (`-w`) | lance les tests, puis à nouveau à chaque changement d'un fichier : un fichier testé, un fichier qu'il utilise, ou un fichier `.ry` d'un dossier donné. Les fichiers sont regardés deux fois par seconde ; Ctrl+C arrête |
| `-- ARGS...` | les mots après `--` sont donnés à `args()` |

**Tests de propriétés.** `check_all(propriete, generateur)` appelle la
fonction `propriete` avec 100 valeurs fabriquées par `generateur`
(`check_all(propriete, generateur, cas)` avec un autre nombre, de 1 à
100 000). La propriété est vraie pour une valeur quand elle donne `true`,
`()`, `Some(...)` ou `Ok(...)` ; elle échoue quand elle donne `false`, `None`
ou `Err(...)`, ou s'arrête sur une erreur (une autre valeur est E005). Les
premières valeurs sont petites, les suivantes utilisent de plus en plus
l'intervalle, et les bords (0, le minimum et le maximum) reviennent souvent.
Quand une valeur fait échouer la propriété, `check_all` la **simplifie** : un
nombre va vers 0 (ou vers le bout de son intervalle le plus proche de 0), un
texte ou une liste raccourcit, puis ses éléments se simplifient, tant que la
propriété échoue encore. Il s'arrête alors avec E015 (ou avec l'erreur de la
propriété, dont le message commence par la valeur), en montrant la valeur la
plus simple trouvée, celle d'où elle vient et la **graine**. Les valeurs ne
dépendent que de la graine et de la place de l'appel dans son fichier : chaque
lancement, sur chaque machine, essaie les mêmes valeurs ;
`rythme test --seed N` en essaie d'autres. La propriété peut aussi venir en
dernier, comme un bloc (voir [3.14](#314-blocs-donnés-aux-appels)) :
`check_all(ints(0, 9)) { |x| x * 0 == 0 }`.

Les générateurs sont des enregistrements, fabriqués par `ints(min, max)`
(nombres entiers, de -1000 à 1000 sans arguments), `floats(min, max)`
(nombres décimaux), `texts(max_length)` (20 au plus sans argument : lettres,
chiffres, espaces, ponctuation, accents), `bools()`,
`lists_of(generateur, max_length)` (10 au plus sans longueur),
`one_of(liste)` (une des valeurs ; une valeur qui échoue est remplacée par une
valeur écrite avant elle) et `records_of({champ: generateur, ...})`, pour
donner plusieurs valeurs. `sample(generateur, n)` donne `n` valeurs (10 sans
`n`) pour voir ce que fabrique un générateur.

Quand `assert_eq` échoue sur des listes ou des enregistrements, le message dit
aussi où les valeurs diffèrent en premier (`[2].name`), ce qui manque ou est en
trop ; sur deux textes courts, une marque montre le premier caractère qui
diffère.

```rythme
fn mon_max(liste) = fold(liste, 0, fn(meilleur, x) = if x > meilleur then x else meilleur)

test "inverser deux fois redonne la liste" = check_all(fn(l) = reverse(reverse(l)) == l, lists_of(ints()))

test "le plus grand est dans la liste" = check_all(fn(l) = is_empty(l) or contains(l, mon_max(l)), lists_of(ints()))

test "un enregistrement" = assert_eq({nom: "Ada", langues: ["en", "fr"]}, {nom: "Ada", langues: ["en", "de"]})
```

`rythme test 47-proprietes.ry` :

```text
Exécution de 3 test(s)
  ✓ inverser deux fois redonne la liste
  ✗ le plus grand est dans la liste
erreur[E015]: la propriété est fausse pour [-1]
 --> 47-proprietes.ry:5:42
  |
5 | test "le plus grand est dans la liste" = check_all(fn(l) = is_empty(l) or contains(l, mon_max(l)), lists_of(ints()))
  |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ fausse pour [-1]
  |
  = note: trouvé à l'essai 1 sur 100, puis simplifié depuis [-3, -1] en 3 étape(s)
  = note: graine 42 : chaque lancement essaie les mêmes valeurs ; `rythme test --seed N` en essaie d'autres
  = aide: la fonction doit être vraie pour chaque valeur du générateur : corrigez la fonction, ou donnez au générateur un intervalle plus petit, comme `ints(1, 100)`
  = en savoir plus : `rythme explain E015`

  ✗ un enregistrement
erreur[E015]: assertion échouée : les valeurs sont différentes
 --> 47-proprietes.ry:7:28
  |
7 | test "un enregistrement" = assert_eq({nom: "Ada", langues: ["en", "fr"]}, {nom: "Ada", langues: ["en", "de"]})
  |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: obtenu :  {nom: "Ada", langues: ["en", "fr"]}
      attendu : {nom: "Ada", langues: ["en", "de"]}
  = note: première différence en langues[1] : obtenu "fr", attendu "de"
  = en savoir plus : `rythme explain E015`

Tests échoués :
  ✗ le plus grand est dans la liste (47-proprietes.ry:5:1)
  ✗ un enregistrement (47-proprietes.ry:7:1)
3 test(s) : 1 réussi(s), 2 échoué(s) (7 ms)
```

### 4.9 Modules : `use`

```text
use geometrie              # le fichier geometrie.ry, utilisé par son nom
use formes/cercle          # le fichier formes/cercle.ry, utilisé comme `cercle`
use geometrie as geo       # le même fichier, sous un autre nom
use "geometrie.ry"         # ancienne forme : ses noms s'utilisent directement
```

- **La forme recommandée est `use nom`**, sans guillemets et sans `.ry` :
  ses définitions s'atteignent alors par ce nom, si bien que chaque nom dit
  d'où il vient. `rythme fmt` écrit `use "geometrie.ry" as geo` sous la
  forme `use geometrie as geo`.
- `use` n'est permis qu'au premier niveau. Le chemin est relatif au
  **dossier du fichier qui contient le `use`** (pour `rythme -e` et le mode
  interactif : le dossier actuel). Ses parties sont des noms séparés par `/`
  sur tous les systèmes, et `..` remonte d'un dossier : `use ../outils/calculs`.
  Un fichier dont le nom n'est pas un nom Rythme (`Mes outils.ry`) s'écrit
  comme un texte, avec un nom : `use "Mes outils.ry" as outils`.
- Le nom est celui du fichier (`cercle` pour `formes/cercle`), ou celui écrit
  après `as`. Alors `geometrie.aire_carre(4)` appelle une fonction,
  `geometrie.origine` lit une valeur, `geometrie.Carre(5)` construit une
  valeur d'un type du fichier, `geometrie.Carre(c)` la reconnaît dans un
  `match`, et `x |> geometrie.aire_carre` marche comme d'habitude. Le
  vérificateur les connaît tous : un nom que le fichier n'a pas est E003
  (avec le plus proche), et le nombre d'arguments est vérifié (E004).
- Un fichier donne ses définitions du premier niveau (valeurs, fonctions,
  constructeurs), et celles des fichiers qu'il utilise avec des guillemets.
  **Un nom qui commence par `_` est privé à son fichier** (`fn _verifie(x)`) :
  les autres fichiers ne peuvent pas l'utiliser (E003). Le nom du fichier
  n'est pas une valeur (`geometrie` seul est E003), et il ne peut rien nommer
  d'autre dans le fichier (E011) ; deux fichiers ne peuvent pas recevoir le
  même nom (E011).
- Chaque fichier a ses propres noms : `geometrie.aire` et `cercle.aire` sont
  deux fonctions différentes, et un fichier peut lui-même définir `aire`. Un
  nom utilisé sans son fichier, mais défini par un autre fichier du dossier,
  reçoit une aide qui nomme ce fichier et le `use` à ajouter (le serveur de
  langage l'ajoute).
- `use "chemin.ry"`, avec des guillemets et sans `as`, est l'ancienne forme :
  les définitions du fichier (sauf les privées) deviennent visibles
  directement, avec celles des fichiers qu'il utilise de cette façon ; un nom
  défini à nouveau, ou défini par deux de ces fichiers, est E011, et le
  message nomme les deux fichiers. `.ry` est ajouté quand le chemin n'a pas
  d'extension.
- Chaque fichier est chargé, vérifié et exécuté **une seule fois** par
  programme, avant le fichier qui l'utilise, où que le `use` soit écrit dans
  ce fichier. Ses expressions du premier niveau s'exécutent (un `print` dans
  un fichier utilisé affiche une fois).
- Un fichier introuvable est E017, avec une suggestion quand un nom proche
  existe (ou la liste des fichiers Rythme du dossier), un dossier aussi ; des
  fichiers qui s'utilisent en cercle sont E017, et le message nomme les deux.
  Les messages montrent le chemin d'un fichier utilisé tel qu'on l'atteint
  depuis le dossier actuel.

```rythme
# geometrie.ry
print("geometrie.ry ne s'exécute qu'une fois")
fn aire_rectangle(l, h) = l * h
fn aire_carre(cote) = aire_rectangle(cote, _verifie(cote))
fn _verifie(x) = if x >= 0 then x else fail("côté négatif")
origine = {x: 0, y: 0}
type Forme = Rectangle(l, h) | Carre(cote)
```

```rythme
# formes/cercle.ry
fn aire(r) = pi * r ** 2
```

```rythme
# principal.ry
use geometrie
use formes/cercle
print(geometrie.aire_rectangle(2, 3), geometrie.aire_carre(4), geometrie.origine)
print(round(cercle.aire(1), 2))
fn aire(forme) = match forme
  geometrie.Rectangle(l, h) -> l * h
  geometrie.Carre(c) -> c * c
end
print(aire(geometrie.Carre(5)))
```

`rythme principal.ry` :

```text
geometrie.ry ne s'exécute qu'une fois
6 16 {x: 0, y: 0}
3.14
25
```

Un fichier à côté de `geometrie.ry` qui oublie le `use`, `rythme oubli.ry` :

```text
erreur[E003]: nom inconnu `aire_rectangle`
 --> oubli.ry:1:7
  |
1 | print(aire_rectangle(2, 3))
  |       ^^^^^^^^^^^^^^ non défini ici
  |
  = aide: `aire_rectangle` est défini dans `geometrie.ry` : ajoutez `use geometrie` en haut du fichier, puis écrivez `geometrie.aire_rectangle`
  = en savoir plus : `rythme explain E003`
```

Un nom privé, `rythme prive.ry` :

```text
erreur[E003]: `_verifie` est privé au fichier `geometrie.ry`
 --> prive.ry:2:7
  |
2 | print(geometrie._verifie(2))
  |       ^^^^^^^^^^^^^^^^^^ privé
  |
  = note: un nom qui commence par `_` ne s'utilise que dans son propre fichier
  = aide: pour l'utiliser depuis d'autres fichiers, renommez-le sans `_` dans `geometrie.ry`
  = en savoir plus : `rythme explain E003`
```

Avec `ping.ry` qui contient `use pong` et `pong.ry` qui contient `use ping`,
`rythme ping.ry` donne :

```text
erreur[E017]: `pong.ry` et `ping.ry` s'utilisent l'un l'autre
 --> pong.ry:1:1
  |
1 | use ping
  | ^^^^^^^^
  |
  = aide: des fichiers ne peuvent pas s'utiliser l'un l'autre : déplacez ce dont les deux ont besoin dans un troisième fichier, utilisé par les deux
  = en savoir plus : `rythme explain E017`
```

---

## 5. Erreurs

### 5.1 Les erreurs sont des valeurs

Rythme n'a ni `null` ni exceptions. Une valeur qui peut manquer est une
`Option` (`Some(valeur)` ou `None`) ; une opération qui peut échouer renvoie
un `Result` (`Ok(valeur)` ou `Err(erreur)`). La bibliothèque suit cette
règle : `first`, `last`, `get`, `find`, `index_of` et `env` renvoient des
Options ; `parse_int`, `parse_float`, `parse_json`, `options`, `input_lines`
et toutes les fonctions des fichiers et dossiers (`read_file`, `write_file`,
`append_file`, `file_info`, `list_folder`, `create_folder`, `delete_file`)
renvoient des Results.

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
  sinon ; 0 s'il est omis). Les codes avec lesquels Rythme lui-même s'arrête
  sont en [7.1](#71-la-ligne-de-commande).

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
  ne peuvent jamais être atteints et les intervalles vides (W004), les
  fonctions inconnues après un point (W005), les intervalles qui se
  chevauchent (W010).

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

Le vérificateur déduit aussi ce que doivent être les **paramètres de chaque
fonction**, d'après la façon dont son code les utilise : `cote * cote` a
besoin de nombres, `upper(t)` d'un texte, `p.nom` d'un enregistrement (ou
d'une valeur dont le constructeur a un champ `nom`), `if c` d'un Bool, un
`match` des natures de ses motifs, un appel à une autre fonction de ce dont
celle-ci a besoin. Il suit chaque chemin de la fonction : les branches d'un
`if` ou d'un `match`, et les définitions d'une fonction (une par cas),
additionnent leurs natures ; une opération après l'autre les restreint ; un
`?`, ou un appel qui peut terminer le programme (`exit`), garde ce qui est
connu à cet endroit. Un appel avec un argument d'une autre nature échoue à
chaque exécution : le vérificateur le signale, avec le code, le message et
l'aide de l'erreur sur laquelle la fonction s'arrêterait, l'endroit où elle
s'arrête dans la fonction, et ce dont la fonction a besoin. Rien n'est
signalé quand l'appel peut marcher : une fonction qui marche sur plusieurs
natures de valeurs (`deux_fois` ci-dessous, sur les nombres, les textes et
les listes), des définitions choisies par leurs motifs ou leurs gardes, la
récursivité, les fonctions jamais appelées. Ce dont ont besoin les fonctions
d'un autre fichier n'est pas déduit.

```rythme
fn aire(cote) = cote * cote
fn crier(t) = upper(t) + "!"
fn deux_fois(x) = x + x
print(aire(3), crier("salut"), deux_fois("ab"))
print(aire("grand"))
print(crier(5))
```

`rythme check 54-parametres.ry` :

```text
erreur[E005]: impossible d'utiliser `*` avec Text et Text
 --> 54-parametres.ry:5:7
  |
5 | print(aire("grand"))
  |       ^^^^^^^^^^^^^ `aire` s'arrête sur cette erreur, à chaque fois
  |
1 | fn aire(cote) = cote * cote
  |                      - dans `aire` : Text "grand" * Text "grand"
  |
  = note: d'après la façon dont `aire` utilise `cote`, elle a besoin de : `fn aire(cote: Int|Float) -> Int|Float`
  = en savoir plus : `rythme explain E005`

erreur[E005]: `upper` attend un texte comme argument 1, mais a reçu Int 5
 --> 54-parametres.ry:6:7
  |
6 | print(crier(5))
  |       ^^^^^^^^ `crier` s'arrête sur cette erreur, à chaque fois
  |
2 | fn crier(t) = upper(t) + "!"
  |                     - dans `crier` : mauvaise sorte de valeur
  |
  = note: d'après la façon dont `crier` utilise `t`, elle a besoin de : `fn crier(t: Text) -> Text`
  = aide: utilisation : `upper(text)`
  = en savoir plus : `rythme explain E005`

✗ 54-parametres.ry : 2 erreur(s)
```

Dans un éditeur, le survol d'une fonction montre ce que le vérificateur
déduit, sous forme de signature : `fn aire(cote: Int|Float) -> Int|Float`,
les natures que doit avoir chaque paramètre (séparées par `|`) et ce que
donne un appel, quand elles sont connues (voir
[7.5](#75-éditeurs-et-serveur-de-langage)).

### 5.6 Codes d'erreur et d'avertissement

Chaque erreur et chaque avertissement a un code. `rythme explain CODE` (ou
`rythme help CODE`, ou `:explain CODE` dans le mode interactif) donne une
explication détaillée avec des exemples ; le code peut s'écrire en
minuscules.

| Code | Nom | Quand |
|------|-----|-------|
| E001 | Erreur de syntaxe | le code ne suit pas la grammaire : une parenthèse ou un `then` manquant, des accolades pour un bloc, un mot-clé d'un autre langage, des comparaisons enchaînées, `_` utilisé comme valeur, `{nom, ..autres}`, une expression trop longue ou imbriquée trop profondément... |
| E002 | Texte jamais fermé | un `"` ou une `{` dans un texte n'est jamais fermé |
| E003 | Nom inconnu | un nom ou un constructeur qui n'est pas défini à cet endroit, une valeur utilisée avant sa définition, ou un nom privé (`_nom`) d'un autre fichier |
| E004 | Mauvais nombre d'arguments | un appel ou un motif de constructeur avec trop ou trop peu de valeurs ; des définitions d'une même fonction avec des nombres de paramètres différents |
| E005 | Mauvaise sorte de valeur | une opération ou une fonction a reçu une sorte de valeur qu'elle n'accepte pas (`"a" + 1`, une condition qui n'est pas un Bool, des valeurs qu'on ne peut pas comparer...) |
| E006 | Aucun cas ne correspond | aucun cas d'un `match`, aucune définition d'une fonction, ou le motif d'une définition n'accepte pas la valeur |
| E007 | Indice hors limites | `liste[i]` ou `texte[i]` hors des positions valides |
| E008 | Division par zéro | `/`, `//` ou `%` par zéro, ou `0 ** -n` |
| E009 | Pas une fonction | on appelle une valeur qui n'est pas une fonction (`x(3)`, `None()`) |
| E010 | Champ inconnu | l'enregistrement ou la variante n'a pas de champ de ce nom |
| E011 | Déjà défini | un nom, un type, un constructeur ou un champ défini deux fois au même endroit, ou une tentative de modifier une valeur (`x += 1`) |
| E012 | Nombre trop grand | plus donnée : les nombres entiers n'ont pas de limite de taille (un nombre de plus de 100000 chiffres est E023) |
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
| E023 | Valeur invalide | une valeur de la bonne sorte mais hors de ce qui est accepté (un nombre négatif d'éléments, `min` d'une liste vide, `sqrt(-1)`, un intervalle, une liste ou un texte trop grand, un nombre entier de plus de 100000 chiffres, un code de sortie hors de 0–255...) |
| E024 | Une valeur est perdue | une ligne calcule une valeur dont rien ne se sert |
| E025 | Problème de projet ou de paquet | un `rythme.toml` ou un `rythme.lock` illisible, une dépendance non installée, ou `rythme install` qui ne peut pas en obtenir une (voir [7.7](#77-projets-et-paquets)) |
| E026 | Regex invalide | une regex qui ne respecte pas une règle (une `(` jamais fermée, `*` sans rien à répéter, un `\q` inconnu...), ou un remplacement de `replace_all` avec un groupe qui n'existe pas |
| W001 | Nom inutilisé | un nom local est défini mais jamais utilisé |
| W002 | Cas oublié | un `match` ou une fonction à un paramètre ne traite pas toutes les valeurs possibles |
| W003 | Fonction intégrée masquée | une définition du premier niveau prend le nom d'une fonction intégrée |
| W004 | Cas jamais atteint | un cas ou une définition vient après un autre qui accepte tout ou les mêmes valeurs (ou après des intervalles qui contiennent ensemble tous ses nombres), ne peut jamais correspondre à la valeur, ou est un intervalle vide (`9..1`) |
| W005 | Fonction inconnue après un point | `valeur.nom(...)` alors qu'aucune fonction ni aucun champ d'enregistrement ne porte ce nom |
| W006 | Comparaison toujours fausse | `==` ou `!=` entre des valeurs de natures qui ne sont jamais égales (`nombre == "3"`) |
| W007 | Test qui ne peut jamais échouer | un test qui ne fait que calculer ou afficher une valeur, sans jamais la comparer à ce qui est attendu |
| W008 | Résultat perdu | une ligne qui ne fait qu'appeler une fonction sans effet, comme `push(liste, x)` : les valeurs ne changent jamais |
| W009 | Fonction affichée | une fonction affichée dans un texte ou par `print` (`<fn nom>`) au lieu de ce qu'elle donne |
| W010 | Intervalles qui se chevauchent | un motif d'intervalle a des nombres en commun avec un intervalle d'un cas ou d'une définition placé avant lui : ils vont toujours au premier |

---

## 6. Bibliothèque standard

Les 174 fonctions intégrées sont toujours disponibles, sans `use`. Comme les
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
E017), `input_lines()` (toutes les lignes de l'entrée standard jusqu'à sa
fin, dans un Result : pour `rythme compter.ry < noms.txt`), `type_of(valeur)`, `help(valeur)`, `assert(condition, message)`,
`assert_eq(obtenu, attendu)`, `fail(message)`, `args()` (les mots donnés
après le nom du fichier sur la ligne de commande), `options()` (les mots de
`args()` lus comme des options, dans un Result : voir plus bas), `env(nom)` (une variable
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
`indentation` espaces, au plus 16), `trim_start` et `trim_end` (les espaces
et retours à la ligne d'un seul côté), `words(texte)` (lettres et chiffres,
avec `'`, `’` ou `-` à l'intérieur d'un mot : `"aujourd'hui"`,
`"peut-être"`), `equals_ignore_case(a, b)` (majuscules et minuscules
comptent comme égales, `ß` comme `SS`), et
`format_number(x, decimales, langue)` (milliers séparés, `decimales`
décimales si elles sont données, au plus 100, arrondies comme `round` ;
`langue` vaut `"en"` par défaut : `1,234.5` ; `"fr"` : `1 234,5`, avec une
espace ordinaire ; `"de"`, `"es"`, `"it"`, `"nl"`, `"pt"` : `1.234,5` ;
`"fr-CA"` compte comme `"fr"`).

**Regex.** Une regex (expression régulière) est un texte qui décrit des
morceaux de texte. `matches(texte, regex)` (le texte *entier*
correspond-il ?), `find_first(texte, regex)` (une Option),
`find_all(texte, regex)` (chaque correspondance, de gauche à droite, sans
chevauchement ; une correspondance vide juste après la précédente est
sautée), `captures(texte, regex)` (une Option d'une liste : la
correspondance entière, puis le texte de chaque groupe, `""` pour un groupe
qui n'y a pas pris part), `captures_all`, `named_captures(texte, regex)`
(une Option d'un enregistrement, un champ par groupe `(?<nom>...)` ; E023
sans groupe nommé), `split_by(texte, regex)` (une correspondance vide ne
coupe jamais) et `replace_all(texte, regex, nouveau)` (`nouveau` est un
texte où `$1`, `$<nom>`, `$0` et `$$` tiennent lieu d'un groupe, d'un groupe
nommé, de la correspondance entière et d'un `$`, ou une fonction qui reçoit
chaque correspondance et doit renvoyer un texte). Une regex est faite de
caractères, de `.` (n'importe quel caractère sauf `\n`), de classes
`[abc]`, `[a-z]`, `[^...]` (un `]` juste après `[` ou `[^` et un `-` à un
bout sont eux-mêmes), de `\d` (0 à 9), `\w` (une lettre, accents compris,
un chiffre ou `_`), `\s` (une espace, une tabulation ou un retour à la
ligne), de leurs contraires `\D`, `\W`, `\S`, de `\b` et `\B` (au bord
d'un mot, ou non), de `^` et `$` (le début et la fin du texte), de `\n`,
`\t`, `\r`, d'une barre oblique inverse devant un signe pour le signe
lui-même (`\.`), d'alternatives `a|b`, de groupes `(...)`, `(?:...)` (sans
capture) et `(?<nom>...)` (ou `(?P<nom>...)`), des répétitions `*`, `+`,
`?`, `{n}`, `{n,}` et `{n,m}` (n et m jusqu'à 1000), qui prennent autant
qu'elles peuvent, ou aussi peu avec un `?` après elles (`*?`), et de `(?i)`
au début, pour ignorer la casse. Les règles habituelles choisissent entre
plusieurs correspondances : la plus à gauche, puis la première
alternative, puis la répétition la plus longue. Une recherche prend un temps
proportionnel à la longueur du texte fois la taille de la regex, quelles
qu'elles soient : le moteur suit tous les chemins à la fois au lieu de les
essayer l'un après l'autre (pas de « retour arrière catastrophique ») ;
pour la répétition d'une partie qui peut correspondre à un texte vide,
comme `(a*)*`, il peut choisir une autre fin que d'autres moteurs. Dans un
texte Rythme, la barre oblique inverse s'écrit deux fois et les accolades
`\{` et `\}` : `"\\d\{4\}"` est la regex `\d{4}` ; un signe entre
crochets n'a pas besoin de barre oblique : `"[.]"`, `"[(]"`. Une erreur
dans une regex arrête le programme avec E026, qui la montre dans la regex ;
quand la regex est écrite dans l'appel, elle est trouvée avant de lancer le
programme, à sa place dans le code.

**Listes.** `map`, `filter`, `reject(liste, f)` (les éléments pour lesquels
f est fausse), `fold(liste, depart, f)`, `scan(liste, depart, f)` (chaque
résultat intermédiaire de `fold`), `each`, `each_with_index(liste, f)`
(appelle `f(x, i)`), `sum` (et `sum(liste, f)`, la somme des `f(x)`),
`product`, `first` et `last` (une Option ; `first(liste, n)` et
`last(liste, n)` donnent les n premiers ou derniers éléments), `rest`, `get(x, cle)`, `push(liste, x)`,
`set_at(liste, i, x)`, `insert_at(liste, i, x)` et `remove_at(liste, i)`
(les positions négatives partent de la fin), `reverse`, `sort`,
`sort_by(liste, cle)` (stable), `unique`, `shuffle`, `slice(x, debut, fin)`,
`take`, `drop`, `take_while(x, f)` et `drop_while(x, f)`, `step_by(x, n)`
(un élément sur n), `chunks(x, n)` (des morceaux de n éléments) et
`windows(x, n)` (chaque groupe de n éléments voisins), `zip`, `enumerate`,
`any`, `all`, `find`, `count` (et `count(liste)`, le nombre d'éléments),
`index_of`, `min_by(liste, cle)` et
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

**Dictionnaires et ensembles.** `dict()`, `dict(paires)` et
`dict(enregistrement)`, `set()` et `set(liste)` les construisent (voir
[2.15](#215-dict-et-set)). Les fonctions des listes et des enregistrements
marchent aussi sur eux : `len`, `is_empty`, `get(dico, clé)` (une Option),
`has(x, clé)` et `contains(x, clé)` (une clé d'un dictionnaire, un élément
d'un ensemble), `put(dico, clé, valeur)`, `push(ensemble, x)`,
`remove(x, clé)` (une clé d'un dictionnaire, un élément d'un ensemble),
`keys(dico)`, `values(x)` (les valeurs d'un dictionnaire, ou les éléments
d'un ensemble, sous forme de liste), `entries(dico)` (une liste de
`[clé, valeur]`), `map_values(dico, f)`, `filter(dico, f)` et `reject(dico, f)`
(f reçoit chaque clé et sa valeur), `filter(ensemble, f)`,
`reject(ensemble, f)`, `each(dico, f)` (f reçoit chaque clé et sa valeur),
`each(ensemble, f)`, `map(ensemble, f)` (l'ensemble des résultats). Puis `union(a, b)` (deux ensembles, ou deux dictionnaires : pour
une clé des deux, la valeur de `b` l'emporte), `intersection(a, b)` et
`difference(a, b)` (pour des dictionnaires, les paires de `a` dont la clé
est, ou n'est pas, dans `b`) et `is_subset(a, b)` (deux ensembles).
`to_json` écrit un dictionnaire dont les clés sont toutes des textes, ou
toutes des symboles, comme un objet JSON, et un ensemble comme un tableau ; `dict(parse_json(texte)?)`
lit un objet comme un dictionnaire. Les autres fonctions des listes ne les
prennent pas : `values(x)` ou `entries(x)` donnent d'abord une liste.

```rythme
a = set([1, 2, 3])
b = set([2, 3, 4])
print(union(a, b), intersection(a, b), difference(a, b), is_subset(set([2]), a))
print(push(a, 0), remove(a, 2), has(a, 2), values(a), map(a, fn(x) = x % 2))
prix = dict([["pomme", 3], ["poire", 5], ["figue", 9]])
print(filter(prix, fn(nom, p) = p < 6), map_values(prix, fn(p) = p * 2))
print(union(prix, dict([["figue", 1], ["kiwi", 4]])))
compte = fold(split("a b a c b a"), dict(), fn(d, m) = put(d, m, unwrap_or(get(d, m), 0) + 1))
print(compte, entries(compte))
print(to_json(compte), to_json(set([1, 2])), dict(unwrap(parse_json("\{\"x\": 1}"))))
```

```text
set(1, 2, 3, 4) set(2, 3) set(1) true
set(0, 1, 2, 3) set(1, 3) true [1, 2, 3] set(0, 1)
dict("poire" => 5, "pomme" => 3) dict("figue" => 18, "poire" => 10, "pomme" => 6)
dict("figue" => 1, "kiwi" => 4, "poire" => 5, "pomme" => 3)
dict("a" => 3, "b" => 2, "c" => 1) [["a", 3], ["b", 2], ["c", 1]]
{"a": 3, "b": 2, "c": 1} [1, 2] dict("x" => 1)
```

**Option et Result.** `unwrap`, `unwrap_or(x, defaut)`, `is_some`,
`is_none`, `is_ok`, `is_err`, `and_then(x, f)` (une étape de plus qui peut
échouer : `f` donne une Option ou un Result), `ok_or(option, erreur)` (une
Option transformée en Result, prête pour `?`), `map_err(result, f)`, et les
constructeurs `Some`, `None`, `Ok`, `Err`.

**Fichiers.** `read_file(chemin)` (`Ok(texte)` ou `Err(message)`),
`write_file(chemin, valeur)` (écrit la forme affichée en remplaçant le
contenu : `Ok(())` ou `Err(message)`), `append_file(chemin, valeur)` (ajoute
à la fin, en créant le fichier si besoin), `file_exists(chemin)` (un Bool,
pour un fichier ou un dossier), `file_info(chemin)` (`Ok({is_folder, size})`,
la taille en octets, 0 pour un dossier), `list_folder(chemin)` (les noms
contenus dans un dossier, dans l'ordre alphabétique), `create_folder(chemin)`
(avec les dossiers manquants autour ; `Ok(())` aussi quand il existe déjà),
`delete_file(chemin)` (un fichier, jamais un dossier). Les chemins relatifs
partent du dossier actuel. Chaque erreur est un `Err(message)` qui nomme le
chemin : `"fichier introuvable : notes.txt"`, `"dossier introuvable :
photos"`, `"permission refusée : ..."`, un fichier donné à la place d'un
dossier ou l'inverse, un chemin vide.

`join_path(parties)` joint une liste de textes avec `/`, que Linux, macOS et
Windows comprennent tous (avec `\` quand les parties utilisent seulement
`\`, comme les chemins Windows) ; un séparateur à la fin d'une partie n'est
pas répété, et les parties vides sont ignorées. Seule la première partie
peut partir d'une racine (`/`, `\`, ou un lecteur comme `C:`) : une partie
comme `"/etc"` après une autre est l'erreur E023, pour qu'un chemin ne sorte
jamais de son dossier par surprise. `split_path(chemin)` coupe un chemin à
chaque `/` ou `\`, quel que soit le système ; la racine d'un chemin absolu
(`/`, `C:\`, `\\` pour un chemin réseau) est sa première partie, et
`join_path` remet les parties ensemble.

`options()` lit les mots de `args()` (ou `options(mots)`, la liste donnée)
dans un enregistrement, dans un Result. `--nom=valeur` donne le champ `nom`
avec le texte `valeur` ; `--nom` et `-n` donnent `true` ; un `-` dans un nom
devient `_` (`--dry-run` donne `dry_run`) ; les autres mots, dans l'ordre,
sont dans le champ `args`, comme tous les mots après `--`. Un nombre négatif
comme `-5` est un mot. Une option donnée deux fois, un nom qui ne commence
pas par une lettre, un mot comme `-abc` (plusieurs lettres après un seul
`-`) et `--args` donnent `Err(message)`. Une option qui peut manquer se lit
avec `get(opts, "nom")` (une Option) ou se teste avec `has(opts, "nom")`.

Dans le playground, toutes les fonctions des fichiers et dossiers et
`input_lines` donnent `Err("`read_file` n'est pas disponible dans le
navigateur : ...")`, et `file_exists` vaut toujours `false`.

**Dates et heures.** Une date est un enregistrement `{year, month, day}`,
avec `hour`, `minute` et `second` pour un moment de la journée (un champ
absent vaut 0), et `offset`, le nombre de minutes d'avance de l'heure sur
UTC, quand il est connu. Les années vont de 1 à 9999, dans le calendrier
grégorien (une année bissextile tous les 4 ans, sauf tous les 100 ans, sauf
tous les 400 ans). `now()` (la date et l'heure locales avec leur `offset` :
d'après le fuseau horaire du système sous Linux et macOS, du navigateur dans
le bac à sable, 0 (UTC) ailleurs), `today()` (`{year, month, day}`),
`parse_date(texte)` (un Result ; ISO 8601 : `"2024-02-29"`, puis si besoin
`T` ou une espace et une heure `13:05` ou `13:05:00`, une fraction de
seconde, ignorée, et `Z` ou un décalage `+01:00`, `+0100` ou `+01`),
`format_date(date)` (le retour en ISO 8601, avec l'heure et le décalage
quand la date en a), `is_date(valeur)`, `add_days(date, n)` et
`add_seconds(date, n)` (l'heure et le décalage restent ; `add_seconds` donne
toujours une heure), `days_between(a, b)` (les jours entiers de a à b,
arrondis vers zéro) et `seconds_between(a, b)` (les décalages comptent),
`day_of_week(date)` (de 1 pour lundi à 7 pour dimanche, comme en ISO 8601),
`days_in_month(annee, mois)`, `is_leap_year(annee)`. Un enregistrement qui
n'est pas une date (un champ inconnu ou absent, un nombre hors limites, un
30 février) arrête ces fonctions avec E023, qui dit ce qui ne va pas. Un
décalage est un nombre fixe : il n'y a pas de règles d'heure d'été, donc un
jour après un moment, c'est la même heure le lendemain.

**Tests de propriétés.** `check_all(propriete, generateur, cas)`,
`sample(generateur, n)`, et les générateurs `ints(min, max)`,
`floats(min, max)`, `texts(max_length)`, `bools()`,
`lists_of(generateur, max_length)`, `one_of(liste)` et `records_of(champs)`
(voir [4.8](#48-tests)). Un générateur est un enregistrement comme
`{generator: "ints", min: 1, max: 6}` ; un générateur mal fait arrête
`check_all` et `sample` avec E023.

**Noms Ruby.** Certaines fonctions ont un second nom, celui que Ruby leur
donne. Chacun est exactement la fonction qu'il nomme (mêmes arguments, mêmes
messages) : `rythme help upcase` le dit, et `rythme help builtins` les liste
par catégorie. Un programme peut donner l'un de ces noms à quelque chose de
à lui sans avertissement.

| Nom Ruby | Pareil que | Nom Ruby | Pareil que |
|----------|------------|----------|------------|
| `puts` | `print` | `select` | `filter` |
| `to_s` | `text` | `reduce`, `inject` | `fold` |
| `to_i` | `int` | `uniq` | `unique` |
| `to_f` | `float` | `each_slice` | `chunks` |
| `length`, `size` | `len` | `each_cons` | `windows` |
| `upcase` | `upper` | `include?` | `contains` |
| `downcase` | `lower` | `empty?` | `is_empty` |
| `strip` | `trim` | `even?`, `odd?` | `is_even`, `is_odd` |
| `start_with?` | `starts_with` | `any?`, `all?` | `any`, `all` |
| `end_with?` | `ends_with` | `key?` | `has` |
| `lstrip`, `rstrip` | `trim_start`, `trim_end` | `casecmp?` | `equals_ignore_case` |

`sort_by`, `min_by`, `max_by`, `group_by`, `zip`, `flatten`, `sum`, `count`,
`first`, `last`, `find`, `map`, `each`, `reject` et `each_with_index` ont le
même nom dans les deux langages. Avec les appels de méthode et les blocs
([3.8](#38-champs-et-appels-de-méthode), [3.14](#314-blocs-donnés-aux-appels)),
du code Ruby marche souvent tel quel :

```rythme
mots = ["poire", "figue", "banane", "figue"]
print(mots.uniq.size, mots.select { |m| m.size > 5 }, mots.reject { |m| m.include?("a") })
print(mots.first(2), mots.last(1), mots.count, mots.sum { |m| m.length })
print(mots.each_slice(3), "  Rythme ".strip.downcase, 42.to_s + " !", "7".to_i + 1)
print([1, 2, 3].reduce(0) do |acc, x| acc + x end, [3, 1, 2].sort_by { |x| -x })
```

```text
3 ["banane"] ["poire", "figue", "figue"]
["poire", "figue"] ["figue"] 4 21
[["poire", "figue", "banane"], ["figue"]] rythme 42 ! 8
6 [3, 2, 1]
```

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

erreur[E005]: `map` attend une liste, un ensemble, une Option ou un Result comme argument 1, mais a reçu une valeur de type Text
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

```rythme
fn sauvegarder(nom)
  create_folder(join_path(["sauvegarde", "2024"]))?
  write_file(join_path(["sauvegarde", "2024", nom]), "gardé")?
  list_folder("sauvegarde/2024")
end
print(sauvegarder("notes.txt"))
print(file_info("sauvegarde/2024/notes.txt"), file_info("sauvegarde"))
print(delete_file("sauvegarde/2024/notes.txt"), delete_file("sauvegarde"))
print(split_path("C:\\Users\\ada\\notes.txt"), split_path("/home/ada/") |> join_path)
print(options(["--dry-run", "--nom=Ada", "notes.txt"]))
print(options(["-abc"]))
print(input_lines())
```

```text
Ok(["notes.txt"])
Ok({is_folder: false, size: 6}) Ok({is_folder: true, size: 0})
Ok(()) Err("sauvegarde est un dossier : `delete_file` supprime seulement des fichiers, jamais des dossiers")
["C:\\", "Users", "ada", "notes.txt"] /home/ada
Ok({args: ["notes.txt"], dry_run: true, nom: "Ada"})
Err("`-abc` n'est pas une option que Rythme comprend : écrivez `--abc` pour un nom long, ou `-a -b` pour plusieurs options d'une lettre")
Ok([])
```

```rythme
depart = unwrap(parse_date("2024-02-28T22:30:00+01:00"))
print(depart)
print(add_days(depart, 1) |> format_date, add_seconds(depart, 5400) |> format_date)
print(days_between({year: 2024, month: 1, day: 1}, {year: 2025, month: 1, day: 1}), days_in_month(2023, 2), is_leap_year(1900))
print(day_of_week({year: 2024, month: 2, day: 29}), seconds_between(depart, {year: 2024, month: 2, day: 28, hour: 21, minute: 30, offset: 0}))
print(parse_date("2023-02-29"))
```

```text
{year: 2024, month: 2, day: 28, hour: 22, minute: 30, second: 0, offset: 60}
2024-02-29T22:30:00+01:00 2024-02-29T00:00:00+01:00
366 28 false
4 0
Err("\"2023-02-29\" n'est pas une date valide : le jour doit aller de 1 à 28 en février 2023, pas 29 (2023 n'est pas une année bissextile)")
```

Une date qui n'existe pas est expliquée :

```rythme
echeance = {year: 2023, month: 2, day: 29}
print(add_days(echeance, 7))
```

```text
erreur[E023]: `add_days` a besoin d'une date valide : le jour doit aller de 1 à 28 en février 2023, pas 29 (2023 n'est pas une année bissextile)
 --> 65-dates.ry:2:7
  |
2 | print(add_days(echeance, 7))
  |       ^^^^^^^^^^^^^^^^^^^^^ pas une date valide
  |
  = aide: une date est un enregistrement comme `{year: 2024, month: 2, day: 29}`, avec `hour`, `minute`, `second` et `offset` (minutes d'avance sur UTC) si besoin
  = en savoir plus : `rythme explain E023`
```

Chercher et découper avec des regex, et des nombres écrits pour les humains :

```rythme
journal = "2024-02-29 ERROR disk full\n2024-03-01 INFO ok\n2024-03-02 ERROR fan"
errors = lines(journal) |> filter(fn(line) = matches(line, ".* ERROR .*"))
print(errors |> map(fn(line) = unwrap(find_first(line, "[0-9-]+"))))
print(find_all("3 cats, 12 dogs and 1 fish", "\\d+"), split_by("a, b;c", "[,;] *"))
print(captures("v2.14", "v(\\d+)\\.(\\d+)"), named_captures("2024-02", "(?<year>\\d+)-(?<month>\\d+)"))
print(replace_all("29/02/2024", "(\\d+)/(\\d+)/(\\d+)", "$3-$2-$1"), replace_all("hello world", "\\b\\w", upper))
print(matches("AB-12", "[A-Z]\{2\}-[0-9]+"), matches("ab-12", "(?i)[A-Z]\{2\}-[0-9]+"), find_all("x<b>y</b>", "<.+?>"))
print(format_number(1234567.891, 2), format_number(1234567.891, 2, "fr"), format_number(-42000), pad_left(format_number(3.5, 2), 8))
print(words("Hello, world! It's well-known."), trim_start("  hi  "), trim_end("  hi  "), equals_ignore_case("Été", "ÉTÉ"))
```

```text
["2024-02-29", "2024-03-02"]
["3", "12", "1"] ["a", "b", "c"]
Some(["v2.14", "2", "14"]) Some({year: "2024", month: "02"})
2024-02-29 Hello World
true true ["<b>", "</b>"]
1,234,567.89 1 234 567,89 -42,000     3.50
["Hello", "world", "It's", "well-known"] hi     hi true
```

Une erreur dans une regex écrite dans le code est montrée avant de lancer le
programme :

```rythme
code = "AB-12"
print(matches(code, "[A-Z]+-([0-9]+"))
```

```text
erreur[E026]: regex invalide : cette `(` n'est jamais fermée
 --> 67-regex.ry:2:29
  |
2 | print(matches(code, "[A-Z]+-([0-9]+"))
  |                             ^ ouverte ici
  |
  = aide: ajoutez une `)` là où le groupe finit ; pour le signe `(` lui-même, écrivez `[(]`
  = en savoir plus : `rythme explain E026`
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
rythme [--lang en|fr|es] [--no-color] [COMMANDE] [ARGUMENTS...]
```

| Commande | Autres noms | Ce qu'elle fait |
|----------|-------------|-----------------|
| *(aucune)* | `repl` | le mode interactif (voir [7.2](#72-le-mode-interactif-repl)) |
| `FICHIER [ARGS...]` | `run FICHIER [ARGS...]`, `lancer` | vérifie puis exécute le programme ; `ARGS` est donné à `args()`. Sans fichier, dans un projet : son programme (voir [7.7](#77-projets-et-paquets)), et de même pour `test`, `check`, `fmt` et `doc` |
| `check FICHIER...` | `verifier`, `vérifier` | vérifie chaque programme sans l'exécuter (voir [5.5](#55-la-vérification-avant-lexécution)) |
| `test CHEMIN... [--filter TEXTE] [--seed N] [--watch] [-- ARGS...]` | `tester` | lance les tests de fichiers et de dossiers (voir [4.8](#48-tests)) |
| `fmt [--check] FICHIER...` | `format`, `formater` | donne au code sa mise en forme officielle ; aussi un dossier, ou `-` pour l'entrée standard (voir [7.4](#74-la-mise-en-forme--rythme-fmt)) |
| `doc [--html] [--out FICHIER] FICHIER...` | `documenter` | écrit la documentation de fichiers ou de dossiers, en Markdown ou sous forme de page web (voir [7.6](#76-la-documentation--rythme-doc)) |
| `new NOM` | `nouveau` | crée le dossier d'un nouveau projet (voir [7.7](#77-projets-et-paquets)) |
| `install` | `installer` | télécharge les dépendances du projet et écrit `rythme.lock` (voir [7.7](#77-projets-et-paquets)) |
| `exercise [list \| start NOM \| check FICHIER]` | `exercises`, `exercice`, `exercices` | les exercices intégrés (voir plus bas) |
| `steps FICHIER [EXPRESSION] [--max N] [--width N]` | `etapes`, `étapes` | montre l'évaluation étape par étape (voir [7.8](#78-étape-par-étape--rythme-etapes)) |
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
tout ce qui suit le nom du fichier va au programme (pour `rythme test`, ce
qui vient après `--`). `--lang` accepte toute
valeur qui commence par le code d'une langue, `en`, `fr` ou `es`
(`--lang es`, `--lang=en`, `--lang fr_CA`) ; une autre valeur, ou `--lang`
sans valeur, est signalée, avec le code de sortie 2.

**Langues des messages.** Chaque message, sujet d'aide, explication d'un code
et page de `rythme learn` existe en anglais, en français et en espagnol
(`--lang es`) ; les mots-clés, les noms des fonctions intégrées et les
commandes restent les mêmes dans toutes les langues. Les fichiers écrits par
`rythme new` suivent aussi la langue ; les exercices existent en anglais et
en français, et sont en anglais pour les autres langues. Les textes des
autres langues sont rangés dans des catalogues (`src/i18n/`) ; CONTRIBUTING.md
explique comment en ajouter une.

**Exercices.** `rythme exercise` (ou `rythme exercise list`) liste les 30
exercices gradués intégrés au programme (les fichiers de
[`docs/exercises/`](exercises/README.fr.md)). `start NOM` écrit l'exercice
désigné par son numéro (`7`) ou son nom de fichier (`07-recursivite`,
`recursivite`) dans le dossier actuel, dans la langue des messages (anglais
ou français) ; il ne
remplace jamais un fichier existant (code de sortie 1). `check FICHIER` lance
les tests du fichier comme `rythme test`, puis explique quoi faire ensuite ;
`FICHIER` peut aussi être le numéro d'un exercice commencé dans le dossier
actuel. Son code de sortie est 0 quand tous les tests passent et que les tests
de l'exercice n'ont pas été modifiés, 1 sinon. Une action ou un exercice
inconnu donne le code de sortie 2.

**Sujets d'aide.** `rythme help SUJET` accepte `start`, `syntax`,
`functions`, `lists`, `text`, `records`, `types`, `match`, `errors`,
`functional`, `tests`, `exercises`, `modules`, `system`, `projects`, `dates`,
`repl`, `fmt`, `doc`, `steps`, `editors`, `ruby` et `builtins`, ainsi que des
noms français ou alternatifs : `syntaxe`, `fonctions`, `listes`, `texte`,
`enregistrements`, `motifs`, `erreurs`, `fonctionnel`, `boucles`, `exercices`,
`fichiers`, `système`, `dossiers`, `chemins`, `projets`, `paquets`,
`interactif`, `formater`, `documentation`, `etapes`, `lsp`, `vscode`,
`éditeurs`, `blocs`, `symboles`, `bibliothèque`... et des noms espagnols :
`sintaxis`, `funciones`, `listas`, `registros`, `tipos`, `patrones`,
`errores`, `pruebas`, `ejercicios`, `proyectos`, `fechas`, `pasos`...

**Codes de sortie.**

| Code | Signification |
|------|---------------|
| 0 | succès ; `check` sans erreur (avertissements permis) ; tous les tests réussis, ou aucun test ; `fmt --check` avec tous les fichiers déjà en forme |
| 1 | une erreur dans le programme (syntaxe, vérification ou exécution), un test échoué, un fichier introuvable, ou des fichiers à mettre en forme pour `fmt --check` |
| 2 | une mauvaise utilisation de la commande : commande ou option inconnue, nom de fichier manquant, langue, sujet d'aide ou code d'erreur inconnu |
| 0 à 255 | le code donné à `exit(code)` par le programme |
| 101 | une erreur interne de Rythme (un bug, à signaler) |

**Environnement.**

- `RYTHME_LANG` : la langue des messages (`en...`, `fr...` ou `es...`). Sans
  elle, la première variable non vide parmi `LC_ALL`, `LC_MESSAGES`, `LANG` et
  `LANGUAGE` décide : le français si elle commence par `fr`, l'espagnol si
  elle commence par `es` (`LANG=es_ES.UTF-8`), l'anglais sinon.
  `--lang` l'emporte sur toutes.
- `NO_COLOR` : quand elle est définie (avec n'importe quelle valeur), les
  messages sont sans couleurs ; `--no-color` fait de même. Les couleurs ne
  sont utilisées que quand la sortie d'erreur est un terminal (sous Windows,
  seulement dans les terminaux connus pour les accepter).
- `HOME` (`USERPROFILE` sous Windows) : là où le mode interactif garde son
  historique, et où `rythme install` garde les paquets (`~/.rythme/packages`).
- `RYTHME_HOME` : un autre dossier pour les paquets (`RYTHME_HOME/packages`).

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
$ rythme --lang de bonjour.ry
langue inconnue `de` : écrivez `--lang` et l'une de en, fr, es, comme `rythme --lang fr bonjour.ry`
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
| `:lang en\|fr\|es` | `:langue` | changer la langue des messages |
| `:learn` | `:apprendre` | la visite guidée |
| `:explain CODE` | `:expliquer` | expliquer un code d'erreur |
| `:names` | `:noms` | les noms définis jusqu'ici |
| `:steps EXPRESSION` | `:etapes`, `:étapes` | montre l'évaluation de l'expression étape par étape (voir [7.8](#78-étape-par-étape--rythme-etapes)) |

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
une erreur est laissé tel quel, et l'erreur est affichée), un bouton
**Étapes** qui montre l'évaluation étape par étape comme `rythme etapes`
(voir [7.8](#78-étape-par-étape--rythme-etapes)), un bouton pour
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
et les commentaires en fin de lignes consécutives sont alignés ; un bloc
donné à un appel écrit sur une ligne reste sur une ligne, avec une espace
autour de ses parties (`xs.map do |x| x * 2 end`, `xs.map { |x| x * 2 }`),
et sur plusieurs lignes ses instructions vont un niveau plus loin que la
ligne de `do`. Là où le langage laisse le choix, les retours à la ligne sont gardés, et les lignes
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
`##`, la signature que le vérificateur déduit pour une fonction, comme
`fn aire(cote: Int|Float) -> Int|Float`, l'explication d'un code d'erreur), la complétion, les paramètres d'un
appel, aller à la définition, les références, renommer, le plan du fichier,
les corrections rapides des fautes de frappe, la mise en forme comme
`rythme fmt`, les types dont le vérificateur est sûr à côté des noms (comme
`noms: List(Text)` ou `moitie(x: Int|Float)`), des couleurs qui suivent ce qu'est chaque nom (jetons
sémantiques), *Lancer* et *Lancer le test* au-dessus du code (« code
lenses »), les noms de tous les fichiers de l'espace de travail, le pliage et
l'agrandissement de la sélection. Les messages sont dans la langue de l'éditeur, sauf si un
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

### 7.6 La documentation : `rythme doc`

```text
rythme doc [--html] [--out FICHIER] FICHIER_OU_DOSSIER...
```

`rythme doc` écrit la documentation de programmes à partir de leurs
commentaires `##` (voir [1.2](#12-commentaires)), de leurs types et des
définitions de leurs fonctions. Rien n'est exécuté.

- `rythme doc geometrie.ry` écrit du Markdown sur la sortie standard
  (`geometrie` suffit quand `geometrie.ry` existe). Un dossier représente
  tous les fichiers `.ry` qu'il contient, ainsi que ceux de ses
  sous-dossiers, les dossiers cachés mis à part ; on peut donner plusieurs
  fichiers et dossiers.
- Un fichier est documenté avec les fichiers qu'il utilise (`use geometrie`,
  `use formes/cercle as cercle`, `use "geometrie.ry"`), chacun une seule
  fois : la documentation d'un programme comprend celle de ses modules.
- `--html` écrit à la place une seule page web, avec l'allure du site,
  claire ou sombre comme le système. Elle n'a besoin de rien d'autre (pas de
  script, pas d'autre fichier) : on peut l'ouvrir depuis le disque ou la
  mettre sur n'importe quel serveur web.
- `--out FICHIER` (`-o`) écrit dans un fichier et le dit ; un nom qui finit
  par `.html` donne la page web.
- `documenter` est un autre nom de `doc` ; les titres (*Fonctions*,
  *Valeurs*...) sont dans la langue des messages.
- Un fichier illisible, ou qui a une erreur de syntaxe, est signalé et
  laissé de côté ; les autres sont documentés, et le code de sortie est 1.

Pour chaque fichier viennent : sa description, les lignes `##` du début du
fichier quand une ligne vide les sépare de ce qui suit ; les fichiers qu'il
utilise, sous forme de liens (avec le nom donné par `as` quand ce n'est pas
celui du fichier) ; ses types ; ses fonctions, avec la première
ligne de **chaque** définition (motifs et conditions `if` compris) ; ses
valeurs (avec leur définition quand elle tient sur une ligne) ; et les noms
de ses tests. Les noms qui commencent par `_` sont laissés de côté, comme
des détails de leur fichier.

Dans un commentaire, un nom entre accents graves, comme `` `aire` `` ou
`` `len(liste)` ``, devient un lien vers sa documentation ; un constructeur
mène à son type ; dans un fichier qui a `use geometrie`,
`` `geometrie.aire` `` mène à `aire` dans `geometrie.ry` ; les fonctions
intégrées citées ainsi sont expliquées à la fin. Une ligne `##` vide commence un nouveau paragraphe, et les lignes qui
commencent par `- ` forment une liste. En Markdown, le code est dans des
blocs `~~~rythme`, et les liens utilisent les ancres que GitHub donne aux
titres.

Avec `geometrie.ry` :

```rythme
## Des formes et leurs aires.

## Une forme à dessiner.
type Forme = Cercle(rayon) | Rect(largeur, hauteur)

## L'aire d'une forme : pour un `Cercle`, `pi` fois le carré
## de son rayon. Pour compter des formes, utilisez `len`.
fn aire(Cercle(r)) = pi * _carre(r)
fn aire(Rect(l, h)) = l * h

## Le nombre de côtés d'un carré.
cotes = 4

fn _carre(x) = x * x

test "aire d'un carré" = aire(Rect(2, 2)) == 4
```

`rythme doc geometrie.ry` écrit :

```text
# geometrie.ry

Des formes et leurs aires.

## Types

### Forme

~~~rythme
type Forme = Cercle(rayon) | Rect(largeur, hauteur)
~~~

Une forme à dessiner.

## Fonctions

### aire

~~~rythme
fn aire(Cercle(r))
fn aire(Rect(l, h))
~~~

L'aire d'une forme : pour un [`Cercle`](#forme), `pi` fois le carré
de son rayon. Pour compter des formes, utilisez [`len`](#len).

## Valeurs

### cotes

~~~rythme
cotes = 4
~~~

Le nombre de côtés d'un carré.

## Tests

- aire d'un carré

## Fonctions intégrées citées

### len

~~~rythme
len(x)
~~~

Longueur d'un texte (en caractères), d'une liste, d'un enregistrement, d'un dictionnaire ou d'un ensemble. Pareil que : `length`, `size` (ses noms Ruby).

Exemple :

~~~rythme
len("été")                 # 3
len([1, 2])                # 2
~~~
```

`rythme doc --html . --out doc.html` écrit la même chose pour tous les
fichiers du dossier, en une seule page, avec un sommaire.

### 7.7 Projets et paquets

Un **projet** est un dossier avec un fichier `rythme.toml`, son code dans
`src/` et ses tests dans `tests/`. Un projet peut utiliser d'autres projets,
ses **dépendances**, depuis git ou depuis un dossier : ce sont des
**paquets**.

**Créer un projet.** `rythme new NOM` crée le dossier `NOM` avec un petit
programme qui marche déjà :

```text
$ rythme new bonjour
Projet `bonjour` créé :
  bonjour/rythme.toml
  bonjour/src/main.ry
  bonjour/src/salutation.ry
  bonjour/tests/salutation_test.ry
  bonjour/README.md
  bonjour/.gitignore
Ensuite : `cd bonjour`, puis `rythme run` pour le lancer et `rythme test` pour le tester (voir `rythme help projects`)
$ cd bonjour
$ rythme run
Bonjour, le monde !
$ rythme test
Exécution de 1 test(s)
  ✓ bonjour
1 test(s) : 1 réussi(s), 0 échoué(s)
```

Le nom d'un projet est un nom Rythme en minuscules (lettres, chiffres et
`_`, pas un mot-clé), car il sert dans `use` : `rythme new mon-app` est
E025, avec l'aide `essayez rythme new mon_app` (code de sortie 2). Un
dossier qui existe déjà et n'est pas vide n'est jamais touché (code de
sortie 1). Les fichiers sont écrits dans la langue des messages
(`src/greeting.ry` en anglais, `src/saludo.ry` en espagnol).

**`rythme.toml`.** Chaque valeur est un texte entre guillemets ; `#`
commence un commentaire.

```toml
name = "bonjour"          # obligatoire : le nom du paquet
version = "0.1.0"         # obligatoire : trois nombres, majeure.mineure.correctif
entry = "src/main.ry"     # le programme ; celui-ci quand il n'est pas donné

[dependencies]
mathlib = { git = "https://github.com/quelqu-un/mathlib", tag = "v1.0.0" }
geometrie = { git = "https://github.com/quelqu-un/geometrie", commit = "4d868c2" }
outils = { path = "../outils" }
```

Une dépendance venant de git donne son adresse et une version fixe : un
`tag` ou un `commit` (au moins 4 chiffres hexadécimaux). Une dépendance
venant d'un dossier donne son `path`, relatif au projet. Le nom avant `=`
est le nom utilisé dans `use`. C'est une petite partie de TOML, lue par
Rythme lui-même : des textes avec `"` (avec `\"`, `\\`, `\n`, `\t`) ou `'`,
et des tables sur une ligne. Toute autre clé, section (`[package]` compris)
ou sorte de valeur est E025, montrée à sa place dans `rythme.toml`, avec une
suggestion quand une clé proche existe :

```text
erreur[E025]: clé inconnue `tga`
 --> rythme.toml:6:32
  |
6 | outils = { path = "../outils", tga = "x" }
  |                                ^^^
  |
  = note: les clés ici sont `git`, `tag`, `commit`, `path`
  = aide: vouliez-vous dire `tag` ?
  = en savoir plus : `rythme explain E025`
```

**`rythme install`.** Dans le dossier du projet (ou l'un de ses dossiers),
`rythme install` télécharge chaque dépendance venant de git, puis les
dépendances de chaque dépendance, et écrit `rythme.lock` :

```text
$ rythme install
Installation des dépendances de `bonjour`
  ✓ mathlib 1.0.0 : tag v1.0.0, commit 4d868c2 (téléchargé)
  ✓ outils 0.2.0 : dossier ../outils
2 paquet(s) prêt(s)
```

- Le téléchargement utilise le programme `git` (`git clone`, puis le commit
  du tag ou du commit). Sans lui, `rythme install` s'arrête avec E025 et dit
  où le trouver ; les dépendances venant d'un dossier n'en ont pas besoin.
- Chaque paquet venant de git est gardé dans son propre dossier sous
  `RYTHME_HOME/packages` (`~/.rythme/packages` par défaut), nommé d'après
  son adresse et sa version, et partagé par tous les projets de
  l'ordinateur. Un paquet déjà là n'est pas téléchargé à nouveau.
- Chaque dépendance doit elle-même être un projet, avec son `rythme.toml`.
- `rythme.lock` garde, pour chaque paquet venant de git, son adresse, son
  tag (ou le commit demandé), et le **commit exact** installé. Gardez-le
  avec le projet : `rythme install` sur un autre ordinateur installe alors
  les mêmes commits, même si un tag a été déplacé. Quand `rythme.toml`
  demande une autre version, `rythme install` la prend et met `rythme.lock`
  à jour ; pour prendre le nouveau commit d'un tag déplacé, supprimez
  `rythme.lock`.
- Un tag ou un commit qui n'existe pas, une adresse que git ne peut pas
  atteindre, un dossier qui n'existe pas : E025, avec ce que git a dit, la
  ligne de `rythme.toml`, et le tag le plus proche quand il y en a un.

**Utiliser un paquet.** `use NOM/FICHIER` est le fichier `src/FICHIER.ry`
du paquet `NOM`, et s'utilise par la dernière partie du chemin, comme tout
`use` (voir [4.9](#49-modules--use)) :

```rythme
# src/main.ry
use mathlib/vecteurs      # le fichier src/vecteurs.ry de mathlib
use mathlib               # le programme de mathlib : l'`entry` de son rythme.toml
use bonjour/salutation    # un fichier du projet lui-même (`bonjour`), utile dans tests/
print(vecteurs.ajouter({x: 1, y: 2}, {x: 3, y: 4}))
```

- Les paquets sont trouvés par le `rythme.toml` le plus proche au-dessus du
  fichier qui contient le `use` : dans un paquet, ce sont ses propres
  dépendances qui servent.
- Un fichier à côté de celui qui contient le `use` passe en premier : `use
  mathlib/vecteurs` est d'abord `mathlib/vecteurs.ry` dans le dossier du
  fichier.
- Les noms privés (`_nom`) d'un paquet restent privés, et ses fichiers sont
  vérifiés comme les autres.
- Un paquet qui n'est pas installé est E025, sur le `use`, avec l'aide de
  lancer `rythme install` ; un fichier que le paquet n'a pas est E017, avec
  le fichier le plus proche. Un `use` qui ne désigne rien dans un projet dit
  quels paquets il peut atteindre.
- Le serveur de langage connaît aussi les paquets : pas de faux problème sur
  `use mathlib/vecteurs`, complétion des noms de paquets après `use `, et de
  leurs fichiers après `use mathlib/`.

**Les commandes sans fichier.** Dans le dossier d'un projet ou dans l'un de
ses dossiers, les commandes prennent les fichiers du projet quand aucun
fichier n'est donné :

| Commande | Travaille sur |
|----------|---------------|
| `rythme run` | le programme, `entry` de `rythme.toml` |
| `rythme test` | chaque fichier de `src/` et de `tests/` (et de leurs dossiers) qui a des tests, l'un après l'autre, chacun avec son nom, puis un bilan, comme `rythme test DOSSIER` (`--filter`, `--seed`, `--watch` et `--` marchent de même) ; le programme quand aucun n'en a |
| `rythme check` | le programme, puis chaque fichier de `src/` et de `tests/` |
| `rythme fmt` | les dossiers `src/` et `tests/` (`rythme fmt --check` aussi) |
| `rythme doc` | le dossier `src/` (`--html`, `--out` marchent comme d'habitude) |

Hors d'un projet, un fichier manquant reste une erreur d'utilisation (code
de sortie 2). Un fichier donné par son nom n'a jamais besoin de
`rythme.toml`, même dans un projet.

### 7.8 Étape par étape : `rythme etapes`

```text
rythme etapes FICHIER [EXPRESSION] [--max N] [--width N]
```

`rythme etapes` (en anglais `rythme steps`) montre comment un programme
calcule ses valeurs, une étape à la fois, pour apprendre et enseigner. Il
suit le **modèle par substitution** : un appel est remplacé par le corps de
sa fonction, où les valeurs des arguments sont écrites à la place des
paramètres, jusqu'à ce qu'il ne reste que le résultat.

- Sans `EXPRESSION`, chaque définition (`nom = ...`) et expression du niveau
  principal du fichier est montrée, dans l'ordre, après la définition de ses
  fonctions et de ses types, comme quand le fichier s'exécute. Avec
  `EXPRESSION` (`rythme etapes fib.ry "fib(5)"`), les définitions du fichier
  sont faites sans être montrées, ses expressions sont sautées, et seule
  l'expression est montrée.
- La première ligne est l'expression telle qu'elle est écrite (les pipes sont
  montrés comme les appels qu'ils font) ; chaque ligne suivante, après `→`,
  est l'expression après une étape de plus, écrite comme `rythme fmt` écrit
  le code.
- Une étape est **un appel ou une opération**, dans l'ordre d'évaluation de
  l'interpréteur (voir [3](#3-expressions)). Le corps d'une fonction appelée
  est montré déjà simplifié autant que possible sans appeler une autre
  fonction : les opérations sur des valeurs connues sont faites, la branche
  de `if` et le cas de `match` sont choisis. Un nom du programme est remplacé
  par sa valeur quand elle est nécessaire. Les fonctions intégrées, et les
  fonctions d'autres fichiers, donnent leur résultat en une étape.
- Ce que le programme affiche apparaît entre les étapes, là où cela se
  produit ; une erreur arrête l'évaluation avec le même message que quand le
  programme s'exécute.
- `--max N` montre au plus `N` étapes de chaque expression (200 par
  défaut) : ensuite, `...` dit que le reste est calculé sans être montré, et
  la dernière ligne est le résultat. `--width N` coupe les lignes de plus de
  `N` caractères (100 par défaut, 20 au moins) avec `…` ; une étape est
  montrée sur 24 lignes au plus. Un calcul qui ne finit jamais est arrêté
  après 10 000 000 d'étapes, et un calcul imbriqué sur plus de 10 000
  niveaux (1 200 dans le playground) est arrêté aussi, avec un message.

La même chose existe dans le mode interactif avec `:etapes EXPRESSION` (voir
[7.2](#72-le-mode-interactif-repl)) et dans le playground avec le bouton
**Étapes** (voir [7.3](#73-le-playground-web)). Par exemple, avec ce
programme :

```rythme
fn fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2)
fn total([]) = 0
fn total([x, ..rest]) = x + total(rest)
fn describe(n)
  kind = if n % 2 == 0 then "pair" else "impair"
  "{n} est {kind}"
end
print(fib(3))
t = total([10, 20, 30])
describe(t // 4)
[1, 2, 3] |> map(fn(x) = x * t)
```

`rythme etapes` montre :

```text
print(fib(3))
  → print(fib(2) + fib(1))
  → print(fib(1) + fib(0) + fib(1))
  → print(1 + fib(0) + fib(1))
  → print(1 + 0 + fib(1))
  → print(1 + fib(1))
  → print(1 + 1)
  → print(2)
2
  → ()

t = total([10, 20, 30])
  → 10 + total([20, 30])
  → 10 + (20 + total([30]))
  → 10 + (20 + (30 + total([])))
  → 10 + (20 + (30 + 0))
  → 10 + (20 + 30)
  → 10 + 50
  → 60

describe(t // 4)
  → describe(60 // 4)
  → describe(15)
  → "15 est impair"

map([1, 2, 3], fn(x) = x * t)
  → [60, 120, 180]
```

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
statement    = fn_def | binding | command | expression ;
command      = NAME [ "?" ] expression { "," { NL } expression }
               [ "do" [ params ] block_end ] ;     (* 3.15 *)

binding      = pattern_atom "=" expression ;
fn_def       = "fn" NAME [ "?" ] "(" [ patterns ] ")" [ "if" expression ] body ;
body         = "=" result
             | NL block_end ;
result       = command | expression ;              (* 3.15 *)
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
             | "." MEMBER [ "(" [ arguments ] ")" ] | block ;
block        = "do" [ params ] block_end
             | "{" params { statement NL } [ statement ] "}" ;
params       = "|" [ pattern_atom { "," pattern_atom } ] "|" | "||" ;
arguments    = expression { "," expression } ;

primary      = INT | FLOAT | TEXT | SYMBOL | "true" | "false" | NAME | CTOR
             | "(" ")" | "(" expression ")"
             | "[" [ list_item { "," list_item } ] "]"
             | "{" [ record_item { "," record_item } ] "}"
             | "fn" "(" [ patterns ] ")" body
             | "if" expression "then" result "else" result
             | "unless" expression "then" result "else" result
             | "match" expression NL case { NL case } NL "end"
             | "do" block_end ;
list_item    = [ ".." ] expression ;
record_item  = ".." expression | KEY ":" expression | NAME ;
case         = pattern [ "if" expression ] "->" result ;

patterns     = pattern { "," pattern } ;
pattern      = pattern_atom { "|" pattern_atom } ;
pattern_atom = "_" | NAME
             | CTOR [ "(" [ patterns ] ")" ]
             | [ "-" ] INT | [ "-" ] FLOAT | PLAIN_TEXT | SYMBOL | "true" | "false"
             | "(" ")" | "(" pattern ")"
             | "[" [ list_patterns ] "]"
             | "{" [ field_pattern { "," field_pattern } ] "}" ;
list_patterns = ".." [ NAME ] [ "," ]
             | pattern [ "," [ list_patterns ] ] ;
field_pattern = KEY ":" pattern | NAME | ".." ;

MEMBER       = NAME [ "?" ] | CTOR | mot-clé | mot réservé ;
SYMBOL       = ":" NAME ;
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
- `NAME [ "?" ]` : un seul nom seulement quand le `?` touche le nom et que
  les règles de [1.4](#14-noms) le disent ; sinon le `?` est l'opérateur
  `postfix`. Un `NAME` suivi d'un `?` qui fait partie du nom (`even?`) est
  aussi un `primary`.
- `block` : seulement après un appel, un appel de méthode, un `.` MEMBER ou
  un NAME, avec `do` ou `{` sur la même ligne
  ([3.14](#314-blocs-donnés-aux-appels)) ; c'est le dernier argument de
  l'appel. `{` suivi de `|` ou `||` commence toujours un bloc, jamais un
  enregistrement.
- `"unless"` : le mot commence un `unless` seulement comme décrit en
  [1.5](#15-mots-clés-et-mots-réservés) ; ailleurs c'est un NAME.
- `SYMBOL` : `:` suivi directement d'un nom, sauf le `:` qui suit un nom de
  champ après `{` ou `,` ([2.14](#214-symbol)).
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
| chiffres d'un nombre entier (Int), dans le code, comme résultat ou lu par `int` | 100 000 | E023 |
| éléments d'une liste (construite par un intervalle, `+`, `..xs`, `repeat`, `flatten`, `flat_map`, `chars`) | 50 000 000 | E023 |
| longueur d'un texte (construit par `+`, `{...}`, `join`, `replace`, `repeat`, `pad_left`, `pad_right`) | 100 000 000 octets en UTF-8 | E023 |
| imbrication des parenthèses, crochets, blocs et autres constructions dans le code | 256 niveaux | E001 |
| profondeur d'une expression : chaque opérateur, appel, indice ou champ compte pour un niveau, donc `1 + 1 + ...` contient au plus 1000 nombres | 1000 niveaux | E001 |
| textes imbriqués dans des `{...}` de textes | 32 niveaux | E001 |
| appels imbriqués (non terminaux) | selon la fonction et la pile : plusieurs centaines de milliers avec `rythme` (environ 900 000 pour une récursion simple), 600 dans le playground | E014 |
| étapes de `rythme etapes` : montrées pour chaque expression ; en tout ; profondeur d'une expression en cours d'évaluation | 200 (`--max`) ; 10 000 000 ; 10 000 niveaux, 1 200 dans le playground | — |
| chiffres de `round(x, chiffres)` | ramenés entre −15 et 15 | — |
| décimales de `text(x, decimales)` | de 0 à 100 | E023 |
| indentation de `to_json(valeur, indentation)` | de 0 à 16 espaces | E023 |
| codes de sortie | de 0 à 255 | E023 |

Les appels terminaux et les fonctions de listes (`map`, `fold`, `sort`...) ne
comptent pas comme des appels imbriqués, et `parse_json` et `to_json` lisent
et écrivent des données imbriquées sur des centaines de milliers de niveaux
(indentées par `to_json(valeur, indentation)`, de telles données seraient
trop grandes : E023).
