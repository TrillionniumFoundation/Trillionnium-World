#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_BEVY_ACCOUNT_CLIENT_BOUNDARY_SUMMARY:-$ROOT/acceptance/S5_native_bevy_device/latest/bevy-account-client-boundary.json}"

mkdir -p "$(dirname "$SUMMARY_FILE")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- account-client-boundary
) >"$SUMMARY_FILE"

jq -e '
  .contract_version == "trillionnium_world_bevy_account_client_boundary_v1"
  and .bevy_native_client_contract == "trillionnium_world_bevy_native_client_v1"
  and .account_api_contract == "trillionnium_world_account_api_v1"
  and .account_boundary_contract == "trillionnium_world_account_client_boundary_v1"
  and .player_client_owner == "trnm-world-bevy"
  and .account_api_owner == "trillionnium_world_account_api"
  and .report_account_client_contract == "trillionnium_world_bevy_account_client_boundary_v1"
  and .report_account_api_contract == "trillionnium_world_account_api_v1"
  and .profile_actor_id == "local-player"
  and .profile_display_name == "Local Trillionnium Player"
  and .default_room_id == "mirror-city-square"
  and .session_bound_to_bevy_actor == true
  and (.visible_entry_points | index("title_login") != null)
  and (.visible_entry_points | index("title_register") != null)
  and .passwords_tokens_or_cookie_values_logged == false
  and .cex_runtime_player_client_allowed == false
  and .bevy_projection_contains_account_client == true
  and .projection_account_client.player_client_owner == "trnm-world-bevy"
  and .green == true
' "$SUMMARY_FILE" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_ACCOUNT_CLIENT_BOUNDARY_GREEN %s\n' "$SUMMARY_FILE"
