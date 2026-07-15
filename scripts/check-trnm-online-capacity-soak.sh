#!/usr/bin/bash
set -euo pipefail

# PATH does not outrank imported Bash functions.  Clear every function before
# the first external command, then enter once through a deliberately minimal
# environment.  The marker is only a recursion guard; functions are cleared on
# both sides of the exec so an inherited marker cannot restore command
# shadowing.
while IFS= read -r inherited_function_name; do
  builtin unset -f "$inherited_function_name"
done < <(builtin compgen -A function)
unset inherited_function_name

readonly TRUSTED_FORMAL_PATH="/usr/sbin:/usr/bin"
readonly EXPECTED_SCOPE_MEMORY_HIGH_BYTES=1610612736
readonly EXPECTED_SCOPE_MEMORY_MAX_BYTES=2147483648
readonly EXPECTED_SCOPE_MEMORY_SWAP_MAX_BYTES=536870912
readonly EXPECTED_SCOPE_CPU_MAX="150000 100000"
readonly EXPECTED_SCOPE_TASKS_MAX=512
readonly FORMAL_MIN_CONCURRENCY=4
readonly FORMAL_MAX_CONCURRENCY=32
readonly FORMAL_MIN_DURATION_SECONDS=7200
readonly FORMAL_MAX_DURATION_SECONDS=86400
readonly FORMAL_MIN_AVAILABLE_MIB=3072
readonly FORMAL_MAX_AVAILABLE_MIB=131072
readonly FORMAL_MIN_DATABASE_CONNECTIONS=1
readonly FORMAL_MAX_DATABASE_CONNECTIONS=40
readonly FORMAL_MIN_MONITOR_INTERVAL_SECONDS=1
readonly FORMAL_MAX_MONITOR_INTERVAL_SECONDS=10
readonly CURL_CONNECT_TIMEOUT_SECONDS=5
readonly CURL_REQUEST_TIMEOUT_SECONDS=30
readonly EXTERNAL_COMMAND_TIMEOUT_SECONDS=30
readonly WORKER_TIMEOUT_SECONDS=1800
readonly WORKER_TERM_GRACE_SECONDS=10
readonly WORKER_KILL_GRACE_SECONDS=5
readonly CLEANUP_TOTAL_TIMEOUT_SECONDS=240

early_fail() {
  echo "TRNM capacity evidence preflight failed: $*" >&2
  exit 2
}

for forbidden_name in \
  BASH_ENV ENV CDPATH LD_PRELOAD LD_LIBRARY_PATH PYTHONHOME PYTHONPATH \
  GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM CURL_HOME \
  HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy \
  DOCKER_HOST DOCKER_CONTEXT DOCKER_CONFIG DOCKER_TLS_VERIFY DOCKER_CERT_PATH \
  CEX_PROJECT_ROOT CEX_ENV_FILE CEX_POSTGRES_CONTAINER_NAME CEX_POSTGRES_USER \
  CEX_POSTGRES_DB CEX_POSTGRES_PASSWORD CEX_DOCKER_USE_SUDO DATABASE_URL \
  LEDGER_ADMIN_TOKEN IDENTITY_ADMIN_TOKEN TRNM_CEX_LEDGER_URL \
  TRNM_ENTITLEMENT_SIGNER_URL TRNM_GAME_SERVER_URL TRNM_GAME_SERVER_RELEASE_DIR \
  TRNM_CAPACITY_ALLOW_DIRTY; do
  [[ ! -v "$forbidden_name" ]] \
    || early_fail "external $forbidden_name overrides are prohibited for formal evidence"
done

case "${TRNM_CAPACITY_SANITIZED_ENTRY:-0}" in
0)
  sanitized_script="$(/usr/bin/realpath -e -- "${BASH_SOURCE[0]}")" \
    || early_fail "capacity harness script path is not canonical"
  sanitized_environment=(
    PATH="$TRUSTED_FORMAL_PATH"
    LC_ALL=C
    HOME="${HOME:-}"
    TRNM_CAPACITY_SANITIZED_ENTRY=1
  )
  for allowed_name in XDG_RUNTIME_DIR \
    TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE TRNM_CAPACITY_SCOPE_PROBE \
    TRNM_CAPACITY_CONCURRENCY TRNM_CAPACITY_DURATION_SECONDS \
    TRNM_CAPACITY_MIN_AVAILABLE_MIB TRNM_CAPACITY_MAX_DATABASE_CONNECTIONS \
    TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS TRNM_CAPACITY_RUN_ID; do
    if [[ -v "$allowed_name" ]]; then
      sanitized_environment+=("$allowed_name=${!allowed_name}")
    fi
  done
  exec /usr/bin/env -i "${sanitized_environment[@]}" "$sanitized_script" "$@"
  ;;
1)
  unset TRNM_CAPACITY_SANITIZED_ENTRY
  ;;
*)
  early_fail "TRNM_CAPACITY_SANITIZED_ENTRY is an internal recursion guard"
  ;;
esac

while IFS= read -r environment_name; do
  case "$environment_name" in
    PATH|LC_ALL|HOME|XDG_RUNTIME_DIR|PWD|SHLVL|_|\
      TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE|TRNM_CAPACITY_SCOPE_PROBE|\
      TRNM_CAPACITY_CONCURRENCY|TRNM_CAPACITY_DURATION_SECONDS|\
      TRNM_CAPACITY_MIN_AVAILABLE_MIB|TRNM_CAPACITY_MAX_DATABASE_CONNECTIONS|\
      TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS|TRNM_CAPACITY_RUN_ID)
      ;;
    *)
      builtin unset "$environment_name" 2>/dev/null \
        || builtin export -n "$environment_name" 2>/dev/null \
        || early_fail "could not clear inherited environment variable: $environment_name"
      ;;
  esac
done < <(builtin compgen -e)
unset environment_name

export PATH="$TRUSTED_FORMAL_PATH"
export LC_ALL=C
export NO_PROXY="127.0.0.1,localhost"
export no_proxy="$NO_PROXY"
unset BASH_ENV ENV CDPATH LD_PRELOAD LD_LIBRARY_PATH PYTHONHOME PYTHONPATH \
  GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM CURL_HOME \
  HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy \
  DOCKER_HOST DOCKER_CONTEXT DOCKER_CONFIG DOCKER_TLS_VERIFY DOCKER_CERT_PATH
canonical_home="$(getent passwd "$UID" | awk -F: -v uid="$UID" \
  '$3 == uid {print $6; exit}')"
canonical_user="$(getent passwd "$UID" | awk -F: -v uid="$UID" \
  '$3 == uid {print $1; exit}')"
canonical_gid="$(id -g)"
[[ -n "$canonical_home" && -n "$canonical_user" \
    && "${HOME:-}" == "$canonical_home" ]] \
  || early_fail "HOME or user identity does not match the canonical passwd entry"
canonical_runtime_dir="/run/user/$UID"
[[ ! -v XDG_RUNTIME_DIR || "$XDG_RUNTIME_DIR" == "$canonical_runtime_dir" ]] \
  || early_fail "XDG_RUNTIME_DIR is not canonical for the current user"
export XDG_RUNTIME_DIR="$canonical_runtime_dir"
export DBUS_SESSION_BUS_ADDRESS="unix:path=$canonical_runtime_dir/bus"

SCRIPT_PATH="$(realpath -e -- "${BASH_SOURCE[0]}")"
case "${TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE:-0}" in
0)
  capacity_scope_nonce="$(date -u +%Y%m%dT%H%M%S)-$$"
  exec systemd-run --user --scope --collect --quiet --expand-environment=no \
    --unit="trnm-capacity-$capacity_scope_nonce" \
    --description='TRNM bounded online capacity harness' \
    -p CPUAccounting=true -p CPUWeight=100 -p CPUQuota=150% \
    -p MemoryAccounting=true -p MemoryHigh=1536M -p MemoryMax=2048M \
    -p MemorySwapMax=512M -p IOAccounting=true -p IOWeight=100 \
    -p TasksAccounting=true -p TasksMax=512 \
    env TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE=1 "$SCRIPT_PATH" "$@"
  ;;
1)
  ;;
*)
  early_fail "TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE must be 0 or 1"
  ;;
esac

RESOURCE_CGROUP="$(awk -F: '$1 == "0" {print $3}' /proc/self/cgroup)"
RESOURCE_CGROUP_ROOT="/sys/fs/cgroup$RESOURCE_CGROUP"
validate_resource_scope() {
  local memory_high memory_max memory_swap_max cpu_max tasks_max scope_unit
  scope_unit="${RESOURCE_CGROUP##*/}"
  [[ -n "$RESOURCE_CGROUP" && "$RESOURCE_CGROUP" != / \
      && "$scope_unit" =~ ^trnm-capacity-[0-9]{8}T[0-9]{6}-[0-9]+\.scope$ \
      && -d "$RESOURCE_CGROUP_ROOT" \
      && "$(systemctl --user show "$scope_unit" -p ControlGroup --value)" == \
        "$RESOURCE_CGROUP" ]] \
    || early_fail "formal evidence is not running in a dedicated cgroup"
  for controller_file in memory.high memory.max memory.swap.max cpu.max pids.max \
    memory.events; do
    [[ -r "$RESOURCE_CGROUP_ROOT/$controller_file" ]] \
      || early_fail "resource cgroup is missing $controller_file"
  done
  memory_high="$(<"$RESOURCE_CGROUP_ROOT/memory.high")"
  memory_max="$(<"$RESOURCE_CGROUP_ROOT/memory.max")"
  memory_swap_max="$(<"$RESOURCE_CGROUP_ROOT/memory.swap.max")"
  cpu_max="$(<"$RESOURCE_CGROUP_ROOT/cpu.max")"
  tasks_max="$(<"$RESOURCE_CGROUP_ROOT/pids.max")"
  [[ "$memory_high" == "$EXPECTED_SCOPE_MEMORY_HIGH_BYTES" \
      && "$memory_max" == "$EXPECTED_SCOPE_MEMORY_MAX_BYTES" \
      && "$memory_swap_max" == "$EXPECTED_SCOPE_MEMORY_SWAP_MAX_BYTES" \
      && "$cpu_max" == "$EXPECTED_SCOPE_CPU_MAX" \
      && "$tasks_max" == "$EXPECTED_SCOPE_TASKS_MAX" ]] \
    || early_fail "actual cgroup limits do not match the formal capacity contract"
}
validate_resource_scope

case "${TRNM_CAPACITY_SCOPE_PROBE:-0}" in
0)
  ;;
