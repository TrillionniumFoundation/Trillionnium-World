# audit-events

`audit-events` is the shared normalized audit-event vocabulary for the maintained Rust external-contract MVPs.

Detailed design: [`../../docs/modules/contracts/audit-events-design.md`](../../docs/modules/contracts/audit-events-design.md)

## Purpose

The crate provides stable event identity, source-contract identity, operation and outcome classes, bounded actor and subject references, optional revision or sequence references, and deterministic structured attributes. It gives `settlement-vault`, `bridge-relay`, and `governance-guard` one reviewable event shape so downstream test indexers and risk tooling do not infer meaning from free-form log text.

## Authority and non-goals

This crate owns schema shape and validation only. An `AuditEvent` does not authenticate a caller, authorize a mutation, prove durable append, establish settlement, provide custody, create Chain finality, or grant production evidence. Producer code must emit only the result actually returned by its state machine. Fixture or in-memory events remain fixture evidence.

## Public contract

The public surface consists of serializable audit records, stable event and result values, constructors, validators, and normalized conversion helpers used by the sibling contract crates. Field names, enum spellings, canonical ordering, identifier domains, and resource ceilings are compatibility material. Human-readable notes are bounded diagnostics and must never be parsed as authority or state.

## State and invariants

The crate owns no mutable state. A valid event has one supported schema version, nonblank bounded identifiers, an explicit producer contract and operation, one unambiguous outcome, deterministic attribute ordering, and no duplicate keys. Immutable event identity is permanently bound to the same canonical bytes and source decision. Secret-like fields, unbounded payloads, conflicting outcomes, and altered reuse of an event identity fail closed.

## Host and durability boundary

Persistence belongs to a reviewed host or outbox. Production adoption requires atomic or explicitly reconciled publication with the accountable mutation, immutable storage identity, integrity hashes, retention and deletion policy, crash recovery, and replay-gap detection. This Rust crate does not open a database, filesystem, network connection, clock, credential store, or deployment channel, and it is not a production evidence sink.

## Verification

Run the workspace checks from the repository root:

```bash
cargo test --manifest-path contracts/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path contracts/Cargo.toml --workspace --all-targets --locked -- -D warnings
python3 scripts/check-trnm-world-contract-module-documentation.py
python3 scripts/test-trnm-world-contract-module-documentation.py
```

Required coverage includes deterministic round trips, stable spellings, unsupported versions, duplicate and oversized attributes, privacy and secret-field rejection, exact duplicate identity, altered conflict, and normalized emission from every producer. Host durability, cross-repository consumption, privacy approval, and release authorization remain separate evidence classes.
