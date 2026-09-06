---
status: current-candidate
owner: trillionnium-world-operations
applies_to: trnm-settlement-worker
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Settlement Worker Shutdown and Quarantine Runbook v1

## Scope

Use this runbook for planned stop, emergency stop, repeated poison work, expired leases and recovery of the World settlement worker. It does not authorize manual receipt creation, identity changes or wallet mutation.

## Preconditions

- identify exact release selector, binary SHA-256, source commit/tree and database target;
- verify operator role and change/incident ticket;
- capture current settlement metrics and database timeline/system identity;
- ensure signer/CEX receipt lookup endpoints are reachable unless the incident specifically concerns them;
- never paste credentials into tickets, commands or evidence.

## Planned shutdown

1. Record UTC start and current metrics.
2. Send SIGTERM through the service manager:

   ```bash
   systemctl --user stop trnm-settlement-worker.service
   ```

3. Confirm logs show `draining` and no new capture/claim after the signal.
4. Wait for the configured application shutdown grace plus process margin.
5. Confirm process exited and service manager did not require SIGKILL.
6. Query metrics:
   - live leases may remain only for work cancelled after grace;
   - no lease owner/generation is manually cleared;
   - remote-succeeded/pending-apply rows remain durable;
   - no campaign apply is partially committed.
7. Record final metrics and evidence hashes.

## Grace expiry

When the worker cannot drain before grace:

- allow the process to exit/cancel local futures;
- do not extend or clear leases by hand;
- preserve remote request identity;
- after lease expiry, replacement worker must lookup signer/CEX receipts before submit;
- treat any missing/ambiguous response as retryable until lookup resolves it;
- escalate repeated grace expiry as capacity/remote-latency incident.

## Emergency stop

For active credential compromise, runaway requests or suspected duplicate-value behavior:

1. disable worker admission at service/orchestration layer;
2. stop the worker;
3. revoke/rotate the affected credential through the owning system;
4. keep database evidence immutable;
5. query remote receipts for every in-flight `remote_request_id`/intent;
6. quarantine affected jobs/captures;
7. do not replay until incident owner and CEX/security approve;
8. capture component lock, timeline and raw logs.

## Quarantine triage

### Identify scope

- match-level: capture cannot be created because terminal/campaign invariants fail;
- capture-level: terminal/campaign fences or job set cannot be applied;
- job-level: claimed row, intent, authorization or receipt is invalid;
- remote-level: signer/CEX outcome remains ambiguous beyond policy;
- database-level: migration/timeline/ownership uncertainty.

### Required quarantine record

- exact scope and ID;
- phase and error class;
- bounded diagnostic and SHA-256;
- first/last observed UTC;
- observation count;
- retry-not-before;
- source release, worker ID and database timeline;
- related capture/job/intent/remote request/receipt identities;
- incident/change ticket;
- operator review status.

### Allowed actions

- inspect immutable rows and views;
- verify intent/receipt hashes;
- query signer/CEX by stable identity;
- append operator review/decision;
- authorize one exact replay only under the replay contract;
- recapture stale campaign state while preserving remote identity;
- leave permanently closed evidence retained.

### Forbidden actions

- direct update of intent ID/hash, remote request ID, receipt or applied timestamp;
- arbitrary state reset to pending;
- deleting capture/job/operator/quarantine evidence;
- treating timeout as 404;
- replaying with a new remote identity;
- marking a remote success as campaign-applied outside the apply transaction;
- manually advancing match settlement marker.

## Repeated poison work

If the same diagnostic hash exceeds the policy threshold:

1. extend quarantine backoff;
2. stop automatic retries for that scope;
3. keep unrelated serialization keys processing;
4. open a defect with exact source and raw evidence;
5. require code/data migration review before retry authorization;
6. update alert/retention evidence without mutating prior records.

## Restart

Before restart:

- migrations/checksums match release;
- database host/instance/timeline fence is healthy;
- signer attestation and CEX readiness pass;
- credentials are distinct and valid;
- release selector is exact and verified;
- no emergency disablement remains active.

After restart:

1. confirm worker identity and configuration bounds;
2. observe expired lease takeover;
3. verify lookup-before-submit on recovered work;
4. verify unrelated-key progress;
5. verify pending-apply drains through exact campaign CAS;
6. record metrics and evidence.

## Escalation triggers

- suspected duplicate value;
- receipt/intent/hash mismatch;
- stale worker mutation accepted;
- old primary/process publishes after failover;
- quarantine growth above policy;
- oldest eligible/pending-apply age above threshold;
- repeated SIGKILL or shutdown grace expiry;
- operator evidence mutation attempt;
- missing raw evidence or component lock.

## Evidence closure

The incident/change closes only after:

- exact before/after state and receipt inventory;
- no duplicate/lost value conclusion or explicit remediation;
- root cause and regression test;
- recovery/rollback result;
- raw artifact hashes;
- reviewer decision;
- gate/status update with limitations and expiry.