#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay.sh"

required_lines=(
  'bevy-build-branch-title-route-all-branch-keyboard-replay.json'
  'SUMMARY_RAW="$SUMMARY.raw"'
  'build-branch-title-route-all-branch-keyboard-replay >"$SUMMARY_RAW"'
  'status = "keyboard_replay_green"'
  'trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1'
  'trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_loop_v1'
  'status == "keyboard_replay_green"'
  'all_branch_keyboard_loop_contract_green == true'
  'branch_count == 3'
  'all_branch_replay_gate == true'
  'ready_for_release_review = true'
  'proof_scope = "host_side_bevy_runtime_replay_not_android_real_device"'
  'green_replay_result_count'
  'recorded_branch_green_count'
  'recorded_sequence_total_count'
  'replay_event_total_count'
  'final_runtime_match_count'
  'combat_victory_branch_count'
  'reward_item_count'
  'branches = {'
  'branches.force.recorded_sequence_count == 10'
  'branches.agility.recorded_sequence_count == 8'
  'branches.craft.recorded_sequence_count == 7'
  'replayed_stat_ids | index("force")'
  'replayed_stat_ids | index("agility")'
  'replayed_stat_ids | index("craft")'
  'replay_results.force.recorded_sequence_count > 0'
  'replay_results.force.replay_event_count == .replay_results.force.recorded_sequence_count'
  'replay_results.force.replay_sequence_signature_match == true'
  'replay_results.force.final_runtime_match == true'
  'ButtonInput<KeyCode> -> handle_native_keyboard_input -> apply_live_native_action'
  'replay_results.force.replay_final_runtime.combat_result_state == "victory"'
  'force-mastery-signet'
  'replay_results.agility.replay_final_runtime.current_room_id'
  'agility-mastery-signet'
  'replay_results.craft.replay_final_runtime.current_room_id'
  'craft-mastery-signet'
  'external_evidence_ignored_for_current_keyboard_replay_pass == true'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'warcraft_iii_asset_copied == false'
  'openra_asset_copied == false'
  'third_party_asset_copied == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing contract guard line: $line" >&2
    exit 1
  fi
done

echo "[PASS] keyboard replay script contract guard covers status-bound replay semantics and no-credit boundaries"
