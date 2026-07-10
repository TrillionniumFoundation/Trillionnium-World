#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-contextual-button-state.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- contextual-button-state >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_contextual_button_state_v1"
  and .contextual_action_deck_contract == "trillionnium_world_bevy_contextual_action_deck_v1"
  and .green == true
  and .initial_visual_gate == true
  and .forge_visual_gate == true
  and .arena_visual_gate == true
  and .rematch_visual_gate == true
  and .final_visual_gate == true
  and .all_pressed_accepted == true
  and (.samples[] | select(.stage == "initial") | .contextual_button_states[] | select(.action_label == "TALK") | .visual_state) == "onboarding_next_button"
  and (.samples[] | select(.stage == "initial") | .contextual_button_texts[] | select(.action_label == "TALK") | .text) == ">> R TALK"
  and (.samples[] | select(.stage == "after_ROOM:forge-workbench") | .contextual_button_states[] | select(.action_label == "TASK:task-starter-forge-intro") | .visual_state) == "primary_enabled"
  and (.samples[] | select(.stage == "after_ROOM:league-coliseum") | .contextual_button_states[] | select(.action_label == "TASK:task-arena-rematch-route") | .reason) == "task_locked_by_progression:task-arena-rematch-route"
  and (.samples[] | select(.stage == "after_EQUIP:bandit_sash") | .contextual_button_texts[] | select(.action_label == "TASK:task-arena-rematch-route") | .text) == "> REMATCH"
  and (.samples[] | select(.stage == "after_TASK:task-arena-rematch-route") | .contextual_button_states[] | select(.action_label == "TASK:task-arena-rematch-route") | .reason) == "task_already_active:task-arena-rematch-route"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CONTEXTUAL_BUTTON_STATE_GREEN %s\n' "$SUMMARY"
