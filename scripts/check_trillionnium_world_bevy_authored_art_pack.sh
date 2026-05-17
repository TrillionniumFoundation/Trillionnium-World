#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-art-pack.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- authored-art-pack >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_art_pack_v1"
  and .green == true
  and .authored_art_pack_gate == true
  and .tileset_polish_gate == true
  and .map_model_visual_gate == true
  and (.authored_art_pack_policy.surface_count >= 120)
  and (.authored_art_pack_policy.asset_pack_ids | index("trnm_world_authored_art_pack_v1") != null)
  and (.authored_art_pack_policy.asset_kinds | index("terrain_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("road_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("building_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("foliage_sprite") != null)
  and (.authored_art_pack_policy.asset_kinds | index("water_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("hud_icon") != null)
  and (.authored_art_pack_policy.asset_kinds | index("hud_glyph") != null)
  and (.authored_art_pack_policy.asset_kinds | index("actor_sprite") != null)
  and (.authored_art_pack_policy.asset_kinds | index("feedback_glyph") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("terrain") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("road") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("building") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("greenery") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("water") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("hud") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("actor") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("feedback") != null)
  and (.authored_art_pack_policy.replacement_slots | index("tile_sprite_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("hud_icon_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("hud_glyph_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("actor_sprite_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("feedback_glyph_slot") != null)
  and (.authored_art_pack_policy.source_origins | index("local_authored_primitive_manifest_v1") != null)
  and (.authored_art_pack_policy.license_scopes | index("project_owned_internal_placeholder") != null)
  and (.authored_art_pack_policy.min_target_resolution_px >= 32)
  and (.authored_art_pack_policy.export_ready_count == .authored_art_pack_policy.surface_count)
  and .asset_boundary == "project_owned_internal_placeholder_manifest_not_external_bitmap_ship_claim"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_AUTHORED_ART_PACK_GREEN $SUMMARY"
