#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BASE_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1.sql"
HARDENING_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql"

: "${PGHOST:?PGHOST is required}"
: "${PGPORT:?PGPORT is required}"
: "${PGUSER:?PGUSER is required}"
: "${PGDATABASE:?PGDATABASE is required}"

for path in "$BASE_MIGRATION" "$HARDENING_MIGRATION"; do
  [[ -s "$path" ]] || {
    echo "missing World authority migration: $path" >&2
    exit 66
  }
done

psql -X -v ON_ERROR_STOP=1 -f "$BASE_MIGRATION"
psql -X -v ON_ERROR_STOP=1 -f "$HARDENING_MIGRATION"

psql -X -v ON_ERROR_STOP=1 -At <<'SQL'
DO $verify$
declare
    base_version text;
    hardening_version text;
    active_index_oid oid;
    active_index_table oid;
    active_index_namespace name;
    active_index_name name;
    active_index_kind "char";
    active_index_persistence "char";
    active_index_method name;
    active_index_unique boolean;
    active_index_valid boolean;
    active_index_ready boolean;
    active_index_live boolean;
    active_index_primary boolean;
    active_index_exclusion boolean;
    active_index_clustered boolean;
    active_index_replica_identity boolean;
    active_index_nulls_not_distinct boolean;
    active_index_key_count integer;
    active_index_attribute_count integer;
    active_index_first_attribute smallint;
    active_index_expression text;
    active_index_column_definition text;
    active_index_predicate text;
begin
    select contract_version into strict base_version
    from public.trnm_world_authority_schema_migrations_v1
    where migration_id = '0001_world_authority_cutover_v1';
    if base_version is distinct from 'trillionnium_world_durable_cutover_v1' then
        raise exception 'unexpected World base migration version: %', base_version;
    end if;

    select contract_version into strict hardening_version
    from public.trnm_world_authority_schema_migrations_v1
    where migration_id = '0002_world_authority_cutover_v1_hardening';
    if hardening_version is distinct from 'trillionnium_world_durable_cutover_v1_hardening_1' then
        raise exception 'unexpected World hardening migration version: %', hardening_version;
    end if;

    active_index_oid := pg_catalog.to_regclass(
        'public.trnm_world_single_active_writer_epoch_v1'
    );
    if active_index_oid is null then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_single_active_writer_index_missing';
    end if;

    select
        index_catalog.indrelid,
        index_namespace.nspname,
        index_relation.relname,
        index_relation.relkind,
        index_relation.relpersistence,
        access_method.amname,
        index_catalog.indisunique,
        index_catalog.indisvalid,
        index_catalog.indisready,
        index_catalog.indislive,
        index_catalog.indisprimary,
        index_catalog.indisexclusion,
        index_catalog.indisclustered,
        index_catalog.indisreplident,
        index_catalog.indnullsnotdistinct,
        index_catalog.indnkeyatts,
        index_catalog.indnatts,
        index_catalog.indkey[0],
        pg_catalog.pg_get_expr(
            index_catalog.indexprs,
            index_catalog.indrelid,
            true
        ),
        pg_catalog.pg_get_indexdef(
            index_catalog.indexrelid,
            1,
            true
        ),
        pg_catalog.pg_get_expr(
            index_catalog.indpred,
            index_catalog.indrelid,
            true
        )
    into strict
        active_index_table,
        active_index_namespace,
        active_index_name,
        active_index_kind,
        active_index_persistence,
        active_index_method,
        active_index_unique,
        active_index_valid,
        active_index_ready,
        active_index_live,
        active_index_primary,
        active_index_exclusion,
        active_index_clustered,
        active_index_replica_identity,
        active_index_nulls_not_distinct,
        active_index_key_count,
        active_index_attribute_count,
        active_index_first_attribute,
        active_index_expression,
        active_index_column_definition,
        active_index_predicate
    from pg_catalog.pg_index as index_catalog
    join pg_catalog.pg_class as index_relation
      on index_relation.oid = index_catalog.indexrelid
    join pg_catalog.pg_namespace as index_namespace
      on index_namespace.oid = index_relation.relnamespace
    join pg_catalog.pg_am as access_method
      on access_method.oid = index_relation.relam
    where index_catalog.indexrelid = active_index_oid;

    if active_index_table is distinct from
           'public.trnm_world_authority_epochs_v1'::pg_catalog.regclass
       or active_index_namespace is distinct from 'public'
       or active_index_name is distinct from
           'trnm_world_single_active_writer_epoch_v1'
       or active_index_kind is distinct from 'i'::"char"
       or active_index_persistence is distinct from 'p'::"char"
       or active_index_method is distinct from 'btree'
       or active_index_unique is distinct from true
       or active_index_valid is distinct from true
       or active_index_ready is distinct from true
       or active_index_live is distinct from true
       or active_index_primary is distinct from false
       or active_index_exclusion is distinct from false
       or active_index_clustered is distinct from false
       or active_index_replica_identity is distinct from false
       or active_index_nulls_not_distinct is distinct from false
       or active_index_key_count is distinct from 1
       or active_index_attribute_count is distinct from 1
       or active_index_first_attribute is distinct from 0
       or active_index_expression not in ('1', '(1)')
       or active_index_column_definition not in ('1', '(1)')
       or active_index_predicate not in (
           '(status = ''active''::text) AND world_writer_enabled',
           '((status = ''active''::text) AND world_writer_enabled)',
           '(status = ''active'') AND world_writer_enabled',
           '((status = ''active'') AND world_writer_enabled)'
       ) then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_single_active_writer_index_semantics_drift',
            detail = pg_catalog.format(
                'oid=%s table=%s schema=%s name=%s kind=%s persistence=%s method=%s unique=%s valid=%s ready=%s live=%s primary=%s exclusion=%s clustered=%s replident=%s nulls_not_distinct=%s key_count=%s attr_count=%s first_att=%s expression=%s column=%s predicate=%s',
                active_index_oid,
                active_index_table,
                active_index_namespace,
                active_index_name,
                active_index_kind,
                active_index_persistence,
                active_index_method,
                active_index_unique,
                active_index_valid,
                active_index_ready,
                active_index_live,
                active_index_primary,
                active_index_exclusion,
                active_index_clustered,
                active_index_replica_identity,
                active_index_nulls_not_distinct,
                active_index_key_count,
                active_index_attribute_count,
                active_index_first_attribute,
                coalesce(active_index_expression, '<null>'),
                coalesce(active_index_column_definition, '<null>'),
                coalesce(active_index_predicate, '<null>')
            );
    end if;

    if not exists (
        select 1 from pg_catalog.pg_constraint
        where conrelid = 'public.trnm_world_events_v1'::regclass
          and conname = 'trnm_world_event_identity_v1'
          and contype = 'c'
    ) then
        raise exception 'World deterministic event identity constraint missing';
    end if;
    if not exists (
        select 1 from pg_catalog.pg_constraint
        where conrelid = 'public.trnm_world_events_v1'::regclass
          and conname = 'trnm_world_event_request_hash_v1'
          and contype = 'c'
    ) then
        raise exception 'World request hash constraint missing';
    end if;
end;
$verify$;

select 'trnm_world_authority_migration_bundle_v1:ok';
SQL
