# `trnm-state`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Platform state-transition, state-root, governance, escrow, and lifecycle authority.

It owns validated mutation, root/replay invariants, governance/pause-sensitive state, and migration/recovery semantics. It does not own network transport, wallet custody, or presentation.

## Invariants

- Failed transitions are side-effect free.
- Replay reconstructs the same state root.
- Governance, pause, escrow, slash, and conservation rules fail closed.
- Migration and restore cannot silently skip objects or tests.
- Authority-sensitive changes are versioned and auditable.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-state --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-state --all-targets --locked -- -D warnings
```

State changes require old snapshots, interrupted migration, rollback/restore, root replay, conservation, duplicate, and adversarial transition tests.

## Open gaps

Large regression suites need continued decomposition and test-list baselines. Public upgrade/migration and disaster-recovery drills remain evidence gaps. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
