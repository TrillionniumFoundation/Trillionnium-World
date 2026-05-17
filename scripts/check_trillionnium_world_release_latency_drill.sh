#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-latency-drill.json"
PORT="${TRILLIONNIUM_WORLD_RELEASE_LATENCY_DRILL_PORT:-28791}"
BIND_ADDR="127.0.0.1:$PORT"
REQUESTS="${TRILLIONNIUM_WORLD_RELEASE_LATENCY_REQUESTS:-120}"
CONCURRENCY="${TRILLIONNIUM_WORLD_RELEASE_LATENCY_CONCURRENCY:-8}"
P95_BUDGET_SECONDS="${TRILLIONNIUM_WORLD_RELEASE_LATENCY_P95_BUDGET_SECONDS:-0.500}"
STATE_FILE="$ACCEPTANCE_DIR/release-latency-drill-state.json"
LOG_FILE="$ACCEPTANCE_DIR/release-latency-drill-server.log"
HEALTH_TIMES="$ACCEPTANCE_DIR/release-latency-health-times.txt"
HOME_TIMES="$ACCEPTANCE_DIR/release-latency-home-times.txt"
ADAPTER_TIMES="$ACCEPTANCE_DIR/release-latency-adapter-times.txt"
COMMAND_EVIDENCE="$ACCEPTANCE_DIR/release-latency-command.json"

mkdir -p "$ACCEPTANCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo build -p trnm-world-server --release
)

BIN="$ROOT/target/release/trnm-world-server"
if [[ ! -x "$BIN" ]]; then
  printf 'release binary missing: %s\n' "$BIN" >&2
  exit 1
fi

rm -f "$STATE_FILE" "$LOG_FILE" "$HEALTH_TIMES" "$HOME_TIMES" "$ADAPTER_TIMES" "$COMMAND_EVIDENCE"
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
  if curl -fsS "http://$BIND_ADDR/health" 2>/dev/null | grep -q 'trillionnium_world_dev_runtime_v1'; then
    STARTED=1
    break
  fi
  sleep 0.1
done
if [[ "$STARTED" -ne 1 ]]; then
  printf 'release latency drill server did not become healthy on %s\n' "$BIND_ADDR" >&2
  exit 1
fi
curl -fsS "http://$BIND_ADDR/health" | grep -q 'trillionnium_world_dev_runtime_v1'

run_probe() {
  local path="$1"
  local output="$2"
  local url="http://$BIND_ADDR$path"
  seq "$REQUESTS" | xargs -n 1 -P "$CONCURRENCY" sh -c 'curl -fsS -o /dev/null -w "%{time_total}\n" "$1"' _ "$url" >"$output"
}

endpoint_metrics_json() {
  local name="$1"
  local path="$2"
  local timings="$3"
  local count p95_index p95 max avg ok
  count="$(wc -l <"$timings" | tr -d ' ')"
  if [[ "$count" -gt 0 ]]; then
    p95_index=$(( (count * 95 + 99) / 100 ))
    p95="$(sort -n "$timings" | awk -v idx="$p95_index" 'NR == idx { print; exit }')"
    max="$(sort -n "$timings" | tail -n 1)"
    avg="$(awk '{ sum += $1 } END { if (NR > 0) printf "%.6f", sum / NR; else printf "0.000000" }' "$timings")"
  else
    p95="999.000000"
    max="999.000000"
    avg="999.000000"
  fi
  ok="$(awk -v count="$count" -v expected="$REQUESTS" -v p95="$p95" -v budget="$P95_BUDGET_SECONDS" 'BEGIN { if (count == expected && p95 <= budget) print "true"; else print "false" }')"
  jq -n \
    --arg name "$name" \
    --arg path "$path" \
    --arg timings "$timings" \
    --argjson request_count "$count" \
    --argjson expected_count "$REQUESTS" \
    --arg p95 "$p95" \
    --arg max "$max" \
    --arg avg "$avg" \
    --arg p95_budget "$P95_BUDGET_SECONDS" \
    --argjson ok "$ok" \
    '{
      name: $name,
      path: $path,
      timings_path: $timings,
      request_count: $request_count,
      expected_count: $expected_count,
      p95_seconds: ($p95 | tonumber),
      max_seconds: ($max | tonumber),
      avg_seconds: ($avg | tonumber),
      p95_budget_seconds: ($p95_budget | tonumber),
      ok: $ok
    }'
}

run_probe "/health" "$HEALTH_TIMES"
run_probe "/world/home" "$HOME_TIMES"
run_probe "/world/adapter-readiness" "$ADAPTER_TIMES"
curl -fsS "http://$BIND_ADDR/world/command?direction=east&actor_id=local-player" >"$COMMAND_EVIDENCE"
grep -q 'starter-studio' "$COMMAND_EVIDENCE"

HEALTH_METRICS="$(endpoint_metrics_json "health" "/health" "$HEALTH_TIMES")"
HOME_METRICS="$(endpoint_metrics_json "world_home" "/world/home" "$HOME_TIMES")"
ADAPTER_METRICS="$(endpoint_metrics_json "adapter_readiness" "/world/adapter-readiness" "$ADAPTER_TIMES")"

cleanup
trap - EXIT

STATUS="$(jq -n \
  --argjson health "$HEALTH_METRICS" \
  --argjson home "$HOME_METRICS" \
  --argjson adapter "$ADAPTER_METRICS" \
  'if ($health.ok and $home.ok and $adapter.ok) then "local_release_latency_drill_green" else "blocked_release_latency_budget" end' \
  -r)"

jq -n \
  --arg contract_version "trillionnium_world_release_latency_drill_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg release_binary "$BIN" \
  --arg release_binary_sha256 "$(sha256sum "$BIN" | awk '{print $1}')" \
  --arg bind_addr "$BIND_ADDR" \
  --arg state_file "$STATE_FILE" \
  --arg command_evidence "$COMMAND_EVIDENCE" \
  --argjson request_count "$REQUESTS" \
  --argjson concurrency "$CONCURRENCY" \
  --argjson health "$HEALTH_METRICS" \
  --argjson home "$HOME_METRICS" \
  --argjson adapter "$ADAPTER_METRICS" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trnm_world_release_latency_drill",
    public_launch_credit: "local_release_load_drill_only_not_multi_node_or_live_traffic",
    public_launch_ready: false,
    release: {
      binary_path: $release_binary,
      binary_sha256: $release_binary_sha256
    },
    local_drill: {
      bind_addr: $bind_addr,
      state_file: $state_file,
      request_count_per_endpoint: $request_count,
      concurrency: $concurrency,
      command_evidence: $command_evidence,
      command_mutation_verified: true
    },
    endpoints: [
      $health,
      $home,
      $adapter
    ],
    live_or_multi_node_requirements: [
      "multi_node_release_latency_evidence",
      "or_live_public_traffic_latency_evidence",
      "public_url_probe_samples",
      "monitoring_timeseries",
      "rollback_under_load_drill"
    ]
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "local_release_latency_drill_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_RELEASE_LATENCY_DRILL_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_RELEASE_LATENCY_DRILL_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
exit 1
