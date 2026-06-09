#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_build_lifecycle_v1'
  'bevy-classic-rts-build-lifecycle.json'
  'bevy-classic-rts-build-lifecycle.ppm'
  'classic-rts-build-lifecycle'
  'input_path == "apply_live_native_action_with_source(classic_rts_build_lifecycle_input)"'
  'RTS:QUEUE:build:watch_tower@7,4'
  'RTS:QUEUE:complete:watch_tower@7,4'
  'RTS:QUEUE:repair:watch_tower@7,4'
  'RTS:QUEUE:cancel:build:1'
  'build_placement_gate == true'
  'completion_gate == true'
  'repair_gate == true'
  'cancel_refund_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS build lifecycle script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BUILD_LIFECYCLE_CONTRACT'
  'native_classic_rts_build_lifecycle_evidence_json'
  'classic-rts-build-lifecycle'
  'classic_rts_build_lifecycle_input'
  'rts_completed_structure_ids'
  'rts_repair_target_id'
  'rts_repair_progress_percent'
  'rts_cancelled_structure_ids'
  'rts_refund_delta_log'
  'rts_structure_health_percents'
  'rts_structure_state'
  'classic_rts_structure_tile_for_id'
  'CLASSIC_RTS_STRUCTURE_COMPLETE_COLOR'
  'CLASSIC_RTS_STRUCTURE_REPAIR_COLOR'
  'CLASSIC_RTS_STRUCTURE_CANCEL_COLOR'
  'CLASSIC_RTS_STRUCTURE_HEALTH_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS build lifecycle source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh'
  'bevy-classic-rts-build-lifecycle.json'
  'classic_rts_build_lifecycle_green'
  'rts_build_lifecycle_build_placement_gate'
  'rts_build_lifecycle_completion_gate'
  'rts_build_lifecycle_repair_gate'
  'rts_build_lifecycle_cancel_refund_gate'
  'rts_build_lifecycle_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS build lifecycle readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS build lifecycle evidence remains connected to live build, complete, repair, cancel/refund runtime state, renderer overlays, and readiness"
