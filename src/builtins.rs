//! The standard library: functions available everywhere, each with its
//! documentation in English and French and an example.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::Write;
use std::rc::Rc;

use crate::diagnostic::{Diagnostic, Fallible};
use crate::interp::{binop, overflow, resolve_index, Interp, Wrap};
use crate::source::{self, Span};
use crate::value::*;

pub type BuiltinFn = fn(&mut Interp, &[Value], Span) -> Fallible<Value>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Category {
    Basics,
    Numbers,
    Text,
    Lists,
    Records,
    Options,
    Files,
}

impl Category {
    pub const ALL: [Category; 7] =
        [Category::Basics, Category::Numbers, Category::Text, Category::Lists, Category::Records, Category::Options, Category::Files];

    pub fn title(self) -> String {
        match self {
            Category::Basics => t!("Basics", "Bases"),
            Category::Numbers => t!("Numbers", "Nombres"),
            Category::Text => t!("Text", "Texte"),
            Category::Lists => t!("Lists", "Listes"),
            Category::Records => t!("Records", "Enregistrements"),
            Category::Options => t!("Option and Result", "Option et Result"),
            Category::Files => t!("Files", "Fichiers"),
        }
    }
}

pub struct Builtin {
    pub name: &'static str,
    pub sig: &'static str,
    pub min: usize,
    pub max: usize,
    pub cat: Category,
    pub en: &'static str,
    pub fr: &'static str,
    pub example: &'static str,
    pub func: BuiltinFn,
}

impl Builtin {
    pub fn doc(&self) -> &'static str {
        crate::lang::pick(self.en, self.fr)
    }
}

const MANY: usize = usize::MAX;

macro_rules! b {
    ($name:literal, $sig:literal, $min:expr, $max:expr, $cat:ident, $f:ident, $en:literal, $fr:literal, $ex:literal) => {
        Builtin { name: $name, sig: $sig, min: $min, max: $max, cat: Category::$cat, en: $en, fr: $fr, example: $ex, func: $f }
    };
}

pub fn lookup(name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|b| b.name == name)
}

