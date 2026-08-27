#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-world-runtime-boundary.sh"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

copy_fixture() {
  rm -rf "$TEMP_ROOT"/*
  mkdir -p "$TEMP_ROOT/contracts/world-runtime" "$TEMP_ROOT/docs" "$TEMP_ROOT/scripts"
  cp -a "$ROOT_DIR/contracts/world-runtime/v1" "$TEMP_ROOT/contracts/world-runtime/v1"
  cp -a "$ROOT_DIR/contracts/world-runtime/rust" "$TEMP_ROOT/contracts/world-runtime/rust"
  cp -a "$ROOT_DIR/contracts/world-runtime/host" "$TEMP_ROOT/contracts/world-runtime/host"
  mkdir -p "$TEMP_ROOT/docs/protocol" "$TEMP_ROOT/docs/development" "$TEMP_ROOT/docs/runbooks"
  cp "$ROOT_DIR/docs/protocol/trnm-world-runtime-v1.md" "$TEMP_ROOT/docs/protocol/"
  cp "$ROOT_DIR/docs/development/trnm-world-nakama-shadow-v1.md" "$TEMP_ROOT/docs/development/"
  cp "$ROOT_DIR/docs/runbooks/trnm-world-authority-cutover-v1.md" "$TEMP_ROOT/docs/runbooks/"
  cp "$ROOT_DIR/scripts/verify-trnm-world-runtime-v1.py" "$TEMP_ROOT/scripts/"
  cp "$ROOT_DIR/scripts/verify-trnm-world-shadow-v1.py" "$TEMP_ROOT/scripts/"
}

copy_fixture
printf '\nreqwest = "0.12"\n' >> "$TEMP_ROOT/contracts/world-runtime/host/Cargo.toml"
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative runtime fixture unexpectedly accepted a network dependency' >&2
  exit 1
fi

copy_fixture
printf '\nuse std::net::TcpStream;\n' >> "$TEMP_ROOT/contracts/world-runtime/host/src/lib.rs"
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative runtime fixture unexpectedly accepted a socket import' >&2
  exit 1
fi

copy_fixture
python3 - "$TEMP_ROOT/contracts/world-runtime/v1/compatibility-matrix.json" <<'PY'
import json
import pathlib
import sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
data["flags"]["public_online_enabled"] = True
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if "$CHECKER" "$TEMP_ROOT" >/dev/null 2>&1; then
  echo 'negative runtime fixture unexpectedly enabled public online' >&2
  exit 1
fi

printf '%s\n' 'TRNM World runtime boundary negative fixtures were rejected.'
