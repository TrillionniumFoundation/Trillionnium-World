#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-live-play-session.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- live-play-session >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_live_play_session_v1"
  and .input_contract == "trillionnium_world_bevy_live_input_sampling_v1"
  and .green == true
  and .running_state_sampling == true
  and (.bevy_window_button_path | contains("Interaction_Pressed"))
  and .keyboard_path_uses_same_action_gate == true
  and (.button_events | length) >= 6
  and (.button_events[] | select(.action == "TALK") | .accepted) == true
  and (.button_events[] | select(.action == "TRAIN") | .accepted) == true
  and (.button_events[] | select(.action == "MOVE:north") | .accepted) == true
  and (.button_events[] | select(.action == "FIGHT") | .accepted) == true
  and (.samples[] | select(.stage == "after_TALK") | .runtime.dialogue_overlay_visible) == true
  and (.samples[] | select(.stage == "after_TRAIN") | .runtime.indoor_tilemap_visible) == true
  and (.samples[] | select(.stage == "after_MOVE:north") | .runtime.tile_step_progress_percent) == 100
  and (.samples[] | select(.stage == "after_FIGHT") | .runtime.combat_overlay_was_visible) == true
  and (.samples[] | select(.stage == "after_FIGHT") | .runtime.enemy_hp) < 40
  and (.samples[] | select(.stage == "after_FIGHT") | .has_quality_surfaces) == true
  and .final_runtime.reward_claimed == true
  and .final_runtime.equipment_ready == true
  and .final_runtime.movement_lock_observed == true
  and .final_runtime.tile_step_progress_percent == 100
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_LIVE_PLAY_SESSION_GREEN %s\n' "$SUMMARY"
