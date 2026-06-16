#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.json"
SUMMARY_RAW="$SUMMARY.raw"
PROBE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.ppm"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-player-motion-probe "$PROBE" >"$SUMMARY_RAW"
)

jq '
  .status = "classic_player_motion_probe_green"
  | .android_s5_real_device_claimed = false
  | .external_evidence_ignored_for_current_player_motion_probe_pass = true
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
test -s "$PROBE"
head -n 1 "$PROBE" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_player_motion_probe_v1"
  and .status == "classic_player_motion_probe_green"
  and .green == true
  and .probe_format == "ppm_p3_rgb"
  and .probe_width == 640
  and .probe_height == 192
  and .probe_bytes > 100000
  and .sample_count == 8
  and .accepted_input_count == 8
  and .unique_color_count >= 16
  and .non_background_pixels > 45000
  and .label_pixel_count > 800
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .accepted_input_gate == true
  and .direction_coverage_gate == true
  and .frame_match_gate == true
  and .manifest_frame_gate == true
  and .sheet_gate == true
  and .label_gate == true
  and ([.selected_frame_ids[]] | index("actor_player_walk_north_1") != null)
  and ([.selected_frame_ids[]] | index("actor_player_walk_north_2") != null)
  and ([.selected_frame_ids[]] | index("actor_player_walk_east_1") != null)
  and ([.selected_frame_ids[]] | index("actor_player_walk_east_2") != null)
  and ([.selected_frame_ids[]] | index("actor_player_walk_south_1") != null)
  and ([.selected_frame_ids[]] | index("actor_player_walk_south_2") != null)
  and ([.selected_frame_ids[]] | index("actor_player_walk_west_1") != null)
  and ([.selected_frame_ids[]] | index("actor_player_walk_west_2") != null)
  and ([.samples[].accepted_local_input] | all)
  and ([.samples[].frame_match] | all)
  and ([.samples[] | select(.case_id == "north_1") | .selected_frame_id] | first) == "actor_player_walk_north_1"
  and ([.samples[] | select(.case_id == "north_2") | .selected_frame_id] | first) == "actor_player_walk_north_2"
  and ([.samples[] | select(.case_id == "east_1") | .selected_frame_id] | first) == "actor_player_walk_east_1"
  and ([.samples[] | select(.case_id == "east_2") | .selected_frame_id] | first) == "actor_player_walk_east_2"
  and ([.samples[] | select(.case_id == "south_1") | .selected_frame_id] | first) == "actor_player_walk_south_1"
  and ([.samples[] | select(.case_id == "south_2") | .selected_frame_id] | first) == "actor_player_walk_south_2"
  and ([.samples[] | select(.case_id == "west_1") | .selected_frame_id] | first) == "actor_player_walk_west_1"
  and ([.samples[] | select(.case_id == "west_2") | .selected_frame_id] | first) == "actor_player_walk_west_2"
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and .android_s5_real_device_claimed == false
  and .external_evidence_ignored_for_current_player_motion_probe_pass == true
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYER_MOTION_PROBE_GREEN %s %s\n' "$SUMMARY" "$PROBE"
