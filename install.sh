#!/bin/sh
# Installs Rythme on Linux or macOS from the ready-to-use programs of a
# GitHub release, after checking their SHA-256 checksum.
# Installe Rythme sous Linux ou macOS à partir des programmes prêts à l'emploi
# d'une version publiée sur GitHub, après avoir vérifié leur somme SHA-256.
#
#   curl -fsSL https://raw.githubusercontent.com/maxlestage/Rythme/master/install.sh | sh
#   curl -fsSL .../install.sh | sh -s -- --dir /usr/local/bin --version v1.0.0
#
# Options (or environment variables / ou variables d'environnement):
#   --dir DIR        RYTHME_INSTALL_DIR   folder of the program (~/.local/bin)
#   --version TAG    RYTHME_VERSION       a release like v1.0.0 (latest)
#                    RYTHME_DOWNLOAD_URL  where releases are downloaded from
#                    RYTHME_LANG          en or fr (otherwise LC_ALL, LANG...)

set -eu

REPO_RELEASES="https://github.com/maxlestage/Rythme/releases"

# Everything is inside functions: a download cut in the middle runs nothing.
# Tout est dans des fonctions : un téléchargement interrompu ne lance rien.

is_french() {
    lang="${RYTHME_LANG:-${LC_ALL:-${LC_MESSAGES:-${LANG:-}}}}"
    case "$lang" in
        fr*) return 0 ;;
        *) return 1 ;;
    esac
}

# say "English" "Français": prints the message in the language of the user.
say() {
    if is_french; then printf '%s\n' "$2" >&2; else printf '%s\n' "$1" >&2; fi
}

fail() {
    if is_french; then printf 'erreur : %s\n' "$2" >&2; else printf 'error: %s\n' "$1" >&2; fi
    exit 1
}

usage() {
    if is_french; then
        cat >&2 <<'EOF'
Installe Rythme (Linux, macOS).

Utilisation : install.sh [--dir DOSSIER] [--version vX.Y.Z]

  --dir DOSSIER      où placer le programme rythme (par défaut ~/.local/bin)
  --version vX.Y.Z   une version précise (par défaut la dernière)
  --help             cette aide

Variables : RYTHME_INSTALL_DIR, RYTHME_VERSION, RYTHME_DOWNLOAD_URL, RYTHME_LANG.
EOF
    else
        cat >&2 <<'EOF'
Installs Rythme (Linux, macOS).

Usage: install.sh [--dir FOLDER] [--version vX.Y.Z]

  --dir FOLDER       where to put the rythme program (default ~/.local/bin)
  --version vX.Y.Z   a given version (default: the latest)
  --help             this help

Variables: RYTHME_INSTALL_DIR, RYTHME_VERSION, RYTHME_DOWNLOAD_URL, RYTHME_LANG.
EOF
    fi
}

# The name of the archive for this system, as the release workflow builds it.
asset_name() {
    os="$(uname -s)"
    arch="$(uname -m)"
    case "$os" in
        Linux) os=linux ;;
        Darwin)
            os=macos
            # An x86_64 shell under Rosetta on Apple silicon: take the native program.
            if [ "$arch" = x86_64 ] && [ "$(sysctl -n sysctl.proc_translated 2>/dev/null || echo 0)" = 1 ]; then
                arch=arm64
            fi
            ;;
        MINGW* | MSYS* | CYGWIN* | Windows*)
            fail "on Windows, use install.ps1 in PowerShell: irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex" \
                "sous Windows, utilisez install.ps1 dans PowerShell : irm https://raw.githubusercontent.com/maxlestage/Rythme/master/install.ps1 | iex"
            ;;
        *)
            fail "no ready-to-use program for the system $os; install with Rust: cargo install rythme" \
                "pas de programme prêt à l'emploi pour le système $os ; installez avec Rust : cargo install rythme"
            ;;
    esac
    case "$arch" in
        x86_64 | amd64) arch=x86_64 ;;
        aarch64 | arm64) arch=arm64 ;;
        *)
            fail "no ready-to-use program for the processor $arch; install with Rust: cargo install rythme" \
                "pas de programme prêt à l'emploi pour le processeur $arch ; installez avec Rust : cargo install rythme"
            ;;
    esac
    printf 'rythme-%s-%s.tar.gz' "$os" "$arch"
}

# download URL FILE
download() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL --retry 3 -o "$2" "$1"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "$2" "$1"
    else
        fail "curl or wget is needed to download Rythme" \
            "curl ou wget est nécessaire pour télécharger Rythme"
    fi
}

