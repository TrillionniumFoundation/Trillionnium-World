#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TC="/usr/sbin/tc"
RTT_MS="${TRNM_NATIVE_CHAOS_RTT_MS:-100}"
LOSS_PERCENT="${TRNM_NATIVE_CHAOS_LOSS_PERCENT:-1}"

if [[ ! "$RTT_MS" =~ ^[0-9]+$ ]] || (( RTT_MS <= 0 )); then
  echo "TRNM_NATIVE_CHAOS_RTT_MS must be a positive integer" >&2
  exit 64
fi
jq -en --arg value "$LOSS_PERCENT" \
  '($value | tonumber) >= 0 and ($value | tonumber) <= 100' >/dev/null

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

native="$(env -u TRNM_NATIVE_CHAOS_LATENCY_MS \
  TRNM_NATIVE_CHAOS_RTT_MS="$RTT_MS" \
  TRNM_NATIVE_CHAOS_LOSS_PERCENT="$LOSS_PERCENT" \
  "$ROOT_DIR/scripts/check-trnm-online-native-two-client.sh")"
if ! "$TC" qdisc show dev lo | grep -q '^qdisc noqueue'; then
  echo "native probe left a non-default loopback qdisc behind" >&2
  "$TC" qdisc show dev lo >&2
  exit 1
fi
jq -n --arg contract_version trnm_online_native_network_chaos_v2 \
  --argjson rtt_ms "$RTT_MS" --argjson loss_percent "$LOSS_PERCENT" \
  --argjson native "$native" '
  {
    contract_version:$contract_version,
    rtt_ms:$rtt_ms,
    one_way_delay_ms:(($rtt_ms + 1) / 2 | floor),
    loss_percent:$loss_percent,
    native:$native,
    passed:(
      $native.status == "passed" and
      $native.frame_contract == "trnm_online_render_frame_timing_v3" and
      $native.network_chaos.configured == true and
      $native.network_chaos.rtt_ms == $rtt_ms and
      $native.network_chaos.loss_percent == $loss_percent and
      $native.network_chaos.matched_transport == "ipv4_loopback_tcp_7005" and
      $native.network_chaos.netem_packets > 0 and
      $native.host_frame_timing.contract_version == "trnm_online_render_frame_timing_v3" and
      $native.guest_frame_timing.contract_version == "trnm_online_render_frame_timing_v3" and
      $native.host_frame_timing.clock == "bevy_time_real" and
      $native.guest_frame_timing.clock == "bevy_time_real" and
      $native.host_frame_timing.measurement_valid == true and
      $native.guest_frame_timing.measurement_valid == true and
      $native.host_frame_timing.targets.minimum_average_fps == 60 and
      $native.guest_frame_timing.targets.minimum_average_fps == 60 and
      $native.host_frame_timing.targets.minimum_one_percent_low_fps == 30 and
      $native.guest_frame_timing.targets.minimum_one_percent_low_fps == 30 and
      $native.host_frame_timing.network_main_thread_passed == true and
      $native.guest_frame_timing.network_main_thread_passed == true and
      $native.host_frame_timing.network_thread_instrumentation.passed == true and
      $native.guest_frame_timing.network_thread_instrumentation.passed == true and
      $native.host_frame_timing.native_input_to_durable_ack.passed == true and
      $native.guest_frame_timing.native_input_to_durable_ack.passed == true and
      $native.host_frame_timing.frame_cadence_passed == true and
      $native.guest_frame_timing.frame_cadence_passed == true and
      $native.host_frame_timing.passed == true and
      $native.guest_frame_timing.passed == true
    )
  }' | tee "$ROOT_DIR/run/native-network-chaos-latest.json"
jq -e '.passed == true' "$ROOT_DIR/run/native-network-chaos-latest.json" >/dev/null
