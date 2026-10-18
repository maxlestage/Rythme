//! "Did you mean...?" suggestions and hints for people coming from other languages.

/// Edit distance between two words: letters to add, remove or change, and
/// two neighbour letters swapped (`lenght` → `length`) count as one edit each.
pub fn distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().flat_map(char::to_lowercase).collect();
    let b: Vec<char> = b.chars().flat_map(char::to_lowercase).collect();
    let (n, m) = (a.len(), b.len());
    let mut d = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    d[0] = (0..=m).collect();
    for i in 1..=n {
        for j in 1..=m {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            d[i][j] = (d[i - 1][j] + 1).min(d[i][j - 1] + 1).min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[n][m]
}

/// The closest candidate to `word`, if one is close enough to be a likely typo.
pub fn closest<'a, I>(word: &str, candidates: I) -> Option<String>
where
    I: IntoIterator<Item = &'a str>,
{
    let len = word.chars().count();
    let max = match len {
        // Every one-letter name is one edit away from any other, and changing
        // two letters of a three-letter name gives an unrelated name.
        0..=1 => return None,
        2..=3 => 1,
        4..=5 => 2,
        // Nobody types such a long name by mistake (and comparing is slow).
        6..=64 => 3,
        _ => return None,
    };
    let mut best: Option<(usize, &str)> = None;
    for c in candidates {
        if c == word || c.starts_with('_') && !word.starts_with('_') || c.chars().count().abs_diff(len) > max {
            continue;
        }
        let d = distance(word, c);
        // On a tie, prefer the candidate whose length is closest to the word.
        let gap = |s: &str| s.chars().count().abs_diff(len);
        if d <= max && best.is_none_or(|(bd, bc)| d < bd || (d == bd && (gap(c), c) < (gap(bc), bc))) {
            best = Some((d, c));
        }
    }
    best.map(|(_, c)| c.to_string())
}

/// Names from other languages and what to use in Rythme instead.
pub fn alias_hint(name: &str) -> Option<String> {
    let rythme = match name {
        "length" | "size" | "count_of" | "strlen" | "lenght" => "len",
        "println" | "puts" | "echo" | "printf" | "console" | "printLn" | "say" | "afficher" => "print",
        "str" | "string" | "to_string" | "toString" | "tostring" | "to_s" | "String" => "text",
        "append" | "push_back" | "add" | "ajouter" => "push",
        "strip" | "chomp" => "trim",
        "toUpperCase" | "uppercase" | "to_upper" | "toupper" | "upcase" | "majuscules" => "upper",
        "toLowerCase" | "lowercase" | "to_lower" | "tolower" | "downcase" | "minuscules" => "lower",
        "capitalise" | "ucfirst" | "upcase_first" | "capitalizeFirst" => "capitalize",
        "padStart" | "pad_start" | "padLeft" | "rjust" | "lpad" | "zfill" => "pad_left",
        "padEnd" | "pad_end" | "padRight" | "ljust" | "rpad" => "pad_right",
        "startsWith" | "startswith" | "hasPrefix" | "has_prefix" => "starts_with",
        "endsWith" | "endswith" | "hasSuffix" | "has_suffix" => "ends_with",
        "replaceAll" | "replace_all" | "gsub" => "replace",
        "implode" => "join",
        "explode" => "split",
        "ord" | "charCodeAt" | "codePointAt" | "char_code_at" | "code_point" | "codepoint" => "char_code",
        "chr" | "unichr" | "fromCharCode" | "fromCodePoint" | "from_char_code" => "char",
        "isdigit" | "isDigit" | "isdecimal" | "isnumeric" | "isNumeric" | "is_numeric" => "is_digit",
        "isalpha" | "isAlpha" | "is_alpha" | "isLetter" | "is_alphabetic" => "is_letter",
        "isspace" | "isSpace" | "is_whitespace" | "isWhitespace" => "is_space",
        "json_parse" | "parseJSON" | "parseJson" | "from_json" | "json_decode" | "loads" | "json_loads" => "parse_json",
        "stringify" | "toJSON" | "toJson" | "json_encode" | "dumps" | "json_dumps" => "to_json",
        "includes" | "include" | "contient" => "contains",
        "has_key" | "haskey" | "hasOwnProperty" | "has_field" => "has",
        "items" | "pairs" => "entries",
        "fromEntries" | "from_pairs" | "fromPairs" | "to_h" | "dict" | "to_record" => "from_entries",
        "mapValues" | "map_vals" | "transform_values" | "valmap" => "map_values",
        "reduce" | "foldl" | "fold_left" | "inject" | "aggregate" => "fold",
        "accumulate" | "scanl" | "running_fold" | "runningFold" | "cumsum" => "scan",
        "forEach" | "for_each" | "foreach" | "pour_chaque" => "each",
        "head" | "car" | "premier" => "first",
        "tail" | "cdr" | "reste" => "rest",
        "substring" | "substr" | "sub" => "slice",
        "indexOf" | "index" | "find_index" | "position" => "index_of",
        "each_with_index" | "with_index" | "zipWithIndex" | "indexed" => "enumerate",
        "sorted" | "trier" => "sort",
        "reversed" | "inverser" => "reverse",
        "uniq" | "distinct" | "dedup" | "nub" => "unique",
        "shuffled" | "melanger" | "mélanger" => "shuffle",
        "select" | "where" | "filtrer" => "filter",
        "detect" | "find_first" | "chercher" => "find",
        "collect" | "transformer" => "map",
        "flatMap" | "flatmap" | "concatMap" | "concat_map" => "flat_map",
        "takeWhile" | "takewhile" => "take_while",
        "dropWhile" | "dropwhile" | "skipWhile" | "skip_while" => "drop_while",
        "step" | "stepBy" | "every_nth" | "take_nth" => "step_by",
        "chunk" | "chunked" | "each_slice" | "batched" | "in_groups_of" => "chunks",
        "windowed" | "sliding" | "each_cons" | "pairwise" | "sliding_window" => "windows",
        "minBy" | "min_by_key" | "minimum_by" => "min_by",
        "maxBy" | "max_by_key" | "maximum_by" => "max_by",
        "setAt" | "set_item" | "update_at" | "replace_at" => "set_at",
        "insert" | "insertAt" | "inserer" | "insérer" => "insert_at",
        "removeAt" | "delete_at" | "deleteAt" | "pop" => "remove_at",
        "even" | "isEven" => "is_even",
        "odd" | "isOdd" => "is_odd",
        "trunc" | "truncate" => "int",
        "pgcd" | "greatest_common_divisor" => "gcd",
        "ppcm" | "least_common_multiple" => "lcm",
        "arcsin" => "asin",
        "arccos" => "acos",
        "arctan" | "atan2" => "atan",
        "parseInt" | "parse" | "to_i" | "atoi" => "parse_int",
        "parseFloat" | "to_f" | "atof" => "parse_float",
        "andThen" | "bind" => "and_then",
        "okOr" | "ok_or_else" => "ok_or",
        "mapErr" | "map_error" | "mapError" => "map_err",
        "readline" | "gets" | "raw_input" | "scanf" | "lire" => "input",
        "open" | "readFile" | "read" | "slurp" => "read_file",
        "appendFile" | "append_to_file" => "append_file",
        "exists" | "path_exists" | "existsSync" | "is_file" | "isfile" => "file_exists",
        "getenv" | "getEnv" | "environ" | "env_var" => "env",
        "assertEqual" | "assert_equal" | "assert_equals" | "assertEquals" => "assert_eq",
        "error" | "panic" | "raise" | "throw" | "die" | "abort" | "erreur" => "fail",
        "isEmpty" | "empty" | "is_empty?" | "vide" => "is_empty",
        "typeof" | "type" | "class_of" | "kind" => "type_of",
        "keys_of" | "cles" | "clés" => "keys",
        "concat" | "concatenate" => "+",
        "power" | "puissance" => "pow",
        "racine" => "sqrt",
        "somme" => "sum",
        "taille" | "longueur" => "len",
        _ => "",
    };
    if !rythme.is_empty() {
        if rythme == "+" {
            return Some(t!(
                "to join two lists or two texts, use `+`: `[1, 2] + [3]`; for a list of lists, use `flatten`",
                "pour joindre deux listes ou deux textes, utilisez `+` : `[1, 2] + [3]` ; pour une liste de listes, utilisez `flatten`"
            ));
        }
        return Some(t!("in Rythme, this is called `{rythme}`", "en Rythme, cela s'appelle `{rythme}`"));
    }
    Some(match name {
        "True" => t!("booleans are written in lowercase: `true`", "les booléens s'écrivent en minuscules : `true`"),
        "False" => t!("booleans are written in lowercase: `false`", "les booléens s'écrivent en minuscules : `false`"),
        "NULL" | "Nil" | "undefined" | "nullptr" | "Nothing" => t!(
            "Rythme has no null: use `None` for \"no value\" and `Some(x)` for a value",
            "Rythme n'a pas de null : utilisez `None` pour « pas de valeur » et `Some(x)` pour une valeur"
        ),
        "Just" => t!("in Rythme, this is called `Some`", "en Rythme, cela s'appelle `Some`"),
        "Left" | "Error" | "Failure" => t!("in Rythme, an error value is written `Err(...)`", "en Rythme, une erreur s'écrit `Err(...)`"),
        "Right" | "Success" => t!("in Rythme, a success value is written `Ok(...)`", "en Rythme, un succès s'écrit `Ok(...)`"),
        "self" | "this" => t!(
            "Rythme has no objects: functions receive their data as arguments, like `area(shape)` or `shape.area()`",
            "Rythme n'a pas d'objets : les fonctions reçoivent leurs données en arguments, comme `aire(forme)` ou `forme.aire()`"
        ),
        "range" => t!(
            "write a range with `..`: `1..10` is the list from 1 to 10 (included), and `0..10 |> step_by(2)` counts by 2",
            "écrivez un intervalle avec `..` : `1..10` est la liste de 1 à 10 (inclus), et `0..10 |> step_by(2)` compte de 2 en 2"
        ),
        "div" | "floor_div" | "floordiv" | "intdiv" | "quot" | "quotient" => {
            t!("use the operator `//` for a whole division: `7 // 2` is 3", "utilisez l'opérateur `//` pour une division entière : `7 // 2` vaut 3")
        }
        "mod" | "modulo" | "rem" | "remainder" | "fmod" => {
            t!("use the operator `%` for the remainder: `7 % 3` is 1", "utilisez l'opérateur `%` pour le reste : `7 % 3` vaut 1")
        }
        "divmod" => t!(
            "use the operators `//` and `%`: `7 // 2` is 3 and `7 % 2` is 1",
            "utilisez les opérateurs `//` et `%` : `7 // 2` vaut 3 et `7 % 2` vaut 1"
        ),
        "format" | "sprintf" | "toFixed" | "to_fixed" | "format_number" | "number_format" => t!(
            "to put values in a text, use braces: \"total: {{x}}\"; to write a number with 2 decimals: `text(x, 2)`",
            "pour mettre des valeurs dans un texte, utilisez des accolades : \"total : {{x}}\" ; pour écrire un nombre avec 2 décimales : `text(x, 2)`"
        ),
        "words" => t!(
            "to cut a text into words, use `split(text)`: without a separator, it cuts at spaces",
            "pour découper un texte en mots, utilisez `split(texte)` : sans séparateur, il découpe aux espaces"
        ),
        "read_lines" | "readlines" | "readLines" => t!(
            "read the whole file with `read_file(path)`, then cut it with `lines(text)`",
            "lisez tout le fichier avec `read_file(chemin)`, puis découpez-le avec `lines(texte)`"
        ),
        "merge" | "assign" | "update" => t!(
            "to merge two records, write `{{..a, ..b}}`: the fields of `b` replace those of `a`",
            "pour fusionner deux enregistrements, écrivez `{{..a, ..b}}` : les champs de `b` remplacent ceux de `a`"
        ),
        "get_or" | "getOrDefault" | "get_or_default" | "fetch" => t!(
            "write `unwrap_or(get(x, key), default)`: the value, or the default if it is missing",
            "écrivez `unwrap_or(get(x, cle), defaut)` : la valeur, ou la valeur par défaut si elle manque"
        ),
        "zip_with" | "zipWith" => t!(
            "pair the lists, then combine each pair: `zip(a, b) |> map(fn([x, y]) = x + y)`",
            "associez les listes, puis combinez chaque paire : `zip(a, b) |> map(fn([x, y]) = x + y)`"
        ),
        "sort_desc" | "rsort" | "sort_descending" | "reverse_sort" => t!(
            "sort from the largest to the smallest with `sort(list) |> reverse`",
            "triez du plus grand au plus petit avec `sort(liste) |> reverse`"
        ),
        "average" | "mean" | "avg" | "moyenne" => t!(
            "compute the average with `sum(list) / len(list)`",
            "calculez la moyenne avec `sum(liste) / len(liste)`"
        ),
        "clamp" => t!(
            "keep x between low and high with `max(low, min(x, high))`",
            "gardez x entre bas et haut avec `max(bas, min(x, haut))`"
        ),
        "count_by" | "tally" | "frequencies" | "Counter" => t!(
            "count how many times each value appears with `group_by(list, fn(x) = x) |> map_values(len)`",
            "comptez combien de fois chaque valeur apparaît avec `group_by(liste, fn(x) = x) |> map_values(len)`"
        ),
        "isnan" | "is_nan" | "isNaN" => t!(
            "a decimal number x is NaN (not a number) when `x != x`",
            "un nombre décimal x est NaN (pas un nombre) quand `x != x`"
        ),
        "new" => t!(
            "there is no `new`: build values directly, like `{{name: \"Ada\"}}` or `Circle(2.0)`",
            "il n'y a pas de `new` : construisez les valeurs directement, comme `{{nom: \"Ada\"}}` ou `Cercle(2.0)`"
        ),
        "mut" | "set" | "setattr" => t!(
            "values never change in Rythme: build a new value instead",
            "les valeurs ne changent jamais en Rythme : construisez une nouvelle valeur à la place"
        ),
        _ => return None,
    })
}

/// Explains how to do in Rythme what a keyword of another language does.
pub fn foreign_keyword_hint(word: &str) -> Option<String> {
    Some(match word {
        "for" | "while" | "loop" => t!(
            "Rythme has no loops. Work on the whole list instead:\n  1..5 |> each(fn(i) = print(i))\n  [1, 2, 3] |> map(fn(x) = x * 2)\nor write a recursive function. See `rythme help functional`.",
            "Rythme n'a pas de boucles. Travaillez sur toute la liste à la place :\n  1..5 |> each(fn(i) = print(i))\n  [1, 2, 3] |> map(fn(x) = x * 2)\nou écrivez une fonction récursive. Voir `rythme help functional`."
        ),
        "break" | "continue" => t!(
            "Rythme has no loops to interrupt: use `find`, `take`, `filter`, or a recursive function",
            "Rythme n'a pas de boucles à interrompre : utilisez `find`, `take`, `filter`, ou une fonction récursive"
        ),
        "return" => t!(
            "no `return` needed: the last expression of a function is its result.\nTo stop early with an error, return `Err(...)` and use `?`.",
            "pas besoin de `return` : la dernière expression d'une fonction est son résultat.\nPour arrêter plus tôt avec une erreur, renvoyez `Err(...)` et utilisez `?`."
        ),
        "let" | "var" | "const" => t!(
            "just write `name = value` (values never change, so no keyword is needed)",
            "écrivez simplement `nom = valeur` (les valeurs ne changent jamais, aucun mot-clé n'est nécessaire)"
        ),
        "def" | "function" | "lambda" => t!(
            "functions are written `fn name(x) = x * 2`, and anonymous ones `fn(x) = x * 2`",
            "les fonctions s'écrivent `fn nom(x) = x * 2`, et les anonymes `fn(x) = x * 2`"
        ),
        "class" => t!(
            "Rythme has no classes. Describe your data with `type`:\n  type Shape = Circle(radius) | Square(side)\nor use a record: {{name: \"Ada\", age: 36}}",
            "Rythme n'a pas de classes. Décrivez vos données avec `type` :\n  type Forme = Cercle(rayon) | Carre(cote)\nou utilisez un enregistrement : {{nom: \"Ada\", age: 36}}"
        ),
        "elif" | "elsif" => t!("write `else if`", "écrivez `else if`"),
        "switch" => t!(
            "use `match`:\n  match value\n    0 -> \"zero\"\n    _ -> \"other\"\n  end",
            "utilisez `match` :\n  match valeur\n    0 -> \"zéro\"\n    _ -> \"autre\"\n  end"
        ),
        "import" => t!(
            "to use another file, write `use other_file`, then `other_file.name`",
            "pour utiliser un autre fichier, écrivez `use autre_fichier`, puis `autre_fichier.nom`"
        ),
        "try" | "catch" | "except" | "raise" | "throw" => t!(
            "errors are values in Rythme: return `Ok(value)` or `Err(message)`,\nthen handle them with `match` or pass them on with `?`. `fail(message)` stops the program.",
            "les erreurs sont des valeurs en Rythme : renvoyez `Ok(valeur)` ou `Err(message)`,\npuis traitez-les avec `match` ou transmettez-les avec `?`. `fail(message)` arrête le programme."
        ),
        "null" | "nil" => t!(
            "Rythme has no null: use `None` for \"no value\" and `Some(x)` for a value",
            "Rythme n'a pas de null : utilisez `None` pour « pas de valeur » et `Some(x)` pour une valeur"
        ),
        _ => return None,
    })
}
