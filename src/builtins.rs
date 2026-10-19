//! The standard library: functions available everywhere, each with its
//! documentation in English and French and an example.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::Write;
use std::rc::Rc;

use crate::bigint::{self, BigInt};
use crate::diagnostic::{Diagnostic, Fallible};
use crate::interp::{binop, resolve_index, too_big, Interp, Wrap, MAX_LIST, MAX_TEXT};
use crate::source::{self, Span};
use crate::types::{self, Kinds, BOOL, DICT, FUNC, INT, LIST, NUMBER, OPTION, RECORD, RESULT, SET, TEXT};
use crate::value::*;

pub(crate) mod date;
mod dicts;
pub mod property;
pub(crate) mod system;
use date::{
    add_days, add_seconds, day_of_week, days_between, days_in_month, format_date, is_date, is_leap_year, now, parse_date, seconds_between,
    today,
};
use dicts::{dict, difference, intersection, is_subset, set, union};
use property::{bools, check_all, floats, ints, lists_of, one_of, records_of, sample, texts};
use system::{create_folder, delete_file, file_info, in_browser, input_lines, io_message, join_path, list_folder, options, split_path};
pub mod regex;
use regex::{captures, captures_all, find_all, find_first, matches, named_captures, replace_all, split_by};
pub mod csv;
mod text_tools;
use csv::{csv_format, csv_parse, csv_records};
use text_tools::{equals_ignore_case, format_number, trim_end, trim_start, words};
mod ml;
use ml::{
    argmax, argmin, dot, identity_matrix, matmul, norm, normalize, ones, percentile, random_choice_weighted, random_normal, relu, shape,
    sigmoid, softmax, stats, std_dev, tanh, train_test_split, transpose, variance, vector_add, vector_mul, vector_scale, vector_sub, zeros,
    zip_with,
};

pub type BuiltinFn = fn(&mut Interp, &[Value], Span) -> Fallible<Value>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Category {
    Basics,
    Numbers,
    Text,
    Lists,
    Records,
    Dicts,
    Options,
    Files,
    Dates,
    Tests,
    Ml,
}

impl Category {
    pub const ALL: [Category; 11] = [
        Category::Basics,
        Category::Numbers,
        Category::Text,
        Category::Lists,
        Category::Records,
        Category::Dicts,
        Category::Options,
        Category::Files,
        Category::Dates,
        Category::Tests,
        Category::Ml,
    ];

