#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VISUAL_FOUNDATION_FIXTURE_LIB="$ROOT/scripts/v2/release_review_packet_integrity_visual_foundation_fixture_lib.sh"
source "$VISUAL_FOUNDATION_FIXTURE_LIB"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
packet_json="$TMP_DIR/release-review-packet.json"
packet_md="$TMP_DIR/release-review-packet.md"
packet_log="$TMP_DIR/release-review-packet.log"
summary_json="$TMP_DIR/release-review-packet-integrity.json"
artifacts_jsonl="$TMP_DIR/artifacts.jsonl"
SOURCE_CHAIN_REFRESH="${TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH:-1}"
add_artifact_from_path() {
  local id="$1"
  local label="$2"
  local artifact_path="$3"
  local role="$4"
  local artifact_sha
  local artifact_bytes
  local contract_version=""
  local status=""
  artifact_sha="$(sha256sum "$artifact_path" | awk '{print $1}')"
  artifact_bytes="$(wc -c <"$artifact_path" | tr -d ' ')"
  if [[ "$artifact_path" == *.json ]]; then
    contract_version="$(jq -r '.contract_version // empty' "$artifact_path" 2>/dev/null || true)"
    status="$(jq -r '.status // .overall_status // empty' "$artifact_path" 2>/dev/null || true)"
  fi
  jq -nc \
    --arg id "$id" \
    --arg label "$label" \
    --arg path "$artifact_path" \
    --arg role "$role" \
    --arg sha "$artifact_sha" \
    --arg bytes "$artifact_bytes" \
    --arg contract_version "$contract_version" \
    --arg status "$status" \
    '{
      id: $id,
      label: $label,
      path: $path,
      role: $role,
      file_status: "present",
      sha256: $sha,
      bytes: ($bytes | tonumber),
      contract_version: (if $contract_version == "" then null else $contract_version end),
      status: (if $status == "" then null else $status end)
    }' >>"$artifacts_jsonl"
}
add_release_review_checkpoint_manifest_packet_fixtures
add_visual_foundation_packet_fixtures
add_modeling_foundation_packet_fixtures
add_performance_budget_packet_fixtures
add_playtest_runner_packet_fixtures
add_classic_playtest_launcher_packet_fixtures
add_classic_playtest_handoff_packet_fixtures
add_campaign_ui_continuity_packet_fixtures
add_map_modeling_packet_fixtures
add_public_launch_readiness_packet_fixtures
add_public_launch_collection_packet_fixtures
add_public_launch_validator_packet_fixtures
add_public_launch_s5_real_device_validator_packet_fixtures
add_public_launch_evidence_intake_packet_fixtures
add_public_launch_blocker_consistency_packet_fixtures
add_public_launch_evidence_kit_packet_fixtures
add_public_launch_template_negative_fixtures_packet_fixtures
add_public_launch_evidence_bundle_packet_fixtures
add_public_launch_bundle_negative_fixtures_packet_fixtures
add_public_launch_status_only_fixture_guard_packet_fixtures
add_public_launch_operator_handoff_packet_fixtures
add_cex_adapter_readiness_packet_fixtures
add_release_signoff_summary_packet_fixtures
add_release_review_quickcheck_packet_fixtures
add_release_review_status_packet_fixtures
add_release_review_convergence_packet_fixtures
add_release_review_packet_convergence_log_packet_fixtures
add_live_window_mouse_hit_test_packet_fixtures
add_camera_minimap_sync_packet_fixtures
add_first_contact_basin_source_manifest_packet_fixtures

semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_semantic_fixture_v1",
  status: "release_review_packet_integrity_semantic_fixture_green",
  green: true,
  fixture_kind: "release_review_convergence_status_quickcheck_release_signoff_cex_adapter_first_minute_command_feedback_and_handoff_first_contact_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_release_review_convergence_status_quickcheck_release_signoff_summary_cex_adapter_readiness_first_minute_command_feedback_and_handoff_first_contact_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 22,
  expected_semantic_failure_names: [
    "release_review_checkpoint_manifest_semantics",
    "release_review_convergence_semantics",
    "release_review_status_semantics",
    "release_review_status_markdown_semantics",
    "release_review_quickcheck_semantics",
    "release_signoff_summary_semantics",
    "cex_adapter_readiness_semantics",
    "first_minute_command_feedback_replay_semantics",
    "first_minute_command_feedback_source_recording_semantics",
    "first_minute_command_feedback_recording_semantics",
    "first_minute_command_feedback_replay_ppm_semantics",
    "first_minute_command_feedback_rejection_replay_semantics",
    "first_minute_command_feedback_rejection_source_recording_semantics",
    "first_minute_command_feedback_rejection_recording_semantics",
    "first_minute_command_feedback_rejection_replay_ppm_semantics",
    "classic_playtest_readiness_full_game_visual_ui_replication_semantics",
    "classic_playtest_readiness_openra_style_screen_set_review_semantics",
    "classic_playtest_readiness_semantics",
    "campaign_outcome_ui_readiness_semantics",
    "classic_playtest_handoff_readiness_semantics",
    "classic_playtest_handoff_packet_semantics",
    "classic_playtest_handoff_packet_markdown_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_semantic_fixture "Release review packet integrity semantic fixture" "$semantic_fixture_json" release_review_gate
bot_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-bot-executor-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture_v1",
  status: "release_review_packet_integrity_bot_executor_semantic_fixture_green",
  green: true,
  fixture_kind: "bot_executor_source_chain_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_bot_executor_source_chain_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 9,
  expected_semantic_failure_names: [
    "bot_planner_action_executor_semantics",
    "bot_planner_action_executor_log_semantics",
    "bot_planner_action_executor_ppm_semantics",
    "bot_planner_executor_replay_determinism_semantics",
    "bot_planner_executor_replay_determinism_log_semantics",
    "bot_planner_executor_replay_determinism_ppm_semantics",
    "multi_match_bot_executor_evaluation_semantics",
    "multi_match_bot_executor_evaluation_log_semantics",
    "multi_match_bot_executor_evaluation_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$bot_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_bot_executor_semantic_fixture "Release review packet integrity bot executor semantic fixture" "$bot_semantic_fixture_json" release_review_gate
bot_matrix_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-bot-executor-matrix-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture_v1",
  status: "release_review_packet_integrity_bot_executor_matrix_semantic_fixture_green",
  green: true,
  fixture_kind: "bot_executor_failure_recovery_matrix_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_bot_executor_failure_recovery_matrix_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 3,
  expected_semantic_failure_names: [
    "bot_executor_failure_recovery_matrix_semantics",
    "bot_executor_failure_recovery_matrix_log_semantics",
    "bot_executor_failure_recovery_matrix_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$bot_matrix_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_bot_executor_matrix_semantic_fixture "Release review packet integrity bot executor failure/recovery matrix semantic fixture" "$bot_matrix_semantic_fixture_json" release_review_gate
bot_gap_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-bot-gap-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture_v1",
  status: "release_review_packet_integrity_bot_gap_semantic_fixture_green",
  green: true,
  fixture_kind: "bot_gap_foundation_micro_intel_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_bot_gap_foundation_micro_intel_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 8,
  expected_semantic_failure_names: [
    "bot_decision_state_gap_semantics",
    "bot_decision_state_gap_ppm_semantics",
    "bot_adaptive_build_order_gap_semantics",
    "bot_adaptive_build_order_gap_ppm_semantics",
    "bot_tactical_micro_gap_semantics",
    "bot_tactical_micro_gap_ppm_semantics",
    "bot_map_intel_gap_semantics",
    "bot_map_intel_gap_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$bot_gap_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_bot_gap_semantic_fixture "Release review packet integrity bot gap semantic fixture" "$bot_gap_semantic_fixture_json" release_review_gate
control_loop_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-control-loop-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture_v1",
  status: "release_review_packet_integrity_control_loop_semantic_fixture_green",
  green: true,
  fixture_kind: "classic_rts_control_loop_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_classic_rts_control_loop_summary_and_ppm_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 2,
  expected_semantic_failure_names: [
    "classic_rts_control_loop_semantics",
    "classic_rts_control_loop_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$control_loop_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_control_loop_semantic_fixture "Release review packet integrity selection/minimap semantic fixture" "$control_loop_semantic_fixture_json" release_review_gate
control_loop_json="$TMP_DIR/bevy-classic-rts-control-loop.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_control_loop_v1",
  green: true,
  preview_width: 1280,
  preview_height: 360,
  preview_format: "ppm_p3_rgb",
  write_gate: true,
  mirror_scene_gate: true,
  coliseum_scene_gate: true,
  non_background_pixels: 460800,
  control_group_id: "1",
  move_selected_unit_count: 4,
  attack_selected_unit_count: 4,
  move_command_queue: ["select_group_1", "move:7,4", "formation:diamond"],
  attack_command_queue: ["select_group_1", "attack:arena_creep_attack"],
  attack_target_id: "arena_creep_attack",
  selection_marker_pixel_count: 1576,
  formation_line_pixel_count: 485,
  command_marker_pixel_count: 808,
  attack_feedback_pixel_count: 601,
  strategy_panel_pixel_count: 140066,
  minimap_pixel_count: 3833,
  fog_pixel_count: 2124,
  vision_pixel_count: 240,
  resource_hud_pixel_count: 462,
  production_queue_pixel_count: 9045,
  move_production_queue: ["train:worker", "train:guard"],
  move_build_queue: ["build:scout_tower"],
  attack_build_queue: ["upgrade:training_hall"],
  move_training_progress_percent: 64,
  attack_build_progress_percent: 56,
  unit_health_card_pixel_count: 804,
  ability_command_pixel_count: 13511,
  target_health_pixel_count: 350,
  attack_target_health_percent: 46,
  attack_active_ability_id: "focus_fire",
  attack_ability_command_ids: ["attack", "focus_fire", "guard", "retreat"],
  attack_combat_event_log: ["focus_fire:arena_creep_attack", "damage:28"],
  selection_gate: true,
  command_queue_gate: true,
  strategy_hud_gate: true,
  macro_loop_gate: true,
  tactical_combat_gate: true,
  gameplay_surface_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$control_loop_json"
add_artifact_from_path native_bevy_classic_rts_control_loop "Native/Bevy classic RTS control loop" "$control_loop_json" release_review_input
control_loop_ppm="$TMP_DIR/bevy-classic-rts-control-loop.ppm"
printf 'P3\n1280 360\n255\n' >"$control_loop_ppm"
truncate -s 4000001 "$control_loop_ppm"
add_artifact_from_path native_bevy_classic_rts_control_loop_ppm "Native/Bevy classic RTS control loop PPM" "$control_loop_ppm" release_review_visual_evidence
selection_minimap_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-selection-minimap-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_selection_minimap_semantic_fixture_v1",
  status: "release_review_packet_integrity_selection_minimap_semantic_fixture_green",
  green: true,
  fixture_kind: "classic_rts_selection_minimap_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_classic_rts_selection_minimap_summary_and_ppm_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 2,
  expected_semantic_failure_names: [
    "classic_rts_selection_minimap_semantics",
    "classic_rts_selection_minimap_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$selection_minimap_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_selection_minimap_semantic_fixture "Release review packet integrity selection/minimap semantic fixture" "$selection_minimap_semantic_fixture_json" release_review_gate
selection_minimap_json="$TMP_DIR/bevy-classic-rts-selection-minimap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_selection_minimap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 720,
  preview_format: "ppm_p3_rgb",
  write_gate: true,
  input_path: "apply_live_native_action_with_source(classic_rts_selection_minimap_input)",
  input_action_count: 4,
  accepted_input_count: 4,
  action_labels: ["RTS:SELECT:box:frontline", "RTS:MOVE:minimap:9,2:rally", "RTS:SELECT:2", "RTS:MOVE:6,5:split"],
  stage_summaries: [{
    stage: "minimap_rally",
    minimap_command_tile_id: "9,2",
    minimap_command_kind: "rally",
    group_route_tile_ids: ["6,5", "7,4", "8,3", "9,2"]
  }],
  final_control_group_id: "2",
  final_selected_unit_ids: ["square_guard_patrol", "square_creep_wander"],
  final_selection_box_tile_ids: ["5,5", "6,5", "5,4", "6,4"],
  final_control_group_assignments: ["1:player|square_guard_patrol|square_worker_carry|square_creep_wander", "2:square_guard_patrol|square_creep_wander"],
  final_active_control_group_ids: ["1", "2"],
  final_minimap_command_tile_id: "6,5",
  final_minimap_command_kind: "split",
  final_group_route_tile_ids: ["5,5", "6,4", "6,5", "7,5", "6,6"],
  final_group_command_state: "split_route:group_2",
  final_command_queue: ["box_select:5,5|6,5|5,4|6,4", "select_group_1", "minimap:rally:9,2", "move:9,2", "formation:rally", "path:6,5>7,4>8,3>9,2", "slots:8,3|9,2|10,2|9,3", "assign_group_2:square_guard_patrol|square_creep_wander", "select_group_2", "split_route:5,5>6,4>6,5>7,5>6,6", "move:6,5", "formation:split", "path:6,5", "slots:5,5|7,5|5,6|7,6", "disperse:5,5|6,4|6,6|7,5"],
  non_background_pixels: 921600,
  selection_box_pixel_count: 4269,
  minimap_command_pixel_count: 2009,
  group_two_pixel_count: 342,
  split_route_pixel_count: 2183,
  live_selection_minimap_input_gate: true,
  selection_box_gate: true,
  control_group_gate: true,
  minimap_command_gate: true,
  split_route_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$selection_minimap_json"
add_artifact_from_path native_bevy_classic_rts_selection_minimap "Native/Bevy classic RTS selection/minimap" "$selection_minimap_json" release_review_input
selection_minimap_ppm="$TMP_DIR/bevy-classic-rts-selection-minimap.ppm"
printf 'P3\n1280 720\n255\n' >"$selection_minimap_ppm"
truncate -s 8000001 "$selection_minimap_ppm"
add_artifact_from_path native_bevy_classic_rts_selection_minimap_ppm "Native/Bevy classic RTS selection/minimap PPM" "$selection_minimap_ppm" release_review_visual_evidence
build_lifecycle_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-build-lifecycle-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_build_lifecycle_semantic_fixture_v1",
  status: "release_review_packet_integrity_build_lifecycle_semantic_fixture_green",
  green: true,
  fixture_kind: "classic_rts_build_lifecycle_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_classic_rts_build_lifecycle_summary_and_ppm_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 2,
  expected_semantic_failure_names: [
    "classic_rts_build_lifecycle_semantics",
    "classic_rts_build_lifecycle_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$build_lifecycle_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_build_lifecycle_semantic_fixture "Release review packet integrity build lifecycle semantic fixture" "$build_lifecycle_semantic_fixture_json" release_review_gate
build_lifecycle_json="$TMP_DIR/bevy-classic-rts-build-lifecycle.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_build_lifecycle_v1",
  green: true,
  preview_width: 640,
  preview_height: 360,
  preview_format: "ppm_p3_rgb",
  write_gate: true,
  input_path: "apply_live_native_action_with_source(classic_rts_build_lifecycle_input)",
  input_action_count: 6,
  accepted_input_count: 6,
  action_labels: ["RTS:SELECT:1", "RTS:QUEUE:build:watch_tower@7,4", "RTS:QUEUE:complete:watch_tower@7,4", "RTS:QUEUE:repair:watch_tower@7,4", "RTS:QUEUE:build:scout_tower@8,4", "RTS:QUEUE:cancel:build:1"],
  final_structure_state: "cancelled:scout_tower@8,4",
  final_build_site_tile_ids: ["7,4", "7,5", "8,4"],
  final_building_blueprint_id: "watch_tower",
  final_building_progress_percent: 100,
  final_completed_structure_ids: ["watch_tower"],
  final_repair_target_id: "watch_tower",
  final_repair_progress_percent: 76,
  final_cancelled_structure_ids: ["scout_tower"],
  final_refund_delta_log: ["gold:+180"],
  final_structure_health_percents: [54, 91],
  final_resource_spend_log: ["commit:210g:build:watch_tower@7,4:1", "commit:45g:repair:watch_tower@7,4:6", "repair:-45g:-20l"],
  final_command_queue: ["select_group_1", "blueprint:watch_tower@7,4", "build_site:7,4|7,5|8,4", "queue:build:watch_tower@7,4", "complete:watch_tower@7,4", "queue:complete:watch_tower@7,4", "repair:watch_tower@7,4", "queue:repair:watch_tower@7,4", "blueprint:scout_tower@8,4", "build_site:8,4|8,5|9,4", "queue:build:scout_tower@8,4", "refund:scout_tower@8,4:gold:+180", "cancel:build:scout_tower@8,4"],
  non_background_pixels: 230400,
  build_blueprint_pixel_count: 72,
  build_progress_pixel_count: 44,
  structure_complete_pixel_count: 112,
  structure_health_pixel_count: 48,
  repair_pixel_count: 88,
  cancel_refund_pixel_count: 56,
  live_build_lifecycle_input_gate: true,
  build_placement_gate: true,
  completion_gate: true,
  repair_gate: true,
  cancel_refund_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$build_lifecycle_json"
add_artifact_from_path native_bevy_classic_rts_build_lifecycle "Native/Bevy classic RTS build lifecycle" "$build_lifecycle_json" release_review_input
build_lifecycle_ppm="$TMP_DIR/bevy-classic-rts-build-lifecycle.ppm"
printf 'P3\n640 360\n255\n' >"$build_lifecycle_ppm"
truncate -s 1000001 "$build_lifecycle_ppm"
add_artifact_from_path native_bevy_classic_rts_build_lifecycle_ppm "Native/Bevy classic RTS build lifecycle PPM" "$build_lifecycle_ppm" release_review_visual_evidence
tech_tree_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-tech-tree-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_tech_tree_semantic_fixture_v1",
  status: "release_review_packet_integrity_tech_tree_semantic_fixture_green",
  green: true,
  fixture_kind: "classic_rts_tech_tree_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_classic_rts_tech_tree_summary_and_ppm_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 2,
  expected_semantic_failure_names: [
    "classic_rts_tech_tree_semantics",
    "classic_rts_tech_tree_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$tech_tree_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_tech_tree_semantic_fixture "Release review packet integrity tech tree semantic fixture" "$tech_tree_semantic_fixture_json" release_review_gate
tech_tree_json="$TMP_DIR/bevy-classic-rts-tech-tree.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_tech_tree_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  preview_format: "ppm_p3_rgb",
  write_gate: true,
  input_path: "apply_live_native_action_with_source(classic_rts_tech_tree_input)",
  input_action_count: 6,
  accepted_input_count: 6,
  action_labels: ["RTS:SELECT:1", "RTS:QUEUE:faction:mirror_guard", "RTS:QUEUE:build:training_hall@4,3", "RTS:QUEUE:research:wayfinder_code@town_hall", "RTS:QUEUE:upgrade:iron_lacing@training_hall", "RTS:QUEUE:unlock:relay_guard"],
  final_faction_id: "mirror_guard",
  final_base_structure_ids: ["town_hall", "training_hall", "signal_spire"],
  final_tech_research_ids: ["wayfinder_code"],
  final_completed_upgrade_ids: ["iron_lacing"],
  final_unlocked_unit_ids: ["worker", "guard", "relay_guard"],
  final_unlocked_structure_ids: ["signal_spire"],
  final_tech_requirements_log: ["base:town_hall|required:training_hall|locked:relay_guard", "structure:training_hall:queued_at:4,3", "research:wayfinder_code:requires:town_hall", "upgrade:iron_lacing:requires:training_hall+wayfinder_code", "unlock:relay_guard:requires:iron_lacing+signal_spire"],
  final_tech_progress_percent: 100,
  final_tech_state: "unlocked:relay_guard",
  final_command_queue: ["select_group_1", "faction:mirror_guard:base_online", "queue:faction:mirror_guard", "blueprint:training_hall@4,3", "build_site:4,3", "queue:build:training_hall@4,3", "research:wayfinder_code@town_hall", "queue:research:wayfinder_code@town_hall", "upgrade:iron_lacing@training_hall", "queue:upgrade:iron_lacing@training_hall", "unlock:relay_guard", "queue:unlock:relay_guard"],
  rts_core_contract: "trnm_rts_core_frame_order_v1",
  rts_tech_tree_core_frame_orders: [1, 2, 3, 4, 5],
  rts_tech_tree_core_frame_order_kind_labels: ["queue", "build", "research", "upgrade", "unlock"],
  rts_tech_tree_core_frame_order_stream_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  rts_tech_tree_core_frame_order_errors: [],
  rts_tech_tree_core_frame_order_stream_error: null,
  rts_tech_tree_core_headless_replay_error: null,
  rts_tech_tree_core_headless_checkpoint_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  rts_tech_tree_core_headless_applied_order_count: 5,
  rts_tech_tree_core_headless_actor_count: 4,
  rts_tech_tree_core_headless_final_frame: 644,
  rts_tech_tree_core_tech_order_count: 3,
  rts_tech_tree_core_research_order_count: 1,
  rts_tech_tree_core_upgrade_order_count: 1,
  rts_tech_tree_core_unlock_order_count: 1,
  rts_tech_tree_core_researched_rule_ids: ["wayfinder_code"],
  rts_tech_tree_core_upgraded_rule_ids: ["iron_lacing"],
  rts_tech_tree_core_unlocked_rule_ids: ["relay_guard"],
  rts_tech_tree_core_source_actor_ids: ["town_hall", "training_hall"],
  rts_tech_tree_core_headless_replay_report: {checkpoint: {tech_tree: {tech_order_count: 3}, event_log: ["frame:642:player:Multi0:kind:research:subjects:1:target:wayfinder_code@town_hall", "frame:643:player:Multi0:kind:upgrade:subjects:1:target:iron_lacing@training_hall", "frame:644:player:Multi0:kind:unlock:subjects:1:target:relay_guard"]}},
  rts_tech_tree_core_frame_order_gate: true,
  rts_tech_tree_core_headless_replay_gate: true,
  non_background_pixels: 1382400,
  tech_base_pixel_count: 180,
  tech_research_pixel_count: 72,
  tech_upgrade_pixel_count: 64,
  tech_unlock_pixel_count: 108,
  tech_requirement_pixel_count: 96,
  live_tech_tree_input_gate: true,
  faction_base_gate: true,
  research_gate: true,
  upgrade_gate: true,
  unlock_gate: true,
  dependency_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$tech_tree_json"
add_artifact_from_path native_bevy_classic_rts_tech_tree "Native/Bevy classic RTS tech tree" "$tech_tree_json" release_review_input
tech_tree_ppm="$TMP_DIR/bevy-classic-rts-tech-tree.ppm"
printf 'P3\n1280 1080\n255\n' >"$tech_tree_ppm"
truncate -s 8000001 "$tech_tree_ppm"
add_artifact_from_path native_bevy_classic_rts_tech_tree_ppm "Native/Bevy classic RTS tech tree PPM" "$tech_tree_ppm" release_review_visual_evidence
projectile_ability_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-projectile-ability-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_projectile_ability_semantic_fixture_v1",
  status: "release_review_packet_integrity_projectile_ability_semantic_fixture_green",
  green: true,
  fixture_kind: "classic_rts_projectile_ability_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_classic_rts_projectile_ability_summary_and_ppm_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 121,
  expected_semantic_failure_count: 2,
  expected_semantic_failure_names: ["classic_rts_projectile_ability_semantics", "classic_rts_projectile_ability_ppm_semantics"],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$projectile_ability_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_projectile_ability_semantic_fixture "Release review packet integrity projectile/ability semantic fixture" "$projectile_ability_semantic_fixture_json" release_review_gate
projectile_ability_json="$TMP_DIR/bevy-classic-rts-projectile-ability.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_projectile_ability_v1",
  green: true,
  preview_width: 640,
  preview_height: 360,
  preview_format: "ppm_p3_rgb",
  write_gate: true,
  input_path: "apply_live_native_action_with_source(classic_rts_projectile_ability_input)",
  input_action_count: 5,
  accepted_input_count: 4,
  action_labels: ["RTS:SELECT:1", "RTS:MOVE:8,4:wedge", "RTS:ATTACK:arena_creep_attack", "RTS:ABILITY:focus_fire", "RTS:ABILITY:misfire"],
  final_active_projectile_id: "misfired_bolt",
  final_projectile_trail_tile_ids: ["5,4", "5,5"],
  final_projectile_impact_tile_id: "5,5",
  final_ability_effect_tile_ids: ["5,5", "7,5"],
  final_ability_damage_ticks: [9, 12, 10],
  final_target_health_percent: 44,
  final_target_armor_percent: 34,
  final_target_shield_percent: 12,
  final_ability_resolution_state: "misfired:guard_break:arena_creep_attack",
  final_command_queue: ["select_group_1", "move:8,4:wedge", "attack:arena_creep_attack", "ability:focus_fire", "ability:misfire", "damage_ticks:9+12+10", "armor_shield:34:12"],
  final_combat_event_log: ["projectile_launch:misfired_bolt", "projectile_impact:misfire:arena_creep_attack", "shield_remaining", "damage:31"],
  rts_core_contract: "trnm_rts_core_frame_order_v1",
  rts_projectile_ability_core_frame_orders: [1, 2, 3, 4],
  rts_projectile_ability_core_frame_order_kind_labels: ["move", "attack", "ability", "ability"],
  rts_projectile_ability_core_frame_order_stream_sha256: "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
  rts_projectile_ability_core_frame_order_errors: [],
  rts_projectile_ability_core_frame_order_stream_error: null,
  rts_projectile_ability_core_headless_replay_error: null,
  rts_projectile_ability_core_headless_checkpoint_sha256: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
  rts_projectile_ability_core_headless_applied_order_count: 4,
  rts_projectile_ability_core_headless_actor_count: 4,
  rts_projectile_ability_core_headless_final_frame: 704,
  rts_projectile_ability_core_headless_ability_order_count: 2,
  rts_projectile_ability_core_headless_ability_rule_ids: ["focus_fire", "guard_break"],
  rts_projectile_ability_core_headless_ability_target_actor_ids: ["arena_creep_attack", "arena_creep_attack"],
  rts_projectile_ability_core_frame_order_gate: true,
  rts_projectile_ability_core_headless_replay_gate: true,
  non_background_pixels: 230400,
  projectile_trail_pixel_count: 30,
  projectile_impact_pixel_count: 20,
  ability_radius_pixel_count: 40,
  damage_tick_pixel_count: 10,
  armor_shield_pixel_count: 8,
  attack_feedback_pixel_count: 80,
  live_projectile_ability_input_gate: true,
  projectile_trail_gate: false,
  projectile_impact_gate: false,
  ability_radius_gate: false,
  damage_tick_gate: false,
  armor_shield_gate: false,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$projectile_ability_json"
add_artifact_from_path native_bevy_classic_rts_projectile_ability "Native/Bevy classic RTS projectile/ability" "$projectile_ability_json" release_review_input
projectile_ability_ppm="$TMP_DIR/bevy-classic-rts-projectile-ability.ppm"
printf 'P3\n641 360\n255\n' >"$projectile_ability_ppm"
truncate -s 1000001 "$projectile_ability_ppm"
add_artifact_from_path native_bevy_classic_rts_projectile_ability_ppm "Native/Bevy classic RTS projectile/ability PPM" "$projectile_ability_ppm" release_review_visual_evidence
if [[ "$SOURCE_CHAIN_REFRESH" != "0" ]]; then
"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh" >"$TMP_DIR/first-minute-command-feedback-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh" >"$TMP_DIR/first-minute-command-feedback-rejection-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh" >"$TMP_DIR/bot-planner-action-executor.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh" >"$TMP_DIR/bot-planner-executor-replay-determinism.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh" >"$TMP_DIR/multi-match-bot-executor-evaluation.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh" >"$TMP_DIR/bot-executor-failure-recovery-matrix.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_decision_state_gap.sh" >"$TMP_DIR/bot-decision-state-gap.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap.sh" >"$TMP_DIR/bot-adaptive-build-order-gap.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh" >"$TMP_DIR/bot-tactical-micro-gap.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_map_intel_gap.sh" >"$TMP_DIR/bot-map-intel-gap.log"
fi
native_dir="$ROOT/acceptance/S5_native_bevy_device/latest"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay "Native/Bevy first-minute command feedback replay" "$native_dir/bevy-first-minute-command-feedback-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_source_recording "Native/Bevy first-minute command feedback source recording" "$native_dir/bevy-first-minute-command-feedback-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_recording "Native/Bevy first-minute command feedback command recording" "$native_dir/bevy-first-minute-command-feedback-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_replay_ppm "Native/Bevy first-minute command feedback replay PPM" "$native_dir/bevy-first-minute-command-feedback-replay.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay "Native/Bevy first-minute command feedback rejection replay" "$native_dir/bevy-first-minute-command-feedback-rejection-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_source_recording "Native/Bevy first-minute command feedback rejection source recording" "$native_dir/bevy-first-minute-command-feedback-rejection-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_recording "Native/Bevy first-minute command feedback rejection recording" "$native_dir/bevy-first-minute-command-feedback-rejection-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay_ppm "Native/Bevy first-minute command feedback rejection replay PPM" "$native_dir/bevy-first-minute-command-feedback-rejection-replay.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_planner_action_executor "Native/Bevy bot planner action executor" "$native_dir/bevy-classic-rts-bot-planner-action-executor.json" release_review_input
add_artifact_from_path native_bevy_bot_planner_action_executor_log "Native/Bevy bot planner action executor log" "$native_dir/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.actions.json" release_review_recording
add_artifact_from_path native_bevy_bot_planner_action_executor_ppm "Native/Bevy bot planner action executor PPM" "$native_dir/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism "Native/Bevy bot planner executor replay determinism" "$native_dir/bevy-classic-rts-bot-planner-executor-replay-determinism.json" release_review_input
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_log "Native/Bevy bot planner executor replay determinism log" "$native_dir/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.replay.json" release_review_recording
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_ppm "Native/Bevy bot planner executor replay determinism PPM" "$native_dir/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation "Native/Bevy multi-match bot executor evaluation" "$native_dir/bevy-classic-rts-multi-match-bot-executor-evaluation.json" release_review_input
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_log "Native/Bevy multi-match bot executor evaluation log" "$native_dir/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.matches.json" release_review_recording
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_ppm "Native/Bevy multi-match bot executor evaluation PPM" "$native_dir/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix "Native/Bevy bot executor failure recovery matrix" "$native_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix.json" release_review_input
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_log "Native/Bevy bot executor failure recovery matrix log" "$native_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.matrix.json" release_review_recording
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_ppm "Native/Bevy bot executor failure recovery matrix PPM" "$native_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_decision_state_gap "Native/Bevy bot decision-state gap" "$native_dir/bevy-classic-rts-bot-decision-state-gap.json" release_review_input
add_artifact_from_path native_bevy_bot_decision_state_gap_ppm "Native/Bevy bot decision-state gap PPM" "$native_dir/bevy-classic-rts-bot-decision-state-gap.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_adaptive_build_order_gap "Native/Bevy bot adaptive build-order gap" "$native_dir/bevy-classic-rts-bot-adaptive-build-order-gap.json" release_review_input
add_artifact_from_path native_bevy_bot_adaptive_build_order_gap_ppm "Native/Bevy bot adaptive build-order gap PPM" "$native_dir/bevy-classic-rts-bot-adaptive-build-order-gap.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_tactical_micro_gap "Native/Bevy bot tactical micro gap" "$native_dir/bevy-classic-rts-bot-tactical-micro-gap.json" release_review_input
add_artifact_from_path native_bevy_bot_tactical_micro_gap_ppm "Native/Bevy bot tactical micro gap PPM" "$native_dir/bevy-classic-rts-bot-tactical-micro-gap.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_map_intel_gap "Native/Bevy bot map intel gap" "$native_dir/bevy-classic-rts-bot-map-intel-gap.json" release_review_input
add_artifact_from_path native_bevy_bot_map_intel_gap_ppm "Native/Bevy bot map intel gap PPM" "$native_dir/bevy-classic-rts-bot-map-intel-gap.ppm" release_review_visual_evidence
production_desktop_review_packet_json="$TMP_DIR/bevy-classic-rts-production-desktop-review-packet.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_production_desktop_review_packet_v1", status: "classic_rts_production_desktop_review_packet_green", green: true, source_contract_count: 3, artifact_count: 6, artifact_bytes_total: 6, gate_count: 10, passed_gate_count: 10, failed_gate_count: 0, production_interaction_surface_count: 6, desktop_screenshot_frame_count: 11, desktop_keyboard_event_count: 13, desktop_mouse_event_count: 15, desktop_mouse_slot_a_bytes: 41520, android_s5_real_device_claimed: false, public_launch_ready_claimed: false, live_public_network_exposure_performed: false, live_osm_ingestion_performed: false, production_ready_desktop_review_shipped: false, source_contracts: {production_interaction_polish: "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1", desktop_playtest_review_packet: "trillionnium_world_bevy_desktop_playtest_review_packet_v1", desktop_real_machine_readiness: "trillionnium_world_bevy_desktop_real_machine_readiness_v1"}, gates: {production_interaction_polish_gate: true, desktop_playtest_review_packet_gate: true, desktop_real_machine_readiness_gate: true, keyboard_visual_review_gate: true, mouse_visual_review_gate: true, artifact_manifest_gate: true, production_to_desktop_review_gate: true, desktop_before_mobile_gate: true, android_s5_real_device_not_claimed_gate: true, public_launch_not_claimed_gate: true}, production_review_summary: {interaction_surface_count: 6, runtime_screen_mode: "player_runtime_command_interaction_screen", runtime_screen_gate: true, evidence_board_only: false, runtime_screen_layout: {drag_select: "visible marquee skin and selection feedback strip", queue_path: "queued waypoint path, rally chain, reservation, and cancel/repath strip"}, ui_skin_runtime_screen_mode: "player_runtime_production_hud_skin_screen", ui_skin_runtime_screen_gate: true, ui_skin_evidence_board_only: false, drag_select_skin_pixel_count: 9980, right_click_move_skin_pixel_count: 9980, attack_lock_skin_pixel_count: 9980, build_ghost_skin_pixel_count: 9980, queue_path_skin_pixel_count: 9700, scroll_minimap_skin_pixel_count: 9700}, desktop_review_summary: {screenshot_frame_count: 11, keyboard_event_count: 13, mouse_event_count: 15, mouse_slot_a_bytes: 41520}, artifact_manifest: [{label: "production_interaction_polish", path: "fixture", sha256: "fixture", bytes: 1}, {label: "production_interaction_polish_preview", path: "fixture", sha256: "fixture", bytes: 1}, {label: "desktop_playtest_review_packet", path: "fixture", sha256: "fixture", bytes: 1}, {label: "desktop_real_machine_readiness", path: "fixture", sha256: "fixture", bytes: 1}, {label: "live_window_screenshot_sequence", path: "fixture", sha256: "fixture", bytes: 1}, {label: "live_window_mouse_hit_test_sequence", path: "fixture", sha256: "fixture", bytes: 1}], no_credit_boundaries: {android_s5_real_device_claimed: false, public_launch_ready_claimed: false, production_ready_desktop_review_shipped: false, desktop_review_scope: "local_linux_desktop_x11_window_keyboard_mouse_with_production_interaction_polish"}}' >"$production_desktop_review_packet_json"
add_artifact_from_path native_bevy_classic_rts_production_desktop_review_packet "Native/Bevy classic RTS production desktop review packet" "$production_desktop_review_packet_json" release_review_input

full_screen_ui_replication_json="$TMP_DIR/bevy-classic-rts-full-screen-ui-replication.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1", status: "classic_rts_full_screen_ui_replication_green", green: true, preview_width: 1280, preview_height: 768, preview_format: "ppm_p3_rgb", replication_surface_count: 10, replication_surface_names: ["TITLE/CAMPAIGN ENTRY", "TACTICAL VIEWPORT", "MAP/MINIMAP CAMERA", "PRODUCTION HUD SKIN", "COMMAND INTERACTIONS", "BUILD + TECH TREE", "UNIT STATUS CARD", "ABILITY/COMBAT UI", "CAMPAIGN OUTCOME", "OPEN-WORLD HANDOFF"], source_contracts: {campaign_entry: "trillionnium_world_bevy_classic_rts_campaign_entry_v1", visual_fidelity: "trillionnium_world_bevy_classic_rts_visual_fidelity_v1", map_ui_modeling_readiness: "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1", production_ui_skin: "trillionnium_world_bevy_classic_rts_production_ui_skin_v1", production_interaction_polish: "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1", build_lifecycle: "trillionnium_world_bevy_classic_rts_build_lifecycle_v1", tech_tree: "trillionnium_world_bevy_classic_rts_tech_tree_v1", campaign_outcome_ui_readiness: "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1", combat_readability_pressure_readiness: "trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1"}, source_headline: {production_ui_runtime_screen_mode: "player_runtime_production_hud_skin_screen", production_ui_runtime_screen_gate: true, interaction_runtime_screen_mode: "player_runtime_command_interaction_screen", interaction_runtime_screen_gate: true, combat_readability_preview_count: 6, combat_readability_source_preview_count: 5}, runtime_screen_mode: "player_runtime_screen", runtime_screen_gate: true, evidence_board_only: false, runtime_screen_layout: {title_campaign_bar: "top-left campaign CTA strip", tactical_viewport: "single large scrollable map viewport", minimap_camera: "in-viewport minimap with live viewport rect", right_rail: "unit status, production, build tech, ability combat, campaign outcome", bottom_command_grid: "player command buttons, queue, and interaction feedback", handoff_panel: "open-world continuation and replay resume"}, screen_matrix_pixel_counts: {board: 381132, title_campaign: 8340, tactical_viewport: 8340, map_minimap: 7876, production_hud_skin: 7876, command_interaction: 7876, build_tech: 7876, combat_overlay: 7876, campaign_outcome: 7876}, full_screen_ui_pixel_counts: {player_first_full_screen_tactical_view_non_background: 112566, player_first_full_screen_tactical_view_frame: 10208, player_first_full_screen_status_strip: 12960}, title_campaign_gate: true, tactical_viewport_gate: true, map_minimap_gate: true, production_skin_gate: true, interaction_polish_gate: true, build_tech_gate: true, combat_ui_gate: true, campaign_outcome_gate: true, source_policy_gate: true, replication_preview_gate: true, source_preview_gate: true, player_first_full_screen_ui_surface_gate: true, full_screen_ui_replication_gate: true, internal_full_screen_ui_replication_claimed: true, external_evidence_ignored_for_current_replication_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false, warcraft_iii_asset_copied: false, openra_asset_copied: false, third_party_asset_copied: false} | .source_paths = (.source_paths // {campaign_entry:"fixture", visual_fidelity:"fixture", map_ui_modeling_readiness:"fixture", production_ui_skin:"fixture", production_interaction_polish:"fixture", build_lifecycle:"fixture", tech_tree:"fixture", campaign_outcome_ui_readiness:"fixture"}) | .replication_slot_ids = (.replication_slot_ids // ["title_campaign_shell", "match_viewport_hud", "map_minimap_camera", "production_hud_surfaces", "interaction_feedback", "build_tech_overlay", "unit_status_card", "ability_combat_overlay", "outcome_reward_panel", "handoff_replay_resume"]) | .replication_source_surfaces = (.replication_source_surfaces // .replication_surface_names) | .source_contract_count = (.source_contracts | keys | length) | .source_path_count = (.source_paths | keys | length) | .source_headline_field_count = (.source_headline | keys | length) | .runtime_screen_layout_count = (.runtime_screen_layout | keys | length) | .screen_matrix_pixel_count_field_count = (.screen_matrix_pixel_counts | keys | length) | .full_screen_ui_pixel_count_field_count = (.full_screen_ui_pixel_counts | keys | length) | .replication_surface_name_count = (.replication_surface_names | length) | .replication_slot_id_count = (.replication_slot_ids | length) | .replication_source_surface_count = (.replication_source_surfaces | length) | .gate_count = ([.title_campaign_gate, .tactical_viewport_gate, .map_minimap_gate, .production_skin_gate, .interaction_polish_gate, .build_tech_gate, .combat_ui_gate, .campaign_outcome_gate, .source_policy_gate, .replication_preview_gate, .runtime_screen_gate, .source_preview_gate, .player_first_full_screen_ui_surface_gate, .full_screen_ui_replication_gate] | length) | .passed_gate_count = ([.title_campaign_gate, .tactical_viewport_gate, .map_minimap_gate, .production_skin_gate, .interaction_polish_gate, .build_tech_gate, .combat_ui_gate, .campaign_outcome_gate, .source_policy_gate, .replication_preview_gate, .runtime_screen_gate, .source_preview_gate, .player_first_full_screen_ui_surface_gate, .full_screen_ui_replication_gate] | map(select(. == true)) | length) | .failed_gate_count = (.gate_count - .passed_gate_count)' >"$full_screen_ui_replication_json"
add_artifact_from_path native_bevy_classic_rts_full_screen_ui_replication "Native/Bevy classic RTS full screen/UI replication" "$full_screen_ui_replication_json" release_review_input
shell_meta_ui_replication_json="$TMP_DIR/bevy-classic-rts-shell-meta-ui-replication.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1", status: "classic_rts_shell_meta_ui_replication_green", green: true, preview_width: 1280, preview_height: 768, preview_format: "ppm_p3_rgb", shell_meta_surface_count: 12, shell_meta_surface_names: ["TITLE / ACCOUNT", "CHARACTER CREATE", "SESSION SLOT MENU", "SAVE SLOT FILE", "SAVE / LOAD CONFIRM", "LOAD / RESUME CTA", "SESSION RECOVERY", "PAUSE / RESUME", "SETTINGS", "INPUT HUD", "BUTTON HIT TEST", "FIRST-MINUTE HANDOFF"], source_contracts: {account_title_flow: "trillionnium_world_bevy_account_title_flow_v1", character_create: "trillionnium_world_bevy_character_create_v1", first_minute_onboarding: "trillionnium_world_bevy_first_minute_onboarding_v1", full_screen_ui_replication: "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1", input_telemetry_hud: "trillionnium_world_bevy_input_telemetry_hud_v1", pause_menu: "trillionnium_world_bevy_pause_menu_v1", session_load_resume: "trillionnium_world_bevy_session_load_resume_v1", session_recovery_ui: "trillionnium_world_bevy_session_recovery_ui_v1", session_save_slot: "trillionnium_world_bevy_session_save_slot_v1", session_slot_confirm: "trillionnium_world_bevy_session_slot_confirm_v1", session_slot_menu: "trillionnium_world_bevy_session_slot_menu_v1", settings_menu: "trillionnium_world_bevy_settings_menu_v1", title_menu: "trillionnium_world_bevy_title_menu_v1", visible_button_hit_test_map: "trillionnium_world_bevy_visible_button_hit_test_map_v1"}, runtime_screen_mode: "player_runtime_shell_meta_screen", runtime_screen_gate: true, player_first_shell_meta_screen_gate: true, evidence_board_only: false, runtime_screen_layout: {account_title_bar: "top account/login/continue CTA strip", character_card: "active player character creation result", session_slot_panel: "visible save slots with selected slot A", save_load_confirm: "confirm/save/load recovery actions", right_meta_rail: "pause, settings, input HUD, and hit-test cards", first_minute_handoff: "bottom create-to-continue gameplay route"}, shell_meta_pixel_counts: {account_title: 5872, board: 398212, button_hit_test: 5872, character_create: 5872, first_minute_handoff: 5872, highlight: 6336, input_hud: 5872, load_resume_cta: 5872, pause_resume: 5872, save_load_confirm: 5872, save_slot_file: 5872, session_recovery: 5872, session_slot_menu: 5872, settings: 5872}, shell_meta_player_first_pixel_counts: {player_first_shell_meta_surface_non_background: 500000, player_first_shell_meta_frame: 13000, player_first_shell_meta_account_bar: 32000, player_first_shell_meta_session_panel: 36000, player_first_shell_meta_right_rail: 32000, player_first_shell_meta_handoff_strip: 95000}, full_screen_ui_replication_gate: true, account_title_gate: true, title_menu_gate: true, character_create_gate: true, session_slot_menu_gate: true, session_save_slot_gate: true, session_slot_confirm_gate: true, session_load_resume_gate: true, session_recovery_gate: true, pause_menu_gate: true, settings_menu_gate: true, input_hud_gate: true, visible_hit_test_gate: true, first_minute_onboarding_gate: true, source_preview_gate: true, shell_meta_preview_gate: true, runtime_screen_gate: true, shell_meta_ui_replication_gate: true, no_external_boundary_gate: true, internal_shell_meta_ui_replication_claimed: true, external_evidence_ignored_for_current_replication_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false, warcraft_iii_asset_copied: false, openra_asset_copied: false, third_party_asset_copied: false} | .source_paths = (.source_paths // {full_screen_ui_replication:"fixture", shell_meta_ui_replication_slots:"fixture"}) | .source_headline = (.source_headline // {full_screen_surface_count:10, account_session_bound:true, title_slot_a_bytes:1024, character_name:"Mira", slot_menu_target_count:10, save_slot_bytes:1024, settings_volume:5, input_keyboard_events:10, onboarding_final_node:"league-coliseum"}) | .shell_meta_slot_ids = (.shell_meta_slot_ids // ["account_title_flow", "character_create", "session_slot_menu", "save_slot_file", "slot_confirm", "load_resume_overlay", "recovery_panel", "pause_menu", "settings_menu", "input_telemetry_hud", "hit_test_map", "onboarding_handoff"]) | .shell_meta_source_surfaces = (.shell_meta_source_surfaces // .shell_meta_surface_names) | .source_contract_count = (.source_contracts | keys | length) | .source_path_count = (.source_paths | keys | length) | .source_headline_field_count = (.source_headline | keys | length) | .runtime_screen_layout_count = (.runtime_screen_layout | keys | length) | .shell_meta_pixel_count_field_count = (.shell_meta_pixel_counts | keys | length) | .shell_meta_player_first_pixel_count_field_count = (.shell_meta_player_first_pixel_counts | keys | length) | .shell_meta_surface_name_count = (.shell_meta_surface_names | length) | .shell_meta_slot_id_count = (.shell_meta_slot_ids | length) | .shell_meta_source_surface_count = (.shell_meta_source_surfaces | length) | .gate_count = ([.full_screen_ui_replication_gate, .account_title_gate, .title_menu_gate, .character_create_gate, .session_slot_menu_gate, .session_save_slot_gate, .session_slot_confirm_gate, .session_load_resume_gate, .session_recovery_gate, .pause_menu_gate, .settings_menu_gate, .input_hud_gate, .visible_hit_test_gate, .first_minute_onboarding_gate, .no_external_boundary_gate, .shell_meta_preview_gate, .runtime_screen_gate, .player_first_shell_meta_screen_gate, .source_preview_gate, .shell_meta_ui_replication_gate] | length) | .passed_gate_count = ([.full_screen_ui_replication_gate, .account_title_gate, .title_menu_gate, .character_create_gate, .session_slot_menu_gate, .session_save_slot_gate, .session_slot_confirm_gate, .session_load_resume_gate, .session_recovery_gate, .pause_menu_gate, .settings_menu_gate, .input_hud_gate, .visible_hit_test_gate, .first_minute_onboarding_gate, .no_external_boundary_gate, .shell_meta_preview_gate, .runtime_screen_gate, .player_first_shell_meta_screen_gate, .source_preview_gate, .shell_meta_ui_replication_gate] | map(select(. == true)) | length) | .failed_gate_count = (.gate_count - .passed_gate_count)' >"$shell_meta_ui_replication_json"
add_artifact_from_path native_bevy_classic_rts_shell_meta_ui_replication "Native/Bevy classic RTS shell/meta UI replication" "$shell_meta_ui_replication_json" release_review_input
match_setup_ui_replication_json="$TMP_DIR/bevy-classic-rts-match-setup-ui-replication.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1", status: "classic_rts_match_setup_ui_replication_green", green: true, preview_width: 1280, preview_height: 768, preview_format: "ppm_p3_rgb", setup_surface_count: 10, setup_surface_names: ["CAMPAIGN ACTIONS", "MAP SELECT", "FACTION SELECT", "SPAWN SLOTS", "RESOURCE RULES", "BOT / DIFFICULTY", "VICTORY CONDITIONS", "MINIMAP PREVIEW", "START READY", "NO-EXTERNAL BOUNDARY"], source_contracts: {shell_meta_ui_replication: "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1", campaign_entry: "trillionnium_world_bevy_classic_rts_campaign_entry_v1", first_contact_basin_spec: "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1", map_ui_modeling_readiness: "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1", tech_tree: "trillionnium_world_bevy_classic_rts_tech_tree_v1"}, setup_pixel_counts: {board: 227743, map_select: 4479, faction_select: 7010, start_ready: 13808}, match_setup_player_first_pixel_counts: {player_first_match_setup_map_non_background: 181305, player_first_match_setup_map_frame: 12648, player_first_match_setup_status_strip: 18896, player_first_match_setup_rules_rail: 104799, player_first_match_setup_ready_strip: 40404}, source_headline: {shell_meta_surface_count: 12, campaign_input_action_count: 73, map_id: "first_contact_basin", map_spawn_count: 4, map_actor_count: 39, map_ui_preview_count: 6, faction_id: "mirror_guard", tech_state: "unlocked:relay_guard"}, runtime_screen_mode: "player_runtime_match_setup_screen", runtime_screen_gate: true, evidence_board_only: false, runtime_screen_layout: {campaign_actions: "top-left start/continue/replay strip", map_select: "large First Contact Basin tactical setup viewport", minimap_preview: "in-map camera fog and spawn-lane preview", right_rules_rail: "faction, resources, bot, victory, and boundary confirmation rail", start_ready: "bottom player launch strip with local Rust/Bevy ready state"}, shell_meta_gate: true, campaign_entry_gate: true, map_spec_gate: true, map_ui_gate: true, faction_gate: true, no_external_boundary_gate: true, setup_preview_gate: true, source_preview_gate: true, player_first_match_setup_screen_gate: true, match_setup_ui_replication_gate: true, internal_match_setup_ui_replication_claimed: true, external_evidence_ignored_for_current_replication_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false, warcraft_iii_asset_copied: false, openra_asset_copied: false, third_party_asset_copied: false} | .source_paths = (.source_paths // {shell_meta_ui_replication:"fixture", campaign_entry:"fixture", first_contact_basin_spec:"fixture"}) | .setup_slot_ids = (.setup_slot_ids // ["campaign_start_continue_replay", "first_contact_basin", "mirror_guard", "four_spawn_lanes", "flux_beacons_expansions", "local_bot_fixture", "beacon_extract", "camera_fog_spawn", "ready_to_start", "no_s5_no_public"]) | .setup_source_surfaces = (.setup_source_surfaces // .setup_surface_names) | .source_contract_count = (.source_contracts | keys | length) | .source_path_count = (.source_paths | keys | length) | .source_headline_field_count = (.source_headline | keys | length) | .runtime_screen_layout_count = (.runtime_screen_layout | keys | length) | .setup_pixel_count_field_count = (.setup_pixel_counts | keys | length) | .match_setup_player_first_pixel_count_field_count = (.match_setup_player_first_pixel_counts | keys | length) | .setup_surface_name_count = (.setup_surface_names | length) | .setup_slot_id_count = (.setup_slot_ids | length) | .setup_source_surface_count = (.setup_source_surfaces | length) | .gate_count = ([.shell_meta_gate, .campaign_entry_gate, .map_spec_gate, .map_ui_gate, .faction_gate, .no_external_boundary_gate, .setup_preview_gate, .runtime_screen_gate, .source_preview_gate, .player_first_match_setup_screen_gate, .match_setup_ui_replication_gate] | length) | .passed_gate_count = ([.shell_meta_gate, .campaign_entry_gate, .map_spec_gate, .map_ui_gate, .faction_gate, .no_external_boundary_gate, .setup_preview_gate, .runtime_screen_gate, .source_preview_gate, .player_first_match_setup_screen_gate, .match_setup_ui_replication_gate] | map(select(. == true)) | length) | .failed_gate_count = (.gate_count - .passed_gate_count)' >"$match_setup_ui_replication_json"
add_artifact_from_path native_bevy_classic_rts_match_setup_ui_replication "Native/Bevy classic RTS match setup UI replication" "$match_setup_ui_replication_json" release_review_input
campaign_outcome_ui_readiness_json="$TMP_DIR/bevy-classic-rts-campaign-outcome-ui-readiness.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1", status: "classic_rts_campaign_outcome_ui_readiness_green", green: true, preview_count: 5, runtime_screen_mode: "player_runtime_campaign_outcome_screen", runtime_screen_gate: true, evidence_board_only: false, runtime_screen_layout: {outcome_flow_lane: "title to victory to aftermath to open-world resume", objective_result_panel: "relay beacon extracted victory and defeat-risk summary", base_assault_panel: "enemy barracks breach resolution", aftermath_rewards_panel: "growth, rewards, and secure expansion next action", open_world_resume_panel: "league-coliseum arena_outdoor resume state"}, campaign_outcome_ui_readiness_gate: true, player_first_campaign_outcome_screen_gate: true, source_contracts: {first_minute_readiness: "trillionnium_world_bevy_classic_rts_first_minute_readiness_v1", objective_victory_loop: "trillionnium_world_bevy_classic_rts_objective_victory_loop_v1", base_assault_resolution: "trillionnium_world_bevy_classic_rts_base_assault_resolution_v1", battle_aftermath: "trillionnium_world_bevy_classic_rts_battle_aftermath_v1", open_world_after_action: "trillionnium_world_bevy_classic_rts_open_world_after_action_v1"}, first_minute_gate: true, objective_victory_gate: true, base_assault_gate: true, battle_aftermath_gate: true, open_world_return_gate: true, native_boundary_gate: true, preview_gate: true, campaign_flow: ["TITLE campaign entry", "objective claim/extract victory", "battle aftermath rewards", "open-world route resume"], first_minute_summary: {input_action_count: 73, final_room: "league-coliseum", final_objective_status: "open_world_after_action_ready", runtime_screen_mode: "player_runtime_first_minute_readiness_screen", runtime_screen_gate: true, evidence_board_only: false, player_first_first_minute_screen_gate: true, first_minute_pixel_counts: {player_first_campaign_view_non_background: 928800, player_first_campaign_route_rail: 372360}}, victory_summary: {accepted_input_count: 6, final_objective_capture_percent: 100, final_objective_result_state: "victory:relay_beacon_extracted", final_defeat_risk_percent: 4, non_background_pixels: 1382400, victory_pixel_count: 3357, extraction_pixel_count: 451}, base_assault_summary: {accepted_input_count: 9, final_base_breach_percent: 100, final_base_assault_result_state: "breached:enemy_barracks", non_background_pixels: 2073600, breach_pixel_count: 2092, assault_path_pixel_count: 792}, aftermath_summary: {accepted_input_count: 12, final_match_result_state: "victory_ready:secure_expansion", final_growth_level: 2, final_next_action_ids: ["secure_expansion"], runtime_screen_mode: "player_runtime_battle_aftermath_screen", runtime_screen_gate: true, evidence_board_only: false, player_first_battle_aftermath_screen_gate: true, battle_aftermath_pixel_counts: {player_first_battle_view_non_background: 526242, player_first_battle_outcome_panel: 223136}}, open_world_summary: {accepted_input_count: 3, final_current_room_id: "league-coliseum", final_map_scene: "arena_outdoor", final_open_world_handoff_state: "resumed:league-coliseum", runtime_screen_mode: "player_runtime_open_world_after_action_screen", runtime_screen_gate: true, evidence_board_only: false, player_first_open_world_after_action_screen_gate: true, open_world_after_action_pixel_counts: {player_first_open_world_view_non_background: 526242, player_first_open_world_view_frame: 11344, player_first_open_world_status_strip: 93012, player_first_open_world_route_panel: 223136, player_first_open_world_timeline: 29184}}, internal_campaign_outcome_ui_readiness_claimed: true, external_evidence_ignored_for_current_outcome_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false} | .preview_paths = (.preview_paths // {first_minute_readiness:"first-minute-readiness.ppm", objective_victory_loop:"objective-victory-loop.ppm", base_assault_resolution:"base-assault-resolution.ppm", battle_aftermath:"battle-aftermath.ppm", open_world_after_action:"open-world-after-action.ppm"}) | .source_contract_count = (.source_contracts | keys | length) | .preview_path_count = (.preview_paths | keys | length) | .runtime_screen_layout_count = (.runtime_screen_layout | keys | length) | .campaign_flow_count = (.campaign_flow | length) | .first_minute_summary_field_count = (.first_minute_summary | keys | length) | .victory_summary_field_count = (.victory_summary | keys | length) | .base_assault_summary_field_count = (.base_assault_summary | keys | length) | .aftermath_summary_field_count = (.aftermath_summary | keys | length) | .open_world_summary_field_count = (.open_world_summary | keys | length) | .gate_count = ([.first_minute_gate, .objective_victory_gate, .base_assault_gate, .battle_aftermath_gate, .open_world_return_gate, .player_first_campaign_outcome_screen_gate, .native_boundary_gate, .preview_gate, .runtime_screen_gate, .campaign_outcome_ui_readiness_gate] | length) | .passed_gate_count = ([.first_minute_gate, .objective_victory_gate, .base_assault_gate, .battle_aftermath_gate, .open_world_return_gate, .player_first_campaign_outcome_screen_gate, .native_boundary_gate, .preview_gate, .runtime_screen_gate, .campaign_outcome_ui_readiness_gate] | map(select(. == true)) | length) | .failed_gate_count = (.gate_count - .passed_gate_count)' >"$campaign_outcome_ui_readiness_json"
add_artifact_from_path native_bevy_classic_rts_campaign_outcome_ui_readiness "Native/Bevy classic RTS campaign outcome UI readiness" "$campaign_outcome_ui_readiness_json" release_review_input

in_match_hud_state_replication_json="$TMP_DIR/bevy-classic-rts-in-match-hud-state-replication.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
  status: "classic_rts_in_match_hud_state_replication_green",
  green: true,
  preview_width: 1280,
  preview_height: 768,
  preview_format: "ppm_p3_rgb",
  source_contracts: {
    production_ui_skin: "trillionnium_world_bevy_classic_rts_production_ui_skin_v1",
    production_interaction_polish: "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1",
    selection_minimap: "trillionnium_world_bevy_classic_rts_selection_minimap_v1",
    unit_status_portrait: "trillionnium_world_bevy_classic_rts_unit_status_portrait_v1",
    selection_command_feedback: "trillionnium_world_bevy_classic_rts_selection_command_feedback_v1",
    ability_tooltip_telegraph: "trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1",
    camera_minimap_sync: "trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1",
    command_queue_path_preview: "trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1",
    full_screen_ui_replication: "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1",
    match_setup_ui_replication: "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
    campaign_outcome_ui_readiness: "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1"
  },
  runtime_screen_mode: "player_runtime_in_match_hud_screen",
  runtime_screen_gate: true,
  evidence_board_only: false,
  runtime_screen_layout: {
    tactical_viewport: "single in-match First Contact Basin tactical viewport",
    top_resource_strip: "top resources, supply, and pressure readout",
    left_selection_panel: "selected group 1 unit and health card stack",
    right_production_ability_rail: "production, build, ability, cooldown, and alert panels",
    minimap_panel: "live minimap, fog, visibility, and objective pressure panel",
    bottom_command_grid: "move/train/build/attack command grid and queue",
    objective_alert_lane: "bottom objective, combat alert, and no-external boundary lane"
  },
  hud_surface_count: 8,
  hud_surface_names: ["RESOURCES", "SELECTION", "COMMAND GRID", "MINIMAP", "PRODUCTION", "ABILITIES", "COMBAT ALERTS", "OBJECTIVE"],
  selected_unit_ids: ["trnm.worker", "trnm.horizon.scout", "trnm.forge.warden", "trnm.flux.relay"],
  active_control_group_ids: ["1", "2"],
  command_queue: ["select_group_1", "move:16,9", "train:trnm.worker", "build:trnm.flux.relay", "attack:trnm.flux.beacon"],
  production_queue: ["train:guard", "train:worker", "upgrade:signal_blade"],
  build_queue: ["build:watch_tower", "upgrade:training_hall"],
  resource_spend_log: ["spent:140g:30l:guard", "queued:210g:60l:upgrade"],
  ability_command_ids: ["worker", "scout", "warden", "relay", "core", "signal"],
  combat_event_log: ["guard_attack_windup", "worker_carry_supply", "creep_counter_swing", "secure_beacon:16,9"],
  visible_tile_ids: ["4,4", "5,4", "6,4", "7,4", "8,4", "6,5", "7,5"],
  fogged_tile_ids: ["0,0", "1,0", "10,0", "11,0", "0,7", "11,7"],
  training_progress_percent: 76,
  build_progress_percent: 58,
  target_health_percent: 38,
  target_armor_percent: 35,
  visibility_percent: 74,
  enemy_pressure_warning_percent: 43,
  army_supply_used: 9,
  army_supply_cap: 18,
  hud_pixel_counts: {non_background: 951961, resources: 1240, selection: 1240, command_grid: 1240, minimap: 1240, production: 1240, abilities: 1240, combat_alerts: 1167, objective: 1240, highlight: 957},
  player_first_in_match_hud_screen_gate: true,
  in_match_hud_player_first_pixel_counts: {player_first_in_match_hud_view_non_background: 541917, player_first_in_match_hud_view_frame: 15370, player_first_in_match_hud_top_status_strip: 49689, player_first_in_match_hud_surface_cards: 50775, player_first_in_match_hud_right_rail_non_background: 133712, player_first_in_match_hud_bottom_command_lane: 67122, player_first_in_match_hud_control_colors: 9847},
  selection_gate: true,
  command_gate: true,
  resource_gate: true,
  production_gate: true,
  ability_gate: true,
  combat_alert_gate: true,
  minimap_objective_gate: true,
  native_client_boundary_gate: true,
  preview_gate: true,
  runtime_screen_gate: true,
  in_match_hud_state_replication_gate: true,
  internal_in_match_hud_state_replication_claimed: true,
  external_evidence_ignored_for_current_replication_pass: true,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$in_match_hud_state_replication_json"
in_match_hud_state_replication_json_tmp="$in_match_hud_state_replication_json.tmp"
jq '
  .source_contract_count = (.source_contracts | keys | length)
  | .selected_unit_count = (.selected_unit_ids | length)
  | .active_control_group_count = (.active_control_group_ids | length)
  | .command_queue_count = (.command_queue | length)
  | .production_queue_count = (.production_queue | length)
  | .build_queue_count = (.build_queue | length)
  | .resource_spend_log_count = (.resource_spend_log | length)
  | .ability_command_count = (.ability_command_ids | length)
  | .combat_event_log_count = (.combat_event_log | length)
  | .visible_tile_count = (.visible_tile_ids | length)
  | .fogged_tile_count = (.fogged_tile_ids | length)
' "$in_match_hud_state_replication_json" >"$in_match_hud_state_replication_json_tmp"
mv "$in_match_hud_state_replication_json_tmp" "$in_match_hud_state_replication_json"
add_artifact_from_path native_bevy_classic_rts_in_match_hud_state_replication "Native/Bevy classic RTS in-match HUD/state replication" "$in_match_hud_state_replication_json" release_review_input


session_state_continuity_json="$TMP_DIR/bevy-classic-rts-session-state-continuity.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_session_state_continuity_v1",
  status: "classic_rts_session_state_continuity_green",
  green: true,
  preview_width: 1600,
  preview_height: 900,
  preview_format: "ppm_p3_rgb",
  source_contracts: {
    shell_meta_ui_replication: "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1",
    session_slot_confirm: "trillionnium_world_bevy_session_slot_confirm_v1",
    session_load_resume: "trillionnium_world_bevy_session_load_resume_v1",
    session_recovery_ui: "trillionnium_world_bevy_session_recovery_ui_v1",
    match_setup_ui_replication: "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
    in_match_hud_state_replication: "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
    campaign_outcome_ui_readiness: "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1",
    campaign_ui_continuity: "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1"
  },
  runtime_screen_mode: "player_runtime_session_resume_screen",
  runtime_screen_gate: true,
  evidence_board_only: false,
  runtime_screen_layout: {
    resume_chain_lane: "single visible save/load/continue chain from match setup into restored play",
    primary_tactical_viewport: "large restored tactical state with save-resume rail",
    pre_match_snapshot: "match setup map/faction/rules snapshot",
    slot_resume_controls: "selected Slot A write, load lock, and continue unlock controls",
    hud_restore_panel: "restored in-match resources, selection, command, minimap, and objective state",
    outcome_resume_panel: "campaign outcome rewards and open-world league-coliseum route resume",
    recovery_guard_panel: "session recovery and no-external boundary guard"
  },
  rts_evidence_session_state_continuity_review_contract: "trnm_rts_evidence_session_state_continuity_review_v1",
  rts_evidence_session_state_continuity_review: {
    contract_version: "trnm_rts_evidence_session_state_continuity_review_v1",
    green: true,
    shell_meta_contract: "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1",
    session_slot_confirm_contract: "trillionnium_world_bevy_session_slot_confirm_v1",
    session_load_resume_contract: "trillionnium_world_bevy_session_load_resume_v1",
    session_recovery_contract: "trillionnium_world_bevy_session_recovery_ui_v1",
    match_setup_contract: "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
    hud_contract: "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
    campaign_outcome_contract: "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1",
    campaign_continuity_contract: "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1",
    preview_width: 1600,
    preview_height: 900,
    state_continuity_surface_count: 8,
    shell_meta_surface_count: 12,
    confirmed_slot_a_bytes: 46253,
    load_resume_slot_a_bytes: 46253,
    load_resume_final_objective_status: "first_playable_loop_complete",
    match_setup_map_id: "first_contact_basin",
    hud_surface_count: 8,
    hud_army_supply_used: 9,
    campaign_outcome_open_world_state: "resumed:league-coliseum",
    campaign_continuity_restored_room_id: "league-coliseum",
    shell_meta_gate: true,
    session_slot_confirm_gate: true,
    session_load_resume_gate: true,
    session_recovery_gate: true,
    match_setup_gate: true,
    hud_restore_gate: true,
    campaign_outcome_gate: true,
    campaign_continuity_gate: true,
    surface_chain_gate: true,
    state_continuity_chain_gate: true,
    native_client_boundary_gate: true,
    preview_gate: true,
    player_first_session_resume_screen_gate: true,
    source_preview_gate: true,
    runtime_screen_gate: true,
    session_state_continuity_gate: true,
    input_path: "trnm-world-bevy session-state continuity source JSON and pixel counts -> trnm-rts-evidence session-state continuity review",
    evidence_path: "trnm-rts-evidence session_state_continuity_review -> Bevy session-state continuity packet artifact",
    source_of_truth: "The RTS evidence crate reviews save-slot confirmation, load-resume lock/continue, recovery guard, match setup, restored HUD, campaign outcome, campaign continuity, source preview readiness, native-client no-credit boundaries, and the player-first session resume screen before trnm-world-bevy includes the session-state continuity artifact in release-review evidence."
  },
  rts_evidence_session_state_continuity_review_gate: true,
  state_continuity_surface_count: 8,
  state_continuity_surface_names: ["MATCH SETUP SNAPSHOT", "SESSION SLOT WRITE", "LOAD RESUME LOCK", "CONTINUE UNLOCK", "IN-MATCH HUD RESTORE", "OUTCOME REWARD STATE", "OPEN-WORLD RESUME", "RECOVERY UI GUARD"],
  resume_chain: ["match_setup_saved", "slot_a_written", "load_resume_locked", "continue_unlocked", "in_match_hud_restored", "campaign_outcome_saved", "open_world_resumed"],
  source_headline: {
    shell_meta_runtime_screen_mode: "player_runtime_shell_meta_screen",
    match_setup_runtime_screen_mode: "player_runtime_match_setup_screen",
    hud_runtime_screen_mode: "player_runtime_in_match_hud_screen",
    load_resume_final_objective_status: "first_playable_loop_complete",
    campaign_outcome_open_world_state: "resumed:league-coliseum",
    campaign_continuity_restored_room_id: "league-coliseum",
    load_resume_slot_a_bytes: 46253
  },
  state_continuity_pixel_counts: {non_background: 1440000, in_match_hud_restore: 12000, open_world_resume: 12000, player_first_resume_view_non_background: 420000, player_first_resume_view_frame: 13504, player_first_resume_status_strip: 23320, player_first_resume_stage_rail: 190000},
  shell_meta_gate: true,
  session_slot_confirm_gate: true,
  session_load_resume_gate: true,
  session_recovery_gate: true,
  match_setup_gate: true,
  hud_restore_gate: true,
  campaign_outcome_gate: true,
  campaign_continuity_gate: true,
  state_continuity_chain_gate: true,
  native_client_boundary_gate: true,
  preview_gate: true,
  player_first_session_resume_screen_gate: true,
  source_preview_gate: true,
  runtime_screen_gate: true,
  session_state_continuity_gate: true,
  internal_session_state_continuity_claimed: true,
  external_evidence_ignored_for_current_replication_pass: true,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$session_state_continuity_json"
add_artifact_from_path native_bevy_classic_rts_session_state_continuity "Native/Bevy classic RTS session state continuity" "$session_state_continuity_json" release_review_input
combat_readability_pressure_readiness_json="$TMP_DIR/bevy-classic-rts-combat-readability-pressure-readiness.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1",
  status: "classic_rts_combat_readability_pressure_readiness_green",
  green: true,
  preview_count: 6,
  source_preview_count: 5,
  player_screen_path: "/tmp/combat-pressure-screen.ppm",
  player_screen_format: "ppm_p3_rgb",
  player_screen_width: 1280,
  player_screen_height: 768,
  runtime_screen_mode: "player_runtime_combat_pressure_screen",
  runtime_screen_gate: true,
  evidence_board_only: false,
  runtime_screen_layout: {unit_status_panel: "selected unit portrait, bars, role, and queue badges", command_feedback_lane: "marquee, attack, error, and acknowledgment feedback", ability_telegraph_panel: "tooltip, range, cooldown, queue, and warning overlays", depth_readability_view: "foreground, behind-building, mask, and target priority cues", pressure_panel: "central keep shield, guard, siege line, and defeat-risk feedback", combat_tactical_viewport: "large central-keep combat pressure tactical viewport", right_pressure_rail: "unit, command, ability, depth, and pressure feedback rail", bottom_command_lane: "player combat commands with attack, ability, hold, break, and retreat states"},
  combat_readability_pressure_readiness_gate: true,
  player_first_combat_pressure_screen_gate: true,
  combat_pressure_pixel_counts: {player_first_combat_pressure_view_non_background: 248151, player_first_combat_pressure_view_frame: 13752, player_first_combat_pressure_status_strip: 26777, player_first_combat_pressure_rail: 123537, player_first_combat_pressure_command_lane: 65616, player_first_combat_pressure_alert: 20927},
  source_contracts: {
    unit_status_portrait: "trillionnium_world_bevy_classic_rts_unit_status_portrait_v1",
    selection_command_feedback: "trillionnium_world_bevy_classic_rts_selection_command_feedback_v1",
    ability_tooltip_telegraph: "trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1",
    depth_readability: "trillionnium_world_bevy_classic_rts_depth_readability_v1",
    central_keep_pressure: "trillionnium_world_bevy_classic_rts_central_keep_pressure_v1"
  },
  unit_status_gate: true,
  command_feedback_gate: true,
  ability_telegraph_gate: true,
  depth_readability_gate: true,
  pressure_feedback_gate: true,
  source_policy_gate: true,
  preview_gate: true,
  unit_status_summary: {portrait_frame_pixel_count: 15339, health_bar_pixel_count: 2612, mana_bar_pixel_count: 2928, role_badge_pixel_count: 10934},
  command_feedback_summary: {marquee_pixel_count: 4760, attack_pixel_count: 1805, error_pixel_count: 7010, ack_pixel_count: 6027},
  ability_telegraph_summary: {accepted_input_count: 6, tooltip_pixel_count: 10466, range_pixel_count: 5036, warning_pixel_count: 6387},
  depth_summary: {foreground_pixel_count: 11000, behind_pixel_count: 1859, building_mask_pixel_count: 2635, target_priority_pixel_count: 1512},
  pressure_summary: {accepted_input_count: 40, final_defeat_risk_percent: 42, final_target_health_percent: 58, final_target_shield_percent: 24, final_central_keep_state: "pressure_locked:central_keep", final_next_action_ids: ["secure_expansion", "rebuild_forward_lodge", "scout_second_base", "enter_inner_lane", "press_central_keep", "break_central_keep"]},
  internal_combat_readability_pressure_readiness_claimed: true,
  external_evidence_ignored_for_current_combat_readability_pass: true,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
} | .preview_paths = (.preview_paths // {unit_status_portrait:"unit-status-portrait.ppm", selection_command_feedback:"selection-command-feedback.ppm", ability_tooltip_telegraph:"ability-tooltip-telegraph.ppm", depth_readability:"depth-readability.ppm", central_keep_pressure:"central-keep-pressure.ppm", combat_pressure_screen:"combat-pressure-screen.ppm"}) | .source_contract_count = (.source_contracts | keys | length) | .preview_path_count = (.preview_paths | keys | length) | .runtime_screen_layout_count = (.runtime_screen_layout | keys | length) | .combat_pressure_pixel_count_field_count = (.combat_pressure_pixel_counts | keys | length) | .unit_status_summary_field_count = (.unit_status_summary | keys | length) | .command_feedback_summary_field_count = (.command_feedback_summary | keys | length) | .ability_telegraph_summary_field_count = (.ability_telegraph_summary | keys | length) | .depth_summary_field_count = (.depth_summary | keys | length) | .pressure_summary_field_count = (.pressure_summary | keys | length) | .gate_count = ([.unit_status_gate, .command_feedback_gate, .ability_telegraph_gate, .depth_readability_gate, .pressure_feedback_gate, .source_policy_gate, .preview_gate, .runtime_screen_gate, .player_first_combat_pressure_screen_gate, .combat_readability_pressure_readiness_gate] | length) | .passed_gate_count = ([.unit_status_gate, .command_feedback_gate, .ability_telegraph_gate, .depth_readability_gate, .pressure_feedback_gate, .source_policy_gate, .preview_gate, .runtime_screen_gate, .player_first_combat_pressure_screen_gate, .combat_readability_pressure_readiness_gate] | map(select(. == true)) | length) | .failed_gate_count = (.gate_count - .passed_gate_count)' >"$combat_readability_pressure_readiness_json"
add_artifact_from_path native_bevy_classic_rts_combat_readability_pressure_readiness "Native/Bevy classic RTS combat readability/pressure readiness" "$combat_readability_pressure_readiness_json" release_review_input

keyboard_replay_json="$TMP_DIR/bevy-build-branch-title-route-all-branch-keyboard-replay.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1",
  status: "keyboard_replay_green",
  build_branch_title_route_all_branch_keyboard_loop_contract: "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_loop_v1",
  green: true,
  all_branch_keyboard_loop_contract_green: true,
  branch_count: 3,
  all_branch_replay_gate: true,
  actor_id: "local-player",
  source_of_truth: "Recorded all-branch title-route keyboard event sequences are replayed on fresh Bevy runtime apps through ButtonInput<KeyCode> and must reproduce the same event signatures plus final branch states.",
  replayed_stat_ids: ["force", "agility", "craft"],
  replay_results: {
    force: {
      green: true, stat_id: "force", title_id: "title-force-gate-warden", reward_item_id: "force-mastery-signet",
      recorded_branch_green: true, recorded_sequence_parse_gate: true, recorded_sequence_path_gate: true,
      recorded_sequence_count: 10, replay_event_count: 10, replay_sequence_signature_match: true, final_runtime_match: true,
      recorded_sequence: [
        {key: "Enter", stage: "equip_title"}, {key: "Enter", stage: "followup_route_1"}, {key: "NumpadEnter", stage: "followup_route_2"}, {key: "Enter", stage: "complete_followup"}, {key: "Enter", stage: "mastery_route_1"}, {key: "KeyJ", stage: "combat_prereq_1"}, {key: "KeyJ", stage: "combat_prereq_2"}, {key: "KeyJ", stage: "combat_prereq_3"}, {key: "KeyJ", stage: "combat_prereq_4"}, {key: "Enter", stage: "complete_mastery"}
      ],
      replay_events: [{signature_match: true, recorded_signature: {input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"}, replay_signature: {input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"}}],
      expected_final_runtime: {current_room_id: "league-coliseum", combat_result_state: "victory", active_build_title_id: "title-force-gate-warden", active_build_title_effect: "arena_gate_reputation_anchor", route_director_task_id: "task-force-mastery-guard-trial", inventory_items: ["force-mastery-signet"], completed_task_ids: ["task-force-mastery-guard-trial"]},
      replay_final_runtime: {current_room_id: "league-coliseum", combat_result_state: "victory", active_build_title_id: "title-force-gate-warden", active_build_title_effect: "arena_gate_reputation_anchor", route_director_task_id: "task-force-mastery-guard-trial", inventory_items: ["force-mastery-signet"], completed_task_ids: ["task-force-mastery-guard-trial"]}
    },
    agility: {
      green: true, stat_id: "agility", title_id: "title-agility-relay-runner", reward_item_id: "agility-mastery-signet",
      recorded_branch_green: true, recorded_sequence_parse_gate: true, recorded_sequence_path_gate: true,
      recorded_sequence_count: 8, replay_event_count: 8, replay_sequence_signature_match: true, final_runtime_match: true,
      recorded_sequence: [
        {key: "Enter", stage: "equip_title"}, {key: "Enter", stage: "followup_route_1"}, {key: "NumpadEnter", stage: "followup_route_2"}, {key: "Enter", stage: "followup_route_3"}, {key: "Enter", stage: "complete_followup"}, {key: "Enter", stage: "mastery_route_1"}, {key: "NumpadEnter", stage: "mastery_route_2"}, {key: "Enter", stage: "complete_mastery"}
      ],
      replay_events: [{signature_match: true, recorded_signature: {input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"}, replay_signature: {input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"}}],
      expected_final_runtime: {current_room_id: "mirror-city-square", combat_result_state: "not_started", active_build_title_id: "title-agility-relay-runner", active_build_title_effect: "relay_route_priority_anchor", route_director_task_id: "task-agility-mastery-shortcut-run", inventory_items: ["agility-mastery-signet"], completed_task_ids: ["task-agility-mastery-shortcut-run"]},
      replay_final_runtime: {current_room_id: "mirror-city-square", combat_result_state: "not_started", active_build_title_id: "title-agility-relay-runner", active_build_title_effect: "relay_route_priority_anchor", route_director_task_id: "task-agility-mastery-shortcut-run", inventory_items: ["agility-mastery-signet"], completed_task_ids: ["task-agility-mastery-shortcut-run"]}
    },
    craft: {
      green: true, stat_id: "craft", title_id: "title-craft-forge-master", reward_item_id: "craft-mastery-signet",
      recorded_branch_green: true, recorded_sequence_parse_gate: true, recorded_sequence_path_gate: true,
      recorded_sequence_count: 7, replay_event_count: 7, replay_sequence_signature_match: true, final_runtime_match: true,
      recorded_sequence: [
        {key: "Enter", stage: "equip_title"}, {key: "Enter", stage: "followup_route_1"}, {key: "NumpadEnter", stage: "followup_route_2"}, {key: "Enter", stage: "followup_route_3"}, {key: "Enter", stage: "complete_followup"}, {key: "Enter", stage: "mastery_route_1"}, {key: "Enter", stage: "complete_mastery"}
      ],
      replay_events: [{signature_match: true, recorded_signature: {input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"}, replay_signature: {input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"}}],
      expected_final_runtime: {current_room_id: "forge-workbench", combat_result_state: "not_started", active_build_title_id: "title-craft-forge-master", active_build_title_effect: "forge_client_trust_anchor", route_director_task_id: "task-craft-mastery-client-order", inventory_items: ["craft-mastery-signet"], completed_task_ids: ["task-craft-mastery-client-order"]},
      replay_final_runtime: {current_room_id: "forge-workbench", combat_result_state: "not_started", active_build_title_id: "title-craft-forge-master", active_build_title_effect: "forge_client_trust_anchor", route_director_task_id: "task-craft-mastery-client-order", inventory_items: ["craft-mastery-signet"], completed_task_ids: ["task-craft-mastery-client-order"]}
    }
  },
  android_s5_real_device_claimed: false,
  external_evidence_ignored_for_current_keyboard_replay_pass: true,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$keyboard_replay_json"
keyboard_replay_counts_json="$TMP_DIR/bevy-build-branch-title-route-all-branch-keyboard-replay-counted.json"
jq '
  def branch_objective_status($branch):
    $branch.replay_final_runtime.objective_status
    // ("build_mastery_challenge_completed:" + $branch.stat_id + ":" + $branch.replay_final_runtime.route_director_task_id);
  def branch_summary($branch):
    {
      recorded_sequence_count: $branch.recorded_sequence_count,
      replay_event_count: $branch.replay_event_count,
      final_runtime_match: $branch.final_runtime_match,
      final_objective_status: branch_objective_status($branch),
      combat_result_state: $branch.replay_final_runtime.combat_result_state,
      current_room_id: $branch.replay_final_runtime.current_room_id,
      reward_item_id: $branch.reward_item_id
    };
  .ready_for_release_review = true
  | .proof_scope = "host_side_bevy_runtime_replay_not_android_real_device"
  | .green_replay_result_count = ([.replay_results[] | select(.green == true)] | length)
  | .recorded_branch_green_count = ([.replay_results[] | select(.recorded_branch_green == true)] | length)
  | .recorded_sequence_total_count = ([.replay_results[].recorded_sequence_count] | add)
  | .replay_event_total_count = ([.replay_results[].replay_event_count] | add)
  | .final_runtime_match_count = ([.replay_results[] | select(.final_runtime_match == true)] | length)
  | .combat_victory_branch_count = ([.replay_results[] | select(.replay_final_runtime.combat_result_state == "victory")] | length)
  | .reward_item_count = ([.replay_results[].reward_item_id] | length)
  | .branches = {
      force: branch_summary(.replay_results.force),
      agility: branch_summary(.replay_results.agility),
      craft: branch_summary(.replay_results.craft)
    }
' "$keyboard_replay_json" >"$keyboard_replay_counts_json"
mv "$keyboard_replay_counts_json" "$keyboard_replay_json"
add_artifact_from_path native_bevy_keyboard_replay "Native/Bevy keyboard replay" "$keyboard_replay_json" release_review_input

classic_animation_preview_json="$TMP_DIR/bevy-classic-animation-preview.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_animation_preview_v1",
  status: "classic_animation_preview_green",
  green: true,
  ready_for_release_review: true,
  preview_format: "ppm_p3_rgb",
  preview_width: 640,
  preview_height: 448,
  preview_bytes: 2596553,
  clip_count: 4,
  rendered_clip_count: 4,
  rendered_frame_slot_count: 15,
  clip_summary_count: 4,
  unique_clip_action_count: 4,
  unique_clip_actor_count: 3,
  gate_count: 8,
  passed_gate_count: 8,
  failed_gate_count: 0,
  unique_color_count: 35,
  non_background_pixels: 80421,
  label_pixel_count: 2062,
  loaded_from_manifest: true,
  atlas_parse_gate: true,
  clip_count_gate: true,
  action_coverage_gate: true,
  fps_gate: true,
  all_clip_refs_valid: true,
  rendered_clip_gate: true,
  preview_sheet_gate: true,
  label_gate: true,
  source_of_truth: "Classic animation preview expands manifest actor clips into visible sprite strips through the same PPM atlas blitter used by the low-spec playtest renderer.",
  clip_summaries: [
    {actor_id: "player", action: "walk", clip_id: "player_cardinal_walk_cycle", fps: 8, frame_count: 8, frame_ids: ["actor_player_walk_south_1", "actor_player_walk_south_2", "actor_player_walk_north_1", "actor_player_walk_north_2", "actor_player_walk_east_1", "actor_player_walk_east_2", "actor_player_walk_west_1", "actor_player_walk_west_2"], refs_valid: true, visible_pixels: 592},
    {actor_id: "mentor", action: "talk", clip_id: "mentor_talk_cycle", fps: 4, frame_count: 2, frame_ids: ["actor_mentor_idle", "actor_mentor_talk"], refs_valid: true, visible_pixels: 151},
    {actor_id: "enemy", action: "attack", clip_id: "enemy_attack_cycle", fps: 6, frame_count: 3, frame_ids: ["actor_enemy_idle", "actor_enemy_attack", "actor_enemy_hit"], refs_valid: true, visible_pixels: 230},
    {actor_id: "enemy", action: "hit", clip_id: "enemy_hit_recover", fps: 5, frame_count: 2, frame_ids: ["actor_enemy_hit", "actor_enemy_idle"], refs_valid: true, visible_pixels: 156}
  ],
  cex_runtime_player_client_allowed: false,
  wgpu_required: false,
  android_s5_real_device_claimed: false,
  external_evidence_ignored_for_current_animation_preview_pass: true,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$classic_animation_preview_json"
add_artifact_from_path native_bevy_classic_animation_preview "Native/Bevy classic animation preview" "$classic_animation_preview_json" release_review_input
classic_animation_preview_ppm="$TMP_DIR/bevy-classic-animation-preview.ppm"
printf 'P3\n640 448\n255\n' >"$classic_animation_preview_ppm"
truncate -s 100001 "$classic_animation_preview_ppm"
add_artifact_from_path native_bevy_classic_animation_preview_ppm "Native/Bevy classic animation preview PPM" "$classic_animation_preview_ppm" release_review_visual_evidence
classic_animation_selector_json="$TMP_DIR/bevy-classic-animation-selector.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_animation_selector_v1",
  status: "classic_animation_selector_green",
  green: true,
  ready_for_release_review: true,
  case_count: 6,
  case_detail_count: 6,
  selected_frame_count: 6,
  unique_selected_frame_count: 6,
  gate_count: 4,
  passed_gate_count: 4,
  failed_gate_count: 0,
  loaded_from_manifest: true,
  atlas_parse_gate: true,
  selector_case_gate: true,
  selected_frame_manifest_gate: true,
  animation_transition_gate: true,
  source_of_truth: "Classic animation selector evidence locks runtime state-to-frame decisions for dialogue, combat, damage, and marker pulse inside trnm-world-bevy.",
  cases: [
    {case_id: "mentor_idle", landmark_id: "mentor", selected_frame_id: "actor_mentor_idle", expected_frame_id: "actor_mentor_idle"},
    {case_id: "mentor_dialogue_talk", landmark_id: "mentor", selected_frame_id: "actor_mentor_talk", expected_frame_id: "actor_mentor_talk"},
    {case_id: "enemy_idle", landmark_id: "enemy", selected_frame_id: "actor_enemy_idle", expected_frame_id: "actor_enemy_idle"},
    {case_id: "enemy_combat_attack", landmark_id: "enemy", selected_frame_id: "actor_enemy_attack", expected_frame_id: "actor_enemy_attack"},
    {case_id: "enemy_combat_hit", landmark_id: "enemy", selected_frame_id: "actor_enemy_hit", expected_frame_id: "actor_enemy_hit"},
    {case_id: "objective_marker_pulse", landmark_id: "objective_gate", selected_frame_id: "marker_interaction", expected_frame_id: "marker_interaction"}
  ],
  selected_frames: ["actor_enemy_attack", "actor_enemy_hit", "marker_interaction", "actor_enemy_idle", "actor_mentor_idle", "actor_mentor_talk"],
  cex_runtime_player_client_allowed: false,
  wgpu_required: false,
  android_s5_real_device_claimed: false,
  external_evidence_ignored_for_current_animation_selector_pass: true,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$classic_animation_selector_json"
add_artifact_from_path native_bevy_classic_animation_selector "Native/Bevy classic animation selector" "$classic_animation_selector_json" release_review_input

classic_player_motion_probe_json="$TMP_DIR/bevy-classic-player-motion-probe.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_player_motion_probe_v1",
  status: "classic_player_motion_probe_green",
  green: true,
  ready_for_release_review: true,
  probe_format: "ppm_p3_rgb",
  probe_width: 640,
  probe_height: 192,
  probe_bytes: 1119395,
  sample_count: 8,
  accepted_input_count: 8,
  sample_detail_count: 8,
  selected_frame_id_count: 8,
  unique_direction_count: 4,
  gate_count: 7,
  passed_gate_count: 7,
  failed_gate_count: 0,
  unique_color_count: 17,
  non_background_pixels: 45334,
  label_pixel_count: 2940,
  loaded_from_manifest: true,
  atlas_parse_gate: true,
  accepted_input_gate: true,
  direction_coverage_gate: true,
  frame_match_gate: true,
  manifest_frame_gate: true,
  sheet_gate: true,
  label_gate: true,
  source_of_truth: "Classic player motion probe drives real NativeControlAction::Move inputs through apply_live_native_action, then proves runtime direction/walk-cycle state selects the expected low-spec player sprite frames.",
  selected_frame_ids: [
    "actor_player_walk_west_1",
    "actor_player_walk_east_1",
    "actor_player_walk_east_2",
    "actor_player_walk_west_2",
    "actor_player_walk_north_1",
    "actor_player_walk_south_2",
    "actor_player_walk_south_1",
    "actor_player_walk_north_2"
  ],
  samples: [
    {case_id: "north_1", direction: "north", expected_frame_id: "actor_player_walk_north_1", selected_frame_id: "actor_player_walk_north_1", accepted_local_input: true, frame_match: true, last_action: "local_move:north", last_result: "local_map_step_before_training"},
    {case_id: "north_2", direction: "north", expected_frame_id: "actor_player_walk_north_2", selected_frame_id: "actor_player_walk_north_2", accepted_local_input: true, frame_match: true, last_action: "local_move:north", last_result: "local_map_step_before_training"},
    {case_id: "east_1", direction: "east", expected_frame_id: "actor_player_walk_east_1", selected_frame_id: "actor_player_walk_east_1", accepted_local_input: true, frame_match: true, last_action: "local_move:east", last_result: "local_map_step_before_training"},
    {case_id: "east_2", direction: "east", expected_frame_id: "actor_player_walk_east_2", selected_frame_id: "actor_player_walk_east_2", accepted_local_input: true, frame_match: true, last_action: "local_move:east", last_result: "local_map_step_before_training"},
    {case_id: "south_1", direction: "south", expected_frame_id: "actor_player_walk_south_1", selected_frame_id: "actor_player_walk_south_1", accepted_local_input: true, frame_match: true, last_action: "local_move:south", last_result: "local_map_step_before_training"},
    {case_id: "south_2", direction: "south", expected_frame_id: "actor_player_walk_south_2", selected_frame_id: "actor_player_walk_south_2", accepted_local_input: true, frame_match: true, last_action: "local_move:south", last_result: "local_map_step_before_training"},
    {case_id: "west_1", direction: "west", expected_frame_id: "actor_player_walk_west_1", selected_frame_id: "actor_player_walk_west_1", accepted_local_input: true, frame_match: true, last_action: "local_move:west", last_result: "local_map_step_before_training"},
    {case_id: "west_2", direction: "west", expected_frame_id: "actor_player_walk_west_2", selected_frame_id: "actor_player_walk_west_2", accepted_local_input: true, frame_match: true, last_action: "local_move:west", last_result: "local_map_step_before_training"}
  ],
  cex_runtime_player_client_allowed: false,
  wgpu_required: false,
  android_s5_real_device_claimed: false,
  external_evidence_ignored_for_current_player_motion_probe_pass: true,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$classic_player_motion_probe_json"
add_artifact_from_path native_bevy_classic_player_motion_probe "Native/Bevy classic player motion probe" "$classic_player_motion_probe_json" release_review_input
classic_player_motion_probe_ppm="$TMP_DIR/bevy-classic-player-motion-probe.ppm"
printf 'P3\n640 192\n255\n' >"$classic_player_motion_probe_ppm"
truncate -s 100001 "$classic_player_motion_probe_ppm"
add_artifact_from_path native_bevy_classic_player_motion_probe_ppm "Native/Bevy classic player motion probe PPM" "$classic_player_motion_probe_ppm" release_review_visual_evidence


action_coach_json="$TMP_DIR/bevy-action-coach.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_action_coach_v1",
  status: "action_coach_green",
  green: true,
  coach_stage_gate: true,
  enter_execution_gate: true,
  final_next_gate: true,
  input_hint_contract_gate: true,
  coach_stage_checks: [
    {stage: "initial", expected_action: "TALK", action_matches: true, clean_player_line: true, coach_line: "ACTION COACH | Enter/NumpadEnter -> TALK | Shortcut R / Enter | Goal Press R or TALK to meet the mentor | Room START"},
    {stage: "after_Enter_TALK", expected_action: "TRAIN", action_matches: true, clean_player_line: true, coach_line: "ACTION COACH | Enter/NumpadEnter -> TRAIN | Shortcut T / Enter | Goal Press T or TRAIN to learn basic_unarmed | Room START"},
    {stage: "after_Enter_TRAIN", expected_action: "MOVE:north", action_matches: true, clean_player_line: true, coach_line: "ACTION COACH | Enter/NumpadEnter -> MOVE:north | Shortcut W / Enter | Goal Move NORTH to enter the arena | Room START"},
    {stage: "after_Enter_MOVE:north", expected_action: "FIGHT", action_matches: true, clean_player_line: true, coach_line: "ACTION COACH | Enter/NumpadEnter -> FIGHT | Shortcut F or Space / Enter | Goal Press F or FIGHT to resolve the first combat | Room ARENA"}
  ],
  enter_execution_checks: [
    {index: 0, key: "Enter", expected_action: "TALK", actual_action: "TALK", matches: true, accepted: true},
    {index: 1, key: "Enter", expected_action: "TRAIN", actual_action: "TRAIN", matches: true, accepted: true},
    {index: 2, key: "Enter", expected_action: "MOVE:north", actual_action: "MOVE:north", matches: true, accepted: true}
  ],
  keyboard_events: [
    {key: "Enter", action: "TALK", accepted: true, input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"},
    {key: "Enter", action: "TRAIN", accepted: true, input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"},
    {key: "Enter", action: "MOVE:north", accepted: true, input_path: "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action"}
  ],
  samples: [
    {input_hint_text: "ACTION COACH | Enter/NumpadEnter -> TALK"},
    {input_hint_text: "ACTION COACH | Enter/NumpadEnter -> TRAIN"},
    {input_hint_text: "ACTION COACH | Enter/NumpadEnter -> MOVE:north"},
    {input_hint_text: "ACTION COACH | Enter/NumpadEnter -> FIGHT"}
  ],
  final_runtime: {
    current_room_id: "league-coliseum",
    objective_status: "arrived_at_objective",
    completed_steps: ["boot", "talk_to_mentor", "npc_dialogue_opened", "train_basic_unarmed", "enter_training_room", "walk_grid_step_recorded", "arrive_at_first_objective", "exit_training_room_to_arena_route"],
    input_feedback_history: [
      {accepted: true, action_label: "TALK", input_source: "keyboard", room_id: "mirror-city-square"},
      {accepted: true, action_label: "TRAIN", input_source: "keyboard", room_id: "mirror-city-square"},
      {accepted: true, action_label: "MOVE:north", input_source: "keyboard", room_id: "league-coliseum"}
    ],
    contextual_action_history: ["contextual_deck:mirror-city-square:TALK", "contextual_deck:mirror-city-square:TRAIN", "contextual_deck:mirror-city-square:TASK:task-fixture-first-route", "contextual_deck:league-coliseum:COMBAT:attack"],
    xp: 10,
    tutorial_step: "resolve_first_combat",
    visited_rooms: ["mirror-city-square", "league-coliseum"]
  },
  source_of_truth: "ACTION COACH text is derived from the same focused action path that Enter/NumpadEnter executes",
  android_s5_real_device_claimed: false,
  external_evidence_ignored_for_current_action_coach_pass: true,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$action_coach_json"
action_coach_counts_json="$TMP_DIR/bevy-action-coach-counted.json"
jq '
  .coach_stage_check_count = (.coach_stage_checks | length)
  | .matched_coach_stage_check_count = ([.coach_stage_checks[] | select(.action_matches == true and .clean_player_line == true)] | length)
  | .enter_execution_check_count = (.enter_execution_checks | length)
  | .accepted_enter_execution_check_count = ([.enter_execution_checks[] | select(.accepted == true)] | length)
  | .matched_enter_execution_check_count = ([.enter_execution_checks[] | select(.matches == true)] | length)
  | .keyboard_event_count = (.keyboard_events | length)
  | .accepted_keyboard_event_count = ([.keyboard_events[] | select(.accepted == true)] | length)
  | .sample_count = (.samples | length)
  | .final_runtime_key_count = (.final_runtime | keys | length)
  | .final_runtime_completed_step_count = (.final_runtime.completed_steps | length)
  | .final_runtime_input_feedback_history_count = (.final_runtime.input_feedback_history | length)
  | .final_runtime_visited_room_count = (.final_runtime.visited_rooms | length)
' "$action_coach_json" >"$action_coach_counts_json"
mv "$action_coach_counts_json" "$action_coach_json"
add_artifact_from_path native_bevy_action_coach "Native/Bevy action coach" "$action_coach_json" release_review_input


player_hud_debug_layer_json="$TMP_DIR/bevy-player-hud-debug-layer.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_player_hud_debug_layer_v1",
  status: "player_hud_debug_layer_green",
  green: true,
  player_hud_gate: true,
  quest_layer_gate: true,
  debug_layer_gate: true,
  scene_debug_gate: true,
  input_hint_gate: true,
  panel_layer_gate: true,
  runtime_gate: true,
  player_layer: {
    character_status_text: "PLAYER HUD | HP 164/176 | Enemy 10 | XP 40 | Coins 10 | Room ARENA | Goal return to title | Next TITLE:OPEN | Gear Starter Gear | Title none",
    forbidden_debug_needles: ["INPUT SUMMARY", "DEBUG LAYER", "contract_version"],
    panel_ids: ["top_character_status", "quest_objective_panel", "room_narrative_panel", "virtual_joystick", "right_action_buttons", "reward_toast", "feedback_banner"],
    quest_panel_text: "PLAYER ROUTE | FIRST MINUTE HUD | NEXT BUTTON: TITLE:OPEN | QUEST JOURNAL | TASK LOG / PROGRESS: [x]CREATE [x]TALK [x]TRAIN [x]ARENA [x]FIGHT [x]SAVE [ ]CONTINUE | TASKS active: none | completed: task-fixture-first-route"
  },
  debug_layer: {
    event_log_text: "DEBUG LAYER | input/runtime diagnostics\n> Reward equipped: Route Guard Staff ready\n> Combat hit: enemy HP 10, player HP 164\n\nINPUT\nINPUT SUMMARY total 11 accepted 7 blocked 4 keyboard 11 buttons 0",
    input_hint_text: "ACTION COACH | Enter/NumpadEnter -> TITLE:OPEN\nINPUT HUD | NEXT TITLE:OPEN\nDEV INPUT\nGATES: 24 ready / 56 locked",
    scene_state_text: "DEBUG LAYER | Scene arena_outdoor | Transition combat_overlay_return_to_map | Dialogue mentor_training_complete | Combat combat_returned_to_map | Step step_north | Frame 2",
    panel_ids: ["event_log_panel", "npc_dialogue_choice_panel", "scene_transition_panel", "walk_animation_panel", "combat_scene_panel", "monochrome_stat_panel", "paper_skill_menu_overlay", "session_diagnostics"]
  },
  final_runtime: {
    current_room_id: "league-coliseum",
    objective_status: "first_playable_loop_complete",
    coins: 10,
    xp: 40,
    player_hp: 164,
    enemy_hp: 10,
    reward_claimed: true,
    equipment_ready: true,
    session_selected_slot_id: "A",
    completed_steps: ["boot", "talk_to_mentor", "train_basic_unarmed", "arrive_at_first_objective", "resolve_first_combat", "complete_first_task", "equip_route_guard_staff"],
    visited_rooms: ["mirror-city-square", "league-coliseum"],
    input_feedback_history: [
      {accepted: false, action_label: "TRAIN"},
      {accepted: false, action_label: "FIGHT"},
      {accepted: true, action_label: "MOVE:north"},
      {accepted: false, action_label: "LOOT:drop"},
      {accepted: false, action_label: "EQUIP:bandit_sash"},
      {accepted: true, action_label: "TALK"},
      {accepted: true, action_label: "TRAIN"},
      {accepted: true, action_label: "MOVE:north"},
      {accepted: true, action_label: "FIGHT"},
      {accepted: true, action_label: "COMPLETE"},
      {accepted: true, action_label: "EQUIP"}
    ],
    contextual_action_labels: ["TITLE:OPEN", "ACCOUNT:REGISTER", "ACCOUNT:LOGIN", "ACCOUNT:CONTINUE", "ROOM:mirror-city-square", "ROOM:delivery-dock", "NPC:enemy-market-bandit", "COMBAT:attack", "COMBAT:defend", "COMBAT:potion", "COMBAT:escape", "COMPLETE", "BAG:open", "SAVE:SLOT", "SLOT:A", "SAVE:A", "SLOT:B", "SAVE:B", "SLOT:C", "SAVE:C", "SAVE:SELECTED", "PAUSE:MENU"]
  },
  source_of_truth: "Bevy runtime samples prove player-facing HUD text and engineering diagnostics render in separate named layers while Rust remains authoritative",
  android_s5_real_device_claimed: false,
  external_evidence_ignored_for_current_player_hud_pass: true,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$player_hud_debug_layer_json"
player_hud_debug_layer_counts_json="$player_hud_debug_layer_json.counts"
jq '
  .player_layer_panel_count = (.player_layer.panel_ids | length)
  | .debug_layer_panel_count = (.debug_layer.panel_ids | length)
  | .final_runtime_key_count = (.final_runtime | keys | length)
  | .final_runtime_completed_step_count = (.final_runtime.completed_steps | length)
  | .final_runtime_input_feedback_history_count = (.final_runtime.input_feedback_history | length)
  | .final_runtime_visited_room_count = (.final_runtime.visited_rooms | length)
  | .final_runtime_contextual_action_label_count = (.final_runtime.contextual_action_labels | length)
' "$player_hud_debug_layer_json" >"$player_hud_debug_layer_counts_json"
mv "$player_hud_debug_layer_counts_json" "$player_hud_debug_layer_json"
add_artifact_from_path native_bevy_player_hud_debug_layer "Native/Bevy player HUD/debug layer" "$player_hud_debug_layer_json" release_review_input


player_ui_rescue_json="$TMP_DIR/bevy-player-ui-rescue.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_player_ui_rescue_v1",
  status: "player_ui_rescue_green",
  green: true,
  player_status_gate: true,
  route_panel_gate: true,
  quest_panel_gate: true,
  action_layer_gate: true,
  debug_deprioritized_gate: true,
  event_log_separation_gate: true,
  button_wall_deprioritized_gate: true,
  contextual_deck_layout_gate: true,
  right_rail_summary_gate: true,
  top_hud_density_gate: true,
  toast_lane_gate: true,
  visual_hierarchy_gate: true,
  art_direction_gate: true,
  scene_readability_gate: true,
  sprite_asset_quality_gate: true,
  map_model_visual_gate: true,
  map_occlusion_gate: true,
  ui_polish_gate: true,
  tileset_polish_gate: true,
  authored_art_pack_gate: true,
  runtime_gate: true,
  source_of_truth: "Bevy player UI rescue keeps the default player surface focused on route, next action, progress, and status while diagnostics remain in named debug layers.",
  player_layer: {
    character_status_text: "PLAYER HUD | HP 164/176 | Enemy 10 | XP 40 | Coins 10 | Room ARENA | Goal return to title | Next TITLE:OPEN | Gear Starter Gear | Title none",
    room_panel_text: "PLAYER ROUTE | FIRST MINUTE HUD | NEXT BUTTON: TITLE:OPEN | NEXT STEP: return to title\nQUEST JOURNAL | TASK LOG / PROGRESS: [x]CREATE [x]TALK [x]TRAIN [x]ARENA [x]FIGHT [x]SAVE [ ]CONTINUE\nSTATE: return to title",
    input_hint_text: "ACTION COACH | Enter/NumpadEnter -> TITLE:OPEN\nPLAYER ACTIONS | READY: TITLE:OPEN, SAVE:SELECTED, COMBAT:attack\nDEV INPUT | GATES: 24 ready / 56 locked",
    visible_quest_summary_text: "CURRENT OBJECTIVE | Reach the League Coliseum objective\nNEXT | TITLE:OPEN - return to title\nPROGRESS | 7/7 | REWARD 10c | gear ready",
    visible_stats_summary_text: "STATS | HP 164/176 | XP 40 | COINS 10",
    visible_bag_summary_text: "BAG | closed | items 1 equipped 0 drops 0\nAFFIX | locked | KEY bandit_sash locked",
    visible_event_summary_text: "LAST | Reward equipped: Route Guard Staff ready\nINPUT | EQUIP ok enabled_after_reward_claim\nNEXT | TITLE:OPEN",
    primary_cta_text: "PRIMARY | Enter -> TITLE:OPEN\nSHORTCUT | Enter | NEXT TITLE:OPEN",
    movement_hint_text: "Numpad / arrows / WASD",
    feedback_banner_text: "TOAST OK | EQUIP | enabled_after_reward_claim",
    feedback_banner_font_size: 8.5,
    feedback_banner_y: -82
  },
  button_wall_policy: {
    hidden_button_count: 34,
    deprioritized_button_count: 75,
    foreground_button_count: 18,
    player_deck_visible_count: 27,
    player_deck_hidden_count: 66
  },
  action_row_policy: {
    active_action_row_count: 3,
    active_action_row_ids: ["core_route_actions", "selected_slot_actions", "combat_bag_stat_actions"]
  },
  art_direction_policy: {
    surface_count: 8,
    surface_ids: ["map_focus_glow", "primary_cta_gold_glow", "action_deck_depth_shadow"],
    palette_roles: ["warm_gold", "cyan_focus", "neutral_shadow"]
  },
  scene_readability_policy: {
    surface_count: 6,
    surface_ids: ["player_selection_ring", "enemy_threat_ring", "objective_route_arrow"],
    focus_roles: ["player_identity", "combat_feedback"],
    visible_actor_kinds: ["npc", "enemy", "drop"],
    map_quality_surface_gate: true
  },
  sprite_asset_policy: {
    surface_count: 31,
    actor_kinds: ["player", "npc", "enemy", "drop", "feedback"],
    asset_roles: ["player_body_layer", "actor_shadow", "combat_hit_feedback_marker"]
  },
  map_model_visual_policy: {
    surface_count: 98,
    building_count: 24,
    road_count: 62,
    greenery_count: 8,
    terrain_count: 4,
    layers: ["building", "road", "greenery", "terrain"],
    visual_roles: ["building_mass", "walkable_road_path", "greenery_cluster", "terrain_zone_surface"]
  },
  map_occlusion_policy: {
    surface_count: 5,
    weighted_ratio: 0.055,
    max_panel_area: 15600,
    max_panel_alpha: 0.28,
    map_roles: ["edge_dialogue_hint", "edge_scene_hint", "edge_step_hint", "edge_combat_hint", "bottom_story_summary"]
  },
  ui_polish_policy: {
    surface_count: 12,
    max_font_size: 18,
    regions: ["top_hud", "right_rail", "action_deck", "movement_cluster", "primary_cta"],
    typography_roles: ["hud_compact", "summary_card", "action_deck_container", "primary_cta"],
    visual_priorities: ["primary", "secondary", "tertiary"]
  },
  tileset_polish_policy: {
    surface_count: 116,
    atlas_families: ["city_ground_tileset_v1", "hud_icon_tileset_v1"],
    layers: ["terrain", "road", "building", "greenery", "water", "hud"],
    asset_roles: ["primary_cta_glyph"],
    detail_roles: ["cta_arrow_marker"]
  },
  authored_art_pack_policy: {
    surface_count: 147,
    asset_pack_ids: ["trnm_world_authored_art_pack_v1"],
    asset_kinds: ["hud_icon", "actor_sprite", "terrain_tile"],
    replacement_slots: ["tile_sprite_slot", "hud_icon_slot", "actor_sprite_slot"],
    source_origins: ["local_authored_primitive_manifest_v1"],
    license_scopes: ["project_owned_internal_placeholder"],
    export_ready_count: 147
  },
  final_runtime: {
    current_room_id: "league-coliseum",
    objective_status: "first_playable_loop_complete",
    coins: 10,
    xp: 40,
    player_hp: 164,
    enemy_hp: 10,
    reward_claimed: true,
    equipment_ready: true,
    session_selected_slot_id: "A",
    completed_steps: ["complete_first_task", "equip_route_guard_staff"],
    input_feedback_history: [
      {accepted: false, action_label: "TRAIN"},
      {accepted: false, action_label: "FIGHT"},
      {accepted: true, action_label: "MOVE:north"},
      {accepted: false, action_label: "LOOT:drop"},
      {accepted: false, action_label: "EQUIP:bandit_sash"},
      {accepted: true, action_label: "TALK"},
      {accepted: true, action_label: "TRAIN"},
      {accepted: true, action_label: "MOVE:north"},
      {accepted: true, action_label: "FIGHT"},
      {accepted: true, action_label: "COMPLETE"},
      {accepted: true, action_label: "EQUIP"}
    ],
    contextual_action_labels: ["TITLE:OPEN", "SAVE:SELECTED", "COMBAT:attack"]
  },
  external_evidence_ignored_for_current_player_ui_rescue_pass: true,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$player_ui_rescue_json"
player_ui_rescue_counts_json="$player_ui_rescue_json.counts"
jq '
  .player_layer_field_count = (.player_layer | keys | length)
  | .action_row_count = (.action_row_policy.active_action_row_ids | length)
  | .art_direction_surface_count = .art_direction_policy.surface_count
  | .scene_readability_surface_count = .scene_readability_policy.surface_count
  | .sprite_asset_surface_count = .sprite_asset_policy.surface_count
  | .map_model_visual_surface_count = .map_model_visual_policy.surface_count
  | .map_model_visual_building_count = .map_model_visual_policy.building_count
  | .map_model_visual_road_count = .map_model_visual_policy.road_count
  | .map_model_visual_greenery_count = .map_model_visual_policy.greenery_count
  | .map_model_visual_terrain_count = .map_model_visual_policy.terrain_count
  | .map_occlusion_surface_count = .map_occlusion_policy.surface_count
  | .ui_polish_surface_count = .ui_polish_policy.surface_count
  | .tileset_polish_surface_count = .tileset_polish_policy.surface_count
  | .authored_art_pack_surface_count = .authored_art_pack_policy.surface_count
  | .final_runtime_key_count = (.final_runtime | keys | length)
  | .final_runtime_completed_step_count = (.final_runtime.completed_steps | length)
  | .final_runtime_input_feedback_history_count = (.final_runtime.input_feedback_history | length)
  | .final_runtime_contextual_action_label_count = (.final_runtime.contextual_action_labels | length)
' "$player_ui_rescue_json" >"$player_ui_rescue_counts_json"
mv "$player_ui_rescue_counts_json" "$player_ui_rescue_json"
add_artifact_from_path native_bevy_player_ui_rescue "Native/Bevy player UI rescue" "$player_ui_rescue_json" release_review_input

live_window_screenshot_sequence_json="$TMP_DIR/bevy-live-window-screenshot-sequence.json"
jq -n '
  ["title", "create", "talk", "train", "training_room", "arena", "fight_result", "save_continue", "title_continue", "resume_continue", "complete"] as $frame_ids |
  ["TITLE:NEW", "CREATE:CONFIRM", "TALK", "TRAIN", "MOVE:north", "FIGHT", "SAVE:SELECTED", "TITLE:OPEN", "TITLE:CONTINUE", "CONTINUE:SESSION"] as $action_labels |
  {
    contract_version: "trillionnium_world_bevy_live_window_screenshot_sequence_v1",
    status: "live_window_screenshot_sequence_green",
    green: true,
    ready_for_release_review: true,
    host_window_gate: true,
    key_count_gate: true,
    frame_count_gate: true,
    frame_sequence_gate: true,
    screenshot_nonblank_gate: true,
    frame_change_gate: true,
    slot_write_gate: true,
    contact_sheet_gate: true,
    final_frame_gate: true,
    runtime_texture_asset_contract: "trillionnium_world_bevy_runtime_texture_asset_v1",
    runtime_texture_asset_gate: true,
    runtime_texture_manifest_file_gate: true,
    runtime_texture_manifest_hash_gate: true,
    runtime_texture_launch_env_gate: true,
    runtime_texture_handle_gate: true,
    runtime_probe_contract: "trillionnium_world_bevy_runtime_probe_v1",
    runtime_texture_sprite_asset_binding_contract: "trillionnium_world_bevy_sprite_asset_binding_v1",
    runtime_texture_sprite_asset_binding_gate: true,
    runtime_texture_sprite_bound_surface_count: 32,
    runtime_texture_sprite_scene_layer_count: 4,
    runtime_texture_sprite_scene_layers: ["actor", "map", "hud", "feedback"],
    runtime_texture_sprite_material_slots: ["hud_icon_material", "world_tile_material", "actor_sprite_material", "feedback_glyph_material"],
    runtime_texture_sprite_material_slot_count: 4,
    runtime_texture_manifest_bytes: 16384,
    runtime_texture_manifest_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    runtime_texture_image_asset_handle_id: "bevy_image_handle::trnm_world_authored_sprite_sheet_v1",
    runtime_texture_atlas_layout_handle_id: "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1",
    slot_a_bytes: 44443,
    contact_sheet_colors: 12606,
    contact_sheet_size: [560, 1228],
    final_frame_bytes: 159694,
    expected_frame_ids: $frame_ids,
    actual_frame_ids: $frame_ids,
    expected_frame_count: ($frame_ids | length),
    actual_frame_count: ($frame_ids | length),
    actions: [$action_labels[] | {action_label: .}],
    action_count: ($action_labels | length),
    key_events: [range(0; 10) | {key: "Return"}],
    key_event_count: 10,
    capture_attempt_counts: [$frame_ids[] | 1],
    capture_attempt_count: ($frame_ids | length),
    changed_frame_count: (($frame_ids | length) - 1),
    frames: [$frame_ids[] as $frame_id | {frame_id: $frame_id, nonblank: true, size: [960, 540], colors_96x54: 3200, diff_mean_from_previous: (if $frame_id == "title" then null else 1.0 end), diff_bbox_from_previous: (if $frame_id == "title" then null else [0, 0, 960, 540] end)}],
    focus_event: {method: "XRaiseWindow+XSetInputFocus"},
    internal_live_window_screenshot_sequence_claimed: true,
    external_evidence_ignored_for_current_live_window_pass: true,
    gpu_upload_claimed: false,
    android_s5_real_device_claimed: false,
    public_launch_ready: false,
    production_ready_ui_claimed: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    warcraft_iii_asset_copied: false,
    openra_asset_copied: false,
    third_party_asset_copied: false,
    live_osm_ingestion_claimed: false
  }
' >"$live_window_screenshot_sequence_json"
add_artifact_from_path native_bevy_live_window_screenshot_sequence "Native/Bevy live-window screenshot sequence" "$live_window_screenshot_sequence_json" release_review_input

sprite_texture_sampling_json="$TMP_DIR/bevy-sprite-texture-sampling.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_sprite_texture_sampling_v1",
  status: "sprite_texture_sampling_green",
  green: true,
  ready_for_release_review: true,
  runtime_texture_asset_contract: "trillionnium_world_bevy_runtime_texture_asset_v1",
  runtime_texture_manifest_probe_contract: "trillionnium_world_bevy_runtime_texture_manifest_probe_v1",
  asset_store_registration_contract: "trillionnium_world_bevy_asset_store_registration_v1",
  sprite_asset_binding_contract: "trillionnium_world_bevy_sprite_asset_binding_v1",
  runtime_summary_gate: true,
  asset_store_registration_gate: true,
  sprite_asset_binding_gate: true,
  image_asset_resolve_gate: true,
  texture_atlas_layout_asset_resolve_gate: true,
  texture_atlas_rect_resolve_gate: true,
  texture_sample_nonblank_gate: true,
  four_layer_texture_sampling_gate: true,
  global_unique_texture_color_gate: true,
  boundary_gate: true,
  runtime_manifest_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  asset_store_registration: {
    green: true,
    asset_store_registered_gate: true,
    bevy_image_store_registration_gate: true,
    texture_atlas_layout_store_registration_gate: true,
    image_asset_handle_id: "bevy_image_handle::trnm_world_authored_sprite_sheet_v1",
    texture_atlas_layout_handle_id: "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1",
    render_world_asset_usage_requested: true
  },
  sprite_binding_lookup: {green: true, binding_count: 32},
  scene_layer_count: 4,
  scene_layers: ["map", "hud", "actor", "feedback"],
  material_slot_count: 4,
  material_slots: ["world_tile_material", "hud_icon_material", "actor_sprite_material", "feedback_glyph_material"],
  sampled_layer_counts: {map: 5, hud: 2, actor: 22, feedback: 3},
  sampled_material_slot_count: 4,
  sampled_material_slot_counts: {world_tile_material: 5, hud_icon_material: 2, actor_sprite_material: 22, feedback_glyph_material: 3},
  sampled_surface_count: 32,
  texture_unique_rgba_color_count: 10,
  sampled_surface_sample_count: 8,
  sampled_surfaces_sample: [
    {scene_layer: "actor", material_slot: "actor_sprite_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 0 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 0 }"},
    {scene_layer: "actor", material_slot: "actor_sprite_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 1 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 1 }"},
    {scene_layer: "map", material_slot: "world_tile_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 2 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 2 }"},
    {scene_layer: "hud", material_slot: "hud_icon_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 3 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 3 }"},
    {scene_layer: "feedback", material_slot: "feedback_glyph_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 4 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 4 }"},
    {scene_layer: "map", material_slot: "world_tile_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 5 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 5 }"},
    {scene_layer: "actor", material_slot: "actor_sprite_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 6 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 6 }"},
    {scene_layer: "actor", material_slot: "actor_sprite_material", image_asset_resolve_gate: true, texture_atlas_layout_asset_resolve_gate: true, texture_atlas_rect_resolve_gate: true, texture_sample_nonblank_gate: true, sample_count: 9, alpha_nonzero_sample_count: 9, texture_rect: {width: 32, height: 32}, sprite_image_asset_id_debug: "AssetId<Image>{ index: 7 }", sprite_texture_atlas_layout_asset_id_debug: "AssetId<TextureAtlasLayout>{ index: 7 }"}
  ],
  host_log_line: "TRNM_WORLD_BEVY_SPRITE_TEXTURE_SAMPLING green=true sampled_surfaces=32 unique_rgba_colors=10",
  asset_boundary: "bevy_assets_image_texture_atlas_cpu_sampling_not_gpu_upload_claim",
  host_side_cpu_texture_sampling_claimed: true,
  external_evidence_ignored_for_current_sprite_texture_pass: true,
  gpu_upload_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false,
  live_osm_ingestion_claimed: false
}' >"$sprite_texture_sampling_json"
add_artifact_from_path native_bevy_sprite_texture_sampling "Native/Bevy sprite texture sampling" "$sprite_texture_sampling_json" release_review_input

live_window_sampled_texture_correlation_json="$TMP_DIR/bevy-live-window-sampled-texture-correlation.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1",
  status: "live_window_sampled_texture_correlation_green",
  green: true,
  ready_for_release_review: true,
  sprite_texture_sampling_contract: "trillionnium_world_bevy_sprite_texture_sampling_v1",
  live_window_texture_correlation_contract: "trillionnium_world_bevy_live_window_texture_correlation_v1",
  gates: {
    sprite_texture_sampling_gate: true,
    live_window_texture_correlation_gate: true,
    same_image_handle_gate: true,
    same_texture_atlas_layout_gate: true,
    same_runtime_manifest_hash_gate: true,
    sampled_layer_count_gate: true,
    sampled_material_slot_count_gate: true,
    sampled_texture_nonblank_gate: true,
    four_layer_sampled_live_correlation_gate: true,
    boundary_gate: true
  },
  runtime_manifest_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  sampled_surface_count: 32,
  texture_unique_rgba_color_count: 10,
  live_frame_count: 11,
  live_final_frame_colors_96x54: 3376,
  image_asset_handle_id: "bevy_image_handle::trnm_world_authored_sprite_sheet_v1",
  texture_atlas_layout_handle_id: "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1",
  sampled_layer_count: 4,
  sampled_layer_counts: {map: 5, hud: 2, actor: 22, feedback: 3},
  sampled_material_slot_count: 4,
  sampled_material_slot_counts: {world_tile_material: 5, hud_icon_material: 2, actor_sprite_material: 22, feedback_glyph_material: 3},
  layer_correlation_count: 4,
  layer_correlations: [
    {scene_layer: "actor", passes: true, sampled_surface_count: 22, sampled_texture_gate: true, live_window_texture_correlation_gate: true, live_pixel_sampled_colors: 5650, live_sprite_binding_count: 22, texture_atlas_indexes: [6, 7, 10, 11], material_slots: ["actor_sprite_material"]},
    {scene_layer: "feedback", passes: true, sampled_surface_count: 3, sampled_texture_gate: true, live_window_texture_correlation_gate: true, live_pixel_sampled_colors: 1497, live_sprite_binding_count: 3, texture_atlas_indexes: [8, 18, 19], material_slots: ["feedback_glyph_material"]},
    {scene_layer: "hud", passes: true, sampled_surface_count: 2, sampled_texture_gate: true, live_window_texture_correlation_gate: true, live_pixel_sampled_colors: 2389, live_sprite_binding_count: 2, texture_atlas_indexes: [5, 9], material_slots: ["hud_icon_material"]},
    {scene_layer: "map", passes: true, sampled_surface_count: 5, sampled_texture_gate: true, live_window_texture_correlation_gate: true, live_pixel_sampled_colors: 6893, live_sprite_binding_count: 5, texture_atlas_indexes: [0, 1, 2, 3, 4], material_slots: ["world_tile_material"]}
  ],
  asset_boundary: "live_window_pixels_correlated_to_cpu_sampled_bevy_texture_atlas_not_gpu_upload_claim",
  internal_live_window_sampled_texture_correlation_claimed: true,
  external_evidence_ignored_for_current_sampled_texture_pass: true,
  gpu_upload_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false,
  live_osm_ingestion_claimed: false
}' >"$live_window_sampled_texture_correlation_json"
add_artifact_from_path native_bevy_live_window_sampled_texture_correlation "Native/Bevy live-window sampled texture correlation" "$live_window_sampled_texture_correlation_json" release_review_input

render_asset_eligibility_json="$TMP_DIR/bevy-render-asset-eligibility.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_render_asset_eligibility_v1",
  status: "render_asset_eligibility_green",
  green: true,
  ready_for_release_review: true,
  runtime_texture_asset_contract: "trillionnium_world_bevy_runtime_texture_asset_v1",
  sprite_texture_sampling_contract: "trillionnium_world_bevy_sprite_texture_sampling_v1",
  live_window_sampled_texture_correlation_contract: "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1",
  runtime_summary_gate: true,
  asset_store_registration_gate: true,
  sampled_live_correlation_gate: true,
  render_asset_usage_gate: true,
  image_descriptor_render_eligibility_gate: true,
  atlas_layout_render_eligibility_gate: true,
  sprite_render_reference_gate: true,
  boundary_gate: true,
  runtime_manifest_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  image_asset_handle_id: "bevy_image_handle::trnm_world_authored_sprite_sheet_v1",
  texture_atlas_layout_handle_id: "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1",
  bevy_image_asset_id_debug: "Handle<Image>(index=1)",
  bevy_texture_atlas_layout_asset_id_debug: "Handle<TextureAtlasLayout>(index=1)",
  image_present: true,
  texture_atlas_layout_present: true,
  image_asset_usage_main_world: true,
  image_asset_usage_render_world: true,
  image_asset_usage_debug: "RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD",
  image_format_debug: "Rgba8UnormSrgb",
  image_dimension_debug: "D2",
  image_dimensions: {width: 256, height: 128, depth_or_array_layers: 1},
  image_data_bytes: 131072,
  texture_atlas_rect_count: 32,
  first_texture_rect_dimensions: {width: 32, height: 32},
  sprite_render_reference_count: 32,
  sprite_render_reference_sample_count: 8,
  sprite_render_references_sample: [
    {scene_layer: "map", material_slot: "world_tile_material", texture_atlas_index: 0, render_asset_reference_gate: true},
    {scene_layer: "map", material_slot: "world_tile_material", texture_atlas_index: 1, render_asset_reference_gate: true},
    {scene_layer: "hud", material_slot: "hud_icon_material", texture_atlas_index: 5, render_asset_reference_gate: true},
    {scene_layer: "actor", material_slot: "actor_sprite_material", texture_atlas_index: 6, render_asset_reference_gate: true},
    {scene_layer: "actor", material_slot: "actor_sprite_material", texture_atlas_index: 7, render_asset_reference_gate: true},
    {scene_layer: "feedback", material_slot: "feedback_glyph_material", texture_atlas_index: 8, render_asset_reference_gate: true},
    {scene_layer: "hud", material_slot: "hud_icon_material", texture_atlas_index: 9, render_asset_reference_gate: true},
    {scene_layer: "actor", material_slot: "actor_sprite_material", texture_atlas_index: 10, render_asset_reference_gate: true}
  ],
  asset_boundary: "bevy_image_render_asset_usage_eligible_not_render_world_extraction_or_gpu_upload_claim",
  host_side_render_asset_eligibility_claimed: true,
  external_evidence_ignored_for_current_render_asset_pass: true,
  render_world_extraction_completed_claimed: false,
  gpu_upload_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false,
  live_osm_ingestion_claimed: false
}' >"$render_asset_eligibility_json"
add_artifact_from_path native_bevy_render_asset_eligibility "Native/Bevy render asset eligibility" "$render_asset_eligibility_json" release_review_input

classic_playtest_readiness_json="$TMP_DIR/bevy-classic-playtest-readiness.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_playtest_readiness_v1",
  status: "classic_playtest_readiness_green",
  green: true,
  checks: {
    classic_rts_full_screen_ui_replication_green: true,
    classic_rts_shell_meta_ui_replication_green: true,
    classic_rts_match_setup_ui_replication_green: true,
    classic_rts_first_contact_basin_spec_green: true,
    classic_rts_campaign_outcome_ui_readiness_green: true,
    classic_rts_campaign_ui_continuity_green: true,
    classic_rts_in_match_hud_state_replication_green: true,
    classic_rts_session_state_continuity_green: true,
    classic_rts_continuous_player_flow_green: true,
    classic_rts_full_game_visual_ui_replication_green: true,
    classic_rts_openra_screen_for_screen_ui_replication_green: true,
    classic_rts_combat_readability_pressure_readiness_green: true,
    classic_rts_playtest_observability_readiness_green: true,
    client_boundary_green: true,
    playtest_runner_status_green: true,
    playtest_launcher_green: true
  },
  gates: {
    rts_full_screen_ui_replication_gate: true,
    rts_full_screen_ui_replication_player_first_screen_gate: true,
    rts_shell_meta_ui_replication_player_first_screen_gate: true,
    rts_shell_meta_ui_replication_gate: true,
    rts_match_setup_ui_replication_gate: true,
    rts_first_contact_basin_spec_gate: true,
    rts_first_contact_runtime_review_gate: true,
    rts_first_contact_runtime_adapter_evidence_gate: true,
    rts_first_contact_offline_adapter_consumption_gate: true,
    rts_first_contact_offline_adapter_session_transition_gate: true,
    rts_first_contact_offline_adapter_lobby_ready_gate: true,
    rts_match_setup_ui_replication_player_first_screen_gate: true,
    rts_campaign_outcome_ui_readiness_runtime_screen_gate: true,
    rts_campaign_outcome_ui_readiness_player_first_screen_gate: true,
    rts_campaign_outcome_ui_readiness_gate: true,
    rts_combat_readability_pressure_player_first_screen_gate: true,
    rts_in_match_hud_state_replication_gate: true,
    rts_in_match_hud_state_replication_player_first_screen_gate: true,
    rts_session_state_continuity_player_first_session_resume_screen_gate: true,
    rts_session_state_continuity_gate: true,
    rts_continuous_player_flow_title_account_gate: true,
    rts_continuous_player_flow_match_setup_gate: true,
    rts_continuous_player_flow_in_match_hud_gate: true,
    rts_continuous_player_flow_command_feedback_gate: true,
    rts_continuous_player_flow_save_resume_gate: true,
    rts_continuous_player_flow_outcome_open_world_gate: true,
    rts_continuous_player_flow_chain_gate: true,
    rts_continuous_player_flow_player_first_continuous_flow_screen_gate: true,
    rts_continuous_player_flow_native_client_boundary_gate: true,
    rts_continuous_player_flow_gate: true,
    rts_continuous_player_flow_rts_evidence_review_gate: true,
    rts_full_game_visual_ui_replication_source_contract_gate: true,
    rts_full_game_visual_ui_replication_source_green_gate: true,
    rts_full_game_visual_ui_replication_runtime_screen_chain_gate: true,
    rts_full_game_visual_ui_replication_runtime_screen_gate: true,
    rts_full_game_visual_ui_replication_player_flow_gate: true,
    rts_full_game_visual_ui_replication_coverage_surface_gate: true,
    rts_full_game_visual_ui_replication_preview_gate: true,
    rts_full_game_visual_ui_replication_player_first_tactical_composition_gate: true,
    rts_full_game_visual_ui_replication_player_first_screen_gate: true,
    rts_live_session_playthrough_player_first_live_session_screen_gate: true,
    rts_live_session_playthrough_runtime_screen_gate: true,
    rts_live_session_playthrough_rts_evidence_review_gate: true,
    rts_full_game_visual_ui_replication_no_copy_boundary_gate: true,
    rts_full_game_visual_ui_replication_rts_evidence_review_gate: true,
    rts_full_game_visual_ui_replication_gate: true,
    rts_openra_screen_for_screen_ui_replication_source_contract_gate: true,
    rts_openra_screen_for_screen_ui_replication_source_green_gate: true,
    rts_openra_screen_for_screen_ui_replication_runtime_vocabulary_gate: true,
    rts_openra_screen_for_screen_ui_replication_widget_root_reference_gate: true,
    rts_openra_screen_for_screen_ui_replication_screen_set_gate: true,
    rts_openra_screen_for_screen_ui_replication_source_screen_chain_gate: true,
    rts_openra_screen_for_screen_ui_replication_preview_gate: true,
    rts_openra_screen_for_screen_ui_replication_no_asset_copy_boundary_gate: true,
    rts_openra_screen_for_screen_ui_replication_player_first_ingame_screen_gate: true,
    rts_openra_screen_for_screen_ui_replication_style_screen_set_gate: true,
    rts_openra_screen_for_screen_ui_replication_gate: true,
    rts_openra_screen_for_screen_ui_replication_rts_evidence_review_gate: true
  },
  headline: {
    rts_full_screen_ui_replication_surface_count: 10,
    rts_full_screen_ui_replication_player_first_tactical_view_non_background: 112566,
    rts_full_screen_ui_replication_player_first_tactical_view_frame_pixel_count: 10208,
    rts_full_screen_ui_replication_player_first_status_strip_pixel_count: 12960,
    rts_shell_meta_ui_replication_surface_count: 12,
    rts_shell_meta_ui_replication_player_first_surface_non_background: 500000,
    rts_shell_meta_ui_replication_player_first_frame_pixel_count: 13000,
    rts_shell_meta_ui_replication_player_first_account_bar_pixel_count: 32000,
    rts_shell_meta_ui_replication_player_first_session_panel_pixel_count: 36000,
    rts_shell_meta_ui_replication_player_first_right_rail_pixel_count: 32000,
    rts_shell_meta_ui_replication_player_first_handoff_strip_pixel_count: 95000,
    rts_match_setup_ui_replication_surface_count: 10,
    rts_match_setup_ui_replication_player_first_map_non_background: 181305,
    rts_match_setup_ui_replication_player_first_map_frame_pixel_count: 12648,
    rts_match_setup_ui_replication_player_first_status_strip_pixel_count: 18896,
    rts_match_setup_ui_replication_player_first_rules_rail_pixel_count: 104799,
    rts_match_setup_ui_replication_player_first_ready_strip_pixel_count: 40404,
    rts_first_contact_basin_spec_map_id: "first_contact_basin",
    rts_first_contact_basin_spec_actor_count: 39,
    rts_first_contact_basin_spec_spawn_count: 4,
    rts_first_contact_runtime_review_contract: "trnm_rts_evidence_bevy_runtime_adapter_v1",
    rts_first_contact_runtime_review_contract_count: 5,
    rts_first_contact_runtime_review_contracts: ["trnm_rts_bevy_runtime_first_contact_player_screen_application_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"],
    rts_first_contact_runtime_review_before_command_queue: ["build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
    rts_first_contact_runtime_review_after_command_queue: ["move:8,4"],
    rts_first_contact_runtime_review_ready_state_labels: ["authority:offline_loopback:no_socket", "player:local-player:ready", "bot:mirror_guard:ready"],
    rts_first_contact_runtime_review_command_stamp_tile: "8,4",
    rts_first_contact_runtime_review_source_of_truth: "First Contact player-screen/offline-adapter application, consumption, session-transition, and lobby-ready review samples",
    rts_campaign_outcome_ui_readiness_runtime_screen_mode: "player_runtime_campaign_outcome_screen",
    rts_campaign_outcome_ui_readiness_evidence_board_only: false,
    rts_campaign_outcome_ui_readiness_first_minute_player_first_non_background: 928800,
    rts_campaign_outcome_ui_readiness_victory_non_background_pixels: 1382400,
    rts_campaign_outcome_ui_readiness_base_assault_non_background_pixels: 2073600,
    rts_campaign_outcome_ui_readiness_aftermath_player_first_view_non_background: 526242,
    rts_campaign_outcome_ui_readiness_open_world_player_first_view_non_background: 541668,
    rts_in_match_hud_state_replication_surface_count: 8,
    rts_in_match_hud_state_replication_command_grid_pixel_count: 1240,
    rts_in_match_hud_state_replication_player_first_view_non_background: 541917,
    rts_in_match_hud_state_replication_player_first_view_frame_pixel_count: 15370,
    rts_in_match_hud_state_replication_player_first_top_status_strip_pixel_count: 49689,
    rts_in_match_hud_state_replication_player_first_surface_card_pixel_count: 50775,
    rts_in_match_hud_state_replication_player_first_right_rail_non_background: 133712,
    rts_in_match_hud_state_replication_player_first_bottom_command_lane_pixel_count: 67122,
    rts_in_match_hud_state_replication_player_first_control_color_pixel_count: 9847,
    rts_session_state_continuity_surface_count: 8,
    rts_session_state_continuity_player_first_resume_view_non_background: 420000,
    rts_session_state_continuity_player_first_resume_view_frame: 13504,
    rts_session_state_continuity_player_first_resume_status_strip: 23320,
    rts_session_state_continuity_player_first_resume_stage_rail: 190000,
    rts_session_state_continuity_slot_a_bytes: 46253,
    rts_session_state_continuity_final_objective_status: "first_playable_loop_complete",
    rts_session_state_continuity_open_world_state: "resumed:league-coliseum",
    rts_continuous_player_flow_step_count: 6,
    rts_continuous_player_flow_non_background_pixels: 1440000,
    rts_continuous_player_flow_title_account_pixel_count: 13552,
    rts_continuous_player_flow_match_setup_pixel_count: 13552,
    rts_continuous_player_flow_in_match_hud_pixel_count: 13552,
    rts_continuous_player_flow_command_feedback_pixel_count: 13552,
    rts_continuous_player_flow_save_load_resume_pixel_count: 13348,
    rts_continuous_player_flow_outcome_open_world_pixel_count: 13552,
    rts_continuous_player_flow_player_first_flow_view_non_background: 420000,
    rts_continuous_player_flow_player_first_flow_view_frame_pixel_count: 13504,
    rts_continuous_player_flow_player_first_flow_status_strip_pixel_count: 23320,
    rts_continuous_player_flow_player_first_flow_stage_rail_pixel_count: 130000,
    rts_continuous_player_flow_final_objective_status: "first_playable_loop_complete",
    rts_continuous_player_flow_open_world_state: "resumed:league-coliseum",
    rts_continuous_player_flow_restored_room_id: "league-coliseum",
    rts_continuous_player_flow_review_contract: "trnm_rts_evidence_continuous_player_flow_review_v1",
    rts_continuous_player_flow_review_source_of_truth: "The RTS evidence crate reviews the six-step continuous player flow from title/account through match setup, in-match HUD, command feedback, save/resume, and outcome/open-world return.",
    rts_full_game_visual_ui_replication_runtime_screen_mode: "player_runtime_full_game_visual_ui_screen",
    rts_full_game_visual_ui_replication_evidence_board_only: false,
    rts_full_game_visual_ui_replication_surface_count: 18,
    rts_full_game_visual_ui_replication_non_background_pixels: 2073600,
    rts_full_game_visual_ui_replication_hud_chrome_pixel_count: 276317,
    rts_full_game_visual_ui_replication_command_pixel_count: 42590,
    rts_full_game_visual_ui_replication_session_pixel_count: 39312,
    rts_full_game_visual_ui_replication_outcome_pixel_count: 26546,
    rts_full_game_visual_ui_replication_player_first_tactical_preview_non_background: 570458,
    rts_full_game_visual_ui_replication_player_first_tactical_viewport_frame_pixel_count: 16704,
    rts_full_game_visual_ui_replication_player_first_tactical_status_strip_pixel_count: 21375,
    rts_live_session_playthrough_runtime_screen_mode: "player_runtime_live_session_playthrough_screen",
    rts_full_game_visual_ui_replication_live_session_stage_count: 6,
    rts_full_game_visual_ui_replication_live_session_accepted_input_count: 91,
    rts_live_session_playthrough_player_first_live_view_non_background: 360000,
    rts_live_session_playthrough_player_first_live_view_frame_pixel_count: 15408,
    rts_live_session_playthrough_player_first_live_status_strip_pixel_count: 20880,
    rts_live_session_playthrough_player_first_live_stage_rail_pixel_count: 103032,
    rts_live_session_playthrough_review_contract: "trnm_rts_evidence_live_session_playthrough_review_v1",
    rts_live_session_playthrough_review_source_of_truth: "The RTS evidence crate reviews the same-process local live session playthrough from title/account through campaign start, in-match HUD, live command feedback, slot A save/load/resume, and open-world outcome.",
    rts_full_game_visual_ui_replication_final_objective_status: "open_world_after_action_ready",
    rts_full_game_visual_ui_replication_open_world_state: "resumed:league-coliseum",
    rts_full_game_visual_ui_replication_review_contract: "trnm_rts_evidence_full_game_visual_ui_replication_review_v1",
    rts_full_game_visual_ui_replication_review_source_of_truth: "The RTS evidence crate reviews the local Rust/Bevy full-game visual/UI replication aggregate.",
    rts_openra_screen_for_screen_ui_replication_screen_count: 8,
    rts_openra_screen_for_screen_ui_replication_surface_count: 8,
    rts_openra_screen_for_screen_ui_replication_widget_root_count: 4,
    rts_openra_screen_for_screen_ui_replication_runtime_screen_mode: "player_runtime_openra_style_ingame_screen_set",
    rts_openra_screen_for_screen_ui_replication_evidence_board_only: false,
    rts_openra_screen_for_screen_ui_replication_player_first_ingame_view_non_background: 80000,
    rts_openra_screen_for_screen_ui_replication_player_first_ingame_sidebar_non_background: 35000,
    rts_openra_screen_for_screen_ui_replication_player_first_ingame_command_lane_non_background: 6000,
    rts_openra_screen_for_screen_ui_replication_style_screen_set_claimed: true,
    rts_openra_screen_for_screen_ui_replication_claimed: false,
    rts_openra_screen_for_screen_ui_replication_asset_parity_claimed: false,
    rts_openra_screen_for_screen_ui_replication_engine_port_claimed: false,
    rts_openra_screen_for_screen_ui_replication_review_contract: "trnm_rts_evidence_openra_style_screen_set_review_v1",
    rts_openra_screen_for_screen_ui_replication_review_source_of_truth: "The RTS evidence crate reviews the OpenRA-style screen-set UI replication boundary.",
    rts_central_keep_pressure_accepted_input_count: 40,
    rts_central_keep_pressure_state: "pressure_locked:central_keep",
    rts_unit_status_portrait_frame_pixel_count: 15339,
    rts_ability_tooltip_telegraph_tooltip_pixel_count: 10466,
    rts_depth_readability_foreground_pixel_count: 11000,
    rts_combat_readability_pressure_player_first_view_non_background: 248151,
    rts_combat_readability_pressure_player_first_view_frame_pixel_count: 13752,
    rts_combat_readability_pressure_player_first_status_strip_pixel_count: 26777,
    rts_combat_readability_pressure_player_first_rail_pixel_count: 123537,
    rts_combat_readability_pressure_player_first_command_lane_pixel_count: 65616,
    rts_combat_readability_pressure_player_first_alert_pixel_count: 20927
  },
  artifacts: {
    classic_rts_continuous_player_flow: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-continuous-player-flow.json",
    classic_rts_continuous_player_flow_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-continuous-player-flow.ppm",
    classic_rts_combat_readability_pressure_screen: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness/combat-pressure-screen.ppm",
    classic_rts_full_game_visual_ui_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.json",
    classic_rts_full_game_visual_ui_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.ppm"
  },
  internal_classic_playtest_readiness_claimed: true,
  external_evidence_ignored_for_current_playtest_pass: true,
  android_s5_real_device_claimed: false,
  public_launch_ready: false,
  production_ready_ui_claimed: false,
  screen_for_screen_openra_ui_claimed: false,
  openra_engine_port_claimed: false,
  warcraft_iii_asset_copied: false,
  openra_asset_copied: false,
  third_party_asset_copied: false
}' >"$classic_playtest_readiness_json"
classic_playtest_readiness_counts_json="$TMP_DIR/bevy-classic-playtest-readiness-counted.json"
jq '
  .check_count = (.checks | length)
  | .passed_check_count = ([.checks[]] | map(select(. == true)) | length)
  | .failed_check_count = ([.checks[]] | map(select(. != true)) | length)
  | .artifact_count = (.artifacts | length)
  | .gate_count = (.gates | length)
  | .true_gate_count = ([.gates[]] | map(select(. == true)) | length)
  | .false_boundary_gate_count = ([
      .gates
      | to_entries[]
      | select((.key == "cex_runtime_player_client_allowed" or .key == "wgpu_required") and .value == false)
    ] | length)
  | .passed_gate_count = (.true_gate_count + .false_boundary_gate_count)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
' "$classic_playtest_readiness_json" >"$classic_playtest_readiness_counts_json"
mv "$classic_playtest_readiness_counts_json" "$classic_playtest_readiness_json"
add_artifact_from_path native_bevy_classic_playtest_readiness "Native/Bevy classic playtest readiness" "$classic_playtest_readiness_json" release_review_input
jq -n \
  --argjson artifacts "$(jq -s '.' "$artifacts_jsonl")" \
  '{
    contract_version: "trillionnium_world_release_review_packet_v1",
    status: "release_review_packet_ready_with_public_launch_blockers",
    ready_for_release_review: true,
    public_launch_ready: false,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    missing_artifacts: [],
    artifacts: $artifacts
  }' >"$packet_json"
{
  printf '# Fixture Packet\n\n'
  printf '## Still Requires Real External Evidence\n\n'
  printf -- '- [ ] fixture blocker\n\n'
  printf '## Boundary\n\n'
  printf -- '- Native/Bevy keyboard replay, classic animation preview/selector, classic player motion, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.\n'
} >"$packet_md"
set +e
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON="$packet_json" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD="$packet_md" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG="$packet_log" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY="$summary_json" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh" --no-refresh >"$TMP_DIR/stdout.log" 2>"$TMP_DIR/stderr.log"
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  echo "[FAIL] packet integrity projectile/ability semantic fixture unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi
if [[ ! -f "$summary_json" ]]; then
  echo "[FAIL] packet integrity projectile/ability semantic fixture did not write summary" >&2
  exit 1
fi
jq -e '
	  .status == "release_review_packet_integrity_blocked"
	  and .green == false
	  and (.failures | length) == 2
	  and ([.failures[].name] | index("classic_rts_projectile_ability_semantics"))
	  and ([.failures[].name] | index("classic_rts_projectile_ability_ppm_semantics"))
	  and (([.failures[].detail] | index("sha256_mismatch")) == null)
	  and (([.failures[].detail] | index("bytes_mismatch")) == null)
	  and (([.failures[].detail] | index("contract_mismatch")) == null)
	  and (([.failures[].detail] | index("status_mismatch")) == null)
	' "$summary_json" >/dev/null
echo "[PASS] release review packet integrity rejects semantically invalid classic RTS projectile/ability summary and PPM even when checksums match"
