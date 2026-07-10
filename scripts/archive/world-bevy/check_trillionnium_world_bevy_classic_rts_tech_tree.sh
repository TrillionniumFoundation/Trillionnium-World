#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.ppm"
RAW_SUMMARY="$SUMMARY.raw.$$"
TMP_SUMMARY="$SUMMARY.tmp.$$"
mkdir -p "$(dirname "$SUMMARY")"
cleanup() {
  rm -f "$RAW_SUMMARY" "$TMP_SUMMARY"
}
trap cleanup EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-tech-tree "$PREVIEW" >"$RAW_SUMMARY"

jq '
  .action_label_count = ((.action_labels // []) | length)
  | .input_source_count = ((.input_sources // []) | length)
  | .stage_summary_count = ((.stage_summaries // []) | length)
  | .final_base_structure_count = ((.final_base_structure_ids // []) | length)
  | .final_build_queue_count = ((.final_build_queue // []) | length)
  | .final_production_queue_count = ((.final_production_queue // []) | length)
  | .final_tech_research_count = ((.final_tech_research_ids // []) | length)
  | .final_completed_upgrade_count = ((.final_completed_upgrade_ids // []) | length)
  | .final_unlocked_unit_count = ((.final_unlocked_unit_ids // []) | length)
  | .final_unlocked_structure_count = ((.final_unlocked_structure_ids // []) | length)
  | .final_tech_requirement_count = ((.final_tech_requirements_log // []) | length)
  | .final_command_queue_count = ((.final_command_queue // []) | length)
  | .rts_tech_tree_core_frame_order_count = ((.rts_tech_tree_core_frame_orders // []) | length)
  | .rts_tech_tree_core_frame_order_kind_label_count = ((.rts_tech_tree_core_frame_order_kind_labels // []) | length)
  | .rts_tech_tree_core_frame_order_error_count = ((.rts_tech_tree_core_frame_order_errors // []) | length)
  | .rts_tech_tree_core_researched_rule_count = ((.rts_tech_tree_core_researched_rule_ids // []) | length)
  | .rts_tech_tree_core_upgraded_rule_count = ((.rts_tech_tree_core_upgraded_rule_ids // []) | length)
  | .rts_tech_tree_core_unlocked_rule_count = ((.rts_tech_tree_core_unlocked_rule_ids // []) | length)
  | .rts_tech_tree_core_source_actor_count = ((.rts_tech_tree_core_source_actor_ids // []) | length)
  | .tech_tree_gate_count = ([.write_gate, .live_tech_tree_input_gate, .faction_base_gate, .research_gate, .upgrade_gate, .unlock_gate, .dependency_gate, .rts_tech_tree_core_frame_order_gate, .rts_tech_tree_core_headless_replay_gate] | length)
  | .tech_tree_passed_gate_count = ([.write_gate, .live_tech_tree_input_gate, .faction_base_gate, .research_gate, .upgrade_gate, .unlock_gate, .dependency_gate, .rts_tech_tree_core_frame_order_gate, .rts_tech_tree_core_headless_replay_gate] | map(select(. == true)) | length)
  | .tech_tree_failed_gate_count = ([.write_gate, .live_tech_tree_input_gate, .faction_base_gate, .research_gate, .upgrade_gate, .unlock_gate, .dependency_gate, .rts_tech_tree_core_frame_order_gate, .rts_tech_tree_core_headless_replay_gate] | map(select(. != true)) | length)
' "$RAW_SUMMARY" >"$TMP_SUMMARY"
mv "$TMP_SUMMARY" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_tech_tree_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_tech_tree_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and .action_label_count == (.action_labels | length)
  and .input_source_count == (.input_sources | length)
  and .stage_summary_count == (.stage_summaries | length)
  and .final_base_structure_count == (.final_base_structure_ids | length)
  and .final_build_queue_count == (.final_build_queue | length)
  and .final_production_queue_count == (.final_production_queue | length)
  and .final_tech_research_count == (.final_tech_research_ids | length)
  and .final_completed_upgrade_count == (.final_completed_upgrade_ids | length)
  and .final_unlocked_unit_count == (.final_unlocked_unit_ids | length)
  and .final_unlocked_structure_count == (.final_unlocked_structure_ids | length)
  and .final_tech_requirement_count == (.final_tech_requirements_log | length)
  and .final_command_queue_count == (.final_command_queue | length)
  and .rts_tech_tree_core_frame_order_count == (.rts_tech_tree_core_frame_orders | length)
  and .rts_tech_tree_core_frame_order_kind_label_count == (.rts_tech_tree_core_frame_order_kind_labels | length)
  and .rts_tech_tree_core_frame_order_error_count == (.rts_tech_tree_core_frame_order_errors | length)
  and .rts_tech_tree_core_researched_rule_count == (.rts_tech_tree_core_researched_rule_ids | length)
  and .rts_tech_tree_core_upgraded_rule_count == (.rts_tech_tree_core_upgraded_rule_ids | length)
  and .rts_tech_tree_core_unlocked_rule_count == (.rts_tech_tree_core_unlocked_rule_ids | length)
  and .rts_tech_tree_core_source_actor_count == (.rts_tech_tree_core_source_actor_ids | length)
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:faction:mirror_guard") != null)
  and (.action_labels | index("RTS:QUEUE:build:training_hall@4,3") != null)
  and (.action_labels | index("RTS:QUEUE:research:wayfinder_code@town_hall") != null)
  and (.action_labels | index("RTS:QUEUE:upgrade:iron_lacing@training_hall") != null)
  and (.action_labels | index("RTS:QUEUE:unlock:relay_guard") != null)
  and .final_faction_id == "mirror_guard"
  and (.final_base_structure_ids | index("town_hall") != null)
  and (.final_base_structure_ids | index("training_hall") != null)
  and (.final_base_structure_ids | index("signal_spire") != null)
  and (.final_tech_research_ids | index("wayfinder_code") != null)
  and (.final_completed_upgrade_ids | index("iron_lacing") != null)
  and (.final_unlocked_unit_ids | index("relay_guard") != null)
  and (.final_unlocked_structure_ids | index("signal_spire") != null)
  and (.final_tech_requirements_log | index("base:town_hall|required:training_hall|locked:relay_guard") != null)
  and (.final_tech_requirements_log | index("upgrade:iron_lacing:requires:training_hall+wayfinder_code") != null)
  and (.final_tech_requirements_log | index("unlock:relay_guard:requires:iron_lacing+signal_spire") != null)
  and .final_tech_progress_percent == 100
  and .final_tech_state == "unlocked:relay_guard"
  and (.final_command_queue | index("faction:mirror_guard:base_online") != null)
  and (.final_command_queue | index("research:wayfinder_code@town_hall") != null)
  and (.final_command_queue | index("upgrade:iron_lacing@training_hall") != null)
  and (.final_command_queue | index("unlock:relay_guard") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_tech_tree_core_frame_order_gate == true
  and .rts_tech_tree_core_headless_replay_gate == true
  and (.rts_tech_tree_core_frame_orders | length == 5)
  and (.rts_tech_tree_core_frame_order_kind_labels | tostring == "[\"queue\",\"build\",\"research\",\"upgrade\",\"unlock\"]")
  and (.rts_tech_tree_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_tech_tree_core_frame_order_errors == []
  and .rts_tech_tree_core_frame_order_stream_error == null
  and .rts_tech_tree_core_headless_replay_error == null
  and .rts_tech_tree_core_headless_applied_order_count == 5
  and .rts_tech_tree_core_headless_actor_count == 4
  and .rts_tech_tree_core_headless_final_frame == 644
  and (.rts_tech_tree_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_tech_tree_core_tech_order_count == 3
  and .rts_tech_tree_core_research_order_count == 1
  and .rts_tech_tree_core_upgrade_order_count == 1
  and .rts_tech_tree_core_unlock_order_count == 1
  and (.rts_tech_tree_core_researched_rule_ids | index("wayfinder_code") != null)
  and (.rts_tech_tree_core_upgraded_rule_ids | index("iron_lacing") != null)
  and (.rts_tech_tree_core_unlocked_rule_ids | index("relay_guard") != null)
  and (.rts_tech_tree_core_source_actor_ids | index("town_hall") != null)
  and (.rts_tech_tree_core_source_actor_ids | index("training_hall") != null)
  and (.rts_tech_tree_core_headless_replay_report.checkpoint.tech_tree.tech_order_count == 3)
  and (.rts_tech_tree_core_headless_replay_report.checkpoint.event_log | any(contains(":kind:research:")))
  and (.rts_tech_tree_core_headless_replay_report.checkpoint.event_log | any(contains(":kind:upgrade:")))
  and (.rts_tech_tree_core_headless_replay_report.checkpoint.event_log | any(contains(":kind:unlock:")))
  and .non_background_pixels > 330000
  and .tech_base_pixel_count > 140
  and .tech_research_pixel_count > 50
  and .tech_upgrade_pixel_count > 40
  and .tech_unlock_pixel_count > 70
  and .tech_requirement_pixel_count > 60
  and .live_tech_tree_input_gate == true
  and .faction_base_gate == true
  and .research_gate == true
  and .upgrade_gate == true
  and .unlock_gate == true
  and .dependency_gate == true
  and .tech_tree_gate_count == 9
  and .tech_tree_passed_gate_count == 9
  and .tech_tree_failed_gate_count == 0
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TECH_TREE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