    pub fn title(self) -> String {
        match self {
            Category::Basics => t!("Basics", "Bases"),
            Category::Numbers => t!("Numbers", "Nombres"),
            Category::Text => t!("Text", "Texte"),
            Category::Lists => t!("Lists", "Listes"),
            Category::Records => t!("Records", "Enregistrements"),
            Category::Dicts => t!("Dictionaries and sets", "Dictionnaires et ensembles"),
            Category::Options => t!("Option and Result", "Option et Result"),
            Category::Files => t!("Files", "Fichiers"),
            Category::Dates => t!("Dates and times", "Dates et heures"),
            Category::Tests => t!("Property tests", "Tests de propriétés"),
            Category::Ml => {
                t!("Machine learning: vectors, matrices, statistics", "Apprentissage automatique : vecteurs, matrices, statistiques")
            }
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

/// The built-in function `name`, also under its Ruby name (`upcase` gives `upper`).
pub fn lookup(name: &str) -> Option<&'static Builtin> {
    let name = alias_of(name).unwrap_or(name);
    BUILTINS.iter().find(|b| b.name == name)
}

/// A second name of a built-in function: the name Ruby gives it.
pub struct Alias {
    pub name: &'static str,
    /// The built-in function it names.
    pub of: &'static str,
}

const fn alias(name: &'static str, of: &'static str) -> Alias {
    Alias { name, of }
}

/// The Ruby names of built-in functions: `xs.select(is_even)`,
/// `"hi".upcase`, `n.even?`. Each one is exactly the function it names.
pub static ALIASES: &[Alias] = &[
    alias("puts", "print"),
    alias("to_s", "text"),
    alias("to_i", "int"),
    alias("to_f", "float"),
    alias("even?", "is_even"),
    alias("odd?", "is_odd"),
    alias("length", "len"),
    alias("size", "len"),
    alias("upcase", "upper"),
    alias("downcase", "lower"),
    alias("strip", "trim"),
    alias("include?", "contains"),
    alias("start_with?", "starts_with"),
    alias("end_with?", "ends_with"),
    alias("select", "filter"),
    alias("reduce", "fold"),
    alias("inject", "fold"),
    alias("uniq", "unique"),
    alias("each_slice", "chunks"),
    alias("each_cons", "windows"),
    alias("any?", "any"),
    alias("all?", "all"),
    alias("empty?", "is_empty"),
    alias("key?", "has"),
    alias("lstrip", "trim_start"),
    alias("rstrip", "trim_end"),
    alias("casecmp?", "equals_ignore_case"),
];

/// The function that `name` is a Ruby name of: `upper` for `upcase`.
pub fn alias_of(name: &str) -> Option<&'static str> {
    ALIASES.iter().find(|a| a.name == name).map(|a| a.of)
}

/// The Ruby names of the built-in function `name`.
pub fn aliases(name: &str) -> impl Iterator<Item = &'static str> + '_ {
    ALIASES.iter().filter(move |a| a.of == name).map(|a| a.name)
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
    b!("text", "text(value, decimals)", 1, 2, Basics, to_text,
       "Converts any value to text. With a number of decimals, writes a number with exactly that many digits after the point.",
       "Convertit n'importe quelle valeur en texte. Avec un nombre de décimales, écrit un nombre avec exactement autant de chiffres après la virgule.",
       "text(42) + \"!\"            # \"42!\"\ntext(3.14159, 2)           # \"3.14\"\ntext(5, 2)                 # \"5.00\""),
    b!("input", "input(prompt)", 0, 1, Basics, input,
       "Shows the prompt, then reads a line typed on the keyboard and returns it as text.",
       "Affiche l'invite, puis lit une ligne tapée au clavier et la renvoie sous forme de texte.",
       "name = input(\"Your name? \")"),
    b!("input_lines", "input_lines()", 0, 0, Basics, input_lines,
       "Reads all the lines of the standard input, until its end: `Ok(list of texts)`, or `Err(message)` if it cannot be read. For `rythme count.ry < names.txt`, or text typed until Ctrl-D (Ctrl-Z then Enter on Windows).",
       "Lit toutes les lignes de l'entrée standard, jusqu'à sa fin : `Ok(liste de textes)`, ou `Err(message)` si elle ne peut pas être lue. Pour `rythme compter.ry < noms.txt`, ou du texte tapé jusqu'à Ctrl-D (Ctrl-Z puis Entrée sous Windows).",
       "match input_lines()\n  Ok(lines) -> print(len(lines), \"lines\")\n  Err(e) -> print(\"Error: {e}\")\nend"),
    b!("type_of", "type_of(value)", 1, 1, Basics, type_of,
       "Returns the name of the type of a value: \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (for `()`), \"Symbol\" (for `:ok`), \"Option\", \"Result\", or the name of your own type.",
       "Renvoie le nom du type d'une valeur : \"Int\", \"Float\", \"Text\", \"Bool\", \"List\", \"Record\", \"Function\", \"Unit\" (pour `()`), \"Symbol\" (pour `:ok`), \"Option\", \"Result\", ou le nom de votre propre type.",
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
    b!("options", "options(words)", 0, 1, Basics, options,
       "Reads the options of the command line (the words of `args()`, or the list given) into a record, in a Result: `--name=value` gives the text `value`, `--name` and `-n` give `true`, a `-` in a name becomes `_`, and the other words are in the field `args` (all of them after `--`). An option given twice, or a word like `-abc`, gives `Err(message)`. Use `get(opts, \"name\")` or `has(opts, \"name\")` for an option that may be missing.",
       "Lit les options de la ligne de commande (les mots de `args()`, ou la liste donnée) dans un enregistrement, dans un Result : `--nom=valeur` donne le texte `valeur`, `--nom` et `-n` donnent `true`, un `-` dans un nom devient `_`, et les autres mots sont dans le champ `args` (tous ceux après `--`). Une option donnée deux fois, ou un mot comme `-abc`, donne `Err(message)`. Utilisez `get(opts, \"nom\")` ou `has(opts, \"nom\")` pour une option qui peut manquer.",
       "options([\"--dry-run\", \"--name=Ada\", \"notes.txt\"])   # Ok({args: [\"notes.txt\"], dry_run: true, name: \"Ada\"})\noptions([\"-v\", \"--\", \"-v\"])   # Ok({args: [\"-v\"], v: true})\noptions([\"--n=1\", \"--n=2\"])   # Err(\"the option `n` is given twice\")"),
    b!("env", "env(name)", 1, 1, Basics, env,
       "The value of an environment variable of the system, as an Option: `Some(text)`, or `None` if it is not set.",
       "La valeur d'une variable d'environnement du système, sous forme d'Option : `Some(texte)`, ou `None` si elle n'est pas définie.",
       "env(\"HOME\")                # Some(\"/home/ada\")"),
    b!("exit", "exit(code)", 0, 1, Basics, exit,
       "Stops the program immediately. The code (0 by default) tells the system if everything went well (0) or not; Rythme itself exits with 1 after an error in the program (see `rythme help system`).",
       "Arrête immédiatement le programme. Le code (0 par défaut) indique au système si tout s'est bien passé (0) ou non ; Rythme lui-même sort avec 1 après une erreur dans le programme (voir `rythme help system`).",
       "exit(1)"),
    b!("time", "time()", 0, 0, Basics, time,
       "Returns the current time in seconds (a Float), counted from 1 January 1970. Subtract two times to measure a duration.",
       "Renvoie l'heure actuelle en secondes (un Float), comptées depuis le 1er janvier 1970. Soustrayez deux instants pour mesurer une durée.",
       "start = time()"),
    b!("random", "random(a, b), random(list)", 0, 2, Basics, random,
       "random() gives a decimal number between 0 and 1; random(a, b) a whole number from a to b (included); random(list) a random element.",
       "random() donne un nombre décimal entre 0 et 1 ; random(a, b) un nombre entier de a à b (inclus) ; random(liste) un élément au hasard.",
       "random(1, 6)               # a dice roll"),
    // ----- Numbers -----
    b!("random_seed", "random_seed(n)", 1, 1, Basics, random_seed,
       "Makes the draws of `random` and `shuffle` that follow the same on every run: the same seed (a whole number) gives the same draws, with the same version of Rythme. For tests and classes; `rythme run --seed N file.ry` does the same from the command line.",
       "Rend identiques à chaque exécution les tirages de `random` et `shuffle` qui suivent : la même graine (un nombre entier) donne les mêmes tirages, avec la même version de Rythme. Pour les tests et les cours ; `rythme run --seed N fichier.ry` fait de même depuis la ligne de commande.",
       "random_seed(42)\nrandom(1, 6)               # 5\nrandom(1, 6)               # 2"),
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
    b!("asin", "asin(x)", 1, 1, Numbers, asin,
       "Arc sine: the angle (in radians) whose sine is x, for x from -1 to 1.",
       "Arc sinus : l'angle (en radians) dont le sinus est x, pour x de -1 à 1.",
       "asin(1)                    # 1.5707963267948966"),
    b!("acos", "acos(x)", 1, 1, Numbers, acos,
       "Arc cosine: the angle (in radians) whose cosine is x, for x from -1 to 1.",
       "Arc cosinus : l'angle (en radians) dont le cosinus est x, pour x de -1 à 1.",
       "acos(1)                    # 0.0"),
    b!("atan", "atan(y, x)", 1, 2, Numbers, atan,
       "Arc tangent: the angle (in radians) whose tangent is y. With two numbers, the angle of the point (x, y), from -pi to pi.",
       "Arc tangente : l'angle (en radians) dont la tangente est y. Avec deux nombres, l'angle du point (x, y), de -pi à pi.",
       "atan(1)                    # 0.7853981633974483\natan(1, -1)                # 2.356194490192345"),
    b!("log", "log(x, base)", 1, 2, Numbers, log,
       "Logarithm: natural (base e) by default, or in the given base.",
       "Logarithme : naturel (base e) par défaut, ou dans la base donnée.",
       "log(100, 10)               # 2.0"),
    b!("exp", "exp(x)", 1, 1, Numbers, exp, "e to the power x.", "e à la puissance x.", "exp(0)                     # 1.0"),
    b!("is_even", "is_even(n)", 1, 1, Numbers, is_even, "Is the whole number even?", "Le nombre entier est-il pair ?", "is_even(4)                 # true"),
    b!("is_odd", "is_odd(n)", 1, 1, Numbers, is_odd, "Is the whole number odd?", "Le nombre entier est-il impair ?", "is_odd(4)                  # false"),
    b!("gcd", "gcd(a, b)", 2, 2, Numbers, gcd,
       "Greatest common divisor of two whole numbers: the largest number that divides both.",
       "Plus grand commun diviseur (PGCD) de deux nombres entiers : le plus grand nombre qui divise les deux.",
       "gcd(12, 18)                # 6"),
    b!("lcm", "lcm(a, b)", 2, 2, Numbers, lcm,
       "Least common multiple of two whole numbers: the smallest number divisible by both.",
       "Plus petit commun multiple (PPCM) de deux nombres entiers : le plus petit nombre divisible par les deux.",
       "lcm(4, 6)                  # 12"),
    // ----- Text -----
    b!("len", "len(x)", 1, 1, Text, len,
       "Length of a text (in characters), a list, a record, a dict or a set.",
       "Longueur d'un texte (en caractères), d'une liste, d'un enregistrement, d'un dictionnaire ou d'un ensemble.",
       "len(\"été\")                 # 3\nlen([1, 2])                # 2"),
    b!("upper", "upper(text)", 1, 1, Text, upper, "The text in uppercase.", "Le texte en majuscules.", "upper(\"abc\")               # \"ABC\""),
    b!("lower", "lower(text)", 1, 1, Text, lower, "The text in lowercase.", "Le texte en minuscules.", "lower(\"ABC\")               # \"abc\""),
    b!("capitalize", "capitalize(text)", 1, 1, Text, capitalize,
       "The text with its first character in uppercase (the rest is unchanged).",
       "Le texte avec son premier caractère en majuscule (le reste ne change pas).",
       "capitalize(\"élise\")        # \"Élise\""),
    b!("trim", "trim(text)", 1, 1, Text, trim,
       "The text without spaces and line breaks at the start and the end.",
       "Le texte sans espaces ni retours à la ligne au début et à la fin.",
       "trim(\"  hi \")              # \"hi\""),
    b!("pad_left", "pad_left(text, width, fill)", 2, 3, Text, pad_left,
       "Adds `fill` (a space by default) before the text until it is `width` characters long: numbers line up in columns. A value that is not a text is first shown as `print` shows it.",
       "Ajoute `fill` (un espace par défaut) avant le texte jusqu'à ce qu'il fasse `width` caractères : les nombres s'alignent en colonnes. Une valeur qui n'est pas un texte est d'abord montrée comme `print` la montre.",
       "pad_left(\"7\", 3, \"0\")       # \"007\"\npad_left(42, 5)            # \"   42\""),
    b!("pad_right", "pad_right(text, width, fill)", 2, 3, Text, pad_right,
       "Adds `fill` (a space by default) after the text until it is `width` characters long.",
       "Ajoute `fill` (un espace par défaut) après le texte jusqu'à ce qu'il fasse `width` caractères.",
       "pad_right(\"ab\", 4, \".\")    # \"ab..\""),
    b!("split", "split(text, separator)", 1, 2, Text, split,
       "Cuts a text into a list of texts. Without separator, cuts at spaces.",
       "Découpe un texte en une liste de textes. Sans séparateur, découpe aux espaces.",
       "split(\"a,b,c\", \",\")        # [\"a\", \"b\", \"c\"]\nsplit(\"hello world\")       # [\"hello\", \"world\"]"),
    b!("join", "join(list, separator)", 1, 2, Text, join,
       "Glues the elements of a list into one text, with the separator between them.",
       "Colle les éléments d'une liste en un seul texte, avec le séparateur entre eux.",
       "join([\"a\", \"b\"], \"-\")      # \"a-b\""),
    b!("contains", "contains(x, item)", 2, 2, Text, contains,
       "Does the text contain this piece of text? Does the list (or the set) contain this element? Does the record have this field, or the dict this key?",
       "Le texte contient-il ce morceau de texte ? La liste (ou l'ensemble) contient-elle cet élément ? L'enregistrement a-t-il ce champ, ou le dictionnaire cette clé ?",
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
    b!("char_code", "char_code(char)", 1, 1, Text, char_code,
       "The number of a character (its Unicode code point).",
       "Le numéro d'un caractère (son point de code Unicode).",
       "char_code(\"A\")             # 65"),
    b!("char", "char(code)", 1, 1, Text, to_char,
       "The character with this number (Unicode code point): the opposite of `char_code`.",
       "Le caractère qui a ce numéro (point de code Unicode) : l'inverse de `char_code`.",
       "char(65)                   # \"A\"\nchar(char_code(\"a\") + 1)   # \"b\""),
    b!("is_digit", "is_digit(text)", 1, 1, Text, is_digit,
       "Is the text made only of digits from 0 to 9 (and not empty)?",
       "Le texte n'est-il fait que de chiffres de 0 à 9 (et non vide) ?",
       "is_digit(\"42\")             # true\nis_digit(\"4a\")             # false"),
    b!("is_letter", "is_letter(text)", 1, 1, Text, is_letter,
       "Is the text made only of letters, accented ones included (and not empty)?",
       "Le texte n'est-il fait que de lettres, accentuées comprises (et non vide) ?",
       "is_letter(\"été\")           # true"),
    b!("is_space", "is_space(text)", 1, 1, Text, is_space,
       "Is the text made only of spaces, tabs and line breaks (and not empty)?",
       "Le texte n'est-il fait que d'espaces, de tabulations et de retours à la ligne (et non vide) ?",
       "is_space(\" \\n\")            # true"),
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
    b!("parse_json", "parse_json(text)", 1, 1, Text, parse_json,
       "Reads a JSON text: `Ok(value)`, or `Err(message)` if the text is not valid JSON. Objects become records, arrays lists, and null `None`.",
       "Lit un texte JSON : `Ok(valeur)`, ou `Err(message)` si le texte n'est pas du JSON valide. Les objets deviennent des enregistrements, les tableaux des listes, et null `None`.",
       "parse_json(\"[1, 2.5, null]\")   # Ok([1, 2.5, None])\nparse_json(\"[1,\")         # Err(...)"),
    b!("to_json", "to_json(value, indent)", 1, 2, Text, to_json,
       "Writes a value as JSON text: records become objects, lists arrays, and `None` null. With `indent`, one element per line, shifted by that many spaces.",
       "Écrit une valeur en texte JSON : les enregistrements deviennent des objets, les listes des tableaux, et `None` null. Avec `indent`, un élément par ligne, décalé de ce nombre d'espaces.",
       "to_json([1.5, None, true])  # \"[1.5, null, true]\""),
    b!("trim_start", "trim_start(text)", 1, 1, Text, trim_start,
       "The text without spaces and line breaks at the start.",
       "Le texte sans espaces ni retours à la ligne au début.",
       "trim_start(\"  hi  \")       # \"hi  \""),
    b!("trim_end", "trim_end(text)", 1, 1, Text, trim_end,
       "The text without spaces and line breaks at the end.",
       "Le texte sans espaces ni retours à la ligne à la fin.",
       "trim_end(\"  hi  \")         # \"  hi\""),
    b!("words", "words(text)", 1, 1, Text, words,
       "The list of the words of a text: letters and digits, accents included, with `'` or `-` inside a word. Punctuation and spaces separate them.",
       "La liste des mots d'un texte : lettres et chiffres, accents compris, avec `'` ou `-` à l'intérieur d'un mot. La ponctuation et les espaces les séparent.",
       "words(\"Hello, world! It's me.\")   # [\"Hello\", \"world\", \"It's\", \"me\"]"),
    b!("equals_ignore_case", "equals_ignore_case(a, b)", 2, 2, Text, equals_ignore_case,
       "Are the two texts the same when uppercase and lowercase letters count as equal?",
       "Les deux textes sont-ils pareils quand majuscules et minuscules comptent comme égales ?",
       "equals_ignore_case(\"Été\", \"éTÉ\")   # true"),
    b!("format_number", "format_number(x, decimals, language)", 1, 3, Text, format_number,
       "Writes a number for people: thousands separated, with exactly `decimals` decimals if given, in the style of a language: \"en\" (1,234.5, by default), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" or \"pt\" (1.234,5). To line it up in a column, use `pad_left`.",
       "Écrit un nombre pour les humains : milliers séparés, avec exactement `decimals` décimales si elles sont données, dans le style d'une langue : \"en\" (1,234.5, par défaut), \"fr\" (1 234,5), \"de\", \"es\", \"it\", \"nl\" ou \"pt\" (1.234,5). Pour l'aligner dans une colonne, utilisez `pad_left`.",
       "format_number(1234567)     # \"1,234,567\"\nformat_number(1234.5, 2)   # \"1,234.50\"\nformat_number(-1234.5, 2, \"fr\")   # \"-1 234,50\""),
    b!("matches", "matches(text, regex)", 2, 2, Text, matches,
       "Does the whole text match the regex (a regular expression: see `rythme help regex`)? To look for the regex anywhere in the text, use `find_first`.",
       "Le texte entier correspond-il à la regex (une expression régulière : voir `rythme help regex`) ? Pour chercher la regex n'importe où dans le texte, utilisez `find_first`.",
       "matches(\"2024\", \"[0-9]+\")      # true\nmatches(\"2024!\", \"[0-9]+\")     # false"),
    b!("find_first", "find_first(text, regex)", 2, 2, Text, find_first,
       "The first piece of the text that matches the regex, as an Option: `Some(piece)`, or `None` if there is none.",
       "Le premier morceau du texte qui correspond à la regex, sous forme d'Option : `Some(morceau)`, ou `None` s'il n'y en a pas.",
       "find_first(\"tel: 06 12\", \"[0-9]+\")   # Some(\"06\")"),
    b!("find_all", "find_all(text, regex)", 2, 2, Text, find_all,
       "Every piece of the text that matches the regex, from left to right.",
       "Tous les morceaux du texte qui correspondent à la regex, de gauche à droite.",
       "find_all(\"3 cats, 12 dogs\", \"[0-9]+\")   # [\"3\", \"12\"]"),
    b!("captures", "captures(text, regex)", 2, 2, Text, captures,
       "The first match of the regex, as an Option of a list: the whole piece, then the text of each group `(...)`, in order (\"\" for a group that matched nothing).",
       "La première correspondance de la regex, sous forme d'Option d'une liste : le morceau entier, puis le texte de chaque groupe `(...)`, dans l'ordre (\"\" pour un groupe qui n'a rien trouvé).",
       "captures(\"2024-02-29\", \"([0-9]+)-([0-9]+)\")   # Some([\"2024-02\", \"2024\", \"02\"])"),
    b!("captures_all", "captures_all(text, regex)", 2, 2, Text, captures_all,
       "Like `captures`, for every match: a list of lists.",
       "Comme `captures`, pour chaque correspondance : une liste de listes.",
       "captures_all(\"a=1, b=2\", \"(\\\\w)=(\\\\d)\")   # [[\"a=1\", \"a\", \"1\"], [\"b=2\", \"b\", \"2\"]]"),
    b!("named_captures", "named_captures(text, regex)", 2, 2, Text, named_captures,
       "The first match of a regex with named groups `(?<name>...)`, as an Option of a record: one field for each name.",
       "La première correspondance d'une regex avec des groupes nommés `(?<nom>...)`, sous forme d'Option d'un enregistrement : un champ par nom.",
       "named_captures(\"2024-02\", \"(?<year>[0-9]+)-(?<month>[0-9]+)\")   # Some({year: \"2024\", month: \"02\"})"),
    b!("split_by", "split_by(text, regex)", 2, 2, Text, split_by,
       "Cuts a text at every piece that matches the regex.",
       "Découpe un texte à chaque morceau qui correspond à la regex.",
       "split_by(\"a, b;c\", \"[,;] *\")   # [\"a\", \"b\", \"c\"]"),
    b!("replace_all", "replace_all(text, regex, new)", 3, 3, Text, replace_all,
       "Replaces every piece that matches the regex. `new` is a text, where `$1` stands for the text of group 1 (`$<name>` for a named group, `$0` for the whole piece, `$$` for a `$`), or a function that receives each piece and gives its replacement.",
       "Remplace chaque morceau qui correspond à la regex. `new` est un texte, où `$1` tient lieu du texte du groupe 1 (`$<nom>` pour un groupe nommé, `$0` pour le morceau entier, `$$` pour un `$`), ou une fonction qui reçoit chaque morceau et donne son remplacement.",
       "replace_all(\"29/02/2024\", \"(\\\\d+)/(\\\\d+)/(\\\\d+)\", \"$3-$2-$1\")   # \"2024-02-29\"\nreplace_all(\"a cat\", \"\\\\b\\\\w\", upper)   # \"A Cat\""),
    b!("csv_parse", "csv_parse(text, separator)", 1, 2, Text, csv_parse,
       "Reads a CSV table (a spreadsheet saved as text): `Ok(list of rows)`, each row a list of texts, or `Err(message)` saying at which line and column the text is not valid CSV. A field between quotes may hold separators, line breaks and quotes (written twice). The separator is \",\" unless given: \";\" for French spreadsheets, \"\\t\" for tabs. See `rythme help csv`.",
       "Lit un tableau CSV (un tableur enregistré en texte) : `Ok(liste de lignes)`, chaque ligne une liste de textes, ou `Err(message)` qui dit à quelle ligne et quelle colonne le texte n'est pas du CSV valide. Un champ entre guillemets peut contenir des séparateurs, des retours à la ligne et des guillemets (écrits deux fois). Le séparateur est \",\" sauf s'il est donné : \";\" pour les tableurs en français, \"\\t\" pour les tabulations. Voir `rythme help csv`.",
       "csv_parse(\"name,age\\nAda,36\")   # Ok([[\"name\", \"age\"], [\"Ada\", \"36\"]])\ncsv_parse(\"a;b\", \";\")             # Ok([[\"a\", \"b\"]])"),
    b!("csv_records", "csv_records(text, separator)", 1, 2, Text, csv_records,
       "Reads a CSV table whose first line names the columns: `Ok(list of records)`, one record for each other line, with one field for each column, in their order, holding a text. `Err(message)` when the text is not valid CSV, when a line does not have as many fields as the header, or when two columns have the same name. To get numbers, use `parse_int` or `parse_float` on the fields.",
       "Lit un tableau CSV dont la première ligne nomme les colonnes : `Ok(liste d'enregistrements)`, un enregistrement pour chaque autre ligne, avec un champ pour chaque colonne, dans leur ordre, qui contient un texte. `Err(message)` quand le texte n'est pas du CSV valide, quand une ligne n'a pas autant de champs que l'en-tête, ou quand deux colonnes ont le même nom. Pour avoir des nombres, utilisez `parse_int` ou `parse_float` sur les champs.",
       "csv_records(\"name,age\\nAda,36\")   # Ok([{name: \"Ada\", age: \"36\"}])"),
    b!("csv_format", "csv_format(rows, separator)", 1, 2, Text, csv_format,
       "Writes rows as a CSV text, each one on its line, ended by a line break. The rows are lists of fields, or records: then a first line names their fields. A field is a text, a number or a Bool, put between quotes only when it needs them (a separator, a quote or a line break inside). The separator is \",\" unless given.",
       "Écrit des lignes en texte CSV, chacune sur sa ligne, terminée par un retour à la ligne. Les lignes sont des listes de champs, ou des enregistrements : une première ligne nomme alors leurs champs. Un champ est un texte, un nombre ou un Bool, mis entre guillemets seulement quand il en a besoin (un séparateur, un guillemet ou un retour à la ligne à l'intérieur). Le séparateur est \",\" sauf s'il est donné.",
       "csv_format([[\"name\", \"note\"], [\"Ada\", \"a, b\"]])   # \"name,note\\nAda,\\\"a, b\\\"\\n\"\ncsv_format([{x: 1, y: 2}], \";\")                   # \"x;y\\n1;2\\n\""),
    // ----- Lists -----
    b!("map", "map(list, f)", 2, 2, Lists, map,
       "Applies the function f to each element and returns the list of the results. Also works on Option and Result, and on a set (giving the set of the results).",
       "Applique la fonction f à chaque élément et renvoie la liste des résultats. Fonctionne aussi sur Option et Result, et sur un ensemble (en donnant l'ensemble des résultats).",
       "[1, 2, 3] |> map(fn(x) = x * 2)   # [2, 4, 6]"),
    b!("filter", "filter(list, f)", 2, 2, Lists, filter,
       "Keeps only the elements for which f returns true. Also works on a set, and on a dict, where f receives each key and its value.",
       "Ne garde que les éléments pour lesquels f renvoie true. Fonctionne aussi sur un ensemble, et sur un dictionnaire, où f reçoit chaque clé et sa valeur.",
       "[1, 2, 3, 4] |> filter(is_even)   # [2, 4]\ndict([[1, \"a\"], [2, \"b\"]]) |> filter(fn(k, v) = k > 1)   # dict(2 => \"b\")"),
    b!("fold", "fold(list, start, f)", 3, 3, Lists, fold,
       "Combines all the elements into one value: starts with `start`, then computes f(result, element) for each element.",
       "Combine tous les éléments en une seule valeur : part de `start`, puis calcule f(résultat, élément) pour chaque élément.",
       "fold([1, 2, 3], 0, fn(total, x) = total + x)   # 6"),
    b!("scan", "scan(list, start, f)", 3, 3, Lists, scan,
       "Like fold, but gives the list of every intermediate result: running totals, for example.",
       "Comme fold, mais donne la liste de tous les résultats intermédiaires : des totaux cumulés, par exemple.",
       "scan([1, 2, 3], 0, fn(total, x) = total + x)   # [1, 3, 6]"),
    b!("each", "each(list, f)", 2, 2, Lists, each,
       "Calls f on each element, for its effect (like printing). Returns (). Also works on a set, and on a dict, where f receives each key and its value.",
       "Appelle f sur chaque élément, pour son effet (comme afficher). Renvoie (). Fonctionne aussi sur un ensemble, et sur un dictionnaire, où f reçoit chaque clé et sa valeur.",
       "1..3 |> each(fn(i) = print(i))\ndict([[\"a\", 1]]).each do |k, v| print(k, v) end   # a 1"),
    b!("sum", "sum(list, f)", 1, 2, Lists, sum,
       "The sum of the numbers of a list; with f, the sum of f(x) for each element.",
       "La somme des nombres d'une liste ; avec f, la somme de f(x) pour chaque élément.",
       "sum([1, 2, 3])             # 6\nsum([\"ab\", \"c\"], len)     # 3"),
    b!("product", "product(list)", 1, 1, Lists, product, "The product of the numbers of a list.", "Le produit des nombres d'une liste.", "product([2, 3, 4])         # 24"),
    b!("first", "first(list, n)", 1, 2, Lists, first,
       "The first element of a list (or character of a text), as an Option: `Some(x)`, or `None` if there is none. With n, the first n elements (like `take`).",
       "Le premier élément d'une liste (ou caractère d'un texte), sous forme d'Option : `Some(x)`, ou `None` s'il n'y en a pas. Avec n, les n premiers éléments (comme `take`).",
       "first([7, 8])              # Some(7)\nfirst([7, 8, 9], 2)        # [7, 8]"),
    b!("last", "last(list, n)", 1, 2, Lists, last,
       "The last element of a list (or character of a text), as an Option. With n, the last n elements.",
       "Le dernier élément d'une liste (ou caractère d'un texte), sous forme d'Option. Avec n, les n derniers éléments.",
       "last([7, 8])               # Some(8)\nlast([7, 8, 9], 2)         # [8, 9]"),
    b!("rest", "rest(list)", 1, 1, Lists, rest,
       "The list (or text) without its first element.",
       "La liste (ou le texte) sans son premier élément.",
       "rest([7, 8, 9])            # [8, 9]"),
    b!("get", "get(x, key)", 2, 2, Lists, get,
       "The element at a position of a list or text, the field of a record, or the value of a key of a dict, as an Option (never an error). Negative positions count from the end.",
       "L'élément à une position d'une liste ou d'un texte, le champ d'un enregistrement, ou la valeur d'une clé d'un dictionnaire, sous forme d'Option (jamais d'erreur). Les positions négatives comptent depuis la fin.",
       "get([7, 8], 5)             # None\nget({a: 1}, \"a\")           # Some(1)\nget(dict([[1, \"one\"]]), 1)   # Some(\"one\")"),
    b!("push", "push(list, x)", 2, 2, Lists, push,
       "A new list with x added at the end; or a new set with x added.",
       "Une nouvelle liste avec x ajouté à la fin ; ou un nouvel ensemble avec x ajouté.",
       "push([1, 2], 3)            # [1, 2, 3]"),
    b!("set_at", "set_at(list, i, x)", 3, 3, Lists, set_at,
       "A new list where the element at position i is replaced by x. Negative positions count from the end.",
       "Une nouvelle liste où l'élément à la position i est remplacé par x. Les positions négatives comptent depuis la fin.",
       "set_at([1, 2, 3], 0, 9)    # [9, 2, 3]"),
    b!("insert_at", "insert_at(list, i, x)", 3, 3, Lists, insert_at,
       "A new list with x inserted so that it is at position i (-1 adds it at the end).",
       "Une nouvelle liste avec x inséré pour qu'il soit à la position i (-1 l'ajoute à la fin).",
       "insert_at([1, 2, 3], 1, 9)  # [1, 9, 2, 3]"),
    b!("remove_at", "remove_at(list, i)", 2, 2, Lists, remove_at,
       "A new list without the element at position i. Negative positions count from the end.",
       "Une nouvelle liste sans l'élément à la position i. Les positions négatives comptent depuis la fin.",
       "remove_at([1, 2, 3], -1)   # [1, 2]"),
    b!("reverse", "reverse(x)", 1, 1, Lists, reverse, "The list or text in reverse order.", "La liste ou le texte dans l'ordre inverse.", "reverse([1, 2, 3])         # [3, 2, 1]"),
    b!("sort", "sort(list)", 1, 1, Lists, sort,
       "The list sorted from smallest to largest (numbers, texts in alphabetical order...). For the largest first: `sort(list) |> reverse`.",
       "La liste triée du plus petit au plus grand (nombres, textes dans l'ordre alphabétique...). Pour le plus grand d'abord : `sort(liste) |> reverse`.",
       "sort([3, 1, 2])            # [1, 2, 3]"),
    b!("sort_by", "sort_by(list, key)", 2, 2, Lists, sort_by,
       "Sorts the list by the value that the function `key` gives for each element. Elements with the same value keep their order.",
       "Trie la liste selon la valeur que la fonction `key` donne pour chaque élément. Les éléments de même valeur gardent leur ordre.",
       "sort_by([\"ccc\", \"a\"], len)  # [\"a\", \"ccc\"]"),
    b!("unique", "unique(list)", 1, 1, Lists, unique,
       "The list without duplicates (the first occurrence is kept).",
       "La liste sans doublons (la première occurrence est gardée).",
       "unique([1, 2, 1])          # [1, 2]"),
    b!("shuffle", "shuffle(list)", 1, 1, Lists, shuffle,
       "The elements of the list in a random order.",
       "Les éléments de la liste dans un ordre au hasard.",
       "shuffle([1, 2, 3])         # [3, 1, 2], or another order"),
    b!("slice", "slice(x, start, end)", 2, 3, Lists, slice,
       "The part of a list or text from position `start` (included) to `end` (excluded). Negative positions count from the end.",
       "La partie d'une liste ou d'un texte de la position `start` (incluse) à `end` (exclue). Les positions négatives comptent depuis la fin.",
       "slice([1, 2, 3, 4], 1, 3)  # [2, 3]\nslice(\"hello\", -3)         # \"llo\""),
    b!("take", "take(x, n)", 2, 2, Lists, take, "The first n elements of a list or text.", "Les n premiers éléments d'une liste ou d'un texte.", "take([1, 2, 3], 2)         # [1, 2]"),
    b!("drop", "drop(x, n)", 2, 2, Lists, drop, "The list or text without its first n elements.", "La liste ou le texte sans ses n premiers éléments.", "drop([1, 2, 3], 2)         # [3]"),
    b!("take_while", "take_while(x, f)", 2, 2, Lists, take_while,
       "The first elements of a list (or characters of a text), as long as f is true for them.",
       "Les premiers éléments d'une liste (ou caractères d'un texte), tant que f est vraie pour eux.",
       "take_while([1, 3, 4, 5], is_odd)   # [1, 3]\ntake_while(\"42 cm\", is_digit)     # \"42\""),
    b!("drop_while", "drop_while(x, f)", 2, 2, Lists, drop_while,
       "The list (or text) without its first elements for which f is true.",
       "La liste (ou le texte) sans ses premiers éléments pour lesquels f est vraie.",
       "drop_while([1, 3, 4, 5], is_odd)   # [4, 5]"),
    b!("step_by", "step_by(x, n)", 2, 2, Lists, step_by,
       "One element out of n of a list (or text), starting with the first. On a range: the numbers with a step.",
       "Un élément sur n d'une liste (ou d'un texte), en commençant par le premier. Sur un intervalle : les nombres avec un pas.",
       "0..10 |> step_by(5)        # [0, 5, 10]"),
    b!("chunks", "chunks(x, n)", 2, 2, Lists, chunks,
       "Cuts a list (or text) into pieces of n elements; the last piece may be shorter.",
       "Découpe une liste (ou un texte) en morceaux de n éléments ; le dernier morceau peut être plus court.",
       "chunks([1, 2, 3, 4, 5], 2)  # [[1, 2], [3, 4], [5]]"),
    b!("windows", "windows(x, n)", 2, 2, Lists, windows,
       "Every group of n neighbouring elements of a list (or text), in order: handy to compare each element with the next one.",
       "Chaque groupe de n éléments voisins d'une liste (ou d'un texte), dans l'ordre : pratique pour comparer chaque élément au suivant.",
       "windows([1, 2, 3, 4], 2)   # [[1, 2], [2, 3], [3, 4]]"),
    b!("zip", "zip(a, b)", 2, 2, Lists, zip,
       "Pairs the elements of two lists: [[a1, b1], [a2, b2], ...]. It stops at the end of the shorter list.",
       "Associe les éléments de deux listes : [[a1, b1], [a2, b2], ...]. S'arrête à la fin de la liste la plus courte.",
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
    b!("count", "count(list, f)", 1, 2, Lists, count,
       "How many elements make f true; without f, how many elements there are.",
       "Combien d'éléments rendent f vraie ; sans f, combien il y a d'éléments.",
       "count([1, 2, 4], is_even)  # 2\ncount([1, 2, 4])           # 3"),
    b!("reject", "reject(list, f)", 2, 2, Lists, reject,
       "The elements for which f is false: the opposite of `filter`. Also works on a set and on a dict, like `filter`.",
       "Les éléments pour lesquels f est fausse : le contraire de `filter`. Fonctionne aussi sur un ensemble et sur un dictionnaire, comme `filter`.",
       "reject([1, 2, 3, 4], is_even) # [1, 3]"),
    b!("each_with_index", "each_with_index(list, f)", 2, 2, Lists, each_with_index,
       "Calls f(element, position) for each element, in order (positions start at 0). For effects, like `print`.",
       "Appelle f(élément, position) pour chaque élément, dans l'ordre (les positions commencent à 0). Pour des effets, comme `print`.",
       "each_with_index([\"a\", \"b\"], fn(x, i) = print(i, x))"),
    b!("index_of", "index_of(x, item)", 2, 2, Lists, index_of,
       "The position of an element in a list (or of a piece of text in a text), as an Option.",
       "La position d'un élément dans une liste (ou d'un morceau de texte dans un texte), sous forme d'Option.",
       "index_of([5, 6, 7], 6)     # Some(1)"),
    b!("min_by", "min_by(list, key)", 2, 2, Lists, min_by,
       "The element for which the function `key` gives the smallest value (the first one if there are several).",
       "L'élément pour lequel la fonction `key` donne la plus petite valeur (le premier s'il y en a plusieurs).",
       "min_by([\"ccc\", \"a\", \"bb\"], len)   # \"a\""),
    b!("max_by", "max_by(list, key)", 2, 2, Lists, max_by,
       "The element for which the function `key` gives the largest value (the first one if there are several).",
       "L'élément pour lequel la fonction `key` donne la plus grande valeur (le premier s'il y en a plusieurs).",
       "people = [{name: \"Ada\", age: 36}, {name: \"Alan\", age: 41}]\nmax_by(people, fn(p) = p.age).name   # \"Alan\""),
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
    b!("is_empty", "is_empty(x)", 1, 1, Lists, is_empty,
       "Is the list, text, record, dict or set empty?",
       "La liste, le texte, l'enregistrement, le dictionnaire ou l'ensemble est-il vide ?",
       "is_empty([])               # true\nis_empty(set())            # true"),
    // ----- Records -----
    b!("keys", "keys(record)", 1, 1, Records, keys,
       "The list of the field names of a record, or of the keys of a dict (in order).",
       "La liste des noms des champs d'un enregistrement, ou des clés d'un dictionnaire (dans l'ordre).",
       "keys({a: 1, b: 2})         # [\"a\", \"b\"]\nkeys(dict([[2, \"b\"], [1, \"a\"]]))   # [1, 2]"),
    b!("values", "values(record)", 1, 1, Records, values,
       "The list of the values of a record or of a dict (in the order of its keys), or of the elements of a set (in order).",
       "La liste des valeurs d'un enregistrement ou d'un dictionnaire (dans l'ordre de ses clés), ou des éléments d'un ensemble (dans l'ordre).",
       "values({a: 1, b: 2})       # [1, 2]\nvalues(set([3, 1, 3]))     # [1, 3]"),
    b!("entries", "entries(record)", 1, 1, Records, entries,
       "The list of the [name, value] pairs of a record, or of the [key, value] pairs of a dict.",
       "La liste des paires [nom, valeur] d'un enregistrement, ou des paires [clé, valeur] d'un dictionnaire.",
       "entries({a: 1})            # [[\"a\", 1]]"),
    b!("from_entries", "from_entries(list)", 1, 1, Records, from_entries,
       "Builds a record from a list of [name, value] pairs: the opposite of `entries`.",
       "Construit un enregistrement à partir d'une liste de paires [nom, valeur] : l'inverse de `entries`.",
       "from_entries([[\"a\", 1], [\"b\", 2]])   # {a: 1, b: 2}"),
    b!("has", "has(record, name)", 2, 2, Records, has,
       "Does the record have this field? Does the dict have this key, or the set this element?",
       "L'enregistrement a-t-il ce champ ? Le dictionnaire a-t-il cette clé, ou l'ensemble cet élément ?",
       "has({a: 1}, \"a\")           # true\nhas(set([1, 2]), 3)        # false"),
    b!("put", "put(record, name, value)", 3, 3, Records, put,
       "A new record with the field set to the value (added or replaced); or a new dict with the key set to the value.",
       "Un nouvel enregistrement avec le champ fixé à la valeur (ajouté ou remplacé) ; ou un nouveau dictionnaire avec la clé fixée à la valeur.",
       "put({a: 1}, \"b\", 2)        # {a: 1, b: 2}\nput(dict(), 1, \"one\")      # dict(1 => \"one\")"),
    b!("remove", "remove(record, name)", 2, 2, Records, remove,
       "A new record without this field; or a new dict without this key, or a new set without this element.",
       "Un nouvel enregistrement sans ce champ ; ou un nouveau dictionnaire sans cette clé, ou un nouvel ensemble sans cet élément.",
       "remove({a: 1, b: 2}, \"a\")  # {b: 2}\nremove(set([1, 2]), 1)     # set(2)"),
    b!("map_values", "map_values(record, f)", 2, 2, Records, map_values,
       "A new record (or dict) where f is applied to every value; the names (or keys) stay the same.",
       "Un nouvel enregistrement (ou dictionnaire) où f est appliquée à chaque valeur ; les noms (ou les clés) restent les mêmes.",
       "map_values({a: 1, b: 2}, fn(x) = x * 10)   # {a: 10, b: 20}"),
    // ----- Dictionaries and sets -----
    b!("dict", "dict(pairs)", 0, 1, Dicts, dict,
       "A dictionary: keys of any kind (but functions), each with a value, kept in the order of the keys. Made from a list of [key, value] pairs (the last value of a key wins), from a record, or empty with `dict()`. Use it with get, put, has, remove, keys, values, entries, len, map_values, filter, union...",
       "Un dictionnaire : des clés de n'importe quelle sorte (sauf les fonctions), chacune avec une valeur, gardées dans l'ordre des clés. Fait d'une liste de paires [clé, valeur] (la dernière valeur d'une clé l'emporte), d'un enregistrement, ou vide avec `dict()`. S'utilise avec get, put, has, remove, keys, values, entries, len, map_values, filter, union...",
       "dict([[2, \"b\"], [1, \"a\"]])   # dict(1 => \"a\", 2 => \"b\")\ndict({x: 1})               # dict(\"x\" => 1)\nget(dict([[[0, 0], \"origin\"]]), [0, 0])   # Some(\"origin\")"),
    b!("set", "set(list)", 0, 1, Dicts, set,
       "A set: values (but functions) without repetition, kept in order. Made from a list, or empty with `set()`. Use it with has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.",
       "Un ensemble : des valeurs (sauf les fonctions) sans répétition, gardées dans l'ordre. Fait d'une liste, ou vide avec `set()`. S'utilise avec has, push, remove, len, values, filter, map, union, intersection, difference, is_subset.",
       "set([3, 1, 3, 2])          # set(1, 2, 3)\npush(set([1]), 5)          # set(1, 5)"),
    b!("union", "union(a, b)", 2, 2, Dicts, union,
       "The elements of two sets together; or the pairs of two dicts, where the value of b wins for a key in both.",
       "Les éléments de deux ensembles réunis ; ou les paires de deux dictionnaires, où la valeur de b l'emporte pour une clé présente dans les deux.",
       "union(set([1, 2]), set([2, 3]))   # set(1, 2, 3)\nunion(dict({a: 1, b: 2}), dict({b: 20}))   # dict(\"a\" => 1, \"b\" => 20)"),
    b!("intersection", "intersection(a, b)", 2, 2, Dicts, intersection,
       "The elements of the set a that are also in b; or the pairs of the dict a whose key is in b.",
       "Les éléments de l'ensemble a qui sont aussi dans b ; ou les paires du dictionnaire a dont la clé est dans b.",
       "intersection(set([1, 2, 3]), set([2, 3, 4]))   # set(2, 3)"),
    b!("difference", "difference(a, b)", 2, 2, Dicts, difference,
       "The elements of the set a that are not in b; or the pairs of the dict a whose key is not in b.",
       "Les éléments de l'ensemble a qui ne sont pas dans b ; ou les paires du dictionnaire a dont la clé n'est pas dans b.",
       "difference(set([1, 2, 3]), set([2]))   # set(1, 3)"),
    b!("is_subset", "is_subset(a, b)", 2, 2, Dicts, is_subset,
       "Is every element of the set a also in the set b?",
       "Chaque élément de l'ensemble a est-il aussi dans l'ensemble b ?",
       "is_subset(set([1, 2]), set([1, 2, 3]))   # true"),
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
    b!("and_then", "and_then(x, f)", 2, 2, Options, and_then,
       "Chains steps that may fail: for `Some(x)` or `Ok(x)`, gives f(x), which must be an Option or a Result; `None` and `Err(e)` stay as they are.",
       "Enchaîne des étapes qui peuvent échouer : pour `Some(x)` ou `Ok(x)`, donne f(x), qui doit être une Option ou un Result ; `None` et `Err(e)` restent tels quels.",
       "parse_int(\"42\") |> and_then(fn(n) = if n > 0 then Ok(n) else Err(\"not positive\"))   # Ok(42)"),
    b!("ok_or", "ok_or(option, error)", 2, 2, Options, ok_or,
       "Turns an Option into a Result: `Some(x)` gives `Ok(x)`, and `None` gives `Err(error)`. Handy before `?`.",
       "Transforme une Option en Result : `Some(x)` donne `Ok(x)`, et `None` donne `Err(error)`. Pratique avant `?`.",
       "ok_or(None, \"missing\")      # Err(\"missing\")"),
    b!("map_err", "map_err(result, f)", 2, 2, Options, map_err,
       "For `Err(e)`, gives `Err(f(e))`; `Ok(x)` stays as it is. Handy to explain an error better.",
       "Pour `Err(e)`, donne `Err(f(e))` ; `Ok(x)` reste tel quel. Pratique pour mieux expliquer une erreur.",
       "map_err(Err(\"not found\"), fn(e) = \"config: {e}\")   # Err(\"config: not found\")"),
    // ----- Files -----
    b!("read_file", "read_file(path)", 1, 1, Files, read_file,
       "Reads a whole text file: `Ok(text)`, or `Err(message)` if it cannot be read.",
       "Lit un fichier texte entier : `Ok(texte)`, ou `Err(message)` s'il ne peut pas être lu.",
       "match read_file(\"notes.txt\")\n  Ok(text) -> print(text)\n  Err(e) -> print(\"Error: {e}\")\nend"),
    b!("write_file", "write_file(path, text)", 2, 2, Files, write_file,
       "Writes a text in a file (replacing its content): `Ok(())` or `Err(message)`.",
       "Écrit un texte dans un fichier (en remplaçant son contenu) : `Ok(())` ou `Err(message)`.",
       "write_file(\"out.txt\", \"hello\")"),
    b!("append_file", "append_file(path, text)", 2, 2, Files, append_file,
       "Adds a text at the end of a file, creating the file if needed: `Ok(())` or `Err(message)`.",
       "Ajoute un texte à la fin d'un fichier, en créant le fichier si besoin : `Ok(())` ou `Err(message)`.",
       "append_file(\"log.txt\", \"done\\n\")"),
    b!("file_exists", "file_exists(path)", 1, 1, Files, file_exists,
       "Does this file (or folder) exist? Always `false` in the browser.",
       "Ce fichier (ou dossier) existe-t-il ? Toujours `false` dans le navigateur.",
       "file_exists(\"notes.txt\")"),
    b!("file_info", "file_info(path)", 1, 1, Files, file_info,
       "What is known about a file or a folder: `Ok({is_folder, size})`, with its size in bytes (0 for a folder), or `Err(message)` if it does not exist or cannot be read.",
       "Ce qu'on sait d'un fichier ou d'un dossier : `Ok({is_folder, size})`, avec sa taille en octets (0 pour un dossier), ou `Err(message)` s'il n'existe pas ou ne peut pas être lu.",
       "file_info(\"notes.txt\")      # Ok({is_folder: false, size: 11})"),
    b!("list_folder", "list_folder(path)", 1, 1, Files, list_folder,
       "The names of the files and folders inside a folder, in alphabetical order: `Ok(list of texts)`, or `Err(message)`. `list_folder(\".\")` lists the current folder.",
       "Les noms des fichiers et dossiers d'un dossier, dans l'ordre alphabétique : `Ok(liste de textes)`, ou `Err(message)`. `list_folder(\".\")` liste le dossier courant.",
       "list_folder(\"photos\")       # Ok([\"cat.jpg\", \"dog.jpg\"])"),
    b!("create_folder", "create_folder(path)", 1, 1, Files, create_folder,
       "Creates a folder, and the folders around it that are missing: `Ok(())`, also when it already exists, or `Err(message)`.",
       "Crée un dossier, et les dossiers autour qui manquent : `Ok(())`, aussi quand il existe déjà, ou `Err(message)`.",
       "create_folder(\"backup/2024\")"),
    b!("delete_file", "delete_file(path)", 1, 1, Files, delete_file,
       "Deletes one file (never a folder: that is an error): `Ok(())` or `Err(message)`. A deleted file cannot be brought back.",
       "Supprime un fichier (jamais un dossier : c'est une erreur) : `Ok(())` ou `Err(message)`. Un fichier supprimé ne peut pas être récupéré.",
       "delete_file(\"old.txt\")"),
    b!("join_path", "join_path(parts)", 1, 1, Files, join_path,
       "Joins the parts of a path with `/`, which works on Linux, macOS and Windows (with `\\` if the parts already use only `\\`). Only the first part can start at a root (`/`, `C:`): a part like \"/etc\" after another one is an error.",
       "Joint les parties d'un chemin avec `/`, qui marche sous Linux, macOS et Windows (avec `\\` si les parties utilisent déjà seulement `\\`). Seule la première partie peut partir d'une racine (`/`, `C:`) : une partie comme \"/etc\" après une autre est une erreur.",
       "join_path([\"data\", \"2024\", \"notes.txt\"])   # \"data/2024/notes.txt\"\njoin_path([\"C:\\\\Users\", \"ada\"])   # \"C:\\\\Users\\\\ada\""),
    b!("split_path", "split_path(path)", 1, 1, Files, split_path,
       "The parts of a path, split at each `/` or `\\`; the first part of an absolute path is its root (`/`, `C:\\`...). `join_path` puts them back together.",
       "Les parties d'un chemin, coupé à chaque `/` ou `\\` ; la première partie d'un chemin absolu est sa racine (`/`, `C:\\`...). `join_path` les remet ensemble.",
       "split_path(\"/home/ada/notes.txt\")   # [\"/\", \"home\", \"ada\", \"notes.txt\"]\nsplit_path(\"C:\\\\Users\\\\ada\")   # [\"C:\\\\\", \"Users\", \"ada\"]\nsplit_path(\"photos/cat.jpg\") |> last   # Some(\"cat.jpg\")"),
    // ----- Dates and times -----
    b!("now", "now()", 0, 0, Dates, now,
       "The date and time now, where the program runs: a record {year, month, day, hour, minute, second, offset}, where `offset` is the number of minutes the local time is ahead of UTC (0 when the system does not tell).",
       "La date et l'heure maintenant, là où le programme tourne : un enregistrement {year, month, day, hour, minute, second, offset}, où `offset` est le nombre de minutes d'avance de l'heure locale sur UTC (0 quand le système ne le dit pas).",
       "now()                      # {year: 2026, month: 10, day: 18, hour: 14, minute: 5, second: 9, offset: 120}"),
    b!("today", "today()", 0, 0, Dates, today,
       "The date of today, where the program runs: a record {year, month, day}.",
       "La date d'aujourd'hui, là où le programme tourne : un enregistrement {year, month, day}.",
       "today()                    # {year: 2026, month: 10, day: 18}"),
    b!("parse_date", "parse_date(text)", 1, 1, Dates, parse_date,
       "Reads a date written as ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", with `Z` or an offset like `+01:00` if needed): `Ok(date)`, or `Err(message)` if it is not a valid date.",
       "Lit une date écrite en ISO 8601 (\"2024-02-29\", \"2024-02-29T13:05:00\", avec `Z` ou un décalage comme `+01:00` si besoin) : `Ok(date)`, ou `Err(message)` si ce n'est pas une date valide.",
       "parse_date(\"2024-02-29\")   # Ok({year: 2024, month: 2, day: 29})\nparse_date(\"2023-02-29\")   # Err(\"\\\"2023-02-29\\\" is not a valid date: ...\")"),
    b!("format_date", "format_date(date)", 1, 1, Dates, format_date,
       "Writes a date as ISO 8601: \"2024-02-29\", with the time if the date has one, and `Z` or the offset if it has one.",
       "Écrit une date en ISO 8601 : \"2024-02-29\", avec l'heure si la date en a une, et `Z` ou le décalage si elle en a un.",
       "format_date({year: 2024, month: 2, day: 29, hour: 9, minute: 5})   # \"2024-02-29T09:05:00\""),
    b!("is_date", "is_date(value)", 1, 1, Dates, is_date,
       "Is the value a valid date: a record {year, month, day} (with `hour`, `minute`, `second`, `offset` if needed) for a day that exists?",
       "La valeur est-elle une date valide : un enregistrement {year, month, day} (avec `hour`, `minute`, `second`, `offset` si besoin) pour un jour qui existe ?",
       "is_date({year: 2024, month: 2, day: 29})   # true\nis_date({year: 2023, month: 2, day: 29})   # false"),
    b!("add_days", "add_days(date, n)", 2, 2, Dates, add_days,
       "The date n days later (or earlier, when n is negative), at the same time of day.",
       "La date n jours plus tard (ou plus tôt, quand n est négatif), à la même heure.",
       "add_days({year: 2024, month: 2, day: 28}, 2)   # {year: 2024, month: 3, day: 1}"),
    b!("add_seconds", "add_seconds(date, n)", 2, 2, Dates, add_seconds,
       "The date and time n seconds later (or earlier, when n is negative). Add `60 * minutes` or `3600 * hours` the same way.",
       "La date et l'heure n secondes plus tard (ou plus tôt, quand n est négatif). Ajoutez `60 * minutes` ou `3600 * heures` de la même façon.",
       "add_seconds({year: 2024, month: 12, day: 31, hour: 23, minute: 59, second: 30}, 45)   # {year: 2025, month: 1, day: 1, hour: 0, minute: 0, second: 15}"),
    b!("days_between", "days_between(a, b)", 2, 2, Dates, days_between,
       "The number of whole days from date a to date b: negative when b is before a.",
       "Le nombre de jours entiers de la date a à la date b : négatif quand b est avant a.",
       "days_between({year: 2024, month: 1, day: 1}, {year: 2025, month: 1, day: 1})   # 366"),
    b!("seconds_between", "seconds_between(a, b)", 2, 2, Dates, seconds_between,
       "The number of seconds from date a to date b, taking their offsets into account: negative when b is before a.",
       "Le nombre de secondes de la date a à la date b, en tenant compte de leurs décalages : négatif quand b est avant a.",
       "seconds_between({year: 2024, month: 1, day: 1}, {year: 2024, month: 1, day: 1, hour: 1})   # 3600"),
    b!("day_of_week", "day_of_week(date)", 1, 1, Dates, day_of_week,
       "The day of the week of a date, from 1 (Monday) to 7 (Sunday), as in ISO 8601.",
       "Le jour de la semaine d'une date, de 1 (lundi) à 7 (dimanche), comme en ISO 8601.",
       "day_of_week({year: 2024, month: 2, day: 29})   # 4"),
    b!("days_in_month", "days_in_month(year, month)", 2, 2, Dates, days_in_month,
       "The number of days of a month (from 1 to 12) of a year: 28 to 31.",
       "Le nombre de jours d'un mois (de 1 à 12) d'une année : de 28 à 31.",
       "days_in_month(2024, 2)     # 29"),
    b!("is_leap_year", "is_leap_year(year)", 1, 1, Dates, is_leap_year,
       "Does the year have a 29 February? Every 4 years, except every 100 years, except every 400 years.",
       "L'année a-t-elle un 29 février ? Tous les 4 ans, sauf tous les 100 ans, sauf tous les 400 ans.",
       "is_leap_year(2024)         # true\nis_leap_year(1900)         # false"),
    // ----- Property tests -----
    b!("check_all", "check_all(property, generator, cases)", 2, 3, Tests, check_all,
       "Checks that a property holds for many values made by a generator (100, or `cases`): the function is called with each value and must give `true` (or `()`, `Some(...)`, `Ok(...)`). When a value makes it false or stops it with an error, that value is simplified as much as possible (shrinking) and shown, with the seed that made it: the same values come back on every run. The property may also come last, as a block: `check_all(ints()) { |x| x + 0 == x }`.",
       "Vérifie qu'une propriété est vraie pour beaucoup de valeurs fabriquées par un générateur (100, ou `cases`) : la fonction est appelée avec chaque valeur et doit donner `true` (ou `()`, `Some(...)`, `Ok(...)`). Quand une valeur la rend fausse ou l'arrête sur une erreur, cette valeur est simplifiée autant que possible et montrée, avec la graine qui l'a donnée : les mêmes valeurs reviennent à chaque lancement. La propriété peut aussi venir en dernier, comme un bloc : `check_all(ints()) { |x| x + 0 == x }`.",
       "check_all(fn(x) = x + 0 == x, ints())   # ()\ncheck_all(fn(l) = reverse(reverse(l)) == l, lists_of(texts()))   # ()\ncheck_all(fn(x) = abs(x) >= 0, floats(-1, 1), 500)   # ()"),
    b!("sample", "sample(generator, n)", 1, 2, Tests, sample,
       "A list of `n` values (10 if not given) made by a generator, to see what `check_all` will try.",
       "Une liste de `n` valeurs (10 sinon) fabriquées par un générateur, pour voir ce que `check_all` essaiera.",
       "sample(ints(1, 6), 5)      # [...]"),
    b!("ints", "ints(min, max)", 0, 2, Tests, ints,
       "A generator of whole numbers from `min` to `max` (from -1000 to 1000 if not given), often near 0 and at the edges. A failing number is simplified towards 0.",
       "Un générateur de nombres entiers de `min` à `max` (de -1000 à 1000 sinon), souvent près de 0 et aux bords. Un nombre qui échoue est simplifié vers 0.",
       "ints(1, 6)                 # {generator: \"ints\", min: 1, max: 6}"),
    b!("floats", "floats(min, max)", 0, 2, Tests, floats,
       "A generator of decimal numbers from `min` to `max` (from -1000.0 to 1000.0 if not given).",
       "Un générateur de nombres décimaux de `min` à `max` (de -1000.0 à 1000.0 sinon).",
       "floats(0, 1)               # {generator: \"floats\", min: 0.0, max: 1.0}"),
    b!("texts", "texts(max_length)", 0, 1, Tests, texts,
       "A generator of texts of at most `max_length` characters (20 if not given): letters mostly, with digits, spaces, punctuation and accents. A failing text is made shorter and simpler.",
       "Un générateur de textes d'au plus `max_length` caractères (20 sinon) : surtout des lettres, avec des chiffres, des espaces, de la ponctuation et des accents. Un texte qui échoue est raccourci et simplifié.",
       "texts(5)                   # {generator: \"texts\", max_length: 5}"),
    b!("bools", "bools()", 0, 0, Tests, bools,
       "A generator of `true` and `false`.",
       "Un générateur de `true` et `false`.",
       "bools()                    # {generator: \"bools\"}"),
    b!("lists_of", "lists_of(generator, max_length)", 1, 2, Tests, lists_of,
       "A generator of lists of at most `max_length` elements (10 if not given), each made by `generator`. A failing list is made shorter, then its elements simpler.",
       "Un générateur de listes d'au plus `max_length` éléments (10 sinon), chacun fabriqué par `generator`. Une liste qui échoue est raccourcie, puis ses éléments simplifiés.",
       "lists_of(bools(), 3)       # {generator: \"lists_of\", of: {generator: \"bools\"}, max_length: 3}"),
    b!("one_of", "one_of(values)", 1, 1, Tests, one_of,
       "A generator that picks one of the values of a list. A failing value is replaced by one written before it.",
       "Un générateur qui choisit une des valeurs d'une liste. Une valeur qui échoue est remplacée par une valeur écrite avant elle.",
       "one_of([\"red\", \"green\"])   # {generator: \"one_of\", values: [\"red\", \"green\"]}"),
    b!("records_of", "records_of(fields)", 1, 1, Tests, records_of,
       "A generator of records, with a generator for each field: `records_of({name: texts(), age: ints(0, 120)})`. Handy to give several values to `check_all`.",
       "Un générateur d'enregistrements, avec un générateur pour chaque champ : `records_of({name: texts(), age: ints(0, 120)})`. Pratique pour donner plusieurs valeurs à `check_all`.",
       "records_of({x: bools()})   # {generator: \"records_of\", fields: {x: {generator: \"bools\"}}}"),

    // ----- Machine learning -----
    b!("tanh", "tanh(x)", 1, 1, Ml, tanh,
       "Hyperbolic tangent: a smooth step from -1 to 1, used as the activation of neurons.",
       "Tangente hyperbolique : une marche douce de -1 à 1, utilisée comme activation des neurones.",
       "tanh(0)                    # 0.0\ntanh(1)                    # 0.7615941559557649"),
    b!("sigmoid", "sigmoid(x)", 1, 1, Ml, sigmoid,
       "The logistic function 1 / (1 + e^-x): a smooth step from 0 to 1, read as a probability. Its slope is `sigmoid(x) * (1 - sigmoid(x))`.",
       "La fonction logistique 1 / (1 + e^-x) : une marche douce de 0 à 1, lue comme une probabilité. Sa pente est `sigmoid(x) * (1 - sigmoid(x))`.",
       "sigmoid(0)                 # 0.5\nsigmoid(2)                 # 0.8807970779778823"),
    b!("relu", "relu(x)", 1, 1, Ml, relu,
       "The rectifier: x when it is positive, 0.0 otherwise. The usual activation of deep networks.",
       "Le redresseur : x quand il est positif, 0.0 sinon. L'activation habituelle des réseaux profonds.",
       "relu(3)                    # 3.0\nrelu(-2)                   # 0.0"),
    b!("dot", "dot(xs, ys)", 2, 2, Ml, dot,
       "Dot product of two vectors (lists of numbers) of the same length: the sum of the products of their numbers, one by one.",
       "Produit scalaire de deux vecteurs (listes de nombres) de même longueur : la somme des produits de leurs nombres, un à un.",
       "dot([1, 2, 3], [4, 5, 6])  # 32.0"),
    b!("norm", "norm(xs)", 1, 1, Ml, norm,
       "Length of a vector: the square root of the sum of the squares of its numbers.",
       "Longueur d'un vecteur : la racine carrée de la somme des carrés de ses nombres.",
       "norm([3, 4])               # 5.0"),
    b!("vector_add", "vector_add(a, b)", 2, 2, Ml, vector_add,
       "Adds two vectors, or two matrices, of the same shape, number by number.",
       "Additionne deux vecteurs, ou deux matrices, de même forme, nombre par nombre.",
       "vector_add([1, 2], [10, 20])   # [11.0, 22.0]\nvector_add([[1], [2]], [[3], [4]])   # [[4.0], [6.0]]"),
    b!("vector_sub", "vector_sub(a, b)", 2, 2, Ml, vector_sub,
       "Subtracts two vectors, or two matrices, of the same shape, number by number.",
       "Soustrait deux vecteurs, ou deux matrices, de même forme, nombre par nombre.",
       "vector_sub([5, 5], [1, 2])     # [4.0, 3.0]"),
    b!("vector_mul", "vector_mul(a, b)", 2, 2, Ml, vector_mul,
       "Multiplies two vectors, or two matrices, of the same shape, number by number (not the product of matrices: that is `matmul`).",
       "Multiplie deux vecteurs, ou deux matrices, de même forme, nombre par nombre (pas le produit de matrices : c'est `matmul`).",
       "vector_mul([1, 2, 3], [2, 2, 2])   # [2.0, 4.0, 6.0]"),
    b!("vector_scale", "vector_scale(a, k)", 2, 2, Ml, vector_scale,
       "Multiplies every number of a vector, or of a matrix, by the number k.",
       "Multiplie chaque nombre d'un vecteur, ou d'une matrice, par le nombre k.",
       "vector_scale([1, 2], 0.5)      # [0.5, 1.0]"),
    b!("zip_with", "zip_with(xs, ys, f)", 3, 3, Lists, zip_with,
       "Calls f with the elements of two lists of the same length, one pair at a time, and gives the list of the results.",
       "Appelle f avec les éléments de deux listes de même longueur, une paire à la fois, et donne la liste des résultats.",
       "zip_with([1, 2], [10, 20], fn(a, b) = a + b)   # [11, 22]"),
    b!("matmul", "matmul(a, b)", 2, 2, Ml, matmul,
       "Product of two matrices (lists of rows): a matrix of r rows and n columns times a matrix of n rows and c columns gives r rows and c columns. With a vector as `b`, gives a vector.",
       "Produit de deux matrices (listes de lignes) : une matrice de r lignes et n colonnes fois une matrice de n lignes et c colonnes donne r lignes et c colonnes. Avec un vecteur comme `b`, donne un vecteur.",
       "matmul([[1, 2], [3, 4]], [[5], [6]])   # [[17.0], [39.0]]\nmatmul([[1, 2], [3, 4]], [1, 1])       # [3.0, 7.0]"),
    b!("transpose", "transpose(m)", 1, 1, Ml, transpose,
       "Swaps the rows and the columns of a matrix.",
       "Échange les lignes et les colonnes d'une matrice.",
       "transpose([[1, 2, 3], [4, 5, 6]])   # [[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]"),
    b!("zeros", "zeros(rows, columns)", 1, 2, Ml, zeros,
       "A vector of `rows` zeros, or with `columns`, a matrix full of zeros.",
       "Un vecteur de `rows` zéros, ou avec `columns`, une matrice remplie de zéros.",
       "zeros(3)                   # [0.0, 0.0, 0.0]\nzeros(2, 2)                # [[0.0, 0.0], [0.0, 0.0]]"),
    b!("ones", "ones(rows, columns)", 1, 2, Ml, ones,
       "A vector of `rows` ones, or with `columns`, a matrix full of ones.",
       "Un vecteur de `rows` uns, ou avec `columns`, une matrice remplie de uns.",
       "ones(2, 3)                 # [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]"),
    b!("identity_matrix", "identity_matrix(n)", 1, 1, Ml, identity_matrix,
       "The identity matrix of n rows and n columns: ones on the diagonal, zeros elsewhere.",
       "La matrice identité de n lignes et n colonnes : des uns sur la diagonale, des zéros ailleurs.",
       "identity_matrix(2)         # [[1.0, 0.0], [0.0, 1.0]]"),
    b!("shape", "shape(x)", 1, 1, Ml, shape,
       "The shape of a vector or a matrix (of lists nested in lists): its length at each level, `[3]` for a vector of 3 numbers, `[2, 3]` for 2 rows of 3 numbers.",
       "La forme d'un vecteur ou d'une matrice (de listes imbriquées dans des listes) : sa longueur à chaque niveau, `[3]` pour un vecteur de 3 nombres, `[2, 3]` pour 2 lignes de 3 nombres.",
       "shape([1, 2, 3])           # [3]\nshape([[1, 2, 3], [4, 5, 6]])   # [2, 3]"),
    b!("stats", "stats(xs)", 1, 1, Ml, stats,
       "A summary of a list of numbers, as a record: `count`, `mean` (the average), `median` (the middle value), `std_dev`, `min` and `max`. `stats(xs).mean` is the mean.",
       "Un résumé d'une liste de nombres, sous forme d'enregistrement : `count`, `mean` (la moyenne), `median` (la valeur du milieu), `std_dev`, `min` et `max`. `stats(xs).mean` est la moyenne.",
       "stats([1, 2, 3, 4])         # {count: 4, mean: 2.5, median: 2.5, std_dev: 1.118033988749895, min: 1.0, max: 4.0}\nstats([3, 1, 2]).median     # 2.0"),
    b!("variance", "variance(xs)", 1, 1, Ml, variance,
       "The variance of a list of numbers: the mean of the squares of their distances to their mean (of the whole population: divided by n).",
       "La variance d'une liste de nombres : la moyenne des carrés de leurs écarts à leur moyenne (de toute la population : divisée par n).",
       "variance([2, 4, 4, 4, 5, 5, 7, 9])   # 4.0"),
    b!("std_dev", "std_dev(xs)", 1, 1, Ml, std_dev,
       "The standard deviation of a list of numbers: the square root of their variance, how far they usually are from their mean.",
       "L'écart type d'une liste de nombres : la racine carrée de leur variance, la distance habituelle à leur moyenne.",
       "std_dev([2, 4, 4, 4, 5, 5, 7, 9])    # 2.0"),
    b!("percentile", "percentile(xs, p)", 2, 2, Ml, percentile,
       "The number below which p percent of the numbers of the list are (p from 0 to 100), between the two closest ones when needed. `percentile(xs, 50)` is the median, the middle value.",
       "Le nombre sous lequel se trouvent p pour cent des nombres de la liste (p de 0 à 100), entre les deux plus proches si besoin. `percentile(xs, 50)` est la médiane, la valeur du milieu.",
       "percentile([3, 1, 2], 50)  # 2.0\npercentile([1, 2, 3, 4], 50)   # 2.5\npercentile([1, 2, 3, 4], 100)  # 4.0"),
    b!("argmax", "argmax(xs)", 1, 1, Ml, argmax,
       "The position of the largest number of a list (the first one if several are equal).",
       "La position du plus grand nombre d'une liste (le premier si plusieurs sont égaux).",
       "argmax([3, 9, 2, 9])       # 1"),
    b!("argmin", "argmin(xs)", 1, 1, Ml, argmin,
       "The position of the smallest number of a list (the first one if several are equal).",
       "La position du plus petit nombre d'une liste (le premier si plusieurs sont égaux).",
       "argmin([3, 9, 2, 9])       # 2"),
    b!("softmax", "softmax(xs)", 1, 1, Ml, softmax,
       "Turns a list of scores into probabilities: each one becomes e^x divided by the sum of them all, so that they are between 0 and 1 and add up to 1, the largest score keeping the largest share.",
       "Change une liste de scores en probabilités : chacun devient e^x divisé par la somme de tous, de sorte qu'ils soient entre 0 et 1 et fassent 1 au total, le plus grand score gardant la plus grande part.",
       "softmax([0, 0])            # [0.5, 0.5]\nsoftmax([1, 2, 3])         # [0.09003057317038046, 0.24472847105479764, 0.6652409557748218]"),
    b!("normalize", "normalize(xs)", 1, 1, Ml, normalize,
       "Rescales a list of numbers from 0.0 (the smallest) to 1.0 (the largest), keeping their proportions. When they are all equal, gives zeros.",
       "Ramène une liste de nombres entre 0.0 (le plus petit) et 1.0 (le plus grand), en gardant leurs proportions. Quand ils sont tous égaux, donne des zéros.",
       "normalize([10, 15, 20])    # [0.0, 0.5, 1.0]"),
    b!("random_normal", "random_normal(mean, sd)", 2, 2, Ml, random_normal,
       "A random decimal number drawn from the normal distribution (the bell curve) of this mean and standard deviation. Follows `random_seed`.",
       "Un nombre décimal au hasard, tiré selon la loi normale (la courbe en cloche) de cette moyenne et de cet écart type. Suit `random_seed`.",
       "random_seed(1)\nrandom_normal(0, 1)        # ..."),
    b!("random_choice_weighted", "random_choice_weighted(xs, weights)", 2, 2, Ml, random_choice_weighted,
       "An element of the list chosen at random, each one with a chance in proportion to its weight (numbers of 0 or more, one per element). Follows `random_seed`.",
       "Un élément de la liste choisi au hasard, chacun avec une chance proportionnelle à son poids (des nombres positifs ou nuls, un par élément). Suit `random_seed`.",
       "random_choice_weighted([\"a\", \"b\"], [0, 1])   # \"b\""),
    b!("train_test_split", "train_test_split(xs, share)", 2, 2, Ml, train_test_split,
       "Shuffles a list, then cuts it in two: a record `{train, test}`, where `train` holds this share of the elements (0.8: 80 percent) and `test` the rest. To learn on some examples and check on others. Follows `random_seed`.",
       "Mélange une liste, puis la coupe en deux : un enregistrement `{train, test}`, où `train` contient cette part des éléments (0.8 : 80 pour cent) et `test` le reste. Pour apprendre sur certains exemples et vérifier sur d'autres. Suit `random_seed`.",
       "train_test_split([1, 2, 3, 4, 5], 0.8).train |> len   # 4"),
];

// ----- argument helpers --------------------------------------------------

fn usage(f: &str) -> String {
    match lookup(f) {
        Some(b) => t!("usage: `{}`", "utilisation : `{}`", b.sig),
        None => String::new(),
    }
}

/// What a built-in function expects as one of its arguments: the kinds of
/// values it accepts, and how messages name them. The checker uses the same
/// descriptions (see [`crate::types`]) to report a wrong argument before
/// running, with the message of [`wrong_argument`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Expected {
    List,
    Text,
    Int,
    Number,
    Function,
    Record,
    Bool,
    ListOrText,
    TextOrList,
    /// What has a length: a text, a list, a record, a dictionary or a set.
    Collection,
    /// What `get` looks into: a list, a text, a record or a dictionary.
    Indexable,
    RecordOrDict,
    RecordDictOrSet,
    ListDictOrSet,
    ListOrSet,
    DictOrSet,
    Dict,
    Set,
    /// What `dict` is made from: `[key, value]` pairs, or a record.
    Pairs,
    /// `min(list)`, or several values.
    ListOrValues,
    /// What `int` converts: a number, a text, or `true` and `false`.
    ToInt,
    /// What `float` converts: a number or a text.
    ToFloat,
    ListOptionOrResult,
    OptionOrResult,
    Option,
    Result,
    /// A record `{year, month, day}`, with a time if needed.
    Date,
    /// A generator of values for `check_all`: a record made by `ints()`...
    Generator,
    /// The replacement of `replace_all`.
    TextOrFunction,
}

impl Expected {
    /// The kinds of values accepted.
    pub fn kinds(self) -> Kinds {
        match self {
            Expected::List | Expected::ListOrValues => LIST,
            Expected::Text => TEXT,
            Expected::Int => INT,
            Expected::Number => NUMBER,
            Expected::Function => FUNC,
            Expected::Record => RECORD,
            Expected::Bool => BOOL,
            Expected::ListOrText | Expected::TextOrList => LIST | TEXT,
            Expected::Collection => TEXT | LIST | RECORD | DICT | SET,
            Expected::Indexable => TEXT | LIST | RECORD | DICT,
            Expected::RecordOrDict => RECORD | DICT,
            Expected::RecordDictOrSet => RECORD | DICT | SET,
            Expected::ListDictOrSet => LIST | DICT | SET,
            Expected::ListOrSet => LIST | SET,
            Expected::DictOrSet => DICT | SET,
            Expected::Dict => DICT,
            Expected::Set => SET,
            Expected::Pairs => LIST | RECORD | DICT,
            Expected::ToInt => NUMBER | TEXT | BOOL,
            Expected::ToFloat => NUMBER | TEXT,
            Expected::ListOptionOrResult => LIST | SET | OPTION | RESULT,
            Expected::OptionOrResult => OPTION | RESULT,
            Expected::Option => OPTION,
            Expected::Result => RESULT,
            Expected::Date | Expected::Generator => RECORD,
            Expected::TextOrFunction => TEXT | FUNC,
        }
    }
}

impl std::fmt::Display for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Expected::List => t!("a list", "une liste"),
            Expected::Text => t!("a text", "un texte"),
            Expected::Int => t!("a whole number (Int)", "un nombre entier (Int)"),
            Expected::Number => t!("a number", "un nombre"),
            Expected::Function => t!("a function", "une fonction"),
            Expected::Record => t!("a record", "un enregistrement"),
            Expected::Bool => t!("true or false", "true ou false"),
            Expected::ListOrText => t!("a list or a text", "une liste ou un texte"),
            Expected::TextOrList => t!("a text or a list", "un texte ou une liste"),
            Expected::Collection => {
                t!("a text, a list, a record, a dict or a set", "un texte, une liste, un enregistrement, un dictionnaire ou un ensemble")
            }
            Expected::Indexable => t!("a list, a text, a record or a dict", "une liste, un texte, un enregistrement ou un dictionnaire"),
            Expected::RecordOrDict => t!("a record or a dict", "un enregistrement ou un dictionnaire"),
            Expected::RecordDictOrSet => t!("a record, a dict or a set", "un enregistrement, un dictionnaire ou un ensemble"),
            Expected::ListDictOrSet => t!("a list, a dict or a set", "une liste, un dictionnaire ou un ensemble"),
            Expected::ListOrSet => t!("a list or a set", "une liste ou un ensemble"),
            Expected::DictOrSet => t!("a dict or a set", "un dictionnaire ou un ensemble"),
            Expected::Dict => t!("a dict", "un dictionnaire"),
            Expected::Set => t!("a set", "un ensemble"),
            Expected::Pairs => t!("a list of [key, value] pairs (or a record)", "une liste de paires [clé, valeur] (ou un enregistrement)"),
            Expected::ListOrValues => t!("a list (or several values)", "une liste (ou plusieurs valeurs)"),
            Expected::ToInt | Expected::ToFloat => t!("a number or a text", "un nombre ou un texte"),
            Expected::ListOptionOrResult => t!("a list, a set, an Option or a Result", "une liste, un ensemble, une Option ou un Result"),
            Expected::OptionOrResult => t!("an Option or a Result", "une Option ou un Result"),
            Expected::Option => t!("an Option", "une Option"),
            Expected::Result => t!("a Result", "un Result"),
            Expected::Date => t!("a date (a record {{year, month, day}})", "une date (un enregistrement {{year, month, day}})"),
            Expected::Generator => t!("a generator (like `ints()`)", "un générateur (comme `ints()`)"),
            Expected::TextOrFunction => t!("a text or a function", "un texte ou une fonction"),
        })
    }
}

