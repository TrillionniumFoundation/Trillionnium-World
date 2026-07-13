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
RUN_ID="online-operations-native-login-$(date +%s)-${RANDOM}"
PLAYER="opslogin$(date +%s)${RANDOM}player"
CREDENTIAL="credential$(date +%s)${RANDOM}012345678901234567890123"
DEVICE="$RUN_ID-device"
EVIDENCE="$ROOT_DIR/acceptance/online-operations-v1-native-login/$RUN_ID"
BIN="$ROOT_DIR/target/release/trnm-online-product"
DISPLAY="${TRNM_OPERATIONS_LOGIN_DISPLAY:-:97}"
XVFB_PID=""
PRODUCT_PID=""
mkdir -p "$EVIDENCE"

cleanup() {
  local status=$?
  [[ -z "$PRODUCT_PID" ]] || kill "$PRODUCT_PID" >/dev/null 2>&1 || true
  [[ -z "$XVFB_PID" ]] || kill "$XVFB_PID" >/dev/null 2>&1 || true
  key_id="$(keyctl search @u user "trnm-online-product:$PLAYER" 2>/dev/null || true)"
  [[ -z "$key_id" ]] || keyctl unlink "$key_id" @u >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

invite="$(curl -fsS "$LEDGER_URL/v1/trnm/product/registration-invites" \
  -H "x-admin-token: $ADMIN_TOKEN" -H 'content-type: application/json' \
  --data-binary '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
identity="$(curl -fsS "$LEDGER_URL/v1/trnm/product/register" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg player "$PLAYER" --arg credential "$CREDENTIAL" --arg invite "$invite" \
    '{player_id:$player,credential:$credential,invite_code:$invite}')")"
ACCOUNT="$(jq -er .account_id <<<"$identity")"

export DISPLAY
Xvfb "$DISPLAY" -screen 0 1280x720x24 -nolisten tcp >"$EVIDENCE/xvfb.log" 2>&1 &
XVFB_PID=$!
for _ in $(seq 1 30); do xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break; sleep 1; done
xdpyinfo -display "$DISPLAY" >/dev/null
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY TRNM_PRODUCT_PLAYER_ID TRNM_PRODUCT_CREDENTIAL TRNM_CEX_ENTRY_TOKEN

window_for_pid() {
  local pid="$1" id actual name
  while read -r id; do
    [[ -n "$id" ]] || continue
    actual="$(xprop -id "$id" _NET_WM_PID 2>/dev/null | awk -F' = ' '{print $2}')"
    name="$(xprop -id "$id" WM_NAME 2>/dev/null || true)"
    if [[ "$actual" == "$pid" && "$name" == *'Online Product v2'* ]]; then
      printf '%s\n' "$id"
      return 0
    fi
  done < <(xwininfo -root -tree 2>/dev/null | awk '/0x[0-9a-f]+/ {print $1}')
  return 1
}

wait_window() {
  local pid="$1" id=""
  for _ in $(seq 1 60); do
    id="$(window_for_pid "$pid" || true)"
    [[ -z "$id" ]] || { printf '%s\n' "$id"; return 0; }
    sleep 1
  done
  return 1
}

wait_state() {
  local path="$1" state="$2"
  for _ in $(seq 1 90); do
    [[ -s "$path" ]] && [[ "$(jq -r .state "$path")" == "$state" ]] && return 0
    sleep 1
  done
  [[ -s "$path" ]] && cat "$path" >&2
  return 1
}

capture_rendered() {
  local window="$1" output="$2" pixels body border title
  for _ in $(seq 1 20); do
    xwd -silent -id "$window" 2>/dev/null | xwdtopnm 2>/dev/null | pnmtopng >"$output"
    pixels="$(pngtopnm "$output" 2>/dev/null | od -An -v -tu1 |
      awk '{for(i=1;i<=NF;i++) if($i>=16)n++} END{print n+0}')"
    body="$(pngtopnm "$output" 2>/dev/null | ppmhist 2>/dev/null |
      awk '$1==9 && $2==18 && $3==16 {print $5}')"
    border="$(pngtopnm "$output" 2>/dev/null | ppmhist 2>/dev/null |
      awk '$1==64 && $2==133 && $3==107 {print $5}')"
    title="$(pngtopnm "$output" 2>/dev/null | ppmhist 2>/dev/null |
      awk '$1==242 && $2==209 && $3==107 {print $5}')"
    if (( pixels >= 5000 && ${body:-0} >= 300000 && ${border:-0} >= 3000 && ${title:-0} >= 2000 )); then
      return 0
    fi
    sleep 1
  done
  return 1
}

