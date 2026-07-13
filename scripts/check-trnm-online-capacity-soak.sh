#!/usr/bin/env bash
set -euo pipefail

SCRIPT_PATH="$(realpath "${BASH_SOURCE[0]}")"
if [[ "${TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE:-0}" != 1 ]]; then
  exec systemd-run --user --scope --collect --quiet --expand-environment=no \
    --description='TRNM bounded online capacity harness' \
    -p CPUAccounting=true -p CPUWeight=100 -p CPUQuota=150% \
    -p MemoryAccounting=true -p MemoryHigh=1536M -p MemoryMax=2048M \
    -p MemorySwapMax=512M -p IOAccounting=true -p IOWeight=100 \
    -p TasksAccounting=true -p TasksMax=512 \
    env TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE=1 "$SCRIPT_PATH" "$@"
fi

RESOURCE_CGROUP="$(awk -F: '$1 == "0" {print $3}' /proc/self/cgroup)"
RESOURCE_CGROUP_ROOT="/sys/fs/cgroup$RESOURCE_CGROUP"
if [[ "${TRNM_CAPACITY_SCOPE_PROBE:-0}" == 1 ]]; then
  jq -n \
    --arg cgroup "$RESOURCE_CGROUP" \
    --arg memory_high "$(<"$RESOURCE_CGROUP_ROOT/memory.high")" \
    --arg memory_max "$(<"$RESOURCE_CGROUP_ROOT/memory.max")" \
    --arg memory_swap_max "$(<"$RESOURCE_CGROUP_ROOT/memory.swap.max")" \
    --arg cpu_max "$(<"$RESOURCE_CGROUP_ROOT/cpu.max")" \
    --arg tasks_max "$(<"$RESOURCE_CGROUP_ROOT/pids.max")" \
    '{status:"passed",cgroup:$cgroup,memory_high_bytes:($memory_high|tonumber),
      memory_max_bytes:($memory_max|tonumber),
      memory_swap_max_bytes:($memory_swap_max|tonumber),cpu_max:$cpu_max,
      tasks_max:($tasks_max|tonumber)}'
  exit 0
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
SIGNER_URL="${TRNM_ENTITLEMENT_SIGNER_URL:-http://127.0.0.1:7010}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
CONCURRENCY="${TRNM_CAPACITY_CONCURRENCY:-4}"
DURATION_SECONDS="${TRNM_CAPACITY_DURATION_SECONDS:-7200}"
MIN_AVAILABLE_MIB="${TRNM_CAPACITY_MIN_AVAILABLE_MIB:-3072}"
MAX_DATABASE_CONNECTIONS="${TRNM_CAPACITY_MAX_DATABASE_CONNECTIONS:-40}"
MONITOR_INTERVAL_SECONDS="${TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS:-10}"
RUN_ID="capacity-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/run/online-capacity/$RUN_ID"
SAMPLES="$EVIDENCE/operational-samples.jsonl"
mkdir -p "$EVIDENCE"

if ! [[ "$CONCURRENCY" =~ ^[1-9][0-9]*$ && "$CONCURRENCY" -le 32 ]]; then
  echo "TRNM_CAPACITY_CONCURRENCY must be between 1 and 32" >&2
  exit 2
fi
for value_name in DURATION_SECONDS MIN_AVAILABLE_MIB MAX_DATABASE_CONNECTIONS \
  MONITOR_INTERVAL_SECONDS; do
  value="${!value_name}"
  if ! [[ "$value" =~ ^[1-9][0-9]*$ ]]; then
    echo "$value_name must be a positive integer" >&2
    exit 2
  fi
done
if (( MAX_DATABASE_CONNECTIONS >= 50 )); then
  echo "TRNM_CAPACITY_MAX_DATABASE_CONNECTIONS must preserve PostgreSQL recovery capacity" >&2
  exit 2
fi
if [[ ! -x "$ROOT_DIR/target/release/trnm-online-e2e" ]]; then
  echo "missing release trnm-online-e2e binary" >&2
  exit 2
fi
if [[ "${TRNM_CAPACITY_ALLOW_DIRTY:-0}" != 1 ]]; then
  if [[ -n "$(git -C "$ROOT_DIR" status --porcelain)" || \
      -n "$(git -C "$CEX_ROOT" status --porcelain)" ]]; then
    echo "capacity evidence requires clean Trillionnium and CEX worktrees" >&2
    exit 2
  fi
