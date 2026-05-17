#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-world-reactions.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-world-reactions >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_world_reactions_v1"
  and .build_branch_outcomes_contract == "trillionnium_world_bevy_build_branch_outcomes_v1"
  and .green == true
  and .all_branch_events_accepted == true
  and .force_world_reaction_gate == true
  and .agility_world_reaction_gate == true
  and .craft_world_reaction_gate == true
  and .reaction_ui_gate == true
  and .branch_reaction_isolation_gate == true
  and .runtime_summaries.force.map_scene == "force_commission_arena_cleared"
  and .runtime_summaries.force.npc_behavior_state == "branch_force_commission_cleared"
  and .runtime_summaries.force.enemy_behavior_state == "force_commission_duelist_defeated"
  and (.runtime_summaries.force.visible_behavior_history | index("branch_world_force_duelist_defeated") != null)
  and (.quest_texts.force | contains("BUILD REACTION | force:arena_duelist_defeated"))
  and (.visible_texts.force_npc | contains("Force commission cleared; duelist yields the lane"))
  and (.visible_texts.force_enemy | contains("force route clear: duelist yields"))
  and .runtime_summaries.agility.map_scene == "delivery_dock_scout_route_cleared"
  and .runtime_summaries.agility.current_room_id == "delivery-dock"
  and .runtime_summaries.agility.npc_behavior_state == "dock_courier_route_chart_updated"
  and (.runtime_summaries.agility.visible_behavior_history | index("branch_world_agility_courier_updated") != null)
  and (.quest_texts.agility | contains("BUILD REACTION | agility:dock_route_marked"))
  and (.visible_texts.agility_npc | contains("Scout markers pinned from dock to arena bridge"))
  and (.visible_texts.agility_enemy | contains("no combat: scout route mapped"))
  and .runtime_summaries.craft.map_scene == "forge_delivery_order_cleared"
  and .runtime_summaries.craft.current_room_id == "forge-workbench"
  and .runtime_summaries.craft.npc_behavior_state == "forge_workbench_order_stamped"
  and (.runtime_summaries.craft.visible_behavior_history | index("branch_world_craft_order_stamped") != null)
  and (.quest_texts.craft | contains("BUILD REACTION | craft:forge_order_stamped"))
  and (.visible_texts.craft_npc | contains("Delivery order stamped; forge queue updated"))
  and (.visible_texts.craft_enemy | contains("no combat: order delivered"))
  and (.quest_texts.force | contains("agility:dock_route_marked") | not)
  and (.quest_texts.force | contains("craft:forge_order_stamped") | not)
  and (.quest_texts.agility | contains("force:arena_duelist_defeated") | not)
  and (.quest_texts.craft | contains("force:arena_duelist_defeated") | not)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_WORLD_REACTIONS_GREEN %s\n' "$SUMMARY"
