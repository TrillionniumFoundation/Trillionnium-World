#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_economy_build.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_economy_build_v1'
  'bevy-classic-rts-economy-build.json'
  'bevy-classic-rts-economy-build.ppm'
  'classic-rts-economy-build'
  'input_path == "apply_live_native_action_with_source(classic_rts_economy_input)"'
  'RTS:QUEUE:harvest:gold_vein'
  'RTS:QUEUE:build:watch_tower@7,4'
  'RTS:QUEUE:train:worker'
  'harvest_loop_gate == true'
  'build_loop_gate == true'
  'production_loop_gate == true'
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_economy_core_frame_order_gate == true'
  'rts_economy_core_headless_replay_gate == true'
  'rts_economy_core_headless_applied_order_count == 3'
  'rts_economy_core_lifecycle_order_count == 2'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS economy script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ECONOMY_BUILD_CONTRACT'
  'native_classic_rts_economy_build_evidence_json'
  'classic-rts-economy-build'
  'classic_rts_economy_input'
  'rts_harvest_node_ids'
  'rts_worker_assignment_ids'
  'rts_dropoff_structure_id'
  'rts_resource_delta_log'
  'rts_build_site_tile_ids'
  'rts_building_blueprint_id'
  'rts_building_progress_percent'
  'rts_economy_state'
  'classic_rts_harvest_tile_for_node'
  'classic_rts_build_site_tiles'
  'CLASSIC_RTS_HARVEST_NODE_COLOR'
  'CLASSIC_RTS_WORKER_ROUTE_COLOR'
  'CLASSIC_RTS_DROPOFF_COLOR'
  'CLASSIC_RTS_BUILD_BLUEPRINT_COLOR'
  'CLASSIC_RTS_BLUEPRINT_PROGRESS_COLOR'
  'RtsFrameOrder::from_live_command_label'
  'first-contact-basin-economy-build'
  'trnm-rts-core-economy-build-rules-v1'
  'rts_economy_core_frame_order_gate'
  'rts_economy_core_headless_replay_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS economy source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_economy_build.sh'
  'bevy-classic-rts-economy-build.json'
  'classic_rts_economy_build_green'
  'rts_economy_harvest_loop_gate'
  'rts_economy_build_loop_gate'
  'rts_economy_production_loop_gate'
  'rts_economy_worker_route_pixel_count'
  'rts_economy_core_frame_order_count'
  'rts_economy_core_headless_applied_order_count'
  'rts_economy_core_frame_order_gate'
  'rts_economy_core_headless_replay_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS economy readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS economy build evidence remains connected to live input, resource/build runtime state, renderer overlays, and readiness"
