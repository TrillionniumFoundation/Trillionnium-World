#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_fog_scouting_intel.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_fog_scouting_intel_v1'
  'bevy-classic-rts-fog-scouting-intel.json'
  'bevy-classic-rts-fog-scouting-intel.ppm'
  'classic-rts-fog-scouting-intel'
  'input_path == "apply_live_native_action_with_source(classic_rts_fog_scouting_intel_input)"'
  'RTS:QUEUE:recon:scout_enemy_base@10,2'
  'RTS:MOVE:9,2:rally'
  'RTS:QUEUE:recon:sweep:enemy_base@10,2'
  'RTS:QUEUE:recon:watchtower_scan@7,4'
  'RTS:QUEUE:recon:mark:enemy_base@10,2'
  'scout_route_gate == true'
  'fog_reveal_gate == true'
  'enemy_structure_intel_gate == true'
  'enemy_unit_intel_gate == true'
  'intel_log_gate == true'
  'visibility_bar_gate == true'
  'rts_fog_core_frame_order_gate == true'
  'rts_fog_core_headless_replay_gate == true'
  'rts_fog_core_headless_recon_order_count == 4'
  'rts_fog_core_headless_mark_order_count == 1'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS fog scouting intel script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FOG_SCOUTING_INTEL_CONTRACT'
  'native_classic_rts_fog_scouting_intel_evidence_json'
  'classic-rts-fog-scouting-intel'
  'classic_rts_fog_scouting_intel_input'
  'rts_scout_unit_ids'
  'rts_scout_route_tile_ids'
  'rts_fog_reveal_tile_ids'
  'rts_revealed_enemy_structure_ids'
  'rts_revealed_enemy_unit_ids'
  'rts_intel_log'
  'rts_visibility_percent'
  'RtsFrameOrder::from_live_command_label'
  'RtsFrameOrderStream::new'
  'rts_fog_core_frame_order_gate'
  'rts_fog_core_headless_replay_gate'
  'rts_fog_core_headless_recon_ids'
  'CLASSIC_RTS_SCOUT_ROUTE_COLOR'
  'CLASSIC_RTS_FOG_REVEAL_COLOR'
  'CLASSIC_RTS_ENEMY_INTEL_COLOR'
  'CLASSIC_RTS_ENEMY_STRUCTURE_COLOR'
  'CLASSIC_RTS_VISIBILITY_BAR_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS fog scouting intel source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_fog_scouting_intel.sh'
  'bevy-classic-rts-fog-scouting-intel.json'
  'classic_rts_fog_scouting_intel_green'
  'rts_fog_scouting_intel_live_input_gate'
  'rts_fog_scouting_intel_scout_route_gate'
  'rts_fog_scouting_intel_fog_reveal_gate'
  'rts_fog_scouting_intel_enemy_structure_gate'
  'rts_fog_scouting_intel_enemy_unit_gate'
  'rts_fog_scouting_intel_intel_log_gate'
  'rts_fog_scouting_intel_visibility_gate'
  'rts_fog_scouting_intel_core_frame_order_gate'
  'rts_fog_scouting_intel_core_headless_replay_gate'
  'rts_fog_scouting_intel_core_recon_order_count'
  'rts_fog_scouting_intel_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS fog scouting intel readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS fog scouting intel evidence remains connected to native scout input, fog reveal, enemy intel, minimap readability, trnm-rts-core replay, renderer overlays, and readiness"
