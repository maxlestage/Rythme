//! The polish of the `rythme` command: `--help` for each command, shell
//! completions, the manual page, `rythme version --verbose`, `rythme
//! upgrade`, programs read from the standard input (`rythme run -`) and
//! folders given to `rythme check`. `main.rs` calls these functions.

use std::path::{Path, PathBuf};

use crate::driver::Driver;
use crate::interp::Mode;
use crate::{help, lang};

/// A command of `rythme`, for its `--help`, the completions and the manual page.
struct Command {
    name: &'static str,
    /// The other names it answers to (`lancer` for `run`).
    aliases: &'static [&'static str],
    synopsis: &'static str,
    /// The options to complete, after the command.
    options: &'static [&'static str],
    /// Does `--help` after this command show its usage? (Some commands show
    /// their own help, and the words after the file of `run` belong to the program.)
    usage: Usage,
    /// The help topic that tells more.
    topic: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq)]
enum Usage {
    /// `--help` anywhere after the command.
    Anywhere,
    /// `--help` before the file only: what follows the file is the program's.
    BeforeFile,
    /// The command shows its own help.
    Own,
}

const COMMANDS: &[Command] = &[
    Command {
        name: "run",
        aliases: &["lancer"],
        synopsis: "rythme run [--seed N] [--sandbox] [--format json] FILE.ry|- [ARGS...]",
        options: &["--seed", "--sandbox", "--format"],
        usage: Usage::BeforeFile,
        topic: Some("system"),
    },
    Command {
        name: "check",
        aliases: &["verifier", "vérifier"],
        synopsis: "rythme check [--format json] [FILE.ry|FOLDER...]",
        options: &["--format"],
        usage: Usage::Anywhere,
        topic: Some("errors"),
    },
    Command {
        name: "test",
        aliases: &["tester"],
        synopsis: "rythme test [--filter NAME] [--seed N] [--watch] [--sandbox] [--format json] [FILE.ry|FOLDER...]",
        options: &["--filter", "--seed", "--watch", "--sandbox", "--format"],
        usage: Usage::Anywhere,
        topic: Some("tests"),
    },
    Command {
        name: "fmt",
        aliases: &["format", "formater"],
        synopsis: "rythme fmt [--check] FILE.ry|FOLDER|-...",
        options: &["--check"],
        usage: Usage::Anywhere,
        topic: Some("fmt"),
    },
    Command {
        name: "doc",
        aliases: &["documenter"],
        synopsis: "rythme doc [--html] [--out FILE] FILE.ry|FOLDER",
        options: &["--html", "--markdown", "--out"],
        usage: Usage::Own,
        topic: Some("doc"),
    },
    Command {
        name: "fix",
        aliases: &["corriger"],
        synopsis: "rythme fix [--dry-run] [--all] [--format json] FILE.ry...",
        options: &["--dry-run", "--all", "--format"],
        usage: Usage::Anywhere,
        topic: None,
    },
    Command {
        name: "steps",
        aliases: &["etapes", "étapes"],
        synopsis: "rythme steps [--max N] [--width N] FILE.ry [\"EXPRESSION\"]",
        options: &["--max", "--width"],
        usage: Usage::BeforeFile,
        topic: Some("steps"),
    },
    Command {
        name: "debug",
        aliases: &["deboguer", "déboguer"],
        synopsis: "rythme debug FILE.ry [ARGS...]",
        options: &[],
        usage: Usage::BeforeFile,
        topic: Some("debug"),
    },
    Command { name: "dap", aliases: &[], synopsis: "rythme dap", options: &[], usage: Usage::Anywhere, topic: Some("debug") },
    Command {
        name: "exercise",
        aliases: &["exercises", "exercice", "exercices"],
        synopsis: "rythme exercise [list | start NAME | check FILE.ry]",
        options: &["list", "start", "check"],
        usage: Usage::Anywhere,
        topic: Some("exercises"),
    },
    Command { name: "new", aliases: &["nouveau"], synopsis: "rythme new NAME", options: &[], usage: Usage::Own, topic: Some("projects") },
    Command {
        name: "install",
        aliases: &["installer"],
        synopsis: "rythme install",
        options: &[],
        usage: Usage::Own,
        topic: Some("projects"),
    },
    Command {
        name: "learn",
        aliases: &["apprendre", "tutorial", "tutoriel"],
        synopsis: "rythme learn",
        options: &[],
        usage: Usage::Anywhere,
        topic: None,
    },
    Command {
        name: "help",
        aliases: &["aide"],
        synopsis: "rythme help [TOPIC|FUNCTION|CODE] [--format json]",
        options: &["--format"],
        usage: Usage::Anywhere,
        topic: None,
    },
    Command {
        name: "explain",
        aliases: &["expliquer"],
        synopsis: "rythme explain [--format json] CODE",
        options: &["--format"],
        usage: Usage::Anywhere,
        topic: None,
    },
    Command {
        name: "-e",
        aliases: &["eval"],
        synopsis: "rythme -e \"CODE\" [ARGS...]",
        options: &[],
        usage: Usage::BeforeFile,
        topic: None,
    },
    Command { name: "repl", aliases: &[], synopsis: "rythme repl", options: &[], usage: Usage::Anywhere, topic: Some("repl") },
    Command { name: "lsp", aliases: &[], synopsis: "rythme lsp", options: &[], usage: Usage::Own, topic: Some("editors") },
    Command { name: "mcp", aliases: &[], synopsis: "rythme mcp", options: &[], usage: Usage::Own, topic: None },
    Command {
        name: "version",
        aliases: &[],
        synopsis: "rythme version [--verbose]",
        options: &["--verbose"],
        usage: Usage::Anywhere,
        topic: None,
    },
    Command {
        name: "completions",
        aliases: &[],
        synopsis: "rythme completions bash|zsh|fish|powershell",
        options: &["bash", "zsh", "fish", "powershell"],
        usage: Usage::Anywhere,
        topic: None,
    },
    Command { name: "man", aliases: &[], synopsis: "rythme man", options: &[], usage: Usage::Anywhere, topic: None },
    Command { name: "upgrade", aliases: &[], synopsis: "rythme upgrade", options: &[], usage: Usage::Anywhere, topic: None },
];

