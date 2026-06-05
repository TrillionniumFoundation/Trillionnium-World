#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCENE_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_scene_preview.sh"
MODEL_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh"
RENDERER_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh"

scene_required_lines=(
  'SUMMARY_RAW="$SUMMARY.raw"'
  'classic_scene_preview_green'
  'trillionnium_world_bevy_classic_scene_preview_v1'
  'status == "classic_scene_preview_green"'
  'external_evidence_ignored_for_current_scene_preview_pass'
  'panel_count == 4'
  'mirror_city_square'
  'mentor_training_room'
  'league_coliseum'
  'actor_player_walk_east_1'
  'actor_mentor_talk'
  'actor_enemy_attack'
  'production_ready_ui_claimed'
  'screen_for_screen_openra_ui_claimed'
  'third_party_asset_copied'
)

model_required_lines=(
  'SUMMARY_RAW="$SUMMARY.raw"'
  'classic_model_catalog_green'
  'trillionnium_world_bevy_classic_model_catalog_v1'
  'status == "classic_model_catalog_green"'
  'external_evidence_ignored_for_current_model_catalog_pass'
  'frame_count >= 43'
  'rendered_frame_count == .frame_count'
  'role_counts.player_actor >= 12'
  'actor_player_walk_south_1'
  'actor_player_walk_north_1'
  'actor_player_walk_east_1'
  'actor_player_walk_west_1'
  'actor_mentor_talk'
  'actor_enemy_attack'
  'marker_objective'
  'third_party_asset_copied'
)

renderer_required_lines=(
  'SUMMARY_RAW="$SUMMARY.raw"'
  'classic_renderer_probe_green'
  'trillionnium_world_bevy_classic_renderer_probe_v1'
  'status == "classic_renderer_probe_green"'
  'external_evidence_ignored_for_current_renderer_probe_pass'
  'frame_width == 640'
  'frame_height == 360'
  'player_frame_id == "actor_player_walk_east_1"'
  'hud_probe_gate == true'
  'player_frame_color_gate == true'
  'scene_frame_gate == true'
  'third_party_asset_copied'
)

for line in "${scene_required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCENE_SCRIPT"; then
    echo "[FAIL] classic scene preview missing line: $line" >&2
    exit 1
  fi
done

for line in "${model_required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$MODEL_SCRIPT"; then
    echo "[FAIL] classic model catalog missing line: $line" >&2
    exit 1
  fi
done

for line in "${renderer_required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$RENDERER_SCRIPT"; then
    echo "[FAIL] classic renderer probe missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic scene/model/renderer scripts keep visible manifest-backed visual semantics and no-credit boundaries"
