---
status: current-candidate
owner: trillionnium-world-server
last_reviewed: 2026-09-08
review_due: 2026-10-08
authority_profile: world_legacy_local_alpha
---

# Trillionnium World WebSocket stream v1

## Scope

The compatibility route `GET /v1/online/matches/:match_id/stream` exposes bounded state delivery for the World-local laboratory server. The stream does not create canonical online authority. A client treats every message as server material bound to the authenticated session, actor generation, protocol/build pair and exact match identity.

## Connection request

`OnlineStreamConnectRequest` binds:

- `protocol_version` and `build_id`;
- `player_id` and `account_id`;
- `next_receipt_sequence` for the next command receipt expected by the client;
- `last_snapshot_hash` for continuity checking.

The session header and route match ID are authenticated independently. A claimed player/account in the JSON body cannot override the authenticated identity.

## Server messages

The serde tag is `message_type`. The stable envelope variants are represented by `schemas/trnm-world-online-stream-v1.schema.json`.

### `full_snapshot`

Carries `actor_generation`, `state_sequence`, `next_receipt_sequence`, `view` and the complete snapshot. The client validates protocol/build, match/member identity, generation, sequence, tick and decoded state hash before replacing visible attached state.

### `snapshot_delta`

Carries `actor_generation`, `state_sequence`, `base_state_sequence`, `view` and a top-level delta. The delta binds base/current snapshot hashes and ticks, changed fields and removed fields. The client applies it only when its current generation, state sequence, snapshot hash and tick exactly match the declared base.

### `resync_required`

Carries `actor_generation` and a bounded reason. The client discards any unverified delta suffix and requests an exact full snapshot/reconnect response. It must not guess missing commands or continue local authority simulation while attached.

## Ordering and continuity

WebSocket arrival order is not an authority substitute. Every accepted message satisfies:

```text
same authenticated match/member
same active actor generation
next state sequence or explicitly accepted full resync
exact base state sequence for a delta
exact base snapshot hash and base tick
decoded next snapshot hash equals the advertised hash
```

A generation change, gap, duplicate with altered bytes, hash mismatch, unknown variant or unsupported protocol/build fails closed and requires resync or reauthentication according to the stable error class.

## Backpressure and limits

Frame size, buffered outgoing messages, receipt suffixes, changed/removed field counts, stream connections per identity and reconnect work are bounded by runtime configuration. The server closes or resyncs a slow consumer rather than allowing unbounded memory. Diagnostics are bounded and do not reflect complete private payloads or credentials.

## Disconnect and reconnect

Disconnect does not transfer authority to the client. A journaled client command may be retried with the exact same identity. Reconnect supplies a full snapshot plus a bounded, continuity-checked receipt suffix when available. If the suffix is truncated or any cursor cannot be proven, `full_snapshot_required=true` and the client resumes from the exact snapshot.

## Security

The server validates authenticated audience, player/account/match membership, protocol/build and resource ceilings before subscribing. Tokens are not placed in query strings or logs. Cross-site/browser origin policy, TLS termination and public-edge controls are deployment concerns and remain external evidence gates.

## Compatibility and evidence

Changing variant tags, required fields, cursor meaning, delta semantics or limits requires a new stream protocol version and fixtures. Schema/source tests prove message-shape reviewability only; exact client/server runs, reconnect faults, load/backpressure and Nakama shadow evidence remain independent.
