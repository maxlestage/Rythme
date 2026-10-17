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
  fautes de frappe.

Installez d'abord Rythme lui-même (voir le [README principal](../README.fr.md#installer))
et vérifiez que `rythme version` marche dans un terminal. Si votre éditeur ne
trouve pas le programme, donnez-lui le chemin complet (par exemple
`~/.cargo/bin/rythme`).

Les messages sont dans la langue de l'éditeur quand il la donne (VS Code le
fait), sinon dans celle de `RYTHME_LANG` ou du système. Pour choisir :
`rythme --lang fr lsp`, ou l'option `{"language": "fr"}` envoyée au démarrage
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
```

`hx --health rythme` vérifie que le programme est trouvé.

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

Zed ajoute les langages par des extensions, et un langage y a besoin d'une
grammaire Tree-sitter, que Rythme n'a pas encore. La partie serveur de langage
d'une telle extension est courte : dans son `extension.toml`,

```toml
[language_servers.rythme]
name = "Rythme"
languages = ["Rythme"]
```

et dans son `src/lib.rs` (avec la bibliothèque `zed_extension_api`) :

```rust
impl zed::Extension for RythmeExtension {
    fn language_server_command(&mut self, _: &zed::LanguageServerId, worktree: &zed::Worktree) -> zed::Result<zed::Command> {
        let command = worktree.which("rythme").ok_or("installez Rythme : `rythme` n'est pas dans le PATH")?;
        Ok(zed::Command { command, args: vec!["--lang".into(), "fr".into(), "lsp".into()], env: Default::default() })
    }
}
```

## Tout autre éditeur

| Ce que demande l'éditeur        | Réponse                                                    |
|---------------------------------|------------------------------------------------------------|
| commande                        | `rythme lsp` (`--stdio` est accepté et ignoré)             |
| transport                       | entrée et sortie standard                                  |
| identifiant, types de fichiers  | `rythme`, `.ry`                                            |
| commentaires                    | `#` (`##` pour la documentation)                           |
| options d'initialisation        | `{"language": "fr"}`, `"en"` ou `"auto"` (facultatif)      |
| réglages                        | `{"rythme": {"language": "fr"}}` change la langue          |
