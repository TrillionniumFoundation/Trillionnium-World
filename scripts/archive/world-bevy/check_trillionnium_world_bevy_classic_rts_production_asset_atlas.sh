#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-asset-atlas.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-asset-atlas.ppm"
mkdir -p "$(dirname "$SUMMARY")"
SUMMARY_RAW="$(mktemp "${SUMMARY}.raw.XXXXXX")"
SUMMARY_TMP="$(mktemp "${SUMMARY}.tmp.XXXXXX")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-production-asset-atlas "$PREVIEW" >"$SUMMARY_RAW"

jq '
  .source_contract_count = (.source_contracts | keys | length)
  | .source_path_count = (.source_paths | keys | length)
  | .atlas_family_name_count = (.atlas_family_names | length)
  | .binding_replacement_slot_count = (.binding_replacement_slots | length)
  | .binding_runtime_target_count = (.binding_runtime_targets | length)
  | .runtime_material_slot_count = (.runtime_material_slots | length)
  | .runtime_scene_layer_count = (.runtime_scene_layers | length)
  | .gate_count = ([
      .production_art_replication_gate,
      .sprite_sheet_gate,
      .texture_atlas_binding_gate,
      .runtime_texture_asset_gate,
      .production_asset_atlas_preview_gate,
      .production_asset_atlas_gate,
      .no_copy_boundary_gate,
      .original_art_policy_gate
    ] | length)
  | .passed_gate_count = ([
      .production_art_replication_gate,
      .sprite_sheet_gate,
      .texture_atlas_binding_gate,
      .runtime_texture_asset_gate,
      .production_asset_atlas_preview_gate,
      .production_asset_atlas_gate,
      .no_copy_boundary_gate,
      .original_art_policy_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_production_asset_atlas_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .source_contract_count == (.source_contracts | keys | length)
  and .source_path_count == (.source_paths | keys | length)
  and .source_contracts.production_art_replication == "trillionnium_world_bevy_classic_rts_production_art_replication_v1"
  and .source_contracts.authored_sprite_sheet == "trillionnium_world_bevy_authored_sprite_sheet_artifact_v1"
  and .source_contracts.authored_texture_atlas_binding == "trillionnium_world_bevy_authored_texture_atlas_binding_v1"
  and .source_contracts.runtime_texture_asset == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .production_family_count >= 9
  and .authored_surface_count >= 120
  and .authored_export_ready_count == .authored_surface_count
  and .atlas_family_count == 10
  and .atlas_family_name_count == (.atlas_family_names | length)
  and (.atlas_family_names | index("TERRAIN TILES") != null)
  and (.atlas_family_names | index("BUILDINGS") != null)
  and (.atlas_family_names | index("PLAYER UNITS") != null)
  and (.atlas_family_names | index("HUD ICONS") != null)
  and (.atlas_family_names | index("FEEDBACK VFX") != null)
  and .first_contact_production_art_pack_id == "first_contact_production_art_pack_v5"
  and .first_contact_pack_atlas_slot_count == 29
  and .first_contact_pack_v2_atlas_slot_count == 5
  and .first_contact_pack_v3_atlas_slot_count == 6
  and .first_contact_pack_v4_atlas_slot_count == 6
  and .first_contact_pack_v5_atlas_slot_count == 6
  and (.first_contact_pack_atlas_slot_names | index("terrain_material") != null)
  and (.first_contact_pack_atlas_slot_names | index("edge_blend") != null)
  and (.first_contact_pack_atlas_slot_names | index("unit_sprite_skin") != null)
  and (.first_contact_pack_atlas_slot_names | index("structure_sprite_skin") != null)
  and (.first_contact_pack_atlas_slot_names | index("hud_skin") != null)
  and (.first_contact_pack_atlas_slot_names | index("action_flow") != null)
  and (.first_contact_pack_atlas_slot_names | index("terrain_texture") != null)
  and (.first_contact_pack_atlas_slot_names | index("transition_tile") != null)
  and (.first_contact_pack_atlas_slot_names | index("unit_silhouette") != null)
  and (.first_contact_pack_atlas_slot_names | index("structure_facade") != null)
  and (.first_contact_pack_atlas_slot_names | index("action_stroke") != null)
  and (.first_contact_pack_atlas_slot_names | index("terrain_cluster") != null)
  and (.first_contact_pack_atlas_slot_names | index("transition_corner") != null)
  and (.first_contact_pack_atlas_slot_names | index("unit_role_block") != null)
  and (.first_contact_pack_atlas_slot_names | index("structure_roof_depth") != null)
  and (.first_contact_pack_atlas_slot_names | index("action_priority_ribbon") != null)
  and (.first_contact_pack_atlas_slot_names | index("hud_command_plate") != null)
  and (.first_contact_pack_atlas_slot_names | index("terrain_strata") != null)
  and (.first_contact_pack_atlas_slot_names | index("edge_occlusion") != null)
  and (.first_contact_pack_atlas_slot_names | index("unit_readability_crest") != null)
  and (.first_contact_pack_atlas_slot_names | index("structure_volume_shadow") != null)
  and (.first_contact_pack_atlas_slot_names | index("action_intent_arrow") != null)
  and (.first_contact_pack_atlas_slot_names | index("hud_focus_strip") != null)
  and (.first_contact_pack_atlas_slot_names | index("terrain_micro_detail") != null)
  and (.first_contact_pack_atlas_slot_names | index("edge_bevel") != null)
  and (.first_contact_pack_atlas_slot_names | index("unit_stance_shadow") != null)
  and (.first_contact_pack_atlas_slot_names | index("structure_light_rim") != null)
  and (.first_contact_pack_atlas_slot_names | index("action_target_chevron") != null)
  and (.first_contact_pack_atlas_slot_names | index("hud_command_highlight") != null)
  and .atlas_frame_count >= 32
  and .sprite_binding_count >= 32
  and .material_asset_count == 4
  and .atlas_bytes > 50000
  and .runtime_asset_bytes > 8192
  and .runtime_scene_layer_count == (.runtime_scene_layers | length)
  and (.runtime_scene_layers | index("map") != null)
  and (.runtime_scene_layers | index("hud") != null)
  and (.runtime_scene_layers | index("actor") != null)
  and (.runtime_scene_layers | index("feedback") != null)
  and .runtime_material_slot_count == (.runtime_material_slots | length)
  and (.runtime_material_slots | index("world_tile_material") != null)
  and (.runtime_material_slots | index("hud_icon_material") != null)
  and (.runtime_material_slots | index("actor_sprite_material") != null)
  and (.runtime_material_slots | index("feedback_glyph_material") != null)
  and .binding_runtime_target_count == (.binding_runtime_targets | length)
  and (.binding_runtime_targets | index("map_tile_renderer") != null)
  and (.binding_runtime_targets | index("hud_renderer") != null)
  and (.binding_runtime_targets | index("actor_renderer") != null)
  and (.binding_runtime_targets | index("feedback_renderer") != null)
  and .binding_replacement_slot_count == (.binding_replacement_slots | length)
  and (.binding_replacement_slots | index("tile_sprite_slot") != null)
  and (.binding_replacement_slots | index("actor_sprite_slot") != null)
  and (.binding_replacement_slots | index("hud_icon_slot") != null)
  and (.binding_replacement_slots | index("feedback_glyph_slot") != null)
  and .atlas_board_pixel_count > 80000
  and .terrain_tile_pixel_count > 1500
  and .road_tile_pixel_count > 1500
  and .water_tile_pixel_count > 1500
  and .foliage_sprite_pixel_count > 1500
  and .building_sprite_pixel_count > 1500
  and .player_unit_sprite_pixel_count > 1500
  and .enemy_unit_sprite_pixel_count > 1500
  and .neutral_unit_sprite_pixel_count > 1500
  and .hud_icon_pixel_count > 2000
  and .feedback_vfx_pixel_count > 1500
  and .runtime_binding_lane_pixel_count > 8000
  and .uv_rect_pixel_count > 6000
  and .first_contact_pack_atlas_slot_pixel_count > 12000
  and .first_contact_pack_atlas_uv_pixel_count > 6000
  and .production_art_replication_gate == true
  and .sprite_sheet_gate == true
  and .texture_atlas_binding_gate == true
  and .runtime_texture_asset_gate == true
  and .production_asset_atlas_preview_gate == true
  and .first_contact_production_art_pack_atlas_gate == true
  and .production_asset_atlas_gate == true
  and .no_copy_boundary_gate == true
  and .original_art_policy_gate == true
  and .gate_count == 8
  and .passed_gate_count == 8
  and .failed_gate_count == 0
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .final_external_bitmap_art_shipped == false
  and .production_ready_art_shipped == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .gpu_upload_claimed == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_ASSET_ATLAS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