/// What a command does, in one line.
fn about(name: &str) -> String {
    match name {
        "run" => t!(
            "run a program; the words after the file are its arguments",
            "lancer un programme ; les mots après le fichier sont ses arguments"
        ),
        "check" => t!("find problems without running the program", "trouver les problèmes sans lancer le programme"),
        "test" => t!("run the tests of a file or of a folder", "lancer les tests d'un fichier ou d'un dossier"),
        "fmt" => t!("give the code its official layout", "donner au code sa mise en forme officielle"),
        "doc" => t!(
            "write the documentation of a file from its ## comments",
            "écrire la documentation d'un fichier à partir de ses commentaires ##"
        ),
        "fix" => t!(
            "apply the safe fixes of the problems that `rythme check` finds",
            "appliquer les corrections sûres des problèmes que trouve `rythme check`"
        ),
        "steps" => t!("show a calculation step by step", "montrer un calcul étape par étape"),
        "debug" => t!(
            "pause a program where you want and look at its values",
            "mettre un programme en pause où vous voulez et regarder ses valeurs"
        ),
        "dap" => t!("the debugger for editors (Debug Adapter Protocol)", "le débogueur pour les éditeurs (Debug Adapter Protocol)"),
        "exercise" => t!("exercises checked automatically", "des exercices vérifiés automatiquement"),
        "new" => t!("create a project (rythme.toml, src/, tests/)", "créer un projet (rythme.toml, src/, tests/)"),
        "install" => t!("download the dependencies of the project", "télécharger les dépendances du projet"),
        "learn" => t!("guided tour of the language, step by step", "visite guidée du langage, pas à pas"),
        "help" => t!("help on a topic, a function or an error code", "de l'aide sur un sujet, une fonction ou un code d'erreur"),
        "explain" => t!("explain an error or warning code, like E003", "expliquer un code d'erreur ou d'avertissement, comme E003"),
        "-e" => t!("run a piece of code given on the command line", "lancer un morceau de code donné sur la ligne de commande"),
        "repl" => t!(
            "interactive mode: try code line by line (also `rythme` alone)",
            "mode interactif : essayer du code ligne par ligne (aussi `rythme` seul)"
        ),
        "lsp" => t!("language server, for editors (VS Code, Neovim...)", "serveur de langage, pour les éditeurs (VS Code, Neovim...)"),
        "mcp" => t!("server for AI assistants (Model Context Protocol)", "serveur pour les assistants d'IA (Model Context Protocol)"),
        "version" => t!("the version of Rythme", "la version de Rythme"),
        "completions" => t!("completion of the commands in a shell", "complétion des commandes dans un shell"),
        "man" => t!("the manual page of the command, to install with `man`", "la page de manuel de la commande, à installer pour `man`"),
        "upgrade" => t!("how to install the latest version of Rythme", "comment installer la dernière version de Rythme"),
        _ => String::new(),
    }
}

