# `trnm-mempool`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Platform transaction/request admission, ordering, duplicate handling, bounded queueing, and recovery.

It does not own final execution, consensus finality, wallet signing, or launch economics.

## Invariants

- Capacity and per-class budgets are bounded.
- Rejected items do not mutate authority state.
- Duplicate/replay behavior is explicit and deterministic.
- Recovery preserves ordering and cannot double-admit a logical request.
- Backpressure and eviction policy are observable.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-mempool --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-mempool --all-targets --locked -- -D warnings
```

Review adversarial floods, priority starvation, restart recovery, duplicate storms, and memory limits.

## Open gaps

The public fee/anti-spam/sponsor/retention policy and sustained adversarial-load evidence remain public-mainnet blockers. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
