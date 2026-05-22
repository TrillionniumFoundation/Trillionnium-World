#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_army_composition_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_army_composition_gap_v1'
  'bevy-classic-rts-bot-army-composition-gap.json'
  'bevy-classic-rts-bot-army-composition-gap.ppm'
  'classic-rts-bot-army-composition-gap'
  'bevy_army_composition_vocabulary_not_openra_native_unit_mix_ai'
  'bevy_native_army_composition_ai_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_bot_economy_tech_target_commit == "f6c47d9"'
  'openra_bot_beacon_pressure_target_commit == "2b6f25b"'
  'openra_organic_bot_terminal_target_commit == "5f1bf76"'
  'army_composition_signal_count >= 24'
  'final_army_composition_state == "terminal_composition_lock_secured"'
  'army_composition_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot army-composition gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_ARMY_COMPOSITION_GAP_CONTRACT'
  'native_classic_rts_bot_army_composition_gap_evidence_json'
  'classic-rts-bot-army-composition-gap'
  'opening_unit_mix_read'
  'frontline_backline_ratio'
  'counter_mix_swap'
  'reinforce_supply_curve'
  'specialist_timing_window'
  'terminal_composition_lock'
  'bevy_army_composition_vocabulary_not_openra_native_unit_mix_ai'
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'army_composition_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot army-composition gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_army_composition_gap.sh'
  'bevy-classic-rts-bot-army-composition-gap.json'
  'classic_rts_bot_army_composition_gap_green'
  'rts_bot_army_composition_gap_stage_count'
  'rts_bot_army_composition_gap_openra_gap_not_closed_gate'
  'rts_bot_army_composition_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot army-composition gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot army-composition gap evidence remains bound to OpenRA economy/tech, beacon pressure, and organic terminal targets while keeping Bevy native unit-mix AI parity unclaimed"
