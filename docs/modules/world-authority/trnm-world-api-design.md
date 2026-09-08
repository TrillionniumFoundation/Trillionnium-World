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

This crate is the internal versioned application boundary between deterministic World domain/command/projection code and identity, session, account, repository, ledger, evidence, metrics and server adapters. It defines DTOs and traits; it does not prove that a production implementation exists. It cannot bypass Nakama canonical online admission/order/recovery/signing, CEX wallet/ledger custody, Chain finality or Integration release locks. Fixture adapters remain test-only.

## Public interfaces

The public surface includes home, command, route, map, tactics and full-split request/response values; actor/session/account/ledger/repository/evidence/metrics receipts/status; stable API and adapter contract constants; and traits such as `WorldIdentityAdapter`, `WorldSessionGuard`, `WorldAccountAdapter`, `WorldLedgerAdapter`, `WorldRepository`, `WorldEvidenceSink` and `WorldMetricsSink`. DTOs carry explicit API/domain/component versions, source-of-truth, immutable IDs, expected/current revisions and bounded decisions.

## State model and invariants

The crate itself is stateless. A response marked accepted must contain complete matching actor/account/session/aggregate/request/revision/contract information. Repository state, ledger effect and evidence emission are separate outcomes; one cannot imply another. A ledger receipt is not a World apply, and World state is not CEX success. Fixture values explicitly identify their fixture source. Unknown or crossed contracts, missing required identity, altered retry, stale revision/lease and invalid source-of-truth fail closed.

## Dependency direction

`trnm-world-api` depends inward on domain, command, projection and map-provider value types plus serialization. It does not implement sockets, SQL, files, clocks, environment configuration, token verification, secrets, remote CEX calls or canonical ordering. Production adapters live in owning runtime packages and implement the traits under least-privilege, timeout, transaction, fencing and observability contracts. Domain packages never import API or adapters.

## Execution, concurrency and cancellation

Trait calls may be synchronous in the current candidate, but production implementations must document blocking and async behavior. Identity/session checks and expected revision are completed before command execution. Repository commit is a short atomic/fenced operation. Ledger/remote work is transaction-free with respect to mutable World rows. Evidence/metrics failure behavior is explicit. Cancellation before repository commit publishes no state; uncertainty after commit is resolved through immutable request/event lookup rather than command replay.

## Persistence and durability

The API owns no store. `WorldRepository` implementations persist canonical aggregate bytes/hash, revision, immutable request/event/decision and writer epoch. `WorldLedgerAdapter` represents separately authoritative ledger interaction and must support exact lookup-before-submit semantics. Evidence sinks retain immutable bounded records; metrics sinks are non-authoritative. Production adapters require PostgreSQL catalog/privilege/fencing/replay conformance. The file repository and fixture receipts are never production defaults.

## Idempotency, retry and failure taxonomy

Each mutation request binds immutable request ID, actor/account/session context, expected aggregate revision, API/domain/command versions and canonical command bytes. An exact duplicate returns the recorded decision/receipt; altered reuse fails. Stable API errors distinguish decode/contract, identity/session/audience, authorization, stale revision/epoch/lease, domain rejection, repository conflict/ambiguity, ledger ambiguity/permanent failure, evidence/metrics degradation, capacity/rate and internal integrity. Free-form diagnostics do not drive authority behavior.

## Versioning and migration

API DTO/trait, domain/command/projection/map, repository event/schema, identity/session, ledger receipt and server transport are independent versions. Required field, identity binding, method semantics, error/retry or source-of-truth changes require a new API/adapter contract, generated schema/vectors, old/new adapter conformance, rollout/rollback and component-lock updates. A new method or DTO does not create a public route without an ADR, Nakama integration and owner approval.

## Resource and performance budgets

DTO body, identifier/text, map/route/task/projection, evidence diagnostic and response bytes/count/depth are bounded. Adapters apply timeout, connection, in-flight, queue and retry ceilings; the API does not expose unbounded collections. Identity/repository/ledger/evidence operations have separate latency budgets so readiness can isolate failure domains. Benchmarks cover maximum valid requests/responses, exact duplicate/stale conflict, adapter fan-out and serialization/allocation overhead.

## Observability and security

Audit records bind API/domain/component versions, request/actor/account/session/aggregate/revision/epoch, command/decision, repository/ledger/evidence result and bounded code. Tokens, credentials, signatures treated as secrets and unbounded remote bodies are redacted. Metrics distinguish identity/session rejection, stale/conflict, domain decision, repository commit/ambiguity, ledger lookup/submit/result and evidence degradation. Trait implementations are reviewed for audience separation, least privilege and no authority laundering.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-api/src/lib.rs`. Normative neighbors are ADR-0003, domain/command/projection/map/server designs, `docs/contracts/trillionnium-world-authority-cutover-v1.json`, provenance JSON and PostgreSQL cutover contracts. Unit/adapter tests and `scripts/check-trnm-world-authority-cutover.sh` cover DTO serialization, source/contract binding, fixture labels, every trait role, identity/account/session separation, repository/ledger separation and malformed/crossed outputs. Production adapters require black-box tests.

## Change checklist and open work

A change identifies DTO/trait/identity/revision/idempotency/transaction/security/resource effects; updates schemas, vectors, adapter implementations, runbooks and rollback; adds exact duplicate/altered/stale/timeout/cancel/ambiguous-result tests; and obtains all affected-owner reviews. Open work includes generated OpenAPI/JSON Schema, stable error/recoverability catalogue, asynchronous production adapter contract, PostgreSQL repository implementation, workload identity/session integration, CEX ledger conformance, immutable evidence sink and Nakama-owned canonical caller.
