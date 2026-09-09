---
status: current-candidate
owner: trillionnium-world-contracts
module: audit-events
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# `audit-events` detailed design

## Scope and authority

`audit-events` defines the shared, normalized Rust vocabulary emitted by the external-contract MVP crates. It standardizes bounded event identity, contract/module identity, operation class, actor/subject references, outcome/reason, sequence or revision references and structured attributes. It owns schema shape only. An event does not authenticate a caller, authorize a mutation, prove durable append, establish settlement, create Chain finality or grant production evidence.

## Public interfaces

The public surface consists of serializable audit-event records, typed event/result/reason values where present, constructor/validation helpers and the current schema/version constant. Producers pass already-decided contract outcomes and stable machine identifiers. Consumers receive a normalized record; human text is diagnostic and cannot be parsed to reconstruct authority. Exact field names and enum spellings are compatibility material and require vectors before external adoption.

## State model and invariants

The crate owns no mutable state. A valid event has one supported schema version, nonblank bounded stable IDs, an explicit source contract and operation, deterministic attribute ordering, a bounded timestamp/sequence representation supplied by the accountable runtime, and one unambiguous outcome. Duplicate attribute keys, unsupported versions, oversized text/maps, secrets and opaque authority payloads are invalid. An audit record cannot assert a success that the producing state machine did not return.

## Dependency direction

The package remains below settlement-vault, bridge-relay and governance-guard. It may depend on deterministic serialization and standard bounded data structures, but not on the other contract crates, World/Nakama/CEX/Chain implementations, SQL, HTTP, filesystem, wall clock, randomness, credentials or deployment configuration. Durable sinks and transport encoders depend on this crate, never the reverse.

## Execution, concurrency and cancellation

Construction and validation are synchronous, deterministic and side-effect free. The crate starts no tasks and owns no locks. Concurrent callers may construct records independently. Cancellation drops a private value. A runtime publishes an event only after the associated state decision reaches its declared durable boundary; this crate cannot make that boundary atomic.

## Persistence and durability

This crate performs no I/O. A production sink must append the canonical serialized event with immutable identity, source revision, component lock and integrity hash in the same transaction or an explicitly reconciled outbox as the accountable mutation. In-memory vectors and unit-test collections are not durable evidence. Sink loss, ambiguous append and replay gaps are runtime failures and cannot be converted to successful auditing by this value crate.

## Idempotency, retry and failure taxonomy

An immutable event ID is permanently bound to the same canonical event bytes and source decision. Exact duplicates may converge in a durable sink; altered reuse is an integrity conflict. Validation failures distinguish unsupported schema, malformed or duplicate identifiers/attributes, bounds violation, invalid outcome/reason combination and forbidden sensitive field. Retry policy belongs to the sink and must not invent a new semantic event for one already-committed mutation.

## Versioning and migration

Schema version, producer contract version, source state revision, storage format and release identity are separate dimensions. Renaming a field/enum, changing required/optional meaning, canonical order, identifier domain, outcome semantics or limits requires a new audit schema, old/new readers, positive/negative vectors, sink/index migration and retirement window. Additive fields need an explicit unknown-field policy.

## Resource and performance budgets

Event bytes, IDs, labels, attribute count, key/value lengths and nesting depth must be explicitly bounded. Validation and canonical serialization must be linear in bounded input and deterministic. Metric labels derived from events use an allow-list and cannot reflect arbitrary actor/subject text. A larger event or attribute budget requires storage/index, denial-of-service and privacy analysis.

## Observability and security

Audit events contain stable non-secret references and bounded machine codes. They must exclude bearer/session credentials, private keys, passwords, cookies, complete remote bodies and unnecessary personal data. Redaction is performed before construction and verified by tests. An event marked fixture, test or in-memory cannot be presented as immutable production evidence.

## Test and evidence traceability

Primary source is `contracts/audit-events/src/`. The workspace boundary and honest status are defined by `contracts/README.md` and this index. Tests must cover deterministic round trips, stable spellings, duplicate/oversized/unsupported values, secret-like field rejection where supported, exact duplicate identity and cross-crate fixture emission. Future sink conformance requires golden bytes, database/outbox fault tests and independent review.

## Change checklist and open work

Changes update Rust types, field dictionary, schema, vectors, all producers/consumers, storage/index effects, limits and privacy review together. Open work includes a generated JSON Schema, canonical byte vectors, stable global error/event catalogue alignment, an immutable production evidence sink, retention/deletion policy, cross-repository consumer conformance and exact release component locks.
