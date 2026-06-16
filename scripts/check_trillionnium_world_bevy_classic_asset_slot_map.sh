#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-asset-slot-map.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-asset-slot-map >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_asset_slot_map_v1"
  and .green == true
  and .slot_count >= 72
  and .category_count >= 8
  and .category_counts.terrain >= 10
  and .category_counts.terrain_detail >= 4
  and .category_counts.unit >= 24
  and .category_counts.prop >= 8
  and .category_counts.marker >= 2
  and .category_counts.building >= 5
  and .category_counts.doodad >= 8
  and .category_counts.vfx_ui >= 6
  and .manifest_frame_slot_count >= 43
  and .procedural_model_slot_count >= 5
  and .doodad_slot_count >= 8
  and .terrain_detail_slot_count >= 4
  and .vfx_slot_count >= 6
  and .neutral_unit_slot_count >= 6
  and .required_categories_present_gate == true
  and .manifest_frame_slots_gate == true
  and .procedural_slots_gate == true
  and .replacement_boundary_gate == true
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .x230_low_spec_renderer_target == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and (.asset_boundary | contains("not_cex_runtime"))
  and (.future_real_asset_contract | contains("trnm-world-bevy"))
  and ([.slots[] | select(.category == "building" and .target_id == "model_town_hall")] | length) == 1
  and ([.slots[] | select(.category == "doodad" and .target_id == "doodad_torch")] | length) == 1
  and ([.slots[] | select(.category == "doodad" and .target_id == "doodad_gold_vein")] | length) == 1
  and ([.slots[] | select(.category == "terrain_detail" and .target_id == "tile_bridge")] | length) == 1
  and ([.slots[] | select(.category == "vfx_ui" and .target_id == "combat_attack_arc")] | length) == 1
  and ([.slots[] | select(.category == "unit" and .target_id == "actor_guard_idle" and .backing_kind == "procedural_neutral_unit")] | length) == 1
  and ([.slots[] | select(.category == "unit" and .target_id == "actor_worker_carry" and .backing_kind == "procedural_neutral_unit")] | length) == 1
  and ([.slots[] | select(.category == "unit" and .target_id == "actor_creep_attack" and .backing_kind == "procedural_neutral_unit")] | length) == 1
  and ([.slots[] | select(.backing_kind == "manifest_frame" and .target_id == "actor_player_idle_south")] | length) == 1
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_SLOT_MAP_GREEN %s\n' "$SUMMARY"
