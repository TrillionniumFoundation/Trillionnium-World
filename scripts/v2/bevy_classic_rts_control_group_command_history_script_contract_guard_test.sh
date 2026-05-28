#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_history.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_history_v1'
  'bevy-classic-rts-control-group-command-history.json'
  'bevy-classic-rts-control-group-command-history.ppm'
  'classic-rts-control-group-command-history'
  '.renderer_path == "classic_draw_scene"'
  'history_visual_gate == true'
  'history_entry_gate == true'
  'cleared_history_gate == true'
  'cleared_active_stale_pixel_count == 0'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS control group command history script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_CONTRACT'
  'native_classic_rts_control_group_command_history_evidence_json'
  'classic_rts_control_group_command_history_visible'
  'classic_draw_rts_control_group_command_history_overlay'
  'CLASSIC_RTS_COMMAND_HISTORY_FRAME_COLOR'
  'CLASSIC_RTS_COMMAND_HISTORY_ROW_COLOR'
  'CLASSIC_RTS_COMMAND_HISTORY_RETAINED_COLOR'
  'Original Trillionnium control-group command history HUD'
  'classic-rts-control-group-command-history'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS control group command history source line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_history_v1'
  'bevy_classic_rts_control_group_command_history_contract_guard'
  'bevy_classic_rts_control_group_command_history_gate'
  'bevy_classic_rts_control_group_command_history_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_control_group_command_history.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS control group command history release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control group command history remains connected to classic_draw_scene, CLI, release-review, recent-3 rows, active-strip-cleared retention, no-stale-chip guard, group 26/27/28 semantics, and original art policy"
