#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-live-window-texture-correlation.json"
RUNTIME_SUMMARY="$EVIDENCE_DIR/bevy-runtime-texture-asset.json"
RUNTIME_MANIFEST="$EVIDENCE_DIR/bevy-runtime-texture-asset-manifest.json"
PIXEL_SUMMARY="$EVIDENCE_DIR/bevy-live-window-layer-pixel-probe.json"
mkdir -p "$EVIDENCE_DIR"

"$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_asset.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_live_window_layer_pixel_probe.sh" >/dev/null

test -s "$RUNTIME_SUMMARY"
test -s "$RUNTIME_MANIFEST"
test -s "$PIXEL_SUMMARY"

python3 - "$RUNTIME_SUMMARY" "$RUNTIME_MANIFEST" "$PIXEL_SUMMARY" "$SUMMARY" <<'PY'
import json
import sys
from pathlib import Path

runtime_summary_path = Path(sys.argv[1])
runtime_manifest_path = Path(sys.argv[2])
pixel_summary_path = Path(sys.argv[3])
summary_path = Path(sys.argv[4])

runtime = json.loads(runtime_summary_path.read_text())
manifest = json.loads(runtime_manifest_path.read_text())
pixel = json.loads(pixel_summary_path.read_text())

image_handle = runtime.get("image_asset_handle_id")
layout_handle = runtime.get("texture_atlas_layout_handle_id")
frame_count = int(runtime.get("frame_count", 0))
sprite_bindings = manifest.get("sprite_bindings", [])
material_handles = {
    material.get("material_asset_handle_id")
    for material in manifest.get("material_asset_manifest", [])
}
required_layers = {"map", "hud", "actor", "feedback"}
min_colors = {
    "map": 3000,
    "hud": 1000,
    "actor": 3000,
    "feedback": 1000,
}

layer_correlations = []
for probe in pixel.get("probes", []):
    layer = probe.get("scene_layer")
    layer_bindings = [
        binding for binding in sprite_bindings if binding.get("scene_layer") == layer
    ]
    indexes = [
        int(binding.get("texture_atlas_index", -1))
        for binding in layer_bindings
        if isinstance(binding.get("texture_atlas_index"), int)
    ]
    material_slots = sorted({
        binding.get("material_slot") for binding in layer_bindings if binding.get("material_slot")
    })
    layer_material_handles = sorted({
        binding.get("material_asset_handle_id")
        for binding in layer_bindings
        if binding.get("material_asset_handle_id")
    })
    bindings_have_image_handle = all(
        binding.get("image_asset_handle_id") == image_handle for binding in layer_bindings
    )
    bindings_have_layout_handle = all(
        binding.get("texture_atlas_layout_handle_id") == layout_handle for binding in layer_bindings
    )
    indexes_valid = bool(indexes) and all(0 <= index < frame_count for index in indexes)
    material_handles_valid = bool(layer_material_handles) and all(
        handle in material_handles for handle in layer_material_handles
    )
    pixel_sampled_colors = int(probe.get("sampled_colors", 0))
    pixel_gate = probe.get("passes") is True and pixel_sampled_colors >= min_colors.get(layer, 1)
    passes = (
        layer in required_layers
        and bool(layer_bindings)
        and bindings_have_image_handle
        and bindings_have_layout_handle
        and indexes_valid
        and material_handles_valid
        and pixel_gate
    )
    layer_correlations.append(
        {
            "scene_layer": layer,
            "probe_id": probe.get("probe_id"),
            "pixel_sampled_colors": pixel_sampled_colors,
            "pixel_avg_stddev": probe.get("avg_stddev"),
            "pixel_region": probe.get("region"),
            "sprite_binding_count": len(layer_bindings),
            "texture_atlas_indexes": indexes,
            "material_slots": material_slots,
            "material_asset_handles": layer_material_handles,
            "image_asset_handle_id": image_handle,
            "texture_atlas_layout_handle_id": layout_handle,
            "bindings_have_image_handle": bindings_have_image_handle,
            "bindings_have_texture_atlas_layout_handle": bindings_have_layout_handle,
            "texture_atlas_indexes_valid": indexes_valid,
            "material_handles_valid": material_handles_valid,
            "pixel_gate": pixel_gate,
            "passes": passes,
        }
    )

