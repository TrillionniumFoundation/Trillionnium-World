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


replay_json="$TMP_DIR/bevy-first-minute-command-feedback-replay.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_replay_v1",
  green: true,
  command_input_action_count: 7,
  accepted_command_input_count: 6,
  first_minute_replay_gate: true,
  command_recording_parse_gate: true,
  live_command_input_gate: true,
  scene_renderer_gate: true,
  history_entry_count: 3,
  history_capacity: 3,
  retained_history_group_ids: ["26", "27", "25"],
  pruned_history_group_ids: ["24"],
  cleared_active_stale_pixel_count: 12,
  preview_width: 1280,
  preview_height: 720,
  android_s5_real_device_claimed: false
}' >"$replay_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay "Native/Bevy first-minute command feedback replay" "$replay_json" release_review_input

source_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-source-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_timeline_contract: "trillionnium_world_bevy_first_minute_interaction_timeline_v1",
  source_timeline_green: false,
  steps: [range(0; 9) | {action_label: ("STEP:" + (tostring))}],
  android_s5_real_device_claimed: false
}' >"$source_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_source_recording "Native/Bevy first-minute command feedback source recording" "$source_recording_json" release_review_recording

command_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_recording_v1",
  source_input_replay_contract: "trillionnium_world_bevy_first_minute_input_replay_v1",
  source_input_recording_contract: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_input_replay_green: true,
  command_history_capacity: 3,
  retained_history_group_ids: ["26", "27", "25"],
  pruned_history_group_ids: ["24"],
  steps: [
    {action_label: "RTS:SELECT:26"},
    {action_label: "RTS:MOVE:18,31:line"},
    {action_label: "RTS:SELECT:27"},
    {action_label: "RTS:MOVE:99,99:line"},
    {action_label: "RTS:SELECT:28"},
    {action_label: "RTS:MOVE:1,31:line"},
    {action_label: "RTS:SELECT:26"}
  ],
  android_s5_real_device_claimed: false
}' >"$command_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_recording "Native/Bevy first-minute command feedback command recording" "$command_recording_json" release_review_recording

ppm_path="$TMP_DIR/bevy-first-minute-command-feedback-replay.ppm"
printf 'P3\n1279 720\n255\n' >"$ppm_path"
truncate -s 8000001 "$ppm_path"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay_ppm "Native/Bevy first-minute command feedback replay PPM" "$ppm_path" release_review_visual_evidence

rejection_replay_json="$TMP_DIR/bevy-first-minute-command-feedback-rejection-replay.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_rejection_replay_v1",
  green: true,
  command_input_action_count: 7,
  accepted_command_input_count: 1,
  blocked_command_input_count: 6,
  blocked_reasons: [
    "rts_group_selection_required",
    "rts_invalid_tile:bad-tile",
    "rts_attack_target_required",
    "rts_attack_required_before_ability",
    "rts_queue_id_required",
    "rts_group_id_required"
  ],
  command_queue_rejection_pollution_count: 0,
  first_minute_replay_gate: true,
  rejection_recording_parse_gate: true,
  command_action_parse_gate: true,
  replay_expectation_gate: true,
  blocked_feedback_gate: true,
  blocked_action_history_gate: true,
  blocked_history_non_pollution_gate: true,
  history_entry_count: 3,
  history_capacity: 3,
  retained_history_group_ids: ["26", "27", "28"],
  pruned_history_group_ids: ["25", "24"],
  cleared_active_stale_pixel_count: 0,
  preview_width: 1280,
  preview_height: 720,
  android_s5_real_device_claimed: false
}' >"$rejection_replay_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay "Native/Bevy first-minute command feedback rejection replay" "$rejection_replay_json" release_review_input

rejection_source_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-rejection-source-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_timeline_contract: "trillionnium_world_bevy_first_minute_interaction_timeline_v1",
  source_timeline_green: true,
  steps: [range(0; 10) | {action_label: ("STEP:" + (tostring))}],
  android_s5_real_device_claimed: false
}' >"$rejection_source_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_source_recording "Native/Bevy first-minute command feedback rejection source recording" "$rejection_source_recording_json" release_review_recording

