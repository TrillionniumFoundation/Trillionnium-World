#!/usr/bin/bash
set -euo pipefail
umask 077

while IFS= read -r inherited_function_name; do
  builtin unset -f "$inherited_function_name"
done < <(builtin compgen -A function)
unset inherited_function_name

readonly HARNESS_CONTRACT="trnm_online_authority_fault_evidence_v2"
readonly DECISION_CONTRACT="trnm_online_authority_fault_decision_v2"
readonly PROFILE="${1:-pg-rtt100}"
readonly CONTRACT_MODE="${TRNM_FAULT_HARNESS_CONTRACT_MODE:-0}"
readonly RESOURCE_SCOPE_ACTIVE="${TRNM_FAULT_RESOURCE_SCOPE_ACTIVE:-0}"
readonly TRUSTED_FORMAL_PATH="/usr/sbin:/usr/bin"

if (( $# > 2 )) || [[ "$PROFILE" != "pg-rtt100" ]]; then
  echo "usage: check-trnm-online-authority-fault-evidence.sh [pg-rtt100] [RELEASE_DIR]" >&2
  exit 64
fi

case "$CONTRACT_MODE" in
  0|1) ;;
  *) echo "TRNM_FAULT_HARNESS_CONTRACT_MODE must be 0 or 1" >&2; exit 64 ;;
esac
case "$RESOURCE_SCOPE_ACTIVE" in
  0|1) ;;
  *) echo "TRNM_FAULT_RESOURCE_SCOPE_ACTIVE must be 0 or 1" >&2; exit 64 ;;
esac
if [[ "$CONTRACT_MODE" == 0 ]]; then
  for forbidden_name in BASH_ENV ENV CDPATH GLOBIGNORE LD_PRELOAD LD_LIBRARY_PATH \
    PYTHONPATH PYTHONHOME PERL5LIB RUBYLIB NODE_OPTIONS NODE_PATH \
    GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM \
    HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy; do
    [[ ! -v "$forbidden_name" ]] || {
      echo "formal fault evidence rejects inherited $forbidden_name" >&2
      exit 64
    }
  done
  unset forbidden_name
  [[ "${PATH:-}" == "$TRUSTED_FORMAL_PATH" ]] || {
    echo "formal fault evidence requires PATH=$TRUSTED_FORMAL_PATH" >&2
    exit 64
  }
  export PATH="$TRUSTED_FORMAL_PATH"
  unset BASH_ENV ENV CDPATH GLOBIGNORE \
    LD_PRELOAD LD_LIBRARY_PATH PYTHONPATH PYTHONHOME PERL5LIB RUBYLIB
  [[ -z "${CEX_PROJECT_ROOT:-}" ]] || {
    echo "formal fault evidence does not accept CEX_PROJECT_ROOT overrides" >&2
    exit 64
  }
  [[ -z "${TRNM_CEX_LEDGER_URL:-}" || "$TRNM_CEX_LEDGER_URL" == "http://127.0.0.1:7002" ]] || {
    echo "formal fault evidence requires the canonical loopback ledger URL" >&2
    exit 64
  }
  [[ -z "${TRNM_ENTITLEMENT_SIGNER_URL:-}" || "$TRNM_ENTITLEMENT_SIGNER_URL" == "http://127.0.0.1:7010" ]] || {
    echo "formal fault evidence requires the canonical loopback signer URL" >&2
    exit 64
  }
fi

if [[ "$CONTRACT_MODE" == 0 ]]; then
  case "${TRNM_FAULT_SANITIZED_ENTRY:-0}" in
  0)
    canonical_account_record="$(/usr/bin/getent passwd "$UID")" \
      || { echo "formal fault evidence cannot resolve the current account" >&2; exit 64; }
    IFS=: read -r canonical_user _ canonical_uid canonical_gid _ canonical_home _ \
      <<<"$canonical_account_record"
    [[ "$canonical_uid" == "$UID" && "$canonical_home" == /* ]] \
      || { echo "formal fault evidence account identity is not canonical" >&2; exit 64; }
    canonical_runtime_dir="/run/user/$UID"
    sanitized_script="$(/usr/bin/realpath -e -- "${BASH_SOURCE[0]}")" \
      || { echo "formal fault evidence script path is not canonical" >&2; exit 64; }
    sanitized_environment=(
      PATH="$TRUSTED_FORMAL_PATH" LC_ALL=C LANG=C TZ=UTC
      HOME="$canonical_home" USER="$canonical_user" LOGNAME="$canonical_user"
      XDG_RUNTIME_DIR="$canonical_runtime_dir"
      DBUS_SESSION_BUS_ADDRESS="unix:path=$canonical_runtime_dir/bus"
      NO_PROXY="127.0.0.1,localhost" no_proxy="127.0.0.1,localhost"
      TRNM_FAULT_SANITIZED_ENTRY=1
    )
    if [[ -v TRNM_FAULT_RESOURCE_SCOPE_ACTIVE ]]; then
      sanitized_environment+=(
        "TRNM_FAULT_RESOURCE_SCOPE_ACTIVE=$TRNM_FAULT_RESOURCE_SCOPE_ACTIVE"
      )
    fi
    exec /usr/bin/env -i "${sanitized_environment[@]}" "$sanitized_script" "$@"
    ;;
  1)
    unset TRNM_FAULT_SANITIZED_ENTRY
    ;;
  *)
    echo "TRNM_FAULT_SANITIZED_ENTRY is an internal recursion guard" >&2
    exit 64
    ;;
  esac
fi

if [[ "$CONTRACT_MODE" == 0 ]]; then
  while IFS= read -r environment_name; do
    case "$environment_name" in
      PATH|LC_ALL|LANG|TZ|HOME|USER|LOGNAME|XDG_RUNTIME_DIR|\
        DBUS_SESSION_BUS_ADDRESS|NO_PROXY|no_proxy|PWD|SHLVL|_|\
        TRNM_FAULT_RESOURCE_SCOPE_ACTIVE)
        ;;
      *)
        builtin unset "$environment_name" 2>/dev/null \
          || builtin export -n "$environment_name" 2>/dev/null \
          || { echo "could not clear formal fault environment variable: $environment_name" >&2; exit 64; }
        ;;
    esac
  done < <(builtin compgen -e)
  unset environment_name
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
if [[ "$CONTRACT_MODE" == 1 ]]; then
  [[ -f "$ROOT_DIR/.trnm-fault-contract-fixture" \
      && ! -L "$ROOT_DIR/.trnm-fault-contract-fixture" \
      && "$(<"$ROOT_DIR/.trnm-fault-contract-fixture")" \
        == trnm-online-authority-fault-contract-fixture-v1 ]] \
    || {
      echo "contract mode is restricted to the isolated shell fixture" >&2
      exit 64
    }
fi
if [[ "$CONTRACT_MODE" == 1 ]]; then
  CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
else
  CEX_ROOT="$ROOT_DIR/../CEX"
fi
RELEASE_ROOT="$ROOT_DIR/run/releases/trnm-game-server"
REQUESTED_RELEASE="${2:-$RELEASE_ROOT/current}"
EVIDENCE_ROOT="$ROOT_DIR/run/online-faults"
LOCK_ROOT="$ROOT_DIR/run/locks"
CANONICAL_JOURNAL="$ROOT_DIR/run/trnm-game-server/published-ticks"
SOURCE_UNIT="$ROOT_DIR/deploy/systemd/trnm-game-server.service"
INSTALLED_UNIT=""
SERVICE="trnm-game-server.service"
PRODUCTION_URL="http://127.0.0.1:7005"
TEST_URL="http://127.0.0.1:7006"
LEDGER_URL="http://127.0.0.1:7002"
SIGNER_URL="http://127.0.0.1:7010"
MAINTENANCE_FAILURE_REASON="local Authority fault harness exact cleanup"
TEST_SERVER_PORT=7006
PROXY_PORT=7543
if [[ "$CONTRACT_MODE" == 1 ]]; then
  WAIT_ATTEMPTS=20
  WAIT_INTERVAL=0.05
  E2E_TIMEOUT_SECONDS=20
else
  WAIT_ATTEMPTS=180
  WAIT_INTERVAL=1
  E2E_TIMEOUT_SECONDS=1500
fi

RUN_DIR=""
RUN_ID=""
RUN_STARTED_AT=""
RELEASE_JSON=""
RELEASE_DIR=""
RELEASE_ID=""
RELEASE_COMMIT=""
RELEASE_TREE=""
GAME_SERVER_BIN=""
ONLINE_E2E_BIN=""
GAME_SERVER_SHA=""
ONLINE_E2E_SHA=""
ORIGINAL_ACTIVE_STATE=""
ORIGINAL_SUB_STATE=""
ORIGINAL_MAIN_PID=""
ORIGINAL_UNIT_FILE_STATE=""
ORIGINAL_RELEASE_DIR=""
ORIGINAL_RELEASE_SHA=""
SERVICE_STATE_CAPTURED=0
ORIGINAL_RUNTIME_VERIFIED=0
TEST_INSTANCE_ID=""
PHYSICAL_HOST_ID=""
DIRECT_DATABASE_URL=""
PROXY_DATABASE_URL=""
DATABASE_PASSWORD=""
HOST_PLAYER=""
HOST_ACCOUNT=""
HOST_SESSION=""
GUEST_PLAYER=""
GUEST_ACCOUNT=""
GUEST_SESSION=""
HOST_RECOVERY=""
GUEST_RECOVERY=""
AUTHORITY_TOKEN=""
MODERATOR_TOKEN=""
SIGNER_TOKEN=""
MONITOR_PID=""
SERVER_PID=""
PROXY_PID=""
E2E_PID=""
declare -A CLEANUP_PROCESS_START CLEANUP_PROCESS_EXE CLEANUP_PROCESS_CGROUP CLEANUP_PROCESS_PGID
QDISC_OWNED=0
QDISC_CONFIGURED=0
QDISC_MUTATION_ATTEMPTED=0
QDISC_STAGE="none"
QDISC_FINGERPRINT=""
QDISC_PACKETS_BEFORE=0
QDISC_PACKETS_AFTER=0
WORKLOAD_RC=1
WORKLOAD_COMPLETE=0
CLEANUP_COMPLETE=0
CLEANUP_FAILED=0
SECRET_REDACTION_REQUIRED=0
SIGNAL_NAME=""
DEFERRED_SIGNAL=""
JOURNAL_LOCK_FD=""
DEPLOYMENT_LOCK_FD=""
MATCH_ID=""
SERVER_IDENTITY_JSON=""
PROXY_IDENTITY_JSON=""
ONLINE_E2E_SHA_BEFORE=""
ONLINE_E2E_SHA_AFTER=""
WORKLOAD_STARTED_EPOCH_MS=0
WORKLOAD_ENDED_EPOCH_MS=0
QDISC_PACKETS_WORKLOAD_BEFORE=0
QDISC_PACKETS_WORKLOAD_AFTER=0
CEX_HELPER_SHA=""
SOURCE_UNIT_SHA=""
INSTALLED_UNIT_SHA=""
SCRIPT_SHA=""
RELEASE_MANIFEST_SHA=""
SHARED_HOST_LOCK_ROOT=""
RESOURCE_CGROUP=""
MONITOR_SURVIVED_WORKLOAD=0
BOUND_INPUTS_UNCHANGED=0

fail() {
  echo "TRNM Authority fault-evidence harness failed: $*" >&2
  return 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || fail "required command is unavailable: $1"
}

close_inherited_mutation_locks() {
  if [[ -n "$HARNESS_LOCK_FD" ]]; then
    exec {HARNESS_LOCK_FD}>&-
  fi
  if [[ -n "$DEPLOYMENT_LOCK_FD" ]]; then
    exec {DEPLOYMENT_LOCK_FD}>&-
  fi
}

validate_open_private_lock() {
  local path="$1" fd="$2" target
  [[ -f "$path" && ! -L "$path" \
      && "$(stat -c '%a' "$path")" == 600 \
      && "$(stat -c '%u' "$path")" == "$(id -u)" \
      && "$(stat -c '%g' "$path")" == "$(id -g)" \
      && "$(stat -c '%h' "$path")" == 1 ]] \
    || fail "lock is not a private owner-only single-link regular file: $path"
  target="$(readlink -e -- "/proc/$$/fd/$fd")" \
    || fail "could not resolve open lock descriptor: $path"
  [[ "$target" == "$(realpath -e -- "$path")" \
      && "$(stat -Lc '%d:%i' "/proc/$$/fd/$fd")" == "$(stat -c '%d:%i' "$path")" ]] \
    || fail "open lock descriptor does not identify the validated lock: $path"
}

ensure_private_directory() {
  local path="$1"
  if [[ -e "$path" || -L "$path" ]]; then
    [[ -d "$path" && ! -L "$path" ]] \
      || fail "private harness path is not a real directory: $path"
  else
    install -d -m 0700 -- "$path"
  fi
  [[ "$(stat -c '%a' "$path")" == 700 \
      && "$(stat -c '%u' "$path")" == "$(id -u)" \
      && "$(stat -c '%g' "$path")" == "$(id -g)" ]] \
    || fail "private harness directory is not owner-only: $path"
}

validate_bounded_resource_cgroup() {
  local relative root memory_max memory_high swap_max pids_max cpu_max quota period scope_unit
  relative="$(awk -F: '$1 == "0" { print $3 }' /proc/self/cgroup)"
  [[ -n "$relative" ]] || fail "formal fault evidence requires cgroup v2"
  scope_unit="${relative##*/}"
  [[ "$scope_unit" =~ ^trnm-authority-fault-[0-9]{8}T[0-9]{6}-[0-9]+\.scope$ \
      && "$(systemctl --user show "$scope_unit" -p ControlGroup --value)" == "$relative" ]] \
    || fail "formal fault evidence is not in its named transient scope"
  root="/sys/fs/cgroup$relative"
  [[ -d "$root" ]] || fail "formal fault evidence cgroup is unavailable"
  memory_max="$(<"$root/memory.max")"
  memory_high="$(<"$root/memory.high")"
  swap_max="$(<"$root/memory.swap.max")"
  pids_max="$(<"$root/pids.max")"
  cpu_max="$(<"$root/cpu.max")"
  read -r quota period <<<"$cpu_max"
  [[ "$memory_max" =~ ^[0-9]+$ && "$memory_max" -le 2147483648 ]] \
    || fail "formal fault evidence requires MemoryMax <= 2 GiB"
  [[ "$memory_high" =~ ^[0-9]+$ && "$memory_high" -le 1610612736 ]] \
    || fail "formal fault evidence requires MemoryHigh <= 1.5 GiB"
  [[ "$swap_max" =~ ^[0-9]+$ && "$swap_max" -le 536870912 ]] \
    || fail "formal fault evidence requires MemorySwapMax <= 512 MiB"
  [[ "$pids_max" =~ ^[0-9]+$ && "$pids_max" -le 512 ]] \
    || fail "formal fault evidence requires TasksMax <= 512"
  [[ "$quota" =~ ^[0-9]+$ && "$period" =~ ^[1-9][0-9]*$ \
      && "$(( quota * 100 ))" -le "$(( period * 150 ))" ]] \
    || fail "formal fault evidence requires CPUQuota <= 150%"
  RESOURCE_CGROUP="$relative"
}

path_is_within() {
  local child="$1" parent="$2"
  [[ "$child" == "$parent" || "$child" == "$parent/"* ]]
}

# Compatibility seam for the published-tick journal layout.  Hot records stay
# flat; acknowledged terminal tombstones use two lowercase hex shards derived
# from the first four simple-UUID digits.
journal_hot_relative_path() {
  printf 'published-%s.json\n' "$1"
}

journal_ack_relative_path() {
  local match_id="$1" simple
  simple="${match_id//-/}"
  printf 'acknowledged/%s/%s/acknowledged-%s.json\n' \
    "${simple:0:2}" "${simple:2:2}" "$match_id"
}

journal_abandonment_relative_path() {
  local match_id="$1" simple
  simple="${match_id//-/}"
  printf 'abandoned/%s/%s/abandoned-%s.json\n' \
    "${simple:0:2}" "${simple:2:2}" "$match_id"
}

atomic_write() {
  local destination="$1" temporary
  temporary="$destination.tmp.$$"
  cat >"$temporary"
  chmod 0600 "$temporary"
  python3 - "$temporary" <<'PY'
import os, sys
fd = os.open(sys.argv[1], os.O_RDONLY)
try:
    os.fsync(fd)
finally:
    os.close(fd)
PY
  mv -f -- "$temporary" "$destination"
  python3 - "$(dirname "$destination")" <<'PY'
import os, sys
fd = os.open(sys.argv[1], os.O_RDONLY | getattr(os, "O_DIRECTORY", 0))
try:
    os.fsync(fd)
finally:
    os.close(fd)
PY
}

unit_property() {
  systemctl --user show "$SERVICE" -p "$1" --value
}

fault_unit_property() {
  systemctl --user show "$1" -p "$2" --value
}

fault_unit_contract_values() {
  case "$1" in
  trnm-game-server.service)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$SOURCE_UNIT" "$ROOT_DIR/scripts/run-trnm-game-server.sh" \
      "$ROOT_DIR/scripts/run-trnm-game-server.sh" \
      2s 402653184 536870912 134217728 256 '200000 100000'
    ;;
  trnm-entitlement-signer.service)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$ROOT_DIR/deploy/systemd/trnm-entitlement-signer.service" \
      "$ROOT_DIR/scripts/run-trnm-entitlement-signer.sh" \
      "$ROOT_DIR/scripts/run-trnm-entitlement-signer.sh" \
      500ms 67108864 100663296 33554432 128 '50000 100000'
    ;;
  cex-trnm-ledger.service)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$CEX_ROOT/deploy/systemd/cex-trnm-ledger.service" \
      "$CEX_ROOT/scripts/run-trnm-economy-service.sh" \
      "$CEX_ROOT/scripts/run-trnm-economy-service.sh ledger" \
      1s 268435456 402653184 134217728 256 '100000 100000'
    ;;
  *) return 1 ;;
  esac
}

fault_effective_unit_matches_source() {
  local unit="$1" source expected_exec expected_argv expected_cpu
  local high max swap tasks expected_cpu_max fragment exec_start
  IFS=$'\t' read -r source expected_exec expected_argv expected_cpu \
    high max swap tasks expected_cpu_max < <(fault_unit_contract_values "$unit") \
    || return 1
  fragment="$(realpath -e -- "$(fault_unit_property "$unit" FragmentPath)")" \
    || return 1
  [[ "$fragment" == "$(realpath -e -- "$HOME/.config/systemd/user/$unit")" \
      && -z "$(fault_unit_property "$unit" DropInPaths)" \
      && "$(fault_unit_property "$unit" CPUAccounting)" == yes \
      && "$(fault_unit_property "$unit" CPUQuotaPerSecUSec)" == "$expected_cpu" \
      && "$(fault_unit_property "$unit" MemoryAccounting)" == yes \
      && "$(fault_unit_property "$unit" MemoryHigh)" == "$high" \
      && "$(fault_unit_property "$unit" MemoryMax)" == "$max" \
      && "$(fault_unit_property "$unit" MemorySwapMax)" == "$swap" \
      && "$(fault_unit_property "$unit" TasksAccounting)" == yes \
      && "$(fault_unit_property "$unit" TasksMax)" == "$tasks" \
      && -f "$source" && ! -L "$source" ]] \
    || return 1
  cmp -s "$source" "$fragment" || return 1
  exec_start="$(fault_unit_property "$unit" ExecStart)" || return 1
  [[ "$exec_start" == *"path=$expected_exec"* \
      && "$exec_start" == *"argv[]=$expected_argv"* \
      && -n "$expected_cpu_max" ]]
}

