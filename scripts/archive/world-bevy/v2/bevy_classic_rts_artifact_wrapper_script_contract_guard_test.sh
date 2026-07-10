#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RUNNER="$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh"

if [[ ! -x "$RUNNER" ]]; then
  printf 'MISSING_ARTIFACT_COMMAND_RUNNER %s\n' "$RUNNER" >&2
  exit 1
fi

direct_cargo_hits="$(rg -n 'cargo run -p trnm-world-bevy' "$ROOT"/scripts/check_trillionnium_world_bevy_classic_rts_*.sh || true)"
if [[ -n "$direct_cargo_hits" ]]; then
  printf 'RTS_ACCEPTANCE_DIRECT_CARGO_RUN_FOUND\n%s\n' "$direct_cargo_hits" >&2
  exit 1
fi

wrapper_count="$(rg -l 'run_trillionnium_world_bevy_artifact_command.sh' "$ROOT"/scripts/check_trillionnium_world_bevy_classic_rts_*.sh | wc -l | tr -d ' ')"
if (( wrapper_count < 100 )); then
  printf 'RTS_ACCEPTANCE_ARTIFACT_WRAPPER_COVERAGE_TOO_LOW count=%s\n' "$wrapper_count" >&2
  exit 1
fi

printf 'TRILLIONNIUM_BEVY_CLASSIC_RTS_ARTIFACT_WRAPPER_SCRIPT_CONTRACT_GREEN wrapper_count=%s\n' "$wrapper_count"