pub static BUILTINS: &[Builtin] = &[
    // ----- Basics -----
    b!("print", "print(value, ...)", 0, MANY, Basics, print,
       "Shows values on the screen, separated by spaces, then goes to the next line.",
       "Affiche des valeurs à l'écran, séparées par des espaces, puis passe à la ligne.",
       "print(\"Hello\", 42)        # Hello 42"),
    b!("debug", "debug(value)", 1, 1, Basics, debug,
       "Shows a value with its position in the code (on the error output) and returns it unchanged. Handy in the middle of a pipeline.",
       "Affiche une valeur avec sa position dans le code (sur la sortie d'erreur) et la renvoie telle quelle. Pratique au milieu d'un enchaînement.",
       "[1, 2] |> debug |> sum      # [debug file.ry:1:10] [1, 2]"),
    b!("text", "text(value)", 1, 1, Basics, to_text,
       "Converts any value to text.",
       "Convertit n'importe quelle valeur en texte.",
       "text(42) + \"!\"            # \"42!\""),
    b!("input", "input(prompt)", 0, 1, Basics, input,
       "Shows the prompt, then reads a line typed on the keyboard and returns it as text.",
       "Affiche l'invite, puis lit une ligne tapée au clavier et la renvoie sous forme de texte.",
       "name = input(\"Your name? \")"),
    b!("type_of", "type_of(value)", 1, 1, Basics, type_of,
       "Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", or the name of your own type.",
       "Renvoie le nom du type d'une valeur : \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", ou le nom de votre propre type.",
       "type_of([1, 2])            # \"List\""),
    b!("help", "help(value)", 0, 1, Basics, help,
       "Shows help: about a function (`help(map)`), a topic (`help(\"lists\")`), or what you can do with a value (`help([1, 2])`).",
       "Affiche de l'aide : sur une fonction (`help(map)`), un sujet (`help(\"lists\")`), ou ce qu'on peut faire avec une valeur (`help([1, 2])`).",
       "help(map)"),
    b!("assert", "assert(condition, message)", 1, 2, Basics, assert,
       "Stops with an error if the condition is false. Mostly used in tests.",
       "S'arrête avec une erreur si la condition est fausse. Surtout utilisé dans les tests.",
       "assert(1 + 1 == 2, \"maths are broken\")"),
    b!("assert_eq", "assert_eq(actual, expected)", 2, 2, Basics, assert_eq,
       "Stops with an error showing both values if they are different. Mostly used in tests.",
       "S'arrête avec une erreur montrant les deux valeurs si elles sont différentes. Surtout utilisé dans les tests.",
       "assert_eq(sum([1, 2]), 3)"),
    b!("fail", "fail(message)", 1, 1, Basics, fail,
       "Stops the program with an error message. Prefer returning `Err(...)` for errors that can be handled.",
       "Arrête le programme avec un message d'erreur. Préférez renvoyer `Err(...)` pour les erreurs qui peuvent être traitées.",
       "fail(\"the file is corrupted\")"),
    b!("args", "args()", 0, 0, Basics, args,
       "Returns the list of the words given after the file name on the command line.",
       "Renvoie la liste des mots donnés après le nom du fichier sur la ligne de commande.",
       "# rythme hello.ry Ada   ->   args() == [\"Ada\"]"),
    b!("exit", "exit(code)", 0, 1, Basics, exit,
       "Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not.",
       "Arrête immédiatement le programme. Le code (0 par défaut) indique au système si tout s'est bien passé (0) ou non.",
       "exit(1)"),
    b!("time", "time()", 0, 0, Basics, time,
       "Returns the current time in seconds (a Float). Subtract two times to measure a duration.",
       "Renvoie l'heure actuelle en secondes (un Float). Soustrayez deux instants pour mesurer une durée.",
       "start = time()"),
    b!("random", "random(min, max)", 0, 2, Basics, random,
       "random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.",
       "random() donne un nombre décimal entre 0 et 1 ; random(a, b) un nombre entier de a à b (inclus) ; random(liste) un élément au hasard.",
       "random(1, 6)               # a dice roll"),
    // ----- Numbers -----
    b!("abs", "abs(x)", 1, 1, Numbers, abs,
       "Absolute value: the number without its sign.",
       "Valeur absolue : le nombre sans son signe.",
       "abs(-3)                    # 3"),
    b!("min", "min(a, b, ...)", 1, MANY, Numbers, min,
       "The smallest of the values, or of the elements of a list.",
       "La plus petite des valeurs, ou des éléments d'une liste.",
       "min(3, 1, 2)               # 1\nmin([4, 8])                # 4"),
    b!("max", "max(a, b, ...)", 1, MANY, Numbers, max,
       "The largest of the values, or of the elements of a list.",
       "La plus grande des valeurs, ou des éléments d'une liste.",
       "max(3, 1, 2)               # 3"),
    b!("sqrt", "sqrt(x)", 1, 1, Numbers, sqrt,
       "Square root.",
       "Racine carrée.",
       "sqrt(16)                   # 4.0"),
    b!("pow", "pow(x, n)", 2, 2, Numbers, pow,
       "x to the power n (same as `x ** n`).",
       "x à la puissance n (comme `x ** n`).",
       "pow(2, 10)                 # 1024"),
    b!("floor", "floor(x)", 1, 1, Numbers, floor,
       "Rounds down to a whole number.",
       "Arrondit à l'entier inférieur.",
       "floor(2.7)                 # 2"),
    b!("ceil", "ceil(x)", 1, 1, Numbers, ceil,
       "Rounds up to a whole number.",
       "Arrondit à l'entier supérieur.",
       "ceil(2.1)                  # 3"),
    b!("round", "round(x, digits)", 1, 2, Numbers, round,
       "Rounds to the nearest whole number, or to the given number of decimal digits.",
       "Arrondit à l'entier le plus proche, ou au nombre de décimales donné.",
       "round(2.5)                 # 3\nround(3.14159, 2)          # 3.14"),
    b!("int", "int(x)", 1, 1, Numbers, to_int,
       "Converts to a whole number (Int): decimals are cut, texts are read.",
       "Convertit en nombre entier (Int) : les décimales sont coupées, les textes sont lus.",
       "int(\"42\")                  # 42\nint(3.9)                   # 3"),
    b!("float", "float(x)", 1, 1, Numbers, to_float,
       "Converts to a decimal number (Float).",
       "Convertit en nombre décimal (Float).",
       "float(\"2.5\")               # 2.5"),
    b!("sin", "sin(x)", 1, 1, Numbers, sin, "Sine (x in radians).", "Sinus (x en radians).", "sin(pi / 2)                # 1.0"),
    b!("cos", "cos(x)", 1, 1, Numbers, cos, "Cosine (x in radians).", "Cosinus (x en radians).", "cos(0)                     # 1.0"),
    b!("tan", "tan(x)", 1, 1, Numbers, tan, "Tangent (x in radians).", "Tangente (x en radians).", "tan(0)                     # 0.0"),
    b!("log", "log(x, base)", 1, 2, Numbers, log,
       "Logarithm: natural (base e) by default, or in the given base.",
       "Logarithme : naturel (base e) par défaut, ou dans la base donnée.",
       "log(100, 10)               # 2.0"),
    b!("exp", "exp(x)", 1, 1, Numbers, exp, "e to the power x.", "e à la puissance x.", "exp(0)                     # 1.0"),
    b!("is_even", "is_even(n)", 1, 1, Numbers, is_even, "Is the whole number even?", "Le nombre entier est-il pair ?", "is_even(4)                 # true"),
    b!("is_odd", "is_odd(n)", 1, 1, Numbers, is_odd, "Is the whole number odd?", "Le nombre entier est-il impair ?", "is_odd(4)                  # false"),
    // ----- Text -----
    b!("len", "len(x)", 1, 1, Text, len,
       "Length of a text (in characters), a list or a record.",
       "Longueur d'un texte (en caractères), d'une liste ou d'un enregistrement.",
       "len(\"été\")                 # 3\nlen([1, 2])                # 2"),
    b!("upper", "upper(text)", 1, 1, Text, upper, "The text in uppercase.", "Le texte en majuscules.", "upper(\"abc\")               # \"ABC\""),
    b!("lower", "lower(text)", 1, 1, Text, lower, "The text in lowercase.", "Le texte en minuscules.", "lower(\"ABC\")               # \"abc\""),
    b!("trim", "trim(text)", 1, 1, Text, trim,
       "The text without spaces and line breaks at the start and the end.",
       "Le texte sans espaces ni retours à la ligne au début et à la fin.",
       "trim(\"  hi \")              # \"hi\""),
    b!("split", "split(text, separator)", 1, 2, Text, split,
       "Cuts a text into a list of texts. Without separator, cuts at spaces.",
       "Découpe un texte en une liste de textes. Sans séparateur, découpe aux espaces.",
       "split(\"a,b,c\", \",\")        # [\"a\", \"b\", \"c\"]\nsplit(\"hello world\")       # [\"hello\", \"world\"]"),
    b!("join", "join(list, separator)", 1, 2, Text, join,
       "Glues the elements of a list into one text, with the separator between them.",
       "Colle les éléments d'une liste en un seul texte, avec le séparateur entre eux.",
       "join([\"a\", \"b\"], \"-\")      # \"a-b\""),
    b!("contains", "contains(x, item)", 2, 2, Text, contains,
       "Does the text contain this piece of text? Does the list contain this element? Does the record have this field?",
       "Le texte contient-il ce morceau de texte ? La liste contient-elle cet élément ? L'enregistrement a-t-il ce champ ?",
       "contains(\"hello\", \"ell\")   # true\ncontains([1, 2], 3)        # false"),
    b!("starts_with", "starts_with(text, start)", 2, 2, Text, starts_with,
       "Does the text start with this piece of text?",
       "Le texte commence-t-il par ce morceau de texte ?",
       "starts_with(\"hello\", \"he\") # true"),
    b!("ends_with", "ends_with(text, end)", 2, 2, Text, ends_with,
       "Does the text end with this piece of text?",
       "Le texte finit-il par ce morceau de texte ?",
       "ends_with(\"photo.png\", \".png\")  # true"),
    b!("replace", "replace(text, old, new)", 3, 3, Text, replace,
       "Replaces every occurrence of `old` by `new`.",
       "Remplace chaque occurrence de `old` par `new`.",
       "replace(\"a-b-c\", \"-\", \"+\")  # \"a+b+c\""),
    b!("chars", "chars(text)", 1, 1, Text, chars,
       "The list of the characters of a text.",
       "La liste des caractères d'un texte.",
       "chars(\"abc\")               # [\"a\", \"b\", \"c\"]"),
    b!("lines", "lines(text)", 1, 1, Text, lines,
       "The list of the lines of a text.",
       "La liste des lignes d'un texte.",
       "lines(\"a\\nb\")              # [\"a\", \"b\"]"),
    b!("repeat", "repeat(x, n)", 2, 2, Text, repeat,
       "A text or a list repeated n times.",
       "Un texte ou une liste répété n fois.",
       "repeat(\"ab\", 3)            # \"ababab\""),
    b!("parse_int", "parse_int(text)", 1, 1, Text, parse_int,
       "Reads a whole number from a text: `Ok(number)`, or `Err(message)` if the text is not a number.",
       "Lit un nombre entier dans un texte : `Ok(nombre)`, ou `Err(message)` si le texte n'est pas un nombre.",
       "parse_int(\"42\")            # Ok(42)\nparse_int(\"abc\")           # Err(...)"),
    b!("parse_float", "parse_float(text)", 1, 1, Text, parse_float,
       "Reads a decimal number from a text: `Ok(number)` or `Err(message)`.",
       "Lit un nombre décimal dans un texte : `Ok(nombre)` ou `Err(message)`.",
       "parse_float(\"2.5\")         # Ok(2.5)"),
    // ----- Lists -----
    b!("map", "map(list, f)", 2, 2, Lists, map,
       "Applies the function f to each element and returns the list of the results. Also works on Option and Result.",
       "Applique la fonction f à chaque élément et renvoie la liste des résultats. Fonctionne aussi sur Option et Result.",
       "[1, 2, 3] |> map(fn(x) = x * 2)   # [2, 4, 6]"),
    b!("filter", "filter(list, f)", 2, 2, Lists, filter,
       "Keeps only the elements for which f returns true.",
       "Ne garde que les éléments pour lesquels f renvoie true.",
       "[1, 2, 3, 4] |> filter(is_even)   # [2, 4]"),
    b!("fold", "fold(list, start, f)", 3, 3, Lists, fold,
       "Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.",
       "Combine tous les éléments en une seule valeur : part de `start`, puis calcule f(résultat, élément) pour chaque élément.",
       "fold([1, 2, 3], 0, fn(total, x) = total + x)   # 6"),
    b!("each", "each(list, f)", 2, 2, Lists, each,
       "Calls f on each element, for its effect (like printing). Returns ().",
       "Appelle f sur chaque élément, pour son effet (comme afficher). Renvoie ().",
       "1..3 |> each(fn(i) = print(i))"),
    b!("sum", "sum(list)", 1, 1, Lists, sum, "The sum of the numbers of a list.", "La somme des nombres d'une liste.", "sum([1, 2, 3])             # 6"),
    b!("product", "product(list)", 1, 1, Lists, product, "The product of the numbers of a list.", "Le produit des nombres d'une liste.", "product([2, 3, 4])         # 24"),
    b!("first", "first(list)", 1, 1, Lists, first,
       "The first element, as an Option: `Some(x)`, or `None` if the list is empty.",
       "Le premier élément, sous forme d'Option : `Some(x)`, ou `None` si la liste est vide.",
       "first([7, 8])              # Some(7)"),
    b!("last", "last(list)", 1, 1, Lists, last,
       "The last element, as an Option.",
       "Le dernier élément, sous forme d'Option.",
       "last([7, 8])               # Some(8)"),
    b!("rest", "rest(list)", 1, 1, Lists, rest,
       "The list without its first element.",
       "La liste sans son premier élément.",
       "rest([7, 8, 9])            # [8, 9]"),
    b!("get", "get(x, key)", 2, 2, Lists, get,
       "The element at a position of a list or text, or the field of a record, as an Option (never an error).",
       "L'élément à une position d'une liste ou d'un texte, ou le champ d'un enregistrement, sous forme d'Option (jamais d'erreur).",
       "get([7, 8], 5)             # None\nget({a: 1}, \"a\")           # Some(1)"),
    b!("push", "push(list, x)", 2, 2, Lists, push,
       "A new list with x added at the end.",
       "Une nouvelle liste avec x ajouté à la fin.",
       "push([1, 2], 3)            # [1, 2, 3]"),
    b!("reverse", "reverse(x)", 1, 1, Lists, reverse, "The list or text in reverse order.", "La liste ou le texte dans l'ordre inverse.", "reverse([1, 2, 3])         # [3, 2, 1]"),
    b!("sort", "sort(list)", 1, 1, Lists, sort,
       "The list sorted from smallest to largest (numbers, texts in alphabetical order...).",
       "La liste triée du plus petit au plus grand (nombres, textes dans l'ordre alphabétique...).",
       "sort([3, 1, 2])            # [1, 2, 3]"),
    b!("sort_by", "sort_by(list, key)", 2, 2, Lists, sort_by,
       "Sorts the list by the value that the function `key` gives for each element.",
       "Trie la liste selon la valeur que la fonction `key` donne pour chaque élément.",
       "sort_by([\"ccc\", \"a\"], len)  # [\"a\", \"ccc\"]"),
    b!("unique", "unique(list)", 1, 1, Lists, unique,
       "The list without duplicates (the first occurrence is kept).",
       "La liste sans doublons (la première occurrence est gardée).",
       "unique([1, 2, 1])          # [1, 2]"),
    b!("slice", "slice(x, start, end)", 2, 3, Lists, slice,
       "The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.",
       "La partie d'une liste ou d'un texte de la position `start` (incluse) à `end` (exclue). Les positions négatives comptent depuis la fin.",
       "slice([1, 2, 3, 4], 1, 3)  # [2, 3]\nslice(\"hello\", -3)         # \"llo\""),
    b!("take", "take(x, n)", 2, 2, Lists, take, "The first n elements of a list or text.", "Les n premiers éléments d'une liste ou d'un texte.", "take([1, 2, 3], 2)         # [1, 2]"),
    b!("drop", "drop(x, n)", 2, 2, Lists, drop, "The list or text without its first n elements.", "La liste ou le texte sans ses n premiers éléments.", "drop([1, 2, 3], 2)         # [3]"),
    b!("zip", "zip(a, b)", 2, 2, Lists, zip,
       "Pairs the elements of two lists: [[a1, b1], [a2, b2], ...].",
       "Associe les éléments de deux listes : [[a1, b1], [a2, b2], ...].",
       "zip([1, 2], [\"a\", \"b\"])    # [[1, \"a\"], [2, \"b\"]]"),
    b!("enumerate", "enumerate(list)", 1, 1, Lists, enumerate,
       "Pairs each element with its position: [[0, x0], [1, x1], ...].",
       "Associe chaque élément à sa position : [[0, x0], [1, x1], ...].",
       "enumerate([\"a\", \"b\"])      # [[0, \"a\"], [1, \"b\"]]"),
    b!("any", "any(list, f)", 2, 2, Lists, any, "Is f true for at least one element?", "f est-elle vraie pour au moins un élément ?", "any([1, 3, 4], is_even)    # true"),
    b!("all", "all(list, f)", 2, 2, Lists, all, "Is f true for every element?", "f est-elle vraie pour chaque élément ?", "all([2, 4], is_even)       # true"),
    b!("find", "find(list, f)", 2, 2, Lists, find,
       "The first element for which f is true, as an Option.",
       "Le premier élément pour lequel f est vraie, sous forme d'Option.",
       "find([1, 4, 6], is_even)   # Some(4)"),
    b!("count", "count(list, f)", 2, 2, Lists, count, "How many elements make f true.", "Combien d'éléments rendent f vraie.", "count([1, 2, 4], is_even)  # 2"),
    b!("index_of", "index_of(x, item)", 2, 2, Lists, index_of,
       "The position of an element in a list (or of a piece of text in a text), as an Option.",
       "La position d'un élément dans une liste (ou d'un morceau de texte dans un texte), sous forme d'Option.",
       "index_of([5, 6, 7], 6)     # Some(1)"),
    b!("flatten", "flatten(list)", 1, 1, Lists, flatten, "Joins a list of lists into one list.", "Réunit une liste de listes en une seule liste.", "flatten([[1], [2, 3]])     # [1, 2, 3]"),
    b!("flat_map", "flat_map(list, f)", 2, 2, Lists, flat_map,
       "Like map, when f returns a list: the lists are joined.",
       "Comme map, quand f renvoie une liste : les listes sont réunies.",
       "flat_map([1, 2], fn(x) = [x, x])   # [1, 1, 2, 2]"),
    b!("partition", "partition(list, f)", 2, 2, Lists, partition,
       "Splits a list in two: [elements where f is true, elements where f is false].",
       "Sépare une liste en deux : [éléments où f est vraie, éléments où f est fausse].",
       "partition([1, 2, 3], is_odd)   # [[1, 3], [2]]"),
    b!("group_by", "group_by(list, key)", 2, 2, Lists, group_by,
       "Groups the elements in a record: the key is the text of what `key` returns, the value the list of matching elements.",
       "Regroupe les éléments dans un enregistrement : la clé est le texte de ce que renvoie `key`, la valeur la liste des éléments correspondants.",
       "group_by([1, 2, 3], is_odd)    # {true: [1, 3], false: [2]}"),
    b!("is_empty", "is_empty(x)", 1, 1, Lists, is_empty, "Is the list, text or record empty?", "La liste, le texte ou l'enregistrement est-il vide ?", "is_empty([])               # true"),
    // ----- Records -----
    b!("keys", "keys(record)", 1, 1, Records, keys, "The list of the field names of a record.", "La liste des noms des champs d'un enregistrement.", "keys({a: 1, b: 2})         # [\"a\", \"b\"]"),
    b!("values", "values(record)", 1, 1, Records, values, "The list of the values of a record.", "La liste des valeurs d'un enregistrement.", "values({a: 1, b: 2})       # [1, 2]"),
    b!("entries", "entries(record)", 1, 1, Records, entries,
       "The list of the [name, value] pairs of a record.",
       "La liste des paires [nom, valeur] d'un enregistrement.",
       "entries({a: 1})            # [[\"a\", 1]]"),
    b!("has", "has(record, name)", 2, 2, Records, has, "Does the record have this field?", "L'enregistrement a-t-il ce champ ?", "has({a: 1}, \"a\")           # true"),
    b!("put", "put(record, name, value)", 3, 3, Records, put,
       "A new record with the field set to the value (added or replaced).",
       "Un nouvel enregistrement avec le champ fixé à la valeur (ajouté ou remplacé).",
       "put({a: 1}, \"b\", 2)        # {a: 1, b: 2}"),
    b!("remove", "remove(record, name)", 2, 2, Records, remove, "A new record without this field.", "Un nouvel enregistrement sans ce champ.", "remove({a: 1, b: 2}, \"a\")  # {b: 2}"),
    // ----- Option and Result -----
    b!("unwrap", "unwrap(x)", 1, 1, Options, unwrap,
       "The value inside `Some(x)` or `Ok(x)`. Stops the program with an error for `None` or `Err(e)`.",
       "La valeur dans `Some(x)` ou `Ok(x)`. Arrête le programme avec une erreur pour `None` ou `Err(e)`.",
       "unwrap(Some(3))            # 3"),
    b!("unwrap_or", "unwrap_or(x, default)", 2, 2, Options, unwrap_or,
       "The value inside `Some(x)` or `Ok(x)`, or the default value for `None` or `Err(e)`.",
       "La valeur dans `Some(x)` ou `Ok(x)`, ou la valeur par défaut pour `None` ou `Err(e)`.",
       "unwrap_or(None, 0)         # 0"),
    b!("is_some", "is_some(option)", 1, 1, Options, is_some, "Is it `Some(...)`?", "Est-ce `Some(...)` ?", "is_some(Some(1))           # true"),
    b!("is_none", "is_none(option)", 1, 1, Options, is_none, "Is it `None`?", "Est-ce `None` ?", "is_none(None)              # true"),
    b!("is_ok", "is_ok(result)", 1, 1, Options, is_ok, "Is it `Ok(...)`?", "Est-ce `Ok(...)` ?", "is_ok(Ok(1))               # true"),
    b!("is_err", "is_err(result)", 1, 1, Options, is_err, "Is it `Err(...)`?", "Est-ce `Err(...)` ?", "is_err(Err(\"oops\"))       # true"),
    // ----- Files -----
    b!("read_file", "read_file(path)", 1, 1, Files, read_file,
       "Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.",
       "Lit un fichier texte entier : `Ok(texte)`, ou `Err(message)` s'il ne peut pas être lu.",
       "match read_file(\"notes.txt\")\n  Ok(text) -> print(text)\n  Err(e) -> print(\"Error: {e}\")\nend"),
    b!("write_file", "write_file(path, text)", 2, 2, Files, write_file,
       "Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.",
       "Écrit un texte dans un fichier (en remplaçant son contenu) : `Ok(())` ou `Err(message)`.",
       "write_file(\"out.txt\", \"hello\")"),
    b!("file_exists", "file_exists(path)", 1, 1, Files, file_exists, "Does this file exist?", "Ce fichier existe-t-il ?", "file_exists(\"notes.txt\")"),
];

