---
status: current-design
owner: trillionnium-world
contract: trnm_settlement_outbox_v1
runtime_status: integrated-pending-p0-evidence
verified_commit: pending-merge
related_adr: ../adr/0002-transaction-free-external-settlement.md
related_protocol: ../protocol/trnm-settlement-receipt-recovery-v1.md
operations_runbook: ../runbooks/trnm-settlement-operations-v1.md
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# TRNM Settlement Outbox v1

## Purpose and current posture

This contract separates durable game state from external signer/CEX latency.
The compatibility server contains a PostgreSQL-backed worker with capture,
external execution, and apply phases. The recovery slice also implements
lookup-before-submit clients, account/campaign serialization, mandatory
PostgreSQL tests, and operator projections.

The source is integrated but not remotely verified. `WORLD-P0-001` remains
open because:

- one inert compatibility `reconcile_economy(&state.cex)` caller remains and
  must be deleted rather than used as a fallback;
- the CEX receipt lookup endpoint is still an owner-repository dependency;
- exact-commit GitHub Actions evidence remains absent;
- deployed signer/CEX response-loss, process-kill, cancellation, shutdown, and
  apply-rollback evidence is incomplete;
- operator replay/retention controls and reviewer signoff remain open.

No source-only result grants trusted CEX settlement, public online, public
market, or production release credit.

## Identity model

Four identities must not be conflated.

### Economic intent

`EconomicIntent.intent_id`, its idempotency key, and the SHA-256 of the exact
authorized JSON bytes are the game-owned economic identity and payload binding.
They survive worker restart, stale apply, recapture, and remote ambiguity.

### Stable contract job key

The standalone `trnm_settlement_outbox_v1` reference contract derives a stable
key from:

- `match_id`;
- `campaign_id`;
- `intent_id`.

### Capture-scoped runtime row

The migration-era runtime keeps a local `job_id` per capture so exact campaign
revision/state hash and terminal-publication fences remain auditable. This row
ID is never the signer/CEX idempotency identity.

### Remote request identity

The database derives:

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

The preimage excludes capture ID/generation and intent hash:

- recapture therefore reuses the same remote request identity;
- reusing one intent ID with changed bytes reaches the same remote key and must
  conflict rather than mint a second entitlement;
- the full `intent_hash` remains the independent payload-integrity fence.

`remote_request_id` is backfilled and trigger-derived. Match, campaign, intent,
and remote alias fields are immutable after insertion. Authorization request ID
and entitlement nonce must equal the derived remote identity.

## Normative state machine

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

Every remote mutation—authorization persistence, attempt start, completion,
retry, and dead letter—requires:

- `state = leased`;
- exact owner;
- exact generation;
- a nonexpired lease.

A stale or expired worker cannot mutate durable state after takeover.

## Capture transaction

The capture transaction:

1. locks one ready terminal match;
2. revalidates terminal publication, cold seal, result, host/generation, tick,
   sequence, and snapshot identity;
3. locks both member campaigns in deterministic order;
4. records every campaign revision/state hash and current compensation/ordinary
   head intent/hash;
5. creates capture-scoped worker rows;
6. commits before any signer/CEX request.

It performs no remote I/O.

## Claim and account/campaign serialization

Claim v2 selects eligible pending, due retryable, or expired leased jobs using a
bounded `FOR UPDATE SKIP LOCKED` window. Claim v1 fails closed with SQLSTATE
`0A000`.

The immutable serialization key is:

```text
primary intent actor account_id
  or, when absent,
"campaign:" + campaign_id
```

A bounded candidate scan uses a transaction-scoped advisory lock for this key.
A second job with the same key is blocked while another job has:

- a live lease; or
- a durable remote success that is still `pending_apply`.

Unrelated accounts remain eligible. Compensation priority is preserved without
a global FIFO.

## Lookup-before-submit recovery

A remote timeout or `5xx` is not proof of failure. The recovery protocol is
`docs/protocol/trnm-settlement-receipt-recovery-v1.md`.

### Signer

1. `GET /v1/signer/receipts/{remote_request_id}`;
2. `200` — validate/reuse exact durable signature response;
3. `404` — submit one exact sign request;
4. lookup transport/`5xx` — retry later without signing.

The returned payload hash and signing receipt hash are recomputed. The
entitlement nonce equals `remote_request_id`; the economic `intent_id` remains
separate.

### CEX

1. query the CEX owner endpoint with exact `intent_id` and
   `x-trnm-intent-sha256`;
