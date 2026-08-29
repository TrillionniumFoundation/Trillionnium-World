---
status: current-candidate
owner: trillionnium-world
contract: trnm_settlement_lifecycle_v2
applies_to_plan: trillionnium-world-development-2026-08-29-v4
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Settlement Lifecycle v2

## Scope

This contract governs movement of a game-owned economic intent from a terminal World match into CEX and back into the campaign projection. It does not grant World wallet custody or public-market authority.

## Identity layers

| Identity | Purpose | Stability |
| --- | --- | --- |
| `capture_id` | binds one local snapshot of terminal/campaign fences | changes after stale recapture |
| `job_id` | identifies one local worker row inside a capture | capture-scoped |
| `intent_id` + `intent_hash` | identifies immutable game economic intent | stable business identity |
| `remote_request_id` | signer/CEX ambiguity and idempotency identity | stable across capture generations |
| `receipt_id` + `receipt_hash` | immutable remote outcome | stable after remote commit |
| lease owner/generation/expiry | fences one worker attempt | changes on claim/takeover |

No retry may silently mint a new remote identity for the same intent.

## Durable state model

### Capture

```text
active -> applied -> active (next head recapture)
active -> finalized
active -> stale
active -> dead_letter
```

A capture binds:

- terminal publication identity and hash;
- match ID and capture generation;
- every campaign ID, revision and state hash;
- each campaign's exact compensation/ordinary head descriptor;
- creation and update timestamps.

### Job

```text
pending -> leased -> succeeded -> campaign_applied
             |          |
             |          +-> pending_apply
             +-> retryable -> leased
             +-> dead_letter
expired leased -> leased by next generation
```

`state = succeeded` means the remote receipt is durable. It does not mean campaign application completed.

### Quarantine

Poison work is recorded separately from ordinary retry:

```text
observed -> quarantined -> reviewed -> retry_authorized | permanently_closed
```

Quarantine records bind scope (`match`, `capture`, `job`), scope ID, phase, error class, bounded diagnostic, diagnostic hash, observation count, first/last observation, retry-not-before, operator decision and retention.

## Phase A — capture transaction

Preconditions:

- terminal match is complete, publication-acknowledged and settlement-pending;
- exact terminal cold/local witness is sealed;
- two expected campaigns exist and validate;
- no active/dead-letter capture already fences the match;
- the match is not under active capture quarantine.

Within one PostgreSQL transaction:

1. lock the terminal match and campaign rows in documented lock order;
2. load and hash terminal identity;
3. validate and hash campaign documents;
4. read exact compensation-first/ordinary head intents;
5. insert capture and one job per nonempty campaign head;
6. commit.

No signer/CEX request, DNS lookup, socket connect or wallet operation may occur before commit.

## Phase B — claim

Claim uses one database function that:

- selects a bounded eligible candidate window;
- excludes nonactive capture, applied job, exhausted attempts and active quarantine;
- respects compensation priority;
- uses per-account/campaign serialization key;
- acquires transaction-scoped advisory serialization lock;
- rejects a key with live lease or pending apply;
- increments lease generation and attempt counters;
- initializes stable authorization timing/nonce/request identity;
- returns one claimed row.

All subsequent worker mutations require:

```text
state = leased
lease_owner = expected owner
lease_generation = expected generation
lease_expires_at > clock_timestamp()
```

## Phase C — transaction-free remote execution

The worker performs no open database transaction while awaiting remote I/O.

### Signer sequence

1. `GET` exact durable signer receipt by `remote_request_id`.
2. Only exact authenticated `404` permits `POST /sign`.
3. Timeout, connection reset, `5xx`, malformed `2xx`, oversized body or undecodable response is **ambiguous/retryable**.
4. Returned receipt is accepted only after request ID, payload hash, issuer, key ID, signature and receipt hash revalidation.
5. Persist authorization under the same live lease fence.

### CEX sequence

1. Compute exact authorized intent hash.
2. `GET` exact CEX receipt by intent ID plus hash.
3. Only exact authenticated `404` permits intent submission.
4. On `409`/duplicate conflict, repeat exact lookup before classification.
5. Timeout, reset, `5xx`, malformed `2xx`, oversized body or undecodable response is ambiguous/retryable.
6. A `200` lookup must bind contract, intent ID/hash and pass `receipt.validate_for`.
7. Persist remote success under live lease fence.

