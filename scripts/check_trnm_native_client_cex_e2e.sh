#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_PROJECT_ROOT="${CEX_PROJECT_ROOT:-$(cd -- "$PROJECT_ROOT/../CEX" && pwd)}"
# shellcheck source=/dev/null
source "$CEX_PROJECT_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${LEDGER_BASE_URL:-http://127.0.0.1:7002}"
CONSUMER_URL="${CONSUMER_ENTRY_BASE_URL:-http://127.0.0.1:8090}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity/ledger admin token required}}"
ENTRY_TOKEN="${CONSUMER_ENTRY_INGRESS_TOKEN:-$ADMIN_TOKEN}"
ORG_ID="00000000-0000-0000-0000-00000000ce01"
RUN_ID="native-client-$(date +%s)-${RANDOM}"
WORK_DIR="$(mktemp -d /tmp/trnm-native-client-cex.XXXXXX)"
CURRENT_PHASE="bootstrap"
trap 'status=$?; if [[ $status -ne 0 ]]; then echo "native client CEX E2E failed in phase $CURRENT_PHASE" >&2; fi; if [[ "${KEEP_TRNM_E2E_WORK_DIR:-0}" == "1" ]]; then echo "retained $WORK_DIR" >&2; else rm -rf "$WORK_DIR"; fi; exit $status' EXIT

post_ledger() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_account() {
  post_ledger /v1/accounts "$(jq -cn --arg org "$ORG_ID" --arg role "$1" --argjson balance "$2" \
    '{org_id:$org,account_type:$role,currency_unit:"credit",initial_balance:$balance}')" | jq -er '.account_id'
}

CURRENT_PHASE="migration"
cex_psql_stdin -f - <"$CEX_PROJECT_ROOT/migrations/0028_add_trnm_seller_hold_and_identity_recovery.sql" >/dev/null
CURRENT_PHASE="accounts"
buyer_id="$(create_account trnm-native-client-buyer 200)"
seller_id="$(create_account trnm-native-client-market 0)"
recovery_key="recovery-${RUN_ID}-initial-credential"
rotated_key="recovery-${RUN_ID}-rotated-credential"

CURRENT_PHASE="identity-register"
post_ledger /v1/trnm/identity/register "$(jq -cn --arg player "$RUN_ID" --arg account "$buyer_id" --arg key "$recovery_key" \
  '{player_id:$player,account_id:$account,recovery_key:$key}')" | jq -e '.recovery_generation == 1' >/dev/null
CURRENT_PHASE="identity-recover"
post_ledger /v1/trnm/identity/recover "$(jq -cn --arg player "$RUN_ID" --arg key "$recovery_key" --arg next "$rotated_key" \
  '{player_id:$player,recovery_key:$key,new_recovery_key:$next}')" | jq -e '.recovery_generation == 2' >/dev/null

export TRNM_CAMPAIGN_SAVE_PATH="$WORK_DIR/campaign.json"
export TRNM_CEX_BASE_URL="$CONSUMER_URL"
export TRNM_CEX_ENTRY_TOKEN="$ENTRY_TOKEN"
export TRNM_CEX_ACCOUNT_ID="$buyer_id"
export TRNM_CEX_ACTOR_ID="$RUN_ID"
export TRNM_CEX_MARKET_ACCOUNT_ID="$seller_id"

binary="$PROJECT_ROOT/target/release/trnm-native-economy-e2e"
[[ -x "$binary" ]] || cargo build --manifest-path "$PROJECT_ROOT/trillionnium/Cargo.toml" --release -p trnm-first-contact --bin trnm-native-economy-e2e

CURRENT_PHASE="native-purchase"
"$binary" purchase | tail -n 1 >"$WORK_DIR/purchase.json"
jq -e '.native_bevy_input and .live_cex_http and .purchase_stage == "Consumed" and
  .item_quantity == 1 and (.ui_text | contains("wallet available"))' "$WORK_DIR/purchase.json" >/dev/null
purchase_id="$(jq -er '.purchase_id' "$WORK_DIR/purchase.json")"

CURRENT_PHASE="seller-hold"
seller_before="$(post_ledger /v1/trnm/economy/wallet "$(jq -cn --arg actor "market-$RUN_ID" --arg account "$seller_id" \
  '{actor_id:$actor,account_id:$account,reconciliation_cursor:1}')")"
jq -e '.available_credits == 0 and .reserved_credits > 0' <<<"$seller_before" >/dev/null

CURRENT_PHASE="service-restart"
systemctl --user restart cex-trnm-ledger.service cex-trnm-consumer.service
for _ in $(seq 1 60); do
  curl -fsS "$LEDGER_URL/v1/trnm/economy/readiness" >/dev/null 2>&1 \
    && curl -fsS "$CONSUMER_URL/v1/trillionnium/economy/adapters/readiness" >/dev/null 2>&1 \
    && break
  sleep 1
done

CURRENT_PHASE="native-verify"
"$binary" verify | tail -n 1 >"$WORK_DIR/verify.json"
jq -e '.native_bevy_input and .live_cex_http and .purchase_stage == "Consumed" and
  .item_quantity == 1 and (.ui_text | contains("wallet available"))' "$WORK_DIR/verify.json" >/dev/null

CURRENT_PHASE="native-cancel"
"$binary" cancel | tail -n 1 >"$WORK_DIR/cancel.json"
jq -e '.native_bevy_input and .live_cex_http and .purchase_stage == "Refunded" and
  .item_quantity == 0 and .wallet.available_credits == 200 and .wallet.reserved_credits == 0' \
  "$WORK_DIR/cancel.json" >/dev/null

CURRENT_PHASE="database-verification"
db_state="$(cex_psql_stdin -Atc "select json_build_object(
  'escrow_status', status,
  'seller_hold_amount', seller_hold_amount,
  'seller_hold_released', seller_hold_released,
  'identity_generation', (select recovery_generation from trnm_player_identities where player_id = '$RUN_ID'),
  'identity_audit_count', (select count(*) from trnm_identity_recovery_audit where player_id = '$RUN_ID')
) from trnm_escrow_trades where purchase_id = '$purchase_id';")"
jq -e '.escrow_status == "reversed" and (.seller_hold_amount|tonumber) == 0 and
  .seller_hold_released and .identity_generation == 2 and .identity_audit_count == 2' \
  <<<"$db_state" >/dev/null

CURRENT_PHASE="report"
jq -n --arg run_id "$RUN_ID" --arg buyer "$buyer_id" --arg seller "$seller_id" \
  --argjson purchase "$(<"$WORK_DIR/purchase.json")" \
  --argjson verify "$(<"$WORK_DIR/verify.json")" \
  --argjson cancel "$(<"$WORK_DIR/cancel.json")" \
  --argjson database "$db_state" \
  '{status:"passed",run_id:$run_id,buyer_account_id:$buyer,seller_account_id:$seller,
    native_client_purchase:$purchase,native_client_after_restart:$verify,
    native_client_after_cancel:$cancel,database:$database}'
