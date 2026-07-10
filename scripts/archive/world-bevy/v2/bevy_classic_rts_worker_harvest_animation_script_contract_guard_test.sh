#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_worker_harvest_animation.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_worker_harvest_animation_v1'
  'bevy-classic-rts-worker-harvest-animation.json'
  'bevy-classic-rts-worker-harvest-animation.ppm'
  'classic-rts-worker-harvest-animation'
  'approach_gate == true'
  'tool_swing_gate == true'
  'resource_pop_gate == true'
  'carry_load_gate == true'
  'dropoff_burst_gate == true'
  'return_path_gate == true'
  'harvest_stage_gate == true'
  'economy_runtime_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_WORKER_HARVEST_ANIMATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS worker harvest animation script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_WORKER_HARVEST_ANIMATION_CONTRACT'
  'native_classic_rts_worker_harvest_animation_evidence_json'
  'classic_rts_worker_harvest_animation_stage'
  'classic_draw_rts_worker_harvest_animation_scene_overlay'
  'CLASSIC_RTS_HARVEST_ANIMATION_APPROACH_COLOR'
  'CLASSIC_RTS_HARVEST_ANIMATION_TOOL_SWING_COLOR'
  'CLASSIC_RTS_HARVEST_ANIMATION_RESOURCE_POP_COLOR'
  'CLASSIC_RTS_HARVEST_ANIMATION_CARRY_LOAD_COLOR'
  'CLASSIC_RTS_HARVEST_ANIMATION_DROPOFF_BURST_COLOR'
  'CLASSIC_RTS_HARVEST_ANIMATION_RETURN_PATH_COLOR'
  'Original Trillionnium worker-harvest animation overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS worker harvest animation source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_worker_harvest_animation.sh'
  'bevy-classic-rts-worker-harvest-animation.json'
  'classic_rts_worker_harvest_animation_green'
  'rts_worker_harvest_animation_approach_gate'
  'rts_worker_harvest_animation_tool_swing_gate'
  'rts_worker_harvest_animation_resource_pop_gate'
  'rts_worker_harvest_animation_carry_load_gate'
  'rts_worker_harvest_animation_dropoff_burst_gate'
  'rts_worker_harvest_animation_return_path_gate'
  'rts_worker_harvest_animation_harvest_stage_gate'
  'rts_worker_harvest_animation_economy_runtime_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS worker harvest animation readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_worker_harvest_animation_v1'
  'bevy_classic_rts_worker_harvest_animation_contract_guard'
  'bevy_classic_rts_worker_harvest_animation_gate'
  'bevy_classic_rts_worker_harvest_animation_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_worker_harvest_animation.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS worker harvest animation release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS worker harvest animation evidence remains connected to renderer, CLI, readiness, release-review, economy runtime, and original art policy"
