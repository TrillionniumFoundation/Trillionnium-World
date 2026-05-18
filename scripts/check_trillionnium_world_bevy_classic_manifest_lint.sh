#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    cargo run -p trnm-world-bevy -- classic-manifest-lint >"$SUMMARY"
)

test -s "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_manifest_lint_v1"
  and .green == true
  and .frame_count >= 43
  and .scene_count >= 3
  and .actor_count >= 3
  and (.duplicate_frame_ids | length) == 0
  and (.out_of_bounds_frame_ids | length) == 0
  and .frame_overlap_detected == false
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .catalog_ready_gate == true
  and .frame_id_naming_gate == true
  and .frame_role_alignment_gate == true
  and .frame_rect_gate == true
  and .source_tile_size_gate == true
  and .required_role_family_gate == true
  and .actor_reference_gate == true
  and .player_direction_gate == true
  and .mentor_enemy_clip_gate == true
  and .scene_shape_gate == true
  and .scene_palette_gate == true
  and .scene_landmark_gate == true
  and .opaque_tile_gate == true
  and .transparent_overlay_gate == true
  and .boundary_gate == true
  and .role_counts.player_actor >= 12
  and .role_counts.npc_actor >= 3
  and .role_counts.enemy_actor >= 4
  and .role_counts.scene_prop >= 8
  and .x230_low_spec_renderer_target == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and (.asset_boundary | contains("not_cex_runtime"))
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_MANIFEST_LINT_GREEN %s\n' "$SUMMARY"
