#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-character-create.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-character-create-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- character-create "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_character_create_v1"
  and .title_menu_contract == "trillionnium_world_bevy_title_menu_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .create_open_gate == true
  and .create_locked_input_gate == true
  and .create_choice_gate == true
  and .back_to_title_gate == true
  and .reopen_preserves_choices_gate == true
  and .confirm_create_gate == true
  and .save_created_character_gate == true
  and .title_new_event.action == "TITLE:NEW"
  and .title_new_event.accepted == true
  and .after_create_open_sample.runtime.session_character_create_visible == true
  and .after_create_open_sample.runtime.session_character_create_input_locked == true
  and (.after_create_open_sample.character_create_text | contains("CREATE ACTIVE"))
  and .create_locked_talk_event.action == "TALK"
  and .create_locked_talk_event.accepted == false
  and .create_locked_talk_event.availability_before == "character_create_confirm_required"
  and .cycle_name_event.action == "CREATE:NAME"
  and .cycle_name_event.accepted == true
  and .cycle_archetype_event.action == "CREATE:ARCHETYPE"
  and .cycle_archetype_event.accepted == true
  and .after_create_choices_sample.runtime.session_character_create_name == "Mira"
  and .after_create_choices_sample.runtime.session_character_create_archetype == "artisan"
  and (.after_create_choices_sample.character_create_text | contains("name Mira"))
  and (.after_create_choices_sample.character_create_text | contains("style artisan"))
  and .back_to_title_event.action == "CREATE:BACK"
  and .back_to_title_event.accepted == true
  and .after_back_to_title_sample.runtime.session_title_menu_visible == true
  and .after_back_to_title_sample.runtime.session_character_create_visible == false
  and .reopen_create_event.action == "TITLE:NEW"
  and .reopen_create_event.accepted == true
  and .after_reopen_create_sample.runtime.session_character_create_name == "Mira"
  and .after_reopen_create_sample.runtime.session_character_create_archetype == "artisan"
  and .confirm_create_event.action == "CREATE:CONFIRM"
  and .confirm_create_event.accepted == true
  and .after_confirm_create_sample.runtime.session_character_create_visible == false
  and .after_confirm_create_sample.runtime.session_title_boot_state == "new_game_started"
  and .after_confirm_create_sample.character_display_name == "Mira"
  and .after_confirm_create_sample.character_title == "artisan starter"
  and .after_confirm_create_sample.character_attributes.craft == 15
  and .save_selected_event.action == "SAVE:SELECTED"
  and .save_selected_event.accepted == true
  and (.after_save_sample.session_slot_text | contains("A:saved"))
  and .final_character.display_name == "Mira"
  and .final_character.title == "artisan starter"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and .character.display_name == "Mira"
  and .character.title == "artisan starter"
  and .character.attributes.craft == 15
  and .first_playable.session_character_create_name == "Mira"
  and .first_playable.session_character_create_archetype == "artisan"
  and .first_playable.session_character_create_visible == false
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CHARACTER_CREATE_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
