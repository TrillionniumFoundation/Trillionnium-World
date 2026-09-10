---
status: current-candidate
owner: trillionnium-world-contracts
module: settlement-vault
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# `settlement-vault` detailed design

## Scope and authority

`settlement-vault` is the Rust MVP state machine for bounded settlement-vault semantics: authorization, deposit or lock identity, release, reversal/slash where defined, replay protection and normalized audit emission. It is not a wallet, ledger, custodian, deployed smart contract, Chain finality source or production settlement service. CEX remains authoritative for real wallet/ledger custody; World may only consume independently verified receipts.

## Public interfaces

The public surface consists of typed vault/operation/actor/asset identifiers, command/request records, in-memory state, decision/result/error values and deterministic methods for the currently implemented lock/release/slash lifecycle. Successful methods return explicit state effects and audit material. Callers cannot obtain authority by supplying display text or an arbitrary success flag. Exact names and methods must be reflected in generated API documentation and canonical vectors before host adoption.

## State model and invariants

Each vault or position has a unique stable identity, one supported contract version and an explicit lifecycle. Amounts are nonnegative, bounded and interpreted under a typed asset/currency identity. A release or slash cannot exceed the currently locked amount; terminal operations cannot be replayed under altered bytes; actor roles are explicit; and every accepted mutation preserves total-accounting invariants defined by the MVP. Errors leave the complete state unchanged. The in-memory state cannot represent custody outside its test process.

## Dependency direction

The crate may depend on `audit-events`, deterministic serialization/hashing and bounded standard collections. It must not import CEX, World, Chain or Nakama implementations, network clients, SQL, filesystem, wall clock, environment credentials or a host runtime. A future host adapter authenticates, meters, persists and commits commands around this pure state machine. The state machine never calls outward while mutating.

## Execution, concurrency and cancellation

Current execution is synchronous and single-owner. It starts no tasks and owns no external locks. A command evaluates against one private candidate and publishes only after all checks pass. Concurrent host callers require an expected revision plus atomic compare-and-swap or one serialized actor; this is a host responsibility. Cancellation before publication drops the candidate. Ambiguous durable commit is resolved by immutable request lookup, not blind replay.

## Persistence and durability

Current state is in-memory test material. Production adoption requires canonical state encoding, durable storage delta, immutable request/result records, atomic state-plus-audit publication, crash recovery, backup/PITR and old-writer fencing. A process restart currently does not prove durable continuity. No README, unit test or serialized fixture may be cited as evidence that funds are held.

## Idempotency, retry and failure taxonomy

Every mutation uses an immutable operation/idempotency identity bound to actor, vault, asset, amount, action, expected state/revision and contract version. Exact duplicates may return the prior decision; altered reuse fails. Failures distinguish malformed/unsupported input, unauthorized actor, missing position, invalid phase, insufficient locked amount, overflow/bounds, duplicate-altered conflict and integrity failure. Transport, host and storage ambiguity are separate runtime families.

## Versioning and migration

Contract semantics, host ABI, state schema, asset policy, audit schema, storage migration and deployment release are independent versions. Any change to role rules, amount arithmetic, lifecycle transitions, idempotency binding, error precedence or canonical bytes requires a new contract/state version, migration and rollback, positive/negative vectors and old/new reader tests. A future WASM artifact must bind source, toolchain and host capability version.

## Resource and performance budgets

Identifier lengths, positions per vault/account, operation history, metadata, serialized state and command bytes are bounded. Arithmetic is checked and deterministic. Validation/mutation must be linear in the bounded affected records and avoid unbounded history scans. Host gas, storage quota, call timeout and concurrent admission are not implemented by this crate and remain explicit open contracts.

## Observability and security

Audit/metrics may expose stable operation/vault/asset IDs and bounded result codes but not credentials, private keys, complete personal data or unbounded metadata. Authorization values supplied by tests are fixtures, not workload identity. Production requires least-privilege host calls, signed package/provenance, independent review, key/custody separation, incident controls and reconciliation with CEX/Chain accountable truth.

## Test and evidence traceability

Primary source is `contracts/settlement-vault/src/`. `contracts/README.md` defines the MVP boundary; `docs/adr/0002-transaction-free-external-settlement.md` defines the separate World/CEX settlement boundary and must not be conflated with this state machine. Tests cover accepted lifecycle paths, unauthorized calls, over-release/slash, overflow, duplicate exact/altered requests, error-state preservation and normalized audit events. Host/storage/fault/custody evidence remains absent.

## Change checklist and open work

A change states affected roles, assets, accounting invariant, lifecycle, idempotency, error, version and resource limits; updates tests/vectors/audit docs together; and receives security/economy review. Open work includes machine schema and vectors, stable error catalogue, host ABI/runtime-spec, deterministic WASM build, gas/storage metering, durable repository, atomic audit outbox, Chain/CEX integration, formal accounting properties, independent audit and explicit Day-1 scope decision.
