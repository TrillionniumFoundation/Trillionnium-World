#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-target-aggro-focus.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-target-aggro-focus.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-target-aggro-focus "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_target_aggro_focus_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_targeting_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:MOVE:8,4:wedge") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:focus_fire") != null)
  and .final_targeting_state == "focus_fire:arena_creep_attack"
  and .final_attack_target_id == "arena_creep_attack"
  and .final_aggro_target_id == "arena_creep_attack"
  and (.final_target_priority_ids | index("arena_creep_attack") != null)
  and (.final_target_priority_ids | index("arena_guard_support") != null)
  and (.final_target_priority_ids | index("arena_worker_support") != null)
  and (.final_focus_fire_unit_ids | length >= 4)
  and (.final_focus_fire_unit_ids | index("player") != null)
  and (.final_threat_level_percents | length >= 3)
  and (.final_threat_level_percents[0] == 100)
  and (.final_command_queue | index("priority:arena_creep_attack>arena_guard_support>arena_worker_support") != null)
  and (.final_command_queue | index("aggro:arena_creep_attack") != null)
  and (.final_command_queue | index("focus:player|square_guard_patrol|square_worker_carry|square_creep_wander") != null)
  and (.final_command_queue | index("focus_fire:arena_creep_attack") != null)
  and (.final_combat_event_log | index("target_acquired:arena_creep_attack") != null)
  and (.final_combat_event_log | index("focus_fire:arena_creep_attack") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_targeting_core_frame_order_gate == true
  and .rts_targeting_core_headless_replay_gate == true
  and (.rts_targeting_core_frame_orders | length == 3)
  and (.rts_targeting_core_frame_order_kind_labels | tostring == "[\"move\",\"attack\",\"ability\"]")
  and (.rts_targeting_core_frame_order_errors | length == 0)
  and .rts_targeting_core_frame_order_stream_error == null
  and (.rts_targeting_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_targeting_core_headless_replay_error == null
  and (.rts_targeting_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_targeting_core_headless_applied_order_count == 3
  and .rts_targeting_core_headless_actor_count >= 4
  and .rts_targeting_core_headless_final_frame == 683
  and .rts_targeting_core_headless_ability_order_count == 1
  and (.rts_targeting_core_headless_ability_rule_ids | index("focus_fire") != null)
  and (.rts_targeting_core_headless_ability_target_actor_ids | index("arena_creep_attack") != null)
  and .non_background_pixels > 220000
  and .target_priority_pixel_count > 80
  and .aggro_pixel_count > 80
  and .focus_fire_pixel_count > 80
  and .threat_bar_pixel_count > 40
  and .attack_feedback_pixel_count > 180
  and .live_targeting_input_gate == true
  and .target_priority_gate == true
  and .aggro_gate == true
  and .focus_fire_gate == true
  and .threat_feedback_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TARGET_AGGRO_FOCUS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
