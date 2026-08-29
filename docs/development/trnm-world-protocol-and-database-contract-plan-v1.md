---
status: current-plan
owner: trillionnium-world
work_items:
  - WORLD-P1-002
  - WORLD-P1-003
last_reviewed: 2026-08-29
---

# Protocol and Database Contract Plan v1

## 1. Goals

Replace implementation-only semantics with versioned, generated and
independently testable contracts. Protocol version, rules/content revision,
build provenance and deployment identity are distinct fields.

## 2. HTTP/OpenAPI scope

Publish OpenAPI for:

- readiness and health;
- compatibility identity/session verification surfaces;
- lobby/match/command/reconnect/read APIs retained during migration;
- moderation/season/replay operations;
- signer readiness/attestation/receipt lookup/sign;
- settlement operator status/replay request;
- deterministic World transition HTTPS service where retained.

Every operation defines:

- authentication audience and scope;
- request/response byte limits;
- stable success/error schemas;
- idempotency key and replay behavior;
- timeout/retry guidance;
- rate-limit headers;
- sensitive-field logging policy;
- deprecation/retirement date.

## 3. WebSocket contract

Publish schemas for full snapshot, delta, command receipt, resync, terminal and
error frames. Define:

- actor generation and canonical sequence;
- base/state sequence continuity;
- snapshot/state hash binding;
- duplicate, gap, truncation and resync behavior;
- maximum frame/collection/decompression sizes;
- unknown frame/field/enum policy;
- keepalive and close codes;
- compatibility capabilities separate from build IDs.

## 4. Stable errors

All public errors carry:

```json
{
  "error_code": "stable_machine_code",
  "reason_code": "stable_subcode",
  "retryable": false,
  "retry_after_ms": null,
  "correlation_id": "bounded-id",
  "detail": "bounded non-sensitive diagnostic"
}
```

Consumers branch on codes, never free-form detail. Internal SQL/stack/credential
material is not returned.

## 5. Canonical encoding

The canonical profile specifies:

- UTF-8 object/array root;
- complete JSON grammar;
- decoded object keys strictly ascending and unique;
- signed-i64 decimal numbers only;
- minimal escapes and exact reencoding;
- maximum depth and byte budgets;
- domain-separated hash preimages;
- decoded recursive authority-field allow/deny rules;
- shared Rust/Go/golden malicious vectors.

## 6. Compatibility matrix

Maintain a machine-readable table with:

- client protocol/capabilities;
- server protocol/capabilities;
- World rules/content revision;
- Nakama adapter version;
- database writer/migration version;
- accepted rolling window;
- retirement deadline;
- drain/rollback constraints.

No compatibility is inferred from semver alone.

## 7. Stored-procedure catalogue

Generate one entry per procedure/view/trigger with:

- schema/name/signature/version;
- owner and EXECUTE privileges;
- security invoker/definer and fixed `search_path`;
- preconditions and invariant fences;
- rows/locks/advisory locks acquired in order;
- postconditions and durable state change;
- result shape and SQLSTATE/error codes;
- idempotency/retry/cancellation behavior;
- supported PostgreSQL versions;
- indexes/query-plan baseline;
- migration introduction and retirement version;
- unit/integration/fault tests.

## 8. Global lock order

The target lock hierarchy is documented and tested, for example:

```text
migration advisory lock
  -> database-host/fleet fence
  -> match row
  -> terminal publication row
  -> campaign rows ordered by campaign_id
  -> settlement capture
  -> settlement jobs ordered by campaign_id/job_id
  -> transaction-scoped account serialization advisory lock
```

Any exception requires an ADR and deadlock test.

## 9. Migration and rollback matrix

For every schema version prove:

- empty database apply;
- supported previous-version upgrade;
- checksum/name drift rejection;
- old writer against new schema during declared window;
- new writer against supported old schema or explicit startup rejection;
- failed migration rollback;
- PITR/timeline change and old-primary isolation;
- privilege reapplication;
- data-retention and irreversible-step approval.

## 10. CI outputs

- OpenAPI and JSON Schema lint;
- implementation fixture validation;
- generated client/server round-trip tests;
- golden hash/error vectors;
- procedure catalogue/source drift check;
- lock graph and migration matrix results;
- query-plan regression artifacts.