1)
  jq -n \
    --arg cgroup "$RESOURCE_CGROUP" \
    --arg memory_high "$(<"$RESOURCE_CGROUP_ROOT/memory.high")" \
    --arg memory_max "$(<"$RESOURCE_CGROUP_ROOT/memory.max")" \
    --arg memory_swap_max "$(<"$RESOURCE_CGROUP_ROOT/memory.swap.max")" \
    --arg cpu_max "$(<"$RESOURCE_CGROUP_ROOT/cpu.max")" \
    --arg tasks_max "$(<"$RESOURCE_CGROUP_ROOT/pids.max")" \
    'def limit: if . == "max" then . else tonumber end;
    {status:"passed",contract_version:"trnm_capacity_resource_scope_probe_v1",
      probe_only:true,formal_evidence:false,validated:true,cgroup:$cgroup,
      memory_high_bytes:($memory_high|limit),
      memory_max_bytes:($memory_max|limit),
      memory_swap_max_bytes:($memory_swap_max|limit),cpu_max:$cpu_max,
      tasks_max:($tasks_max|limit)}'
  exit 0
  ;;
*)
  early_fail "TRNM_CAPACITY_SCOPE_PROBE must be 0 or 1"
  ;;
esac

umask 077
ROOT_DIR="$(dirname "$(dirname "$SCRIPT_PATH")")"
CEX_ROOT="$(realpath -e -- "$ROOT_DIR/../CEX")"
RUN_ROOT="$ROOT_DIR/run"
LOCK_ROOT="$RUN_ROOT/locks"
EVIDENCE_ROOT="$RUN_ROOT/online-capacity"

ensure_private_directory() {
  local directory="$1"
  [[ ! -L "$directory" ]] || early_fail "private directory is a symbolic link: $directory"
  if [[ -e "$directory" ]]; then
    [[ -d "$directory" \
        && "$(stat -c '%u:%g' "$directory")" == "$UID:$canonical_gid" ]] \
      || early_fail "private directory is not owned by the current user: $directory"
    chmod 0700 "$directory"
  else
    install -d -m 0700 -- "$directory"
  fi
  [[ -d "$directory" && ! -L "$directory" \
      && "$(stat -c '%u:%g:%a' "$directory")" == \
        "$UID:$canonical_gid:700" ]] \
    || early_fail "private directory failed owner/mode validation: $directory"
}

validate_lock_path() {
  local lock_file="$1"
  if [[ -e "$lock_file" || -L "$lock_file" ]]; then
    [[ -f "$lock_file" && ! -L "$lock_file" \
        && "$(stat -c '%u:%g:%a:%h' "$lock_file")" == \
          "$UID:$canonical_gid:600:1" ]] \
      || early_fail "lock file is not a private single-link regular file: $lock_file"
  fi
}

validate_lock_fd() {
  local fd="$1" lock_file="$2" fd_path="/proc/self/fd/$1"
  local path_dev_inode fd_dev_inode
  path_dev_inode="$(stat -c '%d:%i' "$lock_file")" || return 1
  fd_dev_inode="$(stat -Lc '%d:%i' "$fd_path")" || return 1
  [[ -f "$fd_path" && ! -L "$lock_file" \
      && "$(stat -Lc '%u:%g:%a:%h' "$fd_path")" == \
        "$UID:$canonical_gid:600:1" \
      && "$fd_dev_inode" == "$path_dev_inode" \
      && "$(readlink -e -- "$fd_path")" == "$(realpath -e -- "$lock_file")" ]] \
    || early_fail "opened lock descriptor failed identity validation: $lock_file"
}

[[ -d "$RUN_ROOT" && ! -L "$RUN_ROOT" \
    && "$(stat -c '%u:%g' "$RUN_ROOT")" == "$UID:$canonical_gid" ]] \
  || early_fail "canonical run root must be a non-symlink directory owned by the current user"
ensure_private_directory "$LOCK_ROOT"
ensure_private_directory "$EVIDENCE_ROOT"
capacity_lock="$LOCK_ROOT/trnm-online-capacity-evidence.lock"
deployment_lock="$LOCK_ROOT/trnm-game-server-deploy.lock"
validate_lock_path "$capacity_lock"
exec 9>>"$capacity_lock"
validate_lock_fd 9 "$capacity_lock"
flock -n 9 || {
  echo "another capacity evidence run is already active" >&2
  exit 2
}
validate_lock_path "$capacity_lock"
validate_lock_fd 9 "$capacity_lock"
validate_lock_path "$deployment_lock"
exec 8>>"$deployment_lock"
validate_lock_fd 8 "$deployment_lock"
flock -n 8 || {
  echo "a game-server deployment mutation is already active" >&2
  exit 2
}
validate_lock_path "$deployment_lock"
validate_lock_fd 8 "$deployment_lock"

CEX_ENV_SOURCE=""
if [[ -e "$CEX_ROOT/.env" || -L "$CEX_ROOT/.env" ]]; then
  [[ -f "$CEX_ROOT/.env" && ! -L "$CEX_ROOT/.env" ]] \
    || early_fail "canonical CEX .env must be a regular non-symlink file"
  CEX_ENV_SOURCE="$CEX_ROOT/.env"
elif [[ -e "$CEX_ROOT/.env.example" || -L "$CEX_ROOT/.env.example" ]]; then
  [[ -f "$CEX_ROOT/.env.example" && ! -L "$CEX_ROOT/.env.example" ]] \
    || early_fail "canonical CEX .env.example must be a regular non-symlink file"
  CEX_ENV_SOURCE="$CEX_ROOT/.env.example"
fi
CEX_ENV_SHA256=""
if [[ -n "$CEX_ENV_SOURCE" ]]; then
  CEX_ENV_SOURCE="$(realpath -e -- "$CEX_ENV_SOURCE")"
  CEX_ENV_SHA256="$(sha256sum "$CEX_ENV_SOURCE" | awk '{print $1}')"
fi
CEX_HELPER="$CEX_ROOT/scripts/_dev-helpers.sh"
[[ -f "$CEX_HELPER" && ! -L "$CEX_HELPER" ]] \
  || early_fail "canonical CEX helper must be a regular non-symlink file"
CEX_HELPER_SHA256="$(sha256sum "$CEX_HELPER" | awk '{print $1}')"
# shellcheck source=/dev/null
source "$CEX_HELPER"
if [[ -n "$CEX_ENV_SOURCE" ]]; then
  cex_load_env "$CEX_ENV_SOURCE"
fi

export PATH="$TRUSTED_FORMAL_PATH"
export HOME="$canonical_home"
export USER="$canonical_user"
export LOGNAME="$canonical_user"
export XDG_RUNTIME_DIR="$canonical_runtime_dir"
export DBUS_SESSION_BUS_ADDRESS="unix:path=$canonical_runtime_dir/bus"
export NO_PROXY="127.0.0.1,localhost"
export no_proxy="$NO_PROXY"
unset BASH_ENV ENV CDPATH LD_PRELOAD LD_LIBRARY_PATH PYTHONHOME PYTHONPATH \
  GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM CURL_HOME \
  HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy \
  DOCKER_HOST DOCKER_CONTEXT DOCKER_CONFIG DOCKER_TLS_VERIFY DOCKER_CERT_PATH
[[ "$(realpath -e -- "$CEX_PROJECT_ROOT")" == "$CEX_ROOT" ]] \
  || early_fail "loaded CEX helper root is not canonical"
for url_contract in \
  "TRNM_CEX_LEDGER_URL:http://127.0.0.1:7002" \
  "TRNM_ENTITLEMENT_SIGNER_URL:http://127.0.0.1:7010" \
  "TRNM_GAME_SERVER_URL:http://127.0.0.1:7005"; do
  IFS=: read -r url_name url_scheme url_rest <<<"$url_contract"
  expected_url="$url_scheme:$url_rest"
  [[ ! -v "$url_name" || "${!url_name}" == "$expected_url" ]] \
    || early_fail "$url_name in the canonical CEX environment is not loopback"
done
cex_effective_database_url | python3 -c '
import sys
from urllib.parse import urlsplit

value = urlsplit(sys.stdin.read().strip())
if value.scheme not in {"postgres", "postgresql"}:
    raise SystemExit("formal CEX database URL is not PostgreSQL")
if value.hostname not in {"127.0.0.1", "localhost"}:
    raise SystemExit("formal CEX database URL is not loopback")
' || early_fail "canonical CEX environment does not select loopback PostgreSQL"
readonly LEDGER_URL="http://127.0.0.1:7002"
readonly SIGNER_URL="http://127.0.0.1:7010"
readonly ONLINE_URL="http://127.0.0.1:7005"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
CONCURRENCY="${TRNM_CAPACITY_CONCURRENCY:-4}"
DURATION_SECONDS="${TRNM_CAPACITY_DURATION_SECONDS:-7200}"
MIN_AVAILABLE_MIB="${TRNM_CAPACITY_MIN_AVAILABLE_MIB:-3072}"
MAX_DATABASE_CONNECTIONS="${TRNM_CAPACITY_MAX_DATABASE_CONNECTIONS:-40}"
MONITOR_INTERVAL_SECONDS="${TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS:-10}"
RUN_ID="${TRNM_CAPACITY_RUN_ID:-capacity-$(date +%s)-${RANDOM}}"
if [[ ! "$RUN_ID" =~ ^capacity-[0-9]+-[0-9]+$ ]]; then
  echo "TRNM_CAPACITY_RUN_ID must match capacity-EPOCH-NONCE" >&2
  exit 2
fi
EVIDENCE="$EVIDENCE_ROOT/$RUN_ID"
SAMPLES="$EVIDENCE/operational-samples.jsonl"

require_integer_range() {
  local name="$1" value="$2" minimum="$3" maximum="$4"
  [[ "$value" =~ ^[1-9][0-9]*$ \
      && "$value" -ge "$minimum" && "$value" -le "$maximum" ]] \
    || early_fail "$name must be an integer between $minimum and $maximum"
}
require_integer_range TRNM_CAPACITY_CONCURRENCY "$CONCURRENCY" \
  "$FORMAL_MIN_CONCURRENCY" "$FORMAL_MAX_CONCURRENCY"
require_integer_range TRNM_CAPACITY_DURATION_SECONDS "$DURATION_SECONDS" \
  "$FORMAL_MIN_DURATION_SECONDS" "$FORMAL_MAX_DURATION_SECONDS"
require_integer_range TRNM_CAPACITY_MIN_AVAILABLE_MIB "$MIN_AVAILABLE_MIB" \
  "$FORMAL_MIN_AVAILABLE_MIB" "$FORMAL_MAX_AVAILABLE_MIB"
require_integer_range TRNM_CAPACITY_MAX_DATABASE_CONNECTIONS \
  "$MAX_DATABASE_CONNECTIONS" "$FORMAL_MIN_DATABASE_CONNECTIONS" \
  "$FORMAL_MAX_DATABASE_CONNECTIONS"
