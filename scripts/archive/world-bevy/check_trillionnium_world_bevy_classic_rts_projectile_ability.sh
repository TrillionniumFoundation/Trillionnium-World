#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.ppm"
RAW_SUMMARY="$SUMMARY.raw.$$"
TMP_SUMMARY="$SUMMARY.tmp.$$"
mkdir -p "$(dirname "$SUMMARY")"
cleanup() {
  rm -f "$RAW_SUMMARY" "$TMP_SUMMARY"
}
trap cleanup EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-projectile-ability "$PREVIEW" >"$RAW_SUMMARY"

jq '
  .action_label_count = ((.action_labels // []) | length)
  | .input_source_count = ((.input_sources // []) | length)
  | .stage_summary_count = ((.stage_summaries // []) | length)
  | .final_projectile_trail_tile_count = ((.final_projectile_trail_tile_ids // []) | length)
  | .final_ability_effect_tile_count = ((.final_ability_effect_tile_ids // []) | length)
  | .final_ability_damage_tick_count = ((.final_ability_damage_ticks // []) | length)
  | .final_command_queue_count = ((.final_command_queue // []) | length)
  | .final_combat_event_log_count = ((.final_combat_event_log // []) | length)
  | .rts_projectile_ability_core_frame_order_count = ((.rts_projectile_ability_core_frame_orders // []) | length)
  | .rts_projectile_ability_core_frame_order_kind_label_count = ((.rts_projectile_ability_core_frame_order_kind_labels // []) | length)
  | .rts_projectile_ability_core_frame_order_error_count = ((.rts_projectile_ability_core_frame_order_errors // []) | length)
  | .rts_projectile_ability_core_headless_ability_rule_count = ((.rts_projectile_ability_core_headless_ability_rule_ids // []) | length)
  | .rts_projectile_ability_core_headless_ability_target_actor_count = ((.rts_projectile_ability_core_headless_ability_target_actor_ids // []) | length)
  | .projectile_ability_gate_count = ([.write_gate, .live_projectile_ability_input_gate, .projectile_trail_gate, .projectile_impact_gate, .ability_radius_gate, .damage_tick_gate, .armor_shield_gate, .rts_projectile_ability_core_frame_order_gate, .rts_projectile_ability_core_headless_replay_gate] | length)
  | .projectile_ability_passed_gate_count = ([.write_gate, .live_projectile_ability_input_gate, .projectile_trail_gate, .projectile_impact_gate, .ability_radius_gate, .damage_tick_gate, .armor_shield_gate, .rts_projectile_ability_core_frame_order_gate, .rts_projectile_ability_core_headless_replay_gate] | map(select(. == true)) | length)
  | .projectile_ability_failed_gate_count = ([.write_gate, .live_projectile_ability_input_gate, .projectile_trail_gate, .projectile_impact_gate, .ability_radius_gate, .damage_tick_gate, .armor_shield_gate, .rts_projectile_ability_core_frame_order_gate, .rts_projectile_ability_core_headless_replay_gate] | map(select(. != true)) | length)
' "$RAW_SUMMARY" >"$TMP_SUMMARY"
mv "$TMP_SUMMARY" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_projectile_ability_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_projectile_ability_input)"
  and .input_action_count == 5
  and .accepted_input_count == 5
  and .action_label_count == (.action_labels | length)
  and .input_source_count == (.input_sources | length)
  and .stage_summary_count == (.stage_summaries | length)
  and .final_projectile_trail_tile_count == (.final_projectile_trail_tile_ids | length)
  and .final_ability_effect_tile_count == (.final_ability_effect_tile_ids | length)
  and .final_ability_damage_tick_count == (.final_ability_damage_ticks | length)
  and .final_command_queue_count == (.final_command_queue | length)
  and .final_combat_event_log_count == (.final_combat_event_log | length)
  and .rts_projectile_ability_core_frame_order_count == (.rts_projectile_ability_core_frame_orders | length)
  and .rts_projectile_ability_core_frame_order_kind_label_count == (.rts_projectile_ability_core_frame_order_kind_labels | length)
  and .rts_projectile_ability_core_frame_order_error_count == (.rts_projectile_ability_core_frame_order_errors | length)
  and .rts_projectile_ability_core_headless_ability_rule_count == (.rts_projectile_ability_core_headless_ability_rule_ids | length)
  and .rts_projectile_ability_core_headless_ability_target_actor_count == (.rts_projectile_ability_core_headless_ability_target_actor_ids | length)
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:MOVE:8,4:wedge") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:focus_fire") != null)
  and (.action_labels | index("RTS:ABILITY:guard_break") != null)
  and .final_active_projectile_id == "guard_break_bolt"
  and (.final_projectile_trail_tile_ids | length) >= 4
  and .final_projectile_impact_tile_id == "6,5"
  and (.final_ability_effect_tile_ids | length) >= 4
  and (.final_ability_effect_tile_ids | index("6,4") != null)
  and (.final_ability_damage_ticks | length) >= 3
  and (.final_ability_damage_ticks | add) >= 72
  and .final_target_health_percent <= 18
  and .final_target_armor_percent == 18
  and .final_target_shield_percent == 0
  and .final_ability_resolution_state == "resolved:guard_break:arena_creep_attack"
  and (.final_command_queue | index("damage_ticks:16+21+35") != null)
  and (.final_command_queue | index("armor_shield:18:0") != null)
  and (.final_combat_event_log | index("projectile_impact:guard_break:arena_creep_attack") != null)
  and (.final_combat_event_log | index("shield_broken") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_projectile_ability_core_frame_order_gate == true
  and .rts_projectile_ability_core_headless_replay_gate == true
  and (.rts_projectile_ability_core_frame_orders | length == 4)
  and (.rts_projectile_ability_core_frame_order_kind_labels | tostring == "[\"move\",\"attack\",\"ability\",\"ability\"]")
  and (.rts_projectile_ability_core_frame_order_errors | length == 0)
  and .rts_projectile_ability_core_frame_order_stream_error == null
  and (.rts_projectile_ability_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_projectile_ability_core_headless_replay_error == null
  and (.rts_projectile_ability_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_projectile_ability_core_headless_applied_order_count == 4
  and .rts_projectile_ability_core_headless_actor_count >= 2
  and .rts_projectile_ability_core_headless_final_frame == 704
  and .rts_projectile_ability_core_headless_ability_order_count == 2
  and (.rts_projectile_ability_core_headless_ability_rule_ids | index("focus_fire") != null)
  and (.rts_projectile_ability_core_headless_ability_rule_ids | index("guard_break") != null)
  and ((.rts_projectile_ability_core_headless_ability_target_actor_ids | map(select(. == "arena_creep_attack")) | length) == 2)
  and .non_background_pixels > 250000
  and .projectile_trail_pixel_count > 80
  and .projectile_impact_pixel_count > 80
  and .ability_radius_pixel_count > 140
  and .damage_tick_pixel_count > 40
  and .armor_shield_pixel_count > 20
  and .attack_feedback_pixel_count > 180
  and .live_projectile_ability_input_gate == true
  and .projectile_trail_gate == true
  and .projectile_impact_gate == true
  and .ability_radius_gate == true
  and .damage_tick_gate == true
  and .armor_shield_gate == true
  and .projectile_ability_gate_count == 9
  and .projectile_ability_passed_gate_count == 9
  and .projectile_ability_failed_gate_count == 0
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PROJECTILE_ABILITY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
