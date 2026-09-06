---
status: current-candidate
owner: trillionnium-world-database
applies_to_plan: trillionnium-world-development-2026-08-29-v4
supported_postgresql:
  - 16.4
last_reviewed: 2026-09-05
review_due: 2026-09-19
---

# Trillionnium World PostgreSQL Contract v1

## Scope

This document defines the database behavior relied on by the World-local compatibility server and settlement worker. It is not a substitute for schema/migration code; CI must verify that implementation and this catalogue converge.

## Supported profile

- PostgreSQL 16.4 is the current exact CI profile.
- A new PostgreSQL minor/major version requires migration, stored-procedure, query-plan and failover evidence.
- Extensions/functions used by migrations must be explicitly inventoried and available before migration starts.

## Migration contract

1. A transaction-scoped or session advisory migration lock serializes writers.
2. `trnm_online_schema_migrations` records version, unique name, SHA-256 and application time.
3. Existing version/name/checksum mismatch fails closed.
4. Each migration is immutable after promotion; repairs use a new migration.
5. Both game-server and settlement-worker entrypoints register the same ordered migrations.
6. Application startup does not begin service admission or remote settlement before required migrations finish.
7. Backup, rollback and old-writer behavior are defined for every release.

## Global lock order

All code uses the following order when more than one object class is locked:

<!-- trnm-lock-order-v2:start -->
```text
1. global migration advisory lock
2. database/physical-host authority advisory locks
3. fleet instance / route ownership rows
4. match row
5. terminal publication ACK row
6. match-member rows ordered by player_id
7. campaign rows ordered by campaign_id, player_id
8. settlement capture row
9. transaction-scoped settlement serialization advisory lock
10. settlement jobs ordered by campaign_id, job_id
11. operator policy/replay/quarantine rows
```
<!-- trnm-lock-order-v2:end -->

No code may acquire an earlier class after a later class. Advisory serialization locks for settlement keys are acquired inside the short claim transaction before the selected job update and released at transaction end.

This shared order is defined by `trnm-world-lock-order-v2.json`; its source
regression coverage is limited to the explicit capture/apply worker paths. Apply
may first read an unlocked routing hint, but must lock match, ACK, members and
campaigns before capture, then revalidate the exact capture/match/state binding.
The detailed scope, disposable-database tests and still-open procedure/trigger/
foreign-key verification are in `trnm-world-lock-order-v1.md`. The shared block
is a design requirement, not a claim that every existing SQL path already meets
it. Migrations remain append-only.

## Transaction classes

| Class | Purpose | Remote I/O allowed |
| --- | --- | --- |
| migration | apply immutable DDL/data migration | no |
| authority command commit | persist canonical compatibility-enclave command state | no |
| terminal publication | atomically persist terminal projections and ACK | no |
| settlement capture | bind terminal/campaign/head-intent snapshot | no |
| settlement claim | lease one eligible job | no |
| settlement remote execution | signer/CEX lookup/submit | yes, no DB transaction held |
| settlement apply | validate receipt and CAS campaign state | no |
| operator replay authorization | append audit and re-enable one attempt | no |
| readiness query | bounded health projection | no mutation |

## Settlement stored-procedure catalogue

### `trnm_online_remote_request_id_v1`

- Input: match ID, campaign ID, intent ID.
- Output: stable domain-separated SHA-256 identity.
- Invariant: excludes local capture/lease generation; immutable intent hash remains a separate fence.
- Failure: null input rejected by `STRICT`; invalid stored identity rejected by trigger/constraint.

### `trnm_online_settlement_serialization_key_v1`

- Input: campaign ID and immutable intent JSON.
- Output: primary account ID or campaign fallback.
- Invariant: same economic account/campaign key has at most one live lease or pending apply.

### `trnm_online_claim_settlement_job_v2`

Preconditions:

- owner nonempty and bounded;
- lease duration 1–300000 ms;
- active capture, unapplied job, attempts available, not quarantined;
- pending/retryable-ready/expired-leased state;
- no same-key live lease or pending apply.

Postconditions:

- exactly zero or one job leased;
- attempts and lease generation increment;
- owner/expiry and stable authorization material set;
- no remote effect.

Retry: caller may retry after database error; claim is fenced and returns at most one row.

