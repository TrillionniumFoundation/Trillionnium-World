#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-world-transition-contract.sh"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

copy_fixture() {
  rm -rf "$TEMP_ROOT"
  mkdir -p \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src" \
    "$TEMP_ROOT/docs/protocol/schemas" \
    "$TEMP_ROOT/docs/protocol/vectors"
  cp "$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1/Cargo.toml" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/Cargo.toml"
  cp "$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1/Cargo.lock" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/Cargo.lock"
  cp "$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1/src/lib.rs" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src/lib.rs"
  cp "$ROOT_DIR/docs/protocol/trnm-world-transition-v1.md" \
    "$TEMP_ROOT/docs/protocol/trnm-world-transition-v1.md"
  cp "$ROOT_DIR/docs/protocol/schemas/trnm-world-transition-v1.schema.json" \
    "$TEMP_ROOT/docs/protocol/schemas/trnm-world-transition-v1.schema.json"
  cp "$ROOT_DIR/docs/protocol/vectors/trnm-world-transition-v1.json" \
    "$TEMP_ROOT/docs/protocol/vectors/trnm-world-transition-v1.json"
}

copy_fixture
python3 - "$TEMP_ROOT/docs/protocol/schemas/trnm-world-transition-v1.schema.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
request = data["$defs"]["request"]
request["properties"]["nakama_session_token"] = {"type": "string"}
request["required"].append("nakama_session_token")
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative fixture unexpectedly accepted an authority session field' >&2
  exit 1
fi

copy_fixture
python3 - "$TEMP_ROOT/docs/protocol/vectors/trnm-world-transition-v1.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["request_vectors"][0]["request"]["command"]["payload"]["sha256"] = "0" * 64
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative fixture unexpectedly accepted a mismatched payload hash' >&2
  exit 1
fi

copy_fixture
cat >> "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src/lib.rs" <<'RUST'

pub struct MatchCompletedV1;
RUST
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative fixture unexpectedly accepted canonical completion ownership' >&2
  exit 1
fi

printf '%s\n' 'TRNM World transition negative fixtures were rejected.'