passed_layers = {correlation["scene_layer"] for correlation in layer_correlations if correlation["passes"]}
gates = {
    "runtime_texture_asset_gate": runtime.get("green") is True,
    "live_window_pixel_probe_gate": pixel.get("green") is True,
    "image_handle_gate": image_handle == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1",
    "texture_atlas_layout_gate": layout_handle == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1",
    "material_slot_correlation_gate": all(
        bool(correlation["material_slots"]) and correlation["material_handles_valid"]
        for correlation in layer_correlations
        if correlation["scene_layer"] in required_layers
    ),
    "sprite_binding_correlation_gate": all(
        correlation["sprite_binding_count"] > 0
        and correlation["bindings_have_image_handle"]
        and correlation["bindings_have_texture_atlas_layout_handle"]
        and correlation["texture_atlas_indexes_valid"]
        for correlation in layer_correlations
        if correlation["scene_layer"] in required_layers
    ),
    "pixel_region_correlation_gate": all(
        correlation["pixel_gate"]
        for correlation in layer_correlations
        if correlation["scene_layer"] in required_layers
    ),
    "four_layer_texture_window_correlation_gate": required_layers.issubset(passed_layers),
    "boundary_gate": (
        runtime.get("gpu_upload_claimed") is False
        and runtime.get("android_s5_real_device_claimed") is False
        and runtime.get("live_osm_ingestion_claimed") is False
        and pixel.get("gpu_upload_claimed") is False
        and pixel.get("android_s5_real_device_claimed") is False
        and pixel.get("live_osm_ingestion_claimed") is False
    ),
}

summary = {
    "contract_version": "trillionnium_world_bevy_live_window_texture_correlation_v1",
    "green": all(gates.values()),
    "runtime_texture_asset_contract": "trillionnium_world_bevy_runtime_texture_asset_v1",
    "live_window_layer_pixel_probe_contract": "trillionnium_world_bevy_live_window_layer_pixel_probe_v1",
    "runtime_summary_path": str(runtime_summary_path),
    "runtime_manifest_path": str(runtime_manifest_path),
    "pixel_summary_path": str(pixel_summary_path),
    "final_frame_path": pixel.get("final_frame_path"),
    "image_asset_handle_id": image_handle,
    "texture_atlas_layout_handle_id": layout_handle,
    "frame_count": frame_count,
    "sprite_binding_count": len(sprite_bindings),
    "runtime_asset_bytes": int(runtime.get("runtime_asset_bytes", 0)),
    "live_frame_count": int(pixel.get("live_frame_count", 0)),
    "live_final_frame_colors_96x54": int(pixel.get("live_final_frame_colors_96x54", 0)),
    "layer_correlations": layer_correlations,
    "passed_layers": sorted(passed_layers),
    "gates": gates,
    "asset_boundary": "live_window_pixels_correlated_to_host_side_bevy_texture_handles_not_gpu_upload_claim",
    "source_of_truth": "This gate correlates the final live-window PNG region probes with the host-side Bevy Image handle, TextureAtlasLayout handle, material handles, and texture atlas indexes produced from the authored runtime texture asset manifest. It proves traceability across evidence artifacts, not GPU upload or Android S5 device rendering.",
    "gpu_upload_claimed": False,
    "android_s5_real_device_claimed": False,
    "live_osm_ingestion_claimed": False,
}
summary_path.write_text(json.dumps(summary, indent=2))
PY

jq -e '
  .contract_version == "trillionnium_world_bevy_live_window_texture_correlation_v1"
  and .green == true
  and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .live_window_layer_pixel_probe_contract == "trillionnium_world_bevy_live_window_layer_pixel_probe_v1"
  and .image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
  and .texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
  and .frame_count >= 32
  and .sprite_binding_count >= 32
  and .runtime_asset_bytes > 8192
  and .live_frame_count >= 11
  and .live_final_frame_colors_96x54 >= 1000
  and .gates.runtime_texture_asset_gate == true
  and .gates.live_window_pixel_probe_gate == true
  and .gates.image_handle_gate == true
  and .gates.texture_atlas_layout_gate == true
  and .gates.material_slot_correlation_gate == true
  and .gates.sprite_binding_correlation_gate == true
  and .gates.pixel_region_correlation_gate == true
  and .gates.four_layer_texture_window_correlation_gate == true
  and .gates.boundary_gate == true
  and ([.layer_correlations[] | select(.scene_layer == "map" and .passes == true and .sprite_binding_count >= 1 and .pixel_sampled_colors >= 3000)] | length) == 1
  and ([.layer_correlations[] | select(.scene_layer == "hud" and .passes == true and .sprite_binding_count >= 1 and .pixel_sampled_colors >= 1000)] | length) == 1
  and ([.layer_correlations[] | select(.scene_layer == "actor" and .passes == true and .sprite_binding_count >= 1 and .pixel_sampled_colors >= 3000)] | length) == 1
  and ([.layer_correlations[] | select(.scene_layer == "feedback" and .passes == true and .sprite_binding_count >= 1 and .pixel_sampled_colors >= 1000)] | length) == 1
  and .asset_boundary == "live_window_pixels_correlated_to_host_side_bevy_texture_handles_not_gpu_upload_claim"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_LIVE_WINDOW_TEXTURE_CORRELATION_GREEN $SUMMARY"
