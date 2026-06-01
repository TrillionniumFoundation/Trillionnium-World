#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-review-ci-gate.json"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_CI_GATE_SUMMARY && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CI_GATE_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CI_GATE_SUMMARY"
fi

CHECK_RESULTS="$(mktemp)"
trap 'rm -f "$CHECK_RESULTS"' EXIT

mkdir -p "$ACCEPTANCE_DIR"

add_check() {
  local name="$1"
  local status="$2"
  local log_path="$3"
  local detail="$4"
  jq -nc \
    --arg name "$name" \
    --arg status "$status" \
    --arg log_path "$log_path" \
    --arg detail "$detail" \
    '{name: $name, status: $status, log_path: $log_path, detail: $detail}' >>"$CHECK_RESULTS"
}

run_check() {
  local name="$1"
  shift
  local log_path="$ACCEPTANCE_DIR/release-review-ci-gate-${name}.log"
  if "$@" >"$log_path" 2>&1; then
    add_check "$name" ok "$log_path" "command_passed"
  else
    add_check "$name" fail "$log_path" "command_failed"
  fi
}

run_check bash_syntax bash -n \
  "$ROOT/scripts/check_trillionnium_world_release_review_quickcheck.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_status.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_convergence.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_semantic_fixture.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture.sh" \
	  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_selection_minimap_semantic_fixture.sh" \
	  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_build_lifecycle_semantic_fixture.sh" \
	  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_tech_tree_semantic_fixture.sh" \
	  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_projectile_ability_semantic_fixture.sh" \
	  "$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh" \
  "$ROOT/scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh" \
  "$ROOT/scripts/check_trillionnium_world_client_boundary.sh" \
  "$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_bundle.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_operator_handoff.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_blocker_consistency.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_status_only_fixtures.sh" \
  "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh" \
  "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh" \
  "$ROOT/scripts/check_trillionnium_world_external_ops_evidence_collection.sh" \
  "$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh" \
  "$ROOT/scripts/check_trillionnium_world_s5_device_evidence.sh" \
  "$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh" \
  "$ROOT/scripts/check_trillionnium_world_halium_sidecar_runtime_dev_surface.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_action_coach.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_player_ui_rescue.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_account_client_boundary.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_account_title_flow.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_preview.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_selector.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_player_motion_probe.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_render_budget.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_scene_preview.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_like_core.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_loop.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_minimap.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tech_tree.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_projectile_ability.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ai_skirmish_pressure.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_objective_victory_loop.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_creep_camp_terrain_route.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_fog_scouting_intel.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_army_production_rally.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_base_assault_resolution.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_battle_aftermath.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_commander_progression.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_expansion_counterattack.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tier_two_siege_push.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_siege_breach_counterplay.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_inner_lane_breakthrough.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_pressure.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_breakthrough.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_mirror_city_restoration.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_open_world_after_action.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_handoff.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_entry.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_visual_fidelity.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_affordance.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_surface.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_structure_modeling.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_environment_life.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_worker_harvest_animation.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_spawn_animation.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_status_portrait.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_command_feedback.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_recall_override_preview.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_history.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_history_prune.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_scrollable_map.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_queue_path_preview.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_preview.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_execution.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_local_obstruction_recovery.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_cadence.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_model_depth.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_sequence.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_behavior.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_combat_impact.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_locomotion_blend.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_transition.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_depth_readability.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_bridge.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_owned_replay_file.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_headless_replay_playback.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_natural_terminal_contract.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_native_bot_ai_planner.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_lane.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_importer.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_payload_decoder.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_replay_reducer.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_decision_state_gap.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_map_intel_gap.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_art_pack.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_sprite_sheet.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_texture_atlas_binding.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_material_consumption.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_material_application.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_asset.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_manifest_probe.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_asset_store_registration.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_sprite_asset_binding.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_render_frame.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_live_visual_bridge.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_layer_pixel_probe.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_texture_correlation.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_render_asset_eligibility.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh" \
  "$ROOT/scripts/check_trillionnium_world_bevy_desktop_playtest_review_packet.sh" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh" \
  "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh" \
  "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" \
  "$ROOT/scripts/check_trillionnium_world_map_modeling_gate.sh" \
  "$ROOT/scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh" \
  "$ROOT/scripts/v2/root_readme_world_release_review_quickcheck_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_status_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_convergence_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_integrity_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_integrity_drift_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_integrity_semantic_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_integrity_bot_executor_semantic_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_integrity_bot_executor_matrix_semantic_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_integrity_bot_gap_semantic_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_packet_integrity_control_loop_semantic_guard_test.sh" \
	  "$ROOT/scripts/v2/release_review_packet_integrity_selection_minimap_semantic_guard_test.sh" \
	  "$ROOT/scripts/v2/release_review_packet_integrity_build_lifecycle_semantic_guard_test.sh" \
	  "$ROOT/scripts/v2/release_review_packet_integrity_tech_tree_semantic_guard_test.sh" \
	  "$ROOT/scripts/v2/release_review_packet_integrity_projectile_ability_semantic_guard_test.sh" \
	  "$ROOT/scripts/v2/release_review_checkpoint_manifest_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/client_boundary_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_bundle_negative_fixtures_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_evidence_bundle_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_template_negative_fixtures_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_evidence_kit_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_operator_handoff_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_readiness_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_blocker_consistency_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/release_review_ci_gate_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/cex_adapter_readiness_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_status_only_fixture_guard_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/cohort_commercial_evidence_collection_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/cohort_commercial_evidence_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/external_ops_evidence_collection_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/external_ops_evidence_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/s5_device_evidence_collector_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/s5_real_device_evidence_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/halium_sidecar_runtime_dev_surface_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/halium_sidecar_runtime_dev_surface_operator_payload_fixture_test.sh" \
  "$ROOT/scripts/v2/halium_sidecar_runtime_dev_surface_unsafe_tar_fixture_test.sh" \
  "$ROOT/scripts/v2/player_ui_rescue_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_account_client_boundary_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_account_title_flow_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_isometric_modeling_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_input_frame_budget_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_playtest_runner_status_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_loop_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_selection_minimap_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_build_lifecycle_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_tech_tree_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_projectile_ability_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_ai_skirmish_pressure_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_objective_victory_loop_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_creep_camp_terrain_route_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_fog_scouting_intel_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_enemy_base_tech_pressure_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_army_production_rally_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_base_assault_resolution_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_battle_aftermath_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_commander_progression_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_expansion_counterattack_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_tier_two_siege_push_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_siege_breach_counterplay_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_inner_lane_breakthrough_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_central_keep_pressure_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_central_keep_breakthrough_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_mirror_city_restoration_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_open_world_after_action_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_campaign_handoff_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_campaign_entry_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_visual_fidelity_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_command_affordance_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_command_surface_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_structure_modeling_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_environment_life_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_worker_harvest_animation_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_production_spawn_animation_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_unit_status_portrait_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_selection_command_feedback_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_ability_tooltip_telegraph_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_group_hotkey_feedback_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_group_recall_formation_preview_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_group_recall_override_preview_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_feedback_strip_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_feedback_lifecycle_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_history_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_history_prune_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_first_minute_command_feedback_replay_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_first_minute_command_feedback_rejection_replay_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_scrollable_map_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_camera_minimap_sync_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_command_queue_path_preview_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_formation_move_preview_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_formation_move_execution_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_local_obstruction_recovery_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_action_cadence_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_unit_model_depth_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_action_sequence_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_npc_behavior_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_combat_impact_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_locomotion_blend_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_npc_transition_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_depth_readability_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_parity_bridge_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_owned_replay_file_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_headless_replay_playback_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_natural_terminal_contract_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_native_bot_ai_planner_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_planner_live_autonomous_bot_loop_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_parity_lane_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_replay_compat_adapter_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_command_vocab_adapter_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_order_serializer_fixture_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_replay_importer_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_order_payload_decoder_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_imported_replay_reducer_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_imported_replay_repro_manifest_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_order_replay_reducer_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_openra_headless_comparison_harness_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_bot_planner_action_executor_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_bot_planner_executor_replay_determinism_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_multi_match_bot_executor_evaluation_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_bot_executor_failure_recovery_matrix_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_bot_decision_state_gap_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_bot_adaptive_build_order_gap_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_bot_tactical_micro_gap_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_rts_bot_map_intel_gap_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_classic_playtest_launcher_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/authored_art_pack_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/authored_sprite_sheet_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/authored_texture_atlas_binding_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/authored_material_consumption_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/authored_material_application_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/runtime_texture_asset_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/runtime_texture_manifest_probe_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/asset_store_registration_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/sprite_asset_binding_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/sprite_texture_sampling_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/authored_render_frame_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/authored_live_visual_bridge_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/live_window_layer_pixel_probe_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/live_window_texture_correlation_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/live_window_sampled_texture_correlation_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/render_asset_eligibility_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/live_window_runtime_texture_manifest_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_desktop_real_machine_readiness_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/bevy_desktop_playtest_review_packet_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/public_launch_evidence_intake_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/release_readiness_release_review_entry_guard_test.sh" \
  "$ROOT/scripts/v2/production_map_pack_public_evidence_collection_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/production_map_pack_public_evidence_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/map_modeling_gate_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/ui_map_modeling_full_alignment_script_contract_guard_test.sh" \
  "$ROOT/scripts/v2/production_map_pack_public_evidence_artifact_guard_test.sh"

