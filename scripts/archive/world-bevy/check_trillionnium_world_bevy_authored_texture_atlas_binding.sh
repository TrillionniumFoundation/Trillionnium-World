#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-texture-atlas-binding.json"
ATLAS="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet.ppm"
MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet-manifest.json"
BINDING="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-texture-atlas-binding-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" authored-texture-atlas-binding "$ATLAS" "$MANIFEST" "$BINDING" >"$SUMMARY"
)

test -s "$ATLAS"
test -s "$MANIFEST"
test -s "$BINDING"
head -n 1 "$ATLAS" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_texture_atlas_binding_v1"
  and .green == true
  and .authored_sprite_sheet_contract == "trillionnium_world_bevy_authored_sprite_sheet_artifact_v1"
  and .binding_count >= 32
  and .binding_bytes > 8192
  and .atlas_source_gate == true
  and .binding_count_gate == true
  and .uv_rect_gate == true
  and .runtime_target_gate == true
  and .material_slot_gate == true
  and .replacement_slot_gate == true
  and .source_boundary_gate == true
  and .binding_write_gate == true
  and .binding_parse_gate == true
  and (.runtime_targets | index("map_tile_renderer") != null)
  and (.runtime_targets | index("hud_renderer") != null)
  and (.runtime_targets | index("actor_renderer") != null)
  and (.runtime_targets | index("feedback_renderer") != null)
  and (.material_slots | index("world_tile_material") != null)
  and (.material_slots | index("hud_icon_material") != null)
  and (.material_slots | index("actor_sprite_material") != null)
  and (.material_slots | index("feedback_glyph_material") != null)
  and (.replacement_slots | index("tile_sprite_slot") != null)
  and (.replacement_slots | index("hud_icon_slot") != null)
  and (.replacement_slots | index("hud_glyph_slot") != null)
  and (.replacement_slots | index("actor_sprite_slot") != null)
  and (.replacement_slots | index("actor_shadow_slot") != null)
  and (.replacement_slots | index("actor_badge_slot") != null)
  and (.replacement_slots | index("feedback_glyph_slot") != null)
  and (.sampler_modes | index("nearest_pixel_art") != null)
  and (.source_origins | index("local_authored_primitive_manifest_v1") != null)
  and (.license_scopes | index("project_owned_internal_placeholder") != null)
  and .asset_boundary == "runtime_texture_atlas_binding_for_generated_local_ppm_not_final_external_art"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_texture_atlas_binding_v1"
  and .atlas_format == "ppm_p3_rgb"
  and .texture_handle_id == "runtime_texture_handle::trnm_world_authored_sprite_sheet_v1"
  and .binding_count >= 32
  and (.bindings | length) >= 32
  and (.bindings[0].uv_rect.u0 == 0)
  and (.bindings[0].uv_rect.v0 == 0)
  and (.bindings[0].uv_rect.u1 > 0)
  and (.bindings[0].uv_rect.v1 > 0)
  and .asset_boundary == "runtime_texture_atlas_binding_for_generated_local_ppm_not_final_external_art"
  and .android_s5_real_device_claimed == false
' "$BINDING" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_AUTHORED_TEXTURE_ATLAS_BINDING_GREEN $SUMMARY $BINDING"