/// The options of a command and what to know about it, a few lines.
fn details(name: &str) -> String {
    match name {
        "run" => t!(
            "  --seed N        the same draws of random and shuffle on every run\n  --sandbox       no files, folders, environment variables nor network\n  --format json   the output and the errors as JSON, for tools\n  FILE may be `-`: the program is read from the standard input.\n  At the root of a project, `rythme run` runs its program.",
            "  --seed N        les mêmes tirages de random et shuffle à chaque fois\n  --sandbox       ni fichiers, ni dossiers, ni variables d'environnement, ni réseau\n  --format json   la sortie et les erreurs en JSON, pour les outils\n  FICHIER peut être `-` : le programme est lu sur l'entrée standard.\n  À la racine d'un projet, `rythme run` lance son programme."
        ),
        "check" => t!(
            "  --format json   the problems as JSON, for tools and AI assistants\n  A folder checks every .ry file inside it; at the root of a project,\n  `rythme check` checks the files of the project.\n  Exit code: 0 without errors (warnings allowed), 1 with errors.",
            "  --format json   les problèmes en JSON, pour les outils et les assistants d'IA\n  Un dossier vérifie chaque fichier .ry qu'il contient ; à la racine d'un\n  projet, `rythme check` vérifie les fichiers du projet.\n  Code de sortie : 0 sans erreur (avertissements permis), 1 avec des erreurs."
        ),
        "test" => t!(
            "  --filter NAME   only the tests whose name contains NAME\n  --seed N        the same draws of random on every run\n  --watch         run the tests again whenever a file changes\n  --sandbox       no files, folders, environment variables nor network\n  --format json   the results as JSON\n  Exit code: 0 when every test passes, 1 otherwise.",
            "  --filter NOM    seulement les tests dont le nom contient NOM\n  --seed N        les mêmes tirages de random à chaque fois\n  --watch         relancer les tests dès qu'un fichier change\n  --sandbox       ni fichiers, ni dossiers, ni variables d'environnement, ni réseau\n  --format json   les résultats en JSON\n  Code de sortie : 0 quand tous les tests passent, 1 sinon."
        ),
        "fmt" => t!(
            "  --check         only check, for continuous integration: exit code 1 when a\n                  file must be formatted\n  A folder formats all its .ry files; `-` reads the code from the standard\n  input and writes it formatted.",
            "  --check         vérifier seulement, pour l'intégration continue : code de\n                  sortie 1 quand un fichier doit être mis en forme\n  Un dossier met en forme tous ses fichiers .ry ; `-` lit le code sur l'entrée\n  standard et l'écrit mis en forme."
        ),
        "fix" => t!(
            "  --dry-run       only show the changes, as a diff\n  --all           also apply the fixes that are only likely\n  --format json   the fixes as JSON\n  Exit code: 0 when no error is left, 1 otherwise (with --dry-run: 1 when\n  something would change).",
            "  --dry-run       montrer seulement les changements, comme un diff\n  --all           appliquer aussi les corrections seulement probables\n  --format json   les corrections en JSON\n  Code de sortie : 0 quand il ne reste aucune erreur, 1 sinon (avec --dry-run :\n  1 quand quelque chose changerait)."
        ),
        "steps" => t!(
            "  --max N         show at most N steps\n  --width N       cut the lines at N characters (20 at least)\n  Without an expression, the steps of the last line of the file.",
            "  --max N         montrer au plus N étapes\n  --width N       couper les lignes à N caractères (20 au moins)\n  Sans expression, les étapes de la dernière ligne du fichier."
        ),
        "debug" => t!(
            "  In the debugger, `help` lists its commands: break, run, next, step,\n  finish, where, print, locals, quit.",
            "  Dans le débogueur, `help` liste ses commandes : break, run, next, step,\n  finish, where, print, locals, quit."
        ),
        "dap" => t!(
            "  Your editor starts it: the VS Code extension does, for the debugger type `rythme`.",
            "  Votre éditeur le lance : l'extension VS Code le fait, pour le type de débogueur `rythme`."
        ),
        "exercise" => t!(
            "  list            the exercises, from the easiest\n  start NAME      write the file of an exercise, to complete\n  check FILE.ry   run its tests and give hints",
            "  list            les exercices, du plus facile au plus difficile\n  start NOM       écrire le fichier d'un exercice, à compléter\n  check FICHIER   lancer ses tests et donner des indices"
        ),
        "help" => t!(
            "  `rythme help` lists the topics, `rythme help builtins` every function,\n  `rythme help llms` the whole language in one text (for AI assistants).",
            "  `rythme help` liste les sujets, `rythme help builtins` chaque fonction,\n  `rythme help llms` tout le langage en un texte (pour les assistants d'IA)."
        ),
        "explain" => t!(
            "  Without a code, the list of the codes; --format json gives them all.",
            "  Sans code, la liste des codes ; --format json les donne tous."
        ),
        "-e" => t!(
            "  The value of the last expression is shown, unless it is ().",
            "  La valeur de la dernière expression est affichée, sauf si c'est ()."
        ),
        "repl" => t!("  `:help` lists the commands of the interactive mode.", "  `:help` liste les commandes du mode interactif."),
        "version" => t!(
            "  --verbose       also the system, the languages, the functions and the formats",
            "  --verbose       aussi le système, les langues, les fonctions et les formats"
        ),
        "completions" => format!(
            "  bash         rythme completions bash > ~/.local/share/bash-completion/completions/rythme\n  zsh          rythme completions zsh > \"${{fpath[1]}}/_rythme\"\n  fish         rythme completions fish > ~/.config/fish/completions/rythme.fish\n  powershell   rythme completions powershell >> $PROFILE\n  {}",
            t!("Then open a new shell.", "Ouvrez ensuite un nouveau shell.")
        ),
        "man" => t!(
            "  rythme man > rythme.1 && man ./rythme.1\n  To install it: put rythme.1 in a folder of the manual pages, like\n  ~/.local/share/man/man1.",
            "  rythme man > rythme.1 && man ./rythme.1\n  Pour l'installer : placez rythme.1 dans un dossier des pages de manuel,\n  comme ~/.local/share/man/man1."
        ),
        _ => String::new(),
    }
}

