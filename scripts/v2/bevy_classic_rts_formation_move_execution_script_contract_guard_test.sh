#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_execution.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_formation_move_execution_v1'
  'bevy-classic-rts-formation-move-execution.json'
  'bevy-classic-rts-formation-move-execution.ppm'
  'classic-rts-formation-move-execution'
  'slot_claim_gate == true'
  'path_reservation_gate == true'
  'stagger_step_gate == true'
  'crowd_avoidance_gate == true'
  'blocked_reroute_gate == true'
  'arrival_lock_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FORMATION_MOVE_EXECUTION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS formation move execution script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FORMATION_MOVE_EXECUTION_CONTRACT'
  'native_classic_rts_formation_move_execution_evidence_json'
  'classic_draw_rts_formation_move_execution_overlay'
  'classic_rts_formation_move_execution_stage'
  'CLASSIC_RTS_FORMATION_EXEC_SLOT_COLOR'
  'CLASSIC_RTS_FORMATION_EXEC_RESERVATION_COLOR'
  'CLASSIC_RTS_FORMATION_EXEC_AVOID_COLOR'
  'CLASSIC_RTS_FORMATION_EXEC_REROUTE_COLOR'
  'CLASSIC_RTS_FORMATION_EXEC_ARRIVAL_COLOR'
  'Original Trillionnium formation move execution overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS formation move execution source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_formation_move_execution.sh'
  'bevy-classic-rts-formation-move-execution.json'
  'classic_rts_formation_move_execution_green'
  'rts_formation_move_execution_live_input_gate'
  'rts_formation_move_execution_slot_claim_gate'
  'rts_formation_move_execution_path_reservation_gate'
  'rts_formation_move_execution_stagger_step_gate'
  'rts_formation_move_execution_crowd_avoidance_gate'
  'rts_formation_move_execution_blocked_reroute_gate'
  'rts_formation_move_execution_arrival_lock_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS formation move execution readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_formation_move_execution_v1'
  'bevy_classic_rts_formation_move_execution_contract_guard'
  'bevy_classic_rts_formation_move_execution_gate'
  'bevy_classic_rts_formation_move_execution_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_formation_move_execution.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS formation move execution release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS formation move execution evidence remains connected to renderer, CLI, readiness, release-review, accepted input, slot claims, path reservation, avoidance, reroute, arrival lock, and original art policy"
