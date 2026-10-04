# Settlement Fault Evidence Plan v1

Status: **current P0 test design**  
Owners: World economy, persistence and reliability maintainers  
Decision: ADR-0002  
Gate affected: `trusted_cex_settlement`

## 1. Objective

Prove that the capture, execute and apply settlement path preserves exact-once economic meaning when PostgreSQL, the signer, CEX, the process or the network fails at every durable boundary. Passing unit tests or source scans alone is insufficient.

## 2. Test environment contract

The harness provisions:

- a dedicated PostgreSQL database with the exact migration ledger;
- one game-server settlement worker with configurable queue, concurrency and timeouts;
- a controllable signer endpoint;
- a controllable CEX endpoint with durable idempotency storage;
- a transport proxy able to delay, drop, duplicate or sever responses after upstream commit;
- an inspector connection that records locks, transactions, campaign state, terminal markers and settlement state;
- exact logs, metrics and artifacts bound to the tested commit.

Every run records PostgreSQL version, system identifier and timeline; signer/CEX build IDs; World commit, tree and binary hash; toolchain; configuration; fault schedule; timestamps; result; and limitations.

## 3. Deterministic fixture

Each scenario creates:

- one exact complete match with acknowledged and cold-sealed terminal publication evidence;
- two member campaigns with known revision and state hash;
- at least one value-releasing economic intent with stable intent ID and authoritative match/result/participants metadata;
- an expected CEX receipt and final campaign/settlement state;
- an unrelated pending match used to detect lock or pool starvation.

Fixture creation is idempotent and validates its own hashes before fault injection.

## 4. Phase instrumentation

The implementation exposes structured events or an equivalent test synchronization surface:

```text
settlement.capture.begin
settlement.capture.locked
settlement.capture.committed
settlement.execute.begin
settlement.signer.request
settlement.signer.response
settlement.cex.request
settlement.cex.response
settlement.execute.complete
settlement.apply.begin
settlement.apply.validated
settlement.apply.committed
settlement.apply.stale
```

Events carry match, campaign and non-secret intent correlation. The fault controller may pause, disconnect or kill at each event.

## 5. Core assertions

For every scenario:

- no external request timestamp precedes `capture.committed`;
- no PostgreSQL business transaction remains open during execute;
- match and campaign row locks are absent during injected external delay;
- unrelated settlement and readiness traffic remains serviceable within the test bound;
- stable intent and signer request IDs are reused after retry;
- at most one durable CEX value event exists for each intent ID;
- campaign state never reflects a receipt that is not durably bound;
- match becomes `settled` only after every required campaign is fully reconciled;
- stale apply writes nothing;
- terminal publication evidence never regresses;
- process restart converges without operator data edits.

## 6. Scenario matrix

### Capture phase

| ID | Fault | Expected result |
| --- | --- | --- |
| C01 | Kill before match lock | No capture and no remote request |
| C02 | Kill after match lock before campaign read | Transaction rolls back and no remote request |
| C03 | Kill after campaign read before commit | Transaction rolls back and no remote request |
| C04 | PostgreSQL disconnect before capture commit | No execute; match remains pending |
| C05 | Terminal marker changes before capture validation | Capture rejects fail-closed |
| C06 | Campaign state hash differs from serialized value | Capture rejects or quarantines corruption |
| C07 | Two workers capture the same match | One wins the lock; the other skips or later recaptures safely |
| C08 | First candidate is locked by another session | Worker uses `SKIP LOCKED` and services an unrelated match |

### Execute phase — signer

| ID | Fault | Expected result |
| --- | --- | --- |
| S01 | Signer unavailable before request | Pending retry and no database lock |
| S02 | Signer delays beyond timeout without commit | Pending retry with the same request ID |
| S03 | Signer commits and response is dropped | Retry returns the exact existing signature or receipt |
| S04 | Duplicate signer response | Accepted only when request hash, key ID and signature binding are exact |
| S05 | Signer returns a different request ID or hash | Hard fail or quarantine; no CEX request |
| S06 | Signer key rotates between retries | Old request resolves by registry policy or exact typed conflict; never silently reissued under a new identity |
| S07 | Worker is cancelled during signer delay | No open database transaction; work remains recapturable |

### Execute phase — CEX

| ID | Fault | Expected result |
| --- | --- | --- |
| E01 | CEX unavailable before request | Pending retry; signed entitlement remains reusable under policy |
| E02 | CEX rejects invalid authoritative metadata | Hard fail or quarantine; no local value is applied |
| E03 | CEX commits and response is dropped | Retry exact intent and return the existing receipt |
| E04 | CEX response times out after commit | Ambiguous outcome; exact retry or query converges |
| E05 | Duplicate or out-of-order response | Exact intent and request binding is required |
| E06 | CEX returns a receipt for another account or intent | Hard fail or quarantine |
| E07 | Signer succeeds but CEX remains unavailable across restart | Durable pending work and stable identities survive |
| E08 | Two workers execute the same capture | CEX idempotency permits one value event; apply remains exact |

### Between execute and apply

