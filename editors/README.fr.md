# Rythme dans votre éditeur

*[English version →](README.md)*

Rythme est livré avec un serveur de langage : `rythme lsp`. Tout éditeur qui
parle le [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
le lance et obtient, pour les fichiers `.ry` :

- les problèmes pendant la frappe : chaque erreur et avertissement de
  `rythme check`, avec l'aide pour les corriger, et les problèmes des fichiers
  utilisés par `use` ;
- la documentation au survol : fonctions intégrées avec un exemple,
  mots-clés, vos fonctions avec leurs commentaires `##`, types,
  constructeurs ; l'explication d'un code d'erreur quand la souris est sur le
  problème ;
- la complétion des noms utilisables à l'endroit du curseur, des fonctions
  intégrées, des mots-clés, des extraits de code, et des noms de fichiers
  dans `use "..."` ;
- les paramètres d'un appel pendant qu'on l'écrit, aller à la définition
  (aussi dans d'autres fichiers), les références, renommer (refusé si cela
  changeait le sens du code), le plan du fichier, les corrections rapides des
  fautes de frappe ;
- la mise en forme du document dans le style de `rythme fmt`.

Installez d'abord Rythme lui-même (voir le [README principal](../README.fr.md#installer))
et vérifiez que `rythme version` marche dans un terminal. Si votre éditeur ne
trouve pas le programme, donnez-lui le chemin complet (par exemple
`~/.cargo/bin/rythme`).

Les messages sont dans la langue de `RYTHME_LANG` quand elle est définie,
sinon dans celle de l'éditeur quand il la donne (VS Code le fait), sinon dans
celle du système. Pour la choisir quoi qu'il en soit : `rythme --lang fr lsp`,
ou l'option `{"language": "fr"}` envoyée au démarrage
(`initializationOptions`).

## VS Code

Utilisez l'extension de [`vscode/`](vscode) : couleurs, extraits de code et
serveur de langage, avec des réglages pour le programme et la langue.

## Neovim

Neovim 0.11 ou plus récent, dans `init.lua` :

```lua
vim.filetype.add({ extension = { ry = "rythme" } })
vim.lsp.config("rythme", {
  cmd = { "rythme", "--lang", "fr", "lsp" },
  filetypes = { "rythme" },
  root_markers = { ".git" },
})
vim.lsp.enable("rythme")
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rythme",
  callback = function() vim.bo.commentstring = "# %s" end,
})
```

Neovim 0.8 à 0.10 :

```lua
vim.filetype.add({ extension = { ry = "rythme" } })
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rythme",
  callback = function(args)
    vim.lsp.start({
      name = "rythme",
      cmd = { "rythme", "--lang", "fr", "lsp" },
      root_dir = vim.fs.dirname(vim.fs.find({ ".git" }, { upward = true, path = args.file })[1]),
    })
  end,
})
```

`K` montre la documentation, `<C-x><C-o>` complète, `gd` (ou
`vim.lsp.buf.definition()`) va à la définition, `vim.lsp.buf.rename()` renomme.

### Couleurs (Tree-sitter)

La [grammaire Tree-sitter](tree-sitter-rythme) donne à Neovim (0.9 ou plus
récent) les couleurs, et l'indentation avec nvim-treesitter. Depuis la racine
d'un clone de ce dépôt (un compilateur C suffit), compilez l'analyseur et
copiez les requêtes :

```sh
mkdir -p ~/.local/share/nvim/site/parser ~/.local/share/nvim/site/queries/rythme
cc -shared -fPIC -O2 -I editors/tree-sitter-rythme/src \
  editors/tree-sitter-rythme/src/parser.c editors/tree-sitter-rythme/src/scanner.c \
  -o ~/.local/share/nvim/site/parser/rythme.so
cp editors/tree-sitter-rythme/queries/*.scm ~/.local/share/nvim/site/queries/rythme/
```

puis, dans `init.lua` :

```lua
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rythme",
  callback = function()
    vim.treesitter.start()
    -- avec nvim-treesitter, l'indentation aussi :
    -- vim.bo.indentexpr = "v:lua.require'nvim-treesitter'.indentexpr()"
  end,
})
```

## Helix

Dans `~/.config/helix/languages.toml` :

```toml
[language-server.rythme]
command = "rythme"
args = ["lsp"]
config = { language = "fr" }        # la langue des messages

[[language]]
name = "rythme"
scope = "source.rythme"
file-types = ["ry"]
roots = []
comment-token = "#"
indent = { tab-width = 2, unit = "  " }
language-servers = ["rythme"]
grammar = "rythme"

[[grammar]]
name = "rythme"
source = { git = "https://github.com/maxlestage/Rythme", rev = "COMMIT", subpath = "editors/tree-sitter-rythme" }
```

La grammaire donne les couleurs et l'indentation : remplacez `COMMIT` par un
commit du dépôt (`git ls-remote https://github.com/maxlestage/Rythme master`
montre le dernier), ou utilisez un clone avec
`source = { path = "/chemin/vers/Rythme/editors/tree-sitter-rythme" }`. Puis
compilez-la et copiez ses requêtes, depuis la racine d'un clone :

```sh
hx --grammar fetch && hx --grammar build
mkdir -p ~/.config/helix/runtime/queries
cp -r editors/helix/queries/rythme ~/.config/helix/runtime/queries/
```

`hx --health rythme` vérifie que le programme, la grammaire et les requêtes sont trouvés.

## Emacs

Avec Eglot (intégré à Emacs 29 et suivants), dans `init.el` :

```elisp
(define-derived-mode rythme-mode prog-mode "Rythme"
  "Mode majeur pour les fichiers Rythme."
  (setq-local comment-start "# ")
  (setq-local font-lock-defaults
              `((,(regexp-opt '("fn" "if" "then" "else" "match" "end" "do" "type"
                                "test" "use" "and" "or" "not" "true" "false")
                              'symbols)
                 . font-lock-keyword-face))))
