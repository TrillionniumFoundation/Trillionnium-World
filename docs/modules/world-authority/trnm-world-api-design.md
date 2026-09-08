---
status: current-candidate
owner: trillionnium-world-api
module: trnm-world-api
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-world-api detailed design

## Scope and authority

This crate is the internal versioned application boundary between deterministic World domain/command/projection code and identity, session, account, WorldRepository, WorldLedgerAdapter, evidence, metrics, routing and deployment adapters. It defines DTOs and contracts; it does not prove a production implementation exists. It cannot bypass Nakama canonical online admission/order/recovery/signing, CEX wallet/ledger custody, Chain finality or Integration release locks. Fixture adapters remain test-only.

## Public interfaces

The legacy `trillionnium_world_runtime_adapter_v1` interface contains synchronous fixture-oriented DTOs and traits used by parity tests. The fail-closed `trillionnium_world_production_adapter_v1` interface adds `WorldProductionAdapterResult<T>`, stable error codes, credential references, identity/session/account request objects, immutable mutation context, repository load/lookup/commit, ledger lookup-before-submit, evidence append, bounded metrics, internal routing authorization and deployment identity. No implementation is bundled or implied.

## State model and invariants

The crate itself is stateless. Production mutation context binds request, actor, account, session generation, aggregate, expected revision, writer epoch, API/domain/command versions and command SHA-256. Repository commit, ledger effect, evidence append and metric record are independent results; one cannot imply another. Exact duplicate identities may converge to the recorded receipt, while altered reuse, stale revision, stale epoch, audience mismatch and crossed contracts fail closed.

## Dependency direction

`trnm-world-api` depends inward on domain, command, projection and map-provider value types plus serialization. It does not implement sockets, SQL, files, clocks, environment configuration, token verification, secrets, remote CEX calls, canonical ordering or deployment. Production implementations belong in owning runtime and repository packages and satisfy these traits under least-privilege, timeout, transaction, fencing, exact component-lock and observability contracts. Inner domain packages never import adapters.

## Execution, concurrency and cancellation

The legacy fixture traits remain synchronous and infallible for deterministic smoke construction only. Production traits are fallible and consume immutable request structs instead of loose argument lists. Identity, audience, session generation, writer epoch and expected revision are validated before a command candidate can be committed. Remote ledger execution occurs without mutable World rows locked. Cancellation before commit publishes no state; uncertainty after commit is resolved by immutable request/event lookup rather than blind replay.

## Persistence and durability

The API owns no store. `WorldProductionRepository` loads an exact aggregate snapshot, looks up a prior request identity and commits a candidate carrying next revision, state hash, route records and writer epoch. `WorldProductionLedgerAdapter` performs exact receipt lookup before any ambiguous resubmission. Evidence sinks append immutable hash-bound records; metrics are explicitly non-authoritative. Production implementations require PostgreSQL catalog, privilege, fencing, replay, PITR and old-primary conformance. File repositories and fixture receipts receive no production credit.

## Idempotency, retry and failure taxonomy

Each operation has a stable request identity and typed result. `WorldProductionAdapterErrorCode` distinguishes invalid input, unsupported contract, unauthenticated/unauthorized/audience mismatch, stale session/epoch/revision, identity conflict, repository unavailable/conflict/ambiguous commit, ledger unavailable/ambiguous/permanent rejection, evidence unavailable, metrics degraded, routing rejected, capacity, shutdown and internal integrity. The `retryable` bit is explicit; safe diagnostics cannot drive authority or contain secrets.

## Versioning and migration

Legacy fixture contracts and the production adapter contract have separate identities. Required field, trait method, hash binding, error meaning, retry semantics, source-of-truth, routing or deployment identity changes require a new version, generated schema and vectors, old/new conformance, rollout/rollback plan and exact component-lock update. Introducing a new internal DTO does not create a public route. Fixture compatibility cannot silently satisfy a production reader or writer contract.

## Resource and performance budgets

Request IDs, actors/accounts/sessions, aggregate IDs, component locks, routes, diagnostics, label counts and text lengths require bounded validation in implementations. Repository and ledger operations have independent timeouts, pool/in-flight/queue/retry limits and latency budgets. Evidence payloads are hash references rather than unbounded bodies; metric labels are bounded vectors. Benchmarks cover maximum valid DTOs, serialization/allocation, exact duplicate lookup, stale conflicts and adapter fan-out at representative aggregate sizes.

## Observability and security

Audit records bind adapter/API/domain/component versions, request, actor/account/session/generation, aggregate/revision/epoch, command hash, repository event/receipt, ledger lookup/submit/result, evidence append and bounded error code. `WorldCredentialReference` carries only credential identity, issuer and audience, never token/password/cookie/private-key values. Production routing is accepted only for authenticated Nakama/internal caller identity and exact component lock. Source declarations cannot grant workload identity, deployment or production authorization.

## Test and evidence traceability

Primary sources are `trillionnium/crates/world-authority/trnm-world-api/src/lib.rs` and `src/production_contract.rs`. Normative neighbors are ADR-0003, domain/command/projection/map/server designs, `docs/contracts/trillionnium-world-authority-cutover-v1.json`, provenance JSON and PostgreSQL cutover contracts. `scripts/check-trnm-world-authority-cutover.sh` plus all-target tests cover serialization, stable error spellings, fixture labels, every legacy role, fail-closed production readiness and secret-field exclusion. Real adapters require black-box evidence.

## Change checklist and open work

A change identifies DTO/trait/identity/revision/idempotency/transaction/security/resource effects; updates schemas, vectors, adapter implementations, runbooks and rollback; adds exact duplicate, altered, stale, timeout, cancellation and ambiguous-result tests; and obtains affected-owner review. Open work is intentionally explicit: generated OpenAPI/JSON Schema, actual PostgreSQL repository, workload identity/session, CEX ledger, immutable evidence/metrics, Nakama/internal routing, deployment identity, exact-head qualification and external production authorization are all absent.