// ----- argument helpers --------------------------------------------------

fn usage(f: &str) -> String {
    match lookup(f) {
        Some(b) => t!("usage: `{}`", "utilisation : `{}`", b.sig),
        None => String::new(),
    }
}

fn wrong(f: &str, i: usize, expected: String, got: &Value, span: Span) -> Box<Diagnostic> {
    let pos = i + 1;
    Diagnostic::error(
        "E005",
        t!(
            "`{f}` expects {expected} as argument {pos}, but got {}",
            "`{f}` attend {expected} comme argument {pos}, mais a reçu {}",
            got.describe()
        ),
    )
    .at(span)
    .label(t!("wrong kind of value", "mauvaise sorte de valeur"))
    .help(usage(f))
    .boxed()
}

fn a_list() -> String {
    t!("a list", "une liste")
}
fn a_text() -> String {
    t!("a text", "un texte")
}
fn an_int() -> String {
    t!("a whole number (Int)", "un nombre entier (Int)")
}
fn a_number() -> String {
    t!("a number", "un nombre")
}
fn a_function() -> String {
    t!("a function", "une fonction")
}
fn a_record() -> String {
    t!("a record", "un enregistrement")
}

fn list<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<&'a List> {
    match &a[i] {
        Value::List(l) => Ok(l),
        v => Err(wrong(f, i, a_list(), v, span)),
    }
}

