# `trnm-bridge-poc`

Status: **proof of concept / migration-only**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Bridge proof-of-concept for relay heartbeat, finality/checkpoint experiments, settlement-loop semantics, replay, and compensation scenarios.

It is not a production bridge or public cross-chain safety claim.

## Invariants

- Source/target domain, bridge identity, nonce, action, proof, transaction receipt, and settlement identity are bound.
- Replay, invalid finality, stale heartbeat, and mismatched proof fail closed.
- Compensation and reversal conserve value.
- Evidence fields are stable and sufficient for replay/incident review.
- Validator/key configuration changes are versioned and authorized.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-bridge-poc --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-bridge-poc --all-targets --locked -- -D warnings
```

## Open gaps

Final trust model, production relayer, proof-material surface, validator/key lifecycle, failure recovery, observability, external audit, and operator runbook remain open. If bridge is excluded from Day-1, record that scope decision explicitly. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
