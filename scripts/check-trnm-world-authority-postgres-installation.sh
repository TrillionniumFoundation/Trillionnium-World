#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASE="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1.sql"
HARDENING="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql"
INSTALLER="$ROOT/scripts/apply-trnm-world-authority-cutover-v1.sh"
STATE_CHECKER="$ROOT/scripts/check-trnm-world-authority-postgres.sh"
RUNNER="$ROOT/scripts/run-trnm-world-authority-postgres-check.sh"

: "${PGHOST:=127.0.0.1}"
: "${PGPORT:=5432}"
: "${PGUSER:=postgres}"
: "${PGDATABASE:=postgres}"
export PGHOST PGPORT PGUSER PGDATABASE

for path in "$BASE" "$HARDENING" "$INSTALLER" "$STATE_CHECKER" "$RUNNER"; do
  [[ -s "$path" ]] || {
    echo "missing World PostgreSQL installation input: $path" >&2
    exit 66
  }
done

# Fail closed on encoding and shell-source integrity before any database is
# created. This prevents a truncated or non-UTF-8 hostile matrix from being
# silently treated as executable qualification source.
python3 - "$0" "$BASE" "$HARDENING" "$INSTALLER" "$STATE_CHECKER" "$RUNNER" <<'PY'
from __future__ import annotations

import pathlib
import sys

for raw in sys.argv[1:]:
    path = pathlib.Path(raw)
    data = path.read_bytes()
    if b"\x00" in data:
        raise SystemExit(f"NUL byte forbidden in World PostgreSQL qualification input: {path}")
    try:
        text = data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise SystemExit(f"non-UTF-8 World PostgreSQL qualification input: {path}: {error}") from error
    if not text.endswith("\n"):
        raise SystemExit(f"World PostgreSQL qualification input lacks final newline: {path}")
PY
bash -n "$0" "$INSTALLER" "$STATE_CHECKER" "$RUNNER"

ADMIN_DB="$PGDATABASE"
suffix="${GITHUB_RUN_ID:-$$}_${RANDOM}"
suffix="${suffix//[^A-Za-z0-9_]/_}"
REAPPLY_DB="trnm_world_reapply_${suffix}"
PARTIAL_DB="trnm_world_partial_${suffix}"
HOSTILE_NONUNIQUE_DB="trnm_world_nonunique_${suffix}"
HOSTILE_WRONG_KEY_DB="trnm_world_wrong_key_${suffix}"
HOSTILE_WRONG_PREDICATE_DB="trnm_world_wrong_predicate_${suffix}"
HOSTILE_NO_PREDICATE_DB="trnm_world_no_predicate_${suffix}"
RUN="$(mktemp -d)"

