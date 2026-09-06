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
    active_index_definition text;
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

    select pg_catalog.pg_get_indexdef(indexrelid)
    into strict active_index_definition
    from pg_catalog.pg_index
    where indexrelid = 'public.trnm_world_single_active_writer_epoch_v1'::regclass;
    if active_index_definition not like '%WHERE ((status = ''active''::text) AND world_writer_enabled)%'
       and active_index_definition not like '%WHERE ((status = ''active'') AND world_writer_enabled)%' then
        raise exception 'single-active-writer partial index predicate drift: %', active_index_definition;
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
