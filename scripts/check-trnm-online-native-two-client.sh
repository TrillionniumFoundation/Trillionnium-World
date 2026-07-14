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
NETEM_APPLIED=0
CHAOS_RTT_MS="${TRNM_NATIVE_CHAOS_RTT_MS:-}"
CHAOS_LOSS_PERCENT="${TRNM_NATIVE_CHAOS_LOSS_PERCENT:-0}"
MATCH_ID=""
mkdir -p "$EVIDENCE/host-save" "$EVIDENCE/guest-save" \
  "$EVIDENCE/host-journal" "$EVIDENCE/guest-journal"

cleanup() {
  local status=$?
  [[ -z "$HOST_PID" ]] || kill "$HOST_PID" >/dev/null 2>&1 || true
  [[ -z "$GUEST_PID" ]] || kill "$GUEST_PID" >/dev/null 2>&1 || true
  [[ -z "$XVFB_PID" ]] || kill "$XVFB_PID" >/dev/null 2>&1 || true
  if [[ "$NETEM_APPLIED" == "1" ]]; then
    sudo -n "${TC:-/usr/sbin/tc}" qdisc del dev lo root >/dev/null 2>&1 || true
  fi
  if [[ -n "$MATCH_ID" ]]; then
    cex_psql_stdin -c "
      update trnm_online_matches
      set phase='failed_closed', settlement_state='failed_closed',
          failure_reason='native render/network smoke completed without settlement',
          updated_at=now()
      where match_id='$MATCH_ID'::uuid and phase='running'" >/dev/null 2>&1 || true
  fi
  systemctl --user unset-environment TRNM_GAME_SERVER_TICK_MS TRNM_ALLOW_ACCELERATED_TEST_CLOCK >/dev/null 2>&1 || true
  systemctl --user reset-failed trnm-game-server.service >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

admin_post() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local role="$1" account player credential session invite identity
  player="$RUN_ID-$role"
  credential="credential-$RUN_ID-$role-012345678901234567890123"
  invite="$(admin_post /v1/trnm/product/registration-invites \
    '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
  identity="$(curl -fsS "$LEDGER_URL/v1/trnm/product/register" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg credential "$credential" --arg invite "$invite" \
      '{player_id:$player,credential:$credential,invite_code:$invite}')")"
  account="$(jq -er .account_id <<<"$identity")"
  session="$(curl -fsS "$LEDGER_URL/v1/trnm/product/login" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg credential "$credential" --arg device "$RUN_ID-$role-device" \
      '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')" \
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
  local rendered_pixels
  for _ in $(seq 1 10); do
    if xwd -silent -id "$window_id" 2>/dev/null \
      | xwdtopnm 2>/dev/null | pnmtopng >"$output" \
      && [[ -s "$output" ]]; then
      rendered_pixels="$(pngtopnm "$output" 2>/dev/null \
        | od -An -v -tu1 \
        | awk '{ for (i = 1; i <= NF; i++) if ($i >= 16) count++ } END { print count + 0 }')"
      if (( rendered_pixels >= 5000 )); then
        return 0
      fi
    fi
    sleep 1
  done
  echo "native window $window_id never produced a non-black rendered frame" >&2
  return 1
}

wait_for_frame_timing() {
  local path="$1"
  for _ in $(seq 1 120); do
    if [[ -s "$path" ]] && jq -e '.frame_count >= 10' "$path" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.25
  done
  echo "native client did not produce ten post-warmup frame samples: $path" >&2
  return 1
}

