//! Help everywhere: topics, function documentation, error explanations and
//! a guided tour, in English and in French.

use std::io::{BufRead, IsTerminal, Write};

use crate::builtins::{self, Builtin, Category, BUILTINS};
use crate::interp::{Interp, Wrap};
use crate::lang::{is_fr, pick};
use crate::source;
use crate::suggest;
use crate::value::Value;

/// Topic names, in the order of the guided tour, with their French aliases.
pub const TOPICS: &[(&str, &[&str])] = &[
    ("start", &["intro", "debut", "début", "commencer"]),
    ("syntax", &["syntaxe"]),
    ("functions", &["fonctions", "fn"]),
    ("lists", &["listes", "list", "liste"]),
    ("text", &["texte", "texts", "textes", "string"]),
    ("records", &["enregistrements", "record", "enregistrement"]),
    ("types", &["type"]),
    ("match", &["patterns", "motifs"]),
    ("errors", &["erreurs", "option", "result", "?"]),
    ("functional", &["fonctionnel", "loops", "boucles"]),
    ("tests", &["test"]),
    ("modules", &["use", "files", "fichiers"]),
    ("repl", &["interactive", "interactif"]),
    ("fmt", &["format", "formater", "formatage", "formatting", "style", "mise-en-forme"]),
    ("editors", &["editor", "lsp", "editeurs", "éditeurs", "editeur", "éditeur", "vscode", "ide"]),
    ("builtins", &["functions-list", "fonctions-integrees", "library", "bibliotheque", "bibliothèque", "stdlib"]),
];

fn canonical_topic(name: &str) -> Option<&'static str> {
    let name = name.trim().to_lowercase();
    TOPICS.iter().find(|(t, aliases)| *t == name || aliases.contains(&name.as_str())).map(|(t, _)| *t)
}

pub fn overview() -> String {
    let v = crate::VERSION;
    if is_fr() {
        format!(
            r#"Rythme {v} — aide

Commandes
  rythme                    mode interactif (essayer du code ligne par ligne)
  rythme fichier.ry         lancer un programme
  rythme check fichier.ry   trouver les problèmes sans lancer le programme
  rythme test fichier.ry    lancer les tests d'un fichier
  rythme fmt fichier.ry     mettre le code en forme (--check : vérifier seulement)
  rythme learn              visite guidée du langage, pas à pas
  rythme help SUJET         aide sur un sujet ou une fonction
  rythme explain E003       expliquer un code d'erreur
  rythme -e "code"          exécuter un morceau de code
  rythme lsp                serveur de langage, pour les éditeurs (VS Code, Neovim...)
  option --lang fr|en       langue des messages (ou variable RYTHME_LANG)

Sujets (rythme help SUJET)
  start       le langage en une minute
  syntax      toute la syntaxe sur une page
  functions   fonctions, |>, récursion
  lists       listes et leurs fonctions
  text        textes
  records     enregistrements {{nom: valeur}}
  types       vos propres types
  match       choisir selon la forme d'une valeur
  errors      Option, Result et ?
  functional  pas de boucles : comment faire à la place
  tests       écrire et lancer des tests
  modules     utiliser plusieurs fichiers
  repl        le mode interactif
  fmt         la mise en forme officielle du code (rythme fmt)
  editors     Rythme dans votre éditeur (VS Code, Neovim, Helix...)
  builtins    toutes les fonctions intégrées

Une fonction précise : rythme help map
"#
        )
    } else {
        format!(
            r#"Rythme {v} — help

Commands
  rythme                    interactive mode (try code line by line)
  rythme file.ry            run a program
  rythme check file.ry      find problems without running the program
  rythme test file.ry       run the tests of a file
  rythme fmt file.ry        give the code its official layout (--check: only check)
  rythme learn              guided tour of the language, step by step
  rythme help TOPIC         help on a topic or a function
  rythme explain E003       explain an error code
  rythme -e "code"          run a piece of code
  rythme lsp                language server, for editors (VS Code, Neovim...)
  option --lang fr|en       language of the messages (or RYTHME_LANG)

Topics (rythme help TOPIC)
  start       the language in one minute
  syntax      the whole syntax on one page
  functions   functions, |>, recursion
  lists       lists and their functions
  text        texts
  records     records {{name: value}}
  types       your own types
  match       choosing according to the shape of a value
  errors      Option, Result and ?
  functional  no loops: what to do instead
  tests       writing and running tests
  modules     using several files
  repl        the interactive mode
  fmt         the official layout of the code (rythme fmt)
  editors     Rythme in your editor (VS Code, Neovim, Helix...)
  builtins    every built-in function

A specific function: rythme help map
"#
        )
    }
}

/// The text of a topic, or the documentation of a built-in function.
pub fn topic(name: &str) -> Option<String> {
    let builtin = builtins::lookup(name.trim());
    if let (Some(b), None) = (builtin, canonical_topic(name)) {
        return Some(builtin_doc(b));
    }
    if let Some(text) = explain(name) {
        return Some(text);
    }
    if name.trim() == "pi" {
        return Some(t!("pi\n  The number π = 3.141592653589793.", "pi\n  Le nombre π = 3.141592653589793."));
    }
    if canonical_topic(name).is_none() {
        if let Some(text) = keyword_help(name.trim()) {
            return Some(text);
        }
    }
    let t = canonical_topic(name)?;
    if t == "builtins" {
        return Some(builtins_list());
    }
    let mut text = topic_text(t).to_string();
    if let Some(b) = builtin {
        // `text` is both a topic and a function: show both.
        text.push_str(&format!("\n\n{}\n{}", t!("The function:", "La fonction :"), builtin_doc(b)));
    }
    Some(text)
}

/// Help when the topic is unknown: suggestions.
pub fn unknown_topic(name: &str) -> String {
    let mut names: Vec<&str> = TOPICS.iter().map(|(t, _)| *t).collect();
    names.extend(BUILTINS.iter().map(|b| b.name));
    let mut out = t!("No help found for `{name}`.", "Aucune aide trouvée pour `{name}`.");
    if let Some(h) = suggest::alias_hint(name) {
        out.push_str(&format!("\n{h}"));
    } else if let Some(best) = suggest::closest(name, names) {
        out.push_str(&t!("\nDid you mean `{best}`?", "\nVouliez-vous dire `{best}` ?"));
    }
    out.push_str(&t!("\nSee the list of topics with `rythme help`.", "\nVoyez la liste des sujets avec `rythme help`."));
    out
}

/// Every keyword, explained: (keyword, English, French, example, topic).
const KEYWORD_DOCS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "fn",
        "Defines a function: `fn name(x) = expression`, or with its body on the following lines, finished by `end`. Without a name, `fn(x) = x * 2` is an anonymous function.",
        "Définit une fonction : `fn nom(x) = expression`, ou avec son corps sur les lignes suivantes, terminé par `end`. Sans nom, `fn(x) = x * 2` est une fonction anonyme.",
        "fn double(x) = x * 2",
        "functions",
    ),
    (
        "if",
        "Chooses between two values: `if condition then value else other`. Every `if` has an `else`, because an `if` always gives a value.",
        "Choisit entre deux valeurs : `if condition then valeur else autre`. Chaque `if` a un `else`, car un `if` donne toujours une valeur.",
        "if n < 0 then \"negative\" else \"positive\"",
        "syntax",
    ),
    (
        "then",
        "Separates the condition of an `if` from the value it gives when the condition is true.",
        "Sépare la condition d'un `if` de la valeur donnée quand la condition est vraie.",
        "if n < 0 then \"negative\" else \"positive\"",
        "syntax",
    ),
    (
        "else",
        "Gives the value of an `if` when the condition is false. It is always required; `else if` chains several conditions.",
        "Donne la valeur d'un `if` quand la condition est fausse. Il est toujours obligatoire ; `else if` enchaîne plusieurs conditions.",
        "if n < 0 then \"negative\" else if n == 0 then \"zero\" else \"positive\"",
        "syntax",
    ),
    (
        "match",
        "Chooses according to the shape of a value: one case per line, `pattern -> result`, finished by `end`. Cases are tried in order, and `rythme check` warns about forgotten cases.",
        "Choisit selon la forme d'une valeur : un cas par ligne, `motif -> résultat`, terminé par `end`. Les cas sont essayés dans l'ordre, et `rythme check` signale les cas oubliés.",
        "match list\n  [] -> \"empty\"\n  [x, ..rest] -> \"starts with {x}\"\nend",
        "match",
    ),
    (
        "end",
        "Closes a block: a function written on several lines, a `match`, a `do` or a `test`.",
        "Ferme un bloc : une fonction écrite sur plusieurs lignes, un `match`, un `do` ou un `test`.",
        "fn greet(name)\n  print(\"Hello, {name}!\")\nend",
        "syntax",
    ),
    (
        "do",
        "Groups several steps inside an expression, up to `end`. The last line gives the value.",
        "Regroupe plusieurs étapes dans une expression, jusqu'à `end`. La dernière ligne donne la valeur.",
        "total = do\n  a = 1\n  a + 1\nend",
        "syntax",
    ),
    (
        "type",
        "Defines your own type and its variants. Each variant is a constructor, with or without fields.",
        "Définit votre propre type et ses variantes. Chaque variante est un constructeur, avec ou sans champs.",
        "type Shape = Circle(radius) | Rect(width, height)",
        "types",
    ),
    (
        "test",
        "Defines a test, next to the code. `rythme test file.ry` runs the tests of the file.",
        "Définit un test, à côté du code. `rythme test fichier.ry` lance les tests du fichier.",
        "test \"double works\" = assert_eq(double(21), 42)",
        "tests",
    ),
    (
        "use",
        "Uses the definitions of another file. The path is relative to the folder of the file that contains `use`.",
        "Utilise les définitions d'un autre fichier. Le chemin est relatif au dossier du fichier qui contient `use`.",
        "use \"geometry.ry\"",
        "modules",
    ),
    (
        "and",
        "True when both conditions are true. The second one is computed only if the first one is true.",
        "Vrai quand les deux conditions sont vraies. La seconde n'est calculée que si la première est vraie.",
        "if age >= 18 and has_ticket then \"enter\" else \"wait\"",
        "syntax",
    ),
    (
        "or",
        "True when at least one of the conditions is true. The second one is computed only if the first one is false.",
        "Vrai quand au moins une des conditions est vraie. La seconde n'est calculée que si la première est fausse.",
        "is_weekend = day == \"saturday\" or day == \"sunday\"",
        "syntax",
    ),
    ("not", "Inverts a condition: `not true` is `false`.", "Inverse une condition : `not true` vaut `false`.", "if not done then \"wait\" else \"go\"", "syntax"),
    ("true", "One of the two values of the type Bool.", "L'une des deux valeurs du type Bool.", "is_ready = true", "syntax"),
    ("false", "One of the two values of the type Bool.", "L'une des deux valeurs du type Bool.", "is_ready = false", "syntax"),
];

