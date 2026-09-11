# `trnm-rts-protocol`

Status: **alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Defines and validates the player-order contract consumed by the deterministic RTS authority.

## Responsibilities

- Typed order kinds, subjects, targets, queues, formations, control groups, stance, and source.
- Frame-order stream validation and deterministic stream hashing.
- Rejection of structurally incomplete or regressing player input.

It does not apply orders, simulate battle state, authenticate sessions, persist replays, or mutate campaigns.

## Contract and invariants

- Protocol: `trnm_rts_order_protocol_v1`.
- Stream frames never regress.
- Every order carries a non-empty player and subject set.
- Targeted orders carry the required typed target.
- Queued and job-lifecycle operations carry stable queue identifiers.
- Unknown versions and invalid stance/rule identifiers fail closed.

Validation errors are side-effect free. Network adapters may translate transport status but may not weaken validation.

## Build and test

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-rts-protocol --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-rts-protocol --all-targets --locked -- -D warnings
```

Breaking fields or enum changes require a new protocol version, compatibility window, crossed-version tests, rollout order, and retirement criteria.

## Security

Bound collection sizes at the caller boundary, authenticate player-to-subject ownership in the authority, and reject replay/sequence reuse. Wire data is always untrusted.

## References

- [`trillionnium-rpg-rts-closed-loop-v1.md`](../../../docs/development/trillionnium-rpg-rts-closed-loop-v1.md)
- [`GAME_STATUS.md`](../../../GAME_STATUS.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)
