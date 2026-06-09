#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${TRNM_WORLD_BEVY_ARTIFACT_BIN:-}"

if [[ -n "$BIN" ]]; then
  if [[ ! -x "$BIN" ]]; then
    printf 'TRNM_WORLD_BEVY_ARTIFACT_BIN is not executable: %s\n' "$BIN" >&2
    exit 1
  fi
  exec "$BIN" "$@"
fi

cd "$ROOT/trillionnium"
CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" exec cargo run -p trnm-world-bevy -- "$@"
