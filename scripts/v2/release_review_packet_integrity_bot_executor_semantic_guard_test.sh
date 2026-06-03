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
  fake_packet_artifact_count: 106,
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
  fake_packet_artifact_count: 106,
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
  fake_packet_artifact_count: 106,
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
  fake_packet_artifact_count: 106,
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
  fake_packet_artifact_count: 106,
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
add_artifact_from_path release_review_packet_integrity_control_loop_semantic_fixture "Release review packet integrity control loop semantic fixture" "$control_loop_semantic_fixture_json" release_review_gate

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
  fake_packet_artifact_count: 106,
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
  fake_packet_artifact_count: 106,
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
  final_build_site_tile_ids: ["8,4", "8,5", "9,4"],
  final_building_blueprint_id: "scout_tower",
  final_building_progress_percent: 100,
  final_completed_structure_ids: ["watch_tower"],
  final_repair_target_id: "watch_tower",
  final_repair_progress_percent: 76,
  final_cancelled_structure_ids: ["scout_tower"],
  final_refund_delta_log: ["gold:+90", "lumber:+30"],
  final_structure_health_percents: [54, 91],
  final_resource_spend_log: ["spent:140g:30l:guard", "repair:-45g:-20l"],
  final_command_queue: ["select_group_1", "blueprint:watch_tower@7,4", "build_site:7,4|7,5|8,4", "queue:build:watch_tower@7,4", "complete:watch_tower@7,4", "queue:complete:watch_tower@7,4", "repair:watch_tower@7,4", "queue:repair:watch_tower@7,4", "blueprint:scout_tower@8,4", "build_site:8,4|8,5|9,4", "queue:build:scout_tower@8,4", "cancel:scout_tower@8,4", "refund:gold:+90|lumber:+30", "queue:cancel:scout_tower@8,4"],
  non_background_pixels: 230400,
  build_blueprint_pixel_count: 1211,
  build_progress_pixel_count: 241,
  structure_complete_pixel_count: 172,
  structure_health_pixel_count: 76,
  repair_pixel_count: 472,
  cancel_refund_pixel_count: 143,
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
  fake_packet_artifact_count: 106,
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
  fake_packet_artifact_count: 106,
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
  preview_width: 1280,
  preview_height: 1080,
  preview_format: "ppm_p3_rgb",
  write_gate: true,
  input_path: "apply_live_native_action_with_source(classic_rts_projectile_ability_input)",
  input_action_count: 5,
  accepted_input_count: 5,
  action_labels: ["RTS:SELECT:1", "RTS:MOVE:8,4:wedge", "RTS:ATTACK:arena_creep_attack", "RTS:ABILITY:focus_fire", "RTS:ABILITY:guard_break"],
  final_active_projectile_id: "guard_break_bolt",
  final_projectile_trail_tile_ids: ["5,4", "5,5", "6,5", "6,6"],
  final_projectile_impact_tile_id: "6,5",
  final_ability_effect_tile_ids: ["6,4", "6,5", "7,5", "6,6"],
  final_ability_damage_ticks: [16, 21, 35],
  final_target_health_percent: 18,
  final_target_armor_percent: 18,
  final_target_shield_percent: 0,
  final_ability_resolution_state: "resolved:guard_break:arena_creep_attack",
  final_command_queue: ["select_group_1", "move:8,4:wedge", "attack:arena_creep_attack", "ability:focus_fire", "ability:guard_break", "damage_ticks:16+21+35", "armor_shield:18:0"],
  final_combat_event_log: ["projectile_launch:guard_break_bolt", "projectile_impact:guard_break:arena_creep_attack", "shield_broken", "damage:72"],
  non_background_pixels: 1382400,
  projectile_trail_pixel_count: 188,
  projectile_impact_pixel_count: 160,
  ability_radius_pixel_count: 260,
  damage_tick_pixel_count: 72,
  armor_shield_pixel_count: 48,
  attack_feedback_pixel_count: 256,
  live_projectile_ability_input_gate: true,
  projectile_trail_gate: true,
  projectile_impact_gate: true,
  ability_radius_gate: true,
  damage_tick_gate: true,
  armor_shield_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$projectile_ability_json"
add_artifact_from_path native_bevy_classic_rts_projectile_ability "Native/Bevy classic RTS projectile/ability" "$projectile_ability_json" release_review_input

