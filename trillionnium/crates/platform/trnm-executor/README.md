# `trnm-executor`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Deterministic platform execution and rollback boundary.

It owns validated execution, resource/environment interpretation, and atomic failure/rollback behavior. It does not own mempool admission, networking, or durable wallet signing.

## Invariants

- The same valid input and version produce the same committed effect.
- Invalid or failed execution cannot leak partial state.
- Resource limits and environment inputs are explicit and bounded.
- Output/state-root material is deterministic and replayable.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-executor --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-executor --all-targets --locked -- -D warnings
```

Changes require negative, rollback, limit, deterministic replay, and compatibility tests. Production sandbox/resource-accounting and public upgrade evidence remain open. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
