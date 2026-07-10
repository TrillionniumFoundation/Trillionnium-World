#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-multi-room-playability.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- multi-room-playability >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_multi_room_playability_v1"
  and .green == true
  and .running_state_sampling == true
  and (.bevy_window_button_path | contains("Interaction_Pressed"))
  and (.button_events | length) >= 9
  and (.button_events[] | select(.action == "ROOM:starter-studio") | .accepted) == true
  and (.button_events[] | select(.action == "NPC:npc-starter-artisan") | .accepted) == true
  and (.button_events[] | select(.action == "TASK:task-starter-forge-intro") | .accepted) == true
  and (.button_events[] | select(.action == "ROOM:client-board") | .accepted) == true
  and .room_count_visited >= 5
  and (.final_runtime.visited_rooms | index("starter-studio")) != null
  and (.final_runtime.visited_rooms | index("forge-workbench")) != null
  and (.final_runtime.visited_rooms | index("asset-yard")) != null
  and (.final_runtime.visited_rooms | index("zbj-market-gate")) != null
  and (.final_runtime.visited_rooms | index("client-board")) != null
  and (.final_runtime.multi_room_history | index("interact_npc:npc-starter-artisan")) != null
  and (.final_runtime.multi_room_history | index("interact_npc:npc-market-clerk")) != null
  and (.final_runtime.active_task_ids | index("task-starter-forge-intro")) != null
  and (.final_runtime.active_task_ids | index("task-market-client-intro")) != null
  and .final_runtime.current_room_id == "client-board"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_MULTI_ROOM_PLAYABILITY_GREEN %s\n' "$SUMMARY"
