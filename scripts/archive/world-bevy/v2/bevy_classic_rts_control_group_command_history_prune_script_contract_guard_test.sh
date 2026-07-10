#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_command_history_prune.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_history_prune_v1'
  'bevy-classic-rts-control-group-command-history-prune.json'
  'bevy-classic-rts-control-group-command-history-prune.ppm'
  'classic-rts-control-group-command-history-prune'
  '.renderer_path == "classic_draw_scene"'
  'retained_entry_gate == true'
  'pruned_entry_gate == true'
  'no_overflow_gate == true'
  'cleared_active_stale_pixel_count == 0'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_PRUNE_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS control group command history prune script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_PRUNE_CONTRACT'
  'native_classic_rts_control_group_command_history_prune_evidence_json'
  'classic_rts_control_group_command_history_prune_visible'
  'CLASSIC_RTS_COMMAND_HISTORY_PRUNED_COLOR'
  'CLASSIC_RTS_COMMAND_HISTORY_LIMIT_COLOR'
  'control_group_command_history_prune:'
  'history_row_pruned:25'
  'Original Trillionnium control-group command history prune HUD'
  'classic-rts-control-group-command-history-prune'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS control group command history prune source line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_command_history_prune_v1'
  'bevy_classic_rts_control_group_command_history_prune_contract_guard'
  'bevy_classic_rts_control_group_command_history_prune_gate'
  'bevy_classic_rts_control_group_command_history_prune_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_control_group_command_history_prune.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS control group command history prune release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control group command history prune remains connected to classic_draw_scene, CLI, release-review, recent-3 capacity, pruned-old rows, no-overflow guard, active-strip-cleared retention, no-stale-chip guard, and original art policy"
