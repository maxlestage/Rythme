//! What the help topics show of the functions added to the library after
//! 1.0: a block at the end of a topic (`rythme help lists`...), so that the
//! texts of 1.0 stay as they are, in every language.

use crate::lang::pick;

/// The block shown after the topic `name`, if it has one.
pub fn extra(name: &str) -> Option<&'static str> {
    let (en, fr) = texts(name)?;
    Some(pick(en, fr))
}

/// Every English block, for the catalogs of the other languages.
pub fn fixed_texts() -> Vec<&'static str> {
    ["lists", "text", "dicts", "errors", "dates"].iter().filter_map(|t| texts(t)).map(|(en, _)| en).collect()
}

fn texts(name: &str) -> Option<(&'static str, &'static str)> {
    Some(match name {
        "lists" => (LISTS_EN, LISTS_FR),
        "text" => (TEXT_EN, TEXT_FR),
        "dicts" => (DICTS_EN, DICTS_FR),
        "errors" => (ERRORS_EN, ERRORS_FR),
        "dates" => (DATES_EN, DATES_FR),
        _ => return None,
    })
}

const LISTS_EN: &str = r##"More functions on lists
  find_index([3, 8, 5], is_even)          # Some(1)
  sort_desc([3, 1, 2])                    # [3, 2, 1]  (sort_by_desc with a key)
  tally(["a", "b", "a"])                  # dict("a" => 2, "b" => 1)
  most_common(words, 3)                   # the 3 most frequent, with their counts
  split_at([1, 2, 3], 1)                  # [[1], [2, 3]]
  rotate_by([1, 2, 3], 1)                 # [2, 3, 1]
  combinations([1, 2, 3], 2)              # [[1, 2], [1, 3], [2, 3]]
  iterate(1, 5, fn(x) = x * 2)            # [1, 2, 4, 8, 16]
  permutations(xs)   cartesian_product(xs, ys)   unzip(pairs)   zip(xs, ys, zs)
  filter_map(xs, f)   chunk_by(xs, f)   unique_by(xs, f)   count_by(xs, f)"##;

const LISTS_FR: &str = r##"D'autres fonctions sur les listes
  find_index([3, 8, 5], is_even)          # Some(1)
  sort_desc([3, 1, 2])                    # [3, 2, 1]  (sort_by_desc avec une clé)
  tally(["a", "b", "a"])                  # dict("a" => 2, "b" => 1)
  most_common(mots, 3)                    # les 3 plus fréquents, avec leur nombre
  split_at([1, 2, 3], 1)                  # [[1], [2, 3]]
  rotate_by([1, 2, 3], 1)                 # [2, 3, 1]
  combinations([1, 2, 3], 2)              # [[1, 2], [1, 3], [2, 3]]
  iterate(1, 5, fn(x) = x * 2)            # [1, 2, 4, 8, 16]
  permutations(xs)   cartesian_product(xs, ys)   unzip(paires)   zip(xs, ys, zs)
  filter_map(xs, f)   chunk_by(xs, f)   unique_by(xs, f)   count_by(xs, f)"##;

const TEXT_EN: &str = r##"More functions on texts
  format(3.14159, ".2f")         # "3.14"  (the format specs of Python: ">8", ",d", ".1%", "#x"...)
  center("hi", 6, "*")           # "**hi**"
  replace_first("a-b-c", "-", "+")          # "a+b-c"
  occurrences("banana", "an")    # 2
  remove_prefix("Mr Smith", "Mr ")          # "Smith"  (and remove_suffix)
  split_once("key=value", "=")   # Some(["key", "value"])
  snake_case("helloWorld")       # "hello_world"  (and camel_case, kebab_case)
  capitalize_words("ada lovelace")          # "Ada Lovelace"
  word_wrap(text, 40)            # lines of at most 40 characters
  to_base(255, 16)   from_base("ff", 16)    # "ff" and Ok(255)
  edit_distance("kitten", "sitting")        # 3"##;