fault_active_cgroup_matches_source() {
  local unit="$1" _source _exec _argv _cpu high max swap tasks cpu_max
  local cgroup pid process_cgroup root
  IFS=$'\t' read -r _source _exec _argv _cpu high max swap tasks cpu_max \
    < <(fault_unit_contract_values "$unit") || return 1
  cgroup="$(fault_unit_property "$unit" ControlGroup)" || return 1
  pid="$(fault_unit_property "$unit" MainPID)" || return 1
  [[ -n "$cgroup" && "$cgroup" != / && "$pid" =~ ^[1-9][0-9]*$ ]] || return 1
  process_cgroup="$(awk -F: '$1 == "0" {print $3}' "/proc/$pid/cgroup")" || return 1
  [[ "$process_cgroup" == "$cgroup" ]] || return 1
  root="/sys/fs/cgroup$cgroup"
  [[ -d "$root" && "$(<"$root/memory.high")" == "$high" \
      && "$(<"$root/memory.max")" == "$max" \
      && "$(<"$root/memory.swap.max")" == "$swap" \
      && "$(<"$root/pids.max")" == "$tasks" \
      && "$(<"$root/cpu.max")" == "$cpu_max" ]]
}

port_listeners() {
  local port="$1"
  ss -H -ltn "( sport = :$port )" 2>/dev/null || true
}

port_is_free() {
  [[ -z "$(port_listeners "$1")" ]]
}

port_is_listening() {
  [[ -n "$(port_listeners "$1")" ]]
}

wait_for_port_state() {
  local port="$1" wanted="$2"
  local attempt
  for ((attempt=0; attempt<WAIT_ATTEMPTS; attempt++)); do
    if [[ "$wanted" == listening ]] && port_is_listening "$port"; then
      return 0
    fi
    if [[ "$wanted" == free ]] && port_is_free "$port"; then
      return 0
    fi
    sleep "$WAIT_INTERVAL"
  done
  fail "port $port did not become $wanted"
}

wait_for_unit_state() {
  local wanted="$1"
  local attempt
  for ((attempt=0; attempt<WAIT_ATTEMPTS; attempt++)); do
    [[ "$(unit_property ActiveState)" == "$wanted" ]] && return 0
    sleep "$WAIT_INTERVAL"
  done
  fail "$SERVICE did not become $wanted"
}

readiness_json() {
  curl -fsS --max-time 5 "$1/v1/online/readiness"
}

wait_for_test_readiness() {
  local body="" attempt
  for ((attempt=0; attempt<WAIT_ATTEMPTS; attempt++)); do
    body="$(readiness_json "$TEST_URL" 2>/dev/null || true)"
    if jq -e --arg instance "$TEST_INSTANCE_ID" '
        .status == "ok"
        and .fleet_instance_id == $instance
        and .authority_clock_operational == true
        and .match_actor_clocks_operational == true
        and .published_tick_journal_operational == true
        and .latest_cold_witness_sentinel_query_healthy == true
        and .latest_cold_witness_sentinel_healthy == true
        and .cold_witness_database_summary_query_healthy == true
        and .local_tombstone_counts_exact == true
        and .local_tombstone_seal_operational == true
        and .operational_readiness.local_cold_witness_seal == true
        and .published_tick_terminal_orphan_recovery_operational == true
      ' >/dev/null 2>&1 <<<"$body"; then
      return 0
    fi
    sleep "$WAIT_INTERVAL"
  done
  fail "standalone Authority did not become ready"
}

wait_for_production_readiness() {
  local body="" attempt
  for ((attempt=0; attempt<WAIT_ATTEMPTS; attempt++)); do
    body="$(readiness_json "$PRODUCTION_URL" 2>/dev/null || true)"
    if jq -e '
        .status == "ok"
        and .authority_clock_operational == true
        and .latest_cold_witness_sentinel_query_healthy == true
        and .latest_cold_witness_sentinel_healthy == true
        and .cold_witness_database_summary_query_healthy == true
        and .local_tombstone_counts_exact == true
        and .local_tombstone_seal_operational == true
        and .operational_readiness.local_cold_witness_seal == true
      ' \
        >/dev/null 2>&1 <<<"$body"; then
      return 0
    fi
    sleep "$WAIT_INTERVAL"
  done
  fail "restored production Authority did not become ready"
}

verify_active_service_binary() {
  local pid executable
  [[ "$(unit_property ActiveState)" == active ]] || return 1
  pid="$(unit_property MainPID)"
  [[ "$pid" =~ ^[1-9][0-9]*$ ]] || return 1
  if [[ "$CONTRACT_MODE" == 1 ]]; then
    return 0
  fi
  executable="$(readlink -e -- "/proc/$pid/exe" 2>/dev/null)" || return 1
  [[ "$executable" == "$ORIGINAL_RELEASE_DIR/trnm-game-server" \
      && "$(sha256sum "$executable" | awk '{print $1}')" == "$ORIGINAL_RELEASE_SHA" ]]
}

capture_bound_process_identity() {
  local pid="$1" expected_executable="$2" port="$3" destination="$4" label="$5"
  local executable executable_sha expected_sha start_ticks cgroup listeners owned=false command_bound=false
  local native_executable_match=false
  [[ "$pid" =~ ^[1-9][0-9]*$ && -d "/proc/$pid" ]] \
    || fail "$label process is not alive for identity capture"
  executable="$(readlink -e -- "/proc/$pid/exe")" \
    || fail "$label executable identity is unavailable"
  expected_executable="$(realpath -e -- "$expected_executable")"
  executable_sha="$(sha256sum "$executable" | awk '{print $1}')"
  expected_sha="$(sha256sum "$expected_executable" | awk '{print $1}')"
  start_ticks="$(awk '{print $22}' "/proc/$pid/stat")"
  cgroup="$(awk -F: '$1 == "0" { print $3 }' "/proc/$pid/cgroup")"
  listeners="$(ss -H -ltnp "( sport = :$port )" 2>/dev/null)"
  grep -Eq "pid=$pid([,)])" <<<"$listeners" && owned=true
  if [[ "$executable" == "$expected_executable" ]]; then
    command_bound=true
    native_executable_match=true
  elif [[ "$CONTRACT_MODE" == 1 ]] \
      && tr '\0' '\n' <"/proc/$pid/cmdline" | grep -Fxq -- "$expected_executable"; then
    command_bound=true
  fi
  [[ "$command_bound" == true && "$owned" == true ]] \
    || fail "$label PID/executable does not own the expected listener"
  if [[ "$CONTRACT_MODE" == 0 ]]; then
    [[ "$cgroup" == "$RESOURCE_CGROUP" ]] \
      || fail "$label escaped the bounded formal cgroup"
  fi
  jq -n --arg contract_version trnm_fault_bound_process_identity_v1 \
    --arg label "$label" --arg executable "$executable" \
    --arg executable_sha256 "$executable_sha" --arg expected_executable "$expected_executable" \
    --arg expected_artifact_sha256 "$expected_sha" --arg start_ticks "$start_ticks" \
    --arg cgroup "$cgroup" --argjson pid "$pid" --argjson port "$port" \
    --argjson listener_owned "$owned" --argjson command_bound "$command_bound" \
    --argjson native_executable_match "$native_executable_match" \
    '{contract_version:$contract_version,label:$label,pid:$pid,port:$port,
      executable:$executable,executable_sha256:$executable_sha256,
      expected_executable:$expected_executable,expected_artifact_sha256:$expected_artifact_sha256,
      process_start_ticks:($start_ticks|tonumber),cgroup:$cgroup,
      command_bound:$command_bound,native_executable_match:$native_executable_match,
      listener_owned_by_pid:$listener_owned}' | atomic_write "$destination"
}

capture_dependency_process_identity() {
  local unit="$1" port="$2" destination="$3"
  local pid executable executable_sha start_ticks cgroup listeners fragment fragment_sha exec_start_sha
  if [[ "$CONTRACT_MODE" == 0 ]]; then
    fault_effective_unit_matches_source "$unit" \
      && fault_active_cgroup_matches_source "$unit" \
      || fail "$unit effective systemd/cgroup contract differs from source"
  fi
  pid="$(systemctl --user show "$unit" -p MainPID --value)"
  [[ "$pid" =~ ^[1-9][0-9]*$ && -d "/proc/$pid" ]] \
    || fail "$unit has no live MainPID"
  executable="$(readlink -e -- "/proc/$pid/exe")" \
    || fail "$unit executable identity is unavailable"
  executable_sha="$(sha256sum "$executable" | awk '{print $1}')"
  start_ticks="$(awk '{print $22}' "/proc/$pid/stat")"
  cgroup="$(awk -F: '$1 == "0" { print $3 }' "/proc/$pid/cgroup")"
  listeners="$(ss -H -ltnp "( sport = :$port )" 2>/dev/null)"
  grep -Eq "pid=$pid([,)])" <<<"$listeners" \
    || fail "$unit MainPID does not own loopback port $port"
  fragment="$(systemctl --user show "$unit" -p FragmentPath --value)"
  fragment="$(realpath -e -- "$fragment")"
  fragment_sha="$(sha256sum "$fragment" | awk '{print $1}')"
  exec_start_sha="$(systemctl --user show "$unit" -p ExecStart --value | sha256sum | awk '{print $1}')"
  jq -n --arg contract_version trnm_fault_dependency_process_identity_v1 \
    --arg unit "$unit" --arg executable "$executable" \
    --arg executable_sha256 "$executable_sha" --arg start_ticks "$start_ticks" \
    --arg cgroup "$cgroup" --arg fragment "$fragment" \
    --arg fragment_sha256 "$fragment_sha" --arg exec_start_sha256 "$exec_start_sha" \
    --argjson pid "$pid" --argjson port "$port" \
    '{contract_version:$contract_version,unit:$unit,pid:$pid,port:$port,
      executable:$executable,executable_sha256:$executable_sha256,
      process_start_ticks:($start_ticks|tonumber),cgroup:$cgroup,
      fragment_path:$fragment,fragment_sha256:$fragment_sha256,
      exec_start_sha256:$exec_start_sha256,listener_owned_by_main_pid:true}' \
    | atomic_write "$destination"
}

capture_bound_inputs_after() {
  local root_head root_tree root_clean cex_head cex_clean
  local script_sha source_sha installed_sha cex_helper_sha release_manifest_sha
  local game_server_sha online_e2e_sha unchanged=false
  root_head="$(git -C "$ROOT_DIR" rev-parse HEAD)"
  root_tree="$(git -C "$ROOT_DIR" rev-parse 'HEAD^{tree}')"
  root_clean="$(git -C "$ROOT_DIR" status --porcelain)"
  cex_head="$(git -C "$CEX_ROOT" rev-parse HEAD)"
  cex_clean="$(git -C "$CEX_ROOT" status --porcelain)"
  script_sha="$(sha256sum "${BASH_SOURCE[0]}" | awk '{print $1}')"
  source_sha="$(sha256sum "$SOURCE_UNIT" | awk '{print $1}')"
  installed_sha="$(sha256sum "$INSTALLED_UNIT" | awk '{print $1}')"
  cex_helper_sha="$(sha256sum "$CEX_ROOT/scripts/_dev-helpers.sh" | awk '{print $1}')"
  release_manifest_sha="$(sha256sum "$RELEASE_DIR/release-manifest.json" | awk '{print $1}')"
  game_server_sha="$(sha256sum "$GAME_SERVER_BIN" | awk '{print $1}')"
  online_e2e_sha="$(sha256sum "$ONLINE_E2E_BIN" | awk '{print $1}')"
  if [[ "$root_head" == "$RELEASE_COMMIT" && "$root_tree" == "$RELEASE_TREE" \
      && -z "$root_clean" && "$cex_head" == "$CEX_COMMIT" && -z "$cex_clean" \
      && "$script_sha" == "$SCRIPT_SHA" && "$source_sha" == "$SOURCE_UNIT_SHA" \
      && "$installed_sha" == "$INSTALLED_UNIT_SHA" \
      && "$cex_helper_sha" == "$CEX_HELPER_SHA" \
      && "$release_manifest_sha" == "$RELEASE_MANIFEST_SHA" \
      && "$game_server_sha" == "$GAME_SERVER_SHA" \
      && "$online_e2e_sha" == "$ONLINE_E2E_SHA" ]]; then
    unchanged=true
    BOUND_INPUTS_UNCHANGED=1
  fi
  jq -n --arg contract_version trnm_fault_bound_inputs_after_v1 \
    --arg root_head "$root_head" --arg root_tree "$root_tree" \
    --arg cex_head "$cex_head" --arg script_sha256 "$script_sha" \
    --arg source_unit_sha256 "$source_sha" --arg installed_unit_sha256 "$installed_sha" \
    --arg cex_helper_sha256 "$cex_helper_sha" \
    --arg release_manifest_sha256 "$release_manifest_sha" \
    --arg game_server_sha256 "$game_server_sha" --arg online_e2e_sha256 "$online_e2e_sha" \
    --argjson root_clean "$([[ -z "$root_clean" ]] && echo true || echo false)" \
    --argjson cex_clean "$([[ -z "$cex_clean" ]] && echo true || echo false)" \
    --argjson unchanged "$unchanged" \
    '{contract_version:$contract_version,unchanged:$unchanged,
      root:{head:$root_head,tree:$root_tree,clean:$root_clean},
      cex:{head:$cex_head,clean:$cex_clean,helper_sha256:$cex_helper_sha256},
      hashes:{script:$script_sha256,source_unit:$source_unit_sha256,
        installed_unit:$installed_unit_sha256,release_manifest:$release_manifest_sha256,
        game_server:$game_server_sha256,online_e2e:$online_e2e_sha256}}' \
    | atomic_write "$RUN_DIR/bound-inputs-after.json"
  [[ "$unchanged" == true ]]
}

assert_loopback_listener() {
  local port="$1" listeners
  listeners="$(port_listeners "$port")"
  [[ -n "$listeners" ]] || fail "expected loopback listener on port $port"
  awk -v port="$port" '
    $4 != "127.0.0.1:" port { exit 1 }
    END { if (NR == 0) exit 1 }
  ' <<<"$listeners" || fail "port $port is not exclusively bound to IPv4 loopback"
}

assert_no_game_server_processes() {
  if pgrep -u "$(id -u)" -a -f \
      '(^|[[:space:]])[^[:space:]]*/trnm-game-server([[:space:]]|$)' \
      >"${RUN_DIR:-/dev/null}/unexpected-game-server-processes.txt" 2>/dev/null; then
    fail "another user-owned trnm-game-server process is still running"
  fi
}

qdisc_show() {
  sudo -n tc qdisc show dev lo
}

qdisc_is_default_noqueue() {
  local state
  state="$(qdisc_show 2>/dev/null || true)"
  [[ "$(wc -l <<<"$state")" -eq 1 ]] \
    && grep -Eq '^qdisc noqueue [^ ]+: root([[:space:]]|$)' <<<"$state"
}

qdisc_configuration() {
  {
    sudo -n tc qdisc show dev lo
    sudo -n tc filter show dev lo parent 1:
  }
}

qdisc_fingerprint() {
  qdisc_configuration | sha256sum | awk '{print $1}'
}

netem_packet_count() {
  sudo -n tc -s qdisc show dev lo | awk '
    /^qdisc netem 30: / { wanted=1; next }
    wanted && / Sent / {
      for (i=1; i<=NF; i++) if ($i == "pkt") { print $(i-1); exit }
    }
  '
}

configure_netem() {
  qdisc_show >"$RUN_DIR/qdisc-before.txt"
  qdisc_is_default_noqueue || fail "loopback qdisc is not the default noqueue discipline"
  sudo -n tc qdisc add dev lo root handle 1: prio bands 3
  QDISC_OWNED=1
  sudo -n tc qdisc add dev lo parent 1:3 handle 30: netem delay 50ms
  sudo -n tc filter add dev lo protocol ip parent 1: prio 30 u32 \
    match ip dport "$PROXY_PORT" 0xffff flowid 1:3
  sudo -n tc filter add dev lo protocol ip parent 1: prio 31 u32 \
    match ip sport "$PROXY_PORT" 0xffff flowid 1:3
  QDISC_CONFIGURED=1
  qdisc_configuration >"$RUN_DIR/qdisc-configured.txt"
  QDISC_FINGERPRINT="$(qdisc_fingerprint)"
  QDISC_PACKETS_BEFORE="$(netem_packet_count)"
  [[ "$QDISC_PACKETS_BEFORE" =~ ^[0-9]+$ ]] || QDISC_PACKETS_BEFORE=0
}

sample_readiness() {
  local sampled body http_ok=true
  sampled="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  body="$(readiness_json "$TEST_URL" 2>/dev/null)" || {
    http_ok=false
    body='null'
  }
  if [[ "$http_ok" == true ]] && ! jq -e 'type == "object"' >/dev/null 2>&1 <<<"$body"; then
    http_ok=false
    body='null'
  fi
  jq -cn --arg sampled_at "$sampled" --argjson http_ok "$http_ok" \
    --argjson body "$body" \
    '{sampled_at:$sampled_at,http_ok:$http_ok,body:$body}' \
    >>"$RUN_DIR/readiness-samples.jsonl"
}

monitor_readiness() {
  trap 'exit 0' INT TERM HUP
  : >"$RUN_DIR/readiness-monitor.started"
  while :; do
    if ! sample_readiness; then
      : >"$RUN_DIR/readiness-monitor.failed"
      return 1
    fi
    sleep 1
  done
}

