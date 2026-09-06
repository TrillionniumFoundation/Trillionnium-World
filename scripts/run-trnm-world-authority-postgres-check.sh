#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT/scripts/check-trnm-world-authority-postgres.sh"
REAL_PSQL="$(command -v psql)"
REAL_PG_RESTORE="$(command -v pg_restore)"
WRAPPER_DIR="$(mktemp -d)"

cleanup() {
  rm -rf "$WRAPPER_DIR"
}
trap cleanup EXIT

[[ -x "$CHECKER" || -f "$CHECKER" ]] || {
  echo "missing World PostgreSQL checker: $CHECKER" >&2
  exit 66
}
for executable in "$REAL_PSQL" "$REAL_PG_RESTORE"; do
  [[ -x "$executable" ]] || {
    echo "PostgreSQL client is not executable: $executable" >&2
    exit 69
  }
done

cat >"$WRAPPER_DIR/psql" <<'WRAPPER'
#!/usr/bin/env bash
set -euo pipefail

: "${TRNM_WORLD_REAL_PSQL:?TRNM_WORLD_REAL_PSQL is required}"

arguments=()
sql_command=""
expect_sql=false
for argument in "$@"; do
  if [[ "$expect_sql" == "true" ]]; then
    sql_command="$argument"
    expect_sql=false
    continue
  fi

  case "$argument" in
    -c)
      expect_sql=true
      ;;
    -?*c)
      # psql accepts combined short options such as -Atc. Preserve every
      # preceding option and consume the following argument as SQL input.
      prefix="${argument%c}"
      if [[ "$prefix" != "-" ]]; then
        arguments+=("$prefix")
      fi
      expect_sql=true
      ;;
    *)
      arguments+=("$argument")
      ;;
  esac
done

if [[ "$expect_sql" == "true" ]]; then
  echo "psql wrapper received -c without a command" >&2
  exit 64
fi

if [[ -n "$sql_command" ]]; then
  # psql variable interpolation is performed for normal input, not reliably for
  # every -c invocation. Feeding the exact command through stdin preserves
  # ON_ERROR_STOP and makes :'name' substitutions deterministic.
  printf '%s\n' "$sql_command" | exec "$TRNM_WORLD_REAL_PSQL" "${arguments[@]}"
fi

exec "$TRNM_WORLD_REAL_PSQL" "${arguments[@]}"
WRAPPER
chmod 0755 "$WRAPPER_DIR/psql"

cat >"$WRAPPER_DIR/pg_restore" <<'WRAPPER'
#!/usr/bin/env bash
set -euo pipefail

: "${TRNM_WORLD_REAL_PG_RESTORE:?TRNM_WORLD_REAL_PG_RESTORE is required}"

has_destination=false
for argument in "$@"; do
  case "$argument" in
    -d|--dbname|--dbname=*|-f|--file|--file=*)
      has_destination=true
      ;;
  esac
done

if [[ "$has_destination" == "false" ]]; then
  : "${PGDATABASE:?PGDATABASE is required when pg_restore has no explicit destination}"
  exec "$TRNM_WORLD_REAL_PG_RESTORE" --dbname="$PGDATABASE" "$@"
fi

exec "$TRNM_WORLD_REAL_PG_RESTORE" "$@"
WRAPPER
chmod 0755 "$WRAPPER_DIR/pg_restore"

export TRNM_WORLD_REAL_PSQL="$REAL_PSQL"
export TRNM_WORLD_REAL_PG_RESTORE="$REAL_PG_RESTORE"
export PATH="$WRAPPER_DIR:$PATH"

exec bash "$CHECKER"
