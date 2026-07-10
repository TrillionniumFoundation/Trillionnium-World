#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-manifest-probe.json"
RUNTIME_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset.json"
RUNTIME_MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_asset.sh" >/dev/null
test -s "$RUNTIME_SUMMARY"
test -s "$RUNTIME_MANIFEST"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" runtime-texture-manifest-probe "$RUNTIME_SUMMARY" "$RUNTIME_MANIFEST" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_runtime_texture_manifest_probe_v1"
  and .green == true
  and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1"
  and .runtime_summary_gate == true
  and .app_resource_registered_gate == true
  and .runtime_probe_gate == true
  and .manifest_hash_gate == true
  and .handle_gate == true
  and (.runtime_manifest_sha256 | length) == 64
  and .app_resource_probe.contract_version == "trillionnium_world_bevy_runtime_texture_manifest_probe_v1"
  and .app_resource_probe.green == true
  and .app_resource_probe.manifest_hash_gate == true
  and .app_resource_probe.handle_gate == true
  and .app_resource_probe.image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
  and .app_resource_probe.texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
  and .app_resource_probe.frame_count >= 32
  and .app_resource_probe.sprite_binding_count >= 32
  and .app_resource_probe.material_asset_count == 4
  and .runtime_probe.runtime_texture_manifest_probe_gate == true
  and .runtime_probe.runtime_texture_manifest_probe.computed_sha256 == .runtime_manifest_sha256
  and .runtime_probe.runtime_texture_manifest_probe.image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
  and (.host_log_line | contains("TRNM_WORLD_BEVY_RUNTIME_TEXTURE_MANIFEST_PROBE"))
  and .asset_boundary == "bevy_app_resource_reads_runtime_texture_manifest_env_and_reports_hash_not_gpu_upload_claim"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_RUNTIME_TEXTURE_MANIFEST_PROBE_GREEN $SUMMARY"
