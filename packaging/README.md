# Packaging Rythme

*En français plus bas.*

What happens when a version tag (`v1.0.0`) is pushed, and what is left to do
by hand.

| Channel | Done by | Needs |
|---|---|---|
| GitHub release: archives, `SHA256SUMS`, `install.sh`, `install.ps1` | [`release.yml`](../.github/workflows/release.yml) | nothing |
| Homebrew formula `rythme.rb` (attached to the release) | `release.yml` + [`render.sh`](render.sh) | nothing |
| Homebrew tap `maxlestage/homebrew-rythme` | `release.yml` | secret `HOMEBREW_TAP_TOKEN` (else skipped) |
| winget manifests `winget/*.yaml` (attached to the release) | `release.yml` + `render.sh` | nothing |
| winget repository `microsoft/winget-pkgs` | by hand (below) | a GitHub account |
| VS Code Marketplace | [`vscode.yml`](../.github/workflows/vscode.yml) | secret `VSCE_PAT` (else skipped) |
| crates.io | by hand (below) | a crates.io token |

Before tagging: the version in `Cargo.toml` must be the tag without `v`
(`release.yml` stops otherwise), and `editors/vscode/package.json` carries the
version of the extension.

## Templates

[`homebrew/rythme.rb`](homebrew/rythme.rb) and [`winget/`](winget) hold
`@VERSION@`, `@DATE@` and `@SHA256:<archive>@`. `render.sh VERSION SHA256SUMS
OUTDIR` fills them in from the checksums of the release and fails if one is
missing; `tests/install.rs` checks it, and that every archive of `release.yml`
is known to the installers and the templates.

## Homebrew

Once: create the public repository `maxlestage/homebrew-rythme`, and a token
that can push to it (fine-grained, *Contents: read and write*) saved as the
secret `HOMEBREW_TAP_TOKEN` of this repository. Each release then updates
`Formula/rythme.rb` there, and anyone can install with:

```sh
brew install maxlestage/rythme/rythme
```

Without the secret: download `rythme.rb` from the release and commit it as
`Formula/rythme.rb` in the tap. Check a formula with
`brew install --formula ./rythme.rb && brew test rythme && brew audit --strict rythme`.

## winget

Download the four `Rythme.Rythme*.yaml` of the release into
`manifests/r/Rythme/Rythme/<version>/` of a fork of
[microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs), check them
with `winget validate --manifest <folder>` and `winget install --manifest
<folder>`, then open a pull request. Or, on Windows:
`wingetcreate submit --token <github token> <folder>`. Afterwards:
`winget install Rythme.Rythme`.

## crates.io

`cargo package --list` shows what is sent (the interpreter, `docs/`,
`examples/`, the READMEs; not the website, the playground or the editors);
`cargo publish --dry-run` builds it. Then `cargo login` and `cargo publish`.
Afterwards: `cargo install rythme`.

## VS Code

Once: a publisher named `rythme` on the
[Marketplace](https://marketplace.visualstudio.com/manage) (the `publisher`
of `package.json`) and an Azure DevOps personal access token with the
*Marketplace (Manage)* scope, saved as the secret `VSCE_PAT`. Each tag then
publishes the extension if its version is new; the `.vsix` is always kept as
the artifact `rythme-vscode` of the workflow.

---

# Distribuer Rythme

Ce qui se passe quand une étiquette de version (`v1.0.0`) est poussée, et ce
qui reste à faire à la main.

| Canal | Fait par | Demande |
|---|---|---|
| Version GitHub : archives, `SHA256SUMS`, `install.sh`, `install.ps1` | [`release.yml`](../.github/workflows/release.yml) | rien |
| Formule Homebrew `rythme.rb` (jointe à la version) | `release.yml` + [`render.sh`](render.sh) | rien |
| Dépôt Homebrew `maxlestage/homebrew-rythme` | `release.yml` | secret `HOMEBREW_TAP_TOKEN` (sinon sauté) |
| Manifestes winget `winget/*.yaml` (joints à la version) | `release.yml` + `render.sh` | rien |
| Dépôt winget `microsoft/winget-pkgs` | à la main (plus bas) | un compte GitHub |
| VS Code Marketplace | [`vscode.yml`](../.github/workflows/vscode.yml) | secret `VSCE_PAT` (sinon sauté) |
| crates.io | à la main (plus bas) | un jeton crates.io |

Avant d'étiqueter : la version de `Cargo.toml` doit être l'étiquette sans `v`
(sinon `release.yml` s'arrête), et `editors/vscode/package.json` porte la
version de l'extension.

## Modèles

[`homebrew/rythme.rb`](homebrew/rythme.rb) et [`winget/`](winget) contiennent
`@VERSION@`, `@DATE@` et `@SHA256:<archive>@`. `render.sh VERSION SHA256SUMS
DOSSIER` les remplit avec les sommes de la version et échoue s'il en manque
une ; `tests/install.rs` le vérifie, ainsi que chaque archive de `release.yml`
est connue des installeurs et des modèles.

## Homebrew

Une fois : créer le dépôt public `maxlestage/homebrew-rythme`, et un jeton qui
peut y pousser (à grain fin, *Contents: read and write*) enregistré comme
secret `HOMEBREW_TAP_TOKEN` de ce dépôt. Chaque version y met alors à jour
`Formula/rythme.rb`, et chacun peut installer avec :

```sh
brew install maxlestage/rythme/rythme
```

Sans le secret : téléchargez `rythme.rb` de la version et ajoutez-le comme
`Formula/rythme.rb` dans le dépôt Homebrew. Pour vérifier une formule :
`brew install --formula ./rythme.rb && brew test rythme && brew audit --strict rythme`.

## winget

Téléchargez les quatre `Rythme.Rythme*.yaml` de la version dans
`manifests/r/Rythme/Rythme/<version>/` d'une copie (fork) de
[microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs), vérifiez-les
avec `winget validate --manifest <dossier>` et `winget install --manifest
<dossier>`, puis ouvrez une pull request. Ou, sous Windows :
`wingetcreate submit --token <jeton github> <dossier>`. Ensuite :
`winget install Rythme.Rythme`.

## crates.io

`cargo package --list` montre ce qui est envoyé (l'interpréteur, `docs/`,
`examples/`, les README ; pas le site, le bac à sable ni les éditeurs) ;
`cargo publish --dry-run` le construit. Puis `cargo login` et `cargo publish`.
Ensuite : `cargo install rythme`.

## VS Code

Une fois : un éditeur (publisher) nommé `rythme` sur le
[Marketplace](https://marketplace.visualstudio.com/manage) (le `publisher` de
`package.json`) et un jeton d'accès personnel Azure DevOps avec la portée
*Marketplace (Manage)*, enregistré comme secret `VSCE_PAT`. Chaque étiquette
publie alors l'extension si sa version est nouvelle ; le `.vsix` est toujours
gardé comme artefact `rythme-vscode` du workflow.
