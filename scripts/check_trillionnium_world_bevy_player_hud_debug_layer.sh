#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json"
SUMMARY_RAW="$SUMMARY.raw"

mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" player-hud-debug-layer >"$SUMMARY_RAW"
)

jq '
  .status = "player_hud_debug_layer_green"
  | .player_layer_panel_count = (.player_layer.panel_ids | length)
  | .debug_layer_panel_count = (.debug_layer.panel_ids | length)
  | .final_runtime_key_count = (.final_runtime | keys | length)
  | .final_runtime_completed_step_count = (.final_runtime.completed_steps | length)
  | .final_runtime_input_feedback_history_count = (.final_runtime.input_feedback_history | length)
  | .final_runtime_visited_room_count = (.final_runtime.visited_rooms | length)
  | .final_runtime_contextual_action_label_count = (.final_runtime.contextual_action_labels | length)
  | .external_evidence_ignored_for_current_player_hud_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY"
rm -f "$SUMMARY_RAW"

jq -e '
  .contract_version == "trillionnium_world_bevy_player_hud_debug_layer_v1"
  and .status == "player_hud_debug_layer_green"
  and .green == true
  and .player_hud_gate == true
  and .quest_layer_gate == true
  and .debug_layer_gate == true
  and .scene_debug_gate == true
  and .input_hint_gate == true
  and .panel_layer_gate == true
  and .runtime_gate == true
  and .player_layer_panel_count == (.player_layer.panel_ids | length)
  and .debug_layer_panel_count == (.debug_layer.panel_ids | length)
  and .final_runtime_key_count == (.final_runtime | keys | length)
  and .final_runtime_completed_step_count == (.final_runtime.completed_steps | length)
  and .final_runtime_input_feedback_history_count == (.final_runtime.input_feedback_history | length)
  and .final_runtime_visited_room_count == (.final_runtime.visited_rooms | length)
  and .final_runtime_contextual_action_label_count == (.final_runtime.contextual_action_labels | length)
  and (.player_layer.character_status_text | contains("PLAYER HUD"))
  and (.player_layer.character_status_text | contains("DEBUG LAYER") | not)
  and (.player_layer.character_status_text | contains("INPUT SUMMARY") | not)
  and (.debug_layer.event_log_text | contains("DEBUG LAYER"))
  and (.debug_layer.event_log_text | contains("INPUT SUMMARY"))
  and (.debug_layer.scene_state_text | contains("DEBUG LAYER"))
  and (.player_layer.panel_ids | index("top_character_status") != null)
  and (.debug_layer.panel_ids | index("event_log_panel") != null)
  and .external_evidence_ignored_for_current_player_hud_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_PLAYER_HUD_DEBUG_LAYER_GREEN $SUMMARY"
