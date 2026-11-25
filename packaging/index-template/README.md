# The package index of Rythme

*En français plus bas.*

This repository is the package index of [Rythme](https://github.com/maxlestage/Rythme):
one file per package, `packages/NAME.toml`, which says where the package
is, what it is for, and for each version its tag, its commit and the
checksum of its files. There is no server: `rythme` reads this repository
with git and keeps a copy of it.

```sh
rythme search vectors        # find packages
rythme info mathlib          # what the index says of one
rythme add mathlib           # add it to a project, then install it
rythme update                # the newest versions within the ranges of rythme.toml
```

The whole story is in the manual of Rythme, section 7.11
([English](https://github.com/maxlestage/Rythme/blob/master/docs/reference.md#711-the-package-index),
[français](https://github.com/maxlestage/Rythme/blob/master/docs/reference.fr.md#711-lindex-des-paquets)),
and in `rythme help packages`.

## Publishing a package

1. Your package is a project made by `rythme new`, alone at the root of a
   public git repository, with these keys at the top of its `rythme.toml`:

   ```toml
   name = "mathlib"
   version = "1.2.0"
   description = "Vectors, matrices and a few numbers"
   license = "MIT"
   repository = "https://github.com/someone/mathlib"
   keywords = ["math", "vectors"]          # optional, at most 5
   authors = ["Someone <someone@example.com>"]   # optional
   ```

2. Commit everything, tag the version and push the tag:
   `git tag v1.2.0 && git push origin v1.2.0`.
3. Run `rythme publish` in the folder of the package. It checks
   `rythme.toml`, the tag, `rythme check` and `rythme test`, then prints the
   text of the entry, with the commit and the checksum. It never sends
   anything by itself.
4. Fork this repository, create `packages/mathlib.toml` with that text (or,
   for a new version of a package already here, add its `[[version]]` block
   at the end of the file), commit, push, and open a pull request.

The checks of the pull request (`.github/workflows/check.yml`) run
`rythme publish --verify` on each entry that changed: every key known, the
name of the file, and for each version that its commit is in the repository,
that its tag is that commit, that its files have the checksum, and that its
`rythme.toml` says this name and this version. They also refuse any change
to the lines of a version already published.

## The rules

- **A name is a Rythme name in lower case** (letters, digits, `_`): it is
  used in `use mathlib/vectors`. First come, first served; a name is not
  taken over, and is not reserved without a package.
- **A published version never changes.** To fix one, publish a new version.
  To withdraw one (a security problem, wrong results), add to its block a
  line `yanked = "why, and which version to use"`: it is never chosen
  again, but projects that locked it still install it.
- **Packages run nowhere here.** The checks only read files; maintainers
  read the pull request before merging it.
- Removing a package entirely is for the maintainers, in serious cases
  (malware, law), and is written down in the pull request that does it.

## An entry

```toml
name = "mathlib"
description = "Vectors, matrices and a few numbers"
repository = "https://github.com/someone/mathlib"
license = "MIT"
keywords = ["math", "vectors"]
authors = ["Someone <someone@example.com>"]

[[version]]
version = "1.0.0"
tag = "v1.0.0"
commit = "4d868c2b0f6c1e1a8d7b8f5d3c2a1b0e9f8d7c6b"
checksum = "sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
```

- `repository` uses `https://` (or `ssh://`, `git@host:path`): Rythme
  refuses other protocols.
- `commit` is the full commit of the tag (40 hexadecimal digits).
- `checksum` is `sha256:` followed by the SHA-256 of, for each file of the
  commit in the order of the bytes of its path: its mode (`100644`), a
  space, its path and a zero byte, then its size in bytes, a zero byte and
  its content (for a submodule: its commit and a zero byte). `rythme
  publish` computes it; Rythme checks it before installing a version.

[`packages/mathlib.toml`](packages/mathlib.toml) is an example with three
versions, one of them withdrawn.

## Creating this repository (for the owner of Rythme)

This folder, `packaging/index-template/` in the repository of Rythme, is the
start of the index that `rythme` uses by default,
`https://github.com/maxlestage/rythme-index`:

1. Create the public repository `maxlestage/rythme-index` on GitHub.
2. Copy this README and `.github/workflows/check.yml` into it, with an empty
   folder `packages/` (a file `packages/.keep`), but **not** the example
   `packages/mathlib.toml`, whose repository, commits and checksums are made
   up; push to `main`.
3. In the settings of the repository, protect `main`: pull requests only,
   with the check *Check the entries* required.

Another index (for a school, a team) is made the same way, anywhere git can
reach; projects choose it with `index = "..."` in their `rythme.toml`, or
everyone with the variable `RYTHME_INDEX`. A folder with a folder `packages/`
is an index too, read as it is, without git.

---

# L'index des paquets de Rythme

Ce dépôt est l'index des paquets de [Rythme](https://github.com/maxlestage/Rythme) :
un fichier par paquet, `packages/NOM.toml`, qui dit où est le paquet, à
quoi il sert, et pour chaque version son tag, son commit et la somme de
contrôle de ses fichiers. Il n'y a pas de serveur : `rythme` lit ce dépôt
avec git et en garde une copie.

```sh
rythme search vecteurs       # trouver des paquets
rythme info mathlib          # ce que l'index dit de l'un d'eux
rythme add mathlib           # l'ajouter à un projet, puis l'installer
rythme update                # les versions les plus récentes dans les intervalles de rythme.toml
```

Tout est dans le manuel de Rythme, section 7.11, et dans
`rythme help packages`.

## Publier un paquet

1. Votre paquet est un projet fait par `rythme new`, seul à la racine d'un
   dépôt git public, avec ces clés en haut de son `rythme.toml` :
   `description`, `license`, `repository` (et, si vous voulez, `keywords`,
   au plus 5, et `authors`).
2. Validez tout, étiquetez la version et poussez le tag :
   `git tag v1.2.0 && git push origin v1.2.0`.
3. Lancez `rythme publish` dans le dossier du paquet. Il vérifie
   `rythme.toml`, le tag, `rythme check` et `rythme test`, puis affiche le
   texte de l'entrée, avec le commit et la somme de contrôle. Il n'envoie
   jamais rien lui-même.
4. Faites une copie (fork) de ce dépôt, créez `packages/mathlib.toml` avec
   ce texte (ou, pour une nouvelle version d'un paquet déjà là, ajoutez son
   bloc `[[version]]` à la fin du fichier), validez, poussez, et ouvrez une
   pull request.

Les vérifications de la pull request lancent `rythme publish --verify` sur
chaque entrée changée, et refusent tout changement des lignes d'une version
déjà publiée.

## Les règles

- **Un nom est un nom Rythme en minuscules** (lettres, chiffres, `_`) : il
  sert dans `use mathlib/vecteurs`. Premier arrivé, premier servi ; un nom
  n'est pas repris, ni réservé sans paquet.
- **Une version publiée ne change jamais.** Pour en corriger une, publiez
  une nouvelle version. Pour en retirer une (un problème de sécurité, des
  résultats faux), ajoutez à son bloc une ligne
  `yanked = "pourquoi, et quelle version utiliser"` : elle n'est plus jamais
  choisie, mais les projets qui l'ont verrouillée l'installent encore.
- **Les paquets ne tournent nulle part ici.** Les vérifications ne font que
  lire des fichiers ; les responsables lisent la pull request avant de la
  fusionner.
- Retirer complètement un paquet revient aux responsables, dans les cas
  graves (logiciel malveillant, loi), et s'écrit dans la pull request qui
  le fait.

## Créer ce dépôt (pour le propriétaire de Rythme)

Ce dossier, `packaging/index-template/` dans le dépôt de Rythme, est le
début de l'index que `rythme` utilise par défaut,
`https://github.com/maxlestage/rythme-index` :

1. Créez le dépôt public `maxlestage/rythme-index` sur GitHub.
2. Copiez-y ce README et `.github/workflows/check.yml`, avec un dossier
   `packages/` vide (un fichier `packages/.keep`), mais **pas** l'exemple
   `packages/mathlib.toml`, dont le dépôt, les commits et les sommes sont
   inventés ; poussez sur `main`.
3. Dans les réglages du dépôt, protégez `main` : pull requests seulement,
   avec la vérification *Check the entries* obligatoire.

Un autre index (pour une école, une équipe) se fait de la même façon,
partout où git peut aller ; les projets le choisissent avec
`index = "..."` dans leur `rythme.toml`, ou tout le monde avec la variable
`RYTHME_INDEX`. Un dossier qui contient un dossier `packages/` est aussi un
index, lu tel quel, sans git.