capture_journal() {
  local output="$1" match_id="${2:-}"
  JOURNAL_ROOT="$CANONICAL_JOURNAL" JOURNAL_MATCH_ID="$match_id" python3 - <<'PY' \
    | atomic_write "$output"
import hashlib, json, os, pathlib, re, stat, unicodedata
root = pathlib.Path(os.environ["JOURNAL_ROOT"])
match_id = os.environ.get("JOURNAL_MATCH_ID", "")
UUID = re.compile(r"[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}")
SHA256 = re.compile(r"[0-9a-f]{64}")
PORTABLE = re.compile(r"[A-Za-z0-9_.:-]{1,128}")
SYSTEM_ID = re.compile(r"[1-9][0-9]{0,19}")
WAL_LSN = re.compile(r"(?:0|[1-9A-F][0-9A-F]{0,7})/(?:0|[1-9A-F][0-9A-F]{0,7})")
HIGH_WATER_KEYS = {
    "contract_version", "journal_owner_id", "instance_id", "physical_host_id",
    "match_id", "actor_generation", "actor_epoch", "tick", "next_sequence",
    "match_revision", "next_input_sequences", "phase", "receipts_replayable",
    "snapshot_hash", "recorded_at_unix_ms",
}
TOMBSTONE_KEYS = {
    "contract_version", "journal_seal_sequence", "high_water", "result_hash", "settlement_state",
    "acknowledged_at_unix_ms", "database_system_identifier",
    "database_timeline_id", "database_wal_lsn",
}
ABANDONMENT_KEYS = {
    "contract_version", "journal_seal_sequence", "high_water", "failure_reason",
    "abandoned_at_unix_ms", "database_system_identifier", "database_timeline_id",
    "database_wal_lsn",
}
LEGACY_ACK_MANIFEST_KEYS = {
    "contract_version", "journal_owner_id", "physical_host_id", "tombstone_count",
    "committed_seal_sequence", "database_system_identifier", "database_timeline_id",
    "latest_tombstone", "latest_tombstone_sha256",
}
COLD_WITNESS_MANIFEST_KEYS = {
    "contract_version", "journal_owner_id", "physical_host_id",
    "terminal_tombstone_count", "abandonment_tombstone_count",
    "committed_seal_sequence", "database_system_identifier", "database_timeline_id",
    "latest_witness", "latest_witness_sha256",
}
OWNER_KEYS = {"contract_version", "journal_owner_id", "physical_host_id"}

def exact_object(value, keys, label):
    if type(value) is not dict or set(value) != keys:
        raise ValueError(f"{label}_schema")

def positive_int(value, label, maximum=None):
    if type(value) is not int or value <= 0 or (maximum is not None and value > maximum):
        raise ValueError(label)

def nonnegative_int(value, label):
    if type(value) is not int or value < 0:
        raise ValueError(label)

def validate_database_lineage(value, label):
    system_id = value["database_system_identifier"]
    if type(system_id) is not str or SYSTEM_ID.fullmatch(system_id) is None \
            or int(system_id) > 18446744073709551615:
        raise ValueError(f"{label}_database_system_identifier")
    positive_int(value["database_timeline_id"], f"{label}_database_timeline_id", 4294967295)
    wal_lsn = value["database_wal_lsn"]
    if type(wal_lsn) is not str or WAL_LSN.fullmatch(wal_lsn) is None or wal_lsn == "0/0":
        raise ValueError(f"{label}_database_wal_lsn")

def validate_high_water(value, filename_match_id):
    exact_object(value, HIGH_WATER_KEYS, "high_water")
    if value["contract_version"] != "trnm_published_tick_high_water_v2":
        raise ValueError("high_water_contract")
    for key in ("journal_owner_id", "match_id", "actor_generation"):
        if type(value[key]) is not str or UUID.fullmatch(value[key]) is None:
            raise ValueError(key)
    for key in ("instance_id", "physical_host_id"):
        if type(value[key]) is not str or PORTABLE.fullmatch(value[key]) is None:
            raise ValueError(key)
    if value["match_id"] != filename_match_id:
        raise ValueError("high_water_filename_match")
    positive_int(value["actor_epoch"], "actor_epoch")
    for key in ("tick", "next_sequence", "match_revision"):
        nonnegative_int(value[key], key)
    cursors = value["next_input_sequences"]
    if type(cursors) is not dict or not 1 <= len(cursors) <= 64:
        raise ValueError("next_input_sequences")
    for player_id, cursor in cursors.items():
        if type(player_id) is not str or PORTABLE.fullmatch(player_id) is None:
            raise ValueError("player_id")
        nonnegative_int(cursor, "input_cursor")
    if value["phase"] not in ("running", "complete"):
        raise ValueError("phase")
    if type(value["receipts_replayable"]) is not bool:
        raise ValueError("receipts_replayable")
    if type(value["snapshot_hash"]) is not str or SHA256.fullmatch(value["snapshot_hash"]) is None:
        raise ValueError("snapshot_hash")
    positive_int(value["recorded_at_unix_ms"], "recorded_at_unix_ms")
    return value

def validate_tombstone(value, filename_match_id):
    exact_object(value, TOMBSTONE_KEYS, "ack_tombstone")
    if value["contract_version"] != "trnm_published_tick_ack_tombstone_v2":
        raise ValueError("ack_tombstone_contract")
    positive_int(value["journal_seal_sequence"], "journal_seal_sequence")
    high_water = validate_high_water(value["high_water"], filename_match_id)
    if high_water["phase"] != "complete" or high_water["receipts_replayable"] is not True:
        raise ValueError("ack_tombstone_terminal_high_water")
    if type(value["result_hash"]) is not str or SHA256.fullmatch(value["result_hash"]) is None:
        raise ValueError("result_hash")
    if value["settlement_state"] not in ("pending", "settled"):
        raise ValueError("settlement_state")
    positive_int(value["acknowledged_at_unix_ms"], "acknowledged_at_unix_ms")
    validate_database_lineage(value, "ack_tombstone")
    return value

def validate_abandonment(value, filename_match_id):
    exact_object(value, ABANDONMENT_KEYS, "abandonment_tombstone")
    if value["contract_version"] != "trnm_published_tick_abandonment_tombstone_v1":
        raise ValueError("abandonment_tombstone_contract")
    positive_int(value["journal_seal_sequence"], "journal_seal_sequence")
    high_water = validate_high_water(value["high_water"], filename_match_id)
    if high_water["phase"] != "running" or high_water["receipts_replayable"] is not True:
        raise ValueError("abandonment_tombstone_running_high_water")
    failure_reason = value["failure_reason"]
    if type(failure_reason) is not str or not failure_reason.strip() \
            or len(failure_reason.encode()) > 1024 \
            or any(unicodedata.category(char) == "Cc" for char in failure_reason):
        raise ValueError("abandonment_failure_reason")
    positive_int(value["abandoned_at_unix_ms"], "abandoned_at_unix_ms")
    validate_database_lineage(value, "abandonment_tombstone")
    return value

def validate_cold_witness(value):
    if type(value) is not dict or len(value) != 1:
        raise ValueError("cold_witness_tag")
    kind, payload = next(iter(value.items()))
    if type(payload) is not dict or type(payload.get("high_water")) is not dict:
        raise ValueError("cold_witness_payload")
    filename_match_id = payload["high_water"].get("match_id", "")
    if kind == "terminal_ack":
        return kind, validate_tombstone(payload, filename_match_id)
    if kind == "failed_closed_abandonment":
        return kind, validate_abandonment(payload, filename_match_id)
    raise ValueError("cold_witness_kind")

def validate_manifest_identity(value, label):
    if type(value["journal_owner_id"]) is not str or UUID.fullmatch(value["journal_owner_id"]) is None:
        raise ValueError(f"{label}_owner")
    if type(value["physical_host_id"]) is not str or PORTABLE.fullmatch(value["physical_host_id"]) is None:
        raise ValueError(f"{label}_host")

def validate_legacy_ack_manifest(value):
    exact_object(value, LEGACY_ACK_MANIFEST_KEYS, "legacy_ack_manifest")
    if value["contract_version"] != "trnm_published_tick_ack_manifest_v1":
        raise ValueError("legacy_ack_manifest_contract")
    validate_manifest_identity(value, "legacy_ack_manifest")
    nonnegative_int(value["tombstone_count"], "tombstone_count")
    nonnegative_int(value["committed_seal_sequence"], "committed_seal_sequence")
    if value["tombstone_count"] != value["committed_seal_sequence"]:
        raise ValueError("legacy_ack_manifest_sequence")
    if value["tombstone_count"] == 0:
        if any(value[key] is not None for key in (
            "database_system_identifier", "database_timeline_id",
            "latest_tombstone", "latest_tombstone_sha256",
        )):
            raise ValueError("legacy_ack_manifest_empty_state")
        return value
    system_id = value["database_system_identifier"]
    if type(system_id) is not str or SYSTEM_ID.fullmatch(system_id) is None \
            or int(system_id) > 18446744073709551615:
        raise ValueError("legacy_ack_manifest_system_identifier")
    positive_int(value["database_timeline_id"], "legacy_ack_manifest_timeline", 4294967295)
    latest = value["latest_tombstone"]
    if type(latest) is not dict or type(latest.get("high_water")) is not dict:
        raise ValueError("legacy_ack_manifest_latest")
    latest = validate_tombstone(latest, latest["high_water"].get("match_id", ""))
    if latest["journal_seal_sequence"] > value["committed_seal_sequence"] \
            or latest["high_water"]["journal_owner_id"] != value["journal_owner_id"] \
            or latest["high_water"]["physical_host_id"] != value["physical_host_id"] \
            or latest["database_system_identifier"] != system_id \
            or latest["database_timeline_id"] != value["database_timeline_id"]:
        raise ValueError("legacy_ack_manifest_latest_identity")
    if type(value["latest_tombstone_sha256"]) is not str \
            or SHA256.fullmatch(value["latest_tombstone_sha256"]) is None:
        raise ValueError("legacy_ack_manifest_latest_sha")
    return value

def validate_cold_witness_manifest(value):
    exact_object(value, COLD_WITNESS_MANIFEST_KEYS, "cold_witness_manifest")
    if value["contract_version"] != "trnm_published_tick_cold_witness_manifest_v2":
        raise ValueError("cold_witness_manifest_contract")
    validate_manifest_identity(value, "cold_witness_manifest")
    nonnegative_int(value["terminal_tombstone_count"], "terminal_tombstone_count")
    nonnegative_int(value["abandonment_tombstone_count"], "abandonment_tombstone_count")
    nonnegative_int(value["committed_seal_sequence"], "committed_seal_sequence")
    total = value["terminal_tombstone_count"] + value["abandonment_tombstone_count"]
    if total != value["committed_seal_sequence"]:
        raise ValueError("cold_witness_manifest_sequence")
    if total == 0:
        if any(value[key] is not None for key in (
            "database_system_identifier", "database_timeline_id",
            "latest_witness", "latest_witness_sha256",
        )):
            raise ValueError("cold_witness_manifest_empty_state")
        return value
    system_id = value["database_system_identifier"]
    if type(system_id) is not str or SYSTEM_ID.fullmatch(system_id) is None \
            or int(system_id) > 18446744073709551615:
        raise ValueError("cold_witness_manifest_system_identifier")
    positive_int(value["database_timeline_id"], "cold_witness_manifest_timeline", 4294967295)
    kind, latest = validate_cold_witness(value["latest_witness"])
    if latest["journal_seal_sequence"] != value["committed_seal_sequence"] \
            or latest["high_water"]["journal_owner_id"] != value["journal_owner_id"] \
            or latest["high_water"]["physical_host_id"] != value["physical_host_id"] \
            or latest["database_system_identifier"] != system_id \
            or latest["database_timeline_id"] != value["database_timeline_id"]:
        raise ValueError("cold_witness_manifest_latest_identity")
    if kind == "terminal_ack" and value["terminal_tombstone_count"] == 0:
        raise ValueError("cold_witness_manifest_terminal_count")
    if kind == "failed_closed_abandonment" and value["abandonment_tombstone_count"] == 0:
        raise ValueError("cold_witness_manifest_abandonment_count")
    if type(value["latest_witness_sha256"]) is not str \
            or SHA256.fullmatch(value["latest_witness_sha256"]) is None:
        raise ValueError("cold_witness_manifest_latest_sha")
    return value

def validate_ack_manifest(value):
    if type(value) is not dict:
        raise ValueError("ack_manifest_schema")
    if value.get("contract_version") == "trnm_published_tick_ack_manifest_v1":
        return validate_legacy_ack_manifest(value)
    return validate_cold_witness_manifest(value)

def journal_inventory_paths(journal_root):
    # Keep relative names in evidence so a future deterministic shard layout is
    # bound without weakening the inventory digest.
    if not journal_root.is_dir():
        return []
    return sorted(
        (p for p in journal_root.rglob("*") if p.is_file() or p.is_symlink()),
        key=lambda p: p.relative_to(journal_root).as_posix(),
    )

items, directories, records, tombstones, abandonments, decode_errors = [], [], [], [], [], []
owner_manifest = None
ack_manifest = None
if root.is_dir():
    for directory in sorted(
        (p for p in root.rglob("*") if p.is_dir() and not p.is_symlink()),
        key=lambda p: p.relative_to(root).as_posix(),
    ):
        meta = directory.lstat()
        directories.append({
            "name": directory.relative_to(root).as_posix(),
            "mode": format(stat.S_IMODE(meta.st_mode), "04o"),
            "uid": meta.st_uid,
            "gid": meta.st_gid,
            "symlink": False,
        })
for path in journal_inventory_paths(root):
    meta = path.lstat()
    relative_name = path.relative_to(root).as_posix()
    item = {
        "name": relative_name,
        "mode": format(stat.S_IMODE(meta.st_mode), "04o"),
        "nlink": meta.st_nlink,
        "uid": meta.st_uid,
        "gid": meta.st_gid,
        "regular": stat.S_ISREG(meta.st_mode),
        "symlink": stat.S_ISLNK(meta.st_mode),
    }
    if item["regular"]:
        data = path.read_bytes()
        item["sha256"] = hashlib.sha256(data).hexdigest()
        item["bytes"] = len(data)
    hot_match = re.fullmatch(rf"published-({UUID.pattern})\.json", path.name)
    tombstone_match = re.fullmatch(rf"acknowledged-({UUID.pattern})\.json", path.name)
    abandonment_match = re.fullmatch(rf"abandoned-({UUID.pattern})\.json", path.name)
    if hot_match or tombstone_match or abandonment_match or relative_name in (
            ".published-tick-owner.json", ".published-tick-ack-manifest.json"):
        try:
            if not item["regular"] or item["symlink"]:
                raise ValueError("not_private_regular_file")
            value = json.loads(data)
            if hot_match:
                record = validate_high_water(value, hot_match.group(1)).copy()
                record.update(name=relative_name, sha256=item["sha256"])
                records.append(record)
            elif tombstone_match:
                simple = tombstone_match.group(1).replace("-", "")
                expected = f"acknowledged/{simple[0:2]}/{simple[2:4]}/{path.name}"
                if relative_name != expected:
                    raise ValueError("ack_tombstone_shard")
                tombstone = validate_tombstone(value, tombstone_match.group(1)).copy()
                tombstone.update(name=relative_name, sha256=item["sha256"])
                tombstones.append(tombstone)
            elif abandonment_match:
                simple = abandonment_match.group(1).replace("-", "")
                expected = f"abandoned/{simple[0:2]}/{simple[2:4]}/{path.name}"
                if relative_name != expected:
                    raise ValueError("abandonment_tombstone_shard")
                abandonment = validate_abandonment(value, abandonment_match.group(1)).copy()
                abandonment.update(name=relative_name, sha256=item["sha256"])
                abandonments.append(abandonment)
            elif relative_name == ".published-tick-ack-manifest.json":
                ack_manifest = validate_ack_manifest(value)
            else:
                exact_object(value, OWNER_KEYS, "owner_manifest")
                if value["contract_version"] != "trnm_published_tick_journal_owner_v1" \
                        or type(value["journal_owner_id"]) is not str \
                        or UUID.fullmatch(value["journal_owner_id"]) is None \
                        or type(value["physical_host_id"]) is not str \
                        or PORTABLE.fullmatch(value["physical_host_id"]) is None:
                    raise ValueError("owner_manifest")
                owner_manifest = value
        except Exception as exc:
            error = {"name": relative_name, "decode_error": f"{type(exc).__name__}:{exc}"}
            decode_errors.append(error)
            if hot_match:
                records.append(error)
            elif tombstone_match:
                tombstones.append(error)
            elif abandonment_match:
                abandonments.append(error)
    items.append(item)

def validate_manifest_inventory(manifest):
    if manifest["contract_version"] == "trnm_published_tick_ack_manifest_v1":
        terminal_count = manifest["tombstone_count"]
        abandonment_count = 0
        latest_kind = "terminal_ack" if manifest["latest_tombstone"] is not None else None
        latest_payload = manifest["latest_tombstone"]
        latest_sha256 = manifest["latest_tombstone_sha256"]
    else:
        terminal_count = manifest["terminal_tombstone_count"]
        abandonment_count = manifest["abandonment_tombstone_count"]
        if manifest["latest_witness"] is None:
            latest_kind, latest_payload = None, None
        else:
            latest_kind, latest_payload = validate_cold_witness(manifest["latest_witness"])
        latest_sha256 = manifest["latest_witness_sha256"]
    if terminal_count != len(tombstones) or abandonment_count != len(abandonments):
        raise ValueError("manifest_inventory_count")
    if latest_payload is None:
        return
    latest_match_id = latest_payload["high_water"]["match_id"]
    simple = latest_match_id.replace("-", "")
    if latest_kind == "terminal_ack":
        expected_name = f"acknowledged/{simple[0:2]}/{simple[2:4]}/acknowledged-{latest_match_id}.json"
        candidates = tombstones
    else:
        expected_name = f"abandoned/{simple[0:2]}/{simple[2:4]}/abandoned-{latest_match_id}.json"
        candidates = abandonments
    actual = [record for record in candidates if record.get("name") == expected_name]
    if len(actual) != 1 or "decode_error" in actual[0]:
        raise ValueError("manifest_latest_witness_path")
    actual_payload = {key: value for key, value in actual[0].items() if key not in ("name", "sha256")}
    if actual_payload != latest_payload or actual[0].get("sha256") != latest_sha256:
        raise ValueError("manifest_latest_witness_payload_or_sha")

if ack_manifest is not None:
    try:
        validate_manifest_inventory(ack_manifest)
    except Exception as exc:
        decode_errors.append({
            "name": ".published-tick-ack-manifest.json",
            "decode_error": f"{type(exc).__name__}:{exc}",
        })
        ack_manifest = None

digest_source = "\n".join(
    f'{x["name"]}\t{x.get("sha256", "-")}\t{x["mode"]}\t{x["nlink"]}\t{x["uid"]}\t{x["gid"]}'
    for x in items
).encode()
owner = next((x for x in items if x["name"] == ".published-tick-owner.json"), None)
root_meta = root.lstat() if root.exists() else None
print(json.dumps({
    "contract_version": "trnm_published_tick_journal_inventory_v2",
    "root": str(root.resolve(strict=False)),
    "root_exists": root.is_dir(),
    "root_mode": format(stat.S_IMODE(root_meta.st_mode), "04o") if root_meta else None,
    "root_uid": root_meta.st_uid if root_meta else None,
    "inventory_digest": hashlib.sha256(digest_source).hexdigest(),
    "owner_manifest_sha256": owner.get("sha256") if owner else None,
    "owner_manifest": owner_manifest,
    "ack_manifest": ack_manifest,
    "ack_manifest_sha256": next(
        (x.get("sha256") for x in items if x["name"] == ".published-tick-ack-manifest.json"), None
    ),
    "record_count": len(records),
    "hot_record_count": len(records),
    "ack_tombstone_count": len(tombstones),
    "abandonment_tombstone_count": len(abandonments),
    "cold_witness_count": len(tombstones) + len(abandonments),
    "run_match_id": match_id or None,
    "run_match_record_present": any(r.get("match_id") == match_id for r in records),
    "run_match_hot_record_present": any(r.get("match_id") == match_id for r in records),
    "run_match_ack_tombstone_present": any(
        r.get("high_water", {}).get("match_id") == match_id for r in tombstones
    ),
    "run_match_abandonment_tombstone_present": any(
        r.get("high_water", {}).get("match_id") == match_id for r in abandonments
    ),
    "run_match_cold_witness_present": any(
        r.get("high_water", {}).get("match_id") == match_id
        for r in tombstones + abandonments
    ),
    "decode_error_count": len(decode_errors),
    "decode_errors": decode_errors,
    "records": records,
    "ack_tombstones": tombstones,
    "abandonment_tombstones": abandonments,
    "directories": directories,
    "items": items,
}, separators=(",", ":")))
PY
}

database_running_count() {
  cex_psql_stdin -At -v ON_ERROR_STOP=1 -c \
    "select count(*) from trnm_online_matches where phase = 'running'"
}

