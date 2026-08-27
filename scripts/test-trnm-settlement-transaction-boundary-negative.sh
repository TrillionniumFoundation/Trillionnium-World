#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-settlement-transaction-boundary.sh"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$TMP_DIR/src"
cat >"$TMP_DIR/src/lib.rs" <<'RS'
async fn capture_match(pool: &Pool) {
    let mut transaction = pool.begin().await.unwrap();
    transaction.commit().await.unwrap();
}

async fn process_claimed_job(cex: &Cex, intent: &Intent) {
    let authorized = cex
        .authorize_settlement_intent(intent, "request", 1, 2, "nonce")
        .await
        .unwrap();
    cex.submit_authorized_settlement_intent(&authorized.intent)
        .await
        .unwrap();
}

async fn apply_capture(pool: &Pool) {
    let mut transaction = pool.begin().await.unwrap();
    transaction.commit().await.unwrap();
}
RS
bash "$CHECKER" scan-only "$TMP_DIR/src" >/dev/null

cat >"$TMP_DIR/src/lib.rs" <<'RS'
async fn capture_match(pool: &Pool, cex: &Cex, intent: &Intent) {
    let mut transaction = pool.begin().await.unwrap();
    cex.submit_authorized_settlement_intent(intent)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
}

async fn apply_capture(pool: &Pool) {
    let mut transaction = pool.begin().await.unwrap();
    transaction.commit().await.unwrap();
}
RS
if bash "$CHECKER" scan-only "$TMP_DIR/src" >/dev/null 2>&1; then
  echo "settlement transaction-boundary negative fixture unexpectedly passed" >&2
  exit 1
fi

echo "TRNM settlement transaction-boundary negative fixture passed"
