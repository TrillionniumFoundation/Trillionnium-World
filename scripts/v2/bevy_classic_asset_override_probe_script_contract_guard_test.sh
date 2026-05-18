#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_override_probe.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_asset_override_probe_v1'
  'bevy-classic-asset-override-probe.json'
  'bevy-classic-asset-override-probe.ppm'
  'classic-asset-overrides'
  'actor_player_idle_south.ppm'
  'TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR'
  'classic-asset-override-probe'
  'override_probe_pixel_count > 300'
  'replacement_boundary_gate == true'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
  'not_cex_runtime'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing asset override probe script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_PROBE_CONTRACT'
  'native_classic_asset_override_probe_evidence_json'
  'classic-asset-override-probe'
  'classic-override-probe'
  'TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR'
  'frame_override_pixels'
  'frame_override_dir'
  'load_classic_frame_overrides'
  'classic_frame_source_pixel'
  'actor_player_idle_south'
  'override_probe_pixel_count'
  'ff00ff'
  'project-local PPM frame overrides'
  'not_cex_runtime'
  'wgpu_required'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing asset override probe source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic asset override probe keeps the local asset replacement path"