projectile_ability_ppm="$TMP_DIR/bevy-classic-rts-projectile-ability.ppm"
printf 'P3\n1280 1080\n255\n' >"$projectile_ability_ppm"
truncate -s 8000001 "$projectile_ability_ppm"
add_artifact_from_path native_bevy_classic_rts_projectile_ability_ppm "Native/Bevy classic RTS projectile/ability PPM" "$projectile_ability_ppm" release_review_visual_evidence

"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh" >"$TMP_DIR/first-minute-command-feedback-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh" >"$TMP_DIR/first-minute-command-feedback-rejection-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh" >"$TMP_DIR/bot-planner-action-executor.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh" >"$TMP_DIR/bot-planner-executor-replay-determinism.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh" >"$TMP_DIR/multi-match-bot-executor-evaluation.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh" >"$TMP_DIR/bot-executor-failure-recovery-matrix.log"

first_minute_dir="$ROOT/acceptance/S5_native_bevy_device/latest"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay "Native/Bevy first-minute command feedback replay" "$first_minute_dir/bevy-first-minute-command-feedback-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_source_recording "Native/Bevy first-minute command feedback source recording" "$first_minute_dir/bevy-first-minute-command-feedback-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_recording "Native/Bevy first-minute command feedback command recording" "$first_minute_dir/bevy-first-minute-command-feedback-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_replay_ppm "Native/Bevy first-minute command feedback replay PPM" "$first_minute_dir/bevy-first-minute-command-feedback-replay.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay "Native/Bevy first-minute command feedback rejection replay" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_source_recording "Native/Bevy first-minute command feedback rejection source recording" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_recording "Native/Bevy first-minute command feedback rejection recording" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay_ppm "Native/Bevy first-minute command feedback rejection replay PPM" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-replay.ppm" release_review_visual_evidence

action_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-action-executor.bad.json"
jq '
  .accepted_action_count = 5
  | .command_marker_hit_count = 5
  | .action_labels[1] = "RTS:QUEUE:recon:sweep:wrong_tile@0,0"
  | .final_runtime_summary.objective_capture_percent = 50
' "$first_minute_dir/bevy-classic-rts-bot-planner-action-executor.json" >"$action_summary_json"
add_artifact_from_path native_bevy_bot_planner_action_executor "Native/Bevy bot planner action executor" "$action_summary_json" release_review_input

action_log_json="$TMP_DIR/bot-planner-action-executor.actions.bad.json"
jq '
  .accepted_action_count = 5
  | .command_marker_hit_count = 5
  | .execution_log[0].accepted = false
  | .execution_log[0].command_marker_hit = false
  | .execution_log[0].action_label = "RTS:QUEUE:faction:wrong_guard"
' "$first_minute_dir/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.actions.json" >"$action_log_json"
add_artifact_from_path native_bevy_bot_planner_action_executor_log "Native/Bevy bot planner action executor log" "$action_log_json" release_review_recording

action_ppm_path="$TMP_DIR/bot-planner-action-executor.bad.ppm"
printf 'P3\n1279 1080\n255\n' >"$action_ppm_path"
truncate -s 8000001 "$action_ppm_path"
add_artifact_from_path native_bevy_bot_planner_action_executor_ppm "Native/Bevy bot planner action executor PPM" "$action_ppm_path" release_review_visual_evidence

replay_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-executor-replay-determinism.bad.json"
jq '
  .accepted_replay_action_count = 5
  | .command_delta_match_count = 5
  | .runtime_determinism_gate = false
  | .replay_final_runtime_sha256 = "bad-runtime-sha"
' "$first_minute_dir/bevy-classic-rts-bot-planner-executor-replay-determinism.json" >"$replay_summary_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism "Native/Bevy bot planner executor replay determinism" "$replay_summary_json" release_review_input

replay_log_json="$TMP_DIR/bot-planner-executor-replay-determinism.replay.bad.json"
jq '
  .accepted_replay_action_count = 5
  | .command_delta_match_count = 5
  | .replay_final_runtime_sha256 = "bad-runtime-sha"
  | .execution_log[0].accepted = false
  | .execution_log[0].command_delta_match = false
' "$first_minute_dir/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.replay.json" >"$replay_log_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_log "Native/Bevy bot planner executor replay determinism log" "$replay_log_json" release_review_recording

