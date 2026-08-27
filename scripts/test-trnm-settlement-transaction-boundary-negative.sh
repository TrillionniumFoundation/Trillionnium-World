#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-settlement-transaction-boundary.sh"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$TMP_DIR/src"
cat >"$TMP_DIR/src/lib.rs" <<'RS'
async fn capture_pending_settlement(pool: &Pool) {
    let mut transaction = pool.begin().await.unwrap();
    transaction.commit().await.unwrap();
}

async fn execute_pending_settlement(mut campaign: Campaign, cex: Cex) {
    tokio::task::spawn_blocking(move || campaign.reconcile_economy(&cex, 8)).await;
}

async fn apply_pending_settlement(pool: &Pool) {
    let mut transaction = pool.begin().await.unwrap();
    transaction.commit().await.unwrap();
}
RS
bash "$CHECKER" full "$TMP_DIR/src" >/dev/null

cat >"$TMP_DIR/src/lib.rs" <<'RS'
async fn capture_pending_settlement(pool: &Pool) {
    let mut transaction = pool.begin().await.unwrap();
    let mut campaign = load_campaign(&mut transaction).await;
    campaign.reconcile_economy(&state.cex, 8);
    transaction.commit().await.unwrap();
}

async fn apply_pending_settlement(pool: &Pool) {
    let mut transaction = pool.begin().await.unwrap();
    transaction.commit().await.unwrap();
}
RS
if bash "$CHECKER" scan-only "$TMP_DIR/src" >/dev/null 2>&1; then
  echo "settlement transaction-boundary negative fixture unexpectedly passed" >&2
  exit 1
fi

echo "TRNM settlement transaction-boundary negative fixture passed"
