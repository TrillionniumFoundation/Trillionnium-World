#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet.json"
ATLAS="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet.ppm"
MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- authored-sprite-sheet "$ATLAS" "$MANIFEST" >"$SUMMARY"
)

test -s "$ATLAS"
test -s "$MANIFEST"
head -n 1 "$ATLAS" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_sprite_sheet_artifact_v1"
  and .green == true
  and .authored_art_pack_contract == "trillionnium_world_bevy_authored_art_pack_v1"
  and .atlas_format == "ppm_p3_rgb"
  and .atlas_bytes > 50000
  and .manifest_bytes > 2048
  and .source_surface_count >= 120
  and .frame_count >= 32
  and .frame_count_gate == true
  and .frame_asset_kind_gate == true
  and .frame_layer_gate == true
  and .frame_slot_gate == true
  and .source_boundary_gate == true
  and .atlas_write_gate == true
  and .manifest_write_gate == true
  and .manifest_parse_gate == true
  and (.frame_asset_kinds | index("terrain_tile") != null)
  and (.frame_asset_kinds | index("road_tile") != null)
  and (.frame_asset_kinds | index("building_tile") != null)
  and (.frame_asset_kinds | index("foliage_sprite") != null)
  and (.frame_asset_kinds | index("water_tile") != null)
  and (.frame_asset_kinds | index("hud_icon") != null)
  and (.frame_asset_kinds | index("hud_glyph") != null)
  and (.frame_asset_kinds | index("actor_sprite") != null)
  and (.frame_asset_kinds | index("feedback_glyph") != null)
  and (.frame_gameplay_layers | index("terrain") != null)
  and (.frame_gameplay_layers | index("road") != null)
  and (.frame_gameplay_layers | index("building") != null)
  and (.frame_gameplay_layers | index("greenery") != null)
  and (.frame_gameplay_layers | index("water") != null)
  and (.frame_gameplay_layers | index("hud") != null)
  and (.frame_gameplay_layers | index("actor") != null)
  and (.frame_gameplay_layers | index("feedback") != null)
  and (.frame_replacement_slots | index("tile_sprite_slot") != null)
  and (.frame_replacement_slots | index("hud_icon_slot") != null)
  and (.frame_replacement_slots | index("hud_glyph_slot") != null)
  and (.frame_replacement_slots | index("actor_sprite_slot") != null)
  and (.frame_replacement_slots | index("actor_shadow_slot") != null)
  and (.frame_replacement_slots | index("actor_badge_slot") != null)
  and (.frame_replacement_slots | index("feedback_glyph_slot") != null)
  and (.frame_source_origins | index("local_authored_primitive_manifest_v1") != null)
  and (.frame_license_scopes | index("project_owned_internal_placeholder") != null)
  and .asset_boundary == "generated_project_owned_local_ppm_atlas_not_final_external_bitmap_art"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_sprite_sheet_artifact_v1"
  and .atlas_format == "ppm_p3_rgb"
  and .frame_count >= 32
  and (.frames | length) >= 32
  and (.frames[0].x == 0)
  and (.frames[0].y == 0)
  and (.frames[0].w == 32)
  and (.frames[0].h == 32)
  and .asset_boundary == "generated_project_owned_local_ppm_atlas_not_final_external_bitmap_art"
  and .android_s5_real_device_claimed == false
' "$MANIFEST" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_AUTHORED_SPRITE_SHEET_GREEN $SUMMARY $ATLAS $MANIFEST"
