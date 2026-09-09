#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASE_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1.sql"
HARDENING_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql"
INSTALLER="$ROOT/scripts/apply-trnm-world-authority-cutover-v1.sh"

: "${PGHOST:=127.0.0.1}"
: "${PGPORT:=5432}"
: "${PGUSER:=postgres}"
: "${PGDATABASE:=postgres}"
export PGHOST PGPORT PGUSER PGDATABASE

for command in psql createdb dropdb pg_dump pg_restore; do
  command -v "$command" >/dev/null || {
    echo "missing PostgreSQL client command: $command" >&2
    exit 69
  }
done
for path in "$BASE_MIGRATION" "$HARDENING_MIGRATION" "$INSTALLER"; do
  [[ -s "$path" ]] || {
    echo "missing World PostgreSQL qualification input: $path" >&2
    exit 66
  }
done

RUN="$(mktemp -d)"
suffix="${GITHUB_RUN_ID:-$$}_${RANDOM}"
suffix="${suffix//[^A-Za-z0-9_]/_}"
TEST_DB="trnm_world_${suffix}"
CONFLICT_DB="trnm_world_conflict_${suffix}"
RESTORE_DB="trnm_world_restore_${suffix}"
ADMIN_DB="$PGDATABASE"

cleanup() {
  PGDATABASE="$ADMIN_DB" dropdb --if-exists --force "$RESTORE_DB" >/dev/null 2>&1 || true
  PGDATABASE="$ADMIN_DB" dropdb --if-exists --force "$CONFLICT_DB" >/dev/null 2>&1 || true
  PGDATABASE="$ADMIN_DB" dropdb --if-exists --force "$TEST_DB" >/dev/null 2>&1 || true
  rm -rf "$RUN"
}
trap cleanup EXIT

admin_createdb() {
  PGDATABASE="$ADMIN_DB" createdb "$1"
}

db_psql() {
  PGDATABASE="$TEST_DB" psql -X -v ON_ERROR_STOP=1 "$@"
}

db_query() {
  db_psql -At "$@"
}

expect_failure_contains() {
  local label="$1"
  local pattern="$2"
  shift 2
  set +e
  "$@" >"$RUN/${label}.out" 2>"$RUN/${label}.err"
  local status=$?
  set -e
  if [[ "$status" -eq 0 ]]; then
    echo "hostile PostgreSQL fixture unexpectedly succeeded: $label" >&2
    cat "$RUN/${label}.out" >&2 || true
    exit 1
  fi
  if ! grep -F "$pattern" "$RUN/${label}.err" >/dev/null; then
    echo "hostile PostgreSQL fixture failed for the wrong reason: $label" >&2
    cat "$RUN/${label}.err" >&2 || true
    exit 1
  fi
  echo "hostile PostgreSQL fixture rejected: $label"
}

admin_createdb "$TEST_DB"
PGDATABASE="$TEST_DB" bash "$INSTALLER" >"$RUN/install.log"

EPOCH1="sha256:$(printf '1%.0s' {1..64})"
EPOCH2="sha256:$(printf '2%.0s' {1..64})"
EPOCH3="sha256:$(printf '3%.0s' {1..64})"
BATCH1="sha256:$(printf 'a%.0s' {1..64})"
BATCH2="sha256:$(printf 'b%.0s' {1..64})"
BATCH3="sha256:$(printf 'c%.0s' {1..64})"
CEX_SHA="$(printf 'd%.0s' {1..40})"
WORLD_SHA="$(printf 'e%.0s' {1..40})"
FENCE1="sha256:$(printf 'f%.0s' {1..64})"
FENCE2="sha256:$(printf '0%.0s' {1..64})"
ROLLBACK1="sha256:$(printf '9%.0s' {1..64})"

# Prepare two independently reconciled epochs. They may coexist in prepared
# state, but the database must never permit both to become active writers.
db_psql \
  -v epoch1="$EPOCH1" -v epoch2="$EPOCH2" \
  -v batch1="$BATCH1" -v batch2="$BATCH2" \
  -v cex_sha="$CEX_SHA" -v world_sha="$WORLD_SHA" \
  -v fence1="$FENCE1" -v fence2="$FENCE2" <<'SQL' >"$RUN/prepare.log"
select public.trnm_world_stage_migration_batch_v1(
    :'epoch1', :'batch1', :'cex_sha', :'world_sha', 1,
    public.trnm_world_sha256_text_v1(
        'world-1|' || public.trnm_world_sha256_text_v1(
            '{"node": "start", "version": 0}'::jsonb::text
        )
    )
);
select public.trnm_world_import_snapshot_v1(
    :'batch1', 'world-1', 'trillionnium_world_state_v1', 'cex:state:1',
    '{"node": "start", "version": 0}'::jsonb::text,
    public.trnm_world_sha256_text_v1(
        '{"node": "start", "version": 0}'::jsonb::text
    )
);
select public.trnm_world_reconcile_migration_batch_v1(:'batch1');
select public.trnm_world_record_cex_writer_fence_v1(:'epoch1', :'fence1');

