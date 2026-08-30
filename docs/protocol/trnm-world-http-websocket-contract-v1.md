---
status: source-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-002
applies_to_profile: world_legacy_local_alpha
last_reviewed: 2026-08-31
review_due: 2026-09-14
---

# Trillionnium World HTTP and WebSocket Contract v1

## Authority ceiling

This contract documents the private `world_legacy_local_alpha` compatibility
enclave. It does not make World the canonical public online authority. Nakama
owns target online admission, total order, idempotency, restart recovery,
archive roots, and `MatchCompletedV1` signing. Public online and the public
player market remain disabled.

## Artifacts

- `docs/protocol/openapi/trnm-world-legacy-local-alpha-v1.openapi.json`
- `docs/protocol/openapi/path-templates.json`
- `docs/protocol/schemas/trnm-world-match-stream-connect-v1.schema.json`
- `docs/protocol/schemas/trnm-world-match-stream-v1.schema.json`
- `docs/protocol/trnm-world-compatibility-matrix-v1.md`

The OpenAPI document lists every router path and binds each path to its
implementation handler. Shared path templates publish request, success, client
error, and server error shapes without duplicating route semantics.

## Authentication and identity

Player operations use `x-trnm-player-session`. Authority and operator surfaces
use `x-trnm-game-authority` where required by implementation policy. A header
validates caller identity only; it cannot grant Nakama signing authority, Chain
finality, CEX custody, or release approval.

Path identities are UUIDs. Request bodies are bounded by the server body limit.
Protocol, build, player, account, expected revision, and idempotency identities
remain implementation-owned fields and are validated before mutation.

## HTTP errors and retry semantics

The stable envelope contains:

```json
{
  "error": "bounded diagnostic",
  "retryable": false,
  "retry_after_seconds": 0,
  "current_revision": 0
}
```

`error` and `retryable` are mandatory. Retryable failures may include a bounded
retry delay. Revision conflicts may include the current revision. Clients must
not retry a non-retryable result by changing request identity, and must preserve
the original idempotency key when retrying a retryable command.

Unknown enum values fail request deserialization with a stable 4xx response.
Unknown fields are admitted only where the implementation's serde model allows
them; they never widen authority or compatibility. Strict published envelopes,
WebSocket messages, and the stream connect contract reject unknown fields.

## WebSocket stream

The stream endpoint is
`GET /v1/online/matches/{match_id}/stream` with subprotocol
`trnm-online-stream-v1`.

The connect query is defined by
`trnm-world-match-stream-connect-v1.schema.json`. The server emits only:

- `full_snapshot`;
- `snapshot_delta`;
- `resync_required`.

The stream is state-only. Client text and binary data frames are rejected with
close code 1008. The server pings every 15 seconds, reauthenticates every
60 seconds, and bounds sends to 2 seconds. Hashes are lowercase SHA-256. A delta
must bind its exact base hash, next hash, base tick, and authoritative tick.
Unknown message types require rejection and resynchronization.

## Compatibility and retirement

Compatibility is exact and matrix-driven. Missing identities, wildcards,
unknown capabilities, and crossed build/protocol pairs fail closed. This
compatibility surface may retire only after exact component locking,
Nakama-only admission, World-local drain or proven takeover, rollback
rehearsal, authority disablement, and an independently approved retirement
record. The earliest declared review date is 2026-09-30; it is not an automatic
retirement authorization.

## Evidence and release effect

The source checker proves route/schema/document consistency. Exact-head CI,
database tests, deployed cutover, custody, public-edge, human, and release
evidence remain separate denominators. Passing this source contract grants no
public-online, player-market, custody, or commercial-release credit.
