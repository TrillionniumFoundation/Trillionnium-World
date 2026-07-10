#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-persistence.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-persistence >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_persistence_v1"
  and .build_branch_world_reactions_contract == "trillionnium_world_bevy_build_branch_world_reactions_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .all_branch_events_accepted == true
  and .save_load_continue_gate == true
  and .resume_overlay_gate == true
  and .force_persistence_gate == true
  and .agility_persistence_gate == true
  and .craft_persistence_gate == true
  and .slot_snapshot_branch_state_gate == true
  and .restored_ui_gate == true
  and .branch_persistence_isolation_gate == true
  and (.slot_bytes.force > 0)
  and (.slot_bytes.agility > 0)
  and (.slot_bytes.craft > 0)
  and .slot_snapshots.force.contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and (.slot_snapshots.force.completed_task_ids | index("task-force-combat-commission") != null)
  and (.slot_snapshots.force.inventory_items | index("force-commission-token") != null)
  and .slot_snapshots.force.map_scene == "force_commission_arena_cleared"
  and .slot_snapshots.force.npc_behavior_state == "branch_force_commission_cleared"
  and .slot_snapshots.force.enemy_behavior_state == "force_commission_duelist_defeated"
  and (.slot_snapshots.force.visible_behavior_history | index("branch_world_force_duelist_defeated") != null)
  and (.quest_texts_after_continue.force | contains("BUILD REACTION | force:arena_duelist_defeated"))
  and (.visible_texts_after_continue.force_npc | contains("Force commission cleared; duelist yields the lane"))
  and (.visible_texts_after_continue.force_enemy | contains("force route clear: duelist yields"))
  and .slot_snapshots.agility.map_scene == "delivery_dock_scout_route_cleared"
  and (.slot_snapshots.agility.completed_task_ids | index("task-agility-scout-route") != null)
  and (.slot_snapshots.agility.inventory_items | index("agility-scout-token") != null)
  and (.slot_snapshots.agility.visible_behavior_history | index("branch_world_agility_courier_updated") != null)
  and (.quest_texts_after_continue.agility | contains("BUILD REACTION | agility:dock_route_marked"))
  and (.visible_texts_after_continue.agility_npc | contains("Scout markers pinned from dock to arena bridge"))
  and (.visible_texts_after_continue.agility_enemy | contains("no combat: scout route mapped"))
  and .slot_snapshots.craft.map_scene == "forge_delivery_order_cleared"
  and (.slot_snapshots.craft.completed_task_ids | index("task-craft-delivery-order") != null)
  and (.slot_snapshots.craft.inventory_items | index("craft-order-token") != null)
  and (.slot_snapshots.craft.visible_behavior_history | index("branch_world_craft_order_stamped") != null)
  and (.quest_texts_after_continue.craft | contains("BUILD REACTION | craft:forge_order_stamped"))
  and (.visible_texts_after_continue.craft_npc | contains("Delivery order stamped; forge queue updated"))
  and (.visible_texts_after_continue.craft_enemy | contains("no combat: order delivered"))
  and (.resume_texts.force_after_load | contains("RESUME ACTIVE"))
  and (.resume_texts.force_after_continue | contains("RESUME READY"))
  and (.resume_texts.agility_after_load | contains("RESUME ACTIVE"))
  and (.resume_texts.agility_after_continue | contains("RESUME READY"))
  and (.resume_texts.craft_after_load | contains("RESUME ACTIVE"))
  and (.resume_texts.craft_after_continue | contains("RESUME READY"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_PERSISTENCE_GREEN %s\n' "$SUMMARY"