/// The error for argument `i` of `f`, of a kind it does not accept. `got`
/// shows the value (`Int 5`), and `kinds` is what it surely is: the hints
/// for the usual mistakes depend on it. The run time and the checker both
/// make this error with this function, so that it is the same before running.
pub fn wrong_argument(f: &str, i: usize, expected: Expected, got: &str, kinds: Kinds, span: Span) -> Box<Diagnostic> {
    let pos = i + 1;
    let mut d = Diagnostic::error(
        "E005",
        t!("`{f}` expects {expected} as argument {pos}, but got {got}", "`{f}` attend {expected} comme argument {pos}, mais a reçu {got}"),
    )
    .at(span)
    .label(t!("wrong kind of value", "mauvaise sorte de valeur"))
    .help(usage(f));
    let is = |k: Kinds| kinds != 0 && kinds & !k == 0;
    // How to get a list out of a text or a record.
    if matches!(expected, Expected::List | Expected::ListOptionOrResult | Expected::ListDictOrSet | Expected::ListOrSet) {
        if is(TEXT) {
            d = d.help(t!(
                "a text is not a list: `chars(text)` gives the list of its characters",
                "un texte n'est pas une liste : `chars(texte)` donne la liste de ses caractères"
            ));
        } else if is(RECORD) {
            d = d.help(t!(
                "a record is not a list: `entries(record)`, `keys(record)` or `values(record)` give one",
                "un enregistrement n'est pas une liste : `entries(enreg)`, `keys(enreg)` ou `values(enreg)` en donnent une"
            ));
        }
    }
    // How to get a list out of a dictionary or a set.
    if expected.kinds() & LIST != 0 && expected.kinds() & (DICT | SET) == 0 {
        if is(DICT) {
            d = d.help(t!(
                "a dict is not a list: `entries(dict)`, `keys(dict)` or `values(dict)` give one",
                "un dictionnaire n'est pas une liste : `entries(dico)`, `keys(dico)` ou `values(dico)` en donnent une"
            ));
        } else if is(SET) {
            d = d.help(t!(
                "a set is not a list: `values(set)` gives the list of its elements, in order",
                "un ensemble n'est pas une liste : `values(ensemble)` donne la liste de ses éléments, dans l'ordre"
            ));
        }
    }
    if f == "map" && i == 0 && is(DICT) {
        d = d.help(t!(
            "to change the values of a dict, use `map_values(dict, f)`",
            "pour changer les valeurs d'un dictionnaire, utilisez `map_values(dico, f)`"
        ));
    }
    // The record functions that have a cousin for lists.
    if matches!(expected, Expected::Record | Expected::RecordOrDict | Expected::RecordDictOrSet) && is(LIST) {
        let cousin = match f {
            "put" => "set_at(list, i, x)",
            "remove" => "remove_at(list, i)",
            "has" => "contains(list, x)",
            _ => "",
        };
        if !cousin.is_empty() {
            d = d.help(t!("for a list, use `{cousin}`", "pour une liste, utilisez `{cousin}`"));
        }
    }
    if expected == Expected::Date && is(TEXT) {
        d = d.help(t!(
            "to read a date from a text like \"2024-02-29\", use `parse_date(text)`",
            "pour lire une date dans un texte comme \"2024-02-29\", utilisez `parse_date(texte)`"
        ));
    }
    if f == "find_first" && i == 0 && is(LIST) {
        d = d.help(t!("for a list, use `find(list, f)`", "pour une liste, utilisez `find(liste, f)`"));
    }
    if f == "flat_map" && i == 0 && is(OPTION | RESULT) {
        d = d.help(t!("for an Option or a Result, use `and_then(x, f)`", "pour une Option ou un Result, utilisez `and_then(x, f)`"));
    }
    d.boxed()
}

