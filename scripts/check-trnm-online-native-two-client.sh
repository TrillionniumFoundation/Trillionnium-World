#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
RUN_ID="online-native-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/acceptance/online-native/$RUN_ID"
BIN="$ROOT_DIR/target/release/trnm-first-contact"
HOST_PID=""
GUEST_PID=""
XVFB_PID=""
mkdir -p "$EVIDENCE/host-save" "$EVIDENCE/guest-save"

cleanup() {
  local status=$?
  [[ -z "$HOST_PID" ]] || kill "$HOST_PID" >/dev/null 2>&1 || true
  [[ -z "$GUEST_PID" ]] || kill "$GUEST_PID" >/dev/null 2>&1 || true
  [[ -z "$XVFB_PID" ]] || kill "$XVFB_PID" >/dev/null 2>&1 || true
  systemctl --user unset-environment TRNM_GAME_SERVER_TICK_MS >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

admin_post() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local role="$1" account player recovery session
  account="$(admin_post /v1/accounts "$(jq -cn \
    --arg org '00000000-0000-0000-0000-00000000ce01' --arg role "$role" \
    '{org_id:$org,account_type:("online-native-"+$role),currency_unit:"credit",initial_balance:0}')" \
    | jq -er .account_id)"
  player="$RUN_ID-$role"
  recovery="recovery-$RUN_ID-$role-012345678901234567890123"
  admin_post /v1/trnm/identity/register "$(jq -cn \
    --arg player "$player" --arg account "$account" --arg recovery "$recovery" \
    '{player_id:$player,account_id:$account,recovery_key:$recovery}')" >/dev/null
  session="$(curl -fsS "$LEDGER_URL/v1/trnm/identity/session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg recovery "$recovery" --arg device "$RUN_ID-$role-device" \
      '{player_id:$player,recovery_key:$recovery,device_id:$device,lifetime_seconds:3600}')" \
    | jq -er .session_token)"
  printf '%s\t%s\t%s\n' "$player" "$account" "$session"
}

player_post() {
  local session="$1" path="$2" body="$3"
  curl -fsS "$ONLINE_URL$path" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$body"
}

window_for_pid() {
  local pid="$1" id actual
  while read -r id; do
    [[ -n "$id" ]] || continue
    actual="$(xprop -id "$id" _NET_WM_PID 2>/dev/null | awk -F' = ' '{print $2}')"
    if [[ "$actual" == "$pid" ]]; then
      printf '%s\n' "$id"
      return 0
    fi
  done < <(xwininfo -root -tree 2>/dev/null | awk \
    '/"Trillionnium — First Contact": \("trnm-first-contact" "trnm-first-contact"\)/ {print $1}')
  return 1
}

wait_for_window() {
  local pid="$1" id=""
  for _ in $(seq 1 90); do
    id="$(window_for_pid "$pid" || true)"
    if [[ -n "$id" ]]; then
      printf '%s\n' "$id"
      return 0
    fi
    sleep 1
  done
  return 1
}

capture() {
  local window_id="$1" output="$2"
  for _ in $(seq 1 10); do
    if xwd -silent -id "$window_id" 2>/dev/null \
      | xwdtopnm 2>/dev/null | pnmtopng >"$output" \
      && [[ -s "$output" ]]; then
      return 0
    fi
    sleep 1
  done
  return 1
}

IFS=$'\t' read -r HOST_PLAYER HOST_ACCOUNT HOST_SESSION < <(create_identity host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_ACCOUNT GUEST_SESSION < <(create_identity guest)

systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS=200
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "$ONLINE_URL/v1/online/readiness" | jq -e '.status == "ok"' >/dev/null

contract="trnm_online_authority_v1"
build="trnm-online-authority-2026.07-v1"
campaign="$(player_post "$HOST_SESSION" /v1/online/campaigns/connect "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg slot "$RUN_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,slot_key:$slot}')")"
created="$(player_post "$HOST_SESSION" /v1/online/matches "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" \
  --arg campaign "$(jq -er .campaign_id <<<"$campaign")" \
  --argjson revision "$(jq -er .campaign_revision <<<"$campaign")" \
  '{protocol_version:$protocol,build_id:$build,campaign_id:$campaign,map_id:"first_contact",expected_campaign_revision:$revision}')")"