rejection_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-rejection-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_rejection_recording_v1",
  source_input_replay_contract: "trillionnium_world_bevy_first_minute_input_replay_v1",
  source_input_recording_contract: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_input_replay_green: true,
  command_history_capacity: 3,
  retained_history_group_ids: ["26", "27", "28"],
  pruned_history_group_ids: ["25", "24"],
  steps: [
    {action_label: "RTS:MOVE:18,31:line", expected_accepted: false, expected_reason: "rts_group_selection_required"},
    {action_label: "RTS:SELECT:26", expected_accepted: true},
    {action_label: "RTS:MOVE:bad-tile:line", expected_accepted: false, expected_reason: "rts_invalid_tile:bad-tile"},
    {action_label: "RTS:ATTACK:", expected_accepted: false, expected_reason: "rts_attack_target_required"},
    {action_label: "RTS:ABILITY:guard_break", expected_accepted: false, expected_reason: "rts_attack_required_before_ability"},
    {action_label: "RTS:QUEUE:", expected_accepted: false, expected_reason: "rts_queue_id_required"},
    {action_label: "RTS:SELECT:", expected_accepted: false, expected_reason: "rts_group_id_required"}
  ],
  android_s5_real_device_claimed: false
}' >"$rejection_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_recording "Native/Bevy first-minute command feedback rejection recording" "$rejection_recording_json" release_review_recording

rejection_ppm_path="$TMP_DIR/bevy-first-minute-command-feedback-rejection-replay.ppm"
printf 'P3\n1280 720\n255\n' >"$rejection_ppm_path"
truncate -s 8000001 "$rejection_ppm_path"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay_ppm "Native/Bevy first-minute command feedback rejection replay PPM" "$rejection_ppm_path" release_review_visual_evidence

action_log_json="$TMP_DIR/bot-planner-action-executor.actions.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1",
  executor_action_count: 6,
  accepted_action_count: 6,
  command_marker_hit_count: 6,
  input_source: "classic_rts_bot_planner_action_executor_input",
  planner_live_decision_log_sha256: "1111111111111111111111111111111111111111111111111111111111111111",
  planner_strategy_checksum_sha256: "2222222222222222222222222222222222222222222222222222222222222222",
  execution_log: [
    {accepted: true, action_label: "RTS:QUEUE:faction:mirror_guard", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:recon:sweep:watchtower_scan@7,4", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:objective:claim:relay_beacon@6,5", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:tier2:push:gate_bulwark@10,3", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:tier2:finish:gate_bulwark@10,3", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"}
  ]
}' >"$action_log_json"
add_artifact_from_path native_bevy_bot_planner_action_executor_log "Native/Bevy bot planner action executor log" "$action_log_json" release_review_recording

action_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-action-executor.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1",
  green: true,
  bot_planner_action_executor_gate: true,
  executor_action_count: 6,
  accepted_action_count: 6,
  command_marker_hit_count: 6,
  action_labels: [
    "RTS:QUEUE:faction:mirror_guard",
    "RTS:QUEUE:recon:sweep:watchtower_scan@7,4",
    "RTS:QUEUE:objective:claim:relay_beacon@6,5",
    "RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost",
    "RTS:QUEUE:tier2:push:gate_bulwark@10,3",
    "RTS:QUEUE:tier2:finish:gate_bulwark@10,3"
  ],
  input_sources: ["classic_rts_bot_planner_action_executor_input"],
  final_runtime_summary: {
    faction_id: "mirror_guard",
    objective_capture_percent: 100,
    tier_two_tech_ids: ["relay_foundry"],
    siege_breach_state: "counterplay_won:gate_bulwark",
    match_result_state: "siege_breakthrough:inner_lane"
  },
  bevy_bot_planner_action_executor_claimed: true,
  bevy_openra_runtime_bot_executor_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false
}' >"$action_summary_json"
add_artifact_from_path native_bevy_bot_planner_action_executor "Native/Bevy bot planner action executor" "$action_summary_json" release_review_input

action_ppm_path="$TMP_DIR/bot-planner-action-executor.ppm"
printf 'P3\n1280 1080\n255\n' >"$action_ppm_path"
truncate -s 8000001 "$action_ppm_path"
add_artifact_from_path native_bevy_bot_planner_action_executor_ppm "Native/Bevy bot planner action executor PPM" "$action_ppm_path" release_review_visual_evidence

replay_log_json="$TMP_DIR/bot-planner-executor-replay-determinism.replay.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1",
  replay_action_count: 6,
  accepted_replay_action_count: 6,
  replay_command_marker_hit_count: 6,
  command_delta_match_count: 6,
  source_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  replay_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  source_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_input_source: "classic_rts_bot_planner_executor_replay_input",
  source_action_log_path: "/tmp/bot-planner-action-executor.actions.json",
  source_action_log_sha256: "5555555555555555555555555555555555555555555555555555555555555555",
  source_executor_summary_sha256: "6666666666666666666666666666666666666666666666666666666666666666",
  execution_log: [
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"}
  ]
}' >"$replay_log_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_log "Native/Bevy bot planner executor replay determinism log" "$replay_log_json" release_review_recording

