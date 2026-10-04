---
status: current-candidate
owner: trillionnium-world-compatibility-protocol
module: trnm-online-protocol
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-online-protocol detailed design

## Scope and authority

`trnm-online-protocol` defines versioned request, response, snapshot, delta, command-receipt, reconnect, lobby/product, operations, and production-status values used by the native client and World-local compatibility enclave. It is a compatibility vocabulary, not the target canonical online authority. Nakama owns participant admission, total order, durable idempotency, restart recovery, canonical archive roots, and completion signing.

## Public interfaces

Message families cover campaign connection, lobby creation/access/invite/ready/queue, match creation/join/start/access, command submission/receipt, full snapshot, top-level delta, reconnect and receipt-gap recovery, inventory/member views, moderator/operations status, and production capability/build reports. Protocol identity and build identity are separate. Requests bind the player/account/campaign/match, expected revision/generation, per-player input sequence, observed tick/base sequence, and typed RTS order where applicable.

## State model and invariants

The crate is stateless, but its message relationships are strict. A full snapshot binds match/generation/revision/tick/sequence and decoded World-state hash. A delta binds the exact base and resulting sequence/state identities. A receipt binds immutable command identity, actor/controller, accepted/rejected outcome, authoritative tick and public cursors. Reconnect either proves a bounded continuous receipt gap from a durable cursor or returns an exact full snapshot. Crossed protocol/build/capability combinations and ambiguous truncation fail closed.

## Dependency direction

Only deterministic serialization, bounded collections, UUID/identity types as approved, and RTS command types are allowed. The protocol must not open sockets, read files/environment/credentials, access SQL, perform authentication, mutate state, or implement retry loops. Native client, World compatibility server, and transition adapters consume it. Future canonical Nakama APIs are owned and versioned in Nakama; they may map to World transition contracts but must not be silently expanded through this crate.

## Execution, concurrency and cancellation

Serialization/validation is synchronous and side-effect free. Transport owners manage asynchronous reads/writes, queues, cancellation, ping/timeout, and reconnect. They must preserve per-connection frame order and bound in-flight messages. Cancellation before a complete verified frame publishes nothing. A slow or malformed connection cannot block unrelated matches. Sequence and generation checks occur before a client/server exposes a message as authoritative; speculative/private state never advances public cursors.

## Persistence and durability

This crate persists nothing. The canonical owner durably stores admitted command identity/order, snapshots/archive roots, member state, receipts, and reconnect cursors. The World compatibility enclave may store its laboratory generation and PostgreSQL/journal evidence, but those records are not Nakama canonical evidence. Clients persist only bounded command-attempt journals and last verified cursors; local files cannot establish participant identity or completion. Every stored wire object retains protocol/build/component identity.

## Idempotency, retry and failure taxonomy

Exact command retries use the same immutable request/sequence/order bytes. Altered duplicates, stale generation/revision, sequence gaps, unauthorized controller, unsupported capability, hash/base mismatch, malformed/truncated frame, and crossed build/protocol are stable failures. Transport loss is recoverable only through reconnect/full snapshot or a proven continuous receipt gap. An error response states whether retry, resync, reconnect, reauthenticate, or permanent rejection is required; free-form text is diagnostic and never drives client authority decisions.

## Versioning and migration

Authority protocol, product/lobby protocol, operations protocol, production status protocol, build identity, World transition contract, and release provenance are separate version axes. Breaking message/enum/required-field/sequence/hash behavior requires a new protocol identity and explicit client/server/Nakama admission matrix. Additive optional fields require declared unknown-field and old-reader behavior. Legacy V2/V3 support needs usage inventory, owner, telemetry, rollback path, start/end dates, and a removal gate.

## Resource and performance budgets

HTTP bodies, WebSocket frames, identifiers, members, invitations, orders, receipt gaps, deltas, snapshots, inventory stacks, pagination, diagnostics, and queues must have explicit byte/count/time ceilings. Decoding validates limits before large allocation. Backpressure and per-match/per-connection/global concurrency budgets are owned by transport runtimes and exposed in readiness. Benchmarks cover maximum valid frames, malformed/deep input, fan-out, reconnect storms, snapshot/delta frequency, serialization latency, and memory.

## Observability and security

Telemetry binds protocol/build/capability, endpoint/message class, match/generation/sequence/tick ranges, result code, frame bytes, queue/backpressure, reconnect/resync reason, and hash verification without logging session tokens or private payloads. Authentication/audience and authorization occur outside the value crate. Unknown fields/versions/capabilities fail according to the published compatibility policy. Client-supplied snapshots, results, balances, receipts, and completion claims remain untrusted.

## Test and evidence traceability

Primary source is `trillionnium/crates/trnm-online-protocol/src/lib.rs`. Normative neighbors include `docs/protocol/trnm-world-http-api-v1.md`, `trnm-world-websocket-v1.md`, `trnm-world-error-catalog-v1.md`, `trnm-world-compatibility-matrix-v1.md`, system-context/authority ADRs, and client/server module designs. Tests cover every message round trip, limits, unknown/duplicate/crossed fields, full/delta/reconnect relationships, pagination/truncation, duplicate races, capability negotiation, old-reader behavior, and exact client/server component-lock conformance.

## Change checklist and open work

Any change updates message catalogue, JSON Schema/OpenAPI/AsyncAPI, stable errors/recoverability, examples/vectors, client/server adapters, compatibility matrix, telemetry, and retirement dates; then runs exact-head and cross-repository conformance. Open work includes generated schemas for all current messages, a complete sequence/generation/reconnect state diagram, bounded backpressure rules, explicit V2/V3 retirement, and replacement of compatibility-only APIs by Nakama-owned canonical contracts.