| ID | Fault | Expected result |
| --- | --- | --- |
| B01 | Process is killed after remote success | Fresh capture reconciles the existing receipt |
| B02 | Campaign revision changes | Apply returns stale with zero writes |
| B03 | Campaign JSON changes without stored-hash update | Corruption is detected; no apply |
| B04 | Terminal ACK or settlement tuple changes | Apply rejects exactness loss |
| B05 | Match is quarantined | No apply or further remote value |
| B06 | One campaign changes and one does not | Validate the complete set before writing either campaign |

### Apply phase

| ID | Fault | Expected result |
| --- | --- | --- |
| A01 | Kill before locks | No local write; fresh capture converges |
| A02 | Kill after locks before validation | Rollback |
| A03 | Kill after validation before first write | Rollback |
| A04 | Kill after campaign writes before marker update | One transaction rolls back all writes |
| A05 | Kill after marker update before match update | One transaction rolls back all writes |
| A06 | PostgreSQL disconnect during commit | Treat outcome as ambiguous; reload exact durable state before retry |
| A07 | Serialization or deadlock error | Bounded retry from fresh state |
| A08 | Two exact applies race | One commits; the second observes settled or stale and writes nothing |
| A09 | First member complete, second still pending | Persist only one fully validated apply set; match and ACK stay pending |
| A10 | Every member complete | ACK and match advance to settled atomically |

### Lifecycle and capacity

| ID | Fault | Expected result |
| --- | --- | --- |
| L01 | Graceful shutdown with queued captures | No new execute; running task finishes or leaves recapturable work within policy |
| L02 | Hard kill with a full worker queue | Restart drains pending work without duplicate value |
| L03 | Signer or CEX outage across many matches | Bounded queue and concurrency; no unbounded tasks or database lock convoy |
| L04 | PostgreSQL high RTT plus external delay | Capture and apply lock windows remain short and independently measured |
| L05 | One poison or corrupt match | Quarantine or rotate the candidate; unrelated matches continue |
| L06 | Dependency recovery after prolonged outage | Backlog converges within rate and capacity limits |

## 7. Lock and transaction proof

During injected signer and CEX delay, sample PostgreSQL and assert:

- no settlement worker session is `idle in transaction` for the captured match;
- no settlement row or table lock blocks a concurrent operation on the unrelated fixture beyond the configured bound;
- connection-pool occupancy remains within the documented budget;
- readiness retains an independent connection path;
- statement and lock timeouts match configuration.

The evidence artifact contains redacted `pg_stat_activity` and `pg_locks` samples. Credentials and sensitive query values are never retained.

## 8. Idempotency proof

For every economic intent, collect:

- original World intent ID and canonical request hash;
- signer request ID, request hash, key ID and signature fingerprint;
- CEX idempotency or intent ID and durable receipt ID/hash;
- all attempt timestamps and transport outcomes;
- final campaign reconciliation cursor and state hash.

Acceptance requires exactly one durable value event and one exact final local binding. Repeated transport attempts are expected and are not duplicate value events.

## 9. Stale-capture proof

The harness mutates campaign state after `execute.begin` and before `apply.begin` using a legitimate concurrent operation. Apply must:

- lock all captured campaigns in deterministic order;
- detect a revision or state-hash mismatch;
- perform zero campaign, ACK and match updates;
- emit `settlement.apply.stale`;
- recapture and converge using durable remote receipts;
- preserve the unrelated legitimate campaign mutation.

## 10. Metrics and observations

Record distributions for:

- capture lock wait and transaction duration;
- external queue wait and execution duration by dependency and outcome;
- apply lock wait, transaction duration and stale rate;
- pending backlog count and oldest age;
- ambiguous outcomes and retries per intent;
- worker queue depth and saturation;
- unrelated-match latency during dependency faults.

This plan does not itself establish a public production SLO. It produces evidence needed to set and review one.

## 11. Automation shape

Recommended jobs:

1. `settlement-unit` — pure exactness and idempotency helpers.
2. `settlement-postgres` — capture/apply transactions and concurrency.
3. `settlement-transport-faults` — controllable signer/CEX proxy.
4. `settlement-process-faults` — kill and restart at phase events.
5. `settlement-capacity` — bounded outage and backlog behavior.

Each job emits machine-readable JSON and a human-readable summary. One failed, skipped or invalid required scenario fails the evidence family.

## 12. Evidence registration

Only a complete passing run may add evidence to `trusted_cex_settlement`. The record includes:

- exact commit and tree;
- exact binary or artifact SHA-256;
- workflow run URL;
- database, signer and CEX versions and environment;
- scenario IDs executed;
- result and limitations;
- reviewer, review date and review due date.

Even after this fault plan passes, trusted settlement remains blocked until production credential custody and every other named denominator requirement pass.

## 13. Exit criteria

- every C, S, E, B, A and L scenario is automated or replaced by an approved equivalent proof;
- no external request occurs under a business transaction in traces or PostgreSQL samples;
- exact-once value and stale-apply properties hold under every injected failure;
- current game CI, P0 boundary and fault workflows pass on one exact integration commit;
- old transaction-spanning code and temporary rollout switch are removed;
- residual limitations remain explicit.
