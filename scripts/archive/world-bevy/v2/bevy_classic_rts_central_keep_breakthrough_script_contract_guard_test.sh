#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_breakthrough.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_central_keep_breakthrough_v1'
  'bevy-classic-rts-central-keep-breakthrough.json'
  'bevy-classic-rts-central-keep-breakthrough.ppm'
  'classic-rts-central-keep-breakthrough'
  'input_path == "apply_live_native_action_with_source(classic_rts_central_keep_breakthrough_input)"'
  'RTS:QUEUE:tier2:keep_breach:central_keep@13,3'
  'RTS:QUEUE:tier2:guardian_counter:high_warden@13,4'
  'RTS:QUEUE:tier2:keep_hold:final_line@12,4'
  'RTS:QUEUE:tier2:keep_break:central_keep@13,3'
  'RTS:QUEUE:tier2:keep_claim:central_keep@13,3'
  'central_keep_pressure_dependency_gate == true'
  'keep_breach_gate == true'
  'guardian_counter_gate == true'
  'keep_hold_gate == true'
  'keep_break_gate == true'
  'keep_claim_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS central keep breakthrough script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CENTRAL_KEEP_BREAKTHROUGH_CONTRACT'
  'native_classic_rts_central_keep_breakthrough_evidence_json'
  'classic-rts-central-keep-breakthrough'
  'classic_rts_central_keep_breakthrough_input'
  'rts_keep_breach_tile_ids'
  'rts_keep_breach_percent'
  'rts_guardian_counter_unit_ids'
  'rts_keep_claim_tile_ids'
  'rts_victory_banner_state'
  'rts_central_keep_breakthrough_state'
  'CLASSIC_RTS_KEEP_BREACH_COLOR'
  'CLASSIC_RTS_KEEP_COUNTER_COLOR'
  'CLASSIC_RTS_KEEP_CLAIM_COLOR'
  'CLASSIC_RTS_KEEP_VICTORY_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS central keep breakthrough source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_central_keep_breakthrough.sh'
  'bevy-classic-rts-central-keep-breakthrough.json'
  'classic_rts_central_keep_breakthrough_green'
  'rts_central_keep_breakthrough_live_input_gate'
  'rts_central_keep_breakthrough_pressure_dependency_gate'
  'rts_central_keep_breakthrough_breach_gate'
  'rts_central_keep_breakthrough_guardian_counter_gate'
  'rts_central_keep_breakthrough_hold_gate'
  'rts_central_keep_breakthrough_break_gate'
  'rts_central_keep_breakthrough_claim_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS central keep breakthrough readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS central keep breakthrough evidence remains connected to keep pressure dependency, breach, guardian counter, final hold, keep break, claim, and readiness"
