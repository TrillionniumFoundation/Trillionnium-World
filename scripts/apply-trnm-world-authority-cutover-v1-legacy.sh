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
    idx record;
    expression_normalized text;
    column_normalized text;
    predicate_normalized text;
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
        i.indrelid as index_table,
        n.nspname as index_namespace,
        c.relname as index_name,
        c.relkind as index_kind,
        c.relpersistence as index_persistence,
        am.amname as index_method,
        i.indisunique as index_unique,
        i.indimmediate as index_immediate,
        i.indisvalid as index_valid,
        i.indisready as index_ready,
        i.indislive as index_live,
        i.indcheckxmin as index_check_xmin,
        i.indisprimary as index_primary,
        i.indisexclusion as index_exclusion,
        i.indisclustered as index_clustered,
        i.indisreplident as index_replica_identity,
        i.indnullsnotdistinct as index_nulls_not_distinct,
        i.indnkeyatts as index_key_count,
        i.indnatts as index_attribute_count,
        i.indkey[0] as index_first_attribute,
        i.indcollation[0] as index_collation,
        i.indoption[0] as index_option,
        opcn.nspname as opclass_namespace,
        opc.opcname as opclass_name,
        opc.opcintype as opclass_input_type,
        opc.opcdefault as opclass_default,
        opc.opckeytype as opclass_key_type,
        opfn.nspname as opfamily_namespace,
        opf.opfname as opfamily_name,
        pg_catalog.pg_get_expr(i.indexprs, i.indrelid, true) as expression,
        pg_catalog.pg_get_indexdef(i.indexrelid, 1, true) as column_definition,
        pg_catalog.pg_get_expr(i.indpred, i.indrelid, true) as predicate
    into strict idx
    from pg_catalog.pg_index i
    join pg_catalog.pg_class c on c.oid = i.indexrelid
    join pg_catalog.pg_namespace n on n.oid = c.relnamespace
    join pg_catalog.pg_am am on am.oid = c.relam
    join pg_catalog.pg_opclass opc
      on opc.oid = i.indclass[0] and opc.opcmethod = am.oid
    join pg_catalog.pg_namespace opcn on opcn.oid = opc.opcnamespace
    join pg_catalog.pg_opfamily opf
      on opf.oid = opc.opcfamily and opf.opfmethod = am.oid
    join pg_catalog.pg_namespace opfn on opfn.oid = opf.opfnamespace
    where i.indexrelid = active_index_oid;

    expression_normalized := regexp_replace(
        coalesce(idx.expression, ''), '[[:space:]()]', '', 'g'
    );
    column_normalized := regexp_replace(
        coalesce(idx.column_definition, ''), '[[:space:]()]', '', 'g'
    );
    predicate_normalized := regexp_replace(
        lower(coalesce(idx.predicate, '')), '[[:space:]()]', '', 'g'
    );

    if idx.index_table is distinct from
           'public.trnm_world_authority_epochs_v1'::pg_catalog.regclass
       or idx.index_namespace is distinct from 'public'
       or idx.index_name is distinct from 'trnm_world_single_active_writer_epoch_v1'
       or idx.index_kind is distinct from 'i'::"char"
       or idx.index_persistence is distinct from 'p'::"char"
       or idx.index_method is distinct from 'btree'
       or idx.index_unique is distinct from true
       or idx.index_immediate is distinct from true
       or idx.index_valid is distinct from true
       or idx.index_ready is distinct from true
       or idx.index_live is distinct from true
       or idx.index_check_xmin is distinct from false
       or idx.index_primary is distinct from false
       or idx.index_exclusion is distinct from false
       or idx.index_clustered is distinct from false
       or idx.index_replica_identity is distinct from false
       or idx.index_nulls_not_distinct is distinct from false
       or idx.index_key_count is distinct from 1
       or idx.index_attribute_count is distinct from 1
       or idx.index_first_attribute is distinct from 0
       or idx.index_collation is distinct from 0::pg_catalog.oid
       or idx.index_option is distinct from 0
       or idx.opclass_namespace is distinct from 'pg_catalog'
       or idx.opclass_name is distinct from 'int4_ops'
       or idx.opclass_input_type is distinct from 'pg_catalog.int4'::pg_catalog.regtype
       or idx.opclass_default is distinct from true
       or idx.opclass_key_type is distinct from 0::pg_catalog.oid
       or idx.opfamily_namespace is distinct from 'pg_catalog'
       or idx.opfamily_name is distinct from 'integer_ops'
       or expression_normalized is distinct from '1'
       or column_normalized is distinct from '1'
       or predicate_normalized not in (
           'status=''active''::textandworld_writer_enabled',
           'status=''active''andworld_writer_enabled'
       ) then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_single_active_writer_index_semantics_drift',
            detail = pg_catalog.format(
                'oid=%s table=%s schema=%s name=%s method=%s unique=%s key_count=%s first_att=%s expression=%s column=%s predicate=%s',
                active_index_oid,
                idx.index_table,
                idx.index_namespace,
                idx.index_name,
                idx.index_method,
                idx.index_unique,
                idx.index_key_count,
                idx.index_first_attribute,
                coalesce(idx.expression, '<null>'),
                coalesce(idx.column_definition, '<null>'),
                coalesce(idx.predicate, '<null>')
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
