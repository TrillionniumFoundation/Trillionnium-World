#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LEGACY_INSTALLER="$ROOT/scripts/apply-trnm-world-authority-cutover-v1-legacy.sh"
BASE_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1.sql"
HARDENING_MIGRATION="$ROOT/deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql"
CATALOG_FINGERPRINT="$ROOT/deploy/postgres/trnm-world-authority-catalog-fingerprint-v1.sql"

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

cleanup_reference() {
  PGDATABASE="$maintenance_db" dropdb --if-exists "$reference_db" >/dev/null 2>&1 || true
}
trap cleanup_reference EXIT INT TERM

PGDATABASE="$maintenance_db" createdb --template=template0 "$reference_db"
PGDATABASE="$reference_db" psql -X -v ON_ERROR_STOP=1 -f "$BASE_MIGRATION" >/dev/null
PGDATABASE="$reference_db" psql -X -v ON_ERROR_STOP=1 -f "$HARDENING_MIGRATION" >/dev/null

reference_catalog="$(PGDATABASE="$reference_db" psql -X -v ON_ERROR_STOP=1 -At -f "$CATALOG_FINGERPRINT")"
target_catalog="$(psql -X -v ON_ERROR_STOP=1 -At -f "$CATALOG_FINGERPRINT")"

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

catalog_sha="$(printf '%s' "$target_catalog" | sha256sum | awk '{print $1}')"
printf 'trnm_world_authority_catalog_v1:ok sha256:%s\n' "$catalog_sha"
cleanup_reference
trap - EXIT INT TERM
