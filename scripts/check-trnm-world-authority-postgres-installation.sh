#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASE="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1.sql"
HARDENING="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql"
INSTALLER="$ROOT/scripts/apply-trnm-world-authority-cutover-v1.sh"

: "${PGHOST:=127.0.0.1}"
: "${PGPORT:=5432}"
: "${PGUSER:=postgres}"
: "${PGDATABASE:=postgres}"
export PGHOST PGPORT PGUSER PGDATABASE

for path in "$BASE" "$HARDENING" "$INSTALLER"; do
  [[ -s "$path" ]] || {
    echo "missing World PostgreSQL installation input: $path" >&2
    exit 66
  }
done

ADMIN_DB="$PGDATABASE"
suffix="${GITHUB_RUN_ID:-$$}_${RANDOM}"
suffix="${suffix//[^A-Za-z0-9_]/_}"
REAPPLY_DB="trnm_world_reapply_${suffix}"
PARTIAL_DB="trnm_world_partial_${suffix}"
HOSTILE_INDEX_DB="trnm_world_hostile_index_${suffix}"
RUN="$(mktemp -d)"

cleanup() {
  for database in "$HOSTILE_INDEX_DB" "$PARTIAL_DB" "$REAPPLY_DB"; do
    PGDATABASE="$ADMIN_DB" dropdb --if-exists --force "$database" >/dev/null 2>&1 || true
  done
  rm -rf "$RUN"
}
trap cleanup EXIT

create_db() {
  PGDATABASE="$ADMIN_DB" createdb "$1"
}

query_db() {
  local database="$1"
  shift
  PGDATABASE="$database" psql -X -v ON_ERROR_STOP=1 -At "$@"
}

# The supported bundle is idempotent as a whole. A second application must not
# add migration rows, mutate definitions, or fail.
create_db "$REAPPLY_DB"
PGDATABASE="$REAPPLY_DB" bash "$INSTALLER" >"$RUN/reapply-first.log"
first_schema_hash="$(query_db "$REAPPLY_DB" -c "select public.trnm_world_sha256_text_v1(string_agg(definition, E'\\n' order by name)) from (select c.relname as name, pg_catalog.pg_get_indexdef(i.indexrelid) as definition from pg_catalog.pg_index i join pg_catalog.pg_class c on c.oid=i.indexrelid where i.indrelid in ('public.trnm_world_authority_epochs_v1'::regclass,'public.trnm_world_events_v1'::regclass,'public.trnm_world_states_v1'::regclass)) definitions;")"
PGDATABASE="$REAPPLY_DB" bash "$INSTALLER" >"$RUN/reapply-second.log"
second_schema_hash="$(query_db "$REAPPLY_DB" -c "select public.trnm_world_sha256_text_v1(string_agg(definition, E'\\n' order by name)) from (select c.relname as name, pg_catalog.pg_get_indexdef(i.indexrelid) as definition from pg_catalog.pg_index i join pg_catalog.pg_class c on c.oid=i.indexrelid where i.indrelid in ('public.trnm_world_authority_epochs_v1'::regclass,'public.trnm_world_events_v1'::regclass,'public.trnm_world_states_v1'::regclass)) definitions;")"
[[ "$first_schema_hash" == "$second_schema_hash" ]] || {
  echo "World cutover bundle reapply changed protected index definitions" >&2
  exit 1
}
[[ "$(query_db "$REAPPLY_DB" -c "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id in ('0001_world_authority_cutover_v1','0002_world_authority_cutover_v1_hardening');")" == "2" ]] || {
  echo "World cutover bundle reapply changed migration marker cardinality" >&2
  exit 1
}

# A base-only database is explicitly unqualified. The supported installer may
# recover it by applying the mandatory hardening and must then verify both exact
# markers.
create_db "$PARTIAL_DB"
PGDATABASE="$PARTIAL_DB" psql -X -v ON_ERROR_STOP=1 -f "$BASE" >"$RUN/partial-base.log"
[[ "$(query_db "$PARTIAL_DB" -c "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id='0001_world_authority_cutover_v1';")" == "1" ]]
[[ "$(query_db "$PARTIAL_DB" -c "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id='0002_world_authority_cutover_v1_hardening';")" == "0" ]]
[[ "$(query_db "$PARTIAL_DB" -c "select count(*) from pg_catalog.pg_class where oid=to_regclass('public.trnm_world_single_active_writer_epoch_v1');")" == "0" ]]
PGDATABASE="$PARTIAL_DB" bash "$INSTALLER" >"$RUN/partial-recovery.log"
[[ "$(query_db "$PARTIAL_DB" -c "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id in ('0001_world_authority_cutover_v1','0002_world_authority_cutover_v1_hardening');")" == "2" ]]

# A same-name index with a different expression/predicate must never be accepted
# as the single-writer control. The hardening file is name-idempotent, so the
# supported installer performs a semantic pg_get_indexdef check and exits
# nonzero before returning qualification to its caller.
create_db "$HOSTILE_INDEX_DB"
PGDATABASE="$HOSTILE_INDEX_DB" psql -X -v ON_ERROR_STOP=1 -f "$BASE" >"$RUN/hostile-index-base.log"
query_db "$HOSTILE_INDEX_DB" -c "create unique index trnm_world_single_active_writer_epoch_v1 on public.trnm_world_authority_epochs_v1 (epoch_id);" >"$RUN/hostile-index-create.log"
set +e
PGDATABASE="$HOSTILE_INDEX_DB" bash "$INSTALLER" >"$RUN/hostile-index-install.out" 2>"$RUN/hostile-index-install.err"
hostile_status=$?
set -e
if [[ "$hostile_status" -eq 0 ]]; then
  echo "supported installer accepted a wrong same-name single-writer index" >&2
  exit 1
fi
if ! grep -F 'single-active-writer partial index predicate drift' "$RUN/hostile-index-install.err" >/dev/null; then
  echo "supported installer rejected the hostile index for an unexpected reason" >&2
  cat "$RUN/hostile-index-install.err" >&2 || true
  exit 1
fi

cat >"$RUN/installation-summary.json" <<JSON
{
  "schema": "trillionnium.world.postgres-installation-evidence.v1",
  "status": "green",
  "bundle_reapply_idempotent": true,
  "base_only_state_unqualified": true,
  "partial_base_recovered_by_supported_installer": true,
  "wrong_same_name_index_rejected": true,
  "production_authorization": "not_granted"
}
JSON

if [[ -n "${TRNM_WORLD_POSTGRES_EVIDENCE_DIR:-}" ]]; then
  mkdir -p "$TRNM_WORLD_POSTGRES_EVIDENCE_DIR"
  cp "$RUN/installation-summary.json" "$RUN/reapply-first.log" \
    "$RUN/reapply-second.log" "$RUN/partial-recovery.log" \
    "$RUN/hostile-index-install.err" "$TRNM_WORLD_POSTGRES_EVIDENCE_DIR/"
fi

cat "$RUN/installation-summary.json"
echo "Trillionnium World PostgreSQL installation protocol qualification: ok"
