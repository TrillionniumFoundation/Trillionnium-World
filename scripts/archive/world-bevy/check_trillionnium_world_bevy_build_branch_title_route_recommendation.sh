#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-recommendation.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-recommendation >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_recommendation_v1"
  and .build_branch_title_loadout_switch_contract == "trillionnium_world_bevy_build_branch_title_loadout_switch_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .initial_unlocked_gate == true
  and .force_route_recommendation_gate == true
  and .agility_route_recommendation_gate == true
  and .craft_route_recommendation_gate == true
  and .route_director_switch_gate == true
  and .save_load_route_gate == true
  and .restore_route_ui_gate == true
  and .button_events.switch_force.accepted == true
  and .button_events.switch_force.availability_before == "enabled_build_title_equip:force:title-force-gate-warden"
  and .button_events.switch_agility.accepted == true
  and .button_events.switch_agility.availability_before == "enabled_build_title_equip:agility:title-agility-relay-runner"
  and .button_events.switch_craft.accepted == true
  and .button_events.switch_craft.availability_before == "enabled_build_title_equip:craft:title-craft-forge-master"
  and .button_events.save_selected.availability_before == "enabled_save_selected_slot:A"
  and .button_events.load_selected.availability_before == "enabled_session_slot_found:A"
  and .button_events.continue_after_load.availability_before == "enabled_session_resume_continue"
  and .route_director_states.after_force.active_build_title_id == "title-force-gate-warden"
  and .route_director_states.after_force.route_director_task_id == "task-force-guard-duty"
  and .route_director_states.after_force.route_director_target_room_id == "league-coliseum"
  and .route_director_states.after_force.route_director_next_room_id == "league-coliseum"
  and (.route_director_states.after_force.quest_panel_text | contains("TITLE ROUTE | active Gate Warden recommends force:task-force-guard-duty via arena_gate_reputation_anchor"))
  and (.route_director_states.after_force.route_director_history | index("active_title_route_recommendation:force:task-force-guard-duty:arena_gate_reputation_anchor") != null)
  and .route_director_states.after_agility.active_build_title_id == "title-agility-relay-runner"
  and .route_director_states.after_agility.route_director_task_id == "task-agility-courier-relay"
  and .route_director_states.after_agility.route_director_target_room_id == "delivery-dock"
  and .route_director_states.after_agility.route_director_next_room_id == "league-coliseum"
  and (.route_director_states.after_agility.quest_panel_text | contains("TITLE ROUTE | active Relay Runner recommends agility:task-agility-courier-relay via relay_route_priority_anchor"))
  and (.route_director_states.after_agility.route_director_history | index("active_title_route_recommendation:agility:task-agility-courier-relay:relay_route_priority_anchor") != null)
  and .route_director_states.after_craft.active_build_title_id == "title-craft-forge-master"
  and .route_director_states.after_craft.route_director_task_id == "task-craft-forge-batch"
  and .route_director_states.after_craft.route_director_target_room_id == "forge-workbench"
  and .route_director_states.after_craft.route_director_next_room_id == "starter-studio"
  and (.route_director_states.after_craft.quest_panel_text | contains("TITLE ROUTE | active Forge Master recommends craft:task-craft-forge-batch via forge_client_trust_anchor"))
  and (.route_director_states.after_craft.route_director_history | index("active_title_route_recommendation:craft:task-craft-forge-batch:forge_client_trust_anchor") != null)
  and .slot_snapshot_after_route_save.present == true
  and .slot_snapshot_after_route_save.contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .slot_snapshot_after_route_save.active_build_title_id == "title-craft-forge-master"
  and .slot_snapshot_after_route_save.route_director_task_id == "task-craft-forge-batch"
  and .slot_snapshot_after_route_save.route_director_target_room_id == "forge-workbench"
  and .final_runtime.active_build_title_id == "title-craft-forge-master"
  and .final_runtime.route_director_task_id == "task-craft-forge-batch"
  and .final_runtime.route_director_target_room_id == "forge-workbench"
  and .final_runtime.route_director_next_room_id == "starter-studio"
  and (.route_director_states.after_continue_craft.quest_panel_text | contains("TITLE ROUTE | active Forge Master recommends craft:task-craft-forge-batch via forge_client_trust_anchor"))
  and (.route_director_states.after_continue_craft.session_slot_text | contains("TITLE Forge Master"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_RECOMMENDATION_GREEN %s\n' "$SUMMARY"