run_check status_contract_guard "$ROOT/scripts/v2/release_review_status_script_contract_guard_test.sh"
run_check convergence_contract_guard "$ROOT/scripts/v2/release_review_convergence_script_contract_guard_test.sh"
run_check packet_contract_guard "$ROOT/scripts/v2/release_review_packet_script_contract_guard_test.sh"
run_check packet_integrity_contract_guard "$ROOT/scripts/v2/release_review_packet_integrity_script_contract_guard_test.sh"
run_check packet_integrity_drift_guard "$ROOT/scripts/v2/release_review_packet_integrity_drift_guard_test.sh"
run_check packet_integrity_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_semantic_guard_test.sh"
run_check packet_integrity_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_semantic_fixture.sh"
run_check packet_integrity_bot_executor_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_bot_executor_semantic_guard_test.sh"
run_check packet_integrity_bot_executor_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture.sh"
run_check packet_integrity_bot_executor_matrix_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_bot_executor_matrix_semantic_guard_test.sh"
run_check packet_integrity_bot_executor_matrix_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture.sh"
run_check packet_integrity_bot_gap_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_bot_gap_semantic_guard_test.sh"
run_check packet_integrity_bot_gap_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture.sh"
run_check packet_integrity_control_loop_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_control_loop_semantic_guard_test.sh"
run_check packet_integrity_control_loop_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture.sh"
run_check packet_integrity_selection_minimap_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_selection_minimap_semantic_guard_test.sh"
run_check packet_integrity_selection_minimap_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_selection_minimap_semantic_fixture.sh"
run_check packet_integrity_build_lifecycle_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_build_lifecycle_semantic_guard_test.sh"
run_check packet_integrity_build_lifecycle_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_build_lifecycle_semantic_fixture.sh"
run_check packet_integrity_tech_tree_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_tech_tree_semantic_guard_test.sh"
run_check packet_integrity_tech_tree_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_tech_tree_semantic_fixture.sh"
run_check packet_integrity_projectile_ability_semantic_guard "$ROOT/scripts/v2/release_review_packet_integrity_projectile_ability_semantic_guard_test.sh"
run_check packet_integrity_projectile_ability_semantic_fixture_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_projectile_ability_semantic_fixture.sh"
run_check public_launch_bundle_negative_fixtures_contract_guard "$ROOT/scripts/v2/public_launch_bundle_negative_fixtures_script_contract_guard_test.sh"
run_check public_launch_bundle_negative_fixtures_gate "$ROOT/scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh"
run_check public_launch_evidence_bundle_contract_guard "$ROOT/scripts/v2/public_launch_evidence_bundle_script_contract_guard_test.sh"
run_check public_launch_evidence_bundle_gate "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_bundle.sh"
run_check public_launch_template_negative_fixtures_contract_guard "$ROOT/scripts/v2/public_launch_template_negative_fixtures_script_contract_guard_test.sh"
run_check public_launch_template_negative_fixtures_gate "$ROOT/scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh"
run_check public_launch_evidence_kit_contract_guard "$ROOT/scripts/v2/public_launch_evidence_kit_script_contract_guard_test.sh"
run_check public_launch_evidence_kit_gate "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh"
run_check public_launch_operator_handoff_contract_guard "$ROOT/scripts/v2/public_launch_operator_handoff_script_contract_guard_test.sh"
run_check public_launch_operator_handoff_gate "$ROOT/scripts/check_trillionnium_world_public_launch_operator_handoff.sh"
run_check public_launch_readiness_contract_guard "$ROOT/scripts/v2/public_launch_readiness_script_contract_guard_test.sh"
run_check public_launch_blocker_consistency_contract_guard "$ROOT/scripts/v2/public_launch_blocker_consistency_script_contract_guard_test.sh"
run_check public_launch_blocker_consistency_gate "$ROOT/scripts/check_trillionnium_world_public_launch_blocker_consistency.sh"
run_check ci_gate_contract_guard "$ROOT/scripts/v2/release_review_ci_gate_script_contract_guard_test.sh"
run_check client_boundary_contract_guard "$ROOT/scripts/v2/client_boundary_script_contract_guard_test.sh"
run_check client_boundary_gate "$ROOT/scripts/check_trillionnium_world_client_boundary.sh"
run_check cex_adapter_readiness_contract_guard "$ROOT/scripts/v2/cex_adapter_readiness_script_contract_guard_test.sh"
run_check cex_adapter_readiness_gate "$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh"
run_check checkpoint_manifest_contract_guard "$ROOT/scripts/v2/release_review_checkpoint_manifest_script_contract_guard_test.sh"
run_check checkpoint_manifest "$ROOT/scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh"
run_check public_launch_status_only_fixture_guard_contract "$ROOT/scripts/v2/public_launch_status_only_fixture_guard_script_contract_guard_test.sh"
run_check public_launch_status_only_fixture_guard "$ROOT/scripts/check_trillionnium_world_public_launch_status_only_fixtures.sh"
run_check cohort_commercial_evidence_collection_contract_guard "$ROOT/scripts/v2/cohort_commercial_evidence_collection_script_contract_guard_test.sh"
run_check cohort_commercial_evidence_collection "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh"
run_check cohort_commercial_evidence_contract_guard "$ROOT/scripts/v2/cohort_commercial_evidence_script_contract_guard_test.sh"
run_check cohort_commercial_evidence_gate "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh"
run_check external_ops_evidence_collection_contract_guard "$ROOT/scripts/v2/external_ops_evidence_collection_script_contract_guard_test.sh"
run_check external_ops_evidence_collection "$ROOT/scripts/check_trillionnium_world_external_ops_evidence_collection.sh"
run_check external_ops_evidence_contract_guard "$ROOT/scripts/v2/external_ops_evidence_script_contract_guard_test.sh"
run_check external_ops_evidence_gate "$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh"
run_check s5_device_evidence_collector_contract_guard "$ROOT/scripts/v2/s5_device_evidence_collector_script_contract_guard_test.sh"
run_check s5_real_device_evidence_contract_guard "$ROOT/scripts/v2/s5_real_device_evidence_script_contract_guard_test.sh"
run_check s5_real_device_evidence_gate "$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh"
run_check halium_sidecar_runtime_dev_surface_contract_guard "$ROOT/scripts/v2/halium_sidecar_runtime_dev_surface_script_contract_guard_test.sh"
run_check halium_sidecar_runtime_dev_surface_gate "$ROOT/scripts/check_trillionnium_world_halium_sidecar_runtime_dev_surface.sh"
run_check halium_sidecar_runtime_dev_surface_operator_payload_fixture "$ROOT/scripts/v2/halium_sidecar_runtime_dev_surface_operator_payload_fixture_test.sh"
run_check halium_sidecar_runtime_dev_surface_unsafe_tar_fixture "$ROOT/scripts/v2/halium_sidecar_runtime_dev_surface_unsafe_tar_fixture_test.sh"
run_check bevy_action_coach_gate "$ROOT/scripts/check_trillionnium_world_bevy_action_coach.sh"
run_check bevy_player_hud_debug_layer_gate "$ROOT/scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh"
run_check bevy_player_ui_rescue_contract_guard "$ROOT/scripts/v2/player_ui_rescue_script_contract_guard_test.sh"
run_check bevy_player_ui_rescue_gate "$ROOT/scripts/check_trillionnium_world_bevy_player_ui_rescue.sh"
run_check bevy_account_client_boundary_contract_guard "$ROOT/scripts/v2/bevy_account_client_boundary_script_contract_guard_test.sh"
run_check bevy_account_client_boundary_gate "$ROOT/scripts/check_trillionnium_world_bevy_account_client_boundary.sh"
run_check bevy_account_title_flow_contract_guard "$ROOT/scripts/v2/bevy_account_title_flow_script_contract_guard_test.sh"
run_check bevy_account_title_flow_gate "$ROOT/scripts/check_trillionnium_world_bevy_account_title_flow.sh"
# Bevy classic low-spec asset contracts: trillionnium_world_bevy_classic_asset_pack_v1 / trillionnium_world_bevy_classic_manifest_lint_v1 / trillionnium_world_bevy_classic_animation_preview_v1 / trillionnium_world_bevy_classic_animation_selector_v1 / trillionnium_world_bevy_classic_player_motion_probe_v1 / trillionnium_world_bevy_classic_input_frame_budget_v1 / trillionnium_world_bevy_classic_render_budget_v1 / trillionnium_world_bevy_classic_scene_preview_v1 / trillionnium_world_bevy_classic_model_catalog_v1 / trillionnium_world_bevy_classic_renderer_probe_v1 / trillionnium_world_bevy_classic_isometric_modeling_v1 / trillionnium_world_bevy_classic_rts_control_loop_v1 / trillionnium_world_bevy_classic_rts_selection_minimap_v1 / trillionnium_world_bevy_classic_rts_build_lifecycle_v1 / trillionnium_world_bevy_classic_rts_tech_tree_v1 / trillionnium_world_bevy_classic_rts_projectile_ability_v1 / trillionnium_world_bevy_classic_rts_ai_skirmish_pressure_v1 / trillionnium_world_bevy_classic_rts_objective_victory_loop_v1 / trillionnium_world_bevy_classic_rts_creep_camp_terrain_route_v1 / trillionnium_world_bevy_classic_rts_fog_scouting_intel_v1 / trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure_v1 / trillionnium_world_bevy_classic_rts_army_production_rally_v1 / trillionnium_world_bevy_classic_rts_base_assault_resolution_v1 / trillionnium_world_bevy_classic_rts_battle_aftermath_v1 / trillionnium_world_bevy_classic_rts_commander_progression_v1 / trillionnium_world_bevy_classic_rts_expansion_counterattack_v1 / trillionnium_world_bevy_classic_rts_tier_two_siege_push_v1 / trillionnium_world_bevy_classic_rts_siege_breach_counterplay_v1 / trillionnium_world_bevy_classic_rts_inner_lane_breakthrough_v1 / trillionnium_world_bevy_classic_rts_central_keep_pressure_v1 / trillionnium_world_bevy_classic_rts_central_keep_breakthrough_v1 / trillionnium_world_bevy_classic_rts_mirror_city_restoration_v1 / trillionnium_world_bevy_classic_rts_open_world_after_action_v1 / trillionnium_world_bevy_classic_rts_campaign_handoff_v1 / trillionnium_world_bevy_classic_rts_campaign_entry_v1 / trillionnium_world_bevy_classic_rts_visual_fidelity_v1 / trillionnium_world_bevy_classic_rts_command_affordance_v1 / trillionnium_world_bevy_classic_rts_command_surface_v1 / trillionnium_world_bevy_classic_rts_structure_modeling_v1 / trillionnium_world_bevy_classic_rts_environment_life_v1 / trillionnium_world_bevy_classic_rts_worker_harvest_animation_v1 / trillionnium_world_bevy_classic_rts_production_spawn_animation_v1 / trillionnium_world_bevy_classic_rts_unit_status_portrait_v1 / trillionnium_world_bevy_classic_rts_selection_command_feedback_v1 / trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1 / trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback_v1 / trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview_v1 / trillionnium_world_bevy_classic_rts_control_group_recall_override_preview_v1 / trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip_v1 / trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle_v1 / trillionnium_world_bevy_classic_rts_scrollable_map_v1 / trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1 / trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1 / trillionnium_world_bevy_classic_rts_formation_move_preview_v1 / trillionnium_world_bevy_classic_rts_formation_move_execution_v1 / trillionnium_world_bevy_classic_rts_local_obstruction_recovery_v1 / trillionnium_world_bevy_classic_rts_action_cadence_v1 / trillionnium_world_bevy_classic_rts_unit_model_depth_v1 / trillionnium_world_bevy_classic_rts_action_sequence_v1 / trillionnium_world_bevy_classic_rts_npc_behavior_loop_v1 / trillionnium_world_bevy_classic_rts_combat_impact_loop_v1 / trillionnium_world_bevy_classic_rts_locomotion_blend_v1 / trillionnium_world_bevy_classic_rts_npc_transition_blend_v1 / trillionnium_world_bevy_classic_rts_depth_readability_v1 / trillionnium_world_bevy_classic_playtest_readiness_v1 / trillionnium_world_bevy_classic_playtest_runner_status_v1 / trillionnium_world_bevy_classic_playtest_launcher_v1
# Bevy classic control-group command history contract: trillionnium_world_bevy_classic_rts_control_group_command_history_v1
# Bevy classic control-group command history prune contract: trillionnium_world_bevy_classic_rts_control_group_command_history_prune_v1
# Bevy first-minute command feedback replay contract: trillionnium_world_bevy_first_minute_command_feedback_replay_v1
# Bevy first-minute command feedback rejection replay contract: trillionnium_world_bevy_first_minute_command_feedback_rejection_replay_v1
# Bevy classic OpenRA parity bridge contract: trillionnium_world_bevy_classic_rts_openra_parity_bridge_v1
# Bevy classic owned replay file contract: trillionnium_world_bevy_classic_rts_owned_replay_file_v1
# Bevy classic headless replay playback contract: trillionnium_world_bevy_classic_rts_headless_replay_playback_v1
# Bevy classic natural terminal contract: trillionnium_world_bevy_classic_rts_natural_terminal_contract_v1
# Bevy classic native bot AI planner contract: trillionnium_world_bevy_classic_rts_native_bot_ai_planner_v1
# Bevy classic OpenRA parity lane contract: trillionnium_world_bevy_classic_rts_openra_parity_lane_v1
# Bevy classic OpenRA replay compatibility adapter contract: trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1
# Bevy classic OpenRA command vocabulary adapter contract: trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter_v1
# Bevy classic OpenRA order serializer fixture contract: trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture_v1
# Bevy classic OpenRA replay importer contract: trillionnium_world_bevy_classic_rts_openra_replay_importer_v1
# Bevy classic OpenRA order payload decoder contract: trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1
# Bevy classic OpenRA imported replay reducer contract: trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer_v1
# Bevy classic OpenRA imported headless comparison harness contract: trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1
# Bevy classic OpenRA imported replay audit ledger contract: trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1
# Bevy classic OpenRA imported replay repro manifest contract: trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1
# Bevy classic OpenRA order replay reducer contract: trillionnium_world_bevy_classic_rts_openra_order_replay_reducer_v1
# Bevy classic OpenRA headless comparison harness contract: trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness_v1
# Bevy classic planner live autonomous bot loop contract: trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop_v1
# Bevy classic bot planner action executor contract: trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1
# Bevy classic bot planner executor replay determinism contract: trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1
# Bevy classic multi-match bot executor evaluation contract: trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1
# Bevy classic bot executor failure/recovery matrix contract: trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1
# Bevy classic bot gap foundation contracts: trillionnium_world_bevy_classic_rts_bot_decision_state_gap_v1 / trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap_v1 / trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap_v1 / trillionnium_world_bevy_classic_rts_bot_map_intel_gap_v1
run_check bevy_classic_asset_pack_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh"
run_check bevy_classic_manifest_lint_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh"
run_check bevy_classic_animation_preview_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_preview.sh"
run_check bevy_classic_animation_selector_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_selector.sh"
run_check bevy_classic_player_motion_probe_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_player_motion_probe.sh"
run_check bevy_classic_input_frame_budget_contract_guard "$ROOT/scripts/v2/bevy_classic_input_frame_budget_script_contract_guard_test.sh"
run_check bevy_classic_input_frame_budget_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh"
run_check bevy_classic_render_budget_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_render_budget.sh"
run_check bevy_classic_scene_preview_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_scene_preview.sh"
run_check bevy_classic_model_catalog_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh"
run_check bevy_classic_renderer_probe_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh"
run_check bevy_classic_isometric_modeling_contract_guard "$ROOT/scripts/v2/bevy_classic_isometric_modeling_script_contract_guard_test.sh"
run_check bevy_classic_isometric_modeling_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh"
run_check bevy_classic_rts_openra_like_core_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_like_core.sh"
run_check bevy_classic_rts_control_loop_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_loop_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_loop_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_loop.sh"
run_check bevy_classic_rts_selection_minimap_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_selection_minimap_script_contract_guard_test.sh"
run_check bevy_classic_rts_selection_minimap_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_minimap.sh"
run_check bevy_classic_rts_build_lifecycle_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_build_lifecycle_script_contract_guard_test.sh"
run_check bevy_classic_rts_build_lifecycle_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh"
run_check bevy_classic_rts_tech_tree_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_tech_tree_script_contract_guard_test.sh"
run_check bevy_classic_rts_tech_tree_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tech_tree.sh"
run_check bevy_classic_rts_projectile_ability_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_projectile_ability_script_contract_guard_test.sh"
run_check bevy_classic_rts_projectile_ability_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_projectile_ability.sh"
run_check bevy_classic_rts_ai_skirmish_pressure_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_ai_skirmish_pressure_script_contract_guard_test.sh"
run_check bevy_classic_rts_ai_skirmish_pressure_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ai_skirmish_pressure.sh"
run_check bevy_classic_rts_objective_victory_loop_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_objective_victory_loop_script_contract_guard_test.sh"
run_check bevy_classic_rts_objective_victory_loop_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_objective_victory_loop.sh"
run_check bevy_classic_rts_creep_camp_terrain_route_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_creep_camp_terrain_route_script_contract_guard_test.sh"
run_check bevy_classic_rts_creep_camp_terrain_route_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_creep_camp_terrain_route.sh"
run_check bevy_classic_rts_fog_scouting_intel_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_fog_scouting_intel_script_contract_guard_test.sh"
run_check bevy_classic_rts_fog_scouting_intel_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_fog_scouting_intel.sh"
run_check bevy_classic_rts_enemy_base_tech_pressure_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_enemy_base_tech_pressure_script_contract_guard_test.sh"
run_check bevy_classic_rts_enemy_base_tech_pressure_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure.sh"
run_check bevy_classic_rts_army_production_rally_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_army_production_rally_script_contract_guard_test.sh"
run_check bevy_classic_rts_army_production_rally_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_army_production_rally.sh"
run_check bevy_classic_rts_base_assault_resolution_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_base_assault_resolution_script_contract_guard_test.sh"
run_check bevy_classic_rts_base_assault_resolution_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_base_assault_resolution.sh"
run_check bevy_classic_rts_battle_aftermath_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_battle_aftermath_script_contract_guard_test.sh"
run_check bevy_classic_rts_battle_aftermath_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_battle_aftermath.sh"
run_check bevy_classic_rts_commander_progression_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_commander_progression_script_contract_guard_test.sh"
run_check bevy_classic_rts_commander_progression_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_commander_progression.sh"
run_check bevy_classic_rts_expansion_counterattack_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_expansion_counterattack_script_contract_guard_test.sh"
run_check bevy_classic_rts_expansion_counterattack_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_expansion_counterattack.sh"
run_check bevy_classic_rts_tier_two_siege_push_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_tier_two_siege_push_script_contract_guard_test.sh"
run_check bevy_classic_rts_tier_two_siege_push_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tier_two_siege_push.sh"
run_check bevy_classic_rts_siege_breach_counterplay_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_siege_breach_counterplay_script_contract_guard_test.sh"
run_check bevy_classic_rts_siege_breach_counterplay_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_siege_breach_counterplay.sh"
run_check bevy_classic_rts_inner_lane_breakthrough_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_inner_lane_breakthrough_script_contract_guard_test.sh"
run_check bevy_classic_rts_inner_lane_breakthrough_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_inner_lane_breakthrough.sh"
run_check bevy_classic_rts_central_keep_pressure_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_central_keep_pressure_script_contract_guard_test.sh"
run_check bevy_classic_rts_central_keep_pressure_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_pressure.sh"
run_check bevy_classic_rts_central_keep_breakthrough_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_central_keep_breakthrough_script_contract_guard_test.sh"
run_check bevy_classic_rts_central_keep_breakthrough_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_breakthrough.sh"
run_check bevy_classic_rts_mirror_city_restoration_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_mirror_city_restoration_script_contract_guard_test.sh"
run_check bevy_classic_rts_mirror_city_restoration_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_mirror_city_restoration.sh"
run_check bevy_classic_rts_open_world_after_action_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_open_world_after_action_script_contract_guard_test.sh"
run_check bevy_classic_rts_open_world_after_action_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_open_world_after_action.sh"
run_check bevy_classic_rts_campaign_handoff_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_campaign_handoff_script_contract_guard_test.sh"
run_check bevy_classic_rts_campaign_handoff_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_handoff.sh"
run_check bevy_classic_rts_campaign_entry_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_campaign_entry_script_contract_guard_test.sh"
run_check bevy_classic_rts_campaign_entry_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_entry.sh"
run_check bevy_classic_rts_visual_fidelity_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_visual_fidelity_script_contract_guard_test.sh"
run_check bevy_classic_rts_visual_fidelity_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_visual_fidelity.sh"
run_check bevy_classic_rts_command_affordance_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_command_affordance_script_contract_guard_test.sh"
run_check bevy_classic_rts_command_affordance_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_affordance.sh"
run_check bevy_classic_rts_command_surface_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_command_surface_script_contract_guard_test.sh"
run_check bevy_classic_rts_command_surface_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_surface.sh"
run_check bevy_classic_rts_structure_modeling_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_structure_modeling_script_contract_guard_test.sh"
run_check bevy_classic_rts_structure_modeling_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_structure_modeling.sh"
run_check bevy_classic_rts_environment_life_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_environment_life_script_contract_guard_test.sh"
run_check bevy_classic_rts_environment_life_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_environment_life.sh"
run_check bevy_classic_rts_worker_harvest_animation_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_worker_harvest_animation_script_contract_guard_test.sh"
run_check bevy_classic_rts_worker_harvest_animation_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_worker_harvest_animation.sh"
run_check bevy_classic_rts_production_spawn_animation_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_production_spawn_animation_script_contract_guard_test.sh"
run_check bevy_classic_rts_production_spawn_animation_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_spawn_animation.sh"
run_check bevy_classic_rts_unit_status_portrait_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_unit_status_portrait_script_contract_guard_test.sh"
run_check bevy_classic_rts_unit_status_portrait_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_status_portrait.sh"
run_check bevy_classic_rts_selection_command_feedback_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_selection_command_feedback_script_contract_guard_test.sh"
run_check bevy_classic_rts_selection_command_feedback_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_command_feedback.sh"
run_check bevy_classic_rts_ability_tooltip_telegraph_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_ability_tooltip_telegraph_script_contract_guard_test.sh"
run_check bevy_classic_rts_ability_tooltip_telegraph_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph.sh"
run_check bevy_classic_rts_control_group_hotkey_feedback_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_group_hotkey_feedback_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_group_hotkey_feedback_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback.sh"
run_check bevy_classic_rts_control_group_recall_formation_preview_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_group_recall_formation_preview_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_group_recall_formation_preview_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview.sh"
run_check bevy_classic_rts_control_group_recall_override_preview_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_group_recall_override_preview_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_group_recall_override_preview_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_recall_override_preview.sh"
run_check bevy_classic_rts_control_group_command_feedback_strip_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_feedback_strip_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_group_command_feedback_strip_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip.sh"
run_check bevy_classic_rts_control_group_command_feedback_lifecycle_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_feedback_lifecycle_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_group_command_feedback_lifecycle_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle.sh"
run_check bevy_classic_rts_control_group_command_history_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_history_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_group_command_history_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_history.sh"
run_check bevy_classic_rts_control_group_command_history_prune_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_control_group_command_history_prune_script_contract_guard_test.sh"
run_check bevy_classic_rts_control_group_command_history_prune_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_history_prune.sh"
run_check bevy_first_minute_command_feedback_replay_contract_guard "$ROOT/scripts/v2/bevy_first_minute_command_feedback_replay_script_contract_guard_test.sh"
run_check bevy_first_minute_command_feedback_replay_gate "$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh"
run_check bevy_first_minute_command_feedback_rejection_replay_contract_guard "$ROOT/scripts/v2/bevy_first_minute_command_feedback_rejection_replay_script_contract_guard_test.sh"
run_check bevy_first_minute_command_feedback_rejection_replay_gate "$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh"
run_check bevy_classic_rts_scrollable_map_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_scrollable_map_script_contract_guard_test.sh"
run_check bevy_classic_rts_scrollable_map_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_scrollable_map.sh"
run_check bevy_classic_rts_camera_minimap_sync_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_camera_minimap_sync_script_contract_guard_test.sh"
run_check bevy_classic_rts_camera_minimap_sync_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh"
run_check bevy_classic_rts_command_queue_path_preview_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_command_queue_path_preview_script_contract_guard_test.sh"
run_check bevy_classic_rts_command_queue_path_preview_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_queue_path_preview.sh"
run_check bevy_classic_rts_formation_move_preview_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_formation_move_preview_script_contract_guard_test.sh"
run_check bevy_classic_rts_formation_move_preview_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_preview.sh"
run_check bevy_classic_rts_formation_move_execution_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_formation_move_execution_script_contract_guard_test.sh"
run_check bevy_classic_rts_formation_move_execution_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_execution.sh"
run_check bevy_classic_rts_local_obstruction_recovery_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_local_obstruction_recovery_script_contract_guard_test.sh"
run_check bevy_classic_rts_local_obstruction_recovery_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_local_obstruction_recovery.sh"
run_check bevy_classic_rts_action_cadence_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_action_cadence_script_contract_guard_test.sh"
run_check bevy_classic_rts_action_cadence_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_cadence.sh"
run_check bevy_classic_rts_unit_model_depth_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_unit_model_depth_script_contract_guard_test.sh"
run_check bevy_classic_rts_unit_model_depth_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_model_depth.sh"
run_check bevy_classic_rts_action_sequence_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_action_sequence_script_contract_guard_test.sh"
run_check bevy_classic_rts_action_sequence_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_sequence.sh"
run_check bevy_classic_rts_npc_behavior_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_npc_behavior_script_contract_guard_test.sh"
run_check bevy_classic_rts_npc_behavior_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_behavior.sh"
run_check bevy_classic_rts_combat_impact_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_combat_impact_script_contract_guard_test.sh"
run_check bevy_classic_rts_combat_impact_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_combat_impact.sh"
run_check bevy_classic_rts_locomotion_blend_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_locomotion_blend_script_contract_guard_test.sh"
run_check bevy_classic_rts_locomotion_blend_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_locomotion_blend.sh"
run_check bevy_classic_rts_npc_transition_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_npc_transition_script_contract_guard_test.sh"
run_check bevy_classic_rts_npc_transition_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_transition.sh"
run_check bevy_classic_rts_depth_readability_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_depth_readability_script_contract_guard_test.sh"
run_check bevy_classic_rts_depth_readability_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_depth_readability.sh"
run_check bevy_classic_rts_openra_parity_bridge_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_parity_bridge_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_parity_bridge_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_bridge.sh"
run_check bevy_classic_rts_owned_replay_file_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_owned_replay_file_script_contract_guard_test.sh"
run_check bevy_classic_rts_owned_replay_file_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_owned_replay_file.sh"
run_check bevy_classic_rts_headless_replay_playback_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_headless_replay_playback_script_contract_guard_test.sh"
run_check bevy_classic_rts_headless_replay_playback_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_headless_replay_playback.sh"
run_check bevy_classic_rts_natural_terminal_contract_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_natural_terminal_contract_script_contract_guard_test.sh"
run_check bevy_classic_rts_natural_terminal_contract_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_natural_terminal_contract.sh"
run_check bevy_classic_rts_native_bot_ai_planner_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_native_bot_ai_planner_script_contract_guard_test.sh"
run_check bevy_classic_rts_native_bot_ai_planner_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_native_bot_ai_planner.sh"
run_check bevy_classic_rts_planner_live_autonomous_bot_loop_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_planner_live_autonomous_bot_loop_script_contract_guard_test.sh"
run_check bevy_classic_rts_planner_live_autonomous_bot_loop_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop.sh"
run_check bevy_classic_rts_openra_parity_lane_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_parity_lane_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_parity_lane_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_lane.sh"
run_check bevy_classic_rts_openra_replay_compat_adapter_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_replay_compat_adapter_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_replay_compat_adapter_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter.sh"
run_check bevy_classic_rts_openra_command_vocab_adapter_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_command_vocab_adapter_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_command_vocab_adapter_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter.sh"
run_check bevy_classic_rts_openra_order_serializer_fixture_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_order_serializer_fixture_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_order_serializer_fixture_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture.sh"
run_check bevy_classic_rts_openra_replay_importer_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_replay_importer_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_replay_importer_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_importer.sh"
run_check bevy_classic_rts_openra_order_payload_decoder_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_order_payload_decoder_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_order_payload_decoder_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_payload_decoder.sh"
run_check bevy_classic_rts_openra_imported_replay_reducer_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_imported_replay_reducer_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_imported_replay_reducer_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer.sh"
run_check bevy_classic_rts_openra_imported_headless_comparison_harness_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_imported_headless_comparison_harness_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_imported_headless_comparison_harness_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness.sh"
run_check bevy_classic_rts_openra_imported_replay_audit_ledger_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_imported_replay_audit_ledger_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_imported_replay_audit_ledger_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger.sh"
run_check bevy_classic_rts_openra_imported_replay_repro_manifest_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_imported_replay_repro_manifest_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_imported_replay_repro_manifest_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest.sh"
run_check bevy_classic_rts_openra_order_replay_reducer_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_order_replay_reducer_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_order_replay_reducer_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_replay_reducer.sh"
run_check bevy_classic_rts_openra_headless_comparison_harness_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_openra_headless_comparison_harness_script_contract_guard_test.sh"
run_check bevy_classic_rts_openra_headless_comparison_harness_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness.sh"
run_check bevy_classic_rts_bot_planner_action_executor_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_bot_planner_action_executor_script_contract_guard_test.sh"
run_check bevy_classic_rts_bot_planner_action_executor_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh"
run_check bevy_classic_rts_bot_planner_executor_replay_determinism_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_bot_planner_executor_replay_determinism_script_contract_guard_test.sh"
run_check bevy_classic_rts_bot_planner_executor_replay_determinism_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh"
run_check bevy_classic_rts_multi_match_bot_executor_evaluation_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_multi_match_bot_executor_evaluation_script_contract_guard_test.sh"
run_check bevy_classic_rts_multi_match_bot_executor_evaluation_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh"
run_check bevy_classic_rts_bot_executor_failure_recovery_matrix_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_bot_executor_failure_recovery_matrix_script_contract_guard_test.sh"
run_check bevy_classic_rts_bot_executor_failure_recovery_matrix_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh"
run_check bevy_classic_rts_bot_decision_state_gap_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_bot_decision_state_gap_script_contract_guard_test.sh"
run_check bevy_classic_rts_bot_decision_state_gap_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_decision_state_gap.sh"
run_check bevy_classic_rts_bot_adaptive_build_order_gap_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_bot_adaptive_build_order_gap_script_contract_guard_test.sh"
run_check bevy_classic_rts_bot_adaptive_build_order_gap_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap.sh"
run_check bevy_classic_rts_bot_tactical_micro_gap_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_bot_tactical_micro_gap_script_contract_guard_test.sh"
run_check bevy_classic_rts_bot_tactical_micro_gap_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh"
run_check bevy_classic_rts_bot_map_intel_gap_contract_guard "$ROOT/scripts/v2/bevy_classic_rts_bot_map_intel_gap_script_contract_guard_test.sh"
run_check bevy_classic_rts_bot_map_intel_gap_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_map_intel_gap.sh"
run_check bevy_classic_playtest_readiness_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
run_check bevy_classic_playtest_runner_status_contract_guard "$ROOT/scripts/v2/bevy_classic_playtest_runner_status_script_contract_guard_test.sh"
run_check bevy_classic_playtest_launcher_contract_guard "$ROOT/scripts/v2/bevy_classic_playtest_launcher_script_contract_guard_test.sh"
run_check bevy_classic_playtest_launcher_gate "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh"
run_check bevy_authored_art_pack_contract_guard "$ROOT/scripts/v2/authored_art_pack_script_contract_guard_test.sh"
run_check bevy_authored_art_pack_gate "$ROOT/scripts/check_trillionnium_world_bevy_authored_art_pack.sh"
run_check bevy_authored_sprite_sheet_contract_guard "$ROOT/scripts/v2/authored_sprite_sheet_script_contract_guard_test.sh"
run_check bevy_authored_sprite_sheet_gate "$ROOT/scripts/check_trillionnium_world_bevy_authored_sprite_sheet.sh"
run_check bevy_authored_texture_atlas_binding_contract_guard "$ROOT/scripts/v2/authored_texture_atlas_binding_script_contract_guard_test.sh"
run_check bevy_authored_texture_atlas_binding_gate "$ROOT/scripts/check_trillionnium_world_bevy_authored_texture_atlas_binding.sh"
run_check bevy_authored_material_consumption_contract_guard "$ROOT/scripts/v2/authored_material_consumption_script_contract_guard_test.sh"
run_check bevy_authored_material_consumption_gate "$ROOT/scripts/check_trillionnium_world_bevy_authored_material_consumption.sh"
run_check bevy_authored_material_application_contract_guard "$ROOT/scripts/v2/authored_material_application_script_contract_guard_test.sh"
run_check bevy_authored_material_application_gate "$ROOT/scripts/check_trillionnium_world_bevy_authored_material_application.sh"
run_check bevy_runtime_texture_asset_contract_guard "$ROOT/scripts/v2/runtime_texture_asset_script_contract_guard_test.sh"
run_check bevy_runtime_texture_asset_gate "$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_asset.sh"
# Runtime texture manifest probe contract: trillionnium_world_bevy_runtime_texture_manifest_probe_v1
run_check bevy_runtime_texture_manifest_probe_contract_guard "$ROOT/scripts/v2/runtime_texture_manifest_probe_script_contract_guard_test.sh"
run_check bevy_runtime_texture_manifest_probe_gate "$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_manifest_probe.sh"
# Bevy asset store registration contract: trillionnium_world_bevy_asset_store_registration_v1
run_check bevy_asset_store_registration_contract_guard "$ROOT/scripts/v2/asset_store_registration_script_contract_guard_test.sh"
run_check bevy_asset_store_registration_gate "$ROOT/scripts/check_trillionnium_world_bevy_asset_store_registration.sh"
# Bevy sprite asset binding contract: trillionnium_world_bevy_sprite_asset_binding_v1
run_check bevy_sprite_asset_binding_contract_guard "$ROOT/scripts/v2/sprite_asset_binding_script_contract_guard_test.sh"
run_check bevy_sprite_asset_binding_gate "$ROOT/scripts/check_trillionnium_world_bevy_sprite_asset_binding.sh"
# Bevy sprite texture sampling contract: trillionnium_world_bevy_sprite_texture_sampling_v1
run_check bevy_sprite_texture_sampling_contract_guard "$ROOT/scripts/v2/sprite_texture_sampling_script_contract_guard_test.sh"
run_check bevy_sprite_texture_sampling_gate "$ROOT/scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh"
run_check bevy_authored_render_frame_contract_guard "$ROOT/scripts/v2/authored_render_frame_script_contract_guard_test.sh"
run_check bevy_authored_render_frame_gate "$ROOT/scripts/check_trillionnium_world_bevy_authored_render_frame.sh"
run_check bevy_live_window_runtime_texture_manifest_contract_guard "$ROOT/scripts/v2/live_window_runtime_texture_manifest_script_contract_guard_test.sh"
run_check bevy_live_window_screenshot_sequence_gate "$ROOT/scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh"
run_check bevy_authored_live_visual_bridge_contract_guard "$ROOT/scripts/v2/authored_live_visual_bridge_script_contract_guard_test.sh"
run_check bevy_authored_live_visual_bridge_gate "$ROOT/scripts/check_trillionnium_world_bevy_authored_live_visual_bridge.sh"
run_check bevy_live_window_layer_pixel_probe_contract_guard "$ROOT/scripts/v2/live_window_layer_pixel_probe_script_contract_guard_test.sh"
run_check bevy_live_window_layer_pixel_probe_gate "$ROOT/scripts/check_trillionnium_world_bevy_live_window_layer_pixel_probe.sh"
run_check bevy_live_window_texture_correlation_contract_guard "$ROOT/scripts/v2/live_window_texture_correlation_script_contract_guard_test.sh"
run_check bevy_live_window_texture_correlation_gate "$ROOT/scripts/check_trillionnium_world_bevy_live_window_texture_correlation.sh"
# Bevy live-window sampled texture correlation contract: trillionnium_world_bevy_live_window_sampled_texture_correlation_v1
run_check bevy_live_window_sampled_texture_correlation_contract_guard "$ROOT/scripts/v2/live_window_sampled_texture_correlation_script_contract_guard_test.sh"
run_check bevy_live_window_sampled_texture_correlation_gate "$ROOT/scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh"
# Bevy render asset eligibility contract: trillionnium_world_bevy_render_asset_eligibility_v1
run_check bevy_render_asset_eligibility_contract_guard "$ROOT/scripts/v2/render_asset_eligibility_script_contract_guard_test.sh"
run_check bevy_render_asset_eligibility_gate "$ROOT/scripts/check_trillionnium_world_bevy_render_asset_eligibility.sh"
run_check bevy_live_window_screenshot_sequence_artifact jq -e '.contract_version == "trillionnium_world_bevy_live_window_screenshot_sequence_v1" and .green == true and .frame_sequence_gate == true and .contact_sheet_gate == true and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1" and .runtime_texture_manifest_hash_gate == true and .runtime_texture_launch_env_gate == true and .runtime_texture_handle_gate == true and .runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1" and .runtime_texture_sprite_asset_binding_contract == "trillionnium_world_bevy_sprite_asset_binding_v1" and .runtime_texture_sprite_asset_binding_gate == true and .runtime_texture_sprite_bound_surface_count >= 24 and .runtime_texture_image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1" and .runtime_texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1" and .gpu_upload_claimed == false and .android_s5_real_device_claimed == false and .live_osm_ingestion_claimed == false' "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json"
run_check bevy_desktop_real_machine_readiness_contract_guard "$ROOT/scripts/v2/bevy_desktop_real_machine_readiness_script_contract_guard_test.sh"
run_check bevy_desktop_real_machine_readiness_gate env TRNM_WORLD_DESKTOP_REAL_MACHINE_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh"
run_check bevy_desktop_playtest_review_packet_contract_guard "$ROOT/scripts/v2/bevy_desktop_playtest_review_packet_script_contract_guard_test.sh"
run_check bevy_desktop_playtest_review_packet_gate env TRNM_WORLD_DESKTOP_PLAYTEST_REVIEW_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_bevy_desktop_playtest_review_packet.sh"
run_check public_launch_evidence_intake_contract_guard "$ROOT/scripts/v2/public_launch_evidence_intake_script_contract_guard_test.sh"
run_check public_launch_evidence_intake_gate "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh"
run_check release_readiness_entry_guard "$ROOT/scripts/v2/release_readiness_release_review_entry_guard_test.sh"
run_check production_map_pack_public_evidence_collection_guard "$ROOT/scripts/v2/production_map_pack_public_evidence_collection_script_contract_guard_test.sh"
run_check production_map_pack_public_evidence_collection "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh"
run_check production_map_pack_public_evidence_guard "$ROOT/scripts/v2/production_map_pack_public_evidence_script_contract_guard_test.sh"
run_check map_modeling_gate_contract_guard "$ROOT/scripts/v2/map_modeling_gate_script_contract_guard_test.sh"
run_check map_modeling_gate "$ROOT/scripts/check_trillionnium_world_map_modeling_gate.sh"
run_check ui_map_modeling_full_alignment_contract_guard "$ROOT/scripts/v2/ui_map_modeling_full_alignment_script_contract_guard_test.sh"
run_check ui_map_modeling_full_alignment_gate env TRNM_WORLD_FULL_ALIGNMENT_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh"
run_check production_map_pack_public_evidence_artifact_guard "$ROOT/scripts/v2/production_map_pack_public_evidence_artifact_guard_test.sh"
run_check readme_release_review_guard "$ROOT/scripts/v2/root_readme_world_release_review_quickcheck_guard_test.sh"
run_check packet_integrity_gate "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"
run_check readme_local_links "$ROOT/scripts/check_root_readme_local_links.sh"
run_check workflow_script_refs env \
  WORKFLOW_SCRIPT_REF_STRICT=1 \
  WORKFLOW_SCRIPT_REF_SUMMARY_PATH="$ACCEPTANCE_DIR/release-review-ci-gate-workflow-script-refs.json" \
  "$ROOT/scripts/validate_workflow_script_refs.sh"

PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
CHECKS_JSON="$(jq -s '.' "$CHECK_RESULTS")"
FAILURES_JSON="$(jq -s '[.[] | select(.status != "ok")]' "$CHECK_RESULTS")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
INTEGRITY_GREEN="$(jq -r '.green // false' "$PACKET_INTEGRITY_JSON" 2>/dev/null || printf 'false')"
READY_FOR_RELEASE_REVIEW="$(jq -r '.ready_for_release_review // false' "$PACKET_INTEGRITY_JSON" 2>/dev/null || printf 'false')"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // false' "$PACKET_INTEGRITY_JSON" 2>/dev/null || printf 'false')"
ARTIFACT_COUNT="$(jq -r '.artifact_count // 0' "$PACKET_INTEGRITY_JSON" 2>/dev/null || printf '0')"
INTEGRITY_FAILURE_COUNT="$(jq -r '(.failures // []) | length' "$PACKET_INTEGRITY_JSON" 2>/dev/null || printf '0')"

GREEN=false
STATUS=release_review_ci_gate_blocked
if [[ "$FAILURE_COUNT" == "0" && "$INTEGRITY_GREEN" == "true" && "$READY_FOR_RELEASE_REVIEW" == "true" ]]; then
  GREEN=true
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    STATUS=release_review_ci_gate_green
  else
    STATUS=release_review_ci_gate_green_with_public_launch_blockers
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_ci_gate_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg packet_integrity_json "$PACKET_INTEGRITY_JSON" \
  --arg workflow_refs_summary "$ACCEPTANCE_DIR/release-review-ci-gate-workflow-script-refs.json" \
  --argjson green "$GREEN" \
  --argjson ready_for_release_review "$READY_FOR_RELEASE_REVIEW" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson artifact_count "$ARTIFACT_COUNT" \
  --argjson integrity_failure_count "$INTEGRITY_FAILURE_COUNT" \
  --argjson checks "$CHECKS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_ci_gate",
    green: $green,
    ready_for_release_review: $ready_for_release_review,
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    ci_gate_rule: "release_review_ci_gate_runs_local_bevy_playability_packet_integrity_static_guards_readme_links_workflow_refs_and_checkpoint_manifest_without_claiming_android_s5_real_device_ready",
    packet_integrity_summary: $packet_integrity_json,
    workflow_script_refs_summary: $workflow_refs_summary,
    artifact_count: $artifact_count,
    packet_integrity_failure_count: $integrity_failure_count,
    checks: $checks,
    failures: $failures,
    reviewer_next_action: (if $green and $public_launch_ready then "review_public_launch_ready_evidence" elif $green then "collect_real_external_public_launch_evidence" else "repair_release_review_ci_gate_failures" end)
  }' >"$SUMMARY_FILE"

case "$STATUS" in
  release_review_ci_gate_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CI_GATE_GREEN %s\n' "$SUMMARY_FILE"
    ;;
  release_review_ci_gate_green_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CI_GATE_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s\n' "$SUMMARY_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CI_GATE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
    exit 1
    ;;
esac
