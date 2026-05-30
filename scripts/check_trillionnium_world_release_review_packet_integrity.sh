#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY"
fi

PACKET_JSON="$ACCEPTANCE_DIR/release-review-packet.json"
PACKET_MD="$ACCEPTANCE_DIR/release-review-packet.md"
PACKET_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-packet.log"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON" ]]; then
  PACKET_JSON="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON"
fi
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD" ]]; then
  PACKET_MD="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD"
fi
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG" ]]; then
  PACKET_LOG="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG"
fi
CHECK_RESULTS="$(mktemp)"
REFRESH_PACKET=1
trap 'rm -f "$CHECK_RESULTS"' EXIT

for arg in "$@"; do
  case "$arg" in
    --no-refresh)
      REFRESH_PACKET=0
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$ACCEPTANCE_DIR"

add_check() {
  local name="$1"
  local status="$2"
  local path="$3"
  local detail="$4"
  local expected="${5:-}"
  local actual="${6:-}"
  jq -nc \
    --arg name "$name" \
    --arg status "$status" \
    --arg path "$path" \
    --arg detail "$detail" \
    --arg expected "$expected" \
    --arg actual "$actual" \
    '{name: $name, status: $status, path: $path, detail: $detail, expected: (if $expected == "" then null else $expected end), actual: (if $actual == "" then null else $actual end)}' >>"$CHECK_RESULTS"
}

require_json_expr() {
  local name="$1"
  local path="$2"
  local expr="$3"
  local detail="$4"
  if [[ ! -f "$path" ]]; then
    add_check "$name" fail "$path" missing
  elif jq -e "$expr" "$path" >/dev/null; then
    add_check "$name" ok "$path" "$detail"
  else
    add_check "$name" fail "$path" "$detail"
  fi
}

packet_artifact_path() {
  local artifact_id="$1"
  if [[ ! -f "$PACKET_JSON" ]]; then
    return 0
  fi
  jq -r --arg artifact_id "$artifact_id" '(.artifacts // [] | map(select(.id == $artifact_id)) | .[0].path) // empty' "$PACKET_JSON" 2>/dev/null || true
}

require_artifact_json_expr() {
  local name="$1"
  local artifact_id="$2"
  local expr="$3"
  local detail="$4"
  local path
  path="$(packet_artifact_path "$artifact_id")"
  if [[ -z "$path" ]]; then
    add_check "$name" fail "$artifact_id" missing_packet_artifact
  else
    require_json_expr "$name" "$path" "$expr" "$detail"
  fi
}

require_artifact_ppm_header() {
  local name="$1"
  local artifact_id="$2"
  local min_bytes="$3"
  local expected_width="${4:-1280}"
  local expected_height="${5:-720}"
  local path
  path="$(packet_artifact_path "$artifact_id")"
  if [[ -z "$path" ]]; then
    add_check "$name" fail "$artifact_id" missing_packet_artifact
    return
  fi
  if [[ ! -f "$path" ]]; then
    add_check "$name" fail "$path" missing
    return
  fi

  local header
  local bytes
  local expected_header
  header="$(head -n 3 "$path" || true)"
  bytes="$(wc -c <"$path" | tr -d ' ')"
  expected_header="$(printf 'P3\n%s %s\n255' "$expected_width" "$expected_height")"
  if [[ "$header" == "$expected_header" && "$bytes" -gt "$min_bytes" ]]; then
    add_check "$name" ok "$path" ppm_header_and_size_match "P3 ${expected_width}x${expected_height} > ${min_bytes} bytes" "$bytes"
  else
    add_check "$name" fail "$path" ppm_header_or_size_mismatch "P3 ${expected_width}x${expected_height} > ${min_bytes} bytes" "bytes=${bytes}"
  fi
}

if [[ "$REFRESH_PACKET" -eq 1 ]]; then
  if TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON="$PACKET_JSON" \
    TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD="$PACKET_MD" \
    "$ROOT/scripts/check_trillionnium_world_release_review_packet.sh" >"$PACKET_LOG" 2>&1; then
    add_check packet_refresh ok "$PACKET_LOG" refreshed
  else
    add_check packet_refresh fail "$PACKET_LOG" failed
  fi
else
  add_check packet_refresh skipped "$PACKET_LOG" no_refresh_requested
fi

