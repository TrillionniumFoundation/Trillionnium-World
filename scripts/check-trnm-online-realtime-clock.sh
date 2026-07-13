#!/usr/bin/env bash
set -euo pipefail

ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
DURATION_SECONDS="${TRNM_REALTIME_CLOCK_DURATION_SECONDS:-600}"
SAMPLE_SECONDS="${TRNM_REALTIME_CLOCK_SAMPLE_SECONDS:-10}"

if ! [[ "$DURATION_SECONDS" =~ ^[1-9][0-9]*$ && "$SAMPLE_SECONDS" =~ ^[1-9][0-9]*$ ]]; then
  echo "clock duration and sample interval must be positive integers" >&2
  exit 2
fi

readiness() {
  curl -fsS "$ONLINE_URL/v1/online/readiness"
}

start="$(readiness)"
jq -e '.status == "ok" and .clock_mode == "real_time_no_catch_up" and
  .tick_rate_hz == 10 and .simulation_ticks_per_wake == 1 and
  .restart_grants_immediate_tick == false and .fleet_capacity <= 4' \
  >/dev/null <<<"$start"
start_elapsed_ms="$(jq -er '.authority_clock_elapsed_ms | numbers' <<<"$start")"
start_wakes="$(jq -er '.authority_clock_wake_count | numbers' <<<"$start")"

remaining="$DURATION_SECONDS"
while (( remaining > 0 )); do
  interval="$SAMPLE_SECONDS"
  (( interval > remaining )) && interval="$remaining"
  sleep "$interval"
  remaining=$((remaining - interval))
  sample="$(readiness)"
  jq -e '.status == "ok" and .clock_mode == "real_time_no_catch_up"' \
    >/dev/null <<<"$sample"
  printf 'clock_soak_remaining_seconds=%s wake_count=%s drift_ticks=%s\n' \
    "$remaining" \
    "$(jq -r '.authority_clock_wake_count' <<<"$sample")" \
    "$(jq -r '.authority_clock_drift_ticks' <<<"$sample")"
done

finish="$(readiness)"
finish_elapsed_ms="$(jq -er '.authority_clock_elapsed_ms | numbers' <<<"$finish")"
finish_wakes="$(jq -er '.authority_clock_wake_count | numbers' <<<"$finish")"
summary="$(jq -n \
  --arg contract_version trnm_online_realtime_clock_v1 \
  --argjson duration_seconds "$DURATION_SECONDS" \
  --argjson start_elapsed_ms "$start_elapsed_ms" \
  --argjson finish_elapsed_ms "$finish_elapsed_ms" \
  --argjson start_wakes "$start_wakes" \
  --argjson finish_wakes "$finish_wakes" '
  (($finish_elapsed_ms - $start_elapsed_ms) / 100.0) as $expected_ticks |
  ($finish_wakes - $start_wakes) as $observed_ticks |
  ($observed_ticks - $expected_ticks) as $drift_ticks |
  {
    contract_version: $contract_version,
    duration_seconds: $duration_seconds,
    expected_ticks: $expected_ticks,
    observed_ticks: $observed_ticks,
    drift_ticks: $drift_ticks,
    absolute_drift_ticks: (if $drift_ticks < 0 then -$drift_ticks else $drift_ticks end),
    passed: ((if $drift_ticks < 0 then -$drift_ticks else $drift_ticks end) < 1.0)
  }')"
jq -e '.passed == true' >/dev/null <<<"$summary"
printf '%s\n' "$summary"
