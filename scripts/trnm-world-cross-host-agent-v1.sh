#!/usr/bin/env bash
set -euo pipefail

phase="${1:-}"
case "$phase" in
  identity|preflight|stop-primary|promote|pitr|old-primary-return|reconcile|cleanup) ;;
  *) echo "unsupported cross-host phase" >&2; exit 64 ;;
esac
: "${TRNM_REMOTE_EVIDENCE_ROOT:?required}"
[[ "$TRNM_REMOTE_EVIDENCE_ROOT" == /* ]] || { echo "evidence root must be absolute" >&2; exit 1; }
umask 077
mkdir -p "$TRNM_REMOTE_EVIDENCE_ROOT"/{raw,summary}

safe_file() {
  [[ -f "$1" && ! -L "$1" && -s "$1" ]] || { echo "missing or unsafe file: $1" >&2; exit 1; }
}

hash_text() {
  printf '%s' "$1" | sha256sum | awk '{print $1}'
}

if [[ "$phase" == identity ]]; then
  hostname_value="$(hostname -f 2>/dev/null || hostname)"
  machine_source=""
  if [[ -r /etc/machine-id ]]; then machine_source="$(cat /etc/machine-id)"; else machine_source="$hostname_value"; fi
  boot_source="$(cat /proc/sys/kernel/random/boot_id 2>/dev/null || printf unavailable)"
  failure_domain="${TRNM_FAILURE_DOMAIN:?TRNM_FAILURE_DOMAIN is required and must identify an independently administered domain}"
  database_system_identifier=""
  timeline=""
  if [[ -n "${TRNM_IDENTITY_DATABASE_URL_FILE:-}" ]]; then
    safe_file "$TRNM_IDENTITY_DATABASE_URL_FILE"
    url="$(cat "$TRNM_IDENTITY_DATABASE_URL_FILE")"
    database_system_identifier="$(psql "$url" -AtX -v ON_ERROR_STOP=1 -c "select system_identifier from pg_control_system()" | tr -d '\r\n')"
    timeline="$(psql "$url" -AtX -v ON_ERROR_STOP=1 -c "select timeline_id from pg_control_checkpoint()" | tr -d '\r\n')"
  elif command -v pg_controldata >/dev/null 2>&1 && [[ -n "${PGDATA:-}" && -d "$PGDATA" ]]; then
    database_system_identifier="$(pg_controldata "$PGDATA" | awk -F: '/Database system identifier/ {gsub(/^[ \t]+/,"",$2); print $2}')"
    timeline="$(pg_controldata "$PGDATA" | awk -F: '/Latest checkpoint.s TimeLineID/ {gsub(/^[ \t]+/,"",$2); print $2}')"
  else
    echo "database identity cannot be observed; set TRNM_IDENTITY_DATABASE_URL_FILE or PGDATA" >&2
    exit 1
  fi
  [[ "$database_system_identifier" =~ ^[0-9]+$ ]] || { echo "invalid database system identifier" >&2; exit 1; }
  [[ "$timeline" =~ ^[0-9]+$ ]] || { echo "invalid database timeline" >&2; exit 1; }
  python3 - "$hostname_value" "$(hash_text "$machine_source")" "$(hash_text "$boot_source")" "$failure_domain" "$database_system_identifier" "$timeline" <<'PY'
import json, sys
print(json.dumps({
  "schema":"trnm_world_cross_host_identity_v1",
  "host_fingerprint":sys.argv[1],
  "machine_fingerprint":sys.argv[2],
  "boot_fingerprint":sys.argv[3],
  "failure_domain":sys.argv[4],
  "database_system_identifier":sys.argv[5],
  "database_timeline":sys.argv[6],
  "declared_replica_of_host_a":bool(int(__import__('os').environ.get('TRNM_DECLARED_REPLICA_OF_HOST_A','0'))),
  "secret_values_recorded":False,
}, sort_keys=True))
PY
  exit 0
fi

# Every mutating or fault phase must be supplied by the accountable environment
# owner as a local executable file. Shell snippets in environment variables are
# deliberately forbidden so the exact hook can be hashed and reviewed.
hook_variable=""
case "$phase" in
  preflight) hook_variable="TRNM_PREFLIGHT_HOOK" ;;
  stop-primary) hook_variable="TRNM_STOP_PRIMARY_HOOK" ;;
  promote) hook_variable="TRNM_PROMOTE_HOOK" ;;
  pitr) hook_variable="TRNM_PITR_HOOK" ;;
  old-primary-return) hook_variable="TRNM_OLD_PRIMARY_RETURN_HOOK" ;;
  reconcile) hook_variable="TRNM_RECONCILE_HOOK" ;;
  cleanup) hook_variable="TRNM_CLEANUP_HOOK" ;;
esac
hook="${!hook_variable:-}"
[[ -n "$hook" ]] || { echo "$hook_variable is required for phase $phase" >&2; exit 1; }
safe_file "$hook"
[[ -x "$hook" ]] || { echo "phase hook is not executable: $hook" >&2; exit 1; }

phase_dir="$TRNM_REMOTE_EVIDENCE_ROOT/raw/$phase"
mkdir -p "$phase_dir"
sha256sum "$hook" > "$phase_dir/hook.sha256"
printf '%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$phase_dir/started-at.txt"
set +e
timeout "${TRNM_REMOTE_PHASE_TIMEOUT_SECONDS:-1200}" "$hook" > "$phase_dir/stdout.log" 2> "$phase_dir/stderr.log"
rc=$?
set -e
printf '%s\n' "$rc" > "$phase_dir/exit-code.txt"
printf '%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$phase_dir/ended-at.txt"
find "$phase_dir" -type f -print0 | sort -z | xargs -0 sha256sum > "$phase_dir/SHA256SUMS"
[[ "$rc" -eq 0 ]] || exit "$rc"
printf 'TRNM_WORLD_CROSS_HOST_PHASE_%s=PASS\n' "$(printf '%s' "$phase" | tr '[:lower:]-' '[:upper:]_')"
