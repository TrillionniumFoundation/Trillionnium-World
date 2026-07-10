#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_endurance_skirmish_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_endurance_skirmish_gap_v1'
  'bevy-classic-rts-endurance-skirmish-gap.json'
  'bevy-classic-rts-endurance-skirmish-gap.ppm'
  'classic-rts-endurance-skirmish-gap'
  'bevy_endurance_vocabulary_not_openra_headless_client_match'
  'bevy_headless_match_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_endurance_skirmish_target_commit == "2cb80a0"'
  'openra_longrun_skirmish_target_commit == "5227d99"'
  'openra_multibot_autostart_target_commit == "4b966c1"'
  'configured_seconds >= 120'
  'winner_claimed == false'
  'endurance_skirmish_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS endurance skirmish gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ENDURANCE_SKIRMISH_GAP_CONTRACT'
  'native_classic_rts_endurance_skirmish_gap_evidence_json'
  'classic-rts-endurance-skirmish-gap'
  'room_autostart'
  'endurance_summary'
  'bevy_endurance_vocabulary_not_openra_headless_client_match'
  'OPENRA_ENDURANCE_SKIRMISH_COMMIT'
  'OPENRA_LONGRUN_SKIRMISH_COMMIT'
  'OPENRA_MULTIBOT_AUTOSTART_COMMIT'
  'endurance_skirmish_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS endurance skirmish gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_endurance_skirmish_gap.sh'
  'bevy-classic-rts-endurance-skirmish-gap.json'
  'classic_rts_endurance_skirmish_gap_green'
  'rts_endurance_skirmish_gap_stage_count'
  'rts_endurance_skirmish_gap_openra_gap_not_closed_gate'
  'rts_endurance_skirmish_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS endurance skirmish gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS endurance skirmish gap evidence remains bound to OpenRA endurance/longrun/autostart targets while keeping Bevy headless-match parity unclaimed"