replay_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-executor-replay-determinism.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1",
  green: true,
  bot_planner_executor_replay_determinism_gate: true,
  source_executor_action_count: 6,
  replay_action_count: 6,
  accepted_replay_action_count: 6,
  replay_command_marker_hit_count: 6,
  command_delta_match_count: 6,
  runtime_determinism_gate: true,
  source_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  replay_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  source_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_input_sources: ["classic_rts_bot_planner_executor_replay_input"],
  bevy_bot_planner_executor_replay_determinism_claimed: true,
  bevy_openra_runtime_bot_executor_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false
}' >"$replay_summary_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism "Native/Bevy bot planner executor replay determinism" "$replay_summary_json" release_review_input

replay_ppm_path="$TMP_DIR/bot-planner-executor-replay-determinism.ppm"
printf 'P3\n1280 1080\n255\n' >"$replay_ppm_path"
truncate -s 8000001 "$replay_ppm_path"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_ppm "Native/Bevy bot planner executor replay determinism PPM" "$replay_ppm_path" release_review_visual_evidence

multi_log_json="$TMP_DIR/multi-match-bot-executor-evaluation.matches.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1",
  variant_count: 4,
  accepted_variant_count: 4,
  total_replay_action_count: 24,
  total_accepted_action_count: 24,
  total_command_marker_hit_count: 24,
  total_command_delta_match_count: 24,
  runtime_sha_match_count: 4,
  command_queue_sha_match_count: 4,
  evaluation_input_source: "classic_rts_multi_match_bot_executor_evaluation_input",
  source_action_log_path: "/tmp/bot-planner-action-executor.actions.json",
  source_action_log_sha256: "5555555555555555555555555555555555555555555555555555555555555555",
  source_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  source_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  source_replay_dir: "/tmp/source-replay",
  variant_summaries: [
    {variant_id: "seed_2026052901_forest_relay", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "forest_relay", economy_variant: "balanced"},
    {variant_id: "seed_2026052902_ridge_watch", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "ridge_watch", economy_variant: "low_gold"},
    {variant_id: "seed_2026052903_marsh_gate", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "marsh_gate", economy_variant: "high_pressure"},
    {variant_id: "seed_2026052904_market_ruins", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "market_ruins", economy_variant: "delayed_tech"}
  ]
}' >"$multi_log_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_log "Native/Bevy multi-match bot executor evaluation log" "$multi_log_json" release_review_recording

multi_summary_json="$TMP_DIR/bevy-classic-rts-multi-match-bot-executor-evaluation.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1",
  green: true,
  multi_match_bot_executor_evaluation_gate: true,
  variant_count: 4,
  accepted_variant_count: 4,
  total_replay_action_count: 24,
  total_accepted_action_count: 24,
  total_command_marker_hit_count: 24,
  total_command_delta_match_count: 24,
  runtime_sha_match_count: 4,
  command_queue_sha_match_count: 4,
  variant_map_values: ["forest_relay", "ridge_watch", "marsh_gate", "market_ruins"],
  bevy_multi_match_bot_executor_evaluation_claimed: true,
  bevy_bot_planner_executor_replay_determinism_claimed: true,
  bevy_openra_runtime_bot_executor_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false
}' >"$multi_summary_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation "Native/Bevy multi-match bot executor evaluation" "$multi_summary_json" release_review_input

multi_ppm_path="$TMP_DIR/multi-match-bot-executor-evaluation.ppm"
printf 'P3\n1280 720\n255\n' >"$multi_ppm_path"
truncate -s 8000001 "$multi_ppm_path"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_ppm "Native/Bevy multi-match bot executor evaluation PPM" "$multi_ppm_path" release_review_visual_evidence

failure_matrix_log_json="$TMP_DIR/bot-executor-failure-recovery-matrix.matrix.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1",
  source_replay_action_count: 6,
  blocked_injection_count: 6,
  blocked_rejection_count: 6,
  blocked_expected_reason_count: 6,
  blocked_feedback_event_count: 6,
  blocked_command_queue_unchanged_count: 6,
  blocked_command_queue_sha_match_count: 6,
  blocked_input_source: "classic_rts_bot_executor_failure_recovery_matrix_blocked_input",
  recovery_input_source: "classic_rts_bot_executor_failure_recovery_matrix_recovery_input",
  recovery_action_count: 6,
  recovery_accepted_action_count: 6,
  recovery_command_marker_hit_count: 6,
  recovery_command_delta_match_count: 6,
  recovery_safe_runtime_sha_match: true,
  command_queue_sha_match: true,
  source_action_log_path: "/tmp/bot-planner-action-executor.actions.json",
  source_action_log_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  source_final_runtime_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  source_recovery_safe_runtime_sha256: "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
  source_command_queue_sha256: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
  source_multi_match_dir: "/tmp/source-multi-match",
  matrix_log: [
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_queue_id_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_group_id_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_attack_required_before_ability", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_invalid_tile:bad-tile", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_queue_id_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_attack_target_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}}
  ]
}' >"$failure_matrix_log_json"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_log "Native/Bevy bot executor failure recovery matrix log" "$failure_matrix_log_json" release_review_recording