/// The documentation of a keyword in the current language, with an example
/// and the help topic that tells more.
pub fn keyword_doc(word: &str) -> Option<(&'static str, &'static str, &'static str)> {
    KEYWORD_DOCS.iter().find(|k| k.0 == word).map(|&(_, en, fr, example, topic)| (pick(en, fr), example, topic))
}

/// What `rythme help then` shows.
pub fn keyword_help(word: &str) -> Option<String> {
    let (doc, example, topic) = keyword_doc(word)?;
    let mut out = format!("{word}  ({})\n  {doc}\n  {}\n", t!("keyword", "mot-clé"), t!("Example:", "Exemple :"));
    for line in example.lines() {
        out.push_str(&format!("    {line}\n"));
    }
    out.push_str(&t!("  More: rythme help {topic}\n", "  Plus : rythme help {topic}\n"));
    Some(out)
}

pub fn builtin_doc(b: &Builtin) -> String {
    let mut out = format!("{}\n  {}\n", b.sig, b.doc());
    out.push_str(&format!("  {}\n", t!("Example:", "Exemple :")));
    for line in b.example.lines() {
        out.push_str(&format!("    {line}\n"));
    }
    out
}

pub fn builtins_list() -> String {
    let mut out = t!("BUILT-IN FUNCTIONS\n", "FONCTIONS INTÉGRÉES\n");
    for cat in Category::ALL {
        out.push_str(&format!("\n{}\n", cat.title()));
        for b in BUILTINS.iter().filter(|b| b.cat == cat) {
            let doc = b.doc();
            let short = doc.split(". ").next().unwrap_or(doc).trim_end_matches('.');
            out.push_str(&format!("  {:<28} {}\n", b.sig, short));
        }
        if cat == Category::Numbers {
            out.push_str(&format!("  {:<28} {}\n", "pi", t!("the number π", "le nombre π")));
        }
        if cat == Category::Options {
            out.push_str(&format!(
                "  {:<28} {}\n",
                "Some(x)  None  Ok(x)  Err(e)",
                t!("build an Option or a Result", "construire une Option ou un Result")
            ));
        }
    }
    out.push_str(&t!(
        "\nDetails and example for one function: rythme help NAME (or help(NAME) in code)\n",
        "\nDétails et exemple pour une fonction : rythme help NOM (ou help(NOM) dans le code)\n"
    ));
    out
}

/// The `##` documentation of the type of a constructor, if it has one.
fn push_type_doc(out: &mut String, c: &crate::value::CtorInfo) {
    if let Some(doc) = &c.doc {
        for line in doc.lines() {
            out.push_str(&format!("  {line}\n"));
        }
    }
}

/// What `help(x)` shows inside a program or in the REPL.
pub fn help_for_value(it: &Interp, v: Option<&Value>) -> String {
    let Some(v) = v else {
        return t!(
            "help(x) explains x: a function (help(map)), a topic (help(\"lists\")), or a value (help([1, 2])).\nTopics: start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins.",
            "help(x) explique x : une fonction (help(map)), un sujet (help(\"lists\")), ou une valeur (help([1, 2])).\nSujets : start, syntax, functions, lists, text, records, types, match, errors, functional, tests, modules, builtins."
        );
    };
    match v {
        Value::Builtin(b) => builtin_doc(b),
        Value::Func(c) => {
            let mut out = format!("{}\n", c.def.signature());
            match &c.def.doc {
                Some(doc) => {
                    for line in doc.lines() {
                        out.push_str(&format!("  {line}\n"));
                    }
                }
                None => out.push_str(&t!(
                    "  (no documentation: write `## description` on the line above `fn`)\n",
                    "  (pas de documentation : écrivez `## description` sur la ligne au-dessus de `fn`)\n"
                )),
            }
            if c.def.name.is_some() {
                out.push_str(&format!("  {} {}\n", t!("defined at", "définie en"), source::location(c.def.span)));
            }
            out
        }
        Value::Ctor(c) => {
            let fields: Vec<String> = c.fields.iter().map(|f| f.to_string()).collect();
            let mut out = t!(
                "{}({})\n  A constructor of the type `{}`.\n",
                "{}({})\n  Un constructeur du type `{}`.\n",
                c.name,
                fields.join(", "),
                c.type_name
            );
            push_type_doc(&mut out, c);
            out
        }
        Value::Text(s) => topic(s).unwrap_or_else(|| unknown_topic(s)),
        other => {
            let ty = other.type_name();
            let cat = match other {
                Value::List(_) => Some(Category::Lists),
                Value::Text(_) => Some(Category::Text),
                Value::Int(_) | Value::Float(_) => Some(Category::Numbers),
                Value::Record(_) => Some(Category::Records),
                Value::Variant(_) if it.wrap(other) != Wrap::Other => Some(Category::Options),
                _ => None,
            };
            let mut out = t!("This is a value of type {ty}: {}\n", "Ceci est une valeur de type {ty} : {}\n", other.repr());
            if let Value::Variant(var) = other {
                push_type_doc(&mut out, &var.ctor);
            }
            if let Some(cat) = cat {
                let names: Vec<&str> = BUILTINS.iter().filter(|b| b.cat == cat).map(|b| b.name).collect();
                out.push_str(&t!("Useful functions: {}\n", "Fonctions utiles : {}\n", names.join(", ")));
                // Functions of other categories that work on this kind of value too.
                let also = match cat {
                    Category::Lists => "len, contains, join, repeat, min, max, to_json",
                    Category::Records => "len, get, contains, is_empty, to_json",
                    Category::Options => "map",
                    _ => "",
                };
                if !also.is_empty() {
                    out.push_str(&t!("Also: {also}\n", "Aussi : {also}\n"));
                }
            }
            if let Value::Variant(var) = other {
                if it.wrap(other) == Wrap::Other {
                    out.push_str(&t!(
                        "Use `match` to handle each variant of `{}`.\n",
                        "Utilisez `match` pour traiter chaque variante de `{}`.\n",
                        var.ctor.type_name
                    ));
                }
            }
            out
        }
    }
}

/// The guided tour: every topic, one after the other.
pub fn learn() {
    let interactive = std::io::stdin().is_terminal();
    let order =
        ["start", "syntax", "functions", "lists", "text", "records", "types", "match", "errors", "functional", "tests", "modules", "repl"];
    for (i, name) in order.iter().enumerate() {
        crate::io::out_line(&format!("── {}/{} ──\n", i + 1, order.len()));
        crate::io::out_line(topic_text(name).trim_end());
        if interactive && i + 1 < order.len() {
            crate::io::out(&format!("\n{}", t!("[Enter] next   [q] quit > ", "[Entrée] suite   [q] quitter > ")));
            let _ = std::io::stdout().flush();
            let mut line = String::new();
            if std::io::stdin().lock().read_line(&mut line).unwrap_or(0) == 0 || line.trim().eq_ignore_ascii_case("q") {
                break;
            }
        }
        crate::io::out_line("");
    }
    crate::io::out_line(&t!(
        "To practice, run `rythme` and type code. `rythme help builtins` lists every function.",
        "Pour pratiquer, lancez `rythme` et tapez du code. `rythme help builtins` liste toutes les fonctions."
    ));
}

