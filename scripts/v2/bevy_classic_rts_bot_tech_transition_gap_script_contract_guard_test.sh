#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tech_transition_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_tech_transition_gap_v1'
  'bevy-classic-rts-bot-tech-transition-gap.json'
  'bevy-classic-rts-bot-tech-transition-gap.ppm'
  'classic-rts-bot-tech-transition-gap'
  'bevy_tech_transition_vocabulary_not_openra_native_tech_switch_ai'
  'bevy_native_tech_transition_ai_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_bot_economy_tech_target_commit == "f6c47d9"'
  'openra_bot_beacon_pressure_target_commit == "2b6f25b"'
  'openra_organic_bot_terminal_target_commit == "5f1bf76"'
  'tech_transition_signal_count >= 24'
  'final_tech_transition_state == "terminal_tech_lock_secured"'
  'tech_transition_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot tech-transition gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_TECH_TRANSITION_GAP_CONTRACT'
  'native_classic_rts_bot_tech_transition_gap_evidence_json'
  'classic-rts-bot-tech-transition-gap'
  'early_signal_read'
  'counter_tech_switch'
  'anti_air_timing'
  'siege_response_window'
  'upgrade_timing_push'
  'terminal_tech_lock'
  'bevy_tech_transition_vocabulary_not_openra_native_tech_switch_ai'
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'tech_transition_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot tech-transition gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_tech_transition_gap.sh'
  'bevy-classic-rts-bot-tech-transition-gap.json'
  'classic_rts_bot_tech_transition_gap_green'
  'rts_bot_tech_transition_gap_stage_count'
  'rts_bot_tech_transition_gap_openra_gap_not_closed_gate'
  'rts_bot_tech_transition_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot tech-transition gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot tech-transition gap evidence remains bound to OpenRA economy/tech, beacon pressure, and organic terminal targets while keeping Bevy native tech-switch AI parity unclaimed"