fi

available_memory_mib() {
  awk '/^MemAvailable:/ {print int($2 / 1024)}' /proc/meminfo
}

host_oom_kills() {
  awk '$1 == "oom_kill" {print $2}' /proc/vmstat
}

unit_property() {
  systemctl --user show "$1" -p "$2" --value
}

unit_memory_event() {
  local unit="$1" key="$2" cgroup
  cgroup="$(unit_property "$unit" ControlGroup)"
  awk -v key="$key" '$1 == key {print $2}' "/sys/fs/cgroup$cgroup/memory.events"
}

resource_memory_event() {
  local key="$1"
  awk -v key="$key" '$1 == key {print $2}' "$RESOURCE_CGROUP_ROOT/memory.events"
}

postgres_runtime() {
  cex_docker inspect "$CEX_POSTGRES_CONTAINER_NAME" --format \
    '{"restart_count":{{.RestartCount}},"oom_killed":{{.State.OOMKilled}},"running":{{.State.Running}},"started_at":{{json .State.StartedAt}}}'
}

database_active_connections() {
  cex_psql_stdin -Atc 'select count(*) from pg_stat_activity'
}

wal_runtime() {
  cex_psql_stdin -Atc "select json_build_object(
    'archived_count', archived_count,
    'failed_count', failed_count,
    'last_archived_wal', last_archived_wal,
    'archiver_recovered', last_failed_time is null
      or coalesce(last_archived_time >= last_failed_time, false))
    from pg_stat_archiver"
}

online_readiness_matches_capacity() {
  local readiness
  readiness="$(curl -fsS --max-time 10 "$ONLINE_URL/v1/online/readiness" 2>/dev/null || true)"
  jq -e --argjson capacity "$CONCURRENCY" '
    .status == "ok" and .clock_mode == "real_time_no_catch_up" and
    .tick_rate_hz == 10 and .fleet_capacity == $capacity and
    .authority_clock_operational == true and
    .database_pool_saturation_healthy == true' >/dev/null 2>&1 <<<"$readiness"
}

ledger_readiness_is_operational() {
  local readiness
  readiness="$(curl -fsS --max-time 10 "$LEDGER_URL/v1/trnm/economy/readiness" 2>/dev/null || true)"
  jq -e '.status == "ok" and .postgres_operations_healthy == true and
    .postgres_operations.pool_saturation_healthy == true and
    .postgres_operations.archiver_recovered == true' \
    >/dev/null 2>&1 <<<"$readiness"
}

signer_readiness_is_operational() {
  local readiness
  readiness="$(curl -fsS --max-time 10 "$SIGNER_URL/v1/signer/readiness" 2>/dev/null || true)"
  jq -e '.status == "ok" and .postgres_receipts == true and
    .database_pool_saturation_healthy == true' >/dev/null 2>&1 <<<"$readiness"
}

require_host_memory_headroom() {
  local available
  available="$(available_memory_mib)"
  if (( available < MIN_AVAILABLE_MIB )); then
    echo "capacity harness requires ${MIN_AVAILABLE_MIB} MiB available; observed ${available} MiB" >&2
    return 1
  fi
}

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

fail_close_running_test_matches() {
  cex_psql_stdin -v ON_ERROR_STOP=1 -c "
    update trnm_online_matches m
    set phase='failed_closed', settlement_state='failed_closed',
        failure_reason='capacity soak interrupted or failed', updated_at=now()
    where m.phase='running' and exists (
      select 1 from trnm_online_match_members mm
      where mm.match_id=m.match_id and mm.player_id like '$RUN_ID-%'
    )" >/dev/null
}