require_integer_range TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS \
  "$MONITOR_INTERVAL_SECONDS" "$FORMAL_MIN_MONITOR_INTERVAL_SECONDS" \
  "$FORMAL_MAX_MONITOR_INTERVAL_SECONDS"
mkdir "$EVIDENCE" || {
  echo "capacity evidence directory already exists: $EVIDENCE" >&2
  exit 2
}

run_release_verification() (
  exec 8>&- 9>&-
  timeout --signal=TERM --kill-after=5s 120 \
    "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$@"
)

release_verification="$(
  run_release_verification "$ROOT_DIR/run/releases/trnm-game-server/current"
)" || {
  echo "capacity evidence requires a verified immutable game-server release" >&2
  exit 2
}
if ! jq -e '
    .verified == true and
    .release_contract_version == "trnm_game_server_release_v2" and
    .fault_harness_capable == true and
    (.binaries.game_server.path | type == "string" and length > 0) and
    (.binaries.online_e2e.path | type == "string" and length > 0)
  ' >/dev/null <<<"$release_verification"; then
  echo "capacity evidence requires a fault-harness-capable v2 release bundle" >&2
  exit 2
fi
RELEASE_DIR="$(jq -er '.release_dir' <<<"$release_verification")"
release_root_real="$(realpath -e -- "$ROOT_DIR/run/releases/trnm-game-server")"
[[ "$(dirname "$RELEASE_DIR")" == "$release_root_real" ]] \
  || early_fail "verified release is not a direct child of the canonical release root"
CURRENT_RELEASE_SELECTOR="$ROOT_DIR/run/releases/trnm-game-server/current"
[[ "$(realpath -e -- "$CURRENT_RELEASE_SELECTOR")" == "$RELEASE_DIR" ]] \
  || early_fail "current release selector changed during release verification"
RELEASE_VERIFICATION_SHA256="$(
  jq -S -c . <<<"$release_verification" | sha256sum | awk '{print $1}'
)"
GAME_SERVER_BINARY="$(jq -er '.binaries.game_server.path' <<<"$release_verification")"
E2E_BINARY="$(jq -er '.binaries.online_e2e.path' <<<"$release_verification")"
RELEASE_MANIFEST="$RELEASE_DIR/release-manifest.json"
RELEASE_MANIFEST_SHA256="$(sha256sum "$RELEASE_MANIFEST" | awk '{print $1}')"
CEX_RUNTIME_INPUT_SHA256="$(
  printf '%s\0%s\0%s\0%s\0' \
    "$CEX_POSTGRES_CONTAINER_NAME" "$CEX_POSTGRES_USER" "$CEX_POSTGRES_DB" \
    "$(cex_effective_database_url)" | sha256sum | awk '{print $1}'
)"
worktrees_are_clean() {
  [[ -z "$(git -C "$ROOT_DIR" status --porcelain --untracked-files=all)" && \
      -z "$(git -C "$CEX_ROOT" status --porcelain --untracked-files=all)" ]]
}
worktrees_are_clean || {
  echo "capacity evidence requires clean Trillionnium and CEX worktrees, including untracked files" >&2
  exit 2
}
if [[ "$(git -C "$ROOT_DIR" rev-parse HEAD)" != \
    "$(jq -er '.git_commit' <<<"$release_verification")" ]]; then
  echo "capacity evidence requires HEAD to match the verified release commit" >&2
  exit 2
fi

available_memory_mib() {
  awk '/^MemAvailable:/ {print int($2 / 1024)}' /proc/meminfo
}

host_oom_kills() {
  awk '$1 == "oom_kill" {print $2}' /proc/vmstat
}

monotonic_seconds() {
  awk '{print int($1)}' /proc/uptime
}

unit_property() (
  local timeout_seconds=20 remaining
  if (( ${CLEANUP_DEADLINE:-0} > 0 )); then
    remaining=$((CLEANUP_DEADLINE - SECONDS))
    (( remaining > 0 )) || return 124
    (( remaining < timeout_seconds )) && timeout_seconds="$remaining"
  fi
  exec 8>&- 9>&-
  timeout --signal=TERM --kill-after=5s "$timeout_seconds" \
    systemctl --user show "$1" -p "$2" --value
)

unit_contract_values() {
  case "$1" in
  trnm-game-server.service)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$HOME/.config/systemd/user/trnm-game-server.service" \
      "$ROOT_DIR/scripts/run-trnm-game-server.sh" \
      "$ROOT_DIR/scripts/run-trnm-game-server.sh" \
      2s 402653184 536870912 134217728 256 '200000 100000'
    ;;
  trnm-entitlement-signer.service)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$HOME/.config/systemd/user/trnm-entitlement-signer.service" \
      "$ROOT_DIR/scripts/run-trnm-entitlement-signer.sh" \
      "$ROOT_DIR/scripts/run-trnm-entitlement-signer.sh" \
      500ms 67108864 100663296 33554432 128 '50000 100000'
    ;;
  cex-trnm-ledger.service)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$HOME/.config/systemd/user/cex-trnm-ledger.service" \
      "$CEX_ROOT/scripts/run-trnm-economy-service.sh" \
      "$CEX_ROOT/scripts/run-trnm-economy-service.sh ledger" \
      1s 268435456 402653184 134217728 256 '100000 100000'
    ;;
  cex-trnm-consumer.service)
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$HOME/.config/systemd/user/cex-trnm-consumer.service" \
      "$CEX_ROOT/scripts/run-trnm-economy-service.sh" \
      "$CEX_ROOT/scripts/run-trnm-economy-service.sh consumer" \
      1s 402653184 536870912 134217728 256 '100000 100000'
    ;;
  *) return 1 ;;
  esac
}

effective_unit_matches_source_contract() {
  local unit="$1" fragment expected_exec expected_argv expected_cpu
  local expected_high expected_max expected_swap expected_tasks expected_cpu_max
  local exec_start
  IFS=$'\t' read -r fragment expected_exec expected_argv expected_cpu \
    expected_high expected_max expected_swap expected_tasks expected_cpu_max \
    < <(unit_contract_values "$unit") || return 1
  [[ -z "$(unit_property "$unit" DropInPaths)" \
      && "$(realpath -e -- "$(unit_property "$unit" FragmentPath)")" == \
        "$(realpath -e -- "$fragment")" \
      && "$(unit_property "$unit" CPUAccounting)" == yes \
      && "$(unit_property "$unit" CPUQuotaPerSecUSec)" == "$expected_cpu" \
      && "$(unit_property "$unit" MemoryAccounting)" == yes \
      && "$(unit_property "$unit" MemoryHigh)" == "$expected_high" \
      && "$(unit_property "$unit" MemoryMax)" == "$expected_max" \
      && "$(unit_property "$unit" MemorySwapMax)" == "$expected_swap" \
      && "$(unit_property "$unit" TasksAccounting)" == yes \
      && "$(unit_property "$unit" TasksMax)" == "$expected_tasks" ]] \
    || return 1
  exec_start="$(unit_property "$unit" ExecStart)" || return 1
  [[ "$exec_start" == *"path=$expected_exec"* \
      && "$exec_start" == *"argv[]=$expected_argv"* \
      && -n "$expected_cpu_max" ]]
}

active_unit_cgroup_matches_source_contract() {
  local unit="$1" _fragment _exec _argv _cpu high max swap tasks cpu_max
  local cgroup pid process_cgroup root
  IFS=$'\t' read -r _fragment _exec _argv _cpu high max swap tasks cpu_max \
    < <(unit_contract_values "$unit") || return 1
  cgroup="$(unit_property "$unit" ControlGroup)" || return 1
  pid="$(unit_property "$unit" MainPID)" || return 1
  [[ -n "$cgroup" && "$cgroup" != / && "$pid" =~ ^[1-9][0-9]*$ ]] || return 1
  process_cgroup="$(awk -F: '$1 == "0" {print $3}' "/proc/$pid/cgroup")" || return 1
  [[ "$process_cgroup" == "$cgroup" ]] || return 1
  root="/sys/fs/cgroup$cgroup"
  [[ -d "$root" \
      && "$(<"$root/memory.high")" == "$high" \
      && "$(<"$root/memory.max")" == "$max" \
      && "$(<"$root/memory.swap.max")" == "$swap" \
      && "$(<"$root/pids.max")" == "$tasks" \
      && "$(<"$root/cpu.max")" == "$cpu_max" ]]
}

process_starttime() {
  sed -E 's/^[0-9]+ \([^)]*\) //' "/proc/$1/stat" | awk '{print $20}'
}

capture_process_identity() {
  local pid="$1" exe_path exe_sha exe_dev_inode starttime
  [[ "$pid" =~ ^[1-9][0-9]*$ && -r "/proc/$pid/stat" ]] || return 1
  exe_path="$(readlink -e "/proc/$pid/exe")" || return 1
  exe_sha="$(sha256sum "/proc/$pid/exe" | awk '{print $1}')" || return 1
  exe_dev_inode="$(stat -Lc '%d:%i' "/proc/$pid/exe")" || return 1
  starttime="$(process_starttime "$pid")" || return 1
  jq -cn \
    --argjson pid "$pid" \
    --arg exe_path "$exe_path" \
    --arg exe_sha256 "$exe_sha" \
    --arg exe_dev_inode "$exe_dev_inode" \
    --arg starttime_ticks "$starttime" \
    '{pid:$pid,exe_path:$exe_path,exe_sha256:$exe_sha256,
      exe_dev_inode:$exe_dev_inode,starttime_ticks:$starttime_ticks}'
}

capture_unit_process_identity() {
  local unit="$1" pid
  pid="$(unit_property "$unit" MainPID)"
  capture_process_identity "$pid"
}

tcp_port_owned_by() {
  local pid="$1" port="$2" listeners
  [[ "$pid" =~ ^[1-9][0-9]*$ && "$port" =~ ^[1-9][0-9]*$ ]] || return 1
  listeners="$(
    exec 8>&- 9>&-
    timeout --signal=TERM --kill-after=2s 5 \
      ss -H -ltnp "sport = :$port" 2>/dev/null
  )" || return 1
  [[ "$(grep -c . <<<"$listeners")" == 1 ]] || return 1
  grep -Fq "pid=$pid," <<<"$listeners"
}

game_server_port_owned_by() {
  tcp_port_owned_by "$1" 7005
}

service_ports_match_baseline() {
  local signer_pid ledger_pid
  signer_pid="$(jq -er '.pid' \
    <<<"${process_identity_before[trnm-entitlement-signer.service]}")" || return 1
  ledger_pid="$(jq -er '.pid' \
    <<<"${process_identity_before[cex-trnm-ledger.service]}")" || return 1
  tcp_port_owned_by "$GAME_SERVER_PID" 7005 \
    && tcp_port_owned_by "$signer_pid" 7010 \
    && tcp_port_owned_by "$ledger_pid" 7002
}

