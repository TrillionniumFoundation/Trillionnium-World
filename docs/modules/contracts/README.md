---
status: current-candidate
owner: trillionnium-world-contracts
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# Rust external-contract MVP module designs

This index covers the independent `contracts/` workspace. These crates freeze bounded contract semantics and normalized audit vocabulary. They are not connected to a canonical World/Chain host ABI, deterministic WASM sandbox, gas/storage runtime, state-root pipeline, production deployment, custody service or public-mainnet authorization.

| Crate | Current responsibility | Detailed design |
|---|---|---|
| `audit-events` | shared normalized audit-event schema | [`audit-events-design.md`](audit-events-design.md) |
| `settlement-vault` | in-memory settlement lock/release/slash semantics | [`settlement-vault-design.md`](settlement-vault-design.md) |
| `bridge-relay` | in-memory proof/nonce/finality relay semantics | [`bridge-relay-design.md`](bridge-relay-design.md) |
| `governance-guard` | in-memory timelock/pause/version-guard semantics | [`governance-guard-design.md`](governance-guard-design.md) |

The target architecture still lacks the documented `sdk/`, `runtime-spec/` and `integration-tests/` packages plus a reviewed host implementation. A crate test pass proves only its current Rust state-machine contract. It cannot prove ABI compatibility, sandbox determinism, gas accounting, durable storage, Chain inclusion/finality, upgrade governance, public deployment or production authorization.