inject_text() {
  local window="$1" value="$2" keys=()
  while IFS= read -r character; do keys+=("$character"); done < <(fold -w1 <<<"$value")
  TRNM_X11_KEY_DELAY=0.03 "$ROOT_DIR/scripts/x11_key_inject.py" "$window" "${keys[@]}"
}

TRNM_PRODUCT_DEVICE_ID="$DEVICE" TRNM_ONLINE_SLOT_KEY="ops-login-${RANDOM}" \
TRNM_CEX_LEDGER_URL="$LEDGER_URL" TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" \
TRNM_PRODUCT_EVIDENCE_PATH="$EVIDENCE/typed-state.json" \
  "$BIN" >"$EVIDENCE/typed.log" 2>&1 &
PRODUCT_PID=$!
TYPED_WINDOW="$(wait_window "$PRODUCT_PID")"
inject_text "$TYPED_WINDOW" "$PLAYER"
"$ROOT_DIR/scripts/x11_key_inject.py" "$TYPED_WINDOW" tab
inject_text "$TYPED_WINDOW" "$CREDENTIAL"
"$ROOT_DIR/scripts/x11_key_inject.py" "$TYPED_WINDOW" f6 f1 f2
wait_state "$EVIDENCE/typed-state.json" LOBBY
jq -e --arg player "$PLAYER" --arg account "$ACCOUNT" \
  '.player_id == $player and .account_id == $account and .text_login_ready == true and
   .credential_source == "Linux kernel user keyring" and .season_id == "season-2026-prealpha-1"' \
  "$EVIDENCE/typed-state.json" >/dev/null
key_id="$(keyctl search @u user "trnm-online-product:$PLAYER")"
[[ "$key_id" =~ ^[0-9]+$ ]]
sleep 3
capture_rendered "$TYPED_WINDOW" "$EVIDENCE/typed-login.png"
kill "$PRODUCT_PID"
wait "$PRODUCT_PID" 2>/dev/null || true
PRODUCT_PID=""
kill "$XVFB_PID"
wait "$XVFB_PID" 2>/dev/null || true
XVFB_PID=""
Xvfb "$DISPLAY" -screen 0 1280x720x24 -nolisten tcp >"$EVIDENCE/recovered-xvfb.log" 2>&1 &
XVFB_PID=$!
for _ in $(seq 1 30); do xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break; sleep 1; done
xdpyinfo -display "$DISPLAY" >/dev/null

TRNM_PRODUCT_PLAYER_ID="$PLAYER" TRNM_PRODUCT_DEVICE_ID="$DEVICE-recovered" \
TRNM_ONLINE_SLOT_KEY="ops-recover-${RANDOM}" TRNM_CEX_LEDGER_URL="$LEDGER_URL" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" \
TRNM_PRODUCT_EVIDENCE_PATH="$EVIDENCE/recovered-state.json" \
  "$BIN" >"$EVIDENCE/recovered.log" 2>&1 &
PRODUCT_PID=$!
RECOVERED_WINDOW="$(wait_window "$PRODUCT_PID")"
"$ROOT_DIR/scripts/x11_key_inject.py" "$RECOVERED_WINDOW" f1 f2
wait_state "$EVIDENCE/recovered-state.json" LOBBY
jq -e --arg player "$PLAYER" '.player_id == $player and
  .credential_source == "Linux kernel user keyring" and .text_login_ready == true' \
  "$EVIDENCE/recovered-state.json" >/dev/null
sleep 3
capture_rendered "$RECOVERED_WINDOW" "$EVIDENCE/recovered-login.png"
cp "$EVIDENCE/recovered-state.json" "$EVIDENCE/recovered-before-forget.json"
"$ROOT_DIR/scripts/x11_key_inject.py" "$RECOVERED_WINDOW" f8
for _ in $(seq 1 20); do
  keyctl search @u user "trnm-online-product:$PLAYER" >/dev/null 2>&1 || break
  sleep 0.25
done
if keyctl search @u user "trnm-online-product:$PLAYER" >/dev/null 2>&1; then
  echo "kernel keyring credential was not removed" >&2
  exit 1
fi

jq -n --arg run_id "$RUN_ID" --arg player "$PLAYER" --arg account "$ACCOUNT" --arg evidence "$EVIDENCE" \
  '{status:"passed",run_id:$run_id,player_id:$player,account_id:$account,evidence:$evidence,
    native_text_entry:true,credential_masked:true,kernel_user_keyring_store:true,
    restart_recovery_without_credential_environment:true,kernel_keyring_forget:true,
    active_season_loaded:true,rendered_frames:2,
    boundary:"automated native text/keyring flow; not a real-human usability session"}' \
  | tee "$EVIDENCE/report.json"
