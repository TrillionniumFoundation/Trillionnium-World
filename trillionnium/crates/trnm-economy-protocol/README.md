# `trnm-economy-protocol`

Status: **alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Defines the game-owned economic boundary between campaign progression and optional settlement backends.

## Responsibilities

- Asset, currency, transferability, intent, receipt, and progression-class semantics.
- Account binding, wallet snapshots, idempotency keys, and backend manifests.
- Server-signed value-entitlement shapes and bounded game policy constants.
- Fail-closed classification of settlement outcomes.

## Non-goals

It does not own the CEX ledger, PostgreSQL transaction, wallet custody, HTTP transport, or campaign mutation. A public-market type does not enable a public player market.

## Public contracts

- `term_exchange_protocol_v2` / `term_exchange_backend_v2`.
- `trnm_server_signed_value_entitlement_v1` and `v2`.
- `trnm_economy_policy_v1`.

## State and invariants

- Local soft credits are bound and non-convertible to wallet credits.
- Temporary RTS resources are ephemeral and never settle as durable wallet value.
- Receipts bind the original intent and derive one explicit progression class.
- Positive wallet issuance requires a valid server entitlement and policy caps.
- Exact duplicates preserve idempotent outcome; mismatched reuse fails closed.
- Public player trading remains disabled until its separate release gate closes.

## Failure semantics

Recoverable network or ledger failures hold progression and remain retryable. Malformed, mismatched, unauthorized, or integrity-invalid responses are hard failures. An adapter must never silently convert a hard failure into local success.

## Build and test

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-economy-protocol --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-economy-protocol --all-targets --locked -- -D warnings
```

## Compatibility and security

Breaking changes require a new protocol/build pair, compatibility matrix, migration order, and removal criteria. Review duplicate issuance, entitlement forgery, account confusion, replay, refund/chargeback races, integer boundaries, and local-to-wallet laundering. Signing and ledger secrets stay outside this crate.

## References

- [`trnm-economy-protocol detailed design`](../../../docs/modules/trnm-economy-protocol-design.md)
- [`trillionnium-term-exchange-kernel-v1.md`](../../../docs/development/trillionnium-term-exchange-kernel-v1.md)
- [`trnm-cex-economy-integration-v1.md`](../../../docs/development/trnm-cex-economy-integration-v1.md)
- [`GAME_STATUS.md`](../../../GAME_STATUS.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)
