#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip_v1'
  'bevy-classic-rts-control-group-command-feedback-strip.json'
  'bevy-classic-rts-control-group-command-feedback-strip.ppm'
  'classic-rts-control-group-command-feedback-strip'
  '.renderer_path == "classic_draw_scene"'
  'group_26_strip_gate == true'
  'group_27_strip_gate == true'
  'group_28_strip_gate == true'
  'filtered_cleared_strip_gate == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_FEEDBACK_STRIP_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS control group command feedback strip script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_FEEDBACK_STRIP_CONTRACT'
  'native_classic_rts_control_group_command_feedback_strip_evidence_json'
  'classic_rts_control_group_command_feedback_strip_stage'
  'classic_draw_rts_control_group_command_feedback_strip_overlay'
  'CLASSIC_RTS_COMMAND_STRIP_HUD_COLOR'
  'CLASSIC_RTS_COMMAND_STRIP_CANCEL_COLOR'
  'CLASSIC_RTS_COMMAND_STRIP_ANCHOR_COLOR'
  'Original Trillionnium control-group command feedback strip'
  'classic-rts-control-group-command-feedback-strip'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS control group command feedback strip source line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip_v1'
  'bevy_classic_rts_control_group_command_feedback_strip_contract_guard'
  'bevy_classic_rts_control_group_command_feedback_strip_gate'
  'bevy_classic_rts_control_group_command_feedback_strip_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS control group command feedback strip release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control group command feedback strip remains connected to classic_draw_scene, CLI, release-review, group-26 queued order, group-27 cancel/override, group-28 formation, filtering, clearing, and original art policy"