replay_ppm_path="$TMP_DIR/bot-planner-executor-replay-determinism.bad.ppm"
printf 'P3\n1280 1079\n255\n' >"$replay_ppm_path"
truncate -s 8000001 "$replay_ppm_path"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_ppm "Native/Bevy bot planner executor replay determinism PPM" "$replay_ppm_path" release_review_visual_evidence

multi_summary_json="$TMP_DIR/bevy-classic-rts-multi-match-bot-executor-evaluation.bad.json"
jq '
  .variant_count = 3
  | .accepted_variant_count = 3
  | .total_accepted_action_count = 23
  | .total_command_delta_match_count = 23
  | .runtime_sha_match_count = 3
  | .variant_map_values = ["forest_relay", "ridge_watch", "marsh_gate"]
' "$first_minute_dir/bevy-classic-rts-multi-match-bot-executor-evaluation.json" >"$multi_summary_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation "Native/Bevy multi-match bot executor evaluation" "$multi_summary_json" release_review_input

multi_log_json="$TMP_DIR/multi-match-bot-executor-evaluation.matches.bad.json"
jq '
  .variant_count = 3
  | .accepted_variant_count = 3
  | .total_accepted_action_count = 23
  | .total_command_delta_match_count = 23
  | .runtime_sha_match_count = 3
  | .variant_summaries[0].accepted_action_count = 5
  | .variant_summaries[0].command_delta_match_count = 5
  | .variant_summaries[0].runtime_sha_match = false
' "$first_minute_dir/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.matches.json" >"$multi_log_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_log "Native/Bevy multi-match bot executor evaluation log" "$multi_log_json" release_review_recording

multi_ppm_path="$TMP_DIR/multi-match-bot-executor-evaluation.bad.ppm"
printf 'P3\n1279 720\n255\n' >"$multi_ppm_path"
truncate -s 8000001 "$multi_ppm_path"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_ppm "Native/Bevy multi-match bot executor evaluation PPM" "$multi_ppm_path" release_review_visual_evidence

add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix "Native/Bevy bot executor failure recovery matrix" "$first_minute_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix.json" release_review_input
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_log "Native/Bevy bot executor failure recovery matrix log" "$first_minute_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.matrix.json" release_review_recording
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_ppm "Native/Bevy bot executor failure recovery matrix PPM" "$first_minute_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.ppm" release_review_visual_evidence

bot_decision_gap_json="$TMP_DIR/bevy-classic-rts-bot-decision-state-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_decision_state_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_decision_gap_state: "bevy_bot_decision_vocabulary_not_openra_native_bot_ai",
  bevy_native_bot_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  bot_decision_stage_count: 6,
  stage_summaries: [
    {stage: "economy_seed"},
    {stage: "scout_objectives"},
    {stage: "capture_beacon"},
    {stage: "tech_switch"},
    {stage: "defend_counter"},
    {stage: "attack_commit_with_counter_repath"}
  ],
  decision_signal_count: 18,
  economy_decision_count: 3,
  objective_decision_count: 4,
  combat_decision_count: 4,
  tech_decision_count: 2,
  final_bot_decision_state: "attack_commit_with_counter_repath",
  final_rts_ai_pressure_percent: 70,
  final_rts_defeat_risk_percent: 35,
  final_objective_capture_percent: 90,
  final_match_result_state: "bot_decision_gap:attack_commit_with_counter_repath",
  final_command_queue: ["decision:combat:attack_commit_with_counter_repath", "parity_claim:false"],
  final_army_production_batch_ids: ["batch:tech:signal+skimmer+bastion"],
  bot_decision_state_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_decision_gap_json"
add_artifact_from_path native_bevy_bot_decision_state_gap "Native/Bevy bot decision-state gap" "$bot_decision_gap_json" release_review_input

bot_decision_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-decision-state-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_decision_gap_ppm"
truncate -s 8000001 "$bot_decision_gap_ppm"
add_artifact_from_path native_bevy_bot_decision_state_gap_ppm "Native/Bevy bot decision-state gap PPM" "$bot_decision_gap_ppm" release_review_visual_evidence

