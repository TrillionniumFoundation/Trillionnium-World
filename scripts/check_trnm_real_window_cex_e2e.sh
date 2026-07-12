#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_PROJECT_ROOT="${CEX_PROJECT_ROOT:-$(cd -- "$PROJECT_ROOT/../CEX" && pwd)}"
# shellcheck source=/dev/null
source "$CEX_PROJECT_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${LEDGER_BASE_URL:-http://127.0.0.1:7002}"
CONSUMER_URL="${CONSUMER_ENTRY_BASE_URL:-http://127.0.0.1:8090}"
ADMIN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?ledger admin token required}}"
ORG="00000000-0000-0000-0000-00000000ce01"
RUN_ID="real-window-$(date +%s)-${RANDOM}"
EVIDENCE="$PROJECT_ROOT/acceptance/native-window/$RUN_ID"
SAVE_DIR="$EVIDENCE/save"
mkdir -p "$SAVE_DIR"
APP_PID=""
SERVICE_WAS_ACTIVE=false
CURRENT_PHASE=bootstrap

cleanup() {
  status=$?
  if [[ "$status" -ne 0 ]]; then
    echo "real-window CEX E2E failed in phase $CURRENT_PHASE" >&2
  fi
  [[ -z "$APP_PID" ]] || kill "$APP_PID" >/dev/null 2>&1 || true
  if [[ "$SERVICE_WAS_ACTIVE" == true ]]; then
    systemctl --user start trillionnium-bevy-playtest.service >/dev/null 2>&1 || true
  fi
  exit "$status"
}
trap cleanup EXIT

post_ledger() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN" \
    -H 'x-trnm-system-operation: true' -H 'content-type: application/json' --data-binary "$2"
}

account="$(post_ledger /v1/accounts "$(jq -cn --arg org "$ORG" \
  '{org_id:$org,account_type:"real-window-player",currency_unit:"credit",initial_balance:200}')" | jq -er .account_id)"
market="$(post_ledger /v1/accounts "$(jq -cn --arg org "$ORG" \
  '{org_id:$org,account_type:"real-window-market",currency_unit:"credit",initial_balance:0}')" | jq -er .account_id)"
player="player-$RUN_ID"
recovery="recovery-$RUN_ID-012345678901234567890123"
post_ledger /v1/trnm/identity/register "$(jq -cn --arg p "$player" --arg a "$account" --arg r "$recovery" \
  '{player_id:$p,account_id:$a,recovery_key:$r}')" >/dev/null
session="$(curl -fsS "$LEDGER_URL/v1/trnm/identity/session" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg p "$player" --arg r "$recovery" \
  '{player_id:$p,recovery_key:$r,device_id:"rendered-window-e2e"}')" | jq -er .session_token)"

binary="$PROJECT_ROOT/target/release/trnm-first-contact"
[[ -x "$binary" ]] || cargo build --manifest-path "$PROJECT_ROOT/trillionnium/Cargo.toml" \
  --release -p trnm-first-contact
chmod +x "$PROJECT_ROOT/scripts/x11_key_inject.py"

if systemctl --user is-active --quiet trillionnium-bevy-playtest.service; then
  SERVICE_WAS_ACTIVE=true
  systemctl --user stop trillionnium-bevy-playtest.service
fi

export TRNM_CAMPAIGN_SAVE_PATH="$SAVE_DIR/campaign.json"
export TRNM_CEX_BASE_URL="$CONSUMER_URL"
export TRNM_CEX_PLAYER_SESSION="$session"
export TRNM_CEX_ACCOUNT_ID="$account"
export TRNM_CEX_ACTOR_ID="$player"
export TRNM_CEX_MARKET_ACCOUNT_ID="$market"
export DISPLAY="${DISPLAY:-:0}"
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY TRNM_CEX_ENTRY_TOKEN
"$binary" >"$EVIDENCE/client.log" 2>&1 &
APP_PID=$!

window_id=""
for _ in $(seq 1 60); do
  window_id="$(xwininfo -root -tree 2>/dev/null | awk \
    '/\("trnm-first-contact" "trnm-first-contact"\)/ {print $1; exit}')"
  [[ -n "$window_id" ]] && break
  sleep 1
done
[[ -n "$window_id" ]]