### Response body budget

Remote diagnostic bodies are bounded before recording. Recommended limits:

- structured success body: 256 KiB;
- error diagnostic body: 16 KiB retained, with full-body hash when transport permits;
- headers: platform HTTP library limits plus explicit server budgets.

Secrets, tokens and signed private material are never copied into error evidence.

## Phase D — apply transaction

Preconditions:

- capture remains active;
- every required job is remote-succeeded and not yet campaign-applied;
- no stale/duplicate job exists for a campaign;
- terminal match remains ready.

Within one PostgreSQL transaction:

1. lock capture, match, campaigns and jobs in documented order;
2. recompute terminal identity hash;
3. recompute every campaign revision/hash fence;
4. revalidate exact head lane, intent ID and intent hash;
5. revalidate job/campaign/capture binding;
6. validate receipt against the captured intent;
7. apply receipt through an in-memory captured-receipt backend only;
8. checked-increment campaign revision;
9. validate resulting campaign;
10. CAS update campaign revision/hash/document;
11. mark `campaign_applied_at` for every job;
12. finalize terminal settlement only when all campaign heads are empty;
13. commit.

No remote I/O occurs in this transaction.

## Stale recapture

When remote success exists but campaign/terminal state changed:

- do not delete or mutate the old receipt;
- mark capture stale with bounded reason;
- preserve remote request/intent/receipt evidence;
- recapture only under an operator- or policy-approved path;
- reuse the same `remote_request_id` for the same intent;
- lookup must recover the existing remote receipt rather than resubmit value.

## Poison isolation

A single failure must not terminate the whole worker batch.

- capture errors are recorded per match and processing continues;
- claim/decode errors quarantine the exact leased job and continue;
- apply errors are recorded per capture and continue;
- identical repeated failure uses backoff;
- unrelated serialization keys continue concurrently;
- quarantine metrics and oldest age are operator-visible.

A panic or process kill remains fail-closed through lease expiry; however routine data errors must not rely on process death for isolation.

## Concurrency

- The worker may process multiple unrelated serialization keys concurrently up to a configured bound.
- Same-key work is serialized in PostgreSQL, not merely in one process.
- A remote-succeeded/pending-apply row blocks another same-key mutation.
- Worker concurrency, pool size, remote timeouts and queue sizes are independently bounded.

## Shutdown

On SIGINT or SIGTERM:

1. set `draining=true`;
2. stop new capture and claim;
3. allow in-flight remote/apply tasks to finish within `shutdown_grace`;
4. after grace, cancel local futures and exit nonzero or dedicated drained-timeout status;
5. do not clear leases manually; takeover occurs only after durable expiry;
6. emit final counts and exact worker identity without secrets.

Systemd stop timeout must exceed application grace plus a small process margin.

## Operator replay

Replay is permitted only for receipt-free dead-letter work and must bind:

- replay request ID;
- job/capture/intent/hash/remote request identity;
- operator ID, change ticket and reason;
- prior state/attempts/error;
- current policy revision;
- retention deadline.

The record is append-only. Replay allows exactly one additional remote attempt and cannot mutate already successful or applied work.

## Required evidence matrix

| Fault | Required observation |
| --- | --- |
| capture transaction rollback | zero externally started requests |
| signer commit + response loss | one sign, lookup recovery |
| signer malformed 2xx | retry/lookup, no second durable sign |
| CEX commit + response loss | one value effect, lookup recovery |
| CEX 409 race | exact lookup before terminal classification |
| worker SIGTERM each phase | prompt drain/exit, recoverable durable state |
| worker SIGKILL each phase | lease expiry/takeover, no double apply |
| lease expires during remote call | stale worker cannot persist result |
| campaign revision/hash drift | zero campaign writes, stale capture |
| duplicate campaign job | database rejection |
| poison match/job/capture | quarantine plus unrelated progress |
| database failover/PITR | old primary/process fenced, evidence retained |

## Promotion boundary

This lifecycle is release-eligible only after exact-head Rust/PostgreSQL checks, deployed signer/CEX/process/database fault evidence, backup/PITR/retention approval, exact artifact/component lock and independent reviewer signoff. Source implementation alone grants no trusted-settlement, custody, public-online or public-market credit.