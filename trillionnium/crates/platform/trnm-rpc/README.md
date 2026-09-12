# `trnm-rpc`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Platform query, transaction, event, and relay API boundary.

It owns versioned request/response handling, query caps/error mapping, and reliability/relay behavior used by current clients. It does not replace validated application authority or provide a production explorer/indexer read model.

## Invariants

- Queries and pagination are bounded.
- Write requests bind identity, nonce, payload, and transaction hash.
- Retry/acknowledgement preserves idempotency and session boundaries.
- Errors are stable enough for automation and never disguise an integrity failure.
- Public schema changes are versioned and compatibility-tested.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-rpc --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-rpc --all-targets --locked -- -D warnings
```

## Open gaps

Stable generated public API documentation, durable historical read models, production exporter integration, abuse controls, and query SLO evidence remain open. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