fn text<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<&'a str> {
    match &a[i] {
        Value::Text(s) => Ok(s),
        v => Err(wrong(f, i, a_text(), v, span)),
    }
}

fn int(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<i64> {
    match &a[i] {
        Value::Int(n) => Ok(*n),
        v => Err(wrong(f, i, an_int(), v, span)),
    }
}

fn number(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<f64> {
    match &a[i] {
        Value::Int(n) => Ok(*n as f64),
        Value::Float(x) => Ok(*x),
        v => Err(wrong(f, i, a_number(), v, span)),
    }
}

fn func(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Value> {
    if a[i].is_callable() {
        Ok(a[i].clone())
    } else {
        Err(wrong(f, i, a_function(), &a[i], span))
    }
}

fn record<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<&'a Rc<Record>> {
    match &a[i] {
        Value::Record(r) => Ok(r),
        v => Err(wrong(f, i, a_record(), v, span)),
    }
}

fn count_arg(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<usize> {
    let n = int(a, i, f, span)?;
    usize::try_from(n)
        .map_err(|_| invalid(t!("`{f}` needs a positive number, not {n}", "`{f}` a besoin d'un nombre positif, pas {n}"), span))
}

fn invalid(msg: String, span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E023", msg).at(span).boxed()
}

/// Calls a function that must answer true or false.
fn test(it: &mut Interp, f: &Value, x: &Value, name: &str, span: Span) -> Fallible<bool> {
    match it.call1(f, x.clone(), span)? {
        Value::Bool(b) => Ok(b),
        other => Err(Diagnostic::error(
            "E005",
            t!(
                "the function given to `{name}` must return true or false, but it returned {}",
                "la fonction donnée à `{name}` doit renvoyer true ou false, mais elle a renvoyé {}",
                other.describe()
            ),
        )
        .at(span)
        .help(usage(name))
        .boxed()),
    }
}

fn out(s: &str) {
    crate::io::out_line(s);
}

fn text_value(s: impl Into<Rc<str>>) -> Value {
    Value::Text(s.into())
}

fn f64_to_int(x: f64, span: Span) -> Fallible<Value> {
    // i64 covers [-2^63, 2^63); NaN and infinities are outside of that range too.
    if !(-9.223_372_036_854_776e18..9.223_372_036_854_776e18).contains(&x) {
        return Err(overflow(span));
    }
    Ok(Value::Int(x as i64))
}

// ----- Basics ------------------------------------------------------------

fn print(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    out(&joined(a, " "));
    Ok(Value::Unit)
}

/// The values as `display` shows them, with `sep` between them.
fn joined(values: &[Value], sep: &str) -> String {
    let mut s = String::new();
    for (i, v) in values.iter().enumerate() {
        if i > 0 {
            s.push_str(sep);
        }
        v.write_display(&mut s);
    }
    s
}

fn debug(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    crate::io::err(&format!("[debug {}] {}\n", source::location(span), a[0].repr()));
    Ok(a[0].clone())
}

fn to_text(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    Ok(match &a[0] {
        Value::Text(_) => a[0].clone(),
        v => text_value(v.display()),
    })
}

fn input(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    if let Some(p) = a.first() {
        crate::io::out(&p.display());
    }
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    Ok(text_value(line.trim_end_matches(['\n', '\r'])))
}

fn type_of(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    Ok(text_value(a[0].type_name()))
}

fn help(it: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    out(crate::help::help_for_value(it, a.first()).trim_end());
    Ok(Value::Unit)
}

fn assert(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Bool(true) => Ok(Value::Unit),
        Value::Bool(false) => {
            let msg = match a.get(1) {
                Some(m) => t!("assertion failed: {}", "assertion échouée : {}", m.display()),
                None => t!("assertion failed", "assertion échouée"),
            };
            Err(Diagnostic::error("E015", msg).at(span).label(t!("this condition is false", "cette condition est fausse")).boxed())
        }
        v => Err(wrong("assert", 0, t!("true or false", "true ou false"), v, span)),
    }
}

fn assert_eq(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if a[0].equals(&a[1]) {
        return Ok(Value::Unit);
    }
    Err(Diagnostic::error("E015", t!("assertion failed: the values are different", "assertion échouée : les valeurs sont différentes"))
        .at(span)
        .note(t!("got:      {}\nexpected: {}", "obtenu :  {}\nattendu : {}", a[0].repr(), a[1].repr()))
        .boxed())
}

fn fail(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Err(Diagnostic::error("E021", a[0].display()).at(span).label(t!("the program stopped here", "le programme s'est arrêté ici")).boxed())
}

fn args(it: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
    Ok(Value::list(it.script_args.iter().map(|s| text_value(s.as_str())).collect()))
}

fn exit(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let code = if a.is_empty() { 0 } else { int(a, 0, "exit", span)? };
    if !(0..=255).contains(&code) {
        return Err(invalid(t!("an exit code goes from 0 to 255, not {code}", "un code de sortie va de 0 à 255, pas {code}"), span));
    }
    if cfg!(target_arch = "wasm32") {
        return Err(Diagnostic::error("E021", t!("the program stopped with exit({code})", "le programme s'est arrêté avec exit({code})"))
            .at(span)
            .boxed());
    }
    let _ = std::io::stdout().flush();
    std::process::exit(code as i32);
}

fn time(_: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
    Ok(Value::Float(crate::io::now()))
}

fn random(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match a.len() {
        0 => Ok(Value::Float((it.random_u64() >> 11) as f64 / (1u64 << 53) as f64)),
        1 => {
            let l = list(a, 0, "random", span)?;
            if l.is_empty() {
                return Err(invalid(
                    t!("cannot pick a random element in an empty list", "impossible de choisir un élément au hasard dans une liste vide"),
                    span,
                ));
            }
            let i = (it.random_u64() % l.len() as u64) as usize;
            Ok(l.as_slice()[i].clone())
        }
        _ => {
            let (lo, hi) = (int(a, 0, "random", span)?, int(a, 1, "random", span)?);
            if hi < lo {
                return Err(invalid(
                    t!(
                        "random({lo}, {hi}): the minimum is larger than the maximum",
                        "random({lo}, {hi}) : le minimum est plus grand que le maximum"
                    ),
                    span,
                ));
            }
            let size = (hi as i128 - lo as i128 + 1) as u128;
            let r = (it.random_u64() as u128) % size;
            Ok(Value::Int((lo as i128 + r as i128) as i64))
        }
    }
}

// ----- Numbers -----------------------------------------------------------

fn abs(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Int(n) => n.checked_abs().map(Value::Int).ok_or_else(|| overflow(span)),
        Value::Float(x) => Ok(Value::Float(x.abs())),
        v => Err(wrong("abs", 0, a_number(), v, span)),
    }
}

fn extreme(a: &[Value], name: &str, want: Ordering, span: Span) -> Fallible<Value> {
    let items: Vec<Value> = if a.len() == 1 {
        match &a[0] {
            Value::List(l) => l.iter().cloned().collect(),
            v => return Err(wrong(name, 0, t!("a list (or several values)", "une liste (ou plusieurs valeurs)"), v, span)),
        }
    } else {
        a.to_vec()
    };
    let mut it = items.into_iter();
    let Some(mut best) = it.next() else {
        return Err(invalid(t!("`{name}` of an empty list", "`{name}` d'une liste vide"), span));
    };
    for v in it {
        match v.compare(&best) {
            Some(o) if o == want => best = v,
            Some(_) => {}
            None => {
                return Err(Diagnostic::error(
                    "E005",
                    t!("`{name}` cannot compare {} and {}", "`{name}` ne peut pas comparer {} et {}", v.describe(), best.describe()),
                )
                .at(span)
                .boxed())
            }
        }
    }
    Ok(best)
}

fn min(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    extreme(a, "min", Ordering::Less, span)
}

fn max(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    extreme(a, "max", Ordering::Greater, span)
}

fn sqrt(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = number(a, 0, "sqrt", span)?;
    if x < 0.0 {
        return Err(Diagnostic::error(
            "E023",
            t!(
                "cannot take the square root of a negative number ({x})",
                "impossible de prendre la racine carrée d'un nombre négatif ({x})"
            ),
        )
        .at(span)
        .help(t!(
            "use `sqrt(abs(x))` if you want the root of its absolute value",
            "utilisez `sqrt(abs(x))` pour la racine de sa valeur absolue"
        ))
        .boxed());
    }
    Ok(Value::Float(x.sqrt()))
}

fn pow(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    binop(crate::ast::BinOp::Pow, &a[0], &a[1], span)
}

fn rounding(a: &[Value], name: &str, span: Span, f: fn(f64) -> f64) -> Fallible<Value> {
    match &a[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(x) => f64_to_int(f(*x), span),
        v => Err(wrong(name, 0, a_number(), v, span)),
    }
}

fn floor(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    rounding(a, "floor", span, f64::floor)
}

fn ceil(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    rounding(a, "ceil", span, f64::ceil)
}

fn round(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if a.len() == 2 {
        let x = number(a, 0, "round", span)?;
        let d = int(a, 1, "round", span)?.clamp(-15, 15);
        let p = 10f64.powi(d as i32);
        return Ok(Value::Float((x * p).round() / p));
    }
    rounding(a, "round", span, f64::round)
}

fn to_int(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(x) => f64_to_int(x.trunc(), span),
        Value::Bool(b) => Ok(Value::Int(*b as i64)),
        Value::Text(s) => {
            let t = s.trim();
            if let Ok(n) = t.replace('_', "").parse::<i64>() {
                return Ok(Value::Int(n));
            }
            if let Ok(x) = t.replace('_', "").parse::<f64>() {
                return f64_to_int(x.trunc(), span);
            }
            Err(Diagnostic::error(
                "E022",
                t!("cannot convert \"{s}\" to a whole number", "impossible de convertir \"{s}\" en nombre entier"),
            )
            .at(span)
            .help(t!(
                "if the text may not be a number, use `parse_int(t)`: it gives `Ok(n)` or `Err(...)` that you can check",
                "si le texte peut ne pas être un nombre, utilisez `parse_int(t)` : il donne `Ok(n)` ou `Err(...)` que vous pouvez vérifier"
            ))
            .boxed())
        }
        v => Err(wrong("int", 0, t!("a number or a text", "un nombre ou un texte"), v, span)),
    }
}

