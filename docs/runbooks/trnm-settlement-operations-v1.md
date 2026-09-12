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
fabrication, silent queue reset, or bypass of immutable settlement identity.

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

- `remote_request_id` — stable signer/CEX request identity;
- `serialization_key` — account ID or campaign fallback;
- `remote_state` — pending, leased, retryable, succeeded, or dead letter;
- `application_state` — waiting remote, pending apply, applied, or blocked;
- `lease_generation` — current fencing generation;
- `remote_attempts` — bounded remote execution attempts.

### Aggregate health

```sql
select * from public.trnm_online_settlement_metrics_v1;
select * from public.trnm_online_settlement_operator_alerts_v1;
select * from public.trnm_online_settlement_operator_policy_current_v1;
```

The projections include:

- remote pending/leased/retryable/succeeded/dead-letter counts;
- `pending_apply` and applied counts;
- expired lease count;
- oldest eligible and pending-apply ages;
- replay volume and earliest evidence retention deadline;
- the exact policy revision and candidate thresholds used for alerts.

A dashboard must not rename `pending_apply` as “settled.” Candidate defaults are
not production SLO approval.

## 3. Triage order

1. Confirm the exact deployed World, signer, CEX and component-lock revisions.
2. Confirm `trnm-settlement-worker` readiness and PostgreSQL connectivity.
3. Inspect aggregate metrics and the current operator policy revision.
4. Inspect the oldest eligible and oldest pending-apply jobs.
5. Group incidents by `serialization_key`, not globally.
6. Verify signer/CEX lookup availability before considering replay.
7. Preserve job, capture, intent, receipt and terminal-publication evidence.

Never delete the match or campaign to clear a queue. Foreign-key restriction is
intentional economic evidence retention.

## 4. Pending and retryable work

For an eligible pending/retryable job:

- confirm its capture remains `active`;
- confirm `remote_request_id`, `intent_id` and `intent_hash` are stable;
- confirm no live lease or `pending_apply` job owns the same serialization key;
- confirm lookup endpoints are available;
- allow the normal worker claim/retry loop to proceed.

Do not manually change request identity, authorization identity, entitlement
nonce, intent identity or payload hash. Database constraints reject drift.

## 5. Expired leases

An expired lease is recoverable through normal claim v2 takeover:

```text
old owner/generation expires
        ↓
new owner claims with higher generation
        ↓
old worker authorization/attempt/complete/retry/dead-letter writes fail
```

Investigate repeated expiry for signer/CEX latency, pool starvation, process
pauses, unavailable lookup or an oversized batch. Do not extend a lease by
direct SQL.

## 6. Signer ambiguity

When signing may have committed but the response was lost:

1. query `/v1/signer/receipts/{remote_request_id}`;
2. `200` — validate and reuse the durable response;
3. `404` — one exact sign submission is allowed;
4. transport/`5xx` — stop and retry lookup later;
5. any binding mismatch — dead-letter and preserve evidence.

A different signer request ID must never be minted for the same job.

## 7. CEX ambiguity

When a CEX intent may have committed but the response was lost:

1. query the CEX receipt endpoint with exact `intent_id` and
   `x-trnm-intent-sha256`;
2. `200` — validate wrapper ID/hash and `EconomicReceipt::validate_for`;
3. `404` — one exact intent submission is allowed;
4. `409` — immutable intent conflict; fail closed;
5. transport/`5xx` — stop and retry lookup later;
6. malformed or mismatched receipt — dead-letter/fail closed.

A timeout or `5xx` is never equivalent to `404`.

## 8. Pending apply

A `remote_succeeded + pending_apply` job has a durable remote result but has not
yet changed campaign progression.

Check that terminal publication identity, campaign revision/state hash, campaign
head and every captured receipt still match. Do not force
`campaign_applied_at` or advance match settlement manually.

## 9. Dead letters and audited replay

A dead letter is terminal evidence. Before considering replay, collect:

- job and capture IDs;
- stable remote request ID;
- intent ID/hash and serialization key;
- owner/generation/attempt counts;
- signer/CEX lookup evidence;
- exact component revisions and incident ticket.

Replay is permitted only through:

```sql
select public.trnm_online_authorize_settlement_replay_v1(
    :request_id,
    :job_id,
    :capture_id,
    :intent_id,
    :intent_hash,
    :remote_request_id,
    :operator_id,
    :change_ticket,
    :reason
);
```

The caller must use a dedicated database role that has explicit `EXECUTE` on
this function. The migration revokes execution from `PUBLIC`.

The function requires all of the following:

- job and capture are both `dead_letter`;
- exact job/capture/intent/hash/remote-request binding;
- no durable receipt, remote completion or campaign-apply marker;
- a valid operator identity, change ticket and bounded reason;
- an active append-only policy revision.

A successful authorization:

- appends an immutable replay evidence row;
- preserves the previous state, attempt counts and error;
- reactivates the same capture and stable remote identity;
- sets `remote_attempts = 15`, so normal processing receives exactly one more
  attempt before returning to dead letter;
- never deletes or overwrites prior replay or policy evidence.

An exact duplicate `request_id` is idempotent. Reusing the request ID with
changed material fails closed. A new request while the job is already retryable,
leased, succeeded, applied, or receipt-bearing fails closed.

Direct SQL such as `update ... set state = 'pending'` remains prohibited.

## 10. Policy, retention and alert changes

Policy changes are append-only:

```sql
select public.trnm_online_append_settlement_operator_policy_v1(
    :retention_days,
    :dead_letter_threshold,
    :expired_lease_threshold,
    :oldest_eligible_seconds,
    :oldest_pending_apply_seconds,
    :replay_24h_threshold,
    :approved_by,
    :change_ticket,
    :reason
);
```

The database enforces a minimum retention of 365 days. The candidate default is
2555 days. Production retention, archive, purge, backup and legal-hold behavior
must be approved separately; this migration intentionally exposes no purge
function.

Alert routing must bind the exact policy revision, environment, dashboard,
paging destination and accountable operator. Candidate thresholds alone earn no
production evidence.

## 11. Shutdown and incident preservation

During shutdown:

- stop new work admission;
- allow bounded in-flight operations to finish or expire;
- preserve leases for generation-fenced takeover;
- do not classify ambiguous remote work without lookup;
- preserve logs without credentials or signing material.

A cancelled or interrupted fault run earns no release credit. Record exact
commit, artifact, environment, commands, failure point and limitations.

## 12. Prohibited actions

- deleting settlement, replay or policy evidence;
- updating, deleting or truncating append-only replay/policy tables;
- editing immutable IDs or hashes;
- setting `campaign_applied_at` by hand;
- changing dead letter state by direct SQL;
- granting replay functions to broad/public roles;
- treating lookup timeout as `404`;
- submitting after an ambiguous lookup;
- sharing signer and game-authority credentials;
- enabling public market or public online based on this runbook.
