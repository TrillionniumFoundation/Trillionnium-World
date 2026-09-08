---
status: current-candidate
owner: Trillionnium World game economy
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-economy-protocol technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

A stateless Rust library exporting account and asset references, intent and receipt types, idempotency identities, wallet snapshots, progression classes, settlement backend identifiers, and V1/V2 signed value-entitlement envelopes. Serialization is serde-based; callers perform transport, signature verification and persistence.

## Responsibilities and non-responsibilities

Defines game-owned immutable economic intents and receipt vocabulary. CEX remains the system of record for wallet, ledger, custody and irreversible value movement.

## Public interfaces and data contracts

Primary families are `term_exchange_protocol_v2`, `term_exchange_backend_v2`, `trnm_economy_policy_v1`, `trnm_server_signed_value_entitlement_v1`, and `trnm_server_signed_value_entitlement_v2`. The stable boundary is the serialized value plus validation result; display strings and local soft-credit balances are never authority inputs.

## State model and invariants

The crate owns no mutable or durable state. An idempotency key is immutable and cannot be reused with different canonical intent bytes. A receipt must agree on request identity, actor/account binding, backend, amount, currency, progression class and immutable intent hash. Local soft credits are non-convertible unless an explicit versioned `DualTrack` policy says otherwise.

## Dependency direction

May depend only on deterministic serialization and hashing libraries. It must not depend on campaign, client, server, SQL, HTTP, filesystem, environment, wall clock, randomness, key stores or CEX implementation crates. Campaign and adapters may depend on it, never the reverse.

## Concurrency and cancellation

There are no tasks, locks or mutable globals. Thread safety follows the exported value types. Ordering, claim serialization and lease fencing belong to settlement runtimes. Any future cache must be immutable or caller-owned and must not change validation outcomes.

## Durability and recovery

The library does not persist. Callers must durably record the exact intent before remote execution and record a verified receipt before campaign application. Recovery uses exact receipt lookup with the same immutable request identity; it never fabricates success from timeout or a locally cached wallet value.

## Failure, retry and idempotency

Validation is side-effect free and returns bounded typed errors. Unknown algorithm, issuer, version, currency, backend, progression class, invalid time window, audience mismatch, altered duplicate or signature mismatch fails closed. Transport timeout, malformed success, conflict and response loss are ambiguous settlement-runtime states and require lookup-before-submit.

## Versioning and migration

Changing signing bytes, required fields, enum meaning, hash domain or validation precedence requires a new contract version. Additive optional fields need old-reader/new-writer and new-reader/old-writer fixtures. Every retirement must name an owner, usage inventory, last accepted date, rollback policy and cross-repository component lock.

## Resource budgets and performance

Serialized intent and receipt sizes must remain bounded by the enclosing HTTP/database contracts. Identifiers are bounded UTF-8 values and diagnostic strings must exclude credentials. Validation must be linear in payload size, allocate only bounded collections and perform no network or disk I/O. Concrete byte ceilings are frozen in the protocol schema before external promotion.

## Observability and security

The crate emits no logs or metrics to avoid leaking value or identity material. Runtimes record normalized event type, immutable request ID, backend, bounded status code and digest, never secrets or full bearer material. Security review focuses on canonical signing bytes, replay domains, audience separation and downgrade rejection.

## Verification traceability

Unit coverage must include round trips, canonical signing fixtures, altered retry rejection, account/audience mismatch, amount and validity bounds, unknown enum rejection, V1/V2 compatibility and duplicate receipt handling. Cross-repository evidence must prove World, signer, CEX and Integration agree on exact bytes and one durable value effect.

## Known gaps and change checklist

Publish generated JSON Schema and golden signing vectors; bind a CEX-selected immutable implementation; add a compatibility/retirement inventory; and attach deployed ambiguity, custody and key-rotation evidence. A change touching a public type, hash input, limit or validation rule must update this design, vectors and the component lock in the same review stack.