fn wrong(f: &str, i: usize, expected: Expected, got: &Value, span: Span) -> Box<Diagnostic> {
    wrong_argument(f, i, expected, &got.describe(), types::kind_of(got), span)
}

/// The error for a function given to `f` that gives back a value of the
/// wrong kind: `wanted` is what `f` needs (`BOOL`, `LIST`, `OPTION` or
/// `RESULT`), `got` shows what it gave. Made the same way by the checker.
pub fn wrong_return(f: &str, wanted: Kinds, got: &str, span: Span) -> Box<Diagnostic> {
    match wanted {
        LIST => invalid(
            t!(
                "the function given to `{f}` must return a list, but it returned {got}",
                "la fonction donnée à `{f}` doit renvoyer une liste, mais elle a renvoyé {got}"
            ),
            span,
        ),
        TEXT => Diagnostic::error(
            "E005",
            t!(
                "the function given to `{f}` must return a text, but it returned {got}",
                "la fonction donnée à `{f}` doit renvoyer un texte, mais elle a renvoyé {got}"
            ),
        )
        .at(span)
        .help(t!("to make a text of any value, use `text(x)`", "pour faire un texte de n'importe quelle valeur, utilisez `text(x)`"))
        .boxed(),
        OPTION | RESULT => {
            let expected = if wanted == OPTION {
                t!("an Option (Some or None)", "une Option (Some ou None)")
            } else {
                t!("a Result (Ok or Err)", "un Result (Ok ou Err)")
            };
            Diagnostic::error(
                "E005",
                t!(
                    "the function given to `{f}` must return {expected}, but it returned {got}",
                    "la fonction donnée à `{f}` doit renvoyer {expected}, mais elle a renvoyé {got}"
                ),
            )
            .at(span)
            .help(t!(
                "to change the value inside without another check, use `map(x, f)`",
                "pour changer la valeur à l'intérieur sans autre vérification, utilisez `map(x, f)`"
            ))
            .boxed()
        }
        _ => Diagnostic::error(
            "E005",
            t!(
                "the function given to `{f}` must return true or false, but it returned {got}",
                "la fonction donnée à `{f}` doit renvoyer true ou false, mais elle a renvoyé {got}"
            ),
        )
        .at(span)
        .help(usage(f))
        .boxed(),
    }
}

