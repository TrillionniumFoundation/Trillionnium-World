# governance-guard

`governance-guard` is the deterministic Rust MVP for proposal, timelock, version-drift, pause, resume, and normalized audit semantics.

Detailed design: [`../../docs/modules/contracts/governance-guard-design.md`](../../docs/modules/contracts/governance-guard-design.md)

## Purpose

The crate provides a bounded state machine for scheduling, queueing, executing, cancelling, pausing, and resuming guarded changes. It freezes proposal and action identity, expected target version, supplied accountable time or height, role references, delay policy, terminal decisions, and audit output so a future host can be tested against one explicit contract.

## Authority and non-goals

This crate is not deployed governance, a multisig, a voting system, an emergency committee, a key custodian, a GitHub ruleset, a Chain upgrade executor, or proof that any live governance control is active. Actor roles are typed inputs for the MVP and do not authenticate production principals. A successful guard decision does not itself execute the protected side effect or grant production authorization.

## Public contract

The public surface includes typed proposal, action, actor, role, parameter, target-version, and time/height values; guard state; schedule, queue, execute, cancel, pause, and resume requests; stable decisions and errors; version-concurrency checks; and normalized audit records. Canonical action hashes, role policy, delay calculation, error precedence, and state transition rules are versioned compatibility material.

## State and invariants

A proposal identity is permanently bound to one canonical action, target and expected version, proposer context, delay, expiry, and contract version. Execution cannot occur before the timelock, after cancellation or expiry, during an incompatible pause state, or when target or parameter versions drift. Pause and resume follow explicit role and lifecycle rules. Exact retries may converge, altered identity reuse fails, and every error preserves complete guard state.

## Host and durability boundary

The current state is in memory and time or height is supplied explicitly. Production adoption requires authenticated principals, quorum or multisig evidence, a canonical time/height source, immutable proposal and decision records, atomic state/audit publication, a durable guarded-action receipt, restart recovery, backup/PITR, old-executor fencing, key rotation, separation of duties, and emergency drills. The crate performs no network, SQL, filesystem, secret, or target-execution operation.

## Verification

Run:

```bash
cargo test --manifest-path contracts/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path contracts/Cargo.toml --workspace --all-targets --locked -- -D warnings
python3 scripts/check-trnm-world-contract-module-documentation.py
```

Coverage must include too-early and expired execution, schedule/execute/cancel, exact and altered duplicate identity, action and target-version drift, parameter concurrency, pause/resume roles and lifecycle, overflow and bounds, state preservation, deterministic audit output, and property tests. Authenticated quorum, durable execution, key custody, and live governance evidence remain separate blockers.
