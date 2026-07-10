#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_affordance.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_command_affordance_v1'
  'bevy-classic-rts-command-affordance.json'
  'bevy-classic-rts-command-affordance.ppm'
  'classic-rts-command-affordance'
  'drag_select_gate == true'
  'right_click_move_gate == true'
  'attack_cursor_gate == true'
  'hotkey_ack_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_AFFORDANCE_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS command affordance script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_AFFORDANCE_CONTRACT'
  'native_classic_rts_command_affordance_evidence_json'
  'classic_draw_rts_command_affordance_drag_marquee'
  'classic_draw_rts_command_affordance_cursor_arrow'
  'classic_draw_rts_command_affordance_target_marker'
  'classic_draw_rts_command_affordance_panel'
  'CLASSIC_RTS_COMMAND_AFFORDANCE_DRAG_COLOR'
  'CLASSIC_RTS_COMMAND_AFFORDANCE_RIGHT_CLICK_COLOR'
  'CLASSIC_RTS_COMMAND_AFFORDANCE_ATTACK_CURSOR_COLOR'
  'CLASSIC_RTS_COMMAND_AFFORDANCE_HOTKEY_COLOR'
  'Original Trillionnium RTS command affordances'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS command affordance source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_command_affordance.sh'
  'bevy-classic-rts-command-affordance.json'
  'classic_rts_command_affordance_green'
  'rts_command_affordance_drag_select_gate'
  'rts_command_affordance_right_click_move_gate'
  'rts_command_affordance_attack_cursor_gate'
  'rts_command_affordance_hotkey_ack_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS command affordance readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS command affordance evidence remains connected to renderer, live input, readiness, and original art policy"