capture_database_terminal() {
  local output="$1"
  cex_psql_stdin -At -v ON_ERROR_STOP=1 -c "
    with candidates as (
      select m.* from trnm_online_matches m
      where exists (
        select 1 from trnm_online_match_members h
        where h.match_id=m.match_id and h.player_id='$HOST_PLAYER'
      ) and exists (
        select 1 from trnm_online_match_members g
        where g.match_id=m.match_id and g.player_id='$GUEST_PLAYER'
      )
    ), selected as (
      select * from candidates order by created_at desc limit 1
    ), command_stats as (
      select count(*)::bigint as command_count,
             count(distinct sequence)::bigint as distinct_sequences,
             count(distinct command_id)::bigint as distinct_command_ids,
             count(distinct (player_id, input_sequence))::bigint as distinct_player_inputs,
             min(sequence) as min_sequence, max(sequence) as max_sequence,
             count(*) filter (where post_simulation_json is null)::bigint as missing_post_simulation
      from trnm_online_commands where match_id=(select match_id from selected)
    ), cursors as (
      select coalesce(jsonb_object_agg(mm.player_id, to_jsonb(mm.next_input_sequence)), '{}'::jsonb) value,
             bool_and(mm.next_input_sequence = (
               select count(*) from trnm_online_commands c
               where c.match_id=mm.match_id and c.player_id=mm.player_id
             )) exact
      from trnm_online_match_members mm
      where mm.match_id=(select match_id from selected)
    ), marker as (
      select a.* from trnm_online_terminal_publication_acks a
      where a.match_id=(select match_id from selected)
    )
    select json_build_object(
      'contract_version','trnm_authority_terminal_database_evidence_v2',
      'match_count',(select count(*) from candidates),
      'match_id',(select match_id::text from selected),
      'phase',(select phase from selected),
      'settlement_state',(select settlement_state from selected),
      'authoritative_tick',(select authoritative_tick from selected),
      'next_sequence',(select next_sequence from selected),
      'checkpoint_sequence',(select checkpoint_sequence from selected),
      'match_revision',(select match_revision from selected),
      'snapshot_hash',(select snapshot_hash from selected),
      'result_hash',(select result_hash from selected),
      'assigned_instance_id',(select assigned_instance_id from selected),
      'assigned_instance_epoch',(select assigned_instance_epoch from selected),
      'assigned_physical_host_id',(select assigned_physical_host_id from selected),
      'terminal_publication_actor_generation',(select terminal_publication_actor_generation::text from selected),
      'command_count',(select command_count from command_stats),
      'command_sequences_contiguous',coalesce((select command_count = (select next_sequence from selected)
        and distinct_sequences=command_count and distinct_command_ids=command_count
        and (command_count=0 or (min_sequence=0 and max_sequence=command_count-1)) from command_stats),false),
      'player_input_sequences_unique',coalesce((select distinct_player_inputs=command_count from command_stats),false),
      'missing_post_simulation',(select missing_post_simulation from command_stats),
      'member_cursors_exact',coalesce((select exact from cursors),false),
      'member_cursors',(select value from cursors),
      'terminal_marker_count',(select count(*) from marker),
      'ack_actor_generation',(select actor_generation::text from marker),
      'ack_instance_id',(select instance_id from marker),
      'ack_actor_epoch',(select actor_epoch from marker),
      'ack_physical_host_id',(select physical_host_id from marker),
      'ack_authoritative_tick',(select authoritative_tick from marker),
      'ack_next_sequence',(select next_sequence from marker),
      'ack_match_revision',(select match_revision from marker),
      'ack_next_input_sequences',(select next_input_sequences from marker),
      'ack_snapshot_hash',(select snapshot_hash from marker),
      'ack_phase',(select phase from marker),
      'ack_result_hash',(select result_hash from marker),
      'ack_settlement_state',(select published_settlement_state from marker),
      'acknowledged_at_unix_ms',(select floor(extract(epoch from acknowledged_at) * 1000)::bigint from marker),
      'database_system_identifier',(select system_identifier::text from pg_control_system()),
      'database_timeline_id',(select timeline_id from pg_control_checkpoint()),
      'database_current_wal_lsn',pg_current_wal_lsn()::text,
      'terminal_marker_exact',coalesce((select
        a.actor_generation=s.terminal_publication_actor_generation
        and a.instance_id=s.assigned_instance_id
        and a.actor_epoch=s.assigned_instance_epoch
        and a.physical_host_id=s.assigned_physical_host_id
        and a.authoritative_tick=s.authoritative_tick
        and a.next_sequence=s.next_sequence
        and a.match_revision=s.match_revision
        and a.next_input_sequences=(select value from cursors)
        and a.snapshot_hash=s.snapshot_hash
        and a.phase='complete'
        and a.result_hash=s.result_hash
        and (a.published_settlement_state=s.settlement_state
          or (a.published_settlement_state='pending' and s.settlement_state='settled'))
        from marker a cross join selected s),false)
    )" | atomic_write "$output"
}

capture_fleet_state() {
  cex_psql_stdin -At -v ON_ERROR_STOP=1 -c "
    select json_build_object(
      'instance_id',instance_id,'instance_epoch',instance_epoch,
      'physical_host_id',physical_host_id,'status',status,
      'active_matches',active_matches,'lease_expires_at',lease_expires_at,
      'open_run_match_count',(
        select count(*) from trnm_online_matches m
        where m.phase in ('waiting','running')
          and m.assigned_instance_id='$TEST_INSTANCE_ID'
          and exists (select 1 from trnm_online_match_members mm
                      where mm.match_id=m.match_id
                        and mm.player_id in ('$HOST_PLAYER','$GUEST_PLAYER'))
      ))
    from trnm_online_fleet_instances where instance_id='$TEST_INSTANCE_ID'" \
    | atomic_write "$1"
}

maintenance_candidate_match_ids() {
  cex_psql_stdin -At -v ON_ERROR_STOP=1 -c "
    /* trnm_online_maintenance_candidates_v1: discovery only; mutations are exact UUID CLI calls */
    select m.match_id::text
    from trnm_online_matches m
    where m.phase in ('waiting','running','failed_closed')
      and (
        (m.phase = 'waiting'
          and m.assigned_instance_id is null
          and m.assigned_instance_epoch = 0
          and m.assigned_physical_host_id is null)
        or (m.phase in ('running','failed_closed')
          and (m.assigned_instance_id='$TEST_INSTANCE_ID'
            or (m.assigned_instance_id is null
              and m.assigned_instance_epoch = 0
              and m.assigned_physical_host_id is null)))
      )
      and exists (select 1 from trnm_online_match_members mm
                  where mm.match_id=m.match_id
                    and mm.player_id in ('$HOST_PLAYER','$GUEST_PLAYER'))
    order by m.match_id"
}

maintenance_report_is_exact_and_atomic() {
  local report="$1" match_id="$2"
  jq -e --arg match_id "$match_id" '
    (keys | sort) == ([
      "adoption_contract","cold_witness_sealed","contract_version","final_phase",
      "hot_witness_present_before","legacy_adoption","local_marker_state",
      "match_id","previous_phase","selector","status","transition_atomic",
      "waiting_db_only"
    ] | sort)
    and .contract_version == "trnm_online_maintenance_fail_close_v1"
    and .status == "completed"
    and .match_id == $match_id
    and .selector == "exact_match_id"
    and .transition_atomic == true
    and .legacy_adoption == false
    and .adoption_contract == null
    and (.previous_phase == "waiting" or .previous_phase == "running"
      or .previous_phase == "failed_closed")
    and .final_phase == "failed_closed"
    and (.waiting_db_only | type) == "boolean"
    and (.hot_witness_present_before | type) == "boolean"
    and (.cold_witness_sealed | type) == "boolean"
    and (
      if .previous_phase == "waiting" then
        .waiting_db_only == true
        and .hot_witness_present_before == false
        and .cold_witness_sealed == false
        and .local_marker_state == null
      elif .previous_phase == "running" then
        .waiting_db_only == false
        and .hot_witness_present_before == true
        and .cold_witness_sealed == true
        and .local_marker_state == "sealed"
      else
        ((.waiting_db_only == true
          and .hot_witness_present_before == false
          and .cold_witness_sealed == false
          and .local_marker_state == null)
        or (.waiting_db_only == false
          and .cold_witness_sealed == true
          and .local_marker_state == "sealed"))
      end
    )' "$report" >/dev/null
}

capture_database_after_maintenance() {
  local output="$1" match_id="$2"
  cex_psql_stdin -At -v ON_ERROR_STOP=1 -c "
    /* trnm_online_maintenance_database_evidence_v1: exact post-maintenance evidence */
    with selected as (
      select * from trnm_online_matches
       where match_id='$match_id'::uuid
    ), cursors as (
      select coalesce(jsonb_object_agg(
               member.player_id,
               to_jsonb(member.next_input_sequence)
               order by member.player_id
             ), '{}'::jsonb) as value
        from trnm_online_match_members member
       where member.match_id='$match_id'::uuid
    ), marker as (
      select * from trnm_online_failed_closed_abandonment_markers
       where match_id='$match_id'::uuid
    ), summary as (
      select * from trnm_online_local_cold_witness_summaries
       where physical_host_id='$PHYSICAL_HOST_ID'
    ), actual as (
      select
        (select count(*)::bigint
           from trnm_online_terminal_publication_acks
          where physical_host_id='$PHYSICAL_HOST_ID') as terminal_total_count,
        (select count(*)::bigint
           from trnm_online_terminal_publication_acks
          where physical_host_id='$PHYSICAL_HOST_ID'
            and local_tombstone_state='sealed') as terminal_sealed_count,
        (select count(*)::bigint
           from trnm_online_failed_closed_abandonment_markers
          where physical_host_id='$PHYSICAL_HOST_ID') as abandonment_total_count,
        (select count(*)::bigint
           from trnm_online_failed_closed_abandonment_markers
          where physical_host_id='$PHYSICAL_HOST_ID'
            and local_tombstone_state='sealed') as abandonment_sealed_count
    )
    select json_build_object(
      'contract_version','trnm_online_maintenance_database_evidence_v1',
      'status','captured',
      'match_count',(select count(*) from selected),
      'match',(select json_build_object(
        'match_id',match_id::text,
        'phase',phase,
        'settlement_state',settlement_state,
        'failure_reason',failure_reason,
        'assigned_instance_id',assigned_instance_id,
        'assigned_instance_epoch',assigned_instance_epoch,
        'assigned_physical_host_id',assigned_physical_host_id,
        'authoritative_tick',authoritative_tick,
        'next_sequence',next_sequence,
        'checkpoint_sequence',checkpoint_sequence,
        'match_revision',match_revision,
        'next_input_sequences',(select value from cursors),
        'snapshot_hash',snapshot_hash,
        'terminal_publication_state',terminal_publication_state,
        'terminal_stage_present',(
          terminal_stage_simulation_json is not null
          or terminal_stage_result_json is not null
          or terminal_stage_result_hash is not null
          or terminal_stage_snapshot_hash is not null
          or terminal_stage_authoritative_tick is not null
          or terminal_stage_next_sequence is not null
          or terminal_stage_match_revision is not null
          or terminal_staged_at is not null
        ),
        'result_present',(result_json is not null or result_hash is not null)
      ) from selected),
      'terminal_marker_count',(
        select count(*) from trnm_online_terminal_publication_acks
         where match_id='$match_id'::uuid
      ),
      'abandonment_marker_count',(select count(*) from marker),
      'abandonment_marker',(select json_build_object(
        'match_id',match_id::text,
        'journal_owner_id',journal_owner_id::text,
        'actor_generation',actor_generation::text,
        'instance_id',instance_id,
        'actor_epoch',actor_epoch,
        'physical_host_id',physical_host_id,
        'authoritative_tick',authoritative_tick,
        'next_sequence',next_sequence,
        'match_revision',match_revision,
        'next_input_sequences',next_input_sequences,
        'snapshot_hash',snapshot_hash,
        'failure_reason',failure_reason,
        'abandoned_at_unix_ms',floor(extract(epoch from abandoned_at) * 1000)::bigint,
        'local_tombstone_state',local_tombstone_state
      ) from marker),
      'summary_row_count',(select count(*) from summary),
      'summary',(select json_build_object(
        'physical_host_id',physical_host_id,
        'terminal_total_count',terminal_total_count,
        'terminal_sealed_count',terminal_sealed_count,
        'abandonment_total_count',abandonment_total_count,
        'abandonment_sealed_count',abandonment_sealed_count
      ) from summary),
      'actual_host_counts',(select json_build_object(
        'terminal_total_count',terminal_total_count,
        'terminal_sealed_count',terminal_sealed_count,
        'abandonment_total_count',abandonment_total_count,
        'abandonment_sealed_count',abandonment_sealed_count
      ) from actual)
    )" | atomic_write "$output"
}

maintenance_post_database_is_exact() {
  local report="$1" database="$2" journal="$3" match_id="$4"
  jq -e --arg match_id "$match_id" \
    --arg reason "$MAINTENANCE_FAILURE_REASON" \
    --arg host "$PHYSICAL_HOST_ID" \
    --arg instance "$TEST_INSTANCE_ID" \
    --slurpfile report "$report" --slurpfile journal "$journal" '
    $report[0] as $report
    | $journal[0] as $journal
    | .contract_version == "trnm_online_maintenance_database_evidence_v1"
    and .status == "captured"
    and .match_count == 1
    and .match.match_id == $match_id
    and .match.phase == "failed_closed"
    and .match.settlement_state == "failed_closed"
    and .match.failure_reason == $reason
    and .match.terminal_publication_state == "pending"
    and .match.terminal_stage_present == false
    and .match.result_present == false
    and .terminal_marker_count == 0
    and (.summary_row_count == 0 or .summary_row_count == 1)
    and (.actual_host_counts | type) == "object"
    and all(.actual_host_counts[]; type == "number" and . >= 0)
    and (
      if .summary_row_count == 0 then
        .summary == null
        and ([.actual_host_counts[]] | add) == 0
      else
        .summary.physical_host_id == $host
        and .summary.terminal_total_count
          == .actual_host_counts.terminal_total_count
        and .summary.terminal_sealed_count
          == .actual_host_counts.terminal_sealed_count
        and .summary.abandonment_total_count
          == .actual_host_counts.abandonment_total_count
        and .summary.abandonment_sealed_count
          == .actual_host_counts.abandonment_sealed_count
      end
    )
    and (
      if $report.waiting_db_only then
        .abandonment_marker_count == 0
        and .abandonment_marker == null
        and .match.assigned_instance_id == null
        and .match.assigned_instance_epoch == 0
        and .match.assigned_physical_host_id == null
      else
        .abandonment_marker_count == 1
        and (.abandonment_marker | type) == "object"
        and .abandonment_marker.match_id == $match_id
        and .abandonment_marker.failure_reason == $reason
        and .abandonment_marker.local_tombstone_state == "sealed"
        and .abandonment_marker.physical_host_id == $host
        and .abandonment_marker.instance_id == $instance
        and .match.assigned_instance_id == .abandonment_marker.instance_id
        and .match.assigned_instance_epoch == .abandonment_marker.actor_epoch
        and .match.assigned_physical_host_id
          == .abandonment_marker.physical_host_id
        and .match.authoritative_tick
          == .abandonment_marker.authoritative_tick
        and .match.next_sequence == .abandonment_marker.next_sequence
        and .match.checkpoint_sequence == .abandonment_marker.next_sequence
        and .match.match_revision == .abandonment_marker.match_revision
        and .match.next_input_sequences
          == .abandonment_marker.next_input_sequences
        and .match.snapshot_hash == .abandonment_marker.snapshot_hash
        and .summary_row_count == 1
        and .summary.abandonment_total_count >= 1
        and .summary.abandonment_sealed_count >= 1
        and ([ $journal.abandonment_tombstones[]
          | select(.high_water.match_id == $match_id) ] | length) == 1
        and ([ $journal.abandonment_tombstones[]
          | select(.high_water.match_id == $match_id) ][0] as $cold
          | .abandonment_marker.journal_owner_id
              == $cold.high_water.journal_owner_id
          and .abandonment_marker.actor_generation
              == $cold.high_water.actor_generation
          and .abandonment_marker.instance_id == $cold.high_water.instance_id
          and .abandonment_marker.actor_epoch == $cold.high_water.actor_epoch
          and .abandonment_marker.physical_host_id
              == $cold.high_water.physical_host_id
          and .abandonment_marker.authoritative_tick == $cold.high_water.tick
          and .abandonment_marker.next_sequence == $cold.high_water.next_sequence
          and .abandonment_marker.match_revision == $cold.high_water.match_revision
          and .abandonment_marker.next_input_sequences
              == $cold.high_water.next_input_sequences
          and .abandonment_marker.snapshot_hash == $cold.high_water.snapshot_hash
          and .abandonment_marker.failure_reason == $cold.failure_reason
          and .abandonment_marker.abandoned_at_unix_ms
              == $cold.abandoned_at_unix_ms)
      end
    )' "$database" >/dev/null
}

maintenance_post_journal_is_exact() {
  local report="$1" journal="$2" match_id="$3"
  jq -e --arg match_id "$match_id" --slurpfile report "$report" '
    $report[0] as $report
    | .decode_error_count == 0
    and .run_match_hot_record_present == false
    and ([.records[] | select(.match_id == $match_id)] | length) == 0
    and (
      if $report.waiting_db_only then
        .run_match_cold_witness_present == false
        and .run_match_ack_tombstone_present == false
        and .run_match_abandonment_tombstone_present == false
        and ([.ack_tombstones[] | select(.high_water.match_id == $match_id)] | length) == 0
        and ([.abandonment_tombstones[]
          | select(.high_water.match_id == $match_id)] | length) == 0
      else
        .run_match_cold_witness_present == true
        and .run_match_ack_tombstone_present == false
        and .run_match_abandonment_tombstone_present == true
        and ([.ack_tombstones[] | select(.high_water.match_id == $match_id)] | length) == 0
        and ([.abandonment_tombstones[]
          | select(.high_water.match_id == $match_id)] | length) == 1
        and .ack_manifest.contract_version
          == "trnm_published_tick_cold_witness_manifest_v2"
        and (.ack_manifest.latest_witness | keys) == ["failed_closed_abandonment"]
        and .ack_manifest.latest_witness.failed_closed_abandonment.high_water.match_id
          == $match_id
        and ([.abandonment_tombstones[]
          | select(.high_water.match_id == $match_id)][0] as $abandonment
          | .ack_manifest.latest_witness.failed_closed_abandonment
              == ($abandonment | del(.name, .sha256))
          and .ack_manifest.latest_witness_sha256 == $abandonment.sha256)
      end
    )' "$journal" >/dev/null
}

create_identity() {
  local label="$1" account player recovery session
  account="$(curl -fsS --max-time 10 "$LEDGER_URL/v1/accounts" \
    -H "x-admin-token: $IDENTITY_ADMIN_TOKEN" -H 'content-type: application/json' \
    --data-binary "$(jq -cn --arg label "$label" \
      '{org_id:"00000000-0000-0000-0000-00000000ce01",account_type:("fault-"+$label),currency_unit:"credit",initial_balance:0}')" \
    | jq -er .account_id)"
  player="$RUN_ID-$label"
  recovery="recovery-$RUN_ID-$label-012345678901234567890123"
  curl -fsS --max-time 10 "$LEDGER_URL/v1/trnm/identity/register" \
    -H "x-admin-token: $IDENTITY_ADMIN_TOKEN" -H 'content-type: application/json' \
    --data-binary "$(jq -cn --arg player "$player" --arg account "$account" \
      --arg recovery "$recovery" \
      '{player_id:$player,account_id:$account,recovery_key:$recovery}')" >/dev/null
  session="$(curl -fsS --max-time 10 "$LEDGER_URL/v1/trnm/identity/session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg recovery "$recovery" --arg device "$RUN_ID-$label-device" \
      '{player_id:$player,recovery_key:$recovery,device_id:$device,lifetime_seconds:3600}')" \
    | jq -er .session_token)"
  printf '%s\t%s\t%s\n' "$player" "$account" "$session"
}

