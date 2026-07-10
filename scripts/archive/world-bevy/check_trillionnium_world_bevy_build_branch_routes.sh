#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-routes.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-routes >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_routes_v1"
  and .equipment_affix_build_contract == "trillionnium_world_bevy_equipment_affix_build_v1"
  and .stat_confirmation_contract == "trillionnium_world_bevy_stat_confirmation_v1"
  and .scene_pack_contract == "trillionnium_world_bevy_scene_pack_v1"
  and .green == true
  and .branch_events_accepted == true
  and .dormant_locked_gate == true
  and .exclusive_unlock_gate == true
  and .force_branch_gate == true
  and .agility_branch_gate == true
  and .craft_branch_gate == true
  and .wrong_branch_guard_gate == true
  and .ui_task_state_gate == true
  and .availability_guards.dormant_force.enabled == false
  and .availability_guards.dormant_force.reason == "build_branch_requires_force:task-force-combat-commission"
  and .availability_guards.force_wrong_craft.enabled == false
  and .availability_guards.force_wrong_craft.reason == "build_branch_requires_craft:task-craft-delivery-order"
  and .runtime_summaries.force.route_director_task_id == "task-force-combat-commission"
  and .runtime_summaries.force.route_director_target_room_id == "league-coliseum"
  and .runtime_summaries.force.objective_status == "build_branch_active:force:task-force-combat-commission"
  and .runtime_summaries.force.combat_scene_state == "force_commission_duelist_ready"
  and (.runtime_summaries.force.active_task_ids | index("task-force-combat-commission") != null)
  and .runtime_summaries.agility.route_director_task_id == "task-agility-scout-route"
  and .runtime_summaries.agility.route_director_target_room_id == "delivery-dock"
  and .runtime_summaries.agility.objective_status == "build_branch_active:agility:task-agility-scout-route"
  and (.runtime_summaries.agility.active_task_ids | index("task-agility-scout-route") != null)
  and .runtime_summaries.craft.route_director_task_id == "task-craft-delivery-order"
  and .runtime_summaries.craft.route_director_target_room_id == "forge-workbench"
  and .runtime_summaries.craft.objective_status == "build_branch_active:craft:task-craft-delivery-order"
  and (.runtime_summaries.craft.active_task_ids | index("task-craft-delivery-order") != null)
  and (.quest_texts.force_after_task | contains("force:task-force-combat-commission:active"))
  and (.quest_texts.agility_after_task | contains("agility:task-agility-scout-route:active"))
  and (.quest_texts.craft_after_task | contains("craft:task-craft-delivery-order:active"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_ROUTES_GREEN %s\n' "$SUMMARY"
