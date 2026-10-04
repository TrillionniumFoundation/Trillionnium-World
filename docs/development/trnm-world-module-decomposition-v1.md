---
status: current-candidate
owner: trillionnium-world-architecture
work_items:
  - WORLD-P1-001
  - WORLD-P1-009
last_reviewed: 2026-09-10
review_due: 2026-10-09
implementation_conformance: not-implied
---

# Trillionnium World Correctness-Oriented Module Decomposition v1

## Objective

Reduce review blast radius without changing authority ownership or silently
rewriting runtime semantics. A module boundary is accepted only when it owns a
coherent set of invariants, has an explicit concurrency/durability contract, and
is tested through its public interface.

## Current debt

The former semantic `trnm-game-server/build.rs` and `src/lib.rs.in` generation authority is retired and removed from the current candidate. Correctness code is now ordinary Git-tracked Rust source,
so hidden build-time rewriting is no longer part of the current candidate.

The remaining structural debt is different: several large crates still compose
ownership-labelled `lib_parts` with `include!`, leaving definitions in one broad
module namespace. This improves source provenance and review chunking but does
not yet provide true Rust module visibility, narrow traits, dependency inversion,
or independently testable package boundaries. Large compatibility files and a
wide application state also remain. No document or source-only gate may convert
this intermediate layout into a completed decomposition claim.

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

## Target campaign and simulation layout

```text
trnm-campaign-core/src/
  contracts/
  state/
  commands/
  economy/
  settlement/
  storage/
  migration/

trnm-rts-sim/src/
  state/
  intake/
  tick/
  movement/
  economy/
  combat/
  objectives/
  replay/
  checkpoint/
```

The target is real `mod`/trait/visibility structure. Merely moving an
`include!` target or adding another ownership manifest does not close a tranche.

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

1. Keep migration registry and checksums directly compiled and catalogue-bound.
2. Move quarantine/operator SQL access behind typed persistence functions.
3. Move capture and apply into separate modules sharing immutable fence types.
4. Move remote execution behind an async transport trait.
5. Expose a bounded orchestration loop that owns shutdown and in-flight tasks.
6. Replace the corresponding ownership `include!` parts with real modules only
   after exact behavior, SQL, fault and evidence comparison passes.

## Authority extraction order

1. Freeze compatibility endpoints and types.
2. Separate deterministic game adapter from admission/order/recovery concerns.
3. Publish and version the World transition contract.
4. Implement the Nakama shadow adapter in the owning repository.
5. Drain compatibility matches and disable new canonical admission.
6. Retire duplicated World-local authority modules after rollback evidence.

## Campaign and simulation extraction order

1. Freeze public types, serialization, hashes, error precedence and golden vectors.
2. Introduce private modules around one ownership section at a time.
3. Replace wildcard/shared-scope access with explicit imports and narrow exports.
4. Put mutation behind command/application interfaces and keep persistence in adapters.
5. Prove error-state byte/hash preservation and replay/checkpoint compatibility.
6. Delete each `include!` only after exact-head tests and independent review pass.

## Review constraints

A decomposition PR must not also:

- add public authority responsibilities;
- change settlement value semantics;
- change canonical hashes without a contract version;
- modify production activation flags;
- claim performance improvement without measurements;
- delete compatibility readers before migration inventory is complete.

Each tranche should stay bounded to one owner/invariant family. A broad mechanical
move that cannot be reviewed and reverted independently is not an accepted
module boundary.

## Acceptance

A module tranche closes only when:

1. real Rust modules/traits replace the corresponding ownership `include!` parts;
2. interfaces are narrower than the extracted implementation;
3. no circular dependency exists;
4. lock-order documentation matches source and SQL;
5. error paths are state-preserving;
6. existing golden/fault tests pass on the exact head and prospective merge;
7. new invariant-focused tests cover cancellation, duplicate, stale fence, and
   resource-exhaustion behavior;
8. format, all-target test and strict Clippy pass under the pinned toolchain;
9. exact evidence and independent review are attached;
10. the change does not alter authority, protocol, economic or release semantics
    without the corresponding version and migration process.
