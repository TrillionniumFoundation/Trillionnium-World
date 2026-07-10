#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-session-recovery-ui.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- session-recovery-ui >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_session_recovery_ui_v1"
  and .session_recovery_contract == "trillionnium_world_bevy_session_recovery_v1"
  and .input_telemetry_hud_contract == "trillionnium_world_bevy_input_telemetry_hud_v1"
  and .green == true
  and .panel_presence_gate == true
  and .recovered_status_gate == true
  and .continued_summary_gate == true
  and .guard_status_gate == true
  and .base_recovery_gate == true
  and (.restored_session_text | contains("SESSION RECOVERED"))
  and (.restored_session_text | contains("checkpoint events 10 accepted 5 blocked 5"))
  and (.restored_session_text | contains("last keyboard COMPLETE"))
  and (.final_session_text | contains("SESSION RECOVERED"))
  and (.final_session_text | contains("checkpoint events 12 accepted 6 blocked 6"))
  and (.final_session_text | contains("last keyboard EQUIP"))
  and (.final_session_text | contains("guard EQUIP:bandit_sash:item_not_in_bag:bandit_sash"))
  and .final_input_telemetry_summary.total_events == 12
  and .final_input_telemetry_summary.accepted_events == 6
  and .final_input_telemetry_summary.blocked_events == 6
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_SESSION_RECOVERY_UI_GREEN %s\n' "$SUMMARY"
