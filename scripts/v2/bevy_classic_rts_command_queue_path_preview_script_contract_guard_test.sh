#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_queue_path_preview.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1'
  'bevy-classic-rts-command-queue-path-preview.json'
  'bevy-classic-rts-command-queue-path-preview.ppm'
  'classic-rts-command-queue-path-preview'
  'queue_stack_gate == true'
  'shift_waypoint_gate == true'
  'rally_chain_gate == true'
  'attack_focus_gate == true'
  'build_reservation_gate == true'
  'cancel_repath_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_QUEUE_PATH_PREVIEW_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS command queue path preview script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_QUEUE_PATH_PREVIEW_CONTRACT'
  'native_classic_rts_command_queue_path_preview_evidence_json'
  'classic_draw_rts_command_queue_path_preview_overlay'
  'classic_rts_command_queue_path_preview_stage'
  'CLASSIC_RTS_QUEUE_PREVIEW_SLOT_COLOR'
  'CLASSIC_RTS_QUEUE_PREVIEW_PATH_COLOR'
  'CLASSIC_RTS_QUEUE_PREVIEW_WAYPOINT_COLOR'
  'CLASSIC_RTS_QUEUE_PREVIEW_TARGET_COLOR'
  'CLASSIC_RTS_QUEUE_PREVIEW_RESERVATION_COLOR'
  'CLASSIC_RTS_QUEUE_PREVIEW_CANCEL_COLOR'
  'Original Trillionnium command queue and path preview overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS command queue path preview source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_command_queue_path_preview.sh'
  'bevy-classic-rts-command-queue-path-preview.json'
  'classic_rts_command_queue_path_preview_green'
  'rts_command_queue_path_preview_live_input_gate'
  'rts_command_queue_path_preview_queue_stack_gate'
  'rts_command_queue_path_preview_shift_waypoint_gate'
  'rts_command_queue_path_preview_rally_chain_gate'
  'rts_command_queue_path_preview_attack_focus_gate'
  'rts_command_queue_path_preview_build_reservation_gate'
  'rts_command_queue_path_preview_cancel_repath_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS command queue path preview readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1'
  'bevy_classic_rts_command_queue_path_preview_contract_guard'
  'bevy_classic_rts_command_queue_path_preview_gate'
  'bevy_classic_rts_command_queue_path_preview_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_command_queue_path_preview.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS command queue path preview release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS command queue path preview evidence remains connected to renderer, CLI, readiness, release-review, live input, queue/path runtime, and original art policy"
