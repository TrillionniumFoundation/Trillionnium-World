#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S3_browser_parity/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/browser-parity.json"
SCREENSHOT_FILE="$ACCEPTANCE_DIR/browser-parity.png"
PORT="${TRILLIONNIUM_WORLD_BROWSER_PARITY_PORT:-28792}"
BIND_ADDR="127.0.0.1:$PORT"
STATE_FILE="$ACCEPTANCE_DIR/browser-parity-state.json"
LOG_FILE="$ACCEPTANCE_DIR/browser-parity-server.log"
CONTRACT="trillionnium_world_standalone_browser_parity_shell_v1"

mkdir -p "$ACCEPTANCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo build -p trnm-world-server
)

BIN="$ROOT/target/debug/trnm-world-server"
if [[ ! -x "$BIN" ]]; then
  printf 'debug binary missing: %s\n' "$BIN" >&2
  exit 1
fi

rm -f "$STATE_FILE" "$SUMMARY_FILE" "$SCREENSHOT_FILE" "$LOG_FILE"
"$BIN" serve --bind "$BIND_ADDR" --state-file "$STATE_FILE" --reset-state >"$LOG_FILE" 2>&1 &
SERVER_PID=$!
cleanup() {
  kill "$SERVER_PID" >/dev/null 2>&1 || true
  wait "$SERVER_PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT

STARTED=0
for _ in $(seq 1 80); do
  if ! kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    cat "$LOG_FILE" >&2 || true
    exit 1
  fi
  if curl -fsS "http://$BIND_ADDR/world/play" 2>/dev/null | grep -q "$CONTRACT"; then
    STARTED=1
    break
  fi
  sleep 0.1
done
if [[ "$STARTED" -ne 1 ]]; then
  printf 'browser parity server did not expose %s on %s\n' "$CONTRACT" "$BIND_ADDR" >&2
  exit 1
fi

node "$ROOT/scripts/check_trillionnium_world_browser_parity.mjs" \
  --base-url "http://$BIND_ADDR" \
  --summary-file "$SUMMARY_FILE" \
  --screenshot-file "$SCREENSHOT_FILE"

cleanup
trap - EXIT

jq -e '.status == "standalone_browser_parity_green"' "$SUMMARY_FILE" >/dev/null
printf 'TRILLIONNIUM_WORLD_BROWSER_PARITY_READY %s\n' "$SUMMARY_FILE"