worker_pids=()
cleanup() {
  local status=$? cleanup_failed=false pid
  trap - EXIT INT TERM
  for pid in "${worker_pids[@]:-}"; do
    kill -TERM "$pid" >/dev/null 2>&1 || true
  done
  for pid in "${worker_pids[@]:-}"; do
    wait "$pid" >/dev/null 2>&1 || true
  done
  fail_close_running_test_matches >/dev/null 2>&1 || cleanup_failed=true
  systemctl --user unset-environment TRNM_FLEET_CAPACITY >/dev/null 2>&1 || cleanup_failed=true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || cleanup_failed=true
  for _ in $(seq 1 60); do
    readiness="$(curl -fsS --max-time 5 "$ONLINE_URL/v1/online/readiness" 2>/dev/null || true)"
    if jq -e '.status == "ok" and .fleet_capacity == 4' \
        >/dev/null 2>&1 <<<"$readiness"; then
      break
    fi
    sleep 1
  done
  readiness="$(curl -fsS --max-time 5 "$ONLINE_URL/v1/online/readiness" 2>/dev/null || true)"
  jq -e '.status == "ok" and .fleet_capacity == 4' \
    >/dev/null 2>&1 <<<"$readiness" || cleanup_failed=true
  if [[ "$cleanup_failed" == true && "$status" -eq 0 ]]; then
    status=1
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

require_host_memory_headroom
for pair in \
  "$ROOT_DIR/deploy/systemd/trnm-game-server.service:$HOME/.config/systemd/user/trnm-game-server.service" \
  "$ROOT_DIR/deploy/systemd/trnm-entitlement-signer.service:$HOME/.config/systemd/user/trnm-entitlement-signer.service" \
  "$CEX_ROOT/deploy/systemd/cex-trnm-ledger.service:$HOME/.config/systemd/user/cex-trnm-ledger.service" \
  "$CEX_ROOT/deploy/systemd/cex-trnm-consumer.service:$HOME/.config/systemd/user/cex-trnm-consumer.service"; do
  IFS=: read -r source_unit installed_unit <<<"$pair"
  cmp -s "$source_unit" "$installed_unit" || {
    echo "installed unit differs from source: $installed_unit" >&2
    exit 2
  }
done

systemctl --user set-environment TRNM_FLEET_CAPACITY="$CONCURRENCY"
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  if online_readiness_matches_capacity && ledger_readiness_is_operational && \
      signer_readiness_is_operational; then
    break
  fi
  sleep 1
done
online_readiness_matches_capacity
ledger_readiness_is_operational
signer_readiness_is_operational

units=(trnm-game-server.service trnm-entitlement-signer.service \
  cex-trnm-ledger.service cex-trnm-consumer.service)
declare -A restarts_before cgroup_oom_before
for unit in "${units[@]}"; do
  restarts_before["$unit"]="$(unit_property "$unit" NRestarts)"
  cgroup_oom_before["$unit"]="$(unit_memory_event "$unit" oom_kill)"
done
resource_oom_before="$(resource_memory_event oom_kill)"
host_oom_before="$(host_oom_kills)"
postgres_before="$(postgres_runtime)"
wal_before="$(wal_runtime)"

TRNM_GIT_HEAD="$(git -C "$ROOT_DIR" rev-parse HEAD)"
CEX_GIT_HEAD="$(git -C "$CEX_ROOT" rev-parse HEAD)"
E2E_SHA256="$(sha256sum "$ROOT_DIR/target/release/trnm-online-e2e" | awk '{print $1}')"
GAME_SERVER_SHA256="$(sha256sum "$ROOT_DIR/target/release/trnm-game-server" | awk '{print $1}')"
SIGNER_SHA256="$(sha256sum "$ROOT_DIR/target/release/trnm-entitlement-signer" | awk '{print $1}')"
started_epoch="$(date +%s)"
started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
deadline_epoch=$((started_epoch + DURATION_SECONDS))

jq -n \
  --arg contract_version trnm_online_capacity_provenance_v1 \
  --arg run_id "$RUN_ID" --arg started_at "$started_at" \
  --arg trnm_git_head "$TRNM_GIT_HEAD" --arg cex_git_head "$CEX_GIT_HEAD" \
  --arg e2e_sha256 "$E2E_SHA256" --arg game_server_sha256 "$GAME_SERVER_SHA256" \
  --arg signer_sha256 "$SIGNER_SHA256" --arg resource_cgroup "$RESOURCE_CGROUP" \
  --argjson postgres "$postgres_before" \
  '{contract_version:$contract_version,run_id:$run_id,started_at:$started_at,
    trnm_git_head:$trnm_git_head,cex_git_head:$cex_git_head,
    binaries:{online_e2e_sha256:$e2e_sha256,game_server_sha256:$game_server_sha256,
      signer_sha256:$signer_sha256},resource_cgroup:$resource_cgroup,
    postgres:$postgres,worktrees_clean:true,installed_units_match_source:true}' \
  >"$EVIDENCE/provenance.json"

