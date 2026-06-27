#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json"
SUMMARY_RAW="$SUMMARY.raw.$$"
SUMMARY_TMP="$SUMMARY.tmp.$$"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-manifest-lint >"$SUMMARY_RAW"
)

jq '
  .status = "classic_manifest_lint_green"
  | .ready_for_release_review = true
  | .gate_count = 16
  | .passed_gate_count = ([
      .atlas_parse_gate,
      .source_tile_size_gate,
      .frame_rect_gate,
      .frame_id_naming_gate,
      .frame_role_alignment_gate,
      .required_role_family_gate,
      .actor_reference_gate,
      .player_direction_gate,
      .mentor_enemy_clip_gate,
      .scene_shape_gate,
      .scene_palette_gate,
      .scene_landmark_gate,
      .opaque_tile_gate,
      .transparent_overlay_gate,
      .catalog_ready_gate,
      .boundary_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
  | .role_family_count = (.role_counts | keys | length)
  | .duplicate_frame_id_count = (.duplicate_frame_ids | length)
  | .out_of_bounds_frame_id_count = (.out_of_bounds_frame_ids | length)
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

test -s "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_manifest_lint_v1"
  and .status == "classic_manifest_lint_green"
  and .green == true
  and .ready_for_release_review == true
  and .gate_count == 16
  and .passed_gate_count == 16
  and .failed_gate_count == 0
  and .role_family_count == (.role_counts | keys | length)
  and .duplicate_frame_id_count == (.duplicate_frame_ids | length)
  and .out_of_bounds_frame_id_count == (.out_of_bounds_frame_ids | length)
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
