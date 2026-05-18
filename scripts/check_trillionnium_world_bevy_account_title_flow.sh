#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_BEVY_ACCOUNT_TITLE_FLOW_SUMMARY:-$ROOT/acceptance/S5_native_bevy_device/latest/bevy-account-title-flow.json}"

mkdir -p "$(dirname "$SUMMARY_FILE")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- account-title-flow
) >"$SUMMARY_FILE"

jq -e '
  .contract_version == "trillionnium_world_bevy_account_title_flow_v1"
  and .account_client_contract == "trillionnium_world_bevy_account_client_boundary_v1"
  and .account_api_contract == "trillionnium_world_account_api_v1"
  and .account_boundary_contract == "trillionnium_world_account_client_boundary_v1"
  and .player_client_owner == "trnm-world-bevy"
  and .account_api_owner == "trillionnium_world_account_api"
  and (.title_actions | index("ACCOUNT:REGISTER") != null)
  and (.title_actions | index("ACCOUNT:LOGIN") != null)
  and (.title_actions | index("ACCOUNT:CONTINUE") != null)
  and .session_account_auth_state == "signed_in"
  and .session_account_display_name == "Local Trillionnium Player"
  and .session_account_last_action == "session"
  and .session_account_session_bound == true
  and .passwords_tokens_or_cookie_values_logged == false
  and .cex_runtime_player_client_allowed == false
  and .register_gate == true
  and .login_gate == true
  and .continue_gate == true
  and .character_identity_gate == true
  and .final_sample.character_display_name == "Local Trillionnium Player"
  and .no_cex_gate == true
  and .green == true
' "$SUMMARY_FILE" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_ACCOUNT_TITLE_FLOW_GREEN %s\n' "$SUMMARY_FILE"
