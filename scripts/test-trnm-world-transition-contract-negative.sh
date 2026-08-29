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
    "$TEMP_ROOT/docs/protocol/vectors" \
    "$TEMP_ROOT/docs/development"
  cp "$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1/Cargo.toml" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/Cargo.toml"
  cp "$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1/Cargo.lock" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/Cargo.lock"
  cp "$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1/src/contract.rs" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src/contract.rs"
  cp "$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1/src/canonical_json.rs" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src/canonical_json.rs"
  cp "$ROOT_DIR/docs/protocol/trnm-world-transition-v1.md" \
    "$TEMP_ROOT/docs/protocol/trnm-world-transition-v1.md"
  cp "$ROOT_DIR/docs/protocol/schemas/trnm-world-transition-v1.schema.json" \
    "$TEMP_ROOT/docs/protocol/schemas/trnm-world-transition-v1.schema.json"
  cp "$ROOT_DIR/docs/protocol/vectors/trnm-world-transition-v1.json" \
    "$TEMP_ROOT/docs/protocol/vectors/trnm-world-transition-v1.json"
  cp "$ROOT_DIR/docs/development/trnm-world-transition-contract-v1.json" \
    "$TEMP_ROOT/docs/development/trnm-world-transition-contract-v1.json"
  if ! "$CHECKER" "$TEMP_ROOT" >/dev/null; then
    echo 'baseline transition negative fixture does not satisfy the real checker' >&2
    exit 1
  fi
}

expect_rejected() {
  local description=$1
  if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
    printf 'negative fixture unexpectedly accepted: %s\n' "$description" >&2
    exit 1
  fi
}

copy_fixture
python3 - "$TEMP_ROOT/docs/protocol/schemas/trnm-world-transition-v1.schema.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
request = data["$defs"]["request"]
request["properties"]["nakama_session_token"] = {"type": "string"}
request["required"].append("nakama_session_token")
path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY
expect_rejected 'authority session field'

copy_fixture
python3 - "$TEMP_ROOT/docs/protocol/vectors/trnm-world-transition-v1.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
data["request_vectors"][0]["request"]["command"]["payload"]["sha256"] = "0" * 64
path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY
expect_rejected 'mismatched payload hash'

copy_fixture
cat >> "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src/contract.rs" <<'RUST'

pub struct MatchCompletedV1;
RUST
expect_rejected 'canonical completion ownership'

copy_fixture
cp "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src/contract.rs" \
  "$TEMP_ROOT/trillionnium/contracts/trnm-world-transition-v1/src/lib.rs"
expect_rejected 'duplicate alternate source root'

copy_fixture
python3 - "$TEMP_ROOT/docs/development/trnm-world-transition-contract-v1.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
data["status"] = "verified_production_ready"
data["production_candidate"] = True
data["verification"]["github_actions_runs"] = "passed"
data["verification"]["remote_ci_covers_current_head"] = True
data["verification"]["independent_exact_head_review"] = "approved"
for item in data["acceptance"]:
    item["state"] = "passed"
data["promotion_blockers"] = []
path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY
expect_rejected 'overclaimed delivery and review status'

printf '%s\n' 'TRNM World transition negative fixtures were rejected.'
