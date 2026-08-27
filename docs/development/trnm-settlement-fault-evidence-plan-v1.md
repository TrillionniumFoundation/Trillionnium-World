# Settlement Fault Evidence Plan v1

Status: **current P0 test design**  
Owners: World economy, persistence and reliability maintainers  
Decision: ADR-0002  
Gate affected: `trusted_cex_settlement`

## 1. Objective

Prove that the capture/execute/apply settlement path preserves exact-once economic meaning when PostgreSQL, the signer, CEX, the process or the network fails at every durable boundary. Passing unit tests or source scans alone is insufficient.

## 2. Test environment contract

The harness provisions:

- a dedicated PostgreSQL database with exact migration ledger;
- one game-server settlement worker with configurable queue/concurrency/timeouts;
- a controllable signer endpoint;
- a controllable CEX endpoint with durable idempotency storage;
- a transport proxy able to delay, drop, duplicate or sever responses after the upstream commit;
- an inspector connection that records locks, transactions, campaign state, terminal markers and settlement state;
- exact logs/metrics and artifact collection bound to the tested commit.

Every run records PostgreSQL version/system identifier/timeline, signer/CEX build IDs, World commit/tree/binary hash, toolchain, configuration, fault schedule, timestamps, result and limitations.

## 3. Deterministic fixture

Each test creates:

- one exact complete match with acknowledged and cold-sealed terminal publication evidence;
- two member campaigns with known revision/state hash;
- at least one value-releasing economic intent with stable intent ID and authoritative match/result/participants metadata;
- expected CEX receipt and final campaign/settlement state;
- an unrelated pending match used to detect lock/pool starvation.

Fixture creation is idempotent and validates its own hashes before fault injection.

## 4. Phase instrumentation

The implementation exposes test-only or structured production events:

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

Events carry match/campaign/intent correlation and no secrets. The fault controller may pause or kill at each event.

## 5. Core assertions

For every scenario:

- no external request timestamp precedes `capture.committed`;
- no PostgreSQL business transaction remains open during execute;
- match/campaign row locks are absent during injected external delay;
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
| C01 | kill before match lock | no capture, no remote request |
| C02 | kill after match lock before campaign read | transaction rolls back, no remote request |
| C03 | kill after campaign read before commit | transaction rolls back, no remote request |
| C04 | PostgreSQL disconnect before capture commit | no execute, match remains pending |
| C05 | terminal marker changes before capture validation | capture rejects fail-closed |
| C06 | campaign state hash differs from serialized value | capture quarantines/rejects corruption |
| C07 | two workers capture same match | one wins lock; other skips or later captures safely |
| C08 | first match locked by another session | worker uses SKIP LOCKED and services unrelated match |

### Execute phase — signer

| ID | Fault | Expected result |
| --- | --- | --- |
| S01 | signer unavailable before request | pending retry, no DB lock |
| S02 | signer delays beyond timeout without commit | pending retry with same request ID |
| S03 | signer commits then response is dropped | retry returns exact existing signature/receipt |
| S04 | duplicate signer response | accepted only when request hash/key ID/signature binding is exact |
| S05 | signer returns different request ID/hash | hard fail/quarantine; no CEX request |
| S06 | signer key rotates between retries | old valid request resolves by registry policy or typed conflict; never silently reissued under a new identity |
| S07 | worker cancelled during signer delay | no open DB transaction; work recapturable |

### Execute phase — CEX

| ID | Fault | Expected result |
| --- | --- | --- |
| E01 | CEX unavailable before request | pending retry, signed entitlement remains reusable under policy |
| E02 | CEX rejects invalid authoritative metadata | hard fail/quarantine, no local value applied |
| E03 | CEX commits then response is dropped | retry exact intent returns existing receipt |
| E04 | CEX response delayed beyond worker timeout after commit | ambiguous outcome; exact retry/query converges |
| E05 | duplicate/out-of-order response | exact intent/request binding required |
| E06 | CEX returns receipt for another account/intent | hard fail/quarantine |
| E07 | signer succeeds but CEX remains unavailable across restart | durable pending work and stable identities survive |
| E08 | two workers execute same capture | CEX idempotency permits one value event; apply remains exact |

### Between execute and apply

| ID | Fault | Expected result |
| --- | --- | --- |
| B01 | process kill after remote success | fresh capture reconciles existing receipt |
| B02 | campaign revision changes | apply returns stale with zero writes |
| B03 | campaign JSON changes without stored hash update | corruption detected; no apply |
| B04 | terminal ACK/settlement tuple changes | apply rejects exactness loss |
| B05 | match is quarantined | no apply or further remote value |
| B06 | one campaign changes, one does not | validate complete set before writing either campaign |

