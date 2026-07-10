#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-asset-store-registration.json"
RUNTIME_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset.json"
RUNTIME_MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_manifest_probe.sh" >/dev/null
test -s "$RUNTIME_SUMMARY"
test -s "$RUNTIME_MANIFEST"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" asset-store-registration "$RUNTIME_SUMMARY" "$RUNTIME_MANIFEST" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_asset_store_registration_v1"
  and .green == true
  and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .runtime_texture_manifest_probe_contract == "trillionnium_world_bevy_runtime_texture_manifest_probe_v1"
  and .runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1"
  and .runtime_summary_gate == true
  and .manifest_probe_gate == true
  and .app_asset_store_registration_gate == true
  and .runtime_probe_gate == true
  and (.runtime_manifest_sha256 | length) == 64
  and .runtime_texture_manifest_probe.green == true
  and .asset_store_registration.contract_version == "trillionnium_world_bevy_asset_store_registration_v1"
  and .asset_store_registration.green == true
  and .asset_store_registration.ppm_parse_gate == true
  and .asset_store_registration.bevy_image_store_registration_gate == true
  and .asset_store_registration.texture_atlas_layout_store_registration_gate == true
  and .asset_store_registration.asset_store_registered_gate == true
  and .asset_store_registration.image_asset_count_after == (.asset_store_registration.image_asset_count_before + 1)
  and .asset_store_registration.texture_atlas_layout_count_after == (.asset_store_registration.texture_atlas_layout_count_before + 1)
  and .asset_store_registration.image_extent_width == 256
  and .asset_store_registration.image_extent_height == 128
  and .asset_store_registration.image_data_bytes == 131072
  and .asset_store_registration.image_unique_color_count >= 8
  and .asset_store_registration.texture_atlas_rect_count >= 32
  and .asset_store_registration.frame_count >= 32
  and .asset_store_registration.sprite_binding_count >= 32
  and .asset_store_registration.material_asset_count == 4
  and (.asset_store_registration.bevy_image_asset_id_debug | contains("index"))
  and (.asset_store_registration.bevy_texture_atlas_layout_asset_id_debug | contains("index"))
  and .asset_store_registration.render_asset_usage == "RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD"
  and .asset_store_registration.render_world_asset_usage_requested == true
  and .runtime_probe.runtime_texture_asset_store_registration_gate == true
  and .runtime_probe.runtime_texture_asset_store_registration.manifest_sha256 == .runtime_manifest_sha256
  and (.host_log_line | contains("TRNM_WORLD_BEVY_ASSET_STORE_REGISTRATION"))
  and .asset_boundary == "bevy_assets_main_world_image_and_texture_atlas_layout_registration_not_gpu_upload_claim"
  and .render_world_asset_usage_requested == true
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_ASSET_STORE_REGISTRATION_GREEN $SUMMARY"
