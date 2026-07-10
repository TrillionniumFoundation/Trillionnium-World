#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-selector.json"
SUMMARY_RAW="$SUMMARY.raw.$$"
SUMMARY_TMP="$SUMMARY.tmp.$$"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-animation-selector >"$SUMMARY_RAW"
)

jq '
  .status = "classic_animation_selector_green"
  | .ready_for_release_review = true
  | .case_detail_count = (.cases | length)
  | .selected_frame_count = (.selected_frames | length)
  | .unique_selected_frame_count = (.selected_frames | unique | length)
  | .gate_count = 4
  | .passed_gate_count = ([
      .atlas_parse_gate,
      .selector_case_gate,
      .selected_frame_manifest_gate,
      .animation_transition_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
  | .android_s5_real_device_claimed = false
  | .external_evidence_ignored_for_current_animation_selector_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

test -s "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_animation_selector_v1"
  and .status == "classic_animation_selector_green"
  and .green == true
  and .ready_for_release_review == true
  and .gate_count == 4
  and .passed_gate_count == 4
  and .failed_gate_count == 0
  and .case_count >= 6
  and .case_detail_count == (.cases | length)
  and .selected_frame_count == (.selected_frames | length)
  and .unique_selected_frame_count == (.selected_frames | unique | length)
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .selector_case_gate == true
  and .selected_frame_manifest_gate == true
  and .animation_transition_gate == true
  and ([.cases[] | select(.case_id == "mentor_idle") | .selected_frame_id] | first) == "actor_mentor_idle"
  and ([.cases[] | select(.case_id == "mentor_dialogue_talk") | .selected_frame_id] | first) == "actor_mentor_talk"
  and ([.cases[] | select(.case_id == "enemy_idle") | .selected_frame_id] | first) == "actor_enemy_idle"
  and ([.cases[] | select(.case_id == "enemy_combat_attack") | .selected_frame_id] | first) == "actor_enemy_attack"
  and ([.cases[] | select(.case_id == "enemy_combat_hit") | .selected_frame_id] | first) == "actor_enemy_hit"
  and ([.cases[] | select(.case_id == "objective_marker_pulse") | .selected_frame_id] | first) == "marker_interaction"
  and ([.selected_frames[]] | index("actor_mentor_talk") != null)
  and ([.selected_frames[]] | index("actor_enemy_attack") != null)
  and ([.selected_frames[]] | index("actor_enemy_hit") != null)
  and ([.selected_frames[]] | index("marker_interaction") != null)
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and .android_s5_real_device_claimed == false
  and .external_evidence_ignored_for_current_animation_selector_pass == true
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ANIMATION_SELECTOR_GREEN %s\n' "$SUMMARY"
