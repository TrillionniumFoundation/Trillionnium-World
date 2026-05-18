#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json"
CATALOG="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.ppm"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    cargo run -p trnm-world-bevy -- classic-model-catalog "$CATALOG" >"$SUMMARY"
)

test -s "$SUMMARY"
test -s "$CATALOG"
head -n 1 "$CATALOG" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_model_catalog_v1"
  and .green == true
  and .catalog_format == "ppm_p3_rgb"
  and .catalog_width == 640
  and .catalog_height >= 1056
  and .catalog_bytes > 100000
  and .cell_width == 160
  and .cell_height == 96
  and .columns == 4
  and .frame_count >= 43
  and .rendered_frame_count == .frame_count
  and .unique_color_count >= 32
  and .non_background_pixels > 40000
  and .label_pixel_count > 2500
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .catalog_sheet_gate == true
  and .label_gate == true
  and .all_frames_rendered_gate == true
  and .player_direction_catalog_gate == true
  and .actor_clip_catalog_gate == true
  and .scene_reference_catalog_gate == true
  and .role_coverage_gate == true
  and .role_counts.player_actor >= 12
  and .role_counts.npc_actor >= 3
  and .role_counts.enemy_actor >= 4
  and .role_counts.scene_prop >= 8
  and ([.frame_summaries[].id] | index("actor_player_idle_south") != null)
  and ([.frame_summaries[].id] | index("actor_player_idle_north") != null)
  and ([.frame_summaries[].id] | index("actor_player_idle_east") != null)
  and ([.frame_summaries[].id] | index("actor_player_idle_west") != null)
  and ([.frame_summaries[].id] | index("actor_player_walk_south_1") != null)
  and ([.frame_summaries[].id] | index("actor_player_walk_north_1") != null)
  and ([.frame_summaries[].id] | index("actor_player_walk_east_1") != null)
  and ([.frame_summaries[].id] | index("actor_player_walk_west_1") != null)
  and ([.frame_summaries[].id] | index("actor_mentor_talk") != null)
  and ([.frame_summaries[].id] | index("actor_enemy_attack") != null)
  and ([.frame_summaries[].id] | index("prop_training_dummy") != null)
  and ([.frame_summaries[].id] | index("marker_objective") != null)
  and ([.frame_summaries[] | select(.id == "actor_player_idle_south") | .visible_pixel_count] | first) > 12
  and ([.frame_summaries[] | select(.id == "prop_reward") | .visible_pixel_count] | first) > 12
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_MODEL_CATALOG_GREEN %s %s\n' "$SUMMARY" "$CATALOG"