MATCH_ID="$(jq -er .match_id <<<"$created")"
player_post "$GUEST_SESSION" /v1/online/matches/join "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" --arg code "$(jq -er .join_code <<<"$created")" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,join_code:$code}')" >/dev/null
player_post "$HOST_SESSION" "/v1/online/matches/$MATCH_ID/start" "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,expected_match_revision:0}')" >/dev/null

export DISPLAY="${TRNM_ONLINE_NATIVE_DISPLAY:-:97}"
Xvfb "$DISPLAY" -screen 0 2560x720x24 -nolisten tcp >"$EVIDENCE/xvfb.log" 2>&1 &
XVFB_PID=$!
for _ in $(seq 1 30); do
  xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break
  sleep 1
done
xdpyinfo -display "$DISPLAY" >/dev/null
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY TRNM_CEX_ENTRY_TOKEN

TRNM_CAMPAIGN_SAVE_PATH="$EVIDENCE/host-save/campaign.json" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" TRNM_ONLINE_MATCH_ID="$MATCH_ID" \
TRNM_CEX_ACTOR_ID="$HOST_PLAYER" TRNM_CEX_ACCOUNT_ID="$HOST_ACCOUNT" \
TRNM_CEX_PLAYER_SESSION="$HOST_SESSION" \
  "$BIN" >"$EVIDENCE/host.log" 2>&1 &
HOST_PID=$!
HOST_WINDOW="$(wait_for_window "$HOST_PID")"

TRNM_CAMPAIGN_SAVE_PATH="$EVIDENCE/guest-save/campaign.json" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" TRNM_ONLINE_MATCH_ID="$MATCH_ID" \
TRNM_CEX_ACTOR_ID="$GUEST_PLAYER" TRNM_CEX_ACCOUNT_ID="$GUEST_ACCOUNT" \
TRNM_CEX_PLAYER_SESSION="$GUEST_SESSION" \
  "$BIN" >"$EVIDENCE/guest.log" 2>&1 &
GUEST_PID=$!
GUEST_WINDOW="$(wait_for_window "$GUEST_PID")"

capture "$HOST_WINDOW" "$EVIDENCE/host-attached.png"
capture "$GUEST_WINDOW" "$EVIDENCE/guest-attached.png"
"$ROOT_DIR/scripts/x11_key_inject.py" "$HOST_WINDOW" q
for _ in $(seq 1 30); do
  host_commands="$(cex_psql_stdin -Atc "select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$HOST_PLAYER'")"
  [[ "$host_commands" -ge 1 ]] && break
  sleep 1
done
[[ "${host_commands:-0}" -ge 1 ]]

"$ROOT_DIR/scripts/x11_key_inject.py" "$GUEST_WINDOW" q
for _ in $(seq 1 30); do
  guest_commands="$(cex_psql_stdin -Atc "select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$GUEST_PLAYER'")"
  [[ "$guest_commands" -ge 1 ]] && break
  sleep 1
done
[[ "${guest_commands:-0}" -ge 1 ]]
capture "$HOST_WINDOW" "$EVIDENCE/host-command-ack.png"
capture "$GUEST_WINDOW" "$EVIDENCE/guest-command-ack.png"

database="$(cex_psql_stdin -Atc "select json_build_object(
  'members',(select count(*) from trnm_online_match_members where match_id = '$MATCH_ID'::uuid),
  'host_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$HOST_PLAYER'),
  'guest_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$GUEST_PLAYER'),
  'fingerprinted_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and length(request_hash) = 64),
  'distinct_control_sets',(select count(distinct controlled_unit_ids) from trnm_online_match_members where match_id = '$MATCH_ID'::uuid)
)" | jq -c .)"
jq -e '.members == 2 and .host_commands >= 1 and .guest_commands >= 1 and
  .fingerprinted_commands == (.host_commands + .guest_commands) and
  .distinct_control_sets == 2' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --arg match_id "$MATCH_ID" --arg evidence "$EVIDENCE" \
  --arg host_window "$HOST_WINDOW" --arg guest_window "$GUEST_WINDOW" \
  --argjson database "$database" \
  '{status:"passed",run_id:$run_id,match_id:$match_id,evidence:$evidence,
    native_x11_clients:2,distinct_windows:($host_window != $guest_window),
    server_authoritative_commands:true,database:$database,
    boundary:"automated two-window native attach/input smoke; not a human multiplayer session"}' \
  | tee "$EVIDENCE/report.json"