sha256_of() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{print $1}'
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | awk '{print $1}'
    elif command -v openssl >/dev/null 2>&1; then
        openssl dgst -sha256 "$1" | awk '{print $NF}'
    else
        fail "sha256sum, shasum or openssl is needed to check the download" \
            "sha256sum, shasum ou openssl est nécessaire pour vérifier le téléchargement"
    fi
}

main() {
    dir="${RYTHME_INSTALL_DIR:-}"
    version="${RYTHME_VERSION:-latest}"
    while [ $# -gt 0 ]; do
        case "$1" in
            --dir)
                [ $# -ge 2 ] || fail "--dir needs a folder" "--dir attend un dossier"
                dir="$2"
                shift 2
                ;;
            --dir=*) dir="${1#--dir=}"; shift ;;
            --version)
                [ $# -ge 2 ] || fail "--version needs a version like v1.0.0" "--version attend une version comme v1.0.0"
                version="$2"
                shift 2
                ;;
            --version=*) version="${1#--version=}"; shift ;;
            -h | --help) usage; exit 0 ;;
            *) usage; fail "unknown option: $1" "option inconnue : $1" ;;
        esac
    done
    if [ -z "$dir" ]; then
        [ -n "${HOME:-}" ] || fail "HOME is not set; choose a folder with --dir" "HOME n'est pas défini ; choisissez un dossier avec --dir"
        dir="$HOME/.local/bin"
    fi

    base="${RYTHME_DOWNLOAD_URL:-$REPO_RELEASES}"
    base="${base%/}"
    case "$version" in
        latest) url="$base/latest/download" ;;
        v*) url="$base/download/$version" ;;
        *) url="$base/download/v$version"; version="v$version" ;;
    esac

    asset="$(asset_name)"
    tmp="$(mktemp -d 2>/dev/null || mktemp -d -t rythme)"
    trap 'rm -rf "$tmp"' EXIT
    trap 'exit 1' INT TERM

    say "Downloading $asset ($version)..." "Téléchargement de $asset ($version)..."
    download "$url/$asset" "$tmp/$asset" ||
        fail "could not download $url/$asset" "impossible de télécharger $url/$asset"
    download "$url/SHA256SUMS" "$tmp/SHA256SUMS" ||
        fail "could not download the checksums $url/SHA256SUMS" "impossible de télécharger les sommes de contrôle $url/SHA256SUMS"

    expected="$(awk -v f="$asset" '$2 == f || $2 == "*" f {print $1; exit}' "$tmp/SHA256SUMS")"
    [ -n "$expected" ] ||
        fail "$asset is not listed in SHA256SUMS" "$asset n'est pas dans SHA256SUMS"
    actual="$(sha256_of "$tmp/$asset")"
    if [ "$actual" != "$expected" ]; then
        fail "the SHA-256 checksum of $asset is wrong (expected $expected, got $actual); nothing was installed" \
            "la somme SHA-256 de $asset est fausse (attendue $expected, obtenue $actual) ; rien n'a été installé"
    fi
    say "Checksum verified (SHA-256)." "Somme de contrôle vérifiée (SHA-256)."

    mkdir -p "$tmp/out"
    tar -xzf "$tmp/$asset" -C "$tmp/out" ||
        fail "could not extract $asset" "impossible d'extraire $asset"
    [ -f "$tmp/out/rythme" ] ||
        fail "$asset does not contain the rythme program" "$asset ne contient pas le programme rythme"

    mkdir -p "$dir" ||
        fail "could not create the folder $dir" "impossible de créer le dossier $dir"
    # Copy next to the target then rename: replacing a running program works.
    cp "$tmp/out/rythme" "$dir/.rythme.new" ||
        fail "could not write in $dir (try --dir or sudo)" "impossible d'écrire dans $dir (essayez --dir ou sudo)"
    chmod 755 "$dir/.rythme.new"
    mv -f "$dir/.rythme.new" "$dir/rythme"

    installed="$("$dir/rythme" --version 2>/dev/null || echo rythme)"
    say "Installed $installed in $dir/rythme." "$installed installé dans $dir/rythme."

    case ":${PATH:-}:" in
        *":$dir:"* | *":${dir%/}:"*)
            say "Try it: rythme learn" "Essayez : rythme learn"
            ;;
        *)
            say "$dir is not in your PATH. Add this line to ~/.profile (or ~/.bashrc, ~/.zshrc), then open a new terminal:" \
                "$dir n'est pas dans votre PATH. Ajoutez cette ligne à ~/.profile (ou ~/.bashrc, ~/.zshrc), puis ouvrez un nouveau terminal :"
            # shellcheck disable=SC2016 # $PATH is written as is, for the profile
            printf '    export PATH="%s:$PATH"\n' "$dir" >&2
            ;;
    esac
}

main "$@"
