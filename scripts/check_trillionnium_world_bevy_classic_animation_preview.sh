#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.json"
SUMMARY_RAW="$SUMMARY.raw"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.ppm"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-animation-preview "$PREVIEW" >"$SUMMARY_RAW"
)

jq '
  .status = "classic_animation_preview_green"
  | .android_s5_real_device_claimed = false
  | .external_evidence_ignored_for_current_animation_preview_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY"
rm -f "$SUMMARY_RAW"

test -s "$SUMMARY"
test -s "$PREVIEW"
head -n 1 "$PREVIEW" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_animation_preview_v1"
  and .status == "classic_animation_preview_green"
  and .green == true
  and .preview_format == "ppm_p3_rgb"
  and .preview_width == 640
  and .preview_height >= 448
  and .preview_bytes > 100000
  and .clip_count >= 4
  and .rendered_clip_count == .clip_count
  and .rendered_frame_slot_count >= 15
  and .unique_color_count >= 32
  and .non_background_pixels > 35000
  and .label_pixel_count > 2000
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .clip_count_gate == true
  and .action_coverage_gate == true
  and .fps_gate == true
  and .all_clip_refs_valid == true
  and .rendered_clip_gate == true
  and .preview_sheet_gate == true
  and .label_gate == true
  and ([.clip_summaries[].action] | index("walk") != null)
  and ([.clip_summaries[].action] | index("talk") != null)
  and ([.clip_summaries[].action] | index("attack") != null)
  and ([.clip_summaries[].action] | index("hit") != null)
  and ([.clip_summaries[] | select(.actor_id == "player" and .action == "walk") | .frame_count] | first) >= 8
  and ([.clip_summaries[] | select(.actor_id == "player" and .action == "walk") | .frame_ids[]] | index("actor_player_walk_south_1") != null)
  and ([.clip_summaries[] | select(.actor_id == "player" and .action == "walk") | .frame_ids[]] | index("actor_player_walk_north_1") != null)
  and ([.clip_summaries[] | select(.actor_id == "player" and .action == "walk") | .frame_ids[]] | index("actor_player_walk_east_1") != null)
  and ([.clip_summaries[] | select(.actor_id == "player" and .action == "walk") | .frame_ids[]] | index("actor_player_walk_west_1") != null)
  and ([.clip_summaries[] | select(.actor_id == "mentor" and .action == "talk") | .frame_count] | first) >= 2
  and ([.clip_summaries[] | select(.actor_id == "mentor" and .action == "talk") | .frame_ids[]] | index("actor_mentor_talk") != null)
  and ([.clip_summaries[] | select(.actor_id == "enemy" and .action == "attack") | .frame_count] | first) >= 3
  and ([.clip_summaries[] | select(.actor_id == "enemy" and .action == "attack") | .frame_ids[]] | index("actor_enemy_attack") != null)
  and ([.clip_summaries[] | select(.actor_id == "enemy" and .action == "hit") | .frame_count] | first) >= 2
  and ([.clip_summaries[] | select(.actor_id == "enemy" and .action == "hit") | .frame_ids[]] | index("actor_enemy_hit") != null)
  and ([.clip_summaries[].refs_valid] | all)
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and .android_s5_real_device_claimed == false
  and .external_evidence_ignored_for_current_animation_preview_pass == true
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ANIMATION_PREVIEW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
