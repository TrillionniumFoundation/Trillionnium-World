#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-render-asset-eligibility.json"
RUNTIME_SUMMARY="$EVIDENCE_DIR/bevy-runtime-texture-asset.json"
RUNTIME_MANIFEST="$EVIDENCE_DIR/bevy-runtime-texture-asset-manifest.json"
SAMPLED_LIVE_CORRELATION="$EVIDENCE_DIR/bevy-live-window-sampled-texture-correlation.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh" >/dev/null
test -s "$RUNTIME_SUMMARY"
test -s "$RUNTIME_MANIFEST"
test -s "$SAMPLED_LIVE_CORRELATION"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- render-asset-eligibility "$RUNTIME_SUMMARY" "$RUNTIME_MANIFEST" "$SAMPLED_LIVE_CORRELATION" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_render_asset_eligibility_v1"
  and .green == true
  and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .sprite_texture_sampling_contract == "trillionnium_world_bevy_sprite_texture_sampling_v1"
  and .live_window_sampled_texture_correlation_contract == "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1"
  and .runtime_summary_gate == true
  and .asset_store_registration_gate == true
  and .sampled_live_correlation_gate == true
  and .render_asset_usage_gate == true
  and .image_descriptor_render_eligibility_gate == true
  and .atlas_layout_render_eligibility_gate == true
  and .sprite_render_reference_gate == true
  and .boundary_gate == true
  and (.runtime_manifest_sha256 | length) == 64
  and .image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
  and .texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
  and (.bevy_image_asset_id_debug | contains("index"))
  and (.bevy_texture_atlas_layout_asset_id_debug | contains("index"))
  and .image_present == true
  and .texture_atlas_layout_present == true
  and .image_asset_usage_main_world == true
  and .image_asset_usage_render_world == true
  and (.image_asset_usage_debug | contains("MAIN_WORLD"))
  and (.image_asset_usage_debug | contains("RENDER_WORLD"))
  and .image_format_debug == "Rgba8UnormSrgb"
  and .image_dimension_debug == "D2"
  and .image_dimensions.width == 256
  and .image_dimensions.height == 128
  and .image_dimensions.depth_or_array_layers == 1
  and .image_data_bytes == 131072
  and .texture_atlas_rect_count >= 32
  and .first_texture_rect_dimensions.width == 32
  and .first_texture_rect_dimensions.height == 32
  and .sprite_render_reference_count >= 24
  and (.sprite_render_references_sample | length) >= 8
  and .sprite_render_references_sample[0].render_asset_reference_gate == true
  and .asset_boundary == "bevy_image_render_asset_usage_eligible_not_render_world_extraction_or_gpu_upload_claim"
  and .host_side_render_asset_eligibility_claimed == true
  and .render_world_extraction_completed_claimed == false
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_RENDER_ASSET_ELIGIBILITY_GREEN $SUMMARY"
