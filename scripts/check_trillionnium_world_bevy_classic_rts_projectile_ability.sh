#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-projectile-ability "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_projectile_ability_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_projectile_ability_input)"
  and .input_action_count == 5
  and .accepted_input_count == 5
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
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PROJECTILE_ABILITY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
