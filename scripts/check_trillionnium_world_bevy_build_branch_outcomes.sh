#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-outcomes.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-outcomes >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_outcomes_v1"
  and .build_branch_routes_contract == "trillionnium_world_bevy_build_branch_routes_v1"
  and .equipment_affix_build_contract == "trillionnium_world_bevy_equipment_affix_build_v1"
  and .green == true
  and .all_branch_events_accepted == true
  and .force_completion_guard_gate == true
  and .target_room_completion_guard_gate == true
  and .force_outcome_gate == true
  and .agility_outcome_gate == true
  and .craft_outcome_gate == true
  and .wrong_branch_reward_guard_gate == true
  and .ui_settlement_gate == true
  and .availability_guards.force_before_victory_complete.enabled == false
  and .availability_guards.force_before_victory_complete.reason == "build_branch_force_victory_required:task-force-combat-commission"
  and .availability_guards.force_after_victory_complete.enabled == true
  and .availability_guards.force_after_victory_complete.reason == "enabled_build_branch_outcome:task-force-combat-commission"
  and .availability_guards.agility_before_target_complete.reason == "build_branch_target_room_required:delivery-dock:task-agility-scout-route"
  and .availability_guards.craft_before_target_complete.reason == "build_branch_target_room_required:forge-workbench:task-craft-delivery-order"
  and .availability_guards.force_wrong_craft_after_completion.enabled == false
  and .availability_guards.force_wrong_craft_after_completion.reason == "build_branch_requires_craft:task-craft-delivery-order"
  and (.runtime_summaries.force.completed_task_ids | index("task-force-combat-commission") != null)
  and (.runtime_summaries.force.active_task_ids | index("task-force-combat-commission") == null)
  and (.runtime_summaries.force.inventory_items | index("force-commission-token") != null)
  and (.runtime_summaries.force.inventory_items | index("craft-order-token") == null)
  and (.runtime_summaries.force.reward_settlement_text | contains("FORCE OUTCOME | duel cleared"))
  and .runtime_summaries.agility.current_room_id == "delivery-dock"
  and (.runtime_summaries.agility.completed_task_ids | index("task-agility-scout-route") != null)
  and (.runtime_summaries.agility.inventory_items | index("agility-scout-token") != null)
  and (.runtime_summaries.agility.reward_settlement_text | contains("AGILITY OUTCOME | scout route mapped"))
  and .runtime_summaries.craft.current_room_id == "forge-workbench"
  and (.runtime_summaries.craft.completed_task_ids | index("task-craft-delivery-order") != null)
  and (.runtime_summaries.craft.inventory_items | index("craft-order-token") != null)
  and (.runtime_summaries.craft.reward_settlement_text | contains("CRAFT OUTCOME | order delivered"))
  and (.quest_texts.force_after_complete | contains("force:task-force-combat-commission:completed"))
  and (.quest_texts.agility_after_complete | contains("agility:task-agility-scout-route:completed"))
  and (.quest_texts.craft_after_complete | contains("craft:task-craft-delivery-order:completed"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_OUTCOMES_GREEN %s\n' "$SUMMARY"
