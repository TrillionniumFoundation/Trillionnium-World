#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-all-branch-keyboard-replay.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-all-branch-keyboard-replay >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1"
  and .build_branch_title_route_all_branch_keyboard_loop_contract == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_loop_v1"
  and .green == true
  and .all_branch_keyboard_loop_contract_green == true
  and .branch_count == 3
  and .all_branch_replay_gate == true
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
  and .replay_results.craft.expected_final_runtime == .replay_results.craft.replay_final_runtime
  and (.replay_results.craft.replay_final_runtime.inventory_items | index("craft-mastery-signet") != null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ALL_BRANCH_KEYBOARD_REPLAY_GREEN %s\n' "$SUMMARY"
