#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_army_production_rally.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_army_production_rally_v1'
  'bevy-classic-rts-army-production-rally.json'
  'bevy-classic-rts-army-production-rally.ppm'
  'classic-rts-army-production-rally'
  'input_path == "apply_live_native_action_with_source(classic_rts_army_production_rally_input)"'
  'RTS:QUEUE:army:supply:field_lodge@6,4'
  'RTS:QUEUE:army:train:guard_pair@training_hall'
  'RTS:QUEUE:army:train:wayfinder_pair@signal_spire'
  'RTS:QUEUE:army:rally:forward_watch@7,4'
  'RTS:QUEUE:army:assign:control_group_3@forward_watch'
  'supply_gate == true'
  'production_batch_gate == true'
  'rally_gate == true'
  'control_group_gate == true'
  'composition_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS army production/rally script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ARMY_PRODUCTION_RALLY_CONTRACT'
  'native_classic_rts_army_production_rally_evidence_json'
  'classic-rts-army-production-rally'
  'classic_rts_army_production_rally_input'
  'rts_army_supply_used'
  'rts_army_supply_cap'
  'rts_army_production_batch_ids'
  'rts_army_spawned_unit_ids'
  'rts_army_rally_tile_ids'
  'rts_army_composition_log'
  'rts_army_production_state'
  'CLASSIC_RTS_ARMY_SUPPLY_COLOR'
  'CLASSIC_RTS_ARMY_SPAWN_COLOR'
  'CLASSIC_RTS_RALLY_LINE_COLOR'
  'CLASSIC_RTS_COMPOSITION_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS army production/rally source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_army_production_rally.sh'
  'bevy-classic-rts-army-production-rally.json'
  'classic_rts_army_production_rally_green'
  'rts_army_production_rally_live_input_gate'
  'rts_army_production_rally_supply_gate'
  'rts_army_production_rally_production_batch_gate'
  'rts_army_production_rally_rally_gate'
  'rts_army_production_rally_control_group_gate'
  'rts_army_production_rally_composition_gate'
  'rts_army_production_rally_spawned_unit_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS army production/rally readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS army production/rally evidence remains connected to supply cap, multi-batch training, rally route, control-group assignment, composition overlays, and readiness"