IFS=$'\t' read -r HOST_PLAYER HOST_ACCOUNT HOST_SESSION < <(create_identity host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_ACCOUNT GUEST_SESSION < <(create_identity guest)

systemctl --user reset-failed trnm-game-server.service
systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS=200 \
  TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "$ONLINE_URL/v1/online/readiness" | jq -e '.status == "ok"' >/dev/null

contract="trnm_online_authority_v3"
build="trnm-online-authority-2026.07-v3"
product_contract="trnm_online_product_v2"
product_build="trnm-online-product-2026.07-v2"
campaign="$(player_post "$HOST_SESSION" /v1/online/campaigns/connect "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg slot "$RUN_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,slot_key:$slot}')")"
guest_campaign="$(player_post "$GUEST_SESSION" /v1/online/campaigns/connect "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" --arg slot "$RUN_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,slot_key:$slot}')")"
lobby="$(player_post "$HOST_SESSION" /v1/product/lobbies "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg campaign "$(jq -er .campaign_id <<<"$campaign")" \
  --arg name "$RUN_ID party" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,display_name:$name,map_id:"first_contact"}')")"
LOBBY_ID="$(jq -er .lobby_id <<<"$lobby")"
invite="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/invites" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg target "$GUEST_PLAYER" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,target_player_id:$target,expected_lobby_revision:0}')")"
lobby="$(player_post "$GUEST_SESSION" /v1/product/lobbies/invites/accept "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" --arg campaign "$(jq -er .campaign_id <<<"$guest_campaign")" \
  --arg token "$(jq -er .invite_token <<<"$invite")" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,invite_token:$token}')")"
lobby="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/ready" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,ready:true,expected_lobby_revision:1}')")"
lobby="$(player_post "$GUEST_SESSION" "/v1/product/lobbies/$LOBBY_ID/ready" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,ready:true,expected_lobby_revision:2}')")"
allocation="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/queue" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,expected_lobby_revision:3}')")"
MATCH_ID="$(jq -er .match_view.match_id <<<"$allocation")"

if [[ -n "$CHAOS_RTT_MS" || -n "${TRNM_NATIVE_CHAOS_LATENCY_MS:-}" ]]; then
  TC="${TC:-/usr/sbin/tc}"
  jq -en --arg value "$CHAOS_LOSS_PERCENT" \
    '($value | tonumber) >= 0 and ($value | tonumber) <= 100' >/dev/null
  if [[ -n "$CHAOS_RTT_MS" ]]; then
    [[ "$CHAOS_RTT_MS" =~ ^[0-9]+$ ]] && (( CHAOS_RTT_MS > 0 ))
    one_way_delay_ms=$(( (CHAOS_RTT_MS + 1) / 2 ))
  else
    [[ "${TRNM_NATIVE_CHAOS_LATENCY_MS}" =~ ^[0-9]+$ ]] \
      && (( TRNM_NATIVE_CHAOS_LATENCY_MS > 0 ))
    one_way_delay_ms="${TRNM_NATIVE_CHAOS_LATENCY_MS}"
    CHAOS_RTT_MS=$(( one_way_delay_ms * 2 ))
  fi
  sudo -n true
  [[ -x "$TC" ]]
  "$TC" qdisc show dev lo | grep -q '^qdisc noqueue'
  sudo -n "$TC" qdisc add dev lo root handle 1: prio bands 3
  NETEM_APPLIED=1
  sudo -n "$TC" qdisc add dev lo parent 1:3 handle 30: netem \
    delay "${one_way_delay_ms}ms" loss "${CHAOS_LOSS_PERCENT}%"
  sudo -n "$TC" filter add dev lo protocol ip parent 1:0 prio 3 u32 \
    match ip dport 7005 0xffff flowid 1:3
  sudo -n "$TC" filter add dev lo protocol ip parent 1:0 prio 4 u32 \
    match ip sport 7005 0xffff flowid 1:3
fi

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
TRNM_ONLINE_COMMAND_JOURNAL_PATH="$EVIDENCE/host-journal/journal.json" \
TRNM_ONLINE_FRAME_TIMING_PATH="$EVIDENCE/host-frame-timing.json" \
  "$BIN" >"$EVIDENCE/host.log" 2>&1 &
