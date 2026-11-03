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

/// The built-in function of Rythme that a name from another language means:
/// `upper` for `toUpperCase` (`+` for `concat`).
pub fn rythme_name(name: &str) -> Option<&'static str> {
    let rythme = match name {
        "strlen" | "lenght" => "len",
        "println" | "echo" | "printf" | "console" | "printLn" | "say" | "afficher" => "print",
        "str" | "string" | "to_string" | "toString" | "tostring" | "String" => "text",
        "append" | "push_back" | "add" | "ajouter" => "push",
        "chomp" => "trim",
        "toUpperCase" | "uppercase" | "to_upper" | "toupper" | "majuscules" => "upper",
        "toLowerCase" | "lowercase" | "to_lower" | "tolower" | "minuscules" => "lower",
        "capitalise" | "ucfirst" | "upcase_first" | "capitalizeFirst" => "capitalize",
        "padStart" | "pad_start" | "padLeft" | "lpad" | "zfill" => "pad_left",
        "padEnd" | "pad_end" | "padRight" | "rpad" => "pad_right",
        "startsWith" | "startswith" | "hasPrefix" | "has_prefix" => "starts_with",
        "endsWith" | "endswith" | "hasSuffix" | "has_suffix" => "ends_with",
        "replaceAll" | "gsub" => "replace",
        "implode" => "join",
        "explode" => "split",
        "charCodeAt" | "codePointAt" | "char_code_at" | "code_point" | "codepoint" => "char_code",
        "unichr" | "fromCharCode" | "fromCodePoint" | "from_char_code" => "char",
        "isdigit" | "isDigit" | "isdecimal" | "isnumeric" | "isNumeric" | "is_numeric" => "is_digit",
        "isalpha" | "isAlpha" | "is_alpha" | "isLetter" | "is_alphabetic" => "is_letter",
        "isspace" | "isSpace" | "is_whitespace" | "isWhitespace" => "is_space",
        "json_parse" | "parseJSON" | "parseJson" | "from_json" | "json_decode" | "loads" | "json_loads" => "parse_json",
        "stringify" | "toJSON" | "toJson" | "json_encode" | "dumps" | "json_dumps" => "to_json",
        "includes" | "include" | "contient" => "contains",
        "each_char" => "chars",
        "has_key" | "haskey" | "hasOwnProperty" | "has_field" => "has",
        "items" | "pairs" => "entries",
        "fromEntries" | "from_pairs" | "fromPairs" | "to_record" => "from_entries",
        "hashmap" | "hash_map" | "to_dict" | "dictionary" | "dictionnaire" => "dict",
        "hashset" | "hash_set" | "to_set" | "frozenset" | "ensemble" => "set",
        "intersect" | "inter" => "intersection",
        "diff" | "subtract" | "minus" => "difference",
        "issubset" | "is_subset_of" | "subset" | "subset_of" | "includes_all" => "is_subset",
        "mapValues" | "map_vals" | "valmap" => "map_values",
        "foldl" | "fold_left" | "aggregate" => "fold",
        "accumulate" | "scanl" | "running_fold" | "runningFold" | "cumsum" => "scan",
        "forEach" | "for_each" | "foreach" | "pour_chaque" => "each",
        "head" | "car" | "premier" => "first",
        "tail" | "cdr" | "reste" => "rest",
        "substring" | "substr" | "sub" => "slice",
        "indexOf" | "index" | "position" => "index_of",
        "with_index" | "each_index" | "zipWithIndex" | "indexed" => "enumerate",
        "sorted" | "trier" => "sort",
        "reversed" | "inverser" => "reverse",
        "distinct" | "dedup" | "nub" => "unique",
        "shuffled" | "melanger" | "mélanger" => "shuffle",
        "where" | "filtrer" => "filter",
        "chercher" => "find",
        "transformer" => "map",
        "flatMap" | "flatmap" | "concatMap" | "concat_map" => "flat_map",
        "takeWhile" | "takewhile" => "take_while",
        "dropWhile" | "dropwhile" | "skipWhile" | "skip_while" => "drop_while",
        "step" | "stepBy" | "every_nth" | "take_nth" => "step_by",
        "chunk" | "chunked" | "batched" | "in_groups_of" => "chunks",
        "windowed" | "sliding" | "pairwise" | "sliding_window" => "windows",
        "minBy" | "min_by_key" | "minimum_by" => "min_by",
        "maxBy" | "max_by_key" | "maximum_by" => "max_by",
        "setAt" | "set_item" | "replace_at" => "set_at",
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
        "parseInt" | "parse" | "atoi" => "parse_int",
        "parseFloat" | "atof" => "parse_float",
        "andThen" | "bind" => "and_then",
        "okOr" | "ok_or_else" => "ok_or",
        "mapErr" | "map_error" | "mapError" => "map_err",
        "readline" | "gets" | "raw_input" | "scanf" | "lire" => "input",
        "open" | "readFile" | "read" | "slurp" => "read_file",
        "appendFile" | "append_to_file" => "append_file",
        "exists" | "path_exists" | "existsSync" | "is_file" | "isfile" => "file_exists",
        "getenv" | "getEnv" | "environ" | "env_var" => "env",
        "listdir" | "readdir" | "readdirSync" | "read_dir" | "scandir" | "list_dir" | "ls" | "dir" => "list_folder",
        "mkdir" | "makedirs" | "mkdirSync" | "create_dir" | "create_dir_all" | "mkdir_p" | "make_dir" => "create_folder",
        "unlink" | "unlinkSync" | "remove_file" | "rm" | "delete" | "del_file" | "supprimer" => "delete_file",
        "stat" | "statSync" | "getsize" | "file_size" | "filesize" | "metadata" | "is_dir" | "isdir" | "is_folder" => "file_info",
        "path_join" | "joinpath" | "join_paths" | "pathJoin" => "join_path",
        "path_split" | "split_paths" | "path_parts" | "components" => "split_path",
        "stdin_lines" | "read_stdin" | "readAll" | "read_all_lines" => "input_lines",
        "argparse" | "getopt" | "getopts" | "parse_args" | "parseArgs" | "flags" | "argv_options" => "options",
        "assertEqual" | "assert_equal" | "assert_equals" | "assertEquals" => "assert_eq",
        "error" | "panic" | "raise" | "throw" | "die" | "abort" | "erreur" => "fail",
        "isEmpty" | "empty" | "is_empty?" | "vide" => "is_empty",
        "typeof" | "type" | "class_of" | "kind" => "type_of",
        "keys_of" | "cles" | "clés" => "keys",
        "concat" | "concatenate" => "+",
        "power" | "puissance" => "pow",
        "racine" => "sqrt",
        "somme" => "sum",
        "utcnow" | "datetime_now" | "date_now" | "maintenant" => "now",
        "date_today" | "aujourdhui" => "today",
        "strptime" | "fromisoformat" | "parse_datetime" | "parseDate" => "parse_date",
        "strftime" | "isoformat" | "toISOString" | "format_datetime" | "formatDate" => "format_date",
        "weekday" | "isoweekday" | "getDay" | "dayOfWeek" | "day_of_the_week" => "day_of_week",
        "isleap" | "isLeapYear" | "is_leap" | "leap_year" => "is_leap_year",
        "daysInMonth" | "monthrange" | "month_length" => "days_in_month",
        "addDays" | "plusDays" | "plus_days" => "add_days",
        "addSeconds" | "plusSeconds" | "plus_seconds" => "add_seconds",
        "taille" | "longueur" => "len",
        "trimStart" | "trim_left" | "trimLeft" | "ltrim" => "trim_start",
        "trimEnd" | "trim_right" | "trimRight" | "rtrim" => "trim_end",
        "split_words" | "word_list" | "mots" => "words",
        "equalsIgnoreCase" | "eq_ignore_case" | "eq_ignore_ascii_case" | "iequals" | "strcasecmp" | "same_text" => "equals_ignore_case",
        "number_format" | "toLocaleString" | "intcomma" | "format_num" => "format_number",
        "fullmatch" | "is_match" | "isMatch" | "regex_match" => "matches",
        "search" | "re_search" | "find_match" | "first_match" => "find_first",
        "findall" | "findAll" | "matchAll" | "match_all" | "find_iter" | "all_matches" => "find_all",
        "groups" | "submatch" | "find_submatch" => "captures",
        "groupdict" | "named_groups" => "named_captures",
        "re_split" | "regex_split" | "split_regex" => "split_by",
        "re_sub" | "regex_replace" | "replace_regex" | "subn" => "replace_all",
        "parse_csv" | "read_csv" | "csv_read" | "csv_reader" | "parseCSV" | "parseCsv" | "csv_decode" => "csv_parse",
        "DictReader" | "dict_reader" | "csv_dicts" | "csv_rows" | "read_csv_records" => "csv_records",
        "to_csv" | "write_csv" | "csv_write" | "csv_writer" | "toCSV" | "generate_csv" | "csv_encode" => "csv_format",
        "summary" | "describe" | "statistics" | "mittelwert" | "promedio" => "stats",
        "median" | "mediane" | "médiane" | "mediana" | "quantile" | "quantiles" => "percentile",
        "std" | "stdev" | "pstdev" | "stddev" | "standard_deviation" | "ecart_type" => "std_dev",
        "pvariance" | "variance_of" => "variance",
        "eye" | "identity_mat" => "identity_matrix",
        "mmul" | "matrix_multiply" | "matrix_product" | "mat_mul" | "matmult" => "matmul",
        "dot_product" | "vdot" => "dot",
        "magnitude" | "length_of_vector" => "norm",
        "logistic" | "expit" => "sigmoid",
        "randn" | "gauss" | "normalvariate" | "random_gauss" => "random_normal",
        "choices" | "weighted_choice" | "random_weighted" => "random_choice_weighted",
        "train_test" | "split_train_test" => "train_test_split",
        "srand" | "set_seed" | "setSeed" | "randomSeed" | "seed_random" | "mt_srand" => "random_seed",
        "count_of" | "countOf" | "count_occurrences" | "occurrences_of" => "occurrences",
        "removeprefix" | "strip_prefix" | "trim_prefix" | "trimPrefix" => "remove_prefix",
        "removesuffix" | "strip_suffix" | "trim_suffix" | "trimSuffix" => "remove_suffix",
        "titleize" | "titlecase" | "capwords" | "ucwords" | "toTitleCase" | "title_case" => "capitalize_words",
        "snakecase" | "snakeCase" | "to_snake_case" => "snake_case",
        "camelize" | "camelcase" | "camelCase" | "to_camel_case" => "camel_case",
        "dasherize" | "kebabcase" | "kebabCase" | "to_kebab_case" | "parameterize" => "kebab_case",
        "swapCase" => "swap_case",
        "isupper" | "isUpperCase" | "is_uppercase" | "isUpper" => "is_upper",
        "islower" | "isLowerCase" | "is_lowercase" | "isLower" => "is_lower",
        "to_bytes" | "getBytes" | "as_bytes" | "bytearray" => "bytes",
        "splitOnce" | "split_first" => "split_once",
        "wordwrap" | "textwrap" | "wrap_text" => "word_wrap",
        "hex" | "oct" | "bin" | "to_radix" | "base_convert" | "int_to_base" => "to_base",
        "from_radix" | "parse_radix" | "strtol" | "int_from_base" => "from_base",
        "integer_digits" | "digits_of" | "to_digits" => "digits",
        "levenshtein" | "levenshtein_distance" | "edit_dist" | "editDistance" | "string_distance" => "edit_distance",
        "repr" | "pformat" | "pretty_inspect" => "inspect",
        "eprint" | "eprintln" | "eputs" | "print_err" | "printErr" | "error_log" => "print_error",
        "clip" | "coerce_in" | "coerceIn" => "constrain",
        "div_mod" | "divMod" | "quotrem" | "quot_rem" => "divmod",
        "comb" | "nCr" | "n_choose_k" => "binomial",
        "isPositive" | "is_pos" => "is_positive",
        "isNegative" | "is_neg" => "is_negative",
        "isZero" => "is_zero",
        "isnan" | "isNaN" => "is_nan",
        "isfinite" | "isFinite" => "is_finite",
        "centre" | "str_pad_both" => "center",
        "replaceFirst" | "replace_once" | "replacen" => "replace_first",
        "lastIndexOf" | "rfind" | "last_index" => "last_index_of",
        "findIndex" | "find_index_of" | "position_if" => "find_index",
        "updateAt" | "update_index" | "adjust_at" => "update_at",
        "rotateLeft" | "rotate_left" | "rotl" => "rotate_by",
        "interpose" => "intersperse",
        "interlace" => "interleave",
        "frequencies" | "Counter" | "value_counts" | "countValues" => "tally",
        "countBy" | "frequencies_by" | "tally_by" => "count_by",
        "mostCommon" | "most_frequent" => "most_common",
        "uniq_by" | "uniqBy" | "distinct_by" | "distinctBy" | "uniqueBy" => "unique_by",
        "rsort" | "sort_descending" | "reverse_sort" | "sortDesc" | "sorted_desc" => "sort_desc",
        "sortByDesc" | "sort_by_descending" | "sort_by_reverse" => "sort_by_desc",
        "chunkBy" | "partition_by" | "partitionBy" | "slice_when" | "chunk_while" => "chunk_by",
        "splitAt" | "split_at_index" => "split_at",
        "cartesian" | "cartesianProduct" | "cross_join" | "crossJoin" => "cartesian_product",
        "permutations_of" => "permutations",
        "combinations_of" => "combinations",
        "unfoldr" => "unfold",
        "filterMap" | "compactMap" | "compact_map" | "mapMaybe" | "map_maybe" => "filter_map",
        "zipWith" => "zip_with",
        "get_in" | "getIn" | "get_path" | "deep_get" => "dig",
        "mergeWith" | "merge_by" => "merge_with",
        "invert_dict" | "swap_keys_values" => "invert",
        "slice_keys" | "selectKeys" | "take_keys" => "select_keys",
        "without_keys" | "drop_keys" | "dissoc" | "rejectKeys" => "reject_keys",
        "mapKeys" | "rename_keys" => "map_keys",
        "upsert" | "update_with" | "update_key" => "update",
        "symmetricDifference" | "symdiff" | "sym_diff" | "symmetric_diff" => "symmetric_difference",
        "isdisjoint" | "isDisjointFrom" => "is_disjoint",
        "unwrapOrElse" | "getOrElse" | "get_or_else" | "orElseGet" | "or_else_get" => "unwrap_or_else",
        "collect_results" | "allOk" | "sequence_results" | "all_results" => "all_ok",
        "addMonths" | "plusMonths" | "plus_months" | "relativedelta" => "add_months",
        "dayOfYear" | "yday" | "day_of_the_year" | "tm_yday" => "day_of_year",
        "b64encode" | "btoa" | "encode64" | "strict_encode64" | "base64encode" => "base64_encode",
        "b64decode" | "atob" | "decode64" | "strict_decode64" | "base64decode" => "base64_decode",
        "encodeURIComponent" | "encodeURI" | "urlencode" | "percent_encode" | "url_escape" | "quote_plus" => "url_encode",
        "decodeURIComponent" | "decodeURI" | "urldecode" | "percent_decode" | "url_unescape" | "unquote_plus" => "url_decode",
        "corrcoef" | "pearson_correlation" => "correlation",
        "covar" => "covariance",
        "linear_space" | "evenly_spaced" => "linspace",
        // JavaScript (docs/javascript.md has them all).
        "findLast" => "find_last",
        "findLastIndex" => "find_last_index",
        "randomUUID" | "uuid4" | "uuidv4" | "uuid_v4" | "new_uuid" => "uuid",
        "setTimeout" | "delay" | "Sleep" | "usleep" | "msleep" => "sleep",
        "reduceRight" | "foldr" | "fold_right" => "fold",
        "isSubsetOf" => "is_subset",
        "toSorted" => "sort",
        "toReversed" => "reverse",
        "hasOwn" => "has",
        "groupBy" => "group_by",
        "log2" | "log10" => "log",
        "flat" => "flatten",
        "some" => "any",
        "every" => "all",
        "at" => "get",
        "fill" => "repeat",
        "exec" => "captures",
        "cube_root" | "cuberoot" => "cbrt",
        "URL" | "urlparse" | "urlsplit" | "parseUrl" | "parseURL" | "url_parse" => "parse_url",
        "URLSearchParams" | "parse_qs" | "parse_qsl" | "parseQuery" | "querystring" => "parse_query",
        "http_build_query" | "build_query" | "to_query" | "formatQuery" | "stringify_query" => "format_query",
        "digest" | "sha256sum" | "hexdigest" | "SHA256" | "sha_256" => "sha256",
        _ => "",
    };
    (!rythme.is_empty()).then_some(rythme)
}

