#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-first-minute-input-replay.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-first-minute-input-replay-slots"
RECORDING="$EVIDENCE_DIR/bevy-first-minute-input-recording.json"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- first-minute-input-replay "$SLOT_DIR" "$RECORDING" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_input_replay_v1"
  and .recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1"
  and .interaction_timeline_contract == "trillionnium_world_bevy_first_minute_interaction_timeline_v1"
  and .next_button_highlight_contract == "trillionnium_world_bevy_next_button_highlight_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .recording_path == "'"$RECORDING"'"
  and .recording_bytes > 512
  and .slot_a_bytes > 512
  and .recording_parse_gate == true
  and .replay_length_gate == true
  and .replay_parse_gate == true
  and .replay_pre_click_binding_gate == true
  and .replay_event_acceptance_gate == true
  and .replay_transition_gate == true
  and .signature_match_gate == true
  and .final_completion_gate == true
  and .replay_slot_write_gate == true
  and (.recording.steps | length) == 10
  and (.replay_steps | length) == 10
  and [.recording.steps[].action_label] == [
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
  and all(.replay_steps[]; .parsed_action == true)
  and all(.replay_steps[]; .pre_click_binding_ok == true)
  and all(.replay_steps[]; .replay_event_ok == true)
  and all(.replay_steps[]; .post_click_transition_ok == true)
  and all(.replay_steps[]; .replay_event.accepted == true)
  and all(.replay_steps[]; (.before_highlighted_buttons | length) == 1)
  and all(.replay_steps[]; .before_highlight_state.visual_state == "onboarding_next_button")
  and (.replay_steps[] | select(.step_name == "move_north") | .action_label) == "MOVE:north"
  and (.replay_steps[] | select(.step_name == "continue_session") | .after_next_button) == "FIRST MINUTE COMPLETE"
  and .recording_final_signature == .replay_final_signature
  and .final_sample_summary.next_button == "FIRST MINUTE COMPLETE"
  and (.final_sample_summary.highlighted_buttons | length) == 0
  and .replay_final_runtime.objective_status == "combat_resolved"
  and .replay_final_runtime.session_resume_input_locked == false
  and .replay_final_runtime.session_continue_cta_visible == false
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_input_recording_v1"
  and .source_timeline_contract == "trillionnium_world_bevy_first_minute_interaction_timeline_v1"
  and .source_timeline_green == true
  and (.steps | length) == 10
  and [.steps[].action_label] == [
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
  and .android_s5_real_device_claimed == false
' "$RECORDING" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_INPUT_REPLAY_GREEN %s recording=%s slot_dir=%s\n' "$SUMMARY" "$RECORDING" "$SLOT_DIR"
