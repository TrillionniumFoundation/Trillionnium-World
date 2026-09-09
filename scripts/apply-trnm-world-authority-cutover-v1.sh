#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LEGACY_INSTALLER="$ROOT/scripts/apply-trnm-world-authority-cutover-v1-legacy.sh"
BASE_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1.sql"
HARDENING_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql"
CATALOG_FINGERPRINT="$ROOT/deploy/postgres/trnm-world-authority-catalog-fingerprint-v1.sql"
EXPECTED_CATALOG_FINGERPRINT_BLOB="ab255eecdc0786b334939f29a3470000af3ddc1e"

: "${PGHOST:?PGHOST is required}"
: "${PGPORT:?PGPORT is required}"
: "${PGUSER:?PGUSER is required}"
: "${PGDATABASE:?PGDATABASE is required}"

for path in "$LEGACY_INSTALLER" "$BASE_MIGRATION" "$HARDENING_MIGRATION" "$CATALOG_FINGERPRINT"; do
  [[ -s "$path" ]] || {
    echo "missing World authority installer input: $path" >&2
    exit 66
  }
done

actual_catalog_blob="$(git -C "$ROOT" hash-object "$CATALOG_FINGERPRINT")"
[[ "$actual_catalog_blob" == "$EXPECTED_CATALOG_FINGERPRINT_BLOB" ]] || {
  echo "World authority catalog fingerprint source drift: expected=$EXPECTED_CATALOG_FINGERPRINT_BLOB actual=$actual_catalog_blob" >&2
  exit 65
}

# First apply and validate the existing state-machine/index semantics.
"$LEGACY_INSTALLER"

# Then prove the entire accepted catalog is exactly equivalent to a clean
# reference install produced from these same immutable migration files.
# This catches CREATE ... IF NOT EXISTS grandfathering of weakened tables,
# constraints, triggers, functions, owners, security/search_path properties,
# indexes and grants while preserving exact-schema reapplication.
raw_ref="trnm_world_catalog_ref_${GITHUB_RUN_ID:-0}_${GITHUB_RUN_ATTEMPT:-0}_$$_${RANDOM}"
reference_db="${raw_ref:0:63}"
maintenance_db="${TRNM_WORLD_MAINTENANCE_DB:-postgres}"
hostile_dbs=()

cleanup_all() {
  for db in "${hostile_dbs[@]:-}"; do
    [[ -n "$db" ]] && PGDATABASE="$maintenance_db" dropdb --if-exists "$db" >/dev/null 2>&1 || true
  done
  PGDATABASE="$maintenance_db" dropdb --if-exists "$reference_db" >/dev/null 2>&1 || true
}
trap cleanup_all EXIT INT TERM

PGDATABASE="$maintenance_db" createdb --template=template0 "$reference_db"
PGDATABASE="$reference_db" psql -X -v ON_ERROR_STOP=1 -f "$BASE_MIGRATION" >/dev/null
PGDATABASE="$reference_db" psql -X -v ON_ERROR_STOP=1 -f "$HARDENING_MIGRATION" >/dev/null

catalog_for_db() {
  local db="$1"
  PGDATABASE="$db" psql -X -v ON_ERROR_STOP=1 -At -f "$CATALOG_FINGERPRINT"
}

reference_catalog="$(catalog_for_db "$reference_db")"
target_catalog="$(catalog_for_db "$PGDATABASE")"

[[ -n "$reference_catalog" && -n "$target_catalog" ]] || {
  echo "World authority catalog fingerprint is empty" >&2
  exit 65
}

if [[ "$target_catalog" != "$reference_catalog" ]]; then
  reference_sha="$(printf '%s' "$reference_catalog" | sha256sum | awk '{print $1}')"
  target_sha="$(printf '%s' "$target_catalog" | sha256sum | awk '{print $1}')"
  echo "World authority catalog drift: target_sha256=$target_sha reference_sha256=$reference_sha" >&2
  exit 65
fi

# Deterministic hostile catalog fixtures. Each database is a template clone of
# the clean reference, receives exactly one semantic drift class, and must stop
# matching the canonical catalog. This proves the comparator is sensitive to
# the concrete bypass classes called out by independent review.
assert_hostile_drift() {
  local label="$1"
  local sql="$2"
  local raw="trnm_world_hostile_${label}_${GITHUB_RUN_ID:-0}_$$_${RANDOM}"
  local db="${raw:0:63}"
  hostile_dbs+=("$db")
  PGDATABASE="$maintenance_db" createdb --template="$reference_db" "$db"
  PGDATABASE="$db" psql -X -v ON_ERROR_STOP=1 -c "$sql" >/dev/null
  local hostile_catalog
  hostile_catalog="$(catalog_for_db "$db")"
  [[ -n "$hostile_catalog" ]] || {
    echo "hostile catalog fixture produced empty fingerprint: $label" >&2
    exit 65
  }
  if [[ "$hostile_catalog" == "$reference_catalog" ]]; then
    echo "hostile catalog fixture escaped comparator: $label" >&2
    exit 65
  fi
  PGDATABASE="$maintenance_db" dropdb --if-exists "$db" >/dev/null
  hostile_dbs[-1]=""
  printf 'trnm_world_authority_catalog_hostile:%s:rejected\n' "$label"
}