fn to_float(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Int(n) => Ok(Value::Float(*n as f64)),
        Value::Float(x) => Ok(Value::Float(*x)),
        Value::Text(s) => s.trim().replace('_', "").parse::<f64>().map(Value::Float).map_err(|_| {
            Diagnostic::error("E022", t!("cannot convert \"{s}\" to a decimal number", "impossible de convertir \"{s}\" en nombre décimal"))
                .at(span)
                .help(t!(
                    "if the text may not be a number, use `parse_float(t)`, which gives `Ok(x)` or `Err(...)`",
                    "si le texte peut ne pas être un nombre, utilisez `parse_float(t)`, qui donne `Ok(x)` ou `Err(...)`"
                ))
                .boxed()
        }),
        v => Err(wrong("float", 0, t!("a number or a text", "un nombre ou un texte"), v, span)),
    }
}

fn sin(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "sin", span)?.sin()))
}

fn cos(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "cos", span)?.cos()))
}

fn tan(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "tan", span)?.tan()))
}

fn exp(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "exp", span)?.exp()))
}

fn log(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = number(a, 0, "log", span)?;
    if x <= 0.0 {
        return Err(invalid(
            t!("the logarithm only exists for positive numbers, not {x}", "le logarithme n'existe que pour les nombres positifs, pas {x}"),
            span,
        ));
    }
    if a.len() == 2 {
        let base = number(a, 1, "log", span)?;
        if base <= 0.0 || base == 1.0 {
            return Err(invalid(t!("invalid logarithm base: {base}", "base de logarithme invalide : {base}"), span));
        }
        // log10 and log2 are exact for powers of their base.
        let r = if base == 10.0 {
            x.log10()
        } else if base == 2.0 {
            x.log2()
        } else {
            x.ln() / base.ln()
        };
        return Ok(Value::Float(r));
    }
    Ok(Value::Float(x.ln()))
}

fn is_even(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(int(a, 0, "is_even", span)? % 2 == 0))
}

fn is_odd(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(int(a, 0, "is_odd", span)? % 2 != 0))
}

// ----- Text --------------------------------------------------------------

