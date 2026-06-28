#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PREVIEW_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_preview.sh"
SELECTOR_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_selector.sh"

preview_required_lines=(
  'bevy-classic-animation-preview.json'
  'bevy-classic-animation-preview.ppm'
  'SUMMARY_RAW="$SUMMARY.raw.$$"'
  'SUMMARY_TMP="$SUMMARY.tmp.$$"'
  'classic-animation-preview "$PREVIEW" >"$SUMMARY_RAW"'
  'status = "classic_animation_preview_green"'
  'ready_for_release_review == true'
  'gate_count == 8'
  'clip_summary_count == (.clip_summaries | length)'
  'unique_clip_action_count == ([.clip_summaries[].action] | unique | length)'
  'unique_clip_actor_count == ([.clip_summaries[].actor_id] | unique | length)'
  'trillionnium_world_bevy_classic_animation_preview_v1'
  'status == "classic_animation_preview_green"'
  'preview_format == "ppm_p3_rgb"'
  'clip_count >= 4'
  'rendered_frame_slot_count >= 15'
  'actor_player_walk_south_1'
  'actor_player_walk_north_1'
  'actor_player_walk_east_1'
  'actor_player_walk_west_1'
  'actor_mentor_talk'
  'actor_enemy_attack'
  'actor_enemy_hit'
  'all_clip_refs_valid == true'
  'external_evidence_ignored_for_current_animation_preview_pass == true'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'third_party_asset_copied == false'
)

selector_required_lines=(
  'bevy-classic-animation-selector.json'
  'SUMMARY_RAW="$SUMMARY.raw.$$"'
  'SUMMARY_TMP="$SUMMARY.tmp.$$"'
  'classic-animation-selector >"$SUMMARY_RAW"'
  'status = "classic_animation_selector_green"'
  'ready_for_release_review == true'
  'gate_count == 4'
  'case_detail_count == (.cases | length)'
  'selected_frame_count == (.selected_frames | length)'
  'unique_selected_frame_count == (.selected_frames | unique | length)'
  'trillionnium_world_bevy_classic_animation_selector_v1'
  'status == "classic_animation_selector_green"'
  'case_count >= 6'
  'selector_case_gate == true'
  'selected_frame_manifest_gate == true'
  'animation_transition_gate == true'
  'mentor_dialogue_talk'
  'enemy_combat_attack'
  'enemy_combat_hit'
  'objective_marker_pulse'
  'external_evidence_ignored_for_current_animation_selector_pass == true'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'third_party_asset_copied == false'
)

for line in "${preview_required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$PREVIEW_SCRIPT"; then
    echo "[FAIL] classic animation preview missing line: $line" >&2
    exit 1
  fi
done

for line in "${selector_required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SELECTOR_SCRIPT"; then
    echo "[FAIL] classic animation selector missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic animation preview/selector keep manifest-backed frame semantics and no-credit boundaries"