register_cleanup_process() {
  local pid="$1" group_mode="$2" expected_executable="$3"
  local attempt stat_line remainder pgid exe cgroup expected_exe
  expected_exe="$(stat -Lc '%d:%i' -- "$expected_executable")" || return 1
  for ((attempt=0; attempt<50; attempt++)); do
    if [[ -r "/proc/$pid/stat" && -r "/proc/$pid/cgroup" ]]; then
      IFS= read -r stat_line <"/proc/$pid/stat" || continue
      remainder="${stat_line##*) }"
      pgid="$(awk '{print $3}' <<<"$remainder")"
      if [[ "$group_mode" != group || "$pgid" == "$pid" ]]; then
        exe="$(stat -Lc '%d:%i' "/proc/$pid/exe" 2>/dev/null)" || continue
        if [[ "$CONTRACT_MODE" == 0 && "$exe" != "$expected_exe" ]]; then
          sleep 0.02
          continue
        fi
        cgroup="$(awk -F: '$1 == "0" {print $3}' "/proc/$pid/cgroup")" || continue
        if [[ "$CONTRACT_MODE" == 0 && "$cgroup" != "$RESOURCE_CGROUP" ]]; then
          continue
        fi
        CLEANUP_PROCESS_START["$pid"]="$(awk '{print $20}' <<<"$remainder")"
        CLEANUP_PROCESS_EXE["$pid"]="$exe"
        CLEANUP_PROCESS_CGROUP["$pid"]="$cgroup"
        CLEANUP_PROCESS_PGID["$pid"]="$pgid"
        return 0
      fi
    fi
    sleep 0.02
  done
  fail "could not bind cleanup identity for process $pid"
}

cleanup_process_identity_matches() {
  local pid="$1" stat_line remainder
  [[ -n "${CLEANUP_PROCESS_START[$pid]:-}" \
      && -r "/proc/$pid/stat" && -r "/proc/$pid/cgroup" ]] || return 1
  IFS= read -r stat_line <"/proc/$pid/stat" || return 1
  remainder="${stat_line##*) }"
  [[ "$(awk '{print $20}' <<<"$remainder")" == "${CLEANUP_PROCESS_START[$pid]}" \
      && "$(stat -Lc '%d:%i' "/proc/$pid/exe" 2>/dev/null)" == \
        "${CLEANUP_PROCESS_EXE[$pid]}" \
      && "$(awk -F: '$1 == "0" {print $3}' "/proc/$pid/cgroup")" == \
        "${CLEANUP_PROCESS_CGROUP[$pid]}" \
      && "$(awk '{print $3}' <<<"$remainder")" == "${CLEANUP_PROCESS_PGID[$pid]}" ]]
}

terminate_group() {
  local pid="$1" label="$2" grace_attempts=40
  [[ -n "$pid" ]] || return 0
  # The Authority drains HTTP for up to 12 seconds and actor/checkpoint work
  # for up to 10 seconds. Do not turn a graceful shutdown into SIGKILL merely
  # because the fault profile itself adds database latency.
  [[ "$label" == standalone-server ]] && grace_attempts=300
  if kill -0 "$pid" >/dev/null 2>&1; then
    cleanup_process_identity_matches "$pid" \
      || { fail "$label PID was reused or changed identity; refusing to signal it"; return 1; }
    kill -TERM -- "-$pid" >/dev/null 2>&1 || kill -TERM "$pid" >/dev/null 2>&1 || true
    local attempt
    for ((attempt=0; attempt<grace_attempts; attempt++)); do
      kill -0 "$pid" >/dev/null 2>&1 || break
      sleep 0.1
    done
    if kill -0 "$pid" >/dev/null 2>&1; then
      cleanup_process_identity_matches "$pid" \
        || { fail "$label PID changed identity before SIGKILL"; return 1; }
      kill -KILL -- "-$pid" >/dev/null 2>&1 || kill -KILL "$pid" >/dev/null 2>&1 || true
    fi
  fi
  wait "$pid" >/dev/null 2>&1 || true
  if kill -0 "$pid" >/dev/null 2>&1; then
    fail "$label process did not stop"
  fi
  return 0
}

scan_and_redact_secrets() {
  local secret mode file scanner_status found=0
  local -a secret_specs=(
    "literal:$IDENTITY_ADMIN_TOKEN"
    "literal:$HOST_SESSION"
    "literal:$GUEST_SESSION"
  )
  if (( ${#DATABASE_PASSWORD} >= 12 )); then
    secret_specs+=("literal:$DATABASE_PASSWORD")
  else
    # Short database passwords can also be ordinary public vocabulary (for
    # example the PostgreSQL service/schema name). Scan only credential
    # contexts so public evidence keys remain intact while connection URLs,
    # assignments, and password fields still fail closed and are redacted.
    secret_specs+=("short_database:$DATABASE_PASSWORD")
  fi
  local spec
  for spec in "${secret_specs[@]}"; do
    mode="${spec%%:*}"
    secret="${spec#*:}"
    [[ -n "$secret" ]] || continue
    while IFS= read -r -d '' file; do
      scanner_status=0
      SECRET_VALUE="$secret" SECRET_MODE="$mode" python3 - "$file" <<'PY' || scanner_status=$?
import os, pathlib, re, sys
from urllib.parse import quote_from_bytes

p = pathlib.Path(sys.argv[1])
value = os.environ["SECRET_VALUE"].encode()
mode = os.environ["SECRET_MODE"]
data = p.read_bytes()
redacted = data
if value and mode == "literal":
    redacted = data.replace(value, b"[REDACTED]")
elif value and mode == "short_database":
    escaped = re.escape(value)
    encoded = quote_from_bytes(value, safe="").encode()
    patterns = [
        re.compile(rb"(:)" + escaped + rb"(?=@)"),
        re.compile(rb"(:)" + re.escape(encoded) + rb"(?=@)", re.IGNORECASE),
        re.compile(
            rb"(\b(?:DATABASE_PASSWORD|PGPASSWORD)\s*=\s*[\"']?)" + escaped
            + rb"(?=[\"']?(?:\r?\n|$))"
        ),
        re.compile(
            rb"(\bpassword\s*=\s*[\"']?)" + escaped
            + rb"(?=[\"']?(?:[&;\s]|$))",
            re.IGNORECASE,
        ),
        re.compile(
            rb"([\"']password[\"']\s*:\s*[\"'])" + escaped
            + rb"(?=[\"'])",
            re.IGNORECASE,
        ),
        re.compile(rb"(?m)^([ \t]*)" + escaped + rb"(?=[ \t]*\r?$)"),
    ]
    for pattern in patterns:
        redacted = pattern.sub(lambda match: match.group(1) + b"[REDACTED]", redacted)
else:
    raise RuntimeError(f"unsupported secret scanner mode: {mode}")
if redacted != data:
    p.write_bytes(redacted)
    raise SystemExit(42)
PY
      if (( scanner_status == 42 )); then
        found=1
      elif (( scanner_status != 0 )); then
        SECRET_REDACTION_REQUIRED=1
        fail "secret scanner could not inspect evidence file"
        return 1
      fi
    done < <(find "$RUN_DIR" -type f -print0)
  done
  if (( found == 1 )); then
    SECRET_REDACTION_REQUIRED=1
    return 1
  fi
  return 0
}

cleanup_resources() {
  local original_status="$1" cleanup_reason="" current_fp=""
  local qdisc_restored=0 removed=0 attempt
  local maintenance_candidate_count=0 maintenance_reports_valid=1
  local maintenance_service_stopped=0 maintenance_exact_only=1
  local maintenance_database_evidence_valid=0
  local maintenance_candidate="" maintenance_report=""
  local maintenance_candidates_file="" maintenance_command_ok=0
  local -a maintenance_candidates=()
  set +e

  terminate_group "$E2E_PID" e2e || { CLEANUP_FAILED=1; cleanup_reason+="e2e_stop;"; }
  E2E_PID=""
  terminate_group "$MONITOR_PID" readiness-monitor \
    || { CLEANUP_FAILED=1; cleanup_reason+="monitor_stop;"; }
  MONITOR_PID=""
  terminate_group "$SERVER_PID" standalone-server \
    || { CLEANUP_FAILED=1; cleanup_reason+="server_stop;"; }
  SERVER_PID=""
  terminate_group "$PROXY_PID" database-proxy \
    || { CLEANUP_FAILED=1; cleanup_reason+="proxy_stop;"; }
  PROXY_PID=""

  if [[ "$(unit_property ActiveState 2>/dev/null)" == inactive ]] \
      && assert_no_game_server_processes \
      && wait_for_port_state "$TEST_SERVER_PORT" free \
      && wait_for_port_state "$PROXY_PORT" free; then
    maintenance_service_stopped=1
  else
    CLEANUP_FAILED=1
    maintenance_reports_valid=0
    cleanup_reason+="maintenance_service_not_stopped;"
  fi

  if [[ -n "$RUN_DIR" && ! -f "$RUN_DIR/database-terminal.json" \
      && -n "$HOST_PLAYER" && -n "$GUEST_PLAYER" ]]; then
    capture_database_terminal "$RUN_DIR/database-terminal.json" \
      || { CLEANUP_FAILED=1; cleanup_reason+="database_capture;"; }
  fi
  if [[ -n "$RUN_DIR" && ! -f "$RUN_DIR/journal-after.json" ]]; then
    capture_journal "$RUN_DIR/journal-after.json" \
      || { CLEANUP_FAILED=1; cleanup_reason+="journal_capture;"; }
  fi
  if [[ -n "$RUN_DIR" ]]; then
    capture_journal "$RUN_DIR/journal-before-maintenance.json" "$MATCH_ID" \
      || { CLEANUP_FAILED=1; cleanup_reason+="journal_before_maintenance_capture;"; }
  fi

  if (( QDISC_OWNED == 1 )); then
    if (( QDISC_CONFIGURED == 1 )); then
      current_fp="$(qdisc_fingerprint 2>/dev/null)"
      if [[ "$current_fp" != "$QDISC_FINGERPRINT" ]]; then
        CLEANUP_FAILED=1
        cleanup_reason+="qdisc_fingerprint_changed;"
      else
        for attempt in 1 2; do
          if sudo -n tc qdisc del dev lo root >/dev/null 2>&1; then
            removed=1
            break
          fi
        done
        (( removed == 1 )) \
          || { CLEANUP_FAILED=1; cleanup_reason+="qdisc_remove;"; }
      fi
    elif qdisc_show 2>/dev/null | grep -q '^qdisc prio 1: root'; then
      sudo -n tc qdisc del dev lo root >/dev/null 2>&1 \
        || { CLEANUP_FAILED=1; cleanup_reason+="partial_qdisc_remove;"; }
    else
      CLEANUP_FAILED=1
      cleanup_reason+="partial_qdisc_owner_lost;"
    fi
  fi
  if [[ -n "$RUN_DIR" ]]; then
    qdisc_show >"$RUN_DIR/qdisc-after.txt" 2>&1 || true
  fi
  if qdisc_is_default_noqueue; then
    qdisc_restored=1
  else
    CLEANUP_FAILED=1
    cleanup_reason+="qdisc_not_restored;"
  fi

  if [[ -n "$HOST_PLAYER" && -n "$GUEST_PLAYER" ]]; then
    maintenance_candidates_file="$RUN_DIR/maintenance-candidates.txt"
    if (( maintenance_service_stopped == 1 )) \
        && maintenance_candidate_match_ids >"$maintenance_candidates_file" 2>"$RUN_DIR/maintenance-candidates.log"; then
      mapfile -t maintenance_candidates <"$maintenance_candidates_file"
      for maintenance_candidate in "${maintenance_candidates[@]}"; do
        if [[ ! "$maintenance_candidate" =~ ^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$ ]]; then
          maintenance_reports_valid=0
          maintenance_exact_only=0
          cleanup_reason+="maintenance_candidate_invalid;"
        fi
      done
      maintenance_candidate_count="${#maintenance_candidates[@]}"
      if (( maintenance_candidate_count > 1 )); then
        maintenance_reports_valid=0
        maintenance_exact_only=0
        cleanup_reason+="maintenance_candidate_ambiguous;"
      fi
    else
      maintenance_reports_valid=0
      cleanup_reason+="maintenance_candidate_discovery;"
    fi

    if (( maintenance_reports_valid == 1 && maintenance_candidate_count == 1 )); then
      maintenance_candidate="${maintenance_candidates[0]}"
      if [[ -n "$MATCH_ID" && "$maintenance_candidate" != "$MATCH_ID" ]]; then
        maintenance_reports_valid=0
        maintenance_exact_only=0
        cleanup_reason+="maintenance_candidate_report_mismatch;"
      fi
    fi

    if (( maintenance_reports_valid == 1 && maintenance_candidate_count == 1 )); then
      maintenance_report="$RUN_DIR/maintenance-fail-close-$maintenance_candidate.json"
      if [[ "$(sha256sum "$GAME_SERVER_BIN" 2>/dev/null | awk '{print $1}')" != "$GAME_SERVER_SHA" ]]; then
        maintenance_reports_valid=0
        cleanup_reason+="maintenance_release_binary_changed;"
      elif (
        close_inherited_mutation_locks
        export DATABASE_URL="$DIRECT_DATABASE_URL"
        export TRNM_PUBLISHED_TICK_JOURNAL_DIR="$CANONICAL_JOURNAL"
        export TRNM_FLEET_INSTANCE_ID="$TEST_INSTANCE_ID"
        export TRNM_FLEET_PHYSICAL_HOST_ID="$PHYSICAL_HOST_ID"
        export TRNM_MAINTENANCE_FAILURE_REASON="$MAINTENANCE_FAILURE_REASON"
        exec timeout --foreground --signal=TERM --kill-after=5s 120s \
          "$GAME_SERVER_BIN" --maintenance-fail-close "$maintenance_candidate"
      ) 2>"$RUN_DIR/maintenance-fail-close-$maintenance_candidate.log" \
          | atomic_write "$maintenance_report"; then
        maintenance_command_ok=1
      else
        maintenance_reports_valid=0
        cleanup_reason+="maintenance_command_failed;"
      fi
      if (( maintenance_command_ok == 1 )) \
          && ! maintenance_report_is_exact_and_atomic \
            "$maintenance_report" "$maintenance_candidate"; then
        maintenance_reports_valid=0
        cleanup_reason+="maintenance_report_invalid;"
      fi
    fi

    capture_journal "$RUN_DIR/journal-after-maintenance.json" \
      "${maintenance_candidate:-$MATCH_ID}" \
      || { maintenance_reports_valid=0; cleanup_reason+="maintenance_journal_capture;"; }
    if (( maintenance_reports_valid == 1 && maintenance_candidate_count == 1 )) \
        && ! maintenance_post_journal_is_exact \
          "$maintenance_report" "$RUN_DIR/journal-after-maintenance.json" \
          "$maintenance_candidate"; then
      maintenance_reports_valid=0
      cleanup_reason+="maintenance_journal_postcondition;"
    fi
    if (( maintenance_candidate_count == 1 )); then
      if capture_database_after_maintenance \
          "$RUN_DIR/database-after-maintenance.json" "$maintenance_candidate"; then
        if (( maintenance_reports_valid == 1 )) \
            && maintenance_post_database_is_exact \
              "$maintenance_report" "$RUN_DIR/database-after-maintenance.json" \
              "$RUN_DIR/journal-after-maintenance.json" "$maintenance_candidate"; then
          maintenance_database_evidence_valid=1
        else
          maintenance_reports_valid=0
          cleanup_reason+="maintenance_database_postcondition;"
        fi
      else
        maintenance_reports_valid=0
        cleanup_reason+="maintenance_database_capture;"
      fi
    elif (( maintenance_candidate_count == 0 && maintenance_reports_valid == 1 )); then
      jq -n '{
        contract_version:"trnm_online_maintenance_database_evidence_v1",
        status:"not_applicable",match_count:0
      }' | atomic_write "$RUN_DIR/database-after-maintenance.json" \
        && maintenance_database_evidence_valid=1 \
        || { maintenance_reports_valid=0; cleanup_reason+="maintenance_database_empty_evidence;"; }
    fi
    if [[ ! -f "$RUN_DIR/database-after-maintenance.json" ]]; then
      jq -n '{
        contract_version:"trnm_online_maintenance_database_evidence_v1",
        status:"unavailable",match_count:0
      }' | atomic_write "$RUN_DIR/database-after-maintenance.json" \
        || cleanup_reason+="maintenance_database_fallback_evidence;"
    fi
    if (( maintenance_candidate_count == 1 && maintenance_reports_valid == 1 )); then
      jq -s '{contract_version:"trnm_online_maintenance_fail_close_collection_v1",
        selector:"exact_match_id",report_count:length,reports:.}' "$maintenance_report" \
        | atomic_write "$RUN_DIR/maintenance-fail-close.json" \
        || { maintenance_reports_valid=0; cleanup_reason+="maintenance_collection;"; }
    else
      jq -n '{contract_version:"trnm_online_maintenance_fail_close_collection_v1",
        selector:"exact_match_id",report_count:0,reports:[]}' \
        | atomic_write "$RUN_DIR/maintenance-fail-close.json" \
        || { maintenance_reports_valid=0; cleanup_reason+="maintenance_collection;"; }
    fi
    (( maintenance_reports_valid == 1 )) \
      || { CLEANUP_FAILED=1; cleanup_reason+="maintenance_fail_closed;"; }

    cex_psql_stdin -v ON_ERROR_STOP=1 -c "
      update trnm_online_fleet_instances
      set status='offline', active_matches=0, lease_expires_at=now(), heartbeat_at=now()
      where instance_id='$TEST_INSTANCE_ID';" >/dev/null 2>&1 \
      || { CLEANUP_FAILED=1; cleanup_reason+="fleet_offline;"; }
    if [[ -n "$RUN_DIR" ]]; then
      capture_fleet_state "$RUN_DIR/fleet-after-cleanup.json" \
        || { CLEANUP_FAILED=1; cleanup_reason+="fleet_capture;"; }
      if ! jq -e '.status=="offline" and .active_matches==0 and .open_run_match_count==0' \
          "$RUN_DIR/fleet-after-cleanup.json" >/dev/null 2>&1; then
        CLEANUP_FAILED=1
        cleanup_reason+="fleet_or_match_orphan;"
      fi
    fi
  fi

  assert_no_game_server_processes \
    || { CLEANUP_FAILED=1; cleanup_reason+="maintenance_process_orphan;"; }

  wait_for_port_state "$TEST_SERVER_PORT" free \
    || { CLEANUP_FAILED=1; cleanup_reason+="test_port_busy;"; }
  wait_for_port_state "$PROXY_PORT" free \
    || { CLEANUP_FAILED=1; cleanup_reason+="proxy_port_busy;"; }

  if (( SERVICE_STATE_CAPTURED == 1 && qdisc_restored == 1 )); then
    if [[ "$ORIGINAL_ACTIVE_STATE" == active ]]; then
      systemctl --user start "$SERVICE" >/dev/null 2>&1 \
        || { CLEANUP_FAILED=1; cleanup_reason+="service_start;"; }
      wait_for_unit_state active \
        || { CLEANUP_FAILED=1; cleanup_reason+="service_state;"; }
      wait_for_production_readiness \
        || { CLEANUP_FAILED=1; cleanup_reason+="service_readiness;"; }
      assert_loopback_listener 7005 \
        || { CLEANUP_FAILED=1; cleanup_reason+="service_bind;"; }
      verify_active_service_binary \
        || { CLEANUP_FAILED=1; cleanup_reason+="service_binary;"; }
      if [[ "$CONTRACT_MODE" == 0 ]]; then
        fault_effective_unit_matches_source "$SERVICE" \
          && fault_active_cgroup_matches_source "$SERVICE" \
          || { CLEANUP_FAILED=1; cleanup_reason+="service_effective_cgroup;"; }
      fi
    else
      systemctl --user stop "$SERVICE" >/dev/null 2>&1 || true
      wait_for_unit_state inactive \
        || { CLEANUP_FAILED=1; cleanup_reason+="service_inactive_restore;"; }
    fi
  elif (( SERVICE_STATE_CAPTURED == 1 )) && [[ "$ORIGINAL_ACTIVE_STATE" == active ]]; then
    CLEANUP_FAILED=1
    cleanup_reason+="service_restore_suppressed_until_qdisc_is_safe;"
  fi

  if (( SERVICE_STATE_CAPTURED == 1 )) && [[ -n "$ORIGINAL_RELEASE_DIR" ]]; then
    local restored_release=""
    restored_release="$(realpath -e -- "$RELEASE_ROOT/current" 2>/dev/null)"
    [[ "$restored_release" == "$ORIGINAL_RELEASE_DIR" ]] \
      || { CLEANUP_FAILED=1; cleanup_reason+="release_selector_changed;"; }
    [[ "$(sha256sum "$restored_release/trnm-game-server" 2>/dev/null | awk '{print $1}')" \
        == "$ORIGINAL_RELEASE_SHA" ]] \
      || { CLEANUP_FAILED=1; cleanup_reason+="release_binary_changed;"; }
  fi

  CLEANUP_COMPLETE=1
  if [[ -n "$RUN_DIR" ]]; then
    jq -n \
      --arg contract_version trnm_online_authority_fault_cleanup_v1 \
      --argjson original_status "$original_status" \
      --arg original_active_state "$ORIGINAL_ACTIVE_STATE" \
      --arg restored_active_state "$(unit_property ActiveState 2>/dev/null)" \
      --arg reason "$cleanup_reason" \
      --argjson cleanup_failed "$CLEANUP_FAILED" \
      --argjson qdisc_default "$(qdisc_is_default_noqueue && echo true || echo false)" \
      --argjson test_port_free "$(port_is_free "$TEST_SERVER_PORT" && echo true || echo false)" \
      --argjson proxy_port_free "$(port_is_free "$PROXY_PORT" && echo true || echo false)" \
      --argjson maintenance_candidate_count "$maintenance_candidate_count" \
      --argjson maintenance_reports_valid "$maintenance_reports_valid" \
      --argjson maintenance_service_stopped "$maintenance_service_stopped" \
      --argjson maintenance_exact_only "$maintenance_exact_only" \
      --argjson maintenance_database_evidence_valid "$maintenance_database_evidence_valid" \
      '{contract_version:$contract_version,original_workload_status:$original_status,
        original_active_state:$original_active_state,restored_active_state:$restored_active_state,
        cleanup_failed:$cleanup_failed,reason:$reason,qdisc_default_noqueue:$qdisc_default,
        test_port_free:$test_port_free,proxy_port_free:$proxy_port_free,
        maintenance_candidate_count:$maintenance_candidate_count,
        maintenance_reports_valid:($maintenance_reports_valid==1),
        maintenance_service_stopped:($maintenance_service_stopped==1),
        maintenance_exact_only:($maintenance_exact_only==1),
        maintenance_database_evidence_valid:($maintenance_database_evidence_valid==1),
        sigkill_or_power_loss_cleanup_guaranteed:false}' \
      | atomic_write "$RUN_DIR/cleanup.json"
  fi
  (( CLEANUP_FAILED == 0 ))
}

