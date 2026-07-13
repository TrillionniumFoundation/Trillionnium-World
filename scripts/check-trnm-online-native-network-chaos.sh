#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TC="${TC:-/usr/sbin/tc}"
LATENCY_MS="${TRNM_NATIVE_CHAOS_LATENCY_MS:-200}"
LOSS_PERCENT="${TRNM_NATIVE_CHAOS_LOSS_PERCENT:-5}"

if ! sudo -n true 2>/dev/null; then
  echo "passwordless sudo is required for scoped loopback netem" >&2
  exit 1
fi
if [[ ! -x "$TC" ]]; then
  echo "tc is required for native network chaos" >&2
  exit 1
fi
if ! "$TC" qdisc show dev lo | grep -q '^qdisc noqueue'; then
  echo "refusing to replace an existing non-default loopback qdisc" >&2
  "$TC" qdisc show dev lo >&2
  exit 1
fi

native="$(TRNM_NATIVE_CHAOS_LATENCY_MS="$LATENCY_MS" \
  TRNM_NATIVE_CHAOS_LOSS_PERCENT="$LOSS_PERCENT" \
  "$ROOT_DIR/scripts/check-trnm-online-native-two-client.sh")"
if ! "$TC" qdisc show dev lo | grep -q '^qdisc noqueue'; then
  echo "native probe left a non-default loopback qdisc behind" >&2
  "$TC" qdisc show dev lo >&2
  exit 1
fi
jq -n --arg contract_version trnm_online_native_network_chaos_v1 \
  --argjson latency_ms "$LATENCY_MS" --argjson loss_percent "$LOSS_PERCENT" \
  --argjson native "$native" '
  {
    contract_version:$contract_version,
    latency_ms:$latency_ms,
    loss_percent:$loss_percent,
    native:$native,
    passed:(
      $native.status == "passed" and
      $native.host_frame_timing.main_thread_updates_over_100ms == 0 and
      $native.guest_frame_timing.main_thread_updates_over_100ms == 0 and
      $native.host_frame_timing.frames_over_100ms == 0 and
      $native.guest_frame_timing.frames_over_100ms == 0 and
      $native.host_frame_timing.max_frame_delta_ms <= 100 and
      $native.guest_frame_timing.max_frame_delta_ms <= 100
    )
  }' | tee "$ROOT_DIR/run/native-network-chaos-latest.json"
jq -e '.passed == true' "$ROOT_DIR/run/native-network-chaos-latest.json" >/dev/null