fn find(name: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|c| c.name == name || c.aliases.contains(&name))
}

/// The usage of a command, as `rythme CMD --help` shows it.
pub fn usage(name: &str) -> Option<String> {
    let c = find(name)?;
    let mut out = format!("{}\n  {}\n", c.synopsis, about(c.name));
    let more = details(c.name);
    if !more.is_empty() {
        out.push_str(&format!("\n{more}\n"));
    }
    let codes = lang::codes();
    out.push_str(&t!(
        "\nOptions of every command: --lang CODE ({codes}), --no-color\n",
        "\nOptions de toutes les commandes : --lang CODE ({codes}), --no-color\n"
    ));
    if let Some(topic) = c.topic {
        out.push_str(&t!("See also `rythme help {topic}`.\n", "Voir aussi `rythme help {topic}`.\n"));
    }
    Some(out)
}

/// `rythme CMD --help` (or `-h`): the usage of the command, if it is one of
/// those whose help is shown here.
pub fn command_help(cmd: &str, rest: &[String]) -> Option<i32> {
    let c = find(cmd)?;
    let asks = |w: &String| w == "--help" || w == "-h";
    let wanted = match c.usage {
        Usage::Own => false,
        // After `--`, the words are the program's (`rythme test file.ry -- --help`).
        Usage::Anywhere => rest.iter().take_while(|w| w.as_str() != "--").any(asks),
        Usage::BeforeFile => rest.iter().take_while(|w| w.starts_with('-') && w.as_str() != "-").any(asks),
    };
    if !wanted {
        return None;
    }
    crate::io::out(&usage(c.name)?);
    Some(0)
}

/// What `rythme help` adds after the topics: the commands of tooling.
pub fn overview_more() -> String {
    t!(
        "\nTools\n  rythme CMD --help         the options of a command, like rythme test --help\n  rythme run -              run the program read from the standard input\n  rythme check FOLDER       check every .ry file of a folder\n  rythme completions SHELL  completion of the commands: bash, zsh, fish, powershell\n  rythme man                the manual page of the command (rythme man > rythme.1)\n  rythme version --verbose  the version, with the system and what Rythme contains\n  rythme upgrade            how to install the latest version\n",
        "\nOutils\n  rythme CMD --help         les options d'une commande, comme rythme test --help\n  rythme run -              lancer le programme lu sur l'entrée standard\n  rythme check DOSSIER      vérifier chaque fichier .ry d'un dossier\n  rythme completions SHELL  complétion des commandes : bash, zsh, fish, powershell\n  rythme man                la page de manuel de la commande (rythme man > rythme.1)\n  rythme version --verbose  la version, avec le système et ce que contient Rythme\n  rythme upgrade            comment installer la dernière version\n"
    )
}

// ----- rythme version --verbose -----------------------------------------------

/// `rythme version [--verbose]`.
pub fn version_command(args: &[String]) -> i32 {
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    if let Some(bad) = args.iter().find(|a| !matches!(a.as_str(), "--verbose" | "-v")) {
        crate::io::err_line(&t!(
            "unknown option `{bad}` for `rythme version`: the option is --verbose",
            "option inconnue `{bad}` pour `rythme version` : l'option est --verbose"
        ));
        return 2;
    }
    crate::io::out(&version_text(verbose));
    0
}