fn topic_text(name: &str) -> &'static str {
    match name {
        "start" => pick(START_EN, START_FR),
        "syntax" => pick(SYNTAX_EN, SYNTAX_FR),
        "functions" => pick(FUNCTIONS_EN, FUNCTIONS_FR),
        "lists" => pick(LISTS_EN, LISTS_FR),
        "text" => pick(TEXT_EN, TEXT_FR),
        "records" => pick(RECORDS_EN, RECORDS_FR),
        "types" => pick(TYPES_EN, TYPES_FR),
        "match" => pick(MATCH_EN, MATCH_FR),
        "errors" => pick(ERRORS_EN, ERRORS_FR),
        "functional" => pick(FUNCTIONAL_EN, FUNCTIONAL_FR),
        "tests" => pick(TESTS_EN, TESTS_FR),
        "modules" => pick(MODULES_EN, MODULES_FR),
        "repl" => pick(REPL_EN, REPL_FR),
        "fmt" => pick(FMT_EN, FMT_FR),
        "editors" => pick(EDITORS_EN, EDITORS_FR),
        _ => "",
    }
}

const START_EN: &str = r#"RYTHME IN ONE MINUTE

Rythme is a simple language where values never change and everything is a
function. Programs are files ending with .ry.

  # A comment
  name = "World"                 # a name for a value (it never changes)
  print("Hello, {name}!")        # {...} inserts a value in a text

  fn square(x) = x * x           # a function
  print(square(4))               # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # keep the even numbers
    |> map(square)               # square each one
    |> sum                       # add them up
    |> print                     # 20

Run it:     rythme hello.ry
Check it:   rythme check hello.ry
Try code:   rythme          (interactive mode)

Rythme helps you all the time: every error says where it is, why, and how
to fix it. `rythme explain E003` explains an error code in detail."#;

const START_FR: &str = r#"RYTHME EN UNE MINUTE

Rythme est un langage simple où les valeurs ne changent jamais et où tout
est fonction. Les programmes sont des fichiers qui finissent par .ry.

  # Un commentaire
  nom = "Monde"                  # un nom pour une valeur (il ne change jamais)
  print("Bonjour, {nom} !")      # {...} insère une valeur dans un texte

  fn carre(x) = x * x            # une fonction
  print(carre(4))                # 16

  [1, 2, 3, 4]
    |> filter(is_even)           # garder les nombres pairs
    |> map(carre)                # mettre chacun au carré
    |> sum                       # les additionner
    |> print                     # 20

Lancer :        rythme bonjour.ry
Vérifier :      rythme check bonjour.ry
Essayer :       rythme          (mode interactif)

Rythme vous aide tout le temps : chaque erreur dit où elle est, pourquoi,
et comment la corriger. `rythme explain E003` explique un code d'erreur."#;

const SYNTAX_EN: &str = r#"SYNTAX

Values
  42   -7   1_000_000            whole numbers (Int)
  3.14   2.0                     decimal numbers (Float)
  "hello"   "total: {x}"         texts (Text); {...} inserts a value
  true   false                   booleans (Bool)
  [1, 2, 3]                      lists
  {name: "Ada", age: 36}         records
  Some(3)   None                 a value that may be missing (Option)
  Ok(3)   Err("oops")            a result that may fail (Result)
  1..5                           the list [1, 2, 3, 4, 5]

Names
  x = 5                          a name for a value; it never changes
  [a, b] = [1, 2]                take a value apart
  {name, age} = person

Operators
  + - * /   // (whole division)   % (remainder)   ** (power)
  == != < <= > >=   and   or   not
  x |> f(y)                      same as f(x, y): steps from left to right
  x.f(y)                         also the same as f(x, y)

Functions
  fn double(x) = x * 2           on one line, with =
  fn greet(name)                 on several lines, finished by `end`
    message = "Hello {name}"
    print(message)               the last line gives the result
  end
  fn(x) = x + 1                  an anonymous function

Conditions (every if has an else)
  if x > 0 then "positive" else if x == 0 then "zero" else "negative"

Choosing by shape
  match list
    [] -> "empty"
    [x] -> "one element: {x}"
    [x, ..rest] -> "{x}, then {len(rest)} more"
  end

Several steps inside an expression
  do
    a = 1
    a + 1
  end

Your own types
  type Shape = Circle(radius) | Rect(width, height)

Tests and other files
  test "double works" = assert_eq(double(2), 4)
  use "geometry.ry"

Comments
  # an ordinary comment
  ## a documentation comment, above a function, shown by help(...)"#;

const SYNTAX_FR: &str = r#"SYNTAXE

Valeurs
  42   -7   1_000_000            nombres entiers (Int)
  3.14   2.0                     nombres décimaux (Float)
  "bonjour"   "total : {x}"      textes (Text) ; {...} insère une valeur
  true   false                   booléens (Bool)
  [1, 2, 3]                      listes
  {nom: "Ada", age: 36}          enregistrements
  Some(3)   None                 une valeur qui peut manquer (Option)
  Ok(3)   Err("oups")            un résultat qui peut échouer (Result)
  1..5                           la liste [1, 2, 3, 4, 5]

Noms
  x = 5                          un nom pour une valeur ; il ne change jamais
  [a, b] = [1, 2]                décomposer une valeur
  {nom, age} = personne

Opérateurs
  + - * /   // (division entière)   % (reste)   ** (puissance)
  == != < <= > >=   and (et)   or (ou)   not (non)
  x |> f(y)                      comme f(x, y) : des étapes de gauche à droite
  x.f(y)                         aussi comme f(x, y)

Fonctions
  fn double(x) = x * 2           sur une ligne, avec =
  fn saluer(nom)                 sur plusieurs lignes, terminée par `end`
    message = "Bonjour {nom}"
    print(message)               la dernière ligne donne le résultat
  end
  fn(x) = x + 1                  une fonction anonyme

Conditions (chaque if a un else)
  if x > 0 then "positif" else if x == 0 then "zéro" else "négatif"

Choisir selon la forme
  match liste
    [] -> "vide"
    [x] -> "un élément : {x}"
    [x, ..reste] -> "{x}, puis {len(reste)} autres"
  end

Plusieurs étapes dans une expression
  do
    a = 1
    a + 1
  end

Vos propres types
  type Forme = Cercle(rayon) | Rect(largeur, hauteur)

Tests et autres fichiers
  test "double marche" = assert_eq(double(2), 4)
  use "geometrie.ry"

Commentaires
  # un commentaire ordinaire
  ## un commentaire de documentation, au-dessus d'une fonction, affiché par help(...)"#;

const FUNCTIONS_EN: &str = r#"FUNCTIONS

Define and call
  fn add(a, b) = a + b
  add(2, 3)                      # 5

On several lines: the last expression is the result
  fn describe(n)
    kind = if n % 2 == 0 then "even" else "odd"
    "{n} is {kind}"
  end

One definition per case (tried in order)
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

With a condition (a guard)
  fn sign(n) if n < 0 = "negative"
  fn sign(0) = "zero"
  fn sign(_) = "positive"

Functions are values
  double = fn(x) = x * 2
  [1, 2, 3] |> map(double)       # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

Pipes read from left to right
  "  Hello  " |> trim |> upper   # "HELLO"
  x |> f(a, b)    means    f(x, a, b)

Method style
  "hello".upper()                # same as upper("hello")

Documentation
  ## Adds two numbers.
  fn add(a, b) = a + b
  help(add)                      # shows "Adds two numbers."

Recursion without limit
  When the recursive call is the very last thing a function does, it can
  repeat millions of times:
  fn count_down(0) = print("Liftoff!")
  fn count_down(n)
    print(n)
    count_down(n - 1)
  end"#;

const FUNCTIONS_FR: &str = r#"FONCTIONS

Définir et appeler
  fn ajouter(a, b) = a + b
  ajouter(2, 3)                  # 5

Sur plusieurs lignes : la dernière expression est le résultat
  fn decrire(n)
    sorte = if n % 2 == 0 then "pair" else "impair"
    "{n} est {sorte}"
  end

Une définition par cas (essayées dans l'ordre)
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)

Avec une condition (une garde)
  fn signe(n) if n < 0 = "négatif"
  fn signe(0) = "zéro"
  fn signe(_) = "positif"

Les fonctions sont des valeurs
  double = fn(x) = x * 2
  [1, 2, 3] |> map(double)       # [2, 4, 6]
  [1, 2, 3] |> map(fn(x) = x + 1)

Les enchaînements se lisent de gauche à droite
  "  Salut  " |> trim |> upper   # "SALUT"
  x |> f(a, b)    signifie    f(x, a, b)

Style méthode
  "salut".upper()                # comme upper("salut")

Documentation
  ## Additionne deux nombres.
  fn ajouter(a, b) = a + b
  help(ajouter)                  # affiche "Additionne deux nombres."

