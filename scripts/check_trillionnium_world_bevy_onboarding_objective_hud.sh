#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-onboarding-objective-hud.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-onboarding-objective-hud-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- onboarding-objective-hud "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_onboarding_objective_hud_v1"
  and .first_minute_onboarding_contract == "trillionnium_world_bevy_first_minute_onboarding_v1"
  and .title_menu_contract == "trillionnium_world_bevy_title_menu_v1"
  and .character_create_contract == "trillionnium_world_bevy_character_create_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .title_hud_gate == true
  and .create_hud_gate == true
  and .spawn_hud_gate == true
  and .mentor_hud_gate == true
  and .training_hud_gate == true
  and .arena_hud_gate == true
  and .fight_hud_gate == true
  and .save_hud_gate == true
  and .title_continue_hud_gate == true
  and .resume_hud_gate == true
  and .complete_hud_gate == true
  and (.initial_title_sample.quest_panel_text | contains("FIRST MINUTE HUD"))
  and (.initial_title_sample.quest_panel_text | contains("NEXT BUTTON: TITLE:NEW"))
  and (.after_create_open_sample.quest_panel_text | contains("NEXT BUTTON: CREATE:CONFIRM"))
  and (.after_spawn_sample.quest_panel_text | contains("NEXT BUTTON: TALK"))
  and (.after_spawn_sample.quest_panel_text | contains("[x]CREATE"))
  and (.after_talk_sample.quest_panel_text | contains("NEXT BUTTON: TRAIN"))
  and (.after_talk_sample.quest_panel_text | contains("[x]TALK"))
  and (.after_train_sample.quest_panel_text | contains("NEXT BUTTON: MOVE:north"))
  and (.after_train_sample.quest_panel_text | contains("[x]TRAIN"))
  and (.after_arena_move_sample.quest_panel_text | contains("NEXT BUTTON: FIGHT"))
  and (.after_arena_move_sample.quest_panel_text | contains("[x]ARENA"))
  and (.after_fight_sample.quest_panel_text | contains("NEXT BUTTON: SAVE:SELECTED"))
  and (.after_fight_sample.quest_panel_text | contains("[x]FIGHT"))
  and (.after_save_sample.quest_panel_text | contains("NEXT BUTTON: TITLE:OPEN"))
  and (.after_save_sample.quest_panel_text | contains("[x]SAVE"))
  and (.after_title_open_sample.quest_panel_text | contains("NEXT BUTTON: TITLE:CONTINUE"))
  and (.after_title_continue_sample.quest_panel_text | contains("NEXT BUTTON: CONTINUE:SESSION"))
  and (.final_sample.quest_panel_text | contains("NEXT BUTTON: FIRST MINUTE COMPLETE"))
  and (.final_sample.quest_panel_text | contains("[x]CONTINUE"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_ONBOARDING_OBJECTIVE_HUD_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
