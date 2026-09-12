# settlement-vault

`settlement-vault` is the deterministic Rust MVP for bounded deposit, lock, release, slash, transfer, pause, replay-protection, accounting, and normalized audit semantics.

Detailed design: [`../../docs/modules/contracts/settlement-vault-design.md`](../../docs/modules/contracts/settlement-vault-design.md)

## Purpose

The crate fixes a reviewable pure state-machine contract for vault and position identity, actors and roles, typed assets, checked amounts, operation identity, lifecycle transitions, exact duplicate behavior, stable errors, and audit material. It provides a semantic baseline for future host, storage, formal-accounting, and cross-system conformance work.

## Authority and non-goals

This crate is not a wallet, ledger, custodian, deployed smart contract, Chain-finality source, CEX implementation, or production settlement service. In-memory balances and fixture roles do not represent held funds or authenticated principals. CEX remains authoritative for real wallet and ledger custody, and World may consume value only through independently verified receipts and the separate capture, execute, and fenced-apply protocol.

## Public contract

The public surface includes typed vault, position, operation, actor, role, asset, request, decision, result, and error values; deterministic deposit, lock, release, slash, transfer, pause, and resume methods where implemented; replay protection; and normalized audit events. An operation identity binds actor, vault or account, asset, amount, action, expected state or revision, and contract version. Exact names, arithmetic, lifecycle and error precedence require versioned vectors.

## State and invariants

Amounts are nonnegative, bounded, and checked. A release or slash cannot exceed the locked amount; total-accounting invariants are preserved; terminal operations cannot be replayed under altered bytes; paused state blocks mutation according to contract; and every rejection leaves the full state unchanged. Exact duplicate requests may converge only after verifying every binding. Altered reuse, unauthorized roles, overflow, missing positions, and invalid transitions fail closed.

## Host and durability boundary

Current state is in memory. Production adoption requires a canonical state encoding, durable storage delta, expected revision or serialized actor, immutable request and result records, atomic state-plus-audit publication, crash recovery, backup/PITR, old-writer fencing, host ABI and runtime specification, deterministic execution and metering, reconciliation, custody separation, and independent security review. This crate performs no SQL, network, filesystem, clock, credential, wallet, or remote settlement operation.

## Verification

Run:

```bash
cargo test --manifest-path contracts/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path contracts/Cargo.toml --workspace --all-targets --locked -- -D warnings
python3 scripts/check-trnm-world-contract-module-documentation.py
```

Coverage must include accepted lifecycle paths, exact and altered duplicate requests, unauthorized actors, over-release and over-slash, accounting conservation, overflow and resource bounds, pause behavior, state preservation, and normalized audit output. Host ABI, durable storage, gas or quota, Chain/CEX integration, custody, and production authorization remain independent evidence gates.
