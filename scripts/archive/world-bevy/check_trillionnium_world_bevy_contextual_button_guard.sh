#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-contextual-button-guard.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- contextual-button-guard >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_contextual_button_guard_v1"
  and .contextual_button_state_contract == "trillionnium_world_bevy_contextual_button_state_v1"
  and .green == true
  and .initial_disabled_guard == true
  and .arena_locked_guard == true
  and .unlocked_accepts_guard == true
  and .blocked_history_gate == true
  and .visual_guard == true
  and .disabled_event_count >= 4
  and .accepted_event_count >= 10
  and (.first_disabled_rematch_sample.runtime.current_room_id == "mirror-city-square")
  and (.first_disabled_rematch_sample.runtime.active_task_ids | length) == 0
  and (.first_disabled_rematch_sample.runtime.equipped_items | length) == 0
  and (.last_rematch_sample.runtime.active_task_ids | index("task-arena-rematch-route")) != null
  and (.last_rematch_sample.runtime.equipped_items | index("bandit_sash")) != null
  and (.final_runtime.blocked_action_history | index("TRAIN:talk_required_before_training")) != null
  and (.final_runtime.blocked_action_history | index("FIGHT:arena_required_before_fight")) != null
  and (.final_runtime.blocked_action_history | index("TASK:task-arena-rematch-route:task_locked_by_progression:task-arena-rematch-route")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CONTEXTUAL_BUTTON_GUARD_GREEN %s\n' "$SUMMARY"
