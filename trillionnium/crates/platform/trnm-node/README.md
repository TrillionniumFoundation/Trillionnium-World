# `trnm-node`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

> Excluded from the native game workspace. Build explicitly and never cite its tests as gameplay evidence.

## Purpose and responsibilities

Node process and orchestration surface for the independent platform workspace. It owns runtime composition, current local block/consensus-loop integration, health, and summary output.

It is not a finished public peer-discovery/state-sync network, native game authority, or proof of validator/operator production readiness.

## Invariants and failure semantics

- Node identity and configuration are explicit.
- State/execution failure cannot silently commit.
- Health must reflect critical dependency and liveness failure.
- Restart and recovery behavior must preserve deterministic state and evidence identity.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-node --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-node --all-targets --locked -- -D warnings
```

Public serialized state/events require explicit versioning, old-fixture tests, rollout/rollback order, and interrupted-operation behavior.

## Open gaps

Peer discovery/bootstrap, authenticated transport/gossip, state/snapshot sync, abuse controls, join/rejoin, and signed operator lifecycle remain public-mainnet blockers. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md) and [`RELEASE_READINESS.md`](../../../../RELEASE_READINESS.md).