service_processes_match_baseline() {
  local unit current
  for unit in "${units[@]}"; do
    current="$(capture_unit_process_identity "$unit")" || return 1
    [[ "$(jq -S -c . <<<"$current")" == \
        "$(jq -S -c . <<<"${process_identity_before[$unit]}")" ]] \
      || return 1
  done
}

postgres_process_matches_baseline() {
  local current
  current="$(postgres_runtime)" || return 1
  jq -e --argjson baseline "$postgres_before" '
    .running == true and .oom_killed == false and
    .restart_count == $baseline.restart_count and
    .container_id == $baseline.container_id and
    .image_id == $baseline.image_id and .pid == $baseline.pid and
    .started_at == $baseline.started_at
  ' >/dev/null <<<"$current"
}

runtime_bindings_match_baseline() {
  service_processes_match_baseline \
    && service_ports_match_baseline \
    && game_server_process_matches_baseline \
    && postgres_process_matches_baseline
}

game_server_process_matches_baseline() {
  local current
  current="$(capture_unit_process_identity trnm-game-server.service)" || return 1
  [[ "$(jq -S -c . <<<"$current")" == \
      "$(jq -S -c . <<<"$GAME_SERVER_PROCESS_BASELINE")" ]] || return 1
  game_server_port_owned_by "$GAME_SERVER_PID"
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

run_cex_docker() (
  exec 8>&- 9>&-
  timeout --signal=TERM --kill-after=5s "$EXTERNAL_COMMAND_TIMEOUT_SECONDS" \
    bash --noprofile --norc -c '
      set -euo pipefail
      exec 8>&- 9>&-
      helper=$1 env_source=$2 expected_env_sha=$3 expected_helper_sha=$4
      shift 4
      [[ -f "$helper" && ! -L "$helper" ]]
      [[ "$(sha256sum "$helper" | awk "{print \$1}")" == "$expected_helper_sha" ]]
      if [[ -n "$env_source" ]]; then
        [[ -f "$env_source" && ! -L "$env_source" ]]
        [[ "$(sha256sum "$env_source" | awk "{print \$1}")" == "$expected_env_sha" ]]
      fi
      # shellcheck source=/dev/null
      source "$helper"
      if [[ -n "$env_source" ]]; then
        cex_load_env "$env_source"
      fi
      export PATH=/usr/sbin:/usr/bin LC_ALL=C
      unset BASH_ENV ENV CDPATH LD_PRELOAD LD_LIBRARY_PATH PYTHONHOME PYTHONPATH \
        GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM CURL_HOME \
        HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy \
        DOCKER_HOST DOCKER_CONTEXT DOCKER_CONFIG DOCKER_TLS_VERIFY DOCKER_CERT_PATH
      cex_docker "$@"
    ' capacity-cex-docker "$CEX_HELPER" "$CEX_ENV_SOURCE" "$CEX_ENV_SHA256" \
      "$CEX_HELPER_SHA256" "$@"
)

run_cex_psql() (
  exec 8>&- 9>&-
  local sql="$1"
  shift
  printf '%s\n' "$sql" | \
    timeout --signal=TERM --kill-after=5s "$EXTERNAL_COMMAND_TIMEOUT_SECONDS" \
      bash --noprofile --norc -c '
        set -euo pipefail
        exec 8>&- 9>&-
        helper=$1 env_source=$2 expected_env_sha=$3 expected_helper_sha=$4
        shift 4
        [[ -f "$helper" && ! -L "$helper" ]]
        [[ "$(sha256sum "$helper" | awk "{print \$1}")" == "$expected_helper_sha" ]]
        if [[ -n "$env_source" ]]; then
          [[ -f "$env_source" && ! -L "$env_source" ]]
          [[ "$(sha256sum "$env_source" | awk "{print \$1}")" == "$expected_env_sha" ]]
        fi
        # shellcheck source=/dev/null
        source "$helper"
        if [[ -n "$env_source" ]]; then
          cex_load_env "$env_source"
        fi
        export PATH=/usr/sbin:/usr/bin LC_ALL=C
        unset BASH_ENV ENV CDPATH LD_PRELOAD LD_LIBRARY_PATH PYTHONHOME PYTHONPATH \
          GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM CURL_HOME \
          HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy \
          DOCKER_HOST DOCKER_CONTEXT DOCKER_CONFIG DOCKER_TLS_VERIFY DOCKER_CERT_PATH
        cex_psql_stdin "$@" -f -
      ' capacity-cex-psql "$CEX_HELPER" "$CEX_ENV_SOURCE" "$CEX_ENV_SHA256" \
        "$CEX_HELPER_SHA256" "$@"
)

bounded_curl() (
  local maximum_seconds="$1"
  shift
  [[ "$maximum_seconds" =~ ^[1-9][0-9]*$ ]] || return 2
  exec 8>&- 9>&-
  timeout --signal=TERM --kill-after=5s "$((maximum_seconds + 5))" \
    curl -q -fsS --connect-timeout "$CURL_CONNECT_TIMEOUT_SECONDS" \
      --max-time "$maximum_seconds" "$@"
)

postgres_runtime() {
  run_cex_docker inspect "$CEX_POSTGRES_CONTAINER_NAME" --format \
    '{"container_id":{{json .Id}},"image_id":{{json .Image}},"pid":{{.State.Pid}},"restart_count":{{.RestartCount}},"oom_killed":{{.State.OOMKilled}},"running":{{.State.Running}},"started_at":{{json .State.StartedAt}}}'
}

database_active_connections() {
  run_cex_psql 'select count(*) from pg_stat_activity' -At
}

wal_runtime() {
  run_cex_psql "select json_build_object(
    'archived_count', archived_count,
    'failed_count', failed_count,
    'last_archived_wal', last_archived_wal,
    'archiver_recovered', last_failed_time is null
      or coalesce(last_archived_time >= last_failed_time, false))
    from pg_stat_archiver" -At
}

online_readiness_matches_capacity() {
  local readiness
  readiness="$(bounded_curl 10 \
    "$ONLINE_URL/v1/online/readiness" 2>/dev/null || true)"
  jq -e --argjson capacity "$CONCURRENCY" '
    .status == "ok" and .clock_mode == "real_time_no_catch_up" and
    .tick_rate_hz == 10 and .fleet_capacity == $capacity and
    .authority_clock_operational == true and
    .database_pool_saturation_healthy == true' >/dev/null 2>&1 <<<"$readiness"
}

ledger_readiness_is_operational() {
  local readiness
  readiness="$(bounded_curl 10 \
    "$LEDGER_URL/v1/trnm/economy/readiness" 2>/dev/null || true)"
  jq -e '.status == "ok" and .postgres_operations_healthy == true and
    .postgres_operations.pool_saturation_healthy == true and
    .postgres_operations.archiver_recovered == true' \
    >/dev/null 2>&1 <<<"$readiness"
}

signer_readiness_is_operational() {
  local readiness
  readiness="$(bounded_curl 10 \
    "$SIGNER_URL/v1/signer/readiness" 2>/dev/null || true)"
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
  bounded_curl "$CURL_REQUEST_TIMEOUT_SECONDS" "$LEDGER_URL$1" \
    -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local label="$1" account player recovery session
  local account_response session_response
  if ! account_response="$(admin_post /v1/accounts "$(jq -cn \
    --arg org '00000000-0000-0000-0000-00000000ce01' --arg label "$label" \
    '{org_id:$org,account_type:("capacity-"+$label),currency_unit:"credit",initial_balance:0}')")"; then
    echo "capacity identity provisioning failed: label=$label stage=account" >&2
    return 1
  fi
  if ! account="$(jq -er '.account_id | select(type == "string" and length > 0)' \
      <<<"$account_response")"; then
    echo "capacity identity provisioning failed: label=$label stage=account-response" >&2
    return 1
  fi
  player="$RUN_ID-$label"
  recovery="recovery-$RUN_ID-$label-012345678901234567890123"
  if ! admin_post /v1/trnm/identity/register "$(jq -cn \
    --arg player "$player" --arg account "$account" --arg recovery "$recovery" \
    '{player_id:$player,account_id:$account,recovery_key:$recovery}')" >/dev/null; then
    echo "capacity identity provisioning failed: label=$label stage=registration" >&2
    return 1
  fi
  if ! session_response="$(bounded_curl "$CURL_REQUEST_TIMEOUT_SECONDS" \
    "$LEDGER_URL/v1/trnm/identity/session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg recovery "$recovery" --arg device "$RUN_ID-$label-device" \
      '{player_id:$player,recovery_key:$recovery,device_id:$device,lifetime_seconds:10800}')")"; then
    echo "capacity identity provisioning failed: label=$label stage=session" >&2
    return 1
  fi
  if ! session="$(jq -er '.session_token | select(type == "string" and length > 0)' \
      <<<"$session_response")"; then
    echo "capacity identity provisioning failed: label=$label stage=session-response" >&2
    return 1
  fi
  printf '%s\t%s\t%s\n' "$player" "$account" "$session"
}