require_json_expr packet_contract "$PACKET_JSON" '.contract_version == "trillionnium_world_release_review_packet_v1"' "packet contract matches"
require_json_expr packet_boundary "$PACKET_JSON" '.android_s5_real_device_claimed == false and .proof_scope == "host_side_bevy_runtime_replay_not_android_real_device"' "packet keeps Android S5 no-claim boundary"
require_json_expr packet_review_status "$PACKET_JSON" '.ready_for_release_review == true and (.status == "release_review_packet_ready_with_public_launch_blockers" or .status == "release_review_packet_green")' "packet is ready for review or public launch review"
require_json_expr packet_missing_artifacts "$PACKET_JSON" '(.missing_artifacts // []) | length == 0' "packet reports no missing artifacts"
require_json_expr packet_artifact_count "$PACKET_JSON" '(.artifacts // []) | length == 87' "packet has expected eighty-seven artifacts including operator handoff, checkpoint manifest, packet semantic negative fixture, packet control-loop semantic negative fixture, bot executor source-chain semantic negative fixture, bot executor failure/recovery matrix semantic negative fixture, bot gap semantic negative fixture, CEX adapter readiness, Bevy action coach, player HUD/debug layer, player UI rescue, classic RTS control loop, first-minute command feedback replay, first-minute command feedback recordings, first-minute command feedback contact sheet, first-minute command feedback rejection replay, first-minute command feedback rejection recordings, first-minute command feedback rejection contact sheet, bot planner action executor summary/log/contact sheet, bot planner executor replay determinism summary/log/contact sheet, multi-match bot executor evaluation summary/log/contact sheet, bot executor failure/recovery matrix summary/log/contact sheet, bot decision-state gap summary/contact sheet, bot adaptive build-order gap summary/contact sheet, bot tactical micro gap summary/contact sheet, bot map intel gap summary/contact sheet, live-window screenshots, sprite texture sampling, sampled texture live-window correlation, render asset eligibility, map modeling gate, bundle negative fixtures, evidence bundle, template negative fixtures, evidence kit, blocker consistency, status-only fixture guard, S5 real-device validation, public launch evidence intake, production map-pack collection, cohort/commercial collection, external ops collection, production map-pack public evidence, cohort/commercial validation, and external ops validation"
require_artifact_json_expr packet_integrity_semantic_fixture release_review_packet_integrity_semantic_fixture '.contract_version == "trillionnium_world_release_review_packet_integrity_semantic_fixture_v1" and .status == "release_review_packet_integrity_semantic_fixture_green" and .green == true and .fake_packet_artifact_count == 87 and .expected_semantic_failure_count == 4 and .expected_semantic_failure_names == ["first_minute_command_feedback_replay_semantics", "first_minute_command_feedback_source_recording_semantics", "first_minute_command_feedback_recording_semantics", "first_minute_command_feedback_replay_ppm_semantics"] and .checksum_mismatch_failure_count == 0 and .bytes_mismatch_failure_count == 0 and .contract_mismatch_failure_count == 0 and .status_mismatch_failure_count == 0 and .ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false' "packet integrity semantic fixture proves bad command feedback semantics fail even when checksums, bytes, contracts, and statuses match"
require_artifact_json_expr packet_integrity_bot_executor_semantic_fixture release_review_packet_integrity_bot_executor_semantic_fixture '.contract_version == "trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture_v1" and .status == "release_review_packet_integrity_bot_executor_semantic_fixture_green" and .green == true and .fake_packet_artifact_count == 87 and .expected_semantic_failure_count == 9 and .expected_semantic_failure_names == ["bot_planner_action_executor_semantics", "bot_planner_action_executor_log_semantics", "bot_planner_action_executor_ppm_semantics", "bot_planner_executor_replay_determinism_semantics", "bot_planner_executor_replay_determinism_log_semantics", "bot_planner_executor_replay_determinism_ppm_semantics", "multi_match_bot_executor_evaluation_semantics", "multi_match_bot_executor_evaluation_log_semantics", "multi_match_bot_executor_evaluation_ppm_semantics"] and .checksum_mismatch_failure_count == 0 and .bytes_mismatch_failure_count == 0 and .contract_mismatch_failure_count == 0 and .status_mismatch_failure_count == 0 and .ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false' "packet integrity bot executor semantic fixture proves bad bot executor source-chain semantics fail even when checksums, bytes, contracts, and statuses match"
require_artifact_json_expr packet_integrity_bot_executor_matrix_semantic_fixture release_review_packet_integrity_bot_executor_matrix_semantic_fixture '.contract_version == "trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture_v1" and .status == "release_review_packet_integrity_bot_executor_matrix_semantic_fixture_green" and .green == true and .fake_packet_artifact_count == 87 and .expected_semantic_failure_count == 3 and .expected_semantic_failure_names == ["bot_executor_failure_recovery_matrix_semantics", "bot_executor_failure_recovery_matrix_log_semantics", "bot_executor_failure_recovery_matrix_ppm_semantics"] and .checksum_mismatch_failure_count == 0 and .bytes_mismatch_failure_count == 0 and .contract_mismatch_failure_count == 0 and .status_mismatch_failure_count == 0 and .ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false' "packet integrity bot executor failure/recovery matrix semantic fixture proves bad matrix semantics fail even when checksums, bytes, contracts, and statuses match"
require_artifact_json_expr packet_integrity_bot_gap_semantic_fixture release_review_packet_integrity_bot_gap_semantic_fixture '.contract_version == "trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture_v1" and .status == "release_review_packet_integrity_bot_gap_semantic_fixture_green" and .green == true and .fake_packet_artifact_count == 87 and .expected_semantic_failure_count == 8 and .expected_semantic_failure_names == ["bot_decision_state_gap_semantics", "bot_decision_state_gap_ppm_semantics", "bot_adaptive_build_order_gap_semantics", "bot_adaptive_build_order_gap_ppm_semantics", "bot_tactical_micro_gap_semantics", "bot_tactical_micro_gap_ppm_semantics", "bot_map_intel_gap_semantics", "bot_map_intel_gap_ppm_semantics"] and .checksum_mismatch_failure_count == 0 and .bytes_mismatch_failure_count == 0 and .contract_mismatch_failure_count == 0 and .status_mismatch_failure_count == 0 and .ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false' "packet integrity bot gap semantic fixture proves bad bot gap foundation/micro/intel semantics fail even when checksums, bytes, contracts, and statuses match"
require_artifact_json_expr packet_integrity_control_loop_semantic_fixture release_review_packet_integrity_control_loop_semantic_fixture '.contract_version == "trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture_v1" and .status == "release_review_packet_integrity_control_loop_semantic_fixture_green" and .green == true and .fake_packet_artifact_count == 87 and .expected_semantic_failure_count == 2 and .expected_semantic_failure_names == ["classic_rts_control_loop_semantics", "classic_rts_control_loop_ppm_semantics"] and .checksum_mismatch_failure_count == 0 and .bytes_mismatch_failure_count == 0 and .contract_mismatch_failure_count == 0 and .status_mismatch_failure_count == 0 and .ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false' "packet integrity control loop semantic fixture proves bad control loop summary and PPM semantics fail even when checksums, bytes, contracts, and statuses match"
require_artifact_json_expr classic_rts_control_loop_semantics native_bevy_classic_rts_control_loop '.contract_version == "trillionnium_world_bevy_classic_rts_control_loop_v1" and .green == true and .preview_width == 1280 and .preview_height == 360 and .preview_format == "ppm_p3_rgb" and .write_gate == true and .mirror_scene_gate == true and .coliseum_scene_gate == true and .non_background_pixels > 120000 and .control_group_id == "1" and .move_selected_unit_count >= 4 and .attack_selected_unit_count >= 4 and (.move_command_queue | index("select_group_1") != null) and (.move_command_queue | index("move:7,4") != null) and (.move_command_queue | index("formation:diamond") != null) and (.attack_command_queue | index("select_group_1") != null) and (.attack_command_queue | index("attack:arena_creep_attack") != null) and .attack_target_id == "arena_creep_attack" and .selection_marker_pixel_count > 500 and .formation_line_pixel_count > 200 and .command_marker_pixel_count > 600 and .attack_feedback_pixel_count > 180 and .strategy_panel_pixel_count > 4000 and .minimap_pixel_count > 2800 and .fog_pixel_count > 400 and .vision_pixel_count > 120 and .resource_hud_pixel_count > 120 and .production_queue_pixel_count > 900 and (.move_production_queue | index("train:worker") != null) and (.move_build_queue | index("build:scout_tower") != null) and (.attack_build_queue | index("upgrade:training_hall") != null) and .move_training_progress_percent >= 50 and .attack_build_progress_percent >= 50 and .unit_health_card_pixel_count > 280 and .ability_command_pixel_count > 800 and .target_health_pixel_count > 60 and .attack_target_health_percent < 60 and .attack_active_ability_id == "focus_fire" and (.attack_ability_command_ids | index("focus_fire") != null) and (.attack_combat_event_log | index("damage:28") != null) and .selection_gate == true and .command_queue_gate == true and .strategy_hud_gate == true and .macro_loop_gate == true and .tactical_combat_gate == true and .gameplay_surface_gate == true and .cex_runtime_player_client_allowed == false and .wgpu_required == false' "classic RTS control loop keeps direct summary semantics for selection, move/attack command queues, macro HUD, tactical combat, and native-client boundary"
require_artifact_ppm_header classic_rts_control_loop_ppm_semantics native_bevy_classic_rts_control_loop_ppm 4000000 1280 360
require_artifact_json_expr first_minute_command_feedback_replay_semantics native_bevy_first_minute_command_feedback_replay '.contract_version == "trillionnium_world_bevy_first_minute_command_feedback_replay_v1" and .green == true and .command_input_action_count == 7 and .accepted_command_input_count == 7 and .first_minute_replay_gate == true and .command_recording_parse_gate == true and .live_command_input_gate == true and .scene_renderer_gate == true and .history_entry_count == 3 and .history_capacity == 3 and .retained_history_group_ids == ["26", "27", "28"] and .pruned_history_group_ids == ["25", "24"] and .cleared_active_stale_pixel_count == 0 and .preview_width == 1280 and .preview_height == 720 and .android_s5_real_device_claimed == false' "first-minute command feedback replay keeps 7/7 live RTS inputs, recent-3 prune evidence, stale-chip absence, 1280x720 contact sheet boundary"
require_artifact_json_expr first_minute_command_feedback_source_recording_semantics native_bevy_first_minute_command_feedback_source_recording '.contract_version == "trillionnium_world_bevy_first_minute_input_recording_v1" and .source_timeline_contract == "trillionnium_world_bevy_first_minute_interaction_timeline_v1" and .source_timeline_green == true and (.steps | length) == 10 and .android_s5_real_device_claimed == false' "first-minute source recording keeps original first-minute replay timeline and Android no-claim boundary"
require_artifact_json_expr first_minute_command_feedback_recording_semantics native_bevy_first_minute_command_feedback_recording '.contract_version == "trillionnium_world_bevy_first_minute_command_feedback_recording_v1" and .source_input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1" and .source_input_recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1" and .source_input_replay_green == true and .command_history_capacity == 3 and .retained_history_group_ids == ["26", "27", "28"] and .pruned_history_group_ids == ["25", "24"] and (.steps | length) == 7 and [.steps[].action_label] == ["RTS:SELECT:26", "RTS:MOVE:18,31:line", "RTS:SELECT:27", "RTS:MOVE:21,25:line", "RTS:SELECT:28", "RTS:MOVE:1,31:line", "RTS:SELECT:26"] and .android_s5_real_device_claimed == false' "command feedback recording keeps exact 7 action labels, recent-3 history, prune list, and Android no-claim boundary"
require_artifact_ppm_header first_minute_command_feedback_replay_ppm_semantics native_bevy_first_minute_command_feedback_replay_ppm 8000000
require_artifact_json_expr first_minute_command_feedback_rejection_replay_semantics native_bevy_first_minute_command_feedback_rejection_replay '.contract_version == "trillionnium_world_bevy_first_minute_command_feedback_rejection_replay_v1" and .green == true and .command_input_action_count == 7 and .accepted_command_input_count == 1 and .blocked_command_input_count == 6 and .blocked_reasons == ["rts_group_selection_required", "rts_invalid_tile:bad-tile", "rts_attack_target_required", "rts_attack_required_before_ability", "rts_queue_id_required", "rts_group_id_required"] and .command_queue_rejection_pollution_count == 0 and .first_minute_replay_gate == true and .rejection_recording_parse_gate == true and .command_action_parse_gate == true and .replay_expectation_gate == true and .blocked_feedback_gate == true and .blocked_action_history_gate == true and .blocked_history_non_pollution_gate == true and .history_entry_count == 3 and .history_capacity == 3 and .retained_history_group_ids == ["26", "27", "28"] and .pruned_history_group_ids == ["25", "24"] and .cleared_active_stale_pixel_count == 0 and .preview_width == 1280 and .preview_height == 720 and .android_s5_real_device_claimed == false' "first-minute command feedback rejection replay keeps 6/7 blocked live RTS inputs, structured reasons, history non-pollution, recent-3 prune evidence, stale-chip absence, 1280x720 contact sheet boundary"
require_artifact_json_expr first_minute_command_feedback_rejection_source_recording_semantics native_bevy_first_minute_command_feedback_rejection_source_recording '.contract_version == "trillionnium_world_bevy_first_minute_input_recording_v1" and .source_timeline_contract == "trillionnium_world_bevy_first_minute_interaction_timeline_v1" and .source_timeline_green == true and (.steps | length) == 10 and .android_s5_real_device_claimed == false' "first-minute rejection source recording keeps original first-minute replay timeline and Android no-claim boundary"
require_artifact_json_expr first_minute_command_feedback_rejection_recording_semantics native_bevy_first_minute_command_feedback_rejection_recording '.contract_version == "trillionnium_world_bevy_first_minute_command_feedback_rejection_recording_v1" and .source_input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1" and .source_input_recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1" and .source_input_replay_green == true and .command_history_capacity == 3 and .retained_history_group_ids == ["26", "27", "28"] and .pruned_history_group_ids == ["25", "24"] and (.steps | length) == 7 and [.steps[].action_label] == ["RTS:MOVE:18,31:line", "RTS:SELECT:26", "RTS:MOVE:bad-tile:line", "RTS:ATTACK:", "RTS:ABILITY:guard_break", "RTS:QUEUE:", "RTS:SELECT:"] and [.steps[] | select(.expected_accepted == false) | .expected_reason] == ["rts_group_selection_required", "rts_invalid_tile:bad-tile", "rts_attack_target_required", "rts_attack_required_before_ability", "rts_queue_id_required", "rts_group_id_required"] and .android_s5_real_device_claimed == false' "command feedback rejection recording keeps exact blocked action labels, expected reasons, recent-3 history, prune list, and Android no-claim boundary"
require_artifact_ppm_header first_minute_command_feedback_rejection_replay_ppm_semantics native_bevy_first_minute_command_feedback_rejection_replay_ppm 8000000
require_artifact_json_expr bot_planner_action_executor_semantics native_bevy_bot_planner_action_executor '.contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1" and .green == true and .bot_planner_action_executor_gate == true and .executor_action_count == 6 and .accepted_action_count == 6 and .command_marker_hit_count == 6 and .action_labels == ["RTS:QUEUE:faction:mirror_guard", "RTS:QUEUE:recon:sweep:watchtower_scan@7,4", "RTS:QUEUE:objective:claim:relay_beacon@6,5", "RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost", "RTS:QUEUE:tier2:push:gate_bulwark@10,3", "RTS:QUEUE:tier2:finish:gate_bulwark@10,3"] and .input_sources == ["classic_rts_bot_planner_action_executor_input"] and .final_runtime_summary.faction_id == "mirror_guard" and .final_runtime_summary.objective_capture_percent == 100 and (.final_runtime_summary.tier_two_tech_ids | index("relay_foundry") != null) and .final_runtime_summary.siege_breach_state == "counterplay_won:gate_bulwark" and .final_runtime_summary.match_result_state == "siege_breakthrough:inner_lane" and .bevy_bot_planner_action_executor_claimed == true and .bevy_openra_runtime_bot_executor_claimed == false and .android_s5_real_device_claimed == false and .public_launch_ready == false' "bot planner action executor keeps six accepted Bevy-native RTS actions, exact action labels, final runtime summary, and public-launch no-claim boundary"
require_artifact_json_expr bot_planner_action_executor_log_semantics native_bevy_bot_planner_action_executor_log '.contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1" and .executor_action_count == 6 and .accepted_action_count == 6 and .command_marker_hit_count == 6 and .input_source == "classic_rts_bot_planner_action_executor_input" and (.execution_log | length) == 6 and (.execution_log | all(.accepted == true and .command_marker_hit == true and .feedback_event_delta == 1 and .input_source == "classic_rts_bot_planner_action_executor_input")) and [.execution_log[].action_label] == ["RTS:QUEUE:faction:mirror_guard", "RTS:QUEUE:recon:sweep:watchtower_scan@7,4", "RTS:QUEUE:objective:claim:relay_beacon@6,5", "RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost", "RTS:QUEUE:tier2:push:gate_bulwark@10,3", "RTS:QUEUE:tier2:finish:gate_bulwark@10,3"]' "bot planner action executor log preserves exact action labels and six accepted command marker hits"
require_artifact_ppm_header bot_planner_action_executor_ppm_semantics native_bevy_bot_planner_action_executor_ppm 8000000 1280 1080
require_artifact_json_expr bot_planner_executor_replay_determinism_semantics native_bevy_bot_planner_executor_replay_determinism '.contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1" and .green == true and .bot_planner_executor_replay_determinism_gate == true and .source_executor_action_count == 6 and .replay_action_count == 6 and .accepted_replay_action_count == 6 and .replay_command_marker_hit_count == 6 and .command_delta_match_count == 6 and .runtime_determinism_gate == true and .source_final_runtime_sha256 == .replay_final_runtime_sha256 and .source_command_queue_sha256 == .replay_command_queue_sha256 and .replay_input_sources == ["classic_rts_bot_planner_executor_replay_input"] and .bevy_bot_planner_executor_replay_determinism_claimed == true and .bevy_openra_runtime_bot_executor_claimed == false and .android_s5_real_device_claimed == false and .public_launch_ready == false' "bot planner executor replay determinism keeps six accepted replay actions, command-delta matches, runtime/queue hashes, and public-launch no-claim boundary"
require_artifact_json_expr bot_planner_executor_replay_determinism_log_semantics native_bevy_bot_planner_executor_replay_determinism_log '.contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1" and .replay_action_count == 6 and .accepted_replay_action_count == 6 and .replay_command_marker_hit_count == 6 and .command_delta_match_count == 6 and .source_final_runtime_sha256 == .replay_final_runtime_sha256 and .source_command_queue_sha256 == .replay_command_queue_sha256 and .replay_input_source == "classic_rts_bot_planner_executor_replay_input" and (.execution_log | length) == 6 and (.execution_log | all(.accepted == true and .action_label_parse_gate == true and .command_marker_hit == true and .command_delta_match == true and .input_source == "classic_rts_bot_planner_executor_replay_input"))' "bot planner executor replay determinism log preserves six accepted replay actions and command-delta matches"
require_artifact_ppm_header bot_planner_executor_replay_determinism_ppm_semantics native_bevy_bot_planner_executor_replay_determinism_ppm 8000000 1280 1080
require_artifact_json_expr multi_match_bot_executor_evaluation_semantics native_bevy_multi_match_bot_executor_evaluation '.contract_version == "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1" and .green == true and .multi_match_bot_executor_evaluation_gate == true and .variant_count == 4 and .accepted_variant_count == 4 and .total_replay_action_count == 24 and .total_accepted_action_count == 24 and .total_command_marker_hit_count == 24 and .total_command_delta_match_count == 24 and .runtime_sha_match_count == 4 and .command_queue_sha_match_count == 4 and (.variant_map_values | sort) == ["forest_relay", "market_ruins", "marsh_gate", "ridge_watch"] and .bevy_multi_match_bot_executor_evaluation_claimed == true and .bevy_bot_planner_executor_replay_determinism_claimed == true and .bevy_openra_runtime_bot_executor_claimed == false and .android_s5_real_device_claimed == false and .public_launch_ready == false' "multi-match bot executor evaluation keeps four deterministic variants, 24 accepted actions, runtime/queue hash matches, and public-launch no-claim boundary"
require_artifact_json_expr multi_match_bot_executor_evaluation_log_semantics native_bevy_multi_match_bot_executor_evaluation_log '.contract_version == "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1" and .variant_count == 4 and .accepted_variant_count == 4 and .total_replay_action_count == 24 and .total_accepted_action_count == 24 and .total_command_marker_hit_count == 24 and .total_command_delta_match_count == 24 and .runtime_sha_match_count == 4 and .command_queue_sha_match_count == 4 and .evaluation_input_source == "classic_rts_multi_match_bot_executor_evaluation_input" and (.variant_summaries | length) == 4 and (.variant_summaries | all(.replay_action_count == 6 and .accepted_action_count == 6 and .command_marker_hit_count == 6 and .command_delta_match_count == 6 and .runtime_sha_match == true and .command_queue_sha_match == true)) and ([.variant_summaries[].map_variant] | sort) == ["forest_relay", "market_ruins", "marsh_gate", "ridge_watch"]' "multi-match bot executor evaluation log preserves four variants, per-variant 6/6 action acceptance, command deltas, and runtime/queue hash matches"
require_artifact_ppm_header multi_match_bot_executor_evaluation_ppm_semantics native_bevy_multi_match_bot_executor_evaluation_ppm 8000000
require_artifact_json_expr bot_executor_failure_recovery_matrix_semantics native_bevy_bot_executor_failure_recovery_matrix '.contract_version == "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1" and .green == true and .bot_executor_failure_recovery_matrix_state == "bevy_executor_rejects_blocked_actions_and_recovers_without_command_queue_pollution_not_openra_runtime_bot" and .bot_executor_failure_recovery_matrix_gate == true and .source_replay_action_count == 6 and .blocked_injection_count == 6 and .blocked_rejection_count == 6 and .blocked_expected_reason_count == 6 and .blocked_feedback_event_count == 6 and .blocked_command_queue_unchanged_count == 6 and .blocked_command_queue_sha_match_count == 6 and (.blocked_reason_values | index("rts_queue_id_required") != null) and (.blocked_reason_values | index("rts_group_id_required") != null) and (.blocked_reason_values | index("rts_attack_required_before_ability") != null) and (.blocked_reason_values | index("rts_invalid_tile:bad-tile") != null) and (.blocked_reason_values | index("rts_attack_target_required") != null) and .blocked_input_sources == ["classic_rts_bot_executor_failure_recovery_matrix_blocked_input"] and .recovery_input_sources == ["classic_rts_bot_executor_failure_recovery_matrix_recovery_input"] and .recovery_action_count == 6 and .recovery_accepted_action_count == 6 and .recovery_command_marker_hit_count == 6 and .recovery_command_delta_match_count == 6 and .feedback_blocked_count == 6 and .feedback_recovery_count == 6 and .final_input_feedback_event_count == 12 and .recovery_safe_runtime_sha_match == true and .command_queue_sha_match == true and (.matrix_log | length) == 6 and (.matrix_log | all(.blocked.accepted == false and .blocked.rejected == true and .blocked.expected_reason_match == true and .blocked.command_queue_unchanged == true and .blocked.command_queue_sha_match == true and .blocked.feedback_event_delta == 1 and .blocked.blocked_history_delta == 1 and .recovery.action_label_parse_gate == true and .recovery.accepted == true and .recovery.command_marker_hit == true and .recovery.command_delta_match == true)) and .source_multi_match_summary.variant_count == 4 and .source_multi_match_summary.total_replay_action_count == 24 and .source_multi_match_summary.total_accepted_action_count == 24 and .final_recovery_safe_runtime_summary.faction_id == "mirror_guard" and .final_recovery_safe_runtime_summary.objective_capture_percent == 100 and (.final_recovery_safe_runtime_summary.tier_two_tech_ids | index("relay_foundry") != null) and .final_recovery_safe_runtime_summary.siege_breach_state == "counterplay_won:gate_bulwark" and .final_recovery_safe_runtime_summary.match_result_state == "siege_breakthrough:inner_lane" and .bevy_bot_executor_failure_recovery_matrix_claimed == true and .bevy_multi_match_bot_executor_evaluation_claimed == true and .bevy_openra_runtime_bot_executor_claimed == false and .android_s5_real_device_claimed == false and .public_launch_ready == false' "bot executor failure/recovery matrix keeps 6/6 blocked rejections, 6/6 recovery actions, unchanged command queues, recovery-safe runtime, multi-match source summary, and public-launch no-claim boundary"
require_artifact_json_expr bot_executor_failure_recovery_matrix_log_semantics native_bevy_bot_executor_failure_recovery_matrix_log '.contract_version == "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1" and .source_replay_action_count == 6 and .blocked_injection_count == 6 and .blocked_rejection_count == 6 and .blocked_command_queue_unchanged_count == 6 and .recovery_action_count == 6 and .recovery_accepted_action_count == 6 and .recovery_command_delta_match_count == 6 and .recovery_safe_runtime_sha_match == true and .command_queue_sha_match == true and [.matrix_log[].blocked.expected_reason] == ["rts_queue_id_required", "rts_group_id_required", "rts_attack_required_before_ability", "rts_invalid_tile:bad-tile", "rts_queue_id_required", "rts_attack_target_required"] and (.matrix_log | length) == 6 and (.matrix_log | all(.blocked.accepted == false and .blocked.rejected == true and .blocked.command_queue_unchanged == true and .blocked.command_queue_sha_match == true and .recovery.accepted == true and .recovery.command_delta_match == true))' "bot executor failure/recovery matrix log preserves exact blocked reason order, command queue non-pollution, and recovery command delta matches"
require_artifact_ppm_header bot_executor_failure_recovery_matrix_ppm_semantics native_bevy_bot_executor_failure_recovery_matrix_ppm 8000000 1280 1080
require_artifact_json_expr bot_decision_state_gap_semantics native_bevy_bot_decision_state_gap '.contract_version == "trillionnium_world_bevy_classic_rts_bot_decision_state_gap_v1" and .green == true and .preview_width == 1280 and .preview_height == 1080 and .write_gate == true and .input_action_count == 0 and .bevy_bot_decision_gap_state == "bevy_bot_decision_vocabulary_not_openra_native_bot_ai" and .bevy_native_bot_ai_claimed == false and .bevy_openra_parity_claimed == false and .openra_gap_not_closed_gate == true and .openra_bot_economy_tech_target_commit == "f6c47d9" and .openra_bot_beacon_pressure_target_commit == "2b6f25b" and .openra_organic_bot_terminal_target_commit == "5f1bf76" and .bot_decision_stage_count == 6 and (.stage_summaries | length) == 6 and (.stage_summaries | map(.stage) | index("economy_seed") != null) and (.stage_summaries | map(.stage) | index("scout_objectives") != null) and (.stage_summaries | map(.stage) | index("capture_beacon") != null) and (.stage_summaries | map(.stage) | index("tech_switch") != null) and (.stage_summaries | map(.stage) | index("defend_counter") != null) and (.stage_summaries | map(.stage) | index("attack_commit_with_counter_repath") != null) and .decision_signal_count >= 18 and .economy_decision_count >= 3 and .objective_decision_count >= 4 and .combat_decision_count >= 4 and .tech_decision_count >= 2 and .final_bot_decision_state == "attack_commit_with_counter_repath" and .final_rts_ai_pressure_percent >= 70 and .final_rts_defeat_risk_percent <= 35 and .final_objective_capture_percent >= 90 and .final_match_result_state == "bot_decision_gap:attack_commit_with_counter_repath" and (.final_command_queue | index("decision:combat:attack_commit_with_counter_repath") != null) and (.final_command_queue | index("parity_claim:false") != null) and (.final_army_production_batch_ids | index("batch:tech:signal+skimmer+bastion") != null) and .bot_decision_state_gap_gate == true and .cex_runtime_player_client_allowed == false and .wgpu_required == false' "bot decision-state gap keeps six decision stages, OpenRA target commit anchors, final attack/counter decision state, and native parity no-claim boundary"
require_artifact_ppm_header bot_decision_state_gap_ppm_semantics native_bevy_bot_decision_state_gap_ppm 8000000 1280 1080
require_artifact_json_expr bot_adaptive_build_order_gap_semantics native_bevy_bot_adaptive_build_order_gap '.contract_version == "trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap_v1" and .green == true and .preview_width == 1280 and .preview_height == 1080 and .write_gate == true and .input_action_count == 0 and .bevy_bot_adaptive_build_gap_state == "bevy_adaptive_build_order_vocabulary_not_openra_native_ai_planner" and .bevy_native_adaptive_ai_claimed == false and .bevy_openra_parity_claimed == false and .openra_gap_not_closed_gate == true and .openra_bot_economy_tech_target_commit == "f6c47d9" and .openra_bot_beacon_pressure_target_commit == "2b6f25b" and .openra_organic_bot_terminal_target_commit == "5f1bf76" and .adaptive_stage_count == 6 and (.stage_summaries | length) == 6 and (.stage_summaries | map(.stage) | index("opening_worker_split") != null) and (.stage_summaries | map(.stage) | index("scout_trigger_response") != null) and (.stage_summaries | map(.stage) | index("expand_or_defend_branch") != null) and (.stage_summaries | map(.stage) | index("tech_counter_switch") != null) and (.stage_summaries | map(.stage) | index("pressure_window_commit") != null) and (.stage_summaries | map(.stage) | index("retreat_rebuild_reattack") != null) and .adaptive_signal_count >= 24 and .opening_build_order_count >= 3 and .scout_trigger_count >= 2 and .branch_switch_count >= 3 and .counter_tech_switch_count >= 2 and .pressure_window_count >= 2 and .retreat_rebuild_count >= 2 and .final_adaptive_state == "pressure_window_rebuild_reattack" and .final_rts_ai_pressure_percent >= 70 and .final_rts_defeat_risk_percent <= 20 and .final_objective_capture_percent >= 90 and .final_match_result_state == "adaptive_build_gap:pressure_window_rebuild_reattack" and (.final_command_queue | index("adaptive_stage:retreat_rebuild_reattack") != null) and (.final_command_queue | index("native_openra_ai_planner:false") != null) and (.final_army_production_batch_ids | index("build_order:signal_array_into_skimmer") != null) and (.final_army_production_batch_ids | index("build_order:pullback_rebuild_then_reattack") != null) and .adaptive_build_order_gap_gate == true and .cex_runtime_player_client_allowed == false and .wgpu_required == false' "bot adaptive build-order gap keeps six adaptive stages, OpenRA target commit anchors, pressure/rebuild/reattack final state, and native parity no-claim boundary"
require_artifact_ppm_header bot_adaptive_build_order_gap_ppm_semantics native_bevy_bot_adaptive_build_order_gap_ppm 8000000 1280 1080
require_artifact_json_expr bot_tactical_micro_gap_semantics native_bevy_bot_tactical_micro_gap '.contract_version == "trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap_v1" and .green == true and .preview_width == 1280 and .preview_height == 1080 and .write_gate == true and .input_action_count == 0 and .bevy_bot_tactical_micro_gap_state == "bevy_tactical_micro_vocabulary_not_openra_native_combat_ai" and .bevy_native_combat_ai_claimed == false and .bevy_openra_parity_claimed == false and .openra_gap_not_closed_gate == true and .openra_bot_economy_tech_target_commit == "f6c47d9" and .openra_bot_beacon_pressure_target_commit == "2b6f25b" and .openra_organic_bot_terminal_target_commit == "5f1bf76" and .micro_stage_count == 6 and (.stage_summaries | length) == 6 and (.stage_summaries | map(.stage) | index("target_priority_probe") != null) and (.stage_summaries | map(.stage) | index("focus_fire_commit") != null) and (.stage_summaries | map(.stage) | index("kite_and_stutter_step") != null) and (.stage_summaries | map(.stage) | index("flank_angle_split") != null) and (.stage_summaries | map(.stage) | index("ability_timing_window") != null) and (.stage_summaries | map(.stage) | index("low_health_pullback_regroup") != null) and .micro_signal_count >= 24 and .target_swap_count >= 3 and .focus_fire_order_count >= 3 and .kite_step_count >= 3 and .flank_angle_count >= 2 and .ability_timing_count >= 2 and .low_health_pullback_count >= 2 and .final_micro_state == "pullback_regroup_reattack" and .final_rts_ai_pressure_percent >= 70 and .final_rts_defeat_risk_percent <= 20 and .final_objective_capture_percent >= 90 and .final_match_result_state == "tactical_micro_gap:pullback_regroup_reattack" and (.final_command_queue | index("micro_stage:low_health_pullback_regroup") != null) and (.final_command_queue | index("native_openra_combat_ai:false") != null) and (.final_army_production_batch_ids | index("micro_control:focus_fire_low_armor_striker") != null) and (.final_army_production_batch_ids | index("micro_control:pull_redline_units_regroup_reattack") != null) and .tactical_micro_gap_gate == true and .cex_runtime_player_client_allowed == false and .wgpu_required == false' "bot tactical micro gap keeps six micro stages, OpenRA target commit anchors, pullback/regroup final state, and native combat-AI no-claim boundary"
require_artifact_ppm_header bot_tactical_micro_gap_ppm_semantics native_bevy_bot_tactical_micro_gap_ppm 8000000 1280 1080
require_artifact_json_expr bot_map_intel_gap_semantics native_bevy_bot_map_intel_gap '.contract_version == "trillionnium_world_bevy_classic_rts_bot_map_intel_gap_v1" and .green == true and .preview_width == 1280 and .preview_height == 1080 and .write_gate == true and .input_action_count == 0 and .bevy_bot_map_intel_gap_state == "bevy_map_intel_vocabulary_not_openra_native_shroud_memory_ai" and .bevy_native_shroud_memory_ai_claimed == false and .bevy_openra_parity_claimed == false and .openra_gap_not_closed_gate == true and .openra_bot_economy_tech_target_commit == "f6c47d9" and .openra_bot_beacon_pressure_target_commit == "2b6f25b" and .openra_organic_bot_terminal_target_commit == "5f1bf76" and .intel_stage_count == 6 and (.stage_summaries | length) == 6 and (.stage_summaries | map(.stage) | index("initial_scout_sweep") != null) and (.stage_summaries | map(.stage) | index("fog_memory_stamp") != null) and (.stage_summaries | map(.stage) | index("expansion_threat_inference") != null) and (.stage_summaries | map(.stage) | index("enemy_tech_read") != null) and (.stage_summaries | map(.stage) | index("hidden_army_prediction") != null) and (.stage_summaries | map(.stage) | index("rotate_pressure_reveal") != null) and .intel_signal_count >= 24 and .scout_sweep_count >= 3 and .fog_memory_stamp_count >= 4 and .expansion_threat_count >= 3 and .enemy_tech_read_count >= 2 and .hidden_army_prediction_count >= 2 and .pressure_rotation_count >= 2 and .final_intel_state == "rotate_pressure_confirmed_beacon" and .final_rts_ai_pressure_percent >= 80 and .final_rts_defeat_risk_percent <= 20 and .final_objective_capture_percent >= 90 and .final_match_result_state == "map_intel_gap:rotate_pressure_confirmed_beacon" and (.final_command_queue | index("intel_stage:rotate_pressure_reveal") != null) and (.final_command_queue | index("native_openra_shroud_memory_ai:false") != null) and (.final_army_production_batch_ids | index("map_intel:fog_memory_last_seen_grid") != null) and (.final_army_production_batch_ids | index("map_intel:rotate_pressure_to_confirmed_beacon") != null) and .map_intel_gap_gate == true and .cex_runtime_player_client_allowed == false and .wgpu_required == false' "bot map intel gap keeps six intel stages, OpenRA target commit anchors, rotate-pressure final state, and native shroud-memory no-claim boundary"
require_artifact_ppm_header bot_map_intel_gap_ppm_semantics native_bevy_bot_map_intel_gap_ppm 8000000 1280 1080

