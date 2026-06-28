#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_player_motion_probe.sh"

required_lines=(
  'bevy-classic-player-motion-probe.json'
  'bevy-classic-player-motion-probe.ppm'
  'SUMMARY_RAW="$SUMMARY.raw.$$"'
  'SUMMARY_TMP="$SUMMARY.tmp.$$"'
  'classic-player-motion-probe "$PROBE" >"$SUMMARY_RAW"'
  'status = "classic_player_motion_probe_green"'
  'ready_for_release_review == true'
  'gate_count == 7'
  'sample_detail_count == (.samples | length)'
  'selected_frame_id_count == (.selected_frame_ids | length)'
  'unique_direction_count == ([.samples[].direction] | unique | length)'
  'trillionnium_world_bevy_classic_player_motion_probe_v1'
  'status == "classic_player_motion_probe_green"'
  'probe_format == "ppm_p3_rgb"'
  'sample_count == 8'
  'accepted_input_count == 8'
  'direction_coverage_gate == true'
  'frame_match_gate == true'
  'manifest_frame_gate == true'
  'actor_player_walk_north_1'
  'actor_player_walk_north_2'
  'actor_player_walk_east_1'
  'actor_player_walk_east_2'
  'actor_player_walk_south_1'
  'actor_player_walk_south_2'
  'actor_player_walk_west_1'
  'actor_player_walk_west_2'
  'samples[].accepted_local_input'
  'samples[].frame_match'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
  'android_s5_real_device_claimed == false'
  'external_evidence_ignored_for_current_player_motion_probe_pass == true'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'warcraft_iii_asset_copied == false'
  'openra_asset_copied == false'
  'third_party_asset_copied == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] classic player motion probe missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic player motion probe keeps Move input to directional walk-frame semantics and no-credit boundaries"
