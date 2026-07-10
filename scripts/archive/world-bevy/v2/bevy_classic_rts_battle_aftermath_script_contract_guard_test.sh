#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_battle_aftermath.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_battle_aftermath_v1'
  'bevy-classic-rts-battle-aftermath.json'
  'bevy-classic-rts-battle-aftermath.ppm'
  'classic-rts-battle-aftermath'
  'runtime_screen_mode == "player_runtime_battle_aftermath_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'player_first_battle_aftermath_screen_gate == true'
  'input_path == "apply_live_native_action_with_source(classic_rts_battle_aftermath_input)"'
  'RTS:QUEUE:aftermath:destroy:enemy_barracks@10,3'
  'RTS:QUEUE:aftermath:promote:control_group_3@10,3'
  'RTS:QUEUE:aftermath:next:secure_expansion@9,2'
  'destruction_gate == true'
  'veteran_gate == true'
  'match_result_gate == true'
  'next_action_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS battle aftermath script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BATTLE_AFTERMATH_CONTRACT'
  'native_classic_rts_battle_aftermath_evidence_json'
  'classic-rts-battle-aftermath'
  'classic_rts_battle_aftermath_input'
  'rts_aftermath_destroyed_structure_ids'
  'rts_aftermath_debris_tile_ids'
  'rts_aftermath_smoke_tile_ids'
  'rts_veteran_unit_ids'
  'rts_veteran_level_log'
  'rts_match_result_state'
  'rts_next_action_ids'
  'CLASSIC_RTS_DEBRIS_COLOR'
  'CLASSIC_RTS_SMOKE_COLOR'
  'CLASSIC_RTS_VETERAN_COLOR'
  'CLASSIC_RTS_MATCH_RESULT_COLOR'
  'CLASSIC_RTS_NEXT_ACTION_COLOR'
  'player_runtime_battle_aftermath_screen'
  'battle_aftermath_pixel_counts'
  'player_first_battle_aftermath_screen_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS battle aftermath source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_battle_aftermath.sh'
  'bevy-classic-rts-battle-aftermath.json'
  'classic_rts_battle_aftermath_green'
  'rts_battle_aftermath_live_input_gate'
  'rts_battle_aftermath_assault_dependency_gate'
  'rts_battle_aftermath_destruction_gate'
  'rts_battle_aftermath_veteran_gate'
  'rts_battle_aftermath_match_result_gate'
  'rts_battle_aftermath_next_action_gate'
  'rts_battle_aftermath_debris_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS battle aftermath readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS battle aftermath evidence remains connected to base assault, building destruction, debris and smoke, veteran promotion, match result, next action routing, renderer overlays, and readiness"