pub fn version_text(verbose: bool) -> String {
    let mut out = format!("rythme {}\n", crate::VERSION);
    if !verbose {
        return out;
    }
    let (os, arch) = (std::env::consts::OS, std::env::consts::ARCH);
    let (codes, now) = (lang::codes(), lang::code());
    let (functions, ruby) = (crate::builtins::BUILTINS.len(), crate::builtins::ALIASES.len());
    let count = |prefix: char| help::CODES.iter().filter(|c| c.starts_with(prefix)).count();
    let range = |prefix: char| {
        let all: Vec<&&str> = help::CODES.iter().filter(|c| c.starts_with(prefix)).collect();
        format!("{}-{}", all.first().map_or("", |c| **c), all.last().map_or("", |c| **c))
    };
    let (errors, warnings, e_range, w_range) = (count('E'), count('W'), range('E'), range('W'));
    let json = crate::machine::VERSION;
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    out.push_str(&t!(
        "system: {os} {arch}\nlanguages of the messages: {codes} (now: {now})\nbuilt-in functions: {functions} (and {ruby} Ruby names)\ncodes: {errors} errors ({e_range}), {warnings} warnings ({w_range})\nJSON output (--format json): version {json}\nprogram: {exe}\n",
        "système : {os} {arch}\nlangues des messages : {codes} (maintenant : {now})\nfonctions intégrées : {functions} (et {ruby} noms Ruby)\ncodes : {errors} erreurs ({e_range}), {warnings} avertissements ({w_range})\nsortie JSON (--format json) : version {json}\nprogramme : {exe}\n"
    ));
    out
}

// ----- rythme upgrade ---------------------------------------------------------

/// How Rythme was installed, guessed from where the program is.
fn installed_by(exe: &str) -> &'static str {
    let lower = exe.replace('\\', "/").to_lowercase();
    if lower.contains("/cellar/") || lower.contains("/homebrew/") || lower.contains("/linuxbrew/") {
        "brew upgrade rythme"
    } else if lower.contains("/winget/") || lower.contains("/microsoft/winget") {
        "winget upgrade Rythme.Rythme"
    } else if lower.contains("/.cargo/bin/") {
        "cargo install rythme --force"
    } else if cfg!(windows) {
        "irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex"
    } else {
        "curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh"
    }
}

/// `rythme upgrade`: says how to install the latest version (nothing is
/// downloaded: the command to type depends on how Rythme was installed).
pub fn upgrade(args: &[String]) -> i32 {
    if let Some(bad) = args.first() {
        crate::io::err_line(&t!(
            "`rythme upgrade` takes no argument, not `{bad}`",
            "`rythme upgrade` ne prend pas d'argument, pas `{bad}`"
        ));
        return 2;
    }
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    let (v, how) = (crate::VERSION, installed_by(&exe));
    crate::io::out(&t!(
        "This is rythme {v} ({exe}).\nTo install the latest version, type:\n  {how}\nThe versions and what changed: https://github.com/maxlestage/Rythme/releases\nWithin 1.x, a program that runs keeps running, with the same output.\n",
        "Ceci est rythme {v} ({exe}).\nPour installer la dernière version, tapez :\n  {how}\nLes versions et ce qui a changé : https://github.com/maxlestage/Rythme/releases\nDans les versions 1.x, un programme qui fonctionne continue de fonctionner, avec la même sortie.\n"
    ));
    0
}

// ----- rythme completions ---------------------------------------------------------

/// The names to complete after `rythme`: the commands, in English.
fn command_names() -> Vec<&'static str> {
    COMMANDS.iter().map(|c| c.name).collect()
}

fn topics() -> Vec<&'static str> {
    help::TOPICS.iter().map(|(t, _)| *t).collect()
}

fn languages() -> Vec<&'static str> {
    lang::LANGUAGES.iter().map(|l| l.code).collect()
}

/// `rythme completions SHELL`.
pub fn completions(args: &[String]) -> i32 {
    let script = match args.first().map(String::as_str) {
        Some("bash") => bash(),
        Some("zsh") => zsh(),
        Some("fish") => fish(),
        Some("powershell" | "pwsh") => powershell(),
        other => {
            let what = match other {
                Some(s) => t!("unknown shell `{s}`", "shell inconnu `{s}`"),
                None => t!("which shell?", "quel shell ?"),
            };
            crate::io::err_line(&t!(
                "{what} Write one of bash, zsh, fish, powershell, like `rythme completions bash`\nsee `rythme completions --help`",
                "{what} Écrivez l'un de bash, zsh, fish, powershell, comme `rythme completions bash`\nvoir `rythme completions --help`"
            ));
            return 2;
        }
    };
    crate::io::out(&script);
    0
}

