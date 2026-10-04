---
status: current-candidate
owner: trillionnium-world-rts-simulation
module: trnm-rts-sim
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-rts-sim detailed design

## Scope and authority

`trnm-rts-sim` implements the Bevy-free deterministic RTS state machine used by campaign missions and skirmishes. It owns World battle rules, authored-map interpretation, units/resources/jobs/technologies/objectives, deterministic AI adapters, snapshots/checkpoints, terminal game-domain results, and unsigned replay material. It does not admit participants, assign canonical online order, sign completion, establish Chain finality, mutate campaign saves directly, or own wallet settlement.

## Public interfaces

The principal interface consumes a complete `BattleSeedV1`, accepts validated `RtsFrameOrder` commands, advances `MissionSimV1` at a fixed 10 ticks per second, exposes bounded deterministic views/snapshots, reads/writes hash-checked checkpoints and replay chunks, and produces `BattleResultV1`. Current identities are `trnm_rts_sim_v16` and `trnm_rts_sim_checkpoint_v16`. Public errors separate invalid command, invalid state, integrity, I/O/serialization, and campaign-contract rejection.

## State model and invariants

Identical seed, ordered accepted/rejected command stream, rules/content revisions, and component bytes must produce byte-identical decisions, state hashes, snapshots, replay chunks, and terminal results. A rejected command preserves the complete state, including positions, party/control membership, resources, jobs/queues, cooldowns, guards, AI/random cursors, objective progress, event/replay counters, and snapshot hash. Entity IDs are unique, arithmetic is bounded/saturating where specified, and terminal settlement material is derived exactly once.

## Dependency direction

The simulation depends on campaign contract types and `trnm-rts-protocol`, but not Bevy, SQL, HTTP, environment variables, credentials, filesystem configuration, or wall-clock time. All nondeterminism enters through versioned seed/rules/content material. Client/server/Nakama adapters may supply admitted commands and observe views, but cannot mutate internal state or choose hidden tie-breaks. Replay/storage adapters operate on exported versioned material rather than internal pointers.

## Execution, concurrency and cancellation

One simulation instance advances through a deterministic single-owner tick loop. A tick freezes admitted input, applies validation and ordering, advances movement/occupancy, logistics/jobs/production/research, abilities/combat, AI/objectives, and then publishes snapshot/replay/terminal material according to the versioned rules. Parallelism may be added only when its reduction/order semantics are proven byte-identical. Cancellation before a tick publication discards private work; a public cursor/hash is advanced only after complete deterministic success.

## Persistence and durability

The pure state machine is memory resident. Checkpoints and replay directories are versioned, chunked, size bounded, and hash verified before use; unsupported/corrupt/crossed-version material fails closed. Runtime owners persist the authoritative snapshot/cursor and accepted command sequence with exact rules/content/build identities. A checkpoint is not canonical online recovery evidence until Nakama or the owning compatibility runtime binds it to its admitted order, generation, archive, and durable publication protocol.

## Idempotency, retry and failure taxonomy

The simulation itself applies a command only when the caller supplies the next admitted order for the current state. Exact duplicate handling, sequence identity, and reconnect belong to the runtime authority; altered duplicates are never silently reinterpreted. Shape/authority/phase/cost/cooldown/target/occupancy/resource failures return a stable rejection and unchanged state. Integrity/hash/version/checkpoint failures reject loading or advancement. I/O errors from optional replay/checkpoint helpers do not partially replace a previously valid persisted artifact.

## Versioning and migration

Changes to tick phase order, arithmetic/rounding, RNG stream/consumption, pathfinding tie-breaks, occupancy resolution, AI decision order, content interpretation, snapshot/replay/checkpoint encoding, command acceptance, or terminal calculation require a new simulation/rules version. Updated positive/negative vectors, old-reader policy, migration/rollback notes, long-match replay fixtures, and cross-implementation or shadow evidence are mandatory. Build labels cannot substitute for rules/content identities.

## Resource and performance budgets

The fixed cadence is 10 ticks per second. Current source bounds replay input at 65,536 orders, uses 512-order replay chunks, and emits seek checkpoints on the declared checkpoint interval. Maps, units, structures, jobs, queues, paths, resource nodes, projectiles/effects, objectives, AI searches, simulation duration, and snapshot size must remain explicitly bounded. Benchmarks report per-tick p50/p95/p99, worst authored maps, pathfinding/congestion, replay/checkpoint cost, memory, and deterministic rerun variance.

## Observability and security

Simulation telemetry exposes rules/content/build/seed identity, tick, state hash, command acceptance/rejection code, bounded phase timing, entity/resource/job counts, replay/checkpoint hashes, and terminal result identity. It must not include credentials or trust client-provided completion/value claims. Untrusted seeds/maps/commands are schema/budget validated before allocation or iteration. Divergence, panic, overflow, hash mismatch, and nondeterministic output are quarantine events, not recoverable cosmetic warnings.

## Test and evidence traceability

Primary source is `trillionnium/crates/trnm-rts-sim/src/` and its ownership manifest. Normative neighbors include `docs/protocol/trnm-world-transition-v1.md`, `docs/protocol/trnm-rts-order-intake-v1.md`, `GAME_STATUS.md`, and `docs/development/trnm-world-testing-strategy-v2.md`. Required tests cover golden seeds/maps, repeatability, every rejected-command state hash, both factions/spawns, path/occupancy, logistics/construction/production/research, objectives/terminal results, AI bounds, checkpoint/replay corruption, long matches, resource exhaustion, fuzz/property input, and supported-toolchain agreement.

## Change checklist and open work

Before modifying the kernel: identify the exact phase/tie-break/RNG/encoding effect; bump the appropriate version; update vectors, migration/readers and replay fixtures; add rejection-state and boundary/property tests; benchmark worst cases; update adapters and component locks; and obtain exact-head plus shadow evidence. Open work includes publishing a normative tick-order table, converting `lib_parts` into true modules, explicit RNG stream documentation, cross-toolchain determinism, broader fuzzing, and Nakama shadow divergence closure.
