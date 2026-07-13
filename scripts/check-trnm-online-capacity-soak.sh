#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
CONCURRENCY="${TRNM_CAPACITY_CONCURRENCY:-4}"
DURATION_SECONDS="${TRNM_CAPACITY_DURATION_SECONDS:-7200}"
RUN_ID="capacity-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/run/online-capacity/$RUN_ID"
mkdir -p "$EVIDENCE"

if ! [[ "$CONCURRENCY" =~ ^[1-9][0-9]*$ && "$CONCURRENCY" -le 32 ]]; then
  echo "TRNM_CAPACITY_CONCURRENCY must be between 1 and 32" >&2
  exit 2
fi
if ! [[ "$DURATION_SECONDS" =~ ^[1-9][0-9]*$ ]]; then
  echo "TRNM_CAPACITY_DURATION_SECONDS must be a positive integer" >&2
  exit 2
fi
if [[ ! -x "$ROOT_DIR/target/release/trnm-online-e2e" ]]; then
  echo "missing release trnm-online-e2e binary" >&2
  exit 2
fi

cleanup() {
  local status=$?
  systemctl --user unset-environment TRNM_FLEET_CAPACITY >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

admin_post() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local label="$1" account player recovery session
  account="$(admin_post /v1/accounts "$(jq -cn \
    --arg org '00000000-0000-0000-0000-00000000ce01' --arg label "$label" \
    '{org_id:$org,account_type:("capacity-"+$label),currency_unit:"credit",initial_balance:0}')" \
    | jq -er .account_id)"
  player="$RUN_ID-$label"
  recovery="recovery-$RUN_ID-$label-012345678901234567890123"
  admin_post /v1/trnm/identity/register "$(jq -cn \
    --arg player "$player" --arg account "$account" --arg recovery "$recovery" \
    '{player_id:$player,account_id:$account,recovery_key:$recovery}')" >/dev/null
  session="$(curl -fsS "$LEDGER_URL/v1/trnm/identity/session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg recovery "$recovery" --arg device "$RUN_ID-$label-device" \
      '{player_id:$player,recovery_key:$recovery,device_id:$device,lifetime_seconds:10800}')" \
    | jq -er .session_token)"
  printf '%s\t%s\t%s\n' "$player" "$account" "$session"
}

systemctl --user set-environment TRNM_FLEET_CAPACITY="$CONCURRENCY"
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  readiness="$(curl -fsS "$ONLINE_URL/v1/online/readiness" 2>/dev/null || true)"
  if jq -e --argjson capacity "$CONCURRENCY" \
    '.status == "ok" and .clock_mode == "real_time_no_catch_up" and
     .tick_rate_hz == 10 and .fleet_capacity == $capacity' \
    >/dev/null 2>&1 <<<"$readiness"; then
    break
  fi
  sleep 1
done
curl -fsS "$ONLINE_URL/v1/online/readiness" \
  | jq -e --argjson capacity "$CONCURRENCY" \
    '.status == "ok" and .clock_mode == "real_time_no_catch_up" and
     .tick_rate_hz == 10 and .fleet_capacity == $capacity' >/dev/null

server_restarts_before="$(systemctl --user show trnm-game-server.service -p NRestarts --value)"
started_epoch="$(date +%s)"
deadline_epoch=$((started_epoch + DURATION_SECONDS))
wave=0
failures=0

