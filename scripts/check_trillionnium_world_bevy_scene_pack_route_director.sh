#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-scene-pack-route-director.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- scene-pack-route-director >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_scene_pack_route_director_v1"
  and .scene_pack_contract == "trillionnium_world_bevy_scene_pack_v1"
  and .green == true
  and .all_plans_valid == true
  and .generated_actions_match_scene_graph == true
  and .all_pressed_accepted == true
  and .runtime_route_director_green == true
  and .final_state_green == true
  and (.route_plans | length) == 3
  and (.route_plans[] | select(.task_id == "task-starter-forge-intro") | .path_room_ids) == ["mirror-city-square", "starter-studio", "forge-workbench"]
  and (.route_plans[] | select(.task_id == "task-market-client-intro") | .path_room_ids) == ["forge-workbench", "asset-yard", "zbj-market-gate", "client-board", "delivery-dock"]
  and (.route_plans[] | select(.task_id == "task-arena-rematch-route") | .path_room_ids) == ["delivery-dock", "league-coliseum"]
  and (.expected_route_button_actions == .actual_route_button_actions)
  and (.final_runtime.current_room_id == "league-coliseum")
  and (.final_runtime.route_director_task_id == "task-arena-rematch-route")
  and (.final_runtime.route_director_next_room_id == null)
  and (.final_runtime.equipped_items | index("bandit_sash")) != null
  and (.final_runtime.active_task_ids | index("task-arena-rematch-route")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_SCENE_PACK_ROUTE_DIRECTOR_GREEN %s\n' "$SUMMARY"
