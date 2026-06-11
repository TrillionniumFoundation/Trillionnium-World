#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_collision_engagement.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_collision_engagement_v1'
  'bevy-classic-rts-collision-engagement.json'
  'bevy-classic-rts-collision-engagement.ppm'
  'classic-rts-collision-engagement'
  'input_path == "apply_live_native_action_with_source(classic_rts_collision_input)"'
  'RTS:MOVE:8,4:wedge'
  'RTS:ATTACK:arena_creep_attack'
  'blocked_detour_spread'
  'engaged:arena_creep_attack'
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_collision_core_frame_order_gate == true'
  'rts_collision_core_headless_replay_gate == true'
  'collision_response_gate == true'
  'engagement_response_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS collision script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COLLISION_ENGAGEMENT_CONTRACT'
  'native_classic_rts_collision_engagement_evidence_json'
  'classic-rts-collision-engagement'
  'classic_rts_collision_input'
  'rts_disperse_tile_ids'
  'rts_engagement_tile_ids'
  'rts_contact_flash_tile_ids'
  'rts_unit_response_state'
  'classic_rts_disperse_slots_for_destination'
  'classic_rts_engagement_tiles_for_target'
  'classic_rts_contact_flash_tiles_for_target'
  'TRNM_RTS_CORE_CONTRACT'
  'RtsFrameOrder::from_live_command_label'
  'RtsFrameOrderStream::new'
  'rts_collision_core_frame_order_gate'
  'rts_collision_core_headless_replay_gate'
  'CLASSIC_RTS_DISPERSION_SLOT_COLOR'
  'CLASSIC_RTS_ENGAGEMENT_RANGE_COLOR'
  'CLASSIC_RTS_CONTACT_FLASH_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS collision source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_collision_engagement.sh'
  'bevy-classic-rts-collision-engagement.json'
  'classic_rts_collision_engagement_green'
  'rts_collision_collision_response_gate'
  'rts_collision_engagement_response_gate'
  'rts_collision_dispersion_slot_pixel_count'
  'rts_collision_core_frame_order_gate'
  'rts_collision_core_headless_replay_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS collision readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS collision engagement evidence remains connected to live input, runtime response state, renderer overlays, and readiness"
