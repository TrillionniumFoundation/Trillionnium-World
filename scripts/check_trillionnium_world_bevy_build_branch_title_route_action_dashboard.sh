#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-action-dashboard.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-action-dashboard >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_action_dashboard_v1"
  and .build_branch_title_route_progress_summary_contract == "trillionnium_world_bevy_build_branch_title_route_progress_summary_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .progress_summary_contract_green == true
  and .equip_dashboard_gate == true
  and .route_dashboard_gate == true
  and .complete_dashboard_gate == true
  and .force_prereq_dashboard_gate == true
  and .done_dashboard_gate == true
  and .save_load_dashboard_gate == true
  and .expected_rows.equip_force == "force:followup_ready reward force-mastery-signet action TITLE:EQUIP:title-force-gate-warden enabled true reason enabled_build_title_equip:force:title-force-gate-warden"
  and .expected_rows.route_craft == "craft:followup_ready reward craft-mastery-signet action TITLE:ROUTE enabled true reason enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  and .expected_rows.complete_craft == "craft:followup_active reward craft-mastery-signet action COMPLETE enabled true reason enabled_build_branch_followup_completion:task-craft-forge-batch"
  and .expected_rows.force_prereq == "force:mastery_active reward force-mastery-signet action COMPLETE enabled false reason build_mastery_force_victory_required:task-force-mastery-guard-trial"
  and .expected_rows.done_force == "force:complete reward force-mastery-signet action DONE enabled false reason title_route_complete:force"
  and .expected_rows.done_agility == "agility:complete reward agility-mastery-signet action DONE enabled false reason title_route_complete:agility"
  and .expected_rows.done_craft == "craft:complete reward craft-mastery-signet action DONE enabled false reason title_route_complete:craft"
  and (.dashboard_samples.equip.character_detail_text | contains("TITLE ROUTE DASHBOARD | force:followup_ready reward force-mastery-signet action TITLE:EQUIP:title-force-gate-warden enabled true reason enabled_build_title_equip:force:title-force-gate-warden"))
  and (.dashboard_samples.equip.character_detail_text | contains("agility:followup_ready reward agility-mastery-signet action TITLE:EQUIP:title-agility-relay-runner enabled true reason enabled_build_title_equip:agility:title-agility-relay-runner"))
  and (.dashboard_samples.route.character_detail_text | contains("craft:followup_ready reward craft-mastery-signet action TITLE:ROUTE enabled true reason enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"))
  and (.dashboard_samples.complete.character_detail_text | contains("craft:followup_active reward craft-mastery-signet action COMPLETE enabled true reason enabled_build_branch_followup_completion:task-craft-forge-batch"))
  and (.dashboard_samples.force_prereq.character_detail_text | contains("force:mastery_active reward force-mastery-signet action COMPLETE enabled false reason build_mastery_force_victory_required:task-force-mastery-guard-trial"))
  and (.dashboard_samples.done.character_detail_text | contains("force:complete reward force-mastery-signet action DONE enabled false reason title_route_complete:force"))
  and (.dashboard_samples.done.character_detail_text | contains("agility:complete reward agility-mastery-signet action DONE enabled false reason title_route_complete:agility"))
  and (.dashboard_samples.done.character_detail_text | contains("craft:complete reward craft-mastery-signet action DONE enabled false reason title_route_complete:craft"))
  and .button_events.save_selected.availability_before == "enabled_save_selected_slot:A"
  and .button_events.load_selected.availability_before == "enabled_session_slot_found:A"
  and .button_events.continue_after_load.availability_before == "enabled_session_resume_continue"
  and .slot_snapshot_after_dashboard_save.present == true
  and (.slot_snapshot_after_dashboard_save.completed_task_ids | index("task-force-mastery-guard-trial") != null)
  and (.slot_snapshot_after_dashboard_save.completed_task_ids | index("task-agility-mastery-shortcut-run") != null)
  and (.slot_snapshot_after_dashboard_save.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.dashboard_samples.after_continue.character_detail_text | contains("TITLE ROUTE DASHBOARD | force:complete reward force-mastery-signet action DONE enabled false reason title_route_complete:force"))
  and (.dashboard_samples.after_continue.character_detail_text | contains("agility:complete reward agility-mastery-signet action DONE enabled false reason title_route_complete:agility"))
  and (.dashboard_samples.after_continue.character_detail_text | contains("craft:complete reward craft-mastery-signet action DONE enabled false reason title_route_complete:craft"))
  and .final_runtime.session_resume_input_locked == false
  and .final_runtime.session_continue_cta_visible == false
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ACTION_DASHBOARD_GREEN %s\n' "$SUMMARY"
