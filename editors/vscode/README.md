# Rythme for Visual Studio Code

Help everywhere, inside the editor, for the [Rythme](https://github.com/maxlestage/Rythme)
language (`.ry` files), in English and in French.

*[Version française plus bas](#rythme-pour-visual-studio-code).*

- **Problems as you type**: every error and warning of `rythme check`, with
  its explanation and how to fix it; the problems of the files you `use` are
  shown on the `use` line. Hover a problem to read the long explanation of
  its code (`rythme explain`). Typos get a quick fix (`Ctrl+.`).
- **Documentation on hover**: built-in functions (with an example),
  keywords, your own functions with their `##` comments, types,
  constructors, parameters.
- **Completion** of the names you can use at the cursor, the built-in
  functions, keywords and snippets (`fn … end`, `match … end`, `test`...),
  and of file names in `use "..."`.
- **The parameters of a call** while you type it, **go to definition**
  (`F12`, also into other files), **find references** (`Shift+F12`),
  **rename** (`F2`, refused when it would change the meaning of the code),
  the **outline** of the file, highlights of a name.
- **Format Document** (`Shift+Alt+F`, or on save with `editor.formatOnSave`):
  the layout of `rythme fmt`.
- **The types of the names**, when the checker is sure of them
  (`names: List(Text)`, `fn area(r) → Float`) and what the code of a function
  needs its parameters to be (`fn half(x: Int|Float)`), **colors** that follow what
  each name is (a function, a parameter, a constructor...), **▶ Run** above
  the first line that does something and **▶ Run test** above each test,
  the names of the whole workspace (`Ctrl+T`), folding, and growing the
  selection (`Shift+Alt+→`). The types can be hidden with the VS Code setting
  `editor.inlayHints.enabled`.
- Colors, automatic indentation and snippets, even without the program.
- Commands (`Ctrl+Shift+P`, then *Rythme*): run the file, run its tests,
  explain an error code, help on a topic or a function, restart the
  language server. The *run* buttons are also at the top right of the editor.

## Install

1. **Install Rythme** itself (the `rythme` program), as explained in the
   [main README](https://github.com/maxlestage/Rythme#install): the install
   script (`install.sh`, `install.ps1`), a ready-made program from the
   Releases page, or `cargo install --git https://github.com/maxlestage/Rythme`.
   Check it in a terminal: `rythme version`.
2. **Install the extension**: from the Visual Studio Marketplace (search
   *Rythme* in the extensions view) once it is published there, or from this
   folder (it needs [Node.js](https://nodejs.org)):

   ```sh
   cd editors/vscode
   npm install
   npx @vscode/vsce package          # makes rythme-1.0.0.vsix
   code --install-extension rythme-1.0.0.vsix
   ```

   Or, to work on the extension: open `editors/vscode` in VS Code, run
   `npm install`, and press `F5` (a VS Code window starts with the extension).
   `npm run check` loads the extension and talks to the language server
   without VS Code (after `cargo build` at the root of the repository).

Open a `.ry` file: the language server starts by itself.

## Settings

| Setting               | Default  | What it does                                                        |
|-----------------------|----------|---------------------------------------------------------------------|
| `rythme.path`         | `rythme` | the program; found on the `PATH`, then in `~/.cargo/bin`             |
| `rythme.language`     | `auto`   | `en`, `fr`, or `auto` (the language of VS Code, or `RYTHME_LANG`)   |
| `rythme.trace.server` | `off`    | `messages` or `verbose` shows the exchanges in the *Rythme* output |

## When something goes wrong

- *The program `rythme` was not found*: install it, or set `rythme.path` to
  its full path (for example `C:\Users\ada\.cargo\bin\rythme.exe`, or
  `./target/debug/rythme` when you work on Rythme itself).
- No problems shown: look at *View → Output → Rythme*, then run
  *Rythme: Restart the language server*. A version of Rythme without the
  `rythme lsp` command is too old: update it.

## Snippets

| Type      | You get                                   |
|-----------|-------------------------------------------|
| `fn`      | `fn name(x) = x`                          |
| `fnb`     | a function on several lines, with `end`   |
| `lambda`  | `fn(x) = x`                               |
| `if`      | `if condition then value else other`      |
| `match`   | a `match` with two cases                  |
| `type`    | `type Name = First(field) \| Second`      |
| `test`    | `test "..." = assert_eq(actual, expected)` |
| `pipe`    | a `|> map |> filter` pipeline             |

Other editors (Neovim, Helix, Zed, Emacs...) can use the same language
server: see [`editors/README.md`](../README.md).

---

# Rythme pour Visual Studio Code

De l'aide partout, dans l'éditeur, pour le langage
[Rythme](https://github.com/maxlestage/Rythme) (fichiers `.ry`), en français
et en anglais.

- **Les problèmes pendant la frappe** : chaque erreur et avertissement de
  `rythme check`, avec son explication et comment corriger ; les problèmes
  des fichiers utilisés par `use` sont signalés sur la ligne du `use`.
  Survolez un problème pour lire l'explication détaillée de son code
  (`rythme explain`). Les fautes de frappe ont une correction rapide (`Ctrl+.`).
- **La documentation au survol** : fonctions intégrées (avec un exemple),
  mots-clés, vos propres fonctions avec leurs commentaires `##`, types,
  constructeurs, paramètres.
- **La complétion** des noms utilisables à l'endroit du curseur, des
  fonctions intégrées, des mots-clés et des extraits (`fn … end`,
  `match … end`, `test`...), et des noms de fichiers dans `use "..."`.
- **Les paramètres d'un appel** pendant qu'on l'écrit, **aller à la
  définition** (`F12`, aussi dans d'autres fichiers), **trouver les
  références** (`Maj+F12`), **renommer** (`F2`, refusé si cela changeait le
  sens du code), le **plan** du fichier, la mise en évidence d'un nom.
- **Mettre le document en forme** (`Maj+Alt+F`, ou à l'enregistrement avec
  `editor.formatOnSave`) : la mise en forme de `rythme fmt`.
- **Les types des noms**, quand le vérificateur en est sûr
  (`noms: List(Text)`, `fn aire(r) → Float`) et ce que le code d'une fonction
  demande à ses paramètres (`fn moitie(x: Int|Float)`), des **couleurs** qui suivent ce
  qu'est chaque nom (une fonction, un paramètre, un constructeur...),
  **▶ Lancer** au-dessus de la première ligne qui fait quelque chose et
  **▶ Lancer le test** au-dessus de chaque test, les noms de tout l'espace de
  travail (`Ctrl+T`), le pliage, et l'agrandissement de la sélection
  (`Maj+Alt+→`). Le réglage `editor.inlayHints.enabled` de VS Code masque les
  types.
- Couleurs, indentation automatique et extraits de code, même sans le programme.
- Des commandes (`Ctrl+Maj+P`, puis *Rythme*) : lancer le fichier, lancer
  ses tests, expliquer un code d'erreur, l'aide sur un sujet ou une
  fonction, redémarrer le serveur de langage. Les boutons *lancer* sont aussi
  en haut à droite de l'éditeur.

## Installer

1. **Installez Rythme** lui-même (le programme `rythme`), comme expliqué
   dans le [README principal](https://github.com/maxlestage/Rythme/blob/main/README.fr.md#installer) :
   le script d'installation (`install.sh`, `install.ps1`), un programme prêt
   à l'emploi depuis la page Releases, ou
   `cargo install --git https://github.com/maxlestage/Rythme`.
   Vérifiez dans un terminal : `rythme version`.
2. **Installez l'extension** : depuis le Visual Studio Marketplace (cherchez
   *Rythme* dans la vue des extensions) une fois qu'elle y est publiée, ou
   depuis ce dossier (il faut [Node.js](https://nodejs.org)) :

   ```sh
   cd editors/vscode
   npm install
   npx @vscode/vsce package          # crée rythme-1.0.0.vsix
   code --install-extension rythme-1.0.0.vsix
   ```

   Ou, pour travailler sur l'extension : ouvrez `editors/vscode` dans VS
   Code, lancez `npm install`, et appuyez sur `F5` (une fenêtre de VS Code
   démarre avec l'extension). `npm run check` charge l'extension et dialogue
   avec le serveur de langage sans VS Code (après `cargo build` à la racine
   du dépôt).

Ouvrez un fichier `.ry` : le serveur de langage démarre tout seul.

## Réglages

| Réglage               | Par défaut | Rôle                                                                  |
|-----------------------|------------|-----------------------------------------------------------------------|
| `rythme.path`         | `rythme`   | le programme ; cherché dans le `PATH`, puis dans `~/.cargo/bin`        |
| `rythme.language`     | `auto`     | `en`, `fr`, ou `auto` (la langue de VS Code, ou `RYTHME_LANG`)        |
| `rythme.trace.server` | `off`      | `messages` ou `verbose` montre les échanges dans la sortie *Rythme* |

## Quand quelque chose ne va pas

- *Le programme `rythme` est introuvable* : installez-le, ou donnez son
  chemin complet dans `rythme.path` (par exemple
  `C:\Users\ada\.cargo\bin\rythme.exe`, ou `./target/debug/rythme` quand vous
  travaillez sur Rythme lui-même).
- Aucun problème affiché : regardez *Affichage → Sortie → Rythme*, puis
  lancez *Rythme : Redémarrer le serveur de langage*. Une version de Rythme
  sans la commande `rythme lsp` est trop ancienne : mettez-la à jour.

Les autres éditeurs (Neovim, Helix, Zed, Emacs...) peuvent utiliser le même
serveur de langage : voir [`editors/README.fr.md`](../README.fr.md).
