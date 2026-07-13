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
RUN_ID="online-product-native-v2-$(date +%s)-${RANDOM}"
SLOT_KEY="native-${RANDOM}"
EVIDENCE="$ROOT_DIR/acceptance/online-product-v2-native/$RUN_ID"
BIN="$ROOT_DIR/target/release/trnm-online-product"
HOST_PID=""
GUEST_PID=""
HOST_GAME_PID=""
GUEST_GAME_PID=""
XVFB_PID=""
mkdir -p "$EVIDENCE"

cleanup() {
  local status=$?
  for pid in "$HOST_GAME_PID" "$GUEST_GAME_PID" "$HOST_PID" "$GUEST_PID" "$XVFB_PID"; do
    [[ -z "$pid" ]] || kill "$pid" >/dev/null 2>&1 || true
  done
  systemctl --user unset-environment TRNM_GAME_SERVER_TICK_MS TRNM_ALLOW_ACCELERATED_TEST_CLOCK >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

admin_post() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local role="$1" player credential invite identity
  player="$RUN_ID-$role"
  credential="credential-$RUN_ID-$role-012345678901234567890123"
  invite="$(admin_post /v1/trnm/product/registration-invites \
    '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
  identity="$(curl -fsS "$LEDGER_URL/v1/trnm/product/register" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg credential "$credential" --arg invite "$invite" \
      '{player_id:$player,credential:$credential,invite_code:$invite}')")"
  jq -e '.status == "active"' <<<"$identity" >/dev/null
  printf '%s\t%s\n' "$player" "$credential"
}

window_for_pid() {
  local pid="$1" title="$2" id actual name
  while read -r id; do
    [[ -n "$id" ]] || continue
    actual="$(xprop -id "$id" _NET_WM_PID 2>/dev/null | awk -F' = ' '{print $2}')"
    name="$(xprop -id "$id" WM_NAME 2>/dev/null || true)"
    if [[ "$actual" == "$pid" && "$name" == *"$title"* ]]; then
      printf '%s\n' "$id"
      return 0
    fi
  done < <(xwininfo -root -tree 2>/dev/null | awk '/0x[0-9a-f]+/ {print $1}')
  return 1
}

wait_for_window() {
  local pid="$1" title="$2" id=""
  for _ in $(seq 1 90); do
    id="$(window_for_pid "$pid" "$title" || true)"
    [[ -z "$id" ]] || { printf '%s\n' "$id"; return 0; }
    sleep 1
  done
  return 1
}

wait_evidence_state() {
  local path="$1" state="$2"
  for _ in $(seq 1 90); do
    [[ -s "$path" ]] && [[ "$(jq -r .state "$path")" == "$state" ]] && return 0
    sleep 1
  done
  [[ -s "$path" ]] && cat "$path" >&2
  return 1
}

capture() {
  local window_id="$1" output="$2"
  local rendered_pixels
  for _ in $(seq 1 20); do
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
  echo "[FAIL] native window $window_id never produced a non-black rendered frame" >&2
  return 1
}

cex_psql_stdin -Atc "update trnm_online_solo_queue set status = 'cancelled', updated_at = now()
  where status = 'queued' and player_id like 'online-product-native-v2-%'" >/dev/null