fn len(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Int(match &a[0] {
        Value::Text(s) => s.chars().count() as i64,
        Value::List(l) => l.len() as i64,
        Value::Record(r) => r.len() as i64,
        v => return Err(wrong("len", 0, t!("a text, a list or a record", "un texte, une liste ou un enregistrement"), v, span)),
    }))
}

fn upper(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "upper", span)?.to_uppercase()))
}

fn lower(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "lower", span)?.to_lowercase()))
}

fn trim(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "trim", span)?.trim()))
}

fn split(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "split", span)?;
    let parts: Vec<Value> = if a.len() == 1 {
        s.split_whitespace().map(text_value).collect()
    } else {
        let sep = text(a, 1, "split", span)?;
        match single_char(sep) {
            _ if sep.is_empty() => s.chars().map(|c| text_value(c.to_string())).collect(),
            // Same result, found much faster.
            Some(c) => s.split(c).map(text_value).collect(),
            None => s.split(sep).map(text_value).collect(),
        }
    };
    Ok(Value::list(parts))
}

fn join(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "join", span)?;
    let sep = if a.len() == 2 { text(a, 1, "join", span)? } else { "" };
    Ok(text_value(joined(l.as_slice(), sep)))
}

fn contains(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(match (&a[0], &a[1]) {
        (Value::Text(s), Value::Text(sub)) => s.contains(&**sub),
        (Value::Text(_), v) => return Err(wrong("contains", 1, a_text(), v, span)),
        (Value::List(l), x) => l.iter().any(|v| v.equals(x)),
        (Value::Record(r), Value::Text(k)) => r.get(k).is_some(),
        (Value::Record(_), v) => return Err(wrong("contains", 1, a_text(), v, span)),
        (v, _) => return Err(wrong("contains", 0, t!("a text, a list or a record", "un texte, une liste ou un enregistrement"), v, span)),
    }))
}

fn starts_with(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(text(a, 0, "starts_with", span)?.starts_with(text(a, 1, "starts_with", span)?)))
}

fn ends_with(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(text(a, 0, "ends_with", span)?.ends_with(text(a, 1, "ends_with", span)?)))
}

fn replace(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "replace", span)?;
    let old = text(a, 1, "replace", span)?;
    let new = text(a, 2, "replace", span)?;
    if old.is_empty() {
        return Err(invalid(t!("`replace` cannot replace an empty text", "`replace` ne peut pas remplacer un texte vide"), span));
    }
    Ok(text_value(match single_char(old) {
        Some(c) => s.replace(c, new),
        None => s.replace(old, new),
    }))
}

/// The character, if the text is one character: searching for a character is
/// much faster than searching for a text.
fn single_char(s: &str) -> Option<char> {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

fn chars(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::list(text(a, 0, "chars", span)?.chars().map(|c| text_value(c.to_string())).collect()))
}

fn lines(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::list(text(a, 0, "lines", span)?.lines().map(text_value).collect()))
}

fn repeat(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = count_arg(a, 1, "repeat", span)?;
    match &a[0] {
        Value::Text(s) => {
            if s.len().saturating_mul(n) > 100_000_000 {
                return Err(invalid(t!("the repeated text would be too large", "le texte répété serait trop grand"), span));
            }
            Ok(text_value(s.repeat(n)))
        }
        Value::List(l) => {
            if l.is_empty() {
                return Ok(Value::list(Vec::new()));
            }
            if l.len().saturating_mul(n) > 50_000_000 {
                return Err(invalid(t!("the repeated list would be too large", "la liste répétée serait trop grande"), span));
            }
            let mut v = Vec::with_capacity(l.len() * n);
            for _ in 0..n {
                v.extend(l.iter().cloned());
            }
            Ok(Value::list(v))
        }
        v => Err(wrong("repeat", 0, t!("a text or a list", "un texte ou une liste"), v, span)),
    }
}

fn parse_int(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_int", span)?;
    Ok(match s.trim().replace('_', "").parse::<i64>() {
        Ok(n) => it.ok(Value::Int(n)),
        Err(_) => it.err(text_value(t!("\"{s}\" is not a whole number", "\"{s}\" n'est pas un nombre entier"))),
    })
}

fn parse_float(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_float", span)?;
    Ok(match s.trim().replace('_', "").parse::<f64>() {
        Ok(x) => it.ok(Value::Float(x)),
        Err(_) => it.err(text_value(t!("\"{s}\" is not a number", "\"{s}\" n'est pas un nombre"))),
    })
}

// ----- Lists -------------------------------------------------------------

fn map(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = func(a, 1, "map", span)?;
    match &a[0] {
        Value::List(l) => {
            let mut out = Vec::with_capacity(l.len());
            for x in l.iter() {
                out.push(it.call1(&f, x.clone(), span)?);
            }
            Ok(Value::list(out))
        }
        v => match it.wrap(v) {
            Wrap::Some => {
                let r = it.call(f, vec![Interp::payload(v)], span)?;
                Ok(it.some(r))
            }
            Wrap::Ok => {
                let r = it.call(f, vec![Interp::payload(v)], span)?;
                Ok(it.ok(r))
            }
            Wrap::None | Wrap::Err => Ok(v.clone()),
            Wrap::Other => Err(wrong("map", 0, t!("a list, an Option or a Result", "une liste, une Option ou un Result"), v, span)),
        },
    }
}

fn filter(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "filter", span)?;
    let f = func(a, 1, "filter", span)?;
    let mut out = Vec::new();
    for x in l.iter() {
        if test(it, &f, x, "filter", span)? {
            out.push(x.clone());
        }
    }
    Ok(Value::list(out))
}

fn fold(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "fold", span)?;
    let f = func(a, 2, "fold", span)?;
    let mut acc = a[1].clone();
    for x in l.iter() {
        acc = it.call2(&f, acc, x.clone(), span)?;
    }
    Ok(acc)
}

fn each(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "each", span)?;
    let f = func(a, 1, "each", span)?;
    for x in l.iter() {
        it.call1(&f, x.clone(), span)?;
    }
    Ok(Value::Unit)
}

fn sum(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "sum", span)?;
    let mut acc = Value::Int(0);
    for x in l.iter() {
        if !matches!(x, Value::Int(_) | Value::Float(_)) {
            return Err(invalid(
                t!(
                    "`sum` needs a list of numbers, but it contains {}",
                    "`sum` a besoin d'une liste de nombres, mais elle contient {}",
                    x.describe()
                ),
                span,
            ));
        }
        if let (Value::Int(total), Value::Int(n)) = (&acc, x) {
            if let Some(t) = total.checked_add(*n) {
                acc = Value::Int(t);
                continue;
            }
        }
        acc = binop(crate::ast::BinOp::Add, &acc, x, span)?;
    }
    Ok(acc)
}

fn product(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "product", span)?;
    let mut acc = Value::Int(1);
    for x in l.iter() {
        if !matches!(x, Value::Int(_) | Value::Float(_)) {
            return Err(invalid(
                t!(
                    "`product` needs a list of numbers, but it contains {}",
                    "`product` a besoin d'une liste de nombres, mais elle contient {}",
                    x.describe()
                ),
                span,
            ));
        }
        if let (Value::Int(total), Value::Int(n)) = (&acc, x) {
            if let Some(t) = total.checked_mul(*n) {
                acc = Value::Int(t);
                continue;
            }
        }
        acc = binop(crate::ast::BinOp::Mul, &acc, x, span)?;
    }
    Ok(acc)
}

fn first(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match &a[0] {
        Value::List(l) => it.option(l.get(0).cloned()),
        Value::Text(s) => it.option(s.chars().next().map(|c| text_value(c.to_string()))),
        v => return Err(wrong("first", 0, t!("a list or a text", "une liste ou un texte"), v, span)),
    })
}

fn last(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match &a[0] {
        Value::List(l) => it.option(l.as_slice().last().cloned()),
        Value::Text(s) => it.option(s.chars().last().map(|c| text_value(c.to_string()))),
        v => return Err(wrong("last", 0, t!("a list or a text", "une liste ou un texte"), v, span)),
    })
}

