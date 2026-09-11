# `trnm-oracle`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Oracle observation validation, reporting, policy, and offline baseline tooling.

It does not establish a live chain-integrated oracle network, production data-source SLA, or oracle-backed settlement readiness.

## Invariants

- Stale, insufficient, malformed, or out-of-policy observations fail closed.
- Source identity, sample count, freshness, drift, aggregation policy, and policy version are auditable.
- Sources and thresholds cannot silently change.
- Replay produces the same accepted/rejected result for the same inputs and policy.
- Poisoning/outlier behavior is bounded and observable.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-oracle --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-oracle --all-targets --locked -- -D warnings
```

## Open gaps

Live ingest, persisted policy/state lifecycle, stable RPC, replay/recovery, high-volume feed, source diversity, production metrics, and incident drills remain open if oracle is in Day-1 scope. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
