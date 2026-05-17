#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json"

mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- player-hud-debug-layer >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_player_hud_debug_layer_v1"
  and .green == true
  and .player_hud_gate == true
  and .quest_layer_gate == true
  and .debug_layer_gate == true
  and .scene_debug_gate == true
  and .input_hint_gate == true
  and .panel_layer_gate == true
  and .runtime_gate == true
  and (.player_layer.character_status_text | contains("PLAYER HUD"))
  and (.player_layer.character_status_text | contains("DEBUG LAYER") | not)
  and (.player_layer.character_status_text | contains("INPUT SUMMARY") | not)
  and (.debug_layer.event_log_text | contains("DEBUG LAYER"))
  and (.debug_layer.event_log_text | contains("INPUT SUMMARY"))
  and (.debug_layer.scene_state_text | contains("DEBUG LAYER"))
  and (.player_layer.panel_ids | index("top_character_status") != null)
  and (.debug_layer.panel_ids | index("event_log_panel") != null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_PLAYER_HUD_DEBUG_LAYER_GREEN $SUMMARY"