# PostgreSQL correctly refuses to drop a referenced primary key. Build the
# hostile clone by temporarily removing the four exact dependent foreign keys,
# replacing the primary key with a plain unique constraint, and restoring the
# foreign keys. The resulting catalog remains referentially valid while lacking
# the required primary-key semantics, so the comparator—not DDL dependency
# failure—is what rejects the fixture.
assert_hostile_drift "missing_epoch_pk" \
  "alter table public.trnm_world_migration_batches_v1 drop constraint trnm_world_migration_batches_v1_epoch_id_fkey;
   alter table public.trnm_world_states_v1 drop constraint trnm_world_states_v1_writer_epoch_id_fkey;
   alter table public.trnm_world_events_v1 drop constraint trnm_world_events_v1_epoch_id_fkey;
   alter table public.trnm_world_authority_receipts_v1 drop constraint trnm_world_authority_receipts_v1_epoch_id_fkey;
   alter table public.trnm_world_authority_epochs_v1 drop constraint trnm_world_authority_epochs_v1_pkey;
   alter table public.trnm_world_authority_epochs_v1 add constraint trnm_world_authority_epochs_v1_epoch_id_unique unique (epoch_id);
   alter table public.trnm_world_migration_batches_v1 add constraint trnm_world_migration_batches_v1_epoch_id_fkey foreign key (epoch_id) references public.trnm_world_authority_epochs_v1(epoch_id);
   alter table public.trnm_world_states_v1 add constraint trnm_world_states_v1_writer_epoch_id_fkey foreign key (writer_epoch_id) references public.trnm_world_authority_epochs_v1(epoch_id);
   alter table public.trnm_world_events_v1 add constraint trnm_world_events_v1_epoch_id_fkey foreign key (epoch_id) references public.trnm_world_authority_epochs_v1(epoch_id);
   alter table public.trnm_world_authority_receipts_v1 add constraint trnm_world_authority_receipts_v1_epoch_id_fkey foreign key (epoch_id) references public.trnm_world_authority_epochs_v1(epoch_id)"

assert_hostile_drift "missing_event_fk" \
  "do \$\$ declare n text; begin select conname into strict n from pg_catalog.pg_constraint where conrelid='public.trnm_world_events_v1'::regclass and contype='f' order by conname limit 1; execute format('alter table public.trnm_world_events_v1 drop constraint %I', n); end \$\$"

assert_hostile_drift "missing_event_unique" \
  "do \$\$ declare n text; begin select conname into strict n from pg_catalog.pg_constraint where conrelid='public.trnm_world_events_v1'::regclass and contype='u' order by conname limit 1; execute format('alter table public.trnm_world_events_v1 drop constraint %I', n); end \$\$"

assert_hostile_drift "weakened_nullability_default" \
  "alter table public.trnm_world_authority_epochs_v1 alter column status drop not null; alter table public.trnm_world_authority_epochs_v1 alter column world_writer_enabled drop default"

assert_hostile_drift "wrong_not_valid_check" \
  "alter table public.trnm_world_authority_epochs_v1 drop constraint trnm_world_authority_never_dual_writer_v1; alter table public.trnm_world_authority_epochs_v1 add constraint trnm_world_authority_never_dual_writer_v1 check (true) not valid"

assert_hostile_drift "disabled_immutable_trigger" \
  "do \$\$ declare r record; begin select c.relname, t.tgname into strict r from pg_catalog.pg_trigger t join pg_catalog.pg_class c on c.oid=t.tgrelid join pg_catalog.pg_namespace n on n.oid=c.relnamespace where n.nspname='public' and c.relname like 'trnm_world_%' and not t.tgisinternal order by c.relname,t.tgname limit 1; execute format('alter table public.%I disable trigger %I', r.relname, r.tgname); end \$\$"

assert_hostile_drift "function_security_search_path" \
  "alter function public.trnm_world_sha256_text_v1(text) security definer; alter function public.trnm_world_sha256_text_v1(text) set search_path = public"

assert_hostile_drift "overbroad_public_grant" \
  "grant select on all tables in schema public to public"

catalog_sha="$(printf '%s' "$target_catalog" | sha256sum | awk '{print $1}')"
printf 'trnm_world_authority_catalog_v1:ok sha256:%s\n' "$catalog_sha"
cleanup_all
trap - EXIT INT TERM