Récursion sans limite
  Quand l'appel récursif est la toute dernière chose que fait une fonction,
  elle peut se répéter des millions de fois :
  fn compte_a_rebours(0) = print("Décollage !")
  fn compte_a_rebours(n)
    print(n)
    compte_a_rebours(n - 1)
  end"#;

const LISTS_EN: &str = r#"LISTS

  numbers = [3, 1, 2]
  numbers[0]                     # 3 (the first; -1 is the last)
  len(numbers)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..numbers]                 # [0, 3, 1, 2]   (.. inserts a whole list)
  numbers + [9]                  # [3, 1, 2, 9]

Work on the whole list (no loops needed)
  map(numbers, fn(x) = x * 10)            # [30, 10, 20]
  filter(numbers, fn(x) = x > 1)          # [3, 2]
  fold(numbers, 0, fn(acc, x) = acc + x)  # 6
  sort(numbers)                           # [1, 2, 3]
  numbers |> each(fn(x) = print(x))       # prints each element

Safe access (never an error)
  first([])                      # None
  get(numbers, 10)               # None

"Change" a list: you get a new list, the old one stays the same
  set_at(numbers, 0, 9)          # [9, 1, 2]
  insert_at(numbers, 1, 7)       # [3, 7, 1, 2]
  remove_at(numbers, -1)         # [3, 1]

Pieces of a list
  take_while(numbers, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

Take a list apart with patterns
  match numbers
    [] -> "empty"
    [first, ..rest] -> "{first}, then {len(rest)} more"
  end

All the functions: rythme help builtins"#;

const LISTS_FR: &str = r#"LISTES

  nombres = [3, 1, 2]
  nombres[0]                     # 3 (le premier ; -1 est le dernier)
  len(nombres)                   # 3
  1..5                           # [1, 2, 3, 4, 5]
  [0, ..nombres]                 # [0, 3, 1, 2]   (.. insère toute une liste)
  nombres + [9]                  # [3, 1, 2, 9]

Travailler sur toute la liste (pas besoin de boucles)
  map(nombres, fn(x) = x * 10)            # [30, 10, 20]
  filter(nombres, fn(x) = x > 1)          # [3, 2]
  fold(nombres, 0, fn(acc, x) = acc + x)  # 6
  sort(nombres)                           # [1, 2, 3]
  nombres |> each(fn(x) = print(x))       # affiche chaque élément

Accès sûr (jamais d'erreur)
  first([])                      # None
  get(nombres, 10)               # None

« Modifier » une liste : on obtient une nouvelle liste, l'ancienne ne change pas
  set_at(nombres, 0, 9)          # [9, 1, 2]
  insert_at(nombres, 1, 7)       # [3, 7, 1, 2]
  remove_at(nombres, -1)         # [3, 1]

Des morceaux d'une liste
  take_while(nombres, fn(x) = x > 2)      # [3]
  chunks([1, 2, 3, 4, 5], 2)              # [[1, 2], [3, 4], [5]]
  windows([1, 2, 3], 2)                   # [[1, 2], [2, 3]]
  0..10 |> step_by(5)                     # [0, 5, 10]

Décomposer une liste avec des motifs
  match nombres
    [] -> "vide"
    [premier, ..reste] -> "{premier}, puis {len(reste)} autres"
  end

Toutes les fonctions : rythme help builtins"#;

const TEXT_EN: &str = r#"TEXT

  name = "Ada"
  "Hello, {name}!"               # "Hello, Ada!"  ({...} inserts any expression)
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3 (characters, accents included)
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("hello", "ell")       # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14" (a number with 2 decimals)
  pad_left("7", 3, "0")          # "007"  (and pad_right: to line up columns)
  char_code("A")   char(66)      # 65 and "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...): a conversion that cannot crash

Special characters: \n (new line), \t (tab), \r (carriage return),
\0 (null character), \" (quote), \\ (backslash), \{ and \} (braces, because
{ starts an insertion), \u{e9} (any character by its Unicode number: é)."#;

const TEXT_FR: &str = r#"TEXTE

  nom = "Ada"
  "Bonjour, {nom} !"             # "Bonjour, Ada !"  ({...} insère n'importe quelle expression)
  "2 + 2 = {2 + 2}"              # "2 + 2 = 4"
  "a" + "b"                      # "ab"
  len("été")                     # 3 (caractères, accents compris)
  upper("abc")   lower("ABC")   trim("  x  ")
  split("a,b,c", ",")            # ["a", "b", "c"]
  join(["a", "b"], "-")          # "a-b"
  replace("2024", "4", "5")      # "2025"
  contains("bonjour", "jour")    # true
  text(42)                       # "42"
  text(3.14159, 2)               # "3.14" (un nombre avec 2 décimales)
  pad_left("7", 3, "0")          # "007"  (et pad_right : pour aligner des colonnes)
  char_code("A")   char(66)      # 65 et "B"
  int("42")                      # 42
  parse_int("x42")               # Err(...) : une conversion qui ne plante jamais

Caractères spéciaux : \n (nouvelle ligne), \t (tabulation), \r (retour
chariot), \0 (caractère nul), \" (guillemet), \\ (barre oblique inverse),
\{ et \} (accolades, car { commence une insertion), \u{e9} (tout caractère
par son numéro Unicode : é)."#;

const RECORDS_EN: &str = r#"RECORDS

A record groups named values:
  ada = {name: "Ada", born: 1815}
  ada.name                       # "Ada"
  ada["name"]                    # "Ada"

Records never change: build a new one from an old one
  older = {..ada, born: 1816}    # same fields, `born` replaced

Take apart
  {name, born} = ada
  fn greet({name}) = "Hello {name}"

Useful functions
  keys(ada)   values(ada)   has(ada, "name")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["name", "Ada"], ["born", 1815]]
  from_entries([["x", 1]])       # {x: 1}

Records as dictionaries
  counts = {apple: 2}
  put(counts, "pear", 5)         # {apple: 2, pear: 5}
  map_values(counts, fn(n) = n * 10)      # {apple: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}

JSON, the format of data on the web
  parse_json(text)               # Ok(...): objects become records, arrays lists
  to_json(value)                 # the text, ready to save with write_file"#;

const RECORDS_FR: &str = r#"ENREGISTREMENTS

Un enregistrement regroupe des valeurs nommées :
  ada = {nom: "Ada", naissance: 1815}
  ada.nom                        # "Ada"
  ada["nom"]                     # "Ada"

Les enregistrements ne changent jamais : on en construit un nouveau
  autre = {..ada, naissance: 1816}   # mêmes champs, `naissance` remplacé

Décomposer
  {nom, naissance} = ada
  fn saluer({nom}) = "Bonjour {nom}"

Fonctions utiles
  keys(ada)   values(ada)   has(ada, "nom")   get(ada, "x")   put(ada, "x", 1)
  entries(ada)                   # [["nom", "Ada"], ["naissance", 1815]]
  from_entries([["x", 1]])       # {x: 1}

Les enregistrements comme dictionnaires
  compte = {pomme: 2}
  put(compte, "poire", 5)        # {pomme: 2, poire: 5}
  map_values(compte, fn(n) = n * 10)      # {pomme: 20}
  group_by(["a", "b", "a"], fn(x) = x) |> map_values(len)   # {a: 2, b: 1}

JSON, le format des données du web
  parse_json(texte)              # Ok(...) : les objets deviennent des enregistrements, les tableaux des listes
  to_json(valeur)                # le texte, prêt à enregistrer avec write_file"#;

const TYPES_EN: &str = r#"YOUR OWN TYPES

A type lists the possible shapes of a value:
  type Shape = Circle(radius) | Rect(width, height) | Empty

  c = Circle(2.0)
  c.radius                       # 2.0

Handle each shape with match
  fn area(shape) = match shape
    Circle(r) -> pi * r * r
    Rect(w, h) -> w * h
    Empty -> 0
  end

Or with one definition per shape
  fn perimeter(Circle(r)) = 2 * pi * r
  fn perimeter(Rect(w, h)) = 2 * (w + h)
  fn perimeter(Empty) = 0

If you forget a shape, `rythme check` tells you which one.

A type can contain itself (trees...):
  type Tree = Leaf | Node(left, value, right)"#;

const TYPES_FR: &str = r#"VOS PROPRES TYPES

Un type liste les formes possibles d'une valeur :
  type Forme = Cercle(rayon) | Rect(largeur, hauteur) | Vide

  c = Cercle(2.0)
  c.rayon                        # 2.0

Traiter chaque forme avec match
  fn aire(forme) = match forme
    Cercle(r) -> pi * r * r
    Rect(l, h) -> l * h
    Vide -> 0
  end

Ou avec une définition par forme
  fn perimetre(Cercle(r)) = 2 * pi * r
  fn perimetre(Rect(l, h)) = 2 * (l + h)
  fn perimetre(Vide) = 0

Si vous oubliez une forme, `rythme check` vous dit laquelle.

Un type peut se contenir lui-même (arbres...) :
  type Arbre = Feuille | Noeud(gauche, valeur, droite)"#;

const MATCH_EN: &str = r#"MATCH AND PATTERNS

`match` compares a value with patterns, in order, and gives the result of the
first one that fits:

  fn describe(x) = match x
    0 -> "zero"
    1 | 2 | 3 -> "small"
    n if n < 0 -> "negative"
    _ -> "big"
  end

Patterns
  42   "text"   true             exactly this value
  x                              anything, named x
  _                              anything, ignored
  Some(x)   Circle(r)            a constructor and what it contains
  []   [a, b]   [first, ..rest]  lists, by shape
  {name, age}                    a record with these fields
  p if condition                 a pattern and a condition (guard)
  a | b                          one pattern or the other

Patterns work everywhere a name is given:
  [x, y] = [1, 2]
  fn head_or_zero([]) = 0
  fn head_or_zero([x, ..]) = x"#;

const MATCH_FR: &str = r#"MATCH ET MOTIFS

`match` compare une valeur à des motifs, dans l'ordre, et donne le résultat
du premier qui correspond :

  fn decrire(x) = match x
    0 -> "zéro"
    1 | 2 | 3 -> "petit"
    n if n < 0 -> "négatif"
    _ -> "grand"
  end

Motifs
  42   "texte"   true            exactement cette valeur
  x                              n'importe quoi, nommé x
  _                              n'importe quoi, ignoré
  Some(x)   Cercle(r)            un constructeur et ce qu'il contient
  []   [a, b]   [premier, ..reste]   des listes, selon leur forme
  {nom, age}                     un enregistrement avec ces champs
  m if condition                 un motif et une condition (garde)
  a | b                          un motif ou l'autre

Les motifs marchent partout où l'on donne un nom :
  [x, y] = [1, 2]
  fn tete_ou_zero([]) = 0
  fn tete_ou_zero([x, ..]) = x"#;

const ERRORS_EN: &str = r#"ERRORS ARE VALUES (Option, Result, ?)

Rythme has no null and no exceptions. A value that may be missing is an
Option: Some(value) or None. An operation that may fail gives a Result:
Ok(value) or Err(message). You cannot forget to handle them.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)

  match divide(10, 2)
    Ok(x) -> print("result: {x}")
    Err(e) -> print("error: {e}")
  end

The ? operator: take the value, or stop and pass the error on
  fn average(a, b)
    x = parse_float(a)?          # if this is Err(...), average returns it
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

Quick tools
  unwrap_or(get(list, 5), 0)     # the value, or a default
  unwrap(Some(3))                # 3 (stops the program on None or Err)
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(person, "age"), "no age")   # Err("no age") if missing: ready for ?
  and_then(x, f)                 # one more step that may fail
  map_err(r, fn(e) = "config: {e}")     # explain an error better