HOST_PID=$!
HOST_WINDOW="$(wait_for_window "$HOST_PID")"
"$ROOT_DIR/scripts/x11_window_move.py" "$HOST_WINDOW" 0 0
sleep 1

capture "$HOST_WINDOW" "$EVIDENCE/host-attached.png"
"$ROOT_DIR/scripts/x11_key_inject.py" "$HOST_WINDOW" q
for _ in $(seq 1 30); do
  host_commands="$(cex_psql_stdin -Atc "select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$HOST_PLAYER'")"
  [[ "$host_commands" -ge 1 ]] && break
  sleep 1
done
[[ "${host_commands:-0}" -ge 1 ]]
capture "$HOST_WINDOW" "$EVIDENCE/host-command-ack.png"
wait_for_frame_timing "$EVIDENCE/host-frame-timing.json"
kill "$HOST_PID" >/dev/null 2>&1 || true
wait "$HOST_PID" 2>/dev/null || true
HOST_PID=""

TRNM_CAMPAIGN_SAVE_PATH="$EVIDENCE/guest-save/campaign.json" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" TRNM_ONLINE_MATCH_ID="$MATCH_ID" \
TRNM_CEX_ACTOR_ID="$GUEST_PLAYER" TRNM_CEX_ACCOUNT_ID="$GUEST_ACCOUNT" \
TRNM_CEX_PLAYER_SESSION="$GUEST_SESSION" \
TRNM_ONLINE_COMMAND_JOURNAL_PATH="$EVIDENCE/guest-journal/journal.json" \
TRNM_ONLINE_FRAME_TIMING_PATH="$EVIDENCE/guest-frame-timing.json" \
  "$BIN" >"$EVIDENCE/guest.log" 2>&1 &
GUEST_PID=$!
GUEST_WINDOW="$(wait_for_window "$GUEST_PID")"
"$ROOT_DIR/scripts/x11_window_move.py" "$GUEST_WINDOW" 0 0
sleep 1
capture "$GUEST_WINDOW" "$EVIDENCE/guest-attached.png"
"$ROOT_DIR/scripts/x11_key_inject.py" "$GUEST_WINDOW" q
for _ in $(seq 1 30); do
  guest_commands="$(cex_psql_stdin -Atc "select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$GUEST_PLAYER'")"
  [[ "$guest_commands" -ge 1 ]] && break
  sleep 1
done
[[ "${guest_commands:-0}" -ge 1 ]]
capture "$GUEST_WINDOW" "$EVIDENCE/guest-command-ack.png"
wait_for_frame_timing "$EVIDENCE/guest-frame-timing.json"
host_frame_timing="$(jq -c . "$EVIDENCE/host-frame-timing.json")"
guest_frame_timing="$(jq -c . "$EVIDENCE/guest-frame-timing.json")"
jq -e '.frame_count >= 10 and .main_thread_updates_over_100ms == 0 and
  .max_main_thread_update_ms <= 100 and .network_requests_on_render_thread == false' \
  >/dev/null <<<"$host_frame_timing"
jq -e '.frame_count >= 10 and .main_thread_updates_over_100ms == 0 and
  .max_main_thread_update_ms <= 100 and .network_requests_on_render_thread == false' \
  >/dev/null <<<"$guest_frame_timing"

for journal in "$EVIDENCE/host-journal/journal.json" \
  "$EVIDENCE/guest-journal/journal.json"; do
  for _ in $(seq 1 40); do
    [[ -s "$journal" ]] \
      && jq -e '.pending_exact_attempts | length == 0' "$journal" >/dev/null 2>&1 \
      && break
    sleep 0.25
  done
  [[ -s "$journal" ]]
  [[ "$(stat -c '%a' "$(dirname "$journal")")" == "700" ]]
  [[ "$(stat -c '%a' "$journal")" == "600" ]]
  [[ "$(stat -c '%a' "$(dirname "$journal")/.$(basename "$journal").lock")" == "600" ]]
  jq -e '.contract_version == "trnm_online_command_journal_v1" and
    (.pending_exact_attempts | length == 0) and
    (.rejected_exact_attempts | length == 0)' "$journal" >/dev/null