select public.trnm_world_stage_migration_batch_v1(
    :'epoch2', :'batch2', :'cex_sha', :'world_sha', 1,
    public.trnm_world_sha256_text_v1(
        'world-2|' || public.trnm_world_sha256_text_v1(
            '{"node": "start", "version": 0}'::jsonb::text
        )
    )
);
select public.trnm_world_import_snapshot_v1(
    :'batch2', 'world-2', 'trillionnium_world_state_v1', 'cex:state:2',
    '{"node": "start", "version": 0}'::jsonb::text,
    public.trnm_world_sha256_text_v1(
        '{"node": "start", "version": 0}'::jsonb::text
    )
);
select public.trnm_world_reconcile_migration_batch_v1(:'batch2');
select public.trnm_world_record_cex_writer_fence_v1(:'epoch2', :'fence2');
SQL

# Hold the global activation advisory lock in session A after activating epoch1.
# Session B targets a different row and must still fail after waiting.
cat >"$RUN/activate-a.sql" <<SQL
begin;
select public.trnm_world_activate_writer_v1('$EPOCH1');
\! touch '$RUN/activation-lock-held'
select pg_catalog.pg_sleep(2);
commit;
SQL
PGDATABASE="$TEST_DB" psql -X -v ON_ERROR_STOP=1 -f "$RUN/activate-a.sql" \
  >"$RUN/activate-a.out" 2>"$RUN/activate-a.err" &
activate_a_pid=$!
for _ in $(seq 1 100); do
  [[ -f "$RUN/activation-lock-held" ]] && break
  if ! kill -0 "$activate_a_pid" 2>/dev/null; then
    break
  fi
  sleep 0.05
done
[[ -f "$RUN/activation-lock-held" ]] || {
  wait "$activate_a_pid" || true
  cat "$RUN/activate-a.err" >&2 || true
  echo "activation concurrency fixture did not acquire the global writer lock" >&2
  exit 1
}
expect_failure_contains \
  concurrent_second_active_epoch \
  trnm_world_different_active_writer_epoch_exists \
  env PGDATABASE="$TEST_DB" psql -X -v ON_ERROR_STOP=1 -Atc \
  "select public.trnm_world_activate_writer_v1('$EPOCH2');"
wait "$activate_a_pid"

active_epoch="$(db_query -c "select epoch_id from public.trnm_world_authority_epochs_v1 where status='active' and world_writer_enabled;")"
[[ "$active_epoch" == "$EPOCH1" ]] || {
  echo "unexpected active World writer epoch: $active_epoch" >&2
  exit 1
}
active_count="$(db_query -c "select count(*) from public.trnm_world_authority_epochs_v1 where status='active' and world_writer_enabled;")"
[[ "$active_count" == "1" ]] || {
  echo "single active World writer invariant failed: $active_count" >&2
  exit 1
}

COMMAND="$(db_query -c "select '{\"direction\":\"east\",\"kind\":\"move\"}'::jsonb::text;")"
RESPONSE="$(db_query -c "select '{\"accepted\":true}'::jsonb::text;")"
STATE_AFTER="$(db_query -c "select '{\"node\":\"east\",\"version\":1}'::jsonb::text;")"
REQUEST_HASH="$(db_query -v value="$COMMAND" -c "select public.trnm_world_sha256_text_v1(:'value');")"
RESPONSE_HASH="$(db_query -v value="$RESPONSE" -c "select public.trnm_world_sha256_text_v1(:'value');")"
STATE_BEFORE_HASH="$(db_query -c "select state_sha256 from public.trnm_world_states_v1 where world_id='world-1';")"
STATE_AFTER_HASH="$(db_query -v value="$STATE_AFTER" -c "select public.trnm_world_sha256_text_v1(:'value');")"
EVENT_ID="$(db_query \
  -v epoch="$EPOCH1" -v request_hash="$REQUEST_HASH" \
  -c "select public.trnm_world_event_id_v1(:'epoch', 'world-1', 'concurrent-replay-1', :'request_hash');")"

