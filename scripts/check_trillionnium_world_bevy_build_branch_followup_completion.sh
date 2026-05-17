#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-followup-completion.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-followup-completion >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_followup_completion_v1"
  and .build_branch_followup_unlocks_contract == "trillionnium_world_bevy_build_branch_followup_unlocks_v1"
  and .build_branch_persistence_contract == "trillionnium_world_bevy_build_branch_persistence_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .all_branch_events_accepted == true
  and .restore_start_complete_gate == true
  and .completion_available_gate == true
  and .followup_completion_persisted_gate == true
  and .force_followup_completion_gate == true
  and .agility_followup_completion_gate == true
  and .craft_followup_completion_gate == true
  and .branch_followup_completion_isolation_gate == true
  and .followup_task_ids.force == "task-force-guard-duty"
  and .followup_task_ids.agility == "task-agility-courier-relay"
  and .followup_task_ids.craft == "task-craft-forge-batch"
  and .reward_items.force == "force-guard-badge"
  and .reward_items.agility == "agility-relay-charm"
  and .reward_items.craft == "craft-batch-seal"
  and (.slot_bytes.force > 0)
  and (.slot_bytes.agility > 0)
  and (.slot_bytes.craft > 0)
  and (.slot_snapshots_after_completion.force.completed_task_ids | index("task-force-guard-duty") != null)
  and (.slot_snapshots_after_completion.force.active_task_ids | index("task-force-guard-duty") == null)
  and (.slot_snapshots_after_completion.force.inventory_items | index("force-guard-badge") != null)
  and .slot_snapshots_after_completion.force.objective_status == "build_branch_followup_completed:force:task-force-guard-duty"
  and .slot_snapshots_after_completion.force.map_scene == "force_guard_post_secured"
  and (.quest_texts_after_followup_completion.force | contains("force:task-force-guard-duty:completed"))
  and (.quest_texts_after_followup_completion.force | contains("BRANCH FOLLOWUP REACTION | force:guard_post_secured"))
  and .runtime_summaries_after_completion.force.npc_behavior_state == "force_guard_duty_report_complete"
  and .runtime_summaries_after_completion.force.enemy_behavior_state == "force_gate_rival_patrols_away"
  and (.visible_texts_after_followup_completion.force_npc | contains("Guard duty secured; arena gate is stable"))
  and (.visible_texts_after_followup_completion.force_enemy | contains("follow-up force complete: rival patrols away"))
  and (.slot_snapshots_after_completion.agility.completed_task_ids | index("task-agility-courier-relay") != null)
  and (.slot_snapshots_after_completion.agility.active_task_ids | index("task-agility-courier-relay") == null)
  and (.slot_snapshots_after_completion.agility.inventory_items | index("agility-relay-charm") != null)
  and .slot_snapshots_after_completion.agility.objective_status == "build_branch_followup_completed:agility:task-agility-courier-relay"
  and .slot_snapshots_after_completion.agility.map_scene == "agility_relay_lane_timed"
  and (.quest_texts_after_followup_completion.agility | contains("agility:task-agility-courier-relay:completed"))
  and (.quest_texts_after_followup_completion.agility | contains("BRANCH FOLLOWUP REACTION | agility:relay_lane_timed"))
  and .runtime_summaries_after_completion.agility.npc_behavior_state == "courier_relay_time_posted"
  and .runtime_summaries_after_completion.agility.enemy_behavior_state == "enemy_routes_fully_bypassed"
  and (.visible_texts_after_followup_completion.agility_npc | contains("Relay time posted; dock route is fast"))
  and (.visible_texts_after_followup_completion.agility_enemy | contains("follow-up agility complete: bypass confirmed"))
  and (.slot_snapshots_after_completion.craft.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.slot_snapshots_after_completion.craft.active_task_ids | index("task-craft-forge-batch") == null)
  and (.slot_snapshots_after_completion.craft.inventory_items | index("craft-batch-seal") != null)
  and .slot_snapshots_after_completion.craft.objective_status == "build_branch_followup_completed:craft:task-craft-forge-batch"
  and .slot_snapshots_after_completion.craft.map_scene == "craft_batch_forge_stamped"
  and (.quest_texts_after_followup_completion.craft | contains("craft:task-craft-forge-batch:completed"))
  and (.quest_texts_after_followup_completion.craft | contains("BRANCH FOLLOWUP REACTION | craft:forge_batch_stamped"))
  and .runtime_summaries_after_completion.craft.npc_behavior_state == "forge_batch_orders_stamped"
  and .runtime_summaries_after_completion.craft.enemy_behavior_state == "enemy_absent_after_forge_batch"
  and (.visible_texts_after_followup_completion.craft_npc | contains("Forge batch stamped; client orders ready"))
  and (.visible_texts_after_followup_completion.craft_enemy | contains("follow-up craft complete: workshop secured"))
  and (.runtime_summaries_after_completion.force.completed_task_ids | index("task-agility-courier-relay") == null)
  and (.runtime_summaries_after_completion.agility.completed_task_ids | index("task-craft-forge-batch") == null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_FOLLOWUP_COMPLETION_GREEN %s\n' "$SUMMARY"
