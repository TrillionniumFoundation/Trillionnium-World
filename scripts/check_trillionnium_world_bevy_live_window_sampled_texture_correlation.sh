#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-live-window-sampled-texture-correlation.json"
SAMPLING_SUMMARY="$EVIDENCE_DIR/bevy-sprite-texture-sampling.json"
LIVE_CORRELATION_SUMMARY="$EVIDENCE_DIR/bevy-live-window-texture-correlation.json"
mkdir -p "$EVIDENCE_DIR"

"$ROOT/scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_live_window_texture_correlation.sh" >/dev/null

test -s "$SAMPLING_SUMMARY"
test -s "$LIVE_CORRELATION_SUMMARY"

python3 - "$SAMPLING_SUMMARY" "$LIVE_CORRELATION_SUMMARY" "$SUMMARY" <<'PY'
import json
import sys
from pathlib import Path

sampling_path = Path(sys.argv[1])
live_correlation_path = Path(sys.argv[2])
summary_path = Path(sys.argv[3])

sampling = json.loads(sampling_path.read_text())
live = json.loads(live_correlation_path.read_text())
required_layers = {"map", "hud", "actor", "feedback"}
required_slots = {
    "world_tile_material",
    "hud_icon_material",
    "actor_sprite_material",
    "feedback_glyph_material",
}

sampled_layer_counts = sampling.get("sampled_layer_counts", {})
sampled_slot_counts = sampling.get("sampled_material_slot_counts", {})
live_layer_correlations = {
    correlation.get("scene_layer"): correlation
    for correlation in live.get("layer_correlations", [])
}

layer_correlations = []
for layer in sorted(required_layers):
    live_layer = live_layer_correlations.get(layer, {})
    sampled_count = int(sampled_layer_counts.get(layer, 0))
    live_passes = live_layer.get("passes") is True
    texture_indexes = live_layer.get("texture_atlas_indexes", [])
    layer_correlations.append(
        {
            "scene_layer": layer,
            "sampled_surface_count": sampled_count,
            "sampled_texture_gate": sampled_count >= 1,
            "live_window_texture_correlation_gate": live_passes,
            "live_pixel_sampled_colors": int(live_layer.get("pixel_sampled_colors", 0) or 0),
            "live_sprite_binding_count": int(live_layer.get("sprite_binding_count", 0) or 0),
            "texture_atlas_indexes": texture_indexes,
            "material_slots": live_layer.get("material_slots", []),
            "passes": sampled_count >= 1 and live_passes and bool(texture_indexes),
        }
    )

gates = {
    "sprite_texture_sampling_gate": sampling.get("green") is True,
    "live_window_texture_correlation_gate": live.get("green") is True,
    "same_image_handle_gate": (
        sampling.get("asset_store_registration", {}).get("image_asset_handle_id")
        == live.get("image_asset_handle_id")
        == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
    ),
    "same_texture_atlas_layout_gate": (
        sampling.get("asset_store_registration", {}).get("texture_atlas_layout_handle_id")
        == live.get("texture_atlas_layout_handle_id")
        == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
    ),
    "same_runtime_manifest_hash_gate": (
        len(str(sampling.get("runtime_manifest_sha256", ""))) == 64
        and sampling.get("runtime_manifest_sha256")
        == sampling.get("asset_store_registration", {}).get("manifest_sha256")
    ),
    "sampled_layer_count_gate": all(
        int(sampled_layer_counts.get(layer, 0)) >= 1 for layer in required_layers
    ),
    "sampled_material_slot_count_gate": all(
        int(sampled_slot_counts.get(slot, 0)) >= 1 for slot in required_slots
    ),
    "sampled_texture_nonblank_gate": (
        sampling.get("texture_sample_nonblank_gate") is True
        and int(sampling.get("sampled_surface_count", 0)) >= 24
        and int(sampling.get("texture_unique_rgba_color_count", 0)) >= 4
    ),
    "four_layer_sampled_live_correlation_gate": all(
        correlation["passes"] for correlation in layer_correlations
    ),
    "boundary_gate": (
        sampling.get("gpu_upload_claimed") is False
        and sampling.get("android_s5_real_device_claimed") is False
        and sampling.get("live_osm_ingestion_claimed") is False
        and live.get("gpu_upload_claimed") is False
        and live.get("android_s5_real_device_claimed") is False
        and live.get("live_osm_ingestion_claimed") is False
    ),
}

