#!/usr/bin/bash
set +xv
set -euo pipefail

while IFS= read -r inherited_function_name; do
  builtin unset -f "$inherited_function_name"
done < <(builtin compgen -A function)
unset inherited_function_name

readonly TRUSTED_FORMAL_PATH="/usr/sbin:/usr/bin"
readonly FORMAL_MIN_DURATION_SECONDS=7200
readonly FORMAL_MAX_DURATION_SECONDS=86400
readonly PROFILE_COMPLETION_GRACE_SECONDS=2400
readonly PROFILE_CLEANUP_KILL_GRACE_SECONDS=300

fail() {
  echo "TRNM formal capacity matrix failed: $*" >&2
  exit 2
}

for forbidden_name in \
  BASH_ENV ENV CDPATH LD_PRELOAD LD_LIBRARY_PATH LD_AUDIT LD_DEBUG LD_PROFILE \
  GCONV_PATH LOCPATH NLSPATH PYTHONHOME PYTHONPATH \
  GIT_DIR GIT_WORK_TREE GIT_CONFIG_GLOBAL GIT_CONFIG_SYSTEM CURL_HOME \
  HTTP_PROXY HTTPS_PROXY ALL_PROXY http_proxy https_proxy all_proxy \
  DOCKER_HOST DOCKER_CONTEXT DOCKER_CONFIG DOCKER_TLS_VERIFY DOCKER_CERT_PATH \
  CEX_PROJECT_ROOT CEX_ENV_FILE CEX_POSTGRES_CONTAINER_NAME CEX_POSTGRES_USER \
  CEX_POSTGRES_DB CEX_POSTGRES_PASSWORD CEX_DOCKER_USE_SUDO DATABASE_URL \
  LEDGER_ADMIN_TOKEN IDENTITY_ADMIN_TOKEN TRNM_CEX_LEDGER_URL \
  TRNM_ENTITLEMENT_SIGNER_URL TRNM_GAME_SERVER_URL TRNM_GAME_SERVER_RELEASE_DIR \
  TRNM_CAPACITY_ALLOW_DIRTY TRNM_CAPACITY_CONCURRENCY \
  TRNM_CAPACITY_DURATION_SECONDS TRNM_CAPACITY_MIN_AVAILABLE_MIB \
  TRNM_CAPACITY_MAX_DATABASE_CONNECTIONS \
  TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS TRNM_CAPACITY_RUN_ID \
  TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE TRNM_CAPACITY_SCOPE_PROBE; do
  [[ ! -v "$forbidden_name" ]] \
    || fail "external $forbidden_name overrides are prohibited"
done


case "${TRNM_CAPACITY_MATRIX_SANITIZED_ENTRY:-0}" in
0)
  sanitized_script="$(/usr/bin/realpath -e -- "${BASH_SOURCE[0]}")" \
    || fail "capacity matrix script path is not canonical"
  sanitized_environment=(
    PATH="$TRUSTED_FORMAL_PATH"
    LC_ALL=C
    HOME="${HOME:-}"
    TRNM_CAPACITY_MATRIX_SANITIZED_ENTRY=1
  )
  for allowed_name in XDG_RUNTIME_DIR TRNM_CAPACITY_PROFILES \
    TRNM_CAPACITY_PROFILE_DURATION_SECONDS; do
    if [[ -v "$allowed_name" ]]; then
      sanitized_environment+=("$allowed_name=${!allowed_name}")
    fi
  done
  exec /usr/bin/env -i "${sanitized_environment[@]}" "$sanitized_script" "$@"
  ;;
1)
  unset TRNM_CAPACITY_MATRIX_SANITIZED_ENTRY
  ;;
*)
  fail "TRNM_CAPACITY_MATRIX_SANITIZED_ENTRY is an internal recursion guard"
  ;;
esac

while IFS= read -r environment_name; do
  case "$environment_name" in
    PATH|LC_ALL|HOME|XDG_RUNTIME_DIR|PWD|SHLVL|_|\
      TRNM_CAPACITY_PROFILES|TRNM_CAPACITY_PROFILE_DURATION_SECONDS)
      ;;
    *)
      builtin unset "$environment_name" 2>/dev/null \
        || builtin export -n "$environment_name" 2>/dev/null \
        || fail "could not clear inherited environment variable: $environment_name"
      ;;
  esac
done < <(builtin compgen -e)
unset environment_name