run_event() {
  local output="$1"
  PGDATABASE="$TEST_DB" psql -X -v ON_ERROR_STOP=1 -At \
    -v event_id="$EVENT_ID" \
    -v epoch="$EPOCH1" \
    -v request_hash="$REQUEST_HASH" \
    -v command="$COMMAND" \
    -v response="$RESPONSE" \
    -v response_hash="$RESPONSE_HASH" \
    -v state_before="$STATE_BEFORE_HASH" \
    -v state_after="$STATE_AFTER" \
    -v state_after_hash="$STATE_AFTER_HASH" \
    -c "select public.trnm_world_apply_event_v1(
          :'event_id', :'epoch', 'world-1', 'concurrent-replay-1',
          :'request_hash', :'command', :'response', :'response_hash',
          :'state_before', :'state_after', :'state_after_hash'
        );" >"$output" 2>&1
}

run_event "$RUN/replay-a.out" & replay_a_pid=$!
run_event "$RUN/replay-b.out" & replay_b_pid=$!
wait "$replay_a_pid"
wait "$replay_b_pid"
cat "$RUN/replay-a.out" "$RUN/replay-b.out" >"$RUN/replay-combined.out"
applied_count="$(grep -o '"status": "applied"' "$RUN/replay-combined.out" | wc -l | tr -d ' ')"
replayed_count="$(grep -o '"status": "replayed_exact"' "$RUN/replay-combined.out" | wc -l | tr -d ' ')"
[[ "$applied_count" == "1" && "$replayed_count" == "1" ]] || {
  cat "$RUN/replay-combined.out" >&2
  echo "concurrent exact replay did not produce one applied and one replayed_exact result" >&2
  exit 1
}
[[ "$(db_query -c "select count(*) from public.trnm_world_events_v1 where world_id='world-1';")" == "1" ]]
[[ "$(db_query -c "select state_generation from public.trnm_world_states_v1 where world_id='world-1';")" == "1" ]]

# Equivalent-but-noncanonical JSON must be rejected rather than acquiring a
# distinct hash/identity. Stage a third batch solely for this hostile case.
db_psql -v epoch="$EPOCH3" -v batch="$BATCH3" -v cex_sha="$CEX_SHA" -v world_sha="$WORLD_SHA" <<'SQL' >/dev/null
select public.trnm_world_stage_migration_batch_v1(
    :'epoch', :'batch', :'cex_sha', :'world_sha', 1,
    'sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'
);
SQL
expect_failure_contains \
  noncanonical_snapshot_json \
  trnm_world_import_state_json_not_canonical \
  env PGDATABASE="$TEST_DB" psql -X -v ON_ERROR_STOP=1 -At \
  -v batch="$BATCH3" \
  -c "select public.trnm_world_import_snapshot_v1(
        :'batch', 'world-3', 'trillionnium_world_state_v1', 'cex:state:3',
        '{\"b\":2, \"a\":1}',
        'sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'
      );"
expect_failure_contains \
  noncanonical_command_json \
  trnm_world_event_command_json_not_canonical \
  env PGDATABASE="$TEST_DB" psql -X -v ON_ERROR_STOP=1 -At \
  -c "select public.trnm_world_apply_event_v1(
        'sha256:0000000000000000000000000000000000000000000000000000000000000000',
        '$EPOCH1', 'world-1', 'noncanonical-command-1',
        'sha256:0000000000000000000000000000000000000000000000000000000000000000',
        '{\"kind\":\"move\", \"direction\":\"west\"}',
        '{\"accepted\": true}',
        'sha256:0000000000000000000000000000000000000000000000000000000000000000',
        '$STATE_AFTER_HASH',
        '{\"node\": \"west\", \"version\": 2}',
        'sha256:0000000000000000000000000000000000000000000000000000000000000000'
      );"

# Rollback is terminal and prevents both new mutations and implicit local
# success while retaining append-only event/receipt evidence.
db_query -v epoch="$EPOCH1" -v receipt="$ROLLBACK1" \
  -c "select public.trnm_world_rollback_writer_v1(:'epoch', :'receipt');" \
  >"$RUN/rollback.out"
[[ "$(db_query -c "select count(*) from public.trnm_world_authority_epochs_v1 where status='active' and world_writer_enabled;")" == "0" ]]
expect_failure_contains \
  mutation_after_rollback \
  trnm_world_writer_not_active_or_dual_writer_risk \
  env PGDATABASE="$TEST_DB" psql -X -v ON_ERROR_STOP=1 -At \
  -v event_id="$EVENT_ID" -v epoch="$EPOCH1" \
  -v request_hash="$REQUEST_HASH" -v command="$COMMAND" \
  -v response="$RESPONSE" -v response_hash="$RESPONSE_HASH" \
  -v state_before="$STATE_BEFORE_HASH" -v state_after="$STATE_AFTER" \
  -v state_after_hash="$STATE_AFTER_HASH" \
  -c "select public.trnm_world_apply_event_v1(
        :'event_id', :'epoch', 'world-1', 'concurrent-replay-1',
        :'request_hash', :'command', :'response', :'response_hash',
        :'state_before', :'state_after', :'state_after_hash'
      );"