done
! grep -Fq -- "$HOST_SESSION" "$EVIDENCE/host-journal/journal.json"
! grep -Fq -- "$GUEST_SESSION" "$EVIDENCE/guest-journal/journal.json"
journal_evidence="$(jq -cn \
  --arg host_directory_mode "$(stat -c '%a' "$EVIDENCE/host-journal")" \
  --arg host_file_mode "$(stat -c '%a' "$EVIDENCE/host-journal/journal.json")" \
  --arg guest_directory_mode "$(stat -c '%a' "$EVIDENCE/guest-journal")" \
  --arg guest_file_mode "$(stat -c '%a' "$EVIDENCE/guest-journal/journal.json")" \
  '{host_directory_mode:$host_directory_mode,host_file_mode:$host_file_mode,
    guest_directory_mode:$guest_directory_mode,guest_file_mode:$guest_file_mode,
    pending_after_ack:0,rejected_after_ack:0,credentials_absent:true}')"

database="$(cex_psql_stdin -Atc "select json_build_object(
  'lobby_status',(select status from trnm_online_lobbies where lobby_id = '$LOBBY_ID'::uuid),
  'allocations',(select count(*) from trnm_online_matchmaking_allocations where lobby_id = '$LOBBY_ID'::uuid and match_id = '$MATCH_ID'::uuid),
  'members',(select count(*) from trnm_online_match_members where match_id = '$MATCH_ID'::uuid),
  'host_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$HOST_PLAYER'),
  'guest_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$GUEST_PLAYER'),
  'fingerprinted_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and length(request_hash) = 64),
  'distinct_control_sets',(select count(distinct controlled_unit_ids) from trnm_online_match_members where match_id = '$MATCH_ID'::uuid)
)" | jq -c .)"
jq -e '.lobby_status == "matched" and .allocations == 1 and
  .members == 2 and .host_commands >= 1 and .guest_commands >= 1 and
  .fingerprinted_commands == (.host_commands + .guest_commands) and
  .distinct_control_sets == 2' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --arg match_id "$MATCH_ID" --arg evidence "$EVIDENCE" \
  --arg host_window "$HOST_WINDOW" --arg guest_window "$GUEST_WINDOW" \
  --arg authority_protocol "$contract" --arg authority_build "$build" \
  --arg product_protocol "$product_contract" --arg product_build "$product_build" \
  --arg chaos_rtt_ms "$CHAOS_RTT_MS" --arg chaos_loss_percent "$CHAOS_LOSS_PERCENT" \
  --argjson database "$database" --argjson host_frame_timing "$host_frame_timing" \
  --argjson guest_frame_timing "$guest_frame_timing" \
  --argjson journal_evidence "$journal_evidence" \
  '{status:"passed",run_id:$run_id,match_id:$match_id,evidence:$evidence,
    authority_protocol:$authority_protocol,authority_build:$authority_build,
    product_protocol:$product_protocol,product_build:$product_build,
    network_chaos:{rtt_ms:($chaos_rtt_ms | if length == 0 then 0 else tonumber end),loss_percent:($chaos_loss_percent|tonumber)},
    native_x11_clients:2,distinct_windows:($host_window != $guest_window),
    client_execution_model:"sequential_on_single_evidence_host_models_separate_player_devices",
    closed_alpha_product_lobby_flow:true,
    server_authoritative_commands:true,database:$database,
    durable_command_journal:$journal_evidence,
    host_frame_timing:$host_frame_timing,guest_frame_timing:$guest_frame_timing,
    boundary:"automated two-process native attach/input smoke measured sequentially on one evidence host; not a human multiplayer session"}' \
  | tee "$EVIDENCE/report.json"