fn rest(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match &a[0] {
        Value::List(l) => Value::List(l.slice(1, l.len())),
        Value::Text(s) => text_value(s.chars().skip(1).collect::<String>()),
        v => return Err(wrong("rest", 0, t!("a list or a text", "une liste ou un texte"), v, span)),
    })
}

fn get(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match (&a[0], &a[1]) {
        (Value::List(l), Value::Int(n)) => it.option(resolve_index(*n, l.len()).map(|i| l.as_slice()[i].clone())),
        (Value::Text(s), Value::Int(n)) => {
            let len = s.chars().count();
            it.option(resolve_index(*n, len).and_then(|i| s.chars().nth(i)).map(|c| text_value(c.to_string())))
        }
        (Value::Record(r), Value::Text(k)) => it.option(r.get(k).cloned()),
        (Value::List(_) | Value::Text(_), v) => return Err(wrong("get", 1, an_int(), v, span)),
        (Value::Record(_), v) => return Err(wrong("get", 1, a_text(), v, span)),
        (v, _) => return Err(wrong("get", 0, t!("a list, a text or a record", "une liste, un texte ou un enregistrement"), v, span)),
    })
}

fn push(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "push", span)?;
    let mut v = Vec::with_capacity(l.len() + 1);
    v.extend(l.iter().cloned());
    v.push(a[1].clone());
    Ok(Value::list(v))
}

fn reverse(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match &a[0] {
        Value::List(l) => Value::list(l.iter().rev().cloned().collect()),
        Value::Text(s) => text_value(s.chars().rev().collect::<String>()),
        v => return Err(wrong("reverse", 0, t!("a list or a text", "une liste ou un texte"), v, span)),
    })
}

fn sort_pairs(pairs: &mut [(Value, Value)], name: &str, span: Span) -> Fallible<()> {
    // Sort with a total order (it never fails), then check that neighbours
    // can really be compared: mixed kinds or NaN end up side by side.
    pairs.sort_by(|x, y| match (&x.0, &y.0) {
        (Value::Int(a), Value::Int(b)) => a.cmp(b),
        (a, b) => a.total_cmp(b),
    });
    let bad = pairs.windows(2).find(|w| w[0].0.compare(&w[1].0).is_none()).map(|w| (w[0].0.clone(), w[1].0.clone()));
    if let Some((x, y)) = bad {
        return Err(Diagnostic::error(
            "E005",
            t!("`{name}` cannot compare {} and {}", "`{name}` ne peut pas comparer {} et {}", x.describe(), y.describe()),
        )
        .at(span)
        .help(t!(
            "all the elements must be of the same kind: all numbers, or all texts...",
            "tous les éléments doivent être de même nature : tous des nombres, ou tous des textes..."
        ))
        .boxed());
    }
    Ok(())
}

fn sort(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "sort", span)?;
    // Whole numbers alone: sorted as such, much faster (equal ones cannot be told apart).
    if l.iter().all(|v| matches!(v, Value::Int(_))) {
        let mut ints: Vec<i64> = l.iter().map(|v| if let Value::Int(n) = v { *n } else { 0 }).collect();
        ints.sort_unstable();
        return Ok(Value::list(ints.into_iter().map(Value::Int).collect()));
    }
    let mut pairs: Vec<(Value, Value)> = l.iter().map(|v| (v.clone(), Value::Unit)).collect();
    sort_pairs(&mut pairs, "sort", span)?;
    Ok(Value::list(pairs.into_iter().map(|p| p.0).collect()))
}

fn sort_by(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "sort_by", span)?;
    let f = func(a, 1, "sort_by", span)?;
    let mut pairs = Vec::with_capacity(l.len());
    for x in l.iter() {
        pairs.push((it.call1(&f, x.clone(), span)?, x.clone()));
    }
    sort_pairs(&mut pairs, "sort_by", span)?;
    Ok(Value::list(pairs.into_iter().map(|p| p.1).collect()))
}

fn unique(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    // Same meaning as `==`. Numbers and texts are found quickly with a set;
    // other values are compared with the ones kept so far.
    #[derive(PartialEq, Eq, Hash)]
    enum Key {
        Int(i64),
        Float(u64),
        Text(Rc<str>),
        Bool(bool),
        Unit,
    }
    fn key(v: &Value) -> Option<Key> {
        Some(match v {
            Value::Int(n) => Key::Int(*n),
            Value::Float(f) if f.is_nan() => return None,
            Value::Float(f) if f.fract() == 0.0 && f.abs() < 9.0e18 => Key::Int(*f as i64),
            Value::Float(f) => Key::Float(f.to_bits()),
            Value::Text(s) => Key::Text(s.clone()),
            Value::Bool(b) => Key::Bool(*b),
            Value::Unit => Key::Unit,
            _ => return None,
        })
    }
    let l = list(a, 0, "unique", span)?;
    let mut seen = HashSet::new();
    let mut others: Vec<&Value> = Vec::new();
    let mut out = Vec::new();
    for x in l.iter() {
        let new = match key(x) {
            Some(k) => seen.insert(k),
            None => {
                let new = !others.iter().any(|o| o.equals(x));
                if new {
                    others.push(x);
                }
                new
            }
        };
        if new {
            out.push(x.clone());
        }
    }
    Ok(Value::list(out))
}

/// Start and end positions for `slice`, `take` and `drop` (negative = from the end).
fn bounds(len: usize, start: i64, end: Option<i64>) -> (usize, usize) {
    let fix = |n: i64| -> usize {
        let n = if n < 0 { len as i64 + n } else { n };
        n.clamp(0, len as i64) as usize
    };
    let s = fix(start);
    let e = end.map(fix).unwrap_or(len);
    (s, e.max(s))
}

fn cut(v: &Value, from: usize, to: usize) -> Value {
    match v {
        Value::List(l) => Value::List(l.slice(from, to)),
        Value::Text(s) => text_value(s.chars().skip(from).take(to - from).collect::<String>()),
        _ => Value::Unit,
    }
}

fn length_of(v: &Value, name: &str, span: Span) -> Fallible<usize> {
    match v {
        Value::List(l) => Ok(l.len()),
        Value::Text(s) => Ok(s.chars().count()),
        v => Err(wrong(name, 0, t!("a list or a text", "une liste ou un texte"), v, span)),
    }
}

fn slice(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let len = length_of(&a[0], "slice", span)?;
    let start = int(a, 1, "slice", span)?;
    let end = if a.len() == 3 { Some(int(a, 2, "slice", span)?) } else { None };
    let (s, e) = bounds(len, start, end);
    Ok(cut(&a[0], s, e))
}

fn take(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let len = length_of(&a[0], "take", span)?;
    let n = count_arg(a, 1, "take", span)?;
    Ok(cut(&a[0], 0, n.min(len)))
}

fn drop(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let len = length_of(&a[0], "drop", span)?;
    let n = count_arg(a, 1, "drop", span)?;
    Ok(cut(&a[0], n.min(len), len))
}

fn zip(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = list(a, 0, "zip", span)?;
    let y = list(a, 1, "zip", span)?;
    Ok(Value::list(x.iter().zip(y.iter()).map(|(p, q)| Value::list(vec![p.clone(), q.clone()])).collect()))
}

fn enumerate(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "enumerate", span)?;
    Ok(Value::list(l.iter().enumerate().map(|(i, v)| Value::list(vec![Value::Int(i as i64), v.clone()])).collect()))
}

fn any(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "any", span)?;
    let f = func(a, 1, "any", span)?;
    for x in l.iter() {
        if test(it, &f, x, "any", span)? {
            return Ok(Value::Bool(true));
        }
    }
    Ok(Value::Bool(false))
}

