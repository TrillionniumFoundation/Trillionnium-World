#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output="${1:-$repo_root/run/perf/trnm-perf-matrix.tsv}"
mkdir -p "$(dirname "$output")"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

measure() {
  local name="$1"
  shift
  /usr/bin/time -f '%e\t%M' -o "$tmp/$name.time" "$@" >"$tmp/$name.out" 2>"$tmp/$name.err"
  read -r seconds rss_kib < "$tmp/$name.time"
  printf '%s\t%s\t%s\n' "$name" "$seconds" "$rss_kib" >> "$output"
}

printf 'gate\tseconds\tmax_rss_kib\n' > "$output"
measure rpg_core cargo test --manifest-path "$repo_root/trillionnium/Cargo.toml" -p trnm-rpg-core --all-targets
measure campaign_core cargo test --manifest-path "$repo_root/trillionnium/Cargo.toml" -p trnm-campaign-core --all-targets
measure rts_sim cargo test --manifest-path "$repo_root/trillionnium/Cargo.toml" -p trnm-rts-sim --all-targets
measure closed_loop cargo test --manifest-path "$repo_root/trillionnium/Cargo.toml" -p trnm-rts-sim --test campaign_closed_loop
measure release_build cargo build --manifest-path "$repo_root/trillionnium/Cargo.toml" --release -p trnm-first-contact

awk -F '\t' 'NR > 1 && ($2 > 90 || $3 > 4194304) { bad=1 } END { exit bad }' "$output"
echo "TRNM performance matrix: green ($output)"