### Apply phase

| ID | Fault | Expected result |
| --- | --- | --- |
| A01 | kill before locks | no local write; fresh capture converges |
| A02 | kill after locks before validation | rollback |
| A03 | kill after validation before first write | rollback |
| A04 | kill after campaign writes before marker update | single transaction rolls back all writes |
| A05 | kill after marker update before match update | single transaction rolls back all writes |
| A06 | PostgreSQL disconnect during commit | outcome treated as ambiguous; reload exact durable state before retry |
| A07 | transaction serialization/deadlock error | bounded retry from fresh state |
| A08 | two exact applies race | one commit; second observes settled/stale and writes nothing |
| A09 | first member complete, second still pending | persist validated reconciled state; match/ACK stay pending |
| A10 | every member complete | ACK and match advance to settled atomically |

### Lifecycle and capacity

| ID | Fault | Expected result |
| --- | --- | --- |
| L01 | graceful shutdown with queued captures | no new execute; running task finishes or leaves recapturable work within policy |
| L02 | hard kill with full worker queue | restart drains pending work without duplicate value |
| L03 | signer/CEX outage under many matches | bounded queue/concurrency; no unbounded tasks or DB lock convoy |
| L04 | PostgreSQL high RTT plus external delay | capture/apply lock windows remain short and independently measured |
| L05 | one poison/corrupt match | quarantine/rotate candidate; unrelated matches continue |
| L06 | dependency recovery after prolonged outage | backlog converges under rate and capacity limits |

## 7. Lock and transaction proof

During an injected signer/CEX delay, query PostgreSQL repeatedly and assert:

- no worker session is `idle in transaction` for the captured match;
- no row/table lock attributable to settlement blocks a concurrent update/read of the unrelated fixture beyond the configured bound;
- connection-pool occupancy remains within the documented budget;
- readiness retains an independent connection path;
- statement and lock timeouts match configuration.

The evidence artifact includes sampled `pg_stat_activity` and `pg_locks` records with credentials and sensitive query values redacted.

## 8. Idempotency proof

For each economic intent, collect:

- original World intent ID and canonical request hash;
- signer request ID, request hash, key ID and signature fingerprint;
- CEX idempotency key/intent ID and durable receipt ID/hash;
- all attempt timestamps and transport outcomes;
- final campaign reconciliation cursor and state hash.

Acceptance requires exactly one durable value event and one exact final local binding. Repeated transport attempts are expected and are not duplicate value events.

## 9. Stale-capture proof

The harness mutates campaign state after `execute.begin` and before `apply.begin` using a legitimate concurrent operation. Apply must:

- lock all captured campaigns in deterministic order;
- detect at least one revision or state-hash mismatch;
- perform zero campaign, ACK and match updates;
- emit `settlement.apply.stale`;
- recapture and converge using durable remote receipts;
- preserve the unrelated legitimate campaign mutation.

## 10. Metrics and SLO observations

Record distributions for:

- capture lock wait and transaction duration;
- external queue wait and execution duration by dependency/outcome;
- apply lock wait, transaction duration and stale rate;
- pending backlog count/oldest age;
- ambiguous outcomes and retries per intent;
- worker queue depth/saturation;
- unrelated-match latency during dependency faults.

This plan does not itself set public production SLOs. It produces evidence needed to set and review them.

## 11. Automation shape

Recommended jobs:

1. `settlement-unit` — pure exactness/idempotency helpers.
2. `settlement-postgres` — capture/apply transactions and concurrency.
3. `settlement-transport-faults` — controllable signer/CEX proxy.
4. `settlement-process-faults` — kill/restart at phase events.
5. `settlement-capacity` — bounded outage/backlog behavior.

Each job emits machine-readable JSON plus human-readable summary. A single failed/invalid scenario fails the evidence family.

## 12. Evidence registration

Only a complete passing run may add evidence to `trusted_cex_settlement`. The evidence record must include:

- exact commit and tree;
- exact binary/artifact SHA-256;
- workflow run URL;
- database/signer/CEX versions and environment;
- scenario IDs executed;
- result and limitations;
- reviewer and review date/due date.

Even after this plan passes, the gate remains blocked until production credential custody and any other named denominator requirements pass.

## 13. Exit criteria

- every C/S/E/B/A/L scenario is automated or explicitly rejected with an approved equivalent proof;
- no external request occurs under a business transaction in traces or PostgreSQL samples;
- exact-once value and stale-apply properties hold under all injected failures;
- current game CI, P0 boundaries and fault workflows pass on one exact integration commit;
- old transaction-spanning code and temporary rollout switch are removed;
- residual limitations remain explicit.
