#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_creep_camp_terrain_route.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_creep_camp_terrain_route_v1'
  'bevy-classic-rts-creep-camp-terrain-route.json'
  'bevy-classic-rts-creep-camp-terrain-route.ppm'
  'classic-rts-creep-camp-terrain-route'
  'input_path == "apply_live_native_action_with_source(classic_rts_creep_camp_terrain_route_input)"'
  'RTS:QUEUE:scout:creep_camp@8,3'
  'RTS:MOVE:8,3:wedge'
  'RTS:ATTACK:forest_creep_camp'
  'RTS:QUEUE:camp:clear:forest_creep_camp@8,3'
  'terrain_route_gate == true'
  'choke_gate == true'
  'camp_clear_gate == true'
  'scout_reveal_gate == true'
  'expansion_route_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS creep camp terrain route script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CREEP_CAMP_TERRAIN_ROUTE_CONTRACT'
  'native_classic_rts_creep_camp_terrain_route_evidence_json'
  'classic-rts-creep-camp-terrain-route'
  'classic_rts_creep_camp_terrain_route_input'
  'rts_creep_camp_tile_ids'
  'rts_creep_camp_unit_ids'
  'rts_creep_camp_state'
  'rts_terrain_route_tile_ids'
  'rts_terrain_choke_tile_ids'
  'rts_expansion_tile_ids'
  'rts_scout_reveal_percent'
  'CLASSIC_RTS_CREEP_CAMP_COLOR'
  'CLASSIC_RTS_TERRAIN_ROUTE_COLOR'
  'CLASSIC_RTS_CHOKE_COLOR'
  'CLASSIC_RTS_EXPANSION_COLOR'
  'CLASSIC_RTS_SCOUT_REVEAL_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS creep camp terrain route source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_creep_camp_terrain_route.sh'
  'bevy-classic-rts-creep-camp-terrain-route.json'
  'classic_rts_creep_camp_terrain_route_green'
  'rts_creep_camp_terrain_route_live_input_gate'
  'rts_creep_camp_terrain_route_terrain_gate'
  'rts_creep_camp_terrain_route_choke_gate'
  'rts_creep_camp_terrain_route_clear_gate'
  'rts_creep_camp_terrain_route_reveal_gate'
  'rts_creep_camp_terrain_route_expansion_gate'
  'rts_creep_camp_terrain_route_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS creep camp terrain route readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS creep camp terrain route evidence remains connected to scouting, choke routing, camp clear, expansion unlock, renderer overlays, and readiness"
