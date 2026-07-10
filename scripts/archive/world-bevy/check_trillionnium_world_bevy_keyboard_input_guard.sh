#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-keyboard-input-guard.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- keyboard-input-guard >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_keyboard_input_guard_v1"
  and .live_input_sampling_contract == "trillionnium_world_bevy_live_input_sampling_v1"
  and .contextual_button_guard_contract == "trillionnium_world_bevy_contextual_button_guard_v1"
  and .green == true
  and .keyboard_path_gate == true
  and .disabled_guard == true
  and .unlocked_accepts_guard == true
  and .movement_key_gate == true
  and .blocked_history_gate == true
  and .visual_parity_gate == true
  and .disabled_event_count >= 5
  and .accepted_event_count >= 6
  and (.keyboard_events[0].key == "KeyT")
  and (.keyboard_events[0].action == "TRAIN")
  and (.keyboard_events[0].accepted == false)
  and (.keyboard_events[0].core_state_unchanged_when_disabled == true)
  and (.keyboard_events[2].key == "KeyW")
  and (.keyboard_events[2].action == "MOVE:north")
  and (.keyboard_events[2].accepted == false)
  and (.keyboard_events[7].key == "KeyW")
  and (.keyboard_events[7].action == "MOVE:north")
  and (.keyboard_events[7].accepted == true)
  and (.keyboard_events[7].current_room_after == "league-coliseum")
  and (.keyboard_events[8].key == "Space")
  and (.keyboard_events[8].action == "FIGHT")
  and (.keyboard_events[8].accepted == true)
  and (.final_runtime.equipment_ready == true)
  and (.final_runtime.objective_status == "first_playable_loop_complete")
  and (.final_runtime.blocked_action_history | index("MOVE:north:training_required_before_route_move")) != null
  and (.final_runtime.blocked_action_history | index("LOOT:drop:victory_required_before_loot")) != null
  and (.final_runtime.blocked_action_history | index("EQUIP:bandit_sash:item_not_in_bag:bandit_sash")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_KEYBOARD_INPUT_GUARD_GREEN %s\n' "$SUMMARY"
