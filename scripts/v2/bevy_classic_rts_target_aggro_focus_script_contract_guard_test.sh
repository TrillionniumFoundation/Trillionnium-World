#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_target_aggro_focus.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_target_aggro_focus_v1'
  'bevy-classic-rts-target-aggro-focus.json'
  'bevy-classic-rts-target-aggro-focus.ppm'
  'classic-rts-target-aggro-focus'
  'input_path == "apply_live_native_action_with_source(classic_rts_targeting_input)"'
  'RTS:MOVE:8,4:wedge'
  'RTS:ATTACK:arena_creep_attack'
  'RTS:ABILITY:focus_fire'
  'focus_fire:arena_creep_attack'
  'target_priority_gate == true'
  'aggro_gate == true'
  'focus_fire_gate == true'
  'threat_feedback_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS target script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TARGET_AGGRO_FOCUS_CONTRACT'
  'native_classic_rts_target_aggro_focus_evidence_json'
  'classic-rts-target-aggro-focus'
  'classic_rts_targeting_input'
  'rts_target_priority_ids'
  'rts_aggro_target_id'
  'rts_focus_fire_unit_ids'
  'rts_threat_level_percents'
  'rts_targeting_state'
  'classic_rts_target_priority_ids_for_target'
  'classic_rts_focus_fire_units_for_target'
  'classic_rts_threat_levels_for_target'
  'CLASSIC_RTS_TARGET_PRIORITY_COLOR'
  'CLASSIC_RTS_AGGRO_RING_COLOR'
  'CLASSIC_RTS_FOCUS_FIRE_COLOR'
  'CLASSIC_RTS_THREAT_BAR_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS target source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_target_aggro_focus.sh'
  'bevy-classic-rts-target-aggro-focus.json'
  'classic_rts_target_aggro_focus_green'
  'rts_targeting_target_priority_gate'
  'rts_targeting_aggro_gate'
  'rts_targeting_focus_fire_gate'
  'rts_targeting_threat_feedback_gate'
  'rts_targeting_focus_fire_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS target readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS target aggro focus evidence remains connected to live input, runtime targeting state, renderer overlays, and readiness"
