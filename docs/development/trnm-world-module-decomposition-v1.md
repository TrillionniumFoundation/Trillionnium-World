---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-001
  - WORLD-P1-009
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Correctness-Oriented Module Decomposition v1

## Objective

Reduce review blast radius without changing authority ownership or silently
rewriting runtime semantics. A module boundary is accepted only when it owns a
coherent set of invariants, has an explicit concurrency/durability contract, and
is tested through its public interface.

## Current debt

`trnm-game-server` now compiles ordinary reviewed source fragments: the semantic build
script and `.rs.in` authority have been retired. The remaining debt is the large
catch-all `src/lib.rs`, whose authority, persistence, readiness, fleet, HTTP, and
actor-runtime invariants still have an unnecessarily broad review blast radius.
Direct source is a prerequisite for decomposition, not evidence that decomposition
is complete, and it grants no release credit.

## Target server layout

```text
trnm-game-server/src/
  application/
    command_service.rs
    query_service.rs
  http/
    public.rs
    player.rs
    moderator.rs
    readiness.rs
  identity/
    player_session.rs
    service_identity.rs
    authorization.rs
  authority/
    actor.rs
    admission.rs
    command_lane.rs
    publication.rs
    recovery.rs
  persistence/
    migrations.rs
    command_store.rs
    checkpoint_store.rs
    terminal_store.rs
    campaign_store.rs
  journal/
    hot.rs
    cold.rs
    manifest.rs
    recovery.rs
  settlement/
    capture.rs
    claim.rs
    execute.rs
    apply.rs
    quarantine.rs
    operator.rs
  readiness/
    database.rs
    authority.rs
    settlement.rs
  fleet/
    lease.rs
    fencing.rs
    drain.rs
  operations/
    replay.rs
    moderation.rs
    season.rs
```

## Target client layout

```text
trnm-first-contact/src/
  campaign/
  ui/
  online/
    transport.rs
    command_journal.rs
    reconciliation.rs
  render/
  simulation_adapter/
  settings/
```

## Mandatory module contract

Every correctness-critical module documents:

- owned state and forbidden state;
- caller/callee authority;
- concurrency and cancellation model;
- global lock-order position;
- durable, private, and public boundaries;
- idempotency identity and retry semantics;
- fail-open/fail-closed behavior;
- resource budgets;
- metrics and alerts;
- unit, database, fault, and integration evidence.

## Dependency direction

```text
protocol/domain
    ^
application
    ^
adapters (HTTP, PostgreSQL, CEX, signer)
    ^
runtime/bootstrap
```

Domain modules do not import HTTP, database, filesystem, systemd, environment,
or wall-clock adapters. Adapters do not bypass application invariants. Runtime
bootstrap wires interfaces but does not own business rules.

## Settlement extraction order

1. Keep the direct migration registry and worker wrapper free of source generation.
2. Move quarantine/operator SQL access behind typed persistence functions.
3. Move capture and apply into separate modules sharing immutable fence types.
4. Move remote execution behind an async transport trait.
5. Expose a bounded orchestration loop that owns shutdown and in-flight tasks.
6. Preserve exact capture/lease/apply and terminal-publication behavior through
   invariant tests on every extraction tranche.

## Authority extraction order

1. Freeze compatibility endpoints and types.
2. Separate deterministic game adapter from admission/order/recovery concerns.
3. Publish the World transition contract.
4. Implement Nakama shadow adapter in the owning repository.
5. Drain compatibility matches and disable new canonical admission.
6. Retire duplicated World-local authority modules after rollback evidence.

## Review constraints

A decomposition PR must not also:

- add public authority responsibilities;
- change settlement value semantics;
- change canonical hashes without a contract version;
- modify production activation flags;
- claim performance improvement without measurements;
- delete compatibility readers before migration inventory is complete.

## Acceptance

A module tranche closes only when:

1. directly compiled source replaces any corresponding semantic template;
2. interfaces are narrower than the extracted implementation;
3. no circular dependency exists;
4. lock-order documentation matches source and SQL;
5. error paths are state-preserving;
6. existing golden/fault tests pass on the exact head;
7. new invariant-focused tests cover cancellation, duplicate, stale fence, and
   resource-exhaustion behavior;
8. exact evidence and independent review are attached.
