---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-003
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Global Lock Order v1

## Rule zero

No signer, CEX, wallet, ledger, arbitrary HTTP, or unbounded filesystem call may
execute while a transaction holds mutable match or campaign row locks.

## Global order

When a transaction needs multiple resources, acquire them in this order and
release all at commit/rollback:

```text
1. global migration advisory lock
2. database/physical-host authority advisory locks
3. fleet instance / route ownership rows
4. match row
5. terminal publication ACK / witness rows
6. match-member rows ordered by player_id
7. campaign rows ordered by campaign_id
8. settlement capture row
9. settlement jobs ordered by campaign_id, job_id
10. transaction-scoped settlement serialization advisory lock
11. operator policy/replay/quarantine rows
```

A code path that cannot obey this order must be redesigned or separately
reviewed with a deadlock proof; it does not improvise another order.

## Read-before-lock scans

Candidate scans use bounded ordering and `SKIP LOCKED` where starvation and
fairness are measured. A scan does not perform remote work and does not claim
success. After selecting an ID, the mutation transaction revalidates all
preconditions under the required locks.

## Settlement phases

### Capture transaction

```text
match -> terminal ACK -> members -> campaigns -> capture/jobs -> commit
```

Produces immutable capture/job fences. No remote call.

### Execute phase

```text
claim transaction commits
remote signer/CEX lookup/submit outside transaction
short lease-fenced result transaction
```

The worker never retains a `Transaction`, `PgConnection`, or row guard across a
remote `await`.

### Apply transaction

```text
capture -> match -> terminal ACK -> campaigns -> jobs -> CAS writes -> commit
```

All campaign fences are validated before the first durable campaign update. The
transaction marks application and final settlement atomically.

## Advisory locks

- Migration lock serializes schema application only.
- Host authority locks fence one authority process for the exact database
  identity/timeline and physical host.
- Settlement serialization lock is transaction-scoped and derived from immutable
  primary account, falling back to campaign namespace.
- Advisory locks do not replace durable live-lease/pending-apply rows after the
  claim transaction commits.

Hash collision consequences must be documented and bounded. The database row
predicate remains the correctness fence.

## Timeouts

Every transaction configures bounded lock/statement timeouts appropriate to its
profile. A timeout rolls back and returns a stable retryable classification; it
never leaves a partially applied business state.

Long maintenance, backup, schema analysis, or operator queries use separate
roles/pools and cannot silently consume player-admission capacity.

## Deadlock tests

Required black-box tests:

- two matches with reversed member/campaign input order;
- two workers for one job;
- same account across different captures;
- unrelated accounts progressing concurrently;
- apply racing new capture;
- operator replay racing claim/takeover;
- fleet drain racing command/checkpoint publication;
- migration startup racing old writer.

Tests assert no unexplained deadlock, bounded latency, one winner where required,
and exact rollback for losers.

## Cancellation

Dropping a request future must release/rescind any initialization reservation.
Dropping a settlement remote future does not transfer a still-live lease; a new
worker waits for expiry/takeover. Shutdown stops admission before drain and does
not hold database transactions during the grace period.

## Observability

Collect:

- lock wait/statement duration by bounded operation code;
- deadlock/timeout counts;
- pool saturation;
- lease age/takeover;
- pending-apply age;
- capture/apply quarantine;
- migration/host-fence health.

Raw SQL, tokens, unbounded IDs, and payload bodies are not metric labels.

## Acceptance

- source and SQL access follow this order;
- generated procedure catalogue records every lock;
- fault/deadlock tests run on supported PostgreSQL versions;
- query plans preserve bounded indexes;
- external-I/O-under-lock scanner and runtime black-box proof both pass.
