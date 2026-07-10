#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_pathing_formation.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_pathing_formation_v1'
  'bevy-classic-rts-pathing-formation.json'
  'bevy-classic-rts-pathing-formation.ppm'
  'classic-rts-pathing-formation'
  'input_path == "apply_live_native_action_with_source(classic_rts_pathing_input)"'
  'RTS:MOVE:8,4:wedge'
  'path:6,5>7,5>8,4'
  'blocked:7,4'
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_pathing_core_frame_order_gate == true'
  'rts_pathing_core_headless_replay_gate == true'
  'path_tile_gate == true'
  'blocked_tile_gate == true'
  'formation_slot_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS pathing script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PATHING_FORMATION_CONTRACT'
  'native_classic_rts_pathing_formation_evidence_json'
  'classic-rts-pathing-formation'
  'classic_rts_pathing_input'
  'rts_path_tile_ids'
  'rts_blocked_tile_ids'
  'rts_formation_slot_tile_ids'
  'classic_rts_path_tiles_for_destination'
  'classic_rts_blocked_tiles_for_destination'
  'classic_rts_formation_slots_for_destination'
  'TRNM_RTS_CORE_CONTRACT'
  'RtsFrameOrder::from_live_command_label'
  'RtsFrameOrderStream::new'
  'rts_pathing_core_frame_order_gate'
  'rts_pathing_core_headless_replay_gate'
  'CLASSIC_RTS_PATH_TILE_COLOR'
  'CLASSIC_RTS_BLOCKED_TILE_COLOR'
  'CLASSIC_RTS_FORMATION_SLOT_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS pathing source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_pathing_formation.sh'
  'bevy-classic-rts-pathing-formation.json'
  'classic_rts_pathing_formation_green'
  'rts_pathing_path_tile_gate'
  'rts_pathing_blocked_tile_gate'
  'rts_pathing_formation_slot_gate'
  'rts_pathing_core_frame_order_gate'
  'rts_pathing_core_headless_replay_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS pathing readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS pathing formation evidence remains connected to live input, renderer overlays, and readiness"
