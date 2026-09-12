# `trnm-campaign-core`

Status: **alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Sole authority for persistent RPG campaign mutation, save compatibility, RPG→RTS seeds, result validation, and one-time settlement.

## Responsibilities

- Campaign aggregate, save slots, schema migration, progression, quests, inventory, injuries, and reputation.
- Construction and validation of battle seeds and battle results.
- Two-phase crash-safe idempotent battle settlement.
- Economic intent/outbox/receipt state that gates persistent progression.

## Non-goals

It does not render Bevy UI, simulate RTS ticks, own external wallet/ledger settlement, or implement online transport.

## Contracts and invariants

- `trnm_campaign_save_v1`.
- `trnm_battle_seed_v8`.
- `trnm_battle_result_v2`.
- `trnm_settlement_receipt_v1`.
- Only this aggregate mutates durable campaign progression.
- The phase transition is `Town -> BattlePending -> PostBattlePending -> Town`.
- A validated result is staged before progression is applied.
- An exact duplicate settlement returns zero new deltas; changed reuse of a settled battle ID is rejected.
- Atomic replacement and restart recovery preserve either the prior or fully advanced valid state.
- Connected value progression remains blocked until the required receipt class is satisfied.

## Failure semantics

I/O or integrity failure must not partially apply progression. Restart either completes a valid staged settlement or rejects it. Corrupt slots remain isolated from healthy slots.

## Build and test

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-campaign-core --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-campaign-core --all-targets --locked -- -D warnings
```

Save changes require old-fixture load, forward migration, interrupted-write, duplicate, and rollback tests. Battle-contract changes require client/server/simulation/replay compatibility review.

## Security

Treat saves and submitted results as hostile. Verify versions, hashes, battle identity, bounded deltas, entitlement provenance, and duplicate semantics before mutation.

## References

- [`trillionnium-rpg-rts-closed-loop-v1.md`](../../../docs/development/trillionnium-rpg-rts-closed-loop-v1.md)
- [`trnm-cex-economy-integration-v1.md`](../../../docs/development/trnm-cex-economy-integration-v1.md)
- [`GAME_STATUS.md`](../../../GAME_STATUS.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)

## Detailed design

- [`trnm-campaign-core detailed design`](../../../docs/modules/trnm-campaign-core-design.md)