while (( $(date +%s) < deadline_epoch )); do
  wave=$((wave + 1))
  pids=()
  reports=()
  for worker in $(seq 1 "$CONCURRENCY"); do
    IFS=$'\t' read -r host_player host_account host_session \
      < <(create_identity "w${wave}-${worker}-host")
    IFS=$'\t' read -r guest_player guest_account guest_session \
      < <(create_identity "w${wave}-${worker}-guest")
    report="$EVIDENCE/wave-${wave}-worker-${worker}.json"
    reports+=("$report")
    TRNM_GAME_SERVER_URL="$ONLINE_URL" \
    TRNM_ONLINE_HOST_PLAYER_ID="$host_player" \
    TRNM_ONLINE_HOST_ACCOUNT_ID="$host_account" \
    TRNM_ONLINE_HOST_SESSION="$host_session" \
    TRNM_ONLINE_GUEST_PLAYER_ID="$guest_player" \
    TRNM_ONLINE_GUEST_ACCOUNT_ID="$guest_account" \
    TRNM_ONLINE_GUEST_SESSION="$guest_session" \
    TRNM_ONLINE_E2E_RESTART_SERVER=0 \
      "$ROOT_DIR/target/release/trnm-online-e2e" >"$report.tmp" 2>"$report.stderr" &
    pids+=("$!")
  done
  for index in "${!pids[@]}"; do
    if wait "${pids[$index]}" && jq -e '.status == "passed"' \
      >/dev/null "${reports[$index]}.tmp"; then
      mv "${reports[$index]}.tmp" "${reports[$index]}"
    else
      failures=$((failures + 1))
    fi
  done
  printf 'capacity_wave=%s concurrency=%s failures=%s elapsed_seconds=%s\n' \
    "$wave" "$CONCURRENCY" "$failures" "$(( $(date +%s) - started_epoch ))" >&2
  if (( failures != 0 )); then
    cex_psql_stdin -v ON_ERROR_STOP=1 -c "
      update trnm_online_matches m
      set phase='failed_closed', settlement_state='failed_closed',
          failure_reason='capacity soak worker failed', updated_at=now()
      where m.phase='running' and exists (
        select 1 from trnm_online_match_members mm
        where mm.match_id=m.match_id and mm.player_id like '$RUN_ID-%'
      )" >/dev/null
    break
  fi
done

server_restarts_after="$(systemctl --user show trnm-game-server.service -p NRestarts --value)"
mapfile -t report_files < <(find "$EVIDENCE" -maxdepth 1 -name 'wave-*-worker-*.json' -type f | sort)
if (( ${#report_files[@]} == 0 )); then
  echo "capacity soak produced no completed match reports" >&2
  exit 1
fi

jq -s \
  --arg contract_version trnm_online_capacity_soak_v1 \
  --arg run_id "$RUN_ID" \
  --argjson requested_duration_seconds "$DURATION_SECONDS" \
  --argjson concurrency "$CONCURRENCY" \
  --argjson waves "$wave" \
  --argjson failures "$failures" \
  --argjson restarts_before "$server_restarts_before" \
  --argjson restarts_after "$server_restarts_after" '
  ([.[].command_ack_ms[]] | sort) as $acks |
  ([.[].match_tick_drift | if . < 0 then -. else . end] | max) as $max_abs_drift |
  (($acks | length) * 95 / 100 | ceil | . - 1) as $p95_index |
  {
    contract_version: $contract_version,
    run_id: $run_id,
    requested_duration_seconds: $requested_duration_seconds,
    concurrency: $concurrency,
    waves: $waves,
    completed_matches: length,
    unique_matches: ([.[].match_id] | unique | length),
    failures: $failures,
    server_restarts: ($restarts_after - $restarts_before),
    all_settled: all(.settlement_state == "settled"),
    command_ack_samples: ($acks | length),
    command_ack_p95_ms: $acks[$p95_index],
    command_ack_max_ms: ($acks | max),
    max_absolute_match_tick_drift: $max_abs_drift,
    passed: (
      $failures == 0 and
      ($restarts_after - $restarts_before) == 0 and
      length >= $concurrency and
      ([.[].match_id] | unique | length) == length and
      all(.settlement_state == "settled") and
      $acks[$p95_index] < 250 and
      $max_abs_drift < 2.0
    )
  }' "${report_files[@]}" >"$EVIDENCE/summary.json"

jq -e '.passed == true' "$EVIDENCE/summary.json" >/dev/null
cat "$EVIDENCE/summary.json"
