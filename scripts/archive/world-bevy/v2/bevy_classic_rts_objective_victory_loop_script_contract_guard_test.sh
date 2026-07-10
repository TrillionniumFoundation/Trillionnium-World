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
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_objective_core_frame_order_gate == true'
  'rts_objective_core_headless_replay_gate == true'
  'rts_objective_core_headless_objective_order_count == 2'
  'rts_objective_core_headless_capture_order_count == 1'
  'rts_objective_core_headless_extract_order_count == 1'
  'objective_marker_gate == true'
  'capture_progress_gate == true'
  'victory_resolution_gate == true'
  'defeat_pressure_gate == true'
  'extraction_gate == true'
  'openra_parity_target_commit == "5f1bf76"'
  'openra_parity_target_natural_terminal == true'
  'openra_parity_target_winner_beacons == 2'
  'openra_parity_target_total_beacons == 4'
  'openra_parity_target_hold_ticks == 3000'
  'bevy_terminal_parity_claimed == false'
  'bevy_objective_controlled_beacons == 2'
  'bevy_objective_total_beacons == 4'
  'bevy_objective_control_ratio_percent == 50'
  'bevy_objective_hold_ticks == 3000'
  'openra_parity_bridge_gate == true'
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
  'OPENRA_ORGANIC_TERMINAL_COMMIT'
  'OPENRA_ORGANIC_TERMINAL_PACKAGE'
  'OPENRA_ORGANIC_TERMINAL_WINNER_BEACONS'
  'OPENRA_ORGANIC_TERMINAL_TOTAL_BEACONS'
  'OPENRA_ORGANIC_TERMINAL_HOLD_TICKS'
  'rts_objective_core_frame_order_gate'
  'rts_objective_core_headless_replay_gate'
  'RtsOrderKind::Capture'
  'RtsOrderKind::Extract'
  'rts_objective_core_headless_objective_ids'
  'bevy_terminal_parity_claimed'
  'openra_parity_bridge_gate'
  'control_2_of_4_flux_beacons_for_3000_ticks'
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
  'rts_objective_victory_loop_openra_parity_bridge_gate'
  'rts_objective_victory_loop_core_frame_order_gate'
  'rts_objective_victory_loop_core_headless_replay_gate'
  'rts_objective_victory_loop_core_objective_order_count'
  'rts_objective_victory_loop_bevy_terminal_parity_claimed'
  'rts_objective_victory_loop_bevy_controlled_beacons'
  'rts_objective_victory_loop_bevy_total_beacons'
  'rts_objective_victory_loop_bevy_hold_ticks'
  'rts_objective_victory_loop_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS objective victory loop readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS objective victory loop evidence remains connected to capture, extraction, victory scoring, defeat pressure, OpenRA parity target binding, renderer overlays, and readiness"
