#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_account_title_flow.sh"

while IFS= read -r line; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] Bevy account title flow script missing contract line: $line" >&2
    exit 1
  fi
done <<'REQUIRED_LINES'
trillionnium_world_bevy_account_title_flow_v1
trillionnium_world_bevy_account_client_boundary_v1
trillionnium_world_account_api_v1
trillionnium_world_account_client_boundary_v1
bevy-account-title-flow.json
TRILLIONNIUM_WORLD_BEVY_ACCOUNT_TITLE_FLOW_SUMMARY
cargo run -p trnm-world-bevy -- account-title-flow
ACCOUNT:REGISTER
ACCOUNT:LOGIN
ACCOUNT:CONTINUE
player_client_owner == "trnm-world-bevy"
account_api_owner == "trillionnium_world_account_api"
passwords_tokens_or_cookie_values_logged == false
cex_runtime_player_client_allowed == false
character_identity_gate == true
account_identity_persistence_gate == true
final_sample.character_display_name == "Local Trillionnium Player"
restored_account_character_display_name == "Local Trillionnium Player"
restored_account_auth_state == "signed_in"
restored_account_last_action == "session"
restored_account_session_bound == true
TRILLIONNIUM_WORLD_BEVY_ACCOUNT_TITLE_FLOW_GREEN
REQUIRED_LINES

echo "[PASS] Bevy account title flow script gates register/login/continue buttons through Trillionnium account API"
