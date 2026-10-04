# `trnm-pouw`

Status: **migration-only compatibility/provenance component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Boundary

This crate retains PoUW verification/challenge compatibility and provenance during the PoCO settlement transition. Its name, tests, and historical evidence do **not** reauthorize a default work-unit payout path or make PoUW the current payout authority.

## Responsibilities

- Legacy-compatible proof verification and challenge guardrails.
- Migration-era audit/replay evidence.
- Failure classification used by retained gates.

## Invariants

- Proof binding, replay, timeout, and challenge checks fail closed.
- Compatibility labels never imply current payout authority.
- Refactors preserve a test-name baseline so coverage cannot disappear silently.
- Verification inputs and outputs remain version/hash bound.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-pouw --lib --locked
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-pouw --lib --locked -- --list
```

## Open gaps

Production verifier packaging, sidecar operations, DA/checkpoint linkage, outage/mismatch replay, and PoCO-first authority evidence remain open wherever required by launch scope. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
