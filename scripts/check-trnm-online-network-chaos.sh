#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TC="${TC:-/usr/sbin/tc}"

if ! sudo -n true 2>/dev/null; then
  echo "passwordless sudo is required for scoped loopback netem" >&2
  exit 1
fi
if [[ ! -x "$TC" ]]; then
  echo "tc is required for network chaos" >&2
  exit 1
fi
if ! "$TC" qdisc show dev lo | grep -q '^qdisc noqueue'; then
  echo "refusing to replace an existing non-default loopback qdisc" >&2
  "$TC" qdisc show dev lo >&2
  exit 1
fi

cleanup() {
  sudo -n "$TC" qdisc del dev lo root >/dev/null 2>&1 || true
}
trap cleanup EXIT INT TERM

profiles=("50 1" "100 3" "200 5")
for profile in "${profiles[@]}"; do
  read -r latency_ms loss_percent <<<"$profile"
  cleanup
  sudo -n "$TC" qdisc add dev lo root handle 1: prio bands 3
  sudo -n "$TC" qdisc add dev lo parent 1:3 handle 30: netem \
    delay "${latency_ms}ms" loss "${loss_percent}%"
  sudo -n "$TC" filter add dev lo protocol ip parent 1:0 prio 3 u32 \
    match ip dport 7005 0xffff flowid 1:3
  result="$(TRNM_E2E_TICK_MS=80 "$ROOT_DIR/scripts/check-trnm-online-authority-e2e.sh")"
  jq -n \
    --argjson latency_ms "$latency_ms" \
    --argjson loss_percent "$loss_percent" \
    --argjson authority "$result" \
    '{latency_ms: $latency_ms, loss_percent: $loss_percent, authority: $authority}'
done
