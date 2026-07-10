#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_first_minute_command_feedback_replay_v1'
  'trillionnium_world_bevy_first_minute_command_feedback_recording_v1'
  'bevy-first-minute-command-feedback-replay.json'
  'bevy-first-minute-command-feedback-recording.json'
  'bevy-first-minute-command-feedback-replay.ppm'
  'first-minute-command-feedback-replay'
  '.renderer_path == "first_minute_input_replay+classic_draw_scene"'
  '.input_path == "apply_live_native_action_with_source(first_minute_command_feedback_replay_input)"'
  '.first_minute_replay_gate == true'
  '.command_recording_parse_gate == true'
  '.live_command_input_gate == true'
  '.cleared_active_stale_pixel_count == 0'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_REPLAY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing first-minute command feedback replay script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_REPLAY_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_RECORDING_CONTRACT'
  'native_first_minute_command_feedback_replay_evidence_json'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_INPUT_REPLAY_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_INPUT_RECORDING_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_FEEDBACK_STRIP_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_FEEDBACK_LIFECYCLE_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_PRUNE_CONTRACT'
  'first_minute_command_feedback_replay_input'
  'first_minute_input_replay+classic_draw_scene'
  'control_group_command_history_prune:cleared_history_bounded'
  'history_row_pruned:25'
  'Original Trillionnium first-minute command feedback replay HUD'
  'first-minute-command-feedback-replay'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing first-minute command feedback replay source line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_first_minute_command_feedback_replay_v1'
  'bevy_first_minute_command_feedback_replay_contract_guard'
  'bevy_first_minute_command_feedback_replay_gate'
  'bevy_first_minute_command_feedback_replay_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing first-minute command feedback replay release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] first-minute command feedback replay remains connected to first-minute input replay, disk command recording, live RTS input, classic_draw_scene, recent-3 history/prune capacity, no-stale-chip guard, CLI, release-review, and original art policy"
