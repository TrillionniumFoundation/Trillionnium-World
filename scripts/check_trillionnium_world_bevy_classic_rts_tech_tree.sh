#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-tech-tree "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_tech_tree_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_tech_tree_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
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
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TECH_TREE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
