# `trnm-types`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

> Excluded from the native game workspace; never use platform test results as gameplay evidence.

## Purpose

Shared platform value types, serialization identities, hashing/signing inputs, and interoperability vocabulary.

This crate does not own state mutation, persistence, transport, or execution policy.

## Invariants

- Serialized identifiers remain canonical and unambiguous.
- Hash and signing payloads are deterministic.
- Unknown or malformed values fail validation before authority mutation.
- Breaking shape changes receive a version and migration rather than silent reinterpretation.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-types --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-types --all-targets --locked -- -D warnings
```

Add round-trip, old-fixture, invalid-input, and cross-version tests for public types. Generated schema coverage and an explicit compatibility matrix remain open. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
