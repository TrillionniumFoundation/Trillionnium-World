#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-rollback-backup-drill.json"
PORT="${TRILLIONNIUM_WORLD_ROLLBACK_DRILL_PORT:-28793}"
BIND_ADDR="127.0.0.1:$PORT"
STATE_FILE="$ACCEPTANCE_DIR/rollback-drill-state.json"
BACKUP_FILE="$ACCEPTANCE_DIR/rollback-drill-state.backup.json"
CORRUPT_FILE="$ACCEPTANCE_DIR/rollback-drill-state.corrupt.json"
RESTORED_HOME="$ACCEPTANCE_DIR/rollback-drill-restored-home.json"
RESTORED_HEALTH="$ACCEPTANCE_DIR/rollback-drill-restored-health.json"
COMMAND_EVIDENCE="$ACCEPTANCE_DIR/rollback-drill-command.json"
LOG_FILE="$ACCEPTANCE_DIR/rollback-drill-server.log"

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

start_server() {
  local reset_flag="$1"
  rm -f "$LOG_FILE"
  if [[ "$reset_flag" == "reset" ]]; then
    "$BIN" serve --bind "$BIND_ADDR" --state-file "$STATE_FILE" --reset-state >"$LOG_FILE" 2>&1 &
  else
    "$BIN" serve --bind "$BIND_ADDR" --state-file "$STATE_FILE" >"$LOG_FILE" 2>&1 &
  fi
  SERVER_PID=$!
  for _ in $(seq 1 80); do
    if ! kill -0 "$SERVER_PID" >/dev/null 2>&1; then
      cat "$LOG_FILE" >&2 || true
      return 1
    fi
    if curl -fsS "http://$BIND_ADDR/health" 2>/dev/null | grep -q 'trillionnium_world_dev_runtime_v1'; then
      return 0
    fi
    sleep 0.1
  done
  printf 'rollback drill server did not become healthy on %s\n' "$BIND_ADDR" >&2
  return 1
}

stop_server() {
  kill "$SERVER_PID" >/dev/null 2>&1 || true
  wait "$SERVER_PID" >/dev/null 2>&1 || true
}

rm -f "$STATE_FILE" "$BACKUP_FILE" "$CORRUPT_FILE" "$RESTORED_HOME" "$RESTORED_HEALTH" "$COMMAND_EVIDENCE"
start_server reset
trap stop_server EXIT
curl -fsS "http://$BIND_ADDR/world/command?direction=east&actor_id=local-player" >"$COMMAND_EVIDENCE"
grep -q 'starter-studio' "$COMMAND_EVIDENCE"
stop_server
trap - EXIT

cp "$STATE_FILE" "$BACKUP_FILE"
BACKUP_SHA256="$(sha256sum "$BACKUP_FILE" | awk '{print $1}')"
printf '{"corrupted": true, "reason": "rollback drill bad state"}' >"$CORRUPT_FILE"
cp "$CORRUPT_FILE" "$STATE_FILE"
CORRUPT_SHA256="$(sha256sum "$STATE_FILE" | awk '{print $1}')"

cp "$BACKUP_FILE" "$STATE_FILE"
RESTORED_SHA256="$(sha256sum "$STATE_FILE" | awk '{print $1}')"
start_server no-reset
trap stop_server EXIT
curl -fsS "http://$BIND_ADDR/health" >"$RESTORED_HEALTH"
curl -fsS "http://$BIND_ADDR/world/home" >"$RESTORED_HOME"
grep -q 'file_backed_json' "$RESTORED_HEALTH"
grep -q 'starter-studio' "$RESTORED_HOME"
stop_server
trap - EXIT

STATUS="release_rollback_backup_drill_green"
if [[ "$BACKUP_SHA256" != "$RESTORED_SHA256" || "$BACKUP_SHA256" == "$CORRUPT_SHA256" ]]; then
  STATUS="release_rollback_backup_drill_failed"
fi

jq -n \
  --arg contract_version "trillionnium_world_release_rollback_backup_drill_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg release_binary "$BIN" \
  --arg release_binary_sha256 "$(sha256sum "$BIN" | awk '{print $1}')" \
  --arg state_file "$STATE_FILE" \
  --arg backup_file "$BACKUP_FILE" \
  --arg corrupt_file "$CORRUPT_FILE" \
  --arg backup_sha256 "$BACKUP_SHA256" \
  --arg corrupt_sha256 "$CORRUPT_SHA256" \
  --arg restored_sha256 "$RESTORED_SHA256" \
  --arg command_evidence "$COMMAND_EVIDENCE" \
  --arg restored_health "$RESTORED_HEALTH" \
  --arg restored_home "$RESTORED_HOME" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trnm_world_release_rollback_backup_drill",
    public_launch_credit: "local_backup_restore_and_bad_state_rollback_drill",
    production_ready: false,
    release: {
      binary_path: $release_binary,
      binary_sha256: $release_binary_sha256
    },
    drill: {
      state_file: $state_file,
      backup_file: $backup_file,
      corrupt_file: $corrupt_file,
      backup_sha256: $backup_sha256,
      corrupt_sha256: $corrupt_sha256,
      restored_sha256: $restored_sha256,
      restored_equals_backup: ($backup_sha256 == $restored_sha256),
      corrupt_differs_from_backup: ($backup_sha256 != $corrupt_sha256),
      command_evidence: $command_evidence,
      restored_health_evidence: $restored_health,
      restored_home_evidence: $restored_home,
      restored_player_node_id: "starter-studio"
    },
    remaining_for_public_release_ops: [
      "off-host_backup_storage",
      "scheduled_backup_policy",
      "managed_database_restore",
      "previous_release_binary_rollback",
      "operator_oncall_runbook"
    ]
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "release_rollback_backup_drill_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_RELEASE_ROLLBACK_BACKUP_DRILL_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_RELEASE_ROLLBACK_BACKUP_DRILL_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
exit 1
