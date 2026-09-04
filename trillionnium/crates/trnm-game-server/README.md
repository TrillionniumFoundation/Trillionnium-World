# trnm-game-server

Status: current candidate / compatibility enclave  
Owner: Trillionnium World server and settlement operations  
Authority profile: `world_legacy_local_alpha`

## Purpose

This crate provides the World-local compatibility server, private operations surfaces, PostgreSQL persistence and migrations, host journals, settlement capture/apply integration, an independently deployed settlement worker, and entitlement signer protocol support needed for laboratory, migration, drain, rollback, and fault evidence.

## Authority and non-goals

It is not the target public canonical online authority. It must not create a second authority generation, load or proxy a Nakama private key, sign canonical `MatchCompletedV1`, claim Chain finality, own wallet custody, or enable a public player market. New canonical admission and order belong to Nakama.

## Public contracts

HTTP and WebSocket behavior is governed by `docs/protocol/trnm-world-http-api-v1.md`, `trnm-world-websocket-v1.md`, and the stable error and compatibility catalogues. PostgreSQL behavior is governed by `docs/database/trnm-world-postgres-contract-v1.md`. External settlement follows capture transaction, transaction-free remote execution, and fenced apply transaction.

## State and invariants

Correctness ownership is partitioned into authority/actor, configuration and migrations, terminal recovery, fleet fencing, identity, application, HTTP routing, readiness, product APIs, campaign persistence, operations, and tests. Global database lock order and live instance/lease generations are mandatory. No signer, CEX, wallet, ledger, or network call may execute while mutable match or campaign rows are locked.

## Dependencies and boundaries

The server may use Axum, Tokio, SQLx/PostgreSQL, bounded HTTP/WebSocket clients, and game-domain crates. Game-server request handlers do not perform settlement remote I/O; the separate worker owns lookup/submit and persists results only under a live exact lease. Runtime configuration uses distinct game, moderator, worker, and signer audiences.

## Failure and recovery

Missing migrations, fence loss, journal ambiguity, poison durable state, unsupported contract versions, or uncertain remote success make readiness false. Response loss and malformed/conflict success are recovered through exact lookup-before-submit. Stale workers and old primaries cannot mutate after takeover. Poison scope is quarantined so unrelated accounts continue.

## Testing and evidence

Required evidence includes source-boundary tests, all-target unit tests, PostgreSQL black-box migrations and procedures, two-worker contention, stale lease rejection, response-loss and kill/cancel matrices, terminal journal corruption, shutdown/drain, PITR/timeline/old-primary isolation, resource limits, and exact-head hosted logs/artifacts. Local tests do not grant public, custody, or cross-host credit.

## Compatibility and change control

Migrations are append-only and checksum bound. Endpoint, schema, lock-order, journal, settlement, authority, or credential changes require corresponding protocol/database/runbook updates and rollback impact. Semantic Rust source generation is forbidden; correctness code must be ordinary reviewable source. Retirement requires Nakama cutover, active-match drain, rollback evidence, and disablement rehearsal.
