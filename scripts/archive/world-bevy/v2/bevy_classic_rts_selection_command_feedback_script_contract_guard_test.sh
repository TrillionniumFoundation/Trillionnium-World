#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_command_feedback.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_selection_command_feedback_v1'
  'bevy-classic-rts-selection-command-feedback.json'
  'bevy-classic-rts-selection-command-feedback.ppm'
  'classic-rts-selection-command-feedback'
  'marquee_gate == true'
  'confirm_gate == true'
  'rally_gate == true'
  'move_gate == true'
  'attack_gate == true'
  'error_gate == true'
  'ack_gate == true'
  'feedback_stage_gate == true'
  'command_runtime_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_COMMAND_FEEDBACK_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS selection command feedback script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_COMMAND_FEEDBACK_CONTRACT'
  'native_classic_rts_selection_command_feedback_evidence_json'
  'classic_rts_selection_command_feedback_stage'
  'classic_draw_rts_selection_command_feedback_overlay'
  'CLASSIC_RTS_SELECTION_FEEDBACK_MARQUEE_COLOR'
  'CLASSIC_RTS_SELECTION_FEEDBACK_CONFIRM_COLOR'
  'CLASSIC_RTS_SELECTION_FEEDBACK_RALLY_COLOR'
  'CLASSIC_RTS_SELECTION_FEEDBACK_MOVE_COLOR'
  'CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR'
  'CLASSIC_RTS_SELECTION_FEEDBACK_ERROR_COLOR'
  'CLASSIC_RTS_SELECTION_FEEDBACK_ACK_COLOR'
  'Original Trillionnium selection-command feedback overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS selection command feedback source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_selection_command_feedback.sh'
  'bevy-classic-rts-selection-command-feedback.json'
  'classic_rts_selection_command_feedback_green'
  'rts_selection_command_feedback_marquee_gate'
  'rts_selection_command_feedback_confirm_gate'
  'rts_selection_command_feedback_rally_gate'
  'rts_selection_command_feedback_move_gate'
  'rts_selection_command_feedback_attack_gate'
  'rts_selection_command_feedback_error_gate'
  'rts_selection_command_feedback_command_runtime_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS selection command feedback readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_selection_command_feedback_v1'
  'bevy_classic_rts_selection_command_feedback_contract_guard'
  'bevy_classic_rts_selection_command_feedback_gate'
  'bevy_classic_rts_selection_command_feedback_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_selection_command_feedback.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS selection command feedback release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS selection command feedback evidence remains connected to renderer, CLI, readiness, release-review, runtime command state, and original art policy"
