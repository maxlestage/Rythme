# Rythme in your editor

*[Version française →](README.fr.md)*

Rythme comes with a language server: `rythme lsp`. Any editor that speaks
the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
starts it and gets, for `.ry` files:

- the problems as you type: every error and warning of `rythme check`, with
  the help to fix them, and the problems of the files you `use`;
- documentation on hover: built-in functions with an example, keywords, your
  functions with their `##` comments, types, constructors; the explanation of
  an error code when the mouse is on the problem;
- completion of the names usable at the cursor, built-ins, keywords,
  snippets, and file names in `use "..."`;
- the parameters of a call while you type it, go to definition (into other
  files too), references, rename (refused when it would change the meaning
  of the code), the outline of the file, quick fixes for typos;
- formatting the document in the style of `rythme fmt`.

First install Rythme itself (see the [main README](../README.md#install)) and
check that `rythme version` works in a terminal. If your editor does not find
the program, give it the full path (for example `~/.cargo/bin/rythme`).

The messages are in the language of `RYTHME_LANG` when it is set, otherwise
in the language of the editor when it tells it (VS Code does), otherwise in
the language of the system. To choose it whatever these say:
`rythme --lang fr lsp`, or the option `{"language": "fr"}` sent at start-up
(`initializationOptions`).

## VS Code

Use the extension in [`vscode/`](vscode): colors, snippets and the language
server, with settings for the program and the language.

## Neovim

Neovim 0.11 or later, in `init.lua`:

```lua
vim.filetype.add({ extension = { ry = "rythme" } })
vim.lsp.config("rythme", {
  cmd = { "rythme", "lsp" },          -- or { "rythme", "--lang", "fr", "lsp" }
  filetypes = { "rythme" },
  root_markers = { ".git" },
})
vim.lsp.enable("rythme")
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rythme",
  callback = function() vim.bo.commentstring = "# %s" end,
})
```

Neovim 0.8 to 0.10:

```lua
vim.filetype.add({ extension = { ry = "rythme" } })
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rythme",
  callback = function(args)
    vim.lsp.start({
      name = "rythme",
      cmd = { "rythme", "lsp" },
      root_dir = vim.fs.dirname(vim.fs.find({ ".git" }, { upward = true, path = args.file })[1]),
    })
  end,
})
```

`K` shows the documentation, `<C-x><C-o>` completes, `gd` (or
`vim.lsp.buf.definition()`) goes to the definition, `vim.lsp.buf.rename()` renames.

### Colors (Tree-sitter)

The [Tree-sitter grammar](tree-sitter-rythme) gives Neovim (0.9 or later) the
colors, and the indentation with nvim-treesitter. From the root of a clone of
this repository (a C compiler is enough), build the parser and copy the
queries:

```sh
mkdir -p ~/.local/share/nvim/site/parser ~/.local/share/nvim/site/queries/rythme
cc -shared -fPIC -O2 -I editors/tree-sitter-rythme/src \
  editors/tree-sitter-rythme/src/parser.c editors/tree-sitter-rythme/src/scanner.c \
  -o ~/.local/share/nvim/site/parser/rythme.so
cp editors/tree-sitter-rythme/queries/*.scm ~/.local/share/nvim/site/queries/rythme/
```

then, in `init.lua`:

```lua
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rythme",
  callback = function()
    vim.treesitter.start()
    -- with nvim-treesitter, the indentation too:
    -- vim.bo.indentexpr = "v:lua.require'nvim-treesitter'.indentexpr()"
  end,
})
```

## Helix

In `~/.config/helix/languages.toml`:

```toml
[language-server.rythme]
command = "rythme"
args = ["lsp"]
# config = { language = "fr" }      # the language of the messages

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

The grammar gives the colors and the indentation: replace `COMMIT` with a
commit of the repository (`git ls-remote https://github.com/maxlestage/Rythme master`
shows the latest one), or use a clone with
`source = { path = "/path/to/Rythme/editors/tree-sitter-rythme" }`. Then build
it and copy its queries, from the root of a clone:

```sh
hx --grammar fetch && hx --grammar build
mkdir -p ~/.config/helix/runtime/queries
cp -r editors/helix/queries/rythme ~/.config/helix/runtime/queries/
```

`hx --health rythme` checks that the program, the grammar and the queries are found.

## Emacs

With Eglot (built into Emacs 29 and later), in `init.el`:

```elisp
(define-derived-mode rythme-mode prog-mode "Rythme"
  "Major mode for Rythme files."
  (setq-local comment-start "# ")
  (setq-local font-lock-defaults
              `((,(regexp-opt '("fn" "if" "then" "else" "match" "end" "do" "type"
                                "test" "use" "and" "or" "not" "true" "false")
                              'symbols)
                 . font-lock-keyword-face))))
(add-to-list 'auto-mode-alist '("\\.ry\\'" . rythme-mode))

(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs '(rythme-mode . ("rythme" "lsp"))))
(add-hook 'rythme-mode-hook #'eglot-ensure)
```

With lsp-mode instead of Eglot:

```elisp
(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(rythme-mode . "rythme"))
  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection '("rythme" "lsp"))
                    :activation-fn (lsp-activate-on "rythme")
                    :server-id 'rythme)))
(add-hook 'rythme-mode-hook #'lsp-deferred)
```

## Zed

The extension in [`zed/`](zed) gives the colors, the indentation, the outline
and the brackets with the Tree-sitter grammar, and starts `rythme lsp`. To
install it from a clone of this repository: in Zed, run the command
*zed: install dev extension* and choose the folder `editors/zed` (Zed builds
it with Rust, installed with rustup).

The settings, in Zed's `settings.json` (all optional):

```json
{
  "lsp": {
    "rythme": {
      "binary": { "path": "/home/me/.cargo/bin/rythme", "arguments": ["lsp"] },
      "initialization_options": { "language": "fr" }
    }
  }
}
```

The extension takes the grammar from this repository at the commit `rev` of
its `extension.toml`: change it when the grammar changes, and after merging a
pull request that changed it (a squash merge gives new commits). Zed wants a
commit SHA, not a branch. To try a grammar from a clone,
`repository = "file:///path/to/Rythme"`.

## Any other editor

| What the editor asks       | Answer                                                   |
|----------------------------|----------------------------------------------------------|
| command                    | `rythme lsp` (`--stdio` is accepted and ignored)         |
| transport                  | standard input and output                                |
| language id, file types    | `rythme`, `.ry`                                          |
| comments                   | `#` (`##` for documentation)                             |
| initialization options     | `{"language": "en"}`, `"fr"` or `"auto"` (optional)       |
| settings                   | `{"rythme": {"language": "fr"}}` changes the language    |
| Tree-sitter grammar        | [`tree-sitter-rythme/`](tree-sitter-rythme), with queries for the colors, the indentation and the scopes |
