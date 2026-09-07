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
HOSTILE_NONUNIQUE_DB="trnm_world_nonunique_${suffix}"
HOSTILE_WRONG_KEY_DB="trnm_world_wrong_key_${suffix}"
HOSTILE_WRONG_PREDICATE_DB="trnm_world_wrong_predicate_${suffix}"
RUN="$(mktemp -d)"

cleanup() {
  for database in \
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

assert_hostile_index_rejected() {
  local database="$1"
  local label="$2"
  local create_sql="$3"

  create_db "$database"
  PGDATABASE="$database" psql -X -v ON_ERROR_STOP=1 -f "$BASE" \
    >"$RUN/${label}-base.log"
  query_db "$database" -c "$create_sql" >"$RUN/${label}-create.log"

  set +e
  PGDATABASE="$database" bash "$INSTALLER" \
    >"$RUN/${label}-install.out" 2>"$RUN/${label}-install.err"
  local status=$?
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
# add migration rows, mutate definitions, or fail.
create_db "$REAPPLY_DB"
PGDATABASE="$REAPPLY_DB" bash "$INSTALLER" >"$RUN/reapply-first.log"
first_schema_hash="$(query_db "$REAPPLY_DB" -c \
  "select public.trnm_world_sha256_text_v1(string_agg(definition, E'\\n' order by name)) from (select c.relname as name, pg_catalog.pg_get_indexdef(i.indexrelid) as definition from pg_catalog.pg_index i join pg_catalog.pg_class c on c.oid=i.indexrelid where i.indrelid in ('public.trnm_world_authority_epochs_v1'::regclass,'public.trnm_world_events_v1'::regclass,'public.trnm_world_states_v1'::regclass)) definitions;")"
PGDATABASE="$REAPPLY_DB" bash "$INSTALLER" >"$RUN/reapply-second.log"
second_schema_hash="$(query_db "$REAPPLY_DB" -c \
  "select public.trnm_world_sha256_text_v1(string_agg(definition, E'\\n' order by name)) from (select c.relname as name, pg_catalog.pg_get_indexdef(i.indexrelid) as definition from pg_catalog.pg_index i join pg_catalog.pg_class c on c.oid=i.indexrelid where i.indrelid in ('public.trnm_world_authority_epochs_v1'::regclass,'public.trnm_world_events_v1'::regclass,'public.trnm_world_states_v1'::regclass)) definitions;")"
[[ "$first_schema_hash" == "$second_schema_hash" ]] || {
  echo "World cutover bundle reapply changed protected index definitions" >&2
  exit 1
}
[[ "$(query_db "$REAPPLY_DB" -c \
  "select count(*) from public.trnm_world_authority_schema_migrations_v1 where migration_id in ('0001_world_authority_cutover_v1','0002_world_authority_cutover_v1_hardening');")" == "2" ]] || {
  echo "World cutover bundle reapply changed migration marker cardinality" >&2
  exit 1
}

# Preserve the exact healthy catalog shape as machine-readable evidence. The
# installer has already rejected every non-exact shape before this query.
query_db "$REAPPLY_DB" -c "
select pg_catalog.json_build_object(
  'schema', 'trillionnium.world.single-writer-index-catalog.v1',
  'index_oid', i.indexrelid,
  'table_oid', i.indrelid,
  'table_name', i.indrelid::regclass::text,
  'index_name', c.relname,
  'namespace', n.nspname,
  'relkind', c.relkind,
  'persistence', c.relpersistence,
  'method', am.amname,
  'unique', i.indisunique,
  'valid', i.indisvalid,
  'ready', i.indisready,
  'live', i.indislive,
  'primary', i.indisprimary,
  'exclusion', i.indisexclusion,
  'clustered', i.indisclustered,
  'replica_identity', i.indisreplident,
  'nulls_not_distinct', i.indnullsnotdistinct,
  'key_count', i.indnkeyatts,
  'attribute_count', i.indnatts,
  'first_attribute', i.indkey[0],
  'expression', pg_catalog.pg_get_expr(i.indexprs, i.indrelid, true),
  'column_definition', pg_catalog.pg_get_indexdef(i.indexrelid, 1, true),
  'predicate', pg_catalog.pg_get_expr(i.indpred, i.indrelid, true)
)::text
from pg_catalog.pg_index i
join pg_catalog.pg_class c on c.oid = i.indexrelid
join pg_catalog.pg_namespace n on n.oid = c.relnamespace
join pg_catalog.pg_am am on am.oid = c.relam
where i.indexrelid =
  'public.trnm_world_single_active_writer_epoch_v1'::regclass;
" >"$RUN/healthy-index-catalog.json"

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

# Hostile same-name objects must fail even when they copy the expected predicate.
# These cases close name-only, substring-only, uniqueness-only, and key-only
# substitution paths.
assert_hostile_index_rejected \
  "$HOSTILE_NONUNIQUE_DB" \
  "hostile-nonunique-same-predicate" \
  "create index trnm_world_single_active_writer_epoch_v1 on public.trnm_world_authority_epochs_v1 ((1)) where status = 'active' and world_writer_enabled;"

assert_hostile_index_rejected \
  "$HOSTILE_WRONG_KEY_DB" \
  "hostile-unique-wrong-key-same-predicate" \
  "create unique index trnm_world_single_active_writer_epoch_v1 on public.trnm_world_authority_epochs_v1 (epoch_id) where status = 'active' and world_writer_enabled;"

assert_hostile_index_rejected \
  "$HOSTILE_WRONG_PREDICATE_DB""À¢&†÷7F–ÆR×Væ—VRÖ6öç7FçB×w&öær×&VF–6FR"À¢&7&VFRVæ—VR–æFW‚G&æÕ÷v÷&ÆE÷6–ævÆUö7F—fU÷w&—FW%öWö6…÷cöâV&Æ–2çG&æÕ÷v÷&ÆEöWF†÷&—G•öWö6‡5÷c‚ƒ’’v†W&R7FGW2Òw&W&VBræBæ÷Bv÷&ÆE÷w&—FW%öVæ&ÆVC²  ¦6Bâ"E%Tâö–ç7FÆÆF–öâ×7VÖÖ'’æ§6öâ"ÃÄ¥4ôà§°¢'66†VÖ#¢'G&–ÆÆ–öææ—VÒçv÷&ÆBç÷7Fw&W2Ö–ç7FÆÆF–öâÖWf–FVæ6Rçc""À¢'7FGW2#¢&w&VVâ"À¢&'VæFÆU÷&VÇ•ö–FV×÷FVçB#¢G'VRÀ¢&&6UööæÇ•÷7FFU÷VçVÆ–f–VB#¢G'VRÀ¢''F–Åö&6U÷&V6÷fW&VEö'•÷7W÷'FVEö–ç7FÆÆW"#¢G'VRÀ¢&W†7Eö–æFW…ö6FÆöu÷6VÖçF–75÷fW&–f–VB#¢G'VRÀ¢'F&vWE÷&VÆF–öå÷fW&–f–VB#¢G'VRÀ¢'Væ—VU÷fÆ–E÷&VG•öÆ—fU÷fW&–f–VB#¢G'VRÀ¢&6öç7FçEöW‡&W76–öåö¶W•÷fW&–f–VB#¢G'VRÀ¢&W†7E÷'F–Å÷&VF–6FU÷fW&–f–VB#¢G'VRÀ¢&æöçVæ—VU÷6ÖU÷&VF–6FUö–æFW…÷&V¦V7FVB#¢G'VRÀ¢'Væ—VU÷w&öæuö¶W•÷6ÖU÷&VF–6FUö–æFW…÷&V¦V7FVB#¢G'VRÀ¢'Væ—VUö6öç7FçE÷w&öæu÷&VF–6FUö–æFW…÷&V¦V7FVB#¢G'VRÀ¢'w&öæu÷6ÖUöæÖUö–æFW…÷&V¦V7FVB#¢G'VRÀ¢'&öGV7F–öåöWF†÷&—¦F–öâ#¢&æ÷Eöw&çFVB §Ğ¤¥4ôà ¦–bµ²Öâ"GµE$äÕõtõ$ÄEõõ5Du$U5ôUd”DTä4UôD•#¢×Ò"ÕÓ²F†Và¢Ö¶F—"×"EE$äÕõtõ$ÄEõõ5Du$U5ôUd”DTä4UôD•" ¢7À¢"E%Tâö–ç7FÆÆF–öâ×7VÖÖ'’æ§6öâ"À¢"E%Tâö†VÇF‡’Ö–æFW‚Ö6FÆöræ§6öâ"À¢"E%Tâ÷&VÇ’Öf—'7BæÆör"À¢"E%Tâ÷&VÇ’×6V6öæBæÆör"À¢"E%Tâ÷'F–Â×&V6÷fW'’æÆör"À¢"E%Tâö†÷7F–ÆRÖæöçVæ—VR×6ÖR×&VF–6FRÖ–ç7FÆÂæW'""À¢"E%Tâö†÷7F–ÆR×Væ—VR×w&öærÖ¶W’×6ÖR×&VF–6FRÖ–ç7FÆÂæW'""À¢"E%Tâö†÷7F–ÆR×Væ—VRÖ6öç7FçB×w&öær×&VF–6FRÖ–ç7FÆÂæW'""À¢"EE$äÕõtõ$ÄEõõ5Du$U5ôUd”DTä4UôD•"ò ¦f ¦6B"E%Tâö–ç7FÆÆF–öâ×7VÖÖ'’æ§6öâ ¦V6†ò%G&–ÆÆ–öææ—VÒv÷&ÆB÷7Fw&U5Â–ç7FÆÆF–öâ&÷Fö6öÂVÆ–f–6F–öã¢ö² 