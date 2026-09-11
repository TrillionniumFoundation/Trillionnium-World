# `trnm-online-protocol`

Status: **alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Defines versioned wire objects shared by the native client, game server, product control plane, operations, and production services.

## Responsibilities

- Authority, stream, lobby, matchmaking, social, moderation, operations, and production request/response shapes.
- Exact protocol/build compatibility validation.
- Snapshot delta construction and application helpers.

It does not perform transport, authentication, persistence, or simulation, and a protocol constant does not establish deployment credit.

## Current contracts

- `trnm_online_authority_v3` with exact-v2 rolling compatibility.
- `trnm-online-stream-v1`.
- `trnm_online_product_v2`.
- `trnm_online_operations_v2`.
- `trnm_online_production_v2`.

## Invariants

- Accepted protocol/build pairs are exact; crossed pairs fail closed.
- V3 uses per-player input cursors and a server-assigned total order.
- Snapshot deltas bind base hash/tick and apply only to that exact base.
- Offline authority identifiers cannot be promoted into online value or ranking.
- Compatibility fields retain documented legacy semantics.

Version, cursor, hash, enum/state, or identity mismatch must surface explicitly. Callers resync or fail closed rather than guessing.

## Build and test

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-online-protocol --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-online-protocol --all-targets --locked -- -D warnings
```

Every compatibility window needs a tested matrix, rollout/drain order, rollback support, and dated contraction criterion. Completed-match compatibility does not imply active cross-version takeover.

## Security

Wire input is hostile. Bound lengths/collections, authenticate in the service, validate actor/control ownership, and never place credentials in URLs or snapshots.

## References

- [`trnm-online-authority-v3.md`](../../../docs/development/trnm-online-authority-v3.md)
- [`trnm-online-product-v2.md`](../../../docs/development/trnm-online-product-v2.md)
- [`trnm-online-operations-v2.md`](../../../docs/development/trnm-online-operations-v2.md)
- [`trnm-online-production-v2.md`](../../../docs/development/trnm-online-production-v2.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)