fn bash() -> String {
    let mut cases = String::new();
    for c in COMMANDS.iter().filter(|c| !c.options.is_empty() || c.name == "help" || c.name == "explain") {
        let words: Vec<&str> = match c.name {
            "help" => topics(),
            "explain" => help::CODES.to_vec(),
            _ => c.options.to_vec(),
        };
        let names: Vec<&str> = std::iter::once(c.name).chain(c.aliases.iter().copied()).collect();
        cases.push_str(&format!("        {}) words=\"{}\" ;;\n", names.join("|"), words.join(" ")));
    }
    format!(
        r#"# Completion of the rythme command for bash, made by `rythme completions bash`.
# Install: rythme completions bash > ~/.local/share/bash-completion/completions/rythme
_rythme() {{
    local cur prev words
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"
    case "$prev" in
        --format) COMPREPLY=($(compgen -W "json" -- "$cur")); return ;;
        --lang) COMPREPLY=($(compgen -W "{langs}" -- "$cur")); return ;;
        --out|-o) COMPREPLY=($(compgen -f -- "$cur")); return ;;
    esac
    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=($(compgen -W "{commands} --lang --no-color --help --version" -- "$cur") $(compgen -f -X '!*.ry' -- "$cur") $(compgen -d -- "$cur"))
        return
    fi
    words=""
    case "${{COMP_WORDS[1]}}" in
{cases}    esac
    COMPREPLY=($(compgen -W "$words --help" -- "$cur") $(compgen -f -X '!*.ry' -- "$cur") $(compgen -d -- "$cur"))
}}
complete -o filenames -F _rythme rythme
"#,
        langs = languages().join(" "),
        commands = command_names().join(" "),
    )
}

/// A description between single quotes for zsh's `_describe`: a quote
/// closes, is escaped and opens again; a colon would end the name.
fn zsh_quoted(s: &str) -> String {
    s.replace('\'', "'\\''").replace(':', "\\:")
}

/// A text between single quotes for fish.
fn fish_quoted(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

fn zsh() -> String {
    let commands: Vec<String> =
        COMMANDS.iter().map(|c| format!("    '{}:{}'", c.name.replace('-', "\\-"), zsh_quoted(&about(c.name)))).collect();
    let mut cases = String::new();
    for c in COMMANDS.iter().filter(|c| !c.options.is_empty() || c.name == "help" || c.name == "explain") {
        let words: Vec<&str> = match c.name {
            "help" => topics(),
            "explain" => help::CODES.to_vec(),
            _ => c.options.to_vec(),
        };
        let names: Vec<&str> = std::iter::once(c.name).chain(c.aliases.iter().copied()).collect();
        cases.push_str(&format!("    {}) compadd -- {} --help ;;\n", names.join("|"), words.join(" ")));
    }
    format!(
        r#"#compdef rythme
# Completion of the rythme command for zsh, made by `rythme completions zsh`.
# Install: rythme completions zsh > "${{fpath[1]}}/_rythme", then open a new shell.
_rythme() {{
  local -a commands
  commands=(
{commands}
  )
  case $words[CURRENT-1] in
    --format) compadd json; return ;;
    --lang) compadd {langs}; return ;;
  esac
  if (( CURRENT == 2 )); then
    _describe 'command' commands
    compadd -- --lang --no-color --help --version
    _files -g '*.ry'
    return
  fi
  case $words[2] in
{cases}  esac
  _files -g '*.ry'
}}
if [ "$funcstack[1]" = "_rythme" ]; then
  _rythme "$@"
else
  compdef _rythme rythme
fi
"#,
        commands = commands.join("\n"),
        langs = languages().join(" "),
    )
}

