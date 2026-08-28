---
status: current-design
owner: trillionnium-world
contract: trnm_settlement_outbox_v1
runtime_status: integrated-pending-p0-evidence
verified_commit: pending-merge
related_adr: ../adr/0002-transaction-free-external-settlement.md
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# TRNM Settlement Outbox v1

## Purpose

This contract separates durable game state from external signer/CEX latency.
The compatibility server now contains a dedicated PostgreSQL-backed settlement
worker with capture, external execution, and apply phases. The implementation is
source-integrated, but it is not yet remotely verified and therefore does not
close `WORLD-P0-001`.

The remaining P0 blockers are explicit:

- one inert compatibility caller remains in `trnm-game-server/src/lib.rs` and
  must be deleted rather than used as a fallback;
- exact-commit GitHub Actions evidence is absent;
- database black-box tests have not yet proved every response-loss, cancellation,
  process-kill, lease-takeover, and apply-rollback boundary;
- operator dashboards, receipt lookup/recovery, and promotion evidence are not
  complete.

The standalone reference implementation lives at
`trillionnium/tools/trnm-settlement-outbox-contract`. It defines the stable
state-machine invariants and remains the normative model for lease fencing and
receipt binding.

## Identity model

Three different identities must not be conflated.

### Stable economic intent identity

The game-owned `EconomicIntent` contains the immutable `intent_id`,
`idempotency_key`, and canonical `intent_hash`. Those values survive transport
ambiguity, worker restart, stale apply, and recapture.

### Contract job identity

The standalone `trnm_settlement_outbox_v1` contract derives its stable job key
from:

- `match_id`;
- `campaign_id`;
- `intent_id`.

Its deterministic contract job ID is independent of a worker lease, capture
generation, or mutable payload bytes. `intent_hash` is validated as a separate
immutable fingerprint.

### Runtime capture row and remote request identity

The current migration-era runtime retains a capture-scoped local `job_id` row so
that exact campaign revision/state-hash and terminal-publication fences remain
reviewable per capture. That local row ID must never be used as the signer/CEX
idempotency identity.

The runtime derives a stable request ID:

```text
remote_request_id =
  "trnm-settlement-remote-v1:" ||
  hex(SHA-256(length_prefixed_utf8(
    "trnm_settlement_remote_v1",
    match_id,
    campaign_id,
    intent_id
  )))
```

Each component is prefixed by its unsigned 32-bit big-endian byte length. The
preimage deliberately excludes `capture_id`, `capture_generation`, and
`intent_hash`:

- excluding capture state makes the request identity survive stale apply and
  recapture;
- excluding `intent_hash` ensures reusing one intent ID with changed payload
  bytes reaches the same remote idempotency key and produces a conflict rather
  than minting a second entitlement/request identity;
- the full `intent_hash` remains mandatory as the independent payload-integrity
  and receipt-binding fence.

`remote_request_id` is an ordinary `NOT NULL` database column whose value is
owned by `trnm_online_remote_request_id_v1`. The migration backfills historical
rows, then installs two database triggers:

- `trnm_online_settlement_remote_id_insert_v1` derives every new row;
- `trnm_online_settlement_remote_id_update_v1` covers match, campaign, intent,
  and `remote_request_id` updates.

A caller may omit the value or supply the exact derived value. Any alternate
value fails with SQLSTATE `23514`; changing an identity-bearing field also fails
rather than silently rebinding an existing economic job. A catalogue assertion
rejects an unexpected generated or otherwise incompatible column left by a
pre-review local migration.

The signer authorization request ID and entitlement nonce are initialized from
`remote_request_id`. The database rejects an authorization result whose request
ID differs from the durable remote identity.

The local runtime representation is still migration debt relative to the
standalone contract's single stable job record. A later schema normalization may
rename or split the capture-scoped row, but it must preserve remote identity and
historical evidence.

## State machine

The normative remote execution state machine is:

```text
Pending
  -> Leased(owner, generation, expires_at)
       -> Succeeded(receipt_id, receipt_hash)
       -> Retryable(next_attempt_at, last_error)
       -> DeadLetter(reason)

Retryable when due -> Leased with a higher generation
Expired Leased     -> Leased with a higher generation
Succeeded          -> terminal remote result
DeadLetter         -> terminal remote failure
```

A lease increments both `attempts` and `lease_generation`. Every durable remote
mutation—authorization persistence, attempt start, receipt completion, retry,
and dead-letter transition—requires all of:

- `state = leased`;
- exact owner;
- exact generation;
- `lease_expires_at > clock_timestamp()`.

An expired worker cannot write a retry or dead letter after a new generation has
become eligible.

## Remote success is not campaign application

Raw database `state = succeeded` means only that a validated remote receipt is
durably stored. It does **not** mean campaign progression was applied.

The operator projection `trnm_online_settlement_job_status_v1` exposes two
separate dimensions:

- `remote_state`, including `remote_succeeded`;
- `application_state`, including `pending_apply`, `applied`, `blocked`, and
  `waiting_remote`.

Player UI, readiness, dashboards, and release evidence must never collapse
`remote_succeeded + pending_apply` into a completed settlement claim.

## Durable evidence retention

Settlement jobs and captures are economic/audit evidence. Foreign keys from the
outbox to match and campaign rows use `ON DELETE RESTRICT`, not cascade.
Deletion requires a separately reviewed retention/archive operation that
preserves intent, receipt, compensation, retry, and dead-letter history.

## Database implementation

The runtime is implemented by:

- `0016_online_settlement_outbox_v1.sql`—base outbox table and readiness
  predicate;
