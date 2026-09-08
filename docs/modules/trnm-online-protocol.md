---
status: current-candidate
owner: Trillionnium World compatibility wire vocabulary
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-online-protocol technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

A stateless Rust library for compatibility-enclave requests, responses, snapshots, deltas, receipts, reconnect, lobby/product, operations and production-status messages. It retains explicit V2/V3 authority/build identities for rolling laboratory compatibility.

## Responsibilities and non-responsibilities

Compatibility message vocabulary only. Nakama owns target participant admission, canonical total order, idempotency, restart recovery, archive roots and completion signing.

## Public interfaces and data contracts

Protocol identity and build identity are separate. Messages bind player/account/campaign/match IDs, revisions, per-player sequence, global cursor, generation, observed tick, hashes and typed RTS orders as applicable. The route catalogue and WebSocket envelope are documented under `docs/protocol/`.

## State model and invariants

The crate owns no runtime state. IDs and collections are bounded; enum spellings and optional-field behavior are explicit. Snapshot/delta/reconnect relations bind base/current sequence, generation, tick and state hash. Crossed protocol/build pairs, stale generation and continuity gaps fail closed.

## Dependency direction

Only serialization, deterministic collections and RTS order types are allowed. The crate does not open sockets, access files/databases, read credentials or decide authority. Client and server adapters implement authentication, rate limiting and transport around these values.

## Concurrency and cancellation

Message construction and validation are synchronous and reentrant. Ordering is represented as explicit fields, never inferred from arrival time. Concurrent adapters must serialize per-session/per-match authority actions and keep decode buffers bounded. The library uses no mutable global state.

## Durability and recovery

Persistence belongs to the runtime. Canonical authorities durably bind admitted messages and cursors. Clients persist only retry journals and last verified snapshot metadata, which do not become authority. Reconnect falls back to an exact full snapshot when suffix continuity cannot be proven.

## Failure, retry and idempotency

Unknown/crossed versions, malformed IDs, sequence gaps, altered duplicates, hash mismatch, stale generation, truncation ambiguity and unsupported capabilities return stable errors. Retry metadata distinguishes retry-same, resync, reauthenticate and permanent rejection. No error path silently downgrades to a legacy contract.

## Versioning and migration

Breaking wire changes require a new protocol identity and an admission matrix. Each legacy pair has an owner, usage inventory, retirement date and rollback policy. Canonical Nakama APIs are defined in the owning repository and must not be expanded through this compatibility crate.

## Resource budgets and performance

Frames, identifiers, deltas, receipt gaps, lobby members, queues and pagination windows are bounded. Transport rejects oversized frames before full buffering, applies backpressure and caps reconnect suffix work. Serialization/validation is linear in accepted frame size.

## Observability and security

Record protocol/build pair, route, bounded result code, frame length, sequence/generation and digest. Never log bearer/session tokens or full private payload by default. Security review covers downgrade, replay, cursor confusion, decompression/amplification and reflected diagnostics.

## Verification traceability

Every message class requires serialization fixtures, full/delta/reconnect negative vectors, pagination/truncation, duplicate races, crossed versions, queue/frame boundaries, old-reader behavior and exact client/server component-lock conformance.

## Known gaps and change checklist

Publish generated JSON schemas for all message families; complete route-to-type inventory; declare V2/V3 retirement dates; add stream backpressure and reconnect load evidence; and align the Nakama adapter to a separately owned canonical API. Current compatibility types do not grant public-online authority.