2. `200` — validate wrapper ID/hash and `EconomicReceipt::validate_for`;
3. `404` — submit one exact authorized intent;
4. `409`, malformed, or mismatched data — fail closed;
5. lookup transport/`5xx` — retry later without submitting.

The World client and black-box fixtures implement this sequence. The CEX owner
endpoint must still land and be component-locked before deployed recovery credit
is possible.

## Apply transaction

Raw `state = succeeded` means only that a validated remote receipt is durable.
It does not mean campaign progression was applied.

Apply:

1. locks the active capture and exact match;
2. revalidates terminal publication identity;
3. locks campaigns in deterministic order;
4. verifies every captured revision/state hash before any write;
5. verifies each current campaign head still matches the captured intent/hash;
6. validates every durable receipt against that exact intent;
7. performs revision/state-hash compare-and-swap updates;
8. marks each job `campaign_applied_at`;
9. finalizes the match only when no economic head remains;
10. commits atomically.

Any mismatch produces stale/dead-letter disposition without partial campaign
writes.

## Operator state and metrics

`trnm_online_settlement_job_status_v1` exposes:

- `remote_state`, including `remote_succeeded`;
- `application_state`, including `pending_apply`, `applied`, `blocked`, and
  `waiting_remote`;
- stable remote request and serialization identities.

`trnm_online_settlement_metrics_v1` exposes:

- pending, leased, retryable, succeeded, and dead-letter counts;
- pending-apply and applied counts;
- expired leases;
- oldest eligible and oldest pending-apply ages;
- maximum remote attempts.

Player UI, readiness, dashboards, and release evidence must never collapse
`remote_succeeded + pending_apply` into “settled.” Operator procedure is in
`docs/runbooks/trnm-settlement-operations-v1.md`.

## Durable evidence retention

Settlement jobs/captures are economic evidence. Their match/campaign foreign
keys use `ON DELETE RESTRICT`. Deletion requires a separately reviewed archive
and retention operation that preserves intent, receipt, compensation, retry,
and dead-letter history.

## Retry and dead letter

- maximum remote attempts: 16;
- maximum lease: five minutes;
- retry delay: bounded deterministic exponential backoff with stable jitter;
- errors: bounded and credential-free;
- exhausted/permanent failures: explicit dead letter;
- retry/dead-letter mutation: live-lease fenced;
- manual in-place replay: prohibited.

Operator replay requires a separately versioned auditable control and cannot
overwrite historical terminal evidence.

## Required fault matrix

At minimum prove:

1. before/after capture commit;
2. after claim commit and before signer;
3. signer commit followed by response loss;
4. after authorization persistence and before CEX;
5. CEX commit followed by response loss;
6. after receipt verification and before remote-success persistence;
7. after remote success and before apply;
8. during campaign update before apply commit;
9. after apply commit and before acknowledgement;
10. lease expiry while the old worker is still running;
11. two workers contending for the same account and unrelated accounts;
12. retry/dead-letter transition;
13. process kill, cancellation, and shutdown with queued/in-flight work.

Every row must converge without duplicate value, stale writes, silent
progression loss, or an expired-worker mutation.

## Current implementation checklist

- [x] Reviewed outbox/capture migrations and indexes.
- [x] Async signer/CEX clients outside business transactions.
- [x] Capture, claim, remote execution, and exact apply phases.
- [x] Trigger-enforced stable SHA-256 remote request identity.
- [x] Immutable economic identity fields and aliases.
- [x] Live lease on every remote mutation.
- [x] Retired weak claim v1.
- [x] Signer durable receipt lookup before sign.
- [x] CEX intent-hash-bound receipt lookup client before submit.
- [x] Unit fixtures for signer/CEX ambiguous response recovery.
- [x] Account/campaign serialization without global FIFO.
- [x] PostgreSQL lease/identity and serialization tests.
- [x] CI configured to require PostgreSQL tests.
- [x] Remote success/application split and aggregate metrics.
- [x] Operator triage/recovery runbook.
- [x] Restrictive evidence retention foreign keys.
- [ ] Land the CEX lookup endpoint in the CEX owner repository.
- [ ] Delete the inert legacy `reconcile_economy` caller.
- [ ] Prove no external request starts before capture commit in black-box runtime.
- [ ] Execute deployed signer/CEX response-loss and process fault matrix.
- [ ] Add reviewed replay/retention controls and deployment thresholds.
- [ ] Bind exact-commit CI/artifacts/environment and reviewer signoff.

Until every unchecked row closes, trusted settlement remains blocked and the
public player market remains disabled.
