#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKER="$ROOT_DIR/scripts/check-trnm-world-authority-boundary.sh"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$TMP_DIR/trillionnium/crates/fixture/src"
printf '%s\n' 'fn deterministic_world_step() {}' \
  >"$TMP_DIR/trillionnium/crates/fixture/src/lib.rs"
bash "$CHECKER" scan-only "$TMP_DIR" >/dev/null

expect_rejected() {
  local label="$1"
  shift
  rm -rf "$TMP_DIR"
  mkdir -p "$TMP_DIR/trillionnium/crates/fixture/src"
  "$@"
  if bash "$CHECKER" scan-only "$TMP_DIR" >/dev/null 2>&1; then
    echo "authority-boundary negative fixture unexpectedly passed: $label" >&2
    exit 1
  fi
}

expect_rejected "Nakama private key" \
  bash -c 'printf "%s\n" "const TRNM_NAKAMA_AUTHORITY_PRIVATE_KEY: &str = \"bad\";" >"$1/trillionnium/crates/fixture/src/lib.rs"' _ "$TMP_DIR"

expect_rejected "World completion signer" \
  bash -c 'printf "%s\n" "fn sign_match_completed_v1() {}" >"$1/trillionnium/crates/fixture/src/lib.rs"' _ "$TMP_DIR"

expect_rejected "World canonical event root" \
  bash -c 'printf "%s\n" "fn world_canonical_event_root() {}" >"$1/trillionnium/crates/fixture/src/lib.rs"' _ "$TMP_DIR"

expect_rejected "sibling Chain path dependency" \
  bash -c 'cat >"$1/trillionnium/crates/fixture/Cargo.toml" <<EOF
[package]
name = "fixture"
version = "0.1.0"

[dependencies]
chain-contract = { path = "../../../../Trillionnium-Chain/contracts" }
EOF' _ "$TMP_DIR"

echo "TRNM World authority-boundary negative fixtures passed"