To stop the whole program: fail("message")."#;

const ERRORS_FR: &str = r#"LES ERREURS SONT DES VALEURS (Option, Result, ?)

Rythme n'a ni null ni exceptions. Une valeur qui peut manquer est une
Option : Some(valeur) ou None. Une opération qui peut échouer donne un
Result : Ok(valeur) ou Err(message). Impossible d'oublier de les traiter.

  fn diviser(a, b) = if b == 0 then Err("division par zéro") else Ok(a / b)

  match diviser(10, 2)
    Ok(x) -> print("résultat : {x}")
    Err(e) -> print("erreur : {e}")
  end

L'opérateur ? : prendre la valeur, ou s'arrêter et transmettre l'erreur
  fn moyenne(a, b)
    x = parse_float(a)?          # si c'est Err(...), moyenne le renvoie
    y = parse_float(b)?
    Ok((x + y) / 2)
  end

Outils rapides
  unwrap_or(get(liste, 5), 0)    # la valeur, ou une valeur par défaut
  unwrap(Some(3))                # 3 (arrête le programme sur None ou Err)
  is_some(x)   is_none(x)   is_ok(x)   is_err(x)
  map(Some(2), fn(x) = x * 10)   # Some(20)
  ok_or(get(personne, "age"), "pas d'âge")   # Err("pas d'âge") s'il manque : prêt pour ?
  and_then(x, f)                 # une étape de plus qui peut échouer
  map_err(r, fn(e) = "config : {e}")    # mieux expliquer une erreur

Pour arrêter tout le programme : fail("message")."#;

const FUNCTIONAL_EN: &str = r#"THINKING FUNCTIONALLY

In Rythme, values never change and there are no loops. It may seem strange,
but programs become easier to read, test and trust: a function always gives
the same result for the same arguments.