fail_close_running_test_matches() {
  local match_id report remaining candidates_left time_slice command_timeout
  local candidate_index=0
  local -a candidates=()
  (( CLEANUP_DEADLINE > SECONDS )) || return 1
  [[ "$(unit_property trnm-game-server.service ActiveState)" == inactive ]] \
    || return 1
  ! process_pid_is_live "${GAME_SERVER_PID:-}" || return 1
  [[ -n "${GAME_SERVER_SHA256:-}" \
      && "$(sha256sum "$GAME_SERVER_BINARY" | awk '{print $1}')" \
        == "${GAME_SERVER_SHA256:-}" ]] || return 1
  mapfile -t candidates < <(run_cex_psql "
    select distinct m.match_id::text
      from trnm_online_matches m
      join trnm_online_match_members mm using (match_id)
     where m.phase in ('waiting','running','failed_closed')
       and mm.player_id like '$RUN_ID-%'
     order by m.match_id" -At -v ON_ERROR_STOP=1)
  for match_id in "${candidates[@]}"; do
    candidate_index=$((candidate_index + 1))
    [[ "$match_id" =~ ^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$ ]] \
      || return 1
    remaining=$((CLEANUP_DEADLINE - SECONDS))
    candidates_left=$((${#candidates[@]} - candidate_index + 1))
    (( remaining > 3 && candidates_left > 0 )) || return 1
    time_slice=$((remaining / candidates_left))
    (( time_slice > 3 )) || return 1
    command_timeout=$((time_slice - 2))
    (( command_timeout > 120 )) && command_timeout=120
    report="$EVIDENCE/maintenance-fail-close-$match_id.json"
    if ! (
      exec 8>&- 9>&-
      export TRNM_GAME_SERVER_RELEASE_DIR="$RELEASE_DIR"
      export TRNM_MAINTENANCE_FAILURE_REASON='capacity soak interrupted or failed'
      exec timeout --foreground --signal=TERM --kill-after=2s "${command_timeout}s" \
        "$ROOT_DIR/scripts/run-trnm-game-server.sh" \
        --maintenance-fail-close "$match_id"
    ) >"$report" 2>"$EVIDENCE/maintenance-fail-close-$match_id.log"; then
      return 1
    fi
    jq -e --arg match_id "$match_id" '
      .contract_version == "trnm_online_maintenance_fail_close_v1"
      and .status == "completed"
      and .match_id == $match_id
      and .selector == "exact_match_id"
      and .transition_atomic == true
      and .legacy_adoption == false
      and .adoption_contract == null
      and (.previous_phase == "waiting" or .previous_phase == "running"
        or .previous_phase == "failed_closed")
      and .final_phase == "failed_closed"
      and (
        if .waiting_db_only then
          .hot_witness_present_before == false
          and .cold_witness_sealed == false
          and .local_marker_state == null
        else
          .cold_witness_sealed == true
          and .local_marker_state == "sealed"
          and (if .previous_phase == "running" then
            .hot_witness_present_before == true
          else
            (.hot_witness_present_before | type) == "boolean"
          end)
        end
      )' "$report" >/dev/null || return 1
  done
  remaining="$(run_cex_psql "
    select count(*)
      from trnm_online_matches m
     where m.phase in ('waiting','running')
       and exists (
         select 1 from trnm_online_match_members mm
          where mm.match_id=m.match_id and mm.player_id like '$RUN_ID-%'
       )" -At -v ON_ERROR_STOP=1)"
  [[ "$remaining" == 0 ]]
}

worker_pids=()
declare -A cleanup_process_start cleanup_process_exe cleanup_process_cgroup cleanup_process_pgid
MONITOR_PID=""
MUTATION_STARTED=0
ORIGINAL_RELEASE_ENV_SET=0
ORIGINAL_RELEASE_ENV_VALUE=""
ORIGINAL_CAPACITY_ENV_SET=0
ORIGINAL_CAPACITY_ENV_VALUE=""
ORIGINAL_SERVICE_ACTIVE=0
ORIGINAL_SERVICE_STATE=""
ORIGINAL_GAME_SERVER_PROCESS=""
ORIGINAL_RELEASE_SELECTOR=""
WORKLOAD_SUMMARY_READY=0
CLEANUP_DEADLINE=0

process_pid_is_live() {
  local pid="$1" stat_line state
  [[ "$pid" =~ ^[1-9][0-9]*$ && -r "/proc/$pid/stat" ]] || return 1
  IFS= read -r stat_line <"/proc/$pid/stat" || return 1
  stat_line="${stat_line##*) }"
  state="${stat_line%% *}"
  [[ "$state" != Z && "$state" != X ]]
}

process_group_is_live() {
  local pgid="$1" state saw_member=0
  while IFS= read -r state; do
    [[ -n "$state" ]] || continue
    saw_member=1
    state="${state:0:1}"
    [[ "$state" == Z || "$state" == X ]] || return 0
  done < <(ps -o stat= --pgroup "$pgid" 2>/dev/null || true)
  (( saw_member == 0 )) && process_pid_is_live "$pgid" && return 0
  return 1
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
        [[ "$exe" == "$expected_exe" ]] || { sleep 0.02; continue; }
        cgroup="$(awk -F: '$1 == "0" {print $3}' "/proc/$pid/cgroup")" || continue
        [[ "$cgroup" == "$RESOURCE_CGROUP" ]] || continue
        cleanup_process_start["$pid"]="$(awk '{print $20}' <<<"$remainder")"
        cleanup_process_exe["$pid"]="$exe"
        cleanup_process_cgroup["$pid"]="$cgroup"
        cleanup_process_pgid["$pid"]="$pgid"
        return 0
      fi
    fi
    sleep 0.02
  done
  echo "could not bind cleanup identity for process $pid" >&2
  return 1
}

cleanup_process_identity_matches() {
  local pid="$1" stat_line remainder
  [[ -n "${cleanup_process_start[$pid]:-}" \
      && -r "/proc/$pid/stat" && -r "/proc/$pid/cgroup" ]] || return 1
  IFS= read -r stat_line <"/proc/$pid/stat" || return 1
  remainder="${stat_line##*) }"
  [[ "$(awk '{print $20}' <<<"$remainder")" == "${cleanup_process_start[$pid]}" \
      && "$(stat -Lc '%d:%i' "/proc/$pid/exe" 2>/dev/null)" == \
        "${cleanup_process_exe[$pid]}" \
      && "$(awk -F: '$1 == "0" {print $3}' "/proc/$pid/cgroup")" == \
        "${cleanup_process_cgroup[$pid]}" \
      && "$(awk '{print $3}' <<<"$remainder")" == "${cleanup_process_pgid[$pid]}" ]]
}

signal_process_group() {
  local pid="$1" signal_name="$2"
  cleanup_process_identity_matches "$pid" || {
    echo "refusing to signal reused or identity-mismatched process group $pid" >&2
    return 1
  }
  kill -s "$signal_name" -- "-$pid" >/dev/null 2>&1 \
    || kill -s "$signal_name" "$pid" >/dev/null 2>&1 \
    || true
}

wait_for_process_state_bounded() {
  local pid="$1" group_mode="$2" timeout_seconds="$3" deadline
  deadline=$((SECONDS + timeout_seconds))
  while (( SECONDS < deadline )); do
    if [[ "$group_mode" == group ]]; then
      process_group_is_live "$pid" || return 0
    else
      process_pid_is_live "$pid" || return 0
    fi
    sleep 0.1
  done
  if [[ "$group_mode" == group ]]; then
    ! process_group_is_live "$pid"
  else
    ! process_pid_is_live "$pid"
  fi
}

terminate_process_bounded() {
  local pid="$1" group_mode="$2" label="$3"
  [[ -n "$pid" ]] || return 0
  if [[ "$group_mode" == group ]]; then
    process_group_is_live "$pid" || { wait "$pid" >/dev/null 2>&1 || true; return 0; }
    cleanup_process_identity_matches "$pid" || return 1
    signal_process_group "$pid" TERM
  else
    process_pid_is_live "$pid" || { wait "$pid" >/dev/null 2>&1 || true; return 0; }
    cleanup_process_identity_matches "$pid" || {
      echo "refusing to signal reused or identity-mismatched process $pid" >&2
      return 1
    }
    kill -TERM "$pid" >/dev/null 2>&1 || true
  fi
  if ! wait_for_process_state_bounded "$pid" "$group_mode" \
      "$WORKER_TERM_GRACE_SECONDS"; then
    if [[ "$group_mode" == group ]]; then
      signal_process_group "$pid" KILL
    else
      kill -KILL "$pid" >/dev/null 2>&1 || true
    fi
    wait_for_process_state_bounded "$pid" "$group_mode" \
      "$WORKER_KILL_GRACE_SECONDS" || {
      echo "$label did not terminate after SIGKILL" >&2
      return 1
    }
  fi
  wait "$pid" >/dev/null 2>&1 || true
}

all_worker_groups_stopped() {
  local pid
  for pid in "${worker_pids[@]}"; do
    process_group_is_live "$pid" && return 1
  done
}

terminate_workers_bounded() {
  local pid deadline failed=0
  (( ${#worker_pids[@]} > 0 )) || return 0
  for pid in "${worker_pids[@]}"; do
    if process_group_is_live "$pid" && ! cleanup_process_identity_matches "$pid"; then
      echo "refusing to clean reused or identity-mismatched worker group $pid" >&2
      return 1
    fi
  done
  for pid in "${worker_pids[@]}"; do
    process_group_is_live "$pid" && signal_process_group "$pid" TERM \
      || true
  done
  deadline=$((SECONDS + WORKER_TERM_GRACE_SECONDS))
  while (( SECONDS < deadline )); do
    all_worker_groups_stopped && break
    sleep 0.1
  done
  for pid in "${worker_pids[@]}"; do
    process_group_is_live "$pid" && signal_process_group "$pid" KILL
  done
  deadline=$((SECONDS + WORKER_KILL_GRACE_SECONDS))
  while (( SECONDS < deadline )); do
    all_worker_groups_stopped && break
    sleep 0.1
  done
  for pid in "${worker_pids[@]}"; do
    if process_group_is_live "$pid"; then
      echo "capacity worker group $pid survived SIGKILL" >&2
      failed=1
    else
      wait "$pid" >/dev/null 2>&1 || true
    fi
  done
  worker_pids=()
  (( failed == 0 ))
}

systemctl_with_timeout() (
  local timeout_seconds="$1"
  shift
  exec 8>&- 9>&-
  timeout --signal=TERM --kill-after=5s "$timeout_seconds" \
    systemctl --user "$@"
)

cleanup_systemctl() {
  local remaining=$((CLEANUP_DEADLINE - SECONDS))
  (( remaining > 0 )) || return 124
  systemctl_with_timeout "$remaining" "$@"
}

capture_manager_variable() {
  local name="$1" set_variable="$2" value_variable="$3" line environment
  environment="$(systemctl_with_timeout 20 show-environment)" || return
  line="$(awk -F= -v name="$name" '$1 == name {print; exit}' <<<"$environment")"
  if [[ -n "$line" ]]; then
    printf -v "$set_variable" '%s' 1
    printf -v "$value_variable" '%s' "${line#*=}"
  else
    printf -v "$set_variable" '%s' 0
    printf -v "$value_variable" '%s' ""
  fi
}

restore_manager_variable() {
  local name="$1" was_set="$2" value="$3"
  if [[ "$was_set" == 1 ]]; then
    cleanup_systemctl set-environment "$name=$value"
  else
    cleanup_systemctl unset-environment "$name"
  fi
}

manager_variable_matches() {
  local name="$1" was_set="$2" value="$3" current environment
  environment="$(cleanup_systemctl show-environment)" || return
  current="$(awk -F= -v name="$name" '$1 == name {print; exit}' <<<"$environment")"
  if [[ "$was_set" == 1 ]]; then
    [[ "$current" == "$name=$value" ]]
  else
    [[ -z "$current" ]]
  fi
}

cleanup() {
  local status=$? cleanup_failed=false restored_process restored_selector_request
  local restored_readiness restored_pid restored=false final_tmp
  trap - EXIT
  trap '' INT TERM HUP
  CLEANUP_DEADLINE=$((SECONDS + CLEANUP_TOTAL_TIMEOUT_SECONDS))
  terminate_process_bounded "$MONITOR_PID" pid operational-monitor \
    || cleanup_failed=true
  MONITOR_PID=""
  terminate_workers_bounded || cleanup_failed=true
  if [[ "$MUTATION_STARTED" == 1 ]]; then
    cleanup_systemctl stop trnm-game-server.service >/dev/null 2>&1 \
      || cleanup_failed=true
    if [[ "$(unit_property trnm-game-server.service ActiveState 2>/dev/null)" != inactive ]] \
        || process_pid_is_live "${GAME_SERVER_PID:-}"; then
      cleanup_failed=true
    elif ! fail_close_running_test_matches >/dev/null 2>&1; then
      cleanup_failed=true
    fi
    restore_manager_variable TRNM_GAME_SERVER_RELEASE_DIR \
      "$ORIGINAL_RELEASE_ENV_SET" "$ORIGINAL_RELEASE_ENV_VALUE" \
      >/dev/null 2>&1 || cleanup_failed=true
    restore_manager_variable TRNM_FLEET_CAPACITY \
      "$ORIGINAL_CAPACITY_ENV_SET" "$ORIGINAL_CAPACITY_ENV_VALUE" \
      >/dev/null 2>&1 || cleanup_failed=true
    if [[ "$ORIGINAL_SERVICE_ACTIVE" == 1 ]]; then
      cleanup_systemctl restart trnm-game-server.service >/dev/null 2>&1 \
        || cleanup_failed=true
      while (( SECONDS < CLEANUP_DEADLINE )); do
        restored_readiness="$(
          bounded_curl 5 "$ONLINE_URL/v1/online/readiness" 2>/dev/null || true
        )"
        if [[ "$(unit_property trnm-game-server.service ActiveState)" == active ]] && \
            jq -e '.status == "ok"' >/dev/null 2>&1 <<<"$restored_readiness"; then
          break
        fi
        sleep 1
      done
      restored_process="$(capture_unit_process_identity trnm-game-server.service 2>/dev/null || true)"
      if [[ -z "$restored_process" || \
          "$(jq -r '.exe_path + ":" + .exe_sha256 + ":" + .exe_dev_inode' \
            <<<"$restored_process" 2>/dev/null)" != \
          "$(jq -r '.exe_path + ":" + .exe_sha256 + ":" + .exe_dev_inode' \
            <<<"$ORIGINAL_GAME_SERVER_PROCESS" 2>/dev/null)" ]]; then
        cleanup_failed=true
      fi
      restored_pid="$(jq -r '.pid // 0' <<<"${restored_process:-null}" 2>/dev/null || echo 0)"
      game_server_port_owned_by "$restored_pid" || cleanup_failed=true
      jq -e '.status == "ok"' >/dev/null 2>&1 <<<"${restored_readiness:-}" \
        || cleanup_failed=true
    else
      cleanup_systemctl stop trnm-game-server.service >/dev/null 2>&1 \
        || cleanup_failed=true
      [[ "$(unit_property trnm-game-server.service ActiveState)" == inactive ]] \
        || cleanup_failed=true
    fi
    manager_variable_matches TRNM_GAME_SERVER_RELEASE_DIR \
      "$ORIGINAL_RELEASE_ENV_SET" "$ORIGINAL_RELEASE_ENV_VALUE" \
      || cleanup_failed=true
    manager_variable_matches TRNM_FLEET_CAPACITY \
      "$ORIGINAL_CAPACITY_ENV_SET" "$ORIGINAL_CAPACITY_ENV_VALUE" \
      || cleanup_failed=true
    restored_selector_request="$ROOT_DIR/run/releases/trnm-game-server/current"
    if [[ "$ORIGINAL_RELEASE_ENV_SET" == 1 ]]; then
      restored_selector_request="$ORIGINAL_RELEASE_ENV_VALUE"
    fi
    [[ "$(realpath -e -- "$restored_selector_request" 2>/dev/null || true)" == \
        "$ORIGINAL_RELEASE_SELECTOR" ]] || cleanup_failed=true
  fi
  if [[ "$cleanup_failed" == true && "$status" -eq 0 ]]; then
    status=1
  fi
  [[ "$cleanup_failed" == false ]] && restored=true
  if [[ "$WORKLOAD_SUMMARY_READY" == 1 \
      && -f "$EVIDENCE/workload-summary.json" ]]; then
    final_tmp="$EVIDENCE/.summary.json.tmp.$$"
    if jq --argjson restored "$restored" --argjson exit_status "$status" '
        .cleanup_restored = $restored
        | .pre_cleanup_exit_status = $exit_status
        | .passed = (.workload_passed == true and $restored and $exit_status == 0)
        | .decision_final = true
      ' "$EVIDENCE/workload-summary.json" >"$final_tmp" \
        && chmod 0600 "$final_tmp" \
        && mv -f -- "$final_tmp" "$EVIDENCE/summary.json"; then
      cat "$EVIDENCE/summary.json"
    else
      rm -f -- "$final_tmp"
      status=1
    fi
    jq -e '.passed == true and .cleanup_restored == true' \
      "$EVIDENCE/summary.json" >/dev/null 2>&1 || status=1
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
trap 'exit 129' HUP

installed_units_match_source() {
  local pair source_unit installed_unit
  for pair in \
    "$ROOT_DIR/deploy/systemd/trnm-game-server.service:$HOME/.config/systemd/user/trnm-game-server.service" \
    "$ROOT_DIR/deploy/systemd/trnm-entitlement-signer.service:$HOME/.config/systemd/user/trnm-entitlement-signer.service" \
    "$CEX_ROOT/deploy/systemd/cex-trnm-ledger.service:$HOME/.config/systemd/user/cex-trnm-ledger.service" \
    "$CEX_ROOT/deploy/systemd/cex-trnm-consumer.service:$HOME/.config/systemd/user/cex-trnm-consumer.service"; do
    IFS=: read -r source_unit installed_unit <<<"$pair"
    cmp -s "$source_unit" "$installed_unit" || return 1
  done
  for source_unit in trnm-game-server.service trnm-entitlement-signer.service \
    cex-trnm-ledger.service cex-trnm-consumer.service; do
    effective_unit_matches_source_contract "$source_unit" || return 1
  done
}

require_host_memory_headroom
installed_units_match_source || {
  echo "one or more installed units differ from canonical source" >&2
  exit 2
}

capture_manager_variable TRNM_GAME_SERVER_RELEASE_DIR \
  ORIGINAL_RELEASE_ENV_SET ORIGINAL_RELEASE_ENV_VALUE
capture_manager_variable TRNM_FLEET_CAPACITY \
  ORIGINAL_CAPACITY_ENV_SET ORIGINAL_CAPACITY_ENV_VALUE
original_selector_request="$ROOT_DIR/run/releases/trnm-game-server/current"
if [[ "$ORIGINAL_RELEASE_ENV_SET" == 1 ]]; then
  original_selector_request="$ORIGINAL_RELEASE_ENV_VALUE"
fi
ORIGINAL_RELEASE_SELECTOR="$(realpath -e -- "$original_selector_request")" || {
  echo "original game-server release selector is unavailable" >&2
  exit 2
}
ORIGINAL_SERVICE_STATE="$(unit_property trnm-game-server.service ActiveState)"
case "$ORIGINAL_SERVICE_STATE" in
active)
  ORIGINAL_SERVICE_ACTIVE=1
  ORIGINAL_GAME_SERVER_PROCESS="$(
    capture_unit_process_identity trnm-game-server.service
  )" || {
    echo "could not capture the original game-server process identity" >&2
    exit 2
  }
  original_readiness="$(
    bounded_curl 5 "$ONLINE_URL/v1/online/readiness" 2>/dev/null || true
  )"
  jq -e '
    .status == "ok" and .active_matches == 0 and .active_match_actors == 0
  ' >/dev/null <<<"$original_readiness" || {
    echo "refusing to restart an authority with active matches or actors" >&2
    exit 2
  }
  ;;
inactive)
  ;;
*)
  echo "game-server must be stably active or inactive before capacity evidence" >&2
  exit 2
  ;;