if [[ -f "$PACKET_MD" ]]; then
  if grep -Fq -- 'Native/Bevy replay, action coach, HUD/debug layer, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.' "$PACKET_MD" && grep -Fq -- 'Still Requires Real External Evidence' "$PACKET_MD"; then
    add_check packet_markdown_boundary ok "$PACKET_MD" "Markdown includes blocker and boundary sections"
  else
    add_check packet_markdown_boundary fail "$PACKET_MD" "Markdown missing blocker or boundary sections"
  fi
else
  add_check packet_markdown_boundary fail "$PACKET_MD" missing
fi

if [[ -f "$PACKET_JSON" ]]; then
  while IFS= read -r encoded; do
    artifact_json="$(printf '%s' "$encoded" | base64 -d)"
    id="$(jq -r '.id' <<<"$artifact_json")"
    path="$(jq -r '.path' <<<"$artifact_json")"
    expected_sha="$(jq -r '.sha256 // empty' <<<"$artifact_json")"
    expected_bytes="$(jq -r '.bytes // empty' <<<"$artifact_json")"
    expected_contract="$(jq -r '.contract_version // empty' <<<"$artifact_json")"
    expected_status="$(jq -r '.status // empty' <<<"$artifact_json")"

    if [[ ! -f "$path" ]]; then
      add_check "artifact_${id}_present" fail "$path" missing
      continue
    fi

    actual_sha="$(sha256sum "$path" | awk '{print $1}')"
    actual_bytes="$(wc -c <"$path" | tr -d ' ')"
    if [[ "$actual_sha" == "$expected_sha" ]]; then
      add_check "artifact_${id}_sha256" ok "$path" sha256_match "$expected_sha" "$actual_sha"
    else
      add_check "artifact_${id}_sha256" fail "$path" sha256_mismatch "$expected_sha" "$actual_sha"
    fi
    if [[ "$actual_bytes" == "$expected_bytes" ]]; then
      add_check "artifact_${id}_bytes" ok "$path" bytes_match "$expected_bytes" "$actual_bytes"
    else
      add_check "artifact_${id}_bytes" fail "$path" bytes_mismatch "$expected_bytes" "$actual_bytes"
    fi

    if [[ "$path" == *.json ]]; then
      actual_contract="$(jq -r '.contract_version // empty' "$path" 2>/dev/null || true)"
      actual_status="$(jq -r '.status // .overall_status // empty' "$path" 2>/dev/null || true)"
      if [[ -n "$expected_contract" ]]; then
        if [[ "$actual_contract" == "$expected_contract" ]]; then
          add_check "artifact_${id}_contract" ok "$path" contract_match "$expected_contract" "$actual_contract"
        else
          add_check "artifact_${id}_contract" fail "$path" contract_mismatch "$expected_contract" "$actual_contract"
        fi
      fi
      if [[ -n "$expected_status" ]]; then
        if [[ "$actual_status" == "$expected_status" ]]; then
          add_check "artifact_${id}_status" ok "$path" status_match "$expected_status" "$actual_status"
        else
          add_check "artifact_${id}_status" fail "$path" status_mismatch "$expected_status" "$actual_status"
        fi
      fi
    fi
  done < <(jq -r '.artifacts[] | @base64' "$PACKET_JSON")