fn list<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<&'a List> {
    match &a[i] {
        Value::List(l) => Ok(l),
        v => Err(wrong(f, i, Expected::List, v, span)),
    }
}

fn text<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<&'a str> {
    match &a[i] {
        Value::Text(s) => Ok(s),
        v => Err(wrong(f, i, Expected::Text, v, span)),
    }
}

fn int(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<i64> {
    match &a[i] {
        Value::Int(n) => Ok(*n),
        Value::Big(n) => {
            Err(invalid(t!("the number {n} is too large for `{f}`", "le nombre {n} est trop grand pour `{f}`", n = short_number(n)), span))
        }
        v => Err(wrong(f, i, Expected::Int, v, span)),
    }
}

/// A large number in a message: its first and last digits.
fn short_number(n: &BigInt) -> String {
    let s = n.to_string();
    let digits = s.trim_start_matches('-').len();
    if digits <= 40 {
        return s;
    }
    let sign = n.is_negative() as usize;
    let lots = t!("{digits} digits", "{digits} chiffres");
    format!("{}...{} ({lots})", &s[..sign + 10], &s[s.len() - 10..])
}

/// A whole number of any size.
fn whole<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<std::borrow::Cow<'a, BigInt>> {
    bigint::big(&a[i]).ok_or_else(|| wrong(f, i, Expected::Int, &a[i], span))
}

