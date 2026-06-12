#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ai-skirmish-pressure.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ai-skirmish-pressure.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-ai-skirmish-pressure "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_ai_skirmish_pressure_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_ai_skirmish_pressure_input)"
  and .input_action_count == 5
  and .accepted_input_count == 5
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:ai:skirmish_wave") != null)
  and (.action_labels | index("RTS:MOVE:8,4:wedge") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:guard_break") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_ai_skirmish_core_frame_order_stream.map_id == "first-contact-basin-ai-skirmish-pressure"
  and .rts_ai_skirmish_core_frame_order_stream.rules_id == "trnm-rts-core-ai-skirmish-pressure-rules-v1"
  and .rts_ai_skirmish_core_frame_order_kind_labels == ["queue", "move", "attack", "ability"]
  and (.rts_ai_skirmish_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.rts_ai_skirmish_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and (.rts_ai_skirmish_core_frame_order_errors | length) == 0
  and .rts_ai_skirmish_core_frame_order_stream_error == null
  and .rts_ai_skirmish_core_headless_replay_error == null
  and .rts_ai_skirmish_core_headless_applied_order_count == 4
  and .rts_ai_skirmish_core_headless_actor_count >= 4
  and .rts_ai_skirmish_core_headless_final_frame == 523
  and .rts_ai_skirmish_core_headless_queue_order_count >= 1
  and .rts_ai_skirmish_core_headless_micro_move_order_count == 1
  and .rts_ai_skirmish_core_headless_attack_order_count == 1
  and .rts_ai_skirmish_core_headless_ability_order_count == 1
  and (.rts_ai_skirmish_core_headless_combat_target_actor_ids | index("arena_creep_attack") != null)
  and (.rts_ai_skirmish_core_headless_combat_target_tile_ids | index("8,4") != null)
  and (.rts_ai_skirmish_core_headless_combat_formation_ids | index("wedge") != null)
  and (.rts_ai_skirmish_core_headless_ability_rule_ids | index("guard_break") != null)
  and (.final_ai_wave_unit_ids | length) >= 3
  and (.final_ai_pressure_tile_ids | length) >= 4
  and (.final_ai_counter_tile_ids | length) >= 4
  and .final_ai_retreat_tile_id == "9,2"
  and .final_ai_pressure_percent <= 34
  and .final_ai_skirmish_state == "countered:guard_break:skirmish_wave"
  and .final_ability_resolution_state == "resolved:guard_break:arena_creep_attack"
  and .final_target_health_percent <= 18
  and (.final_ai_response_log | index("counter_window:guard_break:skirmish_wave") != null)
  and (.final_command_queue | index("ai_counter:5,5>6,5>6,4>7,5") != null)
  and (.final_command_queue | index("ai_retreat:9,2") != null)
  and (.final_combat_event_log | index("shield_broken") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .ai_retreat_pixel_count > 40
  and .ai_pressure_bar_pixel_count > 20
  and .live_ai_skirmish_input_gate == true
  and .ai_wave_gate == true
  and .ai_counter_gate == true
  and .ai_pressure_resolution_gate == true
  and .ai_retreat_gate == true
  and .player_response_gate == true
  and .rts_ai_skirmish_core_frame_order_gate == true
  and .rts_ai_skirmish_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_AI_SKIRMISH_PRESSURE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
