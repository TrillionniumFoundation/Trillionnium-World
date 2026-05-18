#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.ppm"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    cargo run -p trnm-world-bevy -- classic-animation-preview "$PREVIEW" >"$SUMMARY"
)

test -s "$SUMMARY"
test -s "$PREVIEW"
head -n 1 "$PREVIEW" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_animation_preview_v1"
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
  and ([.clip_summaries[] | select(.actor_id == "mentor" and .action == "talk") | .frame_count] | first) >= 2
  and ([.clip_summaries[] | select(.actor_id == "enemy" and .action == "attack") | .frame_count] | first) >= 3
  and ([.clip_summaries[] | select(.actor_id == "enemy" and .action == "hit") | .frame_count] | first) >= 2
  and ([.clip_summaries[].refs_valid] | all)
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ANIMATION_PREVIEW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
