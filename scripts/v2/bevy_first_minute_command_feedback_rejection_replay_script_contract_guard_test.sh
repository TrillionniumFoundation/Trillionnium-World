#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_first_minute_command_feedback_rejection_replay_v1'
  'trillionnium_world_bevy_first_minute_command_feedback_rejection_recording_v1'
  'bevy-first-minute-command-feedback-rejection-replay.json'
  'bevy-first-minute-command-feedback-rejection-recording.json'
  'bevy-first-minute-command-feedback-rejection-replay.ppm'
  'first-minute-command-feedback-rejection-replay'
  '.renderer_path == "first_minute_input_replay+classic_draw_scene"'
  '.input_path == "apply_live_native_action_with_source(first_minute_command_feedback_rejection_replay_input)"'
  '.blocked_command_input_count == 6'
  '.accepted_command_input_count == 1'
  '.blocked_history_non_pollution_gate == true'
  '.command_queue_rejection_pollution_count == 0'
  '.cleared_active_stale_pixel_count == 0'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_REJECTION_REPLAY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing first-minute command feedback rejection replay script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_REJECTION_REPLAY_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_REJECTION_RECORDING_CONTRACT'
  'native_first_minute_command_feedback_rejection_replay_evidence_json'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_INPUT_REPLAY_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_INPUT_RECORDING_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_PRUNE_CONTRACT'
  'first_minute_command_feedback_rejection_replay_input'
  'first_minute_input_replay+classic_draw_scene'
  'rts_group_selection_required'
  'rts_invalid_tile:bad-tile'
  'rts_attack_target_required'
  'rts_attack_required_before_ability'
  'rts_queue_id_required'
  'rts_group_id_required'
  'blocked_step_non_pollution_gate'
  'Original Trillionnium first-minute command feedback rejection replay HUD'
  'first-minute-command-feedback-rejection-replay'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing first-minute command feedback rejection replay source line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_first_minute_command_feedback_rejection_replay_v1'
  'bevy_first_minute_command_feedback_rejection_replay_contract_guard'
  'bevy_first_minute_command_feedback_rejection_replay_gate'
  'bevy_first_minute_command_feedback_rejection_replay_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing first-minute command feedback rejection replay release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] first-minute command feedback rejection replay remains connected to first-minute input replay, disk rejection recording, blocked live RTS input feedback, command-history non-pollution, classic_draw_scene, recent-3 history/prune capacity, no-stale-chip guard, CLI, release-review, and original art policy"
