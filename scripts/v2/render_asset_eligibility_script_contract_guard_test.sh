#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_render_asset_eligibility.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_render_asset_eligibility_v1'
  'render_asset_eligibility_green'
  'bevy-render-asset-eligibility.json'
  'render-asset-eligibility'
  'runtime-texture-render-eligibility'
  'check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh'
  'RenderAssetUsages::MAIN_WORLD'
  'RenderAssetUsages::RENDER_WORLD'
  'image_asset_usage_main_world'
  'image_asset_usage_render_world'
  'image_descriptor_render_eligibility_gate'
  'atlas_layout_render_eligibility_gate'
  'sprite_render_reference_gate'
  'render_world_extraction_completed_claimed'
  'external_evidence_ignored_for_current_render_asset_pass'
  'bevy_image_render_asset_usage_eligible_not_render_world_extraction_or_gpu_upload_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'third_party_asset_copied == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] render asset eligibility contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] render asset eligibility proves Bevy Image render-world usage flags without render extraction/GPU/device claims"
