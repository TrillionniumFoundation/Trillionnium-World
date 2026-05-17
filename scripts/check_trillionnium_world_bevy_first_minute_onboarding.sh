#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-first-minute-onboarding.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-first-minute-onboarding-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- first-minute-onboarding "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_onboarding_v1"
  and .title_menu_contract == "trillionnium_world_bevy_title_menu_v1"
  and .character_create_contract == "trillionnium_world_bevy_character_create_v1"
  and .session_load_resume_contract == "trillionnium_world_bevy_session_load_resume_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .create_gate == true
  and .spawn_gate == true
  and .mentor_talk_gate == true
  and .training_gate == true
  and .arena_move_gate == true
  and .fight_gate == true
  and .save_gate == true
  and .title_continue_gate == true
  and .restored_state_gate == true
  and .input_path_gate == true
  and .title_new_event.action == "TITLE:NEW"
  and .title_new_event.accepted == true
  and .cycle_name_event.action == "CREATE:NAME"
  and .cycle_name_event.accepted == true
  and .cycle_archetype_event.action == "CREATE:ARCHETYPE"
  and .cycle_archetype_event.accepted == true
  and .confirm_create_event.action == "CREATE:CONFIRM"
  and .confirm_create_event.accepted == true
  and .after_spawn_sample.current_node_id == "mirror-city-square"
  and .after_spawn_sample.character_display_name == "Mira"
  and .after_spawn_sample.character_title == "artisan starter"
  and .talk_event.action == "TALK"
  and .talk_event.accepted == true
  and .talk_event.availability_before == "enabled_at_mentor_tile"
  and .after_talk_sample.runtime.dialogue_overlay_visible == true
  and (.after_talk_sample.runtime.npc_bubble_text | contains("TRAIN"))
  and .train_event.action == "TRAIN"
  and .train_event.accepted == true
  and .train_event.availability_before == "enabled_after_dialogue_choice"
  and .after_train_sample.runtime.indoor_tilemap_visible == true
  and .after_train_sample.runtime.active_scene_layer == "mentor_training_room_indoor_tilemap"
  and .after_train_sample.runtime.xp >= 10
  and .move_event.action == "MOVE:north"
  and .move_event.accepted == true
  and .move_event.availability_before == "enabled_route_step_north"
  and .after_arena_move_sample.current_node_id == "league-coliseum"
  and .after_arena_move_sample.runtime.active_scene_layer == "arena_outdoor_tilemap"
  and .fight_event.action == "FIGHT"
  and .fight_event.accepted == true
  and .fight_event.availability_before == "enabled_enemy_adjacent"
  and .after_fight_sample.runtime.objective_status == "combat_resolved"
  and .after_fight_sample.runtime.combat_overlay_was_visible == true
  and .after_fight_sample.runtime.enemy_hp < 40
  and .save_selected_event.action == "SAVE:SELECTED"
  and .save_selected_event.accepted == true
  and .slot_snapshot.character.display_name == "Mira"
  and .slot_snapshot.character.attributes.craft == 15
  and .slot_snapshot.first_playable.objective_status == "combat_resolved"
  and .slot_snapshot.first_playable.session_character_create_archetype == "artisan"
  and .title_open_event.action == "TITLE:OPEN"
  and .title_open_event.accepted == true
  and .title_continue_event.action == "TITLE:CONTINUE"
  and .title_continue_event.accepted == true
  and .title_continue_event.availability_before == "enabled_title_continue_slot:A"
  and .after_title_continue_sample.runtime.session_resume_input_locked == true
  and .continue_after_load_event.action == "CONTINUE:SESSION"
  and .continue_after_load_event.accepted == true
  and .final_sample.runtime.session_resume_input_locked == false
  and .final_sample.current_node_id == "league-coliseum"
  and .final_sample.character_display_name == "Mira"
  and .final_sample.runtime.objective_status == "combat_resolved"
  and .final_input_telemetry_summary.bevy_button_events >= 7
  and .final_input_telemetry_summary.accepted_events >= 7
  and .final_input_telemetry_summary.last_action_label == "CONTINUE:SESSION"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and .character.display_name == "Mira"
  and .character.attributes.craft == 15
  and .first_playable.session_character_create_name == "Mira"
  and .first_playable.session_character_create_archetype == "artisan"
  and .first_playable.objective_status == "combat_resolved"
  and .first_playable.session_resume_input_locked == false
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_ONBOARDING_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
