#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_surface.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_command_surface_v1'
  'bevy-classic-rts-command-surface.json'
  'bevy-classic-rts-command-surface.ppm'
  'classic-rts-command-surface'
  'selection_surface_gate == true'
  'command_grid_surface_gate == true'
  'cooldown_disabled_surface_gate == true'
  'target_queue_surface_gate == true'
  'surface_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_SURFACE_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS command surface script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_SURFACE_CONTRACT'
  'native_classic_rts_command_surface_evidence_json'
  'classic_rts_command_surface_stage'
  'classic_draw_rts_command_surface_overlay'
  'CLASSIC_RTS_COMMAND_SURFACE_SELECTION_FRAME_COLOR'
  'CLASSIC_RTS_COMMAND_SURFACE_READY_COLOR'
  'CLASSIC_RTS_COMMAND_SURFACE_DISABLED_COLOR'
  'CLASSIC_RTS_COMMAND_SURFACE_COOLDOWN_COLOR'
  'CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR'
  'CLASSIC_RTS_COMMAND_SURFACE_QUEUE_COLOR'
  'CLASSIC_RTS_COMMAND_SURFACE_GROUP_TAB_COLOR'
  'Original Trillionnium command-surface overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS command surface source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_command_surface.sh'
  'bevy-classic-rts-command-surface.json'
  'classic_rts_command_surface_green'
  'rts_command_surface_selection_surface_gate'
  'rts_command_surface_command_grid_surface_gate'
  'rts_command_surface_cooldown_disabled_surface_gate'
  'rts_command_surface_target_queue_surface_gate'
  'rts_command_surface_surface_stage_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS command surface readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_command_surface_v1'
  'bevy_classic_rts_command_surface_contract_guard'
  'bevy_classic_rts_command_surface_gate'
  'bevy_classic_rts_command_surface_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_command_surface.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS command surface release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS command surface evidence remains connected to renderer, CLI, readiness, release-review, and original art policy"
