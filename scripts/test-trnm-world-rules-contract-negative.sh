#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-world-rules-contract.sh"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

seed_fixture() {
  rm -rf "$TEMP_ROOT"/*
  mkdir -p \
    "$TEMP_ROOT/trillionnium/contracts" \
    "$TEMP_ROOT/tools" \
    "$TEMP_ROOT/integration/component-locks" \
    "$TEMP_ROOT/docs/runbooks"
  cp -a "$ROOT_DIR/trillionnium/contracts/trnm-world-rules-contract-v1" \
    "$TEMP_ROOT/trillionnium/contracts/trnm-world-rules-contract-v1"
  cp -a "$ROOT_DIR/tools/trnm-world-shadow-diff" \
    "$TEMP_ROOT/tools/trnm-world-shadow-diff"
  cp "$ROOT_DIR/integration/component-locks/trnm-world-rules-v1.lock.json" \
    "$TEMP_ROOT/integration/component-locks/trnm-world-rules-v1.lock.json"
  cp "$ROOT_DIR/docs/runbooks/trnm-world-nakama-authority-cutover-v1.md" \
    "$TEMP_ROOT/docs/runbooks/trnm-world-nakama-authority-cutover-v1.md"
}

seed_fixture
python3 - "$TEMP_ROOT/trillionnium/contracts/trnm-world-rules-contract-v1/schema/transition-request-v1.schema.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["required"].append("player_session")
data["properties"]["player_session"] = {"type": "string"}
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative fixture unexpectedly accepted player-session authority in the rules contract' >&2
  exit 1
fi

seed_fixture
python3 - "$TEMP_ROOT/integration/component-locks/trnm-world-rules-v1.lock.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["compatibility"]["world_local_authority_public"] = True
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative fixture unexpectedly promoted the World compatibility enclave to public authority' >&2
  exit 1
fi

seed_fixture
sed -i 's/pub fn execute_transition_verified/pub fn execute_transition_unverified/' \
  "$TEMP_ROOT/trillionnium/contracts/trnm-world-rules-contract-v1/src/engine.rs"
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative fixture unexpectedly accepted removal of deterministic double-run verification' >&2
  exit 1
fi

seed_fixture
python3 - "$TEMP_ROOT/integration/component-locks/trnm-world-rules-v1.lock.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["producer"]["package_path"] = "../Nakama/local-checkout"
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative fixture unexpectedly accepted sibling-checkout component coupling' >&2
  exit 1
fi

printf '%s\n' 'TRNM World rules contract negative fixtures were rejected.'
