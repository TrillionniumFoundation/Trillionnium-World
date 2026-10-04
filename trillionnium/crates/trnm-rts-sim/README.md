# `trnm-rts-sim`

Status: **alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Provides the Bevy-free deterministic RTS authority that consumes validated frame orders and produces authoritative results, checkpoints, and replay data.

## Responsibilities

- Fixed-tick pathing, occupancy, combat, resources, structures, jobs, technology, AI, objectives, fog, and terminal outcomes.
- Deterministic checkpoints and replay verification.
- Conversion of a valid battle seed into one bounded battle result.

It does not render, collect input, persist campaigns, settle external value, or accept unvalidated network commands.

## Contracts and invariants

- `trnm_rts_sim_v16`.
- `trnm_rts_sim_checkpoint_v16`.
- `trnm_battle_replay_v2`.
- The same versioned seed and order stream produce the same state/result hash.
- Tick advancement is explicit and bounded.
- Player and AI jobs share the same side-tagged authority model.
- Hidden entities cannot be targeted through leaked identifiers.
- Terminal results bind the seed, elapsed ticks, unit outcomes, deltas, and final snapshot hash.
- Replay/checkpoint load verifies hashes before publication.

Invalid orders, corrupt checkpoints, hash mismatch, impossible objectives, or out-of-contract state fail closed without a settlement-eligible result.

## Build and test

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-rts-sim --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-rts-sim --all-targets --locked -- -D warnings
```

Any change to hashes, order effects, content interpretation, or replay encoding requires a contract bump or tested migration plus golden deterministic fixtures.

## Security

Review crafted orders, oversized replay denial, hidden-state leakage, integer overflow, non-deterministic iteration, and content divergence. Limits must be explicit and adversarially tested.

## References

- [`trillionnium-rpg-rts-closed-loop-v1.md`](../../../docs/development/trillionnium-rpg-rts-closed-loop-v1.md)
- [`GAME_STATUS.md`](../../../GAME_STATUS.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)

## Detailed design

- [`trnm-rts-sim detailed design`](../../../docs/modules/trnm-rts-sim-design.md)