bot_adaptive_gap_json="$TMP_DIR/bevy-classic-rts-bot-adaptive-build-order-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_adaptive_build_gap_state: "bevy_adaptive_build_order_vocabulary_not_openra_native_ai_planner",
  bevy_native_adaptive_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  adaptive_stage_count: 6,
  stage_summaries: [
    {stage: "opening_worker_split"},
    {stage: "scout_trigger_response"},
    {stage: "expand_or_defend_branch"},
    {stage: "tech_counter_switch"},
    {stage: "pressure_window_commit"},
    {stage: "retreat_rebuild_reattack"}
  ],
  adaptive_signal_count: 24,
  opening_build_order_count: 3,
  scout_trigger_count: 2,
  branch_switch_count: 3,
  counter_tech_switch_count: 2,
  pressure_window_count: 2,
  retreat_rebuild_count: 2,
  final_adaptive_state: "pressure_window_rebuild_reattack",
  final_rts_ai_pressure_percent: 70,
  final_rts_defeat_risk_percent: 20,
  final_objective_capture_percent: 90,
  final_match_result_state: "adaptive_build_gap:pressure_window_rebuild_reattack",
  final_command_queue: ["adaptive_stage:retreat_rebuild_reattack", "native_openra_ai_planner:false"],
  final_army_production_batch_ids: ["build_order:signal_array_into_skimmer", "build_order:pullback_rebuild_then_reattack"],
  adaptive_build_order_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_adaptive_gap_json"
add_artifact_from_path native_bevy_bot_adaptive_build_order_gap "Native/Bevy bot adaptive build-order gap" "$bot_adaptive_gap_json" release_review_input

bot_adaptive_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-adaptive-build-order-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_adaptive_gap_ppm"
truncate -s 8000001 "$bot_adaptive_gap_ppm"
add_artifact_from_path native_bevy_bot_adaptive_build_order_gap_ppm "Native/Bevy bot adaptive build-order gap PPM" "$bot_adaptive_gap_ppm" release_review_visual_evidence

bot_tactical_micro_gap_json="$TMP_DIR/bevy-classic-rts-bot-tactical-micro-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_tactical_micro_gap_state: "bevy_tactical_micro_vocabulary_not_openra_native_combat_ai",
  bevy_native_combat_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  micro_stage_count: 6,
  stage_summaries: [
    {stage: "target_priority_probe"},
    {stage: "focus_fire_commit"},
    {stage: "kite_and_stutter_step"},
    {stage: "flank_angle_split"},
    {stage: "ability_timing_window"},
    {stage: "low_health_pullback_regroup"}
  ],
  micro_signal_count: 24,
  target_swap_count: 3,
  focus_fire_order_count: 3,
  kite_step_count: 3,
  flank_angle_count: 2,
  ability_timing_count: 2,
  low_health_pullback_count: 2,
  final_micro_state: "pullback_regroup_reattack",
  final_rts_ai_pressure_percent: 70,
  final_rts_defeat_risk_percent: 20,
  final_objective_capture_percent: 90,
  final_match_result_state: "tactical_micro_gap:pullback_regroup_reattack",
  final_command_queue: ["micro_stage:low_health_pullback_regroup", "native_openra_combat_ai:false"],
  final_army_production_batch_ids: ["micro_control:focus_fire_low_armor_striker", "micro_control:pull_redline_units_regroup_reattack"],
  tactical_micro_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_tactical_micro_gap_json"
add_artifact_from_path native_bevy_bot_tactical_micro_gap "Native/Bevy bot tactical micro gap" "$bot_tactical_micro_gap_json" release_review_input

bot_tactical_micro_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-tactical-micro-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_tactical_micro_gap_ppm"
truncate -s 8000001 "$bot_tactical_micro_gap_ppm"
add_artifact_from_path native_bevy_bot_tactical_micro_gap_ppm "Native/Bevy bot tactical micro gap PPM" "$bot_tactical_micro_gap_ppm" release_review_visual_evidence

