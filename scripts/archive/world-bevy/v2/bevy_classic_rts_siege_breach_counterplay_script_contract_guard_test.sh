#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_siege_breach_counterplay.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_siege_breach_counterplay_v1'
  'bevy-classic-rts-siege-breach-counterplay.json'
  'bevy-classic-rts-siege-breach-counterplay.ppm'
  'classic-rts-siege-breach-counterplay'
  'input_path == "apply_live_native_action_with_source(classic_rts_siege_breach_counterplay_input)"'
  'RTS:QUEUE:tier2:breach:gate_bulwark@10,3'
  'RTS:QUEUE:tier2:enemy_repair:gate_bulwark@10,3'
  'RTS:QUEUE:tier2:enemy_flank:ridge_sentries@9,4'
  'RTS:QUEUE:tier2:hold:shield_line@9,3'
  'RTS:QUEUE:tier2:finish:gate_bulwark@10,3'
  'tier_two_dependency_gate == true'
  'breach_window_gate == true'
  'repair_reaction_gate == true'
  'flank_pressure_gate == true'
  'hold_line_gate == true'
  'resolution_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS siege breach counterplay script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SIEGE_BREACH_COUNTERPLAY_CONTRACT'
  'native_classic_rts_siege_breach_counterplay_evidence_json'
  'classic-rts-siege-breach-counterplay'
  'classic_rts_siege_breach_counterplay_input'
  'rts_siege_breach_target_id'
  'rts_siege_breach_tile_ids'
  'rts_enemy_repair_unit_ids'
  'rts_enemy_flank_unit_ids'
  'rts_player_hold_tile_ids'
  'rts_siege_breach_state'
  'CLASSIC_RTS_SIEGE_BREACH_COLOR'
  'CLASSIC_RTS_ENEMY_REPAIR_COLOR'
  'CLASSIC_RTS_ENEMY_FLANK_COLOR'
  'CLASSIC_RTS_PLAYER_HOLD_COLOR'
  'CLASSIC_RTS_COUNTERPLAY_RESOLUTION_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS siege breach counterplay source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_siege_breach_counterplay.sh'
  'bevy-classic-rts-siege-breach-counterplay.json'
  'classic_rts_siege_breach_counterplay_green'
  'rts_siege_breach_counterplay_live_input_gate'
  'rts_siege_breach_counterplay_tier_two_dependency_gate'
  'rts_siege_breach_counterplay_breach_window_gate'
  'rts_siege_breach_counterplay_repair_reaction_gate'
  'rts_siege_breach_counterplay_flank_pressure_gate'
  'rts_siege_breach_counterplay_hold_line_gate'
  'rts_siege_breach_counterplay_resolution_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS siege breach counterplay readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS siege breach counterplay evidence remains connected to tier-two siege dependency, enemy repair/flank reactions, hold-line response, final breach resolution, and readiness"
