#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-asset-pack.json"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
ATLAS="$ROOT/assets/trnm-world/classic/atlas.ppm"
mkdir -p "$(dirname "$SUMMARY")" "$(dirname "$MANIFEST")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- classic-asset-pack "$MANIFEST" "$ATLAS" >"$SUMMARY"
)

test -s "$SUMMARY"
test -s "$MANIFEST"
test -s "$ATLAS"
head -n 1 "$ATLAS" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_asset_pack_v1"
  and .green == true
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .frame_count >= 32
  and .scene_count >= 3
  and .actor_count >= 3
  and .frame_gate == true
  and .scene_gate == true
  and .actor_gate == true
  and .scene_tile_gate == true
  and .scene_landmark_gate == true
  and .directional_player_frame_gate == true
  and .animation_clip_gate == true
  and .x230_low_spec_renderer_target == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and .renderer_uses_manifest == true
  and .asset_boundary == "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime"
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_asset_pack_v1"
  and .atlas_path == "atlas.ppm"
  and .atlas_format == "ppm_p3_rgb"
  and .source_tile_size_px == 16
  and .render_tile_size_px == 32
  and (.frames | length) >= 32
  and (.scenes | length) >= 3
  and (.actors | length) >= 3
  and ([.frames[].id] | index("actor_player_idle_south") != null)
  and ([.frames[].id] | index("actor_player_idle_north") != null)
  and ([.frames[].id] | index("actor_player_idle_east") != null)
  and ([.frames[].id] | index("actor_player_idle_west") != null)
  and ([.frames[].id] | index("actor_player_walk_1") != null)
  and ([.frames[].id] | index("actor_player_walk_south_1") != null)
  and ([.frames[].id] | index("actor_player_walk_south_2") != null)
  and ([.frames[].id] | index("actor_player_walk_north_1") != null)
  and ([.frames[].id] | index("actor_player_walk_north_2") != null)
  and ([.frames[].id] | index("actor_player_walk_east_1") != null)
  and ([.frames[].id] | index("actor_player_walk_east_2") != null)
  and ([.frames[].id] | index("actor_player_walk_west_1") != null)
  and ([.frames[].id] | index("actor_player_walk_west_2") != null)
  and ([.frames[].id] | index("actor_mentor") != null)
  and ([.frames[].id] | index("actor_enemy") != null)
  and ([.frames[].id] | index("prop_signpost") != null)
  and ([.frames[].id] | index("prop_workbench") != null)
  and ([.frames[].id] | index("prop_arena_gate") != null)
  and ([.scenes[].id] | index("mirror_city_square") != null)
  and ([.scenes[].id] | index("mentor_training_room") != null)
  and ([.scenes[].id] | index("league_coliseum") != null)
  and ((.actors[] | select(.id == "player") | .facing_frames | length) >= 4)
  and ((.actors[] | select(.id == "player") | [.facing_frames[].direction] | index("south")) != null)
  and ((.actors[] | select(.id == "player") | [.facing_frames[].direction] | index("north")) != null)
  and ((.actors[] | select(.id == "player") | [.facing_frames[].direction] | index("east")) != null)
  and ((.actors[] | select(.id == "player") | [.facing_frames[].direction] | index("west")) != null)
  and ((.actors[] | select(.id == "player") | [.animation_clips[].action] | index("walk")) != null)
  and ((.actors[] | select(.id == "mentor") | [.animation_clips[].action] | index("talk")) != null)
  and ((.actors[] | select(.id == "enemy") | [.animation_clips[].action] | index("attack")) != null)
  and .x230_low_spec_renderer_target == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$MANIFEST" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_PACK_GREEN %s %s %s\n' "$SUMMARY" "$MANIFEST" "$ATLAS"
