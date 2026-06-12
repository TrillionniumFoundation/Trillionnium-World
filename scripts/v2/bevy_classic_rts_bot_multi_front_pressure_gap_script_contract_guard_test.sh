#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_multi_front_pressure_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
CORE="$ROOT/trillionnium/crates/trnm-rts-core/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_multi_front_pressure_gap_v1'
  'bevy-classic-rts-bot-multi-front-pressure-gap.json'
  'bevy-classic-rts-bot-multi-front-pressure-gap.ppm'
  'classic-rts-bot-multi-front-pressure-gap'
  'bevy_multi_front_pressure_vocabulary_not_openra_native_split_map_ai'
  'bevy_native_multi_front_ai_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_bot_economy_tech_target_commit == "f6c47d9"'
  'openra_bot_beacon_pressure_target_commit == "2b6f25b"'
  'openra_organic_bot_terminal_target_commit == "5f1bf76"'
  'multi_front_signal_count >= 24'
  'final_multi_front_state == "terminal_collapse_secured"'
  'rts_bot_multi_front_core_frame_order_gate == true'
  'rts_bot_multi_front_core_headless_replay_gate == true'
  'rts_bot_multi_front_core_headless_attack_order_count == 2'
  'rts_bot_multi_front_core_headless_micro_move_order_count == 4'
  'multi_front_pressure_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot multi-front pressure gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_MULTI_FRONT_PRESSURE_GAP_CONTRACT'
  'native_classic_rts_bot_multi_front_pressure_gap_evidence_json'
  'classic-rts-bot-multi-front-pressure-gap'
  'dual_scout_lane_probe'
  'decoy_beacon_pressure'
  'main_force_rotate'
  'reinforce_cross_map'
  'simultaneous_expand_hit'
  'collapse_to_terminal'
  'bevy_multi_front_pressure_vocabulary_not_openra_native_split_map_ai'
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'RtsFrameOrder::from_live_command_label'
  'first-contact-basin-bot-multi-front-pressure'
  'trnm-rts-core-bot-multi-front-pressure-rules-v1'
  'dual_scout_lane_probe'
  'decoy_beacon_pressure'
  'main_force_rotate'
  'reinforce_cross_map'
  'simultaneous_expand_hit'
  'collapse_to_terminal'
  'multi_front_pressure_gap_gate'
  'rts_bot_multi_front_core_frame_order_gate'
  'rts_bot_multi_front_core_headless_replay_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$CORE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot multi-front pressure gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_multi_front_pressure_gap.sh'
  'bevy-classic-rts-bot-multi-front-pressure-gap.json'
  'classic_rts_bot_multi_front_pressure_gap_green'
  'rts_bot_multi_front_pressure_gap_stage_count'
  'rts_bot_multi_front_pressure_gap_openra_gap_not_closed_gate'
  'rts_bot_multi_front_pressure_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot multi-front pressure gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot multi-front pressure gap evidence remains bound to OpenRA economy/tech, beacon pressure, and organic terminal targets while keeping Bevy native split-map AI parity unclaimed"
