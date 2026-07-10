#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR/trillionnium"

echo "legacy game workspace is frozen and is not the player product" >&2
exec cargo run \
  --manifest-path crates/legacy-game/Cargo.toml \
  -p trnm-world-bevy \
  --features legacy \
  -- "$@"
