#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_lines=(
  'REFRESH="${TRNM_BEVY_PLAYTEST_READINESS_REFRESH:-1}"'
  'for arg in "$@"; do'
  '--refresh)'
  'REFRESH=1'
  '--no-refresh)'
  'REFRESH=0'
  'unknown option: %s'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$READINESS"; then
    echo "[FAIL] missing classic playtest readiness refresh-mode line: $line" >&2
    exit 1
  fi
done

set +e
"$READINESS" --definitely-not-a-real-option >/dev/null 2>"$ROOT/target/bevy-classic-playtest-readiness-invalid-option.stderr"
status=$?
set -e

if [[ "$status" -ne 2 ]]; then
  echo "[FAIL] readiness script did not reject an unknown option with exit code 2" >&2
  exit 1
fi

if ! grep -Fq 'unknown option: --definitely-not-a-real-option' "$ROOT/target/bevy-classic-playtest-readiness-invalid-option.stderr"; then
  echo "[FAIL] readiness script did not print the rejected option" >&2
  exit 1
fi

echo "[PASS] classic playtest readiness supports explicit --refresh/--no-refresh modes and rejects unknown options"