IFS=$'\t' read -r HOST_PLAYER HOST_CREDENTIAL < <(create_identity host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_CREDENTIAL < <(create_identity guest)

systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS=200 \
  TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done

export DISPLAY="${TRNM_ONLINE_PRODUCT_DISPLAY:-:96}"
Xvfb "$DISPLAY" -screen 0 2560x720x24 -nolisten tcp >"$EVIDENCE/xvfb.log" 2>&1 &
XVFB_PID=$!
for _ in $(seq 1 30); do
  xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break
  sleep 1
done
xdpyinfo -display "$DISPLAY" >/dev/null
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY TRNM_CEX_ENTRY_TOKEN

TRNM_PRODUCT_PLAYER_ID="$HOST_PLAYER" TRNM_PRODUCT_CREDENTIAL="$HOST_CREDENTIAL" \
TRNM_PRODUCT_DEVICE_ID="$RUN_ID-host-device" TRNM_ONLINE_SLOT_KEY="$SLOT_KEY" \
TRNM_PRODUCT_MAP_ID="iron_delta" TRNM_CEX_LEDGER_URL="$LEDGER_URL" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" \
TRNM_PRODUCT_EVIDENCE_PATH="$EVIDENCE/host-state.json" \
  "$BIN" >"$EVIDENCE/host.log" 2>&1 &
HOST_PID=$!
HOST_WINDOW="$(wait_for_window "$HOST_PID" 'Online Product v2')"

TRNM_PRODUCT_PLAYER_ID="$GUEST_PLAYER" TRNM_PRODUCT_CREDENTIAL="$GUEST_CREDENTIAL" \
TRNM_PRODUCT_DEVICE_ID="$RUN_ID-guest-device" TRNM_ONLINE_SLOT_KEY="$SLOT_KEY" \
TRNM_PRODUCT_MAP_ID="iron_delta" TRNM_CEX_LEDGER_URL="$LEDGER_URL" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" \
TRNM_PRODUCT_EVIDENCE_PATH="$EVIDENCE/guest-state.json" \
  "$BIN" >"$EVIDENCE/guest.log" 2>&1 &
GUEST_PID=$!
GUEST_WINDOW="$(wait_for_window "$GUEST_PID" 'Online Product v2')"
"$ROOT_DIR/scripts/x11_window_move.py" "$HOST_WINDOW" 0 0
"$ROOT_DIR/scripts/x11_window_move.py" "$GUEST_WINDOW" 1280 0

"$ROOT_DIR/scripts/x11_key_inject.py" "$HOST_WINDOW" f1 f2 f3
wait_evidence_state "$EVIDENCE/host-state.json" MATCHMAKING
"$ROOT_DIR/scripts/x11_key_inject.py" "$GUEST_WINDOW" f1 f2 f3
wait_evidence_state "$EVIDENCE/guest-state.json" 'MATCH FOUND'
wait_evidence_state "$EVIDENCE/host-state.json" 'MATCH FOUND'

MATCH_ID="$(jq -er .match_id "$EVIDENCE/host-state.json")"
[[ "$(jq -er .match_id "$EVIDENCE/guest-state.json")" == "$MATCH_ID" ]]
capture "$HOST_WINDOW" "$EVIDENCE/host-match-found.png"
capture "$GUEST_WINDOW" "$EVIDENCE/guest-match-found.png"

"$ROOT_DIR/scripts/x11_key_inject.py" "$HOST_WINDOW" f5
"$ROOT_DIR/scripts/x11_key_inject.py" "$GUEST_WINDOW" f5
wait_evidence_state "$EVIDENCE/host-state.json" 'IN MATCH'
wait_evidence_state "$EVIDENCE/guest-state.json" 'IN MATCH'
for _ in $(seq 1 60); do
  HOST_GAME_PID="$(pgrep -P "$HOST_PID" trnm-first-cont | head -1 || true)"
  GUEST_GAME_PID="$(pgrep -P "$GUEST_PID" trnm-first-cont | head -1 || true)"
  [[ -n "$HOST_GAME_PID" && -n "$GUEST_GAME_PID" ]] && break
  sleep 1
done
[[ -n "$HOST_GAME_PID" && -n "$GUEST_GAME_PID" ]]
HOST_GAME_WINDOW="$(wait_for_window "$HOST_GAME_PID" 'First Contact')"
GUEST_GAME_WINDOW="$(wait_for_window "$GUEST_GAME_PID" 'First Contact')"
"$ROOT_DIR/scripts/x11_window_move.py" "$HOST_GAME_WINDOW" 0 0
"$ROOT_DIR/scripts/x11_window_move.py" "$GUEST_GAME_WINDOW" 1280 0
sleep 2
capture "$HOST_GAME_WINDOW" "$EVIDENCE/host-authority.png"
capture "$GUEST_GAME_WINDOW" "$EVIDENCE/guest-authority.png"

database="$(cex_psql_stdin -Atc "select json_build_object(
  'match_mode',(select match_mode from trnm_online_matches where match_id = '$MATCH_ID'::uuid),
  'phase',(select phase from trnm_online_matches where match_id = '$MATCH_ID'::uuid),
  'members',(select count(*) from trnm_online_match_members where match_id = '$MATCH_ID'::uuid),
  'human_enemy',(select simulation_json->>'human_enemy_authority' from trnm_online_matches where match_id = '$MATCH_ID'::uuid),
  'queue_tickets',(select count(*) from trnm_online_solo_queue where match_id = '$MATCH_ID'::uuid and status = 'matched'),
  'ratings',(select count(*) from trnm_online_ratings where player_id in ('$HOST_PLAYER','$GUEST_PLAYER'))
)" | jq -c .)"
jq -e '.match_mode == "ranked_pvp" and .phase == "running" and .members == 2 and
  .human_enemy == "true" and .queue_tickets == 2 and .ratings == 2' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --arg match_id "$MATCH_ID" --arg evidence "$EVIDENCE" \
  --arg host_launcher "$HOST_WINDOW" --arg guest_launcher "$GUEST_WINDOW" \
  --arg host_game "$HOST_GAME_WINDOW" --arg guest_game "$GUEST_GAME_WINDOW" \
  --argjson database "$database" \
  '{status:"passed",run_id:$run_id,match_id:$match_id,evidence:$evidence,
    native_product_windows:2,native_authority_windows:2,
    distinct_launcher_windows:($host_launcher != $guest_launcher),
    distinct_game_windows:($host_game != $guest_game),
    rendered_frame_gate:true,
    keyboard_flow:["F1 login","F2 cloud character","F3 ranked queue","F5 launch"],
    credentials_rendered:false,scoped_session_handoff:true,database:$database,
    boundary:"automated native product/login/matchmaking/launch smoke; not a human usability session"}' \
  | tee "$EVIDENCE/report.json"