operational_check() {
  local available active_connections current_host_oom postgres healthy reason
  available="$(available_memory_mib)"
  active_connections="$(database_active_connections 2>/dev/null || printf 999)"
  current_host_oom="$(host_oom_kills)"
  postgres="$(postgres_runtime 2>/dev/null || printf '{"restart_count":-1,"oom_killed":true,"running":false}')"
  healthy=true
  reason=""
  if (( available < MIN_AVAILABLE_MIB )); then
    healthy=false
    reason+="memory_headroom;"
  fi
  if (( active_connections >= MAX_DATABASE_CONNECTIONS )); then
    healthy=false
    reason+="database_connections;"
  fi
  if [[ "$current_host_oom" != "$host_oom_before" ]]; then
    healthy=false
    reason+="host_oom;"
  fi
  if ! jq -e --argjson baseline "$(jq -r .restart_count <<<"$postgres_before")" '
      .running == true and .oom_killed == false and .restart_count == $baseline' \
      >/dev/null 2>&1 <<<"$postgres"; then
    healthy=false
    reason+="postgres_runtime;"
  fi
  for unit in "${units[@]}"; do
    if [[ "$(unit_property "$unit" ActiveState)" != active || \
        "$(unit_property "$unit" NRestarts)" != "${restarts_before[$unit]}" || \
        "$(unit_memory_event "$unit" oom_kill)" != "${cgroup_oom_before[$unit]}" ]]; then
      healthy=false
      reason+="$unit;"
    fi
  done
  if [[ "$(resource_memory_event oom_kill)" != "$resource_oom_before" ]]; then
    healthy=false
    reason+="harness_oom;"
  fi
  online_readiness_matches_capacity || { healthy=false; reason+="game_readiness;"; }
  ledger_readiness_is_operational || { healthy=false; reason+="ledger_readiness;"; }
  signer_readiness_is_operational || { healthy=false; reason+="signer_readiness;"; }
  jq -cn --argjson sampled_epoch "$(date +%s)" \
    --argjson available_memory_mib "$available" \
    --argjson database_active_connections "$active_connections" \
    --argjson healthy "$healthy" --arg reason "$reason" \
    '{sampled_epoch:$sampled_epoch,available_memory_mib:$available_memory_mib,
      database_active_connections:$database_active_connections,
      healthy:$healthy,reason:$reason}' >>"$SAMPLES"
  [[ "$healthy" == true ]]
}

operational_check
wave=0
failures=0
while (( $(date +%s) < deadline_epoch )); do
  if ! operational_check; then
    failures=$((failures + 1))
    break
  fi
  wave=$((wave + 1))
  worker_pids=()
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
    worker_pids+=("$!")
  done

  monitor_failure="$EVIDENCE/wave-${wave}.monitor-failure"
  (
    while :; do
      workers_alive=false
      for pid in "${worker_pids[@]}"; do
        if kill -0 "$pid" >/dev/null 2>&1; then
          workers_alive=true
          break
        fi
      done
      [[ "$workers_alive" == true ]] || exit 0
      sleep "$MONITOR_INTERVAL_SECONDS"
      if ! operational_check; then
        printf 'operational monitor failed at %s\n' "$(date -Is)" >"$monitor_failure"
        for pid in "${worker_pids[@]}"; do
          kill -TERM "$pid" >/dev/null 2>&1 || true
        done
        exit 1
      fi
    done
  ) &
  monitor_pid=$!

  wave_failed=0
  for index in "${!worker_pids[@]}"; do
    if wait "${worker_pids[$index]}" && jq -e '.status == "passed"' \
      >/dev/null "${reports[$index]}.tmp"; then
      mv "${reports[$index]}.tmp" "${reports[$index]}"
    else
      wave_failed=$((wave_failed + 1))
    fi
  done
  wait "$monitor_pid" || wave_failed=$((wave_failed + 1))
  worker_pids=()
  operational_check || wave_failed=$((wave_failed + 1))
  failures=$((failures + wave_failed))
  printf 'capacity_wave=%s concurrency=%s failures=%s elapsed_seconds=%s\n' \
    "$wave" "$CONCURRENCY" "$failures" "$(( $(date +%s) - started_epoch ))" >&2
  if (( failures != 0 )); then
    fail_close_running_test_matches
    break
  fi
done

finished_epoch="$(date +%s)"
finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
actual_duration_seconds=$((finished_epoch - started_epoch))
operational_check || failures=$((failures + 1))

