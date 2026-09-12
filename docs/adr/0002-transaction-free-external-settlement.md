---
status: accepted
date: 2026-08-27
owner: Trillionnium-World
applies_to:
  - game-server
  - campaign-progression
  - cex-settlement
  - entitlement-signer
---

# ADR-0002: External Settlement Must Run Outside Mutable Database Transactions

## Context

The current compatibility server can lock a completed match and its campaign
rows, deserialize the campaign, and then call the entitlement signer and CEX
ledger through a blocking HTTP client before the PostgreSQL transaction commits.
A dependency timeout can therefore block a Tokio worker and retain mutable row
locks for the duration of multiple network operations.

This couples external latency and ambiguous network outcomes to database lock
hold time. It increases pool starvation, tail latency, deadlock exposure and the
blast radius of signer/CEX degradation.

## Decision

External signer, wallet, ledger, custody, webhook or other network I/O is
forbidden while a transaction owns mutable match, campaign, settlement or
inventory row locks.

Settlement is implemented as a durable outbox with three phases:

1. **Claim transaction** — select an eligible job, bind its immutable intent
   fingerprint, assign a lease owner/generation/expiry, and commit.
2. **External execution** — call signer and CEX asynchronously outside a
   database transaction, using the immutable job identity and idempotency key.
3. **Apply transaction** — verify lease generation and exact receipt binding,
   update campaign/projection state idempotently, and commit.

## Required state

Every job records at least:

- contract version and deterministic job ID;
- match, campaign and intent identity;
- expected campaign revision;
- immutable intent hash;
- attempt count;
- lease owner, generation and expiry;
- retry schedule and last bounded error;
- terminal receipt ID/hash or dead-letter reason;
- timestamps needed for backlog and age telemetry.

## Required behavior

- Expired leases may be reclaimed with a strictly higher generation.
- A stale owner/generation cannot apply a result after reclaim.
- Exact duplicate success is idempotent.
- A mismatched receipt fails closed.
- Signer success followed by response loss and ledger success followed by
  response loss are recovered by querying/retrying the same immutable request.
- Maximum attempts and dead-letter policy are explicit.
- Unrelated jobs continue when one job is poisoned.
- Per-account or per-campaign ordering is explicit where economic semantics
  require it.

## Consequences

Positive:

- database locks are bounded by database work;
- external dependency degradation does not directly pin campaign rows;
- crash recovery and ambiguous commits have explicit durable states;
- settlement backlog is observable and independently scalable.

Costs:

- progression may remain visibly pending while settlement completes;
- another durable table/state machine and worker are required;
- operators need retry/dead-letter runbooks and metrics;
- exact receipt lookup support is required from signer/CEX.

## Transitional implementation

`trillionnium/tools/trnm-settlement-outbox-contract` defines the first invariant
contract and tests. It does not by itself migrate the runtime. Runtime promotion
requires a database migration, async worker, replacement of the legacy locked
network path, and black-box crash/ambiguous-commit evidence.

## Enforcement

- Code review rejects external client calls inside mutable database transactions.
- The runtime migration must include a static guard or test that detects the
  prohibited legacy pattern.
- Fault tests cover every transition and lease-takeover boundary.
