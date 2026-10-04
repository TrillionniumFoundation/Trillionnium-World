# `contracts/` — Rust-native external-contract MVP perimeter

## Current status

This independent workspace contains four crates:

- `audit-events/` — shared normalized audit-event vocabulary;
- `settlement-vault/` — bounded in-memory lock/release/slash semantics;
- `bridge-relay/` — bounded in-memory proof/nonce/finality relay semantics;
- `governance-guard/` — bounded in-memory timelock/pause/version-guard semantics.

Per-crate detailed designs are indexed at `docs/modules/contracts/README.md`.

These crates are **Rust MVP state machines/shared schema**, not a production external-contract runtime. They are not automatically compiled or delivered as canonical `wasm32-unknown-unknown` artifacts; are not connected to a deterministic node-side WASM executor, gas/storage quota, state-root inclusion or stable public RPC ABI; and do not constitute public-mainnet, custody, settlement, bridge or governance readiness.

## Target architecture versus current tree

The target architecture in
`trillionnium/docs/protocol/external-contracts-rust/RUST_NATIVE_EXTERNAL_CONTRACTS_ARCH_2026-03-05.md`
describes:

```text
contracts/
  sdk/
  runtime-spec/
  settlement-vault/
  bridge-relay/
  governance-guard/
  integration-tests/
```

Current reality is narrower:

```text
contracts/
  audit-events/
  settlement-vault/
  bridge-relay/
  governance-guard/
```

`audit-events` is a shared-schema neighbor. It is not a substitute for `sdk/` or
`runtime-spec/`. The target `sdk/`, `runtime-spec/` and `integration-tests/`
packages, canonical host ABI, deterministic sandbox, gas/storage model and
end-to-end replay remain absent.

## Authority boundaries

- `contracts/*` may define deterministic contract semantics and normalized audit records.
- Trillionnium World remains accountable for World game-domain state only.
- Nakama remains accountable for canonical online admission/order/recovery/signing.
- CEX remains accountable for wallet/ledger state and custody.
- Trillionnium Chain remains accountable for consensus, inclusion and finality.
- Integration remains accountable for exact cross-repository component locks and release evidence.

No in-memory role, balance, proof, finality value, pause flag or audit record in
this workspace is production authority. Fixture identities and local test state
cannot be promoted by documentation or naming.

## Module documentation

| Crate | Local role | Detailed design |
|---|---|---|
| `audit-events` | normalized audit schema | `docs/modules/contracts/audit-events-design.md` |
| `settlement-vault` | settlement state-machine MVP | `docs/modules/contracts/settlement-vault-design.md` |
| `bridge-relay` | relay state-machine MVP | `docs/modules/contracts/bridge-relay-design.md` |
| `governance-guard` | governance state-machine MVP | `docs/modules/contracts/governance-guard-design.md` |

Each design covers authority, interfaces, invariants, dependencies, concurrency,
persistence, idempotency, versioning, budgets, security, tests and honestly open
work. `docs/component-catalog.json` records the workspace and crates as
`mvp-perimeter` / `scope-dependent`, not as active game-product modules.

## Build and test boundary

The current workspace gate is:

```bash
cargo fmt --manifest-path contracts/Cargo.toml --all -- --check
cargo test --manifest-path contracts/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path contracts/Cargo.toml --workspace --all-targets --locked -- -D warnings
```

A green run proves only the current Rust crates under the exact tested source,
toolchain and dependency lock. It does not prove host ABI compatibility,
deterministic WASM execution, durable storage, Chain inclusion, custody or a
production deployment.

## Runtime and persistence boundary

The crates currently own no production host, database, network listener, key
store, workload identity or deployment package. A production adoption requires:

1. a versioned host ABI/runtime specification and generated SDK;
2. canonical schema, field dictionary, positive/negative/golden vectors;
3. deterministic build and sandbox behavior across supported toolchains/hosts;
4. explicit gas, storage, call-depth, memory and timeout budgets;
5. durable state delta and atomic audit publication;
6. crash/replay/rollback/PITR and old-writer fencing;
7. versioned RPC/event mapping and compatibility/retirement windows;
8. signed package/SBOM/provenance and independent security review;
9. exact World/CEX/Nakama/Chain/Integration component locks where applicable;
10. an explicit Day-1 scope and production authorization decision.

Until those are independently closed, the safe interpretation is:

```text
external_contracts_rust_direction=started
contract_semantics_mvp=true
canonical_host_runtime_connected=false
production_artifacts=false
public_mainnet_ready=false
production_authorization=not_granted
```

## Day-1 scope

The workspace is scope-dependent rather than an automatic public-launch P0:

- `settlement-vault` becomes launch-blocking only if the launch promise includes its vault/settlement semantics;
- `bridge-relay` becomes launch-blocking only if Day-1 includes cross-chain positioning;
- `governance-guard` supports future upgrade/pause discipline but is not deployed governance;
- `audit-events` is a shared schema and cannot independently promote any other crate.

Scope selection must come from `PROJECT_BOUNDARY.md`, `CURRENT_PLAN.md`,
`RELEASE_READINESS.md` and the accepted Integration release lock—not from the
existence of these directories.

## Change rule

A semantic change must update the relevant Rust types, state invariants, stable
errors, schema/vectors, per-crate design, resource limits, tests and migration or
retirement policy in the same pull request. A change may not add custody,
canonical online authority, Chain finality or public deployment through a local
state-machine API.
