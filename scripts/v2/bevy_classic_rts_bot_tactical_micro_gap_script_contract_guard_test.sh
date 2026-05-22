#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
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
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'tactical_micro_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
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
  'rts_bot_tactical_micro_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot tactical micro gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot tactical micro gap evidence remains bound to OpenRA economy/tech, beacon pressure, and organic terminal targets while keeping Bevy native combat AI parity unclaimed"
