#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_spawn_animation.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_production_spawn_animation_v1'
  'bevy-classic-rts-production-spawn-animation.json'
  'bevy-classic-rts-production-spawn-animation.ppm'
  'classic-rts-production-spawn-animation'
  'queue_pulse_gate == true'
  'training_tick_gate == true'
  'spawn_door_gate == true'
  'rally_flag_gate == true'
  'formation_join_gate == true'
  'supply_flash_gate == true'
  'production_stage_gate == true'
  'production_runtime_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_SPAWN_ANIMATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS production spawn animation script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_SPAWN_ANIMATION_CONTRACT'
  'native_classic_rts_production_spawn_animation_evidence_json'
  'classic_rts_production_spawn_animation_stage'
  'classic_draw_rts_production_spawn_animation_scene_overlay'
  'CLASSIC_RTS_PRODUCTION_SPAWN_QUEUE_PULSE_COLOR'
  'CLASSIC_RTS_PRODUCTION_SPAWN_TRAINING_TICK_COLOR'
  'CLASSIC_RTS_PRODUCTION_SPAWN_DOOR_COLOR'
  'CLASSIC_RTS_PRODUCTION_SPAWN_RALLY_FLAG_COLOR'
  'CLASSIC_RTS_PRODUCTION_SPAWN_FORMATION_JOIN_COLOR'
  'CLASSIC_RTS_PRODUCTION_SPAWN_SUPPLY_FLASH_COLOR'
  'Original Trillionnium production-spawn animation overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS production spawn animation source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_spawn_animation.sh'
  'bevy-classic-rts-production-spawn-animation.json'
  'classic_rts_production_spawn_animation_green'
  'rts_production_spawn_animation_queue_pulse_gate'
  'rts_production_spawn_animation_training_tick_gate'
  'rts_production_spawn_animation_spawn_door_gate'
  'rts_production_spawn_animation_rally_flag_gate'
  'rts_production_spawn_animation_formation_join_gate'
  'rts_production_spawn_animation_supply_flash_gate'
  'rts_production_spawn_animation_production_stage_gate'
  'rts_production_spawn_animation_production_runtime_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS production spawn animation readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_production_spawn_animation_v1'
  'bevy_classic_rts_production_spawn_animation_contract_guard'
  'bevy_classic_rts_production_spawn_animation_gate'
  'bevy_classic_rts_production_spawn_animation_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_production_spawn_animation.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS production spawn animation release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS production spawn animation evidence remains connected to renderer, CLI, readiness, release-review, army runtime, and original art policy"
