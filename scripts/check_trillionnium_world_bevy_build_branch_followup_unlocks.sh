#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-followup-unlocks.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-followup-unlocks >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_followup_unlocks_v1"
  and .build_branch_persistence_contract == "trillionnium_world_bevy_build_branch_persistence_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .all_branch_events_accepted == true
  and .restore_then_start_gate == true
  and .followup_available_after_restore_gate == true
  and .slot_snapshot_followup_gate == true
  and .force_followup_start_gate == true
  and .agility_followup_start_gate == true
  and .craft_followup_start_gate == true
  and .branch_followup_isolation_gate == true
  and .followup_task_ids.force == "task-force-guard-duty"
  and .followup_task_ids.agility == "task-agility-courier-relay"
  and .followup_task_ids.craft == "task-craft-forge-batch"
  and (.slot_bytes.force > 0)
  and (.slot_bytes.agility > 0)
  and (.slot_bytes.craft > 0)
  and (.slot_snapshots.force.completed_task_ids | index("task-force-combat-commission") != null)
  and (.slot_snapshots.force.unlocked_task_ids | index("task-force-guard-duty") != null)
  and (.slot_snapshots.force.active_task_ids | index("task-force-guard-duty") == null)
  and .slot_snapshots.force.route_director_task_id == "task-force-guard-duty"
  and (.quest_texts_after_continue.force | contains("force:task-force-guard-duty:unlocked_after_restore"))
  and .runtime_summaries_after_followup_start.force.objective_status == "build_branch_followup_active:force:task-force-guard-duty"
  and .runtime_summaries_after_followup_start.force.map_scene == "force_guard_post_open"
  and (.runtime_summaries_after_followup_start.force.active_task_ids | index("task-force-guard-duty") != null)
  and (.visible_texts_after_followup_start.force_npc | contains("Guard duty unlocked; hold the arena gate"))
  and (.visible_texts_after_followup_start.force_enemy | contains("follow-up force: rival stands down"))
  and (.slot_snapshots.agility.completed_task_ids | index("task-agility-scout-route") != null)
  and (.slot_snapshots.agility.unlocked_task_ids | index("task-agility-courier-relay") != null)
  and .slot_snapshots.agility.route_director_task_id == "task-agility-courier-relay"
  and (.quest_texts_after_continue.agility | contains("agility:task-agility-courier-relay:unlocked_after_restore"))
  and .runtime_summaries_after_followup_start.agility.objective_status == "build_branch_followup_active:agility:task-agility-courier-relay"
  and .runtime_summaries_after_followup_start.agility.map_scene == "agility_relay_lane_open"
  and (.runtime_summaries_after_followup_start.agility.active_task_ids | index("task-agility-courier-relay") != null)
  and (.visible_texts_after_followup_start.agility_npc | contains("Courier relay unlocked; run the dock markers"))
  and (.visible_texts_after_followup_start.agility_enemy | contains("follow-up agility: no combat, route bypassed"))
  and (.slot_snapshots.craft.completed_task_ids | index("task-craft-delivery-order") != null)
  and (.slot_snapshots.craft.unlocked_task_ids | index("task-craft-forge-batch") != null)
  and .slot_snapshots.craft.route_director_task_id == "task-craft-forge-batch"
  and (.quest_texts_after_continue.craft | contains("craft:task-craft-forge-batch:unlocked_after_restore"))
  and .runtime_summaries_after_followup_start.craft.objective_status == "build_branch_followup_active:craft:task-craft-forge-batch"
  and .runtime_summaries_after_followup_start.craft.map_scene == "craft_batch_forge_open"
  and (.runtime_summaries_after_followup_start.craft.active_task_ids | index("task-craft-forge-batch") != null)
  and (.visible_texts_after_followup_start.craft_npc | contains("Forge batch unlocked; stamp three client orders"))
  and (.visible_texts_after_followup_start.craft_enemy | contains("follow-up craft: workshop safe"))
  and (.runtime_summaries_after_followup_start.force.unlocked_task_ids | index("task-agility-courier-relay") == null)
  and (.runtime_summaries_after_followup_start.agility.unlocked_task_ids | index("task-craft-forge-batch") == null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_FOLLOWUP_UNLOCKS_GREEN %s\n' "$SUMMARY"
