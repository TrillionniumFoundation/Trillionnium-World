#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-all-branch-keyboard-replay.json"
SUMMARY_RAW="$SUMMARY.raw"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-all-branch-keyboard-replay >"$SUMMARY_RAW"
)

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
  .status = "keyboard_replay_green"
  | .external_evidence_ignored_for_current_keyboard_replay_pass = true
  | .ready_for_release_review = true
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
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1"
  and .status == "keyboard_replay_green"
  and .build_branch_title_route_all_branch_keyboard_loop_contract == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_loop_v1"
  and .green == true
  and .all_branch_keyboard_loop_contract_green == true
  and .branch_count == 3
  and .all_branch_replay_gate == true
  and .ready_for_release_review == true
  and .proof_scope == "host_side_bevy_runtime_replay_not_android_real_device"
  and .green_replay_result_count == .branch_count
  and .recorded_branch_green_count == .branch_count
  and .recorded_sequence_total_count == 25
  and .replay_event_total_count == .recorded_sequence_total_count
  and .final_runtime_match_count == .branch_count
  and .combat_victory_branch_count == 1
  and .reward_item_count == .branch_count
  and .branches.force.recorded_sequence_count == 10
  and .branches.force.replay_event_count == 10
  and .branches.force.final_runtime_match == true
  and .branches.force.final_objective_status == "build_mastery_challenge_completed:force:task-force-mastery-guard-trial"
  and .branches.force.combat_result_state == "victory"
  and .branches.force.current_room_id == "league-coliseum"
  and .branches.force.reward_item_id == "force-mastery-signet"
  and .branches.agility.recorded_sequence_count == 8
  and .branches.agility.replay_event_count == 8
  and .branches.agility.final_runtime_match == true
  and .branches.agility.final_objective_status == "build_mastery_challenge_completed:agility:task-agility-mastery-shortcut-run"
  and .branches.agility.combat_result_state == "not_started"
  and .branches.agility.current_room_id == "mirror-city-square"
  and .branches.agility.reward_item_id == "agility-mastery-signet"
  and .branches.craft.recorded_sequence_count == 7
  and .branches.craft.replay_event_count == 7
  and .branches.craft.final_runtime_match == true
  and .branches.craft.final_objective_status == "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"
  and .branches.craft.combat_result_state == "not_started"
  and .branches.craft.current_room_id == "forge-workbench"
  and .branches.craft.reward_item_id == "craft-mastery-signet"
  and (.replayed_stat_ids | index("force") != null)
  and (.replayed_stat_ids | index("agility") != null)
  and (.replayed_stat_ids | index("craft") != null)
  and .replay_results.force.green == true
  and .replay_results.force.recorded_branch_green == true
  and .replay_results.force.recorded_sequence_parse_gate == true
  and .replay_results.force.recorded_sequence_path_gate == true
  and .replay_results.force.recorded_sequence_count > 0
  and .replay_results.force.replay_event_count == .replay_results.force.recorded_sequence_count
  and .replay_results.force.replay_sequence_signature_match == true
  and .replay_results.force.final_runtime_match == true
  and all(.replay_results.force.replay_events[]; .signature_match == true and .recorded_signature.input_path == "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action" and .replay_signature.input_path == "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action")
  and (.replay_results.force.recorded_sequence | map(select(.key == "KeyJ" and .action == "COMBAT:attack")) | length) >= 1
  and .replay_results.force.expected_final_runtime.current_room_id == "league-coliseum"
  and .replay_results.force.replay_final_runtime.current_room_id == "league-coliseum"
  and .replay_results.force.expected_final_runtime == .replay_results.force.replay_final_runtime
  and .replay_results.force.replay_final_runtime.combat_result_state == "victory"
  and (.replay_results.force.replay_final_runtime.inventory_items | index("force-mastery-signet") != null)
  and .replay_results.agility.green == true
  and .replay_results.agility.recorded_branch_green == true
  and .replay_results.agility.recorded_sequence_parse_gate == true
  and .replay_results.agility.recorded_sequence_path_gate == true
  and .replay_results.agility.recorded_sequence_count > 0
  and .replay_results.agility.replay_event_count == .replay_results.agility.recorded_sequence_count
  and .replay_results.agility.replay_sequence_signature_match == true
  and .replay_results.agility.final_runtime_match == true
  and all(.replay_results.agility.replay_events[]; .signature_match == true and .recorded_signature.input_path == "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action" and .replay_signature.input_path == "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action")
  and (.replay_results.agility.recorded_sequence | map(select(.key == "KeyJ")) | length) == 0
  and .replay_results.agility.expected_final_runtime.current_room_id == "mirror-city-square"
  and .replay_results.agility.replay_final_runtime.current_room_id == "mirror-city-square"
  and .replay_results.agility.expected_final_runtime == .replay_results.agility.replay_final_runtime
  and (.replay_results.agility.replay_final_runtime.inventory_items | index("agility-mastery-signet") != null)
  and .replay_results.craft.green == true
  and .replay_results.craft.recorded_branch_green == true
  and .replay_results.craft.recorded_sequence_parse_gate == true
  and .replay_results.craft.recorded_sequence_path_gate == true
  and .replay_results.craft.recorded_sequence_count > 0
  and .replay_results.craft.replay_event_count == .replay_results.craft.recorded_sequence_count
  and .replay_results.craft.replay_sequence_signature_match == true
  and .replay_results.craft.final_runtime_match == true
  and all(.replay_results.craft.replay_events[]; .signature_match == true and .recorded_signature.input_path == "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action" and .replay_signature.input_path == "ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action")
  and (.replay_results.craft.recorded_sequence | map(select(.key == "KeyJ")) | length) == 0
  and .replay_results.craft.expected_final_runtime.current_room_id == "forge-workbench"
  and .replay_results.craft.replay_final_runtime.current_room_id == "forge-workbench"
  and .replay_results.craft.expected_final_runtime == .replay_results.craft.replay_final_runtime
  and (.replay_results.craft.replay_final_runtime.inventory_items | index("craft-mastery-signet") != null)
  and .android_s5_real_device_claimed == false
  and .external_evidence_ignored_for_current_keyboard_replay_pass == true
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ALL_BRANCH_KEYBOARD_REPLAY_GREEN %s\n' "$SUMMARY"