esac
worktrees_are_clean || {
  echo "worktrees changed during capacity preflight" >&2
  exit 2
}

MUTATION_STARTED=1
systemctl_with_timeout 30 set-environment \
  "TRNM_GAME_SERVER_RELEASE_DIR=$RELEASE_DIR" \
  "TRNM_FLEET_CAPACITY=$CONCURRENCY"
systemctl_with_timeout 150 restart trnm-game-server.service
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

GAME_SERVER_PROCESS_BASELINE="$(
  capture_unit_process_identity trnm-game-server.service
)" || {
  echo "could not capture the evidence game-server process identity" >&2
  exit 2
}
GAME_SERVER_PID="$(jq -er '.pid' <<<"$GAME_SERVER_PROCESS_BASELINE")"
expected_game_server_path="$(realpath -e "$GAME_SERVER_BINARY")"
expected_game_server_dev_inode="$(stat -c '%d:%i' "$GAME_SERVER_BINARY")"
if ! jq -e \
    --arg path "$expected_game_server_path" \
    --arg sha "$(jq -r '.binaries.game_server.sha256' <<<"$release_verification")" \
    --arg dev_inode "$expected_game_server_dev_inode" '
      .exe_path == $path and .exe_sha256 == $sha and .exe_dev_inode == $dev_inode
    ' >/dev/null <<<"$GAME_SERVER_PROCESS_BASELINE" || \
    ! game_server_port_owned_by "$GAME_SERVER_PID"; then
  echo "running game-server process or port 7005 is not bound to the verified release" >&2
  exit 2
fi

units=(trnm-game-server.service trnm-entitlement-signer.service \
  cex-trnm-ledger.service cex-trnm-consumer.service)
declare -A restarts_before cgroup_oom_before process_identity_before unit_cgroup_before
for unit in "${units[@]}"; do
  if ! effective_unit_matches_source_contract "$unit" \
      || ! active_unit_cgroup_matches_source_contract "$unit"; then
    echo "effective systemd/cgroup contract does not match source for $unit" >&2
    exit 2
  fi
  unit_cgroup_before["$unit"]="$(unit_property "$unit" ControlGroup)"
  restarts_before["$unit"]="$(unit_property "$unit" NRestarts)"
  cgroup_oom_before["$unit"]="$(unit_memory_event "$unit" oom_kill)"
  process_identity_before["$unit"]="$(capture_unit_process_identity "$unit")" || {
    echo "could not bind capacity evidence to $unit process identity" >&2
    exit 2
  }
done
resource_oom_before="$(resource_memory_event oom_kill)"
host_oom_before="$(host_oom_kills)"
postgres_before="$(postgres_runtime)"
wal_before="$(wal_runtime)"

