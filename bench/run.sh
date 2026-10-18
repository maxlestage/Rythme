#!/usr/bin/env bash
# Measures the speed of the interpreter: builds the release program, runs each
# benchmark of bench/ several times and shows its best wall time, and checks
# that it still prints exactly its .out file (a checksum of its work).
#
#   bench/run.sh                 every benchmark, best of 5 runs
#   bench/run.sh 10              best of 10 runs
#   bench/run.sh 3 fib sort      only these benchmarks, best of 3 runs
#
# RYTHME=path/to/rythme bench/run.sh measures another build instead.
set -euo pipefail
cd "$(dirname "$0")/.."

runs=5
if [[ $# -gt 0 && $1 =~ ^[0-9]+$ ]]; then
  runs=$1
  shift
fi

if [[ -z ${RYTHME:-} ]]; then
  cargo build --release --quiet
  RYTHME="${CARGO_TARGET_DIR:-target}/release/rythme"
fi

if [[ $# -gt 0 ]]; then
  names=("$@")
else
  names=()
  for file in bench/*.ry; do
    names+=("$(basename "$file" .ry)")
  done
fi

# Microseconds since some point in the past.
now() {
  if [[ -n ${EPOCHREALTIME:-} ]]; then
    local t=${EPOCHREALTIME/[.,]/}
    echo "${t#0}"
  else
    echo $(($(date +%s%N) / 1000))
  fi
}

failed=0
printf '%-10s %10s  %s\n' benchmark 'best (s)' output
for name in "${names[@]}"; do
  file="bench/$name.ry"
  best=
  out=
  for ((i = 0; i < runs; i++)); do
    start=$(now)
    out=$("$RYTHME" "$file" 2>&1) || true
    elapsed=$(($(now) - start))
    if [[ -z $best || $elapsed -lt $best ]]; then
      best=$elapsed
    fi
  done
  if [[ $out == "$(cat "bench/$name.out")" ]]; then
    status=ok
  else
    status="WRONG: $out"
    failed=1
  fi
  printf '%-10s %6d.%03d  %s\n' "$name" $((best / 1000000)) $((best / 1000 % 1000)) "$status"
done
exit $failed
