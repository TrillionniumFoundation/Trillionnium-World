#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-art-replication.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-art-replication.ppm"
mkdir -p "$(dirname "$SUMMARY")"
SUMMARY_RAW="$(mktemp "${SUMMARY}.raw.XXXXXX")"
SUMMARY_TMP="$(mktemp "${SUMMARY}.tmp.XXXXXX")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-production-art-replication "$PREVIEW" >"$SUMMARY_RAW"

jq '
  .source_contract_count = (.source_contracts | keys | length)
  | .required_asset_kind_count = (.required_asset_kinds | length)
  | .required_gameplay_layer_count = (.required_gameplay_layers | length)
  | .required_replacement_slot_count = (.required_replacement_slots | length)
  | .gate_count = ([
      .authored_replacement_slot_gate,
      .map_ui_gate,
      .production_preview_gate,
      .production_art_replication_gate,
      .no_copy_boundary_gate,
      .original_art_policy_gate
    ] | length)
  | .passed_gate_count = ([
      .authored_replacement_slot_gate,
      .map_ui_gate,
      .production_preview_gate,
      .production_art_replication_gate,
      .no_copy_boundary_gate,
      .original_art_policy_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_production_art_replication_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .source_contract_count == (.source_contracts | keys | length)
  and .source_contracts.authored_art_pack == "trillionnium_world_bevy_authored_art_pack_v1"
  and .source_contracts.map_ui_modeling_readiness == "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1"
  and .source_contracts.visual_fidelity == "trillionnium_world_bevy_classic_rts_visual_fidelity_v1"
  and .authored_surface_count >= 120
  and .authored_export_ready_count == .authored_surface_count
  and .authored_min_target_resolution_px >= 32
  and .required_asset_kind_count == (.required_asset_kinds | length)
  and (.required_asset_kinds | index("terrain_tile") != null)
  and (.required_asset_kinds | index("road_tile") != null)
  and (.required_asset_kinds | index("building_tile") != null)
  and (.required_asset_kinds | index("foliage_sprite") != null)
  and (.required_asset_kinds | index("water_tile") != null)
  and (.required_asset_kinds | index("hud_icon") != null)
  and (.required_asset_kinds | index("hud_glyph") != null)
  and (.required_asset_kinds | index("actor_sprite") != null)
  and (.required_asset_kinds | index("feedback_glyph") != null)
  and .required_gameplay_layer_count == (.required_gameplay_layers | length)
  and .required_replacement_slot_count == (.required_replacement_slots | length)
  and (.required_replacement_slots | index("tile_sprite_slot") != null)
  and (.required_replacement_slots | index("hud_icon_slot") != null)
  and (.required_replacement_slots | index("hud_glyph_slot") != null)
  and (.required_replacement_slots | index("actor_sprite_slot") != null)
  and (.required_replacement_slots | index("feedback_glyph_slot") != null)
  and .production_family_count == 9
  and .first_contact_production_art_pack_id == "first_contact_production_art_pack_v3"
  and .first_contact_production_pack_family_count == 6
  and .first_contact_production_pack_v2_feature_count == 5
  and .first_contact_production_pack_v3_feature_count == 6
  and .production_board_pixel_count > 80000
  and .actor_silhouette_pixel_count > 5000
  and .building_material_pixel_count > 3000
  and .tileset_variation_pixel_count > 5000
  and .hud_chrome_pixel_count > 6000
  and .material_swatch_pixel_count > 1000
  and .replacement_slot_pixel_count > 2000
  and .first_contact_production_art_pack_pixel_counts.terrain_material > 600
  and .first_contact_production_art_pack_pixel_counts.edge_blend > 600
  and .first_contact_production_art_pack_pixel_counts.unit_sprite_skin > 600
  and .first_contact_production_art_pack_pixel_counts.structure_sprite_skin > 600
  and .first_contact_production_art_pack_pixel_counts.hud_skin > 600
  and .first_contact_production_art_pack_pixel_counts.action_flow > 600
  and .first_contact_production_art_pack_v2_pixel_counts.terrain_texture > 100
  and .first_contact_production_art_pack_v2_pixel_counts.transition_tile > 100
  and .first_contact_production_art_pack_v2_pixel_counts.unit_silhouette > 100
  and .first_contact_production_art_pack_v2_pixel_counts.structure_facade > 100
  and .first_contact_production_art_pack_v2_pixel_counts.action_stroke > 100
  and .first_contact_production_art_pack_v2_gate == true
  and .first_contact_production_art_pack_v3_pixel_counts.terrain_cluster > 100
  and .first_contact_production_art_pack_v3_pixel_counts.transition_corner > 100
  and .first_contact_production_art_pack_v3_pixel_counts.unit_role_block > 100
  and .first_contact_production_art_pack_v3_pixel_counts.structure_roof_depth > 100
  and .first_contact_production_art_pack_v3_pixel_counts.action_priority_ribbon > 100
  and .first_contact_production_art_pack_v3_pixel_counts.hud_command_plate > 100
  and .first_contact_production_art_pack_v3_gate == true
  and .authored_replacement_slot_gate == true
  and .map_ui_gate == true
  and .production_preview_gate == true
  and .first_contact_production_art_pack_gate == true
  and .production_art_replication_gate == true
  and .no_copy_boundary_gate == true
  and .original_art_policy_gate == true
  and .gate_count == 6
  and .passed_gate_count == 6
  and .failed_gate_count == 0
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .final_external_bitmap_art_shipped == false
  and .production_ready_art_shipped == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_ART_REPLICATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