declare -A restarts_after cgroup_oom_after
for unit in "${units[@]}"; do
  restarts_after["$unit"]="$(unit_property "$unit" NRestarts)"
  cgroup_oom_after["$unit"]="$(unit_memory_event "$unit" oom_kill)"
done
resource_oom_after="$(resource_memory_event oom_kill)"
host_oom_after="$(host_oom_kills)"
postgres_after="$(postgres_runtime)"
wal_after="$(wal_runtime)"
active_run_matches_final="$(cex_psql_stdin -Atc "
  select count(*) from trnm_online_matches m
  where m.phase='running' and exists (
    select 1 from trnm_online_match_members mm
    where mm.match_id=m.match_id and mm.player_id like '$RUN_ID-%')")"

journalctl --user --no-pager -o cat --since "@$started_epoch" \
  -u trnm-game-server.service -u trnm-entitlement-signer.service \
  -u cex-trnm-ledger.service -u cex-trnm-consumer.service \
  >"$EVIDENCE/service-journal.log"
cex_docker logs --since "$started_epoch" "$CEX_POSTGRES_CONTAINER_NAME" \
  >"$EVIDENCE/postgres.log" 2>&1 || true
journal_warning_count="$(awk 'BEGIN {IGNORECASE=1}
  /(^|[^[:alpha:]])(warn|error|panic|fatal)([^[:alpha:]]|$)/ {count++}
  END {print count+0}' "$EVIDENCE/service-journal.log")"
postgres_crash_count="$(awk 'BEGIN {IGNORECASE=1}
  /(PANIC|server process .* (terminated|crashed)|database system was interrupted)/ {count++}
  END {print count+0}' "$EVIDENCE/postgres.log")"

mapfile -t report_files < <(find "$EVIDENCE" -maxdepth 1 \
  -name 'wave-*-worker-*.json' -type f | sort)
