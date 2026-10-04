#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$ROOT_DIR/trillionnium/tools/trnm-settlement-outbox-contract/Cargo.toml"
DOC="$ROOT_DIR/docs/development/trnm-settlement-outbox-v1.md"

[[ -s "$MANIFEST" ]] || {
  echo "missing settlement outbox contract manifest: $MANIFEST" >&2
  exit 1
}
[[ -s "$DOC" ]] || {
  echo "missing settlement outbox contract document: $DOC" >&2
  exit 1
}

grep -Fq 'trnm_settlement_outbox_v1' "$DOC"
grep -Fq 'MAX_SETTLEMENT_ATTEMPTS' \
  "$ROOT_DIR/trillionnium/tools/trnm-settlement-outbox-contract/src/lib.rs"

cargo fmt --manifest-path "$MANIFEST" --all -- --check
cargo test --manifest-path "$MANIFEST" --all-targets --locked

echo "TRNM settlement outbox contract: green"
