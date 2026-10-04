#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STATE_CHECKER="$ROOT/scripts/check-trnm-world-authority-postgres.sh"
INSTALL_CHECKER="$ROOT/scripts/check-trnm-world-authority-postgres-installation.sh"
CLIENT_WRAPPER_DIR=""
ADAPTER_WRAPPER_DIR=""
export TRNM_WORLD_POSTGRES_ROOT="$ROOT"

cleanup() {
  [[ -z "$CLIENT_WRAPPER_DIR" ]] || rm -rf "$CLIENT_WRAPPER_DIR"
  [[ -z "$ADAPTER_WRAPPER_DIR" ]] || rm -rf "$ADAPTER_WRAPPER_DIR"
}
trap cleanup EXIT

for checker in "$STATE_CHECKER" "$INSTALL_CHECKER"; do
  [[ -x "$checker" || -f "$checker" ]] || {
    echo "missing World PostgreSQL checker: $checker" >&2
    exit 66
  }
done

if [[ -n "${TRNM_WORLD_POSTGRES_CLIENT_IMAGE:-}" ]]; then
  command -v docker >/dev/null || {
    echo "docker is required for the pinned PostgreSQL client image" >&2
    exit 69
  }
  [[ "$TRNM_WORLD_POSTGRES_CLIENT_IMAGE" =~ @sha256:[0-9a-f]{64}$ ]] || {
    echo "TRNM_WORLD_POSTGRES_CLIENT_IMAGE must be pinned by sha256 digest" >&2
    exit 64
  }

  docker image inspect "$TRNM_WORLD_POSTGRES_CLIENT_IMAGE" >/dev/null 2>&1 \
    || docker pull "$TRNM_WORLD_POSTGRES_CLIENT_IMAGE" >/dev/null

  CLIENT_WRAPPER_DIR="$(mktemp -d)"
  for client_command in psql createdb dropdb pg_dump pg_restore; do
    cat >"$CLIENT_WRAPPER_DIR/$client_command" <<'WRAPPER'
#!/usr/bin/env bash
set -euo pipefail

: "${TRNM_WORLD_POSTGRES_CLIENT_IMAGE:?TRNM_WORLD_POSTGRES_CLIENT_IMAGE is required}"
client_command="$(basename "$0")"
workdir="$(pwd -P)"

docker_args=(
  run --rm -i --network host
  --user "$(id -u):$(id -g)"
  -e "PGHOST=${PGHOST:-127.0.0.1}"
  -e "PGPORT=${PGPORT:-5432}"
  -e "PGUSER=${PGUSER:-postgres}"
  -e "PGPASSWORD=${PGPASSWORD:-}"
  -e "PGDATABASE=${PGDATABASE:-postgres}"
)
mounted_paths=()
add_bind_mount() {
  local path="$1"
  [[ -n "$path" ]] || return 0
  if [[ ! -e "$path" ]]; then
    mkdir -p "$path"
  fi
  path="$(cd "$path" && pwd -P)"
  local existing
  for existing in "${mounted_paths[@]}"; do
    [[ "$existing" != "$path" ]] || return 0
  done
  docker_args+=( -v "$path:$path" )
  mounted_paths+=( "$path" )
}

add_bind_mount /tmp
add_bind_mount "${TRNM_WORLD_POSTGRES_ROOT:-}"
add_bind_mount "${TRNM_WORLD_POSTGRES_EVIDENCE_DIR:-}"
add_bind_mount "$workdir"
docker_args+=( -w "$workdir" )

exec docker "${docker_args[@]}" \
  "$TRNM_WORLD_POSTGRES_CLIENT_IMAGE" "$client_command" "$@"
WRAPPER
    chmod 0755 "$CLIENT_WRAPPER_DIR/$client_command"
  done
  export PATH="$CLIENT_WRAPPER_DIR:$PATH"
fi

REAL_PSQL="$(command -v psql)"
REAL_PG_RESTORE="$(command -v pg_restore)"
for executable in "$REAL_PSQL" "$REAL_PG_RESTORE"; do
  [[ -x "$executable" ]] || {
    echo "PostgreSQL client is not executable: $executable" >&2
    exit 69
  }
done

ADAPTER_WRAPPER_DIR="$(mktemp -d)"
cat >"$ADAPTER_WRAPPER_DIR/psql" <<'WRAPPER'
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
chmod 0755 "$ADAPTER_WRAPPER_DIR/psql"

cat >"$ADAPTER_WRAPPER_DIR/pg_restore" <<'WRAPPER'
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
chmod 0755 "$ADAPTER_WRAPPER_DIR/pg_restore"

export TRNM_WORLD_REAL_PSQL="$REAL_PSQL"
export TRNM_WORLD_REAL_PG_RESTORE="$REAL_PG_RESTORE"
export PATH="$ADAPTER_WRAPPER_DIR:$PATH"

bash "$STATE_CHECKER"
bash "$INSTALL_CHECKER"
