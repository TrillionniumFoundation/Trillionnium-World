#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-fog-scouting-intel.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-fog-scouting-intel.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-fog-scouting-intel "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_fog_scouting_intel_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_fog_scouting_intel_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.action_labels | index("RTS:SELECT:2") != null)
  and (.action_labels | index("RTS:QUEUE:recon:scout_enemy_base@10,2") != null)
  and (.action_labels | index("RTS:MOVE:9,2:rally") != null)
  and (.action_labels | index("RTS:QUEUE:recon:sweep:enemy_base@10,2") != null)
  and (.action_labels | index("RTS:QUEUE:recon:watchtower_scan@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:recon:mark:enemy_base@10,2") != null)
  and (.final_scout_unit_ids | length) >= 2
  and (.final_scout_route_tile_ids | length) >= 5
  and (.final_fog_reveal_tile_ids | length) >= 8
  and (.final_revealed_enemy_structure_ids | length) >= 3
  and (.final_revealed_enemy_unit_ids | length) >= 3
  and (.final_intel_log | index("marked:enemy_base@10,2") != null)
  and .final_visibility_percent == 100
  and (.final_command_queue | index("recon_scout:enemy_base@10,2") != null)
  and (.final_command_queue | index("recon_sweep:enemy_base@10,2") != null)
  and (.final_command_queue | index("recon_mark:enemy_base@10,2") != null)
  and .non_background_pixels > 250000
  and .scout_route_pixel_count > 80
  and .fog_reveal_pixel_count > 80
  and .enemy_structure_pixel_count > 80
  and .enemy_intel_pixel_count > 60
  and .visibility_bar_pixel_count > 20
  and .live_fog_scouting_input_gate == true
  and .scout_route_gate == true
  and .fog_reveal_gate == true
  and .enemy_structure_intel_gate == true
  and .enemy_unit_intel_gate == true
  and .intel_log_gate == true
  and .visibility_bar_gate == true
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_fog_core_frame_order_gate == true
  and .rts_fog_core_headless_replay_gate == true
  and (.rts_fog_core_frame_orders | length == 5)
  and (.rts_fog_core_frame_order_kind_labels | tostring == "[\"recon\",\"move\",\"recon\",\"recon\",\"recon\"]")
  and (.rts_fog_core_frame_order_errors | length == 0)
  and .rts_fog_core_frame_order_stream_error == null
  and (.rts_fog_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_fog_core_headless_replay_error == null
  and (.rts_fog_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_fog_core_headless_applied_order_count == 5
  and .rts_fog_core_headless_actor_count >= 2
  and .rts_fog_core_headless_final_frame == 904
  and .rts_fog_core_headless_recon_order_count == 4
  and .rts_fog_core_headless_scout_order_count == 1
  and .rts_fog_core_headless_sweep_order_count == 1
  and .rts_fog_core_headless_scan_order_count == 1
  and .rts_fog_core_headless_mark_order_count == 1
  and (.rts_fog_core_headless_recon_ids | index("enemy_base") != null)
  and (.rts_fog_core_headless_recon_ids | index("watchtower_scan") != null)
  and (.rts_fog_core_headless_recon_tile_ids | index("10,2") != null)
  and (.rts_fog_core_headless_recon_tile_ids | index("7,4") != null)
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FOG_SCOUTING_INTEL_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