fn number(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<f64> {
    match &a[i] {
        Value::Int(n) => Ok(*n as f64),
        Value::Big(n) => Ok(n.to_f64()),
        Value::Float(x) => Ok(*x),
        v => Err(wrong(f, i, Expected::Number, v, span)),
    }
}

fn func(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Value> {
    if a[i].is_callable() {
        Ok(a[i].clone())
    } else {
        Err(wrong(f, i, Expected::Function, &a[i], span))
    }
}

fn record<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<&'a Rc<Record>> {
    match &a[i] {
        Value::Record(r) => Ok(r),
        v => Err(wrong(f, i, Expected::Record, v, span)),
    }
}

fn count_arg(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<usize> {
    let n = int(a, i, f, span)?;
    usize::try_from(n)
        .map_err(|_| invalid(t!("`{f}` needs a number of 0 or more, not {n}", "`{f}` a besoin d'un nombre positif ou nul, pas {n}"), span))
}

/// A size for `step_by`, `chunks` and `windows`: 1 or more.
fn size_arg(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<usize> {
    match count_arg(a, i, f, span)? {
        0 => Err(invalid(t!("`{f}` needs a number of 1 or more, not 0", "`{f}` a besoin d'un nombre supérieur ou égal à 1, pas 0"), span)),
        n => Ok(n),
    }
}

/// A position in a list (negative = from the end), which must exist.
fn position(a: &[Value], i: usize, len: usize, f: &str, span: Span) -> Fallible<usize> {
    let n = int(a, i, f, span)?;
    resolve_index(n, len).ok_or_else(|| out_of_range(n, len, len.checked_sub(1), span))
}

fn out_of_range(n: i64, len: usize, last: Option<usize>, span: Span) -> Box<Diagnostic> {
    let what = if len == 0 {
        t!("the list is empty", "la liste est vide")
    } else {
        t!("the list has {len} element(s)", "la liste a {len} élément(s)")
    };
    let d = Diagnostic::error("E007", t!("position {n} is out of range: {what}", "la position {n} est hors limites : {what}"))
        .at(span)
        .label(t!("no element here", "aucun élément ici"));
    match last {
        Some(last) => d
            .help(t!(
                "valid positions go from 0 to {last} (or from -1 backwards, counting from the end)",
                "les positions valides vont de 0 à {last} (ou de -1 en reculant, en comptant depuis la fin)"
            ))
            .boxed(),
        None => d.boxed(),
    }
}

fn invalid(msg: String, span: Span) -> Box<Diagnostic> {
    Diagnostic::error("E023", msg).at(span).boxed()
}

/// The error for a function given to `f` that does not take the `given`
/// arguments that `f` gives it: it takes `min` to `max`. Made the same way by
/// the checker.
pub fn wrong_arity(f: &str, given: usize, min: usize, max: usize, span: Span) -> Box<Diagnostic> {
    let expected = if min == max {
        min.to_string()
    } else if max == MANY {
        t!("at least {min}", "au moins {min}")
    } else {
        t!("{min} to {max}", "de {min} à {max}")
    };
    Diagnostic::error(
        "E004",
        t!(
            "`{f}` gives {given} argument(s) to this function, but it takes {expected}",
            "`{f}` donne {given} argument(s) à cette fonction, mais elle en attend {expected}"
        ),
    )
    .at(span)
    .label(t!("takes {expected} argument(s)", "attend {expected} argument(s)"))
    .help(usage(f))
    .boxed()
}

/// Checks that the function `g` given to `f` takes the `given` arguments
/// that `f` gives it: otherwise, the mistake is in the call to `f`.
fn given_arity(f: &str, g: &Value, given: usize, span: Span) -> Fallible<()> {
    let (min, max) = match g {
        Value::Func(c) => (c.def.arity, c.def.arity),
        Value::Builtin(b) => (b.min, b.max),
        Value::Ctor(c) => (c.fields.len(), c.fields.len()),
        _ => (0, MANY),
    };
    if given < min || given > max {
        return Err(wrong_arity(f, given, min, max, span));
    }
    Ok(())
}

/// Calls the function `g` given to `f` with one argument (see `given_arity`).
fn call_given1(it: &mut Interp, f: &str, g: &Value, x: Value, span: Span) -> Fallible<Value> {
    given_arity(f, g, 1, span)?;
    it.call1(g, x, span)
}

/// Calls the function `g` given to `f` with two arguments (see `given_arity`).
fn call_given2(it: &mut Interp, f: &str, g: &Value, x: Value, y: Value, span: Span) -> Fallible<Value> {
    given_arity(f, g, 2, span)?;
    it.call2(g, x, y, span)
}

/// Calls a function that must answer true or false.
fn test(it: &mut Interp, f: &Value, x: &Value, name: &str, span: Span) -> Fallible<bool> {
    match call_given1(it, name, f, x.clone(), span)? {
        Value::Bool(b) => Ok(b),
        other => Err(wrong_return(name, BOOL, &other.describe(), span)),
    }
}

fn out(s: &str) {
    crate::io::out_line(s);
}

fn text_value(s: impl Into<Text>) -> Value {
    Value::Text(s.into())
}

fn f64_to_int(x: f64, span: Span) -> Fallible<Value> {
    if x.is_nan() {
        return Err(Diagnostic::error(
            "E022",
            t!(
                "`nan` (not a number) cannot be converted to a whole number",
                "`nan` (pas un nombre) ne peut pas être converti en nombre entier"
            ),
        )
        .at(span)
        .help(t!(
            "`nan` is the result of a calculation that has none, like `0.0 * inf`: check where this value comes from",
            "`nan` est le résultat d'un calcul qui n'en a pas, comme `0.0 * inf` : vérifiez d'où vient cette valeur"
        ))
        .boxed());
    }
    if x.is_infinite() {
        return Err(Diagnostic::error(
            "E022",
            t!(
                "`{}` (infinity) cannot be converted to a whole number",
                "`{}` (l'infini) ne peut pas être converti en nombre entier",
                fmt_float(x)
            ),
        )
        .at(span)
        .help(t!(
            "an infinity comes from a calculation too large for decimal numbers, like `1e308 * 10.0`, or from a division by `0.0`",
            "un infini vient d'un calcul trop grand pour les nombres décimaux, comme `1e308 * 10.0`, ou d'une division par `0.0`"
        ))
        .boxed());
    }
    Ok(bigint::from_whole_f64(x))
}

// ----- Basics ------------------------------------------------------------

fn print(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    out(&joined(a, " ", usize::MAX).unwrap_or_default());
    Ok(Value::Unit)
}

/// The values as `display` shows them, with `sep` between them; `None` as
/// soon as they take more than `max` bytes, counting a `sep` after each one.
fn joined(values: &[Value], sep: &str, max: usize) -> Option<String> {
    let mut s = String::new();
    for (i, v) in values.iter().enumerate() {
        if i > 0 {
            s.push_str(sep);
        }
        v.write_display(&mut s);
        if s.len() + sep.len() > max {
            return None;
        }
    }
    Some(s)
}

fn debug(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    crate::io::err(&format!("[debug {}] {}\n", source::location(span), a[0].repr()));
    Ok(a[0].clone())
}

fn to_text(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if a.len() == 2 {
        let digits = count_arg(a, 1, "text", span)?;
        if digits > 100 {
            return Err(invalid(
                t!("`text` writes at most 100 decimals, not {digits}", "`text` écrit au plus 100 décimales, pas {digits}"),
                span,
            ));
        }
        return Ok(text_value(match &a[0] {
            Value::Int(n) if digits == 0 => n.to_string(),
            Value::Int(n) => format!("{n}.{}", "0".repeat(digits)),
            Value::Big(n) if digits == 0 => n.to_string(),
            Value::Big(n) => format!("{n}.{}", "0".repeat(digits)),
            Value::Float(x) => fixed(*x, digits),
            v => return Err(wrong("text", 0, Expected::Number, v, span)),
        }));
    }
    Ok(match &a[0] {
        Value::Text(_) => a[0].clone(),
        v => text_value(v.display()),
    })
}

/// A decimal number with exactly `digits` decimals, rounded like `round` does
/// (halves away from zero).
fn fixed(x: f64, digits: usize) -> String {
    if !x.is_finite() {
        return fmt_float(x);
    }
    let p = 10f64.powi(digits as i32);
    let r = if digits <= 15 && x.fract() != 0.0 && (x * p).is_finite() { (x * p).round() / p } else { x };
    let s = format!("{r:.digits$}");
    // -0.001 written with 2 decimals is 0.00, not -0.00.
    match s.strip_prefix('-') {
        Some(rest) if rest.chars().all(|c| c == '0' || c == '.') => rest.to_string(),
        _ => s,
    }
}

fn input(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    use std::io::BufRead;
    if let Some(p) = a.first() {
        crate::io::out(&p.display());
    }
    // The web playground has no keyboard input: it reads an empty text.
    if cfg!(target_arch = "wasm32") {
        return Ok(text_value(""));
    }
    let mut line = Vec::new();
    // At the end of the input, an empty text would let a program that asks
    // again and again run forever.
    if crate::io::input_closed() || !matches!(std::io::stdin().lock().read_until(b'\n', &mut line), Ok(n) if n > 0) {
        if !a.is_empty() {
            crate::io::out("\n");
        }
        return Err(Diagnostic::error("E017", t!("`input` found the end of the input", "`input` a atteint la fin de l'entrée"))
            .at(span)
            .label(t!("no more lines to read", "plus aucune ligne à lire"))
            .note(t!(
                "the keyboard input was closed (Ctrl-D), or the text sent to the program has ended",
                "la saisie au clavier a été fermée (Ctrl-D), ou le texte envoyé au programme est terminé"
            ))
            .boxed());
    }
    Ok(text_value(String::from_utf8_lossy(&line).trim_end_matches(['\n', '\r'])))
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
        v => Err(wrong("assert", 0, Expected::Bool, v, span)),
    }
}

fn assert_eq(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if a[0].equals(&a[1]) {
        return Ok(Value::Unit);
    }
    let (got, expected) = (a[0].repr(), a[1].repr());
    let mut values = t!("got:      {got}\nexpected: {expected}", "obtenu :  {got}\nattendu : {expected}");
    // Two short texts: a mark under the first character that differs.
    if let (Value::Text(x), Value::Text(y)) = (&a[0], &a[1]) {
        if got.chars().count() <= 100 && expected.chars().count() <= 100 {
            let same: String = x.chars().zip(y.chars()).take_while(|(c, d)| c == d).map(|(c, _)| c).collect();
            let column = Value::text(&same).repr().chars().count() - 1;
            let at = same.chars().count() + 1;
            values.push_str(&format!(
                "\n          {}^ {}",
                " ".repeat(column),
                t!("first difference: character {at}", "première différence : caractère {at}")
            ));
        }
    }
    let mut d =
        Diagnostic::error("E015", t!("assertion failed: the values are different", "assertion échouée : les valeurs sont différentes"))
            .at(span)
            .note(values);
    for note in differences(&a[0], &a[1]) {
        d = d.note(note);
    }
    Err(d.boxed())
}

/// Where two different values differ, for `assert_eq`: the first place
/// inside lists and records where they are not the same, and what is there.
fn differences(got: &Value, expected: &Value) -> Vec<String> {
    let mut notes = Vec::new();
    let (mut x, mut y) = (got, expected);
    let mut path = String::new();
    loop {
        let at = if path.is_empty() { String::new() } else { t!(" at {path}", " en {path}") };
        match (x, y) {
            (Value::List(l), Value::List(m)) => {
                if l.len() != m.len() {
                    notes.push(t!(
                        "the list{at} has {} element(s), {} expected",
                        "la liste{at} a {} élément(s), {} attendu(s)",
                        l.len(),
                        m.len()
                    ));
                }
                match l.iter().zip(m.iter()).position(|(a, b)| !a.equals(b)) {
                    Some(i) => {
                        path.push_str(&format!("[{i}]"));
                        (x, y) = (&l.as_slice()[i], &m.as_slice()[i]);
                    }
                    None => {
                        let (more, from, word) = if l.len() > m.len() {
                            (l, m.len(), t!("in excess", "en trop"))
                        } else {
                            (m, l.len(), t!("missing", "manquant(s)"))
                        };
                        let items: Vec<String> = more.iter().skip(from).take(5).map(Value::repr).collect();
                        let dots = if more.len() - from > 5 { ", ..." } else { "" };
                        notes.push(t!("{word} from [{from}]: {}{dots}", "{word} à partir de [{from}] : {}{dots}", items.join(", ")));
                        return notes;
                    }
                }
            }
            (Value::Record(r), Value::Record(s)) => {
                let missing: Vec<&str> = s.keys().iter().filter(|k| r.get(k).is_none()).map(|k| &**k).collect();
                let extra: Vec<&str> = r.keys().iter().filter(|k| s.get(k).is_none()).map(|k| &**k).collect();
                if !missing.is_empty() {
                    notes.push(t!("missing field(s){at}: {}", "champ(s) manquant(s){at} : {}", missing.join(", ")));
                }
                if !extra.is_empty() {
                    notes.push(t!("field(s) not expected{at}: {}", "champ(s) inattendu(s){at} : {}", extra.join(", ")));
                }
                let Some((k, v, w)) = r.iter().find_map(|(k, v)| s.get(k).filter(|w| !v.equals(w)).map(|w| (k, v, w))) else {
                    return notes;
                };
                path.push('.');
                path.push_str(k);
                (x, y) = (v, w);
            }
            _ => break,
        }
    }
    let kinds = if x.type_name() == y.type_name() {
        String::new()
    } else {
        t!(", which are of different types: {} and {}", ", qui sont de types différents : {} et {}", x.type_name(), y.type_name())
    };
    let path = path.strip_prefix('.').unwrap_or(&path);
    if !path.is_empty() {
        notes.insert(
            0,
            t!(
                "first difference at {path}: got {}, expected {}{kinds}",
                "première différence en {path} : obtenu {}, attendu {}{kinds}",
                x.repr(),
                y.repr()
            ),
        );
    } else if !kinds.is_empty() {
        notes.push(t!(
            "the values are of different types: {} and {}",
            "les valeurs sont de types différents : {} et {}",
            x.type_name(),
            y.type_name()
        ));
    }
    // Texts of several lines: the first line that differs.
    if let (Value::Text(s), Value::Text(t)) = (x, y) {
        if s.contains('\n') || t.contains('\n') {
            let (a, b): (Vec<&str>, Vec<&str>) = (s.split('\n').collect(), t.split('\n').collect());
            let i = a.iter().zip(&b).position(|(p, q)| p != q).unwrap_or(a.len().min(b.len()));
            let line = |l: Option<&&str>| l.map(|l| Value::text(l).repr()).unwrap_or_else(|| t!("(no line)", "(pas de ligne)"));
            notes.push(t!(
                "first different line: line {}\n  got:      {}\n  expected: {}",
                "première ligne différente : ligne {}\n  obtenu :  {}\n  attendu : {}",
                i + 1,
                line(a.get(i)),
                line(b.get(i))
            ));
        }
    }
    notes
}

fn fail(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let mut msg = a[0].display();
    if msg.trim().is_empty() {
        msg = t!("the program stopped with `fail`", "le programme s'est arrêté avec `fail`");
    }
    Err(Diagnostic::error("E021", msg).at(span).label(t!("the program stopped here", "le programme s'est arrêté ici")).boxed())
}

fn args(it: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
    Ok(Value::list(it.script_args.iter().map(|s| text_value(s.as_str())).collect()))
}

fn env(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let name = text(a, 0, "env", span)?;
    // The system refuses these names; they are never set.
    if name.is_empty() || name.contains(['=', '\0']) {
        return Ok(it.none());
    }
    Ok(it.option(std::env::var_os(name).map(|v| text_value(v.to_string_lossy()))))
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

fn random_seed(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    it.seed_random(int(a, 0, "random_seed", span)?);
    Ok(Value::Unit)
}

// ----- Numbers -----------------------------------------------------------

fn abs(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Int(n) => Ok(n.checked_abs().map_or_else(|| bigint::value(BigInt::from_i64(*n).abs()), Value::Int)),
        Value::Big(n) => Ok(bigint::value(n.abs())),
        Value::Float(x) => Ok(Value::Float(x.abs())),
        v => Err(wrong("abs", 0, Expected::Number, v, span)),
    }
}

fn extreme(a: &[Value], name: &str, want: Ordering, span: Span) -> Fallible<Value> {
    let items: Vec<Value> = if a.len() == 1 {
        match &a[0] {
            Value::List(l) => l.iter().cloned().collect(),
            v => return Err(wrong(name, 0, Expected::ListOrValues, v, span)),
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
        Value::Int(_) | Value::Big(_) => Ok(a[0].clone()),
        Value::Float(x) => f64_to_int(f(*x), span),
        v => Err(wrong(name, 0, Expected::Number, v, span)),
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
        let scaled = x * p;
        // Whole numbers have no decimals to round (and x * p may become infinite).
        if (d >= 0 && x.fract() == 0.0) || !scaled.is_finite() {
            return Ok(Value::Float(x));
        }
        return Ok(Value::Float(scaled.round() / p));
    }
    rounding(a, "round", span, f64::round)
}

fn to_int(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Int(_) | Value::Big(_) => Ok(a[0].clone()),
        Value::Float(x) => f64_to_int(x.trunc(), span),
        Value::Bool(b) => Ok(Value::Int(*b as i64)),
        Value::Text(s) => {
            let t = s.trim();
            match read_whole(t) {
                Some(Ok(n)) => return Ok(n),
                Some(Err(())) => return Err(bigint::too_large(span)),
                None => {}
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
        v => Err(wrong("int", 0, Expected::ToInt, v, span)),
    }
}

fn to_float(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Int(n) => Ok(Value::Float(*n as f64)),
        Value::Big(n) => Ok(Value::Float(n.to_f64())),
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
        v => Err(wrong("float", 0, Expected::ToFloat, v, span)),
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

/// The number given to `asin` or `acos`, which must be from -1 to 1.
fn unit_interval(a: &[Value], name: &str, span: Span) -> Fallible<f64> {
    let x = number(a, 0, name, span)?;
    if !(-1.0..=1.0).contains(&x) {
        return Err(invalid(t!("`{name}` needs a number from -1 to 1, not {x}", "`{name}` a besoin d'un nombre de -1 à 1, pas {x}"), span));
    }
    Ok(x)
}

fn asin(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(unit_interval(a, "asin", span)?.asin()))
}

fn acos(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(unit_interval(a, "acos", span)?.acos()))
}

fn atan(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let y = number(a, 0, "atan", span)?;
    if a.len() == 2 {
        return Ok(Value::Float(y.atan2(number(a, 1, "atan", span)?)));
    }
    Ok(Value::Float(y.atan()))
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

fn even(a: &[Value], f: &str, span: Span) -> Fallible<bool> {
    match &a[0] {
        Value::Int(n) => Ok(n % 2 == 0),
        Value::Big(n) => Ok(n.is_even()),
        v => Err(wrong(f, 0, Expected::Int, v, span)),
    }
}

fn is_even(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(even(a, "is_even", span)?))
}

fn is_odd(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(!even(a, "is_odd", span)?))
}

/// Euclid's algorithm, on the absolute values (they always fit in a u64).
fn gcd_of(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn gcd(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if let (Value::Int(x), Value::Int(y)) = (&a[0], &a[1]) {
        if let Ok(g) = i64::try_from(gcd_of(x.unsigned_abs(), y.unsigned_abs())) {
            return Ok(Value::Int(g));
        }
    }
    let (x, y) = (whole(a, 0, "gcd", span)?, whole(a, 1, "gcd", span)?);
    Ok(bigint::value(x.gcd(&y)))
}

fn lcm(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if let (Value::Int(x), Value::Int(y)) = (&a[0], &a[1]) {
        if *x == 0 || *y == 0 {
            return Ok(Value::Int(0));
        }
        let (x, y) = (x.unsigned_abs(), y.unsigned_abs());
        if let Some(m) = (x / gcd_of(x, y)).checked_mul(y).and_then(|m| i64::try_from(m).ok()) {
            return Ok(Value::Int(m));
        }
    }
    let (x, y) = (whole(a, 0, "lcm", span)?, whole(a, 1, "lcm", span)?);
    if x.is_zero() || y.is_zero() {
        return Ok(Value::Int(0));
    }
    let (m, _) = x.abs().div_mod_floor(&x.gcd(&y));
    binop(crate::ast::BinOp::Mul, &bigint::value(m), &bigint::value(y.abs()), span)
}

// ----- Text --------------------------------------------------------------

fn len(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Int(match &a[0] {
        Value::Text(s) => s.chars().count() as i64,
        Value::List(l) => l.len() as i64,
        Value::Record(r) => r.len() as i64,
        Value::Dict(m) | Value::Set(m) => m.len() as i64,
        v => return Err(wrong("len", 0, Expected::Collection, v, span)),
    }))
}

fn upper(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "upper", span)?.to_uppercase()))
}

fn lower(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "lower", span)?.to_lowercase()))
}

fn capitalize(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "capitalize", span)?;
    let mut cs = s.chars();
    Ok(match cs.next() {
        Some(c) => text_value(titlecase(c) + cs.as_str()),
        None => a[0].clone(),
    })
}

/// A character written at the start of a word: its uppercase, except where
/// Unicode gives a titlecase that differs (UnicodeData and SpecialCasing).
/// The digraphs `ǆ ǉ ǌ ǳ` become `ǅ ǈ ǋ ǲ`, `ß` becomes `Ss`, `ﬁ` `Fi`, the
/// Greek letters with iota subscript keep it (`ᾳ` gives `ᾼ`), and Georgian
/// letters stay as they are (their uppercase is only for whole words).
pub fn titlecase(c: char) -> String {
    let special = match c {
        '\u{10d0}'..='\u{10fa}' | '\u{10fd}'..='\u{10ff}' => return c.to_string(),
        '\u{1f80}'..='\u{1faf}' => {
            let title = if (c as u32) & 0xf < 8 { c as u32 + 8 } else { c as u32 };
            return char::from_u32(title).map_or_else(|| c.to_string(), String::from);
        }
        '\u{df}' => "Ss",
        '\u{1c4}' => "\u{1c5}",
        '\u{1c5}' => "\u{1c5}",
        '\u{1c6}' => "\u{1c5}",
        '\u{1c7}' => "\u{1c8}",
        '\u{1c8}' => "\u{1c8}",
        '\u{1c9}' => "\u{1c8}",
        '\u{1ca}' => "\u{1cb}",
        '\u{1cb}' => "\u{1cb}",
        '\u{1cc}' => "\u{1cb}",
        '\u{1f1}' => "\u{1f2}",
        '\u{1f2}' => "\u{1f2}",
        '\u{1f3}' => "\u{1f2}",
        '\u{587}' => "\u{535}\u{582}",
        '\u{1fb2}' => "\u{1fba}\u{345}",
        '\u{1fb3}' => "\u{1fbc}",
        '\u{1fb4}' => "\u{386}\u{345}",
        '\u{1fb7}' => "\u{391}\u{342}\u{345}",
        '\u{1fbc}' => "\u{1fbc}",
        '\u{1fc2}' => "\u{1fca}\u{345}",
        '\u{1fc3}' => "\u{1fcc}",
        '\u{1fc4}' => "\u{389}\u{345}",
        '\u{1fc7}' => "\u{397}\u{342}\u{345}",
        '\u{1fcc}' => "\u{1fcc}",
        '\u{1ff2}' => "\u{1ffa}\u{345}",
        '\u{1ff3}' => "\u{1ffc}",
        '\u{1ff4}' => "\u{38f}\u{345}",
        '\u{1ff7}' => "\u{3a9}\u{342}\u{345}",
        '\u{1ffc}' => "\u{1ffc}",
        '\u{fb00}' => "Ff",
        '\u{fb01}' => "Fi",
        '\u{fb02}' => "Fl",
        '\u{fb03}' => "Ffi",
        '\u{fb04}' => "Ffl",
        '\u{fb05}' => "St",
        '\u{fb06}' => "St",
        '\u{fb13}' => "\u{544}\u{576}",
        '\u{fb14}' => "\u{544}\u{565}",
        '\u{fb15}' => "\u{544}\u{56b}",
        '\u{fb16}' => "\u{54e}\u{576}",
        '\u{fb17}' => "\u{544}\u{56d}",
        _ => return c.to_uppercase().collect(),
    };
    special.to_string()
}

fn trim(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(text(a, 0, "trim", span)?.trim()))
}

