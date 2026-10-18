#!/bin/sh
# Fills in the Homebrew formula and the winget manifests of a release.
# Remplit la formule Homebrew et les manifestes winget d'une version.
#
#   packaging/render.sh VERSION SHA256SUMS OUTDIR
#
# VERSION is like 0.1.0 or v0.1.0; SHA256SUMS is the file the release workflow
# publishes (lines "checksum  file"). Writes OUTDIR/rythme.rb and
# OUTDIR/winget/*.yaml, and fails if a checksum is missing.

set -eu

if [ $# -ne 3 ]; then
    echo "usage: packaging/render.sh VERSION SHA256SUMS OUTDIR" >&2
    exit 2
fi
version="${1#v}"
sums="$2"
out="$3"
date="${RELEASE_DATE:-$(date -u +%Y-%m-%d)}"
here="$(cd "$(dirname "$0")" && pwd)"

script="$(mktemp)"
trap 'rm -f "$script"' EXIT
printf 's|@VERSION@|%s|g\ns|@DATE@|%s|g\n' "$version" "$date" >"$script"
while read -r sum file; do
    [ -n "$sum" ] || continue
    file="$(printf '%s' "${file#\*}" | sed 's/[.]/\\./g')"
    printf 's|@SHA256:%s@|%s|g\n' "$file" "$sum" >>"$script"
done <"$sums"

render() {
    sed -f "$script" "$1" >"$2"
    if grep -n -E '@(VERSION|DATE|SHA256:[^@]*)@' "$2" >&2; then
        echo "error: $2 still has a placeholder above (is a file missing from $sums?)" >&2
        exit 1
    fi
}

mkdir -p "$out/winget"
render "$here/homebrew/rythme.rb" "$out/rythme.rb"
for f in "$here"/winget/*.yaml; do
    render "$f" "$out/winget/$(basename "$f")"
done
echo "rendered $out/rythme.rb and $out/winget/ for version $version" >&2