build_artifact_manifest() {
  local destination="$RUN_DIR/artifact-manifest.json" temporary="$RUN_DIR/artifact-manifest.json.tmp.$$"
  RUN_ARTIFACT_ROOT="$RUN_DIR" python3 - <<'PY' >"$temporary"
import hashlib, json, os, pathlib
root = pathlib.Path(os.environ["RUN_ARTIFACT_ROOT"])
artifacts = []
for path in sorted(root.rglob("*")):
    if not path.is_file() or path.name == "decision.json" or ".tmp." in path.name:
        continue
    data = path.read_bytes()
    artifacts.append({"path": str(path.relative_to(root)), "bytes": len(data),
                      "sha256": hashlib.sha256(data).hexdigest()})
print(json.dumps({"contract_version":"trnm_online_authority_fault_artifact_manifest_v1",
                  "decision_excluded_to_avoid_digest_cycle":True,
                  "artifacts":artifacts}, separators=(",", ":")))
PY
  chmod 0600 "$temporary"
  mv -f -- "$temporary" "$destination"
}

write_decision() {
  local exit_status="$1" workload_pass=false formal_pass=false contract_pass=false
  local e2e="$RUN_DIR/e2e-report.json" database="$RUN_DIR/database-terminal.json"
  local journal="$RUN_DIR/journal-after.json" cleanup="$RUN_DIR/cleanup.json"
  local maintenance="$RUN_DIR/maintenance-fail-close.json"
  local maintenance_database="$RUN_DIR/database-after-maintenance.json"
  local readiness_gate=false packet_gate=false effect_gate=false ack_gate=false drift_gate=false
  local database_gate=false journal_gate=false cleanup_gate=false e2e_gate=false
  local monitor_gate=false process_gate=false dependency_gate=false inputs_gate=false

  [[ -f "$e2e" ]] && e2e_gate="$(jq -e '.status=="passed"' "$e2e" >/dev/null && echo true || echo false)"
  [[ -f "$RUN_DIR/readiness-samples.jsonl" ]] && readiness_gate="$(jq -s -e '
    length >= 2 and all(.[];
      .http_ok == true and (.body|type) == "object" and .body.status == "ok"
      and .body.authority_clock_operational == true
      and .body.match_actor_clocks_operational == true
      and (.body.authority_clock_drift_ticks|type) == "number"
      and (.body.max_actor_clock_abs_drift_ticks|type) == "number"
      and (.body.authority_clock_drift_ticks | fabs) < 2
      and (.body.max_actor_clock_abs_drift_ticks | fabs) < 2
      and .body.latest_cold_witness_sentinel_query_healthy == true
      and .body.latest_cold_witness_sentinel_healthy == true
      and .body.cold_witness_database_summary_query_healthy == true
      and .body.local_tombstone_counts_exact == true
      and .body.local_tombstone_seal_operational == true
      and .body.operational_readiness.local_cold_witness_seal == true
      and .body.published_tick_terminal_orphan_recovery_operational == true)
    ' "$RUN_DIR/readiness-samples.jsonl" >/dev/null && echo true || echo false)"
  if (( MONITOR_SURVIVED_WORKLOAD == 1 )) \
      && [[ -f "$RUN_DIR/readiness-monitor.started" \
        && ! -e "$RUN_DIR/readiness-monitor.failed" ]]; then
    monitor_gate=true
  fi
  (( QDISC_PACKETS_AFTER > QDISC_PACKETS_BEFORE )) && packet_gate=true
  [[ -f "$e2e" ]] && effect_gate="$(jq -e '
    .websocket_authoritative_effect_samples_ms as $v
    | ($v|type) == "array" and ($v|length) >= 20
    and all($v[]; type == "number" and . >= 0)
    and (.websocket_authoritative_effect_p95_ms|type) == "number"
    and .websocket_authoritative_effect_p95_ms >= 0
    and (($v|sort) as $sorted
      | $sorted[(((($sorted|length) * 95 + 99) / 100 | floor) - 1)] <= 300)
    and .websocket_authoritative_effect_p95_ms <= 300' \
    "$e2e" >/dev/null && echo true || echo false)"
  [[ -f "$e2e" ]] && ack_gate="$(jq -e '
    .command_ack_ms as $raw
    | ($raw|type) == "array" and ($raw|length) >= 1
    and all($raw[]; type == "number" and . >= 0)
    and (($raw|sort) as $v
      | $v[((($v|length) * 99 + 99) / 100 | floor) - 1] <= 750)' \
    "$e2e" >/dev/null && echo true || echo false)"
  [[ -f "$RUN_DIR/readiness-samples.jsonl" ]] && drift_gate="$(jq -s -e '
    length >= 2 and all(.[];
      .http_ok == true and (.body|type) == "object"
      and (.body.max_actor_clock_cumulative_abs_drift_ticks|type) == "number"
      and .body.max_actor_clock_cumulative_abs_drift_ticks >= 0
      and .body.max_actor_clock_cumulative_abs_drift_ticks < 2)' \
    "$RUN_DIR/readiness-samples.jsonl" >/dev/null && echo true || echo false)"
  [[ -f "$database" ]] && database_gate="$(jq -e --arg instance "$TEST_INSTANCE_ID" '
    .contract_version == "trnm_authority_terminal_database_evidence_v2"
    and .match_count == 1 and .phase == "complete" and .settlement_state == "settled"
    and .checkpoint_sequence == .next_sequence
    and .command_sequences_contiguous == true
    and .player_input_sequences_unique == true
    and .missing_post_simulation == 0
    and .member_cursors_exact == true
    and .terminal_marker_count == 1 and .terminal_marker_exact == true
    and (.acknowledged_at_unix_ms | type) == "number" and .acknowledged_at_unix_ms > 0
    and (.database_system_identifier | type) == "string"
    and (.database_system_identifier | test("^[1-9][0-9]{0,19}$"))
    and (.database_timeline_id | type) == "number" and .database_timeline_id > 0
    and (.database_current_wal_lsn | type) == "string"
    and (.database_current_wal_lsn
      | test("^(0|[1-9A-F][0-9A-F]{0,7})/(0|[1-9A-F][0-9A-F]{0,7})$"))
    and .database_current_wal_lsn != "0/0"
    and .assigned_instance_id == $instance' "$database" >/dev/null && echo true || echo false)"
  [[ -f "$journal" && -f "$RUN_DIR/journal-before.json" && -f "$database" \
      && -n "$MATCH_ID" ]] && journal_gate="$(jq -e \
    --arg match_id "$MATCH_ID" --arg instance "$TEST_INSTANCE_ID" \
    --arg host "$PHYSICAL_HOST_ID" --arg owner_sha "$JOURNAL_OWNER_SHA" \
    --arg hot_file "$(journal_hot_relative_path "$MATCH_ID")" \
    --arg cold_file "$(journal_ack_relative_path "$MATCH_ID")" \
    --arg ack_root acknowledged \
    --arg ack_first "$(journal_ack_relative_path "$MATCH_ID" | cut -d/ -f1-2)" \
    --arg ack_second "$(journal_ack_relative_path "$MATCH_ID" | cut -d/ -f1-3)" \
    --argjson current_uid "$(id -u)" \
    --slurpfile before "$RUN_DIR/journal-before.json" --slurpfile database "$database" '
    def stable_item: {name,mode,nlink,uid,gid,regular,symlink,sha256,bytes};
    def manifest_terminal_count:
      if .contract_version == "trnm_published_tick_ack_manifest_v1" then .tombstone_count
      elif .contract_version == "trnm_published_tick_cold_witness_manifest_v2"
        then .terminal_tombstone_count else null end;
    def manifest_abandonment_count:
      if .contract_version == "trnm_published_tick_ack_manifest_v1" then 0
      elif .contract_version == "trnm_published_tick_cold_witness_manifest_v2"
        then .abandonment_tombstone_count else null end;
    .contract_version == "trnm_published_tick_journal_inventory_v2"
    and $before[0].contract_version == "trnm_published_tick_journal_inventory_v2"
    and .root_exists == true and .root_mode == "0700" and .root_uid == $current_uid
    and .decode_error_count == 0 and $before[0].decode_error_count == 0
    and .owner_manifest_sha256 == $owner_sha
    and .owner_manifest_sha256 == $before[0].owner_manifest_sha256
    and (.owner_manifest | type) == "object"
    and .owner_manifest.contract_version == "trnm_published_tick_journal_owner_v1"
    and (.ack_manifest | type) == "object"
    and ($before[0].ack_manifest | type) == "object"
    and .ack_manifest.contract_version == "trnm_published_tick_cold_witness_manifest_v2"
    and ($before[0].ack_manifest.contract_version == "trnm_published_tick_ack_manifest_v1"
      or $before[0].ack_manifest.contract_version
        == "trnm_published_tick_cold_witness_manifest_v2")
    and .ack_manifest.journal_owner_id == .owner_manifest.journal_owner_id
    and .ack_manifest.physical_host_id == .owner_manifest.physical_host_id
    and $before[0].ack_manifest.journal_owner_id == .owner_manifest.journal_owner_id
    and $before[0].ack_manifest.physical_host_id == .owner_manifest.physical_host_id
    and .ack_manifest.terminal_tombstone_count
      == (($before[0].ack_manifest | manifest_terminal_count) + 1)
    and .ack_manifest.abandonment_tombstone_count
      == ($before[0].ack_manifest | manifest_abandonment_count)
    and .ack_manifest.committed_seal_sequence
      == ($before[0].ack_manifest.committed_seal_sequence + 1)
    and (.ack_manifest.terminal_tombstone_count
      + .ack_manifest.abandonment_tombstone_count)
      == .ack_manifest.committed_seal_sequence
    and .ack_manifest.terminal_tombstone_count == .ack_tombstone_count
    and .ack_manifest.abandonment_tombstone_count == .abandonment_tombstone_count
    and .cold_witness_count == (.ack_tombstone_count + .abandonment_tombstone_count)
    and .cold_witness_count == .ack_manifest.committed_seal_sequence
    and ($before[0].ack_tombstone_count
      == ($before[0].ack_manifest | manifest_terminal_count))
    and ($before[0].abandonment_tombstone_count
      == ($before[0].ack_manifest | manifest_abandonment_count))
    and $before[0].cold_witness_count
      == $before[0].ack_manifest.committed_seal_sequence
    and .ack_manifest.database_system_identifier == $database[0].database_system_identifier
    and .ack_manifest.database_timeline_id == $database[0].database_timeline_id
    and (.ack_manifest_sha256 | type) == "string"
    and (.ack_manifest_sha256 | test("^[0-9a-f]{64}$"))
    and .run_match_hot_record_present == false
    and .run_match_ack_tombstone_present == true
    and .run_match_abandonment_tombstone_present == false
    and .run_match_cold_witness_present == true
    and ([.records[] | select(.match_id == $match_id)] | length) == 0
    and ([.ack_tombstones[] | select(.high_water.match_id == $match_id)] | length) == 1
    and ([.items[] | select(.name == $hot_file)] | length) == 0
    and ([.items[] | select(.name == $cold_file)] | length) == 1
    and ([.items[] | select(.name == $cold_file)][0]
      | .mode == "0600" and .nlink == 1 and .uid == $current_uid
      and .regular == true and .symlink == false
      and (.sha256 | type) == "string" and (.sha256 | test("^[0-9a-f]{64}$"))
      and (.bytes | type) == "number" and .bytes > 0)
    and ([.directories[] | select(.name == $ack_root or .name == $ack_first
      or .name == $ack_second)] | length) == 3
    and all(.directories[] | select(.name == $ack_root or .name == $ack_first
      or .name == $ack_second);
      .mode == "0700" and .uid == $current_uid and .symlink == false)
    and ([.ack_tombstones[] | select(.high_water.match_id == $match_id)][0] as $tombstone
      | $tombstone.high_water as $record
      | $tombstone.contract_version == "trnm_published_tick_ack_tombstone_v2"
      and $tombstone.journal_seal_sequence == .ack_manifest.committed_seal_sequence
      and $record.contract_version == "trnm_published_tick_high_water_v2"
      and $record.journal_owner_id == .owner_manifest.journal_owner_id
      and $record.instance_id == $instance
      and $record.instance_id == $database[0].assigned_instance_id
      and $record.instance_id == $database[0].ack_instance_id
      and $record.physical_host_id == $host
      and $record.physical_host_id == .owner_manifest.physical_host_id
      and $record.physical_host_id == $database[0].assigned_physical_host_id
      and $record.physical_host_id == $database[0].ack_physical_host_id
      and $record.actor_generation == $database[0].terminal_publication_actor_generation
      and $record.actor_generation == $database[0].ack_actor_generation
      and $record.phase == "complete" and $record.receipts_replayable == true
      and $record.actor_epoch == $database[0].assigned_instance_epoch
      and $record.actor_epoch == $database[0].ack_actor_epoch
      and $record.tick == $database[0].authoritative_tick
      and $record.tick == $database[0].ack_authoritative_tick
      and $record.next_sequence == $database[0].next_sequence
      and $record.next_sequence == $database[0].ack_next_sequence
      and $record.match_revision == $database[0].match_revision
      and $record.match_revision == $database[0].ack_match_revision
      and $record.next_input_sequences == $database[0].member_cursors
      and $record.next_input_sequences == $database[0].ack_next_input_sequences
      and $record.snapshot_hash == $database[0].snapshot_hash
      and $record.snapshot_hash == $database[0].ack_snapshot_hash
      and $tombstone.result_hash == $database[0].result_hash
      and $tombstone.result_hash == $database[0].ack_result_hash
      and ($tombstone.settlement_state == $database[0].ack_settlement_state
        or ($tombstone.settlement_state == "pending"
          and $database[0].ack_settlement_state == "settled"))
      and ($tombstone.settlement_state == $database[0].settlement_state
        or ($tombstone.settlement_state == "pending"
          and $database[0].settlement_state == "settled"))
      and $tombstone.acknowledged_at_unix_ms == $database[0].acknowledged_at_unix_ms
      and $tombstone.database_system_identifier == $database[0].database_system_identifier
      and $tombstone.database_timeline_id == $database[0].database_timeline_id
      and ($tombstone.database_wal_lsn | type) == "string"
      and ($tombstone.database_wal_lsn
        | test("^(0|[1-9A-F][0-9A-F]{0,7})/(0|[1-9A-F][0-9A-F]{0,7})$"))
      and $tombstone.database_wal_lsn != "0/0"
      and (.ack_manifest.latest_witness | keys) == ["terminal_ack"]
      and .ack_manifest.latest_witness.terminal_ack
        == ($tombstone | del(.name, .sha256))
      and .ack_manifest.latest_witness_sha256 == $tombstone.sha256)
    and ([$before[0].items[] | select(.name == $hot_file or .name == $cold_file)]
      | length) == 0
    and (([$before[0].items[] | select(.name != ".published-tick.lock"
          and .name != ".published-tick-ack-manifest.json") | stable_item]
        | sort_by(.name))
      == ([.items[] | select(.name != ".published-tick.lock"
          and .name != ".published-tick-ack-manifest.json" and .name != $cold_file)
          | stable_item] | sort_by(.name)))
    and (([$before[0].directories[] | select(.name != $ack_root
          and .name != $ack_first and .name != $ack_second)] | sort_by(.name))
      == ([.directories[] | select(.name != $ack_root
          and .name != $ack_first and .name != $ack_second)] | sort_by(.name)))' \
    "$journal" >/dev/null && echo true || echo false)"
  if [[ -f "$RUN_DIR/server-identity-start.json" && -f "$RUN_DIR/server-identity-end.json" \
      && -f "$RUN_DIR/proxy-identity-start.json" && -f "$RUN_DIR/proxy-identity-end.json" ]]; then
    process_gate="$(jq -s -e --arg server_sha "$GAME_SERVER_SHA" \
      --argjson contract_mode "$CONTRACT_MODE" '
      length == 4 and all(.[]; .listener_owned_by_pid == true and .command_bound == true)
      and .[0].label == "server" and .[1].label == "server"
      and .[2].label == "proxy" and .[3].label == "proxy"
      and .[0].pid == .[1].pid and .[0].executable == .[1].executable
      and .[0].expected_artifact_sha256 == $server_sha
      and .[0].expected_artifact_sha256 == .[1].expected_artifact_sha256
      and .[0].executable_sha256 == .[1].executable_sha256
      and .[0].process_start_ticks == .[1].process_start_ticks
      and .[0].cgroup == .[1].cgroup
      and .[2].pid == .[3].pid and .[2].executable == .[3].executable
      and .[2].expected_artifact_sha256 == .[3].expected_artifact_sha256
      and .[2].executable_sha256 == .[3].executable_sha256
      and .[2].process_start_ticks == .[3].process_start_ticks
      and .[2].cgroup == .[3].cgroup
      and ($contract_mode == 1 or (
        all(.[]; .native_executable_match == true)
        and .[0].executable_sha256 == $server_sha
        and .[2].executable_sha256 == .[2].expected_artifact_sha256))' \
      "$RUN_DIR/server-identity-start.json" "$RUN_DIR/server-identity-end.json" \
      "$RUN_DIR/proxy-identity-start.json" "$RUN_DIR/proxy-identity-end.json" \
      >/dev/null && echo true || echo false)"
  fi
  if [[ "$CONTRACT_MODE" == 1 ]]; then
    dependency_gate=true
  elif [[ -f "$RUN_DIR/ledger-identity-start.json" \
      && -f "$RUN_DIR/ledger-identity-end.json" \
      && -f "$RUN_DIR/signer-identity-start.json" \
      && -f "$RUN_DIR/signer-identity-end.json" ]]; then
    dependency_gate="$(jq -s -e '
      length == 4 and all(.[]; .listener_owned_by_main_pid == true)
      and .[0] == .[1] and .[2] == .[3]
      and .[0].unit == "cex-trnm-ledger.service"
      and .[2].unit == "trnm-entitlement-signer.service"' \
      "$RUN_DIR/ledger-identity-start.json" "$RUN_DIR/ledger-identity-end.json" \
      "$RUN_DIR/signer-identity-start.json" "$RUN_DIR/signer-identity-end.json" \
      >/dev/null && echo true || echo false)"
  fi
  [[ -f "$RUN_DIR/bound-inputs-after.json" ]] && inputs_gate="$(jq -e \
    '.unchanged == true and .root.clean == true and .cex.clean == true' \
    "$RUN_DIR/bound-inputs-after.json" >/dev/null && echo true || echo false)"
  [[ -f "$cleanup" && -f "$maintenance" && -f "$maintenance_database" ]] \
    && cleanup_gate="$(jq -e \
    --slurpfile maintenance "$maintenance" \
    --slurpfile maintenance_database "$maintenance_database" '
    .cleanup_failed == 0 and .qdisc_default_noqueue == true
    and .test_port_free == true and .proxy_port_free == true
    and .maintenance_reports_valid == true
    and .maintenance_service_stopped == true
    and .maintenance_exact_only == true
    and .maintenance_database_evidence_valid == true
    and $maintenance[0].contract_version
      == "trnm_online_maintenance_fail_close_collection_v1"
    and $maintenance[0].selector == "exact_match_id"
    and $maintenance[0].report_count == .maintenance_candidate_count
    and ($maintenance[0].reports | length) == .maintenance_candidate_count
    and all($maintenance[0].reports[];
      .contract_version == "trnm_online_maintenance_fail_close_v1"
      and .status == "completed"
      and .selector == "exact_match_id"
      and .transition_atomic == true
      and .legacy_adoption == false
      and .adoption_contract == null
      and .final_phase == "failed_closed")
    and $maintenance_database[0].contract_version
      == "trnm_online_maintenance_database_evidence_v1"
    and (if .maintenance_candidate_count == 0 then
      $maintenance_database[0].status == "not_applicable"
      and $maintenance_database[0].match_count == 0
    else
      $maintenance_database[0].status == "captured"
      and $maintenance_database[0].match_count == 1
    end)' \
    "$cleanup" >/dev/null && echo true || echo false)"

  if [[ "$e2e_gate" == true && "$readiness_gate" == true && "$packet_gate" == true \
      && "$effect_gate" == true && "$ack_gate" == true && "$drift_gate" == true \
      && "$database_gate" == true && "$journal_gate" == true && "$cleanup_gate" == true \
      && "$monitor_gate" == true && "$process_gate" == true \
      && "$dependency_gate" == true && "$inputs_gate" == true \
      && "$WORKLOAD_RC" -eq 0 && "$exit_status" -eq 0 && "$CLEANUP_FAILED" -eq 0 \
      && "$SECRET_REDACTION_REQUIRED" -eq 0 ]]; then
    workload_pass=true
  fi
  if [[ "$CONTRACT_MODE" == 0 && "$workload_pass" == true ]]; then
    formal_pass=true
  fi
  if [[ "$CONTRACT_MODE" == 1 && "$workload_pass" == true ]]; then
    contract_pass=true
  fi

  jq -n \
    --arg contract_version "$DECISION_CONTRACT" --arg profile "$PROFILE" \
    --arg run_id "$RUN_ID" --arg release_id "$RELEASE_ID" --arg cgroup "$RESOURCE_CGROUP" \
    --argjson passed "$formal_pass" --argjson contract_test_passed "$contract_pass" \
    --argjson local_only true --argjson public_launch_credit false \
    --argjson e2e "$e2e_gate" --argjson readiness "$readiness_gate" \
    --argjson packet "$packet_gate" --argjson effect "$effect_gate" \
    --argjson ack "$ack_gate" --argjson drift "$drift_gate" \
    --argjson database "$database_gate" --argjson journal "$journal_gate" \
    --argjson cleanup "$cleanup_gate" --argjson monitor "$monitor_gate" \
    --argjson process "$process_gate" --argjson dependencies "$dependency_gate" \
    --argjson inputs "$inputs_gate" \
    --argjson packet_before "$QDISC_PACKETS_BEFORE" \
    --argjson packet_after "$QDISC_PACKETS_AFTER" \
    --argjson contract_mode "$CONTRACT_MODE" \
    --argjson secret_free "$(( SECRET_REDACTION_REQUIRED == 0 ? 1 : 0 ))" \
    '{contract_version:$contract_version,
      status:(if $passed then "passed" elif $contract_test_passed then "contract_test_passed" else "failed" end),
      passed:$passed,contract_test_passed:$contract_test_passed,profile:$profile,run_id:$run_id,
      release_id:$release_id,local_only:$local_only,public_launch_credit:$public_launch_credit,
      contract_test_mode:($contract_mode==1),resource_cgroup:$cgroup,
      thresholds:{effect_sample_count_minimum:20,effect_p95_ms_maximum:300,
        command_ack_p99_ms_maximum:750,actor_absolute_drift_ticks_maximum_exclusive:2,
        netem_one_way_delay_ms:50,target_database_rtt_ms:100},
      checks:{e2e_passed:$e2e,readiness_and_actor_clocks_healthy:$readiness,
        netem_packet_delta_positive:$packet,effect_latency_within_budget:$effect,
        command_ack_p99_within_budget:$ack,actor_drift_within_budget:$drift,
        database_terminal_invariants:$database,terminal_journal_cold_ack_tombstone:$journal,
        readiness_monitor_survived_workload:$monitor,bound_process_identity:$process,
        dependency_process_identity:$dependencies,bound_inputs_unchanged:$inputs,
        cleanup_and_restore_complete:$cleanup,
        evidence_secret_free:($secret_free==1)},
      netem_packets:{before:$packet_before,after:$packet_after,delta:($packet_after-$packet_before)},
      limitations:{sigkill_and_power_loss_not_trappable:true,cross_host_rpo_zero_proven:false,
        public_edge_or_ddos_proven:false,profile_scope:"local PostgreSQL RTT injection only"}}' \
    | atomic_write "$RUN_DIR/decision.json"

  if [[ "$CONTRACT_MODE" == 1 ]]; then
    [[ "$contract_pass" == true ]]
  else
    [[ "$formal_pass" == true ]]
  fi
}

