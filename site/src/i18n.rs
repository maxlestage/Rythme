//! The two languages of the site, French and English. The language of a page comes
//! from its URL (`/fr/…`, `/en/…`).
//!
//! Each language is a [`Texts`] constant: forgetting a translation is a compile error.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Lang {
    Fr,
    En,
}

impl Lang {
    /// In the order of the selector.
    pub const ALL: [Lang; 2] = [Lang::Fr, Lang::En];

    pub fn code(self) -> &'static str {
        self.pick(["fr", "en"])
    }

    pub fn name(self) -> &'static str {
        self.pick(["Français", "English"])
    }

    /// The Open Graph locale.
    pub fn locale(self) -> &'static str {
        self.pick(["fr_FR", "en_US"])
    }

    /// Picks the value of this language in `[fr, en]`.
    pub fn pick<T: Copy>(self, values: [T; 2]) -> T {
        values[match self {
            Lang::Fr => 0,
            Lang::En => 1,
        }]
    }

    /// `fr` or `en` exactly.
    pub fn parse(code: &str) -> Option<Lang> {
        Lang::ALL.into_iter().find(|l| l.code() == code)
    }

    /// The language of a tag such as `fr-CA` or `EN`.
    pub fn from_tag(tag: &str) -> Option<Lang> {
        Lang::parse(&tag.get(..2)?.to_ascii_lowercase())
    }

    pub fn texts(self) -> &'static Texts {
        self.pick([&FR, &EN])
    }
}

/// The preferred supported language of an `Accept-Language` header, French by default.
pub fn pick_lang(header: &str) -> Lang {
    let mut ranked: Vec<(Lang, f32)> = header
        .split(',')
        .filter_map(|part| {
            let mut pieces = part.trim().split(";q=");
            let lang = Lang::from_tag(pieces.next()?)?;
            let q = pieces.next().and_then(|q| q.trim().parse().ok()).unwrap_or(1.0);
            Some((lang, q))
        })
        .filter(|(_, q)| *q > 0.0)
        .collect();
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1));
    ranked.first().map_or(Lang::Fr, |r| r.0)
}

/// A title and its text.
pub struct Item {
    pub title: &'static str,
    pub text: &'static str,
}

const fn item(title: &'static str, text: &'static str) -> Item {
    Item { title, text }
}

/// A habit from another language and the Rythme way.
pub struct Habit {
    pub other: &'static str,
    pub rythme: &'static str,
    pub note: &'static str,
}

const fn habit(other: &'static str, rythme: &'static str, note: &'static str) -> Habit {
    Habit { other, rythme, note }
}

/// A step of the tour of the language: a title, a text and code that runs.
pub struct Step {
    pub title: &'static str,
    pub text: &'static str,
    pub code: &'static str,
}

const fn step(title: &'static str, text: &'static str, code: &'static str) -> Step {
    Step { title, text, code }
}