bot_map_intel_gap_json="$TMP_DIR/bevy-classic-rts-bot-map-intel-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_map_intel_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_map_intel_gap_state: "bevy_map_intel_vocabulary_not_openra_native_shroud_memory_ai",
  bevy_native_shroud_memory_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  intel_stage_count: 6,
  stage_summaries: [
    {stage: "initial_scout_sweep"},
    {stage: "fog_memory_stamp"},
    {stage: "expansion_threat_inference"},
    {stage: "enemy_tech_read"},
    {stage: "hidden_army_prediction"},
    {stage: "rotate_pressure_reveal"}
  ],
  intel_signal_count: 24,
  scout_sweep_count: 3,
  fog_memory_stamp_count: 4,
  expansion_threat_count: 3,
  enemy_tech_read_count: 2,
  hidden_army_prediction_count: 2,
  pressure_rotation_count: 2,
  final_intel_state: "rotate_pressure_confirmed_beacon",
  final_rts_ai_pressure_percent: 80,
  final_rts_defeat_risk_percent: 20,
  final_objective_capture_percent: 90,
  final_match_result_state: "map_intel_gap:rotate_pressure_confirmed_beacon",
  final_command_queue: ["intel_stage:rotate_pressure_reveal", "native_openra_shroud_memory_ai:false"],
  final_army_production_batch_ids: ["map_intel:fog_memory_last_seen_grid", "map_intel:rotate_pressure_to_confirmed_beacon"],
  map_intel_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_map_intel_gap_json"
add_artifact_from_path native_bevy_bot_map_intel_gap "Native/Bevy bot map intel gap" "$bot_map_intel_gap_json" release_review_input

bot_map_intel_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-map-intel-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_map_intel_gap_ppm"
truncate -s 8000001 "$bot_map_intel_gap_ppm"
add_artifact_from_path native_bevy_bot_map_intel_gap_ppm "Native/Bevy bot map intel gap PPM" "$bot_map_intel_gap_ppm" release_review_visual_evidence

production_desktop_review_packet_json="$TMP_DIR/bevy-classic-rts-production-desktop-review-packet.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_production_desktop_review_packet_v1", status: "classic_rts_production_desktop_review_packet_green", green: true, source_contracts: {production_interaction_polish: "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1", desktop_playtest_review_packet: "trillionnium_world_bevy_desktop_playtest_review_packet_v1", desktop_real_machine_readiness: "trillionnium_world_bevy_desktop_real_machine_readiness_v1"}, gates: {production_interaction_polish_gate: true, desktop_playtest_review_packet_gate: true, desktop_real_machine_readiness_gate: true, keyboard_visual_review_gate: true, mouse_visual_review_gate: true, artifact_manifest_gate: true, production_to_desktop_review_gate: true, desktop_before_mobile_gate: true, android_s5_real_device_not_claimed_gate: true, public_launch_not_claimed_gate: true}, production_review_summary: {interaction_surface_count: 6, drag_select_skin_pixel_count: 9980, right_click_move_skin_pixel_count: 9980, attack_lock_skin_pixel_count: 9980, build_ghost_skin_pixel_count: 9980, queue_path_skin_pixel_count: 9700, scroll_minimap_skin_pixel_count: 9700}, desktop_review_summary: {screenshot_frame_count: 11, keyboard_event_count: 13, mouse_event_count: 15, mouse_slot_a_bytes: 41520}, artifact_manifest: [{label: "production_interaction_polish", path: "fixture", sha256: "fixture", bytes: 1}, {label: "production_interaction_polish_preview", path: "fixture", sha256: "fixture", bytes: 1}, {label: "desktop_playtest_review_packet", path: "fixture", sha256: "fixture", bytes: 1}, {label: "desktop_real_machine_readiness", path: "fixture", sha256: "fixture", bytes: 1}, {label: "live_window_screenshot_sequence", path: "fixture", sha256: "fixture", bytes: 1}, {label: "live_window_mouse_hit_test_sequence", path: "fixture", sha256: "fixture", bytes: 1}], no_credit_boundaries: {android_s5_real_device_claimed: false, public_launch_ready_claimed: false, production_ready_desktop_review_shipped: false, desktop_review_scope: "local_linux_desktop_x11_window_keyboard_mouse_with_production_interaction_polish"}}' >"$production_desktop_review_packet_json"
add_artifact_from_path native_bevy_classic_rts_production_desktop_review_packet "Native/Bevy classic RTS production desktop review packet" "$production_desktop_review_packet_json" release_review_input