finalize_on_exit() {
  local status=$?
  trap - EXIT
  trap '' INT TERM HUP
  set +e
  cleanup_resources "$status"
  local cleanup_rc=$?
  (( cleanup_rc == 0 )) || status=1
  if [[ -n "$RUN_DIR" ]]; then
    scan_and_redact_secrets || status=1
    build_artifact_manifest || status=1
    write_decision "$status"
    local decision_rc=$?
    (( decision_rc == 0 )) || status=1
  fi
  exit "$status"
}

if [[ "$CONTRACT_MODE" == 0 && "$RESOURCE_SCOPE_ACTIVE" == 0 ]]; then
  command -v systemd-run >/dev/null 2>&1 \
    || fail "formal fault evidence requires systemd-run resource containment"
  scope_nonce="$(date -u +%Y%m%dT%H%M%S)-$$"
  exec systemd-run --user --scope --collect --quiet \
    --unit="trnm-authority-fault-$scope_nonce" \
    --property=MemoryMax=2G --property=MemoryHigh=1536M \
    --property=MemorySwapMax=512M --property=TasksMax=512 \
    --property=CPUQuota=150% --property=RuntimeMaxSec=2700 \
    --setenv=TRNM_FAULT_RESOURCE_SCOPE_ACTIVE=1 \
    "$(realpath -e -- "${BASH_SOURCE[0]}")" "$@"
fi

for command_name in awk bash cmp curl cut date find flock git grep hostname install jq \
  pgrep python3 readlink realpath setsid sha256sum ss stat sudo systemctl \
  timeout tr uname; do
  require_command "$command_name"
done
if [[ "$CONTRACT_MODE" == 0 ]]; then
  (( RESOURCE_SCOPE_ACTIVE == 1 )) \
    || fail "formal fault evidence escaped its bounded resource scope"
  validate_bounded_resource_cgroup
fi
CEX_ROOT="$(realpath -e -- "$CEX_ROOT")"
if [[ "$CONTRACT_MODE" == 0 ]]; then
  [[ "$CEX_ROOT" == "$(realpath -e -- "$ROOT_DIR/../CEX")" ]] \
    || fail "formal CEX root is not canonical"
fi
INSTALLED_UNIT="$(unit_property FragmentPath)"
INSTALLED_UNIT="$(realpath -e -- "$INSTALLED_UNIT")"
[[ -f "$CEX_ROOT/scripts/_dev-helpers.sh" ]] || fail "CEX development helpers are unavailable"
[[ -f "$SOURCE_UNIT" && -f "$INSTALLED_UNIT" ]] || fail "game-server systemd unit is missing"
cmp -s "$SOURCE_UNIT" "$INSTALLED_UNIT" || fail "installed game-server unit differs from source"
if [[ "$CONTRACT_MODE" == 0 ]]; then
  for effective_unit in trnm-game-server.service trnm-entitlement-signer.service \
    cex-trnm-ledger.service; do
    fault_effective_unit_matches_source "$effective_unit" \
      || fail "$effective_unit effective systemd contract differs from source"
  done
fi
SOURCE_UNIT_SHA="$(sha256sum "$SOURCE_UNIT" | awk '{print $1}')"
INSTALLED_UNIT_SHA="$(sha256sum "$INSTALLED_UNIT" | awk '{print $1}')"
CEX_HELPER_SHA="$(sha256sum "$CEX_ROOT/scripts/_dev-helpers.sh" | awk '{print $1}')"

ensure_private_directory "$LOCK_ROOT"
ensure_private_directory "$EVIDENCE_ROOT"
if [[ -e "$LOCK_ROOT/trnm-authority-fault-harness.lock" \
    || -L "$LOCK_ROOT/trnm-authority-fault-harness.lock" ]]; then
  [[ -f "$LOCK_ROOT/trnm-authority-fault-harness.lock" \
      && ! -L "$LOCK_ROOT/trnm-authority-fault-harness.lock" \
      && "$(stat -c '%a' "$LOCK_ROOT/trnm-authority-fault-harness.lock")" == 600 \
      && "$(stat -c '%u:%g:%h' "$LOCK_ROOT/trnm-authority-fault-harness.lock")" \
        == "$(id -u):$(id -g):1" ]] \
    || fail "fault harness lock is not a private owner-only single-link regular file"
fi
exec {HARNESS_LOCK_FD}>>"$LOCK_ROOT/trnm-authority-fault-harness.lock"
validate_open_private_lock "$LOCK_ROOT/trnm-authority-fault-harness.lock" "$HARNESS_LOCK_FD"
flock -n "$HARNESS_LOCK_FD" || fail "another Authority fault harness is already running"
validate_open_private_lock "$LOCK_ROOT/trnm-authority-fault-harness.lock" "$HARNESS_LOCK_FD"
if [[ -e "$LOCK_ROOT/trnm-game-server-deploy.lock" \
    || -L "$LOCK_ROOT/trnm-game-server-deploy.lock" ]]; then
  [[ -f "$LOCK_ROOT/trnm-game-server-deploy.lock" \
      && ! -L "$LOCK_ROOT/trnm-game-server-deploy.lock" \
      && "$(stat -c '%a' "$LOCK_ROOT/trnm-game-server-deploy.lock")" == 600 \
      && "$(stat -c '%u:%g:%h' "$LOCK_ROOT/trnm-game-server-deploy.lock")" \
        == "$(id -u):$(id -g):1" ]] \
    || fail "Authority deployment lock is not a private owner-only single-link regular file"
fi
exec {DEPLOYMENT_LOCK_FD}>>"$LOCK_ROOT/trnm-game-server-deploy.lock"
validate_open_private_lock "$LOCK_ROOT/trnm-game-server-deploy.lock" "$DEPLOYMENT_LOCK_FD"
flock -n "$DEPLOYMENT_LOCK_FD" \
  || fail "Authority release deployment or another protected operation is active"
validate_open_private_lock "$LOCK_ROOT/trnm-game-server-deploy.lock" "$DEPLOYMENT_LOCK_FD"

[[ -z "$(git -C "$ROOT_DIR" status --porcelain)" ]] \
  || fail "Trillionnium worktree must be clean"
[[ -d "$CEX_ROOT/.git" && -z "$(git -C "$CEX_ROOT" status --porcelain)" ]] \
  || fail "CEX worktree must be clean"

RELEASE_JSON="$("$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$REQUESTED_RELEASE")"
jq -e '
  .contract_version == "trnm_game_server_release_verification_v1"
  and .verified == true
  and .release_contract_version == "trnm_game_server_release_v2"
  and .fault_harness_capable == true
  and .isolated_target == true
  and .trusted_target_cache_used == false
  and (.release_manifest_sha256|test("^[0-9a-f]{64}$"))
  and (.release_dir|type=="string")
  and (.git_commit|test("^[0-9a-f]{40}$"))
  and (.git_tree|test("^[0-9a-f]{40}$"))
  and (.binaries.game_server.path|type=="string")
  and (.binaries.game_server.sha256|test("^[0-9a-f]{64}$"))
  and (.binaries.online_e2e.path|type=="string")
  and (.binaries.online_e2e.sha256|test("^[0-9a-f]{64}$"))
  ' >/dev/null <<<"$RELEASE_JSON" || fail "release is not fault-harness-capable v2"
RELEASE_DIR="$(jq -r .release_dir <<<"$RELEASE_JSON")"
RELEASE_DIR="$(realpath -e -- "$RELEASE_DIR")"
RELEASE_ROOT="$(realpath -m -- "$RELEASE_ROOT")"
path_is_within "$RELEASE_DIR" "$RELEASE_ROOT" || fail "release is outside the release root"
RELEASE_ID="$(basename "$RELEASE_DIR")"
[[ "$RELEASE_ID" =~ ^[0-9a-f]{12}-[0-9a-f]{12}-[0-9a-f]{12}$ ]] \
  || fail "release ID is not a toolchain-qualified v2 release ID"
RELEASE_COMMIT="$(jq -r .git_commit <<<"$RELEASE_JSON")"
RELEASE_TREE="$(jq -r .git_tree <<<"$RELEASE_JSON")"
[[ "$(git -C "$ROOT_DIR" rev-parse HEAD)" == "$RELEASE_COMMIT" ]] \
  || fail "worktree HEAD does not match the release commit"
[[ "$(git -C "$ROOT_DIR" rev-parse 'HEAD^{tree}')" == "$RELEASE_TREE" ]] \
  || fail "worktree tree does not match the release tree"
GAME_SERVER_BIN="$(realpath -e -- "$(jq -r .binaries.game_server.path <<<"$RELEASE_JSON")")"
ONLINE_E2E_BIN="$(realpath -e -- "$(jq -r .binaries.online_e2e.path <<<"$RELEASE_JSON")")"
path_is_within "$GAME_SERVER_BIN" "$RELEASE_DIR" || fail "game-server binary escaped release"
path_is_within "$ONLINE_E2E_BIN" "$RELEASE_DIR" || fail "E2E binary escaped release"
GAME_SERVER_SHA="$(jq -r .binaries.game_server.sha256 <<<"$RELEASE_JSON")"
ONLINE_E2E_SHA="$(jq -r .binaries.online_e2e.sha256 <<<"$RELEASE_JSON")"
RELEASE_MANIFEST_SHA="$(jq -r .release_manifest_sha256 <<<"$RELEASE_JSON")"
[[ "$(sha256sum "$GAME_SERVER_BIN" | awk '{print $1}')" == "$GAME_SERVER_SHA" ]] \
  || fail "verified game-server binary changed after release verification"
[[ "$(sha256sum "$ONLINE_E2E_BIN" | awk '{print $1}')" == "$ONLINE_E2E_SHA" ]] \
  || fail "verified E2E binary changed after release verification"
[[ "$(sha256sum "$RELEASE_DIR/release-manifest.json" | awk '{print $1}')" == "$RELEASE_MANIFEST_SHA" ]] \
  || fail "verified release manifest changed after release verification"

ORIGINAL_RELEASE_DIR="$(realpath -e -- "$RELEASE_ROOT/current")"
[[ "$ORIGINAL_RELEASE_DIR" == "$RELEASE_DIR" ]] \
  || fail "fault drill release must be the currently selected release"
ORIGINAL_RELEASE_SHA="$(sha256sum "$ORIGINAL_RELEASE_DIR/trnm-game-server" | awk '{print $1}')"
ORIGINAL_ACTIVE_STATE="$(unit_property ActiveState)"
ORIGINAL_SUB_STATE="$(unit_property SubState)"
ORIGINAL_MAIN_PID="$(unit_property MainPID)"
ORIGINAL_UNIT_FILE_STATE="$(unit_property UnitFileState)"
[[ "$ORIGINAL_ACTIVE_STATE" == active || "$ORIGINAL_ACTIVE_STATE" == inactive ]] \
  || fail "service must be stably active or inactive before the drill"
SERVICE_STATE_CAPTURED=1
if [[ "$CONTRACT_MODE" == 0 && "$ORIGINAL_ACTIVE_STATE" == active ]]; then
  fault_active_cgroup_matches_source "$SERVICE" \
    || fail "running production Authority cgroup differs from the source resource contract"
fi

# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env
if [[ "$CONTRACT_MODE" == 0 ]]; then
  for forbidden_name in BASH_ENV ENV CDPATH GLOBIGNORE LD_PRELOAD LD_LIBRARY_PATH \
    PYTHONPATH PYTHONHOME PERL5LIB RUBYLIB NODE_OPTIONS NODE_PATH \
    GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM \
    HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy; do
    [[ ! -v "$forbidden_name" ]] \
      || fail "canonical CEX environment injected prohibited $forbidden_name"
  done
  unset forbidden_name