pub struct Texts {
    // Meta
    pub meta_title: &'static str,
    pub meta_description: &'static str,
    // Navigation
    pub skip: &'static str,
    pub brand_aria: &'static str,
    pub nav_aria: &'static str,
    pub nav_learn: &'static str,
    pub nav_examples: &'static str,
    pub nav_playground: &'static str,
    pub nav_teach: &'static str,
    pub menu: &'static str,
    pub lang_aria: &'static str,
    // Home: hero
    pub hero_eyebrow: &'static str,
    pub hero_title: &'static str,
    pub hero_lead: &'static str,
    pub cta_try: &'static str,
    pub cta_install: &'static str,
    pub hero_code: &'static str,
    pub hero_output: &'static str,
    // Home: why
    pub why_title: &'static str,
    pub why_lead: &'static str,
    pub why: [Item; 4],
    // Home: errors
    pub errors_title: &'static str,
    pub errors_lead: &'static str,
    pub errors_code: &'static str,
    pub errors_message: &'static str,
    pub errors_points: [&'static str; 3],
    // Home: habits from other languages
    pub habits_title: &'static str,
    pub habits_lead: &'static str,
    pub habits_head: [&'static str; 3],
    pub habits: [Habit; 6],
    // Home: install and commands
    pub install_title: &'static str,
    pub install_lead: &'static str,
    pub install_steps: [Item; 3],
    pub commands_title: &'static str,
    pub commands: [(&'static str, &'static str); 8],
    pub commands_lang: &'static str,
    pub copy: &'static str,
    pub copied: &'static str,
    // Home: call to action
    pub cta_title: &'static str,
    pub cta_text: &'static str,
    // Footer
    pub footer_tagline: &'static str,
    pub footer_made: &'static str,
    pub footer_license: &'static str,
    pub footer_source: &'static str,
    // Playground
    pub pg_title: &'static str,
    pub pg_lead: &'static str,
    pub pg_example: &'static str,
    pub pg_welcome: &'static str,
    pub pg_run: &'static str,
    pub pg_check: &'static str,
    pub pg_test: &'static str,
    pub pg_stop: &'static str,
    pub pg_code: &'static str,
    pub pg_output: &'static str,
    pub pg_help: &'static str,
    pub pg_help_intro: &'static str,
    pub pg_search: &'static str,
    pub pg_search_go: &'static str,
    pub pg_limits: &'static str,
    pub pg_noscript: &'static str,
    pub pg_welcome_code: &'static str,
    // Examples
    pub ex_title: &'static str,
    pub ex_lead: &'static str,
    pub ex_output: &'static str,
    pub ex_tests: &'static str,
    pub ex_open: &'static str,
    pub ex_jump: &'static str,
    // Learn
    pub learn_title: &'static str,
    pub learn_lead: &'static str,
    pub learn_try: &'static str,
    pub tour: [Step; 8],
    pub learn_names: &'static str,
    pub learn_next_title: &'static str,
    pub learn_next: [Item; 3],
    // Teach
    pub teach_lead: &'static str,
    pub teach_toc: &'static str,
    // Not found
    pub nf_title: &'static str,
    pub nf_text: &'static str,
    pub nf_home: &'static str,
}

pub const FR: Texts = Texts {
    meta_title: "Rythme — un langage simple, purement fonctionnel, avec de l'aide partout",
    meta_description: "Rythme est un langage de programmation simple et purement fonctionnel : 15 mots-clés, pas de null, pas d'exceptions, et des erreurs qui expliquent comment les corriger. Essayez-le dans le navigateur.",
    skip: "Aller au contenu",
    brand_aria: "Rythme, accueil",
    nav_aria: "Navigation principale",
    nav_learn: "Apprendre",
    nav_examples: "Exemples",
    nav_playground: "Playground",
    nav_teach: "Enseigner",
    menu: "Menu",
    lang_aria: "Langue",
    hero_eyebrow: "Langage de programmation · v0.1",
    hero_title: "Un langage simple, où chaque erreur vous aide.",
    hero_lead: "Rythme est purement fonctionnel : les valeurs ne changent jamais, tout est fonction, il n'y a ni boucles, ni null, ni exceptions. Et l'interpréteur explique chaque erreur, en français ou en anglais.",
    cta_try: "Essayer dans le navigateur",
    cta_install: "Installer",
    hero_code: "# Les valeurs ne changent jamais. Tout est fonction.\nfn carre(x) = x * x\n\n[1, 2, 3, 4]\n  |> filter(is_even)      # garder les nombres pairs\n  |> map(carre)           # mettre chacun au carré\n  |> sum                  # les additionner\n  |> print\n",
    hero_output: "Résultat",
    why_title: "Pourquoi Rythme ?",
    why_lead: "Moins de concepts à apprendre, moins de bugs à traquer, et de l'aide à chaque pas.",
    why: [
        item(
            "Plus simple que Python",
            "15 mots-clés (Python en a 35) et une seule façon de faire chaque chose. Pas de classes, pas de boucles, pas de modification.",
        ),
        item(
            "Purement fonctionnel",
            "Valeurs immuables, fonctions comme valeurs, chaque if et chaque match est une expression. La récursion terminale n'épuise jamais la pile.",
        ),
        item(
            "Sûr comme Rust",
            "Pas de null : une Option. Pas d'exceptions : un Result, et ? pour transmettre l'erreur. Types algébriques, filtrage par motifs, cas oubliés signalés.",
        ),
        item(
            "De l'aide partout",
            "Chaque erreur dit où, pourquoi et comment corriger. rythme explain, rythme help, rythme learn : tout existe en français et en anglais.",
        ),
    ],
    errors_title: "Des erreurs qui enseignent",
    errors_lead: "Une faute de frappe ? Rythme montre l'endroit exact, explique le problème et propose le bon nom, comme le compilateur de Rust.",
    errors_code: "fn carre(x) = x * x\nprint(caree(4))\n",
    errors_message: "erreur[E003]: nom inconnu `caree`\n --> main.ry:2:7\n  |\n2 | print(caree(4))\n  |       ^^^^^ non défini\n  |\n  = aide: vouliez-vous dire `carre` ?\n  = en savoir plus : `rythme explain E003`\n",
    errors_points: [
        "Le programme est vérifié avant d'être exécuté : tous les problèmes sont signalés d'un coup.",
        "rythme explain E003 détaille n'importe quel code d'erreur, avec des exemples.",
        "Les habitudes venues d'autres langages reçoivent une explication de la façon de faire en Rythme.",
    ],
    habits_title: "Vous venez d'un autre langage ?",
    habits_lead: "Rythme reconnaît vos réflexes et vous montre l'équivalent fonctionnel.",
    habits_head: ["Ailleurs", "En Rythme", "Pourquoi"],
    habits: [
        habit("for x in liste", "liste |> each(print)", "Pas de boucles : on travaille sur toute la liste."),
        habit("x += 1", "y = x + 1", "Les valeurs ne changent pas : on en crée une nouvelle."),
        habit("return x", "fn f(x) = x * 2", "La dernière expression est le résultat."),
        habit("null", "Some(x) | None", "Une valeur qui peut manquer est une Option."),
        habit("try / catch", "Ok(x) | Err(e), ?", "Les erreurs sont des valeurs, transmises par ?."),
        habit("class Forme", "type Forme = Cercle(r) | Rect(l, h)", "Des types algébriques et match."),
    ],
    install_title: "Installer Rythme",
    install_lead: "Un seul programme, sans dépendance, pour Linux, macOS et Windows.",
    install_steps: [
        item(
            "Sans rien d'autre",
            "Téléchargez l'archive de votre système sur la page Releases, extrayez rythme et placez-le dans un dossier de votre PATH.",
        ),
        item("Avec Rust", "cargo install --git https://github.com/maxlestage/Rythme"),
        item("Premier programme", "rythme bonjour.ry"),
    ],
    commands_title: "Les commandes",
    commands: [
        ("rythme", "mode interactif : tapez du code, voyez le résultat"),
        ("rythme bonjour.ry", "lancer un programme"),
        ("rythme check bonjour.ry", "trouver les problèmes sans lancer le programme"),
        ("rythme test bonjour.ry", "lancer les tests écrits dans le fichier"),
        ("rythme help map", "l'aide d'un sujet ou d'une fonction"),
        ("rythme explain E003", "expliquer un code d'erreur"),
        ("rythme learn", "visite guidée du langage"),
        ("rythme -e 'print(1 + 2)'", "lancer un bout de code"),
    ],
    commands_lang: "Ajoutez --lang fr ou --lang en avant la commande pour choisir la langue des messages, ou définissez RYTHME_LANG.",
    copy: "Copier",
    copied: "Copié",
    cta_title: "Prêt à écrire votre premier programme ?",
    cta_text: "Le vrai interpréteur tourne dans votre navigateur, compilé en WebAssembly. Rien à installer.",
    footer_tagline: "Un langage simple, purement fonctionnel, avec de l'aide partout.",
    footer_made: "Site écrit en Rust avec Paquets.",
    footer_license: "Licence MIT",
    footer_source: "Code source",
    pg_title: "Playground",
    pg_lead: "Le vrai interpréteur Rythme, compilé en WebAssembly. Modifiez le code, appuyez sur Lancer.",
    pg_example: "Exemple",
    pg_welcome: "Bienvenue",
    pg_run: "Lancer",
    pg_check: "Vérifier",
    pg_test: "Tester",
    pg_stop: "Arrêter",
    pg_code: "Code Rythme",
    pg_output: "Résultat",
    pg_help: "Aide",
    pg_help_intro: "La même aide que la commande rythme, donnée par l'interpréteur lui-même.",
    pg_search: "une fonction ou un code d'erreur",
    pg_search_go: "Afficher",
    pg_limits: "Dans le navigateur, les fichiers ne peuvent pas être lus, input() renvoie un texte vide, et les appels imbriqués s'arrêtent à 600 (les appels terminaux n'ont pas de limite).",
    pg_noscript: "Le playground a besoin de JavaScript et de WebAssembly. Vous pouvez lire les exemples sans.",
    pg_welcome_code: "# Bienvenue dans Rythme ! Appuyez sur Lancer (Ctrl+Entrée).\n# Les valeurs ne changent jamais, tout est fonction,\n# et chaque erreur s'explique d'elle-même.\n\nfn carre(x) = x * x\n\n[1, 2, 3, 4, 5]\n  |> filter(is_odd)\n  |> map(carre)\n  |> print\n\ntype Forme = Cercle(rayon) | Rect(largeur, hauteur)\n\nfn aire(Cercle(r)) = round(pi * r * r, 2)\nfn aire(Rect(l, h)) = l * h\n\nprint(map([Cercle(1.0), Rect(2, 3)], aire))\n\nfn diviser(a, b) =\n  if b == 0 then Err(\"division par zéro impossible\") else Ok(a / b)\n\nprint(diviser(10, 4), diviser(1, 0))\n\n# Faites une erreur exprès : remplacez carre par caree plus haut, puis Lancer.\n",
    ex_title: "Exemples",
    ex_lead: "De vrais programmes Rythme et ce qu'ils affichent. Chacun s'ouvre dans le playground.",
    ex_output: "Sortie",
    ex_tests: "rythme test",
    ex_open: "Ouvrir dans le playground",
    ex_jump: "Aller à un exemple",
    learn_title: "Le langage en deux minutes",
    learn_lead: "Huit idées, et vous savez lire tout programme Rythme. Chaque morceau de code s'ouvre dans le playground.",
    learn_try: "Essayer",
    tour: [
        step(
            "Noms et valeurs",
            "Un nom reçoit une valeur, une fois pour toutes. Les accolades insèrent une valeur dans un texte.",
            "nom = \"Ada\"\nnaissance = 1815\nprint(\"{nom} est née en {naissance}\")\n",
        ),
        step(
            "Fonctions",
            "Une ligne avec =, ou un bloc fermé par end. La dernière ligne est le résultat : pas de return.",
            "fn double(x) = x * 2\n\nfn saluer(nom)\n  message = \"Bonjour, {nom} !\"\n  message\nend\n\nprint(double(21))\nprint(saluer(\"Ada\"))\n",
        ),
        step(
            "Une définition par cas",
            "Les définitions sont essayées dans l'ordre : le cas de base d'abord.",
            "fn fib(0) = 0\nfn fib(1) = 1\nfn fib(n) = fib(n - 1) + fib(n - 2)\n\nprint(map(0..10, fib))\n",
        ),
        step(
            "Les conditions sont des expressions",
            "Chaque if a un else, et donne une valeur.",
            "fn signe(n) = if n < 0 then \"négatif\" else if n == 0 then \"zéro\" else \"positif\"\n\nprint(signe(-4), signe(0), signe(7))\n",
        ),
        step(
            "Listes, intervalles, enregistrements",
            "|> passe une valeur à la fonction suivante. Mettre à jour un enregistrement en crée un nouveau.",
            "nombres = 1..10\nprint(nombres |> filter(is_even) |> map(fn(x) = x * x))\n\nada = {nom: \"Ada\", naissance: 1815}\nautre = {..ada, naissance: 1816}\nprint(ada.naissance, autre.naissance)\n",
        ),
        step(
            "Vos propres types, et match",
            "Un type liste ses formes possibles ; match choisit selon la forme. Un cas oublié est signalé.",
            "type Forme = Cercle(rayon) | Rect(largeur, hauteur)\n\nfn aire(forme) = match forme\n  Cercle(r) -> round(pi * r * r, 2)\n  Rect(l, h) -> l * h\nend\n\nprint(aire(Cercle(1.0)), aire(Rect(2, 3)))\n",
        ),
        step(
            "Les erreurs sont des valeurs",
            "Une opération qui peut échouer renvoie Ok ou Err. ? s'arrête au premier Err et le renvoie.",
            "fn diviser(a, b) = if b == 0 then Err(\"division par zéro\") else Ok(a / b)\n\nfn moyenne(a, b)\n  x = parse_float(a)?\n  y = parse_float(b)?\n  Ok((x + y) / 2)\nend\n\nprint(diviser(10, 4), diviser(1, 0))\nprint(moyenne(\"4\", \"5\"), moyenne(\"4\", \"cinq\"))\n",
        ),
        step(
            "Les tests vivent à côté du code",
            "rythme test fichier.ry lance les tests et montre, pour chaque échec, la valeur attendue et l'obtenue.",
            "fn double(x) = x * 2\n\ntest \"double marche\" = assert_eq(double(21), 42)\ntest \"double de zéro\" = assert_eq(double(0), 0)\n",
        ),
    ],
    learn_names: "Les mots-clés et les fonctions intégrées sont en anglais, comme dans tous les langages répandus, mais vos propres noms peuvent être en français, accents compris : prénom, âge, résultat.",
    learn_next_title: "Pour aller plus loin",
    learn_next: [
        item("rythme learn", "Une visite guidée, à suivre seul, dans votre terminal."),
        item("rythme help syntax", "Toute la syntaxe sur une page, et help(f) pour chaque fonction."),
        item("Les exemples", "Des programmes complets : FizzBuzz, quicksort, arbres, textes…"),
    ],
    teach_lead: "Un plan de cours en 12 séances, et pourquoi Rythme convient à un premier cours de programmation comme à un cours de programmation fonctionnelle.",
    teach_toc: "Sur cette page",
    nf_title: "Cette page n'existe pas",
    nf_text: "Le lien est peut-être ancien, ou l'adresse contient une faute de frappe. Rythme aurait proposé le bon nom.",
    nf_home: "Retour à l'accueil",
};

pub const EN: Texts = Texts {
    meta_title: "Rythme — a simple, purely functional language, with help everywhere",
    meta_description: "Rythme is a simple, purely functional programming language: 15 keywords, no null, no exceptions, and errors that explain how to fix them. Try it in your browser.",
    skip: "Skip to content",
    brand_aria: "Rythme, home",
    nav_aria: "Main navigation",
    nav_learn: "Learn",
    nav_examples: "Examples",
    nav_playground: "Playground",
    nav_teach: "Teach",
    menu: "Menu",
    lang_aria: "Language",
    hero_eyebrow: "Programming language · v0.1",
    hero_title: "A simple language where every error helps you.",
    hero_lead: "Rythme is purely functional: values never change, everything is a function, and there are no loops, no null and no exceptions. And the interpreter explains every error, in English or in French.",
    cta_try: "Try it in your browser",
    cta_install: "Install",
    hero_code: "# Values never change. Everything is a function.\nfn square(x) = x * x\n\n[1, 2, 3, 4]\n  |> filter(is_even)      # keep the even numbers\n  |> map(square)          # square each one\n  |> sum                  # add them up\n  |> print\n",
    hero_output: "Output",
    why_title: "Why Rythme?",
    why_lead: "Fewer concepts to learn, fewer bugs to chase, and help at every step.",
    why: [
        item("Simpler than Python", "15 keywords (Python has 35) and one way to do each thing. No classes, no loops, no mutation."),
        item(
            "Purely functional",
            "Immutable values, functions as values, every if and match is an expression. Recursion in tail position never runs out of stack.",
        ),
        item(
            "Safe like Rust",
            "No null: an Option. No exceptions: a Result, and ? to pass the error on. Algebraic types, pattern matching, forgotten cases reported.",
        ),
        item(
            "Help everywhere",
            "Every error says where, why and how to fix it. rythme explain, rythme help, rythme learn: all of it in English and French.",
        ),
    ],
    errors_title: "Errors that teach",
    errors_lead: "A typo? Rythme shows the exact place, explains the problem and suggests the right name, the way the Rust compiler does.",
    errors_code: "fn square(x) = x * x\nprint(sqaure(4))\n",
    errors_message: "error[E003]: unknown name `sqaure`\n --> main.ry:2:7\n  |\n2 | print(sqaure(4))\n  |       ^^^^^^ not defined\n  |\n  = help: did you mean `square`?\n  = more about this error: `rythme explain E003`\n",
    errors_points: [
        "The program is checked before it runs: every problem is reported at once.",
        "rythme explain E003 explains any error code in detail, with examples.",
        "Habits carried over from other languages get an explanation of the Rythme way.",
    ],
    habits_title: "Coming from another language?",
    habits_lead: "Rythme recognizes your reflexes and shows you the functional way.",
    habits_head: ["Elsewhere", "In Rythme", "Why"],
    habits: [
        habit("for x in list", "list |> each(print)", "No loops: work on the whole list."),
        habit("x += 1", "y = x + 1", "Values never change: make a new one."),
        habit("return x", "fn f(x) = x * 2", "The last expression is the result."),
        habit("null", "Some(x) | None", "A value that may be missing is an Option."),
        habit("try / catch", "Ok(x) | Err(e), ?", "Errors are values, passed on by ?."),
        habit("class Shape", "type Shape = Circle(r) | Rect(w, h)", "Algebraic types and match."),
    ],
    install_title: "Install Rythme",
    install_lead: "One program, no dependency, for Linux, macOS and Windows.",
    install_steps: [
        item(
            "Nothing else needed",
            "Download the archive for your system on the Releases page, extract rythme and put it in a folder of your PATH.",
        ),
        item("With Rust", "cargo install --git https://github.com/maxlestage/Rythme"),
        item("First program", "rythme hello.ry"),
    ],
    commands_title: "Commands",
    commands: [
        ("rythme", "interactive mode: type code, see the result"),
        ("rythme hello.ry", "run a program"),
        ("rythme check hello.ry", "find problems without running the program"),
        ("rythme test hello.ry", "run the tests written in the file"),
        ("rythme help map", "help on a topic or a function"),
        ("rythme explain E003", "explain an error code"),
        ("rythme learn", "guided tour of the language"),
        ("rythme -e 'print(1 + 2)'", "run a piece of code"),
    ],
    commands_lang: "Add --lang fr or --lang en before the command to choose the language of the messages, or set RYTHME_LANG.",
    copy: "Copy",
    copied: "Copied",
    cta_title: "Ready to write your first program?",
    cta_text: "The real interpreter runs in your browser, compiled to WebAssembly. Nothing to install.",
    footer_tagline: "A simple, purely functional language, with help everywhere.",
    footer_made: "Website written in Rust with Paquets.",
    footer_license: "MIT license",
    footer_source: "Source code",
    pg_title: "Playground",
    pg_lead: "The real Rythme interpreter, compiled to WebAssembly. Edit the code, press Run.",
    pg_example: "Example",
    pg_welcome: "Welcome",
    pg_run: "Run",
    pg_check: "Check",
    pg_test: "Test",
    pg_stop: "Stop",
    pg_code: "Rythme code",
    pg_output: "Output",
    pg_help: "Help",
    pg_help_intro: "The same help as the rythme command, given by the interpreter itself.",
    pg_search: "a function or an error code",
    pg_search_go: "Show",
    pg_limits: "In the browser, files cannot be read, input() returns an empty text, and nested calls stop at 600 (tail calls have no limit).",
    pg_noscript: "The playground needs JavaScript and WebAssembly. You can read the examples without them.",
    pg_welcome_code: "# Welcome to Rythme! Press Run (Ctrl+Enter).\n# Values never change, everything is a function,\n# and every error explains itself.\n\nfn square(x) = x * x\n\n[1, 2, 3, 4, 5]\n  |> filter(is_odd)\n  |> map(square)\n  |> print\n\ntype Shape = Circle(radius) | Rect(width, height)\n\nfn area(Circle(r)) = round(pi * r * r, 2)\nfn area(Rect(w, h)) = w * h\n\nprint(map([Circle(1.0), Rect(2, 3)], area))\n\nfn safe_divide(a, b) =\n  if b == 0 then Err(\"cannot divide by zero\") else Ok(a / b)\n\nprint(safe_divide(10, 4), safe_divide(1, 0))\n\n# Make a mistake on purpose: change square to sqaure above, then Run.\n",
    ex_title: "Examples",
    ex_lead: "Real Rythme programs and what they print. Each one opens in the playground.",
    ex_output: "Output",
    ex_tests: "rythme test",
    ex_open: "Open in the playground",
    ex_jump: "Jump to an example",
    learn_title: "The language in two minutes",
    learn_lead: "Eight ideas, and you can read any Rythme program. Every piece of code opens in the playground.",
    learn_try: "Try it",
    tour: [
        step(
            "Names and values",
            "A name gets a value, once and for all. Braces insert a value in a text.",
            "name = \"Ada\"\nborn = 1815\nprint(\"{name} was born in {born}\")\n",
        ),
        step(
            "Functions",
            "One line with =, or a block closed by end. The last line is the result: no return.",
            "fn double(x) = x * 2\n\nfn greet(name)\n  message = \"Hello, {name}!\"\n  message\nend\n\nprint(double(21))\nprint(greet(\"Ada\"))\n",
        ),
        step(
            "One definition per case",
            "Definitions are tried in order: the base case first.",
            "fn fib(0) = 0\nfn fib(1) = 1\nfn fib(n) = fib(n - 1) + fib(n - 2)\n\nprint(map(0..10, fib))\n",
        ),
        step(
            "Conditions are expressions",
            "Every if has an else, and gives a value.",
            "fn sign(n) = if n < 0 then \"negative\" else if n == 0 then \"zero\" else \"positive\"\n\nprint(sign(-4), sign(0), sign(7))\n",
        ),
        step(
            "Lists, ranges, records",
            "|> passes a value to the next function. Updating a record makes a new one.",
            "numbers = 1..10\nprint(numbers |> filter(is_even) |> map(fn(x) = x * x))\n\nada = {name: \"Ada\", born: 1815}\nolder = {..ada, born: 1816}\nprint(ada.born, older.born)\n",
        ),
        step(
            "Your own types, and match",
            "A type lists its possible shapes; match picks by shape. A forgotten case is reported.",
            "type Shape = Circle(radius) | Rect(width, height)\n\nfn area(shape) = match shape\n  Circle(r) -> round(pi * r * r, 2)\n  Rect(w, h) -> w * h\nend\n\nprint(area(Circle(1.0)), area(Rect(2, 3)))\n",
        ),
        step(
            "Errors are values",
            "An operation that may fail returns Ok or Err. ? stops at the first Err and returns it.",
            "fn divide(a, b) = if b == 0 then Err(\"division by zero\") else Ok(a / b)\n\nfn average(a, b)\n  x = parse_float(a)?\n  y = parse_float(b)?\n  Ok((x + y) / 2)\nend\n\nprint(divide(10, 4), divide(1, 0))\nprint(average(\"4\", \"5\"), average(\"4\", \"five\"))\n",
        ),
        step(
            "Tests live next to the code",
            "rythme test file.ry runs the tests and shows, for each failure, the expected and the obtained values.",
            "fn double(x) = x * 2\n\ntest \"double works\" = assert_eq(double(21), 42)\ntest \"double of zero\" = assert_eq(double(0), 0)\n",
        ),
    ],
    learn_names: "Keywords and built-in functions are in English, like in every widespread language, but your own names can be in any language, accents included: prénom, âge, résultat.",
    learn_next_title: "Going further",
    learn_next: [
        item("rythme learn", "A guided tour to follow alone, in your terminal."),
        item("rythme help syntax", "The whole syntax on one page, and help(f) for every function."),
        item("The examples", "Complete programs: FizzBuzz, quicksort, trees, text processing…"),
    ],
    teach_lead: "A course outline in 12 sessions, and why Rythme fits a first programming course and a functional programming course alike.",
    teach_toc: "On this page",
    nf_title: "This page does not exist",
    nf_text: "The link may be old, or the address has a typo. Rythme would have suggested the right name.",
    nf_home: "Back to the home page",
};
