---
status: current-candidate
owner: trillionnium-world-command
module: trnm-world-command
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-world-command detailed design

## Scope and authority

This crate owns deterministic World-domain command semantics over an already admitted caller and one expected aggregate revision. It resolves movement, transition, task, skill, item, combat/tactics and related World state changes, returning a typed decision and candidate successor. Its authority stops at game-domain business rules. Nakama supplies canonical online identity/order/idempotency/reconnect; repositories supply durable CAS/fencing; CEX owns value; Chain and Integration retain their responsibilities.

## Public interfaces

The public surface includes `WorldCommand`, `WorldCommandDecision`, typed tactics request/outcome values, movement and transition decision helpers, `apply_command`, `apply_tactics_command`, stable result/status/reason strings and contract identities. A caller passes validated typed input and an expected valid `WorldState`. A successful decision exposes the complete resulting state and bounded machine-readable outcome; a rejection provides no partially committed successor.

## State model and invariants

Every command is evaluated against one input revision and one versioned rules/content set. The implementation constructs a private candidate, applies all effects, validates aggregate invariants, and publishes only on success. An error preserves the complete semantic state, including positions, tasks, inventories, skills, health/resources and counters. Movement follows authored edges and transition rules. Task/tactics/item/skill effects remain bounded and referentially valid. No command field may confer participant, canonical sequence, custody or completion authority.

## Dependency direction

The command crate depends inward on `trnm-world-domain` and deterministic support libraries. It must not depend on projections, HTML/UI, API/server adapters, SQL, files, HTTP, clocks, environment, credentials, CEX or Nakama implementations. Application code authenticates and admits a request, binds immutable identity and expected revision, invokes the command, and asks a repository to commit the accepted candidate. Projections are derived after commitment and never feed hidden authority back into decisions.

## Execution, concurrency and cancellation

Command evaluation is synchronous and deterministic. It owns no locks or tasks. One aggregate writer invokes it at a time under the repository/application single-writer protocol. A concurrent request with a stale revision is rejected before or during CAS rather than merged implicitly. Cancellation before durable commit drops the candidate. If a durable commit outcome is uncertain, the caller reads the immutable request/event record and aggregate revision; it never blindly reapplies altered bytes.

## Persistence and durability

The crate performs no persistence. The production repository atomically stores accepted aggregate revision, canonical state/hash, immutable request/event identity and decision. Rejections may be recorded as bounded audit evidence but do not advance the aggregate. The fixture server/file repository is permitted only for tests. PostgreSQL cutover procedures and constraints enforce epochs, stable event IDs, no dual writer and replay equivalence outside this package.

## Idempotency, retry and failure taxonomy

The application binds one immutable idempotency identity to actor/account context, expected revision, command contract and canonical bytes. Exact retries return the recorded decision or state; altered reuse fails. Stable rejection families include unsupported version, malformed command/identifier, unauthorized or unadmitted context, stale revision, unknown subject/target, blocked transition, invalid phase/status, unmet prerequisite, insufficient resource, bounds violation and invariant/integrity failure. Transport/DB ambiguity is not reclassified as command success.

## Versioning and migration

Command vocabulary, decision/error catalogue, domain schema, transition/rules/content semantics, API transport and repository events are independent versions. Changing fields, required targets, error precedence, arithmetic, side effects, traversal tie-breaks or outcome encoding requires a command/rules version, updated vectors, shadow compatibility, migration/rollback notes and every caller/projection update. Adding a command to this crate does not authorize a public endpoint; routing/admission requires separate owner approval.

## Resource and performance budgets

Raw transport is bounded before decoding; typed commands still enforce identifier/text/count/depth ceilings. A command may inspect only bounded relevant aggregate structures and must avoid unbounded historical scans. Movement/path/target selection has deterministic tie-breaks and documented worst-case work. Application queues and database transactions have separate budgets. Benchmarks cover maximum valid state, hostile invalid input, accepted/rejected command latency, candidate clone cost and state-size growth.

## Observability and security

Audit data binds immutable request/event identity, admitted actor/account context, expected and resulting revision/hash, command contract/type, bounded decision/reason and component versions. It excludes credentials, cookies, passwords and unbounded user text. Metrics distinguish accepted, exact duplicate, altered conflict, stale revision, blocked/prerequisite/resource/integrity failures and evaluation duration. Client claims remain proposals; only an admitted order and committed repository event can create durable World state.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-command/src/lib.rs`; domain input is defined by the sibling package. ADR-0003, the cutover architecture document, authority contract JSON and PostgreSQL SQL are normative neighbors. Crate tests and `scripts/check-trnm-world-authority-cutover.sh` cover each command family, movement topology, task/tactics/item/skill effects, rejection state preservation, deterministic repetition and adapter split. Cross-language/Nakama shadow and database replay evidence remain open.

## Change checklist and open work

For each change, identify authority and state effects, expected revision/idempotency binding, error precedence and resource impact; update contract/vector/migration/projection/API docs; add success, rejection, duplicate, stale, overflow and cancellation/ambiguity tests; and obtain exact-head review. Open work includes a generated command catalogue, canonical vectors and error codes, property-based state-preservation testing, application-level immutable event storage, Nakama shadow comparison, and production no-dual-writer evidence.
