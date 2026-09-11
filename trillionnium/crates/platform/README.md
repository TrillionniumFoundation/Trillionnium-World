# TRNM Platform Workspace

Status: **independent alpha/migration workspace**  
Manifest: `trillionnium/crates/platform/Cargo.toml`

This workspace contains chain/node/PoCO platform components retained while repository separation is completed. It is excluded from the native game workspace and cannot be used as gameplay or native-game release evidence.

## Build

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml --workspace --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml --workspace --all-targets --locked -- -D warnings
```

## Components

| Crate | Responsibility |
| --- | --- |
| [`trnm-node`](trnm-node/) | node/runtime orchestration |
| [`trnm-types`](trnm-types/) | canonical platform types and serialization |
| [`trnm-state`](trnm-state/) | state transitions, roots, governance, escrow |
| [`trnm-pouw`](trnm-pouw/) | retained verification/challenge compatibility evidence |
| [`trnm-executor`](trnm-executor/) | deterministic execution and rollback |
| [`trnm-mempool`](trnm-mempool/) | admission, ordering, duplicate/recovery queue |
| [`trnm-rpc`](trnm-rpc/) | query/transaction/event/relay boundary |
| [`trnm-bench`](trnm-bench/) | benchmark/evidence harness |
| [`trnm-worker-agent`](trnm-worker-agent/) | worker task and receipt lifecycle |
| [`trnm-cli`](trnm-cli/) | CLI wallet/query/transaction surface |
| [`trnm-bridge-poc`](trnm-bridge-poc/) | bridge proof-of-concept |
| [`trnm-oracle`](trnm-oracle/) | oracle validation/offline baseline |

## Boundary rules

No new native-game dependency may point into this workspace. Use versioned protocols or generated contracts across the boundary. `trnm-pouw` naming is retained for compatibility/provenance and does not reauthorize a default payout path. Public-mainnet closure is governed by active release truth, not a local workspace pass.

See [`../../../PROJECT_BOUNDARY.md`](../../../PROJECT_BOUNDARY.md), [`../../../docs/architecture/repository-boundaries.md`](../../../docs/architecture/repository-boundaries.md), and [`../../../docs/modules.json`](../../../docs/modules.json).
