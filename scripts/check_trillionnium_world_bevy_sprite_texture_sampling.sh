#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.json"
SUMMARY_RAW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.raw.json"
RUNTIME_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset.json"
RUNTIME_MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_sprite_asset_binding.sh" >/dev/null
test -s "$RUNTIME_SUMMARY"
test -s "$RUNTIME_MANIFEST"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" sprite-texture-sampling "$RUNTIME_SUMMARY" "$RUNTIME_MANIFEST" >"$SUMMARY_RAW"
)

jq '
  .status = "sprite_texture_sampling_green"
  | .external_evidence_ignored_for_current_sprite_texture_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY"
rm -f "$SUMMARY_RAW"

jq -e '
  .contract_version == "trillionnium_world_bevy_sprite_texture_sampling_v1"
  and .status == "sprite_texture_sampling_green"
  and .green == true
  and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .runtime_texture_manifest_probe_contract == "trillionnium_world_bevy_runtime_texture_manifest_probe_v1"
  and .asset_store_registration_contract == "trillionnium_world_bevy_asset_store_registration_v1"
  and .sprite_asset_binding_contract == "trillionnium_world_bevy_sprite_asset_binding_v1"
  and .runtime_summary_gate == true
  and .asset_store_registration_gate == true
  and .sprite_asset_binding_gate == true
  and .image_asset_resolve_gate == true
  and .texture_atlas_layout_asset_resolve_gate == true
  and .texture_atlas_rect_resolve_gate == true
  and .texture_sample_nonblank_gate == true
  and .four_layer_texture_sampling_gate == true
  and .global_unique_texture_color_gate == true
  and .boundary_gate == true
  and (.runtime_manifest_sha256 | length) == 64
  and .asset_store_registration.green == true
  and .asset_store_registration.asset_store_registered_gate == true
  and .asset_store_registration.bevy_image_store_registration_gate == true
  and .asset_store_registration.texture_atlas_layout_store_registration_gate == true
  and .sprite_binding_lookup.green == true
  and .sprite_binding_lookup.binding_count >= 32
  and .sampled_surface_count >= 24
  and .texture_unique_rgba_color_count >= 4
  and (.scene_layers | index("map"))
  and (.scene_layers | index("hud"))
  and (.scene_layers | index("actor"))
  and (.scene_layers | index("feedback"))
  and (.material_slots | index("world_tile_material"))
  and (.material_slots | index("hud_icon_material"))
  and (.material_slots | index("actor_sprite_material"))
  and (.material_slots | index("feedback_glyph_material"))
  and .sampled_layer_counts.map >= 1
  and .sampled_layer_counts.hud >= 1
  and .sampled_layer_counts.actor >= 1
  and .sampled_layer_counts.feedback >= 1
  and .sampled_material_slot_counts.world_tile_material >= 1
  and .sampled_material_slot_counts.hud_icon_material >= 1
  and .sampled_material_slot_counts.actor_sprite_material >= 1
  and .sampled_material_slot_counts.feedback_glyph_material >= 1
  and (.sampled_surfaces_sample | length) >= 8
  and .sampled_surfaces_sample[0].image_asset_resolve_gate == true
  and .sampled_surfaces_sample[0].texture_atlas_layout_asset_resolve_gate == true
  and .sampled_surfaces_sample[0].texture_atlas_rect_resolve_gate == true
  and .sampled_surfaces_sample[0].texture_sample_nonblank_gate == true
  and .sampled_surfaces_sample[0].sample_count >= 5
  and .sampled_surfaces_sample[0].alpha_nonzero_sample_count >= 5
  and (.sampled_surfaces_sample[0].texture_rect.width == 32)
  and (.sampled_surfaces_sample[0].texture_rect.height == 32)
  and (.sampled_surfaces_sample[0].sprite_image_asset_id_debug | contains("index"))
  and (.sampled_surfaces_sample[0].sprite_texture_atlas_layout_asset_id_debug | contains("index"))
  and (.host_log_line | contains("TRNM_WORLD_BEVY_SPRITE_TEXTURE_SAMPLING"))
  and .asset_boundary == "bevy_assets_image_texture_atlas_cpu_sampling_not_gpu_upload_claim"
  and .host_side_cpu_texture_sampling_claimed == true
  and .external_evidence_ignored_for_current_sprite_texture_pass == true
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

echo "TRILLIONNIUM_WORLD_BEVY_SPRITE_TEXTURE_SAMPLING_GREEN $SUMMARY"
