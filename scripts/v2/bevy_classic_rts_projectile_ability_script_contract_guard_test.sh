#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_projectile_ability.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_projectile_ability_v1'
  'bevy-classic-rts-projectile-ability.json'
  'bevy-classic-rts-projectile-ability.ppm'
  'classic-rts-projectile-ability'
  'input_path == "apply_live_native_action_with_source(classic_rts_projectile_ability_input)"'
  'RTS:ATTACK:arena_creep_attack'
  'RTS:ABILITY:focus_fire'
  'RTS:ABILITY:guard_break'
  'projectile_trail_gate == true'
  'projectile_impact_gate == true'
  'ability_radius_gate == true'
  'damage_tick_gate == true'
  'armor_shield_gate == true'
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_projectile_ability_core_frame_order_gate == true'
  'rts_projectile_ability_core_headless_replay_gate == true'
  'rts_projectile_ability_core_headless_applied_order_count == 4'
  'rts_projectile_ability_core_headless_ability_order_count == 2'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS projectile ability script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PROJECTILE_ABILITY_CONTRACT'
  'native_classic_rts_projectile_ability_evidence_json'
  'classic-rts-projectile-ability'
  'classic_rts_projectile_ability_input'
  'rts_projectile_trail_tile_ids'
  'rts_projectile_impact_tile_id'
  'rts_active_projectile_id'
  'rts_ability_effect_tile_ids'
  'rts_ability_damage_ticks'
  'rts_target_armor_percent'
  'rts_target_shield_percent'
  'rts_ability_resolution_state'
  'CLASSIC_RTS_PROJECTILE_TRAIL_COLOR'
  'CLASSIC_RTS_PROJECTILE_IMPACT_COLOR'
  'CLASSIC_RTS_ABILITY_RADIUS_COLOR'
  'CLASSIC_RTS_DAMAGE_TICK_COLOR'
  'CLASSIC_RTS_ARMOR_SHIELD_COLOR'
  'RtsFrameOrder::from_live_command_label'
  'first-contact-basin-projectile-ability'
  'rts_projectile_ability_core_frame_order_gate'
  'rts_projectile_ability_core_headless_replay_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS projectile ability source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_projectile_ability.sh'
  'bevy-classic-rts-projectile-ability.json'
  'classic_rts_projectile_ability_green'
  'rts_projectile_ability_projectile_trail_gate'
  'rts_projectile_ability_projectile_impact_gate'
  'rts_projectile_ability_ability_radius_gate'
  'rts_projectile_ability_damage_tick_gate'
  'rts_projectile_ability_armor_shield_gate'
  'rts_projectile_ability_core_frame_order_gate'
  'rts_projectile_ability_core_headless_replay_gate'
  'rts_projectile_ability_core_headless_ability_order_count'
  'rts_projectile_ability_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS projectile ability readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS projectile ability evidence remains connected to ranged attack, impact, ability radius, damage tick, armor/shield runtime state, renderer overlays, and readiness"