fi
DIRECT_DATABASE_URL="$(cex_effective_database_url)"
[[ -n "${IDENTITY_ADMIN_TOKEN:-}" ]] || fail "identity admin token is unavailable"
mapfile -t database_parts < <(DATABASE_URL_INPUT="$DIRECT_DATABASE_URL" python3 - <<'PY'
import os, urllib.parse
u = urllib.parse.urlsplit(os.environ["DATABASE_URL_INPUT"])
if u.scheme not in {"postgres", "postgresql"}:
    raise SystemExit("database URL is not PostgreSQL")
if u.hostname not in {"127.0.0.1", "localhost"}:
    raise SystemExit("pg-rtt100 only permits a loopback PostgreSQL target")
port = u.port or 5432
if port == 7543:
    raise SystemExit("database target already uses the proxy port")
host = "127.0.0.1"
userinfo = ""
if u.username is not None:
    userinfo = urllib.parse.quote(urllib.parse.unquote(u.username), safe="")
    if u.password is not None:
        userinfo += ":" + urllib.parse.quote(urllib.parse.unquote(u.password), safe="")
    userinfo += "@"
proxy = urllib.parse.urlunsplit((u.scheme, f"{userinfo}127.0.0.1:7543", u.path, u.query, u.fragment))
print(host)
print(port)
print(proxy)
print(urllib.parse.unquote(u.password or ""))
PY
)
(( ${#database_parts[@]} == 4 )) || fail "could not derive the local PostgreSQL proxy URL"
DB_TARGET_HOST="${database_parts[0]}"
DB_TARGET_PORT="${database_parts[1]}"
PROXY_DATABASE_URL="${database_parts[2]}"
DATABASE_PASSWORD="${database_parts[3]}"

ledger_readiness="$(curl -fsS --max-time 5 "$LEDGER_URL/v1/trnm/economy/readiness")"
signer_readiness="$(curl -fsS --max-time 5 "$SIGNER_URL/v1/signer/readiness")"
jq -e '.status=="ok" and .postgres_healthy==true' >/dev/null <<<"$ledger_readiness" \
  || fail "CEX ledger is not ready"
jq -e '.status=="ok" and .postgres_receipts==true' >/dev/null <<<"$signer_readiness" \
  || fail "entitlement signer is not ready"
[[ "$(database_running_count)" == 0 ]] || fail "database still contains a running match"

if [[ "$ORIGINAL_ACTIVE_STATE" == active ]]; then
  production_readiness="$(readiness_json "$PRODUCTION_URL")"
  jq -e '.status=="ok" and .active_matches==0 and .active_match_actors==0' \
    >/dev/null <<<"$production_readiness" || fail "production Authority is not idle and ready"
  assert_loopback_listener 7005
  verify_active_service_binary || fail "running production service is not the selected release binary"
  ORIGINAL_RUNTIME_VERIFIED=1
fi
port_is_free "$TEST_SERVER_PORT" || fail "test server port $TEST_SERVER_PORT is already in use"
port_is_free "$PROXY_PORT" || fail "database proxy port $PROXY_PORT is already in use"
qdisc_is_default_noqueue || fail "loopback qdisc must start at default noqueue"
sudo -n true >/dev/null 2>&1 || fail "passwordless non-interactive tc privilege is unavailable"

[[ -d "$CANONICAL_JOURNAL" && ! -L "$CANONICAL_JOURNAL" ]] \
  || fail "canonical published-tick journal is missing or is a symlink"
[[ "$(stat -c '%a' "$CANONICAL_JOURNAL")" == 700 ]] \
  || fail "canonical published-tick journal must have mode 0700"
[[ "$(stat -c '%u' "$CANONICAL_JOURNAL")" == "$(id -u)" ]] \
  || fail "canonical published-tick journal must be owned by the current user"
for journal_control in \
  "$CANONICAL_JOURNAL/.published-tick-owner.json" \
  "$CANONICAL_JOURNAL/.published-tick.lock"; do
  [[ -f "$journal_control" && ! -L "$journal_control" \
      && "$(stat -c '%a' "$journal_control")" == 600 \
      && "$(stat -c '%u' "$journal_control")" == "$(id -u)" \
      && "$(stat -c '%h' "$journal_control")" == 1 ]] \
    || fail "journal control file is not a private, single-link regular file: $journal_control"
done

RUN_STARTED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
nonce="$(printf '%06d' "$(( (RANDOM << 1 ^ RANDOM) % 1000000 ))")"
RUN_ID="pg-rtt100-${RELEASE_COMMIT:0:12}-$(date -u +%Y%m%dT%H%M%SZ)-$nonce"
RUN_DIR="$EVIDENCE_ROOT/$RUN_ID"
mkdir -m 0700 -- "$RUN_DIR" || fail "could not create a unique fault-evidence run directory"
RUN_DIR="$(realpath -e -- "$RUN_DIR")"
path_is_within "$RUN_DIR" "$(realpath -e -- "$EVIDENCE_ROOT")" \
  || fail "run directory escaped the evidence root"
TEST_INSTANCE_ID="trnm-pg-rtt100-${RELEASE_COMMIT:0:8}-$nonce"
PHYSICAL_HOST_ID="host-$(sha256sum /etc/machine-id | cut -c1-24)"
export TRNM_FAULT_EXPECTED_INSTANCE_ID="$TEST_INSTANCE_ID"
export TRNM_FAULT_EXPECTED_PHYSICAL_HOST_ID="$PHYSICAL_HOST_ID"

jq -n --arg contract_version trnm_online_authority_fault_service_before_v1 \
  --arg active_state "$ORIGINAL_ACTIVE_STATE" --arg sub_state "$ORIGINAL_SUB_STATE" \
  --arg main_pid "$ORIGINAL_MAIN_PID" --arg unit_file_state "$ORIGINAL_UNIT_FILE_STATE" \
  --arg release_dir "$ORIGINAL_RELEASE_DIR" --arg binary_sha256 "$ORIGINAL_RELEASE_SHA" \
  '{contract_version:$contract_version,active_state:$active_state,sub_state:$sub_state,
    main_pid:($main_pid|tonumber),unit_file_state:$unit_file_state,
    release_dir:$release_dir,binary_sha256:$binary_sha256}' \
  | atomic_write "$RUN_DIR/service-before.json"

capture_journal "$RUN_DIR/journal-before.json"
JOURNAL_BEFORE_DIGEST="$(jq -r .inventory_digest "$RUN_DIR/journal-before.json")"
JOURNAL_OWNER_SHA="$(jq -r '.owner_manifest_sha256 // ""' "$RUN_DIR/journal-before.json")"
SCRIPT_SHA="$(sha256sum "${BASH_SOURCE[0]}" | awk '{print $1}')"
CEX_COMMIT="$(git -C "$CEX_ROOT" rev-parse HEAD)"
jq -n --arg contract_version "$HARNESS_CONTRACT" --arg profile "$PROFILE" \
  --arg run_id "$RUN_ID" --arg started_at "$RUN_STARTED_AT" \
  --arg script_sha256 "$SCRIPT_SHA" --arg release_id "$RELEASE_ID" \
  --arg release_manifest_sha256 "$(sha256sum "$RELEASE_DIR/release-manifest.json" | awk '{print $1}')" \
  --arg git_commit "$RELEASE_COMMIT" --arg git_tree "$RELEASE_TREE" \
  --arg game_server_sha256 "$GAME_SERVER_SHA" --arg online_e2e_sha256 "$ONLINE_E2E_SHA" \
  --arg unit_source_sha256 "$(sha256sum "$SOURCE_UNIT" | awk '{print $1}')" \
  --arg unit_installed_sha256 "$(sha256sum "$INSTALLED_UNIT" | awk '{print $1}')" \
  --arg journal_root "$CANONICAL_JOURNAL" --arg journal_before_digest "$JOURNAL_BEFORE_DIGEST" \
  --arg journal_owner_sha256 "$JOURNAL_OWNER_SHA" --arg cex_git_commit "$CEX_COMMIT" \
  --arg kernel "$(uname -srmo)" --arg host "$(hostname)" \
  --argjson contract_mode "$CONTRACT_MODE" \
  --argjson original_runtime_verified "$ORIGINAL_RUNTIME_VERIFIED" \
  '{contract_version:$contract_version,profile:$profile,run_id:$run_id,started_at:$started_at,
    harness_script_sha256:$script_sha256,release_id:$release_id,git_commit:$git_commit,git_tree:$git_tree,
    release_manifest_sha256:$release_manifest_sha256,
    binaries:{game_server_sha256:$game_server_sha256,online_e2e_sha256:$online_e2e_sha256},
    systemd_unit:{source_sha256:$unit_source_sha256,installed_sha256:$unit_installed_sha256,exact_match:true},
    journal:{root:$journal_root,before_inventory_digest:$journal_before_digest,
      owner_manifest_sha256:$journal_owner_sha256},
    cex:{git_commit:$cex_git_commit,worktree_clean:true},kernel:$kernel,host:$host,
    worktree_clean:true,release_commit_matches_head:true,contract_test_mode:($contract_mode==1),
    release_contract_version:"trnm_game_server_release_v2",fault_harness_capable:true,
    original_runtime_binary_verified:(if $contract_mode==1 then false else ($original_runtime_verified==1) end),
    secrets_persisted:false,local_only:true,public_launch_credit:false,
    cross_host_rpo_zero_credit:false}' | atomic_write "$RUN_DIR/provenance.json"

jq -n --arg contract_version trnm_online_authority_fault_configuration_v1 \
  --arg profile "$PROFILE" --arg bind "$TEST_URL" --arg instance "$TEST_INSTANCE_ID" \
  --arg physical_host "$PHYSICAL_HOST_ID" --arg journal "$CANONICAL_JOURNAL" \
  --argjson proxy_port "$PROXY_PORT" --argjson target_rtt_ms 100 \
  '{contract_version:$contract_version,profile:$profile,local_only:true,public_launch_credit:false,
    bind_url:$bind,fleet_instance_id:$instance,physical_host_id:$physical_host,
    canonical_journal:$journal,database_proxy_port:$proxy_port,target_database_rtt_ms:$target_rtt_ms,
    e2e_restart_server:false,arbitrary_command_execution:false,
    cleanup_boundary:"EXIT/INT/TERM/HUP only; SIGKILL and power loss are not trappable"}' \
  | atomic_write "$RUN_DIR/configuration.json"

trap finalize_on_exit EXIT
trap 'SIGNAL_NAME=INT; exit 130' INT
trap 'SIGNAL_NAME=TERM; exit 143' TERM
trap 'SIGNAL_NAME=HUP; exit 129' HUP

if [[ "$CONTRACT_MODE" == 0 ]]; then
  capture_dependency_process_identity \
    cex-trnm-ledger.service 7002 "$RUN_DIR/ledger-identity-start.json"
  capture_dependency_process_identity \
    trnm-entitlement-signer.service 7010 "$RUN_DIR/signer-identity-start.json"
fi
validate_open_private_lock "$LOCK_ROOT/trnm-authority-fault-harness.lock" "$HARNESS_LOCK_FD"
validate_open_private_lock "$LOCK_ROOT/trnm-game-server-deploy.lock" "$DEPLOYMENT_LOCK_FD"
if [[ "$ORIGINAL_ACTIVE_STATE" == active ]]; then
  systemctl --user stop "$SERVICE"
  wait_for_unit_state inactive
fi
wait_for_port_state 7005 free
wait_for_port_state "$TEST_SERVER_PORT" free
wait_for_port_state "$PROXY_PORT" free
assert_no_game_server_processes
[[ "$(database_running_count)" == 0 ]] \
  || fail "a running match appeared across the production stop boundary"

exec {JOURNAL_LOCK_FD}>>"$CANONICAL_JOURNAL/.published-tick.lock"
flock -n "$JOURNAL_LOCK_FD" || fail "canonical journal did not become exclusively available"
flock -u "$JOURNAL_LOCK_FD"
exec {JOURNAL_LOCK_FD}>&-
JOURNAL_LOCK_FD=""

(
  close_inherited_mutation_locks
  exec setsid python3 -u - trnm-fault-proxy \
    "$DB_TARGET_HOST" "$DB_TARGET_PORT" "$PROXY_PORT" <<'PY'
import asyncio
import signal
import sys

if len(sys.argv) != 5 or sys.argv[1] != "trnm-fault-proxy":
    raise SystemExit("invalid TRNM fault proxy invocation")
target_host = sys.argv[2]
target_port = int(sys.argv[3])
listen_port = int(sys.argv[4])

async def close_writer(writer):
    writer.close()
    try:
        await writer.wait_closed()
    except (BrokenPipeError, ConnectionResetError):
        pass

async def pump(reader, writer):
    try:
        while data := await reader.read(64 * 1024):
            writer.write(data)
            await writer.drain()
    finally:
        try:
            writer.write_eof()
        except (AttributeError, OSError, RuntimeError):
            pass

async def proxy(client_reader, client_writer):
    upstream_writer = None
    try:
        upstream_reader, upstream_writer = await asyncio.open_connection(
            target_host, target_port
        )
        await asyncio.gather(
            pump(client_reader, upstream_writer),
            pump(upstream_reader, client_writer),
        )
    except (ConnectionError, OSError, asyncio.IncompleteReadError) as error:
        print(f"proxy connection failed: {error}", file=sys.stderr, flush=True)
    finally:
        await close_writer(client_writer)
        if upstream_writer is not None:
            await close_writer(upstream_writer)

async def main():
    server = await asyncio.start_server(
        proxy, "127.0.0.1", listen_port, reuse_address=True
    )
    loop = asyncio.get_running_loop()
    stop = asyncio.Event()
    for signum in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
        loop.add_signal_handler(signum, stop.set)
    async with server:
        serve = asyncio.create_task(server.serve_forever())
        await stop.wait()
        serve.cancel()
        try:
            await serve
        except asyncio.CancelledError:
            pass

asyncio.run(main())
PY
) >"$RUN_DIR/proxy.log" 2>&1 &
PROXY_PID=$!
register_cleanup_process "$PROXY_PID" group "$(command -v python3)"
wait_for_port_state "$PROXY_PORT" listening
capture_bound_process_identity \
  "$PROXY_PID" "$(command -v python3)" "$PROXY_PORT" "$RUN_DIR/proxy-identity-start.json" proxy

(
  close_inherited_mutation_locks
  export DATABASE_URL="$PROXY_DATABASE_URL"
  export TRNM_CEX_LEDGER_URL="$LEDGER_URL"
  export TRNM_GAME_AUTHORITY_TOKEN="trnm-game-authority-v1:$IDENTITY_ADMIN_TOKEN"
  export TRNM_MODERATOR_TOKEN="trnm-moderator-v1:$IDENTITY_ADMIN_TOKEN"
  export TRNM_ENTITLEMENT_SIGNER_URL="$SIGNER_URL"
  export TRNM_ENTITLEMENT_SIGNER_TOKEN="trnm-isolated-signer-v1:$IDENTITY_ADMIN_TOKEN"
  export TRNM_ASSET_ROOT="$ROOT_DIR/assets"
  export TRNM_PUBLISHED_TICK_JOURNAL_DIR="$CANONICAL_JOURNAL"
  export TRNM_GAME_SERVER_BIND_ADDR="127.0.0.1:$TEST_SERVER_PORT"
  export TRNM_GAME_SERVER_TICK_MS=100
  export TRNM_FLEET_INSTANCE_ID="$TEST_INSTANCE_ID"
  export TRNM_FLEET_REGION=local-fault-evidence
  export TRNM_FLEET_PUBLIC_ENDPOINT="$TEST_URL"
  export TRNM_FLEET_PHYSICAL_HOST_ID="$PHYSICAL_HOST_ID"
  export TRNM_FLEET_CAPACITY=4
  export TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE=600
  export TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES=262144
  exec setsid "$GAME_SERVER_BIN"
) >"$RUN_DIR/server.log" 2>&1 &
SERVER_PID=$!
register_cleanup_process "$SERVER_PID" group "$GAME_SERVER_BIN"
wait_for_port_state "$TEST_SERVER_PORT" listening
wait_for_test_readiness
assert_loopback_listener "$TEST_SERVER_PORT"
capture_bound_process_identity \
  "$SERVER_PID" "$GAME_SERVER_BIN" "$TEST_SERVER_PORT" "$RUN_DIR/server-identity-start.json" server

if flock -n "$CANONICAL_JOURNAL/.published-tick.lock" -c true >/dev/null 2>&1; then
  fail "standalone Authority did not retain the canonical journal lock"
fi

IFS=$'\t' read -r HOST_PLAYER HOST_ACCOUNT HOST_SESSION < <(create_identity host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_ACCOUNT GUEST_SESSION < <(create_identity guest)

configure_netem
sample_readiness
(
  close_inherited_mutation_locks
  monitor_readiness
) &
MONITOR_PID=$!
register_cleanup_process "$MONITOR_PID" pid "/proc/$$/exe"

(
  close_inherited_mutation_locks
  export TRNM_GAME_SERVER_URL="$TEST_URL"
  export TRNM_ONLINE_HOST_PLAYER_ID="$HOST_PLAYER"
  export TRNM_ONLINE_HOST_ACCOUNT_ID="$HOST_ACCOUNT"
  export TRNM_ONLINE_HOST_SESSION="$HOST_SESSION"
  export TRNM_ONLINE_GUEST_PLAYER_ID="$GUEST_PLAYER"
  export TRNM_ONLINE_GUEST_ACCOUNT_ID="$GUEST_ACCOUNT"
  export TRNM_ONLINE_GUEST_SESSION="$GUEST_SESSION"
  export TRNM_ONLINE_E2E_RESTART_SERVER=0
  export TRNM_ONLINE_E2E_EFFECT_SAMPLES=20
  export TRNM_ONLINE_E2E_MAX_EFFECT_P95_MS=300
  export TRNM_ONLINE_E2E_PHASE_TIMEOUT_SECONDS=900
  export TRNM_ONLINE_E2E_COMPLETION_TIMEOUT_SECONDS=1200
  exec setsid timeout --signal=TERM --kill-after=10s "$E2E_TIMEOUT_SECONDS" "$ONLINE_E2E_BIN"
) >"$RUN_DIR/e2e-report.json.tmp" 2>"$RUN_DIR/e2e.stderr" &
E2E_PID=$!
register_cleanup_process "$E2E_PID" group "$(command -v timeout)"
set +e
wait "$E2E_PID"
WORKLOAD_RC=$?
set -e
E2E_PID=""
if kill -0 "$MONITOR_PID" >/dev/null 2>&1 \
    && [[ -f "$RUN_DIR/readiness-monitor.started" \
      && ! -e "$RUN_DIR/readiness-monitor.failed" ]]; then
  MONITOR_SURVIVED_WORKLOAD=1
else
  fail "readiness monitor exited before the workload completed"
fi
terminate_group "$MONITOR_PID" readiness-monitor
MONITOR_PID=""

capture_bound_process_identity \
  "$PROXY_PID" "$(command -v python3)" "$PROXY_PORT" "$RUN_DIR/proxy-identity-end.json" proxy
capture_bound_process_identity \
  "$SERVER_PID" "$GAME_SERVER_BIN" "$TEST_SERVER_PORT" "$RUN_DIR/server-identity-end.json" server
if [[ "$CONTRACT_MODE" == 0 ]]; then
  capture_dependency_process_identity \
    cex-trnm-ledger.service 7002 "$RUN_DIR/ledger-identity-end.json"
  capture_dependency_process_identity \
    trnm-entitlement-signer.service 7010 "$RUN_DIR/signer-identity-end.json"
fi

if [[ "$WORKLOAD_RC" -eq 0 ]] \
    && jq -e '.status=="passed"' "$RUN_DIR/e2e-report.json.tmp" >/dev/null 2>&1; then
  mv -f -- "$RUN_DIR/e2e-report.json.tmp" "$RUN_DIR/e2e-report.json"
else
  mv -f -- "$RUN_DIR/e2e-report.json.tmp" "$RUN_DIR/e2e-output-invalid.json" 2>/dev/null || true
fi

sudo -n tc -s qdisc show dev lo >"$RUN_DIR/qdisc-e2e-after.txt"
QDISC_PACKETS_AFTER="$(netem_packet_count)"
[[ "$QDISC_PACKETS_AFTER" =~ ^[0-9]+$ ]] || QDISC_PACKETS_AFTER=0
MATCH_ID=""
[[ -f "$RUN_DIR/e2e-report.json" ]] && MATCH_ID="$(jq -r '.match_id // empty' "$RUN_DIR/e2e-report.json")"
[[ -z "$MATCH_ID" || "$MATCH_ID" =~ ^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$ ]] \
  || fail "E2E returned an invalid match id"
capture_database_terminal "$RUN_DIR/database-terminal.json"
capture_journal "$RUN_DIR/journal-after.json" "$MATCH_ID"
capture_bound_inputs_after || fail "release, source, unit, or CEX inputs changed during the drill"
WORKLOAD_COMPLETE=1
exit "$WORKLOAD_RC"
