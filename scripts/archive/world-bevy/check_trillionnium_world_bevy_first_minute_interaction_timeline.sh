#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-first-minute-interaction-timeline.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-first-minute-interaction-timeline-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- first-minute-interaction-timeline "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_interaction_timeline_v1"
  and .next_button_highlight_contract == "trillionnium_world_bevy_next_button_highlight_v1"
  and .onboarding_objective_hud_contract == "trillionnium_world_bevy_onboarding_objective_hud_v1"
  and .first_minute_onboarding_contract == "trillionnium_world_bevy_first_minute_onboarding_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .timeline_length_gate == true
  and .expected_sequence_gate == true
  and .every_pre_click_binding_gate == true
  and .every_event_acceptance_gate == true
  and .every_post_click_transition_gate == true
  and .slot_write_gate == true
  and .final_completion_gate == true
  and (.timeline | length) == 10
  and .actual_before_sequence == [
    "TITLE:NEW",
    "CREATE:CONFIRM",
    "TALK",
    "TRAIN",
    "MOVE:north",
    "FIGHT",
    "SAVE:SELECTED",
    "TITLE:OPEN",
    "TITLE:CONTINUE",
    "CONTINUE:SESSION"
  ]
  and .actual_after_sequence == [
    "CREATE:CONFIRM",
    "TALK",
    "TRAIN",
    "MOVE:north",
    "FIGHT",
    "SAVE:SELECTED",
    "TITLE:OPEN",
    "TITLE:CONTINUE",
    "CONTINUE:SESSION",
    "FIRST MINUTE COMPLETE"
  ]
  and all(.timeline[]; .pre_click_binding_ok == true)
  and all(.timeline[]; .event_acceptance_ok == true)
  and all(.timeline[]; .post_click_transition_ok == true)
  and all(.timeline[]; .event.accepted == true)
  and all(.timeline[]; (.before_highlighted_buttons | length) == 1)
  and all(.timeline[]; .before_highlight_state.visual_state == "onboarding_next_button")
  and all(.timeline[]; .before_highlight_state.enabled == true and .before_highlight_state.primary == true)
  and all(.timeline[]; (.before_highlight_text | startswith(">> ")))
  and (.timeline[] | select(.step_name == "move_north") | .pressed_action) == "MOVE:north"
  and (.timeline[] | select(.step_name == "move_north") | .before_sample_summary.highlighted_buttons[0].action_label) == "MOVE:north"
  and (.timeline[] | select(.step_name == "title_continue") | .after_next_button) == "CONTINUE:SESSION"
  and (.timeline[] | select(.step_name == "continue_session") | .after_next_button) == "FIRST MINUTE COMPLETE"
  and .final_sample_summary.next_button == "FIRST MINUTE COMPLETE"
  and (.final_sample_summary.highlighted_buttons | length) == 0
  and .final_runtime.objective_status == "combat_resolved"
  and .final_runtime.session_resume_input_locked == false
  and .final_runtime.session_continue_cta_visible == false
  and (.final_runtime.progression_checkpoint_history | index("title_slot_A_restored")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_INTERACTION_TIMELINE_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
