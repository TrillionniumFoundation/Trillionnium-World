#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_objective_victory_loop.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_objective_victory_loop_v1'
  'bevy-classic-rts-objective-victory-loop.json'
  'bevy-classic-rts-objective-victory-loop.ppm'
  'classic-rts-objective-victory-loop'
  'input_path == "apply_live_native_action_with_source(classic_rts_objective_victory_loop_input)"'
  'RTS:QUEUE:objective:claim:relay_beacon@6,5'
  'RTS:QUEUE:objective:extract:relay_beacon@9,2'
  'objective_marker_gate == true'
  'capture_progress_gate == true'
  'victory_resolution_gate == true'
  'defeat_pressure_gate == true'
  'extraction_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS objective victory loop script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OBJECTIVE_VICTORY_LOOP_CONTRACT'
  'native_classic_rts_objective_victory_loop_evidence_json'
  'classic-rts-objective-victory-loop'
  'classic_rts_objective_victory_loop_input'
  'rts_objective_tile_ids'
  'rts_objective_capture_percent'
  'rts_objective_owner_state'
  'rts_objective_result_state'
  'rts_objective_score_delta_log'
  'rts_objective_extraction_tile_id'
  'rts_defeat_risk_percent'
  'CLASSIC_RTS_OBJECTIVE_COLOR'
  'CLASSIC_RTS_CAPTURE_BAR_COLOR'
  'CLASSIC_RTS_VICTORY_COLOR'
  'CLASSIC_RTS_DEFEAT_RISK_COLOR'
  'CLASSIC_RTS_EXTRACTION_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS objective victory loop source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_objective_victory_loop.sh'
  'bevy-classic-rts-objective-victory-loop.json'
  'classic_rts_objective_victory_loop_green'
  'rts_objective_victory_loop_marker_gate'
  'rts_objective_victory_loop_capture_gate'
  'rts_objective_victory_loop_victory_gate'
  'rts_objective_victory_loop_defeat_pressure_gate'
  'rts_objective_victory_loop_extraction_gate'
  'rts_objective_victory_loop_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS objective victory loop readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS objective victory loop evidence remains connected to capture, extraction, victory scoring, defeat pressure, renderer overlays, and readiness"
