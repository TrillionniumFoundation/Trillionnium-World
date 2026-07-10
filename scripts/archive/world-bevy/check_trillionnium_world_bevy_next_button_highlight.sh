#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-next-button-highlight.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-next-button-highlight-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- next-button-highlight "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_next_button_highlight_v1"
  and .onboarding_objective_hud_contract == "trillionnium_world_bevy_onboarding_objective_hud_v1"
  and .first_minute_onboarding_contract == "trillionnium_world_bevy_first_minute_onboarding_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .title_new_highlight_gate == true
  and .create_confirm_highlight_gate == true
  and .talk_highlight_gate == true
  and .train_highlight_gate == true
  and .move_highlight_gate == true
  and .fight_highlight_gate == true
  and .save_highlight_gate == true
  and .title_open_highlight_gate == true
  and .title_continue_highlight_gate == true
  and .resume_highlight_gate == true
  and .final_no_highlight_gate == true
  and (.initial_title_sample.contextual_button_states[]
    | select(.action_label == "TITLE:NEW")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.initial_title_sample.contextual_button_texts[]
    | select(.action_label == "TITLE:NEW")
    | .text == ">> NEW")
  and (.after_create_open_sample.contextual_button_states[]
    | select(.action_label == "CREATE:CONFIRM")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_spawn_sample.contextual_button_states[]
    | select(.action_label == "TALK")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_talk_sample.contextual_button_states[]
    | select(.action_label == "TRAIN")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_train_sample.contextual_button_states[]
    | select(.action_label == "MOVE:north")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_arena_move_sample.contextual_button_states[]
    | select(.action_label == "FIGHT")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_fight_sample.contextual_button_states[]
    | select(.action_label == "SAVE:SELECTED")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_save_sample.contextual_button_states[]
    | select(.action_label == "TITLE:OPEN")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_title_open_sample.contextual_button_states[]
    | select(.action_label == "TITLE:CONTINUE")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.after_title_continue_sample.contextual_button_states[]
    | select(.action_label == "CONTINUE:SESSION")
    | .visual_state == "onboarding_next_button" and .enabled == true and .primary == true)
  and (.final_sample.quest_panel_text | contains("NEXT BUTTON: FIRST MINUTE COMPLETE"))
  and ([.final_sample.contextual_button_states[] | select(.visual_state == "onboarding_next_button")] | length == 0)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_NEXT_BUTTON_HIGHLIGHT_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
