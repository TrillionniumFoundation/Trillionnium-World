#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT="$($ROOT_DIR/scripts/check-trnm-online-operations-v2-e2e.sh)"
PRODUCT_RUN="$(jq -er .product_run <<<"$REPORT")"
MATCH_ID="$(jq -er .match_id <<<"$REPORT")"
PLAYER="$PRODUCT_RUN-host"
CREDENTIAL="credential-$PRODUCT_RUN-host-012345678901234567890123"
RUN_ID="online-operations-native-replay-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/acceptance/online-operations-v2-native-replay/$RUN_ID"
BIN="$ROOT_DIR/target/release/trnm-online-product"
DISPLAY="${TRNM_OPERATIONS_REPLAY_DISPLAY:-:96}"
XVFB_PID=""
PRODUCT_PID=""
mkdir -p "$EVIDENCE"

cleanup() {
  local status=$?
  [[ -z "$PRODUCT_PID" ]] || kill "$PRODUCT_PID" >/dev/null 2>&1 || true
  [[ -z "$XVFB_PID" ]] || kill "$XVFB_PID" >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

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
    sleep 0.5
  done
  return 1
}

wait_state() {
  local state="$1"
  for _ in $(seq 1 120); do
    [[ -s "$EVIDENCE/state.json" ]] &&
      [[ "$(jq -r .state "$EVIDENCE/state.json")" == "$state" ]] && return 0
    sleep 0.25
  done
  [[ -s "$EVIDENCE/state.json" ]] && cat "$EVIDENCE/state.json" >&2
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
    sleep 0.5
  done
  return 1
}

export DISPLAY
Xvfb "$DISPLAY" -screen 0 1280x720x24 -nolisten tcp >"$EVIDENCE/xvfb.log" 2>&1 &
XVFB_PID=$!
for _ in $(seq 1 30); do xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break; sleep 0.5; done
xdpyinfo -display "$DISPLAY" >/dev/null
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY TRNM_CEX_ENTRY_TOKEN

TRNM_PRODUCT_PLAYER_ID="$PLAYER" TRNM_PRODUCT_CREDENTIAL="$CREDENTIAL" \
TRNM_PRODUCT_DEVICE_ID="$RUN_ID-device" TRNM_ONLINE_SLOT_KEY="ops-replay-${RANDOM}" \
TRNM_PRODUCT_EVIDENCE_PATH="$EVIDENCE/state.json" \
  "$BIN" >"$EVIDENCE/product.log" 2>&1 &
PRODUCT_PID=$!
WINDOW="$(wait_window "$PRODUCT_PID")"
"$ROOT_DIR/scripts/x11_key_inject.py" "$WINDOW" f1 f2
wait_state LOBBY
"$ROOT_DIR/scripts/x11_key_inject.py" "$WINDOW" f9
wait_state "REPLAY READY"
jq -e --arg player "$PLAYER" --arg match "$MATCH_ID" \
  '.contract == "trnm_native_online_operations_v2" and .player_id == $player and
   .match_id == $match and .replay_integrity_verified == true and
   .replay_frame_count >= 2 and (.replay_hash | length) == 64' \
  "$EVIDENCE/state.json" >/dev/null
sleep 2
capture_rendered "$WINDOW" "$EVIDENCE/replay-ready.png"

jq -n --arg run_id "$RUN_ID" --arg product_run "$PRODUCT_RUN" --arg player "$PLAYER" \
  --arg match_id "$MATCH_ID" --arg evidence "$EVIDENCE" \
  --arg replay_hash "$(jq -er .replay_hash "$EVIDENCE/state.json")" \
  --argjson replay_frames "$(jq -er .replay_frame_count "$EVIDENCE/state.json")" \
  '{status:"passed",run_id:$run_id,product_run:$product_run,player_id:$player,
    match_id:$match_id,replay_hash:$replay_hash,replay_frames:$replay_frames,
    native_f9_replay:true,integrity_verified:true,rendered_frames:1,evidence:$evidence,
    boundary:"automated native replay inspection; not human spectating/usability evidence"}' \
  | tee "$EVIDENCE/report.json"