failure_matrix_summary_json="$TMP_DIR/bevy-classic-rts-bot-executor-failure-recovery-matrix.json"
jq -n \
  --argjson matrix_log "$(jq -c '.matrix_log' "$failure_matrix_log_json")" \
  '{
    contract_version: "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1",
    green: true,
    bot_executor_failure_recovery_matrix_state: "bevy_executor_rejects_blocked_actions_and_recovers_without_command_queue_pollution_not_openra_runtime_bot",
    bot_executor_failure_recovery_matrix_gate: true,
    source_replay_action_count: 6,
    blocked_injection_count: 6,
    blocked_rejection_count: 6,
    blocked_expected_reason_count: 6,
    blocked_feedback_event_count: 6,
    blocked_command_queue_unchanged_count: 6,
    blocked_command_queue_sha_match_count: 6,
    blocked_reason_values: ["rts_queue_id_required", "rts_group_id_required", "rts_attack_required_before_ability", "rts_invalid_tile:bad-tile", "rts_attack_target_required"],
    blocked_input_sources: ["classic_rts_bot_executor_failure_recovery_matrix_blocked_input"],
    recovery_input_sources: ["classic_rts_bot_executor_failure_recovery_matrix_recovery_input"],
    recovery_action_count: 6,
    recovery_accepted_action_count: 6,
    recovery_command_marker_hit_count: 6,
    recovery_command_delta_match_count: 6,
    feedback_blocked_count: 6,
    feedback_recovery_count: 6,
    final_input_feedback_event_count: 12,
    recovery_safe_runtime_sha_match: true,
    command_queue_sha_match: true,
    matrix_log: $matrix_log,
    source_multi_match_summary: {
      variant_count: 4,
      total_replay_action_count: 24,
      total_accepted_action_count: 24,
      evaluation_log_sha256: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
    },
    final_recovery_safe_runtime_summary: {
      faction_id: "mirror_guard",
      objective_capture_percent: 100,
      tier_two_tech_ids: ["relay_foundry"],
      siege_breach_state: "counterplay_won:gate_bulwark",
      match_result_state: "siege_breakthrough:inner_lane"
    },
    bevy_bot_executor_failure_recovery_matrix_claimed: true,
    bevy_multi_match_bot_executor_evaluation_claimed: true,
    bevy_openra_runtime_bot_executor_claimed: false,
    android_s5_real_device_claimed: false,
    public_launch_ready: false
  }' >"$failure_matrix_summary_json"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix "Native/Bevy bot executor failure recovery matrix" "$failure_matrix_summary_json" release_review_input

failure_matrix_ppm_path="$TMP_DIR/bot-executor-failure-recovery-matrix.ppm"
printf 'P3\n1280 1080\n255\n' >"$failure_matrix_ppm_path"
truncate -s 8000001 "$failure_matrix_ppm_path"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_ppm "Native/Bevy bot executor failure recovery matrix PPM" "$failure_matrix_ppm_path" release_review_visual_evidence

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
  echo "[FAIL] packet integrity semantic fixture unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi

if [[ ! -f "$summary_json" ]]; then
  echo "[FAIL] packet integrity semantic fixture did not write summary" >&2
  exit 1
fi

jq -e '
  .status == "release_review_packet_integrity_blocked"
  and .green == false
  and (.failures | length) == 4
  and ([.failures[].name] | index("first_minute_command_feedback_replay_semantics"))
  and ([.failures[].name] | index("first_minute_command_feedback_source_recording_semantics"))
  and ([.failures[].name] | index("first_minute_command_feedback_recording_semantics"))
  and ([.failures[].name] | index("first_minute_command_feedback_replay_ppm_semantics"))
  and (([.failures[].detail] | index("sha256_mismatch")) == null)
  and (([.failures[].detail] | index("bytes_mismatch")) == null)
  and (([.failures[].detail] | index("contract_mismatch")) == null)
  and (([.failures[].detail] | index("status_mismatch")) == null)
' "$summary_json" >/dev/null

echo "[PASS] release review packet integrity rejects semantically invalid first-minute command feedback artifacts even when checksums match"
