---
status: current-design
owner: trillionnium-world
contract: trnm_settlement_outbox_v1
runtime_status: foundation-only
related_adr: ../adr/0002-transaction-free-external-settlement.md
last_reviewed: 2026-08-27
---

# TRNM Settlement Outbox v1

## Purpose

This contract separates durable game state from external signer/CEX latency.
It is the migration target for the compatibility server's current settlement
loop, which can perform blocking external calls while campaign rows are locked.

The standalone reference implementation lives at
`trillionnium/tools/trnm-settlement-outbox-contract`. It defines state-machine
invariants and tests. It is not yet wired into production runtime and therefore
does not close the settlement P0 by itself.

## Job identity

A job key contains:

- `match_id`;
- `campaign_id`;
- `intent_id`.

The deterministic job ID is the contract domain plus an unambiguous,
length-prefixed hexadecimal encoding of the three UTF-8 fields. The ID is an
idempotency identity, not a secret or a cryptographic proof. The immutable
`intent_hash` remains the cryptographic binding to the serialized economic
intent.

## State machine

```text
Pending
  -> Leased(owner, generation, expires_at)
       -> Succeeded(receipt_id, receipt_hash)
       -> Retryable(next_attempt_at, last_error)
       -> DeadLetter(reason)

Retryable when due -> Leased with a higher generation
Expired Leased     -> Leased with a higher generation
Succeeded          -> terminal, exact duplicate completion is idempotent
DeadLetter         -> terminal
```

A lease increments both `attempts` and `lease_generation`. A worker may mutate a
leased job only when owner and generation match the current lease and the lease
has not expired. A reclaimed lease invalidates every stale worker result.

## Database target

The runtime migration should use a table equivalent to:

```sql
create table trnm_settlement_jobs (
    job_id text primary key,
    contract_version text not null,
    match_id uuid not null,
    campaign_id text not null,
    intent_id text not null,
    expected_campaign_revision bigint not null,
    intent_hash text not null,
    state text not null,
    attempts integer not null,
    lease_owner text,
    lease_generation bigint not null,
    lease_expires_at timestamptz,
    next_attempt_at timestamptz,
    last_error text,
    receipt_id text,
    receipt_hash text,
    created_at timestamptz not null,
    updated_at timestamptz not null,
    completed_at timestamptz,
    unique (match_id, campaign_id, intent_id)
);
```

Exact DDL is deferred until the runtime integration tranche because it must be
reviewed with existing campaign revision and terminal-publication locks.

## Claim transaction

The claim transaction:

1. selects an eligible `Pending`, due `Retryable`, or expired `Leased` job using
   `FOR UPDATE SKIP LOCKED`;
2. verifies the job is not terminal and attempts remain;
3. increments attempt and generation;
4. writes owner and expiry;
5. commits before any network call.

The transaction does not deserialize and rewrite campaign state unless needed
to verify the immutable intent/revision binding. It never calls signer or CEX.

## External execution

The worker uses asynchronous clients and the immutable job data. Recommended
sequence for positive rewards:

1. retrieve or create the exact entitlement using job ID as request identity;
2. query signer receipt when a previous response is ambiguous;
3. submit/query the exact CEX intent using the immutable intent ID/hash;
4. produce a `SettlementReceiptBindingV1`;
5. enter the apply transaction.

A network timeout is not evidence that the remote side failed. Retrying the same
idempotency identity or querying the receipt is mandatory.

## Apply transaction

The apply transaction:

1. locks the settlement job;
2. verifies current owner, generation and nonexpired lease;
3. verifies receipt job/intent/hash binding;
4. locks the campaign and verifies expected revision or an allowed idempotent
   successor;
5. applies receipt/progression exactly once;
6. marks the job `Succeeded` and commits.

A stale lease, mismatched receipt, unexpected revision or quarantined terminal
projection fails closed.

## Retry and dead letter

- Maximum attempts in the reference contract: 16.
- Lease duration must be positive and no more than five minutes.
- Retry time must be strictly after the failed attempt time.
- Errors are bounded before persistence and must not contain credentials.
- The final failed attempt becomes `DeadLetter`.
- Operator replay requires a new lease generation and an auditable reason; it
  must not overwrite the historical terminal record silently.

## Ordering

Jobs that can change the same spendable or reversible balance must be serialized
by account/campaign semantics. Unrelated jobs should execute concurrently.
Global FIFO is prohibited because one poisoned job must not block refunds,
chargebacks or unrelated players.

## Telemetry

Required metrics:

- eligible, leased, retryable, succeeded and dead-letter counts;
- oldest eligible job age;
- lease expiration/reclaim count;
- attempts histogram;
- signer and CEX latency/outcome;
- ambiguous remote outcome count;
- stale-generation rejection count;
- receipt mismatch count;
- campaign revision conflict count;
- settlement completion lag from match terminal publication.

Required logs bind job ID, match ID, campaign ID, intent ID, attempt and lease
generation, but never credentials or private entitlement material.

## Fault matrix

At minimum inject process death or response loss:

1. before claim commit;
2. after claim commit and before signer call;
3. after signer commit and before response;
4. after signer response and before CEX call;
5. after CEX commit and before response;
6. after receipt verification and before apply transaction;
7. during campaign update before commit;
8. after apply commit and before worker acknowledgement;
9. after lease expiry while the old worker is still running;
10. during dead-letter transition.

Every row must converge without duplicate payout or silent progression loss.

## Runtime migration checklist

- [ ] Add reviewed migration and indexes.
- [ ] Add async signer/CEX clients.
- [ ] Add claim worker and per-account/campaign scheduling.
- [ ] Add exact remote receipt query/idempotency support.
- [ ] Add apply transaction and campaign revision policy.
- [ ] Remove the legacy transaction-held `reconcile_economy` call.
- [ ] Add static guard for external calls under mutable transactions.
- [ ] Add dashboards and operator runbook.
- [ ] Execute the complete fault matrix.
- [ ] Bind promoted release and evidence to the exact commit/binary/toolchain.
