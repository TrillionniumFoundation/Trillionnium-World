---
status: current-candidate
owner: trillionnium-world-contracts
module: governance-guard
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# `governance-guard` detailed design

## Scope and authority

`governance-guard` is the Rust MVP state machine for bounded proposal/timelock, version-drift protection and pause/resume semantics with normalized audit output. It models guard decisions only. It is not deployed governance, a multisig, key custodian, voting system, upgrade executor, emergency committee or proof that GitHub/Chain/runtime governance is active.

## Public interfaces

The public surface consists of typed proposal/action/actor/version identifiers, guard state, request/decision/error values and deterministic methods for the currently implemented schedule/execute/cancel/pause/resume lifecycle. Accepted transitions return explicit effects and audit material. Actor roles are typed fixture inputs; a display name or locally supplied role does not authenticate a production principal. Exact fields and methods need generated schema and vectors before host adoption.

## State model and invariants

A proposal ID is permanently bound to one canonical action, target/version, proposer context and delay policy. Execution cannot occur before the effective timelock, after cancellation/expiry or under a mismatched target version. Pause/resume follows explicit role and state rules; an error preserves all state. Time/height is supplied as a typed accountable input and is never read from an uncontrolled local clock. No transition may silently alter the guarded payload after scheduling.

## Dependency direction

The crate may depend on `audit-events`, deterministic serialization/hashing and bounded collections. It must not import GitHub administration, Chain runtime, World/Nakama/CEX implementations, network clients, SQL, filesystem, clocks, keys, environment configuration or deployment code. A future host authenticates principals, supplies canonical time/height, persists state and executes an independently validated target action only after the guard decision.

## Execution, concurrency and cancellation

Current evaluation is synchronous, deterministic and single-owner. It starts no task and owns no external lock. A future runtime admits one mutation against an expected revision or serialized actor. The guarded side effect is not executed while the guard state is privately mutating; the runtime must define an atomic or reconciled decision-to-execution protocol. Cancellation before publication drops the candidate. Ambiguous execution is resolved by proposal/action identity lookup.

## Persistence and durability

Current state is in-memory. Production adoption requires canonical proposal/state bytes, immutable decision records, durable time/height source binding, atomic state/audit publication, guarded-action receipt, restart recovery, backup/PITR, old-executor fencing and key/role rotation. A unit-test timelock or pause flag does not activate repository, service or Chain governance.

## Idempotency, retry and failure taxonomy

Proposal and execution request identities bind actor, action hash, target/version, schedule/expiry, expected revision and contract version. Exact retries may converge; altered reuse fails. Stable failures distinguish malformed/unsupported input, unauthorized fixture role, duplicate-altered proposal, too-early/expired/cancelled execution, target-version drift, invalid pause state, stale revision, bounds/overflow and integrity failure. Authentication, quorum, signature, storage and target-execution ambiguity belong to adapters/runtime.

## Versioning and migration

Guard semantics, role policy, timelock/time source, action schema, target version, state schema, audit schema, host ABI and deployment release are separate identities. Changing role thresholds, delay calculation, action hash domain, execution conditions, pause behavior, error precedence or canonical bytes requires a new contract/version, migration/rollback, positive/negative vectors and old/new executor compatibility. Emergency changes must not silently bypass the declared guard.

## Resource and performance budgets

Proposal/action bytes, identifiers, pending/terminal proposal counts, metadata, batch size, history and serialized state are bounded. Validation is deterministic and rejects oversized or deep action material before parsing/verification. Host gas, storage quota, queue, time-source freshness and concurrent governance operations require separate budgets. Historical records need compaction/retention rules that preserve auditability.

## Observability and security

Audit data binds stable proposal/action/target/version IDs, supplied time/height, role reference and bounded decision code. It excludes private keys, tokens, signatures treated as secrets and unbounded payloads. Production requires authenticated multisig/quorum or other approved control, key custody/rotation, separation of proposer/approver/executor, emergency process, target allow-list, incident response and independent governance review.

## Test and evidence traceability

Primary source is `contracts/governance-guard/src/`; current scope is defined by `contracts/README.md`. Tests cover schedule/execute/cancel, too-early and expired execution, action/version drift, exact/altered duplicate, pause/resume role/state behavior, overflow/bounds, state preservation and audit normalization. Host time, signatures/quorum, durable execution and live governance probes remain absent.

## Change checklist and open work

A change states affected roles, action hash, delay/time source, target version, lifecycle, errors and resource limits; updates schema/vectors/tests/audit docs; and receives governance/security/target-owner review. Open work includes machine schemas, canonical action vectors, authenticated role/quorum contract, canonical time/height source, host ABI/runtime-spec, durable storage, atomic guarded execution receipt, key custody, emergency drill, independent audit and explicit production governance authorization.