cleanup() {
  for database in \
    "$HOSTILE_NO_PREDICATE_DB" \
    "$HOSTILE_WRONG_PREDICATE_DB" \
    "$HOSTILE_WRONG_KEY_DB" \
    "$HOSTILE_NONUNIQUE_DB" \
    "$PARTIAL_DB" \
    "$REAPPLY_DB"; do
    PGDATABASE="$ADMIN_DB" dropdb --if-exists --force "$database" \
      >/dev/null 2>&1 || true
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

protected_schema_hash() {
  local database="$1"
  query_db "$database" -c "
select public.trnm_world_sha256_text_v1(
  string_agg(definition, E'\\n' order by name)
)
from (
  select
    c.relname as name,
    pg_catalog.pg_get_indexdef(i.indexrelid) as definition
  from pg_catalog.pg_index i
  join pg_catalog.pg_class c on c.oid = i.indexrelid
  where i.indrelid in (
    'public.trnm_world_authority_epochs_v1'::regclass,
    'public.trnm_world_events_v1'::regclass,
    'public.trnm_world_states_v1'::regclass
  )
) definitions;"
}

write_healthy_catalog_evidence() {
  local database="$1"
  query_db "$database" -c "
select pg_catalog.json_build_object(
  'schema', 'trillionnium.world.single-writer-index-catalog.v2',
  'index_oid', i.indexrelid,
  'table_oid', i.indrelid,
  'table_name', i.indrelid::regclass::text,
  'index_name', c.relname,
  'namespace', n.nspname,
  'relkind', c.relkind,
  'persistence', c.relpersistence,
  'method', am.amname,
  'unique', i.indisunique,
  'immediate', i.indimmediate,
  'valid', i.indisvalid,
  'ready', i.indisready,
  'live', i.indislive,
  'check_xmin', i.indcheckxmin,
  'primary', i.indisprimary,
  'exclusion', i.indisexclusion,
  'clustered', i.indisclustered,
  'replica_identity', i.indisreplident,
  'nulls_not_distinct', i.indnullsnotdistinct,
  'key_count', i.indnkeyatts,
  'attribute_count', i.indnatts,
  'first_attribute', i.indkey[0],
  'collation_oid', i.indcollation[0],
  'index_option', i.indoption[0],
  'opclass_namespace', opcn.nspname,
  'opclass_name', opc.opcname,
  'opclass_input_type', opc.opcintype,
  'opclass_default', opc.opcdefault,
  'opclass_key_type', opc.opckeytype,
  'opfamily_namespace', opfn.nspname,
  'opfamily_name', opf.opfname,
  'expression', pg_catalog.pg_get_expr(i.indexprs, i.indrelid, true),
  'column_definition', pg_catalog.pg_get_indexdef(i.indexrelid, 1, true),
  'predicate', pg_catalog.pg_get_expr(i.indpred, i.indrelid, true)
)::text
from pg_catalog.pg_index i
join pg_catalog.pg_class c on c.oid = i.indexrelid
join pg_catalog.pg_namespace n on n.oid = c.relnamespace
join pg_catalog.pg_am am on am.oid = c.relam
join pg_catalog.pg_opclass opc on opc.oid = i.indclass[0] and opc.opcmethod = am.oid
join pg_catalog.pg_namespace opcn on opcn.oid = opc.opcnamespace
join pg_catalog.pg_opfamily opf on opf.oid = opc.opcfamily and opf.opfmethod = am.oid
join pg_catalog.pg_namespace opfn on opfn.oid = opf.opfnamespace
where i.indexrelid =
  'public.trnm_world_single_active_writer_epoch_v1'::regclass;" \
    >"$RUN/healthy-index-catalog.json"
}

assert_hostile_index_rejected() {
  local database="$1"
  local label="$2"
  local create_sql="$3"
  local status

  create_db "$database"
  PGDATABASE="$database" psql -X -v ON_ERROR_STOP=1 -f "$BASE" \
    >"$RUN/${label}-base.log"
  query_db "$database" -c "$create_sql" >"$RUN/${label}-create.log"

  set +e
  PGDATABASE="$database" bash "$INSTALLER" \
    >"$RUN/${label}-install.out" 2>"$RUN/${label}-install.err"
  status=$?
  set -e

  if [[ "$status" -eq 0 ]]; then
    echo "supported installer accepted hostile single-writer index: $label" >&2
    exit 1
  fi
  if ! grep -F \
    'trnm_world_single_active_writer_index_semantics_drift' \
    "$RUN/${label}-install.err" >/dev/null; then
    echo "supported installer rejected $label for an unexpected reason" >&2
    cat "$RUN/${label}-install.err" >&2 || true
    exit 1
  fi
  if [[ "$(query_db "$database" -c \
    "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id='0002_world_authority_cutover_v1_hardening';")" != "1" ]]; then
    echo "hostile fixture did not reach post-hardening catalog verification: $label" >&2
    exit 1
  fi
}

# The supported bundle is idempotent as a whole. A second application must not
# add migration rows, mutate protected definitions, or fail.
create_db "$REAPPLY_DB"
PGDATABASE="$REAPPLY_DB" bash "$INSTALLER" >"$RUN/reapply-first.log"
first_schema_hash="$(protected_schema_hash "$REAPPLY_DB")"
PGDATABASE="$REAPPLY_DB" bash "$INSTALLER" >"$RUN/reapply-second.log"
second_schema_hash="$(protected_schema_hash "$REAPPLY_DB")"
[[ "$first_schema_hash" == "$second_schema_hash" ]] || {
  echo "World cutover bundle reapply changed protected index definitions" >&2
  exit 1
}
[[ "$(query_db "$REAPPLY_DB" -c \
  "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id in ('0001_world_authority_cutover_v1','0002_world_authority_cutover_v1_hardening');")" == "2" ]] || {
  echo "World cutover bundle reapply changed migration marker cardinality" >&2
  exit 1
}
write_healthy_catalog_evidence "$REAPPLY_DB"

# A base-only database is explicitly unqualified. The supported installer may
# recover it by applying mandatory hardening and exact catalog verification.
create_db "$PARTIAL_DB"
PGDATABASE="$PARTIAL_DB" psql -X -v ON_ERROR_STOP=1 -f "$BASE" \
  >"$RUN/partial-base.log"
[[ "$(query_db "$PARTIAL_DB" -c \
  "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id='0001_world_authority_cutover_v1';")" == "1" ]]
[[ "$(query_db "$PARTIAL_DB" -c \
  "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id='0002_world_authority_cutover_v1_hardening';")" == "0" ]]
[[ "$(query_db "$PARTIAL_DB" -c \
  "select count(*) from pg_catalog.pg_class where oid=to_regclass('public.trnm_world_single_active_writer_epoch_v1');")" == "0" ]]
PGDATABASE="$PARTIAL_DB" bash "$INSTALLER" >"$RUN/partial-recovery.log"
[[ "$(query_db "$PARTIAL_DB" -c \
  "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id in ('0001_world_authority_cutover_v1','0002_world_authority_cutover_v1_hardening');")" == "2" ]]

# Hostile same-name objects close uniqueness-, key-, predicate-, and SQL-NULL
# substitution paths. The final case is especially important: pg_get_expr()
# returns SQL NULL for an index without a predicate, which must be rejected
# explicitly rather than being allowed through three-valued boolean logic.
assert_hostile_index_rejected \
  "$HOSTILE_NONUNIQUE_DB" \
  "hostile-nonunique-same-predicate" \
  "create index trnm_world_single_active_writer_epoch_v1 on public.trnm_world_authority_epochs_v1 ((1)) where status = 'active' and world_writer_enabled;"

assert_hostile_index_rejected \
  "$HOSTILE_WRONG_KEY_DB" \
  "hostile-unique-wrong-key-same-predicate" \
  "create unique index trnm_world_single_active_writer_epoch_v1 on public.trnm_world_authority_epochs_v1 (epoch_id) where status = 'active' and world_writer_enabled;"

assert_hostile_index_rejected \
  "$HOSTILE_WRONG_PREDICATE_DB" \
  "hostile-unique-constant-wrong-predicate" \
  "create unique index trnm_world_single_active_writer_epoch_v1 on public.trnm_world_authority_epochs_v1 ((1)) where status = 'prepared' and not world_writer_enabled;"

assert_hostile_index_rejected \
  "$HOSTILE_NO_PREDICATE_DB" \
  "hostile-unique-constant-no-predicate" \
  "create unique index trnm_world_single_active_writer_epoch_v1 on public.trnm_world_authority_epochs_v1 ((1));"

python3 - "$RUN/installation-summary.json" "$first_schema_hash" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
path.write_text(
    json.dumps(
        {
            "schema": "trillionnium.world.postgres-installation-evidence.v3",
            "status": "green",
            "bundle_reapply_idempotent": True,
            "protected_schema_sha256": sys.argv[2],
            "base_only_state_unqualified": True,
            "partial_base_recovered_by_supported_installer": True,
            "single_writer_index_catalog_semantics_exact": True,
            "single_writer_index_nullable_deparse_rejected": True,
            "single_writer_index_builtin_int4_btree_opclass_bound": True,
            "same_name_nonunique_same_predicate_rejected": True,
            "same_name_unique_wrong_key_same_predicate_rejected": True,
            "same_name_unique_constant_wrong_predicate_rejected": True,
            "same_name_unique_constant_no_predicate_rejected": True,
            "hostile_index_fixture_count": 4,
            "production_authorization": "not_granted",
        },
        indent=2,
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
PY

if [[ -n "${TRNM_WORLD_POSTGRES_EVIDENCE_DIR:-}" ]]; then
  mkdir -p "$TRNM_WORLD_POSTGRES_EVIDENCE_DIR"
  find "$RUN" -maxdepth 1 -type f -exec cp {} "$TRNM_WORLD_POSTGRES_EVIDENCE_DIR/" \;
fi

cat "$RUN/installation-summary.json"
echo "Trillionnium World PostgreSQL installation protocol qualification: ok"