full_screen_ui_replication_json="$TMP_DIR/bevy-classic-rts-full-screen-ui-replication.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1", status: "classic_rts_full_screen_ui_replication_green", green: true, preview_width: 1280, preview_height: 768, preview_format: "ppm_p3_rgb", replication_surface_count: 10, replication_surface_names: ["TITLE/CAMPAIGN ENTRY", "TACTICAL VIEWPORT", "MAP/MINIMAP CAMERA", "PRODUCTION HUD SKIN", "COMMAND INTERACTIONS", "BUILD + TECH TREE", "UNIT STATUS CARD", "ABILITY/COMBAT UI", "CAMPAIGN OUTCOME", "OPEN-WORLD HANDOFF"], source_contracts: {campaign_entry: "trillionnium_world_bevy_classic_rts_campaign_entry_v1", visual_fidelity: "trillionnium_world_bevy_classic_rts_visual_fidelity_v1", map_ui_modeling_readiness: "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1", production_ui_skin: "trillionnium_world_bevy_classic_rts_production_ui_skin_v1", production_interaction_polish: "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1", build_lifecycle: "trillionnium_world_bevy_classic_rts_build_lifecycle_v1", tech_tree: "trillionnium_world_bevy_classic_rts_tech_tree_v1", campaign_outcome_ui_readiness: "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1", combat_readability_pressure_readiness: "trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1"}, screen_matrix_pixel_counts: {board: 381132, title_campaign: 8340, tactical_viewport: 8340, map_minimap: 7876, production_hud_skin: 7876, command_interaction: 7876, build_tech: 7876, combat_overlay: 7876, campaign_outcome: 7876}, title_campaign_gate: true, tactical_viewport_gate: true, map_minimap_gate: true, production_skin_gate: true, interaction_polish_gate: true, build_tech_gate: true, combat_ui_gate: true, campaign_outcome_gate: true, source_policy_gate: true, replication_preview_gate: true, source_preview_gate: true, full_screen_ui_replication_gate: true, internal_full_screen_ui_replication_claimed: true, external_evidence_ignored_for_current_replication_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false, warcraft_iii_asset_copied: false, openra_asset_copied: false, third_party_asset_copied: false}' >"$full_screen_ui_replication_json"
add_artifact_from_path native_bevy_classic_rts_full_screen_ui_replication "Native/Bevy classic RTS full screen/UI replication" "$full_screen_ui_replication_json" release_review_input
shell_meta_ui_replication_json="$TMP_DIR/bevy-classic-rts-shell-meta-ui-replication.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1", status: "classic_rts_shell_meta_ui_replication_green", green: true, preview_width: 1280, preview_height: 768, preview_format: "ppm_p3_rgb", shell_meta_surface_count: 12, shell_meta_surface_names: ["TITLE / ACCOUNT", "CHARACTER CREATE", "SESSION SLOT MENU", "SAVE SLOT FILE", "SAVE / LOAD CONFIRM", "LOAD / RESUME CTA", "SESSION RECOVERY", "PAUSE / RESUME", "SETTINGS", "INPUT HUD", "BUTTON HIT TEST", "FIRST-MINUTE HANDOFF"], source_contracts: {account_title_flow: "trillionnium_world_bevy_account_title_flow_v1", character_create: "trillionnium_world_bevy_character_create_v1", first_minute_onboarding: "trillionnium_world_bevy_first_minute_onboarding_v1", full_screen_ui_replication: "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1", input_telemetry_hud: "trillionnium_world_bevy_input_telemetry_hud_v1", pause_menu: "trillionnium_world_bevy_pause_menu_v1", session_load_resume: "trillionnium_world_bevy_session_load_resume_v1", session_recovery_ui: "trillionnium_world_bevy_session_recovery_ui_v1", session_save_slot: "trillionnium_world_bevy_session_save_slot_v1", session_slot_confirm: "trillionnium_world_bevy_session_slot_confirm_v1", session_slot_menu: "trillionnium_world_bevy_session_slot_menu_v1", settings_menu: "trillionnium_world_bevy_settings_menu_v1", title_menu: "trillionnium_world_bevy_title_menu_v1", visible_button_hit_test_map: "trillionnium_world_bevy_visible_button_hit_test_map_v1"}, shell_meta_pixel_counts: {account_title: 5872, board: 398212, button_hit_test: 5872, character_create: 5872, first_minute_handoff: 5872, highlight: 6336, input_hud: 5872, load_resume_cta: 5872, pause_resume: 5872, save_load_confirm: 5872, save_slot_file: 5872, session_recovery: 5872, session_slot_menu: 5872, settings: 5872}, full_screen_ui_replication_gate: true, account_title_gate: true, title_menu_gate: true, character_create_gate: true, session_slot_menu_gate: true, session_save_slot_gate: true, session_slot_confirm_gate: true, session_load_resume_gate: true, session_recovery_gate: true, pause_menu_gate: true, settings_menu_gate: true, input_hud_gate: true, visible_hit_test_gate: true, first_minute_onboarding_gate: true, source_preview_gate: true, shell_meta_preview_gate: true, shell_meta_ui_replication_gate: true, no_external_boundary_gate: true, internal_shell_meta_ui_replication_claimed: true, external_evidence_ignored_for_current_replication_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false, warcraft_iii_asset_copied: false, openra_asset_copied: false, third_party_asset_copied: false}' >"$shell_meta_ui_replication_json"
add_artifact_from_path native_bevy_classic_rts_shell_meta_ui_replication "Native/Bevy classic RTS shell/meta UI replication" "$shell_meta_ui_replication_json" release_review_input
match_setup_ui_replication_json="$TMP_DIR/bevy-classic-rts-match-setup-ui-replication.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1", status: "classic_rts_match_setup_ui_replication_green", green: true, preview_width: 1280, preview_height: 768, preview_format: "ppm_p3_rgb", setup_surface_count: 10, setup_surface_names: ["CAMPAIGN ACTIONS", "MAP SELECT", "FACTION SELECT", "SPAWN SLOTS", "RESOURCE RULES", "BOT / DIFFICULTY", "VICTORY CONDITIONS", "MINIMAP PREVIEW", "START READY", "NO-EXTERNAL BOUNDARY"], source_contracts: {shell_meta_ui_replication: "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1", campaign_entry: "trillionnium_world_bevy_classic_rts_campaign_entry_v1", first_contact_basin_spec: "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1", map_ui_modeling_readiness: "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1", tech_tree: "trillionnium_world_bevy_classic_rts_tech_tree_v1"}, setup_pixel_counts: {board: 381079, map_select: 8340, faction_select: 8340, start_ready: 7876}, source_headline: {shell_meta_surface_count: 12, campaign_input_action_count: 73, map_id: "first_contact_basin", map_spawn_count: 4, map_actor_count: 39, map_ui_preview_count: 6, faction_id: "mirror_guard", tech_state: "unlocked:relay_guard"}, shell_meta_gate: true, campaign_entry_gate: true, map_spec_gate: true, map_ui_gate: true, faction_gate: true, no_external_boundary_gate: true, setup_preview_gate: true, source_preview_gate: true, match_setup_ui_replication_gate: true, internal_match_setup_ui_replication_claimed: true, external_evidence_ignored_for_current_replication_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false, warcraft_iii_asset_copied: false, openra_asset_copied: false, third_party_asset_copied: false}' >"$match_setup_ui_replication_json"
add_artifact_from_path native_bevy_classic_rts_match_setup_ui_replication "Native/Bevy classic RTS match setup UI replication" "$match_setup_ui_replication_json" release_review_input
campaign_outcome_ui_readiness_json="$TMP_DIR/bevy-classic-rts-campaign-outcome-ui-readiness.json"
jq -n '{contract_version: "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1", status: "classic_rts_campaign_outcome_ui_readiness_green", green: true, preview_count: 5, source_contracts: {first_minute_readiness: "trillionnium_world_bevy_classic_rts_first_minute_readiness_v1", objective_victory_loop: "trillionnium_world_bevy_classic_rts_objective_victory_loop_v1", base_assault_resolution: "trillionnium_world_bevy_classic_rts_base_assault_resolution_v1", battle_aftermath: "trillionnium_world_bevy_classic_rts_battle_aftermath_v1", open_world_after_action: "trillionnium_world_bevy_classic_rts_open_world_after_action_v1"}, first_minute_gate: true, objective_victory_gate: true, base_assault_gate: true, battle_aftermath_gate: true, open_world_return_gate: true, native_boundary_gate: true, preview_gate: true, campaign_flow: ["TITLE campaign entry", "objective claim/extract victory", "battle aftermath rewards", "open-world route resume"], first_minute_summary: {input_action_count: 73, final_room: "league-coliseum", final_objective_status: "open_world_after_action_ready"}, victory_summary: {accepted_input_count: 6, final_objective_capture_percent: 100, final_objective_result_state: "victory:relay_beacon_extracted", final_defeat_risk_percent: 4}, base_assault_summary: {accepted_input_count: 9, final_base_breach_percent: 100, final_base_assault_result_state: "breached:enemy_barracks"}, aftermath_summary: {accepted_input_count: 12, final_match_result_state: "victory_ready:secure_expansion", final_growth_level: 2, final_next_action_ids: ["secure_expansion"]}, open_world_summary: {accepted_input_count: 3, final_current_room_id: "league-coliseum", final_map_scene: "arena_outdoor", final_open_world_handoff_state: "resumed:league-coliseum"}, internal_campaign_outcome_ui_readiness_claimed: true, external_evidence_ignored_for_current_outcome_pass: true, android_s5_real_device_claimed: false, public_launch_ready: false, screen_for_screen_openra_ui_claimed: false, openra_engine_port_claimed: false}' >"$campaign_outcome_ui_readiness_json"
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
  hud_pixel_counts: {non_background: 894510, resources: 1240, selection: 1240, command_grid: 1240, minimap: 1240, production: 1240, abilities: 1240, combat_alerts: 1240, objective: 1240, highlight: 960},
  selection_gate: true,
  command_gate: true,
  resource_gate: true,
  production_gate: true,
  ability_gate: true,
  combat_alert_gate: true,
  minimap_objective_gate: true,
  native_client_boundary_gate: true,
  preview_gate: true,
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
add_artifact_from_path native_bevy_classic_rts_in_match_hud_state_replication "Native/Bevy classic RTS in-match HUD/state replication" "$in_match_hud_state_replication_json" release_review_input


