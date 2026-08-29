#!/usr/bin/env bash
set -euo pipefail

root=${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)}
checker="$root/scripts/check-trnm-world-transition-https-fixture.sh"

[[ -f "$checker" ]] || { echo 'negative fixture checker is missing' >&2; exit 1; }
bash "$checker" "$root" >/dev/null

expect_rejected() {
  local name=$1
  local temporary
  temporary=$(mktemp -d)
  mkdir -p "$temporary/tools" "$temporary/docs/development" "$temporary/scripts"
  cp -R "$root/tools/trnm-world-transition-https-v1" "$temporary/tools/"
  cp "$root/docs/development/WORLD_TRANSITION_HTTPS_FIXTURE_V1.md" "$temporary/docs/development/"
  cp "$root/docs/development/world-transition-https-fixture-v1-status.json" "$temporary/docs/development/"
  cp "$checker" "$temporary/scripts/"

  case "$name" in
    authority_overclaim)
      python3 - "$temporary/docs/development/world-transition-https-fixture-v1-status.json" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1]); value = json.loads(path.read_text())
value["authority"]["cutover_authorized"] = True
path.write_text(json.dumps(value, indent=2) + "\n")
PY
      ;;
    tls_downgrade)
      sed -i 's/tls.VersionTLS13/tls.VersionTLS12/g' "$temporary/tools/trnm-world-transition-https-v1/internal/fixture/server.go"
      ;;
    missing_directory_fsync)
      sed -i 's/directory.Sync()/directory.Close()/g' "$temporary/tools/trnm-world-transition-https-v1/internal/fixture/store.go"
      ;;
    forbidden_database_capability)
      sed -i '/"context"/a\	"database/sql"' "$temporary/tools/trnm-world-transition-https-v1/internal/fixture/server.go"
      ;;
    non_scratch_container)
      sed -i 's/^FROM scratch$/FROM alpine:latest/' "$temporary/tools/trnm-world-transition-https-v1/Dockerfile.prebuilt"
      ;;
    *)
      echo "unknown negative fixture: $name" >&2
      rm -rf "$temporary"
      exit 64
      ;;
  esac

  if bash "$temporary/scripts/check-trnm-world-transition-https-fixture.sh" "$temporary" >/dev/null 2>&1; then
    rm -rf "$temporary"
    printf 'negative fixture unexpectedly passed: %s\n' "$name" >&2
    exit 1
  fi
  rm -rf "$temporary"
}

for fixture in \
  authority_overclaim \
  tls_downgrade \
  missing_directory_fsync \
  forbidden_database_capability \
  non_scratch_container; do
  expect_rejected "$fixture"
done

printf '%s\n' 'World transition HTTPS fixture negative matrix: passed'
