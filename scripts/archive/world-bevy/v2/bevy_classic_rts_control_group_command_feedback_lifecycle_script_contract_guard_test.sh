#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle_v1'
  'bevy-classic-rts-control-group-command-feedback-lifecycle.json'
  'bevy-classic-rts-control-group-command-feedback-lifecycle.ppm'
  'classic-rts-control-group-command-feedback-lifecycle'
  '.renderer_path == "classic_draw_scene"'
  'fresh_visual_gate == true'
  'dimmed_visual_gate == true'
  'cleared_visual_gate == true'
  'no_stale_after_decay_gate == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_FEEDBACK_LIFECYCLE_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS control group command feedback lifecycle script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_FEEDBACK_LIFECYCLE_CONTRACT'
  'native_classic_rts_control_group_command_feedback_lifecycle_evidence_json'
  'classic_rts_control_group_command_feedback_lifecycle_stage'
  'classic_draw_rts_control_group_command_feedback_lifecycle_overlay'
  'CLASSIC_RTS_COMMAND_STRIP_DIM_HUD_COLOR'
  'CLASSIC_RTS_COMMAND_STRIP_DIM_CHIP_COLOR'
  'CLASSIC_RTS_COMMAND_STRIP_READY_COLOR'
  'Original Trillionnium control-group command feedback lifecycle HUD'
  'classic-rts-control-group-command-feedback-lifecycle'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS control group command feedback lifecycle source line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle_v1'
  'bevy_classic_rts_control_group_command_feedback_lifecycle_contract_guard'
  'bevy_classic_rts_control_group_command_feedback_lifecycle_gate'
  'bevy_classic_rts_control_group_command_feedback_lifecycle_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS control group command feedback lifecycle release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control group command feedback lifecycle remains connected to classic_draw_scene, CLI, release-review, fresh/dimmed/cleared HUD decay, no-stale-chip guard, group 26/27/28 semantics, and original art policy"