session_state_continuity_json="$TMP_DIR/bevy-classic-rts-session-state-continuity.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_session_state_continuity_v1",
  status: "classic_rts_session_state_continuity_green",
  green: true,
  preview_width: 1280,
  preview_height: 768,
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
  state_continuity_surface_count: 8,
  state_continuity_surface_names: ["MATCH SETUP SNAPSHOT", "SESSION SLOT WRITE", "LOAD RESUME LOCK", "CONTINUE UNLOCK", "IN-MATCH HUD RESTORE", "OUTCOME REWARD STATE", "OPEN-WORLD RESUME", "RECOVERY UI GUARD"],
  resume_chain: ["match_setup_saved", "slot_a_written", "load_resume_locked", "continue_unlocked", "in_match_hud_restored", "campaign_outcome_saved", "open_world_resumed"],
  source_headline: {
    load_resume_final_objective_status: "first_playable_loop_complete",
    campaign_outcome_open_world_state: "resumed:league-coliseum",
    campaign_continuity_restored_room_id: "league-coliseum",
    load_resume_slot_a_bytes: 46253
  },
  state_continuity_pixel_counts: {non_background: 983040, in_match_hud_restore: 3030, open_world_resume: 3030},
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
  source_preview_gate: true,
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
  echo "[FAIL] packet integrity bot executor semantic fixture unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi

