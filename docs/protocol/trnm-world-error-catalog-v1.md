---
status: current-candidate
owner: trillionnium-world-protocol
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none
---

# Trillionnium World stable error catalogue v1

## Purpose

A stable error is a bounded machine decision, not an English log line. The compatibility server and clients classify failures so callers know whether to stop, retry the exact identity, wait, resync, reauthenticate or request operator action. The target envelope is described by `schemas/trnm-world-error-envelope-v1.schema.json`; handlers not yet using that envelope remain an explicit conformance gap.

## Retry classes

| Class | Caller behavior |
|---|---|
| `never` | Permanent rejection for the same contract/version/input. Do not retry unchanged bytes. |
| `retry_same` | Retry only the exact immutable command/request identity and bytes. |
| `retry_after` | Retry the exact identity after bounded `retry_after_ms`. |
| `resync` | Obtain a full snapshot/current revision, then decide whether a new command is valid. |
| `reauthenticate` | Refresh the session/service identity; do not reuse an expired credential. |
| `operator_required` | Quarantine the exact scope and require an audited operator decision. |

## Error families

| Prefix | Meaning | Default class |
|---|---|---|
| `invalid_*` | Grammar, type, identifier, state or command rejection | `never` |
| `unsupported_*` | Protocol/build/schema/content/rules revision not accepted | `never` |
| `unauthorized` / `forbidden` | Missing identity or insufficient audience/role | `reauthenticate` / `never` |
| `revision_conflict` | Expected state/revision no longer current | `resync` |
| `sequence_gap` / `generation_stale` | Cursor or actor generation cannot be continued | `resync` |
| `duplicate_conflict` | Same identity reused with changed immutable bytes | `never` |
| `capacity_exhausted` | A declared bounded queue/pool/connection limit is reached | `retry_after` |
| `dependency_unavailable` | Required database or service is temporarily unavailable | `retry_same` |
| `remote_outcome_unknown` | Signer/CEX may have committed but the exact result is unresolved | `retry_same` through lookup |
| `fence_lost` / `stale_lease` | Process/worker no longer owns the exact generation/lease | `never` for that worker |
| `poison_quarantined` | Corrupt/ambiguous durable scope is isolated | `operator_required` |
| `internal_invariant` | Server detected a correctness invariant failure | `operator_required` |

## Precedence

Validation uses deterministic precedence: transport/resource bounds, syntax/shape, protocol/build, authenticated identity/audience, immutable identity conflict, revision/cursor/fence, domain transition, persistence and dependency result. A later layer must not mask an earlier permanent rejection with a retryable generic error.

## Diagnostic rules

`message` is bounded human context and is not parsed for control flow. It contains no bearer token, secret, private key, database URL, raw signed entitlement or unbounded upstream body. `correlation_id` is opaque and bounded. Upstream response bodies are truncated/redacted before evidence storage.

## Settlement ambiguity

Timeout, connection loss after send, malformed 2xx and duplicate/conflict responses are not proof of failure. The settlement worker records `remote_outcome_unknown`, releases no duplicate value and performs exact lookup-before-submit under the same immutable remote request identity.

## Change control

Adding a code is additive only when old callers safely map it to a documented family. Changing retry class, precedence or meaning is breaking and requires a new catalogue version, negative fixtures, client/server compatibility tests and rollback notes.
