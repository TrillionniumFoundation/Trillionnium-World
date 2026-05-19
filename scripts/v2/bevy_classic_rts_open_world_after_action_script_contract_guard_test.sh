#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_open_world_after_action.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_open_world_after_action_v1'
  'bevy-classic-rts-open-world-after-action.json'
  'bevy-classic-rts-open-world-after-action.ppm'
  'classic-rts-open-world-after-action'
  'input_path == "apply_live_native_action_with_source(classic_rts_open_world_after_action_input)"'
  'RTS:QUEUE:tier2:open_world:after_action@13,3'
  'RTS:QUEUE:tier2:open_world_route:league-coliseum@12,3'
  'RTS:QUEUE:tier2:open_world_resume:league-coliseum@12,3'
  'restoration_dependency_gate == true'
  'open_world_route_gate == true'
  'open_world_panel_gate == true'
  'open_world_resume_gate == true'
  'command_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS open-world script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPEN_WORLD_AFTER_ACTION_CONTRACT'
  'native_classic_rts_open_world_after_action_evidence_json'
  'classic-rts-open-world-after-action'
  'classic_rts_open_world_after_action_input'
  'rts_open_world_route_tile_ids'
  'rts_open_world_panel_ids'
  'rts_open_world_task_ids'
  'rts_open_world_handoff_state'
  'CLASSIC_RTS_OPEN_WORLD_ROUTE_COLOR'
  'CLASSIC_RTS_OPEN_WORLD_PANEL_COLOR'
  'CLASSIC_RTS_OPEN_WORLD_RESUME_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS open-world source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_open_world_after_action.sh'
  'bevy-classic-rts-open-world-after-action.json'
  'classic_rts_open_world_after_action_green'
  'rts_open_world_after_action_live_input_gate'
  'rts_open_world_after_action_restoration_dependency_gate'
  'rts_open_world_after_action_route_gate'
  'rts_open_world_after_action_panel_gate'
  'rts_open_world_after_action_resume_gate'
  'rts_open_world_after_action_command_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS open-world readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS open-world after-action evidence remains connected to restoration dependency, route, panels, resume room, and readiness"