/// A text of exactly one character.
fn one_char(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<char> {
    let s = text(a, i, f, span)?;
    let mut cs = s.chars();
    match (cs.next(), cs.next()) {
        (Some(c), None) => Ok(c),
        _ => Err(invalid(
            t!(
                "`{f}` needs a single character, but \"{s}\" has {}",
                "`{f}` a besoin d'un seul caractère, mais \"{s}\" en a {}",
                s.chars().count()
            ),
            span,
        )),
    }
}

fn pad(a: &[Value], name: &str, before: bool, span: Span) -> Fallible<Value> {
    let width = count_arg(a, 1, name, span)?;
    let fill = if a.len() == 3 { one_char(a, 2, name, span)? } else { ' ' };
    let s = a[0].display();
    let missing = width.saturating_sub(s.chars().count());
    if missing == 0 {
        return Ok(text_value(s));
    }
    if s.len().saturating_add(missing.saturating_mul(fill.len_utf8())) > MAX_TEXT {
        return Err(too_big(false, span));
    }
    let fill: String = std::iter::repeat_n(fill, missing).collect();
    Ok(text_value(if before { fill + &s } else { s + &fill }))
}

fn pad_left(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    pad(a, "pad_left", true, span)
}

fn pad_right(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    pad(a, "pad_right", false, span)
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
    joined(l.as_slice(), sep, MAX_TEXT).map(text_value).ok_or_else(|| too_big(false, span))
}

fn contains(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(match (&a[0], &a[1]) {
        (Value::Text(s), Value::Text(sub)) => s.contains(&**sub),
        (Value::Text(_), v) => return Err(wrong("contains", 1, Expected::Text, v, span)),
        (Value::List(l), x) => l.iter().any(|v| v.equals(x)),
        (Value::Record(r), Value::Text(k)) => r.get(k).is_some(),
        (Value::Record(_), v) => return Err(wrong("contains", 1, Expected::Text, v, span)),
        (Value::Dict(m) | Value::Set(m), x) => dicts::has(m, x),
        (v, _) => return Err(wrong("contains", 0, Expected::Collection, v, span)),
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
    if new.len() > old.len() && s.len() + s.matches(old).count() * (new.len() - old.len()) > MAX_TEXT {
        return Err(too_big(false, span));
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
    let s = text(a, 0, "chars", span)?;
    if s.len() > MAX_LIST && s.chars().count() > MAX_LIST {
        return Err(too_big(true, span));
    }
    Ok(Value::list(s.chars().map(|c| text_value(c.to_string())).collect()))
}

fn lines(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::list(text(a, 0, "lines", span)?.lines().map(text_value).collect()))
}

fn char_code(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Int(one_char(a, 0, "char_code", span)? as i64))
}

fn to_char(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = int(a, 0, "char", span)?;
    match u32::try_from(n).ok().and_then(char::from_u32) {
        Some(c) => Ok(text_value(c.to_string())),
        None => Err(Diagnostic::error("E023", t!("{n} is not the number of a character", "{n} n'est pas le numéro d'un caractère"))
            .at(span)
            .help(t!(
                "character numbers go from 0 to 1114111, except 55296 to 57343",
                "les numéros de caractères vont de 0 à 1114111, sauf de 55296 à 57343"
            ))
            .boxed()),
    }
}

/// Is the text not empty, with only characters that pass `test`?
fn made_of(a: &[Value], name: &str, span: Span, test: fn(char) -> bool) -> Fallible<Value> {
    let s = text(a, 0, name, span)?;
    Ok(Value::Bool(!s.is_empty() && s.chars().all(test)))
}

fn is_digit(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    made_of(a, "is_digit", span, |c| c.is_ascii_digit())
}

fn is_letter(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    made_of(a, "is_letter", span, char::is_alphabetic)
}

fn is_space(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    made_of(a, "is_space", span, char::is_whitespace)
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
        v => Err(wrong("repeat", 0, Expected::TextOrList, v, span)),
    }
}

fn parse_int(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_int", span)?;
    Ok(match read_whole(s.trim()) {
        Some(Ok(n)) => it.ok(n),
        Some(Err(())) => {
            it.err(text_value(t!("this number has more than {} digits", "ce nombre a plus de {} chiffres", bigint::MAX_DIGITS)))
        }
        None => it.err(text_value(t!("\"{s}\" is not a whole number", "\"{s}\" n'est pas un nombre entier"))),
    })
}

/// A whole number written in a text (`-12`, `+7`, `1_000`), of any size:
/// `Some(Err(()))` when it has too many digits.
fn read_whole(t: &str) -> Option<Result<Value, ()>> {
    if let Ok(n) = t.replace('_', "").parse::<i64>() {
        return Some(Ok(Value::Int(n)));
    }
    BigInt::parse(&t.replace('_', ""), 10).map(|r| r.map(bigint::value))
}

fn parse_float(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_float", span)?;
    Ok(match s.trim().replace('_', "").parse::<f64>() {
        Ok(x) => it.ok(Value::Float(x)),
        Err(_) => it.err(text_value(t!("\"{s}\" is not a number", "\"{s}\" n'est pas un nombre"))),
    })
}

// ----- JSON --------------------------------------------------------------

/// A JSON reader that never recurses: arrays and objects may be nested
/// millions of levels deep.
struct Json<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

/// An array or an object being read.
enum Open {
    List(Vec<Value>),
    /// The fields read so far, and the name of the field being read.
    Record(Record, Rc<str>),
}

impl Json<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_space(&mut self) {
        while let Some(b' ' | b'\t' | b'\n' | b'\r') = self.peek() {
            self.pos += 1;
        }
    }

    fn error(&self, msg: String) -> String {
        let mut end = self.pos.min(self.src.len());
        while !self.src.is_char_boundary(end) {
            end -= 1;
        }
        let before = &self.src[..end];
        let line = before.matches('\n').count() + 1;
        let column = before.rsplit('\n').next().unwrap_or("").chars().count() + 1;
        t!("JSON, line {line}, column {column}: {msg}", "JSON, ligne {line}, colonne {column} : {msg}")
    }

    fn expected(&self, what: &str) -> String {
        match self.src.get(self.pos..).and_then(|rest| rest.chars().next()) {
            Some(c) => self.error(t!("expected {what}, but found `{c}`", "on attendait {what}, mais on trouve `{c}`")),
            None => self.error(t!("expected {what}, but the text ends", "on attendait {what}, mais le texte se termine")),
        }
    }

    fn value(&mut self, it: &Interp) -> Result<Value, String> {
        let mut open: Vec<Open> = Vec::new();
        'values: loop {
            self.skip_space();
            let mut v = match self.peek() {
                Some(b'[') => {
                    self.pos += 1;
                    self.skip_space();
                    if self.peek() != Some(b']') {
                        open.push(Open::List(Vec::new()));
                        continue;
                    }
                    self.pos += 1;
                    Value::list(Vec::new())
                }
                Some(b'{') => {
                    self.pos += 1;
                    self.skip_space();
                    if self.peek() != Some(b'}') {
                        let key = self.key()?;
                        open.push(Open::Record(Record::new(), key));
                        continue;
                    }
                    self.pos += 1;
                    Value::Record(Rc::new(Record::new()))
                }
                Some(b'"') => text_value(self.string()?),
                Some(b'-' | b'0'..=b'9') => self.number()?,
                Some(b't') if self.word("true") => Value::Bool(true),
                Some(b'f') if self.word("false") => Value::Bool(false),
                Some(b'n') if self.word("null") => it.none(),
                _ => return Err(self.expected(&t!("a value", "une valeur"))),
            };
            // A value is complete: put it in the array or the object it belongs
            // to, and close the ones that end right after it.
            loop {
                self.skip_space();
                match open.last_mut() {
                    None if self.pos < self.bytes.len() => return Err(self.expected(&t!("the end of the text", "la fin du texte"))),
                    None => return Ok(v),
                    Some(Open::List(items)) => {
                        items.push(v);
                        match self.peek() {
                            Some(b',') => {
                                self.pos += 1;
                                continue 'values;
                            }
                            Some(b']') => self.pos += 1,
                            _ => return Err(self.expected(&t!("`,` or `]`", "`,` ou `]`"))),
                        }
                    }
                    Some(Open::Record(fields, key)) => {
                        fields.set(key.clone(), v);
                        match self.peek() {
                            Some(b',') => {
                                self.pos += 1;
                                self.skip_space();
                                *key = self.key()?;
                                continue 'values;
                            }
                            Some(b'}') => self.pos += 1,
                            _ => return Err(self.expected(&t!("`,` or `}}`", "`,` ou `}}`"))),
                        }
                    }
                }
                v = match open.pop() {
                    Some(Open::List(items)) => Value::list(items),
                    Some(Open::Record(fields, _)) => Value::Record(Rc::new(fields)),
                    None => return Ok(Value::Unit),
                };
            }
        }
    }

    /// `"name":` before a value in an object.
    fn key(&mut self) -> Result<Rc<str>, String> {
        if self.peek() != Some(b'"') {
            return Err(self.expected(&t!("a name in quotes", "un nom entre guillemets")));
        }
        let key = self.string()?;
        self.skip_space();
        if self.peek() != Some(b':') {
            return Err(self.expected("`:`"));
        }
        self.pos += 1;
        Ok(Rc::from(key))
    }

    fn word(&mut self, word: &str) -> bool {
        let found = self.src[self.pos..].starts_with(word);
        if found {
            self.pos += word.len();
        }
        found
    }

    fn digits(&mut self) -> usize {
        let start = self.pos;
        while self.peek().is_some_and(|b| b.is_ascii_digit()) {
            self.pos += 1;
        }
        self.pos - start
    }

    fn number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        let first = self.peek();
        let n = self.digits();
        let bad = |json: &mut Self| {
            json.pos = start;
            json.error(t!(
                "invalid number: JSON writes numbers like 0, -12, 3.5 or 1e-3",
                "nombre invalide : JSON écrit les nombres comme 0, -12, 3.5 ou 1e-3"
            ))
        };
        // JSON writes no `+` and no useless zero in front: 0, 7, -12.
        if n == 0 || (n > 1 && first == Some(b'0')) {
            return Err(bad(self));
        }
        let mut whole = true;
        if self.peek() == Some(b'.') {
            self.pos += 1;
            whole = false;
            if self.digits() == 0 {
                return Err(bad(self));
            }
        }
        if let Some(b'e' | b'E') = self.peek() {
            self.pos += 1;
            whole = false;
            if let Some(b'+' | b'-') = self.peek() {
                self.pos += 1;
            }
            if self.digits() == 0 {
                return Err(bad(self));
            }
        }
        let s = &self.src[start..self.pos];
        if whole {
            if let Ok(n) = s.parse::<i64>() {
                return Ok(Value::Int(n));
            }
            if let Some(Ok(n)) = BigInt::parse(s, 10) {
                return Ok(bigint::value(n));
            }
        }
        // Also whole numbers with too many digits.
        s.parse::<f64>().map(Value::Float).map_err(|_| bad(self))
    }

    /// A text in quotes, with its escapes (`\n`, `\"`, `é`...).
    fn string(&mut self) -> Result<String, String> {
        self.pos += 1;
        let mut out = String::new();
        loop {
            // Copy the plain characters at once. The bytes where it stops are
            // ASCII, so they are always at the start of a character.
            let start = self.pos;
            while self.peek().is_some_and(|b| b != b'"' && b != b'\\' && b >= 0x20) {
                self.pos += 1;
            }
            out.push_str(&self.src[start..self.pos]);
            let c = match self.peek() {
                Some(b'"') => {
                    self.pos += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    self.pos += 1;
                    match self.peek() {
                        Some(b'"') => '"',
                        Some(b'\\') => '\\',
                        Some(b'/') => '/',
                        Some(b'b') => '\u{8}',
                        Some(b'f') => '\u{c}',
                        Some(b'n') => '\n',
                        Some(b'r') => '\r',
                        Some(b't') => '\t',
                        Some(b'u') => {
                            self.pos += 1;
                            out.push(self.unicode()?);
                            continue;
                        }
                        _ => {
                            return Err(self.error(t!(
                                "invalid escape: after `\\` comes one of \" \\ / b f n r t u",
                                "échappement invalide : après `\\` vient l'un de \" \\ / b f n r t u"
                            )))
                        }
                    }
                }
                Some(_) => {
                    return Err(self.error(t!(
                        "a line break or a tab inside a text is written with `\\`: `\\n`, `\\t`",
                        "un retour à la ligne ou une tabulation dans un texte s'écrit avec `\\` : `\\n`, `\\t`"
                    )))
                }
                None => {
                    return Err(self.error(t!("a text is never closed: `\"` is missing", "un texte n'est jamais fermé : il manque `\"`")))
                }
            };
            self.pos += 1;
            out.push(c);
        }
    }

    fn hex4(&mut self) -> Result<u32, String> {
        match self.src.get(self.pos..self.pos + 4).and_then(|h| u32::from_str_radix(h, 16).ok().filter(|_| !h.starts_with('+'))) {
            Some(n) => {
                self.pos += 4;
                Ok(n)
            }
            None => {
                Err(self.error(t!("`\\u` must be followed by 4 hexadecimal digits", "`\\u` doit être suivi de 4 chiffres hexadécimaux")))
            }
        }
    }

    /// The character of a `\uXXXX` escape: those beyond U+FFFF take two escapes.
    fn unicode(&mut self) -> Result<char, String> {
        let half = || {
            t!(
                "invalid `\\u` escape: half of a character (a surrogate) without its other half",
                "échappement `\\u` invalide : une moitié de caractère (surrogate) sans son autre moitié"
            )
        };
        let high = self.hex4()?;
        let code = if (0xD800..0xDC00).contains(&high) {
            if !self.src[self.pos..].starts_with("\\u") {
                return Err(self.error(half()));
            }
            self.pos += 2;
            let low = self.hex4()?;
            if !(0xDC00..0xE000).contains(&low) {
                return Err(self.error(half()));
            }
            0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00)
        } else {
            high
        };
        char::from_u32(code).ok_or_else(|| self.error(half()))
    }
}

fn parse_json(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_json", span)?;
    // Some editors start their files with an invisible mark (BOM).
    let s = s.strip_prefix('\u{feff}').unwrap_or(s);
    let mut json = Json { src: s, bytes: s.as_bytes(), pos: 0 };
    Ok(match json.value(it) {
        Ok(v) => it.ok(v),
        Err(e) => it.err(text_value(e)),
    })
}

fn json_text(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Why a value cannot be written as JSON.
enum Unwritable {
    /// JSON cannot hold this value.
    Value(Value),
    /// The indentation of values nested very deeply would take too much memory.
    TooLarge,
}

/// Writes a value as JSON, without recursion.
fn write_json(it: &Interp, v: &Value, indent: Option<usize>) -> Result<String, Unwritable> {
    enum Item<'a> {
        Value(&'a Value, usize),
        Key(Rc<str>),
        Raw(&'static str),
        /// A new line, indented for this depth (nothing on a single line).
        Line(usize),
    }
    let comma = if indent.is_some() { "," } else { ", " };
    let mut out = String::new();
    let mut todo = vec![Item::Value(v, 0)];
    while let Some(item) = todo.pop() {
        match item {
            Item::Raw(s) => out.push_str(s),
            Item::Key(k) => {
                json_text(&k, &mut out);
                out.push_str(": ");
            }
            Item::Line(depth) => {
                if let Some(n) = indent {
                    if out.len() + n * depth > 100_000_000 {
                        return Err(Unwritable::TooLarge);
                    }
                    out.push('\n');
                    out.extend(std::iter::repeat_n(' ', n * depth));
                }
            }
            Item::Value(v, depth) => match v {
                Value::Unit => out.push_str("null"),
                Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                Value::Int(n) => out.push_str(&n.to_string()),
                Value::Big(n) => out.push_str(&n.to_string()),
                Value::Float(f) if f.is_finite() => out.push_str(&fmt_float(*f)),
                Value::Text(s) => json_text(s, &mut out),
                // `:ok` is written as its name, as Ruby does.
                Value::Symbol(s) => json_text(&s.name(), &mut out),
                Value::List(l) if l.is_empty() => out.push_str("[]"),
                Value::Record(r) if r.is_empty() => out.push_str("{}"),
                Value::Dict(m) if m.is_empty() => out.push_str("{}"),
                Value::Set(m) if m.is_empty() => out.push_str("[]"),
                // A set is written as the list of its elements, in order.
                Value::Set(m) => {
                    out.push('[');
                    todo.push(Item::Raw("]"));
                    todo.push(Item::Line(depth));
                    for (i, x) in m.keys().collect::<Vec<_>>().into_iter().enumerate().rev() {
                        todo.push(Item::Value(x, depth + 1));
                        todo.push(Item::Line(depth + 1));
                        if i > 0 {
                            todo.push(Item::Raw(comma));
                        }
                    }
                }
                // A dictionary whose keys are all texts is written as an object.
                Value::Dict(m) => {
                    // Or whose keys are all symbols, written as their names.
                    let all = |text: bool| {
                        m.keys().all(|k| matches!(k, Value::Text(_)) == text && matches!(k, Value::Text(_) | Value::Symbol(_)))
                    };
                    if !all(true) && !all(false) {
                        return Err(Unwritable::Value(v.clone()));
                    }
                    out.push('{');
                    todo.push(Item::Raw("}"));
                    todo.push(Item::Line(depth));
                    for (i, (k, x)) in m.iter().collect::<Vec<_>>().into_iter().enumerate().rev() {
                        todo.push(Item::Value(x, depth + 1));
                        match k {
                            Value::Text(k) => todo.push(Item::Key(Rc::from(&**k))),
                            Value::Symbol(k) => todo.push(Item::Key(k.name())),
                            _ => {}
                        }
                        todo.push(Item::Line(depth + 1));
                        if i > 0 {
                            todo.push(Item::Raw(comma));
                        }
                    }
                }
                Value::List(l) => {
                    out.push('[');
                    todo.push(Item::Raw("]"));
                    todo.push(Item::Line(depth));
                    for (i, x) in l.iter().enumerate().rev() {
                        todo.push(Item::Value(x, depth + 1));
                        todo.push(Item::Line(depth + 1));
                        if i > 0 {
                            todo.push(Item::Raw(comma));
                        }
                    }
                }
                Value::Record(r) => {
                    out.push('{');
                    todo.push(Item::Raw("}"));
                    todo.push(Item::Line(depth));
                    let fields: Vec<(&Rc<str>, &Value)> = r.iter().collect();
                    for (i, (k, x)) in fields.into_iter().enumerate().rev() {
                        todo.push(Item::Value(x, depth + 1));
                        todo.push(Item::Key(k.clone()));
                        todo.push(Item::Line(depth + 1));
                        if i > 0 {
                            todo.push(Item::Raw(comma));
                        }
                    }
                }
                Value::Variant(var) => match it.wrap(v) {
                    Wrap::None => out.push_str("null"),
                    Wrap::Some => todo.push(Item::Value(&var.fields[0], depth)),
                    _ => return Err(Unwritable::Value(v.clone())),
                },
                other => return Err(Unwritable::Value(other.clone())),
            },
        }
    }
    Ok(out)
}

fn to_json(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let indent = if a.len() == 2 {
        let n = count_arg(a, 1, "to_json", span)?;
        if n > 16 {
            return Err(invalid(
                t!("`to_json` indents by 16 spaces at most, not {n}", "`to_json` décale de 16 espaces au plus, pas {n}"),
                span,
            ));
        }
        Some(n)
    } else {
        None
    };
    write_json(it, &a[0], indent).map(text_value).map_err(|why| {
        let bad = match why {
            Unwritable::Value(v) => v,
            Unwritable::TooLarge => {
                return Diagnostic::error("E023", t!("the JSON text would be too large", "le texte JSON serait trop grand"))
                    .at(span)
                    .help(t!(
                        "the values are nested too deeply to be indented: write them on one line, with `to_json(value)`",
                        "les valeurs sont imbriquées trop profondément pour être décalées : écrivez-les sur une ligne, avec `to_json(valeur)`"
                    ))
                    .boxed()
            }
        };
        let d = match bad {
            Value::Float(_) => Diagnostic::error(
                "E023",
                t!("`to_json` cannot write {} in JSON", "`to_json` ne peut pas écrire {} en JSON", bad.describe()),
            )
            .help(t!("JSON has no infinite numbers and no NaN", "JSON n'a ni nombres infinis ni NaN")),
            Value::Dict(_) => Diagnostic::error(
                "E023",
                t!("`to_json` cannot write {} in JSON", "`to_json` ne peut pas écrire {} en JSON", bad.describe()),
            )
            .help(t!(
                "a dict is written as a JSON object only when its keys are all texts, or all symbols:\nturn the keys into texts first, or write `entries(dict)`",
                "un dictionnaire s'écrit comme un objet JSON seulement quand ses clés sont toutes des textes, ou toutes des symboles :\ntransformez d'abord les clés en textes, ou écrivez `entries(dico)`"
            )),
            _ => Diagnostic::error("E005", t!("`to_json` cannot write {} in JSON", "`to_json` ne peut pas écrire {} en JSON", bad.describe()))
                .help(t!(
                    "JSON only holds numbers, texts, true and false, lists, records and null (`None`):\nturn your value into a record first, like {{kind: \"Circle\", radius: r}}",
                    "JSON ne contient que des nombres, des textes, true et false, des listes, des enregistrements et null (`None`) :\ntransformez d'abord votre valeur en enregistrement, comme {{sorte: \"Cercle\", rayon: r}}"
                )),
        };
        d.at(span).boxed()
    })
}

// ----- Lists -------------------------------------------------------------

fn map(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = func(a, 1, "map", span)?;
    match &a[0] {
        Value::List(l) => {
            let mut out = Vec::with_capacity(l.len());
            for x in l.iter() {
                out.push(call_given1(it, "map", &f, x.clone(), span)?);
            }
            Ok(Value::list(out))
        }
        Value::Set(m) => dicts::map_set(it, m, &f, span),
        v => match it.wrap(v) {
            Wrap::Some => {
                let r = call_given1(it, "map", &f, Interp::payload(v), span)?;
                Ok(it.some(r))
            }
            Wrap::Ok => {
                let r = call_given1(it, "map", &f, Interp::payload(v), span)?;
                Ok(it.ok(r))
            }
            Wrap::None | Wrap::Err => Ok(v.clone()),
            Wrap::Other => Err(wrong("map", 0, Expected::ListOptionOrResult, v, span)),
        },
    }
}

fn filter(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = match &a[0] {
        Value::List(l) => l,
        Value::Dict(m) => {
            let f = func(a, 1, "filter", span)?;
            return dicts::filter_dict(it, "filter", true, m, &f, span);
        }
        Value::Set(m) => {
            let f = func(a, 1, "filter", span)?;
            return dicts::filter_set(it, "filter", true, m, &f, span);
        }
        v => return Err(wrong("filter", 0, Expected::ListDictOrSet, v, span)),
    };
    let f = func(a, 1, "filter", span)?;
    let mut out = Vec::new();
    for x in l.iter() {
        if test(it, &f, x, "filter", span)? {
            out.push(x.clone());
        }
    }
    Ok(Value::list(out))
}

fn reject(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = match &a[0] {
        Value::List(l) => l,
        Value::Dict(m) => {
            let f = func(a, 1, "reject", span)?;
            return dicts::filter_dict(it, "reject", false, m, &f, span);
        }
        Value::Set(m) => {
            let f = func(a, 1, "reject", span)?;
            return dicts::filter_set(it, "reject", false, m, &f, span);
        }
        v => return Err(wrong("reject", 0, Expected::ListDictOrSet, v, span)),
    };
    let f = func(a, 1, "reject", span)?;
    let mut out = Vec::new();
    for x in l.iter() {
        if !test(it, &f, x, "reject", span)? {
            out.push(x.clone());
        }
    }
    Ok(Value::list(out))
}

fn each_with_index(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "each_with_index", span)?;
    let f = func(a, 1, "each_with_index", span)?;
    for (i, x) in l.iter().enumerate() {
        call_given2(it, "each_with_index", &f, x.clone(), Value::Int(i as i64), span)?;
    }
    Ok(Value::Unit)
}

fn fold(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "fold", span)?;
    let f = func(a, 2, "fold", span)?;
    let mut acc = a[1].clone();
    for x in l.iter() {
        acc = call_given2(it, "fold", &f, acc, x.clone(), span)?;
    }
    Ok(acc)
}

