---
status: current-candidate
owner: Trillionnium World deterministic battle simulation
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-rts-sim technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

A Bevy-free fixed-tick simulation consuming `BattleSeedV1` and validated `RtsFrameOrder` values. It advances maps, movement, resources, jobs, structures, technologies, objectives, combat, AI adapters, snapshots, checkpoints, terminal results and chunked replay material.

## Responsibilities and non-responsibilities

Owns deterministic World battle behavior and unsigned game-domain outcome material. Admission, canonical order, completion signatures, finality, campaign mutation and wallet settlement are excluded.

## Public interfaces and data contracts

Current identities are `trnm_rts_sim_v16` and `trnm_rts_sim_checkpoint_v16`; tick rate is 10 Hz. Initial authority input is the complete seed, player input is the ordered validated command stream, and terminal output is `BattleResultV1`. Snapshots/checkpoints/replay chunks carry explicit versions and hashes.

## State model and invariants

Identical seed, ordered commands, rules/content revisions and component bytes produce byte-identical accept/reject outcomes, snapshots, replay and terminal result. Rejected commands preserve the complete canonical state hash. All collections, durations and counters are bounded; arithmetic uses explicit checked/saturating semantics where specified.

## Dependency direction

Depends on campaign contract types and RTS protocol, not Bevy, SQL, HTTP, environment or wall clock. Nondeterminism enters only through versioned seed material. Presentation and transport observe snapshots but cannot mutate internal state. Content lookup is exact-revision-bound.

## Concurrency and cancellation

One match has one logical deterministic writer. Parallel execution may occur across independent matches, never within an ordered transition unless a future version proves deterministic reduction. Cancellation between ticks leaves the last published checkpoint; cancellation during a private tick cannot expose partial state.

## Durability and recovery

The simulation itself is in memory. Owners persist exact seed, canonical command cursor, snapshots/checkpoints, replay chunks and terminal result. Material is hash-verified before restore. The runtime publishes only complete ticks; private speculative work is discarded on failure.

## Failure, retry and idempotency

Invalid orders return `SimError` without mutation. Missing/corrupt/cross-version checkpoints, altered replay chunks and unknown content revisions fail closed. Runtime recovery loads an exact verified checkpoint and replays the canonical suffix. It never guesses a missing command or silently changes a seed.

## Versioning and migration

Changes to tick ordering, arithmetic, RNG consumption, tie-breaks, pathfinding, content interpretation, encoding or terminal calculation require a simulation/rules version bump. Old readers and migration policy remain explicit. Shadow comparison is mandatory before an online component lock accepts new bytes.

## Resource budgets and performance

Fixed tick is 10 Hz. Map dimensions, entities, jobs, queues, orders, replay chunks and total match duration are bounded. Every tick phase has a declared iteration order and target complexity. Benchmarks record p50/p95/p99 tick time, peak allocations, snapshot size and replay throughput on named hardware.

## Observability and security

The kernel emits deterministic counters/events through caller-owned sinks; operational timestamps stay outside hashes. Record match/rules/content digests, tick, command cursor, state hash, rejection code and budget saturation. Security review covers crafted pathfinding load, overflow, replay corruption and divergent platform behavior.

## Verification traceability

Golden seeds/maps, rejected-command preservation, checkpoint/replay round trips, both faction/spawn assignments, pathfinding, objectives, AI boundedness, resource exhaustion and repeated deterministic runs are required. Cross-toolchain and Nakama shadow runs must report zero unexplained divergence.

## Known gaps and change checklist

Publish the exact tick-phase and tie-break specification; isolate kernel, pathfinding, economy/build, combat, AI and replay modules behind explicit interfaces; add Criterion baselines and multi-toolchain differential runs; and bind each invariant to property/fuzz tests. No performance claim is accepted without retained measurements.
