#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-ui-contract.sh"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

mkdir -p \
  "$TEMP_ROOT/trillionnium/crates/trnm-first-contact/src" \
  "$TEMP_ROOT/docs/development"
cp "$ROOT_DIR/trillionnium/crates/trnm-first-contact/src/lib.rs" \
  "$TEMP_ROOT/trillionnium/crates/trnm-first-contact/src/lib.rs"
cp -a "$ROOT_DIR/trillionnium/crates/trnm-first-contact/src/ui" \
  "$TEMP_ROOT/trillionnium/crates/trnm-first-contact/src/ui"
cp "$ROOT_DIR/docs/development/trnm-world-ui-vertical-slice-v1.md" \
  "$TEMP_ROOT/docs/development/trnm-world-ui-vertical-slice-v1.md"
cp "$ROOT_DIR/docs/development/trnm-world-ui-acceptance-v1.json" \
  "$TEMP_ROOT/docs/development/trnm-world-ui-acceptance-v1.json"

cat > "$TEMP_ROOT/trillionnium/crates/trnm-first-contact/src/ui/forbidden.rs" <<'RUST'
struct MatchCompletedV1;
RUST
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative UI fixture unexpectedly accepted a canonical completion type' >&2
  exit 1
fi
rm "$TEMP_ROOT/trillionnium/crates/trnm-first-contact/src/ui/forbidden.rs"

python3 - "$TEMP_ROOT/docs/development/trnm-world-ui-acceptance-v1.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["public_player_market_enabled"] = True
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative UI fixture unexpectedly enabled the public player market' >&2
  exit 1
fi

printf '%s\n' 'TRNM UI negative fixtures were rejected.'