const TEXT_FR: &str = r##"D'autres fonctions sur les textes
  format(3.14159, ".2f")         # "3.14"  (les spécifications de format de Python : ">8", ",d", ".1%", "#x"...)
  center("hi", 6, "*")           # "**hi**"
  replace_first("a-b-c", "-", "+")          # "a+b-c"
  occurrences("banane", "an")    # 2
  remove_prefix("M. Dupont", "M. ")         # "Dupont"  (et remove_suffix)
  split_once("clé=valeur", "=")  # Some(["clé", "valeur"])
  snake_case("helloWorld")       # "hello_world"  (et camel_case, kebab_case)
  capitalize_words("ada lovelace")          # "Ada Lovelace"
  word_wrap(texte, 40)           # des lignes d'au plus 40 caractères
  to_base(255, 16)   from_base("ff", 16)    # "ff" et Ok(255)
  edit_distance("chien", "chine")           # 2"##;

const DICTS_EN: &str = r##"More functions on records and dicts
  dig(data, ["user", "langs", 0])           # Some(...), or None: deep inside
  select_keys(person, ["name", "age"])      # only these keys (reject_keys: without them)
  merge_with(a, b, fn(x, y) = x + y)        # f decides for a key in both
  update(counts, "apples", 1, fn(n) = n + 1)  # +1, or 1 when missing
  invert({a: 1, b: 2})                      # dict(1 => "a", 2 => "b")
  map_keys(r, upper)   symmetric_difference(s, t)   is_disjoint(s, t)"##;

const DICTS_FR: &str = r##"D'autres fonctions sur les enregistrements et les dictionnaires
  dig(donnees, ["user", "langs", 0])        # Some(...), ou None : tout au fond
  select_keys(personne, ["nom", "age"])     # seulement ces clés (reject_keys : sans elles)
  merge_with(a, b, fn(x, y) = x + y)        # f décide pour une clé présente dans les deux
  update(compte, "pommes", 1, fn(n) = n + 1)  # +1, ou 1 quand elle manque
  invert({a: 1, b: 2})                      # dict(1 => "a", 2 => "b")
  map_keys(r, upper)   symmetric_difference(s, t)   is_disjoint(s, t)"##;

const ERRORS_EN: &str = r##"More helpers
  unwrap_or_else(x, fn(e) = ...)   # the default, computed only when it is needed
  expect(x, "a config file")       # like unwrap, with your message if it fails
  all_ok(map(texts, parse_int))    # Ok([...]), or the first Err
  filter_map(xs, f)                # the values of the Some(...) that f gives"##;

const ERRORS_FR: &str = r##"D'autres outils
  unwrap_or_else(x, fn(e) = ...)   # la valeur par défaut, calculée seulement si elle sert
  expect(x, "un fichier de config") # comme unwrap, avec votre message en cas d'échec
  all_ok(map(textes, parse_int))   # Ok([...]), ou le premier Err
  filter_map(xs, f)                # les valeurs des Some(...) que donne f"##;

const DATES_EN: &str = r##"More on dates
  add_months(date, 1)                       # 31 January + 1 month: the end of February
  day_of_year(date)                         # from 1 to 366
  format_date(date, "%d/%m/%Y %H:%M")       # with a pattern: %Y %m %d %H %M %S %B %A...
  parse_date("29/02/2024", "%d/%m/%Y")      # Ok({year: 2024, month: 2, day: 29})"##;

const DATES_FR: &str = r##"Encore sur les dates
  add_months(date, 1)                       # 31 janvier + 1 mois : la fin de février
  day_of_year(date)                         # de 1 à 366
  format_date(date, "%d/%m/%Y %H:%M")       # avec un motif : %Y %m %d %H %M %S %B %A...
  parse_date("29/02/2024", "%d/%m/%Y")      # Ok({year: 2024, month: 2, day: 29})"##;