- `0017_online_settlement_worker_runtime_v1.sql`—capture fences, trigger-enforced
  stable remote identity, live-lease mutation functions, and status projection;
- `trnm-settlement-worker`—bounded async signer/CEX execution outside business
  transactions;
- `settlement_worker.rs`—capture, execute, and exact apply orchestration.

A capture binds:

- exact terminal publication identity;
- match ID and capture generation;
- every campaign revision and state hash;
- the current compensation/ordinary head intent and hash.

## Claim transaction

The claim transaction:

1. selects an eligible `Pending`, due `Retryable`, or expired `Leased` job using
   `FOR UPDATE SKIP LOCKED`;
2. requires an active capture and unapplied campaign state;
3. increments attempt and generation;
4. writes owner and expiry;
5. initializes stable authorization/nonce material from `remote_request_id`;
6. commits before any network call.

The retired v1 claim function fails with SQLSTATE `0A000`; only v2 may lease
work. The claim transaction never calls signer or CEX.

## External execution

The worker uses asynchronous clients and immutable job data:

1. authorize or recover the exact entitlement under `remote_request_id`;
2. persist the exact authorized intent and signer receipt under a live lease;
3. submit/query the exact CEX economic intent using its immutable intent ID and
   idempotency key;
4. validate the returned receipt against the authorized intent;
5. durably store remote success under the same live lease;
6. leave campaign state untouched until a separate apply transaction.

A timeout is not evidence that the remote side failed. Retrying or querying the
same durable identity is mandatory; minting a replacement intent/request ID is
forbidden.

## Apply transaction

The apply transaction:

1. locks the active capture and exact match;
2. revalidates terminal publication, cold seal, result, generation, host, tick,
   sequence, and snapshot identity;
3. locks all member campaigns in deterministic order;
4. verifies every captured campaign revision/state hash before the first write;
5. verifies the current campaign head still matches the captured intent/hash;
6. validates each durable remote receipt against that exact intent;
7. applies campaign updates with revision/state-hash compare-and-swap;
8. marks every job `campaign_applied_at`;
9. advances the match only when no economic head remains;
10. commits atomically.

Any mismatch produces a stale or dead-letter outcome without partial campaign
writes.

## Retry and dead letter

- Maximum remote attempts: 16.
- Lease duration must be positive and no more than five minutes.
- Retry delay is bounded and deterministic with stable jitter.
- Errors are bounded before persistence and must not contain credentials.
- The final failed remote attempt becomes `DeadLetter`.
- Retry and dead-letter writes require a still-live lease.
- Operator replay requires a separately versioned, auditable control; it must not
  overwrite terminal history silently.

## Ordering and isolation

Compensation work is selected before ordinary settlement work. One active
capture exists per match. Unrelated matches may progress independently; a
poisoned job must not become a global FIFO blocker.

Account/campaign serialization and contention behavior still require database
black-box evidence before trusted-settlement promotion.

## Telemetry requirements

Required metrics and projections include:

- remote pending/leased/retryable/succeeded/dead-letter counts;
- application waiting/pending-apply/applied/blocked counts;
- oldest eligible and oldest pending-apply ages;
- lease expiration/reclaim and stale-generation rejection counts;
- attempts and remote-attempt histograms;
- signer and CEX latency/outcome;
- ambiguous remote outcome count;
- receipt mismatch and campaign revision conflict counts;
- settlement completion lag from terminal publication.

Logs may bind job ID, remote request ID, match ID, campaign ID, intent ID,
attempt, and lease generation. Credentials, private keys, raw tokens, and full
private entitlement material must never be logged.

## Fault matrix

At minimum inject process death, timeout, cancellation, or response loss:

1. before capture commit;
2. after capture commit and before claim;
3. after claim commit and before signer call;
4. after signer commit and before response;
5. after authorization persistence and before CEX call;
6. after CEX commit and before response;
7. after receipt verification and before remote-success persistence;
8. after remote-success persistence and before apply;
9. during campaign update before apply commit;
10. after apply commit and before worker acknowledgement;
11. after lease expiry while the old worker is still running;
12. during retry and dead-letter transitions;
13. during worker shutdown with queued and in-flight work;
14. during concurrent claim/apply attempts by two workers.

Every row must converge without duplicate value, stale local writes, silent
progression loss, or an expired worker mutation.

## Runtime migration checklist

- [x] Add reviewed migration and indexes.
- [x] Add async signer/CEX clients.
- [x] Add capture, claim, remote execution, and exact apply phases.
- [x] Persist trigger-enforced SHA-256 remote request identity independent of
  capture generation and payload fingerprint.
- [x] Reject direct or indirect mutation of the derived remote identity.
- [x] Require a live lease for every remote mutation.
- [x] Retire the weaker v1 claim path.
- [x] Separate remote success from campaign application in operator status.
- [x] Prevent upstream cascade deletion of settlement evidence.
- [x] Add static guards and focused contract tests.
- [ ] Add explicit remote receipt lookup/recovery for every ambiguous dependency
  response.
- [ ] Prove account/campaign ordering and two-worker contention in PostgreSQL.
- [ ] Remove the inert legacy `reconcile_economy` caller.
- [ ] Add dashboards and operator replay/retention runbooks.
- [ ] Execute the complete black-box fault matrix.
- [ ] Bind exact-commit remote CI, artifact, toolchain, environment, and reviewer
  evidence.

Until every unchecked row is closed, `trusted_cex_settlement` remains blocked
and public player markets remain disabled.
