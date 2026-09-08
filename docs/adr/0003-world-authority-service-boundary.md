---
status: accepted-candidate
owner: trillionnium-world-architecture
decision_date: 2026-09-08
review_due: 2026-10-08
release_effect: none
---

# ADR-0003: World authority service is deterministic-domain authority, not canonical online authority

## Context

Trillionnium World contains a native client, deterministic campaign/RTS logic and a World-local compatibility server. A separate `world-authority` extraction has also been explored to move World-owned domain, command, projection, map-provider and API code out of CEX. Without a precise boundary, that extraction could accidentally create a second canonical online authority beside Nakama or reassign wallet authority from CEX.

## Decision

A World authority service may own **deterministic game-domain state transitions and projections** under an exact ruleset/content/component revision. It may expose a versioned service API to Nakama and bounded laboratory/migration callers.

It must not own or infer:

- public participant admission or session identity;
- canonical cross-player/global command order;
- durable online idempotency authority;
- reconnect/archive completeness;
- canonical match completion or `MatchCompletedV1` signing;
- Chain inclusion/finality;
- wallet/ledger state, custody or irreversible value success;
- cross-repository release promotion.

Nakama remains the sole target authority for online admission, canonical order, idempotency, restart recovery, archive roots and completion signing. CEX remains authoritative for wallet/ledger settlement and custody. Chain owns finality. Integration owns exact component locks and multi-system evidence.

## Allowed World service flow

```text
Nakama admitted command + canonical sequence + exact component lock
  -> versioned World transition request
  -> World validates deterministic domain input
  -> World returns deterministic accept/reject, state/output hashes and unsigned material
  -> Nakama persists canonical event/root and signs completion when appropriate
```

For offline play, the client may invoke the same deterministic domain APIs locally, but local results remain offline authority only.

## Command terminology

A World `command` module decides deterministic game-domain validity and transition effects. It does not allocate the canonical online sequence, authenticate a player, prove controller ownership or resolve duplicate network submissions. API names and documentation must use `domain command` or `transition request` when ambiguity is possible.

## Projection terminology

A World projection is derived from World-owned deterministic state. A cache in CEX, Nakama or a client is not a new system of record. Projection version, source state hash and component revision travel with the projection. Rebuilding a projection cannot mutate canonical state.

## Persistence

World may persist deterministic domain state needed to execute transitions, but online ownership remains fenced by the Nakama-issued match/generation/component identity. A World database row cannot by itself prove canonical admission, order or completion. Split-brain protection must reject stale World workers and old database primaries.

## CEX interaction

CEX may authenticate settlement-specific ingress, validate its own ledger contracts and return verified receipts. It must not mutate World progression locally after cutover. World settlement uses capture -> transaction-free remote execution -> fenced apply. A CEX response is not accepted solely because it is HTTP success.

## Compatibility enclave

`trnm-game-server` remains `world_legacy_local_alpha` until cutover. New canonical admission is forbidden. The enclave may support deterministic parity, migration, drain, rollback and fault evidence. Retirement requires an active-match inventory, Nakama shadow convergence, admission cutover, rollback rehearsal and endpoint disablement.

## Consequences

- A seven-crate World extraction is acceptable only when each crate documents this authority split and its APIs enforce it.
- `trnm-world-api` cannot become a public player authority surface without a new ADR approved by World, Nakama and Integration owners.
- `trnm-world-server` must receive an externally assigned canonical identity/sequence for online requests; it cannot manufacture them.
- CEX-owned source formerly containing World behavior must be retired or reduced to a versioned client/projection consumer.
- Tests must include negative checks that World cannot sign completion, load Nakama keys, claim finality or mutate wallet state.

## Alternatives rejected

1. **World as complete public MMO authority:** rejected because it duplicates Nakama admission/order/recovery and creates split authority.
2. **CEX as World state owner:** rejected because custody/ledger authority and game-domain mutation have different trust, scaling and review boundaries.
3. **Client-authoritative results:** rejected because modified clients could forge progression, completion and value effects.
4. **Shared mutable authority between World and Nakama:** rejected because no deterministic fencing or evidence model can make two simultaneous canonical writers safe by convention alone.

## Migration acceptance

A cutover tranche is accepted only when exact World/Nakama/CEX/Integration revisions are locked, shadow comparison reports zero unexplained divergence, new admission is Nakama-only, old matches are drained or transferred under a reviewed protocol, rollback/disablement is rehearsed, and independent reviewers confirm no authority or key-surface regression.
