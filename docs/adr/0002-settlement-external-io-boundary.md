# ADR-0002: Settlement External-I/O Boundary

Status: **accepted for implementation**  
Date: 2026-08-27  
Owners: World economy and persistence maintainers  
Related plan: `docs/development/trnm-world-development-plan-v3.md`

## Context

The migration-era World Online Authority reconciles pending campaign economic intents against an isolated signer and CEX ledger. The economy trait is synchronous and the current CEX implementation can perform blocking HTTP.

A database transaction that retains match or campaign row locks while invoking that backend couples remote latency, DNS/transport failures and ambiguous commits to PostgreSQL lock duration, connection-pool occupancy and Tokio runtime availability. Wrapping the same operation in `spawn_blocking` alone does not fix the database-lock coupling.

## Decision

Settlement is split into three explicit phases:

1. **Capture** — a short PostgreSQL transaction locks and validates the exact terminal match and member campaigns, captures revision/state-hash/serialized campaign plus stable intent identities, and commits.
2. **Execute** — signer/CEX reconciliation runs outside every PostgreSQL transaction on a bounded execution lane.
3. **Apply** — a new short PostgreSQL transaction re-locks the exact rows, revalidates terminal publication evidence and captured revision/state hash, applies current reconciled state, and advances settlement idempotently.

No database transaction or pooled connection is passed into the execute phase. No external client is invoked from capture or apply.

## Exactness contract

A capture contains, at minimum:

- match ID and exact terminal publication identity;
- campaign ID;
- stored campaign revision;
- stored state hash;
- serialized campaign value;
- stable pending economic intent/compensation identities.

Apply is permitted only when:

- the match remains complete and pending;
- terminal publication remains acknowledged and cold-sealed;
- the exact terminal marker still matches the durable match tuple;
- every captured campaign exists and has the same revision and state hash;
- the executed result binds the same capture and campaign identity.

All captured rows are validated before the first write. Any mismatch aborts the complete apply transaction with a stale-capture outcome.

## Idempotency contract

External execution preserves original intent IDs, entitlement request IDs and authoritative metadata. A remote success followed by response loss, local conflict, rollback or process crash is retried with the same identity. CEX/signer idempotency must return the existing receipt or an exact typed conflict; it must not mint a replacement value event.

A fresh capture may contain campaign state already advanced by a previous remote operation. Reconciliation consumes the durable receipt/cursor and converges without duplicate value.

## Concurrency and backpressure

- Match candidates are selected with a bounded batch and `FOR UPDATE SKIP LOCKED` during capture.
- External execution uses a bounded queue and bounded concurrency.
- Apply locks match and campaign rows in deterministic order.
- Concurrent workers may execute duplicate remote retries only under stable idempotency identities; only one exact apply may commit.
- A stalled dependency holds no business-row lock.
- Shutdown stops accepting new work and leaves un-applied captures safely recapturable.

## Failure semantics

| Failure boundary | Durable behavior |
| --- | --- |
| Before capture commit | No capture and no remote work |
| After capture commit, before execute | No local mutation; fresh capture is safe |
| Signer committed, response lost | Retry the same signer request identity |
| Signer committed, CEX rejected/failed | Match remains pending; retry exact intent |
| CEX committed, response lost | Retry exact intent and obtain existing receipt |
| Campaign changes during execute | Apply rejects stale capture without writes |
| Apply rolls back after remote success | Fresh capture/reconcile converges idempotently |
| Worker is cancelled | No transaction remains open; match remains pending |
| One member remains incomplete | Safe campaign updates may commit; match remains pending |

Hard corruption, identity conflict or malformed authoritative metadata remains fail-closed and follows the existing quarantine/dead-letter policy. Transport ambiguity is not silently converted to permanent failure.

## Observability

Required metrics/log fields:

- capture attempts, successful captures, lock skips and capture duration;
- execution queue depth, saturation and duration per dependency;
- apply success, stale rejection, retry and rollback;
- ambiguous remote outcome count and oldest age;
- pending match count and oldest pending settlement age;
- match ID, campaign ID and non-secret intent ID in structured logs.

Player sessions, service tokens, private keys and full entitlement signatures must not be logged.

## Tests required before old-path removal

- source boundary test proving no `reconcile_economy` call exists in transaction-owning settlement code;
- database integration test proving external transport begins only after capture commit;
- campaign revision and state-hash drift tests;
- two-worker contention test;
- signer success/response loss and CEX ambiguous-commit tests;
- remote success/apply rollback test;
- cancellation, graceful shutdown and process-kill tests at every phase boundary;
- exact intent-ID preservation test;
- existing CEX exact-once, restart and terminal-publication suites.

## Consequences

Benefits:

- network latency cannot extend PostgreSQL business locks;
- worker/thread starvation is bounded independently from database contention;
- stale local writes are rejected;
- ambiguous remote outcomes are replay-safe;
- settlement becomes independently modular and testable.

Costs:

- remote work may be repeated and therefore requires strong idempotency;
- capture/apply state and metrics add implementation complexity;
- a later fully asynchronous economy backend still needs the same transaction boundary;
- tests require controllable signer/CEX fault injection.

## Rejected alternatives

- **Keep the transaction and use `spawn_blocking`:** rejected because row locks remain coupled to remote latency.
- **Hold a database lease while executing remotely:** rejected as the primary correctness mechanism; leases may coordinate scheduling but cannot authorize stale apply.
- **Write a new intent ID after ambiguity:** rejected because it permits duplicate value.
- **Mark the match failed after any timeout:** rejected because timeout does not prove remote non-commit.
- **Apply each campaign independently without validating the set first:** rejected because it can commit a mixed stale/current settlement tuple.

## Rollout and rollback

The split path is introduced with focused tests, then exercised by existing local E2E and new fault suites. The old transaction-spanning path is removed before P0 closure. A temporary rollout switch must be fail-closed, observable, default to the reviewed path after validation, and include an explicit removal PR.

Rolling back to the old path is a safety regression and earns no release credit. Schema additions, if any, must remain backward-readable until the rollback window closes.
