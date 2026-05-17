#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-progress-summary.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-progress-summary >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_progress_summary_v1"
  and .build_branch_title_route_all_branch_completion_contract == "trillionnium_world_bevy_build_branch_title_route_all_branch_completion_v1"
  and .build_branch_title_loadout_panel_contract == "trillionnium_world_bevy_build_branch_title_loadout_panel_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .all_branch_contract_green == true
  and .loadout_panel_summary_gate == true
  and .summary_complete_gate == true
  and .summary_reward_next_gate == true
  and .save_load_summary_gate == true
  and .summary_rows.force == "force:complete reward force-mastery-signet next title_route_complete:force"
  and .summary_rows.agility == "agility:complete reward agility-mastery-signet next title_route_complete:agility"
  and .summary_rows.craft == "craft:complete reward craft-mastery-signet next title_route_complete:craft"
  and (.ui_samples.after_summary.character_detail_text | contains("TITLE LOADOUT | active Forge Master"))
  and (.ui_samples.after_summary.character_detail_text | contains("TITLE ROUTE SUMMARY | force:complete reward force-mastery-signet next title_route_complete:force"))
  and (.ui_samples.after_summary.character_detail_text | contains("agility:complete reward agility-mastery-signet next title_route_complete:agility"))
  and (.ui_samples.after_summary.character_detail_text | contains("craft:complete reward craft-mastery-signet next title_route_complete:craft"))
  and .button_events.save_selected.availability_before == "enabled_save_selected_slot:A"
  and .button_events.load_selected.availability_before == "enabled_session_slot_found:A"
  and .button_events.continue_after_load.availability_before == "enabled_session_resume_continue"
  and .slot_snapshot_after_summary_save.present == true
  and (.slot_snapshot_after_summary_save.completed_task_ids | index("task-force-mastery-guard-trial") != null)
  and (.slot_snapshot_after_summary_save.completed_task_ids | index("task-agility-mastery-shortcut-run") != null)
  and (.slot_snapshot_after_summary_save.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.slot_snapshot_after_summary_save.inventory_items | index("force-mastery-signet") != null)
  and (.slot_snapshot_after_summary_save.inventory_items | index("agility-mastery-signet") != null)
  and (.slot_snapshot_after_summary_save.inventory_items | index("craft-mastery-signet") != null)
  and (.ui_samples.after_continue.character_detail_text | contains("TITLE ROUTE SUMMARY | force:complete reward force-mastery-signet next title_route_complete:force"))
  and (.ui_samples.after_continue.character_detail_text | contains("agility:complete reward agility-mastery-signet next title_route_complete:agility"))
  and (.ui_samples.after_continue.character_detail_text | contains("craft:complete reward craft-mastery-signet next title_route_complete:craft"))
  and .final_runtime.session_resume_input_locked == false
  and .final_runtime.session_continue_cta_visible == false
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_PROGRESS_SUMMARY_GREEN %s\n' "$SUMMARY"
