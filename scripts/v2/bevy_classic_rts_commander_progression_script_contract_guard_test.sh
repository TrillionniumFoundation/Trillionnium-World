#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_commander_progression.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_commander_progression_v1'
  'bevy-classic-rts-commander-progression.json'
  'bevy-classic-rts-commander-progression.ppm'
  'classic-rts-commander-progression'
  'input_path == "apply_live_native_action_with_source(classic_rts_commander_progression_input)"'
  'RTS:QUEUE:commander:loot:enemy_barracks@10,3'
  'RTS:QUEUE:commander:level:mirror_captain@battlefield'
  'RTS:QUEUE:commander:ability:rally_aura@mirror_captain'
  'loot_gate == true'
  'commander_level_gate == true'
  'ability_point_gate == true'
  'aura_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS commander script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMANDER_PROGRESSION_CONTRACT'
  'native_classic_rts_commander_progression_evidence_json'
  'classic-rts-commander-progression'
  'classic_rts_commander_progression_input'
  'rts_commander_unit_id'
  'rts_commander_level'
  'rts_commander_ability_point_count'
  'rts_commander_aura_tile_ids'
  'rts_commander_ability_log'
  'rts_loot_item_ids'
  'rts_loot_pickup_log'
  'CLASSIC_RTS_COMMANDER_COLOR'
  'CLASSIC_RTS_COMMANDER_AURA_COLOR'
  'CLASSIC_RTS_LOOT_COLOR'
  'CLASSIC_RTS_ABILITY_POINT_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS commander source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_commander_progression.sh'
  'bevy-classic-rts-commander-progression.json'
  'classic_rts_commander_progression_green'
  'rts_commander_progression_live_input_gate'
  'rts_commander_progression_aftermath_dependency_gate'
  'rts_commander_progression_loot_gate'
  'rts_commander_progression_level_gate'
  'rts_commander_progression_ability_point_gate'
  'rts_commander_progression_aura_gate'
  'rts_commander_progression_aura_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS commander readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS commander progression evidence remains connected to aftermath rewards, loot, commander level-up, ability-point spend, aura overlays, and readiness"
