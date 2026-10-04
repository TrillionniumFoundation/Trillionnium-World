# `trnm-worker-agent`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Worker lifecycle, task-execution adapter, receipt submission, retry, and recovery surface.

## Responsibilities and invariants

- Bind tasks, outputs, commits/reveals, transaction hashes, and receipts.
- Bound retries and classify terminal versus recoverable failure.
- Preserve idempotency across restart so one logical result cannot be submitted twice.
- Keep external workload execution isolated from authority and credentials.
- Emit structured evidence for handoff, replay, and incident attribution.

This crate does not own consensus, task settlement authority, or production secret management for operators.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-worker-agent --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-worker-agent --all-targets --locked -- -D warnings
```

Review timeout, cancellation, process crash, duplicate receipt, malicious output, resource exhaustion, and signer failure.

## Open gaps

Production signer/custody, operator deployment, adversarial workload isolation, multi-agent fleet capacity, and public-network evidence remain open. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
