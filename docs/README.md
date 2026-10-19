# Rythme documentation

*[English](#in-english) · [Français](#en-français)*

## In English

- [**Reference manual**](reference.md): the complete, precise description of
  the language and its tools: lexical structure, values and types,
  expressions, definitions, errors and their codes, standard library, command
  line, interactive mode, playground, formatter and language server. Every
  example shows its real output.
- [**Teaching with Rythme**](teaching.md): why Rythme fits a course, a
  12-session course outline, exercises with automatic checking.
- [**Exercises**](exercises/README.md): 36 graded exercises for a first
  course (six about AI), with hints and tests, built into `rythme exercise`.
- **Inside the interpreter**: `rythme learn` (a guided tour),
  `rythme help` (topics and functions), `rythme help builtins` (every built-in
  function), `rythme explain E003` (any error code), `rythme help fmt` (the
  official layout), `rythme help editors`, `help(x)` in a program, `:doc` in
  the interactive mode.
- [**Editors**](../editors/README.md): the language server, `rythme lsp`, in
  VS Code, Neovim, Helix, Zed, Emacs...
- **For AI assistants**: [`llms-full.txt`](llms-full.txt), the whole
  language in one text (`rythme help llms`), its index
  [`llms.txt`](llms.txt), the [JSON output](rythme-output.schema.json) of
  `--format json`, and `rythme mcp` ([manual, 7.10](reference.md#710-rythme-and-ai-assistants)).
- [**Examples**](../examples): small complete programs, each with its expected
  output.
- [**Contributing**](../CONTRIBUTING.md): how the interpreter is organised and
  how to change it.

## En français

- [**Manuel de référence**](reference.fr.md) : la description complète et
  précise du langage et de ses outils : structure lexicale, valeurs et types,
  expressions, définitions, erreurs et leurs codes, bibliothèque standard,
  ligne de commande, mode interactif, playground, mise en forme et serveur de
  langage. Chaque exemple montre ce qu'il affiche réellement.
- [**Enseigner avec Rythme**](enseigner.md) : pourquoi Rythme convient à un
  cours, un plan de cours en 12 séances, des exercices corrigés
  automatiquement.
- [**Exercices**](exercises/README.fr.md) : 36 exercices gradués pour un
  premier cours (dont six sur l'IA), avec des indices et des tests, intégrés à `rythme exercise`.
- **Dans l'interpréteur** : `rythme learn` (une visite guidée),
  `rythme help` (sujets et fonctions), `rythme help builtins` (toutes les
  fonctions intégrées), `rythme explain E003` (n'importe quel code d'erreur),
  `rythme help fmt` (la mise en forme officielle), `rythme help editors`,
  `help(x)` dans un programme, `:doc` dans le mode interactif. Ajoutez
  `--lang fr` (ou définissez `RYTHME_LANG=fr`) pour tout avoir en français.
- [**Éditeurs**](../editors/README.fr.md) : le serveur de langage,
  `rythme lsp`, dans VS Code, Neovim, Helix, Zed, Emacs...
- **Pour les assistants d'IA** : [`llms-full.txt`](llms-full.txt), tout le
  langage en un texte (`rythme help llms`, en anglais), son index
  [`llms.txt`](llms.txt), la [sortie JSON](rythme-output.schema.json) de
  `--format json`, et `rythme mcp` ([manuel, 7.10](reference.fr.md#710-rythme-et-les-assistants-dia)).
- [**Exemples**](../examples) : de petits programmes complets, chacun avec ce
  qu'il doit afficher.
- [**Contribuer**](../CONTRIBUTING.md) (en anglais) : l'organisation de
  l'interpréteur et la façon de le modifier.