export PATH="$TRUSTED_FORMAL_PATH"
export LC_ALL=C
export NO_PROXY="127.0.0.1,localhost"
export no_proxy="$NO_PROXY"
unset BASH_ENV ENV CDPATH LD_PRELOAD LD_LIBRARY_PATH LD_AUDIT LD_DEBUG LD_PROFILE \
  GCONV_PATH LOCPATH NLSPATH PYTHONHOME PYTHONPATH TMPDIR \
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
  || fail "HOME or user identity is not canonical"
canonical_runtime_dir="/run/user/$UID"
[[ ! -v XDG_RUNTIME_DIR || "$XDG_RUNTIME_DIR" == "$canonical_runtime_dir" ]] \
  || fail "XDG_RUNTIME_DIR is not canonical"
export HOME="$canonical_home"
export USER="$canonical_user"
export LOGNAME="$canonical_user"
export XDG_RUNTIME_DIR="$canonical_runtime_dir"
export DBUS_SESSION_BUS_ADDRESS="unix:path=$canonical_runtime_dir/bus"

integer_at_least() {
  local value="$1" minimum="$2"
  [[ "$value" =~ ^[1-9][0-9]*$ && "$value" -ge "$minimum" ]]
}

validate_profile_evidence() {
  local evidence_file="$1" expected_profile="$2" expected_duration="$3"
  local expected_run_id="$4"
  jq -s -e \
    --argjson profile "$expected_profile" \
    --argjson duration "$expected_duration" \
    --arg run_id "$expected_run_id" '
    def finite_number: type == "number" and isfinite;
    def nonnegative_finite: finite_number and . >= 0;
    def positive_integer: type == "number" and isfinite and . >= 1 and floor == .;
    length == 1 and (.[0] |
      type == "object" and
      .contract_version == "trnm_online_capacity_soak_v2" and
      .run_id == $run_id and
      .formal_evidence == true and .public_launch_credit == false and
      .local_only == true and .decision_final == true and
      .cleanup_restored == true and .passed == true and
      .workload_passed == true and .report_schema_valid == true and
      .bounded_resource_scope == true and .resource_scope_validated == true and
      .static_inputs_bound_before_and_after == true and
      .service_processes_and_ports_bound_before_and_after == true and
      .immutable_release_verified_before_and_after == true and
      .concurrency == $profile and
      .requested_duration_seconds == $duration and
      (.actual_duration_seconds | positive_integer and . >= $duration) and
      (.waves | positive_integer) and
      (.completed_matches | positive_integer and . >= $profile) and
      .unique_matches == .completed_matches and .failures == 0 and
      .all_settled == true and .active_run_matches_final == 0 and
      (.command_ack_samples | positive_integer) and
      (.command_ack_p95_ms | nonnegative_finite) and
      (.command_ack_max_ms | nonnegative_finite) and
      (.authoritative_effect_samples | positive_integer) and
      (.authoritative_effect_p95_ms | nonnegative_finite) and
      (.authoritative_effect_max_ms | nonnegative_finite) and
      (.max_absolute_match_tick_drift | nonnegative_finite) and
      (.max_client_terminal_observation_tick_skew |
        nonnegative_finite and . < 20) and
      (.max_actor_clock_cumulative_abs_drift_ticks |
        nonnegative_finite and . < 2) and
      .actor_clock_drift_gate_source ==
        "server_readiness_cumulative_actor_clock" and
      .thresholds.max_actor_clock_cumulative_abs_drift_ticks_exclusive == 2 and
      .thresholds.max_client_terminal_observation_tick_skew_exclusive == 20 and
      .postgres_log_capture_succeeded == true and
      .journal_warning_or_error_count == 0 and
      .postgres_crash_signature_count == 0 and
      .host_oom_kills == 0 and
      (.resource_cgroup | type == "string" and startswith("/") and . != "/") and
      .resource_memory_max_bytes == 2147483648 and
      .minimum_host_available_memory_mib >= 3072 and
      (.source.trnm_git_head | type == "string" and test("^[0-9a-f]{40}([0-9a-f]{24})?$")) and
      (.source.cex_git_head | type == "string" and test("^[0-9a-f]{40}([0-9a-f]{24})?$")) and
      (.source.release_dir | type == "string" and startswith("/")) and
      (.source.release_manifest_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
      (.source.online_e2e_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
      (.source.game_server_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
      (.source.signer_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
      ([.service_restarts[]] | all(. == 0)) and
      ([.cgroup_oom_kills[]] | all(. == 0)) and
      .postgres.restart_delta == 0 and .postgres.after.running == true and
      .postgres.after.oom_killed == false and .wal.failed_delta == 0 and
      .wal.archived_delta > 0 and .wal.after.archiver_recovered == true and
      .operational_samples.count > 0 and
      .operational_samples.all_healthy == true and
      .operational_samples.failed_samples == 0 and
      (.operational_samples.maximum_actor_clock_cumulative_abs_drift_ticks |
        nonnegative_finite and . < 2)
    )
  ' "$evidence_file" >/dev/null
}

profile_source_binding() {
  jq -S -c '{source,resource_memory_max_bytes,
    minimum_host_available_memory_mib}' "$1" | sha256sum | awk '{print $1}'
}

run_self_test() (
  local fixture_root valid short failed null_effect bad_actor_drift
  local bad_terminal_skew not_final duplicate
  fixture_root="$(mktemp -d /tmp/trnm-capacity-matrix-self-test.XXXXXX)"
  trap 'rm -rf -- "$fixture_root"' EXIT
  valid="$fixture_root/valid.json"
  short="$fixture_root/short.json"
  failed="$fixture_root/failed.json"
  null_effect="$fixture_root/null-effect.json"
  bad_actor_drift="$fixture_root/bad-actor-drift.json"
  bad_terminal_skew="$fixture_root/bad-terminal-skew.json"
  not_final="$fixture_root/not-final.json"
  duplicate="$fixture_root/duplicate.json"
  jq -n --arg run_id capacity-1-4 '
    {
      contract_version:"trnm_online_capacity_soak_v2",run_id:$run_id,
      formal_evidence:true,public_launch_credit:false,local_only:true,
      decision_final:true,cleanup_restored:true,passed:true,workload_passed:true,
      report_schema_valid:true,bounded_resource_scope:true,
      resource_scope_validated:true,static_inputs_bound_before_and_after:true,
      service_processes_and_ports_bound_before_and_after:true,
      immutable_release_verified_before_and_after:true,
      concurrency:4,requested_duration_seconds:7200,actual_duration_seconds:7201,
      waves:1,completed_matches:4,unique_matches:4,failures:0,all_settled:true,
      active_run_matches_final:0,command_ack_samples:4,command_ack_p95_ms:10,
      command_ack_max_ms:12,authoritative_effect_samples:20,
      authoritative_effect_p95_ms:20,authoritative_effect_max_ms:25,
      max_absolute_match_tick_drift:10.5,
      max_client_terminal_observation_tick_skew:10.5,
      max_actor_clock_cumulative_abs_drift_ticks:0.5,
      actor_clock_drift_gate_source:"server_readiness_cumulative_actor_clock",
      thresholds:{max_actor_clock_cumulative_abs_drift_ticks_exclusive:2,
        max_client_terminal_observation_tick_skew_exclusive:20},
      postgres_log_capture_succeeded:true,
      journal_warning_or_error_count:0,postgres_crash_signature_count:0,
      host_oom_kills:0,resource_cgroup:"/formal.scope",
      resource_memory_max_bytes:2147483648,minimum_host_available_memory_mib:3072,
      source:{trnm_git_head:("a"*40),cex_git_head:("b"*40),
        release_dir:"/release",release_manifest_sha256:("c"*64),
        online_e2e_sha256:("d"*64),game_server_sha256:("e"*64),
        signer_sha256:("f"*64)},
      service_restarts:{game_server:0,signer:0,ledger:0,consumer:0},
      cgroup_oom_kills:{game_server:0,signer:0,ledger:0,consumer:0,harness:0},
      postgres:{restart_delta:0,after:{running:true,oom_killed:false}},
      wal:{failed_delta:0,archived_delta:1,after:{archiver_recovered:true}},
      operational_samples:{count:2,all_healthy:true,failed_samples:0,
        maximum_actor_clock_cumulative_abs_drift_ticks:0.5}
    }
  ' >"$valid"
  validate_profile_evidence "$valid" 4 7200 capacity-1-4
  jq '.actual_duration_seconds=7199' "$valid" >"$short"
  jq '.passed=false' "$valid" >"$failed"
  jq '.authoritative_effect_p95_ms=null' "$valid" >"$null_effect"
  jq '.max_actor_clock_cumulative_abs_drift_ticks=2
    | .operational_samples.maximum_actor_clock_cumulative_abs_drift_ticks=2' \
    "$valid" >"$bad_actor_drift"
  jq '.max_client_terminal_observation_tick_skew=20' \
    "$valid" >"$bad_terminal_skew"
  jq '.decision_final=false | .cleanup_restored=false' "$valid" >"$not_final"
  { cat "$valid"; cat "$valid"; } >"$duplicate"
  ! validate_profile_evidence "$short" 4 7200 capacity-1-4
  ! validate_profile_evidence "$failed" 4 7200 capacity-1-4
  ! validate_profile_evidence "$null_effect" 4 7200 capacity-1-4
  ! validate_profile_evidence "$bad_actor_drift" 4 7200 capacity-1-4
  ! validate_profile_evidence "$bad_terminal_skew" 4 7200 capacity-1-4
  ! validate_profile_evidence "$not_final" 4 7200 capacity-1-4
  ! validate_profile_evidence "$valid" 8 7200 capacity-1-4
  ! validate_profile_evidence "$duplicate" 4 7200 capacity-1-4
  jq -n '{status:"passed",contract_version:"trnm_online_capacity_matrix_self_test_v1",
    formal_evidence:false,valid_fixture_accepted:true,short_fixture_rejected:true,
    failed_fixture_rejected:true,null_effect_fixture_rejected:true,
    actor_cumulative_drift_fixture_rejected:true,
    terminal_observation_skew_fixture_rejected:true,
    unfinished_cleanup_fixture_rejected:true,profile_mismatch_rejected:true,
    multiple_json_values_rejected:true}'
)

if (( $# > 0 )); then
  [[ "$#" -eq 1 && "$1" == --self-test ]] \
    || fail "the only supported argument is --self-test"
  run_self_test
  exit 0
fi

readonly SCRIPT_PATH="$(realpath -e -- "${BASH_SOURCE[0]}")"
readonly ROOT_DIR="$(dirname "$(dirname "$SCRIPT_PATH")")"
readonly RUN_ROOT="$ROOT_DIR/run"
readonly EVIDENCE_ROOT="$RUN_ROOT/online-capacity"
readonly LOCK_ROOT="$RUN_ROOT/locks"
readonly SOAK_SCRIPT="$ROOT_DIR/scripts/check-trnm-online-capacity-soak.sh"

ensure_private_directory() {
  local directory="$1"
  [[ ! -L "$directory" ]] || fail "private directory is a symbolic link: $directory"
  if [[ -e "$directory" ]]; then
    [[ -d "$directory" \
        && "$(stat -c '%u:%g' "$directory")" == "$UID:$canonical_gid" ]] \
      || fail "private directory has the wrong type or owner: $directory"
    chmod 0700 "$directory"
  else
    install -d -m 0700 -- "$directory"
  fi
  [[ -d "$directory" && ! -L "$directory" \
      && "$(stat -c '%u:%g:%a' "$directory")" == \
        "$UID:$canonical_gid:700" ]] \
    || fail "private directory validation failed: $directory"
}

validate_lock_path() {
  local lock_file="$1"
  if [[ -e "$lock_file" || -L "$lock_file" ]]; then
    [[ -f "$lock_file" && ! -L "$lock_file" \
        && "$(stat -c '%u:%g:%a:%h' "$lock_file")" == \
          "$UID:$canonical_gid:600:1" ]] \
      || fail "matrix lock is not a private single-link regular file"
  fi
}

validate_lock_fd() {
  local lock_file="$1" fd_path=/proc/self/fd/7
  [[ -f "$fd_path" && ! -L "$lock_file" \
      && "$(stat -Lc '%u:%g:%a:%h' "$fd_path")" == \
        "$UID:$canonical_gid:600:1" \
      && "$(stat -Lc '%d:%i' "$fd_path")" == "$(stat -c '%d:%i' "$lock_file")" \
      && "$(readlink -e -- "$fd_path")" == "$(realpath -e -- "$lock_file")" ]] \
    || fail "matrix lock descriptor identity validation failed"
}

atomic_summary() {
  local summary="$EVIDENCE/summary.json" temporary="$EVIDENCE/.summary.json.tmp.$$"
  if jq -e . >"$temporary" && chmod 0600 "$temporary" \
      && mv -- "$temporary" "$summary"; then
    cat "$summary"
    return 0
  fi
  rm -f -- "$temporary"
  return 1
}

write_failure_summary() {
  local status="$1" failed_profile="$2" failed_stdout="$3"
  jq -n \
    --arg contract_version trnm_online_capacity_matrix_v2 \
    --arg run_id "$RUN_ID" --arg status "$status" \
    --argjson failed_profile "$failed_profile" \
    --arg failed_stdout "$failed_stdout" \
    --argjson requested_profiles "$REQUESTED_PROFILES_JSON" \
    --argjson duration "$PROFILE_DURATION_SECONDS" \
    --argjson completed_profiles "$completed_profiles_json" '
    {contract_version:$contract_version,run_id:$run_id,status:$status,
      formal_evidence:true,public_launch_credit:false,local_only:true,
      decision_final:true,passed:false,cleanup_restored:false,
      requested_profiles:$requested_profiles,
      profile_duration_seconds:$duration,failed_profile:$failed_profile,
      failed_profile_stdout:$failed_stdout,
      validated_profiles_before_failure:$completed_profiles,
      measured_capacity:null}
  ' | atomic_summary
}

umask 077
[[ -d "$RUN_ROOT" && ! -L "$RUN_ROOT" \
    && "$(stat -c '%u:%g' "$RUN_ROOT")" == "$UID:$canonical_gid" ]] \
  || fail "canonical run root must be a non-symlink directory owned by the current user"
ensure_private_directory "$LOCK_ROOT"
ensure_private_directory "$EVIDENCE_ROOT"
[[ -x "$SOAK_SCRIPT" && -f "$SOAK_SCRIPT" && ! -L "$SOAK_SCRIPT" ]] \
  || fail "formal capacity soak script is unavailable or not canonical"
SOAK_SCRIPT_SHA256="$(sha256sum "$SOAK_SCRIPT" | awk '{print $1}')"

matrix_lock="$LOCK_ROOT/trnm-online-capacity-matrix.lock"
validate_lock_path "$matrix_lock"
exec 7>>"$matrix_lock"
validate_lock_fd "$matrix_lock"
flock -n 7 || fail "another formal capacity matrix is already active"
validate_lock_path "$matrix_lock"
validate_lock_fd "$matrix_lock"

PROFILES_CSV="${TRNM_CAPACITY_PROFILES:-4,8,16,32}"
PROFILE_DURATION_SECONDS="${TRNM_CAPACITY_PROFILE_DURATION_SECONDS:-7200}"
integer_at_least "$PROFILE_DURATION_SECONDS" "$FORMAL_MIN_DURATION_SECONDS" \
  && (( PROFILE_DURATION_SECONDS <= FORMAL_MAX_DURATION_SECONDS )) \
  || fail "profile duration must be between 7200 and 86400 seconds"

IFS=',' read -r -a profiles <<<"$PROFILES_CSV"
(( ${#profiles[@]} > 0 )) || fail "at least one formal profile is required"
canonical_profiles="$(IFS=,; printf '%s' "${profiles[*]}")"
[[ "$canonical_profiles" == "$PROFILES_CSV" ]] \
  || fail "profiles must be a canonical comma-separated list"
previous_profile=0
for profile in "${profiles[@]}"; do
  [[ "$profile" =~ ^(4|8|16|32)$ ]] \
    || fail "formal profiles must be selected from 4,8,16,32"
  (( profile > previous_profile )) \
    || fail "formal profiles must be unique and strictly increasing"
  previous_profile="$profile"
done
REQUESTED_PROFILES_JSON="$(printf '%s\n' "${profiles[@]}" | jq -s 'map(tonumber)')"

matrix_epoch="$(date +%s)"
RUN_ID="capacity-matrix-${matrix_epoch}-${RANDOM}${RANDOM}"
EVIDENCE="$EVIDENCE_ROOT/$RUN_ID"
mkdir -m 0700 -- "$EVIDENCE"
[[ -d "$EVIDENCE" && ! -L "$EVIDENCE" \
    && "$(stat -c '%u:%g:%a' "$EVIDENCE")" == \
      "$UID:$canonical_gid:700" ]] \
  || fail "matrix evidence directory validation failed"

completed_profiles_json='[]'
profile_source_sha256=""
profile_files=()
for profile in "${profiles[@]}"; do
  [[ "$(sha256sum "$SOAK_SCRIPT" | awk '{print $1}')" == \
      "$SOAK_SCRIPT_SHA256" ]] \
    || fail "capacity soak script changed during the matrix"
  profile_run_id="capacity-${matrix_epoch}-${profile}${RANDOM}${RANDOM}"
  profile_stdout_tmp="$EVIDENCE/.profile-${profile}.stdout.tmp"
  profile_stderr="$EVIDENCE/profile-${profile}.stderr"
  profile_timeout_seconds=$((PROFILE_DURATION_SECONDS + PROFILE_COMPLETION_GRACE_SECONDS))
  set +e
  (
    exec 7>&-
    exec timeout --signal=TERM --kill-after="${PROFILE_CLEANUP_KILL_GRACE_SECONDS}s" \
      "${profile_timeout_seconds}s" env -i \
      PATH="$TRUSTED_FORMAL_PATH" LC_ALL=C HOME="$canonical_home" \
      USER="$canonical_user" LOGNAME="$canonical_user" SHELL=/usr/bin/bash \
      XDG_RUNTIME_DIR="$canonical_runtime_dir" \
      DBUS_SESSION_BUS_ADDRESS="unix:path=$canonical_runtime_dir/bus" \
      NO_PROXY="127.0.0.1,localhost" no_proxy="127.0.0.1,localhost" \
      TRNM_CAPACITY_CONCURRENCY="$profile" \
      TRNM_CAPACITY_DURATION_SECONDS="$PROFILE_DURATION_SECONDS" \
      TRNM_CAPACITY_RUN_ID="$profile_run_id" \
      "$SOAK_SCRIPT"
  ) >"$profile_stdout_tmp" 2>"$profile_stderr"
  profile_status=$?
  set -e
  if (( profile_status != 0 )); then
    failed_stdout="$EVIDENCE/profile-${profile}.failed.stdout"
    mv -- "$profile_stdout_tmp" "$failed_stdout"
    write_failure_summary profile_execution_failed "$profile" "$failed_stdout"
    exit 1
  fi
  if ! validate_profile_evidence "$profile_stdout_tmp" "$profile" \
      "$PROFILE_DURATION_SECONDS" "$profile_run_id"; then
    invalid_stdout="$EVIDENCE/profile-${profile}.invalid.stdout"
    mv -- "$profile_stdout_tmp" "$invalid_stdout"
    write_failure_summary invalid_profile_evidence "$profile" "$invalid_stdout"
    exit 1
  fi
  current_source_sha256="$(profile_source_binding "$profile_stdout_tmp")"
  if [[ -z "$profile_source_sha256" ]]; then
    profile_source_sha256="$current_source_sha256"
  elif [[ "$current_source_sha256" != "$profile_source_sha256" ]]; then
    inconsistent_stdout="$EVIDENCE/profile-${profile}.inconsistent.stdout"
    mv -- "$profile_stdout_tmp" "$inconsistent_stdout"
    write_failure_summary inconsistent_profile_sources "$profile" "$inconsistent_stdout"
    exit 1
  fi
  profile_file="$EVIDENCE/profile-${profile}.json"
  chmod 0600 "$profile_stdout_tmp"
  mv -- "$profile_stdout_tmp" "$profile_file"
  profile_files+=("$profile_file")
  completed_profiles_json="$(
    jq --argjson profile "$profile" '. + [$profile]' <<<"$completed_profiles_json"
  )"
done

[[ "$(sha256sum "$SOAK_SCRIPT" | awk '{print $1}')" == \
    "$SOAK_SCRIPT_SHA256" ]] \
  || fail "capacity soak script changed before matrix finalization"

jq -s \
  --arg contract_version trnm_online_capacity_matrix_v2 \
  --arg run_id "$RUN_ID" \
  --arg source_binding_sha256 "$profile_source_sha256" \
  --argjson requested_profiles "$REQUESTED_PROFILES_JSON" \
  --argjson duration "$PROFILE_DURATION_SECONDS" '
  {
    contract_version:$contract_version,run_id:$run_id,
    status:"all_requested_profiles_passed",
    formal_evidence:true,public_launch_credit:false,local_only:true,
    decision_final:true,cleanup_restored:true,passed:true,
    requested_profiles:$requested_profiles,profile_duration_seconds:$duration,
    source_binding_sha256:$source_binding_sha256,
    profiles:map({run_id,concurrency,requested_duration_seconds,
      actual_duration_seconds,completed_matches,unique_matches,
      command_ack_samples,command_ack_p95_ms,command_ack_max_ms,
      authoritative_effect_samples,authoritative_effect_p95_ms,
      authoritative_effect_max_ms,max_absolute_match_tick_drift,
      max_client_terminal_observation_tick_skew,
      max_actor_clock_cumulative_abs_drift_ticks,
      source,passed,cleanup_restored,decision_final}),
    measured_capacity:(map(.concurrency)|max),
    capacity_claim:"highest fully validated requested local formal profile"
  }
' "${profile_files[@]}" | atomic_summary