fn scan(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "scan", span)?;
    let f = func(a, 2, "scan", span)?;
    let mut acc = a[1].clone();
    let mut out = Vec::with_capacity(l.len());
    for x in l.iter() {
        acc = call_given2(it, "scan", &f, acc, x.clone(), span)?;
        out.push(acc.clone());
    }
    Ok(Value::list(out))
}

fn each(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = match &a[0] {
        Value::List(l) => l,
        Value::Dict(m) => {
            let f = func(a, 1, "each", span)?;
            return dicts::each_dict(it, m, &f, span);
        }
        Value::Set(m) => {
            let f = func(a, 1, "each", span)?;
            return dicts::each_set(it, m, &f, span);
        }
        v => return Err(wrong("each", 0, Expected::ListDictOrSet, v, span)),
    };
    let f = func(a, 1, "each", span)?;
    for x in l.iter() {
        call_given1(it, "each", &f, x.clone(), span)?;
    }
    Ok(Value::Unit)
}

fn sum(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let mut l = list(a, 0, "sum", span)?.clone();
    if a.len() == 2 {
        let f = func(a, 1, "sum", span)?;
        let mut values = Vec::with_capacity(l.len());
        for x in l.iter() {
            values.push(call_given1(it, "sum", &f, x.clone(), span)?);
        }
        l = List::new(values);
    }
    let mut acc = Value::Int(0);
    for x in l.iter() {
        if !matches!(x, Value::Int(_) | Value::Big(_) | Value::Float(_)) {
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
        if !matches!(x, Value::Int(_) | Value::Big(_) | Value::Float(_)) {
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
    if a.len() == 2 {
        let len = length_of(&a[0], "first", span)?;
        let n = count_arg(a, 1, "first", span)?;
        return Ok(cut(&a[0], 0, n.min(len)));
    }
    Ok(match &a[0] {
        Value::List(l) => it.option(l.get(0).cloned()),
        Value::Text(s) => it.option(s.chars().next().map(|c| text_value(c.to_string()))),
        v => return Err(wrong("first", 0, Expected::ListOrText, v, span)),
    })
}

fn last(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if a.len() == 2 {
        let len = length_of(&a[0], "last", span)?;
        let n = count_arg(a, 1, "last", span)?;
        return Ok(cut(&a[0], len - n.min(len), len));
    }
    Ok(match &a[0] {
        Value::List(l) => it.option(l.as_slice().last().cloned()),
        Value::Text(s) => it.option(s.chars().last().map(|c| text_value(c.to_string()))),
        v => return Err(wrong("last", 0, Expected::ListOrText, v, span)),
    })
}

fn rest(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match &a[0] {
        Value::List(l) => Value::List(l.slice(1, l.len())),
        Value::Text(s) => text_value(s.chars().skip(1).collect::<String>()),
        v => return Err(wrong("rest", 0, Expected::ListOrText, v, span)),
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
        (Value::Dict(m), k) => dicts::get(it, m, k),
        (Value::List(_) | Value::Text(_), v) => return Err(wrong("get", 1, Expected::Int, v, span)),
        (Value::Record(_), v) => return Err(wrong("get", 1, Expected::Text, v, span)),
        (v, _) => return Err(wrong("get", 0, Expected::Indexable, v, span)),
    })
}

fn push(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = match &a[0] {
        Value::List(l) => l,
        Value::Set(m) => return dicts::push(m, &a[1], span),
        v => return Err(wrong("push", 0, Expected::ListOrSet, v, span)),
    };
    if l.len() >= MAX_LIST {
        return Err(too_big(true, span));
    }
    // In the room after the list when it can: pushing again and again is fast.
    Ok(Value::List(l.push(a[1].clone())))
}

fn set_at(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "set_at", span)?;
    let i = position(a, 1, l.len(), "set_at", span)?;
    let mut v = l.as_slice().to_vec();
    v[i] = a[2].clone();
    Ok(Value::list(v))
}

fn insert_at(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "insert_at", span)?;
    // One more place than there are elements: after the last one.
    let n = int(a, 1, "insert_at", span)?;
    let i = resolve_index(n, l.len() + 1).ok_or_else(|| out_of_range(n, l.len(), Some(l.len()), span))?;
    let mut v = Vec::with_capacity(l.len() + 1);
    v.extend(l.as_slice()[..i].iter().cloned());
    v.push(a[2].clone());
    v.extend(l.as_slice()[i..].iter().cloned());
    Ok(Value::list(v))
}

fn remove_at(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "remove_at", span)?;
    let i = position(a, 1, l.len(), "remove_at", span)?;
    let mut v = l.as_slice().to_vec();
    v.remove(i);
    Ok(Value::list(v))
}

fn reverse(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(match &a[0] {
        Value::List(l) => Value::list(l.iter().rev().cloned().collect()),
        Value::Text(s) => text_value(s.chars().rev().collect::<String>()),
        v => return Err(wrong("reverse", 0, Expected::ListOrText, v, span)),
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
        let mut d = Diagnostic::error(
            "E005",
            t!("`{name}` cannot compare {} and {}", "`{name}` ne peut pas comparer {} et {}", x.describe(), y.describe()),
        )
        .at(span)
        .help(t!(
            "all the elements must be of the same kind: all numbers, or all texts...",
            "tous les éléments doivent être de même nature : tous des nombres, ou tous des textes..."
        ));
        if let Some(note) = x.homonym_types(&y) {
            d = d.note(note);
        }
        return Err(d.boxed());
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
        pairs.push((call_given1(it, "sort_by", &f, x.clone(), span)?, x.clone()));
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
        Big(Rc<BigInt>),
        Float(u64),
        Text(Text),
        Bool(bool),
        Unit,
    }
    fn key(v: &Value) -> Option<Key> {
        Some(match v {
            Value::Int(n) => Key::Int(*n),
            Value::Big(n) => Key::Big(n.clone()),
            Value::Float(f) if f.is_nan() => return None,
            // Whole decimal numbers have the key of the whole number they are worth.
            Value::Float(f) if f.fract() == 0.0 => match bigint::from_whole_f64(*f) {
                Value::Int(n) => Key::Int(n),
                Value::Big(n) => Key::Big(n),
                _ => unreachable!(),
            },
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

fn shuffle(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let mut v = list(a, 0, "shuffle", span)?.as_slice().to_vec();
    // Fisher-Yates: every order is equally likely.
    for i in (1..v.len()).rev() {
        let j = (it.random_u64() % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
    Ok(Value::list(v))
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
        v => Err(wrong(name, 0, Expected::ListOrText, v, span)),
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

/// How many elements (or characters) at the start make f true.
fn leading(it: &mut Interp, a: &[Value], name: &str, span: Span) -> Fallible<usize> {
    let len = length_of(&a[0], name, span)?;
    let f = func(a, 1, name, span)?;
    match &a[0] {
        Value::List(l) => {
            for (i, x) in l.iter().enumerate() {
                if !test(it, &f, x, name, span)? {
                    return Ok(i);
                }
            }
        }
        Value::Text(s) => {
            for (i, c) in s.chars().enumerate() {
                if !test(it, &f, &text_value(c.to_string()), name, span)? {
                    return Ok(i);
                }
            }
        }
        _ => {}
    }
    Ok(len)
}

fn take_while(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = leading(it, a, "take_while", span)?;
    Ok(cut(&a[0], 0, n))
}

fn drop_while(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = leading(it, a, "drop_while", span)?;
    let len = length_of(&a[0], "drop_while", span)?;
    Ok(cut(&a[0], n, len))
}

fn step_by(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    length_of(&a[0], "step_by", span)?;
    let n = size_arg(a, 1, "step_by", span)?;
    Ok(match &a[0] {
        Value::Text(s) => text_value(s.chars().step_by(n).collect::<String>()),
        Value::List(l) => Value::list(l.iter().step_by(n).cloned().collect()),
        _ => Value::Unit,
    })
}

/// The pieces of a list or text: `count` pieces of `n` elements, `step` apart.
fn pieces(v: &Value, n: usize, step: usize, count: usize) -> Value {
    match v {
        // Pieces of a list share its elements instead of copying them.
        Value::List(l) => Value::list((0..count).map(|k| Value::List(l.slice(k * step, k * step + n))).collect()),
        Value::Text(s) => {
            let cs: Vec<char> = s.chars().collect();
            Value::list((0..count).map(|k| text_value(cs[k * step..(k * step + n).min(cs.len())].iter().collect::<String>())).collect())
        }
        _ => Value::Unit,
    }
}

fn chunks(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let len = length_of(&a[0], "chunks", span)?;
    let n = size_arg(a, 1, "chunks", span)?;
    Ok(pieces(&a[0], n, n, len.div_ceil(n)))
}

fn windows(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let len = length_of(&a[0], "windows", span)?;
    let n = size_arg(a, 1, "windows", span)?;
    let count = (len + 1).saturating_sub(n);
    if matches!(a[0], Value::Text(_)) && count.saturating_mul(n) > 100_000_000 {
        return Err(invalid(t!("the result of `windows` would be too large", "le résultat de `windows` serait trop grand"), span));
    }
    Ok(pieces(&a[0], n, 1, count))
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
    if a.len() == 1 {
        return Ok(Value::Int(l.len() as i64));
    }
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
        (Value::Text(_), v) => return Err(wrong("index_of", 1, Expected::Text, v, span)),
        (v, _) => return Err(wrong("index_of", 0, Expected::ListOrText, v, span)),
    })
}

/// The element whose key is the smallest (`Less`) or the largest (`Greater`);
/// the first one on a tie.
fn extreme_by(it: &mut Interp, a: &[Value], name: &str, want: Ordering, span: Span) -> Fallible<Value> {
    let l = list(a, 0, name, span)?;
    let f = func(a, 1, name, span)?;
    let mut best: Option<(Value, &Value)> = None;
    for x in l.iter() {
        let k = call_given1(it, name, &f, x.clone(), span)?;
        match &best {
            None => best = Some((k, x)),
            Some((bk, _)) => match k.compare(bk) {
                Some(o) if o == want => best = Some((k, x)),
                Some(_) => {}
                None => {
                    return Err(Diagnostic::error(
                        "E005",
                        t!("`{name}` cannot compare {} and {}", "`{name}` ne peut pas comparer {} et {}", k.describe(), bk.describe()),
                    )
                    .at(span)
                    .help(t!(
                        "the function `key` must give values of the same kind for every element: all numbers, or all texts...",
                        "la fonction `key` doit donner des valeurs de même nature pour chaque élément : tous des nombres, ou tous des textes..."
                    ))
                    .boxed())
                }
            },
        }
    }
    match best {
        Some((_, x)) => Ok(x.clone()),
        None => Err(invalid(t!("`{name}` of an empty list", "`{name}` d'une liste vide"), span)),
    }
}

fn min_by(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    extreme_by(it, a, "min_by", Ordering::Less, span)
}

fn max_by(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    extreme_by(it, a, "max_by", Ordering::Greater, span)
}

fn flatten(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "flatten", span)?;
    let mut out = Vec::new();
    for x in l.iter() {
        match x {
            Value::List(inner) if out.len() + inner.len() > MAX_LIST => return Err(too_big(true, span)),
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
        match call_given1(it, "flat_map", &f, x.clone(), span)? {
            Value::List(inner) if out.len() + inner.len() > MAX_LIST => return Err(too_big(true, span)),
            Value::List(inner) => out.extend(inner.iter().cloned()),
            other => return Err(wrong_return("flat_map", LIST, &other.describe(), span)),
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
        let k: Rc<str> = Rc::from(call_given1(it, "group_by", &f, x.clone(), span)?.display());
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
        Value::Dict(m) | Value::Set(m) => m.is_empty(),
        v => return Err(wrong("is_empty", 0, Expected::Collection, v, span)),
    }))
}

// ----- Records -----------------------------------------------------------

fn keys(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Record(r) => Ok(Value::list(r.keys().iter().map(|k| Value::Text(k.into())).collect())),
        Value::Dict(m) => Ok(Value::list(m.keys().cloned().collect())),
        v => Err(wrong("keys", 0, Expected::RecordOrDict, v, span)),
    }
}

fn values(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Record(r) => Ok(Value::list(r.iter().map(|(_, v)| v.clone()).collect())),
        Value::Dict(m) => Ok(Value::list(m.iter().map(|(_, v)| v.clone()).collect())),
        Value::Set(m) => Ok(Value::list(m.keys().cloned().collect())),
        v => Err(wrong("values", 0, Expected::RecordDictOrSet, v, span)),
    }
}

fn entries(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match &a[0] {
        Value::Record(r) => Ok(Value::list(r.iter().map(|(k, v)| Value::list(vec![Value::Text(k.into()), v.clone()])).collect())),
        Value::Dict(m) => Ok(dicts::entries(m)),
        v => Err(wrong("entries", 0, Expected::RecordOrDict, v, span)),
    }
}

fn from_entries(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "from_entries", span)?;
    let mut r = Record::new();
    for pair in l.iter() {
        match pair {
            Value::List(p) if p.len() == 2 && matches!(p.as_slice()[0], Value::Text(_)) => {
                if let Value::Text(k) = &p.as_slice()[0] {
                    r.set(k.into(), p.as_slice()[1].clone());
                }
            }
            other => {
                return Err(Diagnostic::error(
                    "E023",
                    t!(
                        "`from_entries` needs a list of [name, value] pairs, but it contains {}",
                        "`from_entries` a besoin d'une liste de paires [nom, valeur], mais elle contient {}",
                        other.describe()
                    ),
                )
                .at(span)
                .help(t!(
                    "each pair is a list of two elements whose name is a text, like [\"age\", 36]",
                    "chaque paire est une liste de deux éléments dont le nom est un texte, comme [\"age\", 36]"
                ))
                .boxed())
            }
        }
    }
    Ok(Value::Record(Rc::new(r)))
}

fn has(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = match &a[0] {
        Value::Record(r) => r,
        Value::Dict(m) | Value::Set(m) => return Ok(Value::Bool(dicts::has(m, &a[1]))),
        v => return Err(wrong("has", 0, Expected::RecordDictOrSet, v, span)),
    };
    Ok(Value::Bool(r.get(text(a, 1, "has", span)?).is_some()))
}

fn put(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = match &a[0] {
        Value::Record(r) => r,
        Value::Dict(m) => return dicts::put(m, &a[1], &a[2], span),
        v => return Err(wrong("put", 0, Expected::RecordOrDict, v, span)),
    };
    let k = text(a, 1, "put", span)?;
    let mut new = (**r).clone();
    new.set(Rc::from(k), a[2].clone());
    Ok(Value::Record(Rc::new(new)))
}

fn remove(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = match &a[0] {
        Value::Record(r) => r,
        Value::Dict(m) => return Ok(Value::Dict(dicts::remove(m, &a[1]))),
        Value::Set(m) => return Ok(Value::Set(dicts::remove(m, &a[1]))),
        v => return Err(wrong("remove", 0, Expected::RecordDictOrSet, v, span)),
    };
    let k = text(a, 1, "remove", span)?;
    Ok(Value::Record(Rc::new(r.without(k))))
}

fn map_values(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = match &a[0] {
        Value::Record(r) => r,
        Value::Dict(m) => {
            let f = func(a, 1, "map_values", span)?;
            return dicts::map_values(it, m, &f, span);
        }
        v => return Err(wrong("map_values", 0, Expected::RecordOrDict, v, span)),
    };
    let f = func(a, 1, "map_values", span)?;
    let mut out = Record::new();
    for (k, v) in r.iter() {
        out.set(k.clone(), call_given1(it, "map_values", &f, v.clone(), span)?);
    }
    Ok(Value::Record(Rc::new(out)))
}

// ----- Option and Result -------------------------------------------------

fn option_or_result(it: &Interp, v: &Value, name: &str, span: Span) -> Fallible<Wrap> {
    match it.wrap(v) {
        Wrap::Other => Err(wrong(name, 0, Expected::OptionOrResult, v, span)),
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

fn and_then(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let kind = option_or_result(it, &a[0], "and_then", span)?;
    let f = func(a, 1, "and_then", span)?;
    if matches!(kind, Wrap::None | Wrap::Err) {
        return Ok(a[0].clone());
    }
    let r = call_given1(it, "and_then", &f, Interp::payload(&a[0]), span)?;
    match (kind, it.wrap(&r)) {
        (Wrap::Some, Wrap::Some | Wrap::None) | (Wrap::Ok, Wrap::Ok | Wrap::Err) => Ok(r),
        _ => Err(wrong_return("and_then", if kind == Wrap::Some { OPTION } else { RESULT }, &r.describe(), span)),
    }
}

fn ok_or(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    match it.wrap(&a[0]) {
        Wrap::Some => Ok(it.ok(Interp::payload(&a[0]))),
        Wrap::None => Ok(it.err(a[1].clone())),
        _ => Err(wrong("ok_or", 0, Expected::Option, &a[0], span)),
    }
}

fn map_err(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = func(a, 1, "map_err", span)?;
    match it.wrap(&a[0]) {
        Wrap::Ok => Ok(a[0].clone()),
        Wrap::Err => {
            let e = call_given1(it, "map_err", &f, Interp::payload(&a[0]), span)?;
            Ok(it.err(e))
        }
        _ => Err(wrong("map_err", 0, Expected::Result, &a[0], span)),
    }
}

// ----- Files -------------------------------------------------------------

fn read_file(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "read_file", span)?;
    if let Some(e) = in_browser(it, "read_file", false) {
        return Ok(e);
    }
    Ok(match std::fs::read_to_string(path) {
        Ok(s) => it.ok(text_value(s)),
        Err(e) => it.err(text_value(io_message(&e, path, false))),
    })
}

fn write_file(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "write_file", span)?;
    if let Some(e) = in_browser(it, "write_file", false) {
        return Ok(e);
    }
    let content = a[1].display();
    Ok(match std::fs::write(path, content) {
        Ok(()) => it.ok(Value::Unit),
        Err(e) => it.err(text_value(io_message(&e, path, false))),
    })
}

fn append_file(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "append_file", span)?;
    if let Some(e) = in_browser(it, "append_file", false) {
        return Ok(e);
    }
    let content = a[1].display();
    let written = std::fs::OpenOptions::new().append(true).create(true).open(path).and_then(|mut f| f.write_all(content.as_bytes()));
    Ok(match written {
        Ok(()) => it.ok(Value::Unit),
        Err(e) => it.err(text_value(io_message(&e, path, false))),
    })
}

fn file_exists(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let path = text(a, 0, "file_exists", span)?;
    // The browser has no files.
    Ok(Value::Bool(!cfg!(target_arch = "wasm32") && std::path::Path::new(path).exists()))
}
