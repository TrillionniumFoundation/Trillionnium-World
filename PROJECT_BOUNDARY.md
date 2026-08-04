# Trillionnium World Boundary

- Project ID: `trillionnium-world`
- Canonical root: `/home/alex/projects/trillionnium-world`
- Lane: `game-product`
- Remote status: blocked until World and Chain have distinct remotes

## Owns

Gameplay, campaigns, game-server behavior, RTS simulation, player-facing
economy behavior, and production of match/replay evidence.

## Does not own

Chain consensus/runtime/state, Hepta control-plane services, Nakama runtime
infrastructure, or cross-repository E2E orchestration. New code must not enter
the excluded legacy `trillionnium/crates/platform` tree.

The current sibling Chain Cargo path dependency is migration debt. Do not add
another one; replace it with a versioned protocol crate or generated contract.
