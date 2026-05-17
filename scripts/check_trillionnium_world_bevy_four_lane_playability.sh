#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-four-lane-playability.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- four-lane-playability >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_four_lane_playability_v1"
  and .visible_actor_behavior_contract == "trillionnium_world_bevy_visible_actor_behavior_v1"
  and .expanded_input_semantics_contract == "trillionnium_world_bevy_expanded_input_semantics_v1"
  and .green == true
  and .running_state_sampling == true
  and (.bevy_window_button_path | contains("Interaction_Pressed"))
  and .four_lanes.live_interaction_gate == true
  and .four_lanes.map_quality_gate == true
  and .four_lanes.input_semantics_gate == true
  and .four_lanes.acceptance_artifacts_gate == true
  and .visible_actor_surfaces_green == true
  and (.button_events | length) >= 20
  and (.button_events[] | select(.action == "ROOM:delivery-dock") | .accepted) == true
  and (.button_events[] | select(.action == "ROOM:league-coliseum") | .accepted) == true
  and (.button_events[] | select(.action == "LOOT:drop") | .accepted) == true
  and (.button_events[] | select(.action == "BAG:open") | .accepted) == true
  and (.button_events[] | select(.action == "EQUIP:bandit_sash") | .accepted) == true
  and .final_runtime.current_room_id == "league-coliseum"
  and (.final_runtime.visited_rooms | index("client-board")) != null
  and (.final_runtime.visited_rooms | index("delivery-dock")) != null
  and (.final_runtime.visited_rooms | index("league-coliseum")) != null
  and .final_runtime.combat_result_state == "victory"
  and .final_runtime.enemy_hp == 0
  and .final_runtime.bag_open == true
  and (.final_runtime.equipped_items | index("bandit_sash")) != null
  and (.final_runtime.inventory_items | index("bandit_sash")) == null
  and (.final_runtime.loot_history | index("loot:bandit_sash")) != null
  and .final_runtime.growth_level >= 2
  and .final_runtime.growth_stat_points >= 1
  and (.final_runtime.visible_behavior_history | index("item_drop_visible")) != null
  and (.final_runtime.visible_behavior_history | index("equipment_icon_visible")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_FOUR_LANE_PLAYABILITY_GREEN %s\n' "$SUMMARY"
