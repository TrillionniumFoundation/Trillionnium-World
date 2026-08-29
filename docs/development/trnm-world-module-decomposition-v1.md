---
status: current-plan
owner: trillionnium-world
work_item: WORLD-P1-001
last_reviewed: 2026-08-29
---

# Trillionnium World Correctness Module Decomposition v1

## 1. Objective

Reduce correctness blast radius without mixing semantic changes into structural
moves. Each extraction must preserve wire behavior, database behavior and exact
evidence boundaries before later simplification.

## 2. Server target

```text
trnm-game-server/src/
  lib.rs                       public composition only
  http/
    mod.rs
    player.rs
    admin.rs
    websocket.rs
    limits.rs
  identity/
    mod.rs
    sessions.rs
    authorization.rs
  authority/
    mod.rs
    actor.rs
    admission.rs
    command_lane.rs
    publication.rs
    terminal.rs
  persistence/
    mod.rs
    migrations.rs
    command_store.rs
    checkpoint_store.rs
    terminal_store.rs
    campaign_store.rs
  journal/
    mod.rs
    hot.rs
    cold.rs
    manifest.rs
    recovery.rs
  settlement/
    mod.rs
    capture.rs
    claim.rs
    remote.rs
    apply.rs
    operator.rs
    metrics.rs
  readiness/
    mod.rs
    probes.rs
    database.rs
  fleet/
    mod.rs
    leases.rs
    fencing.rs
    routing.rs
  operations/
    mod.rs
    moderation.rs
    seasons.rs
    replay.rs
```

## 3. Client target

```text
trnm-first-contact/src/
  campaign/
    load.rs
    save.rs
    migration.rs
    commands.rs
  ui/
  online/
    transport.rs
    protocol.rs
    journal.rs
    reconciliation.rs
  render/
  simulation_adapter/
```

## 4. Required module contract

Every module owns a `CONTRACT.md` or rustdoc section stating:

- owned state and excluded state;
- accepted inputs and returned errors;
- concurrency and cancellation model;
- lock acquisition order;
- durable versus speculative/public state;
- retry and idempotency identity;
- fail-open/fail-closed decision;
- resource budgets;
- unit/property/black-box evidence.

## 5. Global dependency rules

```text
http -> application/domain ports
identity -> no settlement/persistence implementation dependency
settlement -> economy protocol + persistence ports, not HTTP handlers
authority -> deterministic simulation + persistence ports
persistence -> SQL only, no network clients
journal -> filesystem durability only, no HTTP/database authority
operations -> application ports, no direct hidden state mutation
```

No circular module dependency is allowed. External service clients implement
narrow traits/ports and are injected at composition time.

## 6. Migration slices

1. Extract settlement first because it already has a distinct process and
   contract.
2. Extract migrations/procedure registry from `lib.rs` without changing SQL.
3. Extract HTTP route composition and body/authorization middleware.
4. Extract authority actor/command/publication state machines.
5. Split journal hot/cold/manifest/recovery.
6. Extract readiness and fleet fencing.
7. Split client online transport/journal/reconciliation.
8. Split Campaign command handlers from persistence/migration.

Each slice must be reviewable, compile-clean and behavior-preserving before the
next slice.

## 7. Acceptance

- no source template is transformed into compiled correctness logic;
- public `lib.rs` contains composition and re-exports, not state-machine bodies;
- every state transition is exercised through a narrow invariant-focused API;
- static text scanners are supplementary, not primary correctness evidence;
- lock-order and cancellation tests cover cross-module interactions;
- benchmark/query-plan baselines show no unacceptable hot-path regression.