# The base migration's historical upsert shape is not trusted alone. The
# mandatory hardening migration must detect a pre-existing incompatible marker.
admin_createdb "$CONFLICT_DB"
PGDATABASE="$CONFLICT_DB" psql -X -v ON_ERROR_STOP=1 <<'SQL' >/dev/null
create table public.trnm_world_authority_schema_migrations_v1 (
    migration_id text primary key,
    contract_version text not null,
    applied_at timestamptz not null default transaction_timestamp(),
    check (length(migration_id) between 1 and 128),
    check (length(contract_version) between 1 and 128)
);
insert into public.trnm_world_authority_schema_migrations_v1 (
    migration_id, contract_version
) values (
    '0001_world_authority_cutover_v1',
    'hostile_incompatible_version'
);
SQL
PGDATABASE="$CONFLICT_DB" psql -X -v ON_ERROR_STOP=1 -f "$BASE_MIGRATION" \
  >"$RUN/conflict-base.out" 2>"$RUN/conflict-base.err"
expect_failure_contains \
  migration_version_conflict \
  trnm_world_base_migration_version_conflict \
  env PGDATABASE="$CONFLICT_DB" psql -X -v ON_ERROR_STOP=1 -f "$HARDENING_MIGRATION"

# Cold backup/restore must retain the exact markers, event evidence and disabled
# writer state. This is repository-level DR evidence, not a production drill.
PGDATABASE="$TEST_DB" pg_dump -Fc -f "$RUN/world-authority.dump"
admin_createdb "$RESTORE_DB"
PGDATABASE="$RESTORE_DB" pg_restore --no-owner --no-privileges "$RUN/world-authority.dump"
PGDATABASE="$RESTORE_DB" psql -X -v ON_ERROR_STOP=1 -At <<'SQL' >"$RUN/restore-verification.out"
DO $verify$
begin
    if (select count(*) from public.trnm_world_authority_schema_migrations_v1
        where (migration_id, contract_version) in (
            ('0001_world_authority_cutover_v1', 'trillionnium_world_durable_cutover_v1'),
            ('0002_world_authority_cutover_v1_hardening', 'trillionnium_world_durable_cutover_v1_hardening_1')
        )) <> 2 then
        raise exception 'restored migration marker mismatch';
    end if;
    if (select count(*) from public.trnm_world_events_v1 where world_id = 'world-1') <> 1 then
        raise exception 'restored event count mismatch';
    end if;
    if (select count(*) from public.trnm_world_authority_epochs_v1
        where status = 'active' and world_writer_enabled) <> 0 then
        raise exception 'restored database re-enabled a writer';
    end if;
    if not exists (
        select 1 from pg_catalog.pg_index
        where indexrelid = 'public.trnm_world_single_active_writer_epoch_v1'::regclass
          and indisunique
    ) then
        raise exception 'restored single-active-writer index missing';
    end if;
end;
$verify$;
select 'backup_restore:ok';
SQL

set +e
PGHOST=127.0.0.1 PGPORT=1 PGDATABASE="$TEST_DB" PGCONNECT_TIMEOUT=1 \
  psql -X -Atc 'select 1' >"$RUN/outage.out" 2>"$RUN/outage.err"
outage_status=$?
set -e
[[ "$outage_status" -ne 0 ]] || {
  echo "database outage fixture unexpectedly connected" >&2
  exit 1
}

db_query -c "select pg_catalog.jsonb_build_object(
    'schema', 'trillionnium.world.postgres-cutover-evidence.v1',
    'status', 'green',
    'single_active_epoch_enforced', true,
    'concurrent_exact_replay', true,
    'canonical_json_enforced', true,
    'migration_conflict_fail_closed', true,
    'rollback_disables_writer', true,
    'cold_backup_restore', true,
    'outage_fail_closed', true,
    'production_authorization', 'not_granted'
);" | tee "$RUN/summary.json"

if [[ -n "${TRNM_WORLD_POSTGRES_EVIDENCE_DIR:-}" ]]; then
  mkdir -p "$TRNM_WORLD_POSTGRES_EVIDENCE_DIR"
  cp "$RUN/summary.json" "$RUN/replay-a.out" "$RUN/replay-b.out" \
    "$RUN/restore-verification.out" "$TRNM_WORLD_POSTGRES_EVIDENCE_DIR/"
  sha256sum "$TRNM_WORLD_POSTGRES_EVIDENCE_DIR"/* \
    >"$TRNM_WORLD_POSTGRES_EVIDENCE_DIR/sha256sums.txt"
fi

echo "Trillionnium World PostgreSQL cutover qualification: ok"
