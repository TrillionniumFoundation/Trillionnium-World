#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$root"
python3 -B -m unittest scripts.test_trnm_world_priority_contracts -v
python3 -B -m unittest scripts.test_trnm_world_priority_preflight -v
cargo fmt --manifest-path trillionnium/crates/world-authority/Cargo.toml --all -- --check
cargo test --manifest-path trillionnium/crates/world-authority/Cargo.toml \
  -p trnm-world-api -p trnm-world-server --all-targets --locked
cargo clippy --manifest-path trillionnium/crates/world-authority/Cargo.toml \
  -p trnm-world-api -p trnm-world-server --all-targets --locked -- -D warnings
printf 'WORLD_PRIORITY_CONTRACTS=PASS\nPRODUCTION_AUTHORIZATION=not_granted\n'
