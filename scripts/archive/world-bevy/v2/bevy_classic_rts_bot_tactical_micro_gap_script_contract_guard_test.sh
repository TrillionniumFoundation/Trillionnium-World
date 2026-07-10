#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CORE="$ROOT/trillionnium/crates/trnm-rts-core/src/lib.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap_v1'
  'bevy-classic-rts-bot-tactical-micro-gap.json'
  'bevy-classic-rts-bot-tactical-micro-gap.ppm'
  'classic-rts-bot-tactical-micro-gap'
  'bevy_tactical_micro_vocabulary_not_openra_native_combat_ai'
  'bevy_native_combat_ai_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_bot_economy_tech_target_commit == "f6c47d9"'
  'openra_bot_beacon_pressure_target_commit == "2b6f25b"'
  'openra_organic_bot_terminal_target_commit == "5f1bf76"'
  'micro_signal_count >= 24'
  'rts_bot_tactical_micro_core_frame_order_gate == true'
  'rts_bot_tactical_micro_core_headless_replay_gate == true'
  'rts_bot_tactical_micro_core_headless_focus_fire_order_count == 1'
  'rts_bot_tactical_micro_core_headless_micro_move_order_count == 3'
  'final_micro_state == "pullback_regroup_reattack"'
  'tactical_micro_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot tactical micro gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_TACTICAL_MICRO_GAP_CONTRACT'
  'native_classic_rts_bot_tactical_micro_gap_evidence_json'
  'classic-rts-bot-tactical-micro-gap'
  'target_priority_probe'
  'focus_fire_commit'
  'kite_and_stutter_step'
  'flank_angle_split'
  'ability_timing_window'
  'low_health_pullback_regroup'
  'bevy_tactical_micro_vocabulary_not_openra_native_combat_ai'
  'RTS:FOCUS:low_armor_striker'
  'trnm-rts-core-bot-tactical-micro-rules-v1'
  'RtsOrderKind::FocusFire'
  'RtsTacticalCombatCheckpoint'
  'tactical_combat'
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'rts_bot_tactical_micro_core_frame_order_gate'
  'rts_bot_tactical_micro_core_headless_replay_gate'
  'tactical_micro_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN" "$CORE"; then
    echo "[FAIL] missing classic RTS bot tactical micro gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh'
  'bevy-classic-rts-bot-tactical-micro-gap.json'
  'classic_rts_bot_tactical_micro_gap_green'
  'rts_bot_tactical_micro_gap_stage_count'
  'rts_bot_tactical_micro_gap_openra_gap_not_closed_gate'
  'rts_bot_tactical_micro_gap_core_frame_order_gate'
  'rts_bot_tactical_micro_gap_core_headless_replay_gate'
  'rts_bot_tactical_micro_gap_core_frame_order_stream_sha256'
  'rts_bot_tactical_micro_gap_core_headless_checkpoint_sha256'
  'rts_bot_tactical_micro_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot tactical micro gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot tactical micro gap evidence remains bound to OpenRA economy/tech, beacon pressure, organic terminal targets, trnm-rts-core replay, and Bevy native combat AI no-claim boundary"
