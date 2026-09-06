# trnm-rts-sim

Status: current candidate  
Owner: Trillionnium World deterministic battle simulation

## Purpose

This crate implements the Bevy-free deterministic RTS simulation used by campaign missions and skirmishes. It consumes validated `RtsFrameOrder` values, advances authored maps, units, resources, jobs, technologies, objectives, AI adapters, snapshots, checkpoints, results, and chunked replay material.

## Authority and non-goals

The simulation owns deterministic World battle behavior and World outcome material only. It does not admit participants, assign canonical online order, sign `MatchCompletedV1`, establish Chain finality, mutate campaign saves directly, or own wallet settlement.

## Public contracts

The current identities are `trnm_rts_sim_v16` and `trnm_rts_sim_checkpoint_v16`; the fixed tick rate is 10 ticks per second. `BattleSeedV1` is the complete initial authority input, `RtsFrameOrder` is the only player-command input, and `BattleResultV1` is the terminal game-domain output.

## State and invariants

Identical seed, ordered command stream, rules/content revisions, and component bytes must produce byte-identical accept/reject outcomes, snapshots, replay chunks, and terminal result. Rejected commands preserve the complete state hash, including resources, queues, cooldowns, guards, party membership, replay/event counters, and random cursors. Collections, map dimensions, orders, replay chunks, and simulation duration are bounded.

## Dependencies and boundaries

The crate depends on campaign contract types and the RTS protocol but not Bevy, SQL, HTTP, environment variables, or wall-clock time. All nondeterminism must enter through versioned seed material. Presentation and transport adapters may observe snapshots but may not mutate internal state.

## Failure and recovery

Invalid orders return `SimError` without state mutation. Checkpoints and replay chunks are hash verified before use. Missing, corrupt, crossed-version, or altered material fails closed. Restart/reconnect authority is implemented by the owning runtime using exact snapshots and command cursors.

## Testing and evidence

Required tests include golden maps and seeds, rejected-command preservation, checkpoint round trips, replay chunk/hash verification, queue and production controls, both faction/spawn assignments, pathfinding/occupancy, terminal outcomes, AI boundedness, resource exhaustion, and deterministic repeatability across supported toolchains.

## Compatibility and change control

Any change to tick ordering, arithmetic, RNG consumption, pathfinding tie-breaks, content interpretation, snapshot/replay encoding, or terminal calculation requires a rules/simulation version bump, updated vectors, migration/reader policy, and cross-implementation or shadow evidence before promotion.