fn fish() -> String {
    let mut out = String::from(
        "# Completion of the rythme command for fish, made by `rythme completions fish`.\n# Install: rythme completions fish > ~/.config/fish/completions/rythme.fish\ncomplete -c rythme -f\n",
    );
    // `-e` is an option for fish, not a command to complete.
    let names = command_names().into_iter().filter(|n| !n.starts_with('-')).collect::<Vec<_>>().join(" ");
    out.push_str(&format!(
        "complete -c rythme -l lang -x -a '{}'\ncomplete -c rythme -l no-color\ncomplete -c rythme -s h -l help\ncomplete -c rythme -l version\n",
        languages().join(" ")
    ));
    for c in COMMANDS.iter().filter(|c| !c.name.starts_with('-')) {
        out.push_str(&format!(
            "complete -c rythme -n 'not __fish_seen_subcommand_from {names}' -a '{}' -d '{}'\n",
            c.name,
            fish_quoted(&about(c.name))
        ));
        let words: Vec<&str> = match c.name {
            "help" => topics(),
            "explain" => help::CODES.to_vec(),
            _ => c.options.iter().copied().filter(|o| !o.starts_with('-')).collect(),
        };
        if !words.is_empty() {
            out.push_str(&format!("complete -c rythme -n '__fish_seen_subcommand_from {}' -a '{}'\n", c.name, words.join(" ")));
        }
        for o in c.options.iter().filter(|o| o.starts_with("--")) {
            let value = if *o == "--format" { " -x -a 'json'" } else { "" };
            out.push_str(&format!("complete -c rythme -n '__fish_seen_subcommand_from {}' -l {}{value}\n", c.name, &o[2..]));
        }
    }
    out.push_str(
        "complete -c rythme -n '__fish_seen_subcommand_from run check test fmt doc fix steps debug' -F -a '(__fish_complete_suffix .ry)'\n",
    );
    out
}

fn powershell() -> String {
    let commands: Vec<String> =
        COMMANDS.iter().map(|c| format!("        @{{ Name = '{}'; About = '{}' }}", c.name, about(c.name).replace('\'', "''"))).collect();
    let mut options = String::new();
    for c in COMMANDS {
        let words: Vec<&str> = match c.name {
            "help" => topics(),
            "explain" => help::CODES.to_vec(),
            _ => c.options.to_vec(),
        };
        if !words.is_empty() {
            let list: Vec<String> = words.iter().map(|w| format!("'{w}'")).collect();
            options.push_str(&format!("        '{}' = @({})\n", c.name, list.join(", ")));
        }
    }
    format!(
        r#"# Completion of the rythme command for PowerShell, made by `rythme completions powershell`.
# Install: rythme completions powershell >> $PROFILE
Register-ArgumentCompleter -Native -CommandName rythme -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)
    $commands = @(
{commands}
    )
    $options = @{{
{options}    }}
    $words = @($commandAst.CommandElements | ForEach-Object {{ $_.ToString() }})
    if ($words.Count -le 1 -or ($words.Count -eq 2 -and $wordToComplete -ne '')) {{
        $commands | Where-Object {{ $_.Name -like "$wordToComplete*" }} | ForEach-Object {{
            [System.Management.Automation.CompletionResult]::new($_.Name, $_.Name, 'ParameterValue', $_.About)
        }}
    }} elseif ($options.ContainsKey($words[1])) {{
        $options[$words[1]] | Where-Object {{ $_ -like "$wordToComplete*" }} | ForEach-Object {{
            [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
        }}
    }}
    Get-ChildItem -Path "$wordToComplete*" -ErrorAction SilentlyContinue | Where-Object {{ $_.PSIsContainer -or $_.Extension -eq '.ry' }} | ForEach-Object {{
        [System.Management.Automation.CompletionResult]::new($_.Name, $_.Name, 'ProviderItem', $_.Name)
    }}
}}
"#,
        commands = commands.join("\n"),
    )
}

// ----- rythme man ---------------------------------------------------------------