capture() {
  xwd -silent -id "$window_id" | xwdtopnm 2>/dev/null | pnmtopng >"$EVIDENCE/$1.png"
  test -s "$EVIDENCE/$1.png"
}
keys() {
  "$PROJECT_ROOT/scripts/x11_key_inject.py" "$window_id" "$@"
}

CURRENT_PHASE=title-render
capture 01-title
keys 1
sleep 1
keys n
sleep 1
keys enter
sleep 1
capture 02-character-confirmed
CURRENT_PHASE=market-navigation
keys 8
sleep 1
CURRENT_PHASE=player-session-bind
for _ in $(seq 1 5); do
  keys ctrl+f7
  sleep 1
  jq -e '.economy_mode == "cex_connected"' "$SAVE_DIR/campaign.json" >/dev/null 2>&1 && break
done
capture 02b-after-bind
jq -e '.room == "market_wind_pavilion" and .economy_mode == "cex_connected"' \
  "$SAVE_DIR/campaign.json" >/dev/null

for _ in $(seq 1 22); do
  [[ "$(jq -r .selected_shop_item_index "$SAVE_DIR/campaign.json")" == 9 ]] && break
  keys f11
  sleep 1
done
[[ "$(jq -r .selected_shop_item_index "$SAVE_DIR/campaign.json")" == 9 ]]
CURRENT_PHASE=tradeable-purchase
for _ in $(seq 1 5); do
  keys ctrl+shift+f7
  sleep 1
  if jq -e '.pending_tradeable_purchases[-1].stage == "consumed"' \
    "$SAVE_DIR/campaign.json" >/dev/null 2>&1; then
    break
  fi
done
jq -e '.pending_tradeable_purchases[-1].stage == "consumed"' \
  "$SAVE_DIR/campaign.json" >/dev/null
capture 03-purchased

CURRENT_PHASE=service-restart
systemctl --user restart cex-trnm-ledger.service cex-trnm-consumer.service
for _ in $(seq 1 60); do
  curl -fsS "$LEDGER_URL/v1/trnm/economy/readiness" >/dev/null 2>&1 \
    && curl -fsS "$CONSUMER_URL/v1/trillionnium/economy/adapters/readiness" >/dev/null 2>&1 \
    && break
  sleep 1
done
keys ctrl+f7
sleep 1
capture 04-after-service-restart
CURRENT_PHASE=client-cancel
for _ in $(seq 1 5); do
  keys ctrl+shift+f8
  sleep 1
  jq -e '.pending_tradeable_purchases[-1].stage == "refunded"' \
    "$SAVE_DIR/campaign.json" >/dev/null 2>&1 && break
done
capture 05-cancelled

CURRENT_PHASE=state-verification
jq -e '.pending_tradeable_purchases[-1].stage == "refunded" and
  .wallet_snapshot.available_credits == 200 and .wallet_snapshot.reserved_credits == 0' \
  "$SAVE_DIR/campaign.json" >/dev/null
item_id="$(jq -er '.pending_tradeable_purchases[-1].item_id' "$SAVE_DIR/campaign.json")"
jq -e --arg item "$item_id" \
  '([.progression.inventory[] | select(.item_id == $item) | .quantity] | add // 0) == 0' \
  "$SAVE_DIR/campaign.json" >/dev/null

for image in "$EVIDENCE"/*.png; do
  ffprobe -v error -select_streams v:0 -show_entries stream=width,height \
    -of csv=p=0 "$image"
done >"$EVIDENCE/render-dimensions.txt"
[[ "$(sort -u "$EVIDENCE/render-dimensions.txt" | wc -l)" == 1 ]]
sha256sum "$EVIDENCE"/*.png >"$EVIDENCE/render-sha256.txt"
[[ "$(cut -d' ' -f1 "$EVIDENCE/render-sha256.txt" | sort -u | wc -l)" -ge 4 ]]

jq -n --arg run_id "$RUN_ID" --arg evidence "$EVIDENCE" --arg account "$account" \
  --arg market "$market" --arg item "$item_id" \
  '{status:"passed",run_id:$run_id,evidence:$evidence,player_account:$account,
    market_account:$market,item_id:$item,title_to_character_to_market_navigation:true,
    rendered_x11_frames:6,service_restart_reconciled:true,
    purchase_and_inventory_visible_then_cancelled:true,
    boundary:"automated real X11 render/input evidence; not a human usability session"}' \
  >"$EVIDENCE/report.json"
cat "$EVIDENCE/report.json"