/// Names from other languages and what to use in Rythme instead.
pub fn alias_hint(name: &str) -> Option<String> {
    if let Some(rythme) = rythme_name(name) {
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
        "format" | "sprintf" | "toFixed" | "to_fixed" => t!(
            "to put values in a text, use braces: \"total: {{x}}\"; to write a number with 2 decimals: `text(x, 2)`",
            "pour mettre des valeurs dans un texte, utilisez des accolades : \"total : {{x}}\" ; pour écrire un nombre avec 2 décimales : `text(x, 2)`"
        ),
        "read_lines" | "readlines" | "readLines" => t!(
            "read the whole file with `read_file(path)`, then cut it with `lines(text)`; the lines of the standard input are `input_lines()`",
            "lisez tout le fichier avec `read_file(chemin)`, puis découpez-le avec `lines(texte)` ; les lignes de l'entrée standard sont `input_lines()`"
        ),
        "merge" | "assign" => t!(
            "to merge two records, write `{{..a, ..b}}`: the fields of `b` replace those of `a`; for two dicts or two sets, `union(a, b)`",
            "pour fusionner deux enregistrements, écrivez `{{..a, ..b}}` : les champs de `b` remplacent ceux de `a` ; pour deux dictionnaires ou deux ensembles, `union(a, b)`"
        ),
        "get_or" | "getOrDefault" | "get_or_default" | "fetch" => t!(
            "write `unwrap_or(get(x, key), default)`: the value, or the default if it is missing",
            "écrivez `unwrap_or(get(x, cle), defaut)` : la valeur, ou la valeur par défaut si elle manque"
        ),
        "average" | "mean" | "avg" | "moyenne" => t!(
            "compute the average with `sum(list) / len(list)`",
            "calculez la moyenne avec `sum(liste) / len(liste)`"
        ),
        "times" => t!(
            "`n.times do |i| ... end` is written `(0..n - 1).each do |i| ... end`",
            "`n.times do |i| ... end` s'écrit `(0..n - 1).each do |i| ... end`"
        ),
        "Math" | "math" => t!(
            "the functions of numbers are built in: write `sqrt(x)`, `abs(x)`, `floor(x)`, `pi`... without `Math.`",
            "les fonctions des nombres sont intégrées : écrivez `sqrt(x)`, `abs(x)`, `floor(x)`, `pi`... sans `Math.`"
        ),
        "yield" => t!(
            "Rythme has no generators: build the whole list (`map`, `filter`, `1..n`), or give a function to call",
            "Rythme n'a pas de générateurs : construisez toute la liste (`map`, `filter`, `1..n`), ou donnez une fonction à appeler"
        ),
        "require" | "require_relative" => t!(
            "to use another file, write `use other_file`, then `other_file.name`",
            "pour utiliser un autre fichier, écrivez `use autre_fichier`, puis `autre_fichier.nom`"
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