TRNM_GIT_HEAD="$(git -C "$ROOT_DIR" rev-parse HEAD)"
CEX_GIT_HEAD="$(git -C "$CEX_ROOT" rev-parse HEAD)"
E2E_SHA256="$(sha256sum "$E2E_BINARY" | awk '{print $1}')"
GAME_SERVER_SHA256="$(sha256sum "$GAME_SERVER_BINARY" | awk '{print $1}')"
SIGNER_SHA256="$(jq -r '.exe_sha256' \
  <<<"${process_identity_before[trnm-entitlement-signer.service]}")"
unit_processes="$(jq -n \
  --argjson game_server "${process_identity_before[trnm-game-server.service]}" \
  --argjson signer "${process_identity_before[trnm-entitlement-signer.service]}" \
  --argjson ledger "${process_identity_before[cex-trnm-ledger.service]}" \
  --argjson consumer "${process_identity_before[cex-trnm-consumer.service]}" \
  '{game_server:$game_server,signer:$signer,ledger:$ledger,consumer:$consumer}')"
started_epoch="$(date +%s)"
started_monotonic_seconds="$(monotonic_seconds)"
started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
deadline_monotonic_seconds=$((started_monotonic_seconds + DURATION_SECONDS))

cex_sources_match_baseline() {
  [[ -f "$CEX_HELPER" && ! -L "$CEX_HELPER" \
      && "$(sha256sum "$CEX_HELPER" | awk '{print $1}')" == \
        "$CEX_HELPER_SHA256" ]] || return 1
  if [[ -n "$CEX_ENV_SOURCE" ]]; then
    [[ -f "$CEX_ENV_SOURCE" && ! -L "$CEX_ENV_SOURCE" \
        && "$(sha256sum "$CEX_ENV_SOURCE" | awk '{print $1}')" == \
          "$CEX_ENV_SHA256" ]] || return 1
    if [[ "$CEX_ENV_SOURCE" == "$CEX_ROOT/.env.example" ]]; then
      [[ ! -e "$CEX_ROOT/.env" && ! -L "$CEX_ROOT/.env" ]] || return 1
    fi
  else
    [[ ! -e "$CEX_ROOT/.env" && ! -L "$CEX_ROOT/.env" \
        && ! -e "$CEX_ROOT/.env.example" && ! -L "$CEX_ROOT/.env.example" ]] \
      || return 1
  fi
  [[ "$(
      printf '%s\0%s\0%s\0%s\0' \
        "$CEX_POSTGRES_CONTAINER_NAME" "$CEX_POSTGRES_USER" "$CEX_POSTGRES_DB" \
        "$(cex_effective_database_url)" | sha256sum | awk '{print $1}'
    )" == "$CEX_RUNTIME_INPUT_SHA256" ]]
}

static_inputs_match_baseline() {
  local verification="$1"
  worktrees_are_clean \
    && [[ "$(git -C "$ROOT_DIR" rev-parse HEAD)" == "$TRNM_GIT_HEAD" ]] \
    && [[ "$(git -C "$CEX_ROOT" rev-parse HEAD)" == "$CEX_GIT_HEAD" ]] \
    && [[ "$(realpath -e -- "$CURRENT_RELEASE_SELECTOR")" == "$RELEASE_DIR" ]] \
    && [[ "$(sha256sum "$RELEASE_MANIFEST" | awk '{print $1}')" == \
      "$RELEASE_MANIFEST_SHA256" ]] \
    && [[ "$(sha256sum "$GAME_SERVER_BINARY" | awk '{print $1}')" == \
      "$GAME_SERVER_SHA256" ]] \
    && [[ "$(sha256sum "$E2E_BINARY" | awk '{print $1}')" == \
      "$E2E_SHA256" ]] \
    && [[ "$(jq -S -c . <<<"$verification" | sha256sum | awk '{print $1}')" == \
      "$RELEASE_VERIFICATION_SHA256" ]] \
    && cex_sources_match_baseline \
    && installed_units_match_source
}

jq -n \
  --arg contract_version trnm_online_capacity_provenance_v1 \
  --arg run_id "$RUN_ID" --arg started_at "$started_at" \
  --arg trnm_git_head "$TRNM_GIT_HEAD" --arg cex_git_head "$CEX_GIT_HEAD" \
  --arg release_dir "$RELEASE_DIR" \
  --arg release_contract_version "$(jq -r '.release_contract_version' <<<"$release_verification")" \
  --arg release_manifest_sha256 "$RELEASE_MANIFEST_SHA256" \
  --arg release_verification_sha256 "$RELEASE_VERIFICATION_SHA256" \
  --arg cex_helper_sha256 "$CEX_HELPER_SHA256" \
  --arg cex_environment_source "$CEX_ENV_SOURCE" \
  --arg cex_environment_sha256 "$CEX_ENV_SHA256" \
  --arg cex_runtime_input_sha256 "$CEX_RUNTIME_INPUT_SHA256" \
  --arg shared_deployment_lock "$deployment_lock" \
  --arg e2e_sha256 "$E2E_SHA256" --arg game_server_sha256 "$GAME_SERVER_SHA256" \
  --arg signer_sha256 "$SIGNER_SHA256" --arg resource_cgroup "$RESOURCE_CGROUP" \
  --argjson postgres "$postgres_before" \
  --argjson unit_processes "$unit_processes" \
  '{contract_version:$contract_version,run_id:$run_id,started_at:$started_at,
    trnm_git_head:$trnm_git_head,cex_git_head:$cex_git_head,
    release_dir:$release_dir,release_contract_version:$release_contract_version,
    release_manifest_sha256:$release_manifest_sha256,
    release_verification_sha256:$release_verification_sha256,
    binaries:{online_e2e_sha256:$e2e_sha256,game_server_sha256:$game_server_sha256,
      signer_sha256:$signer_sha256},resource_cgroup:$resource_cgroup,
    cex_inputs:{helper_sha256:$cex_helper_sha256,
      environment_source:$cex_environment_source,
      environment_sha256:$cex_environment_sha256,
      runtime_input_sha256:$cex_runtime_input_sha256},
    shared_deployment_lock:$shared_deployment_lock,
    unit_processes:$unit_processes,postgres:$postgres,
    worktrees_clean:true,installed_units_match_source:true,
    formal_evidence:true,public_launch_credit:false,local_only:true}' \
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
  if ! jq -e --argjson baseline "$postgres_before" '
      .running == true and .oom_killed == false and
      .restart_count == $baseline.restart_count and
      .container_id == $baseline.container_id and
      .image_id == $baseline.image_id and .pid == $baseline.pid and
      .started_at == $baseline.started_at' \
      >/dev/null 2>&1 <<<"$postgres"; then
    healthy=false
    reason+="postgres_runtime;"
  fi
  for unit in "${units[@]}"; do
    if ! effective_unit_matches_source_contract "$unit" \
        || ! active_unit_cgroup_matches_source_contract "$unit" \
        || [[ "$(unit_property "$unit" ActiveState)" != active || \
        "$(unit_property "$unit" ControlGroup)" != "${unit_cgroup_before[$unit]}" || \
        "$(unit_property "$unit" NRestarts)" != "${restarts_before[$unit]}" || \
        "$(unit_memory_event "$unit" oom_kill)" != "${cgroup_oom_before[$unit]}" || \
        "$(jq -S -c . <<<"$(capture_unit_process_identity "$unit" 2>/dev/null || printf null)")" != \
        "$(jq -S -c . <<<"${process_identity_before[$unit]}")" ]]; then
      healthy=false
      reason+="$unit;"
    fi
  done
  game_server_process_matches_baseline \
    || { healthy=false; reason+="game_process_or_port_binding;"; }
  service_ports_match_baseline \
    || { healthy=false; reason+="service_port_binding;"; }
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
while (( $(monotonic_seconds) < deadline_monotonic_seconds )); do
  if ! operational_check; then
    failures=$((failures + 1))
    break
  fi
  wave=$((wave + 1))
  worker_pids=()
  reports=()
  host_players=()
  host_accounts=()
  host_sessions=()
  guest_players=()
  guest_accounts=()
  guest_sessions=()
  identity_provisioning_failed=0
  for worker in $(seq 1 "$CONCURRENCY"); do
    worker_index=$((worker - 1))
    if ! IFS=$'\t' read -r host_player host_account host_session \
        < <(create_identity "w${wave}-${worker}-host"); then
      identity_provisioning_failed=1
      break
    fi
    if ! IFS=$'\t' read -r guest_player guest_account guest_session \
        < <(create_identity "w${wave}-${worker}-guest"); then
      identity_provisioning_failed=1
      break
    fi
    host_players[$worker_index]="$host_player"
    host_accounts[$worker_index]="$host_account"
    host_sessions[$worker_index]="$host_session"
    guest_players[$worker_index]="$guest_player"
    guest_accounts[$worker_index]="$guest_account"
    guest_sessions[$worker_index]="$guest_session"
  done
  if (( identity_provisioning_failed != 0 )); then
    printf 'capacity_wave=%s identity_provisioning=failed workers_started=0\n' \
      "$wave" >&2
    failures=$((failures + 1))
    break
  fi
  for worker in $(seq 1 "$CONCURRENCY"); do
    worker_index=$((worker - 1))
    host_player="${host_players[$worker_index]}"
    host_account="${host_accounts[$worker_index]}"
    host_session="${host_sessions[$worker_index]}"
    guest_player="${guest_players[$worker_index]}"
    guest_account="${guest_accounts[$worker_index]}"
    guest_session="${guest_sessions[$worker_index]}"
    report="$EVIDENCE/wave-${wave}-worker-${worker}.json"
    reports+=("$report")
    (
      exec 8>&- 9>&-
      exec setsid --wait timeout --signal=TERM --kill-after=10s \
        "$WORKER_TIMEOUT_SECONDS" env -i \
        PATH="$TRUSTED_FORMAL_PATH" LC_ALL=C \
        NO_PROXY="127.0.0.1,localhost" no_proxy="127.0.0.1,localhost" \
        TRNM_GAME_SERVER_URL="$ONLINE_URL" \
        TRNM_ONLINE_HOST_PLAYER_ID="$host_player" \
        TRNM_ONLINE_HOST_ACCOUNT_ID="$host_account" \
        TRNM_ONLINE_HOST_SESSION="$host_session" \
        TRNM_ONLINE_GUEST_PLAYER_ID="$guest_player" \
        TRNM_ONLINE_GUEST_ACCOUNT_ID="$guest_account" \
        TRNM_ONLINE_GUEST_SESSION="$guest_session" \
        TRNM_ONLINE_E2E_RESTART_SERVER=0 \
        TRNM_ONLINE_E2E_PHASE_TIMEOUT_SECONDS=900 \
        TRNM_ONLINE_E2E_COMPLETION_TIMEOUT_SECONDS=1200 \
        "$E2E_BINARY"
    ) >"$report.tmp" 2>"$report.stderr" &
    worker_pids+=("$!")
    # setsid establishes the group and then execs the stable --wait wrapper;
    # bind cleanup to the long-lived timeout process that actually owns $!.
    register_cleanup_process "$!" group "$(command -v timeout)"
  done

  monitor_failure="$EVIDENCE/wave-${wave}.monitor-failure"
  (
    exec 8>&- 9>&-
    while :; do
      workers_alive=false
      for pid in "${worker_pids[@]}"; do
        if process_group_is_live "$pid"; then
          workers_alive=true
          break
        fi
      done
      [[ "$workers_alive" == true ]] || exit 0
      sleep "$MONITOR_INTERVAL_SECONDS"
      if ! operational_check; then
        printf 'operational monitor failed at %s\n' "$(date -Is)" >"$monitor_failure"
        for pid in "${worker_pids[@]}"; do
          signal_process_group "$pid" TERM
        done
        exit 1
      fi
    done
  ) &
  MONITOR_PID=$!
  register_cleanup_process "$MONITOR_PID" pid "/proc/$$/exe"

  wave_failed=0
  for index in "${!worker_pids[@]}"; do
    worker_pid="${worker_pids[$index]}"
    if wait "$worker_pid" \
        && ! process_group_is_live "$worker_pid" \
        && jq -e '
          def finite_number: type == "number" and isfinite;
          def finite_nonnegative: finite_number and . >= 0;
          type == "object" and
          .status == "passed" and
          .settlement_state == "settled" and
          (.match_id | type == "string" and
            test("^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")) and
          (.command_ack_ms | type == "array" and length > 0 and
            all(.[]; finite_nonnegative)) and
          (.websocket_authoritative_effect_samples_ms |
            type == "array" and length > 0 and
            all(.[]; finite_nonnegative)) and
          (.websocket_authoritative_effect_p95_ms | finite_nonnegative) and
          (.websocket_authoritative_effect_max_ms | finite_nonnegative) and
          .websocket_full_delta_verified == true and
          (.match_tick_drift | finite_number)
        ' >/dev/null "${reports[$index]}.tmp"; then
      mv "${reports[$index]}.tmp" "${reports[$index]}"
    else
      terminate_process_bounded "$worker_pid" group capacity-worker \
        || wave_failed=$((wave_failed + 1))
      wave_failed=$((wave_failed + 1))
    fi
  done
  if wait_for_process_state_bounded "$MONITOR_PID" pid \
      "$((MONITOR_INTERVAL_SECONDS + EXTERNAL_COMMAND_TIMEOUT_SECONDS + 5))"; then
    wait "$MONITOR_PID" || wave_failed=$((wave_failed + 1))
  else
    terminate_process_bounded "$MONITOR_PID" pid operational-monitor \
      || wave_failed=$((wave_failed + 1))
    wave_failed=$((wave_failed + 1))
  fi
  MONITOR_PID=""
  worker_pids=()
  operational_check || wave_failed=$((wave_failed + 1))
  failures=$((failures + wave_failed))
  printf 'capacity_wave=%s concurrency=%s failures=%s elapsed_seconds=%s\n' \
    "$wave" "$CONCURRENCY" "$failures" \
    "$(( $(monotonic_seconds) - started_monotonic_seconds ))" >&2
  if (( failures != 0 )); then
    break
  fi
