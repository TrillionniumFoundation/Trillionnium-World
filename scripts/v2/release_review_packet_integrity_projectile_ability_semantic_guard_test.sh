#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

packet_json="$TMP_DIR/release-review-packet.json"
packet_md="$TMP_DIR/release-review-packet.md"
packet_log="$TMP_DIR/release-review-packet.log"
summary_json="$TMP_DIR/release-review-packet-integrity.json"
artifacts_jsonl="$TMP_DIR/artifacts.jsonl"

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

for index in $(seq 1 52); do
  artifact_path="$TMP_DIR/fixture_${index}.json"
  jq -nc \
    --arg id "fixture_${index}" \
    '{contract_version: "fixture_contract_v1", status: "fixture_green", payload: $id}' >"$artifact_path"
  add_artifact_from_path "fixture_${index}" "fixture_${index}" "$artifact_path" fixture
done

semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_semantic_fixture_v1",
  status: "release_review_packet_integrity_semantic_fixture_green",
  green: true,
  fixture_kind: "first_minute_command_feedback_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_first_minute_command_feedback_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 99,
  expected_semantic_failure_count: 4,
  expected_semantic_failure_names: [
    "first_minute_command_feedback_replay_semantics",
    "first_minute_command_feedback_source_recording_semantics",
    "first_minute_command_feedback_recording_semantics",
    "first_minute_command_feedback_replay_ppm_semantics"
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
  fake_packet_artifact_count: 99,
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
  fake_packet_artifact_count: 99,
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
  fake_packet_artifact_count: 99,
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
  fake_packet_artifact_count: 99,
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
  fake_packet_artifact_count: 99,
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
  final_control_group_id: "2",
  final_selected_unit_ids: ["square_guard_patrol", "square_creep_wander"],
  final_selection_box_tile_ids: ["5,5", "6,5", "5,4", "6,4"],
  final_control_group_assignments: ["1:player|square_guard_patrol|square_worker_carry|square_creep_wander", "2:square_guard_patrol|square_creep_wander"],
  final_active_control_group_ids: ["1", "2"],
  final_minimap_command_tile_id: "9,2",
  final_minimap_command_kind: "rally",
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
  fake_packet_artifact_count: 99,
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
  action_labels: ["RTS:SELECT:1", "RTS:QUEUE:build:watch_tower@7,4", "RTS:QUEUE:complete:watch_tower@7,4", "RTS:QUEUE:repair:watch_tower@7,4", "RTS:QUEUE:build:scout_tower@8,4", "RTS:QUEUE:cancel:scout_tower@8,4"],
  final_structure_state: "cancelled:scout_tower@8,4",
  final_build_site_tile_ids: ["8,4", "9,4"],
  final_building_blueprint_id: "scout_tower",
  final_building_progress_percent: 100,
  final_completed_structure_ids: ["watch_tower"],
  final_repair_target_id: "watch_tower",
  final_repair_progress_percent: 80,
  final_cancelled_structure_ids: ["scout_tower"],
  final_refund_delta_log: ["gold:+90", "lumber:+30"],
  final_structure_health_percents: [100, 82],
  final_resource_spend_log: ["build:-120g:-40l", "repair:-45g:-20l"],
  final_command_queue: ["select_group_1", "blueprint:watch_tower@7,4", "complete:watch_tower@7,4", "repair:watch_tower@7,4", "blueprint:scout_tower@8,4", "cancel:scout_tower@8,4", "refund:gold:+90|lumber:+30"],
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
  fake_packet_artifact_count: 99,
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
  fake_packet_artifact_count: 99,
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
  printf -- '- Native/Bevy replay, action coach, HUD/debug layer, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.\n'
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