/// Text for roff: backslashes and the dots or quotes that start a line are escaped.
fn roff(s: &str) -> String {
    s.lines()
        .map(|l| {
            let l = l.replace('\\', "\\e").replace('-', "\\-").replace('"', "\\(dq");
            if l.starts_with('.') || l.starts_with('\'') {
                format!("\\&{l}")
            } else {
                l
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `rythme man`: the manual page of the command, in roff, in the language of the messages.
pub fn man(args: &[String]) -> i32 {
    if let Some(bad) = args.first() {
        crate::io::err_line(&t!("`rythme man` takes no argument, not `{bad}`", "`rythme man` ne prend pas d'argument, pas `{bad}`"));
        return 2;
    }
    crate::io::out(&man_page());
    0
}

pub fn man_page() -> String {
    let mut out = format!(".TH RYTHME 1 \"\" \"rythme {}\" \"Rythme\"\n", crate::VERSION);
    let section = |title: String| format!(".SH {}\n", title.to_uppercase());
    out.push_str(&section(t!("Name", "Nom")));
    out.push_str(&roff(&t!(
        "rythme - a small, purely functional language with help everywhere",
        "rythme - un petit langage purement fonctionnel, avec de l'aide partout"
    )));
    out.push('\n');
    out.push_str(&section(t!("Synopsis", "Synopsis")));
    out.push_str(".B rythme\n[\\fIFILE.ry\\fR | \\fICOMMAND\\fR] [\\fIOPTIONS\\fR]\n");
    out.push_str(&section(t!("Description", "Description")));
    out.push_str(&roff(&t!(
        "Rythme runs, checks, tests and formats programs written in Rythme. Without a command, `rythme` starts the interactive mode; `rythme file.ry` runs a program. Every message has a code explained by `rythme explain CODE`, and `rythme help` shows help on the language and its functions.",
        "Rythme lance, vérifie, teste et met en forme les programmes écrits en Rythme. Sans commande, `rythme` démarre le mode interactif ; `rythme fichier.ry` lance un programme. Chaque message a un code expliqué par `rythme explain CODE`, et `rythme help` montre de l'aide sur le langage et ses fonctions."
    )));
    out.push('\n');
    out.push_str(&section(t!("Commands", "Commandes")));
    for c in COMMANDS {
        out.push_str(&format!(".TP\n.B {}\n{}\n", roff(c.synopsis), roff(&about(c.name))));
        let more = details(c.name);
        if !more.is_empty() {
            out.push_str(&format!(".RS\n.nf\n{}\n.fi\n.RE\n", roff(&more)));
        }
    }
    out.push_str(&section(t!("Options", "Options")));
    let codes = lang::codes();
    out.push_str(&format!(
        ".TP\n.B \\-\\-lang CODE\n{}\n.TP\n.B \\-\\-no\\-color\n{}\n.TP\n.B \\-\\-help\n{}\n.TP\n.B \\-\\-version\n{}\n",
        roff(&t!("the language of the messages: {codes}", "la langue des messages : {codes}")),
        roff(&t!("messages without colors", "des messages sans couleurs")),
        roff(&t!("the help; after a command, its options", "l'aide ; après une commande, ses options")),
        roff(&t!("the version of Rythme", "la version de Rythme")),
    ));
    out.push_str(&section(t!("Environment", "Environnement")));
    out.push_str(&format!(
        ".TP\n.B RYTHME_LANG\n{}\n.TP\n.B NO_COLOR\n{}\n",
        roff(&t!(
            "the language of the messages, before LC_ALL, LC_MESSAGES and LANG",
            "la langue des messages, avant LC_ALL, LC_MESSAGES et LANG"
        )),
        roff(&t!("when set, messages without colors", "quand elle est définie, des messages sans couleurs")),
    ));
    out.push_str(&section(t!("Exit status", "Code de sortie")));
    out.push_str(&roff(&t!(
        "0 when everything went well; 1 after an error in the program, a failed test or a problem found; 2 for a command written wrongly. A program may choose its own code with exit(code).",
        "0 quand tout s'est bien passé ; 1 après une erreur dans le programme, un test qui échoue ou un problème trouvé ; 2 pour une commande mal écrite. Un programme peut choisir son propre code avec exit(code)."
    )));
    out.push('\n');
    out.push_str(&section(t!("See also", "Voir aussi")));
    out.push_str("rythme help, rythme learn, https://github.com/maxlestage/Rythme\n");
    out
}

// ----- programs from the standard input, folders to check --------------------------

/// `rythme check FOLDER`: the .ry files of the folders given (the other
/// names stay as they are).
pub fn expand_folders(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for a in args {
        let path = Path::new(a);
        if path.is_dir() {
            let mut found: Vec<PathBuf> = Vec::new();
            crate::format::ry_files(path, &mut found);
            if found.is_empty() {
                crate::io::err_line(&t!("no .ry file in `{a}`", "aucun fichier .ry dans `{a}`"));
            }
            out.extend(found.into_iter().map(|p| p.display().to_string()));
        } else {
            out.push(a.clone());
        }
    }
    out
}

/// `rythme run -` and `rythme -`: the program is read from the standard input.
pub fn run_stdin(script_args: Vec<String>, seed: Option<i64>, sandbox: bool) -> i32 {
    use std::io::Read;
    let mut code = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut code) {
        crate::io::err_line(&t!("the standard input cannot be read: {e}", "l'entrée standard ne peut pas être lue : {e}"));
        return 1;
    }
    // The program took the standard input: `input()` finds its end.
    crate::io::close_input();
    let mut driver = Driver::new();
    driver.interp.sandboxed = sandbox;
    if let Some(seed) = seed {
        driver.interp.seed_random(seed);
    }
    driver.interp.script_args = script_args;
    let cwd = std::env::current_dir().unwrap_or_default();
    let Some(module) = driver.load_source("<stdin>", &code, None, &cwd) else { return 1 };
    i32::from(driver.execute(&module, Mode::Run).is_err())
}