Instead of a loop that prints:       1..5 |> each(fn(i) = print(i))
Instead of a loop that transforms:   map(list, f)
Instead of a loop that selects:      filter(list, f)
Instead of a loop that accumulates:  fold(list, start, f)   or sum, product
Instead of running totals:           scan(list, start, f)
Instead of a loop that searches:     find, any, all, index_of, min_by, max_by
Instead of "break":                  take_while(list, f)
Instead of "while":                  a recursive function

  # Python:  total = 0
  #          for x in numbers: total += x * x
  total = numbers |> map(fn(x) = x * x) |> sum

  # A "while" loop as a recursive function:
  fn collatz(1, steps) = steps
  fn collatz(n, steps) =
    if n % 2 == 0 then collatz(n // 2, steps + 1) else collatz(3 * n + 1, steps + 1)
  collatz(27, 0)                 # 111

Instead of changing a value, make a new one:
  x2 = x + 1
  person2 = {..person, age: 37}
  list2 = push(list, 4)"#;

const FUNCTIONAL_FR: &str = r#"PENSER FONCTIONNEL

En Rythme, les valeurs ne changent jamais et il n'y a pas de boucles. Cela
peut surprendre, mais les programmes deviennent plus faciles à lire, à tester
et à croire : une fonction donne toujours le même résultat pour les mêmes
arguments.

Au lieu d'une boucle qui affiche :      1..5 |> each(fn(i) = print(i))
Au lieu d'une boucle qui transforme :   map(liste, f)
Au lieu d'une boucle qui sélectionne :  filter(liste, f)
Au lieu d'une boucle qui accumule :     fold(liste, depart, f)   ou sum, product
Au lieu de totaux cumulés :             scan(liste, depart, f)
Au lieu d'une boucle qui cherche :      find, any, all, index_of, min_by, max_by
Au lieu de « break » :                  take_while(liste, f)
Au lieu de « tant que » :               une fonction récursive

  # Python :  total = 0
  #           for x in nombres: total += x * x
  total = nombres |> map(fn(x) = x * x) |> sum

  # Une boucle « tant que » en fonction récursive :
  fn collatz(1, etapes) = etapes
  fn collatz(n, etapes) =
    if n % 2 == 0 then collatz(n // 2, etapes + 1) else collatz(3 * n + 1, etapes + 1)
  collatz(27, 0)                 # 111

Au lieu de modifier une valeur, faites-en une nouvelle :
  x2 = x + 1
  personne2 = {..personne, age: 37}
  liste2 = push(liste, 4)"#;

const TESTS_EN: &str = r#"TESTS

Write tests next to your code:
  fn double(x) = x * 2

  test "double of 2 is 4" = assert_eq(double(2), 4)

  test "several checks"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "should stay negative")
  end

Run them:
  rythme test my_file.ry

While testing, top-level expressions such as print(...) are not run: the
definitions are loaded, then each test runs. A test passes if it ends without
error, and its value is not false, Err(...) or None."#;

const TESTS_FR: &str = r#"TESTS

Écrivez les tests à côté de votre code :
  fn double(x) = x * 2

  test "le double de 2 est 4" = assert_eq(double(2), 4)

  test "plusieurs vérifications"
    assert_eq(double(0), 0)
    assert(double(-1) < 0, "doit rester négatif")
  end

Lancez-les :
  rythme test mon_fichier.ry

Pendant les tests, les expressions du premier niveau comme print(...) ne sont
pas exécutées : les définitions sont chargées, puis chaque test est lancé. Un
test réussit s'il se termine sans erreur, et que sa valeur n'est ni false, ni
Err(...), ni None."#;

const MODULES_EN: &str = r#"SEVERAL FILES

  # geometry.ry
  fn area(w, h) = w * h

  # main.ry
  use "geometry.ry"
  print(area(2, 3))

The path is relative to the file that contains `use`. Everything defined at
the top level of the used file becomes available. Each file runs only once."#;

const MODULES_FR: &str = r#"PLUSIEURS FICHIERS

  # geometrie.ry
  fn aire(l, h) = l * h

  # principal.ry
  use "geometrie.ry"
  print(aire(2, 3))

Le chemin est relatif au fichier qui contient `use`. Tout ce qui est défini au
premier niveau du fichier utilisé devient disponible. Chaque fichier n'est
exécuté qu'une fois."#;

const EDITORS_EN: &str = r#"RYTHME IN YOUR EDITOR

Rythme helps inside the editor too: problems as you type, the documentation
of a function under the mouse, completion, the parameters of a call, go to
definition, references, rename, quick fixes for typos, formatting like
`rythme fmt`. This is the language server, that the editor starts by itself:
  rythme lsp

VS Code
  the extension in editors/vscode: colors, snippets and the language server.
  Settings: rythme.path (the program), rythme.language (en, fr or auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  every editor that speaks the Language Server Protocol: tell it to start
  `rythme lsp` for .ry files. Ready-made configurations: editors/README.md

The language of the messages follows the editor, unless `--lang fr|en` or
RYTHME_LANG chooses it: rythme --lang fr lsp"#;

const EDITORS_FR: &str = r#"RYTHME DANS VOTRE ÉDITEUR

Rythme aide aussi dans l'éditeur : les problèmes pendant la frappe, la
documentation d'une fonction sous la souris, la complétion, les paramètres
d'un appel, aller à la définition, les références, renommer, les corrections
rapides des fautes de frappe, la mise en forme comme `rythme fmt`. C'est le
serveur de langage, que l'éditeur lance tout seul :
  rythme lsp

VS Code
  l'extension dans editors/vscode : couleurs, extraits de code et serveur de
  langage. Réglages : rythme.path (le programme), rythme.language (en, fr ou auto).

Neovim, Helix, Zed, Emacs, Sublime Text, Kate...
  tout éditeur qui parle le Language Server Protocol : demandez-lui de lancer
  `rythme lsp` pour les fichiers .ry. Configurations prêtes : editors/README.fr.md

La langue des messages suit celle de l'éditeur, sauf si `--lang fr|en` ou
RYTHME_LANG la choisit : rythme --lang fr lsp"#;

const REPL_EN: &str = r#"THE INTERACTIVE MODE

Run `rythme` without a file. Type an expression and see its value:
  ry> 1 + 2
  3
Definitions are kept from one line to the next. If a line is not finished
(open bracket, block without `end`...), it continues on the next line.

Commands
  :help           this help              :quit          leave (or Ctrl-D)
  :doc map        help on a name         :load f.ry     load a file
  :reset          forget everything      :lang fr|en    change the language
  :learn          guided tour            :explain E003  explain an error
The Tab key completes names."#;

const REPL_FR: &str = r#"LE MODE INTERACTIF

Lancez `rythme` sans fichier. Tapez une expression et voyez sa valeur :
  ry> 1 + 2
  3
Les définitions sont gardées d'une ligne à l'autre. Si une ligne n'est pas
finie (parenthèse ouverte, bloc sans `end`...), elle continue à la ligne
suivante.

Commandes
  :help           cette aide             :quit          quitter (ou Ctrl-D)
  :doc map        aide sur un nom        :load f.ry     charger un fichier
  :reset          tout oublier           :lang fr|en    changer de langue
  :learn          visite guidée          :explain E003  expliquer une erreur
La touche Tab complète les noms."#;

const FMT_EN: &str = r#"FORMATTING: rythme fmt

`rythme fmt` gives code the official layout of Rythme: every program looks
the same, and nobody has to argue about spaces.

  rythme fmt hello.ry            format a file (it is rewritten)
  rythme fmt .                   every .ry file of a folder and its subfolders
  rythme fmt --check .           only check: lists the files to format and
                                 fails (exit code 1) if there are any, for CI
  rythme fmt - < hello.ry        reads the code, writes the result

The style
  - 2 spaces per level of indentation
  - spaces around operators: x = a + b * 2   xs |> map(f)   n -> n * 2
  - none inside brackets or around ..: f(x)   [1, 2]   {name: "Ada"}   1..10
  - a pipeline written on several lines: one step per line
      total = prices
        |> filter(fn(p) = p > 10)
        |> sum
  - a type written on several lines: one variant per line
      type Shape =
        | Circle(radius)
        | Rect(width, height)
  - a list written on several lines ends with a comma
  - one statement per line (no `;`), and `end` on its own line
  - a blank line around a definition written on several lines; never two
    blank lines in a row
  - every comment is kept; the comments at the end of consecutive lines
    are aligned

Line breaks are kept where the language allows a choice, and long lines are
kept as written. The formatter never changes what a program does: a file with
a syntax error is left as it is, and the error is shown."#;

const FMT_FR: &str = r#"MISE EN FORME : rythme fmt

`rythme fmt` donne au code la mise en forme officielle de Rythme : tous les
programmes se ressemblent, et personne n'a à discuter des espaces.

  rythme fmt bonjour.ry          mettre en forme un fichier (il est réécrit)
  rythme fmt .                   tous les fichiers .ry d'un dossier et de ses
                                 sous-dossiers
  rythme fmt --check .           vérifier seulement : liste les fichiers à
                                 mettre en forme et échoue (code de sortie 1)
                                 s'il y en a, pour l'intégration continue
  rythme fmt - < bonjour.ry      lit le code, écrit le résultat

Le style
  - 2 espaces par niveau d'indentation
  - des espaces autour des opérateurs : x = a + b * 2   xs |> map(f)   n -> n * 2
  - aucun dans les crochets ni autour de .. : f(x)   [1, 2]   {nom: "Ada"}   1..10
  - un enchaînement écrit sur plusieurs lignes : une étape par ligne
      total = prix
        |> filter(fn(p) = p > 10)
        |> sum
  - un type écrit sur plusieurs lignes : une variante par ligne
      type Forme =
        | Cercle(rayon)
        | Rect(largeur, hauteur)
  - une liste écrite sur plusieurs lignes finit par une virgule
  - une instruction par ligne (pas de `;`), et `end` sur sa propre ligne
  - une ligne vide autour d'une définition écrite sur plusieurs lignes ;
    jamais deux lignes vides de suite
  - chaque commentaire est gardé ; les commentaires en fin de lignes
    consécutives sont alignés

Les retours à la ligne sont gardés là où le langage laisse le choix, et les
lignes longues sont gardées telles quelles. La mise en forme ne change jamais
ce que fait un programme : un fichier avec une erreur de syntaxe est laissé
tel quel, et l'erreur est affichée."#;

// ----- error explanations -----------------------------------------------

/// Long explanation of an error or warning code.
pub fn explain(code: &str) -> Option<String> {
    let code = code.trim().to_uppercase();
    let (en, fr): (&str, &str) = match code.as_str() {
        "E001" => (
            r#"E001 — Syntax error

The code does not follow the grammar of Rythme at this place. The message
says what was expected. Frequent causes:
  - a missing `)`, `]` or `,`
  - `then` or `else` missing in an `if`
  - braces { } used for a block (Rythme uses a new line and `end`)
  - a keyword from another language (for, while, return, let, def...)

Wrong:   if x > 0 { print(x) }
Right:   if x > 0 then print(x) else print("not positive")"#,
            r#"E001 — Erreur de syntaxe

Le code ne suit pas la grammaire de Rythme à cet endroit. Le message dit ce
qui était attendu. Causes fréquentes :
  - un `)`, `]` ou `,` manquant
  - `then` ou `else` manquant dans un `if`
  - des accolades { } pour un bloc (Rythme utilise une nouvelle ligne et `end`)
  - un mot-clé d'un autre langage (for, while, return, let, def...)

Faux :   if x > 0 { print(x) }
Juste :  if x > 0 then print(x) else print("pas positif")"#,
        ),
        "E002" => (
            r#"E002 — Text never closed

A text starts with " and must end with " on the same text. A `{` inside a
text starts an insertion and must be closed by `}`.

Wrong:   print("hello)
Right:   print("hello")
To write a brace in a text: "\{not inserted\}""#,
            r#"E002 — Texte jamais fermé

Un texte commence par " et doit finir par ". Une `{` dans un texte commence
une insertion et doit être fermée par `}`.

Faux :   print("bonjour)
Juste :  print("bonjour")
Pour écrire une accolade dans un texte : "\{pas inséré\}""#,
        ),
        "E003" => (
            r#"E003 — Unknown name

This name is not defined at this place. Check the spelling (uppercase and
lowercase letters are different), and that the name is defined before it is
used. Inside a function, the parameters and the names defined above are
visible; at the top level, all functions are visible everywhere.

Wrong:   print(lenght([1, 2]))
Right:   print(len([1, 2]))

Rythme suggests the closest existing name when there is one."#,
            r#"E003 — Nom inconnu

Ce nom n'est pas défini à cet endroit. Vérifiez l'orthographe (majuscules et
minuscules sont différentes), et que le nom est défini avant d'être utilisé.
Dans une fonction, les paramètres et les noms définis au-dessus sont
visibles ; au premier niveau, toutes les fonctions sont visibles partout.

Faux :   print(lenght([1, 2]))
Juste :  print(len([1, 2]))

Rythme propose le nom existant le plus proche quand il y en a un."#,
        ),
        "E004" => (
            r#"E004 — Wrong number of arguments

The function is called with more or fewer values than it takes. Remember that
with `|>`, the value on the left is the first argument.

Wrong:   fn add(a, b) = a + b
         add(1)
Right:   add(1, 2)
Also right:  1 |> add(2)

`rythme help NAME` shows how to call a built-in function."#,
            r#"E004 — Mauvais nombre d'arguments

La fonction est appelée avec plus ou moins de valeurs qu'elle n'en prend.
Avec `|>`, la valeur de gauche est le premier argument.

Faux :   fn ajouter(a, b) = a + b
         ajouter(1)
Juste :  ajouter(1, 2)
Juste aussi :  1 |> ajouter(2)

`rythme help NOM` montre comment appeler une fonction intégrée."#,
        ),
        "E005" => (
            r#"E005 — Wrong kind of value

The operation does not work with this kind of value. Rythme never converts
values silently: "1" + 1 is an error, not "11" or 2.

Wrong:   "age: " + 36
Right:   "age: {36}"              (insert the value in the text)
Right:   "age: " + text(36)       (convert it first)

Conditions must be true or false:
Wrong:   if len(list) then ... else ...
Right:   if len(list) > 0 then ... else ..."#,
            r#"E005 — Mauvaise sorte de valeur

L'opération ne fonctionne pas avec cette sorte de valeur. Rythme ne convertit
jamais les valeurs en silence : "1" + 1 est une erreur, ni "11" ni 2.

Faux :   "âge : " + 36
Juste :  "âge : {36}"             (insérer la valeur dans le texte)
Juste :  "âge : " + text(36)      (la convertir d'abord)

Les conditions doivent être true ou false :
Faux :   if len(liste) then ... else ...
Juste :  if len(liste) > 0 then ... else ..."#,
        ),
        "E006" => (
            r#"E006 — No case matches

None of the cases of a `match` (or of the definitions of a function) accepts
this value.

Wrong:   fn name(1) = "one"
         name(2)
Right:   fn name(1) = "one"
         fn name(_) = "another number"

`rythme check` warns about forgotten cases before running (W002)."#,
            r#"E006 — Aucun cas ne correspond

Aucun des cas d'un `match` (ou des définitions d'une fonction) n'accepte
cette valeur.

Faux :   fn nom(1) = "un"
         nom(2)
Juste :  fn nom(1) = "un"
         fn nom(_) = "un autre nombre"

`rythme check` prévient des cas oubliés avant l'exécution (W002)."#,
        ),
        "E007" => (
            r#"E007 — Index out of range

Positions in a list or a text go from 0 to length - 1 (and -1 is the last).

Wrong:   [10, 20][2]
Right:   [10, 20][1]              # 20
Safe:    get([10, 20], 2)         # None, never an error"#,
            r#"E007 — Indice hors limites

Les positions dans une liste ou un texte vont de 0 à longueur - 1 (et -1 est
le dernier).

Faux :   [10, 20][2]
Juste :  [10, 20][1]              # 20
Sûr :    get([10, 20], 2)         # None, jamais d'erreur"#,
        ),
        "E008" => (
            r#"E008 — Division by zero

Dividing by zero has no meaning. Check the divisor before, or return a Result.

  fn divide(a, b) = if b == 0 then Err("division by zero") else Ok(a / b)"#,
            r#"E008 — Division par zéro

Diviser par zéro n'a pas de sens. Vérifiez le diviseur avant, ou renvoyez un
Result.

  fn diviser(a, b) = if b == 0 then Err("division par zéro") else Ok(a / b)"#,
        ),
        "E009" => (
            r#"E009 — Not a function

Parentheses after a value call it, but only functions can be called.

Wrong:   x = 5
         x(3)
Wrong:   None()                   (None is a value)
Right:   None
To multiply, write it:  2 * (x + 1)"#,
            r#"E009 — Pas une fonction

Des parenthèses après une valeur l'appellent, mais seules les fonctions
peuvent être appelées.

Faux :   x = 5
         x(3)
Faux :   None()                   (None est une valeur)
Juste :  None
Pour multiplier, écrivez-le :  2 * (x + 1)"#,
        ),
        "E010" => (
            r#"E010 — Unknown field

The record (or the constructor) has no field with this name.

Wrong:   {name: "Ada"}.nmae
Right:   {name: "Ada"}.name
Safe:    get(record, "name")     # an Option, never an error"#,
            r#"E010 — Champ inconnu

L'enregistrement (ou le constructeur) n'a pas de champ de ce nom.

Faux :   {nom: "Ada"}.nmo
Juste :  {nom: "Ada"}.nom
Sûr :    get(enregistrement, "nom")   # une Option, jamais d'erreur"#,
        ),
        "E011" => (
            r#"E011 — Already defined

In Rythme a name always means the same value: it cannot be changed or defined
twice in the same place. This is what makes programs easy to follow.

Wrong:   x = 1
         x = x + 1
Right:   x = 1
         y = x + 1

For a function with several cases, write the definitions one right after the
other:
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)"#,
            r#"E011 — Déjà défini

En Rythme, un nom désigne toujours la même valeur : il ne peut pas être
modifié ni défini deux fois au même endroit. C'est ce qui rend les
programmes faciles à suivre.

Faux :   x = 1
         x = x + 1
Juste :  x = 1
         y = x + 1

Pour une fonction à plusieurs cas, écrivez les définitions les unes juste
après les autres :
  fn fib(0) = 0
  fn fib(1) = 1
  fn fib(n) = fib(n - 1) + fib(n - 2)"#,
        ),
        "E012" => (
            r#"E012 — Number too large

Whole numbers (Int) go from -9223372036854775808 to 9223372036854775807.
Rythme stops instead of giving a wrong result. For bigger values, use
decimal numbers (Float), which are approximate:

  2 ** 100        # error E012
  2.0 ** 100      # 1.2676506002282294e30"#,
            r#"E012 — Nombre trop grand

Les nombres entiers (Int) vont de -9223372036854775808 à 9223372036854775807.
Rythme s'arrête plutôt que de donner un résultat faux. Pour des valeurs plus
grandes, utilisez des nombres décimaux (Float), qui sont approchés :

  2 ** 100        # erreur E012
  2.0 ** 100      # 1.2676506002282294e30"#,
        ),
        "E013" => (
            r#"E013 — unwrap found no value

`unwrap(x)` takes the value out of Some(...) or Ok(...), and stops the
program on None or Err(...). Handle the missing case instead:

  match get(list, 5)
    Some(x) -> x
    None -> 0
  end
or simply:  unwrap_or(get(list, 5), 0)"#,
            r#"E013 — unwrap n'a pas trouvé de valeur

`unwrap(x)` sort la valeur de Some(...) ou Ok(...), et arrête le programme
sur None ou Err(...). Traitez plutôt le cas manquant :

  match get(liste, 5)
    Some(x) -> x
    None -> 0
  end
ou simplement :  unwrap_or(get(liste, 5), 0)"#,
        ),
        "E014" => (
            r#"E014 — Too many nested calls

A function calls itself (or others) too many times without finishing.
Either it never reaches its base case, or it needs to be written so that the
recursive call is the very last thing it does (a tail call): those calls
have no limit.

Limited:     fn sum([]) = 0
             fn sum([x, ..rest]) = x + sum(rest)       # + happens after
No limit:    fn sum([], acc) = acc
             fn sum([x, ..rest], acc) = sum(rest, acc + x)
Simplest:    sum(list)  or  fold(list, 0, fn(a, x) = a + x)"#,
            r#"E014 — Trop d'appels imbriqués

Une fonction s'appelle elle-même (ou d'autres) trop de fois sans finir. Soit
elle n'atteint jamais son cas de base, soit il faut l'écrire pour que l'appel
récursif soit la toute dernière chose qu'elle fait (un appel terminal) : ces
appels n'ont pas de limite.

Limité :      fn somme([]) = 0
              fn somme([x, ..reste]) = x + somme(reste)       # le + vient après
Sans limite : fn somme([], acc) = acc
              fn somme([x, ..reste], acc) = somme(reste, acc + x)
Plus simple : sum(liste)  ou  fold(liste, 0, fn(a, x) = a + x)"#,
        ),
        "E015" => (
            r#"E015 — Assertion failed

`assert(condition)` or `assert_eq(actual, expected)` found that something is
not as expected. In a test, this means the test fails: the message shows the
values to compare."#,
            r#"E015 — Assertion échouée

`assert(condition)` ou `assert_eq(obtenu, attendu)` a constaté que quelque
chose n'est pas comme prévu. Dans un test, cela veut dire que le test échoue :
le message montre les valeurs à comparer."#,
        ),
        "E016" => (
            r#"E016 — Wrong use of ?

`x?` takes the value out of Some(...) or Ok(...); on None or Err(...), it makes
the current function return it immediately. So it only works inside a
function, on an Option or a Result.

  fn total(a, b)
    x = parse_int(a)?
    y = parse_int(b)?
    Ok(x + y)
  end"#,
            r#"E016 — Mauvaise utilisation de ?

`x?` sort la valeur de Some(...) ou Ok(...) ; sur None ou Err(...), il fait
immédiatement renvoyer cette valeur par la fonction en cours. Il ne marche
donc que dans une fonction, sur une Option ou un Result.

  fn total(a, b)
    x = parse_int(a)?
    y = parse_int(b)?
    Ok(x + y)
  end"#,
        ),
        "E017" => (
            r#"E017 — File problem

A file could not be read, or a file given to `use` was not found, or files
use each other in a circle. Paths in `use` are relative to the folder of the
file that contains the `use`. Programs are text files saved in UTF-8 (the
usual choice of editors).

`input()` also stops with this error when there is nothing more to read:
the keyboard input was closed (Ctrl-D), or the text sent to the program
(`rythme prog.ry < answers.txt`) has no more lines."#,
            r#"E017 — Problème de fichier

Un fichier n'a pas pu être lu, ou un fichier donné à `use` est introuvable,
ou des fichiers s'utilisent les uns les autres en cercle. Les chemins de
`use` sont relatifs au dossier du fichier qui contient le `use`. Les
programmes sont des fichiers texte enregistrés en UTF-8 (le choix habituel
des éditeurs).

`input()` s'arrête aussi avec cette erreur quand il n'y a plus rien à lire :
la saisie au clavier a été fermée (Ctrl-D), ou le texte envoyé au programme
(`rythme prog.ry < reponses.txt`) n'a plus de lignes."#,
        ),
        "E018" => (
            r#"E018 — Unexpected character

This character has no meaning in Rythme here. Texts use straight double
quotes "...", powers use **, and comments start with #."#,
            r#"E018 — Caractère inattendu

Ce caractère n'a pas de sens en Rythme ici. Les textes utilisent des
guillemets droits "...", les puissances **, et les commentaires commencent
par #."#,
        ),
        "E019" => (
            r#"E019 — if without else

In Rythme, `if` is an expression: it always gives a value, so both cases are
needed. Each branch holds a single expression; use `do ... end` for several.

Wrong:   if x > 0 then print("positive")
Right:   if x > 0 then print("positive") else ()
Right:   if x > 0 then "positive" else if x == 0 then "zero" else "negative""#,
            r#"E019 — if sans else

En Rythme, `if` est une expression : il donne toujours une valeur, il faut
donc les deux cas. Chaque branche contient une seule expression ; utilisez
`do ... end` pour en mettre plusieurs.

Faux :   if x > 0 then print("positif")
Juste :  if x > 0 then print("positif") else ()
Juste :  if x > 0 then "positif" else if x == 0 then "zéro" else "négatif""#,
        ),
        "E020" => (
            r#"E020 — Missing end

A function written on several lines, a `match` or a `do` block must finish
with `end` on its own line.

  fn greet(name)
    print("Hello {name}")
  end"#,
            r#"E020 — Il manque end

Une fonction écrite sur plusieurs lignes, un `match` ou un bloc `do` doit se
terminer par `end` sur sa propre ligne.

  fn saluer(nom)
    print("Bonjour {nom}")
  end"#,
        ),
        "E021" => (
            r#"E021 — Program stopped by fail

The program called `fail(message)`, which stops everything. For errors that
the caller could handle, return Err(message) instead."#,
            r#"E021 — Programme arrêté par fail

Le programme a appelé `fail(message)`, qui arrête tout. Pour les erreurs que
l'appelant pourrait traiter, renvoyez plutôt Err(message)."#,
        ),
        "E022" => (
            r#"E022 — Impossible conversion

The text does not contain a number. When the text comes from a user or a
file, use the safe versions, which give a Result:

  match parse_int(input("Age? "))
    Ok(age) -> print("Next year: {age + 1}")
    Err(e) -> print(e)
  end"#,
            r#"E022 — Conversion impossible

Le texte ne contient pas un nombre. Quand le texte vient d'un utilisateur ou
d'un fichier, utilisez les versions sûres, qui donnent un Result :

  match parse_int(input("Âge ? "))
    Ok(age) -> print("L'an prochain : {age + 1}")
    Err(e) -> print(e)
  end"#,
        ),
        "E023" => (
            r#"E023 — Invalid value

The function received a value of the right kind, but outside of what it
accepts: a negative count, the minimum of an empty list, a range too large...
The message explains the limit."#,
            r#"E023 — Valeur invalide

La fonction a reçu une valeur de la bonne sorte, mais en dehors de ce qu'elle
accepte : un nombre négatif d'éléments, le minimum d'une liste vide, un
intervalle trop grand... Le message explique la limite."#,
        ),
        "E024" => (
            r#"E024 — A value is lost

A line computes a value, but nothing uses it: only the last line of a block
gives its value, and values never change, so the result is simply lost.

Wrong:   total = price
         - discount              # a new line: this is a separate statement
Right:   total = price -
           discount              # end the line with the operator to continue
Wrong:   x + 1                   # does not change x
Right:   y = x + 1
To show a value, print it:  print(x + 1)"#,
            r#"E024 — Une valeur est perdue

Une ligne calcule une valeur, mais rien ne l'utilise : seule la dernière
ligne d'un bloc donne sa valeur, et les valeurs ne changent jamais, donc le
résultat est simplement perdu.

Faux :   total = prix
         - remise                # une nouvelle ligne : c'est une instruction à part
Juste :  total = prix -
           remise                # finissez la ligne par l'opérateur pour continuer
Faux :   x + 1                   # ne modifie pas x
Juste :  y = x + 1
Pour afficher une valeur, affichez-la :  print(x + 1)"#,
        ),
        "W001" => (
            r#"W001 — Unused name

A name is defined but never used. It is often a typo or a forgotten step.
If it is on purpose, start the name with `_`: `_unused = ...`."#,
            r#"W001 — Nom inutilisé

Un nom est défini mais jamais utilisé. C'est souvent une faute de frappe ou
une étape oubliée. Si c'est voulu, commencez le nom par `_` : `_inutile = ...`."#,
        ),
        "W002" => (
            r#"W002 — Forgotten case

A `match` (or a function with several definitions) does not handle every
possible value. The message lists what is missing. If that value comes, the
program will stop with E006.

  type Light = Red | Orange | Green
  fn next(Red) = Green
  fn next(Green) = Orange
  # W002: `next` does not handle `Orange`. Add the missing definition:
  fn next(Orange) = Red"#,
            r#"W002 — Cas oublié

Un `match` (ou une fonction à plusieurs définitions) ne traite pas toutes
les valeurs possibles. Le message liste ce qui manque. Si cette valeur
arrive, le programme s'arrêtera avec E006.

  type Feu = Rouge | Orange | Vert
  fn suivant(Rouge) = Vert
  fn suivant(Vert) = Orange
  # W002 : `suivant` ne traite pas `Orange`. Ajoutez la définition manquante :
  fn suivant(Orange) = Rouge"#,
        ),
        "W003" => (
            r#"W003 — Built-in function hidden

You defined a name that already belongs to a built-in function. In your file,
the name now means your definition. Choose another name to keep both."#,
            r#"W003 — Fonction intégrée masquée

Vous avez défini un nom qui appartient déjà à une fonction intégrée. Dans
votre fichier, le nom désigne maintenant votre définition. Choisissez un
autre nom pour garder les deux."#,
        ),
        "W004" => (
            r#"W004 — Case never reached

Cases are tried in order. A case that accepts everything (`_`, or a plain
name) catches every value, so the cases after it can never be used. Put it
last."#,
            r#"W004 — Cas jamais atteint

Les cas sont essayés dans l'ordre. Un cas qui accepte tout (`_`, ou un
simple nom) attrape toutes les valeurs : les cas suivants ne peuvent jamais
servir. Mettez-le en dernier."#,
        ),
        "W005" => (
            r#"W005 — Unknown function after a dot

`value.name(args)` calls the function `name(value, args)`, or the field `name`
of a record. No function has this name, and no record of the file has such a
field: it is probably a typo.

Wrong:   [1, 2].lenght()
Right:   [1, 2].len()"#,
            r#"W005 — Fonction inconnue après un point

`valeur.nom(args)` appelle la fonction `nom(valeur, args)`, ou le champ `nom`
d'un enregistrement. Aucune fonction ne porte ce nom, et aucun enregistrement
du fichier n'a un tel champ : c'est sans doute une faute de frappe.

Faux :   [1, 2].lenght()
Juste :  [1, 2].len()"#,
        ),
        _ => return None,
    };
    Some(pick_owned(en, fr))
}

fn pick_owned(en: &str, fr: &str) -> String {
    if is_fr() {
        fr.to_string()
    } else {
        en.to_string()
    }
}

pub const CODES: &[&str] = &[
    "E001", "E002", "E003", "E004", "E005", "E006", "E007", "E008", "E009", "E010", "E011", "E012", "E013", "E014", "E015", "E016", "E017",
    "E018", "E019", "E020", "E021", "E022", "E023", "E024", "W001", "W002", "W003", "W004", "W005",
];
