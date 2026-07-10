#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_player_hud_debug_layer_v1'
  'player_hud_debug_layer_green'
  'bevy-player-hud-debug-layer.json'
  'SUMMARY_RAW="$SUMMARY.raw.$$"'
  'SUMMARY_TMP="$SUMMARY.tmp.$$"'
  'trap cleanup EXIT'
  'player-hud-debug-layer'
  'native_player_hud_debug_layer_evidence_json'
  'PLAYER HUD'
  'DEBUG LAYER'
  'INPUT SUMMARY'
  'player_hud_gate == true'
  'quest_layer_gate == true'
  'debug_layer_gate == true'
  'scene_debug_gate == true'
  'input_hint_gate == true'
  'panel_layer_gate == true'
  'runtime_gate == true'
  'gate_count = ($gates | length)'
  'passed_gate_count = ($gates | map(select(. == true)) | length)'
  'failed_gate_count = (.gate_count - .passed_gate_count)'
  'gate_count == 7'
  'passed_gate_count == 7'
  'failed_gate_count == 0'
  'player_layer_panel_count == (.player_layer.panel_ids | length)'
  'debug_layer_panel_count == (.debug_layer.panel_ids | length)'
  'final_runtime_key_count == (.final_runtime | keys | length)'
  'final_runtime_completed_step_count == (.final_runtime.completed_steps | length)'
  'final_runtime_input_feedback_history_count == (.final_runtime.input_feedback_history | length)'
  'final_runtime_visited_room_count == (.final_runtime.visited_rooms | length)'
  'final_runtime_contextual_action_label_count == (.final_runtime.contextual_action_labels | length)'
  'external_evidence_ignored_for_current_player_hud_pass'
  'android_s5_real_device_claimed == false'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'third_party_asset_copied == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] player HUD/debug layer contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] player HUD/debug layer keeps player-facing HUD separate from DEBUG/INPUT diagnostics without S5/public/OpenRA-copy claims"