### `trnm_online_begin_settlement_remote_attempt_v1`

- Requires live exact lease.
- Increments bounded remote attempt count.
- Returns new attempt or null when fence lost.
- Must commit before remote request starts so the attempt is durable.

### `trnm_online_store_settlement_authorization_v1`

- Requires live exact lease.
- Request ID must equal durable remote request ID.
- Authorized intent may not alter immutable intent identity.
- Exact duplicate store is idempotent; altered retry fails.

### `trnm_online_complete_settlement_job_v1`

- Requires live exact lease.
- Receipt ID/hash/object and optional wallet snapshot are validated structurally by caller and constraints.
- Persists `state=succeeded` and remote completion only.
- Does not set campaign application.

### `trnm_online_retry_settlement_job_v1`

- Requires live exact lease.
- Sets bounded retry delay or dead-letter when budget exhausted.
- Clears lease.
- Error diagnostic is bounded and must contain no credentials.

### `trnm_online_dead_letter_settlement_job_v1`

- Requires live exact lease.
- Marks permanent remote failure, clears lease and preserves evidence.

### `trnm_online_authorize_settlement_replay_v1`

- Requires exact receipt-free dead-letter job and dead-letter capture.
- Binds every immutable identity plus operator/change/reason/policy.
- Appends immutable evidence.
- Permits one additional remote attempt.
- Cannot operate on remote-success or campaign-applied work.

### Quarantine functions (migration 19 target)

Required API:

- record/update bounded quarantine observation;
- query whether match/capture/job is currently eligible;
- append operator review/decision;
- preserve history and retention;
- prohibit direct destructive reset.

## Constraints and indexes

Required database-enforced invariants:

- SHA-256 fields use lowercase 64-hex checks;
- capture/job/remote/replay IDs use exact domain prefixes;
- match/campaign/intent/remote identity fields are immutable;
- one active capture per match;
- one job per `(capture_id, campaign_id)`;
- one exact `(capture_id, campaign_id, intent_id)`;
- campaign apply timestamp only on remote-succeeded receipt-bearing job;
- foreign keys use `ON DELETE RESTRICT` for settlement/evidence lineage;
- hot eligibility, serialization, pending-apply, expired-lease and quarantine indexes exist;
- append-only operator policy/replay/quarantine decision evidence rejects update/delete/truncate.

## Privilege model

Separate roles are required:

| Role | Required access |
| --- | --- |
| game server | compatibility authority tables/functions; no operator replay; no signer key material |
| settlement worker | capture/claim/remote result/apply functions; read exact campaign/match rows; no arbitrary DDL |
| operator recovery | execute audited policy/replay/quarantine-decision functions only |
| migration role | DDL/migration ledger under controlled deployment |
| read-only observability | metrics/status views only |

`PUBLIC` execute is revoked from sensitive functions. Production grants are explicit and checked in deployment evidence.

## Readiness and observability

Readiness fails closed on:

- migration drift;
- lost host/instance fence;
- poisoned durable authority state;
- unavailable database required for admission;
- settlement schema/procedure mismatch.

Metrics include:

- pending, leased, retryable, remote-succeeded, pending-apply, applied and dead-letter counts;
- expired leases;
- oldest eligible and pending-apply age;
- max attempts;
- quarantine count/age by phase;
- replay count by policy window;
- connection pool saturation and query latency.

## PITR/failover contract

After restore/failover:

1. verify system identifier, timeline and postmaster identity;
2. fence old primary/process before admission;
3. verify migration ledger checksums;
4. reconcile hot/cold terminal witnesses with database summary;
5. inventory live leases and allow expiry/takeover, never hand-edit ownership;
6. recover remote successes through receipt lookup;
7. verify operator and settlement evidence retention;
8. run exact smoke/fault probes before promotion.

A restore cannot be promoted from row counts alone.

## Required tests

- empty database migration to latest;
- supported prior schema upgrade;
- checksum/name drift rejection;
- old-writer rejection;
- concurrent same-key/unrelated-key claims;
- lease expiry/takeover and stale writer rejection;
- capture commit visibility boundary;
- duplicate campaign job constraint;
- operator replay and append-only evidence;
- quarantine isolation;
- rollback/PITR/timeline/old-primary matrix;
- hot query plan thresholds at representative scale.