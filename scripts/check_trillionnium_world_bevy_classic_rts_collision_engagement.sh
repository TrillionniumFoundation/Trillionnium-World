#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-collision-engagement.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-collision-engagement.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-collision-engagement "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_collision_engagement_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 360
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_collision_input)"
  and .input_action_count == 3
  and .accepted_input_count == 3
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:MOVE:8,4:wedge") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and .move_response_state == "blocked_detour_spread"
  and (.move_disperse_tile_ids | index("6,5") != null)
  and (.move_disperse_tile_ids | index("7,5") != null)
  and (.move_disperse_tile_ids | index("8,4") != null)
  and (.move_disperse_tile_ids | index("8,5") != null)
  and .final_unit_response_state == "engaged:arena_creep_attack"
  and (.engagement_tile_ids | length >= 4)
  and (.contact_flash_tile_ids | length >= 2)
  and (.final_command_queue | index("engage:6,5|6,4|7,5|5,5") != null)
  and (.final_command_queue | index("contact:6,5|6,4") != null)
  and .final_attack_target_id == "arena_creep_attack"
  and (.final_combat_event_log | index("target_acquired:arena_creep_attack") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_collision_core_frame_order_gate == true
  and .rts_collision_core_headless_replay_gate == true
  and (.rts_collision_core_frame_orders | length == 2)
  and (.rts_collision_core_frame_order_kind_labels | tostring == "[\"move\",\"attack\"]")
  and (.rts_collision_core_frame_order_errors | length == 0)
  and .rts_collision_core_frame_order_stream_error == null
  and (.rts_collision_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_collision_core_headless_replay_error == null
  and (.rts_collision_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_collision_core_headless_applied_order_count == 2
  and .rts_collision_core_headless_actor_count == 4
  and .rts_collision_core_headless_final_frame == 741
  and .rts_collision_core_headless_attack_order_count == 4
  and (.rts_collision_core_headless_event_log | any(contains(":kind:move:")))
  and (.rts_collision_core_headless_event_log | any(contains(":kind:attack:")))
  and (.rts_collision_core_headless_event_log | any(contains(":target:arena_creep_attack")))
  and .non_background_pixels > 240000
  and .dispersion_slot_pixel_count > 120
  and .engagement_range_pixel_count > 120
  and .contact_flash_pixel_count > 80
  and .blocked_tile_pixel_count > 40
  and .attack_feedback_pixel_count > 180
  and .live_collision_input_gate == true
  and .collision_response_gate == true
  and .engagement_response_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COLLISION_ENGAGEMENT_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