done

finished_monotonic_seconds="$(monotonic_seconds)"
finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
actual_duration_seconds=$((finished_monotonic_seconds - started_monotonic_seconds))
operational_check || failures=$((failures + 1))
worktrees_are_clean || {
  echo "Trillionnium or CEX worktree changed during formal capacity evidence" >&2
  exit 1
}
final_release_verification="$(
  run_release_verification "$RELEASE_DIR"
)" || {
  echo "immutable release failed end-of-run verification" >&2
  exit 1
}
[[ "$(sha256sum "$RELEASE_MANIFEST" | awk '{print $1}')" == \
    "$RELEASE_MANIFEST_SHA256" ]] || {
  echo "release manifest changed during formal capacity evidence" >&2
  exit 1
}
jq -e \
  --arg release_dir "$RELEASE_DIR" \
  --arg manifest_sha "$RELEASE_MANIFEST_SHA256" \
  --arg game_sha "$GAME_SERVER_SHA256" \
  --arg e2e_sha "$E2E_SHA256" '
    .verified == true and
    .release_contract_version == "trnm_game_server_release_v2" and
    .fault_harness_capable == true and
    .isolated_target == true and
    .trusted_target_cache_used == false and
    .release_dir == $release_dir and
    .release_manifest_sha256 == $manifest_sha and
    .binaries.game_server.sha256 == $game_sha and
    .binaries.online_e2e.sha256 == $e2e_sha
  ' >/dev/null <<<"$final_release_verification" || {
  echo "verified release identity changed during formal capacity evidence" >&2
  exit 1
}
static_inputs_match_baseline "$final_release_verification" || {
  echo "source, unit, CEX, or release input changed during formal capacity evidence" >&2
  exit 1
}
runtime_bindings_match_baseline || {
  echo "service process, port, or PostgreSQL binding changed at capacity evidence end" >&2
  exit 1
}

declare -A restarts_after cgroup_oom_after
for unit in "${units[@]}"; do
  restarts_after["$unit"]="$(unit_property "$unit" NRestarts)"
  cgroup_oom_after["$unit"]="$(unit_memory_event "$unit" oom_kill)"
done
resource_oom_after="$(resource_memory_event oom_kill)"
host_oom_after="$(host_oom_kills)"
postgres_after="$(postgres_runtime)"
wal_after="$(wal_runtime)"
active_run_matches_final="$(run_cex_psql "
  select count(*) from trnm_online_matches m
  where m.phase='running' and exists (
    select 1 from trnm_online_match_members mm
    where mm.match_id=m.match_id and mm.player_id like '$RUN_ID-%')" -At)"

(
  exec 8>&- 9>&-
  exec timeout --signal=TERM --kill-after=5s "$EXTERNAL_COMMAND_TIMEOUT_SECONDS" \
    journalctl --user --no-pager -o cat --since "@$started_epoch" \
      -u trnm-game-server.service -u trnm-entitlement-signer.service \
      -u cex-trnm-ledger.service -u cex-trnm-consumer.service
) >"$EVIDENCE/service-journal.log"
postgres_log_capture_ok=true
if ! run_cex_docker logs --since "$started_epoch" "$CEX_POSTGRES_CONTAINER_NAME" \
    >"$EVIDENCE/postgres.log" 2>&1; then
  postgres_log_capture_ok=false
  echo "PostgreSQL log capture failed; formal evidence will fail closed" >&2
fi
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

terminal_release_verification="$(
  run_release_verification "$RELEASE_DIR"
)" || {
  echo "immutable release failed terminal verification" >&2
  exit 1
}
static_inputs_match_baseline "$terminal_release_verification" || {
  echo "terminal source or release input binding failed" >&2
  exit 1
}
runtime_bindings_match_baseline || {
  echo "terminal service process, port, or PostgreSQL binding failed" >&2
  exit 1
}
validate_resource_scope

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
  --argjson postgres_log_capture_ok "$postgres_log_capture_ok" \
  --arg resource_cgroup "$RESOURCE_CGROUP" \
  --arg release_dir "$RELEASE_DIR" \
  --arg release_manifest_sha256 "$RELEASE_MANIFEST_SHA256" \
  --argjson resource_memory_max_bytes "$(<"$RESOURCE_CGROUP_ROOT/memory.max")" \
  --argjson minimum_host_available_memory_mib "$MIN_AVAILABLE_MIB" '
  def finite_number: type == "number" and isfinite;
  def finite_nonnegative: finite_number and . >= 0;
  def valid_report:
    type == "object" and
    .status == "passed" and
    .settlement_state == "settled" and
    (.match_id | type == "string" and
      test("^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")) and
    (.command_ack_ms | type == "array" and length > 0 and
      all(.[]; finite_nonnegative)) and
    (.websocket_authoritative_effect_samples_ms |
      type == "array" and length > 0 and
      all(.[]; finite_nonnegative)) and
    (.websocket_authoritative_effect_p95_ms | finite_nonnegative) and
    (.websocket_authoritative_effect_max_ms | finite_nonnegative) and
    .websocket_full_delta_verified == true and
    (.match_tick_drift | finite_number);
  (all(.[]; valid_report)) as $report_schema_valid |
  ([.[].command_ack_ms[]] | sort) as $acks |
  ([.[].websocket_authoritative_effect_samples_ms[]] | sort) as $effects |
  ([.[].match_tick_drift | fabs] | max) as $max_abs_drift |
  (($acks | length) * 95 / 100 | ceil | . - 1) as $ack_p95_index |
  (($effects | length) * 95 / 100 | ceil | . - 1) as $effect_p95_index |
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
      signer_sha256:$signer_sha256,release_dir:$release_dir,
      release_manifest_sha256:$release_manifest_sha256},
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
    postgres_log_capture_succeeded:$postgres_log_capture_ok,
    bounded_resource_scope:true,resource_scope_validated:true,
    static_inputs_bound_before_and_after:true,
    service_processes_and_ports_bound_before_and_after:true,
    immutable_release_verified_before_and_after:true,
    resource_cgroup:$resource_cgroup,
    formal_evidence:true,public_launch_credit:false,local_only:true,
    resource_memory_max_bytes:$resource_memory_max_bytes,
    minimum_host_available_memory_mib:$minimum_host_available_memory_mib,
    all_settled:all(.settlement_state == "settled"),
    command_ack_samples:($acks|length),command_ack_p95_ms:$acks[$ack_p95_index],
    command_ack_max_ms:($acks|max),max_absolute_match_tick_drift:$max_abs_drift,
    authoritative_effect_samples:($effects|length),
    authoritative_effect_p95_ms:$effects[$effect_p95_index],
    authoritative_effect_max_ms:($effects|max),
    report_schema_valid:$report_schema_valid,
    cleanup_restored:false,decision_final:false,passed:false,
    workload_passed:(
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
      $postgres_log_capture_ok == true and $report_schema_valid == true and
      length >= $concurrency and ([.[].match_id]|unique|length) == length and
      all(.settlement_state == "settled") and
      ($acks|length) > 0 and ($effects|length) > 0 and
      $acks[$ack_p95_index] < 250 and
      $effects[$effect_p95_index] <= 300 and $max_abs_drift < 2.0
    )
  }' "${report_files[@]}" >"$EVIDENCE/workload-summary.json"

WORKLOAD_SUMMARY_READY=1
exit 0
