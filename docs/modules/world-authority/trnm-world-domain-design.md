---
status: current-candidate
owner: trillionnium-world-domain
module: trnm-world-domain
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-world-domain detailed design

## Scope and authority

This crate is the innermost World-domain package for the cutover workspace. It defines the aggregate state, authored fixture records, stable World identifiers, transition semantics, and validation vocabulary consumed by command, projection, map, API, and server layers. It may become the system-of-record model for World business state after an approved cutover. It never owns Nakama admission, global online order, reconnect generations, archive roots or completion signing; CEX wallet/ledger custody; Chain finality; or Integration release truth.

## Public interfaces

The public surface is centered on `WorldState`, typed node/edge/position, character, NPC, task, skill, item, route and transition records, stable contract/version constants, exact lookup helpers, fixture constructors, and aggregate validation. Public values use explicit machine IDs and typed status fields. Display copy is not an identifier. Serialized state must carry the World-domain and transition-semantics identities needed by a reader to reject crossed versions rather than guessing compatibility.

## State model and invariants

A valid aggregate has one explicit contract version; unique, nonblank and bounded IDs; deterministic collection ordering; valid references from edges, positions, tasks, characters, NPCs, skills and items; legal node/transition topology; and bounded numeric/text/collection fields. Referential integrity is checked before a state is publishable. Authority-like keys from a caller cannot be hidden inside opaque metadata. A decoded or migrated state is either completely valid under one supported contract or rejected; there is no partially authoritative aggregate.

## Dependency direction

`trnm-world-domain` may depend only on deterministic serialization, hashing and standard collections approved by the workspace. It does not depend on `trnm-world-command`, projections, UI, API/server, SQL, files, environment variables, wall clocks, randomness providers, HTTP, credentials, or sibling repositories. Higher layers translate external identity, order and storage concerns into typed domain inputs; the domain package never reaches outward to obtain them.

## Execution, concurrency and cancellation

Domain construction, lookup and validation are synchronous, deterministic and side-effect free. There are no threads, tasks, locks, sockets or cancellation points. Multiple callers may read immutable values concurrently. A mutation owner constructs a private candidate through the command/application layer, validates the full candidate, and only then publishes it. Cancellation before publication drops temporary data and cannot advance a public revision, event cursor or durable state.

## Persistence and durability

This crate performs no I/O. The owning repository stores canonical bytes, contract/rules/content identities, aggregate revision, immutable event/request identity, and integrity hash. PostgreSQL is the intended production persistence boundary; the file-backed repository is a fixture only. A save/load cycle must preserve canonical IDs and semantics. Corrupt, truncated, unsupported or lineage-crossed state is quarantined, not rewritten in place. Migration is performed outside this crate under an explicit versioned procedure and rollback plan.

## Idempotency, retry and failure taxonomy

Pure reads and validation are repeatable for identical bytes. Durable mutation idempotency belongs to the application/repository layer and binds an immutable request or event identity to the same expected revision and command bytes. Exact duplicates may return the previously recorded decision; altered duplicates are conflicts. Domain failures distinguish unsupported contract, malformed identity, duplicate identity, missing reference, invalid topology, invalid status/transition, bounds/resource violation and integrity mismatch. No failure silently substitutes fixture data.

## Versioning and migration

Domain schema, authored content revision, transition semantics, API version, database migration and binary build are separate identities. A field or enum change, stable-ID reuse, reference rule, canonical ordering, transition meaning or validation precedence change requires a new contract/rules revision, positive and negative vectors, migration/reader inventory, rollback behavior, downstream command/projection/API updates and exact component-lock review. A removed reader needs a dated usage inventory and independently accepted retirement gate.

## Resource and performance budgets

Identifier lengths, text fields, nodes, edges, positions, characters, NPCs, tasks, skills, items, routes, metadata depth and serialized state bytes are explicitly bounded before production adoption. Validation must be linear or otherwise documented in bounded collection size and must not allocate from an unchecked untrusted length. Large authored packs require measured parse, validation, memory and projection cost. Raising a ceiling requires denial-of-service review, boundary fixtures and updated database/API limits.

## Observability and security

Validation emits stable bounded machine codes plus contract/revision/hash and affected non-sensitive IDs. It never logs secrets, tokens, passwords, cookies, unbounded payloads or private player data. Telemetry distinguishes unsupported version, structural/reference/bounds/integrity failure and successful aggregate validation. Domain output is unsigned World material and cannot be presented as Nakama completion, CEX receipt or Chain finality. Provenance and privacy rules apply to authored map/content fields.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-domain/src/lib.rs`; the package manifest is in the same crate. Normative documents are `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md`, `docs/architecture/cex-world-authority-cutover-v1.md`, and `docs/contracts/trillionnium-world-authority-cutover-v1.json`. `scripts/check-trnm-world-authority-cutover.sh` and crate tests cover fixture validity, uniqueness, references, topology, bounds, canonical serialization, hostile malformed state and repeated deterministic construction. PostgreSQL and cross-repository evidence are separate.

## Change checklist and open work

A change records affected contract/IDs/invariants; updates schema, vectors and migration/rollback; adds exact success and hostile tests; verifies command/projection/API compatibility; measures resource effects; and obtains affected-owner review. Open work includes a standalone machine-readable domain schema, generated field/ID dictionary, canonical byte/hash vectors, property/fuzz coverage, production repository conformance, historical-data migration inventory, and exact-head hosted execution. Until those close, the crate remains a cutover candidate.