if [[ ! -f "$summary_json" ]]; then
  echo "[FAIL] packet integrity bot executor semantic fixture did not write summary" >&2
  exit 1
fi

jq -e '
  .status == "release_review_packet_integrity_blocked"
  and .green == false
  and (.failures | length) == 9
  and ([.failures[].name] | index("bot_planner_action_executor_semantics"))
  and ([.failures[].name] | index("bot_planner_action_executor_log_semantics"))
  and ([.failures[].name] | index("bot_planner_action_executor_ppm_semantics"))
  and ([.failures[].name] | index("bot_planner_executor_replay_determinism_semantics"))
  and ([.failures[].name] | index("bot_planner_executor_replay_determinism_log_semantics"))
  and ([.failures[].name] | index("bot_planner_executor_replay_determinism_ppm_semantics"))
  and ([.failures[].name] | index("multi_match_bot_executor_evaluation_semantics"))
  and ([.failures[].name] | index("multi_match_bot_executor_evaluation_log_semantics"))
  and ([.failures[].name] | index("multi_match_bot_executor_evaluation_ppm_semantics"))
  and (([.failures[].detail] | index("sha256_mismatch")) == null)
  and (([.failures[].detail] | index("bytes_mismatch")) == null)
  and (([.failures[].detail] | index("contract_mismatch")) == null)
  and (([.failures[].detail] | index("status_mismatch")) == null)
' "$summary_json" >/dev/null

echo "[PASS] release review packet integrity rejects semantically invalid bot executor source-chain artifacts even when checksums match"
