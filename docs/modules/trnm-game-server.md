---
status: current-candidate
owner: Trillionnium World compatibility server and settlement operations
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-game-server technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

An Axum/Tokio/PostgreSQL server with compatibility online APIs, product/operations surfaces, migrations, host journals, fleet fencing, terminal recovery, settlement capture/apply integration, a separately invoked worker binary and signer protocol support. Correctness source is ordinary Git-tracked `lib_parts` included from a small root.

## Responsibilities and non-responsibilities

Bounded `world_legacy_local_alpha` compatibility enclave. It is not the target public canonical online authority and cannot sign canonical completion or own wallet custody.

## Public interfaces and data contracts

HTTP routes are inventoried in `docs/protocol/trnm-world-http-api-v1.md`; stream behavior is in `trnm-world-websocket-v1.md`; stable errors are in the error catalogue; database behavior is governed by `docs/database/trnm-world-postgres-contract-v1.md`. Private moderator/worker/signer audiences are distinct.

## State model and invariants

State is partitioned into configuration/migrations, authority actor, campaign persistence, terminal journals, fleet/host fences, settlement capture/jobs/quarantine, product/operations rows and readiness. The global lock order is normative. No remote signer/CEX/network call may execute while mutable match/campaign rows are locked.

## Dependency direction

May use Axum, Tokio, SQLx/PostgreSQL, bounded HTTP/WebSocket clients and game-domain crates. HTTP handlers authenticate/validate then call application services; persistence owns SQL; transport owns remote I/O. New sibling-repository path dependencies and Nakama private-key access are forbidden.

## Concurrency and cancellation

One logical actor serializes each compatibility match. Database leases/fences reject stale processes and workers. Global lock order prevents inversion. Bounded channels, connection pools, stream registries and in-flight settlement tasks apply backpressure. SIGTERM stops admission before bounded drain; cancellation leaves lease-recoverable state.

## Durability and recovery

Migrations are append-only/checksum-bound. Authority publication, terminal ACK, capture and apply use explicit transactions. Hot/cold journal witnesses and database rows are reconciled on restart. Remote success and campaign application are separate durable states. PITR/failover requires timeline and old-primary isolation.

## Failure, retry and idempotency

Missing migrations, fence loss, journal ambiguity, poisoned durable state, unsupported versions or uncertain remote success make readiness false. Malformed/conflict/response-loss outcomes use exact lookup-before-submit. Poison scope is quarantined; unrelated accounts continue. Stale leases cannot persist or apply.

## Versioning and migration

Endpoints, schemas, lock order, journal format, migrations, settlement identity and credentials are versioned. Migrations are immutable; repairs use a new migration. Semantic build-time source generation is forbidden. Retirement requires Nakama cutover, drain, rollback and disablement evidence.

## Resource budgets and performance

Request bodies, response bodies, WebSocket frames, connections, per-match commands, actor/channel queues, database pools, lease durations, diagnostics and shutdown grace are bounded. SLOs track readiness, p95/p99 route latency, tick/command commit latency, pool saturation, settlement age, dead letters and stream backpressure.

## Observability and security

Structured logs/metrics carry route, bounded actor/match/account digests, generation, lease, migration, cursor, state hash and error code. Credentials, bearer tokens and raw signed value material are redacted. Security includes least privilege, SSRF/redirect rejection, body limits, stale-primary fencing and immutable evidence.

## Verification traceability

Source-boundary, all-target, PostgreSQL black-box, contention, stale lease, malformed/response-loss, terminal corruption, shutdown/drain, PITR/timeline, resource and exact-head tests are required. Public/cross-host/custody claims require external evidence and independent review.

## Known gaps and change checklist

Replace textual `include!` ownership parts with true Rust modules/traits without changing semantics; generate route/type/OpenAPI conformance; complete cross-host/PITR matrices; bind a qualified CEX revision; and retire canonical admission after Nakama cutover. Each extraction must preserve tests and narrow visibility.