(add-to-list 'auto-mode-alist '("\\.ry\\'" . rythme-mode))

(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs '(rythme-mode . ("rythme" "--lang" "fr" "lsp"))))
(add-hook 'rythme-mode-hook #'eglot-ensure)
```

Avec lsp-mode plutôt qu'Eglot :

```elisp
(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(rythme-mode . "rythme"))
  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection '("rythme" "--lang" "fr" "lsp"))
                    :activation-fn (lsp-activate-on "rythme")
                    :server-id 'rythme)))
(add-hook 'rythme-mode-hook #'lsp-deferred)
```

## Zed

L'extension dans [`zed/`](zed) donne les couleurs, l'indentation, le plan et
les paires de crochets avec la grammaire Tree-sitter, et lance `rythme lsp`.
Pour l'installer depuis un clone de ce dépôt : dans Zed, lancez la commande
*zed: install dev extension* et choisissez le dossier `editors/zed` (Zed la
compile avec Rust, installé avec rustup).

Les réglages, dans le `settings.json` de Zed (tous facultatifs) :

```json
{
  "lsp": {
    "rythme": {
      "binary": { "path": "/home/moi/.cargo/bin/rythme", "arguments": ["lsp"] },
      "initialization_options": { "language": "fr" }
    }
  }
}
```

L'extension prend la grammaire dans ce dépôt au commit `rev` de son
`extension.toml` : changez-le quand la grammaire change, et après la fusion
d'une pull request qui l'a changée (une fusion « squash » donne de nouveaux
commits). Zed veut l'identifiant (SHA) d'un commit, pas une branche. Pour
essayer la grammaire d'un clone, `repository = "file:///chemin/vers/Rythme"`.

## Tout autre éditeur

| Ce que demande l'éditeur        | Réponse                                                    |
|---------------------------------|------------------------------------------------------------|
| commande                        | `rythme lsp` (`--stdio` est accepté et ignoré)             |
| transport                       | entrée et sortie standard                                  |
| identifiant, types de fichiers  | `rythme`, `.ry`                                            |
| commentaires                    | `#` (`##` pour la documentation)                           |
| options d'initialisation        | `{"language": "fr"}`, `"en"` ou `"auto"` (facultatif)      |
| réglages                        | `{"rythme": {"language": "fr"}}` change la langue          |
| grammaire Tree-sitter           | [`tree-sitter-rythme/`](tree-sitter-rythme), avec les requêtes des couleurs, de l'indentation et des portées |
