#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-input-affordance-feedback.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- input-affordance-feedback >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_input_affordance_feedback_v1"
  and .runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1"
  and .green == true
  and .initial_ready_toast_gate == true
  and .input_hud_gate == true
  and .next_button_affordance_gate == true
  and .locked_button_affordance_gate == true
  and .blocked_fight_toast_gate == true
  and .accepted_title_toast_gate == true
  and .blocked_create_toast_gate == true
  and .android_s5_real_device_claimed == false
  and (.samples[0].input_hint_text | contains("INPUT HUD | NEXT TITLE:NEW"))
  and (.samples[0].input_hint_text | contains("READY:"))
  and (.samples[0].input_hint_text | contains("LOCKED:"))
  and (.samples[1].feedback_banner_text | contains("TOAST BLOCKED | FIGHT"))
  and (.samples[1].feedback_banner_text | contains("NEXT TITLE:NEW"))
  and (.samples[2].feedback_banner_text | contains("TOAST OK | TITLE:NEW"))
  and (.samples[2].feedback_banner_text | contains("NEXT CREATE:CONFIRM"))
  and (.samples[3].feedback_banner_text | contains("TOAST BLOCKED | TALK"))
  and (.samples[3].feedback_banner_text | contains("NEXT CREATE:CONFIRM"))
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_INPUT_AFFORDANCE_FEEDBACK_GREEN %s\n' "$SUMMARY"