fi

CHECKS_JSON="$(jq -s '.' "$CHECK_RESULTS")"
FAILURES_JSON="$(jq -s '[.[] | select(.status == "fail")]' "$CHECK_RESULTS")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // false' "$PACKET_JSON" 2>/dev/null || printf 'false')"
READY_FOR_RELEASE_REVIEW="$(jq -r '.ready_for_release_review // false' "$PACKET_JSON" 2>/dev/null || printf 'false')"
ARTIFACT_COUNT="$(jq -r '(.artifacts // []) | length' "$PACKET_JSON" 2>/dev/null || printf '0')"

GREEN=false
STATUS=release_review_packet_integrity_blocked
if [[ "$FAILURE_COUNT" == "0" && "$READY_FOR_RELEASE_REVIEW" == "true" ]]; then
  GREEN=true
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    STATUS=release_review_packet_integrity_green
  else
    STATUS=release_review_packet_integrity_green_with_public_launch_blockers
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_packet_integrity_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg packet_json "$PACKET_JSON" \
  --arg packet_markdown "$PACKET_MD" \
  --arg packet_log "$PACKET_LOG" \
  --argjson green "$GREEN" \
  --argjson ready_for_release_review "$READY_FOR_RELEASE_REVIEW" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson artifact_count "$ARTIFACT_COUNT" \
  --argjson checks "$CHECKS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_packet_integrity",
    green: $green,
    ready_for_release_review: $ready_for_release_review,
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    packet: {json_path: $packet_json, markdown_path: $packet_markdown, refresh_log_path: $packet_log},
    integrity_rule: "packet_artifact_paths_must_exist_and_recorded_sha256_bytes_contract_status_must_match_current_files_including_checkpoint_manifest_packet_semantic_negative_fixture_classic_rts_control_loop_semantic_negative_fixture_bot_executor_source_chain_semantic_negative_fixture_bot_executor_failure_recovery_matrix_semantic_negative_fixture_bot_gap_foundation_micro_intel_semantic_negative_fixture_cex_adapter_local_bevy_playability_evidence_classic_rts_control_loop_semantics_first_minute_command_feedback_replay_semantics_first_minute_command_feedback_rejection_replay_semantics_bot_executor_source_chain_semantics_bot_executor_failure_recovery_matrix_semantics_and_bot_gap_foundation_micro_intel_semantics",
    artifact_count: $artifact_count,
    checks: $checks,
    failures: $failures,
    reviewer_next_action: (if $green and $public_launch_ready then "review_public_launch_ready_evidence" elif $green then "collect_real_external_public_launch_evidence" else "regenerate_or_repair_release_review_packet" end)
  }' >"$SUMMARY_FILE"

case "$STATUS" in
  release_review_packet_integrity_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_GREEN %s\n' "$SUMMARY_FILE"
    ;;
  release_review_packet_integrity_green_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s\n' "$SUMMARY_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
    exit 1
    ;;
esac
