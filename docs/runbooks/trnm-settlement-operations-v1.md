---
status: current-candidate
owner: trillionnium-world
contract: trnm_settlement_outbox_v1
applies_to:
  - WORLD-P0-001
  - trnm-settlement-worker
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# TRNM Settlement Operations v1

## 1. Scope

This runbook covers the migration-era World settlement worker only. It does not
authorize public player-market operation, direct wallet mutation, manual receipt
fabrication, or silent replay of terminal records.

The operator must distinguish:

- remote execution state;
- campaign application state;
- match finalization state.

`remote_succeeded` with `pending_apply` is not a completed settlement.

## 2. Primary projections

### Per-job state

```sql
select *
  from public.trnm_online_settlement_job_status_v1
 order by updated_at, job_id;
```

Important columns:

- `remote_request_id` — stable signer request identity;
- `serialization_key` — account ID or campaign fallback;
- `remote_state` — pending, leased, retryable, succeeded, or dead letter;
- `application_state` — waiting remote, pending apply, applied, or blocked;
- `lease_generation` — current fencing generation;
- `remote_attempts` — bounded remote execution attempts.

### Aggregate health

```sql
select * from public.trnm_online_settlement_metrics_v1;
```

The projection includes:

- remote pending/leased/retryable/succeeded/dead-letter counts;
- `pending_apply` and applied counts;
- expired lease count;
- oldest eligible job age;
- oldest pending-apply age;
- maximum remote attempts.

A dashboard may render this view, but it must not rename `pending_apply` as
“settled.”

## 3. Triage order

1. Confirm the exact deployed World, signer, and CEX component revisions.
2. Confirm `trnm-settlement-worker` readiness and PostgreSQL connectivity.
3. Inspect aggregate metrics.
4. Inspect the oldest eligible and oldest pending-apply jobs.
5. Group incidents by `serialization_key`, not globally.
6. Verify signer/CEX lookup availability before considering a retry problem.
7. Preserve job, capture, intent, receipt, and terminal-publication evidence.

Never delete the match or campaign to clear a queue. Foreign-key restriction is
intentional economic evidence retention.

## 4. Pending and retryable work

For an eligible pending/retryable job:

- confirm its capture remains `active`;
- confirm `remote_request_id`, `intent_id`, and `intent_hash` are stable;
- confirm no live lease or `pending_apply` job owns the same serialization key;
- confirm lookup endpoints are available;
- allow the normal worker claim/retry loop to proceed.

Do not manually change `remote_request_id`, authorization request ID,
entitlement nonce, intent ID, or intent hash. Database constraints and triggers
reject those changes because they are durable identities.

## 5. Expired leases

An expired lease is recoverable through normal claim v2 takeover:

```text
old owner/generation expires
        ↓
new owner claims with higher generation
        ↓
old worker authorization/attempt/complete/retry/dead-letter writes fail
```

Investigate repeated lease expiry for:

- signer/CEX latency beyond the configured lease;
- database pool starvation;
- process pauses or shutdown handling;
- unavailable receipt lookup;
- excessively large batch size.

Do not extend a lease by direct SQL. Adjust reviewed configuration and rerun the
fault evidence matrix.

## 6. Signer ambiguity

When signing may have committed but the response was lost:

1. query `/v1/signer/receipts/{remote_request_id}` using the signer principal;
2. `200` — validate and reuse the durable response;
3. `404` — one exact sign submission is allowed;
4. transport/`5xx` — stop and retry lookup later; do not sign;
5. hash, issuer, key, signature, or receipt mismatch — dead-letter/fail closed
   and preserve evidence.

A different signer request ID must never be minted for the same job.

## 7. CEX ambiguity

When the CEX intent may have committed but the response was lost:

1. query the CEX receipt endpoint with exact `intent_id` and
   `x-trnm-intent-sha256`;
2. `200` — validate wrapper ID/hash and `EconomicReceipt::validate_for`;
3. `404` — one exact intent submission is allowed;
4. `409` — immutable intent conflict; fail closed;
5. transport/`5xx` — stop and retry lookup later; do not submit;
6. malformed or mismatched receipt — dead-letter/fail closed.

The CEX owner-repository endpoint remains a deployment blocker until its exact
revision and evidence are locked.

## 8. Pending apply

A `remote_succeeded + pending_apply` job has a durable remote result but has not
yet changed campaign progression.

Check:

- terminal publication identity and cold seal remain exact;
- campaign revision and state hash still match the capture;
- campaign head still contains the captured intent/hash;
- every job in the capture has a validated receipt;
- no prior apply transaction is still running.

A stale capture must be marked stale through reviewed application logic. Do not
force `campaign_applied_at` or advance match settlement manually.

## 9. Dead letters

A `dead_letter` record is terminal evidence, not a queue item to edit in place.
Collect:

- job and capture IDs;
- stable remote request ID;
- intent ID/hash and serialization key;
- owner/generation/attempt counts;
- signer/CEX lookup responses;
- receipt or mismatch evidence;
- exact component revisions and timestamps.

Operator replay requires a separately versioned control that creates an
auditable new execution generation without overwriting the historical terminal
record. That replay control is not yet approved; direct SQL replay is prohibited.

## 10. Alerting recommendations

Alert when any of the following exceeds the reviewed environment threshold:

- oldest eligible age;
- oldest pending-apply age;
- expired lease count or reclaim rate;
- retryable backlog;
- dead-letter count;
- maximum remote attempts;
- signer/CEX lookup error rate;
- immutable binding mismatch;
- campaign revision conflict.

Thresholds must be defined by deployment profile and attached to release
evidence. This source runbook does not invent production SLO credit.

## 11. Shutdown and incident preservation

During shutdown:

- stop new work admission;
- allow bounded in-flight network/database operations to finish or expire;
- preserve leased rows for generation-fenced takeover;
- do not mark ambiguous remote work successful or failed without lookup;
- preserve logs without credentials or private signing material.

A cancelled or interrupted fault run earns no release credit. Record the exact
commit, artifact, environment, commands, failure point, and limitations.

## 12. Prohibited actions

- deleting settlement evidence to unblock a match;
- editing immutable IDs or hashes;
- setting `campaign_applied_at` by hand;
- changing `state` from dead letter to pending;
- treating lookup timeout as `404`;
- submitting after an ambiguous lookup;
- sharing signer and game-authority credentials;
- enabling public market or public online based on this runbook.