if (( ${#report_files[@]} == 0 )); then
  echo "capacity soak produced no completed match reports" >&2
  exit 1
fi

samples_summary="$(jq -s '{count:length,
  all_healthy:all(.healthy == true),
  minimum_available_memory_mib:(map(.available_memory_mib)|min),
  maximum_database_active_connections:(map(.database_active_connections)|max),
  failed_samples:(map(select(.healthy == false))|length)}' "$SAMPLES")"
service_restarts="$(jq -n \
  --argjson game "$((restarts_after[trnm-game-server.service] - restarts_before[trnm-game-server.service]))" \
  --argjson signer "$((restarts_after[trnm-entitlement-signer.service] - restarts_before[trnm-entitlement-signer.service]))" \
  --argjson ledger "$((restarts_after[cex-trnm-ledger.service] - restarts_before[cex-trnm-ledger.service]))" \
  --argjson consumer "$((restarts_after[cex-trnm-consumer.service] - restarts_before[cex-trnm-consumer.service]))" \
  '{game_server:$game,signer:$signer,ledger:$ledger,consumer:$consumer}')"
cgroup_oom_kills="$(jq -n \
  --argjson game "$((cgroup_oom_after[trnm-game-server.service] - cgroup_oom_before[trnm-game-server.service]))" \
  --argjson signer "$((cgroup_oom_after[trnm-entitlement-signer.service] - cgroup_oom_before[trnm-entitlement-signer.service]))" \
  --argjson ledger "$((cgroup_oom_after[cex-trnm-ledger.service] - cgroup_oom_before[cex-trnm-ledger.service]))" \
  --argjson consumer "$((cgroup_oom_after[cex-trnm-consumer.service] - cgroup_oom_before[cex-trnm-consumer.service]))" \
  --argjson harness "$((resource_oom_after - resource_oom_before))" \
  '{game_server:$game,signer:$signer,ledger:$ledger,consumer:$consumer,harness:$harness}')"

jq -s \
  --arg contract_version trnm_online_capacity_soak_v2 \
  --arg run_id "$RUN_ID" --arg started_at "$started_at" --arg finished_at "$finished_at" \
  --arg trnm_git_head "$TRNM_GIT_HEAD" --arg cex_git_head "$CEX_GIT_HEAD" \
  --arg e2e_sha256 "$E2E_SHA256" --arg game_server_sha256 "$GAME_SERVER_SHA256" \
  --arg signer_sha256 "$SIGNER_SHA256" \
  --argjson requested_duration_seconds "$DURATION_SECONDS" \
  --argjson actual_duration_seconds "$actual_duration_seconds" \
  --argjson concurrency "$CONCURRENCY" --argjson waves "$wave" \
  --argjson failures "$failures" --argjson service_restarts "$service_restarts" \
  --argjson cgroup_oom_kills "$cgroup_oom_kills" \
  --argjson host_oom_kills "$((host_oom_after - host_oom_before))" \
  --argjson postgres_before "$postgres_before" --argjson postgres_after "$postgres_after" \
  --argjson wal_before "$wal_before" --argjson wal_after "$wal_after" \
  --argjson operational_samples "$samples_summary" \
  --argjson active_run_matches_final "$active_run_matches_final" \
  --argjson journal_warning_count "$journal_warning_count" \
  --argjson postgres_crash_count "$postgres_crash_count" \
  --arg resource_cgroup "$RESOURCE_CGROUP" \
  --argjson resource_memory_max_bytes "$(<"$RESOURCE_CGROUP_ROOT/memory.max")" \
  --argjson minimum_host_available_memory_mib "$MIN_AVAILABLE_MIB" '
  ([.[].command_ack_ms[]] | sort) as $acks |
  ([.[].match_tick_drift | if . < 0 then -. else . end] | max) as $max_abs_drift |
  (($acks | length) * 95 / 100 | ceil | . - 1) as $p95_index |
  ($service_restarts | [.[]] | add) as $service_restart_total |
  ($cgroup_oom_kills | [.[]] | add) as $cgroup_oom_total |
  {
    contract_version:$contract_version,run_id:$run_id,
    started_at:$started_at,finished_at:$finished_at,
    requested_duration_seconds:$requested_duration_seconds,
    actual_duration_seconds:$actual_duration_seconds,
    concurrency:$concurrency,waves:$waves,completed_matches:length,
    unique_matches:([.[].match_id] | unique | length),failures:$failures,
    source:{trnm_git_head:$trnm_git_head,cex_git_head:$cex_git_head,
      online_e2e_sha256:$e2e_sha256,game_server_sha256:$game_server_sha256,
      signer_sha256:$signer_sha256},
    service_restarts:$service_restarts,host_oom_kills:$host_oom_kills,
    cgroup_oom_kills:$cgroup_oom_kills,
    postgres:{before:$postgres_before,after:$postgres_after,
      restart_delta:($postgres_after.restart_count-$postgres_before.restart_count)},
    wal:{before:$wal_before,after:$wal_after,
      archived_delta:($wal_after.archived_count-$wal_before.archived_count),
      failed_delta:($wal_after.failed_count-$wal_before.failed_count)},
    operational_samples:$operational_samples,
    active_run_matches_final:$active_run_matches_final,
    journal_warning_or_error_count:$journal_warning_count,
    postgres_crash_signature_count:$postgres_crash_count,
    bounded_resource_scope:true,resource_cgroup:$resource_cgroup,
    resource_memory_max_bytes:$resource_memory_max_bytes,
    minimum_host_available_memory_mib:$minimum_host_available_memory_mib,
    all_settled:all(.settlement_state == "settled"),
    command_ack_samples:($acks|length),command_ack_p95_ms:$acks[$p95_index],
    command_ack_max_ms:($acks|max),max_absolute_match_tick_drift:$max_abs_drift,
    passed:(
      $failures == 0 and
      $actual_duration_seconds >= $requested_duration_seconds and
      $service_restart_total == 0 and $host_oom_kills == 0 and
      $cgroup_oom_total == 0 and
      ($postgres_after.restart_count-$postgres_before.restart_count) == 0 and
      $postgres_after.oom_killed == false and $postgres_after.running == true and
      ($wal_after.failed_count-$wal_before.failed_count) == 0 and
      ($wal_after.archived_count-$wal_before.archived_count) > 0 and
      $wal_after.archiver_recovered == true and
      $operational_samples.all_healthy == true and
      $active_run_matches_final == 0 and
      $journal_warning_count == 0 and $postgres_crash_count == 0 and
      length >= $concurrency and ([.[].match_id]|unique|length) == length and
      all(.settlement_state == "settled") and
      $acks[$p95_index] < 250 and $max_abs_drift < 2.0
    )
  }' "${report_files[@]}" >"$EVIDENCE/summary.json"

cat "$EVIDENCE/summary.json"
jq -e '.passed == true' "$EVIDENCE/summary.json" >/dev/null