summary = {
    "contract_version": "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1",
    "status": "live_window_sampled_texture_correlation_green",
    "green": all(gates.values()),
    "ready_for_release_review": all(gates.values()),
    "sprite_texture_sampling_contract": "trillionnium_world_bevy_sprite_texture_sampling_v1",
    "live_window_texture_correlation_contract": "trillionnium_world_bevy_live_window_texture_correlation_v1",
    "sampling_summary_path": str(sampling_path),
    "live_correlation_summary_path": str(live_correlation_path),
    "runtime_manifest_sha256": sampling.get("runtime_manifest_sha256"),
    "sampled_surface_count": int(sampling.get("sampled_surface_count", 0)),
    "texture_unique_rgba_color_count": int(sampling.get("texture_unique_rgba_color_count", 0)),
    "live_frame_count": int(live.get("live_frame_count", 0)),
    "live_final_frame_colors_96x54": int(live.get("live_final_frame_colors_96x54", 0)),
    "image_asset_handle_id": live.get("image_asset_handle_id"),
    "texture_atlas_layout_handle_id": live.get("texture_atlas_layout_handle_id"),
    "sampled_layer_count": len(sampled_layer_counts),
    "sampled_layer_counts": sampled_layer_counts,
    "sampled_material_slot_count": len(sampled_slot_counts),
    "sampled_material_slot_counts": sampled_slot_counts,
    "layer_correlation_count": len(layer_correlations),
    "layer_correlations": layer_correlations,
    "gates": gates,
    "asset_boundary": "live_window_pixels_correlated_to_cpu_sampled_bevy_texture_atlas_not_gpu_upload_claim",
    "source_of_truth": "This gate cross-checks CPU-side Bevy Image/TextureAtlasLayout atlas samples against the live-window texture correlation and layer pixel probes. It proves sampled Bevy atlas regions and live-window evidence share the same texture manifest and layer coverage; it does not claim completed GPU upload or Android S5 real-device rendering.",
    "internal_live_window_sampled_texture_correlation_claimed": True,
    "external_evidence_ignored_for_current_sampled_texture_pass": True,
    "gpu_upload_claimed": False,
    "android_s5_real_device_claimed": False,
    "public_launch_ready": False,
    "production_ready_ui_claimed": False,
    "screen_for_screen_openra_ui_claimed": False,
    "openra_engine_port_claimed": False,
    "warcraft_iii_asset_copied": False,
    "openra_asset_copied": False,
    "third_party_asset_copied": False,
    "live_osm_ingestion_claimed": False,
}
summary_path.write_text(json.dumps(summary, indent=2))
PY

jq -e '
	  .contract_version == "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1"
	  and .status == "live_window_sampled_texture_correlation_green"
	  and .green == true
	  and .ready_for_release_review == true
	  and .sprite_texture_sampling_contract == "trillionnium_world_bevy_sprite_texture_sampling_v1"
  and .live_window_texture_correlation_contract == "trillionnium_world_bevy_live_window_texture_correlation_v1"
  and .gates.sprite_texture_sampling_gate == true
  and .gates.live_window_texture_correlation_gate == true
  and .gates.same_image_handle_gate == true
  and .gates.same_texture_atlas_layout_gate == true
  and .gates.same_runtime_manifest_hash_gate == true
  and .gates.sampled_layer_count_gate == true
  and .gates.sampled_material_slot_count_gate == true
  and .gates.sampled_texture_nonblank_gate == true
  and .gates.four_layer_sampled_live_correlation_gate == true
  and .gates.boundary_gate == true
  and .sampled_surface_count >= 24
  and .texture_unique_rgba_color_count >= 4
  and .live_frame_count >= 11
  and .live_final_frame_colors_96x54 >= 1000
	  and .image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
	  and .texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
	  and .sampled_layer_count == (.sampled_layer_counts | keys | length)
	  and .sampled_material_slot_count == (.sampled_material_slot_counts | keys | length)
	  and .layer_correlation_count == (.layer_correlations | length)
	  and .sampled_layer_counts.map >= 1
  and .sampled_layer_counts.hud >= 1
  and .sampled_layer_counts.actor >= 1
  and .sampled_layer_counts.feedback >= 1
  and ([.layer_correlations[] | select(.scene_layer == "map" and .passes == true and .sampled_surface_count >= 1 and .live_pixel_sampled_colors >= 3000)] | length) == 1
  and ([.layer_correlations[] | select(.scene_layer == "hud" and .passes == true and .sampled_surface_count >= 1 and .live_pixel_sampled_colors >= 1000)] | length) == 1
  and ([.layer_correlations[] | select(.scene_layer == "actor" and .passes == true and .sampled_surface_count >= 1 and .live_pixel_sampled_colors >= 3000)] | length) == 1
  and ([.layer_correlations[] | select(.scene_layer == "feedback" and .passes == true and .sampled_surface_count >= 1 and .live_pixel_sampled_colors >= 1000)] | length) == 1
  and .asset_boundary == "live_window_pixels_correlated_to_cpu_sampled_bevy_texture_atlas_not_gpu_upload_claim"
  and .internal_live_window_sampled_texture_correlation_claimed == true
  and .external_evidence_ignored_for_current_sampled_texture_pass == true
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_GREEN $SUMMARY"
