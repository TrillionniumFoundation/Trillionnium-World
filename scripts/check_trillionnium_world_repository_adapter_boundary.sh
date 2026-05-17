#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S3_repository_adapter/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/repository-adapter-boundary.json"
SQLITE_SCHEMA="$ACCEPTANCE_DIR/world_repository_schema.sqlite.sql"
POSTGRES_SCHEMA="$ACCEPTANCE_DIR/world_repository_schema.postgres.sql"
SQLITE_DB="$ACCEPTANCE_DIR/world-repository-smoke.sqlite3"
STATE_FILE="$ACCEPTANCE_DIR/world-state.json"
SMOKE_FILE="$ACCEPTANCE_DIR/dev-runtime-repository-smoke.json"
QUERY_EVIDENCE="$ACCEPTANCE_DIR/sqlite-readback.json"

mkdir -p "$ACCEPTANCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo build -p trnm-world-server
  cargo run -p trnm-world-server -- dev-runtime-repository-smoke "$STATE_FILE" >"$SMOKE_FILE"
)

cat >"$SQLITE_SCHEMA" <<'SQL'
CREATE TABLE IF NOT EXISTS trnm_world_repository_migrations (
  migration_id TEXT PRIMARY KEY,
  applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  checksum TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS trnm_world_states (
  world_id TEXT PRIMARY KEY,
  contract_version TEXT NOT NULL,
  repository_contract TEXT NOT NULL,
  source_of_truth TEXT NOT NULL,
  generation INTEGER NOT NULL DEFAULT 0,
  state_sha256 TEXT NOT NULL,
  state_json TEXT NOT NULL CHECK (json_valid(state_json)),
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS trnm_world_events (
  event_id TEXT PRIMARY KEY,
  world_id TEXT NOT NULL REFERENCES trnm_world_states(world_id),
  idempotency_key TEXT NOT NULL UNIQUE,
  actor_id TEXT NOT NULL,
  command_kind TEXT NOT NULL,
  command_json TEXT NOT NULL CHECK (json_valid(command_json)),
  response_json TEXT NOT NULL CHECK (json_valid(response_json)),
  state_sha256_after TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_trnm_world_events_world_created
  ON trnm_world_events(world_id, created_at);
CREATE INDEX IF NOT EXISTS idx_trnm_world_events_actor_created
  ON trnm_world_events(actor_id, created_at);
SQL

cat >"$POSTGRES_SCHEMA" <<'SQL'
CREATE TABLE IF NOT EXISTS trnm_world_repository_migrations (
  migration_id TEXT PRIMARY KEY,
  applied_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  checksum TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS trnm_world_states (
  world_id TEXT PRIMARY KEY,
  contract_version TEXT NOT NULL,
  repository_contract TEXT NOT NULL,
  source_of_truth TEXT NOT NULL,
  generation BIGINT NOT NULL DEFAULT 0,
  state_sha256 TEXT NOT NULL,
  state_json JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS trnm_world_events (
  event_id TEXT PRIMARY KEY,
  world_id TEXT NOT NULL REFERENCES trnm_world_states(world_id),
  idempotency_key TEXT NOT NULL UNIQUE,
  actor_id TEXT NOT NULL,
  command_kind TEXT NOT NULL,
  command_json JSONB NOT NULL,
  response_json JSONB NOT NULL,
  state_sha256_after TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_trnm_world_events_world_created
  ON trnm_world_events(world_id, created_at);
CREATE INDEX IF NOT EXISTS idx_trnm_world_events_actor_created
  ON trnm_world_events(actor_id, created_at);
SQL

rm -f "$SQLITE_DB" "$QUERY_EVIDENCE"
sqlite3 "$SQLITE_DB" <"$SQLITE_SCHEMA"

STATE_SHA256="$(sha256sum "$STATE_FILE" | awk '{print $1}')"
SQLITE_SCHEMA_SHA256="$(sha256sum "$SQLITE_SCHEMA" | awk '{print $1}')"
POSTGRES_SCHEMA_SHA256="$(sha256sum "$POSTGRES_SCHEMA" | awk '{print $1}')"

sqlite3 "$SQLITE_DB" <<SQL
INSERT INTO trnm_world_repository_migrations (migration_id, checksum)
VALUES ('0001_world_repository_boundary', '$SQLITE_SCHEMA_SHA256');

INSERT INTO trnm_world_states (
  world_id,
  contract_version,
  repository_contract,
  source_of_truth,
  generation,
  state_sha256,
  state_json
) VALUES (
  'local-world',
  json_extract(CAST(readfile('$STATE_FILE') AS TEXT), '$.contract_version'),
  'trillionnium_world_repository_adapter_boundary_v1',
  'sqlite_repository_adapter_boundary_smoke',
  1,
  '$STATE_SHA256',
  CAST(readfile('$STATE_FILE') AS TEXT)
);

INSERT INTO trnm_world_events (
  event_id,
  world_id,
  idempotency_key,
  actor_id,
  command_kind,
  command_json,
  response_json,
  state_sha256_after
) VALUES (
  'repository-smoke-event-1',
  'local-world',
  'repository-smoke-local-player-east-1',
  'local-player',
  'move',
  json_object('actor_id', 'local-player', 'direction', 'east'),
  CAST(readfile('$SMOKE_FILE') AS TEXT),
  '$STATE_SHA256'
);
SQL

sqlite3 -json "$SQLITE_DB" <<'SQL' >"$QUERY_EVIDENCE"
SELECT
  world_id,
  contract_version,
  repository_contract,
  generation,
  json_extract(state_json, '$.positions[0].node_id') AS player_node_id,
  json_array_length(json_extract(state_json, '$.nodes')) AS node_count,
  (SELECT count(*) FROM trnm_world_events WHERE world_id = trnm_world_states.world_id) AS event_count
FROM trnm_world_states
WHERE world_id = 'local-world';
SQL

PLAYER_NODE="$(jq -r '.[0].player_node_id // empty' "$QUERY_EVIDENCE")"
NODE_COUNT="$(jq -r '.[0].node_count // 0' "$QUERY_EVIDENCE")"
EVENT_COUNT="$(jq -r '.[0].event_count // 0' "$QUERY_EVIDENCE")"
STATUS="repository_adapter_boundary_green"
if [[ "$PLAYER_NODE" != "starter-studio" || "$NODE_COUNT" -lt 1 || "$EVENT_COUNT" -lt 1 ]]; then
  STATUS="repository_adapter_boundary_failed"
fi

jq -n \
  --arg contract_version "trillionnium_world_repository_adapter_boundary_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg sqlite_schema "$SQLITE_SCHEMA" \
  --arg sqlite_schema_sha256 "$SQLITE_SCHEMA_SHA256" \
  --arg postgres_schema "$POSTGRES_SCHEMA" \
  --arg postgres_schema_sha256 "$POSTGRES_SCHEMA_SHA256" \
  --arg sqlite_db "$SQLITE_DB" \
  --arg state_file "$STATE_FILE" \
  --arg state_sha256 "$STATE_SHA256" \
  --arg smoke_file "$SMOKE_FILE" \
  --arg query_evidence "$QUERY_EVIDENCE" \
  --arg player_node "$PLAYER_NODE" \
  --argjson node_count "$NODE_COUNT" \
  --argjson event_count "$EVENT_COUNT" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trnm_world_repository_adapter_boundary_gate",
    public_launch_credit: "repository_schema_and_sqlite_read_write_smoke_green_not_external_managed_database",
    production_ready: false,
    accepted_public_launch_status: "repository_adapter_boundary_green",
    schemas: {
      sqlite: {
        path: $sqlite_schema,
        sha256: $sqlite_schema_sha256,
        tables: ["trnm_world_repository_migrations", "trnm_world_states", "trnm_world_events"]
      },
      postgres: {
        path: $postgres_schema,
        sha256: $postgres_schema_sha256,
        json_type: "JSONB",
        tables: ["trnm_world_repository_migrations", "trnm_world_states", "trnm_world_events"]
      }
    },
    sqlite_smoke: {
      database_path: $sqlite_db,
      state_file: $state_file,
      state_sha256: $state_sha256,
      source_smoke: $smoke_file,
      query_evidence: $query_evidence,
      player_node_id: $player_node,
      node_count: $node_count,
      event_count: $event_count,
      read_write_verified: ($status == "repository_adapter_boundary_green")
    },
    adapter_boundary: {
      current_dev_repository: "file_backed_json",
      swappable_repository_contract: "WorldRepository",
      durable_state_table: "trnm_world_states",
      durable_event_table: "trnm_world_events",
      idempotency_key: "trnm_world_events.idempotency_key",
      checksum_field: "state_sha256"
    },
    remaining_for_public_managed_database: [
      "managed_postgres_or_sqlite_target",
      "backup_restore_drill",
      "migration_rollback_drill",
      "connection_pool_and_locking_policy",
      "operator_secret_management"
    ]
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "repository_adapter_boundary_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_REPOSITORY_ADAPTER_BOUNDARY_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_REPOSITORY_ADAPTER_BOUNDARY_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
exit 1
