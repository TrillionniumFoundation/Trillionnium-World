---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-002
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World WebSocket Stream Contract v1

## Boundary

The current World stream is compatibility-enclave state delivery. It does not
own target canonical admission, global command order, archive root, or
`MatchCompletedV1`; those belong to Nakama under ADR-0001.

## Session and upgrade

- Upgrade requires a valid player session in the designated header.
- Query-string credentials are forbidden.
- Origin/public-edge policy is deployment-specific and fails closed when absent.
- One session has bounded concurrent connections and subscriptions.
- Authentication is revalidated on reconnect; a socket is not a durable credential.

## Message classes

```text
hello
full_snapshot
delta
command_receipt
resync_required
terminal
error
heartbeat
```

Every message binds:

- protocol version;
- match ID;
- actor generation/instance epoch where applicable;
- monotonically increasing stream sequence;
- authoritative tick and match revision;
- base sequence for deltas;
- canonical decoded state hash;
- bounded payload.

## Full snapshot

A full snapshot is accepted only when:

- member/session authorization matches the match;
- generation and ownership fence are current;
- snapshot sequence is not older than the accepted cursor;
- decoded `MissionSimV1` hash equals the declared hash;
- tick, revision, phase, result, and member cursor constraints validate together.

## Delta

A delta must bind the exact previously accepted base sequence/hash. Missing,
out-of-order, duplicate-with-different-bytes, or hash-mismatched deltas cause a
`resync_required`; the client does not guess or apply a partial state.

## Command receipt

Receipts carry compatibility command ID, member input sequence, global
compatibility sequence, durable revision, authoritative tick/hash, and result
classification. A receipt is not Nakama canonical completion or wallet
settlement evidence.

## Reconnect

Reconnect returns:

- exact current full snapshot;
- bounded command-receipt gap after the client cursor;
- explicit earliest retained cursor;
- `truncated=true` and mandatory resync when continuity cannot be proven.

Pagination, truncation, duplicates, and terminal actor shutdown are deterministic
and documented by stable error codes.

## Resource and backpressure rules

| Resource | Ceiling/policy |
| --- | --- |
| frame bytes | route-specific, maximum 2 MiB |
| decoded nesting | schema bounded |
| outbound queue | bounded; slow consumers disconnect/resync |
| heartbeat interval | configured and published |
| idle timeout | configured and published |
| command-gap page | bounded count and bytes |
| decompression | disabled unless an explicit ratio/size budget exists |

Unbounded channels or silent frame drops are forbidden.

## Errors

Machine errors distinguish:

```text
unauthenticated
unauthorized_match
unsupported_protocol
stale_generation
sequence_gap
base_hash_mismatch
snapshot_hash_mismatch
resource_budget_exceeded
resync_required
server_draining
internal_unavailable
```

Only errors explicitly marked retryable may reconnect automatically. Retry uses
bounded exponential backoff and jitter.

## Target migration

During shadow/cutover:

- World compatibility stream remains clearly labelled noncanonical;
- Nakama stream owns canonical participant/order/recovery cursors;
- clients never merge cursors from both authorities;
- active compatibility matches drain before canonical admission changes;
- rollback selects one authority profile, never dual publication.

## Acceptance

- JSON Schema/fixture coverage for every message class;
- full/delta/hash/reconnect negative fixtures;
- bounded queue and slow-consumer tests;
- generation/sequence/duplicate races;
- compatibility retirement date and usage inventory;
- Nakama migration tests bind exact component revisions.