fn all(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "all", span)?;
    let f = func(a, 1, "all", span)?;
    for x in l.iter() {
        if !test(it, &f, x, "all", span)? {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn find(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "find", span)?;
    let f = func(a, 1, "find", span)?;
    for x in l.iter() {
        if test(it, &f, x, "find", span)? {
            return Ok(it.some(x.clone()));
        }
    }
    Ok(it.none())
}

fn count(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "count", span)?;
    let f = func(a, 1, "count", span)?;
    let mut n = 0;
    for x in l.iter() {
        if test(it, &f, x, "count", span)? {
            n += 1;
        }
    }
    Ok(Value::Int(n))
}

fn index_of(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match (&a[0], &a[1]) {
        (Value::List(l), x) => it.option(l.iter().position(|v| v.equals(x)).map(|i| Value::Int(i as i64))),
        (Value::Text(s), Value::Text(sub)) => it.option(s.find(&**sub).map(|byte| Value::Int(s[..byte].chars().count() as i64))),
        (Value::Text(_), v) => return Err(wrong("index_of", 1, a_text(), v, span)),
        (v, _) => return Err(wrong("index_of", 0, t!("a list or a text", "une liste ou un texte"), v, span)),
    })
}

fn flatten(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "flatten", span)?;
    let mut out = Vec::new();
    for x in l.iter() {
        match x {
            Value::List(inner) => out.extend(inner.iter().cloned()),
            other => {
                return Err(invalid(
                    t!(
                        "`flatten` needs a list of lists, but it contains {}",
                        "`flatten` a besoin d'une liste de listes, mais elle contient {}",
                        other.describe()
                    ),
                    span,
                ))
            }
        }
    }
    Ok(Value::list(out))
}

fn flat_map(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "flat_map", span)?;
    let f = func(a, 1, "flat_map", span)?;
    let mut out = Vec::new();
    for x in l.iter() {
        match it.call1(&f, x.clone(), span)? {
            Value::List(inner) => out.extend(inner.iter().cloned()),
            other => {
                return Err(invalid(
                    t!(
                        "the function given to `flat_map` must return a list, but it returned {}",
                        "la fonction donnée à `flat_map` doit renvoyer une liste, mais elle a renvoyé {}",
                        other.describe()
                    ),
                    span,
                ))
            }
        }
    }
    Ok(Value::list(out))
}

fn partition(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "partition", span)?;
    let f = func(a, 1, "partition", span)?;
    let (mut yes, mut no) = (Vec::new(), Vec::new());
    for x in l.iter() {
        if test(it, &f, x, "partition", span)? {
            yes.push(x.clone());
        } else {
            no.push(x.clone());
        }
    }
    Ok(Value::list(vec![Value::list(yes), Value::list(no)]))
}

fn group_by(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "group_by", span)?;
    let f = func(a, 1, "group_by", span)?;
    let mut keys: Vec<Rc<str>> = Vec::new();
    let mut groups: std::collections::HashMap<Rc<str>, Vec<Value>> = std::collections::HashMap::new();
    for x in l.iter() {
        let k: Rc<str> = Rc::from(it.call1(&f, x.clone(), span)?.display());
        groups.entry(k.clone()).or_insert_with(|| {
            keys.push(k.clone());
            Vec::new()
        });
        groups.get_mut(&k).unwrap().push(x.clone());
    }
    let mut r = Record::new();
    for k in keys {
        let g = groups.remove(&k).unwrap_or_default();
        r.set(k, Value::list(g));
    }
    Ok(Value::Record(Rc::new(r)))
}

fn is_empty(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(match &a[0] {
        Value::Text(s) => s.is_empty(),
        Value::List(l) => l.is_empty(),
        Value::Record(r) => r.is_empty(),
        v => return Err(wrong("is_empty", 0, t!("a text, a list or a record", "un texte, une liste ou un enregistrement"), v, span)),
    }))
}

// ----- Records -----------------------------------------------------------

fn keys(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = record(a, 0, "keys", span)?;
    Ok(Value::list(r.keys().iter().map(|k| Value::Text(k.clone())).collect()))
}

fn values(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = record(a, 0, "values", span)?;
    Ok(Value::list(r.iter().map(|(_, v)| v.clone()).collect()))
}

fn entries(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = record(a, 0, "entries", span)?;
    Ok(Value::list(r.iter().map(|(k, v)| Value::list(vec![Value::Text(k.clone()), v.clone()])).collect()))
}

fn has(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = record(a, 0, "has", span)?;
    Ok(Value::Bool(r.get(text(a, 1, "has", span)?).is_some()))
}

fn put(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = record(a, 0, "put", span)?;
    let k = text(a, 1, "put", span)?;
    let mut new = (**r).clone();
    new.set(Rc::from(k), a[2].clone());
    Ok(Value::Record(Rc::new(new)))
}

fn remove(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = record(a, 0, "remove", span)?;
    let k = text(a, 1, "remove", span)?;
    Ok(Value::Record(Rc::new(r.without(k))))
}

// ----- Option and Result -------------------------------------------------

fn option_or_result(it: &Interp, v: &Value, name: &str, span: Span) -> Fallible<Wrap> {
    match it.wrap(v) {
        Wrap::Other => Err(wrong(name, 0, t!("an Option or a Result", "une Option ou un Result"), v, span)),
        w => Ok(w),
    }
}

fn unwrap(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match option_or_result(it, &a[0], "unwrap", span)? {
        Wrap::Some | Wrap::Ok => Ok(Interp::payload(&a[0])),
        Wrap::None => Err(Diagnostic::error(
            "E013",
            t!("`unwrap` found `None`: there is no value", "`unwrap` a trouvé `None` : il n'y a pas de valeur"),
        )
        .at(span)
        .help(t!(
            "handle the missing value with `match`, or give a default: `unwrap_or(x, default)`",
            "traitez l'absence de valeur avec `match`, ou donnez une valeur par défaut : `unwrap_or(x, defaut)`"
        ))
        .boxed()),
        _ => Err(Diagnostic::error(
            "E013",
            t!("`unwrap` found an error: {}", "`unwrap` a trouvé une erreur : {}", Interp::payload(&a[0]).display()),
        )
        .at(span)
        .help(t!(
            "handle the error with `match`, pass it on with `?`, or give a default: `unwrap_or(x, default)`",
            "traitez l'erreur avec `match`, transmettez-la avec `?`, ou donnez une valeur par défaut : `unwrap_or(x, defaut)`"
        ))
        .boxed()),
    }
}

fn unwrap_or(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match option_or_result(it, &a[0], "unwrap_or", span)? {
        Wrap::Some | Wrap::Ok => Ok(Interp::payload(&a[0])),
        _ => Ok(a[1].clone()),
    }
}

fn is_some(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(option_or_result(it, &a[0], "is_some", span)? == Wrap::Some))
}

fn is_none(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(option_or_result(it, &a[0], "is_none", span)? == Wrap::None))
}

fn is_ok(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(option_or_result(it, &a[0], "is_ok", span)? == Wrap::Ok))
}

fn is_err(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(option_or_result(it, &a[0], "is_err", span)? == Wrap::Err))
}

// ----- Files -------------------------------------------------------------

fn io_message(e: &std::io::Error, path: &str) -> String {
    match e.kind() {
        std::io::ErrorKind::NotFound => t!("file not found: {path}", "fichier introuvable : {path}"),
        std::io::ErrorKind::PermissionDenied => t!("permission denied: {path}", "permission refusée : {path}"),
        _ => t!("cannot access {path}: {e}", "impossible d'accéder à {path} : {e}"),
    }
}

fn read_file(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "read_file", span)?;
    Ok(match std::fs::read_to_string(path) {
        Ok(s) => it.ok(text_value(s)),
        Err(e) => it.err(text_value(io_message(&e, path))),
    })
}

fn write_file(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "write_file", span)?;
    let content = a[1].display();
    Ok(match std::fs::write(path, content) {
        Ok(()) => it.ok(Value::Unit),
        Err(e) => it.err(text_value(io_message(&e, path))),
    })
}

fn file_exists(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(std::path::Path::new(text(a, 0, "file_exists", span)?).exists()))
}
