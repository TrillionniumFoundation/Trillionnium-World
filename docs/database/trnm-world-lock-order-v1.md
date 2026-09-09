---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-003
last_reviewed: 2026-09-05
review_due: 2026-09-19
---

# Trillionnium World Global Lock Order v1

## Rule zero

No signer, CEX, wallet, ledger, arbitrary HTTP, or unbounded filesystem call may
execute while a transaction holds mutable match or campaign row locks.

## Global order

When a transaction needs multiple resources, acquire them in this order and
release all at commit/rollback:

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
unlocked capture-to-match hint
match -> terminal ACK -> members -> campaigns -> capture -> jobs
-> validate all fences -> CAS writes -> commit
```

The initial capture read acquires no row lock and grants no authority. After
locking the earlier classes, the capture query rechecks `capture_id`, `match_id`
and `state=active` under its row lock. If the record changed or disappeared,
commit a no-op or reject; never continue from the hint alone. All campaign
fences are validated before the first durable campaign update. The transaction
marks application and final settlement atomically.

Both phases explicitly lock the ACK and member rows before campaigns. Campaign
locks follow `campaign.campaign_id, member.player_id`, not player order alone.
Job locks follow `campaign_id, job_id`. Identifier ordering uses the database
profile consistently; changing collation requires fresh ordering/compatibility
evidence. Reusing a lock already held does not acquire an earlier class anew.

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

## Revision 2 executable scope and remaining work

The machine source for the shared order block is
`docs/database/trnm-world-lock-order-v2.json`. The historical filename of this
document is retained to preserve inbound links; the revised block supersedes
its earlier capture-before-match apply example and advisory-after-job order.
`trnm-world-postgres-contract-v1.md` carries the identical checked block.

`check-trnm-settlement-lock-order.py` checks the known ordinary source functions,
SQL lock clauses, ordering keys, exact hint revalidation and both rendered blocks.
It is a conservative source regression guard, not Rust type checking, SQL plan
analysis, a complete call graph or a proof against every lock cycle.

`settlement_lock_order_tests.rs` calls the actual private apply/campaign functions
against a uniquely created disposable PostgreSQL database. One test holds match
and proves the waiting apply function has not locked capture; another reverses
player/campaign identity ordering and proves campaign-a locks before campaign-z.
The test role must have CREATEDB and permission to inspect its own sessions.
No existing database or real match is reset. Missing database configuration
receives no database-evidence credit; the required CI profile sets
`TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST=1` to make absence an error.

The corrected explicit acquisitions are only one part of the global contract.
Implicit foreign-key/trigger locks, operator replay and quarantine procedures,
fleet/terminal publication, query plans, all contention matrices, timeout and
cancellation behavior still need independent SQL/runtime verification. In
particular, a multi-table `FOR UPDATE OF job_row, capture` does not by itself
prove the intended relative order. Existing migrations are not edited by this
repair; any necessary SQL correction uses an append-only successor migration.
Do not mark WORLD-P1-003, direct-source publication, exact-head qualification or
production readiness closed from these source checks.

```bash
python3 scripts/check-trnm-settlement-lock-order.py
python3 scripts/test-trnm-settlement-lock-order.py
TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST=1 cargo test --manifest-path trillionnium/Cargo.toml \
  -p trnm-game-server --all-targets --locked actual_settlement_functions_obey_lock_order -- --nocapture
```

The last command also requires `TRNM_SETTLEMENT_TEST_DATABASE_URL` to point to an
approved disposable test cluster; it is not authorization to use production.
