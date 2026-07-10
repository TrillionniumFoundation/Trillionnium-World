#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_locomotion_blend.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_locomotion_blend_v1'
  'bevy-classic-rts-locomotion-blend.json'
  'bevy-classic-rts-locomotion-blend.ppm'
  'classic-rts-locomotion-blend'
  'path_gate == true'
  'left_step_gate == true'
  'right_step_gate == true'
  'turn_gate == true'
  'slide_gate == true'
  'brake_gate == true'
  'locomotion_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LOCOMOTION_BLEND_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS locomotion blend script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LOCOMOTION_BLEND_CONTRACT'
  'native_classic_rts_locomotion_blend_evidence_json'
  'classic_rts_locomotion_blend_stage'
  'classic_draw_rts_locomotion_blend_marks'
  'CLASSIC_RTS_LOCOMOTION_PATH_COLOR'
  'CLASSIC_RTS_LOCOMOTION_LEFT_STEP_COLOR'
  'CLASSIC_RTS_LOCOMOTION_RIGHT_STEP_COLOR'
  'CLASSIC_RTS_LOCOMOTION_TURN_COLOR'
  'CLASSIC_RTS_LOCOMOTION_SLIDE_COLOR'
  'CLASSIC_RTS_LOCOMOTION_BRAKE_COLOR'
  'Original Trillionnium locomotion blend overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS locomotion blend source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_locomotion_blend.sh'
  'bevy-classic-rts-locomotion-blend.json'
  'classic_rts_locomotion_blend_green'
  'rts_locomotion_blend_path_gate'
  'rts_locomotion_blend_left_step_gate'
  'rts_locomotion_blend_right_step_gate'
  'rts_locomotion_blend_turn_gate'
  'rts_locomotion_blend_slide_gate'
  'rts_locomotion_blend_brake_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS locomotion blend readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_locomotion_blend_v1'
  'bevy_classic_rts_locomotion_blend_contract_guard'
  'bevy_classic_rts_locomotion_blend_gate'
  'bevy_classic_rts_locomotion_blend_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_locomotion_blend.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS locomotion blend release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS locomotion blend evidence remains connected to renderer, readiness, release review, and original art policy"
